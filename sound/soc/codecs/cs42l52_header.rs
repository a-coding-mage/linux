/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * cs42l52.h -- CS42L52 ALSA SoC audio driver
 *
 * Copyright 2012 CirrusLogic, Inc.
 *
 * Author: Georgi Vlaev <joe@nucleusys.com>
 * Author: Brian Austin <brian.austin@cirrus.com>
 */

// Header guard removed in Rust.

pub const CS42L52_NAME: &str = "CS42L52";
pub const CS42L52_DEFAULT_CLK: u32 = 12000000;
pub const CS42L52_MIN_CLK: u32 = 11000000;
pub const CS42L52_MAX_CLK: u32 = 27000000;
pub const CS42L52_DEFAULT_FORMAT: u32 = SNDRV_PCM_FMTBIT_S16_LE;
pub const CS42L52_DEFAULT_MAX_CHANS: u32 = 2;
pub const CS42L52_SYSCLK: u32 = 1;

pub const CS42L52_CHIP_SWICTH: u32 = 1 << 17;
pub const CS42L52_ALL_IN_ONE: u32 = 1 << 16;
pub const CS42L52_CHIP_ONE: u32 = 0x00;
pub const CS42L52_CHIP_TWO: u32 = 0x01;
pub const CS42L52_CHIP_THR: u32 = 0x02;
pub const CS42L52_CHIP_MASK: u32 = 0x0f;

pub const CS42L52_FIX_BITS_CTL: u32 = 0x00;
pub const CS42L52_CHIP: u32 = 0x01;
pub const CS42L52_CHIP_ID: u32 = 0xE0;
pub const CS42L52_CHIP_ID_MASK: u32 = 0xF8;
pub const CS42L52_CHIP_REV_A0: u32 = 0x00;
pub const CS42L52_CHIP_REV_A1: u32 = 0x01;
pub const CS42L52_CHIP_REV_B0: u32 = 0x02;
pub const CS42L52_CHIP_REV_MASK: u32 = 0x07;

pub const CS42L52_PWRCTL1: u32 = 0x02;
pub const CS42L52_PWRCTL1_PDN_ALL: u32 = 0x9F;
pub const CS42L52_PWRCTL1_PDN_CHRG: u32 = 0x80;
pub const CS42L52_PWRCTL1_PDN_PGAB: u32 = 0x10;
pub const CS42L52_PWRCTL1_PDN_PGAA: u32 = 0x08;
pub const CS42L52_PWRCTL1_PDN_ADCB: u32 = 0x04;
pub const CS42L52_PWRCTL1_PDN_ADCA: u32 = 0x02;
pub const CS42L52_PWRCTL1_PDN_CODEC: u32 = 0x01;

pub const CS42L52_PWRCTL2: u32 = 0x03;
pub const CS42L52_PWRCTL2_OVRDB: u32 = 1 << 4;
pub const CS42L52_PWRCTL2_OVRDA: u32 = 1 << 3;
pub const CS42L52_PWRCTL2_PDN_MICB: u32 = 1 << 2;
pub const CS42L52_PWRCTL2_PDN_MICB_SHIFT: u32 = 2;
pub const CS42L52_PWRCTL2_PDN_MICA: u32 = 1 << 1;
pub const CS42L52_PWRCTL2_PDN_MICA_SHIFT: u32 = 1;
pub const CS42L52_PWRCTL2_PDN_MICBIAS: u32 = 1 << 0;
pub const CS42L52_PWRCTL2_PDN_MICBIAS_SHIFT: u32 = 0;

pub const CS42L52_PWRCTL3: u32 = 0x04;
pub const CS42L52_PWRCTL3_HPB_PDN_SHIFT: u32 = 6;
pub const CS42L52_PWRCTL3_HPB_ON_LOW: u32 = 0x00;
pub const CS42L52_PWRCTL3_HPB_ON_HIGH: u32 = 0x01;
pub const CS42L52_PWRCTL3_HPB_ALWAYS_ON: u32 = 0x02;
pub const CS42L52_PWRCTL3_HPB_ALWAYS_OFF: u32 = 0x03;
pub const CS42L52_PWRCTL3_HPA_PDN_SHIFT: u32 = 4;
pub const CS42L52_PWRCTL3_HPA_ON_LOW: u32 = 0x00;
pub const CS42L52_PWRCTL3_HPA_ON_HIGH: u32 = 0x01;
pub const CS42L52_PWRCTL3_HPA_ALWAYS_ON: u32 = 0x02;
pub const CS42L52_PWRCTL3_HPA_ALWAYS_OFF: u32 = 0x03;
pub const CS42L52_PWRCTL3_SPKB_PDN_SHIFT: u32 = 2;
pub const CS42L52_PWRCTL3_SPKB_ON_LOW: u32 = 0x00;
pub const CS42L52_PWRCTL3_SPKB_ON_HIGH: u32 = 0x01;
pub const CS42L52_PWRCTL3_SPKB_ALWAYS_ON: u32 = 0x02;
pub const CS42L52_PWRCTL3_PDN_SPKB: u32 = 1 << 2;
pub const CS42L52_PWRCTL3_PDN_SPKA: u32 = 1 << 0;
pub const CS42L52_PWRCTL3_SPKA_PDN_SHIFT: u32 = 0;
pub const CS42L52_PWRCTL3_SPKA_ON_LOW: u32 = 0x00;
pub const CS42L52_PWRCTL3_SPKA_ON_HIGH: u32 = 0x01;
pub const CS42L52_PWRCTL3_SPKA_ALWAYS_ON: u32 = 0x02;

