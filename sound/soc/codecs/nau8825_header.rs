/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * NAU8825 ALSA SoC audio driver
 *
 * Copyright 2015 Google Inc.
 * Author: Anatol Pomozov <anatol.pomozov@chrominium.org>
 */

pub const NAU8825_REG_RESET: u32 = 0x00;
pub const NAU8825_REG_ENA_CTRL: u32 = 0x01;
pub const NAU8825_REG_IIC_ADDR_SET: u32 = 0x02;
pub const NAU8825_REG_CLK_DIVIDER: u32 = 0x03;
pub const NAU8825_REG_FLL1: u32 = 0x04;
pub const NAU8825_REG_FLL2: u32 = 0x05;
pub const NAU8825_REG_FLL3: u32 = 0x06;
pub const NAU8825_REG_FLL4: u32 = 0x07;
pub const NAU8825_REG_FLL5: u32 = 0x08;
pub const NAU8825_REG_FLL6: u32 = 0x09;
pub const NAU8825_REG_FLL_VCO_RSV: u32 = 0x0a;
pub const NAU8825_REG_HSD_CTRL: u32 = 0x0c;
pub const NAU8825_REG_JACK_DET_CTRL: u32 = 0x0d;
pub const NAU8825_REG_INTERRUPT_MASK: u32 = 0x0f;
pub const NAU8825_REG_IRQ_STATUS: u32 = 0x10;
pub const NAU8825_REG_INT_CLR_KEY_STATUS: u32 = 0x11;
pub const NAU8825_REG_INTERRUPT_DIS_CTRL: u32 = 0x12;
pub const NAU8825_REG_SAR_CTRL: u32 = 0x13;
pub const NAU8825_REG_KEYDET_CTRL: u32 = 0x14;
pub const NAU8825_REG_VDET_THRESHOLD_1: u32 = 0x15;
pub const NAU8825_REG_VDET_THRESHOLD_2: u32 = 0x16;
pub const NAU8825_REG_VDET_THRESHOLD_3: u32 = 0x17;
pub const NAU8825_REG_VDET_THRESHOLD_4: u32 = 0x18;
pub const NAU8825_REG_GPIO34_CTRL: u32 = 0x19;
pub const NAU8825_REG_GPIO12_CTRL: u32 = 0x1a;
pub const NAU8825_REG_TDM_CTRL: u32 = 0x1b;
pub const NAU8825_REG_I2S_PCM_CTRL1: u32 = 0x1c;
pub const NAU8825_REG_I2S_PCM_CTRL2: u32 = 0x1d;
pub const NAU8825_REG_LEFT_TIME_SLOT: u32 = 0x1e;
pub const NAU8825_REG_RIGHT_TIME_SLOT: u32 = 0x1f;
pub const NAU8825_REG_BIQ_CTRL: u32 = 0x20;
pub const NAU8825_REG_BIQ_COF1: u32 = 0x21;
pub const NAU8825_REG_BIQ_COF2: u32 = 0x22;
pub const NAU8825_REG_BIQ_COF3: u32 = 0x23;
pub const NAU8825_REG_BIQ_COF4: u32 = 0x24;
pub const NAU8825_REG_BIQ_COF5: u32 = 0x25;
pub const NAU8825_REG_BIQ_COF6: u32 = 0x26;
pub const NAU8825_REG_BIQ_COF7: u32 = 0x27;
pub const NAU8825_REG_BIQ_COF8: u32 = 0x28;
pub const NAU8825_REG_BIQ_COF9: u32 = 0x29;
pub const NAU8825_REG_BIQ_COF10: u32 = 0x2a;
pub const NAU8825_REG_ADC_RATE: u32 = 0x2b;
pub const NAU8825_REG_DAC_CTRL1: u32 = 0x2c;
pub const NAU8825_REG_DAC_CTRL2: u32 = 0x2d;
pub const NAU8825_REG_DAC_DGAIN_CTRL: u32 = 0x2f;
pub const NAU8825_REG_ADC_DGAIN_CTRL: u32 = 0x30;
pub const NAU8825_REG_MUTE_CTRL: u32 = 0x31;
pub const NAU8825_REG_HSVOL_CTRL: u32 = 0x32;
pub const NAU8825_REG_DACL_CTRL: u32 = 0x33;
pub const NAU8825_REG_DACR_CTRL: u32 = 0x34;
pub const NAU8825_REG_ADC_DRC_KNEE_IP12: u32 = 0x38;
pub const NAU8825_REG_ADC_DRC_KNEE_IP34: u32 = 0x39;
pub const NAU8825_REG_ADC_DRC_SLOPES: u32 = 0x3a;
pub const NAU8825_REG_ADC_DRC_ATKDCY: u32 = 0x3b;
pub const NAU8825_REG_DAC_DRC_KNEE_IP12: u32 = 0x45;
pub const NAU8825_REG_DAC_DRC_KNEE_IP34: u32 = 0x46;
pub const NAU8825_REG_DAC_DRC_SLOPES: u32 = 0x47;
pub const NAU8825_REG_DAC_DRC_ATKDCY: u32 = 0x48;
pub const NAU8825_REG_IMM_MODE_CTRL: u32 = 0x4c;
pub const NAU8825_REG_IMM_RMS_L: u32 = 0x4d;
pub const NAU8825_REG_IMM_RMS_R: u32 = 0x4e;
pub const NAU8825_REG_CLASSG_CTRL: u32 = 0x50;
pub const NAU8825_REG_OPT_EFUSE_CTRL: u32 = 0x51;
pub const NAU8825_REG_MISC_CTRL: u32 = 0x55;
pub const NAU8825_REG_I2C_DEVICE_ID: u32 = 0x58;
pub const NAU8825_REG_SARDOUT_RAM_STATUS: u32 = 0x59;
pub const NAU8825_REG_FLL2_LOWER: u32 = 0x5a;
pub const NAU8825_REG_FLL2_UPPER: u32 = 0x5b;
pub const NAU8825_REG_BIAS_ADJ: u32 = 0x66;
pub const NAU8825_REG_TRIM_SETTINGS: u32 = 0x68;
pub const NAU8825_REG_ANALOG_CONTROL_1: u32 = 0x69;
pub const NAU8825_REG_ANALOG_CONTROL_2: u32 = 0x6a;
pub const NAU8825_REG_ANALOG_ADC_1: u32 = 0x71;
pub const NAU8825_REG_ANALOG_ADC_2: u32 = 0x72;
pub const NAU8825_REG_RDAC: u32 = 0x73;
pub const NAU8825_REG_MIC_BIAS: u32 = 0x74;
pub const NAU8825_REG_BOOST: u32 = 0x76;
pub const NAU8825_REG_FEPGA: u32 = 0x77;
pub const NAU8825_REG_POWER_UP_CONTROL: u32 = 0x7f;
pub const NAU8825_REG_CHARGE_PUMP: u32 = 0x80;
pub const NAU8825_REG_CHARGE_PUMP_INPUT_READ: u32 = 0x81;
pub const NAU8825_REG_GENERAL_STATUS: u32 = 0x82;
pub const NAU8825_REG_MAX: u32 = NAU8825_REG_GENERAL_STATUS;
/* 16-bit control register address, and 16-bits control register data */
pub const NAU8825_REG_ADDR_LEN: u32 = 16;
pub const NAU8825_REG_DATA_LEN: u32 = 16;

