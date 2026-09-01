// SPDX-License-Identifier: GPL-2.0-only
/*
 * max98095.h -- MAX98095 ALSA SoC Audio driver
 *
 * Copyright 2011 Maxim Integrated Products
 */

/*
 * MAX98095 Registers Definition
 */

pub const M98095_000_HOST_DATA: u32 = 0x00;
pub const M98095_001_HOST_INT_STS: u32 = 0x01;
pub const M98095_002_HOST_RSP_STS: u32 = 0x02;
pub const M98095_003_HOST_CMD_STS: u32 = 0x03;
pub const M98095_004_CODEC_STS: u32 = 0x04;
pub const M98095_005_DAI1_ALC_STS: u32 = 0x05;
pub const M98095_006_DAI2_ALC_STS: u32 = 0x06;
pub const M98095_007_JACK_AUTO_STS: u32 = 0x07;
pub const M98095_008_JACK_MANUAL_STS: u32 = 0x08;
pub const M98095_009_JACK_VBAT_STS: u32 = 0x09;
pub const M98095_00A_ACC_ADC_STS: u32 = 0x0A;
pub const M98095_00B_MIC_NG_AGC_STS: u32 = 0x0B;
pub const M98095_00C_SPK_L_VOLT_STS: u32 = 0x0C;
pub const M98095_00D_SPK_R_VOLT_STS: u32 = 0x0D;
pub const M98095_00E_TEMP_SENSOR_STS: u32 = 0x0E;
pub const M98095_00F_HOST_CFG: u32 = 0x0F;
pub const M98095_010_HOST_INT_CFG: u32 = 0x10;
pub const M98095_011_HOST_INT_EN: u32 = 0x11;
pub const M98095_012_CODEC_INT_EN: u32 = 0x12;
pub const M98095_013_JACK_INT_EN: u32 = 0x13;
pub const M98095_014_JACK_INT_EN: u32 = 0x14;
pub const M98095_015_DEC: u32 = 0x15;
pub const M98095_016_RESERVED: u32 = 0x16;
pub const M98095_017_RESERVED: u32 = 0x17;
pub const M98095_018_KEYCODE3: u32 = 0x18;
pub const M98095_019_KEYCODE2: u32 = 0x19;
pub const M98095_01A_KEYCODE1: u32 = 0x1A;
pub const M98095_01B_KEYCODE0: u32 = 0x1B;
pub const M98095_01C_OEMCODE1: u32 = 0x1C;
pub const M98095_01D_OEMCODE0: u32 = 0x1D;
pub const M98095_01E_XCFG1: u32 = 0x1E;
pub const M98095_01F_XCFG2: u32 = 0x1F;
pub const M98095_020_XCFG3: u32 = 0x20;
pub const M98095_021_XCFG4: u32 = 0x21;
pub const M98095_022_XCFG5: u32 = 0x22;
pub const M98095_023_XCFG6: u32 = 0x23;
pub const M98095_024_XGPIO: u32 = 0x24;
pub const M98095_025_XCLKCFG: u32 = 0x25;
pub const M98095_026_SYS_CLK: u32 = 0x26;
pub const M98095_027_DAI1_CLKMODE: u32 = 0x27;
pub const M98095_028_DAI1_CLKCFG_HI: u32 = 0x28;
pub const M98095_029_DAI1_CLKCFG_LO: u32 = 0x29;
pub const M98095_02A_DAI1_FORMAT: u32 = 0x2A;
pub const M98095_02B_DAI1_CLOCK: u32 = 0x2B;
pub const M98095_02C_DAI1_IOCFG: u32 = 0x2C;
pub const M98095_02D_DAI1_TDM: u32 = 0x2D;
pub const M98095_02E_DAI1_FILTERS: u32 = 0x2E;
pub const M98095_02F_DAI1_LVL1: u32 = 0x2F;
pub const M98095_030_DAI1_LVL2: u32 = 0x30;
pub const M98095_031_DAI2_CLKMODE: u32 = 0x31;
pub const M98095_032_DAI2_CLKCFG_HI: u32 = 0x32;
pub const M98095_033_DAI2_CLKCFG_LO: u32 = 0x33;
pub const M98095_034_DAI2_FORMAT: u32 = 0x34;
pub const M98095_035_DAI2_CLOCK: u32 = 0x35;
pub const M98095_036_DAI2_IOCFG: u32 = 0x36;
pub const M98095_037_DAI2_TDM: u32 = 0x37;
pub const M98095_038_DAI2_FILTERS: u32 = 0x38;
pub const M98095_039_DAI2_LVL1: u32 = 0x39;
pub const M98095_03A_DAI2_LVL2: u32 = 0x3A;
pub const M98095_03B_DAI3_CLKMODE: u32 = 0x3B;
pub const M98095_03C_DAI3_CLKCFG_HI: u32 = 0x3C;
pub const M98095_03D_DAI3_CLKCFG_LO: u32 = 0x3D;
pub const M98095_03E_DAI3_FORMAT: u32 = 0x3E;
pub const M98095_03F_DAI3_CLOCK: u32 = 0x3F;
pub const M98095_040_DAI3_IOCFG: u32 = 0x40;
pub const M98095_041_DAI3_TDM: u32 = 0x41;
pub const M98095_042_DAI3_FILTERS: u32 = 0x42;
pub const M98095_043_DAI3_LVL1: u32 = 0x43;
pub const M98095_044_DAI3_LVL2: u32 = 0x44;
pub const M98095_045_CFG_DSP: u32 = 0x45;
pub const M98095_046_DAC_CTRL1: u32 = 0x46;
pub const M98095_047_DAC_CTRL2: u32 = 0x47;
pub const M98095_048_MIX_DAC_LR: u32 = 0x48;
pub const M98095_049_MIX_DAC_M: u32 = 0x49;
pub const M98095_04A_MIX_ADC_LEFT: u32 = 0x4A;
pub const M98095_04B_MIX_ADC_RIGHT: u32 = 0x4B;
pub const M98095_04C_MIX_HP_LEFT: u32 = 0x4C;
pub const M98095_04D_MIX_HP_RIGHT: u32 = 0x4D;
pub const M98095_04E_CFG_HP: u32 = 0x4E;
pub const M98095_04F_MIX_RCV: u32 = 0x4F;
pub const M98095_050_MIX_SPK_LEFT: u32 = 0x50;
pub const M98095_051_MIX_SPK_RIGHT: u32 = 0x51;
pub const M98095_052_MIX_SPK_CFG: u32 = 0x52;
pub const M98095_053_MIX_LINEOUT1: u32 = 0x53;
pub const M98095_054_MIX_LINEOUT2: u32 = 0x54;
pub const M98095_055_MIX_LINEOUT_CFG: u32 = 0x55;
pub const M98095_056_LVL_SIDETONE_DAI12: u32 = 0x56;
pub const M98095_057_LVL_SIDETONE_DAI3: u32 = 0x57;
pub const M98095_058_LVL_DAI1_PLAY: u32 = 0x58;
pub const M98095_059_LVL_DAI1_EQ: u32 = 0x59;
pub const M98095_05A_LVL_DAI2_PLAY: u32 = 0x5A;
pub const M98095_05B_LVL_DAI2_EQ: u32 = 0x5B;
pub const M98095_05C_LVL_DAI3_PLAY: u32 = 0x5C;
pub const M98095_05D_LVL_ADC_L: u32 = 0x5D;
pub const M98095_05E_LVL_ADC_R: u32 = 0x5E;
pub const M98095_05F_LVL_MIC1: u32 = 0x5F;
pub const M98095_060_LVL_MIC2: u32 = 0x60;
pub const M98095_061_LVL_LINEIN: u32 = 0x61;
pub const M98095_062_LVL_LINEOUT1: u32 = 0x62;
pub const M98095_063_LVL_LINEOUT2: u32 = 0x63;
pub const M98095_064_LVL_HP_L: u32 = 0x64;
pub const M98095_065_LVL_HP_R: u32 = 0x65;
pub const M98095_066_LVL_RCV: u32 = 0x66;
pub const M98095_067_LVL_SPK_L: u32 = 0x67;
pub const M98095_068_LVL_SPK_R: u32 = 0x68;
pub const M98095_069_MICAGC_CFG: u32 = 0x69;
pub const M98095_06A_MICAGC_THRESH: u32 = 0x6A;
pub const M98095_06B_SPK_NOISEGATE: u32 = 0x6B;
pub const M98095_06C_DAI1_ALC1_TIME: u32 = 0x6C;
pub const M98095_06D_DAI1_ALC1_COMP: u32 = 0x6D;
pub const M98095_06E_DAI1_ALC1_EXPN: u32 = 0x6E;
pub const M98095_06F_DAI1_ALC1_GAIN: u32 = 0x6F;
pub const M98095_070_DAI1_ALC2_TIME: u32 = 0x70;
pub const M98095_071_DAI1_ALC2_COMP: u32 = 0x71;
pub const M98095_072_DAI1_ALC2_EXPN: u32 = 0x72;
pub const M98095_073_DAI1_ALC2_GAIN: u32 = 0x73;
pub const M98095_074_DAI1_ALC3_TIME: u32 = 0x74;
pub const M98095_075_DAI1_ALC3_COMP: u32 = 0x75;
pub const M98095_076_DAI1_ALC3_EXPN: u32 = 0x76;
pub const M98095_077_DAI1_ALC3_GAIN: u32 = 0x77;
pub const M98095_078_DAI2_ALC1_TIME: u32 = 0x78;
pub const M98095_079_DAI2_ALC1_COMP: u32 = 0x79;
pub const M98095_07A_DAI2_ALC1_EXPN: u32 = 0x7A;
pub const M98095_07B_DAI2_ALC1_GAIN: u32 = 0x7B;
pub const M98095_07C_DAI2_ALC2_TIME: u32 = 0x7C;
pub const M98095_07D_DAI2_ALC2_COMP: u32 = 0x7D;
pub const M98095_07E_DAI2_ALC2_EXPN: u32 = 0x7E;
pub const M98095_07F_DAI2_ALC2_GAIN: u32 = 0x7F;
pub const M98095_080_DAI2_ALC3_TIME: u32 = 0x80;
pub const M98095_081_DAI2_ALC3_COMP: u32 = 0x81;
pub const M98095_082_DAI2_ALC3_EXPN: u32 = 0x82;
pub const M98095_083_DAI2_ALC3_GAIN: u32 = 0x83;
pub const M98095_084_HP_NOISE_GATE: u32 = 0x84;
pub const M98095_085_AUX_ADC: u32 = 0x85;
pub const M98095_086_CFG_LINE: u32 = 0x86;
pub const M98095_087_CFG_MIC: u32 = 0x87;
pub const M98095_088_CFG_LEVEL: u32 = 0x88;
pub const M98095_089_JACK_DET_AUTO: u32 = 0x89;
pub const M98095_08A_JACK_DET_MANUAL: u32 = 0x8A;
pub const M98095_08B_JACK_KEYSCAN_DBC: u32 = 0x8B;
pub const M98095_08C_JACK_KEYSCAN_DLY: u32 = 0x8C;
pub const M98095_08D_JACK_KEY_THRESH: u32 = 0x8D;
pub const M98095_08E_JACK_DC_SLEW: u32 = 0x8E;
pub const M98095_08F_JACK_TEST_CFG: u32 = 0x8F;
pub const M98095_090_PWR_EN_IN: u32 = 0x90;
pub const M98095_091_PWR_EN_OUT: u32 = 0x91;
pub const M98095_092_PWR_EN_OUT: u32 = 0x92;
pub const M98095_093_BIAS_CTRL: u32 = 0x93;
pub const M98095_094_PWR_DAC_21: u32 = 0x94;
pub const M98095_095_PWR_DAC_03: u32 = 0x95;
pub const M98095_096_PWR_DAC_CK: u32 = 0x96;
pub const M98095_097_PWR_SYS: u32 = 0x97;

