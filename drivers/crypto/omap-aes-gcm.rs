// SPDX-License-Identifier: GPL-2.0-only
/*
 * Cryptographic API.
 *
 * Support for OMAP AES GCM HW acceleration.
 *
 * Copyright (c) 2016 Texas Instruments Incorporated
 */

// External kernel declarations and constants are supplied by the surrounding
// translation unit.

unsafe extern "C" {
    fn crypto_finalize_aead_request(engine: *mut crypto_engine, req: *mut aead_request, ret: c_int);
    fn pm_runtime_put_autosuspend(dev: *mut c_void);
    fn aead_request_ctx(req: *mut aead_request) -> *mut omap_aes_reqctx;
    fn dma_sync_sg_for_device(dev: *mut c_void, sg: *mut scatterlist, nents: c_int, direction: c_int);
    fn dma_unmap_sg(dev: *mut c_void, sg: *mut scatterlist, nents: c_int, direction: c_int);
    fn omap_aes_crypt_dma_stop(dd: *mut omap_aes_dev);
    fn omap_crypto_cleanup(sg: *mut scatterlist, orig: *mut scatterlist, assoclen: usize, total: usize, shift: c_int, flags: u16);
    fn scatterwalk_map_and_copy(buf: *mut u8, sg: *mut scatterlist, offset: usize, len: usize, to_sg: c_int);
    fn omap_aes_clear_copy_flags(dd: *mut omap_aes_dev);
    fn sg_init_table(sg: *mut scatterlist, nents: c_int);
    fn omap_crypto_align_sg(sg: *mut *mut scatterlist, len: usize, align: usize, out: *mut scatterlist, flags: u32, shift: c_int, dd_flags: *mut u16) -> c_int;
    fn scatterwalk_ffwd(sg: *mut scatterlist, src: *mut scatterlist, offset: usize) -> *mut scatterlist;
    fn sg_unmark_end(sg: *mut scatterlist);
    fn sg_nents_for_len(sg: *mut scatterlist, len: usize) -> c_int;
    fn crypto_aead_reqtfm(req: *mut aead_request) -> *mut crypto_aead;
    fn crypto_aead_authsize(aead: *mut crypto_aead) -> u32;
    fn aes_encrypt(key: *mut aes_key, out: *mut u8, input: *const u8);
    fn omap_aes_read(dd: *mut omap_aes_dev, reg: u32) -> u32;
    fn crypto_transfer_aead_request_to_engine(engine: *mut crypto_engine, req: *mut aead_request) -> c_int;
    fn crypto_aead_ctx(aead: *mut crypto_aead) -> *mut omap_aes_gcm_ctx;
    fn omap_aes_write_ctrl(dd: *mut omap_aes_dev) -> c_int;
    fn omap_aes_find_dev(rctx: *mut omap_aes_reqctx) -> *mut omap_aes_dev;
    fn crypto_ipsec_check_assoclen(len: usize) -> c_int;
    fn aes_prepareenckey(key: *mut aes_key, input: *const u8, len: u32) -> c_int;
    fn crypto_gcm_check_authsize(authsize: u32) -> c_int;
    fn crypto_rfc4106_check_authsize(authsize: u32) -> c_int;
    fn crypto_aead_set_reqsize(tfm: *mut crypto_aead, size: usize);
    fn omap_aes_crypt_dma_start(dd: *mut omap_aes_dev) -> c_int;
}

type c_int = i32;
type c_void = core::ffi::c_void;

