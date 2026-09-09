// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Shared crypto simd helpers
 *
 * Copyright (c) 2012 Jussi Kivilinna <jussi.kivilinna@mbnet.fi>
 * Copyright (c) 2016 Herbert Xu <herbert@gondor.apana.org.au>
 * Copyright (c) 2019 Google LLC
 *
 * Based on aesni-intel_glue.c by:
 *  Copyright (C) 2008, Intel Corp.
 *    Author: Huang Ying <ying.huang@intel.com>
 */

/*
 * Shared crypto SIMD helpers. These functions dynamically create and register
 * an AEAD algorithm that wraps another, internal algorithm. The wrapper
 * ensures that the internal algorithm is only executed in a context where SIMD
 * instructions are usable, i.e. where may_use_simd() returns true. If SIMD is
 * already usable, the wrapper directly calls the internal algorithm. Otherwise
 * it defers execution to a workqueue via cryptd.
 *
 * This is an alternative to the internal algorithm implementing a fallback for
 * the !may_use_simd() case itself.
 *
 * Note that the wrapper algorithm is asynchronous, i.e. it has the
 * CRYPTO_ALG_ASYNC flag set. Therefore it won't be found by users who
 * explicitly allocate a synchronous algorithm.
 */

#[repr(C)]
pub struct simd_aead_alg {
    pub ialg_name: *const core::ffi::c_char,
    pub alg: aead_alg,
}

#[repr(C)]
pub struct simd_aead_ctx {
    pub cryptd_tfm: *mut cryptd_aead,
}

unsafe fn simd_aead_setkey(tfm: *mut crypto_aead, key: *const u8, key_len: u32) -> i32 {
    let ctx = crypto_aead_ctx(tfm);
    let child = &mut (*(*ctx).cryptd_tfm).base as *mut crypto_aead;

    crypto_aead_clear_flags(child, CRYPTO_TFM_REQ_MASK);
    crypto_aead_set_flags(child, crypto_aead_get_flags(tfm) & CRYPTO_TFM_REQ_MASK);
    crypto_aead_setkey(child, key, key_len)
}

unsafe fn simd_aead_setauthsize(tfm: *mut crypto_aead, authsize: u32) -> i32 {
    let ctx = crypto_aead_ctx(tfm);
    let child = &mut (*(*ctx).cryptd_tfm).base as *mut crypto_aead;

    crypto_aead_setauthsize(child, authsize)
}

unsafe fn simd_aead_encrypt(req: *mut aead_request) -> i32 {
    let tfm = crypto_aead_reqtfm(req);
    let ctx = crypto_aead_ctx(tfm);
    let subreq: *mut aead_request = aead_request_ctx(req);
    *subreq = *req;

    let child = if !crypto_simd_usable()
        || (in_atomic() && cryptd_aead_queued((*ctx).cryptd_tfm))
    {
        &mut (*(*ctx).cryptd_tfm).base as *mut crypto_aead
    } else {
        cryptd_aead_child((*ctx).cryptd_tfm)
    };

    aead_request_set_tfm(subreq, child);
    crypto_aead_encrypt(subreq)
}

unsafe fn simd_aead_decrypt(req: *mut aead_request) -> i32 {
    let tfm = crypto_aead_reqtfm(req);
    let ctx = crypto_aead_ctx(tfm);
    let subreq: *mut aead_request = aead_request_ctx(req);
    *subreq = *req;

    let child = if !crypto_simd_usable()
        || (in_atomic() && cryptd_aead_queued((*ctx).cryptd_tfm))
    {
        &mut (*(*ctx).cryptd_tfm).base as *mut crypto_aead
    } else {
        cryptd_aead_child((*ctx).cryptd_tfm)
    };

    aead_request_set_tfm(subreq, child);
    crypto_aead_decrypt(subreq)
}

unsafe fn simd_aead_exit(tfm: *mut crypto_aead) {
    let ctx = crypto_aead_ctx(tfm);
    cryptd_free_aead((*ctx).cryptd_tfm);
}

unsafe fn simd_aead_init(tfm: *mut crypto_aead) -> i32 {
    let ctx = crypto_aead_ctx(tfm);
    let alg: *mut aead_alg = crypto_aead_alg(tfm);
    let salg = container_of!(alg, simd_aead_alg, alg);
    let cryptd_tfm = cryptd_alloc_aead((*salg).ialg_name, CRYPTO_ALG_INTERNAL, CRYPTO_ALG_INTERNAL);
    if IS_ERR(cryptd_tfm) {
        return PTR_ERR(cryptd_tfm);
    }

    (*ctx).cryptd_tfm = cryptd_tfm;

    let reqsize = core::cmp::max(
        crypto_aead_reqsize(cryptd_aead_child(cryptd_tfm)),
        crypto_aead_reqsize(&mut (*cryptd_tfm).base),
    ) + core::mem::size_of::<aead_request>();
    crypto_aead_set_reqsize(tfm, reqsize);
    0
}

