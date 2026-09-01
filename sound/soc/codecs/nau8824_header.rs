/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * NAU88L24 ALSA SoC audio driver
 *
 * Copyright 2016 Nuvoton Technology Corp.
 * Author: John Hsu <KCHSU0@nuvoton.com>
 */


pub const NAU8824_REG_RESET: u32 = 0x00;
pub const NAU8824_REG_ENA_CTRL: u32 = 0x01;
pub const NAU8824_REG_CLK_GATING_ENA: u32 = 0x02;
pub const NAU8824_REG_CLK_DIVIDER: u32 = 0x03;
pub const NAU8824_REG_FLL1: u32 = 0x04;
pub const NAU8824_REG_FLL2: u32 = 0x05;
pub const NAU8824_REG_FLL3: u32 = 0x06;
pub const NAU8824_REG_FLL4: u32 = 0x07;
pub const NAU8824_REG_FLL5: u32 = 0x08;
pub const NAU8824_REG_FLL6: u32 = 0x09;
pub const NAU8824_REG_FLL_VCO_RSV: u32 = 0x0A;
pub const NAU8824_REG_JACK_DET_CTRL: u32 = 0x0D;
pub const NAU8824_REG_INTERRUPT_SETTING_1: u32 = 0x0F;
pub const NAU8824_REG_IRQ: u32 = 0x10;
pub const NAU8824_REG_CLEAR_INT_REG: u32 = 0x11;
pub const NAU8824_REG_INTERRUPT_SETTING: u32 = 0x12;
pub const NAU8824_REG_SAR_ADC: u32 = 0x13;
pub const NAU8824_REG_VDET_COEFFICIENT: u32 = 0x14;
pub const NAU8824_REG_VDET_THRESHOLD_1: u32 = 0x15;
pub const NAU8824_REG_VDET_THRESHOLD_2: u32 = 0x16;
pub const NAU8824_REG_VDET_THRESHOLD_3: u32 = 0x17;
pub const NAU8824_REG_VDET_THRESHOLD_4: u32 = 0x18;
pub const NAU8824_REG_GPIO_SEL: u32 = 0x1A;
pub const NAU8824_REG_PORT0_I2S_PCM_CTRL_1: u32 = 0x1C;
pub const NAU8824_REG_PORT0_I2S_PCM_CTRL_2: u32 = 0x1D;
pub const NAU8824_REG_PORT0_LEFT_TIME_SLOT: u32 = 0x1E;
pub const NAU8824_REG_PORT0_RIGHT_TIME_SLOT: u32 = 0x1F;
pub const NAU8824_REG_TDM_CTRL: u32 = 0x20;
pub const NAU8824_REG_ADC_HPF_FILTER: u32 = 0x23;
pub const NAU8824_REG_ADC_FILTER_CTRL: u32 = 0x24;
pub const NAU8824_REG_DAC_FILTER_CTRL_1: u32 = 0x25;
pub const NAU8824_REG_DAC_FILTER_CTRL_2: u32 = 0x26;
pub const NAU8824_REG_NOTCH_FILTER_1: u32 = 0x27;
pub const NAU8824_REG_NOTCH_FILTER_2: u32 = 0x28;
pub const NAU8824_REG_EQ1_LOW: u32 = 0x29;
pub const NAU8824_REG_EQ2_EQ3: u32 = 0x2A;
pub const NAU8824_REG_EQ4_EQ5: u32 = 0x2B;
pub const NAU8824_REG_ADC_CH0_DGAIN_CTRL: u32 = 0x2D;
pub const NAU8824_REG_ADC_CH1_DGAIN_CTRL: u32 = 0x2E;
pub const NAU8824_REG_ADC_CH2_DGAIN_CTRL: u32 = 0x2F;
pub const NAU8824_REG_ADC_CH3_DGAIN_CTRL: u32 = 0x30;
pub const NAU8824_REG_DAC_MUTE_CTRL: u32 = 0x31;
pub const NAU8824_REG_DAC_CH0_DGAIN_CTRL: u32 = 0x32;
pub const NAU8824_REG_DAC_CH1_DGAIN_CTRL: u32 = 0x33;
pub const NAU8824_REG_ADC_TO_DAC_ST: u32 = 0x34;
pub const NAU8824_REG_DRC_KNEE_IP12_ADC_CH01: u32 = 0x38;
pub const NAU8824_REG_DRC_KNEE_IP34_ADC_CH01: u32 = 0x39;
pub const NAU8824_REG_DRC_SLOPE_ADC_CH01: u32 = 0x3A;
pub const NAU8824_REG_DRC_ATKDCY_ADC_CH01: u32 = 0x3B;
pub const NAU8824_REG_DRC_KNEE_IP12_ADC_CH23: u32 = 0x3C;
pub const NAU8824_REG_DRC_KNEE_IP34_ADC_CH23: u32 = 0x3D;
pub const NAU8824_REG_DRC_SLOPE_ADC_CH23: u32 = 0x3E;
pub const NAU8824_REG_DRC_ATKDCY_ADC_CH23: u32 = 0x3F;
pub const NAU8824_REG_DRC_GAINL_ADC0: u32 = 0x40;
pub const NAU8824_REG_DRC_GAINL_ADC1: u32 = 0x41;
pub const NAU8824_REG_DRC_GAINL_ADC2: u32 = 0x42;
pub const NAU8824_REG_DRC_GAINL_ADC3: u32 = 0x43;
pub const NAU8824_REG_DRC_KNEE_IP12_DAC: u32 = 0x45;
pub const NAU8824_REG_DRC_KNEE_IP34_DAC: u32 = 0x46;
pub const NAU8824_REG_DRC_SLOPE_DAC: u32 = 0x47;
pub const NAU8824_REG_DRC_ATKDCY_DAC: u32 = 0x48;
pub const NAU8824_REG_DRC_GAIN_DAC_CH0: u32 = 0x49;
pub const NAU8824_REG_DRC_GAIN_DAC_CH1: u32 = 0x4A;
pub const NAU8824_REG_MODE: u32 = 0x4C;
pub const NAU8824_REG_MODE1: u32 = 0x4D;
pub const NAU8824_REG_MODE2: u32 = 0x4E;
pub const NAU8824_REG_CLASSG: u32 = 0x50;
pub const NAU8824_REG_OTP_EFUSE: u32 = 0x51;
pub const NAU8824_REG_OTPDOUT_1: u32 = 0x53;
pub const NAU8824_REG_OTPDOUT_2: u32 = 0x54;
pub const NAU8824_REG_MISC_CTRL: u32 = 0x55;
pub const NAU8824_REG_I2C_TIMEOUT: u32 = 0x56;
pub const NAU8824_REG_TEST_MODE: u32 = 0x57;
pub const NAU8824_REG_I2C_DEVICE_ID: u32 = 0x58;
pub const NAU8824_REG_SAR_ADC_DATA_OUT: u32 = 0x59;
pub const NAU8824_REG_BIAS_ADJ: u32 = 0x66;
pub const NAU8824_REG_PGA_GAIN: u32 = 0x67;
pub const NAU8824_REG_TRIM_SETTINGS: u32 = 0x68;
pub const NAU8824_REG_ANALOG_CONTROL_1: u32 = 0x69;
pub const NAU8824_REG_ANALOG_CONTROL_2: u32 = 0x6A;
pub const NAU8824_REG_ENABLE_LO: u32 = 0x6B;
pub const NAU8824_REG_GAIN_LO: u32 = 0x6C;
pub const NAU8824_REG_CLASSD_GAIN_1: u32 = 0x6D;
pub const NAU8824_REG_CLASSD_GAIN_2: u32 = 0x6E;
pub const NAU8824_REG_ANALOG_ADC_1: u32 = 0x71;
pub const NAU8824_REG_ANALOG_ADC_2: u32 = 0x72;
pub const NAU8824_REG_RDAC: u32 = 0x73;
pub const NAU8824_REG_MIC_BIAS: u32 = 0x74;
pub const NAU8824_REG_HS_VOLUME_CONTROL: u32 = 0x75;
pub const NAU8824_REG_BOOST: u32 = 0x76;
pub const NAU8824_REG_FEPGA: u32 = 0x77;
pub const NAU8824_REG_FEPGA_II: u32 = 0x78;
pub const NAU8824_REG_FEPGA_SE: u32 = 0x79;
pub const NAU8824_REG_FEPGA_ATTENUATION: u32 = 0x7A;
pub const NAU8824_REG_ATT_PORT0: u32 = 0x7B;
pub const NAU8824_REG_ATT_PORT1: u32 = 0x7C;
pub const NAU8824_REG_POWER_UP_CONTROL: u32 = 0x7F;
pub const NAU8824_REG_CHARGE_PUMP_CONTROL: u32 = 0x80;
pub const NAU8824_REG_CHARGE_PUMP_INPUT: u32 = 0x81;
pub const NAU8824_REG_MAX: u32 = NAU8824_REG_CHARGE_PUMP_INPUT;
/* 16-bit control register address, and 16-bits control register data */
pub const NAU8824_REG_ADDR_LEN: u32 = 16;
pub const NAU8824_REG_DATA_LEN: u32 = 16;


