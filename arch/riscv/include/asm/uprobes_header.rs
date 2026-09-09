/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the surrounding kernel translation unit:
// asm/probes.h, asm/text-patching.h, and asm/bug.h.

pub const MAX_UINSN_BYTES: usize = 8;

// CONFIG_RISCV_ISA_C selects the 16-bit breakpoint instruction; otherwise
// the 32-bit breakpoint instruction is selected.
#[cfg(feature = "CONFIG_RISCV_ISA_C")]
pub const UPROBE_SWBP_INSN: u32 = __BUG_INSN_16;
#[cfg(feature = "CONFIG_RISCV_ISA_C")]
pub const UPROBE_SWBP_INSN_SIZE: usize = 2;

#[cfg(not(feature = "CONFIG_RISCV_ISA_C"))]
pub const UPROBE_SWBP_INSN: u32 = __BUG_INSN_32;
#[cfg(not(feature = "CONFIG_RISCV_ISA_C"))]
pub const UPROBE_SWBP_INSN_SIZE: usize = 4;

pub const UPROBE_XOL_SLOT_BYTES: usize = MAX_UINSN_BYTES;

pub type uprobe_opcode_t = u32;

#[repr(C)]
pub struct arch_uprobe_task {
    pub saved_cause: usize,
}

// Supplied by asm/probes.h in the surrounding translation unit.
#[repr(C)]
pub struct arch_probe_insn {
    _private: [u8; 0],
}

#[repr(C)]
pub union arch_uprobe_insn {
    pub insn: [u8; MAX_UINSN_BYTES],
    pub ixol: [u8; MAX_UINSN_BYTES],
}

#[repr(C)]
pub struct arch_uprobe {
    pub code: arch_uprobe_insn,
    pub api: arch_probe_insn,
    pub insn_size: usize,
    pub simulate: bool,
}

// Supplied by the surrounding translation unit.
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_UPROBES")]
unsafe extern "C" {
    pub fn uprobe_breakpoint_handler(regs: *mut pt_regs) -> bool;
    pub fn uprobe_single_step_handler(regs: *mut pt_regs) -> bool;
}

#[cfg(not(feature = "CONFIG_UPROBES"))]
#[inline]
pub unsafe fn uprobe_breakpoint_handler(_regs: *mut pt_regs) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_UPROBES"))]
#[inline]
pub unsafe fn uprobe_single_step_handler(_regs: *mut pt_regs) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
