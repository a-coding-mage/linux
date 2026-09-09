/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2021, Linaro Limited. All rights reserved.
 */

// Translated from aead.h. Dependencies supplied by common.h and core.h remain external.

pub const QCE_MAX_KEY_SIZE: usize = 64;
pub const QCE_CCM4309_SALT_SIZE: usize = 3;

#[repr(C)]
pub struct qce_aead_ctx {
    pub enc_key: [u8; QCE_MAX_KEY_SIZE],
    pub auth_key: [u8; QCE_MAX_KEY_SIZE],
    pub ccm4309_salt: [u8; QCE_CCM4309_SALT_SIZE],
    pub enc_keylen: u32,
    pub auth_keylen: u32,
    pub authsize: u32,
    pub need_fallback: bool,
    pub fallback: *mut crypto_aead,
}

#[repr(C)]
pub struct qce_aead_reqctx {
    pub flags: usize,
    pub iv: *mut u8,
    pub ivsize: u32,
    pub src_nents: i32,
    pub dst_nents: i32,
    pub result_sg: scatterlist,
    pub adata_sg: scatterlist,
    pub dst_tbl: sg_table,
    pub src_tbl: sg_table,
    pub dst_sg: *mut scatterlist,
    pub src_sg: *mut scatterlist,
    pub cryptlen: u32,
    pub assoclen: u32,
    pub adata: *mut u8,
    pub ccm_nonce: [u8; QCE_MAX_NONCE],
    pub ccmresult_buf: [u8; QCE_BAM_BURST_SIZE],
    pub ccm_rfc4309_iv: [u8; QCE_MAX_IV_SIZE],
    pub fallback_req: aead_request,
}

pub unsafe fn to_aead_tmpl(tfm: *mut crypto_aead) -> *mut qce_alg_template {
    let alg = crypto_aead_alg(tfm);

    container_of!(alg, qce_alg_template, alg.aead)
}

extern "C" {
    pub static aead_ops: qce_algo_ops;
}

// External types and symbols supplied by common.h and core.h.
extern "C" {
    fn crypto_aead_alg(tfm: *mut crypto_aead) -> *mut aead_alg;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
