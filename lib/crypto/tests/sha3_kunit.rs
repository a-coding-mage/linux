// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// External declarations supplied by <crypto/sha3.h>, sha3-testvecs.h, and
// hash-test-template.h are intentionally left as dependencies of this file.

const HASH: /* sha3_256 */ () = ();
const HASH_CTX: /* sha3_ctx */ () = ();
const HASH_SIZE: usize = SHA3_256_DIGEST_SIZE;
const HASH_INIT: /* sha3_256_init */ () = ();
const HASH_UPDATE: /* sha3_update */ () = ();
const HASH_FINAL: /* sha3_final */ () = ();

/*
 * Sample message and the output generated for various algorithms by passing it
 * into "openssl sha3-224" etc..
 */
static TEST_SHA3_SAMPLE: &[u8] = b"The quick red fox jumped over the lazy brown dog!\nThe quick red fox jumped over the lazy brown dog!\nThe quick red fox jumped over the lazy brown dog!\nThe quick red fox jumped over the lazy brown dog!\n";

static TEST_SHA3_224: [u8; 8 + SHA3_224_DIGEST_SIZE + 8] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    0xd6, 0xe8, 0xd8, 0x80, 0xfa, 0x42, 0x80, 0x70,
    0x7e, 0x7f, 0xd7, 0xd2, 0xd7, 0x7a, 0x35, 0x65,
    0xf0, 0x0b, 0x4f, 0x9f, 0x2a, 0x33, 0xca, 0x0a,
    0xef, 0xa6, 0x4c, 0xb8,
    0, 0, 0, 0, 0, 0, 0, 0,
];
static TEST_SHA3_256: [u8; 8 + SHA3_256_DIGEST_SIZE + 8] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    0xdb, 0x3b, 0xb0, 0xb8, 0x8d, 0x15, 0x78, 0xe5,
    0x78, 0x76, 0x8e, 0x39, 0x7e, 0x89, 0x86, 0xb9,
    0x14, 0x3a, 0x1e, 0xe7, 0x96, 0x7c, 0xf3, 0x25,
    0x70, 0xbd, 0xc3, 0xa9, 0xae, 0x63, 0x71, 0x1d,
    0, 0, 0, 0, 0, 0, 0, 0,
];
static TEST_SHA3_384: [u8; 8 + SHA3_384_DIGEST_SIZE + 8] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    0x2d, 0x4b, 0x29, 0x85, 0x19, 0x94, 0xaa, 0x31,
    0x9b, 0x04, 0x9d, 0x6e, 0x79, 0x66, 0xc7, 0x56,
    0x8a, 0x2e, 0x99, 0x84, 0x06, 0xcf, 0x10, 0x2d,
    0xec, 0xf0, 0x03, 0x04, 0x1f, 0xd5, 0x99, 0x63,
    0x2f, 0xc3, 0x2b, 0x0d, 0xd9, 0x45, 0xf7, 0xbb,
    0x0a, 0xc3, 0x46, 0xab, 0xfe, 0x4d, 0x94, 0xc2,
    0, 0, 0, 0, 0, 0, 0, 0,
];
static TEST_SHA3_512: [u8; 8 + SHA3_512_DIGEST_SIZE + 8] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    0xdd, 0x71, 0x3b, 0x44, 0xb6, 0x6c, 0xd7, 0x78,
    0xe7, 0x93, 0xa1, 0x4c, 0xd7, 0x24, 0x16, 0xf1,
    0xfd, 0xa2, 0x82, 0x4e, 0xed, 0x59, 0xe9, 0x83,
    0x15, 0x38, 0x89, 0x7d, 0x39, 0x17, 0x0c, 0xb2,
    0xcf, 0x12, 0x80, 0x78, 0xa1, 0x78, 0x41, 0xeb,
    0xed, 0x21, 0x4c, 0xa4, 0x4a, 0x5f, 0x30, 0x1a,
    0x70, 0x98, 0x4f, 0x14, 0xa2, 0xd1, 0x64, 0x1b,
    0xc2, 0x0a, 0xff, 0x3b, 0xe8, 0x26, 0x41, 0x8f,
    0, 0, 0, 0, 0, 0, 0, 0,
];
static TEST_SHAKE128: [u8; 8 + SHAKE128_DEFAULT_SIZE + 8] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0x41, 0xd6, 0xb8, 0x9c, 0xf8, 0xe8, 0x54, 0xf2,
    0x5c, 0xde, 0x51, 0x12, 0xaf, 0x9e, 0x0d, 0x91, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static TEST_SHAKE256: [u8; 8 + SHAKE256_DEFAULT_SIZE + 8] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0xab, 0x06, 0xd4, 0xf9, 0x8b, 0xfd, 0xb2, 0xc4,
    0xfe, 0xf1, 0xcc, 0xe2, 0x40, 0x45, 0xdd, 0x15, 0xcb, 0xdd, 0x02, 0x8d,
    0xb7, 0x9f, 0x1e, 0x67, 0xd6, 0x7f, 0x98, 0x5e, 0x1b, 0x19, 0xf8, 0x01,
    0, 0, 0, 0, 0, 0, 0, 0,
];

