/* SPDX-License-Identifier: GPL-2.0 */
/*
 * sun8i-ce.h - hardware cryptographic offloader for
 * Allwinner H3/A64/H5/H2+/H6 SoC
 *
 * Copyright (C) 2016-2019 Corentin LABBE <clabbe.montjoie@gmail.com>
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

/* CE Registers */
pub const CE_TDQ: u32 = 0x00;
pub const CE_CTR: u32 = 0x04;
pub const CE_ICR: u32 = 0x08;
pub const CE_ISR: u32 = 0x0C;
pub const CE_TLR: u32 = 0x10;
pub const CE_TSR: u32 = 0x14;
pub const CE_ESR: u32 = 0x18;
pub const CE_CSSGR: u32 = 0x1C;
pub const CE_CDSGR: u32 = 0x20;
pub const CE_CSAR: u32 = 0x24;
pub const CE_CDAR: u32 = 0x28;
pub const CE_TPR: u32 = 0x2C;

pub const CE_ENCRYPTION: u32 = 0;
pub const CE_DECRYPTION: u32 = 1 << 8;
pub const CE_COMM_INT: u32 = 1 << 31;
pub const CE_AES_128BITS: u32 = 0;
pub const CE_AES_192BITS: u32 = 1;
pub const CE_AES_256BITS: u32 = 2;
pub const CE_OP_ECB: u32 = 0;
pub const CE_OP_CBC: u32 = 1 << 8;
pub const CE_ALG_AES: u32 = 0;
pub const CE_ALG_DES: u32 = 1;
pub const CE_ALG_3DES: u32 = 2;
pub const CE_ALG_MD5: u32 = 16;
pub const CE_ALG_SHA1: u32 = 17;
pub const CE_ALG_SHA224: u32 = 18;
pub const CE_ALG_SHA256: u32 = 19;
pub const CE_ALG_SHA384: u32 = 20;
pub const CE_ALG_SHA512: u32 = 21;
pub const CE_ALG_TRNG: u32 = 48;
pub const CE_ALG_TRNG_V2: u32 = 0x1c;
pub const CE_ID_NOTSUPP: u32 = 0xFF;
pub const CE_ID_CIPHER_AES: usize = 0;
pub const CE_ID_CIPHER_DES: usize = 1;
pub const CE_ID_CIPHER_DES3: usize = 2;
pub const CE_ID_CIPHER_MAX: usize = 3;
pub const CE_ID_HASH_MD5: usize = 0;
pub const CE_ID_HASH_SHA1: usize = 1;
pub const CE_ID_HASH_SHA224: usize = 2;
pub const CE_ID_HASH_SHA256: usize = 3;
pub const CE_ID_HASH_SHA384: usize = 4;
pub const CE_ID_HASH_SHA512: usize = 5;
pub const CE_ID_HASH_MAX: usize = 6;
pub const CE_ID_OP_ECB: usize = 0;
pub const CE_ID_OP_CBC: usize = 1;
pub const CE_ID_OP_MAX: usize = 2;
pub const CE_ERR_ALGO_NOTSUP: u32 = 1 << 0;
pub const CE_ERR_DATALEN: u32 = 1 << 1;
pub const CE_ERR_KEYSRAM: u32 = 1 << 2;
pub const CE_ERR_ADDR_INVALID: u32 = 1 << 5;
pub const CE_ERR_KEYLADDER: u32 = 1 << 6;
pub const ESR_H3: i32 = 0;
pub const ESR_A64: i32 = 1;
pub const ESR_R40: i32 = 2;
pub const ESR_H5: i32 = 3;
pub const ESR_H6: i32 = 4;
pub const ESR_D1: i32 = 5;
pub const CE_DIE_ID_SHIFT: u32 = 16;
pub const CE_DIE_ID_MASK: u32 = 0x07;
pub const MAX_SG: usize = 8;
pub const CE_MAX_CLOCKS: usize = 4;
pub const CE_DMA_TIMEOUT_MS: u32 = 3000;
pub const MAXFLOW: usize = 4;
pub const CE_MAX_HASH_DIGEST_SIZE: usize = SHA512_DIGEST_SIZE;
pub const CE_MAX_HASH_BLOCK_SIZE: usize = SHA512_BLOCK_SIZE;

