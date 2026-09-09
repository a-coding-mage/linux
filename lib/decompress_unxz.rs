// SPDX-License-Identifier: 0BSD

/* Wrapper for decompressing XZ-compressed kernel, initramfs, and initrd. */
/*
 * The C source contains extensive notes about the in-place decompression
 * safety margin; that calculation is unchanged by this translation.
 *
 * STATIC/XZ_PREBOOT and architecture CONFIG_* selections are build-time C
 * conditions. Their corresponding decoder implementations and headers are
 * supplied by the surrounding build.
 */

use core::ffi::c_void;

#[repr(C)]
pub struct xz_buf {
    pub in_: *mut u8,
    pub in_pos: usize,
    pub in_size: usize,
    pub out: *mut u8,
    pub out_pos: usize,
    pub out_size: usize,
}

#[repr(C)]
pub struct xz_dec {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xz_ret {
    XZ_OK = 0,
    XZ_STREAM_END,
    XZ_MEM_ERROR,
    XZ_FORMAT_ERROR,
    XZ_OPTIONS_ERROR,
    XZ_DATA_ERROR,
    XZ_BUF_ERROR,
}

pub const XZ_SINGLE: u32 = 0;
pub const XZ_DYNALLOC: u32 = 1;
pub const XZ_IOBUF_SIZE: usize = 4096;

extern "C" {
    fn xz_crc32_init();
    fn xz_dec_init(mode: u32, dict_max: u32) -> *mut xz_dec;
    fn xz_dec_run(s: *mut xz_dec, b: *mut xz_buf) -> xz_ret;
    fn xz_dec_end(s: *mut xz_dec);
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

#[cfg(not(memeq))]
unsafe fn memeq(a: *const c_void, b: *const c_void, size: usize) -> bool {
    let x = a as *const u8;
    let y = b as *const u8;
    let mut i = 0;
    while i < size {
        if *x.add(i) != *y.add(i) {
            return false;
        }
        i += 1;
    }
    true
}

#[cfg(not(memzero))]
unsafe fn memzero(buf: *mut c_void, size: usize) {
    let mut b = buf as *mut u8;
    let e = b.add(size);
    while b != e {
        *b = 0;
        b = b.add(1);
    }
}

#[cfg(not(memmove))]
#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut c_void, src: *const c_void, size: usize) -> *mut c_void {
    let d = dest as *mut u8;
    let s = src as *const u8;
    if (d as usize) < (s as usize) {
        for i in 0..size {
            *d.add(i) = *s.add(i);
        }
    } else if (d as usize) > (s as usize) {
        let mut i = size;
        while i > 0 {
            i -= 1;
            *d.add(i) = *s.add(i);
        }
    }
    dest
}

pub type Fill = unsafe extern "C" fn(dest: *mut c_void, size: u64) -> i64;
pub type Flush = unsafe extern "C" fn(src: *mut c_void, size: u64) -> i64;
pub type Error = unsafe extern "C" fn(x: *mut i8);

#[no_mangle]
pub unsafe extern "C" fn unxz(
    mut input: *mut u8,
    mut in_size: i64,
    fill: Option<Fill>,
    flush: Option<Flush>,
    out: *mut u8,
    in_used: *mut i64,
    error: Error,
) -> i32 {
    let mut b: xz_buf;
    let s: *mut xz_dec;
    let mut ret: xz_ret;
    let mut must_free_in = false;

    xz_crc32_init();
    if !in_used.is_null() { *in_used = 0; }

    s = if fill.is_none() && flush.is_none() {
        xz_dec_init(XZ_SINGLE, 0)
    } else {
        xz_dec_init(XZ_DYNALLOC, u32::MAX)
    };
    if s.is_null() {
        error(c"XZ decompressor ran out of memory".as_ptr() as *mut i8);
        return -1;
    }

    if flush.is_none() {
        b.out = out;
        b.out_size = usize::MAX;
    } else {
        b.out_size = XZ_IOBUF_SIZE;
        b.out = malloc(XZ_IOBUF_SIZE) as *mut u8;
        if b.out.is_null() { xz_dec_end(s); error(c"XZ decompressor ran out of memory".as_ptr() as *mut i8); return -1; }
    }

    if input.is_null() {
        must_free_in = true;
        input = malloc(XZ_IOBUF_SIZE) as *mut u8;
        if input.is_null() {
            if flush.is_some() { free(b.out as *mut c_void); }
            xz_dec_end(s);
            error(c"XZ decompressor ran out of memory".as_ptr() as *mut i8);
            return -1;
        }
    }

    b.in_ = input; b.in_pos = 0; b.in_size = in_size as usize; b.out_pos = 0;
    if fill.is_none() && flush.is_none() {
        ret = xz_dec_run(s, &mut b);
    } else {
        loop {
            if b.in_pos == b.in_size {
                if let Some(f) = fill {
                    if !in_used.is_null() { *in_used += b.in_pos as i64; }
                    b.in_pos = 0;
                    in_size = f(input as *mut c_void, XZ_IOBUF_SIZE as u64);
                    if in_size < 0 { ret = xz_ret::XZ_BUF_ERROR; break; }
                    b.in_size = in_size as usize;
                }
            }
            ret = xz_dec_run(s, &mut b);
            if let Some(f) = flush {
                if b.out_pos == b.out_size || (ret != xz_ret::XZ_OK && b.out_pos > 0) {
                    if f(b.out as *mut c_void, b.out_pos as u64) != b.out_pos as i64 { ret = xz_ret::XZ_BUF_ERROR; }
                    b.out_pos = 0;
                }
            }
            if ret != xz_ret::XZ_OK { break; }
        }
        if must_free_in { free(input as *mut c_void); }
        if flush.is_some() { free(b.out as *mut c_void); }
    }
    if !in_used.is_null() { *in_used += b.in_pos as i64; }
    xz_dec_end(s);
    match ret {
        xz_ret::XZ_STREAM_END => 0,
        xz_ret::XZ_MEM_ERROR => { error(c"XZ decompressor ran out of memory".as_ptr() as *mut i8); -1 }
        xz_ret::XZ_FORMAT_ERROR => { error(c"Input is not in the XZ format (wrong magic bytes)".as_ptr() as *mut i8); -1 }
        xz_ret::XZ_OPTIONS_ERROR => { error(c"Input was encoded with settings that are not supported by this XZ decoder".as_ptr() as *mut i8); -1 }
        xz_ret::XZ_DATA_ERROR | xz_ret::XZ_BUF_ERROR => { error(c"XZ-compressed data is corrupt".as_ptr() as *mut i8); -1 }
        _ => { error(c"Bug in the XZ decompressor".as_ptr() as *mut i8); -1 }
    }
}

#[cfg(XZ_PREBOOT)]
pub unsafe extern "C" fn __decompress(
    input: *mut u8, in_size: i64, fill: Option<Fill>, flush: Option<Flush>, out: *mut u8,
    _out_size: i64, in_used: *mut i64, error: Error,
) -> i32 {
    unxz(input, in_size, fill, flush, out, in_used, error)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
