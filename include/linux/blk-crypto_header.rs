/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2019 Google LLC
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blk_crypto_mode_num {
    BLK_ENCRYPTION_MODE_INVALID,
    BLK_ENCRYPTION_MODE_AES_256_XTS,
    BLK_ENCRYPTION_MODE_AES_128_CBC_ESSIV,
    BLK_ENCRYPTION_MODE_ADIANTUM,
    BLK_ENCRYPTION_MODE_SM4_XTS,
    BLK_ENCRYPTION_MODE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blk_crypto_key_type {
    BLK_CRYPTO_KEY_TYPE_RAW = 0x1,
    BLK_CRYPTO_KEY_TYPE_HW_WRAPPED = 0x2,
}

pub const BLK_CRYPTO_MAX_RAW_KEY_SIZE: usize = 64;
pub const BLK_CRYPTO_MAX_HW_WRAPPED_KEY_SIZE: usize = 128;
pub const BLK_CRYPTO_MAX_ANY_KEY_SIZE: usize = if BLK_CRYPTO_MAX_RAW_KEY_SIZE > BLK_CRYPTO_MAX_HW_WRAPPED_KEY_SIZE {
    BLK_CRYPTO_MAX_RAW_KEY_SIZE
} else {
    BLK_CRYPTO_MAX_HW_WRAPPED_KEY_SIZE
};
pub const BLK_CRYPTO_SW_SECRET_SIZE: usize = 32;
pub const BLK_CRYPTO_CFG_ALLOW_HW: i32 = 1 << 0;

#[repr(C)]
pub struct blk_crypto_config {
    pub crypto_mode: blk_crypto_mode_num,
    pub data_unit_size: ::core::ffi::c_uint,
    pub dun_bytes: ::core::ffi::c_uint,
    pub key_type: blk_crypto_key_type,
    pub flags: ::core::ffi::c_int,
}

#[repr(C)]
pub struct blk_crypto_key {
    pub crypto_cfg: blk_crypto_config,
    pub data_unit_size_bits: ::core::ffi::c_uint,
    pub size: ::core::ffi::c_uint,
    pub bytes: [u8; BLK_CRYPTO_MAX_ANY_KEY_SIZE],
}

pub const BLK_CRYPTO_MAX_IV_SIZE: usize = 32;
pub const BLK_CRYPTO_DUN_ARRAY_SIZE: usize = BLK_CRYPTO_MAX_IV_SIZE / core::mem::size_of::<u64>();

#[repr(C)]
pub struct bio_crypt_ctx {
    pub bc_key: *const blk_crypto_key,
    pub bc_dun: [u64; BLK_CRYPTO_DUN_ARRAY_SIZE],
}

#[cfg(CONFIG_BLK_INLINE_ENCRYPTION)]
pub unsafe fn bio_has_crypt_ctx(bio: *mut bio) -> bool {
    unsafe { (*bio).bi_crypt_context }
}

#[cfg(CONFIG_BLK_INLINE_ENCRYPTION)]
pub unsafe fn bio_crypt_ctx(bio: *mut bio) -> *mut bio_crypt_ctx {
    unsafe { (*bio).bi_crypt_context }
}

#[cfg(CONFIG_BLK_INLINE_ENCRYPTION)]
extern "C" {
    pub fn bio_crypt_set_ctx(bio: *mut bio, key: *const blk_crypto_key,
        dun: *const u64, gfp_mask: gfp_t);
    pub fn bio_crypt_dun_is_contiguous(bc: *const bio_crypt_ctx,
        bytes: ::core::ffi::c_uint, next_dun: *const u64) -> bool;
    pub fn blk_crypto_init_key(blk_key: *mut blk_crypto_key, key_bytes: *const u8,
        key_size: usize, key_type: blk_crypto_key_type, crypto_mode: blk_crypto_mode_num,
        dun_bytes: ::core::ffi::c_uint, data_unit_size: ::core::ffi::c_uint,
        flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn blk_crypto_start_using_key(bdev: *mut block_device, key: *const blk_crypto_key) -> ::core::ffi::c_int;
    pub fn blk_crypto_evict_key(bdev: *mut block_device, key: *const blk_crypto_key);
    pub fn blk_crypto_config_supported_natively(bdev: *mut block_device, cfg: *const blk_crypto_config) -> bool;
    pub fn blk_crypto_derive_sw_secret(bdev: *mut block_device, eph_key: *const u8,
        eph_key_size: usize, sw_secret: *mut u8) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_BLK_INLINE_ENCRYPTION))]
pub unsafe fn bio_has_crypt_ctx(_bio: *mut bio) -> bool { false }

#[cfg(not(CONFIG_BLK_INLINE_ENCRYPTION))]
pub unsafe fn bio_crypt_ctx(_bio: *mut bio) -> *mut bio_crypt_ctx { core::ptr::null_mut() }

extern "C" {
    pub fn __blk_crypto_submit_bio(bio: *mut bio) -> bool;
    pub fn __bio_crypt_clone(dst: *mut bio, src: *mut bio, gfp_mask: gfp_t) -> ::core::ffi::c_int;
}

pub unsafe fn blk_crypto_submit_bio(bio: *mut bio) {
    if !bio_has_crypt_ctx(bio) || unsafe { __blk_crypto_submit_bio(bio) } {
        unsafe { submit_bio(bio) };
    }
}

pub unsafe fn bio_crypt_clone(dst: *mut bio, src: *mut bio, gfp_mask: gfp_t) -> ::core::ffi::c_int {
    if bio_has_crypt_ctx(src) {
        unsafe { __bio_crypt_clone(dst, src, gfp_mask) }
    } else {
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
