/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2002-2004, Instant802 Networks, Inc.
 */

// Dependencies supplied by the Linux/kernel translation environment:
// linux/types.h, linux/crypto.h, and "key.h".

use core::ffi::c_int;

// Opaque C types declared by the included headers.
#[repr(C)]
pub struct arc4_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ieee80211_key {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

pub type u8 = core::ffi::c_uchar;
pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;

pub const TKIP_DECRYPT_OK: c_int = 0;
pub const TKIP_DECRYPT_NO_EXT_IV: c_int = -1;
pub const TKIP_DECRYPT_INVALID_KEYIDX: c_int = -2;
pub const TKIP_DECRYPT_REPLAY: c_int = -3;

extern "C" {
    pub fn ieee80211_tkip_encrypt_data(
        ctx: *mut arc4_ctx,
        key: *mut ieee80211_key,
        skb: *mut sk_buff,
        payload: *mut u8,
        payload_len: usize,
    ) -> c_int;

    pub fn ieee80211_tkip_decrypt_data(
        ctx: *mut arc4_ctx,
        key: *mut ieee80211_key,
        payload: *mut u8,
        payload_len: usize,
        ta: *mut u8,
        ra: *mut u8,
        only_iv: c_int,
        queue: c_int,
        out_iv32: *mut u32,
        out_iv16: *mut u16,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
