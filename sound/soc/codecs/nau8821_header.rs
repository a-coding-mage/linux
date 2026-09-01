/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * NAU88L21 ALSA SoC audio driver
 *
 * Copyright 2021 Nuvoton Technology Corp.
 * Author: John Hsu <kchsu0@nuvoton.com>
 * Co-author: Seven Lee <wtli@nuvoton.com>
 */

// C header guard and include directives are intentionally omitted.

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    // External kernel type; size/layout supplied by the translated dependency.
    _unused: [u8; 0],
}

/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * NAU88L21 ALSA SoC audio driver
 *
 * Copyright 2021 Nuvoton Technology Corp.
 * Author: John Hsu <kchsu0@nuvoton.com>
 * Co-author: Seven Lee <wtli@nuvoton.com>
 */


pub const NAU8821_R00_RESET: u32 = 0x00;
pub const NAU8821_R01_ENA_CTRL: u32 = 0x01;
pub const NAU8821_R03_CLK_DIVIDER: u32 = 0x03;
pub const NAU8821_R04_FLL1: u32 = 0x04;
pub const NAU8821_R05_FLL2: u32 = 0x05;
pub const NAU8821_R06_FLL3: u32 = 0x06;
pub const NAU8821_R07_FLL4: u32 = 0x07;
pub const NAU8821_R08_FLL5: u32 = 0x08;
pub const NAU8821_R09_FLL6: u32 = 0x09;
pub const NAU8821_R0A_FLL7: u32 = 0x0a;
pub const NAU8821_R0B_FLL8: u32 = 0x0b;
pub const NAU8821_R0D_JACK_DET_CTRL: u32 = 0x0d;
pub const NAU8821_R0F_INTERRUPT_MASK: u32 = 0x0f;
pub const NAU8821_R10_IRQ_STATUS: u32 = 0x10;
pub const NAU8821_R11_INT_CLR_KEY_STATUS: u32 = 0x11;
pub const NAU8821_R12_INTERRUPT_DIS_CTRL: u32 = 0x12;
pub const NAU8821_R13_DMIC_CTRL: u32 = 0x13;
pub const NAU8821_R1A_GPIO12_CTRL: u32 = 0x1a;
pub const NAU8821_R1B_TDM_CTRL: u32 = 0x1b;
pub const NAU8821_R1C_I2S_PCM_CTRL1: u32 = 0x1c;
pub const NAU8821_R1D_I2S_PCM_CTRL2: u32 = 0x1d;
pub const NAU8821_R1E_LEFT_TIME_SLOT: u32 = 0x1e;
pub const NAU8821_R1F_RIGHT_TIME_SLOT: u32 = 0x1f;
pub const NAU8821_R21_BIQ0_COF1: u32 = 0x21;
pub const NAU8821_R22_BIQ0_COF2: u32 = 0x22;
pub const NAU8821_R23_BIQ0_COF3: u32 = 0x23;
pub const NAU8821_R24_BIQ0_COF4: u32 = 0x24;
pub const NAU8821_R25_BIQ0_COF5: u32 = 0x25;
pub const NAU8821_R26_BIQ0_COF6: u32 = 0x26;
pub const NAU8821_R27_BIQ0_COF7: u32 = 0x27;
pub const NAU8821_R28_BIQ0_COF8: u32 = 0x28;
pub const NAU8821_R29_BIQ0_COF9: u32 = 0x29;
pub const NAU8821_R2A_BIQ0_COF10: u32 = 0x2a;
pub const NAU8821_R2B_ADC_RATE: u32 = 0x2b;
pub const NAU8821_R2C_DAC_CTRL1: u32 = 0x2c;
pub const NAU8821_R2D_DAC_CTRL2: u32 = 0x2d;
pub const NAU8821_R2F_DAC_DGAIN_CTRL: u32 = 0x2f;
pub const NAU8821_R30_ADC_DGAIN_CTRL: u32 = 0x30;
pub const NAU8821_R31_MUTE_CTRL: u32 = 0x31;
pub const NAU8821_R32_HSVOL_CTRL: u32 = 0x32;
pub const NAU8821_R34_DACR_CTRL: u32 = 0x34;
pub const NAU8821_R35_ADC_DGAIN_CTRL1: u32 = 0x35;
pub const NAU8821_R36_ADC_DRC_KNEE_IP12: u32 = 0x36;
pub const NAU8821_R37_ADC_DRC_KNEE_IP34: u32 = 0x37;
pub const NAU8821_R38_ADC_DRC_SLOPES: u32 = 0x38;
pub const NAU8821_R39_ADC_DRC_ATKDCY: u32 = 0x39;
pub const NAU8821_R3A_DAC_DRC_KNEE_IP12: u32 = 0x3a;
pub const NAU8821_R3B_DAC_DRC_KNEE_IP34: u32 = 0x3b;
pub const NAU8821_R3C_DAC_DRC_SLOPES: u32 = 0x3c;
pub const NAU8821_R3D_DAC_DRC_ATKDCY: u32 = 0x3d;
pub const NAU8821_R41_BIQ1_COF1: u32 = 0x41;
pub const NAU8821_R42_BIQ1_COF2: u32 = 0x42;
pub const NAU8821_R43_BIQ1_COF3: u32 = 0x43;
pub const NAU8821_R44_BIQ1_COF4: u32 = 0x44;
pub const NAU8821_R45_BIQ1_COF5: u32 = 0x45;
pub const NAU8821_R46_BIQ1_COF6: u32 = 0x46;
pub const NAU8821_R47_BIQ1_COF7: u32 = 0x47;
pub const NAU8821_R48_BIQ1_COF8: u32 = 0x48;
pub const NAU8821_R49_BIQ1_COF9: u32 = 0x49;
pub const NAU8821_R4A_BIQ1_COF10: u32 = 0x4a;
pub const NAU8821_R4B_CLASSG_CTRL: u32 = 0x4b;
pub const NAU8821_R4C_IMM_MODE_CTRL: u32 = 0x4c;
pub const NAU8821_R4D_IMM_RMS_L: u32 = 0x4d;
pub const NAU8821_R4E_FUSE_CTRL2: u32 = 0x4e;
pub const NAU8821_R4F_FUSE_CTRL3: u32 = 0x4f;
pub const NAU8821_R51_FUSE_CTRL1: u32 = 0x51;
pub const NAU8821_R53_OTPDOUT_1: u32 = 0x53;
pub const NAU8821_R54_OTPDOUT_2: u32 = 0x54;
pub const NAU8821_R55_MISC_CTRL: u32 = 0x55;
pub const NAU8821_R58_I2C_DEVICE_ID: u32 = 0x58;
pub const NAU8821_R59_SARDOUT_RAM_STATUS: u32 = 0x59;
pub const NAU8821_R5A_SOFTWARE_RST: u32 = 0x5a;
pub const NAU8821_R66_BIAS_ADJ: u32 = 0x66;
pub const NAU8821_R68_TRIM_SETTINGS: u32 = 0x68;
pub const NAU8821_R69_ANALOG_CONTROL_1: u32 = 0x69;
pub const NAU8821_R6A_ANALOG_CONTROL_2: u32 = 0x6a;
pub const NAU8821_R6B_PGA_MUTE: u32 = 0x6b;
pub const NAU8821_R71_ANALOG_ADC_1: u32 = 0x71;
pub const NAU8821_R72_ANALOG_ADC_2: u32 = 0x72;
pub const NAU8821_R73_RDAC: u32 = 0x73;
pub const NAU8821_R74_MIC_BIAS: u32 = 0x74;
pub const NAU8821_R76_BOOST: u32 = 0x76;
pub const NAU8821_R77_FEPGA: u32 = 0x77;
pub const NAU8821_R7E_PGA_GAIN: u32 = 0x7e;
pub const NAU8821_R7F_POWER_UP_CONTROL: u32 = 0x7f;
pub const NAU8821_R80_CHARGE_PUMP: u32 = 0x80;
pub const NAU8821_R81_CHARGE_PUMP_INPUT_READ: u32 = 0x81;
pub const NAU8821_R82_GENERAL_STATUS: u32 = 0x82;
pub const NAU8821_REG_MAX: u32 = NAU8821_R82_GENERAL_STATUS;
/* 16-bit control register address, and 16-bits control register data */
pub const NAU8821_REG_ADDR_LEN: u32 = 16;
pub const NAU8821_REG_DATA_LEN: u32 = 16;

