// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2023 Loongson Technology Corporation Limited
 */

// C preprocessor configuration and external headers are supplied by the
// surrounding kernel translation unit.

static mut kasan_pg_dir: [pgd_t; PTRS_PER_PGD] = [/* __initdata, __aligned(PAGE_SIZE) */ unsafe { core::mem::zeroed() }; PTRS_PER_PGD];

#[inline]
unsafe fn __pgd_none(early: bool, pgd: pgd_t) -> bool {
    if cfg!(feature = "__PAGETABLE_P4D_FOLDED") { false }
    else if early { pgd_val(pgd) == 0 }
    else { __pa(pgd_val(pgd)) == __pa(kasan_early_shadow_p4d) as unsigned_long }
}

#[inline]
unsafe fn __p4d_none(early: bool, p4d: p4d_t) -> bool {
    if cfg!(feature = "__PAGETABLE_PUD_FOLDED") { false }
    else if early { p4d_val(p4d) == 0 }
    else { __pa(p4d_val(p4d)) == __pa(kasan_early_shadow_pud) as unsigned_long }
}

#[inline]
unsafe fn __pud_none(early: bool, pud: pud_t) -> bool {
    if cfg!(feature = "__PAGETABLE_PMD_FOLDED") { false }
    else if early { pud_val(pud) == 0 }
    else { __pa(pud_val(pud)) == __pa(kasan_early_shadow_pmd) as unsigned_long }
}

#[inline]
unsafe fn __pmd_none(early: bool, pmd: pmd_t) -> bool {
    if early { pmd_val(pmd) == 0 }
    else { __pa(pmd_val(pmd)) == __pa(kasan_early_shadow_pte) as unsigned_long }
}

#[inline]
unsafe fn __pte_none(early: bool, pte: pte_t) -> bool {
    if early { pte_none(pte) }
    else { (pte_val(pte) & _PFN_MASK) == __pa(kasan_early_shadow_page) as unsigned_long }
}

unsafe fn mem_to_shadow(addr: *const core::ffi::c_void) -> *mut core::ffi::c_void {
    let mut offset: unsigned_long = 0;
    let mut maddr = addr as unsigned_long;
    let xrange = (maddr >> XRANGE_SHIFT) & 0xffff;
    if maddr >= FIXADDR_START { return kasan_early_shadow_page as *mut _; }
    maddr &= XRANGE_SHADOW_MASK;
    offset = match xrange {
        XKPRANGE_CC_SEG => XKPRANGE_CC_SHADOW_OFFSET,
        XKPRANGE_UC_SEG => XKPRANGE_UC_SHADOW_OFFSET,
        XKPRANGE_WC_SEG => XKPRANGE_WC_SHADOW_OFFSET,
        XKVRANGE_VC_SEG => XKVRANGE_VC_SHADOW_OFFSET,
        _ => { WARN_ON(1); return core::ptr::null_mut(); }
    };
    ((maddr >> KASAN_SHADOW_SCALE_SHIFT) + offset) as *mut _
}

pub unsafe fn kasan_mem_to_shadow(addr: *const core::ffi::c_void) -> *mut core::ffi::c_void {
    if kasan_enabled() { mem_to_shadow(addr) } else { kasan_early_shadow_page as *mut _ }
}

pub unsafe fn kasan_shadow_to_mem(shadow_addr: *const core::ffi::c_void) -> *const core::ffi::c_void {
    let addr = shadow_addr as unsigned_long;
    if addr > KASAN_SHADOW_END || addr < KASAN_SHADOW_START { WARN_ON(1); return core::ptr::null(); }
    if addr >= XKVRANGE_VC_SHADOW_OFFSET { return (((addr - XKVRANGE_VC_SHADOW_OFFSET) << KASAN_SHADOW_SCALE_SHIFT) + XKVRANGE_VC_START) as *const _; }
    if addr >= XKPRANGE_WC_SHADOW_OFFSET { return (((addr - XKPRANGE_WC_SHADOW_OFFSET) << KASAN_SHADOW_SCALE_SHIFT) + XKPRANGE_WC_START) as *const _; }
    if addr >= XKPRANGE_UC_SHADOW_OFFSET { return (((addr - XKPRANGE_UC_SHADOW_OFFSET) << KASAN_SHADOW_SCALE_SHIFT) + XKPRANGE_UC_START) as *const _; }
    if addr >= XKPRANGE_CC_SHADOW_OFFSET { return (((addr - XKPRANGE_CC_SHADOW_OFFSET) << KASAN_SHADOW_SCALE_SHIFT) + XKPRANGE_CC_START) as *const _; }
    WARN_ON(1); core::ptr::null()
}

