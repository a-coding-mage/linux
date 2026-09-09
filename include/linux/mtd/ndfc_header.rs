/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Copyright (c) 2006 Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
 *
 *  Info:
 *   Contains defines, datastructures for ndfc nand controller
 */

/* NDFC Register definitions */
pub const NDFC_CMD: u32 = 0x00;
pub const NDFC_ALE: u32 = 0x04;
pub const NDFC_DATA: u32 = 0x08;
pub const NDFC_ECC: u32 = 0x10;
pub const NDFC_BCFG0: u32 = 0x30;
pub const NDFC_BCFG1: u32 = 0x34;
pub const NDFC_BCFG2: u32 = 0x38;
pub const NDFC_BCFG3: u32 = 0x3c;
pub const NDFC_CCR: u32 = 0x40;
pub const NDFC_STAT: u32 = 0x44;
pub const NDFC_HWCTL: u32 = 0x48;
pub const NDFC_REVID: u32 = 0x50;

pub const NDFC_STAT_IS_READY: u32 = 0x01000000;

pub const NDFC_CCR_RESET_CE: u32 = 0x80000000; /* CE Reset */
pub const NDFC_CCR_RESET_ECC: u32 = 0x40000000; /* ECC Reset */
pub const NDFC_CCR_RIE: u32 = 0x20000000; /* Interrupt Enable on Device Rdy */
pub const NDFC_CCR_REN: u32 = 0x10000000; /* Enable wait for Rdy in LinearR */
pub const NDFC_CCR_ROMEN: u32 = 0x08000000; /* Enable ROM In LinearR */
pub const NDFC_CCR_ARE: u32 = 0x04000000; /* Auto-Read Enable */

#[inline]
pub const fn NDFC_CCR_BS(x: u32) -> u32 {
    (x & 0x3) << 24
} /* Select Bank on CE[x] */

pub const NDFC_CCR_BS_MASK: u32 = 0x03000000; /* Select Bank */
pub const NDFC_CCR_ARAC0: u32 = 0x00000000; /* 3 Addr, 1 Col 2 Row 512b page */
pub const NDFC_CCR_ARAC1: u32 = 0x00001000; /* 4 Addr, 1 Col 3 Row 512b page */
pub const NDFC_CCR_ARAC2: u32 = 0x00002000; /* 4 Addr, 2 Col 2 Row 2K page */
pub const NDFC_CCR_ARAC3: u32 = 0x00003000; /* 5 Addr, 2 Col 3 Row 2K page */
pub const NDFC_CCR_ARAC_MASK: u32 = 0x00003000; /* Auto-Read mode Addr Cycles */
pub const NDFC_CCR_RPG: u32 = 0x0000C000; /* Auto-Read Page */
pub const NDFC_CCR_EBCC: u32 = 0x00000004; /* EBC Configuration Completed */
pub const NDFC_CCR_DHC: u32 = 0x00000002; /* Direct Hardware Control Enable */

pub const NDFC_BxCFG_EN: u32 = 0x80000000; /* Bank Enable */
pub const NDFC_BxCFG_CED: u32 = 0x40000000; /* nCE Style */
pub const NDFC_BxCFG_SZ_MASK: u32 = 0x08000000; /* Bank Size */
pub const NDFC_BxCFG_SZ_8BIT: u32 = 0x00000000; /* 8bit */
pub const NDFC_BxCFG_SZ_16BIT: u32 = 0x08000000; /* 16bit */

pub const NDFC_MAX_BANKS: u32 = 4;

#[repr(C)]
pub struct ndfc_controller_settings {
    pub ccr_settings: u32,
    pub ndfc_erpn: u64,
}

#[repr(C)]
pub struct ndfc_chip_settings {
    pub bank_settings: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
