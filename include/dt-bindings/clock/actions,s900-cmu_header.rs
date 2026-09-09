// SPDX-License-Identifier: GPL-2.0+
//
// Device Tree binding constants for Actions Semi S900 Clock Management Unit
//
// Copyright (c) 2014 Actions Semi Inc.
// Copyright (c) 2018 Linaro Ltd.

pub const CLK_NONE: u32 = 0;

/* fixed rate clocks */
pub const CLK_LOSC: u32 = 1;
pub const CLK_HOSC: u32 = 2;

/* pll clocks */
pub const CLK_CORE_PLL: u32 = 3;
pub const CLK_DEV_PLL: u32 = 4;
pub const CLK_DDR_PLL: u32 = 5;
pub const CLK_NAND_PLL: u32 = 6;
pub const CLK_DISPLAY_PLL: u32 = 7;
pub const CLK_DSI_PLL: u32 = 8;
pub const CLK_ASSIST_PLL: u32 = 9;
pub const CLK_AUDIO_PLL: u32 = 10;

/* system clock */
pub const CLK_CPU: u32 = 15;
pub const CLK_DEV: u32 = 16;
pub const CLK_NOC: u32 = 17;
pub const CLK_NOC_MUX: u32 = 18;
pub const CLK_NOC_DIV: u32 = 19;
pub const CLK_AHB: u32 = 20;
pub const CLK_APB: u32 = 21;
pub const CLK_DMAC: u32 = 22;

/* peripheral device clock */
pub const CLK_GPIO: u32 = 23;

pub const CLK_BISP: u32 = 24;
pub const CLK_CSI0: u32 = 25;
pub const CLK_CSI1: u32 = 26;

pub const CLK_DE0: u32 = 27;
pub const CLK_DE1: u32 = 28;
pub const CLK_DE2: u32 = 29;
pub const CLK_DE3: u32 = 30;
pub const CLK_DSI: u32 = 32;

pub const CLK_GPU: u32 = 33;
pub const CLK_GPU_CORE: u32 = 34;
pub const CLK_GPU_MEM: u32 = 35;
pub const CLK_GPU_SYS: u32 = 36;

pub const CLK_HDE: u32 = 37;
pub const CLK_I2C0: u32 = 38;
pub const CLK_I2C1: u32 = 39;
pub const CLK_I2C2: u32 = 40;
pub const CLK_I2C3: u32 = 41;
pub const CLK_I2C4: u32 = 42;
pub const CLK_I2C5: u32 = 43;
pub const CLK_I2SRX: u32 = 44;
pub const CLK_I2STX: u32 = 45;
pub const CLK_IMX: u32 = 46;
pub const CLK_LCD: u32 = 47;
pub const CLK_NAND0: u32 = 48;
pub const CLK_NAND1: u32 = 49;
pub const CLK_PWM0: u32 = 50;
pub const CLK_PWM1: u32 = 51;
pub const CLK_PWM2: u32 = 52;
pub const CLK_PWM3: u32 = 53;
pub const CLK_PWM4: u32 = 54;
pub const CLK_PWM5: u32 = 55;
pub const CLK_SD0: u32 = 56;
pub const CLK_SD1: u32 = 57;
pub const CLK_SD2: u32 = 58;
pub const CLK_SD3: u32 = 59;
pub const CLK_SENSOR: u32 = 60;
pub const CLK_SPEED_SENSOR: u32 = 61;
pub const CLK_SPI0: u32 = 62;
pub const CLK_SPI1: u32 = 63;
pub const CLK_SPI2: u32 = 64;
pub const CLK_SPI3: u32 = 65;
pub const CLK_THERMAL_SENSOR: u32 = 66;
pub const CLK_UART0: u32 = 67;
pub const CLK_UART1: u32 = 68;
pub const CLK_UART2: u32 = 69;
pub const CLK_UART3: u32 = 70;
pub const CLK_UART4: u32 = 71;
pub const CLK_UART5: u32 = 72;
pub const CLK_UART6: u32 = 73;
pub const CLK_VCE: u32 = 74;
pub const CLK_VDE: u32 = 75;

pub const CLK_USB3_480MPLL0: u32 = 76;
pub const CLK_USB3_480MPHY0: u32 = 77;
pub const CLK_USB3_5GPHY: u32 = 78;
pub const CLK_USB3_CCE: u32 = 79;
pub const CLK_USB3_MAC: u32 = 80;

pub const CLK_TIMER: u32 = 83;

pub const CLK_HDMI_AUDIO: u32 = 84;

pub const CLK_24M: u32 = 85;

pub const CLK_EDP: u32 = 86;

pub const CLK_24M_EDP: u32 = 87;
pub const CLK_EDP_PLL: u32 = 88;
pub const CLK_EDP_LINK: u32 = 89;

pub const CLK_USB2H0_PLLEN: u32 = 90;
pub const CLK_USB2H0_PHY: u32 = 91;
pub const CLK_USB2H0_CCE: u32 = 92;
pub const CLK_USB2H1_PLLEN: u32 = 93;
pub const CLK_USB2H1_PHY: u32 = 94;
pub const CLK_USB2H1_CCE: u32 = 95;

pub const CLK_DDR0: u32 = 96;
pub const CLK_DDR1: u32 = 97;
pub const CLK_DMM: u32 = 98;

pub const CLK_ETH_MAC: u32 = 99;
pub const CLK_RMII_REF: u32 = 100;

pub const CLK_NR_CLKS: u32 = CLK_RMII_REF + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
