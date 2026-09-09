// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Crypto API support for BLAKE2b
 *
 * Copyright 2025 Google LLC
 */

// Dependencies supplied by the kernel crypto and BLAKE2b interfaces.

#[repr(C)]
struct blake2b_tfm_ctx {
    keylen: core::ffi::c_uint,
    key: [u8; BLAKE2B_KEY_SIZE],
}

unsafe fn crypto_blake2b_setkey(
    tfm: *mut crypto_shash,
    key: *const u8,
    keylen: core::ffi::c_uint,
) -> core::ffi::c_int {
    let tctx = crypto_shash_ctx(tfm) as *mut blake2b_tfm_ctx;

    if keylen > BLAKE2B_KEY_SIZE as core::ffi::c_uint {
        return -EINVAL;
    }
    core::ptr::copy_nonoverlapping(key, (*tctx).key.as_mut_ptr(), keylen as usize);
    (*tctx).keylen = keylen;
    0
}

unsafe fn blake2b_ctx(desc: *mut shash_desc) -> *mut blake2b_ctx {
    shash_desc_ctx(desc) as *mut blake2b_ctx
}

unsafe fn crypto_blake2b_init(desc: *mut shash_desc) -> core::ffi::c_int {
    let tctx = crypto_shash_ctx((*desc).tfm) as *const blake2b_tfm_ctx;
    let digestsize = crypto_shash_digestsize((*desc).tfm);

    blake2b_init_key(
        blake2b_ctx(desc),
        digestsize,
        (*tctx).key.as_ptr(),
        (*tctx).keylen,
    );
    0
}

unsafe fn crypto_blake2b_update(
    desc: *mut shash_desc,
    data: *const u8,
    len: core::ffi::c_uint,
) -> core::ffi::c_int {
    blake2b_update(blake2b_ctx(desc), data, len);
    0
}

unsafe fn crypto_blake2b_final(
    desc: *mut shash_desc,
    out: *mut u8,
) -> core::ffi::c_int {
    blake2b_final(blake2b_ctx(desc), out);
    0
}

unsafe fn crypto_blake2b_digest(
    desc: *mut shash_desc,
    data: *const u8,
    len: core::ffi::c_uint,
    out: *mut u8,
) -> core::ffi::c_int {
    let tctx = crypto_shash_ctx((*desc).tfm) as *const blake2b_tfm_ctx;
    let digestsize = crypto_shash_digestsize((*desc).tfm);

    blake2b(
        (*tctx).key.as_ptr(),
        (*tctx).keylen,
        data,
        len,
        out,
        digestsize,
    );
    0
}

// The C BLAKE2B_ALG initializer is represented directly by the kernel's
// externally supplied `shash_alg` type and constants.
macro_rules! BLAKE2B_ALG {
    ($name:expr, $digest_size:expr) => {
        shash_alg {
            base: crypto_alg {
                cra_name: $name,
                cra_driver_name: concat!($name, "-lib"),
                cra_priority: 300,
                cra_flags: CRYPTO_ALG_OPTIONAL_KEY,
                cra_blocksize: BLAKE2B_BLOCK_SIZE,
                cra_ctxsize: core::mem::size_of::<blake2b_tfm_ctx>(),
                cra_module: THIS_MODULE,
            },
            digestsize: $digest_size,
            setkey: Some(crypto_blake2b_setkey),
            init: Some(crypto_blake2b_init),
            update: Some(crypto_blake2b_update),
            final_: Some(crypto_blake2b_final),
            digest: Some(crypto_blake2b_digest),
            descsize: core::mem::size_of::<blake2b_ctx>(),
        }
    };
}

static mut algs: [shash_alg; 4] = [
    BLAKE2B_ALG!("blake2b-160", BLAKE2B_160_HASH_SIZE),
    BLAKE2B_ALG!("blake2b-256", BLAKE2B_256_HASH_SIZE),
    BLAKE2B_ALG!("blake2b-384", BLAKE2B_384_HASH_SIZE),
    BLAKE2B_ALG!("blake2b-512", BLAKE2B_512_HASH_SIZE),
];

unsafe fn crypto_blake2b_mod_init() -> core::ffi::c_int {
    crypto_register_shashes(algs.as_mut_ptr(), algs.len())
}

unsafe fn crypto_blake2b_mod_exit() {
    crypto_unregister_shashes(algs.as_mut_ptr(), algs.len());
}

// module_init(crypto_blake2b_mod_init);
// module_exit(crypto_blake2b_mod_exit);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Crypto API support for BLAKE2b");
// MODULE_ALIAS_CRYPTO("blake2b-160");
// MODULE_ALIAS_CRYPTO("blake2b-160-lib");
// MODULE_ALIAS_CRYPTO("blake2b-256");
// MODULE_ALIAS_CRYPTO("blake2b-256-lib");
// MODULE_ALIAS_CRYPTO("blake2b-384");
// MODULE_ALIAS_CRYPTO("blake2b-384-lib");
// MODULE_ALIAS_CRYPTO("blake2b-512");
// MODULE_ALIAS_CRYPTO("blake2b-512-lib");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
