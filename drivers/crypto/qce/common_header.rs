/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2010-2014, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the surrounding translation.

/* xts du size */
pub const QCE_SECTOR_SIZE: u32 = 512;

/* key size in bytes */
pub const QCE_SHA_HMAC_KEY_SIZE: u32 = 64;
pub const QCE_MAX_CIPHER_KEY_SIZE: u32 = AES_KEYSIZE_256;

/* IV length in bytes */
pub const QCE_AES_IV_LENGTH: u32 = AES_BLOCK_SIZE;
/* max of AES_BLOCK_SIZE */
pub const QCE_MAX_IV_SIZE: u32 = AES_BLOCK_SIZE;

/* maximum nonce bytes */
pub const QCE_MAX_NONCE: u32 = 16;
pub const QCE_MAX_NONCE_WORDS: u32 = QCE_MAX_NONCE / core::mem::size_of::<u32>() as u32;

/* burst size alignment requirement */
pub const QCE_MAX_ALIGN_SIZE: u32 = 64;

/* cipher algorithms */
pub const QCE_ALG_AES: u32 = 1u32 << 2;

/* hash and hmac algorithms */
pub const QCE_HASH_SHA256: u32 = 1u32 << 4;
pub const QCE_HASH_SHA256_HMAC: u32 = 1u32 << 6;
pub const QCE_HASH_AES_CMAC: u32 = 1u32 << 7;

/* cipher modes */
pub const QCE_MODE_CBC: u32 = 1u32 << 8;
pub const QCE_MODE_ECB: u32 = 1u32 << 9;
pub const QCE_MODE_CTR: u32 = 1u32 << 10;
pub const QCE_MODE_XTS: u32 = 1u32 << 11;
pub const QCE_MODE_CCM: u32 = 1u32 << 12;
pub const QCE_MODE_MASK: u32 = 0x1f00;

pub const QCE_MODE_CCM_RFC4309: u32 = 1u32 << 13;

/* cipher encryption/decryption operations */
pub const QCE_ENCRYPT: u32 = 1u32 << 30;
pub const QCE_DECRYPT: u32 = 1u32 << 31;

#[inline]
pub const fn IS_AES(flags: u32) -> u32 { flags & QCE_ALG_AES }
#[inline]
pub const fn IS_SHA256(flags: u32) -> u32 { flags & QCE_HASH_SHA256 }
#[inline]
pub const fn IS_SHA256_HMAC(flags: u32) -> u32 { flags & QCE_HASH_SHA256_HMAC }
#[inline]
pub const fn IS_CMAC(flags: u32) -> u32 { flags & QCE_HASH_AES_CMAC }
#[inline]
pub const fn IS_SHA(flags: u32) -> u32 { IS_SHA256(flags) }
#[inline]
pub const fn IS_SHA_HMAC(flags: u32) -> u32 { IS_SHA256_HMAC(flags) }
#[inline]
pub const fn IS_CBC(mode: u32) -> u32 { mode & QCE_MODE_CBC }
#[inline]
pub const fn IS_CTR(mode: u32) -> u32 { mode & QCE_MODE_CTR }
#[inline]
pub const fn IS_XTS(mode: u32) -> u32 { mode & QCE_MODE_XTS }
#[inline]
pub const fn IS_CCM(mode: u32) -> u32 { mode & QCE_MODE_CCM }
#[inline]
pub const fn IS_CCM_RFC4309(mode: u32) -> u32 { mode & QCE_MODE_CCM_RFC4309 }
#[inline]
pub const fn IS_ENCRYPT(dir: u32) -> u32 { dir & QCE_ENCRYPT }
#[inline]
pub const fn IS_DECRYPT(dir: u32) -> u32 { dir & QCE_DECRYPT }

#[repr(C)]
pub struct qce_alg_template {
    pub entry: list_head,
    pub crypto_alg_type: u32,
    pub alg_flags: usize,
    pub std_iv: *const u32,
    pub alg: qce_alg_template_alg,
    pub qce: *mut qce_device,
    pub hash_zero: *const u8,
    pub digest_size: u32,
}

#[repr(C)]
pub union qce_alg_template_alg {
    pub skcipher: skcipher_alg,
    pub ahash: ahash_alg,
    pub aead: aead_alg,
}

extern "C" {
    pub fn qce_cpu_to_be32p_array(dst: *mut u32, src: *const u8, len: u32);
    pub fn qce_check_status(qce: *mut qce_device, status: *mut u32) -> i32;
    pub fn qce_get_version(qce: *mut qce_device, major: *mut u32, minor: *mut u32, step: *mut u32);
    pub fn qce_start(async_req: *mut crypto_async_request, type_: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
