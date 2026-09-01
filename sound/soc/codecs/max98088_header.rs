/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * max98088.h -- MAX98088 ALSA SoC Audio driver
 *
 * Copyright 2010 Maxim Integrated Products
 */

/*
 * MAX98088 Registers Definition
 */
pub const M98088_REG_00_IRQ_STATUS: u32 = 0x00;
pub const M98088_REG_01_MIC_STATUS: u32 = 0x01;
pub const M98088_REG_02_JACK_STATUS: u32 = 0x02;
pub const M98088_REG_03_BATTERY_VOLTAGE: u32 = 0x03;
pub const M98088_REG_0F_IRQ_ENABLE: u32 = 0x0F;
pub const M98088_REG_10_SYS_CLK: u32 = 0x10;
pub const M98088_REG_11_DAI1_CLKMODE: u32 = 0x11;
pub const M98088_REG_12_DAI1_CLKCFG_HI: u32 = 0x12;
pub const M98088_REG_13_DAI1_CLKCFG_LO: u32 = 0x13;
pub const M98088_REG_14_DAI1_FORMAT: u32 = 0x14;
pub const M98088_REG_15_DAI1_CLOCK: u32 = 0x15;
pub const M98088_REG_16_DAI1_IOCFG: u32 = 0x16;
pub const M98088_REG_17_DAI1_TDM: u32 = 0x17;
pub const M98088_REG_18_DAI1_FILTERS: u32 = 0x18;
pub const M98088_REG_19_DAI2_CLKMODE: u32 = 0x19;
pub const M98088_REG_1A_DAI2_CLKCFG_HI: u32 = 0x1A;
pub const M98088_REG_1B_DAI2_CLKCFG_LO: u32 = 0x1B;
pub const M98088_REG_1C_DAI2_FORMAT: u32 = 0x1C;
pub const M98088_REG_1D_DAI2_CLOCK: u32 = 0x1D;
pub const M98088_REG_1E_DAI2_IOCFG: u32 = 0x1E;
pub const M98088_REG_1F_DAI2_TDM: u32 = 0x1F;
pub const M98088_REG_20_DAI2_FILTERS: u32 = 0x20;
pub const M98088_REG_21_SRC: u32 = 0x21;
pub const M98088_REG_22_MIX_DAC: u32 = 0x22;
pub const M98088_REG_23_MIX_ADC_LEFT: u32 = 0x23;
pub const M98088_REG_24_MIX_ADC_RIGHT: u32 = 0x24;
pub const M98088_REG_25_MIX_HP_LEFT: u32 = 0x25;
pub const M98088_REG_26_MIX_HP_RIGHT: u32 = 0x26;
pub const M98088_REG_27_MIX_HP_CNTL: u32 = 0x27;
pub const M98088_REG_28_MIX_REC_LEFT: u32 = 0x28;
pub const M98088_REG_29_MIX_REC_RIGHT: u32 = 0x29;
pub const M98088_REG_2A_MIC_REC_CNTL: u32 = 0x2A;
pub const M98088_REG_2B_MIX_SPK_LEFT: u32 = 0x2B;
pub const M98088_REG_2C_MIX_SPK_RIGHT: u32 = 0x2C;
pub const M98088_REG_2D_MIX_SPK_CNTL: u32 = 0x2D;
pub const M98088_REG_2E_LVL_SIDETONE: u32 = 0x2E;
pub const M98088_REG_2F_LVL_DAI1_PLAY: u32 = 0x2F;
pub const M98088_REG_30_LVL_DAI1_PLAY_EQ: u32 = 0x30;
pub const M98088_REG_31_LVL_DAI2_PLAY: u32 = 0x31;
pub const M98088_REG_32_LVL_DAI2_PLAY_EQ: u32 = 0x32;
pub const M98088_REG_33_LVL_ADC_L: u32 = 0x33;
pub const M98088_REG_34_LVL_ADC_R: u32 = 0x34;
pub const M98088_REG_35_LVL_MIC1: u32 = 0x35;
pub const M98088_REG_36_LVL_MIC2: u32 = 0x36;
pub const M98088_REG_37_LVL_INA: u32 = 0x37;
pub const M98088_REG_38_LVL_INB: u32 = 0x38;
pub const M98088_REG_39_LVL_HP_L: u32 = 0x39;
pub const M98088_REG_3A_LVL_HP_R: u32 = 0x3A;
pub const M98088_REG_3B_LVL_REC_L: u32 = 0x3B;
pub const M98088_REG_3C_LVL_REC_R: u32 = 0x3C;
pub const M98088_REG_3D_LVL_SPK_L: u32 = 0x3D;
pub const M98088_REG_3E_LVL_SPK_R: u32 = 0x3E;
pub const M98088_REG_3F_MICAGC_CFG: u32 = 0x3F;
pub const M98088_REG_40_MICAGC_THRESH: u32 = 0x40;
pub const M98088_REG_41_SPKDHP: u32 = 0x41;
pub const M98088_REG_42_SPKDHP_THRESH: u32 = 0x42;
pub const M98088_REG_43_SPKALC_COMP: u32 = 0x43;
pub const M98088_REG_44_PWRLMT_CFG: u32 = 0x44;
pub const M98088_REG_45_PWRLMT_TIME: u32 = 0x45;
pub const M98088_REG_46_THDLMT_CFG: u32 = 0x46;
pub const M98088_REG_47_CFG_AUDIO_IN: u32 = 0x47;
pub const M98088_REG_48_CFG_MIC: u32 = 0x48;
pub const M98088_REG_49_CFG_LEVEL: u32 = 0x49;
pub const M98088_REG_4A_CFG_BYPASS: u32 = 0x4A;
pub const M98088_REG_4B_CFG_JACKDET: u32 = 0x4B;
pub const M98088_REG_4C_PWR_EN_IN: u32 = 0x4C;
pub const M98088_REG_4D_PWR_EN_OUT: u32 = 0x4D;
pub const M98088_REG_4E_BIAS_CNTL: u32 = 0x4E;
pub const M98088_REG_4F_DAC_BIAS1: u32 = 0x4F;
pub const M98088_REG_50_DAC_BIAS2: u32 = 0x50;
pub const M98088_REG_51_PWR_SYS: u32 = 0x51;
pub const M98088_REG_52_DAI1_EQ_BASE: u32 = 0x52;
pub const M98088_REG_84_DAI2_EQ_BASE: u32 = 0x84;
pub const M98088_REG_B6_DAI1_BIQUAD_BASE: u32 = 0xB6;
pub const M98088_REG_C0_DAI2_BIQUAD_BASE: u32 = 0xC0;
pub const M98088_REG_FF_REV_ID: u32 = 0xFF;

