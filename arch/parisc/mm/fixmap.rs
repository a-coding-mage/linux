// SPDX-License-Identifier: GPL-2.0
/*
 * fixmaps for parisc
 *
 * Copyright (c) 2019 Sven Schnelle <svens@stackframe.org>
 */

// Declarations supplied by the corresponding kernel headers.
use core::ffi::c_int;

pub type fixed_addresses = c_int;
pub type phys_addr_t = usize;
pub type ulong = usize;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct pgd_t {
    _private: [u8; 0],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct p4d_t {
    _private: [u8; 0],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct pud_t {
    _private: [u8; 0],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct pmd_t {
    _private: [u8; 0],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct pte_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

extern "C" {
    static mut init_mm: mm_struct;

    fn __fix_to_virt(idx: fixed_addresses) -> ulong;
    fn pgd_offset_k(vaddr: ulong) -> *mut pgd_t;
    fn p4d_offset(pgd: *mut pgd_t, vaddr: ulong) -> *mut p4d_t;
    fn pud_offset(p4d: *mut p4d_t, vaddr: ulong) -> *mut pud_t;
    fn pmd_offset(pud: *mut pud_t, vaddr: ulong) -> *mut pmd_t;
    fn pte_offset_kernel(pmd: *mut pmd_t, vaddr: ulong) -> *mut pte_t;
    fn __mk_pte(phys: phys_addr_t, prot: ulong) -> pte_t;
    fn set_pte_at(mm: *mut mm_struct, vaddr: ulong, pte: *mut pte_t, entry: pte_t);
    fn flush_tlb_kernel_range(start: ulong, end: ulong);
    fn virt_to_kpte(vaddr: ulong) -> *mut pte_t;
    fn pte_none(pte: pte_t) -> bool;
    fn WARN_ON(condition: bool) -> bool;
    fn pte_clear(mm: *mut mm_struct, vaddr: ulong, pte: *mut pte_t);
}

const PAGE_KERNEL_RWX: ulong = 0;
const PAGE_SIZE: ulong = 4096;

pub unsafe fn set_fixmap(idx: fixed_addresses, phys: phys_addr_t) {
    let vaddr: ulong = __fix_to_virt(idx);
    let pgd: *mut pgd_t = pgd_offset_k(vaddr);
    let p4d: *mut p4d_t = p4d_offset(pgd, vaddr);
    let pud: *mut pud_t = pud_offset(p4d, vaddr);
    let pmd: *mut pmd_t = pmd_offset(pud, vaddr);
    let pte: *mut pte_t;

    pte = pte_offset_kernel(pmd, vaddr);
    set_pte_at(&mut init_mm, vaddr, pte, __mk_pte(phys, PAGE_KERNEL_RWX));
    flush_tlb_kernel_range(vaddr, vaddr.wrapping_add(PAGE_SIZE));
}

pub unsafe fn clear_fixmap(idx: fixed_addresses) {
    let vaddr: ulong = __fix_to_virt(idx);
    let pte: *mut pte_t = virt_to_kpte(vaddr);

    if WARN_ON(pte_none(*pte)) {
        return;
    }

    pte_clear(&mut init_mm, vaddr, pte);

    flush_tlb_kernel_range(vaddr, vaddr.wrapping_add(PAGE_SIZE));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
