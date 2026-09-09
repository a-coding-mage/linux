// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Crypto API support for SHA-224, SHA-256, HMAC-SHA224, and HMAC-SHA256
 *
 * Copyright (c) Jean-Luc Cooke <jlcooke@certainkey.com>
 * Copyright (c) Andrew McDonald <andrew@mcdonald.org.uk>
 * Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
 * SHA224 Support Copyright 2007 Intel Corporation <jonathan.lynch@intel.com>
 * Copyright 2025 Google LLC
 */
// Dependencies are supplied by the surrounding kernel Rust environment.

const SHA256_SHASH_STATE_SIZE: usize = 105;

// static_assert(offsetof(struct __sha256_ctx, state) == 0);
// static_assert(offsetof(struct __sha256_ctx, bytecount) == 32);
// static_assert(offsetof(struct __sha256_ctx, buf) == 40);
// static_assert(sizeof(struct __sha256_ctx) + 1 == SHA256_SHASH_STATE_SIZE);

unsafe fn __crypto_sha256_export(ctx0: *const __sha256_ctx, out: *mut core::ffi::c_void) -> i32 {
    let mut ctx = *ctx0;
    let partial: u8;
    let mut p = out as *mut u8;

    partial = (ctx.bytecount % SHA256_BLOCK_SIZE as _) as u8;
    ctx.bytecount -= partial as _;
    memcpy(p as *mut core::ffi::c_void, &ctx as *const _ as *const core::ffi::c_void, core::mem::size_of::<__sha256_ctx>());
    p = p.add(core::mem::size_of::<__sha256_ctx>());
    *p = partial;
    0
}

unsafe fn __crypto_sha256_import(ctx: *mut __sha256_ctx, input: *const core::ffi::c_void) -> i32 {
    let mut p = input as *const u8;
    memcpy(ctx as *mut core::ffi::c_void, p as *const core::ffi::c_void, core::mem::size_of::<__sha256_ctx>());
    p = p.add(core::mem::size_of::<__sha256_ctx>());
    (*ctx).bytecount += *p as _;
    0
}

unsafe fn __crypto_sha256_export_core(ctx: *const __sha256_ctx, out: *mut core::ffi::c_void) -> i32 {
    memcpy(out, ctx as *const core::ffi::c_void, core::mem::offset_of!(__sha256_ctx, buf));
    0
}

unsafe fn __crypto_sha256_import_core(ctx: *mut __sha256_ctx, input: *const core::ffi::c_void) -> i32 {
    memcpy(ctx as *mut core::ffi::c_void, input, core::mem::offset_of!(__sha256_ctx, buf));
    0
}

/* SHA-224 */

#[no_mangle]
pub static sha224_zero_message_hash: [u8; SHA224_DIGEST_SIZE] = [
    0xd1, 0x4a, 0x02, 0x8c, 0x2a, 0x3a, 0x2b, 0xc9, 0x47,
    0x61, 0x02, 0xbb, 0x28, 0x82, 0x34, 0xc4, 0x15, 0xa2,
    0xb0, 0x1f, 0x82, 0x8e, 0xa6, 0x2a, 0xc5, 0xb3, 0xe4,
    0x2f,
];

