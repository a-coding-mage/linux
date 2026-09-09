/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Device Tree defines for Arizona devices
 *
 * Copyright 2015 Cirrus Logic Inc.
 *
 * Author: Charles Keepax <ckeepax@opensource.wolfsonmicro.com>
 */

// Translated from the C header; the original include guard is omitted.

/* GPIO Function Definitions */
pub const ARIZONA_GP_FN_TXLRCLK: u32 = 0x00;
pub const ARIZONA_GP_FN_GPIO: u32 = 0x01;
pub const ARIZONA_GP_FN_IRQ1: u32 = 0x02;
pub const ARIZONA_GP_FN_IRQ2: u32 = 0x03;
pub const ARIZONA_GP_FN_OPCLK: u32 = 0x04;
pub const ARIZONA_GP_FN_FLL1_OUT: u32 = 0x05;
pub const ARIZONA_GP_FN_FLL2_OUT: u32 = 0x06;
pub const ARIZONA_GP_FN_PWM1: u32 = 0x08;
pub const ARIZONA_GP_FN_PWM2: u32 = 0x09;
pub const ARIZONA_GP_FN_SYSCLK_UNDERCLOCKED: u32 = 0x0A;
pub const ARIZONA_GP_FN_ASYNCCLK_UNDERCLOCKED: u32 = 0x0B;
pub const ARIZONA_GP_FN_FLL1_LOCK: u32 = 0x0C;
pub const ARIZONA_GP_FN_FLL2_LOCK: u32 = 0x0D;
pub const ARIZONA_GP_FN_FLL1_CLOCK_OK: u32 = 0x0F;
pub const ARIZONA_GP_FN_FLL2_CLOCK_OK: u32 = 0x10;
pub const ARIZONA_GP_FN_HEADPHONE_DET: u32 = 0x12;
pub const ARIZONA_GP_FN_MIC_DET: u32 = 0x13;
pub const ARIZONA_GP_FN_WSEQ_STATUS: u32 = 0x15;
pub const ARIZONA_GP_FN_CIF_ADDRESS_ERROR: u32 = 0x16;
pub const ARIZONA_GP_FN_ASRC1_LOCK: u32 = 0x1A;
pub const ARIZONA_GP_FN_ASRC2_LOCK: u32 = 0x1B;
pub const ARIZONA_GP_FN_ASRC_CONFIG_ERROR: u32 = 0x1C;
pub const ARIZONA_GP_FN_DRC1_SIGNAL_DETECT: u32 = 0x1D;
pub const ARIZONA_GP_FN_DRC1_ANTICLIP: u32 = 0x1E;
pub const ARIZONA_GP_FN_DRC1_DECAY: u32 = 0x1F;
pub const ARIZONA_GP_FN_DRC1_NOISE: u32 = 0x20;
pub const ARIZONA_GP_FN_DRC1_QUICK_RELEASE: u32 = 0x21;
pub const ARIZONA_GP_FN_DRC2_SIGNAL_DETECT: u32 = 0x22;
pub const ARIZONA_GP_FN_DRC2_ANTICLIP: u32 = 0x23;
pub const ARIZONA_GP_FN_DRC2_DECAY: u32 = 0x24;
pub const ARIZONA_GP_FN_DRC2_NOISE: u32 = 0x25;
pub const ARIZONA_GP_FN_DRC2_QUICK_RELEASE: u32 = 0x26;
pub const ARIZONA_GP_FN_MIXER_DROPPED_SAMPLE: u32 = 0x27;
pub const ARIZONA_GP_FN_AIF1_CONFIG_ERROR: u32 = 0x28;
pub const ARIZONA_GP_FN_AIF2_CONFIG_ERROR: u32 = 0x29;
pub const ARIZONA_GP_FN_AIF3_CONFIG_ERROR: u32 = 0x2A;
pub const ARIZONA_GP_FN_SPK_TEMP_SHUTDOWN: u32 = 0x2B;
pub const ARIZONA_GP_FN_SPK_TEMP_WARNING: u32 = 0x2C;
pub const ARIZONA_GP_FN_UNDERCLOCKED: u32 = 0x2D;
pub const ARIZONA_GP_FN_OVERCLOCKED: u32 = 0x2E;
pub const ARIZONA_GP_FN_DSP_IRQ1: u32 = 0x35;
pub const ARIZONA_GP_FN_DSP_IRQ2: u32 = 0x36;
pub const ARIZONA_GP_FN_ASYNC_OPCLK: u32 = 0x3D;
pub const ARIZONA_GP_FN_BOOT_DONE: u32 = 0x44;
pub const ARIZONA_GP_FN_DSP1_RAM_READY: u32 = 0x45;
pub const ARIZONA_GP_FN_SYSCLK_ENA_STATUS: u32 = 0x4B;
pub const ARIZONA_GP_FN_ASYNCCLK_ENA_STATUS: u32 = 0x4C;

