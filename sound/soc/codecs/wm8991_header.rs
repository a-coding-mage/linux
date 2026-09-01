/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * wm8991.h  --  audio driver for WM8991
 *
 * Copyright 2007 Wolfson Microelectronics PLC.
 * Author: Graeme Gregory
 *         graeme.gregory@wolfsonmicro.com or linux@wolfsonmicro.com
 */


/*
 * Register values.
 */
pub const WM8991_RESET: u32 = 0x00;
pub const WM8991_POWER_MANAGEMENT_1: u32 = 0x01;
pub const WM8991_POWER_MANAGEMENT_2: u32 = 0x02;
pub const WM8991_POWER_MANAGEMENT_3: u32 = 0x03;
pub const WM8991_AUDIO_INTERFACE_1: u32 = 0x04;
pub const WM8991_AUDIO_INTERFACE_2: u32 = 0x05;
pub const WM8991_CLOCKING_1: u32 = 0x06;
pub const WM8991_CLOCKING_2: u32 = 0x07;
pub const WM8991_AUDIO_INTERFACE_3: u32 = 0x08;
pub const WM8991_AUDIO_INTERFACE_4: u32 = 0x09;
pub const WM8991_DAC_CTRL: u32 = 0x0A;
pub const WM8991_LEFT_DAC_DIGITAL_VOLUME: u32 = 0x0B;
pub const WM8991_RIGHT_DAC_DIGITAL_VOLUME: u32 = 0x0C;
pub const WM8991_DIGITAL_SIDE_TONE: u32 = 0x0D;
pub const WM8991_ADC_CTRL: u32 = 0x0E;
pub const WM8991_LEFT_ADC_DIGITAL_VOLUME: u32 = 0x0F;
pub const WM8991_RIGHT_ADC_DIGITAL_VOLUME: u32 = 0x10;
pub const WM8991_GPIO_CTRL_1: u32 = 0x12;
pub const WM8991_GPIO1_GPIO2: u32 = 0x13;
pub const WM8991_GPIO3_GPIO4: u32 = 0x14;
pub const WM8991_GPIO5_GPIO6: u32 = 0x15;
pub const WM8991_GPIOCTRL_2: u32 = 0x16;
pub const WM8991_GPIO_POL: u32 = 0x17;
pub const WM8991_LEFT_LINE_INPUT_1_2_VOLUME: u32 = 0x18;
pub const WM8991_LEFT_LINE_INPUT_3_4_VOLUME: u32 = 0x19;
pub const WM8991_RIGHT_LINE_INPUT_1_2_VOLUME: u32 = 0x1A;
pub const WM8991_RIGHT_LINE_INPUT_3_4_VOLUME: u32 = 0x1B;
pub const WM8991_LEFT_OUTPUT_VOLUME: u32 = 0x1C;
pub const WM8991_RIGHT_OUTPUT_VOLUME: u32 = 0x1D;
pub const WM8991_LINE_OUTPUTS_VOLUME: u32 = 0x1E;
pub const WM8991_OUT3_4_VOLUME: u32 = 0x1F;
pub const WM8991_LEFT_OPGA_VOLUME: u32 = 0x20;
pub const WM8991_RIGHT_OPGA_VOLUME: u32 = 0x21;
pub const WM8991_SPEAKER_VOLUME: u32 = 0x22;
pub const WM8991_CLASSD1: u32 = 0x23;
pub const WM8991_CLASSD3: u32 = 0x25;
pub const WM8991_INPUT_MIXER1: u32 = 0x27;
pub const WM8991_INPUT_MIXER2: u32 = 0x28;
pub const WM8991_INPUT_MIXER3: u32 = 0x29;
pub const WM8991_INPUT_MIXER4: u32 = 0x2A;
pub const WM8991_INPUT_MIXER5: u32 = 0x2B;
pub const WM8991_INPUT_MIXER6: u32 = 0x2C;
pub const WM8991_OUTPUT_MIXER1: u32 = 0x2D;
pub const WM8991_OUTPUT_MIXER2: u32 = 0x2E;
pub const WM8991_OUTPUT_MIXER3: u32 = 0x2F;
pub const WM8991_OUTPUT_MIXER4: u32 = 0x30;
pub const WM8991_OUTPUT_MIXER5: u32 = 0x31;
pub const WM8991_OUTPUT_MIXER6: u32 = 0x32;
pub const WM8991_OUT3_4_MIXER: u32 = 0x33;
pub const WM8991_LINE_MIXER1: u32 = 0x34;
pub const WM8991_LINE_MIXER2: u32 = 0x35;
pub const WM8991_SPEAKER_MIXER: u32 = 0x36;
pub const WM8991_ADDITIONAL_CONTROL: u32 = 0x37;
pub const WM8991_ANTIPOP1: u32 = 0x38;
pub const WM8991_ANTIPOP2: u32 = 0x39;
pub const WM8991_MICBIAS: u32 = 0x3A;
pub const WM8991_PLL1: u32 = 0x3C;
pub const WM8991_PLL2: u32 = 0x3D;
pub const WM8991_PLL3: u32 = 0x3E;

pub const WM8991_REGISTER_COUNT: u32 = 60;
pub const WM8991_MAX_REGISTER: u32 = 0x3F;

/*
 * Field Definitions.
 */

/*
 * R0 (0x00) - Reset
 */
pub const WM8991_SW_RESET_CHIP_ID_MASK: u32 = 0xFFFF; /* SW_RESET_CHIP_ID - [15:0] */

/*
 * R1 (0x01) - Power Management (1)
 */
pub const WM8991_SPK_ENA: u32 = 0x1000; /* SPK_ENA */
pub const WM8991_SPK_ENA_BIT: u32 = 12;
pub const WM8991_OUT3_ENA: u32 = 0x0800; /* OUT3_ENA */
pub const WM8991_OUT3_ENA_BIT: u32 = 11;
pub const WM8991_OUT4_ENA: u32 = 0x0400; /* OUT4_ENA */
pub const WM8991_OUT4_ENA_BIT: u32 = 10;
pub const WM8991_LOUT_ENA: u32 = 0x0200; /* LOUT_ENA */
pub const WM8991_LOUT_ENA_BIT: u32 = 9;
pub const WM8991_ROUT_ENA: u32 = 0x0100; /* ROUT_ENA */
pub const WM8991_ROUT_ENA_BIT: u32 = 8;
pub const WM8991_MICBIAS_ENA: u32 = 0x0010; /* MICBIAS_ENA */
pub const WM8991_MICBIAS_ENA_BIT: u32 = 4;
pub const WM8991_VMID_MODE_MASK: u32 = 0x0006; /* VMID_MODE - [2:1] */
pub const WM8991_VREF_ENA: u32 = 0x0001; /* VREF_ENA */
pub const WM8991_VREF_ENA_BIT: u32 = 0;

/*
 * R2 (0x02) - Power Management (2)
 */
