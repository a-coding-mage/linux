// SPDX-License-Identifier: GPL-2.0
//
// ALSA SoC Texas Instruments TAS2783 Audio Smart Amplifier
//
// Copyright (C) 2025 Texas Instruments Incorporated
// https://www.ti.com
//
// The TAS2783 driver implements a flexible and configurable
// algo coefficient setting for single TAS2783 chips.
//
// Author: Niranjan H Y <niranjanhy@ti.com>
// Author: Baojun Xu <baojun.xu@ti.com>

// C header dependency: <linux/workqueue.h>
// External symbols expected from surrounding bindings:
// SNDRV_PCM_RATE_44100, SNDRV_PCM_RATE_48000, SNDRV_PCM_RATE_96000,
// SNDRV_PCM_RATE_88200, SNDRV_PCM_FMTBIT_S16_LE, SNDRV_PCM_FMTBIT_S24_LE,
// SNDRV_PCM_FMTBIT_S32_LE, and GENMASK.

pub const TAS2783_DEVICE_RATES: u32 = SNDRV_PCM_RATE_44100
	| SNDRV_PCM_RATE_48000
	| SNDRV_PCM_RATE_96000
	| SNDRV_PCM_RATE_88200;
pub const TAS2783_DEVICE_FORMATS: u32 = SNDRV_PCM_FMTBIT_S16_LE
	| SNDRV_PCM_FMTBIT_S24_LE
	| SNDRV_PCM_FMTBIT_S32_LE;

/* book, page, register */
pub const fn TASDEV_REG_SDW(book: u32, page: u32, reg: u32) -> u32 {
	(book * 256 * 128) + 0x800000 + (page * 128) + reg
}

pub const TAS2783_SW_RESET: u32 = TASDEV_REG_SDW(0x0, 0x00, 0x01);
/* Volume control */
pub const TAS2783_DVC_LVL: u32 = TASDEV_REG_SDW(0x0, 0x00, 0x1A);
pub const TAS2783_AMP_LEVEL: u32 = TASDEV_REG_SDW(0x0, 0x00, 0x03);
pub const TAS2783_AMP_LEVEL_MASK: u32 = GENMASK(5, 1);

pub const PRAM_ADDR_START: u32 = TASDEV_REG_SDW(0x8c, 0x01, 0x8);
pub const PRAM_ADDR_END: u32 = TASDEV_REG_SDW(0x8c, 0xff, 0x7f);
pub const YRAM_ADDR_START: u32 = TASDEV_REG_SDW(0x00, 0x02, 0x8);
pub const YRAM_ADDR_END: u32 = TASDEV_REG_SDW(0x00, 0x37, 0x7f);

/* Calibration data */
pub const TAS2783_CAL_R0: u32 = TASDEV_REG_SDW(0, 0x16, 0x4C);
pub const TAS2783_CAL_INVR0: u32 = TASDEV_REG_SDW(0, 0x16, 0x5C);
pub const TAS2783_CAL_R0LOW: u32 = TASDEV_REG_SDW(0, 0x16, 0x64);
pub const TAS2783_CAL_POWER: u32 = TASDEV_REG_SDW(0, 0x15, 0x44);
pub const TAS2783_CAL_TLIM: u32 = TASDEV_REG_SDW(0, 0x17, 0x58);

/* TAS2783 SDCA Control - function number */
pub const FUNC_NUM_SMART_AMP: u32 = 0x01;

/* TAS2783 SDCA entity */

pub const TAS2783_SDCA_ENT_FU21: u32 = 0x01;
pub const TAS2783_SDCA_ENT_FU23: u32 = 0x02;
pub const TAS2783_SDCA_ENT_FU26: u32 = 0x03;
pub const TAS2783_SDCA_ENT_XU22: u32 = 0x04;
pub const TAS2783_SDCA_ENT_CS24: u32 = 0x05;
pub const TAS2783_SDCA_ENT_CS21: u32 = 0x06;
pub const TAS2783_SDCA_ENT_CS25: u32 = 0x07;
pub const TAS2783_SDCA_ENT_CS26: u32 = 0x08;
pub const TAS2783_SDCA_ENT_CS28: u32 = 0x09;
pub const TAS2783_SDCA_ENT_PDE23: u32 = 0x0C;
pub const TAS2783_SDCA_ENT_UDMPU23: u32 = 0x0E;
pub const TAS2783_SDCA_ENT_SAPU29: u32 = 0x0F;
pub const TAS2783_SDCA_ENT_PPU21: u32 = 0x10;
pub const TAS2783_SDCA_ENT_PPU26: u32 = 0x11;
pub const TAS2783_SDCA_ENT_TG23: u32 = 0x12;
pub const TAS2783_SDCA_ENT_IT21: u32 = 0x13;
pub const TAS2783_SDCA_ENT_IT29: u32 = 0x14;
pub const TAS2783_SDCA_ENT_IT26: u32 = 0x15;
pub const TAS2783_SDCA_ENT_IT28: u32 = 0x16;
pub const TAS2783_SDCA_ENT_OT24: u32 = 0x17;
pub const TAS2783_SDCA_ENT_OT23: u32 = 0x18;
pub const TAS2783_SDCA_ENT_OT25: u32 = 0x19;
pub const TAS2783_SDCA_ENT_OT28: u32 = 0x1A;
pub const TAS2783_SDCA_ENT_MU26: u32 = 0x1b;
pub const TAS2783_SDCA_ENT_OT127: u32 = 0x1E;
pub const TAS2783_SDCA_ENT_FU127: u32 = 0x1F;
pub const TAS2783_SDCA_ENT_CS127: u32 = 0x20;
pub const TAS2783_SDCA_ENT_MFPU21: u32 = 0x22;
pub const TAS2783_SDCA_ENT_MFPU26: u32 = 0x23;

/* TAS2783 SDCA control */
pub const TAS2783_SDCA_CTL_REQ_POW_STATE: u32 = 0x01;
pub const TAS2783_SDCA_CTL_FU_MUTE: u32 = 0x01;
pub const TAS2783_SDCA_CTL_UDMPU_CLUSTER: u32 = 0x10;

pub const TAS2783_DEVICE_CHANNEL_LEFT: u32 = 1;
pub const TAS2783_DEVICE_CHANNEL_RIGHT: u32 = 2;

pub const TAS2783_SDCA_POW_STATE_ON: u32 = 0;
pub const TAS2783_SDCA_POW_STATE_OFF: u32 = 3;

/* calibration data */
pub const TAS2783_CALIB_PARAMS: u32 = 6; /* 5 + 1 unique id */
pub const TAS2783_CALIB_MAX_SPK_COUNT: u32 = 8;
pub const TAS2783_CALIB_HDR_SZ: u32 = 12;
pub const TAS2783_CALIB_CRC_SZ: u32 = 4;
pub const TAS2783_CALIB_DATA_SZ: u32 = TAS2783_CALIB_HDR_SZ
	+ TAS2783_CALIB_CRC_SZ
	+ (TAS2783_CALIB_PARAMS * 4 * TAS2783_CALIB_MAX_SPK_COUNT);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
