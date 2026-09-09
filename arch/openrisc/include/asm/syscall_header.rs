/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 * et al.
 */

// Dependencies supplied by the surrounding kernel translation:
// uapi/linux/audit.h, linux/err.h, and linux/sched.h.

pub unsafe fn syscall_get_nr(
    _task: *mut task_struct,
    regs: *mut pt_regs,
) -> i32 {
    (*regs).orig_gpr11
}

pub unsafe fn syscall_set_nr(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    nr: i32,
) {
    (*regs).orig_gpr11 = nr;
}

pub unsafe fn syscall_rollback(
    _task: *mut task_struct,
    regs: *mut pt_regs,
) {
    (*regs).gpr[11] = (*regs).orig_gpr11;
}

pub unsafe fn syscall_get_error(
    _task: *mut task_struct,
    regs: *mut pt_regs,
) -> isize {
    if is_err_value((*regs).gpr[11]) {
        (*regs).gpr[11]
    } else {
        0
    }
}

pub unsafe fn syscall_get_return_value(
    _task: *mut task_struct,
    regs: *mut pt_regs,
) -> isize {
    (*regs).gpr[11]
}

pub unsafe fn syscall_set_return_value(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    error: i32,
    val: isize,
) {
    (*regs).gpr[11] = if error != 0 { error as isize } else { val };
}

pub unsafe fn syscall_get_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *mut usize,
) {
    core::ptr::copy_nonoverlapping(
        (*regs).gpr.as_ptr().add(3),
        args,
        6,
    );
}

pub unsafe fn syscall_set_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *const usize,
) {
    core::ptr::copy_nonoverlapping(
        args,
        (*regs).gpr.as_mut_ptr().add(3),
        6,
    );
}

pub unsafe fn syscall_get_arch(_task: *mut task_struct) -> i32 {
    AUDIT_ARCH_OPENRISC
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
