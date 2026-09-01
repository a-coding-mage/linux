/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * cs35l32.h -- CS35L32 ALSA SoC audio driver
 *
 * Copyright 2014 CirrusLogic, Inc.
 *
 * Author: Brian Austin <brian.austin@cirrus.com>
 */

#[repr(C)]
pub struct cs35l32_platform_data {
    /* Low Battery Threshold */
    pub batt_thresh: ::std::os::raw::c_uint,
    /* Low Battery Recovery */
    pub batt_recov: ::std::os::raw::c_uint,
    /* LED Current Management*/
    pub led_mng: ::std::os::raw::c_uint,
    /* Audio Gain w/ LED */
    pub audiogain_mng: ::std::os::raw::c_uint,
    /* Boost Management */
    pub boost_mng: ::std::os::raw::c_uint,
    /* Data CFG for DUAL device */
    pub sdout_datacfg: ::std::os::raw::c_uint,
    /* SDOUT Sharing */
    pub sdout_share: ::std::os::raw::c_uint,
}

pub const CS35L32_CHIP_ID: u32 = 0x00035A32;
pub const CS35L32_DEVID_AB: u32 = 0x01; /* Device ID A & B [RO] */
pub const CS35L32_DEVID_CD: u32 = 0x02; /* Device ID C & D [RO] */
pub const CS35L32_DEVID_E: u32 = 0x03; /* Device ID E [RO] */
pub const CS35L32_FAB_ID: u32 = 0x04; /* Fab ID [RO] */
pub const CS35L32_REV_ID: u32 = 0x05; /* Revision ID [RO] */
pub const CS35L32_PWRCTL1: u32 = 0x06; /* Power Ctl 1 */
pub const CS35L32_PWRCTL2: u32 = 0x07; /* Power Ctl 2 */
pub const CS35L32_CLK_CTL: u32 = 0x08; /* Clock Ctl */
pub const CS35L32_BATT_THRESHOLD: u32 = 0x09; /* Low Battery Threshold */
pub const CS35L32_VMON: u32 = 0x0A; /* Voltage Monitor [RO] */
pub const CS35L32_BST_CPCP_CTL: u32 = 0x0B; /* Conv Peak Curr Protection CTL */
pub const CS35L32_IMON_SCALING: u32 = 0x0C; /* IMON Scaling */
pub const CS35L32_AUDIO_LED_MNGR: u32 = 0x0D; /* Audio/LED Pwr Manager */
pub const CS35L32_ADSP_CTL: u32 = 0x0F; /* Serial Port Control */
pub const CS35L32_CLASSD_CTL: u32 = 0x10; /* Class D Amp CTL */
pub const CS35L32_PROTECT_CTL: u32 = 0x11; /* Protection Release CTL */
pub const CS35L32_INT_MASK_1: u32 = 0x12; /* Interrupt Mask 1 */
pub const CS35L32_INT_MASK_2: u32 = 0x13; /* Interrupt Mask 2 */
pub const CS35L32_INT_MASK_3: u32 = 0x14; /* Interrupt Mask 3 */
pub const CS35L32_INT_STATUS_1: u32 = 0x15; /* Interrupt Status 1 [RO] */
pub const CS35L32_INT_STATUS_2: u32 = 0x16; /* Interrupt Status 2 [RO] */
pub const CS35L32_INT_STATUS_3: u32 = 0x17; /* Interrupt Status 3 [RO] */
pub const CS35L32_LED_STATUS: u32 = 0x18; /* LED Lighting Status [RO] */
pub const CS35L32_FLASH_MODE: u32 = 0x19; /* LED Flash Mode Current */
pub const CS35L32_MOVIE_MODE: u32 = 0x1A; /* LED Movie Mode Current */
pub const CS35L32_FLASH_TIMER: u32 = 0x1B; /* LED Flash Timer */
pub const CS35L32_FLASH_INHIBIT: u32 = 0x1C; /* LED Flash Inhibit Current */
pub const CS35L32_MAX_REGISTER: u32 = 0x1C;

pub const CS35L32_MCLK_DIV2: u32 = 0x01;
pub const CS35L32_MCLK_RATIO: u32 = 0x01;
pub const CS35L32_MCLKDIS: u32 = 0x80;
pub const CS35L32_PDN_ALL: u32 = 0x01;
pub const CS35L32_PDN_AMP: u32 = 0x80;
pub const CS35L32_PDN_BOOST: u32 = 0x04;
pub const CS35L32_PDN_IMON: u32 = 0x40;
pub const CS35L32_PDN_VMON: u32 = 0x80;
pub const CS35L32_PDN_VPMON: u32 = 0x20;
pub const CS35L32_PDN_ADSP: u32 = 0x08;

pub const CS35L32_MCLK_DIV2_MASK: u32 = 0x40;
pub const CS35L32_MCLK_RATIO_MASK: u32 = 0x01;
pub const CS35L32_MCLK_MASK: u32 = 0x41;
pub const CS35L32_ADSP_MASTER_MASK: u32 = 0x40;
pub const CS35L32_BOOST_MASK: u32 = 0x03;
pub const CS35L32_GAIN_MGR_MASK: u32 = 0x08;
pub const CS35L32_ADSP_SHARE_MASK: u32 = 0x08;
pub const CS35L32_ADSP_DATACFG_MASK: u32 = 0x30;
pub const CS35L32_SDOUT_3ST: u32 = 0x08;
pub const CS35L32_BATT_REC_MASK: u32 = 0x0E;
pub const CS35L32_BATT_THRESH_MASK: u32 = 0x30;

pub const CS35L32_RATES: u32 = SNDRV_PCM_RATE_48000;
pub const CS35L32_FORMATS: u32 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
