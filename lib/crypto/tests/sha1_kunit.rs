// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2025 Google LLC
 */

// Dependency supplied by <crypto/sha1.h>.
// Test vectors and the hash-test-template.h expansion are supplied externally.

// C preprocessor configuration for hash-test-template.h:
//
// #define HASH sha1
// #define HASH_CTX sha1_ctx
// #define HASH_SIZE SHA1_DIGEST_SIZE
// #define HASH_INIT sha1_init
// #define HASH_UPDATE sha1_update
// #define HASH_FINAL sha1_final
// #define HMAC_KEY hmac_sha1_key
// #define HMAC_CTX hmac_sha1_ctx
// #define HMAC_PREPAREKEY hmac_sha1_preparekey
// #define HMAC_INIT hmac_sha1_init
// #define HMAC_UPDATE hmac_sha1_update
// #define HMAC_FINAL hmac_sha1_final
// #define HMAC hmac_sha1
// #define HMAC_USINGRAWKEY hmac_sha1_usingrawkey

// The following aliases correspond to the template configuration above.
type HASH = sha1;
type HASH_CTX = sha1_ctx;
const HASH_SIZE: usize = SHA1_DIGEST_SIZE;
const HASH_INIT: unsafe extern "C" fn() = sha1_init;
const HASH_UPDATE: unsafe extern "C" fn() = sha1_update;
const HASH_FINAL: unsafe extern "C" fn() = sha1_final;
type HMAC_KEY = hmac_sha1_key;
type HMAC_CTX = hmac_sha1_ctx;
const HMAC_PREPAREKEY: unsafe extern "C" fn() = hmac_sha1_preparekey;
const HMAC_INIT: unsafe extern "C" fn() = hmac_sha1_init;
const HMAC_UPDATE: unsafe extern "C" fn() = hmac_sha1_update;
const HMAC_FINAL: unsafe extern "C" fn() = hmac_sha1_final;
const HMAC: unsafe extern "C" fn() = hmac_sha1;
const HMAC_USINGRAWKEY: unsafe extern "C" fn() = hmac_sha1_usingrawkey;

// Types, functions, globals, and the HASH_KUNIT_CASES expansion are provided
// by the kernel KUnit and hash-test-template dependencies.
extern "C" {
    type sha1;
    type sha1_ctx;
    type hmac_sha1_key;
    type hmac_sha1_ctx;

    static SHA1_DIGEST_SIZE: usize;

    static sha1_init: unsafe extern "C" fn();
    static sha1_update: unsafe extern "C" fn();
    static sha1_final: unsafe extern "C" fn();
    static hmac_sha1_preparekey: unsafe extern "C" fn();
    static hmac_sha1_init: unsafe extern "C" fn();
    static hmac_sha1_update: unsafe extern "C" fn();
    static hmac_sha1_final: unsafe extern "C" fn();
    static hmac_sha1: unsafe extern "C" fn();
    static hmac_sha1_usingrawkey: unsafe extern "C" fn();

    fn benchmark_hash();
}

// `HASH_KUNIT_CASES` is the declaration sequence emitted by
// hash-test-template.h. Its exact Rust representation is intentionally left
// to that external template dependency.
static mut hash_test_cases: () = ();

// static struct kunit_suite hash_test_suite = {
//     .name = "sha1",
//     .test_cases = hash_test_cases,
// };
// kunit_test_suite(hash_test_suite);

// Module metadata:
// MODULE_DESCRIPTION("KUnit tests and benchmark for SHA-1 and HMAC-SHA1");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
