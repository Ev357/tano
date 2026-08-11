use bitflags::bitflags;

bitflags! {
    #[derive(Debug)]
    pub struct FsEventFlag: u32 {
        const NONE                 = objc2_core_services::kFSEventStreamEventFlagNone;
        const MUST_SCAN_SUBDIRS    = objc2_core_services::kFSEventStreamEventFlagMustScanSubDirs;
        const USER_DROPPED         = objc2_core_services::kFSEventStreamEventFlagUserDropped;
        const KERNEL_DROPPED       = objc2_core_services::kFSEventStreamEventFlagKernelDropped;
        const EVENT_IDS_WRAPPED    = objc2_core_services::kFSEventStreamEventFlagEventIdsWrapped;
        const HISTORY_DONE         = objc2_core_services::kFSEventStreamEventFlagHistoryDone;
        const ROOT_CHANGED         = objc2_core_services::kFSEventStreamEventFlagRootChanged;
        const MOUNT                = objc2_core_services::kFSEventStreamEventFlagMount;
        const UNMOUNT              = objc2_core_services::kFSEventStreamEventFlagUnmount;
        const OWN_EVENT            = objc2_core_services::kFSEventStreamEventFlagOwnEvent;
        const ITEM_CREATED         = objc2_core_services::kFSEventStreamEventFlagItemCreated;
        const ITEM_REMOVED         = objc2_core_services::kFSEventStreamEventFlagItemRemoved;
        const ITEM_INODE_META_MOD  = objc2_core_services::kFSEventStreamEventFlagItemInodeMetaMod;
        const ITEM_RENAMED         = objc2_core_services::kFSEventStreamEventFlagItemRenamed;
        const ITEM_MODIFIED        = objc2_core_services::kFSEventStreamEventFlagItemModified;
        const ITEM_FINDER_INFO     = objc2_core_services::kFSEventStreamEventFlagItemFinderInfoMod;
        const ITEM_CHANGE_OWNER    = objc2_core_services::kFSEventStreamEventFlagItemChangeOwner;
        const ITEM_XATTR_MOD       = objc2_core_services::kFSEventStreamEventFlagItemXattrMod;
        const ITEM_IS_FILE         = objc2_core_services::kFSEventStreamEventFlagItemIsFile;
        const ITEM_IS_DIR          = objc2_core_services::kFSEventStreamEventFlagItemIsDir;
        const ITEM_IS_SYMLINK      = objc2_core_services::kFSEventStreamEventFlagItemIsSymlink;
        const ITEM_IS_HARDLINK     = objc2_core_services::kFSEventStreamEventFlagItemIsHardlink;
        const ITEM_IS_LAST_HLINK   = objc2_core_services::kFSEventStreamEventFlagItemIsLastHardlink;
        const ITEM_CLONED          = objc2_core_services::kFSEventStreamEventFlagItemCloned;
    }
}

#[derive(Debug)]
pub struct FsEvent {
    pub path: String,
    pub flag: FsEventFlag,
    pub id: u64,
}

impl FsEvent {
    pub fn new(flag: u32, path: String, id: u64) -> Self {
        let flag = FsEventFlag::from_bits_retain(flag);

        Self { path, flag, id }
    }
}
