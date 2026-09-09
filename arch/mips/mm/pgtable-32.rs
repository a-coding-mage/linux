/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2003 by Ralf Baechle
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/init.h, linux/mm.h, linux/memblock.h, linux/highmem.h,
// asm/fixmap.h, asm/pgalloc.h, and asm/tlbflush.h.

pub unsafe fn pgd_init(addr: *mut core::ffi::c_void) {
    let p = addr as *mut c_ulong;
    let mut i: c_int = 0;

    while i < USER_PTRS_PER_PGD {
        *p.add((i + 0) as usize) = invalid_pte_table as c_ulong;
        *p.add((i + 1) as usize) = invalid_pte_table as c_ulong;
        *p.add((i + 2) as usize) = invalid_pte_table as c_ulong;
        *p.add((i + 3) as usize) = invalid_pte_table as c_ulong;
        *p.add((i + 4) as usize) = invalid_pte_table as c_ulong;
        *p.add((i + 5) as usize) = invalid_pte_table as c_ulong;
        *p.add((i + 6) as usize) = invalid_pte_table as c_ulong;
        *p.add((i + 7) as usize) = invalid_pte_table as c_ulong;
        i += 8;
    }
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn set_pmd_at(mm: *mut mm_struct, addr: c_ulong,
                         pmdp: *mut pmd_t, pmd: pmd_t) {
    *pmdp = pmd;
}

// C __init annotation.
pub unsafe fn pagetable_init() {
    let mut vaddr: c_ulong;
    let pgd_base: *mut pgd_t;

    // CONFIG_HIGHMEM declarations and body are preserved below.
    #[cfg(CONFIG_HIGHMEM)]
    let (mut pgd, mut p4d, mut pud, mut pmd, mut pte):
        (*mut pgd_t, *mut p4d_t, *mut pud_t, *mut pmd_t, *mut pte_t);

    /* Initialize the entire pgd.  */
    pgd_init(swapper_pg_dir as *mut core::ffi::c_void);
    pgd_init(swapper_pg_dir.add(USER_PTRS_PER_PGD as usize) as *mut core::ffi::c_void);

    pgd_base = swapper_pg_dir;

    /*
     * Fixed mappings:
     */
    vaddr = __fix_to_virt(__end_of_fixed_addresses - 1);
    fixrange_init(vaddr & PMD_MASK, vaddr + FIXADDR_SIZE, pgd_base);

    #[cfg(CONFIG_HIGHMEM)]
    {
        /*
         * Permanent kmaps:
         */
        vaddr = PKMAP_BASE;
        fixrange_init(vaddr & PMD_MASK, vaddr + PAGE_SIZE * LAST_PKMAP, pgd_base);

        pgd = swapper_pg_dir.add(pgd_index(vaddr) as usize);
        p4d = p4d_offset(pgd, vaddr);
        pud = pud_offset(p4d, vaddr);
        pmd = pmd_offset(pud, vaddr);
        pte = pte_offset_kernel(pmd, vaddr);
        pkmap_page_table = pte;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