/* ENA_CTRL (0x1) */
pub const NAU8824_DMIC_LCH_EDGE_CH23: u32 = (0x1 << 12);
pub const NAU8824_DMIC_LCH_EDGE_CH01: u32 = (0x1 << 11);
pub const NAU8824_JD_SLEEP_MODE: u32 = (0x1 << 10);
pub const NAU8824_ADC_CH3_DMIC_SFT: u32 = 9;
pub const NAU8824_ADC_CH3_DMIC_EN: u32 = (0x1 << NAU8824_ADC_CH3_DMIC_SFT);
pub const NAU8824_ADC_CH2_DMIC_SFT: u32 = 8;
pub const NAU8824_ADC_CH2_DMIC_EN: u32 = (0x1 << NAU8824_ADC_CH2_DMIC_SFT);
pub const NAU8824_ADC_CH1_DMIC_SFT: u32 = 7;
pub const NAU8824_ADC_CH1_DMIC_EN: u32 = (0x1 << NAU8824_ADC_CH1_DMIC_SFT);
pub const NAU8824_ADC_CH0_DMIC_SFT: u32 = 6;
pub const NAU8824_ADC_CH0_DMIC_EN: u32 = (0x1 << NAU8824_ADC_CH0_DMIC_SFT);
pub const NAU8824_DAC_CH1_EN: u32 = (0x1 << 5);
pub const NAU8824_DAC_CH0_EN: u32 = (0x1 << 4);
pub const NAU8824_ADC_CH3_EN: u32 = (0x1 << 3);
pub const NAU8824_ADC_CH2_EN: u32 = (0x1 << 2);
pub const NAU8824_ADC_CH1_EN: u32 = (0x1 << 1);
pub const NAU8824_ADC_CH0_EN: u32 = 0x1;

