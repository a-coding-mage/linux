/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2008-2009 PetaLogix
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

// C header dependencies: asm/ptrace.h, asm/setup.h, asm/registers.h,
// asm/entry.h, and asm/current.h.

// from kernel/cpu/mb.c
extern "C" {
    pub static cpuinfo_op: seq_operations;
}

#[inline(always)]
pub unsafe fn cpu_relax() {
    barrier();
}

#[inline(always)]
pub unsafe fn task_pt_regs(tsk: *mut task_struct) -> *mut pt_regs {
    ((THREAD_SIZE + task_stack_page(tsk) as usize) as *mut pt_regs).offset(-1)
}

/* Do necessary setup to start up a newly executed thread. */
extern "C" {
    pub fn start_thread(regs: *mut pt_regs, pc: c_ulong, usp: c_ulong);
    pub fn ret_from_fork();
    pub fn ret_from_kernel_thread();
}

/*
 * This is used to define STACK_TOP, and with MMU it must be below
 * kernel base to select the correct PGD when handling MMU exceptions.
 */
pub const TASK_SIZE: usize = CONFIG_KERNEL_START;

/*
 * This decides where the kernel will search for a free chunk of vm
 * space during mmap's.
 */
pub const TASK_UNMAPPED_BASE: usize = TASK_SIZE / 8 * 3;

pub const THREAD_KSP: usize = 0;

/* If you change this, you must change the associated assembly-languages
 * constants defined below, THREAD_*.
 */
#[repr(C)]
pub struct thread_struct {
    /* kernel stack pointer (must be first field in structure) */
    pub ksp: c_ulong,
    pub ksp_limit: c_ulong, /* if ksp <= ksp_limit stack overflow */
    pub pgdir: *mut c_void, /* root of page-table tree */
    pub regs: *mut pt_regs, /* Pointer to saved register state */
}

/* Equivalent to the C designated initializer INIT_THREAD. */
pub unsafe fn init_thread() -> thread_struct {
    thread_struct {
        ksp: core::mem::size_of_val(&init_stack) as c_ulong + init_stack as c_ulong,
        ksp_limit: 0,
        pgdir: swapper_pg_dir,
        regs: core::ptr::null_mut(),
    }
}

extern "C" {
    pub fn __get_wchan(p: *mut task_struct) -> c_ulong;
}

/* The size allocated for kernel stacks. This _must_ be a power of two! */
pub const KERNEL_STACK_SIZE: usize = 0x2000;

/* Return some info about the user process TASK.  */
#[inline(always)]
pub unsafe fn task_tos(task: *mut c_void) -> c_ulong {
    task as c_ulong + KERNEL_STACK_SIZE as c_ulong
}

#[inline(always)]
pub unsafe fn task_regs(task: *mut c_void) -> *mut pt_regs {
    (task_tos(task) as *mut pt_regs).offset(-1)
}

#[inline(always)]
pub unsafe fn task_pt_regs_plus_args(tsk: *mut task_struct) -> *mut c_void {
    task_pt_regs(tsk) as *mut c_void
}

#[inline(always)]
pub unsafe fn task_sp(task: *mut c_void) -> c_ulong {
    (*task_regs(task)).r1
}

#[inline(always)]
pub unsafe fn task_pc(task: *mut c_void) -> c_ulong {
    (*task_regs(task)).pc
}

/* Grotty old names for some.  */
#[inline(always)]
pub unsafe fn KSTK_EIP(task: *mut c_void) -> c_ulong {
    task_pc(task)
}

#[inline(always)]
pub unsafe fn KSTK_ESP(task: *mut c_void) -> c_ulong {
    task_sp(task)
}

pub const STACK_TOP: usize = TASK_SIZE;
pub const STACK_TOP_MAX: usize = STACK_TOP;

#[cfg(CONFIG_DEBUG_FS)]
extern "C" {
    pub static mut of_debugfs_root: *mut dentry;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
