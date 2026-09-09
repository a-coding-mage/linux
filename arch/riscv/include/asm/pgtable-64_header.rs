/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2012 Regents of the University of California */

// Dependencies supplied by other headers/build configurations are intentionally
// referenced here and not implemented in this translation unit.

extern "C" {
    pub static mut pgtable_l4_enabled: bool;
    pub static mut pgtable_l5_enabled: bool;
}

pub const PGDIR_SHIFT_L3: u32 = 30;
pub const PGDIR_SHIFT_L4: u32 = 39;
pub const PGDIR_SHIFT_L5: u32 = 48;

#[inline]
pub unsafe fn PGDIR_SHIFT() -> u32 {
    if pgtable_l5_enabled { PGDIR_SHIFT_L5 } else if pgtable_l4_enabled { PGDIR_SHIFT_L4 } else { PGDIR_SHIFT_L3 }
}
#[inline]
pub unsafe fn PGDIR_SIZE() -> usize { 1usize << PGDIR_SHIFT() }
#[inline]
pub unsafe fn PGDIR_MASK() -> usize { !(PGDIR_SIZE() - 1) }

pub const P4D_SHIFT_L3: u32 = 30;
pub const P4D_SHIFT_L4: u32 = 39;
pub const P4D_SHIFT_L5: u32 = 39;
#[inline]
pub unsafe fn P4D_SHIFT() -> u32 {
    if pgtable_l5_enabled { P4D_SHIFT_L5 } else if pgtable_l4_enabled { P4D_SHIFT_L4 } else { P4D_SHIFT_L3 }
}
#[inline]
pub unsafe fn P4D_SIZE() -> usize { 1usize << P4D_SHIFT() }
#[inline]
pub unsafe fn P4D_MASK() -> usize { !(P4D_SIZE() - 1) }

pub const PUD_SHIFT: u32 = 30;
pub const PUD_SIZE: usize = 1usize << PUD_SHIFT;
pub const PUD_MASK: usize = !(PUD_SIZE - 1);
pub const PMD_SHIFT: u32 = 21;
pub const PMD_SIZE: usize = 1usize << PMD_SHIFT;
pub const PMD_MASK: usize = !(PMD_SIZE - 1);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p4d_t { pub p4d: usize }
#[inline] pub const fn p4d_val(x: p4d_t) -> usize { x.p4d }
#[inline] pub const fn __p4d(x: usize) -> p4d_t { p4d_t { p4d: x } }
#[inline] pub unsafe fn PTRS_PER_P4D() -> usize { PAGE_SIZE / core::mem::size_of::<p4d_t>() }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pud_t { pub pud: usize }
#[inline] pub const fn pud_val(x: pud_t) -> usize { x.pud }
#[inline] pub const fn __pud(x: usize) -> pud_t { pud_t { pud: x } }
#[inline] pub unsafe fn PTRS_PER_PUD() -> usize { PAGE_SIZE / core::mem::size_of::<pud_t>() }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmd_t { pub pmd: usize }
#[inline] pub const fn pmd_val(x: pmd_t) -> usize { x.pmd }
#[inline] pub const fn __pmd(x: usize) -> pmd_t { pmd_t { pmd: x } }
#[inline] pub unsafe fn PTRS_PER_PMD() -> usize { PAGE_SIZE / core::mem::size_of::<pmd_t>() }

pub const MAX_POSSIBLE_PHYSMEM_BITS: u32 = 56;
pub const _PAGE_PFN_MASK: u64 = ((1u64 << 44) - 1) << 10;
pub const _PAGE_NAPOT_SHIFT: u32 = 63;
pub const _PAGE_NAPOT: u64 = 1u64 << _PAGE_NAPOT_SHIFT;
pub const NAPOT_CONT_ORDER_BASE: i32 = 4;
pub const NAPOT_CONT64KB_ORDER: i32 = NAPOT_CONT_ORDER_BASE;
pub const NAPOT_ORDER_MAX: i32 = NAPOT_CONT_ORDER_BASE + 1;

// for_each_napot_order / for_each_napot_order_rev are C loop macros.
#[inline] pub unsafe fn napot_cont_order(val: pte_t) -> u32 { (((val.pte >> _PAGE_PFN_SHIFT) << 1).trailing_zeros()) }
#[inline] pub unsafe fn napot_cont_shift(order: u32) -> u32 { order + PAGE_SHIFT }
#[inline] pub unsafe fn napot_cont_size(order: u32) -> usize { 1usize << napot_cont_shift(order) }
#[inline] pub unsafe fn napot_cont_mask(order: u32) -> usize { !(napot_cont_size(order) - 1) }
#[inline] pub unsafe fn napot_pte_num(order: u32) -> usize { 1usize << order }

