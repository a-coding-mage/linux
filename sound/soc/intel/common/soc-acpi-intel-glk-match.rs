// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-acpi-intel-glk-match.c - tables and support for GLK ACPI enumeration.
 *
 * Copyright (c) 2018, Intel Corporation.
 *
 */

// C dependencies:
// #include <sound/soc-acpi.h>
// #include <sound/soc-acpi-intel-match.h>

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct snd_soc_acpi_codecs {
    pub num_codecs: u32,
    pub codecs: [*const c_char; 3],
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub id: *const c_char,
    pub comp_ids: *const snd_soc_acpi_codecs,
    pub drv_name: *const c_char,
    pub fw_filename: *const c_char,
    pub machine_quirk: Option<
        unsafe extern "C" fn(*const snd_soc_acpi_mach, *const c_void) -> *const snd_soc_acpi_mach,
    >,
    pub quirk_data: *const c_void,
    pub sof_tplg_filename: *const c_char,
    pub tplg_quirk_mask: u64,
}

unsafe extern "C" {
    fn snd_soc_acpi_codec_list(
        mach: *const snd_soc_acpi_mach,
        data: *const c_void,
    ) -> *const snd_soc_acpi_mach;
}

// Constants supplied by <sound/soc-acpi.h>:
// SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER
// SND_SOC_ACPI_TPLG_INTEL_SSP_MSB
// SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER

static essx_83x6: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 3,
    codecs: [
        b"ESSX8316\0".as_ptr() as *const c_char,
        b"ESSX8326\0".as_ptr() as *const c_char,
        b"ESSX8336\0".as_ptr() as *const c_char,
    ],
};

static glk_codecs: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [
        b"MX98357A\0".as_ptr() as *const c_char,
        core::ptr::null(),
        core::ptr::null(),
    ],
};

static glk_rt5682_rt5682s_hp: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 2,
    codecs: [
        b"10EC5682\0".as_ptr() as *const c_char,
        b"RTL5682\0".as_ptr() as *const c_char,
        core::ptr::null(),
    ],
};

#[unsafe(no_mangle)]
pub static mut snd_soc_acpi_intel_glk_machines: [snd_soc_acpi_mach; 6] = [
    snd_soc_acpi_mach {
        id: b"INT343A\0".as_ptr() as *const c_char,
        comp_ids: core::ptr::null(),
        drv_name: b"glk_alc298s_i2s\0".as_ptr() as *const c_char,
        fw_filename: b"intel/dsp_fw_glk.bin\0".as_ptr() as *const c_char,
        machine_quirk: None,
        quirk_data: core::ptr::null(),
        sof_tplg_filename: b"sof-glk-alc298.tplg\0".as_ptr() as *const c_char,
        tplg_quirk_mask: 0,
    },
    snd_soc_acpi_mach {
        id: b"DLGS7219\0".as_ptr() as *const c_char,
        comp_ids: core::ptr::null(),
        drv_name: b"glk_da7219_def\0".as_ptr() as *const c_char,
        fw_filename: b"intel/dsp_fw_glk.bin\0".as_ptr() as *const c_char,
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: &glk_codecs as *const snd_soc_acpi_codecs as *const c_void,
        sof_tplg_filename: b"sof-glk-da7219.tplg\0".as_ptr() as *const c_char,
        tplg_quirk_mask: 0,
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        comp_ids: &glk_rt5682_rt5682s_hp as *const snd_soc_acpi_codecs,
        drv_name: b"glk_rt5682_def\0".as_ptr() as *const c_char,
        fw_filename: b"intel/dsp_fw_glk.bin\0".as_ptr() as *const c_char,
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: &glk_codecs as *const snd_soc_acpi_codecs as *const c_void,
        sof_tplg_filename: b"sof-glk-rt5682.tplg\0".as_ptr() as *const c_char,
        tplg_quirk_mask: 0,
    },
    snd_soc_acpi_mach {
        id: b"10134242\0".as_ptr() as *const c_char,
        comp_ids: core::ptr::null(),
        drv_name: b"glk_cs4242_mx98357a\0".as_ptr() as *const c_char,
        fw_filename: b"intel/dsp_fw_glk.bin\0".as_ptr() as *const c_char,
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: &glk_codecs as *const snd_soc_acpi_codecs as *const c_void,
        sof_tplg_filename: b"sof-glk-cs42l42.tplg\0".as_ptr() as *const c_char,
        tplg_quirk_mask: 0,
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        comp_ids: &essx_83x6 as *const snd_soc_acpi_codecs,
        drv_name: b"sof-essx8336\0".as_ptr() as *const c_char,
        fw_filename: core::ptr::null(),
        machine_quirk: None,
        quirk_data: core::ptr::null(),
        sof_tplg_filename: b"sof-glk-es8336\0".as_ptr() as *const c_char, /* the tplg suffix is added at run time */
        tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER
            | SND_SOC_ACPI_TPLG_INTEL_SSP_MSB
            | SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER,
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        comp_ids: core::ptr::null(),
        drv_name: core::ptr::null(),
        fw_filename: core::ptr::null(),
        machine_quirk: None,
        quirk_data: core::ptr::null(),
        sof_tplg_filename: core::ptr::null(),
        tplg_quirk_mask: 0,
    },
];

// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_glk_machines);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
