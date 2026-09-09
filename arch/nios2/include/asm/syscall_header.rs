/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright Altera Corporation (C) <2014>. All rights reserved
 */

// Dependencies supplied by the corresponding kernel headers:
// <uapi/linux/audit.h>, <linux/err.h>, and <linux/sched.h>.

pub unsafe fn syscall_get_nr(
    _task: *mut task_struct,
    regs: *mut pt_regs,
) -> i32 {
    (*regs).r2
}

pub unsafe fn syscall_set_nr(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    nr: i32,
) {
    (*regs).r2 = nr;
}

pub unsafe fn syscall_rollback(
    _task: *mut task_struct,
    regs: *mut pt_regs,
) {
    (*regs).r2 = (*regs).orig_r2;
    (*regs).r7 = (*regs).orig_r7;
}

pub unsafe fn syscall_get_error(
    _task: *mut task_struct,
    regs: *mut pt_regs,
) -> i64 {
    if (*regs).r7 != 0 { (*regs).r2 as i64 } else { 0 }
}

pub unsafe fn syscall_get_return_value(
    _task: *mut task_struct,
    regs: *mut pt_regs,
) -> i64 {
    (*regs).r2 as i64
}

pub unsafe fn syscall_set_return_value(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    error: i32,
    val: i64,
) {
    if error != 0 {
        /* error < 0, but nios2 uses > 0 return value */
        (*regs).r2 = -error;
        (*regs).r7 = 1;
    } else {
        (*regs).r2 = val as _;
        (*regs).r7 = 0;
    }
}

pub unsafe fn syscall_get_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *mut libc::c_ulong,
) {
    *args = (*regs).r4 as _;
    *args.add(1) = (*regs).r5 as _;
    *args.add(2) = (*regs).r6 as _;
    *args.add(3) = (*regs).r7 as _;
    *args.add(4) = (*regs).r8 as _;
    *args.add(5) = (*regs).r9 as _;
}

pub unsafe fn syscall_set_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *const libc::c_ulong,
) {
    (*regs).r4 = *args.add(0) as _;
    (*regs).r5 = *args.add(1) as _;
    (*regs).r6 = *args.add(2) as _;
    (*regs).r7 = *args.add(3) as _;
    (*regs).r8 = *args.add(4) as _;
    (*regs).r9 = *args.add(5) as _;
}

pub unsafe fn syscall_get_arch(_task: *mut task_struct) -> i32 {
    AUDIT_ARCH_NIOS2
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
