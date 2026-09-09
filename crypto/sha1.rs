// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Crypto API support for SHA-1 and HMAC-SHA1
 *
 * Copyright (c) Alan Smithee.
 * Copyright (c) Andrew McDonald <andrew@mcdonald.org.uk>
 * Copyright (c) Jean-Francois Dive <jef@linuxbe.org>
 * Copyright 2025 Google LLC
 */

use core::ffi::c_void;

/* The following types and functions are supplied by the surrounding kernel. */
extern "C" {
    fn sha1_init(ctx: *mut sha1_ctx);
    fn sha1_update(ctx: *mut sha1_ctx, data: *const u8, len: u32);
    fn sha1_final(ctx: *mut sha1_ctx, out: *mut u8);
    fn sha1(data: *const u8, len: u32, out: *mut u8);
    fn hmac_sha1_preparekey(key: *mut hmac_sha1_key, raw_key: *const u8, keylen: u32);
    fn hmac_sha1_init(ctx: *mut hmac_sha1_ctx, key: *const hmac_sha1_key);
    fn hmac_sha1_update(ctx: *mut hmac_sha1_ctx, data: *const u8, len: u32);
    fn hmac_sha1_final(ctx: *mut hmac_sha1_ctx, out: *mut u8);
    fn hmac_sha1(key: *const hmac_sha1_key, data: *const u8, len: u32, out: *mut u8);
    fn crypto_register_shashes(algs: *mut shash_alg, count: usize) -> i32;
    fn crypto_unregister_shashes(algs: *mut shash_alg, count: usize);
}

#[repr(C)]
pub struct sha1_ctx {
    pub state: [u32; 5],
    pub bytecount: u64,
    pub buf: [u8; SHA1_BLOCK_SIZE],
}

#[repr(C)]
pub struct hmac_sha1_key { pub ostate: sha1_ctx }
#[repr(C)]
pub struct hmac_sha1_ctx { pub sha_ctx: sha1_ctx, pub ostate: sha1_ctx }
#[repr(C)]
pub struct shash_desc { pub tfm: *mut crypto_shash }
#[repr(C)]
pub struct crypto_shash { pub ctx: *mut c_void }
#[repr(C)]
pub struct shash_alg { _private: [u8; 0] }

pub const SHA1_BLOCK_SIZE: usize = 64;
pub const SHA1_DIGEST_SIZE: usize = 20;
pub const SHA1_SHASH_STATE_SIZE: usize = core::mem::size_of::<sha1_ctx>() + 1;

unsafe fn crypto_sha1_ctx(desc: *mut shash_desc) -> *mut sha1_ctx {
    desc as *mut sha1_ctx
}
unsafe fn hmac_sha1_key(desc: *mut crypto_shash) -> *mut hmac_sha1_key {
    (*desc).ctx as *mut hmac_sha1_key
}
unsafe fn hmac_sha1_ctx(desc: *mut shash_desc) -> *mut hmac_sha1_ctx {
    desc as *mut hmac_sha1_ctx
}

unsafe fn __crypto_sha1_export(ctx0: *const sha1_ctx, out: *mut c_void) -> i32 {
    let mut ctx = *ctx0;
    let partial = (ctx.bytecount % SHA1_BLOCK_SIZE as u64) as u8;
    ctx.bytecount -= partial as u64;
    core::ptr::copy_nonoverlapping(&ctx as *const _ as *const u8, out as *mut u8, core::mem::size_of::<sha1_ctx>());
    *(out as *mut u8).add(core::mem::size_of::<sha1_ctx>()) = partial;
    0
}

unsafe fn __crypto_sha1_import(ctx: *mut sha1_ctx, input: *const c_void) -> i32 {
    core::ptr::copy_nonoverlapping(input as *const u8, ctx as *mut u8, core::mem::size_of::<sha1_ctx>());
    (*ctx).bytecount += *(input as *const u8).add(core::mem::size_of::<sha1_ctx>()) as u64;
    0
}

unsafe fn __crypto_sha1_export_core(ctx: *const sha1_ctx, out: *mut c_void) -> i32 {
    core::ptr::copy_nonoverlapping(ctx as *const u8, out as *mut u8, core::mem::offset_of!(sha1_ctx, buf)); 0
}
unsafe fn __crypto_sha1_import_core(ctx: *mut sha1_ctx, input: *const c_void) -> i32 {
    core::ptr::copy_nonoverlapping(input as *const u8, ctx as *mut u8, core::mem::offset_of!(sha1_ctx, buf)); 0
}

