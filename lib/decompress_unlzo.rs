// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * LZO decompressor for the Linux kernel. Code borrowed from the lzo
 * implementation by Markus Franz Xaver Johannes Oberhumer.
 *
 * Linux kernel adaptation:
 * Copyright (C) 2009
 * Albin Tonnerre, Free Electrons <albin.tonnerre@free-electrons.com>
 *
 * Original code:
 * Copyright (C) 1996-2005 Markus Franz Xaver Johannes Oberhumer
 * All Rights Reserved.
 *
 * Markus F.X.J. Oberhumer
 * <markus@oberhumer.com>
 * http://www.oberhumer.com/opensource/lzop/
 */

use core::ffi::c_void;

// C headers and build-time STATIC/PREBOOT configuration are supplied by the
// surrounding kernel translation unit.

type U8 = u8;
type U16 = u16;
type U32 = u32;

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn lzo1x_worst_compress(size: usize) -> usize;
    fn lzo1x_decompress_safe(
        src: *const U8,
        src_len: usize,
        dst: *mut U8,
        dst_len: *mut usize,
    ) -> U8;
    fn get_unaligned_be16(ptr: *const U8) -> U16;
    fn get_unaligned_be32(ptr: *const U8) -> U32;
}

const LZO_E_OK: U8 = 0;
const LZO_BLOCK_SIZE: usize = 256 * 1024;
const HEADER_HAS_FILTER: U32 = 0x00000800;
const HEADER_SIZE_MIN: usize = 9 + 7 + 4 + 8 + 1 + 4;
const HEADER_SIZE_MAX: usize = 9 + 7 + 1 + 8 + 8 + 4 + 1 + 255 + 4;

static LZOP_MAGIC: [U8; 9] = [0x89, 0x4c, 0x5a, 0x4f, 0x00, 0x0d, 0x0a, 0x1a, 0x0a];

unsafe fn parse_header(input: *mut U8, skip: *mut isize, in_len: isize) -> i32 {
    let mut l: isize;
    let mut parse = input;
    let end = input.offset(in_len);
    let version: U16;

    if in_len < HEADER_SIZE_MIN as isize {
        return 0;
    }

    for i in 0..9 {
        if *parse != LZOP_MAGIC[i] {
            return 0;
        }
        parse = parse.add(1);
    }
    version = get_unaligned_be16(parse);
    parse = parse.add(7);
    if version >= 0x0940 {
        parse = parse.add(1);
    }
    if get_unaligned_be32(parse) & HEADER_HAS_FILTER != 0 {
        parse = parse.add(8);
    } else {
        parse = parse.add(4);
    }

    if end.offset_from(parse) < (8 + 1 + 4) {
        return 0;
    }
    parse = parse.add(8);
    if version >= 0x0940 {
        parse = parse.add(4);
    }
    l = *parse as isize;
    parse = parse.add(1);
    if end.offset_from(parse) < l + 4 {
        return 0;
    }
    parse = parse.offset(l + 4);
    *skip = parse.offset_from(input);
    1
}