pub const M98095_0FF_REV_ID: u32 = 0xFF;

pub const M98095_REG_CNT: u32 = 0xFF + 1;
pub const M98095_REG_MAX_CACHED: u32 = 0x97;

/* MAX98095 Registers Bit Fields */

/* M98095_007_JACK_AUTO_STS */
pub const M98095_MIC_IN: u32 = 1 << 3;
pub const M98095_LO_IN: u32 = 1 << 5;
pub const M98095_HP_IN: u32 = 1 << 6;
pub const M98095_DDONE: u32 = 1 << 7;

/* M98095_00F_HOST_CFG */
pub const M98095_SEG: u32 = 1 << 0;
pub const M98095_XTEN: u32 = 1 << 1;
pub const M98095_MDLLEN: u32 = 1 << 2;

/* M98095_013_JACK_INT_EN */
pub const M98095_IMIC_IN: u32 = 1 << 3;
pub const M98095_ILO_IN: u32 = 1 << 5;
pub const M98095_IHP_IN: u32 = 1 << 6;
pub const M98095_IDDONE: u32 = 1 << 7;

/* M98095_027_DAI1_CLKMODE, M98095_031_DAI2_CLKMODE, M98095_03B_DAI3_CLKMODE */
pub const M98095_CLKMODE_MASK: u32 = 0xFF;

