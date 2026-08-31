/* SPDX-License-Identifier: GPL-2.0-or-later */

// Depends on declarations from "common.h".

unsafe extern "C" {
    pub fn memblock_alloc_nid_checks() -> core::ffi::c_int;
    pub fn memblock_alloc_exact_nid_range_checks() -> core::ffi::c_int;
    pub fn __memblock_alloc_nid_numa_checks() -> core::ffi::c_int;
}

// C conditional: #ifdef CONFIG_NUMA
#[cfg(CONFIG_NUMA)]
#[inline]
pub unsafe fn memblock_alloc_nid_numa_checks() -> core::ffi::c_int {
    unsafe {
        __memblock_alloc_nid_numa_checks();
    }
    0
}

// C conditional: #else
#[cfg(not(CONFIG_NUMA))]
#[inline]
pub fn memblock_alloc_nid_numa_checks() -> core::ffi::c_int {
    0
}