const AES_BLOCK_SIZE: usize = 16;
const GCM_AES_IV_SIZE: usize = 12;
const FLAGS_ENCRYPT: u16 = 1 << 0;
const FLAGS_GCM: u16 = 1 << 1;
const FLAGS_RFC4106_GCM: u16 = 1 << 2;
const FLAGS_MODE_MASK: u16 = 0xffff;
const FLAGS_ASSOC_DATA_ST_SHIFT: c_int = 0;
const FLAGS_IN_DATA_ST_SHIFT: c_int = 1;
const FLAGS_OUT_DATA_ST_SHIFT: c_int = 2;
const OMAP_CRYPTO_COPY_DATA: u32 = 1 << 0;
const OMAP_CRYPTO_ZERO_BUF: u32 = 1 << 1;
const OMAP_CRYPTO_FORCE_SINGLE_ENTRY: u32 = 1 << 2;
const OMAP_CRYPTO_FORCE_COPY: u16 = 1 << 3;
const DMA_FROM_DEVICE: c_int = 0;
const DMA_TO_DEVICE: c_int = 1;
const EBADMSG: c_int = 74;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;

#[repr(C)] pub struct crypto_engine { _private: [u8; 0] }
#[repr(C)] pub struct crypto_aead { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct aes_key { _private: [u8; 0] }
#[repr(C)] pub struct aead_request { pub base: [u8; 0], pub src: *mut scatterlist, pub dst: *mut scatterlist, pub assoclen: usize, pub cryptlen: usize, pub iv: *const u8 }
#[repr(C)] pub struct omap_aes_reqctx { pub auth_tag: [u8; 16], pub iv: [u8; 16], pub mode: u16, pub dd: *mut omap_aes_dev }
#[repr(C)] pub struct omap_aes_gcm_ctx { pub akey: aes_key, pub octx: omap_aes_ctx }
#[repr(C)] pub struct omap_aes_ctx { pub key: [u8; 32], pub nonce: [u8; 4], pub keylen: u32 }
#[repr(C)] pub struct omap_aes_dev { pub aead_req: *mut aead_request, pub in_sg: *mut scatterlist, pub out_sg: *mut scatterlist, pub out_sgl: scatterlist, pub in_sgl: [scatterlist; 2], pub orig_out: *mut scatterlist, pub in_sg_len: c_int, pub out_sg_len: c_int, pub total: usize, pub assoc_len: usize, pub authsize: usize, pub flags: u16, pub engine: *mut crypto_engine, pub dev: *mut c_void, pub ctx: *mut omap_aes_ctx }

unsafe fn omap_aes_gcm_finish_req(dd: *mut omap_aes_dev, ret: c_int) {
    (*dd).in_sg = core::ptr::null_mut();
    (*dd).out_sg = core::ptr::null_mut();
    crypto_finalize_aead_request((*dd).engine, (*dd).aead_req, ret);
    pm_runtime_put_autosuspend((*dd).dev);
}

unsafe fn omap_aes_gcm_done_task(dd: *mut omap_aes_dev) {
    let mut ret = 0;
    let alen = ((*dd).assoc_len + AES_BLOCK_SIZE - 1) / AES_BLOCK_SIZE * AES_BLOCK_SIZE;
    let clen = ((*dd).total + AES_BLOCK_SIZE - 1) / AES_BLOCK_SIZE * AES_BLOCK_SIZE;
    let rctx = aead_request_ctx((*dd).aead_req);
    let nsg = if (*dd).assoc_len != 0 && (*dd).total != 0 { 1 } else { 0 };
    dma_sync_sg_for_device((*dd).dev, (*dd).out_sg, (*dd).out_sg_len, DMA_FROM_DEVICE);
    dma_unmap_sg((*dd).dev, (*dd).in_sg, (*dd).in_sg_len, DMA_TO_DEVICE);
    dma_unmap_sg((*dd).dev, (*dd).out_sg, (*dd).out_sg_len, DMA_FROM_DEVICE);
    omap_aes_crypt_dma_stop(dd);
    omap_crypto_cleanup((*dd).out_sg, (*dd).orig_out, (*dd).aead_req.as_ref().unwrap().assoclen, (*dd).total, FLAGS_OUT_DATA_ST_SHIFT, (*dd).flags);
    if (*dd).flags & FLAGS_ENCRYPT != 0 { scatterwalk_map_and_copy((*rctx).auth_tag.as_mut_ptr(), (*dd).aead_req.as_ref().unwrap().dst, (*dd).total + (*dd).aead_req.as_ref().unwrap().assoclen, (*dd).authsize, 1); }
    omap_crypto_cleanup((*dd).in_sgl.as_mut_ptr(), core::ptr::null_mut(), 0, alen, FLAGS_ASSOC_DATA_ST_SHIFT, (*dd).flags);
    omap_crypto_cleanup((*dd).in_sgl.as_mut_ptr().add(nsg), core::ptr::null_mut(), 0, clen, FLAGS_IN_DATA_ST_SHIFT, (*dd).flags);
    if (*dd).flags & FLAGS_ENCRYPT == 0 { for i in 0..(*dd).authsize { if (*rctx).auth_tag[i] != 0 { ret = -EBADMSG; } } }
    omap_aes_gcm_finish_req(dd, ret);
}

