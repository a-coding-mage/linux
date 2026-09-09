/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the MicroBlaze syscall header.
// Dependencies supplied by the original includes are referenced but not implemented here.

/* The system call number is given by the user in R12 */
#[inline]
pub unsafe fn syscall_get_nr(task: *mut task_struct, regs: *mut pt_regs) -> i64 {
    let _ = task;
    (*regs).r12 as i64
}

#[inline]
pub unsafe fn syscall_set_nr(task: *mut task_struct, regs: *mut pt_regs, nr: i32) {
    let _ = task;
    (*regs).r12 = nr;
}

#[inline]
pub unsafe fn syscall_rollback(task: *mut task_struct, regs: *mut pt_regs) {
    let _ = task;
    let _ = regs;
    /* TODO.  */
}

#[inline]
pub unsafe fn syscall_get_error(task: *mut task_struct, regs: *mut pt_regs) -> i64 {
    let _ = task;
    if IS_ERR_VALUE((*regs).r3) {
        (*regs).r3 as i64
    } else {
        0
    }
}

#[inline]
pub unsafe fn syscall_get_return_value(task: *mut task_struct, regs: *mut pt_regs) -> i64 {
    let _ = task;
    (*regs).r3 as i64
}

#[inline]
pub unsafe fn syscall_set_return_value(
    task: *mut task_struct,
    regs: *mut pt_regs,
    error: i32,
    val: i64,
) {
    let _ = task;
    if error != 0 {
        (*regs).r3 = (-(error as i64)) as _;
    } else {
        (*regs).r3 = val as _;
    }
}

#[inline]
pub unsafe fn microblaze_get_syscall_arg(
    regs: *mut pt_regs,
    n: u32,
) -> microblaze_reg_t {
    match n {
        5 => (*regs).r10,
        4 => (*regs).r9,
        3 => (*regs).r8,
        2 => (*regs).r7,
        1 => (*regs).r6,
        0 => (*regs).r5,
        _ => {
            BUG();
            !0
        }
    }
}

#[inline]
pub unsafe fn syscall_get_arguments(
    task: *mut task_struct,
    regs: *mut pt_regs,
    mut args: *mut u64,
) {
    let _ = task;
    let mut i: u32 = 0;
    let mut n: u32 = 6;

    while n != 0 {
        *args = microblaze_get_syscall_arg(regs, i) as u64;
        args = args.add(1);
        i += 1;
        n -= 1;
    }
}

pub unsafe extern "C" fn do_syscall_trace_enter(regs: *mut pt_regs) -> u64;
pub unsafe extern "C" fn do_syscall_trace_leave(regs: *mut pt_regs);

#[inline]
pub unsafe fn syscall_get_arch(task: *mut task_struct) -> i32 {
    let _ = task;
    AUDIT_ARCH_MICROBLAZE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