/* ENA_CTRL (0x01) */
pub const NAU8821_CLK_DAC_INV_SFT: u32 = 14;
// Original C macro is self-referential: #define NAU8821_CLK_DAC_INV (0x1 << NAU8821_CLK_DAC_INV)
pub const NAU8821_CLK_DAC_INV: u32 = 0x1 << NAU8821_CLK_DAC_INV_SFT;
pub const NAU8821_EN_DACR_SFT: u32 = 11;
pub const NAU8821_EN_DACR: u32 = 0x1 << NAU8821_EN_DACR_SFT;
pub const NAU8821_EN_DACL_SFT: u32 = 10;
pub const NAU8821_EN_DACL: u32 = 0x1 << NAU8821_EN_DACL_SFT;
pub const NAU8821_EN_ADCR_SFT: u32 = 9;
pub const NAU8821_EN_ADCR: u32 = 0x1 << NAU8821_EN_ADCR_SFT;
pub const NAU8821_EN_ADCL_SFT: u32 = 8;
pub const NAU8821_EN_ADCL: u32 = 0x1 << NAU8821_EN_ADCL_SFT;
pub const NAU8821_EN_ADC_CLK_SFT: u32 = 7;
pub const NAU8821_EN_ADC_CLK: u32 = 0x1 << NAU8821_EN_ADC_CLK_SFT;
pub const NAU8821_EN_DAC_CLK_SFT: u32 = 6;
pub const NAU8821_EN_DAC_CLK: u32 = 0x1 << NAU8821_EN_DAC_CLK_SFT;
pub const NAU8821_EN_I2S_CLK_SFT: u32 = 4;
pub const NAU8821_EN_I2S_CLK: u32 = 0x1 << NAU8821_EN_I2S_CLK_SFT;
pub const NAU8821_EN_DRC_CLK_SFT: u32 = 0;
pub const NAU8821_EN_DRC_CLK: u32 = 0x1 << NAU8821_EN_DRC_CLK_SFT;

/* CLK_DIVIDER (0x03) */
pub const NAU8821_CLK_SRC_SFT: u32 = 15;
pub const NAU8821_CLK_SRC_MASK: u32 = 0x1 << NAU8821_CLK_SRC_SFT;
pub const NAU8821_CLK_SRC_VCO: u32 = 0x1 << NAU8821_CLK_SRC_SFT;
pub const NAU8821_CLK_SRC_MCLK: u32 = 0x0 << NAU8821_CLK_SRC_SFT;
pub const NAU8821_CLK_CODEC_SRC_SFT: u32 = 13;
pub const NAU8821_CLK_CODEC_SRC_MASK: u32 = 0x1 << NAU8821_CLK_CODEC_SRC_SFT;
pub const NAU8821_CLK_CODEC_SRC_VCO: u32 = 0x1 << NAU8821_CLK_CODEC_SRC_SFT;
pub const NAU8821_CLK_CODEC_SRC_MCLK: u32 = 0x0 << NAU8821_CLK_CODEC_SRC_SFT;
pub const NAU8821_CLK_ADC_SRC_SFT: u32 = 6;
pub const NAU8821_CLK_ADC_SRC_MASK: u32 = 0x3 << NAU8821_CLK_ADC_SRC_SFT;
pub const NAU8821_CLK_DAC_SRC_SFT: u32 = 4;
pub const NAU8821_CLK_DAC_SRC_MASK: u32 = 0x3 << NAU8821_CLK_DAC_SRC_SFT;
pub const NAU8821_CLK_MCLK_SRC_MASK: u32 = 0xf;

