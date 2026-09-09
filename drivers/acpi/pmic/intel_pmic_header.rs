/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: definitions corresponding to <acpi/acpi_lpat.h> and
// the kernel types `struct regmap`, `struct device`, and `acpi_handle` are
// supplied by other translation units.

#[repr(C)]
pub struct pmic_table {
    pub address: core::ffi::c_int, // operation region address
    pub reg: core::ffi::c_int,     // corresponding thermal register
    pub bit: core::ffi::c_int,     // control bit for power
}

#[repr(C)]
pub struct intel_pmic_opregion_data {
    pub get_power: Option<unsafe extern "C" fn(
        r: *mut regmap,
        reg: core::ffi::c_int,
        bit: core::ffi::c_int,
        value: *mut u64,
    ) -> core::ffi::c_int>,
    pub update_power: Option<unsafe extern "C" fn(
        r: *mut regmap,
        reg: core::ffi::c_int,
        bit: core::ffi::c_int,
        on: bool,
    ) -> core::ffi::c_int>,
    pub get_raw_temp: Option<unsafe extern "C" fn(
        r: *mut regmap,
        reg: core::ffi::c_int,
    ) -> core::ffi::c_int>,
    pub update_aux: Option<unsafe extern "C" fn(
        r: *mut regmap,
        reg: core::ffi::c_int,
        raw_temp: core::ffi::c_int,
    ) -> core::ffi::c_int>,
    pub get_policy: Option<unsafe extern "C" fn(
        r: *mut regmap,
        reg: core::ffi::c_int,
        bit: core::ffi::c_int,
        value: *mut u64,
    ) -> core::ffi::c_int>,
    pub update_policy: Option<unsafe extern "C" fn(
        r: *mut regmap,
        reg: core::ffi::c_int,
        bit: core::ffi::c_int,
        enable: core::ffi::c_int,
    ) -> core::ffi::c_int>,
    pub exec_mipi_pmic_seq_element: Option<unsafe extern "C" fn(
        r: *mut regmap,
        i2c_address: u16,
        reg_address: u32,
        value: u32,
        mask: u32,
    ) -> core::ffi::c_int>,
    pub lpat_raw_to_temp: Option<unsafe extern "C" fn(
        lpat_table: *mut acpi_lpat_conversion_table,
        raw: core::ffi::c_int,
    ) -> core::ffi::c_int>,
    pub power_table: *const pmic_table,
    pub power_table_count: core::ffi::c_int,
    pub thermal_table: *const pmic_table,
    pub thermal_table_count: core::ffi::c_int,
    // For generic exec_mipi_pmic_seq_element handling
    pub pmic_i2c_address: core::ffi::c_int,
}

// Opaque types supplied by other translation units.
pub enum regmap {}
pub enum acpi_lpat_conversion_table {}
pub enum device {}

pub type acpi_handle = *mut core::ffi::c_void;

unsafe extern "C" {
    pub fn intel_pmic_install_opregion_handler(
        dev: *mut device,
        handle: acpi_handle,
        regmap: *mut regmap,
        d: *const intel_pmic_opregion_data,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