pub const WM8991_PLL_ENA: u32 = 0x8000; /* PLL_ENA */
pub const WM8991_PLL_ENA_BIT: u32 = 15;
pub const WM8991_TSHUT_ENA: u32 = 0x4000; /* TSHUT_ENA */
pub const WM8991_TSHUT_ENA_BIT: u32 = 14;
pub const WM8991_TSHUT_OPDIS: u32 = 0x2000; /* TSHUT_OPDIS */
pub const WM8991_TSHUT_OPDIS_BIT: u32 = 13;
pub const WM8991_OPCLK_ENA: u32 = 0x0800; /* OPCLK_ENA */
pub const WM8991_OPCLK_ENA_BIT: u32 = 11;
pub const WM8991_AINL_ENA: u32 = 0x0200; /* AINL_ENA */
pub const WM8991_AINL_ENA_BIT: u32 = 9;
pub const WM8991_AINR_ENA: u32 = 0x0100; /* AINR_ENA */
pub const WM8991_AINR_ENA_BIT: u32 = 8;
pub const WM8991_LIN34_ENA: u32 = 0x0080; /* LIN34_ENA */
pub const WM8991_LIN34_ENA_BIT: u32 = 7;
pub const WM8991_LIN12_ENA: u32 = 0x0040; /* LIN12_ENA */
pub const WM8991_LIN12_ENA_BIT: u32 = 6;
pub const WM8991_RIN34_ENA: u32 = 0x0020; /* RIN34_ENA */
pub const WM8991_RIN34_ENA_BIT: u32 = 5;
pub const WM8991_RIN12_ENA: u32 = 0x0010; /* RIN12_ENA */
pub const WM8991_RIN12_ENA_BIT: u32 = 4;
pub const WM8991_ADCL_ENA: u32 = 0x0002; /* ADCL_ENA */
pub const WM8991_ADCL_ENA_BIT: u32 = 1;
pub const WM8991_ADCR_ENA: u32 = 0x0001; /* ADCR_ENA */
pub const WM8991_ADCR_ENA_BIT: u32 = 0;

/*
 * R3 (0x03) - Power Management (3)
 */
pub const WM8991_LON_ENA: u32 = 0x2000; /* LON_ENA */
pub const WM8991_LON_ENA_BIT: u32 = 13;
pub const WM8991_LOP_ENA: u32 = 0x1000; /* LOP_ENA */
pub const WM8991_LOP_ENA_BIT: u32 = 12;
pub const WM8991_RON_ENA: u32 = 0x0800; /* RON_ENA */
pub const WM8991_RON_ENA_BIT: u32 = 11;
pub const WM8991_ROP_ENA: u32 = 0x0400; /* ROP_ENA */
pub const WM8991_ROP_ENA_BIT: u32 = 10;
pub const WM8991_LOPGA_ENA: u32 = 0x0080; /* LOPGA_ENA */
pub const WM8991_LOPGA_ENA_BIT: u32 = 7;
pub const WM8991_ROPGA_ENA: u32 = 0x0040; /* ROPGA_ENA */
pub const WM8991_ROPGA_ENA_BIT: u32 = 6;
pub const WM8991_LOMIX_ENA: u32 = 0x0020; /* LOMIX_ENA */
pub const WM8991_LOMIX_ENA_BIT: u32 = 5;
pub const WM8991_ROMIX_ENA: u32 = 0x0010; /* ROMIX_ENA */
pub const WM8991_ROMIX_ENA_BIT: u32 = 4;
pub const WM8991_DACL_ENA: u32 = 0x0002; /* DACL_ENA */
pub const WM8991_DACL_ENA_BIT: u32 = 1;
pub const WM8991_DACR_ENA: u32 = 0x0001; /* DACR_ENA */
pub const WM8991_DACR_ENA_BIT: u32 = 0;

/*
 * R4 (0x04) - Audio Interface (1)
 */
pub const WM8991_AIFADCL_SRC: u32 = 0x8000; /* AIFADCL_SRC */
pub const WM8991_AIFADCR_SRC: u32 = 0x4000; /* AIFADCR_SRC */
pub const WM8991_AIFADC_TDM: u32 = 0x2000; /* AIFADC_TDM */
pub const WM8991_AIFADC_TDM_CHAN: u32 = 0x1000; /* AIFADC_TDM_CHAN */
pub const WM8991_AIF_BCLK_INV: u32 = 0x0100; /* AIF_BCLK_INV */
pub const WM8991_AIF_LRCLK_INV: u32 = 0x0080; /* AIF_LRCLK_INV */
pub const WM8991_AIF_WL_MASK: u32 = 0x0060; /* AIF_WL - [6:5] */
pub const WM8991_AIF_WL_16BITS: u32 = 0 << 5;
pub const WM8991_AIF_WL_20BITS: u32 = 1 << 5;
pub const WM8991_AIF_WL_24BITS: u32 = 2 << 5;
pub const WM8991_AIF_WL_32BITS: u32 = 3 << 5;
pub const WM8991_AIF_FMT_MASK: u32 = 0x0018; /* AIF_FMT - [4:3] */
pub const WM8991_AIF_TMF_RIGHTJ: u32 = 0 << 3;
pub const WM8991_AIF_TMF_LEFTJ: u32 = 1 << 3;
pub const WM8991_AIF_TMF_I2S: u32 = 2 << 3;
pub const WM8991_AIF_TMF_DSP: u32 = 3 << 3;

/*
 * R5 (0x05) - Audio Interface (2)
 */
pub const WM8991_DACL_SRC: u32 = 0x8000; /* DACL_SRC */
pub const WM8991_DACR_SRC: u32 = 0x4000; /* DACR_SRC */
pub const WM8991_AIFDAC_TDM: u32 = 0x2000; /* AIFDAC_TDM */
pub const WM8991_AIFDAC_TDM_CHAN: u32 = 0x1000; /* AIFDAC_TDM_CHAN */
pub const WM8991_DAC_BOOST_MASK: u32 = 0x0C00; /* DAC_BOOST - [11:10] */
pub const WM8991_DAC_COMP: u32 = 0x0010; /* DAC_COMP */
pub const WM8991_DAC_COMPMODE: u32 = 0x0008; /* DAC_COMPMODE */
pub const WM8991_ADC_COMP: u32 = 0x0004; /* ADC_COMP */
pub const WM8991_ADC_COMPMODE: u32 = 0x0002; /* ADC_COMPMODE */
pub const WM8991_LOOPBACK: u32 = 0x0001; /* LOOPBACK */

/*
 * R6 (0x06) - Clocking (1)
 */
