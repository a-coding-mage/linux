/* zutil.h -- internal interface and configuration of the compression library
 * Copyright (C) 1995-1998 Jean-loup Gailly.
 * For conditions of distribution and use, see copyright notice in zlib.h
 */

/* WARNING: this file should *not* be used by applications. It is
   part of the implementation of the compression library and is
   subject to change. Applications should only use zlib.h.
 */

/* @(#) $Id: zutil.h,v 1.1 2000/01/01 03:32:23 davem Exp $ */

// C dependencies: linux/zlib.h, linux/string.h, and linux/kernel.h.

pub type uch = ::core::ffi::c_uchar;
pub type ush = ::core::ffi::c_ushort;
pub type ulg = ::core::ffi::c_ulong;

/* common constants */

pub const STORED_BLOCK: ::core::ffi::c_int = 0;
pub const STATIC_TREES: ::core::ffi::c_int = 1;
pub const DYN_TREES: ::core::ffi::c_int = 2;
/* The three kinds of block type */

pub const MIN_MATCH: ::core::ffi::c_int = 3;
pub const MAX_MATCH: ::core::ffi::c_int = 258;
/* The minimum and maximum match lengths */

pub const PRESET_DICT: ::core::ffi::c_int = 0x20; /* preset dictionary flag in zlib header */

/* target dependencies */

/* Common defaults */

// #ifndef OS_CODE
pub const OS_CODE: ::core::ffi::c_int = 0x03; /* assume Unix */
// #endif

/* functions */

pub type check_func = Option<unsafe extern "C" fn(
    check: uLong,
    buf: *const Byte,
    len: uInt,
) -> uLong>;

/* checksum functions */

pub const BASE: ::core::ffi::c_long = 65521; /* largest prime smaller than 65536 */
pub const NMAX: ::core::ffi::c_int = 5552;
/* NMAX is the largest n such that 255n(n+1)/2 + (n+1)(BASE-1) <= 2^32-1 */

macro_rules! DO1 {
    ($buf:expr, $i:expr) => {{ s1 += $buf[$i] as _; s2 += s1; }};
}
macro_rules! DO2 {
    ($buf:expr, $i:expr) => {{ DO1!($buf, $i); DO1!($buf, $i + 1); }};
}
macro_rules! DO4 {
    ($buf:expr, $i:expr) => {{ DO2!($buf, $i); DO2!($buf, $i + 2); }};
}
macro_rules! DO8 {
    ($buf:expr, $i:expr) => {{ DO4!($buf, $i); DO4!($buf, $i + 4); }};
}
macro_rules! DO16 {
    ($buf:expr) => {{ DO8!($buf, 0); DO8!($buf, 8); }};
}

/* ========================================================================= */
/*
     Update a running Adler-32 checksum with the bytes buf[0..len-1] and
   return the updated checksum. If buf is NULL, this function returns
   the required initial value for the checksum.
   An Adler-32 checksum is almost as reliable as a CRC32 but can be computed
   much faster. Usage example:

     uLong adler = zlib_adler32(0L, NULL, 0);

     while (read_buffer(buffer, length) != EOF) {
       adler = zlib_adler32(adler, buffer, length);
     }
     if (adler != original_adler) error();
*/
#[inline]
pub unsafe fn zlib_adler32(mut adler: uLong, mut buf: *const Byte, mut len: uInt) -> uLong {
    let mut s1: ::core::ffi::c_ulong = (adler as ::core::ffi::c_ulong) & 0xffff;
    let mut s2: ::core::ffi::c_ulong = ((adler as ::core::ffi::c_ulong) >> 16) & 0xffff;
    let mut k: uInt;

    if buf.is_null() { return 1 as uLong; }

    while len > 0 {
        k = if len < NMAX as uInt { len } else { NMAX as uInt };
        len -= k;
        while k >= 16 {
            DO16!(buf);
            buf = buf.add(16);
            k -= 16;
        }
        if k != 0 {
            loop {
                s1 += *buf as _;
                buf = buf.add(1);
                s2 += s1;
                k -= 1;
                if k == 0 { break; }
            }
        }
        s1 %= BASE as ::core::ffi::c_ulong;
        s2 %= BASE as ::core::ffi::c_ulong;
    }
    ((s2 << 16) | s1) as uLong
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