/* FLL1 (0x04) */
pub const NAU8821_ICTRL_LATCH_SFT: u32 = 10;
pub const NAU8821_ICTRL_LATCH_MASK: u32 = 0x7 << NAU8821_ICTRL_LATCH_SFT;
pub const NAU8821_FLL_RATIO_MASK: u32 = 0x7f;

/* FLL3 (0x06) */
pub const NAU8821_GAIN_ERR_SFT: u32 = 12;
pub const NAU8821_GAIN_ERR_MASK: u32 = 0xf << NAU8821_GAIN_ERR_SFT;
pub const NAU8821_FLL_CLK_SRC_SFT: u32 = 10;
pub const NAU8821_FLL_CLK_SRC_MASK: u32 = 0x3 << NAU8821_FLL_CLK_SRC_SFT;
pub const NAU8821_FLL_CLK_SRC_FS: u32 = 0x3 << NAU8821_FLL_CLK_SRC_SFT;
pub const NAU8821_FLL_CLK_SRC_BLK: u32 = 0x2 << NAU8821_FLL_CLK_SRC_SFT;
pub const NAU8821_FLL_CLK_SRC_MCLK: u32 = 0x0 << NAU8821_FLL_CLK_SRC_SFT;
pub const NAU8821_FLL_INTEGER_MASK: u32 = 0x3ff;

/* FLL4 (0x07) */
pub const NAU8821_HIGHBW_EN_SFT: u32 = 15;
pub const NAU8821_HIGHBW_EN: u32 = 0x1 << NAU8821_HIGHBW_EN_SFT;
pub const NAU8821_FLL_REF_DIV_SFT: u32 = 10;
pub const NAU8821_FLL_REF_DIV_MASK: u32 = 0x3 << NAU8821_FLL_REF_DIV_SFT;

/* FLL5 (0x08) */
pub const NAU8821_FLL_PDB_DAC_EN: u32 = 0x1 << 15;
pub const NAU8821_FLL_LOOP_FTR_EN: u32 = 0x1 << 14;
pub const NAU8821_FLL_CLK_SW_SFT: u32 = 13;
pub const NAU8821_FLL_CLK_SW_MASK: u32 = 0x1 << NAU8821_FLL_CLK_SW_SFT;
pub const NAU8821_FLL_CLK_SW_N2: u32 = 0x1 << NAU8821_FLL_CLK_SW_SFT;
pub const NAU8821_FLL_CLK_SW_REF: u32 = 0x0 << NAU8821_FLL_CLK_SW_SFT;
pub const NAU8821_FLL_FTR_SW_SFT: u32 = 12;
pub const NAU8821_FLL_FTR_SW_MASK: u32 = 0x1 << NAU8821_FLL_FTR_SW_SFT;
pub const NAU8821_FLL_FTR_SW_ACCU: u32 = 0x1 << NAU8821_FLL_FTR_SW_SFT;
pub const NAU8821_FLL_FTR_SW_FILTER: u32 = 0x0 << NAU8821_FLL_FTR_SW_SFT;

/* FLL6 (0x09) */
pub const NAU8821_DCO_EN: u32 = 0x1 << 15;
pub const NAU8821_SDM_EN: u32 = 0x1 << 14;
pub const NAU8821_CUTOFF500: u32 = 0x1 << 13;

/* FLL7 (0x0a) */
pub const NAU8821_FLL_FRACH_MASK: u32 = 0xff;

/* FLL8 (0x0b) */
pub const NAU8821_FLL_FRACL_MASK: u32 = 0xffff;

/* JACK_DET_CTRL (0x0d) */
/* 0 - open, 1 - short to GND */
pub const NAU8821_SPKR_DWN1R_SFT: u32 = 15;
pub const NAU8821_SPKR_DWN1R: u32 = 0x1 << NAU8821_SPKR_DWN1R_SFT;
pub const NAU8821_SPKR_DWN1L_SFT: u32 = 14;
pub const NAU8821_SPKR_DWN1L: u32 = 0x1 << NAU8821_SPKR_DWN1L_SFT;
pub const NAU8821_JACK_DET_RESTART: u32 = 0x1 << 9;
pub const NAU8821_JACK_DET_DB_BYPASS: u32 = 0x1 << 8;
pub const NAU8821_JACK_INSERT_DEBOUNCE_SFT: u32 = 5;
pub const NAU8821_JACK_INSERT_DEBOUNCE_MASK: u32 = 0x7 << NAU8821_JACK_INSERT_DEBOUNCE_SFT;
pub const NAU8821_JACK_EJECT_DEBOUNCE_SFT: u32 = 2;
pub const NAU8821_JACK_EJECT_DEBOUNCE_MASK: u32 = 0x7 << NAU8821_JACK_EJECT_DEBOUNCE_SFT;
/* 0 - active low, 1 - active high */
pub const NAU8821_JACK_POLARITY: u32 = 0x1 << 1;

/* INTERRUPT_MASK (0x0f) */
pub const NAU8821_IRQ_PIN_PULL_UP: u32 = 0x1 << 14;
pub const NAU8821_IRQ_PIN_PULL_EN: u32 = 0x1 << 13;
pub const NAU8821_IRQ_OUTPUT_EN: u32 = 0x1 << 11;
pub const NAU8821_IRQ_RMS_EN: u32 = 0x1 << 8;
pub const NAU8821_IRQ_KEY_RELEASE_EN: u32 = 0x1 << 7;
pub const NAU8821_IRQ_KEY_PRESS_EN: u32 = 0x1 << 6;
pub const NAU8821_IRQ_MIC_DET_EN: u32 = 0x1 << 4;
pub const NAU8821_IRQ_EJECT_EN: u32 = 0x1 << 2;
pub const NAU8821_IRQ_INSERT_EN: u32 = 0x1;

