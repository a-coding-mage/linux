// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2025 Google LLC
 */

// Dependencies supplied by the surrounding kernel/KUnit translation.
use core::ffi::c_void;

/*
 * The following are compatibility functions that present BLAKE2s as an unkeyed
 * hash function that produces hashes of fixed length BLAKE2S_HASH_SIZE, so that
 * hash-test-template.h can be reused to test it.
 */

unsafe fn blake2s_default(data: *const u8, len: usize, out: *mut u8) {
    blake2s(core::ptr::null(), 0, data, len, out, BLAKE2S_HASH_SIZE);
}

unsafe fn blake2s_init_default(ctx: *mut blake2s_ctx) {
    blake2s_init(ctx, BLAKE2S_HASH_SIZE);
}

/*
 * The C preprocessor instantiates hash-test-template.h here with these
 * bindings.  The resulting HASH_KUNIT_CASES are represented by the external
 * declaration below.
 */

/*
 * BLAKE2s specific test case which tests all possible combinations of key
 * length and hash length.
 */
unsafe fn test_blake2s_all_key_and_hash_lens(test: *mut kunit) {
    let data_len: usize = 100;
    let data: *mut u8 = alloc_buf(test, data_len);
    let key: *mut u8 = alloc_buf(test, BLAKE2S_KEY_SIZE);
    let hash: *mut u8 = alloc_buf(test, BLAKE2S_HASH_SIZE);
    let mut main_ctx: blake2s_ctx = core::mem::zeroed();
    let mut main_hash = [0u8; BLAKE2S_HASH_SIZE];

    rand_bytes_seeded_from_len(data, data_len);
    blake2s_init(&mut main_ctx, BLAKE2S_HASH_SIZE);
    for key_len in 0..=BLAKE2S_KEY_SIZE {
        rand_bytes_seeded_from_len(key, key_len);
        for out_len in 1..=BLAKE2S_HASH_SIZE {
            blake2s(key, key_len, data, data_len, hash, out_len);
            blake2s_update(&mut main_ctx, hash, out_len);
        }
    }
    blake2s_final(&mut main_ctx, main_hash.as_mut_ptr());
    KUNIT_ASSERT_MEMEQ(
        test,
        main_hash.as_ptr(),
        blake2s_keyed_testvec_consolidated,
        BLAKE2S_HASH_SIZE,
    );
}

/*
 * BLAKE2s specific test case which tests using a guarded buffer for all allowed
 * key lengths.  Also tests both blake2s() and blake2s_init_key().
 */
unsafe fn test_blake2s_with_guarded_key_buf(test: *mut kunit) {
    let data_len: usize = 100;
    let data: *mut u8 = alloc_buf(test, data_len);
    let guarded_key_buf: *mut u8 = alloc_guarded_buf(test, BLAKE2S_KEY_SIZE);

    rand_bytes(data, data_len);
    for key_len in 0..=BLAKE2S_KEY_SIZE {
        let mut key = [0u8; BLAKE2S_KEY_SIZE];
        let guarded_key = guarded_key_buf.add(BLAKE2S_KEY_SIZE - key_len);
        let mut hash1 = [0u8; BLAKE2S_HASH_SIZE];
        let mut hash2 = [0u8; BLAKE2S_HASH_SIZE];
        let mut ctx: blake2s_ctx = core::mem::zeroed();

        rand_bytes(key.as_mut_ptr(), key_len);
        core::ptr::copy_nonoverlapping(key.as_ptr(), guarded_key, key_len);

        blake2s(key.as_ptr(), key_len, data, data_len, hash1.as_mut_ptr(), BLAKE2S_HASH_SIZE);
        blake2s(guarded_key, key_len, data, data_len, hash2.as_mut_ptr(), BLAKE2S_HASH_SIZE);
        KUNIT_ASSERT_MEMEQ(test, hash1.as_ptr(), hash2.as_ptr(), BLAKE2S_HASH_SIZE);

        blake2s_init_key(&mut ctx, BLAKE2S_HASH_SIZE, guarded_key, key_len);
        blake2s_update(&mut ctx, data, data_len);
        blake2s_final(&mut ctx, hash2.as_mut_ptr());
        KUNIT_ASSERT_MEMEQ(test, hash1.as_ptr(), hash2.as_ptr(), BLAKE2S_HASH_SIZE);
    }
}