/* Alloc memory for shadow memory page table. */
unsafe fn kasan_alloc_zeroed_page(node: i32) -> phys_addr_t {
    let p = memblock_alloc_try_nid(PAGE_SIZE, PAGE_SIZE, __pa(MAX_DMA_ADDRESS), MEMBLOCK_ALLOC_ACCESSIBLE, node);
    if p.is_null() { panic!("{}: Failed to allocate {} bytes align=0x{:x} nid={} from={:x}\n", "kasan_alloc_zeroed_page", PAGE_SIZE, PAGE_SIZE, node, __pa(MAX_DMA_ADDRESS)); }
    __pa(p)
}

unsafe fn kasan_pte_offset(pmdp: *mut pmd_t, addr: unsigned_long, node: i32, early: bool) -> *mut pte_t {
    if __pmd_none(early, pmdp_get(pmdp)) { let p = if early { __pa_symbol(kasan_early_shadow_pte) } else { kasan_alloc_zeroed_page(node) }; if !early { memcpy(__va(p), kasan_early_shadow_pte, core::mem::size_of_val(&kasan_early_shadow_pte)); } pmd_populate_kernel(core::ptr::null_mut(), pmdp, __va(p) as *mut _); }
    pte_offset_kernel(pmdp, addr)
}

unsafe fn kasan_pmd_offset(pudp: *mut pud_t, addr: unsigned_long, node: i32, early: bool) -> *mut pmd_t {
    if __pud_none(early, pudp_get(pudp)) { let p = if early { __pa_symbol(kasan_early_shadow_pmd) } else { kasan_alloc_zeroed_page(node) }; if !early { memcpy(__va(p), kasan_early_shadow_pmd, core::mem::size_of_val(&kasan_early_shadow_pmd)); } pud_populate(&mut init_mm, pudp, __va(p) as *mut _); }
    pmd_offset(pudp, addr)
}

unsafe fn kasan_pud_offset(p4dp: *mut p4d_t, addr: unsigned_long, node: i32, early: bool) -> *mut pud_t {
    if __p4d_none(early, p4dp_get(p4dp)) { let p = if early { __pa_symbol(kasan_early_shadow_pud) } else { kasan_alloc_zeroed_page(node) }; if !early { memcpy(__va(p), kasan_early_shadow_pud, core::mem::size_of_val(&kasan_early_shadow_pud)); } p4d_populate(&mut init_mm, p4dp, __va(p) as *mut _); }
    pud_offset(p4dp, addr)
}

unsafe fn kasan_p4d_offset(pgdp: *mut pgd_t, addr: unsigned_long, node: i32, early: bool) -> *mut p4d_t {
    if __pgd_none(early, pgdp_get(pgdp)) { let p = if early { __pa_symbol(kasan_early_shadow_p4d) } else { kasan_alloc_zeroed_page(node) }; if !early { memcpy(__va(p), kasan_early_shadow_p4d, core::mem::size_of_val(&kasan_early_shadow_p4d)); } pgd_populate(&mut init_mm, pgdp, __va(p) as *mut _); }
    p4d_offset(pgdp, addr)
}

unsafe fn kasan_pte_populate(pmdp: *mut pmd_t, mut addr: unsigned_long, end: unsigned_long, node: i32, early: bool) {
    let mut ptep = kasan_pte_offset(pmdp, addr, node, early);
    loop { let page_phys = if early { __pa_symbol(kasan_early_shadow_page) } else { kasan_alloc_zeroed_page(node) }; let next = addr + PAGE_SIZE; set_pte(ptep, pfn_pte(__phys_to_pfn(page_phys), PAGE_KERNEL)); ptep = ptep.add(1); addr = next; if addr == end || !__pte_none(early, ptep_get(ptep)) { break; } }
}

