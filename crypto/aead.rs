// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * AEAD: Authenticated Encryption with Associated Data
 *
 * This file provides API support for AEAD algorithms.
 *
 * Copyright (c) 2007-2015 Herbert Xu <herbert@gondor.apana.org.au>
 */

// C dependencies are supplied by the surrounding kernel translation unit.

unsafe fn setkey_unaligned(tfm: *mut crypto_aead, key: *const u8, keylen: u32) -> i32 {
    let alignmask: c_ulong = crypto_aead_alignmask(tfm);
    let mut ret: i32;
    let absize: c_ulong = keylen as c_ulong + alignmask;
    let buffer: *mut u8 = kmalloc(absize, GFP_ATOMIC);
    if buffer.is_null() {
        return -ENOMEM;
    }

    let alignbuffer = ALIGN(buffer as c_ulong, alignmask + 1) as *mut u8;
    memcpy(alignbuffer, key, keylen as usize);
    ret = ((*crypto_aead_alg(tfm)).setkey)(tfm, alignbuffer, keylen);
    kfree_sensitive(buffer);
    ret
}

pub unsafe fn crypto_aead_setkey(
    tfm: *mut crypto_aead,
    key: *const u8,
    keylen: u32,
) -> i32 {
    let alignmask: c_ulong = crypto_aead_alignmask(tfm);
    let err: i32;

    if (key as c_ulong) & alignmask != 0 {
        err = setkey_unaligned(tfm, key, keylen);
    } else {
        err = ((*crypto_aead_alg(tfm)).setkey)(tfm, key, keylen);
    }

    if unlikely(err != 0) {
        crypto_aead_set_flags(tfm, CRYPTO_TFM_NEED_KEY);
        return err;
    }

    crypto_aead_clear_flags(tfm, CRYPTO_TFM_NEED_KEY);
    0
}

pub unsafe fn crypto_aead_setauthsize(tfm: *mut crypto_aead, authsize: u32) -> i32 {
    let err: i32;
    if (authsize == 0 && crypto_aead_maxauthsize(tfm) != 0)
        || authsize > crypto_aead_maxauthsize(tfm)
    {
        return -EINVAL;
    }

    if let Some(setauthsize) = (*crypto_aead_alg(tfm)).setauthsize {
        err = setauthsize(tfm, authsize);
        if err != 0 {
            return err;
        }
    }

    (*tfm).authsize = authsize;
    0
}

pub unsafe fn crypto_aead_encrypt(req: *mut aead_request) -> i32 {
    let aead = crypto_aead_reqtfm(req);
    if crypto_aead_get_flags(aead) & CRYPTO_TFM_NEED_KEY != 0 {
        return -ENOKEY;
    }
    ((*crypto_aead_alg(aead)).encrypt)(req)
}

pub unsafe fn crypto_aead_decrypt(req: *mut aead_request) -> i32 {
    let aead = crypto_aead_reqtfm(req);
    if crypto_aead_get_flags(aead) & CRYPTO_TFM_NEED_KEY != 0 {
        return -ENOKEY;
    }
    if (*req).cryptlen < crypto_aead_authsize(aead) {
        return -EINVAL;
    }
    ((*crypto_aead_alg(aead)).decrypt)(req)
}

unsafe fn crypto_aead_exit_tfm(tfm: *mut crypto_tfm) {
    let aead = __crypto_aead_cast(tfm);
    let alg = crypto_aead_alg(aead);
    ((*alg).exit)(aead);
}

unsafe fn crypto_aead_init_tfm(tfm: *mut crypto_tfm) -> i32 {
    let aead = __crypto_aead_cast(tfm);
    let alg = crypto_aead_alg(aead);

    crypto_aead_set_flags(aead, CRYPTO_TFM_NEED_KEY);
    crypto_aead_set_reqsize(aead, crypto_tfm_alg_reqsize(tfm));
    (*aead).authsize = (*alg).maxauthsize;

    if (*alg).exit.is_some() {
        (*aead).base.exit = Some(crypto_aead_exit_tfm);
    }
    if let Some(init) = (*alg).init {
        return init(aead);
    }
    0
}

unsafe fn __maybe_unused_crypto_aead_report(
    skb: *mut sk_buff,
    alg: *mut crypto_alg,
) -> i32 {
    let aead = container_of_aead_alg(alg);
    let mut raead = crypto_report_aead {
        type_: "aead".as_ptr() as *const i8,
        geniv: "<none>".as_ptr() as *const i8,
        blocksize: 0,
        maxauthsize: 0,
        ivsize: 0,
    };
    raead.blocksize = (*alg).cra_blocksize;
    raead.maxauthsize = (*aead).maxauthsize;
    raead.ivsize = (*aead).ivsize;
    nla_put(skb, CRYPTOCFGA_REPORT_AEAD, core::mem::size_of::<crypto_report_aead>(), &raead)
}

