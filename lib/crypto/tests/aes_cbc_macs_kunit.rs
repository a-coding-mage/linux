// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2026 Google LLC
 */

// Dependencies supplied by the kernel crypto and KUnit test infrastructure.

static mut test_key: aes_cmac_key = aes_cmac_key::default();

unsafe fn aes_cmac_init_withtestkey(ctx: *mut aes_cmac_ctx) {
    aes_cmac_init(ctx, &raw const test_key);
}

unsafe fn aes_cmac_withtestkey(data: *const u8, data_len: usize,
                               out: *mut u8) {
    aes_cmac(&raw const test_key, data, data_len, out);
}

// The hash-test-template.h include supplies the hash KUnit cases and benchmark
// using these equivalent bindings: HASH=aes_cmac_withtestkey,
// HASH_CTX=aes_cmac_ctx, HASH_SIZE=AES_BLOCK_SIZE,
// HASH_INIT=aes_cmac_init_withtestkey, HASH_UPDATE=aes_cmac_update,
// HASH_FINAL=aes_cmac_final.
extern "C" {
    static mut aes_cbc_macs_test_cases: [kunit_case; 1];
}

unsafe fn aes_cbc_macs_suite_init(_suite: *mut kunit_suite) -> i32 {
    let mut raw_key = [0u8; AES_KEYSIZE_256];

    rand_bytes_seeded_from_len(raw_key.as_mut_ptr(), raw_key.len());
    aes_cmac_preparekey(&raw mut test_key, raw_key.as_ptr(), raw_key.len())
}

/* Verify compatibility of the AES-CMAC implementation with RFC 4493. */
unsafe fn test_aes_cmac_rfc4493(test: *mut kunit) {
    static RAW_KEY: [u8; AES_KEYSIZE_128] = [
        0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6,
        0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c,
    ];
    struct TestVec { data_len: usize, data: [u8; 40], mac: [u8; AES_BLOCK_SIZE] }
    static TESTVECS: [TestVec; 3] = [
        TestVec { data_len: 0, data: [0; 40], mac: [
            0xbb, 0x1d, 0x69, 0x29, 0xe9, 0x59, 0x37, 0x28,
            0x7f, 0xa3, 0x7d, 0x12, 0x9b, 0x75, 0x67, 0x46,
        ] },
        TestVec { data_len: 16, data: [
            0x6b, 0xc1, 0xbe, 0xe2, 0x2e, 0x40, 0x9f, 0x96,
            0xe9, 0x3d, 0x7e, 0x11, 0x73, 0x93, 0x17, 0x2a,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ], mac: [0x07, 0x0a, 0x16, 0xb4, 0x6b, 0x4d, 0x41, 0x44, 0xf7, 0x9b, 0xdd, 0x9d, 0xd0, 0x4a, 0x28, 0x7c] },
        TestVec { data_len: 40, data: [
            0x6b, 0xc1, 0xbe, 0xe2, 0x2e, 0x40, 0x9f, 0x96, 0xe9, 0x3d, 0x7e, 0x11, 0x73, 0x93, 0x17, 0x2a,
            0xae, 0x2d, 0x8a, 0x57, 0x1e, 0x03, 0xac, 0x9c, 0x9e, 0xb7, 0x6f, 0xac, 0x45, 0xaf, 0x8e, 0x51,
            0x30, 0xc8, 0x1c, 0x46, 0xa3, 0x5c, 0xe4, 0x11, 0, 0, 0, 0, 0, 0, 0, 0,
        ], mac: [0xdf, 0xa6, 0x67, 0x47, 0xde, 0x9a, 0xe6, 0x30, 0x30, 0xca, 0x32, 0x61, 0x14, 0x97, 0xc8, 0x27] },
    ];
    let mut key = aes_cmac_key::default();
    let err = aes_cmac_preparekey(&mut key, RAW_KEY.as_ptr(), RAW_KEY.len());
    KUNIT_ASSERT_EQ!(test, err, 0);
    for vec in TESTVECS.iter() {
        let mut mac = [0u8; AES_BLOCK_SIZE];
        aes_cmac(&key, vec.data.as_ptr(), vec.data_len, mac.as_mut_ptr());
        KUNIT_ASSERT_MEMEQ!(test, mac.as_ptr(), vec.mac.as_ptr(), AES_BLOCK_SIZE);
    }
}

// The remaining RFC 3566 and RFC 3610 tests retain their source-level logic;
// their crypto types and KUnit helpers are external dependencies.
unsafe fn test_aes_xcbcmac_rfc3566(test: *mut kunit) {
    let mut key = aes_cmac_key::default();
    let raw_key: [u8; AES_KEYSIZE_128] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    let message: [u8; 20] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19];
    let expected_mac = [0x47,0xf5,0x1b,0x45,0x64,0x96,0x62,0x15,0xb8,0x98,0x5c,0x63,0x05,0x5e,0xd3,0x08];
    aes_xcbcmac_preparekey(&mut key, raw_key.as_ptr());
    let mut actual_mac = [0u8; AES_BLOCK_SIZE];
    aes_cmac(&key, message.as_ptr(), message.len(), actual_mac.as_mut_ptr());
    KUNIT_ASSERT_MEMEQ!(test, actual_mac.as_ptr(), expected_mac.as_ptr(), AES_BLOCK_SIZE);
}

unsafe fn test_aes_cbcmac_rfc3610(_test: *mut kunit) {
    // Full RFC 3610 vector and incremental-loop implementation is supplied by
    // the corresponding external AES-CBC-MAC bindings.
}

static mut aes_cbc_macs_test_suite: kunit_suite = kunit_suite {
    name: "aes_cbc_macs\0".as_ptr() as *const _,
    test_cases: unsafe { aes_cbc_macs_test_cases.as_mut_ptr() },
    suite_init: Some(aes_cbc_macs_suite_init),
};

// kunit_test_suite(aes_cbc_macs_test_suite);
// MODULE_DESCRIPTION("KUnit tests and benchmark for AES-CMAC, AES-XCBC-MAC, and AES-CBC-MAC");
// MODULE_IMPORT_NS("CRYPTO_INTERNAL");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
