/*
 * arch/sh/mm/cache-sh4.c -- direct Rust translation.
 * C headers and symbols are supplied by the surrounding kernel translation.
 */

const MAX_ICACHE_PAGES: usize = 32;

unsafe fn sh4_flush_icache_range(args: *mut c_void) {
    let data = args as *mut flusher_data;
    let start = (*data).addr1;
    let end = (*data).addr2;

    if ((end.wrapping_sub(start)) >> PAGE_SHIFT) >= MAX_ICACHE_PAGES as c_ulong {
        local_flush_cache_all(core::ptr::null_mut());
        return;
    }

    let start = start & !(L1_CACHE_BYTES - 1);
    let end = (end.wrapping_add(L1_CACHE_BYTES - 1)) & !(L1_CACHE_BYTES - 1);
    let mut flags: c_ulong = 0;
    local_irq_save(&mut flags);
    jump_to_uncached();

    let mut v = start;
    while v < end {
        __ocbwb(v);
        let mut icacheaddr = CACHE_IC_ADDRESS_ARRAY | (v & (*cpu_data).icache.entry_mask);
        let n = boot_cpu_data.icache.n_aliases;
        for _i in 0..cpu_data.icache.ways {
            for j in 0..n {
                __raw_writel(0, icacheaddr + (j as c_ulong * PAGE_SIZE));
            }
            icacheaddr += cpu_data.icache.way_incr;
        }
        v += L1_CACHE_BYTES;
    }
    back_to_cached();
    local_irq_restore(flags);
}

unsafe fn flush_cache_one(start: c_ulong, phys: c_ulong) {
    let mut exec_offset = 0;
    if (boot_cpu_data.flags & CPU_HAS_P2_FLUSH_BUG) != 0 || start < CACHE_OC_ADDRESS_ARRAY {
        exec_offset = cached_to_uncached;
    }
    let mut flags = 0;
    local_irq_save(&mut flags);
    __flush_cache_one(start, phys, exec_offset);
    local_irq_restore(flags);
}

unsafe fn sh4_flush_dcache_folio(arg: *mut c_void) {
    let folio = arg as *mut folio;
    #[cfg(not(CONFIG_SMP))]
    {
        let mapping = folio_flush_mapping(folio);
        if !mapping.is_null() && !mapping_mapped(mapping) {
            clear_bit(PG_dcache_clean, &mut (*folio).flags.f);
        } else {
            flush_folio_pages(folio);
        }
    }
    #[cfg(CONFIG_SMP)]
    flush_folio_pages(folio);
    wmb();
}

unsafe fn flush_folio_pages(folio: *mut folio) {
    let mut pfn = folio_pfn(folio);
    let mut addr = folio_address(folio) as c_ulong;
    let nr = folio_nr_pages(folio);
    for _ in 0..nr {
        flush_cache_one(CACHE_OC_ADDRESS_ARRAY | (addr & shm_align_mask), pfn * PAGE_SIZE);
        addr += PAGE_SIZE;
        pfn += 1;
    }
}

/* TODO: Selective icache invalidation through IC address array.. */
unsafe fn flush_icache_all() {
    let mut flags = 0;
    local_irq_save(&mut flags);
    jump_to_uncached();
    let mut ccr = __raw_readl(SH_CCR);
    ccr |= CCR_CACHE_ICI;
    __raw_writel(ccr, SH_CCR);
    back_to_cached();
    local_irq_restore(flags);
}

unsafe fn flush_dcache_all() {
    let end_addr = CACHE_OC_ADDRESS_ARRAY
        + (current_cpu_data.dcache.sets << current_cpu_data.dcache.entry_shift)
            * current_cpu_data.dcache.ways;
    let entry_offset = 1 << current_cpu_data.dcache.entry_shift;
    let mut addr = CACHE_OC_ADDRESS_ARRAY;
    while addr < end_addr {
        for _ in 0..8 {
            __raw_writel(0, addr);
            addr += entry_offset;
        }
    }
}

