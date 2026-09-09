// SPDX-License-Identifier: GPL-2.0-only
/*
 * AES CBC routines supporting VMX instructions on the Power 8
 *
 * Copyright (C) 2015 International Business Machines Inc.
 *
 * Author: Marcelo Henrique Cerri <mhcerri@br.ibm.com>
 */

// C dependencies:
// asm/simd.h, asm/switch_to.h, crypto/aes.h, crypto/internal/simd.h,
// crypto/internal/skcipher.h, linux/err.h, linux/kernel.h, linux/module.h,
// linux/uaccess.h, and "aesp8-ppc.h".

#[repr(C)]
pub struct p8_aes_cbc_ctx {
    pub fallback: *mut crypto_skcipher,
    pub enc_key: p8_aes_key,
    pub dec_key: p8_aes_key,
}

unsafe fn p8_aes_cbc_init(tfm: *mut crypto_skcipher) -> i32 {
    let ctx: *mut p8_aes_cbc_ctx = crypto_skcipher_ctx(tfm);
    let fallback: *mut crypto_skcipher = crypto_alloc_skcipher(
        b"cbc(aes)\0".as_ptr() as *const i8,
        0,
        CRYPTO_ALG_NEED_FALLBACK | CRYPTO_ALG_ASYNC,
    );

    if IS_ERR(fallback) {
        pr_err(b"Failed to allocate cbc(aes) fallback: %ld\n\0".as_ptr() as *const i8, PTR_ERR(fallback));
        return PTR_ERR(fallback);
    }

    crypto_skcipher_set_reqsize(
        tfm,
        core::mem::size_of::<skcipher_request>() + crypto_skcipher_reqsize(fallback),
    );
    (*ctx).fallback = fallback;
    0
}

unsafe fn p8_aes_cbc_exit(tfm: *mut crypto_skcipher) {
    let ctx: *mut p8_aes_cbc_ctx = crypto_skcipher_ctx(tfm);
    crypto_free_skcipher((*ctx).fallback);
}

unsafe fn p8_aes_cbc_setkey(
    tfm: *mut crypto_skcipher,
    key: *const u8,
    keylen: u32,
) -> i32 {
    let ctx: *mut p8_aes_cbc_ctx = crypto_skcipher_ctx(tfm);
    let mut ret: i32;

    preempt_disable();
    pagefault_disable();
    enable_kernel_vsx();
    ret = aes_p8_set_encrypt_key(key, keylen * 8, &mut (*ctx).enc_key);
    ret |= aes_p8_set_decrypt_key(key, keylen * 8, &mut (*ctx).dec_key);
    disable_kernel_vsx();
    pagefault_enable();
    preempt_enable();

    ret |= crypto_skcipher_setkey((*ctx).fallback, key, keylen);

    if ret != 0 { -EINVAL } else { 0 }
}

unsafe fn p8_aes_cbc_crypt(req: *mut skcipher_request, enc: bool) -> i32 {
    let tfm: *mut crypto_skcipher = crypto_skcipher_reqtfm(req);
    let ctx: *const p8_aes_cbc_ctx = crypto_skcipher_ctx(tfm);
    let mut walk: skcipher_walk = core::mem::zeroed();
    let mut nbytes: u32;
    let mut ret: i32;

    if !crypto_simd_usable() {
        let subreq: *mut skcipher_request = skcipher_request_ctx(req);
        *subreq = *req;
        skcipher_request_set_tfm(subreq, (*ctx).fallback);
        return if enc {
            crypto_skcipher_encrypt(subreq)
        } else {
            crypto_skcipher_decrypt(subreq)
        };
    }

    ret = skcipher_walk_virt(&mut walk, req, false);
    while {
        nbytes = walk.nbytes;
        nbytes != 0
    } {
        preempt_disable();
        pagefault_disable();
        enable_kernel_vsx();
        aes_p8_cbc_encrypt(
            walk.src.virt.addr,
            walk.dst.virt.addr,
            round_down(nbytes, AES_BLOCK_SIZE),
            if enc { &(*ctx).enc_key } else { &(*ctx).dec_key },
            walk.iv,
            enc,
        );
        disable_kernel_vsx();
        pagefault_enable();
        preempt_enable();

        ret = skcipher_walk_done(&mut walk, nbytes % AES_BLOCK_SIZE);
    }
    ret
}

unsafe fn p8_aes_cbc_encrypt(req: *mut skcipher_request) -> i32 {
    p8_aes_cbc_crypt(req, true)
}

unsafe fn p8_aes_cbc_decrypt(req: *mut skcipher_request) -> i32 {
    p8_aes_cbc_crypt(req, false)
}

pub static mut p8_aes_cbc_alg: skcipher_alg = skcipher_alg {
    base: crypto_alg {
        cra_name: b"cbc(aes)\0".as_ptr() as *const i8,
        cra_driver_name: b"p8_aes_cbc\0".as_ptr() as *const i8,
        cra_module: THIS_MODULE,
        cra_priority: 2000,
        cra_flags: CRYPTO_ALG_NEED_FALLBACK,
        cra_blocksize: AES_BLOCK_SIZE,
        cra_ctxsize: core::mem::size_of::<p8_aes_cbc_ctx>(),
    },
    setkey: Some(p8_aes_cbc_setkey),
    encrypt: Some(p8_aes_cbc_encrypt),
    decrypt: Some(p8_aes_cbc_decrypt),
    init: Some(p8_aes_cbc_init),
    exit: Some(p8_aes_cbc_exit),
    min_keysize: AES_MIN_KEY_SIZE,
    max_keysize: AES_MAX_KEY_SIZE,
    ivsize: AES_BLOCK_SIZE,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
