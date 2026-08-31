// SPDX-License-Identifier: GPL-2.0
/*  Copyright(c) 2016-20 Intel Corporation. */

/*
 * C dependencies intentionally left external:
 * assert.h, getopt.h, stdbool.h, stdint.h, stdio.h, stdlib.h, string.h,
 * sys/stat.h, sys/types.h, unistd.h, openssl/err.h, openssl/pem.h,
 * "defines.h", and "main.h".
 *
 * FIXME: OpenSSL 3.0 has deprecated some functions. For now just ignore
 * the warnings.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

extern "C" {
    static sign_key: c_void;
    static sign_key_end: c_void;

    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;

    fn BN_CTX_new() -> *mut BN_CTX;
    fn BN_CTX_free(c: *mut BN_CTX);
    fn BN_bin2bn(s: *const u8, len: c_int, ret: *mut BIGNUM) -> *mut BIGNUM;
    fn BN_new() -> *mut BIGNUM;
    fn BN_free(a: *mut BIGNUM);
    fn BN_mul(r: *mut BIGNUM, a: *const BIGNUM, b: *const BIGNUM, ctx: *mut BN_CTX) -> c_int;
    fn BN_div(
        dv: *mut BIGNUM,
        rem: *mut BIGNUM,
        m: *const BIGNUM,
        d: *const BIGNUM,
        ctx: *mut BN_CTX,
    ) -> c_int;
    fn BN_num_bytes(a: *const BIGNUM) -> c_int;
    fn BN_bn2bin(a: *const BIGNUM, to: *mut u8) -> c_int;

    fn ERR_peek_error() -> c_ulong;
    fn ERR_get_error_line(file: *mut *const c_char, line: *mut c_int) -> c_ulong;
    fn ERR_error_string_n(e: c_ulong, buf: *mut c_char, len: usize);
    fn ERR_print_errors_fp(fp: *mut FILE);

    fn RSA_get0_key(r: *const RSA, n: *mut *const BIGNUM, e: *mut *const BIGNUM, d: *mut *const BIGNUM);
    fn RSA_free(r: *mut RSA);
    fn RSA_sign(
        type_: c_int,
        m: *const u8,
        m_len: c_uint,
        sigret: *mut u8,
        siglen: *mut c_uint,
        rsa: *mut RSA,
    ) -> c_int;

    fn BIO_new_mem_buf(buf: *const c_void, len: c_int) -> *mut BIO;
    fn BIO_free(a: *mut BIO) -> c_int;
    fn PEM_read_bio_RSAPrivateKey(
        bp: *mut BIO,
        x: *mut *mut RSA,
        cb: *mut c_void,
        u: *mut c_void,
    ) -> *mut RSA;

    fn EVP_MD_CTX_create() -> *mut EVP_MD_CTX;
    fn EVP_MD_CTX_destroy(ctx: *mut EVP_MD_CTX);
    fn EVP_sha256() -> *const EVP_MD;
    fn EVP_DigestInit_ex(ctx: *mut EVP_MD_CTX, type_: *const EVP_MD, impl_: *mut c_void) -> c_int;
    fn EVP_DigestUpdate(ctx: *mut EVP_MD_CTX, d: *const c_void, cnt: usize) -> c_int;
    fn EVP_DigestFinal_ex(ctx: *mut EVP_MD_CTX, md: *mut u8, s: *mut c_uint) -> c_int;
    fn SHA256(d: *const u8, n: usize, md: *mut u8) -> *mut u8;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct BN_CTX {
    _private: [u8; 0],
}

#[repr(C)]
pub struct BIGNUM {
    _private: [u8; 0],
}

#[repr(C)]
pub struct RSA {
    _private: [u8; 0],
}

#[repr(C)]
pub struct BIO {
    _private: [u8; 0],
}

#[repr(C)]
pub struct EVP_MD_CTX {
    _private: [u8; 0],
}

#[repr(C)]
pub struct EVP_MD {
    _private: [u8; 0],
}

extern "C" {
    static SGX_MODULUS_SIZE: usize;
    static SHA256_DIGEST_LENGTH: usize;
    static PAGE_SIZE: u64;
    static SGX_ATTR_MODE64BIT: u64;
    static NID_sha256: c_int;
}

#[repr(C)]
pub struct sgx_sigstruct_header {
    pub header1: [u64; 2],
    pub header2: [u64; 2],
}

#[repr(C)]
pub struct sgx_sigstruct_body {
    pub attributes: u64,
    pub xfrm: u64,
    pub mrenclave: [u8; 32],
}

#[repr(C)]
pub struct sgx_sigstruct {
    pub header: sgx_sigstruct_header,
    pub exponent: u32,
    pub modulus: *mut u8,
    pub body: sgx_sigstruct_body,
    pub signature: *mut u8,
    pub q1: *mut u8,
    pub q2: *mut u8,
}

#[repr(C)]
pub struct encl_segment {
    pub size: u64,
    pub offset: u64,
    pub flags: u64,
    pub measure: bool,
    pub src: *const u8,
}

#[repr(C)]
pub struct encl {
    pub sigstruct: sgx_sigstruct,
    pub src_size: u64,
    pub nr_segments: c_int,
    pub segment_tbl: *mut encl_segment,
}

#[repr(C)]
struct q1q2_ctx {
    bn_ctx: *mut BN_CTX,
    m: *mut BIGNUM,
    s: *mut BIGNUM,
    q1: *mut BIGNUM,
    qr: *mut BIGNUM,
    q2: *mut BIGNUM,
}

unsafe fn free_q1q2_ctx(ctx: *mut q1q2_ctx) {
    BN_CTX_free((*ctx).bn_ctx);
    BN_free((*ctx).m);
    BN_free((*ctx).s);
    BN_free((*ctx).q1);
    BN_free((*ctx).qr);
    BN_free((*ctx).q2);
}

unsafe fn alloc_q1q2_ctx(s: *const u8, m: *const u8, ctx: *mut q1q2_ctx) -> bool {
    (*ctx).bn_ctx = BN_CTX_new();
    (*ctx).s = BN_bin2bn(s, SGX_MODULUS_SIZE as c_int, ptr::null_mut());
    (*ctx).m = BN_bin2bn(m, SGX_MODULUS_SIZE as c_int, ptr::null_mut());
    (*ctx).q1 = BN_new();
    (*ctx).qr = BN_new();
    (*ctx).q2 = BN_new();

    if (*ctx).bn_ctx.is_null()
        || (*ctx).s.is_null()
        || (*ctx).m.is_null()
        || (*ctx).q1.is_null()
        || (*ctx).qr.is_null()
        || (*ctx).q2.is_null()
    {
        free_q1q2_ctx(ctx);
        return false;
    }

    true
}

unsafe fn reverse_bytes(data: *mut c_void, length: c_int) {
    let mut i: c_int = 0;
    let mut j: c_int = length - 1;
    let mut temp: u8;
    let ptr = data as *mut u8;

    while i < j {
        temp = *ptr.offset(i as isize);
        *ptr.offset(i as isize) = *ptr.offset(j as isize);
        *ptr.offset(j as isize) = temp;
        i += 1;
        j -= 1;
    }
}

unsafe fn calc_q1q2(s: *const u8, m: *const u8, q1: *mut u8, q2: *mut u8) -> bool {
    let mut ctx: q1q2_ctx = mem::zeroed();
    let mut len: c_int;

    if !alloc_q1q2_ctx(s, m, &mut ctx) {
        fprintf(stderr, b"Not enough memory for Q1Q2 calculation\n\0".as_ptr() as *const c_char);
        return false;
    }

    if BN_mul(ctx.q1, ctx.s, ctx.s, ctx.bn_ctx) == 0 {
        free_q1q2_ctx(&mut ctx);
        return false;
    }

    if BN_div(ctx.q1, ctx.qr, ctx.q1, ctx.m, ctx.bn_ctx) == 0 {
        free_q1q2_ctx(&mut ctx);
        return false;
    }

    if BN_num_bytes(ctx.q1) > SGX_MODULUS_SIZE as c_int {
        fprintf(
            stderr,
            b"Too large Q1 %d bytes\n\0".as_ptr() as *const c_char,
            BN_num_bytes(ctx.q1),
        );
        free_q1q2_ctx(&mut ctx);
        return false;
    }

    if BN_mul(ctx.q2, ctx.s, ctx.qr, ctx.bn_ctx) == 0 {
        free_q1q2_ctx(&mut ctx);
        return false;
    }

    if BN_div(ctx.q2, ptr::null_mut(), ctx.q2, ctx.m, ctx.bn_ctx) == 0 {
        free_q1q2_ctx(&mut ctx);
        return false;
    }

    if BN_num_bytes(ctx.q2) > SGX_MODULUS_SIZE as c_int {
        fprintf(
            stderr,
            b"Too large Q2 %d bytes\n\0".as_ptr() as *const c_char,
            BN_num_bytes(ctx.q2),
        );
        free_q1q2_ctx(&mut ctx);
        return false;
    }

    len = BN_bn2bin(ctx.q1, q1);
    reverse_bytes(q1 as *mut c_void, len);
    len = BN_bn2bin(ctx.q2, q2);
    reverse_bytes(q2 as *mut c_void, len);

    free_q1q2_ctx(&mut ctx);
    true
}

#[repr(C)]
struct sgx_sigstruct_payload {
    header: sgx_sigstruct_header,
    body: sgx_sigstruct_body,
}

unsafe fn check_crypto_errors() -> bool {
    let mut err: c_ulong;
    let mut had_errors = false;
    let mut filename: *const c_char = ptr::null();
    let mut line: c_int = 0;
    let mut str_: [c_char; 256] = [0; 256];

    loop {
        if ERR_peek_error() == 0 {
            break;
        }

        had_errors = true;
        err = ERR_get_error_line(&mut filename, &mut line);
        ERR_error_string_n(err, str_.as_mut_ptr(), mem::size_of_val(&str_));
        fprintf(
            stderr,
            b"crypto: %s: %s:%d\n\0".as_ptr() as *const c_char,
            str_.as_ptr(),
            filename,
            line,
        );
    }

    had_errors
}

unsafe fn get_modulus(key: *mut RSA) -> *const BIGNUM {
    let mut n: *const BIGNUM = ptr::null();

    RSA_get0_key(key, &mut n, ptr::null_mut(), ptr::null_mut());
    n
}

unsafe fn gen_sign_key() -> *mut RSA {
    let sign_key_length: c_ulong;
    let bio: *mut BIO;
    let key: *mut RSA;

    sign_key_length = (&sign_key_end as *const c_void as c_ulong).wrapping_sub(&sign_key as *const c_void as c_ulong);

    bio = BIO_new_mem_buf(&sign_key as *const c_void, sign_key_length as c_int);
    if bio.is_null() {
        return ptr::null_mut();
    }

    key = PEM_read_bio_RSAPrivateKey(bio, ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
    BIO_free(bio);

    key
}

#[repr(u64)]
enum mrtags {
    MRECREATE = 0x0045544145524345,
    MREADD = 0x0000000044444145,
    MREEXTEND = 0x00444E4554584545,
}

unsafe fn mrenclave_update(ctx: *mut EVP_MD_CTX, data: *const c_void) -> bool {
    if EVP_DigestUpdate(ctx, data, 64) == 0 {
        fprintf(stderr, b"digest update failed\n\0".as_ptr() as *const c_char);
        return false;
    }

    true
}

unsafe fn mrenclave_commit(ctx: *mut EVP_MD_CTX, mrenclave: *mut u8) -> bool {
    let mut size: c_uint = 0;

    if EVP_DigestFinal_ex(ctx, mrenclave as *mut u8, &mut size) == 0 {
        fprintf(stderr, b"digest commit failed\n\0".as_ptr() as *const c_char);
        return false;
    }

    if size != 32 {
        fprintf(
            stderr,
            b"invalid digest size = %u\n\0".as_ptr() as *const c_char,
            size,
        );
        return false;
    }

    true
}

#[repr(C, packed)]
struct mrecreate {
    tag: u64,
    ssaframesize: u32,
    size: u64,
    reserved: [u8; 44],
}

unsafe fn mrenclave_ecreate(ctx: *mut EVP_MD_CTX, blob_size: u64) -> bool {
    let mut mrecreate: mrecreate = mem::zeroed();
    let mut encl_size: u64;

    encl_size = 0x1000;
    while encl_size < blob_size {
        encl_size <<= 1;
    }

    memset(
        &mut mrecreate as *mut mrecreate as *mut c_void,
        0,
        mem::size_of_val(&mrecreate),
    );
    mrecreate.tag = mrtags::MRECREATE as u64;
    mrecreate.ssaframesize = 1;
    mrecreate.size = encl_size;

    if EVP_DigestInit_ex(ctx, EVP_sha256(), ptr::null_mut()) == 0 {
        return false;
    }

    mrenclave_update(ctx, &mrecreate as *const mrecreate as *const c_void)
}

#[repr(C, packed)]
struct mreadd {
    tag: u64,
    offset: u64,
    flags: u64, /* SECINFO flags */
    reserved: [u8; 40],
}

