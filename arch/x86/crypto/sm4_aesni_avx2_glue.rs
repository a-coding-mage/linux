/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SM4 Cipher Algorithm, AES-NI/AVX2 optimized.
 * as specified in
 * https://tools.ietf.org/id/draft-ribose-cfrg-sm4-10.html
 *
 * Copyright (c) 2021, Alibaba Group.
 * Copyright (c) 2021 Tianjia Zhang <tianjia.zhang@linux.alibaba.com>.
 */

// Kernel and architecture dependencies supplied by the surrounding repository.

const SM4_CRYPT16_BLOCK_SIZE: usize = SM4_BLOCK_SIZE * 16;

extern "C" {
    fn sm4_aesni_avx2_ctr_enc_blk16(rk: *const u32, dst: *mut u8,
                                    src: *const u8, iv: *mut u8);
    fn sm4_aesni_avx2_cbc_dec_blk16(rk: *const u32, dst: *mut u8,
                                    src: *const u8, iv: *mut u8);

    fn crypto_skcipher_ctx(tfm: *mut crypto_skcipher) -> *mut sm4_ctx;
    fn sm4_expandkey(ctx: *mut sm4_ctx, key: *const u8, key_len: c_uint) -> c_int;
    fn sm4_avx_cbc_decrypt(req: *mut skcipher_request, block_size: usize,
                           decrypt: unsafe extern "C" fn(*const u32, *mut u8,
                                                          *const u8, *mut u8)) -> c_int;
    fn sm4_avx_ctr_crypt(req: *mut skcipher_request, block_size: usize,
                         encrypt: unsafe extern "C" fn(*const u32, *mut u8,
                                                        *const u8, *mut u8)) -> c_int;
    fn sm4_avx_ecb_encrypt(req: *mut skcipher_request) -> c_int;
    fn sm4_avx_ecb_decrypt(req: *mut skcipher_request) -> c_int;
    fn sm4_cbc_encrypt(req: *mut skcipher_request) -> c_int;
    fn crypto_register_skciphers(algs: *mut skcipher_alg, count: usize) -> c_int;
    fn crypto_unregister_skciphers(algs: *mut skcipher_alg, count: usize);
    fn boot_cpu_has(feature: c_uint) -> bool;
    fn cpu_has_xfeatures(mask: u64, feature_name: *mut *const c_char) -> bool;
    fn pr_info(fmt: *const c_char, ...);
}

// External C types and constants supplied by kernel headers.
use core::ffi::{c_char, c_int, c_uint};

extern "C" {
    static THIS_MODULE: module;
}

