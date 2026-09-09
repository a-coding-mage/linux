/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/mach-pxa/include/mach/pxa2xx-regs.h
 *
 *  Taken from pxa-regs.h by Russell King
 *
 *  Author: Nicolas Pitre
 *  Copyright: MontaVista Software Inc.
 */

// Dependency: pxa-regs.h supplies __REG, __REG2, and io_p2v.

/* Power Manager */

macro_rules! PMCR { () => { __REG!(0x40F00000) }; }
macro_rules! PSSR { () => { __REG!(0x40F00004) }; }
macro_rules! PSPR { () => { __REG!(0x40F00008) }; }
macro_rules! PWER { () => { __REG!(0x40F0000C) }; }
macro_rules! PRER { () => { __REG!(0x40F00010) }; }
macro_rules! PFER { () => { __REG!(0x40F00014) }; }
macro_rules! PEDR { () => { __REG!(0x40F00018) }; }
macro_rules! PCFR { () => { __REG!(0x40F0001C) }; }
macro_rules! PGSR0 { () => { __REG!(0x40F00020) }; }
macro_rules! PGSR1 { () => { __REG!(0x40F00024) }; }
macro_rules! PGSR2 { () => { __REG!(0x40F00028) }; }
macro_rules! PGSR3 { () => { __REG!(0x40F0002C) }; }
macro_rules! RCSR { () => { __REG!(0x40F00030) }; }
macro_rules! PSLR { () => { __REG!(0x40F00034) }; }
macro_rules! PSTR { () => { __REG!(0x40F00038) }; }
macro_rules! PSNR { () => { __REG!(0x40F0003C) }; }
macro_rules! PVCR { () => { __REG!(0x40F00040) }; }
macro_rules! PKWR { () => { __REG!(0x40F00050) }; }
macro_rules! PKSR { () => { __REG!(0x40F00054) }; }
macro_rules! PCMD { ($x:expr) => { __REG2!(0x40F00080, ($x) << 2) }; }

macro_rules! PCMD0 { () => { __REG!(0x40F00080 + 0 * 4) }; }
macro_rules! PCMD1 { () => { __REG!(0x40F00080 + 1 * 4) }; }
macro_rules! PCMD2 { () => { __REG!(0x40F00080 + 2 * 4) }; }
macro_rules! PCMD3 { () => { __REG!(0x40F00080 + 3 * 4) }; }
macro_rules! PCMD4 { () => { __REG!(0x40F00080 + 4 * 4) }; }
macro_rules! PCMD5 { () => { __REG!(0x40F00080 + 5 * 4) }; }
macro_rules! PCMD6 { () => { __REG!(0x40F00080 + 6 * 4) }; }
macro_rules! PCMD7 { () => { __REG!(0x40F00080 + 7 * 4) }; }
macro_rules! PCMD8 { () => { __REG!(0x40F00080 + 8 * 4) }; }
macro_rules! PCMD9 { () => { __REG!(0x40F00080 + 9 * 4) }; }
macro_rules! PCMD10 { () => { __REG!(0x40F00080 + 10 * 4) }; }
macro_rules! PCMD11 { () => { __REG!(0x40F00080 + 11 * 4) }; }
macro_rules! PCMD12 { () => { __REG!(0x40F00080 + 12 * 4) }; }
macro_rules! PCMD13 { () => { __REG!(0x40F00080 + 13 * 4) }; }
macro_rules! PCMD14 { () => { __REG!(0x40F00080 + 14 * 4) }; }
macro_rules! PCMD15 { () => { __REG!(0x40F00080 + 15 * 4) }; }
macro_rules! PCMD16 { () => { __REG!(0x40F00080 + 16 * 4) }; }
macro_rules! PCMD17 { () => { __REG!(0x40F00080 + 17 * 4) }; }
macro_rules! PCMD18 { () => { __REG!(0x40F00080 + 18 * 4) }; }
macro_rules! PCMD19 { () => { __REG!(0x40F00080 + 19 * 4) }; }
macro_rules! PCMD20 { () => { __REG!(0x40F00080 + 20 * 4) }; }
macro_rules! PCMD21 { () => { __REG!(0x40F00080 + 21 * 4) }; }
macro_rules! PCMD22 { () => { __REG!(0x40F00080 + 22 * 4) }; }
macro_rules! PCMD23 { () => { __REG!(0x40F00080 + 23 * 4) }; }
macro_rules! PCMD24 { () => { __REG!(0x40F00080 + 24 * 4) }; }
macro_rules! PCMD25 { () => { __REG!(0x40F00080 + 25 * 4) }; }
macro_rules! PCMD26 { () => { __REG!(0x40F00080 + 26 * 4) }; }
macro_rules! PCMD27 { () => { __REG!(0x40F00080 + 27 * 4) }; }
macro_rules! PCMD28 { () => { __REG!(0x40F00080 + 28 * 4) }; }
macro_rules! PCMD29 { () => { __REG!(0x40F00080 + 29 * 4) }; }
macro_rules! PCMD30 { () => { __REG!(0x40F00080 + 30 * 4) }; }
macro_rules! PCMD31 { () => { __REG!(0x40F00080 + 31 * 4) }; }

