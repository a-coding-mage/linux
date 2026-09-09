// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2025 - Google Inc
 * Author: Mostafa Saleh <smostafa@google.com>
 * IOMMU API debug page alloc sanitizer
 */

// C dependency: CONFIG_IOMMU_DEBUG_PAGEALLOC

#[cfg(feature = "CONFIG_IOMMU_DEBUG_PAGEALLOC")]
extern "C" {
    pub static iommu_debug_initialized: StaticKeyFalse;

    pub static page_iommu_debug_ops: page_ext_operations;

    pub fn __iommu_debug_check_unmapped(page: *const page, numpages: i32);
}

#[cfg(feature = "CONFIG_IOMMU_DEBUG_PAGEALLOC")]
#[inline]
pub unsafe fn iommu_debug_check_unmapped(page: *const page, numpages: i32) {
    if static_branch_unlikely(&iommu_debug_initialized) {
        __iommu_debug_check_unmapped(page, numpages);
    }
}

#[cfg(not(feature = "CONFIG_IOMMU_DEBUG_PAGEALLOC"))]
#[inline]
pub unsafe fn iommu_debug_check_unmapped(_page: *const page, _numpages: i32) {}

// Opaque declarations supplied by other translation units.
#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct page_ext_operations {
    _private: [u8; 0],
}

#[repr(C)]
pub struct StaticKeyFalse {
    _private: [u8; 0],
}

extern "C" {
    pub fn static_branch_unlikely(key: *const StaticKeyFalse) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
