// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2012 Xyratex Technology Limited
 */

/*
 * This is crypto api shash wrappers to crc32_le.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

const CHKSUM_BLOCK_SIZE: usize = 1;
const CHKSUM_DIGEST_SIZE: usize = 4;

/** No default init with ~0 */
unsafe extern "C" fn crc32_cra_init(tfm: *mut crypto_tfm) -> i32 {
    let key: *mut u32 = crypto_tfm_ctx(tfm);

    *key = 0;

    0
}

/*
 * Setting the seed allows arbitrary accumulators and flexible XOR policy
 * If your algorithm starts with ~0, then XOR with ~0 before you set
 * the seed.
 */
unsafe extern "C" fn crc32_setkey(
    hash: *mut crypto_shash,
    key: *const u8,
    keylen: u32,
) -> i32 {
    let mctx: *mut u32 = crypto_shash_ctx(hash);

    if keylen as usize != core::mem::size_of::<u32>() {
        return -EINVAL;
    }
    *mctx = get_unaligned_le32(key);
    0
}

unsafe extern "C" fn crc32_init(desc: *mut shash_desc) -> i32 {
    let mctx: *mut u32 = crypto_shash_ctx((*desc).tfm);
    let crcp: *mut u32 = shash_desc_ctx(desc);

    *crcp = *mctx;

    0
}

unsafe extern "C" fn crc32_update(
    desc: *mut shash_desc,
    data: *const u8,
    len: u32,
) -> i32 {
    let crcp: *mut u32 = shash_desc_ctx(desc);

    *crcp = crc32_le(*crcp, data, len);
    0
}

/* No final XOR 0xFFFFFFFF, like crc32_le */
unsafe extern "C" fn __crc32_finup(
    crcp: *mut u32,
    data: *const u8,
    len: u32,
    out: *mut u8,
) -> i32 {
    put_unaligned_le32(crc32_le(*crcp, data, len), out);
    0
}

unsafe extern "C" fn crc32_finup(
    desc: *mut shash_desc,
    data: *const u8,
    len: u32,
    out: *mut u8,
) -> i32 {
    __crc32_finup(shash_desc_ctx(desc), data, len, out)
}

unsafe extern "C" fn crc32_final(desc: *mut shash_desc, out: *mut u8) -> i32 {
    let crcp: *mut u32 = shash_desc_ctx(desc);

    put_unaligned_le32(*crcp, out);
    0
}

unsafe extern "C" fn crc32_digest(
    desc: *mut shash_desc,
    data: *const u8,
    len: u32,
    out: *mut u8,
) -> i32 {
    __crc32_finup(crypto_shash_ctx((*desc).tfm), data, len, out)
}

static mut alg: shash_alg = shash_alg {
    setkey: Some(crc32_setkey),
    init: Some(crc32_init),
    update: Some(crc32_update),
    final_: Some(crc32_final),
    finup: Some(crc32_finup),
    digest: Some(crc32_digest),
    descsize: core::mem::size_of::<u32>(),
    digestsize: CHKSUM_DIGEST_SIZE,

    base: crypto_alg {
        cra_name: b"crc32\0".as_ptr() as *const i8,
        cra_driver_name: b"crc32-lib\0".as_ptr() as *const i8,
        cra_priority: 100,
        cra_flags: CRYPTO_ALG_OPTIONAL_KEY,
        cra_blocksize: CHKSUM_BLOCK_SIZE,
        cra_ctxsize: core::mem::size_of::<u32>(),
        cra_module: THIS_MODULE,
        cra_init: Some(crc32_cra_init),
    },
};

unsafe extern "C" fn crc32_mod_init() -> i32 {
    crypto_register_shash(&raw mut alg)
}

unsafe extern "C" fn crc32_mod_fini() {
    crypto_unregister_shash(&raw mut alg);
}

module_init!(crc32_mod_init);
module_exit!(crc32_mod_fini);

module_author!("Alexander Boyko <alexander_boyko@xyratex.com>");
module_description!("CRC32 calculations wrapper for lib/crc32");
module_license!("GPL");
module_alias_crypto!("crc32");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
