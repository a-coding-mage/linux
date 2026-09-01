/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2020 Intel Corporation.
 */

/*
 * This file defines data structures used in Machine Driver for Intel
 * platforms with Realtek Codecs.
 */

/* C dependencies:
 * #include <sound/soc.h>
 * #include <sound/soc-acpi-intel-ssp-common.h>
 */

/*
 * Realtek ALC1011
 */

pub const RT1011_CODEC_DAI: &str = "rt1011-aif";
/* C macro depends on external RT1011_ACPI_HID:
 * #define RT1011_DEV0_NAME "i2c-" RT1011_ACPI_HID ":00"
 */
/* C macro depends on external RT1011_ACPI_HID:
 * #define RT1011_DEV1_NAME "i2c-" RT1011_ACPI_HID ":01"
 */
/* C macro depends on external RT1011_ACPI_HID:
 * #define RT1011_DEV2_NAME "i2c-" RT1011_ACPI_HID ":02"
 */
/* C macro depends on external RT1011_ACPI_HID:
 * #define RT1011_DEV3_NAME "i2c-" RT1011_ACPI_HID ":03"
 */

unsafe extern "C" {
    pub fn sof_rt1011_dai_link(dev: *mut device, link: *mut snd_soc_dai_link);
    pub fn sof_rt1011_codec_conf(dev: *mut device, card: *mut snd_soc_card);
}

/*
 * Realtek ALC1015 (AUTO)
 */
pub const RT1015P_CODEC_DAI: &str = "HiFi";
/* C macro depends on external RT1015P_ACPI_HID:
 * #define RT1015P_DEV0_NAME RT1015P_ACPI_HID ":00"
 */

unsafe extern "C" {
    pub fn sof_rt1015p_dai_link(link: *mut snd_soc_dai_link);
    pub fn sof_rt1015p_codec_conf(card: *mut snd_soc_card);
}

/*
 * Realtek ALC1015 (I2C)
 */
pub const RT1015_CODEC_DAI: &str = "rt1015-aif";
/* C macro depends on external RT1015_ACPI_HID:
 * #define RT1015_DEV0_NAME "i2c-" RT1015_ACPI_HID ":00"
 */
/* C macro depends on external RT1015_ACPI_HID:
 * #define RT1015_DEV1_NAME "i2c-" RT1015_ACPI_HID ":01"
 */

unsafe extern "C" {
    pub fn sof_rt1015_dai_link(link: *mut snd_soc_dai_link);
    pub fn sof_rt1015_codec_conf(card: *mut snd_soc_card);
}

/*
 * Realtek ALC1308
 */
pub const RT1308_CODEC_DAI: &str = "rt1308-aif";
/* C macro depends on external RT1308_ACPI_HID:
 * #define RT1308_DEV0_NAME "i2c-" RT1308_ACPI_HID ":00"
 */

unsafe extern "C" {
    pub fn sof_rt1308_dai_link(link: *mut snd_soc_dai_link);
}

/*
 * Realtek ALC1019
 */
pub const RT1019P_CODEC_DAI: &str = "HiFi";
/* C macro depends on external RT1019P_ACPI_HID:
 * #define RT1019P_DEV0_NAME RT1019P_ACPI_HID ":00"
 */

unsafe extern "C" {
    pub fn sof_rt1019p_dai_link(link: *mut snd_soc_dai_link);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