#[repr(C)]
pub struct ce_clock {
    pub name: *const core::ffi::c_char,
    pub freq: c_ulong,
    pub max_freq: c_ulong,
}

#[repr(C)]
pub struct ce_variant {
    pub alg_cipher: [c_char; CE_ID_CIPHER_MAX],
    pub alg_hash: [c_char; CE_ID_HASH_MAX],
    pub op_mode: [u32; CE_ID_OP_MAX],
    pub cipher_t_dlen_in_bytes: bool,
    pub hash_t_dlen_in_bits: bool,
    pub trng_t_dlen_in_bytes: bool,
    pub needs_word_addresses: bool,
    pub ce_clks: [ce_clock; CE_MAX_CLOCKS],
    pub esr: i32,
    pub trng: u8,
}

#[repr(C, packed)]
pub struct sginfo { pub addr: __le32, pub len: __le32 }

#[repr(C, packed, align(8))]
pub struct ce_task {
    pub t_id: __le32, pub t_common_ctl: __le32, pub t_sym_ctl: __le32,
    pub t_asym_ctl: __le32, pub t_key: __le32, pub t_iv: __le32,
    pub t_ctr: __le32, pub t_dlen: __le32,
    pub t_src: [sginfo; MAX_SG], pub t_dst: [sginfo; MAX_SG],
    pub next: __le32, pub reserved: [__le32; 3],
}

#[repr(C)]
pub struct sun8i_ce_flow {
    pub engine: *mut crypto_engine, pub complete: completion, pub status: i32,
    pub t_phy: dma_addr_t, pub tl: *mut ce_task,
    #[cfg(CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG)] pub stat_req: c_ulong,
}

#[repr(C)]
pub struct sun8i_ce_dev {
    pub base: *mut core::ffi::c_void,
    pub ceclks: [*mut clk; CE_MAX_CLOCKS],
    pub reset: *mut reset_control, pub dev: *mut device,
    pub mlock: mutex, pub rnglock: mutex, pub chanlist: *mut sun8i_ce_flow,
    pub flow: atomic_t, pub variant: *const ce_variant,
    #[cfg(CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG)] pub dbgfs_dir: *mut dentry,
    #[cfg(CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG)] pub dbgfs_stats: *mut dentry,
    #[cfg(CONFIG_CRYPTO_DEV_SUN8I_CE_TRNG)] pub trng: hwrng,
    #[cfg(all(CONFIG_CRYPTO_DEV_SUN8I_CE_TRNG, CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG))]
    pub hwrng_stat_req: c_ulong,
    #[cfg(all(CONFIG_CRYPTO_DEV_SUN8I_CE_TRNG, CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG))]
    pub hwrng_stat_bytes: c_ulong,
}

#[inline]
pub unsafe fn desc_addr_val(dev: *mut sun8i_ce_dev, addr: dma_addr_t) -> u32 {
    if (*(*dev).variant).needs_word_addresses { (addr / 4) as u32 } else { addr as u32 }
}

#[inline]
pub unsafe fn desc_addr_val_le32(dev: *mut sun8i_ce_dev, addr: dma_addr_t) -> __le32 {
    cpu_to_le32(desc_addr_val(dev, addr))
}

#[repr(C)]
pub struct sun8i_cipher_req_ctx {
    pub op_dir: u32, pub flow: i32, pub nr_sgs: i32, pub nr_sgd: i32,
    pub addr_iv: dma_addr_t, pub addr_key: dma_addr_t,
    pub bounce_iv: [u8; AES_BLOCK_SIZE], pub backup_iv: [u8; AES_BLOCK_SIZE],
    pub fallback_req: skcipher_request,
}

