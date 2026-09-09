/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ARTPEC-6 clock controller indexes
 *
 * Copyright 2016 Axis Communications AB.
 */

pub const ARTPEC6_CLK_CPU: u32 = 0;
pub const ARTPEC6_CLK_CPU_PERIPH: u32 = 1;
pub const ARTPEC6_CLK_NAND_CLKA: u32 = 2;
pub const ARTPEC6_CLK_NAND_CLKB: u32 = 3;
pub const ARTPEC6_CLK_ETH_ACLK: u32 = 4;
pub const ARTPEC6_CLK_DMA_ACLK: u32 = 5;
pub const ARTPEC6_CLK_PTP_REF: u32 = 6;
pub const ARTPEC6_CLK_SD_PCLK: u32 = 7;
pub const ARTPEC6_CLK_SD_IMCLK: u32 = 8;
pub const ARTPEC6_CLK_I2S_HST: u32 = 9;
pub const ARTPEC6_CLK_I2S0_CLK: u32 = 10;
pub const ARTPEC6_CLK_I2S1_CLK: u32 = 11;
pub const ARTPEC6_CLK_UART_PCLK: u32 = 12;
pub const ARTPEC6_CLK_UART_REFCLK: u32 = 13;
pub const ARTPEC6_CLK_I2C: u32 = 14;
pub const ARTPEC6_CLK_SPI_PCLK: u32 = 15;
pub const ARTPEC6_CLK_SPI_SSPCLK: u32 = 16;
pub const ARTPEC6_CLK_SYS_TIMER: u32 = 17;
pub const ARTPEC6_CLK_FRACDIV_IN: u32 = 18;
pub const ARTPEC6_CLK_DBG_PCLK: u32 = 19;

/* This must be the highest clock index plus one. */
pub const ARTPEC6_CLK_NUMCLOCKS: u32 = 20;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
