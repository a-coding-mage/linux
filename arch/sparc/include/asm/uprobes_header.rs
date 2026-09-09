/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * User-space Probes (UProbes) for sparc
 *
 * Copyright (C) 2013 Oracle, Inc.
 *
 * Authors:
 *     Jose E. Marchesi <jose.marchesi@oracle.com>
 *	Eric Saint Etienne <eric.saint.etienne@oracle.com>
 */

pub type uprobe_opcode_t = u32;

pub const MAX_UINSN_BYTES: usize = 4;
pub const UPROBE_XOL_SLOT_BYTES: usize = MAX_UINSN_BYTES * 2;

pub const UPROBE_SWBP_INSN_SIZE: usize = 4;
pub const UPROBE_SWBP_INSN: u32 = 0x91d02073; /* ta 0x73 */
pub const UPROBE_STP_INSN: u32 = 0x91d02074; /* ta 0x74 */

pub const ANNUL_BIT: u32 = 1 << 29;

#[repr(C)]
#[derive(Copy, Clone)]
pub union arch_uprobe__bindgen_ty_1 {
    pub insn: [u8; MAX_UINSN_BYTES],
    pub ixol: u32,
}

#[repr(C)]
pub struct arch_uprobe {
    pub __bindgen_anon_1: arch_uprobe__bindgen_ty_1,
}

#[repr(C)]
pub struct arch_uprobe_task {
    pub saved_tpc: u64,
    pub saved_tnpc: u64,
}

pub struct task_struct;
pub struct notifier_block;
pub struct mm_struct;
pub struct pt_regs;

unsafe extern "C" {
    pub fn arch_uprobe_analyze_insn(
        aup: *mut arch_uprobe,
        mm: *mut mm_struct,
        addr: usize,
    ) -> i32;
    pub fn arch_uprobe_pre_xol(aup: *mut arch_uprobe, regs: *mut pt_regs) -> i32;
    pub fn arch_uprobe_post_xol(aup: *mut arch_uprobe, regs: *mut pt_regs) -> i32;
    pub fn arch_uprobe_xol_was_trapped(tsk: *mut task_struct) -> bool;
    pub fn arch_uprobe_exception_notify(
        self_: *mut notifier_block,
        val: usize,
        data: *mut core::ffi::c_void,
    ) -> i32;
    pub fn arch_uprobe_abort_xol(aup: *mut arch_uprobe, regs: *mut pt_regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
