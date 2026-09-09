/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Shared crypto simd helpers
 */

// C dependencies supplied by the surrounding kernel translation unit:
// asm/simd.h, linux/percpu.h, and linux/types.h.

#[repr(C)]
pub struct simd_aead_alg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct aead_alg {
    _private: [u8; 0],
}

extern "C" {
    pub fn simd_register_aeads_compat(
        algs: *mut aead_alg,
        count: ::core::ffi::c_int,
        simd_algs: *mut *mut simd_aead_alg,
    ) -> ::core::ffi::c_int;

    pub fn simd_unregister_aeads(
        algs: *mut aead_alg,
        count: ::core::ffi::c_int,
        simd_algs: *mut *mut simd_aead_alg,
    );

    // Supplied by asm/simd.h.
    pub fn may_use_simd() -> bool;
}

/*
 * crypto_simd_usable() - is it allowed at this time to use SIMD instructions or
 *                          access the SIMD register file?
 *
 * This delegates to may_use_simd(), except that this also returns false if SIMD
 * in crypto code has been temporarily disabled on this CPU by the crypto
 * self-tests, in order to test the no-SIMD fallback code.  This override is
 * currently limited to configurations where the "full" self-tests are enabled,
 * because it might be a bit too invasive to be part of the "fast" self-tests.
 */
#[cfg(feature = "CONFIG_CRYPTO_SELFTESTS_FULL")]
extern "C" {
    // DECLARE_PER_CPU(bool, crypto_simd_disabled_for_test)
    pub static mut crypto_simd_disabled_for_test: bool;
}

#[cfg(feature = "CONFIG_CRYPTO_SELFTESTS_FULL")]
#[inline]
pub unsafe fn crypto_simd_usable() -> bool {
    may_use_simd() && !crypto_simd_disabled_for_test
}

#[cfg(not(feature = "CONFIG_CRYPTO_SELFTESTS_FULL"))]
#[inline]
pub unsafe fn crypto_simd_usable() -> bool {
    may_use_simd()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
