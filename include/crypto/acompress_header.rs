/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Asynchronous Compression operations
 *
 * Copyright (c) 2016, Intel Corporation
 * Authors: Weigang Li <weigang.li@intel.com>
 *          Giovanni Cabiddu <giovanni.cabiddu@intel.com>
 */

// C includes are supplied by the surrounding kernel translation unit.

pub const CRYPTO_ACOMP_REQ_SRC_VIRT: u32 = 0x00000002;
pub const CRYPTO_ACOMP_REQ_SRC_NONDMA: u32 = 0x00000004;
pub const CRYPTO_ACOMP_REQ_DST_VIRT: u32 = 0x00000008;
pub const CRYPTO_ACOMP_REQ_DST_NONDMA: u32 = 0x00000010;
pub const CRYPTO_ACOMP_REQ_PRIVATE: u32 = CRYPTO_ACOMP_REQ_SRC_VIRT
    | CRYPTO_ACOMP_REQ_SRC_NONDMA
    | CRYPTO_ACOMP_REQ_DST_VIRT
    | CRYPTO_ACOMP_REQ_DST_NONDMA;
pub const CRYPTO_ACOMP_DST_MAX: u32 = 131072;
pub const MAX_SYNC_COMP_REQSIZE: usize = 0;

#[repr(C)]
pub struct acomp_req {
    pub base: crypto_async_request,
    pub src_or_svirt: acomp_req_src,
    pub dst_or_dvirt: acomp_req_dst,
    pub slen: c_uint,
    pub dlen: c_uint,
    pub chain: acomp_req_chain,
    pub __ctx: [c_void; 0],
}

#[repr(C)]
pub union acomp_req_src {
    pub src: *mut scatterlist,
    pub svirt: *const u8,
}

#[repr(C)]
pub union acomp_req_dst {
    pub dst: *mut scatterlist,
    pub dvirt: *mut u8,
}

#[repr(C)]
pub struct acomp_req_chain {
    pub compl: crypto_completion_t,
    pub data: *mut c_void,
    pub ssg: scatterlist,
    pub dsg: scatterlist,
    pub src_or_sfolio: acomp_req_chain_src,
    pub dst_or_dfolio: acomp_req_chain_dst,
    pub flags: u32,
}

#[repr(C)]
pub union acomp_req_chain_src {
    pub src: *const u8,
    pub sfolio: *mut folio,
}

#[repr(C)]
pub union acomp_req_chain_dst {
    pub dst: *mut u8,
    pub dfolio: *mut folio,
}

#[repr(C)]
pub struct crypto_acomp {
    pub compress: Option<unsafe extern "C" fn(*mut acomp_req) -> c_int>,
    pub decompress: Option<unsafe extern "C" fn(*mut acomp_req) -> c_int>,
    pub reqsize: c_uint,
    pub fb: *mut c_void,
    pub base: crypto_tfm,
}

#[repr(C)]
pub struct comp_alg_common {
    pub base: crypto_alg,
}

extern "C" {
    pub fn crypto_alloc_acomp(alg_name: *const c_char, type_: u32, mask: u32) -> *mut crypto_acomp;
    pub fn crypto_alloc_acomp_node(alg_name: *const c_char, type_: u32, mask: u32, node: c_int) -> *mut crypto_acomp;
    pub fn crypto_acomp_compress(req: *mut acomp_req) -> c_int;
    pub fn crypto_acomp_decompress(req: *mut acomp_req) -> c_int;
    pub fn acomp_request_clone(req: *mut acomp_req, total: usize, gfp: gfp_t) -> *mut acomp_req;
}

#[inline]
pub unsafe fn crypto_acomp_tfm(tfm: *mut crypto_acomp) -> *mut crypto_tfm {
    &mut (*tfm).base
}

#[inline]
pub unsafe fn __crypto_comp_alg_common(alg: *mut crypto_alg) -> *mut comp_alg_common {
    alg as *mut comp_alg_common
}

#[inline]
pub unsafe fn __crypto_acomp_tfm(tfm: *mut crypto_tfm) -> *mut crypto_acomp {
    tfm as *mut crypto_acomp
}

