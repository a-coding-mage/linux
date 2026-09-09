/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * sun4i-ss.h - hardware cryptographic accelerator for Allwinner A20 SoC
 *
 * Copyright (C) 2013-2015 Corentin LABBE <clabbe.montjoie@gmail.com>
 *
 * Support AES cipher with 128,192,256 bits keysize.
 * Support MD5 and SHA1 hash algorithms.
 * Support DES and 3DES
 *
 * You could find the datasheet in Documentation/arch/arm/sunxi.rst
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

pub const SS_CTL: u32 = 0x00;
pub const SS_KEY0: u32 = 0x04;
pub const SS_KEY1: u32 = 0x08;
pub const SS_KEY2: u32 = 0x0C;
pub const SS_KEY3: u32 = 0x10;
pub const SS_KEY4: u32 = 0x14;
pub const SS_KEY5: u32 = 0x18;
pub const SS_KEY6: u32 = 0x1C;
pub const SS_KEY7: u32 = 0x20;

pub const SS_IV0: u32 = 0x24;
pub const SS_IV1: u32 = 0x28;
pub const SS_IV2: u32 = 0x2C;
pub const SS_IV3: u32 = 0x30;

pub const SS_FCSR: u32 = 0x44;

pub const SS_MD0: u32 = 0x4C;
pub const SS_MD1: u32 = 0x50;
pub const SS_MD2: u32 = 0x54;
pub const SS_MD3: u32 = 0x58;
pub const SS_MD4: u32 = 0x5C;

pub const SS_RXFIFO: u32 = 0x200;
pub const SS_TXFIFO: u32 = 0x204;

pub const SS_IV_ARBITRARY: u32 = 1 << 14;
pub const SS_ECB: u32 = 0 << 12;
pub const SS_CBC: u32 = 1 << 12;
pub const SS_CTS: u32 = 3 << 12;
pub const SS_CNT_16BITS: u32 = 0 << 10;
pub const SS_CNT_32BITS: u32 = 1 << 10;
pub const SS_CNT_64BITS: u32 = 2 << 10;
pub const SS_AES_128BITS: u32 = 0 << 8;
pub const SS_AES_192BITS: u32 = 1 << 8;
pub const SS_AES_256BITS: u32 = 2 << 8;
pub const SS_ENCRYPTION: u32 = 0 << 7;
pub const SS_DECRYPTION: u32 = 1 << 7;
pub const SS_OP_AES: u32 = 0 << 4;
pub const SS_OP_DES: u32 = 1 << 4;
pub const SS_OP_3DES: u32 = 2 << 4;
pub const SS_OP_SHA1: u32 = 3 << 4;
pub const SS_OP_MD5: u32 = 4 << 4;
pub const SS_DATA_END: u32 = 1 << 2;
pub const SS_DISABLED: u32 = 0 << 0;
pub const SS_ENABLED: u32 = 1 << 0;

pub const SS_RXFIFO_FREE: u32 = 1 << 30;
#[inline]
pub const fn ss_rxfifo_spaces(val: u32) -> u32 { (val >> 24) & 0x3f }
pub const SS_TXFIFO_AVAILABLE: u32 = 1 << 22;
#[inline]
pub const fn ss_txfifo_spaces(val: u32) -> u32 { (val >> 16) & 0x3f }

pub const SS_RX_MAX: usize = 32;
pub const SS_RX_DEFAULT: usize = SS_RX_MAX;
pub const SS_TX_MAX: usize = 33;
pub const SS_RXFIFO_EMP_INT_PENDING: u32 = 1 << 10;
pub const SS_TXFIFO_AVA_INT_PENDING: u32 = 1 << 8;
pub const SS_RXFIFO_EMP_INT_ENABLE: u32 = 1 << 2;
pub const SS_TXFIFO_AVA_INT_ENABLE: u32 = 1 << 0;

#[repr(C)]
pub struct ss_variant {
    pub sha1_in_be: bool,
}

#[repr(C)]
pub struct sun4i_ss_ctx {
    pub variant: *const ss_variant,
    pub base: *mut c_void,
    pub irq: c_int,
    pub busclk: *mut clk,
    pub ssclk: *mut clk,
    pub reset: *mut reset_control,
    pub dev: *mut device,
    pub res: *mut resource,
    pub buf: [c_char; 4 * SS_RX_MAX],
    pub bufo: [c_char; 4 * SS_TX_MAX],
    pub slock: spinlock_t,
    pub dbgfs_dir: *mut dentry,
    pub dbgfs_stats: *mut dentry,
}