pub const WM8991_TOCLK_RATE: u32 = 0x8000; /* TOCLK_RATE */
pub const WM8991_TOCLK_ENA: u32 = 0x4000; /* TOCLK_ENA */
pub const WM8991_OPCLKDIV_MASK: u32 = 0x1E00; /* OPCLKDIV - [12:9] */
pub const WM8991_DCLKDIV_MASK: u32 = 0x01C0; /* DCLKDIV - [8:6] */
pub const WM8991_BCLK_DIV_MASK: u32 = 0x001E; /* BCLK_DIV - [4:1] */
pub const WM8991_BCLK_DIV_1: u32 = 0x0 << 1;
pub const WM8991_BCLK_DIV_1_5: u32 = 0x1 << 1;
pub const WM8991_BCLK_DIV_2: u32 = 0x2 << 1;
pub const WM8991_BCLK_DIV_3: u32 = 0x3 << 1;
pub const WM8991_BCLK_DIV_4: u32 = 0x4 << 1;
pub const WM8991_BCLK_DIV_5_5: u32 = 0x5 << 1;
pub const WM8991_BCLK_DIV_6: u32 = 0x6 << 1;
pub const WM8991_BCLK_DIV_8: u32 = 0x7 << 1;
pub const WM8991_BCLK_DIV_11: u32 = 0x8 << 1;
pub const WM8991_BCLK_DIV_12: u32 = 0x9 << 1;
pub const WM8991_BCLK_DIV_16: u32 = 0xA << 1;
pub const WM8991_BCLK_DIV_22: u32 = 0xB << 1;
pub const WM8991_BCLK_DIV_24: u32 = 0xC << 1;
pub const WM8991_BCLK_DIV_32: u32 = 0xD << 1;
pub const WM8991_BCLK_DIV_44: u32 = 0xE << 1;
pub const WM8991_BCLK_DIV_48: u32 = 0xF << 1;

/*
 * R7 (0x07) - Clocking (2)
 */
pub const WM8991_MCLK_SRC: u32 = 0x8000; /* MCLK_SRC */
pub const WM8991_SYSCLK_SRC: u32 = 0x4000; /* SYSCLK_SRC */
pub const WM8991_CLK_FORCE: u32 = 0x2000; /* CLK_FORCE */
pub const WM8991_MCLK_DIV_MASK: u32 = 0x1800; /* MCLK_DIV - [12:11] */
pub const WM8991_MCLK_DIV_1: u32 = 0 << 11;
pub const WM8991_MCLK_DIV_2: u32 =  2 << 11;
pub const WM8991_MCLK_INV: u32 = 0x0400; /* MCLK_INV */
pub const WM8991_ADC_CLKDIV_MASK: u32 = 0x00E0; /* ADC_CLKDIV - [7:5] */
pub const WM8991_ADC_CLKDIV_1: u32 = 0 << 5;
pub const WM8991_ADC_CLKDIV_1_5: u32 = 1 << 5;
pub const WM8991_ADC_CLKDIV_2: u32 = 2 << 5;
pub const WM8991_ADC_CLKDIV_3: u32 = 3 << 5;
pub const WM8991_ADC_CLKDIV_4: u32 = 4 << 5;
pub const WM8991_ADC_CLKDIV_5_5: u32 = 5 << 5;
pub const WM8991_ADC_CLKDIV_6: u32 = 6 << 5;
pub const WM8991_DAC_CLKDIV_MASK: u32 = 0x001C; /* DAC_CLKDIV - [4:2] */
pub const WM8991_DAC_CLKDIV_1: u32 = 0 << 2;
pub const WM8991_DAC_CLKDIV_1_5: u32 = 1 << 2;
pub const WM8991_DAC_CLKDIV_2: u32 = 2 << 2;
pub const WM8991_DAC_CLKDIV_3: u32 = 3 << 2;
pub const WM8991_DAC_CLKDIV_4: u32 = 4 << 2;
pub const WM8991_DAC_CLKDIV_5_5: u32 = 5 << 2;
pub const WM8991_DAC_CLKDIV_6: u32 = 6 << 2;

/*
 * R8 (0x08) - Audio Interface (3)
 */
pub const WM8991_AIF_MSTR1: u32 = 0x8000; /* AIF_MSTR1 */
pub const WM8991_AIF_MSTR2: u32 = 0x4000; /* AIF_MSTR2 */
pub const WM8991_AIF_SEL: u32 = 0x2000; /* AIF_SEL */
pub const WM8991_ADCLRC_DIR: u32 = 0x0800; /* ADCLRC_DIR */
pub const WM8991_ADCLRC_RATE_MASK: u32 = 0x07FF; /* ADCLRC_RATE - [10:0] */

/*
 * R9 (0x09) - Audio Interface (4)
 */
pub const WM8991_ALRCGPIO1: u32 = 0x8000; /* ALRCGPIO1 */
pub const WM8991_ALRCBGPIO6: u32 = 0x4000; /* ALRCBGPIO6 */
pub const WM8991_AIF_TRIS: u32 = 0x2000; /* AIF_TRIS */
pub const WM8991_DACLRC_DIR: u32 = 0x0800; /* DACLRC_DIR */
pub const WM8991_DACLRC_RATE_MASK: u32 = 0x07FF; /* DACLRC_RATE - [10:0] */

/*
 * R10 (0x0A) - DAC CTRL
 */
pub const WM8991_AIF_LRCLKRATE: u32 = 0x0400; /* AIF_LRCLKRATE */
pub const WM8991_DAC_MONO: u32 = 0x0200; /* DAC_MONO */
pub const WM8991_DAC_SB_FILT: u32 = 0x0100; /* DAC_SB_FILT */
pub const WM8991_DAC_MUTERATE: u32 = 0x0080; /* DAC_MUTERATE */
pub const WM8991_DAC_MUTEMODE: u32 = 0x0040; /* DAC_MUTEMODE */
pub const WM8991_DEEMP_MASK: u32 = 0x0030; /* DEEMP - [5:4] */
pub const WM8991_DAC_MUTE: u32 = 0x0004; /* DAC_MUTE */
pub const WM8991_DACL_DATINV: u32 = 0x0002; /* DACL_DATINV */
pub const WM8991_DACR_DATINV: u32 = 0x0001; /* DACR_DATINV */

/*
 * R11 (0x0B) - Left DAC Digital Volume
 */
pub const WM8991_DAC_VU: u32 = 0x0100; /* DAC_VU */
pub const WM8991_DACL_VOL_MASK: u32 = 0x00FF; /* DACL_VOL - [7:0] */
pub const WM8991_DACL_VOL_SHIFT: u32 = 0;
/*
 * R12 (0x0C) - Right DAC Digital Volume
 */
// Duplicate C macro: #define WM8991_DAC_VU 0x0100 /* DAC_VU */
pub const WM8991_DACR_VOL_MASK: u32 = 0x00FF; /* DACR_VOL - [7:0] */
pub const WM8991_DACR_VOL_SHIFT: u32 = 0;
/*
 * R13 (0x0D) - Digital Side Tone
 */