#[inline]
pub unsafe fn crypto_comp_alg_common(tfm: *mut crypto_acomp) -> *mut comp_alg_common {
    __crypto_comp_alg_common((*crypto_acomp_tfm(tfm)).__crt_alg)
}

#[inline]
pub unsafe fn crypto_acomp_reqsize(tfm: *mut crypto_acomp) -> c_uint {
    (*tfm).reqsize
}

#[inline]
pub unsafe fn acomp_request_set_tfm(req: *mut acomp_req, tfm: *mut crypto_acomp) {
    crypto_request_set_tfm(&mut (*req).base, crypto_acomp_tfm(tfm));
}

#[inline]
pub unsafe fn acomp_is_async(tfm: *mut crypto_acomp) -> bool {
    ((*crypto_comp_alg_common(tfm)).base.cra_flags & CRYPTO_ALG_ASYNC) != 0
}

#[inline]
pub unsafe fn crypto_acomp_reqtfm(req: *mut acomp_req) -> *mut crypto_acomp {
    __crypto_acomp_tfm((*req).base.tfm)
}

#[inline]
pub unsafe fn crypto_free_acomp(tfm: *mut crypto_acomp) {
    crypto_destroy_tfm(tfm as *mut c_void, crypto_acomp_tfm(tfm));
}

#[inline]
pub unsafe fn crypto_has_acomp(alg_name: *const c_char, mut type_: u32, mut mask: u32) -> c_int {
    type_ &= !CRYPTO_ALG_TYPE_MASK;
    type_ |= CRYPTO_ALG_TYPE_ACOMPRESS;
    mask |= CRYPTO_ALG_TYPE_ACOMPRESS_MASK;
    crypto_has_alg(alg_name, type_, mask)
}

#[inline]
pub unsafe fn crypto_acomp_alg_name(tfm: *mut crypto_acomp) -> *const c_char {
    crypto_tfm_alg_name(crypto_acomp_tfm(tfm))
}

#[inline]
pub unsafe fn crypto_acomp_driver_name(tfm: *mut crypto_acomp) -> *const c_char {
    crypto_tfm_alg_driver_name(crypto_acomp_tfm(tfm))
}

#[inline]
pub unsafe fn acomp_request_alloc_extra_noprof(tfm: *mut crypto_acomp, extra: usize, gfp: gfp_t) -> *mut acomp_req {
    let len = align_up(core::mem::size_of::<acomp_req>() + crypto_acomp_reqsize(tfm) as usize, CRYPTO_MINALIGN);
    let len = match len.checked_add(extra) { Some(v) => v, None => return core::ptr::null_mut() };
    let req = kzalloc_noprof(len, gfp) as *mut acomp_req;
    if !req.is_null() { acomp_request_set_tfm(req, tfm); }
    req
}

#[inline]
pub unsafe fn acomp_request_extra(req: *mut acomp_req) -> *mut c_void {
    let len = align_up(core::mem::size_of::<acomp_req>() + crypto_acomp_reqsize(crypto_acomp_reqtfm(req)) as usize, CRYPTO_MINALIGN);
    (req as *mut u8).add(len) as *mut c_void
}

#[inline]
pub unsafe fn acomp_req_on_stack(req: *mut acomp_req) -> bool { crypto_req_on_stack(&(*req).base) }

#[inline]
pub unsafe fn acomp_request_free(req: *mut acomp_req) {
    if req.is_null() || acomp_req_on_stack(req) { return; }
    kfree_sensitive(req as *mut c_void);
}

#[inline]
pub unsafe fn acomp_request_set_callback(req: *mut acomp_req, mut flgs: u32, cmpl: crypto_completion_t, data: *mut c_void) {
    flgs &= !CRYPTO_ACOMP_REQ_PRIVATE;
    flgs |= (*req).base.flags & CRYPTO_ACOMP_REQ_PRIVATE;
    crypto_request_set_callback(&mut (*req).base, flgs, cmpl, data);
}

