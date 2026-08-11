use bitflags::bitflags;

bitflags! {
    #[derive(Debug)]
    pub struct INotifyMask: u32 {
        const ACCESS        = libc::IN_ACCESS;
        const MODIFY        = libc::IN_MODIFY;
        const ATTRIB        = libc::IN_ATTRIB;
        const CLOSE_WRITE   = libc::IN_CLOSE_WRITE;
        const CLOSE_NOWRITE = libc::IN_CLOSE_NOWRITE;
        const OPEN          = libc::IN_OPEN;
        const MOVED_FROM    = libc::IN_MOVED_FROM;
        const MOVED_TO      = libc::IN_MOVED_TO;
        const CREATE        = libc::IN_CREATE;
        const DELETE        = libc::IN_DELETE;
        const DELETE_SELF   = libc::IN_DELETE_SELF;
        const MOVE_SELF     = libc::IN_MOVE_SELF;
        const UNMOUNT       = libc::IN_UNMOUNT;
        const Q_OVERFLOW    = libc::IN_Q_OVERFLOW;
        const IGNORED       = libc::IN_IGNORED;
        const ISDIR         = libc::IN_ISDIR;
        const ONLYDIR       = libc::IN_ONLYDIR;
        const DONT_FOLLOW   = libc::IN_DONT_FOLLOW;
        const EXCL_UNLINK   = libc::IN_EXCL_UNLINK;
        const MASK_CREATE   = libc::IN_MASK_CREATE;
        const MASK_ADD      = libc::IN_MASK_ADD;
        const ONESHOT       = libc::IN_ONESHOT;
    }
}

#[derive(Debug)]
pub struct INotifyEvent {
    pub wd: i32,
    pub mask: INotifyMask,
    pub cookie: u32,
    pub name: Option<String>,
}

impl INotifyEvent {
    pub fn new(wd: i32, mask: u32, cookie: u32, name: Option<String>) -> Self {
        let mask = INotifyMask::from_bits_retain(mask);

        Self {
            wd,
            mask,
            cookie,
            name,
        }
    }
}
