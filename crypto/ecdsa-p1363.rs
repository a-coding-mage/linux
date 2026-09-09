// SPDX-License-Identifier: GPL-2.0
/*
 * ECDSA P1363 signature encoding
 *
 * Copyright (c) 2024 Intel Corporation
 */

// Dependencies supplied by the Linux kernel crypto and module interfaces.

#[repr(C)]
pub struct ecdsa_p1363_ctx {
    pub child: *mut crypto_sig,
}

unsafe fn ecdsa_p1363_verify(
    tfm: *mut crypto_sig,
    src: *const core::ffi::c_void,
    slen: u32,
    digest: *const core::ffi::c_void,
    dlen: u32,
) -> i32 {
    let ctx = crypto_sig_ctx(tfm);
    let keylen = div_round_up_pow2(crypto_sig_keysize((*ctx).child), BITS_PER_BYTE);
    let ndigits = div_round_up_pow2(keylen, core::mem::size_of::<u64>() as u32);
    let mut sig: ecdsa_raw_sig = core::mem::zeroed();

    if slen != 2 * keylen {
        return -EINVAL;
    }

    ecc_digits_from_bytes(src, keylen, sig.r.as_mut_ptr(), ndigits);
    ecc_digits_from_bytes(
        (src as *const u8).add(keylen as usize) as *const core::ffi::c_void,
        keylen,
        sig.s.as_mut_ptr(),
        ndigits,
    );

    crypto_sig_verify((*ctx).child, &sig as *const ecdsa_raw_sig as *const core::ffi::c_void,
                      core::mem::size_of::<ecdsa_raw_sig>() as u32, digest, dlen)
}

unsafe fn ecdsa_p1363_key_size(tfm: *mut crypto_sig) -> u32 {
    let ctx = crypto_sig_ctx(tfm);
    crypto_sig_keysize((*ctx).child)
}

unsafe fn ecdsa_p1363_max_size(tfm: *mut crypto_sig) -> u32 {
    let ctx = crypto_sig_ctx(tfm);
    2 * div_round_up_pow2(crypto_sig_keysize((*ctx).child), BITS_PER_BYTE)
}

unsafe fn ecdsa_p1363_digest_size(tfm: *mut crypto_sig) -> u32 {
    let ctx = crypto_sig_ctx(tfm);
    crypto_sig_digestsize((*ctx).child)
}

unsafe fn ecdsa_p1363_set_pub_key(
    tfm: *mut crypto_sig,
    key: *const core::ffi::c_void,
    keylen: u32,
) -> i32 {
    let ctx = crypto_sig_ctx(tfm);
    crypto_sig_set_pubkey((*ctx).child, key, keylen)
}

unsafe fn ecdsa_p1363_init_tfm(tfm: *mut crypto_sig) -> i32 {
    let inst = sig_alg_instance(tfm);
    let spawn = sig_instance_ctx(inst);
    let ctx = crypto_sig_ctx(tfm);
    let child_tfm = crypto_spawn_sig(spawn);

    if is_err(child_tfm) {
        return ptr_err(child_tfm);
    }

    (*ctx).child = child_tfm;
    0
}

unsafe fn ecdsa_p1363_exit_tfm(tfm: *mut crypto_sig) {
    let ctx = crypto_sig_ctx(tfm);
    crypto_free_sig((*ctx).child);
}

unsafe fn ecdsa_p1363_free(inst: *mut sig_instance) {
    let spawn = sig_instance_ctx(inst);
    crypto_drop_sig(spawn);
    kfree(inst as *mut core::ffi::c_void);
}

unsafe fn ecdsa_p1363_create(
    tmpl: *mut crypto_template,
    tb: *mut *mut rtattr,
) -> i32 {
    let mut spawn: *mut crypto_sig_spawn;
    let inst: *mut sig_instance;
    let ecdsa_alg: *mut sig_alg;
    let mut mask: u32 = 0;
    let mut err: i32;

    err = crypto_check_attr_type(tb, CRYPTO_ALG_TYPE_SIG, &mut mask);
    if err != 0 {
        return err;
    }

    let size = core::mem::size_of::<sig_instance>() + core::mem::size_of::<crypto_sig_spawn>();
    inst = kzalloc(size, GFP_KERNEL) as *mut sig_instance;
    if inst.is_null() {
        return -ENOMEM;
    }

    spawn = sig_instance_ctx(inst);

    err = crypto_grab_sig(
        spawn,
        sig_crypto_instance(inst),
        crypto_attr_alg_name((*tb.add(1))),
        0,
        mask,
    );
    if err != 0 {
        ecdsa_p1363_free(inst);
        return err;
    }

    ecdsa_alg = crypto_spawn_sig_alg(spawn);

    err = -EINVAL;
    if c_strncmp((*ecdsa_alg).base.cra_name, b"ecdsa\0".as_ptr(), 5) != 0 {
        ecdsa_p1363_free(inst);
        return err;
    }

    err = crypto_inst_setname(sig_crypto_instance(inst), (*tmpl).name, &(*ecdsa_alg).base);
    if err != 0 {
        ecdsa_p1363_free(inst);
        return err;
    }

    (*inst).alg.base.cra_priority = (*ecdsa_alg).base.cra_priority;
    (*inst).alg.base.cra_ctxsize = core::mem::size_of::<ecdsa_p1363_ctx>();

    (*inst).alg.init = Some(ecdsa_p1363_init_tfm);
    (*inst).alg.exit = Some(ecdsa_p1363_exit_tfm);

    (*inst).alg.verify = Some(ecdsa_p1363_verify);
    (*inst).alg.key_size = Some(ecdsa_p1363_key_size);
    (*inst).alg.max_size = Some(ecdsa_p1363_max_size);
    (*inst).alg.digest_size = Some(ecdsa_p1363_digest_size);
    (*inst).alg.set_pub_key = Some(ecdsa_p1363_set_pub_key);

    (*inst).free = Some(ecdsa_p1363_free);

    err = sig_register_instance(tmpl, inst);
    if err != 0 {
        ecdsa_p1363_free(inst);
    }
    err
}

#[repr(C)]
pub struct crypto_template {
    pub name: *const u8,
    pub create: Option<unsafe fn(*mut crypto_template, *mut *mut rtattr) -> i32>,
    pub module: *mut core::ffi::c_void,
}

#[no_mangle]
pub static mut ecdsa_p1363_tmpl: crypto_template = crypto_template {
    name: b"p1363\0".as_ptr(),
    create: Some(ecdsa_p1363_create),
    module: core::ptr::null_mut(), // THIS_MODULE
};

// MODULE_ALIAS_CRYPTO("p1363");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
