/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2014 Samsung Electronics Co., Ltd.
 * Author: Naveen Krishna Ch <naveenkrishna.ch@gmail.com>
 */

/* Translated from the C header; preprocessor header guards are omitted. */

/* TOPC */
pub const DOUT_ACLK_PERIS: u32 = 1;
pub const DOUT_SCLK_BUS0_PLL: u32 = 2;
pub const DOUT_SCLK_BUS1_PLL: u32 = 3;
pub const DOUT_SCLK_CC_PLL: u32 = 4;
pub const DOUT_SCLK_MFC_PLL: u32 = 5;
pub const DOUT_ACLK_CCORE_133: u32 = 6;
pub const DOUT_ACLK_MSCL_532: u32 = 7;
pub const ACLK_MSCL_532: u32 = 8;
pub const DOUT_SCLK_AUD_PLL: u32 = 9;
pub const FOUT_AUD_PLL: u32 = 10;
pub const SCLK_AUD_PLL: u32 = 11;
pub const SCLK_MFC_PLL_B: u32 = 12;
pub const SCLK_MFC_PLL_A: u32 = 13;
pub const SCLK_BUS1_PLL_B: u32 = 14;
pub const SCLK_BUS1_PLL_A: u32 = 15;
pub const SCLK_BUS0_PLL_B: u32 = 16;
pub const SCLK_BUS0_PLL_A: u32 = 17;
pub const SCLK_CC_PLL_B: u32 = 18;
pub const SCLK_CC_PLL_A: u32 = 19;
pub const ACLK_CCORE_133: u32 = 20;
pub const ACLK_PERIS_66: u32 = 21;
pub const TOPC_NR_CLK: u32 = 22;

/* TOP0 */
pub const DOUT_ACLK_PERIC1: u32 = 1;
pub const DOUT_ACLK_PERIC0: u32 = 2;
pub const CLK_SCLK_UART0: u32 = 3;
pub const CLK_SCLK_UART1: u32 = 4;
pub const CLK_SCLK_UART2: u32 = 5;
pub const CLK_SCLK_UART3: u32 = 6;
pub const CLK_SCLK_SPI0: u32 = 7;
pub const CLK_SCLK_SPI1: u32 = 8;
pub const CLK_SCLK_SPI2: u32 = 9;
pub const CLK_SCLK_SPI3: u32 = 10;
pub const CLK_SCLK_SPI4: u32 = 11;
pub const CLK_SCLK_SPDIF: u32 = 12;
pub const CLK_SCLK_PCM1: u32 = 13;
pub const CLK_SCLK_I2S1: u32 = 14;
pub const CLK_ACLK_PERIC0_66: u32 = 15;
pub const CLK_ACLK_PERIC1_66: u32 = 16;
pub const TOP0_NR_CLK: u32 = 17;

/* TOP1 */
pub const DOUT_ACLK_FSYS1_200: u32 = 1;
pub const DOUT_ACLK_FSYS0_200: u32 = 2;
pub const DOUT_SCLK_MMC2: u32 = 3;
pub const DOUT_SCLK_MMC1: u32 = 4;
pub const DOUT_SCLK_MMC0: u32 = 5;
pub const CLK_SCLK_MMC2: u32 = 6;
pub const CLK_SCLK_MMC1: u32 = 7;
pub const CLK_SCLK_MMC0: u32 = 8;
pub const CLK_ACLK_FSYS0_200: u32 = 9;
pub const CLK_ACLK_FSYS1_200: u32 = 10;
pub const CLK_SCLK_PHY_FSYS1: u32 = 11;
pub const CLK_SCLK_PHY_FSYS1_26M: u32 = 12;
pub const MOUT_SCLK_UFSUNIPRO20: u32 = 13;
pub const DOUT_SCLK_UFSUNIPRO20: u32 = 14;
pub const CLK_SCLK_UFSUNIPRO20: u32 = 15;
pub const DOUT_SCLK_PHY_FSYS1: u32 = 16;
pub const DOUT_SCLK_PHY_FSYS1_26M: u32 = 17;
pub const TOP1_NR_CLK: u32 = 18;

/* CCORE */
pub const PCLK_RTC: u32 = 1;
pub const CCORE_NR_CLK: u32 = 2;

/* PERIC0 */
pub const PCLK_UART0: u32 = 1;
pub const SCLK_UART0: u32 = 2;
pub const PCLK_HSI2C0: u32 = 3;
pub const PCLK_HSI2C1: u32 = 4;
pub const PCLK_HSI2C4: u32 = 5;
pub const PCLK_HSI2C5: u32 = 6;
pub const PCLK_HSI2C9: u32 = 7;
pub const PCLK_HSI2C10: u32 = 8;
pub const PCLK_HSI2C11: u32 = 9;
pub const PCLK_PWM: u32 = 10;
pub const SCLK_PWM: u32 = 11;
pub const PCLK_ADCIF: u32 = 12;
pub const PERIC0_NR_CLK: u32 = 13;

/* PERIC1 */
pub const PCLK_UART1: u32 = 1;
pub const PCLK_UART2: u32 = 2;
pub const PCLK_UART3: u32 = 3;
pub const SCLK_UART1: u32 = 4;
pub const SCLK_UART2: u32 = 5;
pub const SCLK_UART3: u32 = 6;
pub const PCLK_HSI2C2: u32 = 7;
pub const PCLK_HSI2C3: u32 = 8;
pub const PCLK_HSI2C6: u32 = 9;
pub const PCLK_HSI2C7: u32 = 10;
pub const PCLK_HSI2C8: u32 = 11;
pub const PCLK_SPI0: u32 = 12;
pub const PCLK_SPI1: u32 = 13;
pub const PCLK_SPI2: u32 = 14;
pub const PCLK_SPI3: u32 = 15;
pub const PCLK_SPI4: u32 = 16;
pub const SCLK_SPI0: u32 = 17;
pub const SCLK_SPI1: u32 = 18;
pub const SCLK_SPI2: u32 = 19;
pub const SCLK_SPI3: u32 = 20;
pub const SCLK_SPI4: u32 = 21;
pub const PCLK_I2S1: u32 = 22;
pub const PCLK_PCM1: u32 = 23;
pub const PCLK_SPDIF: u32 = 24;
pub const SCLK_I2S1: u32 = 25;
pub const SCLK_PCM1: u32 = 26;
pub const SCLK_SPDIF: u32 = 27;
pub const PERIC1_NR_CLK: u32 = 28;

