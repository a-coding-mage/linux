/* zconf.h -- configuration of the zlib compression library
 * Copyright (C) 1995-1998 Jean-loup Gailly.
 * For conditions of distribution and use, see copyright notice in zlib.h
 */

/* @(#) $Id$ */

/*
 * The memory requirements for deflate are (in bytes):
 *             (1 << (windowBits+2)) +  (1 << (memLevel+9))
 * that is: 128K for windowBits=15  +  128K for memLevel = 8  (default values)
 * plus a few kilobytes for small objects. For example, if you want to reduce
 * the default memory requirements from 256K to 128K, compile with
 *     make CFLAGS="-O -DMAX_WBITS=14 -DMAX_MEM_LEVEL=7"
 * Of course this will generally degrade compression (there's no free lunch).
 *
 *   The memory requirements for inflate are (in bytes) 1 << windowBits
 * that is, 32K for windowBits=15 (default value) plus a few kilobytes
 * for small objects.
 */

/* Maximum value for memLevel in deflateInit2.
 * C preprocessor override: define MAX_MEM_LEVEL at build time to replace this
 * default.
 */
pub const MAX_MEM_LEVEL: u32 = 8;

/* Maximum value for windowBits in deflateInit2 and inflateInit2.
 * WARNING: reducing MAX_WBITS makes minigzip unable to extract .gz files
 * created by gzip. (Files created by minigzip can still be extracted by
 * gzip.)
 *
 * C preprocessor override: define MAX_WBITS at build time to replace this
 * default.
 */
pub const MAX_WBITS: u32 = 15; /* 32K LZ77 window */

/* default windowBits for decompression. MAX_WBITS is for compression only */
pub const DEF_WBITS: u32 = MAX_WBITS;

/* default memLevel.
 * The original conditional selects 8 when MAX_MEM_LEVEL >= 8, otherwise
 * MAX_MEM_LEVEL. With the defaults above, this is 8.
 */
pub const DEF_MEM_LEVEL: u32 = if MAX_MEM_LEVEL >= 8 {
    8
} else {
    MAX_MEM_LEVEL
};

/* Type declarations */

pub type Byte = u8; /* 8 bits */
pub type uInt = u32; /* 16 bits or more */
pub type uLong = u64; /* 32 bits or more */
pub type voidp = *mut core::ffi::c_void;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
