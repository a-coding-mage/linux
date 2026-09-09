/*
 * PCI Register definitions for the MIPS System Controller.
 *
 * Copyright (C) 2002, 2005  MIPS Technologies, Inc.  All rights reserved.
 * Authors: Carsten Langgaard <carstenl@mips.com>, Maciej W. Rozycki <macro@mips.com>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

/* Register offset addresses. */
pub const MSC01_PCI_ID_OFS: u32 = 0x0000;
pub const MSC01_PCI_SC2PMBASL_OFS: u32 = 0x0208;
pub const MSC01_PCI_SC2PMMSKL_OFS: u32 = 0x0218;
pub const MSC01_PCI_SC2PMMAPL_OFS: u32 = 0x0228;
pub const MSC01_PCI_SC2PIOBASL_OFS: u32 = 0x0248;
pub const MSC01_PCI_SC2PIOMSKL_OFS: u32 = 0x0258;
pub const MSC01_PCI_SC2PIOMAPL_OFS: u32 = 0x0268;
pub const MSC01_PCI_P2SCMSKL_OFS: u32 = 0x0308;
pub const MSC01_PCI_P2SCMAPL_OFS: u32 = 0x0318;
pub const MSC01_PCI_INTCFG_OFS: u32 = 0x0600;
pub const MSC01_PCI_INTSTAT_OFS: u32 = 0x0608;
pub const MSC01_PCI_CFGADDR_OFS: u32 = 0x0610;
pub const MSC01_PCI_CFGDATA_OFS: u32 = 0x0618;
pub const MSC01_PCI_IACK_OFS: u32 = 0x0620;
pub const MSC01_PCI_HEAD0_OFS: u32 = 0x2000; /* DevID, VendorID */
pub const MSC01_PCI_HEAD1_OFS: u32 = 0x2008; /* Status, Command */
pub const MSC01_PCI_HEAD2_OFS: u32 = 0x2010; /* Class code, RevID */
pub const MSC01_PCI_HEAD3_OFS: u32 = 0x2018; /* bist, header, latency */
pub const MSC01_PCI_HEAD4_OFS: u32 = 0x2020; /* BAR 0 */
pub const MSC01_PCI_HEAD5_OFS: u32 = 0x2028; /* BAR 1 */
pub const MSC01_PCI_HEAD6_OFS: u32 = 0x2030; /* BAR 2 */
pub const MSC01_PCI_HEAD7_OFS: u32 = 0x2038; /* BAR 3 */
pub const MSC01_PCI_HEAD8_OFS: u32 = 0x2040; /* BAR 4 */
pub const MSC01_PCI_HEAD9_OFS: u32 = 0x2048; /* BAR 5 */
pub const MSC01_PCI_HEAD10_OFS: u32 = 0x2050; /* CardBus CIS Ptr */
pub const MSC01_PCI_HEAD11_OFS: u32 = 0x2058; /* SubSystem ID, -VendorID */
pub const MSC01_PCI_HEAD12_OFS: u32 = 0x2060; /* ROM BAR */
pub const MSC01_PCI_HEAD13_OFS: u32 = 0x2068; /* Capabilities ptr */
pub const MSC01_PCI_HEAD14_OFS: u32 = 0x2070; /* reserved */
pub const MSC01_PCI_HEAD15_OFS: u32 = 0x2078; /* Maxl, ming, intpin, int */
pub const MSC01_PCI_BAR0_OFS: u32 = 0x2220;
pub const MSC01_PCI_CFG_OFS: u32 = 0x2380;
pub const MSC01_PCI_SWAP_OFS: u32 = 0x2388;

/* Register encodings. */
pub const MSC01_PCI_ID_ID_SHF: u32 = 16;
pub const MSC01_PCI_ID_ID_MSK: u32 = 0x00ff0000;
pub const MSC01_PCI_ID_ID_HOSTBRIDGE: u32 = 82;
pub const MSC01_PCI_ID_MAR_SHF: u32 = 8;
pub const MSC01_PCI_ID_MAR_MSK: u32 = 0x0000ff00;
pub const MSC01_PCI_ID_MIR_SHF: u32 = 0;
pub const MSC01_PCI_ID_MIR_MSK: u32 = 0x000000ff;

