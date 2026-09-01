/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt5677.h  --  RT5677 ALSA SoC audio driver
 *
 * Copyright 2013 Realtek Semiconductor Corp.
 * Author: Oder Chiou <oder_chiou@realtek.com>
 */

// Dependencies from C header: <linux/gpio/driver.h>, <linux/gpio/consumer.h>




/* Info */
pub const RT5677_RESET: u32 = 0x00;
pub const RT5677_VENDOR_ID: u32 = 0xfd;
pub const RT5677_VENDOR_ID1: u32 = 0xfe;
pub const RT5677_VENDOR_ID2: u32 = 0xff;
/*  I/O - Output */
pub const RT5677_LOUT1: u32 = 0x01;
/* I/O - Input */
pub const RT5677_IN1: u32 = 0x03;
pub const RT5677_MICBIAS: u32 = 0x04;
/* I/O - SLIMBus */
pub const RT5677_SLIMBUS_PARAM: u32 = 0x07;
pub const RT5677_SLIMBUS_RX: u32 = 0x08;
pub const RT5677_SLIMBUS_CTRL: u32 = 0x09;
/* I/O */
pub const RT5677_SIDETONE_CTRL: u32 = 0x13;
/* I/O - ADC/DAC */
pub const RT5677_ANA_DAC1_2_3_SRC: u32 = 0x15;
pub const RT5677_IF_DSP_DAC3_4_MIXER: u32 = 0x16;
pub const RT5677_DAC4_DIG_VOL: u32 = 0x17;
pub const RT5677_DAC3_DIG_VOL: u32 = 0x18;
pub const RT5677_DAC1_DIG_VOL: u32 = 0x19;
pub const RT5677_DAC2_DIG_VOL: u32 = 0x1a;
pub const RT5677_IF_DSP_DAC2_MIXER: u32 = 0x1b;
pub const RT5677_STO1_ADC_DIG_VOL: u32 = 0x1c;
pub const RT5677_MONO_ADC_DIG_VOL: u32 = 0x1d;
pub const RT5677_STO1_2_ADC_BST: u32 = 0x1e;
pub const RT5677_STO2_ADC_DIG_VOL: u32 = 0x1f;
/* Mixer - D-D */
pub const RT5677_ADC_BST_CTRL2: u32 = 0x20;
pub const RT5677_STO3_4_ADC_BST: u32 = 0x21;
pub const RT5677_STO3_ADC_DIG_VOL: u32 = 0x22;
pub const RT5677_STO4_ADC_DIG_VOL: u32 = 0x23;
pub const RT5677_STO4_ADC_MIXER: u32 = 0x24;
pub const RT5677_STO3_ADC_MIXER: u32 = 0x25;
pub const RT5677_STO2_ADC_MIXER: u32 = 0x26;
pub const RT5677_STO1_ADC_MIXER: u32 = 0x27;
pub const RT5677_MONO_ADC_MIXER: u32 = 0x28;
pub const RT5677_ADC_IF_DSP_DAC1_MIXER: u32 = 0x29;
pub const RT5677_STO1_DAC_MIXER: u32 = 0x2a;
pub const RT5677_MONO_DAC_MIXER: u32 = 0x2b;
pub const RT5677_DD1_MIXER: u32 = 0x2c;
pub const RT5677_DD2_MIXER: u32 = 0x2d;
pub const RT5677_IF3_DATA: u32 = 0x2f;
pub const RT5677_IF4_DATA: u32 = 0x30;
/* Mixer - PDM */
pub const RT5677_PDM_OUT_CTRL: u32 = 0x31;
pub const RT5677_PDM_DATA_CTRL1: u32 = 0x32;
pub const RT5677_PDM_DATA_CTRL2: u32 = 0x33;
pub const RT5677_PDM1_DATA_CTRL2: u32 = 0x34;
pub const RT5677_PDM1_DATA_CTRL3: u32 = 0x35;
pub const RT5677_PDM1_DATA_CTRL4: u32 = 0x36;
pub const RT5677_PDM2_DATA_CTRL2: u32 = 0x37;
pub const RT5677_PDM2_DATA_CTRL3: u32 = 0x38;
pub const RT5677_PDM2_DATA_CTRL4: u32 = 0x39;
/* TDM */
pub const RT5677_TDM1_CTRL1: u32 = 0x3b;
pub const RT5677_TDM1_CTRL2: u32 = 0x3c;
pub const RT5677_TDM1_CTRL3: u32 = 0x3d;
pub const RT5677_TDM1_CTRL4: u32 = 0x3e;
pub const RT5677_TDM1_CTRL5: u32 = 0x3f;
pub const RT5677_TDM2_CTRL1: u32 = 0x40;
pub const RT5677_TDM2_CTRL2: u32 = 0x41;
pub const RT5677_TDM2_CTRL3: u32 = 0x42;
pub const RT5677_TDM2_CTRL4: u32 = 0x43;
pub const RT5677_TDM2_CTRL5: u32 = 0x44;
/* I2C_MASTER_CTRL */
pub const RT5677_I2C_MASTER_CTRL1: u32 = 0x47;
pub const RT5677_I2C_MASTER_CTRL2: u32 = 0x48;
pub const RT5677_I2C_MASTER_CTRL3: u32 = 0x49;
pub const RT5677_I2C_MASTER_CTRL4: u32 = 0x4a;
pub const RT5677_I2C_MASTER_CTRL5: u32 = 0x4b;
pub const RT5677_I2C_MASTER_CTRL6: u32 = 0x4c;
pub const RT5677_I2C_MASTER_CTRL7: u32 = 0x4d;
pub const RT5677_I2C_MASTER_CTRL8: u32 = 0x4e;
/* DMIC */
pub const RT5677_DMIC_CTRL1: u32 = 0x50;
pub const RT5677_DMIC_CTRL2: u32 = 0x51;
/* Haptic Generator */
pub const RT5677_HAP_GENE_CTRL1: u32 = 0x56;
pub const RT5677_HAP_GENE_CTRL2: u32 = 0x57;
pub const RT5677_HAP_GENE_CTRL3: u32 = 0x58;
pub const RT5677_HAP_GENE_CTRL4: u32 = 0x59;
pub const RT5677_HAP_GENE_CTRL5: u32 = 0x5a;
pub const RT5677_HAP_GENE_CTRL6: u32 = 0x5b;
pub const RT5677_HAP_GENE_CTRL7: u32 = 0x5c;
pub const RT5677_HAP_GENE_CTRL8: u32 = 0x5d;
pub const RT5677_HAP_GENE_CTRL9: u32 = 0x5e;
pub const RT5677_HAP_GENE_CTRL10: u32 = 0x5f;
/* Power */
pub const RT5677_PWR_DIG1: u32 = 0x61;
pub const RT5677_PWR_DIG2: u32 = 0x62;
pub const RT5677_PWR_ANLG1: u32 = 0x63;
pub const RT5677_PWR_ANLG2: u32 = 0x64;
pub const RT5677_PWR_DSP1: u32 = 0x65;
pub const RT5677_PWR_DSP_ST: u32 = 0x66;
pub const RT5677_PWR_DSP2: u32 = 0x67;
pub const RT5677_ADC_DAC_HPF_CTRL1: u32 = 0x68;
/* Private Register Control */
pub const RT5677_PRIV_INDEX: u32 = 0x6a;
pub const RT5677_PRIV_DATA: u32 = 0x6c;
/* Format - ADC/DAC */
pub const RT5677_I2S4_SDP: u32 = 0x6f;
pub const RT5677_I2S1_SDP: u32 = 0x70;
pub const RT5677_I2S2_SDP: u32 = 0x71;
pub const RT5677_I2S3_SDP: u32 = 0x72;
pub const RT5677_CLK_TREE_CTRL1: u32 = 0x73;
pub const RT5677_CLK_TREE_CTRL2: u32 = 0x74;
pub const RT5677_CLK_TREE_CTRL3: u32 = 0x75;
/* Function - Analog */
pub const RT5677_PLL1_CTRL1: u32 = 0x7a;
pub const RT5677_PLL1_CTRL2: u32 = 0x7b;
pub const RT5677_PLL2_CTRL1: u32 = 0x7c;
pub const RT5677_PLL2_CTRL2: u32 = 0x7d;
pub const RT5677_GLB_CLK1: u32 = 0x80;
pub const RT5677_GLB_CLK2: u32 = 0x81;
pub const RT5677_ASRC_1: u32 = 0x83;
pub const RT5677_ASRC_2: u32 = 0x84;
pub const RT5677_ASRC_3: u32 = 0x85;
pub const RT5677_ASRC_4: u32 = 0x86;
pub const RT5677_ASRC_5: u32 = 0x87;
pub const RT5677_ASRC_6: u32 = 0x88;
pub const RT5677_ASRC_7: u32 = 0x89;
pub const RT5677_ASRC_8: u32 = 0x8a;
pub const RT5677_ASRC_9: u32 = 0x8b;
pub const RT5677_ASRC_10: u32 = 0x8c;
pub const RT5677_ASRC_11: u32 = 0x8d;
pub const RT5677_ASRC_12: u32 = 0x8e;
pub const RT5677_ASRC_13: u32 = 0x8f;
pub const RT5677_ASRC_14: u32 = 0x90;
pub const RT5677_ASRC_15: u32 = 0x91;
pub const RT5677_ASRC_16: u32 = 0x92;
pub const RT5677_ASRC_17: u32 = 0x93;
pub const RT5677_ASRC_18: u32 = 0x94;
pub const RT5677_ASRC_19: u32 = 0x95;
pub const RT5677_ASRC_20: u32 = 0x97;
pub const RT5677_ASRC_21: u32 = 0x98;
pub const RT5677_ASRC_22: u32 = 0x99;
pub const RT5677_ASRC_23: u32 = 0x9a;
pub const RT5677_VAD_CTRL1: u32 = 0x9c;
pub const RT5677_VAD_CTRL2: u32 = 0x9d;
pub const RT5677_VAD_CTRL3: u32 = 0x9e;
pub const RT5677_VAD_CTRL4: u32 = 0x9f;
pub const RT5677_VAD_CTRL5: u32 = 0xa0;
/* Function - Digital */
pub const RT5677_DSP_INB_CTRL1: u32 = 0xa3;
pub const RT5677_DSP_INB_CTRL2: u32 = 0xa4;
pub const RT5677_DSP_IN_OUTB_CTRL: u32 = 0xa5;
pub const RT5677_DSP_OUTB0_1_DIG_VOL: u32 = 0xa6;
pub const RT5677_DSP_OUTB2_3_DIG_VOL: u32 = 0xa7;
pub const RT5677_DSP_OUTB4_5_DIG_VOL: u32 = 0xa8;
pub const RT5677_DSP_OUTB6_7_DIG_VOL: u32 = 0xa9;
pub const RT5677_ADC_EQ_CTRL1: u32 = 0xae;
pub const RT5677_ADC_EQ_CTRL2: u32 = 0xaf;
pub const RT5677_EQ_CTRL1: u32 = 0xb0;
pub const RT5677_EQ_CTRL2: u32 = 0xb1;
pub const RT5677_EQ_CTRL3: u32 = 0xb2;
pub const RT5677_SOFT_VOL_ZERO_CROSS1: u32 = 0xb3;
pub const RT5677_JD_CTRL1: u32 = 0xb5;
pub const RT5677_JD_CTRL2: u32 = 0xb6;
pub const RT5677_JD_CTRL3: u32 = 0xb8;
pub const RT5677_IRQ_CTRL1: u32 = 0xbd;
pub const RT5677_IRQ_CTRL2: u32 = 0xbe;
pub const RT5677_GPIO_ST: u32 = 0xbf;
pub const RT5677_GPIO_CTRL1: u32 = 0xc0;
pub const RT5677_GPIO_CTRL2: u32 = 0xc1;
pub const RT5677_GPIO_CTRL3: u32 = 0xc2;
pub const RT5677_STO1_ADC_HI_FILTER1: u32 = 0xc5;
pub const RT5677_STO1_ADC_HI_FILTER2: u32 = 0xc6;
pub const RT5677_MONO_ADC_HI_FILTER1: u32 = 0xc7;
pub const RT5677_MONO_ADC_HI_FILTER2: u32 = 0xc8;
pub const RT5677_STO2_ADC_HI_FILTER1: u32 = 0xc9;
pub const RT5677_STO2_ADC_HI_FILTER2: u32 = 0xca;
pub const RT5677_STO3_ADC_HI_FILTER1: u32 = 0xcb;
pub const RT5677_STO3_ADC_HI_FILTER2: u32 = 0xcc;
pub const RT5677_STO4_ADC_HI_FILTER1: u32 = 0xcd;
pub const RT5677_STO4_ADC_HI_FILTER2: u32 = 0xce;
pub const RT5677_MB_DRC_CTRL1: u32 = 0xd0;
pub const RT5677_DRC1_CTRL1: u32 = 0xd2;
pub const RT5677_DRC1_CTRL2: u32 = 0xd3;
pub const RT5677_DRC1_CTRL3: u32 = 0xd4;
pub const RT5677_DRC1_CTRL4: u32 = 0xd5;
pub const RT5677_DRC1_CTRL5: u32 = 0xd6;
pub const RT5677_DRC1_CTRL6: u32 = 0xd7;
pub const RT5677_DRC2_CTRL1: u32 = 0xd8;
pub const RT5677_DRC2_CTRL2: u32 = 0xd9;
pub const RT5677_DRC2_CTRL3: u32 = 0xda;
pub const RT5677_DRC2_CTRL4: u32 = 0xdb;
pub const RT5677_DRC2_CTRL5: u32 = 0xdc;
pub const RT5677_DRC2_CTRL6: u32 = 0xdd;
pub const RT5677_DRC1_HL_CTRL1: u32 = 0xde;
pub const RT5677_DRC1_HL_CTRL2: u32 = 0xdf;
pub const RT5677_DRC2_HL_CTRL1: u32 = 0xe0;
pub const RT5677_DRC2_HL_CTRL2: u32 = 0xe1;
pub const RT5677_DSP_INB1_SRC_CTRL1: u32 = 0xe3;
pub const RT5677_DSP_INB1_SRC_CTRL2: u32 = 0xe4;
pub const RT5677_DSP_INB1_SRC_CTRL3: u32 = 0xe5;
pub const RT5677_DSP_INB1_SRC_CTRL4: u32 = 0xe6;
pub const RT5677_DSP_INB2_SRC_CTRL1: u32 = 0xe7;
pub const RT5677_DSP_INB2_SRC_CTRL2: u32 = 0xe8;
pub const RT5677_DSP_INB2_SRC_CTRL3: u32 = 0xe9;
pub const RT5677_DSP_INB2_SRC_CTRL4: u32 = 0xea;
pub const RT5677_DSP_INB3_SRC_CTRL1: u32 = 0xeb;
pub const RT5677_DSP_INB3_SRC_CTRL2: u32 = 0xec;
pub const RT5677_DSP_INB3_SRC_CTRL3: u32 = 0xed;
pub const RT5677_DSP_INB3_SRC_CTRL4: u32 = 0xee;
pub const RT5677_DSP_OUTB1_SRC_CTRL1: u32 = 0xef;
pub const RT5677_DSP_OUTB1_SRC_CTRL2: u32 = 0xf0;
pub const RT5677_DSP_OUTB1_SRC_CTRL3: u32 = 0xf1;
pub const RT5677_DSP_OUTB1_SRC_CTRL4: u32 = 0xf2;
pub const RT5677_DSP_OUTB2_SRC_CTRL1: u32 = 0xf3;
pub const RT5677_DSP_OUTB2_SRC_CTRL2: u32 = 0xf4;
pub const RT5677_DSP_OUTB2_SRC_CTRL3: u32 = 0xf5;
pub const RT5677_DSP_OUTB2_SRC_CTRL4: u32 = 0xf6;

/* Virtual DSP Mixer Control */
pub const RT5677_DSP_OUTB_0123_MIXER_CTRL: u32 = 0xf7;
pub const RT5677_DSP_OUTB_45_MIXER_CTRL: u32 = 0xf8;
pub const RT5677_DSP_OUTB_67_MIXER_CTRL: u32 = 0xf9;

/* General Control */
pub const RT5677_DIG_MISC: u32 = 0xfa;
pub const RT5677_GEN_CTRL1: u32 = 0xfb;
pub const RT5677_GEN_CTRL2: u32 = 0xfc;

