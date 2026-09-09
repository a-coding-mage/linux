/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding asm/uapi and Linux headers are
// intentionally referenced here rather than redefined.

pub const S_IRWXUGO: _ = S_IRWXU | S_IRWXG | S_IRWXO;
pub const S_IALLUGO: _ = S_ISUID | S_ISGID | S_ISVTX | S_IRWXUGO;
pub const S_IRUGO: _ = S_IRUSR | S_IRGRP | S_IROTH;
pub const S_IWUGO: _ = S_IWUSR | S_IWGRP | S_IWOTH;
pub const S_IXUGO: _ = S_IXUSR | S_IXGRP | S_IXOTH;

pub const UTIME_NOW: i64 = (1i64 << 30) - 1i64;
pub const UTIME_OMIT: i64 = (1i64 << 30) - 2i64;

#[repr(C)]
pub struct kstat {
    pub result_mask: u32, // What fields the user got
    pub mode: umode_t,
    pub nlink: ::core::ffi::c_uint,
    pub blksize: u32, // Preferred I/O size
    pub attributes: u64,
    pub attributes_mask: u64,
    pub ino: u64,
    pub dev: dev_t,
    pub rdev: dev_t,
    pub uid: kuid_t, // This is logically a vfsuid_t.
    pub gid: kgid_t, // This is logically a vfsgid_t.
    pub size: loff_t,
    pub atime: timespec64,
    pub mtime: timespec64,
    pub ctime: timespec64,
    pub btime: timespec64, // File creation time
    pub blocks: u64,
    pub mnt_id: u64,
    pub change_cookie: u64,
    pub subvol: u64,
    pub dio_mem_align: u32,
    pub dio_offset_align: u32,
    pub dio_read_offset_align: u32,
    pub atomic_write_unit_min: u32,
    pub atomic_write_unit_max: u32,
    pub atomic_write_unit_max_opt: u32,
    pub atomic_write_segments_max: u32,
}

// These definitions are internal to the kernel for now. Mainly used by nfsd.

// mask values
pub const STATX_CHANGE_COOKIE: u32 = 0x40000000u32; // Want/got stx_change_attr

// file attribute values
pub const STATX_ATTR_CHANGE_MONOTONIC: u64 = 0x8000000000000000u64; // version monotonically increases

pub const KSTAT_ATTR_FS_IOC_FLAGS: _ = STATX_ATTR_COMPRESSED
    | STATX_ATTR_IMMUTABLE
    | STATX_ATTR_APPEND
    | STATX_ATTR_NODUMP
    | STATX_ATTR_ENCRYPTED
    | STATX_ATTR_VERITY; // Attrs corresponding to FS_*_FL flags

pub const KSTAT_ATTR_VFS_FLAGS: _ = STATX_ATTR_IMMUTABLE | STATX_ATTR_APPEND;
// Attrs corresponding to S_* flags that are enforced by the VFS

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
