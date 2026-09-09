// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2003-2004, Instant802 Networks, Inc.
 * Copyright 2005-2006, Devicescape Software, Inc.
 * Copyright 2014-2015, Qualcomm Atheros, Inc.
 *
 * Rewrite: Copyright (C) 2013 Linaro Ltd <ard.biesheuvel@linaro.org>
 */

// Linux kernel and crypto dependencies are supplied externally.

pub unsafe fn aead_encrypt(
    tfm: *mut crypto_aead,
    b_0: *mut u8,
    aad: *mut u8,
    aad_len: usize,
    data: *mut u8,
    data_len: usize,
    mic: *mut u8,
) -> i32 {
    let mic_len = crypto_aead_authsize(tfm);
    let mut sg: [scatterlist; 3] = [core::mem::zeroed(); 3];
    let aead_req: *mut aead_request;
    let reqsize: i32 = core::mem::size_of::<aead_request>() as i32
        + crypto_aead_reqsize(tfm);
    let __aad: *mut u8;
    let ret: i32;

    aead_req = kzalloc((reqsize as usize) + aad_len, GFP_ATOMIC);
    if aead_req.is_null() {
        return -ENOMEM;
    }

    __aad = (aead_req as *mut u8).add(reqsize as usize);
    core::ptr::copy_nonoverlapping(aad, __aad, aad_len);

    sg_init_table(sg.as_mut_ptr(), 3);
    sg_set_buf(&mut sg[0], __aad, aad_len);
    sg_set_buf(&mut sg[1], data, data_len);
    sg_set_buf(&mut sg[2], mic, mic_len);

    aead_request_set_tfm(aead_req, tfm);
    aead_request_set_crypt(aead_req, sg.as_mut_ptr(), sg.as_mut_ptr(), data_len, b_0);
    aead_request_set_ad(aead_req, sg[0].length);

    ret = crypto_aead_encrypt(aead_req);
    kfree_sensitive(aead_req);

    ret
}

pub unsafe fn aead_decrypt(
    tfm: *mut crypto_aead,
    b_0: *mut u8,
    aad: *mut u8,
    aad_len: usize,
    data: *mut u8,
    data_len: usize,
    mic: *mut u8,
) -> i32 {
    let mic_len = crypto_aead_authsize(tfm);
    let mut sg: [scatterlist; 3] = [core::mem::zeroed(); 3];
    let aead_req: *mut aead_request;
    let reqsize: i32 = core::mem::size_of::<aead_request>() as i32
        + crypto_aead_reqsize(tfm);
    let __aad: *mut u8;
    let err: i32;

    if data_len == 0 {
        return -EINVAL;
    }

    aead_req = kzalloc((reqsize as usize) + aad_len, GFP_ATOMIC);
    if aead_req.is_null() {
        return -ENOMEM;
    }

    __aad = (aead_req as *mut u8).add(reqsize as usize);
    core::ptr::copy_nonoverlapping(aad, __aad, aad_len);

    sg_init_table(sg.as_mut_ptr(), 3);
    sg_set_buf(&mut sg[0], __aad, aad_len);
    sg_set_buf(&mut sg[1], data, data_len);
    sg_set_buf(&mut sg[2], mic, mic_len);

    aead_request_set_tfm(aead_req, tfm);
    aead_request_set_crypt(
        aead_req,
        sg.as_mut_ptr(),
        sg.as_mut_ptr(),
        data_len + mic_len,
        b_0,
    );
    aead_request_set_ad(aead_req, sg[0].length);

    err = crypto_aead_decrypt(aead_req);
    kfree_sensitive(aead_req);

    err
}

pub unsafe fn aead_key_setup_encrypt(
    alg: *const core::ffi::c_char,
    key: *const u8,
    key_len: usize,
    mic_len: usize,
) -> *mut crypto_aead {
    let tfm: *mut crypto_aead;
    let err: i32;

    tfm = crypto_alloc_aead(alg, 0, CRYPTO_ALG_ASYNC);
    if IS_ERR(tfm) {
        return tfm;
    }

    err = crypto_aead_setkey(tfm, key, key_len);
    if err != 0 {
        crypto_free_aead(tfm);
        return ERR_PTR(err);
    }
    err = crypto_aead_setauthsize(tfm, mic_len);
    if err != 0 {
        crypto_free_aead(tfm);
        return ERR_PTR(err);
    }

    tfm
}

pub unsafe fn aead_key_free(tfm: *mut crypto_aead) {
    crypto_free_aead(tfm);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
