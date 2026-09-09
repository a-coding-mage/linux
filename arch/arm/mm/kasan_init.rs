// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file contains kasan initialization code for ARM.
 *
 * Copyright (c) 2018 Samsung Electronics Co., Ltd.
 * Author: Andrey Ryabinin <ryabinin.a.a@gmail.com>
 * Author: Linus Walleij <linus.walleij@linaro.org>
 */

// Dependencies supplied by the kernel ARM/MM environment are intentionally
// referenced here rather than redefined.

static mut TMP_PGD_TABLE: [pgd_t; PTRS_PER_PGD] = [pgd_t::default(); PTRS_PER_PGD];

pub static mut tmp_pmd_table: [pmd_t; PTRS_PER_PMD] = [pmd_t::default(); PTRS_PER_PMD];

unsafe fn kasan_alloc_block_raw(size: usize) -> *mut core::ffi::c_void
{
    memblock_alloc_try_nid_raw(
        size,
        size,
        __pa(MAX_DMA_ADDRESS),
        MEMBLOCK_ALLOC_NOLEAKTRACE,
        NUMA_NO_NODE,
    )
}

unsafe fn kasan_alloc_block(size: usize) -> *mut core::ffi::c_void
{
    memblock_alloc_try_nid(
        size,
        size,
        __pa(MAX_DMA_ADDRESS),
        MEMBLOCK_ALLOC_NOLEAKTRACE,
        NUMA_NO_NODE,
    )
}

unsafe fn kasan_pte_populate(
    pmdp: *mut pmd_t,
    mut addr: c_ulong,
    end: c_ulong,
    early: bool,
) {
    let mut next: c_ulong;
    let mut ptep: *mut pte_t = pte_offset_kernel(pmdp, addr);

    loop {
        let entry: pte_t;
        let p: *mut core::ffi::c_void;

        next = addr + PAGE_SIZE;

        if !early {
            if !pte_none(READ_ONCE(*ptep)) {
                ptep = ptep.add(1);
                addr = next;
                if addr == end { break; }
                continue;
            }

            p = kasan_alloc_block_raw(PAGE_SIZE);
            if p.is_null() {
                panic!("{} failed to allocate shadow page for address 0x{:lx}\n", "kasan_pte_populate", addr);
                return;
            }
            memset(p, KASAN_SHADOW_INIT, PAGE_SIZE);
            entry = pfn_pte(virt_to_pfn(p), __pgprot(pgprot_val(PAGE_KERNEL)));
        } else if pte_none(READ_ONCE(*ptep)) {
            /*
             * The early shadow memory is mapping all KASan
             * operations to one and the same page in memory,
             * "kasan_early_shadow_page" so that the instrumentation
             * will work on a scratch area until we can set up the
             * proper KASan shadow memory.
             */
            entry = pfn_pte(
                virt_to_pfn(kasan_early_shadow_page),
                __pgprot(_L_PTE_DEFAULT | L_PTE_DIRTY | L_PTE_XN),
            );
        } else {
            /*
             * Early shadow mappings are PMD_SIZE aligned, so if the
             * first entry is already set, they must all be set.
             */
            return;
        }

        set_pte_at(&mut init_mm, addr, ptep, entry);
        ptep = ptep.add(1);
        addr = next;
        if addr == end { break; }
    }
}

/*
 * The pmd (page middle directory) is only used on LPAE
 */
unsafe fn kasan_pmd_populate(
    pudp: *mut pud_t,
    mut addr: c_ulong,
    end: c_ulong,
    early: bool,
) {
    let mut next: c_ulong;
    let mut pmdp: *mut pmd_t = pmd_offset(pudp, addr);

    loop {
        if pmd_none(*pmdp) {
            /*
             * We attempt to allocate a shadow block for the PMDs
             * used by the PTEs for this address if it isn't already
             * allocated.
             */
            let p = if early { kasan_early_shadow_pte } else { kasan_alloc_block(PAGE_SIZE) };
            if p.is_null() {
                panic!("{} failed to allocate shadow block for address 0x{:lx}\n", "kasan_pmd_populate", addr);
                return;
            }
            pmd_populate_kernel(&mut init_mm, pmdp, p);
            flush_pmd_entry(pmdp);
        }

        next = pmd_addr_end(addr, end);
        kasan_pte_populate(pmdp, addr, next, early);
        pmdp = pmdp.add(1);
        addr = next;
        if addr == end { break; }
    }
}

