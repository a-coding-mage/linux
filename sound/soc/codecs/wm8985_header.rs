/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * wm8985.h  --  WM8985 ASoC driver
 *
 * Copyright 2010 Wolfson Microelectronics plc
 *
 * Author: Dimitris Papastamos <dp@opensource.wolfsonmicro.com>
 */

/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * wm8985.h  --  WM8985 ASoC driver
 *
 * Copyright 2010 Wolfson Microelectronics plc
 *
 * Author: Dimitris Papastamos <dp@opensource.wolfsonmicro.com>
 */


pub const WM8985_SOFTWARE_RESET: u32 = 0x00;
pub const WM8985_POWER_MANAGEMENT_1: u32 = 0x01;
pub const WM8985_POWER_MANAGEMENT_2: u32 = 0x02;
pub const WM8985_POWER_MANAGEMENT_3: u32 = 0x03;
pub const WM8985_AUDIO_INTERFACE: u32 = 0x04;
pub const WM8985_COMPANDING_CONTROL: u32 = 0x05;
pub const WM8985_CLOCK_GEN_CONTROL: u32 = 0x06;
pub const WM8985_ADDITIONAL_CONTROL: u32 = 0x07;
pub const WM8985_GPIO_CONTROL: u32 = 0x08;
pub const WM8985_JACK_DETECT_CONTROL_1: u32 = 0x09;
pub const WM8985_DAC_CONTROL: u32 = 0x0A;
pub const WM8985_LEFT_DAC_DIGITAL_VOL: u32 = 0x0B;
pub const WM8985_RIGHT_DAC_DIGITAL_VOL: u32 = 0x0C;
pub const WM8985_JACK_DETECT_CONTROL_2: u32 = 0x0D;
pub const WM8985_ADC_CONTROL: u32 = 0x0E;
pub const WM8985_LEFT_ADC_DIGITAL_VOL: u32 = 0x0F;
pub const WM8985_RIGHT_ADC_DIGITAL_VOL: u32 = 0x10;
pub const WM8985_EQ1_LOW_SHELF: u32 = 0x12;
pub const WM8985_EQ2_PEAK_1: u32 = 0x13;
pub const WM8985_EQ3_PEAK_2: u32 = 0x14;
pub const WM8985_EQ4_PEAK_3: u32 = 0x15;
pub const WM8985_EQ5_HIGH_SHELF: u32 = 0x16;
pub const WM8985_DAC_LIMITER_1: u32 = 0x18;
pub const WM8985_DAC_LIMITER_2: u32 = 0x19;
pub const WM8985_NOTCH_FILTER_1: u32 = 0x1B;
pub const WM8985_NOTCH_FILTER_2: u32 = 0x1C;
pub const WM8985_NOTCH_FILTER_3: u32 = 0x1D;
pub const WM8985_NOTCH_FILTER_4: u32 = 0x1E;
pub const WM8985_ALC_CONTROL_1: u32 = 0x20;
pub const WM8985_ALC_CONTROL_2: u32 = 0x21;
pub const WM8985_ALC_CONTROL_3: u32 = 0x22;
pub const WM8985_NOISE_GATE: u32 = 0x23;
pub const WM8985_PLL_N: u32 = 0x24;
pub const WM8985_PLL_K_1: u32 = 0x25;
pub const WM8985_PLL_K_2: u32 = 0x26;
pub const WM8985_PLL_K_3: u32 = 0x27;
pub const WM8985_3D_CONTROL: u32 = 0x29;
pub const WM8985_OUT4_TO_ADC: u32 = 0x2A;
pub const WM8985_BEEP_CONTROL: u32 = 0x2B;
pub const WM8985_INPUT_CTRL: u32 = 0x2C;
pub const WM8985_LEFT_INP_PGA_GAIN_CTRL: u32 = 0x2D;
pub const WM8985_RIGHT_INP_PGA_GAIN_CTRL: u32 = 0x2E;
pub const WM8985_LEFT_ADC_BOOST_CTRL: u32 = 0x2F;
pub const WM8985_RIGHT_ADC_BOOST_CTRL: u32 = 0x30;
pub const WM8985_OUTPUT_CTRL0: u32 = 0x31;
pub const WM8985_LEFT_MIXER_CTRL: u32 = 0x32;
pub const WM8985_RIGHT_MIXER_CTRL: u32 = 0x33;
pub const WM8985_LOUT1_HP_VOLUME_CTRL: u32 = 0x34;
pub const WM8985_ROUT1_HP_VOLUME_CTRL: u32 = 0x35;
pub const WM8985_LOUT2_SPK_VOLUME_CTRL: u32 = 0x36;
pub const WM8985_ROUT2_SPK_VOLUME_CTRL: u32 = 0x37;
pub const WM8985_OUT3_MIXER_CTRL: u32 = 0x38;
pub const WM8985_OUT4_MONO_MIX_CTRL: u32 = 0x39;
pub const WM8985_OUTPUT_CTRL1: u32 = 0x3C;
pub const WM8985_BIAS_CTRL: u32 = 0x3D;

pub const WM8985_REGISTER_COUNT: u32 = 59;
pub const WM8985_MAX_REGISTER: u32 = 0x3F;

/*
 * Field Definitions.
 */

/*
 * R0 (0x00) - Software Reset
 */
pub const WM8985_SOFTWARE_RESET_MASK: u32 = 0x01FF;  /* SOFTWARE_RESET - [8:0] */
pub const WM8985_SOFTWARE_RESET_SHIFT: u32 = 0;  /* SOFTWARE_RESET - [8:0] */
pub const WM8985_SOFTWARE_RESET_WIDTH: u32 = 9;  /* SOFTWARE_RESET - [8:0] */

/*
 * R1 (0x01) - Power management 1
 */
pub const WM8985_OUT4MIXEN: u32 = 0x0080;  /* OUT4MIXEN */
pub const WM8985_OUT4MIXEN_MASK: u32 = 0x0080;  /* OUT4MIXEN */
pub const WM8985_OUT4MIXEN_SHIFT: u32 = 7;  /* OUT4MIXEN */
pub const WM8985_OUT4MIXEN_WIDTH: u32 = 1;  /* OUT4MIXEN */
pub const WM8985_OUT3MIXEN: u32 = 0x0040;  /* OUT3MIXEN */
pub const WM8985_OUT3MIXEN_MASK: u32 = 0x0040;  /* OUT3MIXEN */
pub const WM8985_OUT3MIXEN_SHIFT: u32 = 6;  /* OUT3MIXEN */
pub const WM8985_OUT3MIXEN_WIDTH: u32 = 1;  /* OUT3MIXEN */
pub const WM8985_PLLEN: u32 = 0x0020;  /* PLLEN */
pub const WM8985_PLLEN_MASK: u32 = 0x0020;  /* PLLEN */
pub const WM8985_PLLEN_SHIFT: u32 = 5;  /* PLLEN */
pub const WM8985_PLLEN_WIDTH: u32 = 1;  /* PLLEN */
pub const WM8985_MICBEN: u32 = 0x0010;  /* MICBEN */
pub const WM8985_MICBEN_MASK: u32 = 0x0010;  /* MICBEN */
pub const WM8985_MICBEN_SHIFT: u32 = 4;  /* MICBEN */
pub const WM8985_MICBEN_WIDTH: u32 = 1;  /* MICBEN */
pub const WM8985_BIASEN: u32 = 0x0008;  /* BIASEN */
pub const WM8985_BIASEN_MASK: u32 = 0x0008;  /* BIASEN */
pub const WM8985_BIASEN_SHIFT: u32 = 3;  /* BIASEN */
pub const WM8985_BIASEN_WIDTH: u32 = 1;  /* BIASEN */
pub const WM8985_BUFIOEN: u32 = 0x0004;  /* BUFIOEN */
pub const WM8985_BUFIOEN_MASK: u32 = 0x0004;  /* BUFIOEN */
pub const WM8985_BUFIOEN_SHIFT: u32 = 2;  /* BUFIOEN */
pub const WM8985_BUFIOEN_WIDTH: u32 = 1;  /* BUFIOEN */
pub const WM8985_VMIDSEL: u32 = 0x0003;  /* VMIDSEL */
pub const WM8985_VMIDSEL_MASK: u32 = 0x0003;  /* VMIDSEL - [1:0] */
pub const WM8985_VMIDSEL_SHIFT: u32 = 0;  /* VMIDSEL - [1:0] */
pub const WM8985_VMIDSEL_WIDTH: u32 = 2;  /* VMIDSEL - [1:0] */

/*
 * R2 (0x02) - Power management 2
 */
pub const WM8985_ROUT1EN: u32 = 0x0100;  /* ROUT1EN */
pub const WM8985_ROUT1EN_MASK: u32 = 0x0100;  /* ROUT1EN */
pub const WM8985_ROUT1EN_SHIFT: u32 = 8;  /* ROUT1EN */
pub const WM8985_ROUT1EN_WIDTH: u32 = 1;  /* ROUT1EN */
pub const WM8985_LOUT1EN: u32 = 0x0080;  /* LOUT1EN */
pub const WM8985_LOUT1EN_MASK: u32 = 0x0080;  /* LOUT1EN */
pub const WM8985_LOUT1EN_SHIFT: u32 = 7;  /* LOUT1EN */
pub const WM8985_LOUT1EN_WIDTH: u32 = 1;  /* LOUT1EN */
pub const WM8985_SLEEP: u32 = 0x0040;  /* SLEEP */
pub const WM8985_SLEEP_MASK: u32 = 0x0040;  /* SLEEP */
pub const WM8985_SLEEP_SHIFT: u32 = 6;  /* SLEEP */
pub const WM8985_SLEEP_WIDTH: u32 = 1;  /* SLEEP */
pub const WM8985_BOOSTENR: u32 = 0x0020;  /* BOOSTENR */
pub const WM8985_BOOSTENR_MASK: u32 = 0x0020;  /* BOOSTENR */
pub const WM8985_BOOSTENR_SHIFT: u32 = 5;  /* BOOSTENR */
pub const WM8985_BOOSTENR_WIDTH: u32 = 1;  /* BOOSTENR */
pub const WM8985_BOOSTENL: u32 = 0x0010;  /* BOOSTENL */
pub const WM8985_BOOSTENL_MASK: u32 = 0x0010;  /* BOOSTENL */
pub const WM8985_BOOSTENL_SHIFT: u32 = 4;  /* BOOSTENL */
pub const WM8985_BOOSTENL_WIDTH: u32 = 1;  /* BOOSTENL */
pub const WM8985_INPGAENR: u32 = 0x0008;  /* INPGAENR */
pub const WM8985_INPGAENR_MASK: u32 = 0x0008;  /* INPGAENR */
pub const WM8985_INPGAENR_SHIFT: u32 = 3;  /* INPGAENR */
pub const WM8985_INPGAENR_WIDTH: u32 = 1;  /* INPGAENR */
pub const WM8985_INPPGAENL: u32 = 0x0004;  /* INPPGAENL */
pub const WM8985_INPPGAENL_MASK: u32 = 0x0004;  /* INPPGAENL */
pub const WM8985_INPPGAENL_SHIFT: u32 = 2;  /* INPPGAENL */
pub const WM8985_INPPGAENL_WIDTH: u32 = 1;  /* INPPGAENL */
pub const WM8985_ADCENR: u32 = 0x0002;  /* ADCENR */
pub const WM8985_ADCENR_MASK: u32 = 0x0002;  /* ADCENR */
pub const WM8985_ADCENR_SHIFT: u32 = 1;  /* ADCENR */
pub const WM8985_ADCENR_WIDTH: u32 = 1;  /* ADCENR */
pub const WM8985_ADCENL: u32 = 0x0001;  /* ADCENL */
pub const WM8985_ADCENL_MASK: u32 = 0x0001;  /* ADCENL */
pub const WM8985_ADCENL_SHIFT: u32 = 0;  /* ADCENL */
pub const WM8985_ADCENL_WIDTH: u32 = 1;  /* ADCENL */