pub const MSC01_PCI_SC2PMBASL_BAS_SHF: u32 = 24;
pub const MSC01_PCI_SC2PMBASL_BAS_MSK: u32 = 0xff000000;
pub const MSC01_PCI_SC2PMMSKL_MSK_SHF: u32 = 24;
pub const MSC01_PCI_SC2PMMSKL_MSK_MSK: u32 = 0xff000000;
pub const MSC01_PCI_SC2PMMAPL_MAP_SHF: u32 = 24;
pub const MSC01_PCI_SC2PMMAPL_MAP_MSK: u32 = 0xff000000;
pub const MSC01_PCI_SC2PIOBASL_BAS_SHF: u32 = 24;
pub const MSC01_PCI_SC2PIOBASL_BAS_MSK: u32 = 0xff000000;
pub const MSC01_PCI_SC2PIOMSKL_MSK_SHF: u32 = 24;
pub const MSC01_PCI_SC2PIOMSKL_MSK_MSK: u32 = 0xff000000;
pub const MSC01_PCI_SC2PIOMAPL_MAP_SHF: u32 = 24;
pub const MSC01_PCI_SC2PIOMAPL_MAP_MSK: u32 = 0xff000000;
pub const MSC01_PCI_P2SCMSKL_MSK_SHF: u32 = 24;
pub const MSC01_PCI_P2SCMSKL_MSK_MSK: u32 = 0xff000000;
pub const MSC01_PCI_P2SCMAPL_MAP_SHF: u32 = 24;
pub const MSC01_PCI_P2SCMAPL_MAP_MSK: u32 = 0xff000000;

pub const MSC01_PCI_INTCFG_RST_SHF: u32 = 10;
pub const MSC01_PCI_INTCFG_RST_MSK: u32 = 0x00000400;
pub const MSC01_PCI_INTCFG_RST_BIT: u32 = 0x00000400;
pub const MSC01_PCI_INTCFG_MWE_SHF: u32 = 9;
pub const MSC01_PCI_INTCFG_MWE_MSK: u32 = 0x00000200;
pub const MSC01_PCI_INTCFG_MWE_BIT: u32 = 0x00000200;
pub const MSC01_PCI_INTCFG_DTO_SHF: u32 = 8;
pub const MSC01_PCI_INTCFG_DTO_MSK: u32 = 0x00000100;
pub const MSC01_PCI_INTCFG_DTO_BIT: u32 = 0x00000100;
pub const MSC01_PCI_INTCFG_MA_SHF: u32 = 7;
pub const MSC01_PCI_INTCFG_MA_MSK: u32 = 0x00000080;
pub const MSC01_PCI_INTCFG_MA_BIT: u32 = 0x00000080;
pub const MSC01_PCI_INTCFG_TA_SHF: u32 = 6;
pub const MSC01_PCI_INTCFG_TA_MSK: u32 = 0x00000040;
pub const MSC01_PCI_INTCFG_TA_BIT: u32 = 0x00000040;
pub const MSC01_PCI_INTCFG_RTY_SHF: u32 = 5;
pub const MSC01_PCI_INTCFG_RTY_MSK: u32 = 0x00000020;
pub const MSC01_PCI_INTCFG_RTY_BIT: u32 = 0x00000020;
pub const MSC01_PCI_INTCFG_MWP_SHF: u32 = 4;
pub const MSC01_PCI_INTCFG_MWP_MSK: u32 = 0x00000010;
pub const MSC01_PCI_INTCFG_MWP_BIT: u32 = 0x00000010;
pub const MSC01_PCI_INTCFG_MRP_SHF: u32 = 3;
pub const MSC01_PCI_INTCFG_MRP_MSK: u32 = 0x00000008;
pub const MSC01_PCI_INTCFG_MRP_BIT: u32 = 0x00000008;
pub const MSC01_PCI_INTCFG_SWP_SHF: u32 = 2;
pub const MSC01_PCI_INTCFG_SWP_MSK: u32 = 0x00000004;
pub const MSC01_PCI_INTCFG_SWP_BIT: u32 = 0x00000004;
pub const MSC01_PCI_INTCFG_SRP_SHF: u32 = 1;
pub const MSC01_PCI_INTCFG_SRP_MSK: u32 = 0x00000002;
pub const MSC01_PCI_INTCFG_SRP_BIT: u32 = 0x00000002;
pub const MSC01_PCI_INTCFG_SE_SHF: u32 = 0;
pub const MSC01_PCI_INTCFG_SE_MSK: u32 = 0x00000001;
pub const MSC01_PCI_INTCFG_SE_BIT: u32 = 0x00000001;