pub const M98088_REG_CNT: u32 = 0xFF + 1;

/* MAX98088 Registers Bit Fields */

/* M98088_REG_11_DAI1_CLKMODE, M98088_REG_19_DAI2_CLKMODE */
pub const M98088_CLKMODE_MASK: u32 = 0xFF;

/* M98088_REG_14_DAI1_FORMAT, M98088_REG_1C_DAI2_FORMAT */
pub const M98088_DAI_MAS: u32 = 1 << 7;
pub const M98088_DAI_WCI: u32 = 1 << 6;
pub const M98088_DAI_BCI: u32 = 1 << 5;
pub const M98088_DAI_DLY: u32 = 1 << 4;
pub const M98088_DAI_TDM: u32 = 1 << 2;
pub const M98088_DAI_FSW: u32 = 1 << 1;
pub const M98088_DAI_WS: u32 = 1 << 0;

/* M98088_REG_15_DAI1_CLOCK, M98088_REG_1D_DAI2_CLOCK */
pub const M98088_DAI_BSEL64: u32 = 1 << 0;
pub const M98088_DAI_OSR64: u32 = 1 << 6;

/* M98088_REG_16_DAI1_IOCFG, M98088_REG_1E_DAI2_IOCFG */
pub const M98088_S1NORMAL: u32 = 1 << 6;
pub const M98088_S2NORMAL: u32 = 2 << 6;
pub const M98088_SDATA: u32 = 3 << 0;

/* M98088_REG_18_DAI1_FILTERS, M98088_REG_20_DAI2_FILTERS */
pub const M98088_DAI_DHF: u32 = 1 << 3;

/* M98088_REG_22_MIX_DAC */
pub const M98088_DAI1L_TO_DACL: u32 = 1 << 7;
pub const M98088_DAI1R_TO_DACL: u32 = 1 << 6;
pub const M98088_DAI2L_TO_DACL: u32 = 1 << 5;
pub const M98088_DAI2R_TO_DACL: u32 = 1 << 4;
pub const M98088_DAI1L_TO_DACR: u32 = 1 << 3;
pub const M98088_DAI1R_TO_DACR: u32 = 1 << 2;
pub const M98088_DAI2L_TO_DACR: u32 = 1 << 1;
pub const M98088_DAI2R_TO_DACR: u32 = 1 << 0;

