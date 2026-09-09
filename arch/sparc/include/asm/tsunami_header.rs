/* SPDX-License-Identifier: GPL-2.0 */
/*
 * tsunami.h: Module specific definitions for Tsunami V8 Sparcs
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

// Dependency supplied by the corresponding assembly/ASI definitions.

/* The MMU control register on the Tsunami:
 *
 * -----------------------------------------------------------------------
 * | implvers |SW|AV|DV|MV| RSV |PC|ITD|ALC| RSV |PE| RC |IE|DE|RSV|NF|ME|
 * -----------------------------------------------------------------------
 *  31      24 23 22 21 20 19-18 17  16 14  13-12 11 10-9  8  7 6-2  1  0
 *
 * SW: Enable Software Table Walks  0=off 1=on
 * AV: Address View bit
 * DV: Data View bit
 * MV: Memory View bit
 * PC: Parity Control
 * ITD: ITBR disable
 * ALC: Alternate Cacheable
 * PE: Parity Enable   0=off 1=on
 * RC: Refresh Control
 * IE: Instruction cache Enable  0=off 1=on
 * DE: Data cache Enable  0=off 1=on
 * NF: No Fault, same as all other SRMMUs
 * ME: MMU Enable, same as all other SRMMUs
 */

pub const TSUNAMI_SW: u32 = 0x00800000;
pub const TSUNAMI_AV: u32 = 0x00400000;
pub const TSUNAMI_DV: u32 = 0x00200000;
pub const TSUNAMI_MV: u32 = 0x00100000;
pub const TSUNAMI_PC: u32 = 0x00020000;
pub const TSUNAMI_ITD: u32 = 0x00010000;
pub const TSUNAMI_ALC: u32 = 0x00008000;
pub const TSUNAMI_PE: u32 = 0x00001000;
pub const TSUNAMI_RCMASK: u32 = 0x00000C00;
pub const TSUNAMI_IENAB: u32 = 0x00000200;
pub const TSUNAMI_DENAB: u32 = 0x00000100;
pub const TSUNAMI_NF: u32 = 0x00000002;
pub const TSUNAMI_ME: u32 = 0x00000001;

#[inline]
pub unsafe fn tsunami_flush_icache() {
    // C: sta %%g0, [%%g0] ASI_M_IC_FLCLEAR
    core::arch::asm!(
        "sta %g0, [%g0] {asi}",
        asi = const ASI_M_IC_FLCLEAR,
        options(nostack)
    );
}

#[inline]
pub unsafe fn tsunami_flush_dcache() {
    // C: sta %%g0, [%%g0] ASI_M_DC_FLCLEAR
    core::arch::asm!(
        "sta %g0, [%g0] {asi}",
        asi = const ASI_M_DC_FLCLEAR,
        options(nostack)
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
