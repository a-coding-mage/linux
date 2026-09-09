/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * The MIPI SDCA specification is available for public downloads at
 * https://www.mipi.org/mipi-sdca-v1-0-download
 *
 * Copyright(c) 2024 Intel Corporation
 */

// C dependencies: linux/types.h and linux/kconfig.h.

pub const SDCA_MAX_FUNCTION_COUNT: usize = 8;

// Forward declarations supplied by other files.
#[repr(C)]
pub struct acpi_table_swft {
    _private: [u8; 0],
}
#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdw_slave {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdca_dev {
    _private: [u8; 0],
}

/**
 * struct sdca_function_desc - short descriptor for an SDCA Function
 * @node: firmware node for the Function.
 * @func_dev: pointer to SDCA function device.
 * @name: Human-readable string.
 * @type: Function topology type.
 * @adr: ACPI address (used for SDCA register access).
 * @duplicate: Internal flag to indicate if other functions of the same type
 * exist.
 */
#[repr(C)]
pub struct sdca_function_desc {
    pub node: *mut fwnode_handle,
    pub func_dev: *mut sdca_dev,
    pub name: *const core::ffi::c_char,
    pub r#type: u32,
    pub adr: u8,
    pub duplicate: bool,
}

/**
 * struct sdca_device_data - structure containing all SDCA related information
 * @interface_revision: Value read from _DSD property, mainly to check
 * for changes between silicon versions.
 * @num_functions: Total number of supported SDCA functions. Invalid/unsupported
 * functions will be skipped.
 * @function: Array of function descriptors.
 * @swft: Pointer to the SWFT table, if available.
 */
#[repr(C)]
pub struct sdca_device_data {
    pub interface_revision: u32,
    pub num_functions: i32,
    pub function: [sdca_function_desc; SDCA_MAX_FUNCTION_COUNT],
    pub swft: *mut acpi_table_swft,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sdca_quirk {
    SDCA_QUIRKS_RT712_VB,
    SDCA_QUIRKS_SKIP_FUNC_TYPE_PATCHING,
}

// The following declarations are enabled when both CONFIG_ACPI and
// CONFIG_SND_SOC_SDCA are enabled in the C build.
#[cfg(all(feature = "acpi", feature = "snd_soc_sdca"))]
unsafe extern "C" {
    pub fn sdca_lookup_functions(slave: *mut sdw_slave);
    pub fn sdca_lookup_swft(slave: *mut sdw_slave);
    pub fn sdca_lookup_interface_revision(slave: *mut sdw_slave);
    pub fn sdca_device_quirk_match(slave: *mut sdw_slave, quirk: sdca_quirk) -> bool;
    pub fn sdca_dev_register_functions(slave: *mut sdw_slave) -> i32;
    pub fn sdca_dev_unregister_functions(slave: *mut sdw_slave);
}

// Fallbacks corresponding to the C !IS_ENABLED(CONFIG_ACPI) ||
// !IS_ENABLED(CONFIG_SND_SOC_SDCA) branch.
#[cfg(not(all(feature = "acpi", feature = "snd_soc_sdca")))]
pub unsafe fn sdca_lookup_functions(_slave: *mut sdw_slave) {}

#[cfg(not(all(feature = "acpi", feature = "snd_soc_sdca")))]
pub unsafe fn sdca_lookup_swft(_slave: *mut sdw_slave) {}

#[cfg(not(all(feature = "acpi", feature = "snd_soc_sdca")))]
pub unsafe fn sdca_lookup_interface_revision(_slave: *mut sdw_slave) {}

#[cfg(not(all(feature = "acpi", feature = "snd_soc_sdca")))]
pub unsafe fn sdca_device_quirk_match(_slave: *mut sdw_slave, _quirk: sdca_quirk) -> bool {
    false
}

#[cfg(not(all(feature = "acpi", feature = "snd_soc_sdca")))]
pub unsafe fn sdca_dev_register_functions(_slave: *mut sdw_slave) -> i32 {
    0
}

#[cfg(not(all(feature = "acpi", feature = "snd_soc_sdca")))]
pub unsafe fn sdca_dev_unregister_functions(_slave: *mut sdw_slave) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
