/* Include dependency from C source: <linux/types.h> */

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
pub const MS_VERBOSE: u32 = 32768; /* War is peace. Verbosity is silence.
                                      MS_VERBOSE is deprecated. */
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

/*
 * Superblock flags that can be altered by MS_REMOUNT
 */
pub const MS_RMT_MASK: u32 = MS_RDONLY | MS_SYNCHRONOUS | MS_MANDLOCK | MS_I_VERSION | MS_LAZYTIME;

/*
 * Old magic mount flag and mask
 */
pub const MS_MGC_VAL: u32 = 0xC0ED0000;
pub const MS_MGC_MSK: u32 = 0xffff0000;

/*
 * open_tree() flags.
 */
pub const OPEN_TREE_CLONE: u32 = 1 << 0; /* Clone the target tree and attach the clone */
pub const OPEN_TREE_NAMESPACE: u32 = 1 << 1; /* Clone the target tree into a new mount namespace */
pub const OPEN_TREE_CLOEXEC: u32 = O_CLOEXEC; /* Close the file on execve() */

/*
 * move_mount() flags.
 */
pub const MOVE_MOUNT_F_SYMLINKS: u32 = 0x00000001; /* Follow symlinks on from path */
pub const MOVE_MOUNT_F_AUTOMOUNTS: u32 = 0x00000002; /* Follow automounts on from path */
pub const MOVE_MOUNT_F_EMPTY_PATH: u32 = 0x00000004; /* Empty from path permitted */
pub const MOVE_MOUNT_T_SYMLINKS: u32 = 0x00000010; /* Follow symlinks on to path */
pub const MOVE_MOUNT_T_AUTOMOUNTS: u32 = 0x00000020; /* Follow automounts on to path */
pub const MOVE_MOUNT_T_EMPTY_PATH: u32 = 0x00000040; /* Empty to path permitted */
pub const MOVE_MOUNT_SET_GROUP: u32 = 0x00000100; /* Set sharing group instead */
pub const MOVE_MOUNT_BENEATH: u32 = 0x00000200; /* Mount beneath top mount */
pub const MOVE_MOUNT__MASK: u32 = 0x00000377;

/*
 * fsopen() flags.
 */
pub const FSOPEN_CLOEXEC: u32 = 0x00000001;

/*
 * fspick() flags.
 */
pub const FSPICK_CLOEXEC: u32 = 0x00000001;
pub const FSPICK_SYMLINK_NOFOLLOW: u32 = 0x00000002;
pub const FSPICK_NO_AUTOMOUNT: u32 = 0x00000004;
pub const FSPICK_EMPTY_PATH: u32 = 0x00000008;

/*
 * The type of fsconfig() call made.
 */
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum fsconfig_command {
    FSCONFIG_SET_FLAG = 0, /* Set parameter, supplying no value */
    FSCONFIG_SET_STRING = 1, /* Set parameter, supplying a string value */
    FSCONFIG_SET_BINARY = 2, /* Set parameter, supplying a binary blob value */
    FSCONFIG_SET_PATH = 3, /* Set parameter, supplying an object by path */
    FSCONFIG_SET_PATH_EMPTY = 4, /* Set parameter, supplying an object by (empty) path */
    FSCONFIG_SET_FD = 5, /* Set parameter, supplying an object by fd */
    FSCONFIG_CMD_CREATE = 6, /* Create new or reuse existing superblock */
    FSCONFIG_CMD_RECONFIGURE = 7, /* Invoke superblock reconfiguration */
    FSCONFIG_CMD_CREATE_EXCL = 8, /* Create new superblock, fail if reusing existing superblock */
}

/*
 * fsmount() flags.
 */
pub const FSMOUNT_CLOEXEC: u32 = 0x00000001;
pub const FSMOUNT_NAMESPACE: u32 = 0x00000002; /* Create the mount in a new mount namespace */

/*
 * Mount attributes.
 */
pub const MOUNT_ATTR_RDONLY: u32 = 0x00000001; /* Mount read-only */
pub const MOUNT_ATTR_NOSUID: u32 = 0x00000002; /* Ignore suid and sgid bits */
pub const MOUNT_ATTR_NODEV: u32 = 0x00000004; /* Disallow access to device special files */
pub const MOUNT_ATTR_NOEXEC: u32 = 0x00000008; /* Disallow program execution */
pub const MOUNT_ATTR__ATIME: u32 = 0x00000070; /* Setting on how atime should be updated */
pub const MOUNT_ATTR_RELATIME: u32 = 0x00000000; /* - Update atime relative to mtime/ctime. */
pub const MOUNT_ATTR_NOATIME: u32 = 0x00000010; /* - Do not update access times. */
pub const MOUNT_ATTR_STRICTATIME: u32 = 0x00000020; /* - Always perform atime updates */
pub const MOUNT_ATTR_NODIRATIME: u32 = 0x00000080; /* Do not update directory access times */
pub const MOUNT_ATTR_IDMAP: u32 = 0x00100000; /* Idmap mount to @userns_fd in struct mount_attr. */
pub const MOUNT_ATTR_NOSYMFOLLOW: u32 = 0x00200000; /* Do not follow symlinks */

/*
 * mount_setattr()
 */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mount_attr {
    pub attr_set: __u64,
    pub attr_clr: __u64,
    pub propagation: __u64,
    pub userns_fd: __u64,
}

/* List of all mount_attr versions. */
pub const MOUNT_ATTR_SIZE_VER0: u32 = 32; /* sizeof first published struct */

