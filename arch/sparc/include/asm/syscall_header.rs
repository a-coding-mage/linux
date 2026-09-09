/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// uapi/linux/audit.h, linux/kernel.h, linux/compat.h, linux/sched.h,
// asm/ptrace.h, and asm/thread_info.h.

/*
 * The syscall table always contains 32 bit pointers since we know that the
 * address of the function to be called is (way) below 4GB.  So the "int"
 * type here is what we want [need] for both 32 bit and 64 bit systems.
 */
extern "C" {
    pub static sys_call_table: [core::ffi::c_uint; 0];
}

/* The system call number is given by the user in %g1 */
pub unsafe fn syscall_get_nr(task: *mut task_struct, regs: *mut pt_regs) -> core::ffi::c_long {
    let _ = task;
    let syscall_p: i32 = pt_regs_is_syscall(regs);

    if syscall_p != 0 {
        (*regs).u_regs[UREG_G1 as usize] as core::ffi::c_long
    } else {
        -1
    }
}

pub unsafe fn syscall_set_nr(task: *mut task_struct, regs: *mut pt_regs, nr: i32) {
    let _ = task;
    /*
     * Unlike syscall_get_nr(), syscall_set_nr() can be called only when
     * the target task is stopped for tracing on entering syscall, so
     * there is no need to have the same check syscall_get_nr() has.
     */
    (*regs).u_regs[UREG_G1 as usize] = nr as _;
}

pub unsafe fn syscall_rollback(task: *mut task_struct, regs: *mut pt_regs) {
    let _ = (task, regs);
    /* XXX This needs some thought.  On Sparc we don't
     * XXX save away the original %o0 value somewhere.
     * XXX Instead we hold it in register %l5 at the top
     * XXX level trap frame and pass this down to the signal
     * XXX dispatch code which is the only place that value
     * XXX ever was needed.
     */
}

// CONFIG_SPARC32 selects the following implementation; otherwise the
// CONFIG_SPARC64 implementation is used.
#[cfg(CONFIG_SPARC32)]
pub unsafe fn syscall_has_error(regs: *mut pt_regs) -> bool {
    ((*regs).psr & PSR_C) != 0
}
#[cfg(CONFIG_SPARC32)]
pub unsafe fn syscall_set_error(regs: *mut pt_regs) {
    (*regs).psr |= PSR_C;
}
#[cfg(CONFIG_SPARC32)]
pub unsafe fn syscall_clear_error(regs: *mut pt_regs) {
    (*regs).psr &= !PSR_C;
}

#[cfg(not(CONFIG_SPARC32))]
pub unsafe fn syscall_has_error(regs: *mut pt_regs) -> bool {
    ((*regs).tstate & (TSTATE_XCARRY | TSTATE_ICARRY)) != 0
}
#[cfg(not(CONFIG_SPARC32))]
pub unsafe fn syscall_set_error(regs: *mut pt_regs) {
    (*regs).tstate |= TSTATE_XCARRY | TSTATE_ICARRY;
}
#[cfg(not(CONFIG_SPARC32))]
pub unsafe fn syscall_clear_error(regs: *mut pt_regs) {
    (*regs).tstate &= !(TSTATE_XCARRY | TSTATE_ICARRY);
}

pub unsafe fn syscall_get_error(task: *mut task_struct, regs: *mut pt_regs) -> core::ffi::c_long {
    let _ = task;
    let val = (*regs).u_regs[UREG_I0 as usize] as core::ffi::c_long;

    if syscall_has_error(regs) { -val } else { 0 }
}

pub unsafe fn syscall_get_return_value(
    task: *mut task_struct,
    regs: *mut pt_regs,
) -> core::ffi::c_long {
    let _ = task;
    (*regs).u_regs[UREG_I0 as usize] as core::ffi::c_long
}

pub unsafe fn syscall_set_return_value(
    task: *mut task_struct,
    regs: *mut pt_regs,
    error: i32,
    val: core::ffi::c_long,
) {
    let _ = task;
    if error != 0 {
        syscall_set_error(regs);
        (*regs).u_regs[UREG_I0 as usize] = (-error) as _;
    } else {
        syscall_clear_error(regs);
        (*regs).u_regs[UREG_I0 as usize] = val as _;
    }
}

pub unsafe fn syscall_get_arguments(
    task: *mut task_struct,
    regs: *mut pt_regs,
    args: *mut core::ffi::c_ulong,
) {
    let mut zero_extend = false;
    let n: usize = 6;

    // CONFIG_SPARC64 conditionally enables this TIF_32BIT check.
    #[cfg(CONFIG_SPARC64)]
    {
        if test_tsk_thread_flag(task, TIF_32BIT) != 0 {
            zero_extend = true;
        }
    }

    for j in 0..n {
        let val = (*regs).u_regs[UREG_I0 as usize + j];
        *args.add(j) = if zero_extend { val as u32 as _ } else { val };
    }
}

pub unsafe fn syscall_set_arguments(
    task: *mut task_struct,
    regs: *mut pt_regs,
    args: *const core::ffi::c_ulong,
) {
    let _ = task;
    for i in 0..6usize {
        (*regs).u_regs[UREG_I0 as usize + i] = *args.add(i);
    }
}

pub unsafe fn syscall_get_arch(task: *mut task_struct) -> i32 {
    // The following preserves the source's CONFIG_SPARC64/CONFIG_COMPAT
    // build-time selection.
    #[cfg(all(CONFIG_SPARC64, CONFIG_COMPAT))]
    {
        return if test_tsk_thread_flag(task, TIF_32BIT) != 0 {
            AUDIT_ARCH_SPARC
        } else {
            AUDIT_ARCH_SPARC64
        };
    }
    #[cfg(all(CONFIG_SPARC64, not(CONFIG_COMPAT)))]
    {
        return AUDIT_ARCH_SPARC64;
    }
    #[cfg(not(CONFIG_SPARC64))]
    {
        let _ = task;
        AUDIT_ARCH_SPARC
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
