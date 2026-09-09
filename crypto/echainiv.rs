// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * echainiv: Encrypted Chain IV Generator
 *
 * This generator generates an IV based on a sequence number by multiplying
 * it with a salt and then encrypting it with the same key as used to encrypt
 * the plain text.  This algorithm requires that the block size be equal
 * to the IV size.  It is mainly useful for CBC.
 *
 * This generator can only be used by algorithms where authentication
 * is performed after encryption (i.e., authenc).
 *
 * Copyright (c) 2015 Herbert Xu <herbert@gondor.apana.org.au>
 */

// C dependencies supplied by the surrounding kernel crypto implementation.

unsafe fn echainiv_encrypt(req: *mut aead_request) -> i32 {
    let geniv = crypto_aead_reqtfm(req);
    let ctx = crypto_aead_ctx(geniv);
    let subreq = aead_request_ctx(req);
    let mut nseqno: __be64 = 0;
    let mut seqno: u64;
    let info: *mut u8;
    let mut ivsize = crypto_aead_ivsize(geniv);

    if (*req).cryptlen < ivsize {
        return -EINVAL;
    }

    aead_request_set_tfm(subreq, (*ctx).child);

    info = (*req).iv;

    if (*req).src != (*req).dst {
        memcpy_sglist((*req).dst, (*req).src, (*req).assoclen + (*req).cryptlen);
    }

    aead_request_set_callback(
        subreq,
        (*req).base.flags,
        (*req).base.complete,
        (*req).base.data,
    );
    aead_request_set_crypt(subreq, (*req).dst, (*req).dst, (*req).cryptlen, info);
    aead_request_set_ad(subreq, (*req).assoclen);

    core::ptr::copy_nonoverlapping(info.add(ivsize - 8), &mut nseqno as *mut __be64 as *mut u8, 8);
    seqno = be64_to_cpu(nseqno);
    core::ptr::write_bytes(info, 0, ivsize as usize);

    scatterwalk_map_and_copy(info, (*req).dst, (*req).assoclen, ivsize, 1);

    loop {
        let mut a: u64 = 0;

        core::ptr::copy_nonoverlapping(
            (*ctx).salt.add(ivsize - 8),
            &mut a as *mut u64 as *mut u8,
            8,
        );

        a |= 1;
        a = a.wrapping_mul(seqno);

        core::ptr::copy_nonoverlapping(&a as *const u64 as *const u8, info.add(ivsize - 8), 8);
        ivsize -= 8;
        if ivsize == 0 {
            break;
        }
    }

    crypto_aead_encrypt(subreq)
}

unsafe fn echainiv_decrypt(req: *mut aead_request) -> i32 {
    let geniv = crypto_aead_reqtfm(req);
    let ctx = crypto_aead_ctx(geniv);
    let subreq = aead_request_ctx(req);
    let compl: crypto_completion_t;
    let data: *mut core::ffi::c_void;
    let ivsize = crypto_aead_ivsize(geniv);

    if (*req).cryptlen < ivsize {
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

unsafe fn echainiv_aead_create(
    tmpl: *mut crypto_template,
    tb: *mut *mut rtattr,
) -> i32 {
    let inst = aead_geniv_alloc(tmpl, tb);
    let mut err: i32;

    if IS_ERR(inst) {
        return PTR_ERR(inst);
    }

    err = -EINVAL;
    if ((*inst).alg.ivsize & (core::mem::size_of::<u64>() as u32 - 1)) != 0
        || (*inst).alg.ivsize == 0
    {
        ((*inst).free)(inst);
        return err;
    }

    (*inst).alg.encrypt = Some(echainiv_encrypt);
    (*inst).alg.decrypt = Some(echainiv_decrypt);

    (*inst).alg.init = Some(aead_init_geniv);
    (*inst).alg.exit = Some(aead_exit_geniv);

    (*inst).alg.base.cra_ctxsize = core::mem::size_of::<aead_geniv_ctx>();
    (*inst).alg.base.cra_ctxsize += (*inst).alg.ivsize as usize;

    err = aead_register_instance(tmpl, inst);
    if err != 0 {
        ((*inst).free)(inst);
    }
    return err;
}

static mut echainiv_tmpl: crypto_template = crypto_template {
    name: "echainiv",
    create: Some(echainiv_aead_create),
    module: THIS_MODULE,
};

unsafe fn echainiv_module_init() -> i32 {
    crypto_register_template(&mut echainiv_tmpl)
}

unsafe fn echainiv_module_exit() {
    crypto_unregister_template(&mut echainiv_tmpl);
}

// module_init(echainiv_module_init);
// module_exit(echainiv_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Encrypted Chain IV Generator");
// MODULE_ALIAS_CRYPTO("echainiv");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
