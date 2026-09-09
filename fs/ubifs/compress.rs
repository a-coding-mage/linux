// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file is part of UBIFS.
 *
 * Copyright (C) 2006-2008 Nokia Corporation.
 * Copyright (C) 2006, 2007 University of Szeged, Hungary
 *
 * Authors: Adrian Hunter
 *          Artem Bityutskiy (Битюцкий Артём)
 *          Zoltan Sogor
 */

/* This file provides a single place to access to compression and decompression. */

use core::ffi::c_void;

#[repr(C)]
pub union ubifs_in_ptr {
    pub buf: *const c_void,
    pub folio: *mut folio,
}

/* Fake description object for the "none" compressor */
static mut none_compr: ubifs_compressor = ubifs_compressor {
    compr_type: UBIFS_COMPR_NONE,
    name: b"none\0".as_ptr() as *const i8,
    capi_name: b"\0".as_ptr() as *const i8,
    ..unsafe { core::mem::zeroed() }
};

#[cfg(feature = "CONFIG_UBIFS_FS_LZO")]
static mut lzo_compr: ubifs_compressor = ubifs_compressor {
    compr_type: UBIFS_COMPR_LZO,
    name: b"lzo\0".as_ptr() as *const i8,
    capi_name: b"lzo\0".as_ptr() as *const i8,
    ..unsafe { core::mem::zeroed() }
};
#[cfg(not(feature = "CONFIG_UBIFS_FS_LZO"))]
static mut lzo_compr: ubifs_compressor = ubifs_compressor {
    compr_type: UBIFS_COMPR_LZO,
    name: b"lzo\0".as_ptr() as *const i8,
    ..unsafe { core::mem::zeroed() }
};

#[cfg(feature = "CONFIG_UBIFS_FS_ZLIB")]
static mut zlib_compr: ubifs_compressor = ubifs_compressor {
    compr_type: UBIFS_COMPR_ZLIB,
    name: b"zlib\0".as_ptr() as *const i8,
    capi_name: b"deflate\0".as_ptr() as *const i8,
    ..unsafe { core::mem::zeroed() }
};
#[cfg(not(feature = "CONFIG_UBIFS_FS_ZLIB"))]
static mut zlib_compr: ubifs_compressor = ubifs_compressor {
    compr_type: UBIFS_COMPR_ZLIB,
    name: b"zlib\0".as_ptr() as *const i8,
    ..unsafe { core::mem::zeroed() }
};

#[cfg(feature = "CONFIG_UBIFS_FS_ZSTD")]
static mut zstd_compr: ubifs_compressor = ubifs_compressor {
    compr_type: UBIFS_COMPR_ZSTD,
    name: b"zstd\0".as_ptr() as *const i8,
    capi_name: b"zstd\0".as_ptr() as *const i8,
    ..unsafe { core::mem::zeroed() }
};
#[cfg(not(feature = "CONFIG_UBIFS_FS_ZSTD"))]
static mut zstd_compr: ubifs_compressor = ubifs_compressor {
    compr_type: UBIFS_COMPR_ZSTD,
    name: b"zstd\0".as_ptr() as *const i8,
    ..unsafe { core::mem::zeroed() }
};

pub static mut ubifs_compressors: [*mut ubifs_compressor; UBIFS_COMPR_TYPES_CNT as usize] =
    [core::ptr::null_mut(); UBIFS_COMPR_TYPES_CNT as usize];

unsafe fn ubifs_compress_common(
    compr_type: *mut i32, in_ptr: ubifs_in_ptr, in_offset: usize, in_len: i32,
    in_folio: bool, out_buf: *mut c_void, out_len: *mut i32,
) {
    let compr = ubifs_compressors[*compr_type as usize];
    let mut dlen = *out_len;
    let mut err: i32;

    if *compr_type == UBIFS_COMPR_NONE || in_len < UBIFS_MIN_COMPR_LEN {
        if in_folio { memcpy_from_folio(out_buf, in_ptr.folio, in_offset, in_len); }
        else { memcpy(out_buf, in_ptr.buf, in_len as usize); }
        *out_len = in_len;
        *compr_type = UBIFS_COMPR_NONE;
        return;
    }

    dlen = core::cmp::min(dlen, in_len - UBIFS_MIN_COMPRESS_DIFF);
    /* ACOMP_REQUEST_ON_STACK/crypto wait are kernel macros and objects supplied by dependencies. */
    let _ = compr;
    err = crypto_acomp_compress_raw(compr, in_ptr, in_offset, in_len, out_buf, &mut dlen, in_folio);
    *out_len = dlen;
    if err == 0 { return; }
    if in_folio { memcpy_from_folio(out_buf, in_ptr.folio, in_offset, in_len); }
    else { memcpy(out_buf, in_ptr.buf, in_len as usize); }
    *out_len = in_len;
    *compr_type = UBIFS_COMPR_NONE;
}

