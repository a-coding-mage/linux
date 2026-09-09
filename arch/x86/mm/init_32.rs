// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 1995 Linus Torvalds
 * Support of BIGMEM added by Gerhard Wichert, Siemens AG, July 1999
 */

// Linux and architecture headers supply the types, constants, macros, and
// external functions referenced below.

pub static mut highstart_pfn: c_ulong = 0;
pub static mut highend_pfn: c_ulong = 0;
pub static mut __vmalloc_start_set: bool = false;

unsafe fn one_md_table_init(pgd: *mut pgd_t) -> *mut pmd_t {
    let p4d: *mut p4d_t;
    let pud: *mut pud_t;
    let pmd_table: *mut pmd_t;

    // CONFIG_X86_PAE conditional code is retained in its source-level form.
    if !(pgd_val(*pgd) & _PAGE_PRESENT) != 0 {
        pmd_table = alloc_low_page() as *mut pmd_t;
        set_pgd(pgd, __pgd(__pa(pmd_table) | _PAGE_PRESENT));
        p4d = p4d_offset(pgd, 0);
        pud = pud_offset(p4d, 0);
        BUG_ON(pmd_table != pmd_offset(pud, 0));
        return pmd_table;
    }
    p4d = p4d_offset(pgd, 0);
    pud = pud_offset(p4d, 0);
    pmd_table = pmd_offset(pud, 0);
    pmd_table
}

unsafe fn one_page_table_init(pmd: *mut pmd_t) -> *mut pte_t {
    if !(pmd_val(*pmd) & _PAGE_PRESENT) != 0 {
        let page_table = alloc_low_page() as *mut pte_t;
        set_pmd(pmd, __pmd(__pa(page_table) | _PAGE_TABLE));
        BUG_ON(page_table != pte_offset_kernel(pmd, 0));
    }
    pte_offset_kernel(pmd, 0)
}

pub unsafe fn populate_extra_pmd(vaddr: c_ulong) -> *mut pmd_t {
    let pgd_idx = pgd_index(vaddr);
    let pmd_idx = pmd_index(vaddr);
    one_md_table_init(swapper_pg_dir.add(pgd_idx as usize)).add(pmd_idx as usize)
}

pub unsafe fn populate_extra_pte(vaddr: c_ulong) -> *mut pte_t {
    let pte_idx = pte_index(vaddr);
    one_page_table_init(populate_extra_pmd(vaddr)).add(pte_idx as usize)
}

unsafe fn page_table_range_init_count(start: c_ulong, end: c_ulong) -> c_ulong {
    let mut count = 0;
    // CONFIG_HIGHMEM conditional code is retained in its source-level form.
    let pmd_idx_kmap_begin = fix_to_virt(FIX_KMAP_END) >> PMD_SHIFT;
    let pmd_idx_kmap_end = fix_to_virt(FIX_KMAP_BEGIN) >> PMD_SHIFT;
    if pmd_idx_kmap_begin == pmd_idx_kmap_end { return 0; }
    let mut vaddr = start;
    let mut pgd_idx = pgd_index(vaddr);
    let mut pmd_idx = pmd_index(vaddr);
    while pgd_idx < PTRS_PER_PGD && vaddr != end {
        while pmd_idx < PTRS_PER_PMD && vaddr != end {
            if (vaddr >> PMD_SHIFT) >= pmd_idx_kmap_begin &&
               (vaddr >> PMD_SHIFT) <= pmd_idx_kmap_end { count += 1; }
            vaddr += PMD_SIZE;
            pmd_idx += 1;
        }
        pmd_idx = 0;
        pgd_idx += 1;
    }
    count
}

unsafe fn page_table_kmap_check(mut pte: *mut pte_t, pmd: *mut pmd_t,
                                vaddr: c_ulong, lastpte: *mut pte_t,
                                adr: *mut *mut c_void) -> *mut pte_t {
    let begin = fix_to_virt(FIX_KMAP_END) >> PMD_SHIFT;
    let end = fix_to_virt(FIX_KMAP_BEGIN) >> PMD_SHIFT;
    if begin != end && (vaddr >> PMD_SHIFT) >= begin && (vaddr >> PMD_SHIFT) <= end {
        BUG_ON(after_bootmem);
        let newpte = *adr as *mut pte_t;
        for i in 0..PTRS_PER_PTE { set_pte(newpte.add(i as usize), *pte.add(i as usize)); }
        *adr = (*adr as c_ulong + PAGE_SIZE) as *mut c_void;
        set_pmd(pmd, __pmd(__pa(newpte) | _PAGE_TABLE));
        BUG_ON(newpte != pte_offset_kernel(pmd, 0));
        __flush_tlb_all();
        pte = newpte;
    }
    BUG_ON(vaddr < fix_to_virt(FIX_KMAP_BEGIN - 1) &&
           vaddr > fix_to_virt(FIX_KMAP_END) && !lastpte.is_null() &&
           lastpte.add(PTRS_PER_PTE as usize) != pte);
    pte
}