/*
 * Structure for getting mount/superblock/filesystem info with statmount(2).
 *
 * The interface is similar to statx(2): individual fields or groups can be
 * selected with the @mask argument of statmount().  Kernel will set the @mask
 * field according to the supported fields.
 *
 * If string fields are selected, then the caller needs to pass a buffer that
 * has space after the fixed part of the structure.  Nul terminated strings are
 * copied there and offsets relative to @str are stored in the relevant fields.
 * If the buffer is too small, then EOVERFLOW is returned.  The actually used
 * size is returned in @size.
 */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct statmount {
    pub size: __u32, /* Total size, including strings */
    pub mnt_opts: __u32, /* [str] Options (comma separated, escaped) */
    pub mask: __u64, /* What results were written */
    pub sb_dev_major: __u32, /* Device ID */
    pub sb_dev_minor: __u32,
    pub sb_magic: __u64, /* ..._SUPER_MAGIC */
    pub sb_flags: __u32, /* SB_{RDONLY,SYNCHRONOUS,DIRSYNC,LAZYTIME} */
    pub fs_type: __u32, /* [str] Filesystem type */
    pub mnt_id: __u64, /* Unique ID of mount */
    pub mnt_parent_id: __u64, /* Unique ID of parent (for root == mnt_id) */
    pub mnt_id_old: __u32, /* Reused IDs used in proc/.../mountinfo */
    pub mnt_parent_id_old: __u32,
    pub mnt_attr: __u64, /* MOUNT_ATTR_... */
    pub mnt_propagation: __u64, /* MS_{SHARED,SLAVE,PRIVATE,UNBINDABLE} */
    pub mnt_peer_group: __u64, /* ID of shared peer group */
    pub mnt_master: __u64, /* Mount receives propagation from this ID */
    pub propagate_from: __u64, /* Propagation from in current namespace */
    pub mnt_root: __u32, /* [str] Root of mount relative to root of fs */
    pub mnt_point: __u32, /* [str] Mountpoint relative to current root */
    pub mnt_ns_id: __u64, /* ID of the mount namespace */
    pub fs_subtype: __u32, /* [str] Subtype of fs_type (if any) */
    pub sb_source: __u32, /* [str] Source string of the mount */
    pub opt_num: __u32, /* Number of fs options */
    pub opt_array: __u32, /* [str] Array of nul terminated fs options */
    pub opt_sec_num: __u32, /* Number of security options */
    pub opt_sec_array: __u32, /* [str] Array of nul terminated security options */
    pub supported_mask: __u64, /* Mask flags that this kernel supports */
    pub mnt_uidmap_num: __u32, /* Number of uid mappings */
    pub mnt_uidmap: __u32, /* [str] Array of uid mappings (as seen from callers namespace) */
    pub mnt_gidmap_num: __u32, /* Number of gid mappings */
    pub mnt_gidmap: __u32, /* [str] Array of gid mappings (as seen from callers namespace) */
    pub __spare2: [__u64; 43],
    pub str: [::std::os::raw::c_char; 0], /* Variable size part containing strings */
}

/*
 * Structure for passing mount ID and miscellaneous parameters to statmount(2)
 * and listmount(2).
 *
 * For statmount(2) @param represents the request mask.
 * For listmount(2) @param represents the last listed mount id (or zero).
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub union mnt_id_req_union {
    pub mnt_ns_fd: __u32,
    pub mnt_fd: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mnt_id_req {
    pub size: __u32,
    pub u: mnt_id_req_union,
    pub mnt_id: __u64,
    pub param: __u64,
    pub mnt_ns_id: __u64,
}

/* List of all mnt_id_req versions. */
pub const MNT_ID_REQ_SIZE_VER0: u32 = 24; /* sizeof first published struct */
pub const MNT_ID_REQ_SIZE_VER1: u32 = 32; /* sizeof second published struct */

/*
 * @mask bits for statmount(2)
 */
pub const STATMOUNT_SB_BASIC: u32 = 0x00000001; /* Want/got sb_... */
pub const STATMOUNT_MNT_BASIC: u32 = 0x00000002; /* Want/got mnt_... */
pub const STATMOUNT_PROPAGATE_FROM: u32 = 0x00000004; /* Want/got propagate_from */
pub const STATMOUNT_MNT_ROOT: u32 = 0x00000008; /* Want/got mnt_root  */
pub const STATMOUNT_MNT_POINT: u32 = 0x00000010; /* Want/got mnt_point */
pub const STATMOUNT_FS_TYPE: u32 = 0x00000020; /* Want/got fs_type */
pub const STATMOUNT_MNT_NS_ID: u32 = 0x00000040; /* Want/got mnt_ns_id */
pub const STATMOUNT_MNT_OPTS: u32 = 0x00000080; /* Want/got mnt_opts */
pub const STATMOUNT_FS_SUBTYPE: u32 = 0x00000100; /* Want/got fs_subtype */
pub const STATMOUNT_SB_SOURCE: u32 = 0x00000200; /* Want/got sb_source */
pub const STATMOUNT_OPT_ARRAY: u32 = 0x00000400; /* Want/got opt_... */
pub const STATMOUNT_OPT_SEC_ARRAY: u32 = 0x00000800; /* Want/got opt_sec... */
pub const STATMOUNT_SUPPORTED_MASK: u32 = 0x00001000; /* Want/got supported mask flags */
pub const STATMOUNT_MNT_UIDMAP: u32 = 0x00002000; /* Want/got uidmap... */
pub const STATMOUNT_MNT_GIDMAP: u32 = 0x00004000; /* Want/got gidmap... */

/*
 * Special @mnt_id values that can be passed to listmount
 */
pub const LSMT_ROOT: u64 = 0xffffffffffffffff; /* root mount */
pub const LISTMOUNT_REVERSE: u32 = 1 << 0; /* List later mounts first */

/*
 * @flag bits for statmount(2)
 */
pub const STATMOUNT_BY_FD: u32 = 0x00000001; /* want mountinfo for given fd */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
