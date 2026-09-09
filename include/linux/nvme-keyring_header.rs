/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2023 Hannes Reinecke, SUSE Labs
 */

/* C dependency: <linux/key.h> */

#[repr(C)]
pub struct key {
    _private: [u8; 0],
}

pub type key_serial_t = i32;

/* CONFIG_NVME_KEYRING-enabled declarations. */
#[cfg(feature = "CONFIG_NVME_KEYRING")]
extern "C" {
    pub fn nvme_tls_psk_refresh(
        keyring: *mut key,
        hostnqn: *const core::ffi::c_char,
        subnqn: *const core::ffi::c_char,
        hmac_id: u8,
        data: *mut u8,
        data_len: usize,
        digest: *const core::ffi::c_char,
    ) -> *mut key;

    pub fn nvme_tls_psk_default(
        keyring: *mut key,
        hostnqn: *const core::ffi::c_char,
        subnqn: *const core::ffi::c_char,
    ) -> key_serial_t;

    pub fn nvme_keyring_id() -> key_serial_t;

    pub fn nvme_tls_key_lookup(key_id: key_serial_t) -> *mut key;
}

/* !CONFIG_NVME_KEYRING: static inline fallback declarations. */
#[cfg(not(feature = "CONFIG_NVME_KEYRING"))]
#[inline]
pub unsafe fn nvme_tls_psk_refresh(
    _keyring: *mut key,
    _hostnqn: *const core::ffi::c_char,
    _subnqn: *mut core::ffi::c_char,
    _hmac_id: u8,
    _data: *mut u8,
    _data_len: usize,
    _digest: *const core::ffi::c_char,
) -> *mut key {
    /* ERR_PTR(-ENOTSUPP) */
    (-524isize) as *mut key
}

#[cfg(not(feature = "CONFIG_NVME_KEYRING"))]
#[inline]
pub unsafe fn nvme_tls_psk_default(
    _keyring: *mut key,
    _hostnqn: *const core::ffi::c_char,
    _subnqn: *const core::ffi::c_char,
) -> key_serial_t {
    0
}

#[cfg(not(feature = "CONFIG_NVME_KEYRING"))]
#[inline]
pub unsafe fn nvme_keyring_id() -> key_serial_t {
    0
}

#[cfg(not(feature = "CONFIG_NVME_KEYRING"))]
#[inline]
pub unsafe fn nvme_tls_key_lookup(_key_id: key_serial_t) -> *mut key {
    /* ERR_PTR(-ENOTSUPP) */
    (-524isize) as *mut key
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
