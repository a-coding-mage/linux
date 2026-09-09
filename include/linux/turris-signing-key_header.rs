/* SPDX-License-Identifier: GPL-2.0 */
/*
 * 2025 by Marek Behún <kabel@kernel.org>
 */

// Translated from turris-signing-key.h.
// The Linux key and device types are supplied by external dependencies.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct key {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_KEYS")]
#[repr(C)]
pub struct turris_signing_key_subtype {
    pub key_size: u16,
    pub data_size: u8,
    pub sig_size: u8,
    pub public_key_size: u8,
    pub hash_algo: *const core::ffi::c_char,
    pub get_public_key: Option<
        unsafe extern "C" fn(key: *const key) -> *const core::ffi::c_void,
    >,
    pub sign: Option<
        unsafe extern "C" fn(
            key: *const key,
            msg: *const core::ffi::c_void,
            signature: *mut core::ffi::c_void,
        ) -> core::ffi::c_int,
    >,
}

#[cfg(feature = "CONFIG_KEYS")]
#[inline]
pub unsafe fn turris_signing_key_get_dev(key: *const key) -> *mut device {
    // The definition of `struct key`, including payload.data, is provided by
    // the external Linux key dependency and is intentionally not reproduced here.
    // C equivalent: return key->payload.data[1];
    let _ = key;
    core::ptr::null_mut()
}

#[cfg(feature = "CONFIG_KEYS")]
unsafe extern "C" {
    pub fn devm_turris_signing_key_create(
        dev: *mut device,
        subtype: *const turris_signing_key_subtype,
        desc: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
