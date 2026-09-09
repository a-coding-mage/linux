/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 1999 by Ralf Baechle
 */

// C dependencies: linux/posix_types.h, asm/sgidefs.h, and linux/types.h.
// The __KERNEL_STRICT_NAMES conditional controls whether fsid_t is exported.

#[cfg(not(feature = "__KERNEL_STRICT_NAMES"))]
pub type fsid_t = __kernel_fsid_t;

#[repr(C)]
pub struct statfs {
    pub f_type: core::ffi::c_long,
    // #define f_fstyp f_type
    pub f_bsize: core::ffi::c_long,
    pub f_frsize: core::ffi::c_long, // Fragment size - unsupported
    pub f_blocks: core::ffi::c_long,
    pub f_bfree: core::ffi::c_long,
    pub f_files: core::ffi::c_long,
    pub f_ffree: core::ffi::c_long,
    pub f_bavail: core::ffi::c_long,

    // Linux specials
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: core::ffi::c_long,
    pub f_flags: core::ffi::c_long,
    pub f_spare: [core::ffi::c_long; 5],
}

// Corresponds to:
// #if (_MIPS_SIM == _MIPS_SIM_ABI32) || (_MIPS_SIM == _MIPS_SIM_NABI32)
#[cfg(any(feature = "mips_abi32", feature = "mips_nabi32"))]
#[repr(C)]
pub struct statfs64 {
    pub f_type: u32,
    pub f_bsize: u32,
    pub f_frsize: u32, // Fragment size - unsupported
    pub __pad: u32,
    pub f_blocks: u64,
    pub f_bfree: u64,
    pub f_files: u64,
    pub f_ffree: u64,
    pub f_bavail: u64,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: u32,
    pub f_flags: u32,
    pub f_spare: [u32; 5],
}

// Corresponds to: #if _MIPS_SIM == _MIPS_SIM_ABI64
#[cfg(feature = "mips_abi64")]
#[repr(C)]
pub struct statfs64 {
    pub f_type: core::ffi::c_long,
    pub f_bsize: core::ffi::c_long,
    pub f_frsize: core::ffi::c_long, // Fragment size - unsupported
    pub f_blocks: core::ffi::c_long,
    pub f_bfree: core::ffi::c_long,
    pub f_files: core::ffi::c_long,
    pub f_ffree: core::ffi::c_long,
    pub f_bavail: core::ffi::c_long,

    // Linux specials
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: core::ffi::c_long,
    pub f_flags: core::ffi::c_long,
    pub f_spare: [core::ffi::c_long; 5],
}

#[cfg(feature = "mips_abi64")]
#[repr(C)]
pub struct compat_statfs64 {
    pub f_type: u32,
    pub f_bsize: u32,
    pub f_frsize: u32, // Fragment size - unsupported
    pub __pad: u32,
    pub f_blocks: u64,
    pub f_bfree: u64,
    pub f_files: u64,
    pub f_ffree: u64,
    pub f_bavail: u64,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: u32,
    pub f_flags: u32,
    pub f_spare: [u32; 5],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