/*
 * R3 (0x03) - Power management 3
 */
pub const WM8985_OUT4EN: u32 = 0x0100;  /* OUT4EN */
pub const WM8985_OUT4EN_MASK: u32 = 0x0100;  /* OUT4EN */
pub const WM8985_OUT4EN_SHIFT: u32 = 8;  /* OUT4EN */
pub const WM8985_OUT4EN_WIDTH: u32 = 1;  /* OUT4EN */
pub const WM8985_OUT3EN: u32 = 0x0080;  /* OUT3EN */
pub const WM8985_OUT3EN_MASK: u32 = 0x0080;  /* OUT3EN */
pub const WM8985_OUT3EN_SHIFT: u32 = 7;  /* OUT3EN */
pub const WM8985_OUT3EN_WIDTH: u32 = 1;  /* OUT3EN */
pub const WM8985_ROUT2EN: u32 = 0x0040;  /* ROUT2EN */
pub const WM8985_ROUT2EN_MASK: u32 = 0x0040;  /* ROUT2EN */
pub const WM8985_ROUT2EN_SHIFT: u32 = 6;  /* ROUT2EN */
pub const WM8985_ROUT2EN_WIDTH: u32 = 1;  /* ROUT2EN */
pub const WM8985_LOUT2EN: u32 = 0x0020;  /* LOUT2EN */
pub const WM8985_LOUT2EN_MASK: u32 = 0x0020;  /* LOUT2EN */
pub const WM8985_LOUT2EN_SHIFT: u32 = 5;  /* LOUT2EN */
pub const WM8985_LOUT2EN_WIDTH: u32 = 1;  /* LOUT2EN */
pub const WM8985_RMIXEN: u32 = 0x0008;  /* RMIXEN */
pub const WM8985_RMIXEN_MASK: u32 = 0x0008;  /* RMIXEN */
pub const WM8985_RMIXEN_SHIFT: u32 = 3;  /* RMIXEN */
pub const WM8985_RMIXEN_WIDTH: u32 = 1;  /* RMIXEN */
pub const WM8985_LMIXEN: u32 = 0x0004;  /* LMIXEN */
pub const WM8985_LMIXEN_MASK: u32 = 0x0004;  /* LMIXEN */
pub const WM8985_LMIXEN_SHIFT: u32 = 2;  /* LMIXEN */
pub const WM8985_LMIXEN_WIDTH: u32 = 1;  /* LMIXEN */
pub const WM8985_DACENR: u32 = 0x0002;  /* DACENR */
pub const WM8985_DACENR_MASK: u32 = 0x0002;  /* DACENR */
pub const WM8985_DACENR_SHIFT: u32 = 1;  /* DACENR */
pub const WM8985_DACENR_WIDTH: u32 = 1;  /* DACENR */
pub const WM8985_DACENL: u32 = 0x0001;  /* DACENL */
pub const WM8985_DACENL_MASK: u32 = 0x0001;  /* DACENL */
pub const WM8985_DACENL_SHIFT: u32 = 0;  /* DACENL */
pub const WM8985_DACENL_WIDTH: u32 = 1;  /* DACENL */

/*
 * R4 (0x04) - Audio Interface
 */
pub const WM8985_BCP: u32 = 0x0100;  /* BCP */
pub const WM8985_BCP_MASK: u32 = 0x0100;  /* BCP */
pub const WM8985_BCP_SHIFT: u32 = 8;  /* BCP */
pub const WM8985_BCP_WIDTH: u32 = 1;  /* BCP */
pub const WM8985_LRP: u32 = 0x0080;  /* LRP */
pub const WM8985_LRP_MASK: u32 = 0x0080;  /* LRP */
pub const WM8985_LRP_SHIFT: u32 = 7;  /* LRP */
pub const WM8985_LRP_WIDTH: u32 = 1;  /* LRP */
pub const WM8985_WL_MASK: u32 = 0x0060;  /* WL - [6:5] */
pub const WM8985_WL_SHIFT: u32 = 5;  /* WL - [6:5] */
pub const WM8985_WL_WIDTH: u32 = 2;  /* WL - [6:5] */
pub const WM8985_FMT_MASK: u32 = 0x0018;  /* FMT - [4:3] */
pub const WM8985_FMT_SHIFT: u32 = 3;  /* FMT - [4:3] */
pub const WM8985_FMT_WIDTH: u32 = 2;  /* FMT - [4:3] */
pub const WM8985_DLRSWAP: u32 = 0x0004;  /* DLRSWAP */
pub const WM8985_DLRSWAP_MASK: u32 = 0x0004;  /* DLRSWAP */
pub const WM8985_DLRSWAP_SHIFT: u32 = 2;  /* DLRSWAP */
pub const WM8985_DLRSWAP_WIDTH: u32 = 1;  /* DLRSWAP */
pub const WM8985_ALRSWAP: u32 = 0x0002;  /* ALRSWAP */
pub const WM8985_ALRSWAP_MASK: u32 = 0x0002;  /* ALRSWAP */
pub const WM8985_ALRSWAP_SHIFT: u32 = 1;  /* ALRSWAP */
pub const WM8985_ALRSWAP_WIDTH: u32 = 1;  /* ALRSWAP */
pub const WM8985_MONO: u32 = 0x0001;  /* MONO */
pub const WM8985_MONO_MASK: u32 = 0x0001;  /* MONO */
pub const WM8985_MONO_SHIFT: u32 = 0;  /* MONO */
pub const WM8985_MONO_WIDTH: u32 = 1;  /* MONO */

/*
 * R5 (0x05) - Companding control
 */
pub const WM8985_WL8: u32 = 0x0020;  /* WL8 */
pub const WM8985_WL8_MASK: u32 = 0x0020;  /* WL8 */
pub const WM8985_WL8_SHIFT: u32 = 5;  /* WL8 */
pub const WM8985_WL8_WIDTH: u32 = 1;  /* WL8 */
pub const WM8985_DAC_COMP_MASK: u32 = 0x0018;  /* DAC_COMP - [4:3] */
pub const WM8985_DAC_COMP_SHIFT: u32 = 3;  /* DAC_COMP - [4:3] */
pub const WM8985_DAC_COMP_WIDTH: u32 = 2;  /* DAC_COMP - [4:3] */
pub const WM8985_ADC_COMP_MASK: u32 = 0x0006;  /* ADC_COMP - [2:1] */
pub const WM8985_ADC_COMP_SHIFT: u32 = 1;  /* ADC_COMP - [2:1] */
pub const WM8985_ADC_COMP_WIDTH: u32 = 2;  /* ADC_COMP - [2:1] */
pub const WM8985_LOOPBACK: u32 = 0x0001;  /* LOOPBACK */
pub const WM8985_LOOPBACK_MASK: u32 = 0x0001;  /* LOOPBACK */
pub const WM8985_LOOPBACK_SHIFT: u32 = 0;  /* LOOPBACK */
pub const WM8985_LOOPBACK_WIDTH: u32 = 1;  /* LOOPBACK */

/*
 * R6 (0x06) - Clock Gen control
 */
pub const WM8985_CLKSEL: u32 = 0x0100;  /* CLKSEL */
pub const WM8985_CLKSEL_MASK: u32 = 0x0100;  /* CLKSEL */
pub const WM8985_CLKSEL_SHIFT: u32 = 8;  /* CLKSEL */
pub const WM8985_CLKSEL_WIDTH: u32 = 1;  /* CLKSEL */
pub const WM8985_MCLKDIV_MASK: u32 = 0x00E0;  /* MCLKDIV - [7:5] */
pub const WM8985_MCLKDIV_SHIFT: u32 = 5;  /* MCLKDIV - [7:5] */
pub const WM8985_MCLKDIV_WIDTH: u32 = 3;  /* MCLKDIV - [7:5] */
pub const WM8985_BCLKDIV_MASK: u32 = 0x001C;  /* BCLKDIV - [4:2] */
pub const WM8985_BCLKDIV_SHIFT: u32 = 2;  /* BCLKDIV - [4:2] */
pub const WM8985_BCLKDIV_WIDTH: u32 = 3;  /* BCLKDIV - [4:2] */
pub const WM8985_MS: u32 = 0x0001;  /* MS */
pub const WM8985_MS_MASK: u32 = 0x0001;  /* MS */
pub const WM8985_MS_SHIFT: u32 = 0;  /* MS */
pub const WM8985_MS_WIDTH: u32 = 1;  /* MS */

/*
 * R7 (0x07) - Additional control
 */
pub const WM8985_M128ENB: u32 = 0x0100;  /* M128ENB */
pub const WM8985_M128ENB_MASK: u32 = 0x0100;  /* M128ENB */
pub const WM8985_M128ENB_SHIFT: u32 = 8;  /* M128ENB */
pub const WM8985_M128ENB_WIDTH: u32 = 1;  /* M128ENB */
pub const WM8985_DCLKDIV_MASK: u32 = 0x00F0;  /* DCLKDIV - [7:4] */
pub const WM8985_DCLKDIV_SHIFT: u32 = 4;  /* DCLKDIV - [7:4] */
pub const WM8985_DCLKDIV_WIDTH: u32 = 4;  /* DCLKDIV - [7:4] */
pub const WM8985_SR_MASK: u32 = 0x000E;  /* SR - [3:1] */
pub const WM8985_SR_SHIFT: u32 = 1;  /* SR - [3:1] */
pub const WM8985_SR_WIDTH: u32 = 3;  /* SR - [3:1] */
pub const WM8985_SLOWCLKEN: u32 = 0x0001;  /* SLOWCLKEN */
pub const WM8985_SLOWCLKEN_MASK: u32 = 0x0001;  /* SLOWCLKEN */
pub const WM8985_SLOWCLKEN_SHIFT: u32 = 0;  /* SLOWCLKEN */
pub const WM8985_SLOWCLKEN_WIDTH: u32 = 1;  /* SLOWCLKEN */

