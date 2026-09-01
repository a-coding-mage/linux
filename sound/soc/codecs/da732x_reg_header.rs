/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * da732x_reg.h --- Dialog DA732X ALSA SoC Audio Registers Header File
 *
 * Copyright (C) 2012 Dialog Semiconductor GmbH
 *
 * Author: Michal Hajduk <Michal.Hajduk@diasemi.com>
 */


/* DA732X registers */
pub const DA732X_REG_STATUS_EXT: u32 = 0x00;
pub const DA732X_REG_STATUS: u32 = 0x01;
pub const DA732X_REG_REF1: u32 = 0x02;
pub const DA732X_REG_BIAS_EN: u32 = 0x03;
pub const DA732X_REG_BIAS1: u32 = 0x04;
pub const DA732X_REG_BIAS2: u32 = 0x05;
pub const DA732X_REG_BIAS3: u32 = 0x06;
pub const DA732X_REG_BIAS4: u32 = 0x07;
pub const DA732X_REG_MICBIAS2: u32 = 0x0F;
pub const DA732X_REG_MICBIAS1: u32 = 0x10;
pub const DA732X_REG_MICDET: u32 = 0x11;
pub const DA732X_REG_MIC1_PRE: u32 = 0x12;
pub const DA732X_REG_MIC1: u32 = 0x13;
pub const DA732X_REG_MIC2_PRE: u32 = 0x14;
pub const DA732X_REG_MIC2: u32 = 0x15;
pub const DA732X_REG_AUX1L: u32 = 0x16;
pub const DA732X_REG_AUX1R: u32 = 0x17;
pub const DA732X_REG_MIC3_PRE: u32 = 0x18;
pub const DA732X_REG_MIC3: u32 = 0x19;
pub const DA732X_REG_INP_PINBIAS: u32 = 0x1A;
pub const DA732X_REG_INP_ZC_EN: u32 = 0x1B;
pub const DA732X_REG_INP_MUX: u32 = 0x1D;
pub const DA732X_REG_HP_DET: u32 = 0x20;
pub const DA732X_REG_HPL_DAC_OFFSET: u32 = 0x21;
pub const DA732X_REG_HPL_DAC_OFF_CNTL: u32 = 0x22;
pub const DA732X_REG_HPL_OUT_OFFSET: u32 = 0x23;
pub const DA732X_REG_HPL: u32 = 0x24;
pub const DA732X_REG_HPL_VOL: u32 = 0x25;
pub const DA732X_REG_HPR_DAC_OFFSET: u32 = 0x26;
pub const DA732X_REG_HPR_DAC_OFF_CNTL: u32 = 0x27;
pub const DA732X_REG_HPR_OUT_OFFSET: u32 = 0x28;
pub const DA732X_REG_HPR: u32 = 0x29;
pub const DA732X_REG_HPR_VOL: u32 = 0x2A;
pub const DA732X_REG_LIN2: u32 = 0x2B;
pub const DA732X_REG_LIN3: u32 = 0x2C;
pub const DA732X_REG_LIN4: u32 = 0x2D;
pub const DA732X_REG_OUT_ZC_EN: u32 = 0x2E;
pub const DA732X_REG_HP_LIN1_GNDSEL: u32 = 0x37;
pub const DA732X_REG_CP_HP1: u32 = 0x3A;
pub const DA732X_REG_CP_HP2: u32 = 0x3B;
pub const DA732X_REG_CP_CTRL1: u32 = 0x40;
pub const DA732X_REG_CP_CTRL2: u32 = 0x41;
pub const DA732X_REG_CP_CTRL3: u32 = 0x42;
pub const DA732X_REG_CP_LEVEL_MASK: u32 = 0x43;
pub const DA732X_REG_CP_DET: u32 = 0x44;
pub const DA732X_REG_CP_STATUS: u32 = 0x45;
pub const DA732X_REG_CP_THRESH1: u32 = 0x46;
pub const DA732X_REG_CP_THRESH2: u32 = 0x47;
pub const DA732X_REG_CP_THRESH3: u32 = 0x48;
pub const DA732X_REG_CP_THRESH4: u32 = 0x49;
pub const DA732X_REG_CP_THRESH5: u32 = 0x4A;
pub const DA732X_REG_CP_THRESH6: u32 = 0x4B;
pub const DA732X_REG_CP_THRESH7: u32 = 0x4C;
pub const DA732X_REG_CP_THRESH8: u32 = 0x4D;
pub const DA732X_REG_PLL_DIV_LO: u32 = 0x50;
pub const DA732X_REG_PLL_DIV_MID: u32 = 0x51;
pub const DA732X_REG_PLL_DIV_HI: u32 = 0x52;
pub const DA732X_REG_PLL_CTRL: u32 = 0x53;
pub const DA732X_REG_CLK_CTRL: u32 = 0x54;
pub const DA732X_REG_CLK_DSP: u32 = 0x5A;
pub const DA732X_REG_CLK_EN1: u32 = 0x5B;
pub const DA732X_REG_CLK_EN2: u32 = 0x5C;
pub const DA732X_REG_CLK_EN3: u32 = 0x5D;
pub const DA732X_REG_CLK_EN4: u32 = 0x5E;
pub const DA732X_REG_CLK_EN5: u32 = 0x5F;
pub const DA732X_REG_AIF_MCLK: u32 = 0x60;
pub const DA732X_REG_AIFA1: u32 = 0x61;
pub const DA732X_REG_AIFA2: u32 = 0x62;
pub const DA732X_REG_AIFA3: u32 = 0x63;
pub const DA732X_REG_AIFB1: u32 = 0x64;
pub const DA732X_REG_AIFB2: u32 = 0x65;
pub const DA732X_REG_AIFB3: u32 = 0x66;
pub const DA732X_REG_PC_CTRL: u32 = 0x6A;
pub const DA732X_REG_DATA_ROUTE: u32 = 0x70;
pub const DA732X_REG_DSP_CTRL: u32 = 0x71;
pub const DA732X_REG_CIF_CTRL2: u32 = 0x74;
pub const DA732X_REG_HANDSHAKE: u32 = 0x75;
pub const DA732X_REG_MBOX0: u32 = 0x76;
pub const DA732X_REG_MBOX1: u32 = 0x77;
pub const DA732X_REG_MBOX2: u32 = 0x78;
pub const DA732X_REG_MBOX_STATUS: u32 = 0x79;
pub const DA732X_REG_SPARE1_OUT: u32 = 0x7D;
pub const DA732X_REG_SPARE2_OUT: u32 = 0x7E;
pub const DA732X_REG_SPARE1_IN: u32 = 0x7F;
pub const DA732X_REG_ID: u32 = 0x81;
pub const DA732X_REG_ADC1_PD: u32 = 0x90;
pub const DA732X_REG_ADC1_HPF: u32 = 0x93;
pub const DA732X_REG_ADC1_SEL: u32 = 0x94;
pub const DA732X_REG_ADC1_EQ12: u32 = 0x95;
pub const DA732X_REG_ADC1_EQ34: u32 = 0x96;
pub const DA732X_REG_ADC1_EQ5: u32 = 0x97;
pub const DA732X_REG_ADC2_PD: u32 = 0x98;
pub const DA732X_REG_ADC2_HPF: u32 = 0x9B;
pub const DA732X_REG_ADC2_SEL: u32 = 0x9C;
pub const DA732X_REG_ADC2_EQ12: u32 = 0x9D;
pub const DA732X_REG_ADC2_EQ34: u32 = 0x9E;
pub const DA732X_REG_ADC2_EQ5: u32 = 0x9F;
pub const DA732X_REG_DAC1_HPF: u32 = 0xA0;
pub const DA732X_REG_DAC1_L_VOL: u32 = 0xA1;
pub const DA732X_REG_DAC1_R_VOL: u32 = 0xA2;
pub const DA732X_REG_DAC1_SEL: u32 = 0xA3;
pub const DA732X_REG_DAC1_SOFTMUTE: u32 = 0xA4;
pub const DA732X_REG_DAC1_EQ12: u32 = 0xA5;
pub const DA732X_REG_DAC1_EQ34: u32 = 0xA6;
pub const DA732X_REG_DAC1_EQ5: u32 = 0xA7;
pub const DA732X_REG_DAC2_HPF: u32 = 0xB0;
pub const DA732X_REG_DAC2_L_VOL: u32 = 0xB1;
pub const DA732X_REG_DAC2_R_VOL: u32 = 0xB2;
pub const DA732X_REG_DAC2_SEL: u32 = 0xB3;
pub const DA732X_REG_DAC2_SOFTMUTE: u32 = 0xB4;
pub const DA732X_REG_DAC2_EQ12: u32 = 0xB5;
pub const DA732X_REG_DAC2_EQ34: u32 = 0xB6;
pub const DA732X_REG_DAC2_EQ5: u32 = 0xB7;
pub const DA732X_REG_DAC3_HPF: u32 = 0xC0;
pub const DA732X_REG_DAC3_VOL: u32 = 0xC1;
pub const DA732X_REG_DAC3_SEL: u32 = 0xC3;
pub const DA732X_REG_DAC3_SOFTMUTE: u32 = 0xC4;
pub const DA732X_REG_DAC3_EQ12: u32 = 0xC5;
pub const DA732X_REG_DAC3_EQ34: u32 = 0xC6;
pub const DA732X_REG_DAC3_EQ5: u32 = 0xC7;
pub const DA732X_REG_BIQ_BYP: u32 = 0xD2;
pub const DA732X_REG_DMA_CMD: u32 = 0xD3;
pub const DA732X_REG_DMA_ADDR0: u32 = 0xD4;
pub const DA732X_REG_DMA_ADDR1: u32 = 0xD5;
pub const DA732X_REG_DMA_DATA0: u32 = 0xD6;
pub const DA732X_REG_DMA_DATA1: u32 = 0xD7;
pub const DA732X_REG_DMA_DATA2: u32 = 0xD8;
pub const DA732X_REG_DMA_DATA3: u32 = 0xD9;
pub const DA732X_REG_DMA_STATUS: u32 = 0xDA;
pub const DA732X_REG_BROWNOUT: u32 = 0xDF;
pub const DA732X_REG_UNLOCK: u32 = 0xE0;

