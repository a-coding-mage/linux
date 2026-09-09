// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/sh/mm/init.c
 *
 *  Copyright (C) 1999  Niibe Yutaka
 *  Copyright (C) 2002 - 2011  Paul Mundt
 *
 *  Based on linux/arch/i386/mm/init.c:
 *   Copyright (C) 1995  Linus Torvalds
 */
// C dependencies: linux/mm.h, linux/swap.h, linux/init.h, linux/gfp.h,
// linux/memblock.h, linux/proc_fs.h, linux/pagemap.h, linux/percpu.h,
// linux/io.h, linux/dma-mapping.h, linux/export.h, and the corresponding
// asm headers plus "ioremap.h".

pub static mut swapper_pg_dir: [pgd_t; PTRS_PER_PGD] = [unsafe { core::mem::zeroed() }; PTRS_PER_PGD];

pub unsafe extern "C" fn generic_mem_init() {
    memblock_add(__MEMORY_START, __MEMORY_SIZE);
}

pub unsafe extern "C" fn plat_mem_setup() {
    /* Nothing to see here, move along. */
}

#[cfg(CONFIG_MMU)]
unsafe fn __get_pte_phys(addr: c_ulong) -> *mut pte_t {
    let pgd = pgd_offset_k(addr);
    if pgd_none(*pgd) {
        pgd_ERROR(*pgd);
        return core::ptr::null_mut();
    }

    let p4d = p4d_alloc(core::ptr::null_mut(), pgd, addr);
    if p4d.is_null() {
        p4d_ERROR(*p4d);
        return core::ptr::null_mut();
    }
    let pud = pud_alloc(core::ptr::null_mut(), p4d, addr);
    if pud.is_null() {
        pud_ERROR(*pud);
        return core::ptr::null_mut();
    }
    let pmd = pmd_alloc(core::ptr::null_mut(), pud, addr);
    if pmd.is_null() {
        pmd_ERROR(*pmd);
        return core::ptr::null_mut();
    }
    pte_offset_kernel(pmd, addr)
}

#[cfg(CONFIG_MMU)]
unsafe fn set_pte_phys(addr: c_ulong, phys: c_ulong, prot: pgprot_t) {
    let pte = __get_pte_phys(addr);
    if !pte_none(*pte) {
        pte_ERROR(*pte);
        return;
    }
    set_pte(pte, pfn_pte(phys >> PAGE_SHIFT, prot));
    local_flush_tlb_one(get_asid(), addr);
    if pgprot_val(prot) & _PAGE_WIRED != 0 {
        tlb_wire_entry(core::ptr::null_mut(), addr, *pte);
    }
}

#[cfg(CONFIG_MMU)]
unsafe fn clear_pte_phys(addr: c_ulong, prot: pgprot_t) {
    let pte = __get_pte_phys(addr);
    if pgprot_val(prot) & _PAGE_WIRED != 0 { tlb_unwire_entry(); }
    set_pte(pte, pfn_pte(0, __pgprot(0)));
    local_flush_tlb_one(get_asid(), addr);
}

#[cfg(CONFIG_MMU)]
pub unsafe extern "C" fn __set_fixmap(idx: fixed_addresses, phys: c_ulong, prot: pgprot_t) {
    let address = __fix_to_virt(idx);
    if idx >= __end_of_fixed_addresses { BUG(); return; }
    set_pte_phys(address, phys, prot);
}

#[cfg(CONFIG_MMU)]
pub unsafe extern "C" fn __clear_fixmap(idx: fixed_addresses, prot: pgprot_t) {
    let address = __fix_to_virt(idx);
    if idx >= __end_of_fixed_addresses { BUG(); return; }
    clear_pte_phys(address, prot);
}

#[cfg(CONFIG_MMU)]
unsafe fn one_md_table_init(pud: *mut pud_t) -> *mut pmd_t {
    if pud_none(*pud) {
        let pmd = memblock_alloc_or_panic(PAGE_SIZE, PAGE_SIZE);
        pud_populate(&init_mm, pud, pmd);
        BUG_ON(pmd != pmd_offset(pud, 0));
    }
    pmd_offset(pud, 0)
}

#[cfg(CONFIG_MMU)]
unsafe fn one_page_table_init(pmd: *mut pmd_t) -> *mut pte_t {
    if pmd_none(*pmd) {
        let pte = memblock_alloc_or_panic(PAGE_SIZE, PAGE_SIZE);
        pmd_populate_kernel(&init_mm, pmd, pte);
        BUG_ON(pte != pte_offset_kernel(pmd, 0));
    }
    pte_offset_kernel(pmd, 0)
}

#[cfg(CONFIG_MMU)]
unsafe fn page_table_kmap_check(pte: *mut pte_t, _pmd: *mut pmd_t, _vaddr: c_ulong, _lastpte: *mut pte_t) -> *mut pte_t { pte }

#[cfg(CONFIG_MMU)]
pub unsafe extern "C" fn page_table_range_init(mut start: c_ulong, end: c_ulong, pgd_base: *mut pgd_t) {
    let mut vaddr = start;
    let mut i = pgd_index(vaddr);
    let mut j = pud_index(vaddr);
    let mut k = pmd_index(vaddr);
    let mut pgd = pgd_base.add(i as usize);
    let mut pte: *mut pte_t = core::ptr::null_mut();
    while i < PTRS_PER_PGD && vaddr != end {
        let mut pud = pgd as *mut pud_t;
        while j < PTRS_PER_PUD && vaddr != end {
            let mut pmd = one_md_table_init(pud);
            #[cfg(not(__PAGETABLE_PMD_FOLDED))]
            { pmd = pmd.add(k as usize); }
            while k < PTRS_PER_PMD && vaddr != end {
                pte = page_table_kmap_check(one_page_table_init(pmd), pmd, vaddr, pte);
                vaddr += PMD_SIZE;
                pmd = pmd.add(1);
                k += 1;
            }
            k = 0; pud = pud.add(1); j += 1;
        }
        j = 0; pgd = pgd.add(1); i += 1;
    }
}