/* M98088_REG_2A_MIC_REC_CNTL */
pub const M98088_REC_LINEMODE: u32 = 1 << 7;
pub const M98088_REC_LINEMODE_MASK: u32 = 1 << 7;

/* M98088_REG_2D_MIX_SPK_CNTL */
pub const M98088_MIX_SPKR_GAIN_MASK: u32 = 3 << 2;
pub const M98088_MIX_SPKR_GAIN_SHIFT: u32 = 2;
pub const M98088_MIX_SPKL_GAIN_MASK: u32 = 3 << 0;
pub const M98088_MIX_SPKL_GAIN_SHIFT: u32 = 0;

/* M98088_REG_2F_LVL_DAI1_PLAY, M98088_REG_31_LVL_DAI2_PLAY */
pub const M98088_DAI_MUTE: u32 = 1 << 7;
pub const M98088_DAI_MUTE_MASK: u32 = 1 << 7;
pub const M98088_DAI_VOICE_GAIN_MASK: u32 = 3 << 4;
pub const M98088_DAI_ATTENUATION_MASK: u32 = 0xF << 0;
pub const M98088_DAI_ATTENUATION_SHIFT: u32 = 0;

/* M98088_REG_35_LVL_MIC1, M98088_REG_36_LVL_MIC2 */
pub const M98088_MICPRE_MASK: u32 = 3 << 5;
pub const M98088_MICPRE_SHIFT: u32 = 5;

/* M98088_REG_3A_LVL_HP_R */
pub const M98088_HP_MUTE: u32 = 1 << 7;

/* M98088_REG_3C_LVL_REC_R */
pub const M98088_REC_MUTE: u32 = 1 << 7;

/* M98088_REG_3E_LVL_SPK_R */
pub const M98088_SP_MUTE: u32 = 1 << 7;

/* M98088_REG_48_CFG_MIC */
pub const M98088_EXTMIC_MASK: u32 = 3 << 0;
pub const M98088_DIGMIC_L: u32 = 1 << 5;
pub const M98088_DIGMIC_R: u32 = 1 << 4;

/* M98088_REG_49_CFG_LEVEL */
pub const M98088_VSEN: u32 = 1 << 6;
pub const M98088_ZDEN: u32 = 1 << 5;
pub const M98088_EQ2EN: u32 = 1 << 1;
pub const M98088_EQ1EN: u32 = 1 << 0;

/* M98088_REG_4C_PWR_EN_IN */
pub const M98088_INAEN: u32 = 1 << 7;
pub const M98088_INBEN: u32 = 1 << 6;
pub const M98088_MBEN: u32 = 1 << 3;
pub const M98088_ADLEN: u32 = 1 << 1;
pub const M98088_ADREN: u32 = 1 << 0;

/* M98088_REG_4D_PWR_EN_OUT */
pub const M98088_HPLEN: u32 = 1 << 7;
pub const M98088_HPREN: u32 = 1 << 6;
pub const M98088_HPEN: u32 = (1 << 7) | (1 << 6);
pub const M98088_SPLEN: u32 = 1 << 5;
pub const M98088_SPREN: u32 = 1 << 4;
pub const M98088_RECEN: u32 = 1 << 3;
pub const M98088_DALEN: u32 = 1 << 1;
pub const M98088_DAREN: u32 = 1 << 0;

/* M98088_REG_51_PWR_SYS */
pub const M98088_SHDNRUN: u32 = 1 << 7;
pub const M98088_PERFMODE: u32 = 1 << 3;
pub const M98088_HPPLYBACK: u32 = 1 << 2;
pub const M98088_PWRSV8K: u32 = 1 << 1;
pub const M98088_PWRSV: u32 = 1 << 0;

/* Line inputs */
pub const LINE_INA: u32 = 0;
pub const LINE_INB: u32 = 1;

pub const M98088_COEFS_PER_BAND: u32 = 5;

pub const fn M98088_BYTE1(w: u32) -> u32 {
    (w >> 8) & 0xff
}

pub const fn M98088_BYTE0(w: u32) -> u32 {
    w & 0xff
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
