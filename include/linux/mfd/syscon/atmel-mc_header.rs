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
pub const AT91_MC_RCB: kernel::ffi::c_ulong = BIT(0);

pub const AT91_MC_ASR: u32 = 0x04;
pub const AT91_MC_UNADD: kernel::ffi::c_ulong = BIT(0);
pub const AT91_MC_MISADD: kernel::ffi::c_ulong = BIT(1);
pub const AT91_MC_ABTSZ: kernel::ffi::c_ulong = GENMASK(9, 8);
pub const AT91_MC_ABTSZ_BYTE: u32 = 0 << 8;
pub const AT91_MC_ABTSZ_HALFWORD: u32 = 1 << 8;
pub const AT91_MC_ABTSZ_WORD: u32 = 2 << 8;
pub const AT91_MC_ABTTYP: kernel::ffi::c_ulong = GENMASK(11, 10);
pub const AT91_MC_ABTTYP_DATAREAD: u32 = 0 << 10;
pub const AT91_MC_ABTTYP_DATAWRITE: u32 = 1 << 10;
pub const AT91_MC_ABTTYP_FETCH: u32 = 2 << 10;
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
pub const AT91_MC_EBI_DBPUC: kernel::ffi::c_ulong = BIT(0);

/* Static Memory Controller (SMC) registers */
macro_rules! AT91_MC_SMC_CSR { ($n:expr) => { 0x70 + (($n) * 4) }; }
pub const AT91_MC_SMC_NWS: kernel::ffi::c_ulong = GENMASK(6, 0);
macro_rules! AT91_MC_SMC_NWS_ { ($x:expr) => { ($x) << 0 }; }
pub const AT91_MC_SMC_WSEN: kernel::ffi::c_ulong = BIT(7);
pub const AT91_MC_SMC_TDF: kernel::ffi::c_ulong = GENMASK(11, 8);
macro_rules! AT91_MC_SMC_TDF_ { ($x:expr) => { ($x) << 8 }; }
pub const AT91_MC_SMC_TDF_MAX: u32 = 0xf;
pub const AT91_MC_SMC_BAT: kernel::ffi::c_ulong = BIT(12);
pub const AT91_MC_SMC_DBW: kernel::ffi::c_ulong = GENMASK(14, 13);
pub const AT91_MC_SMC_DBW_16: u32 = 1 << 13;
pub const AT91_MC_SMC_DBW_8: u32 = 2 << 13;
pub const AT91_MC_SMC_DPR: kernel::ffi::c_ulong = BIT(15);
pub const AT91_MC_SMC_ACSS: kernel::ffi::c_ulong = GENMASK(17, 16);
macro_rules! AT91_MC_SMC_ACSS_ { ($x:expr) => { ($x) << 16 }; }
pub const AT91_MC_SMC_ACSS_MAX: u32 = 3;
pub const AT91_MC_SMC_RWSETUP: kernel::ffi::c_ulong = GENMASK(26, 24);
macro_rules! AT91_MC_SMC_RWSETUP_ { ($x:expr) => { ($x) << 24 }; }
pub const AT91_MC_SMC_RWHOLD: kernel::ffi::c_ulong = GENMASK(30, 28);
macro_rules! AT91_MC_SMC_RWHOLD_ { ($x:expr) => { ($x) << 28 }; }
pub const AT91_MC_SMC_RWHOLDSETUP_MAX: u32 = 7;

