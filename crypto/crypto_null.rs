// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Cryptographic API.
 *
 * Null algorithms, aka Much Ado About Nothing.
 *
 * These are needed for IPsec, and may be useful in general for
 * testing & debugging.
 *
 * The null cipher is compliant with RFC2410.
 *
 * Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
 */

// Dependencies supplied by the kernel crypto subsystem are intentionally
// referenced here rather than reimplemented in this translation.

unsafe fn null_init(_desc: *mut shash_desc) -> i32 {
    0
}

unsafe fn null_update(
    _desc: *mut shash_desc,
    _data: *const u8,
    _len: u32,
) -> i32 {
    0
}

unsafe fn null_final(_desc: *mut shash_desc, _out: *mut u8) -> i32 {
    0
}

unsafe fn null_digest(
    _desc: *mut shash_desc,
    _data: *const u8,
    _len: u32,
    _out: *mut u8,
) -> i32 {
    0
}

unsafe fn null_hash_setkey(
    _tfm: *mut crypto_shash,
    _key: *const u8,
    _keylen: u32,
) -> i32 {
    0
}

unsafe fn null_skcipher_setkey(
    _tfm: *mut crypto_skcipher,
    _key: *const u8,
    _keylen: u32,
) -> i32 {
    0
}

unsafe fn null_skcipher_crypt(req: *mut skcipher_request) -> i32 {
    if (*req).src != (*req).dst {
        memcpy_sglist((*req).dst, (*req).src, (*req).cryptlen);
    }
    0
}

static mut digest_null: shash_alg = shash_alg {
    digestsize: NULL_DIGEST_SIZE,
    setkey: Some(null_hash_setkey),
    init: Some(null_init),
    update: Some(null_update),
    finup: Some(null_digest),
    digest: Some(null_digest),
    final_: Some(null_final),
    base: crypto_alg {
        cra_name: b"digest_null\0".as_ptr() as *const i8,
        cra_driver_name: b"digest_null-generic\0".as_ptr() as *const i8,
        cra_blocksize: NULL_BLOCK_SIZE,
        cra_module: THIS_MODULE,
        ..crypto_alg::default()
    },
};

static mut skcipher_null: skcipher_alg = skcipher_alg {
    base: crypto_alg {
        cra_name: b"ecb(cipher_null)\0".as_ptr() as *const i8,
        cra_driver_name: b"ecb-cipher_null\0".as_ptr() as *const i8,
        cra_priority: 100,
        cra_blocksize: NULL_BLOCK_SIZE,
        cra_ctxsize: 0,
        cra_module: THIS_MODULE,
        ..crypto_alg::default()
    },
    min_keysize: NULL_KEY_SIZE,
    max_keysize: NULL_KEY_SIZE,
    ivsize: NULL_IV_SIZE,
    setkey: Some(null_skcipher_setkey),
    encrypt: Some(null_skcipher_crypt),
    decrypt: Some(null_skcipher_crypt),
    ..skcipher_alg::default()
};

// MODULE_ALIAS_CRYPTO("digest_null");
// MODULE_ALIAS_CRYPTO("ecb(cipher_null)");

unsafe fn crypto_null_mod_init() -> i32 {
    let mut ret: i32 = 0;

    ret = crypto_register_shash(&raw mut digest_null);
    if ret < 0 {
        return ret;
    }

    ret = crypto_register_skcipher(&raw mut skcipher_null);
    if ret < 0 {
        crypto_unregister_shash(&raw mut digest_null);
        return ret;
    }

    0
}

unsafe fn crypto_null_mod_fini() {
    crypto_unregister_shash(&raw mut digest_null);
    crypto_unregister_skcipher(&raw mut skcipher_null);
}

// module_init(crypto_null_mod_init);
// module_exit(crypto_null_mod_fini);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Null Cryptographic Algorithms");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
