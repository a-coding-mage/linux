/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ALSA SoC TLV320AIC3X codec driver
 *
 * Author:      Vladimir Barinov, <vbarinov@embeddedalley.com>
 * Copyright:   (C) 2007 MontaVista Software, Inc., <source@mvista.com>
 */

// C dependency declarations originally supplied by included kernel headers.
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

pub type kernel_ulong_t = core::ffi::c_ulong;

unsafe extern "C" {
    pub static aic3x_regmap: regmap_config;
    pub fn aic3x_probe(
        dev: *mut device,
        regmap: *mut regmap,
        driver_data: kernel_ulong_t,
    ) -> core::ffi::c_int;
    pub fn aic3x_remove(dev: *mut device);
}

pub const AIC3X_MODEL_3X: core::ffi::c_int = 0;
pub const AIC3X_MODEL_33: core::ffi::c_int = 1;
pub const AIC3X_MODEL_3007: core::ffi::c_int = 2;
pub const AIC3X_MODEL_3104: core::ffi::c_int = 3;
pub const AIC3X_MODEL_3106: core::ffi::c_int = 4;

/* AIC3X register space */
pub const AIC3X_CACHEREGNUM: core::ffi::c_int = 110;

/* Page select register */
pub const AIC3X_PAGE_SELECT: core::ffi::c_int = 0;
/* Software reset register */
pub const AIC3X_RESET: core::ffi::c_int = 1;
/* Codec Sample rate select register */
pub const AIC3X_SAMPLE_RATE_SEL_REG: core::ffi::c_int = 2;
/* PLL progrramming register A */
pub const AIC3X_PLL_PROGA_REG: core::ffi::c_int = 3;
/* PLL progrramming register B */
pub const AIC3X_PLL_PROGB_REG: core::ffi::c_int = 4;
/* PLL progrramming register C */
pub const AIC3X_PLL_PROGC_REG: core::ffi::c_int = 5;
/* PLL progrramming register D */
pub const AIC3X_PLL_PROGD_REG: core::ffi::c_int = 6;
/* Codec datapath setup register */
pub const AIC3X_CODEC_DATAPATH_REG: core::ffi::c_int = 7;
/* Audio serial data interface control register A */
pub const AIC3X_ASD_INTF_CTRLA: core::ffi::c_int = 8;
/* Audio serial data interface control register B */
pub const AIC3X_ASD_INTF_CTRLB: core::ffi::c_int = 9;
/* Audio serial data interface control register C */
pub const AIC3X_ASD_INTF_CTRLC: core::ffi::c_int = 10;
/* Audio overflow status and PLL R value programming register */
pub const AIC3X_OVRF_STATUS_AND_PLLR_REG: core::ffi::c_int = 11;
/* Audio codec digital filter control register */
pub const AIC3X_CODEC_DFILT_CTRL: core::ffi::c_int = 12;
/* Headset/button press detection register */
pub const AIC3X_HEADSET_DETECT_CTRL_A: core::ffi::c_int = 13;
pub const AIC3X_HEADSET_DETECT_CTRL_B: core::ffi::c_int = 14;
/* ADC PGA Gain control registers */
pub const LADC_VOL: core::ffi::c_int = 15;
pub const RADC_VOL: core::ffi::c_int = 16;
/* MIC3 control registers */
pub const MIC3LR_2_LADC_CTRL: core::ffi::c_int = 17;
pub const MIC3LR_2_RADC_CTRL: core::ffi::c_int = 18;
/* Line1 Input control registers */
pub const LINE1L_2_LADC_CTRL: core::ffi::c_int = 19;
pub const LINE1R_2_LADC_CTRL: core::ffi::c_int = 21;
pub const LINE1R_2_RADC_CTRL: core::ffi::c_int = 22;
pub const LINE1L_2_RADC_CTRL: core::ffi::c_int = 24;
/* Line2 Input control registers */
pub const LINE2L_2_LADC_CTRL: core::ffi::c_int = 20;
pub const LINE2R_2_RADC_CTRL: core::ffi::c_int = 23;
/* MICBIAS Control Register */
pub const MICBIAS_CTRL: core::ffi::c_int = 25;

/* AGC Control Registers A, B, C */
pub const LAGC_CTRL_A: core::ffi::c_int = 26;
pub const LAGC_CTRL_B: core::ffi::c_int = 27;
pub const LAGC_CTRL_C: core::ffi::c_int = 28;
pub const RAGC_CTRL_A: core::ffi::c_int = 29;
pub const RAGC_CTRL_B: core::ffi::c_int = 30;
pub const RAGC_CTRL_C: core::ffi::c_int = 31;

