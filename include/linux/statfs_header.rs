/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency declarations supplied by the corresponding Linux type and
 * architecture headers are intentionally referenced rather than defined here. */

#[repr(C)]
pub struct kstatfs {
    pub f_type: ::core::ffi::c_long,
    pub f_bsize: ::core::ffi::c_long,
    pub f_blocks: u64,
    pub f_bfree: u64,
    pub f_bavail: u64,
    pub f_files: u64,
    pub f_ffree: u64,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: ::core::ffi::c_long,
    pub f_frsize: ::core::ffi::c_long,
    pub f_flags: ::core::ffi::c_long,
    pub f_spare: [::core::ffi::c_long; 4],
}

/* Definitions for the flag in f_flag.
 *
 * Generally these flags are equivalent to the MS_ flags used in the mount
 * ABI.  The exception is ST_VALID which has the same value as MS_REMOUNT
 * which doesn't make any sense for statfs.
 */
pub const ST_RDONLY: u32 = 0x0001; /* mount read-only */
pub const ST_NOSUID: u32 = 0x0002; /* ignore suid and sgid bits */
pub const ST_NODEV: u32 = 0x0004; /* disallow access to device special files */
pub const ST_NOEXEC: u32 = 0x0008; /* disallow program execution */
pub const ST_SYNCHRONOUS: u32 = 0x0010; /* writes are synced at once */
pub const ST_VALID: u32 = 0x0020; /* f_flags support is implemented */
pub const ST_MANDLOCK: u32 = 0x0040; /* allow mandatory locks on an FS */
/* 0x0080 used for ST_WRITE in glibc */
/* 0x0100 used for ST_APPEND in glibc */
/* 0x0200 used for ST_IMMUTABLE in glibc */
pub const ST_NOATIME: u32 = 0x0400; /* do not update access times */
pub const ST_NODIRATIME: u32 = 0x0800; /* do not update directory access times */
pub const ST_RELATIME: u32 = 0x1000; /* update atime relative to mtime/ctime */
pub const ST_NOSYMFOLLOW: u32 = 0x2000; /* do not follow symlinks */

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn vfs_get_fsid(dentry: *mut dentry, fsid: *mut __kernel_fsid_t) -> ::core::ffi::c_int;
    pub fn le64_to_cpup(p: *const ::core::ffi::c_void) -> u64;
}

#[inline]
pub unsafe fn u64_to_fsid(v: u64) -> __kernel_fsid_t {
    __kernel_fsid_t {
        val: [(v as u32), ((v >> 32) as u32)],
    }
}

/* Fold 16 bytes uuid to 64 bit fsid */
#[inline]
pub unsafe fn uuid_to_fsid(uuid: *mut u8) -> __kernel_fsid_t {
    u64_to_fsid(
        le64_to_cpup(uuid as *const ::core::ffi::c_void)
            ^ le64_to_cpup(uuid.add(::core::mem::size_of::<u64>()) as *const ::core::ffi::c_void),
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
