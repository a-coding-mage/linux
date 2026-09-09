// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ECB: Electronic CodeBook mode
 *
 * Copyright (c) 2006 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Dependencies supplied by the surrounding kernel-crypto Rust environment.

unsafe fn crypto_ecb_crypt(
    cipher: *mut crypto_cipher,
    mut src: *const u8,
    mut dst: *mut u8,
    mut nbytes: c_uint,
    final_: bool,
    fn_: Option<unsafe extern "C" fn(*mut crypto_tfm, *mut u8, *const u8)>,
) -> c_int {
    let bsize: c_uint = crypto_cipher_blocksize(cipher);

    while nbytes >= bsize {
        fn_.unwrap()(crypto_cipher_tfm(cipher), dst, src);

        src = src.add(bsize as usize);
        dst = dst.add(bsize as usize);
        nbytes -= bsize;
    }

    if nbytes != 0 && final_ { -EINVAL } else { nbytes as c_int }
}

unsafe extern "C" fn crypto_ecb_encrypt2(
    tfm: *mut crypto_lskcipher,
    src: *const u8,
    dst: *mut u8,
    len: c_uint,
    _iv: *mut u8,
    flags: u32,
) -> c_int {
    let ctx: *mut *mut crypto_cipher = crypto_lskcipher_ctx(tfm);
    let cipher: *mut crypto_cipher = *ctx;

    crypto_ecb_crypt(
        cipher,
        src,
        dst,
        len,
        (flags & CRYPTO_LSKCIPHER_FLAG_FINAL) != 0,
        (*crypto_cipher_alg(cipher)).cia_encrypt,
    )
}

unsafe extern "C" fn crypto_ecb_decrypt2(
    tfm: *mut crypto_lskcipher,
    src: *const u8,
    dst: *mut u8,
    len: c_uint,
    _iv: *mut u8,
    flags: u32,
) -> c_int {
    let ctx: *mut *mut crypto_cipher = crypto_lskcipher_ctx(tfm);
    let cipher: *mut crypto_cipher = *ctx;

    crypto_ecb_crypt(
        cipher,
        src,
        dst,
        len,
        (flags & CRYPTO_LSKCIPHER_FLAG_FINAL) != 0,
        (*crypto_cipher_alg(cipher)).cia_decrypt,
    )
}

unsafe extern "C" fn lskcipher_setkey_simple2(
    tfm: *mut crypto_lskcipher,
    key: *const u8,
    keylen: c_uint,
) -> c_int {
    let ctx: *mut *mut crypto_cipher = crypto_lskcipher_ctx(tfm);
    let cipher: *mut crypto_cipher = *ctx;

    crypto_cipher_clear_flags(cipher, CRYPTO_TFM_REQ_MASK);
    crypto_cipher_set_flags(cipher, crypto_lskcipher_get_flags(tfm) & CRYPTO_TFM_REQ_MASK);
    crypto_cipher_setkey(cipher, key, keylen)
}

unsafe extern "C" fn lskcipher_init_tfm_simple2(tfm: *mut crypto_lskcipher) -> c_int {
    let inst: *mut lskcipher_instance = lskcipher_alg_instance(tfm);
    let ctx: *mut *mut crypto_cipher = crypto_lskcipher_ctx(tfm);
    let spawn: *mut crypto_cipher_spawn = lskcipher_instance_ctx(inst);
    let cipher: *mut crypto_cipher = crypto_spawn_cipher(spawn);
    if IS_ERR(cipher) { return PTR_ERR(cipher); }
    *ctx = cipher;
    0
}

unsafe extern "C" fn lskcipher_exit_tfm_simple2(tfm: *mut crypto_lskcipher) {
    let ctx: *mut *mut crypto_cipher = crypto_lskcipher_ctx(tfm);
    crypto_free_cipher(*ctx);
}

unsafe extern "C" fn lskcipher_free_instance_simple2(inst: *mut lskcipher_instance) {
    crypto_drop_cipher(lskcipher_instance_ctx(inst));
    kfree(inst as *mut c_void);
}