/* ENA_CTRL (0x1) */
pub const NAU8825_ENABLE_DACR_SFT: u32 = 10;
pub const NAU8825_ENABLE_DACR: u32 = 1 << NAU8825_ENABLE_DACR_SFT;
pub const NAU8825_ENABLE_DACL_SFT: u32 = 9;
pub const NAU8825_ENABLE_DACL: u32 = 1 << NAU8825_ENABLE_DACL_SFT;
pub const NAU8825_ENABLE_ADC_SFT: u32 = 8;
pub const NAU8825_ENABLE_ADC: u32 = 1 << NAU8825_ENABLE_ADC_SFT;
pub const NAU8825_ENABLE_ADC_CLK_SFT: u32 = 7;
pub const NAU8825_ENABLE_ADC_CLK: u32 = 1 << NAU8825_ENABLE_ADC_CLK_SFT;
pub const NAU8825_ENABLE_DAC_CLK_SFT: u32 = 6;
pub const NAU8825_ENABLE_DAC_CLK: u32 = 1 << NAU8825_ENABLE_DAC_CLK_SFT;
pub const NAU8825_ENABLE_SAR_SFT: u32 = 1;

/* CLK_DIVIDER (0x3) */
pub const NAU8825_CLK_SRC_SFT: u32 = 15;
pub const NAU8825_CLK_SRC_MASK: u32 = 1 << NAU8825_CLK_SRC_SFT;
pub const NAU8825_CLK_SRC_VCO: u32 = 1 << NAU8825_CLK_SRC_SFT;
pub const NAU8825_CLK_SRC_MCLK: u32 = 0 << NAU8825_CLK_SRC_SFT;
pub const NAU8825_CLK_ADC_SRC_SFT: u32 = 6;
pub const NAU8825_CLK_ADC_SRC_MASK: u32 = 0x3 << NAU8825_CLK_ADC_SRC_SFT;
pub const NAU8825_CLK_DAC_SRC_SFT: u32 = 4;
pub const NAU8825_CLK_DAC_SRC_MASK: u32 = 0x3 << NAU8825_CLK_DAC_SRC_SFT;
pub const NAU8825_CLK_MCLK_SRC_MASK: u32 = 0xf << 0;

/* FLL1 (0x04) */
pub const NAU8825_ICTRL_LATCH_SFT: u32 = 10;
pub const NAU8825_ICTRL_LATCH_MASK: u32 = 0x7 << NAU8825_ICTRL_LATCH_SFT;
pub const NAU8825_FLL_RATIO_MASK: u32 = 0x7f << 0;

/* FLL3 (0x06) */
pub const NAU8825_GAIN_ERR_SFT: u32 = 12;
pub const NAU8825_GAIN_ERR_MASK: u32 = 0xf << NAU8825_GAIN_ERR_SFT;
pub const NAU8825_FLL_INTEGER_MASK: u32 = 0x3ff << 0;
pub const NAU8825_FLL_CLK_SRC_SFT: u32 = 10;
pub const NAU8825_FLL_CLK_SRC_MASK: u32 = 0x3 << NAU8825_FLL_CLK_SRC_SFT;
pub const NAU8825_FLL_CLK_SRC_MCLK: u32 = 0 << NAU8825_FLL_CLK_SRC_SFT;
pub const NAU8825_FLL_CLK_SRC_BLK: u32 = 0x2 << NAU8825_FLL_CLK_SRC_SFT;
pub const NAU8825_FLL_CLK_SRC_FS: u32 = 0x3 << NAU8825_FLL_CLK_SRC_SFT;

