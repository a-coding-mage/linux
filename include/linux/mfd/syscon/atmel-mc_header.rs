/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2005 Ivan Kokshaysky
 * Copyright (C) SAN People
 *
 * Memory Controllers (MC, EBI, SMC, SDRAMC, BFC) - System peripherals
 * registers.
 * Based on AT91RM9200 datasheet revision E.
 */

/* Memory Controller */
pub const AT91_MC_RCR: u32 = 0x00;
pub const AT91_MC_RCB: _ = BIT(0);

pub const AT91_MC_ASR: u32 = 0x04;
pub const AT91_MC_UNADD: _ = BIT(0);
pub const AT91_MC_MISADD: _ = BIT(1);
pub const AT91_MC_ABTSZ: _ = GENMASK(9, 8);
pub const AT91_MC_ABTSZ_BYTE: _ = 0 << 8;
pub const AT91_MC_ABTSZ_HALFWORD: _ = 1 << 8;
pub const AT91_MC_ABTSZ_WORD: _ = 2 << 8;
pub const AT91_MC_ABTTYP: _ = GENMASK(11, 10);
pub const AT91_MC_ABTTYP_DATAREAD: _ = 0 << 10;
pub const AT91_MC_ABTTYP_DATAWRITE: _ = 1 << 10;
pub const AT91_MC_ABTTYP_FETCH: _ = 2 << 10;
macro_rules! AT91_MC_MST { ($n:expr) => { BIT(16 + ($n)) }; }
macro_rules! AT91_MC_SVMST { ($n:expr) => { BIT(24 + ($n)) }; }

pub const AT91_MC_AASR: u32 = 0x08;

pub const AT91_MC_MPR: u32 = 0x0c;
macro_rules! AT91_MPR_MSTP { ($x:expr) => { GENMASK(2 + (($x) * 4), (($x) * 4)) }; }

/* External Bus Interface (EBI) registers */
pub const AT91_MC_EBI_CSA: u32 = 0x60;
macro_rules! AT91_MC_EBI_CS { ($x:expr) => { BIT($x) }; }
pub const AT91_MC_EBI_NUM_CS: u32 = 8;

pub const AT91_MC_EBI_CFGR: u32 = 0x64;
pub const AT91_MC_EBI_DBPUC: _ = BIT(0);

/* Static Memory Controller (SMC) registers */
macro_rules! AT91_MC_SMC_CSR { ($n:expr) => { 0x70 + (($n) * 4) }; }
pub const AT91_MC_SMC_NWS: _ = GENMASK(6, 0);
macro_rules! AT91_MC_SMC_NWS_ { ($x:expr) => { ($x) << 0 }; }
pub const AT91_MC_SMC_WSEN: _ = BIT(7);
pub const AT91_MC_SMC_TDF: _ = GENMASK(11, 8);
macro_rules! AT91_MC_SMC_TDF_ { ($x:expr) => { ($x) << 8 }; }
pub const AT91_MC_SMC_TDF_MAX: u32 = 0xf;
pub const AT91_MC_SMC_BAT: _ = BIT(12);
pub const AT91_MC_SMC_DBW: _ = GENMASK(14, 13);
pub const AT91_MC_SMC_DBW_16: _ = 1 << 13;
pub const AT91_MC_SMC_DBW_8: _ = 2 << 13;
pub const AT91_MC_SMC_DPR: _ = BIT(15);
pub const AT91_MC_SMC_ACSS: _ = GENMASK(17, 16);
macro_rules! AT91_MC_SMC_ACSS_ { ($x:expr) => { ($x) << 16 }; }
pub const AT91_MC_SMC_ACSS_MAX: u32 = 3;
pub const AT91_MC_SMC_RWSETUP: _ = GENMASK(26, 24);
macro_rules! AT91_MC_SMC_RWSETUP_ { ($x:expr) => { ($x) << 24 }; }
pub const AT91_MC_SMC_RWHOLD: _ = GENMASK(30, 28);
macro_rules! AT91_MC_SMC_RWHOLD_ { ($x:expr) => { ($x) << 28 }; }
pub const AT91_MC_SMC_RWHOLDSETUP_MAX: u32 = 7;

