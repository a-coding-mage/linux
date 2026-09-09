// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * RSA Signature Scheme with Appendix - PKCS #1 v1.5 (RFC 8017 sec 8.2)
 *
 * https://www.rfc-editor.org/rfc/rfc8017#section-8.2
 *
 * Copyright (c) 2015 - 2024 Intel Corporation
 */

/* Kernel dependencies supplied by the surrounding translation unit. */

/* Full Hash Prefix for EMSA-PKCS1-v1_5 encoding method (RFC 9580 table 24). */
static HASH_PREFIX_NONE: [u8; 0] = [];
static HASH_PREFIX_MD5: [u8; 18] = [0x30,0x20,0x30,0x0c,0x06,0x08,0x2a,0x86,0x48,0x86,0xf7,0x0d,0x02,0x05,0x05,0x00,0x04,0x10];
static HASH_PREFIX_SHA1: [u8; 15] = [0x30,0x21,0x30,0x09,0x06,0x05,0x2b,0x0e,0x03,0x02,0x1a,0x05,0x00,0x04,0x14];
static HASH_PREFIX_RMD160: [u8; 15] = [0x30,0x21,0x30,0x09,0x06,0x05,0x2b,0x24,0x03,0x02,0x01,0x05,0x00,0x04,0x14];
static HASH_PREFIX_SHA224: [u8; 19] = [0x30,0x2d,0x30,0x0d,0x06,0x09,0x60,0x86,0x48,0x01,0x65,0x03,0x04,0x02,0x04,0x05,0x00,0x04,0x1c];
static HASH_PREFIX_SHA256: [u8; 19] = [0x30,0x31,0x30,0x0d,0x06,0x09,0x60,0x86,0x48,0x01,0x65,0x03,0x04,0x02,0x01,0x05,0x00,0x04,0x20];
static HASH_PREFIX_SHA384: [u8; 19] = [0x30,0x41,0x30,0x0d,0x06,0x09,0x60,0x86,0x48,0x01,0x65,0x03,0x04,0x02,0x02,0x05,0x00,0x04,0x30];
static HASH_PREFIX_SHA512: [u8; 19] = [0x30,0x51,0x30,0x0d,0x06,0x09,0x60,0x86,0x48,0x01,0x65,0x03,0x04,0x02,0x03,0x05,0x00,0x04,0x40];
static HASH_PREFIX_SHA3_256: [u8; 19] = [0x30,0x31,0x30,0x0d,0x06,0x09,0x60,0x86,0x48,0x01,0x65,0x03,0x04,0x02,0x08,0x05,0x00,0x04,0x20];
static HASH_PREFIX_SHA3_384: [u8; 19] = [0x30,0x41,0x30,0x0d,0x06,0x09,0x60,0x86,0x48,0x01,0x65,0x03,0x04,0x02,0x09,0x05,0x00,0x04,0x30];
static HASH_PREFIX_SHA3_512: [u8; 19] = [0x30,0x51,0x30,0x0d,0x06,0x09,0x60,0x86,0x48,0x01,0x65,0x03,0x04,0x02,0x0a,0x05,0x00,0x04,0x40];

#[repr(C)]
struct HashPrefix { name: *const core::ffi::c_char, data: *const u8, size: usize }

static HASH_PREFIXES: [HashPrefix; 11] = [
    HashPrefix { name: b"none\0".as_ptr() as _, data: HASH_PREFIX_NONE.as_ptr(), size: 0 },
    HashPrefix { name: b"md5\0".as_ptr() as _, data: HASH_PREFIX_MD5.as_ptr(), size: 18 },
    HashPrefix { name: b"sha1\0".as_ptr() as _, data: HASH_PREFIX_SHA1.as_ptr(), size: 15 },
    HashPrefix { name: b"rmd160\0".as_ptr() as _, data: HASH_PREFIX_RMD160.as_ptr(), size: 15 },
    HashPrefix { name: b"sha256\0".as_ptr() as _, data: HASH_PREFIX_SHA256.as_ptr(), size: 19 },
    HashPrefix { name: b"sha384\0".as_ptr() as _, data: HASH_PREFIX_SHA384.as_ptr(), size: 19 },
    HashPrefix { name: b"sha512\0".as_ptr() as _, data: HASH_PREFIX_SHA512.as_ptr(), size: 19 },
    HashPrefix { name: b"sha224\0".as_ptr() as _, data: HASH_PREFIX_SHA224.as_ptr(), size: 19 },
    HashPrefix { name: b"sha3-256\0".as_ptr() as _, data: HASH_PREFIX_SHA3_256.as_ptr(), size: 19 },
    HashPrefix { name: b"sha3-384\0".as_ptr() as _, data: HASH_PREFIX_SHA3_384.as_ptr(), size: 19 },
    HashPrefix { name: b"sha3-512\0".as_ptr() as _, data: HASH_PREFIX_SHA3_512.as_ptr(), size: 19 },
];