pub const CS42L52_DEFAULT_OUTPUT_STATE: u32 = 0x05;
pub const CS42L52_PWRCTL3_CONF_MASK: u32 = 0x03;

pub const CS42L52_CLK_CTL: u32 = 0x05;
pub const CLK_AUTODECT_ENABLE: u32 = 1 << 7;
pub const CLK_SPEED_SHIFT: u32 = 5;
pub const CLK_DS_MODE: u32 = 0x00;
pub const CLK_SS_MODE: u32 = 0x01;
pub const CLK_HS_MODE: u32 = 0x02;
pub const CLK_QS_MODE: u32 = 0x03;
pub const CLK_32K_SR_SHIFT: u32 = 4;
pub const CLK_32K: u32 = 0x01;
pub const CLK_NO_32K: u32 = 0x00;
pub const CLK_27M_MCLK_SHIFT: u32 = 3;
pub const CLK_27M_MCLK: u32 = 0x01;
pub const CLK_NO_27M: u32 = 0x00;
pub const CLK_RATIO_SHIFT: u32 = 1;
pub const CLK_R_128: u32 = 0x00;
pub const CLK_R_125: u32 = 0x01;
pub const CLK_R_132: u32 = 0x02;
pub const CLK_R_136: u32 = 0x03;

pub const CS42L52_IFACE_CTL1: u32 = 0x06;
pub const CS42L52_IFACE_CTL1_MASTER: u32 = 1 << 7;
pub const CS42L52_IFACE_CTL1_SLAVE: u32 = 0 << 7;
pub const CS42L52_IFACE_CTL1_INV_SCLK: u32 = 1 << 6;
pub const CS42L52_IFACE_CTL1_ADC_FMT_I2S: u32 = 1 << 5;
pub const CS42L52_IFACE_CTL1_ADC_FMT_LEFT_J: u32 = 0 << 5;
pub const CS42L52_IFACE_CTL1_DSP_MODE_EN: u32 = 1 << 4;
pub const CS42L52_IFACE_CTL1_DAC_FMT_LEFT_J: u32 = 0 << 2;
pub const CS42L52_IFACE_CTL1_DAC_FMT_I2S: u32 = 1 << 2;
pub const CS42L52_IFACE_CTL1_DAC_FMT_RIGHT_J: u32 = 2 << 2;
pub const CS42L52_IFACE_CTL1_WL_32BIT: u32 = 0x00;
pub const CS42L52_IFACE_CTL1_WL_24BIT: u32 = 0x01;
pub const CS42L52_IFACE_CTL1_WL_20BIT: u32 = 0x02;
pub const CS42L52_IFACE_CTL1_WL_16BIT: u32 = 0x03;
pub const CS42L52_IFACE_CTL1_WL_MASK: u32 = 0xFFFF;

pub const CS42L52_IFACE_CTL2: u32 = 0x07;
pub const CS42L52_IFACE_CTL2_SC_MC_EQ: u32 = 1 << 6;
pub const CS42L52_IFACE_CTL2_LOOPBACK: u32 = 1 << 5;
pub const CS42L52_IFACE_CTL2_S_MODE_OUTPUT_EN: u32 = 0 << 4;
pub const CS42L52_IFACE_CTL2_S_MODE_OUTPUT_HIZ: u32 = 1 << 4;
pub const CS42L52_IFACE_CTL2_HP_SW_INV: u32 = 1 << 3;
pub const CS42L52_IFACE_CTL2_BIAS_LVL: u32 = 0x07;

pub const CS42L52_ADC_PGA_A: u32 = 0x08;
pub const CS42L52_ADC_PGA_B: u32 = 0x09;
pub const CS42L52_ADC_SEL_SHIFT: u32 = 5;
pub const CS42L52_ADC_SEL_AIN1: u32 = 0x00;
pub const CS42L52_ADC_SEL_AIN2: u32 = 0x01;
pub const CS42L52_ADC_SEL_AIN3: u32 = 0x02;
pub const CS42L52_ADC_SEL_AIN4: u32 = 0x03;
pub const CS42L52_ADC_SEL_PGA: u32 = 0x04;

