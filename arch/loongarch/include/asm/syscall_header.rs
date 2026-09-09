/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Author: Hanlu Li <lihanlu@loongson.cn>
 *         Huacai Chen <chenhuacai@loongson.cn>
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Declarations from the included kernel headers are supplied by the surrounding
// translation unit.

extern "C" {
    pub static mut sys_call_table: [*mut core::ffi::c_void; 0];
}

pub unsafe fn syscall_get_nr(_task: *mut task_struct, regs: *mut pt_regs) -> i64 {
    (*regs).regs[11]
}

pub unsafe fn syscall_set_nr(_task: *mut task_struct, regs: *mut pt_regs, nr: i32) {
    (*regs).regs[11] = nr as _;
}

pub unsafe fn syscall_rollback(_task: *mut task_struct, regs: *mut pt_regs) {
    (*regs).regs[4] = (*regs).orig_a0;
}

pub unsafe fn syscall_get_error(_task: *mut task_struct, regs: *mut pt_regs) -> i64 {
    let error: usize = (*regs).regs[4] as usize;

    if IS_ERR_VALUE(error) {
        error as i64
    } else {
        0
    }
}

pub unsafe fn syscall_get_return_value(_task: *mut task_struct, regs: *mut pt_regs) -> i64 {
    (*regs).regs[4]
}

pub unsafe fn syscall_set_return_value(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    error: i32,
    val: i64,
) {
    (*regs).regs[4] = if error as i64 != 0 { error as i64 } else { val };
}

pub unsafe fn syscall_get_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *mut usize,
) {
    *args.add(0) = (*regs).orig_a0 as usize;
    core::ptr::copy_nonoverlapping((*regs).regs.as_ptr().add(5), args.add(1), 5);
}

pub unsafe fn syscall_set_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *mut usize,
) {
    (*regs).orig_a0 = *args.add(0) as _;
    core::ptr::copy_nonoverlapping(args.add(1), (*regs).regs.as_mut_ptr().add(5), 5);
}

pub unsafe fn syscall_get_arch(_task: *mut task_struct) -> i32 {
    #[cfg(CONFIG_32BIT)]
    {
        AUDIT_ARCH_LOONGARCH32
    }
    #[cfg(not(CONFIG_32BIT))]
    {
        AUDIT_ARCH_LOONGARCH64
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
