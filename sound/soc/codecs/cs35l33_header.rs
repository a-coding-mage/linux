// SPDX-License-Identifier: GPL-2.0-only
/*
 * cs35l33.h -- CS35L33 ALSA SoC audio driver
 *
 * Copyright 2016 Cirrus Logic, Inc.
 *
 * Author: Paul Handrigan <paul.handrigan@cirrus.com>
 */

pub const CS35L33_CHIP_ID: u32 = 0x00035A33;
pub const CS35L33_DEVID_AB: u32 = 0x01; /* Device ID A & B [RO] */
pub const CS35L33_DEVID_CD: u32 = 0x02; /* Device ID C & D [RO] */
pub const CS35L33_DEVID_E: u32 = 0x03; /* Device ID E [RO] */
pub const CS35L33_FAB_ID: u32 = 0x04; /* Fab ID [RO] */
pub const CS35L33_REV_ID: u32 = 0x05; /* Revision ID [RO] */
pub const CS35L33_PWRCTL1: u32 = 0x06; /* Power Ctl 1 */
pub const CS35L33_PWRCTL2: u32 = 0x07; /* Power Ctl 2 */
pub const CS35L33_CLK_CTL: u32 = 0x08; /* Clock Ctl */
pub const CS35L33_BST_PEAK_CTL: u32 = 0x09; /* Max Current for Boost */
pub const CS35L33_PROTECT_CTL: u32 = 0x0A; /* Amp Protection Parameters */
pub const CS35L33_BST_CTL1: u32 = 0x0B; /* Boost Converter CTL1 */
pub const CS35L33_BST_CTL2: u32 = 0x0C; /* Boost Converter CTL2 */
pub const CS35L33_ADSP_CTL: u32 = 0x0D; /* Serial Port Control */
pub const CS35L33_ADC_CTL: u32 = 0x0E; /* ADC Control */
pub const CS35L33_DAC_CTL: u32 = 0x0F; /* DAC Control */
pub const CS35L33_DIG_VOL_CTL: u32 = 0x10; /* Digital Volume CTL */
pub const CS35L33_CLASSD_CTL: u32 = 0x11; /* Class D Amp CTL */
pub const CS35L33_AMP_CTL: u32 = 0x12; /* Amp Gain/Protecton Release CTL */
pub const CS35L33_INT_MASK_1: u32 = 0x13; /* Interrupt Mask 1 */
pub const CS35L33_INT_MASK_2: u32 = 0x14; /* Interrupt Mask 2 */
pub const CS35L33_INT_STATUS_1: u32 = 0x15; /* Interrupt Status 1 [RO] */
pub const CS35L33_INT_STATUS_2: u32 = 0x16; /* Interrupt Status 2 [RO] */
pub const CS35L33_DIAG_LOCK: u32 = 0x17; /* Diagnostic Mode Register Lock */
pub const CS35L33_DIAG_CTRL_1: u32 = 0x18; /* Diagnostic Mode Register Control */
pub const CS35L33_DIAG_CTRL_2: u32 = 0x19; /* Diagnostic Mode Register Control 2 */
pub const CS35L33_HG_MEMLDO_CTL: u32 = 0x23; /* H/G Memory/LDO CTL */
pub const CS35L33_HG_REL_RATE: u32 = 0x24; /* H/G Release Rate */
pub const CS35L33_LDO_DEL: u32 = 0x25; /* LDO Entry Delay/VPhg Control 1 */
pub const CS35L33_HG_HEAD: u32 = 0x29; /* H/G Headroom */
pub const CS35L33_HG_EN: u32 = 0x2A; /* H/G Enable/VPhg CNT2 */
pub const CS35L33_TX_VMON: u32 = 0x2D; /* TDM TX Control 1 (VMON) */
pub const CS35L33_TX_IMON: u32 = 0x2E; /* TDM TX Control 2 (IMON) */
pub const CS35L33_TX_VPMON: u32 = 0x2F; /* TDM TX Control 3 (VPMON) */
pub const CS35L33_TX_VBSTMON: u32 = 0x30; /* TDM TX Control 4 (VBSTMON) */
pub const CS35L33_TX_FLAG: u32 = 0x31; /* TDM TX Control 5 (FLAG) */
pub const CS35L33_TX_EN1: u32 = 0x32; /* TDM TX Enable 1 */
pub const CS35L33_TX_EN2: u32 = 0x33; /* TDM TX Enable 2 */
pub const CS35L33_TX_EN3: u32 = 0x34; /* TDM TX Enable 3 */
pub const CS35L33_TX_EN4: u32 = 0x35; /* TDM TX Enable 4 */
pub const CS35L33_RX_AUD: u32 = 0x36; /* TDM RX Control 1 */
pub const CS35L33_RX_SPLY: u32 = 0x37; /* TDM RX Control 2 */
pub const CS35L33_RX_ALIVE: u32 = 0x38; /* TDM RX Control 3 */
pub const CS35L33_BST_CTL4: u32 = 0x39; /* Boost Converter Control 4 */
pub const CS35L33_HG_STATUS: u32 = 0x3F; /* H/G Status */
pub const CS35L33_MAX_REGISTER: u32 = 0x59;

