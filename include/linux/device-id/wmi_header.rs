/* SPDX-License-Identifier: GPL-2.0 */

/* WMI */

pub const WMI_MODULE_PREFIX: &str = "wmi:";

/**
 * struct wmi_device_id - WMI device identifier
 * @guid_string: 36 char string of the form fa50ff2b-f2e8-45de-83fa-65417f2f49ba
 * @context: pointer to driver specific data
 */
#[repr(C)]
pub struct wmi_device_id {
    pub guid_string: [core::ffi::c_char; UUID_STRING_LEN + 1],
    pub context: *const core::ffi::c_void,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
