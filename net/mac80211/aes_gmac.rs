// SPDX-License-Identifier: GPL-2.0-only
/*
 * AES-GMAC for IEEE 802.11 BIP-GMAC-128 and BIP-GMAC-256
 * Copyright 2015, Qualcomm Atheros, Inc.
 */

use core::ffi::c_void;

// C dependencies supplied by the surrounding kernel translation.
#[repr(C)]
pub struct crypto_aead {
    _private: [u8; 0],
}

#[repr(C)]
pub struct aead_request {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scatterlist {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn crypto_aead_reqsize(tfm: *mut crypto_aead) -> usize;
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree_sensitive(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, count: usize) -> *mut c_void;
    fn memset(dest: *mut c_void, value: i32, count: usize) -> *mut c_void;
    fn sg_init_table(sgl: *mut scatterlist, nents: u32);
    fn sg_set_buf(sg: *mut scatterlist, buf: *const c_void, buflen: usize);
    fn aead_request_set_tfm(req: *mut aead_request, tfm: *mut crypto_aead);
    fn aead_request_set_crypt(
        req: *mut aead_request,
        src: *mut scatterlist,
        dst: *mut scatterlist,
        cryptlen: usize,
        iv: *mut u8,
    );
    fn aead_request_set_ad(req: *mut aead_request, assoclen: usize);
    fn crypto_aead_encrypt(req: *mut aead_request) -> i32;
    fn crypto_alloc_aead(alg_name: *const u8, type_: u32, mask: u32) -> *mut crypto_aead;
    fn is_err(ptr: *const c_void) -> bool;
    fn crypto_aead_setkey(tfm: *mut crypto_aead, key: *const u8, key_len: usize) -> i32;
    fn crypto_aead_setauthsize(tfm: *mut crypto_aead, authsize: usize) -> i32;
    fn crypto_free_aead(tfm: *mut crypto_aead);
    fn err_ptr(err: i32) -> *mut crypto_aead;
    fn ieee80211_is_beacon(fc: u16) -> bool;
}

const AES_BLOCK_SIZE: usize = 16;
const GMAC_AAD_LEN: usize = 20;
const GMAC_NONCE_LEN: usize = 12;
const IEEE80211_GMAC_MIC_LEN: usize = 16;
const GFP_ATOMIC: u32 = 0;
const CRYPTO_ALG_ASYNC: u32 = 0;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;

pub unsafe fn ieee80211_aes_gmac(
    tfm: *mut crypto_aead,
    aad: *const u8,
    nonce: *mut u8,
    data: *const u8,
    data_len: usize,
    mic: *mut u8,
) -> i32 {
    let mut sg: [scatterlist; 5] = core::mem::MaybeUninit::uninit().assume_init();
    let mut iv = [0u8; AES_BLOCK_SIZE];
    let reqsize = core::mem::size_of::<aead_request>() + crypto_aead_reqsize(tfm);
    let aead_req = kzalloc(reqsize + IEEE80211_GMAC_MIC_LEN + GMAC_AAD_LEN, GFP_ATOMIC)
        as *mut aead_request;
    if aead_req.is_null() {
        return -ENOMEM;
    }

    let zero = (aead_req as *mut u8).add(reqsize);
    let __aad = zero.add(IEEE80211_GMAC_MIC_LEN);
    memcpy(__aad as *mut c_void, aad as *const c_void, GMAC_AAD_LEN);

    let fc = (aad as *const u16).read_unaligned();
    if ieee80211_is_beacon(fc) {
        sg_init_table(sg.as_mut_ptr(), 5);
        sg_set_buf(sg.as_mut_ptr().add(0), __aad as *const c_void, GMAC_AAD_LEN);
        sg_set_buf(sg.as_mut_ptr().add(1), zero as *const c_void, 8);
        sg_set_buf(sg.as_mut_ptr().add(2), data.add(8) as *const c_void, data_len - 8 - IEEE80211_GMAC_MIC_LEN);
        sg_set_buf(sg.as_mut_ptr().add(3), zero as *const c_void, IEEE80211_GMAC_MIC_LEN);
        sg_set_buf(sg.as_mut_ptr().add(4), mic as *const c_void, IEEE80211_GMAC_MIC_LEN);
    } else {
        sg_init_table(sg.as_mut_ptr(), 4);
        sg_set_buf(sg.as_mut_ptr().add(0), __aad as *const c_void, GMAC_AAD_LEN);
        sg_set_buf(sg.as_mut_ptr().add(1), data as *const c_void, data_len - IEEE80211_GMAC_MIC_LEN);
        sg_set_buf(sg.as_mut_ptr().add(2), zero as *const c_void, IEEE80211_GMAC_MIC_LEN);
        sg_set_buf(sg.as_mut_ptr().add(3), mic as *const c_void, IEEE80211_GMAC_MIC_LEN);
    }

    memcpy(iv.as_mut_ptr() as *mut c_void, nonce as *const c_void, GMAC_NONCE_LEN);
    memset(iv.as_mut_ptr().add(GMAC_NONCE_LEN) as *mut c_void, 0, AES_BLOCK_SIZE - GMAC_NONCE_LEN);
    iv[AES_BLOCK_SIZE - 1] = 0x01;

    aead_request_set_tfm(aead_req, tfm);
    aead_request_set_crypt(aead_req, sg.as_mut_ptr(), sg.as_mut_ptr(), 0, iv.as_mut_ptr());
    aead_request_set_ad(aead_req, GMAC_AAD_LEN + data_len);
    let ret = crypto_aead_encrypt(aead_req);
    kfree_sensitive(aead_req as *mut c_void);
    ret
}

pub unsafe fn ieee80211_aes_gmac_key_setup(key: *const u8, key_len: usize) -> *mut crypto_aead {
    let tfm = crypto_alloc_aead(b"gcm(aes)\0".as_ptr(), 0, CRYPTO_ALG_ASYNC);
    if is_err(tfm as *const c_void) {
        return tfm;
    }
    let mut err = crypto_aead_setkey(tfm, key, key_len);
    if err == 0 {
        err = crypto_aead_setauthsize(tfm, IEEE80211_GMAC_MIC_LEN);
    }
    if err == 0 {
        return tfm;
    }
    crypto_free_aead(tfm);
    err_ptr(err)
}

pub unsafe fn ieee80211_aes_gmac_key_free(tfm: *mut crypto_aead) {
    crypto_free_aead(tfm);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
