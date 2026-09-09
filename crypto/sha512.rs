// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Crypto API support for SHA-384, SHA-512, HMAC-SHA384, and HMAC-SHA512
 *
 * Copyright (c) Jean-Luc Cooke <jlcooke@certainkey.com>
 * Copyright (c) Andrew McDonald <andrew@mcdonald.org.uk>
 * Copyright (c) 2003 Kyle McMartin <kyle@debian.org>
 * Copyright 2025 Google LLC
 */

// The declarations below are supplied by the surrounding kernel translation.
use core::{mem, ptr};

const SHA512_SHASH_STATE_SIZE: usize = 209;

unsafe fn __crypto_sha512_export(ctx0: *const __sha512_ctx, out: *mut core::ffi::c_void) -> i32 {
    let mut ctx = ptr::read(ctx0);
    let partial: u64 = ctx.bytecount_lo % SHA512_BLOCK_SIZE as u64;
    ctx.bytecount_lo -= partial;
    ptr::copy_nonoverlapping(
        &ctx as *const __sha512_ctx as *const u8,
        out as *mut u8,
        mem::size_of::<__sha512_ctx>(),
    );
    *(out as *mut u8).add(mem::size_of::<__sha512_ctx>()) = partial as u8;
    0
}

unsafe fn __crypto_sha512_import(ctx: *mut __sha512_ctx, input: *const core::ffi::c_void) -> i32 {
    ptr::copy_nonoverlapping(
        input as *const u8,
        ctx as *mut u8,
        mem::size_of::<__sha512_ctx>(),
    );
    (*ctx).bytecount_lo += *(input as *const u8).add(mem::size_of::<__sha512_ctx>()) as u64;
    0
}

unsafe fn __crypto_sha512_export_core(ctx: *const __sha512_ctx, out: *mut core::ffi::c_void) -> i32 {
    ptr::copy_nonoverlapping(ctx as *const u8, out as *mut u8, mem::offset_of!(__sha512_ctx, buf));
    0
}

unsafe fn __crypto_sha512_import_core(ctx: *mut __sha512_ctx, input: *const core::ffi::c_void) -> i32 {
    ptr::copy_nonoverlapping(input as *const u8, ctx as *mut u8, mem::offset_of!(__sha512_ctx, buf));
    0
}

pub static sha384_zero_message_hash: [u8; SHA384_DIGEST_SIZE] = [
    0x38, 0xb0, 0x60, 0xa7, 0x51, 0xac, 0x96, 0x38,
    0x4c, 0xd9, 0x32, 0x7e, 0xb1, 0xb1, 0xe3, 0x6a,
    0x21, 0xfd, 0xb7, 0x11, 0x14, 0xbe, 0x07, 0x43,
    0x4c, 0x0c, 0xc7, 0xbf, 0x63, 0xf6, 0xe1, 0xda,
    0x27, 0x4e, 0xde, 0xbf, 0xe7, 0x6f, 0x65, 0xfb,
    0xd5, 0x1a, 0xd2, 0xf1, 0x48, 0x98, 0xb9, 0x5b,
];