pub const WM8991_ADCL_DAC_SVOL_MASK: u32 = 0x0F; /* ADCL_DAC_SVOL - [12:9] */
pub const WM8991_ADCL_DAC_SVOL_SHIFT: u32 = 9;
pub const WM8991_ADCR_DAC_SVOL_MASK: u32 = 0x0F; /* ADCR_DAC_SVOL - [8:5] */
pub const WM8991_ADCR_DAC_SVOL_SHIFT: u32 = 5;
pub const WM8991_ADC_TO_DACL_MASK: u32 = 0x03; /* ADC_TO_DACL - [3:2] */
pub const WM8991_ADC_TO_DACL_SHIFT: u32 = 2;
pub const WM8991_ADC_TO_DACR_MASK: u32 = 0x03; /* ADC_TO_DACR - [1:0] */
pub const WM8991_ADC_TO_DACR_SHIFT: u32 = 0;

/*
 * R14 (0x0E) - ADC CTRL
 */
pub const WM8991_ADC_HPF_ENA: u32 = 0x0100; /* ADC_HPF_ENA */
pub const WM8991_ADC_HPF_ENA_BIT: u32 = 8;
pub const WM8991_ADC_HPF_CUT_MASK: u32 = 0x03; /* ADC_HPF_CUT - [6:5] */
pub const WM8991_ADC_HPF_CUT_SHIFT: u32 = 5;
pub const WM8991_ADCL_DATINV: u32 = 0x0002; /* ADCL_DATINV */
pub const WM8991_ADCL_DATINV_BIT: u32 = 1;
pub const WM8991_ADCR_DATINV: u32 = 0x0001; /* ADCR_DATINV */
pub const WM8991_ADCR_DATINV_BIT: u32 = 0;

/*
 * R15 (0x0F) - Left ADC Digital Volume
 */
pub const WM8991_ADC_VU: u32 = 0x0100; /* ADC_VU */
pub const WM8991_ADCL_VOL_MASK: u32 = 0x00FF; /* ADCL_VOL - [7:0] */
pub const WM8991_ADCL_VOL_SHIFT: u32 = 0;

/*
 * R16 (0x10) - Right ADC Digital Volume
 */
// Duplicate C macro: #define WM8991_ADC_VU 0x0100 /* ADC_VU */
pub const WM8991_ADCR_VOL_MASK: u32 = 0x00FF; /* ADCR_VOL - [7:0] */
pub const WM8991_ADCR_VOL_SHIFT: u32 = 0;

/*
 * R18 (0x12) - GPIO CTRL 1
 */
pub const WM8991_IRQ: u32 = 0x1000; /* IRQ */
pub const WM8991_TEMPOK: u32 = 0x0800; /* TEMPOK */
pub const WM8991_MICSHRT: u32 = 0x0400; /* MICSHRT */
pub const WM8991_MICDET: u32 = 0x0200; /* MICDET */
pub const WM8991_PLL_LCK: u32 = 0x0100; /* PLL_LCK */
pub const WM8991_GPI8_STATUS: u32 = 0x0080; /* GPI8_STATUS */
pub const WM8991_GPI7_STATUS: u32 = 0x0040; /* GPI7_STATUS */
pub const WM8991_GPIO6_STATUS: u32 = 0x0020; /* GPIO6_STATUS */
pub const WM8991_GPIO5_STATUS: u32 = 0x0010; /* GPIO5_STATUS */
pub const WM8991_GPIO4_STATUS: u32 = 0x0008; /* GPIO4_STATUS */
pub const WM8991_GPIO3_STATUS: u32 = 0x0004; /* GPIO3_STATUS */
pub const WM8991_GPIO2_STATUS: u32 = 0x0002; /* GPIO2_STATUS */
pub const WM8991_GPIO1_STATUS: u32 = 0x0001; /* GPIO1_STATUS */

/*
 * R19 (0x13) - GPIO1 & GPIO2
 */
pub const WM8991_GPIO2_DEB_ENA: u32 = 0x8000; /* GPIO2_DEB_ENA */
pub const WM8991_GPIO2_IRQ_ENA: u32 = 0x4000; /* GPIO2_IRQ_ENA */
pub const WM8991_GPIO2_PU: u32 = 0x2000; /* GPIO2_PU */
pub const WM8991_GPIO2_PD: u32 = 0x1000; /* GPIO2_PD */
pub const WM8991_GPIO2_SEL_MASK: u32 = 0x0F00; /* GPIO2_SEL - [11:8] */
pub const WM8991_GPIO1_DEB_ENA: u32 = 0x0080; /* GPIO1_DEB_ENA */
pub const WM8991_GPIO1_IRQ_ENA: u32 = 0x0040; /* GPIO1_IRQ_ENA */
pub const WM8991_GPIO1_PU: u32 = 0x0020; /* GPIO1_PU */
pub const WM8991_GPIO1_PD: u32 = 0x0010; /* GPIO1_PD */
pub const WM8991_GPIO1_SEL_MASK: u32 = 0x000F; /* GPIO1_SEL - [3:0] */

/*
 * R20 (0x14) - GPIO3 & GPIO4
 */
pub const WM8991_GPIO4_DEB_ENA: u32 = 0x8000; /* GPIO4_DEB_ENA */
pub const WM8991_GPIO4_IRQ_ENA: u32 = 0x4000; /* GPIO4_IRQ_ENA */
pub const WM8991_GPIO4_PU: u32 = 0x2000; /* GPIO4_PU */
pub const WM8991_GPIO4_PD: u32 = 0x1000; /* GPIO4_PD */
pub const WM8991_GPIO4_SEL_MASK: u32 = 0x0F00; /* GPIO4_SEL - [11:8] */
pub const WM8991_GPIO3_DEB_ENA: u32 = 0x0080; /* GPIO3_DEB_ENA */
pub const WM8991_GPIO3_IRQ_ENA: u32 = 0x0040; /* GPIO3_IRQ_ENA */
pub const WM8991_GPIO3_PU: u32 = 0x0020; /* GPIO3_PU */
pub const WM8991_GPIO3_PD: u32 = 0x0010; /* GPIO3_PD */
pub const WM8991_GPIO3_SEL_MASK: u32 = 0x000F; /* GPIO3_SEL - [3:0] */

/*
 * R21 (0x15) - GPIO5 & GPIO6
 */
pub const WM8991_GPIO6_DEB_ENA: u32 = 0x8000; /* GPIO6_DEB_ENA */
pub const WM8991_GPIO6_IRQ_ENA: u32 = 0x4000; /* GPIO6_IRQ_ENA */
pub const WM8991_GPIO6_PU: u32 = 0x2000; /* GPIO6_PU */
pub const WM8991_GPIO6_PD: u32 = 0x1000; /* GPIO6_PD */
pub const WM8991_GPIO6_SEL_MASK: u32 = 0x0F00; /* GPIO6_SEL - [11:8] */
pub const WM8991_GPIO5_DEB_ENA: u32 = 0x0080; /* GPIO5_DEB_ENA */
pub const WM8991_GPIO5_IRQ_ENA: u32 = 0x0040; /* GPIO5_IRQ_ENA */
pub const WM8991_GPIO5_PU: u32 = 0x0020; /* GPIO5_PU */
pub const WM8991_GPIO5_PD: u32 = 0x0010; /* GPIO5_PD */
pub const WM8991_GPIO5_SEL_MASK: u32 = 0x000F; /* GPIO5_SEL - [3:0] */

