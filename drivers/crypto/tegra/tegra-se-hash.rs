// SPDX-License-Identifier: GPL-2.0-only
// SPDX-FileCopyrightText: Copyright (c) 2023 NVIDIA CORPORATION & AFFILIATES. All rights reserved.
/* Crypto driver to handle HASH algorithms using NVIDIA Security Engine. */

#[repr(C)]
pub struct TegraShaCtx {
    pub se: *mut tegra_se,
    pub alg: u32,
    pub fallback: bool,
    pub key_id: u32,
    pub fallback_tfm: *mut crypto_ahash,
}

#[repr(C)]
pub struct TegraShaReqctx {
    pub src_sg: *mut scatterlist,
    pub datbuf: tegra_se_datbuf,
    pub residue: tegra_se_datbuf,
    pub digest: tegra_se_datbuf,
    pub intr_res: tegra_se_datbuf,
    pub alg: u32,
    pub config: u32,
    pub total_len: u32,
    pub blk_size: u32,
    pub task: u32,
    pub key_id: u32,
    pub result: [u32; HASH_RESULT_REG_COUNT],
    pub fallback_req: ahash_request,
}

unsafe fn tegra_sha_fallback_init(req: *mut ahash_request) -> i32 {
    let rctx = ahash_request_ctx(req);
    let tfm = crypto_ahash_reqtfm(req);
    let ctx = crypto_ahash_ctx(tfm);
    ahash_request_set_tfm(&mut (*rctx).fallback_req, (*ctx).fallback_tfm);
    ahash_request_set_callback(&mut (*rctx).fallback_req, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP, (*req).base.complete, (*req).base.data);
    crypto_ahash_init(&mut (*rctx).fallback_req)
}

unsafe fn tegra_sha_fallback_update(req: *mut ahash_request) -> i32 {
    let rctx = ahash_request_ctx(req); let tfm = crypto_ahash_reqtfm(req); let ctx = crypto_ahash_ctx(tfm);
    ahash_request_set_tfm(&mut (*rctx).fallback_req, (*ctx).fallback_tfm);
    ahash_request_set_callback(&mut (*rctx).fallback_req, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP, (*req).base.complete, (*req).base.data);
    ahash_request_set_crypt(&mut (*rctx).fallback_req, (*req).src, core::ptr::null_mut(), (*req).nbytes);
    crypto_ahash_update(&mut (*rctx).fallback_req)
}

unsafe fn tegra_sha_fallback_final(req: *mut ahash_request) -> i32 {
    let rctx = ahash_request_ctx(req); let tfm = crypto_ahash_reqtfm(req); let ctx = crypto_ahash_ctx(tfm);
    ahash_request_set_tfm(&mut (*rctx).fallback_req, (*ctx).fallback_tfm);
    ahash_request_set_callback(&mut (*rctx).fallback_req, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP, (*req).base.complete, (*req).base.data);
    ahash_request_set_crypt(&mut (*rctx).fallback_req, core::ptr::null_mut(), (*req).result, 0);
    crypto_ahash_final(&mut (*rctx).fallback_req)
}

unsafe fn tegra_sha_fallback_finup(req: *mut ahash_request) -> i32 {
    let rctx = ahash_request_ctx(req); let tfm = crypto_ahash_reqtfm(req); let ctx = crypto_ahash_ctx(tfm);
    ahash_request_set_tfm(&mut (*rctx).fallback_req, (*ctx).fallback_tfm);
    ahash_request_set_callback(&mut (*rctx).fallback_req, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP, (*req).base.complete, (*req).base.data);
    ahash_request_set_crypt(&mut (*rctx).fallback_req, (*req).src, (*req).result, (*req).nbytes);
    crypto_ahash_finup(&mut (*rctx).fallback_req)
}

unsafe fn tegra_sha_fallback_digest(req: *mut ahash_request) -> i32 {
    let rctx = ahash_request_ctx(req); let tfm = crypto_ahash_reqtfm(req); let ctx = crypto_ahash_ctx(tfm);
    ahash_request_set_tfm(&mut (*rctx).fallback_req, (*ctx).fallback_tfm);
    ahash_request_set_callback(&mut (*rctx).fallback_req, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP, (*req).base.complete, (*req).base.data);
    ahash_request_set_crypt(&mut (*rctx).fallback_req, (*req).src, (*req).result, (*req).nbytes);
    crypto_ahash_digest(&mut (*rctx).fallback_req)
}

