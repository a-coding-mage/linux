/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * wm9081.c  --  WM9081 ALSA SoC Audio driver
 *
 * Author: Mark Brown
 *
 * Copyright 2009 Wolfson Microelectronics plc
 */

/* C header dependency: <sound/soc.h> */

/*
 * SYSCLK sources
 */
pub const WM9081_SYSCLK_MCLK: u32 = 1;  /* Use MCLK without FLL */
pub const WM9081_SYSCLK_FLL_MCLK: u32 = 2;  /* Use MCLK, enabling FLL if required */

/*
 * Register values.
 */
pub const WM9081_SOFTWARE_RESET: u32 = 0x00;
pub const WM9081_ANALOGUE_LINEOUT: u32 = 0x02;
pub const WM9081_ANALOGUE_SPEAKER_PGA: u32 = 0x03;
pub const WM9081_VMID_CONTROL: u32 = 0x04;
pub const WM9081_BIAS_CONTROL_1: u32 = 0x05;
pub const WM9081_ANALOGUE_MIXER: u32 = 0x07;
pub const WM9081_ANTI_POP_CONTROL: u32 = 0x08;
pub const WM9081_ANALOGUE_SPEAKER_1: u32 = 0x09;
pub const WM9081_ANALOGUE_SPEAKER_2: u32 = 0x0A;
pub const WM9081_POWER_MANAGEMENT: u32 = 0x0B;
pub const WM9081_CLOCK_CONTROL_1: u32 = 0x0C;
pub const WM9081_CLOCK_CONTROL_2: u32 = 0x0D;
pub const WM9081_CLOCK_CONTROL_3: u32 = 0x0E;
pub const WM9081_FLL_CONTROL_1: u32 = 0x10;
pub const WM9081_FLL_CONTROL_2: u32 = 0x11;
pub const WM9081_FLL_CONTROL_3: u32 = 0x12;
pub const WM9081_FLL_CONTROL_4: u32 = 0x13;
pub const WM9081_FLL_CONTROL_5: u32 = 0x14;
pub const WM9081_AUDIO_INTERFACE_1: u32 = 0x16;
pub const WM9081_AUDIO_INTERFACE_2: u32 = 0x17;
pub const WM9081_AUDIO_INTERFACE_3: u32 = 0x18;
pub const WM9081_AUDIO_INTERFACE_4: u32 = 0x19;
pub const WM9081_INTERRUPT_STATUS: u32 = 0x1A;
pub const WM9081_INTERRUPT_STATUS_MASK: u32 = 0x1B;
pub const WM9081_INTERRUPT_POLARITY: u32 = 0x1C;
pub const WM9081_INTERRUPT_CONTROL: u32 = 0x1D;
pub const WM9081_DAC_DIGITAL_1: u32 = 0x1E;
pub const WM9081_DAC_DIGITAL_2: u32 = 0x1F;
pub const WM9081_DRC_1: u32 = 0x20;
pub const WM9081_DRC_2: u32 = 0x21;
pub const WM9081_DRC_3: u32 = 0x22;
pub const WM9081_DRC_4: u32 = 0x23;
pub const WM9081_WRITE_SEQUENCER_1: u32 = 0x26;
pub const WM9081_WRITE_SEQUENCER_2: u32 = 0x27;
pub const WM9081_MW_SLAVE_1: u32 = 0x28;
pub const WM9081_EQ_1: u32 = 0x2A;
pub const WM9081_EQ_2: u32 = 0x2B;
pub const WM9081_EQ_3: u32 = 0x2C;
pub const WM9081_EQ_4: u32 = 0x2D;
pub const WM9081_EQ_5: u32 = 0x2E;
pub const WM9081_EQ_6: u32 = 0x2F;
pub const WM9081_EQ_7: u32 = 0x30;
pub const WM9081_EQ_8: u32 = 0x31;
pub const WM9081_EQ_9: u32 = 0x32;
pub const WM9081_EQ_10: u32 = 0x33;
pub const WM9081_EQ_11: u32 = 0x34;
pub const WM9081_EQ_12: u32 = 0x35;
pub const WM9081_EQ_13: u32 = 0x36;
pub const WM9081_EQ_14: u32 = 0x37;
pub const WM9081_EQ_15: u32 = 0x38;
pub const WM9081_EQ_16: u32 = 0x39;
pub const WM9081_EQ_17: u32 = 0x3A;
pub const WM9081_EQ_18: u32 = 0x3B;
pub const WM9081_EQ_19: u32 = 0x3C;
pub const WM9081_EQ_20: u32 = 0x3D;

pub const WM9081_REGISTER_COUNT: u32 = 55;
pub const WM9081_MAX_REGISTER: u32 = 0x3D;

/*
 * Field Definitions.
 */

/*
 * R0 (0x00) - Software Reset
 */
pub const WM9081_SW_RST_DEV_ID1_MASK: u32 = 0xFFFF;  /* SW_RST_DEV_ID1 - [15:0] */
pub const WM9081_SW_RST_DEV_ID1_SHIFT: u32 = 0;  /* SW_RST_DEV_ID1 - [15:0] */
pub const WM9081_SW_RST_DEV_ID1_WIDTH: u32 = 16;  /* SW_RST_DEV_ID1 - [15:0] */

/*
 * R2 (0x02) - Analogue Lineout
 */
pub const WM9081_LINEOUT_MUTE: u32 = 0x0080;  /* LINEOUT_MUTE */
pub const WM9081_LINEOUT_MUTE_MASK: u32 = 0x0080;  /* LINEOUT_MUTE */
pub const WM9081_LINEOUT_MUTE_SHIFT: u32 = 7;  /* LINEOUT_MUTE */
pub const WM9081_LINEOUT_MUTE_WIDTH: u32 = 1;  /* LINEOUT_MUTE */
pub const WM9081_LINEOUTZC: u32 = 0x0040;  /* LINEOUTZC */
pub const WM9081_LINEOUTZC_MASK: u32 = 0x0040;  /* LINEOUTZC */
pub const WM9081_LINEOUTZC_SHIFT: u32 = 6;  /* LINEOUTZC */
pub const WM9081_LINEOUTZC_WIDTH: u32 = 1;  /* LINEOUTZC */
pub const WM9081_LINEOUT_VOL_MASK: u32 = 0x003F;  /* LINEOUT_VOL - [5:0] */
pub const WM9081_LINEOUT_VOL_SHIFT: u32 = 0;  /* LINEOUT_VOL - [5:0] */
pub const WM9081_LINEOUT_VOL_WIDTH: u32 = 6;  /* LINEOUT_VOL - [5:0] */

/*
 * R3 (0x03) - Analogue Speaker PGA
 */
pub const WM9081_SPKPGA_MUTE: u32 = 0x0080;  /* SPKPGA_MUTE */
pub const WM9081_SPKPGA_MUTE_MASK: u32 = 0x0080;  /* SPKPGA_MUTE */
pub const WM9081_SPKPGA_MUTE_SHIFT: u32 = 7;  /* SPKPGA_MUTE */
pub const WM9081_SPKPGA_MUTE_WIDTH: u32 = 1;  /* SPKPGA_MUTE */
pub const WM9081_SPKPGAZC: u32 = 0x0040;  /* SPKPGAZC */
pub const WM9081_SPKPGAZC_MASK: u32 = 0x0040;  /* SPKPGAZC */
pub const WM9081_SPKPGAZC_SHIFT: u32 = 6;  /* SPKPGAZC */
pub const WM9081_SPKPGAZC_WIDTH: u32 = 1;  /* SPKPGAZC */
pub const WM9081_SPKPGA_VOL_MASK: u32 = 0x003F;  /* SPKPGA_VOL - [5:0] */
pub const WM9081_SPKPGA_VOL_SHIFT: u32 = 0;  /* SPKPGA_VOL - [5:0] */
pub const WM9081_SPKPGA_VOL_WIDTH: u32 = 6;  /* SPKPGA_VOL - [5:0] */

/*
 * R4 (0x04) - VMID Control
 */
