/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AMD Cryptographic Coprocessor (CCP) crypto API support
 *
 * Copyright (C) 2013,2017 Advanced Micro Devices, Inc.
 *
 * Author: Tom Lendacky <thomas.lendacky@amd.com>
 */

/* Linux/kernel and crypto dependencies are supplied by the surrounding crate. */

pub const CCP_LOG_LEVEL: &str = "KERN_INFO";
pub const CCP_CRA_PRIORITY: i32 = 300;

#[repr(C)]
pub struct ccp_crypto_skcipher_alg {
    pub entry: list_head,
    pub mode: u32,
    pub alg: skcipher_alg,
}

#[repr(C)]
pub struct ccp_crypto_aead {
    pub entry: list_head,
    pub mode: u32,
    pub alg: aead_alg,
}

#[repr(C)]
pub struct ccp_crypto_ahash_alg {
    pub entry: list_head,
    pub init: *const __be32,
    pub type_: u32,
    pub mode: u32,
    /* Child algorithm used for HMAC, CMAC, etc */
    pub child_alg: [c_char; CRYPTO_MAX_ALG_NAME],
    pub alg: ahash_alg,
}

#[repr(C)]
pub struct ccp_crypto_akcipher_alg {
    pub entry: list_head,
    pub alg: akcipher_alg,
}

#[inline]
pub unsafe fn ccp_crypto_skcipher_alg(tfm: *mut crypto_skcipher) -> *mut ccp_crypto_skcipher_alg {
    let alg = crypto_skcipher_alg(tfm);
    container_of!(alg, ccp_crypto_skcipher_alg, alg)
}

#[inline]
pub unsafe fn ccp_crypto_ahash_alg(tfm: *mut crypto_tfm) -> *mut ccp_crypto_ahash_alg {
    let alg = (*tfm).__crt_alg;
    let ahash_alg = container_of!(alg, ahash_alg, halg.base);
    container_of!(ahash_alg, ccp_crypto_ahash_alg, alg)
}

/* AES related defines */
#[repr(C)]
pub struct ccp_aes_ctx {
    /* Fallback cipher for XTS with unsupported unit sizes */
    pub tfm_skcipher: *mut crypto_skcipher,
    pub engine: ccp_engine,
    pub type_: ccp_aes_type,
    pub mode: ccp_aes_mode,
    pub key_sg: scatterlist,
    pub key_len: c_uint,
    pub key: [u8; AES_MAX_KEY_SIZE * 2],
    pub nonce: [u8; CTR_RFC3686_NONCE_SIZE],
    /* CMAC key structures */
    pub k1_sg: scatterlist,
    pub k2_sg: scatterlist,
    pub kn_len: c_uint,
    pub k1: [u8; AES_BLOCK_SIZE],
    pub k2: [u8; AES_BLOCK_SIZE],
}

#[repr(C)]
pub struct ccp_aes_req_ctx {
    pub iv_sg: scatterlist,
    pub iv: [u8; AES_BLOCK_SIZE],
    pub tag_sg: scatterlist,
    pub tag: [u8; AES_BLOCK_SIZE],
    /* Fields used for RFC3686 requests */
    pub rfc3686_info: *mut u8,
    pub rfc3686_iv: [u8; AES_BLOCK_SIZE],
    pub cmd: ccp_cmd,
    pub fallback_req: skcipher_request, /* keep at the end */
}

#[repr(C)]
pub struct ccp_aes_cmac_req_ctx {
    pub null_msg: c_uint,
    pub final_: c_uint,
    pub src: *mut scatterlist,
    pub nbytes: c_uint,
    pub hash_cnt: u64,
    pub hash_rem: c_uint,
    pub data_sg: sg_table,
    pub iv_sg: scatterlist,
    pub iv: [u8; AES_BLOCK_SIZE],
    pub buf_sg: scatterlist,
    pub buf_count: c_uint,
    pub buf: [u8; AES_BLOCK_SIZE],
    pub pad_sg: scatterlist,
    pub pad_count: c_uint,
    pub pad: [u8; AES_BLOCK_SIZE],
    pub cmd: ccp_cmd,
}

#[repr(C)]
pub struct ccp_aes_cmac_exp_ctx {
    pub null_msg: c_uint,
    pub iv: [u8; AES_BLOCK_SIZE],
    pub buf_count: c_uint,
    pub buf: [u8; AES_BLOCK_SIZE],
}

