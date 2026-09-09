/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * MFD driver for twl4030 audio submodule
 *
 * Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
 *
 * Copyright:   (C) 2009 Nokia Corporation
 */

/* Codec registers */
pub const TWL4030_REG_CODEC_MODE: u8 = 0x01;
pub const TWL4030_REG_OPTION: u8 = 0x02;
pub const TWL4030_REG_UNKNOWN: u8 = 0x03;
pub const TWL4030_REG_MICBIAS_CTL: u8 = 0x04;
pub const TWL4030_REG_ANAMICL: u8 = 0x05;
pub const TWL4030_REG_ANAMICR: u8 = 0x06;
pub const TWL4030_REG_AVADC_CTL: u8 = 0x07;
pub const TWL4030_REG_ADCMICSEL: u8 = 0x08;
pub const TWL4030_REG_DIGMIXING: u8 = 0x09;
pub const TWL4030_REG_ATXL1PGA: u8 = 0x0A;
pub const TWL4030_REG_ATXR1PGA: u8 = 0x0B;
pub const TWL4030_REG_AVTXL2PGA: u8 = 0x0C;
pub const TWL4030_REG_AVTXR2PGA: u8 = 0x0D;
pub const TWL4030_REG_AUDIO_IF: u8 = 0x0E;
pub const TWL4030_REG_VOICE_IF: u8 = 0x0F;
pub const TWL4030_REG_ARXR1PGA: u8 = 0x10;
pub const TWL4030_REG_ARXL1PGA: u8 = 0x11;
pub const TWL4030_REG_ARXR2PGA: u8 = 0x12;
pub const TWL4030_REG_ARXL2PGA: u8 = 0x13;
pub const TWL4030_REG_VRXPGA: u8 = 0x14;
pub const TWL4030_REG_VSTPGA: u8 = 0x15;
pub const TWL4030_REG_VRX2ARXPGA: u8 = 0x16;
pub const TWL4030_REG_AVDAC_CTL: u8 = 0x17;
pub const TWL4030_REG_ARX2VTXPGA: u8 = 0x18;
pub const TWL4030_REG_ARXL1_APGA_CTL: u8 = 0x19;
pub const TWL4030_REG_ARXR1_APGA_CTL: u8 = 0x1A;
pub const TWL4030_REG_ARXL2_APGA_CTL: u8 = 0x1B;
pub const TWL4030_REG_ARXR2_APGA_CTL: u8 = 0x1C;
pub const TWL4030_REG_ATX2ARXPGA: u8 = 0x1D;
pub const TWL4030_REG_BT_IF: u8 = 0x1E;
pub const TWL4030_REG_BTPGA: u8 = 0x1F;
pub const TWL4030_REG_BTSTPGA: u8 = 0x20;
pub const TWL4030_REG_EAR_CTL: u8 = 0x21;
pub const TWL4030_REG_HS_SEL: u8 = 0x22;
pub const TWL4030_REG_HS_GAIN_SET: u8 = 0x23;
pub const TWL4030_REG_HS_POPN_SET: u8 = 0x24;
pub const TWL4030_REG_PREDL_CTL: u8 = 0x25;
pub const TWL4030_REG_PREDR_CTL: u8 = 0x26;
pub const TWL4030_REG_PRECKL_CTL: u8 = 0x27;
pub const TWL4030_REG_PRECKR_CTL: u8 = 0x28;
pub const TWL4030_REG_HFL_CTL: u8 = 0x29;
pub const TWL4030_REG_HFR_CTL: u8 = 0x2A;
pub const TWL4030_REG_ALC_CTL: u8 = 0x2B;
pub const TWL4030_REG_ALC_SET1: u8 = 0x2C;
pub const TWL4030_REG_ALC_SET2: u8 = 0x2D;
pub const TWL4030_REG_BOOST_CTL: u8 = 0x2E;
pub const TWL4030_REG_SOFTVOL_CTL: u8 = 0x2F;
pub const TWL4030_REG_DTMF_FREQSEL: u8 = 0x30;
pub const TWL4030_REG_DTMF_TONEXT1H: u8 = 0x31;
pub const TWL4030_REG_DTMF_TONEXT1L: u8 = 0x32;
pub const TWL4030_REG_DTMF_TONEXT2H: u8 = 0x33;
pub const TWL4030_REG_DTMF_TONEXT2L: u8 = 0x34;
pub const TWL4030_REG_DTMF_TONOFF: u8 = 0x35;
pub const TWL4030_REG_DTMF_WANONOFF: u8 = 0x36;
pub const TWL4030_REG_I2S_RX_SCRAMBLE_H: u8 = 0x37;
pub const TWL4030_REG_I2S_RX_SCRAMBLE_M: u8 = 0x38;
pub const TWL4030_REG_I2S_RX_SCRAMBLE_L: u8 = 0x39;
pub const TWL4030_REG_APLL_CTL: u8 = 0x3A;
pub const TWL4030_REG_DTMF_CTL: u8 = 0x3B;
pub const TWL4030_REG_DTMF_PGA_CTL2: u8 = 0x3C;
pub const TWL4030_REG_DTMF_PGA_CTL1: u8 = 0x3D;
pub const TWL4030_REG_MISC_SET_1: u8 = 0x3E;
pub const TWL4030_REG_PCMBTMUX: u8 = 0x3F;
pub const TWL4030_REG_RX_PATH_SEL: u8 = 0x43;
pub const TWL4030_REG_VDL_APGA_CTL: u8 = 0x44;
pub const TWL4030_REG_VIBRA_CTL: u8 = 0x45;
pub const TWL4030_REG_VIBRA_SET: u8 = 0x46;
pub const TWL4030_REG_VIBRA_PWM_SET: u8 = 0x47;
pub const TWL4030_REG_ANAMIC_GAIN: u8 = 0x48;
pub const TWL4030_REG_MISC_SET_2: u8 = 0x49;

