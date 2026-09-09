/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

/* C dependencies: asm/ptrace.h, sysdep/archsetjmp.h, linux/prefetch.h,
 * and asm/cpufeatures.h provide the referenced types and constants. */

use core::ffi::{c_ulong, c_void};

#[repr(C)]
pub struct pt_regs;

#[repr(C)]
pub struct task_struct;

#[repr(C)]
pub struct mm_struct;

#[repr(C)]
pub struct arch_thread;

#[repr(C)]
pub struct jmp_buf;

#[repr(C)]
pub struct thread_struct {
    pub segv_regs: *mut pt_regs,
    pub prev_sched: *mut task_struct,
    pub arch: arch_thread,
    pub switch_buf: jmp_buf,
    pub request: thread_struct_request_outer,
    pub segv_continue: *mut c_void,
    /* Contains variable sized FP registers */
    pub regs: pt_regs,
}

#[repr(C)]
pub struct thread_struct_request_outer {
    pub thread: thread_struct_request_thread,
}

#[repr(C)]
pub struct thread_struct_request_thread {
    pub proc: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub arg: *mut c_void,
}

/*
 * C initializer equivalent; EMPTY_REGS and INIT_ARCH_THREAD are supplied by
 * their respective dependencies.
 */
#[macro_export]
macro_rules! INIT_THREAD {
    () => {
        $crate::thread_struct {
            regs: EMPTY_REGS,
            prev_sched: core::ptr::null_mut(),
            arch: INIT_ARCH_THREAD,
            request: $crate::thread_struct_request_outer {
                thread: $crate::thread_struct_request_thread {
                    proc: None,
                    arg: core::ptr::null_mut(),
                },
            },
            segv_regs: core::ptr::null_mut(),
            switch_buf: jmp_buf {},
            segv_continue: core::ptr::null_mut(),
        }
    };
}

/*
 * User space process size: 3GB (default).
 */
extern "C" {
    pub static mut task_size: c_ulong;
}

pub const TASK_SIZE: c_ulong = unsafe { task_size };

/* STACK_TOP and STACK_TOP_MAX are redefined here for this architecture. */
extern "C" {
    pub static mut stacksizelim: c_ulong;
}

pub const STACK_ROOM: c_ulong = unsafe { stacksizelim };
pub const STACK_TOP: c_ulong = TASK_SIZE - 2 * PAGE_SIZE;
pub const STACK_TOP_MAX: c_ulong = STACK_TOP;

/* This decides where the kernel will search for a free chunk of vm
 * space during mmap's.
 */
pub const TASK_UNMAPPED_BASE: c_ulong = 0x40000000;

extern "C" {
    pub fn start_thread(regs: *mut pt_regs, entry: c_ulong, stack: c_ulong);
}

#[repr(C)]
pub union cpuinfo_um_capability {
    pub x86_capability: [u32; NCAPINTS + NBUGINTS],
    pub x86_capability_alignment: c_ulong,
}

#[repr(C)]
pub struct cpuinfo_um {
    pub loops_per_jiffy: c_ulong,
    pub cache_alignment: i32,
    pub capability: cpuinfo_um_capability,
}

extern "C" {
    pub static mut boot_cpu_data: cpuinfo_um;
}

#[inline]
pub unsafe fn cache_line_size() -> i32 {
    boot_cpu_data.cache_alignment
}

/* C macro: get_thread_reg(reg, &tsk->thread.switch_buf) */
#[macro_export]
macro_rules! KSTK_REG {
    ($tsk:expr, $reg:expr) => {
        get_thread_reg($reg, &mut (*$tsk).thread.switch_buf)
    };
}

extern "C" {
    pub fn __get_wchan(p: *mut task_struct) -> c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
