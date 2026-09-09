/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides clock numbers for the ingenic,x1000-cgu DT binding.
 *
 * They are roughly ordered as:
 *   - external clocks
 *   - PLLs
 *   - muxes/dividers in the order they appear in the x1000 programmers manual
 *   - gates in order of their bit in the CLKGR* registers
 */

pub const X1000_CLK_EXCLK: u32 = 0;
pub const X1000_CLK_RTCLK: u32 = 1;
pub const X1000_CLK_APLL: u32 = 2;
pub const X1000_CLK_MPLL: u32 = 3;
pub const X1000_CLK_OTGPHY: u32 = 4;
pub const X1000_CLK_SCLKA: u32 = 5;
pub const X1000_CLK_CPUMUX: u32 = 6;
pub const X1000_CLK_CPU: u32 = 7;
pub const X1000_CLK_L2CACHE: u32 = 8;
pub const X1000_CLK_AHB0: u32 = 9;
pub const X1000_CLK_AHB2PMUX: u32 = 10;
pub const X1000_CLK_AHB2: u32 = 11;
pub const X1000_CLK_PCLK: u32 = 12;
pub const X1000_CLK_DDR: u32 = 13;
pub const X1000_CLK_MAC: u32 = 14;
pub const X1000_CLK_LCD: u32 = 15;
pub const X1000_CLK_MSCMUX: u32 = 16;
pub const X1000_CLK_MSC0: u32 = 17;
pub const X1000_CLK_MSC1: u32 = 18;
pub const X1000_CLK_OTG: u32 = 19;
pub const X1000_CLK_SSIPLL: u32 = 20;
pub const X1000_CLK_SSIPLL_DIV2: u32 = 21;
pub const X1000_CLK_SSIMUX: u32 = 22;
pub const X1000_CLK_EMC: u32 = 23;
pub const X1000_CLK_EFUSE: u32 = 24;
pub const X1000_CLK_SFC: u32 = 25;
pub const X1000_CLK_I2C0: u32 = 26;
pub const X1000_CLK_I2C1: u32 = 27;
pub const X1000_CLK_I2C2: u32 = 28;
pub const X1000_CLK_UART0: u32 = 29;
pub const X1000_CLK_UART1: u32 = 30;
pub const X1000_CLK_UART2: u32 = 31;
pub const X1000_CLK_TCU: u32 = 32;
pub const X1000_CLK_SSI: u32 = 33;
pub const X1000_CLK_OST: u32 = 34;
pub const X1000_CLK_PDMA: u32 = 35;
pub const X1000_CLK_EXCLK_DIV512: u32 = 36;
pub const X1000_CLK_RTC: u32 = 37;
pub const X1000_CLK_AIC: u32 = 38;
pub const X1000_CLK_I2SPLLMUX: u32 = 39;
pub const X1000_CLK_I2SPLL: u32 = 40;
pub const X1000_CLK_I2S: u32 = 41;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
