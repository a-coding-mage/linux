/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  S390 version
 *
 *  Derived from "include/asm-i386/statfs.h"
 */

/*
 * We can't use <asm-generic/statfs.h> because in 64-bit mode
 * we mix ints of different sizes in our struct statfs.
 */

/* The C header includes <linux/types.h> and aliases __kernel_fsid_t when
 * __KERNEL_STRICT_NAMES is not defined. The dependency is supplied elsewhere.
 */
#[cfg(not(__KERNEL_STRICT_NAMES))]
pub type fsid_t = __kernel_fsid_t;

#[repr(C)]
pub struct statfs {
    pub f_type: core::ffi::c_uint,
    pub f_bsize: core::ffi::c_uint,
    pub f_blocks: core::ffi::c_ulong,
    pub f_bfree: core::ffi::c_ulong,
    pub f_bavail: core::ffi::c_ulong,
    pub f_files: core::ffi::c_ulong,
    pub f_ffree: core::ffi::c_ulong,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: core::ffi::c_uint,
    pub f_frsize: core::ffi::c_uint,
    pub f_flags: core::ffi::c_uint,
    pub f_spare: [core::ffi::c_uint; 5],
}

#[repr(C)]
pub struct statfs64 {
    pub f_type: core::ffi::c_uint,
    pub f_bsize: core::ffi::c_uint,
    pub f_blocks: core::ffi::c_ulonglong,
    pub f_bfree: core::ffi::c_ulonglong,
    pub f_bavail: core::ffi::c_ulonglong,
    pub f_files: core::ffi::c_ulonglong,
    pub f_ffree: core::ffi::c_ulonglong,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: core::ffi::c_uint,
    pub f_frsize: core::ffi::c_uint,
    pub f_flags: core::ffi::c_uint,
    pub f_spare: [core::ffi::c_uint; 5],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
