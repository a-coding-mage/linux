/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of arch/arm/include/asm/pgtable-3level.h */

/* With LPAE, each of the three page-table levels has 512 8-byte entries. */
pub const PTRS_PER_PTE: usize = 512;
pub const PTRS_PER_PMD: usize = 512;
pub const PTRS_PER_PGD: usize = 4;
pub const PTE_HWTABLE_PTRS: usize = 0;
pub const PTE_HWTABLE_OFF: usize = 0;
pub const PTE_HWTABLE_SIZE: usize = PTRS_PER_PTE * core::mem::size_of::<u64>();
pub const MAX_POSSIBLE_PHYSMEM_BITS: usize = 40;
pub const PGDIR_SHIFT: usize = 30;
pub const PMD_SHIFT: usize = 21;
pub const PMD_SIZE: usize = 1usize << PMD_SHIFT;
pub const PMD_MASK: usize = !((1usize << PMD_SHIFT) - 1);
pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !((1usize << PGDIR_SHIFT) - 1);
pub const SECTION_SHIFT: usize = 21;
pub const SECTION_SIZE: usize = 1usize << SECTION_SHIFT;
pub const SECTION_MASK: usize = !((1usize << SECTION_SHIFT) - 1);

/* PAGE_OFFSET, PAGE_SHIFT and the page-table value types are supplied externally. */
pub const USER_PTRS_PER_PGD: usize = PAGE_OFFSET / PGDIR_SIZE;
pub const HPAGE_SHIFT: usize = PMD_SHIFT;
pub const HPAGE_SIZE: usize = 1usize << HPAGE_SHIFT;
pub const HPAGE_MASK: usize = !(HPAGE_SIZE - 1);
pub const HUGETLB_PAGE_ORDER: usize = HPAGE_SHIFT - PAGE_SHIFT;

pub const L_PTE_VALID: pteval_t = (1 as pteval_t) << 0;
pub const L_PTE_PRESENT: pteval_t = (3 as pteval_t) << 0;
pub const L_PTE_USER: pteval_t = (1 as pteval_t) << 6;
pub const L_PTE_SHARED: pteval_t = (3 as pteval_t) << 8;
pub const L_PTE_YOUNG: pteval_t = (1 as pteval_t) << 10;
pub const L_PTE_XN: pteval_t = (1 as pteval_t) << 54;
pub const L_PTE_DIRTY: pteval_t = (1 as pteval_t) << 55;
pub const L_PTE_SPECIAL: pteval_t = (1 as pteval_t) << 56;
pub const L_PTE_NONE: pteval_t = (1 as pteval_t) << 57;
pub const L_PTE_RDONLY: pteval_t = (1 as pteval_t) << 58;
pub const L_PTE_SWP_EXCLUSIVE: pteval_t = (1 as pteval_t) << 7;
pub const L_PMD_SECT_VALID: pmdval_t = (1 as pmdval_t) << 0;
pub const L_PMD_SECT_DIRTY: pmdval_t = (1 as pmdval_t) << 55;
pub const L_PMD_SECT_NONE: pmdval_t = (1 as pmdval_t) << 57;
pub const L_PMD_SECT_RDONLY: pteval_t = (1 as pteval_t) << 58;
pub const L_PTE_XN_HIGH: u32 = 1 << (54 - 32);
pub const L_PTE_DIRTY_HIGH: u32 = 1 << (55 - 32);
pub const L_PTE_MT_UNCACHED: pteval_t = (0 as pteval_t) << 2;
pub const L_PTE_MT_BUFFERABLE: pteval_t = (1 as pteval_t) << 2;
pub const L_PTE_MT_WRITETHROUGH: pteval_t = (2 as pteval_t) << 2;
pub const L_PTE_MT_WRITEBACK: pteval_t = (3 as pteval_t) << 2;
pub const L_PTE_MT_WRITEALLOC: pteval_t = (7 as pteval_t) << 2;
pub const L_PTE_MT_DEV_SHARED: pteval_t = (4 as pteval_t) << 2;
pub const L_PTE_MT_DEV_NONSHARED: pteval_t = (4 as pteval_t) << 2;
pub const L_PTE_MT_DEV_WC: pteval_t = (1 as pteval_t) << 2;
pub const L_PTE_MT_DEV_CACHED: pteval_t = (3 as pteval_t) << 2;
pub const L_PTE_MT_MASK: pteval_t = (7 as pteval_t) << 2;
pub const L_PGD_SWAPPER: pgdval_t = (1 as pgdval_t) << 55;

pub const fn pud_none(pud: pud_t) -> bool { pud_val(pud) == 0 }
pub const fn pud_bad(pud: pud_t) -> bool { (pud_val(pud) & PUD_TABLE_BIT) == 0 }
pub const fn pud_present(pud: pud_t) -> bool { pud_val(pud) != 0 }
pub const fn pmd_table(pmd: pmd_t) -> bool { (pmd_val(pmd) & PMD_TYPE_MASK) == PMD_TYPE_TABLE }
pub const fn pmd_sect(pmd: pmd_t) -> bool { (pmd_val(pmd) & PMD_TYPE_MASK) == PMD_TYPE_SECT }
pub const fn pmd_leaf(pmd: pmd_t) -> bool { pmd_sect(pmd) }

