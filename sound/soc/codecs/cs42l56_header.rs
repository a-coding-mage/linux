// SPDX-License-Identifier: GPL-2.0-only
/*
 * cs42l52.h -- CS42L56 ALSA SoC audio driver
 *
 * Copyright 2014 CirrusLogic, Inc.
 *
 * Author: Brian Austin <brian.austin@cirrus.com>
 */

pub const CS42L56_CHIP_ID_1: u32 = 0x01;
pub const CS42L56_CHIP_ID_2: u32 = 0x02;
pub const CS42L56_PWRCTL_1: u32 = 0x03;
pub const CS42L56_PWRCTL_2: u32 = 0x04;
pub const CS42L56_CLKCTL_1: u32 = 0x05;
pub const CS42L56_CLKCTL_2: u32 = 0x06;
pub const CS42L56_SERIAL_FMT: u32 = 0x07;
pub const CS42L56_CLASSH_CTL: u32 = 0x08;
pub const CS42L56_MISC_CTL: u32 = 0x09;
pub const CS42L56_INT_STATUS: u32 = 0x0a;
pub const CS42L56_PLAYBACK_CTL: u32 = 0x0b;
pub const CS42L56_DSP_MUTE_CTL: u32 = 0x0c;
pub const CS42L56_ADCA_MIX_VOLUME: u32 = 0x0d;
pub const CS42L56_ADCB_MIX_VOLUME: u32 = 0x0e;
pub const CS42L56_PCMA_MIX_VOLUME: u32 = 0x0f;
pub const CS42L56_PCMB_MIX_VOLUME: u32 = 0x10;
pub const CS42L56_ANAINPUT_ADV_VOLUME: u32 = 0x11;
pub const CS42L56_DIGINPUT_ADV_VOLUME: u32 = 0x12;
pub const CS42L56_MASTER_A_VOLUME: u32 = 0x13;
pub const CS42L56_MASTER_B_VOLUME: u32 = 0x14;
pub const CS42L56_BEEP_FREQ_ONTIME: u32 = 0x15;
pub const CS42L56_BEEP_FREQ_OFFTIME: u32 = 0x16;
pub const CS42L56_BEEP_TONE_CFG: u32 = 0x17;
pub const CS42L56_TONE_CTL: u32 = 0x18;
pub const CS42L56_CHAN_MIX_SWAP: u32 = 0x19;
pub const CS42L56_AIN_REFCFG_ADC_MUX: u32 = 0x1a;
pub const CS42L56_HPF_CTL: u32 = 0x1b;
pub const CS42L56_MISC_ADC_CTL: u32 = 0x1c;
pub const CS42L56_GAIN_BIAS_CTL: u32 = 0x1d;
pub const CS42L56_PGAA_MUX_VOLUME: u32 = 0x1e;
pub const CS42L56_PGAB_MUX_VOLUME: u32 = 0x1f;
pub const CS42L56_ADCA_ATTENUATOR: u32 = 0x20;
pub const CS42L56_ADCB_ATTENUATOR: u32 = 0x21;
pub const CS42L56_ALC_EN_ATTACK_RATE: u32 = 0x22;
pub const CS42L56_ALC_RELEASE_RATE: u32 = 0x23;
pub const CS42L56_ALC_THRESHOLD: u32 = 0x24;
pub const CS42L56_NOISE_GATE_CTL: u32 = 0x25;
pub const CS42L56_ALC_LIM_SFT_ZC: u32 = 0x26;
pub const CS42L56_AMUTE_HPLO_MUX: u32 = 0x27;
pub const CS42L56_HPA_VOLUME: u32 = 0x28;
pub const CS42L56_HPB_VOLUME: u32 = 0x29;
pub const CS42L56_LOA_VOLUME: u32 = 0x2a;
pub const CS42L56_LOB_VOLUME: u32 = 0x2b;
pub const CS42L56_LIM_THRESHOLD_CTL: u32 = 0x2c;
pub const CS42L56_LIM_CTL_RELEASE_RATE: u32 = 0x2d;
pub const CS42L56_LIM_ATTACK_RATE: u32 = 0x2e;

