/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2011 Tobias Klauser <tklauser@distanz.ch>
 */

// The C header guard and include are intentionally omitted.  `cpuinfo` is
// supplied by the corresponding dependency when non-assembler code is used.

/* control register numbers */
pub const CTL_FSTATUS: u32 = 0;
pub const CTL_ESTATUS: u32 = 1;
pub const CTL_BSTATUS: u32 = 2;
pub const CTL_IENABLE: u32 = 3;
pub const CTL_IPENDING: u32 = 4;
pub const CTL_CPUID: u32 = 5;
pub const CTL_RSV1: u32 = 6;
pub const CTL_EXCEPTION: u32 = 7;
pub const CTL_PTEADDR: u32 = 8;
pub const CTL_TLBACC: u32 = 9;
pub const CTL_TLBMISC: u32 = 10;
pub const CTL_RSV2: u32 = 11;
pub const CTL_BADADDR: u32 = 12;
pub const CTL_CONFIG: u32 = 13;
pub const CTL_MPUBASE: u32 = 14;
pub const CTL_MPUACC: u32 = 15;

/* Access control registers using GCC builtins.  These are supplied by the
 * target/compiler integration; declarations are retained here. */
unsafe extern "C" {
    pub fn RDCTL(r: u32) -> u32;
    pub fn WRCTL(r: u32, v: u32);
}

/* status register bits */
pub const STATUS_PIE: u32 = 1 << 0; // processor interrupt enable
pub const STATUS_U: u32 = 1 << 1; // user mode
pub const STATUS_EH: u32 = 1 << 2; // Exception mode

/* estatus register bits */
pub const ESTATUS_EPIE: u32 = 1 << 0; // processor interrupt enable
pub const ESTATUS_EU: u32 = 1 << 1; // user mode
pub const ESTATUS_EH: u32 = 1 << 2; // Exception mode

/* tlbmisc register bits */
pub const TLBMISC_PID_SHIFT: u32 = 4;
// In C this depends on `cpuinfo.tlb_pid_num_bits` and is unavailable to the
// assembler configuration; pass that field explicitly at use sites.
#[inline]
pub const fn TLBMISC_PID_MASK(tlb_pid_num_bits: u32) -> u32 {
    (1u32 << tlb_pid_num_bits).wrapping_sub(1)
}
pub const TLBMISC_WAY_MASK: u32 = 0xf;
pub const TLBMISC_WAY_SHIFT: u32 = 20;

// TLB PID
#[inline]
pub const fn TLBMISC_PID(tlb_pid_num_bits: u32) -> u32 {
    TLBMISC_PID_MASK(tlb_pid_num_bits) << TLBMISC_PID_SHIFT
}
pub const TLBMISC_WE: u32 = 1 << 18; // TLB write enable
pub const TLBMISC_RD: u32 = 1 << 19; // TLB read
#[inline]
pub const fn TLBMISC_WAY() -> u32 {
    TLBMISC_WAY_MASK << TLBMISC_WAY_SHIFT
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
