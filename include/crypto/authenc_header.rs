/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Authenc: Simple AEAD wrapper for IPsec
 *
 * Copyright (c) 2007 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Dependency supplied by the Linux type definitions: `u8` and `__be32`.

pub const CRYPTO_AUTHENC_KEYA_UNSPEC: i32 = 0;
pub const CRYPTO_AUTHENC_KEYA_PARAM: i32 = 1;

#[repr(C)]
pub struct crypto_authenc_key_param {
	pub enckeylen: __be32,
}

#[repr(C)]
pub struct crypto_authenc_keys {
	pub authkey: *const u8,
	pub enckey: *const u8,

	pub authkeylen: core::ffi::c_uint,
	pub enckeylen: core::ffi::c_uint,
}

unsafe extern "C" {
	pub fn crypto_authenc_extractkeys(
		keys: *mut crypto_authenc_keys,
		key: *const u8,
		keylen: core::ffi::c_uint,
	) -> i32;
	pub fn crypto_krb5enc_extractkeys(
		keys: *mut crypto_authenc_keys,
		key: *const u8,
		keylen: core::ffi::c_uint,
	) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
