/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2008-2009 PetaLogix
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

// C header dependencies are supplied by the surrounding translation unit:
// linux/sched.h, linux/threads.h, asm/processor.h, asm/mmu.h, and asm/page.h.

extern "C" {
    pub fn _tlbie(address: ::core::ffi::c_ulong);
    pub fn _tlbia();
    pub fn preempt_disable();
    pub fn preempt_enable();
}

#[inline(always)]
pub unsafe fn __tlbia() {
    preempt_disable();
    _tlbia();
    preempt_enable();
}

#[inline(always)]
pub unsafe fn __tlbie(x: ::core::ffi::c_ulong) {
    _tlbie(x);
}

#[inline]
pub unsafe fn local_flush_tlb_all() {
    __tlbia();
}

#[inline]
pub unsafe fn local_flush_tlb_mm(_mm: *mut mm_struct) {
    __tlbia();
}

#[inline]
pub unsafe fn local_flush_tlb_page(
    _vma: *mut vm_area_struct,
    vmaddr: ::core::ffi::c_ulong,
) {
    __tlbie(vmaddr);
}

#[inline]
pub unsafe fn local_flush_tlb_range(
    _vma: *mut vm_area_struct,
    _start: ::core::ffi::c_ulong,
    _end: ::core::ffi::c_ulong,
) {
    __tlbia();
}

// #define flush_tlb_kernel_range(start, end) do { } while (0)
#[inline]
pub fn flush_tlb_kernel_range(
    _start: ::core::ffi::c_ulong,
    _end: ::core::ffi::c_ulong,
) {
}

// #define update_mmu_cache_range(vmf, vma, addr, ptep, nr) do { } while (0)
#[inline]
pub fn update_mmu_cache_range(
    _vmf: *mut vm_fault,
    _vma: *mut vm_area_struct,
    _addr: ::core::ffi::c_ulong,
    _ptep: *mut pte_t,
    _nr: ::core::ffi::c_ulong,
) {
}

// The original macro invokes update_mmu_cache_range(NULL, vma, addr, ptep, 1).
#[macro_export]
macro_rules! update_mmu_cache {
    ($vma:expr, $addr:expr, $pte:expr) => {
        $crate::update_mmu_cache_range(
            core::ptr::null_mut(),
            $vma,
            $addr,
            ptep,
            1,
        )
    };
}

// #define flush_tlb_all local_flush_tlb_all
// #define flush_tlb_mm local_flush_tlb_mm
// #define flush_tlb_page local_flush_tlb_page
// #define flush_tlb_range local_flush_tlb_range
pub use local_flush_tlb_all as flush_tlb_all;
pub use local_flush_tlb_mm as flush_tlb_mm;
pub use local_flush_tlb_page as flush_tlb_page;
pub use local_flush_tlb_range as flush_tlb_range;

/*
 * This is called in munmap when we have freed up some page-table
 * pages.  We don't need to do anything here, there's nothing special
 * about our page-table pages.  -- paulus
 */
#[inline]
pub fn flush_tlb_pgtables(
    _mm: *mut mm_struct,
    _start: ::core::ffi::c_ulong,
    _end: ::core::ffi::c_ulong,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
