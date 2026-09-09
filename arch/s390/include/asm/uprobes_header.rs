/* SPDX-License-Identifier: GPL-2.0 */
/*
 *    User-space Probes (UProbes) for s390
 *
 *    Copyright IBM Corp. 2014
 *    Author(s): Jan Willeke,
 */

// Original header guard: _ASM_UPROBES_H
// Dependency from <linux/notifier.h> is provided by the surrounding kernel bindings.

pub type uprobe_opcode_t = u16;

pub const UPROBE_XOL_SLOT_BYTES: usize = 256; /* cache aligned */

pub const UPROBE_SWBP_INSN: u32 = 0x0002;
pub const UPROBE_SWBP_INSN_SIZE: usize = 2;

#[repr(C)]
pub union arch_uprobe_insn {
    pub insn: [uprobe_opcode_t; 3],
    pub ixol: [uprobe_opcode_t; 3],
}

#[repr(C)]
pub struct arch_uprobe {
    pub insn: arch_uprobe_insn,
    pub saved_per: u32,
    pub saved_int_code: u32,
}

#[repr(C)]
pub struct arch_uprobe_task {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
