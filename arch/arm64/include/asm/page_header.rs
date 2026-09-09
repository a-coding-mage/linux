/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Based on arch/arm/include/asm/page.h
 *
 * Copyright (C) 1995-2003 Russell King
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependency intent: symbols from asm/page-def.h, linux/personality.h,
// linux/types.h, asm/pgtable-types.h, asm/memory.h, and asm-generic/getorder.h
// are supplied by other translated units.

use core::ffi::c_void;

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct folio {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn copy_page(to: *mut c_void, from: *const c_void);
    pub fn clear_page(to: *mut c_void);

    pub fn copy_user_highpage(
        to: *mut page,
        from: *mut page,
        vaddr: u64,
        vma: *mut vm_area_struct,
    );

    pub fn copy_highpage(to: *mut page, from: *mut page);

    pub fn vma_alloc_zeroed_movable_folio(
        vma: *mut vm_area_struct,
        vaddr: u64,
    ) -> *mut folio;

    pub fn tag_clear_highpages(to: *mut page, numpages: i32, clear_pages: bool) -> bool;

    pub fn pfn_is_map_memory(pfn: u64) -> i32;
}

// __HAVE_ARCH_COPY_USER_HIGHPAGE
pub const __HAVE_ARCH_COPY_USER_HIGHPAGE: bool = true;

// __HAVE_ARCH_COPY_HIGHPAGE
pub const __HAVE_ARCH_COPY_HIGHPAGE: bool = true;

// vma_alloc_zeroed_movable_folio is an alias for the function declaration.
pub use vma_alloc_zeroed_movable_folio as vma_alloc_zeroed_movable_folio_alias;

pub const __HAVE_ARCH_TAG_CLEAR_HIGHPAGES: bool = true;

pub type pgtable_t = *mut page;

#[inline(always)]
pub unsafe fn copy_user_page(
    to: *mut c_void,
    from: *const c_void,
    _vaddr: u64,
    _pg: *mut page,
) {
    unsafe { copy_page(to, from) }
}

// Build-time condition preserved from CONFIG_ARM64_MTE.
#[cfg(feature = "CONFIG_ARM64_MTE")]
#[macro_export]
macro_rules! VMA_DATA_DEFAULT_FLAGS {
    () => {
        append_vma_flags(VMA_DATA_FLAGS_TSK_EXEC, VMA_MTE_ALLOWED_BIT)
    };
}

#[cfg(not(feature = "CONFIG_ARM64_MTE"))]
#[macro_export]
macro_rules! VMA_DATA_DEFAULT_FLAGS {
    () => {
        VMA_DATA_FLAGS_TSK_EXEC
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
