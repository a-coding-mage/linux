/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1994 - 2000 Ralf Baechle
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 * Kevin D. Kissell, kevink@mips.com and Carsten Langgaard, carstenl@mips.com
 * Copyright (C) 2000 MIPS Technologies, Inc. All rights reserved.
 */

// Linux header dependencies are supplied by the surrounding translation unit.

pub static mut empty_zero_page: c_ulong = 0;
pub static mut zero_page_mask: c_ulong = 0;

pub unsafe fn arch_setup_zero_pages() {
    let order: c_uint = if cpu_has_vce { 3 } else { 0 };
    empty_zero_page = memblock_alloc_or_panic(PAGE_SIZE << order, PAGE_SIZE) as c_ulong;
    zero_page_mask = ((PAGE_SIZE << order) - 1) & PAGE_MASK;
}

unsafe fn __kmap_pgprot(page: *mut page, addr: c_ulong, prot: pgprot_t) -> *mut c_void {
    let mut idx: enum_fixed_addresses;
    let mut old_mmid: c_uint = 0;
    let mut vaddr: c_ulong;
    let mut flags: c_ulong = 0;
    let mut entrylo: c_ulong;
    let old_ctx: c_ulong;
    let pte: pte_t;
    let tlbidx: c_int;

    BUG_ON(folio_test_dcache_dirty(page_folio(page)));
    preempt_disable();
    pagefault_disable();
    idx = ((addr >> PAGE_SHIFT) & (FIX_N_COLOURS - 1)) as enum_fixed_addresses;
    idx = (idx as c_uint + if in_interrupt() { FIX_N_COLOURS } else { 0 }) as enum_fixed_addresses;
    vaddr = __fix_to_virt(FIX_CMAP_END - idx as c_int);
    pte = mk_pte(page, prot);
    entrylo = pte_to_entrylo(pte.pte_high);

    local_irq_save(&mut flags);
    old_ctx = read_c0_entryhi();
    write_c0_entryhi(vaddr & (PAGE_MASK << 1));
    write_c0_entrylo0(entrylo);
    write_c0_entrylo1(entrylo);
    if cpu_has_mmid {
        old_mmid = read_c0_memorymapid();
        write_c0_memorymapid(MMID_KERNEL_WIRED);
    }
    if cpu_has_xpa {
        entrylo = pte.pte_low & _PFNX_MASK;
        writex_c0_entrylo0(entrylo);
        writex_c0_entrylo1(entrylo);
    }
    tlbidx = num_wired_entries();
    write_c0_wired(tlbidx + 1);
    write_c0_index(tlbidx);
    mtc0_tlbw_hazard();
    tlb_write_indexed();
    tlbw_use_hazard();
    write_c0_entryhi(old_ctx);
    if cpu_has_mmid { write_c0_memorymapid(old_mmid); }
    local_irq_restore(flags);
    vaddr as *mut c_void
}

pub unsafe fn kmap_coherent(page: *mut page, addr: c_ulong) -> *mut c_void { __kmap_pgprot(page, addr, PAGE_KERNEL) }
pub unsafe fn kmap_noncoherent(page: *mut page, addr: c_ulong) -> *mut c_void { __kmap_pgprot(page, addr, PAGE_KERNEL_NC) }

pub unsafe fn kunmap_coherent() {
    let mut flags: c_ulong = 0;
    let old_ctx: c_ulong;
    local_irq_save(&mut flags);
    old_ctx = read_c0_entryhi();
    let wired = num_wired_entries() - 1;
    write_c0_wired(wired);
    write_c0_index(wired);
    write_c0_entryhi(UNIQUE_ENTRYHI(wired));
    write_c0_entrylo0(0);
    write_c0_entrylo1(0);
    mtc0_tlbw_hazard(); tlb_write_indexed(); tlbw_use_hazard();
    write_c0_entryhi(old_ctx);
    local_irq_restore(flags); pagefault_enable(); preempt_enable();
}

