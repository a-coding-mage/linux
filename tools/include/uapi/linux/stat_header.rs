/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependency intent from C header: #include <linux/types.h> */

/* C conditional: defined(__KERNEL__) || !defined(__GLIBC__) || (__GLIBC__ < 2) */
pub const S_IFMT: u32 = 0o170000;
pub const S_IFSOCK: u32 = 0o140000;
pub const S_IFLNK: u32 = 0o120000;
pub const S_IFREG: u32 = 0o100000;
pub const S_IFBLK: u32 = 0o060000;
pub const S_IFDIR: u32 = 0o040000;
pub const S_IFCHR: u32 = 0o020000;
pub const S_IFIFO: u32 = 0o010000;
pub const S_ISUID: u32 = 0o004000;
pub const S_ISGID: u32 = 0o002000;
pub const S_ISVTX: u32 = 0o001000;

pub const fn S_ISLNK(m: u32) -> bool {
    (m & S_IFMT) == S_IFLNK
}

pub const fn S_ISREG(m: u32) -> bool {
    (m & S_IFMT) == S_IFREG
}

pub const fn S_ISDIR(m: u32) -> bool {
    (m & S_IFMT) == S_IFDIR
}

pub const fn S_ISCHR(m: u32) -> bool {
    (m & S_IFMT) == S_IFCHR
}

pub const fn S_ISBLK(m: u32) -> bool {
    (m & S_IFMT) == S_IFBLK
}

pub const fn S_ISFIFO(m: u32) -> bool {
    (m & S_IFMT) == S_IFIFO
}

pub const fn S_ISSOCK(m: u32) -> bool {
    (m & S_IFMT) == S_IFSOCK
}

pub const S_IRWXU: u32 = 0o0700;
pub const S_IRUSR: u32 = 0o0400;
pub const S_IWUSR: u32 = 0o0200;
pub const S_IXUSR: u32 = 0o0100;

pub const S_IRWXG: u32 = 0o0070;
pub const S_IRGRP: u32 = 0o0040;
pub const S_IWGRP: u32 = 0o0020;
pub const S_IXGRP: u32 = 0o0010;

pub const S_IRWXO: u32 = 0o0007;
pub const S_IROTH: u32 = 0o0004;
pub const S_IWOTH: u32 = 0o0002;
pub const S_IXOTH: u32 = 0o0001;

/*
 * Timestamp structure for the timestamps in struct statx.
 *
 * tv_sec holds the number of seconds before (negative) or after (positive)
 * 00:00:00 1st January 1970 UTC.
 *
 * tv_nsec holds a number of nanoseconds (0..999,999,999) after the tv_sec time.
 *
 * __reserved is held in case we need a yet finer resolution.
 */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct statx_timestamp {
    pub tv_sec: __s64,
    pub tv_nsec: __u32,
    pub __reserved: __s32,
}

/*
 * Structures for the extended file attribute retrieval system call
 * (statx()).
 *
 * The caller passes a mask of what they're specifically interested in as a
 * parameter to statx().  What statx() actually got will be indicated in
 * st_mask upon return.
 *
 * For each bit in the mask argument:
 *
 * - if the datum is not supported:
 *
 *   - the bit will be cleared, and
 *
 *   - the datum will be set to an appropriate fabricated value if one is
 *     available (eg. CIFS can take a default uid and gid), otherwise
 *
 *   - the field will be cleared;
 *
 * - otherwise, if explicitly requested:
 *
 *   - the datum will be synchronised to the server if AT_STATX_FORCE_SYNC is
 *     set or if the datum is considered out of date, and
 *
 *   - the field will be filled in and the bit will be set;
 *
 * - otherwise, if not requested, but available in approximate form without any
 *   effort, it will be filled in anyway, and the bit will be set upon return
 *   (it might not be up to date, however, and no attempt will be made to
 *   synchronise the internal state first);
 *
 * - otherwise the field and the bit will be cleared before returning.
 *
 * Items in STATX_BASIC_STATS may be marked unavailable on return, but they
 * will have values installed for compatibility purposes so that stat() and
 * co. can be emulated in userspace.
 */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct statx {
    /* 0x00 */
    /* What results were written [uncond] */
    pub stx_mask: __u32,

    /* Preferred general I/O size [uncond] */
    pub stx_blksize: __u32,

    /* Flags conveying information about the file [uncond] */
    pub stx_attributes: __u64,

    /* 0x10 */
    /* Number of hard links */
    pub stx_nlink: __u32,

    /* User ID of owner */
    pub stx_uid: __u32,

    /* Group ID of owner */
    pub stx_gid: __u32,

    /* File mode */
    pub stx_mode: __u16,
    pub __spare0: [__u16; 1],

    /* 0x20 */
    /* Inode number */
    pub stx_ino: __u64,

    /* File size */
    pub stx_size: __u64,

    /* Number of 512-byte blocks allocated */
    pub stx_blocks: __u64,

    /* Mask to show what's supported in stx_attributes */
    pub stx_attributes_mask: __u64,

    /* 0x40 */
    /* Last access time */
    pub stx_atime: statx_timestamp,

    /* File creation time */
    pub stx_btime: statx_timestamp,

    /* Last attribute change time */
    pub stx_ctime: statx_timestamp,

    /* Last data modification time */
    pub stx_mtime: statx_timestamp,

    /* 0x80 */
    /* Device ID of special file [if bdev/cdev] */
    pub stx_rdev_major: __u32,
    pub stx_rdev_minor: __u32,

    /* ID of device containing file [uncond] */
    pub stx_dev_major: __u32,
    pub stx_dev_minor: __u32,

    /* 0x90 */
    pub stx_mnt_id: __u64,

    /* Memory buffer alignment for direct I/O */
    pub stx_dio_mem_align: __u32,

    /* File offset alignment for direct I/O */
    pub stx_dio_offset_align: __u32,

    /* 0xa0 */
    /* Subvolume identifier */
    pub stx_subvol: __u64,

    /* Min atomic write unit in bytes */
    pub stx_atomic_write_unit_min: __u32,

    /* Max atomic write unit in bytes */
    pub stx_atomic_write_unit_max: __u32,

    /* 0xb0 */
    /* Max atomic write segment count */
    pub stx_atomic_write_segments_max: __u32,

    /* File offset alignment for direct I/O reads */
    pub stx_dio_read_offset_align: __u32,

    /* Optimised max atomic write unit in bytes */
    pub stx_atomic_write_unit_max_opt: __u32,
    pub __spare2: [__u32; 1],

    /* 0xc0 */
    pub __spare3: [__u64; 8], /* Spare space for future expansion */

    /* 0x100 */
}

