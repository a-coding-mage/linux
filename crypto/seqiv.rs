// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * seqiv: Sequence Number IV Generator
 *
 * This generator generates an IV based on a sequence number by xoring it
 * with a salt.  This algorithm is mainly useful for CTR and similar modes.
 *
 * Copyright (c) 2007 Herbert Xu <herbert@gondor.apana.org.au>
 */

unsafe fn seqiv_aead_encrypt_complete2(req: *mut aead_request, err: i32) {
    let subreq: *mut aead_request = aead_request_ctx(req);
    let mut geniv: *mut crypto_aead;

    if err == -EINPROGRESS || err == -EBUSY {
        return;
    }

    if err == 0 {
        geniv = crypto_aead_reqtfm(req);
        memcpy((*req).iv, (*subreq).iv, crypto_aead_ivsize(geniv));
    }

out:
    kfree_sensitive((*subreq).iv);
}

unsafe fn seqiv_aead_encrypt_complete(data: *mut core::ffi::c_void, err: i32) {
    let req: *mut aead_request = data as *mut aead_request;

    seqiv_aead_encrypt_complete2(req, err);
    aead_request_complete(req, err);
}

unsafe fn seqiv_aead_encrypt(req: *mut aead_request) -> i32 {
    let geniv: *mut crypto_aead = crypto_aead_reqtfm(req);
    let ctx: *mut aead_geniv_ctx = crypto_aead_ctx(geniv);
    let subreq: *mut aead_request = aead_request_ctx(req);
    let mut compl: crypto_completion_t;
    let mut unaligned_info: bool;
    let mut data: *mut core::ffi::c_void;
    let mut info: *mut u8;
    let ivsize: u32 = 8;
    let mut err: i32;

    if (*req).cryptlen < ivsize {
        return -EINVAL;
    }

    aead_request_set_tfm(subreq, (*ctx).child);

    compl = (*req).base.complete;
    data = (*req).base.data;
    info = (*req).iv;

    if (*req).src != (*req).dst {
        memcpy_sglist((*req).dst, (*req).src, (*req).assoclen + (*req).cryptlen);
    }

    unaligned_info = !IS_ALIGNED(info as usize, crypto_aead_alignmask(geniv) + 1);
    if unlikely(unaligned_info) {
        info = kmemdup((*req).iv, ivsize, if (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP != 0 {
            GFP_KERNEL
        } else {
            GFP_ATOMIC
        });
        if info.is_null() {
            return -ENOMEM;
        }

        compl = seqiv_aead_encrypt_complete;
        data = req as *mut core::ffi::c_void;
    }

    aead_request_set_callback(subreq, (*req).base.flags, compl, data);
    aead_request_set_crypt(subreq, (*req).dst, (*req).dst, (*req).cryptlen - ivsize, info);
    aead_request_set_ad(subreq, (*req).assoclen + ivsize);

    crypto_xor(info, (*ctx).salt, ivsize);
    scatterwalk_map_and_copy(info, (*req).dst, (*req).assoclen, ivsize, 1);

    err = crypto_aead_encrypt(subreq);
    if unlikely(unaligned_info) {
        seqiv_aead_encrypt_complete2(req, err);
    }
    err
}

unsafe fn seqiv_aead_decrypt(req: *mut aead_request) -> i32 {
    let geniv: *mut crypto_aead = crypto_aead_reqtfm(req);
    let ctx: *mut aead_geniv_ctx = crypto_aead_ctx(geniv);
    let subreq: *mut aead_request = aead_request_ctx(req);
    let compl: crypto_completion_t;
    let data: *mut core::ffi::c_void;
    let ivsize: u32 = 8;

    if (*req).cryptlen < ivsize + crypto_aead_authsize(geniv) {
        return -EINVAL;
    }

    aead_request_set_tfm(subreq, (*ctx).child);

    compl = (*req).base.complete;
    data = (*req).base.data;

    aead_request_set_callback(subreq, (*req).base.flags, compl, data);
    aead_request_set_crypt(subreq, (*req).src, (*req).dst, (*req).cryptlen - ivsize, (*req).iv);
    aead_request_set_ad(subreq, (*req).assoclen + ivsize);

    scatterwalk_map_and_copy((*req).iv, (*req).src, (*req).assoclen, ivsize, 0);

    crypto_aead_decrypt(subreq)
}

unsafe fn seqiv_aead_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> i32 {
    let inst: *mut aead_instance;
    let mut err: i32;

    inst = aead_geniv_alloc(tmpl, tb);

    if IS_ERR(inst) {
        return PTR_ERR(inst);
    }

    err = -EINVAL;
    if (*inst).alg.ivsize != core::mem::size_of::<u64>() {
        goto free_inst;
    }

    (*inst).alg.encrypt = Some(seqiv_aead_encrypt);
    (*inst).alg.decrypt = Some(seqiv_aead_decrypt);

    (*inst).alg.init = Some(aead_init_geniv);
    (*inst).alg.exit = Some(aead_exit_geniv);

    (*inst).alg.base.cra_ctxsize = core::mem::size_of::<aead_geniv_ctx>();
    (*inst).alg.base.cra_ctxsize += (*inst).alg.ivsize;

    err = aead_register_instance(tmpl, inst);
    if err != 0 {
free_inst:
        ((*inst).free)(inst);
    }
    err
}

static mut seqiv_tmpl: crypto_template = crypto_template {
    name: "seqiv",
    create: Some(seqiv_aead_create),
    module: THIS_MODULE,
};

unsafe fn seqiv_module_init() -> i32 {
    crypto_register_template(&mut seqiv_tmpl)
}

unsafe fn seqiv_module_exit() {
    crypto_unregister_template(&mut seqiv_tmpl);
}

module_init!(seqiv_module_init);
module_exit!(seqiv_module_exit);

MODULE_LICENSE!("GPL");
MODULE_DESCRIPTION!("Sequence Number IV Generator");
MODULE_ALIAS_CRYPTO!("seqiv");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
