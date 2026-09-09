/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2012-2013 Hisilicon Limited.
 * Copyright (c) 2012-2013 Linaro Limited.
 *
 * Author: Haojian Zhuang <haojian.zhuang@linaro.org>
 *         Xin Li <li.xin@linaro.org>
 */

pub const HI3620_NONE_CLOCK: u32 = 0;

/* fixed rate & fixed factor clocks */
pub const HI3620_OSC32K: u32 = 1;
pub const HI3620_OSC26M: u32 = 2;
pub const HI3620_PCLK: u32 = 3;
pub const HI3620_PLL_ARM0: u32 = 4;
pub const HI3620_PLL_ARM1: u32 = 5;
pub const HI3620_PLL_PERI: u32 = 6;
pub const HI3620_PLL_USB: u32 = 7;
pub const HI3620_PLL_HDMI: u32 = 8;
pub const HI3620_PLL_GPU: u32 = 9;
pub const HI3620_RCLK_TCXO: u32 = 10;
pub const HI3620_RCLK_CFGAXI: u32 = 11;
pub const HI3620_RCLK_PICO: u32 = 12;

/* mux clocks */
pub const HI3620_TIMER0_MUX: u32 = 32;
pub const HI3620_TIMER1_MUX: u32 = 33;
pub const HI3620_TIMER2_MUX: u32 = 34;
pub const HI3620_TIMER3_MUX: u32 = 35;
pub const HI3620_TIMER4_MUX: u32 = 36;
pub const HI3620_TIMER5_MUX: u32 = 37;
pub const HI3620_TIMER6_MUX: u32 = 38;
pub const HI3620_TIMER7_MUX: u32 = 39;
pub const HI3620_TIMER8_MUX: u32 = 40;
pub const HI3620_TIMER9_MUX: u32 = 41;
pub const HI3620_UART0_MUX: u32 = 42;
pub const HI3620_UART1_MUX: u32 = 43;
pub const HI3620_UART2_MUX: u32 = 44;
pub const HI3620_UART3_MUX: u32 = 45;
pub const HI3620_UART4_MUX: u32 = 46;
pub const HI3620_SPI0_MUX: u32 = 47;
pub const HI3620_SPI1_MUX: u32 = 48;
pub const HI3620_SPI2_MUX: u32 = 49;
pub const HI3620_SAXI_MUX: u32 = 50;
pub const HI3620_PWM0_MUX: u32 = 51;
pub const HI3620_PWM1_MUX: u32 = 52;
pub const HI3620_SD_MUX: u32 = 53;
pub const HI3620_MMC1_MUX: u32 = 54;
pub const HI3620_MMC1_MUX2: u32 = 55;
pub const HI3620_G2D_MUX: u32 = 56;
pub const HI3620_VENC_MUX: u32 = 57;
pub const HI3620_VDEC_MUX: u32 = 58;
pub const HI3620_VPP_MUX: u32 = 59;
pub const HI3620_EDC0_MUX: u32 = 60;
pub const HI3620_LDI0_MUX: u32 = 61;
pub const HI3620_EDC1_MUX: u32 = 62;
pub const HI3620_LDI1_MUX: u32 = 63;
pub const HI3620_RCLK_HSIC: u32 = 64;
pub const HI3620_MMC2_MUX: u32 = 65;
pub const HI3620_MMC3_MUX: u32 = 66;

/* divider clocks */
pub const HI3620_SHAREAXI_DIV: u32 = 128;
pub const HI3620_CFGAXI_DIV: u32 = 129;
pub const HI3620_SD_DIV: u32 = 130;
pub const HI3620_MMC1_DIV: u32 = 131;
pub const HI3620_HSIC_DIV: u32 = 132;
pub const HI3620_MMC2_DIV: u32 = 133;
pub const HI3620_MMC3_DIV: u32 = 134;

