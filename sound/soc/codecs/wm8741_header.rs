/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * wm8741.h  --  WM8423 ASoC driver
 *
 * Copyright 2010 Wolfson Microelectronics, plc
 *
 * Author: Ian Lartey <ian@opensource.wolfsonmicro.com>
 *
 * Based on wm8753.h
 */

/*
 * Register values.
 */
pub const WM8741_DACLLSB_ATTENUATION: u32 = 0x00;
pub const WM8741_DACLMSB_ATTENUATION: u32 = 0x01;
pub const WM8741_DACRLSB_ATTENUATION: u32 = 0x02;
pub const WM8741_DACRMSB_ATTENUATION: u32 = 0x03;
pub const WM8741_VOLUME_CONTROL: u32 = 0x04;
pub const WM8741_FORMAT_CONTROL: u32 = 0x05;
pub const WM8741_FILTER_CONTROL: u32 = 0x06;
pub const WM8741_MODE_CONTROL_1: u32 = 0x07;
pub const WM8741_MODE_CONTROL_2: u32 = 0x08;
pub const WM8741_RESET: u32 = 0x09;
pub const WM8741_ADDITIONAL_CONTROL_1: u32 = 0x20;

pub const WM8741_REGISTER_COUNT: u32 = 11;
pub const WM8741_MAX_REGISTER: u32 = 0x20;

/*
 * Field Definitions.
 */

/*
 * R0 (0x00) - DACLLSB_ATTENUATION
 */
pub const WM8741_UPDATELL: u32 = 0x0020; /* UPDATELL */
pub const WM8741_UPDATELL_MASK: u32 = 0x0020; /* UPDATELL */
pub const WM8741_UPDATELL_SHIFT: u32 = 5; /* UPDATELL */
pub const WM8741_UPDATELL_WIDTH: u32 = 1; /* UPDATELL */
pub const WM8741_LAT_4_0_MASK: u32 = 0x001F; /* LAT[4:0] - [4:0] */
pub const WM8741_LAT_4_0_SHIFT: u32 = 0; /* LAT[4:0] - [4:0] */
pub const WM8741_LAT_4_0_WIDTH: u32 = 5; /* LAT[4:0] - [4:0] */

/*
 * R1 (0x01) - DACLMSB_ATTENUATION
 */
pub const WM8741_UPDATELM: u32 = 0x0020; /* UPDATELM */
pub const WM8741_UPDATELM_MASK: u32 = 0x0020; /* UPDATELM */
pub const WM8741_UPDATELM_SHIFT: u32 = 5; /* UPDATELM */
pub const WM8741_UPDATELM_WIDTH: u32 = 1; /* UPDATELM */
pub const WM8741_LAT_9_5_0_MASK: u32 = 0x001F; /* LAT[9:5] - [4:0] */
pub const WM8741_LAT_9_5_0_SHIFT: u32 = 0; /* LAT[9:5] - [4:0] */
pub const WM8741_LAT_9_5_0_WIDTH: u32 = 5; /* LAT[9:5] - [4:0] */

/*
 * R2 (0x02) - DACRLSB_ATTENUATION
 */
pub const WM8741_UPDATERL: u32 = 0x0020; /* UPDATERL */
pub const WM8741_UPDATERL_MASK: u32 = 0x0020; /* UPDATERL */
pub const WM8741_UPDATERL_SHIFT: u32 = 5; /* UPDATERL */
pub const WM8741_UPDATERL_WIDTH: u32 = 1; /* UPDATERL */
pub const WM8741_RAT_4_0_MASK: u32 = 0x001F; /* RAT[4:0] - [4:0] */
pub const WM8741_RAT_4_0_SHIFT: u32 = 0; /* RAT[4:0] - [4:0] */
pub const WM8741_RAT_4_0_WIDTH: u32 = 5; /* RAT[4:0] - [4:0] */

/*
 * R3 (0x03) - DACRMSB_ATTENUATION
 */
pub const WM8741_UPDATERM: u32 = 0x0020; /* UPDATERM */
pub const WM8741_UPDATERM_MASK: u32 = 0x0020; /* UPDATERM */
pub const WM8741_UPDATERM_SHIFT: u32 = 5; /* UPDATERM */
pub const WM8741_UPDATERM_WIDTH: u32 = 1; /* UPDATERM */
pub const WM8741_RAT_9_5_0_MASK: u32 = 0x001F; /* RAT[9:5] - [4:0] */
pub const WM8741_RAT_9_5_0_SHIFT: u32 = 0; /* RAT[9:5] - [4:0] */
pub const WM8741_RAT_9_5_0_WIDTH: u32 = 5; /* RAT[9:5] - [4:0] */

/*
 * R4 (0x04) - VOLUME_CONTROL
 */
