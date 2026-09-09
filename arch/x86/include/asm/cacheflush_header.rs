/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: linux/mm.h
// Caches aren't brain-dead on the intel.
// Dependency: asm-generic/cacheflush.h
// Dependency: asm/special_insns.h

unsafe extern "C" {
    pub fn clflush_cache_range(addr: *mut core::ffi::c_void, size: core::ffi::c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