pub const CS42L52_ANALOG_HPF_CTL: u32 = 0x0A;
pub const CS42L52_HPF_CTL_ANLGSFTB: u32 = 1 << 3;
pub const CS42L52_HPF_CTL_ANLGSFTA: u32 = 1 << 0;

pub const CS42L52_ADC_HPF_FREQ: u32 = 0x0B;
pub const CS42L52_ADC_MISC_CTL: u32 = 0x0C;
pub const CS42L52_ADC_MISC_CTL_SOURCE_DSP: u32 = 1 << 6;

pub const CS42L52_PB_CTL1: u32 = 0x0D;
pub const CS42L52_PB_CTL1_HP_GAIN_SHIFT: u32 = 5;
pub const CS42L52_PB_CTL1_HP_GAIN_03959: u32 = 0x00;
pub const CS42L52_PB_CTL1_HP_GAIN_04571: u32 = 0x01;
pub const CS42L52_PB_CTL1_HP_GAIN_05111: u32 = 0x02;
pub const CS42L52_PB_CTL1_HP_GAIN_06047: u32 = 0x03;
pub const CS42L52_PB_CTL1_HP_GAIN_07099: u32 = 0x04;
pub const CS42L52_PB_CTL1_HP_GAIN_08399: u32 = 0x05;
pub const CS42L52_PB_CTL1_HP_GAIN_10000: u32 = 0x06;
pub const CS42L52_PB_CTL1_HP_GAIN_11430: u32 = 0x07;
pub const CS42L52_PB_CTL1_INV_PCMB: u32 = 1 << 3;
pub const CS42L52_PB_CTL1_INV_PCMA: u32 = 1 << 2;
pub const CS42L52_PB_CTL1_MSTB_MUTE: u32 = 1 << 1;
pub const CS42L52_PB_CTL1_MSTA_MUTE: u32 = 1 << 0;
pub const CS42L52_PB_CTL1_MUTE_MASK: u32 = 0x03;
pub const CS42L52_PB_CTL1_MUTE: u32 = 3;
pub const CS42L52_PB_CTL1_UNMUTE: u32 = 0;

pub const CS42L52_MISC_CTL: u32 = 0x0E;
pub const CS42L52_MISC_CTL_DEEMPH: u32 = 1 << 2;
pub const CS42L52_MISC_CTL_DIGSFT: u32 = 1 << 1;
pub const CS42L52_MISC_CTL_DIGZC: u32 = 1 << 0;

pub const CS42L52_PB_CTL2: u32 = 0x0F;
pub const CS42L52_PB_CTL2_HPB_MUTE: u32 = 1 << 7;
pub const CS42L52_PB_CTL2_HPA_MUTE: u32 = 1 << 6;
pub const CS42L52_PB_CTL2_SPKB_MUTE: u32 = 1 << 5;
pub const CS42L52_PB_CTL2_SPKA_MUTE: u32 = 1 << 4;
pub const CS42L52_PB_CTL2_SPK_SWAP: u32 = 1 << 2;
pub const CS42L52_PB_CTL2_SPK_MONO: u32 = 1 << 1;
pub const CS42L52_PB_CTL2_SPK_MUTE50: u32 = 1 << 0;

pub const CS42L52_MICA_CTL: u32 = 0x10;
pub const CS42L52_MICB_CTL: u32 = 0x11;
pub const CS42L52_MIC_CTL_MIC_SEL_MASK: u32 = 0xBF;
pub const CS42L52_MIC_CTL_MIC_SEL_SHIFT: u32 = 6;
pub const CS42L52_MIC_CTL_TYPE_MASK: u32 = 0x20;
pub const CS42L52_MIC_CTL_TYPE_SHIFT: u32 = 5;

pub const CS42L52_PGAA_CTL: u32 = 0x12;
pub const CS42L52_PGAB_CTL: u32 = 0x13;
pub const CS42L52_PGAX_CTL_VOL_12DB: u32 = 24;
pub const CS42L52_PGAX_CTL_VOL_6DB: u32 = 12; /*step size 0.5db*/

pub const CS42L52_PASSTHRUA_VOL: u32 = 0x14;
pub const CS42L52_PASSTHRUB_VOL: u32 = 0x15;

pub const CS42L52_ADCA_VOL: u32 = 0x16;
pub const CS42L52_ADCB_VOL: u32 = 0x17;
pub const CS42L52_ADCX_VOL_24DB: u32 = 24; /*step size 1db*/
pub const CS42L52_ADCX_VOL_12DB: u32 = 12;
pub const CS42L52_ADCX_VOL_6DB: u32 = 6;

