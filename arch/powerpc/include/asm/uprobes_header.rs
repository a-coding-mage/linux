/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * User-space Probes (UProbes) for powerpc
 *
 * Copyright IBM Corporation, 2007-2012
 *
 * Adapted from the x86 port by Ananth N Mavinakayanahalli <ananth@in.ibm.com>
 */

/* Dependencies supplied by the surrounding kernel translation. */

pub type uprobe_opcode_t = u32;

pub const MAX_UINSN_BYTES: usize = 8;
pub const UPROBE_XOL_SLOT_BYTES: usize = MAX_UINSN_BYTES;

/* The following alias is needed for reference from arch-agnostic code. */
pub const UPROBE_SWBP_INSN: u32 = BREAKPOINT_INSTRUCTION;
pub const UPROBE_SWBP_INSN_SIZE: usize = 4; /* swbp insn size in bytes */

#[repr(C)]
pub union arch_uprobe {
    pub insn: [u32; 2],
    pub ixol: [u32; 2],
}

#[repr(C)]
pub struct arch_uprobe_task {
    pub saved_trap_nr: ::core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
