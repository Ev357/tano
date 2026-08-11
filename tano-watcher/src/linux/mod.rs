use std::{
    collections::HashMap,
    ffi::CString,
    fs::{self, File as StdFile},
    io::{Error, ErrorKind},
    os::unix::io::{FromRawFd, RawFd},
};

use color_eyre::eyre::Result;
use tokio::{
    io::unix::AsyncFd,
    sync::mpsc::{self, UnboundedSender},
    task::JoinHandle,
};

use crate::{
    linux::{
        debouncer_task::{DebouncerCommand, debouncer_task},
        reader_task::reader_task,
    },
    watch_entry::WatchEntry,
    watch_event::WatchEvent,
    watch_id::WatchId,
    watcher::Watcher,
};

pub mod debouncer_task;
pub mod inotify_event;
pub mod reader_task;

#[derive(Debug)]
pub struct INotifyWatcher {
    reader_handle: JoinHandle<()>,
    debouncer_handle: JoinHandle<()>,
    fd: RawFd,
    cmd_tx: UnboundedSender<DebouncerCommand>,
    id_to_wd: HashMap<WatchId, i32>,
}

impl Watcher for INotifyWatcher {
    fn new(event_handler: UnboundedSender<Result<WatchEvent>>) -> Result<Self> {
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<DebouncerCommand>();

        let fd = unsafe { libc::inotify_init1(libc::IN_NONBLOCK) };
        if fd < 0 {
            return Err(Error::last_os_error().into());
        }

        let std_file = unsafe { StdFile::from_raw_fd(fd) };

        let async_fd = AsyncFd::new(std_file)?;

        let reader_handle = reader_task(async_fd, cmd_tx.clone(), event_handler.clone());
        let debouncer_handle = debouncer_task(cmd_rx, event_handler);

        Ok(Self {
            reader_handle,
            debouncer_handle,
            fd,
            cmd_tx,
            id_to_wd: std::collections::HashMap::new(),
        })
    }

    fn watch(&mut self, watch_entry: WatchEntry) -> Result<()> {
        let canonical_path = fs::canonicalize(watch_entry.path)?;
        let path = canonical_path.to_str().ok_or_else(|| {
            Error::new(ErrorKind::InvalidInput, "Invalid UTF-8 in canonical path")
        })?;

        let c_path = CString::new(path)
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "Null byte in path"))?;

        const MASK: u32 = libc::IN_MODIFY
            | libc::IN_ATTRIB
            | libc::IN_CLOSE_WRITE
            | libc::IN_MOVED_FROM
            | libc::IN_MOVED_TO
            | libc::IN_CREATE
            | libc::IN_DELETE;

        let wd = unsafe { libc::inotify_add_watch(self.fd, c_path.as_ptr(), MASK) };
        if wd < 0 {
            return Err(Error::last_os_error().into());
        }

        self.id_to_wd.insert(watch_entry.id, wd);

        let _ = self.cmd_tx.send(DebouncerCommand::Watch {
            wd,
            path: path.to_string(),
            watch_id: watch_entry.id,
            filter: watch_entry.filter,
        });

        Ok(())
    }

    fn unwatch(&mut self, watch_id: &WatchId) -> Result<()> {
        if let Some(wd) = self.id_to_wd.remove(watch_id) {
            let ret = unsafe { libc::inotify_rm_watch(self.fd, wd) };
            if ret < 0 {
                return Err(Error::last_os_error().into());
            }

            let _ = self.cmd_tx.send(DebouncerCommand::Unwatch { wd });
        }
        Ok(())
    }
}

impl Drop for INotifyWatcher {
    fn drop(&mut self) {
        self.reader_handle.abort();
        self.debouncer_handle.abort();
    }
}
