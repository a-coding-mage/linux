// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/*
 * Copyright (C) 2025 Google LLC.
 */

use core::ffi::{c_char, c_int, c_long, c_uchar, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const OPEN_SSL_ERR_BUF_LEN: usize = 256;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENOSPC: c_int = 28;

const ERR_TXT_STRING: c_int = 0x02;
const KEY_SPEC_SESSION_KEYRING: c_int = -3;

#[cfg(target_arch = "x86_64")]
const __NR_add_key: c_long = 248;
#[cfg(target_arch = "aarch64")]
const __NR_add_key: c_long = 217;

#[repr(C)]
pub struct BIO {
    _private: [u8; 0],
}

#[repr(C)]
pub struct EVP_PKEY {
    _private: [u8; 0],
}

#[repr(C)]
pub struct X509 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CMS_ContentInfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct EVP_MD {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_load_and_run_opts {
    pub insns: *const c_void,
    pub insns_sz: usize,
    pub data: *const c_void,
    pub data_sz: usize,
    pub excl_prog_hash: *mut c_uchar,
    pub excl_prog_hash_sz: c_uint,
    pub signature: *mut c_void,
    pub signature_sz: usize,
}

#[allow(non_camel_case_types)]
type c_uint = u32;

unsafe extern "C" {
    static mut private_key_path: *const c_char;
    static mut cert_path: *const c_char;

    static CMS_NOCERTS: c_int;
    static CMS_PARTIAL: c_int;
    static CMS_BINARY: c_int;
    static CMS_DETACHED: c_int;
    static CMS_STREAM: c_int;
    static CMS_NOSMIMECAP: c_int;
    static CMS_USE_KEYID: c_int;
    static CMS_NOATTR: c_int;

    fn p_err(fmt: *const c_char, ...);

    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn syscall(num: c_long, ...) -> c_long;

    fn BIO_new_file(filename: *const c_char, mode: *const c_char) -> *mut BIO;
    fn BIO_new_mem_buf(buf: *const c_void, len: c_int) -> *mut BIO;
    fn BIO_new(type_: *const c_void) -> *mut BIO;
    fn BIO_s_mem() -> *const c_void;
    fn BIO_free(a: *mut BIO) -> c_int;
    fn BIO_read(b: *mut BIO, data: *mut c_void, dlen: c_int) -> c_int;
    fn BIO_reset(b: *mut BIO) -> c_int;
    fn BIO_get_mem_data(b: *mut BIO, pp: *mut *mut c_char) -> c_long;

    fn PEM_read_bio_PrivateKey(
        bp: *mut BIO,
        x: *mut *mut EVP_PKEY,
        cb: *mut c_void,
        u: *mut c_void,
    ) -> *mut EVP_PKEY;
    fn PEM_read_bio_X509(
        bp: *mut BIO,
        x: *mut *mut X509,
        cb: *mut c_void,
        u: *mut c_void,
    ) -> *mut X509;
    fn d2i_X509_bio(bp: *mut BIO, x: *mut *mut X509) -> *mut X509;
    fn i2d_X509(a: *mut X509, out: *mut *mut c_uchar) -> c_int;
    fn X509_free(a: *mut X509);

    fn EVP_sha256() -> *const EVP_MD;
    fn EVP_Digest(
        data: *const c_void,
        count: usize,
        md: *mut c_uchar,
        size: *mut c_uint,
        type_: *const EVP_MD,
        impl_: *mut c_void,
    ) -> c_int;
    fn EVP_PKEY_free(key: *mut EVP_PKEY);

    fn CMS_sign(
        signcert: *mut X509,
        pkey: *mut EVP_PKEY,
        certs: *mut c_void,
        data: *mut BIO,
        flags: c_uint,
    ) -> *mut CMS_ContentInfo;
    fn CMS_add1_signer(
        cms: *mut CMS_ContentInfo,
        signcert: *mut X509,
        pkey: *mut EVP_PKEY,
        md: *const EVP_MD,
        flags: c_uint,
    ) -> *mut c_void;
    fn CMS_final(cms: *mut CMS_ContentInfo, data: *mut BIO, dcont: *mut BIO, flags: c_uint) -> c_int;
    fn i2d_CMS_bio_stream(out: *mut BIO, cms: *mut CMS_ContentInfo, in_: *mut BIO, flags: c_int) -> c_int;
    fn CMS_ContentInfo_free(cms: *mut CMS_ContentInfo);

    fn OPENSSL_free(addr: *mut c_void);
    fn ERR_get_error_all(
        file: *mut *const c_char,
        line: *mut c_int,
        func: *mut *const c_char,
        data: *mut *const c_char,
        flags: *mut c_int,
    ) -> c_ulong;
    fn ERR_error_string_n(e: c_ulong, buf: *mut c_char, len: usize);
    fn ERR_peek_error() -> c_ulong;
}

unsafe fn display_openssl_errors(_l: c_int) {
    let mut buf = [0 as c_char; OPEN_SSL_ERR_BUF_LEN];
    let mut file: *const c_char = ptr::null();
    let mut data: *const c_char = ptr::null();
    let mut flags: c_int = 0;
    let mut line: c_int = 0;

    loop {
        let e = unsafe {
            ERR_get_error_all(
                &mut file,
                &mut line,
                ptr::null_mut(),
                &mut data,
                &mut flags,
            )
        };
        if e == 0 {
            break;
        }
        unsafe { ERR_error_string_n(e, buf.as_mut_ptr(), size_of::<[c_char; OPEN_SSL_ERR_BUF_LEN]>()) };
        if !data.is_null() && (flags & ERR_TXT_STRING) != 0 {
            unsafe {
                p_err(
                    c"OpenSSL %s: %s:%d: %s".as_ptr(),
                    buf.as_ptr(),
                    file,
                    line,
                    data,
                )
            };
        } else {
            unsafe {
                p_err(
                    c"OpenSSL %s: %s:%d".as_ptr(),
                    buf.as_ptr(),
                    file,
                    line,
                )
            };
        }
    }
}

macro_rules! DISPLAY_OSSL_ERR {
    ($cond:expr) => {{
        let __cond: bool = $cond;
        if __cond && unsafe { ERR_peek_error() } != 0 {
            unsafe { display_openssl_errors(line!() as c_int) };
        }
    }};
}

macro_rules! goto_cleanup {
    ($bd_out:expr, $cms:expr, $x509:expr, $private_key:expr, $bd_in:expr, $data:expr, $err:expr) => {{
        unsafe { BIO_free($bd_out) };
        unsafe { CMS_ContentInfo_free($cms) };
        unsafe { X509_free($x509) };
        unsafe { EVP_PKEY_free($private_key) };
        unsafe { BIO_free($bd_in) };
        unsafe { free($data) };
        DISPLAY_OSSL_ERR!($err < 0);
        return $err;
    }};
}

unsafe fn read_private_key(pkey_path: *const c_char) -> *mut EVP_PKEY {
    let mut private_key: *mut EVP_PKEY = ptr::null_mut();
    let b: *mut BIO;

    b = unsafe { BIO_new_file(pkey_path, c"rb".as_ptr()) };
    private_key = unsafe {
        PEM_read_bio_PrivateKey(b, ptr::null_mut(), ptr::null_mut(), ptr::null_mut())
    };
    unsafe { BIO_free(b) };
    DISPLAY_OSSL_ERR!(private_key.is_null());
    private_key
}

unsafe fn read_x509(x509_name: *const c_char) -> *mut X509 {
    let mut buf = [0u8; 2];
    let mut x509: *mut X509 = ptr::null_mut();
    let b: *mut BIO;
    let n: c_int;

    b = unsafe { BIO_new_file(x509_name, c"rb".as_ptr()) };
    if b.is_null() {
        unsafe { BIO_free(b) };
        DISPLAY_OSSL_ERR!(x509.is_null());
        return x509;
    }

    /* Look at the first two bytes of the file to determine the encoding */
    n = unsafe { BIO_read(b, buf.as_mut_ptr() as *mut c_void, 2) };
    if n != 2 {
        unsafe { BIO_free(b) };
        DISPLAY_OSSL_ERR!(x509.is_null());
        return x509;
    }

    if unsafe { BIO_reset(b) } != 0 {
        unsafe { BIO_free(b) };
        DISPLAY_OSSL_ERR!(x509.is_null());
        return x509;
    }

    if buf[0] == 0x30 && buf[1] >= 0x81 && buf[1] <= 0x84 {
        /* Assume raw DER encoded X.509 */
        x509 = unsafe { d2i_X509_bio(b, ptr::null_mut()) };
    } else {
        /* Assume PEM encoded X.509 */
        x509 = unsafe { PEM_read_bio_X509(b, ptr::null_mut(), ptr::null_mut(), ptr::null_mut()) };
    }

    unsafe { BIO_free(b) };
    DISPLAY_OSSL_ERR!(x509.is_null());
    x509
}

#[no_mangle]
pub unsafe extern "C" fn register_session_key(key_der_path: *const c_char) -> u32 {
    let mut der_buf: *mut c_uchar = ptr::null_mut();
    let mut x509: *mut X509 = ptr::null_mut();
    let mut key_id: c_int = -1;
    let der_len: c_int;

    if key_der_path.is_null() {
        return key_id as u32;
    }
    x509 = unsafe { read_x509(key_der_path) };
    if x509.is_null() {
        unsafe { X509_free(x509) };
        unsafe { OPENSSL_free(der_buf as *mut c_void) };
        DISPLAY_OSSL_ERR!(key_id == -1);
        return key_id as u32;
    }
    der_len = unsafe { i2d_X509(x509, &mut der_buf) };
    if der_len < 0 {
        unsafe { X509_free(x509) };
        unsafe { OPENSSL_free(der_buf as *mut c_void) };
        DISPLAY_OSSL_ERR!(key_id == -1);
        return key_id as u32;
    }
    key_id = unsafe {
        syscall(
            __NR_add_key,
            c"asymmetric".as_ptr(),
            key_der_path,
            der_buf,
            der_len as usize,
            KEY_SPEC_SESSION_KEYRING,
        ) as c_int
    };

    unsafe { X509_free(x509) };
    unsafe { OPENSSL_free(der_buf as *mut c_void) };
    DISPLAY_OSSL_ERR!(key_id == -1);
    key_id as u32
}

#[no_mangle]
pub unsafe extern "C" fn bpftool_prog_sign(opts: *mut bpf_load_and_run_opts) -> c_int {
    let mut bd_in: *mut BIO = ptr::null_mut();
    let mut bd_out: *mut BIO = ptr::null_mut();
    let mut private_key: *mut EVP_PKEY = ptr::null_mut();
    let mut cms: *mut CMS_ContentInfo = ptr::null_mut();
    let mut actual_sig_len: c_long = 0;
    let mut x509: *mut X509 = ptr::null_mut();
    let mut data: *mut c_void = ptr::null_mut();
    let data_sz: usize;
    let mut err: c_int = 0;

    data_sz = unsafe { (*opts).insns_sz + (*opts).data_sz };
    data = unsafe { malloc(data_sz) };
    if data.is_null() {
        err = -ENOMEM;
        goto_cleanup!(bd_out, cms, x509, private_key, bd_in, data, err);
    }
    unsafe { memcpy(data, (*opts).insns, (*opts).insns_sz) };
    if unsafe { (*opts).data_sz } != 0 {
        unsafe {
            memcpy(
                (data as *mut c_char).add((*opts).insns_sz) as *mut c_void,
                (*opts).data,
                (*opts).data_sz,
            )
        };
    }

    bd_in = unsafe { BIO_new_mem_buf(data, data_sz as c_int) };
    if bd_in.is_null() {
        err = -ENOMEM;
        goto_cleanup!(bd_out, cms, x509, private_key, bd_in, data, err);
    }

    private_key = unsafe { read_private_key(private_key_path) };
    if private_key.is_null() {
        err = -EINVAL;
        goto_cleanup!(bd_out, cms, x509, private_key, bd_in, data, err);
    }

    x509 = unsafe { read_x509(cert_path) };
    if x509.is_null() {
        err = -EINVAL;
        goto_cleanup!(bd_out, cms, x509, private_key, bd_in, data, err);
    }

    cms = unsafe {
        CMS_sign(
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            (CMS_NOCERTS | CMS_PARTIAL | CMS_BINARY | CMS_DETACHED | CMS_STREAM) as c_uint,
        )
    };
    if cms.is_null() {
        err = -EINVAL;
        goto_cleanup!(bd_out, cms, x509, private_key, bd_in, data, err);
    }

    if unsafe {
        CMS_add1_signer(
            cms,
            x509,
            private_key,
            EVP_sha256(),
            (CMS_NOCERTS | CMS_BINARY | CMS_NOSMIMECAP | CMS_USE_KEYID | CMS_NOATTR) as c_uint,
        )
    }
    .is_null()
    {
        err = -EINVAL;
        goto_cleanup!(bd_out, cms, x509, private_key, bd_in, data, err);
    }

    if unsafe { CMS_final(cms, bd_in, ptr::null_mut(), (CMS_NOCERTS | CMS_BINARY) as c_uint) } != 1 {
        err = -EIO;
        goto_cleanup!(bd_out, cms, x509, private_key, bd_in, data, err);
    }

    if unsafe {
        EVP_Digest(
            (*opts).insns,
            (*opts).insns_sz,
            (*opts).excl_prog_hash,
            &mut (*opts).excl_prog_hash_sz,
            EVP_sha256(),
            ptr::null_mut(),
        )
    } != 1
    {
        err = -EIO;
        goto_cleanup!(bd_out, cms, x509, private_key, bd_in, data, err);
    }

    bd_out = unsafe { BIO_new(BIO_s_mem()) };
    if bd_out.is_null() {
        err = -ENOMEM;
        goto_cleanup!(bd_out, cms, x509, private_key, bd_in, data, err);
    }

    if unsafe { i2d_CMS_bio_stream(bd_out, cms, ptr::null_mut(), 0) } == 0 {
        err = -EIO;
        goto_cleanup!(bd_out, cms, x509, private_key, bd_in, data, err);
    }

    actual_sig_len = unsafe { BIO_get_mem_data(bd_out, ptr::null_mut()) };
    if actual_sig_len <= 0 {
        err = -EIO;
        goto_cleanup!(bd_out, cms, x509, private_key, bd_in, data, err);
    }

    if actual_sig_len as usize > unsafe { (*opts).signature_sz } {
        err = -ENOSPC;
        goto_cleanup!(bd_out, cms, x509, private_key, bd_in, data, err);
    }

    if unsafe { BIO_read(bd_out, (*opts).signature, actual_sig_len as c_int) } != actual_sig_len as c_int {
        err = -EIO;
        goto_cleanup!(bd_out, cms, x509, private_key, bd_in, data, err);
    }

    unsafe { (*opts).signature_sz = actual_sig_len as usize };
    goto_cleanup!(bd_out, cms, x509, private_key, bd_in, data, err);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