unsafe fn tegra_sha_fallback_import(req: *mut ahash_request, input: *const core::ffi::c_void) -> i32 {
    let rctx = ahash_request_ctx(req); let tfm = crypto_ahash_reqtfm(req); let ctx = crypto_ahash_ctx(tfm);
    ahash_request_set_tfm(&mut (*rctx).fallback_req, (*ctx).fallback_tfm);
    ahash_request_set_callback(&mut (*rctx).fallback_req, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP, (*req).base.complete, (*req).base.data);
    crypto_ahash_import(&mut (*rctx).fallback_req, input)
}

unsafe fn tegra_sha_fallback_export(req: *mut ahash_request, output: *mut core::ffi::c_void) -> i32 {
    let rctx = ahash_request_ctx(req); let tfm = crypto_ahash_reqtfm(req); let ctx = crypto_ahash_ctx(tfm);
    ahash_request_set_tfm(&mut (*rctx).fallback_req, (*ctx).fallback_tfm);
    ahash_request_set_callback(&mut (*rctx).fallback_req, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP, (*req).base.complete, (*req).base.data);
    crypto_ahash_export(&mut (*rctx).fallback_req, output)
}

unsafe fn tegra_sha_get_config(alg: u32) -> i32 {
    let mut cfg = 0;
    match alg {
        SE_ALG_SHA1 => { cfg |= SE_SHA_ENC_ALG_SHA | SE_SHA_ENC_MODE_SHA1; }
        SE_ALG_HMAC_SHA224 | SE_ALG_SHA224 => { if alg == SE_ALG_HMAC_SHA224 { cfg |= SE_SHA_ENC_ALG_HMAC; } cfg |= SE_SHA_ENC_ALG_SHA | SE_SHA_ENC_MODE_SHA224; }
        SE_ALG_HMAC_SHA256 | SE_ALG_SHA256 => { if alg == SE_ALG_HMAC_SHA256 { cfg |= SE_SHA_ENC_ALG_HMAC; } cfg |= SE_SHA_ENC_ALG_SHA | SE_SHA_ENC_MODE_SHA256; }
        SE_ALG_HMAC_SHA384 | SE_ALG_SHA384 => { if alg == SE_ALG_HMAC_SHA384 { cfg |= SE_SHA_ENC_ALG_HMAC; } cfg |= SE_SHA_ENC_ALG_SHA | SE_SHA_ENC_MODE_SHA384; }
        SE_ALG_HMAC_SHA512 | SE_ALG_SHA512 => { if alg == SE_ALG_HMAC_SHA512 { cfg |= SE_SHA_ENC_ALG_HMAC; } cfg |= SE_SHA_ENC_ALG_SHA | SE_SHA_ENC_MODE_SHA512; }
        SE_ALG_SHA3_224 => cfg |= SE_SHA_ENC_ALG_SHA | SE_SHA_ENC_MODE_SHA3_224,
        SE_ALG_SHA3_256 => cfg |= SE_SHA_ENC_ALG_SHA | SE_SHA_ENC_MODE_SHA3_256,
        SE_ALG_SHA3_384 => cfg |= SE_SHA_ENC_ALG_SHA | SE_SHA_ENC_MODE_SHA3_384,
        SE_ALG_SHA3_512 => cfg |= SE_SHA_ENC_ALG_SHA | SE_SHA_ENC_MODE_SHA3_512,
        _ => return -EINVAL,
    }; cfg
}

// The remaining driver operations retain the kernel ABI and are declared here;
// their implementations are supplied by the surrounding translated driver.
extern "C" {
    pub fn tegra_sha_do_one_req(engine: *mut crypto_engine, areq: *mut core::ffi::c_void) -> i32;
    pub fn tegra_sha_init(req: *mut ahash_request) -> i32;
    pub fn tegra_sha_update(req: *mut ahash_request) -> i32;
    pub fn tegra_sha_final(req: *mut ahash_request) -> i32;
    pub fn tegra_sha_finup(req: *mut ahash_request) -> i32;
    pub fn tegra_sha_digest(req: *mut ahash_request) -> i32;
    pub fn tegra_sha_export(req: *mut ahash_request, out: *mut core::ffi::c_void) -> i32;
    pub fn tegra_sha_import(req: *mut ahash_request, input: *const core::ffi::c_void) -> i32;
    pub fn tegra_hmac_setkey(tfm: *mut crypto_ahash, key: *const u8, keylen: u32) -> i32;
}

extern "C" {
    pub fn tegra_init_hash(se: *mut tegra_se) -> i32;
    pub fn tegra_deinit_hash(se: *mut tegra_se);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
