/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016 Intel Corporation. All rights reserved
 * Copyright (C) 2016 Altera Corporation. All rights reserved
 *
 * derived from Steffen Trumtrar's "altr,rst-mgr-a10.h"
 */

/* MPUMODRST */
pub const CPU0_RESET: u32 = 0;
pub const CPU1_RESET: u32 = 1;
pub const CPU2_RESET: u32 = 2;
pub const CPU3_RESET: u32 = 3;

/* PER0MODRST */
pub const EMAC0_RESET: u32 = 32;
pub const EMAC1_RESET: u32 = 33;
pub const EMAC2_RESET: u32 = 34;
pub const USB0_RESET: u32 = 35;
pub const USB1_RESET: u32 = 36;
pub const NAND_RESET: u32 = 37;
pub const COMBOPHY_RESET: u32 = 38;
pub const SDMMC_RESET: u32 = 39;
pub const EMAC0_OCP_RESET: u32 = 40;
pub const EMAC1_OCP_RESET: u32 = 41;
pub const EMAC2_OCP_RESET: u32 = 42;
pub const USB0_OCP_RESET: u32 = 43;
pub const USB1_OCP_RESET: u32 = 44;
pub const NAND_OCP_RESET: u32 = 45;
/* 46 is empty */
pub const SDMMC_OCP_RESET: u32 = 47;
pub const DMA_RESET: u32 = 48;
pub const SPIM0_RESET: u32 = 49;
pub const SPIM1_RESET: u32 = 50;
pub const SPIS0_RESET: u32 = 51;
pub const SPIS1_RESET: u32 = 52;
pub const DMA_OCP_RESET: u32 = 53;
pub const EMAC_PTP_RESET: u32 = 54;
/* 55 is empty*/
pub const DMAIF0_RESET: u32 = 56;
pub const DMAIF1_RESET: u32 = 57;
pub const DMAIF2_RESET: u32 = 58;
pub const DMAIF3_RESET: u32 = 59;
pub const DMAIF4_RESET: u32 = 60;
pub const DMAIF5_RESET: u32 = 61;
pub const DMAIF6_RESET: u32 = 62;
pub const DMAIF7_RESET: u32 = 63;

/* PER1MODRST */
pub const WATCHDOG0_RESET: u32 = 64;
pub const WATCHDOG1_RESET: u32 = 65;
pub const WATCHDOG2_RESET: u32 = 66;
pub const WATCHDOG3_RESET: u32 = 67;
pub const L4SYSTIMER0_RESET: u32 = 68;
pub const L4SYSTIMER1_RESET: u32 = 69;
pub const SPTIMER0_RESET: u32 = 70;
pub const SPTIMER1_RESET: u32 = 71;
pub const I2C0_RESET: u32 = 72;
pub const I2C1_RESET: u32 = 73;
pub const I2C2_RESET: u32 = 74;
pub const I2C3_RESET: u32 = 75;
pub const I2C4_RESET: u32 = 76;
pub const I3C0_RESET: u32 = 77;
pub const I3C1_RESET: u32 = 78;
/* 79 is empty */
pub const UART0_RESET: u32 = 80;
pub const UART1_RESET: u32 = 81;
/* 82-87 is empty */
pub const GPIO0_RESET: u32 = 88;
pub const GPIO1_RESET: u32 = 89;
pub const WATCHDOG4_RESET: u32 = 90;

/* BRGMODRST */
pub const SOC2FPGA_RESET: u32 = 96;
pub const LWHPS2FPGA_RESET: u32 = 97;
pub const FPGA2SOC_RESET: u32 = 98;
pub const F2SSDRAM0_RESET: u32 = 99;
pub const F2SSDRAM1_RESET: u32 = 100;
pub const F2SSDRAM2_RESET: u32 = 101;
pub const DDRSCH_RESET: u32 = 102;

/* COLDMODRST */
pub const CPUPO0_RESET: u32 = 160;
pub const CPUPO1_RESET: u32 = 161;
pub const CPUPO2_RESET: u32 = 162;
pub const CPUPO3_RESET: u32 = 163;
/* 164-167 is empty */
pub const L2_RESET: u32 = 168;

/* DBGMODRST */
pub const DBG_RESET: u32 = 224;
pub const CSDAP_RESET: u32 = 225;

/* TAPMODRST */
pub const TAP_RESET: u32 = 256;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
