// SPDX-License-Identifier: GPL-2.0
/* Rust translation of dev_printk.h. */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

#[repr(C)]
pub struct device;

pub const PRINTK_INFO_SUBSYSTEM_LEN: usize = 16;
pub const PRINTK_INFO_DEVICE_LEN: usize = 48;

#[repr(C)]
pub struct dev_printk_info {
    pub subsystem: [core::ffi::c_char; PRINTK_INFO_SUBSYSTEM_LEN],
    pub device: [core::ffi::c_char; PRINTK_INFO_DEVICE_LEN],
}

#[cfg(feature = "CONFIG_PRINTK")]
extern "C" {
    pub fn dev_vprintk_emit(level: i32, dev: *const device, fmt: *const core::ffi::c_char, args: *mut core::ffi::c_void) -> i32;
    pub fn dev_printk_emit(level: i32, dev: *const device, fmt: *const core::ffi::c_char, ...) -> i32;
    pub fn _dev_printk(level: *const core::ffi::c_char, dev: *const device, fmt: *const core::ffi::c_char, ...);
    pub fn _dev_emerg(dev: *const device, fmt: *const core::ffi::c_char, ...);
    pub fn _dev_alert(dev: *const device, fmt: *const core::ffi::c_char, ...);
    pub fn _dev_crit(dev: *const device, fmt: *const core::ffi::c_char, ...);
    pub fn _dev_err(dev: *const device, fmt: *const core::ffi::c_char, ...);
    pub fn _dev_warn(dev: *const device, fmt: *const core::ffi::c_char, ...);
    pub fn _dev_notice(dev: *const device, fmt: *const core::ffi::c_char, ...);
    pub fn _dev_info(dev: *const device, fmt: *const core::ffi::c_char, ...);
}

#[cfg(not(feature = "CONFIG_PRINTK"))]
pub unsafe fn dev_vprintk_emit(_level: i32, _dev: *const device, _fmt: *const core::ffi::c_char, _args: *mut core::ffi::c_void) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_PRINTK"))]
pub unsafe fn dev_printk_emit(_level: i32, _dev: *const device, _fmt: *const core::ffi::c_char, _: ...) -> i32 { 0 }

#[cfg(not(feature = "CONFIG_PRINTK"))]
pub unsafe fn __dev_printk(_level: *const core::ffi::c_char, _dev: *const device, _vaf: *mut core::ffi::c_void) {}
#[cfg(not(feature = "CONFIG_PRINTK"))]
pub unsafe fn _dev_printk(_level: *const core::ffi::c_char, _dev: *const device, _fmt: *const core::ffi::c_char, _: ...) {}

#[cfg(not(feature = "CONFIG_PRINTK"))]
macro_rules! empty_dev_fn { ($name:ident) => { pub unsafe fn $name(_dev: *const device, _fmt: *const core::ffi::c_char, _: ...) {} }; }
#[cfg(not(feature = "CONFIG_PRINTK"))] empty_dev_fn!(_dev_emerg);
#[cfg(not(feature = "CONFIG_PRINTK"))] empty_dev_fn!(_dev_crit);
#[cfg(not(feature = "CONFIG_PRINTK"))] empty_dev_fn!(_dev_alert);
#[cfg(not(feature = "CONFIG_PRINTK"))] empty_dev_fn!(_dev_err);
#[cfg(not(feature = "CONFIG_PRINTK"))] empty_dev_fn!(_dev_warn);
#[cfg(not(feature = "CONFIG_PRINTK"))] empty_dev_fn!(_dev_notice);
#[cfg(not(feature = "CONFIG_PRINTK"))] empty_dev_fn!(_dev_info);

// C preprocessor logging macros, retained as Rust macros with equivalent
// argument forwarding and conditional intent.
#[macro_export]
macro_rules! dev_printk { ($level:expr, $dev:expr, $fmt:expr $(, $arg:expr)*) => {{ $crate::_dev_printk($level, $dev, $fmt $(, $arg)*); }}; }
#[macro_export]
macro_rules! dev_no_printk { ($level:expr, $dev:expr, $fmt:expr $(, $arg:expr)*) => {{ if false { $crate::_dev_printk($level, $dev, $fmt $(, $arg)*); } }}; }

#[macro_export] macro_rules! dev_emerg { ($dev:expr, $fmt:expr $(, $arg:expr)*) => { $crate::dev_printk!(KERN_EMERG, $dev, $fmt $(, $arg)*); }; }
#[macro_export] macro_rules! dev_crit { ($dev:expr, $fmt:expr $(, $arg:expr)*) => { $crate::dev_printk!(KERN_CRIT, $dev, $fmt $(, $arg)*); }; }
#[macro_export] macro_rules! dev_alert { ($dev:expr, $fmt:expr $(, $arg:expr)*) => { $crate::dev_printk!(KERN_ALERT, $dev, $fmt $(, $arg)*); }; }
#[macro_export] macro_rules! dev_err { ($dev:expr, $fmt:expr $(, $arg:expr)*) => { $crate::dev_printk!(KERN_ERR, $dev, $fmt $(, $arg)*); }; }
#[macro_export] macro_rules! dev_warn { ($dev:expr, $fmt:expr $(, $arg:expr)*) => { $crate::dev_printk!(KERN_WARNING, $dev, $fmt $(, $arg)*); }; }
#[macro_export] macro_rules! dev_notice { ($dev:expr, $fmt:expr $(, $arg:expr)*) => { $crate::dev_printk!(KERN_NOTICE, $dev, $fmt $(, $arg)*); }; }
#[macro_export] macro_rules! dev_info { ($dev:expr, $fmt:expr $(, $arg:expr)*) => { $crate::dev_printk!(KERN_INFO, $dev, $fmt $(, $arg)*); }; }