unsafe fn unlzo(
    mut input: *mut U8,
    mut in_len: isize,
    fill: Option<unsafe extern "C" fn(*mut c_void, usize) -> isize>,
    flush: Option<unsafe extern "C" fn(*mut c_void, usize) -> isize>,
    output: *mut U8,
    posp: *mut isize,
    error: unsafe extern "C" fn(*mut i8),
) -> i32 {
    let mut r: U8 = 0;
    let mut skip: isize = 0;
    let mut src_len: U32;
    let mut dst_len: U32;
    let mut tmp: usize;
    let mut in_buf: *mut U8;
    let mut in_buf_save: *mut U8;
    let mut out_buf: *mut U8;
    let mut ret: i32 = -1;

    if !output.is_null() {
        out_buf = output;
    } else if flush.is_none() {
        error(b"NULL output pointer and no flush function provided\0".as_ptr() as *mut i8);
        return ret;
    } else {
        out_buf = malloc(LZO_BLOCK_SIZE) as *mut U8;
        if out_buf.is_null() {
            error(b"Could not allocate output buffer\0".as_ptr() as *mut i8);
            return ret;
        }
    }

    if !input.is_null() && fill.is_some() {
        error(b"Both input pointer and fill function provided, don't know what to do\0".as_ptr() as *mut i8);
        if output.is_null() { free(out_buf as *mut c_void); }
        return ret;
    } else if !input.is_null() {
        in_buf = input;
    } else if fill.is_none() {
        error(b"NULL input pointer and missing fill function\0".as_ptr() as *mut i8);
        if output.is_null() { free(out_buf as *mut c_void); }
        return ret;
    } else {
        in_buf = malloc(lzo1x_worst_compress(LZO_BLOCK_SIZE)) as *mut U8;
        if in_buf.is_null() {
            error(b"Could not allocate input buffer\0".as_ptr() as *mut i8);
            if output.is_null() { free(out_buf as *mut c_void); }
            return ret;
        }
    }
    in_buf_save = in_buf;
    if !posp.is_null() { *posp = 0; }
    if let Some(f) = fill {
        in_buf = in_buf.add(HEADER_SIZE_MAX);
        in_len = f(in_buf as *mut c_void, HEADER_SIZE_MAX);
    }
    if parse_header(in_buf, &mut skip, in_len) == 0 {
        error(b"invalid header\0".as_ptr() as *mut i8);
        if input.is_null() { free(in_buf_save as *mut c_void); }
        if output.is_null() { free(out_buf as *mut c_void); }
        return ret;
    }
    in_buf = in_buf.offset(skip);
    in_len -= skip;
    if fill.is_some() {
        memcpy(in_buf_save as *mut c_void, in_buf as *const c_void, in_len as usize);
        in_buf = in_buf_save;
    }
    if !posp.is_null() { *posp = skip; }

    loop {
        if let Some(f) = fill {
            if in_len < 4 {
                skip = f(in_buf.offset(in_len) as *mut c_void, (4 - in_len) as usize);
                if skip > 0 { in_len += skip; }
            }
        }
        if in_len < 4 { error(b"file corrupted\0".as_ptr() as *mut i8); break; }
        dst_len = get_unaligned_be32(in_buf);
        in_buf = in_buf.add(4); in_len -= 4;
        if dst_len == 0 { if !posp.is_null() { *posp += 4; } ret = 0; break; }
        if dst_len as usize > LZO_BLOCK_SIZE { error(b"dest len longer than block size\0".as_ptr() as *mut i8); break; }
        if let Some(f) = fill {
            if in_len < 8 { skip = f(in_buf.offset(in_len) as *mut c_void, (8 - in_len) as usize); if skip > 0 { in_len += skip; } }
        }
        if in_len < 8 { error(b"file corrupted\0".as_ptr() as *mut i8); break; }
        src_len = get_unaligned_be32(in_buf); in_buf = in_buf.add(8); in_len -= 8;
        if src_len == 0 || src_len > dst_len { error(b"file corrupted\0".as_ptr() as *mut i8); break; }
        if let Some(f) = fill {
            if in_len < src_len as isize { skip = f(in_buf.offset(in_len) as *mut c_void, (src_len as isize - in_len) as usize); if skip > 0 { in_len += skip; } }
        }
        if in_len < src_len as isize { error(b"file corrupted\0".as_ptr() as *mut i8); break; }
        tmp = dst_len as usize;
        if dst_len == src_len { memcpy(out_buf as *mut c_void, in_buf as *const c_void, src_len as usize); }
        else { r = lzo1x_decompress_safe(in_buf, src_len as usize, out_buf, &mut tmp); if r != LZO_E_OK || dst_len as usize != tmp { error(b"Compressed data violation\0".as_ptr() as *mut i8); break; } }
        if let Some(f) = flush { if f(out_buf as *mut c_void, dst_len as usize) != dst_len as isize { break; } }
        if !output.is_null() { out_buf = out_buf.add(dst_len as usize); }
        if !posp.is_null() { *posp += src_len as isize + 12; }
        in_buf = in_buf.add(src_len as usize); in_len -= src_len as isize;
        if fill.is_some() { if in_len > 0 { for i in 0..in_len { *in_buf_save.offset(i) = *in_buf.offset(i); } } in_buf = in_buf_save; }
    }
    if input.is_null() { free(in_buf_save as *mut c_void); }
    if output.is_null() { free(out_buf as *mut c_void); }
    ret
}

#[allow(dead_code)]
unsafe fn __decompress(buf: *mut U8, len: isize, fill: Option<unsafe extern "C" fn(*mut c_void, usize) -> isize>, flush: Option<unsafe extern "C" fn(*mut c_void, usize) -> isize>, out_buf: *mut U8, _olen: isize, pos: *mut isize, error: unsafe extern "C" fn(*mut i8)) -> i32 {
    unlzo(buf, len, fill, flush, out_buf, pos, error)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
