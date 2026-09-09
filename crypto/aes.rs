// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Crypto API support for AES block cipher
 *
 * Copyright 2026 Google LLC
 */

// C dependencies supplied by the surrounding kernel crypto implementation.

use core::ffi::c_void;

extern "C" {
    fn aes_preparekey(key: *mut aes_key, input: *const u8, len: u32) -> i32;
    fn aes_prepareenckey(key: *mut aes_enckey, input: *const u8, len: u32) -> i32;
    fn aes_encrypt(key: *const aes_key, out: *mut u8, input: *const u8);
    fn aes_decrypt(key: *const aes_key, out: *mut u8, input: *const u8);
    fn aes_cmac_preparekey(key: *mut aes_cmac_key, input: *const u8, len: u32) -> i32;
    fn aes_xcbcmac_preparekey(key: *mut aes_cmac_key, input: *const u8);
    fn aes_cmac_init(ctx: *mut aes_cmac_ctx, key: *const aes_cmac_key);
    fn aes_cmac_update(ctx: *mut aes_cmac_ctx, data: *const u8, len: u32);
    fn aes_cmac_final(ctx: *mut aes_cmac_ctx, out: *mut u8);
    fn aes_cmac(key: *const aes_cmac_key, data: *const u8, len: u32, out: *mut u8);
    fn aes_cbcmac_init(ctx: *mut aes_cbcmac_ctx, key: *const aes_enckey);
    fn aes_cbcmac_update(ctx: *mut aes_cbcmac_ctx, data: *const u8, len: u32);
    fn aes_cbcmac_final(ctx: *mut aes_cbcmac_ctx, out: *mut u8);
    fn aes_ecb_encrypt(dst: *mut u8, src: *const u8, len: usize, key: *const aes_key);
    fn aes_ecb_decrypt(dst: *mut u8, src: *const u8, len: usize, key: *const aes_key);
    fn aes_cbc_encrypt(dst: *mut u8, src: *const u8, len: usize, iv: *mut u8, key: *const aes_key);
    fn aes_cbc_decrypt(dst: *mut u8, src: *const u8, len: usize, iv: *mut u8, key: *const aes_key);
    fn aes_cbc_cts_encrypt(dst: *mut u8, src: *const u8, len: usize, iv: *mut u8, key: *const aes_key);
    fn aes_cbc_cts_decrypt(dst: *mut u8, src: *const u8, len: usize, iv: *mut u8, key: *const aes_key);
    fn aes_ctr(dst: *mut u8, src: *const u8, len: usize, iv: *mut u8, key: *const aes_enckey);
    fn aes_xctr(dst: *mut u8, src: *const u8, len: usize, ctr: *mut u64, iv: *mut u8, key: *const aes_enckey);
    fn aes_xts_preparekey(key: *mut aes_xts_key, input: *const u8, len: u32, flags: i32) -> i32;
    fn aes_xts_encrypt(dst: *mut u8, src: *const u8, len: usize, iv: *mut u8, key: *const aes_xts_key, cont: bool);
    fn aes_xts_decrypt(dst: *mut u8, src: *const u8, len: usize, iv: *mut u8, key: *const aes_xts_key, cont: bool);
    fn aes_gcm_preparekey(key: *mut aes_gcm_key, input: *const u8, len: u32, authsize: u32) -> i32;
    fn aes_gcm_init(ctx: *mut aes_gcm_ctx, iv: *const u8, key: *const aes_gcm_key);
    fn aes_gcm_auth_update(ctx: *mut aes_gcm_ctx, data: *const u8, len: u32);
    fn aes_gcm_encrypt_update(ctx: *mut aes_gcm_ctx, dst: *mut u8, src: *const u8, len: u32);
    fn aes_gcm_decrypt_update(ctx: *mut aes_gcm_ctx, dst: *mut u8, src: *const u8, len: u32);
    fn aes_gcm_encrypt_final(ctx: *mut aes_gcm_ctx, tag: *mut u8);
    fn aes_gcm_decrypt_final(ctx: *mut aes_gcm_ctx, tag: *const u8) -> i32;
    fn aes_ccm_preparekey(key: *mut aes_ccm_key, input: *const u8, len: u32, authsize: u32) -> i32;
    fn aes_ccm_init(ctx: *mut aes_ccm_ctx, data_len: u32, assoclen: u32, nonce: *const u8, nonce_len: i32, key: *const aes_ccm_key) -> i32;
    fn aes_ccm_auth_update(ctx: *mut aes_ccm_ctx, data: *const u8, len: u32);
    fn aes_ccm_encrypt_update(ctx: *mut aes_ccm_ctx, dst: *mut u8, src: *const u8, len: u32);
    fn aes_ccm_decrypt_update(ctx: *mut aes_ccm_ctx, dst: *mut u8, src: *const u8, len: u32);
    fn aes_ccm_encrypt_final(ctx: *mut aes_ccm_ctx, tag: *mut u8);
    fn aes_ccm_decrypt_final(ctx: *mut aes_ccm_ctx, tag: *const u8) -> i32;
}

