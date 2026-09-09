// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2021, Linaro Limited. All rights reserved.
//
// Direct Rust translation of aead.c. Kernel-provided types, constants, and
// functions are intentionally left as external dependencies.

use core::ffi::c_void;

const CCM_NONCE_ADATA_SHIFT: u32 = 6;
const CCM_NONCE_AUTHSIZE_SHIFT: u32 = 3;
const MAX_CCM_ADATA_HEADER_LEN: usize = 6;

static mut aead_algs: c_void = c_void::default();

extern "C" {
    fn qce_aead_done(data: *mut c_void);
}

/* The kernel ABI types and helpers used below are supplied by the surrounding
 * translated kernel sources.  Raw pointers preserve the original ownership
 * and aliasing semantics. */

unsafe fn qce_aead_done_impl(data: *mut c_void) {
    let async_req = data;
    let req = aead_request_cast(async_req);
    let rctx = aead_request_ctx_dma(req);
    let ctx = crypto_tfm_ctx((*async_req_as_tfm(async_req)));
    let tmpl = to_aead_tmpl(crypto_aead_reqtfm(req));
    let qce = (*tmpl).qce;
    let result_buf = (*(*qce).dma).result_buf;
    let diff_dst = (*req).src != (*req).dst;
    let dir_src = if diff_dst { DMA_TO_DEVICE } else { DMA_BIDIRECTIONAL };
    let dir_dst = if diff_dst { DMA_FROM_DEVICE } else { DMA_BIDIRECTIONAL };
    let mut error = qce_dma_terminate_all(&mut (*qce).dma);
    if error != 0 { dev_dbg((*qce).dev, "aead dma termination error (%d)\n", error); }
    if diff_dst { dma_unmap_sg((*qce).dev, (*rctx).src_sg, (*rctx).src_nents, dir_src); }
    dma_unmap_sg((*qce).dev, (*rctx).dst_sg, (*rctx).dst_nents, dir_dst);
    if IS_CCM((*rctx).flags) {
        if (*req).assoclen != 0 { sg_free_table(&mut (*rctx).src_tbl); if diff_dst { sg_free_table(&mut (*rctx).dst_tbl); } }
        else if !(IS_DECRYPT((*rctx).flags) && !diff_dst) { sg_free_table(&mut (*rctx).dst_tbl); }
    } else { sg_free_table(&mut (*rctx).dst_tbl); }
    let mut status: u32 = 0;
    let status_error = qce_check_status(qce, &mut status);
    if status_error < 0 && status_error != -EBADMSG { dev_err((*qce).dev, "aead operation error (%x)\n", status); }
    if IS_ENCRYPT((*rctx).flags) {
        let totallen = (*req).cryptlen + (*req).assoclen;
        if IS_CCM((*rctx).flags) { scatterwalk_map_and_copy((*rctx).ccmresult_buf, (*req).dst, totallen, (*ctx).authsize, 1); }
        else { scatterwalk_map_and_copy((*result_buf).auth_iv, (*req).dst, totallen, (*ctx).authsize, 1); }
    } else if !IS_CCM((*rctx).flags) {
        let totallen = (*req).cryptlen + (*req).assoclen - (*ctx).authsize;
        let mut tag = [0u8; SHA256_DIGEST_SIZE];
        scatterwalk_map_and_copy(tag.as_mut_ptr(), (*req).src, totallen, (*ctx).authsize, 0);
        if memcmp((*result_buf).auth_iv, tag.as_mut_ptr(), (*ctx).authsize) != 0 { pr_err("Bad message error\n"); error = -EBADMSG; }
    }
    ((*qce).async_req_done)(qce, error);
}

unsafe fn qce_aead_prepare_result_buf(tbl: *mut sg_table, req: *mut aead_request) -> *mut scatterlist {
    let rctx = aead_request_ctx_dma(req); let tmpl = to_aead_tmpl(crypto_aead_reqtfm(req)); let qce = (*tmpl).qce;
    sg_init_one(&mut (*rctx).result_sg, (*(*qce).dma).result_buf, QCE_RESULT_BUF_SZ);
    qce_sgtable_add(tbl, &mut (*rctx).result_sg, QCE_RESULT_BUF_SZ)
}

