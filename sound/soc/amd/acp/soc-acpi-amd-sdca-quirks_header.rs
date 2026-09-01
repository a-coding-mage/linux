/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * soc-acpi-amd-sdca-quirks.h - tables and support for SDCA quirks
 *
 * Copyright(c) 2025 Advanced Micro Devices, Inc. All rights reserved.
 *
 */

// C header guard omitted in Rust:
// _SND_SOC_ACPI_AMD_SDCA_QUIRKS

// Original C condition:
// #if IS_ENABLED(CONFIG_SND_SOC_ACPI_AMD_SDCA_QUIRKS)

#[cfg(CONFIG_SND_SOC_ACPI_AMD_SDCA_QUIRKS)]
extern "C" {
    pub fn snd_soc_acpi_amd_sdca_is_device_rt712_vb(arg: *mut core::ffi::c_void) -> bool;
}

// Original C fallback for:
// #else

#[cfg(not(CONFIG_SND_SOC_ACPI_AMD_SDCA_QUIRKS))]
#[inline]
pub unsafe fn snd_soc_acpi_amd_sdca_is_device_rt712_vb(_arg: *mut core::ffi::c_void) -> bool {
    false
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
