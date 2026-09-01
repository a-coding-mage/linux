// SPDX-License-Identifier: GPL-2.0-only
/* Copyright 2025 Google LLC */

// C dependencies: <test_progs.h> and "bpf/libbpf_internal.h".

use core::ffi::c_void;
use core::ptr;

const MAX_LEN: usize = 4096;

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn libbpf_sha256(data: *const __u8, len: usize, digest: *mut __u8);
}

/* Test libbpf_sha256() for all lengths from 0 to MAX_LEN inclusively. */
pub unsafe extern "C" fn test_sha256() {
    /*
     * The correctness of this value was verified by running this test with
     * libbpf_sha256() replaced by OpenSSL's SHA256().
     */
    static EXPECTED_DIGEST_OF_DIGESTS: [__u8; SHA256_DIGEST_LENGTH] = [
        0x62, 0x30, 0x0e, 0x1d, 0xea, 0x7f, 0xc4, 0x74,
        0xfd, 0x8e, 0x64, 0x0b, 0xd8, 0x5f, 0xea, 0x04,
        0xf3, 0xef, 0x77, 0x42, 0xc2, 0x01, 0xb8, 0x90,
        0x6e, 0x19, 0x91, 0x1b, 0xca, 0xb3, 0x28, 0x42,
    ];
    let mut seed: __u64 = 0;
    let mut data: *mut __u8 = ptr::null_mut();
    let mut digests: *mut __u8 = ptr::null_mut();
    let mut digest_of_digests: [__u8; SHA256_DIGEST_LENGTH] = [0; SHA256_DIGEST_LENGTH];
    let mut i: usize;

    'out: loop {
        data = malloc(MAX_LEN) as *mut __u8;
        if !ASSERT_NEQ!(data, ptr::null_mut(), "malloc") {
            break 'out;
        }
        digests = malloc((MAX_LEN + 1) * SHA256_DIGEST_LENGTH) as *mut __u8;
        if !ASSERT_NEQ!(digests, ptr::null_mut(), "malloc") {
            break 'out;
        }

        /* Generate MAX_LEN bytes of "random" data deterministically. */
        i = 0;
        while i < MAX_LEN {
            seed = (seed.wrapping_mul(25214903917).wrapping_add(11)) & ((1u64 << 48) - 1);
            *data.add(i) = (seed >> 16) as __u8;
            i += 1;
        }

        /* Calculate a digest for each length 0 through MAX_LEN inclusively. */
        i = 0;
        while i <= MAX_LEN {
            libbpf_sha256(data, i, digests.add(i * SHA256_DIGEST_LENGTH));
            i += 1;
        }

        /* Calculate and verify the digest of all the digests. */
        libbpf_sha256(
            digests,
            (MAX_LEN + 1) * SHA256_DIGEST_LENGTH,
            digest_of_digests.as_mut_ptr(),
        );
        ASSERT_MEMEQ!(
            digest_of_digests.as_ptr(),
            EXPECTED_DIGEST_OF_DIGESTS.as_ptr(),
            SHA256_DIGEST_LENGTH,
            "digest_of_digests"
        );
        break 'out;
    }
    free(data as *mut c_void);
    free(digests as *mut c_void);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
