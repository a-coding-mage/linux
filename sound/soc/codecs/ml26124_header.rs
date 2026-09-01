/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2011 LAPIS Semiconductor Co., Ltd.
 */

/* Clock Control Register */
pub const ML26124_SMPLING_RATE: u32 = 0x00;
pub const ML26124_PLLNL: u32 = 0x02;
pub const ML26124_PLLNH: u32 = 0x04;
pub const ML26124_PLLML: u32 = 0x06;
pub const ML26124_PLLMH: u32 = 0x08;
pub const ML26124_PLLDIV: u32 = 0x0a;
pub const ML26124_CLK_EN: u32 = 0x0c;
pub const ML26124_CLK_CTL: u32 = 0x0e;

/* System Control Register */
pub const ML26124_SW_RST: u32 = 0x10;
pub const ML26124_REC_PLYBAK_RUN: u32 = 0x12;
pub const ML26124_MIC_TIM: u32 = 0x14;

/* Power Mnagement Register */
pub const ML26124_PW_REF_PW_MNG: u32 = 0x20;
pub const ML26124_PW_IN_PW_MNG: u32 = 0x22;
pub const ML26124_PW_DAC_PW_MNG: u32 = 0x24;
pub const ML26124_PW_SPAMP_PW_MNG: u32 = 0x26;
pub const ML26124_PW_LOUT_PW_MNG: u32 = 0x28;
pub const ML26124_PW_VOUT_PW_MNG: u32 = 0x2a;
pub const ML26124_PW_ZCCMP_PW_MNG: u32 = 0x2e;

/* Analog Reference Control Register */
pub const ML26124_PW_MICBIAS_VOL: u32 = 0x30;

/* Input/Output Amplifier Control Register */
pub const ML26124_PW_MIC_IN_VOL: u32 = 0x32;
pub const ML26124_PW_MIC_BOST_VOL: u32 = 0x38;
pub const ML26124_PW_SPK_AMP_VOL: u32 = 0x3a;
pub const ML26124_PW_AMP_VOL_FUNC: u32 = 0x48;
pub const ML26124_PW_AMP_VOL_FADE: u32 = 0x4a;

/* Analog Path Control Register */
pub const ML26124_SPK_AMP_OUT: u32 = 0x54;
pub const ML26124_MIC_IF_CTL: u32 = 0x5a;
pub const ML26124_MIC_SELECT: u32 = 0xe8;

/* Audio Interface Control Register */
pub const ML26124_SAI_TRANS_CTL: u32 = 0x60;
pub const ML26124_SAI_RCV_CTL: u32 = 0x62;
pub const ML26124_SAI_MODE_SEL: u32 = 0x64;

/* DSP Control Register */
pub const ML26124_FILTER_EN: u32 = 0x66;
pub const ML26124_DVOL_CTL: u32 = 0x68;
pub const ML26124_MIXER_VOL_CTL: u32 = 0x6a;
pub const ML26124_RECORD_DIG_VOL: u32 = 0x6c;
pub const ML26124_PLBAK_DIG_VOL: u32 = 0x70;
pub const ML26124_DIGI_BOOST_VOL: u32 = 0x72;
pub const ML26124_EQ_GAIN_BRAND0: u32 = 0x74;
pub const ML26124_EQ_GAIN_BRAND1: u32 = 0x76;
pub const ML26124_EQ_GAIN_BRAND2: u32 = 0x78;
pub const ML26124_EQ_GAIN_BRAND3: u32 = 0x7a;
pub const ML26124_EQ_GAIN_BRAND4: u32 = 0x7c;
pub const ML26124_HPF2_CUTOFF: u32 = 0x7e;
pub const ML26124_EQBRAND0_F0L: u32 = 0x80;
pub const ML26124_EQBRAND0_F0H: u32 = 0x82;
pub const ML26124_EQBRAND0_F1L: u32 = 0x84;
pub const ML26124_EQBRAND0_F1H: u32 = 0x86;
pub const ML26124_EQBRAND1_F0L: u32 = 0x88;
pub const ML26124_EQBRAND1_F0H: u32 = 0x8a;
pub const ML26124_EQBRAND1_F1L: u32 = 0x8c;
pub const ML26124_EQBRAND1_F1H: u32 = 0x8e;
pub const ML26124_EQBRAND2_F0L: u32 = 0x90;
pub const ML26124_EQBRAND2_F0H: u32 = 0x92;
pub const ML26124_EQBRAND2_F1L: u32 = 0x94;
pub const ML26124_EQBRAND2_F1H: u32 = 0x96;
pub const ML26124_EQBRAND3_F0L: u32 = 0x98;
pub const ML26124_EQBRAND3_F0H: u32 = 0x9a;
pub const ML26124_EQBRAND3_F1L: u32 = 0x9c;
pub const ML26124_EQBRAND3_F1H: u32 = 0x9e;
pub const ML26124_EQBRAND4_F0L: u32 = 0xa0;
pub const ML26124_EQBRAND4_F0H: u32 = 0xa2;
pub const ML26124_EQBRAND4_F1L: u32 = 0xa4;
pub const ML26124_EQBRAND4_F1H: u32 = 0xa6;