unsafe fn kasan_pgd_populate(mut addr: c_ulong, end: c_ulong, early: bool) {
    let mut next: c_ulong;
    let mut pgdp: *mut pgd_t;
    let p4dp: *mut p4d_t;
    let pudp: *mut pud_t;

    pgdp = pgd_offset_k(addr);

    loop {
        /*
         * Allocate and populate the shadow block of p4d folded into
         * pud folded into pmd if it doesn't already exist
         */
        if !early && pgd_none(*pgdp) {
            let p = kasan_alloc_block(PAGE_SIZE);
            if p.is_null() {
                panic!("{} failed to allocate shadow block for address 0x{:lx}\n", "kasan_pgd_populate", addr);
                return;
            }
            pgd_populate(&mut init_mm, pgdp, p);
        }

        next = pgd_addr_end(addr, end);
        /*
         * We just immediately jump over the p4d and pud page
         * directories since we believe ARM32 will never gain four
         * nor five level page tables.
         */
        p4dp = p4d_offset(pgdp, addr);
        pudp = pud_offset(p4dp, addr);

        kasan_pmd_populate(pudp, addr, next, early);
        pgdp = pgdp.add(1);
        addr = next;
        if addr == end { break; }
    }
}

extern "C" {
    fn lookup_processor_type(arch: c_uint) -> *mut proc_info_list;
}

pub unsafe fn kasan_early_init() {
    let list: *mut proc_info_list;

    /*
     * locate processor in the list of supported processor
     * types.  The linker builds this table for us from the
     * entries in arch/arm/mm/proc-*.S
     */
    list = lookup_processor_type(read_cpuid_id());
    if !list.is_null() {
        // #ifdef MULTI_CPU
        processor = *(*list).proc;
        // #endif
    }

    // BUILD_BUG_ON((KASAN_SHADOW_END - (1UL << 29)) != KASAN_SHADOW_OFFSET);
    kasan_pgd_populate(KASAN_SHADOW_START, KASAN_SHADOW_END, true);
}

unsafe fn clear_pgds(mut start: c_ulong, end: c_ulong) {
    while start != 0 && start < end {
        pmd_clear(pmd_off_k(start));
        start += PMD_SIZE;
    }
}

unsafe fn create_mapping(start: *mut core::ffi::c_void, end: *mut core::ffi::c_void) -> c_int {
    let shadow_start = kasan_mem_to_shadow(start);
    let shadow_end = kasan_mem_to_shadow(end);

    pr_info!("Mapping kernel virtual memory block: %px-%px at shadow: %px-%px\n", start, end, shadow_start, shadow_end);

    kasan_pgd_populate(
        (shadow_start as c_ulong) & PAGE_MASK,
        PAGE_ALIGN(shadow_end as c_ulong),
        false,
    );
    0
}