/* M98095_02A_DAI1_FORMAT, M98095_034_DAI2_FORMAT, M98095_03E_DAI3_FORMAT */
pub const M98095_DAI_MAS: u32 = 1 << 7;
pub const M98095_DAI_WCI: u32 = 1 << 6;
pub const M98095_DAI_BCI: u32 = 1 << 5;
pub const M98095_DAI_DLY: u32 = 1 << 4;
pub const M98095_DAI_TDM: u32 = 1 << 2;
pub const M98095_DAI_FSW: u32 = 1 << 1;
pub const M98095_DAI_WS: u32 = 1 << 0;

/* M98095_02B_DAI1_CLOCK, M98095_035_DAI2_CLOCK, M98095_03F_DAI3_CLOCK */
pub const M98095_DAI_BSEL64: u32 = 1 << 0;
pub const M98095_DAI_DOSR_DIV2: u32 = 0 << 5;
pub const M98095_DAI_DOSR_DIV4: u32 = 1 << 5;

/* M98095_02C_DAI1_IOCFG, M98095_036_DAI2_IOCFG, M98095_040_DAI3_IOCFG */
pub const M98095_S1NORMAL: u32 = 1 << 6;
pub const M98095_S2NORMAL: u32 = 2 << 6;
pub const M98095_S3NORMAL: u32 = 3 << 6;
pub const M98095_SDATA: u32 = 3 << 0;