/*
 * R8 (0x08) - GPIO Control
 */
pub const WM8985_GPIO1GP: u32 = 0x0100;  /* GPIO1GP */
pub const WM8985_GPIO1GP_MASK: u32 = 0x0100;  /* GPIO1GP */
pub const WM8985_GPIO1GP_SHIFT: u32 = 8;  /* GPIO1GP */
pub const WM8985_GPIO1GP_WIDTH: u32 = 1;  /* GPIO1GP */
pub const WM8985_GPIO1GPU: u32 = 0x0080;  /* GPIO1GPU */
pub const WM8985_GPIO1GPU_MASK: u32 = 0x0080;  /* GPIO1GPU */
pub const WM8985_GPIO1GPU_SHIFT: u32 = 7;  /* GPIO1GPU */
pub const WM8985_GPIO1GPU_WIDTH: u32 = 1;  /* GPIO1GPU */
pub const WM8985_GPIO1GPD: u32 = 0x0040;  /* GPIO1GPD */
pub const WM8985_GPIO1GPD_MASK: u32 = 0x0040;  /* GPIO1GPD */
pub const WM8985_GPIO1GPD_SHIFT: u32 = 6;  /* GPIO1GPD */
pub const WM8985_GPIO1GPD_WIDTH: u32 = 1;  /* GPIO1GPD */
pub const WM8758_OPCLKDIV_MASK: u32 = 0x0030;  /* OPCLKDIV - [1:0] */
pub const WM8758_OPCLKDIV_SHIFT: u32 = 4;  /* OPCLKDIV - [1:0] */
pub const WM8758_OPCLKDIV_WIDTH: u32 = 2;  /* OPCLKDIV - [1:0] */
pub const WM8985_GPIO1POL: u32 = 0x0008;  /* GPIO1POL */
pub const WM8985_GPIO1POL_MASK: u32 = 0x0008;  /* GPIO1POL */
pub const WM8985_GPIO1POL_SHIFT: u32 = 3;  /* GPIO1POL */
pub const WM8985_GPIO1POL_WIDTH: u32 = 1;  /* GPIO1POL */
pub const WM8985_GPIO1SEL_MASK: u32 = 0x0007;  /* GPIO1SEL - [2:0] */
pub const WM8985_GPIO1SEL_SHIFT: u32 = 0;  /* GPIO1SEL - [2:0] */
pub const WM8985_GPIO1SEL_WIDTH: u32 = 3;  /* GPIO1SEL - [2:0] */

/*
 * R9 (0x09) - Jack Detect Control 1
 */
pub const WM8758_JD_VMID1_MASK: u32 = 0x0100;  /* JD_VMID1 */
pub const WM8758_JD_VMID1_SHIFT: u32 = 8;  /* JD_VMID1 */
pub const WM8758_JD_VMID1_WIDTH: u32 = 1;  /* JD_VMID1 */
pub const WM8758_JD_VMID0_MASK: u32 = 0x0080;  /* JD_VMID0 */
pub const WM8758_JD_VMID0_SHIFT: u32 = 7;  /* JD_VMID0 */
pub const WM8758_JD_VMID0_WIDTH: u32 = 1;  /* JD_VMID0 */
pub const WM8985_JD_EN: u32 = 0x0040;  /* JD_EN */
pub const WM8985_JD_EN_MASK: u32 = 0x0040;  /* JD_EN */
pub const WM8985_JD_EN_SHIFT: u32 = 6;  /* JD_EN */
pub const WM8985_JD_EN_WIDTH: u32 = 1;  /* JD_EN */
pub const WM8985_JD_SEL_MASK: u32 = 0x0030;  /* JD_SEL - [5:4] */
pub const WM8985_JD_SEL_SHIFT: u32 = 4;  /* JD_SEL - [5:4] */
pub const WM8985_JD_SEL_WIDTH: u32 = 2;  /* JD_SEL - [5:4] */

/*
 * R10 (0x0A) - DAC Control
 */
pub const WM8985_SOFTMUTE: u32 = 0x0040;  /* SOFTMUTE */
pub const WM8985_SOFTMUTE_MASK: u32 = 0x0040;  /* SOFTMUTE */
pub const WM8985_SOFTMUTE_SHIFT: u32 = 6;  /* SOFTMUTE */
pub const WM8985_SOFTMUTE_WIDTH: u32 = 1;  /* SOFTMUTE */
pub const WM8985_DACOSR128: u32 = 0x0008;  /* DACOSR128 */
pub const WM8985_DACOSR128_MASK: u32 = 0x0008;  /* DACOSR128 */
pub const WM8985_DACOSR128_SHIFT: u32 = 3;  /* DACOSR128 */
pub const WM8985_DACOSR128_WIDTH: u32 = 1;  /* DACOSR128 */
pub const WM8985_AMUTE: u32 = 0x0004;  /* AMUTE */
pub const WM8985_AMUTE_MASK: u32 = 0x0004;  /* AMUTE */
pub const WM8985_AMUTE_SHIFT: u32 = 2;  /* AMUTE */
pub const WM8985_AMUTE_WIDTH: u32 = 1;  /* AMUTE */
pub const WM8985_DACPOLR: u32 = 0x0002;  /* DACPOLR */
pub const WM8985_DACPOLR_MASK: u32 = 0x0002;  /* DACPOLR */
pub const WM8985_DACPOLR_SHIFT: u32 = 1;  /* DACPOLR */
pub const WM8985_DACPOLR_WIDTH: u32 = 1;  /* DACPOLR */
pub const WM8985_DACPOLL: u32 = 0x0001;  /* DACPOLL */
pub const WM8985_DACPOLL_MASK: u32 = 0x0001;  /* DACPOLL */
pub const WM8985_DACPOLL_SHIFT: u32 = 0;  /* DACPOLL */
pub const WM8985_DACPOLL_WIDTH: u32 = 1;  /* DACPOLL */

/*
 * R11 (0x0B) - Left DAC digital Vol
 */
pub const WM8985_DACVU: u32 = 0x0100;  /* DACVU */
pub const WM8985_DACVU_MASK: u32 = 0x0100;  /* DACVU */
pub const WM8985_DACVU_SHIFT: u32 = 8;  /* DACVU */
pub const WM8985_DACVU_WIDTH: u32 = 1;  /* DACVU */
pub const WM8985_DACVOLL_MASK: u32 = 0x00FF;  /* DACVOLL - [7:0] */
pub const WM8985_DACVOLL_SHIFT: u32 = 0;  /* DACVOLL - [7:0] */
pub const WM8985_DACVOLL_WIDTH: u32 = 8;  /* DACVOLL - [7:0] */

/*
 * R12 (0x0C) - Right DAC digital vol
 */
pub const WM8985_DACVOLR_MASK: u32 = 0x00FF;  /* DACVOLR - [7:0] */
pub const WM8985_DACVOLR_SHIFT: u32 = 0;  /* DACVOLR - [7:0] */
pub const WM8985_DACVOLR_WIDTH: u32 = 8;  /* DACVOLR - [7:0] */

/*
 * R13 (0x0D) - Jack Detect Control 2
 */
pub const WM8985_JD_EN1_MASK: u32 = 0x00F0;  /* JD_EN1 - [7:4] */
pub const WM8985_JD_EN1_SHIFT: u32 = 4;  /* JD_EN1 - [7:4] */
pub const WM8985_JD_EN1_WIDTH: u32 = 4;  /* JD_EN1 - [7:4] */
pub const WM8985_JD_EN0_MASK: u32 = 0x000F;  /* JD_EN0 - [3:0] */
pub const WM8985_JD_EN0_SHIFT: u32 = 0;  /* JD_EN0 - [3:0] */
pub const WM8985_JD_EN0_WIDTH: u32 = 4;  /* JD_EN0 - [3:0] */

/*
 * R14 (0x0E) - ADC Control
 */
pub const WM8985_HPFEN: u32 = 0x0100;  /* HPFEN */
pub const WM8985_HPFEN_MASK: u32 = 0x0100;  /* HPFEN */
pub const WM8985_HPFEN_SHIFT: u32 = 8;  /* HPFEN */
pub const WM8985_HPFEN_WIDTH: u32 = 1;  /* HPFEN */
pub const WM8985_HPFAPP: u32 = 0x0080;  /* HPFAPP */
pub const WM8985_HPFAPP_MASK: u32 = 0x0080;  /* HPFAPP */
pub const WM8985_HPFAPP_SHIFT: u32 = 7;  /* HPFAPP */
pub const WM8985_HPFAPP_WIDTH: u32 = 1;  /* HPFAPP */
pub const WM8985_HPFCUT_MASK: u32 = 0x0070;  /* HPFCUT - [6:4] */
pub const WM8985_HPFCUT_SHIFT: u32 = 4;  /* HPFCUT - [6:4] */
pub const WM8985_HPFCUT_WIDTH: u32 = 3;  /* HPFCUT - [6:4] */
pub const WM8985_ADCOSR128: u32 = 0x0008;  /* ADCOSR128 */
pub const WM8985_ADCOSR128_MASK: u32 = 0x0008;  /* ADCOSR128 */
pub const WM8985_ADCOSR128_SHIFT: u32 = 3;  /* ADCOSR128 */
pub const WM8985_ADCOSR128_WIDTH: u32 = 1;  /* ADCOSR128 */
pub const WM8985_ADCRPOL: u32 = 0x0002;  /* ADCRPOL */
pub const WM8985_ADCRPOL_MASK: u32 = 0x0002;  /* ADCRPOL */
pub const WM8985_ADCRPOL_SHIFT: u32 = 1;  /* ADCRPOL */
pub const WM8985_ADCRPOL_WIDTH: u32 = 1;  /* ADCRPOL */
pub const WM8985_ADCLPOL: u32 = 0x0001;  /* ADCLPOL */
pub const WM8985_ADCLPOL_MASK: u32 = 0x0001;  /* ADCLPOL */
pub const WM8985_ADCLPOL_SHIFT: u32 = 0;  /* ADCLPOL */
pub const WM8985_ADCLPOL_WIDTH: u32 = 1;  /* ADCLPOL */

/*
 * R15 (0x0F) - Left ADC Digital Vol
 */
pub const WM8985_ADCVU: u32 = 0x0100;  /* ADCVU */
pub const WM8985_ADCVU_MASK: u32 = 0x0100;  /* ADCVU */
pub const WM8985_ADCVU_SHIFT: u32 = 8;  /* ADCVU */
pub const WM8985_ADCVU_WIDTH: u32 = 1;  /* ADCVU */
pub const WM8985_ADCVOLL_MASK: u32 = 0x00FF;  /* ADCVOLL - [7:0] */
pub const WM8985_ADCVOLL_SHIFT: u32 = 0;  /* ADCVOLL - [7:0] */
pub const WM8985_ADCVOLL_WIDTH: u32 = 8;  /* ADCVOLL - [7:0] */

/*
 * R16 (0x10) - Right ADC Digital Vol
 */
