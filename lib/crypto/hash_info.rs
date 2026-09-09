// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Hash Info: Hash algorithms information
 *
 * Copyright (c) 2013 Dmitry Kasatkin <d.kasatkin@samsung.com>
 */

// The following names and constants are supplied by the corresponding Rust
// translation of <crypto/hash_info.h> and the hash implementations.

pub static hash_algo_name: [*const core::ffi::c_char; HASH_ALGO__LAST] = [
    b"md4\0".as_ptr() as *const core::ffi::c_char,
    b"md5\0".as_ptr() as *const core::ffi::c_char,
    b"sha1\0".as_ptr() as *const core::ffi::c_char,
    b"rmd160\0".as_ptr() as *const core::ffi::c_char,
    b"sha256\0".as_ptr() as *const core::ffi::c_char,
    b"sha384\0".as_ptr() as *const core::ffi::c_char,
    b"sha512\0".as_ptr() as *const core::ffi::c_char,
    b"sha224\0".as_ptr() as *const core::ffi::c_char,
    b"rmd128\0".as_ptr() as *const core::ffi::c_char,
    b"rmd256\0".as_ptr() as *const core::ffi::c_char,
    b"rmd320\0".as_ptr() as *const core::ffi::c_char,
    b"wp256\0".as_ptr() as *const core::ffi::c_char,
    b"wp384\0".as_ptr() as *const core::ffi::c_char,
    b"wp512\0".as_ptr() as *const core::ffi::c_char,
    b"tgr128\0".as_ptr() as *const core::ffi::c_char,
    b"tgr160\0".as_ptr() as *const core::ffi::c_char,
    b"tgr192\0".as_ptr() as *const core::ffi::c_char,
    b"sm3\0".as_ptr() as *const core::ffi::c_char,
    b"streebog256\0".as_ptr() as *const core::ffi::c_char,
    b"streebog512\0".as_ptr() as *const core::ffi::c_char,
    b"sha3-256\0".as_ptr() as *const core::ffi::c_char,
    b"sha3-384\0".as_ptr() as *const core::ffi::c_char,
    b"sha3-512\0".as_ptr() as *const core::ffi::c_char,
];
// EXPORT_SYMBOL_GPL(hash_algo_name);

pub static hash_digest_size: [core::ffi::c_int; HASH_ALGO__LAST] = [
    MD5_DIGEST_SIZE,
    MD5_DIGEST_SIZE,
    SHA1_DIGEST_SIZE,
    RMD160_DIGEST_SIZE,
    SHA256_DIGEST_SIZE,
    SHA384_DIGEST_SIZE,
    SHA512_DIGEST_SIZE,
    SHA224_DIGEST_SIZE,
    RMD128_DIGEST_SIZE,
    RMD256_DIGEST_SIZE,
    RMD320_DIGEST_SIZE,
    WP256_DIGEST_SIZE,
    WP384_DIGEST_SIZE,
    WP512_DIGEST_SIZE,
    TGR128_DIGEST_SIZE,
    TGR160_DIGEST_SIZE,
    TGR192_DIGEST_SIZE,
    SM3256_DIGEST_SIZE,
    STREEBOG256_DIGEST_SIZE,
    STREEBOG512_DIGEST_SIZE,
    SHA3_256_DIGEST_SIZE,
    SHA3_384_DIGEST_SIZE,
    SHA3_512_DIGEST_SIZE,
];
// EXPORT_SYMBOL_GPL(hash_digest_size);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
