// SPDX-License-Identifier: GPL-2.0
/*
 * Implementation of HKDF ("HMAC-based Extract-and-Expand Key Derivation
 * Function"), aka RFC 5869.  See also the original paper (Krawczyk 2010):
 * "Cryptographic Extraction and Key Derivation: The HKDF Scheme".
 *
 * This is used to derive keys from the fscrypt master keys (or from the
 * "software secrets" which hardware derives from the fscrypt master keys, in
 * the case that the fscrypt master keys are hardware-wrapped keys).
 *
 * Copyright 2019 Google LLC
 */

// Dependency declarations supplied by fscrypt_private.h.
extern "C" {
    static SHA512_DIGEST_SIZE: usize;

    fn hmac_sha512_usingrawkey(
        key: *const u8,
        keylen: usize,
        data: *const u8,
        datalen: usize,
        out: *mut u8,
    );
    fn hmac_sha512_preparekey(hkdf: *mut hmac_sha512_key, key: *const u8, keylen: usize);
    fn memzero_explicit(ptr: *mut u8, len: usize);
    fn hmac_sha512_init(ctx: *mut hmac_sha512_ctx, hkdf: *const hmac_sha512_key);
    fn hmac_sha512_update(ctx: *mut hmac_sha512_ctx, data: *const u8, len: usize);
    fn hmac_sha512_final(ctx: *mut hmac_sha512_ctx, out: *mut u8);
    fn memcpy(dest: *mut u8, src: *const u8, len: usize) -> *mut u8;
    fn WARN_ON_ONCE(condition: bool);
}

#[repr(C)]
pub struct hmac_sha512_key {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hmac_sha512_ctx {
    _private: [u8; 0],
}

const HKDF_HASHLEN: usize = SHA512_DIGEST_SIZE;

pub unsafe fn fscrypt_init_hkdf(
    hkdf: *mut hmac_sha512_key,
    master_key: *const u8,
    master_key_size: u32,
) {
    static DEFAULT_SALT: [u8; HKDF_HASHLEN] = [0; HKDF_HASHLEN];
    let mut prk = [0u8; HKDF_HASHLEN];

    hmac_sha512_usingrawkey(
        DEFAULT_SALT.as_ptr(),
        core::mem::size_of_val(&DEFAULT_SALT),
        master_key,
        master_key_size as usize,
        prk.as_mut_ptr(),
    );
    hmac_sha512_preparekey(hkdf, prk.as_ptr(), core::mem::size_of_val(&prk));
    memzero_explicit(prk.as_mut_ptr(), core::mem::size_of_val(&prk));
}

pub unsafe fn fscrypt_hkdf_expand(
    hkdf: *const hmac_sha512_key,
    context: u8,
    info: *const u8,
    infolen: u32,
    okm: *mut u8,
    okmlen: u32,
) {
    let mut ctx: hmac_sha512_ctx = core::mem::zeroed();
    let mut counter: u8 = 1;
    let mut tmp = [0u8; HKDF_HASHLEN];

    WARN_ON_ONCE((okmlen as usize) > 255 * HKDF_HASHLEN);

    let mut i: usize = 0;
    while i < okmlen as usize {
        hmac_sha512_init(&mut ctx, hkdf);
        if i != 0 {
            hmac_sha512_update(&mut ctx, okm.add(i - HKDF_HASHLEN), HKDF_HASHLEN);
        }
        hmac_sha512_update(&mut ctx, b"fscrypt\0".as_ptr(), 8);
        hmac_sha512_update(&mut ctx, &context, 1);
        hmac_sha512_update(&mut ctx, info, infolen as usize);
        hmac_sha512_update(&mut ctx, &counter, 1);
        if okmlen as usize - i < HKDF_HASHLEN {
            hmac_sha512_final(&mut ctx, tmp.as_mut_ptr());
            memcpy(okm.add(i), tmp.as_ptr(), okmlen as usize - i);
            memzero_explicit(tmp.as_mut_ptr(), core::mem::size_of_val(&tmp));
        } else {
            hmac_sha512_final(&mut ctx, okm.add(i));
        }
        counter = counter.wrapping_add(1);
        i += HKDF_HASHLEN;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
