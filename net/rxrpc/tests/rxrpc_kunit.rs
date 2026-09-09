// SPDX-License-Identifier: GPL-2.0-only
/*
 * Unit tests for RxRPC crypto functions
 *
 * Copyright 2026 Google LLC
 */

// Dependencies supplied by the surrounding kernel/RxRPC environment:
// ../ar-internal.h, crypto/des.h, and kunit/test.h

#[repr(C)]
pub struct fcrypt_pcbc_testvec {
    pub key: [u8; FCRYPT_BSIZE],
    pub iv: [u8; FCRYPT_BSIZE],
    pub ptext: *const u8, // plaintext
    pub ctext: *const u8, // ciphertext
    pub nblocks: usize, // length of ptext and ctext in blocks
}

// FCrypt-PCBC test vectors
static FCRYPT_PCBC_TESTVECS: &[fcrypt_pcbc_testvec] = &[
    fcrypt_pcbc_testvec {
        // http://www.openafs.org/pipermail/openafs-devel/2000-December/005320.html
        key: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        iv: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        ptext: b"\x00\x00\x00\x00\x00\x00\x00\x00".as_ptr(),
        ctext: b"\x0E\x09\x00\xC7\x3E\xF7\xED\x41".as_ptr(),
        nblocks: 1,
    },
    fcrypt_pcbc_testvec {
        key: [0x11, 0x44, 0x77, 0xAA, 0xDD, 0x00, 0x33, 0x66],
        iv: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        ptext: b"\x12\x34\x56\x78\x9A\xBC\xDE\xF0".as_ptr(),
        ctext: b"\xD8\xED\x78\x74\x77\xEC\x06\x80".as_ptr(),
        nblocks: 1,
    },
    fcrypt_pcbc_testvec {
        // From Arla
        key: [0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87],
        iv: [0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10],
        ptext: b"The quick brown fox jumps over the lazy dogs.\0\0".as_ptr(),
        ctext: b"\x00\xf0\x0e\x11\x75\xe6\x23\x82\xee\xac\x98\x62\x44\x51\xe4\x84\xc3\x59\xd8\xaa\x64\x60\xae\xf7\xd2\xd9\x13\x79\x72\xa3\x45\x03\x23\xb5\x62\xd7\x0c\xf5\x27\xd1\xf8\x91\x3c\xac\x44\x22\x92\xef".as_ptr(),
        nblocks: 6,
    },
    fcrypt_pcbc_testvec {
        key: [0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10],
        iv: [0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87],
        ptext: b"The quick brown fox jumps over the lazy dogs.\0\0".as_ptr(),
        ctext: b"\xca\x90\xf5\x9d\xcb\xd4\xd2\x3c\x01\x88\x7f\x3e\x31\x6e\x62\x9d\xd8\xe0\x57\xa3\x06\x3a\x42\x58\x2a\x28\xfe\x72\x52\x2f\xdd\xe0\x19\x89\x09\x1c\x2a\x8e\x8c\x94\xfc\xc7\x68\xe4\x88\xaa\xde\x0f".as_ptr(),
        nblocks: 6,
    },
];

extern "C" {
    fn fcrypt_preparekey(key: *mut fcrypt_key, raw_key: *const u8);
    fn fcrypt_pcbc_encrypt(key: *const fcrypt_key, iv: *const u8, src: *const u8, dst: *mut u8, nblocks: usize);
    fn fcrypt_pcbc_decrypt(key: *const fcrypt_key, iv: *const u8, src: *const u8, dst: *mut u8, nblocks: usize);
    fn des_expand_key(ctx: *mut des_ctx, key: *const u8, len: usize) -> i32;
    fn des_pcbc_decrypt_inplace(ctx: *const des_ctx, iv: __le64, data: *mut u8, len: usize);
}

unsafe fn test_fcrypt_pcbc(test: *mut kunit) {
    let mut data = [0u8; 48];

    for tv in FCRYPT_PCBC_TESTVECS.iter() {
        let nblocks = tv.nblocks;
        let len = nblocks * FCRYPT_BSIZE;
        let mut key: fcrypt_key = core::mem::zeroed();

        KUNIT_ASSERT_GE!(test, core::mem::size_of_val(&data), len);

        fcrypt_preparekey(&mut key, tv.key.as_ptr());

        // out-of-place encryption
        fcrypt_pcbc_encrypt(&key, tv.iv.as_ptr(), tv.ptext, data.as_mut_ptr(), nblocks);
        KUNIT_ASSERT_MEMEQ!(test, tv.ctext, data.as_ptr(), len);

        // in-place encryption
        core::ptr::copy_nonoverlapping(tv.ptext, data.as_mut_ptr(), len);
        fcrypt_pcbc_encrypt(&key, tv.iv.as_ptr(), data.as_ptr(), data.as_mut_ptr(), nblocks);
        KUNIT_ASSERT_MEMEQ!(test, tv.ctext, data.as_ptr(), len);

        // out-of-place decryption
        fcrypt_pcbc_decrypt(&key, tv.iv.as_ptr(), tv.ctext, data.as_mut_ptr(), nblocks);
        KUNIT_ASSERT_MEMEQ!(test, tv.ptext, data.as_ptr(), len);

        // in-place decryption
        core::ptr::copy_nonoverlapping(tv.ctext, data.as_mut_ptr(), len);
        fcrypt_pcbc_decrypt(&key, tv.iv.as_ptr(), data.as_ptr(), data.as_mut_ptr(), nblocks);
        KUNIT_ASSERT_MEMEQ!(test, tv.ptext, data.as_ptr(), len);
    }
}

unsafe fn test_des_pcbc(test: *mut kunit) {
    // This was generated from the original pcbc(des) crypto API code.
    static EXPECTED_PTEXT: [u8; 24] = [
        0xc8, 0xe2, 0x3c, 0xdf, 0x80, 0x61, 0x8a, 0xad, 0xa5, 0x52, 0xb4, 0x20,
        0x74, 0x32, 0x1f, 0xe4, 0x2c, 0x15, 0x7d, 0x21, 0x57, 0xda, 0x3f, 0x31,
    ];
    let mut key = [0u8; 8];
    let mut iv: des_iv = unsafe { core::mem::zeroed() };
    let mut data = [0u8; 24];
    let mut ctx: des_ctx = unsafe { core::mem::zeroed() };

    for i in 0..8 {
        key[i] = i as u8;
        iv.b[i] = (255 - i) as u8;
    }
    for i in 0..data.len() {
        data[i] = i as u8;
    }

    let err = des_expand_key(&mut ctx, key.as_ptr(), key.len());
    KUNIT_ASSERT_EQ!(test, 0, err);

    des_pcbc_decrypt_inplace(&ctx, iv.w, data.as_mut_ptr(), data.len());
    KUNIT_ASSERT_MEMEQ!(test, EXPECTED_PTEXT.as_ptr(), data.as_ptr(), data.len());
}

// KUNIT_CASE(test_fcrypt_pcbc), KUNIT_CASE(test_des_pcbc)
static RXRPC_TEST_SUITE: &str = "rxrpc";

// kunit_test_suite(rxrpc_test_suite);
// MODULE_DESCRIPTION("Unit tests for RxRPC crypto functions");
// MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
