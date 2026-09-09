/* SPDX-License-Identifier: GPL-2.0 */
/****************************************************************************/

/*
 * mcfdebug.h -- ColdFire Debug Module support.
 *
 * (C) Copyright 2001, Lineo Inc. (www.lineo.com)
 */

/****************************************************************************/

/* Define the debug module registers */
pub const MCFDEBUG_CSR: u32 = 0x0; /* Configuration status */
pub const MCFDEBUG_BAAR: u32 = 0x5; /* BDM address attribute */
pub const MCFDEBUG_AATR: u32 = 0x6; /* Address attribute trigger */
pub const MCFDEBUG_TDR: u32 = 0x7; /* Trigger definition */
pub const MCFDEBUG_PBR: u32 = 0x8; /* PC breakpoint */
pub const MCFDEBUG_PBMR: u32 = 0x9; /* PC breakpoint mask */
pub const MCFDEBUG_ABHR: u32 = 0xc; /* High address breakpoint */
pub const MCFDEBUG_ABLR: u32 = 0xd; /* Low address breakpoint */
pub const MCFDEBUG_DBR: u32 = 0xe; /* Data breakpoint */
pub const MCFDEBUG_DBMR: u32 = 0xf; /* Data breakpoint mask */

/* Define some handy constants for the trigger definition register */
pub const MCFDEBUG_TDR_TRC_DISP: u32 = 0x00000000;
pub const MCFDEBUG_TDR_TRC_HALT: u32 = 0x40000000;
pub const MCFDEBUG_TDR_TRC_INTR: u32 = 0x80000000;
pub const MCFDEBUG_TDR_LXT1: u32 = 0x00004000;
pub const MCFDEBUG_TDR_LXT2: u32 = 0x00008000;
pub const MCFDEBUG_TDR_EBL1: u32 = 0x00002000;
pub const MCFDEBUG_TDR_EBL2: u32 = 0x20000000;
pub const MCFDEBUG_TDR_EDLW1: u32 = 0x00001000;
pub const MCFDEBUG_TDR_EDLW2: u32 = 0x10000000;
pub const MCFDEBUG_TDR_EDWL1: u32 = 0x00000800;
pub const MCFDEBUG_TDR_EDWL2: u32 = 0x08000000;
pub const MCFDEBUG_TDR_EDWU1: u32 = 0x00000400;
pub const MCFDEBUG_TDR_EDWU2: u32 = 0x04000000;
pub const MCFDEBUG_TDR_EDLL1: u32 = 0x00000200;
pub const MCFDEBUG_TDR_EDLL2: u32 = 0x02000000;
pub const MCFDEBUG_TDR_EDLM1: u32 = 0x00000100;
pub const MCFDEBUG_TDR_EDLM2: u32 = 0x01000000;
pub const MCFDEBUG_TDR_EDUM1: u32 = 0x00000080;
pub const MCFDEBUG_TDR_EDUM2: u32 = 0x00800000;
pub const MCFDEBUG_TDR_EDUU1: u32 = 0x00000040;
pub const MCFDEBUG_TDR_EDUU2: u32 = 0x00400000;
pub const MCFDEBUG_TDR_DI1: u32 = 0x00000020;
pub const MCFDEBUG_TDR_DI2: u32 = 0x00200000;
pub const MCFDEBUG_TDR_EAI1: u32 = 0x00000010;
pub const MCFDEBUG_TDR_EAI2: u32 = 0x00100000;
pub const MCFDEBUG_TDR_EAR1: u32 = 0x00000008;
pub const MCFDEBUG_TDR_EAR2: u32 = 0x00080000;
pub const MCFDEBUG_TDR_EAL1: u32 = 0x00000004;
pub const MCFDEBUG_TDR_EAL2: u32 = 0x00040000;
pub const MCFDEBUG_TDR_EPC1: u32 = 0x00000002;
pub const MCFDEBUG_TDR_EPC2: u32 = 0x00020000;
pub const MCFDEBUG_TDR_PCI1: u32 = 0x00000001;
pub const MCFDEBUG_TDR_PCI2: u32 = 0x00010000;

/* Constants for the address attribute trigger register */
pub const MCFDEBUG_AAR_RESET: u32 = 0x00000005;
/* Fields not yet implemented */

/* And some definitions for the writable sections of the CSR */
pub const MCFDEBUG_CSR_RESET: u32 = 0x00100000;
pub const MCFDEBUG_CSR_PSTCLK: u32 = 0x00020000;
pub const MCFDEBUG_CSR_IPW: u32 = 0x00010000;
pub const MCFDEBUG_CSR_MAP: u32 = 0x00008000;
pub const MCFDEBUG_CSR_TRC: u32 = 0x00004000;
pub const MCFDEBUG_CSR_EMU: u32 = 0x00002000;
pub const MCFDEBUG_CSR_DDC_READ: u32 = 0x00000800;
pub const MCFDEBUG_CSR_DDC_WRITE: u32 = 0x00001000;
pub const MCFDEBUG_CSR_UHE: u32 = 0x00000400;
pub const MCFDEBUG_CSR_BTB0: u32 = 0x00000000;
pub const MCFDEBUG_CSR_BTB2: u32 = 0x00000100;
pub const MCFDEBUG_CSR_BTB3: u32 = 0x00000200;
pub const MCFDEBUG_CSR_BTB4: u32 = 0x00000300;
pub const MCFDEBUG_CSR_NPL: u32 = 0x00000040;
pub const MCFDEBUG_CSR_SSM: u32 = 0x00000010;

/* Constants for the BDM address attribute register */
pub const MCFDEBUG_BAAR_RESET: u32 = 0x00000005;
/* Fields not yet implemented */

/* This routine wraps up the wdebug asm instruction. */
#[inline]
pub unsafe fn wdebug(reg: i32, data: u64) {
    let mut dbg_spc: [u16; 6] = [0; 6];
    let dbg = (((dbg_spc.as_mut_ptr() as usize) + 3) & 0xfffffffc) as *mut u16;

    *dbg.add(0) = (0x2c80u16 | ((reg as u16) & 0xf));
    *dbg.add(1) = ((data >> 16) & 0xffff) as u16;
    *dbg.add(2) = (data & 0xffff) as u16;
    *dbg.add(3) = 0;

    /* The original active implementation emits: asm("wdebug (%0)" :: "a" (dbg)); */
    #[cfg(target_arch = "m68k")]
    core::arch::asm!("wdebug ({0})", in("a0") dbg);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
