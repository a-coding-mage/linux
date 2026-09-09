/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner, Simon Wunderlich
 */

// C dependencies: "main.h", linux/atomic.h, linux/bitops.h,
// linux/compiler.h, and linux/printk.h.

#[cfg(CONFIG_BATMAN_ADV_DEBUG)]
extern "C" {
    pub fn batadv_debug_log_setup(bat_priv: *mut batadv_priv) -> ::core::ffi::c_int;
    pub fn batadv_debug_log_cleanup(bat_priv: *mut batadv_priv);
}

#[cfg(not(CONFIG_BATMAN_ADV_DEBUG))]
#[inline]
pub unsafe fn batadv_debug_log_setup(_bat_priv: *mut batadv_priv) -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_BATMAN_ADV_DEBUG))]
#[inline]
pub unsafe fn batadv_debug_log_cleanup(_bat_priv: *mut batadv_priv) {}

/**
 * enum batadv_dbg_level - available log levels
 */
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum batadv_dbg_level {
    /** @BATADV_DBG_BATMAN: OGM and TQ computations related messages */
    BATADV_DBG_BATMAN = 1 << 0,
    /** @BATADV_DBG_ROUTES: route added / changed / deleted */
    BATADV_DBG_ROUTES = 1 << 1,
    /** @BATADV_DBG_TT: translation table messages */
    BATADV_DBG_TT = 1 << 2,
    /** @BATADV_DBG_BLA: bridge loop avoidance messages */
    BATADV_DBG_BLA = 1 << 3,
    /** @BATADV_DBG_DAT: ARP snooping and DAT related messages */
    BATADV_DBG_DAT = 1 << 4,
    /** @BATADV_DBG_MCAST: multicast related messages */
    BATADV_DBG_MCAST = 1 << 6,
    /** @BATADV_DBG_TP_METER: throughput meter messages */
    BATADV_DBG_TP_METER = 1 << 7,
    /** @BATADV_DBG_ALL: the union of all the above log levels */
    BATADV_DBG_ALL = 255,
}

#[cfg(CONFIG_BATMAN_ADV_DEBUG)]
extern "C" {
    pub fn batadv_debug_log(
        bat_priv: *mut batadv_priv,
        fmt: *const ::core::ffi::c_char,
        ...,
    ) -> ::core::ffi::c_int;
}

#[cfg(CONFIG_BATMAN_ADV_DEBUG)]
#[macro_export]
macro_rules! _batadv_dbg {
    ($type:expr, $bat_priv:expr, $ratelimited:expr, $fmt:expr $(, $arg:expr)*) => {{
        let __batpriv = $bat_priv;
        if unsafe { ::core::ptr::read_volatile(&(*__batpriv).log_level) } & ($type as _) != 0
            && (!$ratelimited || unsafe { net_ratelimit() } != 0)
        {
            unsafe {
                $crate::batadv_debug_log(__batpriv, $fmt $(, $arg)*);
            }
        }
    }};
}

#[cfg(not(CONFIG_BATMAN_ADV_DEBUG))]
#[inline]
pub unsafe fn _batadv_dbg(
    _type: ::core::ffi::c_int,
    _bat_priv: *mut batadv_priv,
    _ratelimited: ::core::ffi::c_int,
    _fmt: *const ::core::ffi::c_char,
    ...,
) {
}

#[macro_export]
macro_rules! batadv_dbg {
    ($type:expr, $bat_priv:expr, $fmt:expr $(, $arg:expr)*) => {
        $crate::_batadv_dbg!($type, $bat_priv, 0, $fmt $(, $arg)*)
    };
}

#[macro_export]
macro_rules! batadv_dbg_ratelimited {
    ($type:expr, $bat_priv:expr, $fmt:expr $(, $arg:expr)*) => {
        $crate::_batadv_dbg!($type, $bat_priv, 1, $fmt $(, $arg)*)
    };
}

#[macro_export]
macro_rules! batadv_info {
    ($net_dev:expr, $fmt:expr $(, $arg:expr)*) => {{
        let _netdev = $net_dev;
        let _batpriv = unsafe { netdev_priv(_netdev) };
        $crate::batadv_dbg!($crate::batadv_dbg_level::BATADV_DBG_ALL, _batpriv, $fmt $(, $arg)*);
        unsafe { pr_info!("%s: " $fmt, (*_netdev).name $(, $arg)*) };
    }};
}

#[macro_export]
macro_rules! batadv_err {
    ($net_dev:expr, $fmt:expr $(, $arg:expr)*) => {{
        let _netdev = $net_dev;
        let _batpriv = unsafe { netdev_priv(_netdev) };
        $crate::batadv_dbg!($crate::batadv_dbg_level::BATADV_DBG_ALL, _batpriv, $fmt $(, $arg)*);
        unsafe { pr_err!("%s: " $fmt, (*_netdev).name $(, $arg)*) };
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