/* SDRAM Controller registers */
pub const AT91_MC_SDRAMC_MR: u32 = 0x90;
pub const AT91_MC_SDRAMC_MODE: _ = GENMASK(3, 0);
pub const AT91_MC_SDRAMC_MODE_NORMAL: _ = 0 << 0;
pub const AT91_MC_SDRAMC_MODE_NOP: _ = 1 << 0;
pub const AT91_MC_SDRAMC_MODE_PRECHARGE: _ = 2 << 0;
pub const AT91_MC_SDRAMC_MODE_LMR: _ = 3 << 0;
pub const AT91_MC_SDRAMC_MODE_REFRESH: _ = 4 << 0;
pub const AT91_MC_SDRAMC_DBW_16: _ = BIT(4);
pub const AT91_MC_SDRAMC_TR: u32 = 0x94;
pub const AT91_MC_SDRAMC_COUNT: _ = GENMASK(11, 0);
pub const AT91_MC_SDRAMC_CR: u32 = 0x98;
pub const AT91_MC_SDRAMC_NC: _ = GENMASK(1, 0);
pub const AT91_MC_SDRAMC_NC_8: _ = 0 << 0;
pub const AT91_MC_SDRAMC_NC_9: _ = 1 << 0;
pub const AT91_MC_SDRAMC_NC_10: _ = 2 << 0;
pub const AT91_MC_SDRAMC_NC_11: _ = 3 << 0;
pub const AT91_MC_SDRAMC_NR: _ = GENMASK(3, 2);
pub const AT91_MC_SDRAMC_NR_11: _ = 0 << 2;
pub const AT91_MC_SDRAMC_NR_12: _ = 1 << 2;
pub const AT91_MC_SDRAMC_NR_13: _ = 2 << 2;
pub const AT91_MC_SDRAMC_NB: _ = BIT(4);
pub const AT91_MC_SDRAMC_NB_2: _ = 0 << 4;
pub const AT91_MC_SDRAMC_NB_4: _ = 1 << 4;
pub const AT91_MC_SDRAMC_CAS: _ = GENMASK(6, 5);
pub const AT91_MC_SDRAMC_CAS_2: _ = 2 << 5;
pub const AT91_MC_SDRAMC_TWR: _ = GENMASK(10, 7);
pub const AT91_MC_SDRAMC_TRC: _ = GENMASK(14, 11);
pub const AT91_MC_SDRAMC_TRP: _ = GENMASK(18, 15);
pub const AT91_MC_SDRAMC_TRCD: _ = GENMASK(22, 19);
pub const AT91_MC_SDRAMC_TRAS: _ = GENMASK(26, 23);
pub const AT91_MC_SDRAMC_TXSR: _ = GENMASK(30, 27);
pub const AT91_MC_SDRAMC_SRR: u32 = 0x9c;
pub const AT91_MC_SDRAMC_SRCB: _ = BIT(0);
pub const AT91_MC_SDRAMC_LPR: u32 = 0xa0;
pub const AT91_MC_SDRAMC_LPCB: _ = BIT(0);
pub const AT91_MC_SDRAMC_IER: u32 = 0xa4;
pub const AT91_MC_SDRAMC_IDR: u32 = 0xa8;
pub const AT91_MC_SDRAMC_IMR: u32 = 0xac;
pub const AT91_MC_SDRAMC_ISR: u32 = 0xb0;
pub const AT91_MC_SDRAMC_RES: _ = BIT(0);

/* Burst Flash Controller register */
pub const AT91_MC_BFC_MR: u32 = 0xc0;
pub const AT91_MC_BFC_BFCOM: _ = GENMASK(1, 0);
pub const AT91_MC_BFC_BFCOM_DISABLED: _ = 0 << 0;
pub const AT91_MC_BFC_BFCOM_ASYNC: _ = 1 << 0;
pub const AT91_MC_BFC_BFCOM_BURST: _ = 2 << 0;
pub const AT91_MC_BFC_BFCC: _ = GENMASK(3, 2);
pub const AT91_MC_BFC_BFCC_MCK: _ = 1 << 2;
pub const AT91_MC_BFC_BFCC_DIV2: _ = 2 << 2;
pub const AT91_MC_BFC_BFCC_DIV4: _ = 3 << 2;
pub const AT91_MC_BFC_AVL: _ = GENMASK(7, 4);
pub const AT91_MC_BFC_PAGES: _ = GENMASK(10, 8);
pub const AT91_MC_BFC_PAGES_NO_PAGE: _ = 0 << 8;
pub const AT91_MC_BFC_PAGES_16: _ = 1 << 8;
pub const AT91_MC_BFC_PAGES_32: _ = 2 << 8;
pub const AT91_MC_BFC_PAGES_64: _ = 3 << 8;
pub const AT91_MC_BFC_PAGES_128: _ = 4 << 8;
pub const AT91_MC_BFC_PAGES_256: _ = 5 << 8;
pub const AT91_MC_BFC_PAGES_512: _ = 6 << 8;
pub const AT91_MC_BFC_PAGES_1024: _ = 7 << 8;
pub const AT91_MC_BFC_OEL: _ = GENMASK(13, 12);
pub const AT91_MC_BFC_BAAEN: _ = BIT(16);
pub const AT91_MC_BFC_BFOEH: _ = BIT(17);
pub const AT91_MC_BFC_MUXEN: _ = BIT(18);
pub const AT91_MC_BFC_RDYEN: _ = BIT(19);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
