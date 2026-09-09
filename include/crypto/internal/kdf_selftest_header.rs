/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (C) 2021, Stephan Mueller <smueller@chronox.de>
 */

/* Translated from kdf_selftest.h. */

#[repr(C)]
pub struct kdf_testvec {
    pub key: *mut u8,
    pub keylen: usize,
    pub ikm: *mut u8,
    pub ikmlen: usize,
    pub info: kvec,
    pub expected: *mut u8,
    pub expectedlen: usize,
}

#[inline]
pub unsafe fn kdf_test(
    test: *const kdf_testvec,
    name: *const core::ffi::c_char,
    crypto_kdf_setkey: unsafe extern "C" fn(
        kmd: *mut crypto_shash,
        key: *const u8,
        keylen: usize,
        ikm: *const u8,
        ikmlen: usize,
    ) -> i32,
    crypto_kdf_generate: unsafe extern "C" fn(
        kmd: *mut crypto_shash,
        info: *const kvec,
        info_nvec: u32,
        dst: *mut u8,
        dlen: u32,
    ) -> i32,
) -> i32 {
    let mut kmd: *mut crypto_shash;
    let mut ret: i32;
    let buf: *mut u8 = kzalloc((*test).expectedlen, GFP_KERNEL);

    if buf.is_null() {
        return -ENOMEM;
    }

    kmd = crypto_alloc_shash(name, 0, 0);
    if IS_ERR(kmd) {
        pr_err(
            b"alg: kdf: could not allocate hash handle for %s\n\0".as_ptr()
                as *const core::ffi::c_char,
            name,
        );
        kfree(buf);
        return -ENOMEM;
    }

    ret = crypto_kdf_setkey(
        kmd,
        (*test).key,
        (*test).keylen,
        (*test).ikm,
        (*test).ikmlen,
    );
    if ret != 0 {
        pr_err(b"alg: kdf: could not set key derivation key\n\0".as_ptr());
    } else {
        ret = crypto_kdf_generate(
            kmd,
            &(*test).info,
            1,
            buf,
            (*test).expectedlen as u32,
        );
        if ret != 0 {
            pr_err(b"alg: kdf: could not obtain key data\n\0".as_ptr());
        } else {
            ret = memcmp((*test).expected as *const _, buf as *const _, (*test).expectedlen);
            if ret != 0 {
                ret = -EINVAL;
            }
        }
    }

    crypto_free_shash(kmd);
    kfree(buf);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
