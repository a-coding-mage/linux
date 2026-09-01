// SPDX-License-Identifier: GPL-2.0-only
/*
 * alc5623.h  --  alc562[123] ALSA Soc Audio driver
 *
 * Copyright 2008 Realtek Microelectronics
 * Copyright 2010 Arnaud Patard <arnaud.patard@rtp-net.org>
 *
 * Author: flove <flove@realtek.com>
 * Arnaud Patard <arnaud.patard@rtp-net.org>
 */

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

pub const ALC5623_RESET: u16 = 0x00;
/*				5621 5622 5623  */
/* speaker output vol		   2    2       */
/* line output vol                      4    2  */
/* HP output vol		   4    0    4  */
pub const ALC5623_SPK_OUT_VOL: u16 = 0x02;
pub const ALC5623_HP_OUT_VOL: u16 = 0x04;
pub const ALC5623_MONO_AUX_OUT_VOL: u16 = 0x06;
pub const ALC5623_AUXIN_VOL: u16 = 0x08;
pub const ALC5623_LINE_IN_VOL: u16 = 0x0A;
pub const ALC5623_STEREO_DAC_VOL: u16 = 0x0C;
pub const ALC5623_MIC_VOL: u16 = 0x0E;
pub const ALC5623_MIC_ROUTING_CTRL: u16 = 0x10;
pub const ALC5623_ADC_REC_GAIN: u16 = 0x12;
pub const ALC5623_ADC_REC_MIXER: u16 = 0x14;
pub const ALC5623_SOFT_VOL_CTRL_TIME: u16 = 0x16;
/* ALC5623_OUTPUT_MIXER_CTRL :			*/
/* same remark as for reg 2 line vs speaker	*/
pub const ALC5623_OUTPUT_MIXER_CTRL: u16 = 0x1C;
pub const ALC5623_MIC_CTRL: u16 = 0x22;

pub const ALC5623_DAI_CONTROL: u16 = 0x34;
pub const ALC5623_DAI_SDP_MASTER_MODE: u16 = 0 << 15;
pub const ALC5623_DAI_SDP_SLAVE_MODE: u16 = 1 << 15;
pub const ALC5623_DAI_I2S_PCM_MODE: u16 = 1 << 14;
pub const ALC5623_DAI_MAIN_I2S_BCLK_POL_CTRL: u16 = 1 << 7;
pub const ALC5623_DAI_ADC_DATA_L_R_SWAP: u16 = 1 << 5;
pub const ALC5623_DAI_DAC_DATA_L_R_SWAP: u16 = 1 << 4;
pub const ALC5623_DAI_I2S_DL_MASK: u16 = 3 << 2;
pub const ALC5623_DAI_I2S_DL_32: u16 = 3 << 2;
pub const ALC5623_DAI_I2S_DL_24: u16 = 2 << 2;
pub const ALC5623_DAI_I2S_DL_20: u16 = 1 << 2;
pub const ALC5623_DAI_I2S_DL_16: u16 = 0 << 2;
pub const ALC5623_DAI_I2S_DF_PCM: u16 = 3 << 0;
pub const ALC5623_DAI_I2S_DF_LEFT: u16 = 2 << 0;
pub const ALC5623_DAI_I2S_DF_RIGHT: u16 = 1 << 0;
pub const ALC5623_DAI_I2S_DF_I2S: u16 = 0 << 0;

pub const ALC5623_STEREO_AD_DA_CLK_CTRL: u16 = 0x36;
pub const ALC5623_COMPANDING_CTRL: u16 = 0x38;

pub const ALC5623_PWR_MANAG_ADD1: u16 = 0x3A;
pub const ALC5623_PWR_ADD1_MAIN_I2S_EN: u16 = 1 << 15;
pub const ALC5623_PWR_ADD1_ZC_DET_PD_EN: u16 = 1 << 14;
pub const ALC5623_PWR_ADD1_MIC1_BIAS_EN: u16 = 1 << 11;
pub const ALC5623_PWR_ADD1_SHORT_CURR_DET_EN: u16 = 1 << 10;
pub const ALC5623_PWR_ADD1_SOFTGEN_EN: u16 = 1 << 8; /* rsvd on 5622 */
pub const ALC5623_PWR_ADD1_DEPOP_BUF_HP: u16 = 1 << 6; /* rsvd on 5622 */
pub const ALC5623_PWR_ADD1_HP_OUT_AMP: u16 = 1 << 5;
pub const ALC5623_PWR_ADD1_HP_OUT_ENH_AMP: u16 = 1 << 4; /* rsvd on 5622 */
pub const ALC5623_PWR_ADD1_DEPOP_BUF_AUX: u16 = 1 << 2;
pub const ALC5623_PWR_ADD1_AUX_OUT_AMP: u16 = 1 << 1;
pub const ALC5623_PWR_ADD1_AUX_OUT_ENH_AMP: u16 = 1 << 0; /* rsvd on 5622 */

