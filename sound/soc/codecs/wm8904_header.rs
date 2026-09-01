/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * wm8904.h  --  WM8904 ASoC driver
 *
 * Copyright 2009 Wolfson Microelectronics, plc
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */


pub const WM8904_CLK_AUTO: u16 = 0;
pub const WM8904_CLK_MCLK: u16 = 1;
pub const WM8904_CLK_FLL: u16 = 2;

pub const WM8904_FLL_MCLK: u16 = 1;
pub const WM8904_FLL_BCLK: u16 = 2;
pub const WM8904_FLL_LRCLK: u16 = 3;
pub const WM8904_FLL_FREE_RUNNING: u16 = 4;

/*
 * Register values.
 */
pub const WM8904_SW_RESET_AND_ID: u16 = 0x00;
pub const WM8904_REVISION: u16 = 0x01;
pub const WM8904_BIAS_CONTROL_0: u16 = 0x04;
pub const WM8904_VMID_CONTROL_0: u16 = 0x05;
pub const WM8904_MIC_BIAS_CONTROL_0: u16 = 0x06;
pub const WM8904_MIC_BIAS_CONTROL_1: u16 = 0x07;
pub const WM8904_ANALOGUE_DAC_0: u16 = 0x08;
pub const WM8904_MIC_FILTER_CONTROL: u16 = 0x09;
pub const WM8904_ANALOGUE_ADC_0: u16 = 0x0A;
pub const WM8904_POWER_MANAGEMENT_0: u16 = 0x0C;
pub const WM8904_POWER_MANAGEMENT_2: u16 = 0x0E;
pub const WM8904_POWER_MANAGEMENT_3: u16 = 0x0F;
pub const WM8904_POWER_MANAGEMENT_6: u16 = 0x12;
pub const WM8904_CLOCK_RATES_0: u16 = 0x14;
pub const WM8904_CLOCK_RATES_1: u16 = 0x15;
pub const WM8904_CLOCK_RATES_2: u16 = 0x16;
pub const WM8904_AUDIO_INTERFACE_0: u16 = 0x18;
pub const WM8904_AUDIO_INTERFACE_1: u16 = 0x19;
pub const WM8904_AUDIO_INTERFACE_2: u16 = 0x1A;
pub const WM8904_AUDIO_INTERFACE_3: u16 = 0x1B;
pub const WM8904_DAC_DIGITAL_VOLUME_LEFT: u16 = 0x1E;
pub const WM8904_DAC_DIGITAL_VOLUME_RIGHT: u16 = 0x1F;
pub const WM8904_DAC_DIGITAL_0: u16 = 0x20;
pub const WM8904_DAC_DIGITAL_1: u16 = 0x21;
pub const WM8904_ADC_DIGITAL_VOLUME_LEFT: u16 = 0x24;
pub const WM8904_ADC_DIGITAL_VOLUME_RIGHT: u16 = 0x25;
pub const WM8904_ADC_DIGITAL_0: u16 = 0x26;
pub const WM8904_DIGITAL_MICROPHONE_0: u16 = 0x27;
pub const WM8904_DRC_0: u16 = 0x28;
pub const WM8904_DRC_1: u16 = 0x29;
pub const WM8904_DRC_2: u16 = 0x2A;
pub const WM8904_DRC_3: u16 = 0x2B;
pub const WM8904_ANALOGUE_LEFT_INPUT_0: u16 = 0x2C;
pub const WM8904_ANALOGUE_RIGHT_INPUT_0: u16 = 0x2D;
pub const WM8904_ANALOGUE_LEFT_INPUT_1: u16 = 0x2E;
pub const WM8904_ANALOGUE_RIGHT_INPUT_1: u16 = 0x2F;
pub const WM8904_ANALOGUE_OUT1_LEFT: u16 = 0x39;
pub const WM8904_ANALOGUE_OUT1_RIGHT: u16 = 0x3A;
pub const WM8904_ANALOGUE_OUT2_LEFT: u16 = 0x3B;
pub const WM8904_ANALOGUE_OUT2_RIGHT: u16 = 0x3C;
pub const WM8904_ANALOGUE_OUT12_ZC: u16 = 0x3D;
pub const WM8904_DC_SERVO_0: u16 = 0x43;
pub const WM8904_DC_SERVO_1: u16 = 0x44;
pub const WM8904_DC_SERVO_2: u16 = 0x45;
pub const WM8904_DC_SERVO_4: u16 = 0x47;
pub const WM8904_DC_SERVO_5: u16 = 0x48;
pub const WM8904_DC_SERVO_6: u16 = 0x49;
pub const WM8904_DC_SERVO_7: u16 = 0x4A;
pub const WM8904_DC_SERVO_8: u16 = 0x4B;
pub const WM8904_DC_SERVO_9: u16 = 0x4C;
pub const WM8904_DC_SERVO_READBACK_0: u16 = 0x4D;
pub const WM8904_ANALOGUE_HP_0: u16 = 0x5A;
pub const WM8904_ANALOGUE_LINEOUT_0: u16 = 0x5E;
pub const WM8904_CHARGE_PUMP_0: u16 = 0x62;
pub const WM8904_CLASS_W_0: u16 = 0x68;
pub const WM8904_WRITE_SEQUENCER_0: u16 = 0x6C;
pub const WM8904_WRITE_SEQUENCER_1: u16 = 0x6D;
pub const WM8904_WRITE_SEQUENCER_2: u16 = 0x6E;
pub const WM8904_WRITE_SEQUENCER_3: u16 = 0x6F;
pub const WM8904_WRITE_SEQUENCER_4: u16 = 0x70;
pub const WM8904_FLL_CONTROL_1: u16 = 0x74;
pub const WM8904_FLL_CONTROL_2: u16 = 0x75;
pub const WM8904_FLL_CONTROL_3: u16 = 0x76;
pub const WM8904_FLL_CONTROL_4: u16 = 0x77;
pub const WM8904_FLL_CONTROL_5: u16 = 0x78;
pub const WM8904_GPIO_CONTROL_1: u16 = 0x79;
pub const WM8904_GPIO_CONTROL_2: u16 = 0x7A;
pub const WM8904_GPIO_CONTROL_3: u16 = 0x7B;
pub const WM8904_GPIO_CONTROL_4: u16 = 0x7C;
pub const WM8904_DIGITAL_PULLS: u16 = 0x7E;
pub const WM8904_INTERRUPT_STATUS: u16 = 0x7F;
pub const WM8904_INTERRUPT_STATUS_MASK: u16 = 0x80;
pub const WM8904_INTERRUPT_POLARITY: u16 = 0x81;
pub const WM8904_INTERRUPT_DEBOUNCE: u16 = 0x82;
pub const WM8904_EQ1: u16 = 0x86;
pub const WM8904_EQ2: u16 = 0x87;
pub const WM8904_EQ3: u16 = 0x88;
pub const WM8904_EQ4: u16 = 0x89;
pub const WM8904_EQ5: u16 = 0x8A;
pub const WM8904_EQ6: u16 = 0x8B;
pub const WM8904_EQ7: u16 = 0x8C;
pub const WM8904_EQ8: u16 = 0x8D;
pub const WM8904_EQ9: u16 = 0x8E;
pub const WM8904_EQ10: u16 = 0x8F;
pub const WM8904_EQ11: u16 = 0x90;
pub const WM8904_EQ12: u16 = 0x91;
pub const WM8904_EQ13: u16 = 0x92;
pub const WM8904_EQ14: u16 = 0x93;
pub const WM8904_EQ15: u16 = 0x94;
pub const WM8904_EQ16: u16 = 0x95;
pub const WM8904_EQ17: u16 = 0x96;
pub const WM8904_EQ18: u16 = 0x97;
pub const WM8904_EQ19: u16 = 0x98;
pub const WM8904_EQ20: u16 = 0x99;
pub const WM8904_EQ21: u16 = 0x9A;
pub const WM8904_EQ22: u16 = 0x9B;
pub const WM8904_EQ23: u16 = 0x9C;
pub const WM8904_EQ24: u16 = 0x9D;
pub const WM8904_CONTROL_INTERFACE_TEST_1: u16 = 0xA1;
pub const WM8904_ADC_TEST_0: u16 = 0xC6;
pub const WM8904_ANALOGUE_OUTPUT_BIAS_0: u16 = 0xCC;
pub const WM8904_FLL_NCO_TEST_0: u16 = 0xF7;
pub const WM8904_FLL_NCO_TEST_1: u16 = 0xF8;

pub const WM8904_REGISTER_COUNT: u16 = 101;
pub const WM8904_MAX_REGISTER: u16 = 0xF8;

/*
 * Field Definitions.
 */

/*
 * R0 (0x00) - SW Reset and ID
 */
pub const WM8904_SW_RST_DEV_ID1_MASK: u16 = 0xFFFF;  /* SW_RST_DEV_ID1 - [15:0] */
pub const WM8904_SW_RST_DEV_ID1_SHIFT: u16 = 0;  /* SW_RST_DEV_ID1 - [15:0] */
pub const WM8904_SW_RST_DEV_ID1_WIDTH: u16 = 16;  /* SW_RST_DEV_ID1 - [15:0] */

/*
 * R1 (0x01) - Revision
 */
pub const WM8904_REVISION_MASK: u16 = 0x000F;  /* REVISION - [3:0] */
pub const WM8904_REVISION_SHIFT: u16 = 0;  /* REVISION - [3:0] */
pub const WM8904_REVISION_WIDTH: u16 = 16;  /* REVISION - [3:0] */

/*
 * R4 (0x04) - Bias Control 0
 */
pub const WM8904_POBCTRL: u16 = 0x0010;  /* POBCTRL */
pub const WM8904_POBCTRL_MASK: u16 = 0x0010;  /* POBCTRL */
pub const WM8904_POBCTRL_SHIFT: u16 = 4;  /* POBCTRL */
pub const WM8904_POBCTRL_WIDTH: u16 = 1;  /* POBCTRL */
pub const WM8904_ISEL_MASK: u16 = 0x000C;  /* ISEL - [3:2] */
pub const WM8904_ISEL_SHIFT: u16 = 2;  /* ISEL - [3:2] */
pub const WM8904_ISEL_WIDTH: u16 = 2;  /* ISEL - [3:2] */
pub const WM8904_STARTUP_BIAS_ENA: u16 = 0x0002;  /* STARTUP_BIAS_ENA */
pub const WM8904_STARTUP_BIAS_ENA_MASK: u16 = 0x0002;  /* STARTUP_BIAS_ENA */
pub const WM8904_STARTUP_BIAS_ENA_SHIFT: u16 = 1;  /* STARTUP_BIAS_ENA */
pub const WM8904_STARTUP_BIAS_ENA_WIDTH: u16 = 1;  /* STARTUP_BIAS_ENA */
pub const WM8904_BIAS_ENA: u16 = 0x0001;  /* BIAS_ENA */
pub const WM8904_BIAS_ENA_MASK: u16 = 0x0001;  /* BIAS_ENA */
pub const WM8904_BIAS_ENA_SHIFT: u16 = 0;  /* BIAS_ENA */
pub const WM8904_BIAS_ENA_WIDTH: u16 = 1;  /* BIAS_ENA */

/*
 * R5 (0x05) - VMID Control 0
 */
pub const WM8904_VMID_BUF_ENA: u16 = 0x0040;  /* VMID_BUF_ENA */
pub const WM8904_VMID_BUF_ENA_MASK: u16 = 0x0040;  /* VMID_BUF_ENA */
pub const WM8904_VMID_BUF_ENA_SHIFT: u16 = 6;  /* VMID_BUF_ENA */
pub const WM8904_VMID_BUF_ENA_WIDTH: u16 = 1;  /* VMID_BUF_ENA */
pub const WM8904_VMID_RES_MASK: u16 = 0x0006;  /* VMID_RES - [2:1] */
pub const WM8904_VMID_RES_SHIFT: u16 = 1;  /* VMID_RES - [2:1] */
pub const WM8904_VMID_RES_WIDTH: u16 = 2;  /* VMID_RES - [2:1] */
pub const WM8904_VMID_ENA: u16 = 0x0001;  /* VMID_ENA */
pub const WM8904_VMID_ENA_MASK: u16 = 0x0001;  /* VMID_ENA */
pub const WM8904_VMID_ENA_SHIFT: u16 = 0;  /* VMID_ENA */
pub const WM8904_VMID_ENA_WIDTH: u16 = 1;  /* VMID_ENA */

/*
 * R8 (0x08) - Analogue DAC 0
 */
pub const WM8904_DAC_BIAS_SEL_MASK: u16 = 0x0018;  /* DAC_BIAS_SEL - [4:3] */
pub const WM8904_DAC_BIAS_SEL_SHIFT: u16 = 3;  /* DAC_BIAS_SEL - [4:3] */
pub const WM8904_DAC_BIAS_SEL_WIDTH: u16 = 2;  /* DAC_BIAS_SEL - [4:3] */
pub const WM8904_DAC_VMID_BIAS_SEL_MASK: u16 = 0x0006;  /* DAC_VMID_BIAS_SEL - [2:1] */
pub const WM8904_DAC_VMID_BIAS_SEL_SHIFT: u16 = 1;  /* DAC_VMID_BIAS_SEL - [2:1] */
pub const WM8904_DAC_VMID_BIAS_SEL_WIDTH: u16 = 2;  /* DAC_VMID_BIAS_SEL - [2:1] */

/*
 * R9 (0x09) - mic Filter Control
 */
pub const WM8904_MIC_DET_SET_THRESHOLD_MASK: u16 = 0xF000;  /* MIC_DET_SET_THRESHOLD - [15:12] */
pub const WM8904_MIC_DET_SET_THRESHOLD_SHIFT: u16 = 12;  /* MIC_DET_SET_THRESHOLD - [15:12] */
pub const WM8904_MIC_DET_SET_THRESHOLD_WIDTH: u16 = 4;  /* MIC_DET_SET_THRESHOLD - [15:12] */
pub const WM8904_MIC_DET_RESET_THRESHOLD_MASK: u16 = 0x0F00;  /* MIC_DET_RESET_THRESHOLD - [11:8] */
pub const WM8904_MIC_DET_RESET_THRESHOLD_SHIFT: u16 = 8;  /* MIC_DET_RESET_THRESHOLD - [11:8] */
pub const WM8904_MIC_DET_RESET_THRESHOLD_WIDTH: u16 = 4;  /* MIC_DET_RESET_THRESHOLD - [11:8] */
pub const WM8904_MIC_SHORT_SET_THRESHOLD_MASK: u16 = 0x00F0;  /* MIC_SHORT_SET_THRESHOLD - [7:4] */
pub const WM8904_MIC_SHORT_SET_THRESHOLD_SHIFT: u16 = 4;  /* MIC_SHORT_SET_THRESHOLD - [7:4] */
pub const WM8904_MIC_SHORT_SET_THRESHOLD_WIDTH: u16 = 4;  /* MIC_SHORT_SET_THRESHOLD - [7:4] */
pub const WM8904_MIC_SHORT_RESET_THRESHOLD_MASK: u16 = 0x000F;  /* MIC_SHORT_RESET_THRESHOLD - [3:0] */
pub const WM8904_MIC_SHORT_RESET_THRESHOLD_SHIFT: u16 = 0;  /* MIC_SHORT_RESET_THRESHOLD - [3:0] */
pub const WM8904_MIC_SHORT_RESET_THRESHOLD_WIDTH: u16 = 4;  /* MIC_SHORT_RESET_THRESHOLD - [3:0] */

/*
 * R10 (0x0A) - Analogue ADC 0
 */
pub const WM8904_ADC_OSR128: u16 = 0x0001;  /* ADC_OSR128 */
pub const WM8904_ADC_OSR128_MASK: u16 = 0x0001;  /* ADC_OSR128 */
pub const WM8904_ADC_OSR128_SHIFT: u16 = 0;  /* ADC_OSR128 */
pub const WM8904_ADC_OSR128_WIDTH: u16 = 1;  /* ADC_OSR128 */

/*
 * R12 (0x0C) - Power Management 0
 */
pub const WM8904_INL_ENA: u16 = 0x0002;  /* INL_ENA */
pub const WM8904_INL_ENA_MASK: u16 = 0x0002;  /* INL_ENA */
pub const WM8904_INL_ENA_SHIFT: u16 = 1;  /* INL_ENA */
pub const WM8904_INL_ENA_WIDTH: u16 = 1;  /* INL_ENA */
pub const WM8904_INR_ENA: u16 = 0x0001;  /* INR_ENA */
pub const WM8904_INR_ENA_MASK: u16 = 0x0001;  /* INR_ENA */
pub const WM8904_INR_ENA_SHIFT: u16 = 0;  /* INR_ENA */
pub const WM8904_INR_ENA_WIDTH: u16 = 1;  /* INR_ENA */

/*
 * R14 (0x0E) - Power Management 2
 */
pub const WM8904_HPL_PGA_ENA: u16 = 0x0002;  /* HPL_PGA_ENA */
pub const WM8904_HPL_PGA_ENA_MASK: u16 = 0x0002;  /* HPL_PGA_ENA */
pub const WM8904_HPL_PGA_ENA_SHIFT: u16 = 1;  /* HPL_PGA_ENA */
pub const WM8904_HPL_PGA_ENA_WIDTH: u16 = 1;  /* HPL_PGA_ENA */
pub const WM8904_HPR_PGA_ENA: u16 = 0x0001;  /* HPR_PGA_ENA */
pub const WM8904_HPR_PGA_ENA_MASK: u16 = 0x0001;  /* HPR_PGA_ENA */
pub const WM8904_HPR_PGA_ENA_SHIFT: u16 = 0;  /* HPR_PGA_ENA */
pub const WM8904_HPR_PGA_ENA_WIDTH: u16 = 1;  /* HPR_PGA_ENA */

/*
 * R15 (0x0F) - Power Management 3
 */
