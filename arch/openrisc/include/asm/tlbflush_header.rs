/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 * et al.
 */

// Dependencies supplied by the translated kernel sources.

/*
 *  - flush_tlb() flushes the current mm struct TLBs
 *  - flush_tlb_all() flushes all processes TLBs
 *  - flush_tlb_mm(mm) flushes the specified mm context TLB's
 *  - flush_tlb_page(vma, vmaddr) flushes one page
 *  - flush_tlb_range(vma, start, end) flushes a range of pages
 */
extern "C" {
    pub fn local_flush_tlb_all();
    pub fn local_flush_tlb_mm(mm: *mut mm_struct);
    pub fn local_flush_tlb_page(vma: *mut vm_area_struct, addr: ::core::ffi::c_ulong);
    pub fn local_flush_tlb_range(
        vma: *mut vm_area_struct,
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
    );
}

// The following aliases correspond to the !CONFIG_SMP preprocessor macros.
#[cfg(not(feature = "CONFIG_SMP"))]
pub unsafe fn flush_tlb_all() {
    local_flush_tlb_all()
}

#[cfg(not(feature = "CONFIG_SMP"))]
pub unsafe fn flush_tlb_mm(mm: *mut mm_struct) {
    local_flush_tlb_mm(mm)
}

#[cfg(not(feature = "CONFIG_SMP"))]
pub unsafe fn flush_tlb_page(vma: *mut vm_area_struct, addr: ::core::ffi::c_ulong) {
    local_flush_tlb_page(vma, addr)
}

#[cfg(not(feature = "CONFIG_SMP"))]
pub unsafe fn flush_tlb_range(
    vma: *mut vm_area_struct,
    start: ::core::ffi::c_ulong,
    end: ::core::ffi::c_ulong,
) {
    local_flush_tlb_range(vma, start, end)
}

// CONFIG_SMP selects these externally defined operations.
#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub fn flush_tlb_all();
    pub fn flush_tlb_mm(mm: *mut mm_struct);
    pub fn flush_tlb_page(vma: *mut vm_area_struct, addr: ::core::ffi::c_ulong);
    pub fn flush_tlb_range(
        vma: *mut vm_area_struct,
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
    );
}

pub unsafe fn flush_tlb() {
    flush_tlb_mm((*current()).mm);
}

pub unsafe fn flush_tlb_kernel_range(
    start: ::core::ffi::c_ulong,
    end: ::core::ffi::c_ulong,
) {
    flush_tlb_range(::core::ptr::null_mut(), start, end);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