#[cfg(CONFIG_RISCV_ISA_SVNAPOT)] pub const HUGE_MAX_HSTATE: i32 = 2 + (NAPOT_ORDER_MAX - NAPOT_CONT_ORDER_BASE);
#[cfg(not(CONFIG_RISCV_ISA_SVNAPOT))] pub const HUGE_MAX_HSTATE: i32 = 2;

pub const _PAGE_NOCACHE_SVPBMT: usize = 1usize << 61;
pub const _PAGE_IO_SVPBMT: usize = 1usize << 62;
pub const _PAGE_MTMASK_SVPBMT: usize = _PAGE_NOCACHE_SVPBMT | _PAGE_IO_SVPBMT;
pub const _PAGE_PMA_THEAD: usize = (1usize << 62) | (1usize << 61) | (1usize << 60);
pub const _PAGE_NOCACHE_THEAD: usize = (1usize << 61) | (1usize << 60);
pub const _PAGE_IO_THEAD: usize = (1usize << 63) | (1usize << 60);
pub const _PAGE_MTMASK_THEAD: usize = _PAGE_PMA_THEAD | _PAGE_IO_THEAD | (1usize << 59);

#[inline] pub unsafe fn riscv_page_mtmask() -> u64 { let mut val = 0u64; ALT_SVPBMT(&mut val, _PAGE_MTMASK); val }
#[inline] pub unsafe fn riscv_page_nocache() -> u64 { let mut val = 0u64; ALT_SVPBMT(&mut val, _PAGE_NOCACHE); val }
#[inline] pub unsafe fn riscv_page_io() -> u64 { let mut val = 0u64; ALT_SVPBMT(&mut val, _PAGE_IO); val }
#[inline] pub unsafe fn _PAGE_NOCACHE() -> u64 { riscv_page_nocache() }
#[inline] pub unsafe fn _PAGE_IO() -> u64 { riscv_page_io() }
#[inline] pub unsafe fn _PAGE_MTMASK() -> u64 { riscv_page_mtmask() }

#[inline] pub unsafe fn pud_present(pud: pud_t) -> i32 { (pud_val(pud) & _PAGE_PRESENT) as i32 }
#[inline] pub unsafe fn pud_none(pud: pud_t) -> i32 { (pud_val(pud) == 0) as i32 }
#[inline] pub unsafe fn pud_bad(pud: pud_t) -> i32 { (!pud_present(pud) != 0 || (pud_val(pud) & _PAGE_LEAF) != 0) as i32 }
#[inline] pub unsafe fn pud_leaf(pud: pud_t) -> bool { pud_present(pud) != 0 && (pud_val(pud) & _PAGE_LEAF) != 0 }
#[inline] pub unsafe fn pud_user(pud: pud_t) -> i32 { (pud_val(pud) & _PAGE_USER) as i32 }
#[inline] pub unsafe fn set_pud(pudp: *mut pud_t, pud: pud_t) { core::ptr::write_volatile(pudp, pud); }
#[inline] pub unsafe fn pud_clear(pudp: *mut pud_t) { set_pud(pudp, __pud(0)); }
#[inline] pub unsafe fn pfn_pud(pfn: usize, prot: pgprot_t) -> pud_t { __pud((pfn << _PAGE_PFN_SHIFT) | pgprot_val(prot)) }
#[inline] pub unsafe fn _pud_pfn(pud: pud_t) -> usize { __page_val_to_pfn(pud_val(pud)) }
#[inline] pub unsafe fn pud_pgtable(pud: pud_t) -> *mut pmd_t { pfn_to_virt(__page_val_to_pfn(pud_val(pud))) as *mut pmd_t }
#[inline] pub unsafe fn pud_page(pud: pud_t) -> *mut page { pfn_to_page(__page_val_to_pfn(pud_val(pud))) }

#[inline] pub unsafe fn mm_p4d_folded(_mm: *mut mm_struct) -> bool { !pgtable_l5_enabled }
#[inline] pub unsafe fn mm_pud_folded(_mm: *mut mm_struct) -> bool { !pgtable_l4_enabled }
#[inline] pub unsafe fn pmd_index(addr: usize) -> usize { (addr >> PMD_SHIFT) & (PTRS_PER_PMD() - 1) }
#[inline] pub unsafe fn pfn_pmd(pfn: usize, prot: pgprot_t) -> pmd_t { let mut prot_val = pgprot_val(prot); ALT_THEAD_PMA(&mut prot_val); __pmd((pfn << _PAGE_PFN_SHIFT) | prot_val) }
#[inline] pub unsafe fn _pmd_pfn(pmd: pmd_t) -> usize { __page_val_to_pfn(pmd_val(pmd)) }

