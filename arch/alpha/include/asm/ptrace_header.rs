/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations from <uapi/asm/ptrace.h> are supplied by
// the surrounding translation unit.

#[inline]
pub const fn arch_has_single_step() -> i32 {
    1
}

#[inline]
pub unsafe fn user_mode(regs: *const pt_regs) -> bool {
    ((*regs).ps & 8) != 0
}

#[inline]
pub unsafe fn instruction_pointer(regs: *const pt_regs) -> _ {
    (*regs).pc
}

#[inline]
pub unsafe fn profile_pc(regs: *const pt_regs) -> _ {
    instruction_pointer(regs)
}

#[inline]
pub unsafe fn current_user_stack_pointer() -> _ {
    rdusp()
}

#[inline]
pub unsafe fn task_pt_regs(task: *mut task_struct) -> *mut pt_regs {
    ((task_stack_page(task) as *mut u8).add(2 * PAGE_SIZE) as *mut pt_regs).sub(1)
}

#[inline]
pub unsafe fn current_pt_regs() -> *mut pt_regs {
    ((current_thread_info() as *mut u8).add(2 * PAGE_SIZE) as *mut pt_regs).sub(1)
}

#[inline]
pub unsafe fn force_successful_syscall_return() {
    (*current_pt_regs()).r0 = 0;
}

#[inline]
pub unsafe fn regs_return_value(regs: *const pt_regs) -> _ {
    (*regs).r0
}

/* Helpers for working with the user stack pointer */
#[inline]
pub unsafe fn user_stack_pointer(regs: *const pt_regs) -> _ {
    /* Valid for user-mode regs */
    (*regs).usp
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