pub const WM8985_ADCVOLR_MASK: u32 = 0x00FF;  /* ADCVOLR - [7:0] */
pub const WM8985_ADCVOLR_SHIFT: u32 = 0;  /* ADCVOLR - [7:0] */
pub const WM8985_ADCVOLR_WIDTH: u32 = 8;  /* ADCVOLR - [7:0] */

/*
 * R18 (0x12) - EQ1 - low shelf
 */
pub const WM8985_EQ3DMODE: u32 = 0x0100;  /* EQ3DMODE */
pub const WM8985_EQ3DMODE_MASK: u32 = 0x0100;  /* EQ3DMODE */
pub const WM8985_EQ3DMODE_SHIFT: u32 = 8;  /* EQ3DMODE */
pub const WM8985_EQ3DMODE_WIDTH: u32 = 1;  /* EQ3DMODE */
pub const WM8985_EQ1C_MASK: u32 = 0x0060;  /* EQ1C - [6:5] */
pub const WM8985_EQ1C_SHIFT: u32 = 5;  /* EQ1C - [6:5] */
pub const WM8985_EQ1C_WIDTH: u32 = 2;  /* EQ1C - [6:5] */
pub const WM8985_EQ1G_MASK: u32 = 0x001F;  /* EQ1G - [4:0] */
pub const WM8985_EQ1G_SHIFT: u32 = 0;  /* EQ1G - [4:0] */
pub const WM8985_EQ1G_WIDTH: u32 = 5;  /* EQ1G - [4:0] */

/*
 * R19 (0x13) - EQ2 - peak 1
 */
pub const WM8985_EQ2BW: u32 = 0x0100;  /* EQ2BW */
pub const WM8985_EQ2BW_MASK: u32 = 0x0100;  /* EQ2BW */
pub const WM8985_EQ2BW_SHIFT: u32 = 8;  /* EQ2BW */
pub const WM8985_EQ2BW_WIDTH: u32 = 1;  /* EQ2BW */
pub const WM8985_EQ2C_MASK: u32 = 0x0060;  /* EQ2C - [6:5] */
pub const WM8985_EQ2C_SHIFT: u32 = 5;  /* EQ2C - [6:5] */
pub const WM8985_EQ2C_WIDTH: u32 = 2;  /* EQ2C - [6:5] */
pub const WM8985_EQ2G_MASK: u32 = 0x001F;  /* EQ2G - [4:0] */
pub const WM8985_EQ2G_SHIFT: u32 = 0;  /* EQ2G - [4:0] */
pub const WM8985_EQ2G_WIDTH: u32 = 5;  /* EQ2G - [4:0] */

/*
 * R20 (0x14) - EQ3 - peak 2
 */
pub const WM8985_EQ3BW: u32 = 0x0100;  /* EQ3BW */
pub const WM8985_EQ3BW_MASK: u32 = 0x0100;  /* EQ3BW */
pub const WM8985_EQ3BW_SHIFT: u32 = 8;  /* EQ3BW */
pub const WM8985_EQ3BW_WIDTH: u32 = 1;  /* EQ3BW */
pub const WM8985_EQ3C_MASK: u32 = 0x0060;  /* EQ3C - [6:5] */
pub const WM8985_EQ3C_SHIFT: u32 = 5;  /* EQ3C - [6:5] */
pub const WM8985_EQ3C_WIDTH: u32 = 2;  /* EQ3C - [6:5] */
pub const WM8985_EQ3G_MASK: u32 = 0x001F;  /* EQ3G - [4:0] */
pub const WM8985_EQ3G_SHIFT: u32 = 0;  /* EQ3G - [4:0] */
pub const WM8985_EQ3G_WIDTH: u32 = 5;  /* EQ3G - [4:0] */

/*
 * R21 (0x15) - EQ4 - peak 3
 */
pub const WM8985_EQ4BW: u32 = 0x0100;  /* EQ4BW */
pub const WM8985_EQ4BW_MASK: u32 = 0x0100;  /* EQ4BW */
pub const WM8985_EQ4BW_SHIFT: u32 = 8;  /* EQ4BW */
pub const WM8985_EQ4BW_WIDTH: u32 = 1;  /* EQ4BW */
pub const WM8985_EQ4C_MASK: u32 = 0x0060;  /* EQ4C - [6:5] */
pub const WM8985_EQ4C_SHIFT: u32 = 5;  /* EQ4C - [6:5] */
pub const WM8985_EQ4C_WIDTH: u32 = 2;  /* EQ4C - [6:5] */
pub const WM8985_EQ4G_MASK: u32 = 0x001F;  /* EQ4G - [4:0] */
pub const WM8985_EQ4G_SHIFT: u32 = 0;  /* EQ4G - [4:0] */
pub const WM8985_EQ4G_WIDTH: u32 = 5;  /* EQ4G - [4:0] */

/*
 * R22 (0x16) - EQ5 - high shelf
 */
pub const WM8985_EQ5C_MASK: u32 = 0x0060;  /* EQ5C - [6:5] */
pub const WM8985_EQ5C_SHIFT: u32 = 5;  /* EQ5C - [6:5] */
pub const WM8985_EQ5C_WIDTH: u32 = 2;  /* EQ5C - [6:5] */
pub const WM8985_EQ5G_MASK: u32 = 0x001F;  /* EQ5G - [4:0] */
pub const WM8985_EQ5G_SHIFT: u32 = 0;  /* EQ5G - [4:0] */
pub const WM8985_EQ5G_WIDTH: u32 = 5;  /* EQ5G - [4:0] */

/*
 * R24 (0x18) - DAC Limiter 1
 */
pub const WM8985_LIMEN: u32 = 0x0100;  /* LIMEN */
pub const WM8985_LIMEN_MASK: u32 = 0x0100;  /* LIMEN */
pub const WM8985_LIMEN_SHIFT: u32 = 8;  /* LIMEN */
pub const WM8985_LIMEN_WIDTH: u32 = 1;  /* LIMEN */
pub const WM8985_LIMDCY_MASK: u32 = 0x00F0;  /* LIMDCY - [7:4] */
pub const WM8985_LIMDCY_SHIFT: u32 = 4;  /* LIMDCY - [7:4] */
pub const WM8985_LIMDCY_WIDTH: u32 = 4;  /* LIMDCY - [7:4] */
pub const WM8985_LIMATK_MASK: u32 = 0x000F;  /* LIMATK - [3:0] */
pub const WM8985_LIMATK_SHIFT: u32 = 0;  /* LIMATK - [3:0] */
pub const WM8985_LIMATK_WIDTH: u32 = 4;  /* LIMATK - [3:0] */

/*
 * R25 (0x19) - DAC Limiter 2
 */
pub const WM8985_LIMLVL_MASK: u32 = 0x0070;  /* LIMLVL - [6:4] */
pub const WM8985_LIMLVL_SHIFT: u32 = 4;  /* LIMLVL - [6:4] */
pub const WM8985_LIMLVL_WIDTH: u32 = 3;  /* LIMLVL - [6:4] */
pub const WM8985_LIMBOOST_MASK: u32 = 0x000F;  /* LIMBOOST - [3:0] */
pub const WM8985_LIMBOOST_SHIFT: u32 = 0;  /* LIMBOOST - [3:0] */
pub const WM8985_LIMBOOST_WIDTH: u32 = 4;  /* LIMBOOST - [3:0] */

/*
 * R27 (0x1B) - Notch Filter 1
 */
pub const WM8985_NFU: u32 = 0x0100;  /* NFU */
pub const WM8985_NFU_MASK: u32 = 0x0100;  /* NFU */
pub const WM8985_NFU_SHIFT: u32 = 8;  /* NFU */
pub const WM8985_NFU_WIDTH: u32 = 1;  /* NFU */
pub const WM8985_NFEN: u32 = 0x0080;  /* NFEN */
pub const WM8985_NFEN_MASK: u32 = 0x0080;  /* NFEN */
pub const WM8985_NFEN_SHIFT: u32 = 7;  /* NFEN */
pub const WM8985_NFEN_WIDTH: u32 = 1;  /* NFEN */
pub const WM8985_NFA0_13_7_MASK: u32 = 0x007F;  /* NFA0(13:7) - [6:0] */
pub const WM8985_NFA0_13_7_SHIFT: u32 = 0;  /* NFA0(13:7) - [6:0] */
pub const WM8985_NFA0_13_7_WIDTH: u32 = 7;  /* NFA0(13:7) - [6:0] */

/*
 * R28 (0x1C) - Notch Filter 2
 */
pub const WM8985_NFA0_6_0_MASK: u32 = 0x007F;  /* NFA0(6:0) - [6:0] */
pub const WM8985_NFA0_6_0_SHIFT: u32 = 0;  /* NFA0(6:0) - [6:0] */
pub const WM8985_NFA0_6_0_WIDTH: u32 = 7;  /* NFA0(6:0) - [6:0] */

/*
 * R29 (0x1D) - Notch Filter 3
 */
pub const WM8985_NFA1_13_7_MASK: u32 = 0x007F;  /* NFA1(13:7) - [6:0] */
pub const WM8985_NFA1_13_7_SHIFT: u32 = 0;  /* NFA1(13:7) - [6:0] */
pub const WM8985_NFA1_13_7_WIDTH: u32 = 7;  /* NFA1(13:7) - [6:0] */

/*
 * R30 (0x1E) - Notch Filter 4
 */
pub const WM8985_NFA1_6_0_MASK: u32 = 0x007F;  /* NFA1(6:0) - [6:0] */
pub const WM8985_NFA1_6_0_SHIFT: u32 = 0;  /* NFA1(6:0) - [6:0] */
pub const WM8985_NFA1_6_0_WIDTH: u32 = 7;  /* NFA1(6:0) - [6:0] */

/*
 * R32 (0x20) - ALC control 1
 */
pub const WM8985_ALCSEL_MASK: u32 = 0x0180;  /* ALCSEL - [8:7] */
pub const WM8985_ALCSEL_SHIFT: u32 = 7;  /* ALCSEL - [8:7] */
pub const WM8985_ALCSEL_WIDTH: u32 = 2;  /* ALCSEL - [8:7] */
pub const WM8985_ALCMAX_MASK: u32 = 0x0038;  /* ALCMAX - [5:3] */
pub const WM8985_ALCMAX_SHIFT: u32 = 3;  /* ALCMAX - [5:3] */
pub const WM8985_ALCMAX_WIDTH: u32 = 3;  /* ALCMAX - [5:3] */
pub const WM8985_ALCMIN_MASK: u32 = 0x0007;  /* ALCMIN - [2:0] */
pub const WM8985_ALCMIN_SHIFT: u32 = 0;  /* ALCMIN - [2:0] */
pub const WM8985_ALCMIN_WIDTH: u32 = 3;  /* ALCMIN - [2:0] */

/*
 * R33 (0x21) - ALC control 2
 */
