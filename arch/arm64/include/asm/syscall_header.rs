/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependencies supplied by the surrounding kernel translation:
// uapi/linux/audit.h, linux/compat.h, and linux/err.h.

pub type SyscallFnT = unsafe extern "C" fn(regs: *const pt_regs) -> ::core::ffi::c_long;

extern "C" {
    pub static sys_call_table: *const SyscallFnT;
}

// CONFIG_COMPAT
extern "C" {
    pub static compat_sys_call_table: *const SyscallFnT;
}

pub unsafe fn syscall_get_nr(_task: *mut task_struct, regs: *mut pt_regs) -> ::core::ffi::c_int {
    (*regs).syscallno
}

pub unsafe fn syscall_rollback(_task: *mut task_struct, regs: *mut pt_regs) {
    (*regs).regs[0] = (*regs).orig_x0;
}

pub unsafe fn syscall_get_return_value(
    task: *mut task_struct,
    regs: *mut pt_regs,
) -> ::core::ffi::c_long {
    let mut val: ::core::ffi::c_ulong = (*regs).regs[0];

    if is_compat_thread(task_thread_info(task)) {
        val = sign_extend64(val, 31);
    }

    val as ::core::ffi::c_long
}

pub unsafe fn syscall_get_error(
    task: *mut task_struct,
    regs: *mut pt_regs,
) -> ::core::ffi::c_long {
    let error: ::core::ffi::c_ulong = syscall_get_return_value(task, regs) as ::core::ffi::c_ulong;

    if IS_ERR_VALUE(error) {
        error as ::core::ffi::c_long
    } else {
        0
    }
}

pub unsafe fn syscall_set_return_value(
    task: *mut task_struct,
    regs: *mut pt_regs,
    error: ::core::ffi::c_int,
    mut val: ::core::ffi::c_long,
) {
    if error != 0 {
        val = error as ::core::ffi::c_long;
    }

    if is_compat_thread(task_thread_info(task)) {
        val = lower_32_bits(val) as ::core::ffi::c_long;
    }

    (*regs).regs[0] = val as ::core::ffi::c_ulong;
}

pub unsafe fn syscall_set_nr(
    task: *mut task_struct,
    regs: *mut pt_regs,
    nr: ::core::ffi::c_int,
) {
    (*regs).syscallno = nr;
    if nr == -1 {
        /*
         * When the syscall number is set to -1, the syscall will be
         * skipped.  In this case the syscall return value has to be
         * set explicitly, otherwise the first syscall argument is
         * returned as the syscall return value.
         */
        syscall_set_return_value(task, regs, -ENOSYS, 0);
    }
}

pub unsafe fn syscall_get_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *mut ::core::ffi::c_ulong,
) {
    *args.add(0) = (*regs).orig_x0;
    *args.add(1) = (*regs).regs[1];
    *args.add(2) = (*regs).regs[2];
    *args.add(3) = (*regs).regs[3];
    *args.add(4) = (*regs).regs[4];
    *args.add(5) = (*regs).regs[5];
}

pub unsafe fn syscall_set_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *const ::core::ffi::c_ulong,
) {
    (*regs).regs[0] = *args.add(0);
    (*regs).regs[1] = *args.add(1);
    (*regs).regs[2] = *args.add(2);
    (*regs).regs[3] = *args.add(3);
    (*regs).regs[4] = *args.add(4);
    (*regs).regs[5] = *args.add(5);

    /*
     * Also copy the first argument into orig_x0
     * so that syscall_get_arguments() would return it
     * instead of the previous value.
     */
    (*regs).orig_x0 = (*regs).regs[0];
}

/*
 * We don't care about endianness (__AUDIT_ARCH_LE bit) here because
 * AArch64 has the same system calls both on little- and big- endian.
 */
pub unsafe fn syscall_get_arch(task: *mut task_struct) -> ::core::ffi::c_int {
    if is_compat_thread(task_thread_info(task)) {
        return AUDIT_ARCH_ARM;
    }

    AUDIT_ARCH_AARCH64
}

extern "C" {
    pub fn syscall_trace_enter(regs: *mut pt_regs) -> ::core::ffi::c_int;
    pub fn syscall_trace_exit(regs: *mut pt_regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