/* FLL4 (0x07) */
pub const NAU8825_FLL_REF_DIV_SFT: u32 = 10;
pub const NAU8825_FLL_REF_DIV_MASK: u32 = 0x3 << NAU8825_FLL_REF_DIV_SFT;

/* FLL5 (0x08) */
pub const NAU8825_FLL_PDB_DAC_EN: u32 = 0x1 << 15;
pub const NAU8825_FLL_LOOP_FTR_EN: u32 = 0x1 << 14;
pub const NAU8825_FLL_CLK_SW_MASK: u32 = 0x1 << 13;
pub const NAU8825_FLL_CLK_SW_N2: u32 = 0x1 << 13;
pub const NAU8825_FLL_CLK_SW_REF: u32 = 0x0 << 13;
pub const NAU8825_FLL_FTR_SW_MASK: u32 = 0x1 << 12;
pub const NAU8825_FLL_FTR_SW_ACCU: u32 = 0x1 << 12;
pub const NAU8825_FLL_FTR_SW_FILTER: u32 = 0x0 << 12;

/* FLL6 (0x9) */
pub const NAU8825_DCO_EN: u32 = 0x1 << 15;
pub const NAU8825_SDM_EN: u32 = 0x1 << 14;
pub const NAU8825_CUTOFF500: u32 = 0x1 << 13;

/* HSD_CTRL (0xc) */
pub const NAU8825_HSD_AUTO_MODE: u32 = 1 << 6;
/* 0 - open, 1 - short to GND */
pub const NAU8825_SPKR_ENGND1: u32 = 1 << 3;
pub const NAU8825_SPKR_ENGND2: u32 = 1 << 2;
pub const NAU8825_SPKR_DWN1R: u32 = 1 << 1;
pub const NAU8825_SPKR_DWN1L: u32 = 1 << 0;

/* JACK_DET_CTRL (0xd) */
pub const NAU8825_JACK_DET_RESTART: u32 = 1 << 9;
pub const NAU8825_JACK_DET_DB_BYPASS: u32 = 1 << 8;
pub const NAU8825_JACK_INSERT_DEBOUNCE_SFT: u32 = 5;
pub const NAU8825_JACK_INSERT_DEBOUNCE_MASK: u32 = 0x7 << NAU8825_JACK_INSERT_DEBOUNCE_SFT;
pub const NAU8825_JACK_EJECT_DEBOUNCE_SFT: u32 = 2;
pub const NAU8825_JACK_EJECT_DEBOUNCE_MASK: u32 = 0x7 << NAU8825_JACK_EJECT_DEBOUNCE_SFT;
pub const NAU8825_JACK_POLARITY: u32 = 1 << 1; /* 0 - active low, 1 - active high */

/* INTERRUPT_MASK (0xf) */
pub const NAU8825_IRQ_PIN_PULLUP: u32 = 1 << 14;
pub const NAU8825_IRQ_PIN_PULL_EN: u32 = 1 << 13;
pub const NAU8825_IRQ_OUTPUT_EN: u32 = 1 << 11;
pub const NAU8825_IRQ_HEADSET_COMPLETE_EN: u32 = 1 << 10;
pub const NAU8825_IRQ_RMS_EN: u32 = 1 << 8;
pub const NAU8825_IRQ_KEY_RELEASE_EN: u32 = 1 << 7;
pub const NAU8825_IRQ_KEY_SHORT_PRESS_EN: u32 = 1 << 5;
pub const NAU8825_IRQ_EJECT_EN: u32 = 1 << 2;
pub const NAU8825_IRQ_INSERT_EN: u32 = 1 << 0;

/* IRQ_STATUS (0x10) */
pub const NAU8825_HEADSET_COMPLETION_IRQ: u32 = 1 << 10;
pub const NAU8825_SHORT_CIRCUIT_IRQ: u32 = 1 << 9;
pub const NAU8825_IMPEDANCE_MEAS_IRQ: u32 = 1 << 8;
pub const NAU8825_KEY_IRQ_MASK: u32 = 0x7 << 5;
pub const NAU8825_KEY_RELEASE_IRQ: u32 = 1 << 7;
pub const NAU8825_KEY_LONG_PRESS_IRQ: u32 = 1 << 6;
pub const NAU8825_KEY_SHORT_PRESS_IRQ: u32 = 1 << 5;
pub const NAU8825_MIC_DETECTION_IRQ: u32 = 1 << 4;
pub const NAU8825_JACK_EJECTION_IRQ_MASK: u32 = 3 << 2;
pub const NAU8825_JACK_EJECTION_DETECTED: u32 = 1 << 2;
pub const NAU8825_JACK_INSERTION_IRQ_MASK: u32 = 3 << 0;
pub const NAU8825_JACK_INSERTION_DETECTED: u32 = 1 << 0;

