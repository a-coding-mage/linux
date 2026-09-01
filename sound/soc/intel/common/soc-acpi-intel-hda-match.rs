// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2018, Intel Corporation.

/*
 * soc-acpi-intel-hda-match.c - tables and support for HDA+ACPI enumeration.
 *
 */

// Dependencies from C includes:
// #include <sound/soc-acpi.h>
// #include <sound/soc-acpi-intel-match.h>

extern "C" {
    pub static SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER: u64;
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub id: *const ::core::ffi::c_char,
    pub drv_name: *const ::core::ffi::c_char,
    pub sof_tplg_filename: *const ::core::ffi::c_char,
    pub tplg_quirk_mask: u64,
}

#[no_mangle]
pub static mut snd_soc_acpi_intel_hda_machines: [snd_soc_acpi_mach; 2] = [
    snd_soc_acpi_mach {
        /* .id is not used in this file */
        id: ::core::ptr::null(),
        drv_name: b"skl_hda_dsp_generic\0".as_ptr() as *const ::core::ffi::c_char,
        sof_tplg_filename: b"sof-hda-generic\0".as_ptr() as *const ::core::ffi::c_char, /* the tplg suffix is added at run time */
        tplg_quirk_mask: unsafe { SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER },
    },
    snd_soc_acpi_mach {
        id: ::core::ptr::null(),
        drv_name: ::core::ptr::null(),
        sof_tplg_filename: ::core::ptr::null(),
        tplg_quirk_mask: 0,
    },
];
// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_hda_machines);

// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("Intel Common ACPI Match module");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
