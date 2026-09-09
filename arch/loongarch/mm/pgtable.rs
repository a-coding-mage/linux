// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */
// Dependencies supplied by the corresponding Linux architecture headers.

pub unsafe fn dmw_virt_to_page(kaddr: ::core::ffi::c_ulong) -> *mut page {
    phys_to_page(__pa(kaddr))
}

pub unsafe fn tlb_virt_to_page(kaddr: ::core::ffi::c_ulong) -> *mut page {
    phys_to_page(pfn_to_phys(pte_pfn((*virt_to_kpte(kaddr)).read())))
}

pub unsafe fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    let mut init: *mut pgd_t;
    let ret = __pgd_alloc(mm, 0);
    if !ret.is_null() {
        init = pgd_offset(&raw mut init_mm, 0 as ::core::ffi::c_ulong);
        pgd_init(ret as *mut ::core::ffi::c_void);
        core::ptr::copy_nonoverlapping(
            init.add(USER_PTRS_PER_PGD),
            ret.add(USER_PTRS_PER_PGD),
            PTRS_PER_PGD - USER_PTRS_PER_PGD,
        );
    }
    ret
}

pub unsafe fn pgd_init(addr: *mut ::core::ffi::c_void) {
    let entry: ::core::ffi::c_ulong = {
        // Build-time page-table folding conditions are preserved from C.
        #[cfg(not(__PAGETABLE_PUD_FOLDED))]
        { invalid_pud_table as ::core::ffi::c_ulong }
        #[cfg(all(__PAGETABLE_PUD_FOLDED, not(__PAGETABLE_PMD_FOLDED)))]
        { invalid_pmd_table as ::core::ffi::c_ulong }
        #[cfg(all(__PAGETABLE_PUD_FOLDED, __PAGETABLE_PMD_FOLDED))]
        { invalid_pte_table as ::core::ffi::c_ulong }
    };
    let mut p = addr as *mut ::core::ffi::c_ulong;
    let end = p.add(PTRS_PER_PGD);
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
        if p == end { break; }
    }
}

// C condition: #ifndef __PAGETABLE_PMD_FOLDED
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
pub unsafe fn pmd_init(addr: *mut ::core::ffi::c_void) {
    let pagetable = invalid_pte_table as ::core::ffi::c_ulong;
    let mut p = addr as *mut ::core::ffi::c_ulong;
    let end = p.add(PTRS_PER_PMD);
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
        if p == end { break; }
    }
}

// C condition: #ifndef __PAGETABLE_PUD_FOLDED
#[cfg(not(__PAGETABLE_PUD_FOLDED))]
pub unsafe fn pud_init(addr: *mut ::core::ffi::c_void) {
    let pagetable = invalid_pmd_table as ::core::ffi::c_ulong;
    let mut p = addr as *mut ::core::ffi::c_ulong;
    let end = p.add(PTRS_PER_PUD);
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
        if p == end { break; }
    }
}

pub unsafe fn kernel_pte_init(addr: *mut ::core::ffi::c_void) {
    let mut p = addr as *mut ::core::ffi::c_ulong;
    let end = p.add(PTRS_PER_PTE);
    loop {
        *p.add(0) = _PAGE_GLOBAL;
        *p.add(1) = _PAGE_GLOBAL;
        *p.add(2) = _PAGE_GLOBAL;
        *p.add(3) = _PAGE_GLOBAL;
        *p.add(4) = _PAGE_GLOBAL;
        p = p.add(8);
        *p.sub(3) = _PAGE_GLOBAL;
        *p.sub(2) = _PAGE_GLOBAL;
        *p.sub(1) = _PAGE_GLOBAL;
        if p == end { break; }
    }
}

pub unsafe fn set_pmd_at(
    mm: *mut mm_struct,
    addr: ::core::ffi::c_ulong,
    pmdp: *mut pmd_t,
    pmd: pmd_t,
) {
    let _ = (mm, addr);
    core::ptr::write_volatile(pmdp, pmd);
    flush_tlb_all();
}

// __init; CONFIG_HIGHMEM declarations and body are preserved conditionally.
pub unsafe fn pagetable_init() {
    pgd_init(swapper_pg_dir as *mut ::core::ffi::c_void);
    pgd_init(invalid_pg_dir as *mut ::core::ffi::c_void);
    #[cfg(not(__PAGETABLE_PUD_FOLDED))]
    pud_init(invalid_pud_table as *mut ::core::ffi::c_void);
    #[cfg(not(__PAGETABLE_PMD_FOLDED))]
    pmd_init(invalid_pmd_table as *mut ::core::ffi::c_void);

    #[cfg(CONFIG_HIGHMEM)]
    {
        let mut vaddr: ::core::ffi::c_ulong = PKMAP_BASE;
        fixrange_init(vaddr & PMD_MASK, vaddr + PAGE_SIZE * LAST_PKMAP, swapper_pg_dir);
        let pgd = swapper_pg_dir.add(pgd_index(vaddr));
        let p4d = p4d_offset(pgd, vaddr);
        let pud = pud_offset(p4d, vaddr);
        let pmd = pmd_offset(pud, vaddr);
        let pte = pte_offset_kernel(pmd, vaddr);
        pkmap_page_table = pte;
        vaddr = __fix_to_virt(__end_of_fixed_addresses - 1);
        fixrange_init(vaddr & PMD_MASK, vaddr + FIXADDR_SIZE, swapper_pg_dir);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
