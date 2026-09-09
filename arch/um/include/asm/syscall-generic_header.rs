/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Access to user system call parameters and results
 *
 * See asm-generic/syscall.h for function descriptions.
 *
 * Copyright (C) 2015 Mickaël Salaün <mic@digikod.net>
 */

// Dependencies supplied by the surrounding translation unit:
// asm/ptrace.h, linux/err.h, linux/sched.h, and sysdep/ptrace.h.

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct uml_pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    pub regs: uml_pt_regs,
}

extern "C" {
    fn PT_REGS_SYSCALL_NR(regs: *mut pt_regs) -> i32;
    fn PT_REGS_SET_SYSCALL_NR(regs: *mut pt_regs, nr: i32);
    fn regs_return_value(regs: *mut pt_regs) -> i64;
    fn IS_ERR_VALUE(error: i64) -> bool;
    fn PT_REGS_SET_SYSCALL_RETURN(regs: *mut pt_regs, value: i64);

    fn UPT_SYSCALL_ARG1(regs: *const uml_pt_regs) -> u64;
    fn UPT_SYSCALL_ARG2(regs: *const uml_pt_regs) -> u64;
    fn UPT_SYSCALL_ARG3(regs: *const uml_pt_regs) -> u64;
    fn UPT_SYSCALL_ARG4(regs: *const uml_pt_regs) -> u64;
    fn UPT_SYSCALL_ARG5(regs: *const uml_pt_regs) -> u64;
    fn UPT_SYSCALL_ARG6(regs: *const uml_pt_regs) -> u64;
    fn UPT_SET_SYSCALL_ARG1(regs: *mut uml_pt_regs, value: u64);
    fn UPT_SET_SYSCALL_ARG2(regs: *mut uml_pt_regs, value: u64);
    fn UPT_SET_SYSCALL_ARG3(regs: *mut uml_pt_regs, value: u64);
    fn UPT_SET_SYSCALL_ARG4(regs: *mut uml_pt_regs, value: u64);
    fn UPT_SET_SYSCALL_ARG5(regs: *mut uml_pt_regs, value: u64);
    fn UPT_SET_SYSCALL_ARG6(regs: *mut uml_pt_regs, value: u64);
}

pub unsafe fn syscall_get_nr(_task: *mut task_struct, regs: *mut pt_regs) -> i32 {
    PT_REGS_SYSCALL_NR(regs)
}

pub unsafe fn syscall_set_nr(_task: *mut task_struct, regs: *mut pt_regs, nr: i32) {
    PT_REGS_SET_SYSCALL_NR(regs, nr);
}

pub unsafe fn syscall_rollback(_task: *mut task_struct, _regs: *mut pt_regs) {
    /* do nothing */
}

pub unsafe fn syscall_get_error(_task: *mut task_struct, regs: *mut pt_regs) -> i64 {
    let error = regs_return_value(regs);
    if IS_ERR_VALUE(error) { error } else { 0 }
}

pub unsafe fn syscall_get_return_value(_task: *mut task_struct, regs: *mut pt_regs) -> i64 {
    regs_return_value(regs)
}

pub unsafe fn syscall_set_return_value(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    error: i32,
    val: i64,
) {
    // C's `(long) error ?: val` selects val when the converted error is zero.
    PT_REGS_SET_SYSCALL_RETURN(regs, if error as i64 != 0 { error as i64 } else { val });
}

pub unsafe fn syscall_get_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    mut args: *mut u64,
) {
    let r = &mut (*regs).regs as *mut uml_pt_regs;
    *args = UPT_SYSCALL_ARG1(r); args = args.add(1);
    *args = UPT_SYSCALL_ARG2(r); args = args.add(1);
    *args = UPT_SYSCALL_ARG3(r); args = args.add(1);
    *args = UPT_SYSCALL_ARG4(r); args = args.add(1);
    *args = UPT_SYSCALL_ARG5(r); args = args.add(1);
    *args = UPT_SYSCALL_ARG6(r);
}

pub unsafe fn syscall_set_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    mut args: *const u64,
) {
    let r = &mut (*regs).regs as *mut uml_pt_regs;
    UPT_SET_SYSCALL_ARG1(r, *args); args = args.add(1);
    UPT_SET_SYSCALL_ARG2(r, *args); args = args.add(1);
    UPT_SET_SYSCALL_ARG3(r, *args); args = args.add(1);
    UPT_SET_SYSCALL_ARG4(r, *args); args = args.add(1);
    UPT_SET_SYSCALL_ARG5(r, *args); args = args.add(1);
    UPT_SET_SYSCALL_ARG6(r, *args);
}

/* See arch/x86/um/asm/syscall.h for syscall_get_arch() definition. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
