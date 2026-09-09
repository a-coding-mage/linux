// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2010-2014, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel/QCE translation.

#[repr(C)]
struct QceShaSavedState {
    pending_buf: [u8; QCE_SHA_MAX_BLOCKSIZE],
    partial_digest: [u8; QCE_SHA_MAX_DIGESTSIZE],
    byte_count: [u32; 2], // __be32
    pending_buflen: ::core::ffi::c_uint,
    flags: ::core::ffi::c_uint,
    count: u64,
    first_blk: bool,
}

static mut AHASH_ALGS: ListHead = ListHead::new();

static STD_IV_SHA256: [u32; SHA256_DIGEST_SIZE / core::mem::size_of::<u32>()] = [
    SHA256_H0, SHA256_H1, SHA256_H2, SHA256_H3,
    SHA256_H4, SHA256_H5, SHA256_H6, SHA256_H7,
];

unsafe fn qce_ahash_done(data: *mut core::ffi::c_void) {
    let async_req = data as *mut CryptoAsyncRequest;
    let req = ahash_request_cast(async_req);
    let ahash = crypto_ahash_reqtfm(req);
    let rctx = ahash_request_ctx_dma(req);
    let tmpl = to_ahash_tmpl((*async_req).tfm);
    let qce = (*tmpl).qce;
    let result = (*qce).dma.result_buf;
    let digestsize = crypto_ahash_digestsize(ahash);
    let mut error: i32;
    let mut status: u32 = 0;

    error = qce_dma_terminate_all(&mut (*qce).dma);
    if error != 0 { dev_dbg((*qce).dev, "ahash dma termination error (%d)\n", error); }
    dma_unmap_sg((*qce).dev, (*req).src, (*rctx).src_nents, DMA_TO_DEVICE);
    dma_unmap_sg((*qce).dev, &mut (*rctx).result_sg, 1, DMA_FROM_DEVICE);
    memcpy((*rctx).digest.as_mut_ptr() as *mut _, (*result).auth_iv.as_ptr() as *const _, digestsize);
    if !(*req).result.is_null() && (*rctx).last_blk { memcpy((*req).result as *mut _, (*result).auth_iv.as_ptr() as *const _, digestsize); }
    (*rctx).byte_count[0] = cpu_to_be32((*result).auth_byte_count[0]);
    (*rctx).byte_count[1] = cpu_to_be32((*result).auth_byte_count[1]);
    error = qce_check_status(qce, &mut status);
    if error < 0 { dev_dbg((*qce).dev, "ahash operation error (%x)\n", status); }
    (*req).src = (*rctx).src_orig;
    (*req).nbytes = (*rctx).nbytes_orig;
    (*rctx).last_blk = false;
    (*rctx).first_blk = false;
    ((*tmpl).qce).async_req_done((*tmpl).qce, error);
}

unsafe fn qce_ahash_async_req_handle(async_req: *mut CryptoAsyncRequest) -> i32 {
    let req = ahash_request_cast(async_req);
    let rctx = ahash_request_ctx_dma(req);
    let ctx = crypto_tfm_ctx((*async_req).tfm);
    let tmpl = to_ahash_tmpl((*async_req).tfm);
    let qce = (*tmpl).qce;
    let flags = (*rctx).flags;
    let mut ret: i32;
    if IS_SHA_HMAC(flags) { (*rctx).authkey = (*ctx).authkey; (*rctx).authklen = QCE_SHA_HMAC_KEY_SIZE; }
    else if IS_CMAC(flags) { (*rctx).authkey = (*ctx).authkey; (*rctx).authklen = AES_KEYSIZE_128; }
    (*rctx).src_nents = sg_nents_for_len((*req).src, (*req).nbytes);
    if (*rctx).src_nents < 0 { dev_err((*qce).dev, "Invalid numbers of src SG.\n"); return (*rctx).src_nents; }
    ret = dma_map_sg((*qce).dev, (*req).src, (*rctx).src_nents, DMA_TO_DEVICE); if ret == 0 { return -EIO; }
    sg_init_one(&mut (*rctx).result_sg, (*qce).dma.result_buf as *mut _, QCE_RESULT_BUF_SZ);
    ret = dma_map_sg((*qce).dev, &mut (*rctx).result_sg, 1, DMA_FROM_DEVICE); if ret == 0 { dma_unmap_sg((*qce).dev, (*req).src, (*rctx).src_nents, DMA_TO_DEVICE); return -EIO; }
    ret = qce_dma_prep_sgs(&mut (*qce).dma, (*req).src, (*rctx).src_nents, &mut (*rctx).result_sg, 1, Some(qce_ahash_done), async_req);
    if ret != 0 { dma_unmap_sg((*qce).dev, &mut (*rctx).result_sg, 1, DMA_FROM_DEVICE); dma_unmap_sg((*qce).dev, (*req).src, (*rctx).src_nents, DMA_TO_DEVICE); return ret; }
    qce_dma_issue_pending(&mut (*qce).dma);
    ret = qce_start(async_req, (*tmpl).crypto_alg_type); if ret != 0 { qce_dma_terminate_all(&mut (*qce).dma); dma_unmap_sg((*qce).dev, &mut (*rctx).result_sg, 1, DMA_FROM_DEVICE); dma_unmap_sg((*qce).dev, (*req).src, (*rctx).src_nents, DMA_TO_DEVICE); }
    ret
}