/*
 * R22 (0x16) - GPIOCTRL 2
 */
pub const WM8991_RD_3W_ENA: u32 = 0x8000; /* RD_3W_ENA */
pub const WM8991_MODE_3W4W: u32 = 0x4000; /* MODE_3W4W */
pub const WM8991_TEMPOK_IRQ_ENA: u32 = 0x0800; /* TEMPOK_IRQ_ENA */
pub const WM8991_MICSHRT_IRQ_ENA: u32 = 0x0400; /* MICSHRT_IRQ_ENA */
pub const WM8991_MICDET_IRQ_ENA: u32 = 0x0200; /* MICDET_IRQ_ENA */
pub const WM8991_PLL_LCK_IRQ_ENA: u32 = 0x0100; /* PLL_LCK_IRQ_ENA */
pub const WM8991_GPI8_DEB_ENA: u32 = 0x0080; /* GPI8_DEB_ENA */
pub const WM8991_GPI8_IRQ_ENA: u32 = 0x0040; /* GPI8_IRQ_ENA */
pub const WM8991_GPI8_ENA: u32 = 0x0010; /* GPI8_ENA */
pub const WM8991_GPI7_DEB_ENA: u32 = 0x0008; /* GPI7_DEB_ENA */
pub const WM8991_GPI7_IRQ_ENA: u32 = 0x0004; /* GPI7_IRQ_ENA */
pub const WM8991_GPI7_ENA: u32 = 0x0001; /* GPI7_ENA */

/*
 * R23 (0x17) - GPIO_POL
 */
pub const WM8991_IRQ_INV: u32 = 0x1000; /* IRQ_INV */
pub const WM8991_TEMPOK_POL: u32 = 0x0800; /* TEMPOK_POL */
pub const WM8991_MICSHRT_POL: u32 = 0x0400; /* MICSHRT_POL */
pub const WM8991_MICDET_POL: u32 = 0x0200; /* MICDET_POL */
pub const WM8991_PLL_LCK_POL: u32 = 0x0100; /* PLL_LCK_POL */
pub const WM8991_GPI8_POL: u32 = 0x0080; /* GPI8_POL */
pub const WM8991_GPI7_POL: u32 = 0x0040; /* GPI7_POL */
pub const WM8991_GPIO6_POL: u32 = 0x0020; /* GPIO6_POL */
pub const WM8991_GPIO5_POL: u32 = 0x0010; /* GPIO5_POL */
pub const WM8991_GPIO4_POL: u32 = 0x0008; /* GPIO4_POL */
pub const WM8991_GPIO3_POL: u32 = 0x0004; /* GPIO3_POL */
pub const WM8991_GPIO2_POL: u32 = 0x0002; /* GPIO2_POL */
pub const WM8991_GPIO1_POL: u32 = 0x0001; /* GPIO1_POL */

/*
 * R24 (0x18) - Left Line Input 1&2 Volume
 */
pub const WM8991_IPVU: u32 = 0x0100; /* IPVU */
pub const WM8991_LI12MUTE: u32 = 0x0080; /* LI12MUTE */
pub const WM8991_LI12MUTE_BIT: u32 = 7;
pub const WM8991_LI12ZC: u32 = 0x0040; /* LI12ZC */
pub const WM8991_LI12ZC_BIT: u32 = 6;
pub const WM8991_LIN12VOL_MASK: u32 = 0x001F; /* LIN12VOL - [4:0] */
pub const WM8991_LIN12VOL_SHIFT: u32 = 0;
/*
 * R25 (0x19) - Left Line Input 3&4 Volume
 */
// Duplicate C macro: #define WM8991_IPVU 0x0100 /* IPVU */
pub const WM8991_LI34MUTE: u32 = 0x0080; /* LI34MUTE */
pub const WM8991_LI34MUTE_BIT: u32 = 7;
pub const WM8991_LI34ZC: u32 = 0x0040; /* LI34ZC */
pub const WM8991_LI34ZC_BIT: u32 = 6;
pub const WM8991_LIN34VOL_MASK: u32 = 0x001F; /* LIN34VOL - [4:0] */
pub const WM8991_LIN34VOL_SHIFT: u32 = 0;

/*
 * R26 (0x1A) - Right Line Input 1&2 Volume
 */
// Duplicate C macro: #define WM8991_IPVU 0x0100 /* IPVU */
pub const WM8991_RI12MUTE: u32 = 0x0080; /* RI12MUTE */
pub const WM8991_RI12MUTE_BIT: u32 = 7;
pub const WM8991_RI12ZC: u32 = 0x0040; /* RI12ZC */
pub const WM8991_RI12ZC_BIT: u32 = 6;
pub const WM8991_RIN12VOL_MASK: u32 = 0x001F; /* RIN12VOL - [4:0] */
pub const WM8991_RIN12VOL_SHIFT: u32 = 0;

/*
 * R27 (0x1B) - Right Line Input 3&4 Volume
 */
// Duplicate C macro: #define WM8991_IPVU 0x0100 /* IPVU */
pub const WM8991_RI34MUTE: u32 = 0x0080; /* RI34MUTE */
pub const WM8991_RI34MUTE_BIT: u32 = 7;
pub const WM8991_RI34ZC: u32 = 0x0040; /* RI34ZC */
pub const WM8991_RI34ZC_BIT: u32 = 6;
pub const WM8991_RIN34VOL_MASK: u32 = 0x001F; /* RIN34VOL - [4:0] */
pub const WM8991_RIN34VOL_SHIFT: u32 = 0;

/*
 * R28 (0x1C) - Left Output Volume
 */
pub const WM8991_OPVU: u32 = 0x0100; /* OPVU */
pub const WM8991_LOZC: u32 = 0x0080; /* LOZC */
pub const WM8991_LOZC_BIT: u32 = 7;
pub const WM8991_LOUTVOL_MASK: u32 = 0x007F; /* LOUTVOL - [6:0] */
pub const WM8991_LOUTVOL_SHIFT: u32 = 0;
/*
 * R29 (0x1D) - Right Output Volume
 */
// Duplicate C macro: #define WM8991_OPVU 0x0100 /* OPVU */
pub const WM8991_ROZC: u32 = 0x0080; /* ROZC */
pub const WM8991_ROZC_BIT: u32 = 7;
pub const WM8991_ROUTVOL_MASK: u32 = 0x007F; /* ROUTVOL - [6:0] */
pub const WM8991_ROUTVOL_SHIFT: u32 = 0;
/*
 * R30 (0x1E) - Line Outputs Volume
 */