/* DAC Power and Left High Power Output control registers */
pub const DAC_PWR: core::ffi::c_int = 37;
pub const HPLCOM_CFG: core::ffi::c_int = 37;
/* Right High Power Output control registers */
pub const HPRCOM_CFG: core::ffi::c_int = 38;
/* High Power Output Stage Control Register */
pub const HPOUT_SC: core::ffi::c_int = 40;
/* DAC Output Switching control registers */
pub const DAC_LINE_MUX: core::ffi::c_int = 41;
/* High Power Output Driver Pop Reduction registers */
pub const HPOUT_POP_REDUCTION: core::ffi::c_int = 42;
/* DAC Digital control registers */
pub const LDAC_VOL: core::ffi::c_int = 43;
pub const RDAC_VOL: core::ffi::c_int = 44;
/* Left High Power Output control registers */
pub const LINE2L_2_HPLOUT_VOL: core::ffi::c_int = 45;
pub const PGAL_2_HPLOUT_VOL: core::ffi::c_int = 46;
pub const DACL1_2_HPLOUT_VOL: core::ffi::c_int = 47;
pub const LINE2R_2_HPLOUT_VOL: core::ffi::c_int = 48;
pub const PGAR_2_HPLOUT_VOL: core::ffi::c_int = 49;
pub const DACR1_2_HPLOUT_VOL: core::ffi::c_int = 50;
pub const HPLOUT_CTRL: core::ffi::c_int = 51;
/* Left High Power COM control registers */
pub const LINE2L_2_HPLCOM_VOL: core::ffi::c_int = 52;
pub const PGAL_2_HPLCOM_VOL: core::ffi::c_int = 53;
pub const DACL1_2_HPLCOM_VOL: core::ffi::c_int = 54;
pub const LINE2R_2_HPLCOM_VOL: core::ffi::c_int = 55;
pub const PGAR_2_HPLCOM_VOL: core::ffi::c_int = 56;
pub const DACR1_2_HPLCOM_VOL: core::ffi::c_int = 57;
pub const HPLCOM_CTRL: core::ffi::c_int = 58;
/* Right High Power Output control registers */
pub const LINE2L_2_HPROUT_VOL: core::ffi::c_int = 59;
pub const PGAL_2_HPROUT_VOL: core::ffi::c_int = 60;
pub const DACL1_2_HPROUT_VOL: core::ffi::c_int = 61;
pub const LINE2R_2_HPROUT_VOL: core::ffi::c_int = 62;
pub const PGAR_2_HPROUT_VOL: core::ffi::c_int = 63;
pub const DACR1_2_HPROUT_VOL: core::ffi::c_int = 64;
pub const HPROUT_CTRL: core::ffi::c_int = 65;
/* Right High Power COM control registers */
pub const LINE2L_2_HPRCOM_VOL: core::ffi::c_int = 66;
pub const PGAL_2_HPRCOM_VOL: core::ffi::c_int = 67;
pub const DACL1_2_HPRCOM_VOL: core::ffi::c_int = 68;
pub const LINE2R_2_HPRCOM_VOL: core::ffi::c_int = 69;
pub const PGAR_2_HPRCOM_VOL: core::ffi::c_int = 70;
pub const DACR1_2_HPRCOM_VOL: core::ffi::c_int = 71;
pub const HPRCOM_CTRL: core::ffi::c_int = 72;
/* Mono Line Output Plus/Minus control registers */
pub const LINE2L_2_MONOLOPM_VOL: core::ffi::c_int = 73;
pub const PGAL_2_MONOLOPM_VOL: core::ffi::c_int = 74;
pub const DACL1_2_MONOLOPM_VOL: core::ffi::c_int = 75;
pub const LINE2R_2_MONOLOPM_VOL: core::ffi::c_int = 76;
pub const PGAR_2_MONOLOPM_VOL: core::ffi::c_int = 77;
pub const DACR1_2_MONOLOPM_VOL: core::ffi::c_int = 78;
pub const MONOLOPM_CTRL: core::ffi::c_int = 79;
/* Class-D speaker driver on tlv320aic3007 */
pub const CLASSD_CTRL: core::ffi::c_int = 73;
/* Left Line Output Plus/Minus control registers */
pub const LINE2L_2_LLOPM_VOL: core::ffi::c_int = 80;
pub const PGAL_2_LLOPM_VOL: core::ffi::c_int = 81;
pub const DACL1_2_LLOPM_VOL: core::ffi::c_int = 82;
pub const LINE2R_2_LLOPM_VOL: core::ffi::c_int = 83;
pub const PGAR_2_LLOPM_VOL: core::ffi::c_int = 84;
pub const DACR1_2_LLOPM_VOL: core::ffi::c_int = 85;
pub const LLOPM_CTRL: core::ffi::c_int = 86;
/* Right Line Output Plus/Minus control registers */
pub const LINE2L_2_RLOPM_VOL: core::ffi::c_int = 87;
pub const PGAL_2_RLOPM_VOL: core::ffi::c_int = 88;
pub const DACL1_2_RLOPM_VOL: core::ffi::c_int = 89;
pub const LINE2R_2_RLOPM_VOL: core::ffi::c_int = 90;
pub const PGAR_2_RLOPM_VOL: core::ffi::c_int = 91;
pub const DACR1_2_RLOPM_VOL: core::ffi::c_int = 92;
pub const RLOPM_CTRL: core::ffi::c_int = 93;
/* GPIO/IRQ registers */
pub const AIC3X_STICKY_IRQ_FLAGS_REG: core::ffi::c_int = 96;
pub const AIC3X_RT_IRQ_FLAGS_REG: core::ffi::c_int = 97;
pub const AIC3X_GPIO1_REG: core::ffi::c_int = 98;
pub const AIC3X_GPIO2_REG: core::ffi::c_int = 99;
pub const AIC3X_GPIOA_REG: core::ffi::c_int = 100;
pub const AIC3X_GPIOB_REG: core::ffi::c_int = 101;
/* Clock generation control register */
pub const AIC3X_CLKGEN_CTRL_REG: core::ffi::c_int = 102;
/* New AGC registers */
pub const LAGCN_ATTACK: core::ffi::c_int = 103;
pub const LAGCN_DECAY: core::ffi::c_int = 104;
pub const RAGCN_ATTACK: core::ffi::c_int = 105;
pub const RAGCN_DECAY: core::ffi::c_int = 106;
/* New Programmable ADC Digital Path and I2C Bus Condition Register */
pub const NEW_ADC_DIGITALPATH: core::ffi::c_int = 107;
/* Passive Analog Signal Bypass Selection During Powerdown Register */
pub const PASSIVE_BYPASS: core::ffi::c_int = 108;
/* DAC Quiescent Current Adjustment Register */
pub const DAC_ICC_ADJ: core::ffi::c_int = 109;