pub const WM9081_VMID_BUF_ENA: u32 = 0x0020;  /* VMID_BUF_ENA */
pub const WM9081_VMID_BUF_ENA_MASK: u32 = 0x0020;  /* VMID_BUF_ENA */
pub const WM9081_VMID_BUF_ENA_SHIFT: u32 = 5;  /* VMID_BUF_ENA */
pub const WM9081_VMID_BUF_ENA_WIDTH: u32 = 1;  /* VMID_BUF_ENA */
pub const WM9081_VMID_RAMP: u32 = 0x0008;  /* VMID_RAMP */
pub const WM9081_VMID_RAMP_MASK: u32 = 0x0008;  /* VMID_RAMP */
pub const WM9081_VMID_RAMP_SHIFT: u32 = 3;  /* VMID_RAMP */
pub const WM9081_VMID_RAMP_WIDTH: u32 = 1;  /* VMID_RAMP */
pub const WM9081_VMID_SEL_MASK: u32 = 0x0006;  /* VMID_SEL - [2:1] */
pub const WM9081_VMID_SEL_SHIFT: u32 = 1;  /* VMID_SEL - [2:1] */
pub const WM9081_VMID_SEL_WIDTH: u32 = 2;  /* VMID_SEL - [2:1] */
pub const WM9081_VMID_FAST_ST: u32 = 0x0001;  /* VMID_FAST_ST */
pub const WM9081_VMID_FAST_ST_MASK: u32 = 0x0001;  /* VMID_FAST_ST */
pub const WM9081_VMID_FAST_ST_SHIFT: u32 = 0;  /* VMID_FAST_ST */
pub const WM9081_VMID_FAST_ST_WIDTH: u32 = 1;  /* VMID_FAST_ST */

/*
 * R5 (0x05) - Bias Control 1
 */
pub const WM9081_BIAS_SRC: u32 = 0x0040;  /* BIAS_SRC */
pub const WM9081_BIAS_SRC_MASK: u32 = 0x0040;  /* BIAS_SRC */
pub const WM9081_BIAS_SRC_SHIFT: u32 = 6;  /* BIAS_SRC */
pub const WM9081_BIAS_SRC_WIDTH: u32 = 1;  /* BIAS_SRC */
pub const WM9081_STBY_BIAS_LVL: u32 = 0x0020;  /* STBY_BIAS_LVL */
pub const WM9081_STBY_BIAS_LVL_MASK: u32 = 0x0020;  /* STBY_BIAS_LVL */
pub const WM9081_STBY_BIAS_LVL_SHIFT: u32 = 5;  /* STBY_BIAS_LVL */
pub const WM9081_STBY_BIAS_LVL_WIDTH: u32 = 1;  /* STBY_BIAS_LVL */
pub const WM9081_STBY_BIAS_ENA: u32 = 0x0010;  /* STBY_BIAS_ENA */
pub const WM9081_STBY_BIAS_ENA_MASK: u32 = 0x0010;  /* STBY_BIAS_ENA */
pub const WM9081_STBY_BIAS_ENA_SHIFT: u32 = 4;  /* STBY_BIAS_ENA */
pub const WM9081_STBY_BIAS_ENA_WIDTH: u32 = 1;  /* STBY_BIAS_ENA */
pub const WM9081_BIAS_LVL_MASK: u32 = 0x000C;  /* BIAS_LVL - [3:2] */
pub const WM9081_BIAS_LVL_SHIFT: u32 = 2;  /* BIAS_LVL - [3:2] */
pub const WM9081_BIAS_LVL_WIDTH: u32 = 2;  /* BIAS_LVL - [3:2] */
pub const WM9081_BIAS_ENA: u32 = 0x0002;  /* BIAS_ENA */
pub const WM9081_BIAS_ENA_MASK: u32 = 0x0002;  /* BIAS_ENA */
pub const WM9081_BIAS_ENA_SHIFT: u32 = 1;  /* BIAS_ENA */
pub const WM9081_BIAS_ENA_WIDTH: u32 = 1;  /* BIAS_ENA */
pub const WM9081_STARTUP_BIAS_ENA: u32 = 0x0001;  /* STARTUP_BIAS_ENA */
pub const WM9081_STARTUP_BIAS_ENA_MASK: u32 = 0x0001;  /* STARTUP_BIAS_ENA */
pub const WM9081_STARTUP_BIAS_ENA_SHIFT: u32 = 0;  /* STARTUP_BIAS_ENA */
pub const WM9081_STARTUP_BIAS_ENA_WIDTH: u32 = 1;  /* STARTUP_BIAS_ENA */

/*
 * R7 (0x07) - Analogue Mixer
 */
pub const WM9081_DAC_SEL: u32 = 0x0010;  /* DAC_SEL */
pub const WM9081_DAC_SEL_MASK: u32 = 0x0010;  /* DAC_SEL */
pub const WM9081_DAC_SEL_SHIFT: u32 = 4;  /* DAC_SEL */
pub const WM9081_DAC_SEL_WIDTH: u32 = 1;  /* DAC_SEL */
pub const WM9081_IN2_VOL: u32 = 0x0008;  /* IN2_VOL */
pub const WM9081_IN2_VOL_MASK: u32 = 0x0008;  /* IN2_VOL */
pub const WM9081_IN2_VOL_SHIFT: u32 = 3;  /* IN2_VOL */
pub const WM9081_IN2_VOL_WIDTH: u32 = 1;  /* IN2_VOL */
pub const WM9081_IN2_ENA: u32 = 0x0004;  /* IN2_ENA */
pub const WM9081_IN2_ENA_MASK: u32 = 0x0004;  /* IN2_ENA */
pub const WM9081_IN2_ENA_SHIFT: u32 = 2;  /* IN2_ENA */
pub const WM9081_IN2_ENA_WIDTH: u32 = 1;  /* IN2_ENA */
pub const WM9081_IN1_VOL: u32 = 0x0002;  /* IN1_VOL */
pub const WM9081_IN1_VOL_MASK: u32 = 0x0002;  /* IN1_VOL */
pub const WM9081_IN1_VOL_SHIFT: u32 = 1;  /* IN1_VOL */
pub const WM9081_IN1_VOL_WIDTH: u32 = 1;  /* IN1_VOL */
pub const WM9081_IN1_ENA: u32 = 0x0001;  /* IN1_ENA */
pub const WM9081_IN1_ENA_MASK: u32 = 0x0001;  /* IN1_ENA */
pub const WM9081_IN1_ENA_SHIFT: u32 = 0;  /* IN1_ENA */
pub const WM9081_IN1_ENA_WIDTH: u32 = 1;  /* IN1_ENA */

/*
 * R8 (0x08) - Anti Pop Control
 */
pub const WM9081_LINEOUT_DISCH: u32 = 0x0004;  /* LINEOUT_DISCH */
pub const WM9081_LINEOUT_DISCH_MASK: u32 = 0x0004;  /* LINEOUT_DISCH */
pub const WM9081_LINEOUT_DISCH_SHIFT: u32 = 2;  /* LINEOUT_DISCH */
pub const WM9081_LINEOUT_DISCH_WIDTH: u32 = 1;  /* LINEOUT_DISCH */
pub const WM9081_LINEOUT_VROI: u32 = 0x0002;  /* LINEOUT_VROI */
pub const WM9081_LINEOUT_VROI_MASK: u32 = 0x0002;  /* LINEOUT_VROI */
pub const WM9081_LINEOUT_VROI_SHIFT: u32 = 1;  /* LINEOUT_VROI */
pub const WM9081_LINEOUT_VROI_WIDTH: u32 = 1;  /* LINEOUT_VROI */
pub const WM9081_LINEOUT_CLAMP: u32 = 0x0001;  /* LINEOUT_CLAMP */
pub const WM9081_LINEOUT_CLAMP_MASK: u32 = 0x0001;  /* LINEOUT_CLAMP */
pub const WM9081_LINEOUT_CLAMP_SHIFT: u32 = 0;  /* LINEOUT_CLAMP */
pub const WM9081_LINEOUT_CLAMP_WIDTH: u32 = 1;  /* LINEOUT_CLAMP */

/*
 * R9 (0x09) - Analogue Speaker 1
 */
