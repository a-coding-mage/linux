// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * crypto_shash support for CRC-32C
 *
 *@Article{castagnoli-crc,
 * author =       { Guy Castagnoli and Stefan Braeuer and Martin Herrman},
 * title =        {{Optimization of Cyclic Redundancy-Check Codes with 24
 *                 and 32 Parity Bits}},
 * journal =      IEEE Transactions on Communication,
 * year =         {1993},
 * volume =       {41},
 * number =       {6},
 * pages =        {},
 * month =        {June},
 *}
 *
 * Copyright (c) 2004 Cisco Systems, Inc.
 * Copyright (c) 2008 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Dependencies supplied by the surrounding kernel translation.

const CHKSUM_BLOCK_SIZE: u32 = 1;
const CHKSUM_DIGEST_SIZE: u32 = 4;

#[repr(C)]
struct chksum_ctx {
    key: u32,
}

#[repr(C)]
struct chksum_desc_ctx {
    crc: u32,
}

unsafe fn chksum_init(desc: *mut shash_desc) -> i32 {
    let mctx: *mut chksum_ctx = crypto_shash_ctx((*desc).tfm);
    let ctx: *mut chksum_desc_ctx = shash_desc_ctx(desc);

    (*ctx).crc = (*mctx).key;

    0
}

/*
 * Setting the seed allows arbitrary accumulators and flexible XOR policy
 * If your algorithm starts with ~0, then XOR with ~0 before you set
 * the seed.
 */
unsafe fn chksum_setkey(
    tfm: *mut crypto_shash,
    key: *const u8,
    keylen: u32,
) -> i32 {
    let mctx: *mut chksum_ctx = crypto_shash_ctx(tfm);

    if keylen != core::mem::size_of::<u32>() as u32 {
        return -EINVAL;
    }
    (*mctx).key = get_unaligned_le32(key);
    0
}

unsafe fn chksum_update(
    desc: *mut shash_desc,
    data: *const u8,
    length: u32,
) -> i32 {
    let ctx: *mut chksum_desc_ctx = shash_desc_ctx(desc);

    (*ctx).crc = crc32c((*ctx).crc, data, length);
    0
}

unsafe fn chksum_final(desc: *mut shash_desc, out: *mut u8) -> i32 {
    let ctx: *mut chksum_desc_ctx = shash_desc_ctx(desc);

    put_unaligned_le32(!(*ctx).crc, out);
    0
}

unsafe fn __chksum_finup(
    crcp: *mut u32,
    data: *const u8,
    len: u32,
    out: *mut u8,
) -> i32 {
    put_unaligned_le32(!crc32c(*crcp, data, len), out);
    0
}

unsafe fn chksum_finup(
    desc: *mut shash_desc,
    data: *const u8,
    len: u32,
    out: *mut u8,
) -> i32 {
    let ctx: *mut chksum_desc_ctx = shash_desc_ctx(desc);

    __chksum_finup(&mut (*ctx).crc, data, len, out)
}

unsafe fn chksum_digest(
    desc: *mut shash_desc,
    data: *const u8,
    length: u32,
    out: *mut u8,
) -> i32 {
    let mctx: *mut chksum_ctx = crypto_shash_ctx((*desc).tfm);

    __chksum_finup(&mut (*mctx).key, data, length, out)
}

unsafe fn crc32c_cra_init(tfm: *mut crypto_tfm) -> i32 {
    let mctx: *mut chksum_ctx = crypto_tfm_ctx(tfm);

    (*mctx).key = !0u32;
    0
}

static mut alg: shash_alg = shash_alg {
    digestsize: CHKSUM_DIGEST_SIZE,
    setkey: Some(chksum_setkey),
    init: Some(chksum_init),
    update: Some(chksum_update),
    final_: Some(chksum_final),
    finup: Some(chksum_finup),
    digest: Some(chksum_digest),
    descsize: core::mem::size_of::<chksum_desc_ctx>() as u32,

    base: crypto_alg {
        cra_name: "crc32c",
        cra_driver_name: "crc32c-lib",
        cra_priority: 100,
        cra_flags: CRYPTO_ALG_OPTIONAL_KEY,
        cra_blocksize: CHKSUM_BLOCK_SIZE,
        cra_ctxsize: core::mem::size_of::<chksum_ctx>() as u32,
        cra_module: THIS_MODULE,
        cra_init: Some(crc32c_cra_init),
    },
};

unsafe fn crc32c_mod_init() -> i32 {
    crypto_register_shash(&mut alg)
}

unsafe fn crc32c_mod_fini() {
    crypto_unregister_shash(&mut alg);
}

// module_init(crc32c_mod_init);
// module_exit(crc32c_mod_fini);

// MODULE_AUTHOR("Clay Haapala <chaapala@cisco.com>");
// MODULE_DESCRIPTION("CRC32c (Castagnoli) calculations wrapper for lib/crc32c");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_CRYPTO("crc32c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
