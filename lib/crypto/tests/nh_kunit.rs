// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2025 Google LLC
 */
// Dependencies supplied by the kernel KUnit and NH test-vector headers are
// intentionally left as external symbols.

use core::ffi::c_void;

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
    pub name: *const u8,
    pub test_cases: *mut kunit_case,
}

extern "C" {
    static nh_test_key: [u8; NH_KEY_BYTES];
    static nh_test_msg: [u8; 1024];
    static nh_test_val16: [u8; NH_HASH_BYTES];
    static nh_test_val96: [u8; NH_HASH_BYTES];
    static nh_test_val256: [u8; NH_HASH_BYTES];
    static nh_test_val1024: [u8; NH_HASH_BYTES];

    fn memdup_buf(test: *mut kunit, src: *const c_void, len: usize) -> *mut u32;
    fn le32_to_cpu_array(array: *mut u32, words: usize);
    fn nh(key: *const u32, msg: *const u8, len: usize, hash: *mut u64);
    fn kunit_assert_memeq(
        test: *mut kunit,
        left: *const c_void,
        right: *const c_void,
        len: usize,
    );
}

// External build-time constants from <crypto/nh.h> and the test vectors.
extern "Rust" {
    static NH_KEY_BYTES: usize;
    static NH_KEY_WORDS: usize;
    static NH_NUM_PASSES: usize;
    static NH_HASH_BYTES: usize;
}

unsafe fn test_nh(test: *mut kunit) {
    let key = memdup_buf(test, nh_test_key.as_ptr() as *const c_void, NH_KEY_BYTES);
    let mut hash = [0u64; NH_NUM_PASSES];

    le32_to_cpu_array(key, NH_KEY_WORDS);

    nh(key, nh_test_msg.as_ptr(), 16, hash.as_mut_ptr());
    kunit_assert_memeq(
        test,
        hash.as_ptr() as *const c_void,
        nh_test_val16.as_ptr() as *const c_void,
        NH_HASH_BYTES,
    );

    nh(key, nh_test_msg.as_ptr(), 96, hash.as_mut_ptr());
    kunit_assert_memeq(
        test,
        hash.as_ptr() as *const c_void,
        nh_test_val96.as_ptr() as *const c_void,
        NH_HASH_BYTES,
    );

    nh(key, nh_test_msg.as_ptr(), 256, hash.as_mut_ptr());
    kunit_assert_memeq(
        test,
        hash.as_ptr() as *const c_void,
        nh_test_val256.as_ptr() as *const c_void,
        NH_HASH_BYTES,
    );

    nh(key, nh_test_msg.as_ptr(), 1024, hash.as_mut_ptr());
    kunit_assert_memeq(
        test,
        hash.as_ptr() as *const c_void,
        nh_test_val1024.as_ptr() as *const c_void,
        NH_HASH_BYTES,
    );
}

// KUNIT_CASE(test_nh), followed by the terminating empty case, corresponds
// to the kernel's test-case registration table.
static mut nh_test_cases: [*const (); 2] = [test_nh as *const (), core::ptr::null()];

static mut nh_test_suite: kunit_suite = kunit_suite {
    name: b"nh\0".as_ptr(),
    test_cases: nh_test_cases.as_mut_ptr() as *mut kunit_case,
};

// kunit_test_suite(nh_test_suite);
// MODULE_DESCRIPTION("KUnit tests for NH");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
