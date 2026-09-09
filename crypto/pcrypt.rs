// SPDX-License-Identifier: GPL-2.0-only
/*
 * pcrypt - Parallel crypto wrapper.
 *
 * Copyright (C) 2009 secunet Security Networks AG
 * Copyright (C) 2009 Steffen Klassert <steffen.klassert@secunet.com>
 */

// External Linux kernel crypto, padata, kobject, CPU, module, and pcrypt
// declarations supplied by the surrounding kernel Rust bindings.

static mut pencrypt: *mut padata_instance = core::ptr::null_mut();
static mut pdecrypt: *mut padata_instance = core::ptr::null_mut();
static mut pcrypt_kset: *mut kset = core::ptr::null_mut();

#[repr(C)]
struct pcrypt_instance_ctx {
    spawn: crypto_aead_spawn,
    psenc: *mut padata_shell,
    psdec: *mut padata_shell,
    tfm_count: atomic_t,
}

#[repr(C)]
struct pcrypt_aead_ctx {
    child: *mut crypto_aead,
    cb_cpu: c_uint,
}

#[inline]
unsafe fn pcrypt_tfm_ictx(tfm: *mut crypto_aead) -> *mut pcrypt_instance_ctx {
    aead_instance_ctx(aead_alg_instance(tfm))
}

unsafe fn pcrypt_aead_setkey(
    parent: *mut crypto_aead,
    key: *const u8,
    keylen: c_uint,
) -> c_int {
    let ctx = crypto_aead_ctx(parent) as *mut pcrypt_aead_ctx;
    crypto_aead_setkey((*ctx).child, key, keylen)
}

unsafe fn pcrypt_aead_setauthsize(parent: *mut crypto_aead, authsize: c_uint) -> c_int {
    let ctx = crypto_aead_ctx(parent) as *mut pcrypt_aead_ctx;
    crypto_aead_setauthsize((*ctx).child, authsize)
}

unsafe extern "C" fn pcrypt_aead_serial(padata: *mut padata_priv) {
    let preq = pcrypt_padata_request(padata);
    let req = pcrypt_request_ctx(preq);
    aead_request_complete((*req).base.data, (*padata).info);
}

unsafe extern "C" fn pcrypt_aead_done(data: *mut c_void, err: c_int) {
    let req = data as *mut aead_request;
    let preq = aead_request_ctx(req);
    let padata = pcrypt_request_padata(preq);

    if err == -EINPROGRESS {
        return;
    }

    (*padata).info = err;
    padata_do_serial(padata);
}

unsafe extern "C" fn pcrypt_aead_enc(padata: *mut padata_priv) {
    let preq = pcrypt_padata_request(padata);
    let req = pcrypt_request_ctx(preq);
    let ret = crypto_aead_encrypt(req);

    if ret == -EINPROGRESS || ret == -EBUSY {
        return;
    }

    (*padata).info = ret;
    padata_do_serial(padata);
}

unsafe fn pcrypt_aead_encrypt(req: *mut aead_request) -> c_int {
    let preq = aead_request_ctx(req);
    let creq = pcrypt_request_ctx(preq);
    let padata = pcrypt_request_padata(preq);
    let aead = crypto_aead_reqtfm(req);
    let ctx = crypto_aead_ctx(aead) as *mut pcrypt_aead_ctx;
    let flags = aead_request_flags(req);
    let ictx = pcrypt_tfm_ictx(aead);

    core::ptr::write_bytes(padata as *mut u8, 0, core::mem::size_of::<padata_priv>());
    (*padata).parallel = Some(pcrypt_aead_enc);
    (*padata).serial = Some(pcrypt_aead_serial);

    aead_request_set_tfm(creq, (*ctx).child);
    aead_request_set_callback(
        creq,
        flags & !CRYPTO_TFM_REQ_MAY_SLEEP,
        Some(pcrypt_aead_done),
        req as *mut c_void,
    );
    aead_request_set_crypt(creq, (*req).src, (*req).dst, (*req).cryptlen, (*req).iv);
    aead_request_set_ad(creq, (*req).assoclen);

    let err = padata_do_parallel((*ictx).psenc, padata, &mut (*ctx).cb_cpu);
    if err == 0 {
        return -EINPROGRESS;
    }
    if err == -EBUSY {
        /* try non-parallel mode */
        aead_request_set_callback(creq, flags, (*req).base.complete, (*req).base.data);
        return crypto_aead_encrypt(creq);
    }
    err
}

unsafe extern "C" fn pcrypt_aead_dec(padata: *mut padata_priv) {
    let preq = pcrypt_padata_request(padata);
    let req = pcrypt_request_ctx(preq);
    let ret = crypto_aead_decrypt(req);

    if ret == -EINPROGRESS || ret == -EBUSY {
        return;
    }

    (*padata).info = ret;
    padata_do_serial(padata);
}