/* IRQ_STATUS (0x10) */
pub const NAU8821_SHORT_CIRCUIT_IRQ: u32 = 0x1 << 9;
pub const NAU8821_IMPEDANCE_MEAS_IRQ: u32 = 0x1 << 8;
pub const NAU8821_KEY_IRQ_SFT: u32 = 6;
pub const NAU8821_KEY_IRQ_MASK: u32 = 0x3 << NAU8821_KEY_IRQ_SFT;
pub const NAU8821_KEY_RELEASE_IRQ: u32 = 0x2 << NAU8821_KEY_IRQ_SFT;
pub const NAU8821_KEY_SHORT_PRESS_IRQ: u32 = 0x1 << NAU8821_KEY_IRQ_SFT;
pub const NAU8821_MIC_DETECT_IRQ: u32 = 0x1 << 4;
pub const NAU8821_JACK_EJECT_IRQ_MASK: u32 = 0x3 << 2;
pub const NAU8821_JACK_EJECT_DETECTED: u32 = 0x1 << 2;
pub const NAU8821_JACK_INSERT_IRQ_MASK: u32 = 0x3;
pub const NAU8821_JACK_INSERT_DETECTED: u32 = 0x1;

/* INTERRUPT_DIS_CTRL (0x12) */
pub const NAU8821_IRQ_KEY_RELEASE_DIS: u32 = 0x1 << 7;
pub const NAU8821_IRQ_KEY_PRESS_DIS: u32 = 0x1 << 6;
pub const NAU8821_IRQ_MIC_DIS: u32 = 0x1 << 4;
pub const NAU8821_IRQ_EJECT_DIS: u32 = 0x1 << 2;
pub const NAU8821_IRQ_INSERT_DIS: u32 = 0x1;

/* DMIC_CTRL (0x13) */
pub const NAU8821_DMIC_DS_SFT: u32 = 7;
pub const NAU8821_DMIC_DS_MASK: u32 = 0x1 << NAU8821_DMIC_DS_SFT;
pub const NAU8821_DMIC_DS_HIGH: u32 = 0x1 << NAU8821_DMIC_DS_SFT;
pub const NAU8821_DMIC_DS_LOW: u32 = 0x0 << NAU8821_DMIC_DS_SFT;
pub const NAU8821_DMIC_SRC_SFT: u32 = 1;
pub const NAU8821_DMIC_SRC_MASK: u32 = 0x3 << NAU8821_DMIC_SRC_SFT;
pub const NAU8821_CLK_DMIC_SRC: u32 = 0x2 << NAU8821_DMIC_SRC_SFT;
pub const NAU8821_DMIC_EN_SFT: u32 = 0;
pub const NAU8821_DMIC_SLEW_SFT: u32 = 8;
pub const NAU8821_DMIC_SLEW_MASK: u32 = 0x7 << NAU8821_DMIC_SLEW_SFT;

/* GPIO12_CTRL (0x1a) */
/* 0 - pull down, 1 - pull up */
pub const NAU8821_JKDET_PULL_UP: u32 = 0x1 << 11;
/* 0 - enable pull, 1 - disable */
pub const NAU8821_JKDET_PULL_EN: u32 = 0x1 << 9;
/* 0 - enable input, 1 - enable output */
pub const NAU8821_JKDET_OUTPUT_EN: u32 = 0x1 << 8;

/* TDM_CTRL (0x1b) */
pub const NAU8821_TDM_EN_SFT: u32 = 15;
pub const NAU8821_TDM_EN: u32 = 0x1 << NAU8821_TDM_EN_SFT;
pub const NAU8821_ADCPHS_SFT: u32 = 13;
pub const NAU8821_DACL_CH_SFT: u32 = 7;
pub const NAU8821_DACL_CH_MASK: u32 = 0x7 << NAU8821_DACL_CH_SFT;
pub const NAU8821_DACR_CH_SFT: u32 = 4;
pub const NAU8821_DACR_CH_MASK: u32 = 0x7 << NAU8821_DACR_CH_SFT;
pub const NAU8821_ADCL_CH_SFT: u32 = 2;
pub const NAU8821_ADCL_CH_MASK: u32 = 0x3 << NAU8821_ADCL_CH_SFT;
pub const NAU8821_ADCR_CH_SFT: u32 = 0;
pub const NAU8821_ADCR_CH_MASK: u32 = 0x3;

/* I2S_PCM_CTRL1 (0x1c) */
pub const NAU8821_I2S_BP_SFT: u32 = 7;
pub const NAU8821_I2S_BP_MASK: u32 = 0x1 << NAU8821_I2S_BP_SFT;
pub const NAU8821_I2S_BP_INV: u32 = 0x1 << NAU8821_I2S_BP_SFT;
pub const NAU8821_I2S_PCMB_SFT: u32 = 6;
pub const NAU8821_I2S_PCMB_MASK: u32 = 0x1 << NAU8821_I2S_PCMB_SFT;
pub const NAU8821_I2S_PCMB_EN: u32 = 0x1 << NAU8821_I2S_PCMB_SFT;
pub const NAU8821_I2S_DL_SFT: u32 = 2;
pub const NAU8821_I2S_DL_MASK: u32 = 0x3 << NAU8821_I2S_DL_SFT;
pub const NAU8821_I2S_DL_32: u32 = 0x3 << NAU8821_I2S_DL_SFT;
pub const NAU8821_I2S_DL_24: u32 = 0x2 << NAU8821_I2S_DL_SFT;
pub const NAU8821_I2S_DL_20: u32 = 0x1 << NAU8821_I2S_DL_SFT;
pub const NAU8821_I2S_DL_16: u32 = 0x0 << NAU8821_I2S_DL_SFT;
pub const NAU8821_I2S_DF_MASK: u32 = 0x3;
pub const NAU8821_I2S_DF_PCM_AB: u32 = 0x3;
pub const NAU8821_I2S_DF_I2S: u32 = 0x2;
pub const NAU8821_I2S_DF_LEFT: u32 = 0x1;
pub const NAU8821_I2S_DF_RIGTH: u32 = 0x0;

