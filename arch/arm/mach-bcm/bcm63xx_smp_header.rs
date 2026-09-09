/* SPDX-License-Identifier: GPL-2.0 */

// Forward declaration of the externally defined device-tree node type.
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    pub fn bcm63xx_pmb_power_on_cpu(dn: *mut device_node) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
