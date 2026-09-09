/* SPDX-License-Identifier: GPL-2.0 */
//
// ALSA SoC Texas Instruments TAS2770 Audio Smart Amplifier
//
// Copyright (C) 2025 Texas Instruments Incorporated
// https://www.ti.com
//
// The TAS2770 hda driver implements for one, two, or even multiple
// TAS2770 chips.
//
// Author: Baojun Xu <baojun.xu@ti.com>
//

// The TASDEVICE_REG! and DECLARE_TLV_DB_SCALE! macros are supplied by the
// translated dependencies corresponding to the original C definitions.

pub const TAS2770_DVC_LEVEL: u32 = TASDEVICE_REG!(0x0, 0x0, 0x05);
pub const TAS2770_AMP_LEVEL: u32 = TASDEVICE_REG!(0x0, 0x0, 0x03);

DECLARE_TLV_DB_SCALE!(tas2770_dvc_tlv, -10000, 50, 0);
DECLARE_TLV_DB_SCALE!(tas2770_amp_tlv, 1100, 50, 0);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