/* I2S_PCM_CTRL2 (0x1d) */
pub const NAU8821_I2S_TRISTATE_SFT: u32 = 15;
pub const NAU8821_I2S_TRISTATE: u32 = 0x1 << NAU8821_I2S_TRISTATE_SFT;
pub const NAU8821_I2S_LRC_DIV_SFT: u32 = 12;
pub const NAU8821_I2S_LRC_DIV_MASK: u32 = 0x3 << NAU8821_I2S_LRC_DIV_SFT;
pub const NAU8821_I2S_MS_SFT: u32 = 3;
pub const NAU8821_I2S_MS_MASK: u32 = 0x1 << NAU8821_I2S_MS_SFT;
pub const NAU8821_I2S_MS_MASTER: u32 = 0x1 << NAU8821_I2S_MS_SFT;
pub const NAU8821_I2S_MS_SLAVE: u32 = 0x0 << NAU8821_I2S_MS_SFT;
pub const NAU8821_I2S_BLK_DIV_MASK: u32 = 0x7;

/* LEFT_TIME_SLOT (0x1e) */
pub const NAU8821_TSLOT_L_OFFSET_MASK: u32 = 0x3ff;
pub const NAU8821_DIS_FS_SHORT_DET: u32 = 0x1 << 13;

/* RIGHT_TIME_SLOT (0x1f) */
pub const NAU8821_TSLOT_R_OFFSET_MASK: u32 = 0x3ff;

/* BIQ0_COF10 (0x2a) */
pub const NAU8821_BIQ0_ADC_EN_SFT: u32 = 3;
pub const NAU8821_BIQ0_ADC_EN_EN: u32 = 0x1 << NAU8821_BIQ0_ADC_EN_SFT;

/* ADC_RATE (0x2b) */
pub const NAU8821_ADC_SYNC_DOWN_SFT: u32 = 0;
pub const NAU8821_ADC_SYNC_DOWN_MASK: u32 = 0x3;
pub const NAU8821_ADC_SYNC_DOWN_256: u32 = 0x3;
pub const NAU8821_ADC_SYNC_DOWN_128: u32 = 0x2;
pub const NAU8821_ADC_SYNC_DOWN_64: u32 = 0x1;
pub const NAU8821_ADC_SYNC_DOWN_32: u32 = 0x0;
pub const NAU8821_ADC_L_SRC_SFT: u32 = 15;
pub const NAU8821_ADC_L_SRC_EN: u32 = 0x1 << NAU8821_ADC_L_SRC_SFT;
pub const NAU8821_ADC_R_SRC_SFT: u32 = 14;
pub const NAU8821_ADC_R_SRC_EN: u32 = 0x1 << NAU8821_ADC_R_SRC_SFT;

/* DAC_CTRL1 (0x2c) */
pub const NAU8821_DAC_OVERSAMPLE_SFT: u32 = 0;
pub const NAU8821_DAC_OVERSAMPLE_MASK: u32 = 0x7;
pub const NAU8821_DAC_OVERSAMPLE_32: u32 = 0x4;
pub const NAU8821_DAC_OVERSAMPLE_128: u32 = 0x2;
pub const NAU8821_DAC_OVERSAMPLE_256: u32 = 0x1;
pub const NAU8821_DAC_OVERSAMPLE_64: u32 = 0x0;

/* DAC_DGAIN_CTRL (0x2f) */
pub const NAU8821_DAC1_TO_DAC0_ST_SFT: u32 = 8;
pub const NAU8821_DAC1_TO_DAC0_ST_MASK: u32 = 0xff << NAU8821_DAC1_TO_DAC0_ST_SFT;
pub const NAU8821_DAC0_TO_DAC1_ST_SFT: u32 = 0;
pub const NAU8821_DAC0_TO_DAC1_ST_MASK: u32 = 0xff;

/* MUTE_CTRL (0x31) */
pub const NAU8821_DAC_ZC_EN: u32 = 0x1 << 12;
pub const NAU8821_DAC_SOFT_MUTE: u32 = 0x1 << 9;
pub const NAU8821_ADC_ZC_EN: u32 = 0x1 << 2;
pub const NAU8821_ADC_SOFT_MUTE: u32 = 0x1 << 1;

/* HSVOL_CTRL (0x32) */
pub const NAU8821_HP_MUTE: u32 = 0x1 << 15;
pub const NAU8821_HP_MUTE_AUTO: u32 = 0x1 << 14;
pub const NAU8821_HPL_MUTE: u32 = 0x1 << 13;
pub const NAU8821_HPR_MUTE: u32 = 0x1 << 12;
pub const NAU8821_HPL_VOL_SFT: u32 = 4;
pub const NAU8821_HPL_VOL_MASK: u32 = 0x3 << NAU8821_HPL_VOL_SFT;
pub const NAU8821_HPR_VOL_SFT: u32 = 0;
pub const NAU8821_HPR_VOL_MASK: u32 = 0x3 << NAU8821_HPR_VOL_SFT;

/* DACR_CTRL (0x34) */
pub const NAU8821_DACR_CH_VOL_SFT: u32 = 8;
pub const NAU8821_DACR_CH_VOL_MASK: u32 = 0xff << NAU8821_DACR_CH_VOL_SFT;
pub const NAU8821_DACL_CH_VOL_SFT: u32 = 0;
pub const NAU8821_DACL_CH_VOL_MASK: u32 = 0xff;

/* ADC_DGAIN_CTRL1 (0x35) */
pub const NAU8821_ADCR_CH_VOL_SFT: u32 = 8;
pub const NAU8821_ADCR_CH_VOL_MASK: u32 = 0xff << NAU8821_ADCR_CH_VOL_SFT;
pub const NAU8821_ADCL_CH_VOL_SFT: u32 = 0;
pub const NAU8821_ADCL_CH_VOL_MASK: u32 = 0xff;

/* ADC_DRC_KNEE_IP12 (0x36) */
pub const NAU8821_DRC_ENA_ADC_SFT: u32 = 15;
pub const NAU8821_DRC_ENA_ADC_EN: u32 = 0x1 << NAU8821_DRC_ENA_ADC_SFT;