pub const WM8904_LINEOUTL_PGA_ENA: u16 = 0x0002;  /* LINEOUTL_PGA_ENA */
pub const WM8904_LINEOUTL_PGA_ENA_MASK: u16 = 0x0002;  /* LINEOUTL_PGA_ENA */
pub const WM8904_LINEOUTL_PGA_ENA_SHIFT: u16 = 1;  /* LINEOUTL_PGA_ENA */
pub const WM8904_LINEOUTL_PGA_ENA_WIDTH: u16 = 1;  /* LINEOUTL_PGA_ENA */
pub const WM8904_LINEOUTR_PGA_ENA: u16 = 0x0001;  /* LINEOUTR_PGA_ENA */
pub const WM8904_LINEOUTR_PGA_ENA_MASK: u16 = 0x0001;  /* LINEOUTR_PGA_ENA */
pub const WM8904_LINEOUTR_PGA_ENA_SHIFT: u16 = 0;  /* LINEOUTR_PGA_ENA */
pub const WM8904_LINEOUTR_PGA_ENA_WIDTH: u16 = 1;  /* LINEOUTR_PGA_ENA */

/*
 * R18 (0x12) - Power Management 6
 */
pub const WM8904_DACL_ENA: u16 = 0x0008;  /* DACL_ENA */
pub const WM8904_DACL_ENA_MASK: u16 = 0x0008;  /* DACL_ENA */
pub const WM8904_DACL_ENA_SHIFT: u16 = 3;  /* DACL_ENA */
pub const WM8904_DACL_ENA_WIDTH: u16 = 1;  /* DACL_ENA */
pub const WM8904_DACR_ENA: u16 = 0x0004;  /* DACR_ENA */
pub const WM8904_DACR_ENA_MASK: u16 = 0x0004;  /* DACR_ENA */
pub const WM8904_DACR_ENA_SHIFT: u16 = 2;  /* DACR_ENA */
pub const WM8904_DACR_ENA_WIDTH: u16 = 1;  /* DACR_ENA */
pub const WM8904_ADCL_ENA: u16 = 0x0002;  /* ADCL_ENA */
pub const WM8904_ADCL_ENA_MASK: u16 = 0x0002;  /* ADCL_ENA */
pub const WM8904_ADCL_ENA_SHIFT: u16 = 1;  /* ADCL_ENA */
pub const WM8904_ADCL_ENA_WIDTH: u16 = 1;  /* ADCL_ENA */
pub const WM8904_ADCR_ENA: u16 = 0x0001;  /* ADCR_ENA */
pub const WM8904_ADCR_ENA_MASK: u16 = 0x0001;  /* ADCR_ENA */
pub const WM8904_ADCR_ENA_SHIFT: u16 = 0;  /* ADCR_ENA */
pub const WM8904_ADCR_ENA_WIDTH: u16 = 1;  /* ADCR_ENA */

/*
 * R20 (0x14) - Clock Rates 0
 */
pub const WM8904_TOCLK_RATE_DIV16: u16 = 0x4000;  /* TOCLK_RATE_DIV16 */
pub const WM8904_TOCLK_RATE_DIV16_MASK: u16 = 0x4000;  /* TOCLK_RATE_DIV16 */
pub const WM8904_TOCLK_RATE_DIV16_SHIFT: u16 = 14;  /* TOCLK_RATE_DIV16 */
pub const WM8904_TOCLK_RATE_DIV16_WIDTH: u16 = 1;  /* TOCLK_RATE_DIV16 */
pub const WM8904_TOCLK_RATE_X4: u16 = 0x2000;  /* TOCLK_RATE_X4 */
pub const WM8904_TOCLK_RATE_X4_MASK: u16 = 0x2000;  /* TOCLK_RATE_X4 */
pub const WM8904_TOCLK_RATE_X4_SHIFT: u16 = 13;  /* TOCLK_RATE_X4 */
pub const WM8904_TOCLK_RATE_X4_WIDTH: u16 = 1;  /* TOCLK_RATE_X4 */
pub const WM8904_SR_MODE: u16 = 0x1000;  /* SR_MODE */
pub const WM8904_SR_MODE_MASK: u16 = 0x1000;  /* SR_MODE */
pub const WM8904_SR_MODE_SHIFT: u16 = 12;  /* SR_MODE */
pub const WM8904_SR_MODE_WIDTH: u16 = 1;  /* SR_MODE */
pub const WM8904_MCLK_DIV: u16 = 0x0001;  /* MCLK_DIV */
pub const WM8904_MCLK_DIV_MASK: u16 = 0x0001;  /* MCLK_DIV */
pub const WM8904_MCLK_DIV_SHIFT: u16 = 0;  /* MCLK_DIV */
pub const WM8904_MCLK_DIV_WIDTH: u16 = 1;  /* MCLK_DIV */

/*
 * R21 (0x15) - Clock Rates 1
 */
pub const WM8904_CLK_SYS_RATE_MASK: u16 = 0x3C00;  /* CLK_SYS_RATE - [13:10] */
pub const WM8904_CLK_SYS_RATE_SHIFT: u16 = 10;  /* CLK_SYS_RATE - [13:10] */
pub const WM8904_CLK_SYS_RATE_WIDTH: u16 = 4;  /* CLK_SYS_RATE - [13:10] */
pub const WM8904_SAMPLE_RATE_MASK: u16 = 0x0007;  /* SAMPLE_RATE - [2:0] */
pub const WM8904_SAMPLE_RATE_SHIFT: u16 = 0;  /* SAMPLE_RATE - [2:0] */
pub const WM8904_SAMPLE_RATE_WIDTH: u16 = 3;  /* SAMPLE_RATE - [2:0] */

/*
 * R22 (0x16) - Clock Rates 2
 */
pub const WM8904_MCLK_INV: u16 = 0x8000;  /* MCLK_INV */
pub const WM8904_MCLK_INV_MASK: u16 = 0x8000;  /* MCLK_INV */
pub const WM8904_MCLK_INV_SHIFT: u16 = 15;  /* MCLK_INV */
pub const WM8904_MCLK_INV_WIDTH: u16 = 1;  /* MCLK_INV */
pub const WM8904_SYSCLK_SRC: u16 = 0x4000;  /* SYSCLK_SRC */
pub const WM8904_SYSCLK_SRC_MASK: u16 = 0x4000;  /* SYSCLK_SRC */
pub const WM8904_SYSCLK_SRC_SHIFT: u16 = 14;  /* SYSCLK_SRC */
pub const WM8904_SYSCLK_SRC_WIDTH: u16 = 1;  /* SYSCLK_SRC */
pub const WM8904_TOCLK_RATE: u16 = 0x1000;  /* TOCLK_RATE */
pub const WM8904_TOCLK_RATE_MASK: u16 = 0x1000;  /* TOCLK_RATE */
pub const WM8904_TOCLK_RATE_SHIFT: u16 = 12;  /* TOCLK_RATE */
pub const WM8904_TOCLK_RATE_WIDTH: u16 = 1;  /* TOCLK_RATE */
pub const WM8904_OPCLK_ENA: u16 = 0x0008;  /* OPCLK_ENA */
pub const WM8904_OPCLK_ENA_MASK: u16 = 0x0008;  /* OPCLK_ENA */
pub const WM8904_OPCLK_ENA_SHIFT: u16 = 3;  /* OPCLK_ENA */
pub const WM8904_OPCLK_ENA_WIDTH: u16 = 1;  /* OPCLK_ENA */
pub const WM8904_CLK_SYS_ENA: u16 = 0x0004;  /* CLK_SYS_ENA */
pub const WM8904_CLK_SYS_ENA_MASK: u16 = 0x0004;  /* CLK_SYS_ENA */
pub const WM8904_CLK_SYS_ENA_SHIFT: u16 = 2;  /* CLK_SYS_ENA */
pub const WM8904_CLK_SYS_ENA_WIDTH: u16 = 1;  /* CLK_SYS_ENA */
pub const WM8904_CLK_DSP_ENA: u16 = 0x0002;  /* CLK_DSP_ENA */
pub const WM8904_CLK_DSP_ENA_MASK: u16 = 0x0002;  /* CLK_DSP_ENA */
pub const WM8904_CLK_DSP_ENA_SHIFT: u16 = 1;  /* CLK_DSP_ENA */
pub const WM8904_CLK_DSP_ENA_WIDTH: u16 = 1;  /* CLK_DSP_ENA */
pub const WM8904_TOCLK_ENA: u16 = 0x0001;  /* TOCLK_ENA */
pub const WM8904_TOCLK_ENA_MASK: u16 = 0x0001;  /* TOCLK_ENA */
pub const WM8904_TOCLK_ENA_SHIFT: u16 = 0;  /* TOCLK_ENA */
pub const WM8904_TOCLK_ENA_WIDTH: u16 = 1;  /* TOCLK_ENA */

/*
 * R24 (0x18) - Audio Interface 0
 */
pub const WM8904_DACL_DATINV: u16 = 0x1000;  /* DACL_DATINV */
pub const WM8904_DACL_DATINV_MASK: u16 = 0x1000;  /* DACL_DATINV */
pub const WM8904_DACL_DATINV_SHIFT: u16 = 12;  /* DACL_DATINV */
pub const WM8904_DACL_DATINV_WIDTH: u16 = 1;  /* DACL_DATINV */
pub const WM8904_DACR_DATINV: u16 = 0x0800;  /* DACR_DATINV */
pub const WM8904_DACR_DATINV_MASK: u16 = 0x0800;  /* DACR_DATINV */
pub const WM8904_DACR_DATINV_SHIFT: u16 = 11;  /* DACR_DATINV */
pub const WM8904_DACR_DATINV_WIDTH: u16 = 1;  /* DACR_DATINV */
pub const WM8904_DAC_BOOST_MASK: u16 = 0x0600;  /* DAC_BOOST - [10:9] */
pub const WM8904_DAC_BOOST_SHIFT: u16 = 9;  /* DAC_BOOST - [10:9] */
pub const WM8904_DAC_BOOST_WIDTH: u16 = 2;  /* DAC_BOOST - [10:9] */
pub const WM8904_LOOPBACK: u16 = 0x0100;  /* LOOPBACK */
pub const WM8904_LOOPBACK_MASK: u16 = 0x0100;  /* LOOPBACK */
pub const WM8904_LOOPBACK_SHIFT: u16 = 8;  /* LOOPBACK */
pub const WM8904_LOOPBACK_WIDTH: u16 = 1;  /* LOOPBACK */
pub const WM8904_AIFADCL_SRC: u16 = 0x0080;  /* AIFADCL_SRC */
pub const WM8904_AIFADCL_SRC_MASK: u16 = 0x0080;  /* AIFADCL_SRC */
pub const WM8904_AIFADCL_SRC_SHIFT: u16 = 7;  /* AIFADCL_SRC */
pub const WM8904_AIFADCL_SRC_WIDTH: u16 = 1;  /* AIFADCL_SRC */
pub const WM8904_AIFADCR_SRC: u16 = 0x0040;  /* AIFADCR_SRC */
pub const WM8904_AIFADCR_SRC_MASK: u16 = 0x0040;  /* AIFADCR_SRC */
pub const WM8904_AIFADCR_SRC_SHIFT: u16 = 6;  /* AIFADCR_SRC */
pub const WM8904_AIFADCR_SRC_WIDTH: u16 = 1;  /* AIFADCR_SRC */
pub const WM8904_AIFDACL_SRC: u16 = 0x0020;  /* AIFDACL_SRC */
pub const WM8904_AIFDACL_SRC_MASK: u16 = 0x0020;  /* AIFDACL_SRC */
pub const WM8904_AIFDACL_SRC_SHIFT: u16 = 5;  /* AIFDACL_SRC */
pub const WM8904_AIFDACL_SRC_WIDTH: u16 = 1;  /* AIFDACL_SRC */
pub const WM8904_AIFDACR_SRC: u16 = 0x0010;  /* AIFDACR_SRC */
pub const WM8904_AIFDACR_SRC_MASK: u16 = 0x0010;  /* AIFDACR_SRC */
pub const WM8904_AIFDACR_SRC_SHIFT: u16 = 4;  /* AIFDACR_SRC */
pub const WM8904_AIFDACR_SRC_WIDTH: u16 = 1;  /* AIFDACR_SRC */
pub const WM8904_ADC_COMP: u16 = 0x0008;  /* ADC_COMP */
pub const WM8904_ADC_COMP_MASK: u16 = 0x0008;  /* ADC_COMP */
pub const WM8904_ADC_COMP_SHIFT: u16 = 3;  /* ADC_COMP */
pub const WM8904_ADC_COMP_WIDTH: u16 = 1;  /* ADC_COMP */
pub const WM8904_ADC_COMPMODE: u16 = 0x0004;  /* ADC_COMPMODE */
pub const WM8904_ADC_COMPMODE_MASK: u16 = 0x0004;  /* ADC_COMPMODE */
pub const WM8904_ADC_COMPMODE_SHIFT: u16 = 2;  /* ADC_COMPMODE */
pub const WM8904_ADC_COMPMODE_WIDTH: u16 = 1;  /* ADC_COMPMODE */
pub const WM8904_DAC_COMP: u16 = 0x0002;  /* DAC_COMP */
pub const WM8904_DAC_COMP_MASK: u16 = 0x0002;  /* DAC_COMP */
pub const WM8904_DAC_COMP_SHIFT: u16 = 1;  /* DAC_COMP */
pub const WM8904_DAC_COMP_WIDTH: u16 = 1;  /* DAC_COMP */
pub const WM8904_DAC_COMPMODE: u16 = 0x0001;  /* DAC_COMPMODE */
pub const WM8904_DAC_COMPMODE_MASK: u16 = 0x0001;  /* DAC_COMPMODE */
pub const WM8904_DAC_COMPMODE_SHIFT: u16 = 0;  /* DAC_COMPMODE */
pub const WM8904_DAC_COMPMODE_WIDTH: u16 = 1;  /* DAC_COMPMODE */

/*
 * R25 (0x19) - Audio Interface 1
 */
pub const WM8904_AIFDAC_TDM: u16 = 0x2000;  /* AIFDAC_TDM */
pub const WM8904_AIFDAC_TDM_MASK: u16 = 0x2000;  /* AIFDAC_TDM */
pub const WM8904_AIFDAC_TDM_SHIFT: u16 = 13;  /* AIFDAC_TDM */
pub const WM8904_AIFDAC_TDM_WIDTH: u16 = 1;  /* AIFDAC_TDM */
pub const WM8904_AIFDAC_TDM_CHAN: u16 = 0x1000;  /* AIFDAC_TDM_CHAN */
pub const WM8904_AIFDAC_TDM_CHAN_MASK: u16 = 0x1000;  /* AIFDAC_TDM_CHAN */
pub const WM8904_AIFDAC_TDM_CHAN_SHIFT: u16 = 12;  /* AIFDAC_TDM_CHAN */
pub const WM8904_AIFDAC_TDM_CHAN_WIDTH: u16 = 1;  /* AIFDAC_TDM_CHAN */
pub const WM8904_AIFADC_TDM: u16 = 0x0800;  /* AIFADC_TDM */
pub const WM8904_AIFADC_TDM_MASK: u16 = 0x0800;  /* AIFADC_TDM */
pub const WM8904_AIFADC_TDM_SHIFT: u16 = 11;  /* AIFADC_TDM */
pub const WM8904_AIFADC_TDM_WIDTH: u16 = 1;  /* AIFADC_TDM */
pub const WM8904_AIFADC_TDM_CHAN: u16 = 0x0400;  /* AIFADC_TDM_CHAN */
pub const WM8904_AIFADC_TDM_CHAN_MASK: u16 = 0x0400;  /* AIFADC_TDM_CHAN */
pub const WM8904_AIFADC_TDM_CHAN_SHIFT: u16 = 10;  /* AIFADC_TDM_CHAN */
pub const WM8904_AIFADC_TDM_CHAN_WIDTH: u16 = 1;  /* AIFADC_TDM_CHAN */
pub const WM8904_AIF_TRIS: u16 = 0x0100;  /* AIF_TRIS */
pub const WM8904_AIF_TRIS_MASK: u16 = 0x0100;  /* AIF_TRIS */
pub const WM8904_AIF_TRIS_SHIFT: u16 = 8;  /* AIF_TRIS */
pub const WM8904_AIF_TRIS_WIDTH: u16 = 1;  /* AIF_TRIS */
pub const WM8904_AIF_BCLK_INV: u16 = 0x0080;  /* AIF_BCLK_INV */
pub const WM8904_AIF_BCLK_INV_MASK: u16 = 0x0080;  /* AIF_BCLK_INV */
pub const WM8904_AIF_BCLK_INV_SHIFT: u16 = 7;  /* AIF_BCLK_INV */
pub const WM8904_AIF_BCLK_INV_WIDTH: u16 = 1;  /* AIF_BCLK_INV */
pub const WM8904_BCLK_DIR: u16 = 0x0040;  /* BCLK_DIR */
pub const WM8904_BCLK_DIR_MASK: u16 = 0x0040;  /* BCLK_DIR */
pub const WM8904_BCLK_DIR_SHIFT: u16 = 6;  /* BCLK_DIR */
pub const WM8904_BCLK_DIR_WIDTH: u16 = 1;  /* BCLK_DIR */
pub const WM8904_AIF_LRCLK_INV: u16 = 0x0010;  /* AIF_LRCLK_INV */
pub const WM8904_AIF_LRCLK_INV_MASK: u16 = 0x0010;  /* AIF_LRCLK_INV */
pub const WM8904_AIF_LRCLK_INV_SHIFT: u16 = 4;  /* AIF_LRCLK_INV */
pub const WM8904_AIF_LRCLK_INV_WIDTH: u16 = 1;  /* AIF_LRCLK_INV */
pub const WM8904_AIF_WL_MASK: u16 = 0x000C;  /* AIF_WL - [3:2] */
pub const WM8904_AIF_WL_SHIFT: u16 = 2;  /* AIF_WL - [3:2] */
pub const WM8904_AIF_WL_WIDTH: u16 = 2;  /* AIF_WL - [3:2] */
pub const WM8904_AIF_FMT_MASK: u16 = 0x0003;  /* AIF_FMT - [1:0] */
pub const WM8904_AIF_FMT_SHIFT: u16 = 0;  /* AIF_FMT - [1:0] */
pub const WM8904_AIF_FMT_WIDTH: u16 = 2;  /* AIF_FMT - [1:0] */

