/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) ST-Ericsson SA 2011
 * Author: Lee Jones <lee.jones@linaro.org> for ST-Ericsson.
 */

// Dependency supplied by <linux/device.h>.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute_group {
    _private: [u8; 0],
}

#[repr(C)]
pub struct soc_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct soc_device_attribute {
    pub machine: *const core::ffi::c_char,
    pub family: *const core::ffi::c_char,
    pub revision: *const core::ffi::c_char,
    pub serial_number: *const core::ffi::c_char,
    pub soc_id: *const core::ffi::c_char,
    pub data: *const core::ffi::c_void,
    pub custom_attr_group: *const attribute_group,
}

/// soc_device_register - register SoC as a device
/// @soc_plat_dev_attr: Attributes passed from platform to be attributed to a SoC
///
/// Returns:
/// - %NULL if the SoC bus is not yet registered;
/// - on success, the newly allocated &struct soc_device pointer;
/// - on failure, a negative error code as an ERR_PTR().
extern "C" {
    pub fn soc_device_register(
        soc_plat_dev_attr: *mut soc_device_attribute,
    ) -> *mut soc_device;

    /// soc_device_unregister - unregister SoC device
    /// @soc_dev: SoC device to be unregistered
    pub fn soc_device_unregister(soc_dev: *mut soc_device);

    /// soc_device_to_device - helper function to fetch struct device
    /// @soc: Previously registered SoC device container
    ///
    /// Returns: &struct device pointer for this @soc
    pub fn soc_device_to_device(soc: *mut soc_device) -> *mut device;

    /// soc_attr_read_machine - retrieve the machine model and store it in
    ///                         the soc_device_attribute structure
    /// @soc_dev_attr: SoC attribute structure to store the model in
    ///
    /// Returns:
    /// 0 on success, negative error number on failure.
    pub fn soc_attr_read_machine(soc_dev_attr: *mut soc_device_attribute) -> core::ffi::c_int;
}

#[cfg(CONFIG_SOC_BUS)]
extern "C" {
    pub fn soc_device_match(
        matches: *const soc_device_attribute,
    ) -> *const soc_device_attribute;
}

#[cfg(not(CONFIG_SOC_BUS))]
#[inline]
pub unsafe fn soc_device_match(
    _matches: *const soc_device_attribute,
) -> *const soc_device_attribute {
    core::ptr::null()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
