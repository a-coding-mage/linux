// SPDX-License-Identifier: GPL-2.0
// Linux/kernel, crypto, and local driver headers are supplied by other units.

const GCM_AES_SALT_SIZE: usize = 4;

#[repr(C)]
pub union GphP3 {
    pub bits: GphP3Bits,
    pub param: u16,
}
#[repr(C)]
pub struct GphP3Bits { pub auth_offset: u16, pub iv_offset: u16 }

unsafe fn nitrox_aes_gcm_setkey(aead: *mut crypto_aead, key: *const u8, keylen: u32) -> i32 {
    let nctx = crypto_aead_ctx(aead);
    let fctx = (*nctx).u.fctx;
    let aes_keylen = flexi_aes_keylen(keylen);
    if aes_keylen < 0 { return -EINVAL; }
    let mut flags = (*fctx).flags;
    flags.fu = be64_to_cpu((*fctx).flags.f);
    flags.w0.aes_keylen = aes_keylen;
    (*fctx).flags.f = cpu_to_be64(flags.fu);
    core::ptr::write_bytes(&mut (*fctx).crypto as *mut _, 0, core::mem::size_of_val(&(*fctx).crypto));
    core::ptr::copy_nonoverlapping(key, (*fctx).crypto.u.key.as_mut_ptr(), keylen as usize);
    0
}

unsafe fn nitrox_aead_setauthsize(aead: *mut crypto_aead, authsize: u32) -> i32 {
    let nctx = crypto_aead_ctx(aead); let fctx = (*nctx).u.fctx;
    let mut flags = (*fctx).flags; flags.fu = be64_to_cpu((*fctx).flags.f);
    flags.w0.mac_len = authsize; (*fctx).flags.f = cpu_to_be64(flags.fu);
    (*aead).authsize = authsize; 0
}

unsafe fn nitrox_aes_gcm_setauthsize(aead: *mut crypto_aead, authsize: u32) -> i32 {
    match authsize { 4 | 8 | 12 | 13 | 14 | 15 | 16 => {}, _ => return -EINVAL }
    nitrox_aead_setauthsize(aead, authsize)
}

unsafe fn alloc_src_sglist(nkreq: *mut nitrox_kcrypt_request, src: *mut scatterlist, iv: *const i8, ivsize: i32, buflen: i32) -> i32 {
    let mut nents = sg_nents_for_len(src, buflen); if nents < 0 { return nents; }
    nents += 1; let ret = alloc_src_req_buf(nkreq, nents, ivsize); if ret != 0 { return ret; }
    nitrox_creq_copy_iv((*nkreq).src, iv, ivsize); nitrox_creq_set_src_sg(nkreq, nents, ivsize, src, buflen); 0
}
unsafe fn alloc_dst_sglist(nkreq: *mut nitrox_kcrypt_request, dst: *mut scatterlist, ivsize: i32, buflen: i32) -> i32 {
    let mut nents = sg_nents_for_len(dst, buflen); if nents < 0 { return nents; }
    nents += 3; let ret = alloc_dst_req_buf(nkreq, nents); if ret != 0 { return ret; }
    nitrox_creq_set_orh(nkreq); nitrox_creq_set_comp(nkreq); nitrox_creq_set_dst_sg(nkreq, nents, ivsize, dst, buflen); 0
}
unsafe fn free_src_sglist(nkreq: *mut nitrox_kcrypt_request) { kfree((*nkreq).src); }
unsafe fn free_dst_sglist(nkreq: *mut nitrox_kcrypt_request) { kfree((*nkreq).dst); }

unsafe fn nitrox_set_creq(rctx: *mut nitrox_aead_rctx) -> i32 {
    let creq = &mut (*rctx).nkreq.creq; let mut param3 = GphP3 { param: 0 };
    creq.flags = (*rctx).flags; creq.gfp = if (*rctx).flags & CRYPTO_TFM_REQ_MAY_SLEEP != 0 { GFP_KERNEL } else { GFP_ATOMIC };
    creq.ctrl.value = 0; creq.opcode = FLEXI_CRYPTO_ENCRYPT_HMAC; creq.ctrl.s.arg = (*rctx).ctrl_arg;
    creq.gph.param0 = cpu_to_be16((*rctx).cryptlen as u16); creq.gph.param1 = cpu_to_be16(((*rctx).cryptlen + (*rctx).assoclen) as u16);
    creq.gph.param2 = cpu_to_be16(((*rctx).ivsize + (*rctx).assoclen) as u16);
    (*param3.bits_mut()).iv_offset = 0; (*param3.bits_mut()).auth_offset = (*rctx).ivsize as u16;
    creq.gph.param3 = cpu_to_be16(param3.param); creq.ctx_handle = (*rctx).ctx_handle;
    creq.ctrl.s.ctxl = core::mem::size_of::<flexi_crypto_context>() as _;
    let ret = alloc_src_sglist(&mut (*rctx).nkreq, (*rctx).src, (*rctx).iv, (*rctx).ivsize, (*rctx).srclen); if ret != 0 { return ret; }
    let ret = alloc_dst_sglist(&mut (*rctx).nkreq, (*rctx).dst, (*rctx).ivsize, (*rctx).dstlen); if ret != 0 { free_src_sglist(&mut (*rctx).nkreq); return ret; } 0
}

unsafe extern "C" fn nitrox_aead_callback(arg: *mut core::ffi::c_void, mut err: i32) {
    let areq = arg as *mut aead_request; let rctx = aead_request_ctx(areq);
    free_src_sglist(&mut (*rctx).nkreq); free_dst_sglist(&mut (*rctx).nkreq);
    if err != 0 { pr_err_ratelimited!("request failed status 0x%0x\n", err); err = -EINVAL; }
    aead_request_complete(areq, err);
}
unsafe fn nitrox_aes_gcm_assoclen_supported(assoclen: u32) -> bool { assoclen <= 512 }

// The remaining operations preserve the C driver callbacks and algorithm table.
// Types and helper symbols are external declarations supplied by the driver headers.
extern "C" {
    fn nitrox_aes_gcm_enc(areq: *mut aead_request) -> i32;
    fn nitrox_aes_gcm_dec(areq: *mut aead_request) -> i32;
    fn nitrox_rfc4106_enc(areq: *mut aead_request) -> i32;
    fn nitrox_rfc4106_dec(areq: *mut aead_request) -> i32;
}

pub unsafe fn nitrox_register_aeads() -> i32 { crypto_register_aeads(nitrox_aeads.as_mut_ptr(), nitrox_aeads.len() as _ ) }
pub unsafe fn nitrox_unregister_aeads() { crypto_unregister_aeads(nitrox_aeads.as_mut_ptr(), nitrox_aeads.len() as _); }

// C aead_alg initializers retain field names and callbacks for the external ABI.
extern "C" { static mut nitrox_aeads: [aead_alg; 2]; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