/* CLK_GATING_ENA (0x02) */
pub const NAU8824_CLK_ADC_CH23_EN: u32 = (0x1 << 15);
pub const NAU8824_CLK_ADC_CH01_EN: u32 = (0x1 << 14);
pub const NAU8824_CLK_DAC_CH1_EN: u32 = (0x1 << 13);
pub const NAU8824_CLK_DAC_CH0_EN: u32 = (0x1 << 12);
pub const NAU8824_CLK_I2S_EN: u32 = (0x1 << 7);
pub const NAU8824_CLK_GAIN_EN: u32 = (0x1 << 5);
pub const NAU8824_CLK_SAR_EN: u32 = (0x1 << 3);
pub const NAU8824_CLK_DMIC_CH23_EN: u32 = (0x1 << 1);

/* CLK_DIVIDER (0x3) */
pub const NAU8824_CLK_SRC_SFT: u32 = 15;
pub const NAU8824_CLK_SRC_MASK: u32 = (1 << NAU8824_CLK_SRC_SFT);
pub const NAU8824_CLK_SRC_VCO: u32 = (1 << NAU8824_CLK_SRC_SFT);
pub const NAU8824_CLK_SRC_MCLK: u32 = (0 << NAU8824_CLK_SRC_SFT);
pub const NAU8824_CLK_MCLK_SRC_MASK: u32 = (0xf << 0);
pub const NAU8824_CLK_DMIC_SRC_SFT: u32 = 10;
pub const NAU8824_CLK_DMIC_SRC_MASK: u32 = (0x7 << NAU8824_CLK_DMIC_SRC_SFT);
pub const NAU8824_CLK_ADC_SRC_SFT: u32 = 6;
pub const NAU8824_CLK_ADC_SRC_MASK: u32 = (0x3 << NAU8824_CLK_ADC_SRC_SFT);
pub const NAU8824_CLK_DAC_SRC_SFT: u32 = 4;
pub const NAU8824_CLK_DAC_SRC_MASK: u32 = (0x3 << NAU8824_CLK_DAC_SRC_SFT);

/* FLL1 (0x04) */
pub const NAU8824_FLL_RATIO_MASK: u32 = (0x7f << 0);