/* Page select register bits */
pub const PAGE0_SELECT: core::ffi::c_int = 0;
pub const PAGE1_SELECT: core::ffi::c_int = 1;

/* Audio serial data interface control register A bits */
pub const BIT_CLK_MASTER: core::ffi::c_int = 0x80;
pub const WORD_CLK_MASTER: core::ffi::c_int = 0x40;
pub const DOUT_TRISTATE: core::ffi::c_int = 0x20;

/* Codec Datapath setup register 7 */
pub const FSREF_44100: core::ffi::c_int = 1 << 7;
pub const FSREF_48000: core::ffi::c_int = 0 << 7;
pub const DUAL_RATE_MODE: core::ffi::c_int = (1 << 5) | (1 << 6);
pub const LDAC2LCH: core::ffi::c_int = 0x1 << 3;
pub const RDAC2RCH: core::ffi::c_int = 0x1 << 1;
pub const LDAC2RCH: core::ffi::c_int = 0x2 << 3;
pub const RDAC2LCH: core::ffi::c_int = 0x2 << 1;
pub const LDAC2MONOMIX: core::ffi::c_int = 0x3 << 3;
pub const RDAC2MONOMIX: core::ffi::c_int = 0x3 << 1;

/* PLL registers bitfields */
pub const PLLP_SHIFT: core::ffi::c_int = 0;
pub const PLLP_MASK: core::ffi::c_int = 7;
pub const PLLQ_SHIFT: core::ffi::c_int = 3;
pub const PLLR_SHIFT: core::ffi::c_int = 0;
pub const PLLJ_SHIFT: core::ffi::c_int = 2;
pub const PLLD_MSB_SHIFT: core::ffi::c_int = 0;
pub const PLLD_LSB_SHIFT: core::ffi::c_int = 2;

/* Clock generation register bits */
pub const CODEC_CLKIN_PLLDIV: core::ffi::c_int = 0;
pub const CODEC_CLKIN_CLKDIV: core::ffi::c_int = 1;
pub const PLL_CLKIN_SHIFT: core::ffi::c_int = 4;
pub const MCLK_SOURCE: core::ffi::c_int = 0x0;
pub const PLL_CLKDIV_SHIFT: core::ffi::c_int = 0;
pub const PLLCLK_IN_MASK: core::ffi::c_int = 0x30;
pub const PLLCLK_IN_SHIFT: core::ffi::c_int = 4;
pub const CLKDIV_IN_MASK: core::ffi::c_int = 0xc0;
pub const CLKDIV_IN_SHIFT: core::ffi::c_int = 6;
/* clock in source */
pub const CLKIN_MCLK: core::ffi::c_int = 0;
pub const CLKIN_GPIO2: core::ffi::c_int = 1;
pub const CLKIN_BCLK: core::ffi::c_int = 2;