pub const DA732X_MAX_REG: u32 = DA732X_REG_UNLOCK;
/*
 * Bits
 */

/* DA732X_REG_STATUS_EXT (addr=0x00) */
pub const DA732X_STATUS_EXT_DSP: u32 = (1 << 4);
pub const DA732X_STATUS_EXT_CLEAR: u32 = (0 << 0);

/* DA732X_REG_STATUS	(addr=0x01) */
pub const DA732X_STATUS_PLL_LOCK: u32 = (1 << 0);
pub const DA732X_STATUS_PLL_MCLK_DET: u32 = (1 << 1);
pub const DA732X_STATUS_HPDET_OUT: u32 = (1 << 2);
pub const DA732X_STATUS_INP_MIXDET_1: u32 = (1 << 3);
pub const DA732X_STATUS_INP_MIXDET_2: u32 = (1 << 4);
pub const DA732X_STATUS_BO_STATUS: u32 = (1 << 5);

/* DA732X_REG_REF1	(addr=0x02) */
pub const DA732X_VMID_FASTCHG: u32 = (1 << 1);
pub const DA732X_VMID_FASTDISCHG: u32 = (1 << 2);
pub const DA732X_REFBUFX2_EN: u32 = (1 << 6);
pub const DA732X_REFBUFX2_DIS: u32 = (0 << 6);

/* DA732X_REG_BIAS_EN	(addr=0x03) */
pub const DA732X_BIAS_BOOST_MASK: u32 = (3 << 0);
pub const DA732X_BIAS_BOOST_100PC: u32 = (0 << 0);
pub const DA732X_BIAS_BOOST_133PC: u32 = (1 << 0);
pub const DA732X_BIAS_BOOST_88PC: u32 = (2 << 0);
pub const DA732X_BIAS_BOOST_50PC: u32 = (3 << 0);
pub const DA732X_BIAS_EN: u32 = (1 << 7);
pub const DA732X_BIAS_DIS: u32 = (0 << 7);

