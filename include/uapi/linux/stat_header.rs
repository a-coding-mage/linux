/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: Linux integer types (__s16/__s32/__s64/__u16/__u32/__u64)
// are supplied by the corresponding translated Linux types module.

// These definitions are conditional in the C header for kernel, non-glibc,
// and older glibc builds; the condition is retained here as source intent.
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

#[inline]
pub const fn S_ISLNK(m: u32) -> bool { (m & S_IFMT) == S_IFLNK }
#[inline]
pub const fn S_ISREG(m: u32) -> bool { (m & S_IFMT) == S_IFREG }
#[inline]
pub const fn S_ISDIR(m: u32) -> bool { (m & S_IFMT) == S_IFDIR }
#[inline]
pub const fn S_ISCHR(m: u32) -> bool { (m & S_IFMT) == S_IFCHR }
#[inline]
pub const fn S_ISBLK(m: u32) -> bool { (m & S_IFMT) == S_IFBLK }
#[inline]
pub const fn S_ISFIFO(m: u32) -> bool { (m & S_IFMT) == S_IFIFO }
#[inline]
pub const fn S_ISSOCK(m: u32) -> bool { (m & S_IFMT) == S_IFSOCK }

pub const S_IRWXU: u32 = 0o700;
pub const S_IRUSR: u32 = 0o400;
pub const S_IWUSR: u32 = 0o200;
pub const S_IXUSR: u32 = 0o100;
pub const S_IRWXG: u32 = 0o070;
pub const S_IRGRP: u32 = 0o040;
pub const S_IWGRP: u32 = 0o020;
pub const S_IXGRP: u32 = 0o010;
pub const S_IRWXO: u32 = 0o007;
pub const S_IROTH: u32 = 0o004;
pub const S_IWOTH: u32 = 0o002;
pub const S_IXOTH: u32 = 0o001;

#[repr(C)]
pub struct statx_timestamp {
    pub tv_sec: __s64,
    pub tv_nsec: __u32,
    pub __reserved: __s32,
}

#[repr(C)]
pub struct statx {
    pub stx_mask: __u32,
    pub stx_blksize: __u32,
    pub stx_attributes: __u64,
    pub stx_nlink: __u32,
    pub stx_uid: __u32,
    pub stx_gid: __u32,
    pub stx_mode: __u16,
    pub __spare0: [__u16; 1],
    pub stx_ino: __u64,
    pub stx_size: __u64,
    pub stx_blocks: __u64,
    pub stx_attributes_mask: __u64,
    pub stx_atime: statx_timestamp,
    pub stx_btime: statx_timestamp,
    pub stx_ctime: statx_timestamp,
    pub stx_mtime: statx_timestamp,
    pub stx_rdev_major: __u32,
    pub stx_rdev_minor: __u32,
    pub stx_dev_major: __u32,
    pub stx_dev_minor: __u32,
    pub stx_mnt_id: __u64,
    pub stx_dio_mem_align: __u32,
    pub stx_dio_offset_align: __u32,
    pub stx_subvol: __u64,
    pub stx_atomic_write_unit_min: __u32,
    pub stx_atomic_write_unit_max: __u32,
    pub stx_atomic_write_segments_max: __u32,
    pub stx_dio_read_offset_align: __u32,
    pub stx_atomic_write_unit_max_opt: __u32,
    pub __spare2: [__u32; 1],
    pub __spare3: [__u64; 8],
}

pub const STATX_TYPE: __u32 = 0x00000001;
pub const STATX_MODE: __u32 = 0x00000002;
pub const STATX_NLINK: __u32 = 0x00000004;
pub const STATX_UID: __u32 = 0x00000008;
pub const STATX_GID: __u32 = 0x00000010;
pub const STATX_ATIME: __u32 = 0x00000020;
pub const STATX_MTIME: __u32 = 0x00000040;
pub const STATX_CTIME: __u32 = 0x00000080;
pub const STATX_INO: __u32 = 0x00000100;
pub const STATX_SIZE: __u32 = 0x00000200;
pub const STATX_BLOCKS: __u32 = 0x00000400;
pub const STATX_BASIC_STATS: __u32 = 0x000007ff;
pub const STATX_BTIME: __u32 = 0x00000800;
pub const STATX_MNT_ID: __u32 = 0x00001000;
pub const STATX_DIOALIGN: __u32 = 0x00002000;
pub const STATX_MNT_ID_UNIQUE: __u32 = 0x00004000;
pub const STATX_SUBVOL: __u32 = 0x00008000;
pub const STATX_WRITE_ATOMIC: __u32 = 0x00010000;
pub const STATX_DIO_READ_ALIGN: __u32 = 0x00020000;
pub const STATX__RESERVED: __u32 = 0x80000000;

// Available only when __KERNEL__ is not defined in the C header.
pub const STATX_ALL: __u32 = 0x00000fff;

pub const STATX_ATTR_COMPRESSED: __u64 = 0x00000004;
pub const STATX_ATTR_IMMUTABLE: __u64 = 0x00000010;
pub const STATX_ATTR_APPEND: __u64 = 0x00000020;
pub const STATX_ATTR_NODUMP: __u64 = 0x00000040;
pub const STATX_ATTR_ENCRYPTED: __u64 = 0x00000800;
pub const STATX_ATTR_AUTOMOUNT: __u64 = 0x00001000;
pub const STATX_ATTR_MOUNT_ROOT: __u64 = 0x00002000;
pub const STATX_ATTR_VERITY: __u64 = 0x00100000;
pub const STATX_ATTR_DAX: __u64 = 0x00200000;
pub const STATX_ATTR_WRITE_ATOMIC: __u64 = 0x00400000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
