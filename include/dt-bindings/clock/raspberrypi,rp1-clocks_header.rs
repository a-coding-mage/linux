/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (C) 2021 Raspberry Pi Ltd.
 */

pub const RP1_PLL_SYS_CORE: u32 = 0;
pub const RP1_PLL_AUDIO_CORE: u32 = 1;
pub const RP1_PLL_VIDEO_CORE: u32 = 2;

pub const RP1_PLL_SYS: u32 = 3;
pub const RP1_PLL_AUDIO: u32 = 4;
pub const RP1_PLL_VIDEO: u32 = 5;

pub const RP1_PLL_SYS_PRI_PH: u32 = 6;
pub const RP1_PLL_SYS_SEC_PH: u32 = 7;
pub const RP1_PLL_AUDIO_PRI_PH: u32 = 8;

pub const RP1_PLL_SYS_SEC: u32 = 9;
pub const RP1_PLL_AUDIO_SEC: u32 = 10;
pub const RP1_PLL_VIDEO_SEC: u32 = 11;

pub const RP1_CLK_SYS: u32 = 12;
pub const RP1_CLK_SLOW_SYS: u32 = 13;
pub const RP1_CLK_DMA: u32 = 14;
pub const RP1_CLK_UART: u32 = 15;
pub const RP1_CLK_ETH: u32 = 16;
pub const RP1_CLK_PWM0: u32 = 17;
pub const RP1_CLK_PWM1: u32 = 18;
pub const RP1_CLK_AUDIO_IN: u32 = 19;
pub const RP1_CLK_AUDIO_OUT: u32 = 20;
pub const RP1_CLK_I2S: u32 = 21;
pub const RP1_CLK_MIPI0_CFG: u32 = 22;
pub const RP1_CLK_MIPI1_CFG: u32 = 23;
pub const RP1_CLK_PCIE_AUX: u32 = 24;
pub const RP1_CLK_USBH0_MICROFRAME: u32 = 25;
pub const RP1_CLK_USBH1_MICROFRAME: u32 = 26;
pub const RP1_CLK_USBH0_SUSPEND: u32 = 27;
pub const RP1_CLK_USBH1_SUSPEND: u32 = 28;
pub const RP1_CLK_ETH_TSU: u32 = 29;
pub const RP1_CLK_ADC: u32 = 30;
pub const RP1_CLK_SDIO_TIMER: u32 = 31;
pub const RP1_CLK_SDIO_ALT_SRC: u32 = 32;
pub const RP1_CLK_GP0: u32 = 33;
pub const RP1_CLK_GP1: u32 = 34;
pub const RP1_CLK_GP2: u32 = 35;
pub const RP1_CLK_GP3: u32 = 36;
pub const RP1_CLK_GP4: u32 = 37;
pub const RP1_CLK_GP5: u32 = 38;
pub const RP1_CLK_VEC: u32 = 39;
pub const RP1_CLK_DPI: u32 = 40;
pub const RP1_CLK_MIPI0_DPI: u32 = 41;
pub const RP1_CLK_MIPI1_DPI: u32 = 42;

// Extra PLL output channels - RP1B0 only
pub const RP1_PLL_VIDEO_PRI_PH: u32 = 43;
pub const RP1_PLL_AUDIO_TERN: u32 = 44;

// MIPI clocks managed by the DSI driver
pub const RP1_CLK_MIPI0_DSI_BYTECLOCK: u32 = 45;
pub const RP1_CLK_MIPI1_DSI_BYTECLOCK: u32 = 46;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
