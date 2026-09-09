/* SPDX-License-Identifier: GPL-2.0 */
/*
 * OpenFirmware regulator support routines
 *
 */

/* C header guard: __LINUX_OF_REG_H */

use core::ffi::c_void;

#[repr(C)]
pub struct regulator_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regulator_init_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_regulator_match {
    pub name: *const core::ffi::c_char,
    pub driver_data: *mut c_void,
    pub init_data: *mut regulator_init_data,
    pub of_node: *mut device_node,
    pub desc: *const regulator_desc,
}

#[cfg(feature = "CONFIG_OF")]
extern "C" {
    pub fn of_get_regulator_init_data(
        dev: *mut device,
        node: *mut device_node,
        desc: *const regulator_desc,
    ) -> *mut regulator_init_data;

    pub fn of_regulator_match(
        dev: *mut device,
        node: *mut device_node,
        matches: *mut of_regulator_match,
        num_matches: u32,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_OF"))]
#[inline]
pub unsafe fn of_get_regulator_init_data(
    _dev: *mut device,
    _node: *mut device_node,
    _desc: *const regulator_desc,
) -> *mut regulator_init_data {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_OF"))]
#[inline]
pub unsafe fn of_regulator_match(
    _dev: *mut device,
    _node: *mut device_node,
    _matches: *mut of_regulator_match,
    _num_matches: u32,
) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
