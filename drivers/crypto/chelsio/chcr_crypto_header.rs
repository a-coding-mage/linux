/*
 * This file is part of the Chelsio T6 Crypto driver for Linux.
 *
 * Copyright (c) 2003-2016 Chelsio Communications, Inc. All rights reserved.
 *
 * This software is available under a choice of one of two licenses.  You may
 * choose to be licensed under the terms of the GNU General Public License
 * (GPL) Version 2, available from the file COPYING in the main directory of
 * this source tree, or the OpenIB.org BSD license.
 */

/* C header dependencies are supplied by the surrounding translation unit. */
use core::ffi::c_void;
use core::mem::ManuallyDrop;
type c_int = i32;
type c_uint = u32;
type c_uchar = u8;
type c_ushort = u16;

pub const GHASH_BLOCK_SIZE: usize = 16;
pub const GHASH_DIGEST_SIZE: usize = 16;
pub const CCM_B0_SIZE: usize = 16;
pub const CCM_AAD_FIELD_SIZE: usize = 2;
// 511 - 16 (For IV)
pub const T6_MAX_AAD_SIZE: usize = 495;

pub const CHCR_CRA_PRIORITY: i32 = 500;
pub const CHCR_AEAD_PRIORITY: i32 = 6000;
pub const CHCR_AES_MAX_KEY_LEN: usize = 2 * AES_MAX_KEY_SIZE; /* consider xts */
pub const CHCR_MAX_CRYPTO_IV_LEN: usize = 16; /* AES IV len */
pub const CHCR_MAX_AUTHENC_AES_KEY_LEN: usize = 32; /* max aes key length */
pub const CHCR_MAX_AUTHENC_SHA_KEY_LEN: usize = 128; /* max sha key length */
pub const CHCR_GIVENCRYPT_OP: i32 = 2;

