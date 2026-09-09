// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2025 Google LLC
 */

// Dependencies supplied by the kernel crypto, test-vector, and KUnit support.
// The C source includes crypto/sha2.h, sha256-testvecs.h, test-utils.h, and
// expands hash-test-template.h with the following configuration:
// HASH = sha256, HASH_CTX = sha256_ctx, HASH_SIZE = SHA256_DIGEST_SIZE,
// HASH_INIT = sha256_init, HASH_UPDATE = sha256_update,
// HASH_FINAL = sha256_final, HMAC_KEY = hmac_sha256_key,
// HMAC_CTX = hmac_sha256_ctx, HMAC_PREPAREKEY = hmac_sha256_preparekey,
// HMAC_INIT = hmac_sha256_init, HMAC_UPDATE = hmac_sha256_update,
// HMAC_FINAL = hmac_sha256_final, HMAC = hmac_sha256,
// HMAC_USINGRAWKEY = hmac_sha256_usingrawkey.

extern "C" {
    fn alloc_guarded_buf(test: *mut kunit, size: usize) -> *mut u8;
    fn alloc_buf(test: *mut kunit, size: usize) -> *mut u8;
    fn rand_bytes(buf: *mut u8, len: usize);
    fn rand_length(max: usize) -> usize;
    fn memset(dest: *mut core::ffi::c_void, value: i32, count: usize) -> *mut core::ffi::c_void;
    fn sha256_init(ctx: *mut sha256_ctx);
    fn sha256_update(ctx: *mut sha256_ctx, data: *const u8, len: usize);
    fn sha256_final(ctx: *mut sha256_ctx, hash: *mut u8);
    fn sha256_finup_2x(
        ctx: *mut sha256_ctx,
        data1: *const u8,
        data2: *const u8,
        len: usize,
        hash1: *mut u8,
        hash2: *mut u8,
    );
    fn sha256_finup_2x_is_optimized() -> bool;
    fn kunit_skip(test: *mut kunit, msg: *const core::ffi::c_char);
    fn preempt_disable();
    fn preempt_enable();
    fn ktime_get_ns() -> u64;
    fn kunit_info(test: *mut kunit, fmt: *const core::ffi::c_char, ...);
}

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sha256_ctx {
    pub ctx: sha256_state,
}

#[repr(C)]
pub struct sha256_state {
    pub bytecount: u64,
    _private: [u8; 0],
}

const SHA256_DIGEST_SIZE: usize = 32;
const SHA256_BLOCK_SIZE: usize = 64;

// Test for sha256_finup_2x(). Specifically, choose various data lengths and
// salt lengths, and verify the result against sha256_update() and sha256_final().
unsafe fn test_sha256_finup_2x(test: *mut kunit) {
    let max_data_len: usize = 16384;
    let data1_buf = alloc_guarded_buf(test, max_data_len);
    let data2_buf = alloc_guarded_buf(test, max_data_len);
    let hash1 = alloc_guarded_buf(test, SHA256_DIGEST_SIZE);
    let hash2 = alloc_guarded_buf(test, SHA256_DIGEST_SIZE);
    let mut expected_hash1 = [0u8; SHA256_DIGEST_SIZE];
    let mut expected_hash2 = [0u8; SHA256_DIGEST_SIZE];
    let mut salt = [0u8; SHA256_BLOCK_SIZE];
    let ctx = alloc_guarded_buf(test, core::mem::size_of::<sha256_ctx>()) as *mut sha256_ctx;

    rand_bytes(data1_buf, max_data_len);
    rand_bytes(data2_buf, max_data_len);
    rand_bytes(salt.as_mut_ptr(), salt.len());
    memset(ctx.cast(), 0, core::mem::size_of::<sha256_ctx>());

    for _i in 0..500 {
        let salt_len = rand_length(salt.len());
        let data_len = rand_length(max_data_len);
        let data1 = data1_buf.add(max_data_len - data_len);
        let data2 = data2_buf.add(max_data_len - data_len);
        let mut orig_ctx: sha256_ctx = core::mem::zeroed();

        sha256_init(ctx);
        sha256_update(ctx, salt.as_ptr(), salt_len);
        orig_ctx = *ctx;

        sha256_finup_2x(ctx, data1, data2, data_len, hash1, hash2);
        // KUNIT_ASSERT_MEMEQ_MSG(test, ctx, &orig_ctx, sizeof(*ctx), ...)
        kunit_assert_memeq_msg(test, ctx.cast(), (&orig_ctx as *const sha256_ctx).cast(), core::mem::size_of::<sha256_ctx>(), b"sha256_finup_2x() modified its ctx argument\0".as_ptr().cast());

        sha256_update(ctx, data1, data_len);
        sha256_final(ctx, expected_hash1.as_mut_ptr());
        sha256_update(&mut orig_ctx, data2, data_len);
        sha256_final(&mut orig_ctx, expected_hash2.as_mut_ptr());
        kunit_assert_memeq_msg(test, hash1.cast(), expected_hash1.as_ptr().cast(), SHA256_DIGEST_SIZE, b"Wrong hash1\0".as_ptr().cast());
        kunit_assert_memeq_msg(test, hash2.cast(), expected_hash2.as_ptr().cast(), SHA256_DIGEST_SIZE, b"Wrong hash2\0".as_ptr().cast());
    }
}