/* ALC Control Register */
pub const ML26124_ALC_MODE: u32 = 0xb0;
pub const ML26124_ALC_ATTACK_TIM: u32 = 0xb2;
pub const ML26124_ALC_DECAY_TIM: u32 = 0xb4;
pub const ML26124_ALC_HOLD_TIM: u32 = 0xb6;
pub const ML26124_ALC_TARGET_LEV: u32 = 0xb8;
pub const ML26124_ALC_MAXMIN_GAIN: u32 = 0xba;
pub const ML26124_NOIS_GATE_THRSH: u32 = 0xbc;
pub const ML26124_ALC_ZERO_TIMOUT: u32 = 0xbe;

/* Playback Limiter Control Register */
pub const ML26124_PL_ATTACKTIME: u32 = 0xc0;
pub const ML26124_PL_DECAYTIME: u32 = 0xc2;
pub const ML26124_PL_TARGETTIME: u32 = 0xc4;
pub const ML26124_PL_MAXMIN_GAIN: u32 = 0xc6;
pub const ML26124_PLYBAK_BOST_VOL: u32 = 0xc8;
pub const ML26124_PL_0CROSS_TIMOUT: u32 = 0xca;

/* Video Amplifer Control Register */
pub const ML26124_VIDEO_AMP_GAIN_CTL: u32 = 0xd0;
pub const ML26124_VIDEO_AMP_SETUP1: u32 = 0xd2;
pub const ML26124_VIDEO_AMP_CTL2: u32 = 0xd4;

/* Clock select for machine driver */
pub const ML26124_USE_PLL: u32 = 0;
pub const ML26124_USE_MCLKI_256FS: u32 = 1;
pub const ML26124_USE_MCLKI_512FS: u32 = 2;
pub const ML26124_USE_MCLKI_1024FS: u32 = 3;

/* Register Mask */
pub const ML26124_R0_MASK: u32 = 0xf;
pub const ML26124_R2_MASK: u32 = 0xff;
pub const ML26124_R4_MASK: u32 = 0x1;
pub const ML26124_R6_MASK: u32 = 0xf;
pub const ML26124_R8_MASK: u32 = 0x3f;
pub const ML26124_Ra_MASK: u32 = 0x1f;
pub const ML26124_Rc_MASK: u32 = 0x1f;
pub const ML26124_Re_MASK: u32 = 0x7;
pub const ML26124_R10_MASK: u32 = 0x1;
pub const ML26124_R12_MASK: u32 = 0x17;
pub const ML26124_R14_MASK: u32 = 0x3f;
pub const ML26124_R20_MASK: u32 = 0x47;
pub const ML26124_R22_MASK: u32 = 0xa;
pub const ML26124_R24_MASK: u32 = 0x2;
pub const ML26124_R26_MASK: u32 = 0x1f;
pub const ML26124_R28_MASK: u32 = 0x2;
pub const ML26124_R2a_MASK: u32 = 0x2;
pub const ML26124_R2e_MASK: u32 = 0x2;
pub const ML26124_R30_MASK: u32 = 0x7;
pub const ML26124_R32_MASK: u32 = 0x3f;
pub const ML26124_R38_MASK: u32 = 0x38;
pub const ML26124_R3a_MASK: u32 = 0x3f;
pub const ML26124_R48_MASK: u32 = 0x3;
pub const ML26124_R4a_MASK: u32 = 0x7;
pub const ML26124_R54_MASK: u32 = 0x2a;
pub const ML26124_R5a_MASK: u32 = 0x3;
pub const ML26124_Re8_MASK: u32 = 0x3;
pub const ML26124_R60_MASK: u32 = 0xff;
pub const ML26124_R62_MASK: u32 = 0xff;
pub const ML26124_R64_MASK: u32 = 0x1;
pub const ML26124_R66_MASK: u32 = 0xff;
pub const ML26124_R68_MASK: u32 = 0x3b;
pub const ML26124_R6a_MASK: u32 = 0xf3;
pub const ML26124_R6c_MASK: u32 = 0xff;
pub const ML26124_R70_MASK: u32 = 0xff;

pub const ML26124_MCLKEN: u32 = 1 << 0;
pub const ML26124_PLLEN: u32 = 1 << 1;
pub const ML26124_PLLOE: u32 = 1 << 2;
pub const ML26124_MCLKOE: u32 = 1 << 3;

pub const ML26124_BLT_ALL_ON: u32 = 0x1f;
pub const ML26124_BLT_PREAMP_ON: u32 = 0x13;

pub const ML26124_MICBEN_ON: u32 = 1 << 2;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ml26124_regs {
    ML26124_MCLK = 0,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ml26124_clk_in {
    ML26124_USE_PLLOUT = 0,
    ML26124_USE_MCLKI,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
