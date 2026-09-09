/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * AEAD: Authenticated Encryption with Associated Data
 *
 * Copyright (c) 2007-2015 Herbert Xu <herbert@gondor.apana.org.au>
 */

// C dependencies: crypto/aead.h, crypto/algapi.h, linux/stddef.h,
// linux/types.h

use core::mem::ManuallyDrop;

pub struct rtattr;

#[repr(C)]
pub struct aead_instance {
    pub free: Option<unsafe extern "C" fn(inst: *mut aead_instance)>,
    pub data: ManuallyDrop<aead_instance_union>,
}

#[repr(C)]
pub union aead_instance_union {
    pub s: ManuallyDrop<aead_instance_s>,
    pub alg: ManuallyDrop<aead_alg>,
}

#[repr(C)]
pub struct aead_instance_s {
    pub head: [core::ffi::c_char; core::mem::offset_of!(aead_alg, base)],
    pub base: crypto_instance,
}

#[repr(C)]
pub struct crypto_aead_spawn {
    pub base: crypto_spawn,
}

#[repr(C)]
pub struct aead_queue {
    pub base: crypto_queue,
}

#[inline]
pub unsafe fn crypto_aead_ctx(tfm: *mut crypto_aead) -> *mut core::ffi::c_void {
    crypto_tfm_ctx(&mut (*tfm).base)
}

#[inline]
pub unsafe fn crypto_aead_ctx_dma(tfm: *mut crypto_aead) -> *mut core::ffi::c_void {
    crypto_tfm_ctx_dma(&mut (*tfm).base)
}

#[inline]
pub unsafe fn aead_crypto_instance(inst: *mut aead_instance) -> *mut crypto_instance {
    container_of(&mut (*(*inst).data).alg.base, crypto_instance, alg)
}

#[inline]
pub unsafe fn aead_instance(inst: *mut crypto_instance) -> *mut aead_instance {
    container_of(&mut (*inst).alg, aead_instance, alg.base)
}

#[inline]
pub unsafe fn aead_alg_instance(aead: *mut crypto_aead) -> *mut aead_instance {
    aead_instance(crypto_tfm_alg_instance(&mut (*aead).base))
}

#[inline]
pub unsafe fn aead_instance_ctx(inst: *mut aead_instance) -> *mut core::ffi::c_void {
    crypto_instance_ctx(aead_crypto_instance(inst))
}

#[inline]
pub unsafe fn aead_request_ctx(req: *mut aead_request) -> *mut core::ffi::c_void {
    (*req).__ctx
}

#[inline]
pub unsafe fn aead_request_ctx_dma(req: *mut aead_request) -> *mut core::ffi::c_void {
    let mut align: u32 = crypto_dma_align();
    if align <= crypto_tfm_ctx_alignment() {
        align = 1;
    }
    ptr_align(aead_request_ctx(req), align)
}

#[inline]
pub unsafe fn aead_request_complete(req: *mut aead_request, err: i32) {
    crypto_request_complete(&mut (*req).base, err);
}

#[inline]
pub unsafe fn aead_request_flags(req: *mut aead_request) -> u32 {
    (*req).base.flags
}

#[inline]
pub unsafe fn aead_request_cast(req: *mut crypto_async_request) -> *mut aead_request {
    container_of(req, aead_request, base)
}

pub unsafe extern "C" fn crypto_grab_aead(
    spawn: *mut crypto_aead_spawn,
    inst: *mut crypto_instance,
    name: *const core::ffi::c_char,
    type_: u32,
    mask: u32,
) -> i32;

#[inline]
pub unsafe fn crypto_drop_aead(spawn: *mut crypto_aead_spawn) {
    crypto_drop_spawn(&mut (*spawn).base);
}

#[inline]
pub unsafe fn crypto_spawn_aead_alg(spawn: *mut crypto_aead_spawn) -> *mut aead_alg {
    container_of((*spawn).base.alg, aead_alg, base)
}

#[inline]
pub unsafe fn crypto_spawn_aead(spawn: *mut crypto_aead_spawn) -> *mut crypto_aead {
    crypto_spawn_tfm2(&mut (*spawn).base)
}

#[inline]
pub unsafe fn crypto_aead_set_reqsize(aead: *mut crypto_aead, reqsize: u32) {
    (*aead).reqsize = reqsize;
}

#[inline]
pub unsafe fn crypto_aead_set_reqsize_dma(aead: *mut crypto_aead, mut reqsize: u32) {
    reqsize += crypto_dma_align() & !(crypto_tfm_ctx_alignment() - 1);
    (*aead).reqsize = reqsize;
}

#[inline]
pub unsafe fn aead_init_queue(queue: *mut aead_queue, max_qlen: u32) {
    crypto_init_queue(&mut (*queue).base, max_qlen);
}

#[inline]
pub unsafe fn crypto_aead_alg_chunksize(alg: *mut aead_alg) -> u32 {
    (*alg).chunksize
}

/**
 * crypto_aead_chunksize() - obtain chunk size
 * @tfm: cipher handle
 *
 * The block size is set to one for ciphers such as CCM.  However,
 * you still need to provide incremental updates in multiples of
 * the underlying block size as the IV does not have sub-block
 * granularity.  This is known in this API as the chunk size.
 *
 * Return: chunk size in bytes
 */
#[inline]
pub unsafe fn crypto_aead_chunksize(tfm: *mut crypto_aead) -> u32 {
    crypto_aead_alg_chunksize(crypto_aead_alg(tfm))
}

pub unsafe extern "C" fn crypto_register_aead(alg: *mut aead_alg) -> i32;
pub unsafe extern "C" fn crypto_unregister_aead(alg: *mut aead_alg);
pub unsafe extern "C" fn crypto_register_aeads(algs: *mut aead_alg, count: i32) -> i32;
pub unsafe extern "C" fn crypto_unregister_aeads(algs: *mut aead_alg, count: i32);
pub unsafe extern "C" fn aead_register_instance(
    tmpl: *mut crypto_template,
    inst: *mut aead_instance,
) -> i32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