/* FLL3 (0x06) */
pub const NAU8824_FLL_INTEGER_MASK: u32 = (0x3ff << 0);
pub const NAU8824_FLL_CLK_SRC_SFT: u32 = 10;
pub const NAU8824_FLL_CLK_SRC_MASK: u32 = (0x3 << NAU8824_FLL_CLK_SRC_SFT);
pub const NAU8824_FLL_CLK_SRC_MCLK: u32 = (0 << NAU8824_FLL_CLK_SRC_SFT);
pub const NAU8824_FLL_CLK_SRC_BLK: u32 = (0x2 << NAU8824_FLL_CLK_SRC_SFT);
pub const NAU8824_FLL_CLK_SRC_FS: u32 = (0x3 << NAU8824_FLL_CLK_SRC_SFT);

/* FLL4 (0x07) */
pub const NAU8824_FLL_REF_DIV_SFT: u32 = 10;
pub const NAU8824_FLL_REF_DIV_MASK: u32 = (0x3 << NAU8824_FLL_REF_DIV_SFT);

/* FLL5 (0x08) */
pub const NAU8824_FLL_PDB_DAC_EN: u32 = (0x1 << 15);
pub const NAU8824_FLL_LOOP_FTR_EN: u32 = (0x1 << 14);
pub const NAU8824_FLL_CLK_SW_MASK: u32 = (0x1 << 13);
pub const NAU8824_FLL_CLK_SW_N2: u32 = (0x1 << 13);
pub const NAU8824_FLL_CLK_SW_REF: u32 = (0x0 << 13);
pub const NAU8824_FLL_FTR_SW_MASK: u32 = (0x1 << 12);
pub const NAU8824_FLL_FTR_SW_ACCU: u32 = (0x1 << 12);
pub const NAU8824_FLL_FTR_SW_FILTER: u32 = (0x0 << 12);

/* FLL6 (0x9) */
pub const NAU8824_DCO_EN: u32 = (0x1 << 15);
pub const NAU8824_SDM_EN: u32 = (0x1 << 14);

/* IRQ (0x10) */
pub const NAU8824_SHORT_CIRCUIT_IRQ: u32 = (0x1 << 7);
pub const NAU8824_IMPEDANCE_MEAS_IRQ: u32 = (0x1 << 6);
pub const NAU8824_KEY_RELEASE_IRQ: u32 = (0x1 << 5);
pub const NAU8824_KEY_LONG_PRESS_IRQ: u32 = (0x1 << 4);
pub const NAU8824_KEY_SHORT_PRESS_IRQ: u32 = (0x1 << 3);
pub const NAU8824_JACK_EJECTION_DETECTED: u32 = (0x1 << 1);
pub const NAU8824_JACK_INSERTION_DETECTED: u32 = 0x1;

/* JACK_DET_CTRL (0x0D) */
pub const NAU8824_JACK_EJECT_DT_SFT: u32 = 2;
pub const NAU8824_JACK_EJECT_DT_MASK: u32 = (0x3 << NAU8824_JACK_EJECT_DT_SFT);
pub const NAU8824_JACK_LOGIC: u32 = (0x1 << 1);


/* INTERRUPT_SETTING_1 (0x0F) */
pub const NAU8824_IRQ_EJECT_EN: u32 = (0x1 << 9);
pub const NAU8824_IRQ_INSERT_EN: u32 = (0x1 << 8);

/* INTERRUPT_SETTING (0x12) */
pub const NAU8824_IRQ_KEY_RELEASE_DIS: u32 = (0x1 << 5);
pub const NAU8824_IRQ_KEY_SHORT_PRESS_DIS: u32 = (0x1 << 3);
pub const NAU8824_IRQ_EJECT_DIS: u32 = (0x1 << 1);
pub const NAU8824_IRQ_INSERT_DIS: u32 = 0x1;

/* SAR_ADC (0x13) */
pub const NAU8824_SAR_ADC_EN_SFT: u32 = 12;
pub const NAU8824_SAR_TRACKING_GAIN_SFT: u32 = 8;
pub const NAU8824_SAR_TRACKING_GAIN_MASK: u32 = (0x7 << NAU8824_SAR_TRACKING_GAIN_SFT);
pub const NAU8824_SAR_COMPARE_TIME_SFT: u32 = 2;
pub const NAU8824_SAR_COMPARE_TIME_MASK: u32 = (3 << 2);
pub const NAU8824_SAR_SAMPLING_TIME_SFT: u32 = 0;
pub const NAU8824_SAR_SAMPLING_TIME_MASK: u32 = (3 << 0);

