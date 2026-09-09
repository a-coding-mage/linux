/*
 * SPDX-License-Identifier: GPL-2.0
 * Copyright (C) 2018 Intel Corporation
 */

// C header guard: __ACPI_NFIT_H

// Conditional on CONFIG_ACPI_NFIT being enabled.
#[cfg(feature = "CONFIG_ACPI_NFIT")]
extern "C" {
    pub fn nfit_get_smbios_id(device_handle: u32, flags: *mut u16) -> i32;
}

// Fallback when CONFIG_ACPI_NFIT is disabled.
#[cfg(not(feature = "CONFIG_ACPI_NFIT"))]
#[inline]
pub unsafe fn nfit_get_smbios_id(device_handle: u32, flags: *mut u16) -> i32 {
    let _ = device_handle;
    let _ = flags;
    -EOPNOTSUPP
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
