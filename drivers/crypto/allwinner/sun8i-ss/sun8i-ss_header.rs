/* SPDX-License-Identifier: GPL-2.0 */
/*
 * sun8i-ss.h - hardware cryptographic offloader for
 * Allwinner A80/A83T SoC
 *
 * Copyright (C) 2016-2019 Corentin LABBE <clabbe.montjoie@gmail.com>
 */

// Types supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

pub const SS_START: u32 = 1;

pub const SS_ENCRYPTION: u32 = 0;
pub const SS_DECRYPTION: u32 = 1 << 6;

pub const SS_ALG_AES: u32 = 0;
pub const SS_ALG_DES: u32 = 1 << 2;
pub const SS_ALG_3DES: u32 = 2 << 2;
pub const SS_ALG_MD5: u32 = 3 << 2;
pub const SS_ALG_SHA1: u32 = 6 << 2;
pub const SS_ALG_SHA224: u32 = 7 << 2;
pub const SS_ALG_SHA256: u32 = 8 << 2;

pub const SS_CTL_REG: u32 = 0x00;
pub const SS_INT_CTL_REG: u32 = 0x04;
pub const SS_INT_STA_REG: u32 = 0x08;
pub const SS_KEY_ADR_REG: u32 = 0x10;
pub const SS_IV_ADR_REG: u32 = 0x18;
pub const SS_SRC_ADR_REG: u32 = 0x20;
pub const SS_DST_ADR_REG: u32 = 0x28;
pub const SS_LEN_ADR_REG: u32 = 0x30;

pub const SS_ID_NOTSUPP: u32 = 0xFF;

pub const SS_ID_CIPHER_AES: usize = 0;
pub const SS_ID_CIPHER_DES: usize = 1;
pub const SS_ID_CIPHER_DES3: usize = 2;
pub const SS_ID_CIPHER_MAX: usize = 3;

pub const SS_ID_OP_ECB: usize = 0;
pub const SS_ID_OP_CBC: usize = 1;
pub const SS_ID_OP_MAX: usize = 2;

pub const SS_AES_128BITS: u32 = 0;
pub const SS_AES_192BITS: u32 = 1;
pub const SS_AES_256BITS: u32 = 2;

pub const SS_OP_ECB: u32 = 0;
pub const SS_OP_CBC: u32 = 1 << 13;

pub const SS_ID_HASH_MD5: usize = 0;
pub const SS_ID_HASH_SHA1: usize = 1;
pub const SS_ID_HASH_SHA224: usize = 2;
pub const SS_ID_HASH_SHA256: usize = 3;
pub const SS_ID_HASH_MAX: usize = 4;

pub const SS_FLOW0: u32 = 1 << 30;
pub const SS_FLOW1: u32 = 1 << 31;
pub const MAX_SG: usize = 8;
pub const MAXFLOW: usize = 2;
pub const SS_MAX_CLOCKS: usize = 2;
pub const SS_DIE_ID_SHIFT: u32 = 20;
pub const SS_DIE_ID_MASK: u32 = 0x07;
pub const MAX_PAD_SIZE: usize = 4096;

#[repr(C)]
pub struct ss_clock {
    pub name: *const core::ffi::c_char,
    pub freq: core::ffi::c_ulong,
    pub max_freq: core::ffi::c_ulong,
}

#[repr(C)]
pub struct ss_variant {
    pub alg_cipher: [core::ffi::c_char; SS_ID_CIPHER_MAX],
    pub alg_hash: [core::ffi::c_char; SS_ID_HASH_MAX],
    pub op_mode: [u32; SS_ID_OP_MAX],
    pub ss_clks: [ss_clock; SS_MAX_CLOCKS],
}

#[repr(C)]
pub struct sginfo {
    pub addr: u32,
    pub len: u32,
}

#[repr(C)]
pub struct sun8i_ss_flow {
    pub engine: *mut crypto_engine,
    pub complete: completion,
    pub status: core::ffi::c_int,
    pub iv: [*mut u8; MAX_SG],
    pub biv: *mut u8,
    pub pad: *mut core::ffi::c_void,
    pub result: *mut core::ffi::c_void,
    #[cfg(feature = "CONFIG_CRYPTO_DEV_SUN8I_SS_DEBUG")]
    pub stat_req: libc::c_ulong,
}

#[repr(C)]
pub struct sun8i_ss_dev {
    pub base: *mut core::ffi::c_void,
    pub ssclks: [*mut clk; SS_MAX_CLOCKS],
    pub reset: *mut reset_control,
    pub dev: *mut device,
    pub mlock: mutex,
    pub flows: *mut sun8i_ss_flow,
    pub flow: atomic_t,
    pub variant: *const ss_variant,
    #[cfg(feature = "CONFIG_CRYPTO_DEV_SUN8I_SS_DEBUG")]
    pub dbgfs_dir: *mut dentry,
    pub dbgfs_stats: *mut dentry,
}

#[repr(C)]
pub struct sun8i_cipher_req_ctx {
    pub t_src: [sginfo; MAX_SG],
    pub t_dst: [sginfo; MAX_SG],
    pub p_key: u32,
    pub p_iv: [u32; MAX_SG],
    pub niv: core::ffi::c_int,
    pub method: u32,
    pub op_mode: u32,
    pub op_dir: u32,
    pub flow: core::ffi::c_int,
    pub ivlen: core::ffi::c_uint,
    pub keylen: core::ffi::c_uint,
    pub fallback_req: skcipher_request, // keep at the end
}

#[repr(C)]
pub struct sun8i_cipher_tfm_ctx {
    pub key: *mut u32,
    pub keylen: u32,
    pub ss: *mut sun8i_ss_dev,
    pub fallback_tfm: *mut crypto_skcipher,
}

#[repr(C)]
pub struct sun8i_ss_hash_tfm_ctx {
    pub fallback_tfm: *mut crypto_ahash,
    pub ss: *mut sun8i_ss_dev,
    pub ipad: *mut u8,
    pub opad: *mut u8,
    pub key: [u8; SHA256_BLOCK_SIZE],
    pub keylen: core::ffi::c_int,
}

#[repr(C)]
pub struct sun8i_ss_hash_reqctx {
    pub t_src: [sginfo; MAX_SG],
    pub t_dst: [sginfo; MAX_SG],
    pub method: u32,
    pub flow: core::ffi::c_int,
    // Must be last as it ends in a flexible-array member.
    pub fallback_req: ahash_request,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