unsafe fn simd_aead_create_compat(
    ialg: *mut aead_alg,
    algname: *const core::ffi::c_char,
    drvname: *const core::ffi::c_char,
    basename: *const core::ffi::c_char,
) -> *mut simd_aead_alg {
    let salg = kzalloc_obj::<simd_aead_alg>();
    if salg.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    (*salg).ialg_name = basename;
    let alg = &mut (*salg).alg;
    let mut err = -ENAMETOOLONG;

    if snprintf((*alg).base.cra_name.as_mut_ptr(), CRYPTO_MAX_ALG_NAME, c"%s", algname)
        >= CRYPTO_MAX_ALG_NAME
    {
        kfree(salg as *mut core::ffi::c_void);
        return ERR_PTR(err);
    }
    if snprintf((*alg).base.cra_driver_name.as_mut_ptr(), CRYPTO_MAX_ALG_NAME, c"%s", drvname)
        >= CRYPTO_MAX_ALG_NAME
    {
        kfree(salg as *mut core::ffi::c_void);
        return ERR_PTR(err);
    }

    (*alg).base.cra_flags = CRYPTO_ALG_ASYNC | ((*ialg).base.cra_flags & CRYPTO_ALG_INHERITED_FLAGS);
    (*alg).base.cra_priority = (*ialg).base.cra_priority;
    (*alg).base.cra_blocksize = (*ialg).base.cra_blocksize;
    (*alg).base.cra_alignmask = (*ialg).base.cra_alignmask;
    (*alg).base.cra_module = (*ialg).base.cra_module;
    (*alg).base.cra_ctxsize = core::mem::size_of::<simd_aead_ctx>();
    (*alg).ivsize = (*ialg).ivsize;
    (*alg).maxauthsize = (*ialg).maxauthsize;
    (*alg).chunksize = (*ialg).chunksize;
    (*alg).init = Some(simd_aead_init);
    (*alg).exit = Some(simd_aead_exit);
    (*alg).setkey = Some(simd_aead_setkey);
    (*alg).setauthsize = Some(simd_aead_setauthsize);
    (*alg).encrypt = Some(simd_aead_encrypt);
    (*alg).decrypt = Some(simd_aead_decrypt);

    err = crypto_register_aead(alg);
    if err != 0 {
        kfree(salg as *mut core::ffi::c_void);
        return ERR_PTR(err);
    }
    salg
}

unsafe fn simd_aead_free(salg: *mut simd_aead_alg) {
    crypto_unregister_aead(&mut (*salg).alg);
    kfree(salg as *mut core::ffi::c_void);
}

pub unsafe fn simd_register_aeads_compat(
    algs: *mut aead_alg,
    count: i32,
    simd_algs: *mut *mut simd_aead_alg,
) -> i32 {
    for i in 0..count {
        let alg = algs.add(i as usize);
        if WARN_ON(strncmp((*alg).base.cra_name.as_ptr(), c"__", 2) != 0
            || strncmp((*alg).base.cra_driver_name.as_ptr(), c"__", 2) != 0)
        {
            return -EINVAL;
        }
    }

    let err = crypto_register_aeads(algs, count);
    if err != 0 {
        return err;
    }

    for i in 0..count {
        let alg = algs.add(i as usize);
        let algname = (*alg).base.cra_name.as_ptr().add(2);
        let drvname = (*alg).base.cra_driver_name.as_ptr().add(2);
        let basename = (*alg).base.cra_driver_name.as_ptr();
        let simd = simd_aead_create_compat(alg, algname, drvname, basename);
        if IS_ERR(simd) {
            simd_unregister_aeads(algs, count, simd_algs);
            return PTR_ERR(simd);
        }
        *simd_algs.add(i as usize) = simd;
    }
    0
}

pub unsafe fn simd_unregister_aeads(
    algs: *mut aead_alg,
    count: i32,
    simd_algs: *mut *mut simd_aead_alg,
) {
    crypto_unregister_aeads(algs, count);
    for i in 0..count {
        let slot = simd_algs.add(i as usize);
        if !(*slot).is_null() {
            simd_aead_free(*slot);
            *slot = core::ptr::null_mut();
        }
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