pub const WM9081_SPK_DCGAIN_MASK: u32 = 0x0038;  /* SPK_DCGAIN - [5:3] */
pub const WM9081_SPK_DCGAIN_SHIFT: u32 = 3;  /* SPK_DCGAIN - [5:3] */
pub const WM9081_SPK_DCGAIN_WIDTH: u32 = 3;  /* SPK_DCGAIN - [5:3] */
pub const WM9081_SPK_ACGAIN_MASK: u32 = 0x0007;  /* SPK_ACGAIN - [2:0] */
pub const WM9081_SPK_ACGAIN_SHIFT: u32 = 0;  /* SPK_ACGAIN - [2:0] */
pub const WM9081_SPK_ACGAIN_WIDTH: u32 = 3;  /* SPK_ACGAIN - [2:0] */

/*
 * R10 (0x0A) - Analogue Speaker 2
 */
pub const WM9081_SPK_MODE: u32 = 0x0040;  /* SPK_MODE */
pub const WM9081_SPK_MODE_MASK: u32 = 0x0040;  /* SPK_MODE */
pub const WM9081_SPK_MODE_SHIFT: u32 = 6;  /* SPK_MODE */
pub const WM9081_SPK_MODE_WIDTH: u32 = 1;  /* SPK_MODE */
pub const WM9081_SPK_INV_MUTE: u32 = 0x0010;  /* SPK_INV_MUTE */
pub const WM9081_SPK_INV_MUTE_MASK: u32 = 0x0010;  /* SPK_INV_MUTE */
pub const WM9081_SPK_INV_MUTE_SHIFT: u32 = 4;  /* SPK_INV_MUTE */
pub const WM9081_SPK_INV_MUTE_WIDTH: u32 = 1;  /* SPK_INV_MUTE */
pub const WM9081_OUT_SPK_CTRL: u32 = 0x0008;  /* OUT_SPK_CTRL */
pub const WM9081_OUT_SPK_CTRL_MASK: u32 = 0x0008;  /* OUT_SPK_CTRL */
pub const WM9081_OUT_SPK_CTRL_SHIFT: u32 = 3;  /* OUT_SPK_CTRL */
pub const WM9081_OUT_SPK_CTRL_WIDTH: u32 = 1;  /* OUT_SPK_CTRL */

/*
 * R11 (0x0B) - Power Management
 */
pub const WM9081_TSHUT_ENA: u32 = 0x0100;  /* TSHUT_ENA */
pub const WM9081_TSHUT_ENA_MASK: u32 = 0x0100;  /* TSHUT_ENA */
pub const WM9081_TSHUT_ENA_SHIFT: u32 = 8;  /* TSHUT_ENA */
pub const WM9081_TSHUT_ENA_WIDTH: u32 = 1;  /* TSHUT_ENA */
pub const WM9081_TSENSE_ENA: u32 = 0x0080;  /* TSENSE_ENA */
pub const WM9081_TSENSE_ENA_MASK: u32 = 0x0080;  /* TSENSE_ENA */
pub const WM9081_TSENSE_ENA_SHIFT: u32 = 7;  /* TSENSE_ENA */
pub const WM9081_TSENSE_ENA_WIDTH: u32 = 1;  /* TSENSE_ENA */
pub const WM9081_TEMP_SHUT: u32 = 0x0040;  /* TEMP_SHUT */
pub const WM9081_TEMP_SHUT_MASK: u32 = 0x0040;  /* TEMP_SHUT */
pub const WM9081_TEMP_SHUT_SHIFT: u32 = 6;  /* TEMP_SHUT */
pub const WM9081_TEMP_SHUT_WIDTH: u32 = 1;  /* TEMP_SHUT */
pub const WM9081_LINEOUT_ENA: u32 = 0x0010;  /* LINEOUT_ENA */
pub const WM9081_LINEOUT_ENA_MASK: u32 = 0x0010;  /* LINEOUT_ENA */
pub const WM9081_LINEOUT_ENA_SHIFT: u32 = 4;  /* LINEOUT_ENA */
pub const WM9081_LINEOUT_ENA_WIDTH: u32 = 1;  /* LINEOUT_ENA */
pub const WM9081_SPKPGA_ENA: u32 = 0x0004;  /* SPKPGA_ENA */
pub const WM9081_SPKPGA_ENA_MASK: u32 = 0x0004;  /* SPKPGA_ENA */
pub const WM9081_SPKPGA_ENA_SHIFT: u32 = 2;  /* SPKPGA_ENA */
pub const WM9081_SPKPGA_ENA_WIDTH: u32 = 1;  /* SPKPGA_ENA */
pub const WM9081_SPK_ENA: u32 = 0x0002;  /* SPK_ENA */
pub const WM9081_SPK_ENA_MASK: u32 = 0x0002;  /* SPK_ENA */
pub const WM9081_SPK_ENA_SHIFT: u32 = 1;  /* SPK_ENA */
pub const WM9081_SPK_ENA_WIDTH: u32 = 1;  /* SPK_ENA */
pub const WM9081_DAC_ENA: u32 = 0x0001;  /* DAC_ENA */
pub const WM9081_DAC_ENA_MASK: u32 = 0x0001;  /* DAC_ENA */
pub const WM9081_DAC_ENA_SHIFT: u32 = 0;  /* DAC_ENA */
pub const WM9081_DAC_ENA_WIDTH: u32 = 1;  /* DAC_ENA */

/*
 * R12 (0x0C) - Clock Control 1
 */
pub const WM9081_CLK_OP_DIV_MASK: u32 = 0x1C00;  /* CLK_OP_DIV - [12:10] */
pub const WM9081_CLK_OP_DIV_SHIFT: u32 = 10;  /* CLK_OP_DIV - [12:10] */
pub const WM9081_CLK_OP_DIV_WIDTH: u32 = 3;  /* CLK_OP_DIV - [12:10] */
pub const WM9081_CLK_TO_DIV_MASK: u32 = 0x0300;  /* CLK_TO_DIV - [9:8] */
pub const WM9081_CLK_TO_DIV_SHIFT: u32 = 8;  /* CLK_TO_DIV - [9:8] */
pub const WM9081_CLK_TO_DIV_WIDTH: u32 = 2;  /* CLK_TO_DIV - [9:8] */
pub const WM9081_MCLKDIV2: u32 = 0x0080;  /* MCLKDIV2 */
pub const WM9081_MCLKDIV2_MASK: u32 = 0x0080;  /* MCLKDIV2 */
pub const WM9081_MCLKDIV2_SHIFT: u32 = 7;  /* MCLKDIV2 */
pub const WM9081_MCLKDIV2_WIDTH: u32 = 1;  /* MCLKDIV2 */

/*
 * R13 (0x0D) - Clock Control 2
 */
pub const WM9081_CLK_SYS_RATE_MASK: u32 = 0x00F0;  /* CLK_SYS_RATE - [7:4] */
pub const WM9081_CLK_SYS_RATE_SHIFT: u32 = 4;  /* CLK_SYS_RATE - [7:4] */
pub const WM9081_CLK_SYS_RATE_WIDTH: u32 = 4;  /* CLK_SYS_RATE - [7:4] */
pub const WM9081_SAMPLE_RATE_MASK: u32 = 0x000F;  /* SAMPLE_RATE - [3:0] */
pub const WM9081_SAMPLE_RATE_SHIFT: u32 = 0;  /* SAMPLE_RATE - [3:0] */
pub const WM9081_SAMPLE_RATE_WIDTH: u32 = 4;  /* SAMPLE_RATE - [3:0] */

/*
 * R14 (0x0E) - Clock Control 3
 */
