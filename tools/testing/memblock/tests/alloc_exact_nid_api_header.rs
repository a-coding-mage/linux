/* SPDX-License-Identifier: GPL-2.0-or-later */

/* Depends on declarations from "common.h" in the original C header. */

use core::ffi::c_int;

unsafe extern "C" {
    pub fn memblock_alloc_exact_nid_checks() -> c_int;
    pub fn __memblock_alloc_exact_nid_numa_checks() -> c_int;
}

/* CONFIG_NUMA */
#[cfg(CONFIG_NUMA)]
#[inline]
pub unsafe fn memblock_alloc_exact_nid_numa_checks() -> c_int {
    unsafe {
        __memblock_alloc_exact_nid_numa_checks();
    }
    0
}

/* !CONFIG_NUMA */
#[cfg(not(CONFIG_NUMA))]
#[inline]
pub unsafe fn memblock_alloc_exact_nid_numa_checks() -> c_int {
    0
}
