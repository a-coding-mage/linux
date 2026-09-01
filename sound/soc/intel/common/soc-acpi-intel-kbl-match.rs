// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-acpi-intel-kbl-match.c - tables and support for KBL ACPI enumeration.
 *
 * Copyright (c) 2018, Intel Corporation.
 *
 */

// C dependencies:
// #include <sound/soc-acpi.h>
// #include <sound/soc-acpi-intel-match.h>

use crate::{snd_soc_acpi_codec_list, snd_soc_acpi_codecs, snd_soc_acpi_mach};

static KBL_CODECS: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [c"10508825".as_ptr()],
};

static KBL_POPPY_CODECS: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [c"10EC5663".as_ptr()],
};

static KBL_5663_5514_CODECS: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 2,
    codecs: [c"10EC5663".as_ptr(), c"10EC5514".as_ptr()],
};

static KBL_7219_98357_CODECS: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [c"MX98357A".as_ptr()],
};

static KBL_7219_98927_CODECS: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [c"MX98927".as_ptr()],
};

static KBL_7219_98373_CODECS: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [c"MX98373".as_ptr()],
};

#[no_mangle]
pub static mut snd_soc_acpi_intel_kbl_machines: [snd_soc_acpi_mach; 13] = [
    snd_soc_acpi_mach {
        id: c"INT343A".as_ptr(),
        drv_name: c"kbl_alc286s_i2s".as_ptr(),
        fw_filename: c"intel/dsp_fw_kbl.bin".as_ptr(),
        ..snd_soc_acpi_mach::default()
    },
    snd_soc_acpi_mach {
        id: c"INT343B".as_ptr(),
        drv_name: c"kbl_n88l25_s4567".as_ptr(),
        fw_filename: c"intel/dsp_fw_kbl.bin".as_ptr(),
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: &KBL_CODECS as *const snd_soc_acpi_codecs as *const _,
        ..snd_soc_acpi_mach::default()
    },
    snd_soc_acpi_mach {
        id: c"MX98357A".as_ptr(),
        drv_name: c"kbl_n88l25_m98357a".as_ptr(),
        fw_filename: c"intel/dsp_fw_kbl.bin".as_ptr(),
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: &KBL_CODECS as *const snd_soc_acpi_codecs as *const _,
        ..snd_soc_acpi_mach::default()
    },
    snd_soc_acpi_mach {
        id: c"MX98927".as_ptr(),
        drv_name: c"kbl_r5514_5663_max".as_ptr(),
        fw_filename: c"intel/dsp_fw_kbl.bin".as_ptr(),
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: &KBL_5663_5514_CODECS as *const snd_soc_acpi_codecs as *const _,
        ..snd_soc_acpi_mach::default()
    },
    snd_soc_acpi_mach {
        id: c"MX98927".as_ptr(),
        drv_name: c"kbl_rt5663_m98927".as_ptr(),
        fw_filename: c"intel/dsp_fw_kbl.bin".as_ptr(),
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: &KBL_POPPY_CODECS as *const snd_soc_acpi_codecs as *const _,
        ..snd_soc_acpi_mach::default()
    },
    snd_soc_acpi_mach {
        id: c"10EC5663".as_ptr(),
        drv_name: c"kbl_rt5663".as_ptr(),
        fw_filename: c"intel/dsp_fw_kbl.bin".as_ptr(),
        ..snd_soc_acpi_mach::default()
    },
    snd_soc_acpi_mach {
        id: c"DLGS7219".as_ptr(),
        drv_name: c"kbl_da7219_mx98357a".as_ptr(),
        fw_filename: c"intel/dsp_fw_kbl.bin".as_ptr(),
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: &KBL_7219_98357_CODECS as *const snd_soc_acpi_codecs as *const _,
        ..snd_soc_acpi_mach::default()
    },
    snd_soc_acpi_mach {
        id: c"DLGS7219".as_ptr(),
        drv_name: c"kbl_da7219_max98927".as_ptr(),
        fw_filename: c"intel/dsp_fw_kbl.bin".as_ptr(),
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: &KBL_7219_98927_CODECS as *const snd_soc_acpi_codecs as *const _,
        ..snd_soc_acpi_mach::default()
    },
    snd_soc_acpi_mach {
        id: c"10EC5660".as_ptr(),
        drv_name: c"kbl_rt5660".as_ptr(),
        fw_filename: c"intel/dsp_fw_kbl.bin".as_ptr(),
        ..snd_soc_acpi_mach::default()
    },
    snd_soc_acpi_mach {
        id: c"10EC3277".as_ptr(),
        drv_name: c"kbl_rt5660".as_ptr(),
        fw_filename: c"intel/dsp_fw_kbl.bin".as_ptr(),
        ..snd_soc_acpi_mach::default()
    },
    snd_soc_acpi_mach {
        id: c"DLGS7219".as_ptr(),
        drv_name: c"kbl_da7219_mx98373".as_ptr(),
        fw_filename: c"intel/dsp_fw_kbl.bin".as_ptr(),
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: &KBL_7219_98373_CODECS as *const snd_soc_acpi_codecs as *const _,
        ..snd_soc_acpi_mach::default()
    },
    snd_soc_acpi_mach {
        id: c"MX98373".as_ptr(),
        drv_name: c"kbl_max98373".as_ptr(),
        fw_filename: c"intel/dsp_fw_kbl.bin".as_ptr(),
        ..snd_soc_acpi_mach::default()
    },
    snd_soc_acpi_mach {
        ..snd_soc_acpi_mach::default()
    },
];

// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_kbl_machines);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