pub const WM8741_AMUTE: u32 = 0x0080; /* AMUTE */
pub const WM8741_AMUTE_MASK: u32 = 0x0080; /* AMUTE */
pub const WM8741_AMUTE_SHIFT: u32 = 7; /* AMUTE */
pub const WM8741_AMUTE_WIDTH: u32 = 1; /* AMUTE */
pub const WM8741_ZFLAG_MASK: u32 = 0x0060; /* ZFLAG - [6:5] */
pub const WM8741_ZFLAG_SHIFT: u32 = 5; /* ZFLAG - [6:5] */
pub const WM8741_ZFLAG_WIDTH: u32 = 2; /* ZFLAG - [6:5] */
pub const WM8741_IZD: u32 = 0x0010; /* IZD */
pub const WM8741_IZD_MASK: u32 = 0x0010; /* IZD */
pub const WM8741_IZD_SHIFT: u32 = 4; /* IZD */
pub const WM8741_IZD_WIDTH: u32 = 1; /* IZD */
pub const WM8741_SOFT: u32 = 0x0008; /* SOFT MUTE */
pub const WM8741_SOFT_MASK: u32 = 0x0008; /* SOFT MUTE */
pub const WM8741_SOFT_SHIFT: u32 = 3; /* SOFT MUTE */
pub const WM8741_SOFT_WIDTH: u32 = 1; /* SOFT MUTE */
pub const WM8741_ATC: u32 = 0x0004; /* ATC */
pub const WM8741_ATC_MASK: u32 = 0x0004; /* ATC */
pub const WM8741_ATC_SHIFT: u32 = 2; /* ATC */
pub const WM8741_ATC_WIDTH: u32 = 1; /* ATC */
pub const WM8741_ATT2DB: u32 = 0x0002; /* ATT2DB */
pub const WM8741_ATT2DB_MASK: u32 = 0x0002; /* ATT2DB */
pub const WM8741_ATT2DB_SHIFT: u32 = 1; /* ATT2DB */
pub const WM8741_ATT2DB_WIDTH: u32 = 1; /* ATT2DB */
pub const WM8741_VOL_RAMP: u32 = 0x0001; /* VOL_RAMP */
pub const WM8741_VOL_RAMP_MASK: u32 = 0x0001; /* VOL_RAMP */
pub const WM8741_VOL_RAMP_SHIFT: u32 = 0; /* VOL_RAMP */
pub const WM8741_VOL_RAMP_WIDTH: u32 = 1; /* VOL_RAMP */

/*
 * R5 (0x05) - FORMAT_CONTROL
 */
pub const WM8741_PWDN: u32 = 0x0080; /* PWDN */
pub const WM8741_PWDN_MASK: u32 = 0x0080; /* PWDN */
pub const WM8741_PWDN_SHIFT: u32 = 7; /* PWDN */
pub const WM8741_PWDN_WIDTH: u32 = 1; /* PWDN */
pub const WM8741_REV: u32 = 0x0040; /* REV */
pub const WM8741_REV_MASK: u32 = 0x0040; /* REV */
pub const WM8741_REV_SHIFT: u32 = 6; /* REV */
pub const WM8741_REV_WIDTH: u32 = 1; /* REV */
pub const WM8741_BCP: u32 = 0x0020; /* BCP */
pub const WM8741_BCP_MASK: u32 = 0x0020; /* BCP */
pub const WM8741_BCP_SHIFT: u32 = 5; /* BCP */
pub const WM8741_BCP_WIDTH: u32 = 1; /* BCP */
pub const WM8741_LRP: u32 = 0x0010; /* LRP */
pub const WM8741_LRP_MASK: u32 = 0x0010; /* LRP */
pub const WM8741_LRP_SHIFT: u32 = 4; /* LRP */
pub const WM8741_LRP_WIDTH: u32 = 1; /* LRP */
pub const WM8741_FMT_MASK: u32 = 0x000C; /* FMT - [3:2] */
pub const WM8741_FMT_SHIFT: u32 = 2; /* FMT - [3:2] */
pub const WM8741_FMT_WIDTH: u32 = 2; /* FMT - [3:2] */
pub const WM8741_IWL_MASK: u32 = 0x0003; /* IWL - [1:0] */
pub const WM8741_IWL_SHIFT: u32 = 0; /* IWL - [1:0] */
pub const WM8741_IWL_WIDTH: u32 = 2; /* IWL - [1:0] */

/*
 * R6 (0x06) - FILTER_CONTROL
 */
pub const WM8741_ZFLAG_HI: u32 = 0x0080; /* ZFLAG_HI */
pub const WM8741_ZFLAG_HI_MASK: u32 = 0x0080; /* ZFLAG_HI */
pub const WM8741_ZFLAG_HI_SHIFT: u32 = 7; /* ZFLAG_HI */
pub const WM8741_ZFLAG_HI_WIDTH: u32 = 1; /* ZFLAG_HI */
pub const WM8741_DEEMPH_MASK: u32 = 0x0060; /* DEEMPH - [6:5] */
pub const WM8741_DEEMPH_SHIFT: u32 = 5; /* DEEMPH - [6:5] */
pub const WM8741_DEEMPH_WIDTH: u32 = 2; /* DEEMPH - [6:5] */
pub const WM8741_DSDFILT_MASK: u32 = 0x0018; /* DSDFILT - [4:3] */
pub const WM8741_DSDFILT_SHIFT: u32 = 3; /* DSDFILT - [4:3] */
pub const WM8741_DSDFILT_WIDTH: u32 = 2; /* DSDFILT - [4:3] */
pub const WM8741_FIRSEL_MASK: u32 = 0x0007; /* FIRSEL - [2:0] */
pub const WM8741_FIRSEL_SHIFT: u32 = 0; /* FIRSEL - [2:0] */
pub const WM8741_FIRSEL_WIDTH: u32 = 3; /* FIRSEL - [2:0] */

