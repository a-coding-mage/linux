// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-apci-intel-cfl-match.c - tables and support for CFL ACPI enumeration.
 *
 * Copyright (c) 2019, Intel Corporation.
 *
 */

// C dependencies:
// #include <sound/soc-acpi.h>
// #include <sound/soc-acpi-intel-match.h>

extern "C" {
    pub type snd_soc_acpi_mach;
}

#[no_mangle]
pub static mut snd_soc_acpi_intel_cfl_machines: [snd_soc_acpi_mach; 1] = [
    unsafe { ::core::mem::zeroed() },
];
// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_cfl_machines);

#[no_mangle]
pub static mut snd_soc_acpi_intel_cfl_sdw_machines: [snd_soc_acpi_mach; 1] = [
    unsafe { ::core::mem::zeroed() },
];
// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_cfl_sdw_machines);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
