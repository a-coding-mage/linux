/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2014 Samsung Electronics Co., Ltd.
 * Copyright (c) 2016 Krzysztof Kozlowski
 *
 * Device Tree binding constants for Exynos5421 clock controller.
 */

/* core clocks */
pub const CLK_FIN_PLL: u32 = 1;
pub const CLK_FOUT_APLL: u32 = 2;
pub const CLK_FOUT_CPLL: u32 = 3;
pub const CLK_FOUT_MPLL: u32 = 4;
pub const CLK_FOUT_BPLL: u32 = 5;
pub const CLK_FOUT_KPLL: u32 = 6;
pub const CLK_FOUT_EPLL: u32 = 7;

/* gate for special clocks (sclk) */
pub const CLK_SCLK_UART0: u32 = 128;
pub const CLK_SCLK_UART1: u32 = 129;
pub const CLK_SCLK_UART2: u32 = 130;
pub const CLK_SCLK_UART3: u32 = 131;
pub const CLK_SCLK_MMC0: u32 = 132;
pub const CLK_SCLK_MMC1: u32 = 133;
pub const CLK_SCLK_MMC2: u32 = 134;
pub const CLK_SCLK_USBD300: u32 = 150;
pub const CLK_SCLK_USBD301: u32 = 151;
pub const CLK_SCLK_USBPHY300: u32 = 152;
pub const CLK_SCLK_USBPHY301: u32 = 153;
pub const CLK_SCLK_PWM: u32 = 155;

/* gate clocks */
pub const CLK_UART0: u32 = 257;
pub const CLK_UART1: u32 = 258;
pub const CLK_UART2: u32 = 259;
pub const CLK_UART3: u32 = 260;
pub const CLK_I2C0: u32 = 261;
pub const CLK_I2C1: u32 = 262;
pub const CLK_I2C2: u32 = 263;
pub const CLK_I2C3: u32 = 264;
pub const CLK_USI0: u32 = 265;
pub const CLK_USI1: u32 = 266;
pub const CLK_USI2: u32 = 267;
pub const CLK_USI3: u32 = 268;
pub const CLK_TSADC: u32 = 270;
pub const CLK_PWM: u32 = 279;
pub const CLK_MCT: u32 = 315;
pub const CLK_WDT: u32 = 316;
pub const CLK_RTC: u32 = 317;
pub const CLK_TMU: u32 = 318;
pub const CLK_MMC0: u32 = 351;
pub const CLK_MMC1: u32 = 352;
pub const CLK_MMC2: u32 = 353;
pub const CLK_PDMA0: u32 = 362;
pub const CLK_PDMA1: u32 = 363;
pub const CLK_USBH20: u32 = 365;
pub const CLK_USBD300: u32 = 366;
pub const CLK_USBD301: u32 = 367;
pub const CLK_SSS: u32 = 471;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
