// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2025 Google LLC
 */

// C dependencies:
// #include <crypto/sha2.h>
// #include "sha224-testvecs.h"
// #include "hash-test-template.h"

// The following aliases correspond directly to the C preprocessor aliases.
// Their definitions are supplied by the included crypto and test-template
// dependencies.
type HASH = sha224;
type HASH_CTX = sha224_ctx;
const HASH_SIZE: usize = SHA224_DIGEST_SIZE;
const HASH_INIT: unsafe extern "C" fn() = sha224_init;
const HASH_UPDATE: unsafe extern "C" fn() = sha224_update;
const HASH_FINAL: unsafe extern "C" fn() = sha224_final;
type HMAC_KEY = hmac_sha224_key;
type HMAC_CTX = hmac_sha224_ctx;
const HMAC_PREPAREKEY: unsafe extern "C" fn() = hmac_sha224_preparekey;
const HMAC_INIT: unsafe extern "C" fn() = hmac_sha224_init;
const HMAC_UPDATE: unsafe extern "C" fn() = hmac_sha224_update;
const HMAC_FINAL: unsafe extern "C" fn() = hmac_sha224_final;
const HMAC: unsafe extern "C" fn() = hmac_sha224;
const HMAC_USINGRAWKEY: unsafe extern "C" fn() = hmac_sha224_usingrawkey;

// HASH_KUNIT_CASES, KUNIT_CASE, and the empty case initializer are supplied
// by hash-test-template.h.  The declarations below retain the C objects and
// their externally supplied types and initializers.
static mut hash_test_cases: [kunit_case; 3] = [
    HASH_KUNIT_CASES,
    KUNIT_CASE!(benchmark_hash),
    kunit_case {},
];

static mut hash_test_suite: kunit_suite = kunit_suite {
    name: "sha224",
    test_cases: unsafe { &mut hash_test_cases },
};

// Equivalent of: kunit_test_suite(hash_test_suite);
kunit_test_suite!(hash_test_suite);

// Equivalent module metadata.
#[allow(dead_code)]
const MODULE_DESCRIPTION: &str = "KUnit tests and benchmark for SHA-224 and HMAC-SHA224";
#[allow(dead_code)]
const MODULE_LICENSE: &str = "GPL";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
