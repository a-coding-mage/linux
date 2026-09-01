/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * soc-acpi-intel-sdw-mockup-match.h - tables and support for SoundWire
 * mockup device ACPI enumeration.
 *
 * Copyright (c) 2021, Intel Corporation.
 *
 */

/* Depends on the external C definition of struct snd_soc_acpi_link_adr. */
unsafe extern "C" {
    pub static sdw_mockup_headset_1amp_mic: [snd_soc_acpi_link_adr; 0];
    pub static sdw_mockup_headset_2amps_mic: [snd_soc_acpi_link_adr; 0];
    pub static sdw_mockup_mic_headset_1amp: [snd_soc_acpi_link_adr; 0];
    pub static sdw_mockup_multi_func: [snd_soc_acpi_link_adr; 0];
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