/*
 * R26 (0x1A) - Audio Interface 2
 */
pub const WM8904_OPCLK_DIV_MASK: u16 = 0x0F00;  /* OPCLK_DIV - [11:8] */
pub const WM8904_OPCLK_DIV_SHIFT: u16 = 8;  /* OPCLK_DIV - [11:8] */
pub const WM8904_OPCLK_DIV_WIDTH: u16 = 4;  /* OPCLK_DIV - [11:8] */
pub const WM8904_BCLK_DIV_MASK: u16 = 0x001F;  /* BCLK_DIV - [4:0] */
pub const WM8904_BCLK_DIV_SHIFT: u16 = 0;  /* BCLK_DIV - [4:0] */
pub const WM8904_BCLK_DIV_WIDTH: u16 = 5;  /* BCLK_DIV - [4:0] */

/*
 * R27 (0x1B) - Audio Interface 3
 */
pub const WM8904_LRCLK_DIR: u16 = 0x0800;  /* LRCLK_DIR */
pub const WM8904_LRCLK_DIR_MASK: u16 = 0x0800;  /* LRCLK_DIR */
pub const WM8904_LRCLK_DIR_SHIFT: u16 = 11;  /* LRCLK_DIR */
pub const WM8904_LRCLK_DIR_WIDTH: u16 = 1;  /* LRCLK_DIR */
pub const WM8904_LRCLK_RATE_MASK: u16 = 0x07FF;  /* LRCLK_RATE - [10:0] */
pub const WM8904_LRCLK_RATE_SHIFT: u16 = 0;  /* LRCLK_RATE - [10:0] */
pub const WM8904_LRCLK_RATE_WIDTH: u16 = 11;  /* LRCLK_RATE - [10:0] */

/*
 * R30 (0x1E) - DAC Digital Volume Left
 */
pub const WM8904_DAC_VU: u16 = 0x0100;  /* DAC_VU */
pub const WM8904_DAC_VU_MASK: u16 = 0x0100;  /* DAC_VU */
pub const WM8904_DAC_VU_SHIFT: u16 = 8;  /* DAC_VU */
pub const WM8904_DAC_VU_WIDTH: u16 = 1;  /* DAC_VU */
pub const WM8904_DACL_VOL_MASK: u16 = 0x00FF;  /* DACL_VOL - [7:0] */
pub const WM8904_DACL_VOL_SHIFT: u16 = 0;  /* DACL_VOL - [7:0] */
pub const WM8904_DACL_VOL_WIDTH: u16 = 8;  /* DACL_VOL - [7:0] */

/*
 * R31 (0x1F) - DAC Digital Volume Right
 */
pub const WM8904_DACR_VOL_MASK: u16 = 0x00FF;  /* DACR_VOL - [7:0] */
pub const WM8904_DACR_VOL_SHIFT: u16 = 0;  /* DACR_VOL - [7:0] */
pub const WM8904_DACR_VOL_WIDTH: u16 = 8;  /* DACR_VOL - [7:0] */

/*
 * R32 (0x20) - DAC Digital 0
 */
pub const WM8904_ADCL_DAC_SVOL_MASK: u16 = 0x0F00;  /* ADCL_DAC_SVOL - [11:8] */
pub const WM8904_ADCL_DAC_SVOL_SHIFT: u16 = 8;  /* ADCL_DAC_SVOL - [11:8] */
pub const WM8904_ADCL_DAC_SVOL_WIDTH: u16 = 4;  /* ADCL_DAC_SVOL - [11:8] */
pub const WM8904_ADCR_DAC_SVOL_MASK: u16 = 0x00F0;  /* ADCR_DAC_SVOL - [7:4] */
pub const WM8904_ADCR_DAC_SVOL_SHIFT: u16 = 4;  /* ADCR_DAC_SVOL - [7:4] */
pub const WM8904_ADCR_DAC_SVOL_WIDTH: u16 = 4;  /* ADCR_DAC_SVOL - [7:4] */
pub const WM8904_ADC_TO_DACL_MASK: u16 = 0x000C;  /* ADC_TO_DACL - [3:2] */
pub const WM8904_ADC_TO_DACL_SHIFT: u16 = 2;  /* ADC_TO_DACL - [3:2] */
pub const WM8904_ADC_TO_DACL_WIDTH: u16 = 2;  /* ADC_TO_DACL - [3:2] */
pub const WM8904_ADC_TO_DACR_MASK: u16 = 0x0003;  /* ADC_TO_DACR - [1:0] */
pub const WM8904_ADC_TO_DACR_SHIFT: u16 = 0;  /* ADC_TO_DACR - [1:0] */
pub const WM8904_ADC_TO_DACR_WIDTH: u16 = 2;  /* ADC_TO_DACR - [1:0] */

/*
 * R33 (0x21) - DAC Digital 1
 */
pub const WM8904_DAC_MONO: u16 = 0x1000;  /* DAC_MONO */
pub const WM8904_DAC_MONO_MASK: u16 = 0x1000;  /* DAC_MONO */
pub const WM8904_DAC_MONO_SHIFT: u16 = 12;  /* DAC_MONO */
pub const WM8904_DAC_MONO_WIDTH: u16 = 1;  /* DAC_MONO */
pub const WM8904_DAC_SB_FILT: u16 = 0x0800;  /* DAC_SB_FILT */
pub const WM8904_DAC_SB_FILT_MASK: u16 = 0x0800;  /* DAC_SB_FILT */
pub const WM8904_DAC_SB_FILT_SHIFT: u16 = 11;  /* DAC_SB_FILT */
pub const WM8904_DAC_SB_FILT_WIDTH: u16 = 1;  /* DAC_SB_FILT */
pub const WM8904_DAC_MUTERATE: u16 = 0x0400;  /* DAC_MUTERATE */
pub const WM8904_DAC_MUTERATE_MASK: u16 = 0x0400;  /* DAC_MUTERATE */
pub const WM8904_DAC_MUTERATE_SHIFT: u16 = 10;  /* DAC_MUTERATE */
pub const WM8904_DAC_MUTERATE_WIDTH: u16 = 1;  /* DAC_MUTERATE */
pub const WM8904_DAC_UNMUTE_RAMP: u16 = 0x0200;  /* DAC_UNMUTE_RAMP */
pub const WM8904_DAC_UNMUTE_RAMP_MASK: u16 = 0x0200;  /* DAC_UNMUTE_RAMP */
pub const WM8904_DAC_UNMUTE_RAMP_SHIFT: u16 = 9;  /* DAC_UNMUTE_RAMP */
pub const WM8904_DAC_UNMUTE_RAMP_WIDTH: u16 = 1;  /* DAC_UNMUTE_RAMP */
pub const WM8904_DAC_OSR128: u16 = 0x0040;  /* DAC_OSR128 */
pub const WM8904_DAC_OSR128_MASK: u16 = 0x0040;  /* DAC_OSR128 */
pub const WM8904_DAC_OSR128_SHIFT: u16 = 6;  /* DAC_OSR128 */
pub const WM8904_DAC_OSR128_WIDTH: u16 = 1;  /* DAC_OSR128 */
pub const WM8904_DAC_MUTE: u16 = 0x0008;  /* DAC_MUTE */
pub const WM8904_DAC_MUTE_MASK: u16 = 0x0008;  /* DAC_MUTE */
pub const WM8904_DAC_MUTE_SHIFT: u16 = 3;  /* DAC_MUTE */
pub const WM8904_DAC_MUTE_WIDTH: u16 = 1;  /* DAC_MUTE */
pub const WM8904_DEEMPH_MASK: u16 = 0x0006;  /* DEEMPH - [2:1] */
pub const WM8904_DEEMPH_SHIFT: u16 = 1;  /* DEEMPH - [2:1] */
pub const WM8904_DEEMPH_WIDTH: u16 = 2;  /* DEEMPH - [2:1] */

/*
 * R36 (0x24) - ADC Digital Volume Left
 */
pub const WM8904_ADC_VU: u16 = 0x0100;  /* ADC_VU */
pub const WM8904_ADC_VU_MASK: u16 = 0x0100;  /* ADC_VU */
pub const WM8904_ADC_VU_SHIFT: u16 = 8;  /* ADC_VU */
pub const WM8904_ADC_VU_WIDTH: u16 = 1;  /* ADC_VU */
pub const WM8904_ADCL_VOL_MASK: u16 = 0x00FF;  /* ADCL_VOL - [7:0] */
pub const WM8904_ADCL_VOL_SHIFT: u16 = 0;  /* ADCL_VOL - [7:0] */
pub const WM8904_ADCL_VOL_WIDTH: u16 = 8;  /* ADCL_VOL - [7:0] */

/*
 * R37 (0x25) - ADC Digital Volume Right
 */
pub const WM8904_ADCR_VOL_MASK: u16 = 0x00FF;  /* ADCR_VOL - [7:0] */
pub const WM8904_ADCR_VOL_SHIFT: u16 = 0;  /* ADCR_VOL - [7:0] */
pub const WM8904_ADCR_VOL_WIDTH: u16 = 8;  /* ADCR_VOL - [7:0] */

/*
 * R38 (0x26) - ADC Digital 0
 */
pub const WM8904_ADC_HPF_CUT_MASK: u16 = 0x0060;  /* ADC_HPF_CUT - [6:5] */
pub const WM8904_ADC_HPF_CUT_SHIFT: u16 = 5;  /* ADC_HPF_CUT - [6:5] */
pub const WM8904_ADC_HPF_CUT_WIDTH: u16 = 2;  /* ADC_HPF_CUT - [6:5] */
pub const WM8904_ADC_HPF: u16 = 0x0010;  /* ADC_HPF */
pub const WM8904_ADC_HPF_MASK: u16 = 0x0010;  /* ADC_HPF */
pub const WM8904_ADC_HPF_SHIFT: u16 = 4;  /* ADC_HPF */
pub const WM8904_ADC_HPF_WIDTH: u16 = 1;  /* ADC_HPF */
pub const WM8904_ADCL_DATINV: u16 = 0x0002;  /* ADCL_DATINV */
pub const WM8904_ADCL_DATINV_MASK: u16 = 0x0002;  /* ADCL_DATINV */
pub const WM8904_ADCL_DATINV_SHIFT: u16 = 1;  /* ADCL_DATINV */
pub const WM8904_ADCL_DATINV_WIDTH: u16 = 1;  /* ADCL_DATINV */
pub const WM8904_ADCR_DATINV: u16 = 0x0001;  /* ADCR_DATINV */
pub const WM8904_ADCR_DATINV_MASK: u16 = 0x0001;  /* ADCR_DATINV */
pub const WM8904_ADCR_DATINV_SHIFT: u16 = 0;  /* ADCR_DATINV */
pub const WM8904_ADCR_DATINV_WIDTH: u16 = 1;  /* ADCR_DATINV */

/*
 * R39 (0x27) - Digital Microphone 0
 */
pub const WM8904_DMIC_ENA: u16 = 0x1000;  /* DMIC_ENA */
pub const WM8904_DMIC_ENA_MASK: u16 = 0x1000;  /* DMIC_ENA */
pub const WM8904_DMIC_ENA_SHIFT: u16 = 12;  /* DMIC_ENA */
pub const WM8904_DMIC_ENA_WIDTH: u16 = 1;  /* DMIC_ENA */
pub const WM8904_DMIC_SRC: u16 = 0x0800;  /* DMIC_SRC */
pub const WM8904_DMIC_SRC_MASK: u16 = 0x0800;  /* DMIC_SRC */
pub const WM8904_DMIC_SRC_SHIFT: u16 = 11;  /* DMIC_SRC */
pub const WM8904_DMIC_SRC_WIDTH: u16 = 1;  /* DMIC_SRC */

/*
 * R40 (0x28) - DRC 0
 */
pub const WM8904_DRC_ENA: u16 = 0x8000;  /* DRC_ENA */
pub const WM8904_DRC_ENA_MASK: u16 = 0x8000;  /* DRC_ENA */
pub const WM8904_DRC_ENA_SHIFT: u16 = 15;  /* DRC_ENA */
pub const WM8904_DRC_ENA_WIDTH: u16 = 1;  /* DRC_ENA */
pub const WM8904_DRC_DAC_PATH: u16 = 0x4000;  /* DRC_DAC_PATH */
pub const WM8904_DRC_DAC_PATH_MASK: u16 = 0x4000;  /* DRC_DAC_PATH */
pub const WM8904_DRC_DAC_PATH_SHIFT: u16 = 14;  /* DRC_DAC_PATH */
pub const WM8904_DRC_DAC_PATH_WIDTH: u16 = 1;  /* DRC_DAC_PATH */
pub const WM8904_DRC_GS_HYST_LVL_MASK: u16 = 0x1800;  /* DRC_GS_HYST_LVL - [12:11] */
pub const WM8904_DRC_GS_HYST_LVL_SHIFT: u16 = 11;  /* DRC_GS_HYST_LVL - [12:11] */
pub const WM8904_DRC_GS_HYST_LVL_WIDTH: u16 = 2;  /* DRC_GS_HYST_LVL - [12:11] */
pub const WM8904_DRC_STARTUP_GAIN_MASK: u16 = 0x07C0;  /* DRC_STARTUP_GAIN - [10:6] */
pub const WM8904_DRC_STARTUP_GAIN_SHIFT: u16 = 6;  /* DRC_STARTUP_GAIN - [10:6] */
pub const WM8904_DRC_STARTUP_GAIN_WIDTH: u16 = 5;  /* DRC_STARTUP_GAIN - [10:6] */
pub const WM8904_DRC_FF_DELAY: u16 = 0x0020;  /* DRC_FF_DELAY */
pub const WM8904_DRC_FF_DELAY_MASK: u16 = 0x0020;  /* DRC_FF_DELAY */
pub const WM8904_DRC_FF_DELAY_SHIFT: u16 = 5;  /* DRC_FF_DELAY */
pub const WM8904_DRC_FF_DELAY_WIDTH: u16 = 1;  /* DRC_FF_DELAY */
pub const WM8904_DRC_GS_ENA: u16 = 0x0008;  /* DRC_GS_ENA */
pub const WM8904_DRC_GS_ENA_MASK: u16 = 0x0008;  /* DRC_GS_ENA */
pub const WM8904_DRC_GS_ENA_SHIFT: u16 = 3;  /* DRC_GS_ENA */
pub const WM8904_DRC_GS_ENA_WIDTH: u16 = 1;  /* DRC_GS_ENA */
pub const WM8904_DRC_QR: u16 = 0x0004;  /* DRC_QR */
pub const WM8904_DRC_QR_MASK: u16 = 0x0004;  /* DRC_QR */
pub const WM8904_DRC_QR_SHIFT: u16 = 2;  /* DRC_QR */
pub const WM8904_DRC_QR_WIDTH: u16 = 1;  /* DRC_QR */
pub const WM8904_DRC_ANTICLIP: u16 = 0x0002;  /* DRC_ANTICLIP */
pub const WM8904_DRC_ANTICLIP_MASK: u16 = 0x0002;  /* DRC_ANTICLIP */
pub const WM8904_DRC_ANTICLIP_SHIFT: u16 = 1;  /* DRC_ANTICLIP */
pub const WM8904_DRC_ANTICLIP_WIDTH: u16 = 1;  /* DRC_ANTICLIP */
pub const WM8904_DRC_GS_HYST: u16 = 0x0001;  /* DRC_GS_HYST */
pub const WM8904_DRC_GS_HYST_MASK: u16 = 0x0001;  /* DRC_GS_HYST */
pub const WM8904_DRC_GS_HYST_SHIFT: u16 = 0;  /* DRC_GS_HYST */
pub const WM8904_DRC_GS_HYST_WIDTH: u16 = 1;  /* DRC_GS_HYST */

/*
 * R41 (0x29) - DRC 1
 */
pub const WM8904_DRC_ATK_MASK: u16 = 0xF000;  /* DRC_ATK - [15:12] */
pub const WM8904_DRC_ATK_SHIFT: u16 = 12;  /* DRC_ATK - [15:12] */
pub const WM8904_DRC_ATK_WIDTH: u16 = 4;  /* DRC_ATK - [15:12] */
pub const WM8904_DRC_DCY_MASK: u16 = 0x0F00;  /* DRC_DCY - [11:8] */
pub const WM8904_DRC_DCY_SHIFT: u16 = 8;  /* DRC_DCY - [11:8] */
pub const WM8904_DRC_DCY_WIDTH: u16 = 4;  /* DRC_DCY - [11:8] */
pub const WM8904_DRC_QR_THR_MASK: u16 = 0x00C0;  /* DRC_QR_THR - [7:6] */
pub const WM8904_DRC_QR_THR_SHIFT: u16 = 6;  /* DRC_QR_THR - [7:6] */
pub const WM8904_DRC_QR_THR_WIDTH: u16 = 2;  /* DRC_QR_THR - [7:6] */
pub const WM8904_DRC_QR_DCY_MASK: u16 = 0x0030;  /* DRC_QR_DCY - [5:4] */
pub const WM8904_DRC_QR_DCY_SHIFT: u16 = 4;  /* DRC_QR_DCY - [5:4] */
pub const WM8904_DRC_QR_DCY_WIDTH: u16 = 2;  /* DRC_QR_DCY - [5:4] */
pub const WM8904_DRC_MINGAIN_MASK: u16 = 0x000C;  /* DRC_MINGAIN - [3:2] */
pub const WM8904_DRC_MINGAIN_SHIFT: u16 = 2;  /* DRC_MINGAIN - [3:2] */
pub const WM8904_DRC_MINGAIN_WIDTH: u16 = 2;  /* DRC_MINGAIN - [3:2] */
pub const WM8904_DRC_MAXGAIN_MASK: u16 = 0x0003;  /* DRC_MAXGAIN - [1:0] */
pub const WM8904_DRC_MAXGAIN_SHIFT: u16 = 0;  /* DRC_MAXGAIN - [1:0] */
pub const WM8904_DRC_MAXGAIN_WIDTH: u16 = 2;  /* DRC_MAXGAIN - [1:0] */

