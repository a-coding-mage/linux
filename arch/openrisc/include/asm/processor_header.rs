/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 * et al.
 */

// The original header includes asm/spr_defs.h, asm/page.h, and asm/ptrace.h.

pub const STACK_TOP: usize = TASK_SIZE;
pub const STACK_TOP_MAX: usize = STACK_TOP;

/* Kernel and user SR register setting. */
pub const KERNEL_SR: u32 = SPR_SR_DME | SPR_SR_IME | SPR_SR_ICE | SPR_SR_DCE | SPR_SR_SM;
pub const USER_SR: u32 =
    SPR_SR_DME | SPR_SR_IME | SPR_SR_ICE | SPR_SR_DCE | SPR_SR_IEE | SPR_SR_TEE;

/* SR bits user space may change via sigreturn, the rest stay kernel owned. */
pub const SPR_SR_USER_MASK: u32 = SPR_SR_F | SPR_SR_CY | SPR_SR_OV;

/*
 * User space process size. This is hardcoded into a few places,
 * so don't change it unless you know what you are doing.
 */
pub const TASK_SIZE: usize = 0x8000_0000usize;

/* This decides where the kernel will search for a free chunk of vm
 * space during mmap's.
 */
pub const TASK_UNMAPPED_BASE: usize = TASK_SIZE / 8 * 3;

pub struct task_struct;

#[repr(C)]
pub struct thread_struct {
    /* Floating point control status register. */
    pub fpcsr: libc::c_long,
}

/*
 * At user->kernel entry, the pt_regs struct is stacked on the top of the
 * kernel-stack.  This function allows us to find those regs for a task.
 * Notice that subsequent pt_regs stackings, like recursive interrupts
 * occurring while we're in the kernel, won't affect this - only the first
 * user->kernel transition registers are reached by this (i.e. not regs
 * for running signal handler)
 */
#[inline]
pub unsafe fn user_regs(thread_info: *mut core::ffi::c_void) -> *mut pt_regs {
    (((thread_info as usize) + THREAD_SIZE - STACK_FRAME_OVERHEAD) as *mut pt_regs).sub(1)
}

/* Dito but for the currently running task. */
#[inline]
pub unsafe fn task_pt_regs(task: *mut task_struct) -> *mut pt_regs {
    user_regs(task_thread_info(task))
}

pub unsafe fn init_sp() -> usize {
    core::mem::size_of_val(&init_stack) + (&init_stack as *const _ as usize)
}

/* INIT_THREAD is an empty struct initializer in the C header. */
pub const INIT_THREAD: () = ();

#[inline]
pub unsafe fn kstk_eip(tsk: *mut task_struct) -> usize {
    (*task_pt_regs(tsk)).pc
}

#[inline]
pub unsafe fn kstk_esp(tsk: *mut task_struct) -> usize {
    (*task_pt_regs(tsk)).sp
}

unsafe extern "C" {
    pub fn start_thread(regs: *mut pt_regs, nip: usize, sp: usize);
    pub fn __get_wchan(p: *mut task_struct) -> usize;
    pub fn show_registers(regs: *mut pt_regs);
    pub fn barrier();
}

#[inline]
pub unsafe fn cpu_relax() {
    barrier();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
