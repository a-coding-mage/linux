/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2019 Sean Anderson <seanga2@gmail.com>
 * Copyright (c) 2020 Western Digital Corporation or its affiliates.
 */

/*
 * Kendryte K210 SoC system controller K210_SYSCTL_SOFT_RESET register bits.
 * Taken from Kendryte SDK (kendryte-standalone-sdk).
 */
pub const K210_RST_ROM: u32 = 0;
pub const K210_RST_DMA: u32 = 1;
pub const K210_RST_AI: u32 = 2;
pub const K210_RST_DVP: u32 = 3;
pub const K210_RST_FFT: u32 = 4;
pub const K210_RST_GPIO: u32 = 5;
pub const K210_RST_SPI0: u32 = 6;
pub const K210_RST_SPI1: u32 = 7;
pub const K210_RST_SPI2: u32 = 8;
pub const K210_RST_SPI3: u32 = 9;
pub const K210_RST_I2S0: u32 = 10;
pub const K210_RST_I2S1: u32 = 11;
pub const K210_RST_I2S2: u32 = 12;
pub const K210_RST_I2C0: u32 = 13;
pub const K210_RST_I2C1: u32 = 14;
pub const K210_RST_I2C2: u32 = 15;
pub const K210_RST_UART1: u32 = 16;
pub const K210_RST_UART2: u32 = 17;
pub const K210_RST_UART3: u32 = 18;
pub const K210_RST_AES: u32 = 19;
pub const K210_RST_FPIOA: u32 = 20;
pub const K210_RST_TIMER0: u32 = 21;
pub const K210_RST_TIMER1: u32 = 22;
pub const K210_RST_TIMER2: u32 = 23;
pub const K210_RST_WDT0: u32 = 24;
pub const K210_RST_WDT1: u32 = 25;
pub const K210_RST_SHA: u32 = 26;
pub const K210_RST_RTC: u32 = 29;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
