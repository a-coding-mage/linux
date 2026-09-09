/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * User-space Probes (UProbes) for x86
 *
 * Copyright (C) IBM Corporation, 2008-2011
 * Authors:
 *	Srikar Dronamraju
 *	Jim Keniston
 */

// Dependency supplied by the surrounding translation unit: linux/notifier.h

pub type uprobe_opcode_t = u8;

pub const MAX_UINSN_BYTES: usize = 16;
pub const UPROBE_XOL_SLOT_BYTES: usize = 128; /* to keep it cache aligned */

pub const UPROBE_SWBP_INSN: u8 = 0xcc;
pub const UPROBE_SWBP_INSN_SIZE: usize = 1;

pub const ARCH_UPROBE_FLAG_CAN_OPTIMIZE: u32 = 0;
pub const ARCH_UPROBE_FLAG_OPTIMIZE_FAIL: u32 = 1;

pub struct uprobe_xol_ops;

#[repr(C)]
pub union arch_uprobe_insn {
    pub insn: [u8; MAX_UINSN_BYTES],
    pub ixol: [u8; MAX_UINSN_BYTES],
}

#[repr(C)]
pub struct arch_uprobe_branch {
    pub offs: i32,
    pub ilen: u8,
    pub opc1: u8,
}

#[repr(C)]
pub struct arch_uprobe_defparam {
    pub fixups: u8,
    pub ilen: u8,
}

#[repr(C)]
pub struct arch_uprobe_push {
    pub reg_offset: u8, /* to the start of pt_regs */
    pub ilen: u8,
}

#[repr(C)]
pub union arch_uprobe_variant {
    pub branch: arch_uprobe_branch,
    pub defparam: arch_uprobe_defparam,
    pub push: arch_uprobe_push,
}

#[repr(C)]
pub struct arch_uprobe {
    pub insn: arch_uprobe_insn,
    pub ops: *const uprobe_xol_ops,
    pub variant: arch_uprobe_variant,
    pub flags: usize,
}

#[repr(C)]
pub struct arch_uprobe_task {
    #[cfg(CONFIG_X86_64)]
    pub saved_scratch_register: usize,
    pub saved_trap_nr: u32,
    pub saved_tf: u32,
}

#[cfg(CONFIG_UPROBES)]
unsafe extern "C" {
    pub fn is_uprobe_at_func_entry(regs: *mut pt_regs) -> bool;
}

#[cfg(not(CONFIG_UPROBES))]
pub unsafe fn is_uprobe_at_func_entry(_regs: *mut pt_regs) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
