/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __ASM_SPARC_CACHETYPE_H
// Dependency from <asm/page.h> is supplied externally.

#[cfg(CONFIG_SPARC32)]
extern "C" {
    pub static mut vac_cache_size: core::ffi::c_int;
}

#[cfg(CONFIG_SPARC32)]
#[inline]
pub unsafe fn cpu_dcache_is_aliasing() -> bool {
    vac_cache_size > PAGE_SIZE
}

#[cfg(not(CONFIG_SPARC32))]
#[inline]
pub const fn cpu_dcache_is_aliasing() -> bool {
    L1DCACHE_SIZE > PAGE_SIZE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