pub const WM9081_CLK_SRC_SEL: u32 = 0x2000;  /* CLK_SRC_SEL */
pub const WM9081_CLK_SRC_SEL_MASK: u32 = 0x2000;  /* CLK_SRC_SEL */
pub const WM9081_CLK_SRC_SEL_SHIFT: u32 = 13;  /* CLK_SRC_SEL */
pub const WM9081_CLK_SRC_SEL_WIDTH: u32 = 1;  /* CLK_SRC_SEL */
pub const WM9081_CLK_OP_ENA: u32 = 0x0020;  /* CLK_OP_ENA */
pub const WM9081_CLK_OP_ENA_MASK: u32 = 0x0020;  /* CLK_OP_ENA */
pub const WM9081_CLK_OP_ENA_SHIFT: u32 = 5;  /* CLK_OP_ENA */
pub const WM9081_CLK_OP_ENA_WIDTH: u32 = 1;  /* CLK_OP_ENA */
pub const WM9081_CLK_TO_ENA: u32 = 0x0004;  /* CLK_TO_ENA */
pub const WM9081_CLK_TO_ENA_MASK: u32 = 0x0004;  /* CLK_TO_ENA */
pub const WM9081_CLK_TO_ENA_SHIFT: u32 = 2;  /* CLK_TO_ENA */
pub const WM9081_CLK_TO_ENA_WIDTH: u32 = 1;  /* CLK_TO_ENA */
pub const WM9081_CLK_DSP_ENA: u32 = 0x0002;  /* CLK_DSP_ENA */
pub const WM9081_CLK_DSP_ENA_MASK: u32 = 0x0002;  /* CLK_DSP_ENA */
pub const WM9081_CLK_DSP_ENA_SHIFT: u32 = 1;  /* CLK_DSP_ENA */
pub const WM9081_CLK_DSP_ENA_WIDTH: u32 = 1;  /* CLK_DSP_ENA */
pub const WM9081_CLK_SYS_ENA: u32 = 0x0001;  /* CLK_SYS_ENA */
pub const WM9081_CLK_SYS_ENA_MASK: u32 = 0x0001;  /* CLK_SYS_ENA */
pub const WM9081_CLK_SYS_ENA_SHIFT: u32 = 0;  /* CLK_SYS_ENA */
pub const WM9081_CLK_SYS_ENA_WIDTH: u32 = 1;  /* CLK_SYS_ENA */

/*
 * R16 (0x10) - FLL Control 1
 */
pub const WM9081_FLL_HOLD: u32 = 0x0008;  /* FLL_HOLD */
pub const WM9081_FLL_HOLD_MASK: u32 = 0x0008;  /* FLL_HOLD */
pub const WM9081_FLL_HOLD_SHIFT: u32 = 3;  /* FLL_HOLD */
pub const WM9081_FLL_HOLD_WIDTH: u32 = 1;  /* FLL_HOLD */
pub const WM9081_FLL_FRAC: u32 = 0x0004;  /* FLL_FRAC */
pub const WM9081_FLL_FRAC_MASK: u32 = 0x0004;  /* FLL_FRAC */
pub const WM9081_FLL_FRAC_SHIFT: u32 = 2;  /* FLL_FRAC */
pub const WM9081_FLL_FRAC_WIDTH: u32 = 1;  /* FLL_FRAC */
pub const WM9081_FLL_ENA: u32 = 0x0001;  /* FLL_ENA */
pub const WM9081_FLL_ENA_MASK: u32 = 0x0001;  /* FLL_ENA */
pub const WM9081_FLL_ENA_SHIFT: u32 = 0;  /* FLL_ENA */
pub const WM9081_FLL_ENA_WIDTH: u32 = 1;  /* FLL_ENA */

/*
 * R17 (0x11) - FLL Control 2
 */
pub const WM9081_FLL_OUTDIV_MASK: u32 = 0x0700;  /* FLL_OUTDIV - [10:8] */
pub const WM9081_FLL_OUTDIV_SHIFT: u32 = 8;  /* FLL_OUTDIV - [10:8] */
pub const WM9081_FLL_OUTDIV_WIDTH: u32 = 3;  /* FLL_OUTDIV - [10:8] */
pub const WM9081_FLL_CTRL_RATE_MASK: u32 = 0x0070;  /* FLL_CTRL_RATE - [6:4] */
pub const WM9081_FLL_CTRL_RATE_SHIFT: u32 = 4;  /* FLL_CTRL_RATE - [6:4] */
pub const WM9081_FLL_CTRL_RATE_WIDTH: u32 = 3;  /* FLL_CTRL_RATE - [6:4] */
pub const WM9081_FLL_FRATIO_MASK: u32 = 0x0007;  /* FLL_FRATIO - [2:0] */
pub const WM9081_FLL_FRATIO_SHIFT: u32 = 0;  /* FLL_FRATIO - [2:0] */
pub const WM9081_FLL_FRATIO_WIDTH: u32 = 3;  /* FLL_FRATIO - [2:0] */

/*
 * R18 (0x12) - FLL Control 3
 */
pub const WM9081_FLL_K_MASK: u32 = 0xFFFF;  /* FLL_K - [15:0] */
pub const WM9081_FLL_K_SHIFT: u32 = 0;  /* FLL_K - [15:0] */
pub const WM9081_FLL_K_WIDTH: u32 = 16;  /* FLL_K - [15:0] */

/*
 * R19 (0x13) - FLL Control 4
 */
pub const WM9081_FLL_N_MASK: u32 = 0x7FE0;  /* FLL_N - [14:5] */
pub const WM9081_FLL_N_SHIFT: u32 = 5;  /* FLL_N - [14:5] */
pub const WM9081_FLL_N_WIDTH: u32 = 10;  /* FLL_N - [14:5] */
pub const WM9081_FLL_GAIN_MASK: u32 = 0x000F;  /* FLL_GAIN - [3:0] */
pub const WM9081_FLL_GAIN_SHIFT: u32 = 0;  /* FLL_GAIN - [3:0] */
pub const WM9081_FLL_GAIN_WIDTH: u32 = 4;  /* FLL_GAIN - [3:0] */

/*
 * R20 (0x14) - FLL Control 5
 */
pub const WM9081_FLL_CLK_REF_DIV_MASK: u32 = 0x0018;  /* FLL_CLK_REF_DIV - [4:3] */
pub const WM9081_FLL_CLK_REF_DIV_SHIFT: u32 = 3;  /* FLL_CLK_REF_DIV - [4:3] */
pub const WM9081_FLL_CLK_REF_DIV_WIDTH: u32 = 2;  /* FLL_CLK_REF_DIV - [4:3] */
pub const WM9081_FLL_CLK_SRC_MASK: u32 = 0x0003;  /* FLL_CLK_SRC - [1:0] */
pub const WM9081_FLL_CLK_SRC_SHIFT: u32 = 0;  /* FLL_CLK_SRC - [1:0] */
pub const WM9081_FLL_CLK_SRC_WIDTH: u32 = 2;  /* FLL_CLK_SRC - [1:0] */

/*
 * R22 (0x16) - Audio Interface 1
 */
pub const WM9081_AIFDAC_CHAN: u32 = 0x0040;  /* AIFDAC_CHAN */
pub const WM9081_AIFDAC_CHAN_MASK: u32 = 0x0040;  /* AIFDAC_CHAN */
pub const WM9081_AIFDAC_CHAN_SHIFT: u32 = 6;  /* AIFDAC_CHAN */
pub const WM9081_AIFDAC_CHAN_WIDTH: u32 = 1;  /* AIFDAC_CHAN */
pub const WM9081_AIFDAC_TDM_SLOT_MASK: u32 = 0x0030;  /* AIFDAC_TDM_SLOT - [5:4] */
pub const WM9081_AIFDAC_TDM_SLOT_SHIFT: u32 = 4;  /* AIFDAC_TDM_SLOT - [5:4] */
pub const WM9081_AIFDAC_TDM_SLOT_WIDTH: u32 = 2;  /* AIFDAC_TDM_SLOT - [5:4] */
pub const WM9081_AIFDAC_TDM_MODE_MASK: u32 = 0x000C;  /* AIFDAC_TDM_MODE - [3:2] */
pub const WM9081_AIFDAC_TDM_MODE_SHIFT: u32 = 2;  /* AIFDAC_TDM_MODE - [3:2] */
pub const WM9081_AIFDAC_TDM_MODE_WIDTH: u32 = 2;  /* AIFDAC_TDM_MODE - [3:2] */
pub const WM9081_DAC_COMP: u32 = 0x0002;  /* DAC_COMP */
pub const WM9081_DAC_COMP_MASK: u32 = 0x0002;  /* DAC_COMP */
pub const WM9081_DAC_COMP_SHIFT: u32 = 1;  /* DAC_COMP */
pub const WM9081_DAC_COMP_WIDTH: u32 = 1;  /* DAC_COMP */
pub const WM9081_DAC_COMPMODE: u32 = 0x0001;  /* DAC_COMPMODE */
pub const WM9081_DAC_COMPMODE_MASK: u32 = 0x0001;  /* DAC_COMPMODE */
pub const WM9081_DAC_COMPMODE_SHIFT: u32 = 0;  /* DAC_COMPMODE */
pub const WM9081_DAC_COMPMODE_WIDTH: u32 = 1;  /* DAC_COMPMODE */

