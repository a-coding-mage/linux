/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2005 Oracle.  All rights reserved.
 */

// Dependency intent preserved from the C header: linux/sched.h.

/* bits that are frequently given and infrequently matched in the low word */
/* NOTE: If you add a flag, you need to also update masklog.c! */
pub const ML_TCP: u64 = 0x0000_0000_0000_0001;
pub const ML_MSG: u64 = 0x0000_0000_0000_0002;
pub const ML_SOCKET: u64 = 0x0000_0000_0000_0004;
pub const ML_HEARTBEAT: u64 = 0x0000_0000_0000_0008;
pub const ML_HB_BIO: u64 = 0x0000_0000_0000_0010;
pub const ML_DLMFS: u64 = 0x0000_0000_0000_0020;
pub const ML_DLM: u64 = 0x0000_0000_0000_0040;
pub const ML_DLM_DOMAIN: u64 = 0x0000_0000_0000_0080;
pub const ML_DLM_THREAD: u64 = 0x0000_0000_0000_0100;
pub const ML_DLM_MASTER: u64 = 0x0000_0000_0000_0200;
pub const ML_DLM_RECOVERY: u64 = 0x0000_0000_0000_0400;
pub const ML_DLM_GLUE: u64 = 0x0000_0000_0000_0800;
pub const ML_VOTE: u64 = 0x0000_0000_0000_1000;
pub const ML_CONN: u64 = 0x0000_0000_0000_2000;
pub const ML_QUORUM: u64 = 0x0000_0000_0000_4000;
pub const ML_BASTS: u64 = 0x0000_0000_0000_8000;
pub const ML_CLUSTER: u64 = 0x0000_0000_0001_0000;

/* bits that are infrequently given and frequently matched in the high word */
pub const ML_ERROR: u64 = 0x1000_0000_0000_0000;
pub const ML_NOTICE: u64 = 0x2000_0000_0000_0000;
pub const ML_KTHREAD: u64 = 0x4000_0000_0000_0000;

pub const MLOG_INITIAL_AND_MASK: u64 = ML_ERROR | ML_NOTICE;
pub const MLOG_MASK_PREFIX: u64 = 0;

// Build-time CONFIG_OCFS2_DEBUG_MASKLOG intent from the C header.
#[cfg(feature = "CONFIG_OCFS2_DEBUG_MASKLOG")]
pub const ML_ALLOWED_BITS: u64 = !0u64;
#[cfg(not(feature = "CONFIG_OCFS2_DEBUG_MASKLOG"))]
pub const ML_ALLOWED_BITS: u64 = ML_ERROR | ML_NOTICE;

pub const MLOG_MAX_BITS: usize = 64;

#[repr(C)]
pub struct MlogBits {
    pub words: [core::ffi::c_ulong; MLOG_MAX_BITS / (core::mem::size_of::<core::ffi::c_ulong>() * 8)],
}

unsafe extern "C" {
    pub static mut mlog_and_bits: MlogBits;
    pub static mut mlog_not_bits: MlogBits;

    pub fn __mlog_printk(
        m: *const u64,
        func: *const core::ffi::c_char,
        line: core::ffi::c_int,
        fmt: *const core::ffi::c_char,
        ...,
    );
}

#[cfg(target_pointer_width = "32")]
#[inline]
pub unsafe fn __mlog_test_u64(mask: u64, bits: &MlogBits) -> bool {
    ((mask as u32 as core::ffi::c_ulong) & bits.words[0]) != 0
        || (((mask >> 32) as u64 as core::ffi::c_ulong) & bits.words[1]) != 0
}

#[cfg(not(target_pointer_width = "32"))]
#[inline]
pub unsafe fn __mlog_test_u64(mask: u64, bits: &MlogBits) -> bool {
    (mask as core::ffi::c_ulong & bits.words[0]) != 0
}

#[cfg(target_pointer_width = "32")]
#[macro_export]
macro_rules! __mlog_set_u64 { ($mask:expr, $bits:expr) => {{
    $bits.words[0] |= ($mask as u32 as core::ffi::c_ulong);
    $bits.words[1] |= (($mask as u64) >> 32) as core::ffi::c_ulong;
}} }

#[cfg(not(target_pointer_width = "32"))]
#[macro_export]
macro_rules! __mlog_set_u64 { ($mask:expr, $bits:expr) => {{ $bits.words[0] |= $mask as core::ffi::c_ulong; }} }

#[cfg(target_pointer_width = "32")]
#[macro_export]
macro_rules! __mlog_clear_u64 { ($mask:expr, $bits:expr) => {{
    $bits.words[0] &= !(($mask as u32) as core::ffi::c_ulong);
    $bits.words[1] &= !((($mask as u64) >> 32) as core::ffi::c_ulong);
}} }

#[cfg(not(target_pointer_width = "32"))]
#[macro_export]
macro_rules! __mlog_clear_u64 { ($mask:expr, $bits:expr) => {{ $bits.words[0] &= !($mask as core::ffi::c_ulong); }} }

#[macro_export]
macro_rules! mlog {
    ($mask:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        let _m: u64 = MLOG_MASK_PREFIX | ($mask as u64);
        if (_m & ML_ALLOWED_BITS) != 0 {
            $crate::__mlog_printk(&_m, concat!(module_path!(), "\0").as_ptr() as *const core::ffi::c_char, line!() as core::ffi::c_int, concat!($fmt, "\0").as_ptr() as *const core::ffi::c_char $(, $arg)*);
        }
    }};
}

// C mlog_ratelimited, mlog_errno, and mlog_bug_on_msg require kernel ratelimit,
// errno, and BUG dependencies supplied by the surrounding translation.

// Dependency intent preserved from the C header: linux/kobject.h and linux/sysfs.h.
#[repr(C)]
pub struct Kset { _private: [u8; 0] }

unsafe extern "C" {
    pub fn mlog_sys_init(o2cb_subsys: *mut Kset) -> core::ffi::c_int;
    pub fn mlog_sys_shutdown();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
