/* SPDX-License-Identifier: GPL-2.0
 *
 * Device Tree binding constants for Actions Semi S700 Clock Management Unit
 *
 * Copyright (c) 2014 Actions Semi Inc.
 * Author: David Liu <liuwei@actions-semi.com>
 *
 * Author: Pathiban Nallathambi <pn@denx.de>
 * Author: Saravanan Sekar <sravanhome@gmail.com>
 */

// Translated from the C header actions,s700-cmu.h.

pub const CLK_NONE: u32 = 0;

/* pll clocks */
pub const CLK_CORE_PLL: u32 = 1;
pub const CLK_DEV_PLL: u32 = 2;
pub const CLK_DDR_PLL: u32 = 3;
pub const CLK_NAND_PLL: u32 = 4;
pub const CLK_DISPLAY_PLL: u32 = 5;
pub const CLK_TVOUT_PLL: u32 = 6;
pub const CLK_CVBS_PLL: u32 = 7;
pub const CLK_AUDIO_PLL: u32 = 8;
pub const CLK_ETHERNET_PLL: u32 = 9;

/* system clock */
pub const CLK_CPU: u32 = 10;
pub const CLK_DEV: u32 = 11;
pub const CLK_AHB: u32 = 12;
pub const CLK_APB: u32 = 13;
pub const CLK_DMAC: u32 = 14;
pub const CLK_NOC0_CLK_MUX: u32 = 15;
pub const CLK_NOC1_CLK_MUX: u32 = 16;
pub const CLK_HP_CLK_MUX: u32 = 17;
pub const CLK_HP_CLK_DIV: u32 = 18;
pub const CLK_NOC1_CLK_DIV: u32 = 19;
pub const CLK_NOC0: u32 = 20;
pub const CLK_NOC1: u32 = 21;
pub const CLK_SENOR_SRC: u32 = 22;

/* peripheral device clock */
pub const CLK_GPIO: u32 = 23;
pub const CLK_TIMER: u32 = 24;
pub const CLK_DSI: u32 = 25;
pub const CLK_CSI: u32 = 26;
pub const CLK_SI: u32 = 27;
pub const CLK_DE: u32 = 28;
pub const CLK_HDE: u32 = 29;
pub const CLK_VDE: u32 = 30;
pub const CLK_VCE: u32 = 31;
pub const CLK_NAND: u32 = 32;
pub const CLK_SD0: u32 = 33;
pub const CLK_SD1: u32 = 34;
pub const CLK_SD2: u32 = 35;

pub const CLK_UART0: u32 = 36;
pub const CLK_UART1: u32 = 37;
pub const CLK_UART2: u32 = 38;
pub const CLK_UART3: u32 = 39;
pub const CLK_UART4: u32 = 40;
pub const CLK_UART5: u32 = 41;
pub const CLK_UART6: u32 = 42;

pub const CLK_PWM0: u32 = 43;
pub const CLK_PWM1: u32 = 44;
pub const CLK_PWM2: u32 = 45;
pub const CLK_PWM3: u32 = 46;
pub const CLK_PWM4: u32 = 47;
pub const CLK_PWM5: u32 = 48;
pub const CLK_GPU3D: u32 = 49;

pub const CLK_I2C0: u32 = 50;
pub const CLK_I2C1: u32 = 51;
pub const CLK_I2C2: u32 = 52;
pub const CLK_I2C3: u32 = 53;

pub const CLK_SPI0: u32 = 54;
pub const CLK_SPI1: u32 = 55;
pub const CLK_SPI2: u32 = 56;
pub const CLK_SPI3: u32 = 57;

pub const CLK_USB3_480MPLL0: u32 = 58;
pub const CLK_USB3_480MPHY0: u32 = 59;
pub const CLK_USB3_5GPHY: u32 = 60;
pub const CLK_USB3_CCE: u32 = 61;
pub const CLK_USB3_MAC: u32 = 62;

pub const CLK_LCD: u32 = 63;
pub const CLK_HDMI_AUDIO: u32 = 64;
pub const CLK_I2SRX: u32 = 65;
pub const CLK_I2STX: u32 = 66;

pub const CLK_SENSOR0: u32 = 67;
pub const CLK_SENSOR1: u32 = 68;

pub const CLK_HDMI_DEV: u32 = 69;

pub const CLK_ETHERNET: u32 = 70;
pub const CLK_RMII_REF: u32 = 71;

pub const CLK_USB2H0_PLLEN: u32 = 72;
pub const CLK_USB2H0_PHY: u32 = 73;
pub const CLK_USB2H0_CCE: u32 = 74;
pub const CLK_USB2H1_PLLEN: u32 = 75;
pub const CLK_USB2H1_PHY: u32 = 76;
pub const CLK_USB2H1_CCE: u32 = 77;

pub const CLK_TVOUT: u32 = 78;

pub const CLK_THERMAL_SENSOR: u32 = 79;

pub const CLK_IRC_SWITCH: u32 = 80;
pub const CLK_PCM1: u32 = 81;
pub const CLK_NR_CLKS: u32 = CLK_PCM1 + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
