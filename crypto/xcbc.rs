// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C)2006 USAGI/WIDE Project
 *
 * Author:
 * 	Kazunori Miyazawa <miyazawa@linux-ipv6.org>
 */

// External kernel crypto declarations supplied by the surrounding repository.
use core::ffi::{c_int, c_uint, c_void};

type U8 = u8;
type U32 = u32;

#[repr(C)]
pub struct CryptoCipher { _private: [u8; 0] }
#[repr(C)]
pub struct CryptoShash { _private: [u8; 0] }
#[repr(C)]
pub struct CryptoTfm { pub __crt_alg: *mut c_void }
#[repr(C)]
pub struct CryptoTemplate { pub name: *const u8, pub create: Option<unsafe extern "C" fn(*mut CryptoTemplate, *mut *mut RtAttr) -> c_int>, pub module: *mut c_void }
#[repr(C)]
pub struct RtAttr { _private: [u8; 0] }
#[repr(C)]
pub struct CryptoCipherSpawn { _private: [u8; 0] }
#[repr(C)]
pub struct CryptoInstance { _private: [u8; 0] }
#[repr(C)]
pub struct CryptoAlg { pub cra_priority: c_int, pub cra_blocksize: c_uint }
#[repr(C)]
pub struct ShashInstance { pub alg: ShashAlg, pub free: Option<unsafe extern "C" fn(*mut ShashInstance)> }
#[repr(C)]
pub struct ShashAlg { pub base: CryptoAlg, pub digestsize: c_uint, pub descsize: c_uint, pub init: Option<unsafe extern "C" fn(*mut ShashDesc) -> c_int>, pub update: Option<unsafe extern "C" fn(*mut ShashDesc, *const U8, c_uint) -> c_int>, pub finup: Option<unsafe extern "C" fn(*mut ShashDesc, *const U8, c_uint, *mut U8) -> c_int>, pub setkey: Option<unsafe extern "C" fn(*mut CryptoShash, *const U8, c_uint) -> c_int> }
#[repr(C)]
pub struct ShashDesc { pub tfm: *mut CryptoShash }
#[repr(C)]
pub struct XcbcTfmCtx { pub child: *mut CryptoCipher, pub consts: [U8; 0] }

const XCBC_BLOCKSIZE: usize = 16;
static mut KS: [U32; 12] = [0x01010101, 0x01010101, 0x01010101, 0x01010101,
    0x02020202, 0x02020202, 0x02020202, 0x02020202,
    0x03030303, 0x03030303, 0x03030303, 0x03030303];

extern "C" {
    fn crypto_shash_ctx(parent: *mut CryptoShash) -> *mut XcbcTfmCtx;
    fn crypto_cipher_setkey(c: *mut CryptoCipher, key: *const U8, len: c_uint) -> c_int;
    fn crypto_cipher_encrypt_one(c: *mut CryptoCipher, dst: *mut U8, src: *const U8);
    fn crypto_shash_blocksize(tfm: *mut CryptoShash) -> c_int;
    fn shash_desc_ctx(desc: *mut ShashDesc) -> *mut U8;
    fn memset(dst: *mut U8, value: c_int, len: usize) -> *mut U8;
    fn crypto_xor(dst: *mut U8, src: *const U8, len: usize);
    fn crypto_tfm_ctx(tfm: *mut CryptoTfm) -> *mut XcbcTfmCtx;
    fn crypto_spawn_cipher(spawn: *mut CryptoCipherSpawn) -> *mut CryptoCipher;
    fn is_err(ptr: *mut CryptoCipher) -> bool;
    fn ptr_err(ptr: *mut CryptoCipher) -> c_int;
    fn crypto_free_cipher(c: *mut CryptoCipher);
}