/* INTERRUPT_DIS_CTRL (0x12) */
pub const NAU8825_IRQ_HEADSET_COMPLETE_DIS: u32 = 1 << 10;
pub const NAU8825_IRQ_KEY_RELEASE_DIS: u32 = 1 << 7;
pub const NAU8825_IRQ_KEY_SHORT_PRESS_DIS: u32 = 1 << 5;
pub const NAU8825_IRQ_EJECT_DIS: u32 = 1 << 2;
pub const NAU8825_IRQ_INSERT_DIS: u32 = 1 << 0;

/* SAR_CTRL (0x13) */
pub const NAU8825_SAR_ADC_EN_SFT: u32 = 12;
pub const NAU8825_SAR_ADC_EN: u32 = 1 << NAU8825_SAR_ADC_EN_SFT;
pub const NAU8825_SAR_INPUT_MASK: u32 = 1 << 11;
pub const NAU8825_SAR_INPUT_JKSLV: u32 = 1 << 11;
pub const NAU8825_SAR_INPUT_JKR2: u32 = 0 << 11;
pub const NAU8825_SAR_TRACKING_GAIN_SFT: u32 = 8;
pub const NAU8825_SAR_TRACKING_GAIN_MASK: u32 = 0x7 << NAU8825_SAR_TRACKING_GAIN_SFT;
pub const NAU8825_SAR_HV_SEL_SFT: u32 = 7;
pub const NAU8825_SAR_HV_SEL_MASK: u32 = 1 << NAU8825_SAR_HV_SEL_SFT;
pub const NAU8825_SAR_HV_SEL_MICBIAS: u32 = 0 << NAU8825_SAR_HV_SEL_SFT;
pub const NAU8825_SAR_HV_SEL_VDDMIC: u32 = 1 << NAU8825_SAR_HV_SEL_SFT;
pub const NAU8825_SAR_RES_SEL_SFT: u32 = 4;
pub const NAU8825_SAR_RES_SEL_MASK: u32 = 0x7 << NAU8825_SAR_RES_SEL_SFT;
pub const NAU8825_SAR_RES_SEL_35K: u32 = 0 << NAU8825_SAR_RES_SEL_SFT;
pub const NAU8825_SAR_RES_SEL_70K: u32 = 1 << NAU8825_SAR_RES_SEL_SFT;
pub const NAU8825_SAR_RES_SEL_170K: u32 = 2 << NAU8825_SAR_RES_SEL_SFT;
pub const NAU8825_SAR_RES_SEL_360K: u32 = 3 << NAU8825_SAR_RES_SEL_SFT;
pub const NAU8825_SAR_RES_SEL_SHORTED: u32 = 4 << NAU8825_SAR_RES_SEL_SFT;
pub const NAU8825_SAR_COMPARE_TIME_SFT: u32 = 2;
pub const NAU8825_SAR_COMPARE_TIME_MASK: u32 = 3 << 2;
pub const NAU8825_SAR_SAMPLING_TIME_SFT: u32 = 0;
pub const NAU8825_SAR_SAMPLING_TIME_MASK: u32 = 3 << 0;

/* KEYDET_CTRL (0x14) */
pub const NAU8825_KEYDET_SHORTKEY_DEBOUNCE_SFT: u32 = 12;
pub const NAU8825_KEYDET_SHORTKEY_DEBOUNCE_MASK: u32 = 0x3 << NAU8825_KEYDET_SHORTKEY_DEBOUNCE_SFT;
pub const NAU8825_KEYDET_LEVELS_NR_SFT: u32 = 8;
pub const NAU8825_KEYDET_LEVELS_NR_MASK: u32 = 0x7 << 8;
pub const NAU8825_KEYDET_HYSTERESIS_SFT: u32 = 0;
pub const NAU8825_KEYDET_HYSTERESIS_MASK: u32 = 0xf;

/* GPIO12_CTRL (0x1a) */
pub const NAU8825_JKDET_PULL_UP: u32 = 1 << 11; /* 0 - pull down, 1 - pull up */
pub const NAU8825_JKDET_PULL_EN: u32 = 1 << 9; /* 0 - enable pull, 1 - disable */
pub const NAU8825_JKDET_OUTPUT_EN: u32 = 1 << 8; /* 0 - enable input, 1 - enable output */

/* TDM_CTRL (0x1b) */
pub const NAU8825_TDM_MODE: u32 = 0x1 << 15;
pub const NAU8825_TDM_OFFSET_EN: u32 = 0x1 << 14;
pub const NAU8825_TDM_DACL_RX_SFT: u32 = 6;
pub const NAU8825_TDM_DACL_RX_MASK: u32 = 0x3 << NAU8825_TDM_DACL_RX_SFT;
pub const NAU8825_TDM_DACR_RX_SFT: u32 = 4;
pub const NAU8825_TDM_DACR_RX_MASK: u32 = 0x3 << NAU8825_TDM_DACR_RX_SFT;
pub const NAU8825_TDM_TX_MASK: u32 = 0x3;

