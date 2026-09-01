/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * This file defines data structures used in Machine Driver for Intel
 * platforms with Cirrus Logic Codecs.
 *
 * Copyright 2022 Intel Corporation.
 */

// Dependencies from the original C header:
// #include <sound/soc.h>
// #include <sound/soc-acpi-intel-ssp-common.h>

/*
 * Cirrus Logic CS35L41/CS35L53
 */
pub const CS35L41_CODEC_DAI: &str = "cs35l41-pcm";
pub const CS35L41_DEV0_NAME: &str = concat!("i2c-", CS35L41_ACPI_HID, ":00");
pub const CS35L41_DEV1_NAME: &str = concat!("i2c-", CS35L41_ACPI_HID, ":01");
pub const CS35L41_DEV2_NAME: &str = concat!("i2c-", CS35L41_ACPI_HID, ":02");
pub const CS35L41_DEV3_NAME: &str = concat!("i2c-", CS35L41_ACPI_HID, ":03");

unsafe extern "C" {
    pub fn cs35l41_set_dai_link(link: *mut snd_soc_dai_link);
    pub fn cs35l41_set_codec_conf(card: *mut snd_soc_card);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