/*
 * R42 (0x2A) - DRC 2
 */
pub const WM8904_DRC_HI_COMP_MASK: u16 = 0x0038;  /* DRC_HI_COMP - [5:3] */
pub const WM8904_DRC_HI_COMP_SHIFT: u16 = 3;  /* DRC_HI_COMP - [5:3] */
pub const WM8904_DRC_HI_COMP_WIDTH: u16 = 3;  /* DRC_HI_COMP - [5:3] */
pub const WM8904_DRC_LO_COMP_MASK: u16 = 0x0007;  /* DRC_LO_COMP - [2:0] */
pub const WM8904_DRC_LO_COMP_SHIFT: u16 = 0;  /* DRC_LO_COMP - [2:0] */
pub const WM8904_DRC_LO_COMP_WIDTH: u16 = 3;  /* DRC_LO_COMP - [2:0] */

/*
 * R43 (0x2B) - DRC 3
 */
pub const WM8904_DRC_KNEE_IP_MASK: u16 = 0x07E0;  /* DRC_KNEE_IP - [10:5] */
pub const WM8904_DRC_KNEE_IP_SHIFT: u16 = 5;  /* DRC_KNEE_IP - [10:5] */
pub const WM8904_DRC_KNEE_IP_WIDTH: u16 = 6;  /* DRC_KNEE_IP - [10:5] */
pub const WM8904_DRC_KNEE_OP_MASK: u16 = 0x001F;  /* DRC_KNEE_OP - [4:0] */
pub const WM8904_DRC_KNEE_OP_SHIFT: u16 = 0;  /* DRC_KNEE_OP - [4:0] */
pub const WM8904_DRC_KNEE_OP_WIDTH: u16 = 5;  /* DRC_KNEE_OP - [4:0] */

/*
 * R44 (0x2C) - Analogue Left Input 0
 */
pub const WM8904_LINMUTE: u16 = 0x0080;  /* LINMUTE */
pub const WM8904_LINMUTE_MASK: u16 = 0x0080;  /* LINMUTE */
pub const WM8904_LINMUTE_SHIFT: u16 = 7;  /* LINMUTE */
pub const WM8904_LINMUTE_WIDTH: u16 = 1;  /* LINMUTE */
pub const WM8904_LIN_VOL_MASK: u16 = 0x001F;  /* LIN_VOL - [4:0] */
pub const WM8904_LIN_VOL_SHIFT: u16 = 0;  /* LIN_VOL - [4:0] */
pub const WM8904_LIN_VOL_WIDTH: u16 = 5;  /* LIN_VOL - [4:0] */

/*
 * R45 (0x2D) - Analogue Right Input 0
 */
pub const WM8904_RINMUTE: u16 = 0x0080;  /* RINMUTE */
pub const WM8904_RINMUTE_MASK: u16 = 0x0080;  /* RINMUTE */
pub const WM8904_RINMUTE_SHIFT: u16 = 7;  /* RINMUTE */
pub const WM8904_RINMUTE_WIDTH: u16 = 1;  /* RINMUTE */
pub const WM8904_RIN_VOL_MASK: u16 = 0x001F;  /* RIN_VOL - [4:0] */
pub const WM8904_RIN_VOL_SHIFT: u16 = 0;  /* RIN_VOL - [4:0] */
pub const WM8904_RIN_VOL_WIDTH: u16 = 5;  /* RIN_VOL - [4:0] */

/*
 * R46 (0x2E) - Analogue Left Input 1
 */
pub const WM8904_INL_CM_ENA: u16 = 0x0040;  /* INL_CM_ENA */
pub const WM8904_INL_CM_ENA_MASK: u16 = 0x0040;  /* INL_CM_ENA */
pub const WM8904_INL_CM_ENA_SHIFT: u16 = 6;  /* INL_CM_ENA */
pub const WM8904_INL_CM_ENA_WIDTH: u16 = 1;  /* INL_CM_ENA */
pub const WM8904_L_IP_SEL_N_MASK: u16 = 0x0030;  /* L_IP_SEL_N - [5:4] */
pub const WM8904_L_IP_SEL_N_SHIFT: u16 = 4;  /* L_IP_SEL_N - [5:4] */
pub const WM8904_L_IP_SEL_N_WIDTH: u16 = 2;  /* L_IP_SEL_N - [5:4] */
pub const WM8904_L_IP_SEL_P_MASK: u16 = 0x000C;  /* L_IP_SEL_P - [3:2] */
pub const WM8904_L_IP_SEL_P_SHIFT: u16 = 2;  /* L_IP_SEL_P - [3:2] */
pub const WM8904_L_IP_SEL_P_WIDTH: u16 = 2;  /* L_IP_SEL_P - [3:2] */
pub const WM8904_L_MODE_MASK: u16 = 0x0003;  /* L_MODE - [1:0] */
pub const WM8904_L_MODE_SHIFT: u16 = 0;  /* L_MODE - [1:0] */
pub const WM8904_L_MODE_WIDTH: u16 = 2;  /* L_MODE - [1:0] */

/*
 * R47 (0x2F) - Analogue Right Input 1
 */
pub const WM8904_INR_CM_ENA: u16 = 0x0040;  /* INR_CM_ENA */
pub const WM8904_INR_CM_ENA_MASK: u16 = 0x0040;  /* INR_CM_ENA */
pub const WM8904_INR_CM_ENA_SHIFT: u16 = 6;  /* INR_CM_ENA */
pub const WM8904_INR_CM_ENA_WIDTH: u16 = 1;  /* INR_CM_ENA */
pub const WM8904_R_IP_SEL_N_MASK: u16 = 0x0030;  /* R_IP_SEL_N - [5:4] */
pub const WM8904_R_IP_SEL_N_SHIFT: u16 = 4;  /* R_IP_SEL_N - [5:4] */
pub const WM8904_R_IP_SEL_N_WIDTH: u16 = 2;  /* R_IP_SEL_N - [5:4] */
pub const WM8904_R_IP_SEL_P_MASK: u16 = 0x000C;  /* R_IP_SEL_P - [3:2] */
pub const WM8904_R_IP_SEL_P_SHIFT: u16 = 2;  /* R_IP_SEL_P - [3:2] */
pub const WM8904_R_IP_SEL_P_WIDTH: u16 = 2;  /* R_IP_SEL_P - [3:2] */
pub const WM8904_R_MODE_MASK: u16 = 0x0003;  /* R_MODE - [1:0] */
pub const WM8904_R_MODE_SHIFT: u16 = 0;  /* R_MODE - [1:0] */
pub const WM8904_R_MODE_WIDTH: u16 = 2;  /* R_MODE - [1:0] */

/*
 * R57 (0x39) - Analogue OUT1 Left
 */
pub const WM8904_HPOUTL_MUTE: u16 = 0x0100;  /* HPOUTL_MUTE */
pub const WM8904_HPOUTL_MUTE_MASK: u16 = 0x0100;  /* HPOUTL_MUTE */
pub const WM8904_HPOUTL_MUTE_SHIFT: u16 = 8;  /* HPOUTL_MUTE */
pub const WM8904_HPOUTL_MUTE_WIDTH: u16 = 1;  /* HPOUTL_MUTE */
pub const WM8904_HPOUT_VU: u16 = 0x0080;  /* HPOUT_VU */
pub const WM8904_HPOUT_VU_MASK: u16 = 0x0080;  /* HPOUT_VU */
pub const WM8904_HPOUT_VU_SHIFT: u16 = 7;  /* HPOUT_VU */
pub const WM8904_HPOUT_VU_WIDTH: u16 = 1;  /* HPOUT_VU */
pub const WM8904_HPOUTLZC: u16 = 0x0040;  /* HPOUTLZC */
pub const WM8904_HPOUTLZC_MASK: u16 = 0x0040;  /* HPOUTLZC */
pub const WM8904_HPOUTLZC_SHIFT: u16 = 6;  /* HPOUTLZC */
pub const WM8904_HPOUTLZC_WIDTH: u16 = 1;  /* HPOUTLZC */
pub const WM8904_HPOUTL_VOL_MASK: u16 = 0x003F;  /* HPOUTL_VOL - [5:0] */
pub const WM8904_HPOUTL_VOL_SHIFT: u16 = 0;  /* HPOUTL_VOL - [5:0] */
pub const WM8904_HPOUTL_VOL_WIDTH: u16 = 6;  /* HPOUTL_VOL - [5:0] */

/*
 * R58 (0x3A) - Analogue OUT1 Right
 */
pub const WM8904_HPOUTR_MUTE: u16 = 0x0100;  /* HPOUTR_MUTE */
pub const WM8904_HPOUTR_MUTE_MASK: u16 = 0x0100;  /* HPOUTR_MUTE */
pub const WM8904_HPOUTR_MUTE_SHIFT: u16 = 8;  /* HPOUTR_MUTE */
pub const WM8904_HPOUTR_MUTE_WIDTH: u16 = 1;  /* HPOUTR_MUTE */
pub const WM8904_HPOUTRZC: u16 = 0x0040;  /* HPOUTRZC */
pub const WM8904_HPOUTRZC_MASK: u16 = 0x0040;  /* HPOUTRZC */
pub const WM8904_HPOUTRZC_SHIFT: u16 = 6;  /* HPOUTRZC */
pub const WM8904_HPOUTRZC_WIDTH: u16 = 1;  /* HPOUTRZC */
pub const WM8904_HPOUTR_VOL_MASK: u16 = 0x003F;  /* HPOUTR_VOL - [5:0] */
pub const WM8904_HPOUTR_VOL_SHIFT: u16 = 0;  /* HPOUTR_VOL - [5:0] */
pub const WM8904_HPOUTR_VOL_WIDTH: u16 = 6;  /* HPOUTR_VOL - [5:0] */

/*
 * R59 (0x3B) - Analogue OUT2 Left
 */
pub const WM8904_LINEOUTL_MUTE: u16 = 0x0100;  /* LINEOUTL_MUTE */
pub const WM8904_LINEOUTL_MUTE_MASK: u16 = 0x0100;  /* LINEOUTL_MUTE */
pub const WM8904_LINEOUTL_MUTE_SHIFT: u16 = 8;  /* LINEOUTL_MUTE */
pub const WM8904_LINEOUTL_MUTE_WIDTH: u16 = 1;  /* LINEOUTL_MUTE */
pub const WM8904_LINEOUT_VU: u16 = 0x0080;  /* LINEOUT_VU */
pub const WM8904_LINEOUT_VU_MASK: u16 = 0x0080;  /* LINEOUT_VU */
pub const WM8904_LINEOUT_VU_SHIFT: u16 = 7;  /* LINEOUT_VU */
pub const WM8904_LINEOUT_VU_WIDTH: u16 = 1;  /* LINEOUT_VU */
pub const WM8904_LINEOUTLZC: u16 = 0x0040;  /* LINEOUTLZC */
pub const WM8904_LINEOUTLZC_MASK: u16 = 0x0040;  /* LINEOUTLZC */
pub const WM8904_LINEOUTLZC_SHIFT: u16 = 6;  /* LINEOUTLZC */
pub const WM8904_LINEOUTLZC_WIDTH: u16 = 1;  /* LINEOUTLZC */
pub const WM8904_LINEOUTL_VOL_MASK: u16 = 0x003F;  /* LINEOUTL_VOL - [5:0] */
pub const WM8904_LINEOUTL_VOL_SHIFT: u16 = 0;  /* LINEOUTL_VOL - [5:0] */
pub const WM8904_LINEOUTL_VOL_WIDTH: u16 = 6;  /* LINEOUTL_VOL - [5:0] */

/*
 * R60 (0x3C) - Analogue OUT2 Right
 */
pub const WM8904_LINEOUTR_MUTE: u16 = 0x0100;  /* LINEOUTR_MUTE */
pub const WM8904_LINEOUTR_MUTE_MASK: u16 = 0x0100;  /* LINEOUTR_MUTE */
pub const WM8904_LINEOUTR_MUTE_SHIFT: u16 = 8;  /* LINEOUTR_MUTE */
pub const WM8904_LINEOUTR_MUTE_WIDTH: u16 = 1;  /* LINEOUTR_MUTE */
pub const WM8904_LINEOUTRZC: u16 = 0x0040;  /* LINEOUTRZC */
pub const WM8904_LINEOUTRZC_MASK: u16 = 0x0040;  /* LINEOUTRZC */
pub const WM8904_LINEOUTRZC_SHIFT: u16 = 6;  /* LINEOUTRZC */
pub const WM8904_LINEOUTRZC_WIDTH: u16 = 1;  /* LINEOUTRZC */
pub const WM8904_LINEOUTR_VOL_MASK: u16 = 0x003F;  /* LINEOUTR_VOL - [5:0] */
pub const WM8904_LINEOUTR_VOL_SHIFT: u16 = 0;  /* LINEOUTR_VOL - [5:0] */
pub const WM8904_LINEOUTR_VOL_WIDTH: u16 = 6;  /* LINEOUTR_VOL - [5:0] */

/*
 * R61 (0x3D) - Analogue OUT12 ZC
 */
pub const WM8904_HPL_BYP_ENA: u16 = 0x0008;  /* HPL_BYP_ENA */
pub const WM8904_HPL_BYP_ENA_MASK: u16 = 0x0008;  /* HPL_BYP_ENA */
pub const WM8904_HPL_BYP_ENA_SHIFT: u16 = 3;  /* HPL_BYP_ENA */
pub const WM8904_HPL_BYP_ENA_WIDTH: u16 = 1;  /* HPL_BYP_ENA */
pub const WM8904_HPR_BYP_ENA: u16 = 0x0004;  /* HPR_BYP_ENA */
pub const WM8904_HPR_BYP_ENA_MASK: u16 = 0x0004;  /* HPR_BYP_ENA */
pub const WM8904_HPR_BYP_ENA_SHIFT: u16 = 2;  /* HPR_BYP_ENA */
pub const WM8904_HPR_BYP_ENA_WIDTH: u16 = 1;  /* HPR_BYP_ENA */
pub const WM8904_LINEOUTL_BYP_ENA: u16 = 0x0002;  /* LINEOUTL_BYP_ENA */
pub const WM8904_LINEOUTL_BYP_ENA_MASK: u16 = 0x0002;  /* LINEOUTL_BYP_ENA */
pub const WM8904_LINEOUTL_BYP_ENA_SHIFT: u16 = 1;  /* LINEOUTL_BYP_ENA */
pub const WM8904_LINEOUTL_BYP_ENA_WIDTH: u16 = 1;  /* LINEOUTL_BYP_ENA */
pub const WM8904_LINEOUTR_BYP_ENA: u16 = 0x0001;  /* LINEOUTR_BYP_ENA */
pub const WM8904_LINEOUTR_BYP_ENA_MASK: u16 = 0x0001;  /* LINEOUTR_BYP_ENA */
pub const WM8904_LINEOUTR_BYP_ENA_SHIFT: u16 = 0;  /* LINEOUTR_BYP_ENA */
pub const WM8904_LINEOUTR_BYP_ENA_WIDTH: u16 = 1;  /* LINEOUTR_BYP_ENA */

/*
 * R67 (0x43) - DC Servo 0
 */
pub const WM8904_DCS_ENA_CHAN_3: u16 = 0x0008;  /* DCS_ENA_CHAN_3 */
pub const WM8904_DCS_ENA_CHAN_3_MASK: u16 = 0x0008;  /* DCS_ENA_CHAN_3 */
pub const WM8904_DCS_ENA_CHAN_3_SHIFT: u16 = 3;  /* DCS_ENA_CHAN_3 */
pub const WM8904_DCS_ENA_CHAN_3_WIDTH: u16 = 1;  /* DCS_ENA_CHAN_3 */
pub const WM8904_DCS_ENA_CHAN_2: u16 = 0x0004;  /* DCS_ENA_CHAN_2 */
pub const WM8904_DCS_ENA_CHAN_2_MASK: u16 = 0x0004;  /* DCS_ENA_CHAN_2 */
pub const WM8904_DCS_ENA_CHAN_2_SHIFT: u16 = 2;  /* DCS_ENA_CHAN_2 */
pub const WM8904_DCS_ENA_CHAN_2_WIDTH: u16 = 1;  /* DCS_ENA_CHAN_2 */
pub const WM8904_DCS_ENA_CHAN_1: u16 = 0x0002;  /* DCS_ENA_CHAN_1 */
pub const WM8904_DCS_ENA_CHAN_1_MASK: u16 = 0x0002;  /* DCS_ENA_CHAN_1 */
pub const WM8904_DCS_ENA_CHAN_1_SHIFT: u16 = 1;  /* DCS_ENA_CHAN_1 */
pub const WM8904_DCS_ENA_CHAN_1_WIDTH: u16 = 1;  /* DCS_ENA_CHAN_1 */
pub const WM8904_DCS_ENA_CHAN_0: u16 = 0x0001;  /* DCS_ENA_CHAN_0 */
pub const WM8904_DCS_ENA_CHAN_0_MASK: u16 = 0x0001;  /* DCS_ENA_CHAN_0 */
pub const WM8904_DCS_ENA_CHAN_0_SHIFT: u16 = 0;  /* DCS_ENA_CHAN_0 */
pub const WM8904_DCS_ENA_CHAN_0_WIDTH: u16 = 1;  /* DCS_ENA_CHAN_0 */

/*
 * R68 (0x44) - DC Servo 1
 */
