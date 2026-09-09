/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * CTR: Counter mode
 *
 * Copyright (c) 2007 Herbert Xu <herbert@gondor.apana.org.au>
 */

// C header guard: _CRYPTO_CTR_H

pub const CTR_RFC3686_NONCE_SIZE: u32 = 4;
pub const CTR_RFC3686_IV_SIZE: u32 = 8;
pub const CTR_RFC3686_BLOCK_SIZE: u32 = 16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
