/* SPDX-License-Identifier: GPL-2.0 */

// Header guard: LINUX_DEVICE_ID_OF_H

/*
 * Struct used for matching a device
 */
#[repr(C)]
pub struct of_device_id {
    pub name: [core::ffi::c_char; 32],
    pub type_: [core::ffi::c_char; 32],
    pub compatible: [core::ffi::c_char; 128],
    pub data: *const core::ffi::c_void,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
