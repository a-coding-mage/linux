// SPDX-License-Identifier: GPL-2.0
// Rust translation of lib/decompress_inflate.c, preserving the native wrapper
// semantics (including its limited gzip-header handling and unchecked trailer).

use crate::bindings::*;
use crate::zlib_inflate::{
    zlib_inflate, zlib_inflateEnd, zlib_inflateInit2, zlib_inflate_workspacesize,
};
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::{mem, ptr};

const GZIP_IOBUF_SIZE: c_ulong = 16 * 1024;
pub(crate) type FillFn = unsafe extern "C" fn(*mut c_void, c_ulong) -> c_long;
pub(crate) type FlushFn = unsafe extern "C" fn(*mut c_void, c_ulong) -> c_long;
pub(crate) type ErrorFn = unsafe extern "C" fn(*mut c_char);

// include/linux/decompress/mm.h's STATIC allocator takes int, not size_t.
// The boot extraction module provides these native symbols.
#[cfg(PREBOOT)]
extern "C" {
    #[link_name = "malloc"]
    fn boot_malloc(size: c_int) -> *mut c_void;
    fn free(pointer: *mut c_void);
}

#[cfg(not(PREBOOT))]
extern "C" {
    fn lupos_zlib_kmalloc(size: usize, flags: gfp_t) -> *mut c_void;
    #[link_name = "lupos_zlib_kfree"]
    fn free(pointer: *mut c_void);
}

#[inline]
unsafe fn malloc(size: usize) -> *mut c_void {
    unsafe {
        #[cfg(PREBOOT)]
        {
            boot_malloc(size as c_int)
        }
        #[cfg(not(PREBOOT))]
        {
            lupos_zlib_kmalloc(size, GFP_KERNEL as gfp_t)
        }
    }
}

#[cfg_attr(not(PREBOOT), link_section = ".init.text")]
unsafe extern "C" fn nofill(_buffer: *mut c_void, _len: c_ulong) -> c_long {
    -1
}

