/* Sign a module file using the given key.
 *
 * Copyright © 2014-2016 Red Hat, Inc. All Rights Reserved.
 * Copyright © 2015      Intel Corporation.
 * Copyright © 2016      Hewlett Packard Enterprise Development LP
 *
 * Authors: David Howells <dhowells@redhat.com>
 *          David Woodhouse <dwmw2@infradead.org>
 *          Juerg Haefliger <juerg.haefliger@hpe.com>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public License
 * as published by the Free Software Foundation; either version 2.1
 * of the licence, or (at your option) any later version.
 */

// C dependencies supplied by the surrounding build: libc, OpenSSL, ssl-common,
// and linux/module_signature.h.

use std::ffi::{c_char, c_int, c_uint, c_ulong, c_void, CStr};
use std::ptr;

#[repr(C)]
struct module_signature {
    id_type: u8,
    _pad: [u8; 3],
    sig_len: u32,
}

const MODULE_SIGNATURE_TYPE_PKCS7: u8 = 2;
extern "C" {
    static MODULE_SIGNATURE_MARKER: [c_char; 0];
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn exit(status: c_int) -> !;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn asprintf(ret: *mut *mut c_char, format: *const c_char, ...) -> c_int;
    fn rename(old: *const c_char, new: *const c_char) -> c_int;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    static mut optind: c_int;
    fn htonl(hostlong: u32) -> u32;
}

#[repr(C)] struct EVP_PKEY { _private: [u8; 0] }
#[repr(C)] struct X509 { _private: [u8; 0] }
#[repr(C)] struct CMS_ContentInfo { _private: [u8; 0] }
#[repr(C)] struct BIO { _private: [u8; 0] }
#[repr(C)] struct EVP_MD { _private: [u8; 0] }

extern "C" {
    fn BIO_new_file(name: *const c_char, mode: *const c_char) -> *mut BIO;
    fn BIO_free(b: *mut BIO) -> c_int;
    fn BIO_read(b: *mut BIO, buf: *mut c_void, len: c_int) -> c_int;
    fn BIO_write(b: *mut BIO, buf: *const c_void, len: c_int) -> c_int;
    fn BIO_reset(b: *mut BIO) -> c_int;
    fn BIO_should_retry(b: *mut BIO) -> c_int;
    fn BIO_number_written(b: *mut BIO) -> c_ulong;
    fn PEM_read_bio_PrivateKey(b: *mut BIO, p: *mut *mut EVP_PKEY, cb: Option<unsafe extern "C" fn(*mut c_char, c_int, c_int, *mut c_void) -> c_int>, u: *mut c_void) -> *mut EVP_PKEY;
    fn PEM_read_bio_X509(b: *mut BIO, x: *mut *mut X509, cb: *mut c_void, u: *mut c_void) -> *mut X509;
    fn d2i_X509_bio(b: *mut BIO, x: *mut *mut X509) -> *mut X509;
    fn CMS_sign(a: *mut c_void, b: *mut c_void, c: *mut c_void, d: *mut c_void, flags: c_uint) -> *mut CMS_ContentInfo;
    fn CMS_add1_signer(cms: *mut CMS_ContentInfo, x: *mut X509, key: *mut EVP_PKEY, md: *const EVP_MD, flags: c_uint) -> *mut c_void;
    fn CMS_final(cms: *mut CMS_ContentInfo, b: *mut BIO, d: *mut c_void, flags: c_uint) -> c_int;
    fn i2d_CMS_bio_stream(b: *mut BIO, cms: *mut CMS_ContentInfo, d: *mut c_void, flags: c_int) -> c_int;
    fn EVP_get_digestbyname(name: *const c_char) -> *const EVP_MD;
    fn OpenSSL_add_all_algorithms();
    fn OpenSSL_add_all_digests();
    fn ERR_load_crypto_strings();
    fn ERR_clear_error();
    fn drain_openssl_errors(line: c_int, do_exit: c_int);
}

static mut key_pass: *mut c_char = ptr::null_mut();

unsafe extern "C" fn format() -> ! {
    fprintf(stderr, b"Usage: scripts/sign-file [-dp] <hash algo> <key> <x509> <module> [<dest>]\0".as_ptr() as _,);
    fprintf(stderr, b"       scripts/sign-file -s <raw sig> <hash algo> <x509> <module> [<dest>]\n\0".as_ptr() as _);
    exit(2)
}

unsafe extern "C" fn pem_pw_cb(buf: *mut c_char, len: c_int, _w: c_int, _v: *mut c_void) -> c_int {
    if key_pass.is_null() { return -1; }
    let pwlen = strlen(key_pass) as c_int;
    if pwlen >= len { return -1; }
    strcpy(buf, key_pass);
    key_pass = ptr::null_mut();
    pwlen
}

unsafe fn read_private_key(name: *const c_char) -> *mut EVP_PKEY {
    if strncmp(name, b"pkcs11:\0".as_ptr() as _, 7) == 0 {
        fprintf(stderr, b"no pkcs11 engine/provider available\n\0".as_ptr() as _);
        exit(1)
    }
    let b = BIO_new_file(name, b"rb\0".as_ptr() as _);
    if b.is_null() { exit(1); }
    let key = PEM_read_bio_PrivateKey(b, ptr::null_mut(), Some(pem_pw_cb), ptr::null_mut());
    if key.is_null() { exit(1); }
    BIO_free(b);
    key
}

