/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// uapi/linux/audit.h, linux/audit.h, linux/sched.h, linux/types.h,
// and asm/ptrace.h.

pub unsafe fn syscall_get_arch(_task: *mut task_struct) -> i32 {
    AUDIT_ARCH_ALPHA
}

pub unsafe fn syscall_get_return_value(
    _task: *mut task_struct,
    regs: *mut pt_regs,
) -> i64 {
    if (*regs).r19 != 0 {
        -((*regs).r0 as i64)
    } else {
        (*regs).r0 as i64
    }
}

pub unsafe fn syscall_get_error(
    _task: *mut task_struct,
    regs: *mut pt_regs,
) -> i64 {
    if (*regs).r19 != 0 {
        -((*regs).r0 as i64)
    } else {
        0
    }
}

/*
 * Alpha syscall ABI / kernel conventions:
 *  - PAL provides syscall number in r0 on entry.
 *  - The kernel tracks the active syscall number in regs->r1 (mutable) and
 *    preserves the original syscall number in regs->r2 for rollback/restart.
 *  - Return value is in regs->r0, with regs->r19 ("a3") as the error flag
 *    (0=success, 1=error; on error regs->r0 holds positive errno).
 */

pub unsafe fn syscall_get_nr(
    _task: *mut task_struct,
    regs: *mut pt_regs,
) -> i64 {
    (*regs).r1 as i64
}

pub unsafe fn syscall_set_nr(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    nr: i64,
) {
    (*regs).r1 = nr as u64;
}

/*
 * Syscall arguments:
 *   regs->r16..regs->r21 carry up to 6 syscall arguments on entry.
 *   Note: regs->r19 is also used as "a3" (error flag) on syscall return.
 */

pub unsafe fn syscall_get_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *mut u64,
) {
    *args.add(0) = (*regs).r16;
    *args.add(1) = (*regs).r17;
    *args.add(2) = (*regs).r18;
    *args.add(3) = (*regs).r19;
    *args.add(4) = (*regs).r20;
    *args.add(5) = (*regs).r21;
}

pub unsafe fn syscall_set_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *const u64,
) {
    (*regs).r16 = *args.add(0);
    (*regs).r17 = *args.add(1);
    (*regs).r18 = *args.add(2);
    (*regs).r19 = *args.add(3);
    (*regs).r20 = *args.add(4);
    (*regs).r21 = *args.add(5);
}

/*
 * Set return value for a syscall.
 * Alpha uses r0 for return value and r19 ("a3") as the error indicator:
 *   a3 = 0 => success
 *   a3 = 1 => error, and userspace interprets r0 as errno (positive).
 *
 * The kernel reports errors to userspace by setting a3=1 and placing a
 * positive errno value in r0. Some syscall paths do this in entry.S,
 * while others (e.g. seccomp/ptrace helpers) use syscall_set_return_value().
 */

pub unsafe fn syscall_set_return_value(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    error: i32,
    val: i64,
) {
    if error != 0 {
        /* error is negative errno in this tree */
        (*regs).r0 = (-error) as u64; /* positive errno */
        (*regs).r19 = 1;              /* a3 = error */
    } else {
        (*regs).r0 = val as u64;
        (*regs).r19 = 0; /* a3 = success */
    }
}

/* Restore the original syscall nr after seccomp/ptrace modified regs->r1. */
pub unsafe fn syscall_rollback(
    _task: *mut task_struct,
    regs: *mut pt_regs,
) {
    (*regs).r1 = (*regs).r2;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
