/* SPDX-License-Identifier: GPL-2.0 */

// External declaration from the translated dependencies.
unsafe extern "C" {
    pub fn kaslr_get_random_long(purpose: *const core::ffi::c_char) -> core::ffi::c_ulong;
}

#[cfg(CONFIG_RANDOMIZE_MEMORY)]
unsafe extern "C" {
    pub fn kernel_randomize_memory();
    pub fn init_trampoline_kaslr();
}

#[cfg(not(CONFIG_RANDOMIZE_MEMORY))]
#[inline]
pub fn kernel_randomize_memory() {}

#[cfg(not(CONFIG_RANDOMIZE_MEMORY))]
#[inline]
pub fn init_trampoline_kaslr() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