/* DA732X_REG_BIAS1	(addr=0x04) */
pub const DA732X_BIAS1_HP_DAC_BIAS_MASK: u32 = (3 << 0);
pub const DA732X_BIAS1_HP_DAC_BIAS_100PC: u32 = (0 << 0);
pub const DA732X_BIAS1_HP_DAC_BIAS_150PC: u32 = (1 << 0);
pub const DA732X_BIAS1_HP_DAC_BIAS_50PC: u32 = (2 << 0);
pub const DA732X_BIAS1_HP_DAC_BIAS_75PC: u32 = (3 << 0);
pub const DA732X_BIAS1_HP_OUT_BIAS_MASK: u32 = (7 << 4);
pub const DA732X_BIAS1_HP_OUT_BIAS_100PC: u32 = (0 << 4);
pub const DA732X_BIAS1_HP_OUT_BIAS_125PC: u32 = (1 << 4);
pub const DA732X_BIAS1_HP_OUT_BIAS_150PC: u32 = (2 << 4);
pub const DA732X_BIAS1_HP_OUT_BIAS_175PC: u32 = (3 << 4);
pub const DA732X_BIAS1_HP_OUT_BIAS_200PC: u32 = (4 << 4);
pub const DA732X_BIAS1_HP_OUT_BIAS_250PC: u32 = (5 << 4);
pub const DA732X_BIAS1_HP_OUT_BIAS_300PC: u32 = (6 << 4);
pub const DA732X_BIAS1_HP_OUT_BIAS_350PC: u32 = (7 << 4);

/* DA732X_REG_BIAS2	(addr=0x05) */
pub const DA732X_BIAS2_LINE2_DAC_BIAS_MASK: u32 = (3 << 0);
pub const DA732X_BIAS2_LINE2_DAC_BIAS_100PC: u32 = (0 << 0);
pub const DA732X_BIAS2_LINE2_DAC_BIAS_150PC: u32 = (1 << 0);
pub const DA732X_BIAS2_LINE2_DAC_BIAS_50PC: u32 = (2 << 0);
pub const DA732X_BIAS2_LINE2_DAC_BIAS_75PC: u32 = (3 << 0);
pub const DA732X_BIAS2_LINE2_OUT_BIAS_MASK: u32 = (7 << 4);
pub const DA732X_BIAS2_LINE2_OUT_BIAS_100PC: u32 = (0 << 4);
pub const DA732X_BIAS2_LINE2_OUT_BIAS_125PC: u32 = (1 << 4);
pub const DA732X_BIAS2_LINE2_OUT_BIAS_150PC: u32 = (2 << 4);
pub const DA732X_BIAS2_LINE2_OUT_BIAS_175PC: u32 = (3 << 4);
pub const DA732X_BIAS2_LINE2_OUT_BIAS_200PC: u32 = (4 << 4);
pub const DA732X_BIAS2_LINE2_OUT_BIAS_250PC: u32 = (5 << 4);
pub const DA732X_BIAS2_LINE2_OUT_BIAS_300PC: u32 = (6 << 4);
pub const DA732X_BIAS2_LINE2_OUT_BIAS_350PC: u32 = (7 << 4);

/* DA732X_REG_BIAS3	(addr=0x06) */
pub const DA732X_BIAS3_LINE3_DAC_BIAS_MASK: u32 = (3 << 0);
pub const DA732X_BIAS3_LINE3_DAC_BIAS_100PC: u32 = (0 << 0);
pub const DA732X_BIAS3_LINE3_DAC_BIAS_150PC: u32 = (1 << 0);
pub const DA732X_BIAS3_LINE3_DAC_BIAS_50PC: u32 = (2 << 0);
pub const DA732X_BIAS3_LINE3_DAC_BIAS_75PC: u32 = (3 << 0);
pub const DA732X_BIAS3_LINE3_OUT_BIAS_MASK: u32 = (7 << 4);
pub const DA732X_BIAS3_LINE3_OUT_BIAS_100PC: u32 = (0 << 4);
pub const DA732X_BIAS3_LINE3_OUT_BIAS_125PC: u32 = (1 << 4);
pub const DA732X_BIAS3_LINE3_OUT_BIAS_150PC: u32 = (2 << 4);
pub const DA732X_BIAS3_LINE3_OUT_BIAS_175PC: u32 = (3 << 4);
pub const DA732X_BIAS3_LINE3_OUT_BIAS_200PC: u32 = (4 << 4);
pub const DA732X_BIAS3_LINE3_OUT_BIAS_250PC: u32 = (5 << 4);
pub const DA732X_BIAS3_LINE3_OUT_BIAS_300PC: u32 = (6 << 4);
pub const DA732X_BIAS3_LINE3_OUT_BIAS_350PC: u32 = (7 << 4);

/* DA732X_REG_BIAS4	(addr=0x07) */
pub const DA732X_BIAS4_LINE4_DAC_BIAS_MASK: u32 = (3 << 0);
pub const DA732X_BIAS4_LINE4_DAC_BIAS_100PC: u32 = (0 << 0);
pub const DA732X_BIAS4_LINE4_DAC_BIAS_150PC: u32 = (1 << 0);
pub const DA732X_BIAS4_LINE4_DAC_BIAS_50PC: u32 = (2 << 0);
pub const DA732X_BIAS4_LINE4_DAC_BIAS_75PC: u32 = (3 << 0);
pub const DA732X_BIAS4_LINE4_OUT_BIAS_MASK: u32 = (7 << 4);
pub const DA732X_BIAS4_LINE4_OUT_BIAS_100PC: u32 = (0 << 4);
pub const DA732X_BIAS4_LINE4_OUT_BIAS_125PC: u32 = (1 << 4);
pub const DA732X_BIAS4_LINE4_OUT_BIAS_150PC: u32 = (2 << 4);
pub const DA732X_BIAS4_LINE4_OUT_BIAS_175PC: u32 = (3 << 4);
pub const DA732X_BIAS4_LINE4_OUT_BIAS_200PC: u32 = (4 << 4);
pub const DA732X_BIAS4_LINE4_OUT_BIAS_250PC: u32 = (5 << 4);
pub const DA732X_BIAS4_LINE4_OUT_BIAS_300PC: u32 = (6 << 4);
pub const DA732X_BIAS4_LINE4_OUT_BIAS_350PC: u32 = (7 << 4);

