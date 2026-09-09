/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2001 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// Dependency supplied by the corresponding sysdep/ptrace translation:
// `struct uml_pt_regs`.

#[repr(C)]
pub struct uml_pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub enum um_irq_type {
    IRQ_READ,
    IRQ_WRITE,
    NUM_IRQ_TYPES,
}

#[repr(C)]
pub struct siginfo {
    _private: [u8; 0],
}

#[allow(improper_ctypes)]
extern "C" {
    pub fn sigio_handler(
        sig: ::core::ffi::c_int,
        unused_si: *mut siginfo,
        regs: *mut uml_pt_regs,
        mc: *mut ::core::ffi::c_void,
    );
    pub fn sigchld_handler(
        sig: ::core::ffi::c_int,
        unused_si: *mut siginfo,
        regs: *mut uml_pt_regs,
        mc: *mut ::core::ffi::c_void,
    );
    pub fn sigio_run_timetravel_handlers();
    pub fn free_irq_by_fd(fd: ::core::ffi::c_int);
    pub fn deactivate_fd(fd: ::core::ffi::c_int, irqnum: ::core::ffi::c_int);
    pub fn deactivate_all_fds() -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