/* DSP Mode I2C Control*/
pub const RT5677_DSP_I2C_OP_CODE: u32 = 0x00;
pub const RT5677_DSP_I2C_ADDR_LSB: u32 = 0x01;
pub const RT5677_DSP_I2C_ADDR_MSB: u32 = 0x02;
pub const RT5677_DSP_I2C_DATA_LSB: u32 = 0x03;
pub const RT5677_DSP_I2C_DATA_MSB: u32 = 0x04;

/* Index of Codec Private Register definition */
pub const RT5677_PR_DRC1_CTRL_1: u32 = 0x01;
pub const RT5677_PR_DRC1_CTRL_2: u32 = 0x02;
pub const RT5677_PR_DRC1_CTRL_3: u32 = 0x03;
pub const RT5677_PR_DRC1_CTRL_4: u32 = 0x04;
pub const RT5677_PR_DRC1_CTRL_5: u32 = 0x05;
pub const RT5677_PR_DRC1_CTRL_6: u32 = 0x06;
pub const RT5677_PR_DRC1_CTRL_7: u32 = 0x07;
pub const RT5677_PR_DRC2_CTRL_1: u32 = 0x08;
pub const RT5677_PR_DRC2_CTRL_2: u32 = 0x09;
pub const RT5677_PR_DRC2_CTRL_3: u32 = 0x0a;
pub const RT5677_PR_DRC2_CTRL_4: u32 = 0x0b;
pub const RT5677_PR_DRC2_CTRL_5: u32 = 0x0c;
pub const RT5677_PR_DRC2_CTRL_6: u32 = 0x0d;
pub const RT5677_PR_DRC2_CTRL_7: u32 = 0x0e;
pub const RT5677_BIAS_CUR1: u32 = 0x10;
pub const RT5677_BIAS_CUR2: u32 = 0x12;
pub const RT5677_BIAS_CUR3: u32 = 0x13;
pub const RT5677_BIAS_CUR4: u32 = 0x14;
pub const RT5677_BIAS_CUR5: u32 = 0x15;
pub const RT5677_VREF_LOUT_CTRL: u32 = 0x17;
pub const RT5677_DIG_VOL_CTRL1: u32 = 0x1a;
pub const RT5677_DIG_VOL_CTRL2: u32 = 0x1b;
pub const RT5677_ANA_ADC_GAIN_CTRL: u32 = 0x1e;
pub const RT5677_VAD_SRAM_TEST1: u32 = 0x20;
pub const RT5677_VAD_SRAM_TEST2: u32 = 0x21;
pub const RT5677_VAD_SRAM_TEST3: u32 = 0x22;
pub const RT5677_VAD_SRAM_TEST4: u32 = 0x23;
pub const RT5677_PAD_DRV_CTRL: u32 = 0x26;
pub const RT5677_DIG_IN_PIN_ST_CTRL1: u32 = 0x29;
pub const RT5677_DIG_IN_PIN_ST_CTRL2: u32 = 0x2a;
pub const RT5677_DIG_IN_PIN_ST_CTRL3: u32 = 0x2b;
pub const RT5677_PLL1_INT: u32 = 0x38;
pub const RT5677_PLL2_INT: u32 = 0x39;
pub const RT5677_TEST_CTRL1: u32 = 0x3a;
pub const RT5677_TEST_CTRL2: u32 = 0x3b;
pub const RT5677_TEST_CTRL3: u32 = 0x3c;
pub const RT5677_CHOP_DAC_ADC: u32 = 0x3d;
pub const RT5677_SOFT_DEPOP_DAC_CLK_CTRL: u32 = 0x3e;
pub const RT5677_CROSS_OVER_FILTER1: u32 = 0x90;
pub const RT5677_CROSS_OVER_FILTER2: u32 = 0x91;
pub const RT5677_CROSS_OVER_FILTER3: u32 = 0x92;
pub const RT5677_CROSS_OVER_FILTER4: u32 = 0x93;
pub const RT5677_CROSS_OVER_FILTER5: u32 = 0x94;
pub const RT5677_CROSS_OVER_FILTER6: u32 = 0x95;
pub const RT5677_CROSS_OVER_FILTER7: u32 = 0x96;
pub const RT5677_CROSS_OVER_FILTER8: u32 = 0x97;
pub const RT5677_CROSS_OVER_FILTER9: u32 = 0x98;
pub const RT5677_CROSS_OVER_FILTER10: u32 = 0x99;

/* global definition */
pub const RT5677_L_MUTE: u32 = (0x1 << 15);
pub const RT5677_L_MUTE_SFT: u32 = 15;
pub const RT5677_VOL_L_MUTE: u32 = (0x1 << 14);
pub const RT5677_VOL_L_SFT: u32 = 14;
pub const RT5677_R_MUTE: u32 = (0x1 << 7);
pub const RT5677_R_MUTE_SFT: u32 = 7;
pub const RT5677_VOL_R_MUTE: u32 = (0x1 << 6);
pub const RT5677_VOL_R_SFT: u32 = 6;
pub const RT5677_L_VOL_MASK: u32 = (0x7f << 9);
pub const RT5677_L_VOL_SFT: u32 = 9;
pub const RT5677_R_VOL_MASK: u32 = (0x7f << 1);
pub const RT5677_R_VOL_SFT: u32 = 1;

/* LOUT1 Control (0x01) */
pub const RT5677_LOUT1_L_MUTE: u32 = (0x1 << 15);
pub const RT5677_LOUT1_L_MUTE_SFT: u32 = (15);
pub const RT5677_LOUT1_L_DF: u32 = (0x1 << 14);
pub const RT5677_LOUT1_L_DF_SFT: u32 = (14);
pub const RT5677_LOUT2_L_MUTE: u32 = (0x1 << 13);
pub const RT5677_LOUT2_L_MUTE_SFT: u32 = (13);
pub const RT5677_LOUT2_L_DF: u32 = (0x1 << 12);
pub const RT5677_LOUT2_L_DF_SFT: u32 = (12);
pub const RT5677_LOUT3_L_MUTE: u32 = (0x1 << 11);
pub const RT5677_LOUT3_L_MUTE_SFT: u32 = (11);
pub const RT5677_LOUT3_L_DF: u32 = (0x1 << 10);
pub const RT5677_LOUT3_L_DF_SFT: u32 = (10);
pub const RT5677_LOUT1_ENH_DRV: u32 = (0x1 << 9);
pub const RT5677_LOUT1_ENH_DRV_SFT: u32 = (9);
pub const RT5677_LOUT2_ENH_DRV: u32 = (0x1 << 8);
pub const RT5677_LOUT2_ENH_DRV_SFT: u32 = (8);
pub const RT5677_LOUT3_ENH_DRV: u32 = (0x1 << 7);
pub const RT5677_LOUT3_ENH_DRV_SFT: u32 = (7);

/* IN1 Control (0x03) */
pub const RT5677_BST_MASK1: u32 = (0xf << 12);
pub const RT5677_BST_SFT1: u32 = 12;
pub const RT5677_BST_MASK2: u32 = (0xf << 8);
pub const RT5677_BST_SFT2: u32 = 8;
pub const RT5677_IN_DF1: u32 = (0x1 << 7);
pub const RT5677_IN_DF1_SFT: u32 = 7;
pub const RT5677_IN_DF2: u32 = (0x1 << 6);
pub const RT5677_IN_DF2_SFT: u32 = 6;

/* Micbias Control (0x04) */
pub const RT5677_MICBIAS1_OUTVOLT_MASK: u32 = (0x1 << 15);
pub const RT5677_MICBIAS1_OUTVOLT_SFT: u32 = (15);
pub const RT5677_MICBIAS1_OUTVOLT_2_7V: u32 = (0x0 << 15);
pub const RT5677_MICBIAS1_OUTVOLT_2_25V: u32 = (0x1 << 15);
pub const RT5677_MICBIAS1_CTRL_VDD_MASK: u32 = (0x1 << 14);
pub const RT5677_MICBIAS1_CTRL_VDD_SFT: u32 = (14);
pub const RT5677_MICBIAS1_CTRL_VDD_1_8V: u32 = (0x0 << 14);
pub const RT5677_MICBIAS1_CTRL_VDD_3_3V: u32 = (0x1 << 14);
pub const RT5677_MICBIAS1_OVCD_MASK: u32 = (0x1 << 11);
pub const RT5677_MICBIAS1_OVCD_SHIFT: u32 = (11);
pub const RT5677_MICBIAS1_OVCD_DIS: u32 = (0x0 << 11);
pub const RT5677_MICBIAS1_OVCD_EN: u32 = (0x1 << 11);
pub const RT5677_MICBIAS1_OVTH_MASK: u32 = (0x3 << 9);
pub const RT5677_MICBIAS1_OVTH_SFT: u32 = 9;
pub const RT5677_MICBIAS1_OVTH_640UA: u32 = (0x0 << 9);
pub const RT5677_MICBIAS1_OVTH_1280UA: u32 = (0x1 << 9);
pub const RT5677_MICBIAS1_OVTH_1920UA: u32 = (0x2 << 9);

/* SLIMbus Parameter (0x07) */

/* SLIMbus Rx (0x08) */
pub const RT5677_SLB_ADC4_MASK: u32 = (0x3 << 6);
pub const RT5677_SLB_ADC4_SFT: u32 = 6;
pub const RT5677_SLB_ADC3_MASK: u32 = (0x3 << 4);
pub const RT5677_SLB_ADC3_SFT: u32 = 4;
pub const RT5677_SLB_ADC2_MASK: u32 = (0x3 << 2);
pub const RT5677_SLB_ADC2_SFT: u32 = 2;
pub const RT5677_SLB_ADC1_MASK: u32 = (0x3 << 0);
pub const RT5677_SLB_ADC1_SFT: u32 = 0;

/* SLIMBus control (0x09) */

/* Sidetone Control (0x13) */
pub const RT5677_ST_HPF_SEL_MASK: u32 = (0x7 << 13);
pub const RT5677_ST_HPF_SEL_SFT: u32 = 13;
pub const RT5677_ST_HPF_PATH: u32 = (0x1 << 12);
pub const RT5677_ST_HPF_PATH_SFT: u32 = 12;
pub const RT5677_ST_SEL_MASK: u32 = (0x7 << 9);
pub const RT5677_ST_SEL_SFT: u32 = 9;
pub const RT5677_ST_EN: u32 = (0x1 << 6);
pub const RT5677_ST_EN_SFT: u32 = 6;
pub const RT5677_ST_GAIN: u32 = (0x1 << 5);
pub const RT5677_ST_GAIN_SFT: u32 = 5;
pub const RT5677_ST_VOL_MASK: u32 = (0x1f << 0);
pub const RT5677_ST_VOL_SFT: u32 = 0;

/* Analog DAC1/2/3 Source Control (0x15) */
pub const RT5677_ANA_DAC3_SRC_SEL_MASK: u32 = (0x3 << 4);
pub const RT5677_ANA_DAC3_SRC_SEL_SFT: u32 = 4;
pub const RT5677_ANA_DAC1_2_SRC_SEL_MASK: u32 = (0x3 << 0);
pub const RT5677_ANA_DAC1_2_SRC_SEL_SFT: u32 = 0;

/* IF/DSP to DAC3/4 Mixer Control (0x16) */
pub const RT5677_M_DAC4_L_VOL: u32 = (0x1 << 15);
pub const RT5677_M_DAC4_L_VOL_SFT: u32 = 15;
pub const RT5677_SEL_DAC4_L_SRC_MASK: u32 = (0x7 << 12);
pub const RT5677_SEL_DAC4_L_SRC_SFT: u32 = 12;
pub const RT5677_M_DAC4_R_VOL: u32 = (0x1 << 11);
pub const RT5677_M_DAC4_R_VOL_SFT: u32 = 11;
pub const RT5677_SEL_DAC4_R_SRC_MASK: u32 = (0x7 << 8);
pub const RT5677_SEL_DAC4_R_SRC_SFT: u32 = 8;
pub const RT5677_M_DAC3_L_VOL: u32 = (0x1 << 7);
pub const RT5677_M_DAC3_L_VOL_SFT: u32 = 7;
pub const RT5677_SEL_DAC3_L_SRC_MASK: u32 = (0x7 << 4);
pub const RT5677_SEL_DAC3_L_SRC_SFT: u32 = 4;
pub const RT5677_M_DAC3_R_VOL: u32 = (0x1 << 3);
pub const RT5677_M_DAC3_R_VOL_SFT: u32 = 3;
pub const RT5677_SEL_DAC3_R_SRC_MASK: u32 = (0x7 << 0);
pub const RT5677_SEL_DAC3_R_SRC_SFT: u32 = 0;

/* DAC4 Digital Volume (0x17) */
pub const RT5677_DAC4_L_VOL_MASK: u32 = (0xff << 8);
pub const RT5677_DAC4_L_VOL_SFT: u32 = 8;
pub const RT5677_DAC4_R_VOL_MASK: u32 = (0xff);
pub const RT5677_DAC4_R_VOL_SFT: u32 = 0;

/* DAC3 Digital Volume (0x18) */
pub const RT5677_DAC3_L_VOL_MASK: u32 = (0xff << 8);
pub const RT5677_DAC3_L_VOL_SFT: u32 = 8;
pub const RT5677_DAC3_R_VOL_MASK: u32 = (0xff);
pub const RT5677_DAC3_R_VOL_SFT: u32 = 0;

/* DAC1 Digital Volume (0x19) */
pub const RT5677_DAC1_L_VOL_MASK: u32 = (0xff << 8);
pub const RT5677_DAC1_L_VOL_SFT: u32 = 8;
pub const RT5677_DAC1_R_VOL_MASK: u32 = (0xff);
pub const RT5677_DAC1_R_VOL_SFT: u32 = 0;

/* DAC2 Digital Volume (0x1a) */
pub const RT5677_DAC2_L_VOL_MASK: u32 = (0xff << 8);
pub const RT5677_DAC2_L_VOL_SFT: u32 = 8;
pub const RT5677_DAC2_R_VOL_MASK: u32 = (0xff);
pub const RT5677_DAC2_R_VOL_SFT: u32 = 0;

/* IF/DSP to DAC2 Mixer Control (0x1b) */
pub const RT5677_M_DAC2_L_VOL: u32 = (0x1 << 7);
pub const RT5677_M_DAC2_L_VOL_SFT: u32 = 7;
pub const RT5677_SEL_DAC2_L_SRC_MASK: u32 = (0x7 << 4);
pub const RT5677_SEL_DAC2_L_SRC_SFT: u32 = 4;
pub const RT5677_M_DAC2_R_VOL: u32 = (0x1 << 3);
pub const RT5677_M_DAC2_R_VOL_SFT: u32 = 3;
pub const RT5677_SEL_DAC2_R_SRC_MASK: u32 = (0x7 << 0);
pub const RT5677_SEL_DAC2_R_SRC_SFT: u32 = 0;

/* Stereo1 ADC Digital Volume Control (0x1c) */
pub const RT5677_STO1_ADC_L_VOL_MASK: u32 = (0x3f << 9);
pub const RT5677_STO1_ADC_L_VOL_SFT: u32 = 9;
pub const RT5677_STO1_ADC_R_VOL_MASK: u32 = (0x3f << 1);
pub const RT5677_STO1_ADC_R_VOL_SFT: u32 = 1;

/* Mono ADC Digital Volume Control (0x1d) */
pub const RT5677_MONO_ADC_L_VOL_MASK: u32 = (0x3f << 9);
pub const RT5677_MONO_ADC_L_VOL_SFT: u32 = 9;
pub const RT5677_MONO_ADC_R_VOL_MASK: u32 = (0x3f << 1);
pub const RT5677_MONO_ADC_R_VOL_SFT: u32 = 1;

/* Stereo 1/2 ADC Boost Gain Control (0x1e) */
pub const RT5677_STO1_ADC_L_BST_MASK: u32 = (0x3 << 14);
pub const RT5677_STO1_ADC_L_BST_SFT: u32 = 14;
pub const RT5677_STO1_ADC_R_BST_MASK: u32 = (0x3 << 12);
pub const RT5677_STO1_ADC_R_BST_SFT: u32 = 12;
pub const RT5677_STO1_ADC_COMP_MASK: u32 = (0x3 << 10);
pub const RT5677_STO1_ADC_COMP_SFT: u32 = 10;
pub const RT5677_STO2_ADC_L_BST_MASK: u32 = (0x3 << 8);
pub const RT5677_STO2_ADC_L_BST_SFT: u32 = 8;
pub const RT5677_STO2_ADC_R_BST_MASK: u32 = (0x3 << 6);
pub const RT5677_STO2_ADC_R_BST_SFT: u32 = 6;
pub const RT5677_STO2_ADC_COMP_MASK: u32 = (0x3 << 4);
pub const RT5677_STO2_ADC_COMP_SFT: u32 = 4;

