/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2023, Linaro Limited
 */

// Dependency intent from the C header:
// <linux/blk-crypto.h> and <linux/types.h>.

use core::ffi::c_void;

#[repr(C)]
pub struct qcom_ice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct blk_crypto_key {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

// Opaque dependency type corresponding to enum blk_crypto_key_type.
pub type blk_crypto_key_type = i32;

extern "C" {
    pub fn qcom_ice_enable(ice: *mut qcom_ice) -> i32;
    pub fn qcom_ice_resume(ice: *mut qcom_ice) -> i32;
    pub fn qcom_ice_suspend(ice: *mut qcom_ice) -> i32;
    pub fn qcom_ice_program_key(
        ice: *mut qcom_ice,
        slot: u32,
        blk_key: *const blk_crypto_key,
    ) -> i32;
    pub fn qcom_ice_evict_key(ice: *mut qcom_ice, slot: i32) -> i32;
    pub fn qcom_ice_get_supported_key_type(ice: *mut qcom_ice) -> blk_crypto_key_type;
    pub fn qcom_ice_derive_sw_secret(
        ice: *mut qcom_ice,
        eph_key: *const u8,
        eph_key_size: usize,
        sw_secret: *mut u8,
    ) -> i32;
    pub fn qcom_ice_generate_key(ice: *mut qcom_ice, lt_key: *mut u8) -> i32;
    pub fn qcom_ice_prepare_key(
        ice: *mut qcom_ice,
        lt_key: *const u8,
        lt_key_size: usize,
        eph_key: *mut u8,
    ) -> i32;
    pub fn qcom_ice_import_key(
        ice: *mut qcom_ice,
        raw_key: *const u8,
        raw_key_size: usize,
        lt_key: *mut u8,
    ) -> i32;
    pub fn devm_of_qcom_ice_get(dev: *mut device) -> *mut qcom_ice;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