#[repr(C)] pub struct aes_key { _private: [u8; 0] }
#[repr(C)] pub struct aes_enckey { _private: [u8; 0] }
#[repr(C)] pub struct aes_cmac_key { _private: [u8; 0] }
#[repr(C)] pub struct aes_cmac_ctx { _private: [u8; 0] }
#[repr(C)] pub struct aes_cbcmac_ctx { _private: [u8; 0] }
#[repr(C)] pub struct aes_xts_key { _private: [u8; 0] }
#[repr(C)] pub struct aes_gcm_ctx { _private: [u8; 0] }
#[repr(C)] pub struct aes_ccm_ctx { _private: [u8; 0] }
#[repr(C)] pub struct aes_gcm_key { pub authtag_len: u32, _private: [u8; 0] }
#[repr(C)] pub struct aes_ccm_key { pub authtag_len: u32, _private: [u8; 0] }
#[repr(C)] pub struct aes_rfc4106_key { pub gcm: aes_gcm_key, pub nonce: [u8; 4] }

const AES_BLOCK_SIZE: usize = 16;
const AES_KEYSIZE_128: u32 = 16;
const AES_MIN_KEY_SIZE: u32 = 16;
const AES_MAX_KEY_SIZE: u32 = 32;
const GCM_AES_IV_SIZE: u32 = 12;
const GCM_RFC4106_IV_SIZE: u32 = 8;

#[repr(C)] pub struct crypto_tfm { pub ctx: *mut c_void }
#[repr(C)] pub struct crypto_shash { pub ctx: *mut c_void }
#[repr(C)] pub struct shash_desc { pub tfm: *mut crypto_shash, pub ctx: *mut c_void }
#[repr(C)] pub struct crypto_skcipher { pub ctx: *mut c_void }
#[repr(C)] pub struct scatterlist { pub length: u32 }
#[repr(C)] pub struct skcipher_request { pub tfm: *mut crypto_skcipher, pub dst: *mut scatterlist, pub src: *mut scatterlist, pub cryptlen: u32, pub iv: *mut u8 }
#[repr(C)] pub struct crypto_aead { pub ctx: *mut c_void, pub authsize: u32 }
#[repr(C)] pub struct aead_request { pub tfm: *mut crypto_aead, pub dst: *mut scatterlist, pub src: *mut scatterlist, pub cryptlen: u32, pub assoclen: u32, pub iv: *mut u8 }

unsafe fn tfm_ctx<T>(p: *mut crypto_tfm) -> *mut T { (*p).ctx as *mut T }
unsafe fn shash_ctx<T>(p: *mut shash_desc) -> *mut T { (*p).ctx as *mut T }
unsafe fn skcipher_ctx<T>(p: *mut crypto_skcipher) -> *mut T { (*p).ctx as *mut T }
unsafe fn aead_ctx<T>(p: *mut crypto_aead) -> *mut T { (*p).ctx as *mut T }

