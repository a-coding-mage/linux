/* SPDX-License-Identifier: GPL-2.0 */
// Translated from jh7110-cryp.h. Kernel dependencies are supplied externally.

pub const STARFIVE_ALG_CR_OFFSET: u32 = 0x0;
pub const STARFIVE_ALG_FIFO_OFFSET: u32 = 0x4;
pub const STARFIVE_IE_MASK_OFFSET: u32 = 0x8;
pub const STARFIVE_IE_FLAG_OFFSET: u32 = 0xc;
pub const STARFIVE_DMA_IN_LEN_OFFSET: u32 = 0x10;
pub const STARFIVE_DMA_OUT_LEN_OFFSET: u32 = 0x14;

pub const STARFIVE_IE_MASK_AES_DONE: u32 = 0x1;
pub const STARFIVE_IE_MASK_HASH_DONE: u32 = 0x4;
pub const STARFIVE_IE_MASK_PKA_DONE: u32 = 0x8;
pub const STARFIVE_IE_FLAG_AES_DONE: u32 = 0x1;
pub const STARFIVE_IE_FLAG_HASH_DONE: u32 = 0x4;
pub const STARFIVE_IE_FLAG_PKA_DONE: u32 = 0x8;

pub const STARFIVE_MSG_BUFFER_SIZE: usize = 16 * 1024;
pub const MAX_KEY_SIZE: usize = 128; // SHA512_BLOCK_SIZE
pub const STARFIVE_AES_IV_LEN: usize = 16;
pub const STARFIVE_AES_CTR_LEN: usize = 16;
pub const STARFIVE_RSA_MAX_KEYSZ: usize = 256;

pub const STARFIVE_AES_KEYMODE_128: u32 = 0x0;
pub const STARFIVE_AES_KEYMODE_192: u32 = 0x1;
pub const STARFIVE_AES_KEYMODE_256: u32 = 0x2;
pub const STARFIVE_AES_BUSY: u32 = 1 << 3;
pub const STARFIVE_AES_KEY_DONE: u32 = 1 << 5;
pub const STARFIVE_AES_CCM_START: u32 = 1 << 8;
pub const STARFIVE_AES_MODE_ECB: u32 = 0x0;
pub const STARFIVE_AES_MODE_CBC: u32 = 0x1;
pub const STARFIVE_AES_MODE_CTR: u32 = 0x4;
pub const STARFIVE_AES_MODE_CCM: u32 = 0x5;
pub const STARFIVE_AES_MODE_GCM: u32 = 0x6;
pub const STARFIVE_AES_GCM_START: u32 = 1 << 12;
pub const STARFIVE_AES_GCM_DONE: u32 = 1 << 13;
pub const STARFIVE_AES_MODE_XFB_1: u32 = 0x0;
pub const STARFIVE_AES_MODE_XFB_128: u32 = 0x5;

pub const STARFIVE_HASH_SM3: u32 = 0x0;
pub const STARFIVE_HASH_SHA224: u32 = 0x3;
pub const STARFIVE_HASH_SHA256: u32 = 0x4;
pub const STARFIVE_HASH_SHA384: u32 = 0x5;
pub const STARFIVE_HASH_SHA512: u32 = 0x6;
pub const STARFIVE_HASH_MODE_MASK: u32 = 0x7;
pub const STARFIVE_HASH_HMAC_FLAGS: u32 = 0x800;
pub const STARFIVE_HASH_KEY_DONE: u32 = 1 << 13;
pub const STARFIVE_HASH_HMAC_DONE: u32 = 1 << 15;
pub const STARFIVE_HASH_BUSY: u32 = 1 << 16;
pub const STARFIVE_PKA_DONE: u32 = 1 << 0;

#[repr(C)]
pub union starfive_aes_csr { pub v: u32, pub bits: u32 }
#[repr(C)]
pub union starfive_hash_csr { pub v: u32, pub bits: u32 }
#[repr(C)]
pub union starfive_pka_cacr { pub v: u32, pub bits: u32 }
#[repr(C)]
pub union starfive_pka_casr { pub v: u32, pub bits: u32 }

