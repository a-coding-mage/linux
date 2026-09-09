// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2024 Meta, Inc */
// Dependencies supplied by the Linux kernel and other translation units:
// linux/types.h, linux/module.h, linux/bpf_crypto.h, crypto/skcipher.h

use core::ffi::{c_char, c_int, c_uint, c_void};

type U8 = u8;
type U32 = u32;

#[repr(C)]
struct BpfCryptoType {
    alloc_tfm: Option<unsafe extern "C" fn(*const c_char) -> *mut c_void>,
    free_tfm: Option<unsafe extern "C" fn(*mut c_void)>,
    has_algo: Option<unsafe extern "C" fn(*const c_char) -> c_int>,
    setkey: Option<unsafe extern "C" fn(*mut c_void, *const U8, c_uint) -> c_int>,
    encrypt: Option<unsafe extern "C" fn(*mut c_void, *const U8, *mut U8, c_uint, *mut U8) -> c_int>,
    decrypt: Option<unsafe extern "C" fn(*mut c_void, *const U8, *mut U8, c_uint, *mut U8) -> c_int>,
    ivsize: Option<unsafe extern "C" fn(*mut c_void) -> c_uint>,
    statesize: Option<unsafe extern "C" fn(*mut c_void) -> c_uint>,
    get_flags: Option<unsafe extern "C" fn(*mut c_void) -> U32>,
    owner: *mut c_void,
    name: *const c_char,
}

unsafe extern "C" {
    fn crypto_alloc_lskcipher(algo: *const c_char, type_: u32, mask: u32) -> *mut c_void;
    fn crypto_free_lskcipher(tfm: *mut c_void);
    fn crypto_has_skcipher(algo: *const c_char, type_: u32, mask: u32) -> c_int;
    fn crypto_lskcipher_setkey(tfm: *mut c_void, key: *const U8, keylen: c_uint) -> c_int;
    fn crypto_lskcipher_get_flags(tfm: *mut c_void) -> U32;
    fn crypto_lskcipher_ivsize(tfm: *mut c_void) -> c_uint;
    fn crypto_lskcipher_statesize(tfm: *mut c_void) -> c_uint;
    fn crypto_lskcipher_encrypt(tfm: *mut c_void, src: *const U8, dst: *mut U8, len: c_uint, siv: *mut U8) -> c_int;
    fn crypto_lskcipher_decrypt(tfm: *mut c_void, src: *const U8, dst: *mut U8, len: c_uint, siv: *mut U8) -> c_int;
    fn bpf_crypto_register_type(type_: *const BpfCryptoType) -> c_int;
    fn bpf_crypto_unregister_type(type_: *const BpfCryptoType) -> c_int;
    fn warn_on_once(condition: bool);
}

const CRYPTO_ALG_TYPE_LSKCIPHER: u32 = 0;
const CRYPTO_ALG_TYPE_MASK: u32 = 0;
static mut THIS_MODULE: *mut c_void = core::ptr::null_mut();

unsafe extern "C" fn bpf_crypto_lskcipher_alloc_tfm(algo: *const c_char) -> *mut c_void {
    crypto_alloc_lskcipher(algo, 0, 0)
}

unsafe extern "C" fn bpf_crypto_lskcipher_free_tfm(tfm: *mut c_void) {
    crypto_free_lskcipher(tfm);
}

unsafe extern "C" fn bpf_crypto_lskcipher_has_algo(algo: *const c_char) -> c_int {
    crypto_has_skcipher(algo, CRYPTO_ALG_TYPE_LSKCIPHER, CRYPTO_ALG_TYPE_MASK)
}

unsafe extern "C" fn bpf_crypto_lskcipher_setkey(tfm: *mut c_void, key: *const U8, keylen: c_uint) -> c_int {
    crypto_lskcipher_setkey(tfm, key, keylen)
}

unsafe extern "C" fn bpf_crypto_lskcipher_get_flags(tfm: *mut c_void) -> U32 {
    crypto_lskcipher_get_flags(tfm)
}

unsafe extern "C" fn bpf_crypto_lskcipher_ivsize(tfm: *mut c_void) -> c_uint {
    crypto_lskcipher_ivsize(tfm)
}

unsafe extern "C" fn bpf_crypto_lskcipher_statesize(tfm: *mut c_void) -> c_uint {
    crypto_lskcipher_statesize(tfm)
}

unsafe extern "C" fn bpf_crypto_lskcipher_encrypt(tfm: *mut c_void, src: *const U8, dst: *mut U8, len: c_uint, siv: *mut U8) -> c_int {
    crypto_lskcipher_encrypt(tfm, src, dst, len, siv)
}

unsafe extern "C" fn bpf_crypto_lskcipher_decrypt(tfm: *mut c_void, src: *const U8, dst: *mut U8, len: c_uint, siv: *mut U8) -> c_int {
    crypto_lskcipher_decrypt(tfm, src, dst, len, siv)
}

static BPF_CRYPTO_LSKCIPHER_TYPE: BpfCryptoType = BpfCryptoType {
    alloc_tfm: Some(bpf_crypto_lskcipher_alloc_tfm),
    free_tfm: Some(bpf_crypto_lskcipher_free_tfm),
    has_algo: Some(bpf_crypto_lskcipher_has_algo),
    setkey: Some(bpf_crypto_lskcipher_setkey),
    encrypt: Some(bpf_crypto_lskcipher_encrypt),
    decrypt: Some(bpf_crypto_lskcipher_decrypt),
    ivsize: Some(bpf_crypto_lskcipher_ivsize),
    statesize: Some(bpf_crypto_lskcipher_statesize),
    get_flags: Some(bpf_crypto_lskcipher_get_flags),
    owner: unsafe { core::ptr::addr_of_mut!(THIS_MODULE).read() },
    name: b"skcipher\0".as_ptr() as *const c_char,
};

unsafe extern "C" fn bpf_crypto_skcipher_init() -> c_int {
    bpf_crypto_register_type(&BPF_CRYPTO_LSKCIPHER_TYPE)
}

unsafe extern "C" fn bpf_crypto_skcipher_exit() {
    let err: c_int = bpf_crypto_unregister_type(&BPF_CRYPTO_LSKCIPHER_TYPE);
    warn_on_once(err != 0);
}

// module_init(bpf_crypto_skcipher_init);
// module_exit(bpf_crypto_skcipher_exit);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Symmetric key cipher support for BPF");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
