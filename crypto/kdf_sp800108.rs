// SPDX-License-Identifier: GPL-2.0

/*
 * SP800-108 Key-derivation function
 *
 * Copyright (C) 2021, Stephan Mueller <smueller@chronox.de>
 */

/* Dependencies: linux::fips, linux::module, crypto::kdf_sp800108,
 * crypto::internal::kdf_selftest. */

/* SP800-108 CTR KDF implementation */
pub unsafe fn crypto_kdf108_ctr_generate(
    kmd: *mut crypto_shash,
    info: *const kvec,
    info_nvec: u32,
    mut dst: *mut u8,
    mut dlen: u32,
) -> i32 {
    let mut desc = SHASH_DESC_ON_STACK(kmd);
    let mut counter: u32 = cpu_to_be32(1);
    let h: u32 = crypto_shash_digestsize(kmd);
    let dlen_orig = dlen;
    let mut err: i32 = 0;
    let dst_orig = dst;

    (*desc).tfm = kmd;

    while dlen != 0 {
        err = crypto_shash_init(desc);
        if err != 0 {
            break;
        }

        err = crypto_shash_update(
            desc,
            (&mut counter as *mut u32).cast::<u8>(),
            core::mem::size_of::<u32>(),
        );
        if err != 0 {
            break;
        }

        for i in 0..info_nvec {
            let v = &*info.add(i as usize);
            err = crypto_shash_update(desc, v.iov_base.cast::<u8>(), v.iov_len);
            if err != 0 {
                break;
            }
        }
        if err != 0 {
            break;
        }

        if dlen < h {
            let mut tmpbuffer = [0u8; HASH_MAX_DIGESTSIZE];
            err = crypto_shash_final(desc, tmpbuffer.as_mut_ptr());
            if err != 0 {
                break;
            }
            core::ptr::copy_nonoverlapping(tmpbuffer.as_ptr(), dst, dlen as usize);
            memzero_explicit(tmpbuffer.as_mut_ptr().cast(), h as usize);
            break;
        }

        err = crypto_shash_final(desc, dst);
        if err != 0 {
            break;
        }

        dlen -= h;
        dst = dst.add(h as usize);
        counter = cpu_to_be32(be32_to_cpu(counter).wrapping_add(1));
    }

    if err != 0 {
        memzero_explicit(dst_orig.cast(), dlen_orig as usize);
    }
    shash_desc_zero(desc);
    err
}

/* The seeding of the KDF */
pub unsafe fn crypto_kdf108_setkey(
    kmd: *mut crypto_shash,
    key: *const u8,
    keylen: usize,
    ikm: *const u8,
    ikmlen: usize,
) -> i32 {
    let ds = crypto_shash_digestsize(kmd);

    /* SP800-108 does not support IKM */
    if !ikm.is_null() || ikmlen != 0 {
        return -EINVAL;
    }

    /* Check according to SP800-108 section 7.2 */
    if ds as usize > keylen {
        return -EINVAL;
    }

    /* Set the key for the MAC used for the KDF. */
    crypto_shash_setkey(kmd, key, keylen)
}

/*
 * Test vector obtained from
 * http://csrc.nist.gov/groups/STM/cavp/documents/KBKDF800-108/CounterMode.zip
 */
static KDF_CTR_HMAC_SHA256_TV_TEMPLATE: [kdf_testvec; 1] = [kdf_testvec {
    key: b"\xdd\x1d\x91\xb7\xd9\x0b\x2b\xd3\x13\x85\x33\xce\x92\xb2\x72\xfb\xf8\xa3\x69\x31\x6a\xef\xe2\x42\xe6\x59\xcc\x0a\xe2\x38\xaf\xe0\0",
    keylen: 32,
    ikm: core::ptr::null(),
    ikmlen: 0,
    info: kvec {
        iov_base: b"\x01\x32\x2b\x96\xb3\x0a\xcd\x19\x79\x79\x44\x4e\x46\x8e\x1c\x5c\x68\x59\xbf\x1b\x1c\xf9\x51\xb7\xe7\x25\x30\x3e\x23\x7e\x46\xb8\x64\xa1\x45\xfa\xb2\x5e\x51\x7b\x08\xf8\x68\x3d\x03\x15\xbb\x29\x11\xd8\x0a\x0e\x8a\xba\x17\xf3\xb4\x13\xfa\xac".as_ptr().cast(),
        iov_len: 60,
    },
    expected: b"\x10\x62\x13\x42\xbf\xb0\xfd\x40\x04\x6c\x0e\x29\xf2\xcf\xdb\xf0".as_ptr(),
    expectedlen: 16,
}];

unsafe fn crypto_kdf108_init() -> i32 {
    if !IS_ENABLED(CONFIG_CRYPTO_SELFTESTS) {
        return 0;
    }
    let ret = kdf_test(
        &KDF_CTR_HMAC_SHA256_TV_TEMPLATE[0],
        c"hmac(sha256)".as_ptr(),
        crypto_kdf108_setkey,
        crypto_kdf108_ctr_generate,
    );
    if ret != 0 {
        if fips_enabled {
            panic!("alg: self-tests for CTR-KDF (hmac(sha256)) failed (rc={})\n", ret);
        }
        WARN!(1, "alg: self-tests for CTR-KDF (hmac(sha256)) failed (rc={})\n", ret);
    } else if fips_enabled {
        pr_info!("alg: self-tests for CTR-KDF (hmac(sha256)) passed\n");
    }
    ret
}

unsafe fn crypto_kdf108_exit() {}

module_init!(crypto_kdf108_init);
module_exit!(crypto_kdf108_exit);

MODULE_LICENSE!("GPL v2");
MODULE_AUTHOR!("Stephan Mueller <smueller@chronox.de>");
MODULE_DESCRIPTION!("Key Derivation Function conformant to SP800-108");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