/* PERIS */
pub const PCLK_CHIPID: u32 = 1;
pub const SCLK_CHIPID: u32 = 2;
pub const PCLK_WDT: u32 = 3;
pub const PCLK_TMU: u32 = 4;
pub const SCLK_TMU: u32 = 5;
pub const PERIS_NR_CLK: u32 = 6;

/* FSYS0 */
pub const ACLK_MMC2: u32 = 1;
pub const ACLK_AXIUS_USBDRD30X_FSYS0X: u32 = 2;
pub const ACLK_USBDRD300: u32 = 3;
pub const SCLK_USBDRD300_SUSPENDCLK: u32 = 4;
pub const SCLK_USBDRD300_REFCLK: u32 = 5;
pub const PHYCLK_USBDRD300_UDRD30_PIPE_PCLK_USER: u32 = 6;
pub const PHYCLK_USBDRD300_UDRD30_PHYCLK_USER: u32 = 7;
pub const OSCCLK_PHY_CLKOUT_USB30_PHY: u32 = 8;
pub const ACLK_PDMA0: u32 = 9;
pub const ACLK_PDMA1: u32 = 10;
pub const FSYS0_NR_CLK: u32 = 11;

/* FSYS1 */
pub const ACLK_MMC1: u32 = 1;
pub const ACLK_MMC0: u32 = 2;
pub const PHYCLK_UFS20_TX0_SYMBOL: u32 = 3;
pub const PHYCLK_UFS20_RX0_SYMBOL: u32 = 4;
pub const PHYCLK_UFS20_RX1_SYMBOL: u32 = 5;
pub const ACLK_UFS20_LINK: u32 = 6;
pub const SCLK_UFSUNIPRO20_USER: u32 = 7;
pub const PHYCLK_UFS20_RX1_SYMBOL_USER: u32 = 8;
pub const PHYCLK_UFS20_RX0_SYMBOL_USER: u32 = 9;
pub const PHYCLK_UFS20_TX0_SYMBOL_USER: u32 = 10;
pub const OSCCLK_PHY_CLKOUT_EMBEDDED_COMBO_PHY: u32 = 11;
pub const SCLK_COMBO_PHY_EMBEDDED_26M: u32 = 12;
pub const DOUT_PCLK_FSYS1: u32 = 13;
pub const PCLK_GPIO_FSYS1: u32 = 14;
pub const MOUT_FSYS1_PHYCLK_SEL1: u32 = 15;
pub const FSYS1_NR_CLK: u32 = 16;

/* MSCL */
pub const USERMUX_ACLK_MSCL_532: u32 = 1;
pub const DOUT_PCLK_MSCL: u32 = 2;
pub const ACLK_MSCL_0: u32 = 3;
pub const ACLK_MSCL_1: u32 = 4;
pub const ACLK_JPEG: u32 = 5;
pub const ACLK_G2D: u32 = 6;
pub const ACLK_LH_ASYNC_SI_MSCL_0: u32 = 7;
pub const ACLK_LH_ASYNC_SI_MSCL_1: u32 = 8;
pub const ACLK_AXI2ACEL_BRIDGE: u32 = 9;
pub const ACLK_XIU_MSCLX_0: u32 = 10;
pub const ACLK_XIU_MSCLX_1: u32 = 11;
pub const ACLK_QE_MSCL_0: u32 = 12;
pub const ACLK_QE_MSCL_1: u32 = 13;
pub const ACLK_QE_JPEG: u32 = 14;
pub const ACLK_QE_G2D: u32 = 15;
pub const ACLK_PPMU_MSCL_0: u32 = 16;
pub const ACLK_PPMU_MSCL_1: u32 = 17;
pub const ACLK_MSCLNP_133: u32 = 18;
pub const ACLK_AHB2APB_MSCL0P: u32 = 19;
pub const ACLK_AHB2APB_MSCL1P: u32 = 20;
pub const PCLK_MSCL_0: u32 = 21;
pub const PCLK_MSCL_1: u32 = 22;
pub const PCLK_JPEG: u32 = 23;
pub const PCLK_G2D: u32 = 24;
pub const PCLK_QE_MSCL_0: u32 = 25;
pub const PCLK_QE_MSCL_1: u32 = 26;
pub const PCLK_QE_JPEG: u32 = 27;
pub const PCLK_QE_G2D: u32 = 28;
pub const PCLK_PPMU_MSCL_0: u32 = 29;
pub const PCLK_PPMU_MSCL_1: u32 = 30;
pub const PCLK_AXI2ACEL_BRIDGE: u32 = 31;
pub const PCLK_PMU_MSCL: u32 = 32;
pub const MSCL_NR_CLK: u32 = 33;

/* AUD */
pub const SCLK_I2S: u32 = 1;
pub const SCLK_PCM: u32 = 2;
pub const PCLK_I2S: u32 = 3;
pub const PCLK_PCM: u32 = 4;
pub const ACLK_ADMA: u32 = 5;
pub const AUD_NR_CLK: u32 = 6;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
