// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Key-agreement Protocol Primitives (KPP)
 *
 * Copyright (c) 2016, Intel Corporation
 * Authors: Salvatore Benedetto <salvatore.benedetto@intel.com>
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn crypto_kpp_report(
    skb: *mut sk_buff,
    alg: *mut crypto_alg,
) -> c_int {
    let rkpp = crypto_report_kpp {
        type_: b"kpp\0".as_ptr() as *const c_char,
    };

    nla_put(
        skb,
        CRYPTOCFGA_REPORT_KPP,
        core::mem::size_of::<crypto_report_kpp>(),
        &rkpp as *const crypto_report_kpp as *const c_void,
    )
}

unsafe fn crypto_kpp_show(m: *mut seq_file, alg: *mut crypto_alg) {
    seq_puts(m, b"type         : kpp\n\0".as_ptr() as *const c_char);
}

unsafe fn crypto_kpp_exit_tfm(tfm: *mut crypto_tfm) {
    let kpp = __crypto_kpp_tfm(tfm);
    let alg = crypto_kpp_alg(kpp);

    ((*alg).exit.unwrap())(kpp);
}

unsafe fn crypto_kpp_init_tfm(tfm: *mut crypto_tfm) -> c_int {
    let kpp = __crypto_kpp_tfm(tfm);
    let alg = crypto_kpp_alg(kpp);

    if (*alg).exit.is_some() {
        (*kpp).base.exit = Some(crypto_kpp_exit_tfm);
    }

    if let Some(init) = (*alg).init {
        return init(kpp);
    }

    0
}

unsafe fn crypto_kpp_free_instance(inst: *mut crypto_instance) {
    let kpp = kpp_instance(inst);

    ((*kpp).free.unwrap())(kpp);
}

static mut crypto_kpp_type: crypto_type = crypto_type {
    extsize: Some(crypto_alg_extsize),
    init_tfm: Some(crypto_kpp_init_tfm),
    free: Some(crypto_kpp_free_instance),
    // CONFIG_PROC_FS
    show: Some(crypto_kpp_show),
    // IS_ENABLED(CONFIG_CRYPTO_USER)
    report: Some(crypto_kpp_report),
    maskclear: !CRYPTO_ALG_TYPE_MASK,
    maskset: CRYPTO_ALG_TYPE_MASK,
    type_: CRYPTO_ALG_TYPE_KPP,
    tfmsize: core::mem::offset_of!(crypto_kpp, base),
    algsize: core::mem::offset_of!(kpp_alg, base),
};

unsafe fn crypto_alloc_kpp(
    alg_name: *const c_char,
    type_: u32,
    mask: u32,
) -> *mut crypto_kpp {
    crypto_alloc_tfm(alg_name, &crypto_kpp_type, type_, mask)
}

unsafe fn crypto_grab_kpp(
    spawn: *mut crypto_kpp_spawn,
    inst: *mut crypto_instance,
    name: *const c_char,
    type_: u32,
    mask: u32,
) -> c_int {
    (*spawn).base.frontend = &crypto_kpp_type;
    crypto_grab_spawn(&mut (*spawn).base, inst, name, type_, mask)
}

unsafe fn crypto_has_kpp(alg_name: *const c_char, type_: u32, mask: u32) -> c_int {
    crypto_type_has_alg(alg_name, &crypto_kpp_type, type_, mask)
}

unsafe fn kpp_prepare_alg(alg: *mut kpp_alg) {
    let base = &mut (*alg).base;

    base.cra_type = &crypto_kpp_type;
    base.cra_flags &= !CRYPTO_ALG_TYPE_MASK;
    base.cra_flags |= CRYPTO_ALG_TYPE_KPP;
}

unsafe fn crypto_register_kpp(alg: *mut kpp_alg) -> c_int {
    let base = &mut (*alg).base;

    kpp_prepare_alg(alg);
    crypto_register_alg(base)
}

unsafe fn crypto_unregister_kpp(alg: *mut kpp_alg) {
    crypto_unregister_alg(&mut (*alg).base);
}

unsafe fn kpp_register_instance(
    tmpl: *mut crypto_template,
    inst: *mut kpp_instance,
) -> c_int {
    if WARN_ON((*inst).free.is_none()) {
        return -EINVAL;
    }

    kpp_prepare_alg(&mut (*inst).alg);

    crypto_register_instance(tmpl, kpp_crypto_instance(inst))
}

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Key-agreement Protocol Primitives");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