pub const WM8991_LONMUTE: u32 = 0x0040; /* LONMUTE */
pub const WM8991_LONMUTE_BIT: u32 = 6;
pub const WM8991_LOPMUTE: u32 = 0x0020; /* LOPMUTE */
pub const WM8991_LOPMUTE_BIT: u32 = 5;
pub const WM8991_LOATTN: u32 = 0x0010; /* LOATTN */
pub const WM8991_LOATTN_BIT: u32 = 4;
pub const WM8991_RONMUTE: u32 = 0x0004; /* RONMUTE */
pub const WM8991_RONMUTE_BIT: u32 = 2;
pub const WM8991_ROPMUTE: u32 = 0x0002; /* ROPMUTE */
pub const WM8991_ROPMUTE_BIT: u32 = 1;
pub const WM8991_ROATTN: u32 = 0x0001; /* ROATTN */
pub const WM8991_ROATTN_BIT: u32 = 0;

/*
 * R31 (0x1F) - Out3/4 Volume
 */
pub const WM8991_OUT3MUTE: u32 = 0x0020; /* OUT3MUTE */
pub const WM8991_OUT3MUTE_BIT: u32 = 5;
pub const WM8991_OUT3ATTN: u32 = 0x0010; /* OUT3ATTN */
pub const WM8991_OUT3ATTN_BIT: u32 = 4;
pub const WM8991_OUT4MUTE: u32 = 0x0002; /* OUT4MUTE */
pub const WM8991_OUT4MUTE_BIT: u32 = 1;
pub const WM8991_OUT4ATTN: u32 = 0x0001; /* OUT4ATTN */
pub const WM8991_OUT4ATTN_BIT: u32 = 0;

/*
 * R32 (0x20) - Left OPGA Volume
 */
// Duplicate C macro: #define WM8991_OPVU 0x0100 /* OPVU */
pub const WM8991_LOPGAZC: u32 = 0x0080; /* LOPGAZC */
pub const WM8991_LOPGAZC_BIT: u32 = 7;
pub const WM8991_LOPGAVOL_MASK: u32 = 0x007F; /* LOPGAVOL - [6:0] */
pub const WM8991_LOPGAVOL_SHIFT: u32 = 0;

/*
 * R33 (0x21) - Right OPGA Volume
 */
// Duplicate C macro: #define WM8991_OPVU 0x0100 /* OPVU */
pub const WM8991_ROPGAZC: u32 = 0x0080; /* ROPGAZC */
pub const WM8991_ROPGAZC_BIT: u32 = 7;
pub const WM8991_ROPGAVOL_MASK: u32 = 0x007F; /* ROPGAVOL - [6:0] */
pub const WM8991_ROPGAVOL_SHIFT: u32 = 0;
/*
 * R34 (0x22) - Speaker Volume
 */
pub const WM8991_SPKVOL_MASK: u32 = 0x0003; /* SPKVOL - [1:0] */
pub const WM8991_SPKVOL_SHIFT: u32 = 0;

/*
 * R35 (0x23) - ClassD1
 */
pub const WM8991_CDMODE: u32 = 0x0100; /* CDMODE */
pub const WM8991_CDMODE_BIT: u32 = 8;

/*
 * R37 (0x25) - ClassD3
 */
pub const WM8991_DCGAIN_MASK: u32 = 0x0007; /* DCGAIN - [5:3] */
pub const WM8991_DCGAIN_SHIFT: u32 = 3;
pub const WM8991_ACGAIN_MASK: u32 = 0x0007; /* ACGAIN - [2:0] */
pub const WM8991_ACGAIN_SHIFT: u32 = 0;
/*
 * R39 (0x27) - Input Mixer1
 */
pub const WM8991_AINLMODE_MASK: u32 = 0x000C; /* AINLMODE - [3:2] */
pub const WM8991_AINLMODE_SHIFT: u32 = 2;
pub const WM8991_AINRMODE_MASK: u32 = 0x0003; /* AINRMODE - [1:0] */
pub const WM8991_AINRMODE_SHIFT: u32 = 0;

/*
 * R40 (0x28) - Input Mixer2
 */
pub const WM8991_LMP4: u32 = 0x0080; /* LMP4 */
pub const WM8991_LMP4_BIT: u32 = 7; /* LMP4 */
pub const WM8991_LMN3: u32 = 0x0040; /* LMN3 */
pub const WM8991_LMN3_BIT: u32 = 6; /* LMN3 */
pub const WM8991_LMP2: u32 = 0x0020; /* LMP2 */
pub const WM8991_LMP2_BIT: u32 = 5; /* LMP2 */
pub const WM8991_LMN1: u32 = 0x0010; /* LMN1 */
pub const WM8991_LMN1_BIT: u32 = 4; /* LMN1 */
pub const WM8991_RMP4: u32 = 0x0008; /* RMP4 */
pub const WM8991_RMP4_BIT: u32 = 3; /* RMP4 */
pub const WM8991_RMN3: u32 = 0x0004; /* RMN3 */
pub const WM8991_RMN3_BIT: u32 = 2; /* RMN3 */
pub const WM8991_RMP2: u32 = 0x0002; /* RMP2 */
pub const WM8991_RMP2_BIT: u32 = 1; /* RMP2 */
pub const WM8991_RMN1: u32 = 0x0001; /* RMN1 */
pub const WM8991_RMN1_BIT: u32 = 0; /* RMN1 */

/*
 * R41 (0x29) - Input Mixer3
 */
pub const WM8991_L34MNB: u32 = 0x0100; /* L34MNB */
pub const WM8991_L34MNB_BIT: u32 = 8;
pub const WM8991_L34MNBST: u32 = 0x0080; /* L34MNBST */
pub const WM8991_L34MNBST_BIT: u32 = 7;
pub const WM8991_L12MNB: u32 = 0x0020; /* L12MNB */
pub const WM8991_L12MNB_BIT: u32 = 5;
pub const WM8991_L12MNBST: u32 = 0x0010; /* L12MNBST */
pub const WM8991_L12MNBST_BIT: u32 = 4;
pub const WM8991_LDBVOL_MASK: u32 = 0x0007; /* LDBVOL - [2:0] */
pub const WM8991_LDBVOL_SHIFT: u32 = 0;

/*
 * R42 (0x2A) - Input Mixer4
 */
