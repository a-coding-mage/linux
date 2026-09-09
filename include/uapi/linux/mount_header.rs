/* Translated from linux/mount.h. */

/*
 * These are the fs-independent mount-flags: up to 32 flags are supported
 *
 * Usage of these is restricted within the kernel to core mount(2) code and
 * callers of sys_mount() only.  Filesystems should be using the SB_*
 * equivalent instead.
 */
pub const MS_RDONLY: u32 = 1; /* Mount read-only */
pub const MS_NOSUID: u32 = 2; /* Ignore suid and sgid bits */
pub const MS_NODEV: u32 = 4; /* Disallow access to device special files */
pub const MS_NOEXEC: u32 = 8; /* Disallow program execution */
pub const MS_SYNCHRONOUS: u32 = 16; /* Writes are synced at once */
pub const MS_REMOUNT: u32 = 32; /* Alter flags of a mounted FS */
pub const MS_MANDLOCK: u32 = 64; /* Allow mandatory locks on an FS */
pub const MS_DIRSYNC: u32 = 128; /* Directory modifications are synchronous */
pub const MS_NOSYMFOLLOW: u32 = 256; /* Do not follow symlinks */
pub const MS_NOATIME: u32 = 1024; /* Do not update access times. */
pub const MS_NODIRATIME: u32 = 2048; /* Do not update directory access times */
pub const MS_BIND: u32 = 4096;
pub const MS_MOVE: u32 = 8192;
pub const MS_REC: u32 = 16384;
pub const MS_VERBOSE: u32 = 32768; /* War is peace. Verbosity is silence. MS_VERBOSE is deprecated. */
pub const MS_SILENT: u32 = 32768;
pub const MS_POSIXACL: u32 = 1 << 16; /* VFS does not apply the umask */
pub const MS_UNBINDABLE: u32 = 1 << 17; /* change to unbindable */
pub const MS_PRIVATE: u32 = 1 << 18; /* change to private */
pub const MS_SLAVE: u32 = 1 << 19; /* change to slave */
pub const MS_SHARED: u32 = 1 << 20; /* change to shared */
pub const MS_RELATIME: u32 = 1 << 21; /* Update atime relative to mtime/ctime. */
pub const MS_KERNMOUNT: u32 = 1 << 22; /* this is a kern_mount call */
pub const MS_I_VERSION: u32 = 1 << 23; /* Update inode I_version field */
pub const MS_STRICTATIME: u32 = 1 << 24; /* Always perform atime updates */
pub const MS_LAZYTIME: u32 = 1 << 25; /* Update the on-disk [acm]times lazily */

/* These sb flags are internal to the kernel */
pub const MS_SUBMOUNT: u32 = 1 << 26;
pub const MS_NOREMOTELOCK: u32 = 1 << 27;
pub const MS_NOSEC: u32 = 1 << 28;
pub const MS_BORN: u32 = 1 << 29;
pub const MS_ACTIVE: u32 = 1 << 30;
pub const MS_NOUSER: u32 = 1 << 31;

/* Superblock flags that can be altered by MS_REMOUNT */
pub const MS_RMT_MASK: u32 = MS_RDONLY | MS_SYNCHRONOUS | MS_MANDLOCK | MS_I_VERSION | MS_LAZYTIME;

/* Old magic mount flag and mask */
pub const MS_MGC_VAL: u32 = 0xC0ED0000;
pub const MS_MGC_MSK: u32 = 0xffff0000;

/* open_tree() flags. */
pub const OPEN_TREE_CLONE: u32 = 1 << 0;
pub const OPEN_TREE_NAMESPACE: u32 = 1 << 1;
pub const OPEN_TREE_CLOEXEC: u32 = O_CLOEXEC;

/* move_mount() flags. */
pub const MOVE_MOUNT_F_SYMLINKS: u32 = 0x00000001;
pub const MOVE_MOUNT_F_AUTOMOUNTS: u32 = 0x00000002;
pub const MOVE_MOUNT_F_EMPTY_PATH: u32 = 0x00000004;
pub const MOVE_MOUNT_T_SYMLINKS: u32 = 0x00000010;
pub const MOVE_MOUNT_T_AUTOMOUNTS: u32 = 0x00000020;
pub const MOVE_MOUNT_T_EMPTY_PATH: u32 = 0x00000040;
pub const MOVE_MOUNT_SET_GROUP: u32 = 0x00000100;
pub const MOVE_MOUNT_BENEATH: u32 = 0x00000200;
pub const MOVE_MOUNT__MASK: u32 = 0x00000377;

/* fsopen() flags. */
pub const FSOPEN_CLOEXEC: u32 = 0x00000001;

/* fspick() flags. */
pub const FSPICK_CLOEXEC: u32 = 0x00000001;
pub const FSPICK_SYMLINK_NOFOLLOW: u32 = 0x00000002;
pub const FSPICK_NO_AUTOMOUNT: u32 = 0x00000004;
pub const FSPICK_EMPTY_PATH: u32 = 0x00000008;

