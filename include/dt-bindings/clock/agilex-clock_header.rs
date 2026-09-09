/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2019, Intel Corporation
 */

/* fixed rate clocks */
pub const AGILEX_OSC1: u32 = 0;
pub const AGILEX_CB_INTOSC_HS_DIV2_CLK: u32 = 1;
pub const AGILEX_CB_INTOSC_LS_CLK: u32 = 2;
pub const AGILEX_L4_SYS_FREE_CLK: u32 = 3;
pub const AGILEX_F2S_FREE_CLK: u32 = 4;

/* PLL clocks */
pub const AGILEX_MAIN_PLL_CLK: u32 = 5;
pub const AGILEX_MAIN_PLL_C0_CLK: u32 = 6;
pub const AGILEX_MAIN_PLL_C1_CLK: u32 = 7;
pub const AGILEX_MAIN_PLL_C2_CLK: u32 = 8;
pub const AGILEX_MAIN_PLL_C3_CLK: u32 = 9;
pub const AGILEX_PERIPH_PLL_CLK: u32 = 10;
pub const AGILEX_PERIPH_PLL_C0_CLK: u32 = 11;
pub const AGILEX_PERIPH_PLL_C1_CLK: u32 = 12;
pub const AGILEX_PERIPH_PLL_C2_CLK: u32 = 13;
pub const AGILEX_PERIPH_PLL_C3_CLK: u32 = 14;
pub const AGILEX_MPU_FREE_CLK: u32 = 15;
pub const AGILEX_MPU_CCU_CLK: u32 = 16;
pub const AGILEX_BOOT_CLK: u32 = 17;

/* fixed factor clocks */
pub const AGILEX_L3_MAIN_FREE_CLK: u32 = 18;
pub const AGILEX_NOC_FREE_CLK: u32 = 19;
pub const AGILEX_S2F_USR0_CLK: u32 = 20;
pub const AGILEX_NOC_CLK: u32 = 21;
pub const AGILEX_EMAC_A_FREE_CLK: u32 = 22;
pub const AGILEX_EMAC_B_FREE_CLK: u32 = 23;
pub const AGILEX_EMAC_PTP_FREE_CLK: u32 = 24;
pub const AGILEX_GPIO_DB_FREE_CLK: u32 = 25;
pub const AGILEX_SDMMC_FREE_CLK: u32 = 26;
pub const AGILEX_S2F_USER0_FREE_CLK: u32 = 27;
pub const AGILEX_S2F_USER1_FREE_CLK: u32 = 28;
pub const AGILEX_PSI_REF_FREE_CLK: u32 = 29;

/* Gate clocks */
pub const AGILEX_MPU_CLK: u32 = 30;
pub const AGILEX_MPU_L2RAM_CLK: u32 = 31;
pub const AGILEX_MPU_PERIPH_CLK: u32 = 32;
pub const AGILEX_L4_MAIN_CLK: u32 = 33;
pub const AGILEX_L4_MP_CLK: u32 = 34;
pub const AGILEX_L4_SP_CLK: u32 = 35;
pub const AGILEX_CS_AT_CLK: u32 = 36;
pub const AGILEX_CS_TRACE_CLK: u32 = 37;
pub const AGILEX_CS_PDBG_CLK: u32 = 38;
pub const AGILEX_CS_TIMER_CLK: u32 = 39;
pub const AGILEX_S2F_USER0_CLK: u32 = 40;
pub const AGILEX_EMAC0_CLK: u32 = 41;
pub const AGILEX_EMAC1_CLK: u32 = 43;
pub const AGILEX_EMAC2_CLK: u32 = 44;
pub const AGILEX_EMAC_PTP_CLK: u32 = 45;
pub const AGILEX_GPIO_DB_CLK: u32 = 46;
pub const AGILEX_NAND_CLK: u32 = 47;
pub const AGILEX_PSI_REF_CLK: u32 = 48;
pub const AGILEX_S2F_USER1_CLK: u32 = 49;
pub const AGILEX_SDMMC_CLK: u32 = 50;
pub const AGILEX_SPI_M_CLK: u32 = 51;
pub const AGILEX_USB_CLK: u32 = 52;
pub const AGILEX_NAND_X_CLK: u32 = 53;
pub const AGILEX_NAND_ECC_CLK: u32 = 54;
pub const AGILEX_NUM_CLKS: u32 = 55;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