/*
 * Flags to be stx_mask
 *
 * Query request/result mask for statx() and struct statx::stx_mask.
 *
 * These bits should be set in the mask argument of statx() to request
 * particular items when calling statx().
 */
pub const STATX_TYPE: u32 = 0x00000001; /* Want/got stx_mode & S_IFMT */
pub const STATX_MODE: u32 = 0x00000002; /* Want/got stx_mode & ~S_IFMT */
pub const STATX_NLINK: u32 = 0x00000004; /* Want/got stx_nlink */
pub const STATX_UID: u32 = 0x00000008; /* Want/got stx_uid */
pub const STATX_GID: u32 = 0x00000010; /* Want/got stx_gid */
pub const STATX_ATIME: u32 = 0x00000020; /* Want/got stx_atime */
pub const STATX_MTIME: u32 = 0x00000040; /* Want/got stx_mtime */
pub const STATX_CTIME: u32 = 0x00000080; /* Want/got stx_ctime */
pub const STATX_INO: u32 = 0x00000100; /* Want/got stx_ino */
pub const STATX_SIZE: u32 = 0x00000200; /* Want/got stx_size */
pub const STATX_BLOCKS: u32 = 0x00000400; /* Want/got stx_blocks */
pub const STATX_BASIC_STATS: u32 = 0x000007ff; /* The stuff in the normal stat struct */
pub const STATX_BTIME: u32 = 0x00000800; /* Want/got stx_btime */
pub const STATX_MNT_ID: u32 = 0x00001000; /* Got stx_mnt_id */
pub const STATX_DIOALIGN: u32 = 0x00002000; /* Want/got direct I/O alignment info */
pub const STATX_MNT_ID_UNIQUE: u32 = 0x00004000; /* Want/got extended stx_mount_id */
pub const STATX_SUBVOL: u32 = 0x00008000; /* Want/got stx_subvol */
pub const STATX_WRITE_ATOMIC: u32 = 0x00010000; /* Want/got atomic_write_* fields */
pub const STATX_DIO_READ_ALIGN: u32 = 0x00020000; /* Want/got dio read alignment info */

pub const STATX__RESERVED: u32 = 0x80000000; /* Reserved for future struct statx expansion */

/* C conditional: #ifndef __KERNEL__ */
/*
 * This is deprecated, and shall remain the same value in the future.  To avoid
 * confusion please use the equivalent (STATX_BASIC_STATS | STATX_BTIME)
 * instead.
 */
pub const STATX_ALL: u32 = 0x00000fff;

/*
 * Attributes to be found in stx_attributes and masked in stx_attributes_mask.
 *
 * These give information about the features or the state of a file that might
 * be of use to ordinary userspace programs such as GUIs or ls rather than
 * specialised tools.
 *
 * Note that the flags marked [I] correspond to the FS_IOC_SETFLAGS flags
 * semantically.  Where possible, the numerical value is picked to correspond
 * also.  Note that the DAX attribute indicates that the file is in the CPU
 * direct access state.  It does not correspond to the per-inode flag that
 * some filesystems support.
 *
 */
pub const STATX_ATTR_COMPRESSED: u64 = 0x00000004; /* [I] File is compressed by the fs */
pub const STATX_ATTR_IMMUTABLE: u64 = 0x00000010; /* [I] File is marked immutable */
pub const STATX_ATTR_APPEND: u64 = 0x00000020; /* [I] File is append-only */
pub const STATX_ATTR_NODUMP: u64 = 0x00000040; /* [I] File is not to be dumped */
pub const STATX_ATTR_ENCRYPTED: u64 = 0x00000800; /* [I] File requires key to decrypt in fs */
pub const STATX_ATTR_AUTOMOUNT: u64 = 0x00001000; /* Dir: Automount trigger */
pub const STATX_ATTR_MOUNT_ROOT: u64 = 0x00002000; /* Root of a mount */
pub const STATX_ATTR_VERITY: u64 = 0x00100000; /* [I] Verity protected file */
pub const STATX_ATTR_DAX: u64 = 0x00200000; /* File is currently in DAX state */
pub const STATX_ATTR_WRITE_ATOMIC: u64 = 0x00400000; /* File supports atomic write operations */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
