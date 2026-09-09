// SPDX-License-Identifier: GPL-2.0
/*
 * KASAN for 64-bit Book3e powerpc
 *
 * Copyright 2022, Christophe Leroy, CS GROUP France
 */

// Translated from the C implementation. Kernel-provided types, constants,
// globals, and functions are intentionally referenced as external dependencies.

#[inline]
unsafe fn kasan_pud_table(p4d: p4d_t) -> bool {
    p4d_page(p4d) == virt_to_page(lm_alias(kasan_early_shadow_pud))
}

#[inline]
unsafe fn kasan_pmd_table(pud: pud_t) -> bool {
    pud_page(pud) == virt_to_page(lm_alias(kasan_early_shadow_pmd))
}

#[inline]
unsafe fn kasan_pte_table(pmd: pmd_t) -> bool {
    pmd_page(pmd) == virt_to_page(lm_alias(kasan_early_shadow_pte))
}

unsafe fn kasan_map_kernel_page(ea: c_ulong, pa: c_ulong, prot: pgprot_t) -> c_int {
    let mut pgdp: *mut pgd_t;
    let mut p4dp: *mut p4d_t;
    let mut pudp: *mut pud_t;
    let mut pmdp: *mut pmd_t;
    let mut ptep: *mut pte_t;

    pgdp = pgd_offset_k(ea);
    p4dp = p4d_offset(pgdp, ea);
    if kasan_pud_table(*p4dp) {
        pudp = memblock_alloc_or_panic(PUD_TABLE_SIZE, PUD_TABLE_SIZE);
        memcpy(pudp as *mut c_void, kasan_early_shadow_pud as *const c_void, PUD_TABLE_SIZE);
        p4d_populate(&init_mm, p4dp, pudp);
    }
    pudp = pud_offset(p4dp, ea);
    if kasan_pmd_table(*pudp) {
        pmdp = memblock_alloc_or_panic(PMD_TABLE_SIZE, PMD_TABLE_SIZE);
        memcpy(pmdp as *mut c_void, kasan_early_shadow_pmd as *const c_void, PMD_TABLE_SIZE);
        pud_populate(&init_mm, pudp, pmdp);
    }
    pmdp = pmd_offset(pudp, ea);
    if kasan_pte_table(*pmdp) {
        ptep = memblock_alloc_or_panic(PTE_TABLE_SIZE, PTE_TABLE_SIZE);
        memcpy(ptep as *mut c_void, kasan_early_shadow_pte as *const c_void, PTE_TABLE_SIZE);
        pmd_populate_kernel(&init_mm, pmdp, ptep);
    }
    ptep = pte_offset_kernel(pmdp, ea);

    __set_pte_at(&init_mm, ea, ptep, pfn_pte(pa >> PAGE_SHIFT, prot), 0);

    0
}

unsafe fn kasan_init_phys_region(start: *mut c_void, end: *mut c_void) {
    let k_start: c_ulong = ALIGN_DOWN(kasan_mem_to_shadow(start) as c_ulong, PAGE_SIZE);
    let k_end: c_ulong = ALIGN(kasan_mem_to_shadow(end) as c_ulong, PAGE_SIZE);

    let mut va = memblock_alloc_or_panic(k_end - k_start, PAGE_SIZE) as *mut u8;
    let mut k_cur = k_start;
    while k_cur < k_end {
        kasan_map_kernel_page(k_cur, __pa(va as *mut c_void), PAGE_KERNEL);
        k_cur += PAGE_SIZE;
        va = va.add(PAGE_SIZE as usize);
    }
}

pub unsafe fn kasan_early_init() {
    let mut pgd = pgd_offset_k(KASAN_SHADOW_START);
    let zero_pte: pte_t = pfn_pte(virt_to_pfn(kasan_early_shadow_page), PAGE_KERNEL);

    BUILD_BUG_ON(!IS_ALIGNED(KASAN_SHADOW_START, PGDIR_SIZE));
    BUILD_BUG_ON(!IS_ALIGNED(KASAN_SHADOW_END, PGDIR_SIZE));

    for i in 0..PTRS_PER_PTE {
        __set_pte_at(&init_mm, kasan_early_shadow_page as c_ulong,
                     &mut kasan_early_shadow_pte[i as usize], zero_pte, 0);
    }

    for i in 0..PTRS_PER_PMD {
        pmd_populate_kernel(&init_mm, &mut kasan_early_shadow_pmd[i as usize],
                            kasan_early_shadow_pte);
    }

    for i in 0..PTRS_PER_PUD {
        pud_populate(&init_mm, &mut kasan_early_shadow_pud[i as usize],
                     kasan_early_shadow_pmd);
    }

    let mut addr = KASAN_SHADOW_START;
    while addr != KASAN_SHADOW_END {
        p4d_populate(&init_mm, p4d_offset(pgd, addr), kasan_early_shadow_pud);
        pgd = pgd.add(1);
        addr += PGDIR_SIZE;
    }
}

pub unsafe fn kasan_init() {
    let mut start: phys_addr_t;
    let mut end: phys_addr_t;
    let mut i: u64;
    let zero_pte: pte_t = pfn_pte(virt_to_pfn(kasan_early_shadow_page), PAGE_KERNEL_RO);

    for_each_mem_range!(i, &mut start, &mut end, {
        kasan_init_phys_region(phys_to_virt(start), phys_to_virt(end));
    });

    if IS_ENABLED(CONFIG_KASAN_VMALLOC) {
        kasan_remove_zero_shadow(VMALLOC_START as *mut c_void, VMALLOC_SIZE);
    }

    for i in 0..PTRS_PER_PTE {
        __set_pte_at(&init_mm, kasan_early_shadow_page as c_ulong,
                     &mut kasan_early_shadow_pte[i as usize], zero_pte, 0);
    }

    flush_tlb_kernel_range(KASAN_SHADOW_START, KASAN_SHADOW_END);
    memset(kasan_early_shadow_page as *mut c_void, 0, PAGE_SIZE);

    /* Enable error messages */
    init_task.kasan_depth = 0;
    kasan_init_generic();
}

pub unsafe fn kasan_late_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