/* Device ID and Rev ID Masks */
pub const CS42L56_DEVID: u32 = 0x56;
pub const CS42L56_CHIP_ID_MASK: u32 = 0xff;
pub const CS42L56_AREV_MASK: u32 = 0x1c;
pub const CS42L56_MTLREV_MASK: u32 = 0x03;

/* Power bit masks */
pub const CS42L56_PDN_ALL_MASK: u32 = 0x01;
pub const CS42L56_PDN_ADCA_MASK: u32 = 0x02;
pub const CS42L56_PDN_ADCB_MASK: u32 = 0x04;
pub const CS42L56_PDN_CHRG_MASK: u32 = 0x08;
pub const CS42L56_PDN_BIAS_MASK: u32 = 0x10;
pub const CS42L56_PDN_VBUF_MASK: u32 = 0x20;
pub const CS42L56_PDN_LOA_MASK: u32 = 0x03;
pub const CS42L56_PDN_LOB_MASK: u32 = 0x0c;
pub const CS42L56_PDN_HPA_MASK: u32 = 0x30;
pub const CS42L56_PDN_HPB_MASK: u32 = 0xc0;

/* serial port and clk masks */
pub const CS42L56_MASTER_MODE: u32 = 0x40;
pub const CS42L56_SLAVE_MODE: u32 = 0;
pub const CS42L56_MS_MODE_MASK: u32 = 0x40;
pub const CS42L56_SCLK_INV: u32 = 0x20;
pub const CS42L56_SCLK_INV_MASK: u32 = 0x20;
pub const CS42L56_SCLK_MCLK_MASK: u32 = 0x18;
pub const CS42L56_MCLK_PREDIV: u32 = 0x04;
pub const CS42L56_MCLK_PREDIV_MASK: u32 = 0x04;
pub const CS42L56_MCLK_DIV2: u32 = 0x02;
pub const CS42L56_MCLK_DIV2_MASK: u32 = 0x02;
pub const CS42L56_MCLK_DIS_MASK: u32 = 0x01;
pub const CS42L56_CLK_AUTO_MASK: u32 = 0x20;
pub const CS42L56_CLK_RATIO_MASK: u32 = 0x1f;
pub const CS42L56_DIG_FMT_I2S: u32 = 0;
pub const CS42L56_DIG_FMT_LEFT_J: u32 = 0x08;
pub const CS42L56_DIG_FMT_MASK: u32 = 0x08;

/* Class H and misc ctl masks */
pub const CS42L56_ADAPT_PWR_MASK: u32 = 0xc0;
pub const CS42L56_CHRG_FREQ_MASK: u32 = 0x0f;
pub const CS42L56_DIG_MUX_MASK: u32 = 0x80;
pub const CS42L56_ANLGSFT_MASK: u32 = 0x10;
pub const CS42L56_ANLGZC_MASK: u32 = 0x08;
pub const CS42L56_DIGSFT_MASK: u32 = 0x04;
pub const CS42L56_FREEZE_MASK: u32 = 0x01;
pub const CS42L56_MIC_BIAS_MASK: u32 = 0x03;
pub const CS42L56_HPFA_FREQ_MASK: u32 = 0x03;
pub const CS42L56_HPFB_FREQ_MASK: u32 = 0xc0;
pub const CS42L56_AIN1A_REF_MASK: u32 = 0x10;
pub const CS42L56_AIN2A_REF_MASK: u32 = 0x40;
pub const CS42L56_AIN1B_REF_MASK: u32 = 0x20;
pub const CS42L56_AIN2B_REF_MASK: u32 = 0x80;

