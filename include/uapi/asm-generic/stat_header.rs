/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Everybody gets this wrong and has to stick with it for all
 * eternity. Hopefully, this version gets used by new architectures
 * so they don't fall into the same traps.
 *
 * stat64 is copied from powerpc64, with explicit padding added.
 * stat is the same structure layout on 64-bit, without the 'long long'
 * types.
 *
 * By convention, 64 bit architectures use the stat interface, while
 * 32 bit architectures use the stat64 interface. Note that we don't
 * provide an __old_kernel_stat here, which new architecture should
 * not have to start with.
 */

// Dependency supplied by the target environment: <asm/bitsperlong.h>.

pub const STAT_HAVE_NSEC: u32 = 1;

#[repr(C)]
pub struct stat {
    pub st_dev: ::core::ffi::c_ulong,       /* Device.  */
    pub st_ino: ::core::ffi::c_ulong,       /* File serial number.  */
    pub st_mode: ::core::ffi::c_uint,       /* File mode.  */
    pub st_nlink: ::core::ffi::c_uint,      /* Link count.  */
    pub st_uid: ::core::ffi::c_uint,        /* User ID of the file's owner.  */
    pub st_gid: ::core::ffi::c_uint,        /* Group ID of the file's group. */
    pub st_rdev: ::core::ffi::c_ulong,      /* Device number, if device.  */
    pub __pad1: ::core::ffi::c_ulong,
    pub st_size: ::core::ffi::c_long,       /* Size of file, in bytes.  */
    pub st_blksize: ::core::ffi::c_int,     /* Optimal block size for I/O.  */
    pub __pad2: ::core::ffi::c_int,
    pub st_blocks: ::core::ffi::c_long,     /* Number 512-byte blocks allocated. */
    pub st_atime: ::core::ffi::c_long,      /* Time of last access.  */
    pub st_atime_nsec: ::core::ffi::c_ulong,
    pub st_mtime: ::core::ffi::c_long,      /* Time of last modification.  */
    pub st_mtime_nsec: ::core::ffi::c_ulong,
    pub st_ctime: ::core::ffi::c_long,      /* Time of last status change.  */
    pub st_ctime_nsec: ::core::ffi::c_ulong,
    pub __unused4: ::core::ffi::c_uint,
    pub __unused5: ::core::ffi::c_uint,
}

/* This matches struct stat64 in glibc2.1. Only used for 32 bit. */
/* C condition: __BITS_PER_LONG != 64 || defined(__ARCH_WANT_STAT64). */
#[cfg(any(not(target_pointer_width = "64"), feature = "__ARCH_WANT_STAT64"))]
#[repr(C)]
pub struct stat64 {
    pub st_dev: ::core::ffi::c_ulonglong,    /* Device.  */
    pub st_ino: ::core::ffi::c_ulonglong,    /* File serial number.  */
    pub st_mode: ::core::ffi::c_uint,        /* File mode.  */
    pub st_nlink: ::core::ffi::c_uint,       /* Link count.  */
    pub st_uid: ::core::ffi::c_uint,         /* User ID of the file's owner.  */
    pub st_gid: ::core::ffi::c_uint,         /* Group ID of the file's group. */
    pub st_rdev: ::core::ffi::c_ulonglong,   /* Device number, if device.  */
    pub __pad1: ::core::ffi::c_ulonglong,
    pub st_size: ::core::ffi::c_longlong,    /* Size of file, in bytes.  */
    pub st_blksize: ::core::ffi::c_int,      /* Optimal block size for I/O.  */
    pub __pad2: ::core::ffi::c_int,
    pub st_blocks: ::core::ffi::c_longlong,  /* Number 512-byte blocks allocated. */
    pub st_atime: ::core::ffi::c_int,        /* Time of last access.  */
    pub st_atime_nsec: ::core::ffi::c_uint,
    pub st_mtime: ::core::ffi::c_int,        /* Time of last modification.  */
    pub st_mtime_nsec: ::core::ffi::c_uint,
    pub st_ctime: ::core::ffi::c_int,        /* Time of last status change.  */
    pub st_ctime_nsec: ::core::ffi::c_uint,
    pub __unused4: ::core::ffi::c_uint,
    pub __unused5: ::core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
