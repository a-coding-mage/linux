/* SPDX-License-Identifier: GPL-2.0 */

// The Linux once_lite definitions and kernel logging/rate-limit symbols are
// supplied by other translation units.

#[repr(C)]
pub struct xfs_mount {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_buf {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn xfs_printk_level(
        kern_level: *const ::core::ffi::c_char,
        mp: *const xfs_mount,
        fmt: *const ::core::ffi::c_char,
        ...,
    );

    pub fn _xfs_alert_tag(
        mp: *const xfs_mount,
        tag: u32,
        fmt: *const ::core::ffi::c_char,
        ...,
    );

    pub fn assfail(
        mp: *mut xfs_mount,
        expr: *mut ::core::ffi::c_char,
        f: *mut ::core::ffi::c_char,
        l: ::core::ffi::c_int,
    );
    pub fn asswarn(
        mp: *mut xfs_mount,
        expr: *mut ::core::ffi::c_char,
        f: *mut ::core::ffi::c_char,
        l: ::core::ffi::c_int,
    );

    pub fn xfs_hex_dump(p: *const ::core::ffi::c_void, length: ::core::ffi::c_int);

    pub fn xfs_buf_alert_ratelimited(
        bp: *mut xfs_buf,
        rlmsg: *const ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...,
    );
}

#[macro_export]
macro_rules! xfs_printk_index_wrap {
    ($kern_level:expr, $mp:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            printk_index_subsys_emit!(concat!("%sXFS%s: ", $kern_level, $fmt) $(, $arg)*);
            $crate::xfs_printk_level($kern_level, $mp, $fmt $(, $arg)*);
        }
    }};
}

#[macro_export]
macro_rules! xfs_emerg { ($mp:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { $crate::xfs_printk_index_wrap!(KERN_EMERG, $mp, $fmt $(, $arg)*); }; }
#[macro_export]
macro_rules! xfs_alert { ($mp:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { $crate::xfs_printk_index_wrap!(KERN_ALERT, $mp, $fmt $(, $arg)*); }; }
#[macro_export]
macro_rules! xfs_crit { ($mp:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { $crate::xfs_printk_index_wrap!(KERN_CRIT, $mp, $fmt $(, $arg)*); }; }
#[macro_export]
macro_rules! xfs_err { ($mp:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { $crate::xfs_printk_index_wrap!(KERN_ERR, $mp, $fmt $(, $arg)*); }; }
#[macro_export]
macro_rules! xfs_warn { ($mp:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { $crate::xfs_printk_index_wrap!(KERN_WARNING, $mp, $fmt $(, $arg)*); }; }
#[macro_export]
macro_rules! xfs_notice { ($mp:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { $crate::xfs_printk_index_wrap!(KERN_NOTICE, $mp, $fmt $(, $arg)*); }; }
#[macro_export]
macro_rules! xfs_info { ($mp:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { $crate::xfs_printk_index_wrap!(KERN_INFO, $mp, $fmt $(, $arg)*); }; }

// DEBUG is a build-time condition from the C header.
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! xfs_debug { ($mp:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { $crate::xfs_printk_index_wrap!(KERN_DEBUG, $mp, $fmt $(, $arg)*); }; }
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! xfs_debug { ($mp:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{ let _ = &$mp; let _ = &$fmt; $(let _ = &$arg;)* }}; }

#[macro_export]
macro_rules! xfs_alert_tag {
    ($mp:expr, $tag:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            printk_index_subsys_emit!(concat!("%sXFS%s: ", KERN_ALERT, $fmt) $(, $arg)*);
            $crate::_xfs_alert_tag($mp, $tag, $fmt $(, $arg)*);
        }
    }};
}

#[macro_export]
macro_rules! xfs_printk_ratelimited {
    ($func:ident, $dev:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        static mut _RS: Option<RateLimitState> = None;
        unsafe {
            if __ratelimit(&mut _RS, DEFAULT_RATELIMIT_INTERVAL, DEFAULT_RATELIMIT_BURST) {
                $crate::$func!($dev, $fmt $(, $arg)*);
            }
        }
    }};
}

#[macro_export]
macro_rules! xfs_printk_once { ($func:ident, $dev:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { DO_ONCE_LITE!($crate::$func, $dev, $fmt $(, $arg)*); }; }

macro_rules! xfs_printk_ratelimited_wrappers { ($($name:ident),*) => { $(
    #[macro_export]
    macro_rules! $name { ($dev:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { $crate::xfs_printk_ratelimited!($name, $dev, $fmt $(, $arg)*); }; }
)* }; }

// Per-level wrappers retain the original C macro interface.
#[macro_export] macro_rules! xfs_emerg_ratelimited { ($dev:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { xfs_printk_ratelimited!(xfs_emerg, $dev, $fmt $(, $arg)*); }; }
#[macro_export] macro_rules! xfs_alert_ratelimited { ($dev:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { xfs_printk_ratelimited!(xfs_alert, $dev, $fmt $(, $arg)*); }; }
#[macro_export] macro_rules! xfs_crit_ratelimited { ($dev:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { xfs_printk_ratelimited!(xfs_crit, $dev, $fmt $(, $arg)*); }; }
#[macro_export] macro_rules! xfs_err_ratelimited { ($dev:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { xfs_printk_ratelimited!(xfs_err, $dev, $fmt $(, $arg)*); }; }
#[macro_export] macro_rules! xfs_warn_ratelimited { ($dev:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { xfs_printk_ratelimited!(xfs_warn, $dev, $fmt $(, $arg)*); }; }
#[macro_export] macro_rules! xfs_notice_ratelimited { ($dev:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { xfs_printk_ratelimited!(xfs_notice, $dev, $fmt $(, $arg)*); }; }
#[macro_export] macro_rules! xfs_info_ratelimited { ($dev:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { xfs_printk_ratelimited!(xfs_info, $dev, $fmt $(, $arg)*); }; }
#[macro_export] macro_rules! xfs_debug_ratelimited { ($dev:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { xfs_printk_ratelimited!(xfs_debug, $dev, $fmt $(, $arg)*); }; }

#[macro_export] macro_rules! xfs_warn_once { ($dev:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { xfs_printk_once!(xfs_warn, $dev, $fmt $(, $arg)*); }; }
#[macro_export] macro_rules! xfs_notice_once { ($dev:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { xfs_printk_once!(xfs_notice, $dev, $fmt $(, $arg)*); }; }
#[macro_export] macro_rules! xfs_info_once { ($dev:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { xfs_printk_once!(xfs_info, $dev, $fmt $(, $arg)*); }; }

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum xfs_experimental_feat {
    XFS_EXPERIMENTAL_SHRINK,
    XFS_EXPERIMENTAL_LARP,
    XFS_EXPERIMENTAL_MAX,
}

unsafe extern "C" {
    pub fn xfs_warn_experimental(mp: *mut xfs_mount, f: xfs_experimental_feat);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