unsafe fn do_encrypt_iv(req: *mut aead_request, tag: *mut u32, iv: *mut u32) -> c_int {
    let ctx = crypto_aead_ctx(crypto_aead_reqtfm(req));
    aes_encrypt(&mut (*ctx).akey, tag as *mut u8, iv as *const u8);
    0
}

#[no_mangle] pub unsafe extern "C" fn omap_aes_gcm_dma_out_callback(data: *mut c_void) {
    let dd = data as *mut omap_aes_dev;
    let rctx = aead_request_ctx((*dd).aead_req);
    let mut tag = [0u32; 4];
    if (*dd).flags & FLAGS_ENCRYPT == 0 { scatterwalk_map_and_copy(tag.as_mut_ptr() as *mut u8, (*dd).aead_req.as_ref().unwrap().src, (*dd).total + (*dd).aead_req.as_ref().unwrap().assoclen, (*dd).authsize, 0); }
    for i in 0..4 { let val = omap_aes_read(dd, i as u32); let p = (*rctx).auth_tag.as_mut_ptr() as *mut u32; *p.add(i) ^= val; if (*dd).flags & FLAGS_ENCRYPT == 0 { *p.add(i) ^= tag[i]; } }
    omap_aes_gcm_done_task(dd);
}

unsafe fn omap_aes_gcm_handle_queue(dd: *mut omap_aes_dev, req: *mut aead_request) -> c_int { if !req.is_null() { crypto_transfer_aead_request_to_engine((*dd).engine, req) } else { 0 } }

unsafe fn omap_aes_gcm_copy_buffers(dd: *mut omap_aes_dev, req: *mut aead_request) -> c_int {
    let aead = crypto_aead_reqtfm(req); let authlen = crypto_aead_authsize(aead) as usize; let mut assoclen = (*req).assoclen; let mut cryptlen = (*req).cryptlen;
    if (*dd).flags & FLAGS_RFC4106_GCM != 0 { assoclen -= 8; } if (*dd).flags & FLAGS_ENCRYPT == 0 { cryptlen -= authlen; }
    let alen = (assoclen + 15) / 16 * 16; let clen = (cryptlen + 15) / 16 * 16; let nsg = if assoclen != 0 && cryptlen != 0 { 1 } else { 0 };
    omap_aes_clear_copy_flags(dd); sg_init_table((*dd).in_sgl.as_mut_ptr(), nsg + 1);
    let mut tmp; let mut sg_arr = [scatterlist { _private: [] }, scatterlist { _private: [] }];
    if assoclen != 0 { tmp = (*req).src; let r = omap_crypto_align_sg(&mut tmp, assoclen, 16, (*dd).in_sgl.as_mut_ptr(), OMAP_CRYPTO_COPY_DATA | OMAP_CRYPTO_ZERO_BUF | OMAP_CRYPTO_FORCE_SINGLE_ENTRY, FLAGS_ASSOC_DATA_ST_SHIFT, &mut (*dd).flags); if r != 0 { return r; } }
    if cryptlen != 0 { tmp = scatterwalk_ffwd(sg_arr.as_mut_ptr(), (*req).src, (*req).assoclen); if nsg != 0 { sg_unmark_end((*dd).in_sgl.as_mut_ptr()); } let r = omap_crypto_align_sg(&mut tmp, cryptlen, 16, (*dd).in_sgl.as_mut_ptr().add(nsg), OMAP_CRYPTO_COPY_DATA | OMAP_CRYPTO_ZERO_BUF | OMAP_CRYPTO_FORCE_SINGLE_ENTRY, FLAGS_IN_DATA_ST_SHIFT, &mut (*dd).flags); if r != 0 { return r; } }
    (*dd).in_sg = (*dd).in_sgl.as_mut_ptr(); (*dd).total = cryptlen; (*dd).assoc_len = assoclen; (*dd).authsize = authlen; (*dd).out_sg = scatterwalk_ffwd(sg_arr.as_mut_ptr(), (*req).dst, (*req).assoclen); (*dd).orig_out = (*req).dst;
    let flags = if (*req).src == (*req).dst || (*dd).out_sg == sg_arr.as_mut_ptr() { OMAP_CRYPTO_FORCE_COPY } else { 0 }; if cryptlen != 0 { let r = omap_crypto_align_sg(&mut (*dd).out_sg, cryptlen, 16, &mut (*dd).out_sgl, flags as u32, FLAGS_OUT_DATA_ST_SHIFT, &mut (*dd).flags); if r != 0 { return r; } }
    (*dd).in_sg_len = sg_nents_for_len((*dd).in_sg, alen + clen); (*dd).out_sg_len = sg_nents_for_len((*dd).out_sg, clen); 0
}

