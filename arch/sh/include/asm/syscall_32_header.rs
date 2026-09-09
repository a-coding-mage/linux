/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// audit constants, `task_struct`, `pt_regs`, and `IS_ERR_VALUE`.

/* The system call number is given by the user in R3 */
#[inline]
pub unsafe fn syscall_get_nr(
    _task: *mut task_struct,
    regs: *mut pt_regs,
) -> libc::c_long {
    if (*regs).tra >= 0 {
        (*regs).regs[3] as libc::c_long
    } else {
        -1
    }
}

#[inline]
pub unsafe fn syscall_set_nr(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    nr: libc::c_int,
) {
    /*
     * Unlike syscall_get_nr(), syscall_set_nr() can be called only when
     * the target task is stopped for tracing on entering syscall, so
     * there is no need to have the same check syscall_get_nr() has.
     */
    (*regs).regs[3] = nr as _;
}

#[inline]
pub unsafe fn syscall_rollback(_task: *mut task_struct, _regs: *mut pt_regs) {
    /*
     * XXX: This needs some thought. On SH we don't
     * save away the original r0 value anywhere.
     */
}

#[inline]
pub unsafe fn syscall_get_error(
    _task: *mut task_struct,
    regs: *mut pt_regs,
) -> libc::c_long {
    if IS_ERR_VALUE((*regs).regs[0]) {
        (*regs).regs[0] as libc::c_long
    } else {
        0
    }
}

#[inline]
pub unsafe fn syscall_get_return_value(
    _task: *mut task_struct,
    regs: *mut pt_regs,
) -> libc::c_long {
    (*regs).regs[0] as libc::c_long
}

#[inline]
pub unsafe fn syscall_set_return_value(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    error: libc::c_int,
    val: libc::c_long,
) {
    (*regs).regs[0] = if error != 0 { error as libc::c_long } else { val } as _;
}

#[inline]
pub unsafe fn syscall_get_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *mut libc::c_ulong,
) {
    /* Argument pattern is: R4, R5, R6, R7, R0, R1 */
    *args.add(5) = (*regs).regs[1] as _;
    *args.add(4) = (*regs).regs[0] as _;
    *args.add(3) = (*regs).regs[7] as _;
    *args.add(2) = (*regs).regs[6] as _;
    *args.add(1) = (*regs).regs[5] as _;
    *args.add(0) = (*regs).regs[4] as _;
}

#[inline]
pub unsafe fn syscall_set_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *const libc::c_ulong,
) {
    (*regs).regs[1] = *args.add(5) as _;
    (*regs).regs[0] = *args.add(4) as _;
    (*regs).regs[7] = *args.add(3) as _;
    (*regs).regs[6] = *args.add(2) as _;
    (*regs).regs[5] = *args.add(1) as _;
    (*regs).regs[4] = *args.add(0) as _;
}

#[inline]
pub unsafe fn syscall_get_arch(_task: *mut task_struct) -> libc::c_int {
    let mut arch = AUDIT_ARCH_SH;

    // CONFIG_CPU_LITTLE_ENDIAN conditionally adds __AUDIT_ARCH_LE.
    #[cfg(CONFIG_CPU_LITTLE_ENDIAN)]
    {
        arch |= __AUDIT_ARCH_LE;
    }
    arch
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
