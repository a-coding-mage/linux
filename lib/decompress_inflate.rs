// SPDX-License-Identifier: GPL-2.0
// C conditional compilation and included zlib sources are supplied by the surrounding build.

use core::ffi::c_void;

const GZIP_IOBUF_SIZE: usize = 16 * 1024;

type FillFn = unsafe extern "C" fn(*mut c_void, usize) -> isize;
type FlushFn = unsafe extern "C" fn(*mut c_void, usize) -> isize;
type ErrorFn = unsafe extern "C" fn(*mut i8);

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

// Supplied by zlib_inflate and the kernel decompression environment.
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut u8,
    pub avail_in: isize,
    pub next_out: *mut u8,
    pub avail_out: isize,
    pub workspace: *mut c_void,
}

extern "C" {
    fn zlib_inflate_workspacesize() -> usize;
    fn zlib_inflateInit2(strm: *mut z_stream_s, window_bits: i32) -> i32;
    fn zlib_inflate(strm: *mut z_stream_s, flush: i32) -> i32;
    fn zlib_inflateEnd(strm: *mut z_stream_s) -> i32;
}

const Z_OK: i32 = 0;
const Z_STREAM_END: i32 = 1;
const MAX_WBITS: i32 = 15;

unsafe extern "C" fn nofill(_buffer: *mut c_void, _len: usize) -> isize {
    -1
}

/* Included from initramfs et al code */
unsafe extern "C" fn __gunzip(
    mut buf: *mut u8,
    mut len: isize,
    mut fill: Option<FillFn>,
    flush: Option<FlushFn>,
    mut out_buf: *mut u8,
    mut out_len: isize,
    pos: *mut isize,
    error: ErrorFn,
) -> i32 {
    let mut zbuf: *mut u8;
    let strm: *mut z_stream_s;
    let mut rc: i32 = -1;

    if flush.is_some() {
        out_len = 0x8000; /* 32 K */
        out_buf = malloc(out_len as usize) as *mut u8;
    } else if out_len == 0 {
        out_len = (usize::MAX - out_buf as usize) as isize; /* no limit */
    }
    if out_buf.is_null() {
        error(b"Out of memory while allocating output buffer\0".as_ptr() as *mut i8);
        return rc;
    }

    if !buf.is_null() {
        zbuf = buf;
    } else {
        zbuf = malloc(GZIP_IOBUF_SIZE) as *mut u8;
        len = 0;
    }
    if zbuf.is_null() {
        error(b"Out of memory while allocating input buffer\0".as_ptr() as *mut i8);
        if flush.is_some() { free(out_buf as *mut c_void); }
        return rc;
    }

    strm = malloc(core::mem::size_of::<z_stream_s>()) as *mut z_stream_s;
    if strm.is_null() {
        error(b"Out of memory while allocating z_stream\0".as_ptr() as *mut i8);
        if buf.is_null() { free(zbuf as *mut c_void); }
        if flush.is_some() { free(out_buf as *mut c_void); }
        return rc;
    }

    (*strm).workspace = malloc(zlib_inflate_workspacesize());
    if (*strm).workspace.is_null() {
        error(b"Out of memory while allocating workspace\0".as_ptr() as *mut i8);
        free(strm as *mut c_void);
        if buf.is_null() { free(zbuf as *mut c_void); }
        free(out_buf as *mut c_void);
        return rc;
    }

    if fill.is_none() { fill = Some(nofill); }
    if len == 0 { len = fill.unwrap()(zbuf as *mut c_void, GZIP_IOBUF_SIZE); }

    /* verify the gzip header */
    if len < 10 || *zbuf != 0x1f || *zbuf.add(1) != 0x8b || *zbuf.add(2) != 0x08 {
        if !pos.is_null() { *pos = 0; }
        error(b"Not a gzip file\0".as_ptr() as *mut i8);
        free((*strm).workspace); free(strm as *mut c_void);
        if buf.is_null() { free(zbuf as *mut c_void); }
        if flush.is_some() { free(out_buf as *mut c_void); } return rc;
    }

    (*strm).next_in = zbuf.add(10);
    (*strm).avail_in = len - 10;
    if *zbuf.add(3) & 0x8 != 0 {
        loop {
            if (*strm).avail_in == 0 {
                error(b"header error\0".as_ptr() as *mut i8);
                free((*strm).workspace); free(strm as *mut c_void);
                if buf.is_null() { free(zbuf as *mut c_void); }
                if flush.is_some() { free(out_buf as *mut c_void); }
                return rc;
            }
            (*strm).avail_in -= 1;
            let p = (*strm).next_in;
            (*strm).next_in = p.add(1);
            if *p == 0 { break; }
        }
    }

    (*strm).next_out = out_buf;
    (*strm).avail_out = out_len;
    rc = zlib_inflateInit2(strm, -MAX_WBITS);

    while rc == Z_OK {
        if (*strm).avail_in == 0 {
            len = fill.unwrap()(zbuf as *mut c_void, GZIP_IOBUF_SIZE);
            if len < 0 { rc = -1; error(b"read error\0".as_ptr() as *mut i8); break; }
            (*strm).next_in = zbuf; (*strm).avail_in = len;
        }
        rc = zlib_inflate(strm, 0);
        if flush.is_some() && (*strm).next_out > out_buf {
            let l = (*strm).next_out.offset_from(out_buf) as isize;
            if flush.unwrap()(out_buf as *mut c_void, l as usize) != l {
                rc = -1; error(b"write error\0".as_ptr() as *mut i8); break;
            }
            (*strm).next_out = out_buf; (*strm).avail_out = out_len;
        }
        if rc == Z_STREAM_END { rc = 0; break; }
        if rc != Z_OK { error(b"uncompression error\0".as_ptr() as *mut i8); rc = -1; }
    }

    zlib_inflateEnd(strm);
    if !pos.is_null() { *pos = (*strm).next_in.offset_from(zbuf) as isize + 8; }
    free((*strm).workspace); free(strm as *mut c_void);
    if buf.is_null() { free(zbuf as *mut c_void); }
    if flush.is_some() { free(out_buf as *mut c_void); }
    rc
}

#[inline(never)]
// The non-PREBOOT and PREBOOT entry points are selected by the original build configuration.
#[cfg(not(feature = "preboot"))]
unsafe extern "C" fn gunzip(buf: *mut u8, len: isize, fill: Option<FillFn>, flush: Option<FlushFn>, out_buf: *mut u8, pos: *mut isize, error: ErrorFn) -> i32 {
    __gunzip(buf, len, fill, flush, out_buf, 0, pos, error)
}

#[cfg(feature = "preboot")]
unsafe extern "C" fn __decompress(buf: *mut u8, len: isize, fill: Option<FillFn>, flush: Option<FlushFn>, out_buf: *mut u8, out_len: isize, pos: *mut isize, error: ErrorFn) -> i32 {
    __gunzip(buf, len, fill, flush, out_buf, out_len, pos, error)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