#[cfg_attr(not(PREBOOT), link_section = ".init.text")]
unsafe fn __gunzip(
    buf: *mut u8,
    mut len: c_long,
    fill: Option<FillFn>,
    flush: Option<FlushFn>,
    mut out_buf: *mut u8,
    mut out_len: c_long,
    pos: *mut c_long,
    error: ErrorFn,
) -> c_int {
    unsafe {
        let mut rc = -1;
        if flush.is_some() {
            out_len = 0x8000;
            out_buf = malloc(out_len as usize).cast();
        } else if out_len == 0 {
            out_len = usize::MAX.wrapping_sub(out_buf as usize) as c_long;
        }
        if out_buf.is_null() {
            error(b"Out of memory while allocating output buffer\0".as_ptr() as *mut c_char);
            return rc;
        }
        let zbuf = if !buf.is_null() {
            buf
        } else {
            len = 0;
            malloc(GZIP_IOBUF_SIZE as usize).cast::<u8>()
        };
        if zbuf.is_null() {
            error(b"Out of memory while allocating input buffer\0".as_ptr() as *mut c_char);
            if flush.is_some() {
                free(out_buf.cast());
            }
            return rc;
        }
        let strm = malloc(mem::size_of::<z_stream>()).cast::<z_stream>();
        if strm.is_null() {
            error(b"Out of memory while allocating z_stream\0".as_ptr() as *mut c_char);
            if buf.is_null() {
                free(zbuf.cast());
            }
            if flush.is_some() {
                free(out_buf.cast());
            }
            return rc;
        }
        #[cfg(not(CONFIG_ZLIB_DFLTCC))]
        let workspace_size = if flush.is_some() {
            zlib_inflate_workspacesize()
        } else {
            mem::size_of::<inflate_state>() as c_int
        };
        #[cfg(CONFIG_ZLIB_DFLTCC)]
        let workspace_size = zlib_inflate_workspacesize();
        (*strm).workspace = malloc(workspace_size as usize);
        if (*strm).workspace.is_null() {
            error(b"Out of memory while allocating workspace\0".as_ptr() as *mut c_char);
            free(strm.cast());
            if buf.is_null() {
                free(zbuf.cast());
            }
            if flush.is_some() {
                free(out_buf.cast());
            }
            return rc;
        }
        let fill = fill.unwrap_or(nofill);
        // The single exit block below corresponds to gunzip_5; allocation
        // failures above preserve each earlier native cleanup label.
        'gunzip_5: {
            if len == 0 {
                len = fill(zbuf.cast(), GZIP_IOBUF_SIZE);
            }
            if len < 10 || *zbuf != 0x1f || *zbuf.add(1) != 0x8b || *zbuf.add(2) != 0x08 {
                if !pos.is_null() {
                    *pos = 0;
                }
                error(b"Not a gzip file\0".as_ptr() as *mut c_char);
                break 'gunzip_5;
            }
            (*strm).next_in = zbuf.add(10);
            (*strm).avail_in = (len - 10) as c_ulong;
            if *zbuf.add(3) & 0x8 != 0 {
                loop {
                    if (*strm).avail_in == 0 {
                        error(b"header error\0".as_ptr() as *mut c_char);
                        break 'gunzip_5;
                    }
                    (*strm).avail_in -= 1;
                    let ch = *(*strm).next_in;
                    (*strm).next_in = (*strm).next_in.add(1);
                    if ch == 0 {
                        break;
                    }
                }
            }
            (*strm).next_out = out_buf;
            (*strm).avail_out = out_len as c_ulong;
            rc = zlib_inflateInit2(strm, -(MAX_WBITS as c_int));
            #[cfg(not(CONFIG_ZLIB_DFLTCC))]
            if flush.is_none() {
                let state = (*strm).workspace.cast::<inflate_state>();
                (*state).wsize = 0;
                (*state).window = ptr::null_mut();
            }
            while rc == Z_OK as c_int {
                if (*strm).avail_in == 0 {
                    // Native TODO: simultaneous pos and fill are not handled.
                    len = fill(zbuf.cast(), GZIP_IOBUF_SIZE);
                    if len < 0 {
                        rc = -1;
                        error(b"read error\0".as_ptr() as *mut c_char);
                        break;
                    }
                    (*strm).next_in = zbuf;
                    (*strm).avail_in = len as c_ulong;
                }
                rc = zlib_inflate(strm, 0);
                if let Some(flush) = flush {
                    if (*strm).next_out > out_buf {
                        let written = (*strm).next_out.offset_from(out_buf) as c_long;
                        if written != flush(out_buf.cast(), written as c_ulong) {
                            rc = -1;
                            error(b"write error\0".as_ptr() as *mut c_char);
                            break;
                        }
                        (*strm).next_out = out_buf;
                        (*strm).avail_out = out_len as c_ulong;
                    }
                }
                if rc == Z_STREAM_END as c_int {
                    rc = 0;
                    break;
                } else if rc != Z_OK as c_int {
                    error(b"uncompression error\0".as_ptr() as *mut c_char);
                    rc = -1;
                }
            }
            zlib_inflateEnd(strm);
            if !pos.is_null() {
                *pos = ((*strm).next_in.offset_from(zbuf) as c_long).wrapping_add(8);
            }
        }
        free((*strm).workspace);
        free(strm.cast());
        if buf.is_null() {
            free(zbuf.cast());
        }
        if flush.is_some() {
            free(out_buf.cast());
        }
        rc
    }
}

#[cfg(not(PREBOOT))]
#[no_mangle]
#[link_section = ".init.text"]
pub(crate) unsafe extern "C" fn gunzip(
    buf: *mut u8,
    len: c_long,
    fill: Option<FillFn>,
    flush: Option<FlushFn>,
    out_buf: *mut u8,
    pos: *mut c_long,
    error: ErrorFn,
) -> c_int {
    unsafe { __gunzip(buf, len, fill, flush, out_buf, 0, pos, error) }
}

#[cfg(PREBOOT)]
#[no_mangle]
pub(crate) unsafe extern "C" fn __decompress(
    buf: *mut u8,
    len: c_long,
    fill: Option<FillFn>,
    flush: Option<FlushFn>,
    out_buf: *mut u8,
    out_len: c_long,
    pos: *mut c_long,
    error: ErrorFn,
) -> c_int {
    unsafe { __gunzip(buf, len, fill, flush, out_buf, out_len, pos, error) }
}
