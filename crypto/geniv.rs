// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * geniv: Shared IV generator code
 *
 * This file provides common code to IV generators such as seqiv.
 *
 * Copyright (c) 2007-2019 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

unsafe fn aead_geniv_setkey(
    tfm: *mut crypto_aead,
    key: *const u8,
    keylen: u32,
) -> i32 {
    let ctx = crypto_aead_ctx(tfm);
    crypto_aead_setkey((*ctx).child, key, keylen)
}

unsafe fn aead_geniv_setauthsize(tfm: *mut crypto_aead, authsize: u32) -> i32 {
    let ctx = crypto_aead_ctx(tfm);
    crypto_aead_setauthsize((*ctx).child, authsize)
}

unsafe fn aead_geniv_free(inst: *mut aead_instance) {
    crypto_drop_aead(aead_instance_ctx(inst));
    kfree(inst as *mut core::ffi::c_void);
}

unsafe fn aead_geniv_alloc(
    tmpl: *mut crypto_template,
    tb: *mut *mut rtattr,
) -> *mut aead_instance {
    let mut spawn: *mut crypto_aead_spawn;
    let mut inst: *mut aead_instance;
    let mut alg: *mut aead_alg;
    let mut ivsize: u32;
    let mut maxauthsize: u32;
    let mut mask: u32 = 0;
    let mut err: i32;

    err = crypto_check_attr_type(tb, CRYPTO_ALG_TYPE_AEAD, &mut mask);
    if err != 0 {
        return ERR_PTR(err);
    }

    inst = kzalloc(
        core::mem::size_of::<aead_instance>() + core::mem::size_of::<crypto_aead_spawn>(),
        GFP_KERNEL,
    ) as *mut aead_instance;
    if inst.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    spawn = aead_instance_ctx(inst);

    err = crypto_grab_aead(
        spawn,
        aead_crypto_instance(inst),
        crypto_attr_alg_name(*tb.add(1)),
        0,
        mask,
    );
    if err != 0 {
        aead_geniv_free(inst);
        return ERR_PTR(err);
    }

    alg = crypto_spawn_aead_alg(spawn);
    ivsize = crypto_aead_alg_ivsize(alg);
    maxauthsize = crypto_aead_alg_maxauthsize(alg);

    err = -EINVAL;
    if ivsize < core::mem::size_of::<u64>() as u32 {
        aead_geniv_free(inst);
        return ERR_PTR(err);
    }

    err = -ENAMETOOLONG;
    if snprintf(
        (*inst).alg.base.cra_name.as_mut_ptr(),
        CRYPTO_MAX_ALG_NAME,
        c"%s(%s)".as_ptr(),
        (*tmpl).name,
        (*alg).base.cra_name,
    ) >= CRYPTO_MAX_ALG_NAME
    {
        aead_geniv_free(inst);
        return ERR_PTR(err);
    }
    if snprintf(
        (*inst).alg.base.cra_driver_name.as_mut_ptr(),
        CRYPTO_MAX_ALG_NAME,
        c"%s(%s)".as_ptr(),
        (*tmpl).name,
        (*alg).base.cra_driver_name,
    ) >= CRYPTO_MAX_ALG_NAME
    {
        aead_geniv_free(inst);
        return ERR_PTR(err);
    }

    (*inst).alg.base.cra_priority = (*alg).base.cra_priority;
    (*inst).alg.base.cra_blocksize = (*alg).base.cra_blocksize;
    (*inst).alg.base.cra_alignmask = (*alg).base.cra_alignmask;
    (*inst).alg.base.cra_ctxsize = core::mem::size_of::<aead_geniv_ctx>();
    (*inst).alg.setkey = Some(aead_geniv_setkey);
    (*inst).alg.setauthsize = Some(aead_geniv_setauthsize);
    (*inst).alg.ivsize = ivsize;
    (*inst).alg.maxauthsize = maxauthsize;
    (*inst).free = Some(aead_geniv_free);

    inst
}

unsafe fn aead_init_geniv(aead: *mut crypto_aead) -> i32 {
    let ctx = crypto_aead_ctx(aead);
    let inst = aead_alg_instance(aead);
    let mut child: *mut crypto_aead;
    let mut err = crypto_stdrng_get_bytes((*ctx).salt.as_mut_ptr(), crypto_aead_ivsize(aead));
    if err != 0 {
        return err;
    }

    child = crypto_spawn_aead(aead_instance_ctx(inst));
    err = PTR_ERR(child);
    if IS_ERR(child) {
        return err;
    }

    (*ctx).child = child;
    crypto_aead_set_reqsize(
        aead,
        crypto_aead_reqsize(child) + core::mem::size_of::<aead_request>() as u32,
    );
    err = 0;
    err
}

unsafe fn aead_exit_geniv(tfm: *mut crypto_aead) {
    let ctx = crypto_aead_ctx(tfm);
    crypto_free_aead((*ctx).child);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