/* gate clocks */
pub const HI3620_TIMERCLK01: u32 = 160;
pub const HI3620_TIMER_RCLK01: u32 = 161;
pub const HI3620_TIMERCLK23: u32 = 162;
pub const HI3620_TIMER_RCLK23: u32 = 163;
pub const HI3620_TIMERCLK45: u32 = 164;
pub const HI3620_TIMERCLK67: u32 = 165;
pub const HI3620_TIMERCLK89: u32 = 166;
pub const HI3620_RTCCLK: u32 = 167;
pub const HI3620_KPC_CLK: u32 = 168;
pub const HI3620_GPIOCLK0: u32 = 169;
pub const HI3620_GPIOCLK1: u32 = 170;
pub const HI3620_GPIOCLK2: u32 = 171;
pub const HI3620_GPIOCLK3: u32 = 172;
pub const HI3620_GPIOCLK4: u32 = 173;
pub const HI3620_GPIOCLK5: u32 = 174;
pub const HI3620_GPIOCLK6: u32 = 175;
pub const HI3620_GPIOCLK7: u32 = 176;
pub const HI3620_GPIOCLK8: u32 = 177;
pub const HI3620_GPIOCLK9: u32 = 178;
pub const HI3620_GPIOCLK10: u32 = 179;
pub const HI3620_GPIOCLK11: u32 = 180;
pub const HI3620_GPIOCLK12: u32 = 181;
pub const HI3620_GPIOCLK13: u32 = 182;
pub const HI3620_GPIOCLK14: u32 = 183;
pub const HI3620_GPIOCLK15: u32 = 184;
pub const HI3620_GPIOCLK16: u32 = 185;
pub const HI3620_GPIOCLK17: u32 = 186;
pub const HI3620_GPIOCLK18: u32 = 187;
pub const HI3620_GPIOCLK19: u32 = 188;
pub const HI3620_GPIOCLK20: u32 = 189;
pub const HI3620_GPIOCLK21: u32 = 190;
pub const HI3620_DPHY0_CLK: u32 = 191;
pub const HI3620_DPHY1_CLK: u32 = 192;
pub const HI3620_DPHY2_CLK: u32 = 193;
pub const HI3620_USBPHY_CLK: u32 = 194;
pub const HI3620_ACP_CLK: u32 = 195;
pub const HI3620_PWMCLK0: u32 = 196;
pub const HI3620_PWMCLK1: u32 = 197;
pub const HI3620_UARTCLK0: u32 = 198;
pub const HI3620_UARTCLK1: u32 = 199;
pub const HI3620_UARTCLK2: u32 = 200;
pub const HI3620_UARTCLK3: u32 = 201;
pub const HI3620_UARTCLK4: u32 = 202;
pub const HI3620_SPICLK0: u32 = 203;
pub const HI3620_SPICLK1: u32 = 204;
pub const HI3620_SPICLK2: u32 = 205;
pub const HI3620_I2CCLK0: u32 = 206;
pub const HI3620_I2CCLK1: u32 = 207;
pub const HI3620_I2CCLK2: u32 = 208;
pub const HI3620_I2CCLK3: u32 = 209;
pub const HI3620_SCI_CLK: u32 = 210;
pub const HI3620_DDRC_PER_CLK: u32 = 211;
pub const HI3620_DMAC_CLK: u32 = 212;
pub const HI3620_USB2DVC_CLK: u32 = 213;
pub const HI3620_SD_CLK: u32 = 214;
pub const HI3620_MMC_CLK1: u32 = 215;
pub const HI3620_MMC_CLK2: u32 = 216;
pub const HI3620_MMC_CLK3: u32 = 217;
pub const HI3620_MCU_CLK: u32 = 218;

pub const HI3620_SD_CIUCLK: u32 = 0;
pub const HI3620_MMC_CIUCLK1: u32 = 1;
pub const HI3620_MMC_CIUCLK2: u32 = 2;
pub const HI3620_MMC_CIUCLK3: u32 = 3;

pub const HI3620_NR_CLKS: u32 = 219;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