/* I2S_PCM_CTRL1 (0x1c) */
pub const NAU8825_I2S_BP_SFT: u32 = 7;
pub const NAU8825_I2S_BP_MASK: u32 = 1 << NAU8825_I2S_BP_SFT;
pub const NAU8825_I2S_BP_INV: u32 = 1 << NAU8825_I2S_BP_SFT;
pub const NAU8825_I2S_PCMB_SFT: u32 = 6;
pub const NAU8825_I2S_PCMB_MASK: u32 = 1 << NAU8825_I2S_PCMB_SFT;
pub const NAU8825_I2S_PCMB_EN: u32 = 1 << NAU8825_I2S_PCMB_SFT;
pub const NAU8825_I2S_DL_SFT: u32 = 2;
pub const NAU8825_I2S_DL_MASK: u32 = 0x3 << NAU8825_I2S_DL_SFT;
pub const NAU8825_I2S_DL_16: u32 = 0 << NAU8825_I2S_DL_SFT;
pub const NAU8825_I2S_DL_20: u32 = 1 << NAU8825_I2S_DL_SFT;
pub const NAU8825_I2S_DL_24: u32 = 2 << NAU8825_I2S_DL_SFT;
pub const NAU8825_I2S_DL_32: u32 = 3 << NAU8825_I2S_DL_SFT;
pub const NAU8825_I2S_DF_SFT: u32 = 0;
pub const NAU8825_I2S_DF_MASK: u32 = 0x3 << NAU8825_I2S_DF_SFT;
pub const NAU8825_I2S_DF_RIGTH: u32 = 0 << NAU8825_I2S_DF_SFT;
pub const NAU8825_I2S_DF_LEFT: u32 = 1 << NAU8825_I2S_DF_SFT;
pub const NAU8825_I2S_DF_I2S: u32 = 2 << NAU8825_I2S_DF_SFT;
pub const NAU8825_I2S_DF_PCM_AB: u32 = 3 << NAU8825_I2S_DF_SFT;

/* I2S_PCM_CTRL2 (0x1d) */
pub const NAU8825_I2S_TRISTATE: u32 = 1 << 15; /* 0 - normal mode, 1 - Hi-Z output */
pub const NAU8825_I2S_LRC_DIV_SFT: u32 = 12;
pub const NAU8825_I2S_LRC_DIV_MASK: u32 = 0x3 << NAU8825_I2S_LRC_DIV_SFT;
pub const NAU8825_I2S_PCM_TS_EN_SFT: u32 = 10;
pub const NAU8825_I2S_PCM_TS_EN_MASK: u32 = 1 << NAU8825_I2S_PCM_TS_EN_SFT;
pub const NAU8825_I2S_PCM_TS_EN: u32 = 1 << NAU8825_I2S_PCM_TS_EN_SFT;
pub const NAU8825_I2S_MS_SFT: u32 = 3;
pub const NAU8825_I2S_MS_MASK: u32 = 1 << NAU8825_I2S_MS_SFT;
pub const NAU8825_I2S_MS_MASTER: u32 = 1 << NAU8825_I2S_MS_SFT;
pub const NAU8825_I2S_MS_SLAVE: u32 = 0 << NAU8825_I2S_MS_SFT;
pub const NAU8825_I2S_BLK_DIV_MASK: u32 = 0x7;

/* LEFT_TIME_SLOT (0x1e) */
pub const NAU8825_FS_ERR_CMP_SEL_SFT: u32 = 14;
pub const NAU8825_FS_ERR_CMP_SEL_MASK: u32 = 0x3 << NAU8825_FS_ERR_CMP_SEL_SFT;
pub const NAU8825_DIS_FS_SHORT_DET: u32 = 1 << 13;
pub const NAU8825_TSLOT_L0_MASK: u32 = 0x3ff;
pub const NAU8825_TSLOT_R0_MASK: u32 = 0x3ff;

/* BIQ_CTRL (0x20) */
pub const NAU8825_BIQ_WRT_SFT: u32 = 4;
pub const NAU8825_BIQ_WRT_EN: u32 = 1 << NAU8825_BIQ_WRT_SFT;
pub const NAU8825_BIQ_PATH_SFT: u32 = 0;
pub const NAU8825_BIQ_PATH_MASK: u32 = 1 << NAU8825_BIQ_PATH_SFT;
pub const NAU8825_BIQ_PATH_ADC: u32 = 0 << NAU8825_BIQ_PATH_SFT;
pub const NAU8825_BIQ_PATH_DAC: u32 = 1 << NAU8825_BIQ_PATH_SFT;

/* ADC_RATE (0x2b) */
pub const NAU8825_ADC_SINC4_SFT: u32 = 4;
pub const NAU8825_ADC_SINC4_EN: u32 = 1 << NAU8825_ADC_SINC4_SFT;
pub const NAU8825_ADC_SYNC_DOWN_SFT: u32 = 0;
pub const NAU8825_ADC_SYNC_DOWN_MASK: u32 = 0x3;
pub const NAU8825_ADC_SYNC_DOWN_32: u32 = 0;
pub const NAU8825_ADC_SYNC_DOWN_64: u32 = 1;
pub const NAU8825_ADC_SYNC_DOWN_128: u32 = 2;
pub const NAU8825_ADC_SYNC_DOWN_256: u32 = 3;

/* DAC_CTRL1 (0x2c) */
pub const NAU8825_DAC_CLIP_OFF: u32 = 1 << 7;
pub const NAU8825_DAC_OVERSAMPLE_SFT: u32 = 0;
pub const NAU8825_DAC_OVERSAMPLE_MASK: u32 = 0x7;
pub const NAU8825_DAC_OVERSAMPLE_64: u32 = 0;
pub const NAU8825_DAC_OVERSAMPLE_256: u32 = 1;
pub const NAU8825_DAC_OVERSAMPLE_128: u32 = 2;
pub const NAU8825_DAC_OVERSAMPLE_32: u32 = 4;

