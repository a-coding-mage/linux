/* Extract X.509 certificate in DER form from PKCS#11 or PEM.
 *
 * Copyright © 2014-2015 Red Hat, Inc. All Rights Reserved.
 * Copyright © 2015      Intel Corporation.
 *
 * Authors: David Howells <dhowells@redhat.com>
 *          David Woodhouse <dwmw2@infradead.org>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public License
 * as published by the Free Software Foundation; either version 2.1
 * of the licence, or (at your option) any later version.
 */

use std::ffi::{c_char, c_int, c_ulong, c_void, CStr};
use std::ptr;

// OpenSSL and ssl-common.h declarations are supplied by the surrounding build.
#[repr(C)] pub struct BIO { _private: [u8; 0] }
#[repr(C)] pub struct X509 { _private: [u8; 0] }
#[repr(C)] pub struct X509_NAME { _private: [u8; 0] }
#[repr(C)] pub struct OSSL_STORE_CTX { _private: [u8; 0] }
#[repr(C)] pub struct OSSL_STORE_INFO { _private: [u8; 0] }

extern "C" {
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fclose(file: *mut c_void) -> c_int;
    fn OpenSSL_add_all_algorithms();
    fn ERR_load_crypto_strings();
    fn ERR_clear_error();
    fn ERR_peek_last_error() -> c_ulong;
    fn ERR_GET_LIB(e: c_ulong) -> c_int;
    fn ERR_GET_REASON(e: c_ulong) -> c_int;
    fn drain_openssl_errors(line: c_int, fatal: c_int);
    fn BIO_new_file(path: *const c_char, mode: *const c_char) -> *mut BIO;
    fn BIO_free(b: *mut BIO);
    fn X509_get_subject_name(x: *mut X509) -> *mut X509_NAME;
    fn X509_NAME_oneline(n: *mut X509_NAME, buf: *mut c_char, size: c_int) -> *mut c_char;
    fn i2d_X509_bio(b: *mut BIO, x: *mut X509) -> c_int;
    fn PEM_read_bio_X509(b: *mut BIO, x: *mut *mut X509, cb: *mut c_void, u: *mut c_void) -> *mut X509;
    fn OSSL_PROVIDER_try_load(ctx: *mut c_void, name: *const c_char, retain: bool) -> *mut c_void;
    fn OSSL_STORE_open(uri: *const c_char, ui: *mut c_void, ui_data: *mut c_void, post_process: *mut c_void, post_process_data: *mut c_void) -> *mut OSSL_STORE_CTX;
    fn OSSL_STORE_eof(ctx: *mut OSSL_STORE_CTX) -> bool;
    fn OSSL_STORE_load(ctx: *mut OSSL_STORE_CTX) -> *mut OSSL_STORE_INFO;
    fn OSSL_STORE_INFO_get_type(info: *const OSSL_STORE_INFO) -> c_int;
    fn OSSL_STORE_INFO_get1_CERT(info: *const OSSL_STORE_INFO) -> *mut X509;
    fn OSSL_STORE_INFO_free(info: *mut OSSL_STORE_INFO);
    fn OSSL_STORE_close(ctx: *mut OSSL_STORE_CTX) -> c_int;
}

static mut WB: *mut BIO = ptr::null_mut();
static mut CERT_DST: *mut c_char = ptr::null_mut();
static mut VERBOSE: bool = false;

#[inline(never)]
unsafe fn format() -> ! {
    // fprintf(stderr, "Usage: extract-cert <source> <dest>\n");
    exit(2)
}

unsafe fn write_cert(x509: *mut X509) {
    let mut buf = [0 as c_char; 200];
    if WB.is_null() {
        WB = BIO_new_file(CERT_DST, b"wb\0".as_ptr() as *const c_char);
        // ERR(!wb, "%s", cert_dst);
        if WB.is_null() { exit(1); }
    }
    X509_NAME_oneline(X509_get_subject_name(x509), buf.as_mut_ptr(), buf.len() as c_int);
    if i2d_X509_bio(WB, x509) == 0 { exit(1); }
    if VERBOSE {
        // fprintf(stderr, "Extracted cert: %s\n", buf);
    }
}

unsafe fn load_cert_pkcs11(cert_src: *const c_char) -> *mut X509 {
    let mut cert: *mut X509 = ptr::null_mut();
    // USE_PKCS11_PROVIDER is selected by the OpenSSL version at build time.
    if OSSL_PROVIDER_try_load(ptr::null_mut(), b"pkcs11\0".as_ptr() as *const c_char, true).is_null() { exit(1); }
    if OSSL_PROVIDER_try_load(ptr::null_mut(), b"default\0".as_ptr() as *const c_char, true).is_null() { exit(1); }
    let store = OSSL_STORE_open(cert_src, ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
    if store.is_null() { exit(1); }
    while !OSSL_STORE_eof(store) {
        let info = OSSL_STORE_load(store);
        if info.is_null() {
            drain_openssl_errors(0, 0);
            continue;
        }
        // OSSL_STORE_INFO_CERT
        if OSSL_STORE_INFO_get_type(info) == 1 {
            cert = OSSL_STORE_INFO_get1_CERT(info);
            if cert.is_null() { exit(1); }
        }
        OSSL_STORE_INFO_free(info);
        if !cert.is_null() { break; }
    }
    OSSL_STORE_close(store);
    cert
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    OpenSSL_add_all_algorithms();
    ERR_load_crypto_strings();
    ERR_clear_error();
    let verbose_env = getenv(b"KBUILD_VERBOSE\0".as_ptr() as *const c_char);
    if !verbose_env.is_null() && !strchr(verbose_env, '1' as c_int).is_null() { VERBOSE = true; }
    if argc != 3 { format(); }
    let cert_src = *argv.add(1);
    CERT_DST = *argv.add(2);
    if *cert_src == 0 {
        let f = fopen(CERT_DST, b"wb\0".as_ptr() as *const c_char);
        if f.is_null() { exit(1); }
        fclose(f);
        exit(0);
    } else if strncmp(cert_src, b"pkcs11:\0".as_ptr() as *const c_char, 7) == 0 {
        let cert = load_cert_pkcs11(cert_src);
        if cert.is_null() { exit(1); }
        write_cert(cert);
    } else {
        let b = BIO_new_file(cert_src, b"rb\0".as_ptr() as *const c_char);
        if b.is_null() { exit(1); }
        loop {
            let x509 = PEM_read_bio_X509(b, ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
            if !WB.is_null() && x509.is_null() {
                let err = ERR_peek_last_error();
                if ERR_GET_LIB(err) == 9 && ERR_GET_REASON(err) == 108 {
                    ERR_clear_error();
                    break;
                }
            }
            if x509.is_null() { exit(1); }
            write_cert(x509);
        }
    }
    if !WB.is_null() { BIO_free(WB); }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
