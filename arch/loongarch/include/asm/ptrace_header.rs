/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

/*
 * This struct defines the way the registers are stored on the stack during
 * a system call/exception. If you add a register here, please also add it to
 * regoffset_table[] in arch/loongarch/kernel/ptrace.c.
 */
#[repr(C, align(8))]
pub struct pt_regs {
    /* Main processor registers. */
    pub regs: [usize; 32],

    /* Original syscall arg0. */
    pub orig_a0: usize,

    /* Special CSR registers. */
    pub csr_era: usize,
    pub csr_badvaddr: usize,
    pub csr_crmd: usize,
    pub csr_prmd: usize,
    pub csr_euen: usize,
    pub csr_ecfg: usize,
    pub csr_estat: usize,
    pub __last: [usize; 0],
}

#[inline(always)]
pub unsafe fn regs_irqs_disabled(regs: *mut pt_regs) -> bool {
    unsafe { ((*regs).csr_prmd & CSR_PRMD_PIE) == 0 }
}

#[inline]
pub unsafe fn kernel_stack_pointer(regs: *mut pt_regs) -> usize {
    unsafe { (*regs).regs[3] }
}

/*
 * Don't use asm-generic/ptrace.h it defines FP accessors that don't make
 * sense on LoongArch.  We rather want an error if they get invoked.
 */

#[inline]
pub unsafe fn instruction_pointer_set(regs: *mut pt_regs, val: usize) {
    unsafe { (*regs).csr_era = val; }
}

/* Query offset/name of register from its name/offset */
unsafe extern "C" {
    pub fn regs_query_register_offset(name: *const core::ffi::c_char) -> i32;
}

pub const MAX_REG_OFFSET: usize = core::mem::offset_of!(pt_regs, __last) - core::mem::size_of::<usize>();

/**
 * regs_get_register() - get register value from its offset
 * @regs:       pt_regs from which register value is gotten.
 * @offset:     offset number of the register.
 *
 * regs_get_register returns the value of a register. The @offset is the
 * offset of the register in struct pt_regs address which specified by @regs.
 * If @offset is bigger than MAX_REG_OFFSET, this returns 0.
 */
#[inline]
pub unsafe fn regs_get_register(regs: *mut pt_regs, offset: u32) -> usize {
    if offset as usize > MAX_REG_OFFSET {
        return 0;
    }
    unsafe { *((regs as *mut u8).add(offset as usize) as *mut usize) }
}

/**
 * regs_within_kernel_stack() - check the address in the stack
 * @regs:       pt_regs which contains kernel stack pointer.
 * @addr:       address which is checked.
 */
#[inline]
pub unsafe fn regs_within_kernel_stack(regs: *mut pt_regs, addr: usize) -> i32 {
    unsafe {
        ((addr & !(THREAD_SIZE - 1)) == (kernel_stack_pointer(regs) & !(THREAD_SIZE - 1))) as i32
    }
}

/**
 * regs_get_kernel_stack_nth() - get Nth entry of the stack
 * @regs:       pt_regs which contains kernel stack pointer.
 * @n:          stack entry number.
 */
#[inline]
pub unsafe fn regs_get_kernel_stack_nth(regs: *mut pt_regs, n: u32) -> usize {
    let addr = unsafe { (kernel_stack_pointer(regs) as *mut usize).add(n as usize) };
    if unsafe { regs_within_kernel_stack(regs, addr as usize) } != 0 {
        unsafe { *addr }
    } else {
        0
    }
}

pub struct task_struct;

#[inline]
pub unsafe fn regs_get_kernel_argument(regs: *mut pt_regs, mut n: u32) -> usize {
    const NR_REG_ARGUMENTS: u32 = 8;
    let args: [usize; 8] = [
        core::mem::offset_of!(pt_regs, regs[4]),
        core::mem::offset_of!(pt_regs, regs[5]),
        core::mem::offset_of!(pt_regs, regs[6]),
        core::mem::offset_of!(pt_regs, regs[7]),
        core::mem::offset_of!(pt_regs, regs[8]),
        core::mem::offset_of!(pt_regs, regs[9]),
        core::mem::offset_of!(pt_regs, regs[10]),
        core::mem::offset_of!(pt_regs, regs[11]),
    ];
    if n < NR_REG_ARGUMENTS {
        unsafe { regs_get_register(regs, args[n as usize] as u32) }
    } else {
        n -= NR_REG_ARGUMENTS;
        unsafe { regs_get_kernel_stack_nth(regs, n) }
    }
}

/* Does the process account for user or for system time? */
#[inline]
pub unsafe fn user_mode(regs: *mut pt_regs) -> bool {
    unsafe { ((*regs).csr_prmd & PLV_MASK) == PLV_USER }
}

#[inline]
pub unsafe fn regs_return_value(regs: *mut pt_regs) -> isize {
    unsafe { (*regs).regs[4] as isize }
}

#[inline]
pub unsafe fn regs_set_return_value(regs: *mut pt_regs, val: usize) {
    unsafe { (*regs).regs[4] = val; }
}

#[inline]
pub unsafe fn instruction_pointer(regs: *mut pt_regs) -> usize {
    unsafe { (*regs).csr_era }
}

#[inline]
pub unsafe fn profile_pc(regs: *mut pt_regs) -> usize {
    unsafe { instruction_pointer(regs) }
}

unsafe extern "C" {
    pub fn die(str_: *const core::ffi::c_char, regs: *mut pt_regs);
}

#[inline]
pub unsafe fn die_if_kernel(str_: *const core::ffi::c_char, regs: *mut pt_regs) {
    if unsafe { !user_mode(regs) } {
        unsafe { die(str_, regs); }
    }
}

#[inline]
pub unsafe fn current_pt_regs() -> *mut pt_regs {
    unsafe {
        (((current_stack_pointer | (THREAD_SIZE - 1)) + 1) as *mut pt_regs).sub(1)
    }
}

/* Helpers for working with the user stack pointer */
#[inline]
pub unsafe fn user_stack_pointer(regs: *mut pt_regs) -> usize {
    unsafe { (*regs).regs[3] }
}

#[inline]
pub unsafe fn user_stack_pointer_set(regs: *mut pt_regs, val: usize) {
    unsafe { (*regs).regs[3] = val; }
}

// CONFIG_HAVE_HW_BREAKPOINT conditionally provides this architecture helper.
#[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
#[inline]
pub const fn arch_has_single_step() -> i32 { 1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