/* VDET_COEFFICIENT (0x14) */
pub const NAU8824_SHORTKEY_DEBOUNCE_SFT: u32 = 12;
pub const NAU8824_SHORTKEY_DEBOUNCE_MASK: u32 = (0x3 << NAU8824_SHORTKEY_DEBOUNCE_SFT);
pub const NAU8824_LEVELS_NR_SFT: u32 = 8;
pub const NAU8824_LEVELS_NR_MASK: u32 = (0x7 << 8);
pub const NAU8824_HYSTERESIS_SFT: u32 = 0;
pub const NAU8824_HYSTERESIS_MASK: u32 = 0xf;

/* PORT0_I2S_PCM_CTRL_1 (0x1C) */
pub const NAU8824_I2S_BP_SFT: u32 = 7;
pub const NAU8824_I2S_BP_MASK: u32 = (1 << NAU8824_I2S_BP_SFT);
pub const NAU8824_I2S_BP_INV: u32 = (1 << NAU8824_I2S_BP_SFT);
pub const NAU8824_I2S_PCMB_SFT: u32 = 6;
pub const NAU8824_I2S_PCMB_EN: u32 = (1 << NAU8824_I2S_PCMB_SFT);
pub const NAU8824_I2S_DL_SFT: u32 = 2;
pub const NAU8824_I2S_DL_MASK: u32 = (0x3 << NAU8824_I2S_DL_SFT);
pub const NAU8824_I2S_DL_16: u32 = (0 << NAU8824_I2S_DL_SFT);
pub const NAU8824_I2S_DL_20: u32 = (1 << NAU8824_I2S_DL_SFT);
pub const NAU8824_I2S_DL_24: u32 = (2 << NAU8824_I2S_DL_SFT);
pub const NAU8824_I2S_DL_32: u32 = (3 << NAU8824_I2S_DL_SFT);
pub const NAU8824_I2S_DF_MASK: u32 = 0x3;
pub const NAU8824_I2S_DF_RIGTH: u32 = 0;
pub const NAU8824_I2S_DF_LEFT: u32 = 1;
pub const NAU8824_I2S_DF_I2S: u32 = 2;
pub const NAU8824_I2S_DF_PCM_AB: u32 = 3;


/* PORT0_I2S_PCM_CTRL_2 (0x1D) */
pub const NAU8824_I2S_LRC_DIV_SFT: u32 = 12;
pub const NAU8824_I2S_LRC_DIV_MASK: u32 = (0x3 << NAU8824_I2S_LRC_DIV_SFT);
pub const NAU8824_I2S_MS_SFT: u32 = 3;
pub const NAU8824_I2S_MS_MASK: u32 = (1 << NAU8824_I2S_MS_SFT);
pub const NAU8824_I2S_MS_MASTER: u32 = (1 << NAU8824_I2S_MS_SFT);
pub const NAU8824_I2S_MS_SLAVE: u32 = (0 << NAU8824_I2S_MS_SFT);
pub const NAU8824_I2S_BLK_DIV_MASK: u32 = 0x7;

/* PORT0_LEFT_TIME_SLOT (0x1E) */
pub const NAU8824_TSLOT_L_MASK: u32 = 0x3ff;

/* TDM_CTRL (0x20) */
pub const NAU8824_TDM_MODE: u32 = (0x1 << 15);
pub const NAU8824_TDM_OFFSET_EN: u32 = (0x1 << 14);
pub const NAU8824_TDM_DACL_RX_SFT: u32 = 6;
pub const NAU8824_TDM_DACL_RX_MASK: u32 = (0x3 << NAU8824_TDM_DACL_RX_SFT);
pub const NAU8824_TDM_DACR_RX_SFT: u32 = 4;
pub const NAU8824_TDM_DACR_RX_MASK: u32 = (0x3 << NAU8824_TDM_DACR_RX_SFT);
pub const NAU8824_TDM_TX_MASK: u32 = 0xf;

/* ADC_FILTER_CTRL (0x24) */
pub const NAU8824_ADC_SYNC_DOWN_MASK: u32 = 0x3;
pub const NAU8824_ADC_SYNC_DOWN_32: u32 = 0;
pub const NAU8824_ADC_SYNC_DOWN_64: u32 = 1;
pub const NAU8824_ADC_SYNC_DOWN_128: u32 = 2;
pub const NAU8824_ADC_SYNC_DOWN_256: u32 = 3;

