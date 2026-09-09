/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies corresponding to <crypto/b128ops.h>,
// <crypto/internal/skcipher.h>, and <linux/fips.h> are supplied externally.

pub const XTS_BLOCK_SIZE: usize = 16;
pub const XTS_FORBID_WEAK_KEYS: i32 = 1 << 0;

extern "C" {
    static fips_enabled: bool;

    fn crypto_memneq(a: *const u8, b: *const u8, size: usize) -> i32;
    fn crypto_skcipher_get_flags(tfm: *mut crypto_skcipher) -> u32;
}

#[repr(C)]
pub struct crypto_skcipher {
    _private: [u8; 0],
}

pub unsafe fn __xts_verify_key(key: *const u8, keylen: usize, flags: i32) -> i32 {
    /*
     * key consists of keys of equal size concatenated, therefore
     * the length must be even.
     */
    if keylen % 2 != 0 {
        return -EINVAL;
    }

    /*
     * In FIPS mode only a combined key length of either 256 or
     * 512 bits is allowed, c.f. FIPS 140-3 IG C.I.
     */
    if fips_enabled && keylen != 32 && keylen != 64 {
        return -EINVAL;
    }

    /*
     * Ensure that the AES and tweak key are not identical when
     * in FIPS mode or the FORBID_WEAK_KEYS flag is set.
     */
    if (fips_enabled || (flags & XTS_FORBID_WEAK_KEYS) != 0)
        && crypto_memneq(key, key.add(keylen / 2), keylen / 2) == 0
    {
        return -EINVAL;
    }

    0
}

pub unsafe fn xts_verify_key(
    tfm: *mut crypto_skcipher,
    key: *const u8,
    keylen: u32,
) -> i32 {
    let flags: i32 = if (crypto_skcipher_get_flags(tfm)
        & CRYPTO_TFM_REQ_FORBID_WEAK_KEYS)
        != 0
    {
        XTS_FORBID_WEAK_KEYS
    } else {
        0
    };

    __xts_verify_key(key, keylen as usize, flags)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
