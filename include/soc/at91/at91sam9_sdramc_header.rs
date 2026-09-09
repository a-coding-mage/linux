/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * arch/arm/mach-at91/include/mach/at91sam9_sdramc.h
 *
 * Copyright (C) 2007 Andrew Victor
 * Copyright (C) 2007 Atmel Corporation.
 *
 * SDRAM Controllers (SDRAMC) - System peripherals registers.
 * Based on AT91SAM9261 datasheet revision D.
 */

/* SDRAM Controller (SDRAMC) registers */
pub const AT91_SDRAMC_MR: u32 = 0x00; /* SDRAM Controller Mode Register */
pub const AT91_SDRAMC_MODE: u32 = 0xf << 0; /* Command Mode */
pub const AT91_SDRAMC_MODE_NORMAL: u32 = 0;
pub const AT91_SDRAMC_MODE_NOP: u32 = 1;
pub const AT91_SDRAMC_MODE_PRECHARGE: u32 = 2;
pub const AT91_SDRAMC_MODE_LMR: u32 = 3;
pub const AT91_SDRAMC_MODE_REFRESH: u32 = 4;
pub const AT91_SDRAMC_MODE_EXT_LMR: u32 = 5;
pub const AT91_SDRAMC_MODE_DEEP: u32 = 6;

pub const AT91_SDRAMC_TR: u32 = 0x04; /* SDRAM Controller Refresh Timer Register */
pub const AT91_SDRAMC_COUNT: u32 = 0xfff << 0; /* Refresh Timer Counter */

pub const AT91_SDRAMC_CR: u32 = 0x08; /* SDRAM Controller Configuration Register */
pub const AT91_SDRAMC_NC: u32 = 3 << 0; /* Number of Column Bits */
pub const AT91_SDRAMC_NC_8: u32 = 0 << 0;
pub const AT91_SDRAMC_NC_9: u32 = 1 << 0;
pub const AT91_SDRAMC_NC_10: u32 = 2 << 0;
pub const AT91_SDRAMC_NC_11: u32 = 3 << 0;
pub const AT91_SDRAMC_NR: u32 = 3 << 2; /* Number of Row Bits */
pub const AT91_SDRAMC_NR_11: u32 = 0 << 2;
pub const AT91_SDRAMC_NR_12: u32 = 1 << 2;
pub const AT91_SDRAMC_NR_13: u32 = 2 << 2;
pub const AT91_SDRAMC_NB: u32 = 1 << 4; /* Number of Banks */
pub const AT91_SDRAMC_NB_2: u32 = 0 << 4;
pub const AT91_SDRAMC_NB_4: u32 = 1 << 4;
pub const AT91_SDRAMC_CAS: u32 = 3 << 5; /* CAS Latency */
pub const AT91_SDRAMC_CAS_1: u32 = 1 << 5;
pub const AT91_SDRAMC_CAS_2: u32 = 2 << 5;
pub const AT91_SDRAMC_CAS_3: u32 = 3 << 5;
pub const AT91_SDRAMC_DBW: u32 = 1 << 7; /* Data Bus Width */
pub const AT91_SDRAMC_DBW_32: u32 = 0 << 7;
pub const AT91_SDRAMC_DBW_16: u32 = 1 << 7;
pub const AT91_SDRAMC_TWR: u32 = 0xf << 8; /* Write Recovery Delay */
pub const AT91_SDRAMC_TRC: u32 = 0xf << 12; /* Row Cycle Delay */
pub const AT91_SDRAMC_TRP: u32 = 0xf << 16; /* Row Precharge Delay */
pub const AT91_SDRAMC_TRCD: u32 = 0xf << 20; /* Row to Column Delay */
pub const AT91_SDRAMC_TRAS: u32 = 0xf << 24; /* Active to Precharge Delay */
pub const AT91_SDRAMC_TXSR: u32 = 0xf << 28; /* Exit Self Refresh to Active Delay */

pub const AT91_SDRAMC_LPR: u32 = 0x10; /* SDRAM Controller Low Power Register */
pub const AT91_SDRAMC_LPCB: u32 = 3 << 0; /* Low-power Configurations */
pub const AT91_SDRAMC_LPCB_DISABLE: u32 = 0;
pub const AT91_SDRAMC_LPCB_SELF_REFRESH: u32 = 1;
pub const AT91_SDRAMC_LPCB_POWER_DOWN: u32 = 2;
pub const AT91_SDRAMC_LPCB_DEEP_POWER_DOWN: u32 = 3;
pub const AT91_SDRAMC_PASR: u32 = 7 << 4; /* Partial Array Self Refresh */
pub const AT91_SDRAMC_TCSR: u32 = 3 << 8; /* Temperature Compensated Self Refresh */
pub const AT91_SDRAMC_DS: u32 = 3 << 10; /* Drive Strength */
pub const AT91_SDRAMC_TIMEOUT: u32 = 3 << 12; /* Time to define when Low Power Mode is enabled */
pub const AT91_SDRAMC_TIMEOUT_0_CLK_CYCLES: u32 = 0 << 12;
pub const AT91_SDRAMC_TIMEOUT_64_CLK_CYCLES: u32 = 1 << 12;
pub const AT91_SDRAMC_TIMEOUT_128_CLK_CYCLES: u32 = 2 << 12;

pub const AT91_SDRAMC_IER: u32 = 0x14; /* SDRAM Controller Interrupt Enable Register */
pub const AT91_SDRAMC_IDR: u32 = 0x18; /* SDRAM Controller Interrupt Disable Register */
pub const AT91_SDRAMC_IMR: u32 = 0x1C; /* SDRAM Controller Interrupt Mask Register */
pub const AT91_SDRAMC_ISR: u32 = 0x20; /* SDRAM Controller Interrupt Status Register */
pub const AT91_SDRAMC_RES: u32 = 1 << 0; /* Refresh Error Status */

pub const AT91_SDRAMC_MDR: u32 = 0x24; /* SDRAM Memory Device Register */
pub const AT91_SDRAMC_MD: u32 = 3 << 0; /* Memory Device Type */
pub const AT91_SDRAMC_MD_SDRAM: u32 = 0;
pub const AT91_SDRAMC_MD_LOW_POWER_SDRAM: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
