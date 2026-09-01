/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * cs35l34.h -- CS35L34 ALSA SoC audio driver
 *
 * Copyright 2016 Cirrus Logic, Inc.
 *
 * Author: Paul Handrigan <Paul.Handrigan@cirrus.com>
 */

pub const CS35L34_CHIP_ID: u32 = 0x00035A34;
pub const CS35L34_DEVID_AB: u32 = 0x01; /* Device ID A & B [RO] */
pub const CS35L34_DEVID_CD: u32 = 0x02; /* Device ID C & D [RO] */
pub const CS35L34_DEVID_E: u32 = 0x03; /* Device ID E [RO] */
pub const CS35L34_FAB_ID: u32 = 0x04; /* Fab ID [RO] */
pub const CS35L34_REV_ID: u32 = 0x05; /* Revision ID [RO] */
pub const CS35L34_PWRCTL1: u32 = 0x06; /* Power Ctl 1 */
pub const CS35L34_PWRCTL2: u32 = 0x07; /* Power Ctl 2 */
pub const CS35L34_PWRCTL3: u32 = 0x08; /* Power Ctl 3 */
pub const CS35L34_ADSP_CLK_CTL: u32 = 0x0A; /* (ADSP) Clock Ctl */
pub const CS35L34_MCLK_CTL: u32 = 0x0B; /* Master Clocking Ctl */
pub const CS35L34_AMP_INP_DRV_CTL: u32 = 0x14; /* Amp Input Drive Ctl */
pub const CS35L34_AMP_DIG_VOL_CTL: u32 = 0x15; /* Amplifier Dig Volume Ctl */
pub const CS35L34_AMP_DIG_VOL: u32 = 0x16; /* Amplifier Dig Volume */
pub const CS35L34_AMP_ANLG_GAIN_CTL: u32 = 0x17; /* Amplifier Analog Gain Ctl */
pub const CS35L34_PROTECT_CTL: u32 = 0x18; /* Amp Gain - Prot Ctl Param */
pub const CS35L34_AMP_KEEP_ALIVE_CTL: u32 = 0x1A; /* Amplifier Keep Alive Ctl */
pub const CS35L34_BST_CVTR_V_CTL: u32 = 0x1D; /* Boost Conv Voltage Ctl */
pub const CS35L34_BST_PEAK_I: u32 = 0x1E; /* Boost Conv Peak Current */
pub const CS35L34_BST_RAMP_CTL: u32 = 0x20; /* Boost Conv Soft Ramp Ctl */
pub const CS35L34_BST_CONV_COEF_1: u32 = 0x21; /* Boost Conv Coefficients 1 */
pub const CS35L34_BST_CONV_COEF_2: u32 = 0x22; /* Boost Conv Coefficients 2 */
pub const CS35L34_BST_CONV_SLOPE_COMP: u32 = 0x23; /* Boost Conv Slope Comp */
pub const CS35L34_BST_CONV_SW_FREQ: u32 = 0x24; /* Boost Conv L BST SW Freq */
pub const CS35L34_CLASS_H_CTL: u32 = 0x30; /* CLS H Control */
pub const CS35L34_CLASS_H_HEADRM_CTL: u32 = 0x31; /* CLS H Headroom Ctl */
pub const CS35L34_CLASS_H_RELEASE_RATE: u32 = 0x32; /* CLS H Release Rate */
pub const CS35L34_CLASS_H_FET_DRIVE_CTL: u32 = 0x33; /* CLS H Weak FET Drive Ctl */
pub const CS35L34_CLASS_H_STATUS: u32 = 0x38; /* CLS H Status */
pub const CS35L34_VPBR_CTL: u32 = 0x3A; /* VPBR Ctl */
pub const CS35L34_VPBR_VOL_CTL: u32 = 0x3B; /* VPBR Volume Ctl */
pub const CS35L34_VPBR_TIMING_CTL: u32 = 0x3C; /* VPBR Timing Ctl */
pub const CS35L34_PRED_MAX_ATTEN_SPK_LOAD: u32 = 0x40; /* PRD Max Atten / Spkr Load */
pub const CS35L34_PRED_BROWNOUT_THRESH: u32 = 0x41; /* PRD Brownout Threshold */
pub const CS35L34_PRED_BROWNOUT_VOL_CTL: u32 = 0x42; /* PRD Brownout Volume Ctl */
pub const CS35L34_PRED_BROWNOUT_RATE_CTL: u32 = 0x43; /* PRD Brownout Rate Ctl */
pub const CS35L34_PRED_WAIT_CTL: u32 = 0x44; /* PRD Wait Ctl */
pub const CS35L34_PRED_ZVP_INIT_IMP_CTL: u32 = 0x46; /* PRD ZVP Initial Imp Ctl */
pub const CS35L34_PRED_MAN_SAFE_VPI_CTL: u32 = 0x47; /* PRD Manual Safe VPI Ctl */
pub const CS35L34_VPBR_ATTEN_STATUS: u32 = 0x4B; /* VPBR Attenuation Status */
pub const CS35L34_PRED_BRWNOUT_ATT_STATUS: u32 = 0x4C; /* PRD Brownout Atten Status */
pub const CS35L34_SPKR_MON_CTL: u32 = 0x4E; /* Speaker Monitoring Ctl */
pub const CS35L34_ADSP_I2S_CTL: u32 = 0x50; /* ADSP I2S Ctl */
pub const CS35L34_ADSP_TDM_CTL: u32 = 0x51; /* ADSP TDM Ctl */
pub const CS35L34_TDM_TX_CTL_1_VMON: u32 = 0x52; /* TDM TX Ctl 1 (VMON) */
pub const CS35L34_TDM_TX_CTL_2_IMON: u32 = 0x53; /* TDM TX Ctl 2 (IMON) */
pub const CS35L34_TDM_TX_CTL_3_VPMON: u32 = 0x54; /* TDM TX Ctl 3 (VPMON) */
pub const CS35L34_TDM_TX_CTL_4_VBSTMON: u32 = 0x55; /* TDM TX Ctl 4 (VBSTMON) */
pub const CS35L34_TDM_TX_CTL_5_FLAG1: u32 = 0x56; /* TDM TX Ctl 5 (FLAG1) */
pub const CS35L34_TDM_TX_CTL_6_FLAG2: u32 = 0x57; /* TDM TX Ctl 6 (FLAG2) */
pub const CS35L34_TDM_TX_SLOT_EN_1: u32 = 0x5A; /* TDM TX Slot Enable */
pub const CS35L34_TDM_TX_SLOT_EN_2: u32 = 0x5B; /* TDM TX Slot Enable */
pub const CS35L34_TDM_TX_SLOT_EN_3: u32 = 0x5C; /* TDM TX Slot Enable */
pub const CS35L34_TDM_TX_SLOT_EN_4: u32 = 0x5D; /* TDM TX Slot Enable */
pub const CS35L34_TDM_RX_CTL_1_AUDIN: u32 = 0x5E; /* TDM RX Ctl 1 */
pub const CS35L34_TDM_RX_CTL_3_ALIVE: u32 = 0x60; /* TDM RX Ctl 3 (ALIVE) */
pub const CS35L34_MULT_DEV_SYNCH1: u32 = 0x62; /* Multidevice Synch */
pub const CS35L34_MULT_DEV_SYNCH2: u32 = 0x63; /* Multidevice Synch 2 */
pub const CS35L34_PROT_RELEASE_CTL: u32 = 0x64; /* Protection Release Ctl */
pub const CS35L34_DIAG_MODE_REG_LOCK: u32 = 0x68; /* Diagnostic Mode Reg Lock */
pub const CS35L34_DIAG_MODE_CTL_1: u32 = 0x69; /* Diagnostic Mode Ctl 1 */
pub const CS35L34_DIAG_MODE_CTL_2: u32 = 0x6A; /* Diagnostic Mode Ctl 2 */
pub const CS35L34_INT_MASK_1: u32 = 0x70; /* Interrupt Mask 1 */
pub const CS35L34_INT_MASK_2: u32 = 0x71; /* Interrupt Mask 2 */
pub const CS35L34_INT_MASK_3: u32 = 0x72; /* Interrupt Mask 3 */
pub const CS35L34_INT_MASK_4: u32 = 0x73; /* Interrupt Mask 4 */
pub const CS35L34_INT_STATUS_1: u32 = 0x74; /* Interrupt Status 1 */
pub const CS35L34_INT_STATUS_2: u32 = 0x75; /* Interrupt Status 2 */
pub const CS35L34_INT_STATUS_3: u32 = 0x76; /* Interrupt Status 3 */
pub const CS35L34_INT_STATUS_4: u32 = 0x77; /* Interrupt Status 4 */
pub const CS35L34_OTP_TRIM_STATUS: u32 = 0x7E; /* OTP Trim Status */

