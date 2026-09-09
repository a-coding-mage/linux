/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Public Key Encryption
 *
 * Copyright (c) 2015, Intel Corporation
 * Authors: Tadeusz Struk <tadeusz.struk@intel.com>
 */

use core::ffi::c_char;

#[repr(C)]
pub struct akcipher_instance {
    pub free: Option<unsafe extern "C" fn(inst: *mut akcipher_instance)>,
    pub u: akcipher_instance_union,
}

#[repr(C)]
pub union akcipher_instance_union {
    pub s: akcipher_instance_s,
    pub alg: akcipher_alg,
}

#[repr(C)]
pub struct akcipher_instance_s {
    /* C: char head[offsetof(struct akcipher_alg, base)]; */
    pub head: [c_char; 0],
    pub base: crypto_instance,
}

#[repr(C)]
pub struct crypto_akcipher_spawn {
    pub base: crypto_spawn,
}

pub unsafe fn akcipher_request_ctx(req: *mut akcipher_request) -> *mut core::ffi::c_void {
    (*req).__ctx
}

pub unsafe fn akcipher_request_ctx_dma(req: *mut akcipher_request) -> *mut core::ffi::c_void {
    let mut align = crypto_dma_align();
    if align <= crypto_tfm_ctx_alignment() {
        align = 1;
    }
    ptr_align(akcipher_request_ctx(req), align)
}

pub unsafe fn akcipher_set_reqsize(akcipher: *mut crypto_akcipher, reqsize: u32) {
    (*akcipher).reqsize = reqsize;
}

pub unsafe fn akcipher_set_reqsize_dma(akcipher: *mut crypto_akcipher, mut reqsize: u32) {
    reqsize = reqsize.wrapping_add(crypto_dma_align() & !(crypto_tfm_ctx_alignment().wrapping_sub(1)));
    (*akcipher).reqsize = reqsize;
}

pub unsafe fn akcipher_tfm_ctx(tfm: *mut crypto_akcipher) -> *mut core::ffi::c_void {
    crypto_tfm_ctx(&mut (*tfm).base)
}

pub unsafe fn akcipher_tfm_ctx_dma(tfm: *mut crypto_akcipher) -> *mut core::ffi::c_void {
    crypto_tfm_ctx_dma(&mut (*tfm).base)
}

pub unsafe fn akcipher_request_complete(req: *mut akcipher_request, err: i32) {
    crypto_request_complete(&mut (*req).base, err);
}

pub unsafe fn akcipher_alg_name(tfm: *mut crypto_akcipher) -> *const c_char {
    (*crypto_akcipher_tfm(tfm)).__crt_alg.as_ref().unwrap().__base.cra_name.as_ptr()
}

pub unsafe fn akcipher_crypto_instance(inst: *mut akcipher_instance) -> *mut crypto_instance {
    container_of_alg_base(&mut (*inst).u.alg.base)
}

pub unsafe fn akcipher_instance(inst: *mut crypto_instance) -> *mut akcipher_instance {
    container_of_akcipher_alg(&mut (*inst).alg)
}

pub unsafe fn akcipher_alg_instance(akcipher: *mut crypto_akcipher) -> *mut akcipher_instance {
    akcipher_instance(crypto_tfm_alg_instance(&mut (*akcipher).base))
}

pub unsafe fn akcipher_instance_ctx(inst: *mut akcipher_instance) -> *mut core::ffi::c_void {
    crypto_instance_ctx(akcipher_crypto_instance(inst))
}

extern "C" {
    pub fn crypto_grab_akcipher(
        spawn: *mut crypto_akcipher_spawn,
        inst: *mut crypto_instance,
        name: *const c_char,
        type_: u32,
        mask: u32,
    ) -> i32;

    pub fn crypto_register_akcipher(alg: *mut akcipher_alg) -> i32;
    pub fn crypto_unregister_akcipher(alg: *mut akcipher_alg);
    pub fn akcipher_register_instance(
        tmpl: *mut crypto_template,
        inst: *mut akcipher_instance,
    ) -> i32;
}

pub unsafe fn crypto_spawn_akcipher(spawn: *mut crypto_akcipher_spawn) -> *mut crypto_akcipher {
    crypto_spawn_tfm2(&mut (*spawn).base)
}

pub unsafe fn crypto_drop_akcipher(spawn: *mut crypto_akcipher_spawn) {
    crypto_drop_spawn(&mut (*spawn).base);
}

pub unsafe fn crypto_spawn_akcipher_alg(spawn: *mut crypto_akcipher_spawn) -> *mut akcipher_alg {
    container_of_akcipher_alg_base((*spawn).base.alg)
}

/* External declarations and container_of/align helpers are supplied by the
 * surrounding crypto headers in the complete translation unit. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
