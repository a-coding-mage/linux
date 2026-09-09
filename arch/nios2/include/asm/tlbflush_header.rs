/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2010 Tobias Klauser <tklauser@distanz.ch>
 */

// C dependency: struct mm_struct;
#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

/*
 * TLB flushing:
 *
 *  - flush_tlb_all() flushes all processes TLB entries
 *  - flush_tlb_mm(mm) flushes the specified mm context TLB entries
 *  - flush_tlb_range(vma, start, end) flushes a range of pages
 *  - flush_tlb_page(vma, address) flushes a page
 *  - flush_tlb_kernel_range(start, end) flushes a range of kernel pages
 *  - flush_tlb_kernel_page(address) flushes a kernel page
 *
 *  - reload_tlb_page(vma, address, pte) flushes the TLB for address like
 *    flush_tlb_page, then replaces it with a TLB for pte.
 */
unsafe extern "C" {
    pub fn flush_tlb_all();
    pub fn flush_tlb_mm(mm: *mut mm_struct);
    // C dependency: struct vm_area_struct;
    pub fn flush_tlb_range(
        vma: *mut vm_area_struct,
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
    );
    pub fn flush_tlb_kernel_range(
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
    );
    pub fn reload_tlb_page(
        vma: *mut vm_area_struct,
        addr: ::core::ffi::c_ulong,
        pte: pte_t,
    );
}

// C dependency: struct vm_area_struct;
#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

// C dependency: pte_t.
pub type pte_t = ::core::ffi::c_ulong;

// C dependency: PAGE_SIZE.
unsafe extern "C" {
    static PAGE_SIZE: ::core::ffi::c_ulong;
}

#[inline]
pub unsafe fn flush_tlb_page(vma: *mut vm_area_struct, address: ::core::ffi::c_ulong) {
    flush_tlb_range(vma, address, address.wrapping_add(PAGE_SIZE));
}

#[inline]
pub unsafe fn flush_tlb_kernel_page(address: ::core::ffi::c_ulong) {
    flush_tlb_kernel_range(address, address.wrapping_add(PAGE_SIZE));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