/*
 * R7 (0x07) - MODE_CONTROL_1
 */
pub const WM8741_MODE8X: u32 = 0x0080; /* MODE8X */
pub const WM8741_MODE8X_MASK: u32 = 0x0080; /* MODE8X */
pub const WM8741_MODE8X_SHIFT: u32 = 7; /* MODE8X */
pub const WM8741_MODE8X_WIDTH: u32 = 1; /* MODE8X */
pub const WM8741_OSR_MASK: u32 = 0x0060; /* OSR - [6:5] */
pub const WM8741_OSR_SHIFT: u32 = 5; /* OSR - [6:5] */
pub const WM8741_OSR_WIDTH: u32 = 2; /* OSR - [6:5] */
pub const WM8741_SR_MASK: u32 = 0x001C; /* SR - [4:2] */
pub const WM8741_SR_SHIFT: u32 = 2; /* SR - [4:2] */
pub const WM8741_SR_WIDTH: u32 = 3; /* SR - [4:2] */
pub const WM8741_MODESEL_MASK: u32 = 0x0003; /* MODESEL - [1:0] */
pub const WM8741_MODESEL_SHIFT: u32 = 0; /* MODESEL - [1:0] */
pub const WM8741_MODESEL_WIDTH: u32 = 2; /* MODESEL - [1:0] */

/*
 * R8 (0x08) - MODE_CONTROL_2
 */
pub const WM8741_DSD_GAIN: u32 = 0x0040; /* DSD_GAIN */
pub const WM8741_DSD_GAIN_MASK: u32 = 0x0040; /* DSD_GAIN */
pub const WM8741_DSD_GAIN_SHIFT: u32 = 6; /* DSD_GAIN */
pub const WM8741_DSD_GAIN_WIDTH: u32 = 1; /* DSD_GAIN */
pub const WM8741_SDOUT: u32 = 0x0020; /* SDOUT */
pub const WM8741_SDOUT_MASK: u32 = 0x0020; /* SDOUT */
pub const WM8741_SDOUT_SHIFT: u32 = 5; /* SDOUT */
pub const WM8741_SDOUT_WIDTH: u32 = 1; /* SDOUT */
pub const WM8741_DOUT: u32 = 0x0010; /* DOUT */
pub const WM8741_DOUT_MASK: u32 = 0x0010; /* DOUT */
pub const WM8741_DOUT_SHIFT: u32 = 4; /* DOUT */
pub const WM8741_DOUT_WIDTH: u32 = 1; /* DOUT */
pub const WM8741_DIFF_MASK: u32 = 0x000C; /* DIFF - [3:2] */
pub const WM8741_DIFF_SHIFT: u32 = 2; /* DIFF - [3:2] */
pub const WM8741_DIFF_WIDTH: u32 = 2; /* DIFF - [3:2] */
pub const WM8741_DITHER_MASK: u32 = 0x0003; /* DITHER - [1:0] */
pub const WM8741_DITHER_SHIFT: u32 = 0; /* DITHER - [1:0] */
pub const WM8741_DITHER_WIDTH: u32 = 2; /* DITHER - [1:0] */

/* DIFF field values */
pub const WM8741_DIFF_MODE_STEREO: u32 = 0; /* stereo normal */
pub const WM8741_DIFF_MODE_STEREO_REVERSED: u32 = 2; /* stereo reversed */
pub const WM8741_DIFF_MODE_MONO_LEFT: u32 = 1; /* mono left */
pub const WM8741_DIFF_MODE_MONO_RIGHT: u32 = 3; /* mono right */

/*
 * R32 (0x20) - ADDITONAL_CONTROL_1
 */
pub const WM8741_DSD_LEVEL: u32 = 0x0002; /* DSD_LEVEL */
pub const WM8741_DSD_LEVEL_MASK: u32 = 0x0002; /* DSD_LEVEL */
pub const WM8741_DSD_LEVEL_SHIFT: u32 = 1; /* DSD_LEVEL */
pub const WM8741_DSD_LEVEL_WIDTH: u32 = 1; /* DSD_LEVEL */
pub const WM8741_DSD_NO_NOTCH: u32 = 0x0001; /* DSD_NO_NOTCH */
pub const WM8741_DSD_NO_NOTCH_MASK: u32 = 0x0001; /* DSD_NO_NOTCH */
pub const WM8741_DSD_NO_NOTCH_SHIFT: u32 = 0; /* DSD_NO_NOTCH */
pub const WM8741_DSD_NO_NOTCH_WIDTH: u32 = 1; /* DSD_NO_NOTCH */

pub const WM8741_SYSCLK: u32 = 0;

#[repr(C)]
pub struct wm8741_platform_data {
    pub diff_mode: u32, /* Differential Output Mode */
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