/* Bitfield Definitions */
/* TWL4030_CODEC_MODE (0x01) Fields */
pub const TWL4030_APLL_RATE: u8 = 0xF0;
pub const TWL4030_APLL_RATE_8000: u8 = 0x00;
pub const TWL4030_APLL_RATE_11025: u8 = 0x10;
pub const TWL4030_APLL_RATE_12000: u8 = 0x20;
pub const TWL4030_APLL_RATE_16000: u8 = 0x40;
pub const TWL4030_APLL_RATE_22050: u8 = 0x50;
pub const TWL4030_APLL_RATE_24000: u8 = 0x60;
pub const TWL4030_APLL_RATE_32000: u8 = 0x80;
pub const TWL4030_APLL_RATE_44100: u8 = 0x90;
pub const TWL4030_APLL_RATE_48000: u8 = 0xA0;
pub const TWL4030_APLL_RATE_96000: u8 = 0xE0;
pub const TWL4030_SEL_16K: u8 = 0x08;
pub const TWL4030_CODECPDZ: u8 = 0x02;
pub const TWL4030_OPT_MODE: u8 = 0x01;
pub const TWL4030_OPTION_1: u8 = 1 << 0;
pub const TWL4030_OPTION_2: u8 = 0 << 0;

/* TWL4030_OPTION (0x02) Fields */
pub const TWL4030_ATXL1_EN: u8 = 1 << 0;
pub const TWL4030_ATXR1_EN: u8 = 1 << 1;
pub const TWL4030_ATXL2_VTXL_EN: u8 = 1 << 2;
pub const TWL4030_ATXR2_VTXR_EN: u8 = 1 << 3;
pub const TWL4030_ARXL1_VRX_EN: u8 = 1 << 4;
pub const TWL4030_ARXR1_EN: u8 = 1 << 5;
pub const TWL4030_ARXL2_EN: u8 = 1 << 6;
pub const TWL4030_ARXR2_EN: u8 = 1 << 7;

