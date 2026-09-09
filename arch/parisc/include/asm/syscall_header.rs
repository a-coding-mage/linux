/* SPDX-License-Identifier: GPL-2.0 */
/* syscall.h */

// Dependencies supplied by the surrounding kernel translation:
// uapi/linux/audit.h, linux/compat.h, linux/err.h, and asm/ptrace.h.

pub const NR_syscalls: _ = __NR_Linux_syscalls;

pub unsafe fn syscall_get_nr(tsk: *mut task_struct, regs: *mut pt_regs) -> libc::c_long {
    let _ = tsk;
    (*regs).gr[20] as libc::c_long
}

pub unsafe fn syscall_set_nr(tsk: *mut task_struct, regs: *mut pt_regs, nr: libc::c_int) {
    let _ = tsk;
    (*regs).gr[20] = nr as _;
}

pub unsafe fn syscall_get_arguments(
    tsk: *mut task_struct,
    regs: *mut pt_regs,
    args: *mut libc::c_ulong,
) {
    let _ = tsk;
    (*args.add(5)) = (*regs).gr[21];
    (*args.add(4)) = (*regs).gr[22];
    (*args.add(3)) = (*regs).gr[23];
    (*args.add(2)) = (*regs).gr[24];
    (*args.add(1)) = (*regs).gr[25];
    (*args.add(0)) = (*regs).gr[26];
}

pub unsafe fn syscall_set_arguments(
    tsk: *mut task_struct,
    regs: *mut pt_regs,
    args: *mut libc::c_ulong,
) {
    let _ = tsk;
    (*regs).gr[21] = *args.add(5);
    (*regs).gr[22] = *args.add(4);
    (*regs).gr[23] = *args.add(3);
    (*regs).gr[24] = *args.add(2);
    (*regs).gr[25] = *args.add(1);
    (*regs).gr[26] = *args.add(0);
}

pub unsafe fn syscall_get_error(
    task: *mut task_struct,
    regs: *mut pt_regs,
) -> libc::c_long {
    let _ = task;
    let error = (*regs).gr[28];
    if IS_ERR_VALUE(error) { error as libc::c_long } else { 0 }
}

pub unsafe fn syscall_get_return_value(
    task: *mut task_struct,
    regs: *mut pt_regs,
) -> libc::c_long {
    let _ = task;
    (*regs).gr[28] as libc::c_long
}

pub unsafe fn syscall_set_return_value(
    task: *mut task_struct,
    regs: *mut pt_regs,
    error: libc::c_int,
    val: libc::c_long,
) {
    let _ = task;
    (*regs).gr[28] = if error != 0 { error as _ } else { val as _ };
}

pub unsafe fn syscall_rollback(task: *mut task_struct, regs: *mut pt_regs) {
    let _ = (task, regs);
    /* do nothing */
}

pub unsafe fn syscall_get_arch(task: *mut task_struct) -> libc::c_int {
    let mut arch = AUDIT_ARCH_PARISC;
    #[cfg(CONFIG_64BIT)]
    if !__is_compat_task(task) {
        arch = AUDIT_ARCH_PARISC64;
    }
    arch
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