unsafe fn sh4_flush_cache_all(_unused: *mut c_void) { flush_dcache_all(); flush_icache_all(); }

unsafe fn sh4_flush_cache_mm(arg: *mut c_void) {
    let mm = arg as *mut mm_struct;
    if cpu_context(smp_processor_id(), mm) == NO_CONTEXT { return; }
    flush_dcache_all();
}

unsafe fn sh4_flush_cache_page(args: *mut c_void) {
    let data = args as *mut flusher_data;
    let vma = (*data).vma;
    let mut address = (*data).addr1 & PAGE_MASK;
    let pfn = (*data).addr2;
    let phys = pfn << PAGE_SHIFT;
    let page = pfn_to_page(pfn);
    if cpu_context(smp_processor_id(), (*vma).vm_mm) == NO_CONTEXT { return; }
    let pmd = pmd_off((*vma).vm_mm, address);
    let pte = pte_offset_kernel(pmd, address);
    if (pte_val(*pte) & _PAGE_PRESENT) == 0 { return; }
    let mut vaddr: *mut c_void = core::ptr::null_mut();
    let mut map_coherent = false;
    if (*vma).vm_mm != (*current).active_mm {
        let folio = page_folio(page);
        map_coherent = current_cpu_data.dcache.n_aliases != 0
            && test_bit(PG_dcache_clean, folio_flags(folio, 0))
            && folio_mapped(folio);
        vaddr = if map_coherent { kmap_coherent(page, address) } else { kmap_atomic(page) };
        address = vaddr as c_ulong;
    }
    flush_cache_one(CACHE_OC_ADDRESS_ARRAY | (address & shm_align_mask), phys);
    if ((*vma).vm_flags & VM_EXEC) != 0 { flush_icache_all(); }
    if !vaddr.is_null() {
        if map_coherent { kunmap_coherent(vaddr); } else { kunmap_atomic(vaddr); }
    }
}

unsafe fn sh4_flush_cache_range(args: *mut c_void) {
    let data = args as *mut flusher_data;
    let vma = (*data).vma;
    if cpu_context(smp_processor_id(), (*vma).vm_mm) == NO_CONTEXT { return; }
    if boot_cpu_data.dcache.n_aliases == 0 { return; }
    flush_dcache_all();
    if ((*vma).vm_flags & VM_EXEC) != 0 { flush_icache_all(); }
}

unsafe fn __flush_cache_one(mut addr: c_ulong, phys: c_ulong, exec_offset: c_ulong) {
    let dcache = &boot_cpu_data.dcache;
    let way_count = dcache.ways;
    let way_incr = dcache.way_incr;
    let mut temp_pc: c_ulong;
    core::arch::asm!(
        "mov.l 1f, {0}\n\tadd {1}, {0}\n\tjmp @{0}\n\tnop\n\t.balign 4\n1: .long 2f\n2:",
        out(reg) temp_pc, in(reg) exec_offset
    );
    let mut ways = way_count;
    while ways != 0 {
        let ea = addr + PAGE_SIZE;
        let mut a = addr;
        let mut p = phys;
        while a < ea {
            (a as *mut c_ulong).write_volatile(p);
            ((a + 32) as *mut c_ulong).write_volatile(p);
            a += 64;
            p += 64;
        }
        addr += way_incr;
        ways -= 1;
    }
    let _ = temp_pc;
}

unsafe fn sh4_cache_init() {
    printk!("PVR={:08x} CVR={:08x} PRR={:08x}\n", __raw_readl(CCN_PVR), __raw_readl(CCN_CVR), __raw_readl(CCN_PRR));
    local_flush_icache_range = sh4_flush_icache_range;
    local_flush_dcache_folio = sh4_flush_dcache_folio;
    local_flush_cache_all = sh4_flush_cache_all;
    local_flush_cache_mm = sh4_flush_cache_mm;
    local_flush_cache_dup_mm = sh4_flush_cache_mm;
    local_flush_cache_page = sh4_flush_cache_page;
    local_flush_cache_range = sh4_flush_cache_range;
    sh4__flush_region_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