pub unsafe fn copy_user_highpage(to: *mut page, from: *mut page, vaddr: c_ulong, _vma: *mut vm_area_struct) {
    let src = page_folio(from);
    let vto = kmap_atomic(to);
    if cpu_has_dc_aliases && folio_mapped(src) && !folio_test_dcache_dirty(src) {
        let vfrom = kmap_coherent(from, vaddr); copy_page(vto, vfrom); kunmap_coherent();
    } else { let vfrom = kmap_atomic(from); copy_page(vto, vfrom); kunmap_atomic(vfrom); }
    if !cpu_has_ic_fills_f_dc || pages_do_alias(vto as c_ulong, vaddr & PAGE_MASK) { flush_data_cache_page(vto as c_ulong); }
    kunmap_atomic(vto); smp_wmb();
}

pub unsafe fn copy_to_user_page(vma: *mut vm_area_struct, page: *mut page, vaddr: c_ulong, dst: *mut c_void, src: *const c_void, len: c_ulong) {
    let folio = page_folio(page);
    if cpu_has_dc_aliases && folio_mapped(folio) && !folio_test_dcache_dirty(folio) {
        let vto = (kmap_coherent(page, vaddr) as *mut u8).add((vaddr & !PAGE_MASK) as usize) as *mut c_void;
        memcpy(vto, src, len); kunmap_coherent();
    } else { memcpy(dst, src, len); if cpu_has_dc_aliases { folio_set_dcache_dirty(folio); } }
    if (*vma).vm_flags & VM_EXEC != 0 { flush_cache_page(vma, vaddr, page_to_pfn(page)); }
}

pub unsafe fn copy_from_user_page(vma: *mut vm_area_struct, page: *mut page, vaddr: c_ulong, dst: *mut c_void, src: *const c_void, len: c_ulong) {
    let folio = page_folio(page);
    if cpu_has_dc_aliases && folio_mapped(folio) && !folio_test_dcache_dirty(folio) {
        let vfrom = (kmap_coherent(page, vaddr) as *mut u8).add((vaddr & !PAGE_MASK) as usize) as *const c_void;
        memcpy(dst, vfrom, len); kunmap_coherent();
    } else { memcpy(dst, src, len); if cpu_has_dc_aliases { folio_set_dcache_dirty(folio); } }
}

pub unsafe fn fixrange_init(start: c_ulong, end: c_ulong, pgd_base: *mut pgd_t) {
    // CONFIG_HIGHMEM-gated page-table initialization is preserved here.
    #[cfg(CONFIG_HIGHMEM)] {
        let mut vaddr = start; let mut i = pgd_index(vaddr); let mut j = pud_index(vaddr); let mut k = pmd_index(vaddr);
        let mut pgd = pgd_base.add(i as usize);
        while i < PTRS_PER_PGD && vaddr < end { let mut pud = pgd as *mut pud_t;
            while j < PTRS_PER_PUD && vaddr < end { let mut pmd = pud as *mut pmd_t;
                while k < PTRS_PER_PMD && vaddr < end { if pmd_none(*pmd) { let pte = memblock_alloc_low(PAGE_SIZE, PAGE_SIZE); if pte.is_null() { panic!("fixrange_init: allocation failed"); } set_pmd(pmd, __pmd(pte as c_ulong)); BUG_ON(pte != pte_offset_kernel(pmd, 0)); } vaddr += PMD_SIZE; pmd = pmd.add(1); k += 1; }
                k = 0; pud = pud.add(1); j += 1;
            } j = 0; pgd = pgd.add(1); i += 1;
        }
    }
}

#[repr(C)] pub struct maar_walk_info { pub cfg: [maar_config; 16], pub num_cfg: c_uint }

unsafe fn maar_res_walk(start_pfn: c_ulong, nr_pages: c_ulong, data: *mut c_void) -> c_int {
    let wi = &mut *(data as *mut maar_walk_info);
    if WARN_ON(wi.num_cfg >= wi.cfg.len()) { return -1; }
    let cfg = &mut wi.cfg[wi.num_cfg as usize];
    let maar_align = BIT(MIPS_MAAR_ADDR_SHIFT + 4);
    cfg.lower = ALIGN(PFN_PHYS(start_pfn), maar_align);
    cfg.upper = ALIGN_DOWN(PFN_PHYS(start_pfn + nr_pages), maar_align) - 1;
    cfg.attrs = MIPS_MAAR_S; wi.num_cfg += 1; 0
}

