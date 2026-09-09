/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Freescale MPC85xx Memory Controller kernel module
 * Author: Dave Jiang <djiang@mvista.com>
 *
 * 2006-2007 (c) MontaVista Software, Inc.
 */

use core::ffi::{c_char, c_void};

pub const MPC85XX_REVISION: &str = " Ver: 2.0.0";
pub const EDAC_MOD_STR: &str = "MPC85xx_edac";

/* C macro: edac_printk(level, "MPC85xx", fmt, ##arg). */
macro_rules! mpc85xx_printk {
    ($level:expr, $fmt:expr $(, $arg:expr)*) => {
        edac_printk!($level, "MPC85xx", $fmt $(, $arg)*)
    };
}

/* L2 Err defines */
pub const MPC85XX_L2_ERRINJHI: u32 = 0x0000;
pub const MPC85XX_L2_ERRINJLO: u32 = 0x0004;
pub const MPC85XX_L2_ERRINJCTL: u32 = 0x0008;
pub const MPC85XX_L2_CAPTDATAHI: u32 = 0x0020;
pub const MPC85XX_L2_CAPTDATALO: u32 = 0x0024;
pub const MPC85XX_L2_CAPTECC: u32 = 0x0028;
pub const MPC85XX_L2_ERRDET: u32 = 0x0040;
pub const MPC85XX_L2_ERRDIS: u32 = 0x0044;
pub const MPC85XX_L2_ERRINTEN: u32 = 0x0048;
pub const MPC85XX_L2_ERRATTR: u32 = 0x004c;
pub const MPC85XX_L2_ERRADDR: u32 = 0x0050;
pub const MPC85XX_L2_ERRCTL: u32 = 0x0058;

/* Error Interrupt Enable */
pub const L2_EIE_L2CFGINTEN: u32 = 0x1;
pub const L2_EIE_SBECCINTEN: u32 = 0x4;
pub const L2_EIE_MBECCINTEN: u32 = 0x8;
pub const L2_EIE_TPARINTEN: u32 = 0x10;
pub const L2_EIE_MASK: u32 = L2_EIE_L2CFGINTEN | L2_EIE_SBECCINTEN |
    L2_EIE_MBECCINTEN | L2_EIE_TPARINTEN;

/* Error Detect */
pub const L2_EDE_L2CFGERR: u32 = 0x1;
pub const L2_EDE_SBECCERR: u32 = 0x4;
pub const L2_EDE_MBECCERR: u32 = 0x8;
pub const L2_EDE_TPARERR: u32 = 0x10;
pub const L2_EDE_MULL2ERR: u32 = 0x80000000;
pub const L2_EDE_CE_MASK: u32 = L2_EDE_SBECCERR;
pub const L2_EDE_UE_MASK: u32 = L2_EDE_L2CFGERR | L2_EDE_MBECCERR | L2_EDE_TPARERR;
pub const L2_EDE_MASK: u32 = L2_EDE_L2CFGERR | L2_EDE_SBECCERR |
    L2_EDE_MBECCERR | L2_EDE_TPARERR | L2_EDE_MULL2ERR;

/* PCI Err defines */
pub const PCI_EDE_TOE: u32 = 0x00000001;
pub const PCI_EDE_SCM: u32 = 0x00000002;
pub const PCI_EDE_IRMSV: u32 = 0x00000004;
pub const PCI_EDE_ORMSV: u32 = 0x00000008;
pub const PCI_EDE_OWMSV: u32 = 0x00000010;
pub const PCI_EDE_TGT_ABRT: u32 = 0x00000020;
pub const PCI_EDE_MST_ABRT: u32 = 0x00000040;
pub const PCI_EDE_TGT_PERR: u32 = 0x00000080;
pub const PCI_EDE_MST_PERR: u32 = 0x00000100;
pub const PCI_EDE_RCVD_SERR: u32 = 0x00000200;
pub const PCI_EDE_ADDR_PERR: u32 = 0x00000400;
pub const PCI_EDE_MULTI_ERR: u32 = 0x80000000;
pub const PCI_EDE_PERR_MASK: u32 = PCI_EDE_TGT_PERR | PCI_EDE_MST_PERR | PCI_EDE_ADDR_PERR;

pub const MPC85XX_PCI_ERR_DR: u32 = 0x0000;
pub const MPC85XX_PCI_ERR_CAP_DR: u32 = 0x0004;
pub const MPC85XX_PCI_ERR_EN: u32 = 0x0008;
pub const PEX_ERR_ICCAIE_EN_BIT: u32 = 0x00020000;
pub const MPC85XX_PCI_ERR_ATTRIB: u32 = 0x000c;
pub const MPC85XX_PCI_ERR_ADDR: u32 = 0x0010;
pub const PEX_ERR_ICCAD_DISR_BIT: u32 = 0x00020000;
pub const MPC85XX_PCI_ERR_EXT_ADDR: u32 = 0x0014;
pub const MPC85XX_PCI_ERR_DL: u32 = 0x0018;
pub const MPC85XX_PCI_ERR_DH: u32 = 0x001c;
pub const MPC85XX_PCI_GAS_TIMR: u32 = 0x0020;
pub const MPC85XX_PCI_PCIX_TIMR: u32 = 0x0024;
pub const MPC85XX_PCIE_ERR_CAP_R0: u32 = 0x0028;
pub const MPC85XX_PCIE_ERR_CAP_R1: u32 = 0x002c;
pub const MPC85XX_PCIE_ERR_CAP_R2: u32 = 0x0030;
pub const MPC85XX_PCIE_ERR_CAP_R3: u32 = 0x0034;

#[repr(C)]
pub struct mpc85xx_l2_pdata {
    pub name: *mut c_char,
    pub edac_idx: i32,
    pub l2_vbase: *mut c_void,
    pub irq: i32,
}

#[repr(C)]
pub struct mpc85xx_pci_pdata {
    pub name: *mut c_char,
    pub is_pcie: bool,
    pub edac_idx: i32,
    pub pci_vbase: *mut c_void,
    pub irq: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
