/* SPDX-License-Identifier: GPL-2.0 */
/*
 * GCC stack protector support.
 *
 * (This is directly adopted from the ARM implementation)
 *
 * Stack protector works by putting predefined pattern at the start of
 * the stack frame and verifying that it hasn't been overwritten when
 * returning from the function.  The pattern is called stack canary
 * and gcc expects it to be defined by a global variable called
 * "__stack_chk_guard" on MIPS.  This unfortunately means that on SMP
 * we cannot have a different canary value per task.
 */

// The C header guard `_ASM_STACKPROTECTOR_H` is represented by this Rust
// translation unit's normal single-definition semantics.

unsafe extern "C" {
    pub static mut __stack_chk_guard: libc::c_ulong;
    pub fn get_random_canary() -> libc::c_ulong;
}

/*
 * Initialize the stackprotector canary value.
 *
 * NOTE: this must only be called from functions that never return,
 * and it must always be inlined.
 */
#[inline(always)]
pub unsafe fn boot_init_stack_canary() {
    let canary: libc::c_ulong = unsafe { get_random_canary() };

    // `current->stack_canary = canary;` requires the kernel's external
    // `current` task pointer and its task_struct definition.
    // TODO: assign `canary` to current.stack_canary at the integration site.
    unsafe {
        __stack_chk_guard = canary;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
