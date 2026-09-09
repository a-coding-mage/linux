/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Adapted for linux kernel by Alexander Mamaev:
 * - remove implementations of get_unaligned_
 * - assume GCC is always defined
 * - ISO C90
 * - linux kernel code style
 */

// `#include <linux/types.h>`

use core::ffi::c_void;

/* globals from xpress_decompress.c */
#[repr(C)]
pub struct xpress_decompressor {
    _private: [u8; 0],
}

extern "C" {
    pub fn xpress_allocate_decompressor() -> *mut xpress_decompressor;
    pub fn xpress_free_decompressor(d: *mut xpress_decompressor);
    pub fn xpress_decompress(
        d: *mut xpress_decompressor,
        compressed_data: *const c_void,
        compressed_size: usize,
        uncompressed_data: *mut c_void,
        uncompressed_size: usize,
    ) -> i32;
}

/* globals from lzx_decompress.c */
#[repr(C)]
pub struct lzx_decompressor {
    _private: [u8; 0],
}

extern "C" {
    pub fn lzx_allocate_decompressor() -> *mut lzx_decompressor;
    pub fn lzx_free_decompressor(d: *mut lzx_decompressor);
    pub fn lzx_decompress(
        d: *mut lzx_decompressor,
        compressed_data: *const c_void,
        compressed_size: usize,
        uncompressed_data: *mut c_void,
        uncompressed_size: usize,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
