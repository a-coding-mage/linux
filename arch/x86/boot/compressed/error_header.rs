/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header BOOT_COMPRESSED_ERROR_H.
// The original declarations use the Linux __noreturn and __cold attributes;
// those build-time attributes are preserved here as comments.

use core::ffi::c_char;

unsafe extern "C" {
    pub fn warn(m: *const c_char);
    pub fn error(m: *mut c_char) -> !;
    pub fn panic(fmt: *const c_char, ... ) -> !;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
