// SPDX-License-Identifier: GPL-2.0
/*
   BlueZ - Bluetooth protocol stack for Linux
   Copyright (C) 2000-2001 Qualcomm Incorporated

   Bluetooth kernel library.
*/

use core::ffi::{c_char, c_int, c_uint, c_ushort, c_uchar};

// Supplied by the Bluetooth/kernel dependencies.
#[repr(C)]
pub struct bdaddr_t {
    pub b: [c_uchar; 6],
}

extern "C" {
    static EBADRQC: c_int;
    static ENOTCONN: c_int;
    static EIO: c_int;
    static EHOSTDOWN: c_int;
    static EACCES: c_int;
    static EBADE: c_int;
    static ENOMEM: c_int;
    static ETIMEDOUT: c_int;
    static EMLINK: c_int;
    static EALREADY: c_int;
    static EBUSY: c_int;
    static ECONNREFUSED: c_int;
    static EOPNOTSUPP: c_int;
    static EINVAL: c_int;
    static ECONNRESET: c_int;
    static ECONNABORTED: c_int;
    static ELOOP: c_int;
    static EPROTONOSUPPORT: c_int;
    static EPROTO: c_int;
    static ENOSYS: c_int;
}

/// Swaps the byte order of a Bluetooth device address.
pub unsafe extern "C" fn baswap(dst: *mut bdaddr_t, src: *const bdaddr_t) {
    let s = src as *const c_uchar;
    let d = dst as *mut c_uchar;
    let mut i: c_uint = 0;
    while i < 6 {
        *d.add(i as usize) = *s.add((5 - i) as usize);
        i += 1;
    }
}

pub unsafe extern "C" fn bt_to_errno(code: c_ushort) -> c_int {
    match code {
        0 => 0,
        0x01 => EBADRQC,
        0x02 => ENOTCONN,
        0x03 => EIO,
        0x04 | 0x3c => EHOSTDOWN,
        0x05 => EACCES,
        0x06 => EBADE,
        0x07 => ENOMEM,
        0x08 | 0x10 => ETIMEDOUT,
        0x09 | 0x0a => EMLINK,
        0x0b => EALREADY,
        0x0c => EBUSY,
        0x0d | 0x0e | 0x0f | 0x1b => ECONNREFUSED,
        0x11 | 0x20 | 0x27 | 0x29 => EOPNOTSUPP,
        0x12 => EINVAL,
        0x13 | 0x14 | 0x15 => ECONNRESET,
        0x16 => ECONNABORTED,
        0x17 => ELOOP,
        0x18 => EACCES,
        0x1a => EPROTONOSUPPORT,
        0x19 | 0x1e | 0x23 | 0x24 | 0x25 => EPROTO,
        _ => ENOSYS,
    }
}

pub unsafe extern "C" fn bt_status(err: c_int) -> c_uchar {
    if err >= 0 { return err as c_uchar; }
    match err {
        x if x == -EBADRQC => 0x01,
        x if x == -ENOTCONN => 0x02,
        x if x == -EIO => 0x03,
        x if x == -EHOSTDOWN => 0x04,
        x if x == -EACCES => 0x05,
        x if x == -EBADE => 0x06,
        x if x == -ENOMEM => 0x07,
        x if x == -ETIMEDOUT => 0x08,
        x if x == -EMLINK => 0x09,
        x if x == -EALREADY => 0x0b,
        x if x == -EBUSY => 0x0c,
        x if x == -ECONNREFUSED => 0x0d,
        x if x == -EOPNOTSUPP => 0x11,
        x if x == -EINVAL => 0x12,
        x if x == -ECONNRESET => 0x13,
        x if x == -ECONNABORTED => 0x16,
        x if x == -ELOOP => 0x17,
        x if x == -EPROTONOSUPPORT => 0x1a,
        x if x == -EPROTO => 0x19,
        _ => 0x1f,
    }
}

// C variadic entry points retain their ABI; formatting/logging is provided by the kernel.
pub unsafe extern "C" fn bt_info(_format: *const c_char, ...) {}
pub unsafe extern "C" fn bt_warn(_format: *const c_char, ...) {}
pub unsafe extern "C" fn bt_err(_format: *const c_char, ...) {}
pub unsafe extern "C" fn bt_warn_ratelimited(_format: *const c_char, ...) {}
pub unsafe extern "C" fn bt_err_ratelimited(_format: *const c_char, ...) {}

#[cfg(feature = "CONFIG_BT_FEATURE_DEBUG")]
static mut DEBUG_ENABLE: bool = false;

#[cfg(feature = "CONFIG_BT_FEATURE_DEBUG")]
pub unsafe extern "C" fn bt_dbg_set(enable: bool) { DEBUG_ENABLE = enable; }

#[cfg(feature = "CONFIG_BT_FEATURE_DEBUG")]
pub unsafe extern "C" fn bt_dbg_get() -> bool { DEBUG_ENABLE }

#[cfg(feature = "CONFIG_BT_FEATURE_DEBUG")]
pub unsafe extern "C" fn bt_dbg(_format: *const c_char, ...) {
    if !DEBUG_ENABLE { return; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