/* DAC_FILTER_CTRL_1 (0x25) */
pub const NAU8824_DAC_CICCLP_OFF: u32 = (0x1 << 7);
pub const NAU8824_DAC_OVERSAMPLE_MASK: u32 = 0x7;
pub const NAU8824_DAC_OVERSAMPLE_64: u32 = 0;
pub const NAU8824_DAC_OVERSAMPLE_256: u32 = 1;
pub const NAU8824_DAC_OVERSAMPLE_128: u32 = 2;
pub const NAU8824_DAC_OVERSAMPLE_32: u32 = 4;

/* DAC_MUTE_CTRL (0x31) */
pub const NAU8824_DAC_CH01_MIX: u32 = 0x3;
pub const NAU8824_DAC_ZC_EN: u32 = (0x1 << 11);

/* DAC_CH0_DGAIN_CTRL (0x32) */
pub const NAU8824_DAC_CH0_SEL_SFT: u32 = 9;
pub const NAU8824_DAC_CH0_SEL_MASK: u32 = (0x1 << NAU8824_DAC_CH0_SEL_SFT);
pub const NAU8824_DAC_CH0_SEL_I2S0: u32 = (0x0 << NAU8824_DAC_CH0_SEL_SFT);
pub const NAU8824_DAC_CH0_SEL_I2S1: u32 = (0x1 << NAU8824_DAC_CH0_SEL_SFT);
pub const NAU8824_DAC_CH0_VOL_MASK: u32 = 0x1ff;

/* DAC_CH1_DGAIN_CTRL (0x33) */
pub const NAU8824_DAC_CH1_SEL_SFT: u32 = 9;
pub const NAU8824_DAC_CH1_SEL_MASK: u32 = (0x1 << NAU8824_DAC_CH1_SEL_SFT);
pub const NAU8824_DAC_CH1_SEL_I2S0: u32 = (0x0 << NAU8824_DAC_CH1_SEL_SFT);
pub const NAU8824_DAC_CH1_SEL_I2S1: u32 = (0x1 << NAU8824_DAC_CH1_SEL_SFT);
pub const NAU8824_DAC_CH1_VOL_MASK: u32 = 0x1ff;

/* CLASSG (0x50) */
pub const NAU8824_CLASSG_TIMER_SFT: u32 = 8;
pub const NAU8824_CLASSG_TIMER_MASK: u32 = (0x3f << NAU8824_CLASSG_TIMER_SFT);
pub const NAU8824_CLASSG_LDAC_EN_SFT: u32 = 2;
pub const NAU8824_CLASSG_RDAC_EN_SFT: u32 = 1;
pub const NAU8824_CLASSG_EN_SFT: u32 = 0;

/* SAR_ADC_DATA_OUT (0x59) */
pub const NAU8824_SAR_ADC_DATA_MASK: u32 = 0xff;

/* BIAS_ADJ (0x66) */
pub const NAU8824_VMID: u32 = (1 << 6);
pub const NAU8824_VMID_SEL_SFT: u32 = 4;
pub const NAU8824_VMID_SEL_MASK: u32 = (3 << NAU8824_VMID_SEL_SFT);
pub const NAU8824_DMIC2_EN_SFT: u32 = 3;
pub const NAU8824_DMIC1_EN_SFT: u32 = 2;

/* TRIM_SETTINGS (0x68) */
pub const NAU8824_DRV_CURR_INC: u32 = (1 << 15);

/* ANALOG_CONTROL_1 (0x69) */
pub const NAU8824_DMIC_CLK_DRV_STRG: u32 = (1 << 3);
pub const NAU8824_DMIC_CLK_SLEW_FAST: u32 = (0x7);

/* ANALOG_CONTROL_2 (0x6A) */
pub const NAU8824_CLASSD_CLAMP_DIS_SFT: u32 = 3;
pub const NAU8824_CLASSD_CLAMP_DIS: u32 = (0x1 << NAU8824_CLASSD_CLAMP_DIS_SFT);

/* ENABLE_LO (0x6B) */
pub const NAU8824_TEST_DAC_SFT: u32 = 14;
pub const NAU8824_TEST_DAC_EN: u32 = (0x3 << NAU8824_TEST_DAC_SFT);
pub const NAU8824_DACL_HPR_EN_SFT: u32 = 3;
pub const NAU8824_DACL_HPR_EN: u32 = (0x1 << NAU8824_DACL_HPR_EN_SFT);
pub const NAU8824_DACR_HPR_EN_SFT: u32 = 2;
pub const NAU8824_DACR_HPR_EN: u32 = (0x1 << NAU8824_DACR_HPR_EN_SFT);
pub const NAU8824_DACR_HPL_EN_SFT: u32 = 1;
pub const NAU8824_DACR_HPL_EN: u32 = (0x1 << NAU8824_DACR_HPL_EN_SFT);
pub const NAU8824_DACL_HPL_EN_SFT: u32 = 0;
pub const NAU8824_DACL_HPL_EN: u32 = 0x1;