#[repr(C)]
pub struct sun8i_cipher_tfm_ctx {
    pub key: *mut u32, pub keylen: u32, pub ce: *mut sun8i_ce_dev,
    pub fallback_tfm: *mut crypto_skcipher,
}

#[repr(C)]
pub struct sun8i_ce_hash_tfm_ctx { pub ce: *mut sun8i_ce_dev, pub fallback_tfm: *mut crypto_ahash }

#[repr(C)]
pub struct sun8i_ce_hash_reqctx {
    pub flow: i32, pub nr_sgs: i32, pub result_len: usize, pub pad_len: usize,
    pub addr_res: dma_addr_t, pub addr_pad: dma_addr_t,
    pub result: [u8; CE_MAX_HASH_DIGEST_SIZE], pub pad: [u8; 2 * CE_MAX_HASH_BLOCK_SIZE],
    pub fallback_req: ahash_request,
}

#[repr(C)]
pub union sun8i_ce_alg_template_alg { pub skcipher: skcipher_engine_alg, pub hash: ahash_engine_alg }
#[repr(C)]
pub struct sun8i_ce_alg_template {
    pub type_: u32, pub ce_algo_id: u32, pub ce_blockmode: u32, pub ce: *mut sun8i_ce_dev,
    pub alg: sun8i_ce_alg_template_alg, pub stat_req: c_ulong, pub stat_fb: c_ulong,
    pub stat_bytes: c_ulong, pub stat_fb_maxsg: c_ulong, pub stat_fb_leniv: c_ulong,
    pub stat_fb_len0: c_ulong, pub stat_fb_mod16: c_ulong, pub stat_fb_srcali: c_ulong,
    pub stat_fb_srclen: c_ulong, pub stat_fb_dstali: c_ulong, pub stat_fb_dstlen: c_ulong,
    pub fbname: [c_char; CRYPTO_MAX_ALG_NAME],
}

extern "C" {
    pub fn sun8i_ce_aes_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: c_uint) -> i32;
    pub fn sun8i_ce_des3_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: c_uint) -> i32;
    pub fn sun8i_ce_cipher_init(tfm: *mut crypto_tfm) -> i32;
    pub fn sun8i_ce_cipher_exit(tfm: *mut crypto_tfm);
    pub fn sun8i_ce_cipher_do_one(engine: *mut crypto_engine, areq: *mut core::ffi::c_void) -> i32;
    pub fn sun8i_ce_skdecrypt(areq: *mut skcipher_request) -> i32;
    pub fn sun8i_ce_skencrypt(areq: *mut skcipher_request) -> i32;
    pub fn sun8i_ce_get_engine_number(ce: *mut sun8i_ce_dev) -> i32;
    pub fn sun8i_ce_run_task(ce: *mut sun8i_ce_dev, flow: i32, name: *const c_char) -> i32;
    pub fn sun8i_ce_hash_init_tfm(tfm: *mut crypto_ahash) -> i32;
    pub fn sun8i_ce_hash_exit_tfm(tfm: *mut crypto_ahash);
    pub fn sun8i_ce_hash_init(areq: *mut ahash_request) -> i32;
    pub fn sun8i_ce_hash_export(areq: *mut ahash_request, out: *mut core::ffi::c_void) -> i32;
    pub fn sun8i_ce_hash_import(areq: *mut ahash_request, input: *const core::ffi::c_void) -> i32;
    pub fn sun8i_ce_hash_final(areq: *mut ahash_request) -> i32;
    pub fn sun8i_ce_hash_update(areq: *mut ahash_request) -> i32;
    pub fn sun8i_ce_hash_finup(areq: *mut ahash_request) -> i32;
    pub fn sun8i_ce_hash_digest(areq: *mut ahash_request) -> i32;
    pub fn sun8i_ce_hash_run(engine: *mut crypto_engine, breq: *mut core::ffi::c_void) -> i32;
    pub fn sun8i_ce_hwrng_register(ce: *mut sun8i_ce_dev) -> i32;
    pub fn sun8i_ce_hwrng_unregister(ce: *mut sun8i_ce_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
