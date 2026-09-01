/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2025 Intel Corporation.
 */

/*
 * This file defines data structures used in Machine Driver for Intel
 * platforms with Texas Instruments Codecs.
 */

// C dependencies:
// #include <sound/soc.h>
// #include <sound/soc-acpi-intel-ssp-common.h>

/*
 * Texas Instruments TAS2563
 */
pub const TAS2563_CODEC_DAI: &str = "tasdev_codec";
// C macro used adjacent string literal concatenation:
// #define TAS2563_DEV0_NAME "i2c-" TAS2563_ACPI_HID ":00"
pub const TAS2563_DEV0_NAME_PREFIX: &str = "i2c-";
pub const TAS2563_DEV0_NAME_SUFFIX: &str = ":00";

unsafe extern "C" {
    pub fn sof_tas2563_dai_link(link: *mut snd_soc_dai_link);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
