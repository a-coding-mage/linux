/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Key-agreement Protocol Primitives (KPP) */

use core::ffi::{c_char, c_int, c_void};

/* Types and helpers supplied by <crypto/kpp.h> and <crypto/algapi.h>. */
#[repr(C)]
pub struct kpp_alg {
    pub base: crypto_alg,
}
#[repr(C)] pub struct crypto_alg { pub cra_name: *const c_char }
#[repr(C)] pub struct crypto_instance { pub _private: [u8; 0] }
#[repr(C)] pub struct crypto_spawn { pub alg: *mut crypto_alg }
#[repr(C)] pub struct crypto_tfm { pub _private: [u8; 0] }
#[repr(C)] pub struct crypto_kpp { pub base: crypto_tfm, pub reqsize: u32 }
#[repr(C)] pub struct kpp_request { pub base: crypto_async_request, pub __ctx: *mut c_void }
#[repr(C)] pub struct crypto_async_request { pub _private: [u8; 0] }
#[repr(C)] pub struct crypto_template { pub _private: [u8; 0] }

#[repr(C)]
pub struct kpp_instance {
    pub free: Option<unsafe extern "C" fn(inst: *mut kpp_instance)>,
    pub s: kpp_instance_s,
}

#[repr(C)]
pub union kpp_instance_s {
    pub s: kpp_instance_state,
    pub alg: kpp_alg,
}

#[repr(C)]
pub struct kpp_instance_state {
    /* C uses char head[offsetof(struct kpp_alg, base)] here. */
    pub head: [u8; 0],
    pub base: crypto_instance,
}

#[repr(C)]
pub struct crypto_kpp_spawn {
    pub base: crypto_spawn,
}

#[inline]
pub unsafe fn kpp_request_ctx(req: *mut kpp_request) -> *mut c_void {
    (*req).__ctx
}

#[inline]
pub unsafe fn kpp_request_ctx_dma(req: *mut kpp_request) -> *mut c_void {
    let mut align = crypto_dma_align();
    if align <= crypto_tfm_ctx_alignment() { align = 1; }
    let ptr = kpp_request_ctx(req) as usize;
    ((ptr + align as usize - 1) & !(align as usize - 1)) as *mut c_void
}

#[inline]
pub unsafe fn kpp_set_reqsize(kpp: *mut crypto_kpp, reqsize: u32) {
    (*kpp).reqsize = reqsize;
}

#[inline]
pub unsafe fn kpp_set_reqsize_dma(kpp: *mut crypto_kpp, mut reqsize: u32) {
    reqsize += crypto_dma_align() & !(crypto_tfm_ctx_alignment() - 1);
    (*kpp).reqsize = reqsize;
}

#[inline]
pub unsafe fn kpp_tfm_ctx(tfm: *mut crypto_kpp) -> *mut c_void {
    crypto_tfm_ctx(&mut (*tfm).base)
}

#[inline]
pub unsafe fn kpp_tfm_ctx_dma(tfm: *mut crypto_kpp) -> *mut c_void {
    crypto_tfm_ctx_dma(&mut (*tfm).base)
}

#[inline]
pub unsafe fn kpp_request_complete(req: *mut kpp_request, err: c_int) {
    crypto_request_complete(&mut (*req).base, err);
}

#[inline]
pub unsafe fn kpp_alg_name(tfm: *mut crypto_kpp) -> *const c_char {
    (*crypto_kpp_tfm(tfm)).__crt_alg.cra_name
}

#[inline]
pub unsafe fn kpp_crypto_instance(inst: *mut kpp_instance) -> *mut crypto_instance {
    &mut (*inst).s.s.base
}

#[inline]
pub unsafe fn kpp_instance(inst: *mut crypto_instance) -> *mut kpp_instance {
    inst as *mut kpp_instance
}

#[inline]
pub unsafe fn kpp_alg_instance(kpp: *mut crypto_kpp) -> *mut kpp_instance {
    kpp_instance(crypto_tfm_alg_instance(&mut (*kpp).base))
}

#[inline]
pub unsafe fn kpp_instance_ctx(inst: *mut kpp_instance) -> *mut c_void {
    crypto_instance_ctx(kpp_crypto_instance(inst))
}

extern "C" {
    pub fn crypto_register_kpp(alg: *mut kpp_alg) -> c_int;
    pub fn crypto_unregister_kpp(alg: *mut kpp_alg);
    pub fn kpp_register_instance(tmpl: *mut crypto_template, inst: *mut kpp_instance) -> c_int;
    pub fn crypto_grab_kpp(spawn: *mut crypto_kpp_spawn, inst: *mut crypto_instance,
                           name: *const c_char, type_: u32, mask: u32) -> c_int;
    pub fn crypto_drop_spawn(spawn: *mut crypto_spawn);
    pub fn crypto_spawn_tfm2(spawn: *mut crypto_spawn) -> *mut crypto_kpp;
    pub fn crypto_dma_align() -> u32;
    pub fn crypto_tfm_ctx_alignment() -> u32;
    pub fn crypto_tfm_ctx(tfm: *mut crypto_tfm) -> *mut c_void;
    pub fn crypto_tfm_ctx_dma(tfm: *mut crypto_tfm) -> *mut c_void;
    pub fn crypto_request_complete(req: *mut crypto_async_request, err: c_int);
    pub fn crypto_kpp_tfm(tfm: *mut crypto_kpp) -> *mut crypto_kpp;
    pub fn crypto_tfm_alg_instance(tfm: *mut crypto_tfm) -> *mut crypto_instance;
    pub fn crypto_instance_ctx(inst: *mut crypto_instance) -> *mut c_void;
}

#[inline]
pub unsafe fn crypto_drop_kpp(spawn: *mut crypto_kpp_spawn) {
    crypto_drop_spawn(&mut (*spawn).base);
}

#[inline]
pub unsafe fn crypto_spawn_kpp_alg(spawn: *mut crypto_kpp_spawn) -> *mut kpp_alg {
    (*spawn).base.alg as *mut kpp_alg
}

#[inline]
pub unsafe fn crypto_spawn_kpp(spawn: *mut crypto_kpp_spawn) -> *mut crypto_kpp {
    crypto_spawn_tfm2(&mut (*spawn).base)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