pub const CS35L34_MAX_REGISTER: u32 = 0x7F;
pub const CS35L34_REGISTER_COUNT: u32 = 0x4E;

pub const CS35L34_MCLK_5644: u32 = 5644800;
pub const CS35L34_MCLK_6144: u32 = 6144000;
pub const CS35L34_MCLK_6: u32 = 6000000;
pub const CS35L34_MCLK_11289: u32 = 11289600;
pub const CS35L34_MCLK_12: u32 = 12000000;
pub const CS35L34_MCLK_12288: u32 = 12288000;

/* CS35L34_PWRCTL1 */
pub const CS35L34_SFT_RST: u32 = 1 << 7;
pub const CS35L34_DISCHG_FLT: u32 = 1 << 1;
pub const CS35L34_PDN_ALL: u32 = 1;

/* CS35L34_PWRCTL2 */
pub const CS35L34_PDN_VMON: u32 = 1 << 7;
pub const CS35L34_PDN_IMON: u32 = 1 << 6;
pub const CS35L34_PDN_CLASSH: u32 = 1 << 5;
pub const CS35L34_PDN_VPBR: u32 = 1 << 4;
pub const CS35L34_PDN_PRED: u32 = 1 << 3;
pub const CS35L34_PDN_BST: u32 = 1 << 2;
pub const CS35L34_PDN_AMP: u32 = 1;

