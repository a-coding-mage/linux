// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2021 sigma star gmbh
 */

// C dependencies:
// <crypto/aead.h>, <crypto/aes.h>, <crypto/algapi.h>, <crypto/gcm.h>,
// <crypto/skcipher.h>, <keys/trusted-type.h>, <linux/key-type.h>,
// <linux/module.h>, <linux/printk.h>, <linux/random.h>,
// <linux/scatterlist.h>, <soc/fsl/dcp.h>

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const DCP_BLOB_VERSION: u8 = 1;
const DCP_BLOB_AUTHLEN: usize = 16;

type U8 = u8;
type U32 = u32;
type Le32 = u32;

/**
 * DOC: dcp blob format
 *
 * The Data Co-Processor (DCP) provides hardware-bound AES keys using its
 * AES encryption engine only. It does not provide direct key sealing/unsealing.
 * To make DCP hardware encryption keys usable as trust source, we define
 * our own custom format that uses a hardware-bound key to secure the sealing
 * key stored in the key blob.
 *
 * Whenever a new trusted key using DCP is generated, we generate a random 128-bit
 * blob encryption key (BEK) and 128-bit nonce. The BEK and nonce are used to
 * encrypt the trusted key payload using AES-128-GCM.
 *
 * The BEK itself is encrypted using the hardware-bound key using the DCP's AES
 * encryption engine with AES-128-ECB. The encrypted BEK, generated nonce,
 * BEK-encrypted payload and authentication tag make up the blob format together
 * with a version number, payload length and authentication tag.
 */

/**
 * struct dcp_blob_fmt - DCP BLOB format.
 *
 * @fmt_version: Format version, currently being %1.
 * @blob_key: Random AES 128 key which is used to encrypt @payload,
 *            @blob_key itself is encrypted with OTP or UNIQUE device key in
 *            AES-128-ECB mode by DCP.
 * @nonce: Random nonce used for @payload encryption.
 * @payload_len: Length of the plain text @payload.
 * @payload: The payload itself, encrypted using AES-128-GCM and @blob_key,
 *           GCM auth tag of size DCP_BLOB_AUTHLEN is attached at the end of it.
 *
 * The total size of a DCP BLOB is sizeof(struct dcp_blob_fmt) + @payload_len +
 * DCP_BLOB_AUTHLEN.
 */
#[repr(C, packed)]
pub struct dcp_blob_fmt {
    pub fmt_version: U8,
    pub blob_key: [U8; AES_KEYSIZE_128],
    pub nonce: [U8; AES_KEYSIZE_128],
    pub payload_len: Le32,
    pub payload: [U8; 0],
}

#[repr(C)]
pub struct trusted_key_payload {
    pub key: *mut U8,
    pub key_len: c_uint,
    pub blob: *mut U8,
    pub blob_len: c_uint,
}

#[repr(C)]
pub struct trusted_key_ops {
    pub exit: Option<unsafe extern "C" fn()>,
    pub init: Option<unsafe extern "C" fn() -> c_int>,
    pub seal: Option<unsafe extern "C" fn(*mut trusted_key_payload, *mut c_char) -> c_int>,
    pub unseal: Option<unsafe extern "C" fn(*mut trusted_key_payload, *mut c_char) -> c_int>,
    pub migratable: c_int,
}

