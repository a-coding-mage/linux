/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Access to user system call parameters and results
 *
 * Copyright (C) 2008 Red Hat, Inc.  All rights reserved.
 *
 * See asm-generic/syscall.h for descriptions of what we must do here.
 */

/* Dependencies: uapi/linux/audit.h, linux/sched.h, linux/thread_info.h. */

/* CONFIG_ARCH_HAS_SYSCALL_WRAPPER selects the corresponding C function type. */
#[cfg(CONFIG_ARCH_HAS_SYSCALL_WRAPPER)]
pub type SyscallFn = unsafe extern "C" fn(*const PtRegs) -> c_long;
#[cfg(not(CONFIG_ARCH_HAS_SYSCALL_WRAPPER))]
pub type SyscallFn = unsafe extern "C" fn(
    c_ulong, c_ulong, c_ulong, c_ulong, c_ulong, c_ulong,
) -> c_long;

/* ftrace syscalls requires exporting the sys_call_table. */
extern "C" {
    pub static sys_call_table: [SyscallFn; 0];
    pub static compat_sys_call_table: [SyscallFn; 0];
}

pub type c_long = isize;
pub type c_ulong = usize;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PtRegs {
    pub gpr: [c_ulong; 32],
    pub orig_gpr3: c_ulong,
    pub ccr: c_ulong,
}

extern "C" {
    fn trap_is_syscall(regs: *const PtRegs) -> bool;
    fn trap_is_scv(regs: *const PtRegs) -> bool;
    fn is_tsk_32bit_task(task: *const task_struct) -> bool;
    fn set_thread_flag(flag: usize);
    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
}

/* External constants/macros supplied by the corresponding kernel headers. */
extern "C" {
    static _AUDIT_ARCH_PPC: i32;
    static _AUDIT_ARCH_PPC64LE: i32;
    static _AUDIT_ARCH_PPC64: i32;
    static _TIF_SYSCALL_RET: usize;
}

#[inline]
pub unsafe fn syscall_get_nr(_task: *mut task_struct, regs: *mut PtRegs) -> i32 {
    /* Returning an int makes 0xffffffff be interpreted as -1 on a 64-bit kernel. */
    if trap_is_syscall(regs) { (*regs).gpr[0] as i32 } else { -1 }
}

#[inline]
pub unsafe fn syscall_set_nr(_task: *mut task_struct, regs: *mut PtRegs, nr: i32) {
    (*regs).gpr[0] = nr as c_ulong;
}

#[inline]
pub unsafe fn syscall_rollback(_task: *mut task_struct, regs: *mut PtRegs) {
    (*regs).gpr[3] = (*regs).orig_gpr3;
}

#[inline]
pub unsafe fn syscall_get_error(_task: *mut task_struct, regs: *mut PtRegs) -> c_long {
    if trap_is_scv(regs) {
        let error = (*regs).gpr[3];
        if error >= (!0usize - 4095) { error as c_long } else { 0 }
    } else {
        if ((*regs).ccr & 0x10000000usize) != 0 {
            ((*regs).gpr[3] as c_long).wrapping_neg()
        } else { 0 }
    }
}

#[inline]
pub unsafe fn syscall_get_return_value(_task: *mut task_struct, regs: *mut PtRegs) -> c_long {
    (*regs).gpr[3] as c_long
}

#[inline]
pub unsafe fn syscall_set_return_value(_task: *mut task_struct, regs: *mut PtRegs, error: i32, val: c_long) {
    if trap_is_scv(regs) {
        (*regs).gpr[3] = if error != 0 { error as c_long } else { val } as c_ulong;
    } else if error != 0 {
        (*regs).ccr |= 0x10000000usize;
        (*regs).gpr[3] = error as c_ulong;
    } else {
        (*regs).ccr &= !0x10000000usize;
        (*regs).gpr[3] = val as c_ulong;
    }
    set_thread_flag(_TIF_SYSCALL_RET);
}

#[inline]
pub unsafe fn syscall_get_arguments(_task: *mut task_struct, regs: *mut PtRegs, args: *mut c_ulong) {
    let mask = if is_tsk_32bit_task(_task) { 0xffff_ffffusize } else { !0usize };
    let mut n = 6usize;
    while n != 0 {
        n -= 1;
        let val = if n == 0 { (*regs).orig_gpr3 } else { (*regs).gpr[3 + n] };
        *args.add(n) = val & mask;
    }
}

#[inline]
pub unsafe fn syscall_set_arguments(_task: *mut task_struct, regs: *mut PtRegs, args: *const c_ulong) {
    core::ptr::copy_nonoverlapping(args, (*regs).gpr.as_mut_ptr().add(3), 6);
    (*regs).orig_gpr3 = *args;
}

#[inline]
pub unsafe fn syscall_get_arch(task: *mut task_struct) -> i32 {
    if is_tsk_32bit_task(task) { _AUDIT_ARCH_PPC } else {
        /* IS_ENABLED(CONFIG_CPU_LITTLE_ENDIAN) */
        #[cfg(CONFIG_CPU_LITTLE_ENDIAN)]
        { _AUDIT_ARCH_PPC64LE }
        #[cfg(not(CONFIG_CPU_LITTLE_ENDIAN))]
        { _AUDIT_ARCH_PPC64 }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