/*
 * R23 (0x17) - Audio Interface 2
 */
pub const WM9081_AIF_TRIS: u32 = 0x0200;  /* AIF_TRIS */
pub const WM9081_AIF_TRIS_MASK: u32 = 0x0200;  /* AIF_TRIS */
pub const WM9081_AIF_TRIS_SHIFT: u32 = 9;  /* AIF_TRIS */
pub const WM9081_AIF_TRIS_WIDTH: u32 = 1;  /* AIF_TRIS */
pub const WM9081_DAC_DAT_INV: u32 = 0x0100;  /* DAC_DAT_INV */
pub const WM9081_DAC_DAT_INV_MASK: u32 = 0x0100;  /* DAC_DAT_INV */
pub const WM9081_DAC_DAT_INV_SHIFT: u32 = 8;  /* DAC_DAT_INV */
pub const WM9081_DAC_DAT_INV_WIDTH: u32 = 1;  /* DAC_DAT_INV */
pub const WM9081_AIF_BCLK_INV: u32 = 0x0080;  /* AIF_BCLK_INV */
pub const WM9081_AIF_BCLK_INV_MASK: u32 = 0x0080;  /* AIF_BCLK_INV */
pub const WM9081_AIF_BCLK_INV_SHIFT: u32 = 7;  /* AIF_BCLK_INV */
pub const WM9081_AIF_BCLK_INV_WIDTH: u32 = 1;  /* AIF_BCLK_INV */
pub const WM9081_BCLK_DIR: u32 = 0x0040;  /* BCLK_DIR */
pub const WM9081_BCLK_DIR_MASK: u32 = 0x0040;  /* BCLK_DIR */
pub const WM9081_BCLK_DIR_SHIFT: u32 = 6;  /* BCLK_DIR */
pub const WM9081_BCLK_DIR_WIDTH: u32 = 1;  /* BCLK_DIR */
pub const WM9081_LRCLK_DIR: u32 = 0x0020;  /* LRCLK_DIR */
pub const WM9081_LRCLK_DIR_MASK: u32 = 0x0020;  /* LRCLK_DIR */
pub const WM9081_LRCLK_DIR_SHIFT: u32 = 5;  /* LRCLK_DIR */
pub const WM9081_LRCLK_DIR_WIDTH: u32 = 1;  /* LRCLK_DIR */
pub const WM9081_AIF_LRCLK_INV: u32 = 0x0010;  /* AIF_LRCLK_INV */
pub const WM9081_AIF_LRCLK_INV_MASK: u32 = 0x0010;  /* AIF_LRCLK_INV */
pub const WM9081_AIF_LRCLK_INV_SHIFT: u32 = 4;  /* AIF_LRCLK_INV */
pub const WM9081_AIF_LRCLK_INV_WIDTH: u32 = 1;  /* AIF_LRCLK_INV */
pub const WM9081_AIF_WL_MASK: u32 = 0x000C;  /* AIF_WL - [3:2] */
pub const WM9081_AIF_WL_SHIFT: u32 = 2;  /* AIF_WL - [3:2] */
pub const WM9081_AIF_WL_WIDTH: u32 = 2;  /* AIF_WL - [3:2] */
pub const WM9081_AIF_FMT_MASK: u32 = 0x0003;  /* AIF_FMT - [1:0] */
pub const WM9081_AIF_FMT_SHIFT: u32 = 0;  /* AIF_FMT - [1:0] */
pub const WM9081_AIF_FMT_WIDTH: u32 = 2;  /* AIF_FMT - [1:0] */

/*
 * R24 (0x18) - Audio Interface 3
 */
pub const WM9081_BCLK_DIV_MASK: u32 = 0x001F;  /* BCLK_DIV - [4:0] */
pub const WM9081_BCLK_DIV_SHIFT: u32 = 0;  /* BCLK_DIV - [4:0] */
pub const WM9081_BCLK_DIV_WIDTH: u32 = 5;  /* BCLK_DIV - [4:0] */

/*
 * R25 (0x19) - Audio Interface 4
 */
pub const WM9081_LRCLK_RATE_MASK: u32 = 0x07FF;  /* LRCLK_RATE - [10:0] */
pub const WM9081_LRCLK_RATE_SHIFT: u32 = 0;  /* LRCLK_RATE - [10:0] */
pub const WM9081_LRCLK_RATE_WIDTH: u32 = 11;  /* LRCLK_RATE - [10:0] */

/*
 * R26 (0x1A) - Interrupt Status
 */
pub const WM9081_WSEQ_BUSY_EINT: u32 = 0x0004;  /* WSEQ_BUSY_EINT */
pub const WM9081_WSEQ_BUSY_EINT_MASK: u32 = 0x0004;  /* WSEQ_BUSY_EINT */
pub const WM9081_WSEQ_BUSY_EINT_SHIFT: u32 = 2;  /* WSEQ_BUSY_EINT */
pub const WM9081_WSEQ_BUSY_EINT_WIDTH: u32 = 1;  /* WSEQ_BUSY_EINT */
pub const WM9081_TSHUT_EINT: u32 = 0x0001;  /* TSHUT_EINT */
pub const WM9081_TSHUT_EINT_MASK: u32 = 0x0001;  /* TSHUT_EINT */
pub const WM9081_TSHUT_EINT_SHIFT: u32 = 0;  /* TSHUT_EINT */
pub const WM9081_TSHUT_EINT_WIDTH: u32 = 1;  /* TSHUT_EINT */

/*
 * R27 (0x1B) - Interrupt Status Mask
 */
pub const WM9081_IM_WSEQ_BUSY_EINT: u32 = 0x0004;  /* IM_WSEQ_BUSY_EINT */
pub const WM9081_IM_WSEQ_BUSY_EINT_MASK: u32 = 0x0004;  /* IM_WSEQ_BUSY_EINT */
pub const WM9081_IM_WSEQ_BUSY_EINT_SHIFT: u32 = 2;  /* IM_WSEQ_BUSY_EINT */
pub const WM9081_IM_WSEQ_BUSY_EINT_WIDTH: u32 = 1;  /* IM_WSEQ_BUSY_EINT */
pub const WM9081_IM_TSHUT_EINT: u32 = 0x0001;  /* IM_TSHUT_EINT */
pub const WM9081_IM_TSHUT_EINT_MASK: u32 = 0x0001;  /* IM_TSHUT_EINT */
pub const WM9081_IM_TSHUT_EINT_SHIFT: u32 = 0;  /* IM_TSHUT_EINT */
pub const WM9081_IM_TSHUT_EINT_WIDTH: u32 = 1;  /* IM_TSHUT_EINT */

/*
 * R28 (0x1C) - Interrupt Polarity
 */
pub const WM9081_TSHUT_INV: u32 = 0x0001;  /* TSHUT_INV */
pub const WM9081_TSHUT_INV_MASK: u32 = 0x0001;  /* TSHUT_INV */
pub const WM9081_TSHUT_INV_SHIFT: u32 = 0;  /* TSHUT_INV */
pub const WM9081_TSHUT_INV_WIDTH: u32 = 1;  /* TSHUT_INV */

/*
 * R29 (0x1D) - Interrupt Control
 */