#[inline] pub unsafe fn set_p4d(p4dp: *mut p4d_t, p4d: p4d_t) { if pgtable_l4_enabled { core::ptr::write_volatile(p4dp, p4d) } else { set_pud(p4dp as *mut pud_t, __pud(p4d_val(p4d))); } }
#[inline] pub unsafe fn p4d_none(p4d: p4d_t) -> i32 { if pgtable_l4_enabled { (p4d_val(p4d) == 0) as i32 } else { 0 } }
#[inline] pub unsafe fn p4d_present(p4d: p4d_t) -> i32 { if pgtable_l4_enabled { (p4d_val(p4d) & _PAGE_PRESENT) as i32 } else { 1 } }
#[inline] pub unsafe fn p4d_bad(p4d: p4d_t) -> i32 { if pgtable_l4_enabled { (p4d_present(p4d) == 0) as i32 } else { 0 } }
#[inline] pub unsafe fn p4d_clear(p4d: *mut p4d_t) { if pgtable_l4_enabled { set_p4d(p4d, __p4d(0)); } }
#[inline] pub unsafe fn pfn_p4d(pfn: usize, prot: pgprot_t) -> p4d_t { __p4d((pfn << _PAGE_PFN_SHIFT) | pgprot_val(prot)) }
#[inline] pub unsafe fn _p4d_pfn(p4d: p4d_t) -> usize { __page_val_to_pfn(p4d_val(p4d)) }
#[inline] pub unsafe fn p4d_pgtable(p4d: p4d_t) -> *mut pud_t { if pgtable_l4_enabled { pfn_to_virt(__page_val_to_pfn(p4d_val(p4d))) as *mut pud_t } else { pud_pgtable(__pud(p4d_val(p4d))) } }
#[inline] pub unsafe fn p4d_page_vaddr(p4d: p4d_t) -> usize { p4d_pgtable(p4d) as usize }
#[inline] pub unsafe fn p4d_page(p4d: p4d_t) -> *mut page { pfn_to_page(__page_val_to_pfn(p4d_val(p4d))) }
#[inline] pub unsafe fn pud_index(addr: usize) -> usize { (addr >> PUD_SHIFT) & (PTRS_PER_PUD() - 1) }
extern "C" { pub fn pud_offset(p4d: *mut p4d_t, address: usize) -> *mut pud_t; }

#[inline] pub unsafe fn set_pgd(pgdp: *mut pgd_t, pgd: pgd_t) { if pgtable_l5_enabled { core::ptr::write_volatile(pgdp, pgd) } else { set_p4d(pgdp as *mut p4d_t, __p4d(pgd_val(pgd))); } }
#[inline] pub unsafe fn pgd_none(pgd: pgd_t) -> i32 { if pgtable_l5_enabled { (pgd_val(pgd) == 0) as i32 } else { 0 } }
#[inline] pub unsafe fn pgd_present(pgd: pgd_t) -> i32 { if pgtable_l5_enabled { (pgd_val(pgd) & _PAGE_PRESENT) as i32 } else { 1 } }
#[inline] pub unsafe fn pgd_bad(pgd: pgd_t) -> i32 { if pgtable_l5_enabled { (pgd_present(pgd) == 0) as i32 } else { 0 } }
#[inline] pub unsafe fn pgd_clear(pgd: *mut pgd_t) { if pgtable_l5_enabled { set_pgd(pgd, __pgd(0)); } }
#[inline] pub unsafe fn pgd_pgtable(pgd: pgd_t) -> *mut p4d_t { if pgtable_l5_enabled { pfn_to_virt(__page_val_to_pfn(pgd_val(pgd))) as *mut p4d_t } else { p4d_pgtable(__p4d(pgd_val(pgd))) } }
#[inline] pub unsafe fn pgd_page_vaddr(pgd: pgd_t) -> usize { pgd_pgtable(pgd) as usize }
#[inline] pub unsafe fn pgd_page(pgd: pgd_t) -> *mut page { pfn_to_page(__page_val_to_pfn(pgd_val(pgd))) }
#[inline] pub unsafe fn p4d_index(addr: usize) -> usize { (addr >> P4D_SHIFT()) & (PTRS_PER_P4D() - 1) }
extern "C" { pub fn p4d_offset(pgd: *mut pgd_t, address: usize) -> *mut p4d_t; }

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
extern "C" {
    pub fn pmd_pte(pmd: pmd_t) -> pte_t;
    pub fn pud_pte(pud: pud_t) -> pte_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