/* Register bitfields */
pub const TWL4030_MICBIAS2_CTL: u8 = 0x40;
pub const TWL4030_MICBIAS1_CTL: u8 = 0x20;
pub const TWL4030_HSMICBIAS_EN: u8 = 0x04;
pub const TWL4030_MICBIAS2_EN: u8 = 0x02;
pub const TWL4030_MICBIAS1_EN: u8 = 0x01;
pub const TWL4030_CNCL_OFFSET_START: u8 = 0x80;
pub const TWL4030_OFFSET_CNCL_SEL: u8 = 0x60;
pub const TWL4030_OFFSET_CNCL_SEL_ARX1: u8 = 0x00;
pub const TWL4030_OFFSET_CNCL_SEL_ARX2: u8 = 0x20;
pub const TWL4030_OFFSET_CNCL_SEL_VRX: u8 = 0x40;
pub const TWL4030_OFFSET_CNCL_SEL_ALL: u8 = 0x60;
pub const TWL4030_MICAMPL_EN: u8 = 0x10;
pub const TWL4030_CKMIC_EN: u8 = 0x08;
pub const TWL4030_AUXL_EN: u8 = 0x04;
pub const TWL4030_HSMIC_EN: u8 = 0x02;
pub const TWL4030_MAINMIC_EN: u8 = 0x01;
pub const TWL4030_MICAMPR_EN: u8 = 0x10;
pub const TWL4030_AUXR_EN: u8 = 0x04;
pub const TWL4030_SUBMIC_EN: u8 = 0x01;
pub const TWL4030_ADCL_EN: u8 = 0x08;
pub const TWL4030_AVADC_CLK_PRIORITY: u8 = 0x04;
pub const TWL4030_ADCR_EN: u8 = 0x02;
pub const TWL4030_DIGMIC1_EN: u8 = 0x08;
pub const TWL4030_TX2IN_SEL: u8 = 0x04;
pub const TWL4030_DIGMIC0_EN: u8 = 0x02;
pub const TWL4030_TX1IN_SEL: u8 = 0x01;
pub const TWL4030_AIF_SLAVE_EN: u8 = 0x80;
pub const TWL4030_DATA_WIDTH: u8 = 0x60;
pub const TWL4030_DATA_WIDTH_16S_16W: u8 = 0x00;
pub const TWL4030_DATA_WIDTH_32S_16W: u8 = 0x40;
pub const TWL4030_DATA_WIDTH_32S_24W: u8 = 0x60;
pub const TWL4030_AIF_FORMAT: u8 = 0x18;
pub const TWL4030_AIF_FORMAT_CODEC: u8 = 0x00;
pub const TWL4030_AIF_FORMAT_LEFT: u8 = 0x08;
pub const TWL4030_AIF_FORMAT_RIGHT: u8 = 0x10;
pub const TWL4030_AIF_FORMAT_TDM: u8 = 0x18;
pub const TWL4030_AIF_TRI_EN: u8 = 0x04;
pub const TWL4030_CLK256FS_EN: u8 = 0x02;
pub const TWL4030_AIF_EN: u8 = 0x01;
pub const TWL4030_VIF_SLAVE_EN: u8 = 0x80;
pub const TWL4030_VIF_DIN_EN: u8 = 0x40;
pub const TWL4030_VIF_DOUT_EN: u8 = 0x20;
pub const TWL4030_VIF_SWAP: u8 = 0x10;
pub const TWL4030_VIF_FORMAT: u8 = 0x08;
pub const TWL4030_VIF_TRI_EN: u8 = 0x04;
pub const TWL4030_VIF_SUB_EN: u8 = 0x02;
pub const TWL4030_VIF_EN: u8 = 0x01;
pub const TWL4030_EAR_GAIN: u8 = 0x30;
pub const TWL4030_HSR_GAIN: u8 = 0x0C;
pub const TWL4030_HSR_GAIN_PWR_DOWN: u8 = 0x00;
pub const TWL4030_HSR_GAIN_PLUS_6DB: u8 = 0x04;
pub const TWL4030_HSR_GAIN_0DB: u8 = 0x08;
pub const TWL4030_HSR_GAIN_MINUS_6DB: u8 = 0x0C;
pub const TWL4030_HSL_GAIN: u8 = 0x03;
pub const TWL4030_HSL_GAIN_PWR_DOWN: u8 = 0x00;
pub const TWL4030_HSL_GAIN_PLUS_6DB: u8 = 0x01;
pub const TWL4030_HSL_GAIN_0DB: u8 = 0x02;
pub const TWL4030_HSL_GAIN_MINUS_6DB: u8 = 0x03;
pub const TWL4030_VMID_EN: u8 = 0x40;
pub const TWL4030_EXTMUTE: u8 = 0x20;
pub const TWL4030_RAMP_DELAY: u8 = 0x1C;
pub const TWL4030_RAMP_DELAY_20MS: u8 = 0x00;
pub const TWL4030_RAMP_DELAY_40MS: u8 = 0x04;
pub const TWL4030_RAMP_DELAY_81MS: u8 = 0x08;
pub const TWL4030_RAMP_DELAY_161MS: u8 = 0x0C;
pub const TWL4030_RAMP_DELAY_323MS: u8 = 0x10;
pub const TWL4030_RAMP_DELAY_645MS: u8 = 0x14;
pub const TWL4030_RAMP_DELAY_1291MS: u8 = 0x18;
pub const TWL4030_RAMP_DELAY_2581MS: u8 = 0x1C;
pub const TWL4030_RAMP_EN: u8 = 0x02;
pub const TWL4030_PREDL_GAIN: u8 = 0x30;
pub const TWL4030_PREDR_GAIN: u8 = 0x30;
pub const TWL4030_PRECKL_GAIN: u8 = 0x30;
pub const TWL4030_PRECKR_GAIN: u8 = 0x30;
pub const TWL4030_HF_CTL_HB_EN: u8 = 0x04;
pub const TWL4030_HF_CTL_LOOP_EN: u8 = 0x08;
pub const TWL4030_HF_CTL_RAMP_EN: u8 = 0x10;
pub const TWL4030_HF_CTL_REF_EN: u8 = 0x20;
pub const TWL4030_APLL_EN: u8 = 0x10;
pub const TWL4030_APLL_INFREQ: u8 = 0x0F;
pub const TWL4030_APLL_INFREQ_19200KHZ: u8 = 0x05;
pub const TWL4030_APLL_INFREQ_26000KHZ: u8 = 0x06;
pub const TWL4030_APLL_INFREQ_38400KHZ: u8 = 0x0F;
pub const TWL4030_CLK64_EN: u8 = 0x80;
pub const TWL4030_SCRAMBLE_EN: u8 = 0x40;
pub const TWL4030_FMLOOP_EN: u8 = 0x20;
pub const TWL4030_SMOOTH_ANAVOL_EN: u8 = 0x02;
pub const TWL4030_DIGMIC_LR_SWAP_EN: u8 = 0x01;
pub const TWL4030_VIBRA_EN: u8 = 0x01;
pub const TWL4030_VIBRA_DIR: u8 = 0x02;
pub const TWL4030_VIBRA_AUDIO_SEL_L1: u8 = 0x00 << 2;
pub const TWL4030_VIBRA_AUDIO_SEL_R1: u8 = 0x01 << 2;
pub const TWL4030_VIBRA_AUDIO_SEL_L2: u8 = 0x02 << 2;
pub const TWL4030_VIBRA_AUDIO_SEL_R2: u8 = 0x03 << 2;
pub const TWL4030_VIBRA_SEL: u8 = 0x10;
pub const TWL4030_VIBRA_DIR_SEL: u8 = 0x20;

/* TWL4030 codec resource IDs */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum twl4030_audio_res {
    TWL4030_AUDIO_RES_POWER = 0,
    TWL4030_AUDIO_RES_APLL,
    TWL4030_AUDIO_RES_MAX,
}

unsafe extern "C" {
    pub fn twl4030_audio_disable_resource(id: twl4030_audio_res) -> ::core::ffi::c_int;
    pub fn twl4030_audio_enable_resource(id: twl4030_audio_res) -> ::core::ffi::c_int;
    pub fn twl4030_audio_get_mclk() -> ::core::ffi::c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
