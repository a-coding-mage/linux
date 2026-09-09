/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Translated from the ARC syscall header. The included kernel types and
// helpers are supplied by the surrounding translation unit.

extern "C" {
    pub static mut sys_call_table: [*mut core::ffi::c_void; 0];
}

#[inline]
pub unsafe fn syscall_get_nr(
    task: *mut task_struct,
    regs: *mut pt_regs,
) -> libc::c_long {
    let _ = task;
    if user_mode(regs) && in_syscall(regs) {
        (*regs).r8 as libc::c_long
    } else {
        -1
    }
}

#[inline]
pub unsafe fn syscall_set_nr(
    task: *mut task_struct,
    regs: *mut pt_regs,
    nr: libc::c_int,
) {
    let _ = task;
    /*
     * Unlike syscall_get_nr(), syscall_set_nr() can be called only when
     * the target task is stopped for tracing on entering syscall, so
     * there is no need to have the same check syscall_get_nr() has.
     */
    (*regs).r8 = nr;
}

#[inline]
pub unsafe fn syscall_rollback(task: *mut task_struct, regs: *mut pt_regs) {
    let _ = task;
    (*regs).r0 = (*regs).orig_r0;
}

#[inline]
pub unsafe fn syscall_get_error(
    task: *mut task_struct,
    regs: *mut pt_regs,
) -> libc::c_long {
    let _ = task;
    /* 0 if syscall succeeded, otherwise -Errorcode */
    if IS_ERR_VALUE((*regs).r0) {
        (*regs).r0
    } else {
        0
    }
}

#[inline]
pub unsafe fn syscall_get_return_value(
    task: *mut task_struct,
    regs: *mut pt_regs,
) -> libc::c_long {
    let _ = task;
    (*regs).r0
}

#[inline]
pub unsafe fn syscall_set_return_value(
    task: *mut task_struct,
    regs: *mut pt_regs,
    error: libc::c_int,
    val: libc::c_long,
) {
    let _ = task;
    (*regs).r0 = if error != 0 { error as libc::c_long } else { val };
}

/*
 * @i:      argument index [0,5]
 * @n:      number of arguments; n+i must be [1,6].
 */
#[inline]
pub unsafe fn syscall_get_arguments(
    task: *mut task_struct,
    regs: *mut pt_regs,
    args: *mut libc::c_ulong,
) {
    let _ = task;
    let mut inside_ptregs: *mut libc::c_ulong = &mut (*regs).r0;
    let mut n: libc::c_uint = 6;
    let mut i: libc::c_uint = 0;

    while n != 0 {
        *args.add(i as usize) = *inside_ptregs;
        i += 1;
        n -= 1;
        inside_ptregs = inside_ptregs.sub(1);
    }
}

#[inline]
pub unsafe fn syscall_set_arguments(
    task: *mut task_struct,
    regs: *mut pt_regs,
    args: *const libc::c_ulong,
) {
    let _ = task;
    let mut inside_ptregs: *mut libc::c_ulong = &mut (*regs).r0;
    let mut n: libc::c_uint = 6;
    let mut i: libc::c_uint = 0;

    while n != 0 {
        *inside_ptregs = *args.add(i as usize);
        i += 1;
        n -= 1;
        inside_ptregs = inside_ptregs.sub(1);
    }
}

#[inline]
pub unsafe fn syscall_get_arch(task: *mut task_struct) -> libc::c_int {
    let _ = task;
    // CONFIG_ISA_ARCOMPACT / CONFIG_CPU_BIG_ENDIAN are build-time conditions.
    if cfg!(feature = "ISA_ARCOMPACT") {
        if cfg!(feature = "CPU_BIG_ENDIAN") {
            AUDIT_ARCH_ARCOMPACTBE
        } else {
            AUDIT_ARCH_ARCOMPACT
        }
    } else if cfg!(feature = "CPU_BIG_ENDIAN") {
        AUDIT_ARCH_ARCV2BE
    } else {
        AUDIT_ARCH_ARCV2
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