pub const PCMD_MBC: u32 = 1 << 12;
pub const PCMD_DCE: u32 = 1 << 11;
pub const PCMD_LC: u32 = 1 << 10;
/* FIXME: PCMD_SQC need be checked. */
pub const PCMD_SQC: u32 = 3 << 8; /* currently only bit 8 is changeable, bit 9 should be 0 all day. */
pub const PVCR_VCSA: u32 = 1 << 14;
pub const PVCR_CommandDelay: u32 = 0xf80;
pub const PCFR_PI2C_EN: u32 = 1 << 6;

pub const PSSR_OTGPH: u32 = 1 << 6;
pub const PSSR_RDH: u32 = 1 << 5;
pub const PSSR_PH: u32 = 1 << 4;
pub const PSSR_STS: u32 = 1 << 3;
pub const PSSR_VFS: u32 = 1 << 2;
pub const PSSR_BFS: u32 = 1 << 1;
pub const PSSR_SSS: u32 = 1 << 0;
pub const PSLR_SL_ROD: u32 = 1 << 20;
pub const PCFR_RO: u32 = 1 << 15;
pub const PCFR_PO: u32 = 1 << 14;
pub const PCFR_GPROD: u32 = 1 << 12;
pub const PCFR_L1_EN: u32 = 1 << 11;
pub const PCFR_FVC: u32 = 1 << 10;
pub const PCFR_DC_EN: u32 = 1 << 7;
pub const PCFR_PI2CEN: u32 = 1 << 6;
pub const PCFR_GPR_EN: u32 = 1 << 4;
pub const PCFR_DS: u32 = 1 << 3;
pub const PCFR_FS: u32 = 1 << 2;
pub const PCFR_FP: u32 = 1 << 1;
pub const PCFR_OPDE: u32 = 1 << 0;
pub const RCSR_GPR: u32 = 1 << 3;
pub const RCSR_SMR: u32 = 1 << 2;
pub const RCSR_WDR: u32 = 1 << 1;
pub const RCSR_HWR: u32 = 1 << 0;

macro_rules! PWER_GPIO { ($nb:expr) => { 1u32 << ($nb) }; }
macro_rules! PWER_GPIO0 { () => { PWER_GPIO!(0) }; }
macro_rules! PWER_GPIO1 { () => { PWER_GPIO!(1) }; }
macro_rules! PWER_GPIO2 { () => { PWER_GPIO!(2) }; }
macro_rules! PWER_GPIO3 { () => { PWER_GPIO!(3) }; }
macro_rules! PWER_GPIO4 { () => { PWER_GPIO!(4) }; }
macro_rules! PWER_GPIO5 { () => { PWER_GPIO!(5) }; }
macro_rules! PWER_GPIO6 { () => { PWER_GPIO!(6) }; }
macro_rules! PWER_GPIO7 { () => { PWER_GPIO!(7) }; }
macro_rules! PWER_GPIO8 { () => { PWER_GPIO!(8) }; }
macro_rules! PWER_GPIO9 { () => { PWER_GPIO!(9) }; }
macro_rules! PWER_GPIO10 { () => { PWER_GPIO!(10) }; }
macro_rules! PWER_GPIO11 { () => { PWER_GPIO!(11) }; }
macro_rules! PWER_GPIO12 { () => { PWER_GPIO!(12) }; }
macro_rules! PWER_GPIO13 { () => { PWER_GPIO!(13) }; }
macro_rules! PWER_GPIO14 { () => { PWER_GPIO!(14) }; }
macro_rules! PWER_GPIO15 { () => { PWER_GPIO!(15) }; }
pub const PWER_RTC: u32 = 0x80000000;

/* PXA2xx specific Core clock definitions */
macro_rules! CCCR { () => { io_p2v!(0x41300000) }; }
macro_rules! CCSR { () => { io_p2v!(0x4130000C) }; }
macro_rules! CKEN { () => { io_p2v!(0x41300004) }; }
macro_rules! OSCC { () => { io_p2v!(0x41300008) }; }

pub const OSCC_OON: u32 = 1 << 1;
pub const OSCC_OOK: u32 = 1 << 0;

/* PWRMODE register M field values */
pub const PWRMODE_IDLE: u32 = 0x1;
pub const PWRMODE_STANDBY: u32 = 0x2;
pub const PWRMODE_SLEEP: u32 = 0x3;
pub const PWRMODE_DEEPSLEEP: u32 = 0x7;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
