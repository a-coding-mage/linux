/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Support for verifying ML-DSA signatures
 *
 * Copyright 2025 Google LLC
 */

/* linux/types.h: u8 -> u8, s32 -> i32, and size_t -> usize. */

/* Identifier for an ML-DSA parameter set */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mldsa_alg {
    MLDSA44, /* ML-DSA-44 */
    MLDSA65, /* ML-DSA-65 */
    MLDSA87, /* ML-DSA-87 */
}

/* Lengths of ML-DSA public keys and signatures in bytes */
pub const MLDSA44_PUBLIC_KEY_SIZE: usize = 1312;
pub const MLDSA65_PUBLIC_KEY_SIZE: usize = 1952;
pub const MLDSA87_PUBLIC_KEY_SIZE: usize = 2592;
pub const MLDSA44_SIGNATURE_SIZE: usize = 2420;
pub const MLDSA65_SIGNATURE_SIZE: usize = 3309;
pub const MLDSA87_SIGNATURE_SIZE: usize = 4627;

/**
 * mldsa_verify() - Verify an ML-DSA signature
 * @alg: The ML-DSA parameter set to use
 * @sig: The signature
 * @sig_len: Length of the signature in bytes.  Should match the
 *           MLDSA*_SIGNATURE_SIZE constant associated with @alg,
 *           otherwise -EBADMSG will be returned.
 * @msg: The message
 * @msg_len: Length of the message in bytes
 * @pk: The public key
 * @pk_len: Length of the public key in bytes.  Should match the
 *          MLDSA*_PUBLIC_KEY_SIZE constant associated with @alg,
 *          otherwise -EBADMSG will be returned.
 *
 * This verifies a signature using pure ML-DSA with the specified parameter set.
 * The context string is assumed to be empty.  This corresponds to FIPS 204
 * Algorithm 3 "ML-DSA.Verify" with the ctx parameter set to the empty string
 * and the lengths of the signature and key given explicitly by the caller.
 *
 * Context: Might sleep
 *
 * Return:
 * * 0 if the signature is valid
 * * -EBADMSG if the signature and/or public key is malformed
 * * -EKEYREJECTED if the signature is invalid but otherwise well-formed
 * * -ENOMEM if out of memory so the validity of the signature is unknown
 */
unsafe extern "C" {
    pub fn mldsa_verify(
        alg: mldsa_alg,
        sig: *const u8,
        sig_len: usize,
        msg: *const u8,
        msg_len: usize,
        pk: *const u8,
        pk_len: usize,
    ) -> i32;
}

/* Internal function, exposed only for unit testing.
 * The C declaration is conditional on CONFIG_CRYPTO_LIB_MLDSA_KUNIT_TEST;
 * preserve that build-time condition here for consumers to select. */
#[cfg(feature = "CONFIG_CRYPTO_LIB_MLDSA_KUNIT_TEST")]
unsafe extern "C" {
    pub fn mldsa_use_hint(h: u8, r: i32, gamma2: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
