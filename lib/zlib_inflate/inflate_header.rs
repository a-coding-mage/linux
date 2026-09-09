/* inflate.h -- internal inflate state definition
 * Copyright (C) 1995-2004 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */

/* WARNING: this file should *not* be used by applications. It is
   part of the implementation of the compression library and is
   subject to change. Applications should only use zlib.h.
 */

/* Dependency supplied by inftrees.h: `code` and `ENOUGH`. */

/* Possible inflate modes between inflate() calls */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum inflate_mode {
    HEAD,
    FLAGS,
    TIME,
    OS,
    EXLEN,
    EXTRA,
    NAME,
    COMMENT,
    HCRC,
    DICTID,
    DICT,
    TYPE,
    TYPEDO,
    STORED,
    COPY,
    TABLE,
    LENLENS,
    CODELENS,
    LEN,
    LENEXT,
    DIST,
    DISTEXT,
    MATCH,
    LIT,
    CHECK,
    LENGTH,
    DONE,
    BAD,
    MEM,
    SYNC,
}

/*
    State maintained between inflate() calls. Approximately 7K bytes.
    The `code` type and `ENOUGH` constant are supplied by inftrees.h.
*/
#[repr(C)]
pub struct inflate_state {
    pub mode: inflate_mode,
    pub last: i32,
    pub wrap: i32,
    pub havedict: i32,
    pub flags: i32,
    pub dmax: u32,
    pub check: ::std::os::raw::c_ulong,
    pub total: ::std::os::raw::c_ulong,
    /* gz_headerp head; -- where to save gzip header information */
    pub wbits: u32,
    pub wsize: u32,
    pub whave: u32,
    pub write: u32,
    pub window: *mut u8,
    pub hold: ::std::os::raw::c_ulong,
    pub bits: u32,
    pub length: u32,
    pub offset: u32,
    pub extra: u32,
    pub lencode: *const code,
    pub distcode: *const code,
    pub lenbits: u32,
    pub distbits: u32,
    pub ncode: u32,
    pub nlen: u32,
    pub ndist: u32,
    pub have: u32,
    pub next: *mut code,
    pub lens: [u16; 320],
    pub work: [u16; 288],
    pub codes: [code; ENOUGH],
}

/* Reverse the bytes in a 32-bit value */
#[inline]
pub const fn REVERSE(q: u32) -> u32 {
    ((q >> 24) & 0xff)
        .wrapping_add((q >> 8) & 0xff00)
        .wrapping_add((q & 0xff00) << 8)
        .wrapping_add((q & 0xff) << 24)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