unsafe fn omap_aes_gcm_prepare_req(req: *mut aead_request, dd: *mut omap_aes_dev) -> c_int { let rctx = aead_request_ctx(req); let ctx = crypto_aead_ctx(crypto_aead_reqtfm(req)); (*dd).aead_req = req; (*rctx).mode &= FLAGS_MODE_MASK; (*dd).flags = ((*dd).flags & !FLAGS_MODE_MASK) | (*rctx).mode; let err = omap_aes_gcm_copy_buffers(dd, req); if err != 0 { return err; } (*dd).ctx = &mut (*ctx).octx; omap_aes_write_ctrl(dd) }

unsafe fn omap_aes_gcm_crypt(req: *mut aead_request, mode: u16) -> c_int { let rctx = aead_request_ctx(req); let authlen = crypto_aead_authsize(crypto_aead_reqtfm(req)) as usize; let counter: u32 = 1u32.to_be(); (*rctx).auth_tag.fill(0); (*rctx).iv[GCM_AES_IV_SIZE..GCM_AES_IV_SIZE + 4].copy_from_slice(&counter.to_ne_bytes()); let err = do_encrypt_iv(req, (*rctx).auth_tag.as_mut_ptr() as *mut u32, (*rctx).iv.as_mut_ptr() as *mut u32); if err != 0 { return err; } let assoclen = if mode & FLAGS_RFC4106_GCM != 0 { (*req).assoclen - 8 } else { (*req).assoclen }; if assoclen + (*req).cryptlen == 0 { scatterwalk_map_and_copy((*rctx).auth_tag.as_mut_ptr(), (*req).dst, 0, authlen, 1); return 0; } let dd = omap_aes_find_dev(rctx); if dd.is_null() { return -ENODEV; } (*rctx).mode = mode; omap_aes_gcm_handle_queue(dd, req) }

