/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2008-2009 Red Hat, Inc.  All rights reserved.
 * Copyright 2010 Tilera Corporation. All Rights Reserved.
 * Copyright 2015 Regents of the University of California, Berkeley
 *
 * See asm-generic/syscall.h for descriptions of what we must do here.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// asm/hwprobe.h, uapi/linux/audit.h, linux/sched.h, and linux/err.h.

/* The array of function pointers for syscalls. */
unsafe extern "C" {
    pub static sys_call_table: *const *mut core::ffi::c_void;
    pub static compat_sys_call_table: *const *mut core::ffi::c_void;
}

/*
 * Only the low 32 bits of orig_a0 are meaningful, so we return int.
 * This importantly ignores the high bits on 64-bit, so comparisons
 * sign-extend the low 32 bits.
 */
pub unsafe fn syscall_get_nr(task: *mut task_struct, regs: *mut pt_regs) -> i32 {
    let _ = task;
    (*regs).a7
}

pub unsafe fn syscall_set_nr(task: *mut task_struct, regs: *mut pt_regs, nr: i32) {
    let _ = task;
    (*regs).a7 = nr;
}

pub unsafe fn syscall_rollback(task: *mut task_struct, regs: *mut pt_regs) {
    let _ = task;
    (*regs).a0 = (*regs).orig_a0;
}

pub unsafe fn syscall_get_error(task: *mut task_struct, regs: *mut pt_regs) -> core::ffi::c_long {
    let _ = task;
    let error = (*regs).a0 as core::ffi::c_ulong;
    if is_err_value(error) {
        error as core::ffi::c_long
    } else {
        0
    }
}

pub unsafe fn syscall_get_return_value(
    task: *mut task_struct,
    regs: *mut pt_regs,
) -> core::ffi::c_long {
    let _ = task;
    (*regs).a0
}

pub unsafe fn syscall_set_return_value(
    task: *mut task_struct,
    regs: *mut pt_regs,
    error: i32,
    val: core::ffi::c_long,
) {
    let _ = task;
    (*regs).a0 = if error as core::ffi::c_long != 0 {
        error as core::ffi::c_long
    } else {
        val
    };
}

pub unsafe fn syscall_get_arguments(
    task: *mut task_struct,
    regs: *mut pt_regs,
    args: *mut core::ffi::c_ulong,
) {
    let _ = task;
    *args.add(0) = (*regs).orig_a0;
    *args.add(1) = (*regs).a1;
    *args.add(2) = (*regs).a2;
    *args.add(3) = (*regs).a3;
    *args.add(4) = (*regs).a4;
    *args.add(5) = (*regs).a5;
}

pub unsafe fn syscall_set_arguments(
    task: *mut task_struct,
    regs: *mut pt_regs,
    args: *const core::ffi::c_ulong,
) {
    let _ = task;
    (*regs).orig_a0 = *args.add(0);
    (*regs).a1 = *args.add(1);
    (*regs).a2 = *args.add(2);
    (*regs).a3 = *args.add(3);
    (*regs).a4 = *args.add(4);
    (*regs).a5 = *args.add(5);
}

pub unsafe fn syscall_get_arch(task: *mut task_struct) -> i32 {
    let _ = task;
    // CONFIG_64BIT selects AUDIT_ARCH_RISCV64; otherwise AUDIT_ARCH_RISCV32.
    #[cfg(CONFIG_64BIT)]
    {
        AUDIT_ARCH_RISCV64
    }
    #[cfg(not(CONFIG_64BIT))]
    {
        AUDIT_ARCH_RISCV32
    }
}

pub type syscall_t = unsafe extern "C" fn(*const pt_regs) -> core::ffi::c_long;

pub unsafe fn syscall_handler(regs: *mut pt_regs, syscall: core::ffi::c_ulong) {
    let fn_: syscall_t;

    // CONFIG_COMPAT conditionally selects the compat syscall table for 32-bit userspace.
    #[cfg(CONFIG_COMPAT)]
    {
        if ((*regs).status & SR_UXL) == SR_UXL_32 {
            fn_ = *(compat_sys_call_table.add(syscall as usize) as *const syscall_t);
        } else {
            fn_ = *(sys_call_table.add(syscall as usize) as *const syscall_t);
        }
    }
    #[cfg(not(CONFIG_COMPAT))]
    {
        fn_ = *(sys_call_table.add(syscall as usize) as *const syscall_t);
    }

    (*regs).a0 = fn_(regs);
}

unsafe extern "C" {
    pub fn sys_riscv_flush_icache(
        start: usize,
        end: usize,
        flags: usize,
    ) -> core::ffi::c_long;

    pub fn sys_riscv_hwprobe(
        pairs: *mut riscv_hwprobe,
        pair_count: usize,
        cpu_count: usize,
        cpus: *mut core::ffi::c_ulong,
        flags: u32,
    ) -> core::ffi::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
