/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2019 Samsung Electronics Co., Ltd.
 */

// Dependency supplied by the surrounding kernel translation.
use core::ffi::c_int;

// Corresponds to <crypto/aead.h> and the list implementation.
// These types are defined by external dependencies.
#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_aead {
    _private: [u8; 0],
}

pub const CRYPTO_AEAD_AES_GCM: usize = 16;
pub const CRYPTO_AEAD_AES_CCM: usize = CRYPTO_AEAD_AES_GCM + 1;
pub const CRYPTO_AEAD_MAX: usize = CRYPTO_AEAD_AES_CCM + 1;

#[repr(C)]
pub struct ksmbd_crypto_ctx {
    pub list: list_head,
    pub ccmaes: [*mut crypto_aead; CRYPTO_AEAD_MAX],
}

#[inline]
pub unsafe fn CRYPTO_GCM(c: *mut ksmbd_crypto_ctx) -> *mut crypto_aead {
    (*c).ccmaes[CRYPTO_AEAD_AES_GCM]
}

#[inline]
pub unsafe fn CRYPTO_CCM(c: *mut ksmbd_crypto_ctx) -> *mut crypto_aead {
    (*c).ccmaes[CRYPTO_AEAD_AES_CCM]
}

unsafe extern "C" {
    pub fn ksmbd_release_crypto_ctx(ctx: *mut ksmbd_crypto_ctx);
    pub fn ksmbd_crypto_ctx_find_gcm() -> *mut ksmbd_crypto_ctx;
    pub fn ksmbd_crypto_ctx_find_ccm() -> *mut ksmbd_crypto_ctx;
    pub fn ksmbd_crypto_destroy();
    pub fn ksmbd_crypto_create() -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
