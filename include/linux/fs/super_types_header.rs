/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/fs/super_types.h. Included C dependencies are supplied elsewhere.

pub struct BackingDevInfo;
pub struct BlockDevice;
pub struct Dentry;
pub struct DentryOperations;
pub struct DquotOperations;
pub struct ExportOperations;
pub struct File;
pub struct FileSystemType;
pub struct FscryptOperations;
pub struct FsnotifySbInfo;
pub struct FsverityOperations;
pub struct Kstatfs;
pub struct Mount;
pub struct MtdInfo;
pub struct QuotactlOps;
pub struct Shrinker;
pub struct SuperDev;
pub struct UnicodeMap;
pub struct UserNamespace;
pub struct WorkqueueStruct;
pub struct WritebackControl;
pub struct XattrHandler;
pub struct FserrorEvent;
pub struct Inode;
pub struct SeqFile;
pub struct ShrinkControl;
pub struct Dquot;
pub struct SuperBlock;

extern "C" {
    pub static mut blockdev_superblock: *mut SuperBlock;
}

pub const SB_UNFROZEN: u32 = 0;
pub const SB_FREEZE_WRITE: u32 = 1;
pub const SB_FREEZE_PAGEFAULT: u32 = 2;
pub const SB_FREEZE_FS: u32 = 3;
pub const SB_FREEZE_COMPLETE: u32 = 4;
pub const SB_FREEZE_LEVELS: usize = (SB_FREEZE_COMPLETE - 1) as usize;

#[repr(C)]
pub struct SbWriters {
    pub frozen: u16,
    pub freeze_kcount: i32,
    pub freeze_ucount: i32,
    pub freeze_owner: *const core::ffi::c_void,
    pub rw_sem: [PerCpuRwSemaphore; SB_FREEZE_LEVELS],
}

#[repr(u32)]
pub enum FreezeHolder {
    FREEZE_HOLDER_KERNEL = 1u32 << 0,
    FREEZE_HOLDER_USERSPACE = 1u32 << 1,
    FREEZE_MAY_NEST = 1u32 << 2,
    FREEZE_EXCL = 1u32 << 3,
}

pub struct PerCpuRwSemaphore;
pub struct ListHead;
pub struct RwSemaphore;
pub struct Refcount;
pub struct Atomic;
pub struct HlistHead;
pub struct Spinlock;
pub struct Mutex;
pub struct HlistNode;
pub struct QuotaInfo;
pub struct Errseq;
pub struct ListLru;
pub struct RcuHead;
pub struct WorkStruct;
pub struct AtomicLong;
pub type DevT = u64;
pub type LoffT = i64;
pub type Time64T = i64;
pub type UuidT = [u8; 16];

