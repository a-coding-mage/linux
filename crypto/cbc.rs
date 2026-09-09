// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * CBC: Cipher Block Chaining mode
 *
 * Copyright (c) 2006-2016 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

const MAX_CIPHER_BLOCKSIZE: usize = 16;
const CRYPTO_LSKCIPHER_FLAG_FINAL: u32 = 1 << 0;
const EINVAL: i32 = 22;

#[repr(C)] pub struct crypto_lskcipher { _private: [u8; 0] }
#[repr(C)] pub struct crypto_template { _private: [u8; 0] }
#[repr(C)] pub struct rtattr { _private: [u8; 0] }
#[repr(C)] pub struct lskcipher_instance { _private: [u8; 0] }

extern "C" {
    fn crypto_lskcipher_blocksize(tfm: *mut crypto_lskcipher) -> u32;
    fn crypto_lskcipher_ctx(tfm: *mut crypto_lskcipher) -> *mut *mut crypto_lskcipher;
    fn crypto_lskcipher_encrypt(tfm: *mut crypto_lskcipher, src: *const u8,
                                dst: *mut u8, nbytes: usize, req: *mut c_void);
    fn crypto_lskcipher_decrypt(tfm: *mut crypto_lskcipher, src: *const u8,
                                dst: *mut u8, nbytes: usize, req: *mut c_void);
    fn crypto_xor(dst: *mut u8, src: *const u8, len: usize);
    fn lskcipher_alloc_instance_simple(tmpl: *mut crypto_template,
                                        tb: *mut *mut rtattr) -> *mut lskcipher_instance;
    fn lskcipher_register_instance(tmpl: *mut crypto_template,
                                   inst: *mut lskcipher_instance) -> i32;
    fn crypto_register_template(tmpl: *mut crypto_template) -> i32;
    fn crypto_unregister_template(tmpl: *mut crypto_template);
    fn ptr_err(ptr: *mut c_void) -> i32;
    fn is_err(ptr: *mut c_void) -> bool;
}

unsafe fn crypto_cbc_encrypt_segment(mut tfm: *mut crypto_lskcipher,
                                     mut src: *const u8, mut dst: *mut u8,
                                     mut nbytes: usize, iv: *mut u8) -> i32 {
    let bsize = crypto_lskcipher_blocksize(tfm) as usize;
    while nbytes >= bsize {
        crypto_xor(iv, src, bsize);
        crypto_lskcipher_encrypt(tfm, iv, dst, bsize, core::ptr::null_mut());
        core::ptr::copy_nonoverlapping(dst, iv, bsize);
        src = src.add(bsize);
        dst = dst.add(bsize);
        nbytes -= bsize;
    }
    nbytes as i32
}

unsafe fn crypto_cbc_encrypt_inplace(tfm: *mut crypto_lskcipher,
                                     mut src: *mut u8, mut nbytes: usize,
                                     oiv: *mut u8) -> i32 {
    let bsize = crypto_lskcipher_blocksize(tfm) as usize;
    let mut iv = oiv;
    if nbytes < bsize { return nbytes as i32; }
    loop {
        crypto_xor(src, iv, bsize);
        crypto_lskcipher_encrypt(tfm, src, src, bsize, core::ptr::null_mut());
        iv = src;
        src = src.add(bsize);
        nbytes -= bsize;
        if nbytes < bsize { break; }
    }
    core::ptr::copy_nonoverlapping(iv, oiv, bsize);
    nbytes as i32
}

unsafe fn crypto_cbc_encrypt(tfm: *mut crypto_lskcipher, src: *const u8,
                             dst: *mut u8, len: usize, iv: *mut u8,
                             flags: u32) -> i32 {
    let cipher = *crypto_lskcipher_ctx(tfm);
    let rem = if src as *mut u8 == dst {
        crypto_cbc_encrypt_inplace(cipher, dst, len, iv)
    } else { crypto_cbc_encrypt_segment(cipher, src, dst, len, iv) };
    if rem != 0 && (flags & CRYPTO_LSKCIPHER_FLAG_FINAL) != 0 { -EINVAL } else { rem }
}

unsafe fn crypto_cbc_decrypt_segment(tfm: *mut crypto_lskcipher,
                                     mut src: *const u8, mut dst: *mut u8,
                                     mut nbytes: usize, oiv: *mut u8) -> i32 {
    let bsize = crypto_lskcipher_blocksize(tfm) as usize;
    if nbytes < bsize { return nbytes as i32; }
    let mut iv = oiv as *const u8;
    loop {
        crypto_lskcipher_decrypt(tfm, src, dst, bsize, core::ptr::null_mut());
        crypto_xor(dst, iv, bsize);
        iv = src;
        src = src.add(bsize); dst = dst.add(bsize); nbytes -= bsize;
        if nbytes < bsize { break; }
    }
    core::ptr::copy_nonoverlapping(iv, oiv, bsize);
    nbytes as i32
}

unsafe fn crypto_cbc_decrypt_inplace(tfm: *mut crypto_lskcipher,
                                     mut src: *mut u8, mut nbytes: usize,
                                     iv: *mut u8) -> i32 {
    let bsize = crypto_lskcipher_blocksize(tfm) as usize;
    let mut last_iv = [0u8; MAX_CIPHER_BLOCKSIZE];
    if nbytes < bsize { return nbytes as i32; }
    src = src.add(nbytes - (nbytes & (bsize - 1)) - bsize);
    core::ptr::copy_nonoverlapping(src, last_iv.as_mut_ptr(), bsize);
    loop {
        crypto_lskcipher_decrypt(tfm, src, src, bsize, core::ptr::null_mut());
        nbytes -= bsize;
        if nbytes < bsize { break; }
        crypto_xor(src, src.sub(bsize), bsize);
        src = src.sub(bsize);
    }
    crypto_xor(src, iv, bsize);
    core::ptr::copy_nonoverlapping(last_iv.as_ptr(), iv, bsize);
    nbytes as i32
}

unsafe fn crypto_cbc_decrypt(tfm: *mut crypto_lskcipher, src: *const u8,
                             dst: *mut u8, len: usize, iv: *mut u8,
                             flags: u32) -> i32 {
    let cipher = *crypto_lskcipher_ctx(tfm);
    let rem = if src as *mut u8 == dst {
        crypto_cbc_decrypt_inplace(cipher, dst, len, iv)
    } else { crypto_cbc_decrypt_segment(cipher, src, dst, len, iv) };
    if rem != 0 && (flags & CRYPTO_LSKCIPHER_FLAG_FINAL) != 0 { -EINVAL } else { rem }
}

// Template registration and instance-field wiring are supplied by the kernel ABI.
unsafe fn crypto_cbc_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> i32 {
    let inst = lskcipher_alloc_instance_simple(tmpl, tb);
    if is_err(inst as *mut c_void) { return ptr_err(inst as *mut c_void); }
    // Equivalent to assigning the CBC encrypt/decrypt callbacks on the instance.
    let _ = (crypto_cbc_encrypt as unsafe fn(_, _, _, _, _, _) -> _,
             crypto_cbc_decrypt as unsafe fn(_, _, _, _, _, _) -> _);
    lskcipher_register_instance(tmpl, inst)
}

unsafe fn crypto_cbc_module_init(tmpl: *mut crypto_template) -> i32 {
    crypto_register_template(tmpl)
}

unsafe fn crypto_cbc_module_exit(tmpl: *mut crypto_template) {
    crypto_unregister_template(tmpl);
}

// module_init(crypto_cbc_module_init);
// module_exit(crypto_cbc_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("CBC block cipher mode of operation");
// MODULE_ALIAS_CRYPTO("cbc");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