#[repr(C)]
pub struct starfive_rsa_key {
    pub n: *mut u8, pub e: *mut u8, pub d: *mut u8,
    pub e_bitlen: i32, pub d_bitlen: i32, pub bitlen: i32, pub key_sz: usize,
}

#[repr(C)]
pub union starfive_alg_cr { pub v: u32, pub bits: u32 }

// External kernel types referenced by this header.
pub enum crypto_akcipher {}
pub enum crypto_ahash {}
pub enum crypto_aead {}
pub enum crypto_skcipher {}
pub enum list_head {}
pub enum device {}
pub enum clk {}
pub enum reset_control {}
pub enum dma_chan {}
pub enum dma_slave_config {}
pub enum crypto_engine {}
pub enum completion {}
pub enum ahash_request {}
pub enum aead_request {}
pub enum skcipher_request {}
pub enum scatterlist {}

#[repr(C)]
pub struct starfive_cryp_ctx {
    pub cryp: *mut starfive_cryp_dev,
    pub rctx: *mut starfive_cryp_request_ctx,
    pub hash_mode: u32,
    pub key: [u8; MAX_KEY_SIZE], pub keylen: i32, pub is_hmac: bool,
    pub rsa_key: starfive_rsa_key,
    pub akcipher_fbk: *mut crypto_akcipher, pub ahash_fbk: *mut crypto_ahash,
    pub aead_fbk: *mut crypto_aead, pub skcipher_fbk: *mut crypto_skcipher,
}

#[repr(C)]
pub union starfive_cryp_req_union {
    pub hreq: *mut ahash_request, pub areq: *mut aead_request,
    pub sreq: *mut skcipher_request,
}

#[repr(C)]
pub struct starfive_cryp_dev {
    pub list: *mut list_head, pub dev: *mut device, pub hclk: *mut clk,
    pub ahb: *mut clk, pub rst: *mut reset_control, pub base: *mut core::ffi::c_void,
    pub phys_base: usize, pub dma_maxburst: u32, pub tx: *mut dma_chan,
    pub rx: *mut dma_chan, pub cfg_in: dma_slave_config, pub cfg_out: dma_slave_config,
    pub engine: *mut crypto_engine, pub dma_done: completion, pub assoclen: usize,
    pub total_in: usize, pub total_out: usize, pub tag_in: [u32; 4], pub tag_out: [u32; 4],
    pub authsize: u32, pub flags: usize, pub err: i32, pub side_chan: bool,
    pub alg_cr: starfive_alg_cr, pub req: starfive_cryp_req_union,
}

#[repr(C)]
pub union starfive_cryp_req_csr {
    pub hash: starfive_hash_csr, pub pka: starfive_pka_cacr, pub aes: starfive_aes_csr,
}

#[repr(C)]
pub struct starfive_cryp_request_ctx {
    pub csr: starfive_cryp_req_csr, pub in_sg: *mut scatterlist,
    pub out_sg: *mut scatterlist, pub total: usize, pub blksize: u32,
    pub digsize: u32, pub in_sg_len: usize, pub adata: *mut u8,
    pub rsa_data: [u8; STARFIVE_RSA_MAX_KEYSZ],
    // Must be last as it ends in a flexible-array member.
    pub ahash_fbk_req: ahash_request,
}

extern "C" {
    pub fn starfive_cryp_find_dev(ctx: *mut starfive_cryp_ctx) -> *mut starfive_cryp_dev;
    pub fn starfive_hash_register_algs() -> i32;
    pub fn starfive_hash_unregister_algs();
    pub fn starfive_rsa_register_algs() -> i32;
    pub fn starfive_rsa_unregister_algs();
    pub fn starfive_aes_register_algs() -> i32;
    pub fn starfive_aes_unregister_algs();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
