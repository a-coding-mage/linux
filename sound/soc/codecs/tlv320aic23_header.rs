/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ALSA SoC TLV320AIC23 codec driver
 *
 * Author:      Arun KS, <arunks@mistralsolutions.com>
 * Copyright:   (C) 2008 Mistral Solutions Pvt Ltd
 */

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static tlv320aic23_regmap: regmap_config;
    pub fn tlv320aic23_probe(dev: *mut device, regmap: *mut regmap) -> ::std::os::raw::c_int;
}

/* Codec TLV320AIC23 */
pub const TLV320AIC23_LINVOL: u32 = 0x00;
pub const TLV320AIC23_RINVOL: u32 = 0x01;
pub const TLV320AIC23_LCHNVOL: u32 = 0x02;
pub const TLV320AIC23_RCHNVOL: u32 = 0x03;
pub const TLV320AIC23_ANLG: u32 = 0x04;
pub const TLV320AIC23_DIGT: u32 = 0x05;
pub const TLV320AIC23_PWR: u32 = 0x06;
pub const TLV320AIC23_DIGT_FMT: u32 = 0x07;
pub const TLV320AIC23_SRATE: u32 = 0x08;
pub const TLV320AIC23_ACTIVE: u32 = 0x09;
pub const TLV320AIC23_RESET: u32 = 0x0F;

/* Left (right) line input volume control register */
pub const TLV320AIC23_LRS_ENABLED: u32 = 0x0100;
pub const TLV320AIC23_LIM_MUTED: u32 = 0x0080;
pub const TLV320AIC23_LIV_DEFAULT: u32 = 0x0017;
pub const TLV320AIC23_LIV_MAX: u32 = 0x001f;
pub const TLV320AIC23_LIV_MIN: u32 = 0x0000;

/* Left (right) channel headphone volume control register */
pub const TLV320AIC23_LZC_ON: u32 = 0x0080;
pub const TLV320AIC23_LHV_DEFAULT: u32 = 0x0079;
pub const TLV320AIC23_LHV_MAX: u32 = 0x007f;
pub const TLV320AIC23_LHV_MIN: u32 = 0x0000;

/* Analog audio path control register */
pub const fn TLV320AIC23_STA_REG(x: u32) -> u32 {
    x << 6
}
pub const TLV320AIC23_STE_ENABLED: u32 = 0x0020;
pub const TLV320AIC23_DAC_SELECTED: u32 = 0x0010;
pub const TLV320AIC23_BYPASS_ON: u32 = 0x0008;
pub const TLV320AIC23_INSEL_MIC: u32 = 0x0004;
pub const TLV320AIC23_MICM_MUTED: u32 = 0x0002;
pub const TLV320AIC23_MICB_20DB: u32 = 0x0001;

/* Digital audio path control register */
pub const TLV320AIC23_DACM_MUTE: u32 = 0x0008;
pub const TLV320AIC23_DEEMP_32K: u32 = 0x0002;
pub const TLV320AIC23_DEEMP_44K: u32 = 0x0004;
pub const TLV320AIC23_DEEMP_48K: u32 = 0x0006;
pub const TLV320AIC23_ADCHP_ON: u32 = 0x0001;

/* Power control down register */
pub const TLV320AIC23_DEVICE_PWR_OFF: u32 = 0x0080;
pub const TLV320AIC23_CLK_OFF: u32 = 0x0040;
pub const TLV320AIC23_OSC_OFF: u32 = 0x0020;
pub const TLV320AIC23_OUT_OFF: u32 = 0x0010;
pub const TLV320AIC23_DAC_OFF: u32 = 0x0008;
pub const TLV320AIC23_ADC_OFF: u32 = 0x0004;
pub const TLV320AIC23_MIC_OFF: u32 = 0x0002;
pub const TLV320AIC23_LINE_OFF: u32 = 0x0001;

/* Digital audio interface register */
pub const TLV320AIC23_MS_MASTER: u32 = 0x0040;
pub const TLV320AIC23_LRSWAP_ON: u32 = 0x0020;
pub const TLV320AIC23_LRP_ON: u32 = 0x0010;
pub const TLV320AIC23_IWL_16: u32 = 0x0000;
pub const TLV320AIC23_IWL_20: u32 = 0x0004;
pub const TLV320AIC23_IWL_24: u32 = 0x0008;
pub const TLV320AIC23_IWL_32: u32 = 0x000C;
pub const TLV320AIC23_FOR_I2S: u32 = 0x0002;
pub const TLV320AIC23_FOR_DSP: u32 = 0x0003;
pub const TLV320AIC23_FOR_LJUST: u32 = 0x0001;

/* Sample rate control register */
pub const TLV320AIC23_CLKOUT_HALF: u32 = 0x0080;
pub const TLV320AIC23_CLKIN_HALF: u32 = 0x0040;
pub const TLV320AIC23_BOSR_384fs: u32 = 0x0002; /* BOSR_272fs in USB mode */
pub const TLV320AIC23_USB_CLK_ON: u32 = 0x0001;
pub const TLV320AIC23_SR_MASK: u32 = 0xf;
pub const TLV320AIC23_CLKOUT_SHIFT: u32 = 7;
pub const TLV320AIC23_CLKIN_SHIFT: u32 = 6;
pub const TLV320AIC23_SR_SHIFT: u32 = 2;
pub const TLV320AIC23_BOSR_SHIFT: u32 = 1;

/* Digital interface register */
pub const TLV320AIC23_ACT_ON: u32 = 0x0001;

/*
 * AUDIO related MACROS
 */

pub const TLV320AIC23_DEFAULT_OUT_VOL: u32 = 0x70;
pub const TLV320AIC23_DEFAULT_IN_VOLUME: u32 = 0x10;

pub const TLV320AIC23_OUT_VOL_MIN: u32 = TLV320AIC23_LHV_MIN;
pub const TLV320AIC23_OUT_VOL_MAX: u32 = TLV320AIC23_LHV_MAX;
pub const TLV320AIC23_OUT_VO_RANGE: u32 = TLV320AIC23_OUT_VOL_MAX - TLV320AIC23_OUT_VOL_MIN;
pub const TLV320AIC23_OUT_VOL_MASK: u32 = TLV320AIC23_OUT_VOL_MAX;

pub const TLV320AIC23_IN_VOL_MIN: u32 = TLV320AIC23_LIV_MIN;
pub const TLV320AIC23_IN_VOL_MAX: u32 = TLV320AIC23_LIV_MAX;
pub const TLV320AIC23_IN_VOL_RANGE: u32 = TLV320AIC23_IN_VOL_MAX - TLV320AIC23_IN_VOL_MIN;
pub const TLV320AIC23_IN_VOL_MASK: u32 = TLV320AIC23_IN_VOL_MAX;

pub const TLV320AIC23_SIDETONE_MASK: u32 = 0x1c0;
pub const TLV320AIC23_SIDETONE_0: u32 = 0x100;
pub const TLV320AIC23_SIDETONE_6: u32 = 0x000;
pub const TLV320AIC23_SIDETONE_9: u32 = 0x040;
pub const TLV320AIC23_SIDETONE_12: u32 = 0x080;
pub const TLV320AIC23_SIDETONE_18: u32 = 0x0c0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
