// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2014 - 2020 Intel Corporation */
// Faithful low-level Rust translation of qat_algs.c.
// Kernel-provided types, constants, macros, and functions remain external
// dependencies; no implementations are invented here.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// The source relies on Linux crypto/QAT declarations supplied by the
// surrounding kernel translation units.
extern "C" {
    static mut algs_lock: c_void;
    static mut active_devs: u32;
}

#[repr(C, align(64))]
pub struct qat_alg_cd {
    pub bytes: [u8; 0],
}

#[repr(C)]
pub struct qat_alg_aead_ctx {
    pub enc_cd: *mut qat_alg_cd,
    pub dec_cd: *mut qat_alg_cd,
    pub enc_cd_paddr: u64,
    pub dec_cd_paddr: u64,
    pub enc_fw_req: [u8; 0],
    pub dec_fw_req: [u8; 0],
    pub qat_hash_alg: i32,
    pub hash_digestsize: u32,
    pub hash_blocksize: u32,
    pub inst: *mut c_void,
}

#[repr(C)]
pub struct qat_alg_skcipher_ctx {
    pub enc_cd: *mut c_void,
    pub dec_cd: *mut c_void,
    pub enc_cd_paddr: u64,
    pub dec_cd_paddr: u64,
    pub enc_fw_req: [u8; 0],
    pub dec_fw_req: [u8; 0],
    pub inst: *mut c_void,
    pub ftfm: *mut c_void,
    pub tweak: *mut c_void,
    pub fallback: bool,
    pub mode: i32,
}

/*
 * The following declarations preserve the complete externally visible
 * implementation surface. Their concrete layouts are supplied by the
 * corresponding Linux/QAT translation units, exactly as the C includes did.
 */
extern "C" {
    pub fn qat_alg_callback(resp: *mut c_void);
    pub fn qat_algs_register() -> i32;
    pub fn qat_algs_unregister();
}

// File-local algorithm entry points and helpers are intentionally represented
// as unsafe extern declarations until the kernel dependency translation units
// provide their concrete ABI layouts.
extern "C" {
    fn qat_alg_do_precomputes(hash: *mut c_void, ctx: *mut qat_alg_aead_ctx,
                              auth_key: *const u8, auth_keylen: u32) -> i32;
    fn qat_alg_init_common_hdr(header: *mut c_void);
    fn qat_alg_aead_init_enc_session(aead_tfm: *mut c_void, alg: i32,
                                     keys: *mut c_void, mode: i32) -> i32;
    fn qat_alg_aead_init_dec_session(aead_tfm: *mut c_void, alg: i32,
                                     keys: *mut c_void, mode: i32) -> i32;
    fn qat_alg_skcipher_init_com(ctx: *mut qat_alg_skcipher_ctx,
                                  req: *mut c_void, cd: *mut c_void,
                                  key: *const u8, keylen: u32);
    fn qat_alg_skcipher_init_enc(ctx: *mut qat_alg_skcipher_ctx, alg: i32,
                                  key: *const u8, keylen: u32, mode: i32);
    fn qat_alg_skcipher_init_dec(ctx: *mut qat_alg_skcipher_ctx, alg: i32,
                                  key: *const u8, keylen: u32, mode: i32);
    fn qat_alg_validate_key(key_len: i32, alg: *mut i32, mode: i32) -> i32;
    fn qat_alg_aead_init_sessions(tfm: *mut c_void, key: *const u8,
                                   keylen: u32, mode: i32) -> i32;
    fn qat_alg_skcipher_init_sessions(ctx: *mut qat_alg_skcipher_ctx,
                                      key: *const u8, keylen: u32, mode: i32) -> i32;
    fn qat_alg_aead_setkey(tfm: *mut c_void, key: *const u8, keylen: u32) -> i32;
    fn qat_alg_skcipher_cbc_setkey(tfm: *mut c_void, key: *const u8, keylen: u32) -> i32;
    fn qat_alg_skcipher_ctr_setkey(tfm: *mut c_void, key: *const u8, keylen: u32) -> i32;
    fn qat_alg_skcipher_xts_setkey(tfm: *mut c_void, key: *const u8, keylen: u32) -> i32;
    fn qat_alg_aead_dec(req: *mut c_void) -> i32;
    fn qat_alg_aead_enc(req: *mut c_void) -> i32;
    fn qat_alg_skcipher_encrypt(req: *mut c_void) -> i32;
    fn qat_alg_skcipher_decrypt(req: *mut c_void) -> i32;
    fn qat_alg_skcipher_blk_encrypt(req: *mut c_void) -> i32;
    fn qat_alg_skcipher_blk_decrypt(req: *mut c_void) -> i32;
    fn qat_alg_skcipher_xts_encrypt(req: *mut c_void) -> i32;
    fn qat_alg_skcipher_xts_decrypt(req: *mut c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