unsafe fn qce_ahash_init(req: *mut AhashRequest) -> i32 {
    let rctx = ahash_request_ctx_dma(req); let tmpl = to_ahash_tmpl((*req).base.tfm);
    memset(rctx as *mut _, 0, core::mem::size_of::<QceShaReqctx>()); (*rctx).first_blk = true; (*rctx).last_blk = false; (*rctx).flags = (*tmpl).alg_flags;
    memcpy((*rctx).digest.as_mut_ptr() as *mut _, (*tmpl).std_iv as *const _, core::mem::size_of_val(&(*rctx).digest)); 0
}

unsafe fn qce_ahash_export(req: *mut AhashRequest, out: *mut core::ffi::c_void) -> i32 {
    let r = ahash_request_ctx_dma(req); let s = out as *mut QceShaSavedState;
    memcpy((*s).pending_buf.as_mut_ptr() as *mut _, (*r).buf.as_ptr() as *const _, (*r).buflen as usize); memcpy((*s).partial_digest.as_mut_ptr() as *mut _, (*r).digest.as_ptr() as *const _, core::mem::size_of_val(&(*r).digest));
    (*s).byte_count = (*r).byte_count; (*s).pending_buflen = (*r).buflen; (*s).count = (*r).count; (*s).first_blk = (*r).first_blk; (*s).flags = (*r).flags; 0
}
unsafe fn qce_ahash_import(req: *mut AhashRequest, input: *const core::ffi::c_void) -> i32 {
    let r = ahash_request_ctx_dma(req); let s = input as *const QceShaSavedState; memset(r as *mut _, 0, core::mem::size_of::<QceShaReqctx>()); (*r).count=(*s).count; (*r).buflen=(*s).pending_buflen; (*r).first_blk=(*s).first_blk; (*r).flags=(*s).flags; (*r).byte_count=(*s).byte_count; memcpy((*r).buf.as_mut_ptr() as *mut _, (*s).pending_buf.as_ptr() as *const _, (*r).buflen as usize); memcpy((*r).digest.as_mut_ptr() as *mut _, (*s).partial_digest.as_ptr() as *const _, core::mem::size_of_val(&(*r).digest)); 0
}

unsafe fn qce_ahash_update(req: *mut AhashRequest) -> i32 {
    let tfm=crypto_ahash_reqtfm(req); let r=ahash_request_ctx_dma(req); let tmpl=to_ahash_tmpl((*req).base.tfm); let qce=(*tmpl).qce; let blocksize=crypto_tfm_alg_blocksize(crypto_ahash_tfm(tfm)); (*r).count += (*req).nbytes as u64; let total=(*req).nbytes+(*r).buflen;
    if total <= blocksize { scatterwalk_map_and_copy((*r).buf.as_mut_ptr().add((*r).buflen as usize),(*req).src,0,(*req).nbytes,0); (*r).buflen+=(*req).nbytes; return 0; }
    (*r).src_orig=(*req).src; (*r).nbytes_orig=(*req).nbytes; if (*r).buflen!=0 { memcpy((*r).tmpbuf.as_mut_ptr() as *mut _,(*r).buf.as_ptr() as *const _,(*r).buflen as usize); }
    let mut later=total%blocksize; if later==0 {later=blocksize;} scatterwalk_map_and_copy((*r).buf.as_mut_ptr(),(*req).src,(*req).nbytes-later,later,0);
    if (*r).buflen!=0 { sg_init_table((*r).sg.as_mut_ptr(),2); sg_set_buf((*r).sg.as_mut_ptr(),(*r).tmpbuf.as_mut_ptr() as *mut _,(*r).buflen); sg_chain((*r).sg.as_mut_ptr(),2,(*req).src); (*req).src=(*r).sg.as_mut_ptr(); }
    (*req).nbytes=total-later; (*r).buflen=later; (*qce).async_req_enqueue((*tmpl).qce,&mut (*req).base)
}

unsafe fn qce_ahash_final(req:*mut AhashRequest)->i32 { let r=ahash_request_ctx_dma(req); let t=to_ahash_tmpl((*req).base.tfm); if (*r).buflen==0 { if !(*t).hash_zero.is_null(){memcpy((*req).result as *mut _,(*t).hash_zero as *const _,(*t).alg.ahash.halg.digestsize);} return 0;} (*r).last_blk=true; (*r).src_orig=(*req).src; (*r).nbytes_orig=(*req).nbytes; memcpy((*r).tmpbuf.as_mut_ptr() as *mut _,(*r).buf.as_ptr() as *const _,(*r).buflen); sg_init_one((*r).sg.as_mut_ptr(),(*r).tmpbuf.as_mut_ptr() as *mut _,(*r).buflen); (*req).src=(*r).sg.as_mut_ptr(); (*req).nbytes=(*r).buflen; (*t).qce.async_req_enqueue((*t).qce,&mut (*req).base) }

unsafe fn qce_ahash_digest(req:*mut AhashRequest)->i32 { let r=ahash_request_ctx_dma(req); let t=to_ahash_tmpl((*req).base.tfm); let _=qce_ahash_init(req); (*r).src_orig=(*req).src; (*r).nbytes_orig=(*req).nbytes; (*r).first_blk=true; (*r).last_blk=true; if (*r).nbytes_orig==0 {if !(*t).hash_zero.is_null(){memcpy((*req).result as *mut _,(*t).hash_zero as *const _,(*t).alg.ahash.halg.digestsize);} return 0;} (*t).qce.async_req_enqueue((*t).qce,&mut (*req).base) }

// Algorithm registration data and callbacks are supplied through the translated QCE interfaces.
#[no_mangle] pub static mut ahash_ops: QceAlgoOps = QceAlgoOps { r#type: CRYPTO_ALG_TYPE_AHASH, register_algs: qce_ahash_register, unregister_algs: qce_ahash_unregister, async_req_handle: qce_ahash_async_req_handle };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
