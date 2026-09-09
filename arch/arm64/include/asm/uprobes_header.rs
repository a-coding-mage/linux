/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2014-2016 Pratyush Anand <panand@redhat.com>
 */

// Dependencies supplied by the surrounding translation unit:
// asm/debug-monitors.h, asm/insn.h, and asm/probes.h

pub const UPROBE_SWBP_INSN: u32 = cpu_to_le32(BRK64_OPCODE_UPROBES);
pub const UPROBE_SWBP_INSN_SIZE: usize = AARCH64_INSN_SIZE;
pub const UPROBE_XOL_SLOT_BYTES: usize = AARCH64_INSN_SIZE;

pub type uprobe_opcode_t = __le32;

#[repr(C)]
pub struct arch_uprobe_task {}

#[repr(C)]
pub union arch_uprobe_insn {
    pub insn: __le32,
    pub ixol: __le32,
}

#[repr(C)]
pub struct arch_uprobe {
    pub insn: arch_uprobe_insn,
    pub api: arch_probe_insn,
    pub simulate: bool,
}

extern "C" {
    pub fn uprobe_brk_handler(regs: *mut pt_regs, esr: c_ulong) -> c_int;
}

// CONFIG_UPROBES conditionally selects the external single-step handler.
#[cfg(feature = "CONFIG_UPROBES")]
extern "C" {
    pub fn uprobe_single_step_handler(regs: *mut pt_regs, esr: c_ulong) -> c_int;
}

#[cfg(not(feature = "CONFIG_UPROBES"))]
#[inline]
pub unsafe fn uprobe_single_step_handler(regs: *mut pt_regs, esr: c_ulong) -> c_int {
    let _ = (regs, esr);
    DBG_HOOK_ERROR
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