/* Software reset register bits */
pub const SOFT_RESET: core::ffi::c_int = 0x80;

/* PLL progrramming register A bits */
pub const PLL_ENABLE: core::ffi::c_int = 0x80;

/* Route bits */
pub const ROUTE_ON: core::ffi::c_int = 0x80;

/* Mute bits */
pub const UNMUTE: core::ffi::c_int = 0x08;
pub const MUTE_ON: core::ffi::c_int = 0x80;

/* Power bits */
pub const LADC_PWR_ON: core::ffi::c_int = 0x04;
pub const RADC_PWR_ON: core::ffi::c_int = 0x04;
pub const LDAC_PWR_ON: core::ffi::c_int = 0x80;
pub const RDAC_PWR_ON: core::ffi::c_int = 0x40;
pub const HPLOUT_PWR_ON: core::ffi::c_int = 0x01;
pub const HPROUT_PWR_ON: core::ffi::c_int = 0x01;
pub const HPLCOM_PWR_ON: core::ffi::c_int = 0x01;
pub const HPRCOM_PWR_ON: core::ffi::c_int = 0x01;
pub const MONOLOPM_PWR_ON: core::ffi::c_int = 0x01;
pub const LLOPM_PWR_ON: core::ffi::c_int = 0x01;
pub const RLOPM_PWR_ON: core::ffi::c_int = 0x01;

pub const fn INVERT_VOL(val: core::ffi::c_int) -> core::ffi::c_int {
    0x7f - val
}

/* Default output volume (inverted) */
pub const DEFAULT_VOL: core::ffi::c_int = INVERT_VOL(0x50);
/* Default input volume */
pub const DEFAULT_GAIN: core::ffi::c_int = 0x20;

/* MICBIAS Control Register */
pub const MICBIAS_LEVEL_SHIFT: core::ffi::c_int = 6;
pub const MICBIAS_LEVEL_MASK: core::ffi::c_int = 3 << 6;

/* HPOUT_SC */
pub const HPOUT_SC_OCMV_MASK: core::ffi::c_int = 3 << 6;
pub const HPOUT_SC_OCMV_SHIFT: core::ffi::c_int = 6;
pub const HPOUT_SC_OCMV_1_35V: core::ffi::c_int = 0;
pub const HPOUT_SC_OCMV_1_5V: core::ffi::c_int = 1;
pub const HPOUT_SC_OCMV_1_65V: core::ffi::c_int = 2;
pub const HPOUT_SC_OCMV_1_8V: core::ffi::c_int = 3;

/* headset detection / button API */

/* The AIC3x supports detection of stereo headsets (GND + left + right signal)
 * and cellular headsets (GND + speaker output + microphone input).
 * It is recommended to enable MIC bias for this function to work properly.
 * For more information, please refer to the datasheet. */
pub const AIC3X_HEADSET_DETECT_OFF: core::ffi::c_int = 0;
pub const AIC3X_HEADSET_DETECT_STEREO: core::ffi::c_int = 1;
pub const AIC3X_HEADSET_DETECT_CELLULAR: core::ffi::c_int = 2;
pub const AIC3X_HEADSET_DETECT_BOTH: core::ffi::c_int = 3;

pub const AIC3X_HEADSET_DEBOUNCE_16MS: core::ffi::c_int = 0;
pub const AIC3X_HEADSET_DEBOUNCE_32MS: core::ffi::c_int = 1;
pub const AIC3X_HEADSET_DEBOUNCE_64MS: core::ffi::c_int = 2;
pub const AIC3X_HEADSET_DEBOUNCE_128MS: core::ffi::c_int = 3;
pub const AIC3X_HEADSET_DEBOUNCE_256MS: core::ffi::c_int = 4;
pub const AIC3X_HEADSET_DEBOUNCE_512MS: core::ffi::c_int = 5;

pub const AIC3X_BUTTON_DEBOUNCE_0MS: core::ffi::c_int = 0;
pub const AIC3X_BUTTON_DEBOUNCE_8MS: core::ffi::c_int = 1;
pub const AIC3X_BUTTON_DEBOUNCE_16MS: core::ffi::c_int = 2;
pub const AIC3X_BUTTON_DEBOUNCE_32MS: core::ffi::c_int = 3;