#[repr(C)]
pub struct sm4_ctx {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct crypto_skcipher {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct skcipher_request {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct module {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct crypto_alg {
    pub cra_name: *const c_char,
    pub cra_driver_name: *const c_char,
    pub cra_priority: c_int,
    pub cra_blocksize: usize,
    pub cra_ctxsize: usize,
    pub cra_module: *const module,
}

#[repr(C)]
pub struct skcipher_alg {
    pub base: crypto_alg,
    pub min_keysize: usize,
    pub max_keysize: usize,
    pub ivsize: usize,
    pub chunksize: usize,
    pub walksize: usize,
    pub setkey: unsafe extern "C" fn(*mut crypto_skcipher, *const u8, c_uint) -> c_int,
    pub encrypt: unsafe extern "C" fn(*mut skcipher_request) -> c_int,
    pub decrypt: unsafe extern "C" fn(*mut skcipher_request) -> c_int,
}

unsafe extern "C" fn sm4_skcipher_setkey(tfm: *mut crypto_skcipher,
                                          key: *const u8, key_len: c_uint) -> c_int {
    let ctx = crypto_skcipher_ctx(tfm);
    sm4_expandkey(ctx, key, key_len)
}

unsafe extern "C" fn cbc_decrypt(req: *mut skcipher_request) -> c_int {
    sm4_avx_cbc_decrypt(req, SM4_CRYPT16_BLOCK_SIZE, sm4_aesni_avx2_cbc_dec_blk16)
}

unsafe extern "C" fn ctr_crypt(req: *mut skcipher_request) -> c_int {
    sm4_avx_ctr_crypt(req, SM4_CRYPT16_BLOCK_SIZE, sm4_aesni_avx2_ctr_enc_blk16)
}

static mut sm4_aesni_avx2_skciphers: [skcipher_alg; 3] = [
    skcipher_alg {
        base: crypto_alg { cra_name: b"ecb(sm4)\0".as_ptr() as *const c_char,
            cra_driver_name: b"ecb-sm4-aesni-avx2\0".as_ptr() as *const c_char,
            cra_priority: 500, cra_blocksize: SM4_BLOCK_SIZE, cra_ctxsize: core::mem::size_of::<sm4_ctx>(),
            cra_module: unsafe { &THIS_MODULE } },
        min_keysize: SM4_KEY_SIZE, max_keysize: SM4_KEY_SIZE, ivsize: 0, chunksize: 0,
        walksize: 16 * SM4_BLOCK_SIZE, setkey: sm4_skcipher_setkey,
        encrypt: sm4_avx_ecb_encrypt, decrypt: sm4_avx_ecb_decrypt,
    },
    skcipher_alg {
        base: crypto_alg { cra_name: b"cbc(sm4)\0".as_ptr() as *const c_char,
            cra_driver_name: b"cbc-sm4-aesni-avx2\0".as_ptr() as *const c_char,
            cra_priority: 500, cra_blocksize: SM4_BLOCK_SIZE, cra_ctxsize: core::mem::size_of::<sm4_ctx>(),
            cra_module: unsafe { &THIS_MODULE } },
        min_keysize: SM4_KEY_SIZE, max_keysize: SM4_KEY_SIZE, ivsize: SM4_BLOCK_SIZE, chunksize: 0,
        walksize: 16 * SM4_BLOCK_SIZE, setkey: sm4_skcipher_setkey,
        encrypt: sm4_cbc_encrypt, decrypt: cbc_decrypt,
    },
    skcipher_alg {
        base: crypto_alg { cra_name: b"ctr(sm4)\0".as_ptr() as *const c_char,
            cra_driver_name: b"ctr-sm4-aesni-avx2\0".as_ptr() as *const c_char,
            cra_priority: 500, cra_blocksize: 1, cra_ctxsize: core::mem::size_of::<sm4_ctx>(),
            cra_module: unsafe { &THIS_MODULE } },
        min_keysize: SM4_KEY_SIZE, max_keysize: SM4_KEY_SIZE, ivsize: SM4_BLOCK_SIZE,
        chunksize: SM4_BLOCK_SIZE, walksize: 16 * SM4_BLOCK_SIZE,
        setkey: sm4_skcipher_setkey, encrypt: ctr_crypt, decrypt: ctr_crypt,
    },
];

unsafe extern "C" fn sm4_init() -> c_int {
    let mut feature_name: *const c_char = core::ptr::null();
    if !boot_cpu_has(X86_FEATURE_AVX) || !boot_cpu_has(X86_FEATURE_AVX2)
        || !boot_cpu_has(X86_FEATURE_AES) || !boot_cpu_has(X86_FEATURE_OSXSAVE) {
        pr_info(b"AVX2 or AES-NI instructions are not detected.\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }
    if !cpu_has_xfeatures(XFEATURE_MASK_SSE | XFEATURE_MASK_YMM, &mut feature_name) {
        pr_info(b"CPU feature '%s' is not supported.\n\0".as_ptr() as *const c_char, feature_name);
        return -ENODEV;
    }
    crypto_register_skciphers(sm4_aesni_avx2_skciphers.as_mut_ptr(), 3)
}

unsafe extern "C" fn sm4_exit() {
    crypto_unregister_skciphers(sm4_aesni_avx2_skciphers.as_mut_ptr(), 3);
}

// module_init(sm4_init); module_exit(sm4_exit);
// MODULE_LICENSE("GPL v2");
// MODULE_AUTHOR("Tianjia Zhang <tianjia.zhang@linux.alibaba.com>");
// MODULE_DESCRIPTION("SM4 Cipher Algorithm, AES-NI/AVX2 optimized");
// MODULE_ALIAS_CRYPTO("sm4");
// MODULE_ALIAS_CRYPTO("sm4-aesni-avx2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
