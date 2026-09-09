/* SPDX-License-Identifier: GPL-2.0 */
/* XDR standard data types and function declarations (Rust translation). */

use core::ffi::{c_char, c_int, c_void};

pub type __be32 = u32;
pub type __u32 = u32;
pub type __u64 = u64;
pub type gfp_t = usize;
pub type ssize_t = isize;
pub type size_t = usize;
pub type u8 = core::primitive::u8;
pub type u32 = core::primitive::u32;

#[repr(C)] pub struct bio_vec { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct folio { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct rpc_rqst { _private: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvec { pub iov_base: *mut c_void, pub iov_len: usize }

pub const XDR_UNIT: usize = 4;
#[inline] pub const fn xdr_quadlen(l: usize) -> usize { (l + 3) >> 2 }
pub const XDR_MAX_NETOBJ: usize = 1024;

#[repr(C)] pub struct xdr_netobj { pub len: u32, pub data: *mut u8 }
#[repr(C)]
pub struct xdr_buf {
    pub head: [kvec; 1], pub tail: [kvec; 1], pub bvec: *mut bio_vec,
    pub pages: *mut *mut page, pub page_base: u32, pub page_len: u32,
    pub flags: u32, pub buflen: u32, pub len: u32,
}
pub const XDRBUF_READ: u32 = 0x01;
pub const XDRBUF_WRITE: u32 = 0x02;
pub const XDRBUF_SPARSE_PAGES: u32 = 0x04;

#[inline] pub unsafe fn xdr_buf_init(buf: *mut xdr_buf, start: *mut c_void, len: usize) {
    (*buf).head[0].iov_base = start; (*buf).head[0].iov_len = len;
    (*buf).tail[0].iov_len = 0; (*buf).pages = core::ptr::null_mut();
    (*buf).page_len = 0; (*buf).flags = 0; (*buf).len = 0; (*buf).buflen = len as u32;
}

pub const xdr_zero: __be32 = 0;
pub const xdr_one: __be32 = 1;
pub const xdr_two: __be32 = 2;
/* RPC constants are supplied by the surrounding RPC headers. */
extern "C" {
    pub fn xdr_encode_opaque_fixed(p: *mut __be32, ptr: *const c_void, len: u32) -> *mut __be32;
    pub fn xdr_encode_opaque(p: *mut __be32, ptr: *const c_void, len: u32) -> *mut __be32;
    pub fn xdr_encode_string(p: *mut __be32, s: *const c_char) -> *mut __be32;
    pub fn xdr_encode_netobj(p: *mut __be32, obj: *const xdr_netobj) -> *mut __be32;
    pub fn xdr_inline_pages(buf: *mut xdr_buf, buflen: u32, pages: *mut *mut page, base: u32, len: u32);
    pub fn xdr_terminate_string(buf: *const xdr_buf, len: u32);
    pub fn xdr_buf_pagecount(buf: *const xdr_buf) -> usize;
    pub fn xdr_alloc_bvec(buf: *mut xdr_buf, gfp: gfp_t) -> c_int;
    pub fn xdr_free_bvec(buf: *mut xdr_buf);
    pub fn xdr_buf_to_bvec(bvec: *mut bio_vec, size: u32, xdr: *const xdr_buf) -> c_int;
    pub fn xdr_buf_to_sg(buf: *const xdr_buf, offset: u32, len: u32, sg: *mut scatterlist, nsg: u32) -> c_int;
    pub fn xdr_buf_to_sg_alloc(buf: *const xdr_buf, offset: u32, len: u32, sg: *mut scatterlist, nents: u32, overflow: *mut *mut scatterlist, gfp: gfp_t) -> c_int;
}
pub const XDR_BUF_TO_SG_NENTS: usize = 8;

