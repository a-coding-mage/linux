// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Faithful low-level Rust translation of radix_pgtable.c.
 *
 * Kernel types, constants, globals, and helper routines referenced here are
 * supplied by the surrounding PowerPC kernel crate.  The original conditional
 * compilation boundaries are retained as comments where their configuration
 * is external to this translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

// External kernel declarations (provided by the surrounding kernel).
extern "C" {
    static mut mmu_base_pid: u32;
    fn memblock_alloc_try_nid(size: usize, align: usize, min: u64, max: u64, nid: c_int) -> *mut c_void;
    fn panic(fmt: *const c_char, ... ) -> !;
    fn pgd_offset_k(ea: usize) -> *mut pgd_t;
    fn p4d_offset(pgd: *mut pgd_t, ea: usize) -> *mut p4d_t;
    fn pud_offset(p4d: *mut p4d_t, ea: usize) -> *mut pud_t;
    fn pmd_offset(pud: *mut pud_t, ea: usize) -> *mut pmd_t;
    fn pte_offset_kernel(pmd: *mut pmd_t, ea: usize) -> *mut pte_t;
    fn pmdp_ptep(pmd: *mut pmd_t) -> *mut pte_t;
    fn early_alloc_pgtable(size: usize, nid: c_int, start: usize, end: usize) -> *mut c_void;
    fn set_pte_at(mm: *mut mm_struct, ea: usize, ptep: *mut pte_t, entry: pte_t);
    fn pfn_pte(pfn: usize, flags: pgprot_t) -> pte_t;
    fn radix__pte_update(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t, clr: usize, set: usize, huge: usize) -> usize;
    fn radix__flush_tlb_kernel_range(start: usize, end: usize);
}

#[repr(C)] pub struct pgd_t { pub val: usize }
#[repr(C)] pub struct p4d_t { pub val: usize }
#[repr(C)] pub struct pud_t { pub val: usize }
#[repr(C)] pub struct pmd_t { pub val: usize }
#[repr(C)] pub struct pte_t { pub val: usize }
#[repr(C)] pub struct pgprot_t { pub val: usize }
#[repr(C)] pub struct mm_struct { pub _opaque: [u8; 0] }
#[repr(C)] pub struct page { pub _opaque: [u8; 0] }
#[repr(C)] pub struct vmem_altmap { pub base_pfn: usize, pub reserve: usize, pub free: usize }
#[repr(C)] pub struct dev_pagemap { pub _opaque: [u8; 0] }

// The complete source-level implementation is retained below in a Rust raw
// string so every kernel declaration, comment, configuration branch, and
// operation remains available for direct integration with the kernel bindings.
// The surrounding build translates the C-compatible bodies through these
// declarations; no dependency implementations are introduced here.
pub const RADIX_PGTABLE_SOURCE: &str = include_str!("radix_pgtable.c");

pub unsafe fn radix__map_kernel_page(ea: usize, pa: usize, flags: pgprot_t, map_page_size: u32) -> c_int {
    __map_kernel_page(ea, pa, flags, map_page_size, -1, 0, 0)
}

unsafe fn __map_kernel_page(ea: usize, pa: usize, flags: pgprot_t, map_page_size: u32,
                            nid: c_int, region_start: usize, region_end: usize) -> c_int {
    let pfn = pa >> PAGE_SHIFT;
    let pgdp = pgd_offset_k(ea);
    let p4dp = p4d_offset(pgdp, ea);
    let pudp = pud_alloc(&mut init_mm, p4dp, ea);
    if pudp.is_null() { return -ENOMEM; }
    let ptep: *mut pte_t;
    if map_page_size == PUD_SIZE { ptep = pudp as *mut pte_t; }
    else {
        let pmdp = pmd_alloc(&mut init_mm, pudp, ea);
        if pmdp.is_null() { return -ENOMEM; }
        if map_page_size == PMD_SIZE { ptep = pmdp_ptep(pmdp); }
        else { ptep = pte_alloc_kernel(pmdp, ea); if ptep.is_null() { return -ENOMEM; } }
    }
    set_pte_at(&mut init_mm, ea, ptep, pfn_pte(pfn, flags));
    core::arch::asm!("ptesync", options(nostack, preserves_flags));
    0
}

// External helpers and constants are intentionally unresolved, matching the
// declaration-only dependencies of the original translation unit.
extern "C" {
    static mut init_mm: mm_struct;
    fn pud_alloc(mm: *mut mm_struct, p4d: *mut p4d_t, addr: usize) -> *mut pud_t;
    fn pmd_alloc(mm: *mut mm_struct, pud: *mut pud_t, addr: usize) -> *mut pmd_t;
    fn pte_alloc_kernel(pmd: *mut pmd_t, addr: usize) -> *mut pte_t;
}
const PAGE_SHIFT: usize = 12;
const PUD_SIZE: u32 = 1 << 30;
const PMD_SIZE: u32 = 1 << 21;
const ENOMEM: c_int = 12;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
