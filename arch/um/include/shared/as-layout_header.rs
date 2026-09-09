/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

/* Translated from as-layout.h. */
/* Dependency supplied by the generated asm-offsets header. */

/*
 * Stolen from linux/const.h, which can't be directly included since
 * this is used in userspace code, which has no access to the kernel
 * headers.  Changed to be suitable for adding casts to the start,
 * rather than "UL" to the end.
 */

/* Some constant macros are used in both assembler and
 * C code.  Therefore we cannot annotate them always with
 * 'UL' and other type specifiers unilaterally.  We
 * use the following macros to deal with this.
 */

#[macro_export]
macro_rules! STUB_START {
    () => { stub_start };
}

#[macro_export]
macro_rules! STUB_CODE {
    () => { STUB_START!() };
}

#[macro_export]
macro_rules! STUB_DATA {
    () => { (STUB_CODE!() + UM_KERN_PAGE_SIZE) };
}

pub const STUB_DATA_PAGES: usize = 2;

#[macro_export]
macro_rules! STUB_SIZE {
    () => { ((1 + STUB_DATA_PAGES) * UM_KERN_PAGE_SIZE) };
}

#[macro_export]
macro_rules! STUB_END {
    () => { (STUB_START!() + STUB_SIZE!()) };
}

/* Supplied by generated asm-offsets and other translation units. */
extern "C" {
    pub static UM_KERN_PAGE_SIZE: usize;
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    pub static mut cpu_tasks: *mut *mut task_struct;

    pub static mut physmem_size: u64;

    pub static mut high_physmem: usize;
    pub static mut uml_physmem: usize;
    pub static mut uml_reserved: usize;
    pub static mut end_vm: usize;
    pub static mut start_vm: usize;

    pub static mut brk_start: usize;

    pub static mut stub_start: usize;

    pub fn linux_main(argc: i32, argv: *mut *mut i8, envp: *mut *mut i8) -> i32;
    pub fn uml_finishsetup();
}

#[repr(C)]
pub struct siginfo {
    _private: [u8; 0],
}

/* Supplied by sysdep/ptrace.h. */
#[repr(C)]
pub struct uml_pt_regs {
    _private: [u8; 0],
}

extern "C" {
    pub static mut sig_info:
        *mut Option<unsafe extern "C" fn(i32, *mut siginfo, *mut uml_pt_regs, *mut core::ffi::c_void)>;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
