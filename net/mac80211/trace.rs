// SPDX-License-Identifier: GPL-2.0
// bug in tracepoint.h, it should include this

// C includes and build-time configuration are supplied by the surrounding
// kernel translation unit.  The implementation is omitted when __CHECKER__
// is defined, and the functions below are present only with
// CONFIG_MAC80211_MESSAGE_TRACING.

#![feature(c_variadic)]

use core::ffi::{c_char, VaListImpl};

// External kernel types and functions supplied by other translation units.
#[repr(C)]
pub struct va_format {
    pub fmt: *const c_char,
    pub va: *mut VaListImpl<'static>,
}

extern "C" {
    fn pr_info(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn wiphy_dbg(wiphy: *mut wiphy, fmt: *const c_char, ...);
    fn trace_mac80211_info(vaf: *mut va_format);
    fn trace_mac80211_dbg(vaf: *mut va_format);
    fn trace_mac80211_err(vaf: *mut va_format);
}

#[repr(C)]
pub struct wiphy {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_MAC80211_MESSAGE_TRACING")]
pub unsafe extern "C" fn __sdata_info(fmt: *const c_char, mut args: ...) {
    let mut vaf = va_format {
        fmt,
        va: core::ptr::null_mut(),
    };

    // va_start(args, fmt);
    vaf.va = &mut args;

    pr_info(b"%pV\0".as_ptr() as *const c_char, &mut vaf);
    trace_mac80211_info(&mut vaf);
    // va_end(args);
}

#[cfg(feature = "CONFIG_MAC80211_MESSAGE_TRACING")]
pub unsafe extern "C" fn __sdata_dbg(print: bool, fmt: *const c_char, mut args: ...) {
    let mut vaf = va_format {
        fmt,
        va: core::ptr::null_mut(),
    };

    // va_start(args, fmt);
    vaf.va = &mut args;

    if print {
        pr_debug(b"%pV\0".as_ptr() as *const c_char, &mut vaf);
    }
    trace_mac80211_dbg(&mut vaf);
    // va_end(args);
}

#[cfg(feature = "CONFIG_MAC80211_MESSAGE_TRACING")]
pub unsafe extern "C" fn __sdata_err(fmt: *const c_char, mut args: ...) {
    let mut vaf = va_format {
        fmt,
        va: core::ptr::null_mut(),
    };

    // va_start(args, fmt);
    vaf.va = &mut args;

    pr_err(b"%pV\0".as_ptr() as *const c_char, &mut vaf);
    trace_mac80211_err(&mut vaf);
    // va_end(args);
}

#[cfg(feature = "CONFIG_MAC80211_MESSAGE_TRACING")]
pub unsafe extern "C" fn __wiphy_dbg(
    wiphy: *mut wiphy,
    print: bool,
    fmt: *const c_char,
    mut args: ...,
) {
    let mut vaf = va_format {
        fmt,
        va: core::ptr::null_mut(),
    };

    // va_start(args, fmt);
    vaf.va = &mut args;

    if print {
        wiphy_dbg(wiphy, b"%pV\0".as_ptr() as *const c_char, &mut vaf);
    }
    trace_mac80211_dbg(&mut vaf);
    // va_end(args);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
