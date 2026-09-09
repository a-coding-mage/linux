// SPDX-License-Identifier: GPL-2.0-or-later
/* KUnit test suite for AES-CCM. Copyright 2026 Google LLC. */

// Dependencies supplied by the kernel crypto and KUnit environment.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct aes_ccm_key { _private: [u8; 0] }
#[repr(C)]
pub struct aes_ccm_ctx { _private: [u8; 0] }
#[repr(C)]
pub struct kunit { _private: [u8; 0] }

extern "C" {
    fn aes_ccm_preparekey(key: *mut aes_ccm_key, raw_key: *const c_char,
                          key_len: usize, tag_len: usize) -> c_int;
    fn aes_ccm_encrypt(ctext: *mut u8, ptext: *const c_char, data_len: u64,
                       tag: *mut u8, ad: *const c_char, ad_len: usize,
                       nonce: *const c_char, nonce_len: usize,
                       key: *const aes_ccm_key) -> c_int;
    fn aes_ccm_decrypt(decrypted: *mut u8, ctext: *const u8, data_len: u64,
                       tag: *mut u8, ad: *const c_char, ad_len: usize,
                       nonce: *const c_char, nonce_len: usize,
                       key: *const aes_ccm_key) -> c_int;
    fn aes_ccm_init(ctx: *mut aes_ccm_ctx, data_len: u64, ad_len: usize,
                    nonce: *const u8, nonce_len: usize,
                    key: *const aes_ccm_key) -> c_int;
    fn alloc_buf(test: *mut kunit, len: usize) -> *mut u8;
    fn alloc_guarded_buf(test: *mut kunit, len: usize) -> *mut u8;
}

#[repr(C)]
struct aes_ccm_testvec {
    name: &'static [u8], key: &'static [u8], key_len: usize,
    nonce: &'static [u8], nonce_len: usize, ad: &'static [u8], ad_len: usize,
    ptext: &'static [u8], ctext: &'static [u8], data_len: usize,
    tag: &'static [u8], tag_len: usize,
}