#[repr(C)] pub struct xdr_stream {
    pub p: *mut __be32, pub buf: *mut xdr_buf, pub end: *mut __be32, pub iov: *mut kvec,
    pub scratch: kvec, pub page_ptr: *mut *mut page, pub page_kaddr: *mut c_void,
    pub nwords: u32, pub rqst: *mut rpc_rqst,
}
pub type kxdreproc_t = unsafe extern "C" fn(*mut rpc_rqst, *mut xdr_stream, *const c_void);
pub type kxdrdproc_t = unsafe extern "C" fn(*mut rpc_rqst, *mut xdr_stream, *mut c_void) -> c_int;

extern "C" {
    pub fn xdr_reserve_space(xdr: *mut xdr_stream, nbytes: usize) -> *mut __be32;
    pub fn xdr_inline_decode(xdr: *mut xdr_stream, nbytes: usize) -> *mut __be32;
    pub fn __xdr_commit_encode(xdr: *mut xdr_stream);
    pub fn xdr_encode_hyper(p: *mut __be32, val: __u64) -> *mut __be32;
    pub fn xdr_decode_hyper(p: *mut __be32, val: *mut __u64) -> *mut __be32;
    pub fn xdr_encode_array(p: *mut __be32, s: *const c_void, len: u32) -> *mut __be32;
    pub fn xdr_encode_bool(p: *mut __be32, n: u32) -> *mut __be32;
    pub fn xdr_stream_decode_u32(xdr: *mut xdr_stream, p: *mut u32) -> ssize_t;
    pub fn xdr_stream_encode_u32(xdr: *mut xdr_stream, n: u32) -> ssize_t;
}

#[repr(C)] pub struct xdr_array2_desc {
    pub elem_size: u32, pub array_len: u32, pub array_maxlen: u32,
    pub xcode: Option<unsafe extern "C" fn(*mut xdr_array2_desc, *mut c_void) -> c_int>,
}
pub const XDR_ARRAY2_DESC_SIZE: usize = core::mem::size_of::<xdr_array2_desc>();

#[inline] pub const fn xdr_align_size(n: usize) -> usize { (n + (XDR_UNIT - 1)) & !(XDR_UNIT - 1) }
#[inline] pub const fn xdr_pad_size(n: usize) -> usize { xdr_align_size(n) - n }
#[inline] pub unsafe fn xdr_stream_remaining(xdr: *const xdr_stream) -> usize { ((*xdr).nwords as usize) << 2 }
#[inline] pub unsafe fn xdr_set_scratch_buffer(xdr: *mut xdr_stream, buf: *mut c_void, buflen: usize) { (*xdr).scratch = kvec { iov_base: buf, iov_len: buflen }; }
#[inline] pub unsafe fn xdr_reset_scratch_buffer(xdr: *mut xdr_stream) { xdr_set_scratch_buffer(xdr, core::ptr::null_mut(), 0); }
#[inline] pub unsafe fn xdr_commit_encode(xdr: *mut xdr_stream) { if (*xdr).scratch.iov_len != 0 { __xdr_commit_encode(xdr); } }