/* ADC_DRC_KNEE_IP34 (0x37) */
pub const NAU8821_DRC_KNEE4_IP_ADC_SFT: u32 = 8;
pub const NAU8821_DRC_KNEE4_IP_ADC_MASK: u32 = 0xff << NAU8821_DRC_KNEE4_IP_ADC_SFT;
pub const NAU8821_DRC_KNEE3_IP_ADC_SFT: u32 = 0;
pub const NAU8821_DRC_KNEE3_IP_ADC_MASK: u32 = 0xff;

/* ADC_DRC_SLOPES (0x38) */
pub const NAU8821_DRC_NG_SLP_ADC_SFT: u32 = 12;
pub const NAU8821_DRC_EXP_SLP_ADC_SFT: u32 = 9;
pub const NAU8821_DRC_CMP2_SLP_ADC_SFT: u32 = 6;
pub const NAU8821_DRC_CMP1_SLP_ADC_SFT: u32 = 3;
pub const NAU8821_DRC_LMT_SLP_ADC_SFT: u32 = 0;

/* ADC_DRC_ATKDCY (0x39) */
pub const NAU8821_DRC_PK_COEF1_ADC_SFT: u32 = 12;
pub const NAU8821_DRC_PK_COEF2_ADC_SFT: u32 = 8;
pub const NAU8821_DRC_ATK_ADC_SFT: u32 = 4;
pub const NAU8821_DRC_DCY_ADC_SFT: u32 = 0;

/* BIQ1_COF10 (0x4a) */
pub const NAU8821_BIQ1_DAC_EN_SFT: u32 = 3;
pub const NAU8821_BIQ1_DAC_EN_EN: u32 = 0x1 << NAU8821_BIQ1_DAC_EN_SFT;

/* CLASSG_CTRL (0x4b) */
pub const NAU8821_CLASSG_TIMER_SFT: u32 = 8;
pub const NAU8821_CLASSG_TIMER_MASK: u32 = 0x3f << NAU8821_CLASSG_TIMER_SFT;
pub const NAU8821_CLASSG_TIMER_64MS: u32 = 0x20 << NAU8821_CLASSG_TIMER_SFT;
pub const NAU8821_CLASSG_TIMER_32MS: u32 = 0x10 << NAU8821_CLASSG_TIMER_SFT;
pub const NAU8821_CLASSG_TIMER_16MS: u32 = 0x8 << NAU8821_CLASSG_TIMER_SFT;
pub const NAU8821_CLASSG_TIMER_8MS: u32 = 0x4 << NAU8821_CLASSG_TIMER_SFT;
pub const NAU8821_CLASSG_TIMER_2MS: u32 = 0x2 << NAU8821_CLASSG_TIMER_SFT;
pub const NAU8821_CLASSG_TIMER_1MS: u32 = 0x1 << NAU8821_CLASSG_TIMER_SFT;
pub const NAU8821_CLASSG_RDAC_EN_SFT: u32 = 2;
pub const NAU8821_CLASSG_RDAC_EN: u32 = 0x1 << NAU8821_CLASSG_RDAC_EN_SFT;
pub const NAU8821_CLASSG_LDAC_EN_SFT: u32 = 1;
pub const NAU8821_CLASSG_LDAC_EN: u32 = 0x1 << NAU8821_CLASSG_LDAC_EN_SFT;
pub const NAU8821_CLASSG_EN_SFT: u32 = 0;
pub const NAU8821_CLASSG_EN: u32 = 0x1;

/* IMM_MODE_CTRL (0x4c) */
pub const NAU8821_IMM_THD_SFT: u32 = 8;
pub const NAU8821_IMM_THD_MASK: u32 = 0x3f << NAU8821_IMM_THD_SFT;
pub const NAU8821_IMM_GEN_VOL_SFT: u32 = 6;
pub const NAU8821_IMM_GEN_VOL_MASK: u32 = 0x3 << NAU8821_IMM_GEN_VOL_SFT;
pub const NAU8821_IMM_CYC_SFT: u32 = 4;
pub const NAU8821_IMM_CYC_MASK: u32 = 0x3 << NAU8821_IMM_CYC_SFT;
pub const NAU8821_IMM_EN: u32 = 0x1 << 3;
pub const NAU8821_IMM_DAC_SRC_MASK: u32 = 0x3;

/* I2C_DEVICE_ID (0x58) */
pub const NAU8821_KEYDET: u32 = 0x1 << 7;
pub const NAU8821_MICDET: u32 = 0x1 << 6;
pub const NAU8821_SOFTWARE_ID_MASK: u32 = 0x3;

/* BIAS_ADJ (0x66) */
pub const NAU8821_BIAS_HP_IMP: u32 = 0x1 << 15;
pub const NAU8821_BIAS_TESTDAC_SFT: u32 = 8;
pub const NAU8821_BIAS_TESTDAC_EN: u32 = 0x3 << NAU8821_BIAS_TESTDAC_SFT;
pub const NAU8821_BIAS_TESTDACR_EN: u32 = 0x2 << NAU8821_BIAS_TESTDAC_SFT;
pub const NAU8821_BIAS_TESTDACL_EN: u32 = 0x1 << NAU8821_BIAS_TESTDAC_SFT;
pub const NAU8821_BIAS_VMID: u32 = 0x1 << 6;
pub const NAU8821_BIAS_VMID_SEL_SFT: u32 = 4;
pub const NAU8821_BIAS_VMID_SEL_MASK: u32 = 0x3 << NAU8821_BIAS_VMID_SEL_SFT;

/* ANALOG_CONTROL_1 (0x69) */
pub const NAU8821_JD_POL_SFT: u32 = 2;
pub const NAU8821_JD_POL_MASK: u32 = 0x1 << NAU8821_JD_POL_SFT;
pub const NAU8821_JD_POL_INV: u32 = 0x1 << NAU8821_JD_POL_SFT;
pub const NAU8821_JD_OUT_POL_SFT: u32 = 1;
pub const NAU8821_JD_OUT_POL_MASK: u32 = 0x1 << NAU8821_JD_OUT_POL_SFT;
pub const NAU8821_JD_OUT_POL_INV: u32 = 0x1 << NAU8821_JD_OUT_POL_SFT;
pub const NAU8821_JD_EN_SFT: u32 = 0;
pub const NAU8821_JD_EN: u32 = 0x1;

