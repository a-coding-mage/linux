// SPDX-License-Identifier: GPL-2.0-only
/*
 * SM3 secure hash, as specified by OSCCA GM/T 0004-2012 SM3 and
 * described at https://tools.ietf.org/html/draft-shen-sm3-hash-01
 *
 * Copyright (C) 2017 ARM Limited or its affiliates.
 * Written by Gilad Ben-Yossef <gilad@benyossef.com>
 * Copyright (C) 2021 Tianjia Zhang <tianjia.zhang@linux.alibaba.com>
 * Copyright 2026 Google LLC
 */

// Dependencies supplied by the surrounding kernel and crypto implementation.
use core::ffi::c_void;

type U8 = u8;

#[repr(C)]
pub struct shash_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sm3_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct shash_alg {
    _private: [u8; 0],
}

extern "C" {
    fn shash_desc_ctx(desc: *mut shash_desc) -> *mut c_void;
    fn sm3_init(ctx: *mut sm3_ctx);
    fn sm3_update(ctx: *mut sm3_ctx, data: *const U8, len: u32);
    fn sm3_final(ctx: *mut sm3_ctx, out: *mut U8);
    fn sm3(data: *const U8, len: u32, out: *mut U8);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn crypto_register_shash(alg: *mut shash_alg) -> i32;
    fn crypto_unregister_shash(alg: *mut shash_alg);
}

// SM3_CTX(desc) == ((struct sm3_ctx *)shash_desc_ctx(desc))
#[inline]
unsafe fn sm3_ctx_from_desc(desc: *mut shash_desc) -> *mut sm3_ctx {
    shash_desc_ctx(desc) as *mut sm3_ctx
}

unsafe fn crypto_sm3_init(desc: *mut shash_desc) -> i32 {
    sm3_init(sm3_ctx_from_desc(desc));
    0
}

unsafe fn crypto_sm3_update(desc: *mut shash_desc, data: *const U8, len: u32) -> i32 {
    sm3_update(sm3_ctx_from_desc(desc), data, len);
    0
}

unsafe fn crypto_sm3_final(desc: *mut shash_desc, out: *mut U8) -> i32 {
    sm3_final(sm3_ctx_from_desc(desc), out);
    0
}

unsafe fn crypto_sm3_digest(
    _desc: *mut shash_desc,
    data: *const U8,
    len: u32,
    out: *mut U8,
) -> i32 {
    sm3(data, len, out);
    0
}

unsafe fn crypto_sm3_export_core(desc: *mut shash_desc, out: *mut c_void) -> i32 {
    memcpy(
        out,
        sm3_ctx_from_desc(desc) as *const c_void,
        core::mem::size_of::<sm3_ctx>(),
    );
    0
}

unsafe fn crypto_sm3_import_core(desc: *mut shash_desc, input: *const c_void) -> i32 {
    memcpy(
        sm3_ctx_from_desc(desc) as *mut c_void,
        input,
        core::mem::size_of::<sm3_ctx>(),
    );
    0
}

// The initializer preserves the C shash_alg aggregate and its externally
// supplied field layout; the concrete definition is provided by the kernel.
static mut sm3_alg: shash_alg = shash_alg { _private: [] };

unsafe fn crypto_sm3_mod_init() -> i32 {
    crypto_register_shash(&mut sm3_alg)
}

unsafe fn crypto_sm3_mod_exit() {
    crypto_unregister_shash(&mut sm3_alg);
}

// module_init(crypto_sm3_mod_init);
// module_exit(crypto_sm3_mod_exit);
// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("Crypto API support for SM3");
// MODULE_ALIAS_CRYPTO("sm3");
// MODULE_ALIAS_CRYPTO("sm3-lib");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