#[repr(C)]
pub struct skcipher_request {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scatterlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_skcipher {
    _private: [u8; 0],
}

#[repr(C)]
pub struct aead_request {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_aead {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_wait {
    _private: [u8; 0],
}

static mut use_otp_key: bool = false;
// module_param_named(dcp_use_otp_key, use_otp_key, bool, 0);
// MODULE_PARM_DESC(dcp_use_otp_key, "Use OTP instead of UNIQUE key for sealing");

static mut skip_zk_test: bool = false;
// module_param_named(dcp_skip_zk_test, skip_zk_test, bool, 0);
// MODULE_PARM_DESC(dcp_skip_zk_test, "Don't test whether device keys are zero'ed");

unsafe extern "C" {
    static mut key_type_trusted: c_void;

    fn crypto_alloc_skcipher(
        alg_name: *const c_char,
        type_: c_uint,
        mask: c_uint,
    ) -> *mut crypto_skcipher;
    fn crypto_free_skcipher(tfm: *mut crypto_skcipher);
    fn skcipher_request_alloc(
        tfm: *mut crypto_skcipher,
        gfp: c_uint,
    ) -> *mut skcipher_request;
    fn skcipher_request_free(req: *mut skcipher_request);
    fn skcipher_request_set_callback(
        req: *mut skcipher_request,
        flags: c_uint,
        compl: unsafe extern "C" fn(*mut c_void, c_int),
        data: *mut c_void,
    );
    fn crypto_req_done(req: *mut c_void, err: c_int);
    fn crypto_skcipher_setkey(
        tfm: *mut crypto_skcipher,
        key: *const U8,
        keylen: c_uint,
    ) -> c_int;
    fn sg_init_one(sg: *mut scatterlist, buf: *const c_void, buflen: c_uint);
    fn skcipher_request_set_crypt(
        req: *mut skcipher_request,
        src: *mut scatterlist,
        dst: *mut scatterlist,
        cryptlen: c_uint,
        iv: *mut U8,
    );
    fn crypto_skcipher_encrypt(req: *mut skcipher_request) -> c_int;
    fn crypto_skcipher_decrypt(req: *mut skcipher_request) -> c_int;
    fn crypto_wait_req(err: c_int, wait: *mut crypto_wait) -> c_int;

    fn crypto_alloc_aead(alg_name: *const c_char, type_: c_uint, mask: c_uint) -> *mut crypto_aead;
    fn crypto_free_aead(tfm: *mut crypto_aead);
    fn crypto_aead_setauthsize(tfm: *mut crypto_aead, authsize: c_uint) -> c_int;
    fn aead_request_alloc(tfm: *mut crypto_aead, gfp: c_uint) -> *mut aead_request;
    fn aead_request_free(req: *mut aead_request);
    fn aead_request_set_crypt(
        req: *mut aead_request,
        src: *mut scatterlist,
        dst: *mut scatterlist,
        cryptlen: c_uint,
        iv: *mut U8,
    );
    fn aead_request_set_callback(
        req: *mut aead_request,
        flags: c_uint,
        compl: unsafe extern "C" fn(*mut c_void, c_int),
        data: *mut c_void,
    );
    fn aead_request_set_ad(req: *mut aead_request, assoclen: c_uint);
    fn crypto_aead_setkey(tfm: *mut crypto_aead, key: *const U8, keylen: c_uint) -> c_int;
    fn crypto_aead_encrypt(req: *mut aead_request) -> c_int;
    fn crypto_aead_decrypt(req: *mut aead_request) -> c_int;

    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn memzero_explicit(ptr: *mut c_void, size: usize);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn get_random_bytes(buf: *mut c_void, nbytes: c_int);
    fn put_unaligned_le32(val: U32, p: *mut Le32);
    fn le32_to_cpu(val: Le32) -> U32;
    fn register_key_type(ktype: *mut c_void) -> c_int;
    fn unregister_key_type(ktype: *mut c_void);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
}

unsafe fn is_err<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn ptr_err<T>(ptr: *mut T) -> c_int {
    ptr as isize as c_int
}

unsafe fn calc_blob_len(payload_len: c_uint) -> usize {
    size_of::<dcp_blob_fmt>() + payload_len as usize + DCP_BLOB_AUTHLEN
}

unsafe extern "C" fn do_dcp_crypto(in_: *mut U8, out: *mut U8, do_encrypt: bool) -> c_int {
    let mut req: *mut skcipher_request = ptr::null_mut();
    let mut src_sg: scatterlist = core::mem::zeroed();
    let mut dst_sg: scatterlist = core::mem::zeroed();
    let mut tfm: *mut crypto_skcipher;
    let mut paes_key: [U8; DCP_PAES_KEYSIZE] = [0; DCP_PAES_KEYSIZE];
    let mut wait: crypto_wait = core::mem::zeroed();
    let mut res: c_int = 0;

    if use_otp_key {
        paes_key[0] = DCP_PAES_KEY_OTP;
    } else {
        paes_key[0] = DCP_PAES_KEY_UNIQUE;
    }

    tfm = crypto_alloc_skcipher(
        c"ecb-paes-dcp".as_ptr(),
        CRYPTO_ALG_INTERNAL,
        CRYPTO_ALG_INTERNAL,
    );
    if is_err(tfm) {
        res = ptr_err(tfm);
        tfm = ptr::null_mut();
        goto_out_dcp(req, tfm);
        return res;
    }

    req = skcipher_request_alloc(tfm, GFP_NOFS);
    if req.is_null() {
        res = -ENOMEM;
        goto_out_dcp(req, tfm);
        return res;
    }

    skcipher_request_set_callback(
        req,
        CRYPTO_TFM_REQ_MAY_BACKLOG | CRYPTO_TFM_REQ_MAY_SLEEP,
        crypto_req_done,
        &mut wait as *mut _ as *mut c_void,
    );
    res = crypto_skcipher_setkey(
        tfm,
        paes_key.as_ptr(),
        size_of::<[U8; DCP_PAES_KEYSIZE]>() as c_uint,
    );
    if res < 0 {
        goto_out_dcp(req, tfm);
        return res;
    }

    sg_init_one(&mut src_sg, in_ as *const c_void, AES_KEYSIZE_128 as c_uint);
    sg_init_one(&mut dst_sg, out as *const c_void, AES_KEYSIZE_128 as c_uint);
    skcipher_request_set_crypt(
        req,
        &mut src_sg,
        &mut dst_sg,
        AES_KEYSIZE_128 as c_uint,
        ptr::null_mut(),
    );

    if do_encrypt {
        res = crypto_wait_req(crypto_skcipher_encrypt(req), &mut wait);
    } else {
        res = crypto_wait_req(crypto_skcipher_decrypt(req), &mut wait);
    }

    goto_out_dcp(req, tfm);
    res
}

unsafe fn goto_out_dcp(req: *mut skcipher_request, tfm: *mut crypto_skcipher) {
    skcipher_request_free(req);
    crypto_free_skcipher(tfm);
}

unsafe extern "C" fn do_aead_crypto(
    in_: *mut U8,
    out: *mut U8,
    len: usize,
    key: *mut U8,
    nonce: *mut U8,
    do_encrypt: bool,
) -> c_int {
    let mut aead_req: *mut aead_request = ptr::null_mut();
    let mut src_sg: scatterlist = core::mem::zeroed();
    let mut dst_sg: scatterlist = core::mem::zeroed();
    let aead: *mut crypto_aead;
    let mut ret: c_int;
    let mut wait: crypto_wait = core::mem::zeroed();

    aead = crypto_alloc_aead(c"gcm(aes)".as_ptr(), 0, CRYPTO_ALG_ASYNC);
    if is_err(aead) {
        ret = ptr_err(aead);
        return ret;
    }

    ret = crypto_aead_setauthsize(aead, DCP_BLOB_AUTHLEN as c_uint);
    if ret < 0 {
        pr_err(c"Can't set crypto auth tag len: %d\n".as_ptr(), ret);
        crypto_free_aead(aead);
        return ret;
    }

    aead_req = aead_request_alloc(aead, GFP_KERNEL);
    if aead_req.is_null() {
        ret = -ENOMEM;
        crypto_free_aead(aead);
        return ret;
    }

    sg_init_one(&mut src_sg, in_ as *const c_void, len as c_uint);
    if do_encrypt {
        /*
         * If we encrypt our buffer has extra space for the auth tag.
         */
        sg_init_one(
            &mut dst_sg,
            out as *const c_void,
            (len + DCP_BLOB_AUTHLEN) as c_uint,
        );
    } else {
        sg_init_one(&mut dst_sg, out as *const c_void, len as c_uint);
    }

    aead_request_set_crypt(aead_req, &mut src_sg, &mut dst_sg, len as c_uint, nonce);
    aead_request_set_callback(
        aead_req,
        CRYPTO_TFM_REQ_MAY_SLEEP,
        crypto_req_done,
        &mut wait as *mut _ as *mut c_void,
    );
    aead_request_set_ad(aead_req, 0);

    if crypto_aead_setkey(aead, key, AES_KEYSIZE_128 as c_uint) != 0 {
        pr_err(c"Can't set crypto AEAD key\n".as_ptr());
        ret = -EINVAL;
        aead_request_free(aead_req);
        crypto_free_aead(aead);
        return ret;
    }

    if do_encrypt {
        ret = crypto_wait_req(crypto_aead_encrypt(aead_req), &mut wait);
    } else {
        ret = crypto_wait_req(crypto_aead_decrypt(aead_req), &mut wait);
    }

    aead_request_free(aead_req);
    crypto_free_aead(aead);
    ret
}

unsafe extern "C" fn decrypt_blob_key(encrypted_key: *mut U8, plain_key: *mut U8) -> c_int {
    do_dcp_crypto(encrypted_key, plain_key, false)
}

unsafe extern "C" fn encrypt_blob_key(plain_key: *mut U8, encrypted_key: *mut U8) -> c_int {
    do_dcp_crypto(plain_key, encrypted_key, true)
}

unsafe extern "C" fn trusted_dcp_seal(
    p: *mut trusted_key_payload,
    _datablob: *mut c_char,
) -> c_int {
    let b: *mut dcp_blob_fmt = (*p).blob as *mut dcp_blob_fmt;
    let blen: usize;
    let mut ret: c_int;
    let plain_blob_key: *mut U8;

    blen = calc_blob_len((*p).key_len);
    if blen > MAX_BLOB_SIZE {
        return -E2BIG;
    }

    plain_blob_key = kmalloc(AES_KEYSIZE_128, GFP_KERNEL) as *mut U8;
    if plain_blob_key.is_null() {
        return -ENOMEM;
    }

    (*b).fmt_version = DCP_BLOB_VERSION;
    get_random_bytes((*b).nonce.as_mut_ptr() as *mut c_void, AES_KEYSIZE_128 as c_int);
    get_random_bytes(plain_blob_key as *mut c_void, AES_KEYSIZE_128 as c_int);

    ret = do_aead_crypto(
        (*p).key,
        (*b).payload.as_mut_ptr(),
        (*p).key_len as usize,
        plain_blob_key,
        (*b).nonce.as_mut_ptr(),
        true,
    );
    if ret != 0 {
        pr_err(c"Unable to encrypt blob payload: %i\n".as_ptr(), ret);
        memzero_explicit(plain_blob_key as *mut c_void, AES_KEYSIZE_128);
        kfree(plain_blob_key as *const c_void);
        return ret;
    }

    ret = encrypt_blob_key(plain_blob_key, (*b).blob_key.as_mut_ptr());
    if ret != 0 {
        pr_err(c"Unable to encrypt blob key: %i\n".as_ptr(), ret);
        memzero_explicit(plain_blob_key as *mut c_void, AES_KEYSIZE_128);
        kfree(plain_blob_key as *const c_void);
        return ret;
    }

    put_unaligned_le32((*p).key_len, ptr::addr_of_mut!((*b).payload_len));
    (*p).blob_len = blen as c_uint;
    ret = 0;

    memzero_explicit(plain_blob_key as *mut c_void, AES_KEYSIZE_128);
    kfree(plain_blob_key as *const c_void);

    ret
}

unsafe extern "C" fn trusted_dcp_unseal(
    p: *mut trusted_key_payload,
    _datablob: *mut c_char,
) -> c_int {
    let b: *mut dcp_blob_fmt = (*p).blob as *mut dcp_blob_fmt;
    let blen: usize;
    let mut ret: c_int;
    let mut plain_blob_key: *mut U8 = ptr::null_mut();

    if (*b).fmt_version != DCP_BLOB_VERSION {
        pr_err(
            c"DCP blob has bad version: %i, expected %i\n".as_ptr(),
            (*b).fmt_version as c_int,
            DCP_BLOB_VERSION as c_int,
        );
        ret = -EINVAL;
        return ret;
    }

    (*p).key_len = le32_to_cpu(ptr::addr_of!((*b).payload_len).read_unaligned());
    if (*p).key_len < MIN_KEY_SIZE || (*p).key_len > MAX_KEY_SIZE {
        ret = -EINVAL;
        return ret;
    }

    blen = calc_blob_len((*p).key_len);
    if blen != (*p).blob_len as usize {
        pr_err(
            c"DCP blob has bad length: %zu != %u\n".as_ptr(),
            blen,
            (*p).blob_len,
        );
        ret = -EINVAL;
        return ret;
    }

    plain_blob_key = kmalloc(AES_KEYSIZE_128, GFP_KERNEL) as *mut U8;
    if plain_blob_key.is_null() {
        ret = -ENOMEM;
        return ret;
    }

    ret = decrypt_blob_key((*b).blob_key.as_mut_ptr(), plain_blob_key);
    if ret != 0 {
        pr_err(c"Unable to decrypt blob key: %i\n".as_ptr(), ret);
        memzero_explicit(plain_blob_key as *mut c_void, AES_KEYSIZE_128);
        kfree(plain_blob_key as *const c_void);
        return ret;
    }

    ret = do_aead_crypto(
        (*b).payload.as_mut_ptr(),
        (*p).key,
        (*p).key_len as usize + DCP_BLOB_AUTHLEN,
        plain_blob_key,
        (*b).nonce.as_mut_ptr(),
        false,
    );
    if ret != 0 {
        pr_err(c"Unwrap of DCP payload failed: %i\n".as_ptr(), ret);
        memzero_explicit(plain_blob_key as *mut c_void, AES_KEYSIZE_128);
        kfree(plain_blob_key as *const c_void);
        return ret;
    }

    ret = 0;
    if !plain_blob_key.is_null() {
        memzero_explicit(plain_blob_key as *mut c_void, AES_KEYSIZE_128);
        kfree(plain_blob_key as *const c_void);
    }

    ret
}

unsafe extern "C" fn test_for_zero_key() -> c_int {
    /*
     * Encrypting a plaintext of all 0x55 bytes will yield
     * this ciphertext in case the DCP test key is used.
     */
    static BAD: [U8; 16] = [
        0x9a, 0xda, 0xe0, 0x54, 0xf6, 0x3d, 0xfa, 0xff, 0x5e, 0xa1, 0x8e, 0x45, 0xed, 0xf6,
        0xea, 0x6f,
    ];
    let mut buf: *mut c_void = ptr::null_mut();
    let mut ret: c_int = 0;

    if skip_zk_test {
        kfree(buf);
        return ret;
    }

    buf = kmalloc(AES_BLOCK_SIZE, GFP_KERNEL);
    if buf.is_null() {
        ret = -ENOMEM;
        kfree(buf);
        return ret;
    }

    memset(buf, 0x55, AES_BLOCK_SIZE);

    ret = do_dcp_crypto(buf as *mut U8, buf as *mut U8, true);
    if ret != 0 {
        kfree(buf);
        return ret;
    }

    if memcmp(buf, BAD.as_ptr() as *const c_void, AES_BLOCK_SIZE) == 0 {
        pr_warn(c"Device neither in secure nor trusted mode!\n".as_ptr());
        ret = -EINVAL;
    }
    kfree(buf);
    ret
}

unsafe extern "C" fn trusted_dcp_init() -> c_int {
    let ret: c_int;

    if use_otp_key {
        pr_info(c"Using DCP OTP key\n".as_ptr());
    }

    ret = test_for_zero_key();
    if ret != 0 {
        pr_warn(c"Test for zero'ed keys failed: %i\n".as_ptr(), ret);

        return -EINVAL;
    }

    register_key_type(&mut key_type_trusted as *mut c_void)
}

unsafe extern "C" fn trusted_dcp_exit() {
    unregister_key_type(&mut key_type_trusted as *mut c_void);
}

pub static mut dcp_trusted_key_ops: trusted_key_ops = trusted_key_ops {
    exit: Some(trusted_dcp_exit),
    init: Some(trusted_dcp_init),
    seal: Some(trusted_dcp_seal),
    unseal: Some(trusted_dcp_unseal),
    migratable: 0,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
