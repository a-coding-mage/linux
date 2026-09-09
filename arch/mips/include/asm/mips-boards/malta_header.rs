/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Carsten Langgaard, carstenl@mips.com
 * Copyright (C) 2000 MIPS Technologies, Inc.  All rights reserved.
 *
 * Defines of the Malta board specific address-MAP, registers, etc.
 */

use core::ffi::c_void;

// Dependencies supplied by the corresponding architecture headers.
extern "C" {
    fn GT_READ(reg: usize) -> usize;
    fn MSC_READ(reg: usize, value: *mut usize);
    fn ioremap(addr: usize, size: usize) -> *mut c_void;
    fn outb(value: u8, address: u16);
}

/* Mips interrupt controller found in SOCit variations */
pub const MIPS_MSC01_IC_REG_BASE: usize = 0x1bc40000;
pub const MIPS_SOCITSC_IC_REG_BASE: usize = 0x1ffa0000;

/*
 * Malta I/O ports base address for the Galileo GT64120 and Algorithmics
 * Bonito system controllers.
 */
// GT_PCI0IOLD_OFS and MSC01_PCI_SC2PIOBASL are supplied by the corresponding
// architecture headers.
pub unsafe fn MALTA_GT_PORT_BASE() -> usize {
    get_gt_port_base(GT_PCI0IOLD_OFS)
}

pub unsafe fn MALTA_BONITO_PORT_BASE() -> usize {
    ioremap(0x1fd00000, 0x10000) as usize
}

pub unsafe fn MALTA_MSC_PORT_BASE() -> usize {
    get_msc_port_base(MSC01_PCI_SC2PIOBASL)
}

pub unsafe fn get_gt_port_base(reg: usize) -> usize {
    let addr = GT_READ(reg);
    ioremap((addr & 0xffff) << 21, 0x10000) as usize
}

pub unsafe fn get_msc_port_base(reg: usize) -> usize {
    let mut addr = 0usize;
    MSC_READ(reg, &mut addr as *mut usize);
    ioremap(addr, 0x10000) as usize
}

/* GCMP Specific definitions */
pub const GCMP_BASE_ADDR: usize = 0x1fbf8000;
pub const GCMP_ADDRSPACE_SZ: usize = 256 * 1024;

/* GIC Specific definitions */
pub const GIC_BASE_ADDR: usize = 0x1bdc0000;
pub const GIC_ADDRSPACE_SZ: usize = 128 * 1024;

/* CPC Specific definitions */
pub const CPC_BASE_ADDR: usize = 0x1bde0000;

/* MSC01 BIU Specific definitions
 * FIXME : These should be elsewhere ?
 */
pub const MSC01_BIU_REG_BASE: usize = 0x1bc80000;
pub const MSC01_BIU_ADDRSPACE_SZ: usize = 256 * 1024;
pub const MSC01_SC_CFG_OFS: usize = 0x0110;
pub const MSC01_SC_CFG_GICPRES_MSK: usize = 0x00000004;
pub const MSC01_SC_CFG_GICPRES_SHF: usize = 2;
pub const MSC01_SC_CFG_GICENA_SHF: usize = 3;

/* Malta RTC-device indirect register access. */
pub const MALTA_RTC_ADR_REG: u16 = 0x70;
pub const MALTA_RTC_DAT_REG: u16 = 0x71;

/* Malta SMSC FDC37M817 Super I/O Controller register. */
pub const SMSC_CONFIG_REG: u16 = 0x3f0;
pub const SMSC_DATA_REG: u16 = 0x3f1;

pub const SMSC_CONFIG_DEVNUM: u8 = 0x7;
pub const SMSC_CONFIG_ACTIVATE: u8 = 0x30;
pub const SMSC_CONFIG_ENTER: u8 = 0x55;
pub const SMSC_CONFIG_EXIT: u8 = 0xaa;

pub const SMSC_CONFIG_DEVNUM_FLOPPY: u8 = 0;
pub const SMSC_CONFIG_ACTIVATE_ENABLE: u8 = 1;

pub unsafe fn SMSC_WRITE(x: u8, a: u16) {
    outb(x, a);
}

pub const MALTA_JMPRS_REG: usize = 0x1f000210;

// __init declaration preserved as an external C ABI function.
extern "C" {
    pub fn malta_dt_shim(fdt: *mut c_void) -> *mut c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