pub const WM8904_DCS_TRIG_SINGLE_3: u16 = 0x8000;  /* DCS_TRIG_SINGLE_3 */
pub const WM8904_DCS_TRIG_SINGLE_3_MASK: u16 = 0x8000;  /* DCS_TRIG_SINGLE_3 */
pub const WM8904_DCS_TRIG_SINGLE_3_SHIFT: u16 = 15;  /* DCS_TRIG_SINGLE_3 */
pub const WM8904_DCS_TRIG_SINGLE_3_WIDTH: u16 = 1;  /* DCS_TRIG_SINGLE_3 */
pub const WM8904_DCS_TRIG_SINGLE_2: u16 = 0x4000;  /* DCS_TRIG_SINGLE_2 */
pub const WM8904_DCS_TRIG_SINGLE_2_MASK: u16 = 0x4000;  /* DCS_TRIG_SINGLE_2 */
pub const WM8904_DCS_TRIG_SINGLE_2_SHIFT: u16 = 14;  /* DCS_TRIG_SINGLE_2 */
pub const WM8904_DCS_TRIG_SINGLE_2_WIDTH: u16 = 1;  /* DCS_TRIG_SINGLE_2 */
pub const WM8904_DCS_TRIG_SINGLE_1: u16 = 0x2000;  /* DCS_TRIG_SINGLE_1 */
pub const WM8904_DCS_TRIG_SINGLE_1_MASK: u16 = 0x2000;  /* DCS_TRIG_SINGLE_1 */
pub const WM8904_DCS_TRIG_SINGLE_1_SHIFT: u16 = 13;  /* DCS_TRIG_SINGLE_1 */
pub const WM8904_DCS_TRIG_SINGLE_1_WIDTH: u16 = 1;  /* DCS_TRIG_SINGLE_1 */
pub const WM8904_DCS_TRIG_SINGLE_0: u16 = 0x1000;  /* DCS_TRIG_SINGLE_0 */
pub const WM8904_DCS_TRIG_SINGLE_0_MASK: u16 = 0x1000;  /* DCS_TRIG_SINGLE_0 */
pub const WM8904_DCS_TRIG_SINGLE_0_SHIFT: u16 = 12;  /* DCS_TRIG_SINGLE_0 */
pub const WM8904_DCS_TRIG_SINGLE_0_WIDTH: u16 = 1;  /* DCS_TRIG_SINGLE_0 */
pub const WM8904_DCS_TRIG_SERIES_3: u16 = 0x0800;  /* DCS_TRIG_SERIES_3 */
pub const WM8904_DCS_TRIG_SERIES_3_MASK: u16 = 0x0800;  /* DCS_TRIG_SERIES_3 */
pub const WM8904_DCS_TRIG_SERIES_3_SHIFT: u16 = 11;  /* DCS_TRIG_SERIES_3 */
pub const WM8904_DCS_TRIG_SERIES_3_WIDTH: u16 = 1;  /* DCS_TRIG_SERIES_3 */
pub const WM8904_DCS_TRIG_SERIES_2: u16 = 0x0400;  /* DCS_TRIG_SERIES_2 */
pub const WM8904_DCS_TRIG_SERIES_2_MASK: u16 = 0x0400;  /* DCS_TRIG_SERIES_2 */
pub const WM8904_DCS_TRIG_SERIES_2_SHIFT: u16 = 10;  /* DCS_TRIG_SERIES_2 */
pub const WM8904_DCS_TRIG_SERIES_2_WIDTH: u16 = 1;  /* DCS_TRIG_SERIES_2 */
pub const WM8904_DCS_TRIG_SERIES_1: u16 = 0x0200;  /* DCS_TRIG_SERIES_1 */
pub const WM8904_DCS_TRIG_SERIES_1_MASK: u16 = 0x0200;  /* DCS_TRIG_SERIES_1 */
pub const WM8904_DCS_TRIG_SERIES_1_SHIFT: u16 = 9;  /* DCS_TRIG_SERIES_1 */
pub const WM8904_DCS_TRIG_SERIES_1_WIDTH: u16 = 1;  /* DCS_TRIG_SERIES_1 */
pub const WM8904_DCS_TRIG_SERIES_0: u16 = 0x0100;  /* DCS_TRIG_SERIES_0 */
pub const WM8904_DCS_TRIG_SERIES_0_MASK: u16 = 0x0100;  /* DCS_TRIG_SERIES_0 */
pub const WM8904_DCS_TRIG_SERIES_0_SHIFT: u16 = 8;  /* DCS_TRIG_SERIES_0 */
pub const WM8904_DCS_TRIG_SERIES_0_WIDTH: u16 = 1;  /* DCS_TRIG_SERIES_0 */
pub const WM8904_DCS_TRIG_STARTUP_3: u16 = 0x0080;  /* DCS_TRIG_STARTUP_3 */
pub const WM8904_DCS_TRIG_STARTUP_3_MASK: u16 = 0x0080;  /* DCS_TRIG_STARTUP_3 */
pub const WM8904_DCS_TRIG_STARTUP_3_SHIFT: u16 = 7;  /* DCS_TRIG_STARTUP_3 */
pub const WM8904_DCS_TRIG_STARTUP_3_WIDTH: u16 = 1;  /* DCS_TRIG_STARTUP_3 */
pub const WM8904_DCS_TRIG_STARTUP_2: u16 = 0x0040;  /* DCS_TRIG_STARTUP_2 */
pub const WM8904_DCS_TRIG_STARTUP_2_MASK: u16 = 0x0040;  /* DCS_TRIG_STARTUP_2 */
pub const WM8904_DCS_TRIG_STARTUP_2_SHIFT: u16 = 6;  /* DCS_TRIG_STARTUP_2 */
pub const WM8904_DCS_TRIG_STARTUP_2_WIDTH: u16 = 1;  /* DCS_TRIG_STARTUP_2 */
pub const WM8904_DCS_TRIG_STARTUP_1: u16 = 0x0020;  /* DCS_TRIG_STARTUP_1 */
pub const WM8904_DCS_TRIG_STARTUP_1_MASK: u16 = 0x0020;  /* DCS_TRIG_STARTUP_1 */
pub const WM8904_DCS_TRIG_STARTUP_1_SHIFT: u16 = 5;  /* DCS_TRIG_STARTUP_1 */
pub const WM8904_DCS_TRIG_STARTUP_1_WIDTH: u16 = 1;  /* DCS_TRIG_STARTUP_1 */
pub const WM8904_DCS_TRIG_STARTUP_0: u16 = 0x0010;  /* DCS_TRIG_STARTUP_0 */
pub const WM8904_DCS_TRIG_STARTUP_0_MASK: u16 = 0x0010;  /* DCS_TRIG_STARTUP_0 */
pub const WM8904_DCS_TRIG_STARTUP_0_SHIFT: u16 = 4;  /* DCS_TRIG_STARTUP_0 */
pub const WM8904_DCS_TRIG_STARTUP_0_WIDTH: u16 = 1;  /* DCS_TRIG_STARTUP_0 */
pub const WM8904_DCS_TRIG_DAC_WR_3: u16 = 0x0008;  /* DCS_TRIG_DAC_WR_3 */
pub const WM8904_DCS_TRIG_DAC_WR_3_MASK: u16 = 0x0008;  /* DCS_TRIG_DAC_WR_3 */
pub const WM8904_DCS_TRIG_DAC_WR_3_SHIFT: u16 = 3;  /* DCS_TRIG_DAC_WR_3 */
pub const WM8904_DCS_TRIG_DAC_WR_3_WIDTH: u16 = 1;  /* DCS_TRIG_DAC_WR_3 */
pub const WM8904_DCS_TRIG_DAC_WR_2: u16 = 0x0004;  /* DCS_TRIG_DAC_WR_2 */
pub const WM8904_DCS_TRIG_DAC_WR_2_MASK: u16 = 0x0004;  /* DCS_TRIG_DAC_WR_2 */
pub const WM8904_DCS_TRIG_DAC_WR_2_SHIFT: u16 = 2;  /* DCS_TRIG_DAC_WR_2 */
pub const WM8904_DCS_TRIG_DAC_WR_2_WIDTH: u16 = 1;  /* DCS_TRIG_DAC_WR_2 */
pub const WM8904_DCS_TRIG_DAC_WR_1: u16 = 0x0002;  /* DCS_TRIG_DAC_WR_1 */
pub const WM8904_DCS_TRIG_DAC_WR_1_MASK: u16 = 0x0002;  /* DCS_TRIG_DAC_WR_1 */
pub const WM8904_DCS_TRIG_DAC_WR_1_SHIFT: u16 = 1;  /* DCS_TRIG_DAC_WR_1 */
pub const WM8904_DCS_TRIG_DAC_WR_1_WIDTH: u16 = 1;  /* DCS_TRIG_DAC_WR_1 */
pub const WM8904_DCS_TRIG_DAC_WR_0: u16 = 0x0001;  /* DCS_TRIG_DAC_WR_0 */
pub const WM8904_DCS_TRIG_DAC_WR_0_MASK: u16 = 0x0001;  /* DCS_TRIG_DAC_WR_0 */
pub const WM8904_DCS_TRIG_DAC_WR_0_SHIFT: u16 = 0;  /* DCS_TRIG_DAC_WR_0 */
pub const WM8904_DCS_TRIG_DAC_WR_0_WIDTH: u16 = 1;  /* DCS_TRIG_DAC_WR_0 */

/*
 * R69 (0x45) - DC Servo 2
 */
pub const WM8904_DCS_TIMER_PERIOD_23_MASK: u16 = 0x0F00;  /* DCS_TIMER_PERIOD_23 - [11:8] */
pub const WM8904_DCS_TIMER_PERIOD_23_SHIFT: u16 = 8;  /* DCS_TIMER_PERIOD_23 - [11:8] */
pub const WM8904_DCS_TIMER_PERIOD_23_WIDTH: u16 = 4;  /* DCS_TIMER_PERIOD_23 - [11:8] */
pub const WM8904_DCS_TIMER_PERIOD_01_MASK: u16 = 0x000F;  /* DCS_TIMER_PERIOD_01 - [3:0] */
pub const WM8904_DCS_TIMER_PERIOD_01_SHIFT: u16 = 0;  /* DCS_TIMER_PERIOD_01 - [3:0] */
pub const WM8904_DCS_TIMER_PERIOD_01_WIDTH: u16 = 4;  /* DCS_TIMER_PERIOD_01 - [3:0] */

/*
 * R71 (0x47) - DC Servo 4
 */
pub const WM8904_DCS_SERIES_NO_23_MASK: u16 = 0x007F;  /* DCS_SERIES_NO_23 - [6:0] */
pub const WM8904_DCS_SERIES_NO_23_SHIFT: u16 = 0;  /* DCS_SERIES_NO_23 - [6:0] */
pub const WM8904_DCS_SERIES_NO_23_WIDTH: u16 = 7;  /* DCS_SERIES_NO_23 - [6:0] */

/*
 * R72 (0x48) - DC Servo 5
 */
pub const WM8904_DCS_SERIES_NO_01_MASK: u16 = 0x007F;  /* DCS_SERIES_NO_01 - [6:0] */
pub const WM8904_DCS_SERIES_NO_01_SHIFT: u16 = 0;  /* DCS_SERIES_NO_01 - [6:0] */
pub const WM8904_DCS_SERIES_NO_01_WIDTH: u16 = 7;  /* DCS_SERIES_NO_01 - [6:0] */

/*
 * R73 (0x49) - DC Servo 6
 */
pub const WM8904_DCS_DAC_WR_VAL_3_MASK: u16 = 0x00FF;  /* DCS_DAC_WR_VAL_3 - [7:0] */
pub const WM8904_DCS_DAC_WR_VAL_3_SHIFT: u16 = 0;  /* DCS_DAC_WR_VAL_3 - [7:0] */
pub const WM8904_DCS_DAC_WR_VAL_3_WIDTH: u16 = 8;  /* DCS_DAC_WR_VAL_3 - [7:0] */

/*
 * R74 (0x4A) - DC Servo 7
 */
pub const WM8904_DCS_DAC_WR_VAL_2_MASK: u16 = 0x00FF;  /* DCS_DAC_WR_VAL_2 - [7:0] */
pub const WM8904_DCS_DAC_WR_VAL_2_SHIFT: u16 = 0;  /* DCS_DAC_WR_VAL_2 - [7:0] */
pub const WM8904_DCS_DAC_WR_VAL_2_WIDTH: u16 = 8;  /* DCS_DAC_WR_VAL_2 - [7:0] */

/*
 * R75 (0x4B) - DC Servo 8
 */
pub const WM8904_DCS_DAC_WR_VAL_1_MASK: u16 = 0x00FF;  /* DCS_DAC_WR_VAL_1 - [7:0] */
pub const WM8904_DCS_DAC_WR_VAL_1_SHIFT: u16 = 0;  /* DCS_DAC_WR_VAL_1 - [7:0] */
pub const WM8904_DCS_DAC_WR_VAL_1_WIDTH: u16 = 8;  /* DCS_DAC_WR_VAL_1 - [7:0] */

/*
 * R76 (0x4C) - DC Servo 9
 */
pub const WM8904_DCS_DAC_WR_VAL_0_MASK: u16 = 0x00FF;  /* DCS_DAC_WR_VAL_0 - [7:0] */
pub const WM8904_DCS_DAC_WR_VAL_0_SHIFT: u16 = 0;  /* DCS_DAC_WR_VAL_0 - [7:0] */
pub const WM8904_DCS_DAC_WR_VAL_0_WIDTH: u16 = 8;  /* DCS_DAC_WR_VAL_0 - [7:0] */

/*
 * R77 (0x4D) - DC Servo Readback 0
 */
pub const WM8904_DCS_CAL_COMPLETE_MASK: u16 = 0x0F00;  /* DCS_CAL_COMPLETE - [11:8] */
pub const WM8904_DCS_CAL_COMPLETE_SHIFT: u16 = 8;  /* DCS_CAL_COMPLETE - [11:8] */
pub const WM8904_DCS_CAL_COMPLETE_WIDTH: u16 = 4;  /* DCS_CAL_COMPLETE - [11:8] */
pub const WM8904_DCS_DAC_WR_COMPLETE_MASK: u16 = 0x00F0;  /* DCS_DAC_WR_COMPLETE - [7:4] */
pub const WM8904_DCS_DAC_WR_COMPLETE_SHIFT: u16 = 4;  /* DCS_DAC_WR_COMPLETE - [7:4] */
pub const WM8904_DCS_DAC_WR_COMPLETE_WIDTH: u16 = 4;  /* DCS_DAC_WR_COMPLETE - [7:4] */
pub const WM8904_DCS_STARTUP_COMPLETE_MASK: u16 = 0x000F;  /* DCS_STARTUP_COMPLETE - [3:0] */
pub const WM8904_DCS_STARTUP_COMPLETE_SHIFT: u16 = 0;  /* DCS_STARTUP_COMPLETE - [3:0] */
pub const WM8904_DCS_STARTUP_COMPLETE_WIDTH: u16 = 4;  /* DCS_STARTUP_COMPLETE - [3:0] */

/*
 * R90 (0x5A) - Analogue HP 0
 */
pub const WM8904_HPL_RMV_SHORT: u16 = 0x0080;  /* HPL_RMV_SHORT */
pub const WM8904_HPL_RMV_SHORT_MASK: u16 = 0x0080;  /* HPL_RMV_SHORT */
pub const WM8904_HPL_RMV_SHORT_SHIFT: u16 = 7;  /* HPL_RMV_SHORT */
pub const WM8904_HPL_RMV_SHORT_WIDTH: u16 = 1;  /* HPL_RMV_SHORT */
pub const WM8904_HPL_ENA_OUTP: u16 = 0x0040;  /* HPL_ENA_OUTP */
pub const WM8904_HPL_ENA_OUTP_MASK: u16 = 0x0040;  /* HPL_ENA_OUTP */
pub const WM8904_HPL_ENA_OUTP_SHIFT: u16 = 6;  /* HPL_ENA_OUTP */
pub const WM8904_HPL_ENA_OUTP_WIDTH: u16 = 1;  /* HPL_ENA_OUTP */
pub const WM8904_HPL_ENA_DLY: u16 = 0x0020;  /* HPL_ENA_DLY */
pub const WM8904_HPL_ENA_DLY_MASK: u16 = 0x0020;  /* HPL_ENA_DLY */
pub const WM8904_HPL_ENA_DLY_SHIFT: u16 = 5;  /* HPL_ENA_DLY */
pub const WM8904_HPL_ENA_DLY_WIDTH: u16 = 1;  /* HPL_ENA_DLY */
pub const WM8904_HPL_ENA: u16 = 0x0010;  /* HPL_ENA */
pub const WM8904_HPL_ENA_MASK: u16 = 0x0010;  /* HPL_ENA */
pub const WM8904_HPL_ENA_SHIFT: u16 = 4;  /* HPL_ENA */
pub const WM8904_HPL_ENA_WIDTH: u16 = 1;  /* HPL_ENA */
pub const WM8904_HPR_RMV_SHORT: u16 = 0x0008;  /* HPR_RMV_SHORT */
pub const WM8904_HPR_RMV_SHORT_MASK: u16 = 0x0008;  /* HPR_RMV_SHORT */
pub const WM8904_HPR_RMV_SHORT_SHIFT: u16 = 3;  /* HPR_RMV_SHORT */
pub const WM8904_HPR_RMV_SHORT_WIDTH: u16 = 1;  /* HPR_RMV_SHORT */
pub const WM8904_HPR_ENA_OUTP: u16 = 0x0004;  /* HPR_ENA_OUTP */
pub const WM8904_HPR_ENA_OUTP_MASK: u16 = 0x0004;  /* HPR_ENA_OUTP */
pub const WM8904_HPR_ENA_OUTP_SHIFT: u16 = 2;  /* HPR_ENA_OUTP */
pub const WM8904_HPR_ENA_OUTP_WIDTH: u16 = 1;  /* HPR_ENA_OUTP */
pub const WM8904_HPR_ENA_DLY: u16 = 0x0002;  /* HPR_ENA_DLY */
pub const WM8904_HPR_ENA_DLY_MASK: u16 = 0x0002;  /* HPR_ENA_DLY */
pub const WM8904_HPR_ENA_DLY_SHIFT: u16 = 1;  /* HPR_ENA_DLY */
pub const WM8904_HPR_ENA_DLY_WIDTH: u16 = 1;  /* HPR_ENA_DLY */
pub const WM8904_HPR_ENA: u16 = 0x0001;  /* HPR_ENA */
pub const WM8904_HPR_ENA_MASK: u16 = 0x0001;  /* HPR_ENA */
pub const WM8904_HPR_ENA_SHIFT: u16 = 0;  /* HPR_ENA */
pub const WM8904_HPR_ENA_WIDTH: u16 = 1;  /* HPR_ENA */