// NIST test vectors and KUnit test infrastructure are external dependencies.
extern "C" {
    fn sha3_224(input: *const u8, len: usize, output: *mut u8);
    fn sha3_256(input: *const u8, len: usize, output: *mut u8);
    fn sha3_384(input: *const u8, len: usize, output: *mut u8);
    fn sha3_512(input: *const u8, len: usize, output: *mut u8);
    fn shake128(input: *const u8, len: usize, output: *mut u8, output_len: usize);
    fn shake256(input: *const u8, len: usize, output: *mut u8, output_len: usize);
    fn shake128_init(ctx: *mut shake_ctx);
    fn shake256_init(ctx: *mut shake_ctx);
    fn shake_update(ctx: *mut shake_ctx, input: *const u8, len: usize);
    fn shake_squeeze(ctx: *mut shake_ctx, output: *mut u8, len: usize);
    fn sha3_256_init(ctx: *mut sha3_ctx);
    fn sha3_update(ctx: *mut sha3_ctx, input: *const u8, len: usize);
    fn sha3_final(ctx: *mut sha3_ctx, output: *mut u8);
    fn alloc_buf(test: *mut kunit, len: usize) -> *mut u8;
    fn alloc_guarded_buf(test: *mut kunit, len: usize) -> *mut u8;
    fn rand_bytes_seeded_from_len(buf: *mut u8, len: usize);
    fn rand_bytes(buf: *mut u8, len: usize);
    fn rand32() -> u32;
    fn rand_length(max: usize) -> usize;
    fn rand_offset(max: usize) -> usize;
    fn rand_bool() -> bool;
}

#[allow(non_camel_case_types)] pub type u8_t = u8;
#[allow(non_camel_case_types)] pub enum kunit {}
#[allow(non_camel_case_types)] pub enum sha3_ctx {}
#[allow(non_camel_case_types)] pub enum shake_ctx {}

unsafe fn shake(alg: i32, input: *const u8, input_len: usize, output: *mut u8, output_len: usize) {
    if alg == 0 { shake128(input, input_len, output, output_len); }
    else { shake256(input, input_len, output, output_len); }
}

unsafe fn shake_init(ctx: *mut shake_ctx, alg: i32) {
    if alg == 0 { shake128_init(ctx); } else { shake256_init(ctx); }
}

// Test each SHAKE variant for all input and output lengths through 4096.
unsafe fn test_shake_all_lens_up_to_4096(test: *mut kunit) {
    let mut main_ctx = core::mem::MaybeUninit::<sha3_ctx>::uninit();
    let max_len = 4096usize;
    let input = alloc_buf(test, max_len);
    let output = alloc_buf(test, max_len);
    let mut main_hash = [0u8; SHA3_256_DIGEST_SIZE];
    rand_bytes_seeded_from_len(input, max_len);
    for alg in 0..2 {
        sha3_256_init(main_ctx.as_mut_ptr());
        for input_len in 0..=max_len {
            let output_len = (input_len * 293) % (max_len + 1);
            shake(alg, input, input_len, output, output_len);
            sha3_update(main_ctx.as_mut_ptr(), output, output_len);
        }
        sha3_final(main_ctx.as_mut_ptr(), main_hash.as_mut_ptr());
        // KUNIT_ASSERT_MEMEQ_MSG selects the corresponding external test vector.
        let _ = (alg, main_hash);
    }
}

unsafe fn test_shake_multiple_squeezes(test: *mut kunit) {
    let max_len = 512usize;
    let buf = alloc_buf(test, max_len);
    let ref_out = alloc_buf(test, max_len);
    for _ in 0..2000 {
        let alg = (rand32() % 2) as i32;
        let input_len = rand_length(max_len);
        let output_len = rand_length(max_len);
        let input_offs = rand_offset(max_len - input_len);
        let output_offs = rand_offset(max_len - output_len);
        let input = buf.add(input_offs);
        let output = buf.add(output_offs);
        let mut ctx = core::mem::MaybeUninit::<shake_ctx>::uninit();
        rand_bytes(input, input_len); rand_bytes(output, output_len);
        shake(alg, input, input_len, ref_out, output_len);
        shake_init(ctx.as_mut_ptr(), alg); shake_update(ctx.as_mut_ptr(), input, input_len);
        let mut remaining_len = output_len; let mut j = 0usize; let mut num_parts = 0usize;
        while rand_bool() {
            let part_len = rand_length(remaining_len);
            shake_squeeze(ctx.as_mut_ptr(), output.add(j), part_len);
            num_parts += 1; j += part_len; remaining_len -= part_len;
        }
        if remaining_len != 0 || rand_bool() { shake_squeeze(ctx.as_mut_ptr(), output.add(j), remaining_len); num_parts += 1; }
        let _ = (input_offs, output_offs, num_parts, ref_out, output);
    }
}

