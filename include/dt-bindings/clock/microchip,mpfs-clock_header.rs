/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Daire McNamara,<daire.mcnamara@microchip.com>
 * Copyright (C) 2020-2022 Microchip Technology Inc.  All rights reserved.
 */

pub const CLK_CPU: u32 = 0;
pub const CLK_AXI: u32 = 1;
pub const CLK_AHB: u32 = 2;

pub const CLK_ENVM: u32 = 3;
pub const CLK_MAC0: u32 = 4;
pub const CLK_MAC1: u32 = 5;
pub const CLK_MMC: u32 = 6;
pub const CLK_TIMER: u32 = 7;
pub const CLK_MMUART0: u32 = 8;
pub const CLK_MMUART1: u32 = 9;
pub const CLK_MMUART2: u32 = 10;
pub const CLK_MMUART3: u32 = 11;
pub const CLK_MMUART4: u32 = 12;
pub const CLK_SPI0: u32 = 13;
pub const CLK_SPI1: u32 = 14;
pub const CLK_I2C0: u32 = 15;
pub const CLK_I2C1: u32 = 16;
pub const CLK_CAN0: u32 = 17;
pub const CLK_CAN1: u32 = 18;
pub const CLK_USB: u32 = 19;
pub const CLK_RESERVED: u32 = 20;
pub const CLK_RTC: u32 = 21;
pub const CLK_QSPI: u32 = 22;
pub const CLK_GPIO0: u32 = 23;
pub const CLK_GPIO1: u32 = 24;
pub const CLK_GPIO2: u32 = 25;
pub const CLK_DDRC: u32 = 26;
pub const CLK_FIC0: u32 = 27;
pub const CLK_FIC1: u32 = 28;
pub const CLK_FIC2: u32 = 29;
pub const CLK_FIC3: u32 = 30;
pub const CLK_ATHENA: u32 = 31;
pub const CLK_CFM: u32 = 32;

pub const CLK_RTCREF: u32 = 33;
pub const CLK_MSSPLL: u32 = 34;
pub const CLK_MSSPLL0: u32 = 34;
pub const CLK_MSSPLL1: u32 = 35;
pub const CLK_MSSPLL2: u32 = 36;
pub const CLK_MSSPLL3: u32 = 37;
/* 38 is reserved for MSS PLL internals */

/* Clock Conditioning Circuitry Clock IDs */

pub const CLK_CCC_PLL0: u32 = 0;
pub const CLK_CCC_PLL1: u32 = 1;
pub const CLK_CCC_DLL0: u32 = 2;
pub const CLK_CCC_DLL1: u32 = 3;

pub const CLK_CCC_PLL0_OUT0: u32 = 4;
pub const CLK_CCC_PLL0_OUT1: u32 = 5;
pub const CLK_CCC_PLL0_OUT2: u32 = 6;
pub const CLK_CCC_PLL0_OUT3: u32 = 7;

pub const CLK_CCC_PLL1_OUT0: u32 = 8;
pub const CLK_CCC_PLL1_OUT1: u32 = 9;
pub const CLK_CCC_PLL1_OUT2: u32 = 10;
pub const CLK_CCC_PLL1_OUT3: u32 = 11;

pub const CLK_CCC_DLL0_OUT0: u32 = 12;
pub const CLK_CCC_DLL0_OUT1: u32 = 13;

pub const CLK_CCC_DLL1_OUT0: u32 = 14;
pub const CLK_CCC_DLL1_OUT1: u32 = 15;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
