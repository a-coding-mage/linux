// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/lib/zlib_deflate/deflate_syms.c
 *
 * Exported symbols for the deflate functionality.
 *
 */

// C dependencies:
// #include <linux/module.h>
// #include <linux/init.h>
// #include <linux/zlib.h>

// The zlib stream type is supplied by the corresponding zlib dependency.
use core::ffi::c_int;

#[repr(C)]
pub struct z_stream {
    _private: [u8; 0],
}

extern "C" {
    pub fn zlib_deflate_workspacesize() -> usize;
    pub static mut zlib_deflate_dfltcc_enabled: bool;
    pub fn zlib_deflate(strm: *mut z_stream) -> c_int;
    pub fn zlib_deflateInit2(
        strm: *mut z_stream,
        level: c_int,
        method: c_int,
        window_bits: c_int,
        mem_level: c_int,
        strategy: c_int,
    ) -> c_int;
    pub fn zlib_deflateEnd(strm: *mut z_stream) -> c_int;
    pub fn zlib_deflateReset(strm: *mut z_stream) -> c_int;
}

// EXPORT_SYMBOL(zlib_deflate_workspacesize);
// EXPORT_SYMBOL(zlib_deflate_dfltcc_enabled);
// EXPORT_SYMBOL(zlib_deflate);
// EXPORT_SYMBOL(zlib_deflateInit2);
// EXPORT_SYMBOL(zlib_deflateEnd);
// EXPORT_SYMBOL(zlib_deflateReset);
// MODULE_DESCRIPTION("Data compression using the deflation algorithm");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