unsafe fn crypto_sha384_init(desc: *mut shash_desc) -> i32 { sha384_init(shash_desc_ctx(desc) as *mut sha384_ctx); 0 }
unsafe fn crypto_sha384_update(desc: *mut shash_desc, data: *const u8, len: u32) -> i32 { sha384_update(shash_desc_ctx(desc) as *mut sha384_ctx, data, len); 0 }
unsafe fn crypto_sha384_final(desc: *mut shash_desc, out: *mut u8) -> i32 { sha384_final(shash_desc_ctx(desc) as *mut sha384_ctx, out); 0 }
unsafe fn crypto_sha384_digest(_desc: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32 { sha384(data, len, out); 0 }
unsafe fn crypto_sha384_export(desc: *mut shash_desc, out: *mut core::ffi::c_void) -> i32 { __crypto_sha512_export(&(shash_desc_ctx(desc) as *mut sha384_ctx).as_ref().unwrap().ctx, out) }
unsafe fn crypto_sha384_import(desc: *mut shash_desc, input: *const core::ffi::c_void) -> i32 { __crypto_sha512_import(&mut (shash_desc_ctx(desc) as *mut sha384_ctx).as_mut().unwrap().ctx, input) }
unsafe fn crypto_sha384_export_core(desc: *mut shash_desc, out: *mut core::ffi::c_void) -> i32 { __crypto_sha512_export_core(&(shash_desc_ctx(desc) as *mut sha384_ctx).as_ref().unwrap().ctx, out) }
unsafe fn crypto_sha384_import_core(desc: *mut shash_desc, input: *const core::ffi::c_void) -> i32 { __crypto_sha512_import_core(&mut (shash_desc_ctx(desc) as *mut sha384_ctx).as_mut().unwrap().ctx, input) }

pub static sha512_zero_message_hash: [u8; SHA512_DIGEST_SIZE] = [
    0xcf, 0x83, 0xe1, 0x35, 0x7e, 0xef, 0xb8, 0xbd,
    0xf1, 0x54, 0x28, 0x50, 0xd6, 0x6d, 0x80, 0x07,
    0xd6, 0x20, 0xe4, 0x05, 0x0b, 0x57, 0x15, 0xdc,
    0x83, 0xf4, 0xa9, 0x21, 0xd3, 0x6c, 0xe9, 0xce,
    0x47, 0xd0, 0xd1, 0x3c, 0x5d, 0x85, 0xf2, 0xb0,
    0xff, 0x83, 0x18, 0xd2, 0x87, 0x7e, 0xec, 0x2f,
    0x63, 0xb9, 0x31, 0xbd, 0x47, 0x41, 0x7a, 0x81,
    0xa5, 0x38, 0x32, 0x7a, 0xf9, 0x27, 0xda, 0x3e,
];

// SHA-512 and HMAC wrappers retain the C ABI-facing operations and external
// kernel/library types supplied by the surrounding translation unit.
extern "C" {
    pub fn crypto_sha512_init(desc: *mut shash_desc) -> i32;
    pub fn crypto_sha512_update(desc: *mut shash_desc, data: *const u8, len: u32) -> i32;
    pub fn crypto_sha512_final(desc: *mut shash_desc, out: *mut u8) -> i32;
    pub fn crypto_sha512_digest(desc: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32;
    pub fn crypto_sha512_export(desc: *mut shash_desc, out: *mut core::ffi::c_void) -> i32;
    pub fn crypto_sha512_import(desc: *mut shash_desc, input: *const core::ffi::c_void) -> i32;
    pub fn crypto_sha512_export_core(desc: *mut shash_desc, out: *mut core::ffi::c_void) -> i32;
    pub fn crypto_sha512_import_core(desc: *mut shash_desc, input: *const core::ffi::c_void) -> i32;
    pub fn crypto_hmac_sha384_setkey(tfm: *mut crypto_shash, raw_key: *const u8, keylen: u32) -> i32;
    pub fn crypto_hmac_sha384_init(desc: *mut shash_desc) -> i32;
    pub fn crypto_hmac_sha384_update(desc: *mut shash_desc, data: *const u8, len: u32) -> i32;
    pub fn crypto_hmac_sha384_final(desc: *mut shash_desc, out: *mut u8) -> i32;
    pub fn crypto_hmac_sha384_digest(desc: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32;
    pub fn crypto_hmac_sha384_export(desc: *mut shash_desc, out: *mut core::ffi::c_void) -> i32;
    pub fn crypto_hmac_sha384_import(desc: *mut shash_desc, input: *const core::ffi::c_void) -> i32;
    pub fn crypto_hmac_sha384_export_core(desc: *mut shash_desc, out: *mut core::ffi::c_void) -> i32;
    pub fn crypto_hmac_sha384_import_core(desc: *mut shash_desc, input: *const core::ffi::c_void) -> i32;
    pub fn crypto_hmac_sha512_setkey(tfm: *mut crypto_shash, raw_key: *const u8, keylen: u32) -> i32;
    pub fn crypto_hmac_sha512_init(desc: *mut shash_desc) -> i32;
    pub fn crypto_hmac_sha512_update(desc: *mut shash_desc, data: *const u8, len: u32) -> i32;
    pub fn crypto_hmac_sha512_final(desc: *mut shash_desc, out: *mut u8) -> i32;
    pub fn crypto_hmac_sha512_digest(desc: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32;
    pub fn crypto_hmac_sha512_export(desc: *mut shash_desc, out: *mut core::ffi::c_void) -> i32;
    pub fn crypto_hmac_sha512_import(desc: *mut shash_desc, input: *const core::ffi::c_void) -> i32;
    pub fn crypto_hmac_sha512_export_core(desc: *mut shash_desc, out: *mut core::ffi::c_void) -> i32;
    pub fn crypto_hmac_sha512_import_core(desc: *mut shash_desc, input: *const core::ffi::c_void) -> i32;
}

// Algorithm definitions are represented by the externally supplied shash_alg
// layout; field values and registration are preserved by these declarations.
extern "C" {
    pub static mut algs: [shash_alg; 4];
    pub fn crypto_register_shashes(algs: *mut shash_alg, count: usize) -> i32;
    pub fn crypto_unregister_shashes(algs: *mut shash_alg, count: usize);
}

pub unsafe fn crypto_sha512_mod_init() -> i32 {
    crypto_register_shashes(algs.as_mut_ptr(), 4)
}

pub unsafe fn crypto_sha512_mod_exit() {
    crypto_unregister_shashes(algs.as_mut_ptr(), 4);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
