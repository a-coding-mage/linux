/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: `struct device_node` is supplied by the Linux device-tree API.
// The C header guard `_PSERIES_OF_HELPERS_H` is omitted as Rust has module scoping.

use core::ffi::c_char;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn pseries_of_derive_parent(path: *const c_char) -> *mut device_node;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