/* SDRAM Controller registers */
pub const AT91_MC_SDRAMC_MR: u32 = 0x90;
pub const AT91_MC_SDRAMC_MODE: kernel::ffi::c_ulong = GENMASK(3, 0);
pub const AT91_MC_SDRAMC_MODE_NORMAL: u32 = 0 << 0;
pub const AT91_MC_SDRAMC_MODE_NOP: u32 = 1 << 0;
pub const AT91_MC_SDRAMC_MODE_PRECHARGE: u32 = 2 << 0;
pub const AT91_MC_SDRAMC_MODE_LMR: u32 = 3 << 0;
pub const AT91_MC_SDRAMC_MODE_REFRESH: u32 = 4 << 0;
pub const AT91_MC_SDRAMC_DBW_16: kernel::ffi::c_ulong = BIT(4);
pub const AT91_MC_SDRAMC_TR: u32 = 0x94;
pub const AT91_MC_SDRAMC_COUNT: kernel::ffi::c_ulong = GENMASK(11, 0);
pub const AT91_MC_SDRAMC_CR: u32 = 0x98;
pub const AT91_MC_SDRAMC_NC: kernel::ffi::c_ulong = GENMASK(1, 0);
pub const AT91_MC_SDRAMC_NC_8: u32 = 0 << 0;
pub const AT91_MC_SDRAMC_NC_9: u32 = 1 << 0;
pub const AT91_MC_SDRAMC_NC_10: u32 = 2 << 0;
pub const AT91_MC_SDRAMC_NC_11: u32 = 3 << 0;
pub const AT91_MC_SDRAMC_NR: kernel::ffi::c_ulong = GENMASK(3, 2);
pub const AT91_MC_SDRAMC_NR_11: u32 = 0 << 2;
pub const AT91_MC_SDRAMC_NR_12: u32 = 1 << 2;
pub const AT91_MC_SDRAMC_NR_13: u32 = 2 << 2;
pub const AT91_MC_SDRAMC_NB: kernel::ffi::c_ulong = BIT(4);
pub const AT91_MC_SDRAMC_NB_2: u32 = 0 << 4;
pub const AT91_MC_SDRAMC_NB_4: u32 = 1 << 4;
pub const AT91_MC_SDRAMC_CAS: kernel::ffi::c_ulong = GENMASK(6, 5);
pub const AT91_MC_SDRAMC_CAS_2: u32 = 2 << 5;
pub const AT91_MC_SDRAMC_TWR: kernel::ffi::c_ulong = GENMASK(10, 7);
pub const AT91_MC_SDRAMC_TRC: kernel::ffi::c_ulong = GENMASK(14, 11);
pub const AT91_MC_SDRAMC_TRP: kernel::ffi::c_ulong = GENMASK(18, 15);
pub const AT91_MC_SDRAMC_TRCD: kernel::ffi::c_ulong = GENMASK(22, 19);
pub const AT91_MC_SDRAMC_TRAS: kernel::ffi::c_ulong = GENMASK(26, 23);
pub const AT91_MC_SDRAMC_TXSR: kernel::ffi::c_ulong = GENMASK(30, 27);
pub const AT91_MC_SDRAMC_SRR: u32 = 0x9c;
pub const AT91_MC_SDRAMC_SRCB: kernel::ffi::c_ulong = BIT(0);
pub const AT91_MC_SDRAMC_LPR: u32 = 0xa0;
pub const AT91_MC_SDRAMC_LPCB: kernel::ffi::c_ulong = BIT(0);
pub const AT91_MC_SDRAMC_IER: u32 = 0xa4;
pub const AT91_MC_SDRAMC_IDR: u32 = 0xa8;
pub const AT91_MC_SDRAMC_IMR: u32 = 0xac;
pub const AT91_MC_SDRAMC_ISR: u32 = 0xb0;
pub const AT91_MC_SDRAMC_RES: kernel::ffi::c_ulong = BIT(0);

/* Burst Flash Controller register */
pub const AT91_MC_BFC_MR: u32 = 0xc0;
pub const AT91_MC_BFC_BFCOM: kernel::ffi::c_ulong = GENMASK(1, 0);
pub const AT91_MC_BFC_BFCOM_DISABLED: u32 = 0 << 0;
pub const AT91_MC_BFC_BFCOM_ASYNC: u32 = 1 << 0;
pub const AT91_MC_BFC_BFCOM_BURST: u32 = 2 << 0;
pub const AT91_MC_BFC_BFCC: kernel::ffi::c_ulong = GENMASK(3, 2);
pub const AT91_MC_BFC_BFCC_MCK: u32 = 1 << 2;
pub const AT91_MC_BFC_BFCC_DIV2: u32 = 2 << 2;
pub const AT91_MC_BFC_BFCC_DIV4: u32 = 3 << 2;
pub const AT91_MC_BFC_AVL: kernel::ffi::c_ulong = GENMASK(7, 4);
pub const AT91_MC_BFC_PAGES: kernel::ffi::c_ulong = GENMASK(10, 8);
pub const AT91_MC_BFC_PAGES_NO_PAGE: u32 = 0 << 8;
pub const AT91_MC_BFC_PAGES_16: u32 = 1 << 8;
pub const AT91_MC_BFC_PAGES_32: u32 = 2 << 8;
pub const AT91_MC_BFC_PAGES_64: u32 = 3 << 8;
pub const AT91_MC_BFC_PAGES_128: u32 = 4 << 8;
pub const AT91_MC_BFC_PAGES_256: u32 = 5 << 8;
pub const AT91_MC_BFC_PAGES_512: u32 = 6 << 8;
pub const AT91_MC_BFC_PAGES_1024: u32 = 7 << 8;
pub const AT91_MC_BFC_OEL: kernel::ffi::c_ulong = GENMASK(13, 12);
pub const AT91_MC_BFC_BAAEN: kernel::ffi::c_ulong = BIT(16);
pub const AT91_MC_BFC_BFOEH: kernel::ffi::c_ulong = BIT(17);
pub const AT91_MC_BFC_MUXEN: kernel::ffi::c_ulong = BIT(18);
pub const AT91_MC_BFC_RDYEN: kernel::ffi::c_ulong = BIT(19);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
