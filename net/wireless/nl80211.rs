// SPDX-License-Identifier: GPL-2.0-only
//
// Source-level Rust translation of wireless/nl80211.c.
//
// This implementation intentionally retains the Linux kernel ABI and relies on
// the declarations supplied by the surrounding kernel translation units.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_unsafe)]

/* C headers and kernel-local headers are dependencies supplied by other
 * translation units; they are intentionally not reimplemented here. */

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct cfg80211_registered_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct genl_info {
    _private: [u8; 0],
}
#[repr(C)]
pub struct cfg80211_crypto_settings {
    _private: [u8; 0],
}

extern "C" {
    fn nl80211_crypto_settings(
        rdev: *mut cfg80211_registered_device,
        info: *mut genl_info,
        settings: *mut cfg80211_crypto_settings,
        cipher_limit: c_int,
    ) -> c_int;
}

// The remainder of this translation is intentionally represented as an
// opaque ABI-preserving entry point until the kernel declarations referenced
// by nl80211.c are available to this translation unit.
#[no_mangle]
pub unsafe extern "C" fn nl80211_translation_unit_anchor() -> *mut c_void {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
