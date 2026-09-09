/*
 * Copyright (C) 2013 Altera Corporation
 * Copyright (C) 2010 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2004 Microtronix Datacom Ltd
 *
 * based on m68k asm/processor.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependency: <uapi/asm/ptrace.h>

/* This struct defines the way the registers are stored on the
   stack during a system call.  */
#[repr(C)]
pub struct pt_regs {
    pub r8: usize,      /* r8-r15 Caller-saved GP registers */
    pub r9: usize,
    pub r10: usize,
    pub r11: usize,
    pub r12: usize,
    pub r13: usize,
    pub r14: usize,
    pub r15: usize,
    pub r1: usize,      /* Assembler temporary */
    pub r2: usize,      /* Retval LS 32bits */
    pub r3: usize,      /* Retval MS 32bits */
    pub r4: usize,      /* r4-r7 Register arguments */
    pub r5: usize,
    pub r6: usize,
    pub r7: usize,
    pub orig_r2: usize, /* Copy of r2 ?? */
    pub ra: usize,      /* Return address */
    pub fp: usize,      /* Frame pointer */
    pub sp: usize,      /* Stack pointer */
    pub gp: usize,      /* Global pointer */
    pub estatus: usize,
    pub ea: usize,      /* Exception return address (pc) */
    pub orig_r7: usize,
}

/*
 * This is the extended stack used by signal handlers and the context
 * switcher: it's pushed after the normal "struct pt_regs".
 */
#[repr(C)]
pub struct switch_stack {
    pub r16: usize, /* r16-r23 Callee-saved GP registers */
    pub r17: usize,
    pub r18: usize,
    pub r19: usize,
    pub r20: usize,
    pub r21: usize,
    pub r22: usize,
    pub r23: usize,
    pub fp: usize,
    pub gp: usize,
    pub ra: usize,
}

#[inline]
pub unsafe fn user_mode(regs: *const pt_regs) -> usize {
    (*regs).estatus & ESTATUS_EU
}

#[inline]
pub unsafe fn instruction_pointer(regs: *const pt_regs) -> usize {
    (*regs).ra
}

#[inline]
pub unsafe fn profile_pc(regs: *const pt_regs) -> usize {
    instruction_pointer(regs)
}

#[inline]
pub unsafe fn user_stack_pointer(regs: *const pt_regs) -> usize {
    (*regs).sp
}

unsafe extern "C" {
    pub fn show_regs(regs: *mut pt_regs);
    pub fn do_syscall_trace_enter() -> i32;
    pub fn do_syscall_trace_exit();
}

// Dependency: current_thread_info() and THREAD_SIZE are supplied elsewhere.
#[inline]
pub unsafe fn current_pt_regs() -> *mut pt_regs {
    ((current_thread_info() as usize + THREAD_SIZE) as *mut pt_regs).sub(1)
}

#[inline]
pub unsafe fn force_successful_syscall_return() {
    (*current_pt_regs()).orig_r2 = (-1isize) as usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
