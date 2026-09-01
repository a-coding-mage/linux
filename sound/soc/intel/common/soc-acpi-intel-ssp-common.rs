// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2023 Intel Corporation

// Translated from Linux includes:
// #include <linux/device.h>
// #include <sound/soc-acpi.h>
// #include <sound/soc-acpi-intel-ssp-common.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int};
use core::ptr;

// External declarations supplied by the translated kernel headers/modules.
extern "C" {
    pub type device;

    fn acpi_dev_present(hid: *const c_char, uid: *const c_char, hrv: c_int) -> bool;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

extern "Rust" {
    static CS42L42_ACPI_HID: *const c_char;
    static DA7219_ACPI_HID: *const c_char;
    static ES8316_ACPI_HID: *const c_char;
    static ES8326_ACPI_HID: *const c_char;
    static ES8336_ACPI_HID: *const c_char;
    static NAU8825_ACPI_HID: *const c_char;
    static RT5650_ACPI_HID: *const c_char;
    static RT5682_ACPI_HID: *const c_char;
    static RT5682S_ACPI_HID: *const c_char;
    static CS35L41_ACPI_HID: *const c_char;
    static MAX_98357A_ACPI_HID: *const c_char;
    static MAX_98360A_ACPI_HID: *const c_char;
    static MAX_98373_ACPI_HID: *const c_char;
    static MAX_98390_ACPI_HID: *const c_char;
    static NAU8318_ACPI_HID: *const c_char;
    static RT1011_ACPI_HID: *const c_char;
    static RT1015_ACPI_HID: *const c_char;
    static RT1015P_ACPI_HID: *const c_char;
    static RT1019P_ACPI_HID: *const c_char;
    static RT1308_ACPI_HID: *const c_char;
    static TAS2563_ACPI_HID: *const c_char;

    static CODEC_NONE: snd_soc_acpi_intel_codec;
    static CODEC_CS42L42: snd_soc_acpi_intel_codec;
    static CODEC_DA7219: snd_soc_acpi_intel_codec;
    static CODEC_ES8316: snd_soc_acpi_intel_codec;
    static CODEC_ES8326: snd_soc_acpi_intel_codec;
    static CODEC_ES8336: snd_soc_acpi_intel_codec;
    static CODEC_NAU8825: snd_soc_acpi_intel_codec;
    static CODEC_RT5650: snd_soc_acpi_intel_codec;
    static CODEC_RT5682: snd_soc_acpi_intel_codec;
    static CODEC_RT5682S: snd_soc_acpi_intel_codec;
    static CODEC_CS35L41: snd_soc_acpi_intel_codec;
    static CODEC_MAX98357A: snd_soc_acpi_intel_codec;
    static CODEC_MAX98360A: snd_soc_acpi_intel_codec;
    static CODEC_MAX98373: snd_soc_acpi_intel_codec;
    static CODEC_MAX98390: snd_soc_acpi_intel_codec;
    static CODEC_NAU8318: snd_soc_acpi_intel_codec;
    static CODEC_RT1011: snd_soc_acpi_intel_codec;
    static CODEC_RT1015: snd_soc_acpi_intel_codec;
    static CODEC_RT1015P: snd_soc_acpi_intel_codec;
    static CODEC_RT1019P: snd_soc_acpi_intel_codec;
    static CODEC_RT1308: snd_soc_acpi_intel_codec;
    static CODEC_TAS2563: snd_soc_acpi_intel_codec;
}

pub type snd_soc_acpi_intel_codec = c_int;

/*
 * Codec probe function
 */
#[repr(C)]
struct codec_map {
    name: *const c_char,
    tplg_suffix: *const c_char,
    acpi_hid: *const c_char,
    codec_type: snd_soc_acpi_intel_codec,
}

unsafe impl Sync for codec_map {}

macro_rules! CODEC_MAP_ENTRY {
    ($n:expr, $s:expr, $h:expr, $t:expr) => {
        codec_map {
            name: $n.as_ptr() as *const c_char,
            tplg_suffix: $s.as_ptr() as *const c_char,
            acpi_hid: $h,
            codec_type: $t,
        }
    };
}

static codecs: [codec_map; 9] = unsafe {
    [
        /* Cirrus Logic */
        CODEC_MAP_ENTRY!(b"CS42L42\0", b"cs42l42\0", CS42L42_ACPI_HID, CODEC_CS42L42),

        /* Dialog */
        CODEC_MAP_ENTRY!(b"DA7219\0", b"da7219\0", DA7219_ACPI_HID, CODEC_DA7219),

        /* Everest */
        CODEC_MAP_ENTRY!(b"ES8316\0", b"es8336\0", ES8316_ACPI_HID, CODEC_ES8316),
        CODEC_MAP_ENTRY!(b"ES8326\0", b"es8336\0", ES8326_ACPI_HID, CODEC_ES8326),
        CODEC_MAP_ENTRY!(b"ES8336\0", b"es8336\0", ES8336_ACPI_HID, CODEC_ES8336),

        /* Nuvoton */
        CODEC_MAP_ENTRY!(b"NAU8825\0", b"nau8825\0", NAU8825_ACPI_HID, CODEC_NAU8825),

        /* Realtek */
        CODEC_MAP_ENTRY!(b"RT5650\0", b"rt5650\0", RT5650_ACPI_HID, CODEC_RT5650),
        CODEC_MAP_ENTRY!(b"RT5682\0", b"rt5682\0", RT5682_ACPI_HID, CODEC_RT5682),
        CODEC_MAP_ENTRY!(b"RT5682S\0", b"rt5682\0", RT5682S_ACPI_HID, CODEC_RT5682S),
    ]
};

static amps: [codec_map; 13] = unsafe {
    [
        /* Cirrus Logic */
        CODEC_MAP_ENTRY!(b"CS35L41\0", b"cs35l41\0", CS35L41_ACPI_HID, CODEC_CS35L41),

        /* Maxim */
        CODEC_MAP_ENTRY!(b"MAX98357A\0", b"max98357a\0", MAX_98357A_ACPI_HID, CODEC_MAX98357A),
        CODEC_MAP_ENTRY!(b"MAX98360A\0", b"max98360a\0", MAX_98360A_ACPI_HID, CODEC_MAX98360A),
        CODEC_MAP_ENTRY!(b"MAX98373\0", b"max98373\0", MAX_98373_ACPI_HID, CODEC_MAX98373),
        CODEC_MAP_ENTRY!(b"MAX98390\0", b"max98390\0", MAX_98390_ACPI_HID, CODEC_MAX98390),

        /* Nuvoton */
        CODEC_MAP_ENTRY!(b"NAU8318\0", b"nau8318\0", NAU8318_ACPI_HID, CODEC_NAU8318),

        /* Realtek */
        CODEC_MAP_ENTRY!(b"RT1011\0", b"rt1011\0", RT1011_ACPI_HID, CODEC_RT1011),
        CODEC_MAP_ENTRY!(b"RT1015\0", b"rt1015\0", RT1015_ACPI_HID, CODEC_RT1015),
        CODEC_MAP_ENTRY!(b"RT1015P\0", b"rt1015\0", RT1015P_ACPI_HID, CODEC_RT1015P),
        CODEC_MAP_ENTRY!(b"RT1019P\0", b"rt1019\0", RT1019P_ACPI_HID, CODEC_RT1019P),
        CODEC_MAP_ENTRY!(b"RT1308\0", b"rt1308\0", RT1308_ACPI_HID, CODEC_RT1308),

        /* Texas Instruments */
        CODEC_MAP_ENTRY!(b"TAS2563\0", b"tas2563\0", TAS2563_ACPI_HID, CODEC_TAS2563),

        /*
         * Monolithic components
         *
         * Only put components that can serve as both the amp and the codec below this line.
         * This will ensure that if the part is used just as a codec and there is an amp as well
         * then the amp will be selected properly.
         */
        CODEC_MAP_ENTRY!(b"RT5650\0", b"rt5650\0", RT5650_ACPI_HID, CODEC_RT5650),
    ]
};

#[no_mangle]
pub unsafe extern "C" fn snd_soc_acpi_intel_detect_codec_type(
    dev: *mut device,
) -> snd_soc_acpi_intel_codec {
    let mut i: usize = 0;

    while i < codecs.len() {
        if !acpi_dev_present(codecs[i].acpi_hid, ptr::null(), -1) {
            i += 1;
            continue;
        }

        dev_dbg(
            dev,
            b"codec %s found\n\0".as_ptr() as *const c_char,
            codecs[i].name,
        );
        return codecs[i].codec_type;
    }

    CODEC_NONE
}
// EXPORT_SYMBOL_NS(snd_soc_acpi_intel_detect_codec_type, "SND_SOC_ACPI_INTEL_MATCH");

#[no_mangle]
pub unsafe extern "C" fn snd_soc_acpi_intel_detect_amp_type(
    dev: *mut device,
) -> snd_soc_acpi_intel_codec {
    let mut i: usize = 0;

    while i < amps.len() {
        if !acpi_dev_present(amps[i].acpi_hid, ptr::null(), -1) {
            i += 1;
            continue;
        }

        dev_dbg(
            dev,
            b"amp %s found\n\0".as_ptr() as *const c_char,
            amps[i].name,
        );
        return amps[i].codec_type;
    }

    CODEC_NONE
}
// EXPORT_SYMBOL_NS(snd_soc_acpi_intel_detect_amp_type, "SND_SOC_ACPI_INTEL_MATCH");

#[no_mangle]
pub unsafe extern "C" fn snd_soc_acpi_intel_get_codec_name(
    codec_type: snd_soc_acpi_intel_codec,
) -> *const c_char {
    let mut i: usize = 0;

    while i < codecs.len() {
        if codecs[i].codec_type != codec_type {
            i += 1;
            continue;
        }

        return codecs[i].name;
    }
    i = 0;
    while i < amps.len() {
        if amps[i].codec_type != codec_type {
            i += 1;
            continue;
        }

        return amps[i].name;
    }

    ptr::null()
}
// EXPORT_SYMBOL_NS(snd_soc_acpi_intel_get_codec_name, "SND_SOC_ACPI_INTEL_MATCH");

#[no_mangle]
pub unsafe extern "C" fn snd_soc_acpi_intel_get_codec_tplg_suffix(
    codec_type: snd_soc_acpi_intel_codec,
) -> *const c_char {
    let mut i: usize = 0;

    while i < codecs.len() {
        if codecs[i].codec_type != codec_type {
            i += 1;
            continue;
        }

        return codecs[i].tplg_suffix;
    }

    ptr::null()
}
// EXPORT_SYMBOL_NS(snd_soc_acpi_intel_get_codec_tplg_suffix, "SND_SOC_ACPI_INTEL_MATCH");

#[no_mangle]
pub unsafe extern "C" fn snd_soc_acpi_intel_get_amp_tplg_suffix(
    codec_type: snd_soc_acpi_intel_codec,
) -> *const c_char {
    let mut i: usize = 0;

    while i < amps.len() {
        if amps[i].codec_type != codec_type {
            i += 1;
            continue;
        }

        return amps[i].tplg_suffix;
    }

    ptr::null()
}
// EXPORT_SYMBOL_NS(snd_soc_acpi_intel_get_amp_tplg_suffix, "SND_SOC_ACPI_INTEL_MATCH");

// MODULE_DESCRIPTION("ASoC Intel SOF Common Machine Driver Helpers");
// MODULE_AUTHOR("Brent Lu <brent.lu@intel.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
