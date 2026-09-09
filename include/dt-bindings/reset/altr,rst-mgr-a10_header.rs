/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2014, Steffen Trumtrar <s.trumtrar@pengutronix.de>
 */

/* MPUMODRST */
pub const CPU0_RESET: u32 = 0;
pub const CPU1_RESET: u32 = 1;
pub const WDS_RESET: u32 = 2;
pub const SCUPER_RESET: u32 = 3;

/* PER0MODRST */
pub const EMAC0_RESET: u32 = 32;
pub const EMAC1_RESET: u32 = 33;
pub const EMAC2_RESET: u32 = 34;
pub const USB0_RESET: u32 = 35;
pub const USB1_RESET: u32 = 36;
pub const NAND_RESET: u32 = 37;
pub const QSPI_RESET: u32 = 38;
pub const SDMMC_RESET: u32 = 39;
pub const EMAC0_OCP_RESET: u32 = 40;
pub const EMAC1_OCP_RESET: u32 = 41;
pub const EMAC2_OCP_RESET: u32 = 42;
pub const USB0_OCP_RESET: u32 = 43;
pub const USB1_OCP_RESET: u32 = 44;
pub const NAND_OCP_RESET: u32 = 45;
pub const QSPI_OCP_RESET: u32 = 46;
pub const SDMMC_OCP_RESET: u32 = 47;
pub const DMA_RESET: u32 = 48;
pub const SPIM0_RESET: u32 = 49;
pub const SPIM1_RESET: u32 = 50;
pub const SPIS0_RESET: u32 = 51;
pub const SPIS1_RESET: u32 = 52;
pub const DMA_OCP_RESET: u32 = 53;
pub const EMAC_PTP_RESET: u32 = 54;
/* 55 is empty */
pub const DMAIF0_RESET: u32 = 56;
pub const DMAIF1_RESET: u32 = 57;
pub const DMAIF2_RESET: u32 = 58;
pub const DMAIF3_RESET: u32 = 59;
pub const DMAIF4_RESET: u32 = 60;
pub const DMAIF5_RESET: u32 = 61;
pub const DMAIF6_RESET: u32 = 62;
pub const DMAIF7_RESET: u32 = 63;

/* PER1MODRST */
pub const L4WD0_RESET: u32 = 64;
pub const L4WD1_RESET: u32 = 65;
pub const L4SYSTIMER0_RESET: u32 = 66;
pub const L4SYSTIMER1_RESET: u32 = 67;
pub const SPTIMER0_RESET: u32 = 68;
pub const SPTIMER1_RESET: u32 = 69;
/* 70-71 is reserved */
pub const I2C0_RESET: u32 = 72;
pub const I2C1_RESET: u32 = 73;
pub const I2C2_RESET: u32 = 74;
pub const I2C3_RESET: u32 = 75;
pub const I2C4_RESET: u32 = 76;
/* 77-79 is reserved */
pub const UART0_RESET: u32 = 80;
pub const UART1_RESET: u32 = 81;
/* 82-87 is reserved */
pub const GPIO0_RESET: u32 = 88;
pub const GPIO1_RESET: u32 = 89;
pub const GPIO2_RESET: u32 = 90;

/* BRGMODRST */
pub const HPS2FPGA_RESET: u32 = 96;
pub const LWHPS2FPGA_RESET: u32 = 97;
pub const FPGA2HPS_RESET: u32 = 98;
pub const F2SSDRAM0_RESET: u32 = 99;
pub const F2SSDRAM1_RESET: u32 = 100;
pub const F2SSDRAM2_RESET: u32 = 101;
pub const DDRSCH_RESET: u32 = 102;

/* SYSMODRST */
pub const ROM_RESET: u32 = 128;
pub const OCRAM_RESET: u32 = 129;
/* 130 is reserved */
pub const FPGAMGR_RESET: u32 = 131;
pub const S2F_RESET: u32 = 132;
pub const SYSDBG_RESET: u32 = 133;
pub const OCRAM_OCP_RESET: u32 = 134;

/* COLDMODRST */
pub const CLKMGRCOLD_RESET: u32 = 160;
/* 161-162 is reserved */
pub const S2FCOLD_RESET: u32 = 163;
pub const TIMESTAMPCOLD_RESET: u32 = 164;
pub const TAPCOLD_RESET: u32 = 165;
pub const HMCCOLD_RESET: u32 = 166;
pub const IOMGRCOLD_RESET: u32 = 167;

/* NRSTMODRST */
pub const NRSTPINOE_RESET: u32 = 192;

/* DBGMODRST */
pub const DBG_RESET: u32 = 224;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