/* CLASSD_GAIN_1 (0x6D) */
pub const NAU8824_CLASSD_GAIN_1R_SFT: u32 = 8;
pub const NAU8824_CLASSD_GAIN_1R_MASK: u32 = (0x1f << NAU8824_CLASSD_GAIN_1R_SFT);
pub const NAU8824_CLASSD_EN_SFT: u32 = 7;
pub const NAU8824_CLASSD_EN: u32 = (0x1 << NAU8824_CLASSD_EN_SFT);
pub const NAU8824_CLASSD_GAIN_1L_MASK: u32 = 0x1f;

/* CLASSD_GAIN_2 (0x6E) */
pub const NAU8824_CLASSD_GAIN_2R_SFT: u32 = 8;
pub const NAU8824_CLASSD_GAIN_2R_MASK: u32 = (0x1f << NAU8824_CLASSD_GAIN_1R_SFT);
// Duplicate C macro NAU8824_CLASSD_EN_SFT retained by the earlier Rust const.
// Duplicate C macro NAU8824_CLASSD_EN retained by the earlier Rust const.
pub const NAU8824_CLASSD_GAIN_2L_MASK: u32 = 0x1f;

/* ANALOG_ADC_2 (0x72) */
pub const NAU8824_ADCR_EN_SFT: u32 = 7;
pub const NAU8824_ADCL_EN_SFT: u32 = 6;

/* RDAC (0x73) */
pub const NAU8824_DACR_EN_SFT: u32 = 13;
pub const NAU8824_DACL_EN_SFT: u32 = 12;
pub const NAU8824_DACR_CLK_SFT: u32 = 9;
pub const NAU8824_DACL_CLK_SFT: u32 = 8;
pub const NAU8824_RDAC_CLK_DELAY_SFT: u32 = 4;
pub const NAU8824_RDAC_CLK_DELAY_MASK: u32 = (0x7 << NAU8824_RDAC_CLK_DELAY_SFT);
pub const NAU8824_RDAC_VREF_SFT: u32 = 2;
pub const NAU8824_RDAC_VREF_MASK: u32 = (0x3 << NAU8824_RDAC_VREF_SFT);

/* MIC_BIAS (0x74) */
pub const NAU8824_MICBIAS_JKSLV: u32 = (1 << 14);
pub const NAU8824_MICBIAS_JKR2: u32 = (1 << 12);
pub const NAU8824_MICBIAS_POWERUP_SFT: u32 = 8;
pub const NAU8824_MICBIAS_VOLTAGE_SFT: u32 = 0;
pub const NAU8824_MICBIAS_VOLTAGE_MASK: u32 = 0x7;

/* BOOST (0x76) */
pub const NAU8824_PRECHARGE_DIS: u32 = (0x1 << 13);
pub const NAU8824_GLOBAL_BIAS_EN: u32 = (0x1 << 12);
pub const NAU8824_HP_BOOST_DIS_SFT: u32 = 9;
pub const NAU8824_HP_BOOST_DIS: u32 = (0x1 << NAU8824_HP_BOOST_DIS_SFT);
pub const NAU8824_HP_BOOST_G_DIS_SFT: u32 = 8;
pub const NAU8824_HP_BOOST_G_DIS: u32 = (0x1 << NAU8824_HP_BOOST_G_DIS_SFT);
pub const NAU8824_SHORT_SHUTDOWN_DIG_EN: u32 = (1 << 7);
pub const NAU8824_SHORT_SHUTDOWN_EN: u32 = (1 << 6);