pub const WM8985_ALCHLD_MASK: u32 = 0x00F0;  /* ALCHLD - [7:4] */
pub const WM8985_ALCHLD_SHIFT: u32 = 4;  /* ALCHLD - [7:4] */
pub const WM8985_ALCHLD_WIDTH: u32 = 4;  /* ALCHLD - [7:4] */
pub const WM8985_ALCLVL_MASK: u32 = 0x000F;  /* ALCLVL - [3:0] */
pub const WM8985_ALCLVL_SHIFT: u32 = 0;  /* ALCLVL - [3:0] */
pub const WM8985_ALCLVL_WIDTH: u32 = 4;  /* ALCLVL - [3:0] */

/*
 * R34 (0x22) - ALC control 3
 */
pub const WM8985_ALCMODE: u32 = 0x0100;  /* ALCMODE */
pub const WM8985_ALCMODE_MASK: u32 = 0x0100;  /* ALCMODE */
pub const WM8985_ALCMODE_SHIFT: u32 = 8;  /* ALCMODE */
pub const WM8985_ALCMODE_WIDTH: u32 = 1;  /* ALCMODE */
pub const WM8985_ALCDCY_MASK: u32 = 0x00F0;  /* ALCDCY - [7:4] */
pub const WM8985_ALCDCY_SHIFT: u32 = 4;  /* ALCDCY - [7:4] */
pub const WM8985_ALCDCY_WIDTH: u32 = 4;  /* ALCDCY - [7:4] */
pub const WM8985_ALCATK_MASK: u32 = 0x000F;  /* ALCATK - [3:0] */
pub const WM8985_ALCATK_SHIFT: u32 = 0;  /* ALCATK - [3:0] */
pub const WM8985_ALCATK_WIDTH: u32 = 4;  /* ALCATK - [3:0] */

/*
 * R35 (0x23) - Noise Gate
 */
pub const WM8985_NGEN: u32 = 0x0008;  /* NGEN */
pub const WM8985_NGEN_MASK: u32 = 0x0008;  /* NGEN */
pub const WM8985_NGEN_SHIFT: u32 = 3;  /* NGEN */
pub const WM8985_NGEN_WIDTH: u32 = 1;  /* NGEN */
pub const WM8985_NGTH_MASK: u32 = 0x0007;  /* NGTH - [2:0] */
pub const WM8985_NGTH_SHIFT: u32 = 0;  /* NGTH - [2:0] */
pub const WM8985_NGTH_WIDTH: u32 = 3;  /* NGTH - [2:0] */

/*
 * R36 (0x24) - PLL N
 */
pub const WM8985_PLL_PRESCALE: u32 = 0x0010;  /* PLL_PRESCALE */
pub const WM8985_PLL_PRESCALE_MASK: u32 = 0x0010;  /* PLL_PRESCALE */
pub const WM8985_PLL_PRESCALE_SHIFT: u32 = 4;  /* PLL_PRESCALE */
pub const WM8985_PLL_PRESCALE_WIDTH: u32 = 1;  /* PLL_PRESCALE */
pub const WM8985_PLLN_MASK: u32 = 0x000F;  /* PLLN - [3:0] */
pub const WM8985_PLLN_SHIFT: u32 = 0;  /* PLLN - [3:0] */
pub const WM8985_PLLN_WIDTH: u32 = 4;  /* PLLN - [3:0] */

/*
 * R37 (0x25) - PLL K 1
 */
pub const WM8985_PLLK_23_18_MASK: u32 = 0x003F;  /* PLLK(23:18) - [5:0] */
pub const WM8985_PLLK_23_18_SHIFT: u32 = 0;  /* PLLK(23:18) - [5:0] */
pub const WM8985_PLLK_23_18_WIDTH: u32 = 6;  /* PLLK(23:18) - [5:0] */

/*
 * R38 (0x26) - PLL K 2
 */
pub const WM8985_PLLK_17_9_MASK: u32 = 0x01FF;  /* PLLK(17:9) - [8:0] */
pub const WM8985_PLLK_17_9_SHIFT: u32 = 0;  /* PLLK(17:9) - [8:0] */
pub const WM8985_PLLK_17_9_WIDTH: u32 = 9;  /* PLLK(17:9) - [8:0] */

/*
 * R39 (0x27) - PLL K 3
 */
pub const WM8985_PLLK_8_0_MASK: u32 = 0x01FF;  /* PLLK(8:0) - [8:0] */
pub const WM8985_PLLK_8_0_SHIFT: u32 = 0;  /* PLLK(8:0) - [8:0] */
pub const WM8985_PLLK_8_0_WIDTH: u32 = 9;  /* PLLK(8:0) - [8:0] */

/*
 * R41 (0x29) - 3D control
 */
pub const WM8985_DEPTH3D_MASK: u32 = 0x000F;  /* DEPTH3D - [3:0] */
pub const WM8985_DEPTH3D_SHIFT: u32 = 0;  /* DEPTH3D - [3:0] */
pub const WM8985_DEPTH3D_WIDTH: u32 = 4;  /* DEPTH3D - [3:0] */

/*
 * R42 (0x2A) - OUT4 to ADC
 */
pub const WM8985_OUT4_2ADCVOL_MASK: u32 = 0x01C0;  /* OUT4_2ADCVOL - [8:6] */
pub const WM8985_OUT4_2ADCVOL_SHIFT: u32 = 6;  /* OUT4_2ADCVOL - [8:6] */
pub const WM8985_OUT4_2ADCVOL_WIDTH: u32 = 3;  /* OUT4_2ADCVOL - [8:6] */
pub const WM8985_OUT4_2LNR: u32 = 0x0020;  /* OUT4_2LNR */
pub const WM8985_OUT4_2LNR_MASK: u32 = 0x0020;  /* OUT4_2LNR */
pub const WM8985_OUT4_2LNR_SHIFT: u32 = 5;  /* OUT4_2LNR */
pub const WM8985_OUT4_2LNR_WIDTH: u32 = 1;  /* OUT4_2LNR */
pub const WM8758_VMIDTOG_MASK: u32 = 0x0010;  /* VMIDTOG */
pub const WM8758_VMIDTOG_SHIFT: u32 = 4;  /* VMIDTOG */
pub const WM8758_VMIDTOG_WIDTH: u32 = 1;  /* VMIDTOG */
pub const WM8758_OUT2DEL_MASK: u32 = 0x0008;  /* OUT2DEL */
pub const WM8758_OUT2DEL_SHIFT: u32 = 3;  /* OUT2DEL */
pub const WM8758_OUT2DEL_WIDTH: u32 = 1;  /* OUT2DEL */
pub const WM8985_POBCTRL: u32 = 0x0004;  /* POBCTRL */
pub const WM8985_POBCTRL_MASK: u32 = 0x0004;  /* POBCTRL */
pub const WM8985_POBCTRL_SHIFT: u32 = 2;  /* POBCTRL */
pub const WM8985_POBCTRL_WIDTH: u32 = 1;  /* POBCTRL */
pub const WM8985_DELEN: u32 = 0x0002;  /* DELEN */
pub const WM8985_DELEN_MASK: u32 = 0x0002;  /* DELEN */
pub const WM8985_DELEN_SHIFT: u32 = 1;  /* DELEN */
pub const WM8985_DELEN_WIDTH: u32 = 1;  /* DELEN */
pub const WM8985_OUT1DEL: u32 = 0x0001;  /* OUT1DEL */
pub const WM8985_OUT1DEL_MASK: u32 = 0x0001;  /* OUT1DEL */
pub const WM8985_OUT1DEL_SHIFT: u32 = 0;  /* OUT1DEL */
pub const WM8985_OUT1DEL_WIDTH: u32 = 1;  /* OUT1DEL */

/*
 * R43 (0x2B) - Beep control
 */
pub const WM8985_BYPL2RMIX: u32 = 0x0100;  /* BYPL2RMIX */
pub const WM8985_BYPL2RMIX_MASK: u32 = 0x0100;  /* BYPL2RMIX */
pub const WM8985_BYPL2RMIX_SHIFT: u32 = 8;  /* BYPL2RMIX */
pub const WM8985_BYPL2RMIX_WIDTH: u32 = 1;  /* BYPL2RMIX */
pub const WM8985_BYPR2LMIX: u32 = 0x0080;  /* BYPR2LMIX */
pub const WM8985_BYPR2LMIX_MASK: u32 = 0x0080;  /* BYPR2LMIX */
pub const WM8985_BYPR2LMIX_SHIFT: u32 = 7;  /* BYPR2LMIX */
pub const WM8985_BYPR2LMIX_WIDTH: u32 = 1;  /* BYPR2LMIX */
pub const WM8985_MUTERPGA2INV: u32 = 0x0020;  /* MUTERPGA2INV */
pub const WM8985_MUTERPGA2INV_MASK: u32 = 0x0020;  /* MUTERPGA2INV */
pub const WM8985_MUTERPGA2INV_SHIFT: u32 = 5;  /* MUTERPGA2INV */
pub const WM8985_MUTERPGA2INV_WIDTH: u32 = 1;  /* MUTERPGA2INV */
pub const WM8985_INVROUT2: u32 = 0x0010;  /* INVROUT2 */
pub const WM8985_INVROUT2_MASK: u32 = 0x0010;  /* INVROUT2 */
pub const WM8985_INVROUT2_SHIFT: u32 = 4;  /* INVROUT2 */
pub const WM8985_INVROUT2_WIDTH: u32 = 1;  /* INVROUT2 */
pub const WM8985_BEEPVOL_MASK: u32 = 0x000E;  /* BEEPVOL - [3:1] */
pub const WM8985_BEEPVOL_SHIFT: u32 = 1;  /* BEEPVOL - [3:1] */
pub const WM8985_BEEPVOL_WIDTH: u32 = 3;  /* BEEPVOL - [3:1] */
pub const WM8758_DELEN2_MASK: u32 = 0x0004;  /* DELEN2 */
pub const WM8758_DELEN2_SHIFT: u32 = 2;  /* DELEN2 */
pub const WM8758_DELEN2_WIDTH: u32 = 1;  /* DELEN2 */
pub const WM8985_BEEPEN: u32 = 0x0001;  /* BEEPEN */
pub const WM8985_BEEPEN_MASK: u32 = 0x0001;  /* BEEPEN */
pub const WM8985_BEEPEN_SHIFT: u32 = 0;  /* BEEPEN */
pub const WM8985_BEEPEN_WIDTH: u32 = 1;  /* BEEPEN */

/*
 * R44 (0x2C) - Input ctrl
 */
