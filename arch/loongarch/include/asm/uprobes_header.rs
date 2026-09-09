/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency supplied by asm/inst.h in the original header.

pub type uprobe_opcode_t = u32;

pub const MAX_UINSN_BYTES: usize = 8;
pub const UPROBE_XOL_SLOT_BYTES: usize = MAX_UINSN_BYTES;
pub const UPROBE_SWBP_INSN_SIZE: usize = LOONGARCH_INSN_SIZE;

// Original macro: __emit_break(BRK_UPROBE_BP)
// Original macro: __emit_break(BRK_UPROBE_XOLBP)
// The instruction encoding helpers and constants are supplied externally.
#[macro_export]
macro_rules! UPROBE_SWBP_INSN {
    () => { __emit_break(BRK_UPROBE_BP) };
}

#[macro_export]
macro_rules! UPROBE_XOLBP_INSN {
    () => { __emit_break(BRK_UPROBE_XOLBP) };
}

#[repr(C)]
pub struct arch_uprobe {
    pub insn: [u32; 2],
    pub ixol: [u32; 2],
    pub simulate: bool,
}

#[repr(C)]
pub struct arch_uprobe_task {
    pub saved_trap_nr: usize,
}

#[cfg(CONFIG_UPROBES)]
extern "C" {
    pub fn uprobe_breakpoint_handler(regs: *mut pt_regs) -> bool;
    pub fn uprobe_singlestep_handler(regs: *mut pt_regs) -> bool;
}

#[cfg(not(CONFIG_UPROBES))]
#[inline]
pub unsafe fn uprobe_breakpoint_handler(_regs: *mut pt_regs) -> bool {
    false
}

#[cfg(not(CONFIG_UPROBES))]
#[inline]
pub unsafe fn uprobe_singlestep_handler(_regs: *mut pt_regs) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