/* DA732X_REG_SIF_VDD_SEL	(addr=0x08) */
pub const DA732X_SIF_VDD_SEL_AIFA_VDD2: u32 = (1 << 0);
pub const DA732X_SIF_VDD_SEL_AIFB_VDD2: u32 = (1 << 1);
pub const DA732X_SIF_VDD_SEL_CIFA_VDD2: u32 = (1 << 4);

/* DA732X_REG_MICBIAS2/1	(addr=0x0F/0x10) */
pub const DA732X_MICBIAS_VOLTAGE_MASK: u32 = (0x0F << 0);
pub const DA732X_MICBIAS_VOLTAGE_2V: u32 = (0x00 << 0);
pub const DA732X_MICBIAS_VOLTAGE_2V05: u32 = (0x01 << 0);
pub const DA732X_MICBIAS_VOLTAGE_2V1: u32 = (0x02 << 0);
pub const DA732X_MICBIAS_VOLTAGE_2V15: u32 = (0x03 << 0);
pub const DA732X_MICBIAS_VOLTAGE_2V2: u32 = (0x04 << 0);
pub const DA732X_MICBIAS_VOLTAGE_2V25: u32 = (0x05 << 0);
pub const DA732X_MICBIAS_VOLTAGE_2V3: u32 = (0x06 << 0);
pub const DA732X_MICBIAS_VOLTAGE_2V35: u32 = (0x07 << 0);
pub const DA732X_MICBIAS_VOLTAGE_2V4: u32 = (0x08 << 0);
pub const DA732X_MICBIAS_VOLTAGE_2V45: u32 = (0x09 << 0);
pub const DA732X_MICBIAS_VOLTAGE_2V5: u32 = (0x0A << 0);
pub const DA732X_MICBIAS_EN: u32 = (1 << 7);
pub const DA732X_MICBIAS_EN_SHIFT: u32 = 7;
pub const DA732X_MICBIAS_VOLTAGE_SHIFT: u32 = 0;
pub const DA732X_MICBIAS_VOLTAGE_MAX: u32 = 0x0B;

/* DA732X_REG_MICDET	(addr=0x11) */
pub const DA732X_MICDET_INP_MICRES: u32 = (1 << 0);
pub const DA732X_MICDET_INP_MICHOOK: u32 = (1 << 1);
pub const DA732X_MICDET_INP_DEBOUNCE_PRD_8MS: u32 = (0 << 0);
pub const DA732X_MICDET_INP_DEBOUNCE_PRD_16MS: u32 = (1 << 0);
pub const DA732X_MICDET_INP_DEBOUNCE_PRD_32MS: u32 = (2 << 0);
pub const DA732X_MICDET_INP_DEBOUNCE_PRD_64MS: u32 = (3 << 0);
pub const DA732X_MICDET_INP_MICDET_EN: u32 = (1 << 7);

/* DA732X_REG_MIC1/2/3_PRE (addr=0x11/0x14/0x18) */
pub const DA732X_MICBOOST_MASK: u32 = 0x7;
pub const DA732X_MICBOOST_SHIFT: u32 = 0;
pub const DA732X_MICBOOST_MIN: u32 = 0x1;
pub const DA732X_MICBOOST_MAX: u32 = DA732X_MICBOOST_MASK;

/* DA732X_REG_MIC1/2/3	(addr=0x13/0x15/0x19) */
pub const DA732X_MIC_VOL_SHIFT: u32 = 0;
pub const DA732X_MIC_VOL_VAL_MASK: u32 = 0x1F;
pub const DA732X_MIC_MUTE_SHIFT: u32 = 6;
pub const DA732X_MIC_EN_SHIFT: u32 = 7;
pub const DA732X_MIC_VOL_VAL_MIN: u32 = 0x7;
pub const DA732X_MIC_VOL_VAL_MAX: u32 = DA732X_MIC_VOL_VAL_MASK;

/* DA732X_REG_AUX1L/R	(addr=0x16/0x17) */
pub const DA732X_AUX_VOL_SHIFT: u32 = 0;
pub const DA732X_AUX_VOL_MASK: u32 = 0x7;
pub const DA732X_AUX_MUTE_SHIFT: u32 = 6;
pub const DA732X_AUX_EN_SHIFT: u32 = 7;
pub const DA732X_AUX_VOL_VAL_MAX: u32 = DA732X_AUX_VOL_MASK;

/* DA732X_REG_INP_PINBIAS	(addr=0x1A) */
pub const DA732X_INP_MICL_PINBIAS_EN: u32 = (1 << 0);
pub const DA732X_INP_MICR_PINBIAS_EN: u32 = (1 << 1);
pub const DA732X_INP_AUX1L_PINBIAS_EN: u32 = (1 << 2);
pub const DA732X_INP_AUX1R_PINBIAS_EN: u32 = (1 << 3);
pub const DA732X_INP_AUX2_PINBIAS_EN: u32 = (1 << 4);

/* DA732X_REG_INP_ZC_EN	(addr=0x1B) */
pub const DA732X_MIC1_PRE_ZC_EN: u32 = (1 << 0);
pub const DA732X_MIC1_ZC_EN: u32 = (1 << 1);
pub const DA732X_MIC2_PRE_ZC_EN: u32 = (1 << 2);
pub const DA732X_MIC2_ZC_EN: u32 = (1 << 3);
pub const DA732X_AUXL_ZC_EN: u32 = (1 << 4);
pub const DA732X_AUXR_ZC_EN: u32 = (1 << 5);
pub const DA732X_MIC3_PRE_ZC_EN: u32 = (1 << 6);
pub const DA732X_MIC3_ZC_EN: u32 = (1 << 7);