#[no_mangle]
pub static sha1_zero_message_hash: [u8; SHA1_DIGEST_SIZE] = [
    0xda, 0x39, 0xa3, 0xee, 0x5e, 0x6b, 0x4b, 0x0d,
    0x32, 0x55, 0xbf, 0xef, 0x95, 0x60, 0x18, 0x90,
    0xaf, 0xd8, 0x07, 0x09,
];

unsafe fn crypto_sha1_init(desc: *mut shash_desc) -> i32 { sha1_init(crypto_sha1_ctx(desc)); 0 }
unsafe fn crypto_sha1_update(desc: *mut shash_desc, data: *const u8, len: u32) -> i32 { sha1_update(crypto_sha1_ctx(desc), data, len); 0 }
unsafe fn crypto_sha1_final(desc: *mut shash_desc, out: *mut u8) -> i32 { sha1_final(crypto_sha1_ctx(desc), out); 0 }
unsafe fn crypto_sha1_digest(_: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32 { sha1(data, len, out); 0 }
unsafe fn crypto_sha1_export(desc: *mut shash_desc, out: *mut c_void) -> i32 { __crypto_sha1_export(crypto_sha1_ctx(desc), out) }
unsafe fn crypto_sha1_import(desc: *mut shash_desc, input: *const c_void) -> i32 { __crypto_sha1_import(crypto_sha1_ctx(desc), input) }
unsafe fn crypto_sha1_export_core(desc: *mut shash_desc, out: *mut c_void) -> i32 { __crypto_sha1_export_core(crypto_sha1_ctx(desc), out) }
unsafe fn crypto_sha1_import_core(desc: *mut shash_desc, input: *const c_void) -> i32 { __crypto_sha1_import_core(crypto_sha1_ctx(desc), input) }

unsafe fn crypto_hmac_sha1_setkey(tfm: *mut crypto_shash, key: *const u8, len: u32) -> i32 { hmac_sha1_preparekey(hmac_sha1_key(tfm), key, len); 0 }
unsafe fn crypto_hmac_sha1_init(desc: *mut shash_desc) -> i32 { hmac_sha1_init(hmac_sha1_ctx(desc), hmac_sha1_key((*desc).tfm)); 0 }
unsafe fn crypto_hmac_sha1_update(desc: *mut shash_desc, data: *const u8, len: u32) -> i32 { hmac_sha1_update(hmac_sha1_ctx(desc), data, len); 0 }
unsafe fn crypto_hmac_sha1_final(desc: *mut shash_desc, out: *mut u8) -> i32 { hmac_sha1_final(hmac_sha1_ctx(desc), out); 0 }
unsafe fn crypto_hmac_sha1_digest(desc: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32 { hmac_sha1(hmac_sha1_key((*desc).tfm), data, len, out); 0 }
unsafe fn crypto_hmac_sha1_export(desc: *mut shash_desc, out: *mut c_void) -> i32 { __crypto_sha1_export(&(*hmac_sha1_ctx(desc)).sha_ctx, out) }
unsafe fn crypto_hmac_sha1_import(desc: *mut shash_desc, input: *const c_void) -> i32 { let ctx = hmac_sha1_ctx(desc); (*ctx).ostate = (*hmac_sha1_key((*desc).tfm)).ostate; __crypto_sha1_import(&mut (*ctx).sha_ctx, input) }
unsafe fn crypto_hmac_sha1_export_core(desc: *mut shash_desc, out: *mut c_void) -> i32 { __crypto_sha1_export_core(&(*hmac_sha1_ctx(desc)).sha_ctx, out) }
unsafe fn crypto_hmac_sha1_import_core(desc: *mut shash_desc, input: *const c_void) -> i32 { let ctx = hmac_sha1_ctx(desc); (*ctx).ostate = (*hmac_sha1_key((*desc).tfm)).ostate; __crypto_sha1_import_core(&mut (*ctx).sha_ctx, input) }

// The algorithm table, module registration, and metadata are provided by the kernel ABI.
extern "C" {
    static mut algs: [shash_alg; 2];
}

unsafe fn crypto_sha1_mod_init() -> i32 { crypto_register_shashes(algs.as_mut_ptr(), 2) }
unsafe fn crypto_sha1_mod_exit() { crypto_unregister_shashes(algs.as_mut_ptr(), 2); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