pub const WM8991_R34MNB: u32 = 0x0100; /* R34MNB */
pub const WM8991_R34MNB_BIT: u32 = 8;
pub const WM8991_R34MNBST: u32 = 0x0080; /* R34MNBST */
pub const WM8991_R34MNBST_BIT: u32 = 7;
pub const WM8991_R12MNB: u32 = 0x0020; /* R12MNB */
pub const WM8991_R12MNB_BIT: u32 = 5;
pub const WM8991_R12MNBST: u32 = 0x0010; /* R12MNBST */
pub const WM8991_R12MNBST_BIT: u32 = 4;
pub const WM8991_RDBVOL_MASK: u32 = 0x0007; /* RDBVOL - [2:0] */
pub const WM8991_RDBVOL_SHIFT: u32 = 0;

/*
 * R43 (0x2B) - Input Mixer5
 */
pub const WM8991_LI2BVOL_MASK: u32 = 0x07; /* LI2BVOL - [8:6] */
pub const WM8991_LI2BVOL_SHIFT: u32 = 6;
pub const WM8991_LR4BVOL_MASK: u32 = 0x07; /* LR4BVOL - [5:3] */
pub const WM8991_LR4BVOL_SHIFT: u32 = 3;
pub const WM8991_LL4BVOL_MASK: u32 = 0x07; /* LL4BVOL - [2:0] */
pub const WM8991_LL4BVOL_SHIFT: u32 = 0;

/*
 * R44 (0x2C) - Input Mixer6
 */
pub const WM8991_RI2BVOL_MASK: u32 = 0x07; /* RI2BVOL - [8:6] */
pub const WM8991_RI2BVOL_SHIFT: u32 = 6;
pub const WM8991_RL4BVOL_MASK: u32 = 0x07; /* RL4BVOL - [5:3] */
pub const WM8991_RL4BVOL_SHIFT: u32 = 3;
pub const WM8991_RR4BVOL_MASK: u32 = 0x07; /* RR4BVOL - [2:0] */
pub const WM8991_RR4BVOL_SHIFT: u32 = 0;

/*
 * R45 (0x2D) - Output Mixer1
 */
pub const WM8991_LRBLO: u32 = 0x0080; /* LRBLO */
pub const WM8991_LRBLO_BIT: u32 = 7;
pub const WM8991_LLBLO: u32 = 0x0040; /* LLBLO */
pub const WM8991_LLBLO_BIT: u32 = 6;
pub const WM8991_LRI3LO: u32 = 0x0020; /* LRI3LO */
pub const WM8991_LRI3LO_BIT: u32 = 5;
pub const WM8991_LLI3LO: u32 = 0x0010; /* LLI3LO */
pub const WM8991_LLI3LO_BIT: u32 = 4;
pub const WM8991_LR12LO: u32 = 0x0008; /* LR12LO */
pub const WM8991_LR12LO_BIT: u32 = 3;
pub const WM8991_LL12LO: u32 = 0x0004; /* LL12LO */
pub const WM8991_LL12LO_BIT: u32 = 2;
pub const WM8991_LDLO: u32 = 0x0001; /* LDLO */
pub const WM8991_LDLO_BIT: u32 = 0;

/*
 * R46 (0x2E) - Output Mixer2
 */
pub const WM8991_RLBRO: u32 = 0x0080; /* RLBRO */
pub const WM8991_RLBRO_BIT: u32 = 7;
pub const WM8991_RRBRO: u32 = 0x0040; /* RRBRO */
pub const WM8991_RRBRO_BIT: u32 = 6;
pub const WM8991_RLI3RO: u32 = 0x0020; /* RLI3RO */
pub const WM8991_RLI3RO_BIT: u32 = 5;
pub const WM8991_RRI3RO: u32 = 0x0010; /* RRI3RO */
pub const WM8991_RRI3RO_BIT: u32 = 4;
pub const WM8991_RL12RO: u32 = 0x0008; /* RL12RO */
pub const WM8991_RL12RO_BIT: u32 = 3;
pub const WM8991_RR12RO: u32 = 0x0004; /* RR12RO */
pub const WM8991_RR12RO_BIT: u32 = 2;
pub const WM8991_RDRO: u32 = 0x0001; /* RDRO */
pub const WM8991_RDRO_BIT: u32 = 0;

/*
 * R47 (0x2F) - Output Mixer3
 */
pub const WM8991_LLI3LOVOL_MASK: u32 = 0x07; /* LLI3LOVOL - [8:6] */
pub const WM8991_LLI3LOVOL_SHIFT: u32 = 6;
pub const WM8991_LR12LOVOL_MASK: u32 = 0x07; /* LR12LOVOL - [5:3] */
pub const WM8991_LR12LOVOL_SHIFT: u32 = 3;
pub const WM8991_LL12LOVOL_MASK: u32 = 0x07; /* LL12LOVOL - [2:0] */
pub const WM8991_LL12LOVOL_SHIFT: u32 = 0;

/*
 * R48 (0x30) - Output Mixer4
 */
pub const WM8991_RRI3ROVOL_MASK: u32 = 0x07; /* RRI3ROVOL - [8:6] */
pub const WM8991_RRI3ROVOL_SHIFT: u32 = 6;
pub const WM8991_RL12ROVOL_MASK: u32 = 0x07; /* RL12ROVOL - [5:3] */
pub const WM8991_RL12ROVOL_SHIFT: u32 = 3;
pub const WM8991_RR12ROVOL_MASK: u32 = 0x07; /* RR12ROVOL - [2:0] */
pub const WM8991_RR12ROVOL_SHIFT: u32 = 0;

/*
 * R49 (0x31) - Output Mixer5
 */
pub const WM8991_LRI3LOVOL_MASK: u32 = 0x07; /* LRI3LOVOL - [8:6] */
pub const WM8991_LRI3LOVOL_SHIFT: u32 = 6;
pub const WM8991_LRBLOVOL_MASK: u32 = 0x07; /* LRBLOVOL - [5:3] */
pub const WM8991_LRBLOVOL_SHIFT: u32 = 3;
pub const WM8991_LLBLOVOL_MASK: u32 = 0x07; /* LLBLOVOL - [2:0] */
pub const WM8991_LLBLOVOL_SHIFT: u32 = 0;

/*
 * R50 (0x32) - Output Mixer6
 */
pub const WM8991_RLI3ROVOL_MASK: u32 = 0x07; /* RLI3ROVOL - [8:6] */
pub const WM8991_RLI3ROVOL_SHIFT: u32 = 6;
pub const WM8991_RLBROVOL_MASK: u32 = 0x07; /* RLBROVOL - [5:3] */
pub const WM8991_RLBROVOL_SHIFT: u32 = 3;
pub const WM8991_RRBROVOL_MASK: u32 = 0x07; /* RRBROVOL - [2:0] */
pub const WM8991_RRBROVOL_SHIFT: u32 = 0;

/*
 * R51 (0x33) - Out3/4 Mixer
 */
