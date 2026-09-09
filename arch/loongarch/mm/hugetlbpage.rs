// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// C dependencies:
// linux/fs.h, linux/mm.h, linux/hugetlb.h, linux/pagemap.h, linux/err.h,
// linux/sysctl.h, asm/mman.h, asm/tlb.h, and asm/tlbflush.h

pub unsafe fn huge_pte_alloc(
    mm: *mut mm_struct,
    vma: *mut vm_area_struct,
    addr: usize,
    sz: usize,
) -> *mut pte_t {
    let pgd: *mut pgd_t;
    let p4d: *mut p4d_t;
    let pud: *mut pud_t;
    let mut pte: *mut pte_t = core::ptr::null_mut();

    pgd = pgd_offset(mm, addr);
    p4d = p4d_alloc(mm, pgd, addr);
    pud = pud_alloc(mm, p4d, addr);
    if !pud.is_null() {
        pte = pmd_alloc(mm, pud, addr) as *mut pte_t;
    }

    pte
}

pub unsafe fn huge_pte_offset(
    mm: *mut mm_struct,
    addr: usize,
    sz: usize,
) -> *mut pte_t {
    let pgd: *mut pgd_t;
    let p4d: *mut p4d_t;
    let pud: *mut pud_t;
    let mut pmd: *mut pmd_t = core::ptr::null_mut();

    pgd = pgd_offset(mm, addr);
    if pgd_present(pgdp_get(pgd)) {
        p4d = p4d_offset(pgd, addr);
        if p4d_present(p4dp_get(p4d)) {
            pud = pud_offset(p4d, addr);
            if pud_present(pudp_get(pud)) {
                pmd = pmd_offset(pud, addr);
            }
        }
    }

    if pmd.is_null() || pmd_none(pmdp_get(pmd)) {
        core::ptr::null_mut()
    } else {
        pmd as *mut pte_t
    }
}

pub unsafe fn pmd_to_entrylo(pmd_val: usize) -> u64 {
    let val: u64;
    /* PMD as PTE. Must be huge page */
    if !pmd_leaf(__pmd(pmd_val)) {
        panic!("{}", "pmd_to_entrylo");
    }

    val = (pmd_val ^ _PAGE_HUGE) as u64;
    val | ((val & _PAGE_HGLOBAL)
        >> (_PAGE_HGLOBAL_SHIFT - _PAGE_GLOBAL_SHIFT))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