pub const CS35L33_MCLK_5644: u32 = 5644800;
pub const CS35L33_MCLK_6144: u32 = 6144000;
pub const CS35L33_MCLK_6: u32 = 6000000;
pub const CS35L33_MCLK_11289: u32 = 11289600;
pub const CS35L33_MCLK_12: u32 = 12000000;
pub const CS35L33_MCLK_12288: u32 = 12288000;

/* CS35L33_PWRCTL1 */
pub const CS35L33_PDN_AMP: u32 = 1 << 7;
pub const CS35L33_PDN_BST: u32 = 1 << 2;
pub const CS35L33_PDN_ALL: u32 = 1;

/* CS35L33_PWRCTL2 */
pub const CS35L33_PDN_VMON_SHIFT: u32 = 7;
pub const CS35L33_PDN_VMON: u32 = 1 << CS35L33_PDN_VMON_SHIFT;
pub const CS35L33_PDN_IMON_SHIFT: u32 = 6;
pub const CS35L33_PDN_IMON: u32 = 1 << CS35L33_PDN_IMON_SHIFT;
pub const CS35L33_PDN_VPMON_SHIFT: u32 = 5;
pub const CS35L33_PDN_VPMON: u32 = 1 << CS35L33_PDN_VPMON_SHIFT;
pub const CS35L33_PDN_VBSTMON_SHIFT: u32 = 4;
pub const CS35L33_PDN_VBSTMON: u32 = 1 << CS35L33_PDN_VBSTMON_SHIFT;
pub const CS35L33_SDOUT_3ST_I2S_SHIFT: u32 = 3;
pub const CS35L33_SDOUT_3ST_I2S: u32 = 1 << CS35L33_SDOUT_3ST_I2S_SHIFT;
pub const CS35L33_PDN_SDIN_SHIFT: u32 = 2;
pub const CS35L33_PDN_SDIN: u32 = 1 << CS35L33_PDN_SDIN_SHIFT;
pub const CS35L33_PDN_TDM_SHIFT: u32 = 1;
pub const CS35L33_PDN_TDM: u32 = 1 << CS35L33_PDN_TDM_SHIFT;

/* CS35L33_CLK_CTL */
pub const CS35L33_MCLKDIS: u32 = 1 << 7;
pub const CS35L33_MCLKDIV2: u32 = 1 << 6;
pub const CS35L33_SDOUT_3ST_TDM: u32 = 1 << 5;
pub const CS35L33_INT_FS_RATE: u32 = 1 << 4;
pub const CS35L33_ADSP_FS: u32 = 0xF;

/* CS35L33_PROTECT_CTL */
pub const CS35L33_ALIVE_WD_DIS: u32 = 3 << 2;

/* CS35L33_BST_CTL1 */
pub const CS35L33_BST_CTL_SRC: u32 = 1 << 6;
pub const CS35L33_BST_CTL_SHIFT: u32 = 1 << 5;
pub const CS35L33_BST_CTL_MASK: u32 = 0x3F;

