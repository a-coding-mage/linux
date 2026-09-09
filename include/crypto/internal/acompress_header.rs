/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Asynchronous Compression operations
 *
 * Copyright (c) 2016, Intel Corporation
 * Authors: Weigang Li <weigang.li@intel.com>
 *          Giovanni Cabiddu <giovanni.cabiddu@intel.com>
 */

use core::ffi::{c_char, c_int, c_void};

// #include <crypto/acompress.h>
// #include <crypto/algapi.h>
// #include <crypto/scatterwalk.h>
// #include <linux/compiler_types.h>
// #include <linux/cpumask_types.h>
// #include <linux/spinlock.h>
// #include <linux/workqueue_types.h>

// Equivalent of `ACOMP_FBREQ_ON_STACK(name, req)`; C token-pasting and
// stack-allocation details remain build-context dependent.

#[repr(C)]
pub struct acomp_alg {
    pub compress: Option<unsafe extern "C" fn(req: *mut acomp_req) -> c_int>,
    pub decompress: Option<unsafe extern "C" fn(req: *mut acomp_req) -> c_int>,
    pub init: Option<unsafe extern "C" fn(tfm: *mut crypto_acomp) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(tfm: *mut crypto_acomp)>,
    pub calg: comp_alg_common,
}

#[repr(C)]
pub struct crypto_acomp_stream {
    pub lock: spinlock_t,
    pub ctx: *mut c_void,
}

#[repr(C)]
pub struct crypto_acomp_streams {
    // These must come first because of struct scomp_alg.
    pub alloc_ctx: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub free_ctx: Option<unsafe extern "C" fn(*mut c_void)>,
    pub streams: *mut crypto_acomp_stream,
    pub stream_work: work_struct,
    pub stream_want: cpumask_t,
}

#[repr(C)]
pub struct acomp_walk {
    pub src: acomp_walk_src,
    pub dst: acomp_walk_dst,
    pub slen: u32,
    pub dlen: u32,
    pub flags: c_int,
}

#[repr(C)]
pub union acomp_walk_src {
    pub virt: acomp_walk_src_virt,
    pub in_: scatter_walk,
}

#[repr(C)]
pub struct acomp_walk_src_virt {
    pub addr: *const c_void,
}

#[repr(C)]
pub union acomp_walk_dst {
    pub virt: acomp_walk_dst_virt,
    pub out: scatter_walk,
}

#[repr(C)]
pub struct acomp_walk_dst_virt {
    pub addr: *mut c_void,
}

/* Transform internal helpers. */
#[inline]
pub unsafe fn acomp_request_ctx(req: *mut acomp_req) -> *mut c_void {
    (*req).__ctx
}

#[inline]
pub unsafe fn acomp_tfm_ctx(tfm: *mut crypto_acomp) -> *mut c_void {
    (*tfm).base.__crt_ctx
}

#[inline]
pub unsafe fn acomp_request_complete(req: *mut acomp_req, err: c_int) {
    crypto_request_complete(&mut (*req).base, err);
}

extern "C" {
    pub fn crypto_register_acomp(alg: *mut acomp_alg) -> c_int;
    pub fn crypto_unregister_acomp(alg: *mut acomp_alg);
    pub fn crypto_register_acomps(algs: *mut acomp_alg, count: c_int) -> c_int;
    pub fn crypto_unregister_acomps(algs: *mut acomp_alg, count: c_int);
    pub fn crypto_acomp_free_streams(s: *mut crypto_acomp_streams);
    pub fn crypto_acomp_alloc_streams(s: *mut crypto_acomp_streams) -> c_int;
    pub fn _crypto_acomp_lock_stream_bh(s: *mut crypto_acomp_streams) -> *mut crypto_acomp_stream;
    pub fn acomp_walk_done_src(walk: *mut acomp_walk, used: c_int);
    pub fn acomp_walk_done_dst(walk: *mut acomp_walk, used: c_int);
    pub fn acomp_walk_next_src(walk: *mut acomp_walk) -> c_int;
    pub fn acomp_walk_next_dst(walk: *mut acomp_walk) -> c_int;
    pub fn acomp_walk_virt(walk: *mut acomp_walk, req: *mut acomp_req, atomic: bool) -> c_int;
}