/* GPIO Configuration Bits */
pub const ARIZONA_GPN_DIR: u32 = 0x8000;
pub const ARIZONA_GPN_PU: u32 = 0x4000;
pub const ARIZONA_GPN_PD: u32 = 0x2000;
pub const ARIZONA_GPN_LVL: u32 = 0x0800;
pub const ARIZONA_GPN_POL: u32 = 0x0400;
pub const ARIZONA_GPN_OP_CFG: u32 = 0x0200;
pub const ARIZONA_GPN_DB: u32 = 0x0100;

/* Provide some defines for the most common configs */
pub const ARIZONA_GP_DEFAULT: u32 = 0xffff_ffff;
pub const ARIZONA_GP_OUTPUT: u32 = ARIZONA_GP_FN_GPIO;
pub const ARIZONA_GP_INPUT: u32 = ARIZONA_GP_FN_GPIO | ARIZONA_GPN_DIR;

pub const ARIZONA_32KZ_MCLK1: u32 = 1;
pub const ARIZONA_32KZ_MCLK2: u32 = 2;
pub const ARIZONA_32KZ_NONE: u32 = 3;

pub const ARIZONA_DMIC_MICVDD: u32 = 0;
pub const ARIZONA_DMIC_MICBIAS1: u32 = 1;
pub const ARIZONA_DMIC_MICBIAS2: u32 = 2;
pub const ARIZONA_DMIC_MICBIAS3: u32 = 3;

pub const ARIZONA_INMODE_DIFF: u32 = 0;
pub const ARIZONA_INMODE_SE: u32 = 1;
pub const ARIZONA_INMODE_DMIC: u32 = 2;

pub const ARIZONA_MICD_TIME_CONTINUOUS: u32 = 0;
pub const ARIZONA_MICD_TIME_250US: u32 = 1;
pub const ARIZONA_MICD_TIME_500US: u32 = 2;
pub const ARIZONA_MICD_TIME_1MS: u32 = 3;
pub const ARIZONA_MICD_TIME_2MS: u32 = 4;
pub const ARIZONA_MICD_TIME_4MS: u32 = 5;
pub const ARIZONA_MICD_TIME_8MS: u32 = 6;
pub const ARIZONA_MICD_TIME_16MS: u32 = 7;
pub const ARIZONA_MICD_TIME_32MS: u32 = 8;
pub const ARIZONA_MICD_TIME_64MS: u32 = 9;
pub const ARIZONA_MICD_TIME_128MS: u32 = 10;
pub const ARIZONA_MICD_TIME_256MS: u32 = 11;
pub const ARIZONA_MICD_TIME_512MS: u32 = 12;

pub const ARIZONA_ACCDET_MODE_MIC: u32 = 0;
pub const ARIZONA_ACCDET_MODE_HPL: u32 = 1;
pub const ARIZONA_ACCDET_MODE_HPR: u32 = 2;
pub const ARIZONA_ACCDET_MODE_HPM: u32 = 4;
pub const ARIZONA_ACCDET_MODE_ADC: u32 = 7;

pub const ARIZONA_GPSW_OPEN: u32 = 0;
pub const ARIZONA_GPSW_CLOSED: u32 = 1;
pub const ARIZONA_GPSW_CLAMP_ENABLED: u32 = 2;
pub const ARIZONA_GPSW_CLAMP_DISABLED: u32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