pub unsafe fn ubifs_compress(_c: *const ubifs_info, in_buf: *const c_void, in_len: i32,
                             out_buf: *mut c_void, out_len: *mut i32, compr_type: *mut i32) {
    ubifs_compress_common(compr_type, ubifs_in_ptr { buf: in_buf }, 0, in_len, false, out_buf, out_len);
}

pub unsafe fn ubifs_compress_folio(_c: *const ubifs_info, in_folio: *mut folio,
                                   in_offset: usize, in_len: i32, out_buf: *mut c_void,
                                   out_len: *mut i32, compr_type: *mut i32) {
    ubifs_compress_common(compr_type, ubifs_in_ptr { folio: in_folio }, in_offset, in_len, true, out_buf, out_len);
}

unsafe fn ubifs_decompress_common(c: *const ubifs_info, in_buf: *const c_void, in_len: i32,
                                  out_ptr: *mut c_void, out_offset: usize, out_len: *mut i32,
                                  out_folio: bool, compr_type: i32) -> i32 {
    if compr_type < 0 || compr_type >= UBIFS_COMPR_TYPES_CNT {
        ubifs_err(c, "invalid compression type %d", compr_type); return -EINVAL;
    }
    let compr = ubifs_compressors[compr_type as usize];
    if (*compr).capi_name.is_null() {
        ubifs_err(c, "compression is not compiled in"); return -EINVAL;
    }
    if compr_type == UBIFS_COMPR_NONE {
        if out_folio { memcpy_to_folio(out_ptr, out_offset, in_buf, in_len); }
        else { memcpy(out_ptr, in_buf, in_len as usize); }
        *out_len = in_len; return 0;
    }
    let mut dlen = *out_len;
    let err = crypto_acomp_decompress_raw(compr, in_buf, in_len, out_ptr, out_offset, &mut dlen, out_folio);
    *out_len = dlen;
    if err != 0 { ubifs_err(c, "cannot decompress %d bytes, compressor, error %d", in_len, err); }
    err
}

pub unsafe fn ubifs_decompress(c: *const ubifs_info, in_buf: *const c_void, in_len: i32,
                               out_buf: *mut c_void, out_len: *mut i32, compr_type: i32) -> i32 {
    ubifs_decompress_common(c, in_buf, in_len, out_buf, 0, out_len, false, compr_type)
}

pub unsafe fn ubifs_decompress_folio(c: *const ubifs_info, in_buf: *const c_void, in_len: i32,
                                     out_folio: *mut folio, out_offset: usize, out_len: *mut i32,
                                     compr_type: i32) -> i32 {
    ubifs_decompress_common(c, in_buf, in_len, out_folio as *mut c_void, out_offset, out_len, true, compr_type)
}

pub unsafe fn ubifs_compressors_init() -> i32 {
    let mut err = compr_init(&mut lzo_compr); if err != 0 { return err; }
    err = compr_init(&mut zstd_compr); if err != 0 { compr_exit(&mut lzo_compr); return err; }
    err = compr_init(&mut zlib_compr); if err != 0 { compr_exit(&mut zstd_compr); compr_exit(&mut lzo_compr); return err; }
    ubifs_compressors[UBIFS_COMPR_NONE as usize] = &mut none_compr; 0
}

pub unsafe fn ubifs_compressors_exit() {
    compr_exit(&mut lzo_compr); compr_exit(&mut zlib_compr); compr_exit(&mut zstd_compr);
}

/* External declarations and kernel-provided types/functions are supplied by the surrounding UBIFS translation. */
extern "C" {
    fn compr_init(compr: *mut ubifs_compressor) -> i32;
    fn compr_exit(compr: *mut ubifs_compressor);
    fn crypto_acomp_compress_raw(compr: *mut ubifs_compressor, input: ubifs_in_ptr, offset: usize, len: i32, out: *mut c_void, dlen: *mut i32, folio: bool) -> i32;
    fn crypto_acomp_decompress_raw(compr: *mut ubifs_compressor, input: *const c_void, len: i32, out: *mut c_void, offset: usize, dlen: *mut i32, folio: bool) -> i32;
    fn memcpy(dst: *mut c_void, src: *const c_void, len: usize);
    fn memcpy_from_folio(dst: *mut c_void, src: *mut folio, offset: usize, len: i32);
    fn memcpy_to_folio(dst: *mut c_void, offset: usize, src: *const c_void, len: i32);
    fn ubifs_err(c: *const ubifs_info, fmt: *const str, ...);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