pub unsafe fn platform_maar_init(num_pairs: c_uint) -> c_uint {
    let mut wi = maar_walk_info { cfg: [core::mem::zeroed(); 16], num_cfg: 0 };
    walk_system_ram_range(0, max_pfn, &mut wi as *mut _ as *mut c_void, maar_res_walk);
    let n = maar_config(wi.cfg.as_mut_ptr(), wi.num_cfg, num_pairs);
    if n < wi.num_cfg { pr_warn!("Not enough MAAR pairs ({}) for all memory regions ({})", num_pairs, wi.num_cfg); } n
}

pub unsafe fn maar_init() {
    if !cpu_has_maar { return; }
    write_c0_maari(!0); back_to_back_c0_hazard();
    let num_maars = read_c0_maari() + 1; WARN_ON(num_maars % 2);
    let used = platform_maar_init(num_maars / 2);
    let mut i = used * 2; while i < num_maars { write_c0_maari(i); back_to_back_c0_hazard(); write_c0_maar(0); back_to_back_c0_hazard(); i += 1; }
    pr_info!("MAAR configuration:\n");
    i = 0;
    while i < num_maars { write_c0_maari(i); back_to_back_c0_hazard(); let mut upper = read_c0_maar() as phys_addr_t; write_c0_maari(i + 1); back_to_back_c0_hazard(); let mut lower = read_c0_maar() as phys_addr_t;
        let attr = lower & upper; lower = (lower & MIPS_MAAR_ADDR) << 4; upper = ((upper & MIPS_MAAR_ADDR) << 4) | 0xffff;
        pr_info!("  [{}]: ", i / 2); if attr & MIPS_MAAR_V != MIPS_MAAR_V { pr_cont!("disabled\n"); i += 2; continue; }
        pr_cont!("%pa-%pa", &lower, &upper); if attr & MIPS_MAAR_S != 0 { pr_cont!(" speculate"); } pr_cont!("\n"); i += 2;
    }
}

#[cfg(not(CONFIG_NUMA))]
pub unsafe fn arch_zone_limits_init(max_zone_pfns: *mut c_ulong) {
    #[cfg(CONFIG_ZONE_DMA)] { *max_zone_pfns.add(ZONE_DMA as usize) = MAX_DMA_PFN; }
    #[cfg(CONFIG_ZONE_DMA32)] { *max_zone_pfns.add(ZONE_DMA32 as usize) = MAX_DMA32_PFN; }
    *max_zone_pfns.add(ZONE_NORMAL as usize) = max_low_pfn;
    #[cfg(CONFIG_HIGHMEM)] { *max_zone_pfns.add(ZONE_HIGHMEM as usize) = highend_pfn; if cpu_has_dc_aliases && max_low_pfn != highend_pfn { *max_zone_pfns.add(ZONE_HIGHMEM as usize) = max_low_pfn; } }
}

#[cfg(not(CONFIG_NUMA))]
unsafe fn highmem_init() {
    #[cfg(CONFIG_HIGHMEM)] { if cpu_has_dc_aliases { if highstart_pfn != 0 { memblock_remove(PFN_PHYS(highstart_pfn), !0); } return; } let mut tmp = highstart_pfn; while tmp < highend_pfn { let page = pfn_to_page(tmp); if !memblock_is_memory(PFN_PHYS(tmp)) { SetPageReserved(page); } tmp += 1; } }
}

#[cfg(not(CONFIG_NUMA))]
pub unsafe fn arch_mm_preinit() {
    BUILD_BUG_ON!(IS_ENABLED(CONFIG_32BIT) && PFN_PTE_SHIFT > PAGE_SHIFT);
    maar_init(); highmem_init();
}

pub unsafe fn free_init_pages(what: *const c_char, begin: c_ulong, end: c_ulong) {
    let mut pfn = PFN_UP(begin); while pfn < PFN_DOWN(end) { let page = pfn_to_page(pfn); let addr = phys_to_virt(PFN_PHYS(pfn)); memset(addr, POISON_FREE_INITMEM, PAGE_SIZE); free_reserved_page(page); pfn += 1; }
    printk!(KERN_INFO, "Freeing %s: %ldk freed\n", what, (end - begin) >> 10);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