/* CS35L33_BST_CTL2 */
pub const CS35L33_TDM_WD_SEL: u32 = 1 << 4;
pub const CS35L33_ALIVE_WD_DIS2: u32 = 1 << 3;
pub const CS35L33_VBST_SR_STEP: u32 = 0x3;

/* CS35L33_ADSP_CTL */
pub const CS35L33_ADSP_DRIVE: u32 = 1 << 7;
pub const CS35L33_MS_MASK: u32 = 1 << 6;
pub const CS35L33_SDIN_LOC: u32 = 3 << 4;
pub const CS35L33_ALIVE_RATE: u32 = 0x3;

/* CS35L33_ADC_CTL */
pub const CS35L33_INV_VMON: u32 = 1 << 7;
pub const CS35L33_INV_IMON: u32 = 1 << 6;
pub const CS35L33_ADC_NOTCH_DIS: u32 = 1 << 5;
pub const CS35L33_IMON_SCALE: u32 = 0xF;

/* CS35L33_DAC_CTL */
pub const CS35L33_INV_DAC: u32 = 1 << 7;
pub const CS35L33_DAC_NOTCH_DIS: u32 = 1 << 5;
pub const CS35L33_DIGSFT: u32 = 1 << 4;
pub const CS35L33_DSR_RATE: u32 = 0xF;

/* CS35L33_CLASSD_CTL */
pub const CS35L33_AMP_SD: u32 = 1 << 6;
pub const CS35L33_AMP_DRV_SEL_SRC: u32 = 1 << 5;
pub const CS35L33_AMP_DRV_SEL_MASK: u32 = 0x10;
pub const CS35L33_AMP_DRV_SEL_SHIFT: u32 = 4;
pub const CS35L33_AMP_CAL: u32 = 1 << 3;
pub const CS35L33_GAIN_CHG_ZC_MASK: u32 = 0x04;
pub const CS35L33_GAIN_CHG_ZC_SHIFT: u32 = 2;
pub const CS35L33_CLASS_D_CTL_MASK: u32 = 0x3F;

/* CS35L33_AMP_CTL */
pub const CS35L33_AMP_GAIN: u32 = 0xF0;
pub const CS35L33_CAL_ERR_RLS: u32 = 1 << 3;
pub const CS35L33_AMP_SHORT_RLS: u32 = 1 << 2;
pub const CS35L33_OTW_RLS: u32 = 1 << 1;
pub const CS35L33_OTE_RLS: u32 = 1;

/* CS35L33_INT_MASK_1 */
pub const CS35L33_M_CAL_ERR_SHIFT: u32 = 6;
pub const CS35L33_M_CAL_ERR: u32 = 1 << CS35L33_M_CAL_ERR_SHIFT;
pub const CS35L33_M_ALIVE_ERR_SHIFT: u32 = 5;
pub const CS35L33_M_ALIVE_ERR: u32 = 1 << CS35L33_M_ALIVE_ERR_SHIFT;
pub const CS35L33_M_AMP_SHORT_SHIFT: u32 = 2;
pub const CS35L33_M_AMP_SHORT: u32 = 1 << CS35L33_M_AMP_SHORT_SHIFT;
pub const CS35L33_M_OTW_SHIFT: u32 = 1;
pub const CS35L33_M_OTW: u32 = 1 << CS35L33_M_OTW_SHIFT;
pub const CS35L33_M_OTE_SHIFT: u32 = 0;
pub const CS35L33_M_OTE: u32 = 1 << CS35L33_M_OTE_SHIFT;

/* CS35L33_INT_STATUS_1 */
pub const CS35L33_CAL_ERR: u32 = 1 << 6;
pub const CS35L33_ALIVE_ERR: u32 = 1 << 5;
pub const CS35L33_ADSPCLK_ERR: u32 = 1 << 4;
pub const CS35L33_MCLK_ERR: u32 = 1 << 3;
pub const CS35L33_AMP_SHORT: u32 = 1 << 2;
pub const CS35L33_OTW: u32 = 1 << 1;
pub const CS35L33_OTE: u32 = 1 << 0;