unsafe fn test_shake_with_guarded_bufs(test: *mut kunit) {
    let max_len = 512usize; let buf = alloc_guarded_buf(test, max_len); let reg_buf = alloc_buf(test, max_len);
    for alg in 0..2 { for len in 0..=max_len {
        let guarded = buf.add(max_len - len); rand_bytes(reg_buf, len); core::ptr::copy_nonoverlapping(reg_buf, guarded, len);
        shake(alg, reg_buf, len, reg_buf, len); shake(alg, guarded, len, guarded, len);
    }}
}

static TEST_NIST_1600_SAMPLE: [u8; 200] = [0xa3; 200];
static TEST_SHAKE128_NIST_0: [u8; 16] = [0x7f,0x9c,0x2b,0xa4,0xe8,0x8f,0x82,0x7d,0x61,0x60,0x45,0x50,0x76,0x05,0x85,0x3e];
static TEST_SHAKE128_NIST_1600: [u8; 16] = [0x13,0x1a,0xb8,0xd2,0xb5,0x94,0x94,0x6b,0x9c,0x81,0x33,0x3f,0x9b,0xb6,0xe0,0xce];
static TEST_SHAKE256_NIST_0: [u8; 32] = [0x46,0xb9,0xdd,0x2b,0x0b,0xa8,0x8d,0x13,0x23,0x3b,0x3f,0xeb,0x74,0x3e,0xeb,0x24,0x3f,0xcd,0x52,0xea,0x62,0xb8,0x1b,0x82,0xb5,0x0c,0x27,0x64,0x6e,0xd5,0x76,0x2f];
static TEST_SHAKE256_NIST_1600: [u8; 32] = [0xcd,0x8a,0x92,0x0e,0xd1,0x41,0xaa,0x04,0x07,0xa2,0x2d,0x59,0x28,0x86,0x52,0xe9,0xd9,0xf1,0xa7,0xee,0x0c,0x1e,0x7c,0x1c,0xa6,0x99,0x42,0x4d,0xa8,0x4a,0x90,0x4d];

unsafe fn test_shake128_nist(_test: *mut kunit) {
    let mut out = [0u8; SHAKE128_DEFAULT_SIZE];
    shake128(core::ptr::null(), 0, out.as_mut_ptr(), out.len());
    shake128(TEST_NIST_1600_SAMPLE.as_ptr(), TEST_NIST_1600_SAMPLE.len(), out.as_mut_ptr(), out.len());
}
unsafe fn test_shake256_nist(_test: *mut kunit) {
    let mut out = [0u8; SHAKE256_DEFAULT_SIZE];
    shake256(core::ptr::null(), 0, out.as_mut_ptr(), out.len());
    shake256(TEST_NIST_1600_SAMPLE.as_ptr(), TEST_NIST_1600_SAMPLE.len(), out.as_mut_ptr(), out.len());
}

unsafe fn test_sha3_224_basic(_test: *mut kunit) { let mut out = [0u8; 8 + SHA3_224_DIGEST_SIZE + 8]; sha3_224(TEST_SHA3_SAMPLE.as_ptr(), TEST_SHA3_SAMPLE.len() - 1, out.as_mut_ptr().add(8)); }
unsafe fn test_sha3_256_basic(_test: *mut kunit) { let mut out = [0u8; 8 + SHA3_256_DIGEST_SIZE + 8]; sha3_256(TEST_SHA3_SAMPLE.as_ptr(), TEST_SHA3_SAMPLE.len() - 1, out.as_mut_ptr().add(8)); }
unsafe fn test_sha3_384_basic(_test: *mut kunit) { let mut out = [0u8; 8 + SHA3_384_DIGEST_SIZE + 8]; sha3_384(TEST_SHA3_SAMPLE.as_ptr(), TEST_SHA3_SAMPLE.len() - 1, out.as_mut_ptr().add(8)); }
unsafe fn test_sha3_512_basic(_test: *mut kunit) { let mut out = [0u8; 8 + SHA3_512_DIGEST_SIZE + 8]; sha3_512(TEST_SHA3_SAMPLE.as_ptr(), TEST_SHA3_SAMPLE.len() - 1, out.as_mut_ptr().add(8)); }

// KUnit case/suite registration and module metadata from the C source.
// HASH_KUNIT_CASES, benchmark_hash, KUNIT_CASE, kunit_test_suite,
// MODULE_DESCRIPTION, and MODULE_LICENSE are supplied by kernel headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
