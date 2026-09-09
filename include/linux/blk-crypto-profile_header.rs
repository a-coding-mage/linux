/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2019 Google LLC
 */

// Translated from linux/blk-crypto-profile.h.  C includes and their supplied
// symbols are intentionally left as external dependencies.

use core::ffi::c_int;

#[repr(C)]
pub struct blk_crypto_ll_ops {
    pub keyslot_program: Option<unsafe extern "C" fn(
        profile: *mut blk_crypto_profile,
        key: *const blk_crypto_key,
        slot: core::ffi::c_uint,
    ) -> c_int>,
    pub keyslot_evict: Option<unsafe extern "C" fn(
        profile: *mut blk_crypto_profile,
        key: *const blk_crypto_key,
        slot: core::ffi::c_uint,
    ) -> c_int>,
    pub derive_sw_secret: Option<unsafe extern "C" fn(
        profile: *mut blk_crypto_profile,
        eph_key: *const u8,
        eph_key_size: usize,
        sw_secret: *mut u8,
    ) -> c_int>,
    pub import_key: Option<unsafe extern "C" fn(
        profile: *mut blk_crypto_profile,
        raw_key: *const u8,
        raw_key_size: usize,
        lt_key: *mut u8,
    ) -> c_int>,
    pub generate_key: Option<unsafe extern "C" fn(
        profile: *mut blk_crypto_profile,
        lt_key: *mut u8,
    ) -> c_int>,
    pub prepare_key: Option<unsafe extern "C" fn(
        profile: *mut blk_crypto_profile,
        lt_key: *const u8,
        lt_key_size: usize,
        eph_key: *mut u8,
    ) -> c_int>,
}

#[repr(C)]
pub struct blk_crypto_profile {
    pub ll_ops: blk_crypto_ll_ops,
    pub max_dun_bytes_supported: core::ffi::c_uint,
    pub key_types_supported: core::ffi::c_uint,
    pub modes_supported: [core::ffi::c_uint; BLK_ENCRYPTION_MODE_MAX],
    pub dev: *mut device,
    pub num_slots: core::ffi::c_uint,
    pub lock: rw_semaphore,
    pub lockdep_key: lock_class_key,
    pub idle_slots_wait_queue: wait_queue_head_t,
    pub idle_slots: list_head,
    pub idle_slots_lock: spinlock_t,
    pub slot_hashtable: *mut hlist_head,
    pub log_slot_ht_size: core::ffi::c_uint,
    pub slots: *mut blk_crypto_keyslot,
}

extern "C" {
    pub fn blk_crypto_profile_init(
        profile: *mut blk_crypto_profile,
        num_slots: core::ffi::c_uint,
    ) -> c_int;
    pub fn devm_blk_crypto_profile_init(
        dev: *mut device,
        profile: *mut blk_crypto_profile,
        num_slots: core::ffi::c_uint,
    ) -> c_int;
    pub fn blk_crypto_keyslot_index(slot: *mut blk_crypto_keyslot) -> core::ffi::c_uint;
    pub fn blk_crypto_reprogram_all_keys(profile: *mut blk_crypto_profile);
    pub fn blk_crypto_profile_destroy(profile: *mut blk_crypto_profile);
    pub fn blk_crypto_import_key(
        profile: *mut blk_crypto_profile,
        raw_key: *const u8,
        raw_key_size: usize,
        lt_key: *mut u8,
    ) -> c_int;
    pub fn blk_crypto_generate_key(
        profile: *mut blk_crypto_profile,
        lt_key: *mut u8,
    ) -> c_int;
    pub fn blk_crypto_prepare_key(
        profile: *mut blk_crypto_profile,
        lt_key: *const u8,
        lt_key_size: usize,
        eph_key: *mut u8,
    ) -> c_int;
    pub fn blk_crypto_intersect_capabilities(
        parent: *mut blk_crypto_profile,
        child: *const blk_crypto_profile,
    );
    pub fn blk_crypto_has_capabilities(
        target: *const blk_crypto_profile,
        reference: *const blk_crypto_profile,
    ) -> bool;
    pub fn blk_crypto_update_capabilities(
        dst: *mut blk_crypto_profile,
        src: *const blk_crypto_profile,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