/* Stereo2 ADC Digital Volume Control (0x1f) */
pub const RT5677_STO2_ADC_L_VOL_MASK: u32 = (0x7f << 8);
pub const RT5677_STO2_ADC_L_VOL_SFT: u32 = 8;
pub const RT5677_STO2_ADC_R_VOL_MASK: u32 = (0x7f);
pub const RT5677_STO2_ADC_R_VOL_SFT: u32 = 0;

/* ADC Boost Gain Control 2 (0x20) */
pub const RT5677_MONO_ADC_L_BST_MASK: u32 = (0x3 << 14);
pub const RT5677_MONO_ADC_L_BST_SFT: u32 = 14;
pub const RT5677_MONO_ADC_R_BST_MASK: u32 = (0x3 << 12);
pub const RT5677_MONO_ADC_R_BST_SFT: u32 = 12;
pub const RT5677_MONO_ADC_COMP_MASK: u32 = (0x3 << 10);
pub const RT5677_MONO_ADC_COMP_SFT: u32 = 10;

/* Stereo 3/4 ADC Boost Gain Control (0x21) */
pub const RT5677_STO3_ADC_L_BST_MASK: u32 = (0x3 << 14);
pub const RT5677_STO3_ADC_L_BST_SFT: u32 = 14;
pub const RT5677_STO3_ADC_R_BST_MASK: u32 = (0x3 << 12);
pub const RT5677_STO3_ADC_R_BST_SFT: u32 = 12;
pub const RT5677_STO3_ADC_COMP_MASK: u32 = (0x3 << 10);
pub const RT5677_STO3_ADC_COMP_SFT: u32 = 10;
pub const RT5677_STO4_ADC_L_BST_MASK: u32 = (0x3 << 8);
pub const RT5677_STO4_ADC_L_BST_SFT: u32 = 8;
pub const RT5677_STO4_ADC_R_BST_MASK: u32 = (0x3 << 6);
pub const RT5677_STO4_ADC_R_BST_SFT: u32 = 6;
pub const RT5677_STO4_ADC_COMP_MASK: u32 = (0x3 << 4);
pub const RT5677_STO4_ADC_COMP_SFT: u32 = 4;

/* Stereo3 ADC Digital Volume Control (0x22) */
pub const RT5677_STO3_ADC_L_VOL_MASK: u32 = (0x7f << 8);
pub const RT5677_STO3_ADC_L_VOL_SFT: u32 = 8;
pub const RT5677_STO3_ADC_R_VOL_MASK: u32 = (0x7f);
pub const RT5677_STO3_ADC_R_VOL_SFT: u32 = 0;

/* Stereo4 ADC Digital Volume Control (0x23) */
pub const RT5677_STO4_ADC_L_VOL_MASK: u32 = (0x7f << 8);
pub const RT5677_STO4_ADC_L_VOL_SFT: u32 = 8;
pub const RT5677_STO4_ADC_R_VOL_MASK: u32 = (0x7f);
pub const RT5677_STO4_ADC_R_VOL_SFT: u32 = 0;

/* Stereo4 ADC Mixer control (0x24) */
pub const RT5677_M_STO4_ADC_L2: u32 = (0x1 << 15);
pub const RT5677_M_STO4_ADC_L2_SFT: u32 = 15;
pub const RT5677_M_STO4_ADC_L1: u32 = (0x1 << 14);
pub const RT5677_M_STO4_ADC_L1_SFT: u32 = 14;
pub const RT5677_SEL_STO4_ADC1_MASK: u32 = (0x3 << 12);
pub const RT5677_SEL_STO4_ADC1_SFT: u32 = 12;
pub const RT5677_SEL_STO4_ADC2_MASK: u32 = (0x3 << 10);
pub const RT5677_SEL_STO4_ADC2_SFT: u32 = 10;
pub const RT5677_SEL_STO4_DMIC_MASK: u32 = (0x3 << 8);
pub const RT5677_SEL_STO4_DMIC_SFT: u32 = 8;
pub const RT5677_M_STO4_ADC_R1: u32 = (0x1 << 7);
pub const RT5677_M_STO4_ADC_R1_SFT: u32 = 7;
pub const RT5677_M_STO4_ADC_R2: u32 = (0x1 << 6);
pub const RT5677_M_STO4_ADC_R2_SFT: u32 = 6;

/* Stereo3 ADC Mixer control (0x25) */
pub const RT5677_M_STO3_ADC_L2: u32 = (0x1 << 15);
pub const RT5677_M_STO3_ADC_L2_SFT: u32 = 15;
pub const RT5677_M_STO3_ADC_L1: u32 = (0x1 << 14);
pub const RT5677_M_STO3_ADC_L1_SFT: u32 = 14;
pub const RT5677_SEL_STO3_ADC1_MASK: u32 = (0x3 << 12);
pub const RT5677_SEL_STO3_ADC1_SFT: u32 = 12;
pub const RT5677_SEL_STO3_ADC2_MASK: u32 = (0x3 << 10);
pub const RT5677_SEL_STO3_ADC2_SFT: u32 = 10;
pub const RT5677_SEL_STO3_DMIC_MASK: u32 = (0x3 << 8);
pub const RT5677_SEL_STO3_DMIC_SFT: u32 = 8;
pub const RT5677_M_STO3_ADC_R1: u32 = (0x1 << 7);
pub const RT5677_M_STO3_ADC_R1_SFT: u32 = 7;
pub const RT5677_M_STO3_ADC_R2: u32 = (0x1 << 6);
pub const RT5677_M_STO3_ADC_R2_SFT: u32 = 6;

/* Stereo2 ADC Mixer Control (0x26) */
pub const RT5677_M_STO2_ADC_L2: u32 = (0x1 << 15);
pub const RT5677_M_STO2_ADC_L2_SFT: u32 = 15;
pub const RT5677_M_STO2_ADC_L1: u32 = (0x1 << 14);
pub const RT5677_M_STO2_ADC_L1_SFT: u32 = 14;
pub const RT5677_SEL_STO2_ADC1_MASK: u32 = (0x3 << 12);
pub const RT5677_SEL_STO2_ADC1_SFT: u32 = 12;
pub const RT5677_SEL_STO2_ADC2_MASK: u32 = (0x3 << 10);
pub const RT5677_SEL_STO2_ADC2_SFT: u32 = 10;
pub const RT5677_SEL_STO2_DMIC_MASK: u32 = (0x3 << 8);
pub const RT5677_SEL_STO2_DMIC_SFT: u32 = 8;
pub const RT5677_M_STO2_ADC_R1: u32 = (0x1 << 7);
pub const RT5677_M_STO2_ADC_R1_SFT: u32 = 7;
pub const RT5677_M_STO2_ADC_R2: u32 = (0x1 << 6);
pub const RT5677_M_STO2_ADC_R2_SFT: u32 = 6;
pub const RT5677_SEL_STO2_LR_MIX_MASK: u32 = (0x1 << 0);
pub const RT5677_SEL_STO2_LR_MIX_SFT: u32 = 0;
pub const RT5677_SEL_STO2_LR_MIX_L: u32 = (0x0 << 0);
pub const RT5677_SEL_STO2_LR_MIX_LR: u32 = (0x1 << 0);

/* Stereo1 ADC Mixer control (0x27) */
pub const RT5677_M_STO1_ADC_L2: u32 = (0x1 << 15);
pub const RT5677_M_STO1_ADC_L2_SFT: u32 = 15;
pub const RT5677_M_STO1_ADC_L1: u32 = (0x1 << 14);
pub const RT5677_M_STO1_ADC_L1_SFT: u32 = 14;
pub const RT5677_SEL_STO1_ADC1_MASK: u32 = (0x3 << 12);
pub const RT5677_SEL_STO1_ADC1_SFT: u32 = 12;
pub const RT5677_SEL_STO1_ADC2_MASK: u32 = (0x3 << 10);
pub const RT5677_SEL_STO1_ADC2_SFT: u32 = 10;
pub const RT5677_SEL_STO1_DMIC_MASK: u32 = (0x3 << 8);
pub const RT5677_SEL_STO1_DMIC_SFT: u32 = 8;
pub const RT5677_M_STO1_ADC_R1: u32 = (0x1 << 7);
pub const RT5677_M_STO1_ADC_R1_SFT: u32 = 7;
pub const RT5677_M_STO1_ADC_R2: u32 = (0x1 << 6);
pub const RT5677_M_STO1_ADC_R2_SFT: u32 = 6;

/* Mono ADC Mixer control (0x28) */
pub const RT5677_M_MONO_ADC_L2: u32 = (0x1 << 15);
pub const RT5677_M_MONO_ADC_L2_SFT: u32 = 15;
pub const RT5677_M_MONO_ADC_L1: u32 = (0x1 << 14);
pub const RT5677_M_MONO_ADC_L1_SFT: u32 = 14;
pub const RT5677_SEL_MONO_ADC_L1_MASK: u32 = (0x3 << 12);
pub const RT5677_SEL_MONO_ADC_L1_SFT: u32 = 12;
pub const RT5677_SEL_MONO_ADC_L2_MASK: u32 = (0x3 << 10);
pub const RT5677_SEL_MONO_ADC_L2_SFT: u32 = 10;
pub const RT5677_SEL_MONO_DMIC_L_MASK: u32 = (0x3 << 8);
pub const RT5677_SEL_MONO_DMIC_L_SFT: u32 = 8;
pub const RT5677_M_MONO_ADC_R1: u32 = (0x1 << 7);
pub const RT5677_M_MONO_ADC_R1_SFT: u32 = 7;
pub const RT5677_M_MONO_ADC_R2: u32 = (0x1 << 6);
pub const RT5677_M_MONO_ADC_R2_SFT: u32 = 6;
pub const RT5677_SEL_MONO_ADC_R1_MASK: u32 = (0x3 << 4);
pub const RT5677_SEL_MONO_ADC_R1_SFT: u32 = 4;
pub const RT5677_SEL_MONO_ADC_R2_MASK: u32 = (0x3 << 2);
pub const RT5677_SEL_MONO_ADC_R2_SFT: u32 = 2;
pub const RT5677_SEL_MONO_DMIC_R_MASK: u32 = (0x3 << 0);
pub const RT5677_SEL_MONO_DMIC_R_SFT: u32 = 0;

/* ADC/IF/DSP to DAC1 Mixer control (0x29) */
pub const RT5677_M_ADDA_MIXER1_L: u32 = (0x1 << 15);
pub const RT5677_M_ADDA_MIXER1_L_SFT: u32 = 15;
pub const RT5677_M_DAC1_L: u32 = (0x1 << 14);
pub const RT5677_M_DAC1_L_SFT: u32 = 14;
pub const RT5677_DAC1_L_SEL_MASK: u32 = (0x7 << 8);
pub const RT5677_DAC1_L_SEL_SFT: u32 = 8;
pub const RT5677_M_ADDA_MIXER1_R: u32 = (0x1 << 7);
pub const RT5677_M_ADDA_MIXER1_R_SFT: u32 = 7;
pub const RT5677_M_DAC1_R: u32 = (0x1 << 6);
pub const RT5677_M_DAC1_R_SFT: u32 = 6;
pub const RT5677_ADDA1_SEL_MASK: u32 = (0x3 << 0);
pub const RT5677_ADDA1_SEL_SFT: u32 = 0;

/* Stereo1 DAC Mixer L/R Control (0x2a) */
pub const RT5677_M_ST_DAC1_L: u32 = (0x1 << 15);
pub const RT5677_M_ST_DAC1_L_SFT: u32 = 15;
pub const RT5677_M_DAC1_L_STO_L: u32 = (0x1 << 13);
pub const RT5677_M_DAC1_L_STO_L_SFT: u32 = 13;
pub const RT5677_DAC1_L_STO_L_VOL_MASK: u32 = (0x1 << 12);
pub const RT5677_DAC1_L_STO_L_VOL_SFT: u32 = 12;
pub const RT5677_M_DAC2_L_STO_L: u32 = (0x1 << 11);
pub const RT5677_M_DAC2_L_STO_L_SFT: u32 = 11;
pub const RT5677_DAC2_L_STO_L_VOL_MASK: u32 = (0x1 << 10);
pub const RT5677_DAC2_L_STO_L_VOL_SFT: u32 = 10;
pub const RT5677_M_DAC1_R_STO_L: u32 = (0x1 << 9);
pub const RT5677_M_DAC1_R_STO_L_SFT: u32 = 9;
pub const RT5677_DAC1_R_STO_L_VOL_MASK: u32 = (0x1 << 8);
pub const RT5677_DAC1_R_STO_L_VOL_SFT: u32 = 8;
pub const RT5677_M_ST_DAC1_R: u32 = (0x1 << 7);
pub const RT5677_M_ST_DAC1_R_SFT: u32 = 7;
pub const RT5677_M_DAC1_R_STO_R: u32 = (0x1 << 5);
pub const RT5677_M_DAC1_R_STO_R_SFT: u32 = 5;
pub const RT5677_DAC1_R_STO_R_VOL_MASK: u32 = (0x1 << 4);
pub const RT5677_DAC1_R_STO_R_VOL_SFT: u32 = 4;
pub const RT5677_M_DAC2_R_STO_R: u32 = (0x1 << 3);
pub const RT5677_M_DAC2_R_STO_R_SFT: u32 = 3;
pub const RT5677_DAC2_R_STO_R_VOL_MASK: u32 = (0x1 << 2);
pub const RT5677_DAC2_R_STO_R_VOL_SFT: u32 = 2;
pub const RT5677_M_DAC1_L_STO_R: u32 = (0x1 << 1);
pub const RT5677_M_DAC1_L_STO_R_SFT: u32 = 1;
pub const RT5677_DAC1_L_STO_R_VOL_MASK: u32 = (0x1 << 0);
pub const RT5677_DAC1_L_STO_R_VOL_SFT: u32 = 0;

/* Mono DAC Mixer L/R Control (0x2b) */
pub const RT5677_M_ST_DAC2_L: u32 = (0x1 << 15);
pub const RT5677_M_ST_DAC2_L_SFT: u32 = 15;
pub const RT5677_M_DAC2_L_MONO_L: u32 = (0x1 << 13);
pub const RT5677_M_DAC2_L_MONO_L_SFT: u32 = 13;
pub const RT5677_DAC2_L_MONO_L_VOL_MASK: u32 = (0x1 << 12);
pub const RT5677_DAC2_L_MONO_L_VOL_SFT: u32 = 12;
pub const RT5677_M_DAC2_R_MONO_L: u32 = (0x1 << 11);
pub const RT5677_M_DAC2_R_MONO_L_SFT: u32 = 11;
pub const RT5677_DAC2_R_MONO_L_VOL_MASK: u32 = (0x1 << 10);
pub const RT5677_DAC2_R_MONO_L_VOL_SFT: u32 = 10;
pub const RT5677_M_DAC1_L_MONO_L: u32 = (0x1 << 9);
pub const RT5677_M_DAC1_L_MONO_L_SFT: u32 = 9;
pub const RT5677_DAC1_L_MONO_L_VOL_MASK: u32 = (0x1 << 8);
pub const RT5677_DAC1_L_MONO_L_VOL_SFT: u32 = 8;
pub const RT5677_M_ST_DAC2_R: u32 = (0x1 << 7);
pub const RT5677_M_ST_DAC2_R_SFT: u32 = 7;
pub const RT5677_M_DAC2_R_MONO_R: u32 = (0x1 << 5);
pub const RT5677_M_DAC2_R_MONO_R_SFT: u32 = 5;
pub const RT5677_DAC2_R_MONO_R_VOL_MASK: u32 = (0x1 << 4);
pub const RT5677_DAC2_R_MONO_R_VOL_SFT: u32 = 4;
pub const RT5677_M_DAC1_R_MONO_R: u32 = (0x1 << 3);
pub const RT5677_M_DAC1_R_MONO_R_SFT: u32 = 3;
pub const RT5677_DAC1_R_MONO_R_VOL_MASK: u32 = (0x1 << 2);
pub const RT5677_DAC1_R_MONO_R_VOL_SFT: u32 = 2;
pub const RT5677_M_DAC2_L_MONO_R: u32 = (0x1 << 1);
pub const RT5677_M_DAC2_L_MONO_R_SFT: u32 = 1;
pub const RT5677_DAC2_L_MONO_R_VOL_MASK: u32 = (0x1 << 0);
pub const RT5677_DAC2_L_MONO_R_VOL_SFT: u32 = 0;

