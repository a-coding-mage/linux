/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

/* CONFIG_COMPAT */
#[cfg(feature = "CONFIG_COMPAT")]
#[repr(C)]
pub struct compat_sigcontext {
    /* We always set these two fields to 0 */
    pub trap_no: compat_ulong_t,
    pub error_code: compat_ulong_t,

    pub oldmask: compat_ulong_t,
    pub arm_r0: compat_ulong_t,
    pub arm_r1: compat_ulong_t,
    pub arm_r2: compat_ulong_t,
    pub arm_r3: compat_ulong_t,
    pub arm_r4: compat_ulong_t,
    pub arm_r5: compat_ulong_t,
    pub arm_r6: compat_ulong_t,
    pub arm_r7: compat_ulong_t,
    pub arm_r8: compat_ulong_t,
    pub arm_r9: compat_ulong_t,
    pub arm_r10: compat_ulong_t,
    pub arm_fp: compat_ulong_t,
    pub arm_ip: compat_ulong_t,
    pub arm_sp: compat_ulong_t,
    pub arm_lr: compat_ulong_t,
    pub arm_pc: compat_ulong_t,
    pub arm_cpsr: compat_ulong_t,
    pub fault_address: compat_ulong_t,
}

#[cfg(feature = "CONFIG_COMPAT")]
#[repr(C, align(8))]
pub struct compat_ucontext {
    pub uc_flags: compat_ulong_t,
    pub uc_link: compat_uptr_t,
    pub uc_stack: compat_stack_t,
    pub uc_mcontext: compat_sigcontext,
    pub uc_sigmask: compat_sigset_t,
    pub __unused: [core::ffi::c_int; 32 - (core::mem::size_of::<compat_sigset_t>() / core::mem::size_of::<core::ffi::c_int>())],
    pub uc_regspace: [compat_ulong_t; 128],
}

#[cfg(feature = "CONFIG_COMPAT")]
#[repr(C)]
pub struct compat_sigframe {
    pub uc: compat_ucontext,
    pub retcode: [compat_ulong_t; 2],
}

#[cfg(feature = "CONFIG_COMPAT")]
#[repr(C)]
pub struct compat_rt_sigframe {
    pub info: compat_siginfo,
    pub sig: compat_sigframe,
}

#[cfg(feature = "CONFIG_COMPAT")]
unsafe extern "C" {
    pub fn compat_setup_frame(
        usig: core::ffi::c_int,
        ksig: *mut ksignal,
        set: *mut sigset_t,
        regs: *mut pt_regs,
    ) -> core::ffi::c_int;
    pub fn compat_setup_rt_frame(
        usig: core::ffi::c_int,
        ksig: *mut ksignal,
        set: *mut sigset_t,
        regs: *mut pt_regs,
    ) -> core::ffi::c_int;
    pub fn compat_setup_restart_syscall(regs: *mut pt_regs);
}

#[cfg(not(feature = "CONFIG_COMPAT"))]
#[inline]
pub unsafe fn compat_setup_frame(
    _usid: core::ffi::c_int,
    _ksig: *mut ksignal,
    _set: *mut sigset_t,
    _regs: *mut pt_regs,
) -> core::ffi::c_int {
    -ENOSYS
}

#[cfg(not(feature = "CONFIG_COMPAT"))]
#[inline]
pub unsafe fn compat_setup_rt_frame(
    _usig: core::ffi::c_int,
    _ksig: *mut ksignal,
    _set: *mut sigset_t,
    _regs: *mut pt_regs,
) -> core::ffi::c_int {
    -ENOSYS
}

#[cfg(not(feature = "CONFIG_COMPAT"))]
#[inline]
pub unsafe fn compat_setup_restart_syscall(_regs: *mut pt_regs) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
