/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * es8311.c -- es8311 ALSA SoC audio driver
 *
 * Copyright (C) 2024 Matteo Martelli <matteomartelli3@gmail.com>
 *
 * Author: Matteo Martelli <matteomartelli3@gmail.com>
 */

/* C header dependency: <linux/bitops.h> for BIT() and GENMASK(). */
pub const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

pub const fn GENMASK(h: u32, l: u32) -> u32 {
    u32::MAX.wrapping_shl(l) & u32::MAX.wrapping_shr(31 - h)
}

pub const ES8311_RESET: u32 = 0x00;
pub const ES8311_RESET_CSM_ON: u32 = BIT(7);
pub const ES8311_RESET_MSC: u32 = BIT(6);
pub const ES8311_RESET_RST_MASK: u32 = GENMASK(4, 0);

/* Clock Manager Registers */
pub const ES8311_CLKMGR1: u32 = 0x01;
pub const ES8311_CLKMGR1_MCLK_SEL: u32 = BIT(7);
pub const ES8311_CLKMGR1_MCLK_ON: u32 = BIT(5);
pub const ES8311_CLKMGR1_BCLK_ON: u32 = BIT(4);
pub const ES8311_CLKMGR1_CLKADC_ON_SHIFT: u32 = 3;
pub const ES8311_CLKMGR1_CLKDAC_ON_SHIFT: u32 = 2;
pub const ES8311_CLKMGR1_ANACLKADC_ON_SHIFT: u32 = 1;
pub const ES8311_CLKMGR1_ANACLKDAC_ON_SHIFT: u32 = 0;
pub const ES8311_CLKMGR2: u32 = 0x02;
pub const ES8311_CLKMGR2_DIV_PRE_MASK: u32 = GENMASK(7, 5);
pub const ES8311_CLKMGR2_DIV_PRE_SHIFT: u32 = 5;
pub const ES8311_CLKMGR2_DIV_PRE_MAX: u32 = 0x07;
pub const ES8311_CLKMGR2_MULT_PRE_MASK: u32 = GENMASK(4, 3);
pub const ES8311_CLKMGR2_MULT_PRE_SHIFT: u32 = 3;
pub const ES8311_CLKMGR3: u32 = 0x03;
pub const ES8311_CLKMGR4: u32 = 0x04;
pub const ES8311_CLKMGR5: u32 = 0x05;
pub const ES8311_CLKMGR5_ADC_DIV_MASK: u32 = GENMASK(7, 4);
pub const ES8311_CLKMGR5_ADC_DIV_SHIFT: u32 = 4;
pub const ES8311_CLKMGR5_DAC_DIV_MASK: u32 = GENMASK(3, 0);
pub const ES8311_CLKMGR5_DAC_DIV_SHIFT: u32 = 0;
pub const ES8311_CLKMGR6: u32 = 0x06;
pub const ES8311_CLKMGR6_BCLK_INV: u32 = BIT(5);
pub const ES8311_CLKMGR6_DIV_BCLK_MASK: u32 = GENMASK(4, 0);
pub const ES8311_CLKMGR7: u32 = 0x07;
pub const ES8311_CLKMGR7_LRCLK_DIV_H_MASK: u32 = GENMASK(3, 0);
pub const ES8311_CLKMGR8: u32 = 0x08;
pub const ES8311_CLKMGR_LRCLK_DIV_MAX: u32 = 0x0FFF;

/* SDP Mode Registers */
pub const ES8311_SDP_IN: u32 = 0x09;
pub const ES8311_SDP_IN_SEL_SHIFT: u32 = 7;
pub const ES8311_SDP_OUT: u32 = 0x0A;
/* Following values are the same for both SPD_IN and SDP_OUT */
pub const ES8311_SDP_MUTE_SHIFT: u32 = 6;
pub const ES8311_SDP_LRP: u32 = BIT(5);
pub const ES8311_SDP_WL_MASK: u32 = GENMASK(4, 2);
pub const ES8311_SDP_WL_SHIFT: u32 = 2;
pub const ES8311_SDP_WL_24: u32 = 0x00;
pub const ES8311_SDP_WL_20: u32 = 0x01;
pub const ES8311_SDP_WL_18: u32 = 0x02;
pub const ES8311_SDP_WL_16: u32 = 0x03;
pub const ES8311_SDP_WL_32: u32 = 0x04;
pub const ES8311_SDP_FMT_MASK: u32 = GENMASK(1, 0);
pub const ES8311_SDP_FMT_I2S: u32 = 0x00;
pub const ES8311_SDP_FMT_LEFT_J: u32 = 0x01;
pub const ES8311_SDP_FMT_DSP: u32 = 0x03;

