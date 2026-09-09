/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard is omitted; this file is intended to be included once by
// the surrounding Rust translation unit.

// `unsigned long` from the source header.  Its width follows the target ABI.
pub type CUnsignedLong = usize;

#[repr(C)]
pub struct StackCurrent {
    pub stack_canary: CUnsignedLong,
}

extern "C" {
    pub static mut __stack_chk_guard: CUnsignedLong;
    pub static mut current: *mut StackCurrent;
    pub fn get_random_canary() -> CUnsignedLong;
}

/*
 * Initialize the stackprotector canary value.
 *
 * NOTE: this must only be called from functions that never return,
 * and it must always be inlined.
 */
#[inline(always)]
pub unsafe fn boot_init_stack_canary() {
    let canary: CUnsignedLong = get_random_canary();

    (*current).stack_canary = canary;
    __stack_chk_guard = (*current).stack_canary;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
