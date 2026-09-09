// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2025 Google LLC
 */

// C dependencies: <crypto/md5.h> and "md5-testvecs.h".
// The hash-test-template.h include supplies the hash tests and benchmark.

// C preprocessor aliases used by hash-test-template.h.
type HASH = md5;
type HASH_CTX = md5_ctx;
const HASH_SIZE: usize = MD5_DIGEST_SIZE;
const HASH_INIT: fn() = md5_init;
const HASH_UPDATE: fn() = md5_update;
const HASH_FINAL: fn() = md5_final;
type HMAC_KEY = hmac_md5_key;
type HMAC_CTX = hmac_md5_ctx;
const HMAC_PREPAREKEY: fn() = hmac_md5_preparekey;
const HMAC_INIT: fn() = hmac_md5_init;
const HMAC_UPDATE: fn() = hmac_md5_update;
const HMAC_FINAL: fn() = hmac_md5_final;
const HMAC: fn() = hmac_md5;
const HMAC_USINGRAWKEY: fn() = hmac_md5_usingrawkey;

// HASH_KUNIT_CASES and KUNIT_CASE are provided by hash-test-template.h and
// the KUnit headers, respectively.
static mut hash_test_cases: [kunit_case; 3] = [
    HASH_KUNIT_CASES,
    KUNIT_CASE!(benchmark_hash),
    kunit_case {
        ..unsafe { core::mem::zeroed() }
    },
];

static mut hash_test_suite: kunit_suite = kunit_suite {
    name: "md5",
    test_cases: hash_test_cases.as_mut_ptr(),
};

// Corresponds to kunit_test_suite(hash_test_suite).
kunit_test_suite!(hash_test_suite);

// MODULE_DESCRIPTION("KUnit tests and benchmark for MD5 and HMAC-MD5");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