unsafe fn mrenclave_eadd(ctx: *mut EVP_MD_CTX, offset: u64, flags: u64) -> bool {
    let mut mreadd: mreadd = mem::zeroed();

    memset(
        &mut mreadd as *mut mreadd as *mut c_void,
        0,
        mem::size_of_val(&mreadd),
    );
    mreadd.tag = mrtags::MREADD as u64;
    mreadd.offset = offset;
    mreadd.flags = flags;

    mrenclave_update(ctx, &mreadd as *const mreadd as *const c_void)
}

#[repr(C, packed)]
struct mreextend {
    tag: u64,
    offset: u64,
    reserved: [u8; 48],
}

unsafe fn mrenclave_eextend(ctx: *mut EVP_MD_CTX, offset: u64, data: *const u8) -> bool {
    let mut mreextend: mreextend = mem::zeroed();
    let mut i: c_int;

    i = 0;
    while i < 0x1000 {
        memset(
            &mut mreextend as *mut mreextend as *mut c_void,
            0,
            mem::size_of_val(&mreextend),
        );
        mreextend.tag = mrtags::MREEXTEND as u64;
        mreextend.offset = offset + i as u64;

        if !mrenclave_update(ctx, &mreextend as *const mreextend as *const c_void) {
            return false;
        }

        if !mrenclave_update(ctx, data.offset((i + 0x00) as isize) as *const c_void) {
            return false;
        }

        if !mrenclave_update(ctx, data.offset((i + 0x40) as isize) as *const c_void) {
            return false;
        }

        if !mrenclave_update(ctx, data.offset((i + 0x80) as isize) as *const c_void) {
            return false;
        }

        if !mrenclave_update(ctx, data.offset((i + 0xC0) as isize) as *const c_void) {
            return false;
        }

        i += 0x100;
    }

    true
}

