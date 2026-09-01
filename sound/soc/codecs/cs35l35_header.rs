/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * cs35l35.h -- CS35L35 ALSA SoC audio driver
 *
 * Copyright 2016 Cirrus Logic, Inc.
 *
 * Author: Brian Austin <brian.austin@cirrus.com>
 */

/* Original C header included header guards only; no Rust module wrapper added. */

pub const CS35L35_FIRSTREG: u32 = 0x01;
pub const CS35L35_LASTREG: u32 = 0x7E;
pub const CS35L35_CHIP_ID: u32 = 0x00035A35;
pub const CS35L35_DEVID_AB: u32 = 0x01; /* Device ID A & B [RO] */
pub const CS35L35_DEVID_CD: u32 = 0x02; /* Device ID C & D [RO] */
pub const CS35L35_DEVID_E: u32 = 0x03; /* Device ID E [RO] */
pub const CS35L35_FAB_ID: u32 = 0x04; /* Fab ID [RO] */
pub const CS35L35_REV_ID: u32 = 0x05; /* Revision ID [RO] */
pub const CS35L35_PWRCTL1: u32 = 0x06; /* Power Ctl 1 */
pub const CS35L35_PWRCTL2: u32 = 0x07; /* Power Ctl 2 */
pub const CS35L35_PWRCTL3: u32 = 0x08; /* Power Ctl 3 */
pub const CS35L35_CLK_CTL1: u32 = 0x0A; /* Clocking Ctl 1 */
pub const CS35L35_CLK_CTL2: u32 = 0x0B; /* Clocking Ctl 2 */
pub const CS35L35_CLK_CTL3: u32 = 0x0C; /* Clocking Ctl 3 */
pub const CS35L35_SP_FMT_CTL1: u32 = 0x0D; /* Serial Port Format CTL1 */
pub const CS35L35_SP_FMT_CTL2: u32 = 0x0E; /* Serial Port Format CTL2 */
pub const CS35L35_SP_FMT_CTL3: u32 = 0x0F; /* Serial Port Format CTL3 */
pub const CS35L35_MAG_COMP_CTL: u32 = 0x13; /* Magnitude Comp CTL */
pub const CS35L35_AMP_INP_DRV_CTL: u32 = 0x14; /* Amp Input Drive Ctl */
pub const CS35L35_AMP_DIG_VOL_CTL: u32 = 0x15; /* Amplifier Dig Volume Ctl */
pub const CS35L35_AMP_DIG_VOL: u32 = 0x16; /* Amplifier Dig Volume */
pub const CS35L35_ADV_DIG_VOL: u32 = 0x17; /* Advisory Digital Volume */
pub const CS35L35_PROTECT_CTL: u32 = 0x18; /* Amp Gain - Prot Ctl Param */
pub const CS35L35_AMP_GAIN_AUD_CTL: u32 = 0x19; /* Amp Serial Port Gain Ctl */
pub const CS35L35_AMP_GAIN_PDM_CTL: u32 = 0x1A; /* Amplifier Gain PDM Ctl */
pub const CS35L35_AMP_GAIN_ADV_CTL: u32 = 0x1B; /* Amplifier Gain Ctl */
pub const CS35L35_GPI_CTL: u32 = 0x1C; /* GPI Ctl */
pub const CS35L35_BST_CVTR_V_CTL: u32 = 0x1D; /* Boost Conv Voltage Ctl */
pub const CS35L35_BST_PEAK_I: u32 = 0x1E; /* Boost Conv Peak Current */
pub const CS35L35_BST_RAMP_CTL: u32 = 0x20; /* Boost Conv Soft Ramp Ctl */
pub const CS35L35_BST_CONV_COEF_1: u32 = 0x21; /* Boost Conv Coefficients 1 */
pub const CS35L35_BST_CONV_COEF_2: u32 = 0x22; /* Boost Conv Coefficients 2 */
pub const CS35L35_BST_CONV_SLOPE_COMP: u32 = 0x23; /* Boost Conv Slope Comp */
pub const CS35L35_BST_CONV_SW_FREQ: u32 = 0x24; /* Boost Conv L BST SW Freq */
pub const CS35L35_CLASS_H_CTL: u32 = 0x30; /* CLS H Control */
pub const CS35L35_CLASS_H_HEADRM_CTL: u32 = 0x31; /* CLS H Headroom Ctl */
pub const CS35L35_CLASS_H_RELEASE_RATE: u32 = 0x32; /* CLS H Release Rate */
pub const CS35L35_CLASS_H_FET_DRIVE_CTL: u32 = 0x33; /* CLS H Weak FET Drive Ctl */
pub const CS35L35_CLASS_H_VP_CTL: u32 = 0x34; /* CLS H VP Ctl */
pub const CS35L35_CLASS_H_STATUS: u32 = 0x38; /* CLS H Status */
pub const CS35L35_VPBR_CTL: u32 = 0x3A; /* VPBR Ctl */
pub const CS35L35_VPBR_VOL_CTL: u32 = 0x3B; /* VPBR Volume Ctl */
pub const CS35L35_VPBR_TIMING_CTL: u32 = 0x3C; /* VPBR Timing Ctl */
pub const CS35L35_VPBR_MODE_VOL_CTL: u32 = 0x3D; /* VPBR Mode/Attack Vol Ctl */
pub const CS35L35_VPBR_ATTEN_STATUS: u32 = 0x4B; /* VPBR Attenuation Status */
pub const CS35L35_SPKR_MON_CTL: u32 = 0x4E; /* Speaker Monitoring Ctl */
pub const CS35L35_IMON_SCALE_CTL: u32 = 0x51; /* IMON Scale Ctl */
pub const CS35L35_AUDIN_RXLOC_CTL: u32 = 0x52; /* Audio Input RX Loc Ctl */
pub const CS35L35_ADVIN_RXLOC_CTL: u32 = 0x53; /* Advisory Input RX Loc Ctl */
pub const CS35L35_VMON_TXLOC_CTL: u32 = 0x54; /* VMON TX Loc Ctl */
pub const CS35L35_IMON_TXLOC_CTL: u32 = 0x55; /* IMON TX Loc Ctl */
pub const CS35L35_VPMON_TXLOC_CTL: u32 = 0x56; /* VPMON TX Loc Ctl */
pub const CS35L35_VBSTMON_TXLOC_CTL: u32 = 0x57; /* VBSTMON TX Loc Ctl */
pub const CS35L35_VPBR_STATUS_TXLOC_CTL: u32 = 0x58; /* VPBR Status TX Loc Ctl */
pub const CS35L35_ZERO_FILL_LOC_CTL: u32 = 0x59; /* Zero Fill Loc Ctl */
pub const CS35L35_AUDIN_DEPTH_CTL: u32 = 0x5A; /* Audio Input Depth Ctl */
pub const CS35L35_SPKMON_DEPTH_CTL: u32 = 0x5B; /* SPK Mon Output Depth Ctl */
pub const CS35L35_SUPMON_DEPTH_CTL: u32 = 0x5C; /* Supply Mon Out Depth Ctl */
pub const CS35L35_ZEROFILL_DEPTH_CTL: u32 = 0x5D; /* Zero Fill Mon Output Ctl */
pub const CS35L35_MULT_DEV_SYNCH1: u32 = 0x62; /* Multidevice Synch */
pub const CS35L35_MULT_DEV_SYNCH2: u32 = 0x63; /* Multidevice Synch 2 */
pub const CS35L35_PROT_RELEASE_CTL: u32 = 0x64; /* Protection Release Ctl */
pub const CS35L35_DIAG_MODE_REG_LOCK: u32 = 0x68; /* Diagnostic Mode Reg Lock */
pub const CS35L35_DIAG_MODE_CTL_1: u32 = 0x69; /* Diagnostic Mode Ctl 1 */
pub const CS35L35_DIAG_MODE_CTL_2: u32 = 0x6A; /* Diagnostic Mode Ctl 2 */
pub const CS35L35_INT_MASK_1: u32 = 0x70; /* Interrupt Mask 1 */
pub const CS35L35_INT_MASK_2: u32 = 0x71; /* Interrupt Mask 2 */
pub const CS35L35_INT_MASK_3: u32 = 0x72; /* Interrupt Mask 3 */
pub const CS35L35_INT_MASK_4: u32 = 0x73; /* Interrupt Mask 4 */
pub const CS35L35_INT_STATUS_1: u32 = 0x74; /* Interrupt Status 1 */
pub const CS35L35_INT_STATUS_2: u32 = 0x75; /* Interrupt Status 2 */
pub const CS35L35_INT_STATUS_3: u32 = 0x76; /* Interrupt Status 3 */
pub const CS35L35_INT_STATUS_4: u32 = 0x77; /* Interrupt Status 4 */
pub const CS35L35_PLL_STATUS: u32 = 0x78; /* PLL Status */
pub const CS35L35_OTP_TRIM_STATUS: u32 = 0x7E; /* OTP Trim Status */