unsafe fn test_sha256_finup_2x_defaultctx(test: *mut kunit) {
    let data_len: usize = 128;
    let data = alloc_buf(test, 2 * data_len);
    let mut ctx: sha256_ctx = core::mem::zeroed();
    let mut hash1_a = [0u8; SHA256_DIGEST_SIZE];
    let mut hash2_a = [0u8; SHA256_DIGEST_SIZE];
    let mut hash1_b = [0u8; SHA256_DIGEST_SIZE];
    let mut hash2_b = [0u8; SHA256_DIGEST_SIZE];

    rand_bytes(data, 2 * data_len);
    sha256_init(&mut ctx);
    sha256_finup_2x(&mut ctx, data, data.add(data_len), data_len, hash1_a.as_mut_ptr(), hash2_a.as_mut_ptr());
    sha256_finup_2x(core::ptr::null_mut(), data, data.add(data_len), data_len, hash1_b.as_mut_ptr(), hash2_b.as_mut_ptr());
    kunit_assert_memeq(test, hash1_a.as_ptr().cast(), hash1_b.as_ptr().cast(), SHA256_DIGEST_SIZE);
    kunit_assert_memeq(test, hash2_a.as_ptr().cast(), hash2_b.as_ptr().cast(), SHA256_DIGEST_SIZE);
}

unsafe fn test_sha256_finup_2x_hugelen(test: *mut kunit) {
    let data_len = 4 * SHA256_BLOCK_SIZE;
    let data = alloc_buf(test, data_len);
    let mut ctx: sha256_ctx = core::mem::zeroed();
    let mut expected_hash = [0u8; SHA256_DIGEST_SIZE];
    let mut hash = [0u8; SHA256_DIGEST_SIZE];
    rand_bytes(data, data_len);
    for align in 0..SHA256_BLOCK_SIZE {
        sha256_init(&mut ctx);
        ctx.ctx.bytecount = 0x123456789abcd00u64.wrapping_add(align as u64);
        sha256_finup_2x(&mut ctx, data, data, data_len, hash.as_mut_ptr(), hash.as_mut_ptr());
        sha256_update(&mut ctx, data, data_len);
        sha256_final(&mut ctx, expected_hash.as_mut_ptr());
        kunit_assert_memeq(test, hash.as_ptr().cast(), expected_hash.as_ptr().cast(), SHA256_DIGEST_SIZE);
    }
}

unsafe fn benchmark_sha256_finup_2x(test: *mut kunit) {
    let salt_lens_to_test: [usize; 3] = [0, 32, 64];
    let data_len: usize = 4096;
    let num_iters: usize = 4096;
    let data = alloc_buf(test, data_len * 2);
    let mut ctx: sha256_ctx = core::mem::zeroed();
    let mut hash1 = [0u8; SHA256_DIGEST_SIZE];
    let mut hash2 = [0u8; SHA256_DIGEST_SIZE];
    if !config_crypto_lib_benchmark() { kunit_skip(test, b"not enabled\0".as_ptr().cast()); }
    if !sha256_finup_2x_is_optimized() { kunit_skip(test, b"not relevant\0".as_ptr().cast()); }
    rand_bytes(data, data_len * 2);
    for _ in 0..num_iters { sha256_finup_2x(core::ptr::null_mut(), data, data.add(data_len), data_len, hash1.as_mut_ptr(), hash2.as_mut_ptr()); }
    for &salt_len in &salt_lens_to_test {
        sha256_init(&mut ctx);
        sha256_update(&mut ctx, data, salt_len);
        preempt_disable();
        let t0 = ktime_get_ns();
        for _ in 0..num_iters { sha256_finup_2x(&mut ctx, data, data.add(data_len), data_len, hash1.as_mut_ptr(), hash2.as_mut_ptr()); }
        let t1 = ktime_get_ns();
        preempt_enable();
        let elapsed = if t1 == t0 { 1 } else { t1 - t0 };
        kunit_info(test, b"data_len=%zu salt_len=%zu: %llu MB/s\0".as_ptr().cast(), data_len, salt_len, ((data_len as u64 * 2 * num_iters as u64 * 1000) / elapsed));
    }
}

// Template-generated HASH_KUNIT_CASES and KUnit suite registration.
// static struct kunit_case hash_test_cases[] = { HASH_KUNIT_CASES, ... };
// static struct kunit_suite hash_test_suite = { .name = "sha256", .test_cases = hash_test_cases };
// kunit_test_suite(hash_test_suite);
// MODULE_DESCRIPTION("KUnit tests and benchmark for SHA-256 and HMAC-SHA256");
// MODULE_LICENSE("GPL");

extern "C" {
    fn kunit_assert_memeq(test: *mut kunit, left: *const core::ffi::c_void, right: *const core::ffi::c_void, len: usize);
    fn kunit_assert_memeq_msg(test: *mut kunit, left: *const core::ffi::c_void, right: *const core::ffi::c_void, len: usize, msg: *const core::ffi::c_char, ...);
    fn config_crypto_lib_benchmark() -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