/* ANALOG_CONTROL_2 (0x6a) */
pub const NAU8821_HP_NON_CLASSG_CURRENT_2xADJ: u32 = 0x1 << 12;
pub const NAU8821_DAC_CAPACITOR_MSB: u32 = 0x1 << 1;
pub const NAU8821_DAC_CAPACITOR_LSB: u32 = 0x1;

/* MUTE_MIC_L_N (0x6b) */
pub const NAU8821_MUTE_MICNL_SFT: u32 = 5;
pub const NAU8821_MUTE_MICNL_EN: u32 = 0x1 << NAU8821_MUTE_MICNL_SFT;
pub const NAU8821_MUTE_MICNR_SFT: u32 = 4;
pub const NAU8821_MUTE_MICNR_EN: u32 = 0x1 << NAU8821_MUTE_MICNR_SFT;
pub const NAU8821_MUTE_MICRP_SFT: u32 = 2;
pub const NAU8821_MUTE_MICRP_EN: u32 = 0x1 << NAU8821_MUTE_MICRP_SFT;

/* ANALOG_ADC_1 (0x71) */
pub const NAU8821_MICDET_EN_SFT: u32 = 0;
pub const NAU8821_MICDET_MASK: u32 = 0x1;
pub const NAU8821_MICDET_DIS: u32 = 0x1;
pub const NAU8821_MICDET_EN: u32 = 0x0;

/* ANALOG_ADC_2 (0x72) */
pub const NAU8821_ADC_VREFSEL_SFT: u32 = 8;
pub const NAU8821_ADC_VREFSEL_MASK: u32 = 0x3 << NAU8821_ADC_VREFSEL_SFT;
pub const NAU8821_POWERUP_ADCL_SFT: u32 = 6;
pub const NAU8821_POWERUP_ADCL: u32 = 0x1 << NAU8821_POWERUP_ADCL_SFT;
pub const NAU8821_POWERUP_ADCR_SFT: u32 = 4;
pub const NAU8821_POWERUP_ADCR: u32 = 0x1 << NAU8821_POWERUP_ADCR_SFT;

/* RDAC (0x73) */
pub const NAU8821_DACR_EN_SFT: u32 = 13;
pub const NAU8821_DACR_EN: u32 = 0x3 << NAU8821_DACR_EN_SFT;
pub const NAU8821_DACL_EN_SFT: u32 = 12;
pub const NAU8821_DACL_EN: u32 = 0x3 << NAU8821_DACL_EN_SFT;
pub const NAU8821_DACR_CLK_EN_SFT: u32 = 9;
pub const NAU8821_DACR_CLK_EN: u32 = 0x3 << NAU8821_DACR_CLK_EN_SFT;
pub const NAU8821_DACL_CLK_EN_SFT: u32 = 8;
pub const NAU8821_DACL_CLK_EN: u32 = 0x3 << NAU8821_DACL_CLK_EN_SFT;
pub const NAU8821_DAC_CLK_DELAY_SFT: u32 = 4;
pub const NAU8821_DAC_CLK_DELAY_MASK: u32 = 0x7 << NAU8821_DAC_CLK_DELAY_SFT;
pub const NAU8821_DAC_VREF_SFT: u32 = 2;
pub const NAU8821_DAC_VREF_MASK: u32 = 0x3 << NAU8821_DAC_VREF_SFT;

/* MIC_BIAS (0x74) */
pub const NAU8821_MICBIAS_JKR2: u32 = 0x1 << 12;
pub const NAU8821_MICBIAS_LOWNOISE_SFT: u32 = 10;
pub const NAU8821_MICBIAS_LOWNOISE_EN: u32 = 0x1 << NAU8821_MICBIAS_LOWNOISE_SFT;
pub const NAU8821_MICBIAS_POWERUP_SFT: u32 = 8;
pub const NAU8821_MICBIAS_POWERUP_EN: u32 = 0x1 << NAU8821_MICBIAS_POWERUP_SFT;
pub const NAU8821_MICBIAS_VOLTAGE_SFT: u32 = 0;
pub const NAU8821_MICBIAS_VOLTAGE_MASK: u32 = 0x7;

/* BOOST (0x76) */
pub const NAU8821_PRECHARGE_DIS: u32 = 0x1 << 13;
pub const NAU8821_GLOBAL_BIAS_EN: u32 = 0x1 << 12;
pub const NAU8821_HP_BOOST_DISCHRG_SFT: u32 = 11;
pub const NAU8821_HP_BOOST_DISCHRG_EN: u32 = 0x1 << NAU8821_HP_BOOST_DISCHRG_SFT;
pub const NAU8821_HP_BOOST_DIS_SFT: u32 = 9;
pub const NAU8821_HP_BOOST_DIS: u32 = 0x1 << NAU8821_HP_BOOST_DIS_SFT;
pub const NAU8821_HP_BOOST_G_DIS: u32 = 0x1 << 8;
pub const NAU8821_SHORT_SHUTDOWN_EN: u32 = 0x1 << 6;

