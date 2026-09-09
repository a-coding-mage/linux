/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Copyright (C) 2024-2025, Ventana Micro Systems Inc.
 *	Author: Sunil V L <sunilvl@ventanamicro.com>
 */

// C header dependencies and build-time CONFIG_* conditions are represented
// below with Rust declarations and cfg attributes.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_ACPI_RIMT")]
unsafe extern "C" {
    pub fn rimt_iommu_register(dev: *mut device) -> i32;
}

#[cfg(not(feature = "CONFIG_ACPI_RIMT"))]
#[inline]
pub unsafe fn rimt_iommu_register(_dev: *mut device) -> i32 {
    -19 // ENODEV
}

#[cfg(all(feature = "CONFIG_IOMMU_API", feature = "CONFIG_ACPI_RIMT"))]
unsafe extern "C" {
    pub fn rimt_iommu_configure_id(dev: *mut device, id_in: *const u32) -> i32;
}

#[cfg(not(all(feature = "CONFIG_IOMMU_API", feature = "CONFIG_ACPI_RIMT")))]
#[inline]
pub unsafe fn rimt_iommu_configure_id(_dev: *mut device, _id_in: *const u32) -> i32 {
    -19 // ENODEV
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
