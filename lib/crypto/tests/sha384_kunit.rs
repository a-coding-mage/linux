// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2025 Google LLC
 */

// Dependencies supplied by the kernel crypto and KUnit test infrastructure:
// #include <crypto/sha2.h>
// #include "sha384-testvecs.h"

// The following aliases instantiate hash-test-template.h for SHA-384.
type HASH = sha384;
type HASH_CTX = sha384_ctx;
const HASH_SIZE: usize = SHA384_DIGEST_SIZE;
const HASH_INIT: unsafe extern "C" fn(*mut HASH_CTX) = sha384_init;
const HASH_UPDATE: unsafe extern "C" fn(*mut HASH_CTX, *const u8, usize) = sha384_update;
const HASH_FINAL: unsafe extern "C" fn(*mut HASH_CTX, *mut u8) = sha384_final;
type HMAC_KEY = hmac_sha384_key;
type HMAC_CTX = hmac_sha384_ctx;
const HMAC_PREPAREKEY: unsafe extern "C" fn(*mut HMAC_KEY, *const u8, usize) =
    hmac_sha384_preparekey;
const HMAC_INIT: unsafe extern "C" fn(*mut HMAC_CTX, *const HMAC_KEY) = hmac_sha384_init;
const HMAC_UPDATE: unsafe extern "C" fn(*mut HMAC_CTX, *const u8, usize) = hmac_sha384_update;
const HMAC_FINAL: unsafe extern "C" fn(*mut HMAC_CTX, *mut u8) = hmac_sha384_final;
const HMAC: unsafe extern "C" fn(*const u8, usize, *const u8, usize, *mut u8) = hmac_sha384;
const HMAC_USINGRAWKEY: unsafe extern "C" fn(*const u8, usize, *const u8, usize, *mut u8) =
    hmac_sha384_usingrawkey;

// This file is the SHA-384 instantiation of hash-test-template.h.
// The template-provided declarations and test implementations are external
// dependencies and are intentionally not reproduced here.

static mut hash_test_cases: [kunit_case; 3] = [
    HASH_KUNIT_CASES,
    KUNIT_CASE!(benchmark_hash),
    kunit_case {},
];

static mut hash_test_suite: kunit_suite = kunit_suite {
    name: "sha384",
    test_cases: hash_test_cases.as_ptr(),
};

// Equivalent of kunit_test_suite(hash_test_suite).
kunit_test_suite!(hash_test_suite);

module_description!("KUnit tests and benchmark for SHA-384 and HMAC-SHA384");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