pub unsafe fn pud_clear(pudp: *mut pud_t) {
    *pudp = __pud(0);
    clean_pmd_entry(pudp);
}
pub unsafe fn set_pud(pudp: *mut pud_t, pud: pud_t) {
    *pudp = pud;
    flush_pmd_entry(pudp);
}
pub unsafe fn pud_pgtable(pud: pud_t) -> *mut pmd_t {
    __va(pud_val(pud) & PHYS_MASK & (PAGE_MASK as s32))
}
pub const fn pmd_bad(pmd: pmd_t) -> bool { (pmd_val(pmd) & PMD_TABLE_BIT) == 0 }
pub unsafe fn copy_pmd(pmdpd: *mut pmd_t, pmdps: *const pmd_t) {
    *pmdpd = *pmdps;
    flush_pmd_entry(pmdpd);
}
pub unsafe fn pmd_clear(pmdp: *mut pmd_t) {
    *pmdp = __pmd(0);
    clean_pmd_entry(pmdp);
}

pub const fn pte_same(pte_a: pte_t, pte_b: pte_t) -> bool {
    let a = if pte_present(pte_a) { pte_val(pte_a) & !PTE_EXT_NG } else { pte_val(pte_a) };
    let b = if pte_present(pte_b) { pte_val(pte_b) & !PTE_EXT_NG } else { pte_val(pte_b) };
    a == b
}
pub unsafe fn set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: pteval_t) { cpu_set_pte_ext(ptep, __pte(pte_val(pte) | ext)); }
pub const fn pte_huge(pte: pte_t) -> bool { pte_val(pte) != 0 && (pte_val(pte) & PTE_TABLE_BIT) == 0 }
pub const fn pte_mkhuge(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !PTE_TABLE_BIT) }
pub const fn pmd_isset(pmd: pmd_t, val: pmdval_t) -> pmdval_t { pmd_val(pmd) & val }
pub const fn pmd_isclear(pmd: pmd_t, val: pmdval_t) -> bool { (pmd_val(pmd) & val) == 0 }
pub const fn pmd_present(pmd: pmd_t) -> bool { pmd_isset(pmd, L_PMD_SECT_VALID) != 0 }
pub const fn pmd_young(pmd: pmd_t) -> bool { pmd_isset(pmd, PMD_SECT_AF) != 0 }
pub const fn pte_special(pte: pte_t) -> bool { pte_isset(pte, L_PTE_SPECIAL) }
pub fn pte_mkspecial(mut pte: pte_t) -> pte_t { pte_val(pte) |= L_PTE_SPECIAL; pte }
pub const fn pmd_write(pmd: pmd_t) -> bool { pmd_isclear(pmd, L_PMD_SECT_RDONLY) }
pub const fn pmd_dirty(pmd: pmd_t) -> bool { pmd_isset(pmd, L_PMD_SECT_DIRTY) != 0 }
pub const fn pmd_hugewillfault(pmd: pmd_t) -> bool { !pmd_young(pmd) || !pmd_write(pmd) }
pub const fn pmd_mkhuge(pmd: pmd_t) -> pmd_t { __pmd(pmd_val(pmd) & !PMD_TABLE_BIT) }
pub const fn pmd_pfn(pmd: pmd_t) -> usize { ((pmd_val(pmd) & PMD_MASK) & PHYS_MASK) >> PAGE_SHIFT }
pub const fn pfn_pmd(pfn: usize, prot: pgprot_t) -> pmd_t { __pmd(((pfn as phys_addr_t) << PAGE_SHIFT) | pgprot_val(prot)) }

/* No hardware dirty/accessed bits -- generic_pmdp_establish() fits. */
pub use generic_pmdp_establish as pmdp_establish;

pub const fn pmd_mkinvalid(pmd: pmd_t) -> pmd_t { __pmd(pmd_val(pmd) & !L_PMD_SECT_VALID) }
pub fn pmd_modify(mut pmd: pmd_t, newprot: pgprot_t) -> pmd_t {
    let mask: pmdval_t = PMD_SECT_USER | PMD_SECT_XN | L_PMD_SECT_RDONLY as pmdval_t | L_PMD_SECT_VALID | L_PMD_SECT_NONE;
    pmd_val(pmd) = (pmd_val(pmd) & !mask) | (pgprot_val(newprot) & mask); pmd
}
pub unsafe fn set_pmd_at(mm: *mut mm_struct, addr: usize, pmdp: *mut pmd_t, mut pmd: pmd_t) {
    BUG_ON(addr >= TASK_SIZE);
    if pmd_val(pmd) & L_PMD_SECT_NONE != 0 { pmd_val(pmd) &= !L_PMD_SECT_VALID; }
    if pmd_write(pmd) && pmd_dirty(pmd) { pmd_val(pmd) &= !PMD_SECT_AP2; }
    else { pmd_val(pmd) |= PMD_SECT_AP2; }
    *pmdp = __pmd(pmd_val(pmd) | PMD_SECT_nG);
    flush_pmd_entry(pmdp);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