/* DD Mixer 1 Control (0x2c) */
pub const RT5677_M_STO_L_DD1_L: u32 = (0x1 << 15);
pub const RT5677_M_STO_L_DD1_L_SFT: u32 = 15;
pub const RT5677_STO_L_DD1_L_VOL_MASK: u32 = (0x1 << 14);
pub const RT5677_STO_L_DD1_L_VOL_SFT: u32 = 14;
pub const RT5677_M_MONO_L_DD1_L: u32 = (0x1 << 13);
pub const RT5677_M_MONO_L_DD1_L_SFT: u32 = 13;
pub const RT5677_MONO_L_DD1_L_VOL_MASK: u32 = (0x1 << 12);
pub const RT5677_MONO_L_DD1_L_VOL_SFT: u32 = 12;
pub const RT5677_M_DAC3_L_DD1_L: u32 = (0x1 << 11);
pub const RT5677_M_DAC3_L_DD1_L_SFT: u32 = 11;
pub const RT5677_DAC3_L_DD1_L_VOL_MASK: u32 = (0x1 << 10);
pub const RT5677_DAC3_L_DD1_L_VOL_SFT: u32 = 10;
pub const RT5677_M_DAC3_R_DD1_L: u32 = (0x1 << 9);
pub const RT5677_M_DAC3_R_DD1_L_SFT: u32 = 9;
pub const RT5677_DAC3_R_DD1_L_VOL_MASK: u32 = (0x1 << 8);
pub const RT5677_DAC3_R_DD1_L_VOL_SFT: u32 = 8;
pub const RT5677_M_STO_R_DD1_R: u32 = (0x1 << 7);
pub const RT5677_M_STO_R_DD1_R_SFT: u32 = 7;
pub const RT5677_STO_R_DD1_R_VOL_MASK: u32 = (0x1 << 6);
pub const RT5677_STO_R_DD1_R_VOL_SFT: u32 = 6;
pub const RT5677_M_MONO_R_DD1_R: u32 = (0x1 << 5);
pub const RT5677_M_MONO_R_DD1_R_SFT: u32 = 5;
pub const RT5677_MONO_R_DD1_R_VOL_MASK: u32 = (0x1 << 4);
pub const RT5677_MONO_R_DD1_R_VOL_SFT: u32 = 4;
pub const RT5677_M_DAC3_R_DD1_R: u32 = (0x1 << 3);
pub const RT5677_M_DAC3_R_DD1_R_SFT: u32 = 3;
pub const RT5677_DAC3_R_DD1_R_VOL_MASK: u32 = (0x1 << 2);
pub const RT5677_DAC3_R_DD1_R_VOL_SFT: u32 = 2;
pub const RT5677_M_DAC3_L_DD1_R: u32 = (0x1 << 1);
pub const RT5677_M_DAC3_L_DD1_R_SFT: u32 = 1;
pub const RT5677_DAC3_L_DD1_R_VOL_MASK: u32 = (0x1 << 0);
pub const RT5677_DAC3_L_DD1_R_VOL_SFT: u32 = 0;

/* DD Mixer 2 Control (0x2d) */
pub const RT5677_M_STO_L_DD2_L: u32 = (0x1 << 15);
pub const RT5677_M_STO_L_DD2_L_SFT: u32 = 15;
pub const RT5677_STO_L_DD2_L_VOL_MASK: u32 = (0x1 << 14);
pub const RT5677_STO_L_DD2_L_VOL_SFT: u32 = 14;
pub const RT5677_M_MONO_L_DD2_L: u32 = (0x1 << 13);
pub const RT5677_M_MONO_L_DD2_L_SFT: u32 = 13;
pub const RT5677_MONO_L_DD2_L_VOL_MASK: u32 = (0x1 << 12);
pub const RT5677_MONO_L_DD2_L_VOL_SFT: u32 = 12;
pub const RT5677_M_DAC4_L_DD2_L: u32 = (0x1 << 11);
pub const RT5677_M_DAC4_L_DD2_L_SFT: u32 = 11;
pub const RT5677_DAC4_L_DD2_L_VOL_MASK: u32 = (0x1 << 10);
pub const RT5677_DAC4_L_DD2_L_VOL_SFT: u32 = 10;
pub const RT5677_M_DAC4_R_DD2_L: u32 = (0x1 << 9);
pub const RT5677_M_DAC4_R_DD2_L_SFT: u32 = 9;
pub const RT5677_DAC4_R_DD2_L_VOL_MASK: u32 = (0x1 << 8);
pub const RT5677_DAC4_R_DD2_L_VOL_SFT: u32 = 8;
pub const RT5677_M_STO_R_DD2_R: u32 = (0x1 << 7);
pub const RT5677_M_STO_R_DD2_R_SFT: u32 = 7;
pub const RT5677_STO_R_DD2_R_VOL_MASK: u32 = (0x1 << 6);
pub const RT5677_STO_R_DD2_R_VOL_SFT: u32 = 6;
pub const RT5677_M_MONO_R_DD2_R: u32 = (0x1 << 5);
pub const RT5677_M_MONO_R_DD2_R_SFT: u32 = 5;
pub const RT5677_MONO_R_DD2_R_VOL_MASK: u32 = (0x1 << 4);
pub const RT5677_MONO_R_DD2_R_VOL_SFT: u32 = 4;
pub const RT5677_M_DAC4_R_DD2_R: u32 = (0x1 << 3);
pub const RT5677_M_DAC4_R_DD2_R_SFT: u32 = 3;
pub const RT5677_DAC4_R_DD2_R_VOL_MASK: u32 = (0x1 << 2);
pub const RT5677_DAC4_R_DD2_R_VOL_SFT: u32 = 2;
pub const RT5677_M_DAC4_L_DD2_R: u32 = (0x1 << 1);
pub const RT5677_M_DAC4_L_DD2_R_SFT: u32 = 1;
pub const RT5677_DAC4_L_DD2_R_VOL_MASK: u32 = (0x1 << 0);
pub const RT5677_DAC4_L_DD2_R_VOL_SFT: u32 = 0;

/* IF3 data control (0x2f) */
pub const RT5677_IF3_DAC_SEL_MASK: u32 = (0x3 << 6);
pub const RT5677_IF3_DAC_SEL_SFT: u32 = 6;
pub const RT5677_IF3_ADC_SEL_MASK: u32 = (0x3 << 4);
pub const RT5677_IF3_ADC_SEL_SFT: u32 = 4;
pub const RT5677_IF3_ADC_IN_MASK: u32 = (0xf << 0);
pub const RT5677_IF3_ADC_IN_SFT: u32 = 0;

/* IF4 data control (0x30) */
pub const RT5677_IF4_ADC_IN_MASK: u32 = (0xf << 4);
pub const RT5677_IF4_ADC_IN_SFT: u32 = 4;
pub const RT5677_IF4_DAC_SEL_MASK: u32 = (0x3 << 2);
pub const RT5677_IF4_DAC_SEL_SFT: u32 = 2;
pub const RT5677_IF4_ADC_SEL_MASK: u32 = (0x3 << 0);
pub const RT5677_IF4_ADC_SEL_SFT: u32 = 0;

/* PDM Output Control (0x31) */
pub const RT5677_M_PDM1_L: u32 = (0x1 << 15);
pub const RT5677_M_PDM1_L_SFT: u32 = 15;
pub const RT5677_SEL_PDM1_L_MASK: u32 = (0x3 << 12);
pub const RT5677_SEL_PDM1_L_SFT: u32 = 12;
pub const RT5677_M_PDM1_R: u32 = (0x1 << 11);
pub const RT5677_M_PDM1_R_SFT: u32 = 11;
pub const RT5677_SEL_PDM1_R_MASK: u32 = (0x3 << 8);
pub const RT5677_SEL_PDM1_R_SFT: u32 = 8;
pub const RT5677_M_PDM2_L: u32 = (0x1 << 7);
pub const RT5677_M_PDM2_L_SFT: u32 = 7;
pub const RT5677_SEL_PDM2_L_MASK: u32 = (0x3 << 4);
pub const RT5677_SEL_PDM2_L_SFT: u32 = 4;
pub const RT5677_M_PDM2_R: u32 = (0x1 << 3);
pub const RT5677_M_PDM2_R_SFT: u32 = 3;
pub const RT5677_SEL_PDM2_R_MASK: u32 = (0x3 << 0);
pub const RT5677_SEL_PDM2_R_SFT: u32 = 0;

/* PDM I2C / Data Control 1 (0x32) */
pub const RT5677_PDM2_PW_DOWN: u32 = (0x1 << 7);
pub const RT5677_PDM1_PW_DOWN: u32 = (0x1 << 6);
pub const RT5677_PDM2_BUSY: u32 = (0x1 << 5);
pub const RT5677_PDM1_BUSY: u32 = (0x1 << 4);
pub const RT5677_PDM_PATTERN: u32 = (0x1 << 3);
pub const RT5677_PDM_GAIN: u32 = (0x1 << 2);
pub const RT5677_PDM_DIV_MASK: u32 = (0x3 << 0);

/* PDM I2C / Data Control 2 (0x33) */
pub const RT5677_PDM1_I2C_ID: u32 = (0xf << 12);
pub const RT5677_PDM1_EXE: u32 = (0x1 << 11);
pub const RT5677_PDM1_I2C_CMD: u32 = (0x1 << 10);
pub const RT5677_PDM1_I2C_EXE: u32 = (0x1 << 9);
pub const RT5677_PDM1_I2C_BUSY: u32 = (0x1 << 8);
pub const RT5677_PDM2_I2C_ID: u32 = (0xf << 4);
pub const RT5677_PDM2_EXE: u32 = (0x1 << 3);
pub const RT5677_PDM2_I2C_CMD: u32 = (0x1 << 2);
pub const RT5677_PDM2_I2C_EXE: u32 = (0x1 << 1);
pub const RT5677_PDM2_I2C_BUSY: u32 = (0x1 << 0);

/* TDM1 control 1 (0x3b) */
pub const RT5677_IF1_ADC_MODE_MASK: u32 = (0x1 << 12);
pub const RT5677_IF1_ADC_MODE_SFT: u32 = 12;
pub const RT5677_IF1_ADC_MODE_I2S: u32 = (0x0 << 12);
pub const RT5677_IF1_ADC_MODE_TDM: u32 = (0x1 << 12);
pub const RT5677_IF1_ADC1_SWAP_MASK: u32 = (0x3 << 6);
pub const RT5677_IF1_ADC1_SWAP_SFT: u32 = 6;
pub const RT5677_IF1_ADC2_SWAP_MASK: u32 = (0x3 << 4);
pub const RT5677_IF1_ADC2_SWAP_SFT: u32 = 4;
pub const RT5677_IF1_ADC3_SWAP_MASK: u32 = (0x3 << 2);
pub const RT5677_IF1_ADC3_SWAP_SFT: u32 = 2;
pub const RT5677_IF1_ADC4_SWAP_MASK: u32 = (0x3 << 0);
pub const RT5677_IF1_ADC4_SWAP_SFT: u32 = 0;

/* TDM1 control 2 (0x3c) */
pub const RT5677_IF1_ADC4_MASK: u32 = (0x3 << 10);
pub const RT5677_IF1_ADC4_SFT: u32 = 10;
pub const RT5677_IF1_ADC3_MASK: u32 = (0x3 << 8);
pub const RT5677_IF1_ADC3_SFT: u32 = 8;
pub const RT5677_IF1_ADC2_MASK: u32 = (0x3 << 6);
pub const RT5677_IF1_ADC2_SFT: u32 = 6;
pub const RT5677_IF1_ADC1_MASK: u32 = (0x3 << 4);
pub const RT5677_IF1_ADC1_SFT: u32 = 4;
pub const RT5677_IF1_ADC_CTRL_MASK: u32 = (0x7 << 0);
pub const RT5677_IF1_ADC_CTRL_SFT: u32 = 0;

/* TDM1 control 4 (0x3e) */
pub const RT5677_IF1_DAC0_MASK: u32 = (0x7 << 12);
pub const RT5677_IF1_DAC0_SFT: u32 = 12;
pub const RT5677_IF1_DAC1_MASK: u32 = (0x7 << 8);
pub const RT5677_IF1_DAC1_SFT: u32 = 8;
pub const RT5677_IF1_DAC2_MASK: u32 = (0x7 << 4);
pub const RT5677_IF1_DAC2_SFT: u32 = 4;
pub const RT5677_IF1_DAC3_MASK: u32 = (0x7 << 0);
pub const RT5677_IF1_DAC3_SFT: u32 = 0;

/* TDM1 control 5 (0x3f) */
pub const RT5677_IF1_DAC4_MASK: u32 = (0x7 << 12);
pub const RT5677_IF1_DAC4_SFT: u32 = 12;
pub const RT5677_IF1_DAC5_MASK: u32 = (0x7 << 8);
pub const RT5677_IF1_DAC5_SFT: u32 = 8;
pub const RT5677_IF1_DAC6_MASK: u32 = (0x7 << 4);
pub const RT5677_IF1_DAC6_SFT: u32 = 4;
pub const RT5677_IF1_DAC7_MASK: u32 = (0x7 << 0);
pub const RT5677_IF1_DAC7_SFT: u32 = 0;

/* TDM2 control 1 (0x40) */
pub const RT5677_IF2_ADC_MODE_MASK: u32 = (0x1 << 12);
pub const RT5677_IF2_ADC_MODE_SFT: u32 = 12;
pub const RT5677_IF2_ADC_MODE_I2S: u32 = (0x0 << 12);
pub const RT5677_IF2_ADC_MODE_TDM: u32 = (0x1 << 12);
pub const RT5677_IF2_ADC1_SWAP_MASK: u32 = (0x3 << 6);
pub const RT5677_IF2_ADC1_SWAP_SFT: u32 = 6;
pub const RT5677_IF2_ADC2_SWAP_MASK: u32 = (0x3 << 4);
pub const RT5677_IF2_ADC2_SWAP_SFT: u32 = 4;
pub const RT5677_IF2_ADC3_SWAP_MASK: u32 = (0x3 << 2);
pub const RT5677_IF2_ADC3_SWAP_SFT: u32 = 2;
pub const RT5677_IF2_ADC4_SWAP_MASK: u32 = (0x3 << 0);
pub const RT5677_IF2_ADC4_SWAP_SFT: u32 = 0;

/* TDM2 control 2 (0x41) */
pub const RT5677_IF2_ADC4_MASK: u32 = (0x3 << 10);
pub const RT5677_IF2_ADC4_SFT: u32 = 10;
pub const RT5677_IF2_ADC3_MASK: u32 = (0x3 << 8);
pub const RT5677_IF2_ADC3_SFT: u32 = 8;
pub const RT5677_IF2_ADC2_MASK: u32 = (0x3 << 6);
pub const RT5677_IF2_ADC2_SFT: u32 = 6;
pub const RT5677_IF2_ADC1_MASK: u32 = (0x3 << 4);
pub const RT5677_IF2_ADC1_SFT: u32 = 4;
pub const RT5677_IF2_ADC_CTRL_MASK: u32 = (0x7 << 0);
pub const RT5677_IF2_ADC_CTRL_SFT: u32 = 0;

/* TDM2 control 4 (0x43) */
pub const RT5677_IF2_DAC0_MASK: u32 = (0x7 << 12);
pub const RT5677_IF2_DAC0_SFT: u32 = 12;
pub const RT5677_IF2_DAC1_MASK: u32 = (0x7 << 8);
pub const RT5677_IF2_DAC1_SFT: u32 = 8;
pub const RT5677_IF2_DAC2_MASK: u32 = (0x7 << 4);
pub const RT5677_IF2_DAC2_SFT: u32 = 4;
pub const RT5677_IF2_DAC3_MASK: u32 = (0x7 << 0);
pub const RT5677_IF2_DAC3_SFT: u32 = 0;

/* TDM2 control 5 (0x44) */
pub const RT5677_IF2_DAC4_MASK: u32 = (0x7 << 12);
pub const RT5677_IF2_DAC4_SFT: u32 = 12;
pub const RT5677_IF2_DAC5_MASK: u32 = (0x7 << 8);
pub const RT5677_IF2_DAC5_SFT: u32 = 8;
pub const RT5677_IF2_DAC6_MASK: u32 = (0x7 << 4);
pub const RT5677_IF2_DAC6_SFT: u32 = 4;
pub const RT5677_IF2_DAC7_MASK: u32 = (0x7 << 0);
pub const RT5677_IF2_DAC7_SFT: u32 = 0;

