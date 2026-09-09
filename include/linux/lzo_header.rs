/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  LZO Public Kernel Interface
 *  A mini subset of the LZO real-time data compression library
 *
 *  Copyright (C) 1996-2012 Markus F.X.J. Oberhumer <markus@oberhumer.com>
 *
 *  The full LZO package can be found at:
 *  http://www.oberhumer.com/opensource/lzo/
 *
 *  Changed for Linux kernel use by:
 *  Nitin Gupta <nitingupta910@gmail.com>
 *  Richard Purdie <rpurdie@openedhand.com>
 */

use core::ffi::c_void;

pub const LZO1X_1_MEM_COMPRESS: usize = 8192 * core::mem::size_of::<u16>();
pub const LZO1X_MEM_COMPRESS: usize = LZO1X_1_MEM_COMPRESS;

#[macro_export]
macro_rules! lzo1x_worst_compress {
    ($x:expr) => {
        ($x) + (($x) / 16) + 64 + 3 + 2
    };
}

/* This requires 'wrkmem' of size LZO1X_1_MEM_COMPRESS */
extern "C" {
    pub fn lzo1x_1_compress(
        src: *const u8,
        src_len: usize,
        dst: *mut u8,
        dst_len: *mut usize,
        wrkmem: *mut c_void,
    ) -> i32;
}

/* Same as above but does not write more than dst_len to dst. */
extern "C" {
    pub fn lzo1x_1_compress_safe(
        src: *const u8,
        src_len: usize,
        dst: *mut u8,
        dst_len: *mut usize,
        wrkmem: *mut c_void,
    ) -> i32;
}

/* This requires 'wrkmem' of size LZO1X_1_MEM_COMPRESS */
extern "C" {
    pub fn lzorle1x_1_compress(
        src: *const u8,
        src_len: usize,
        dst: *mut u8,
        dst_len: *mut usize,
        wrkmem: *mut c_void,
    ) -> i32;
}

/* Same as above but does not write more than dst_len to dst. */
extern "C" {
    pub fn lzorle1x_1_compress_safe(
        src: *const u8,
        src_len: usize,
        dst: *mut u8,
        dst_len: *mut usize,
        wrkmem: *mut c_void,
    ) -> i32;
}

/* safe decompression with overrun testing */
extern "C" {
    pub fn lzo1x_decompress_safe(
        src: *const u8,
        src_len: usize,
        dst: *mut u8,
        dst_len: *mut usize,
    ) -> i32;
}

/*
 * Return values (< 0 = Error)
 */
pub const LZO_E_OK: i32 = 0;
pub const LZO_E_ERROR: i32 = -1;
pub const LZO_E_OUT_OF_MEMORY: i32 = -2;
pub const LZO_E_NOT_COMPRESSIBLE: i32 = -3;
pub const LZO_E_INPUT_OVERRUN: i32 = -4;
pub const LZO_E_OUTPUT_OVERRUN: i32 = -5;
pub const LZO_E_LOOKBEHIND_OVERRUN: i32 = -6;
pub const LZO_E_EOF_NOT_FOUND: i32 = -7;
pub const LZO_E_INPUT_NOT_CONSUMED: i32 = -8;
pub const LZO_E_NOT_YET_IMPLEMENTED: i32 = -9;
pub const LZO_E_INVALID_ARGUMENT: i32 = -10;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
