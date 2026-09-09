/* SPDX-License-Identifier: GPL-2.0 */

//! Common OMAP board-device declarations.
//!
//! The `menelaus_platform_data` type is supplied by the translated
//! `linux/mfd/menelaus.h` dependency.

use core::ffi::c_void;

extern "C" {
    pub fn n8x0_legacy_init() -> *mut c_void;

    pub static mut n8x0_menelaus_platform_data: crate::menelaus_platform_data;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