/* Digital Microphone Control 1 (0x50) */
pub const RT5677_DMIC_1_EN_MASK: u32 = (0x1 << 15);
pub const RT5677_DMIC_1_EN_SFT: u32 = 15;
pub const RT5677_DMIC_1_DIS: u32 = (0x0 << 15);
pub const RT5677_DMIC_1_EN: u32 = (0x1 << 15);
pub const RT5677_DMIC_2_EN_MASK: u32 = (0x1 << 14);
pub const RT5677_DMIC_2_EN_SFT: u32 = 14;
pub const RT5677_DMIC_2_DIS: u32 = (0x0 << 14);
pub const RT5677_DMIC_2_EN: u32 = (0x1 << 14);
pub const RT5677_DMIC_L_STO1_LH_MASK: u32 = (0x1 << 13);
pub const RT5677_DMIC_L_STO1_LH_SFT: u32 = 13;
pub const RT5677_DMIC_L_STO1_LH_FALLING: u32 = (0x0 << 13);
pub const RT5677_DMIC_L_STO1_LH_RISING: u32 = (0x1 << 13);
pub const RT5677_DMIC_R_STO1_LH_MASK: u32 = (0x1 << 12);
pub const RT5677_DMIC_R_STO1_LH_SFT: u32 = 12;
pub const RT5677_DMIC_R_STO1_LH_FALLING: u32 = (0x0 << 12);
pub const RT5677_DMIC_R_STO1_LH_RISING: u32 = (0x1 << 12);
pub const RT5677_DMIC_L_STO3_LH_MASK: u32 = (0x1 << 11);
pub const RT5677_DMIC_L_STO3_LH_SFT: u32 = 11;
pub const RT5677_DMIC_L_STO3_LH_FALLING: u32 = (0x0 << 11);
pub const RT5677_DMIC_L_STO3_LH_RISING: u32 = (0x1 << 11);
pub const RT5677_DMIC_R_STO3_LH_MASK: u32 = (0x1 << 10);
pub const RT5677_DMIC_R_STO3_LH_SFT: u32 = 10;
pub const RT5677_DMIC_R_STO3_LH_FALLING: u32 = (0x0 << 10);
pub const RT5677_DMIC_R_STO3_LH_RISING: u32 = (0x1 << 10);
pub const RT5677_DMIC_L_STO2_LH_MASK: u32 = (0x1 << 9);
pub const RT5677_DMIC_L_STO2_LH_SFT: u32 = 9;
pub const RT5677_DMIC_L_STO2_LH_FALLING: u32 = (0x0 << 9);
pub const RT5677_DMIC_L_STO2_LH_RISING: u32 = (0x1 << 9);
pub const RT5677_DMIC_R_STO2_LH_MASK: u32 = (0x1 << 8);
pub const RT5677_DMIC_R_STO2_LH_SFT: u32 = 8;
pub const RT5677_DMIC_R_STO2_LH_FALLING: u32 = (0x0 << 8);
pub const RT5677_DMIC_R_STO2_LH_RISING: u32 = (0x1 << 8);
pub const RT5677_DMIC_CLK_MASK: u32 = (0x7 << 5);
pub const RT5677_DMIC_CLK_SFT: u32 = 5;
pub const RT5677_DMIC_3_EN_MASK: u32 = (0x1 << 4);
pub const RT5677_DMIC_3_EN_SFT: u32 = 4;
pub const RT5677_DMIC_3_DIS: u32 = (0x0 << 4);
pub const RT5677_DMIC_3_EN: u32 = (0x1 << 4);
pub const RT5677_DMIC_R_MONO_LH_MASK: u32 = (0x1 << 2);
pub const RT5677_DMIC_R_MONO_LH_SFT: u32 = 2;
pub const RT5677_DMIC_R_MONO_LH_FALLING: u32 = (0x0 << 2);
pub const RT5677_DMIC_R_MONO_LH_RISING: u32 = (0x1 << 2);
pub const RT5677_DMIC_L_STO4_LH_MASK: u32 = (0x1 << 1);
pub const RT5677_DMIC_L_STO4_LH_SFT: u32 = 1;
pub const RT5677_DMIC_L_STO4_LH_FALLING: u32 = (0x0 << 1);
pub const RT5677_DMIC_L_STO4_LH_RISING: u32 = (0x1 << 1);
pub const RT5677_DMIC_R_STO4_LH_MASK: u32 = (0x1 << 0);
pub const RT5677_DMIC_R_STO4_LH_SFT: u32 = 0;
pub const RT5677_DMIC_R_STO4_LH_FALLING: u32 = (0x0 << 0);
pub const RT5677_DMIC_R_STO4_LH_RISING: u32 = (0x1 << 0);

/* Digital Microphone Control 2 (0x51) */
pub const RT5677_DMIC_4_EN_MASK: u32 = (0x1 << 15);
pub const RT5677_DMIC_4_EN_SFT: u32 = 15;
pub const RT5677_DMIC_4_DIS: u32 = (0x0 << 15);
pub const RT5677_DMIC_4_EN: u32 = (0x1 << 15);
pub const RT5677_DMIC_4L_LH_MASK: u32 = (0x1 << 7);
pub const RT5677_DMIC_4L_LH_SFT: u32 = 7;
pub const RT5677_DMIC_4L_LH_FALLING: u32 = (0x0 << 7);
pub const RT5677_DMIC_4L_LH_RISING: u32 = (0x1 << 7);
pub const RT5677_DMIC_4R_LH_MASK: u32 = (0x1 << 6);
pub const RT5677_DMIC_4R_LH_SFT: u32 = 6;
pub const RT5677_DMIC_4R_LH_FALLING: u32 = (0x0 << 6);
pub const RT5677_DMIC_4R_LH_RISING: u32 = (0x1 << 6);
pub const RT5677_DMIC_3L_LH_MASK: u32 = (0x1 << 5);
pub const RT5677_DMIC_3L_LH_SFT: u32 = 5;
pub const RT5677_DMIC_3L_LH_FALLING: u32 = (0x0 << 5);
pub const RT5677_DMIC_3L_LH_RISING: u32 = (0x1 << 5);
pub const RT5677_DMIC_3R_LH_MASK: u32 = (0x1 << 4);
pub const RT5677_DMIC_3R_LH_SFT: u32 = 4;
pub const RT5677_DMIC_3R_LH_FALLING: u32 = (0x0 << 4);
pub const RT5677_DMIC_3R_LH_RISING: u32 = (0x1 << 4);
pub const RT5677_DMIC_2L_LH_MASK: u32 = (0x1 << 3);
pub const RT5677_DMIC_2L_LH_SFT: u32 = 3;
pub const RT5677_DMIC_2L_LH_FALLING: u32 = (0x0 << 3);
pub const RT5677_DMIC_2L_LH_RISING: u32 = (0x1 << 3);
pub const RT5677_DMIC_2R_LH_MASK: u32 = (0x1 << 2);
pub const RT5677_DMIC_2R_LH_SFT: u32 = 2;
pub const RT5677_DMIC_2R_LH_FALLING: u32 = (0x0 << 2);
pub const RT5677_DMIC_2R_LH_RISING: u32 = (0x1 << 2);
pub const RT5677_DMIC_1L_LH_MASK: u32 = (0x1 << 1);
pub const RT5677_DMIC_1L_LH_SFT: u32 = 1;
pub const RT5677_DMIC_1L_LH_FALLING: u32 = (0x0 << 1);
pub const RT5677_DMIC_1L_LH_RISING: u32 = (0x1 << 1);
pub const RT5677_DMIC_1R_LH_MASK: u32 = (0x1 << 0);
pub const RT5677_DMIC_1R_LH_SFT: u32 = 0;
pub const RT5677_DMIC_1R_LH_FALLING: u32 = (0x0 << 0);
pub const RT5677_DMIC_1R_LH_RISING: u32 = (0x1 << 0);

/* Power Management for Digital 1 (0x61) */
pub const RT5677_PWR_I2S1: u32 = (0x1 << 15);
pub const RT5677_PWR_I2S1_BIT: u32 = 15;
pub const RT5677_PWR_I2S2: u32 = (0x1 << 14);
pub const RT5677_PWR_I2S2_BIT: u32 = 14;
pub const RT5677_PWR_I2S3: u32 = (0x1 << 13);
pub const RT5677_PWR_I2S3_BIT: u32 = 13;
pub const RT5677_PWR_DAC1: u32 = (0x1 << 12);
pub const RT5677_PWR_DAC1_BIT: u32 = 12;
pub const RT5677_PWR_DAC2: u32 = (0x1 << 11);
pub const RT5677_PWR_DAC2_BIT: u32 = 11;
pub const RT5677_PWR_I2S4: u32 = (0x1 << 10);
pub const RT5677_PWR_I2S4_BIT: u32 = 10;
pub const RT5677_PWR_SLB: u32 = (0x1 << 9);
pub const RT5677_PWR_SLB_BIT: u32 = 9;
pub const RT5677_PWR_DAC3: u32 = (0x1 << 7);
pub const RT5677_PWR_DAC3_BIT: u32 = 7;
pub const RT5677_PWR_ADCFED2: u32 = (0x1 << 4);
pub const RT5677_PWR_ADCFED2_BIT: u32 = 4;
pub const RT5677_PWR_ADCFED1: u32 = (0x1 << 3);
pub const RT5677_PWR_ADCFED1_BIT: u32 = 3;
pub const RT5677_PWR_ADC_L: u32 = (0x1 << 2);
pub const RT5677_PWR_ADC_L_BIT: u32 = 2;
pub const RT5677_PWR_ADC_R: u32 = (0x1 << 1);
pub const RT5677_PWR_ADC_R_BIT: u32 = 1;
pub const RT5677_PWR_I2C_MASTER: u32 = (0x1 << 0);
pub const RT5677_PWR_I2C_MASTER_BIT: u32 = 0;

/* Power Management for Digital 2 (0x62) */
pub const RT5677_PWR_ADC_S1F: u32 = (0x1 << 15);
pub const RT5677_PWR_ADC_S1F_BIT: u32 = 15;
pub const RT5677_PWR_ADC_MF_L: u32 = (0x1 << 14);
pub const RT5677_PWR_ADC_MF_L_BIT: u32 = 14;
pub const RT5677_PWR_ADC_MF_R: u32 = (0x1 << 13);
pub const RT5677_PWR_ADC_MF_R_BIT: u32 = 13;
pub const RT5677_PWR_DAC_S1F: u32 = (0x1 << 12);
pub const RT5677_PWR_DAC_S1F_BIT: u32 = 12;
pub const RT5677_PWR_DAC_M2F_L: u32 = (0x1 << 11);
pub const RT5677_PWR_DAC_M2F_L_BIT: u32 = 11;
pub const RT5677_PWR_DAC_M2F_R: u32 = (0x1 << 10);
pub const RT5677_PWR_DAC_M2F_R_BIT: u32 = 10;
pub const RT5677_PWR_DAC_M3F_L: u32 = (0x1 << 9);
pub const RT5677_PWR_DAC_M3F_L_BIT: u32 = 9;
pub const RT5677_PWR_DAC_M3F_R: u32 = (0x1 << 8);
pub const RT5677_PWR_DAC_M3F_R_BIT: u32 = 8;
pub const RT5677_PWR_DAC_M4F_L: u32 = (0x1 << 7);
pub const RT5677_PWR_DAC_M4F_L_BIT: u32 = 7;
pub const RT5677_PWR_DAC_M4F_R: u32 = (0x1 << 6);
pub const RT5677_PWR_DAC_M4F_R_BIT: u32 = 6;
pub const RT5677_PWR_ADC_S2F: u32 = (0x1 << 5);
pub const RT5677_PWR_ADC_S2F_BIT: u32 = 5;
pub const RT5677_PWR_ADC_S3F: u32 = (0x1 << 4);
pub const RT5677_PWR_ADC_S3F_BIT: u32 = 4;
pub const RT5677_PWR_ADC_S4F: u32 = (0x1 << 3);
pub const RT5677_PWR_ADC_S4F_BIT: u32 = 3;
pub const RT5677_PWR_PDM1: u32 = (0x1 << 2);
pub const RT5677_PWR_PDM1_BIT: u32 = 2;
pub const RT5677_PWR_PDM2: u32 = (0x1 << 1);
pub const RT5677_PWR_PDM2_BIT: u32 = 1;

/* Power Management for Analog 1 (0x63) */
pub const RT5677_PWR_VREF1: u32 = (0x1 << 15);
pub const RT5677_PWR_VREF1_BIT: u32 = 15;
pub const RT5677_PWR_FV1: u32 = (0x1 << 14);
pub const RT5677_PWR_FV1_BIT: u32 = 14;
pub const RT5677_PWR_MB: u32 = (0x1 << 13);
pub const RT5677_PWR_MB_BIT: u32 = 13;
pub const RT5677_PWR_LO1: u32 = (0x1 << 12);
pub const RT5677_PWR_LO1_BIT: u32 = 12;
pub const RT5677_PWR_BG: u32 = (0x1 << 11);
pub const RT5677_PWR_BG_BIT: u32 = 11;
pub const RT5677_PWR_LO2: u32 = (0x1 << 10);
pub const RT5677_PWR_LO2_BIT: u32 = 10;
pub const RT5677_PWR_LO3: u32 = (0x1 << 9);
pub const RT5677_PWR_LO3_BIT: u32 = 9;
pub const RT5677_PWR_VREF2: u32 = (0x1 << 8);
pub const RT5677_PWR_VREF2_BIT: u32 = 8;
pub const RT5677_PWR_FV2: u32 = (0x1 << 7);
pub const RT5677_PWR_FV2_BIT: u32 = 7;
pub const RT5677_LDO2_SEL_MASK: u32 = (0x7 << 4);
pub const RT5677_LDO2_SEL_SFT: u32 = 4;
pub const RT5677_LDO1_SEL_MASK: u32 = (0x7 << 0);
pub const RT5677_LDO1_SEL_SFT: u32 = 0;

/* Power Management for Analog 2 (0x64) */
pub const RT5677_PWR_BST1: u32 = (0x1 << 15);
pub const RT5677_PWR_BST1_BIT: u32 = 15;
pub const RT5677_PWR_BST2: u32 = (0x1 << 14);
pub const RT5677_PWR_BST2_BIT: u32 = 14;
pub const RT5677_PWR_CLK_MB1: u32 = (0x1 << 13);
pub const RT5677_PWR_CLK_MB1_BIT: u32 = 13;
pub const RT5677_PWR_SLIM: u32 = (0x1 << 12);
pub const RT5677_PWR_SLIM_BIT: u32 = 12;
pub const RT5677_PWR_MB1: u32 = (0x1 << 11);
pub const RT5677_PWR_MB1_BIT: u32 = 11;
pub const RT5677_PWR_PP_MB1: u32 = (0x1 << 10);
pub const RT5677_PWR_PP_MB1_BIT: u32 = 10;
pub const RT5677_PWR_PLL1: u32 = (0x1 << 9);
pub const RT5677_PWR_PLL1_BIT: u32 = 9;
pub const RT5677_PWR_PLL2: u32 = (0x1 << 8);
pub const RT5677_PWR_PLL2_BIT: u32 = 8;
pub const RT5677_PWR_CORE: u32 = (0x1 << 7);
pub const RT5677_PWR_CORE_BIT: u32 = 7;
pub const RT5677_PWR_CLK_MB: u32 = (0x1 << 6);
pub const RT5677_PWR_CLK_MB_BIT: u32 = 6;
pub const RT5677_PWR_BST1_P: u32 = (0x1 << 5);
pub const RT5677_PWR_BST1_P_BIT: u32 = 5;
pub const RT5677_PWR_BST2_P: u32 = (0x1 << 4);
pub const RT5677_PWR_BST2_P_BIT: u32 = 4;
pub const RT5677_PWR_IPTV: u32 = (0x1 << 3);
pub const RT5677_PWR_IPTV_BIT: u32 = 3;
pub const RT5677_PWR_25M_CLK: u32 = (0x1 << 1);
pub const RT5677_PWR_25M_CLK_BIT: u32 = 1;
pub const RT5677_PWR_LDO1: u32 = (0x1 << 0);
pub const RT5677_PWR_LDO1_BIT: u32 = 0;