/* The type of fsconfig() call made. */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum fsconfig_command {
    FSCONFIG_SET_FLAG = 0,
    FSCONFIG_SET_STRING = 1,
    FSCONFIG_SET_BINARY = 2,
    FSCONFIG_SET_PATH = 3,
    FSCONFIG_SET_PATH_EMPTY = 4,
    FSCONFIG_SET_FD = 5,
    FSCONFIG_CMD_CREATE = 6,
    FSCONFIG_CMD_RECONFIGURE = 7,
    FSCONFIG_CMD_CREATE_EXCL = 8,
}

/* fsmount() flags. */
pub const FSMOUNT_CLOEXEC: u32 = 0x00000001;
pub const FSMOUNT_NAMESPACE: u32 = 0x00000002;

/* Mount attributes. */
pub const MOUNT_ATTR_RDONLY: u32 = 0x00000001;
pub const MOUNT_ATTR_NOSUID: u32 = 0x00000002;
pub const MOUNT_ATTR_NODEV: u32 = 0x00000004;
pub const MOUNT_ATTR_NOEXEC: u32 = 0x00000008;
pub const MOUNT_ATTR__ATIME: u32 = 0x00000070;
pub const MOUNT_ATTR_RELATIME: u32 = 0x00000000;
pub const MOUNT_ATTR_NOATIME: u32 = 0x00000010;
pub const MOUNT_ATTR_STRICTATIME: u32 = 0x00000020;
pub const MOUNT_ATTR_NODIRATIME: u32 = 0x00000080;
pub const MOUNT_ATTR_IDMAP: u32 = 0x00100000;
pub const MOUNT_ATTR_NOSYMFOLLOW: u32 = 0x00200000;

/* mount_setattr() */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mount_attr {
    pub attr_set: __u64,
    pub attr_clr: __u64,
    pub propagation: __u64,
    pub userns_fd: __u64,
}

pub const MOUNT_ATTR_SIZE_VER0: u32 = 32;

#[repr(C)]
pub struct statmount {
    pub size: __u32, pub mnt_opts: __u32, pub mask: __u64, pub sb_dev_major: __u32,
    pub sb_dev_minor: __u32, pub sb_magic: __u64, pub sb_flags: __u32, pub fs_type: __u32,
    pub mnt_id: __u64, pub mnt_parent_id: __u64, pub mnt_id_old: __u32, pub mnt_parent_id_old: __u32,
    pub mnt_attr: __u64, pub mnt_propagation: __u64, pub mnt_peer_group: __u64, pub mnt_master: __u64,
    pub propagate_from: __u64, pub mnt_root: __u32, pub mnt_point: __u32, pub mnt_ns_id: __u64,
    pub fs_subtype: __u32, pub sb_source: __u32, pub opt_num: __u32, pub opt_array: __u32,
    pub opt_sec_num: __u32, pub opt_sec_array: __u32, pub supported_mask: __u64,
    pub mnt_uidmap_num: __u32, pub mnt_uidmap: __u32, pub mnt_gidmap_num: __u32, pub mnt_gidmap: __u32,
    pub __spare2: [__u64; 43], pub str_: [core::ffi::c_char; 0],
}

#[repr(C)]
pub union mnt_id_req_mnt {
    pub mnt_ns_fd: __u32,
    pub mnt_fd: __u32,
}

#[repr(C)]
pub struct mnt_id_req {
    pub size: __u32,
    pub mnt: mnt_id_req_mnt,
    pub mnt_id: __u64,
    pub param: __u64,
    pub mnt_ns_id: __u64,
}

pub const MNT_ID_REQ_SIZE_VER0: u32 = 24;
pub const MNT_ID_REQ_SIZE_VER1: u32 = 32;

/* @mask bits for statmount(2) */
pub const STATMOUNT_SB_BASIC: u32 = 0x00000001;
pub const STATMOUNT_MNT_BASIC: u32 = 0x00000002;
pub const STATMOUNT_PROPAGATE_FROM: u32 = 0x00000004;
pub const STATMOUNT_MNT_ROOT: u32 = 0x00000008;
pub const STATMOUNT_MNT_POINT: u32 = 0x00000010;
pub const STATMOUNT_FS_TYPE: u32 = 0x00000020;
pub const STATMOUNT_MNT_NS_ID: u32 = 0x00000040;
pub const STATMOUNT_MNT_OPTS: u32 = 0x00000080;
pub const STATMOUNT_FS_SUBTYPE: u32 = 0x00000100;
pub const STATMOUNT_SB_SOURCE: u32 = 0x00000200;
pub const STATMOUNT_OPT_ARRAY: u32 = 0x00000400;
pub const STATMOUNT_OPT_SEC_ARRAY: u32 = 0x00000800;
pub const STATMOUNT_SUPPORTED_MASK: u32 = 0x00001000;
pub const STATMOUNT_MNT_UIDMAP: u32 = 0x00002000;
pub const STATMOUNT_MNT_GIDMAP: u32 = 0x00004000;

/* Special @mnt_id values that can be passed to listmount */
pub const LSMT_ROOT: u64 = 0xffffffffffffffff;
pub const LISTMOUNT_REVERSE: u32 = 1 << 0;

/* @flag bits for statmount(2) */
pub const STATMOUNT_BY_FD: u32 = 0x00000001;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
