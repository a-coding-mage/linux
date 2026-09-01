// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-acpi-intel-skl-match.c - tables and support for SKL ACPI enumeration.
 *
 * Copyright (c) 2018, Intel Corporation.
 *
 */

// C dependencies:
// #include <sound/soc-acpi.h>
// #include <sound/soc-acpi-intel-match.h>

use core::ffi::{c_char, c_void};

extern "C" {
    fn snd_soc_acpi_codec_list(arg: *mut c_void) -> i32;
}

#[repr(C)]
pub struct snd_soc_acpi_codecs {
    pub num_codecs: i32,
    pub codecs: [*const c_char; 1],
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub id: *const c_char,
    pub drv_name: *const c_char,
    pub fw_filename: *const c_char,
    pub machine_quirk: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub quirk_data: *const c_void,
}

static skl_codecs: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 1,
    codecs: [b"10508825\0".as_ptr() as *const c_char],
};

#[no_mangle]
pub static mut snd_soc_acpi_intel_skl_machines: [snd_soc_acpi_mach; 4] = [
    snd_soc_acpi_mach {
        id: b"INT343A\0".as_ptr() as *const c_char,
        drv_name: b"skl_alc286s_i2s\0".as_ptr() as *const c_char,
        fw_filename: b"intel/dsp_fw_release.bin\0".as_ptr() as *const c_char,
        machine_quirk: None,
        quirk_data: core::ptr::null(),
    },
    snd_soc_acpi_mach {
        id: b"INT343B\0".as_ptr() as *const c_char,
        drv_name: b"skl_n88l25_s4567\0".as_ptr() as *const c_char,
        fw_filename: b"intel/dsp_fw_release.bin\0".as_ptr() as *const c_char,
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: &skl_codecs as *const snd_soc_acpi_codecs as *const c_void,
    },
    snd_soc_acpi_mach {
        id: b"MX98357A\0".as_ptr() as *const c_char,
        drv_name: b"skl_n88l25_m98357a\0".as_ptr() as *const c_char,
        fw_filename: b"intel/dsp_fw_release.bin\0".as_ptr() as *const c_char,
        machine_quirk: Some(snd_soc_acpi_codec_list),
        quirk_data: &skl_codecs as *const snd_soc_acpi_codecs as *const c_void,
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        drv_name: core::ptr::null(),
        fw_filename: core::ptr::null(),
        machine_quirk: None,
        quirk_data: core::ptr::null(),
    },
];

// EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_skl_machines);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