pub const CS42L52_ADCA_MIXER_VOL: u32 = 0x18;
pub const CS42L52_ADCB_MIXER_VOL: u32 = 0x19;
pub const CS42L52_ADC_MIXER_VOL_12DB: u32 = 0x18;

pub const CS42L52_PCMA_MIXER_VOL: u32 = 0x1A;
pub const CS42L52_PCMB_MIXER_VOL: u32 = 0x1B;

pub const CS42L52_BEEP_FREQ: u32 = 0x1C;
pub const CS42L52_BEEP_VOL: u32 = 0x1D;
pub const CS42L52_BEEP_TONE_CTL: u32 = 0x1E;
pub const CS42L52_BEEP_RATE_SHIFT: u32 = 4;
pub const CS42L52_BEEP_RATE_MASK: u32 = 0x0F;

pub const CS42L52_TONE_CTL: u32 = 0x1F;
pub const CS42L52_BEEP_EN_MASK: u32 = 0x3F;

pub const CS42L52_MASTERA_VOL: u32 = 0x20;
pub const CS42L52_MASTERB_VOL: u32 = 0x21;

pub const CS42L52_HPA_VOL: u32 = 0x22;
pub const CS42L52_HPB_VOL: u32 = 0x23;
pub const CS42L52_DEFAULT_HP_VOL: u32 = 0xF0;

pub const CS42L52_SPKA_VOL: u32 = 0x24;
pub const CS42L52_SPKB_VOL: u32 = 0x25;
pub const CS42L52_DEFAULT_SPK_VOL: u32 = 0xF0;

pub const CS42L52_ADC_PCM_MIXER: u32 = 0x26;

pub const CS42L52_LIMITER_CTL1: u32 = 0x27;
pub const CS42L52_LIMITER_CTL2: u32 = 0x28;
pub const CS42L52_LIMITER_AT_RATE: u32 = 0x29;

pub const CS42L52_ALC_CTL: u32 = 0x2A;
pub const CS42L52_ALC_CTL_ALCB_ENABLE_SHIFT: u32 = 7;
pub const CS42L52_ALC_CTL_ALCA_ENABLE_SHIFT: u32 = 6;
pub const CS42L52_ALC_CTL_FASTEST_ATTACK: u32 = 0;

pub const CS42L52_ALC_RATE: u32 = 0x2B;
pub const CS42L52_ALC_SLOWEST_RELEASE: u32 = 0x3F;

pub const CS42L52_ALC_THRESHOLD: u32 = 0x2C;
pub const CS42L52_ALC_MAX_RATE_SHIFT: u32 = 5;
pub const CS42L52_ALC_MIN_RATE_SHIFT: u32 = 2;
pub const CS42L52_ALC_RATE_0DB: u32 = 0;
pub const CS42L52_ALC_RATE_3DB: u32 = 1;
pub const CS42L52_ALC_RATE_6DB: u32 = 2;

pub const CS42L52_NOISE_GATE_CTL: u32 = 0x2D;
pub const CS42L52_NG_ENABLE_SHIFT: u32 = 6;
pub const CS42L52_NG_THRESHOLD_SHIFT: u32 = 2;
pub const CS42L52_NG_MIN_70DB: u32 = 2;
pub const CS42L52_NG_DELAY_SHIFT: u32 = 0;
pub const CS42L52_NG_DELAY_100MS: u32 = 1;

pub const CS42L52_CLK_STATUS: u32 = 0x2E;
pub const CS42L52_BATT_COMPEN: u32 = 0x2F;

pub const CS42L52_BATT_LEVEL: u32 = 0x30;
pub const CS42L52_SPK_STATUS: u32 = 0x31;
pub const CS42L52_SPK_STATUS_PIN_SHIFT: u32 = 3;
pub const CS42L52_SPK_STATUS_PIN_HIGH: u32 = 1;

pub const CS42L52_TEM_CTL: u32 = 0x32;
pub const CS42L52_TEM_CTL_SET: u32 = 0x80;
pub const CS42L52_THE_FOLDBACK: u32 = 0x33;
pub const CS42L52_CHARGE_PUMP: u32 = 0x34;
pub const CS42L52_CHARGE_PUMP_MASK: u32 = 0xF0;
pub const CS42L52_CHARGE_PUMP_SHIFT: u32 = 4;
pub const CS42L52_FIX_BITS1: u32 = 0x3E;
pub const CS42L52_FIX_BITS2: u32 = 0x47;

pub const CS42L52_MAX_REGISTER: u32 = 0x47;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
