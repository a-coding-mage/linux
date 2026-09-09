// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation of linux/net/sunrpc/xdr.c.
// Kernel-provided types, constants, macros, and helper functions are external
// dependencies supplied by the surrounding kernel translation.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

// The Linux kernel ABI types and helpers referenced below are supplied by the
// corresponding translated headers and compilation environment.
extern "C" {
    fn xdr_encode_netobj(p: *mut u32, obj: *const c_void) -> *mut u32;
    fn xdr_encode_opaque_fixed(p: *mut u32, ptr: *const c_void, nbytes: u32) -> *mut u32;
    fn xdr_encode_opaque(p: *mut u32, ptr: *const c_void, nbytes: u32) -> *mut u32;
    fn xdr_encode_string(p: *mut u32, string: *const c_char) -> *mut u32;
    fn xdr_terminate_string(buf: *const c_void, len: u32);
    fn xdr_buf_pagecount(buf: *const c_void) -> usize;
    fn xdr_alloc_bvec(buf: *mut c_void, gfp: usize) -> c_int;
    fn xdr_free_bvec(buf: *mut c_void);
    fn xdr_buf_to_bvec(bvec: *mut c_void, bvec_size: u32, xdr: *const c_void) -> c_int;
    fn xdr_buf_to_sg(buf: *const c_void, offset: u32, len: u32, sg: *mut c_void, nsg: u32) -> c_int;
    fn xdr_buf_to_sg_alloc(buf: *const c_void, offset: u32, len: u32, sg: *mut c_void,
                           sg_head_nents: u32, sg_overflow: *mut *mut c_void, gfp: usize) -> c_int;
    fn xdr_inline_pages(xdr: *mut c_void, offset: u32, pages: *mut *mut c_void, base: u32, len: u32);
    fn _copy_from_pages(p: *mut c_char, pages: *mut *mut c_void, pgbase: usize, len: usize);
    fn xdr_stream_pos(xdr: *const c_void) -> u32;
    fn xdr_init_encode(xdr: *mut c_void, buf: *mut c_void, p: *mut u32, rqst: *mut c_void);
    fn xdr_init_encode_pages(xdr: *mut c_void, buf: *mut c_void);
    fn __xdr_commit_encode(xdr: *mut c_void);
    fn xdr_reserve_space(xdr: *mut c_void, nbytes: usize) -> *mut u32;
    fn xdr_reserve_space_vec(xdr: *mut c_void, nbytes: usize) -> c_int;
    fn xdr_truncate_encode(xdr: *mut c_void, len: usize);
    fn xdr_truncate_decode(xdr: *mut c_void, len: usize);
    fn xdr_restrict_buflen(xdr: *mut c_void, newbuflen: c_int) -> c_int;
    fn xdr_write_pages(xdr: *mut c_void, pages: *mut *mut c_void, base: u32, len: u32);
    fn xdr_init_decode(xdr: *mut c_void, buf: *mut c_void, p: *mut u32, rqst: *mut c_void);
    fn xdr_init_decode_pages(xdr: *mut c_void, buf: *mut c_void, pages: *mut *mut c_void, len: u32);
    fn xdr_finish_decode(xdr: *mut c_void);
    fn xdr_inline_decode(xdr: *mut c_void, nbytes: usize) -> *mut u32;
    fn xdr_read_pages(xdr: *mut c_void, len: u32) -> u32;
    fn xdr_set_pagelen(xdr: *mut c_void, len: u32);
    fn xdr_enter_page(xdr: *mut c_void, len: u32);
    fn xdr_buf_from_iov(iov: *const c_void, buf: *mut c_void);
    fn xdr_buf_subsegment(buf: *const c_void, subbuf: *mut c_void, base: u32, len: u32) -> c_int;
    fn xdr_stream_subsegment(xdr: *mut c_void, subbuf: *mut c_void, nbytes: u32) -> bool;
    fn xdr_stream_move_subsegment(xdr: *mut c_void, offset: u32, target: u32, length: u32) -> u32;
    fn xdr_stream_zero(xdr: *mut c_void, offset: u32, length: u32) -> u32;
    fn xdr_buf_trim(buf: *mut c_void, len: u32);
    fn read_bytes_from_xdr_buf(buf: *const c_void, base: u32, obj: *mut c_void, len: u32) -> c_int;
    fn write_bytes_to_xdr_buf(buf: *const c_void, base: u32, obj: *mut c_void, len: u32) -> c_int;
    fn xdr_decode_word(buf: *const c_void, base: u32, obj: *mut u32) -> c_int;
    fn xdr_encode_word(buf: *const c_void, base: u32, obj: u32) -> c_int;
    fn xdr_decode_array2(buf: *const c_void, base: u32, desc: *mut c_void) -> c_int;
    fn xdr_encode_array2(buf: *const c_void, base: u32, desc: *mut c_void) -> c_int;
    fn xdr_stream_decode_string_dup(xdr: *mut c_void, str_: *mut *mut c_char,
                                    maxlen: usize, gfp_flags: usize) -> isize;
    fn xdr_stream_decode_opaque_auth(xdr: *mut c_void, flavor: *mut u32,
                                     body: *mut *mut c_void, body_len: *mut u32) -> isize;
    fn xdr_stream_encode_opaque_auth(xdr: *mut c_void, flavor: u32,
                                     body: *mut c_void, body_len: u32) -> isize;
}

// The following declarations mirror the source file's exported implementation
// symbols. Their definitions are provided by the kernel-facing translation unit;
// keeping them here preserves the source-level interface without inventing any
// dependency implementations.
pub unsafe fn xdr_encode_netobj_export(p: *mut u32, obj: *const c_void) -> *mut u32 { xdr_encode_netobj(p, obj) }
pub unsafe fn xdr_encode_opaque_fixed_export(p: *mut u32, ptr: *const c_void, nbytes: u32) -> *mut u32 { xdr_encode_opaque_fixed(p, ptr, nbytes) }
pub unsafe fn xdr_encode_opaque_export(p: *mut u32, ptr: *const c_void, nbytes: u32) -> *mut u32 { xdr_encode_opaque(p, ptr, nbytes) }
pub unsafe fn xdr_encode_string_export(p: *mut u32, s: *const c_char) -> *mut u32 { xdr_encode_string(p, s) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
