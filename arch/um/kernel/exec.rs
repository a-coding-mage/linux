// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addto,linux.intel}.com)
 */

// Dependencies supplied by the surrounding kernel/UML translation.

#[repr(C)]
pub struct pt_regs {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct thread_struct {
    pub arch: arch_thread,
}

#[repr(C)]
pub struct arch_thread {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub thread: thread_struct,
}

extern "C" {
    fn arch_flush_thread(arch: *mut arch_thread);
    fn current_pt_regs() -> *mut pt_regs;
    fn current() -> *mut task_struct;
    fn get_safe_registers(gp: *mut core::ffi::c_void, fp: *mut core::ffi::c_void);
    fn clear_thread_flag(flag: core::ffi::c_int);
    fn pt_regs_gp(regs: *mut pt_regs) -> *mut core::ffi::c_void;
    fn pt_regs_fp(regs: *mut pt_regs) -> *mut core::ffi::c_void;
    fn pt_regs_ip(regs: *mut pt_regs) -> *mut c_ulong;
    fn pt_regs_sp(regs: *mut pt_regs) -> *mut c_ulong;
    static TIF_SINGLESTEP: core::ffi::c_int;
}

// PT_REGS_IP, PT_REGS_SP, TIF_SINGLESTEP, and the register layout are supplied
// by the architecture-specific dependencies.

pub unsafe fn flush_thread() {
    arch_flush_thread(&mut (*current()).thread.arch);

    get_safe_registers(pt_regs_gp(current_pt_regs()), pt_regs_fp(current_pt_regs()));
}

pub unsafe fn start_thread(regs: *mut pt_regs, eip: c_ulong, esp: c_ulong) {
    *pt_regs_ip(regs) = eip;
    *pt_regs_sp(regs) = esp;
    clear_thread_flag(TIF_SINGLESTEP);
}

pub type c_ulong = core::ffi::c_ulong;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
