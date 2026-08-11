use std::{
    ffi::{self},
    fs::File as StdFile,
    io::Read,
    mem, ptr,
};

use color_eyre::eyre::Result;
use tokio::{io::unix::AsyncFd, sync::mpsc::UnboundedSender, task::JoinHandle};

use crate::{
    linux::{debouncer_task::DebouncerCommand, inotify_event::INotifyEvent},
    watch_event::WatchEvent,
};

pub fn reader_task(
    async_fd: AsyncFd<StdFile>,
    cmd_tx: UnboundedSender<DebouncerCommand>,
    tx: UnboundedSender<Result<WatchEvent>>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut buffer = [0u8; 4096];

        loop {
            let mut guard = match async_fd.readable().await {
                Ok(guard) => guard,
                Err(_) => break,
            };

            let read_result = guard.try_io(|inner_file| inner_file.get_ref().read(&mut buffer));

            let bytes_read = match read_result {
                Ok(Ok(len)) if len > 0 => len,
                Ok(Ok(_)) => break,
                Ok(Err(error)) => {
                    let _ = tx.send(Err(error.into()));
                    break;
                }
                _ => continue,
            };

            let mut offset = 0;
            while offset < bytes_read {
                let event_ptr =
                    unsafe { buffer.as_ptr().add(offset) as *const libc::inotify_event };
                let event = unsafe { ptr::read_unaligned(event_ptr) };

                let mut event_name = None;
                if event.len > 0 {
                    let name_ptr = unsafe {
                        buffer
                            .as_ptr()
                            .add(offset + mem::size_of::<libc::inotify_event>())
                            as *const i8
                    };
                    let c_str = unsafe { ffi::CStr::from_ptr(name_ptr) };
                    event_name = Some(c_str.to_string_lossy().to_string());
                }

                let inotify_event =
                    INotifyEvent::new(event.wd, event.mask, event.cookie, event_name);

                if cmd_tx
                    .send(DebouncerCommand::Event {
                        event: inotify_event,
                    })
                    .is_err()
                {
                    break;
                }

                offset += mem::size_of::<libc::inotify_event>() + (event.len as usize);
            }
        }
    })
}