#[repr(C)]
pub struct SuperOperations {
    pub alloc_inode: Option<unsafe extern "C" fn(*mut SuperBlock) -> *mut Inode>,
    pub destroy_inode: Option<unsafe extern "C" fn(*mut Inode)>,
    pub free_inode: Option<unsafe extern "C" fn(*mut Inode)>,
    pub dirty_inode: Option<unsafe extern "C" fn(*mut Inode, i32)>,
    pub write_inode: Option<unsafe extern "C" fn(*mut Inode, *mut WritebackControl) -> i32>,
    pub sync_inode_metadata: Option<unsafe extern "C" fn(*mut Inode, *mut WritebackControl) -> i32>,
    pub drop_inode: Option<unsafe extern "C" fn(*mut Inode) -> i32>,
    pub evict_inode: Option<unsafe extern "C" fn(*mut Inode)>,
    pub put_super: Option<unsafe extern "C" fn(*mut SuperBlock)>,
    pub sync_fs: Option<unsafe extern "C" fn(*mut SuperBlock, i32) -> i32>,
    pub freeze_super: Option<unsafe extern "C" fn(*mut SuperBlock, FreezeHolder, *const core::ffi::c_void) -> i32>,
    pub freeze_fs: Option<unsafe extern "C" fn(*mut SuperBlock) -> i32>,
    pub thaw_super: Option<unsafe extern "C" fn(*mut SuperBlock, FreezeHolder, *const core::ffi::c_void) -> i32>,
    pub unfreeze_fs: Option<unsafe extern "C" fn(*mut SuperBlock) -> i32>,
    pub statfs: Option<unsafe extern "C" fn(*mut Dentry, *mut Kstatfs) -> i32>,
    pub umount_begin: Option<unsafe extern "C" fn(*mut SuperBlock)>,
    pub show_options: Option<unsafe extern "C" fn(*mut SeqFile, *mut Dentry) -> i32>,
    pub show_devname: Option<unsafe extern "C" fn(*mut SeqFile, *mut Dentry) -> i32>,
    pub show_path: Option<unsafe extern "C" fn(*mut SeqFile, *mut Dentry) -> i32>,
    pub show_stats: Option<unsafe extern "C" fn(*mut SeqFile, *mut Dentry) -> i32>,
    #[cfg(CONFIG_QUOTA)]
    pub quota_read: Option<unsafe extern "C" fn(*mut SuperBlock, i32, *mut i8, usize, LoffT) -> isize>,
    #[cfg(CONFIG_QUOTA)]
    pub quota_write: Option<unsafe extern "C" fn(*mut SuperBlock, i32, *const i8, usize, LoffT) -> isize>,
    #[cfg(CONFIG_QUOTA)]
    pub get_dquots: Option<unsafe extern "C" fn(*mut Inode) -> *mut *mut Dquot>,
    pub nr_cached_objects: Option<unsafe extern "C" fn(*mut SuperBlock, *mut ShrinkControl) -> isize>,
    pub free_cached_objects: Option<unsafe extern "C" fn(*mut SuperBlock, *mut ShrinkControl) -> isize>,
    pub remove_bdev: Option<unsafe extern "C" fn(*mut SuperBlock, *mut BlockDevice) -> i32>,
    pub shutdown: Option<unsafe extern "C" fn(*mut SuperBlock)>,
    pub report_error: Option<unsafe extern "C" fn(*const FserrorEvent)>,
}

#[repr(C)]
pub struct SuperBlock {
    pub s_list: ListHead,
    pub s_dev: DevT,
    pub s_super_dev: *mut SuperDev,
    pub s_blocksize_bits: u8,
    pub s_blocksize: usize,
    pub s_maxbytes: LoffT,
    pub s_type: *mut FileSystemType,
    pub s_op: *const SuperOperations,
    pub dq_op: *const DquotOperations,
    pub s_qcop: *const QuotactlOps,
    pub s_export_op: *const ExportOperations,
    pub s_flags: usize,
    pub s_iflags: usize,
    pub s_magic: usize,
    pub s_root: *mut Dentry,
    pub s_umount: RwSemaphore,
    pub s_passive: Refcount,
    pub s_active: Atomic,
    #[cfg(CONFIG_SECURITY)] pub s_security: *mut core::ffi::c_void,
    pub s_xattr: *const *const XattrHandler,
    #[cfg(CONFIG_FS_ENCRYPTION)] pub s_cop: *const FscryptOperations,
    #[cfg(CONFIG_FS_ENCRYPTION)] pub s_master_keys: *mut core::ffi::c_void,
    #[cfg(CONFIG_FS_VERITY)] pub s_vop: *const FsverityOperations,
    #[cfg(CONFIG_UNICODE)] pub s_encoding: *mut UnicodeMap,
    #[cfg(CONFIG_UNICODE)] pub s_encoding_flags: u16,
    pub s_roots: HlistHead,
    pub s_roots_lock: Spinlock,
    pub s_mounts: *mut Mount,
    pub s_bdev: *mut BlockDevice,
    pub s_bdev_file: *mut File,
    pub s_bdi: *mut BackingDevInfo,
    pub s_mtd: *mut MtdInfo,
    pub s_instances: HlistNode,
    pub s_quota_types: u32,
    pub s_dquot: QuotaInfo,
    pub s_writers: SbWriters,
    pub s_fs_info: *mut core::ffi::c_void,
    pub s_time_gran: u32,
    pub s_time_min: Time64T,
    pub s_time_max: Time64T,
    #[cfg(CONFIG_FSNOTIFY)] pub s_fsnotify_mask: u32,
    #[cfg(CONFIG_FSNOTIFY)] pub s_fsnotify_info: *mut FsnotifySbInfo,
    pub s_id: [i8; 32],
    pub s_uuid: UuidT,
    pub s_uuid_len: u8,
    pub s_sysfs_name: [i8; 37],
    pub s_max_links: u32,
    pub s_d_flags: u32,
    pub s_vfs_rename_mutex: Mutex,
    pub s_subtype: *const i8,
    pub __s_d_op: *const DentryOperations,
    pub s_shrink: *mut Shrinker,
    pub s_remove_count: AtomicLong,
    pub s_readonly_remount: i32,
    pub s_wb_err: Errseq,
    pub s_dio_done_wq: *mut WorkqueueStruct,
    pub s_pins: HlistHead,
    pub s_user_ns: *mut UserNamespace,
    pub s_dentry_lru: ListLru,
    pub s_inode_lru: ListLru,
    pub rcu: RcuHead,
    pub destroy_work: WorkStruct,
    pub s_sync_lock: Mutex,
    pub s_stack_depth: i32,
    pub s_inode_list_lock: Spinlock,
    pub s_inodes: ListHead,
    pub s_inode_wblist_lock: Spinlock,
    pub s_inodes_wb: ListHead,
    pub s_min_writeback_pages: i64,
    pub s_pending_errors: Refcount,
    #[cfg(CONFIG_CGROUP_WRITEBACK)] pub s_isw_nr_in_flight: Atomic,
}

