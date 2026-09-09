/* SPDX-License-Identifier: GPL-2.0-only */

use core::ffi::c_char;
use core::ffi::c_int;
use core::ffi::c_void;

/*
 * enum zl3073x_fw_component_id - Identifiers for possible flash components
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum zl3073x_fw_component_id {
    ZL_FW_COMPONENT_INVALID = -1,
    ZL_FW_COMPONENT_UTIL = 0,
    ZL_FW_COMPONENT_FW1,
    ZL_FW_COMPONENT_FW2,
    ZL_FW_COMPONENT_FW3,
    ZL_FW_COMPONENT_CFG0,
    ZL_FW_COMPONENT_CFG1,
    ZL_FW_COMPONENT_CFG2,
    ZL_FW_COMPONENT_CFG3,
    ZL_FW_COMPONENT_CFG4,
    ZL_FW_COMPONENT_CFG5,
    ZL_FW_COMPONENT_CFG6,
    ZL_FW_NUM_COMPONENTS,
}

/**
 * struct zl3073x_fw_component - Firmware component
 * @id: Flash component ID
 * @size: Size of the buffer
 * @data: Pointer to buffer with component data
 */
#[repr(C)]
pub struct zl3073x_fw_component {
    pub id: zl3073x_fw_component_id,
    pub size: usize,
    pub data: *mut c_void,
}

/**
 * struct zl3073x_fw - Firmware bundle
 * @component: firmware components array
 */
#[repr(C)]
pub struct zl3073x_fw {
    pub component: [*mut zl3073x_fw_component; ZL_FW_NUM_COMPONENTS as usize],
}

unsafe extern "C" {
    pub fn zl3073x_fw_load(
        zldev: *mut zl3073x_dev,
        data: *const c_char,
        size: usize,
        extack: *mut netlink_ext_ack,
    ) -> *mut zl3073x_fw;
    pub fn zl3073x_fw_free(fw: *mut zl3073x_fw);

    pub fn zl3073x_fw_flash(
        zldev: *mut zl3073x_dev,
        zlfw: *mut zl3073x_fw,
        extack: *mut netlink_ext_ack,
    ) -> c_int;
}

// External types supplied by other dependencies.
extern "C" {
    pub type zl3073x_dev;
    pub type netlink_ext_ack;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
