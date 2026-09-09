/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Cryptographic API.
 *
 * Copyright (c) 2023 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Dependency equivalent of <crypto/internal/hash.h>.
// Dependency equivalent of "internal.h".

use core::ffi::c_int;

// Opaque types supplied by the dependent headers.
pub enum crypto_type {}
pub enum hash_alg_common {}

extern "C" {
    pub static crypto_shash_type: crypto_type;

    pub fn hash_prepare_alg(alg: *mut hash_alg_common) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