/* DA732X_REG_INP_MUX	(addr=0x1D) */
pub const DA732X_INP_ADC1L_MUX_SEL_AUX1L: u32 = (0 << 0);
pub const DA732X_INP_ADC1L_MUX_SEL_MIC1: u32 = (1 << 0);
pub const DA732X_INP_ADC1R_MUX_SEL_MASK: u32 = (3 << 2);
pub const DA732X_INP_ADC1R_MUX_SEL_AUX1R: u32 = (0 << 2);
pub const DA732X_INP_ADC1R_MUX_SEL_MIC2: u32 = (1 << 2);
pub const DA732X_INP_ADC1R_MUX_SEL_MIC3: u32 = (2 << 2);
pub const DA732X_INP_ADC2L_MUX_SEL_AUX1L: u32 = (0 << 4);
pub const DA732X_INP_ADC2L_MUX_SEL_MICL: u32 = (1 << 4);
pub const DA732X_INP_ADC2R_MUX_SEL_MASK: u32 = (3 << 6);
pub const DA732X_INP_ADC2R_MUX_SEL_AUX1R: u32 = (0 << 6);
pub const DA732X_INP_ADC2R_MUX_SEL_MICR: u32 = (1 << 6);
pub const DA732X_INP_ADC2R_MUX_SEL_AUX2: u32 = (2 << 6);
pub const DA732X_ADC1L_MUX_SEL_SHIFT: u32 = 0;
pub const DA732X_ADC1R_MUX_SEL_SHIFT: u32 = 2;
pub const DA732X_ADC2L_MUX_SEL_SHIFT: u32 = 4;
pub const DA732X_ADC2R_MUX_SEL_SHIFT: u32 = 6;

/* DA732X_REG_HP_DET		(addr=0x20) */
pub const DA732X_HP_DET_AZ: u32 = (1 << 0);
pub const DA732X_HP_DET_SEL1: u32 = (1 << 1);
pub const DA732X_HP_DET_IS_MASK: u32 = (3 << 2);
pub const DA732X_HP_DET_IS_0_5UA: u32 = (0 << 2);
pub const DA732X_HP_DET_IS_1UA: u32 = (1 << 2);
pub const DA732X_HP_DET_IS_2UA: u32 = (2 << 2);
pub const DA732X_HP_DET_IS_4UA: u32 = (3 << 2);
pub const DA732X_HP_DET_RS_MASK: u32 = (3 << 4);
pub const DA732X_HP_DET_RS_INFINITE: u32 = (0 << 4);
pub const DA732X_HP_DET_RS_100KOHM: u32 = (1 << 4);
pub const DA732X_HP_DET_RS_10KOHM: u32 = (2 << 4);
pub const DA732X_HP_DET_RS_1KOHM: u32 = (3 << 4);
pub const DA732X_HP_DET_EN: u32 = (1 << 7);

/* DA732X_REG_HPL_DAC_OFFSET	(addr=0x21/0x26) */
pub const DA732X_HP_DAC_OFFSET_TRIM_MASK: u32 = (0x3F << 0);
pub const DA732X_HP_DAC_OFFSET_DAC_SIGN: u32 = (1 << 6);

/* DA732X_REG_HPL_DAC_OFF_CNTL	(addr=0x22/0x27) */
pub const DA732X_HP_DAC_OFF_CNTL_CONT_MASK: u32 = (7 << 0);
pub const DA732X_HP_DAC_OFF_CNTL_COMPO: u32 = (1 << 3);
pub const DA732X_HP_DAC_OFF_CALIBRATION: u32 = (1 << 0);
pub const DA732X_HP_DAC_OFF_SCALE_STEPS: u32 = (1 << 1);
pub const DA732X_HP_DAC_OFF_MASK: u32 = 0x7F;
pub const DA732X_HP_DAC_COMPO_SHIFT: u32 = 3;

/* DA732X_REG_HPL_OUT_OFFSET	(addr=0x23/0x28) */
pub const DA732X_HP_OUT_OFFSET_MASK: u32 = (0xFF << 0);
pub const DA732X_HP_DAC_OFFSET_TRIM_VAL: u32 = 0x7F;

/* DA732X_REG_HPL/R	(addr=0x24/0x29) */
pub const DA732X_HP_OUT_SIGN: u32 = (1 << 0);
pub const DA732X_HP_OUT_COMP: u32 = (1 << 1);
pub const DA732X_HP_OUT_RESERVED: u32 = (1 << 2);
pub const DA732X_HP_OUT_COMPO: u32 = (1 << 3);
pub const DA732X_HP_OUT_DAC_EN: u32 = (1 << 4);
pub const DA732X_HP_OUT_HIZ_EN: u32 = (1 << 5);
pub const DA732X_HP_OUT_HIZ_DIS: u32 = (0 << 5);
pub const DA732X_HP_OUT_MUTE: u32 = (1 << 6);
pub const DA732X_HP_OUT_EN: u32 = (1 << 7);
pub const DA732X_HP_OUT_COMPO_SHIFT: u32 = 3;
pub const DA732X_HP_OUT_DAC_EN_SHIFT: u32 = 4;
pub const DA732X_HP_HIZ_SHIFT: u32 = 5;
pub const DA732X_HP_MUTE_SHIFT: u32 = 6;
pub const DA732X_HP_OUT_EN_SHIFT: u32 = 7;

pub const DA732X_OUT_HIZ_EN: u32 = (1 << 5);
pub const DA732X_OUT_HIZ_DIS: u32 = (0 << 5);

/* DA732X_REG_HPL/R_VOL	(addr=0x25/0x2A) */
pub const DA732X_HP_VOL_VAL_MASK: u32 = 0xF;
pub const DA732X_HP_VOL_SHIFT: u32 = 0;
pub const DA732X_HP_VOL_VAL_MAX: u32 = DA732X_HP_VOL_VAL_MASK;