pub const CHCR_ENCRYPT_OP: i32 = 0;
pub const CHCR_DECRYPT_OP: i32 = 1;
pub const CHCR_SCMD_SEQ_NO_CTRL_32BIT: i32 = 1;
pub const CHCR_SCMD_SEQ_NO_CTRL_48BIT: i32 = 2;
pub const CHCR_SCMD_SEQ_NO_CTRL_64BIT: i32 = 3;
pub const CHCR_SCMD_PROTO_VERSION_GENERIC: i32 = 4;
pub const CHCR_SCMD_AUTH_CTRL_AUTH_CIPHER: i32 = 0;
pub const CHCR_SCMD_AUTH_CTRL_CIPHER_AUTH: i32 = 1;
pub const CHCR_SCMD_CIPHER_MODE_NOP: i32 = 0;
pub const CHCR_SCMD_CIPHER_MODE_AES_CBC: i32 = 1;
pub const CHCR_SCMD_CIPHER_MODE_AES_GCM: i32 = 2;
pub const CHCR_SCMD_CIPHER_MODE_AES_CTR: i32 = 3;
pub const CHCR_SCMD_CIPHER_MODE_GENERIC_AES: i32 = 4;
pub const CHCR_SCMD_CIPHER_MODE_AES_XTS: i32 = 6;
pub const CHCR_SCMD_CIPHER_MODE_AES_CCM: i32 = 7;
pub const CHCR_SCMD_AUTH_MODE_NOP: i32 = 0;
pub const CHCR_SCMD_AUTH_MODE_SHA1: i32 = 1;
pub const CHCR_SCMD_AUTH_MODE_SHA224: i32 = 2;
pub const CHCR_SCMD_AUTH_MODE_SHA256: i32 = 3;
pub const CHCR_SCMD_AUTH_MODE_GHASH: i32 = 4;
pub const CHCR_SCMD_AUTH_MODE_SHA512_224: i32 = 5;
pub const CHCR_SCMD_AUTH_MODE_SHA512_256: i32 = 6;
pub const CHCR_SCMD_AUTH_MODE_SHA512_384: i32 = 7;
pub const CHCR_SCMD_AUTH_MODE_SHA512_512: i32 = 8;
pub const CHCR_SCMD_AUTH_MODE_CBCMAC: i32 = 9;
pub const CHCR_SCMD_AUTH_MODE_CMAC: i32 = 10;
pub const CHCR_SCMD_HMAC_CTRL_NOP: i32 = 0;
pub const CHCR_SCMD_HMAC_CTRL_NO_TRUNC: i32 = 1;
pub const CHCR_SCMD_HMAC_CTRL_TRUNC_RFC4366: i32 = 2;
pub const CHCR_SCMD_HMAC_CTRL_IPSEC_96BIT: i32 = 3;
pub const CHCR_SCMD_HMAC_CTRL_PL1: i32 = 4;
pub const CHCR_SCMD_HMAC_CTRL_PL2: i32 = 5;
pub const CHCR_SCMD_HMAC_CTRL_PL3: i32 = 6;
pub const CHCR_SCMD_HMAC_CTRL_DIV2: i32 = 7;
pub const VERIFY_HW: i32 = 0;
pub const VERIFY_SW: i32 = 1;
pub const CHCR_SCMD_IVGEN_CTRL_HW: i32 = 0;
pub const CHCR_SCMD_IVGEN_CTRL_SW: i32 = 1;
pub const CHCR_KEYCTX_MAC_KEY_SIZE_128: i32 = 0;
pub const CHCR_KEYCTX_MAC_KEY_SIZE_160: i32 = 1;
pub const CHCR_KEYCTX_MAC_KEY_SIZE_192: i32 = 2;
pub const CHCR_KEYCTX_MAC_KEY_SIZE_256: i32 = 3;
pub const CHCR_KEYCTX_MAC_KEY_SIZE_512: i32 = 4;
pub const CHCR_KEYCTX_CIPHER_KEY_SIZE_128: i32 = 0;
pub const CHCR_KEYCTX_CIPHER_KEY_SIZE_192: i32 = 1;
pub const CHCR_KEYCTX_CIPHER_KEY_SIZE_256: i32 = 2;
pub const CHCR_KEYCTX_NO_KEY: i32 = 15;
pub const CHCR_CPL_FW4_PLD_IV_OFFSET: usize = 5 * 64;
pub const CHCR_CPL_FW4_PLD_HASH_RESULT_OFFSET: usize = 7 * 64;
pub const CHCR_CPL_FW4_PLD_DATA_SIZE: usize = 4 * 64;
pub const KEY_CONTEXT_HDR_SALT_AND_PAD: usize = 16;
#[inline] pub const fn flits_to_bytes(x: usize) -> usize { x * 8 }
pub const IV_NOP: i32 = 0;
pub const IV_IMMEDIATE: i32 = 1;
pub const IV_DSGL: i32 = 2;
pub const AEAD_H_SIZE: usize = 16;
pub const CRYPTO_ALG_SUB_TYPE_MASK: u32 = 0x0f000000;
pub const CRYPTO_ALG_SUB_TYPE_HASH_HMAC: u32 = 0x01000000;
pub const CRYPTO_ALG_SUB_TYPE_AEAD_RFC4106: u32 = 0x02000000;
pub const CRYPTO_ALG_SUB_TYPE_AEAD_GCM: u32 = 0x03000000;
pub const CRYPTO_ALG_SUB_TYPE_CBC_SHA: u32 = 0x04000000;
pub const CRYPTO_ALG_SUB_TYPE_AEAD_CCM: u32 = 0x05000000;
pub const CRYPTO_ALG_SUB_TYPE_AEAD_RFC4309: u32 = 0x06000000;
pub const CRYPTO_ALG_SUB_TYPE_CBC_NULL: u32 = 0x07000000;
pub const CRYPTO_ALG_SUB_TYPE_CTR: u32 = 0x08000000;
pub const CRYPTO_ALG_SUB_TYPE_CTR_RFC3686: u32 = 0x09000000;
pub const CRYPTO_ALG_SUB_TYPE_XTS: u32 = 0x0a000000;
pub const CRYPTO_ALG_SUB_TYPE_CBC: u32 = 0x0b000000;
pub const CRYPTO_ALG_SUB_TYPE_CTR_SHA: u32 = 0x0c000000;
pub const CRYPTO_ALG_SUB_TYPE_CTR_NULL: u32 = 0x0d000000;
pub const CRYPTO_ALG_TYPE_HMAC: u32 = CRYPTO_ALG_TYPE_AHASH | CRYPTO_ALG_SUB_TYPE_HASH_HMAC;
pub const MAX_SCRATCH_PAD_SIZE: usize = 32;
pub const CHCR_HASH_MAX_BLOCK_SIZE_64: usize = 64;
pub const CHCR_HASH_MAX_BLOCK_SIZE_128: usize = 128;
pub const CHCR_SRC_SG_SIZE: usize = 0x10000 - core::mem::size_of::<i32>();
pub const CHCR_DST_SG_SIZE: usize = 2048;