/* ADC_DGAIN_CTRL (0x30) */
pub const NAU8825_ADC_DIG_VOL_MASK: u32 = 0xff;

/* MUTE_CTRL (0x31) */
pub const NAU8825_DAC_ZERO_CROSSING_EN: u32 = 1 << 9;
pub const NAU8825_DAC_SOFT_MUTE: u32 = 1 << 9;

/* HSVOL_CTRL (0x32) */
pub const NAU8825_HP_MUTE: u32 = 1 << 15;
pub const NAU8825_HP_MUTE_AUTO: u32 = 1 << 14;
pub const NAU8825_HPL_MUTE: u32 = 1 << 13;
pub const NAU8825_HPR_MUTE: u32 = 1 << 12;
pub const NAU8825_HPL_VOL_SFT: u32 = 6;
pub const NAU8825_HPL_VOL_MASK: u32 = 0x3f << NAU8825_HPL_VOL_SFT;
pub const NAU8825_HPR_VOL_SFT: u32 = 0;
pub const NAU8825_HPR_VOL_MASK: u32 = 0x3f << NAU8825_HPR_VOL_SFT;
pub const NAU8825_HP_VOL_MIN: u32 = 0x36;

/* DACL_CTRL (0x33) */
pub const NAU8825_DACL_CH_SEL_SFT: u32 = 9;
pub const NAU8825_DACL_CH_SEL_MASK: u32 = 0x1 << NAU8825_DACL_CH_SEL_SFT;
pub const NAU8825_DACL_CH_SEL_L: u32 = 0x0 << NAU8825_DACL_CH_SEL_SFT;
pub const NAU8825_DACL_CH_SEL_R: u32 = 0x1 << NAU8825_DACL_CH_SEL_SFT;
pub const NAU8825_DACL_CH_VOL_MASK: u32 = 0xff;

/* DACR_CTRL (0x34) */
pub const NAU8825_DACR_CH_SEL_SFT: u32 = 9;
pub const NAU8825_DACR_CH_SEL_MASK: u32 = 0x1 << NAU8825_DACR_CH_SEL_SFT;
pub const NAU8825_DACR_CH_SEL_L: u32 = 0x0 << NAU8825_DACR_CH_SEL_SFT;
pub const NAU8825_DACR_CH_SEL_R: u32 = 0x1 << NAU8825_DACR_CH_SEL_SFT;
pub const NAU8825_DACR_CH_VOL_MASK: u32 = 0xff;

/* IMM_MODE_CTRL (0x4C) */
pub const NAU8825_IMM_THD_SFT: u32 = 8;
pub const NAU8825_IMM_THD_MASK: u32 = 0x3f << NAU8825_IMM_THD_SFT;
pub const NAU8825_IMM_GEN_VOL_SFT: u32 = 6;
pub const NAU8825_IMM_GEN_VOL_MASK: u32 = 0x3 << NAU8825_IMM_GEN_VOL_SFT;
pub const NAU8825_IMM_GEN_VOL_1_2nd: u32 = 0x0 << NAU8825_IMM_GEN_VOL_SFT;
pub const NAU8825_IMM_GEN_VOL_1_4th: u32 = 0x1 << NAU8825_IMM_GEN_VOL_SFT;
pub const NAU8825_IMM_GEN_VOL_1_8th: u32 = 0x2 << NAU8825_IMM_GEN_VOL_SFT;
pub const NAU8825_IMM_GEN_VOL_1_16th: u32 = 0x3 << NAU8825_IMM_GEN_VOL_SFT;

pub const NAU8825_IMM_CYC_SFT: u32 = 4;
pub const NAU8825_IMM_CYC_MASK: u32 = 0x3 << NAU8825_IMM_CYC_SFT;
pub const NAU8825_IMM_CYC_1024: u32 = 0x0 << NAU8825_IMM_CYC_SFT;
pub const NAU8825_IMM_CYC_2048: u32 = 0x1 << NAU8825_IMM_CYC_SFT;
pub const NAU8825_IMM_CYC_4096: u32 = 0x2 << NAU8825_IMM_CYC_SFT;
pub const NAU8825_IMM_CYC_8192: u32 = 0x3 << NAU8825_IMM_CYC_SFT;
pub const NAU8825_IMM_EN: u32 = 1 << 3;
pub const NAU8825_IMM_DAC_SRC_MASK: u32 = 0x7;
pub const NAU8825_IMM_DAC_SRC_BIQ: u32 = 0x0;
pub const NAU8825_IMM_DAC_SRC_DRC: u32 = 0x1;
pub const NAU8825_IMM_DAC_SRC_MIX: u32 = 0x2;
pub const NAU8825_IMM_DAC_SRC_SIN: u32 = 0x3;

