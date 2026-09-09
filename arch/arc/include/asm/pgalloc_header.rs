/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * vineetg: June 2011
 *  -"/proc/meminfo | grep PageTables" kept on increasing
 *   Recently added pgtable dtor was not getting called.
 *
 * vineetg: May 2011
 *  -Variable pg-sz means that Page Tables could be variable sized themselves
 *    So calculate it based on addr traversal split [pgd-bits:pte-bits:xxx]
 *  -Page Table size capped to max 1 to save memory - hence verified.
 *  -Since these deal with constants, gcc compile-time optimizes them.
 *
 * vineetg: Nov 2010
 *  -Added pgtable ctor/dtor used for pgtable mem accounting
 *
 * vineetg: April 2010
 *  -Switched pgtable_t from being struct page * to unsigned long
 *      =Needed so that Page Table allocator (pte_alloc_one) is not forced to
 *       deal with struct page. That way in future we can make it allocate
 *       multiple PG Tbls in one Page Frame
 *      =sweet side effect is avoiding calls to ugly page_address( ) from the
 *       pg-tlb allocator sub-sys (pte_alloc_one, ptr_free, pmd_populate)
 *
 *  Amit Bhor, Sameer Dhavale: Codito Technologies 2004
 */

// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn pmd_populate_kernel(mm: *mut mm_struct, pmd: *mut pmd_t, pte: *mut pte_t) {
    /*
     * The cast to long below is OK in 32-bit PAE40 regime with long long pte
     * Despite "wider" pte, the pte table needs to be in non-PAE low memory
     * as all higher levels can only hold long pointers.
     *
     * The cast itself is needed given simplistic definition of set_pmd()
     */
    set_pmd(pmd, __pmd(pte as usize as c_ulong));
}

pub unsafe fn pmd_populate(mm: *mut mm_struct, pmd: *mut pmd_t, pte_page: pgtable_t) {
    set_pmd(pmd, __pmd(page_address(pte_page) as usize as c_ulong));
}

pub unsafe fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    let ret: *mut pgd_t = __pgd_alloc(mm, 0);

    if !ret.is_null() {
        let num: c_int;
        let num2: c_int;

        num = USER_PTRS_PER_PGD + USER_KERNEL_GUTTER / PGDIR_SIZE;
        num2 = VMALLOC_SIZE / PGDIR_SIZE;
        memcpy(
            ret.add(num as usize),
            swapper_pg_dir.add(num as usize),
            (num2 as usize).wrapping_mul(core::mem::size_of::<pgd_t>()),
        );
    }
    ret
}

#[cfg(CONFIG_PGTABLE_LEVELS > 3)]
pub unsafe fn p4d_populate(mm: *mut mm_struct, p4dp: *mut p4d_t, pudp: *mut pud_t) {
    set_p4d(p4dp, __p4d(pudp as usize as c_ulong));
}

#[cfg(CONFIG_PGTABLE_LEVELS > 3)]
#[macro_export]
macro_rules! __pud_free_tlb {
    ($tlb:expr, $pmd:expr, $addr:expr) => {
        pud_free((*($tlb)).mm, $pmd)
    };
}

#[cfg(CONFIG_PGTABLE_LEVELS > 2)]
pub unsafe fn pud_populate(mm: *mut mm_struct, pudp: *mut pud_t, pmdp: *mut pmd_t) {
    set_pud(pudp, __pud(pmdp as usize as c_ulong));
}

#[cfg(CONFIG_PGTABLE_LEVELS > 2)]
#[macro_export]
macro_rules! __pmd_free_tlb {
    ($tlb:expr, $pmd:expr, $addr:expr) => {
        pmd_free((*($tlb)).mm, $pmd)
    };
}

#[macro_export]
macro_rules! __pte_free_tlb {
    ($tlb:expr, $pte:expr, $addr:expr) => {
        pte_free((*($tlb)).mm, $pte)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
