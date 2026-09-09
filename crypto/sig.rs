/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Public Key Signature Algorithm
 *
 * Copyright (c) 2023 Herbert Xu <herbert@gondor.apana.org.au>
 */

// External kernel crypto declarations are supplied by the surrounding crate.

unsafe fn crypto_sig_exit_tfm(tfm: *mut crypto_tfm) {
    let sig: *mut crypto_sig = __crypto_sig_tfm(tfm);
    let alg: *mut sig_alg = crypto_sig_alg(sig);

    ((*alg).exit.unwrap())(sig);
}

unsafe fn crypto_sig_init_tfm(tfm: *mut crypto_tfm) -> c_int {
    let sig: *mut crypto_sig = __crypto_sig_tfm(tfm);
    let alg: *mut sig_alg = crypto_sig_alg(sig);

    if (*alg).exit.is_some() {
        (*sig).base.exit = Some(crypto_sig_exit_tfm);
    }

    if let Some(init) = (*alg).init {
        return init(sig);
    }

    0
}

unsafe fn crypto_sig_free_instance(inst: *mut crypto_instance) {
    let sig: *mut sig_instance = sig_instance(inst);

    ((*sig).free.unwrap())(sig);
}

#[allow(unused_variables)]
unsafe fn crypto_sig_show(m: *mut seq_file, alg: *mut crypto_alg) {
    seq_puts(m, "type         : sig\0".as_ptr() as *const c_char);
}

#[allow(unused_variables)]
unsafe fn crypto_sig_report(skb: *mut sk_buff, alg: *mut crypto_alg) -> c_int {
    let rsig = crypto_report_sig {
        r#type: "sig\0".as_ptr() as *const c_char,
    };

    nla_put(
        skb,
        CRYPTOCFGA_REPORT_SIG,
        core::mem::size_of::<crypto_report_sig>() as c_int,
        &rsig as *const crypto_report_sig as *const c_void,
    )
}

// CONFIG_PROC_FS and CONFIG_CRYPTO_USER conditionally provide the fields below.
static crypto_sig_type: crypto_type = crypto_type {
    extsize: crypto_alg_extsize,
    init_tfm: Some(crypto_sig_init_tfm),
    free: Some(crypto_sig_free_instance),
    // show: Some(crypto_sig_show),
    // report: Some(crypto_sig_report),
    maskclear: !CRYPTO_ALG_TYPE_MASK,
    maskset: CRYPTO_ALG_TYPE_MASK,
    r#type: CRYPTO_ALG_TYPE_SIG,
    tfmsize: core::mem::offset_of!(crypto_sig, base),
    algsize: core::mem::offset_of!(sig_alg, base),
};

pub unsafe fn crypto_alloc_sig(
    alg_name: *const c_char,
    r#type: u32,
    mask: u32,
) -> *mut crypto_sig {
    crypto_alloc_tfm(alg_name, &crypto_sig_type, r#type, mask)
}

unsafe fn sig_default_sign(
    _tfm: *mut crypto_sig,
    _src: *const c_void,
    _slen: c_uint,
    _dst: *mut c_void,
    _dlen: c_uint,
) -> c_int {
    -ENOSYS
}

unsafe fn sig_default_verify(
    _tfm: *mut crypto_sig,
    _src: *const c_void,
    _slen: c_uint,
    _dst: *const c_void,
    _dlen: c_uint,
) -> c_int {
    -ENOSYS
}

unsafe fn sig_default_set_key(
    _tfm: *mut crypto_sig,
    _key: *const c_void,
    _keylen: c_uint,
) -> c_int {
    -ENOSYS
}

unsafe fn sig_default_size(tfm: *mut crypto_sig) -> c_uint {
    div_round_up_pow2(crypto_sig_keysize(tfm), BITS_PER_BYTE)
}

unsafe fn sig_prepare_alg(alg: *mut sig_alg) -> c_int {
    let base: *mut crypto_alg = &mut (*alg).base;

    if (*alg).sign.is_none() {
        (*alg).sign = Some(sig_default_sign);
    }
    if (*alg).verify.is_none() {
        (*alg).verify = Some(sig_default_verify);
    }
    if (*alg).set_priv_key.is_none() {
        (*alg).set_priv_key = Some(sig_default_set_key);
    }
    if (*alg).set_pub_key.is_none() {
        return -EINVAL;
    }
    if (*alg).key_size == 0 {
        return -EINVAL;
    }
    if (*alg).max_size.is_none() {
        (*alg).max_size = Some(sig_default_size);
    }
    if (*alg).digest_size.is_none() {
        (*alg).digest_size = Some(sig_default_size);
    }

    (*base).cra_type = &crypto_sig_type;
    (*base).cra_flags &= !CRYPTO_ALG_TYPE_MASK;
    (*base).cra_flags |= CRYPTO_ALG_TYPE_SIG;

    0
}

pub unsafe fn crypto_register_sig(alg: *mut sig_alg) -> c_int {
    let base: *mut crypto_alg = &mut (*alg).base;
    let err = sig_prepare_alg(alg);
    if err != 0 {
        return err;
    }

    crypto_register_alg(base)
}

pub unsafe fn crypto_unregister_sig(alg: *mut sig_alg) {
    crypto_unregister_alg(&mut (*alg).base);
}

pub unsafe fn sig_register_instance(
    tmpl: *mut crypto_template,
    inst: *mut sig_instance,
) -> c_int {
    if warn_on((*inst).free.is_none()) {
        return -EINVAL;
    }

    let err = sig_prepare_alg(&mut (*inst).alg);
    if err != 0 {
        return err;
    }

    crypto_register_instance(tmpl, sig_crypto_instance(inst))
}

pub unsafe fn crypto_grab_sig(
    spawn: *mut crypto_sig_spawn,
    inst: *mut crypto_instance,
    name: *const c_char,
    r#type: u32,
    mask: u32,
) -> c_int {
    (*spawn).base.frontend = &crypto_sig_type;
    crypto_grab_spawn(&mut (*spawn).base, inst, name, r#type, mask)
}

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Public Key Signature Algorithms");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
