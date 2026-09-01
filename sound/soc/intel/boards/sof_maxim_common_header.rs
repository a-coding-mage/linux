/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2020 Intel Corporation.
 */

/*
 * This file defines data structures used in Machine Driver for Intel
 * platforms with Maxim Codecs.
 */

/* Dependencies from the C header:
 * sound/soc.h
 * sound/soc-acpi-intel-ssp-common.h
 */

/*
 * Maxim MAX98373
 */
pub const MAX_98373_CODEC_DAI: &str = "max98373-aif1";
pub const MAX_98373_DEV0_NAME: &str = concat!("i2c-", MAX_98373_ACPI_HID, ":00");
pub const MAX_98373_DEV1_NAME: &str = concat!("i2c-", MAX_98373_ACPI_HID, ":01");

unsafe extern "C" {
    pub fn max_98373_dai_link(dev: *mut device, link: *mut snd_soc_dai_link);
    pub fn max_98373_set_codec_conf(card: *mut snd_soc_card);
}

/*
 * Maxim MAX98390
 */
pub const MAX_98390_CODEC_DAI: &str = "max98390-aif1";
pub const MAX_98390_DEV0_NAME: &str = concat!("i2c-", MAX_98390_ACPI_HID, ":00");
pub const MAX_98390_DEV1_NAME: &str = concat!("i2c-", MAX_98390_ACPI_HID, ":01");
pub const MAX_98390_DEV2_NAME: &str = concat!("i2c-", MAX_98390_ACPI_HID, ":02");
pub const MAX_98390_DEV3_NAME: &str = concat!("i2c-", MAX_98390_ACPI_HID, ":03");

unsafe extern "C" {
    pub fn max_98390_dai_link(dev: *mut device, link: *mut snd_soc_dai_link);
    pub fn max_98390_set_codec_conf(dev: *mut device, card: *mut snd_soc_card);
}

/*
 * Maxim MAX98357A/MAX98360A
 */
pub const MAX_98357A_CODEC_DAI: &str = "HiFi";
pub const MAX_98357A_DEV0_NAME: &str = concat!(MAX_98357A_ACPI_HID, ":00");
pub const MAX_98360A_DEV0_NAME: &str = concat!(MAX_98360A_ACPI_HID, ":00");

unsafe extern "C" {
    pub fn max_98357a_dai_link(link: *mut snd_soc_dai_link);
    pub fn max_98360a_dai_link(link: *mut snd_soc_dai_link);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