/*
 * BLAKE2s specific test case which tests using a guarded output buffer for all
 * allowed output lengths.
 */
unsafe fn test_blake2s_with_guarded_out_buf(test: *mut kunit) {
    let data_len: usize = 100;
    let data: *mut u8 = alloc_buf(test, data_len);
    let out_buf: *mut u8 = alloc_guarded_buf(test, BLAKE2S_HASH_SIZE);

    rand_bytes(data, data_len);
    for out_len in 1..=BLAKE2S_HASH_SIZE {
        let mut hash = [0u8; BLAKE2S_HASH_SIZE];
        let guarded_hash = out_buf.add(BLAKE2S_HASH_SIZE - out_len);

        blake2s(core::ptr::null(), 0, data, data_len, hash.as_mut_ptr(), out_len);
        blake2s(core::ptr::null(), 0, data, data_len, guarded_hash, out_len);
        KUNIT_ASSERT_MEMEQ(test, hash.as_ptr(), guarded_hash, out_len);
    }
}

static mut blake2s_test_cases: [kunit_case; 6] = [
    HASH_KUNIT_CASES,
    KUNIT_CASE!(test_blake2s_all_key_and_hash_lens),
    KUNIT_CASE!(test_blake2s_with_guarded_key_buf),
    KUNIT_CASE!(test_blake2s_with_guarded_out_buf),
    KUNIT_CASE!(benchmark_hash),
    kunit_case {},
];

static mut blake2s_test_suite: kunit_suite = kunit_suite {
    name: "blake2s\0".as_ptr() as *const i8,
    test_cases: unsafe { blake2s_test_cases.as_mut_ptr() },
};

// kunit_test_suite(blake2s_test_suite);

// MODULE_DESCRIPTION("KUnit tests and benchmark for BLAKE2s");
// MODULE_LICENSE("GPL");

extern "C" {
    static blake2s_keyed_testvec_consolidated: *const u8;
    fn blake2s(key: *const u8, key_len: usize, data: *const u8, len: usize, out: *mut u8, out_len: usize);
    fn blake2s_init(ctx: *mut blake2s_ctx, out_len: usize);
    fn blake2s_init_key(ctx: *mut blake2s_ctx, out_len: usize, key: *const u8, key_len: usize);
    fn blake2s_update(ctx: *mut blake2s_ctx, data: *const u8, len: usize);
    fn blake2s_final(ctx: *mut blake2s_ctx, out: *mut u8);
    fn alloc_buf(test: *mut kunit, len: usize) -> *mut u8;
    fn alloc_guarded_buf(test: *mut kunit, len: usize) -> *mut u8;
    fn rand_bytes(data: *mut u8, len: usize);
    fn rand_bytes_seeded_from_len(data: *mut u8, len: usize);
    fn KUNIT_ASSERT_MEMEQ(test: *mut kunit, actual: *const u8, expected: *const u8, len: usize);
}

#[repr(C)]
pub struct blake2s_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kunit_case {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kunit_suite {
    name: *const i8,
    test_cases: *mut kunit_case,
}

const BLAKE2S_KEY_SIZE: usize = 32;
const BLAKE2S_HASH_SIZE: usize = 32;

// The generated template cases and benchmark are supplied by hash-test-template.h.
extern "C" {
    static HASH_KUNIT_CASES: kunit_case;
    fn benchmark_hash(test: *mut kunit);
}

macro_rules! KUNIT_CASE {
    ($func:ident) => {
        kunit_case {}
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
