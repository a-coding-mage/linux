// SPDX-License-Identifier: GPL-2.0+
/*
 * ECDSA X9.62 signature encoding
 *
 * Copyright (c) 2021 IBM Corporation
 * Copyright (c) 2024 Intel Corporation
 */

// Dependencies supplied by the surrounding kernel implementation are intentionally
// referenced by name here rather than reimplemented in this translation.

#[repr(C)]
pub struct ecdsa_x962_ctx {
    pub child: *mut crypto_sig,
}

#[repr(C)]
pub struct ecdsa_x962_signature_ctx {
    pub sig: ecdsa_raw_sig,
    pub ndigits: c_uint,
}

unsafe fn ecdsa_get_signature_rs(
    dest: *mut u64,
    _hdrlen: usize,
    _tag: u8,
    value: *const core::ffi::c_void,
    vlen: usize,
    ndigits: c_uint,
) -> c_int {
    let bufsize = ndigits as usize * core::mem::size_of::<u64>();
    let mut d = value as *const i8;

    if value.is_null() || vlen == 0 || vlen > bufsize + 1 {
        return -EINVAL;
    }

    /*
     * vlen may be 1 byte larger than bufsize due to a leading zero byte
     * (necessary if the most significant bit of the integer is set).
     */
    let mut length = vlen;
    if length > bufsize {
        /* skip over leading zeros that make 'value' a positive int */
        if *d == 0 {
            length -= 1;
            d = d.add(1);
        } else {
            return -EINVAL;
        }
    }

    ecc_digits_from_bytes(d, length, dest, ndigits);
    0
}

pub unsafe extern "C" fn ecdsa_get_signature_r(
    context: *mut core::ffi::c_void,
    hdrlen: usize,
    tag: u8,
    value: *const core::ffi::c_void,
    vlen: usize,
) -> c_int {
    let sig_ctx = &mut *(context as *mut ecdsa_x962_signature_ctx);
    ecdsa_get_signature_rs(sig_ctx.sig.r.as_mut_ptr(), hdrlen, tag, value, vlen, sig_ctx.ndigits)
}

pub unsafe extern "C" fn ecdsa_get_signature_s(
    context: *mut core::ffi::c_void,
    hdrlen: usize,
    tag: u8,
    value: *const core::ffi::c_void,
    vlen: usize,
) -> c_int {
    let sig_ctx = &mut *(context as *mut ecdsa_x962_signature_ctx);
    ecdsa_get_signature_rs(sig_ctx.sig.s.as_mut_ptr(), hdrlen, tag, value, vlen, sig_ctx.ndigits)
}

unsafe fn ecdsa_x962_verify(
    tfm: *mut crypto_sig,
    src: *const core::ffi::c_void,
    slen: c_uint,
    digest: *const core::ffi::c_void,
    dlen: c_uint,
) -> c_int {
    let ctx = crypto_sig_ctx(tfm) as *mut ecdsa_x962_ctx;
    let mut sig_ctx: ecdsa_x962_signature_ctx = core::mem::zeroed();
    let ndigits = (crypto_sig_keysize((*ctx).child) as usize)
        .div_ceil(core::mem::size_of::<u64>() * BITS_PER_BYTE as usize);
    sig_ctx.ndigits = ndigits as c_uint;

    let err = asn1_ber_decoder(
        &ecdsasignature_decoder,
        &mut sig_ctx as *mut _ as *mut core::ffi::c_void,
        src,
        slen,
    );
    if err < 0 {
        return err;
    }

    crypto_sig_verify(
        (*ctx).child,
        &sig_ctx.sig as *const _ as *const core::ffi::c_void,
        core::mem::size_of_val(&sig_ctx.sig),
        digest,
        dlen,
    )
}

unsafe fn ecdsa_x962_key_size(tfm: *mut crypto_sig) -> c_uint {
    let ctx = crypto_sig_ctx(tfm) as *mut ecdsa_x962_ctx;
    crypto_sig_keysize((*ctx).child)
}

unsafe fn ecdsa_x962_max_size(tfm: *mut crypto_sig) -> c_uint {
    let ctx = crypto_sig_ctx(tfm) as *mut ecdsa_x962_ctx;
    let alg = crypto_sig_alg((*ctx).child);
    let mut slen = (crypto_sig_keysize((*ctx).child) as usize)
        .div_ceil(BITS_PER_BYTE as usize);
    if strcmp((*alg).base.cra_name, b"ecdsa-nist-p521\0") != 0 {
        slen += 1;
    }
    slen = 2 * (slen + 2);
    (1 + (slen >= 128) as usize + 1 + slen) as c_uint
}

