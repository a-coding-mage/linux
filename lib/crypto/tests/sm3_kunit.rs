// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2026 Google LLC
 */

// Dependency declarations supplied by the kernel crypto and KUnit headers:
// #include <crypto/sm3.h>
// #include "sm3-testvecs.h"

// The following aliases configure the included hash-test-template.h in the C
// source.  The template is an external dependency and is intentionally not
// reimplemented here.
// #define HASH sm3
// #define HASH_CTX sm3_ctx
// #define HASH_SIZE SM3_DIGEST_SIZE
// #define HASH_INIT sm3_init
// #define HASH_UPDATE sm3_update
// #define HASH_FINAL sm3_final
// #include "hash-test-template.h"

// Expansion of HASH_KUNIT_CASES from hash-test-template.h.
// Expansion of KUNIT_CASE(benchmark_hash) from the KUnit headers.
static mut sm3_test_cases: [kunit_case; 3] = [
    HASH_KUNIT_CASES,
    KUNIT_CASE!(benchmark_hash),
    kunit_case {
        ..unsafe { core::mem::zeroed() }
    },
];

static mut sm3_test_suite: kunit_suite = kunit_suite {
    name: "sm3",
    test_cases: sm3_test_cases.as_ptr(),
};

// kunit_test_suite(sm3_test_suite);

// MODULE_DESCRIPTION("KUnit tests and benchmark for SM3");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
