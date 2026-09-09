/* SPDX-License-Identifier: GPL-2.0-or-later */

// C forward declarations:
#[repr(C)]
pub struct dsa_port {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dsa_switch {
    _private: [u8; 0],
}

extern "C" {
    pub fn dsa_port_devlink_setup(dp: *mut dsa_port) -> ::core::ffi::c_int;
    pub fn dsa_port_devlink_teardown(dp: *mut dsa_port);
    pub fn dsa_switch_devlink_register(ds: *mut dsa_switch);
    pub fn dsa_switch_devlink_unregister(ds: *mut dsa_switch);
    pub fn dsa_switch_devlink_alloc(ds: *mut dsa_switch) -> ::core::ffi::c_int;
    pub fn dsa_switch_devlink_free(ds: *mut dsa_switch);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
