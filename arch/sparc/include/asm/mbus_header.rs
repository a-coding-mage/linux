/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mbus.h:  Various defines for MBUS modules.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

// Dependencies supplied by the surrounding translation unit:
// <asm/ross.h>    /* HyperSparc stuff */
// <asm/viking.h>  /* Ugh, bug city... */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mbus_module {
    HyperSparc = 0,
    Swift_ok = 4,
    Swift_bad_c = 5,
    Swift_lots_o_bugs = 6,
    Tsunami = 7,
    Viking_12 = 8,
    Viking_2x = 9,
    Viking_30 = 10,
    Viking_35 = 11,
    Viking_new = 12,
    TurboSparc = 13,
    SRMMU_INVAL_MOD = 14,
}

extern "C" {
    pub static mut srmmu_modtype: mbus_module;
    pub static mut viking_rev: ::core::ffi::c_uint;
    pub static mut swift_rev: ::core::ffi::c_uint;
    pub static mut cypress_rev: ::core::ffi::c_uint;
}

/* HW Mbus module bugs we have to deal with */
pub const HWBUG_COPYBACK_BROKEN: u32 = 0x00000001;
pub const HWBUG_ASIFLUSH_BROKEN: u32 = 0x00000002;
pub const HWBUG_VACFLUSH_BITROT: u32 = 0x00000004;
pub const HWBUG_KERN_ACCBROKEN: u32 = 0x00000008;
pub const HWBUG_KERN_CBITBROKEN: u32 = 0x00000010;
pub const HWBUG_MODIFIED_BITROT: u32 = 0x00000020;
pub const HWBUG_PC_BADFAULT_ADDR: u32 = 0x00000040;
pub const HWBUG_SUPERSCALAR_BAD: u32 = 0x00000080;
pub const HWBUG_PACINIT_BITROT: u32 = 0x00000100;

/* First the module type values. To find out which you have, just load
 * the mmu control register from ASI_M_MMUREG alternate address space and
 * shift the value right 28 bits.
 */
/* IMPL field means the company which produced the chip. */
pub const MBUS_VIKING: u32 = 0x4; /* bleech, Texas Instruments Module */
pub const MBUS_LSI: u32 = 0x3; /* LSI Logics */
pub const MBUS_ROSS: u32 = 0x1; /* Ross is nice */
pub const MBUS_FMI: u32 = 0x0; /* Fujitsu Microelectronics/Swift */

/* Ross Module versions */
pub const ROSS_604_REV_CDE: u32 = 0x0; /* revisions c, d, and e */
pub const ROSS_604_REV_F: u32 = 0x1; /* revision f */
pub const ROSS_605: u32 = 0xf; /* revision a, a.1, and a.2 */
pub const ROSS_605_REV_B: u32 = 0xe; /* revision b */

/* TI Viking Module versions */
pub const VIKING_REV_12: u32 = 0x1; /* Version 1.2 or SPARCclassic's CPU */
pub const VIKING_REV_2: u32 = 0x2; /* Version 2.1, 2.2, 2.3, and 2.4 */
pub const VIKING_REV_30: u32 = 0x3; /* Version 3.0 */
pub const VIKING_REV_35: u32 = 0x4; /* Version 3.5 */

/* LSI Logics. */
pub const LSI_L64815: u32 = 0x0;

/* Fujitsu */
pub const FMI_AURORA: u32 = 0x4; /* MB8690x, a Swift module... */
pub const FMI_TURBO: u32 = 0x5; /* MB86907, a TurboSparc module... */

/* For multiprocessor support we need to be able to obtain the CPU id and
 * the MBUS Module id.
 */

/* The CPU ID is encoded in the trap base register, 20 bits to the left of
 * bit zero, with 2 bits being significant.
 */
pub const TBR_ID_SHIFT: u32 = 20;

#[inline]
pub unsafe fn get_cpuid() -> i32 {
    let mut retval: i32;
    ::core::arch::asm!(
        "rd %tbr, {0}",
        "srl {0}, {1}, {0}",
        out(reg) retval,
        const TBR_ID_SHIFT,
    );
    retval & 3
}

#[inline]
pub unsafe fn get_modid() -> i32 {
    get_cpuid() | 0x8
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