pub unsafe extern "C" fn allocate_pgdat(nid: c_uint) {
    let (mut start_pfn, mut end_pfn) = (0, 0);
    get_pfn_range_for_nid(nid, &mut start_pfn, &mut end_pfn);
    #[cfg(CONFIG_NUMA)] alloc_node_data(nid);
    (*NODE_DATA(nid)).node_start_pfn = start_pfn;
    (*NODE_DATA(nid)).node_spanned_pages = end_pfn - start_pfn;
}

unsafe fn do_init_bootmem() {
    let (mut start_pfn, mut end_pfn) = (0, 0);
    let mut i = 0;
    for_each_mem_pfn_range!(i, MAX_NUMNODES, &mut start_pfn, &mut end_pfn, core::ptr::null_mut(), {
        __add_active_range(0, start_pfn, end_pfn);
    });
    allocate_pgdat(0);
    node_set_online(0);
    plat_mem_setup();
}

unsafe fn early_reserve_mem() {
    let zero_base = (__MEMORY_START as u32).wrapping_add(PHYSICAL_OFFSET as u32);
    let start = zero_base.wrapping_add(CONFIG_ZERO_PAGE_OFFSET as u32);
    let start_pfn = PFN_UP(__pa(_end));
    memblock_reserve(start as c_ulong, (PFN_PHYS(start_pfn) + PAGE_SIZE - 1) - start as c_ulong);
    if CONFIG_ZERO_PAGE_OFFSET != 0 { memblock_reserve(zero_base as c_ulong, CONFIG_ZERO_PAGE_OFFSET); }
    check_for_initrd();
    reserve_crashkernel();
}

pub unsafe extern "C" fn arch_zone_limits_init(max_zone_pfns: *mut c_ulong) { *max_zone_pfns.add(ZONE_NORMAL as usize) = max_low_pfn; }

pub unsafe extern "C" fn paging_init() {
    sh_mv.mv_mem_init();
    early_reserve_mem();
    if let Some(reserve) = sh_mv.mv_mem_reserve { reserve(); }
    memblock_enforce_memory_limit(memory_limit);
    memblock_allow_resize();
    memblock_dump_all();
    max_low_pfn = memblock_end_of_DRAM() >> PAGE_SHIFT;
    max_pfn = max_low_pfn;
    min_low_pfn = __MEMORY_START >> PAGE_SHIFT;
    nodes_clear(node_online_map);
    memory_start = __va(__MEMORY_START) as c_ulong;
    memory_end = memory_start + if memory_limit != 0 { memory_limit } else { memblock_phys_mem_size() };
    uncached_init(); pmb_init(); do_init_bootmem(); ioremap_fixed_init();
    core::ptr::write_bytes(swapper_pg_dir.as_mut_ptr(), 0, swapper_pg_dir.len());
    set_TTB(swapper_pg_dir.as_mut_ptr());
    let vaddr = __fix_to_virt(__end_of_fixed_addresses - 1) & PMD_MASK;
    let end = (FIXADDR_TOP + PMD_SIZE - 1) & PMD_MASK;
    page_table_range_init(vaddr, end, swapper_pg_dir.as_mut_ptr());
    kmap_coherent_init();
}

pub static mut mem_init_done: c_uint = 0;

pub unsafe extern "C" fn mem_init() {
    cpu_cache_init();
    vsyscall_init();
    pr_info!(
        "virtual kernel memory layout:\n"
        "    fixmap  : 0x{:08x} - 0x{:08x}   ({:4} kB)\n"
        "    vmalloc : 0x{:08x} - 0x{:08x}   ({:4} MB)\n"
        "    lowmem  : 0x{:08x} - 0x{:08x}   ({:4} MB) (cached)\n"
        "      .init : 0x{:08x} - 0x{:08x}   ({:4} kB)\n"
        "      .data : 0x{:08x} - 0x{:08x}   ({:4} kB)\n"
        "      .text : 0x{:08x} - 0x{:08x}   ({:4} kB)\n",
        FIXADDR_START, FIXADDR_TOP, (FIXADDR_TOP - FIXADDR_START) >> 10,
        VMALLOC_START as c_ulong, VMALLOC_END, (VMALLOC_END - VMALLOC_START) >> 20,
        memory_start, high_memory, (high_memory - memory_start) >> 20,
        &__init_begin as *const _ as c_ulong, &__init_end as *const _ as c_ulong,
        ((&__init_end as *const _ as c_ulong) - (&__init_begin as *const _ as c_ulong)) >> 10,
        &_etext as *const _ as c_ulong, &_edata as *const _ as c_ulong,
        ((&_edata as *const _ as c_ulong) - (&_etext as *const _ as c_ulong)) >> 10,
        &_text as *const _ as c_ulong, &_etext as *const _ as c_ulong,
        ((&_etext as *const _ as c_ulong) - (&_text as *const _ as c_ulong)) >> 10,
    );
    mem_init_done = 1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