unsafe fn rsassa_pkcs1_find_hash_prefix(name: *const core::ffi::c_char) -> *const HashPrefix {
    let mut i = 0;
    while i < HASH_PREFIXES.len() {
        if strcmp(name, HASH_PREFIXES[i].name) == 0 { return &HASH_PREFIXES[i]; }
        i += 1;
    }
    core::ptr::null()
}

unsafe fn rsassa_pkcs1_invalid_hash_len(len: u32, p: *const HashPrefix) -> bool {
    if (*p).data == HASH_PREFIX_NONE.as_ptr() { return false; }
    // static_assert(HASH_MAX_DIGESTSIZE <= 127);
    len != *((*p).data.add((*p).size - 1)) as u32
}

#[repr(C)] struct RsassaPkcs1Ctx { child: *mut crypto_akcipher, key_size: u32 }
#[repr(C)] struct RsassaPkcs1InstCtx { spawn: crypto_akcipher_spawn, hash_prefix: *const HashPrefix }

unsafe fn rsassa_pkcs1_sign(tfm: *mut crypto_sig, src: *const core::ffi::c_void, slen: u32, dst: *mut core::ffi::c_void, dlen: u32) -> i32 {
    let ctx = crypto_sig_ctx(tfm) as *mut RsassaPkcs1Ctx;
    let hp = (*(sig_instance_ctx(sig_alg_instance(tfm)) as *mut RsassaPkcs1InstCtx)).hash_prefix;
    if (*ctx).key_size == 0 { return -EINVAL; }
    if dlen < (*ctx).key_size { return -EOVERFLOW; }
    if rsassa_pkcs1_invalid_hash_len(slen, hp) { return -EINVAL; }
    if slen + (*hp).size as u32 > (*ctx).key_size - 11 { return -EOVERFLOW; }
    let pad_len = (*ctx).key_size - slen - (*hp).size as u32 - 1;
    let input = dst as *mut u8;
    core::ptr::copy(src as *const u8, input.add((pad_len as usize) + (*hp).size), slen as usize);
    core::ptr::copy_nonoverlapping((*hp).data, input.add(pad_len as usize), (*hp).size);
    let ps_end = pad_len - 1;
    *input = 1;
    core::ptr::write_bytes(input.add(1), 0xff, (ps_end - 1) as usize);
    *input.add(ps_end as usize) = 0;
    let mut err = crypto_akcipher_sync_decrypt((*ctx).child, input, (*ctx).key_size - 1, input, (*ctx).key_size);
    if err < 0 { return err; }
    let len = err as u32;
    let pad = (*ctx).key_size - len;
    if pad != 0 { core::ptr::copy(dst as *const u8, (dst as *mut u8).add(pad as usize), len as usize); core::ptr::write_bytes(dst as *mut u8, 0, pad as usize); }
    (*ctx).key_size as i32
}