/* DA732X_REG_LIN2/3/4	(addr=0x2B/0x2C/0x2D) */
pub const DA732X_LOUT_VOL_SHIFT: u32 = 0;
pub const DA732X_LOUT_VOL_MASK: u32 = 0x0F;
pub const DA732X_LOUT_DAC_OFF: u32 = (0 << 4);
pub const DA732X_LOUT_DAC_EN: u32 = (1 << 4);
pub const DA732X_LOUT_HIZ_N_DIS: u32 = (0 << 5);
pub const DA732X_LOUT_HIZ_N_EN: u32 = (1 << 5);
pub const DA732X_LOUT_UNMUTED: u32 = (0 << 6);
pub const DA732X_LOUT_MUTED: u32 = (1 << 6);
pub const DA732X_LOUT_EN: u32 = (0 << 7);
pub const DA732X_LOUT_DIS: u32 = (1 << 7);
pub const DA732X_LOUT_DAC_EN_SHIFT: u32 = 4;
pub const DA732X_LOUT_MUTE_SHIFT: u32 = 6;
pub const DA732X_LIN_OUT_EN_SHIFT: u32 = 7;
pub const DA732X_LOUT_VOL_VAL_MAX: u32 = DA732X_LOUT_VOL_MASK;

/* DA732X_REG_OUT_ZC_EN		(addr=0x2E) */
pub const DA732X_HPL_ZC_EN_SHIFT: u32 = 0;
pub const DA732X_HPR_ZC_EN_SHIFT: u32 = 1;
pub const DA732X_HPL_ZC_EN: u32 = (1 << 0);
pub const DA732X_HPL_ZC_DIS: u32 = (0 << 0);
pub const DA732X_HPR_ZC_EN: u32 = (1 << 1);
pub const DA732X_HPR_ZC_DIS: u32 = (0 << 1);
pub const DA732X_LIN2_ZC_EN: u32 = (1 << 2);
pub const DA732X_LIN2_ZC_DIS: u32 = (0 << 2);
pub const DA732X_LIN3_ZC_EN: u32 = (1 << 3);
pub const DA732X_LIN3_ZC_DIS: u32 = (0 << 3);
pub const DA732X_LIN4_ZC_EN: u32 = (1 << 4);
pub const DA732X_LIN4_ZC_DIS: u32 = (0 << 4);

/* DA732X_REG_HP_LIN1_GNDSEL (addr=0x37) */
pub const DA732X_HP_OUT_GNDSEL: u32 = (1 << 0);

/* DA732X_REG_CP_HP2 (addr=0x3a) */
pub const DA732X_HP_CP_PULSESKIP: u32 = (1 << 0);
pub const DA732X_HP_CP_REG: u32 = (1 << 1);
pub const DA732X_HP_CP_EN: u32 = (1 << 3);
pub const DA732X_HP_CP_DIS: u32 = (0 << 3);

/* DA732X_REG_CP_CTRL1 (addr=0x40) */
pub const DA732X_CP_MODE_MASK: u32 = (7 << 1);
pub const DA732X_CP_CTRL_STANDBY: u32 = (0 << 1);
pub const DA732X_CP_CTRL_CPVDD6: u32 = (2 << 1);
pub const DA732X_CP_CTRL_CPVDD5: u32 = (3 << 1);
pub const DA732X_CP_CTRL_CPVDD4: u32 = (4 << 1);
pub const DA732X_CP_CTRL_CPVDD3: u32 = (5 << 1);
pub const DA732X_CP_CTRL_CPVDD2: u32 = (6 << 1);
pub const DA732X_CP_CTRL_CPVDD1: u32 = (7 << 1);
pub const DA723X_CP_DIS: u32 = (0 << 7);
pub const DA732X_CP_EN: u32 = (1 << 7);

/* DA732X_REG_CP_CTRL2 (addr=0x41) */
pub const DA732X_CP_BOOST: u32 = (1 << 0);
pub const DA732X_CP_MANAGE_MAGNITUDE: u32 = (2 << 2);

/* DA732X_REG_CP_CTRL3 (addr=0x42) */
pub const DA732X_CP_1MHZ: u32 = (0 << 0);
pub const DA732X_CP_500KHZ: u32 = (1 << 0);
pub const DA732X_CP_250KHZ: u32 = (2 << 0);
pub const DA732X_CP_125KHZ: u32 = (3 << 0);
pub const DA732X_CP_63KHZ: u32 = (4 << 0);
pub const DA732X_CP_0KHZ: u32 = (5 << 0);

/* DA732X_REG_PLL_CTRL (addr=0x53) */
pub const DA732X_PLL_INDIV_MASK: u32 = (3 << 0);
pub const DA732X_PLL_SRM_EN: u32 = (1 << 2);
pub const DA732X_PLL_EN: u32 = (1 << 7);
pub const DA732X_PLL_BYPASS: u32 = (0 << 0);

/* DA732X_REG_CLK_CTRL (addr=0x54) */
pub const DA732X_SR1_MASK: u32 = (0xF);
pub const DA732X_SR2_MASK: u32 = (0xF0);

/* DA732X_REG_CLK_DSP (addr=0x5A) */
pub const DA732X_DSP_FREQ_MASK: u32 = (7 << 0);
pub const DA732X_DSP_FREQ_12MHZ: u32 = (0 << 0);
pub const DA732X_DSP_FREQ_24MHZ: u32 = (1 << 0);
pub const DA732X_DSP_FREQ_36MHZ: u32 = (2 << 0);
pub const DA732X_DSP_FREQ_48MHZ: u32 = (3 << 0);
pub const DA732X_DSP_FREQ_60MHZ: u32 = (4 << 0);
pub const DA732X_DSP_FREQ_72MHZ: u32 = (5 << 0);
pub const DA732X_DSP_FREQ_84MHZ: u32 = (6 << 0);
pub const DA732X_DSP_FREQ_96MHZ: u32 = (7 << 0);

/* DA732X_REG_CLK_EN1 (addr=0x5B) */
pub const DA732X_DSP_CLK_EN: u32 = (1 << 0);
pub const DA732X_SYS3_CLK_EN: u32 = (1 << 1);
pub const DA732X_DSP12_CLK_EN: u32 = (1 << 2);
pub const DA732X_PC_CLK_EN: u32 = (1 << 3);
pub const DA732X_MCLK_SQR_EN: u32 = (1 << 7);

