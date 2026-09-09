/* SPDX-License-Identifier: GPL-2.0+ */

// Forward declarations from the C header:
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn ap_cp_unique_name(
        dev: *mut device,
        np: *mut device_node,
        name: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