unsafe fn page_table_range_init(start: c_ulong, end: c_ulong, pgd_base: *mut pgd_t) {
    let mut pgd_idx = pgd_index(start);
    let mut pmd_idx = pmd_index(start);
    let mut vaddr = start;
    let mut pgd = pgd_base.add(pgd_idx as usize);
    let mut pte: *mut pte_t = core::ptr::null_mut();
    let count = page_table_range_init_count(start, end);
    let mut adr: *mut c_void = if count != 0 { alloc_low_pages(count) } else { core::ptr::null_mut() };
    while pgd_idx < PTRS_PER_PGD && vaddr != end {
        let mut pmd = one_md_table_init(pgd).add(pmd_index(vaddr) as usize);
        while pmd_idx < PTRS_PER_PMD && vaddr != end {
            pte = page_table_kmap_check(one_page_table_init(pmd), pmd, vaddr, pte, &mut adr);
            vaddr += PMD_SIZE; pmd = pmd.add(1); pmd_idx += 1;
        }
        pmd_idx = 0; pgd = pgd.add(1); pgd_idx += 1;
    }
}

unsafe fn is_x86_32_kernel_text(addr: c_ulong) -> i32 {
    if addr >= _text as c_ulong && addr <= __init_end as c_ulong { 1 } else { 0 }
}

pub unsafe fn kernel_physical_mapping_init(mut start: c_ulong, end: c_ulong,
                                           page_size_mask: c_ulong, prot: pgprot_t) -> c_ulong {
    let mut use_pse = page_size_mask == (1 << PG_LEVEL_2M);
    let mut last_map_addr = end;
    let start_pfn = start >> PAGE_SHIFT; let end_pfn = end >> PAGE_SHIFT;
    let pgd_base = swapper_pg_dir; let mut mapping_iter = 1;
    if !boot_cpu_has(X86_FEATURE_PSE) { use_pse = 0; }
    'repeat: loop {
        let mut pages_2m = 0; let mut pages_4k = 0; let mut pfn = start_pfn;
        let mut pgd_idx = pgd_index((pfn << PAGE_SHIFT) + PAGE_OFFSET);
        let mut pgd = pgd_base.add(pgd_idx as usize);
        while pgd_idx < PTRS_PER_PGD {
            let mut pmd = one_md_table_init(pgd);
            if pfn >= end_pfn { pgd = pgd.add(1); pgd_idx += 1; continue; }
            let mut pmd_idx = pmd_index((pfn << PAGE_SHIFT) + PAGE_OFFSET);
            pmd = pmd.add(pmd_idx as usize);
            while pmd_idx < PTRS_PER_PMD && pfn < end_pfn {
                let mut addr = pfn * PAGE_SIZE + PAGE_OFFSET;
                if use_pse != 0 {
                    let mut pprot = PAGE_KERNEL_LARGE;
                    let init_prot = __pgprot(PTE_IDENT_ATTR | _PAGE_PSE);
                    pfn &= PMD_MASK >> PAGE_SHIFT;
                    let addr2 = (pfn + PTRS_PER_PTE - 1) * PAGE_SIZE + PAGE_OFFSET + PAGE_SIZE - 1;
                    if is_x86_32_kernel_text(addr) != 0 || is_x86_32_kernel_text(addr2) != 0 { pprot = PAGE_KERNEL_LARGE_EXEC; }
                    pages_2m += 1;
                    set_pmd(pmd, pfn_pmd(pfn, if mapping_iter == 1 { init_prot } else { pprot }));
                    pfn += PTRS_PER_PTE; pmd = pmd.add(1); pmd_idx += 1; continue;
                }
                let mut pte = one_page_table_init(pmd).add(pte_index((pfn << PAGE_SHIFT) + PAGE_OFFSET) as usize);
                let mut pte_ofs = pte_index((pfn << PAGE_SHIFT) + PAGE_OFFSET);
                while pte_ofs < PTRS_PER_PTE && pfn < end_pfn {
                    let mut pprot = PAGE_KERNEL; let init_prot = __pgprot(PTE_IDENT_ATTR);
                    if is_x86_32_kernel_text(addr) != 0 { pprot = PAGE_KERNEL_EXEC; }
                    pages_4k += 1;
                    set_pte(pte, pfn_pte(pfn, if mapping_iter == 1 { init_prot } else { pprot }));
                    if mapping_iter == 1 { last_map_addr = (pfn << PAGE_SHIFT) + PAGE_SIZE; }
                    pte = pte.add(1); pfn += 1; pte_ofs += 1; addr += PAGE_SIZE;
                }
                pmd = pmd.add(1); pmd_idx += 1;
            }
            pgd = pgd.add(1); pgd_idx += 1;
        }
        if mapping_iter == 1 { update_page_count(PG_LEVEL_2M, pages_2m); update_page_count(PG_LEVEL_4K, pages_4k); __flush_tlb_all(); mapping_iter = 2; continue 'repeat; }
        return last_map_addr;
    }
}