/* FEPGA (0x77) */
pub const NAU8821_ACDC_CTRL_SFT: u32 = 14;
pub const NAU8821_ACDC_CTRL_MASK: u32 = 0x3 << NAU8821_ACDC_CTRL_SFT;
pub const NAU8821_ACDC_VREF_MICP: u32 = 0x1 << NAU8821_ACDC_CTRL_SFT;
pub const NAU8821_ACDC_VREF_MICN: u32 = 0x2 << NAU8821_ACDC_CTRL_SFT;
pub const NAU8821_FEPGA_MODEL_SFT: u32 = 4;
pub const NAU8821_FEPGA_MODEL_MASK: u32 = 0xf << NAU8821_FEPGA_MODEL_SFT;
pub const NAU8821_FEPGA_MODEL_AAF: u32 = 0x1 << NAU8821_FEPGA_MODEL_SFT;
pub const NAU8821_FEPGA_MODEL_DIS: u32 = 0x2 << NAU8821_FEPGA_MODEL_SFT;
pub const NAU8821_FEPGA_MODEL_IMP12K: u32 = 0x8 << NAU8821_FEPGA_MODEL_SFT;
pub const NAU8821_FEPGA_MODER_SFT: u32 = 0;
pub const NAU8821_FEPGA_MODER_MASK: u32 = 0xf;
pub const NAU8821_FEPGA_MODER_AAF: u32 = 0x1;
pub const NAU8821_FEPGA_MODER_DIS: u32 = 0x2;
pub const NAU8821_FEPGA_MODER_IMP12K: u32 = 0x8;


/* PGA_GAIN (0x7e) */
pub const NAU8821_PGA_GAIN_L_SFT: u32 = 8;
pub const NAU8821_PGA_GAIN_L_MASK: u32 = 0x3f << NAU8821_PGA_GAIN_L_SFT;
pub const NAU8821_PGA_GAIN_R_SFT: u32 = 0;
pub const NAU8821_PGA_GAIN_R_MASK: u32 = 0x3f;

/* POWER_UP_CONTROL (0x7f) */
pub const NAU8821_PUP_PGA_L_SFT: u32 = 15;
pub const NAU8821_PUP_PGA_L: u32 = 0x1 << NAU8821_PUP_PGA_L_SFT;
pub const NAU8821_PUP_PGA_R_SFT: u32 = 14;
pub const NAU8821_PUP_PGA_R: u32 = 0x1 << NAU8821_PUP_PGA_R_SFT;
pub const NAU8821_PUP_INTEG_R_SFT: u32 = 5;
pub const NAU8821_PUP_INTEG_R: u32 = 0x1 << NAU8821_PUP_INTEG_R_SFT;
pub const NAU8821_PUP_INTEG_L_SFT: u32 = 4;
pub const NAU8821_PUP_INTEG_L: u32 = 0x1 << NAU8821_PUP_INTEG_L_SFT;
pub const NAU8821_PUP_DRV_INSTG_R_SFT: u32 = 3;
pub const NAU8821_PUP_DRV_INSTG_R: u32 = 0x1 << NAU8821_PUP_DRV_INSTG_R_SFT;
pub const NAU8821_PUP_DRV_INSTG_L_SFT: u32 = 2;
pub const NAU8821_PUP_DRV_INSTG_L: u32 = 0x1 << NAU8821_PUP_DRV_INSTG_L_SFT;
pub const NAU8821_PUP_MAIN_DRV_R_SFT: u32 = 1;
pub const NAU8821_PUP_MAIN_DRV_R: u32 = 0x1 << NAU8821_PUP_MAIN_DRV_R_SFT;
pub const NAU8821_PUP_MAIN_DRV_L_SFT: u32 = 0;
pub const NAU8821_PUP_MAIN_DRV_L: u32 = 0x1;

/* CHARGE_PUMP (0x80) */
pub const NAU8821_JAMNODCLOW: u32 = 0x1 << 10;
pub const NAU8821_POWER_DOWN_DACR_SFT: u32 = 9;
pub const NAU8821_POWER_DOWN_DACR: u32 = 0x1 << NAU8821_POWER_DOWN_DACR_SFT;
pub const NAU8821_POWER_DOWN_DACL_SFT: u32 = 8;
pub const NAU8821_POWER_DOWN_DACL: u32 = 0x1 << NAU8821_POWER_DOWN_DACL_SFT;
pub const NAU8821_CHANRGE_PUMP_EN_SFT: u32 = 5;
pub const NAU8821_CHANRGE_PUMP_EN: u32 = 0x1 << NAU8821_CHANRGE_PUMP_EN_SFT;

/* GENERAL_STATUS (0x82) */
pub const NAU8821_GPIO2_IN_SFT: u32 = 1;
pub const NAU8821_GPIO2_IN: u32 = 0x1 << NAU8821_GPIO2_IN_SFT;

pub const NUVOTON_CODEC_DAI: &str = "nau8821-hifi";

/* System Clock Source */

/* System Clock Source */
pub const NAU8821_CLK_DIS: u32 = 0;
pub const NAU8821_CLK_MCLK: u32 = 1;
pub const NAU8821_CLK_INTERNAL: u32 = 2;
pub const NAU8821_CLK_FLL_MCLK: u32 = 3;
pub const NAU8821_CLK_FLL_BLK: u32 = 4;
pub const NAU8821_CLK_FLL_FS: u32 = 5;


#[repr(C)]
pub struct nau8821 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub dapm: *mut snd_soc_dapm_context,
    pub jack: *mut snd_soc_jack,
    pub jdet_work: delayed_work,
    pub jdet_active: bool,
    pub irq: ::std::os::raw::c_int,
    pub clk_id: ::std::os::raw::c_int,
    pub micbias_voltage: ::std::os::raw::c_int,
    pub vref_impedance: ::std::os::raw::c_int,
    pub jkdet_enable: bool,
    pub jkdet_pull_enable: bool,
    pub jkdet_pull_up: bool,
    pub left_input_single_end: bool,
    pub jkdet_polarity: ::std::os::raw::c_int,
    pub jack_insert_debounce: ::std::os::raw::c_int,
    pub jack_eject_debounce: ::std::os::raw::c_int,
    pub fs: ::std::os::raw::c_int,
    pub dmic_clk_threshold: ::std::os::raw::c_int,
    pub dmic_slew_rate: ::std::os::raw::c_int,
    pub key_enable: ::std::os::raw::c_int,
    pub adc_delay: ::std::os::raw::c_int,
}


unsafe extern "C" {
    pub fn nau8821_enable_jack_detect(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
    ) -> ::std::os::raw::c_int;
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