unsafe fn pcrypt_aead_decrypt(req: *mut aead_request) -> c_int {
    let preq = aead_request_ctx(req);
    let creq = pcrypt_request_ctx(preq);
    let padata = pcrypt_request_padata(preq);
    let aead = crypto_aead_reqtfm(req);
    let ctx = crypto_aead_ctx(aead) as *mut pcrypt_aead_ctx;
    let flags = aead_request_flags(req);
    let ictx = pcrypt_tfm_ictx(aead);

    core::ptr::write_bytes(padata as *mut u8, 0, core::mem::size_of::<padata_priv>());
    (*padata).parallel = Some(pcrypt_aead_dec);
    (*padata).serial = Some(pcrypt_aead_serial);

    aead_request_set_tfm(creq, (*ctx).child);
    aead_request_set_callback(
        creq,
        flags & !CRYPTO_TFM_REQ_MAY_SLEEP,
        Some(pcrypt_aead_done),
        req as *mut c_void,
    );
    aead_request_set_crypt(creq, (*req).src, (*req).dst, (*req).cryptlen, (*req).iv);
    aead_request_set_ad(creq, (*req).assoclen);

    let err = padata_do_parallel((*ictx).psdec, padata, &mut (*ctx).cb_cpu);
    if err == 0 {
        return -EINPROGRESS;
    }
    if err == -EBUSY {
        /* try non-parallel mode */
        aead_request_set_callback(creq, flags, (*req).base.complete, (*req).base.data);
        return crypto_aead_decrypt(creq);
    }
    err
}

unsafe fn pcrypt_aead_init_tfm(tfm: *mut crypto_aead) -> c_int {
    let inst = aead_alg_instance(tfm);
    let ictx = aead_instance_ctx(inst);
    let ctx = crypto_aead_ctx(tfm) as *mut pcrypt_aead_ctx;
    let cpu_index = (atomic_inc_return(&mut (*ictx).tfm_count) as c_uint)
        % cpumask_weight(cpu_online_mask);

    (*ctx).cb_cpu = cpumask_nth(cpu_index, cpu_online_mask);
    let cipher = crypto_spawn_aead(&mut (*ictx).spawn);
    if IS_ERR(cipher) {
        return PTR_ERR(cipher);
    }
    (*ctx).child = cipher;
    crypto_aead_set_reqsize(
        tfm,
        core::mem::size_of::<pcrypt_request>()
            + core::mem::size_of::<aead_request>()
            + crypto_aead_reqsize(cipher),
    );
    0
}

unsafe fn pcrypt_aead_exit_tfm(tfm: *mut crypto_aead) {
    let ctx = crypto_aead_ctx(tfm) as *mut pcrypt_aead_ctx;
    crypto_free_aead((*ctx).child);
}

unsafe fn pcrypt_free(inst: *mut aead_instance) {
    let ctx = aead_instance_ctx(inst);
    crypto_drop_aead(&mut (*ctx).spawn);
    padata_free_shell((*ctx).psdec);
    padata_free_shell((*ctx).psenc);
    kfree(inst as *mut c_void);
}

unsafe fn pcrypt_init_instance(inst: *mut crypto_instance, alg: *mut crypto_alg) -> c_int {
    if snprintf(
        (*inst).alg.cra_driver_name.as_mut_ptr(),
        CRYPTO_MAX_ALG_NAME,
        c"pcrypt(%s)".as_ptr(),
        (*alg).cra_driver_name.as_ptr(),
    ) >= CRYPTO_MAX_ALG_NAME as c_int {
        return -ENAMETOOLONG;
    }
    memcpy(
        (*inst).alg.cra_name.as_mut_ptr() as *mut c_void,
        (*alg).cra_name.as_ptr() as *const c_void,
        CRYPTO_MAX_ALG_NAME,
    );
    (*inst).alg.cra_priority = (*alg).cra_priority + 100;
    (*inst).alg.cra_blocksize = (*alg).cra_blocksize;
    (*inst).alg.cra_alignmask = (*alg).cra_alignmask;
    0
}