pub const ALC5623_PWR_MANAG_ADD2: u16 = 0x3C;
pub const ALC5623_PWR_ADD2_LINEOUT: u16 = 1 << 15; /* rt5623 */
pub const ALC5623_PWR_ADD2_CLASS_AB: u16 = 1 << 15; /* rt5621 */
pub const ALC5623_PWR_ADD2_CLASS_D: u16 = 1 << 14; /* rt5621 */
pub const ALC5623_PWR_ADD2_VREF: u16 = 1 << 13;
pub const ALC5623_PWR_ADD2_PLL: u16 = 1 << 12;
pub const ALC5623_PWR_ADD2_DAC_REF_CIR: u16 = 1 << 10;
pub const ALC5623_PWR_ADD2_L_DAC_CLK: u16 = 1 << 9;
pub const ALC5623_PWR_ADD2_R_DAC_CLK: u16 = 1 << 8;
pub const ALC5623_PWR_ADD2_L_ADC_CLK_GAIN: u16 = 1 << 7;
pub const ALC5623_PWR_ADD2_R_ADC_CLK_GAIN: u16 = 1 << 6;
pub const ALC5623_PWR_ADD2_L_HP_MIXER: u16 = 1 << 5;
pub const ALC5623_PWR_ADD2_R_HP_MIXER: u16 = 1 << 4;
pub const ALC5623_PWR_ADD2_SPK_MIXER: u16 = 1 << 3;
pub const ALC5623_PWR_ADD2_MONO_MIXER: u16 = 1 << 2;
pub const ALC5623_PWR_ADD2_L_ADC_REC_MIXER: u16 = 1 << 1;
pub const ALC5623_PWR_ADD2_R_ADC_REC_MIXER: u16 = 1 << 0;

pub const ALC5623_PWR_MANAG_ADD3: u16 = 0x3E;
pub const ALC5623_PWR_ADD3_MAIN_BIAS: u16 = 1 << 15;
pub const ALC5623_PWR_ADD3_AUXOUT_L_VOL_AMP: u16 = 1 << 14;
pub const ALC5623_PWR_ADD3_AUXOUT_R_VOL_AMP: u16 = 1 << 13;
pub const ALC5623_PWR_ADD3_SPK_OUT: u16 = 1 << 12;
pub const ALC5623_PWR_ADD3_HP_L_OUT_VOL: u16 = 1 << 10;
pub const ALC5623_PWR_ADD3_HP_R_OUT_VOL: u16 = 1 << 9;
pub const ALC5623_PWR_ADD3_LINEIN_L_VOL: u16 = 1 << 7;
pub const ALC5623_PWR_ADD3_LINEIN_R_VOL: u16 = 1 << 6;
pub const ALC5623_PWR_ADD3_AUXIN_L_VOL: u16 = 1 << 5;
pub const ALC5623_PWR_ADD3_AUXIN_R_VOL: u16 = 1 << 4;
pub const ALC5623_PWR_ADD3_MIC1_FUN_CTRL: u16 = 1 << 3;
pub const ALC5623_PWR_ADD3_MIC2_FUN_CTRL: u16 = 1 << 2;
pub const ALC5623_PWR_ADD3_MIC1_BOOST_AD: u16 = 1 << 1;
pub const ALC5623_PWR_ADD3_MIC2_BOOST_AD: u16 = 1 << 0;

pub const ALC5623_ADD_CTRL_REG: u16 = 0x40;