unsafe extern "C" fn lskcipher_alloc_instance_simple2(
    tmpl: *mut crypto_template,
    tb: *mut *mut rtattr,
) -> *mut lskcipher_instance {
    let mut spawn: *mut crypto_cipher_spawn;
    let inst: *mut lskcipher_instance;
    let cipher_alg: *mut crypto_alg;
    let mut mask: u32 = 0;
    let mut err: c_int;

    err = crypto_check_attr_type(tb, CRYPTO_ALG_TYPE_LSKCIPHER, &mut mask);
    if err != 0 { return ERR_PTR(err); }
    inst = kzalloc(core::mem::size_of::<lskcipher_instance>() + core::mem::size_of::<crypto_cipher_spawn>(), GFP_KERNEL) as *mut lskcipher_instance;
    if inst.is_null() { return ERR_PTR(-ENOMEM); }
    spawn = lskcipher_instance_ctx(inst);
    err = crypto_grab_cipher(spawn, lskcipher_crypto_instance(inst), crypto_attr_alg_name(*tb.add(1)), 0, mask);
    if err != 0 { lskcipher_free_instance_simple2(inst); return ERR_PTR(err); }
    cipher_alg = crypto_spawn_cipher_alg(spawn);
    err = crypto_inst_setname(lskcipher_crypto_instance(inst), (*tmpl).name, cipher_alg);
    if err != 0 { lskcipher_free_instance_simple2(inst); return ERR_PTR(err); }

    (*inst).free = Some(lskcipher_free_instance_simple2);
    (*inst).alg.co.base.cra_blocksize = (*cipher_alg).cra_blocksize;
    (*inst).alg.co.base.cra_alignmask = (*cipher_alg).cra_alignmask;
    (*inst).alg.co.base.cra_priority = (*cipher_alg).cra_priority;
    (*inst).alg.co.min_keysize = (*cipher_alg).cra_cipher.cia_min_keysize;
    (*inst).alg.co.max_keysize = (*cipher_alg).cra_cipher.cia_max_keysize;
    (*inst).alg.co.ivsize = (*cipher_alg).cra_blocksize;
    (*inst).alg.co.base.cra_ctxsize = core::mem::size_of::<*mut crypto_cipher>();
    (*inst).alg.setkey = Some(lskcipher_setkey_simple2);
    (*inst).alg.init = Some(lskcipher_init_tfm_simple2);
    (*inst).alg.exit = Some(lskcipher_exit_tfm_simple2);
    inst
}

unsafe extern "C" fn crypto_ecb_create2(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> c_int {
    let inst = lskcipher_alloc_instance_simple2(tmpl, tb);
    if IS_ERR(inst) { return PTR_ERR(inst); }
    (*inst).alg.co.ivsize = 0;
    (*inst).alg.encrypt = Some(crypto_ecb_encrypt2);
    (*inst).alg.decrypt = Some(crypto_ecb_decrypt2);
    let err = lskcipher_register_instance(tmpl, inst);
    if err != 0 { ((*inst).free.unwrap())(inst); }
    err
}

unsafe extern "C" fn crypto_ecb_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> c_int {
    let inst = lskcipher_alloc_instance_simple(tmpl, tb);
    if IS_ERR(inst) { return crypto_ecb_create2(tmpl, tb); }
    let spawn = lskcipher_instance_ctx(inst);
    let cipher_alg = crypto_lskcipher_spawn_alg(spawn);
    (*inst).alg.co.ivsize = 0;
    if (*cipher_alg).co.ivsize != 0 { return -EINVAL; }
    (*inst).alg.co.base.cra_ctxsize = (*cipher_alg).co.base.cra_ctxsize;
    (*inst).alg.setkey = (*cipher_alg).setkey;
    (*inst).alg.encrypt = (*cipher_alg).encrypt;
    (*inst).alg.decrypt = (*cipher_alg).decrypt;
    (*inst).alg.init = (*cipher_alg).init;
    (*inst).alg.exit = (*cipher_alg).exit;
    let err = lskcipher_register_instance(tmpl, inst);
    if err != 0 { ((*inst).free.unwrap())(inst); }
    err
}

static mut crypto_ecb_tmpl: crypto_template = crypto_template {
    name: "ecb",
    create: Some(crypto_ecb_create),
    module: THIS_MODULE,
};

unsafe extern "C" fn crypto_ecb_module_init() -> c_int {
    crypto_register_template(&mut crypto_ecb_tmpl)
}

unsafe extern "C" fn crypto_ecb_module_exit() {
    crypto_unregister_template(&mut crypto_ecb_tmpl);
}

module_init!(crypto_ecb_module_init);
module_exit!(crypto_ecb_module_exit);

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("ECB block cipher mode of operation");
// MODULE_ALIAS_CRYPTO("ecb");
// MODULE_IMPORT_NS("CRYPTO_INTERNAL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
