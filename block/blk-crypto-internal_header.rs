/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2019 Google LLC
 */

// C dependencies: <linux/bio.h>, <linux/blk-mq.h>

/* Represents a crypto mode supported by blk-crypto  */
#[repr(C)]
pub struct blk_crypto_mode {
    pub name: *const core::ffi::c_char, /* name of this mode, shown in sysfs */
    pub cipher_str: *const core::ffi::c_char, /* crypto API name (for fallback case) */
    pub keysize: core::ffi::c_uint, /* key size in bytes */
    pub security_strength: core::ffi::c_uint, /* security strength in bytes */
    pub ivsize: core::ffi::c_uint, /* iv size in bytes */
}

extern "C" {
    pub static blk_crypto_modes: [blk_crypto_mode; 0];
}

// CONFIG_BLK_INLINE_ENCRYPTION
extern "C" {
    pub fn blk_crypto_sysfs_register(disk: *mut gendisk) -> core::ffi::c_int;
    pub fn blk_crypto_sysfs_unregister(disk: *mut gendisk);
    pub fn bio_crypt_dun_increment(dun: *mut u64, inc: core::ffi::c_uint);
    pub fn bio_crypt_rq_ctx_compatible(rq: *mut request, bio: *mut bio) -> bool;
    pub fn bio_crypt_ctx_mergeable(
        bc1: *mut bio_crypt_ctx, bc1_bytes: core::ffi::c_uint,
        bc2: *mut bio_crypt_ctx,
    ) -> bool;
    pub fn blk_crypto_get_keyslot(
        profile: *mut blk_crypto_profile, key: *const blk_crypto_key,
        slot_ptr: *mut *mut blk_crypto_keyslot,
    ) -> blk_status_t;
    pub fn blk_crypto_put_keyslot(slot: *mut blk_crypto_keyslot);
    pub fn __blk_crypto_evict_key(profile: *mut blk_crypto_profile, key: *const blk_crypto_key) -> core::ffi::c_int;
    pub fn blk_crypto_ioctl(bdev: *mut block_device, cmd: core::ffi::c_uint, argp: *mut core::ffi::c_void) -> core::ffi::c_int;
}

// The following helpers are enabled when CONFIG_BLK_INLINE_ENCRYPTION is set.
#[inline]
pub unsafe fn bio_crypt_ctx_back_mergeable(req: *mut request, bio_ptr: *mut bio) -> bool {
    bio_crypt_ctx_mergeable((*req).crypt_ctx, blk_rq_bytes(req), (*bio_ptr).bi_crypt_context)
}

#[inline]
pub unsafe fn bio_crypt_ctx_front_mergeable(req: *mut request, bio_ptr: *mut bio) -> bool {
    bio_crypt_ctx_mergeable((*bio_ptr).bi_crypt_context, (*bio_ptr).bi_iter.bi_size, (*req).crypt_ctx)
}

#[inline]
pub unsafe fn bio_crypt_ctx_merge_rq(req: *mut request, next: *mut request) -> bool {
    bio_crypt_ctx_mergeable((*req).crypt_ctx, blk_rq_bytes(req), (*next).crypt_ctx)
}

#[inline]
pub unsafe fn blk_crypto_rq_set_defaults(rq: *mut request) {
    (*rq).crypt_ctx = core::ptr::null_mut();
    (*rq).crypt_keyslot = core::ptr::null_mut();
}

#[inline]
pub unsafe fn blk_crypto_rq_is_encrypted(rq: *mut request) -> bool { !(*rq).crypt_ctx.is_null() }

#[inline]
pub unsafe fn blk_crypto_rq_has_keyslot(rq: *mut request) -> bool { !(*rq).crypt_keyslot.is_null() }

#[inline]
pub unsafe fn blk_crypto_supported(bio_ptr: *mut bio) -> bool {
    blk_crypto_config_supported_natively((*bio_ptr).bi_bdev,
        &mut (*(*bio_ptr).bi_crypt_context).bc_key.crypto_cfg)
}