pub const SB_RDONLY: usize = 1 << 0;
pub const SB_NOSUID: usize = 1 << 1;
pub const SB_NODEV: usize = 1 << 2;
pub const SB_NOEXEC: usize = 1 << 3;
pub const SB_SYNCHRONOUS: usize = 1 << 4;
pub const SB_MANDLOCK: usize = 1 << 6;
pub const SB_DIRSYNC: usize = 1 << 7;
pub const SB_NOATIME: usize = 1 << 10;
pub const SB_NODIRATIME: usize = 1 << 11;
pub const SB_SILENT: usize = 1 << 15;
pub const SB_POSIXACL: usize = 1 << 16;
pub const SB_INLINECRYPT: usize = 1 << 17;
pub const SB_KERNMOUNT: usize = 1 << 22;
pub const SB_I_VERSION: usize = 1 << 23;
pub const SB_LAZYTIME: usize = 1 << 25;
pub const SB_DEAD: usize = 1 << 21;
pub const SB_DYING: usize = 1 << 24;
pub const SB_FORCE: usize = 1 << 27;
pub const SB_NOSEC: usize = 1 << 28;
pub const SB_BORN: usize = 1 << 29;
pub const SB_ACTIVE: usize = 1 << 30;
pub const SB_NOUSER: usize = 1 << 31;
pub const SB_ENC_STRICT_MODE_FL: u32 = 1 << 0;
pub const SB_ENC_NO_COMPAT_FALLBACK_FL: u32 = 1 << 1;
pub const SB_I_CGROUPWB: u32 = 0x00000001;
pub const SB_I_NOEXEC: u32 = 0x00000002;
pub const SB_I_NODEV: u32 = 0x00000004;
pub const SB_I_STABLE_WRITES: u32 = 0x00000008;
pub const SB_I_RESTRICTED_VARIANT: u32 = 0x00000010;
pub const SB_I_IMA_UNVERIFIABLE_SIGNATURE: u32 = 0x00000020;
pub const SB_I_UNTRUSTED_MOUNTER: u32 = 0x00000040;
pub const SB_I_EVM_HMAC_UNSUPPORTED: u32 = 0x00000080;
pub const SB_I_SKIP_SYNC: u32 = 0x00000100;
pub const SB_I_PERSB_BDI: u32 = 0x00000200;
pub const SB_I_TS_EXPIRY_WARNED: u32 = 0x00000400;
pub const SB_I_RETIRED: u32 = 0x00000800;
pub const SB_I_NOUMASK: u32 = 0x00001000;
pub const SB_I_NOIDMAP: u32 = 0x00002000;
pub const SB_I_ALLOW_HSM: u32 = 0x00004000;
pub const SB_I_NO_DATA_INTEGRITY: u32 = 0x00008000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
