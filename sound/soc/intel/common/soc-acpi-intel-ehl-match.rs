// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-acpi-intel-ehl-match.c - tables and support for EHL ACPI enumeration.
 *
 * Copyright (c) 2019, Intel Corporation.
 *
 */

// C dependencies:
// #include <sound/soc-acpi.h>
// #include <sound/soc-acpi-intel-match.h>

pub static mut snd_soc_acpi_intel_ehl_machines: [snd_soc_acpi_mach; 2] = [
    snd_soc_acpi_mach {
        id: b"10EC5660\0".as_ptr() as *const _,
        drv_name: b"ehl_rt5660\0".as_ptr() as *const _,
        sof_tplg_filename: b"sof-ehl-rt5660.tplg\0".as_ptr() as *const _,
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];

// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_ehl_machines);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