pub const MSC01_PCI_INTSTAT_RST_SHF: u32 = 10;
pub const MSC01_PCI_INTSTAT_RST_MSK: u32 = 0x00000400;
pub const MSC01_PCI_INTSTAT_RST_BIT: u32 = 0x00000400;
pub const MSC01_PCI_INTSTAT_MWE_SHF: u32 = 9;
pub const MSC01_PCI_INTSTAT_MWE_MSK: u32 = 0x00000200;
pub const MSC01_PCI_INTSTAT_MWE_BIT: u32 = 0x00000200;
pub const MSC01_PCI_INTSTAT_DTO_SHF: u32 = 8;
pub const MSC01_PCI_INTSTAT_DTO_MSK: u32 = 0x00000100;
pub const MSC01_PCI_INTSTAT_DTO_BIT: u32 = 0x00000100;
pub const MSC01_PCI_INTSTAT_MA_SHF: u32 = 7;
pub const MSC01_PCI_INTSTAT_MA_MSK: u32 = 0x00000080;
pub const MSC01_PCI_INTSTAT_MA_BIT: u32 = 0x00000080;
pub const MSC01_PCI_INTSTAT_TA_SHF: u32 = 6;
pub const MSC01_PCI_INTSTAT_TA_MSK: u32 = 0x00000040;
pub const MSC01_PCI_INTSTAT_TA_BIT: u32 = 0x00000040;
pub const MSC01_PCI_INTSTAT_RTY_SHF: u32 = 5;
pub const MSC01_PCI_INTSTAT_RTY_MSK: u32 = 0x00000020;
pub const MSC01_PCI_INTSTAT_RTY_BIT: u32 = 0x00000020;
pub const MSC01_PCI_INTSTAT_MWP_SHF: u32 = 4;
pub const MSC01_PCI_INTSTAT_MWP_MSK: u32 = 0x00000010;
pub const MSC01_PCI_INTSTAT_MWP_BIT: u32 = 0x00000010;
pub const MSC01_PCI_INTSTAT_MRP_SHF: u32 = 3;
pub const MSC01_PCI_INTSTAT_MRP_MSK: u32 = 0x00000008;
pub const MSC01_PCI_INTSTAT_MRP_BIT: u32 = 0x00000008;
pub const MSC01_PCI_INTSTAT_SWP_SHF: u32 = 2;
pub const MSC01_PCI_INTSTAT_SWP_MSK: u32 = 0x00000004;
pub const MSC01_PCI_INTSTAT_SWP_BIT: u32 = 0x00000004;
pub const MSC01_PCI_INTSTAT_SRP_SHF: u32 = 1;
pub const MSC01_PCI_INTSTAT_SRP_MSK: u32 = 0x00000002;
pub const MSC01_PCI_INTSTAT_SRP_BIT: u32 = 0x00000002;
pub const MSC01_PCI_INTSTAT_SE_SHF: u32 = 0;
pub const MSC01_PCI_INTSTAT_SE_MSK: u32 = 0x00000001;
pub const MSC01_PCI_INTSTAT_SE_BIT: u32 = 0x00000001;

pub const MSC01_PCI_CFGADDR_BNUM_SHF: u32 = 16;
pub const MSC01_PCI_CFGADDR_BNUM_MSK: u32 = 0x00ff0000;
pub const MSC01_PCI_CFGADDR_DNUM_SHF: u32 = 11;
pub const MSC01_PCI_CFGADDR_DNUM_MSK: u32 = 0x0000f800;
pub const MSC01_PCI_CFGADDR_FNUM_SHF: u32 = 8;
pub const MSC01_PCI_CFGADDR_FNUM_MSK: u32 = 0x00000700;
pub const MSC01_PCI_CFGADDR_RNUM_SHF: u32 = 2;
pub const MSC01_PCI_CFGADDR_RNUM_MSK: u32 = 0x000000fc;
pub const MSC01_PCI_CFGDATA_DATA_SHF: u32 = 0;
pub const MSC01_PCI_CFGDATA_DATA_MSK: u32 = 0xffffffff;

