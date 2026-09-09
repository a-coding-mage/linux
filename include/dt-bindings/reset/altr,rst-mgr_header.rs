/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2014, Steffen Trumtrar <s.trumtrar@pengutronix.de>
 */

/* MPUMODRST */
pub const CPU0_RESET: i32 = 0;
pub const CPU1_RESET: i32 = 1;
pub const WDS_RESET: i32 = 2;
pub const SCUPER_RESET: i32 = 3;
pub const L2_RESET: i32 = 4;

/* PERMODRST */
pub const EMAC0_RESET: i32 = 32;
pub const EMAC1_RESET: i32 = 33;
pub const USB0_RESET: i32 = 34;
pub const USB1_RESET: i32 = 35;
pub const NAND_RESET: i32 = 36;
pub const QSPI_RESET: i32 = 37;
pub const L4WD0_RESET: i32 = 38;
pub const L4WD1_RESET: i32 = 39;
pub const OSC1TIMER0_RESET: i32 = 40;
pub const OSC1TIMER1_RESET: i32 = 41;
pub const SPTIMER0_RESET: i32 = 42;
pub const SPTIMER1_RESET: i32 = 43;
pub const I2C0_RESET: i32 = 44;
pub const I2C1_RESET: i32 = 45;
pub const I2C2_RESET: i32 = 46;
pub const I2C3_RESET: i32 = 47;
pub const UART0_RESET: i32 = 48;
pub const UART1_RESET: i32 = 49;
pub const SPIM0_RESET: i32 = 50;
pub const SPIM1_RESET: i32 = 51;
pub const SPIS0_RESET: i32 = 52;
pub const SPIS1_RESET: i32 = 53;
pub const SDMMC_RESET: i32 = 54;
pub const CAN0_RESET: i32 = 55;
pub const CAN1_RESET: i32 = 56;
pub const GPIO0_RESET: i32 = 57;
pub const GPIO1_RESET: i32 = 58;
pub const GPIO2_RESET: i32 = 59;
pub const DMA_RESET: i32 = 60;
pub const SDR_RESET: i32 = 61;

/* PER2MODRST */
pub const DMAIF0_RESET: i32 = 64;
pub const DMAIF1_RESET: i32 = 65;
pub const DMAIF2_RESET: i32 = 66;
pub const DMAIF3_RESET: i32 = 67;
pub const DMAIF4_RESET: i32 = 68;
pub const DMAIF5_RESET: i32 = 69;
pub const DMAIF6_RESET: i32 = 70;
pub const DMAIF7_RESET: i32 = 71;

/* BRGMODRST */
pub const HPS2FPGA_RESET: i32 = 96;
pub const LWHPS2FPGA_RESET: i32 = 97;
pub const FPGA2HPS_RESET: i32 = 98;

/* MISCMODRST*/
pub const ROM_RESET: i32 = 128;
pub const OCRAM_RESET: i32 = 129;
pub const SYSMGR_RESET: i32 = 130;
pub const SYSMGRCOLD_RESET: i32 = 131;
pub const FPGAMGR_RESET: i32 = 132;
pub const ACPIDMAP_RESET: i32 = 133;
pub const S2F_RESET: i32 = 134;
pub const S2FCOLD_RESET: i32 = 135;
pub const NRSTPIN_RESET: i32 = 136;
pub const TIMESTAMPCOLD_RESET: i32 = 137;
pub const CLKMGRCOLD_RESET: i32 = 138;
pub const SCANMGR_RESET: i32 = 139;
pub const FRZCTRLCOLD_RESET: i32 = 140;
pub const SYSDBG_RESET: i32 = 141;
pub const DBG_RESET: i32 = 142;
pub const TAPCOLD_RESET: i32 = 143;
pub const SDRCOLD_RESET: i32 = 144;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