/* M98095_02E_DAI1_FILTERS, M98095_038_DAI2_FILTERS, M98095_042_DAI3_FILTERS */
pub const M98095_DAI_DHF: u32 = 1 << 3;

/* M98095_045_DSP_CFG */
pub const M98095_DSPNORMAL: u32 = 5 << 4;

/* M98095_048_MIX_DAC_LR */
pub const M98095_DAI1L_TO_DACR: u32 = 1 << 7;
pub const M98095_DAI1R_TO_DACR: u32 = 1 << 6;
pub const M98095_DAI2M_TO_DACR: u32 = 1 << 5;
pub const M98095_DAI1L_TO_DACL: u32 = 1 << 3;
pub const M98095_DAI1R_TO_DACL: u32 = 1 << 2;
pub const M98095_DAI2M_TO_DACL: u32 = 1 << 1;
pub const M98095_DAI3M_TO_DACL: u32 = 1 << 0;

/* M98095_049_MIX_DAC_M */
pub const M98095_DAI1L_TO_DACM: u32 = 1 << 3;
pub const M98095_DAI1R_TO_DACM: u32 = 1 << 2;
pub const M98095_DAI2M_TO_DACM: u32 = 1 << 1;
pub const M98095_DAI3M_TO_DACM: u32 = 1 << 0;

/* M98095_04E_MIX_HP_CFG */
pub const M98095_HPNORMAL: u32 = 3 << 4;

/* M98095_05F_LVL_MIC1, M98095_060_LVL_MIC2 */
pub const M98095_MICPRE_MASK: u32 = 3 << 5;
pub const M98095_MICPRE_SHIFT: u32 = 5;