/* The defines below are ONLY valid for a MEM bar! */
pub const MSC01_PCI_BAR0_SIZE_SHF: u32 = 4;
pub const MSC01_PCI_BAR0_SIZE_MSK: u32 = 0xfffffff0;
pub const MSC01_PCI_BAR0_P_SHF: u32 = 3;
pub const MSC01_PCI_BAR0_P_MSK: u32 = 0x00000008;
pub const MSC01_PCI_BAR0_P_BIT: u32 = MSC01_PCI_BAR0_P_MSK;
pub const MSC01_PCI_BAR0_D_SHF: u32 = 1;
pub const MSC01_PCI_BAR0_D_MSK: u32 = 0x00000006;
pub const MSC01_PCI_BAR0_T_SHF: u32 = 0;
pub const MSC01_PCI_BAR0_T_MSK: u32 = 0x00000001;
pub const MSC01_PCI_BAR0_T_BIT: u32 = MSC01_PCI_BAR0_T_MSK;

pub const MSC01_PCI_CFG_RA_SHF: u32 = 17;
pub const MSC01_PCI_CFG_RA_MSK: u32 = 0x00020000;
pub const MSC01_PCI_CFG_RA_BIT: u32 = MSC01_PCI_CFG_RA_MSK;
pub const MSC01_PCI_CFG_G_SHF: u32 = 16;
pub const MSC01_PCI_CFG_G_MSK: u32 = 0x00010000;
pub const MSC01_PCI_CFG_G_BIT: u32 = MSC01_PCI_CFG_G_MSK;
pub const MSC01_PCI_CFG_EN_SHF: u32 = 15;
pub const MSC01_PCI_CFG_EN_MSK: u32 = 0x00008000;
pub const MSC01_PCI_CFG_EN_BIT: u32 = MSC01_PCI_CFG_EN_MSK;
pub const MSC01_PCI_CFG_MAXRTRY_SHF: u32 = 0;
pub const MSC01_PCI_CFG_MAXRTRY_MSK: u32 = 0x00000fff;
pub const MSC01_PCI_SWAP_IO_SHF: u32 = 18;
pub const MSC01_PCI_SWAP_IO_MSK: u32 = 0x000c0000;
pub const MSC01_PCI_SWAP_MEM_SHF: u32 = 16;
pub const MSC01_PCI_SWAP_MEM_MSK: u32 = 0x00030000;
pub const MSC01_PCI_SWAP_BAR0_SHF: u32 = 0;
pub const MSC01_PCI_SWAP_BAR0_MSK: u32 = 0x00000003;
pub const MSC01_PCI_SWAP_NOSWAP: u32 = 0;
pub const MSC01_PCI_SWAP_BYTESWAP: u32 = 1;

/* MIPS System controller PCI register base. */
pub const MIPS_MSC01_PCI_REG_BASE: usize = 0x1bd00000;
pub const MIPS_SOCITSC_PCI_REG_BASE: usize = 0x1ff10000;

extern "C" {
    pub static mut _pcictrl_msc: usize;
}

/* MSC01_PCI_REG_BASE is the externally supplied _pcictrl_msc variable. */
#[macro_export]
macro_rules! MSC_WRITE {
    ($reg:expr, $data:expr) => {{ unsafe { core::ptr::write_volatile(($reg as *mut u32), $data as u32); } }};
}
#[macro_export]
macro_rules! MSC_READ {
    ($reg:expr, $data:expr) => {{ $data = unsafe { core::ptr::read_volatile($reg as *const u32) }; }};
}