/* CS35L34_PWRCTL3 */
pub const CS35L34_MCLK_DIS: u32 = 1 << 7;
pub const CS35L34_PDN_VBSTMON_OUT: u32 = 1 << 4;
pub const CS35L34_PDN_VMON_OUT: u32 = 1 << 3;
/* Tristate the ADSP SDOUT when in I2C mode */
pub const CS35L34_PDN_SDOUT: u32 = 1 << 2;
pub const CS35L34_PDN_SDIN: u32 = 1 << 1;
pub const CS35L34_PDN_TDM: u32 = 1;

/* CS35L34_ADSP_CLK_CTL */
pub const CS35L34_ADSP_RATE: u32 = 0xF;
pub const CS35L34_ADSP_DRIVE: u32 = 1 << 4;
pub const CS35L34_ADSP_M_S: u32 = 1 << 7;

/* CS35L34_MCLK_CTL */
pub const CS35L34_MCLK_DIV: u32 = 1 << 4;
pub const CS35L34_MCLK_RATE_MASK: u32 = 0x7;
pub const CS35L34_MCLK_RATE_6P1440: u32 = 0x2;
pub const CS35L34_MCLK_RATE_6P0000: u32 = 0x1;
pub const CS35L34_MCLK_RATE_5P6448: u32 = 0x0;
pub const CS35L34_MCLKDIS: u32 = 1 << 7;
pub const CS35L34_MCLKDIV2: u32 = 1 << 6;
pub const CS35L34_SDOUT_3ST_TDM: u32 = 1 << 5;
pub const CS35L34_INT_FS_RATE: u32 = 1 << 4;
pub const CS35L34_ADSP_FS: u32 = 0xF;

/* CS35L34_AMP_INP_DRV_CTL */
pub const CS35L34_DRV_STR_SRC: u32 = 1 << 1;
pub const CS35L34_DRV_STR: u32 = 1;

/* CS35L34_AMP_DIG_VOL_CTL */
pub const CS35L34_AMP_DSR_RATE_MASK: u32 = 0xF0;
pub const CS35L34_AMP_DSR_RATE_SHIFT: u32 = 1 << 4;
pub const CS35L34_NOTCH_DIS: u32 = 1 << 3;
pub const CS35L34_AMP_DIGSFT: u32 = 1 << 1;
pub const CS35L34_INV: u32 = 1;

/* CS35L34_PROTECT_CTL */
pub const CS35L34_OTW_ATTN_MASK: u32 = 0xC;
pub const CS35L34_OTW_THRD_MASK: u32 = 0x3;
pub const CS35L34_MUTE: u32 = 1 << 5;
pub const CS35L34_GAIN_ZC: u32 = 1 << 4;
pub const CS35L34_GAIN_ZC_MASK: u32 = 0x10;
pub const CS35L34_GAIN_ZC_SHIFT: u32 = 4;

