// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2025 Google LLC
 */

// Dependencies supplied by the kernel crypto and KUnit test infrastructure.
// The C hash-test-template.h expansion is represented by the corresponding
// external test-case symbols.

unsafe fn blake2b_default(data: *const u8, len: usize, out: *mut u8) {
    blake2b(core::ptr::null(), 0, data, len, out, BLAKE2B_HASH_SIZE);
}

unsafe fn blake2b_init_default(ctx: *mut blake2b_ctx) {
    blake2b_init(ctx, BLAKE2B_HASH_SIZE);
}

/*
 * Generate the HASH_KUNIT_CASES using hash-test-template.h.  These test BLAKE2b
 * with a key length of 0 and a hash length of BLAKE2B_HASH_SIZE.
 */
// HASH = blake2b_default
// HASH_CTX = blake2b_ctx
// HASH_SIZE = BLAKE2B_HASH_SIZE
// HASH_INIT = blake2b_init_default
// HASH_UPDATE = blake2b_update
// HASH_FINAL = blake2b_final
// HASH_KUNIT_CASES is supplied by the hash-test-template.h expansion.

/*
 * BLAKE2b specific test case which tests all possible combinations of key
 * length and hash length.
 */
unsafe fn test_blake2b_all_key_and_hash_lens(test: *mut kunit) {
    let data_len: usize = 100;
    let data: *mut u8 = alloc_buf(test, data_len);
    let key: *mut u8 = alloc_buf(test, BLAKE2B_KEY_SIZE);
    let hash: *mut u8 = alloc_buf(test, BLAKE2B_HASH_SIZE);
    let mut main_ctx: blake2b_ctx = core::mem::zeroed();
    let mut main_hash = [0u8; BLAKE2B_HASH_SIZE];

    rand_bytes_seeded_from_len(data, data_len);
    blake2b_init(&mut main_ctx, BLAKE2B_HASH_SIZE);
    for key_len in 0..=BLAKE2B_KEY_SIZE {
        rand_bytes_seeded_from_len(key, key_len);
        for out_len in 1..=BLAKE2B_HASH_SIZE {
            blake2b(key, key_len, data, data_len, hash, out_len);
            blake2b_update(&mut main_ctx, hash, out_len);
        }
    }
    blake2b_final(&mut main_ctx, main_hash.as_mut_ptr());
    KUNIT_ASSERT_MEMEQ(test, main_hash.as_ptr(), blake2b_keyed_testvec_consolidated, BLAKE2B_HASH_SIZE);
}

/*
 * BLAKE2b specific test case which tests using a guarded buffer for all allowed
 * key lengths.  Also tests both blake2b() and blake2b_init_key().
 */
unsafe fn test_blake2b_with_guarded_key_buf(test: *mut kunit) {
    let data_len: usize = 100;
    let data: *mut u8 = alloc_buf(test, data_len);
    let guarded_key_buf: *mut u8 = alloc_guarded_buf(test, BLAKE2B_KEY_SIZE);

    rand_bytes(data, data_len);
    for key_len in 0..=BLAKE2B_KEY_SIZE {
        let mut key = [0u8; BLAKE2B_KEY_SIZE];
        let guarded_key = guarded_key_buf.add(BLAKE2B_KEY_SIZE - key_len);
        let mut hash1 = [0u8; BLAKE2B_HASH_SIZE];
        let mut hash2 = [0u8; BLAKE2B_HASH_SIZE];
        let mut ctx: blake2b_ctx = core::mem::zeroed();

        rand_bytes(key.as_mut_ptr(), key_len);
        core::ptr::copy_nonoverlapping(key.as_ptr(), guarded_key, key_len);

        blake2b(key.as_ptr(), key_len, data, data_len, hash1.as_mut_ptr(), BLAKE2B_HASH_SIZE);
        blake2b(guarded_key, key_len, data, data_len, hash2.as_mut_ptr(), BLAKE2B_HASH_SIZE);
        KUNIT_ASSERT_MEMEQ(test, hash1.as_ptr(), hash2.as_ptr(), BLAKE2B_HASH_SIZE);

        blake2b_init_key(&mut ctx, BLAKE2B_HASH_SIZE, guarded_key, key_len);
        blake2b_update(&mut ctx, data, data_len);
        blake2b_final(&mut ctx, hash2.as_mut_ptr());
        KUNIT_ASSERT_MEMEQ(test, hash1.as_ptr(), hash2.as_ptr(), BLAKE2B_HASH_SIZE);
    }
}

/*
 * BLAKE2b specific test case which tests using a guarded output buffer for all
 * allowed output lengths.
 */
unsafe fn test_blake2b_with_guarded_out_buf(test: *mut kunit) {
    let data_len: usize = 100;
    let data: *mut u8 = alloc_buf(test, data_len);
    let out_buf: *mut u8 = alloc_guarded_buf(test, BLAKE2B_HASH_SIZE);

    rand_bytes(data, data_len);
    for out_len in 1..=BLAKE2B_HASH_SIZE {
        let mut hash = [0u8; BLAKE2B_HASH_SIZE];
        let guarded_hash = out_buf.add(BLAKE2B_HASH_SIZE - out_len);

        blake2b(core::ptr::null(), 0, data, data_len, hash.as_mut_ptr(), out_len);
        blake2b(core::ptr::null(), 0, data, data_len, guarded_hash, out_len);
        KUNIT_ASSERT_MEMEQ(test, hash.as_ptr(), guarded_hash, out_len);
    }
}

// The following KUnit case and suite declarations correspond to the C
// HASH_KUNIT_CASES/template expansion and KUNIT registration macros.
static mut blake2b_test_cases: [kunit_case; 5] = [
    KUNIT_CASES,
    KUNIT_CASE(test_blake2b_all_key_and_hash_lens),
    KUNIT_CASE(test_blake2b_with_guarded_key_buf),
    KUNIT_CASE(test_blake2b_with_guarded_out_buf),
    KUNIT_CASE(benchmark_hash),
];

static mut blake2b_test_suite: kunit_suite = kunit_suite {
    name: "blake2b\0".as_ptr() as *const i8,
    test_cases: blake2b_test_cases.as_mut_ptr(),
};

// kunit_test_suite(blake2b_test_suite);
// MODULE_DESCRIPTION("KUnit tests and benchmark for BLAKE2b");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