/* CLASSG_CTRL (0x50) */
pub const NAU8825_CLASSG_TIMER_SFT: u32 = 8;
pub const NAU8825_CLASSG_TIMER_MASK: u32 = 0x3f << NAU8825_CLASSG_TIMER_SFT;
pub const NAU8825_CLASSG_TIMER_1ms: u32 = 0x1 << NAU8825_CLASSG_TIMER_SFT;
pub const NAU8825_CLASSG_TIMER_2ms: u32 = 0x2 << NAU8825_CLASSG_TIMER_SFT;
pub const NAU8825_CLASSG_TIMER_8ms: u32 = 0x4 << NAU8825_CLASSG_TIMER_SFT;
pub const NAU8825_CLASSG_TIMER_16ms: u32 = 0x8 << NAU8825_CLASSG_TIMER_SFT;
pub const NAU8825_CLASSG_TIMER_32ms: u32 = 0x10 << NAU8825_CLASSG_TIMER_SFT;
pub const NAU8825_CLASSG_TIMER_64ms: u32 = 0x20 << NAU8825_CLASSG_TIMER_SFT;
pub const NAU8825_CLASSG_LDAC_EN: u32 = 0x1 << 2;
pub const NAU8825_CLASSG_RDAC_EN: u32 = 0x1 << 1;
pub const NAU8825_CLASSG_EN: u32 = 1 << 0;

/* I2C_DEVICE_ID (0x58) */
pub const NAU8825_GPIO2JD1: u32 = 1 << 7;
pub const NAU8825_SOFTWARE_ID_MASK: u32 = 0x3;
pub const NAU8825_SOFTWARE_ID_NAU8825: u32 = 0x0;
pub const NAU8825_SOFTWARE_ID_NAU8825C: u32 = 0x1;

/* BIAS_ADJ (0x66) */
pub const NAU8825_BIAS_HPR_IMP: u32 = 1 << 15;
pub const NAU8825_BIAS_HPL_IMP: u32 = 1 << 14;
pub const NAU8825_BIAS_TESTDAC_SFT: u32 = 8;
pub const NAU8825_BIAS_TESTDAC_EN: u32 = 0x3 << NAU8825_BIAS_TESTDAC_SFT;
pub const NAU8825_BIAS_TESTDACR_EN: u32 = 0x2 << NAU8825_BIAS_TESTDAC_SFT;
pub const NAU8825_BIAS_TESTDACL_EN: u32 = 0x1 << NAU8825_BIAS_TESTDAC_SFT;
pub const NAU8825_BIAS_VMID: u32 = 1 << 6;
pub const NAU8825_BIAS_VMID_SEL_SFT: u32 = 4;
pub const NAU8825_BIAS_VMID_SEL_MASK: u32 = 3 << NAU8825_BIAS_VMID_SEL_SFT;

/* ANALOG_CONTROL_1 (0x69) */
pub const NAU8825_TESTDACIN_SFT: u32 = 14;
pub const NAU8825_TESTDACIN_MASK: u32 = 0x3 << NAU8825_TESTDACIN_SFT;
pub const NAU8825_TESTDACIN_HIGH: u32 = 1 << NAU8825_TESTDACIN_SFT;
pub const NAU8825_TESTDACIN_LOW: u32 = 2 << NAU8825_TESTDACIN_SFT;
pub const NAU8825_TESTDACIN_GND: u32 = 3 << NAU8825_TESTDACIN_SFT;

/* ANALOG_CONTROL_2 (0x6a) */
pub const NAU8825_HP_NON_CLASSG_CURRENT_2xADJ: u32 = 1 << 12;
pub const NAU8825_DAC_CAPACITOR_MSB: u32 = 1 << 1;
pub const NAU8825_DAC_CAPACITOR_LSB: u32 = 1 << 0;

/* ANALOG_ADC_2 (0x72) */
pub const NAU8825_ADC_VREFSEL_MASK: u32 = 0x3 << 8;
pub const NAU8825_ADC_VREFSEL_ANALOG: u32 = 0 << 8;
pub const NAU8825_ADC_VREFSEL_VMID: u32 = 1 << 8;
pub const NAU8825_ADC_VREFSEL_VMID_PLUS_0_5DB: u32 = 2 << 8;
pub const NAU8825_ADC_VREFSEL_VMID_PLUS_1DB: u32 = 3 << 8;
pub const NAU8825_POWERUP_ADCL: u32 = 1 << 6;

/* RDAC (0x73) */
pub const NAU8825_RDAC_FS_BCLK_ENB: u32 = 1 << 15;
pub const NAU8825_RDAC_EN_SFT: u32 = 12;
pub const NAU8825_RDAC_EN: u32 = 0x3 << NAU8825_RDAC_EN_SFT;
pub const NAU8825_RDAC_CLK_EN_SFT: u32 = 8;
pub const NAU8825_RDAC_CLK_EN: u32 = 0x3 << NAU8825_RDAC_CLK_EN_SFT;
pub const NAU8825_RDAC_CLK_DELAY_SFT: u32 = 4;
pub const NAU8825_RDAC_CLK_DELAY_MASK: u32 = 0x7 << NAU8825_RDAC_CLK_DELAY_SFT;
pub const NAU8825_RDAC_VREF_SFT: u32 = 2;
pub const NAU8825_RDAC_VREF_MASK: u32 = 0x3 << NAU8825_RDAC_VREF_SFT;

/* MIC_BIAS (0x74) */
pub const NAU8825_MICBIAS_JKSLV: u32 = 1 << 14;
pub const NAU8825_MICBIAS_JKR2: u32 = 1 << 12;
pub const NAU8825_MICBIAS_LOWNOISE_SFT: u32 = 10;
pub const NAU8825_MICBIAS_LOWNOISE_MASK: u32 = 0x1 << NAU8825_MICBIAS_LOWNOISE_SFT;
pub const NAU8825_MICBIAS_LOWNOISE_EN: u32 = 0x1 << NAU8825_MICBIAS_LOWNOISE_SFT;
pub const NAU8825_MICBIAS_POWERUP_SFT: u32 = 8;
pub const NAU8825_MICBIAS_VOLTAGE_SFT: u32 = 0;
pub const NAU8825_MICBIAS_VOLTAGE_MASK: u32 = 0x7;

