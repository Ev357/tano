use std::{
    ffi::{CStr, c_void},
    os::raw::c_char,
    ptr::NonNull,
};

use objc2_core_services::{ConstFSEventStreamRef, FSEventStreamEventFlags, FSEventStreamEventId};
use tokio::sync::mpsc::UnboundedSender;

use crate::macos::{debouncer_task::DebouncerCommand, fs_event::FsEvent};

pub unsafe extern "C-unwind" fn fsevent_callback(
    _stream_ref: ConstFSEventStreamRef,
    info: *mut c_void,
    num_events: usize,
    event_paths: NonNull<c_void>,
    event_flags: NonNull<FSEventStreamEventFlags>,
    event_ids: NonNull<FSEventStreamEventId>,
) {
    let tx = unsafe { &*(info as *const UnboundedSender<DebouncerCommand>) };

    let paths = event_paths.as_ptr() as *const *const c_char;
    let flags = event_flags.as_ptr();
    let ids = event_ids.as_ptr();

    for index in 0..num_events {
        let path = unsafe { CStr::from_ptr(*paths.add(index)) };
        let path = path.to_string_lossy().to_string();

        let flag = unsafe { *flags.add(index) };
        let id = unsafe { *ids.add(index) };

        let event = FsEvent::new(flag, path, id);
        let _ = tx.send(DebouncerCommand::Event { event });
    }
}