pub const AIC3X_HEADSET_DETECT_ENABLED: core::ffi::c_int = 0x80;
pub const AIC3X_HEADSET_DETECT_SHIFT: core::ffi::c_int = 5;
pub const AIC3X_HEADSET_DETECT_MASK: core::ffi::c_int = 3;
pub const AIC3X_HEADSET_DEBOUNCE_SHIFT: core::ffi::c_int = 2;
pub const AIC3X_HEADSET_DEBOUNCE_MASK: core::ffi::c_int = 7;
pub const AIC3X_BUTTON_DEBOUNCE_SHIFT: core::ffi::c_int = 0;
pub const AIC3X_BUTTON_DEBOUNCE_MASK: core::ffi::c_int = 3;

/* GPIO API */
pub const AIC3X_GPIO1_FUNC_DISABLED: core::ffi::c_int = 0;
pub const AIC3X_GPIO1_FUNC_AUDIO_WORDCLK_ADC: core::ffi::c_int = 1;
pub const AIC3X_GPIO1_FUNC_CLOCK_MUX: core::ffi::c_int = 2;
pub const AIC3X_GPIO1_FUNC_CLOCK_MUX_DIV2: core::ffi::c_int = 3;
pub const AIC3X_GPIO1_FUNC_CLOCK_MUX_DIV4: core::ffi::c_int = 4;
pub const AIC3X_GPIO1_FUNC_CLOCK_MUX_DIV8: core::ffi::c_int = 5;
pub const AIC3X_GPIO1_FUNC_SHORT_CIRCUIT_IRQ: core::ffi::c_int = 6;
pub const AIC3X_GPIO1_FUNC_AGC_NOISE_IRQ: core::ffi::c_int = 7;
pub const AIC3X_GPIO1_FUNC_INPUT: core::ffi::c_int = 8;
pub const AIC3X_GPIO1_FUNC_OUTPUT: core::ffi::c_int = 9;
pub const AIC3X_GPIO1_FUNC_DIGITAL_MIC_MODCLK: core::ffi::c_int = 10;
pub const AIC3X_GPIO1_FUNC_AUDIO_WORDCLK: core::ffi::c_int = 11;
pub const AIC3X_GPIO1_FUNC_BUTTON_IRQ: core::ffi::c_int = 12;
pub const AIC3X_GPIO1_FUNC_HEADSET_DETECT_IRQ: core::ffi::c_int = 13;
pub const AIC3X_GPIO1_FUNC_HEADSET_DETECT_OR_BUTTON_IRQ: core::ffi::c_int = 14;
pub const AIC3X_GPIO1_FUNC_ALL_IRQ: core::ffi::c_int = 16;

pub const AIC3X_GPIO2_FUNC_DISABLED: core::ffi::c_int = 0;
pub const AIC3X_GPIO2_FUNC_HEADSET_DETECT_IRQ: core::ffi::c_int = 2;
pub const AIC3X_GPIO2_FUNC_INPUT: core::ffi::c_int = 3;
pub const AIC3X_GPIO2_FUNC_OUTPUT: core::ffi::c_int = 4;
pub const AIC3X_GPIO2_FUNC_DIGITAL_MIC_INPUT: core::ffi::c_int = 5;
pub const AIC3X_GPIO2_FUNC_AUDIO_BITCLK: core::ffi::c_int = 8;
pub const AIC3X_GPIO2_FUNC_HEADSET_DETECT_OR_BUTTON_IRQ: core::ffi::c_int = 9;
pub const AIC3X_GPIO2_FUNC_ALL_IRQ: core::ffi::c_int = 10;
pub const AIC3X_GPIO2_FUNC_SHORT_CIRCUIT_OR_AGC_IRQ: core::ffi::c_int = 11;
pub const AIC3X_GPIO2_FUNC_HEADSET_OR_BUTTON_PRESS_OR_SHORT_CIRCUIT_IRQ: core::ffi::c_int = 12;
pub const AIC3X_GPIO2_FUNC_SHORT_CIRCUIT_IRQ: core::ffi::c_int = 13;
pub const AIC3X_GPIO2_FUNC_AGC_NOISE_IRQ: core::ffi::c_int = 14;
pub const AIC3X_GPIO2_FUNC_BUTTON_PRESS_IRQ: core::ffi::c_int = 15;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum aic3x_micbias_voltage {
    AIC3X_MICBIAS_OFF = 0,
    AIC3X_MICBIAS_2_0V = 1,
    AIC3X_MICBIAS_2_5V = 2,
    AIC3X_MICBIAS_AVDDV = 3,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