/* Playback Capture ctl masks */
pub const CS42L56_PDN_DSP_MASK: u32 = 0x80;
pub const CS42L56_DEEMPH_MASK: u32 = 0x40;
pub const CS42L56_PLYBCK_GANG_MASK: u32 = 0x10;
pub const CS42L56_PCM_INV_MASK: u32 = 0x0c;
pub const CS42L56_MUTE_ALL: u32 = 0xff;
pub const CS42L56_UNMUTE: u32 = 0;
pub const CS42L56_ADCAMIX_MUTE_MASK: u32 = 0x40;
pub const CS42L56_ADCBMIX_MUTE_MASK: u32 = 0x80;
pub const CS42L56_PCMAMIX_MUTE_MASK: u32 = 0x10;
pub const CS42L56_PCMBMIX_MUTE_MASK: u32 = 0x20;
pub const CS42L56_MSTB_MUTE_MASK: u32 = 0x02;
pub const CS42L56_MSTA_MUTE_MASK: u32 = 0x01;
pub const CS42L56_ADCA_MUTE_MASK: u32 = 0x01;
pub const CS42L56_ADCB_MUTE_MASK: u32 = 0x02;
pub const CS42L56_HP_MUTE_MASK: u32 = 0x80;
pub const CS42L56_LO_MUTE_MASK: u32 = 0x80;

/* Beep masks */
pub const CS42L56_BEEP_FREQ_MASK: u32 = 0xf0;
pub const CS42L56_BEEP_ONTIME_MASK: u32 = 0x0f;
pub const CS42L56_BEEP_OFFTIME_MASK: u32 = 0xe0;
pub const CS42L56_BEEP_CFG_MASK: u32 = 0xc0;
pub const CS42L56_BEEP_TREBCF_MASK: u32 = 0x18;
pub const CS42L56_BEEP_BASSCF_MASK: u32 = 0x06;
pub const CS42L56_BEEP_TCEN_MASK: u32 = 0x01;
pub const CS42L56_BEEP_RATE_SHIFT: u32 = 4;
pub const CS42L56_BEEP_EN_MASK: u32 = 0x3f;

/* Supported MCLKS */
pub const CS42L56_MCLK_5P6448MHZ: u32 = 5644800;
pub const CS42L56_MCLK_6MHZ: u32 = 6000000;
pub const CS42L56_MCLK_6P144MHZ: u32 = 6144000;
pub const CS42L56_MCLK_11P2896MHZ: u32 = 11289600;
pub const CS42L56_MCLK_12MHZ: u32 = 12000000;
pub const CS42L56_MCLK_12P288MHZ: u32 = 12288000;
pub const CS42L56_MCLK_22P5792MHZ: u32 = 22579200;
pub const CS42L56_MCLK_24MHZ: u32 = 24000000;
pub const CS42L56_MCLK_24P576MHZ: u32 = 24576000;

/* Clock ratios */
pub const CS42L56_MCLK_LRCLK_128: u32 = 0x08;
pub const CS42L56_MCLK_LRCLK_125: u32 = 0x09;
pub const CS42L56_MCLK_LRCLK_136: u32 = 0x0b;
pub const CS42L56_MCLK_LRCLK_192: u32 = 0x0c;
pub const CS42L56_MCLK_LRCLK_187P5: u32 = 0x0d;
pub const CS42L56_MCLK_LRCLK_256: u32 = 0x10;
pub const CS42L56_MCLK_LRCLK_250: u32 = 0x11;
pub const CS42L56_MCLK_LRCLK_272: u32 = 0x13;
pub const CS42L56_MCLK_LRCLK_384: u32 = 0x14;
pub const CS42L56_MCLK_LRCLK_375: u32 = 0x15;
pub const CS42L56_MCLK_LRCLK_512: u32 = 0x18;
pub const CS42L56_MCLK_LRCLK_500: u32 = 0x19;
pub const CS42L56_MCLK_LRCLK_544: u32 = 0x1b;
pub const CS42L56_MCLK_LRCLK_750: u32 = 0x1c;
pub const CS42L56_MCLK_LRCLK_768: u32 = 0x1d;

pub const CS42L56_MAX_REGISTER: u32 = 0x34;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