/* BOOST (0x76) */
pub const NAU8825_PRECHARGE_DIS: u32 = 1 << 13;
pub const NAU8825_GLOBAL_BIAS_EN: u32 = 1 << 12;
pub const NAU8825_DISCHRG_EN: u32 = 1 << 11;
pub const NAU8825_HP_BOOST_DIS: u32 = 1 << 9;
pub const NAU8825_HP_BOOST_G_DIS: u32 = 1 << 8;
pub const NAU8825_SHORT_SHUTDOWN_EN: u32 = 1 << 6;

/* FEPGA (0x77) */
pub const NAU8825_ACDC_CTRL_SFT: u32 = 14;
pub const NAU8825_ACDC_CTRL_MASK: u32 = 0x3 << NAU8825_ACDC_CTRL_SFT;
pub const NAU8825_ACDC_VREF_MICP: u32 = 0x1 << NAU8825_ACDC_CTRL_SFT;
pub const NAU8825_ACDC_VREF_MICN: u32 = 0x2 << NAU8825_ACDC_CTRL_SFT;

/* POWER_UP_CONTROL (0x7f) */
pub const NAU8825_POWERUP_INTEGR_R: u32 = 1 << 5;
pub const NAU8825_POWERUP_INTEGR_L: u32 = 1 << 4;
pub const NAU8825_POWERUP_DRV_IN_R: u32 = 1 << 3;
pub const NAU8825_POWERUP_DRV_IN_L: u32 = 1 << 2;
pub const NAU8825_POWERUP_HP_DRV_R: u32 = 1 << 1;
pub const NAU8825_POWERUP_HP_DRV_L: u32 = 1 << 0;

/* CHARGE_PUMP (0x80) */
pub const NAU8825_ADCOUT_DS_SFT: u32 = 12;
pub const NAU8825_ADCOUT_DS_MASK: u32 = 1 << NAU8825_ADCOUT_DS_SFT;
pub const NAU8825_JAMNODCLOW: u32 = 1 << 10;
pub const NAU8825_POWER_DOWN_DACR: u32 = 1 << 9;
pub const NAU8825_POWER_DOWN_DACL: u32 = 1 << 8;
pub const NAU8825_CHANRGE_PUMP_EN: u32 = 1 << 5;

/* System Clock Source */
pub const NAU8825_CLK_DIS: u32 = 0;
pub const NAU8825_CLK_MCLK: u32 = 1;
pub const NAU8825_CLK_INTERNAL: u32 = 2;
pub const NAU8825_CLK_FLL_MCLK: u32 = 3;
pub const NAU8825_CLK_FLL_BLK: u32 = 4;
pub const NAU8825_CLK_FLL_FS: u32 = 5;

/* Cross talk detection state */
pub const NAU8825_XTALK_PREPARE: u32 = 0;
pub const NAU8825_XTALK_HPR_R2L: u32 = 1;
pub const NAU8825_XTALK_HPL_R2L: u32 = 2;
pub const NAU8825_XTALK_IMM: u32 = 3;
pub const NAU8825_XTALK_DONE: u32 = 4;

#[repr(C)]
pub struct nau8825 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub dapm: *mut snd_soc_dapm_context,
    pub jack: *mut snd_soc_jack,
    pub mclk: *mut clk,
    pub xtalk_work: work_struct,
    pub xtalk_sem: semaphore,
    pub sw_id: core::ffi::c_int,
    pub irq: core::ffi::c_int,
    pub mclk_freq: core::ffi::c_int, /* 0 - mclk is disabled */
    pub button_pressed: core::ffi::c_int,
    pub micbias_voltage: core::ffi::c_int,
    pub vref_impedance: core::ffi::c_int,
    pub jkdet_enable: bool,
    pub jkdet_pull_enable: bool,
    pub jkdet_pull_up: bool,
    pub jkdet_polarity: core::ffi::c_int,
    pub sar_threshold_num: core::ffi::c_int,
    pub sar_threshold: [core::ffi::c_int; 8],
    pub sar_hysteresis: core::ffi::c_int,
    pub sar_voltage: core::ffi::c_int,
    pub sar_compare_time: core::ffi::c_int,
    pub sar_sampling_time: core::ffi::c_int,
    pub key_debounce: core::ffi::c_int,
    pub jack_insert_debounce: core::ffi::c_int,
    pub jack_eject_debounce: core::ffi::c_int,
    pub high_imped: core::ffi::c_int,
    pub xtalk_state: core::ffi::c_int,
    pub xtalk_event: core::ffi::c_int,
    pub xtalk_event_mask: core::ffi::c_int,
    pub xtalk_protect: bool,
    pub imp_rms: [core::ffi::c_int; NAU8825_XTALK_IMM as usize],
    pub xtalk_enable: core::ffi::c_int,
    pub xtalk_baktab_initialized: bool, /* True if initialized. */
    pub adcout_ds: bool,
    pub adc_delay: core::ffi::c_int,
}

unsafe extern "C" {
    pub fn nau8825_enable_jack_detect(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
