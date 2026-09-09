/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Access to user system call parameters and results
 *
 * Copyright (C) 2008-2009 Red Hat, Inc.  All rights reserved.
 *
 * See asm-generic/syscall.h for descriptions of what we must do here.
 */

/* C dependencies: linux/audit.h, linux/sched.h, linux/err.h,
 * asm/thread_info.h, and asm/unistd.h. */

pub type SysCallPtrT = unsafe extern "C" fn(regs: *const PtRegs) -> libc::c_long;
pub static mut sys_call_table: *const SysCallPtrT = core::ptr::null();

/* Only the low 32 bits of orig_ax are meaningful, so we return int. */
pub unsafe fn syscall_get_nr(_task: *mut TaskStruct, regs: *mut PtRegs) -> libc::c_int {
    (*regs).orig_ax as libc::c_int
}

pub unsafe fn syscall_set_nr(_task: *mut TaskStruct, regs: *mut PtRegs, nr: libc::c_int) {
    (*regs).orig_ax = nr as _;
}

pub unsafe fn syscall_rollback(_task: *mut TaskStruct, regs: *mut PtRegs) {
    (*regs).ax = (*regs).orig_ax;
}

pub unsafe fn syscall_get_error(task: *mut TaskStruct, regs: *mut PtRegs) -> libc::c_long {
    let mut error = (*regs).ax as libc::c_ulong;
    #[cfg(feature = "CONFIG_IA32_EMULATION")]
    {
        /* TS_COMPAT is set for 32-bit syscall entries and remains set until
         * we return to user mode. */
        if ((*task).thread_info.status & (TS_COMPAT | TS_I386_REGS_POKED)) != 0 {
            error = (error as libc::c_int) as libc::c_long as libc::c_ulong;
        }
    }
    if is_err_value(error) { error as libc::c_long } else { 0 }
}

pub unsafe fn syscall_get_return_value(_task: *mut TaskStruct, regs: *mut PtRegs) -> libc::c_long {
    (*regs).ax as libc::c_long
}

pub unsafe fn syscall_set_return_value(
    _task: *mut TaskStruct, regs: *mut PtRegs, error: libc::c_int, val: libc::c_long,
) {
    (*regs).ax = if error != 0 { error as libc::c_long } else { val } as _;
}

#[cfg(feature = "CONFIG_X86_32")]
pub unsafe fn syscall_get_arguments(_task: *mut TaskStruct, regs: *mut PtRegs, args: *mut libc::c_ulong) {
    *args.add(0) = (*regs).bx;
    *args.add(1) = (*regs).cx;
    *args.add(2) = (*regs).dx;
    *args.add(3) = (*regs).si;
    *args.add(4) = (*regs).di;
    *args.add(5) = (*regs).bp;
}

#[cfg(feature = "CONFIG_X86_32")]
pub unsafe fn syscall_set_arguments(_task: *mut TaskStruct, regs: *mut PtRegs, args: *const libc::c_ulong) {
    (*regs).bx = *args.add(0);
    (*regs).cx = *args.add(1);
    (*regs).dx = *args.add(2);
    (*regs).si = *args.add(3);
    (*regs).di = *args.add(4);
    (*regs).bp = *args.add(5);
}

#[cfg(feature = "CONFIG_X86_32")]
pub unsafe fn syscall_get_arch(_task: *mut TaskStruct) -> libc::c_int { AUDIT_ARCH_I386 }

#[cfg(not(feature = "CONFIG_X86_32"))]
pub unsafe fn syscall_get_arguments(task: *mut TaskStruct, regs: *mut PtRegs, args: *mut libc::c_ulong) {
    #[cfg(feature = "CONFIG_IA32_EMULATION")]
    if ((*task).thread_info.status & TS_COMPAT) != 0 {
        *args.add(0) = (*regs).bx; *args.add(1) = (*regs).cx; *args.add(2) = (*regs).dx;
        *args.add(3) = (*regs).si; *args.add(4) = (*regs).di; *args.add(5) = (*regs).bp;
        return;
    }
    *args.add(0) = (*regs).di; *args.add(1) = (*regs).si; *args.add(2) = (*regs).dx;
    *args.add(3) = (*regs).r10; *args.add(4) = (*regs).r8; *args.add(5) = (*regs).r9;
}

#[cfg(not(feature = "CONFIG_X86_32"))]
pub unsafe fn syscall_set_arguments(task: *mut TaskStruct, regs: *mut PtRegs, args: *const libc::c_ulong) {
    #[cfg(feature = "CONFIG_IA32_EMULATION")]
    if ((*task).thread_info.status & TS_COMPAT) != 0 {
        (*regs).bx = *args.add(0); (*regs).cx = *args.add(1); (*regs).dx = *args.add(2);
        (*regs).si = *args.add(3); (*regs).di = *args.add(4); (*regs).bp = *args.add(5);
        return;
    }
    (*regs).di = *args.add(0); (*regs).si = *args.add(1); (*regs).dx = *args.add(2);
    (*regs).r10 = *args.add(3); (*regs).r8 = *args.add(4); (*regs).r9 = *args.add(5);
}

#[cfg(not(feature = "CONFIG_X86_32"))]
pub unsafe fn syscall_get_arch(task: *mut TaskStruct) -> libc::c_int {
    #[cfg(feature = "CONFIG_IA32_EMULATION")]
    { if ((*task).thread_info.status & TS_COMPAT) != 0 { return AUDIT_ARCH_I386; } }
    AUDIT_ARCH_X86_64
}

#[cfg(not(feature = "CONFIG_X86_32"))]
extern "C" {
    pub fn do_syscall_64(regs: *mut PtRegs, nr: libc::c_long) -> bool;
    pub fn do_int80_emulation(regs: *mut PtRegs);
}

extern "C" {
    pub fn do_int80_syscall_32(regs: *mut PtRegs);
    pub fn do_fast_syscall_32(regs: *mut PtRegs) -> bool;
    pub fn do_SYSENTER_32(regs: *mut PtRegs) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
