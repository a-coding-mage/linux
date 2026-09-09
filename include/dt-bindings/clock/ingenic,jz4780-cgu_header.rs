/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides clock numbers for the ingenic,jz4780-cgu DT binding.
 *
 * They are roughly ordered as:
 *   - external clocks
 *   - PLLs
 *   - muxes/dividers in the order they appear in the jz4780 programmers manual
 *   - gates in order of their bit in the CLKGR* registers
 */

pub const JZ4780_CLK_EXCLK: u32 = 0;
pub const JZ4780_CLK_RTCLK: u32 = 1;
pub const JZ4780_CLK_APLL: u32 = 2;
pub const JZ4780_CLK_MPLL: u32 = 3;
pub const JZ4780_CLK_EPLL: u32 = 4;
pub const JZ4780_CLK_VPLL: u32 = 5;
pub const JZ4780_CLK_OTGPHY: u32 = 6;
pub const JZ4780_CLK_SCLKA: u32 = 7;
pub const JZ4780_CLK_CPUMUX: u32 = 8;
pub const JZ4780_CLK_CPU: u32 = 9;
pub const JZ4780_CLK_L2CACHE: u32 = 10;
pub const JZ4780_CLK_AHB0: u32 = 11;
pub const JZ4780_CLK_AHB2PMUX: u32 = 12;
pub const JZ4780_CLK_AHB2: u32 = 13;
pub const JZ4780_CLK_PCLK: u32 = 14;
pub const JZ4780_CLK_DDR: u32 = 15;
pub const JZ4780_CLK_VPU: u32 = 16;
pub const JZ4780_CLK_I2SPLL: u32 = 17;
pub const JZ4780_CLK_I2S: u32 = 18;
pub const JZ4780_CLK_LCD0PIXCLK: u32 = 19;
pub const JZ4780_CLK_LCD1PIXCLK: u32 = 20;
pub const JZ4780_CLK_MSCMUX: u32 = 21;
pub const JZ4780_CLK_MSC0: u32 = 22;
pub const JZ4780_CLK_MSC1: u32 = 23;
pub const JZ4780_CLK_MSC2: u32 = 24;
pub const JZ4780_CLK_UHC: u32 = 25;
pub const JZ4780_CLK_SSIPLL: u32 = 26;
pub const JZ4780_CLK_SSI: u32 = 27;
pub const JZ4780_CLK_CIMMCLK: u32 = 28;
pub const JZ4780_CLK_PCMPLL: u32 = 29;
pub const JZ4780_CLK_PCM: u32 = 30;
pub const JZ4780_CLK_GPU: u32 = 31;
pub const JZ4780_CLK_HDMI: u32 = 32;
pub const JZ4780_CLK_BCH: u32 = 33;
pub const JZ4780_CLK_NEMC: u32 = 34;
pub const JZ4780_CLK_OTG0: u32 = 35;
pub const JZ4780_CLK_SSI0: u32 = 36;
pub const JZ4780_CLK_SMB0: u32 = 37;
pub const JZ4780_CLK_SMB1: u32 = 38;
pub const JZ4780_CLK_SCC: u32 = 39;
pub const JZ4780_CLK_AIC: u32 = 40;
pub const JZ4780_CLK_TSSI0: u32 = 41;
pub const JZ4780_CLK_OWI: u32 = 42;
pub const JZ4780_CLK_KBC: u32 = 43;
pub const JZ4780_CLK_SADC: u32 = 44;
pub const JZ4780_CLK_UART0: u32 = 45;
pub const JZ4780_CLK_UART1: u32 = 46;
pub const JZ4780_CLK_UART2: u32 = 47;
pub const JZ4780_CLK_UART3: u32 = 48;
pub const JZ4780_CLK_SSI1: u32 = 49;
pub const JZ4780_CLK_SSI2: u32 = 50;
pub const JZ4780_CLK_PDMA: u32 = 51;
pub const JZ4780_CLK_GPS: u32 = 52;
pub const JZ4780_CLK_MAC: u32 = 53;
pub const JZ4780_CLK_SMB2: u32 = 54;
pub const JZ4780_CLK_CIM: u32 = 55;
pub const JZ4780_CLK_LCD: u32 = 56;
pub const JZ4780_CLK_TVE: u32 = 57;
pub const JZ4780_CLK_IPU: u32 = 58;
pub const JZ4780_CLK_DDR0: u32 = 59;
pub const JZ4780_CLK_DDR1: u32 = 60;
pub const JZ4780_CLK_SMB3: u32 = 61;
pub const JZ4780_CLK_TSSI1: u32 = 62;
pub const JZ4780_CLK_COMPRESS: u32 = 63;
pub const JZ4780_CLK_AIC1: u32 = 64;
pub const JZ4780_CLK_GPVLC: u32 = 65;
pub const JZ4780_CLK_OTG1: u32 = 66;
pub const JZ4780_CLK_UART4: u32 = 67;
pub const JZ4780_CLK_AHBMON: u32 = 68;
pub const JZ4780_CLK_SMB4: u32 = 69;
pub const JZ4780_CLK_DES: u32 = 70;
pub const JZ4780_CLK_X2D: u32 = 71;
pub const JZ4780_CLK_CORE1: u32 = 72;
pub const JZ4780_CLK_EXCLK_DIV512: u32 = 73;
pub const JZ4780_CLK_RTC: u32 = 74;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
