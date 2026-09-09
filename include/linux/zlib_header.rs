/* zlib.h -- interface of the 'zlib' general purpose compression library

  Copyright (C) 1995-2005 Jean-loup Gailly and Mark Adler

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.
*/

// Dependency supplied by linux/zconf.h is intentionally left external.

/* zlib deflate based on ZLIB_VERSION "1.1.3" */
/* zlib inflate based on ZLIB_VERSION "1.2.3" */

/*
  This is a modified version of zlib for use inside the Linux kernel.
  The main changes are to perform all memory allocation in advance.

  Inflation Changes:
    * Z_PACKET_FLUSH is added and used by ppp_deflate. Before returning
      this checks there is no more input data available and the next data
      is a STORED block. It also resets the mode to be read for the next
      data, all as per PPP requirements.
    * Addition of zlib_inflateIncomp which copies incompressible data into
      the history window and adjusts the accoutning without calling
      zlib_inflate itself to inflate the data.
*/

#[repr(C)]
pub struct internal_state;

#[repr(C)]
pub struct z_stream {
    pub next_in: *const Byte,
    pub avail_in: uLong,
    pub total_in: uLong,
    pub next_out: *mut Byte,
    pub avail_out: uLong,
    pub total_out: uLong,
    pub msg: *mut ::core::ffi::c_char,
    pub state: *mut internal_state,
    pub workspace: *mut ::core::ffi::c_void,
    pub data_type: ::core::ffi::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}

pub type z_streamp = *mut z_stream;

pub const Z_NO_FLUSH: ::core::ffi::c_int = 0;
pub const Z_PARTIAL_FLUSH: ::core::ffi::c_int = 1;
pub const Z_PACKET_FLUSH: ::core::ffi::c_int = 2;
pub const Z_SYNC_FLUSH: ::core::ffi::c_int = 3;
pub const Z_FULL_FLUSH: ::core::ffi::c_int = 4;
pub const Z_FINISH: ::core::ffi::c_int = 5;
pub const Z_BLOCK: ::core::ffi::c_int = 6;

pub const Z_OK: ::core::ffi::c_int = 0;
pub const Z_STREAM_END: ::core::ffi::c_int = 1;
pub const Z_NEED_DICT: ::core::ffi::c_int = 2;
pub const Z_ERRNO: ::core::ffi::c_int = -1;
pub const Z_STREAM_ERROR: ::core::ffi::c_int = -2;
pub const Z_DATA_ERROR: ::core::ffi::c_int = -3;
pub const Z_MEM_ERROR: ::core::ffi::c_int = -4;
pub const Z_BUF_ERROR: ::core::ffi::c_int = -5;
pub const Z_VERSION_ERROR: ::core::ffi::c_int = -6;

pub const Z_NO_COMPRESSION: ::core::ffi::c_int = 0;
pub const Z_BEST_SPEED: ::core::ffi::c_int = 1;
pub const Z_BEST_COMPRESSION: ::core::ffi::c_int = 9;
pub const Z_DEFAULT_COMPRESSION: ::core::ffi::c_int = -1;
pub const Z_FILTERED: ::core::ffi::c_int = 1;
pub const Z_HUFFMAN_ONLY: ::core::ffi::c_int = 2;
pub const Z_DEFAULT_STRATEGY: ::core::ffi::c_int = 0;
pub const Z_BINARY: ::core::ffi::c_int = 0;
pub const Z_ASCII: ::core::ffi::c_int = 1;
pub const Z_UNKNOWN: ::core::ffi::c_int = 2;
pub const Z_DEFLATED: ::core::ffi::c_int = 8;

extern "C" {
    pub fn zlib_deflate_workspacesize(windowBits: ::core::ffi::c_int, memLevel: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn zlib_deflate_dfltcc_enabled() -> ::core::ffi::c_int;
    pub fn zlib_deflate(strm: z_streamp, flush: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn zlib_deflateEnd(strm: z_streamp) -> ::core::ffi::c_int;
    pub fn zlib_inflate_workspacesize() -> ::core::ffi::c_int;
    pub fn zlib_inflate(strm: z_streamp, flush: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn zlib_inflateEnd(strm: z_streamp) -> ::core::ffi::c_int;
    pub fn zlib_deflateReset(strm: z_streamp) -> ::core::ffi::c_int;
    pub fn zlib_inflateReset(strm: z_streamp) -> ::core::ffi::c_int;
    pub fn zlib_inflateIncomp(strm: *mut z_stream) -> ::core::ffi::c_int;
    pub fn zlib_deflateInit2(
        strm: z_streamp, level: ::core::ffi::c_int, method: ::core::ffi::c_int,
        windowBits: ::core::ffi::c_int, memLevel: ::core::ffi::c_int,
        strategy: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn zlib_inflateInit2(strm: z_streamp, windowBits: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn zlib_inflate_blob(dst: *mut ::core::ffi::c_void, dst_sz: ::core::ffi::c_uint,
                             src: *const ::core::ffi::c_void, src_sz: ::core::ffi::c_uint) -> ::core::ffi::c_int;
}

#[inline]
pub const unsafe fn deflateBound(s: uLong) -> uLong {
    s.wrapping_add((s.wrapping_add(7)) >> 3)
        .wrapping_add((s.wrapping_add(63)) >> 6)
        .wrapping_add(11)
}

// These macros depend on MAX_WBITS, DEF_MEM_LEVEL, and DEF_WBITS supplied by zconf.h.
#[inline]
pub unsafe fn zlib_deflateInit(strm: z_streamp, level: ::core::ffi::c_int) -> ::core::ffi::c_int {
    zlib_deflateInit2(strm, level, Z_DEFLATED, MAX_WBITS, DEF_MEM_LEVEL, Z_DEFAULT_STRATEGY)
}

#[inline]
pub unsafe fn zlib_inflateInit(strm: z_streamp) -> ::core::ffi::c_int {
    zlib_inflateInit2(strm, DEF_WBITS)
}

#[cfg(not(any(_Z_UTIL_H, NO_DUMMY_DECL)))]
#[repr(C)]
pub struct internal_state_dummy { pub dummy: ::core::ffi::c_int }

/* Utility function: initialize zlib, unpack binary blob, clean up zlib,
 * return len or negative error code. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