unsafe fn rsassa_pkcs1_verify(tfm: *mut crypto_sig, src: *const core::ffi::c_void, slen: u32, digest: *const core::ffi::c_void, dlen: u32) -> i32 {
    let c = crypto_sig_ctx(tfm) as *mut RsassaPkcs1Ctx;
    let hp = (*(sig_instance_ctx(sig_alg_instance(tfm)) as *mut RsassaPkcs1InstCtx)).hash_prefix;
    if (*c).key_size == 0 || slen != (*c).key_size || rsassa_pkcs1_invalid_hash_len(dlen, hp) { return -EINVAL; }
    let mut out = vec![0u8; slen as usize];
    core::ptr::copy_nonoverlapping(src as *const u8, out.as_mut_ptr(), slen as usize);
    let err = crypto_akcipher_encrypt((*c).child, out.as_mut_ptr(), slen);
    if err != 0 { return err; }
    let mut n = slen as usize;
    if n < (*c).key_size as usize - 1 { return -EINVAL; }
    if n == (*c).key_size as usize { if out[0] != 0 { return -EINVAL; } n -= 1; out = out[1..].to_vec(); }
    if out[0] != 1 { return -EBADMSG; }
    let mut pos = 1usize; while pos < n && out[pos] == 0xff { pos += 1; }
    if pos < 9 || pos == n || out[pos] != 0 { return -EBADMSG; }
    pos += 1;
    if (*hp).size > n - pos || crypto_memneq(out.as_ptr().add(pos), (*hp).data, (*hp).size) != 0 { return -EBADMSG; }
    pos += (*hp).size;
    if dlen as usize != n - pos || memcmp(digest as *const u8, out.as_ptr().add(pos), dlen as usize) != 0 { return -EKEYREJECTED; }
    0
}
unsafe fn rsassa_pkcs1_key_size(tfm: *mut crypto_sig) -> u32 { (*(crypto_sig_ctx(tfm) as *mut RsassaPkcs1Ctx)).key_size * BITS_PER_BYTE }
unsafe fn rsassa_pkcs1_set_pub_key(tfm: *mut crypto_sig, key: *const core::ffi::c_void, keylen: u32) -> i32 { let c=crypto_sig_ctx(tfm) as *mut RsassaPkcs1Ctx; rsa_set_key((*c).child, &mut (*c).key_size, RSA_PUB, key, keylen) }
unsafe fn rsassa_pkcs1_set_priv_key(tfm: *mut crypto_sig, key: *const core::ffi::c_void, keylen: u32) -> i32 { let c=crypto_sig_ctx(tfm) as *mut RsassaPkcs1Ctx; rsa_set_key((*c).child, &mut (*c).key_size, RSA_PRIV, key, keylen) }

unsafe fn rsassa_pkcs1_init_tfm(_tfm: *mut crypto_sig) -> i32 { 0 }
unsafe fn rsassa_pkcs1_exit_tfm(_tfm: *mut crypto_sig) {}
unsafe fn rsassa_pkcs1_free(_inst: *mut core::ffi::c_void) {}
unsafe fn rsassa_pkcs1_create(_tmpl: *mut CryptoTemplate, _tb: *mut *mut core::ffi::c_void) -> i32 { 0 }

#[repr(C)] struct CryptoTemplate { name: *const core::ffi::c_char, create: Option<unsafe fn(*mut CryptoTemplate, *mut *mut core::ffi::c_void) -> i32>, module: *mut core::ffi::c_void }
#[no_mangle] static mut rsassa_pkcs1_tmpl: CryptoTemplate = CryptoTemplate { name: b"pkcs1\0".as_ptr() as _, create: None, module: core::ptr::null_mut() };

// External symbols are supplied by the Linux crypto subsystem and other translated files.
extern "C" { fn strcmp(_: *const core::ffi::c_char, _: *const core::ffi::c_char) -> i32; fn memcmp(_: *const u8, _: *const u8, _: usize) -> i32; fn crypto_memneq(_: *const u8, _: *const u8, _: usize) -> i32; fn crypto_sig_ctx(_: *mut crypto_sig) -> *mut core::ffi::c_void; fn sig_alg_instance(_: *mut crypto_sig) -> *mut core::ffi::c_void; fn sig_instance_ctx(_: *mut core::ffi::c_void) -> *mut core::ffi::c_void; fn crypto_akcipher_sync_decrypt(_: *mut crypto_akcipher, _: *mut u8, _: u32, _: *mut u8, _: u32) -> i32; fn crypto_akcipher_encrypt(_: *mut crypto_akcipher, _: *mut u8, _: u32) -> i32; fn rsa_set_key(_: *mut crypto_akcipher, _: *mut u32, _: i32, _: *const core::ffi::c_void, _: u32) -> i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
