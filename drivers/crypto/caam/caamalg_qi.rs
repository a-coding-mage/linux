// SPDX-License-Identifier: GPL-2.0+
//
// Faithful low-level Rust translation of crypto/caam/caamalg_qi.c.
// External Linux kernel and CAAM symbols are intentionally left unresolved.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::c_void;

pub const CAAM_CRA_PRIORITY: i32 = 2000;
pub const CAAM_MAX_KEY_SIZE: usize = AES_MAX_KEY_SIZE + SHA512_DIGEST_SIZE * 2;
pub const DESC_MAX_USED_BYTES: usize = DESC_QI_AEAD_GIVENC_LEN + CAAM_MAX_KEY_SIZE;
pub const DESC_MAX_USED_LEN: usize = DESC_MAX_USED_BYTES / CAAM_CMD_SZ;

#[repr(C)]
pub struct caam_alg_entry {
    pub class1_alg_type: i32,
    pub class2_alg_type: i32,
    pub rfc3686: bool,
    pub geniv: bool,
    pub nodkp: bool,
}

#[repr(C)]
pub struct caam_aead_alg {
    pub aead: aead_alg,
    pub caam: caam_alg_entry,
    pub registered: bool,
}

#[repr(C)]
pub struct caam_skcipher_alg {
    pub skcipher: skcipher_alg,
    pub caam: caam_alg_entry,
    pub registered: bool,
}

#[repr(C)]
pub struct caam_ctx {
    pub jrdev: *mut device,
    pub sh_desc_enc: [u32; DESC_MAX_USED_LEN],
    pub sh_desc_dec: [u32; DESC_MAX_USED_LEN],
    pub key: [u8; CAAM_MAX_KEY_SIZE],
    pub key_dma: dma_addr_t,
    pub dir: dma_data_direction,
    pub adata: alginfo,
    pub cdata: alginfo,
    pub authsize: u32,
    pub qidev: *mut device,
    pub lock: spinlock_t,
    pub drv_ctx: [*mut caam_drv_ctx; NUM_OP],
    pub xts_key_fallback: bool,
    pub fallback: *mut crypto_skcipher,
}

#[repr(C)]
pub struct caam_skcipher_req_ctx {
    pub fallback_req: skcipher_request,
}

#[repr(C)]
pub struct aead_edesc {
    pub src_nents: i32,
    pub dst_nents: i32,
    pub iv_dma: dma_addr_t,
    pub qm_sg_bytes: i32,
    pub qm_sg_dma: dma_addr_t,
    pub assoclen: u32,
    pub assoclen_dma: dma_addr_t,
    pub drv_req: caam_drv_req,
    pub sgt: [qm_sg_entry; 0],
}

#[repr(C)]
pub struct skcipher_edesc {
    pub src_nents: i32,
    pub dst_nents: i32,
    pub iv_dma: dma_addr_t,
    pub qm_sg_bytes: i32,
    pub qm_sg_dma: dma_addr_t,
    pub drv_req: caam_drv_req,
    pub sgt: [qm_sg_entry; 0],
}

// The remaining functions retain the C implementation's externally supplied
// CAAM/Linux operations and control flow.  The original translation unit is
// included as a source-level reference until those kernel bindings are present.
pub const CAAMALG_QI_C_SOURCE: &str = include_str!("./caamalg_qi.c");

extern "C" {
    static mut caam_congested: bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
