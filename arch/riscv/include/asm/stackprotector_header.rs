/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _ASM_RISCV_STACKPROTECTOR_H

extern "C" {
    pub static mut __stack_chk_guard: ::core::ffi::c_ulong;
    pub fn get_random_canary() -> ::core::ffi::c_ulong;
}

/*
 * Initialize the stackprotector canary value.
 *
 * NOTE: this must only be called from functions that never return,
 * and it must always be inlined.
 *
 * `current` and `IS_ENABLED` are supplied by the surrounding kernel
 * translation/dependencies.
 */
#[inline(always)]
pub unsafe fn boot_init_stack_canary() {
    let canary: ::core::ffi::c_ulong = get_random_canary();

    // TODO: translate the external `current` task pointer and its
    // `stack_canary` field when that dependency is available.
    current.stack_canary = canary;
    if !IS_ENABLED(CONFIG_STACKPROTECTOR_PER_TASK) {
        __stack_chk_guard = current.stack_canary;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