/* Registers absolute addresses. */
pub const MSC01_PCI_ID: usize = unsafe { _pcictrl_msc } + MSC01_PCI_ID_OFS as usize;
pub const MSC01_PCI_SC2PMBASL: usize = unsafe { _pcictrl_msc } + MSC01_PCI_SC2PMBASL_OFS as usize;
pub const MSC01_PCI_SC2PMMSKL: usize = unsafe { _pcictrl_msc } + MSC01_PCI_SC2PMMSKL_OFS as usize;
pub const MSC01_PCI_SC2PMMAPL: usize = unsafe { _pcictrl_msc } + MSC01_PCI_SC2PMMAPL_OFS as usize;
pub const MSC01_PCI_SC2PIOBASL: usize = unsafe { _pcictrl_msc } + MSC01_PCI_SC2PIOBASL_OFS as usize;
pub const MSC01_PCI_SC2PIOMSKL: usize = unsafe { _pcictrl_msc } + MSC01_PCI_SC2PIOMSKL_OFS as usize;
pub const MSC01_PCI_SC2PIOMAPL: usize = unsafe { _pcictrl_msc } + MSC01_PCI_SC2PIOMAPL_OFS as usize;
pub const MSC01_PCI_P2SCMSKL: usize = unsafe { _pcictrl_msc } + MSC01_PCI_P2SCMSKL_OFS as usize;
pub const MSC01_PCI_P2SCMAPL: usize = unsafe { _pcictrl_msc } + MSC01_PCI_P2SCMAPL_OFS as usize;
pub const MSC01_PCI_INTCFG: usize = unsafe { _pcictrl_msc } + MSC01_PCI_INTCFG_OFS as usize;
pub const MSC01_PCI_INTSTAT: usize = unsafe { _pcictrl_msc } + MSC01_PCI_INTSTAT_OFS as usize;
pub const MSC01_PCI_CFGADDR: usize = unsafe { _pcictrl_msc } + MSC01_PCI_CFGADDR_OFS as usize;
pub const MSC01_PCI_CFGDATA: usize = unsafe { _pcictrl_msc } + MSC01_PCI_CFGDATA_OFS as usize;
pub const MSC01_PCI_IACK: usize = unsafe { _pcictrl_msc } + MSC01_PCI_IACK_OFS as usize;
pub const MSC01_PCI_HEAD0: usize = unsafe { _pcictrl_msc } + MSC01_PCI_HEAD0_OFS as usize;
pub const MSC01_PCI_HEAD1: usize = unsafe { _pcictrl_msc } + MSC01_PCI_HEAD1_OFS as usize;
pub const MSC01_PCI_HEAD2: usize = unsafe { _pcictrl_msc } + MSC01_PCI_HEAD2_OFS as usize;
pub const MSC01_PCI_HEAD3: usize = unsafe { _pcictrl_msc } + MSC01_PCI_HEAD3_OFS as usize;
pub const MSC01_PCI_HEAD4: usize = unsafe { _pcictrl_msc } + MSC01_PCI_HEAD4_OFS as usize;
pub const MSC01_PCI_HEAD5: usize = unsafe { _pcictrl_msc } + MSC01_PCI_HEAD5_OFS as usize;
pub const MSC01_PCI_HEAD6: usize = unsafe { _pcictrl_msc } + MSC01_PCI_HEAD6_OFS as usize;
pub const MSC01_PCI_HEAD7: usize = unsafe { _pcictrl_msc } + MSC01_PCI_HEAD7_OFS as usize;
pub const MSC01_PCI_HEAD8: usize = unsafe { _pcictrl_msc } + MSC01_PCI_HEAD8_OFS as usize;
pub const MSC01_PCI_HEAD9: usize = unsafe { _pcictrl_msc } + MSC01_PCI_HEAD9_OFS as usize;
pub const MSC01_PCI_HEAD10: usize = unsafe { _pcictrl_msc } + MSC01_PCI_HEAD10_OFS as usize;
pub const MSC01_PCI_HEAD11: usize = unsafe { _pcictrl_msc } + MSC01_PCI_HEAD11_OFS as usize;
pub const MSC01_PCI_HEAD12: usize = unsafe { _pcictrl_msc } + MSC01_PCI_HEAD11_OFS as usize;
pub const MSC01_PCI_HEAD13: usize = unsafe { _pcictrl_msc } + MSC01_PCI_HEAD11_OFS as usize;
pub const MSC01_PCI_HEAD14: usize = unsafe { _pcictrl_msc } + MSC01_PCI_HEAD11_OFS as usize;
pub const MSC01_PCI_HEAD15: usize = unsafe { _pcictrl_msc } + MSC01_PCI_HEAD11_OFS as usize;
pub const MSC01_PCI_BAR0: usize = unsafe { _pcictrl_msc } + MSC01_PCI_BAR0_OFS as usize;
pub const MSC01_PCI_CFG: usize = unsafe { _pcictrl_msc } + MSC01_PCI_CFG_OFS as usize;
pub const MSC01_PCI_SWAP: usize = unsafe { _pcictrl_msc } + MSC01_PCI_SWAP_OFS as usize;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
