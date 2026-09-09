/* SPDX-License-Identifier: GPL-2.0 */
//
// ALSA SoC Texas Instruments TAS5825 Audio Smart Amplifier
//
// Copyright (C) 2025 Texas Instruments Incorporated
// https://www.ti.com
//
// The TAS5825 hda driver implements for one or two TAS5825 chips.
//
// Author: Baojun Xu <baojun.xu@ti.com>
//

// TASDEVICE_REG is supplied by the translated dependencies.
pub const TAS5825_DVC_LEVEL: u32 = TASDEVICE_REG(0x0, 0x0, 0x4c);
pub const TAS5825_AMP_LEVEL: u32 = TASDEVICE_REG(0x0, 0x0, 0x54);

// DECLARE_TLV_DB_SCALE(name, minimum, step, mute) is represented by its
// three source parameters: minimum dB value, step, and mute flag.
#[allow(non_upper_case_globals)]
pub static tas5825_dvc_tlv: [i32; 3] = [-10300, 50, 0];
#[allow(non_upper_case_globals)]
pub static tas5825_amp_tlv: [i32; 3] = [-1550, 50, 0];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
