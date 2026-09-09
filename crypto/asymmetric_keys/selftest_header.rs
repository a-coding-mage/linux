/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Helper function for self-testing PKCS#7 signature verification.
 *
 * Copyright (C) 2024 Joachim Vandersmissen <git@jvdsn.com>
 */

extern "C" {
    pub fn fips_signature_selftest(
        name: *const ::core::ffi::c_char,
        keys: *const u8,
        keys_len: usize,
        data: *const u8,
        data_len: usize,
        sig: *const u8,
        sig_len: usize,
    );
}

// CONFIG_FIPS_SIGNATURE_SELFTEST_RSA controls whether the external
// implementation is available at build time.
#[cfg(CONFIG_FIPS_SIGNATURE_SELFTEST_RSA)]
extern "C" {
    pub fn fips_signature_selftest_rsa();
}

#[cfg(not(CONFIG_FIPS_SIGNATURE_SELFTEST_RSA))]
#[inline]
pub fn fips_signature_selftest_rsa() {}

// CONFIG_FIPS_SIGNATURE_SELFTEST_ECDSA controls whether the external
// implementation is available at build time.
#[cfg(CONFIG_FIPS_SIGNATURE_SELFTEST_ECDSA)]
extern "C" {
    pub fn fips_signature_selftest_ecdsa();
}

#[cfg(not(CONFIG_FIPS_SIGNATURE_SELFTEST_ECDSA))]
#[inline]
pub fn fips_signature_selftest_ecdsa() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
