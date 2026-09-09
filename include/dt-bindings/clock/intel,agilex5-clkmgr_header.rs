/* SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause */
/*
 * Copyright (C) 2023, Intel Corporation
 */

/* fixed rate clocks */
pub const AGILEX5_OSC1: u32 = 0;
pub const AGILEX5_CB_INTOSC_HS_DIV2_CLK: u32 = 1;
pub const AGILEX5_CB_INTOSC_LS_CLK: u32 = 2;
pub const AGILEX5_F2S_FREE_CLK: u32 = 3;

/* PLL clocks */
pub const AGILEX5_MAIN_PLL_CLK: u32 = 4;
pub const AGILEX5_MAIN_PLL_C0_CLK: u32 = 5;
pub const AGILEX5_MAIN_PLL_C1_CLK: u32 = 6;
pub const AGILEX5_MAIN_PLL_C2_CLK: u32 = 7;
pub const AGILEX5_MAIN_PLL_C3_CLK: u32 = 8;
pub const AGILEX5_PERIPH_PLL_CLK: u32 = 9;
pub const AGILEX5_PERIPH_PLL_C0_CLK: u32 = 10;
pub const AGILEX5_PERIPH_PLL_C1_CLK: u32 = 11;
pub const AGILEX5_PERIPH_PLL_C2_CLK: u32 = 12;
pub const AGILEX5_PERIPH_PLL_C3_CLK: u32 = 13;
pub const AGILEX5_CORE0_FREE_CLK: u32 = 14;
pub const AGILEX5_CORE1_FREE_CLK: u32 = 15;
pub const AGILEX5_CORE2_FREE_CLK: u32 = 16;
pub const AGILEX5_CORE3_FREE_CLK: u32 = 17;
pub const AGILEX5_DSU_FREE_CLK: u32 = 18;
pub const AGILEX5_BOOT_CLK: u32 = 19;

/* fixed factor clocks */
pub const AGILEX5_L3_MAIN_FREE_CLK: u32 = 20;
pub const AGILEX5_NOC_FREE_CLK: u32 = 21;
pub const AGILEX5_S2F_USR0_CLK: u32 = 22;
pub const AGILEX5_NOC_CLK: u32 = 23;
pub const AGILEX5_EMAC_A_FREE_CLK: u32 = 24;
pub const AGILEX5_EMAC_B_FREE_CLK: u32 = 25;
pub const AGILEX5_EMAC_PTP_FREE_CLK: u32 = 26;
pub const AGILEX5_GPIO_DB_FREE_CLK: u32 = 27;
pub const AGILEX5_S2F_USER0_FREE_CLK: u32 = 28;
pub const AGILEX5_S2F_USER1_FREE_CLK: u32 = 29;
pub const AGILEX5_PSI_REF_FREE_CLK: u32 = 30;
pub const AGILEX5_USB31_FREE_CLK: u32 = 31;

/* Gate clocks */
pub const AGILEX5_CORE0_CLK: u32 = 32;
pub const AGILEX5_CORE1_CLK: u32 = 33;
pub const AGILEX5_CORE2_CLK: u32 = 34;
pub const AGILEX5_CORE3_CLK: u32 = 35;
pub const AGILEX5_MPU_CLK: u32 = 36;
pub const AGILEX5_MPU_PERIPH_CLK: u32 = 37;
pub const AGILEX5_MPU_CCU_CLK: u32 = 38;
pub const AGILEX5_L4_MAIN_CLK: u32 = 39;
pub const AGILEX5_L4_MP_CLK: u32 = 40;
pub const AGILEX5_L4_SYS_FREE_CLK: u32 = 41;
pub const AGILEX5_L4_SP_CLK: u32 = 42;
pub const AGILEX5_CS_AT_CLK: u32 = 43;
pub const AGILEX5_CS_TRACE_CLK: u32 = 44;
pub const AGILEX5_CS_PDBG_CLK: u32 = 45;
pub const AGILEX5_EMAC1_CLK: u32 = 47;
pub const AGILEX5_EMAC2_CLK: u32 = 48;
pub const AGILEX5_EMAC_PTP_CLK: u32 = 49;
pub const AGILEX5_GPIO_DB_CLK: u32 = 50;
pub const AGILEX5_S2F_USER0_CLK: u32 = 51;
pub const AGILEX5_S2F_USER1_CLK: u32 = 52;
pub const AGILEX5_PSI_REF_CLK: u32 = 53;
pub const AGILEX5_USB31_SUSPEND_CLK: u32 = 54;
pub const AGILEX5_EMAC0_CLK: u32 = 46;
pub const AGILEX5_USB31_BUS_CLK_EARLY: u32 = 55;
pub const AGILEX5_USB2OTG_HCLK: u32 = 56;
pub const AGILEX5_SPIM_0_CLK: u32 = 57;
pub const AGILEX5_SPIM_1_CLK: u32 = 58;
pub const AGILEX5_SPIS_0_CLK: u32 = 59;
pub const AGILEX5_SPIS_1_CLK: u32 = 60;
pub const AGILEX5_DMA_CORE_CLK: u32 = 61;
pub const AGILEX5_DMA_HS_CLK: u32 = 62;
pub const AGILEX5_I3C_0_CORE_CLK: u32 = 63;
pub const AGILEX5_I3C_1_CORE_CLK: u32 = 64;
pub const AGILEX5_I2C_0_PCLK: u32 = 65;
pub const AGILEX5_I2C_1_PCLK: u32 = 66;
pub const AGILEX5_I2C_EMAC0_PCLK: u32 = 67;
pub const AGILEX5_I2C_EMAC1_PCLK: u32 = 68;
pub const AGILEX5_I2C_EMAC2_PCLK: u32 = 69;
pub const AGILEX5_UART_0_PCLK: u32 = 70;
pub const AGILEX5_UART_1_PCLK: u32 = 71;
pub const AGILEX5_SPTIMER_0_PCLK: u32 = 72;
pub const AGILEX5_SPTIMER_1_PCLK: u32 = 73;
pub const AGILEX5_DFI_CLK: u32 = 74;
pub const AGILEX5_NAND_NF_CLK: u32 = 75;
pub const AGILEX5_NAND_BCH_CLK: u32 = 76;
pub const AGILEX5_SDMMC_SDPHY_REG_CLK: u32 = 77;
pub const AGILEX5_SDMCLK: u32 = 78;
pub const AGILEX5_SOFTPHY_REG_PCLK: u32 = 79;
pub const AGILEX5_SOFTPHY_PHY_CLK: u32 = 80;
pub const AGILEX5_SOFTPHY_CTRL_CLK: u32 = 81;
pub const AGILEX5_NUM_CLKS: u32 = 82;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
