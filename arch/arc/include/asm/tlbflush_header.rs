/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Dependency supplied by the surrounding translation unit: linux/mm.h

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

extern "C" {
    pub fn local_flush_tlb_all();
    pub fn local_flush_tlb_mm(mm: *mut mm_struct);
    pub fn local_flush_tlb_page(vma: *mut vm_area_struct, page: usize);
    pub fn local_flush_tlb_kernel_range(start: usize, end: usize);
    pub fn local_flush_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize);

    // CONFIG_TRANSPARENT_HUGEPAGE
    #[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
    pub fn local_flush_pmd_tlb_range(
        vma: *mut vm_area_struct,
        start: usize,
        end: usize,
    );
}

// When CONFIG_SMP is not enabled, these C macros alias the local operations.
#[cfg(not(feature = "CONFIG_SMP"))]
pub unsafe fn flush_tlb_range(vma: *mut vm_area_struct, s: usize, e: usize) {
    local_flush_tlb_range(vma, s, e)
}

#[cfg(not(feature = "CONFIG_SMP"))]
pub unsafe fn flush_tlb_page(vma: *mut vm_area_struct, page: usize) {
    local_flush_tlb_page(vma, page)
}

#[cfg(not(feature = "CONFIG_SMP"))]
pub unsafe fn flush_tlb_kernel_range(s: usize, e: usize) {
    local_flush_tlb_kernel_range(s, e)
}

#[cfg(not(feature = "CONFIG_SMP"))]
pub unsafe fn flush_tlb_all() {
    local_flush_tlb_all()
}

#[cfg(not(feature = "CONFIG_SMP"))]
pub unsafe fn flush_tlb_mm(mm: *mut mm_struct) {
    local_flush_tlb_mm(mm)
}

// CONFIG_TRANSPARENT_HUGEPAGE
#[cfg(all(not(feature = "CONFIG_SMP"), feature = "CONFIG_TRANSPARENT_HUGEPAGE"))]
pub unsafe fn flush_pmd_tlb_range(vma: *mut vm_area_struct, s: usize, e: usize) {
    local_flush_pmd_tlb_range(vma, s, e)
}

// When CONFIG_SMP is enabled, these operations are externally supplied.
#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub fn flush_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize);
    pub fn flush_tlb_page(vma: *mut vm_area_struct, page: usize);
    pub fn flush_tlb_kernel_range(start: usize, end: usize);
    pub fn flush_tlb_all();
    pub fn flush_tlb_mm(mm: *mut mm_struct);

    // CONFIG_TRANSPARENT_HUGEPAGE
    #[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
    pub fn flush_pmd_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