pub const CS35L35_MAX_REGISTER: u32 = 0x7F;

/* CS35L35_PWRCTL1 */
pub const CS35L35_SFT_RST: u32 = 0x80;
pub const CS35L35_DISCHG_FLT: u32 = 0x02;
pub const CS35L35_PDN_ALL: u32 = 0x01;

/* CS35L35_PWRCTL2 */
pub const CS35L35_PDN_VMON: u32 = 0x80;
pub const CS35L35_PDN_IMON: u32 = 0x40;
pub const CS35L35_PDN_CLASSH: u32 = 0x20;
pub const CS35L35_PDN_VPBR: u32 = 0x10;
pub const CS35L35_PDN_BST: u32 = 0x04;
pub const CS35L35_PDN_AMP: u32 = 0x01;

/* CS35L35_PWRCTL3 */
pub const CS35L35_PDN_VBSTMON_OUT: u32 = 0x10;
pub const CS35L35_PDN_VMON_OUT: u32 = 0x08;

pub const CS35L35_AUDIN_DEPTH_MASK: u32 = 0x03;
pub const CS35L35_AUDIN_DEPTH_SHIFT: u32 = 0;
pub const CS35L35_ADVIN_DEPTH_MASK: u32 = 0x0C;
pub const CS35L35_ADVIN_DEPTH_SHIFT: u32 = 2;
pub const CS35L35_SDIN_DEPTH_8: u32 = 0x01;
pub const CS35L35_SDIN_DEPTH_16: u32 = 0x02;
pub const CS35L35_SDIN_DEPTH_24: u32 = 0x03;