unsafe fn kasan_pmd_populate(pudp: *mut pud_t, mut addr: unsigned_long, end: unsigned_long, node: i32, early: bool) { let mut pmdp = kasan_pmd_offset(pudp, addr, node, early); loop { let next = pmd_addr_end(addr, end); kasan_pte_populate(pmdp, addr, next, node, early); pmdp = pmdp.add(1); addr = next; if addr == end || !__pmd_none(early, pmdp_get(pmdp)) { break; } } }
unsafe fn kasan_pud_populate(p4dp: *mut p4d_t, mut addr: unsigned_long, end: unsigned_long, node: i32, early: bool) { let mut pudp = kasan_pud_offset(p4dp, addr, node, early); loop { let next = pud_addr_end(addr, end); kasan_pmd_populate(pudp, addr, next, node, early); pudp = pudp.add(1); addr = next; if addr == end || !__pud_none(early, READ_ONCE(*pudp)) { break; } } }
unsafe fn kasan_p4d_populate(pgdp: *mut pgd_t, mut addr: unsigned_long, end: unsigned_long, node: i32, early: bool) { let mut p4dp = kasan_p4d_offset(pgdp, addr, node, early); loop { let next = p4d_addr_end(addr, end); kasan_pud_populate(p4dp, addr, next, node, early); p4dp = p4dp.add(1); addr = next; if addr == end || !__p4d_none(early, READ_ONCE(*p4dp)) { break; } } }
unsafe fn kasan_pgd_populate(mut addr: unsigned_long, end: unsigned_long, node: i32, early: bool) { let mut pgdp = pgd_offset_k(addr); loop { let next = pgd_addr_end(addr, end); kasan_p4d_populate(pgdp, addr, next, node, early); pgdp = pgdp.add(1); addr = next; if addr == end { break; } } }

/* Set up full kasan mappings, ensuring that the mapped pages are zeroed */
unsafe fn kasan_map_populate(start: unsigned_long, end: unsigned_long, node: i32) { kasan_pgd_populate(start & PAGE_MASK, PAGE_ALIGN(end), node, false); }

pub unsafe extern "C" fn kasan_early_init() { BUILD_BUG_ON(!IS_ALIGNED(KASAN_SHADOW_START, PGDIR_SIZE)); BUILD_BUG_ON(!IS_ALIGNED(KASAN_SHADOW_END + 1, PGDIR_SIZE)); }
unsafe fn kasan_set_pgd(pgdp: *mut pgd_t, pgdval: pgd_t) { WRITE_ONCE(*pgdp, pgdval); }
unsafe fn clear_pgds(mut start: unsigned_long, end: unsigned_long) { while start < end { let next = pgd_addr_end(start, end); kasan_set_pgd(pgd_offset_k(start), __pgd(0)); start = next; } }

pub unsafe extern "C" fn kasan_init() {
    let mut i: u64 = 0; let mut pa_start: phys_addr_t; let mut pa_end: phys_addr_t;
    if KASAN_SHADOW_END < vm_map_base { pr_warn!("PGDIR_SIZE too large for cpu_vabits, KernelAddressSanitizer disabled.\n"); return; }
    memcpy(kasan_pg_dir.as_mut_ptr(), swapper_pg_dir, core::mem::size_of_val(&kasan_pg_dir));
    csr_write64(__pa_symbol(kasan_pg_dir), LOONGARCH_CSR_PGDH); local_flush_tlb_all();
    clear_pgds(KASAN_SHADOW_START, KASAN_SHADOW_END);
    kasan_pgd_populate(KASAN_SHADOW_START, KASAN_SHADOW_END, NUMA_NO_NODE, true);
    kasan_populate_early_shadow(mem_to_shadow(VMALLOC_START as *const _), mem_to_shadow(KFENCE_AREA_END as *const _));
    for_each_mem_range(i, &mut pa_start, &mut pa_end) { let start = phys_to_virt(pa_start) as *mut _; let end = phys_to_virt(pa_end) as *mut _; kasan_map_populate(mem_to_shadow(start) as unsigned_long, mem_to_shadow(end) as unsigned_long, NUMA_NO_NODE); }
    kasan_map_populate(mem_to_shadow(MODULES_VADDR as *const _) as unsigned_long, mem_to_shadow(MODULES_END as *const _) as unsigned_long, NUMA_NO_NODE);
    for i in 0..PTRS_PER_PTE { set_pte(&mut kasan_early_shadow_pte[i], pfn_pte(__phys_to_pfn(__pa_symbol(kasan_early_shadow_page)), PAGE_KERNEL_RO)); }
    memset(kasan_early_shadow_page, 0, PAGE_SIZE); csr_write64(__pa_symbol(swapper_pg_dir), LOONGARCH_CSR_PGDH); local_flush_tlb_all();
    init_task.kasan_depth = 0; kasan_init_generic();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