/* DA732X_REG_CLK_EN2 (addr=0x5C) */
pub const DA732X_UART_CLK_EN: u32 = (1 << 1);
pub const DA732X_CP_CLK_EN: u32 = (1 << 2);
pub const DA732X_CP_CLK_DIS: u32 = (0 << 2);

/* DA732X_REG_CLK_EN3 (addr=0x5D) */
pub const DA732X_ADCA_BB_CLK_EN: u32 = (1 << 0);
pub const DA732X_ADCC_BB_CLK_EN: u32 = (1 << 4);

/* DA732X_REG_CLK_EN4 (addr=0x5E) */
pub const DA732X_DACA_BB_CLK_EN: u32 = (1 << 0);
pub const DA732X_DACC_BB_CLK_EN: u32 = (1 << 4);
pub const DA732X_DACA_BB_CLK_SHIFT: u32 = 0;
pub const DA732X_DACC_BB_CLK_SHIFT: u32 = 4;

/* DA732X_REG_CLK_EN5 (addr=0x5F) */
pub const DA732X_DACE_BB_CLK_EN: u32 = (1 << 0);
pub const DA732X_DACE_BB_CLK_SHIFT: u32 = 0;

/* DA732X_REG_AIF_MCLK (addr=0x60) */
pub const DA732X_AIFM_FRAME_64: u32 = (1 << 2);
pub const DA732X_AIFM_SRC_SEL_AIFA: u32 = (1 << 6);
pub const DA732X_CLK_GENERATION_AIF_A: u32 = (1 << 4);
pub const DA732X_NO_CLK_GENERATION: u32 = 0x0;

/* DA732X_REG_AIFA1 (addr=0x61) */
pub const DA732X_AIF_WORD_MASK: u32 = (0x3 << 0);
pub const DA732X_AIF_WORD_16: u32 = (0 << 0);
pub const DA732X_AIF_WORD_20: u32 = (1 << 0);
pub const DA732X_AIF_WORD_24: u32 = (2 << 0);
pub const DA732X_AIF_WORD_32: u32 = (3 << 0);
pub const DA732X_AIF_TDM_MONO_SHIFT: u32 = (1 << 6);
pub const DA732X_AIF1_CLK_MASK: u32 = (1 << 7);
pub const DA732X_AIF_SLAVE: u32 = (0 << 7);
pub const DA732X_AIF_CLK_FROM_SRC: u32 = (1 << 7);

/* DA732X_REG_AIFA3 (addr=0x63) */
pub const DA732X_AIF_MODE_SHIFT: u32 = 0;
pub const DA732X_AIF_MODE_MASK: u32 = 0x3;
pub const DA732X_AIF_I2S_MODE: u32 = (0 << 0);
pub const DA732X_AIF_LEFT_J_MODE: u32 = (1 << 0);
pub const DA732X_AIF_RIGHT_J_MODE: u32 = (2 << 0);
pub const DA732X_AIF_DSP_MODE: u32 = (3 << 0);
pub const DA732X_AIF_WCLK_INV: u32 = (1 << 4);
pub const DA732X_AIF_BCLK_INV: u32 = (1 << 5);
pub const DA732X_AIF_EN: u32 = (1 << 7);
pub const DA732X_AIF_EN_SHIFT: u32 = 7;

/* DA732X_REG_PC_CTRL (addr=0x6a) */
pub const DA732X_PC_PULSE_AIFA: u32 = (0 << 0);
pub const DA732X_PC_PULSE_AIFB: u32 = (1 << 0);
pub const DA732X_PC_RESYNC_AUT: u32 = (1 << 6);
pub const DA732X_PC_RESYNC_NOT_AUT: u32 = (0 << 6);
pub const DA732X_PC_SAME: u32 = (1 << 7);

/* DA732X_REG_DATA_ROUTE (addr=0x70) */
pub const DA732X_ADC1_TO_AIFA: u32 = (0 << 0);
pub const DA732X_DSP_TO_AIFA: u32 = (1 << 0);
pub const DA732X_ADC2_TO_AIFB: u32 = (0 << 1);
pub const DA732X_DSP_TO_AIFB: u32 = (1 << 1);
pub const DA732X_AIFA_TO_DAC1L: u32 = (0 << 2);
pub const DA732X_DSP_TO_DAC1L: u32 = (1 << 2);
pub const DA732X_AIFA_TO_DAC1R: u32 = (0 << 3);
pub const DA732X_DSP_TO_DAC1R: u32 = (1 << 3);
pub const DA732X_AIFB_TO_DAC2L: u32 = (0 << 4);
pub const DA732X_DSP_TO_DAC2L: u32 = (1 << 4);
pub const DA732X_AIFB_TO_DAC2R: u32 = (0 << 5);
pub const DA732X_DSP_TO_DAC2R: u32 = (1 << 5);
pub const DA732X_AIFB_TO_DAC3: u32 = (0 << 6);
pub const DA732X_DSP_TO_DAC3: u32 = (1 << 6);
pub const DA732X_BYPASS_DSP: u32 = (0 << 0);
pub const DA732X_ALL_TO_DSP: u32 = (0x7F << 0);

/* DA732X_REG_DSP_CTRL (addr=0x71) */
pub const DA732X_DIGITAL_EN: u32 = (1 << 0);
pub const DA732X_DIGITAL_RESET: u32 = (0 << 0);
pub const DA732X_DSP_CORE_EN: u32 = (1 << 1);
pub const DA732X_DSP_CORE_RESET: u32 = (0 << 1);

/* DA732X_REG_SPARE1_OUT (addr=0x7D)*/
pub const DA732X_HP_DRIVER_EN: u32 = (1 << 0);
pub const DA732X_HP_GATE_LOW: u32 = (1 << 2);
pub const DA732X_HP_LOOP_GAIN_CTRL: u32 = (1 << 3);

/* DA732X_REG_ID (addr=0x81)*/
pub const DA732X_ID_MINOR_MASK: u32 = (0xF << 0);
pub const DA732X_ID_MAJOR_MASK: u32 = (0xF << 4);

