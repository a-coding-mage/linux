/* SPDX-License-Identifier: GPL-2.0 */

// CONFIG_X86_32 is a build-time condition corresponding to the source header.
#[cfg(CONFIG_X86_32)]
extern "C" {
    pub fn doublefault_init_cpu_tss();
}

#[cfg(not(CONFIG_X86_32))]
#[inline]
pub fn doublefault_init_cpu_tss() {}

// asmlinkage; __noreturn
extern "C" {
    pub fn doublefault_shim() -> !;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