pub const WM8985_MBVSEL: u32 = 0x0100;  /* MBVSEL */
pub const WM8985_MBVSEL_MASK: u32 = 0x0100;  /* MBVSEL */
pub const WM8985_MBVSEL_SHIFT: u32 = 8;  /* MBVSEL */
pub const WM8985_MBVSEL_WIDTH: u32 = 1;  /* MBVSEL */
pub const WM8985_R2_2INPPGA: u32 = 0x0040;  /* R2_2INPPGA */
pub const WM8985_R2_2INPPGA_MASK: u32 = 0x0040;  /* R2_2INPPGA */
pub const WM8985_R2_2INPPGA_SHIFT: u32 = 6;  /* R2_2INPPGA */
pub const WM8985_R2_2INPPGA_WIDTH: u32 = 1;  /* R2_2INPPGA */
pub const WM8985_RIN2INPPGA: u32 = 0x0020;  /* RIN2INPPGA */
pub const WM8985_RIN2INPPGA_MASK: u32 = 0x0020;  /* RIN2INPPGA */
pub const WM8985_RIN2INPPGA_SHIFT: u32 = 5;  /* RIN2INPPGA */
pub const WM8985_RIN2INPPGA_WIDTH: u32 = 1;  /* RIN2INPPGA */
pub const WM8985_RIP2INPPGA: u32 = 0x0010;  /* RIP2INPPGA */
pub const WM8985_RIP2INPPGA_MASK: u32 = 0x0010;  /* RIP2INPPGA */
pub const WM8985_RIP2INPPGA_SHIFT: u32 = 4;  /* RIP2INPPGA */
pub const WM8985_RIP2INPPGA_WIDTH: u32 = 1;  /* RIP2INPPGA */
pub const WM8985_L2_2INPPGA: u32 = 0x0004;  /* L2_2INPPGA */
pub const WM8985_L2_2INPPGA_MASK: u32 = 0x0004;  /* L2_2INPPGA */
pub const WM8985_L2_2INPPGA_SHIFT: u32 = 2;  /* L2_2INPPGA */
pub const WM8985_L2_2INPPGA_WIDTH: u32 = 1;  /* L2_2INPPGA */
pub const WM8985_LIN2INPPGA: u32 = 0x0002;  /* LIN2INPPGA */
pub const WM8985_LIN2INPPGA_MASK: u32 = 0x0002;  /* LIN2INPPGA */
pub const WM8985_LIN2INPPGA_SHIFT: u32 = 1;  /* LIN2INPPGA */
pub const WM8985_LIN2INPPGA_WIDTH: u32 = 1;  /* LIN2INPPGA */
pub const WM8985_LIP2INPPGA: u32 = 0x0001;  /* LIP2INPPGA */
pub const WM8985_LIP2INPPGA_MASK: u32 = 0x0001;  /* LIP2INPPGA */
pub const WM8985_LIP2INPPGA_SHIFT: u32 = 0;  /* LIP2INPPGA */
pub const WM8985_LIP2INPPGA_WIDTH: u32 = 1;  /* LIP2INPPGA */

/*
 * R45 (0x2D) - Left INP PGA gain ctrl
 */
pub const WM8985_INPGAVU: u32 = 0x0100;  /* INPGAVU */
pub const WM8985_INPGAVU_MASK: u32 = 0x0100;  /* INPGAVU */
pub const WM8985_INPGAVU_SHIFT: u32 = 8;  /* INPGAVU */
pub const WM8985_INPGAVU_WIDTH: u32 = 1;  /* INPGAVU */
pub const WM8985_INPPGAZCL: u32 = 0x0080;  /* INPPGAZCL */
pub const WM8985_INPPGAZCL_MASK: u32 = 0x0080;  /* INPPGAZCL */
pub const WM8985_INPPGAZCL_SHIFT: u32 = 7;  /* INPPGAZCL */
pub const WM8985_INPPGAZCL_WIDTH: u32 = 1;  /* INPPGAZCL */
pub const WM8985_INPPGAMUTEL: u32 = 0x0040;  /* INPPGAMUTEL */
pub const WM8985_INPPGAMUTEL_MASK: u32 = 0x0040;  /* INPPGAMUTEL */
pub const WM8985_INPPGAMUTEL_SHIFT: u32 = 6;  /* INPPGAMUTEL */
pub const WM8985_INPPGAMUTEL_WIDTH: u32 = 1;  /* INPPGAMUTEL */
pub const WM8985_INPPGAVOLL_MASK: u32 = 0x003F;  /* INPPGAVOLL - [5:0] */
pub const WM8985_INPPGAVOLL_SHIFT: u32 = 0;  /* INPPGAVOLL - [5:0] */
pub const WM8985_INPPGAVOLL_WIDTH: u32 = 6;  /* INPPGAVOLL - [5:0] */

/*
 * R46 (0x2E) - Right INP PGA gain ctrl
 */
pub const WM8985_INPPGAZCR: u32 = 0x0080;  /* INPPGAZCR */
pub const WM8985_INPPGAZCR_MASK: u32 = 0x0080;  /* INPPGAZCR */
pub const WM8985_INPPGAZCR_SHIFT: u32 = 7;  /* INPPGAZCR */
pub const WM8985_INPPGAZCR_WIDTH: u32 = 1;  /* INPPGAZCR */
pub const WM8985_INPPGAMUTER: u32 = 0x0040;  /* INPPGAMUTER */
pub const WM8985_INPPGAMUTER_MASK: u32 = 0x0040;  /* INPPGAMUTER */
pub const WM8985_INPPGAMUTER_SHIFT: u32 = 6;  /* INPPGAMUTER */
pub const WM8985_INPPGAMUTER_WIDTH: u32 = 1;  /* INPPGAMUTER */
pub const WM8985_INPPGAVOLR_MASK: u32 = 0x003F;  /* INPPGAVOLR - [5:0] */
pub const WM8985_INPPGAVOLR_SHIFT: u32 = 0;  /* INPPGAVOLR - [5:0] */
pub const WM8985_INPPGAVOLR_WIDTH: u32 = 6;  /* INPPGAVOLR - [5:0] */

/*
 * R47 (0x2F) - Left ADC BOOST ctrl
 */
pub const WM8985_PGABOOSTL: u32 = 0x0100;  /* PGABOOSTL */
pub const WM8985_PGABOOSTL_MASK: u32 = 0x0100;  /* PGABOOSTL */
pub const WM8985_PGABOOSTL_SHIFT: u32 = 8;  /* PGABOOSTL */
pub const WM8985_PGABOOSTL_WIDTH: u32 = 1;  /* PGABOOSTL */
pub const WM8985_L2_2BOOSTVOL_MASK: u32 = 0x0070;  /* L2_2BOOSTVOL - [6:4] */
pub const WM8985_L2_2BOOSTVOL_SHIFT: u32 = 4;  /* L2_2BOOSTVOL - [6:4] */
pub const WM8985_L2_2BOOSTVOL_WIDTH: u32 = 3;  /* L2_2BOOSTVOL - [6:4] */
pub const WM8985_AUXL2BOOSTVOL_MASK: u32 = 0x0007;  /* AUXL2BOOSTVOL - [2:0] */
pub const WM8985_AUXL2BOOSTVOL_SHIFT: u32 = 0;  /* AUXL2BOOSTVOL - [2:0] */
pub const WM8985_AUXL2BOOSTVOL_WIDTH: u32 = 3;  /* AUXL2BOOSTVOL - [2:0] */

/*
 * R48 (0x30) - Right ADC BOOST ctrl
 */
pub const WM8985_PGABOOSTR: u32 = 0x0100;  /* PGABOOSTR */
pub const WM8985_PGABOOSTR_MASK: u32 = 0x0100;  /* PGABOOSTR */
pub const WM8985_PGABOOSTR_SHIFT: u32 = 8;  /* PGABOOSTR */
pub const WM8985_PGABOOSTR_WIDTH: u32 = 1;  /* PGABOOSTR */
pub const WM8985_R2_2BOOSTVOL_MASK: u32 = 0x0070;  /* R2_2BOOSTVOL - [6:4] */
pub const WM8985_R2_2BOOSTVOL_SHIFT: u32 = 4;  /* R2_2BOOSTVOL - [6:4] */
pub const WM8985_R2_2BOOSTVOL_WIDTH: u32 = 3;  /* R2_2BOOSTVOL - [6:4] */
pub const WM8985_AUXR2BOOSTVOL_MASK: u32 = 0x0007;  /* AUXR2BOOSTVOL - [2:0] */
pub const WM8985_AUXR2BOOSTVOL_SHIFT: u32 = 0;  /* AUXR2BOOSTVOL - [2:0] */
pub const WM8985_AUXR2BOOSTVOL_WIDTH: u32 = 3;  /* AUXR2BOOSTVOL - [2:0] */

/*
 * R49 (0x31) - Output ctrl
 */
pub const WM8758_HP_COM: u32 = 0x0100;  /* HP_COM */
pub const WM8758_HP_COM_MASK: u32 = 0x0100;  /* HP_COM */
pub const WM8758_HP_COM_SHIFT: u32 = 8;  /* HP_COM */
pub const WM8758_HP_COM_WIDTH: u32 = 1;  /* HP_COM */
pub const WM8758_LINE_COM: u32 = 0x0080;  /* LINE_COM */
pub const WM8758_LINE_COM_MASK: u32 = 0x0080;  /* LINE_COM */
pub const WM8758_LINE_COM_SHIFT: u32 = 7;  /* LINE_COM */
pub const WM8758_LINE_COM_WIDTH: u32 = 1;  /* LINE_COM */
pub const WM8985_DACL2RMIX: u32 = 0x0040;  /* DACL2RMIX */
pub const WM8985_DACL2RMIX_MASK: u32 = 0x0040;  /* DACL2RMIX */
pub const WM8985_DACL2RMIX_SHIFT: u32 = 6;  /* DACL2RMIX */
pub const WM8985_DACL2RMIX_WIDTH: u32 = 1;  /* DACL2RMIX */
pub const WM8985_DACR2LMIX: u32 = 0x0020;  /* DACR2LMIX */
pub const WM8985_DACR2LMIX_MASK: u32 = 0x0020;  /* DACR2LMIX */
pub const WM8985_DACR2LMIX_SHIFT: u32 = 5;  /* DACR2LMIX */
pub const WM8985_DACR2LMIX_WIDTH: u32 = 1;  /* DACR2LMIX */
pub const WM8985_OUT4BOOST: u32 = 0x0010;  /* OUT4BOOST */
pub const WM8985_OUT4BOOST_MASK: u32 = 0x0010;  /* OUT4BOOST */
pub const WM8985_OUT4BOOST_SHIFT: u32 = 4;  /* OUT4BOOST */
pub const WM8985_OUT4BOOST_WIDTH: u32 = 1;  /* OUT4BOOST */
pub const WM8985_OUT3BOOST: u32 = 0x0008;  /* OUT3BOOST */
pub const WM8985_OUT3BOOST_MASK: u32 = 0x0008;  /* OUT3BOOST */
pub const WM8985_OUT3BOOST_SHIFT: u32 = 3;  /* OUT3BOOST */
pub const WM8985_OUT3BOOST_WIDTH: u32 = 1;  /* OUT3BOOST */
pub const WM8758_OUT4ENDEL: u32 = 0x0010;  /* OUT4ENDEL */
pub const WM8758_OUT4ENDEL_MASK: u32 = 0x0010;  /* OUT4ENDEL */
pub const WM8758_OUT4ENDEL_SHIFT: u32 = 4;  /* OUT4ENDEL */
pub const WM8758_OUT4ENDEL_WIDTH: u32 = 1;  /* OUT4ENDEL */
pub const WM8758_OUT3ENDEL: u32 = 0x0008;  /* OUT3ENDEL */
pub const WM8758_OUT3ENDEL_MASK: u32 = 0x0008;  /* OUT3ENDEL */
pub const WM8758_OUT3ENDEL_SHIFT: u32 = 3;  /* OUT3ENDEL */
pub const WM8758_OUT3ENDEL_WIDTH: u32 = 1;  /* OUT3ENDEL */
pub const WM8985_TSOPCTRL: u32 = 0x0004;  /* TSOPCTRL */
pub const WM8985_TSOPCTRL_MASK: u32 = 0x0004;  /* TSOPCTRL */
pub const WM8985_TSOPCTRL_SHIFT: u32 = 2;  /* TSOPCTRL */
pub const WM8985_TSOPCTRL_WIDTH: u32 = 1;  /* TSOPCTRL */
pub const WM8985_TSDEN: u32 = 0x0002;  /* TSDEN */
pub const WM8985_TSDEN_MASK: u32 = 0x0002;  /* TSDEN */
pub const WM8985_TSDEN_SHIFT: u32 = 1;  /* TSDEN */
pub const WM8985_TSDEN_WIDTH: u32 = 1;  /* TSDEN */
pub const WM8985_VROI: u32 = 0x0001;  /* VROI */
pub const WM8985_VROI_MASK: u32 = 0x0001;  /* VROI */
pub const WM8985_VROI_SHIFT: u32 = 0;  /* VROI */
pub const WM8985_VROI_WIDTH: u32 = 1;  /* VROI */

