/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * geniv: IV generation
 *
 * Copyright (c) 2015 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Dependency intent from <crypto/internal/aead.h> and <linux/types.h>.

#[allow(non_camel_case_types)]
pub type u8 = core::primitive::u8;

#[repr(C)]
pub struct crypto_aead;

#[repr(C)]
pub struct crypto_template;

#[repr(C)]
pub struct aead_instance;

#[repr(C)]
pub struct rtattr;

#[repr(C)]
pub struct aead_geniv_ctx {
    pub child: *mut crypto_aead,
    // Flexible array member, aligned as in C with __alignof__(u32).
    pub salt: [u8; 0],
}

extern "C" {
    pub fn aead_geniv_alloc(
        tmpl: *mut crypto_template,
        tb: *mut *mut rtattr,
    ) -> *mut aead_instance;
    pub fn aead_init_geniv(tfm: *mut crypto_aead) -> core::ffi::c_int;
    pub fn aead_exit_geniv(tfm: *mut crypto_aead);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