/*
 * R94 (0x5E) - Analogue Lineout 0
 */
pub const WM8904_LINEOUTL_RMV_SHORT: u16 = 0x0080;  /* LINEOUTL_RMV_SHORT */
pub const WM8904_LINEOUTL_RMV_SHORT_MASK: u16 = 0x0080;  /* LINEOUTL_RMV_SHORT */
pub const WM8904_LINEOUTL_RMV_SHORT_SHIFT: u16 = 7;  /* LINEOUTL_RMV_SHORT */
pub const WM8904_LINEOUTL_RMV_SHORT_WIDTH: u16 = 1;  /* LINEOUTL_RMV_SHORT */
pub const WM8904_LINEOUTL_ENA_OUTP: u16 = 0x0040;  /* LINEOUTL_ENA_OUTP */
pub const WM8904_LINEOUTL_ENA_OUTP_MASK: u16 = 0x0040;  /* LINEOUTL_ENA_OUTP */
pub const WM8904_LINEOUTL_ENA_OUTP_SHIFT: u16 = 6;  /* LINEOUTL_ENA_OUTP */
pub const WM8904_LINEOUTL_ENA_OUTP_WIDTH: u16 = 1;  /* LINEOUTL_ENA_OUTP */
pub const WM8904_LINEOUTL_ENA_DLY: u16 = 0x0020;  /* LINEOUTL_ENA_DLY */
pub const WM8904_LINEOUTL_ENA_DLY_MASK: u16 = 0x0020;  /* LINEOUTL_ENA_DLY */
pub const WM8904_LINEOUTL_ENA_DLY_SHIFT: u16 = 5;  /* LINEOUTL_ENA_DLY */
pub const WM8904_LINEOUTL_ENA_DLY_WIDTH: u16 = 1;  /* LINEOUTL_ENA_DLY */
pub const WM8904_LINEOUTL_ENA: u16 = 0x0010;  /* LINEOUTL_ENA */
pub const WM8904_LINEOUTL_ENA_MASK: u16 = 0x0010;  /* LINEOUTL_ENA */
pub const WM8904_LINEOUTL_ENA_SHIFT: u16 = 4;  /* LINEOUTL_ENA */
pub const WM8904_LINEOUTL_ENA_WIDTH: u16 = 1;  /* LINEOUTL_ENA */
pub const WM8904_LINEOUTR_RMV_SHORT: u16 = 0x0008;  /* LINEOUTR_RMV_SHORT */
pub const WM8904_LINEOUTR_RMV_SHORT_MASK: u16 = 0x0008;  /* LINEOUTR_RMV_SHORT */
pub const WM8904_LINEOUTR_RMV_SHORT_SHIFT: u16 = 3;  /* LINEOUTR_RMV_SHORT */
pub const WM8904_LINEOUTR_RMV_SHORT_WIDTH: u16 = 1;  /* LINEOUTR_RMV_SHORT */
pub const WM8904_LINEOUTR_ENA_OUTP: u16 = 0x0004;  /* LINEOUTR_ENA_OUTP */
pub const WM8904_LINEOUTR_ENA_OUTP_MASK: u16 = 0x0004;  /* LINEOUTR_ENA_OUTP */
pub const WM8904_LINEOUTR_ENA_OUTP_SHIFT: u16 = 2;  /* LINEOUTR_ENA_OUTP */
pub const WM8904_LINEOUTR_ENA_OUTP_WIDTH: u16 = 1;  /* LINEOUTR_ENA_OUTP */
pub const WM8904_LINEOUTR_ENA_DLY: u16 = 0x0002;  /* LINEOUTR_ENA_DLY */
pub const WM8904_LINEOUTR_ENA_DLY_MASK: u16 = 0x0002;  /* LINEOUTR_ENA_DLY */
pub const WM8904_LINEOUTR_ENA_DLY_SHIFT: u16 = 1;  /* LINEOUTR_ENA_DLY */
pub const WM8904_LINEOUTR_ENA_DLY_WIDTH: u16 = 1;  /* LINEOUTR_ENA_DLY */
pub const WM8904_LINEOUTR_ENA: u16 = 0x0001;  /* LINEOUTR_ENA */
pub const WM8904_LINEOUTR_ENA_MASK: u16 = 0x0001;  /* LINEOUTR_ENA */
pub const WM8904_LINEOUTR_ENA_SHIFT: u16 = 0;  /* LINEOUTR_ENA */
pub const WM8904_LINEOUTR_ENA_WIDTH: u16 = 1;  /* LINEOUTR_ENA */

/*
 * R98 (0x62) - Charge Pump 0
 */
pub const WM8904_CP_ENA: u16 = 0x0001;  /* CP_ENA */
pub const WM8904_CP_ENA_MASK: u16 = 0x0001;  /* CP_ENA */
pub const WM8904_CP_ENA_SHIFT: u16 = 0;  /* CP_ENA */
pub const WM8904_CP_ENA_WIDTH: u16 = 1;  /* CP_ENA */

/*
 * R104 (0x68) - Class W 0
 */
pub const WM8904_CP_DYN_PWR: u16 = 0x0001;  /* CP_DYN_PWR */
pub const WM8904_CP_DYN_PWR_MASK: u16 = 0x0001;  /* CP_DYN_PWR */
pub const WM8904_CP_DYN_PWR_SHIFT: u16 = 0;  /* CP_DYN_PWR */
pub const WM8904_CP_DYN_PWR_WIDTH: u16 = 1;  /* CP_DYN_PWR */

/*
 * R108 (0x6C) - Write Sequencer 0
 */
pub const WM8904_WSEQ_ENA: u16 = 0x0100;  /* WSEQ_ENA */
pub const WM8904_WSEQ_ENA_MASK: u16 = 0x0100;  /* WSEQ_ENA */
pub const WM8904_WSEQ_ENA_SHIFT: u16 = 8;  /* WSEQ_ENA */
pub const WM8904_WSEQ_ENA_WIDTH: u16 = 1;  /* WSEQ_ENA */
pub const WM8904_WSEQ_WRITE_INDEX_MASK: u16 = 0x001F;  /* WSEQ_WRITE_INDEX - [4:0] */
pub const WM8904_WSEQ_WRITE_INDEX_SHIFT: u16 = 0;  /* WSEQ_WRITE_INDEX - [4:0] */
pub const WM8904_WSEQ_WRITE_INDEX_WIDTH: u16 = 5;  /* WSEQ_WRITE_INDEX - [4:0] */

/*
 * R109 (0x6D) - Write Sequencer 1
 */
pub const WM8904_WSEQ_DATA_WIDTH_MASK: u16 = 0x7000;  /* WSEQ_DATA_WIDTH - [14:12] */
pub const WM8904_WSEQ_DATA_WIDTH_SHIFT: u16 = 12;  /* WSEQ_DATA_WIDTH - [14:12] */
pub const WM8904_WSEQ_DATA_WIDTH_WIDTH: u16 = 3;  /* WSEQ_DATA_WIDTH - [14:12] */
pub const WM8904_WSEQ_DATA_START_MASK: u16 = 0x0F00;  /* WSEQ_DATA_START - [11:8] */
pub const WM8904_WSEQ_DATA_START_SHIFT: u16 = 8;  /* WSEQ_DATA_START - [11:8] */
pub const WM8904_WSEQ_DATA_START_WIDTH: u16 = 4;  /* WSEQ_DATA_START - [11:8] */
pub const WM8904_WSEQ_ADDR_MASK: u16 = 0x00FF;  /* WSEQ_ADDR - [7:0] */
pub const WM8904_WSEQ_ADDR_SHIFT: u16 = 0;  /* WSEQ_ADDR - [7:0] */
pub const WM8904_WSEQ_ADDR_WIDTH: u16 = 8;  /* WSEQ_ADDR - [7:0] */

/*
 * R110 (0x6E) - Write Sequencer 2
 */
pub const WM8904_WSEQ_EOS: u16 = 0x4000;  /* WSEQ_EOS */
pub const WM8904_WSEQ_EOS_MASK: u16 = 0x4000;  /* WSEQ_EOS */
pub const WM8904_WSEQ_EOS_SHIFT: u16 = 14;  /* WSEQ_EOS */
pub const WM8904_WSEQ_EOS_WIDTH: u16 = 1;  /* WSEQ_EOS */
pub const WM8904_WSEQ_DELAY_MASK: u16 = 0x0F00;  /* WSEQ_DELAY - [11:8] */
pub const WM8904_WSEQ_DELAY_SHIFT: u16 = 8;  /* WSEQ_DELAY - [11:8] */
pub const WM8904_WSEQ_DELAY_WIDTH: u16 = 4;  /* WSEQ_DELAY - [11:8] */
pub const WM8904_WSEQ_DATA_MASK: u16 = 0x00FF;  /* WSEQ_DATA - [7:0] */
pub const WM8904_WSEQ_DATA_SHIFT: u16 = 0;  /* WSEQ_DATA - [7:0] */
pub const WM8904_WSEQ_DATA_WIDTH: u16 = 8;  /* WSEQ_DATA - [7:0] */

/*
 * R111 (0x6F) - Write Sequencer 3
 */
pub const WM8904_WSEQ_ABORT: u16 = 0x0200;  /* WSEQ_ABORT */
pub const WM8904_WSEQ_ABORT_MASK: u16 = 0x0200;  /* WSEQ_ABORT */
pub const WM8904_WSEQ_ABORT_SHIFT: u16 = 9;  /* WSEQ_ABORT */
pub const WM8904_WSEQ_ABORT_WIDTH: u16 = 1;  /* WSEQ_ABORT */
pub const WM8904_WSEQ_START: u16 = 0x0100;  /* WSEQ_START */
pub const WM8904_WSEQ_START_MASK: u16 = 0x0100;  /* WSEQ_START */
pub const WM8904_WSEQ_START_SHIFT: u16 = 8;  /* WSEQ_START */
pub const WM8904_WSEQ_START_WIDTH: u16 = 1;  /* WSEQ_START */
pub const WM8904_WSEQ_START_INDEX_MASK: u16 = 0x003F;  /* WSEQ_START_INDEX - [5:0] */
pub const WM8904_WSEQ_START_INDEX_SHIFT: u16 = 0;  /* WSEQ_START_INDEX - [5:0] */
pub const WM8904_WSEQ_START_INDEX_WIDTH: u16 = 6;  /* WSEQ_START_INDEX - [5:0] */

/*
 * R112 (0x70) - Write Sequencer 4
 */
pub const WM8904_WSEQ_CURRENT_INDEX_MASK: u16 = 0x03F0;  /* WSEQ_CURRENT_INDEX - [9:4] */
pub const WM8904_WSEQ_CURRENT_INDEX_SHIFT: u16 = 4;  /* WSEQ_CURRENT_INDEX - [9:4] */
pub const WM8904_WSEQ_CURRENT_INDEX_WIDTH: u16 = 6;  /* WSEQ_CURRENT_INDEX - [9:4] */
pub const WM8904_WSEQ_BUSY: u16 = 0x0001;  /* WSEQ_BUSY */
pub const WM8904_WSEQ_BUSY_MASK: u16 = 0x0001;  /* WSEQ_BUSY */
pub const WM8904_WSEQ_BUSY_SHIFT: u16 = 0;  /* WSEQ_BUSY */
pub const WM8904_WSEQ_BUSY_WIDTH: u16 = 1;  /* WSEQ_BUSY */

/*
 * R116 (0x74) - FLL Control 1
 */
pub const WM8904_FLL_FRACN_ENA: u16 = 0x0004;  /* FLL_FRACN_ENA */
pub const WM8904_FLL_FRACN_ENA_MASK: u16 = 0x0004;  /* FLL_FRACN_ENA */
pub const WM8904_FLL_FRACN_ENA_SHIFT: u16 = 2;  /* FLL_FRACN_ENA */
pub const WM8904_FLL_FRACN_ENA_WIDTH: u16 = 1;  /* FLL_FRACN_ENA */
pub const WM8904_FLL_OSC_ENA: u16 = 0x0002;  /* FLL_OSC_ENA */
pub const WM8904_FLL_OSC_ENA_MASK: u16 = 0x0002;  /* FLL_OSC_ENA */
pub const WM8904_FLL_OSC_ENA_SHIFT: u16 = 1;  /* FLL_OSC_ENA */
pub const WM8904_FLL_OSC_ENA_WIDTH: u16 = 1;  /* FLL_OSC_ENA */
pub const WM8904_FLL_ENA: u16 = 0x0001;  /* FLL_ENA */
pub const WM8904_FLL_ENA_MASK: u16 = 0x0001;  /* FLL_ENA */
pub const WM8904_FLL_ENA_SHIFT: u16 = 0;  /* FLL_ENA */
pub const WM8904_FLL_ENA_WIDTH: u16 = 1;  /* FLL_ENA */

/*
 * R117 (0x75) - FLL Control 2
 */
pub const WM8904_FLL_OUTDIV_MASK: u16 = 0x3F00;  /* FLL_OUTDIV - [13:8] */
pub const WM8904_FLL_OUTDIV_SHIFT: u16 = 8;  /* FLL_OUTDIV - [13:8] */
pub const WM8904_FLL_OUTDIV_WIDTH: u16 = 6;  /* FLL_OUTDIV - [13:8] */
pub const WM8904_FLL_CTRL_RATE_MASK: u16 = 0x0070;  /* FLL_CTRL_RATE - [6:4] */
pub const WM8904_FLL_CTRL_RATE_SHIFT: u16 = 4;  /* FLL_CTRL_RATE - [6:4] */
pub const WM8904_FLL_CTRL_RATE_WIDTH: u16 = 3;  /* FLL_CTRL_RATE - [6:4] */
pub const WM8904_FLL_FRATIO_MASK: u16 = 0x0007;  /* FLL_FRATIO - [2:0] */
pub const WM8904_FLL_FRATIO_SHIFT: u16 = 0;  /* FLL_FRATIO - [2:0] */
pub const WM8904_FLL_FRATIO_WIDTH: u16 = 3;  /* FLL_FRATIO - [2:0] */

/*
 * R118 (0x76) - FLL Control 3
 */
pub const WM8904_FLL_K_MASK: u16 = 0xFFFF;  /* FLL_K - [15:0] */
pub const WM8904_FLL_K_SHIFT: u16 = 0;  /* FLL_K - [15:0] */
pub const WM8904_FLL_K_WIDTH: u16 = 16;  /* FLL_K - [15:0] */

/*
 * R119 (0x77) - FLL Control 4
 */
pub const WM8904_FLL_N_MASK: u16 = 0x7FE0;  /* FLL_N - [14:5] */
pub const WM8904_FLL_N_SHIFT: u16 = 5;  /* FLL_N - [14:5] */
pub const WM8904_FLL_N_WIDTH: u16 = 10;  /* FLL_N - [14:5] */
pub const WM8904_FLL_GAIN_MASK: u16 = 0x000F;  /* FLL_GAIN - [3:0] */
pub const WM8904_FLL_GAIN_SHIFT: u16 = 0;  /* FLL_GAIN - [3:0] */
pub const WM8904_FLL_GAIN_WIDTH: u16 = 4;  /* FLL_GAIN - [3:0] */

/*
 * R120 (0x78) - FLL Control 5
 */
pub const WM8904_FLL_CLK_REF_DIV_MASK: u16 = 0x0018;  /* FLL_CLK_REF_DIV - [4:3] */
pub const WM8904_FLL_CLK_REF_DIV_SHIFT: u16 = 3;  /* FLL_CLK_REF_DIV - [4:3] */
pub const WM8904_FLL_CLK_REF_DIV_WIDTH: u16 = 2;  /* FLL_CLK_REF_DIV - [4:3] */
pub const WM8904_FLL_CLK_REF_SRC_MASK: u16 = 0x0003;  /* FLL_CLK_REF_SRC - [1:0] */
pub const WM8904_FLL_CLK_REF_SRC_SHIFT: u16 = 0;  /* FLL_CLK_REF_SRC - [1:0] */
pub const WM8904_FLL_CLK_REF_SRC_WIDTH: u16 = 2;  /* FLL_CLK_REF_SRC - [1:0] */

/*
 * R126 (0x7E) - Digital Pulls
 */
