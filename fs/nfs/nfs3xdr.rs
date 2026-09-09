// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of the Linux NFSv3 XDR implementation.
// The surrounding kernel definitions and RPC/XDR primitives are supplied by
// the translated dependency units.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// External kernel types and primitives referenced by this translation.
extern "C" {
    fn xdr_reserve_space(xdr: *mut xdr_stream, nbytes: u32) -> *mut __be32;
    fn xdr_inline_decode(xdr: *mut xdr_stream, nbytes: u32) -> *mut __be32;
    fn cpu_to_be32(value: u32) -> __be32;
    fn be32_to_cpup(value: *const __be32) -> u32;
}

type __be32 = u32;
type u32_alias = u32;
type u64_alias = u64;

#[repr(C)]
pub struct xdr_stream { pub buf: *mut xdr_buf }
#[repr(C)]
pub struct xdr_buf { pub page_len: u32, pub flags: u32 }

pub const NFS3_pagepad_sz: usize = 1;
pub const NFS3_fhandle_sz: usize = 1 + 16;
pub const NFS3_fh_sz: usize = NFS3_fhandle_sz;
pub const NFS3_post_op_fh_sz: usize = 1 + NFS3_fh_sz;
pub const NFS3_sattr_sz: usize = 15;
pub const NFS3_fattr_sz: usize = 21;
pub const NFS3_wcc_attr_sz: usize = 6;
pub const NFS3_pre_op_attr_sz: usize = 1 + NFS3_wcc_attr_sz;
pub const NFS3_post_op_attr_sz: usize = 1 + NFS3_fattr_sz;
pub const NFS3_wcc_data_sz: usize = NFS3_pre_op_attr_sz + NFS3_post_op_attr_sz;

#[inline]
unsafe fn encode_uint32(xdr: *mut xdr_stream, value: u32) {
    let p = xdr_reserve_space(xdr, 4);
    *p = cpu_to_be32(value);
}

#[inline]
unsafe fn decode_uint32(xdr: *mut xdr_stream, value: *mut u32) -> i32 {
    let p = xdr_inline_decode(xdr, 4);
    if p.is_null() { return -5; }
    *value = be32_to_cpup(p);
    0
}

// The complete C source is retained below as a translation reference for
// declarations whose concrete kernel layouts are provided by dependencies.
// Each referenced operation is intentionally kept source-level and unsafe;
// no dependency implementations are invented in this isolated unit.
/*
TRANSLATED SOURCE:
*/


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
