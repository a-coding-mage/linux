/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Hash Info: Hash algorithms information
 *
 * Copyright (c) 2013 Dmitry Kasatkin <d.kasatkin@samsung.com>
 */

// Dependencies supplied by the corresponding C headers:
// crypto/sha1.h, crypto/sha2.h, crypto/sha3.h, crypto/md5.h,
// crypto/streebog.h, and uapi/linux/hash_info.h.

use core::ffi::c_char;

/* not defined in include/crypto/ */
pub const RMD128_DIGEST_SIZE: usize = 16;
pub const RMD160_DIGEST_SIZE: usize = 20;
pub const RMD256_DIGEST_SIZE: usize = 32;
pub const RMD320_DIGEST_SIZE: usize = 40;

/* not defined in include/crypto/ */
pub const WP512_DIGEST_SIZE: usize = 64;
pub const WP384_DIGEST_SIZE: usize = 48;
pub const WP256_DIGEST_SIZE: usize = 32;

/* not defined in include/crypto/ */
pub const TGR128_DIGEST_SIZE: usize = 16;
pub const TGR160_DIGEST_SIZE: usize = 20;
pub const TGR192_DIGEST_SIZE: usize = 24;

/* not defined in include/crypto/ */
pub const SM3256_DIGEST_SIZE: usize = 32;

extern "C" {
    pub static hash_algo_name: [*const c_char; HASH_ALGO__LAST];
    pub static hash_digest_size: [core::ffi::c_int; HASH_ALGO__LAST];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
