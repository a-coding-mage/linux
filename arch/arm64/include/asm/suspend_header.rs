/* SPDX-License-Identifier: GPL-2.0 */

// The original header guard was: __ASM_SUSPEND_H

pub const NR_CTX_REGS: usize = 14;
pub const NR_CALLEE_SAVED_REGS: usize = 12;

/*
 * struct cpu_suspend_ctx must be 16-byte aligned since it is allocated on
 * the stack, which must be 16-byte aligned on v8
 */
#[repr(C, align(16))]
pub struct cpu_suspend_ctx {
    /*
     * This struct must be kept in sync with
     * cpu_do_{suspend/resume} in mm/proc.S
     */
    pub ctx_regs: [u64; NR_CTX_REGS],
    pub sp: u64,
}

/*
 * Memory to save the cpu state is allocated on the stack by
 * __cpu_suspend_enter()'s caller, and populated by __cpu_suspend_enter().
 * This data must survive until cpu_resume() is called.
 *
 * This struct describes the size and the layout of the saved cpu state.
 * The layout of the callee_saved_regs is defined by the implementation
 * of __cpu_suspend_enter(), and cpu_resume(). This struct must be passed
 * in by the caller as __cpu_suspend_enter()'s stack-frame is gone once it
 * returns, and the data would be subsequently corrupted by the call to the
 * finisher.
 */
#[repr(C)]
pub struct sleep_stack_data {
    pub system_regs: cpu_suspend_ctx,
    pub callee_saved_regs: [usize; NR_CALLEE_SAVED_REGS],
}

unsafe extern "C" {
    pub static mut sleep_save_stash: *mut usize;

    pub fn cpu_suspend(arg: usize, fn_: Option<unsafe extern "C" fn(usize) -> i32>) -> i32;
    pub fn cpu_resume();
    pub fn __cpu_suspend_enter(state: *mut sleep_stack_data) -> i32;
    pub fn __cpu_suspend_exit();
    pub fn _cpu_resume();

    pub fn swsusp_arch_suspend() -> i32;
    pub fn swsusp_arch_resume() -> i32;
    pub fn arch_hibernation_header_save(addr: *mut core::ffi::c_void, max_size: u32) -> i32;
    pub fn arch_hibernation_header_restore(addr: *mut core::ffi::c_void) -> i32;

    /* Used to resume on the CPU we hibernated on */
    pub fn hibernate_resume_nonboot_cpu_disable() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