/* Remaining declarations and inline helpers retain their C ABI through the external implementation. */
extern "C" {
    pub fn xdr_buf_from_iov(iov: *const kvec, buf: *mut xdr_buf);
    pub fn xdr_buf_subsegment(buf: *const xdr_buf, sub: *mut xdr_buf, base: u32, len: u32) -> c_int;
    pub fn xdr_buf_trim(buf: *mut xdr_buf, len: u32);
    pub fn read_bytes_from_xdr_buf(buf: *const xdr_buf, base: u32, obj: *mut c_void, len: u32) -> c_int;
    pub fn write_bytes_to_xdr_buf(buf: *const xdr_buf, base: u32, obj: *mut c_void, len: u32) -> c_int;
    pub fn xdr_encode_word(buf: *const xdr_buf, base: u32, word: u32) -> c_int;
    pub fn xdr_decode_word(buf: *const xdr_buf, base: u32, word: *mut u32) -> c_int;
    pub fn xdr_decode_array2(buf: *const xdr_buf, base: u32, desc: *mut xdr_array2_desc) -> c_int;
    pub fn xdr_encode_array2(buf: *const xdr_buf, base: u32, desc: *mut xdr_array2_desc) -> c_int;
    pub fn _copy_from_pages(p: *mut c_char, pages: *mut *mut page, pgbase: usize, len: usize);
    pub fn xdr_init_encode(xdr: *mut xdr_stream, buf: *mut xdr_buf, p: *mut __be32, rqst: *mut rpc_rqst);
    pub fn xdr_init_encode_pages(xdr: *mut xdr_stream, buf: *mut xdr_buf);
    pub fn xdr_reserve_space_vec(xdr: *mut xdr_stream, nbytes: usize) -> c_int;
    pub fn xdr_truncate_encode(xdr: *mut xdr_stream, len: usize);
    pub fn xdr_truncate_decode(xdr: *mut xdr_stream, len: usize);
    pub fn xdr_restrict_buflen(xdr: *mut xdr_stream, len: c_int) -> c_int;
    pub fn xdr_write_pages(xdr: *mut xdr_stream, pages: *mut *mut page, base: u32, len: u32);
    pub fn xdr_stream_pos(xdr: *const xdr_stream) -> u32;
    pub fn xdr_page_pos(xdr: *const xdr_stream) -> u32;
    pub fn xdr_init_decode(xdr: *mut xdr_stream, buf: *mut xdr_buf, p: *mut __be32, rqst: *mut rpc_rqst);
    pub fn xdr_init_decode_pages(xdr: *mut xdr_stream, buf: *mut xdr_buf, pages: *mut *mut page, len: u32);
    pub fn xdr_finish_decode(xdr: *mut xdr_stream);
    pub fn xdr_read_pages(xdr: *mut xdr_stream, len: u32) -> u32;
    pub fn xdr_enter_page(xdr: *mut xdr_stream, len: u32);
    pub fn xdr_set_pagelen(xdr: *mut xdr_stream, len: u32);
    pub fn xdr_stream_subsegment(xdr: *mut xdr_stream, sub: *mut xdr_buf, len: u32) -> bool;
    pub fn xdr_stream_move_subsegment(xdr: *mut xdr_stream, offset: u32, target: u32, len: u32) -> u32;
    pub fn xdr_stream_zero(xdr: *mut xdr_stream, offset: u32, len: u32) -> u32;
    pub fn xdr_stream_decode_string_dup(xdr: *mut xdr_stream, s: *mut *mut c_char, maxlen: usize, gfp: gfp_t) -> ssize_t;
    pub fn xdr_stream_decode_opaque_auth(xdr: *mut xdr_stream, flavor: *mut u32, body: *mut *mut c_void, len: *mut u32) -> ssize_t;
    pub fn xdr_stream_encode_opaque_auth(xdr: *mut xdr_stream, flavor: u32, body: *mut c_void, len: u32) -> ssize_t;
    pub fn xdr_netobj_dup(dst: *mut xdr_netobj, src: *mut xdr_netobj, gfp: gfp_t);
    pub fn xdr_adjust_iovec(iov: *mut kvec, p: *mut __be32) -> c_int;
    pub fn xdr_stream_encode_item_present(xdr: *mut xdr_stream) -> ssize_t;
    pub fn xdr_stream_encode_item_absent(xdr: *mut xdr_stream) -> c_int;
    pub fn xdr_stream_encode_bool(xdr: *mut xdr_stream, n: u32) -> c_int;
    pub fn xdr_stream_encode_u64(xdr: *mut xdr_stream, n: u64) -> ssize_t;
    pub fn xdr_stream_decode_bool(xdr: *mut xdr_stream, p: *mut u32) -> ssize_t;
    pub fn xdr_stream_decode_u64(xdr: *mut xdr_stream, p: *mut u64) -> ssize_t;
    pub fn xdr_stream_decode_uint32_array(xdr: *mut xdr_stream, array: *mut u32, size: usize) -> ssize_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
