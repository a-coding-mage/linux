/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// uapi/linux/audit.h and asm/unistd.h.

extern "C" {
    pub static sys_call_table: [usize; 0];
}

pub unsafe fn syscall_get_nr(_task: *mut task_struct, regs: *mut pt_regs) -> i32 {
    (*regs).orig_d0
}

pub unsafe fn syscall_set_nr(_task: *mut task_struct, regs: *mut pt_regs, nr: i32) {
    (*regs).orig_d0 = nr;
}

pub unsafe fn syscall_rollback(_task: *mut task_struct, regs: *mut pt_regs) {
    (*regs).d0 = (*regs).orig_d0;
}

pub unsafe fn syscall_get_error(_task: *mut task_struct, regs: *mut pt_regs) -> isize {
    let error = (*regs).d0 as usize;

    if is_err_value(error) {
        error as isize
    } else {
        0
    }
}

pub unsafe fn syscall_get_return_value(_task: *mut task_struct, regs: *mut pt_regs) -> isize {
    (*regs).d0 as isize
}

pub unsafe fn syscall_set_return_value(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    error: i32,
    val: isize,
) {
    (*regs).d0 = if error != 0 { error as isize } else { val };
}

pub unsafe fn syscall_get_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    mut args: *mut usize,
) {
    *args = (*regs).orig_d0 as usize;
    args = args.add(1);

    core::ptr::copy_nonoverlapping(
        &(*regs).d1 as *const _ as *const u8,
        args as *mut u8,
        5 * core::mem::size_of::<usize>(),
    );
}

pub unsafe fn syscall_set_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    mut args: *mut usize,
) {
    (*regs).orig_d0 = *args as _;
    args = args.add(1);

    core::ptr::copy_nonoverlapping(
        args as *const u8,
        &mut (*regs).d1 as *mut _ as *mut u8,
        5 * core::mem::size_of::<usize>(),
    );
}

pub unsafe fn syscall_get_arch(_task: *mut task_struct) -> i32 {
    AUDIT_ARCH_M68K
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