unsafe fn pcrypt_create_aead(
    tmpl: *mut crypto_template,
    tb: *mut *mut rtattr,
    algt: *mut crypto_attr_type,
) -> c_int {
    let inst = kzalloc(core::mem::size_of::<aead_instance>() + core::mem::size_of::<pcrypt_instance_ctx>(), GFP_KERNEL) as *mut aead_instance;
    if inst.is_null() { return -ENOMEM; }
    let ctx = aead_instance_ctx(inst);
    (*ctx).psenc = padata_alloc_shell(pencrypt);
    if (*ctx).psenc.is_null() { pcrypt_free(inst); return -ENOMEM; }
    (*ctx).psdec = padata_alloc_shell(pdecrypt);
    if (*ctx).psdec.is_null() { pcrypt_free(inst); return -ENOMEM; }
    let mask = crypto_algt_inherited_mask(algt);
    let mut err = crypto_grab_aead(&mut (*ctx).spawn, aead_crypto_instance(inst), crypto_attr_alg_name(*tb.add(1)), 0, mask);
    if err != 0 { pcrypt_free(inst); return err; }
    let alg = crypto_spawn_aead_alg(&mut (*ctx).spawn);
    err = pcrypt_init_instance(aead_crypto_instance(inst), &mut (*alg).base);
    if err != 0 { pcrypt_free(inst); return err; }
    (*inst).alg.base.cra_flags |= CRYPTO_ALG_ASYNC;
    (*inst).alg.ivsize = crypto_aead_alg_ivsize(alg);
    (*inst).alg.maxauthsize = crypto_aead_alg_maxauthsize(alg);
    (*inst).alg.base.cra_ctxsize = core::mem::size_of::<pcrypt_aead_ctx>();
    (*inst).alg.init = Some(pcrypt_aead_init_tfm);
    (*inst).alg.exit = Some(pcrypt_aead_exit_tfm);
    (*inst).alg.setkey = Some(pcrypt_aead_setkey);
    (*inst).alg.setauthsize = Some(pcrypt_aead_setauthsize);
    (*inst).alg.encrypt = Some(pcrypt_aead_encrypt);
    (*inst).alg.decrypt = Some(pcrypt_aead_decrypt);
    (*inst).free = Some(pcrypt_free);
    err = aead_register_instance(tmpl, inst);
    if err != 0 { pcrypt_free(inst); }
    err
}

unsafe fn pcrypt_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> c_int {
    let algt = crypto_get_attr_type(tb);
    if IS_ERR(algt) { return PTR_ERR(algt); }
    match (*algt).type_ & (*algt).mask & CRYPTO_ALG_TYPE_MASK {
        CRYPTO_ALG_TYPE_AEAD => pcrypt_create_aead(tmpl, tb, algt),
        _ => -EINVAL,
    }
}

unsafe fn pcrypt_sysfs_add(pinst: *mut padata_instance, name: *const c_char) -> c_int {
    (*pinst).kobj.kset = pcrypt_kset;
    let ret = kobject_add(&mut (*pinst).kobj, core::ptr::null_mut(), c"%s".as_ptr(), name);
    if ret == 0 { kobject_uevent(&mut (*pinst).kobj, KOBJ_ADD); }
    ret
}

unsafe fn pcrypt_init_padata(pinst: *mut *mut padata_instance, name: *const c_char) -> c_int {
    let mut ret = -ENOMEM;
    *pinst = padata_alloc(name);
    if (*pinst).is_null() { return ret; }
    ret = pcrypt_sysfs_add(*pinst, name);
    if ret != 0 { padata_free(*pinst); }
    ret
}

static mut pcrypt_tmpl: crypto_template = crypto_template {
    name: c"pcrypt".as_ptr(),
    create: Some(pcrypt_create),
    module: THIS_MODULE,
};

unsafe extern "C" fn pcrypt_init() -> c_int {
    let mut err = -ENOMEM;
    pcrypt_kset = kset_create_and_add(c"pcrypt".as_ptr(), core::ptr::null(), kernel_kobj);
    if pcrypt_kset.is_null() { return err; }
    err = pcrypt_init_padata(&mut pencrypt, c"pencrypt".as_ptr());
    if err != 0 { kset_unregister(pcrypt_kset); return err; }
    err = pcrypt_init_padata(&mut pdecrypt, c"pdecrypt".as_ptr());
    if err != 0 { padata_free(pencrypt); kset_unregister(pcrypt_kset); return err; }
    crypto_register_template(&mut pcrypt_tmpl)
}

unsafe extern "C" fn pcrypt_exit() {
    crypto_unregister_template(&mut pcrypt_tmpl);
    padata_free(pencrypt);
    padata_free(pdecrypt);
    kset_unregister(pcrypt_kset);
}

module_init!(pcrypt_init);
module_exit!(pcrypt_exit);

MODULE_LICENSE!(c"GPL");
MODULE_AUTHOR!(c"Steffen Klassert <steffen.klassert@secunet.com>");
MODULE_DESCRIPTION!(c"Parallel crypto wrapper");
MODULE_ALIAS_CRYPTO!(c"pcrypt");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
