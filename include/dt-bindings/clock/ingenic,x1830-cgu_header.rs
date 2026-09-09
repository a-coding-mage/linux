/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides clock numbers for the ingenic,x1830-cgu DT binding.
 *
 * They are roughly ordered as:
 *   - external clocks
 *   - PLLs
 *   - muxes/dividers in the order they appear in the x1830 programmers manual
 *   - gates in order of their bit in the CLKGR* registers
 */

pub const X1830_CLK_EXCLK: i32 = 0;
pub const X1830_CLK_RTCLK: i32 = 1;
pub const X1830_CLK_APLL: i32 = 2;
pub const X1830_CLK_MPLL: i32 = 3;
pub const X1830_CLK_EPLL: i32 = 4;
pub const X1830_CLK_VPLL: i32 = 5;
pub const X1830_CLK_OTGPHY: i32 = 6;
pub const X1830_CLK_SCLKA: i32 = 7;
pub const X1830_CLK_CPUMUX: i32 = 8;
pub const X1830_CLK_CPU: i32 = 9;
pub const X1830_CLK_L2CACHE: i32 = 10;
pub const X1830_CLK_AHB0: i32 = 11;
pub const X1830_CLK_AHB2PMUX: i32 = 12;
pub const X1830_CLK_AHB2: i32 = 13;
pub const X1830_CLK_PCLK: i32 = 14;
pub const X1830_CLK_DDR: i32 = 15;
pub const X1830_CLK_MAC: i32 = 16;
pub const X1830_CLK_LCD: i32 = 17;
pub const X1830_CLK_MSCMUX: i32 = 18;
pub const X1830_CLK_MSC0: i32 = 19;
pub const X1830_CLK_MSC1: i32 = 20;
pub const X1830_CLK_SSIPLL: i32 = 21;
pub const X1830_CLK_SSIPLL_DIV2: i32 = 22;
pub const X1830_CLK_SSIMUX: i32 = 23;
pub const X1830_CLK_EMC: i32 = 24;
pub const X1830_CLK_EFUSE: i32 = 25;
pub const X1830_CLK_OTG: i32 = 26;
pub const X1830_CLK_SSI0: i32 = 27;
pub const X1830_CLK_SMB0: i32 = 28;
pub const X1830_CLK_SMB1: i32 = 29;
pub const X1830_CLK_SMB2: i32 = 30;
pub const X1830_CLK_UART0: i32 = 31;
pub const X1830_CLK_UART1: i32 = 32;
pub const X1830_CLK_SSI1: i32 = 33;
pub const X1830_CLK_SFC: i32 = 34;
pub const X1830_CLK_PDMA: i32 = 35;
pub const X1830_CLK_TCU: i32 = 36;
pub const X1830_CLK_DTRNG: i32 = 37;
pub const X1830_CLK_OST: i32 = 38;
pub const X1830_CLK_EXCLK_DIV512: i32 = 39;
pub const X1830_CLK_RTC: i32 = 40;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