/* System registers */
pub const ES8311_SYS1: u32 = 0x0B;
pub const ES8311_SYS2: u32 = 0x0C;
pub const ES8311_SYS3: u32 = 0x0D;
pub const ES8311_SYS3_PDN_ANA_SHIFT: u32 = 7;
pub const ES8311_SYS3_PDN_IBIASGEN_SHIFT: u32 = 6;
pub const ES8311_SYS3_PDN_ADCBIASGEN_SHIFT: u32 = 5;
pub const ES8311_SYS3_PDN_ADCVREFGEN_SHIFT: u32 = 4;
pub const ES8311_SYS3_PDN_DACVREFGEN_SHIFT: u32 = 3;
pub const ES8311_SYS3_PDN_VREF_SHIFT: u32 = 2;
pub const ES8311_SYS3_PDN_VMIDSEL_MASK: u32 = GENMASK(1, 0);
pub const ES8311_SYS3_PDN_VMIDSEL_POWER_DOWN: u32 = 0;
pub const ES8311_SYS3_PDN_VMIDSEL_STARTUP_NORMAL_SPEED: u32 = 1;
pub const ES8311_SYS3_PDN_VMIDSEL_NORMAL_OPERATION: u32 = 2;
pub const ES8311_SYS3_PDN_VMIDSEL_STARTUP_FAST_SPEED: u32 = 3;
pub const ES8311_SYS4: u32 = 0x0E;
pub const ES8311_SYS4_PDN_PGA_SHIFT: u32 = 6;
pub const ES8311_SYS4_PDN_MOD_SHIFT: u32 = 5;
pub const ES8311_SYS5: u32 = 0x0F;
pub const ES8311_SYS6: u32 = 0x10;
pub const ES8311_SYS7: u32 = 0x11;
pub const ES8311_SYS8: u32 = 0x12;
pub const ES8311_SYS8_PDN_DAC_SHIFT: u32 = 1;
pub const ES8311_SYS9: u32 = 0x13;
pub const ES8311_SYS9_HPSW_SHIFT: u32 = 4;
pub const ES8311_SYS10: u32 = 0x14;
pub const ES8311_SYS10_DMIC_ON_SHIFT: u32 = 6;
pub const ES8311_SYS10_LINESEL_SHIFT: u32 = 4;
pub const ES8311_SYS10_PGAGAIN_SHIFT: u32 = 0;
pub const ES8311_SYS10_PGAGAIN_MAX: u32 = 0x0A;

/* ADC Registers*/
pub const ES8311_ADC1: u32 = 0x15;
pub const ES8311_ADC1_RAMPRATE_SHIFT: u32 = 4;
pub const ES8311_ADC2: u32 = 0x16;
pub const ES8311_ADC2_INV_SHIFT: u32 = 4;
pub const ES8311_ADC2_SCALE_SHIFT: u32 = 0;
pub const ES8311_ADC2_SCALE_MAX: u32 = 0x07;
pub const ES8311_ADC3: u32 = 0x17;
pub const ES8311_ADC3_VOLUME_SHIFT: u32 = 0;
pub const ES8311_ADC3_VOLUME_MAX: u32 = 0xFF;
pub const ES8311_ADC4: u32 = 0x18;
pub const ES8311_ADC4_ALC_EN_SHIFT: u32 = 7;
pub const ES8311_ADC4_AUTOMUTE_EN_SHIFT: u32 = 6;
pub const ES8311_ADC4_ALC_WINSIZE_SHIFT: u32 = 0;
pub const ES8311_ADC5: u32 = 0x19;
pub const ES8311_ADC5_ALC_MAXLEVEL_SHIFT: u32 = 4;
pub const ES8311_ADC5_ALC_MAXLEVEL_MAX: u32 = 0x0F;
pub const ES8311_ADC5_ALC_MINLEVEL_SHIFT: u32 = 0;
pub const ES8311_ADC5_ALC_MINLEVEL_MAX: u32 = 0x0F;
pub const ES8311_ADC6: u32 = 0x1A;
pub const ES8311_ADC6_AUTOMUTE_WS_SHIFT: u32 = 4;
pub const ES8311_ADC6_AUTOMUTE_NG_SHIFT: u32 = 0;
pub const ES8311_ADC6_AUTOMUTE_NG_MAX: u32 = 0x0F;

pub const ES8311_ADC7: u32 = 0x1B;
pub const ES8311_ADC7_AUTOMUTE_VOL_SHIFT: u32 = 5;
pub const ES8311_ADC7_AUTOMUTE_VOL_MAX: u32 = 0x07;
pub const ES8311_ADC8: u32 = 0x1C;
pub const ES8311_ADC8_EQBYPASS_SHIFT: u32 = 6;
pub const ES8311_ADC8_HPF_SHIFT: u32 = 5;

/* DAC Registers */
pub const ES8311_DAC1: u32 = 0x31;
pub const ES8311_DAC1_DAC_DSMMUTE: u32 = BIT(6);
pub const ES8311_DAC1_DAC_DEMMUTE: u32 = BIT(5);
pub const ES8311_DAC2: u32 = 0x32;
pub const ES8311_DAC2_VOLUME_MAX: u32 = 0xFF;
pub const ES8311_DAC3: u32 = 0x33;
pub const ES8311_DAC4: u32 = 0x34;
pub const ES8311_DAC4_DRC_EN_SHIFT: u32 = 7;
pub const ES8311_DAC4_DRC_WINSIZE_SHIFT: u32 = 0;
pub const ES8311_DAC5: u32 = 0x35;
pub const ES8311_DAC5_DRC_MAXLEVEL_SHIFT: u32 = 4;
pub const ES8311_DAC5_DRC_MAXLEVEL_MAX: u32 = 0x0F;
pub const ES8311_DAC5_DRC_MINLEVEL_SHIFT: u32 = 0;
pub const ES8311_DAC5_DRC_MINLEVEL_MAX: u32 = 0x0F;
pub const ES8311_DAC6: u32 = 0x37;
pub const ES8311_DAC6_RAMPRATE_SHIFT: u32 = 4;
pub const ES8311_DAC6_EQBYPASS_SHIFT: u32 = 3;

/* GPIO Registers */
pub const ES8311_GPIO: u32 = 0x44;
pub const ES8311_GPIO_ADC2DAC_SEL_SHIFT: u32 = 7;
pub const ES8311_GPIO_ADCDAT_SEL_SHIFT: u32 = 4;

/* Chip Info Registers */
pub const ES8311_CHIPID1: u32 = 0xFD; /* 0x83 */
pub const ES8311_CHIPID2: u32 = 0xFE; /* 0x11 */
pub const ES8311_CHIPVER: u32 = 0xFF;

pub const ES8311_REG_MAX: u32 = 0xFF;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
