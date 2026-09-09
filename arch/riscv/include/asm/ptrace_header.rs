/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Regents of the University of California
 */

/* Translated from <uapi/asm/ptrace.h>, <asm/csr.h>, and <linux/compiler.h>. */

#[repr(C)]
pub struct pt_regs {
    pub epc: usize,
    pub ra: usize,
    pub sp: usize,
    pub gp: usize,
    pub tp: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub s0: usize,
    pub s1: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    /* Supervisor/Machine CSRs */
    pub status: usize,
    pub badaddr: usize,
    pub cause: usize,
    /* a0 value before the syscall */
    pub orig_a0: usize,
}

pub const PTRACE_SYSEMU: i32 = 0x1f;
pub const PTRACE_SYSEMU_SINGLESTEP: i32 = 0x20;

/* CONFIG_64BIT selects "%016lx"; otherwise it selects "%08lx". */
#[cfg(target_pointer_width = "64")]
pub const REG_FMT: &str = "%016lx";
#[cfg(not(target_pointer_width = "64"))]
pub const REG_FMT: &str = "%08lx";

/* SR_PP is supplied by asm/csr.h. */
#[inline]
pub unsafe fn user_mode(regs: *const pt_regs) -> bool {
    ((*regs).status & SR_PP) == 0
}

pub const MAX_REG_OFFSET: usize = core::mem::offset_of!(pt_regs, orig_a0);

/* Helpers for working with the instruction pointer */
#[inline]
pub unsafe fn instruction_pointer(regs: *mut pt_regs) -> usize {
    (*regs).epc
}

#[inline]
pub unsafe fn instruction_pointer_set(regs: *mut pt_regs, val: usize) {
    (*regs).epc = val;
}

#[inline]
pub unsafe fn profile_pc(regs: *mut pt_regs) -> usize {
    instruction_pointer(regs)
}

/* Helpers for working with the user stack pointer */
#[inline]
pub unsafe fn user_stack_pointer(regs: *mut pt_regs) -> usize {
    (*regs).sp
}

#[inline]
pub unsafe fn user_stack_pointer_set(regs: *mut pt_regs, val: usize) {
    (*regs).sp = val;
}

/* Valid only for Kernel mode traps. */
#[inline]
pub unsafe fn kernel_stack_pointer(regs: *mut pt_regs) -> usize {
    (*regs).sp
}

/* Helpers for working with the frame pointer */
#[inline]
pub unsafe fn frame_pointer(regs: *mut pt_regs) -> usize {
    (*regs).s0
}

#[inline]
pub unsafe fn frame_pointer_set(regs: *mut pt_regs, val: usize) {
    (*regs).s0 = val;
}

#[inline]
pub unsafe fn regs_return_value(regs: *mut pt_regs) -> usize {
    (*regs).a0
}

#[inline]
pub unsafe fn regs_set_return_value(regs: *mut pt_regs, val: usize) {
    (*regs).a0 = val;
}

unsafe extern "C" {
    pub fn regs_query_register_offset(name: *const core::ffi::c_char) -> i32;
    pub fn regs_get_kernel_stack_nth(regs: *mut pt_regs, n: u32) -> usize;
    pub fn prepare_ftrace_return(parent: *mut usize, self_addr: usize, frame_pointer: usize);
}

/**
 * regs_get_register() - get register value from its offset
 * @regs:    pt_regs from which register value is gotten
 * @offset:  offset of the register.
 *
 * regs_get_register returns the value of a register whose offset from @regs.
 * The @offset is the offset of the register in struct pt_regs.
 * If @offset is bigger than MAX_REG_OFFSET, this returns 0.
 */
#[inline]
pub unsafe fn regs_get_register(regs: *mut pt_regs, offset: u32) -> usize {
    if (offset as usize) > MAX_REG_OFFSET {
        return 0;
    }
    *((regs as *mut u8).add(offset as usize) as *mut usize)
}

/**
 * regs_get_kernel_argument() - get Nth function argument in kernel
 * @regs:       pt_regs of that context
 * @n:          function argument number (start from 0)
 *
 * regs_get_argument() returns @n th argument of the function call.
 *
 * Note you can get the parameter correctly if the function has no
 * more than eight arguments.
 */
#[inline]
pub unsafe fn regs_get_kernel_argument(regs: *mut pt_regs, n: u32) -> usize {
    const NR_REG_ARGUMENTS: u32 = 8;
    const ARGUMENT_OFFS: [u32; 8] = [
        core::mem::offset_of!(pt_regs, a0) as u32,
        core::mem::offset_of!(pt_regs, a1) as u32,
        core::mem::offset_of!(pt_regs, a2) as u32,
        core::mem::offset_of!(pt_regs, a3) as u32,
        core::mem::offset_of!(pt_regs, a4) as u32,
        core::mem::offset_of!(pt_regs, a5) as u32,
        core::mem::offset_of!(pt_regs, a6) as u32,
        core::mem::offset_of!(pt_regs, a7) as u32,
    ];
    if n < NR_REG_ARGUMENTS {
        regs_get_register(regs, ARGUMENT_OFFS[n as usize])
    } else {
        0
    }
}

#[inline(always)]
pub unsafe fn regs_irqs_disabled(regs: *mut pt_regs) -> bool {
    !((*regs).status & SR_PIE != 0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