#[no_mangle] pub unsafe extern "C" fn omap_aes_gcm_encrypt(req: *mut aead_request) -> c_int { let rctx = aead_request_ctx(req); (*rctx).iv[..GCM_AES_IV_SIZE].copy_from_slice(core::slice::from_raw_parts((*req).iv, GCM_AES_IV_SIZE)); omap_aes_gcm_crypt(req, FLAGS_ENCRYPT | FLAGS_GCM) }
#[no_mangle] pub unsafe extern "C" fn omap_aes_gcm_decrypt(req: *mut aead_request) -> c_int { let rctx = aead_request_ctx(req); (*rctx).iv[..GCM_AES_IV_SIZE].copy_from_slice(core::slice::from_raw_parts((*req).iv, GCM_AES_IV_SIZE)); omap_aes_gcm_crypt(req, FLAGS_GCM) }
#[no_mangle] pub unsafe extern "C" fn omap_aes_4106gcm_encrypt(req: *mut aead_request) -> c_int { let ctx = crypto_aead_ctx(crypto_aead_reqtfm(req)); let rctx = aead_request_ctx(req); (*rctx).iv[..4].copy_from_slice(&(*ctx).octx.nonce); (*rctx).iv[4..12].copy_from_slice(core::slice::from_raw_parts((*req).iv, 8)); let e = crypto_ipsec_check_assoclen((*req).assoclen); if e != 0 { e } else { omap_aes_gcm_crypt(req, FLAGS_ENCRYPT | FLAGS_GCM | FLAGS_RFC4106_GCM) } }
#[no_mangle] pub unsafe extern "C" fn omap_aes_4106gcm_decrypt(req: *mut aead_request) -> c_int { let ctx = crypto_aead_ctx(crypto_aead_reqtfm(req)); let rctx = aead_request_ctx(req); (*rctx).iv[..4].copy_from_slice(&(*ctx).octx.nonce); (*rctx).iv[4..12].copy_from_slice(core::slice::from_raw_parts((*req).iv, 8)); let e = crypto_ipsec_check_assoclen((*req).assoclen); if e != 0 { e } else { omap_aes_gcm_crypt(req, FLAGS_GCM | FLAGS_RFC4106_GCM) } }

#[no_mangle] pub unsafe extern "C" fn omap_aes_gcm_setkey(tfm: *mut crypto_aead, key: *const u8, keylen: u32) -> c_int { let ctx = crypto_aead_ctx(tfm); let ret = aes_prepareenckey(&mut (*ctx).akey, key, keylen); if ret != 0 { return ret; } core::ptr::copy_nonoverlapping(key, (*ctx).octx.key.as_mut_ptr(), keylen as usize); (*ctx).octx.keylen = keylen; 0 }
#[no_mangle] pub unsafe extern "C" fn omap_aes_4106gcm_setkey(tfm: *mut crypto_aead, key: *const u8, mut keylen: u32) -> c_int { let ctx = crypto_aead_ctx(tfm); if keylen < 4 { return -EINVAL; } keylen -= 4; let ret = aes_prepareenckey(&mut (*ctx).akey, key, keylen); if ret != 0 { return ret; } core::ptr::copy_nonoverlapping(key, (*ctx).octx.key.as_mut_ptr(), keylen as usize); core::ptr::copy_nonoverlapping(key.add(keylen as usize), (*ctx).octx.nonce.as_mut_ptr(), 4); (*ctx).octx.keylen = keylen; 0 }
#[no_mangle] pub unsafe extern "C" fn omap_aes_gcm_setauthsize(_tfm: *mut crypto_aead, authsize: u32) -> c_int { crypto_gcm_check_authsize(authsize) }
#[no_mangle] pub unsafe extern "C" fn omap_aes_4106gcm_setauthsize(_parent: *mut crypto_aead, authsize: u32) -> c_int { crypto_rfc4106_check_authsize(authsize) }
#[no_mangle] pub unsafe extern "C" fn omap_aes_gcm_crypt_req(engine: *mut crypto_engine, areq: *mut c_void) -> c_int { let req = areq as *mut aead_request; let rctx = aead_request_ctx(req); let dd = (*rctx).dd; if dd.is_null() { return -ENODEV; } let ret = omap_aes_gcm_prepare_req(req, dd); if ret != 0 { return ret; } if (*dd).in_sg_len != 0 { omap_aes_crypt_dma_start(dd) } else { omap_aes_gcm_dma_out_callback(dd as *mut c_void); 0 } }
#[no_mangle] pub unsafe extern "C" fn omap_aes_gcm_cra_init(tfm: *mut crypto_aead) -> c_int { crypto_aead_set_reqsize(tfm, core::mem::size_of::<omap_aes_reqctx>()); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