#[repr(C)]
pub struct sun4i_ss_alg_template {
    pub type_: u32,
    pub mode: u32,
    pub alg: sun4i_ss_alg_template_alg,
    pub ss: *mut sun4i_ss_ctx,
    pub stat_req: c_ulong,
    pub stat_fb: c_ulong,
    pub stat_bytes: c_ulong,
    pub stat_opti: c_ulong,
}

#[repr(C)]
pub union sun4i_ss_alg_template_alg {
    pub crypto: skcipher_alg,
    pub hash: ahash_alg,
}

#[repr(C)]
pub struct sun4i_tfm_ctx {
    pub key: [u32; AES_MAX_KEY_SIZE / 4],
    pub keylen: u32,
    pub keymode: u32,
    pub ss: *mut sun4i_ss_ctx,
    pub fallback_tfm: *mut crypto_skcipher,
}

#[repr(C)]
pub struct sun4i_cipher_req_ctx {
    pub mode: u32,
    pub backup_iv: [u8; AES_BLOCK_SIZE],
    pub fallback_req: skcipher_request,
}

#[repr(C)]
pub struct sun4i_req_ctx {
    pub mode: u32,
    pub byte_count: u64,
    pub hash: [u32; 5],
    pub buf: [c_char; 64],
    pub len: u32,
    pub flags: c_int,
}

extern "C" {
    pub fn sun4i_hash_crainit(tfm: *mut crypto_tfm) -> c_int;
    pub fn sun4i_hash_craexit(tfm: *mut crypto_tfm);
    pub fn sun4i_hash_init(areq: *mut ahash_request) -> c_int;
    pub fn sun4i_hash_update(areq: *mut ahash_request) -> c_int;
    pub fn sun4i_hash_final(areq: *mut ahash_request) -> c_int;
    pub fn sun4i_hash_finup(areq: *mut ahash_request) -> c_int;
    pub fn sun4i_hash_digest(areq: *mut ahash_request) -> c_int;
    pub fn sun4i_hash_export_md5(areq: *mut ahash_request, out: *mut c_void) -> c_int;
    pub fn sun4i_hash_import_md5(areq: *mut ahash_request, input: *const c_void) -> c_int;
    pub fn sun4i_hash_export_sha1(areq: *mut ahash_request, out: *mut c_void) -> c_int;
    pub fn sun4i_hash_import_sha1(areq: *mut ahash_request, input: *const c_void) -> c_int;

    pub fn sun4i_ss_cbc_aes_encrypt(areq: *mut skcipher_request) -> c_int;
    pub fn sun4i_ss_cbc_aes_decrypt(areq: *mut skcipher_request) -> c_int;
    pub fn sun4i_ss_ecb_aes_encrypt(areq: *mut skcipher_request) -> c_int;
    pub fn sun4i_ss_ecb_aes_decrypt(areq: *mut skcipher_request) -> c_int;
    pub fn sun4i_ss_cbc_des_encrypt(areq: *mut skcipher_request) -> c_int;
    pub fn sun4i_ss_cbc_des_decrypt(areq: *mut skcipher_request) -> c_int;
    pub fn sun4i_ss_ecb_des_encrypt(areq: *mut skcipher_request) -> c_int;
    pub fn sun4i_ss_ecb_des_decrypt(areq: *mut skcipher_request) -> c_int;
    pub fn sun4i_ss_cbc_des3_encrypt(areq: *mut skcipher_request) -> c_int;
    pub fn sun4i_ss_cbc_des3_decrypt(areq: *mut skcipher_request) -> c_int;
    pub fn sun4i_ss_ecb_des3_encrypt(areq: *mut skcipher_request) -> c_int;
    pub fn sun4i_ss_ecb_des3_decrypt(areq: *mut skcipher_request) -> c_int;
    pub fn sun4i_ss_cipher_init(tfm: *mut crypto_tfm) -> c_int;
    pub fn sun4i_ss_cipher_exit(tfm: *mut crypto_tfm);
    pub fn sun4i_ss_aes_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: u32) -> c_int;
    pub fn sun4i_ss_des_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: u32) -> c_int;
    pub fn sun4i_ss_des3_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: u32) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