pub const ALC5623_GLOBAL_CLK_CTRL_REG: u16 = 0x42;
pub const ALC5623_GBL_CLK_SYS_SOUR_SEL_PLL: u16 = 1 << 15;
pub const ALC5623_GBL_CLK_SYS_SOUR_SEL_MCLK: u16 = 0 << 15;
pub const ALC5623_GBL_CLK_PLL_SOUR_SEL_BITCLK: u16 = 1 << 14;
pub const ALC5623_GBL_CLK_PLL_SOUR_SEL_MCLK: u16 = 0 << 14;
pub const ALC5623_GBL_CLK_PLL_DIV_RATIO_DIV8: u16 = 3 << 1;
pub const ALC5623_GBL_CLK_PLL_DIV_RATIO_DIV4: u16 = 2 << 1;
pub const ALC5623_GBL_CLK_PLL_DIV_RATIO_DIV2: u16 = 1 << 1;
pub const ALC5623_GBL_CLK_PLL_DIV_RATIO_DIV1: u16 = 0 << 1;
pub const ALC5623_GBL_CLK_PLL_PRE_DIV2: u16 = 1 << 0;
pub const ALC5623_GBL_CLK_PLL_PRE_DIV1: u16 = 0 << 0;

pub const ALC5623_PLL_CTRL: u16 = 0x44;
pub const fn ALC5623_PLL_CTRL_N_VAL(n: u32) -> u32 {
    ((n) & 0xff) << 8
}
pub const fn ALC5623_PLL_CTRL_K_VAL(k: u32) -> u32 {
    ((k) & 0x7) << 4
}
pub const fn ALC5623_PLL_CTRL_M_VAL(m: u32) -> u32 {
    (m) & 0xf
}

pub const ALC5623_GPIO_OUTPUT_PIN_CTRL: u16 = 0x4A;
pub const ALC5623_GPIO_PIN_CONFIG: u16 = 0x4C;
pub const ALC5623_GPIO_PIN_POLARITY: u16 = 0x4E;
pub const ALC5623_GPIO_PIN_STICKY: u16 = 0x50;
pub const ALC5623_GPIO_PIN_WAKEUP: u16 = 0x52;
pub const ALC5623_GPIO_PIN_STATUS: u16 = 0x54;
pub const ALC5623_GPIO_PIN_SHARING: u16 = 0x56;
pub const ALC5623_OVER_CURR_STATUS: u16 = 0x58;
pub const ALC5623_JACK_DET_CTRL: u16 = 0x5A;

pub const ALC5623_MISC_CTRL: u16 = 0x5E;
pub const ALC5623_MISC_DISABLE_FAST_VREG: u16 = 1 << 15;
pub const ALC5623_MISC_SPK_CLASS_AB_OC_PD: u16 = 1 << 13; /* 5621 */
pub const ALC5623_MISC_SPK_CLASS_AB_OC_DET: u16 = 1 << 12; /* 5621 */
pub const ALC5623_MISC_HP_DEPOP_MODE3_EN: u16 = 1 << 10;
pub const ALC5623_MISC_HP_DEPOP_MODE2_EN: u16 = 1 << 9;
pub const ALC5623_MISC_HP_DEPOP_MODE1_EN: u16 = 1 << 8;
pub const ALC5623_MISC_AUXOUT_DEPOP_MODE3_EN: u16 = 1 << 6;
pub const ALC5623_MISC_AUXOUT_DEPOP_MODE2_EN: u16 = 1 << 5;
pub const ALC5623_MISC_AUXOUT_DEPOP_MODE1_EN: u16 = 1 << 4;
pub const ALC5623_MISC_M_DAC_L_INPUT: u16 = 1 << 3;
pub const ALC5623_MISC_M_DAC_R_INPUT: u16 = 1 << 2;
pub const ALC5623_MISC_IRQOUT_INV_CTRL: u16 = 1 << 0;

pub const ALC5623_PSEDUEO_SPATIAL_CTRL: u16 = 0x60;
pub const ALC5623_EQ_CTRL: u16 = 0x62;
pub const ALC5623_EQ_MODE_ENABLE: u16 = 0x66;
pub const ALC5623_AVC_CTRL: u16 = 0x68;
pub const ALC5623_HID_CTRL_INDEX: u16 = 0x6A;
pub const ALC5623_HID_CTRL_DATA: u16 = 0x6C;
pub const ALC5623_VENDOR_ID1: u16 = 0x7C;
pub const ALC5623_VENDOR_ID2: u16 = 0x7E;

pub const ALC5623_PLL_FR_MCLK: u16 = 0;
pub const ALC5623_PLL_FR_BCK: u16 = 1;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
