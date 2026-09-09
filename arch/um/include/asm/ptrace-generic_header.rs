/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// The C header includes <sysdep/ptrace.h>; its supplied types and UPT_* /
// EMPTY_UML_PT_REGS definitions remain external dependencies of this file.

#[repr(C)]
pub struct pt_regs {
    pub regs: uml_pt_regs,
}

pub const fn arch_has_single_step() -> i32 {
    1
}

#[macro_export]
macro_rules! EMPTY_REGS {
    () => {
        $crate::pt_regs { regs: EMPTY_UML_PT_REGS }
    };
}

#[inline]
pub unsafe fn PT_REGS_IP(r: *mut pt_regs) -> _ {
    UPT_IP(&mut (*r).regs)
}

#[inline]
pub unsafe fn PT_REGS_SP(r: *mut pt_regs) -> _ {
    UPT_SP(&mut (*r).regs)
}

#[inline]
pub unsafe fn PT_REGS_RESTART_SYSCALL(r: *mut pt_regs) -> _ {
    UPT_RESTART_SYSCALL(&mut (*r).regs)
}

#[inline]
pub unsafe fn PT_REGS_SYSCALL_NR(r: *mut pt_regs) -> _ {
    UPT_SYSCALL_NR(&mut (*r).regs)
}

#[inline]
pub unsafe fn instruction_pointer(regs: *mut pt_regs) -> _ {
    PT_REGS_IP(regs)
}

pub const PTRACE_OLDSETOPTIONS: i32 = 21;

pub struct task_struct;

extern "C" {
    pub fn subarch_ptrace(
        child: *mut task_struct,
        request: i64,
        addr: usize,
        data: usize,
    ) -> i64;
    pub fn getreg(child: *mut task_struct, regno: i32) -> usize;
    pub fn putreg(child: *mut task_struct, regno: i32, value: usize) -> i32;

    pub fn poke_user(child: *mut task_struct, addr: i64, data: i64) -> i32;
    pub fn peek_user(child: *mut task_struct, addr: i64, data: i64) -> i32;

    pub fn arch_set_tls(new: *mut task_struct, tls: usize) -> i32;
    pub fn clear_flushed_tls(task: *mut task_struct);
    pub fn syscall_trace_enter(regs: *mut pt_regs) -> i32;
    pub fn syscall_trace_leave(regs: *mut pt_regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