unsafe fn permanent_kmaps_init(pgd_base: *mut pgd_t) {
    let vaddr = PKMAP_BASE; page_table_range_init(vaddr, vaddr + PAGE_SIZE * LAST_PKMAP, pgd_base); pkmap_page_table = virt_to_kpte(vaddr);
}

pub unsafe fn sync_initial_page_table() {
    clone_pgd_range(initial_page_table.add(KERNEL_PGD_BOUNDARY as usize), swapper_pg_dir.add(KERNEL_PGD_BOUNDARY as usize), KERNEL_PGD_PTRS);
    clone_pgd_range(initial_page_table, swapper_pg_dir.add(KERNEL_PGD_BOUNDARY as usize), min(KERNEL_PGD_PTRS, KERNEL_PGD_BOUNDARY));
}

pub unsafe fn native_pagetable_init() {
    let base = swapper_pg_dir; let mut pfn = max_low_pfn; let limit = 1 << (32 - PAGE_SHIFT);
    while pfn < limit {
        let va = PAGE_OFFSET + (pfn << PAGE_SHIFT); let pgd = base.add(pgd_index(va) as usize);
        if !pgd_present(*pgd) { break; } let p4d = p4d_offset(pgd, va); let pud = pud_offset(p4d, va); let pmd = pmd_offset(pud, va);
        if !pmd_present(*pmd) { break; } if pmd_leaf(*pmd) { pr_warn!("try to clear pte for ram above max_low_pfn: pfn: %lx pmd: %p pmd phys: %lx, but pmd is big page and is not using pte !\n", pfn, pmd, __pa(pmd)); BUG_ON(1); }
        let pte = pte_offset_kernel(pmd, va); if !pte_present(*pte) { break; } printk!(KERN_DEBUG, "clearing pte for ram above max_low_pfn: pfn: %lx pmd: %p pmd phys: %lx pte: %p pte phys: %lx\n", pfn, pmd, __pa(pmd), pte, __pa(pte)); pte_clear(core::ptr::null_mut(), va, pte); pfn += 1;
    }
    paging_init();
}

pub unsafe fn early_ioremap_page_table_range_init() {
    let vaddr = __fix_to_virt(__end_of_fixed_addresses - 1) & PMD_MASK; let end = (FIXADDR_TOP + PMD_SIZE - 1) & PMD_MASK;
    page_table_range_init(vaddr, end, swapper_pg_dir); early_ioremap_reset();
}

unsafe fn pagetable_init() { permanent_kmaps_init(swapper_pg_dir); }

pub const DEFAULT_PTE_MASK: pteval_t = !(_PAGE_NX | _PAGE_GLOBAL);
pub static mut __supported_pte_mask: pteval_t = DEFAULT_PTE_MASK;
pub static mut __default_kernel_pte_mask: pteval_t = DEFAULT_PTE_MASK;
static mut highmem_pages: c_uint = u32::MAX;

unsafe fn parse_highmem(arg: *mut c_char) -> i32 { if arg.is_null() { return -EINVAL; } highmem_pages = memparse(arg, &mut arg) >> PAGE_SHIFT; 0 }

