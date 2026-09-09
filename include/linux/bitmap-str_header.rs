/* SPDX-License-Identifier: GPL-2.0 */

// Translated from <linux/bitmap-str.h>.
// The original header includes <linux/types.h>; the corresponding primitive
// Rust types are used directly here.

unsafe extern "C" {
    pub fn bitmap_parse_user(
        ubuf: *const core::ffi::c_char,
        ulen: u32,
        dst: *mut core::ffi::c_ulong,
        nbits: i32,
    ) -> i32;

    pub fn bitmap_print_bitmask_to_buf(
        buf: *mut core::ffi::c_char,
        maskp: *const core::ffi::c_ulong,
        nmaskbits: i32,
        off: i64,
        count: usize,
    ) -> i32;

    pub fn bitmap_print_list_to_buf(
        buf: *mut core::ffi::c_char,
        maskp: *const core::ffi::c_ulong,
        nmaskbits: i32,
        off: i64,
        count: usize,
    ) -> i32;

    pub fn bitmap_parse(
        buf: *const core::ffi::c_char,
        buflen: u32,
        dst: *mut core::ffi::c_ulong,
        nbits: i32,
    ) -> i32;

    pub fn bitmap_parselist(
        buf: *const core::ffi::c_char,
        maskp: *mut core::ffi::c_ulong,
        nmaskbits: i32,
    ) -> i32;

    pub fn bitmap_parselist_user(
        ubuf: *const core::ffi::c_char,
        ulen: u32,
        dst: *mut core::ffi::c_ulong,
        nbits: i32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
