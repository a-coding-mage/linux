/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Tenstorrent Atlantis PRCM Clock and Reset Indices
 *
 * Copyright (c) 2026 Tenstorrent
 */

/*
 * RCPU Domain Clock IDs
 */
pub const CLK_RCPU_PLL: u32 = 0;
pub const CLK_RCPU_ROOT: u32 = 1;
pub const CLK_RCPU_DIV2: u32 = 2;
pub const CLK_RCPU_DIV4: u32 = 3;
pub const CLK_RCPU_RTC: u32 = 4;
pub const CLK_SMNDMA0_ACLK: u32 = 5;
pub const CLK_SMNDMA1_ACLK: u32 = 6;
pub const CLK_WDT0_PCLK: u32 = 7;
pub const CLK_WDT1_PCLK: u32 = 8;
pub const CLK_TIMER_PCLK: u32 = 9;
pub const CLK_PVTC_PCLK: u32 = 10;
pub const CLK_PMU_PCLK: u32 = 11;
pub const CLK_MAILBOX_HCLK: u32 = 12;
pub const CLK_SEC_SPACC_HCLK: u32 = 13;
pub const CLK_SEC_OTP_HCLK: u32 = 14;
pub const CLK_TRNG_PCLK: u32 = 15;
pub const CLK_SEC_CRC_HCLK: u32 = 16;
pub const CLK_SMN_HCLK: u32 = 17;
pub const CLK_AHB0_HCLK: u32 = 18;
pub const CLK_SMN_PCLK: u32 = 19;
pub const CLK_SMN_CLK: u32 = 20;
pub const CLK_SCRATCHPAD_CLK: u32 = 21;
pub const CLK_RCPU_CORE_CLK: u32 = 22;
pub const CLK_RCPU_ROM_CLK: u32 = 23;
pub const CLK_OTP_LOAD_CLK: u32 = 24;
pub const CLK_NOC_PLL: u32 = 25;
pub const CLK_NOCC_CLK: u32 = 26;
pub const CLK_NOCC_DIV2: u32 = 27;
pub const CLK_NOCC_DIV4: u32 = 28;
pub const CLK_NOCC_RTC: u32 = 29;
pub const CLK_NOCC_CAN: u32 = 30;
pub const CLK_QSPI_SCLK: u32 = 31;
pub const CLK_QSPI_HCLK: u32 = 32;
pub const CLK_I2C0_PCLK: u32 = 33;
pub const CLK_I2C1_PCLK: u32 = 34;
pub const CLK_I2C2_PCLK: u32 = 35;
pub const CLK_I2C3_PCLK: u32 = 36;
pub const CLK_I2C4_PCLK: u32 = 37;
pub const CLK_UART0_PCLK: u32 = 38;
pub const CLK_UART1_PCLK: u32 = 39;
pub const CLK_UART2_PCLK: u32 = 40;
pub const CLK_UART3_PCLK: u32 = 41;
pub const CLK_UART4_PCLK: u32 = 42;
pub const CLK_SPI0_PCLK: u32 = 43;
pub const CLK_SPI1_PCLK: u32 = 44;
pub const CLK_SPI2_PCLK: u32 = 45;
pub const CLK_SPI3_PCLK: u32 = 46;
pub const CLK_GPIO_PCLK: u32 = 47;
pub const CLK_CAN0_HCLK: u32 = 48;
pub const CLK_CAN0_CLK: u32 = 49;
pub const CLK_CAN1_HCLK: u32 = 50;
pub const CLK_CAN1_CLK: u32 = 51;
pub const CLK_CAN0_TIMER_CLK: u32 = 52;
pub const CLK_CAN1_TIMER_CLK: u32 = 53;

/* RCPU domain reset */
pub const RST_SMNDMA0: u32 = 0;
pub const RST_SMNDMA1: u32 = 1;
pub const RST_WDT0: u32 = 2;
pub const RST_WDT1: u32 = 3;
pub const RST_TMR: u32 = 4;
pub const RST_PVTC: u32 = 5;
pub const RST_PMU: u32 = 6;
pub const RST_MAILBOX: u32 = 7;
pub const RST_SPACC: u32 = 8;
pub const RST_OTP: u32 = 9;
pub const RST_TRNG: u32 = 10;
pub const RST_CRC: u32 = 11;
pub const RST_QSPI: u32 = 12;
pub const RST_I2C0: u32 = 13;
pub const RST_I2C1: u32 = 14;
pub const RST_I2C2: u32 = 15;
pub const RST_I2C3: u32 = 16;
pub const RST_I2C4: u32 = 17;
pub const RST_UART0: u32 = 18;
pub const RST_UART1: u32 = 19;
pub const RST_UART2: u32 = 20;
pub const RST_UART3: u32 = 21;
pub const RST_UART4: u32 = 22;
pub const RST_SPI0: u32 = 23;
pub const RST_SPI1: u32 = 24;
pub const RST_SPI2: u32 = 25;
pub const RST_SPI3: u32 = 26;
pub const RST_GPIO: u32 = 27;
pub const RST_CAN0: u32 = 28;
pub const RST_CAN1: u32 = 29;
pub const RST_I2S0: u32 = 30;
pub const RST_I2S1: u32 = 31;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
