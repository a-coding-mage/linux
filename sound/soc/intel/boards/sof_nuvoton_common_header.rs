/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * This file defines data structures used in Machine Driver for Intel
 * platforms with Nuvoton Codecs.
 *
 * Copyright 2023 Intel Corporation.
 */

/* C dependencies: <sound/soc.h>, <sound/soc-acpi-intel-ssp-common.h> */

/*
 * Nuvoton NAU8318
 */
pub const NAU8318_CODEC_DAI: &str = "nau8315-hifi";

/*
 * C macro intent:
 * #define NAU8318_DEV0_NAME "i2c-" NAU8318_ACPI_HID ":00"
 *
 * NAU8318_ACPI_HID is provided by an included dependency, so this file-local
 * translation preserves the declaration dependency rather than inventing it.
 */
pub const NAU8318_DEV0_NAME_PREFIX: &str = "i2c-";
pub const NAU8318_DEV0_NAME_SUFFIX: &str = ":00";

unsafe extern "C" {
    pub fn nau8318_set_dai_link(link: *mut snd_soc_dai_link);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
