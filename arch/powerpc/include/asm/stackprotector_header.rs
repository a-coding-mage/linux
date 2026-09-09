/* SPDX-License-Identifier: GPL-2.0 */
/*
 * GCC stack protector support.
 *
 */

// Dependencies supplied by the corresponding architecture headers.
extern "C" {
    fn get_random_canary() -> usize;
    fn current() -> *mut Current;
    fn get_paca() -> *mut Paca;
}

#[repr(C)]
pub struct Current {
    pub stack_canary: usize,
}

#[repr(C)]
pub struct Paca {
    pub canary: usize,
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

    (*current()).stack_canary = canary;
    // CONFIG_PPC64 build-time condition from the original header.
    #[cfg(CONFIG_PPC64)]
    {
        (*get_paca()).canary = canary;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