/* CS35L33_INT_STATUS_2 */
pub const CS35L33_VMON_OVFL: u32 = 1 << 7;
pub const CS35L33_IMON_OVFL: u32 = 1 << 6;
pub const CS35L33_VPMON_OVFL: u32 = 1 << 5;
pub const CS35L33_VBSTMON_OVFL: u32 = 1 << 4;
pub const CS35L33_PDN_DONE: u32 = 1;

/* CS35L33_BST_CTL4 */
pub const CS35L33_BST_RGS: u32 = 0x70;
pub const CS35L33_BST_COEFF3: u32 = 0xF;

/* CS35L33_HG_MEMLDO_CTL */
pub const CS35L33_MEM_DEPTH_SHIFT: u32 = 5;
pub const CS35L33_MEM_DEPTH_MASK: u32 = 0x3 << CS35L33_MEM_DEPTH_SHIFT;
pub const CS35L33_LDO_THLD_SHIFT: u32 = 1;
pub const CS35L33_LDO_THLD_MASK: u32 = 0xF << CS35L33_LDO_THLD_SHIFT;
pub const CS35L33_LDO_DISABLE_SHIFT: u32 = 0;
pub const CS35L33_LDO_DISABLE_MASK: u32 = 0x1 << CS35L33_LDO_DISABLE_SHIFT;

/* CS35L33_LDO_DEL */
pub const CS35L33_VP_HG_VA_SHIFT: u32 = 5;
pub const CS35L33_VP_HG_VA_MASK: u32 = 0x7 << CS35L33_VP_HG_VA_SHIFT;
pub const CS35L33_LDO_ENTRY_DELAY_SHIFT: u32 = 2;
pub const CS35L33_LDO_ENTRY_DELAY_MASK: u32 = 0x7 << CS35L33_LDO_ENTRY_DELAY_SHIFT;
pub const CS35L33_VP_HG_RATE_SHIFT: u32 = 0;
pub const CS35L33_VP_HG_RATE_MASK: u32 = 0x3 << CS35L33_VP_HG_RATE_SHIFT;

/* CS35L33_HG_HEAD */
pub const CS35L33_HD_RM_SHIFT: u32 = 0;
pub const CS35L33_HD_RM_MASK: u32 = 0x7F << CS35L33_HD_RM_SHIFT;

/* CS35L33_HG_EN */
pub const CS35L33_CLASS_HG_ENA_SHIFT: u32 = 7;
pub const CS35L33_CLASS_HG_EN_MASK: u32 = 0x1 << CS35L33_CLASS_HG_ENA_SHIFT;
pub const CS35L33_VP_HG_AUTO_SHIFT: u32 = 6;
pub const CS35L33_VP_HG_AUTO_MASK: u32 = 0x1 << 6;
pub const CS35L33_VP_HG_SHIFT: u32 = 0;
pub const CS35L33_VP_HG_MASK: u32 = 0x1F << CS35L33_VP_HG_SHIFT;

// Depends on ALSA PCM constants supplied by other translated files.
pub const CS35L33_RATES: u32 = SNDRV_PCM_RATE_8000_48000;
pub const CS35L33_FORMATS: u32 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE;

/* CS35L33_{RX,TX}_X */
pub const CS35L33_X_STATE_SHIFT: u32 = 7;
pub const CS35L33_X_STATE: u32 = 1 << CS35L33_X_STATE_SHIFT;
pub const CS35L33_X_LOC_SHIFT: u32 = 0;
pub const CS35L33_X_LOC: u32 = 0x1F << CS35L33_X_LOC_SHIFT;

/* CS35L33_RX_AUD */
pub const CS35L33_AUDIN_RX_DEPTH_SHIFT: u32 = 5;
pub const CS35L33_AUDIN_RX_DEPTH: u32 = 0x7 << CS35L33_AUDIN_RX_DEPTH_SHIFT;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
