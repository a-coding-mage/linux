// SPDX-License-Identifier: GPL-2.0-only
/*
 * Wrapper for decompressing LZ4-compressed kernel, initramfs, and initrd
 *
 * Copyright (C) 2013, LG Electronics, Kyungsik Lee <kyungsik.lee@lge.com>
 */

// In the C source, STATIC/PREBOOT and the included implementation are supplied
// by the surrounding build configuration.

use core::ffi::c_void;

pub const LZ4_DEFAULT_UNCOMPRESSED_CHUNK_SIZE: usize = 8usize << 20;
pub const ARCHIVE_MAGICNUMBER: u32 = 0x184C2102;

extern "C" {
    fn large_malloc(size: usize) -> *mut u8;
    fn large_free(ptr: *mut u8);
    fn LZ4_compressBound(input_size: usize) -> usize;
    fn LZ4_decompress_safe(
        source: *const u8,
        destination: *mut u8,
        compressed_size: i32,
        max_decompressed_size: i32,
    ) -> i32;
    fn get_unaligned_le32(ptr: *const u8) -> u32;
}

#[inline]
unsafe fn unlz4(
    mut input: *mut u8,
    in_len: isize,
    fill: Option<unsafe extern "C" fn(*mut c_void, usize) -> isize>,
    flush: Option<unsafe extern "C" fn(*mut c_void, usize) -> isize>,
    output: *mut u8,
    posp: *mut isize,
    error: unsafe extern "C" fn(*mut i8),
) -> i32 {
    let mut ret: i32 = -1;
    let mut chunksize: usize = 0;
    let uncomp_chunksize: usize = LZ4_DEFAULT_UNCOMPRESSED_CHUNK_SIZE;
    let mut inp: *mut u8;
    let mut inp_start: *mut u8;
    let mut outp: *mut u8;
    let mut size: isize = in_len;
    let mut dest_len: usize;

    if !output.is_null() {
        outp = output;
    } else if flush.is_none() {
        error(b"NULL output pointer and no flush function provided\0".as_ptr() as *mut i8);
        return ret;
    } else {
        outp = large_malloc(uncomp_chunksize);
        if outp.is_null() {
            error(b"Could not allocate output buffer\0".as_ptr() as *mut i8);
            return ret;
        }
    }

    if !input.is_null() && fill.is_some() {
        error(b"Both input pointer and fill function provided,\0".as_ptr() as *mut i8);
        if output.is_null() { large_free(outp); }
        return ret;
    } else if !input.is_null() {
        inp = input;
    } else if fill.is_none() {
        error(b"NULL input pointer and missing fill function\0".as_ptr() as *mut i8);
        if output.is_null() { large_free(outp); }
        return ret;
    } else {
        inp = large_malloc(LZ4_compressBound(uncomp_chunksize));
        if inp.is_null() {
            error(b"Could not allocate input buffer\0".as_ptr() as *mut i8);
            if output.is_null() { large_free(outp); }
            return ret;
        }
    }
    inp_start = inp;

    if !posp.is_null() { *posp = 0; }

    if let Some(fill_fn) = fill {
        size = fill_fn(inp as *mut c_void, 4);
        if size < 4 {
            error(b"data corrupted\0".as_ptr() as *mut i8);
            if input.is_null() { large_free(inp_start); }
            if output.is_null() { large_free(outp); }
            return ret;
        }
    }

    chunksize = get_unaligned_le32(inp) as usize;
    if chunksize != ARCHIVE_MAGICNUMBER as usize {
        error(b"invalid header\0".as_ptr() as *mut i8);
        if input.is_null() { large_free(inp_start); }
        if output.is_null() { large_free(outp); }
        return ret;
    }
    if fill.is_none() { inp = inp.add(4); size -= 4; }
    if !posp.is_null() { *posp += 4; }

    loop {
        if let Some(fill_fn) = fill {
            size = fill_fn(inp as *mut c_void, 4);
            if size == 0 { break; }
            if size < 4 { error(b"data corrupted\0".as_ptr() as *mut i8); break; }
        } else if size < 4 { break; }

        chunksize = get_unaligned_le32(inp) as usize;
        if chunksize == ARCHIVE_MAGICNUMBER as usize {
            if fill.is_none() { inp = inp.add(4); size -= 4; }
            if !posp.is_null() { *posp += 4; }
            continue;
        }
        if fill.is_none() && chunksize == 0 { break; }
        if !posp.is_null() { *posp += 4; }
        if fill.is_none() {
            inp = inp.add(4); size -= 4;
        } else {
            if chunksize > LZ4_compressBound(uncomp_chunksize) {
                error(b"chunk length is longer than allocated\0".as_ptr() as *mut i8); break;
            }
            size = fill.unwrap()(inp as *mut c_void, chunksize);
            if size < chunksize as isize { error(b"data corrupted\0".as_ptr() as *mut i8); break; }
        }

        dest_len = uncomp_chunksize;
        ret = LZ4_decompress_safe(inp, outp, chunksize as i32, dest_len as i32);
        if ret < 0 { error(b"Decoding failed\0".as_ptr() as *mut i8); break; }
        dest_len = ret as usize;
        ret = -1;
        if let Some(flush_fn) = flush {
            if flush_fn(outp as *mut c_void, dest_len) != dest_len as isize { break; }
        }
        if !output.is_null() { outp = outp.add(dest_len); }
        if !posp.is_null() { *posp += chunksize as isize; }
        if fill.is_none() {
            size -= chunksize as isize;
            if size == 0 { break; }
            if size < 0 { error(b"data corrupted\0".as_ptr() as *mut i8); break; }
            inp = inp.add(chunksize);
        }
    }

    ret = 0;
    if input.is_null() { large_free(inp_start); }
    if output.is_null() { large_free(outp); }
    ret
}

// The PREBOOT-only __decompress entry point is retained as a conditional item.
#[cfg(feature = "preboot")]
unsafe fn __decompress(
    buf: *mut u8, in_len: isize,
    fill: Option<unsafe extern "C" fn(*mut c_void, usize) -> isize>,
    flush: Option<unsafe extern "C" fn(*mut c_void, usize) -> isize>,
    output: *mut u8, _out_len: isize, posp: *mut isize,
    error: unsafe extern "C" fn(*mut i8),
) -> i32 {
    unlz4(buf, in_len - 4, fill, flush, output, posp, error)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