unsafe fn mrenclave_segment(ctx: *mut EVP_MD_CTX, encl: *mut encl, seg: *mut encl_segment) -> bool {
    let end: u64 = (*seg).size;
    let mut offset: u64;

    offset = 0;
    while offset < end {
        if !mrenclave_eadd(ctx, (*seg).offset + offset, (*seg).flags) {
            return false;
        }

        if (*seg).measure {
            if !mrenclave_eextend(ctx, (*seg).offset + offset, (*seg).src.offset(offset as isize)) {
                return false;
            }
        }

        offset += PAGE_SIZE;
    }

    true
}

#[no_mangle]
pub unsafe extern "C" fn encl_measure(encl: *mut encl) -> bool {
    let header1: [u64; 2] = [0x000000E100000006, 0x0000000000010000];
    let header2: [u64; 2] = [0x0000006000000101, 0x0000000100000060];
    let sigstruct: *mut sgx_sigstruct = &mut (*encl).sigstruct;
    let mut payload: sgx_sigstruct_payload = mem::zeroed();
    let mut digest: [u8; 32] = [0; 32];
    let mut ctx: *mut EVP_MD_CTX = ptr::null_mut();
    let mut siglen: c_uint = 0;
    let mut key: *mut RSA = ptr::null_mut();
    let mut i: c_int;

    memset(
        sigstruct as *mut c_void,
        0,
        mem::size_of::<sgx_sigstruct>(),
    );

    (*sigstruct).header.header1[0] = header1[0];
    (*sigstruct).header.header1[1] = header1[1];
    (*sigstruct).header.header2[0] = header2[0];
    (*sigstruct).header.header2[1] = header2[1];
    (*sigstruct).exponent = 3;
    (*sigstruct).body.attributes = SGX_ATTR_MODE64BIT;
    (*sigstruct).body.xfrm = 3;

    /* sanity check */
    if check_crypto_errors() {
        RSA_free(key);
        return false;
    }

    key = gen_sign_key();
    if key.is_null() {
        ERR_print_errors_fp(stdout);
        RSA_free(key);
        return false;
    }

    BN_bn2bin(get_modulus(key), (*sigstruct).modulus);

    ctx = EVP_MD_CTX_create();
    if ctx.is_null() {
        RSA_free(key);
        return false;
    }

    if !mrenclave_ecreate(ctx, (*encl).src_size) {
        EVP_MD_CTX_destroy(ctx);
        RSA_free(key);
        return false;
    }

    i = 0;
    while i < (*encl).nr_segments {
        let seg: *mut encl_segment = (*encl).segment_tbl.offset(i as isize);

        if !mrenclave_segment(ctx, encl, seg) {
            EVP_MD_CTX_destroy(ctx);
            RSA_free(key);
            return false;
        }

        i += 1;
    }

    if !mrenclave_commit(ctx, (*sigstruct).body.mrenclave.as_mut_ptr()) {
        EVP_MD_CTX_destroy(ctx);
        RSA_free(key);
        return false;
    }

    memcpy(
        &mut payload.header as *mut sgx_sigstruct_header as *mut c_void,
        &(*sigstruct).header as *const sgx_sigstruct_header as *const c_void,
        mem::size_of_val(&(*sigstruct).header),
    );
    memcpy(
        &mut payload.body as *mut sgx_sigstruct_body as *mut c_void,
        &(*sigstruct).body as *const sgx_sigstruct_body as *const c_void,
        mem::size_of_val(&(*sigstruct).body),
    );

    SHA256(
        &payload as *const sgx_sigstruct_payload as *const u8,
        mem::size_of_val(&payload),
        digest.as_mut_ptr(),
    );

    if RSA_sign(
        NID_sha256,
        digest.as_ptr(),
        SHA256_DIGEST_LENGTH as c_uint,
        (*sigstruct).signature,
        &mut siglen,
        key,
    ) == 0
    {
        EVP_MD_CTX_destroy(ctx);
        RSA_free(key);
        return false;
    }

    if !calc_q1q2(
        (*sigstruct).signature,
        (*sigstruct).modulus,
        (*sigstruct).q1,
        (*sigstruct).q2,
    ) {
        EVP_MD_CTX_destroy(ctx);
        RSA_free(key);
        return false;
    }

    /* BE -> LE */
    reverse_bytes((*sigstruct).signature as *mut c_void, SGX_MODULUS_SIZE as c_int);
    reverse_bytes((*sigstruct).modulus as *mut c_void, SGX_MODULUS_SIZE as c_int);

    EVP_MD_CTX_destroy(ctx);
    RSA_free(key);
    true
}