/*
 * R50 (0x32) - Left mixer ctrl
 */
pub const WM8985_AUXLMIXVOL_MASK: u32 = 0x01C0;  /* AUXLMIXVOL - [8:6] */
pub const WM8985_AUXLMIXVOL_SHIFT: u32 = 6;  /* AUXLMIXVOL - [8:6] */
pub const WM8985_AUXLMIXVOL_WIDTH: u32 = 3;  /* AUXLMIXVOL - [8:6] */
pub const WM8985_AUXL2LMIX: u32 = 0x0020;  /* AUXL2LMIX */
pub const WM8985_AUXL2LMIX_MASK: u32 = 0x0020;  /* AUXL2LMIX */
pub const WM8985_AUXL2LMIX_SHIFT: u32 = 5;  /* AUXL2LMIX */
pub const WM8985_AUXL2LMIX_WIDTH: u32 = 1;  /* AUXL2LMIX */
pub const WM8985_BYPLMIXVOL_MASK: u32 = 0x001C;  /* BYPLMIXVOL - [4:2] */
pub const WM8985_BYPLMIXVOL_SHIFT: u32 = 2;  /* BYPLMIXVOL - [4:2] */
pub const WM8985_BYPLMIXVOL_WIDTH: u32 = 3;  /* BYPLMIXVOL - [4:2] */
pub const WM8985_BYPL2LMIX: u32 = 0x0002;  /* BYPL2LMIX */
pub const WM8985_BYPL2LMIX_MASK: u32 = 0x0002;  /* BYPL2LMIX */
pub const WM8985_BYPL2LMIX_SHIFT: u32 = 1;  /* BYPL2LMIX */
pub const WM8985_BYPL2LMIX_WIDTH: u32 = 1;  /* BYPL2LMIX */
pub const WM8985_DACL2LMIX: u32 = 0x0001;  /* DACL2LMIX */
pub const WM8985_DACL2LMIX_MASK: u32 = 0x0001;  /* DACL2LMIX */
pub const WM8985_DACL2LMIX_SHIFT: u32 = 0;  /* DACL2LMIX */
pub const WM8985_DACL2LMIX_WIDTH: u32 = 1;  /* DACL2LMIX */

/*
 * R51 (0x33) - Right mixer ctrl
 */
pub const WM8985_AUXRMIXVOL_MASK: u32 = 0x01C0;  /* AUXRMIXVOL - [8:6] */
pub const WM8985_AUXRMIXVOL_SHIFT: u32 = 6;  /* AUXRMIXVOL - [8:6] */
pub const WM8985_AUXRMIXVOL_WIDTH: u32 = 3;  /* AUXRMIXVOL - [8:6] */
pub const WM8985_AUXR2RMIX: u32 = 0x0020;  /* AUXR2RMIX */
pub const WM8985_AUXR2RMIX_MASK: u32 = 0x0020;  /* AUXR2RMIX */
pub const WM8985_AUXR2RMIX_SHIFT: u32 = 5;  /* AUXR2RMIX */
pub const WM8985_AUXR2RMIX_WIDTH: u32 = 1;  /* AUXR2RMIX */
pub const WM8985_BYPRMIXVOL_MASK: u32 = 0x001C;  /* BYPRMIXVOL - [4:2] */
pub const WM8985_BYPRMIXVOL_SHIFT: u32 = 2;  /* BYPRMIXVOL - [4:2] */
pub const WM8985_BYPRMIXVOL_WIDTH: u32 = 3;  /* BYPRMIXVOL - [4:2] */
pub const WM8985_BYPR2RMIX: u32 = 0x0002;  /* BYPR2RMIX */
pub const WM8985_BYPR2RMIX_MASK: u32 = 0x0002;  /* BYPR2RMIX */
pub const WM8985_BYPR2RMIX_SHIFT: u32 = 1;  /* BYPR2RMIX */
pub const WM8985_BYPR2RMIX_WIDTH: u32 = 1;  /* BYPR2RMIX */
pub const WM8985_DACR2RMIX: u32 = 0x0001;  /* DACR2RMIX */
pub const WM8985_DACR2RMIX_MASK: u32 = 0x0001;  /* DACR2RMIX */
pub const WM8985_DACR2RMIX_SHIFT: u32 = 0;  /* DACR2RMIX */
pub const WM8985_DACR2RMIX_WIDTH: u32 = 1;  /* DACR2RMIX */

/*
 * R52 (0x34) - LOUT1 (HP) volume ctrl
 */
pub const WM8985_OUT1VU: u32 = 0x0100;  /* OUT1VU */
pub const WM8985_OUT1VU_MASK: u32 = 0x0100;  /* OUT1VU */
pub const WM8985_OUT1VU_SHIFT: u32 = 8;  /* OUT1VU */
pub const WM8985_OUT1VU_WIDTH: u32 = 1;  /* OUT1VU */
pub const WM8985_LOUT1ZC: u32 = 0x0080;  /* LOUT1ZC */
pub const WM8985_LOUT1ZC_MASK: u32 = 0x0080;  /* LOUT1ZC */
pub const WM8985_LOUT1ZC_SHIFT: u32 = 7;  /* LOUT1ZC */
pub const WM8985_LOUT1ZC_WIDTH: u32 = 1;  /* LOUT1ZC */
pub const WM8985_LOUT1MUTE: u32 = 0x0040;  /* LOUT1MUTE */
pub const WM8985_LOUT1MUTE_MASK: u32 = 0x0040;  /* LOUT1MUTE */
pub const WM8985_LOUT1MUTE_SHIFT: u32 = 6;  /* LOUT1MUTE */
pub const WM8985_LOUT1MUTE_WIDTH: u32 = 1;  /* LOUT1MUTE */
pub const WM8985_LOUT1VOL_MASK: u32 = 0x003F;  /* LOUT1VOL - [5:0] */
pub const WM8985_LOUT1VOL_SHIFT: u32 = 0;  /* LOUT1VOL - [5:0] */
pub const WM8985_LOUT1VOL_WIDTH: u32 = 6;  /* LOUT1VOL - [5:0] */

/*
 * R53 (0x35) - ROUT1 (HP) volume ctrl
 */
pub const WM8985_ROUT1ZC: u32 = 0x0080;  /* ROUT1ZC */
pub const WM8985_ROUT1ZC_MASK: u32 = 0x0080;  /* ROUT1ZC */
pub const WM8985_ROUT1ZC_SHIFT: u32 = 7;  /* ROUT1ZC */
pub const WM8985_ROUT1ZC_WIDTH: u32 = 1;  /* ROUT1ZC */
pub const WM8985_ROUT1MUTE: u32 = 0x0040;  /* ROUT1MUTE */
pub const WM8985_ROUT1MUTE_MASK: u32 = 0x0040;  /* ROUT1MUTE */
pub const WM8985_ROUT1MUTE_SHIFT: u32 = 6;  /* ROUT1MUTE */
pub const WM8985_ROUT1MUTE_WIDTH: u32 = 1;  /* ROUT1MUTE */
pub const WM8985_ROUT1VOL_MASK: u32 = 0x003F;  /* ROUT1VOL - [5:0] */
pub const WM8985_ROUT1VOL_SHIFT: u32 = 0;  /* ROUT1VOL - [5:0] */
pub const WM8985_ROUT1VOL_WIDTH: u32 = 6;  /* ROUT1VOL - [5:0] */

/*
 * R54 (0x36) - LOUT2 (SPK) volume ctrl
 */
pub const WM8985_OUT2VU: u32 = 0x0100;  /* OUT2VU */
pub const WM8985_OUT2VU_MASK: u32 = 0x0100;  /* OUT2VU */
pub const WM8985_OUT2VU_SHIFT: u32 = 8;  /* OUT2VU */
pub const WM8985_OUT2VU_WIDTH: u32 = 1;  /* OUT2VU */
pub const WM8985_LOUT2ZC: u32 = 0x0080;  /* LOUT2ZC */
pub const WM8985_LOUT2ZC_MASK: u32 = 0x0080;  /* LOUT2ZC */
pub const WM8985_LOUT2ZC_SHIFT: u32 = 7;  /* LOUT2ZC */
pub const WM8985_LOUT2ZC_WIDTH: u32 = 1;  /* LOUT2ZC */
pub const WM8985_LOUT2MUTE: u32 = 0x0040;  /* LOUT2MUTE */
pub const WM8985_LOUT2MUTE_MASK: u32 = 0x0040;  /* LOUT2MUTE */
pub const WM8985_LOUT2MUTE_SHIFT: u32 = 6;  /* LOUT2MUTE */
pub const WM8985_LOUT2MUTE_WIDTH: u32 = 1;  /* LOUT2MUTE */
pub const WM8985_LOUT2VOL_MASK: u32 = 0x003F;  /* LOUT2VOL - [5:0] */
pub const WM8985_LOUT2VOL_SHIFT: u32 = 0;  /* LOUT2VOL - [5:0] */
pub const WM8985_LOUT2VOL_WIDTH: u32 = 6;  /* LOUT2VOL - [5:0] */

/*
 * R55 (0x37) - ROUT2 (SPK) volume ctrl
 */