/* M98095_064_LVL_HP_L, M98095_065_LVL_HP_R */
pub const M98095_HP_MUTE: u32 = 1 << 7;

/* M98095_066_LVL_RCV */
pub const M98095_REC_MUTE: u32 = 1 << 7;

/* M98095_067_LVL_SPK_L, M98095_068_LVL_SPK_R */
pub const M98095_SP_MUTE: u32 = 1 << 7;

/* M98095_087_CFG_MIC */
pub const M98095_MICSEL_MASK: u32 = 3 << 0;
pub const M98095_DIGMIC_L: u32 = 1 << 2;
pub const M98095_DIGMIC_R: u32 = 1 << 3;
pub const M98095_DIGMIC2L: u32 = 1 << 4;
pub const M98095_DIGMIC2R: u32 = 1 << 5;

/* M98095_088_CFG_LEVEL */
pub const M98095_VSEN: u32 = 1 << 6;
pub const M98095_ZDEN: u32 = 1 << 5;
pub const M98095_BQ2EN: u32 = 1 << 3;
pub const M98095_BQ1EN: u32 = 1 << 2;
pub const M98095_EQ2EN: u32 = 1 << 1;
pub const M98095_EQ1EN: u32 = 1 << 0;

/* M98095_089_JACK_DET_AUTO */
pub const M98095_PIN5EN: u32 = 1 << 2;
pub const M98095_JDEN: u32 = 1 << 7;

/* M98095_090_PWR_EN_IN */
pub const M98095_INEN: u32 = 1 << 7;
pub const M98095_MB2EN: u32 = 1 << 3;
pub const M98095_MB1EN: u32 = 1 << 2;
pub const M98095_MBEN: u32 = 3 << 2;
pub const M98095_ADREN: u32 = 1 << 1;
pub const M98095_ADLEN: u32 = 1 << 0;

/* M98095_091_PWR_EN_OUT */
pub const M98095_HPLEN: u32 = 1 << 7;
pub const M98095_HPREN: u32 = 1 << 6;
pub const M98095_SPLEN: u32 = 1 << 5;
pub const M98095_SPREN: u32 = 1 << 4;
pub const M98095_RECEN: u32 = 1 << 3;
pub const M98095_DALEN: u32 = 1 << 1;
pub const M98095_DAREN: u32 = 1 << 0;

/* M98095_092_PWR_EN_OUT */
pub const M98095_SPK_FIXEDSPECTRUM: u32 = 0 << 4;
pub const M98095_SPK_SPREADSPECTRUM: u32 = 1 << 4;

/* M98095_097_PWR_SYS */
pub const M98095_SHDNRUN: u32 = 1 << 7;
pub const M98095_PERFMODE: u32 = 1 << 3;
pub const M98095_HPPLYBACK: u32 = 1 << 2;
pub const M98095_PWRSV8K: u32 = 1 << 1;
pub const M98095_PWRSV: u32 = 1 << 0;

pub const M98095_COEFS_PER_BAND: u32 = 5;

pub const fn M98095_BYTE1(w: u32) -> u32 {
    (w >> 8) & 0xff
}

pub const fn M98095_BYTE0(w: u32) -> u32 {
    w & 0xff
}

/* Equalizer filter coefficients */
pub const M98095_110_DAI1_EQ_BASE: u32 = 0x10;
pub const M98095_142_DAI2_EQ_BASE: u32 = 0x42;

/* Biquad filter coefficients */
pub const M98095_174_DAI1_BQ_BASE: u32 = 0x74;
pub const M98095_17E_DAI2_BQ_BASE: u32 = 0x7E;

/* Default Delay used in Slew Rate Calculation for Jack detection */
pub const M98095_DEFAULT_SLEW_DELAY: u32 = 0x18;

use core::ffi::c_int;

unsafe extern "C" {
    pub fn max98095_jack_detect(
        component: *mut snd_soc_component,
        hp_jack: *mut snd_soc_jack,
        mic_jack: *mut snd_soc_jack,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
