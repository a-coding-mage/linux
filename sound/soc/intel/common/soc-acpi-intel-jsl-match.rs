// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-apci-intel-jsl-match.c - tables and support for JSL ACPI enumeration.
 *
 * Copyright (c) 2019-2020, Intel Corporation.
 *
 */

// C includes translated as dependency intent:
// #include <sound/soc-acpi.h>
// #include <sound/soc-acpi-intel-match.h>

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    static snd_soc_acpi_codec_list: c_void;
}

#[repr(C)]
pub struct snd_soc_acpi_codecs {
    pub num_codecs: u32,
    pub codecs: [*const c_char; 3],
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub id: *const c_char,
    pub drv_name: *const c_char,
    pub machine_quirk: *const c_void,
    pub quirk_data: *const c_void,
    pub sof_tplg_filename: *const c_char,
    pub tplg_quirk_mask: u64,
    pub comp_ids: *const snd_soc_acpi_codecs,
}

const NULL_CHAR: *const c_char = core::ptr::null();

static essx_83x6: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 3,
    codecs: [
        b"ESSX8316\0".as_ptr() as *const c_char,
        b"ESSX8326\0".as_ptr() as *const c_char,
        b"ESSX8336\0".as_ptr() as *const c_char,
    ],
};

static mx98373_spk: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [b"MX98373\0".as_ptr() as *const c_char, NULL_CHAR, NULL_CHAR],
};

static rt1015_spk: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [b"10EC1015\0".as_ptr() as *const c_char, NULL_CHAR, NULL_CHAR],
};

static rt1015p_spk: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [b"RTL1015\0".as_ptr() as *const c_char, NULL_CHAR, NULL_CHAR],
};

static mx98360a_spk: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [b"MX98360A\0".as_ptr() as *const c_char, NULL_CHAR, NULL_CHAR],
};

static mut rt5650_spk: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [b"10EC5650\0".as_ptr() as *const c_char, NULL_CHAR, NULL_CHAR],
};

static rt5682_rt5682s_hp: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 2,
    codecs: [b"10EC5682\0".as_ptr() as *const c_char, b"RTL5682\0".as_ptr() as *const c_char, NULL_CHAR],
};

/*
 * When adding new entry to the snd_soc_acpi_intel_jsl_machines array,
 * use .quirk_data member to distinguish different machine driver,
 * and keep ACPI .id field unchanged for the common codec.
 */
