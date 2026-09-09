/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from <uapi/asm/ptrace.h>, <asm/traps.h>, linux/types.h,
 * and linux/compiler.h. These dependencies are supplied elsewhere. */

#[cfg(not(assembler))]
pub const PS_S: u32 = 0x8000_0000; /* Supervisor Mode */

#[cfg(not(assembler))]
pub const USR_BKPT: u32 = 0x1464;

#[cfg(not(assembler))]
#[inline]
pub const fn arch_has_single_step() -> i32 { 1 }

#[cfg(not(assembler))]
#[inline]
pub unsafe fn current_pt_regs() -> *mut pt_regs {
    // C: ((struct pt_regs *)((char *)current_thread_info() + THREAD_SIZE) - 1)
    ((current_thread_info() as *mut u8).add(THREAD_SIZE) as *mut pt_regs).sub(1)
}

#[cfg(not(assembler))]
#[inline]
pub unsafe fn user_stack_pointer(regs: *const pt_regs) -> _ {
    (*regs).usp
}

#[cfg(not(assembler))]
#[inline]
pub unsafe fn user_mode(regs: *const pt_regs) -> bool {
    ((*regs).sr & PS_S as _) == 0
}

#[cfg(not(assembler))]
#[inline]
pub unsafe fn instruction_pointer(regs: *const pt_regs) -> _ { (*regs).pc }

#[cfg(not(assembler))]
#[inline]
pub unsafe fn profile_pc(regs: *const pt_regs) -> _ { instruction_pointer(regs) }

#[cfg(not(assembler))]
#[inline]
pub unsafe fn trap_no(regs: *const pt_regs) -> _ { ((*regs).sr >> 16) & 0xff }

#[cfg(not(assembler))]
#[inline]
pub unsafe fn instruction_pointer_set(regs: *mut pt_regs, val: _) {
    (*regs).pc = val;
}

/* MAX_REG_OFFSET depends on the target ABI, as in the original conditional. */
#[cfg(all(not(assembler), cskyabiv2))]
pub const MAX_REG_OFFSET: usize = core::mem::offset_of!(pt_regs, dcsr);
#[cfg(all(not(assembler), not(cskyabiv2)))]
pub const MAX_REG_OFFSET: usize = core::mem::offset_of!(pt_regs, regs[9]);

#[cfg(not(assembler))]
#[inline]
pub unsafe fn in_syscall(regs: *const pt_regs) -> bool {
    (((*regs).sr >> 16) & 0xff) == VEC_TRAP0
}

#[cfg(not(assembler))]
#[inline]
pub unsafe fn forget_syscall(regs: *mut pt_regs) {
    (*regs).sr &= !(0xff << 16);
}

#[cfg(not(assembler))]
#[inline]
pub unsafe fn regs_return_value(regs: *const pt_regs) -> _ { (*regs).a0 }

#[cfg(not(assembler))]
#[inline]
pub unsafe fn regs_set_return_value(regs: *mut pt_regs, val: _) {
    (*regs).a0 = val;
}

/* Valid only for Kernel mode traps. */
#[cfg(not(assembler))]
#[inline]
pub unsafe fn kernel_stack_pointer(regs: *const pt_regs) -> _ { (*regs).usp }

#[cfg(not(assembler))]
#[inline]
pub unsafe fn frame_pointer(regs: *const pt_regs) -> _ { (*regs).regs[4] }

#[cfg(not(assembler))]
#[inline]
pub unsafe fn frame_pointer_set(regs: *mut pt_regs, val: _) {
    (*regs).regs[4] = val;
}

extern "C" {
    pub fn regs_query_register_offset(name: *const core::ffi::c_char) -> i32;
    pub fn regs_get_kernel_stack_nth(regs: *mut pt_regs, n: u32) -> usize;
}

/*
 * regs_get_register() - get register value from its offset
 * @regs:      pt_regs from which register value is gotten
 * @offset:    offset of the register.
 *
 * regs_get_register returns the value of a register whose offset from @regs.
 * The @offset is the offset of the register in struct pt_regs.
 * If @offset is bigger than MAX_REG_OFFSET, this returns 0.
 */
#[cfg(not(assembler))]
#[inline]
pub unsafe fn regs_get_register(regs: *mut pt_regs, offset: u32) -> usize {
    if offset as usize > MAX_REG_OFFSET {
        return 0;
    }
    *((regs as *mut u8).add(offset as usize) as *mut usize)
}

extern "C" {
    pub fn syscall_trace_enter(regs: *mut pt_regs) -> i32;
    pub fn syscall_trace_exit(regs: *mut pt_regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
