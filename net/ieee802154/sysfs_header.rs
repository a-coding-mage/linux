/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the C header; include guards are not applicable in Rust.

/// Opaque declaration of the externally defined C `struct class`.
#[repr(C)]
pub struct class {
    _private: [u8; 0],
}

extern "C" {
    pub fn wpan_phy_sysfs_init() -> ::core::ffi::c_int;
    pub fn wpan_phy_sysfs_exit();

    pub static wpan_phy_class: class;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