pub const WM8985_ROUT2ZC: u32 = 0x0080;  /* ROUT2ZC */
pub const WM8985_ROUT2ZC_MASK: u32 = 0x0080;  /* ROUT2ZC */
pub const WM8985_ROUT2ZC_SHIFT: u32 = 7;  /* ROUT2ZC */
pub const WM8985_ROUT2ZC_WIDTH: u32 = 1;  /* ROUT2ZC */
pub const WM8985_ROUT2MUTE: u32 = 0x0040;  /* ROUT2MUTE */
pub const WM8985_ROUT2MUTE_MASK: u32 = 0x0040;  /* ROUT2MUTE */
pub const WM8985_ROUT2MUTE_SHIFT: u32 = 6;  /* ROUT2MUTE */
pub const WM8985_ROUT2MUTE_WIDTH: u32 = 1;  /* ROUT2MUTE */
pub const WM8985_ROUT2VOL_MASK: u32 = 0x003F;  /* ROUT2VOL - [5:0] */
pub const WM8985_ROUT2VOL_SHIFT: u32 = 0;  /* ROUT2VOL - [5:0] */
pub const WM8985_ROUT2VOL_WIDTH: u32 = 6;  /* ROUT2VOL - [5:0] */

/*
 * R56 (0x38) - OUT3 mixer ctrl
 */
pub const WM8985_OUT3MUTE: u32 = 0x0040;  /* OUT3MUTE */
pub const WM8985_OUT3MUTE_MASK: u32 = 0x0040;  /* OUT3MUTE */
pub const WM8985_OUT3MUTE_SHIFT: u32 = 6;  /* OUT3MUTE */
pub const WM8985_OUT3MUTE_WIDTH: u32 = 1;  /* OUT3MUTE */
pub const WM8985_OUT4_2OUT3: u32 = 0x0008;  /* OUT4_2OUT3 */
pub const WM8985_OUT4_2OUT3_MASK: u32 = 0x0008;  /* OUT4_2OUT3 */
pub const WM8985_OUT4_2OUT3_SHIFT: u32 = 3;  /* OUT4_2OUT3 */
pub const WM8985_OUT4_2OUT3_WIDTH: u32 = 1;  /* OUT4_2OUT3 */
pub const WM8985_BYPL2OUT3: u32 = 0x0004;  /* BYPL2OUT3 */
pub const WM8985_BYPL2OUT3_MASK: u32 = 0x0004;  /* BYPL2OUT3 */
pub const WM8985_BYPL2OUT3_SHIFT: u32 = 2;  /* BYPL2OUT3 */
pub const WM8985_BYPL2OUT3_WIDTH: u32 = 1;  /* BYPL2OUT3 */
pub const WM8985_LMIX2OUT3: u32 = 0x0002;  /* LMIX2OUT3 */
pub const WM8985_LMIX2OUT3_MASK: u32 = 0x0002;  /* LMIX2OUT3 */
pub const WM8985_LMIX2OUT3_SHIFT: u32 = 1;  /* LMIX2OUT3 */
pub const WM8985_LMIX2OUT3_WIDTH: u32 = 1;  /* LMIX2OUT3 */
pub const WM8985_LDAC2OUT3: u32 = 0x0001;  /* LDAC2OUT3 */
pub const WM8985_LDAC2OUT3_MASK: u32 = 0x0001;  /* LDAC2OUT3 */
pub const WM8985_LDAC2OUT3_SHIFT: u32 = 0;  /* LDAC2OUT3 */
pub const WM8985_LDAC2OUT3_WIDTH: u32 = 1;  /* LDAC2OUT3 */

/*
 * R57 (0x39) - OUT4 (MONO) mix ctrl
 */
pub const WM8985_OUT3_2OUT4: u32 = 0x0080;  /* OUT3_2OUT4 */
pub const WM8985_OUT3_2OUT4_MASK: u32 = 0x0080;  /* OUT3_2OUT4 */
pub const WM8985_OUT3_2OUT4_SHIFT: u32 = 7;  /* OUT3_2OUT4 */
pub const WM8985_OUT3_2OUT4_WIDTH: u32 = 1;  /* OUT3_2OUT4 */
pub const WM8985_OUT4MUTE: u32 = 0x0040;  /* OUT4MUTE */
pub const WM8985_OUT4MUTE_MASK: u32 = 0x0040;  /* OUT4MUTE */
pub const WM8985_OUT4MUTE_SHIFT: u32 = 6;  /* OUT4MUTE */
pub const WM8985_OUT4MUTE_WIDTH: u32 = 1;  /* OUT4MUTE */
pub const WM8985_OUT4ATTN: u32 = 0x0020;  /* OUT4ATTN */
pub const WM8985_OUT4ATTN_MASK: u32 = 0x0020;  /* OUT4ATTN */
pub const WM8985_OUT4ATTN_SHIFT: u32 = 5;  /* OUT4ATTN */
pub const WM8985_OUT4ATTN_WIDTH: u32 = 1;  /* OUT4ATTN */
pub const WM8985_LMIX2OUT4: u32 = 0x0010;  /* LMIX2OUT4 */
pub const WM8985_LMIX2OUT4_MASK: u32 = 0x0010;  /* LMIX2OUT4 */
pub const WM8985_LMIX2OUT4_SHIFT: u32 = 4;  /* LMIX2OUT4 */
pub const WM8985_LMIX2OUT4_WIDTH: u32 = 1;  /* LMIX2OUT4 */
pub const WM8985_LDAC2OUT4: u32 = 0x0008;  /* LDAC2OUT4 */
pub const WM8985_LDAC2OUT4_MASK: u32 = 0x0008;  /* LDAC2OUT4 */
pub const WM8985_LDAC2OUT4_SHIFT: u32 = 3;  /* LDAC2OUT4 */
pub const WM8985_LDAC2OUT4_WIDTH: u32 = 1;  /* LDAC2OUT4 */
pub const WM8985_BYPR2OUT4: u32 = 0x0004;  /* BYPR2OUT4 */
pub const WM8985_BYPR2OUT4_MASK: u32 = 0x0004;  /* BYPR2OUT4 */
pub const WM8985_BYPR2OUT4_SHIFT: u32 = 2;  /* BYPR2OUT4 */
pub const WM8985_BYPR2OUT4_WIDTH: u32 = 1;  /* BYPR2OUT4 */
pub const WM8985_RMIX2OUT4: u32 = 0x0002;  /* RMIX2OUT4 */
pub const WM8985_RMIX2OUT4_MASK: u32 = 0x0002;  /* RMIX2OUT4 */
pub const WM8985_RMIX2OUT4_SHIFT: u32 = 1;  /* RMIX2OUT4 */
pub const WM8985_RMIX2OUT4_WIDTH: u32 = 1;  /* RMIX2OUT4 */
pub const WM8985_RDAC2OUT4: u32 = 0x0001;  /* RDAC2OUT4 */
pub const WM8985_RDAC2OUT4_MASK: u32 = 0x0001;  /* RDAC2OUT4 */
pub const WM8985_RDAC2OUT4_SHIFT: u32 = 0;  /* RDAC2OUT4 */
pub const WM8985_RDAC2OUT4_WIDTH: u32 = 1;  /* RDAC2OUT4 */

/*
 * R60 (0x3C) - OUTPUT ctrl
 */
pub const WM8985_VIDBUFFTST_MASK: u32 = 0x01E0;  /* VIDBUFFTST - [8:5] */
pub const WM8985_VIDBUFFTST_SHIFT: u32 = 5;  /* VIDBUFFTST - [8:5] */
pub const WM8985_VIDBUFFTST_WIDTH: u32 = 4;  /* VIDBUFFTST - [8:5] */
pub const WM8985_HPTOG: u32 = 0x0008;  /* HPTOG */
pub const WM8985_HPTOG_MASK: u32 = 0x0008;  /* HPTOG */
pub const WM8985_HPTOG_SHIFT: u32 = 3;  /* HPTOG */
pub const WM8985_HPTOG_WIDTH: u32 = 1;  /* HPTOG */

/*
 * R61 (0x3D) - BIAS CTRL
 */
pub const WM8985_BIASCUT: u32 = 0x0100;  /* BIASCUT */
pub const WM8985_BIASCUT_MASK: u32 = 0x0100;  /* BIASCUT */
pub const WM8985_BIASCUT_SHIFT: u32 = 8;  /* BIASCUT */
pub const WM8985_BIASCUT_WIDTH: u32 = 1;  /* BIASCUT */
pub const WM8985_HALFIPBIAS: u32 = 0x0080;  /* HALFIPBIAS */
pub const WM8985_HALFIPBIAS_MASK: u32 = 0x0080;  /* HALFIPBIAS */
pub const WM8985_HALFIPBIAS_SHIFT: u32 = 7;  /* HALFIPBIAS */
pub const WM8985_HALFIPBIAS_WIDTH: u32 = 1;  /* HALFIPBIAS */
pub const WM8758_HALFIPBIAS: u32 = 0x0040;  /* HALFI_IPGA */
pub const WM8758_HALFI_IPGA_MASK: u32 = 0x0040;  /* HALFI_IPGA */
pub const WM8758_HALFI_IPGA_SHIFT: u32 = 6;  /* HALFI_IPGA */
pub const WM8758_HALFI_IPGA_WIDTH: u32 = 1;  /* HALFI_IPGA */
pub const WM8985_VBBIASTST_MASK: u32 = 0x0060;  /* VBBIASTST - [6:5] */
pub const WM8985_VBBIASTST_SHIFT: u32 = 5;  /* VBBIASTST - [6:5] */
pub const WM8985_VBBIASTST_WIDTH: u32 = 2;  /* VBBIASTST - [6:5] */
pub const WM8985_BUFBIAS_MASK: u32 = 0x0018;  /* BUFBIAS - [4:3] */
pub const WM8985_BUFBIAS_SHIFT: u32 = 3;  /* BUFBIAS - [4:3] */
pub const WM8985_BUFBIAS_WIDTH: u32 = 2;  /* BUFBIAS - [4:3] */
pub const WM8985_ADCBIAS_MASK: u32 = 0x0006;  /* ADCBIAS - [2:1] */
pub const WM8985_ADCBIAS_SHIFT: u32 = 1;  /* ADCBIAS - [2:1] */
pub const WM8985_ADCBIAS_WIDTH: u32 = 2;  /* ADCBIAS - [2:1] */
pub const WM8985_HALFOPBIAS: u32 = 0x0001;  /* HALFOPBIAS */
pub const WM8985_HALFOPBIAS_MASK: u32 = 0x0001;  /* HALFOPBIAS */
pub const WM8985_HALFOPBIAS_SHIFT: u32 = 0;  /* HALFOPBIAS */
pub const WM8985_HALFOPBIAS_WIDTH: u32 = 1;  /* HALFOPBIAS */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum clk_src {
    WM8985_CLKSRC_MCLK = 0,
    WM8985_CLKSRC_PLL = 1,
}

pub const WM8985_PLL: u32 = 0;


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