/* 3DES related defines */
#[repr(C)]
pub struct ccp_des3_ctx {
    pub engine: ccp_engine,
    pub type_: ccp_des3_type,
    pub mode: ccp_des3_mode,
    pub key_sg: scatterlist,
    pub key_len: c_uint,
    pub key: [u8; AES_MAX_KEY_SIZE],
}

#[repr(C)]
pub struct ccp_des3_req_ctx {
    pub iv_sg: scatterlist,
    pub iv: [u8; AES_BLOCK_SIZE],
    pub cmd: ccp_cmd,
}

/* SHA-related defines. These values must be large enough to accommodate any variant. */
pub const MAX_SHA_CONTEXT_SIZE: usize = SHA512_DIGEST_SIZE;
pub const MAX_SHA_BLOCK_SIZE: usize = SHA512_BLOCK_SIZE;

#[repr(C)]
pub struct ccp_sha_ctx {
    pub opad_sg: scatterlist,
    pub opad_count: c_uint,
    pub key_len: c_uint,
    pub key: [u8; MAX_SHA_BLOCK_SIZE],
    pub ipad: [u8; MAX_SHA_BLOCK_SIZE],
    pub opad: [u8; MAX_SHA_BLOCK_SIZE],
    pub hmac_tfm: *mut crypto_shash,
}

#[repr(C)]
pub struct ccp_sha_req_ctx {
    pub type_: ccp_sha_type,
    pub msg_bits: u64,
    pub first: c_uint,
    pub final_: c_uint,
    pub src: *mut scatterlist,
    pub nbytes: c_uint,
    pub hash_cnt: u64,
    pub hash_rem: c_uint,
    pub data_sg: sg_table,
    pub ctx_sg: scatterlist,
    pub ctx: [u8; MAX_SHA_CONTEXT_SIZE],
    pub buf_sg: scatterlist,
    pub buf_count: c_uint,
    pub buf: [u8; MAX_SHA_BLOCK_SIZE],
    /* CCP driver command */
    pub cmd: ccp_cmd,
}

#[repr(C)]
pub struct ccp_sha_exp_ctx {
    pub type_: ccp_sha_type,
    pub msg_bits: u64,
    pub first: c_uint,
    pub ctx: [u8; MAX_SHA_CONTEXT_SIZE],
    pub buf_count: c_uint,
    pub buf: [u8; MAX_SHA_BLOCK_SIZE],
}

/* RSA related defines */
#[repr(C)]
pub struct ccp_rsa_ctx {
    pub key_len: c_uint, /* in bits */
    pub e_sg: scatterlist,
    pub e_buf: *mut u8,
    pub e_len: c_uint,
    pub n_sg: scatterlist,
    pub n_buf: *mut u8,
    pub n_len: c_uint,
    pub d_sg: scatterlist,
    pub d_buf: *mut u8,
    pub d_len: c_uint,
}

#[repr(C)]
pub struct ccp_rsa_req_ctx {
    pub cmd: ccp_cmd,
}

pub const CCP_RSA_MAXMOD: usize = 4 * 1024 / 8;
pub const CCP5_RSA_MAXMOD: usize = 16 * 1024 / 8;

/* Common Context Structure */
#[repr(C)]
pub union ccp_ctx_u {
    pub aes: ccp_aes_ctx,
    pub rsa: ccp_rsa_ctx,
    pub sha: ccp_sha_ctx,
    pub des3: ccp_des3_ctx,
}

#[repr(C)]
pub struct ccp_ctx {
    pub complete: Option<unsafe extern "C" fn(req: *mut crypto_async_request, ret: c_int) -> c_int>,
    pub u: ccp_ctx_u,
}

extern "C" {
    pub fn ccp_crypto_enqueue_request(req: *mut crypto_async_request, cmd: *mut ccp_cmd) -> c_int;
    pub fn ccp_crypto_sg_table_add(table: *mut sg_table, sg_add: *mut scatterlist) -> *mut scatterlist;
    pub fn ccp_register_aes_algs(head: *mut list_head) -> c_int;
    pub fn ccp_register_aes_cmac_algs(head: *mut list_head) -> c_int;
    pub fn ccp_register_aes_xts_algs(head: *mut list_head) -> c_int;
    pub fn ccp_register_aes_aeads(head: *mut list_head) -> c_int;
    pub fn ccp_register_sha_algs(head: *mut list_head) -> c_int;
    pub fn ccp_register_des3_algs(head: *mut list_head) -> c_int;
    pub fn ccp_register_rsa_algs(head: *mut list_head) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