// When CONFIG_BLK_INLINE_ENCRYPTION is unset, the helpers above have these semantics:
// blk_crypto_sysfs_register returns 0; unregister is a no-op; merge checks return true;
// rq_set_defaults is a no-op; encrypted/keyslot checks return false;
// blk_crypto_ioctl returns -ENOTTY; blk_crypto_supported returns false.

extern "C" {
    pub fn blk_crypto_config_supported_natively(bdev: *mut block_device, cfg: *mut crypto_cfg) -> bool;
    pub fn __bio_crypt_advance(bio: *mut bio, bytes: core::ffi::c_uint);
    pub fn __bio_crypt_free_ctx(bio: *mut bio);
    pub fn __blk_crypto_rq_get_keyslot(rq: *mut request) -> blk_status_t;
    pub fn __blk_crypto_rq_put_keyslot(rq: *mut request);
    pub fn __blk_crypto_free_request(rq: *mut request);
    pub fn __blk_crypto_rq_bio_prep(rq: *mut request, bio: *mut bio, gfp_mask: gfp_t) -> core::ffi::c_int;
    pub fn blk_crypto_fallback_bio_prep(bio: *mut bio) -> bool;
}

#[inline]
pub unsafe fn bio_crypt_advance(bio_ptr: *mut bio, bytes: core::ffi::c_uint) {
    if bio_has_crypt_ctx(bio_ptr) { __bio_crypt_advance(bio_ptr, bytes); }
}

#[inline]
pub unsafe fn bio_crypt_free_ctx(bio_ptr: *mut bio) {
    if bio_has_crypt_ctx(bio_ptr) { __bio_crypt_free_ctx(bio_ptr); }
}

#[inline]
pub unsafe fn bio_crypt_do_front_merge(rq: *mut request, bio_ptr: *mut bio) {
    // CONFIG_BLK_INLINE_ENCRYPTION
    if bio_has_crypt_ctx(bio_ptr) {
        core::ptr::copy_nonoverlapping(
            (*bio_ptr).bi_crypt_context, (*rq).crypt_ctx,
            core::mem::size_of_val(&(*(*rq).crypt_ctx).bc_dun));
    }
}

#[inline]
pub unsafe fn blk_crypto_rq_get_keyslot(rq: *mut request) -> blk_status_t {
    if blk_crypto_rq_is_encrypted(rq) { __blk_crypto_rq_get_keyslot(rq) } else { BLK_STS_OK }
}

#[inline]
pub unsafe fn blk_crypto_rq_put_keyslot(rq: *mut request) {
    if blk_crypto_rq_has_keyslot(rq) { __blk_crypto_rq_put_keyslot(rq); }
}

#[inline]
pub unsafe fn blk_crypto_free_request(rq: *mut request) {
    if blk_crypto_rq_is_encrypted(rq) { __blk_crypto_free_request(rq); }
}

/**
 * blk_crypto_rq_bio_prep - Prepare a request's crypt_ctx when its first bio
 *                            is inserted
 * @rq: The request to prepare
 * @bio: The first bio being inserted into the request
 * @gfp_mask: Memory allocation flags
 *
 * Return: 0 on success, -ENOMEM if out of memory.  -ENOMEM is only possible if
 *         @gfp_mask doesn't include %__GFP_DIRECT_RECLAIM.
 */
#[inline]
pub unsafe fn blk_crypto_rq_bio_prep(rq: *mut request, bio_ptr: *mut bio, gfp_mask: gfp_t) -> core::ffi::c_int {
    if bio_has_crypt_ctx(bio_ptr) { __blk_crypto_rq_bio_prep(rq, bio_ptr, gfp_mask) } else { 0 }
}

#[inline]
pub unsafe fn blk_crypto_supported_without_inline_encryption(_bio: *mut bio) -> bool { false }

// CONFIG_BLK_INLINE_ENCRYPTION_FALLBACK
extern "C" {
    pub fn blk_crypto_fallback_start_using_mode(mode_num: blk_crypto_mode_num) -> core::ffi::c_int;
    pub fn blk_crypto_fallback_evict_key(key: *const blk_crypto_key) -> core::ffi::c_int;
}
// When CONFIG_BLK_INLINE_ENCRYPTION_FALLBACK is unset, start_using_mode warns
// once and returns -ENOPKG, while fallback_evict_key returns 0.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