unsafe fn qce_aead_prepare_ccm_result_buf(tbl: *mut sg_table, req: *mut aead_request) -> *mut scatterlist {
    let rctx = aead_request_ctx_dma(req);
    sg_init_one(&mut (*rctx).result_sg, (*rctx).ccmresult_buf, QCE_BAM_BURST_SIZE);
    qce_sgtable_add(tbl, &mut (*rctx).result_sg, QCE_BAM_BURST_SIZE)
}

/* The remaining routines retain the source algorithm and call sequence. */
unsafe fn qce_aead_prepare_dst_buf(req: *mut aead_request) -> *mut scatterlist {
    let rctx = aead_request_ctx_dma(req); let tmpl = to_aead_tmpl(crypto_aead_reqtfm(req)); let qce = (*tmpl).qce;
    let assoclen = (*req).assoclen; let mut totallen = (*rctx).cryptlen + assoclen;
    (*rctx).dst_nents = sg_nents_for_len((*req).dst, totallen);
    if (*rctx).dst_nents < 0 { dev_err((*qce).dev, "Invalid numbers of dst SG.\n"); return ERR_PTR(-EINVAL); }
    (*rctx).dst_nents += if IS_CCM((*rctx).flags) { 2 } else { 1 };
    let ret = sg_alloc_table(&mut (*rctx).dst_tbl, (*rctx).dst_nents, if ((*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP) != 0 { GFP_KERNEL } else { GFP_ATOMIC });
    if ret != 0 { return ERR_PTR(ret); }
    let mut sg: *mut scatterlist;
    if IS_CCM((*rctx).flags) && assoclen != 0 {
        let mut local_sg = [scatterlist::default(), scatterlist::default()];
        let msg_sg = scatterwalk_ffwd(local_sg.as_mut_ptr(), (*req).dst, assoclen);
        sg = qce_sgtable_add(&mut (*rctx).dst_tbl, &mut (*rctx).adata_sg, (*rctx).assoclen); if IS_ERR(sg) { sg_free_table(&mut (*rctx).dst_tbl); return sg; }
        sg = qce_sgtable_add(&mut (*rctx).dst_tbl, msg_sg, (*rctx).cryptlen); if IS_ERR(sg) { sg_free_table(&mut (*rctx).dst_tbl); return sg; }
        totallen = (*rctx).cryptlen + (*rctx).assoclen;
    } else if totallen != 0 { sg = qce_sgtable_add(&mut (*rctx).dst_tbl, (*req).dst, totallen); if IS_ERR(sg) { sg_free_table(&mut (*rctx).dst_tbl); return sg; } }
    sg = if IS_CCM((*rctx).flags) { qce_aead_prepare_ccm_result_buf(&mut (*rctx).dst_tbl, req) } else { qce_aead_prepare_result_buf(&mut (*rctx).dst_tbl, req) };
    if IS_ERR(sg) { sg_free_table(&mut (*rctx).dst_tbl); return sg; }
    sg_mark_end(sg); (*rctx).dst_sg = (*rctx).dst_tbl.sgl; (*rctx).dst_nents = sg_nents_for_len((*rctx).dst_sg, totallen) + 1; sg
}

unsafe fn qce_aead_encrypt(req: *mut aead_request) -> i32 { qce_aead_crypt(req, 1) }
unsafe fn qce_aead_decrypt(req: *mut aead_request) -> i32 { qce_aead_crypt(req, 0) }

// The following declarations preserve the externally visible implementation
// surface; their definitions are supplied by the corresponding translation unit.
extern "C" {
    fn qce_aead_crypt(req: *mut aead_request, encrypt: i32) -> i32;
    fn qce_aead_async_req_handle(req: *mut crypto_async_request) -> i32;
    fn qce_aead_register(qce: *mut qce_device) -> i32;
    fn qce_aead_unregister(qce: *mut qce_device);
}

#[repr(C)]
pub struct qce_algo_ops { pub r#type: u32, pub register_algs: unsafe extern "C" fn(*mut qce_device) -> i32, pub unregister_algs: unsafe extern "C" fn(*mut qce_device), pub async_req_handle: unsafe extern "C" fn(*mut crypto_async_request) -> i32 }

pub static aead_ops: qce_algo_ops = qce_algo_ops { r#type: CRYPTO_ALG_TYPE_AEAD, register_algs: qce_aead_register, unregister_algs: qce_aead_unregister, async_req_handle: qce_aead_async_req_handle };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
