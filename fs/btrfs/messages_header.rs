/* SPDX-License-Identifier: GPL-2.0 */

// Translated from btrfs/messages.h. C build-time configuration conditions are
// preserved below as Rust cfg conditions where a direct mapping is possible.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub enum btrfs_fs_info {}

#[cfg(feature = "kernel")]
#[inline]
pub unsafe fn btrfs_no_printk(_fs_info: *const btrfs_fs_info, _fmt: *const c_char, _args: ...) {}

#[cfg(feature = "config_printk")]
extern "C" {
    pub fn _btrfs_printk(
        fs_info: *const btrfs_fs_info,
        level: c_uint,
        fmt: *const c_char,
        ...,
    );
}

// The original CONFIG_PRINTK-disabled macros call btrfs_no_printk.
#[cfg(not(feature = "config_printk"))]
#[macro_export]
macro_rules! btrfs_printk_in_rcu {
    ($fs_info:expr, $level:expr, $fmt:expr $(, $args:expr)*) => {
        unsafe { $crate::btrfs_no_printk($fs_info, $fmt $(, $args)*) }
    };
}
#[cfg(not(feature = "config_printk"))]
#[macro_export]
macro_rules! btrfs_printk_rl_in_rcu {
    ($fs_info:expr, $level:expr, $fmt:expr $(, $args:expr)*) => {
        unsafe { $crate::btrfs_no_printk($fs_info, $fmt $(, $args)*) }
    };
}

// Print a message with filesystem info, enclosed in RCU protection.
#[macro_export]
macro_rules! btrfs_crit { ($fs_info:expr, $fmt:expr $(, $args:expr)*) => { $crate::btrfs_printk_in_rcu!($fs_info, LOGLEVEL_CRIT, $fmt $(, $args)*) }; }
#[macro_export]
macro_rules! btrfs_err { ($fs_info:expr, $fmt:expr $(, $args:expr)*) => { $crate::btrfs_printk_in_rcu!($fs_info, LOGLEVEL_ERR, $fmt $(, $args)*) }; }
#[macro_export]
macro_rules! btrfs_warn { ($fs_info:expr, $fmt:expr $(, $args:expr)*) => { $crate::btrfs_printk_in_rcu!($fs_info, LOGLEVEL_WARNING, $fmt $(, $args)*) }; }
#[macro_export]
macro_rules! btrfs_info { ($fs_info:expr, $fmt:expr $(, $args:expr)*) => { $crate::btrfs_printk_in_rcu!($fs_info, LOGLEVEL_INFO, $fmt $(, $args)*) }; }

// Wrappers that use a ratelimited printk
#[macro_export]
macro_rules! btrfs_crit_rl { ($fs_info:expr, $fmt:expr $(, $args:expr)*) => { $crate::btrfs_printk_rl_in_rcu!($fs_info, LOGLEVEL_CRIT, $fmt $(, $args)*) }; }
#[macro_export]
macro_rules! btrfs_err_rl { ($fs_info:expr, $fmt:expr $(, $args:expr)*) => { $crate::btrfs_printk_rl_in_rcu!($fs_info, LOGLEVEL_ERR, $fmt $(, $args)*) }; }
#[macro_export]
macro_rules! btrfs_warn_rl { ($fs_info:expr, $fmt:expr $(, $args:expr)*) => { $crate::btrfs_printk_rl_in_rcu!($fs_info, LOGLEVEL_WARNING, $fmt $(, $args)*) }; }
#[macro_export]
macro_rules! btrfs_info_rl { ($fs_info:expr, $fmt:expr $(, $args:expr)*) => { $crate::btrfs_printk_rl_in_rcu!($fs_info, LOGLEVEL_INFO, $fmt $(, $args)*) }; }

