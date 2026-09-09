/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency: declarations and constants supplied by the Linux ACPI/device
// environment are expected to be available to the including translation unit.

#[cfg(CONFIG_ACPI_VIOT)]
unsafe extern "C" {
    pub fn acpi_viot_early_init();
    pub fn acpi_viot_init();
    pub fn viot_iommu_configure(dev: *mut device) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_ACPI_VIOT))]
pub unsafe extern "C" fn acpi_viot_early_init() {}

#[cfg(not(CONFIG_ACPI_VIOT))]
pub unsafe extern "C" fn acpi_viot_init() {}

#[cfg(not(CONFIG_ACPI_VIOT))]
pub unsafe extern "C" fn viot_iommu_configure(_dev: *mut device) -> ::core::ffi::c_int {
    -ENODEV
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