#[inline] pub unsafe fn a_ctx(tfm: *mut crypto_aead) -> *mut chcr_context { crypto_aead_ctx(tfm) }
#[inline] pub unsafe fn c_ctx(tfm: *mut crypto_skcipher) -> *mut chcr_context { crypto_skcipher_ctx(tfm) }
#[inline] pub unsafe fn h_ctx(tfm: *mut crypto_ahash) -> *mut chcr_context { crypto_tfm_ctx(crypto_ahash_tfm(tfm)) }

#[repr(C)] pub struct ablk_ctx { pub sw_cipher: *mut crypto_skcipher, pub key_ctx_hdr: __be32, pub enckey_len: c_uint, pub ciph_mode: c_uchar, pub key: [u8; CHCR_AES_MAX_KEY_LEN], pub nonce: [u8; 4], pub rrkey: [u8; AES_MAX_KEY_SIZE] }
#[repr(C)] pub struct chcr_aead_reqctx { pub skb: *mut sk_buff, pub iv_dma: dma_addr_t, pub b0_dma: dma_addr_t, pub b0_len: c_uint, pub op: c_uint, pub imm: u16, pub verify: u16, pub txqidx: u16, pub rxqidx: u16, pub iv: [u8; CHCR_MAX_CRYPTO_IV_LEN + MAX_SCRATCH_PAD_SIZE], pub scratch_pad: *mut u8 }
#[repr(C)] pub struct ulptx_walk { pub sgl: *mut ulptx_sgl, pub nents: c_uint, pub pair_idx: c_uint, pub last_sg_len: c_uint, pub last_sg: *mut scatterlist, pub pair: *mut ulptx_sge_pair }
#[repr(C)] pub struct dsgl_walk { pub nents: c_uint, pub last_sg_len: c_uint, pub last_sg: *mut scatterlist, pub dsgl: *mut cpl_rx_phys_dsgl, pub to: *mut phys_sge_pairs }
#[repr(C)] pub struct chcr_gcm_ctx { pub ghash_h: [u8; AEAD_H_SIZE] }
#[repr(C)] pub struct chcr_authenc_ctx { pub dec_rrkey: [u8; AES_MAX_KEY_SIZE], pub h_iopad: [u8; 2 * CHCR_HASH_MAX_DIGEST_SIZE], pub auth_mode: c_uchar }
#[repr(C)] pub union __aead_ctx { pub gcm: ManuallyDrop<chcr_gcm_ctx>, pub authenc: ManuallyDrop<chcr_authenc_ctx> }
#[repr(C)] pub struct chcr_aead_ctx { pub key_ctx_hdr: __be32, pub enckey_len: c_uint, pub sw_cipher: *mut crypto_aead, pub salt: [u8; MAX_SALT], pub key: [u8; CHCR_AES_MAX_KEY_LEN], pub nonce: [u8; 4], pub hmac_ctrl: u16, pub mayverify: u16, pub ctx: [__aead_ctx; 0] }
#[repr(C)] pub struct hmac_ctx { pub ipad: [u8; CHCR_HASH_MAX_BLOCK_SIZE_128], pub opad: [u8; CHCR_HASH_MAX_BLOCK_SIZE_128] }
#[repr(C)] pub union __crypto_ctx { pub hmacctx: ManuallyDrop<hmac_ctx>, pub ablkctx: ManuallyDrop<ablk_ctx>, pub aeadctx: ManuallyDrop<chcr_aead_ctx> }
#[repr(C)] pub struct chcr_context { pub dev: *mut chcr_dev, pub rxq_perchan: c_uchar, pub txq_perchan: c_uchar, pub ntxq: c_uint, pub nrxq: c_uint, pub cbc_aes_aio_done: completion, pub crypto_ctx: [__crypto_ctx; 0] }
#[repr(C)] pub struct chcr_hctx_per_wr { pub srcsg: *mut scatterlist, pub skb: *mut sk_buff, pub dma_addr: dma_addr_t, pub dma_len: u32, pub src_ofst: c_uint, pub processed: c_uint, pub result: u32, pub is_sg_map: u8, pub imm: u8, pub isfinal: u8 }
#[repr(C)] pub struct chcr_ahash_req_ctx { pub hctx_wr: chcr_hctx_per_wr, pub reqbfr: *mut u8, pub skbfr: *mut u8, pub data_len: u64, pub txqidx: u16, pub rxqidx: u16, pub reqlen: u8, pub partial_hash: [u8; CHCR_HASH_MAX_DIGEST_SIZE], pub bfr1: [u8; CHCR_HASH_MAX_BLOCK_SIZE_128], pub bfr2: [u8; CHCR_HASH_MAX_BLOCK_SIZE_128] }
#[repr(C)] pub struct chcr_skcipher_req_ctx { pub skb: *mut sk_buff, pub dstsg: *mut scatterlist, pub processed: c_uint, pub last_req_len: c_uint, pub partial_req: c_uint, pub srcsg: *mut scatterlist, pub src_ofst: c_uint, pub dst_ofst: c_uint, pub op: c_uint, pub imm: u16, pub iv: [u8; CHCR_MAX_CRYPTO_IV_LEN], pub init_iv: [u8; CHCR_MAX_CRYPTO_IV_LEN], pub txqidx: u16, pub rxqidx: u16, pub fallback_req: skcipher_request }
#[repr(C)] pub struct chcr_alg_template { pub type_: u32, pub is_registered: u32, pub alg: ManuallyDrop<chcr_alg_template_alg> }
#[repr(C)] pub union chcr_alg_template_alg { pub skcipher: ManuallyDrop<skcipher_alg>, pub hash: ManuallyDrop<ahash_alg>, pub aead: ManuallyDrop<aead_alg> }