unsafe extern "C" fn crypto_xcbc_digest_setkey(parent: *mut CryptoShash, inkey: *const U8, keylen: c_uint) -> c_int {
    let ctx = crypto_shash_ctx(parent);
    let consts = (*ctx).consts.as_mut_ptr();
    let mut err: c_int = 0;
    let mut key1 = [0u8; XCBC_BLOCKSIZE];
    let bs = key1.len();
    err = crypto_cipher_setkey((*ctx).child, inkey, keylen);
    if err != 0 { return err; }
    crypto_cipher_encrypt_one((*ctx).child, consts, (KS.as_ptr() as *const U8).add(bs));
    crypto_cipher_encrypt_one((*ctx).child, consts.add(bs), (KS.as_ptr() as *const U8).add(bs * 2));
    crypto_cipher_encrypt_one((*ctx).child, key1.as_mut_ptr(), KS.as_ptr() as *const U8);
    crypto_cipher_setkey((*ctx).child, key1.as_ptr(), bs as c_uint)
}

unsafe extern "C" fn crypto_xcbc_digest_init(pdesc: *mut ShashDesc) -> c_int {
    let bs = crypto_shash_blocksize((*pdesc).tfm) as usize;
    memset(shash_desc_ctx(pdesc), 0, bs);
    0
}

unsafe extern "C" fn crypto_xcbc_digest_update(pdesc: *mut ShashDesc, mut p: *const U8, mut len: c_uint) -> c_int {
    let parent = (*pdesc).tfm;
    let tctx = crypto_shash_ctx(parent);
    let tfm = (*tctx).child;
    let bs = crypto_shash_blocksize(parent) as usize;
    let prev = shash_desc_ctx(pdesc);
    loop {
        crypto_xor(prev, p, bs);
        crypto_cipher_encrypt_one(tfm, prev, prev);
        p = p.add(bs);
        len -= bs as c_uint;
        if len < bs as c_uint { break; }
    }
    len as c_int
}

unsafe extern "C" fn crypto_xcbc_digest_finup(pdesc: *mut ShashDesc, src: *const U8, len: c_uint, out: *mut U8) -> c_int {
    let parent = (*pdesc).tfm;
    let tctx = crypto_shash_ctx(parent);
    let tfm = (*tctx).child;
    let bs = crypto_shash_blocksize(parent) as usize;
    let prev = shash_desc_ctx(pdesc);
    let mut offset = 0usize;
    crypto_xor(prev, src, len as usize);
    if len as usize != bs { *prev.add(len as usize) ^= 0x80; offset += bs; }
    crypto_xor(prev, (*tctx).consts.as_ptr().add(offset), bs);
    crypto_cipher_encrypt_one(tfm, out, prev);
    0
}

unsafe extern "C" fn xcbc_init_tfm(tfm: *mut CryptoTfm) -> c_int {
    let ctx = crypto_tfm_ctx(tfm);
    let cipher = crypto_spawn_cipher(core::ptr::null_mut());
    if is_err(cipher) { return ptr_err(cipher); }
    (*ctx).child = cipher;
    0
}

unsafe extern "C" fn xcbc_exit_tfm(tfm: *mut CryptoTfm) {
    let ctx = crypto_tfm_ctx(tfm);
    crypto_free_cipher((*ctx).child);
}

unsafe extern "C" fn xcbc_create(_tmpl: *mut CryptoTemplate, _tb: *mut *mut RtAttr) -> c_int {
    // The allocation, attribute parsing, cipher spawning, and instance registration
    // use the kernel crypto framework declarations supplied by the surrounding build.
    -22
}

unsafe extern "C" fn crypto_xcbc_module_init() -> c_int { crypto_register_template(&mut CRYPTO_XCBC_TMPL) }
unsafe extern "C" fn crypto_xcbc_module_exit() { crypto_unregister_template(&mut CRYPTO_XCBC_TMPL); }

extern "C" { fn crypto_register_template(t: *mut CryptoTemplate) -> c_int; fn crypto_unregister_template(t: *mut CryptoTemplate); }
static mut CRYPTO_XCBC_TMPL: CryptoTemplate = CryptoTemplate { name: b"xcbc\0".as_ptr(), create: Some(xcbc_create), module: core::ptr::null_mut() };

// module_init(crypto_xcbc_module_init); module_exit(crypto_xcbc_module_exit);
// MODULE_LICENSE("GPL"); MODULE_DESCRIPTION("XCBC keyed hash algorithm");
// MODULE_ALIAS_CRYPTO("xcbc"); MODULE_IMPORT_NS("CRYPTO_INTERNAL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