#[cfg(feature = "debug")]
#[macro_export]
macro_rules! btrfs_debug { ($fs_info:expr, $fmt:expr $(, $args:expr)*) => { $crate::btrfs_printk_in_rcu!($fs_info, LOGLEVEL_DEBUG, $fmt $(, $args)*) }; }
#[cfg(feature = "debug")]
#[macro_export]
macro_rules! btrfs_debug_rl { ($fs_info:expr, $fmt:expr $(, $args:expr)*) => { $crate::btrfs_printk_rl_in_rcu!($fs_info, LOGLEVEL_DEBUG, $fmt $(, $args)*) }; }
#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules! btrfs_debug { ($fs_info:expr, $fmt:expr $(, $args:expr)*) => { { let _ = &$fs_info; } }; }
#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules! btrfs_debug_rl { ($fs_info:expr, $fmt:expr $(, $args:expr)*) => { { let _ = &$fs_info; } }; }

#[cfg(feature = "config_printk")]
#[macro_export]
macro_rules! btrfs_printk_in_rcu { ($fs_info:expr, $level:expr, $fmt:expr $(, $args:expr)*) => {{ unsafe { _btrfs_printk($fs_info, $level, $fmt $(, $args)*); } }}; }
#[cfg(feature = "config_printk")]
#[macro_export]
macro_rules! btrfs_printk_rl_in_rcu { ($fs_info:expr, $level:expr, $fmt:expr $(, $args:expr)*) => {{ unsafe { _btrfs_printk($fs_info, $level, $fmt $(, $args)*); } }}; }

#[cfg(feature = "config_btrfs_assert")]
#[inline]
pub unsafe fn verify_assert_printk_format(_fmt: *const c_char, _args: ...) {}

// Assertion formatting and BUG/BUILD_BUG_ON_INVALID depend on kernel macros;
// preserve their call shape as Rust macro interfaces.
#[cfg(feature = "config_btrfs_assert")]
#[macro_export]
macro_rules! ASSERT {
    ($cond:expr $(, $args:tt)*) => {{ if !$cond { unsafe { BUG(); } } }};
}
#[cfg(not(feature = "config_btrfs_assert"))]
#[macro_export]
macro_rules! ASSERT { ($cond:expr $(, $args:tt)*) => {{ let _ = &$cond; }}; }

#[cfg(feature = "config_btrfs_debug")]
#[macro_export]
macro_rules! DEBUG_WARN { ($($args:tt)*) => { WARN!(1, KERN_ERR $($args)*) }; }
#[cfg(not(feature = "config_btrfs_debug"))]
#[macro_export]
macro_rules! DEBUG_WARN { ($($args:tt)*) => {}; }

extern "C" {
    pub fn __btrfs_handle_fs_error(
        fs_info: *mut btrfs_fs_info, function: *const c_char, line: c_uint,
        error: c_int, fmt: *const c_char, ...);
    pub fn btrfs_decode_error(error: c_int) -> *const c_char;
    pub fn __btrfs_panic(
        fs_info: *const btrfs_fs_info, function: *const c_char, line: c_uint,
        error: c_int, fmt: *const c_char, ...);
}

#[macro_export]
macro_rules! btrfs_handle_fs_error { ($fs_info:expr, $error:expr, $fmt:expr $(, $args:expr)*) => { unsafe { __btrfs_handle_fs_error($fs_info, core::ptr::null(), 0, $error, $fmt $(, $args)*); } }; }
#[macro_export]
macro_rules! btrfs_panic { ($fs_info:expr, $error:expr, $fmt:expr $(, $args:expr)*) => {{ unsafe { __btrfs_panic($fs_info, core::ptr::null(), 0, $error, $fmt $(, $args)*); BUG(); } }}; }

#[cfg(target_pointer_width = "32")]
pub const BTRFS_32BIT_MAX_FILE_SIZE: u64 = ((c_ulong::MAX as u64 + 1) << PAGE_SHIFT);
#[cfg(target_pointer_width = "32")]
pub const BTRFS_32BIT_EARLY_WARN_THRESHOLD: u64 = BTRFS_32BIT_MAX_FILE_SIZE * 5 / 8;
#[cfg(target_pointer_width = "32")]
extern "C" {
    pub fn btrfs_warn_32bit_limit(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_err_32bit_limit(fs_info: *mut btrfs_fs_info);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