/* Power Management for DSP (0x65) */
pub const RT5677_PWR_SR7: u32 = (0x1 << 10);
pub const RT5677_PWR_SR7_BIT: u32 = 10;
pub const RT5677_PWR_SR6: u32 = (0x1 << 9);
pub const RT5677_PWR_SR6_BIT: u32 = 9;
pub const RT5677_PWR_SR5: u32 = (0x1 << 8);
pub const RT5677_PWR_SR5_BIT: u32 = 8;
pub const RT5677_PWR_SR4: u32 = (0x1 << 7);
pub const RT5677_PWR_SR4_BIT: u32 = 7;
pub const RT5677_PWR_SR3: u32 = (0x1 << 6);
pub const RT5677_PWR_SR3_BIT: u32 = 6;
pub const RT5677_PWR_SR2: u32 = (0x1 << 5);
pub const RT5677_PWR_SR2_BIT: u32 = 5;
pub const RT5677_PWR_SR1: u32 = (0x1 << 4);
pub const RT5677_PWR_SR1_BIT: u32 = 4;
pub const RT5677_PWR_SR0: u32 = (0x1 << 3);
pub const RT5677_PWR_SR0_BIT: u32 = 3;
pub const RT5677_PWR_MLT: u32 = (0x1 << 2);
pub const RT5677_PWR_MLT_BIT: u32 = 2;
pub const RT5677_PWR_DSP: u32 = (0x1 << 1);
pub const RT5677_PWR_DSP_BIT: u32 = 1;
pub const RT5677_PWR_DSP_CPU: u32 = (0x1 << 0);
pub const RT5677_PWR_DSP_CPU_BIT: u32 = 0;

/* Power Status for DSP (0x66) */
pub const RT5677_PWR_SR7_RDY: u32 = (0x1 << 9);
pub const RT5677_PWR_SR7_RDY_BIT: u32 = 9;
pub const RT5677_PWR_SR6_RDY: u32 = (0x1 << 8);
pub const RT5677_PWR_SR6_RDY_BIT: u32 = 8;
pub const RT5677_PWR_SR5_RDY: u32 = (0x1 << 7);
pub const RT5677_PWR_SR5_RDY_BIT: u32 = 7;
pub const RT5677_PWR_SR4_RDY: u32 = (0x1 << 6);
pub const RT5677_PWR_SR4_RDY_BIT: u32 = 6;
pub const RT5677_PWR_SR3_RDY: u32 = (0x1 << 5);
pub const RT5677_PWR_SR3_RDY_BIT: u32 = 5;
pub const RT5677_PWR_SR2_RDY: u32 = (0x1 << 4);
pub const RT5677_PWR_SR2_RDY_BIT: u32 = 4;
pub const RT5677_PWR_SR1_RDY: u32 = (0x1 << 3);
pub const RT5677_PWR_SR1_RDY_BIT: u32 = 3;
pub const RT5677_PWR_SR0_RDY: u32 = (0x1 << 2);
pub const RT5677_PWR_SR0_RDY_BIT: u32 = 2;
pub const RT5677_PWR_MLT_RDY: u32 = (0x1 << 1);
pub const RT5677_PWR_MLT_RDY_BIT: u32 = 1;
pub const RT5677_PWR_DSP_RDY: u32 = (0x1 << 0);
pub const RT5677_PWR_DSP_RDY_BIT: u32 = 0;

/* Power Management for DSP (0x67) */
pub const RT5677_PWR_SLIM_ISO: u32 = (0x1 << 11);
pub const RT5677_PWR_SLIM_ISO_BIT: u32 = 11;
pub const RT5677_PWR_CORE_ISO: u32 = (0x1 << 10);
pub const RT5677_PWR_CORE_ISO_BIT: u32 = 10;
pub const RT5677_PWR_DSP_ISO: u32 = (0x1 << 9);
pub const RT5677_PWR_DSP_ISO_BIT: u32 = 9;
pub const RT5677_PWR_SR7_ISO: u32 = (0x1 << 8);
pub const RT5677_PWR_SR7_ISO_BIT: u32 = 8;
pub const RT5677_PWR_SR6_ISO: u32 = (0x1 << 7);
pub const RT5677_PWR_SR6_ISO_BIT: u32 = 7;
pub const RT5677_PWR_SR5_ISO: u32 = (0x1 << 6);
pub const RT5677_PWR_SR5_ISO_BIT: u32 = 6;
pub const RT5677_PWR_SR4_ISO: u32 = (0x1 << 5);
pub const RT5677_PWR_SR4_ISO_BIT: u32 = 5;
pub const RT5677_PWR_SR3_ISO: u32 = (0x1 << 4);
pub const RT5677_PWR_SR3_ISO_BIT: u32 = 4;
pub const RT5677_PWR_SR2_ISO: u32 = (0x1 << 3);
pub const RT5677_PWR_SR2_ISO_BIT: u32 = 3;
pub const RT5677_PWR_SR1_ISO: u32 = (0x1 << 2);
pub const RT5677_PWR_SR1_ISO_BIT: u32 = 2;
pub const RT5677_PWR_SR0_ISO: u32 = (0x1 << 1);
pub const RT5677_PWR_SR0_ISO_BIT: u32 = 1;
pub const RT5677_PWR_MLT_ISO: u32 = (0x1 << 0);
pub const RT5677_PWR_MLT_ISO_BIT: u32 = 0;

/* I2S1/2/3/4 Audio Serial Data Port Control (0x6f 0x70 0x71 0x72) */
pub const RT5677_I2S_MS_MASK: u32 = (0x1 << 15);
pub const RT5677_I2S_MS_SFT: u32 = 15;
pub const RT5677_I2S_MS_M: u32 = (0x0 << 15);
pub const RT5677_I2S_MS_S: u32 = (0x1 << 15);
pub const RT5677_I2S_O_CP_MASK: u32 = (0x3 << 10);
pub const RT5677_I2S_O_CP_SFT: u32 = 10;
pub const RT5677_I2S_O_CP_OFF: u32 = (0x0 << 10);
pub const RT5677_I2S_O_CP_U_LAW: u32 = (0x1 << 10);
pub const RT5677_I2S_O_CP_A_LAW: u32 = (0x2 << 10);
pub const RT5677_I2S_I_CP_MASK: u32 = (0x3 << 8);
pub const RT5677_I2S_I_CP_SFT: u32 = 8;
pub const RT5677_I2S_I_CP_OFF: u32 = (0x0 << 8);
pub const RT5677_I2S_I_CP_U_LAW: u32 = (0x1 << 8);
pub const RT5677_I2S_I_CP_A_LAW: u32 = (0x2 << 8);
pub const RT5677_I2S_BP_MASK: u32 = (0x1 << 7);
pub const RT5677_I2S_BP_SFT: u32 = 7;
pub const RT5677_I2S_BP_NOR: u32 = (0x0 << 7);
pub const RT5677_I2S_BP_INV: u32 = (0x1 << 7);
pub const RT5677_I2S_DL_MASK: u32 = (0x3 << 2);
pub const RT5677_I2S_DL_SFT: u32 = 2;
pub const RT5677_I2S_DL_16: u32 = (0x0 << 2);
pub const RT5677_I2S_DL_20: u32 = (0x1 << 2);
pub const RT5677_I2S_DL_24: u32 = (0x2 << 2);
pub const RT5677_I2S_DL_8: u32 = (0x3 << 2);
pub const RT5677_I2S_DF_MASK: u32 = (0x3 << 0);
pub const RT5677_I2S_DF_SFT: u32 = 0;
pub const RT5677_I2S_DF_I2S: u32 = (0x0 << 0);
pub const RT5677_I2S_DF_LEFT: u32 = (0x1 << 0);
pub const RT5677_I2S_DF_PCM_A: u32 = (0x2 << 0);
pub const RT5677_I2S_DF_PCM_B: u32 = (0x3 << 0);

/* Clock Tree Control 1 (0x73) */
pub const RT5677_I2S_PD1_MASK: u32 = (0x7 << 12);
pub const RT5677_I2S_PD1_SFT: u32 = 12;
pub const RT5677_I2S_PD1_1: u32 = (0x0 << 12);
pub const RT5677_I2S_PD1_2: u32 = (0x1 << 12);
pub const RT5677_I2S_PD1_3: u32 = (0x2 << 12);
pub const RT5677_I2S_PD1_4: u32 = (0x3 << 12);
pub const RT5677_I2S_PD1_6: u32 = (0x4 << 12);
pub const RT5677_I2S_PD1_8: u32 = (0x5 << 12);
pub const RT5677_I2S_PD1_12: u32 = (0x6 << 12);
pub const RT5677_I2S_PD1_16: u32 = (0x7 << 12);
pub const RT5677_I2S_BCLK_MS2_MASK: u32 = (0x1 << 11);
pub const RT5677_I2S_BCLK_MS2_SFT: u32 = 11;
pub const RT5677_I2S_BCLK_MS2_32: u32 = (0x0 << 11);
pub const RT5677_I2S_BCLK_MS2_64: u32 = (0x1 << 11);
pub const RT5677_I2S_PD2_MASK: u32 = (0x7 << 8);
pub const RT5677_I2S_PD2_SFT: u32 = 8;
pub const RT5677_I2S_PD2_1: u32 = (0x0 << 8);
pub const RT5677_I2S_PD2_2: u32 = (0x1 << 8);
pub const RT5677_I2S_PD2_3: u32 = (0x2 << 8);
pub const RT5677_I2S_PD2_4: u32 = (0x3 << 8);
pub const RT5677_I2S_PD2_6: u32 = (0x4 << 8);
pub const RT5677_I2S_PD2_8: u32 = (0x5 << 8);
pub const RT5677_I2S_PD2_12: u32 = (0x6 << 8);
pub const RT5677_I2S_PD2_16: u32 = (0x7 << 8);
pub const RT5677_I2S_BCLK_MS3_MASK: u32 = (0x1 << 7);
pub const RT5677_I2S_BCLK_MS3_SFT: u32 = 7;
pub const RT5677_I2S_BCLK_MS3_32: u32 = (0x0 << 7);
pub const RT5677_I2S_BCLK_MS3_64: u32 = (0x1 << 7);
pub const RT5677_I2S_PD3_MASK: u32 = (0x7 << 4);
pub const RT5677_I2S_PD3_SFT: u32 = 4;
pub const RT5677_I2S_PD3_1: u32 = (0x0 << 4);
pub const RT5677_I2S_PD3_2: u32 = (0x1 << 4);
pub const RT5677_I2S_PD3_3: u32 = (0x2 << 4);
pub const RT5677_I2S_PD3_4: u32 = (0x3 << 4);
pub const RT5677_I2S_PD3_6: u32 = (0x4 << 4);
pub const RT5677_I2S_PD3_8: u32 = (0x5 << 4);
pub const RT5677_I2S_PD3_12: u32 = (0x6 << 4);
pub const RT5677_I2S_PD3_16: u32 = (0x7 << 4);
pub const RT5677_I2S_BCLK_MS4_MASK: u32 = (0x1 << 3);
pub const RT5677_I2S_BCLK_MS4_SFT: u32 = 3;
pub const RT5677_I2S_BCLK_MS4_32: u32 = (0x0 << 3);
pub const RT5677_I2S_BCLK_MS4_64: u32 = (0x1 << 3);
pub const RT5677_I2S_PD4_MASK: u32 = (0x7 << 0);
pub const RT5677_I2S_PD4_SFT: u32 = 0;
pub const RT5677_I2S_PD4_1: u32 = (0x0 << 0);
pub const RT5677_I2S_PD4_2: u32 = (0x1 << 0);
pub const RT5677_I2S_PD4_3: u32 = (0x2 << 0);
pub const RT5677_I2S_PD4_4: u32 = (0x3 << 0);
pub const RT5677_I2S_PD4_6: u32 = (0x4 << 0);
pub const RT5677_I2S_PD4_8: u32 = (0x5 << 0);
pub const RT5677_I2S_PD4_12: u32 = (0x6 << 0);
pub const RT5677_I2S_PD4_16: u32 = (0x7 << 0);

/* Clock Tree Control 2 (0x74) */
pub const RT5677_I2S_PD5_MASK: u32 = (0x7 << 12);
pub const RT5677_I2S_PD5_SFT: u32 = 12;
pub const RT5677_I2S_PD5_1: u32 = (0x0 << 12);
pub const RT5677_I2S_PD5_2: u32 = (0x1 << 12);
pub const RT5677_I2S_PD5_3: u32 = (0x2 << 12);
pub const RT5677_I2S_PD5_4: u32 = (0x3 << 12);
pub const RT5677_I2S_PD5_6: u32 = (0x4 << 12);
pub const RT5677_I2S_PD5_8: u32 = (0x5 << 12);
pub const RT5677_I2S_PD5_12: u32 = (0x6 << 12);
pub const RT5677_I2S_PD5_16: u32 = (0x7 << 12);
pub const RT5677_I2S_PD6_MASK: u32 = (0x7 << 8);
pub const RT5677_I2S_PD6_SFT: u32 = 8;
pub const RT5677_I2S_PD6_1: u32 = (0x0 << 8);
pub const RT5677_I2S_PD6_2: u32 = (0x1 << 8);
pub const RT5677_I2S_PD6_3: u32 = (0x2 << 8);
pub const RT5677_I2S_PD6_4: u32 = (0x3 << 8);
pub const RT5677_I2S_PD6_6: u32 = (0x4 << 8);
pub const RT5677_I2S_PD6_8: u32 = (0x5 << 8);
pub const RT5677_I2S_PD6_12: u32 = (0x6 << 8);
pub const RT5677_I2S_PD6_16: u32 = (0x7 << 8);
pub const RT5677_I2S_PD7_MASK: u32 = (0x7 << 4);
pub const RT5677_I2S_PD7_SFT: u32 = 4;
pub const RT5677_I2S_PD7_1: u32 = (0x0 << 4);
pub const RT5677_I2S_PD7_2: u32 = (0x1 << 4);
pub const RT5677_I2S_PD7_3: u32 = (0x2 << 4);
pub const RT5677_I2S_PD7_4: u32 = (0x3 << 4);
pub const RT5677_I2S_PD7_6: u32 = (0x4 << 4);
pub const RT5677_I2S_PD7_8: u32 = (0x5 << 4);
pub const RT5677_I2S_PD7_12: u32 = (0x6 << 4);
pub const RT5677_I2S_PD7_16: u32 = (0x7 << 4);
pub const RT5677_I2S_PD8_MASK: u32 = (0x7 << 0);
pub const RT5677_I2S_PD8_SFT: u32 = 0;
pub const RT5677_I2S_PD8_1: u32 = (0x0 << 0);
pub const RT5677_I2S_PD8_2: u32 = (0x1 << 0);
pub const RT5677_I2S_PD8_3: u32 = (0x2 << 0);
pub const RT5677_I2S_PD8_4: u32 = (0x3 << 0);
pub const RT5677_I2S_PD8_6: u32 = (0x4 << 0);
pub const RT5677_I2S_PD8_8: u32 = (0x5 << 0);
pub const RT5677_I2S_PD8_12: u32 = (0x6 << 0);
pub const RT5677_I2S_PD8_16: u32 = (0x7 << 0);

/* Clock Tree Control 3 (0x75) */
pub const RT5677_DSP_ASRC_O_MASK: u32 = (0x3 << 6);
pub const RT5677_DSP_ASRC_O_SFT: u32 = 6;
pub const RT5677_DSP_ASRC_O_1_0: u32 = (0x0 << 6);
pub const RT5677_DSP_ASRC_O_1_5: u32 = (0x1 << 6);
pub const RT5677_DSP_ASRC_O_2_0: u32 = (0x2 << 6);
pub const RT5677_DSP_ASRC_O_3_0: u32 = (0x3 << 6);
pub const RT5677_DSP_ASRC_I_MASK: u32 = (0x3 << 4);
pub const RT5677_DSP_ASRC_I_SFT: u32 = 4;
pub const RT5677_DSP_ASRC_I_1_0: u32 = (0x0 << 4);
pub const RT5677_DSP_ASRC_I_1_5: u32 = (0x1 << 4);
pub const RT5677_DSP_ASRC_I_2_0: u32 = (0x2 << 4);
pub const RT5677_DSP_ASRC_I_3_0: u32 = (0x3 << 4);
pub const RT5677_DSP_BUS_PD_MASK: u32 = (0x7 << 0);
pub const RT5677_DSP_BUS_PD_SFT: u32 = 0;
pub const RT5677_DSP_BUS_PD_1: u32 = (0x0 << 0);
pub const RT5677_DSP_BUS_PD_2: u32 = (0x1 << 0);
pub const RT5677_DSP_BUS_PD_3: u32 = (0x2 << 0);
pub const RT5677_DSP_BUS_PD_4: u32 = (0x3 << 0);
pub const RT5677_DSP_BUS_PD_6: u32 = (0x4 << 0);
pub const RT5677_DSP_BUS_PD_8: u32 = (0x5 << 0);
pub const RT5677_DSP_BUS_PD_12: u32 = (0x6 << 0);
pub const RT5677_DSP_BUS_PD_16: u32 = (0x7 << 0);