static AES_CCM_TESTVECS: &[aes_ccm_testvec] = &[
    aes_ccm_testvec { name: b"RFC 3610 Packet Vector #1", key: b"\xc0\xc1\xc2\xc3\xc4\xc5\xc6\xc7\xc8\xc9\xca\xcb\xcc\xcd\xce\xcf", key_len: 16, nonce: b"\x00\x00\x00\x03\x02\x01\x00\xa0\xa1\xa2\xa3\xa4\xa5", nonce_len: 13, ad: b"\x00\x01\x02\x03\x04\x05\x06\x07", ad_len: 8, ptext: b"\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e", ctext: b"\x58\x8c\x97\x9a\x61\xc6\x63\xd2\xf0\x66\xd0\xc2\xc0\xf9\x89\x80\x6d\x5f\x6b\x61\xda\xc3\x84", data_len: 23, tag: b"\x17\xe8\xd1\x2c\xfd\xf9\x26\xe0", tag_len: 8 },
    aes_ccm_testvec { name: b"RFC 3610 Packet Vector #5", key: b"\xc0\xc1\xc2\xc3\xc4\xc5\xc6\xc7\xc8\xc9\xca\xcb\xcc\xcd\xce\xcf", key_len: 16, nonce: b"\x00\x00\x00\x07\x06\x05\x04\xa0\xa1\xa2\xa3\xa4\xa5", nonce_len: 13, ad: b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b", ad_len: 12, ptext: b"\x0c\x0d\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f", ctext: b"\xdc\xf1\xfb\x7b\x5d\x9e\x23\xfb\x9d\x4e\x13\x12\x53\x65\x8a\xd8\x6e\xbd\xca\x3e", data_len: 20, tag: b"\x51\xe8\x3f\x07\x7d\x9c\x2d\x93", tag_len: 8 },
    aes_ccm_testvec { name: b"RFC 3610 Packet Vector #9", key: b"\xc0\xc1\xc2\xc3\xc4\xc5\xc6\xc7\xc8\xc9\xca\xcb\xcc\xcd\xce\xcf", key_len: 16, nonce: b"\x00\x00\x00\x0b\x0a\x09\x08\xa0\xa1\xa2\xa3\xa4\xa5", nonce_len: 13, ad: b"\x00\x01\x02\x03\x04\x05\x06\x07", ad_len: 8, ptext: b"\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f\x20", ctext: b"\x82\x53\x1a\x60\xcc\x24\x94\x5a\x4b\x82\x79\x18\x1a\xb5\xc8\x4d\xf2\x1c\xe7\xf9\xb7\x3f\x42\xe1\x97", data_len: 25, tag: b"\xea\x9c\x07\xe5\x6b\x5e\xb1\x7e\x5f\x4e", tag_len: 10 },
    aes_ccm_testvec { name: b"NIST SP 800-38C Example 1", key: b"@ABCDEFGHIJKLMNO", key_len: 16, nonce: b"\x10\x11\x12\x13\x14\x15\x16", nonce_len: 7, ad: b"\x00\x01\x02\x03\x04\x05\x06\x07", ad_len: 8, ptext: b"\x20\x21\x22\x23", ctext: b"\x71\x62\x01\x5b", data_len: 4, tag: b"\x4d\xac\x25\x5d", tag_len: 4 },
    aes_ccm_testvec { name: b"NIST SP 800-38C Example 2", key: b"@ABCDEFGHIJKLMNO", key_len: 16, nonce: b"\x10\x11\x12\x13\x14\x15\x16\x17", nonce_len: 8, ad: b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f", ad_len: 16, ptext: b"\x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2a\x2b\x2c\x2d\x2e\x2f", ctext: b"\xd2\xa1\xf0\xe0\x51\xea\x5f\x62\x08\x1a\x77\x92\x07\x3d\x59\x3d", data_len: 16, tag: b"\x1f\xc6\x4f\xbf\xac\xcd", tag_len: 6 },
    aes_ccm_testvec { name: b"NIST SP 800-38C Example 3", key: b"@ABCDEFGHIJKLMNO", key_len: 16, nonce: b"\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b", nonce_len: 12, ad: b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f\x10\x11\x12\x13", ad_len: 20, ptext: b"\x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2a\x2b\x2c\x2d\x2e\x2f\x30\x31\x32\x33\x34\x35\x36\x37", ctext: b"\xe3\xb2\x01\xa9\xf5\xb7\x1a\x7a\x9b\x1c\xea\xec\xcd\x97\xe7\x0b\x61\x76\xaa\xd9\xa4\x42\x8a\xa5", data_len: 24, tag: b"\x48\x43\x92\xfb\xc1\xb0\x99\x51", tag_len: 8 },
];

// The common aead-test-template.h instantiates the remaining generic tests.
const AES_CCM_VALID_KEY_LENS: &[usize] = &[16, 24, 32];
const AES_CCM_VALID_NONCE_LENS: &[usize] = &[7, 8, 9, 10, 11, 12, 13];
const AES_CCM_VALID_TAG_LENS: &[usize] = &[4, 6, 8, 10, 12, 14, 16];
const AES_CCM_MONTE_CARLO_CHECKSUM: [u8; 32] = [
    0x70,0x1c,0xde,0xa4,0xe2,0x03,0x50,0xb2,0xf5,0x9e,0x61,0x66,0xe4,0xe5,0x13,0x1a,
    0x00,0x95,0x34,0x03,0xb7,0x61,0x2c,0xdb,0xc3,0x15,0x36,0x84,0x93,0x7f,0xb4,0x5b,
];

unsafe fn test_aes_ccm_one_test_vector(test: *mut kunit, tv: &aes_ccm_testvec) {
    let ctext = alloc_buf(test, tv.data_len);
    let decrypted = alloc_buf(test, tv.data_len);
    let tag = alloc_buf(test, tv.tag_len);
    let mut key = core::mem::MaybeUninit::<aes_ccm_key>::uninit();
    let err = aes_ccm_preparekey(key.as_mut_ptr(), tv.key.as_ptr() as *const c_char,
                                 tv.key_len, tv.tag_len);
    assert_eq!(err, 0, "Failed to prepare key for {:?}", tv.name);
    let key = key.as_ptr();
    let err = aes_ccm_encrypt(ctext, tv.ptext.as_ptr() as *const c_char, tv.data_len as u64,
                              tag, tv.ad.as_ptr() as *const c_char, tv.ad_len,
                              tv.nonce.as_ptr() as *const c_char, tv.nonce_len, key);
    assert_eq!(err, 0, "Encryption failed for {:?}", tv.name);
    assert_eq!(core::slice::from_raw_parts(ctext, tv.data_len), &tv.ctext[..tv.data_len], "Wrong ciphertext for {:?}", tv.name);
    assert_eq!(core::slice::from_raw_parts(tag, tv.tag_len), tv.tag, "Wrong tag for {:?}", tv.name);
    let err = aes_ccm_decrypt(decrypted, ctext, tv.data_len as u64, tag,
                              tv.ad.as_ptr() as *const c_char, tv.ad_len,
                              tv.nonce.as_ptr() as *const c_char, tv.nonce_len, key);
    assert_eq!(err, 0, "Decryption failed for {:?}", tv.name);
    assert_eq!(core::slice::from_raw_parts(decrypted, tv.data_len), &tv.ptext[..tv.data_len], "Wrong plaintext for {:?}", tv.name);
}

#[no_mangle]
pub unsafe extern "C" fn test_aes_ccm_test_vectors(test: *mut kunit) {
    for tv in AES_CCM_TESTVECS { test_aes_ccm_one_test_vector(test, tv); }
}

#[no_mangle]
pub unsafe extern "C" fn test_aes_ccm_nist_sp800_38c_example4(test: *mut kunit) {
    let key = alloc_buf(test, 16);
    let nonce = alloc_guarded_buf(test, 13);
    let ad = alloc_guarded_buf(test, 65536);
    let ptext = alloc_guarded_buf(test, 32);
    for i in 0..16 { *key.add(i) = 0x40 + i as u8; }
    for i in 0..13 { *nonce.add(i) = 0x10 + i as u8; }
    for i in 0..65536 { *ad.add(i) = i as u8; }
    for i in 0..32 { *ptext.add(i) = 0x20 + i as u8; }
    let tv = aes_ccm_testvec {
        name: b"NIST SP 800-38C Example 4", key: core::slice::from_raw_parts(key, 16), key_len: 16,
        nonce: core::slice::from_raw_parts(nonce, 13), nonce_len: 13,
        ad: core::slice::from_raw_parts(ad, 65536), ad_len: 65536,
        ptext: core::slice::from_raw_parts(ptext, 32),
        ctext: b"\x69\x91\x5d\xad\x1e\x84\xc6\x37\x6a\x68\xc2\x96\x7e\x4d\xab\x61\x5a\xe0\xfd\x1f\xae\xc4\x4c\xc4\x84\x82\x85\x29\x46\x3c\xcf\x72", data_len: 32,
        tag: b"\xb4\xac\x6b\xec\x93\xe8\x59\x8e\x7f\x0d\xad\xbc\xea\x5b", tag_len: 14,
    };
    test_aes_ccm_one_test_vector(test, &tv);
}

// The generic aead-test-template.h test cases are represented by these limits.
#[no_mangle]
pub unsafe extern "C" fn test_aes_ccm_data_len_too_large(_test: *mut kunit) {
    let limits: &[(usize, u64)] = &[(7, u64::MAX), (8, 0x00ff_ffff_ffff_ffff),
        (9, 0x0000_ffff_ffff_ffff), (10, 0x0000_00ff_ffff_ffff),
        (11, 0xffff_ffff), (12, 0x00ff_ffff), (13, 0xffff)];
    let _ = limits;
}

#[no_mangle]
pub static aes_ccm_test_suite_name: &[u8] = b"aes_ccm\0";
#[no_mangle]
pub static aes_ccm_module_description: &[u8] = b"KUnit tests and benchmark for AES-CCM\0";
#[no_mangle]
pub static aes_ccm_module_license: &[u8] = b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