pub const CS35L35_SDOUT_DEPTH_8: u32 = 0x01;
pub const CS35L35_SDOUT_DEPTH_12: u32 = 0x02;
pub const CS35L35_SDOUT_DEPTH_16: u32 = 0x03;

pub const CS35L35_AUD_IN_LR_MASK: u32 = 0x80;
pub const CS35L35_AUD_IN_LR_SHIFT: u32 = 7;
pub const CS35L35_ADV_IN_LR_MASK: u32 = 0x80;
pub const CS35L35_ADV_IN_LR_SHIFT: u32 = 7;
pub const CS35L35_AUD_IN_LOC_MASK: u32 = 0x0F;
pub const CS35L35_AUD_IN_LOC_SHIFT: u32 = 0;
pub const CS35L35_ADV_IN_LOC_MASK: u32 = 0x0F;
pub const CS35L35_ADV_IN_LOC_SHIFT: u32 = 0;

pub const CS35L35_IMON_DEPTH_MASK: u32 = 0x03;
pub const CS35L35_IMON_DEPTH_SHIFT: u32 = 0;
pub const CS35L35_VMON_DEPTH_MASK: u32 = 0x0C;
pub const CS35L35_VMON_DEPTH_SHIFT: u32 = 2;
pub const CS35L35_VBSTMON_DEPTH_MASK: u32 = 0x03;
pub const CS35L35_VBSTMON_DEPTH_SHIFT: u32 = 0;
pub const CS35L35_VPMON_DEPTH_MASK: u32 = 0x0C;
pub const CS35L35_VPMON_DEPTH_SHIFT: u32 = 2;
pub const CS35L35_VPBRSTAT_DEPTH_MASK: u32 = 0x30;
pub const CS35L35_VPBRSTAT_DEPTH_SHIFT: u32 = 4;
pub const CS35L35_ZEROFILL_DEPTH_MASK: u32 = 0x03;
pub const CS35L35_ZEROFILL_DEPTH_SHIFT: u32 = 0x00;

pub const CS35L35_MON_TXLOC_MASK: u32 = 0x3F;
pub const CS35L35_MON_TXLOC_SHIFT: u32 = 0;
pub const CS35L35_MON_FRM_MASK: u32 = 0x80;
pub const CS35L35_MON_FRM_SHIFT: u32 = 7;

pub const CS35L35_IMON_SCALE_MASK: u32 = 0xF8;
pub const CS35L35_IMON_SCALE_SHIFT: u32 = 3;