unsafe fn __maybe_unused_crypto_aead_show(m: *mut seq_file, alg: *mut crypto_alg) {
    let aead = container_of_aead_alg(alg);
    seq_printf(m, "type         : aead\n");
    seq_printf(m, "async        : %s\n", str_yes_no((*alg).cra_flags & CRYPTO_ALG_ASYNC));
    seq_printf(m, "blocksize    : %u\n", (*alg).cra_blocksize);
    seq_printf(m, "ivsize       : %u\n", (*aead).ivsize);
    seq_printf(m, "maxauthsize  : %u\n", (*aead).maxauthsize);
    seq_printf(m, "geniv        : <none>\n");
}

unsafe fn crypto_aead_free_instance(inst: *mut crypto_instance) {
    let aead = aead_instance(inst);
    ((*aead).free)(aead);
}

// #ifdef CONFIG_PROC_FS / #if IS_ENABLED(CONFIG_CRYPTO_USER) are preserved
// as conditional fields by the surrounding configuration.
static crypto_aead_type: crypto_type = crypto_type {
    extsize: crypto_alg_extsize,
    init_tfm: Some(crypto_aead_init_tfm),
    free: Some(crypto_aead_free_instance),
    show: Some(__maybe_unused_crypto_aead_show),
    report: Some(__maybe_unused_crypto_aead_report),
    maskclear: !CRYPTO_ALG_TYPE_MASK,
    maskset: CRYPTO_ALG_TYPE_MASK,
    type_: CRYPTO_ALG_TYPE_AEAD,
    tfmsize: core::mem::offset_of!(crypto_aead, base),
    algsize: core::mem::offset_of!(aead_alg, base),
};

pub unsafe fn crypto_grab_aead(
    spawn: *mut crypto_aead_spawn,
    inst: *mut crypto_instance,
    name: *const i8,
    type_: u32,
    mask: u32,
) -> i32 {
    (*spawn).base.frontend = &crypto_aead_type;
    crypto_grab_spawn(&mut (*spawn).base, inst, name, type_, mask)
}

pub unsafe fn crypto_alloc_aead(alg_name: *const i8, type_: u32, mask: u32) -> *mut crypto_aead {
    crypto_alloc_tfm(alg_name, &crypto_aead_type, type_, mask)
}

pub unsafe fn crypto_alloc_sync_aead(
    alg_name: *const i8,
    mut type_: u32,
    mut mask: u32,
) -> *mut crypto_sync_aead {
    // Only sync algorithms are allowed.
    mask |= CRYPTO_ALG_ASYNC;
    type_ &= !CRYPTO_ALG_ASYNC;
    let tfm = crypto_alloc_tfm(alg_name, &crypto_aead_type, type_, mask);
    if !IS_ERR(tfm)
        && WARN_ON(crypto_aead_reqsize(tfm) > MAX_SYNC_AEAD_REQSIZE)
    {
        crypto_free_aead(tfm);
        return ERR_PTR(-EINVAL);
    }
    tfm as *mut crypto_sync_aead
}

pub unsafe fn crypto_has_aead(alg_name: *const i8, type_: u32, mask: u32) -> i32 {
    crypto_type_has_alg(alg_name, &crypto_aead_type, type_, mask)
}

unsafe fn aead_prepare_alg(alg: *mut aead_alg) -> i32 {
    let base = &mut (*alg).base;
    if max3((*alg).maxauthsize, (*alg).ivsize, (*alg).chunksize) > PAGE_SIZE / 8 {
        return -EINVAL;
    }
    if (*alg).chunksize == 0 {
        (*alg).chunksize = base.cra_blocksize;
    }
    base.cra_type = &crypto_aead_type;
    base.cra_flags &= !CRYPTO_ALG_TYPE_MASK;
    base.cra_flags |= CRYPTO_ALG_TYPE_AEAD;
    0
}

pub unsafe fn crypto_register_aead(alg: *mut aead_alg) -> i32 {
    let err = aead_prepare_alg(alg);
    if err != 0 {
        return err;
    }
    crypto_register_alg(&mut (*alg).base)
}

pub unsafe fn crypto_unregister_aead(alg: *mut aead_alg) {
    crypto_unregister_alg(&mut (*alg).base);
}

pub unsafe fn crypto_register_aeads(algs: *mut aead_alg, count: i32) -> i32 {
    let mut i = 0;
    while i < count {
        let ret = crypto_register_aead(algs.add(i as usize));
        if ret != 0 {
            while i > 0 {
                i -= 1;
                crypto_unregister_aead(algs.add(i as usize));
            }
            return ret;
        }
        i += 1;
    }
    0
}

pub unsafe fn crypto_unregister_aeads(algs: *mut aead_alg, count: i32) {
    let mut i = count - 1;
    while i >= 0 {
        crypto_unregister_aead(algs.add(i as usize));
        i -= 1;
    }
}

pub unsafe fn aead_register_instance(
    tmpl: *mut crypto_template,
    inst: *mut aead_instance,
) -> i32 {
    if WARN_ON((*inst).free.is_none()) {
        return -EINVAL;
    }
    let err = aead_prepare_alg(&mut (*inst).alg);
    if err != 0 {
        return err;
    }
    crypto_register_instance(tmpl, aead_crypto_instance(inst))
}

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Authenticated Encryption with Associated Data (AEAD)");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
