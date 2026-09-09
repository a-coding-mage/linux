// SPDX-License-Identifier: GPL-2.0

// DISABLE_BRANCH_PROFILING

// The declarations used below are provided by the corresponding kernel
// headers and architecture-specific translation units.

unsafe fn kasan_prot_ro() -> pgprot_t {
    if early_mmu_has_feature(MMU_FTR_HPTE_TABLE) {
        PAGE_READONLY
    } else {
        PAGE_KERNEL_RO
    }
}

unsafe fn kasan_populate_pte(mut ptep: *mut pte_t, prot: pgprot_t) {
    let va = kasan_early_shadow_page as *mut core::ffi::c_void as usize;
    let pa = __pa(kasan_early_shadow_page);

    for _i in 0..PTRS_PER_PTE {
        __set_pte_at(
            &mut init_mm,
            va,
            ptep,
            pfn_pte(PHYS_PFN(pa), prot),
            1,
        );
        ptep = ptep.add(1);
    }
}

pub unsafe fn kasan_init_shadow_page_tables(
    k_start: usize,
    k_end: usize,
) -> i32 {
    let mut pmd = pmd_off_k(k_start);
    let mut k_cur = k_start;

    while k_cur != k_end {
        let k_next = pgd_addr_end(k_cur, k_end);

        if (pmd_page_vaddr(*pmd) as *mut core::ffi::c_void) != kasan_early_shadow_pte {
            k_cur = k_next;
            pmd = pmd.add(1);
            continue;
        }

        let new = memblock_alloc(PTE_FRAG_SIZE, PTE_FRAG_SIZE);
        if new.is_null() {
            return -ENOMEM;
        }
        kasan_populate_pte(new, PAGE_KERNEL);
        pmd_populate_kernel(&mut init_mm, pmd, new);

        k_cur = k_next;
        pmd = pmd.add(1);
    }
    0
}

// __weak
pub unsafe fn kasan_init_region(start: *mut core::ffi::c_void, size: usize) -> i32 {
    let mut k_start = kasan_mem_to_shadow(start) as usize;
    let k_end = kasan_mem_to_shadow(start.add(size)) as usize;
    let ret = kasan_init_shadow_page_tables(k_start, k_end);
    if ret != 0 {
        return ret;
    }

    k_start &= PAGE_MASK;
    let block = memblock_alloc(k_end - k_start, PAGE_SIZE);
    if block.is_null() {
        return -ENOMEM;
    }

    let mut k_cur = k_start & PAGE_MASK;
    while k_cur < k_end {
        let pmd = pmd_off_k(k_cur);
        let va = (block as usize + k_cur - k_start) as *mut core::ffi::c_void;
        let pte = pfn_pte(PHYS_PFN(__pa(va)), PAGE_KERNEL);

        __set_pte_at(&mut init_mm, k_cur, pte_offset_kernel(pmd, k_cur), pte, 0);
        k_cur += PAGE_SIZE;
    }
    flush_tlb_kernel_range(k_start, k_end);
    0
}

pub unsafe fn kasan_update_early_region(k_start: usize, k_end: usize, pte: pte_t) {
    let mut k_cur = k_start;
    while k_cur != k_end {
        let pmd = pmd_off_k(k_cur);
        let ptep = pte_offset_kernel(pmd, k_cur);

        if pte_page(*ptep) != virt_to_page(lm_alias(kasan_early_shadow_page)) {
            k_cur += PAGE_SIZE;
            continue;
        }

        __set_pte_at(&mut init_mm, k_cur, ptep, pte, 0);
        k_cur += PAGE_SIZE;
    }

    flush_tlb_kernel_range(k_start, k_end);
}

unsafe fn kasan_remap_early_shadow_ro() {
    let prot = kasan_prot_ro();
    let pa = __pa(kasan_early_shadow_page);

    kasan_populate_pte(kasan_early_shadow_pte, prot);
    kasan_update_early_region(
        KASAN_SHADOW_START,
        KASAN_SHADOW_END,
        pfn_pte(PHYS_PFN(pa), prot),
    );
}

unsafe fn kasan_unmap_early_shadow_vmalloc() {
    let mut k_start = kasan_mem_to_shadow(VMALLOC_START as *mut core::ffi::c_void) as usize;
    let mut k_end = kasan_mem_to_shadow(VMALLOC_END as *mut core::ffi::c_void) as usize;

    kasan_update_early_region(k_start, k_end, __pte(0));

    // #ifdef MODULES_VADDR
    k_start = kasan_mem_to_shadow(MODULES_VADDR as *mut core::ffi::c_void) as usize;
    k_end = kasan_mem_to_shadow(MODULES_END as *mut core::ffi::c_void) as usize;
    kasan_update_early_region(k_start, k_end, __pte(0));
    // #endif
}

pub unsafe fn kasan_mmu_init() {
    if early_mmu_has_feature(MMU_FTR_HPTE_TABLE) {
        let ret = kasan_init_shadow_page_tables(KASAN_SHADOW_START, KASAN_SHADOW_END);
        if ret != 0 {
            panic!("kasan: kasan_init_shadow_page_tables() failed");
        }
    }
}

pub unsafe fn kasan_init() {
    let mut i: u64 = 0;
    let mut base: phys_addr_t = 0;
    let mut end: phys_addr_t = 0;

    // for_each_mem_range(i, &base, &end)
    while for_each_mem_range(&mut i, &mut base, &mut end) {
        let top = core::cmp::min(end, total_lowmem);
        if base >= top {
            continue;
        }

        let ret = kasan_init_region(__va(base), top - base);
        if ret != 0 {
            panic!("kasan: kasan_init_region() failed");
        }
    }

    if IS_ENABLED(CONFIG_KASAN_VMALLOC) {
        let ret = kasan_init_shadow_page_tables(KASAN_SHADOW_START, KASAN_SHADOW_END);
        if ret != 0 {
            panic!("kasan: kasan_init_shadow_page_tables() failed");
        }
    }

    kasan_remap_early_shadow_ro();
    clear_page(kasan_early_shadow_page);

    /* At this point kasan is fully initialized. Enable error messages */
    init_task.kasan_depth = 0;
    kasan_init_generic();
}

pub unsafe fn kasan_late_init() {
    if IS_ENABLED(CONFIG_KASAN_VMALLOC) {
        kasan_unmap_early_shadow_vmalloc();
    }
}

pub unsafe fn kasan_early_init() {
    let mut addr = KASAN_SHADOW_START;
    let end = KASAN_SHADOW_END;
    let mut pmd = pmd_off_k(addr);

    BUILD_BUG_ON(KASAN_SHADOW_START & !PGDIR_MASK);
    kasan_populate_pte(kasan_early_shadow_pte, PAGE_KERNEL);

    loop {
        let next = pgd_addr_end(addr, end);
        pmd_populate_kernel(&mut init_mm, pmd, kasan_early_shadow_pte);
        pmd = pmd.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