pub type create_wr_t = Option<unsafe extern "C" fn(req: *mut aead_request, qid: c_ushort, size: c_int) -> *mut sk_buff>;

extern "C" {
    pub fn chcr_verify_tag(req: *mut aead_request, input: *mut u8, err: *mut c_int);
    pub fn chcr_aead_dma_map(dev: *mut device, req: *mut aead_request, op_type: c_ushort) -> c_int;
    pub fn chcr_aead_dma_unmap(dev: *mut device, req: *mut aead_request, op_type: c_ushort);
    pub fn chcr_add_aead_dst_ent(req: *mut aead_request, phys_cpl: *mut cpl_rx_phys_dsgl, qid: c_ushort);
    pub fn chcr_add_aead_src_ent(req: *mut aead_request, ulptx: *mut ulptx_sgl);
    pub fn chcr_add_cipher_src_ent(req: *mut skcipher_request, ulptx: *mut c_void, wrparam: *mut cipher_wr_param);
    pub fn chcr_cipher_dma_map(dev: *mut device, req: *mut skcipher_request) -> c_int;
    pub fn chcr_cipher_dma_unmap(dev: *mut device, req: *mut skcipher_request);
    pub fn chcr_add_cipher_dst_ent(req: *mut skcipher_request, phys_cpl: *mut cpl_rx_phys_dsgl, wrparam: *mut cipher_wr_param, qid: c_ushort);
    pub fn chcr_add_hash_src_ent(req: *mut ahash_request, ulptx: *mut ulptx_sgl, param: *mut hash_wr_param);
    pub fn chcr_hash_dma_map(dev: *mut device, req: *mut ahash_request) -> c_int;
    pub fn chcr_hash_dma_unmap(dev: *mut device, req: *mut ahash_request);
    pub fn chcr_aead_common_exit(req: *mut aead_request);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