pub const WM8904_MCLK_PU: u16 = 0x0080;  /* MCLK_PU */
pub const WM8904_MCLK_PU_MASK: u16 = 0x0080;  /* MCLK_PU */
pub const WM8904_MCLK_PU_SHIFT: u16 = 7;  /* MCLK_PU */
pub const WM8904_MCLK_PU_WIDTH: u16 = 1;  /* MCLK_PU */
pub const WM8904_MCLK_PD: u16 = 0x0040;  /* MCLK_PD */
pub const WM8904_MCLK_PD_MASK: u16 = 0x0040;  /* MCLK_PD */
pub const WM8904_MCLK_PD_SHIFT: u16 = 6;  /* MCLK_PD */
pub const WM8904_MCLK_PD_WIDTH: u16 = 1;  /* MCLK_PD */
pub const WM8904_DACDAT_PU: u16 = 0x0020;  /* DACDAT_PU */
pub const WM8904_DACDAT_PU_MASK: u16 = 0x0020;  /* DACDAT_PU */
pub const WM8904_DACDAT_PU_SHIFT: u16 = 5;  /* DACDAT_PU */
pub const WM8904_DACDAT_PU_WIDTH: u16 = 1;  /* DACDAT_PU */
pub const WM8904_DACDAT_PD: u16 = 0x0010;  /* DACDAT_PD */
pub const WM8904_DACDAT_PD_MASK: u16 = 0x0010;  /* DACDAT_PD */
pub const WM8904_DACDAT_PD_SHIFT: u16 = 4;  /* DACDAT_PD */
pub const WM8904_DACDAT_PD_WIDTH: u16 = 1;  /* DACDAT_PD */
pub const WM8904_LRCLK_PU: u16 = 0x0008;  /* LRCLK_PU */
pub const WM8904_LRCLK_PU_MASK: u16 = 0x0008;  /* LRCLK_PU */
pub const WM8904_LRCLK_PU_SHIFT: u16 = 3;  /* LRCLK_PU */
pub const WM8904_LRCLK_PU_WIDTH: u16 = 1;  /* LRCLK_PU */
pub const WM8904_LRCLK_PD: u16 = 0x0004;  /* LRCLK_PD */
pub const WM8904_LRCLK_PD_MASK: u16 = 0x0004;  /* LRCLK_PD */
pub const WM8904_LRCLK_PD_SHIFT: u16 = 2;  /* LRCLK_PD */
pub const WM8904_LRCLK_PD_WIDTH: u16 = 1;  /* LRCLK_PD */
pub const WM8904_BCLK_PU: u16 = 0x0002;  /* BCLK_PU */
pub const WM8904_BCLK_PU_MASK: u16 = 0x0002;  /* BCLK_PU */
pub const WM8904_BCLK_PU_SHIFT: u16 = 1;  /* BCLK_PU */
pub const WM8904_BCLK_PU_WIDTH: u16 = 1;  /* BCLK_PU */
pub const WM8904_BCLK_PD: u16 = 0x0001;  /* BCLK_PD */
pub const WM8904_BCLK_PD_MASK: u16 = 0x0001;  /* BCLK_PD */
pub const WM8904_BCLK_PD_SHIFT: u16 = 0;  /* BCLK_PD */
pub const WM8904_BCLK_PD_WIDTH: u16 = 1;  /* BCLK_PD */

/*
 * R127 (0x7F) - Interrupt Status
 */
pub const WM8904_IRQ: u16 = 0x0400;  /* IRQ */
pub const WM8904_IRQ_MASK: u16 = 0x0400;  /* IRQ */
pub const WM8904_IRQ_SHIFT: u16 = 10;  /* IRQ */
pub const WM8904_IRQ_WIDTH: u16 = 1;  /* IRQ */
pub const WM8904_GPIO_BCLK_EINT: u16 = 0x0200;  /* GPIO_BCLK_EINT */
pub const WM8904_GPIO_BCLK_EINT_MASK: u16 = 0x0200;  /* GPIO_BCLK_EINT */
pub const WM8904_GPIO_BCLK_EINT_SHIFT: u16 = 9;  /* GPIO_BCLK_EINT */
pub const WM8904_GPIO_BCLK_EINT_WIDTH: u16 = 1;  /* GPIO_BCLK_EINT */
pub const WM8904_WSEQ_EINT: u16 = 0x0100;  /* WSEQ_EINT */
pub const WM8904_WSEQ_EINT_MASK: u16 = 0x0100;  /* WSEQ_EINT */
pub const WM8904_WSEQ_EINT_SHIFT: u16 = 8;  /* WSEQ_EINT */
pub const WM8904_WSEQ_EINT_WIDTH: u16 = 1;  /* WSEQ_EINT */
pub const WM8904_GPIO3_EINT: u16 = 0x0080;  /* GPIO3_EINT */
pub const WM8904_GPIO3_EINT_MASK: u16 = 0x0080;  /* GPIO3_EINT */
pub const WM8904_GPIO3_EINT_SHIFT: u16 = 7;  /* GPIO3_EINT */
pub const WM8904_GPIO3_EINT_WIDTH: u16 = 1;  /* GPIO3_EINT */
pub const WM8904_GPIO2_EINT: u16 = 0x0040;  /* GPIO2_EINT */
pub const WM8904_GPIO2_EINT_MASK: u16 = 0x0040;  /* GPIO2_EINT */
pub const WM8904_GPIO2_EINT_SHIFT: u16 = 6;  /* GPIO2_EINT */
pub const WM8904_GPIO2_EINT_WIDTH: u16 = 1;  /* GPIO2_EINT */
pub const WM8904_GPIO1_EINT: u16 = 0x0020;  /* GPIO1_EINT */
pub const WM8904_GPIO1_EINT_MASK: u16 = 0x0020;  /* GPIO1_EINT */
pub const WM8904_GPIO1_EINT_SHIFT: u16 = 5;  /* GPIO1_EINT */
pub const WM8904_GPIO1_EINT_WIDTH: u16 = 1;  /* GPIO1_EINT */
pub const WM8904_GPI8_EINT: u16 = 0x0010;  /* GPI8_EINT */
pub const WM8904_GPI8_EINT_MASK: u16 = 0x0010;  /* GPI8_EINT */
pub const WM8904_GPI8_EINT_SHIFT: u16 = 4;  /* GPI8_EINT */
pub const WM8904_GPI8_EINT_WIDTH: u16 = 1;  /* GPI8_EINT */
pub const WM8904_GPI7_EINT: u16 = 0x0008;  /* GPI7_EINT */
pub const WM8904_GPI7_EINT_MASK: u16 = 0x0008;  /* GPI7_EINT */
pub const WM8904_GPI7_EINT_SHIFT: u16 = 3;  /* GPI7_EINT */
pub const WM8904_GPI7_EINT_WIDTH: u16 = 1;  /* GPI7_EINT */
pub const WM8904_FLL_LOCK_EINT: u16 = 0x0004;  /* FLL_LOCK_EINT */
pub const WM8904_FLL_LOCK_EINT_MASK: u16 = 0x0004;  /* FLL_LOCK_EINT */
pub const WM8904_FLL_LOCK_EINT_SHIFT: u16 = 2;  /* FLL_LOCK_EINT */
pub const WM8904_FLL_LOCK_EINT_WIDTH: u16 = 1;  /* FLL_LOCK_EINT */
pub const WM8904_MIC_SHRT_EINT: u16 = 0x0002;  /* MIC_SHRT_EINT */
pub const WM8904_MIC_SHRT_EINT_MASK: u16 = 0x0002;  /* MIC_SHRT_EINT */
pub const WM8904_MIC_SHRT_EINT_SHIFT: u16 = 1;  /* MIC_SHRT_EINT */
pub const WM8904_MIC_SHRT_EINT_WIDTH: u16 = 1;  /* MIC_SHRT_EINT */
pub const WM8904_MIC_DET_EINT: u16 = 0x0001;  /* MIC_DET_EINT */
pub const WM8904_MIC_DET_EINT_MASK: u16 = 0x0001;  /* MIC_DET_EINT */
pub const WM8904_MIC_DET_EINT_SHIFT: u16 = 0;  /* MIC_DET_EINT */
pub const WM8904_MIC_DET_EINT_WIDTH: u16 = 1;  /* MIC_DET_EINT */

/*
 * R128 (0x80) - Interrupt Status Mask
 */
pub const WM8904_IM_GPIO_BCLK_EINT: u16 = 0x0200;  /* IM_GPIO_BCLK_EINT */
pub const WM8904_IM_GPIO_BCLK_EINT_MASK: u16 = 0x0200;  /* IM_GPIO_BCLK_EINT */
pub const WM8904_IM_GPIO_BCLK_EINT_SHIFT: u16 = 9;  /* IM_GPIO_BCLK_EINT */
pub const WM8904_IM_GPIO_BCLK_EINT_WIDTH: u16 = 1;  /* IM_GPIO_BCLK_EINT */
pub const WM8904_IM_WSEQ_EINT: u16 = 0x0100;  /* IM_WSEQ_EINT */
pub const WM8904_IM_WSEQ_EINT_MASK: u16 = 0x0100;  /* IM_WSEQ_EINT */
pub const WM8904_IM_WSEQ_EINT_SHIFT: u16 = 8;  /* IM_WSEQ_EINT */
pub const WM8904_IM_WSEQ_EINT_WIDTH: u16 = 1;  /* IM_WSEQ_EINT */
pub const WM8904_IM_GPIO3_EINT: u16 = 0x0080;  /* IM_GPIO3_EINT */
pub const WM8904_IM_GPIO3_EINT_MASK: u16 = 0x0080;  /* IM_GPIO3_EINT */
pub const WM8904_IM_GPIO3_EINT_SHIFT: u16 = 7;  /* IM_GPIO3_EINT */
pub const WM8904_IM_GPIO3_EINT_WIDTH: u16 = 1;  /* IM_GPIO3_EINT */
pub const WM8904_IM_GPIO2_EINT: u16 = 0x0040;  /* IM_GPIO2_EINT */
pub const WM8904_IM_GPIO2_EINT_MASK: u16 = 0x0040;  /* IM_GPIO2_EINT */
pub const WM8904_IM_GPIO2_EINT_SHIFT: u16 = 6;  /* IM_GPIO2_EINT */
pub const WM8904_IM_GPIO2_EINT_WIDTH: u16 = 1;  /* IM_GPIO2_EINT */
pub const WM8904_IM_GPIO1_EINT: u16 = 0x0020;  /* IM_GPIO1_EINT */
pub const WM8904_IM_GPIO1_EINT_MASK: u16 = 0x0020;  /* IM_GPIO1_EINT */
pub const WM8904_IM_GPIO1_EINT_SHIFT: u16 = 5;  /* IM_GPIO1_EINT */
pub const WM8904_IM_GPIO1_EINT_WIDTH: u16 = 1;  /* IM_GPIO1_EINT */
pub const WM8904_IM_GPI8_EINT: u16 = 0x0010;  /* IM_GPI8_EINT */
pub const WM8904_IM_GPI8_EINT_MASK: u16 = 0x0010;  /* IM_GPI8_EINT */
pub const WM8904_IM_GPI8_EINT_SHIFT: u16 = 4;  /* IM_GPI8_EINT */
pub const WM8904_IM_GPI8_EINT_WIDTH: u16 = 1;  /* IM_GPI8_EINT */
pub const WM8904_IM_GPI7_EINT: u16 = 0x0008;  /* IM_GPI7_EINT */
pub const WM8904_IM_GPI7_EINT_MASK: u16 = 0x0008;  /* IM_GPI7_EINT */
pub const WM8904_IM_GPI7_EINT_SHIFT: u16 = 3;  /* IM_GPI7_EINT */
pub const WM8904_IM_GPI7_EINT_WIDTH: u16 = 1;  /* IM_GPI7_EINT */
pub const WM8904_IM_FLL_LOCK_EINT: u16 = 0x0004;  /* IM_FLL_LOCK_EINT */
pub const WM8904_IM_FLL_LOCK_EINT_MASK: u16 = 0x0004;  /* IM_FLL_LOCK_EINT */
pub const WM8904_IM_FLL_LOCK_EINT_SHIFT: u16 = 2;  /* IM_FLL_LOCK_EINT */
pub const WM8904_IM_FLL_LOCK_EINT_WIDTH: u16 = 1;  /* IM_FLL_LOCK_EINT */
pub const WM8904_IM_MIC_SHRT_EINT: u16 = 0x0002;  /* IM_MIC_SHRT_EINT */
pub const WM8904_IM_MIC_SHRT_EINT_MASK: u16 = 0x0002;  /* IM_MIC_SHRT_EINT */
pub const WM8904_IM_MIC_SHRT_EINT_SHIFT: u16 = 1;  /* IM_MIC_SHRT_EINT */
pub const WM8904_IM_MIC_SHRT_EINT_WIDTH: u16 = 1;  /* IM_MIC_SHRT_EINT */
pub const WM8904_IM_MIC_DET_EINT: u16 = 0x0001;  /* IM_MIC_DET_EINT */
pub const WM8904_IM_MIC_DET_EINT_MASK: u16 = 0x0001;  /* IM_MIC_DET_EINT */
pub const WM8904_IM_MIC_DET_EINT_SHIFT: u16 = 0;  /* IM_MIC_DET_EINT */
pub const WM8904_IM_MIC_DET_EINT_WIDTH: u16 = 1;  /* IM_MIC_DET_EINT */

/*
 * R129 (0x81) - Interrupt Polarity
 */
pub const WM8904_GPIO_BCLK_EINT_POL: u16 = 0x0200;  /* GPIO_BCLK_EINT_POL */
pub const WM8904_GPIO_BCLK_EINT_POL_MASK: u16 = 0x0200;  /* GPIO_BCLK_EINT_POL */
pub const WM8904_GPIO_BCLK_EINT_POL_SHIFT: u16 = 9;  /* GPIO_BCLK_EINT_POL */
pub const WM8904_GPIO_BCLK_EINT_POL_WIDTH: u16 = 1;  /* GPIO_BCLK_EINT_POL */
pub const WM8904_WSEQ_EINT_POL: u16 = 0x0100;  /* WSEQ_EINT_POL */
pub const WM8904_WSEQ_EINT_POL_MASK: u16 = 0x0100;  /* WSEQ_EINT_POL */
pub const WM8904_WSEQ_EINT_POL_SHIFT: u16 = 8;  /* WSEQ_EINT_POL */
pub const WM8904_WSEQ_EINT_POL_WIDTH: u16 = 1;  /* WSEQ_EINT_POL */
pub const WM8904_GPIO3_EINT_POL: u16 = 0x0080;  /* GPIO3_EINT_POL */
pub const WM8904_GPIO3_EINT_POL_MASK: u16 = 0x0080;  /* GPIO3_EINT_POL */
pub const WM8904_GPIO3_EINT_POL_SHIFT: u16 = 7;  /* GPIO3_EINT_POL */
pub const WM8904_GPIO3_EINT_POL_WIDTH: u16 = 1;  /* GPIO3_EINT_POL */
pub const WM8904_GPIO2_EINT_POL: u16 = 0x0040;  /* GPIO2_EINT_POL */
pub const WM8904_GPIO2_EINT_POL_MASK: u16 = 0x0040;  /* GPIO2_EINT_POL */
pub const WM8904_GPIO2_EINT_POL_SHIFT: u16 = 6;  /* GPIO2_EINT_POL */
pub const WM8904_GPIO2_EINT_POL_WIDTH: u16 = 1;  /* GPIO2_EINT_POL */
pub const WM8904_GPIO1_EINT_POL: u16 = 0x0020;  /* GPIO1_EINT_POL */
pub const WM8904_GPIO1_EINT_POL_MASK: u16 = 0x0020;  /* GPIO1_EINT_POL */
pub const WM8904_GPIO1_EINT_POL_SHIFT: u16 = 5;  /* GPIO1_EINT_POL */
pub const WM8904_GPIO1_EINT_POL_WIDTH: u16 = 1;  /* GPIO1_EINT_POL */
pub const WM8904_GPI8_EINT_POL: u16 = 0x0010;  /* GPI8_EINT_POL */
pub const WM8904_GPI8_EINT_POL_MASK: u16 = 0x0010;  /* GPI8_EINT_POL */
pub const WM8904_GPI8_EINT_POL_SHIFT: u16 = 4;  /* GPI8_EINT_POL */
pub const WM8904_GPI8_EINT_POL_WIDTH: u16 = 1;  /* GPI8_EINT_POL */
pub const WM8904_GPI7_EINT_POL: u16 = 0x0008;  /* GPI7_EINT_POL */
pub const WM8904_GPI7_EINT_POL_MASK: u16 = 0x0008;  /* GPI7_EINT_POL */
pub const WM8904_GPI7_EINT_POL_SHIFT: u16 = 3;  /* GPI7_EINT_POL */
pub const WM8904_GPI7_EINT_POL_WIDTH: u16 = 1;  /* GPI7_EINT_POL */
pub const WM8904_FLL_LOCK_EINT_POL: u16 = 0x0004;  /* FLL_LOCK_EINT_POL */
pub const WM8904_FLL_LOCK_EINT_POL_MASK: u16 = 0x0004;  /* FLL_LOCK_EINT_POL */
pub const WM8904_FLL_LOCK_EINT_POL_SHIFT: u16 = 2;  /* FLL_LOCK_EINT_POL */
pub const WM8904_FLL_LOCK_EINT_POL_WIDTH: u16 = 1;  /* FLL_LOCK_EINT_POL */
pub const WM8904_MIC_SHRT_EINT_POL: u16 = 0x0002;  /* MIC_SHRT_EINT_POL */
pub const WM8904_MIC_SHRT_EINT_POL_MASK: u16 = 0x0002;  /* MIC_SHRT_EINT_POL */
pub const WM8904_MIC_SHRT_EINT_POL_SHIFT: u16 = 1;  /* MIC_SHRT_EINT_POL */
pub const WM8904_MIC_SHRT_EINT_POL_WIDTH: u16 = 1;  /* MIC_SHRT_EINT_POL */
pub const WM8904_MIC_DET_EINT_POL: u16 = 0x0001;  /* MIC_DET_EINT_POL */
pub const WM8904_MIC_DET_EINT_POL_MASK: u16 = 0x0001;  /* MIC_DET_EINT_POL */
pub const WM8904_MIC_DET_EINT_POL_SHIFT: u16 = 0;  /* MIC_DET_EINT_POL */
pub const WM8904_MIC_DET_EINT_POL_WIDTH: u16 = 1;  /* MIC_DET_EINT_POL */

/*
 * R130 (0x82) - Interrupt Debounce
 */