#[inline]
pub unsafe fn acomp_request_issg(req: *mut acomp_req) -> bool {
    !((*req).base.flags & (CRYPTO_ACOMP_REQ_SRC_VIRT | CRYPTO_ACOMP_REQ_DST_VIRT) != 0)
}

#[inline]
pub unsafe fn acomp_request_src_isvirt(req: *mut acomp_req) -> bool {
    (*req).base.flags & CRYPTO_ACOMP_REQ_SRC_VIRT != 0
}

#[inline]
pub unsafe fn acomp_request_dst_isvirt(req: *mut acomp_req) -> bool {
    (*req).base.flags & CRYPTO_ACOMP_REQ_DST_VIRT != 0
}

#[inline]
pub unsafe fn acomp_request_isvirt(req: *mut acomp_req) -> bool {
    (*req).base.flags & (CRYPTO_ACOMP_REQ_SRC_VIRT | CRYPTO_ACOMP_REQ_DST_VIRT) != 0
}

#[inline]
pub unsafe fn acomp_request_src_isnondma(req: *mut acomp_req) -> bool {
    (*req).base.flags & CRYPTO_ACOMP_REQ_SRC_NONDMA != 0
}

#[inline]
pub unsafe fn acomp_request_dst_isnondma(req: *mut acomp_req) -> bool {
    (*req).base.flags & CRYPTO_ACOMP_REQ_DST_NONDMA != 0
}

#[inline]
pub unsafe fn acomp_request_isnondma(req: *mut acomp_req) -> bool {
    (*req).base.flags & (CRYPTO_ACOMP_REQ_SRC_NONDMA | CRYPTO_ACOMP_REQ_DST_NONDMA) != 0
}

#[inline]
pub unsafe fn crypto_acomp_req_virt(tfm: *mut crypto_acomp) -> bool {
    crypto_tfm_req_virt(&mut (*tfm).base)
}

#[inline]
pub unsafe fn crypto_acomp_unlock_stream_bh(stream: *mut crypto_acomp_stream) {
    spin_unlock_bh(&mut (*stream).lock);
}

#[inline]
pub unsafe fn acomp_walk_more_src(walk: *const acomp_walk, cur: c_int) -> bool {
    (*walk).slen != cur as u32
}

#[inline]
pub unsafe fn acomp_request_flags(req: *mut acomp_req) -> u32 {
    crypto_request_flags(&(*req).base) & !CRYPTO_ACOMP_REQ_PRIVATE
}

#[inline]
pub unsafe fn crypto_acomp_fb(tfm: *mut crypto_acomp) -> *mut crypto_acomp {
    __crypto_acomp_tfm(crypto_acomp_tfm(tfm).fb)
}

#[inline]
pub unsafe fn acomp_fbreq_on_stack_init(buf: *mut c_char, old: *mut acomp_req) -> *mut acomp_req {
    let tfm = crypto_acomp_reqtfm(old);
    let req = buf as *mut acomp_req;
    crypto_stack_request_init(&mut (*req).base, crypto_acomp_tfm(crypto_acomp_fb(tfm)));
    acomp_request_set_callback(req, acomp_request_flags(old), None, core::ptr::null_mut());
    (*req).base.flags &= !CRYPTO_ACOMP_REQ_PRIVATE;
    (*req).base.flags |= (*old).base.flags & CRYPTO_ACOMP_REQ_PRIVATE;
    (*req).src = (*old).src;
    (*req).dst = (*old).dst;
    (*req).slen = (*old).slen;
    (*req).dlen = (*old).dlen;
    req
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
