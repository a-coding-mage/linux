/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/asm-alpha/processor.h
 *
 * Copyright (C) 1994 Linus Torvalds
 */

/*
 * We have a 42-bit user address space: 4TB user VM...
 */
pub const TASK_SIZE: u64 = 0x40000000000u64;

pub const STACK_TOP: u64 = 0x00120000000u64;

pub const STACK_TOP_MAX: u64 = 0x00120000000u64;

/* This decides where the kernel will search for a free chunk of vm
 * space during mmap's.
 */
pub const TASK_UNMAPPED_BASE: u64 = TASK_SIZE / 2;

/* This is dead.  Everything has been moved to thread_info.  */
#[repr(C)]
pub struct thread_struct {}

#[macro_export]
macro_rules! INIT_THREAD {
    () => { thread_struct {} };
}

/* Do necessary setup to start up a newly executed thread.  */
#[repr(C)]
pub struct pt_regs;

unsafe extern "C" {
    pub fn start_thread(regs: *mut pt_regs, new_pc: u64, new_sp: u64);
}

/* Free all resources held by a thread. */
#[repr(C)]
pub struct task_struct;

unsafe extern "C" {
    pub fn __get_wchan(p: *mut task_struct) -> u64;
}

#[macro_export]
macro_rules! KSTK_EIP {
    ($tsk:expr) => { task_pt_regs($tsk).pc };
}

#[macro_export]
macro_rules! KSTK_ESP {
    ($tsk:expr) => {
        if ($tsk) == current {
            rdusp()
        } else {
            task_thread_info($tsk).pcb.usp
        }
    };
}

#[macro_export]
macro_rules! cpu_relax {
    () => { barrier() };
}

/* These declarations indicate architecture support supplied elsewhere. */
pub const ARCH_HAS_PREFETCH: () = ();
pub const ARCH_HAS_PREFETCHW: () = ();

pub unsafe fn prefetch(ptr: *const core::ffi::c_void) {
    /* Equivalent to __builtin_prefetch(ptr, 0, 3); supplied by the target/compiler. */
    unsafe { __builtin_prefetch(ptr, 0, 3) };
}

pub unsafe fn prefetchw(ptr: *const core::ffi::c_void) {
    /* Equivalent to __builtin_prefetch(ptr, 1, 3); supplied by the target/compiler. */
    unsafe { __builtin_prefetch(ptr, 1, 3) };
}

unsafe extern "C" {
    fn __builtin_prefetch(ptr: *const core::ffi::c_void, rw: i32, locality: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