/* FEPGA (0x77) */
pub const NAU8824_FEPGA_MODER_SHORT_SFT: u32 = 7;
pub const NAU8824_FEPGA_MODER_SHORT_EN: u32 = (0x1 << NAU8824_FEPGA_MODER_SHORT_SFT);
pub const NAU8824_FEPGA_MODER_MIC2_SFT: u32 = 5;
pub const NAU8824_FEPGA_MODER_MIC2_EN: u32 = (0x1 << NAU8824_FEPGA_MODER_MIC2_SFT);
pub const NAU8824_FEPGA_MODER_HSMIC_SFT: u32 = 4;
pub const NAU8824_FEPGA_MODER_HSMIC_EN: u32 = (0x1 << NAU8824_FEPGA_MODER_HSMIC_SFT);
pub const NAU8824_FEPGA_MODEL_SHORT_SFT: u32 = 3;
pub const NAU8824_FEPGA_MODEL_SHORT_EN: u32 = (0x1 << NAU8824_FEPGA_MODEL_SHORT_SFT);
pub const NAU8824_FEPGA_MODEL_MIC1_SFT: u32 = 1;
pub const NAU8824_FEPGA_MODEL_MIC1_EN: u32 = (0x1 << NAU8824_FEPGA_MODEL_MIC1_SFT);
pub const NAU8824_FEPGA_MODEL_HSMIC_SFT: u32 = 0;
pub const NAU8824_FEPGA_MODEL_HSMIC_EN: u32 = (0x1 << NAU8824_FEPGA_MODEL_HSMIC_SFT);

/* FEPGA_II (0x78) */
pub const NAU8824_FEPGA_GAINR_SFT: u32 = 5;
pub const NAU8824_FEPGA_GAINR_MASK: u32 = (0x1f << NAU8824_FEPGA_GAINR_SFT);
pub const NAU8824_FEPGA_GAINL_SFT: u32 = 0;
pub const NAU8824_FEPGA_GAINL_MASK: u32 = 0x1f;

/* CHARGE_PUMP_CONTROL (0x80) */
pub const NAU8824_JAMNODCLOW: u32 = (0x1 << 15);
pub const NAU8824_SPKR_PULL_DOWN: u32 = (0x1 << 13);
pub const NAU8824_SPKL_PULL_DOWN: u32 = (0x1 << 12);
pub const NAU8824_POWER_DOWN_DACR: u32 = (0x1 << 9);
pub const NAU8824_POWER_DOWN_DACL: u32 = (0x1 << 8);
pub const NAU8824_CHARGE_PUMP_EN_SFT: u32 = 5;
pub const NAU8824_CHARGE_PUMP_EN: u32 = (0x1 << NAU8824_CHARGE_PUMP_EN_SFT);


pub const NAU8824_CODEC_DAI: &str = "nau8824-hifi";

/* System Clock Source */
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Nau8824Clk {
    NAU8824_CLK_DIS = 0,
    NAU8824_CLK_MCLK = 1,
    NAU8824_CLK_INTERNAL = 2,
    NAU8824_CLK_FLL_MCLK = 3,
    NAU8824_CLK_FLL_BLK = 4,
    NAU8824_CLK_FLL_FS = 5,
}

#[repr(C)]
pub struct nau8824 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub dapm: *mut snd_soc_dapm_context,
    pub jack: *mut snd_soc_jack,
    pub jdet_work: work_struct,
    pub jd_sem: semaphore,
    pub mclk: *mut clk,
    pub fs: core::ffi::c_int,
    pub irq: core::ffi::c_int,
    pub resume_lock: core::ffi::c_int,
    pub micbias_voltage: core::ffi::c_int,
    pub vref_impedance: core::ffi::c_int,
    pub jkdet_polarity: core::ffi::c_int,
    pub sar_threshold_num: core::ffi::c_int,
    pub sar_threshold: [core::ffi::c_int; 8],
    pub sar_hysteresis: core::ffi::c_int,
    pub sar_voltage: core::ffi::c_int,
    pub sar_compare_time: core::ffi::c_int,
    pub sar_sampling_time: core::ffi::c_int,
    pub key_debounce: core::ffi::c_int,
    pub jack_eject_debounce: core::ffi::c_int,
}

#[repr(C)]
pub struct nau8824_fll {
    pub mclk_src: core::ffi::c_int,
    pub ratio: core::ffi::c_int,
    pub fll_frac: core::ffi::c_int,
    pub fll_int: core::ffi::c_int,
    pub clk_ref_div: core::ffi::c_int,
}

#[repr(C)]
pub struct nau8824_fll_attr {
    pub param: core::ffi::c_uint,
    pub val: core::ffi::c_uint,
}

#[repr(C)]
pub struct nau8824_osr_attr {
    pub osr: core::ffi::c_uint,
    pub clk_src: core::ffi::c_uint,
}


unsafe extern "C" {
    pub fn nau8824_enable_jack_detect(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
    ) -> core::ffi::c_int;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
