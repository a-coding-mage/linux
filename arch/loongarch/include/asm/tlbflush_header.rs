/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependency supplied by the surrounding kernel translation:
// #include <linux/mm.h>
use core::ffi::c_ulong;

/*
 * TLB flushing:
 *
 *  - flush_tlb_all() flushes all processes TLB entries
 *  - flush_tlb_mm(mm) flushes the specified mm context TLB entries
 *  - flush_tlb_page(vma, vmaddr) flushes one page
 *  - flush_tlb_range(vma, start, end) flushes a range of pages
 *  - flush_tlb_kernel_range(start, end) flushes a range of kernel pages
 */
unsafe extern "C" {
    pub fn local_flush_tlb_all();
    pub fn local_flush_tlb_user();
    pub fn local_flush_tlb_kernel();
    pub fn local_flush_tlb_mm(mm: *mut mm_struct);
    pub fn local_flush_tlb_range(
        vma: *mut vm_area_struct,
        start: c_ulong,
        end: c_ulong,
    );
    pub fn local_flush_tlb_kernel_range(start: c_ulong, end: c_ulong);
    pub fn local_flush_tlb_page(vma: *mut vm_area_struct, page: c_ulong);
    pub fn local_flush_tlb_one(vaddr: c_ulong);
}

// CONFIG_SMP selects the externally implemented cross-CPU operations.
#[cfg(feature = "CONFIG_SMP")]
unsafe extern "C" {
    pub fn flush_tlb_all();
    pub fn flush_tlb_mm(mm: *mut mm_struct);
    pub fn flush_tlb_range(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong);
    pub fn flush_tlb_kernel_range(start: c_ulong, end: c_ulong);
    pub fn flush_tlb_page(vma: *mut vm_area_struct, page: c_ulong);
    pub fn flush_tlb_one(vaddr: c_ulong);
}

// !CONFIG_SMP: the C macros alias the public operations to their local forms.
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn flush_tlb_all() {
    local_flush_tlb_all()
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn flush_tlb_mm(mm: *mut mm_struct) {
    local_flush_tlb_mm(mm)
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn flush_tlb_range(vma: *mut vm_area_struct, vmaddr: c_ulong, end: c_ulong) {
    local_flush_tlb_range(vma, vmaddr, end)
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn flush_tlb_kernel_range(vmaddr: c_ulong, end: c_ulong) {
    local_flush_tlb_kernel_range(vmaddr, end)
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn flush_tlb_page(vma: *mut vm_area_struct, page: c_ulong) {
    local_flush_tlb_page(vma, page)
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn flush_tlb_one(vaddr: c_ulong) {
    local_flush_tlb_one(vaddr)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
