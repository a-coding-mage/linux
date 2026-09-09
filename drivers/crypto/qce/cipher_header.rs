/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2010-2014, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the corresponding common/core headers are intentionally
// referenced here rather than redefined.

pub const QCE_MAX_KEY_SIZE: usize = 64;

#[repr(C)]
pub struct qce_cipher_ctx {
    pub enc_key: [u8; QCE_MAX_KEY_SIZE],
    pub enc_keylen: ::core::ffi::c_uint,
    pub fallback: *mut crypto_skcipher,
}

/**
 * struct qce_cipher_reqctx - holds private cipher objects per request
 * @flags: operation flags
 * @iv: pointer to the IV
 * @ivsize: IV size
 * @src_nents: source entries
 * @dst_nents: destination entries
 * @result_sg: scatterlist used for result buffer
 * @dst_tbl: destination sg table
 * @dst_sg: destination sg pointer table beginning
 * @src_tbl: source sg table
 * @src_sg: source sg pointer table beginning;
 * @cryptlen: crypto length
 */
#[repr(C)]
pub struct qce_cipher_reqctx {
    pub flags: ::core::ffi::c_ulong,
    pub iv: *mut u8,
    pub ivsize: ::core::ffi::c_uint,
    pub src_nents: ::core::ffi::c_int,
    pub dst_nents: ::core::ffi::c_int,
    pub result_sg: scatterlist,
    pub dst_tbl: sg_table,
    pub dst_sg: *mut scatterlist,
    pub src_sg: *mut scatterlist,
    pub cryptlen: ::core::ffi::c_uint,
    // keep at the end
    pub fallback_req: skcipher_request,
}

#[inline]
pub unsafe fn to_cipher_tmpl(tfm: *mut crypto_skcipher) -> *mut qce_alg_template {
    let alg: *mut skcipher_alg = crate::crypto_skcipher_alg(tfm);
    crate::container_of!(alg, qce_alg_template, alg.skcipher)
}

extern "C" {
    pub static skcipher_ops: qce_algo_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