pub const RT5677_PLL_INP_MAX: u32 = 40000000;
pub const RT5677_PLL_INP_MIN: u32 = 2048000;
/* PLL M/N/K Code Control 1 (0x7a 0x7c) */
pub const RT5677_PLL_N_MAX: u32 = 0x1ff;
pub const RT5677_PLL_N_MASK: u32 = (RT5677_PLL_N_MAX << 7);
pub const RT5677_PLL_N_SFT: u32 = 7;
pub const RT5677_PLL_K_BP: u32 = (0x1 << 5);
pub const RT5677_PLL_K_BP_SFT: u32 = 5;
pub const RT5677_PLL_K_MAX: u32 = 0x1f;
pub const RT5677_PLL_K_MASK: u32 = (RT5677_PLL_K_MAX);
pub const RT5677_PLL_K_SFT: u32 = 0;

/* PLL M/N/K Code Control 2 (0x7b 0x7d) */
pub const RT5677_PLL_M_MAX: u32 = 0xf;
pub const RT5677_PLL_M_MASK: u32 = (RT5677_PLL_M_MAX << 12);
pub const RT5677_PLL_M_SFT: u32 = 12;
pub const RT5677_PLL_M_BP: u32 = (0x1 << 11);
pub const RT5677_PLL_M_BP_SFT: u32 = 11;
pub const RT5677_PLL_UPDATE_PLL1: u32 = (0x1 << 1);
pub const RT5677_PLL_UPDATE_PLL1_SFT: u32 = 1;

/* Global Clock Control 1 (0x80) */
pub const RT5677_SCLK_SRC_MASK: u32 = (0x3 << 14);
pub const RT5677_SCLK_SRC_SFT: u32 = 14;
pub const RT5677_SCLK_SRC_MCLK: u32 = (0x0 << 14);
pub const RT5677_SCLK_SRC_PLL1: u32 = (0x1 << 14);
pub const RT5677_SCLK_SRC_RCCLK: u32 = (0x2 << 14); /* 25MHz */
pub const RT5677_SCLK_SRC_SLIM: u32 = (0x3 << 14);
pub const RT5677_PLL1_SRC_MASK: u32 = (0x7 << 11);
pub const RT5677_PLL1_SRC_SFT: u32 = 11;
pub const RT5677_PLL1_SRC_MCLK: u32 = (0x0 << 11);
pub const RT5677_PLL1_SRC_BCLK1: u32 = (0x1 << 11);
pub const RT5677_PLL1_SRC_BCLK2: u32 = (0x2 << 11);
pub const RT5677_PLL1_SRC_BCLK3: u32 = (0x3 << 11);
pub const RT5677_PLL1_SRC_BCLK4: u32 = (0x4 << 11);
pub const RT5677_PLL1_SRC_RCCLK: u32 = (0x5 << 11);
pub const RT5677_PLL1_SRC_SLIM: u32 = (0x6 << 11);
pub const RT5677_MCLK_SRC_MASK: u32 = (0x1 << 10);
pub const RT5677_MCLK_SRC_SFT: u32 = 10;
pub const RT5677_MCLK1_SRC: u32 = (0x0 << 10);
pub const RT5677_MCLK2_SRC: u32 = (0x1 << 10);
pub const RT5677_PLL1_PD_MASK: u32 = (0x1 << 8);
pub const RT5677_PLL1_PD_SFT: u32 = 8;
pub const RT5677_PLL1_PD_1: u32 = (0x0 << 8);
pub const RT5677_PLL1_PD_2: u32 = (0x1 << 8);
pub const RT5677_DAC_OSR_MASK: u32 = (0x3 << 6);
pub const RT5677_DAC_OSR_SFT: u32 = 6;
pub const RT5677_DAC_OSR_128: u32 = (0x0 << 6);
pub const RT5677_DAC_OSR_64: u32 = (0x1 << 6);
pub const RT5677_DAC_OSR_32: u32 = (0x2 << 6);
pub const RT5677_ADC_OSR_MASK: u32 = (0x3 << 4);
pub const RT5677_ADC_OSR_SFT: u32 = 4;
pub const RT5677_ADC_OSR_128: u32 = (0x0 << 4);
pub const RT5677_ADC_OSR_64: u32 = (0x1 << 4);
pub const RT5677_ADC_OSR_32: u32 = (0x2 << 4);

/* Global Clock Control 2 (0x81) */
pub const RT5677_PLL2_PR_SRC_MASK: u32 = (0x1 << 15);
pub const RT5677_PLL2_PR_SRC_SFT: u32 = 15;
pub const RT5677_PLL2_PR_SRC_MCLK1: u32 = (0x0 << 15);
pub const RT5677_PLL2_PR_SRC_MCLK2: u32 = (0x1 << 15);
pub const RT5677_PLL2_SRC_MASK: u32 = (0x7 << 12);
pub const RT5677_PLL2_SRC_SFT: u32 = 12;
pub const RT5677_PLL2_SRC_MCLK: u32 = (0x0 << 12);
pub const RT5677_PLL2_SRC_BCLK1: u32 = (0x1 << 12);
pub const RT5677_PLL2_SRC_BCLK2: u32 = (0x2 << 12);
pub const RT5677_PLL2_SRC_BCLK3: u32 = (0x3 << 12);
pub const RT5677_PLL2_SRC_BCLK4: u32 = (0x4 << 12);
pub const RT5677_PLL2_SRC_RCCLK: u32 = (0x5 << 12);
pub const RT5677_PLL2_SRC_SLIM: u32 = (0x6 << 12);
pub const RT5677_DSP_ASRC_O_SRC: u32 = (0x3 << 10);
pub const RT5677_DSP_ASRC_O_SRC_SFT: u32 = 10;
pub const RT5677_DSP_ASRC_O_MCLK: u32 = (0x0 << 10);
pub const RT5677_DSP_ASRC_O_PLL1: u32 = (0x1 << 10);
pub const RT5677_DSP_ASRC_O_SLIM: u32 = (0x2 << 10);
pub const RT5677_DSP_ASRC_O_RCCLK: u32 = (0x3 << 10);
pub const RT5677_DSP_ASRC_I_SRC: u32 = (0x3 << 8);
pub const RT5677_DSP_ASRC_I_SRC_SFT: u32 = 8;
pub const RT5677_DSP_ASRC_I_MCLK: u32 = (0x0 << 8);
pub const RT5677_DSP_ASRC_I_PLL1: u32 = (0x1 << 8);
pub const RT5677_DSP_ASRC_I_SLIM: u32 = (0x2 << 8);
pub const RT5677_DSP_ASRC_I_RCCLK: u32 = (0x3 << 8);
pub const RT5677_DSP_CLK_SRC_MASK: u32 = (0x1 << 7);
pub const RT5677_DSP_CLK_SRC_SFT: u32 = 7;
pub const RT5677_DSP_CLK_SRC_PLL2: u32 = (0x0 << 7);
pub const RT5677_DSP_CLK_SRC_BYPASS: u32 = (0x1 << 7);

/* ASRC Control 3 (0x85) */
pub const RT5677_DA_STO_CLK_SEL_MASK: u32 = (0xf << 12);
pub const RT5677_DA_STO_CLK_SEL_SFT: u32 = 12;
pub const RT5677_DA_MONO2L_CLK_SEL_MASK: u32 = (0xf << 4);
pub const RT5677_DA_MONO2L_CLK_SEL_SFT: u32 = 4;
pub const RT5677_DA_MONO2R_CLK_SEL_MASK: u32 = (0xf << 0);
pub const RT5677_DA_MONO2R_CLK_SEL_SFT: u32 = 0;

/* ASRC Control 4 (0x86) */
pub const RT5677_DA_MONO3L_CLK_SEL_MASK: u32 = (0xf << 12);
pub const RT5677_DA_MONO3L_CLK_SEL_SFT: u32 = 12;
pub const RT5677_DA_MONO3R_CLK_SEL_MASK: u32 = (0xf << 8);
pub const RT5677_DA_MONO3R_CLK_SEL_SFT: u32 = 8;
pub const RT5677_DA_MONO4L_CLK_SEL_MASK: u32 = (0xf << 4);
pub const RT5677_DA_MONO4L_CLK_SEL_SFT: u32 = 4;
pub const RT5677_DA_MONO4R_CLK_SEL_MASK: u32 = (0xf << 0);
pub const RT5677_DA_MONO4R_CLK_SEL_SFT: u32 = 0;

/* ASRC Control 5 (0x87) */
pub const RT5677_AD_STO1_CLK_SEL_MASK: u32 = (0xf << 12);
pub const RT5677_AD_STO1_CLK_SEL_SFT: u32 = 12;
pub const RT5677_AD_STO2_CLK_SEL_MASK: u32 = (0xf << 8);
pub const RT5677_AD_STO2_CLK_SEL_SFT: u32 = 8;
pub const RT5677_AD_STO3_CLK_SEL_MASK: u32 = (0xf << 4);
pub const RT5677_AD_STO3_CLK_SEL_SFT: u32 = 4;
pub const RT5677_AD_STO4_CLK_SEL_MASK: u32 = (0xf << 0);
pub const RT5677_AD_STO4_CLK_SEL_SFT: u32 = 0;

/* ASRC Control 6 (0x88) */
pub const RT5677_AD_MONOL_CLK_SEL_MASK: u32 = (0xf << 12);
pub const RT5677_AD_MONOL_CLK_SEL_SFT: u32 = 12;
pub const RT5677_AD_MONOR_CLK_SEL_MASK: u32 = (0xf << 8);
pub const RT5677_AD_MONOR_CLK_SEL_SFT: u32 = 8;

/* ASRC Control 7 (0x89) */
pub const RT5677_DSP_OB_0_3_CLK_SEL_MASK: u32 = (0xf << 12);
pub const RT5677_DSP_OB_0_3_CLK_SEL_SFT: u32 = 12;
pub const RT5677_DSP_OB_4_7_CLK_SEL_MASK: u32 = (0xf << 8);
pub const RT5677_DSP_OB_4_7_CLK_SEL_SFT: u32 = 8;

/* ASRC Control 8 (0x8a) */
pub const RT5677_I2S1_CLK_SEL_MASK: u32 = (0xf << 12);
pub const RT5677_I2S1_CLK_SEL_SFT: u32 = 12;
pub const RT5677_I2S2_CLK_SEL_MASK: u32 = (0xf << 8);
pub const RT5677_I2S2_CLK_SEL_SFT: u32 = 8;
pub const RT5677_I2S3_CLK_SEL_MASK: u32 = (0xf << 4);
pub const RT5677_I2S3_CLK_SEL_SFT: u32 = 4;
pub const RT5677_I2S4_CLK_SEL_MASK: u32 = (0xf);
pub const RT5677_I2S4_CLK_SEL_SFT: u32 = 0;

/* VAD Function Control 1 (0x9c) */
pub const RT5677_VAD_MIN_DUR_MASK: u32 = (0x3 << 13);
pub const RT5677_VAD_MIN_DUR_SFT: u32 = 13;
pub const RT5677_VAD_ADPCM_BYPASS: u32 = (1 << 10);
pub const RT5677_VAD_ADPCM_BYPASS_BIT: u32 = 10;
pub const RT5677_VAD_FG2ENC: u32 = (1 << 9);
pub const RT5677_VAD_FG2ENC_BIT: u32 = 9;
pub const RT5677_VAD_BUF_OW: u32 = (1 << 8);
pub const RT5677_VAD_BUF_OW_BIT: u32 = 8;
pub const RT5677_VAD_CLR_FLAG: u32 = (1 << 7);
pub const RT5677_VAD_CLR_FLAG_BIT: u32 = 7;
pub const RT5677_VAD_BUF_POP: u32 = (1 << 6);
pub const RT5677_VAD_BUF_POP_BIT: u32 = 6;
pub const RT5677_VAD_BUF_PUSH: u32 = (1 << 5);
pub const RT5677_VAD_BUF_PUSH_BIT: u32 = 5;
pub const RT5677_VAD_DET_ENABLE: u32 = (1 << 4);
pub const RT5677_VAD_DET_ENABLE_BIT: u32 = 4;
pub const RT5677_VAD_FUNC_ENABLE: u32 = (1 << 3);
pub const RT5677_VAD_FUNC_ENABLE_BIT: u32 = 3;
pub const RT5677_VAD_FUNC_RESET: u32 = (1 << 2);
pub const RT5677_VAD_FUNC_RESET_BIT: u32 = 2;

/* VAD Function Control 4 (0x9f) */
pub const RT5677_VAD_OUT_SRC_RATE_MASK: u32 = (0x1 << 11);
pub const RT5677_VAD_OUT_SRC_RATE_SFT: u32 = 11;
pub const RT5677_VAD_OUT_SRC_MASK: u32 = (0x1 << 10);
pub const RT5677_VAD_OUT_SRC_SFT: u32 = 10;
pub const RT5677_VAD_SRC_MASK: u32 = (0x3 << 8);
pub const RT5677_VAD_SRC_SFT: u32 = 8;
pub const RT5677_VAD_LV_DIFF_MASK: u32 = (0xff << 0);
pub const RT5677_VAD_LV_DIFF_SFT: u32 = 0;

/* DSP InBound Control (0xa3) */
pub const RT5677_IB01_SRC_MASK: u32 = (0x7 << 12);
pub const RT5677_IB01_SRC_SFT: u32 = 12;
pub const RT5677_IB23_SRC_MASK: u32 = (0x7 << 8);
pub const RT5677_IB23_SRC_SFT: u32 = 8;
pub const RT5677_IB45_SRC_MASK: u32 = (0x7 << 4);
pub const RT5677_IB45_SRC_SFT: u32 = 4;
pub const RT5677_IB6_SRC_MASK: u32 = (0x7 << 0);
pub const RT5677_IB6_SRC_SFT: u32 = 0;

/* DSP InBound Control (0xa4) */
pub const RT5677_IB7_SRC_MASK: u32 = (0x7 << 12);
pub const RT5677_IB7_SRC_SFT: u32 = 12;
pub const RT5677_IB8_SRC_MASK: u32 = (0x7 << 8);
pub const RT5677_IB8_SRC_SFT: u32 = 8;
pub const RT5677_IB9_SRC_MASK: u32 = (0x7 << 4);
pub const RT5677_IB9_SRC_SFT: u32 = 4;

/* DSP In/OutBound Control (0xa5) */
pub const RT5677_SEL_SRC_OB23: u32 = (0x1 << 4);
pub const RT5677_SEL_SRC_OB23_SFT: u32 = 4;
pub const RT5677_SEL_SRC_OB01: u32 = (0x1 << 3);
pub const RT5677_SEL_SRC_OB01_SFT: u32 = 3;
pub const RT5677_SEL_SRC_IB45: u32 = (0x1 << 2);
pub const RT5677_SEL_SRC_IB45_SFT: u32 = 2;
pub const RT5677_SEL_SRC_IB23: u32 = (0x1 << 1);
pub const RT5677_SEL_SRC_IB23_SFT: u32 = 1;
pub const RT5677_SEL_SRC_IB01: u32 = (0x1 << 0);
pub const RT5677_SEL_SRC_IB01_SFT: u32 = 0;

/* Jack Detect Control 1 (0xb5) */
pub const RT5677_SEL_GPIO_JD1_MASK: u32 = (0x3 << 14);
pub const RT5677_SEL_GPIO_JD1_SFT: u32 = 14;
pub const RT5677_SEL_GPIO_JD2_MASK: u32 = (0x3 << 12);
pub const RT5677_SEL_GPIO_JD2_SFT: u32 = 12;
pub const RT5677_SEL_GPIO_JD3_MASK: u32 = (0x3 << 10);
pub const RT5677_SEL_GPIO_JD3_SFT: u32 = 10;