pub const WM9081_IRQ_POL: u32 = 0x8000;  /* IRQ_POL */
pub const WM9081_IRQ_POL_MASK: u32 = 0x8000;  /* IRQ_POL */
pub const WM9081_IRQ_POL_SHIFT: u32 = 15;  /* IRQ_POL */
pub const WM9081_IRQ_POL_WIDTH: u32 = 1;  /* IRQ_POL */
pub const WM9081_IRQ_OP_CTRL: u32 = 0x0001;  /* IRQ_OP_CTRL */
pub const WM9081_IRQ_OP_CTRL_MASK: u32 = 0x0001;  /* IRQ_OP_CTRL */
pub const WM9081_IRQ_OP_CTRL_SHIFT: u32 = 0;  /* IRQ_OP_CTRL */
pub const WM9081_IRQ_OP_CTRL_WIDTH: u32 = 1;  /* IRQ_OP_CTRL */

/*
 * R30 (0x1E) - DAC Digital 1
 */
pub const WM9081_DAC_VOL_MASK: u32 = 0x00FF;  /* DAC_VOL - [7:0] */
pub const WM9081_DAC_VOL_SHIFT: u32 = 0;  /* DAC_VOL - [7:0] */
pub const WM9081_DAC_VOL_WIDTH: u32 = 8;  /* DAC_VOL - [7:0] */

/*
 * R31 (0x1F) - DAC Digital 2
 */
pub const WM9081_DAC_MUTERATE: u32 = 0x0400;  /* DAC_MUTERATE */
pub const WM9081_DAC_MUTERATE_MASK: u32 = 0x0400;  /* DAC_MUTERATE */
pub const WM9081_DAC_MUTERATE_SHIFT: u32 = 10;  /* DAC_MUTERATE */
pub const WM9081_DAC_MUTERATE_WIDTH: u32 = 1;  /* DAC_MUTERATE */
pub const WM9081_DAC_MUTEMODE: u32 = 0x0200;  /* DAC_MUTEMODE */
pub const WM9081_DAC_MUTEMODE_MASK: u32 = 0x0200;  /* DAC_MUTEMODE */
pub const WM9081_DAC_MUTEMODE_SHIFT: u32 = 9;  /* DAC_MUTEMODE */
pub const WM9081_DAC_MUTEMODE_WIDTH: u32 = 1;  /* DAC_MUTEMODE */
pub const WM9081_DAC_MUTE: u32 = 0x0008;  /* DAC_MUTE */
pub const WM9081_DAC_MUTE_MASK: u32 = 0x0008;  /* DAC_MUTE */
pub const WM9081_DAC_MUTE_SHIFT: u32 = 3;  /* DAC_MUTE */
pub const WM9081_DAC_MUTE_WIDTH: u32 = 1;  /* DAC_MUTE */
pub const WM9081_DEEMPH_MASK: u32 = 0x0006;  /* DEEMPH - [2:1] */
pub const WM9081_DEEMPH_SHIFT: u32 = 1;  /* DEEMPH - [2:1] */
pub const WM9081_DEEMPH_WIDTH: u32 = 2;  /* DEEMPH - [2:1] */

/*
 * R32 (0x20) - DRC 1
 */
pub const WM9081_DRC_ENA: u32 = 0x8000;  /* DRC_ENA */
pub const WM9081_DRC_ENA_MASK: u32 = 0x8000;  /* DRC_ENA */
pub const WM9081_DRC_ENA_SHIFT: u32 = 15;  /* DRC_ENA */
pub const WM9081_DRC_ENA_WIDTH: u32 = 1;  /* DRC_ENA */
pub const WM9081_DRC_STARTUP_GAIN_MASK: u32 = 0x07C0;  /* DRC_STARTUP_GAIN - [10:6] */
pub const WM9081_DRC_STARTUP_GAIN_SHIFT: u32 = 6;  /* DRC_STARTUP_GAIN - [10:6] */
pub const WM9081_DRC_STARTUP_GAIN_WIDTH: u32 = 5;  /* DRC_STARTUP_GAIN - [10:6] */
pub const WM9081_DRC_FF_DLY: u32 = 0x0020;  /* DRC_FF_DLY */
pub const WM9081_DRC_FF_DLY_MASK: u32 = 0x0020;  /* DRC_FF_DLY */
pub const WM9081_DRC_FF_DLY_SHIFT: u32 = 5;  /* DRC_FF_DLY */
pub const WM9081_DRC_FF_DLY_WIDTH: u32 = 1;  /* DRC_FF_DLY */
pub const WM9081_DRC_QR: u32 = 0x0004;  /* DRC_QR */
pub const WM9081_DRC_QR_MASK: u32 = 0x0004;  /* DRC_QR */
pub const WM9081_DRC_QR_SHIFT: u32 = 2;  /* DRC_QR */
pub const WM9081_DRC_QR_WIDTH: u32 = 1;  /* DRC_QR */
pub const WM9081_DRC_ANTICLIP: u32 = 0x0002;  /* DRC_ANTICLIP */
pub const WM9081_DRC_ANTICLIP_MASK: u32 = 0x0002;  /* DRC_ANTICLIP */
pub const WM9081_DRC_ANTICLIP_SHIFT: u32 = 1;  /* DRC_ANTICLIP */
pub const WM9081_DRC_ANTICLIP_WIDTH: u32 = 1;  /* DRC_ANTICLIP */

/*
 * R33 (0x21) - DRC 2
 */
pub const WM9081_DRC_ATK_MASK: u32 = 0xF000;  /* DRC_ATK - [15:12] */
pub const WM9081_DRC_ATK_SHIFT: u32 = 12;  /* DRC_ATK - [15:12] */
pub const WM9081_DRC_ATK_WIDTH: u32 = 4;  /* DRC_ATK - [15:12] */
pub const WM9081_DRC_DCY_MASK: u32 = 0x0F00;  /* DRC_DCY - [11:8] */
pub const WM9081_DRC_DCY_SHIFT: u32 = 8;  /* DRC_DCY - [11:8] */
pub const WM9081_DRC_DCY_WIDTH: u32 = 4;  /* DRC_DCY - [11:8] */
pub const WM9081_DRC_QR_THR_MASK: u32 = 0x00C0;  /* DRC_QR_THR - [7:6] */
pub const WM9081_DRC_QR_THR_SHIFT: u32 = 6;  /* DRC_QR_THR - [7:6] */
pub const WM9081_DRC_QR_THR_WIDTH: u32 = 2;  /* DRC_QR_THR - [7:6] */
pub const WM9081_DRC_QR_DCY_MASK: u32 = 0x0030;  /* DRC_QR_DCY - [5:4] */
pub const WM9081_DRC_QR_DCY_SHIFT: u32 = 4;  /* DRC_QR_DCY - [5:4] */
pub const WM9081_DRC_QR_DCY_WIDTH: u32 = 2;  /* DRC_QR_DCY - [5:4] */
pub const WM9081_DRC_MINGAIN_MASK: u32 = 0x000C;  /* DRC_MINGAIN - [3:2] */
pub const WM9081_DRC_MINGAIN_SHIFT: u32 = 2;  /* DRC_MINGAIN - [3:2] */
pub const WM9081_DRC_MINGAIN_WIDTH: u32 = 2;  /* DRC_MINGAIN - [3:2] */
pub const WM9081_DRC_MAXGAIN_MASK: u32 = 0x0003;  /* DRC_MAXGAIN - [1:0] */
pub const WM9081_DRC_MAXGAIN_SHIFT: u32 = 0;  /* DRC_MAXGAIN - [1:0] */
pub const WM9081_DRC_MAXGAIN_WIDTH: u32 = 2;  /* DRC_MAXGAIN - [1:0] */

/*
 * R34 (0x22) - DRC 3
 */
pub const WM9081_DRC_HI_COMP_MASK: u32 = 0x0038;  /* DRC_HI_COMP - [5:3] */
pub const WM9081_DRC_HI_COMP_SHIFT: u32 = 3;  /* DRC_HI_COMP - [5:3] */
pub const WM9081_DRC_HI_COMP_WIDTH: u32 = 3;  /* DRC_HI_COMP - [5:3] */
pub const WM9081_DRC_LO_COMP_MASK: u32 = 0x0007;  /* DRC_LO_COMP - [2:0] */
pub const WM9081_DRC_LO_COMP_SHIFT: u32 = 0;  /* DRC_LO_COMP - [2:0] */
pub const WM9081_DRC_LO_COMP_WIDTH: u32 = 3;  /* DRC_LO_COMP - [2:0] */