pub const WM8991_VSEL_MASK: u32 = 0x0180; /* VSEL - [8:7] */
pub const WM8991_LI4O3: u32 = 0x0020; /* LI4O3 */
pub const WM8991_LI4O3_BIT: u32 = 5;
pub const WM8991_LPGAO3: u32 = 0x0010; /* LPGAO3 */
pub const WM8991_LPGAO3_BIT: u32 = 4;
pub const WM8991_RI4O4: u32 = 0x0002; /* RI4O4 */
pub const WM8991_RI4O4_BIT: u32 = 1;
pub const WM8991_RPGAO4: u32 = 0x0001; /* RPGAO4 */
pub const WM8991_RPGAO4_BIT: u32 = 0;
/*
 * R52 (0x34) - Line Mixer1
 */
pub const WM8991_LLOPGALON: u32 = 0x0040; /* LLOPGALON */
pub const WM8991_LLOPGALON_BIT: u32 = 6;
pub const WM8991_LROPGALON: u32 = 0x0020; /* LROPGALON */
pub const WM8991_LROPGALON_BIT: u32 = 5;
pub const WM8991_LOPLON: u32 = 0x0010; /* LOPLON */
pub const WM8991_LOPLON_BIT: u32 = 4;
pub const WM8991_LR12LOP: u32 = 0x0004; /* LR12LOP */
pub const WM8991_LR12LOP_BIT: u32 = 2;
pub const WM8991_LL12LOP: u32 = 0x0002; /* LL12LOP */
pub const WM8991_LL12LOP_BIT: u32 = 1;
pub const WM8991_LLOPGALOP: u32 = 0x0001; /* LLOPGALOP */
pub const WM8991_LLOPGALOP_BIT: u32 = 0;
/*
 * R53 (0x35) - Line Mixer2
 */
pub const WM8991_RROPGARON: u32 = 0x0040; /* RROPGARON */
pub const WM8991_RROPGARON_BIT: u32 = 6;
pub const WM8991_RLOPGARON: u32 = 0x0020; /* RLOPGARON */
pub const WM8991_RLOPGARON_BIT: u32 = 5;
pub const WM8991_ROPRON: u32 = 0x0010; /* ROPRON */
pub const WM8991_ROPRON_BIT: u32 = 4;
pub const WM8991_RL12ROP: u32 = 0x0004; /* RL12ROP */
pub const WM8991_RL12ROP_BIT: u32 = 2;
pub const WM8991_RR12ROP: u32 = 0x0002; /* RR12ROP */
pub const WM8991_RR12ROP_BIT: u32 = 1;
pub const WM8991_RROPGAROP: u32 = 0x0001; /* RROPGAROP */
pub const WM8991_RROPGAROP_BIT: u32 = 0;

/*
 * R54 (0x36) - Speaker Mixer
 */
pub const WM8991_LB2SPK: u32 = 0x0080; /* LB2SPK */
pub const WM8991_LB2SPK_BIT: u32 = 7;
pub const WM8991_RB2SPK: u32 = 0x0040; /* RB2SPK */
pub const WM8991_RB2SPK_BIT: u32 = 6;
pub const WM8991_LI2SPK: u32 = 0x0020; /* LI2SPK */
pub const WM8991_LI2SPK_BIT: u32 = 5;
pub const WM8991_RI2SPK: u32 = 0x0010; /* RI2SPK */
pub const WM8991_RI2SPK_BIT: u32 = 4;
pub const WM8991_LOPGASPK: u32 = 0x0008; /* LOPGASPK */
pub const WM8991_LOPGASPK_BIT: u32 = 3;
pub const WM8991_ROPGASPK: u32 = 0x0004; /* ROPGASPK */
pub const WM8991_ROPGASPK_BIT: u32 = 2;
pub const WM8991_LDSPK: u32 = 0x0002; /* LDSPK */
pub const WM8991_LDSPK_BIT: u32 = 1;
pub const WM8991_RDSPK: u32 = 0x0001; /* RDSPK */
pub const WM8991_RDSPK_BIT: u32 = 0;

/*
 * R55 (0x37) - Additional Control
 */
pub const WM8991_VROI: u32 = 0x0001; /* VROI */

/*
 * R56 (0x38) - AntiPOP1
 */
pub const WM8991_DIS_LLINE: u32 = 0x0020; /* DIS_LLINE */
pub const WM8991_DIS_RLINE: u32 = 0x0010; /* DIS_RLINE */
pub const WM8991_DIS_OUT3: u32 = 0x0008; /* DIS_OUT3 */
pub const WM8991_DIS_OUT4: u32 = 0x0004; /* DIS_OUT4 */
pub const WM8991_DIS_LOUT: u32 = 0x0002; /* DIS_LOUT */
pub const WM8991_DIS_ROUT: u32 = 0x0001; /* DIS_ROUT */

/*
 * R57 (0x39) - AntiPOP2
 */
pub const WM8991_SOFTST: u32 = 0x0040; /* SOFTST */
pub const WM8991_BUFIOEN: u32 = 0x0008; /* BUFIOEN */
pub const WM8991_BUFDCOPEN: u32 = 0x0004; /* BUFDCOPEN */
pub const WM8991_POBCTRL: u32 = 0x0002; /* POBCTRL */
pub const WM8991_VMIDTOG: u32 = 0x0001; /* VMIDTOG */

/*
 * R58 (0x3A) - MICBIAS
 */
pub const WM8991_MCDSCTH_MASK: u32 = 0x00C0; /* MCDSCTH - [7:6] */
pub const WM8991_MCDTHR_MASK: u32 = 0x0038; /* MCDTHR - [5:3] */
pub const WM8991_MCD: u32 = 0x0004; /* MCD */
pub const WM8991_MBSEL: u32 = 0x0001; /* MBSEL */

/*
 * R60 (0x3C) - PLL1
 */
pub const WM8991_SDM: u32 = 0x0080; /* SDM */
pub const WM8991_PRESCALE: u32 = 0x0040; /* PRESCALE */
pub const WM8991_PLLN_MASK: u32 = 0x000F; /* PLLN - [3:0] */

/*
 * R61 (0x3D) - PLL2
 */
pub const WM8991_PLLK1_MASK: u32 = 0x00FF; /* PLLK1 - [7:0] */

/*
 * R62 (0x3E) - PLL3
 */
pub const WM8991_PLLK2_MASK: u32 = 0x00FF; /* PLLK2 - [7:0] */

pub const WM8991_MCLK_DIV: u32 = 0;
pub const WM8991_DACCLK_DIV: u32 = 1;
pub const WM8991_ADCCLK_DIV: u32 = 2;
pub const WM8991_BCLK_DIV: u32 = 3;

macro_rules! SOC_WM899X_OUTPGA_SINGLE_R_TLV {
    ($xname:expr, $reg:expr, $shift:expr, $max:expr, $invert:expr, $tlv_array:expr) => {
        SOC_SINGLE_EXT_TLV!(
            $xname,
            $reg,
            $shift,
            $max,
            $invert,
            snd_soc_get_volsw,
            wm899x_outpga_put_volsw_vu,
            $tlv_array
        )
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
