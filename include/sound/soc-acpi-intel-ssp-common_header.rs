/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2023 Intel Corporation.
 */

/* Cirrus Logic */
pub const CS35L41_ACPI_HID: &str = "CSC3541";
pub const CS42L42_ACPI_HID: &str = "10134242";

/* Dialog */
pub const DA7219_ACPI_HID: &str = "DLGS7219";

/* Everest */
pub const ES8316_ACPI_HID: &str = "ESSX8316";
pub const ES8326_ACPI_HID: &str = "ESSX8326";
pub const ES8336_ACPI_HID: &str = "ESSX8336";

pub const MAX_98357A_ACPI_HID: &str = "MX98357A";
pub const MAX_98360A_ACPI_HID: &str = "MX98360A";
pub const MAX_98373_ACPI_HID: &str = "MX98373";
pub const MAX_98390_ACPI_HID: &str = "MX98390";

/* Nuvoton */
pub const NAU8318_ACPI_HID: &str = "NVTN2012";
pub const NAU8825_ACPI_HID: &str = "10508825";

/* Realtek */
pub const RT1011_ACPI_HID: &str = "10EC1011";
pub const RT1015_ACPI_HID: &str = "10EC1015";
pub const RT1015P_ACPI_HID: &str = "RTL1015";
pub const RT1019P_ACPI_HID: &str = "RTL1019";
pub const RT1308_ACPI_HID: &str = "10EC1308";
pub const RT5650_ACPI_HID: &str = "10EC5650";
pub const RT5682_ACPI_HID: &str = "10EC5682";
pub const RT5682S_ACPI_HID: &str = "RTL5682";

/* Texas Instruments */
pub const TAS2563_ACPI_HID: &str = "TXNW2563";

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum snd_soc_acpi_intel_codec {
    CODEC_NONE,

    /* headphone codec */
    CODEC_CS42L42,
    CODEC_DA7219,
    CODEC_ES8316,
    CODEC_ES8326,
    CODEC_ES8336,
    CODEC_NAU8825,
    CODEC_RT5650,
    CODEC_RT5682,
    CODEC_RT5682S,

    /* speaker amplifier */
    CODEC_CS35L41,
    CODEC_MAX98357A,
    CODEC_MAX98360A,
    CODEC_MAX98373,
    CODEC_MAX98390,
    CODEC_NAU8318,
    CODEC_RT1011,
    CODEC_RT1015,
    CODEC_RT1015P,
    CODEC_RT1019P,
    CODEC_RT1308,
    CODEC_TAS2563,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

extern "C" {
    pub fn snd_soc_acpi_intel_detect_codec_type(
        dev: *mut device,
    ) -> snd_soc_acpi_intel_codec;
    pub fn snd_soc_acpi_intel_detect_amp_type(
        dev: *mut device,
    ) -> snd_soc_acpi_intel_codec;

    pub fn snd_soc_acpi_intel_get_codec_name(
        codec_type: snd_soc_acpi_intel_codec,
    ) -> *const u8;

    pub fn snd_soc_acpi_intel_get_codec_tplg_suffix(
        codec_type: snd_soc_acpi_intel_codec,
    ) -> *const u8;
    pub fn snd_soc_acpi_intel_get_amp_tplg_suffix(
        codec_type: snd_soc_acpi_intel_codec,
    ) -> *const u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