#[inline]
pub unsafe fn acomp_request_set_params(req: *mut acomp_req, src: *mut scatterlist, dst: *mut scatterlist, slen: c_uint, dlen: c_uint) {
    (*req).src_or_svirt.src = src; (*req).dst_or_dvirt.dst = dst; (*req).slen = slen; (*req).dlen = dlen;
    (*req).base.flags &= !(CRYPTO_ACOMP_REQ_SRC_VIRT | CRYPTO_ACOMP_REQ_SRC_NONDMA | CRYPTO_ACOMP_REQ_DST_VIRT | CRYPTO_ACOMP_REQ_DST_NONDMA);
}

#[inline]
pub unsafe fn acomp_request_set_src_sg(req: *mut acomp_req, src: *mut scatterlist, slen: c_uint) { (*req).src_or_svirt.src = src; (*req).slen = slen; (*req).base.flags &= !CRYPTO_ACOMP_REQ_SRC_NONDMA; (*req).base.flags &= !CRYPTO_ACOMP_REQ_SRC_VIRT; }
#[inline]
pub unsafe fn acomp_request_set_src_dma(req: *mut acomp_req, src: *const u8, slen: c_uint) { (*req).src_or_svirt.svirt = src; (*req).slen = slen; (*req).base.flags &= !CRYPTO_ACOMP_REQ_SRC_NONDMA; (*req).base.flags |= CRYPTO_ACOMP_REQ_SRC_VIRT; }
#[inline]
pub unsafe fn acomp_request_set_src_nondma(req: *mut acomp_req, src: *const u8, slen: c_uint) { (*req).src_or_svirt.svirt = src; (*req).slen = slen; (*req).base.flags |= CRYPTO_ACOMP_REQ_SRC_NONDMA | CRYPTO_ACOMP_REQ_SRC_VIRT; }
#[inline]
pub unsafe fn acomp_request_set_src_folio(req: *mut acomp_req, folio: *mut folio, off: usize, len: c_uint) { sg_init_table(&mut (*req).chain.ssg, 1); sg_set_folio(&mut (*req).chain.ssg, folio, len, off); acomp_request_set_src_sg(req, &mut (*req).chain.ssg, len); }
#[inline]
pub unsafe fn acomp_request_set_dst_sg(req: *mut acomp_req, dst: *mut scatterlist, dlen: c_uint) { (*req).dst_or_dvirt.dst = dst; (*req).dlen = dlen; (*req).base.flags &= !CRYPTO_ACOMP_REQ_DST_NONDMA; (*req).base.flags &= !CRYPTO_ACOMP_REQ_DST_VIRT; }
#[inline]
pub unsafe fn acomp_request_set_dst_dma(req: *mut acomp_req, dst: *mut u8, dlen: c_uint) { (*req).dst_or_dvirt.dvirt = dst; (*req).dlen = dlen; (*req).base.flags &= !CRYPTO_ACOMP_REQ_DST_NONDMA; (*req).base.flags |= CRYPTO_ACOMP_REQ_DST_VIRT; }
#[inline]
pub unsafe fn acomp_request_set_dst_nondma(req: *mut acomp_req, dst: *mut u8, dlen: c_uint) { (*req).dst_or_dvirt.dvirt = dst; (*req).dlen = dlen; (*req).base.flags |= CRYPTO_ACOMP_REQ_DST_NONDMA | CRYPTO_ACOMP_REQ_DST_VIRT; }
#[inline]
pub unsafe fn acomp_request_set_dst_folio(req: *mut acomp_req, folio: *mut folio, off: usize, len: c_uint) { sg_init_table(&mut (*req).chain.dsg, 1); sg_set_folio(&mut (*req).chain.dsg, folio, len, off); acomp_request_set_dst_sg(req, &mut (*req).chain.dsg, len); }

#[inline]
pub unsafe fn acomp_request_on_stack_init(buf: *mut c_char, tfm: *mut crypto_acomp) -> *mut acomp_req { let req = buf as *mut acomp_req; crypto_stack_request_init(&mut (*req).base, crypto_acomp_tfm(tfm)); req }

// Build-time C macros ACOMP_REQUEST_ON_STACK, ACOMP_REQUEST_CLONE, allocation
// wrappers, and alignment helpers are represented by their Rust equivalents or
// remain supplied by the surrounding translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
