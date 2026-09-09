/* SPDX-License-Identifier: GPL-2.0 */
//
// ALSA SoC Texas Instruments TAS2781 Audio Smart Amplifier
//
// Copyright (C) 2022 - 2025 Texas Instruments Incorporated
// https://www.ti.com
//
// The TAS2781 driver implements a flexible and configurable
// algo coefficient setting for one, two, or even multiple
// TAS2781 chips.
//
// Author: Shenghao Ding <shenghao-ding@ti.com>
//

// DECLARE_TLV_DB_SCALE expands to an unsigned-integer TLV descriptor array:
// { SNDRV_CTL_TLVT_DB_SCALE, minimum, step, mute }
// The SNDRV_CTL_TLVT_DB_SCALE value is supplied by the ALSA dependency.
// These declarations preserve the header's local data and the dependency's
// macro expansion shape for use by the translated driver.
pub static tas2781_dvc_tlv: [i32; 4] = [
    SNDRV_CTL_TLVT_DB_SCALE as i32,
    -10000,
    50,
    0,
];

pub static tas2781_amp_tlv: [i32; 4] = [
    SNDRV_CTL_TLVT_DB_SCALE as i32,
    1100,
    50,
    0,
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
