/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Rabin Vincent <rabin at rab.in>
 */

// Dependency equivalents supplied by the surrounding translation unit:
// asm/probes.h and asm/opcodes.h

pub type uprobe_opcode_t = u32;

pub const MAX_UINSN_BYTES: usize = 4;
pub const UPROBE_XOL_SLOT_BYTES: usize = 64;

pub const UPROBE_SWBP_ARM_INSN: u32 = 0xe7f001f9;
pub const UPROBE_SS_ARM_INSN: u32 = 0xe7f001fa;
// Equivalent of __opcode_to_mem_arm(UPROBE_SWBP_ARM_INSN), supplied by asm/opcodes.h.
pub const UPROBE_SWBP_INSN: u32 = __opcode_to_mem_arm(UPROBE_SWBP_ARM_INSN);
pub const UPROBE_SWBP_INSN_SIZE: usize = 4;

#[repr(C)]
pub struct arch_uprobe_task {
    pub backup: u32,
    pub saved_trap_no: core::ffi::c_ulong,
}

#[repr(C)]
pub struct arch_uprobe {
    pub insn: [u8; MAX_UINSN_BYTES],
    pub ixol: [core::ffi::c_ulong; 2],
    pub bpinsn: uprobe_opcode_t,
    pub simulate: bool,
    pub pcreg: u32,
    pub prehandler: Option<
        unsafe extern "C" fn(
            auprobe: *mut arch_uprobe,
            autask: *mut arch_uprobe_task,
            regs: *mut pt_regs,
        ),
    >,
    pub posthandler: Option<
        unsafe extern "C" fn(
            auprobe: *mut arch_uprobe,
            autask: *mut arch_uprobe_task,
            regs: *mut pt_regs,
        ),
    >,
    pub asi: arch_probes_insn,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