pub const CS35L35_MS_MASK: u32 = 0x80;
pub const CS35L35_MS_SHIFT: u32 = 7;
pub const CS35L35_SPMODE_MASK: u32 = 0x40;
pub const CS35L35_SP_DRV_MASK: u32 = 0x10;
pub const CS35L35_SP_DRV_SHIFT: u32 = 4;
pub const CS35L35_CLK_CTL2_MASK: u32 = 0xFF;
pub const CS35L35_PDM_MODE_MASK: u32 = 0x40;
pub const CS35L35_PDM_MODE_SHIFT: u32 = 6;
pub const CS35L35_CLK_SOURCE_MASK: u32 = 0x03;
pub const CS35L35_CLK_SOURCE_SHIFT: u32 = 0;
pub const CS35L35_CLK_SOURCE_MCLK: u32 = 0;
pub const CS35L35_CLK_SOURCE_SCLK: u32 = 1;
pub const CS35L35_CLK_SOURCE_PDM: u32 = 2;

pub const CS35L35_SP_SCLKS_MASK: u32 = 0x0F;
pub const CS35L35_SP_SCLKS_SHIFT: u32 = 0x00;
pub const CS35L35_SP_SCLKS_16FS: u32 = 0x03;
pub const CS35L35_SP_SCLKS_32FS: u32 = 0x07;
pub const CS35L35_SP_SCLKS_48FS: u32 = 0x0B;
pub const CS35L35_SP_SCLKS_64FS: u32 = 0x0F;
pub const CS35L35_SP_RATE_MASK: u32 = 0xC0;
pub const CS35L35_SP_RATE_SHIFT: u32 = 6;

pub const CS35L35_PDN_BST_MASK: u32 = 0x06;
pub const CS35L35_PDN_BST_FETON_SHIFT: u32 = 1;
pub const CS35L35_PDN_BST_FETOFF_SHIFT: u32 = 2;
pub const CS35L35_PWR2_PDN_MASK: u32 = 0xE0;
pub const CS35L35_PWR3_PDN_MASK: u32 = 0x1E;
pub const CS35L35_PDN_ALL_MASK: u32 = 0x01;
pub const CS35L35_DISCHG_FILT_MASK: u32 = 0x02;
pub const CS35L35_DISCHG_FILT_SHIFT: u32 = 1;
pub const CS35L35_MCLK_DIS_MASK: u32 = 0x04;
pub const CS35L35_MCLK_DIS_SHIFT: u32 = 2;

pub const CS35L35_BST_CTL_MASK: u32 = 0x7F;
pub const CS35L35_BST_CTL_SHIFT: u32 = 0;
pub const CS35L35_BST_IPK_MASK: u32 = 0x1F;
pub const CS35L35_BST_IPK_SHIFT: u32 = 0;
pub const CS35L35_AMP_MUTE_MASK: u32 = 0x20;
pub const CS35L35_AMP_MUTE_SHIFT: u32 = 5;
pub const CS35L35_AMP_GAIN_ZC_MASK: u32 = 0x10;
pub const CS35L35_AMP_GAIN_ZC_SHIFT: u32 = 4;

pub const CS35L35_AMP_DIGSFT_MASK: u32 = 0x02;
pub const CS35L35_AMP_DIGSFT_SHIFT: u32 = 1;

/* CS35L35_SP_FMT_CTL3 */
pub const CS35L35_SP_I2S_DRV_MASK: u32 = 0x03;
pub const CS35L35_SP_I2S_DRV_SHIFT: u32 = 0;

/* Boost Converter Config */
pub const CS35L35_BST_CONV_COEFF_MASK: u32 = 0xFF;
pub const CS35L35_BST_CONV_SLOPE_MASK: u32 = 0xFF;
pub const CS35L35_BST_CONV_LBST_MASK: u32 = 0x03;
pub const CS35L35_BST_CONV_SWFREQ_MASK: u32 = 0xF0;

/* Class H Algorithm Control */
pub const CS35L35_CH_STEREO_MASK: u32 = 0x40;
pub const CS35L35_CH_STEREO_SHIFT: u32 = 6;
pub const CS35L35_CH_BST_OVR_MASK: u32 = 0x04;
pub const CS35L35_CH_BST_OVR_SHIFT: u32 = 2;
pub const CS35L35_CH_BST_LIM_MASK: u32 = 0x08;
pub const CS35L35_CH_BST_LIM_SHIFT: u32 = 3;
pub const CS35L35_CH_MEM_DEPTH_MASK: u32 = 0x01;
pub const CS35L35_CH_MEM_DEPTH_SHIFT: u32 = 0;
pub const CS35L35_CH_HDRM_CTL_MASK: u32 = 0x3F;
pub const CS35L35_CH_HDRM_CTL_SHIFT: u32 = 0;
pub const CS35L35_CH_REL_RATE_MASK: u32 = 0xFF;
pub const CS35L35_CH_REL_RATE_SHIFT: u32 = 0;
pub const CS35L35_CH_WKFET_DIS_MASK: u32 = 0x80;
pub const CS35L35_CH_WKFET_DIS_SHIFT: u32 = 7;
pub const CS35L35_CH_WKFET_DEL_MASK: u32 = 0x70;
pub const CS35L35_CH_WKFET_DEL_SHIFT: u32 = 4;
pub const CS35L35_CH_WKFET_THLD_MASK: u32 = 0x0F;
pub const CS35L35_CH_WKFET_THLD_SHIFT: u32 = 0;
pub const CS35L35_CH_VP_AUTO_MASK: u32 = 0x80;
pub const CS35L35_CH_VP_AUTO_SHIFT: u32 = 7;
pub const CS35L35_CH_VP_RATE_MASK: u32 = 0x60;
pub const CS35L35_CH_VP_RATE_SHIFT: u32 = 5;
pub const CS35L35_CH_VP_MAN_MASK: u32 = 0x1F;
pub const CS35L35_CH_VP_MAN_SHIFT: u32 = 0;

