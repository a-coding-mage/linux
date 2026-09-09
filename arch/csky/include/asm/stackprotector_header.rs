/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard `_ASM_STACKPROTECTOR_H` has no executable Rust equivalent.

extern "C" {
    pub static mut __stack_chk_guard: usize;
    pub fn get_random_canary() -> usize;
}

// The `current` task pointer and its concrete task type are supplied by the
// surrounding kernel dependencies; this layout names the field used here.
#[repr(C)]
pub struct Current {
    pub stack_canary: usize,
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
    let canary: usize = get_random_canary();

    current.stack_canary = canary;
    __stack_chk_guard = current.stack_canary;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