unsafe fn crypto_sha224_init(desc: *mut shash_desc) -> i32 { sha224_init(SHA224_CTX(desc)); 0 }
unsafe fn crypto_sha224_update(desc: *mut shash_desc, data: *const u8, len: u32) -> i32 { sha224_update(SHA224_CTX(desc), data, len); 0 }
unsafe fn crypto_sha224_final(desc: *mut shash_desc, out: *mut u8) -> i32 { sha224_final(SHA224_CTX(desc), out); 0 }
unsafe fn crypto_sha224_digest(_desc: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32 { sha224(data, len, out); 0 }
unsafe fn crypto_sha224_export(desc: *mut shash_desc, out: *mut core::ffi::c_void) -> i32 { __crypto_sha256_export(&(*SHA224_CTX(desc)).ctx, out) }
unsafe fn crypto_sha224_import(desc: *mut shash_desc, input: *const core::ffi::c_void) -> i32 { __crypto_sha256_import(&mut (*SHA224_CTX(desc)).ctx, input) }
unsafe fn crypto_sha224_export_core(desc: *mut shash_desc, out: *mut core::ffi::c_void) -> i32 { __crypto_sha256_export_core(&(*SHA224_CTX(desc)).ctx, out) }
unsafe fn crypto_sha224_import_core(desc: *mut shash_desc, input: *const core::ffi::c_void) -> i32 { __crypto_sha256_import_core(&mut (*SHA224_CTX(desc)).ctx, input) }

/* SHA-256 */

#[no_mangle]
pub static sha256_zero_message_hash: [u8; SHA256_DIGEST_SIZE] = [
    0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14,
    0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9, 0x24,
    0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c,
    0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52, 0xb8, 0x55,
];

unsafe fn crypto_sha256_init(desc: *mut shash_desc) -> i32 { sha256_init(SHA256_CTX(desc)); 0 }
unsafe fn crypto_sha256_update(desc: *mut shash_desc, data: *const u8, len: u32) -> i32 { sha256_update(SHA256_CTX(desc), data, len); 0 }
unsafe fn crypto_sha256_final(desc: *mut shash_desc, out: *mut u8) -> i32 { sha256_final(SHA256_CTX(desc), out); 0 }
unsafe fn crypto_sha256_digest(_desc: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32 { sha256(data, len, out); 0 }
unsafe fn crypto_sha256_export(desc: *mut shash_desc, out: *mut core::ffi::c_void) -> i32 { __crypto_sha256_export(&(*SHA256_CTX(desc)).ctx, out) }
unsafe fn crypto_sha256_import(desc: *mut shash_desc, input: *const core::ffi::c_void) -> i32 { __crypto_sha256_import(&mut (*SHA256_CTX(desc)).ctx, input) }
unsafe fn crypto_sha256_export_core(desc: *mut shash_desc, out: *mut core::ffi::c_void) -> i32 { __crypto_sha256_export_core(&(*SHA256_CTX(desc)).ctx, out) }
unsafe fn crypto_sha256_import_core(desc: *mut shash_desc, input: *const core::ffi::c_void) -> i32 { __crypto_sha256_import_core(&mut (*SHA256_CTX(desc)).ctx, input) }

/* HMAC-SHA224 */
unsafe fn crypto_hmac_sha224_setkey(tfm: *mut crypto_shash, raw_key: *const u8, keylen: u32) -> i32 { hmac_sha224_preparekey(HMAC_SHA224_KEY(tfm), raw_key, keylen); 0 }
unsafe fn crypto_hmac_sha224_init(desc: *mut shash_desc) -> i32 { hmac_sha224_init(HMAC_SHA224_CTX(desc), HMAC_SHA224_KEY((*desc).tfm)); 0 }
unsafe fn crypto_hmac_sha224_update(desc: *mut shash_desc, data: *const u8, len: u32) -> i32 { hmac_sha224_update(HMAC_SHA224_CTX(desc), data, len); 0 }
unsafe fn crypto_hmac_sha224_final(desc: *mut shash_desc, out: *mut u8) -> i32 { hmac_sha224_final(HMAC_SHA224_CTX(desc), out); 0 }
unsafe fn crypto_hmac_sha224_digest(desc: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32 { hmac_sha224(HMAC_SHA224_KEY((*desc).tfm), data, len, out); 0 }
unsafe fn crypto_hmac_sha224_export(desc: *mut shash_desc, out: *mut core::ffi::c_void) -> i32 { __crypto_sha256_export(&(*HMAC_SHA224_CTX(desc)).ctx.sha_ctx, out) }
unsafe fn crypto_hmac_sha224_import(desc: *mut shash_desc, input: *const core::ffi::c_void) -> i32 { let ctx = HMAC_SHA224_CTX(desc); (*ctx).ctx.ostate = (*HMAC_SHA224_KEY((*desc).tfm)).key.ostate; __crypto_sha256_import(&mut (*ctx).ctx.sha_ctx, input) }
unsafe fn crypto_hmac_sha224_export_core(desc: *mut shash_desc, out: *mut core::ffi::c_void) -> i32 { __crypto_sha256_export_core(&(*HMAC_SHA224_CTX(desc)).ctx.sha_ctx, out) }
unsafe fn crypto_hmac_sha224_import_core(desc: *mut shash_desc, input: *const core::ffi::c_void) -> i32 { let ctx = HMAC_SHA224_CTX(desc); (*ctx).ctx.ostate = (*HMAC_SHA224_KEY((*desc).tfm)).key.ostate; __crypto_sha256_import_core(&mut (*ctx).ctx.sha_ctx, input) }

/* HMAC-SHA256 */
unsafe fn crypto_hmac_sha256_setkey(tfm: *mut crypto_shash, raw_key: *const u8, keylen: u32) -> i32 { hmac_sha256_preparekey(HMAC_SHA256_KEY(tfm), raw_key, keylen); 0 }
unsafe fn crypto_hmac_sha256_init(desc: *mut shash_desc) -> i32 { hmac_sha256_init(HMAC_SHA256_CTX(desc), HMAC_SHA256_KEY((*desc).tfm)); 0 }
unsafe fn crypto_hmac_sha256_update(desc: *mut shash_desc, data: *const u8, len: u32) -> i32 { hmac_sha256_update(HMAC_SHA256_CTX(desc), data, len); 0 }
unsafe fn crypto_hmac_sha256_final(desc: *mut shash_desc, out: *mut u8) -> i32 { hmac_sha256_final(HMAC_SHA256_CTX(desc), out); 0 }
unsafe fn crypto_hmac_sha256_digest(desc: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32 { hmac_sha256(HMAC_SHA256_KEY((*desc).tfm), data, len, out); 0 }
unsafe fn crypto_hmac_sha256_export(desc: *mut shash_desc, out: *mut core::ffi::c_void) -> i32 { __crypto_sha256_export(&(*HMAC_SHA256_CTX(desc)).ctx.sha_ctx, out) }
unsafe fn crypto_hmac_sha256_import(desc: *mut shash_desc, input: *const core::ffi::c_void) -> i32 { let ctx = HMAC_SHA256_CTX(desc); (*ctx).ctx.ostate = (*HMAC_SHA256_KEY((*desc).tfm)).key.ostate; __crypto_sha256_import(&mut (*ctx).ctx.sha_ctx, input) }
unsafe fn crypto_hmac_sha256_export_core(desc: *mut shash_desc, out: *mut core::ffi::c_void) -> i32 { __crypto_sha256_export_core(&(*HMAC_SHA256_CTX(desc)).ctx.sha_ctx, out) }
unsafe fn crypto_hmac_sha256_import_core(desc: *mut shash_desc, input: *const core::ffi::c_void) -> i32 { let ctx = HMAC_SHA256_CTX(desc); (*ctx).ctx.ostate = (*HMAC_SHA256_KEY((*desc).tfm)).key.ostate; __crypto_sha256_import_core(&mut (*ctx).ctx.sha_ctx, input) }

/* Algorithm definitions */
// The surrounding kernel provides `shash_alg`; these entries preserve the
// original names, priorities, sizes, callbacks, and registration order.
#[allow(non_upper_case_globals)]
static mut algs: [shash_alg; 4] = [
    shash_alg::new("sha224", "sha224-lib", 300, SHA224_BLOCK_SIZE, SHA224_DIGEST_SIZE, crypto_sha224_init, crypto_sha224_update, crypto_sha224_final, crypto_sha224_digest, crypto_sha224_export, crypto_sha224_import, crypto_sha224_export_core, crypto_sha224_import_core, core::mem::size_of::<sha224_ctx>(), SHA256_SHASH_STATE_SIZE),
    shash_alg::new("sha256", "sha256-lib", 300, SHA256_BLOCK_SIZE, SHA256_DIGEST_SIZE, crypto_sha256_init, crypto_sha256_update, crypto_sha256_final, crypto_sha256_digest, crypto_sha256_export, crypto_sha256_import, crypto_sha256_export_core, crypto_sha256_import_core, core::mem::size_of::<sha256_ctx>(), SHA256_SHASH_STATE_SIZE),
    shash_alg::new_hmac("hmac(sha224)", "hmac-sha224-lib", 300, SHA224_BLOCK_SIZE, SHA224_DIGEST_SIZE, crypto_hmac_sha224_setkey, crypto_hmac_sha224_init, crypto_hmac_sha224_update, crypto_hmac_sha224_final, crypto_hmac_sha224_digest, crypto_hmac_sha224_export, crypto_hmac_sha224_import, crypto_hmac_sha224_export_core, crypto_hmac_sha224_import_core, core::mem::size_of::<hmac_sha224_key>(), core::mem::size_of::<hmac_sha224_ctx>(), SHA256_SHASH_STATE_SIZE),
    shash_alg::new_hmac("hmac(sha256)", "hmac-sha256-lib", 300, SHA256_BLOCK_SIZE, SHA256_DIGEST_SIZE, crypto_hmac_sha256_setkey, crypto_hmac_sha256_init, crypto_hmac_sha256_update, crypto_hmac_sha256_final, crypto_hmac_sha256_digest, crypto_hmac_sha256_export, crypto_hmac_sha256_import, crypto_hmac_sha256_export_core, crypto_hmac_sha256_import_core, core::mem::size_of::<hmac_sha256_key>(), core::mem::size_of::<hmac_sha256_ctx>(), SHA256_SHASH_STATE_SIZE),
];

unsafe fn crypto_sha256_mod_init() -> i32 { crypto_register_shashes(algs.as_mut_ptr(), algs.len()) }
unsafe fn crypto_sha256_mod_exit() { crypto_unregister_shashes(algs.as_mut_ptr(), algs.len()); }

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Crypto API support for SHA-224, SHA-256, HMAC-SHA224, and HMAC-SHA256");
// MODULE_ALIAS_CRYPTO("sha224"); MODULE_ALIAS_CRYPTO("sha224-lib");
// MODULE_ALIAS_CRYPTO("sha256"); MODULE_ALIAS_CRYPTO("sha256-lib");
// MODULE_ALIAS_CRYPTO("hmac(sha224)"); MODULE_ALIAS_CRYPTO("hmac-sha224-lib");
// MODULE_ALIAS_CRYPTO("hmac(sha256)"); MODULE_ALIAS_CRYPTO("hmac-sha256-lib");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
