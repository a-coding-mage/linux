/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Header file for the Atmel DDR/SDR SDRAM Controller
 *
 * Copyright (C) 2010 Atmel Corporation
 *	Nicolas Ferre <nicolas.ferre@atmel.com>
 */

pub const AT91_DDRSDRC_MR: u32 = 0x00; /* Mode Register */
pub const AT91_DDRSDRC_MODE: u32 = 0x7 << 0; /* Command Mode */
pub const AT91_DDRSDRC_MODE_NORMAL: u32 = 0;
pub const AT91_DDRSDRC_MODE_NOP: u32 = 1;
pub const AT91_DDRSDRC_MODE_PRECHARGE: u32 = 2;
pub const AT91_DDRSDRC_MODE_LMR: u32 = 3;
pub const AT91_DDRSDRC_MODE_REFRESH: u32 = 4;
pub const AT91_DDRSDRC_MODE_EXT_LMR: u32 = 5;
pub const AT91_DDRSDRC_MODE_DEEP: u32 = 6;

pub const AT91_DDRSDRC_RTR: u32 = 0x04; /* Refresh Timer Register */
pub const AT91_DDRSDRC_COUNT: u32 = 0xfff << 0; /* Refresh Timer Counter */

pub const AT91_DDRSDRC_CR: u32 = 0x08; /* Configuration Register */
pub const AT91_DDRSDRC_NC: u32 = 3 << 0; /* Number of Column Bits */
pub const AT91_DDRSDRC_NC_SDR8: u32 = 0 << 0;
pub const AT91_DDRSDRC_NC_SDR9: u32 = 1 << 0;
pub const AT91_DDRSDRC_NC_SDR10: u32 = 2 << 0;
pub const AT91_DDRSDRC_NC_SDR11: u32 = 3 << 0;
pub const AT91_DDRSDRC_NC_DDR9: u32 = 0 << 0;
pub const AT91_DDRSDRC_NC_DDR10: u32 = 1 << 0;
pub const AT91_DDRSDRC_NC_DDR11: u32 = 2 << 0;
pub const AT91_DDRSDRC_NC_DDR12: u32 = 3 << 0;
pub const AT91_DDRSDRC_NR: u32 = 3 << 2; /* Number of Row Bits */
pub const AT91_DDRSDRC_NR_11: u32 = 0 << 2;
pub const AT91_DDRSDRC_NR_12: u32 = 1 << 2;
pub const AT91_DDRSDRC_NR_13: u32 = 2 << 2;
pub const AT91_DDRSDRC_NR_14: u32 = 3 << 2;
pub const AT91_DDRSDRC_CAS: u32 = 7 << 4; /* CAS Latency */
pub const AT91_DDRSDRC_CAS_2: u32 = 2 << 4;
pub const AT91_DDRSDRC_CAS_3: u32 = 3 << 4;
pub const AT91_DDRSDRC_CAS_25: u32 = 6 << 4;
pub const AT91_DDRSDRC_RST_DLL: u32 = 1 << 7; /* Reset DLL */
pub const AT91_DDRSDRC_DICDS: u32 = 1 << 8; /* Output impedance control */
pub const AT91_DDRSDRC_DIS_DLL: u32 = 1 << 9; /* Disable DLL [SAM9 Only] */
pub const AT91_DDRSDRC_OCD: u32 = 1 << 12; /* Off-Chip Driver [SAM9 Only] */
pub const AT91_DDRSDRC_DQMS: u32 = 1 << 16; /* Mask Data is Shared [SAM9 Only] */
pub const AT91_DDRSDRC_ACTBST: u32 = 1 << 18; /* Active Bank X to Burst Stop Read Access Bank Y [SAM9 Only] */

pub const AT91_DDRSDRC_T0PR: u32 = 0x0C; /* Timing 0 Register */
pub const AT91_DDRSDRC_TRAS: u32 = 0xf << 0; /* Active to Precharge delay */
pub const AT91_DDRSDRC_TRCD: u32 = 0xf << 4; /* Row to Column delay */
pub const AT91_DDRSDRC_TWR: u32 = 0xf << 8; /* Write recovery delay */
pub const AT91_DDRSDRC_TRC: u32 = 0xf << 12; /* Row cycle delay */
pub const AT91_DDRSDRC_TRP: u32 = 0xf << 16; /* Row precharge delay */
pub const AT91_DDRSDRC_TRRD: u32 = 0xf << 20; /* Active BankA to BankB */
pub const AT91_DDRSDRC_TWTR: u32 = 0x7 << 24; /* Internal Write to Read delay */
pub const AT91_DDRSDRC_RED_WRRD: u32 = 0x1 << 27; /* Reduce Write to Read Delay [SAM9 Only] */
pub const AT91_DDRSDRC_TMRD: u32 = 0xf << 28; /* Load mode to active/refresh delay */

pub const AT91_DDRSDRC_T1PR: u32 = 0x10; /* Timing 1 Register */
pub const AT91_DDRSDRC_TRFC: u32 = 0x1f << 0; /* Row Cycle Delay */
pub const AT91_DDRSDRC_TXSNR: u32 = 0xff << 8; /* Exit self-refresh to non-read */
pub const AT91_DDRSDRC_TXSRD: u32 = 0xff << 16; /* Exit self-refresh to read */
pub const AT91_DDRSDRC_TXP: u32 = 0xf << 24; /* Exit power-down delay */

