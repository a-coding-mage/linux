/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Defines of the MIPS boards specific address-MAP, registers, etc.
 *
 * Copyright (C) 2000,2012 MIPS Technologies, Inc.
 * All rights reserved.
 * Authors: Carsten Langgaard <carstenl@mips.com>
 *          Steven J. Hill <sjhill@mips.com>
 */

// Dependencies supplied by the surrounding kernel translation:
// asm/addrspace.h, asm/byteorder.h, and asm/mips-boards/bonito64.h

/*
 * Display register base.
 */
pub const ASCII_DISPLAY_WORD_BASE: usize = 0x1f000410;
pub const ASCII_DISPLAY_POS_BASE: usize = 0x1f000418;

/*
 * Revision register.
 */
pub const MIPS_REVISION_REG: usize = 0x1fc00010;
pub const MIPS_REVISION_CORID_QED_RM5261: i32 = 0;
pub const MIPS_REVISION_CORID_CORE_LV: i32 = 1;
pub const MIPS_REVISION_CORID_BONITO64: i32 = 2;
pub const MIPS_REVISION_CORID_CORE_20K: i32 = 3;
pub const MIPS_REVISION_CORID_CORE_FPGA: i32 = 4;
pub const MIPS_REVISION_CORID_CORE_MSC: i32 = 5;
pub const MIPS_REVISION_CORID_CORE_EMUL: i32 = 6;
pub const MIPS_REVISION_CORID_CORE_FPGA2: i32 = 7;
pub const MIPS_REVISION_CORID_CORE_FPGAR2: i32 = 8;
pub const MIPS_REVISION_CORID_CORE_FPGA3: i32 = 9;
pub const MIPS_REVISION_CORID_CORE_24K: i32 = 10;
pub const MIPS_REVISION_CORID_CORE_FPGA4: i32 = 11;
pub const MIPS_REVISION_CORID_CORE_FPGA5: i32 = 12;

/**** Artificial corid defines ****/
/*
 *  CoreEMUL with   Bonito   System Controller is treated like a Core20K
 *  CoreEMUL with SOC-it 101 System Controller is treated like a CoreMSC
 */
pub const MIPS_REVISION_CORID_CORE_EMUL_BON: i32 = -1;
pub const MIPS_REVISION_CORID_CORE_EMUL_MSC: i32 = -2;

// `ioremap` is supplied by the surrounding kernel translation.
#[macro_export]
macro_rules! MIPS_REVISION_CORID {
    () => {
        ((unsafe {
            core::ptr::read_volatile(
                (ioremap($crate::MIPS_REVISION_REG, 4) as *const u32),
            )
        } >> 10) & 0x3f)
    };
}

pub const MIPS_REVISION_SCON_OTHER: i32 = 0;
pub const MIPS_REVISION_SCON_SOCITSC: i32 = 1;
pub const MIPS_REVISION_SCON_SOCITSCP: i32 = 2;

/* Artificial SCON defines for MIPS_REVISION_SCON_OTHER */
pub const MIPS_REVISION_SCON_UNKNOWN: i32 = -1;
pub const MIPS_REVISION_SCON_GT64120: i32 = -2;
pub const MIPS_REVISION_SCON_BONITO: i32 = -3;
pub const MIPS_REVISION_SCON_BRTL: i32 = -4;
pub const MIPS_REVISION_SCON_SOCIT: i32 = -5;
pub const MIPS_REVISION_SCON_ROCIT: i32 = -6;

#[macro_export]
macro_rules! MIPS_REVISION_SCONID {
    () => {
        ((unsafe {
            core::ptr::read_volatile(
                (ioremap($crate::MIPS_REVISION_REG, 4) as *const u32),
            )
        } >> 24) & 0xff)
    };
}

unsafe extern "C" {
    pub static mut mips_revision_sconid: i32;
}

#[cfg(feature = "CONFIG_PCI")]
unsafe extern "C" {
    pub fn mips_pcibios_init();
}

#[cfg(not(feature = "CONFIG_PCI"))]
#[macro_export]
macro_rules! mips_pcibios_init {
    () => {{}};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
