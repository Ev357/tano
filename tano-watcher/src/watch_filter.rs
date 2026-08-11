use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct WatchFilter: u8 {
        const IGNORE_DIRECTORIES = 1 << 0;
        const IGNORE_FILES       = 1 << 1;
    }
}
