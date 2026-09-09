// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Crypto API support for MD5 and HMAC-MD5
 *
 * Copyright 2025 Google LLC
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

const MD5_SHASH_STATE_SIZE: usize = core::mem::size_of::<md5_ctx>() + 1;

extern "C" {
    type md5_ctx;
    type md5_state;
    type hmac_md5_key;
    type hmac_md5_ctx;
    type shash_desc;
    type crypto_shash;
    type shash_alg;

    fn md5_init(ctx: *mut md5_ctx);
    fn md5_update(ctx: *mut md5_ctx, data: *const u8, len: u32);
    fn md5_final(ctx: *mut md5_ctx, out: *mut u8);
    fn md5(data: *const u8, len: u32, out: *mut u8);
    fn hmac_md5_preparekey(key: *mut hmac_md5_key, raw_key: *const u8, keylen: u32);
    fn hmac_md5_init(ctx: *mut hmac_md5_ctx, key: *const hmac_md5_key);
    fn hmac_md5_update(ctx: *mut hmac_md5_ctx, data: *const u8, len: u32);
    fn hmac_md5_final(ctx: *mut hmac_md5_ctx, out: *mut u8);
    fn hmac_md5(key: *const hmac_md5_key, data: *const u8, len: u32, out: *mut u8);
    fn crypto_register_shashes(algs: *mut shash_alg, count: usize) -> i32;
    fn crypto_unregister_shashes(algs: *mut shash_alg, count: usize);
}

const MD5_BLOCK_SIZE: u32 = 64;
const MD5_DIGEST_SIZE: usize = 16;

#[no_mangle]
pub static md5_zero_message_hash: [u8; MD5_DIGEST_SIZE] = [
    0xd4, 0x1d, 0x8c, 0xd9, 0x8f, 0x00, 0xb2, 0x04,
    0xe9, 0x80, 0x09, 0x98, 0xec, 0xf8, 0x42, 0x7e,
];

unsafe fn __crypto_md5_export(ctx0: *const md5_ctx, out: *mut c_void) -> i32 {
    let mut ctx = core::ptr::read(ctx0);
    let bytecount = (&ctx as *const md5_ctx as *const u8).add(16) as *const u64;
    let partial = (*bytecount % MD5_BLOCK_SIZE as u64) as u32;
    *(bytecount as *mut u64) -= partial as u64;
    core::ptr::copy_nonoverlapping(
        &ctx as *const md5_ctx as *const u8,
        out as *mut u8,
        core::mem::size_of::<md5_ctx>(),
    );
    *((out as *mut u8).add(core::mem::size_of::<md5_ctx>())) = partial as u8;
    0
}

unsafe fn __crypto_md5_import(ctx: *mut md5_ctx, input: *const c_void) -> i32 {
    core::ptr::copy_nonoverlapping(
        input as *const u8,
        ctx as *mut u8,
        core::mem::size_of::<md5_ctx>(),
    );
    let p = (input as *const u8).add(core::mem::size_of::<md5_ctx>());
    let bytecount = (ctx as *mut u8).add(16) as *mut u64;
    *bytecount += *p as u64;
    0
}

unsafe fn __crypto_md5_export_core(ctx: *const md5_ctx, out: *mut c_void) -> i32 {
    core::ptr::copy_nonoverlapping(
        ctx as *const u8,
        out as *mut u8,
        core::mem::offset_of!(md5_ctx, buf),
    );
    0
}

unsafe fn __crypto_md5_import_core(ctx: *mut md5_ctx, input: *const c_void) -> i32 {
    core::ptr::copy_nonoverlapping(
        input as *const u8,
        ctx as *mut u8,
        core::mem::offset_of!(md5_ctx, buf),
    );
    0
}

unsafe fn crypto_md5_init(desc: *mut shash_desc) -> i32 {
    md5_init(desc as *mut md5_ctx);
    0
}

unsafe fn crypto_md5_update(desc: *mut shash_desc, data: *const u8, len: u32) -> i32 {
    md5_update(desc as *mut md5_ctx, data, len);
    0
}

unsafe fn crypto_md5_final(desc: *mut shash_desc, out: *mut u8) -> i32 {
    md5_final(desc as *mut md5_ctx, out);
    0
}

unsafe fn crypto_md5_digest(desc: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32 {
    md5(data, len, out);
    0
}

unsafe fn crypto_md5_export(desc: *mut shash_desc, out: *mut c_void) -> i32 {
    __crypto_md5_export(desc as *const md5_ctx, out)
}

unsafe fn crypto_md5_import(desc: *mut shash_desc, input: *const c_void) -> i32 {
    __crypto_md5_import(desc as *mut md5_ctx, input)
}

unsafe fn crypto_md5_export_core(desc: *mut shash_desc, out: *mut c_void) -> i32 {
    __crypto_md5_export_core(desc as *const md5_ctx, out)
}

unsafe fn crypto_md5_import_core(desc: *mut shash_desc, input: *const c_void) -> i32 {
    __crypto_md5_import_core(desc as *mut md5_ctx, input)
}

unsafe fn crypto_hmac_md5_setkey(tfm: *mut crypto_shash, raw_key: *const u8, keylen: u32) -> i32 {
    hmac_md5_preparekey(tfm as *mut hmac_md5_key, raw_key, keylen);
    0
}

unsafe fn crypto_hmac_md5_init(desc: *mut shash_desc) -> i32 {
    hmac_md5_init(desc as *mut hmac_md5_ctx, desc as *const hmac_md5_key);
    0
}

unsafe fn crypto_hmac_md5_update(desc: *mut shash_desc, data: *const u8, len: u32) -> i32 {
    hmac_md5_update(desc as *mut hmac_md5_ctx, data, len);
    0
}

unsafe fn crypto_hmac_md5_final(desc: *mut shash_desc, out: *mut u8) -> i32 {
    hmac_md5_final(desc as *mut hmac_md5_ctx, out);
    0
}

unsafe fn crypto_hmac_md5_digest(desc: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32 {
    hmac_md5(desc as *const hmac_md5_key, data, len, out);
    0
}

unsafe fn crypto_hmac_md5_export(desc: *mut shash_desc, out: *mut c_void) -> i32 {
    __crypto_md5_export(desc as *const md5_ctx, out)
}

unsafe fn crypto_hmac_md5_import(desc: *mut shash_desc, input: *const c_void) -> i32 {
    __crypto_md5_import(desc as *mut md5_ctx, input)
}

unsafe fn crypto_hmac_md5_export_core(desc: *mut shash_desc, out: *mut c_void) -> i32 {
    __crypto_md5_export_core(desc as *const md5_ctx, out)
}

unsafe fn crypto_hmac_md5_import_core(desc: *mut shash_desc, input: *const c_void) -> i32 {
    __crypto_md5_import_core(desc as *mut md5_ctx, input)
}

// The C shash_alg aggregate is supplied by the kernel ABI translation.
extern "C" {
    static mut algs: [shash_alg; 2];
    fn crypto_md5_mod_init() -> i32;
    fn crypto_md5_mod_exit();
}

const MODULE_LICENSE: &str = "GPL";
const MODULE_DESCRIPTION: &str = "Crypto API support for MD5 and HMAC-MD5";
const MODULE_ALIASES: [&str; 4] = ["md5", "md5-lib", "hmac(md5)", "hmac-md5-lib"];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