pub unsafe fn kasan_init() {
    let mut pa_start: phys_addr_t;
    let mut pa_end: phys_addr_t;
    let mut i: u64;

    /*
     * We are going to perform proper setup of shadow memory.
     *
     * At first we should unmap early shadow (clear_pgds() call bellow).
     * However, instrumented code can't execute without shadow memory.
     *
     * To keep the early shadow memory MMU tables around while setting up
     * the proper shadow memory, we copy swapper_pg_dir (the initial page
     * table) to tmp_pgd_table and use that to keep the early shadow memory
     * mapped until the full shadow setup is finished. Then we swap back
     * to the proper swapper_pg_dir.
     */

    memcpy(TMP_PGD_TABLE.as_mut_ptr(), swapper_pg_dir, core::mem::size_of_val(&TMP_PGD_TABLE));
    // #ifdef CONFIG_ARM_LPAE
    // BUILD_BUG_ON(pgd_index(KASAN_SHADOW_START) != pgd_index(KASAN_SHADOW_END));
    memcpy(
        tmp_pmd_table.as_mut_ptr(),
        pgd_page_vaddr(*pgd_offset_k(KASAN_SHADOW_START)) as *const _,
        core::mem::size_of_val(&tmp_pmd_table),
    );
    set_pgd(
        &mut TMP_PGD_TABLE[pgd_index(KASAN_SHADOW_START)],
        __pgd(__pa(tmp_pmd_table.as_mut_ptr()) | PMD_TYPE_TABLE | L_PGD_SWAPPER),
    );
    // #endif
    cpu_switch_mm(TMP_PGD_TABLE.as_mut_ptr(), &mut init_mm);
    local_flush_tlb_all();

    clear_pgds(KASAN_SHADOW_START, KASAN_SHADOW_END);

    // if !IS_ENABLED(CONFIG_KASAN_VMALLOC)
    kasan_populate_early_shadow(
        kasan_mem_to_shadow(VMALLOC_START as *mut _),
        kasan_mem_to_shadow(VMALLOC_END as *mut _),
    );
    // endif

    kasan_populate_early_shadow(
        kasan_mem_to_shadow(VMALLOC_END as *mut _),
        kasan_mem_to_shadow((-1isize) as *mut _) + 1,
    );

    for_each_mem_range(&mut i, &mut pa_start, &mut pa_end) {
        let start = __va(pa_start);
        let mut end = __va(pa_end);

        /* Do not attempt to shadow highmem */
        if pa_start >= arm_lowmem_limit {
            pr_info!("Skip highmem block at %pa-%pa\n", &pa_start, &pa_end);
            continue;
        }
        if pa_end > arm_lowmem_limit {
            pr_info!("Truncating shadow for memory block at %pa-%pa to lowmem region at %pa\n", &pa_start, &pa_end, &arm_lowmem_limit);
            end = __va(arm_lowmem_limit);
        }
        create_mapping(start, end);
    }

    /*
     * 1. The module global variables are in MODULES_VADDR ~ MODULES_END,
     *    so we need to map this area if CONFIG_KASAN_VMALLOC=n. With
     *    VMALLOC support KASAN will manage this region dynamically,
     *    refer to kasan_populate_vmalloc() and ARM's implementation of
     *    module_alloc().
     * 2. PKMAP_BASE ~ PKMAP_BASE+PMD_SIZE's shadow and MODULES_VADDR
     *    ~ MODULES_END's shadow is in the same PMD_SIZE, so we can't
     *    use kasan_populate_zero_shadow.
     */
    // if !IS_ENABLED(CONFIG_KASAN_VMALLOC) && IS_ENABLED(CONFIG_MODULES)
    create_mapping(MODULES_VADDR as *mut _, MODULES_END as *mut _);
    // endif
    create_mapping(PKMAP_BASE as *mut _, (PKMAP_BASE + PMD_SIZE) as *mut _);

    /*
     * KAsan may reuse the contents of kasan_early_shadow_pte directly, so
     * we should make sure that it maps the zero page read-only.
     */
    i = 0;
    while i < PTRS_PER_PTE {
        set_pte_at(
            &mut init_mm,
            KASAN_SHADOW_START + i * PAGE_SIZE,
            &mut kasan_early_shadow_pte[i as usize],
            pfn_pte(
                virt_to_pfn(kasan_early_shadow_page),
                __pgprot(pgprot_val(PAGE_KERNEL) | L_PTE_RDONLY),
            ),
        );
        i += 1;
    }

    cpu_switch_mm(swapper_pg_dir, &mut init_mm);
    local_flush_tlb_all();

    memset(kasan_early_shadow_page, 0, PAGE_SIZE);
    init_task.kasan_depth = 0;
    kasan_init_generic();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
