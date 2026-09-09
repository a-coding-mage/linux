/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
 * Copyright (c) 2002 David S. Miller (davem@redhat.com)
 * Copyright (c) 2005 Herbert Xu <herbert@gondor.apana.org.au>
 *
 * Portions derived from Cryptoapi, by Alexander Kjeldaas <astor@fast.no>
 * and Nettle, by Niels Möller.
 */

// Dependency declarations supplied by the translated crypto API.
extern "C" {
    fn crypto_alloc_base(alg_name: *const core::ffi::c_char, type_: u32, mask: u32) -> *mut crypto_tfm;
    fn crypto_free_tfm(tfm: *mut crypto_tfm);
    fn crypto_has_alg(alg_name: *const core::ffi::c_char, type_: u32, mask: u32) -> i32;
    fn crypto_tfm_alg_blocksize(tfm: *mut crypto_tfm) -> u32;
    fn crypto_tfm_alg_alignmask(tfm: *mut crypto_tfm) -> u32;
    fn crypto_tfm_get_flags(tfm: *mut crypto_tfm) -> u32;
    fn crypto_tfm_set_flags(tfm: *mut crypto_tfm, flags: u32);
    fn crypto_tfm_clear_flags(tfm: *mut crypto_tfm, flags: u32);
    fn crypto_grab_spawn(base: *mut crypto_spawn, inst: *mut crypto_instance,
                         name: *const core::ffi::c_char, type_: u32, mask: u32) -> i32;
    fn crypto_drop_spawn(base: *mut crypto_spawn);
    fn crypto_spawn_tfm(base: *mut crypto_spawn, type_: u32, mask: u32) -> *mut crypto_tfm;
}

#[repr(C)]
pub struct crypto_cipher {
    pub base: crypto_tfm,
}

#[inline]
pub unsafe fn __crypto_cipher_cast(tfm: *mut crypto_tfm) -> *mut crypto_cipher {
    tfm as *mut crypto_cipher
}

#[inline]
pub unsafe fn crypto_alloc_cipher(alg_name: *const core::ffi::c_char, mut type_: u32, mut mask: u32) -> *mut crypto_cipher {
    type_ &= !CRYPTO_ALG_TYPE_MASK;
    type_ |= CRYPTO_ALG_TYPE_CIPHER;
    mask |= CRYPTO_ALG_TYPE_MASK;
    __crypto_cipher_cast(crypto_alloc_base(alg_name, type_, mask))
}

#[inline]
pub unsafe fn crypto_cipher_tfm(tfm: *mut crypto_cipher) -> *mut crypto_tfm {
    core::ptr::addr_of_mut!((*tfm).base)
}

#[inline]
pub unsafe fn crypto_free_cipher(tfm: *mut crypto_cipher) {
    crypto_free_tfm(crypto_cipher_tfm(tfm));
}

#[inline]
pub unsafe fn crypto_has_cipher(alg_name: *const core::ffi::c_char, mut type_: u32, mut mask: u32) -> i32 {
    type_ &= !CRYPTO_ALG_TYPE_MASK;
    type_ |= CRYPTO_ALG_TYPE_CIPHER;
    mask |= CRYPTO_ALG_TYPE_MASK;
    crypto_has_alg(alg_name, type_, mask)
}

#[inline]
pub unsafe fn crypto_cipher_blocksize(tfm: *mut crypto_cipher) -> u32 {
    crypto_tfm_alg_blocksize(crypto_cipher_tfm(tfm))
}

#[inline]
pub unsafe fn crypto_cipher_alignmask(tfm: *mut crypto_cipher) -> u32 {
    crypto_tfm_alg_alignmask(crypto_cipher_tfm(tfm))
}

#[inline]
pub unsafe fn crypto_cipher_get_flags(tfm: *mut crypto_cipher) -> u32 {
    crypto_tfm_get_flags(crypto_cipher_tfm(tfm))
}

#[inline]
pub unsafe fn crypto_cipher_set_flags(tfm: *mut crypto_cipher, flags: u32) {
    crypto_tfm_set_flags(crypto_cipher_tfm(tfm), flags);
}

#[inline]
pub unsafe fn crypto_cipher_clear_flags(tfm: *mut crypto_cipher, flags: u32) {
    crypto_tfm_clear_flags(crypto_cipher_tfm(tfm), flags);
}

extern "C" {
    pub fn crypto_cipher_setkey(tfm: *mut crypto_cipher, key: *const u8, keylen: u32) -> i32;
    pub fn crypto_cipher_encrypt_one(tfm: *mut crypto_cipher, dst: *mut u8, src: *const u8);
    pub fn crypto_cipher_decrypt_one(tfm: *mut crypto_cipher, dst: *mut u8, src: *const u8);
}

#[repr(C)]
pub struct crypto_cipher_spawn {
    pub base: crypto_spawn,
}

#[inline]
pub unsafe fn crypto_grab_cipher(spawn: *mut crypto_cipher_spawn, inst: *mut crypto_instance,
                                 name: *const core::ffi::c_char, mut type_: u32, mut mask: u32) -> i32 {
    type_ &= !CRYPTO_ALG_TYPE_MASK;
    type_ |= CRYPTO_ALG_TYPE_CIPHER;
    mask |= CRYPTO_ALG_TYPE_MASK;
    crypto_grab_spawn(core::ptr::addr_of_mut!((*spawn).base), inst, name, type_, mask)
}

#[inline]
pub unsafe fn crypto_drop_cipher(spawn: *mut crypto_cipher_spawn) {
    crypto_drop_spawn(core::ptr::addr_of_mut!((*spawn).base));
}

#[inline]
pub unsafe fn crypto_spawn_cipher_alg(spawn: *mut crypto_cipher_spawn) -> *mut crypto_alg {
    (*spawn).base.alg
}

#[inline]
pub unsafe fn crypto_spawn_cipher(spawn: *mut crypto_cipher_spawn) -> *mut crypto_cipher {
    __crypto_cipher_cast(crypto_spawn_tfm(core::ptr::addr_of_mut!((*spawn).base), CRYPTO_ALG_TYPE_CIPHER, CRYPTO_ALG_TYPE_MASK))
}

#[inline]
pub unsafe fn crypto_cipher_alg(tfm: *mut crypto_cipher) -> *mut cipher_alg {
    (*crypto_cipher_tfm(tfm)).__crt_alg.as_mut().unwrap().cra_cipher.as_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
