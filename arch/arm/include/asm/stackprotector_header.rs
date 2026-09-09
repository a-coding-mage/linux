/* SPDX-License-Identifier: GPL-2.0 */
/*
 * GCC stack protector support.
 *
 * Stack protector works by putting predefined pattern at the start of
 * the stack frame and verifying that it hasn't been overwritten when
 * returning from the function.  The pattern is called stack canary
 * and gcc expects it to be defined by a global variable called
 * "__stack_chk_guard" on ARM.  This prevents SMP systems from using a
 * different value for each task unless we enable a GCC plugin that
 * replaces these symbol references with references to each task's own
 * value.
 */

use core::ffi::c_ulong;

/* Supplied by the thread-info dependency. */
extern "C" {
    pub static mut __stack_chk_guard: c_ulong;
    pub fn get_random_canary() -> c_ulong;
    pub static mut current: *mut ThreadInfo;
}

/* The complete definition is supplied by asm/thread_info.h. */
#[repr(C)]
pub struct ThreadInfo {
    pub stack_canary: c_ulong,
}

/*
 * Initialize the stackprotector canary value.
 *
 * NOTE: this must only be called from functions that never return,
 * and it must always be inlined.
 */
#[inline(always)]
pub unsafe fn boot_init_stack_canary() {
    let canary: c_ulong = get_random_canary();

    (*current).stack_canary = canary;
    /* CONFIG_STACKPROTECTOR_PER_TASK controls this assignment at build time. */
    #[cfg(not(feature = "CONFIG_STACKPROTECTOR_PER_TASK"))]
    {
        __stack_chk_guard = (*current).stack_canary;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
