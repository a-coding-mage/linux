// SPDX-License-Identifier: GPL-2.0-only
/*
 * AES XTS routines supporting VMX In-core instructions on Power 8
 *
 * Copyright (C) 2015 International Business Machines Inc.
 *
 * Author: Leonidas S. Barbosa <leosilva@linux.vnet.ibm.com>
 */

// Dependencies supplied by the corresponding kernel headers and aesp8-ppc.h.

#[repr(C)]
pub struct p8_aes_xts_ctx {
    pub fallback: *mut crypto_skcipher,
    pub enc_key: p8_aes_key,
    pub dec_key: p8_aes_key,
    pub tweak_key: p8_aes_key,
}

unsafe fn p8_aes_xts_init(tfm: *mut crypto_skcipher) -> c_int {
    let ctx: *mut p8_aes_xts_ctx = crypto_skcipher_ctx(tfm);
    let fallback: *mut crypto_skcipher;

    fallback = crypto_alloc_skcipher(
        "xts(aes)\0".as_ptr() as *const c_char,
        0,
        CRYPTO_ALG_NEED_FALLBACK | CRYPTO_ALG_ASYNC,
    );
    if IS_ERR(fallback) {
        pr_err!("Failed to allocate xts(aes) fallback: %ld\n", PTR_ERR(fallback));
        return PTR_ERR(fallback);
    }

    crypto_skcipher_set_reqsize(
        tfm,
        core::mem::size_of::<skcipher_request>() + crypto_skcipher_reqsize(fallback),
    );
    (*ctx).fallback = fallback;
    0
}

unsafe fn p8_aes_xts_exit(tfm: *mut crypto_skcipher) {
    let ctx: *mut p8_aes_xts_ctx = crypto_skcipher_ctx(tfm);

    crypto_free_skcipher((*ctx).fallback);
}

unsafe fn p8_aes_xts_setkey(
    tfm: *mut crypto_skcipher,
    key: *const u8,
    keylen: c_uint,
) -> c_int {
    let ctx: *mut p8_aes_xts_ctx = crypto_skcipher_ctx(tfm);
    let mut ret: c_int;

    ret = xts_verify_key(tfm, key, keylen);
    if ret != 0 {
        return ret;
    }

    preempt_disable();
    pagefault_disable();
    enable_kernel_vsx();
    ret = aes_p8_set_encrypt_key(
        key.add((keylen / 2) as usize),
        (keylen / 2) * 8,
        &mut (*ctx).tweak_key,
    );
    ret |= aes_p8_set_encrypt_key(key, (keylen / 2) * 8, &mut (*ctx).enc_key);
    ret |= aes_p8_set_decrypt_key(key, (keylen / 2) * 8, &mut (*ctx).dec_key);
    disable_kernel_vsx();
    pagefault_enable();
    preempt_enable();

    ret |= crypto_skcipher_setkey((*ctx).fallback, key, keylen);

    if ret != 0 { -EINVAL } else { 0 }
}

unsafe fn p8_aes_xts_crypt(req: *mut skcipher_request, enc: c_int) -> c_int {
    let tfm: *mut crypto_skcipher = crypto_skcipher_reqtfm(req);
    let ctx: *const p8_aes_xts_ctx = crypto_skcipher_ctx(tfm);
    let mut walk: skcipher_walk = core::mem::zeroed();
    let mut nbytes: c_uint;
    let mut tweak = [0u8; AES_BLOCK_SIZE];
    let mut ret: c_int;

    if (*req).cryptlen < AES_BLOCK_SIZE {
        return -EINVAL;
    }

    if !crypto_simd_usable() || ((*req).cryptlen % XTS_BLOCK_SIZE) != 0 {
        let subreq: *mut skcipher_request = skcipher_request_ctx(req);
        *subreq = *req;
        skcipher_request_set_tfm(subreq, (*ctx).fallback);
        return if enc != 0 {
            crypto_skcipher_encrypt(subreq)
        } else {
            crypto_skcipher_decrypt(subreq)
        };
    }

    ret = skcipher_walk_virt(&mut walk, req, false);
    if ret != 0 {
        return ret;
    }

    preempt_disable();
    pagefault_disable();
    enable_kernel_vsx();
    aes_p8_encrypt(walk.iv, tweak.as_mut_ptr(), &(*ctx).tweak_key);
    disable_kernel_vsx();
    pagefault_enable();
    preempt_enable();

    while {
        nbytes = walk.nbytes;
        nbytes != 0
    } {
        preempt_disable();
        pagefault_disable();
        enable_kernel_vsx();
        if enc != 0 {
            aes_p8_xts_encrypt(
                walk.src.virt.addr,
                walk.dst.virt.addr,
                round_down(nbytes, AES_BLOCK_SIZE),
                &(*ctx).enc_key,
                core::ptr::null_mut(),
                tweak.as_mut_ptr(),
            );
        } else {
            aes_p8_xts_decrypt(
                walk.src.virt.addr,
                walk.dst.virt.addr,
                round_down(nbytes, AES_BLOCK_SIZE),
                &(*ctx).dec_key,
                core::ptr::null_mut(),
                tweak.as_mut_ptr(),
            );
        }
        disable_kernel_vsx();
        pagefault_enable();
        preempt_enable();

        ret = skcipher_walk_done(&mut walk, nbytes % AES_BLOCK_SIZE);
    }
    ret
}

unsafe fn p8_aes_xts_encrypt(req: *mut skcipher_request) -> c_int {
    p8_aes_xts_crypt(req, 1)
}

unsafe fn p8_aes_xts_decrypt(req: *mut skcipher_request) -> c_int {
    p8_aes_xts_crypt(req, 0)
}

pub static mut p8_aes_xts_alg: skcipher_alg = skcipher_alg {
    base: crypto_alg {
        cra_name: "xts(aes)\0".as_ptr() as *const c_char,
        cra_driver_name: "p8_aes_xts\0".as_ptr() as *const c_char,
        cra_module: THIS_MODULE,
        cra_priority: 2000,
        cra_flags: CRYPTO_ALG_NEED_FALLBACK,
        cra_blocksize: AES_BLOCK_SIZE,
        cra_ctxsize: core::mem::size_of::<p8_aes_xts_ctx>(),
    },
    setkey: Some(p8_aes_xts_setkey),
    encrypt: Some(p8_aes_xts_encrypt),
    decrypt: Some(p8_aes_xts_decrypt),
    init: Some(p8_aes_xts_init),
    exit: Some(p8_aes_xts_exit),
    min_keysize: 2 * AES_MIN_KEY_SIZE,
    max_keysize: 2 * AES_MAX_KEY_SIZE,
    ivsize: AES_BLOCK_SIZE,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