/*
 * R35 (0x23) - DRC 4
 */
pub const WM9081_DRC_KNEE_IP_MASK: u32 = 0x07E0;  /* DRC_KNEE_IP - [10:5] */
pub const WM9081_DRC_KNEE_IP_SHIFT: u32 = 5;  /* DRC_KNEE_IP - [10:5] */
pub const WM9081_DRC_KNEE_IP_WIDTH: u32 = 6;  /* DRC_KNEE_IP - [10:5] */
pub const WM9081_DRC_KNEE_OP_MASK: u32 = 0x001F;  /* DRC_KNEE_OP - [4:0] */
pub const WM9081_DRC_KNEE_OP_SHIFT: u32 = 0;  /* DRC_KNEE_OP - [4:0] */
pub const WM9081_DRC_KNEE_OP_WIDTH: u32 = 5;  /* DRC_KNEE_OP - [4:0] */

/*
 * R38 (0x26) - Write Sequencer 1
 */
pub const WM9081_WSEQ_ENA: u32 = 0x8000;  /* WSEQ_ENA */
pub const WM9081_WSEQ_ENA_MASK: u32 = 0x8000;  /* WSEQ_ENA */
pub const WM9081_WSEQ_ENA_SHIFT: u32 = 15;  /* WSEQ_ENA */
pub const WM9081_WSEQ_ENA_WIDTH: u32 = 1;  /* WSEQ_ENA */
pub const WM9081_WSEQ_ABORT: u32 = 0x0200;  /* WSEQ_ABORT */
pub const WM9081_WSEQ_ABORT_MASK: u32 = 0x0200;  /* WSEQ_ABORT */
pub const WM9081_WSEQ_ABORT_SHIFT: u32 = 9;  /* WSEQ_ABORT */
pub const WM9081_WSEQ_ABORT_WIDTH: u32 = 1;  /* WSEQ_ABORT */
pub const WM9081_WSEQ_START: u32 = 0x0100;  /* WSEQ_START */
pub const WM9081_WSEQ_START_MASK: u32 = 0x0100;  /* WSEQ_START */
pub const WM9081_WSEQ_START_SHIFT: u32 = 8;  /* WSEQ_START */
pub const WM9081_WSEQ_START_WIDTH: u32 = 1;  /* WSEQ_START */
pub const WM9081_WSEQ_START_INDEX_MASK: u32 = 0x007F;  /* WSEQ_START_INDEX - [6:0] */
pub const WM9081_WSEQ_START_INDEX_SHIFT: u32 = 0;  /* WSEQ_START_INDEX - [6:0] */
pub const WM9081_WSEQ_START_INDEX_WIDTH: u32 = 7;  /* WSEQ_START_INDEX - [6:0] */

/*
 * R39 (0x27) - Write Sequencer 2
 */
pub const WM9081_WSEQ_CURRENT_INDEX_MASK: u32 = 0x07F0;  /* WSEQ_CURRENT_INDEX - [10:4] */
pub const WM9081_WSEQ_CURRENT_INDEX_SHIFT: u32 = 4;  /* WSEQ_CURRENT_INDEX - [10:4] */
pub const WM9081_WSEQ_CURRENT_INDEX_WIDTH: u32 = 7;  /* WSEQ_CURRENT_INDEX - [10:4] */
pub const WM9081_WSEQ_BUSY: u32 = 0x0001;  /* WSEQ_BUSY */
pub const WM9081_WSEQ_BUSY_MASK: u32 = 0x0001;  /* WSEQ_BUSY */
pub const WM9081_WSEQ_BUSY_SHIFT: u32 = 0;  /* WSEQ_BUSY */
pub const WM9081_WSEQ_BUSY_WIDTH: u32 = 1;  /* WSEQ_BUSY */

/*
 * R40 (0x28) - MW Slave 1
 */
pub const WM9081_SPI_CFG: u32 = 0x0020;  /* SPI_CFG */
pub const WM9081_SPI_CFG_MASK: u32 = 0x0020;  /* SPI_CFG */
pub const WM9081_SPI_CFG_SHIFT: u32 = 5;  /* SPI_CFG */
pub const WM9081_SPI_CFG_WIDTH: u32 = 1;  /* SPI_CFG */
pub const WM9081_SPI_4WIRE: u32 = 0x0010;  /* SPI_4WIRE */
pub const WM9081_SPI_4WIRE_MASK: u32 = 0x0010;  /* SPI_4WIRE */
pub const WM9081_SPI_4WIRE_SHIFT: u32 = 4;  /* SPI_4WIRE */
pub const WM9081_SPI_4WIRE_WIDTH: u32 = 1;  /* SPI_4WIRE */
pub const WM9081_ARA_ENA: u32 = 0x0008;  /* ARA_ENA */
pub const WM9081_ARA_ENA_MASK: u32 = 0x0008;  /* ARA_ENA */
pub const WM9081_ARA_ENA_SHIFT: u32 = 3;  /* ARA_ENA */
pub const WM9081_ARA_ENA_WIDTH: u32 = 1;  /* ARA_ENA */
pub const WM9081_AUTO_INC: u32 = 0x0002;  /* AUTO_INC */
pub const WM9081_AUTO_INC_MASK: u32 = 0x0002;  /* AUTO_INC */
pub const WM9081_AUTO_INC_SHIFT: u32 = 1;  /* AUTO_INC */
pub const WM9081_AUTO_INC_WIDTH: u32 = 1;  /* AUTO_INC */

/*
 * R42 (0x2A) - EQ 1
 */
pub const WM9081_EQ_B1_GAIN_MASK: u32 = 0xF800;  /* EQ_B1_GAIN - [15:11] */
pub const WM9081_EQ_B1_GAIN_SHIFT: u32 = 11;  /* EQ_B1_GAIN - [15:11] */
pub const WM9081_EQ_B1_GAIN_WIDTH: u32 = 5;  /* EQ_B1_GAIN - [15:11] */
pub const WM9081_EQ_B2_GAIN_MASK: u32 = 0x07C0;  /* EQ_B2_GAIN - [10:6] */
pub const WM9081_EQ_B2_GAIN_SHIFT: u32 = 6;  /* EQ_B2_GAIN - [10:6] */
pub const WM9081_EQ_B2_GAIN_WIDTH: u32 = 5;  /* EQ_B2_GAIN - [10:6] */
pub const WM9081_EQ_B4_GAIN_MASK: u32 = 0x003E;  /* EQ_B4_GAIN - [5:1] */
pub const WM9081_EQ_B4_GAIN_SHIFT: u32 = 1;  /* EQ_B4_GAIN - [5:1] */
pub const WM9081_EQ_B4_GAIN_WIDTH: u32 = 5;  /* EQ_B4_GAIN - [5:1] */
pub const WM9081_EQ_ENA: u32 = 0x0001;  /* EQ_ENA */
pub const WM9081_EQ_ENA_MASK: u32 = 0x0001;  /* EQ_ENA */
pub const WM9081_EQ_ENA_SHIFT: u32 = 0;  /* EQ_ENA */
pub const WM9081_EQ_ENA_WIDTH: u32 = 1;  /* EQ_ENA */

/*
 * R43 (0x2B) - EQ 2
 */
pub const WM9081_EQ_B3_GAIN_MASK: u32 = 0xF800;  /* EQ_B3_GAIN - [15:11] */
pub const WM9081_EQ_B3_GAIN_SHIFT: u32 = 11;  /* EQ_B3_GAIN - [15:11] */
pub const WM9081_EQ_B3_GAIN_WIDTH: u32 = 5;  /* EQ_B3_GAIN - [15:11] */
pub const WM9081_EQ_B5_GAIN_MASK: u32 = 0x07C0;  /* EQ_B5_GAIN - [10:6] */
pub const WM9081_EQ_B5_GAIN_SHIFT: u32 = 6;  /* EQ_B5_GAIN - [10:6] */
pub const WM9081_EQ_B5_GAIN_WIDTH: u32 = 5;  /* EQ_B5_GAIN - [10:6] */

