/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency supplied by <asm/probes.h>.

pub const MAX_UINSN_BYTES: usize = 4;

pub const UPROBE_SWBP_INSN: u32 = USR_BKPT;
pub const UPROBE_SWBP_INSN_SIZE: usize = 2;
pub const UPROBE_XOL_SLOT_BYTES: usize = MAX_UINSN_BYTES;

pub type uprobe_opcode_t = u32;

#[repr(C)]
pub struct arch_uprobe_task {
    pub saved_trap_no: core::ffi::c_ulong,
}

#[repr(C)]
pub union arch_uprobe__bindgen_ty_1 {
    pub insn: [u8; MAX_UINSN_BYTES],
    pub ixol: [u8; MAX_UINSN_BYTES],
}

#[repr(C)]
pub struct arch_uprobe {
    pub __bindgen_anon_1: arch_uprobe__bindgen_ty_1,
    pub api: arch_probe_insn,
    pub insn_size: core::ffi::c_ulong,
    pub simulate: bool,
}

extern "C" {
    pub fn uprobe_breakpoint_handler(regs: *mut pt_regs) -> core::ffi::c_int;
    pub fn uprobe_single_step_handler(regs: *mut pt_regs) -> core::ffi::c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
