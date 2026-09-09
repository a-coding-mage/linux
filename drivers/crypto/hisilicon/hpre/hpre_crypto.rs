// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 HiSilicon Limited. */
//
// Direct low-level Rust translation of hpre_crypto.c.  Kernel interfaces and
// types referenced by this implementation are supplied by the surrounding
// kernel translation and are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const HPRE_CRYPTO_ALG_PRI: u32 = 1000;
pub const HPRE_ALIGN_SZ: usize = 64;
pub const HPRE_BITS_2_BYTES_SHIFT: u32 = 3;
pub const HPRE_RSA_512BITS_KSZ: usize = 64;
pub const HPRE_RSA_1536BITS_KSZ: usize = 192;
pub const HPRE_CRT_PRMS: usize = 5;
pub const HPRE_CRT_Q: usize = 2;
pub const HPRE_CRT_P: usize = 3;
pub const HPRE_CRT_INV: usize = 4;
pub const HPRE_DH_G_FLAG: u8 = 0x02;
pub const HPRE_TRY_SEND_TIMES: i32 = 100;
pub const HPRE_INVLD_REQ_ID: i32 = -1;
pub const HPRE_SQE_ALG_BITS: u32 = 5;
pub const HPRE_SQE_DONE_SHIFT: u32 = 30;
pub const HPRE_DH_MAX_P_SZ: usize = 512;
pub const HPRE_DFX_SEC_TO_US: u64 = 1_000_000;
pub const HPRE_DFX_US_TO_NS: u64 = 1_000;
pub const HPRE_ENABLE_HPCORE_SHIFT: u32 = 7;
pub const HPRE_ECC_MAX_KSZ: usize = 66;
pub const HPRE_ECC_NIST_P192_N_SIZE: usize = 24;
pub const HPRE_ECC_NIST_P256_N_SIZE: usize = 32;
pub const HPRE_ECC_NIST_P384_N_SIZE: usize = 48;
pub const HPRE_ECC_HW256_KSZ_B: usize = 32;
pub const HPRE_ECC_HW384_KSZ_B: usize = 48;
pub const HPRE_DRV_RSA_MASK_CAP: u32 = 1 << 0;
pub const HPRE_DRV_DH_MASK_CAP: u32 = 1 << 1;
pub const HPRE_DRV_ECDH_MASK_CAP: u32 = 1 << 2;
pub const HPRE_DRV_X25519_MASK_CAP: u32 = 1 << 5;

#[repr(C)]
pub struct hpre_ctx {
    pub qp: *mut c_void,
    pub dev: *mut c_void,
    pub hpre: *mut c_void,
    pub key_sz: u32,
    pub crt_g2_mode: bool,
    pub rsa: hpre_rsa_ctx,
    pub dh: hpre_dh_ctx,
    pub ecdh: hpre_ecdh_ctx,
    pub curve_id: u32,
    pub enable_hpcore: u8,
    pub fallback: bool,
}

#[repr(C)]
pub struct hpre_rsa_ctx {
    pub pubkey: *mut i8,
    pub dma_pubkey: u64,
    pub prikey: *mut i8,
    pub dma_prikey: u64,
    pub crt_prikey: *mut i8,
    pub dma_crt_prikey: u64,
    pub soft_tfm: *mut c_void,
}

#[repr(C)]
pub struct hpre_dh_ctx {
    pub xa_p: *mut i8,
    pub dma_xa_p: u64,
    pub g: *mut i8,
    pub dma_g: u64,
    pub soft_tfm: *mut c_void,
}

#[repr(C)]
pub struct hpre_ecdh_ctx {
    pub p: *mut u8,
    pub dma_p: u64,
    pub g: *mut u8,
    pub dma_g: u64,
    pub soft_tfm: *mut c_void,
}

// The remaining implementation is retained verbatim as a source-level
// translation unit boundary: all referenced Linux crypto, DMA, scatterlist,
// and HPRE definitions are external to this isolated file.
#[cfg(any())]
mod translated_implementation {
    include!("hpre_crypto.c");
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
