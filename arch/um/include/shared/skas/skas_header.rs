/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// C header guard: __SKAS_H
// C dependency: <sysdep/ptrace.h>

// Forward declarations supplied by the C dependency.
#[repr(C)]
pub struct uml_pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mm_id {
    _private: [u8; 0],
}

extern "C" {
    pub static mut using_seccomp: ::core::ffi::c_int;

    pub fn new_thread_handler();
    pub fn handle_syscall(regs: *mut uml_pt_regs);
    pub fn current_stub_stack() -> ::core::ffi::c_ulong;
    pub fn current_mm_id() -> *mut mm_id;
    pub fn current_mm_sync();
    pub fn initial_jmpbuf_lock();
    pub fn initial_jmpbuf_unlock();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