unsafe fn read_x509(name: *const c_char) -> *mut X509 {
    let mut buf = [0u8; 2];
    let b = BIO_new_file(name, b"rb\0".as_ptr() as _);
    if b.is_null() { exit(1); }
    let n = BIO_read(b, buf.as_mut_ptr() as _, 2);
    if n != 2 { if BIO_should_retry(b) != 0 || n >= 0 { exit(1); } exit(1); }
    if BIO_reset(b) != 0 { exit(1); }
    let x = if buf[0] == 0x30 && (0x81..=0x84).contains(&buf[1]) {
        d2i_X509_bio(b, ptr::null_mut())
    } else {
        PEM_read_bio_X509(b, ptr::null_mut(), ptr::null_mut(), ptr::null_mut())
    };
    BIO_free(b);
    if x.is_null() { exit(1); }
    x
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut sig_info = module_signature { id_type: MODULE_SIGNATURE_TYPE_PKCS7, _pad: [0; 3], sig_len: 0 };
    let mut args = argv;
    let mut count = argc;
    let mut raw_sig = false;
    let mut save_sig = false;
    let mut sign_only = false;
    let mut use_keyid = 0u32;
    OpenSSL_add_all_algorithms(); ERR_load_crypto_strings(); ERR_clear_error();
    key_pass = getenv(b"KBUILD_SIGN_PIN\0".as_ptr() as _);
    loop {
        let opt = getopt(count, args, b"sdpk\0".as_ptr() as _);
        match opt { 115 => raw_sig = true, 112 => save_sig = true, 100 => { sign_only = true; save_sig = true; }, 107 => use_keyid = 0x10, -1 => break, _ => format() }
    }
    count -= optind; args = args.add(optind as usize);
    if count < 4 || count > 5 { format(); }
    let (raw_sig_name, hash_algo, private_key_name) = if raw_sig { (*args, *args.add(1), ptr::null_mut()) } else { (ptr::null_mut(), *args, *args.add(1)) };
    let x509_name = *args.add(2); let module_name = *args.add(3);
    let (dest_name, replace_orig) = if count == 5 && strcmp(module_name, *args.add(4)) != 0 { (*args.add(4), false) } else {
        let mut p = ptr::null_mut(); asprintf(&mut p, b"%s.~signed~\0".as_ptr() as _, module_name); (p, true)
    };
    let bm = BIO_new_file(module_name, b"rb\0".as_ptr() as _); if bm.is_null() { exit(1); }
    let mut cms = ptr::null_mut();
    if !raw_sig { let key = read_private_key(private_key_name); let cert = read_x509(x509_name); OpenSSL_add_all_digests(); let md = EVP_get_digestbyname(hash_algo); if md.is_null() { exit(1); }
        let flags = 0x800 | 0x100 | 0x80 | 0x40 | 0x20 | 0x400 | 0x200 | use_keyid;
        cms = CMS_sign(ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), flags); if cms.is_null() { exit(1); }
        if CMS_add1_signer(cms, cert, key, md, flags).is_null() || CMS_final(cms, bm, ptr::null_mut(), flags) != 1 { exit(1); }
        if save_sig { let mut p = ptr::null_mut(); asprintf(&mut p, b"%s.p7s\0".as_ptr() as _, module_name); let b = BIO_new_file(p, b"wb\0".as_ptr() as _); if b.is_null() || i2d_CMS_bio_stream(b, cms, ptr::null_mut(), 0) != 1 { exit(1); } BIO_free(b); }
        if sign_only { BIO_free(bm); return 0; }
    }
    let bd = BIO_new_file(dest_name, b"wb\0".as_ptr() as _); if bd.is_null() || BIO_reset(bm) < 0 { exit(1); }
    let mut buf = [0u8; 4096]; let mut n;
    loop { n = BIO_read(bm, buf.as_mut_ptr() as _, 4096); if n <= 0 { break; } if BIO_write(bd, buf.as_ptr() as _, n) < 0 { exit(1); } }
    BIO_free(bm); if n < 0 { exit(1); } let module_size = BIO_number_written(bd);
    if !raw_sig { if i2d_CMS_bio_stream(bd, cms, ptr::null_mut(), 0) != 1 { exit(1); } }
    else { let b = BIO_new_file(raw_sig_name, b"rb\0".as_ptr() as _); if b.is_null() { exit(1); } loop { n = BIO_read(b, buf.as_mut_ptr() as _, 4096); if n <= 0 { break; } if BIO_write(bd, buf.as_ptr() as _, n) < 0 { exit(1); } } BIO_free(b); }
    sig_info.sig_len = htonl((BIO_number_written(bd) - module_size) as u32);
    if BIO_write(bd, &sig_info as *const _ as _, std::mem::size_of::<module_signature>() as c_int) < 0 || BIO_write(bd, MODULE_SIGNATURE_MARKER.as_ptr() as _, 0) < 0 { exit(1); }
    if BIO_free(bd) != 1 { exit(1); } if replace_orig && rename(dest_name, module_name) < 0 { exit(1); } 0
}

fn main() { unsafe { std::process::exit(main_impl(std::env::args().count() as c_int, ptr::null_mut())); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
