/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1999, 2000 by Silicon Graphics
 * Copyright (C) 2003 by Ralf Baechle
 */

// Dependencies supplied by the surrounding kernel translation unit.

pub unsafe fn pgd_init(addr: *mut core::ffi::c_void) {
    let entry: usize;

    // !defined(__PAGETABLE_PUD_FOLDED)
    #[cfg(not(__PAGETABLE_PUD_FOLDED))]
    {
        entry = invalid_pud_table as usize;
    }
    // !defined(__PAGETABLE_PMD_FOLDED)
    #[cfg(all(__PAGETABLE_PUD_FOLDED, not(__PAGETABLE_PMD_FOLDED)))]
    {
        entry = invalid_pmd_table as usize;
    }
    #[cfg(all(__PAGETABLE_PUD_FOLDED, __PAGETABLE_PMD_FOLDED))]
    {
        entry = invalid_pte_table as usize;
    }

    let mut p = addr as *mut usize;
    let end = p.add(PTRS_PER_PGD as usize);

    loop {
        *p.add(0) = entry;
        *p.add(1) = entry;
        *p.add(2) = entry;
        *p.add(3) = entry;
        *p.add(4) = entry;
        p = p.add(8);
        *p.sub(3) = entry;
        *p.sub(2) = entry;
        *p.sub(1) = entry;
        if p == end {
            break;
        }
    }
}

// #ifndef __PAGETABLE_PMD_FOLDED
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
pub unsafe fn pmd_init(addr: *mut core::ffi::c_void) {
    let pagetable = invalid_pte_table as usize;
    let mut p = addr as *mut usize;
    let end = p.add(PTRS_PER_PMD as usize);

    loop {
        *p.add(0) = pagetable;
        *p.add(1) = pagetable;
        *p.add(2) = pagetable;
        *p.add(3) = pagetable;
        *p.add(4) = pagetable;
        p = p.add(8);
        *p.sub(3) = pagetable;
        *p.sub(2) = pagetable;
        *p.sub(1) = pagetable;
        if p == end {
            break;
        }
    }
}
// EXPORT_SYMBOL_GPL(pmd_init);

// #ifndef __PAGETABLE_PUD_FOLDED
#[cfg(not(__PAGETABLE_PUD_FOLDED))]
pub unsafe fn pud_init(addr: *mut core::ffi::c_void) {
    let pagetable = invalid_pmd_table as usize;
    let mut p = addr as *mut usize;
    let end = p.add(PTRS_PER_PUD as usize);

    loop {
        *p.add(0) = pagetable;
        *p.add(1) = pagetable;
        *p.add(2) = pagetable;
        *p.add(3) = pagetable;
        *p.add(4) = pagetable;
        p = p.add(8);
        *p.sub(3) = pagetable;
        *p.sub(2) = pagetable;
        *p.sub(1) = pagetable;
        if p == end {
            break;
        }
    }
}

// #ifdef CONFIG_TRANSPARENT_HUGEPAGE
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn set_pmd_at(mm: *mut mm_struct, addr: usize, pmdp: *mut pmd_t, pmd: pmd_t) {
    let _ = (mm, addr);
    *pmdp = pmd;
}

// __init
pub unsafe fn pagetable_init() {
    let vaddr: usize;
    let pgd_base: *mut pgd_t;

    /* Initialize the entire pgd.  */
    pgd_init(swapper_pg_dir as *mut core::ffi::c_void);
    // #ifndef __PAGETABLE_PUD_FOLDED
    #[cfg(not(__PAGETABLE_PUD_FOLDED))]
    pud_init(invalid_pud_table as *mut core::ffi::c_void);
    // #ifndef __PAGETABLE_PMD_FOLDED
    #[cfg(not(__PAGETABLE_PMD_FOLDED))]
    pmd_init(invalid_pmd_table as *mut core::ffi::c_void);
    pgd_base = swapper_pg_dir;
    /*
     * Fixed mappings:
     */
    vaddr = __fix_to_virt(__end_of_fixed_addresses - 1) & PMD_MASK;
    fixrange_init(vaddr, vaddr + FIXADDR_SIZE, pgd_base);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
