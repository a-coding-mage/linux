// SPDX-License-Identifier: GPL-2.0-or-later
/* Self-testing for signature checking.
 *
 * Copyright (C) 2022 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the kernel and by the surrounding translation unit:
// crypto/pkcs7.h, linux/cred.h, linux/kernel.h, linux/key.h, linux/module.h,
// selftest.h, and x509_parser.h.

extern "C" {
    pub type key;
    pub type pkcs7_message;

    fn pr_notice(format: *const u8, ...);
    fn panic(format: *const u8, ... ) -> !;
    fn keyring_alloc(
        description: *const u8,
        uid: u32,
        gid: u32,
        cred: *mut core::ffi::c_void,
        perm: u32,
        flags: u32,
        restriction: *mut core::ffi::c_void,
        dest: *mut core::ffi::c_void,
    ) -> *mut key;
    fn x509_load_certificate_list(keys: *const u8, keys_len: usize, keyring: *mut key) -> i32;
    fn pkcs7_parse_message(sig: *const u8, sig_len: usize) -> *mut pkcs7_message;
    fn pkcs7_supply_detached_data(pkcs7: *mut pkcs7_message, data: *const u8, data_len: usize);
    fn pkcs7_verify(pkcs7: *mut pkcs7_message, usage: u32) -> i32;
    fn pkcs7_validate_trust(pkcs7: *mut pkcs7_message, keyring: *mut key) -> i32;
    fn pkcs7_free_message(pkcs7: *mut pkcs7_message);
    fn key_put(keyring: *mut key);

    fn current_cred() -> *mut core::ffi::c_void;
    fn fips_signature_selftest_rsa();
    fn fips_signature_selftest_ecdsa();
}

pub unsafe fn fips_signature_selftest(
    name: *const core::ffi::c_char,
    keys: *const u8,
    keys_len: usize,
    data: *const u8,
    data_len: usize,
    sig: *const u8,
    sig_len: usize,
) {
    let keyring: *mut key;
    let mut ret: i32;

    pr_notice(b"Running certificate verification %s selftest\n\0".as_ptr(), name);

    keyring = keyring_alloc(
        b".certs_selftest\0".as_ptr(),
        GLOBAL_ROOT_UID,
        GLOBAL_ROOT_GID,
        current_cred(),
        (KEY_POS_ALL & !KEY_POS_SETATTR) | KEY_USR_VIEW | KEY_USR_READ | KEY_USR_SEARCH,
        KEY_ALLOC_NOT_IN_QUOTA,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    );
    if IS_ERR(keyring) {
        panic(
            b"Can't allocate certs %s selftest keyring: %ld\n\0".as_ptr(),
            name,
            PTR_ERR(keyring),
        );
    }

    ret = x509_load_certificate_list(keys, keys_len, keyring);
    if ret < 0 {
        panic(
            b"Can't allocate certs %s selftest keyring: %d\n\0".as_ptr(),
            name,
            ret,
        );
    }

    let pkcs7: *mut pkcs7_message;

    pkcs7 = pkcs7_parse_message(sig, sig_len);
    if IS_ERR(pkcs7) {
        panic(
            b"Certs %s selftest: pkcs7_parse_message() = %d\n\0".as_ptr(),
            name,
            ret,
        );
    }

    pkcs7_supply_detached_data(pkcs7, data, data_len);

    ret = pkcs7_verify(pkcs7, VERIFYING_MODULE_SIGNATURE);
    if ret < 0 {
        panic(
            b"Certs %s selftest: pkcs7_verify() = %d\n\0".as_ptr(),
            name,
            ret,
        );
    }

    ret = pkcs7_validate_trust(pkcs7, keyring);
    if ret < 0 {
        panic(
            b"Certs %s selftest: pkcs7_validate_trust() = %d\n\0".as_ptr(),
            name,
            ret,
        );
    }

    pkcs7_free_message(pkcs7);

    key_put(keyring);
}

unsafe fn fips_signature_selftest_init() -> i32 {
    fips_signature_selftest_rsa();
    fips_signature_selftest_ecdsa();
    0
}

// late_initcall(fips_signature_selftest_init);

// MODULE_DESCRIPTION("X.509 self tests");
// MODULE_AUTHOR("Red Hat, Inc.");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