pub const AT91_DDRSDRC_T2PR: u32 = 0x14; /* Timing 2 Register [SAM9 Only] */
pub const AT91_DDRSDRC_TXARD: u32 = 0xf << 0; /* Exit active power down delay to read command in mode "Fast Exit" */
pub const AT91_DDRSDRC_TXARDS: u32 = 0xf << 4; /* Exit active power down delay to read command in mode "Slow Exit" */
pub const AT91_DDRSDRC_TRPA: u32 = 0xf << 8; /* Row Precharge All delay */
pub const AT91_DDRSDRC_TRTP: u32 = 0x7 << 12; /* Read to Precharge delay */

pub const AT91_DDRSDRC_LPR: u32 = 0x1C; /* Low Power Register */
pub const AT91_DDRSDRC_LPCB: u32 = 3 << 0; /* Low-power Configurations */
pub const AT91_DDRSDRC_LPCB_DISABLE: u32 = 0;
pub const AT91_DDRSDRC_LPCB_SELF_REFRESH: u32 = 1;
pub const AT91_DDRSDRC_LPCB_POWER_DOWN: u32 = 2;
pub const AT91_DDRSDRC_LPCB_DEEP_POWER_DOWN: u32 = 3;
pub const AT91_DDRSDRC_CLKFR: u32 = 1 << 2; /* Clock Frozen */
pub const AT91_DDRSDRC_LPDDR2_PWOFF: u32 = 1 << 3; /* LPDDR Power Off */
pub const AT91_DDRSDRC_PASR: u32 = 7 << 4; /* Partial Array Self Refresh */
pub const AT91_DDRSDRC_TCSR: u32 = 3 << 8; /* Temperature Compensated Self Refresh */
pub const AT91_DDRSDRC_DS: u32 = 3 << 10; /* Drive Strength */
pub const AT91_DDRSDRC_TIMEOUT: u32 = 3 << 12; /* Time to define when Low Power Mode is enabled */
pub const AT91_DDRSDRC_TIMEOUT_0_CLK_CYCLES: u32 = 0 << 12;
pub const AT91_DDRSDRC_TIMEOUT_64_CLK_CYCLES: u32 = 1 << 12;
pub const AT91_DDRSDRC_TIMEOUT_128_CLK_CYCLES: u32 = 2 << 12;
pub const AT91_DDRSDRC_APDE: u32 = 1 << 16; /* Active power down exit time */
pub const AT91_DDRSDRC_UPD_MR: u32 = 3 << 20; /* Update load mode register and extended mode register */

pub const AT91_DDRSDRC_MDR: u32 = 0x20; /* Memory Device Register */
pub const AT91_DDRSDRC_MD: u32 = 7 << 0; /* Memory Device Type */
pub const AT91_DDRSDRC_MD_SDR: u32 = 0;
pub const AT91_DDRSDRC_MD_LOW_POWER_SDR: u32 = 1;
pub const AT91_DDRSDRC_MD_LOW_POWER_DDR: u32 = 3;
pub const AT91_DDRSDRC_MD_LPDDR3: u32 = 5;
pub const AT91_DDRSDRC_MD_DDR2: u32 = 6; /* [SAM9 Only] */
pub const AT91_DDRSDRC_MD_LPDDR2: u32 = 7;
pub const AT91_DDRSDRC_DBW: u32 = 1 << 4; /* Data Bus Width */
pub const AT91_DDRSDRC_DBW_32BITS: u32 = 0 << 4;
pub const AT91_DDRSDRC_DBW_16BITS: u32 = 1 << 4;

pub const AT91_DDRSDRC_DLL: u32 = 0x24; /* DLL Information Register */
pub const AT91_DDRSDRC_MDINC: u32 = 1 << 0; /* Master Delay increment */
pub const AT91_DDRSDRC_MDDEC: u32 = 1 << 1; /* Master Delay decrement */
pub const AT91_DDRSDRC_MDOVF: u32 = 1 << 2; /* Master Delay Overflow */
pub const AT91_DDRSDRC_MDVAL: u32 = 0xff << 8; /* Master Delay value */

pub const AT91_DDRSDRC_HS: u32 = 0x2C; /* High Speed Register [SAM9 Only] */
pub const AT91_DDRSDRC_DIS_ATCP_RD: u32 = 1 << 2; /* Anticip read access is disabled */

#[inline]
pub const fn AT91_DDRSDRC_DELAY(n: u32) -> u32 { 0x30 + (0x4 * n) } /* Delay I/O Register n */

pub const AT91_DDRSDRC_WPMR: u32 = 0xE4; /* Write Protect Mode Register [SAM9 Only] */
pub const AT91_DDRSDRC_WP: u32 = 1 << 0; /* Write protect enable */
pub const AT91_DDRSDRC_WPKEY: u32 = 0xffffff << 8; /* Write protect key */
pub const AT91_DDRSDRC_KEY: u32 = 0x444452 << 8; /* Write protect key = "DDR" */

pub const AT91_DDRSDRC_WPSR: u32 = 0xE8; /* Write Protect Status Register [SAM9 Only] */
pub const AT91_DDRSDRC_WPVS: u32 = 1 << 0; /* Write protect violation status */
pub const AT91_DDRSDRC_WPVSRC: u32 = 0xffff << 8; /* Write protect violation source */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