unsafe fn ecdsa_x962_digest_size(tfm: *mut crypto_sig) -> c_uint {
    let ctx = crypto_sig_ctx(tfm) as *mut ecdsa_x962_ctx;
    crypto_sig_digestsize((*ctx).child)
}

unsafe fn ecdsa_x962_set_pub_key(tfm: *mut crypto_sig, key: *const core::ffi::c_void, keylen: c_uint) -> c_int {
    let ctx = crypto_sig_ctx(tfm) as *mut ecdsa_x962_ctx;
    crypto_sig_set_pubkey((*ctx).child, key, keylen)
}

unsafe fn ecdsa_x962_init_tfm(tfm: *mut crypto_sig) -> c_int {
    let inst = sig_alg_instance(tfm);
    let spawn = sig_instance_ctx(inst);
    let ctx = crypto_sig_ctx(tfm) as *mut ecdsa_x962_ctx;
    let child_tfm = crypto_spawn_sig(spawn);
    if IS_ERR(child_tfm) {
        return PTR_ERR(child_tfm);
    }
    (*ctx).child = child_tfm;
    0
}

unsafe fn ecdsa_x962_exit_tfm(tfm: *mut crypto_sig) {
    let ctx = crypto_sig_ctx(tfm) as *mut ecdsa_x962_ctx;
    crypto_free_sig((*ctx).child);
}

unsafe fn ecdsa_x962_free(inst: *mut sig_instance) {
    let spawn = sig_instance_ctx(inst);
    crypto_drop_sig(spawn);
    kfree(inst as *mut core::ffi::c_void);
}

unsafe fn ecdsa_x962_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> c_int {
    let mut spawn: *mut crypto_sig_spawn;
    let mut inst: *mut sig_instance;
    let mut ecdsa_alg: *mut sig_alg;
    let mut mask: u32 = 0;
    let mut err = crypto_check_attr_type(tb, CRYPTO_ALG_TYPE_SIG, &mut mask);
    if err != 0 { return err; }
    inst = kzalloc(core::mem::size_of::<sig_instance>() + core::mem::size_of::<crypto_sig_spawn>(), GFP_KERNEL) as *mut sig_instance;
    if inst.is_null() { return -ENOMEM; }
    spawn = sig_instance_ctx(inst);
    err = crypto_grab_sig(spawn, sig_crypto_instance(inst), crypto_attr_alg_name(*tb.add(1)), 0, mask);
    if err != 0 { ecdsa_x962_free(inst); return err; }
    ecdsa_alg = crypto_spawn_sig_alg(spawn);
    if strncmp((*ecdsa_alg).base.cra_name, b"ecdsa", 5) != 0 { ecdsa_x962_free(inst); return -EINVAL; }
    err = crypto_inst_setname(sig_crypto_instance(inst), (*tmpl).name, &(*ecdsa_alg).base);
    if err != 0 { ecdsa_x962_free(inst); return err; }
    (*inst).alg.base.cra_priority = (*ecdsa_alg).base.cra_priority;
    (*inst).alg.base.cra_ctxsize = core::mem::size_of::<ecdsa_x962_ctx>();
    (*inst).alg.init = Some(ecdsa_x962_init_tfm);
    (*inst).alg.exit = Some(ecdsa_x962_exit_tfm);
    (*inst).alg.verify = Some(ecdsa_x962_verify);
    (*inst).alg.key_size = Some(ecdsa_x962_key_size);
    (*inst).alg.max_size = Some(ecdsa_x962_max_size);
    (*inst).alg.digest_size = Some(ecdsa_x962_digest_size);
    (*inst).alg.set_pub_key = Some(ecdsa_x962_set_pub_key);
    (*inst).free = Some(ecdsa_x962_free);
    err = sig_register_instance(tmpl, inst);
    if err != 0 { ecdsa_x962_free(inst); }
    err
}

#[no_mangle]
pub static mut ecdsa_x962_tmpl: crypto_template = crypto_template {
    name: b"x962\0".as_ptr() as *const i8,
    create: Some(ecdsa_x962_create),
    module: THIS_MODULE,
};

// MODULE_ALIAS_CRYPTO("x962");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