/* CS35L34_AMP_KEEP_ALIVE_CTL */
pub const CS35L34_ALIVE_WD_DIS: u32 = 1 << 2;

/* CS35L34_BST_CVTR_V_CTL */
pub const CS35L34_BST_CVTL_MASK: u32 = 0x3F;

/* CS35L34_BST_PEAK_I */
pub const CS35L34_BST_PEAK_MASK: u32 = 0x3F;

/* CS35L34_ADSP_I2S_CTL */
pub const CS35L34_I2S_LOC_MASK: u32 = 0xC;
pub const CS35L34_I2S_LOC_SHIFT: u32 = 2;

/* CS35L34_MULT_DEV_SYNCH2 */
pub const CS35L34_SYNC2_MASK: u32 = 0xF;

/* CS35L34_PROT_RELEASE_CTL */
pub const CS35L34_CAL_ERR_RLS: u32 = 1 << 7;
pub const CS35L34_SHORT_RLS: u32 = 1 << 2;
pub const CS35L34_OTW_RLS: u32 = 1 << 1;
pub const CS35L34_OTE_RLS: u32 = 1;

/* CS35L34_INT_MASK_1 */
pub const CS35L34_M_CAL_ERR_SHIFT: u32 = 7;
pub const CS35L34_M_CAL_ERR: u32 = 1 << CS35L34_M_CAL_ERR_SHIFT;
pub const CS35L34_M_ALIVE_ERR_SHIFT: u32 = 5;
pub const CS35L34_M_ALIVE_ERR: u32 = 1 << CS35L34_M_ALIVE_ERR_SHIFT;
pub const CS35L34_M_ADSP_CLK_SHIFT: u32 = 4;
pub const CS35L34_M_ADSP_CLK_ERR: u32 = 1 << CS35L34_M_ADSP_CLK_SHIFT;
pub const CS35L34_M_MCLK_SHIFT: u32 = 3;
pub const CS35L34_M_MCLK_ERR: u32 = 1 << CS35L34_M_MCLK_SHIFT;
pub const CS35L34_M_AMP_SHORT_SHIFT: u32 = 2;
pub const CS35L34_M_AMP_SHORT: u32 = 1 << CS35L34_M_AMP_SHORT_SHIFT;
pub const CS35L34_M_OTW_SHIFT: u32 = 1;
pub const CS35L34_M_OTW: u32 = 1 << CS35L34_M_OTW_SHIFT;
pub const CS35L34_M_OTE_SHIFT: u32 = 0;
pub const CS35L34_M_OTE: u32 = 1 << CS35L34_M_OTE_SHIFT;

/* CS35L34_INT_MASK_2 */
pub const CS35L34_M_PDN_DONE_SHIFT: u32 = 4;
pub const CS35L34_M_PDN_DONE: u32 = 1 << CS35L34_M_PDN_DONE_SHIFT;
pub const CS35L34_M_PRED_SHIFT: u32 = 3;
pub const CS35L34_M_PRED_ERR: u32 = 1 << CS35L34_M_PRED_SHIFT;
pub const CS35L34_M_PRED_CLR_SHIFT: u32 = 2;
pub const CS35L34_M_PRED_CLR: u32 = 1 << CS35L34_M_PRED_CLR_SHIFT;
pub const CS35L34_M_VPBR_SHIFT: u32 = 1;
pub const CS35L34_M_VPBR_ERR: u32 = 1 << CS35L34_M_VPBR_SHIFT;
pub const CS35L34_M_VPBR_CLR_SHIFT: u32 = 0;
pub const CS35L34_M_VPBR_CLR: u32 = 1 << CS35L34_M_VPBR_CLR_SHIFT;

/* CS35L34_INT_MASK_3 */
pub const CS35L34_M_BST_HIGH_SHIFT: u32 = 4;
pub const CS35L34_M_BST_HIGH: u32 = 1 << CS35L34_M_BST_HIGH_SHIFT;
pub const CS35L34_M_BST_HIGH_FLAG_SHIFT: u32 = 3;
pub const CS35L34_M_BST_HIGH_FLAG: u32 = 1 << CS35L34_M_BST_HIGH_FLAG_SHIFT;
pub const CS35L34_M_BST_IPK_FLAG_SHIFT: u32 = 2;
pub const CS35L34_M_BST_IPK_FLAG: u32 = 1 << CS35L34_M_BST_IPK_FLAG_SHIFT;
pub const CS35L34_M_LBST_SHORT_SHIFT: u32 = 0;
pub const CS35L34_M_LBST_SHORT: u32 = 1 << CS35L34_M_LBST_SHORT_SHIFT;