#[cfg(feature = "DEBUG")]
#[macro_export] macro_rules! dev_dbg { ($dev:expr, $fmt:expr $(, $arg:expr)*) => { $crate::dev_printk!(KERN_DEBUG, $dev, $fmt $(, $arg)*); }; }
#[cfg(not(feature = "DEBUG"))]
#[macro_export] macro_rules! dev_dbg { ($dev:expr, $fmt:expr $(, $arg:expr)*) => { $crate::dev_no_printk!(KERN_DEBUG, $dev, $fmt $(, $arg)*); }; }

#[macro_export] macro_rules! dev_emerg_once { ($($t:tt)*) => { $crate::dev_emerg!($($t)*); }; }
#[macro_export] macro_rules! dev_alert_once { ($($t:tt)*) => { $crate::dev_alert!($($t)*); }; }
#[macro_export] macro_rules! dev_crit_once { ($($t:tt)*) => { $crate::dev_crit!($($t)*); }; }
#[macro_export] macro_rules! dev_err_once { ($($t:tt)*) => { $crate::dev_err!($($t)*); }; }
#[macro_export] macro_rules! dev_warn_once { ($($t:tt)*) => { $crate::dev_warn!($($t)*); }; }
#[macro_export] macro_rules! dev_notice_once { ($($t:tt)*) => { $crate::dev_notice!($($t)*); }; }
#[macro_export] macro_rules! dev_info_once { ($($t:tt)*) => { $crate::dev_info!($($t)*); }; }
#[macro_export] macro_rules! dev_dbg_once { ($($t:tt)*) => { $crate::dev_dbg!($($t)*); }; }

extern "C" {
    pub fn dev_err_probe(dev: *const device, err: i32, fmt: *const core::ffi::c_char, ...) -> i32;
    pub fn dev_warn_probe(dev: *const device, err: i32, fmt: *const core::ffi::c_char, ...) -> i32;
}

#[macro_export]
macro_rules! dev_err_ptr_probe { ($dev:expr, $err:expr, $fmt:expr $(, $arg:expr)*) => { ERR_PTR($crate::dev_err_probe($dev, $err, $fmt $(, $arg)*)) }; }
#[macro_export]
macro_rules! dev_err_cast_probe { ($dev:expr, $err_ptr:expr, $fmt:expr $(, $arg:expr)*) => { ERR_PTR($crate::dev_err_probe($dev, PTR_ERR($err_ptr), $fmt $(, $arg)*)) }; }

// Remaining source macros preserve the original conditional and forwarding intent.
#[macro_export]
macro_rules! dev_printk_index_emit { ($level:expr, $fmt:expr $(, $arg:expr)*) => {{ printk_index_subsys_emit!("%s %s: ", $level, $fmt $(, $arg)*); }}; }
#[macro_export]
macro_rules! dev_printk_index_wrap { ($func:ident, $level:expr, $dev:expr, $fmt:expr $(, $arg:expr)*) => {{ $crate::dev_printk_index_emit!($level, $fmt); $func($dev, $fmt $(, $arg)*); }}; }
#[macro_export]
macro_rules! dev_level_once { ($level:ident, $dev:expr, $fmt:expr $(, $arg:expr)*) => {{ static mut __PRINT_ONCE: bool = false; unsafe { if !__PRINT_ONCE { __PRINT_ONCE = true; $crate::$level!($dev, $fmt $(, $arg)*); } } }}; }
#[macro_export]
macro_rules! dev_level_ratelimited { ($level:ident, $dev:expr, $fmt:expr $(, $arg:expr)*) => {{ if __ratelimit(&_rs) { $crate::$level!($dev, $fmt $(, $arg)*); } }}; }
#[macro_export] macro_rules! dev_emerg_ratelimited { ($($t:tt)*) => { $crate::dev_level_ratelimited!(dev_emerg, $($t)*); }; }
#[macro_export] macro_rules! dev_alert_ratelimited { ($($t:tt)*) => { $crate::dev_level_ratelimited!(dev_alert, $($t)*); }; }
#[macro_export] macro_rules! dev_crit_ratelimited { ($($t:tt)*) => { $crate::dev_level_ratelimited!(dev_crit, $($t)*); }; }
#[macro_export] macro_rules! dev_err_ratelimited { ($($t:tt)*) => { $crate::dev_level_ratelimited!(dev_err, $($t)*); }; }
#[macro_export] macro_rules! dev_warn_ratelimited { ($($t:tt)*) => { $crate::dev_level_ratelimited!(dev_warn, $($t)*); }; }
#[macro_export] macro_rules! dev_notice_ratelimited { ($($t:tt)*) => { $crate::dev_level_ratelimited!(dev_notice, $($t)*); }; }
#[macro_export] macro_rules! dev_info_ratelimited { ($($t:tt)*) => { $crate::dev_level_ratelimited!(dev_info, $($t)*); }; }
#[macro_export] macro_rules! dev_dbg_ratelimited { ($($t:tt)*) => { $crate::dev_level_ratelimited!(dev_dbg, $($t)*); }; }
#[macro_export] macro_rules! dev_vdbg { ($($t:tt)*) => { $crate::dev_dbg!($($t)*); }; }
#[macro_export] macro_rules! dev_WARN { ($dev:expr, $format:expr $(, $arg:expr)*) => { WARN!(1, "%s %s: " $format, dev_driver_string($dev), dev_name($dev) $(, $arg)*); }; }
#[macro_export] macro_rules! dev_WARN_ONCE { ($dev:expr, $condition:expr, $format:expr $(, $arg:expr)*) => { WARN_ONCE!($condition, "%s %s: " $format, dev_driver_string($dev), dev_name($dev) $(, $arg)*); }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