/* IRQ Control 1 (0xbd) */
pub const RT5677_STA_GPIO_JD1: u32 = (0x1 << 15);
pub const RT5677_STA_GPIO_JD1_SFT: u32 = 15;
pub const RT5677_EN_IRQ_GPIO_JD1: u32 = (0x1 << 14);
pub const RT5677_EN_IRQ_GPIO_JD1_SFT: u32 = 14;
pub const RT5677_EN_GPIO_JD1_STICKY: u32 = (0x1 << 13);
pub const RT5677_EN_GPIO_JD1_STICKY_SFT: u32 = 13;
pub const RT5677_INV_GPIO_JD1: u32 = (0x1 << 12);
pub const RT5677_INV_GPIO_JD1_SFT: u32 = 12;
pub const RT5677_STA_GPIO_JD2: u32 = (0x1 << 11);
pub const RT5677_STA_GPIO_JD2_SFT: u32 = 11;
pub const RT5677_EN_IRQ_GPIO_JD2: u32 = (0x1 << 10);
pub const RT5677_EN_IRQ_GPIO_JD2_SFT: u32 = 10;
pub const RT5677_EN_GPIO_JD2_STICKY: u32 = (0x1 << 9);
pub const RT5677_EN_GPIO_JD2_STICKY_SFT: u32 = 9;
pub const RT5677_INV_GPIO_JD2: u32 = (0x1 << 8);
pub const RT5677_INV_GPIO_JD2_SFT: u32 = 8;
pub const RT5677_STA_MICBIAS1_OVCD: u32 = (0x1 << 7);
pub const RT5677_STA_MICBIAS1_OVCD_SFT: u32 = 7;
pub const RT5677_EN_IRQ_MICBIAS1_OVCD: u32 = (0x1 << 6);
pub const RT5677_EN_IRQ_MICBIAS1_OVCD_SFT: u32 = 6;
pub const RT5677_EN_MICBIAS1_OVCD_STICKY: u32 = (0x1 << 5);
pub const RT5677_EN_MICBIAS1_OVCD_STICKY_SFT: u32 = 5;
pub const RT5677_INV_MICBIAS1_OVCD: u32 = (0x1 << 4);
pub const RT5677_INV_MICBIAS1_OVCD_SFT: u32 = 4;
pub const RT5677_STA_GPIO_JD3: u32 = (0x1 << 3);
pub const RT5677_STA_GPIO_JD3_SFT: u32 = 3;
pub const RT5677_EN_IRQ_GPIO_JD3: u32 = (0x1 << 2);
pub const RT5677_EN_IRQ_GPIO_JD3_SFT: u32 = 2;
pub const RT5677_EN_GPIO_JD3_STICKY: u32 = (0x1 << 1);
pub const RT5677_EN_GPIO_JD3_STICKY_SFT: u32 = 1;
pub const RT5677_INV_GPIO_JD3: u32 = (0x1 << 0);
pub const RT5677_INV_GPIO_JD3_SFT: u32 = 0;

/* GPIO status (0xbf) */
pub const RT5677_GPIO6_STATUS_MASK: u32 = (0x1 << 5);
pub const RT5677_GPIO6_STATUS_SFT: u32 = 5;
pub const RT5677_GPIO5_STATUS_MASK: u32 = (0x1 << 4);
pub const RT5677_GPIO5_STATUS_SFT: u32 = 4;
pub const RT5677_GPIO4_STATUS_MASK: u32 = (0x1 << 3);
pub const RT5677_GPIO4_STATUS_SFT: u32 = 3;
pub const RT5677_GPIO3_STATUS_MASK: u32 = (0x1 << 2);
pub const RT5677_GPIO3_STATUS_SFT: u32 = 2;
pub const RT5677_GPIO2_STATUS_MASK: u32 = (0x1 << 1);
pub const RT5677_GPIO2_STATUS_SFT: u32 = 1;
pub const RT5677_GPIO1_STATUS_MASK: u32 = (0x1 << 0);
pub const RT5677_GPIO1_STATUS_SFT: u32 = 0;

/* GPIO Control 1 (0xc0) */
pub const RT5677_GPIO1_PIN_MASK: u32 = (0x1 << 15);
pub const RT5677_GPIO1_PIN_SFT: u32 = 15;
pub const RT5677_GPIO1_PIN_GPIO1: u32 = (0x0 << 15);
pub const RT5677_GPIO1_PIN_IRQ: u32 = (0x1 << 15);
pub const RT5677_IPTV_MODE_MASK: u32 = (0x1 << 14);
pub const RT5677_IPTV_MODE_SFT: u32 = 14;
pub const RT5677_IPTV_MODE_GPIO: u32 = (0x0 << 14);
pub const RT5677_IPTV_MODE_IPTV: u32 = (0x1 << 14);
pub const RT5677_FUNC_MODE_MASK: u32 = (0x1 << 13);
pub const RT5677_FUNC_MODE_SFT: u32 = 13;
pub const RT5677_FUNC_MODE_DMIC_GPIO: u32 = (0x0 << 13);
pub const RT5677_FUNC_MODE_JTAG: u32 = (0x1 << 13);

/* GPIO Control 2 (0xc1) & 3 (0xc2) common bits */
pub const RT5677_GPIOx_DIR_MASK: u32 = (0x1 << 2);
pub const RT5677_GPIOx_DIR_SFT: u32 = 2;
pub const RT5677_GPIOx_DIR_IN: u32 = (0x0 << 2);
pub const RT5677_GPIOx_DIR_OUT: u32 = (0x1 << 2);
pub const RT5677_GPIOx_OUT_MASK: u32 = (0x1 << 1);
pub const RT5677_GPIOx_OUT_SFT: u32 = 1;
pub const RT5677_GPIOx_OUT_LO: u32 = (0x0 << 1);
pub const RT5677_GPIOx_OUT_HI: u32 = (0x1 << 1);
pub const RT5677_GPIOx_P_MASK: u32 = (0x1 << 0);
pub const RT5677_GPIOx_P_SFT: u32 = 0;
pub const RT5677_GPIOx_P_NOR: u32 = (0x0 << 0);
pub const RT5677_GPIOx_P_INV: u32 = (0x1 << 0);

/* General Control (0xfa) */
pub const RT5677_IRQ_DEBOUNCE_SEL_MASK: u32 = (0x3 << 3);
pub const RT5677_IRQ_DEBOUNCE_SEL_MCLK: u32 = (0x0 << 3);
pub const RT5677_IRQ_DEBOUNCE_SEL_RC: u32 = (0x1 << 3);
pub const RT5677_IRQ_DEBOUNCE_SEL_SLIM: u32 = (0x2 << 3);

/* Virtual DSP Mixer Control (0xf7 0xf8 0xf9) */
pub const RT5677_DSP_IB_01_H: u32 = (0x1 << 15);
pub const RT5677_DSP_IB_01_H_SFT: u32 = 15;
pub const RT5677_DSP_IB_23_H: u32 = (0x1 << 14);
pub const RT5677_DSP_IB_23_H_SFT: u32 = 14;
pub const RT5677_DSP_IB_45_H: u32 = (0x1 << 13);
pub const RT5677_DSP_IB_45_H_SFT: u32 = 13;
pub const RT5677_DSP_IB_6_H: u32 = (0x1 << 12);
pub const RT5677_DSP_IB_6_H_SFT: u32 = 12;
pub const RT5677_DSP_IB_7_H: u32 = (0x1 << 11);
pub const RT5677_DSP_IB_7_H_SFT: u32 = 11;
pub const RT5677_DSP_IB_8_H: u32 = (0x1 << 10);
pub const RT5677_DSP_IB_8_H_SFT: u32 = 10;
pub const RT5677_DSP_IB_9_H: u32 = (0x1 << 9);
pub const RT5677_DSP_IB_9_H_SFT: u32 = 9;
pub const RT5677_DSP_IB_01_L: u32 = (0x1 << 7);
pub const RT5677_DSP_IB_01_L_SFT: u32 = 7;
pub const RT5677_DSP_IB_23_L: u32 = (0x1 << 6);
pub const RT5677_DSP_IB_23_L_SFT: u32 = 6;
pub const RT5677_DSP_IB_45_L: u32 = (0x1 << 5);
pub const RT5677_DSP_IB_45_L_SFT: u32 = 5;
pub const RT5677_DSP_IB_6_L: u32 = (0x1 << 4);
pub const RT5677_DSP_IB_6_L_SFT: u32 = 4;
pub const RT5677_DSP_IB_7_L: u32 = (0x1 << 3);
pub const RT5677_DSP_IB_7_L_SFT: u32 = 3;
pub const RT5677_DSP_IB_8_L: u32 = (0x1 << 2);
pub const RT5677_DSP_IB_8_L_SFT: u32 = 2;
pub const RT5677_DSP_IB_9_L: u32 = (0x1 << 1);
pub const RT5677_DSP_IB_9_L_SFT: u32 = 1;

/* General Control2 (0xfc)*/
pub const RT5677_GPIO5_FUNC_MASK: u32 = (0x1 << 9);
pub const RT5677_GPIO5_FUNC_GPIO: u32 = (0x0 << 9);
pub const RT5677_GPIO5_FUNC_DMIC: u32 = (0x1 << 9);

pub const RT5677_FIRMWARE1: &str = "rt5677_dsp_fw1.bin";
pub const RT5677_FIRMWARE2: &str = "rt5677_dsp_fw2.bin";

pub const RT5677_DRV_NAME: &str = "rt5677";

/* System Clock Source */
pub const RT5677_SCLK_S_MCLK: u32 = 0;
pub const RT5677_SCLK_S_PLL1: u32 = 1;
pub const RT5677_SCLK_S_RCCLK: u32 = 2;

/* PLL1 Source */
pub const RT5677_PLL1_S_MCLK: u32 = 0;
pub const RT5677_PLL1_S_BCLK1: u32 = 1;
pub const RT5677_PLL1_S_BCLK2: u32 = 2;
pub const RT5677_PLL1_S_BCLK3: u32 = 3;
pub const RT5677_PLL1_S_BCLK4: u32 = 4;

pub const RT5677_AIF1: usize = 0;
pub const RT5677_AIF2: usize = 1;
pub const RT5677_AIF3: usize = 2;
pub const RT5677_AIF4: usize = 3;
pub const RT5677_AIF5: usize = 4;
pub const RT5677_AIFS: usize = 5;
pub const RT5677_DSPBUFF: usize = 6;

pub const RT5677_GPIO1: u32 = 0;
pub const RT5677_GPIO2: u32 = 1;
pub const RT5677_GPIO3: u32 = 2;
pub const RT5677_GPIO4: u32 = 3;
pub const RT5677_GPIO5: u32 = 4;
pub const RT5677_GPIO6: u32 = 5;
pub const RT5677_GPIO_NUM: u32 = 6;

pub const RT5677_IRQ_JD1: u32 = 0;
pub const RT5677_IRQ_JD2: u32 = 1;
pub const RT5677_IRQ_JD3: u32 = 2;
pub const RT5677_IRQ_NUM: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rt5677_type {
    RT5677 = 1,
    RT5676 = 2,
}

/* ASRC clock source selection */
pub const RT5677_CLK_SEL_SYS: u32 = 0;
pub const RT5677_CLK_SEL_I2S1_ASRC: u32 = 1;
pub const RT5677_CLK_SEL_I2S2_ASRC: u32 = 2;
pub const RT5677_CLK_SEL_I2S3_ASRC: u32 = 3;
pub const RT5677_CLK_SEL_I2S4_ASRC: u32 = 4;
pub const RT5677_CLK_SEL_I2S5_ASRC: u32 = 5;
pub const RT5677_CLK_SEL_I2S6_ASRC: u32 = 6;
pub const RT5677_CLK_SEL_SYS2: u32 = 7;
pub const RT5677_CLK_SEL_SYS3: u32 = 8;
pub const RT5677_CLK_SEL_SYS4: u32 = 9;
pub const RT5677_CLK_SEL_SYS5: u32 = 10;
pub const RT5677_CLK_SEL_SYS6: u32 = 11;
pub const RT5677_CLK_SEL_SYS7: u32 = 12;

/* filter mask */
pub const RT5677_DA_STEREO_FILTER: u32 = 0x1;
pub const RT5677_DA_MONO2_L_FILTER: u32 = 0x1 << 1;
pub const RT5677_DA_MONO2_R_FILTER: u32 = 0x1 << 2;
pub const RT5677_DA_MONO3_L_FILTER: u32 = 0x1 << 3;
pub const RT5677_DA_MONO3_R_FILTER: u32 = 0x1 << 4;
pub const RT5677_DA_MONO4_L_FILTER: u32 = 0x1 << 5;
pub const RT5677_DA_MONO4_R_FILTER: u32 = 0x1 << 6;
pub const RT5677_AD_STEREO1_FILTER: u32 = 0x1 << 7;
pub const RT5677_AD_STEREO2_FILTER: u32 = 0x1 << 8;
pub const RT5677_AD_STEREO3_FILTER: u32 = 0x1 << 9;
pub const RT5677_AD_STEREO4_FILTER: u32 = 0x1 << 10;
pub const RT5677_AD_MONO_L_FILTER: u32 = 0x1 << 11;
pub const RT5677_AD_MONO_R_FILTER: u32 = 0x1 << 12;
pub const RT5677_DSP_OB_0_3_FILTER: u32 = 0x1 << 13;
pub const RT5677_DSP_OB_4_7_FILTER: u32 = 0x1 << 14;
pub const RT5677_I2S1_SOURCE: u32 = 0x1 << 15;
pub const RT5677_I2S2_SOURCE: u32 = 0x1 << 16;
pub const RT5677_I2S3_SOURCE: u32 = 0x1 << 17;
pub const RT5677_I2S4_SOURCE: u32 = 0x1 << 18;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rt5677_dmic2_clk {
    RT5677_DMIC_CLK1 = 0,
    RT5677_DMIC_CLK2 = 1,
}

#[repr(C)]
pub struct rt5677_platform_data {
    /* IN1/IN2/LOUT1/LOUT2/LOUT3 can optionally be differential */
    pub in1_diff: bool,
    pub in2_diff: bool,
    pub lout1_diff: bool,
    pub lout2_diff: bool,
    pub lout3_diff: bool,
    /* DMIC2 clock source selection */
    pub dmic2_clk_pin: rt5677_dmic2_clk,

    /* configures GPIO, 0 - floating, 1 - pulldown, 2 - pullup */
    pub gpio_config: [u8; 6],

    /* jd1 can select 0 ~ 3 as OFF, GPIO1, GPIO2 and GPIO3 respectively */
    pub jd1_gpio: u32,
    /* jd2 and jd3 can select 0 ~ 3 as
        OFF, GPIO4, GPIO5 and GPIO6 respectively */
    pub jd2_gpio: u32,
    pub jd3_gpio: u32,

    /* Set MICBIAS1 VDD 1v8 or 3v3 */
    pub micbias1_vdd_3v3: bool,
}

// External C types supplied by other translated headers.
pub enum snd_soc_component {}
pub enum device {}
pub enum regmap {}
pub enum firmware {}
pub enum mutex {}
pub enum gpio_desc {}
pub enum gpio_chip {}
pub enum delayed_work {}
pub enum irq_domain {}

#[repr(C)]
pub struct rt5677_priv {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
    pub pdata: rt5677_platform_data,
    pub regmap: *mut regmap,
    pub regmap_physical: *mut regmap,
    pub fw1: *const firmware,
    pub fw2: *const firmware,
    pub dsp_cmd_lock: mutex,
    pub dsp_pri_lock: mutex,

    pub sysclk: i32,
    pub sysclk_src: i32,
    pub lrck: [i32; RT5677_AIFS],
    pub bclk: [i32; RT5677_AIFS],
    pub master: [i32; RT5677_AIFS],
    pub pll_src: i32,
    pub pll_in: i32,
    pub pll_out: i32,
    pub pow_ldo2: *mut gpio_desc, /* POW_LDO2 pin */
    pub reset_pin: *mut gpio_desc, /* RESET pin */
    pub type_: rt5677_type,
    // Present in C only when CONFIG_GPIOLIB is enabled.
    #[cfg(CONFIG_GPIOLIB)]
    pub gpio_chip: gpio_chip,
    pub dsp_vad_en_request: bool, /* DSP VAD enable/disable request */
    pub dsp_vad_en: bool, /* dsp_work parameter */
    pub is_dsp_mode: bool,
    pub is_vref_slow: bool,
    pub dsp_work: delayed_work,

    /* Interrupt handling */
    pub domain: *mut irq_domain,
    pub irq_lock: mutex,
    pub irq_en: u32,
    pub resume_irq_check: delayed_work,
    pub irq: i32,

    pub set_dsp_vad: Option<unsafe extern "C" fn(component: *mut snd_soc_component, on: bool) -> i32>,
}

unsafe extern "C" {
    pub fn rt5677_sel_asrc_clk_src(
        component: *mut snd_soc_component,
        filter_mask: u32,
        clk_src: u32,
    ) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