/* CS35L34_INT_MASK_4 */
pub const CS35L34_M_VMON_OVFL_SHIFT: u32 = 3;
pub const CS35L34_M_VMON_OVFL: u32 = 1 << CS35L34_M_VMON_OVFL_SHIFT;
pub const CS35L34_M_IMON_OVFL_SHIFT: u32 = 2;
pub const CS35L34_M_IMON_OVFL: u32 = 1 << CS35L34_M_IMON_OVFL_SHIFT;
pub const CS35L34_M_VPMON_OVFL_SHIFT: u32 = 1;
pub const CS35L34_M_VPMON_OVFL: u32 = 1 << CS35L34_M_VPMON_OVFL_SHIFT;
pub const CS35L34_M_VBSTMON_OVFL_SHIFT: u32 = 1;
pub const CS35L34_M_VBSTMON_OVFL: u32 = 1 << CS35L34_M_VBSTMON_OVFL_SHIFT;

/* CS35L34_INT_1 */
pub const CS35L34_CAL_ERR: u32 = 1 << CS35L34_M_CAL_ERR_SHIFT;
pub const CS35L34_ALIVE_ERR: u32 = 1 << CS35L34_M_ALIVE_ERR_SHIFT;
/* Source repeats CS35L34_M_ADSP_CLK_ERR here with the same definition. */
pub const CS35L34_MCLK_ERR: u32 = 1 << CS35L34_M_MCLK_SHIFT;
pub const CS35L34_AMP_SHORT: u32 = 1 << CS35L34_M_AMP_SHORT_SHIFT;
pub const CS35L34_OTW: u32 = 1 << CS35L34_M_OTW_SHIFT;
pub const CS35L34_OTE: u32 = 1 << CS35L34_M_OTE_SHIFT;

/* CS35L34_INT_2 */
pub const CS35L34_PDN_DONE: u32 = 1 << CS35L34_M_PDN_DONE_SHIFT;
pub const CS35L34_PRED_ERR: u32 = 1 << CS35L34_M_PRED_SHIFT;
pub const CS35L34_PRED_CLR: u32 = 1 << CS35L34_M_PRED_CLR_SHIFT;
pub const CS35L34_VPBR_ERR: u32 = 1 << CS35L34_M_VPBR_SHIFT;
pub const CS35L34_VPBR_CLR: u32 = 1 << CS35L34_M_VPBR_CLR_SHIFT;

/* CS35L34_INT_3 */
pub const CS35L34_BST_HIGH: u32 = 1 << CS35L34_M_BST_HIGH_SHIFT;
pub const CS35L34_BST_HIGH_FLAG: u32 = 1 << CS35L34_M_BST_HIGH_FLAG_SHIFT;
pub const CS35L34_BST_IPK_FLAG: u32 = 1 << CS35L34_M_BST_IPK_FLAG_SHIFT;
pub const CS35L34_LBST_SHORT: u32 = 1 << CS35L34_M_LBST_SHORT_SHIFT;

/* CS35L34_INT_4 */
pub const CS35L34_VMON_OVFL: u32 = 1 << CS35L34_M_VMON_OVFL_SHIFT;
pub const CS35L34_IMON_OVFL: u32 = 1 << CS35L34_M_IMON_OVFL_SHIFT;
pub const CS35L34_VPMON_OVFL: u32 = 1 << CS35L34_M_VPMON_OVFL_SHIFT;
pub const CS35L34_VBSTMON_OVFL: u32 = 1 << CS35L34_M_VBSTMON_OVFL_SHIFT;

/* CS35L34_{RX,TX}_X */
pub const CS35L34_X_STATE_SHIFT: u32 = 7;
pub const CS35L34_X_STATE: u32 = 1 << CS35L34_X_STATE_SHIFT;
pub const CS35L34_X_LOC_SHIFT: u32 = 0;
pub const CS35L34_X_LOC: u32 = 0x1F << CS35L34_X_LOC_SHIFT;

pub const CS35L34_RATES: u32 = SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_32000;
pub const CS35L34_FORMATS: u32 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
