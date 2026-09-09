/*
 * GCC stack protector support.
 *
 * (This is directly adopted from the ARM implementation)
 *
 * Stack protector works by putting predefined pattern at the start of
 * the stack frame and verifying that it hasn't been overwritten when
 * returning from the function.  The pattern is called stack canary
 * and gcc expects it to be defined by a global variable called
 * "__stack_chk_guard" on Xtensa.  This unfortunately means that on SMP
 * we cannot have a different canary value per task.
 */

// Header guard: _ASM_STACKPROTECTOR_H

use core::ffi::c_ulong;

extern "C" {
    pub static mut __stack_chk_guard: c_ulong;
    pub fn get_random_canary() -> c_ulong;
}

#[repr(C)]
pub struct Current {
    pub stack_canary: c_ulong,
}

extern "C" {
    pub static mut current: Current;
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

    current.stack_canary = canary;
    __stack_chk_guard = current.stack_canary;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
