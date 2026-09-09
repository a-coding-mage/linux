/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides clock numbers for the ingenic,jz4740-cgu DT binding.
 *
 * They are roughly ordered as:
 *   - external clocks
 *   - PLLs
 *   - muxes/dividers in the order they appear in the jz4740 programmers manual
 *   - gates in order of their bit in the CLKGR* registers
 */

// Header guard: __DT_BINDINGS_CLOCK_JZ4740_CGU_H__

pub const JZ4740_CLK_EXT: u32 = 0;
pub const JZ4740_CLK_RTC: u32 = 1;
pub const JZ4740_CLK_PLL: u32 = 2;
pub const JZ4740_CLK_PLL_HALF: u32 = 3;
pub const JZ4740_CLK_CCLK: u32 = 4;
pub const JZ4740_CLK_HCLK: u32 = 5;
pub const JZ4740_CLK_PCLK: u32 = 6;
pub const JZ4740_CLK_MCLK: u32 = 7;
pub const JZ4740_CLK_LCD: u32 = 8;
pub const JZ4740_CLK_LCD_PCLK: u32 = 9;
pub const JZ4740_CLK_I2S: u32 = 10;
pub const JZ4740_CLK_SPI: u32 = 11;
pub const JZ4740_CLK_MMC: u32 = 12;
pub const JZ4740_CLK_UHC: u32 = 13;
pub const JZ4740_CLK_UDC: u32 = 14;
pub const JZ4740_CLK_UART0: u32 = 15;
pub const JZ4740_CLK_UART1: u32 = 16;
pub const JZ4740_CLK_DMA: u32 = 17;
pub const JZ4740_CLK_IPU: u32 = 18;
pub const JZ4740_CLK_ADC: u32 = 19;
pub const JZ4740_CLK_I2C: u32 = 20;
pub const JZ4740_CLK_AIC: u32 = 21;
pub const JZ4740_CLK_TCU: u32 = 22;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
