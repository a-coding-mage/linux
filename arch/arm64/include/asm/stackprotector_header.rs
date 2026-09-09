/* SPDX-License-Identifier: GPL-2.0 */
/*
 * GCC stack protector support.
 *
 * Stack protector works by putting predefined pattern at the start of
 * the stack frame and verifying that it hasn't been overwritten when
 * returning from the function.  The pattern is called stack canary
 * and gcc expects it to be defined by a global variable called
 * "__stack_chk_guard" on ARM.  This unfortunately means that on SMP
 * we cannot have a different canary value per task.
 */

use core::ffi::c_ulong;

// Dependency supplied by asm/pointer_auth.h and the surrounding kernel.
extern "C" {
    pub static mut __stack_chk_guard: c_ulong;
    pub fn get_random_canary() -> c_ulong;
    pub fn ptrauth_thread_init_kernel(current: *mut TaskStruct);
    pub fn ptrauth_thread_switch_kernel(current: *mut TaskStruct);
    pub fn ptrauth_enable();
    pub static mut current: *mut TaskStruct;
}

// Opaque dependency shape required by this header's field access.
#[repr(C)]
pub struct TaskStruct {
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
    // CONFIG_STACKPROTECTOR is a build-time condition from the original C header.
    #[cfg(feature = "CONFIG_STACKPROTECTOR")]
    {
        let canary: c_ulong = get_random_canary();

        (*current).stack_canary = canary;
        // IS_ENABLED(CONFIG_STACKPROTECTOR_PER_TASK) is a build-time condition.
        #[cfg(not(feature = "CONFIG_STACKPROTECTOR_PER_TASK"))]
        {
            __stack_chk_guard = (*current).stack_canary;
        }
    }
    ptrauth_thread_init_kernel(current);
    ptrauth_thread_switch_kernel(current);
    ptrauth_enable();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
