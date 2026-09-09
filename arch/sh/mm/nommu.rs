// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/sh/mm/nommu.c
 *
 * Various helper routines and stubs for MMUless SH.
 *
 * Copyright (C) 2002 - 2009 Paul Mundt
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kernel.h, linux/init.h, linux/string.h, linux/mm.h,
// asm/cacheflush.h, asm/tlbflush.h, asm/page.h, linux/uaccess.h

/*
 * Nothing too terribly exciting here ..
 */
pub unsafe fn copy_page(to: *mut core::ffi::c_void, from: *mut core::ffi::c_void) {
    core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, PAGE_SIZE);
}

pub unsafe fn __copy_user(
    to: *mut core::ffi::c_void,
    from: *const core::ffi::c_void,
    n: usize,
) -> usize {
    core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, n);
    0
}

pub unsafe fn __clear_user(to: *mut core::ffi::c_void, n: usize) -> usize {
    core::ptr::write_bytes(to as *mut u8, 0, n);
    0
}

pub unsafe fn local_flush_tlb_all() {
    BUG();
}

pub unsafe fn local_flush_tlb_mm(_mm: *mut mm_struct) {
    BUG();
}

pub unsafe fn local_flush_tlb_range(
    _vma: *mut vm_area_struct,
    _start: c_ulong,
    _end: c_ulong,
) {
    BUG();
}

pub unsafe fn local_flush_tlb_page(_vma: *mut vm_area_struct, _page: c_ulong) {
    BUG();
}

pub unsafe fn local_flush_tlb_one(_asid: c_ulong, _page: c_ulong) {
    BUG();
}

pub unsafe fn local_flush_tlb_kernel_range(_start: c_ulong, _end: c_ulong) {
    BUG();
}

pub unsafe fn __flush_tlb_global() {}

pub unsafe fn __update_tlb(_vma: *mut vm_area_struct, _address: c_ulong, _pte: pte_t) {}

pub unsafe fn kmap_coherent_init() {}

pub unsafe fn kmap_coherent(_page: *mut page, _addr: c_ulong) -> *mut core::ffi::c_void {
    BUG();
    core::ptr::null_mut()
}

pub unsafe fn kunmap_coherent(_kvaddr: *mut core::ffi::c_void) {
    BUG();
}

pub unsafe fn page_table_range_init(
    _start: c_ulong,
    _end: c_ulong,
    _pgd_base: *mut pgd_t,
) {
}

pub unsafe fn __set_fixmap(_idx: fixed_addresses, _phys: c_ulong, _prot: pgprot_t) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
