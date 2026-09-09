/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Access to user system call parameters and results
 *
 * See asm-generic/syscall.h for descriptions of what we must do here.
 */

// Dependencies supplied by the surrounding kernel translation.

pub const NR_syscalls: usize = __NR_syscalls;

extern "C" {
    pub static sys_call_table: [::core::ffi::c_ulong; 0];
}

#[inline]
pub unsafe fn syscall_get_nr(task: *mut task_struct, _regs: *mut pt_regs) -> ::core::ffi::c_int {
    if IS_ENABLED!(CONFIG_AEABI) && !IS_ENABLED!(CONFIG_OABI_COMPAT) {
        return (*task_thread_info(task)).abi_syscall;
    }

    if (*task_thread_info(task)).abi_syscall == -1 {
        return -1;
    }

    (*task_thread_info(task)).abi_syscall & __NR_SYSCALL_MASK
}

#[inline]
pub unsafe fn __in_oabi_syscall(task: *mut task_struct) -> bool {
    IS_ENABLED!(CONFIG_OABI_COMPAT)
        && ((*task_thread_info(task)).abi_syscall & __NR_OABI_SYSCALL_BASE) != 0
}

#[inline]
pub unsafe fn in_oabi_syscall() -> bool {
    __in_oabi_syscall(current)
}

#[inline]
pub unsafe fn syscall_rollback(_task: *mut task_struct, regs: *mut pt_regs) {
    (*regs).ARM_r0 = (*regs).ARM_ORIG_r0;
}

#[inline]
pub unsafe fn syscall_get_error(
    _task: *mut task_struct,
    regs: *mut pt_regs,
) -> ::core::ffi::c_long {
    let error: ::core::ffi::c_ulong = (*regs).ARM_r0;
    if IS_ERR_VALUE(error) { error as ::core::ffi::c_long } else { 0 }
}

#[inline]
pub unsafe fn syscall_get_return_value(
    _task: *mut task_struct,
    regs: *mut pt_regs,
) -> ::core::ffi::c_long {
    (*regs).ARM_r0
}

#[inline]
pub unsafe fn syscall_set_return_value(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    error: ::core::ffi::c_int,
    val: ::core::ffi::c_long,
) {
    (*regs).ARM_r0 = if error != 0 { error as ::core::ffi::c_long } else { val };
}

#[inline]
pub unsafe fn syscall_set_nr(
    task: *mut task_struct,
    regs: *mut pt_regs,
    nr: ::core::ffi::c_int,
) {
    if nr == -1 {
        (*task_thread_info(task)).abi_syscall = -1;
        /*
         * When the syscall number is set to -1, the syscall will be
         * skipped. In this case the syscall return value has to be set
         * explicitly, otherwise the first syscall argument is returned.
         */
        syscall_set_return_value(task, regs, -ENOSYS, 0);
        return;
    }
    if IS_ENABLED!(CONFIG_AEABI) && !IS_ENABLED!(CONFIG_OABI_COMPAT) {
        (*task_thread_info(task)).abi_syscall = nr;
        return;
    }
    (*task_thread_info(task)).abi_syscall =
        ((*task_thread_info(task)).abi_syscall & !__NR_SYSCALL_MASK)
            | (nr & __NR_SYSCALL_MASK);
}

#[inline]
pub unsafe fn syscall_get_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *mut ::core::ffi::c_ulong,
) {
    *args = (*regs).ARM_ORIG_r0;
    core::ptr::copy_nonoverlapping(
        (&(*regs).ARM_r0).add(1),
        args.add(1),
        5,
    );
}

#[inline]
pub unsafe fn syscall_set_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *const ::core::ffi::c_ulong,
) {
    core::ptr::copy_nonoverlapping(args, &mut (*regs).ARM_r0, 6);
    /*
     * Also copy the first argument into ARM_ORIG_r0
     * so that syscall_get_arguments() would return it
     * instead of the previous value.
     */
    (*regs).ARM_ORIG_r0 = (*regs).ARM_r0;
}

#[inline]
pub unsafe fn syscall_get_arch(_task: *mut task_struct) -> ::core::ffi::c_int {
    /* ARM tasks don't change audit architectures on the fly. */
    AUDIT_ARCH_ARM
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