pub const WM8904_GPIO_BCLK_EINT_DB: u16 = 0x0200;  /* GPIO_BCLK_EINT_DB */
pub const WM8904_GPIO_BCLK_EINT_DB_MASK: u16 = 0x0200;  /* GPIO_BCLK_EINT_DB */
pub const WM8904_GPIO_BCLK_EINT_DB_SHIFT: u16 = 9;  /* GPIO_BCLK_EINT_DB */
pub const WM8904_GPIO_BCLK_EINT_DB_WIDTH: u16 = 1;  /* GPIO_BCLK_EINT_DB */
pub const WM8904_WSEQ_EINT_DB: u16 = 0x0100;  /* WSEQ_EINT_DB */
pub const WM8904_WSEQ_EINT_DB_MASK: u16 = 0x0100;  /* WSEQ_EINT_DB */
pub const WM8904_WSEQ_EINT_DB_SHIFT: u16 = 8;  /* WSEQ_EINT_DB */
pub const WM8904_WSEQ_EINT_DB_WIDTH: u16 = 1;  /* WSEQ_EINT_DB */
pub const WM8904_GPIO3_EINT_DB: u16 = 0x0080;  /* GPIO3_EINT_DB */
pub const WM8904_GPIO3_EINT_DB_MASK: u16 = 0x0080;  /* GPIO3_EINT_DB */
pub const WM8904_GPIO3_EINT_DB_SHIFT: u16 = 7;  /* GPIO3_EINT_DB */
pub const WM8904_GPIO3_EINT_DB_WIDTH: u16 = 1;  /* GPIO3_EINT_DB */
pub const WM8904_GPIO2_EINT_DB: u16 = 0x0040;  /* GPIO2_EINT_DB */
pub const WM8904_GPIO2_EINT_DB_MASK: u16 = 0x0040;  /* GPIO2_EINT_DB */
pub const WM8904_GPIO2_EINT_DB_SHIFT: u16 = 6;  /* GPIO2_EINT_DB */
pub const WM8904_GPIO2_EINT_DB_WIDTH: u16 = 1;  /* GPIO2_EINT_DB */
pub const WM8904_GPIO1_EINT_DB: u16 = 0x0020;  /* GPIO1_EINT_DB */
pub const WM8904_GPIO1_EINT_DB_MASK: u16 = 0x0020;  /* GPIO1_EINT_DB */
pub const WM8904_GPIO1_EINT_DB_SHIFT: u16 = 5;  /* GPIO1_EINT_DB */
pub const WM8904_GPIO1_EINT_DB_WIDTH: u16 = 1;  /* GPIO1_EINT_DB */
pub const WM8904_GPI8_EINT_DB: u16 = 0x0010;  /* GPI8_EINT_DB */
pub const WM8904_GPI8_EINT_DB_MASK: u16 = 0x0010;  /* GPI8_EINT_DB */
pub const WM8904_GPI8_EINT_DB_SHIFT: u16 = 4;  /* GPI8_EINT_DB */
pub const WM8904_GPI8_EINT_DB_WIDTH: u16 = 1;  /* GPI8_EINT_DB */
pub const WM8904_GPI7_EINT_DB: u16 = 0x0008;  /* GPI7_EINT_DB */
pub const WM8904_GPI7_EINT_DB_MASK: u16 = 0x0008;  /* GPI7_EINT_DB */
pub const WM8904_GPI7_EINT_DB_SHIFT: u16 = 3;  /* GPI7_EINT_DB */
pub const WM8904_GPI7_EINT_DB_WIDTH: u16 = 1;  /* GPI7_EINT_DB */
pub const WM8904_FLL_LOCK_EINT_DB: u16 = 0x0004;  /* FLL_LOCK_EINT_DB */
pub const WM8904_FLL_LOCK_EINT_DB_MASK: u16 = 0x0004;  /* FLL_LOCK_EINT_DB */
pub const WM8904_FLL_LOCK_EINT_DB_SHIFT: u16 = 2;  /* FLL_LOCK_EINT_DB */
pub const WM8904_FLL_LOCK_EINT_DB_WIDTH: u16 = 1;  /* FLL_LOCK_EINT_DB */
pub const WM8904_MIC_SHRT_EINT_DB: u16 = 0x0002;  /* MIC_SHRT_EINT_DB */
pub const WM8904_MIC_SHRT_EINT_DB_MASK: u16 = 0x0002;  /* MIC_SHRT_EINT_DB */
pub const WM8904_MIC_SHRT_EINT_DB_SHIFT: u16 = 1;  /* MIC_SHRT_EINT_DB */
pub const WM8904_MIC_SHRT_EINT_DB_WIDTH: u16 = 1;  /* MIC_SHRT_EINT_DB */
pub const WM8904_MIC_DET_EINT_DB: u16 = 0x0001;  /* MIC_DET_EINT_DB */
pub const WM8904_MIC_DET_EINT_DB_MASK: u16 = 0x0001;  /* MIC_DET_EINT_DB */
pub const WM8904_MIC_DET_EINT_DB_SHIFT: u16 = 0;  /* MIC_DET_EINT_DB */
pub const WM8904_MIC_DET_EINT_DB_WIDTH: u16 = 1;  /* MIC_DET_EINT_DB */

/*
 * R134 (0x86) - EQ1
 */
pub const WM8904_EQ_ENA: u16 = 0x0001;  /* EQ_ENA */
pub const WM8904_EQ_ENA_MASK: u16 = 0x0001;  /* EQ_ENA */
pub const WM8904_EQ_ENA_SHIFT: u16 = 0;  /* EQ_ENA */
pub const WM8904_EQ_ENA_WIDTH: u16 = 1;  /* EQ_ENA */

/*
 * R135 (0x87) - EQ2
 */
pub const WM8904_EQ_B1_GAIN_MASK: u16 = 0x001F;  /* EQ_B1_GAIN - [4:0] */
pub const WM8904_EQ_B1_GAIN_SHIFT: u16 = 0;  /* EQ_B1_GAIN - [4:0] */
pub const WM8904_EQ_B1_GAIN_WIDTH: u16 = 5;  /* EQ_B1_GAIN - [4:0] */

/*
 * R136 (0x88) - EQ3
 */
pub const WM8904_EQ_B2_GAIN_MASK: u16 = 0x001F;  /* EQ_B2_GAIN - [4:0] */
pub const WM8904_EQ_B2_GAIN_SHIFT: u16 = 0;  /* EQ_B2_GAIN - [4:0] */
pub const WM8904_EQ_B2_GAIN_WIDTH: u16 = 5;  /* EQ_B2_GAIN - [4:0] */

/*
 * R137 (0x89) - EQ4
 */
pub const WM8904_EQ_B3_GAIN_MASK: u16 = 0x001F;  /* EQ_B3_GAIN - [4:0] */
pub const WM8904_EQ_B3_GAIN_SHIFT: u16 = 0;  /* EQ_B3_GAIN - [4:0] */
pub const WM8904_EQ_B3_GAIN_WIDTH: u16 = 5;  /* EQ_B3_GAIN - [4:0] */

/*
 * R138 (0x8A) - EQ5
 */
pub const WM8904_EQ_B4_GAIN_MASK: u16 = 0x001F;  /* EQ_B4_GAIN - [4:0] */
pub const WM8904_EQ_B4_GAIN_SHIFT: u16 = 0;  /* EQ_B4_GAIN - [4:0] */
pub const WM8904_EQ_B4_GAIN_WIDTH: u16 = 5;  /* EQ_B4_GAIN - [4:0] */

/*
 * R139 (0x8B) - EQ6
 */
pub const WM8904_EQ_B5_GAIN_MASK: u16 = 0x001F;  /* EQ_B5_GAIN - [4:0] */
pub const WM8904_EQ_B5_GAIN_SHIFT: u16 = 0;  /* EQ_B5_GAIN - [4:0] */
pub const WM8904_EQ_B5_GAIN_WIDTH: u16 = 5;  /* EQ_B5_GAIN - [4:0] */

/*
 * R140 (0x8C) - EQ7
 */
pub const WM8904_EQ_B1_A_MASK: u16 = 0xFFFF;  /* EQ_B1_A - [15:0] */
pub const WM8904_EQ_B1_A_SHIFT: u16 = 0;  /* EQ_B1_A - [15:0] */
pub const WM8904_EQ_B1_A_WIDTH: u16 = 16;  /* EQ_B1_A - [15:0] */

/*
 * R141 (0x8D) - EQ8
 */
pub const WM8904_EQ_B1_B_MASK: u16 = 0xFFFF;  /* EQ_B1_B - [15:0] */
pub const WM8904_EQ_B1_B_SHIFT: u16 = 0;  /* EQ_B1_B - [15:0] */
pub const WM8904_EQ_B1_B_WIDTH: u16 = 16;  /* EQ_B1_B - [15:0] */

/*
 * R142 (0x8E) - EQ9
 */
pub const WM8904_EQ_B1_PG_MASK: u16 = 0xFFFF;  /* EQ_B1_PG - [15:0] */
pub const WM8904_EQ_B1_PG_SHIFT: u16 = 0;  /* EQ_B1_PG - [15:0] */
pub const WM8904_EQ_B1_PG_WIDTH: u16 = 16;  /* EQ_B1_PG - [15:0] */

/*
 * R143 (0x8F) - EQ10
 */
pub const WM8904_EQ_B2_A_MASK: u16 = 0xFFFF;  /* EQ_B2_A - [15:0] */
pub const WM8904_EQ_B2_A_SHIFT: u16 = 0;  /* EQ_B2_A - [15:0] */
pub const WM8904_EQ_B2_A_WIDTH: u16 = 16;  /* EQ_B2_A - [15:0] */

/*
 * R144 (0x90) - EQ11
 */
pub const WM8904_EQ_B2_B_MASK: u16 = 0xFFFF;  /* EQ_B2_B - [15:0] */
pub const WM8904_EQ_B2_B_SHIFT: u16 = 0;  /* EQ_B2_B - [15:0] */
pub const WM8904_EQ_B2_B_WIDTH: u16 = 16;  /* EQ_B2_B - [15:0] */

/*
 * R145 (0x91) - EQ12
 */
pub const WM8904_EQ_B2_C_MASK: u16 = 0xFFFF;  /* EQ_B2_C - [15:0] */
pub const WM8904_EQ_B2_C_SHIFT: u16 = 0;  /* EQ_B2_C - [15:0] */
pub const WM8904_EQ_B2_C_WIDTH: u16 = 16;  /* EQ_B2_C - [15:0] */

/*
 * R146 (0x92) - EQ13
 */
pub const WM8904_EQ_B2_PG_MASK: u16 = 0xFFFF;  /* EQ_B2_PG - [15:0] */
pub const WM8904_EQ_B2_PG_SHIFT: u16 = 0;  /* EQ_B2_PG - [15:0] */
pub const WM8904_EQ_B2_PG_WIDTH: u16 = 16;  /* EQ_B2_PG - [15:0] */

/*
 * R147 (0x93) - EQ14
 */
pub const WM8904_EQ_B3_A_MASK: u16 = 0xFFFF;  /* EQ_B3_A - [15:0] */
pub const WM8904_EQ_B3_A_SHIFT: u16 = 0;  /* EQ_B3_A - [15:0] */
pub const WM8904_EQ_B3_A_WIDTH: u16 = 16;  /* EQ_B3_A - [15:0] */

/*
 * R148 (0x94) - EQ15
 */
pub const WM8904_EQ_B3_B_MASK: u16 = 0xFFFF;  /* EQ_B3_B - [15:0] */
pub const WM8904_EQ_B3_B_SHIFT: u16 = 0;  /* EQ_B3_B - [15:0] */
pub const WM8904_EQ_B3_B_WIDTH: u16 = 16;  /* EQ_B3_B - [15:0] */

/*
 * R149 (0x95) - EQ16
 */
pub const WM8904_EQ_B3_C_MASK: u16 = 0xFFFF;  /* EQ_B3_C - [15:0] */
pub const WM8904_EQ_B3_C_SHIFT: u16 = 0;  /* EQ_B3_C - [15:0] */
pub const WM8904_EQ_B3_C_WIDTH: u16 = 16;  /* EQ_B3_C - [15:0] */

/*
 * R150 (0x96) - EQ17
 */
pub const WM8904_EQ_B3_PG_MASK: u16 = 0xFFFF;  /* EQ_B3_PG - [15:0] */
pub const WM8904_EQ_B3_PG_SHIFT: u16 = 0;  /* EQ_B3_PG - [15:0] */
pub const WM8904_EQ_B3_PG_WIDTH: u16 = 16;  /* EQ_B3_PG - [15:0] */

/*
 * R151 (0x97) - EQ18
 */
pub const WM8904_EQ_B4_A_MASK: u16 = 0xFFFF;  /* EQ_B4_A - [15:0] */
pub const WM8904_EQ_B4_A_SHIFT: u16 = 0;  /* EQ_B4_A - [15:0] */
pub const WM8904_EQ_B4_A_WIDTH: u16 = 16;  /* EQ_B4_A - [15:0] */

/*
 * R152 (0x98) - EQ19
 */
pub const WM8904_EQ_B4_B_MASK: u16 = 0xFFFF;  /* EQ_B4_B - [15:0] */
pub const WM8904_EQ_B4_B_SHIFT: u16 = 0;  /* EQ_B4_B - [15:0] */
pub const WM8904_EQ_B4_B_WIDTH: u16 = 16;  /* EQ_B4_B - [15:0] */

/*
 * R153 (0x99) - EQ20
 */
pub const WM8904_EQ_B4_C_MASK: u16 = 0xFFFF;  /* EQ_B4_C - [15:0] */
pub const WM8904_EQ_B4_C_SHIFT: u16 = 0;  /* EQ_B4_C - [15:0] */
pub const WM8904_EQ_B4_C_WIDTH: u16 = 16;  /* EQ_B4_C - [15:0] */

/*
 * R154 (0x9A) - EQ21
 */
pub const WM8904_EQ_B4_PG_MASK: u16 = 0xFFFF;  /* EQ_B4_PG - [15:0] */
pub const WM8904_EQ_B4_PG_SHIFT: u16 = 0;  /* EQ_B4_PG - [15:0] */
pub const WM8904_EQ_B4_PG_WIDTH: u16 = 16;  /* EQ_B4_PG - [15:0] */

/*
 * R155 (0x9B) - EQ22
 */
pub const WM8904_EQ_B5_A_MASK: u16 = 0xFFFF;  /* EQ_B5_A - [15:0] */
pub const WM8904_EQ_B5_A_SHIFT: u16 = 0;  /* EQ_B5_A - [15:0] */
pub const WM8904_EQ_B5_A_WIDTH: u16 = 16;  /* EQ_B5_A - [15:0] */

/*
 * R156 (0x9C) - EQ23
 */
pub const WM8904_EQ_B5_B_MASK: u16 = 0xFFFF;  /* EQ_B5_B - [15:0] */
pub const WM8904_EQ_B5_B_SHIFT: u16 = 0;  /* EQ_B5_B - [15:0] */
pub const WM8904_EQ_B5_B_WIDTH: u16 = 16;  /* EQ_B5_B - [15:0] */

/*
 * R157 (0x9D) - EQ24
 */
pub const WM8904_EQ_B5_PG_MASK: u16 = 0xFFFF;  /* EQ_B5_PG - [15:0] */
pub const WM8904_EQ_B5_PG_SHIFT: u16 = 0;  /* EQ_B5_PG - [15:0] */
pub const WM8904_EQ_B5_PG_WIDTH: u16 = 16;  /* EQ_B5_PG - [15:0] */

/*
 * R161 (0xA1) - Control Interface Test 1
 */
pub const WM8904_USER_KEY: u16 = 0x0002;  /* USER_KEY */
pub const WM8904_USER_KEY_MASK: u16 = 0x0002;  /* USER_KEY */
pub const WM8904_USER_KEY_SHIFT: u16 = 1;  /* USER_KEY */
pub const WM8904_USER_KEY_WIDTH: u16 = 1;  /* USER_KEY */

/*
 * R198 (0xC6) - ADC Test 0
 */
pub const WM8904_ADC_128_OSR_TST_MODE: u16 = 0x0004;  /* ADC_128_OSR_TST_MODE */
pub const WM8904_ADC_128_OSR_TST_MODE_SHIFT: u16 = 2;  /* ADC_128_OSR_TST_MODE */
pub const WM8904_ADC_128_OSR_TST_MODE_WIDTH: u16 = 1;  /* ADC_128_OSR_TST_MODE */
pub const WM8904_ADC_BIASX1P5: u16 = 0x0001;  /* ADC_BIASX1P5 */
pub const WM8904_ADC_BIASX1P5_SHIFT: u16 = 0;  /* ADC_BIASX1P5 */
pub const WM8904_ADC_BIASX1P5_WIDTH: u16 = 1;  /* ADC_BIASX1P5 */

/*
 * R204 (0xCC) - Analogue Output Bias 0
 */
pub const WM8904_PGA_BIAS_MASK: u16 = 0x0070;  /* PGA_BIAS - [6:4] */
pub const WM8904_PGA_BIAS_SHIFT: u16 = 4;  /* PGA_BIAS - [6:4] */
pub const WM8904_PGA_BIAS_WIDTH: u16 = 3;  /* PGA_BIAS - [6:4] */

/*
 * R247 (0xF7) - FLL NCO Test 0
 */
pub const WM8904_FLL_FRC_NCO: u16 = 0x0001;  /* FLL_FRC_NCO */
pub const WM8904_FLL_FRC_NCO_MASK: u16 = 0x0001;  /* FLL_FRC_NCO */
pub const WM8904_FLL_FRC_NCO_SHIFT: u16 = 0;  /* FLL_FRC_NCO */
pub const WM8904_FLL_FRC_NCO_WIDTH: u16 = 1;  /* FLL_FRC_NCO */

/*
 * R248 (0xF8) - FLL NCO Test 1
 */
pub const WM8904_FLL_FRC_NCO_VAL_MASK: u16 = 0x003F;  /* FLL_FRC_NCO_VAL - [5:0] */
pub const WM8904_FLL_FRC_NCO_VAL_SHIFT: u16 = 0;  /* FLL_FRC_NCO_VAL - [5:0] */
pub const WM8904_FLL_FRC_NCO_VAL_WIDTH: u16 = 6;  /* FLL_FRC_NCO_VAL - [5:0] */


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