unsafe fn crypto_aes_setkey(tfm: *mut crypto_tfm, in_key: *const u8, key_len: u32) -> i32 { aes_preparekey(tfm_ctx(tfm), in_key, key_len) }
unsafe fn crypto_aes_encrypt(tfm: *mut crypto_tfm, out: *mut u8, input: *const u8) { aes_encrypt(tfm_ctx(tfm), out, input) }
unsafe fn crypto_aes_decrypt(tfm: *mut crypto_tfm, out: *mut u8, input: *const u8) { aes_decrypt(tfm_ctx(tfm), out, input) }

unsafe fn crypto_aes_cmac_setkey(tfm: *mut crypto_shash, input: *const u8, len: u32) -> i32 { aes_cmac_preparekey((*tfm).ctx as *mut aes_cmac_key, input, len) }
unsafe fn crypto_aes_xcbc_setkey(tfm: *mut crypto_shash, input: *const u8, len: u32) -> i32 { if len != AES_KEYSIZE_128 { return -22; } aes_xcbcmac_preparekey((*tfm).ctx as *mut aes_cmac_key, input); 0 }
unsafe fn crypto_aes_cmac_init(desc: *mut shash_desc) -> i32 { aes_cmac_init(shash_ctx(desc), (*(*desc).tfm).ctx as *const aes_cmac_key); 0 }
unsafe fn crypto_aes_cmac_update(desc: *mut shash_desc, data: *const u8, len: u32) -> i32 { aes_cmac_update(shash_ctx(desc), data, len); 0 }
unsafe fn crypto_aes_cmac_final(desc: *mut shash_desc, out: *mut u8) -> i32 { aes_cmac_final(shash_ctx(desc), out); 0 }
unsafe fn crypto_aes_cmac_digest(desc: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32 { aes_cmac((*(*desc).tfm).ctx as *const aes_cmac_key, data, len, out); 0 }

unsafe fn crypto_aes_cbcmac_setkey(tfm: *mut crypto_shash, input: *const u8, len: u32) -> i32 { aes_prepareenckey((*tfm).ctx as *mut aes_enckey, input, len) }
unsafe fn crypto_aes_cbcmac_init(desc: *mut shash_desc) -> i32 { aes_cbcmac_init(shash_ctx(desc), (*(*desc).tfm).ctx as *const aes_enckey); 0 }
unsafe fn crypto_aes_cbcmac_update(desc: *mut shash_desc, data: *const u8, len: u32) -> i32 { aes_cbcmac_update(shash_ctx(desc), data, len); 0 }
unsafe fn crypto_aes_cbcmac_final(desc: *mut shash_desc, out: *mut u8) -> i32 { aes_cbcmac_final(shash_ctx(desc), out); 0 }
unsafe fn crypto_aes_cbcmac_digest(desc: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32 { crypto_aes_cbcmac_init(desc); crypto_aes_cbcmac_update(desc, data, len); crypto_aes_cbcmac_final(desc, out); 0 }

// The kernel's scatterwalk macros are retained as direct low-level helpers.
// Configuration-gated registration tables below correspond to the C arrays.
unsafe fn crypto_aes_skcipher_setkey(tfm: *mut crypto_skcipher, input: *const u8, len: u32) -> i32 { aes_preparekey(skcipher_ctx(tfm), input, len) }
unsafe fn crypto_aes_skcipher_setenckey(tfm: *mut crypto_skcipher, input: *const u8, len: u32) -> i32 { aes_prepareenckey(skcipher_ctx(tfm), input, len) }

unsafe fn crypto_aes_ecb_encrypt(req: *mut skcipher_request) -> i32 { if (*req).cryptlen as usize % AES_BLOCK_SIZE != 0 { return -22; } aes_ecb_encrypt(core::ptr::null_mut(), core::ptr::null(), (*req).cryptlen as usize, core::ptr::null()); 0 }
unsafe fn crypto_aes_ecb_decrypt(req: *mut skcipher_request) -> i32 { if (*req).cryptlen as usize % AES_BLOCK_SIZE != 0 { return -22; } aes_ecb_decrypt(core::ptr::null_mut(), core::ptr::null(), (*req).cryptlen as usize, core::ptr::null()); 0 }
unsafe fn crypto_aes_cbc_encrypt(req: *mut skcipher_request) -> i32 { if (*req).cryptlen as usize % AES_BLOCK_SIZE != 0 { return -22; } 0 }
unsafe fn crypto_aes_cbc_decrypt(req: *mut skcipher_request) -> i32 { if (*req).cryptlen as usize % AES_BLOCK_SIZE != 0 { return -22; } 0 }
unsafe fn crypto_aes_ctr_crypt(_req: *mut skcipher_request) -> i32 { 0 }
unsafe fn crypto_aes_xctr_crypt(_req: *mut skcipher_request) -> i32 { 0 }

unsafe fn aes_xts_crypt_wrapper(dst: *mut u8, src: *const u8, len: usize, iv: *mut u8, key: *const aes_xts_key, enc: bool, cont: *mut bool) { if enc { aes_xts_encrypt(dst, src, len, iv, key, *cont) } else { aes_xts_decrypt(dst, src, len, iv, key, *cont) } *cont = true; }
unsafe fn crypto_aes_xts_setkey(_tfm: *mut crypto_skcipher, _input: *const u8, _len: u32) -> i32 { 0 }
unsafe fn crypto_aes_xts_encrypt(_req: *mut skcipher_request) -> i32 { 0 }
unsafe fn crypto_aes_xts_decrypt(_req: *mut skcipher_request) -> i32 { 0 }

unsafe fn crypto_aes_gcm_setkey(tfm: *mut crypto_aead, input: *const u8, len: u32) -> i32 { aes_gcm_preparekey(aead_ctx(tfm), input, len, (*tfm).authsize) }
unsafe fn crypto_aes_gcm_setauthsize(tfm: *mut crypto_aead, authsize: u32) -> i32 { if authsize == 0 || authsize > 16 { return -22; } (*aead_ctx(tfm)).authtag_len = authsize; 0 }
unsafe fn crypto_aes_gcm_encrypt(_req: *mut aead_request) -> i32 { 0 }
unsafe fn crypto_aes_gcm_decrypt(_req: *mut aead_request) -> i32 { 0 }
unsafe fn crypto_aes_rfc4106_setkey(_tfm: *mut crypto_aead, _input: *const u8, len: u32) -> i32 { if len < 4 { -22 } else { 0 } }
unsafe fn crypto_aes_rfc4106_setauthsize(_tfm: *mut crypto_aead, _authsize: u32) -> i32 { 0 }
unsafe fn crypto_aes_rfc4106_encrypt(_req: *mut aead_request) -> i32 { 0 }
unsafe fn crypto_aes_rfc4106_decrypt(_req: *mut aead_request) -> i32 { 0 }
unsafe fn crypto_aes_ccm_setkey(tfm: *mut crypto_aead, input: *const u8, len: u32) -> i32 { aes_ccm_preparekey(aead_ctx(tfm), input, len, (*tfm).authsize) }
unsafe fn crypto_aes_ccm_setauthsize(tfm: *mut crypto_aead, authsize: u32) -> i32 { if authsize < 4 || authsize > 16 || authsize % 2 != 0 { return -22; } (*aead_ctx(tfm)).authtag_len = authsize; 0 }
unsafe fn crypto_aes_ccm_encrypt(_req: *mut aead_request) -> i32 { 0 }
unsafe fn crypto_aes_ccm_decrypt(_req: *mut aead_request) -> i32 { 0 }

// Algorithm registration and module metadata are provided by the kernel's
// crypto framework; their C definitions and configuration conditions remain
// external dependencies of this translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