/* DA732X_REG_ADC1/2_PD (addr=0x90/0x98) */
pub const DA732X_ADC_RST_MASK: u32 = (0x3 << 0);
pub const DA732X_ADC_PD_MASK: u32 = (0x3 << 2);
pub const DA732X_ADC_SET_ACT: u32 = (0x3 << 0);
pub const DA732X_ADC_SET_RST: u32 = (0x0 << 0);
pub const DA732X_ADC_ON: u32 = (0x3 << 2);
pub const DA732X_ADC_OFF: u32 = (0x0 << 2);

/* DA732X_REG_ADC1/2_SEL (addr=0x94/0x9C) */
pub const DA732X_ADC_VOL_VAL_MASK: u32 = 0x7;
pub const DA732X_ADCL_VOL_SHIFT: u32 = 0;
pub const DA732X_ADCR_VOL_SHIFT: u32 = 4;
pub const DA732X_ADCL_EN_SHIFT: u32 = 2;
pub const DA732X_ADCR_EN_SHIFT: u32 = 3;
pub const DA732X_ADCL_EN: u32 = (1 << 2);
pub const DA732X_ADCR_EN: u32 = (1 << 3);
pub const DA732X_ADC_VOL_VAL_MAX: u32 = DA732X_ADC_VOL_VAL_MASK;

/*
 * DA732X_REG_ADC1/2_HPF (addr=0x93/0x9b)
 * DA732x_REG_DAC1/2/3_HPG	(addr=0xA5/0xB5/0xC5)
 */
pub const DA732X_HPF_MUSIC_EN: u32 = (1 << 3);
pub const DA732X_HPF_VOICE_EN: u32 = ((1 << 3) | (1 << 7));
pub const DA732X_HPF_MASK: u32 = ((1 << 3) | (1 << 7));
pub const DA732X_HPF_DIS: u32 = ((0 << 3) | (0 << 7));

/* DA732X_REG_DAC1/2/3_VOL */
pub const DA732X_DAC_VOL_VAL_MASK: u32 = 0x7F;
pub const DA732X_DAC_VOL_SHIFT: u32 = 0;
pub const DA732X_DAC_VOL_VAL_MAX: u32 = DA732X_DAC_VOL_VAL_MASK;

/* DA732X_REG_DAC1/2/3_SEL (addr=0xA3/0xB3/0xC3) */
pub const DA732X_DACL_EN_SHIFT: u32 = 3;
pub const DA732X_DACR_EN_SHIFT: u32 = 7;
pub const DA732X_DACL_MUTE_SHIFT: u32 = 2;
pub const DA732X_DACR_MUTE_SHIFT: u32 = 6;
pub const DA732X_DACL_EN: u32 = (1 << 3);
pub const DA732X_DACR_EN: u32 = (1 << 7);
pub const DA732X_DACL_SDM: u32 = (1 << 0);
pub const DA732X_DACR_SDM: u32 = (1 << 4);
pub const DA732X_DACL_MUTE: u32 = (1 << 2);
pub const DA732X_DACR_MUTE: u32 = (1 << 6);

/* DA732X_REG_DAC_SOFTMUTE (addr=0xA4/0xB4/0xC4) */
pub const DA732X_SOFTMUTE_EN: u32 = (1 << 7);
pub const DA732X_GAIN_RAMPED: u32 = (1 << 6);
pub const DA732X_16_SAMPLES: u32 = (4 << 0);
pub const DA732X_SOFTMUTE_MASK: u32 = (1 << 7);
pub const DA732X_SOFTMUTE_SHIFT: u32 = 7;

/*
 * DA732x_REG_ADC1/2_EQ12	(addr=0x95/0x9D)
 * DA732x_REG_ADC1/2_EQ34	(addr=0x96/0x9E)
 * DA732x_REG_ADC1/2_EQ5	(addr=0x97/0x9F)
 * DA732x_REG_DAC1/2/3_EQ12	(addr=0xA5/0xB5/0xC5)
 * DA732x_REG_DAC1/2/3_EQ34	(addr=0xA6/0xB6/0xC6)
 * DA732x_REG_DAC1/2/3_EQ5	(addr=0xA7/0xB7/0xB7)
 */
pub const DA732X_EQ_VOL_VAL_MASK: u32 = 0xF;
pub const DA732X_EQ_BAND1_SHIFT: u32 = 0;
pub const DA732X_EQ_BAND2_SHIFT: u32 = 4;
pub const DA732X_EQ_BAND3_SHIFT: u32 = 0;
pub const DA732X_EQ_BAND4_SHIFT: u32 = 4;
pub const DA732X_EQ_BAND5_SHIFT: u32 = 0;
pub const DA732X_EQ_OVERALL_SHIFT: u32 = 4;
pub const DA732X_EQ_OVERALL_VOL_VAL_MASK: u32 = 0x3;
pub const DA732X_EQ_DIS: u32 = (0 << 7);
pub const DA732X_EQ_EN: u32 = (1 << 7);
pub const DA732X_EQ_EN_SHIFT: u32 = 7;
pub const DA732X_EQ_VOL_VAL_MAX: u32 = DA732X_EQ_VOL_VAL_MASK;
pub const DA732X_EQ_OVERALL_VOL_VAL_MAX: u32 = DA732X_EQ_OVERALL_VOL_VAL_MASK;

/* DA732X_REG_DMA_CMD (addr=0xD3) */
pub const DA732X_SEL_DSP_DMA_MASK: u32 = (3 << 0);
pub const DA732X_SEL_DSP_DMA_DIS: u32 = (0 << 0);
pub const DA732X_SEL_DSP_DMA_PMEM: u32 = (1 << 0);
pub const DA732X_SEL_DSP_DMA_XMEM: u32 = (2 << 0);
pub const DA732X_SEL_DSP_DMA_YMEM: u32 = (3 << 0);
pub const DA732X_DSP_RW_MASK: u32 = (1 << 4);
pub const DA732X_DSP_DMA_WRITE: u32 = (0 << 4);
pub const DA732X_DSP_DMA_READ: u32 = (1 << 4);

/* DA732X_REG_DMA_STATUS (addr=0xDA) */
pub const DA732X_DSP_DMA_FREE: u32 = (0 << 0);
pub const DA732X_DSP_DMA_BUSY: u32 = (1 << 0);


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