unsafe fn lowmem_pfn_init() {
    max_low_pfn = max_pfn; if highmem_pages == u32::MAX { highmem_pages = 0; }
    if highmem_pages >= max_pfn { printk!(KERN_ERR, "highmem size (%luMB) is bigger than pages available (%luMB)!\n", pages_to_mb(highmem_pages), pages_to_mb(max_pfn)); highmem_pages = 0; }
    if highmem_pages != 0 { if max_low_pfn - highmem_pages < 64 * 1024 * 1024 / PAGE_SIZE { printk!(KERN_ERR, "highmem size (%luMB) results in <64MB lowmem, ignoring it!\n", pages_to_mb(highmem_pages)); highmem_pages = 0; } max_low_pfn -= highmem_pages; }
}

unsafe fn highmem_pfn_init() {
    max_low_pfn = MAXMEM_PFN; if highmem_pages == u32::MAX { highmem_pages = max_pfn - MAXMEM_PFN; }
    if highmem_pages + MAXMEM_PFN < max_pfn { max_pfn = MAXMEM_PFN + highmem_pages; }
    if highmem_pages + MAXMEM_PFN > max_pfn { printk!(KERN_WARNING, "only %luMB highmem pages available, ignoring highmem size of %luMB!\n", pages_to_mb(max_pfn - MAXMEM_PFN), pages_to_mb(highmem_pages)); highmem_pages = 0; }
    if max_pfn > MAX_NONPAE_PFN { max_pfn = MAX_NONPAE_PFN; printk!(KERN_WARNING, "Warning: only 4GB will be used. Support for CONFIG_HIGHMEM64G was removed!\n"); }
}

pub unsafe fn find_low_pfn_range() { if max_pfn <= MAXMEM_PFN { lowmem_pfn_init(); } else { highmem_pfn_init(); } }

pub unsafe fn initmem_init() {
    highstart_pfn = max_pfn; highend_pfn = max_pfn; if max_pfn > max_low_pfn { highstart_pfn = max_low_pfn; }
    printk!(KERN_NOTICE, "%ldMB HIGHMEM available.\n", pages_to_mb(highend_pfn - highstart_pfn)); high_memory = (__va(highstart_pfn * PAGE_SIZE - 1) as c_ulong + 1) as *mut c_void;
    memblock_set_node(0, PHYS_ADDR_MAX, &mut memblock.memory, 0); __vmalloc_start_set = true;
    printk!(KERN_NOTICE, "%ldMB LOWMEM available.\n", pages_to_mb(max_low_pfn)); printk!(KERN_INFO, "  mapped low ram: 0 - %08lx\n", max_pfn_mapped << PAGE_SHIFT); printk!(KERN_INFO, "  low ram: 0 - %08lx\n", max_low_pfn << PAGE_SHIFT);
}

pub unsafe fn paging_init() { pagetable_init(); __flush_tlb_all(); olpc_dt_build_devicetree(); }

unsafe fn test_wp_bit() {
    let mut z: c_char = 0; printk!(KERN_INFO, "Checking if this processor honours the WP bit even in supervisor mode..."); __set_fixmap(FIX_WP_TEST, __pa_symbol(empty_zero_page), PAGE_KERNEL_RO);
    if copy_to_kernel_nofault(fix_to_virt(FIX_WP_TEST) as *mut c_char, &mut z, 1) != 0 { clear_fixmap(FIX_WP_TEST); printk!(KERN_CONT, "Ok.\n"); return; }
    printk!(KERN_CONT, "No.\n"); panic!("Linux doesn't support CPUs with broken WP.");
}

pub unsafe fn arch_mm_preinit() { pci_iommu_alloc(); BUG_ON(!mem_map); }
pub static mut kernel_set_to_readonly: i32 = 0;

unsafe fn mark_nxdata_nx() {
    let start = PFN_ALIGN(_etext); let size = (((__init_end as c_ulong + HPAGE_SIZE) & HPAGE_MASK) - start);
    if __supported_pte_mask & _PAGE_NX != 0 { printk!(KERN_INFO, "NX-protecting the kernel data: %luk\n", size >> 10); } set_memory_nx(start, size >> PAGE_SHIFT);
}

pub unsafe fn mark_rodata_ro() {
    let start = PFN_ALIGN(_text); let size = __end_rodata as c_ulong - start; set_pages_ro(virt_to_page(start), size >> PAGE_SHIFT); pr_info!("Write protecting kernel text and read-only data: %luk\n", size >> 10); kernel_set_to_readonly = 1;
    set_pages_rw(virt_to_page(start), size >> PAGE_SHIFT); pr_info!("Testing CPA: write protecting again\n"); set_pages_ro(virt_to_page(start), size >> PAGE_SHIFT); mark_nxdata_nx();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