#[unsafe(no_mangle)]
pub static mut snd_soc_acpi_intel_jsl_machines: [snd_soc_acpi_mach; 10] = [
    snd_soc_acpi_mach {
        id: b"DLGS7219\0".as_ptr() as *const c_char,
        drv_name: b"jsl_da7219_def\0".as_ptr() as *const c_char,
        machine_quirk: unsafe { &snd_soc_acpi_codec_list as *const c_void },
        quirk_data: &mx98373_spk as *const snd_soc_acpi_codecs as *const c_void,
        sof_tplg_filename: b"sof-jsl-da7219.tplg\0".as_ptr() as *const c_char,
        tplg_quirk_mask: 0,
        comp_ids: core::ptr::null(),
    },
    snd_soc_acpi_mach {
        id: b"DLGS7219\0".as_ptr() as *const c_char,
        drv_name: b"jsl_da7219_def\0".as_ptr() as *const c_char,
        machine_quirk: unsafe { &snd_soc_acpi_codec_list as *const c_void },
        quirk_data: &mx98360a_spk as *const snd_soc_acpi_codecs as *const c_void,
        sof_tplg_filename: b"sof-jsl-da7219-mx98360a.tplg\0".as_ptr() as *const c_char,
        tplg_quirk_mask: 0,
        comp_ids: core::ptr::null(),
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        drv_name: b"jsl_rt5682_def\0".as_ptr() as *const c_char,
        machine_quirk: unsafe { &snd_soc_acpi_codec_list as *const c_void },
        quirk_data: &rt1015_spk as *const snd_soc_acpi_codecs as *const c_void,
        sof_tplg_filename: b"sof-jsl-rt5682-rt1015.tplg\0".as_ptr() as *const c_char,
        tplg_quirk_mask: 0,
        comp_ids: &rt5682_rt5682s_hp as *const snd_soc_acpi_codecs,
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        drv_name: b"jsl_rt5682_def\0".as_ptr() as *const c_char,
        machine_quirk: unsafe { &snd_soc_acpi_codec_list as *const c_void },
        quirk_data: &rt1015p_spk as *const snd_soc_acpi_codecs as *const c_void,
        sof_tplg_filename: b"sof-jsl-rt5682-rt1015.tplg\0".as_ptr() as *const c_char,
        tplg_quirk_mask: 0,
        comp_ids: &rt5682_rt5682s_hp as *const snd_soc_acpi_codecs,
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        drv_name: b"jsl_rt5682_def\0".as_ptr() as *const c_char,
        machine_quirk: unsafe { &snd_soc_acpi_codec_list as *const c_void },
        quirk_data: &mx98360a_spk as *const snd_soc_acpi_codecs as *const c_void,
        sof_tplg_filename: b"sof-jsl-rt5682-mx98360a.tplg\0".as_ptr() as *const c_char,
        tplg_quirk_mask: 0,
        comp_ids: &rt5682_rt5682s_hp as *const snd_soc_acpi_codecs,
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        drv_name: b"jsl_rt5682_def\0".as_ptr() as *const c_char,
        machine_quirk: core::ptr::null(),
        quirk_data: core::ptr::null(),
        sof_tplg_filename: b"sof-jsl-rt5682.tplg\0".as_ptr() as *const c_char,
        tplg_quirk_mask: 0,
        comp_ids: &rt5682_rt5682s_hp as *const snd_soc_acpi_codecs,
    },
    snd_soc_acpi_mach {
        id: b"10134242\0".as_ptr() as *const c_char,
        drv_name: b"jsl_cs4242_mx98360a\0".as_ptr() as *const c_char,
        machine_quirk: unsafe { &snd_soc_acpi_codec_list as *const c_void },
        quirk_data: &mx98360a_spk as *const snd_soc_acpi_codecs as *const c_void,
        sof_tplg_filename: b"sof-jsl-cs42l42-mx98360a.tplg\0".as_ptr() as *const c_char,
        tplg_quirk_mask: 0,
        comp_ids: core::ptr::null(),
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        drv_name: b"sof-essx8336\0".as_ptr() as *const c_char,
        machine_quirk: core::ptr::null(),
        quirk_data: core::ptr::null(),
        sof_tplg_filename: b"sof-jsl-es8336\0".as_ptr() as *const c_char, /* the tplg suffix is added at run time */
        tplg_quirk_mask: SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER
            | SND_SOC_ACPI_TPLG_INTEL_SSP_MSB
            | SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER,
        comp_ids: &essx_83x6 as *const snd_soc_acpi_codecs,
    },
    snd_soc_acpi_mach {
        id: b"10EC5650\0".as_ptr() as *const c_char,
        drv_name: b"jsl_rt5682_def\0".as_ptr() as *const c_char,
        machine_quirk: unsafe { &snd_soc_acpi_codec_list as *const c_void },
        quirk_data: unsafe { &rt5650_spk as *const snd_soc_acpi_codecs as *const c_void },
        sof_tplg_filename: b"sof-jsl-rt5650.tplg\0".as_ptr() as *const c_char,
        tplg_quirk_mask: 0,
        comp_ids: core::ptr::null(),
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        drv_name: core::ptr::null(),
        machine_quirk: core::ptr::null(),
        quirk_data: core::ptr::null(),
        sof_tplg_filename: core::ptr::null(),
        tplg_quirk_mask: 0,
        comp_ids: core::ptr::null(),
    },
];

// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_jsl_machines);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