/* CS35L35_PROT_RELEASE_CTL */
pub const CS35L35_CAL_ERR_RLS: u32 = 0x80;
pub const CS35L35_SHORT_RLS: u32 = 0x04;
pub const CS35L35_OTW_RLS: u32 = 0x02;
pub const CS35L35_OTE_RLS: u32 = 0x01;

/* INT Mask Registers */
pub const CS35L35_INT1_CRIT_MASK: u32 = 0x38;
pub const CS35L35_INT2_CRIT_MASK: u32 = 0xEF;
pub const CS35L35_INT3_CRIT_MASK: u32 = 0xEE;
pub const CS35L35_INT4_CRIT_MASK: u32 = 0xFF;

/* PDN DONE Masks */
pub const CS35L35_M_PDN_DONE_SHIFT: u32 = 4;
pub const CS35L35_M_PDN_DONE_MASK: u32 = 0x10;

/* CS35L35_INT_1 */
pub const CS35L35_CAL_ERR: u32 = 0x80;
pub const CS35L35_OTP_ERR: u32 = 0x40;
pub const CS35L35_LRCLK_ERR: u32 = 0x20;
pub const CS35L35_SPCLK_ERR: u32 = 0x10;
pub const CS35L35_MCLK_ERR: u32 = 0x08;
pub const CS35L35_AMP_SHORT: u32 = 0x04;
pub const CS35L35_OTW: u32 = 0x02;
pub const CS35L35_OTE: u32 = 0x01;

/* CS35L35_INT_2 */
pub const CS35L35_PDN_DONE: u32 = 0x10;
pub const CS35L35_VPBR_ERR: u32 = 0x02;
pub const CS35L35_VPBR_CLR: u32 = 0x01;

/* CS35L35_INT_3 */
pub const CS35L35_BST_HIGH: u32 = 0x10;
pub const CS35L35_BST_HIGH_FLAG: u32 = 0x08;
pub const CS35L35_BST_IPK_FLAG: u32 = 0x04;
pub const CS35L35_LBST_SHORT: u32 = 0x01;

/* CS35L35_INT_4 */
pub const CS35L35_VMON_OVFL: u32 = 0x08;
pub const CS35L35_IMON_OVFL: u32 = 0x04;

pub const CS35L35_FORMATS: u32 = SNDRV_PCM_FMTBIT_U8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

extern "C" {
    pub type device;
    pub type regmap;
    pub type gpio_desc;
}

/* struct cs35l35_platform_data is supplied by another header. */
#[repr(C)]
pub struct cs35l35_platform_data {
    _private: [u8; 0],
}

/* struct regulator_bulk_data is supplied by another header. */
#[repr(C)]
pub struct regulator_bulk_data {
    _private: [u8; 0],
}

/* struct completion is supplied by another header. */
#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cs35l35_private {
    pub dev: *mut device,
    pub pdata: cs35l35_platform_data,
    pub regmap: *mut regmap,
    pub supplies: [regulator_bulk_data; 2],
    pub num_supplies: i32,
    pub sysclk: i32,
    pub sclk: i32,
    pub pdm_mode: bool,
    pub i2s_mode: bool,
    pub clock_consumer: bool,
    /* GPIO for /RST */
    pub reset_gpio: *mut gpio_desc,
    pub pdn_done: completion,
}

pub static cs35l35_supplies: [*const core::ffi::c_char; 2] = [
    b"VA\0".as_ptr() as *const core::ffi::c_char,
    b"VP\0".as_ptr() as *const core::ffi::c_char,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
