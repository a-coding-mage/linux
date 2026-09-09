/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * vpd_decode.h
 *
 * Google VPD decoding routines.
 *
 * Copyright 2017 Google Inc.
 */

pub const VPD_OK: i32 = 0;
pub const VPD_FAIL: i32 = 1;

pub const VPD_TYPE_TERMINATOR: i32 = 0;
pub const VPD_TYPE_STRING: i32 = 1;
pub const VPD_TYPE_INFO: i32 = 0xfe;
pub const VPD_TYPE_IMPLICIT_TERMINATOR: i32 = 0xff;

/* Callback for vpd_decode_string to invoke. */
pub type vpd_decode_callback = unsafe extern "C" fn(
    key: *const u8,
    key_len: u32,
    value: *const u8,
    value_len: u32,
    arg: *mut core::ffi::c_void,
) -> i32;

/*
 * vpd_decode_string
 *
 * Given the encoded string, this function invokes callback with extracted
 * (key, value). The *consumed will be plused the number of bytes consumed in
 * this function.
 *
 * The input_buf points to the first byte of the input buffer.
 *
 * The *consumed starts from 0, which is actually the next byte to be decoded.
 * It can be non-zero to be used in multiple calls.
 *
 * If one entry is successfully decoded, sends it to callback and returns the
 * result.
 */
unsafe extern "C" {
    pub fn vpd_decode_string(
        max_len: u32,
        input_buf: *const u8,
        consumed: *mut u32,
        callback: vpd_decode_callback,
        callback_arg: *mut core::ffi::c_void,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