/*
 * R44 (0x2C) - EQ 3
 */
pub const WM9081_EQ_B1_A_MASK: u32 = 0xFFFF;  /* EQ_B1_A - [15:0] */
pub const WM9081_EQ_B1_A_SHIFT: u32 = 0;  /* EQ_B1_A - [15:0] */
pub const WM9081_EQ_B1_A_WIDTH: u32 = 16;  /* EQ_B1_A - [15:0] */

/*
 * R45 (0x2D) - EQ 4
 */
pub const WM9081_EQ_B1_B_MASK: u32 = 0xFFFF;  /* EQ_B1_B - [15:0] */
pub const WM9081_EQ_B1_B_SHIFT: u32 = 0;  /* EQ_B1_B - [15:0] */
pub const WM9081_EQ_B1_B_WIDTH: u32 = 16;  /* EQ_B1_B - [15:0] */

/*
 * R46 (0x2E) - EQ 5
 */
pub const WM9081_EQ_B1_PG_MASK: u32 = 0xFFFF;  /* EQ_B1_PG - [15:0] */
pub const WM9081_EQ_B1_PG_SHIFT: u32 = 0;  /* EQ_B1_PG - [15:0] */
pub const WM9081_EQ_B1_PG_WIDTH: u32 = 16;  /* EQ_B1_PG - [15:0] */

/*
 * R47 (0x2F) - EQ 6
 */
pub const WM9081_EQ_B2_A_MASK: u32 = 0xFFFF;  /* EQ_B2_A - [15:0] */
pub const WM9081_EQ_B2_A_SHIFT: u32 = 0;  /* EQ_B2_A - [15:0] */
pub const WM9081_EQ_B2_A_WIDTH: u32 = 16;  /* EQ_B2_A - [15:0] */

/*
 * R48 (0x30) - EQ 7
 */
pub const WM9081_EQ_B2_B_MASK: u32 = 0xFFFF;  /* EQ_B2_B - [15:0] */
pub const WM9081_EQ_B2_B_SHIFT: u32 = 0;  /* EQ_B2_B - [15:0] */
pub const WM9081_EQ_B2_B_WIDTH: u32 = 16;  /* EQ_B2_B - [15:0] */

/*
 * R49 (0x31) - EQ 8
 */
pub const WM9081_EQ_B2_C_MASK: u32 = 0xFFFF;  /* EQ_B2_C - [15:0] */
pub const WM9081_EQ_B2_C_SHIFT: u32 = 0;  /* EQ_B2_C - [15:0] */
pub const WM9081_EQ_B2_C_WIDTH: u32 = 16;  /* EQ_B2_C - [15:0] */

/*
 * R50 (0x32) - EQ 9
 */
pub const WM9081_EQ_B2_PG_MASK: u32 = 0xFFFF;  /* EQ_B2_PG - [15:0] */
pub const WM9081_EQ_B2_PG_SHIFT: u32 = 0;  /* EQ_B2_PG - [15:0] */
pub const WM9081_EQ_B2_PG_WIDTH: u32 = 16;  /* EQ_B2_PG - [15:0] */

/*
 * R51 (0x33) - EQ 10
 */
pub const WM9081_EQ_B4_A_MASK: u32 = 0xFFFF;  /* EQ_B4_A - [15:0] */
pub const WM9081_EQ_B4_A_SHIFT: u32 = 0;  /* EQ_B4_A - [15:0] */
pub const WM9081_EQ_B4_A_WIDTH: u32 = 16;  /* EQ_B4_A - [15:0] */

/*
 * R52 (0x34) - EQ 11
 */
pub const WM9081_EQ_B4_B_MASK: u32 = 0xFFFF;  /* EQ_B4_B - [15:0] */
pub const WM9081_EQ_B4_B_SHIFT: u32 = 0;  /* EQ_B4_B - [15:0] */
pub const WM9081_EQ_B4_B_WIDTH: u32 = 16;  /* EQ_B4_B - [15:0] */

/*
 * R53 (0x35) - EQ 12
 */
pub const WM9081_EQ_B4_C_MASK: u32 = 0xFFFF;  /* EQ_B4_C - [15:0] */
pub const WM9081_EQ_B4_C_SHIFT: u32 = 0;  /* EQ_B4_C - [15:0] */
pub const WM9081_EQ_B4_C_WIDTH: u32 = 16;  /* EQ_B4_C - [15:0] */

/*
 * R54 (0x36) - EQ 13
 */
pub const WM9081_EQ_B4_PG_MASK: u32 = 0xFFFF;  /* EQ_B4_PG - [15:0] */
pub const WM9081_EQ_B4_PG_SHIFT: u32 = 0;  /* EQ_B4_PG - [15:0] */
pub const WM9081_EQ_B4_PG_WIDTH: u32 = 16;  /* EQ_B4_PG - [15:0] */

/*
 * R55 (0x37) - EQ 14
 */
pub const WM9081_EQ_B3_A_MASK: u32 = 0xFFFF;  /* EQ_B3_A - [15:0] */
pub const WM9081_EQ_B3_A_SHIFT: u32 = 0;  /* EQ_B3_A - [15:0] */
pub const WM9081_EQ_B3_A_WIDTH: u32 = 16;  /* EQ_B3_A - [15:0] */

/*
 * R56 (0x38) - EQ 15
 */
pub const WM9081_EQ_B3_B_MASK: u32 = 0xFFFF;  /* EQ_B3_B - [15:0] */
pub const WM9081_EQ_B3_B_SHIFT: u32 = 0;  /* EQ_B3_B - [15:0] */
pub const WM9081_EQ_B3_B_WIDTH: u32 = 16;  /* EQ_B3_B - [15:0] */

/*
 * R57 (0x39) - EQ 16
 */
pub const WM9081_EQ_B3_C_MASK: u32 = 0xFFFF;  /* EQ_B3_C - [15:0] */
pub const WM9081_EQ_B3_C_SHIFT: u32 = 0;  /* EQ_B3_C - [15:0] */
pub const WM9081_EQ_B3_C_WIDTH: u32 = 16;  /* EQ_B3_C - [15:0] */

/*
 * R58 (0x3A) - EQ 17
 */
pub const WM9081_EQ_B3_PG_MASK: u32 = 0xFFFF;  /* EQ_B3_PG - [15:0] */
pub const WM9081_EQ_B3_PG_SHIFT: u32 = 0;  /* EQ_B3_PG - [15:0] */
pub const WM9081_EQ_B3_PG_WIDTH: u32 = 16;  /* EQ_B3_PG - [15:0] */

/*
 * R59 (0x3B) - EQ 18
 */
pub const WM9081_EQ_B5_A_MASK: u32 = 0xFFFF;  /* EQ_B5_A - [15:0] */
pub const WM9081_EQ_B5_A_SHIFT: u32 = 0;  /* EQ_B5_A - [15:0] */
pub const WM9081_EQ_B5_A_WIDTH: u32 = 16;  /* EQ_B5_A - [15:0] */

/*
 * R60 (0x3C) - EQ 19
 */
pub const WM9081_EQ_B5_B_MASK: u32 = 0xFFFF;  /* EQ_B5_B - [15:0] */
pub const WM9081_EQ_B5_B_SHIFT: u32 = 0;  /* EQ_B5_B - [15:0] */
pub const WM9081_EQ_B5_B_WIDTH: u32 = 16;  /* EQ_B5_B - [15:0] */

/*
 * R61 (0x3D) - EQ 20
 */
pub const WM9081_EQ_B5_PG_MASK: u32 = 0xFFFF;  /* EQ_B5_PG - [15:0] */
pub const WM9081_EQ_B5_PG_SHIFT: u32 = 0;  /* EQ_B5_PG - [15:0] */
pub const WM9081_EQ_B5_PG_WIDTH: u32 = 16;  /* EQ_B5_PG - [15:0] */



// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
