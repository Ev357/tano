use std::{collections::HashMap, ffi::c_void, fs};

use color_eyre::eyre::{Result, eyre};
use dispatch2::{DispatchQueue, DispatchRetained};
use objc2_core_foundation::{CFArray, CFString};
use objc2_core_services::{
    FSEventStreamContext, FSEventStreamCreate, FSEventStreamInvalidate, FSEventStreamRef,
    FSEventStreamRelease, FSEventStreamSetDispatchQueue, FSEventStreamStart, FSEventStreamStop,
    kFSEventStreamCreateFlagFileEvents, kFSEventStreamCreateFlagNoDefer,
    kFSEventStreamEventIdSinceNow,
};
use tokio::{
    sync::mpsc::{self, UnboundedSender},
    task::JoinHandle,
};

use crate::{
    macos::{
        debouncer_task::{DebouncerCommand, debouncer_task},
        fsevent_callback::fsevent_callback,
    },
    watch_entry::WatchEntry,
    watch_event::WatchEvent,
    watch_id::WatchId,
    watcher::Watcher,
};

pub mod debouncer_task;
pub mod fs_event;
mod fsevent_callback;
pub mod watch_map;

#[derive(Debug)]
pub struct FsEventWatcher {
    debouncer_handle: JoinHandle<()>,
    cmd_tx: UnboundedSender<DebouncerCommand>,
    stream: Option<FSEventStreamRef>,
    queue: DispatchRetained<DispatchQueue>,
    paths: HashMap<WatchId, String>,
    tx_ptr: *mut c_void,
}

impl Watcher for FsEventWatcher {
    fn new(event_handler: UnboundedSender<Result<WatchEvent>>) -> Result<Self> {
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<DebouncerCommand>();

        let tx_ptr = Box::into_raw(Box::new(cmd_tx.clone())) as *mut c_void;

        let queue = DispatchQueue::new("tano_watcher", None);
        let debouncer_handle = debouncer_task(cmd_rx, event_handler);

        Ok(Self {
            debouncer_handle,
            cmd_tx,
            stream: None,
            queue,
            paths: HashMap::new(),
            tx_ptr,
        })
    }

    fn watch(&mut self, watch_entry: WatchEntry) -> Result<()> {
        let canonical_path = fs::canonicalize(&watch_entry.path)?;
        let path = canonical_path
            .to_str()
            .ok_or_else(|| eyre!("Invalid UTF-8 in canonical path"))?;

        self.paths.insert(watch_entry.id, path.to_string());

        let _ = self.cmd_tx.send(DebouncerCommand::Watch {
            id: watch_entry.id,
            path: path.to_string(),
            filter: watch_entry.filter,
        });

        self.recreate()?;
        Ok(())
    }

    fn unwatch(&mut self, watch_id: &WatchId) -> Result<()> {
        self.paths.remove(watch_id);

        let _ = self
            .cmd_tx
            .send(DebouncerCommand::Unwatch { id: *watch_id });

        self.recreate()?;

        Ok(())
    }
}

unsafe impl Send for FsEventWatcher {}
unsafe impl Sync for FsEventWatcher {}

impl FsEventWatcher {
    fn recreate(&mut self) -> Result<()> {
        if self.paths.is_empty() {
            if let Some(old_stream) = self.stream.take() {
                unsafe {
                    FSEventStreamStop(old_stream);
                    FSEventStreamInvalidate(old_stream);
                    FSEventStreamRelease(old_stream);
                }
            }
            return Ok(());
        }

        let paths_to_watch: Vec<_> = self
            .paths
            .values()
            .map(|path| CFString::from_str(path))
            .collect();

        let paths_to_watch_cf = CFArray::from_retained_objects(&paths_to_watch);

        let mut context = FSEventStreamContext {
            version: 0,
            info: self.tx_ptr,
            retain: None,
            release: None,
            copyDescription: None,
        };

        let new_stream = unsafe {
            FSEventStreamCreate(
                None,
                Some(fsevent_callback),
                &mut context,
                paths_to_watch_cf.as_opaque(),
                kFSEventStreamEventIdSinceNow,
                0.0,
                kFSEventStreamCreateFlagFileEvents | kFSEventStreamCreateFlagNoDefer,
            )
        };

        if new_stream.is_null() {
            return Err(eyre!("macOS FSEventStreamCreate returned a null pointer"));
        }

        unsafe {
            FSEventStreamSetDispatchQueue(new_stream, Some(&self.queue));
        }

        let started = unsafe { FSEventStreamStart(new_stream) };
        if !started {
            unsafe {
                FSEventStreamInvalidate(new_stream);
                FSEventStreamRelease(new_stream);
            }
            return Err(eyre!(
                "macOS FSEventStreamStart failed to start the watcher"
            ));
        }

        if let Some(old_stream) = self.stream.take() {
            unsafe {
                FSEventStreamStop(old_stream);
                FSEventStreamInvalidate(old_stream);
                FSEventStreamRelease(old_stream);
            }
        }

        self.stream = Some(new_stream);
        Ok(())
    }
}

impl Drop for FsEventWatcher {
    fn drop(&mut self) {
        if let Some(stream) = self.stream.take() {
            unsafe {
                FSEventStreamStop(stream);
                FSEventStreamInvalidate(stream);
                FSEventStreamRelease(stream);
            }
        }

        unsafe {
            let _reclaimed_tx =
                Box::from_raw(self.tx_ptr as *mut UnboundedSender<DebouncerCommand>);
        }

        self.debouncer_handle.abort();
    }
}
