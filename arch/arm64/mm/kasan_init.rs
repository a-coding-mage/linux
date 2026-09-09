// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file contains kasan initialization code for ARM64.
 *
 * Copyright (c) 2015 Samsung Electronics Co., Ltd.
 * Author: Andrey Ryabinin <ryabinin.a.a@gmail.com>
 */

// C includes and configuration-dependent declarations are supplied by the kernel bindings.
// This implementation is enabled when CONFIG_KASAN_GENERIC or CONFIG_KASAN_SW_TAGS is set.

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
static mut TMP_PG_DIR: [pgd_t; PTRS_PER_PTE] = [unsafe { core::mem::zeroed() }; PTRS_PER_PTE];

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
unsafe fn kasan_alloc_zeroed_page(node: i32) -> phys_addr_t {
    let p = memblock_alloc_try_nid(
        PAGE_SIZE,
        PAGE_SIZE,
        __pa(MAX_DMA_ADDRESS),
        MEMBLOCK_ALLOC_NOLEAKTRACE,
        node,
    );
    if p.is_null() {
        panic!("{}: Failed to allocate {} bytes align=0x{:x} nid={} from {:x}\n", "kasan_alloc_zeroed_page", PAGE_SIZE, PAGE_SIZE, node, __pa(MAX_DMA_ADDRESS));
    }
    __pa(p)
}

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
unsafe fn kasan_alloc_raw_page(node: i32) -> phys_addr_t {
    let p = memblock_alloc_try_nid_raw(
        PAGE_SIZE,
        PAGE_SIZE,
        __pa(MAX_DMA_ADDRESS),
        MEMBLOCK_ALLOC_NOLEAKTRACE,
        node,
    );
    if p.is_null() {
        panic!("{}: Failed to allocate {} bytes align=0x{:x} nid={} from {:x}\n", "kasan_alloc_raw_page", PAGE_SIZE, PAGE_SIZE, node, __pa(MAX_DMA_ADDRESS));
    }
    __pa(p)
}

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
unsafe fn kasan_pte_offset(pmdp: *mut pmd_t, addr: usize, node: i32, early: bool) -> *mut pte_t {
    if pmd_none(core::ptr::read_volatile(pmdp)) {
        let pte_phys = if early { __pa_symbol(kasan_early_shadow_pte) } else { kasan_alloc_zeroed_page(node) };
        __pmd_populate(pmdp, pte_phys, PMD_TYPE_TABLE);
    }
    if early { pte_offset_kimg(pmdp, addr) } else { pte_offset_kernel(pmdp, addr) }
}

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
unsafe fn kasan_pmd_offset(pudp: *mut pud_t, addr: usize, node: i32, early: bool) -> *mut pmd_t {
    if pud_none(core::ptr::read_volatile(pudp)) {
        let pmd_phys = if early { __pa_symbol(kasan_early_shadow_pmd) } else { kasan_alloc_zeroed_page(node) };
        __pud_populate(pudp, pmd_phys, PUD_TYPE_TABLE);
    }
    if early { pmd_offset_kimg(pudp, addr) } else { pmd_offset(pudp, addr) }
}

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
unsafe fn kasan_pud_offset(p4dp: *mut p4d_t, addr: usize, node: i32, early: bool) -> *mut pud_t {
    if p4d_none(core::ptr::read_volatile(p4dp)) {
        let pud_phys = if early { __pa_symbol(kasan_early_shadow_pud) } else { kasan_alloc_zeroed_page(node) };
        __p4d_populate(p4dp, pud_phys, P4D_TYPE_TABLE);
    }
    if early { pud_offset_kimg(p4dp, addr) } else { pud_offset(p4dp, addr) }
}

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
unsafe fn kasan_p4d_offset(pgdp: *mut pgd_t, addr: usize, node: i32, early: bool) -> *mut p4d_t {
    if pgd_none(core::ptr::read_volatile(pgdp)) {
        let p4d_phys = if early { __pa_symbol(kasan_early_shadow_p4d) } else { kasan_alloc_zeroed_page(node) };
        __pgd_populate(pgdp, p4d_phys, PGD_TYPE_TABLE);
    }
    if early { p4d_offset_kimg(pgdp, addr) } else { p4d_offset(pgdp, addr) }
}

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
unsafe fn kasan_pte_populate(pmdp: *mut pmd_t, mut addr: usize, end: usize, node: i32, early: bool) {
    let mut ptep = kasan_pte_offset(pmdp, addr, node, early);
    loop {
        let page_phys = if early { __pa_symbol(kasan_early_shadow_page) } else { kasan_alloc_raw_page(node) };
        if !early { core::ptr::write_bytes(__va(page_phys), KASAN_SHADOW_INIT, PAGE_SIZE); }
        let next = addr + PAGE_SIZE;
        __set_pte(ptep, pfn_pte(__phys_to_pfn(page_phys), PAGE_KERNEL));
        ptep = ptep.add(1);
        addr = next;
        if addr == end || !pte_none(__ptep_get(ptep)) { break; }
    }
}

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
unsafe fn kasan_pmd_populate(pudp: *mut pud_t, mut addr: usize, end: usize, node: i32, early: bool) {
    let mut pmdp = kasan_pmd_offset(pudp, addr, node, early);
    loop {
        let next = pmd_addr_end(addr, end);
        kasan_pte_populate(pmdp, addr, next, node, early);
        pmdp = pmdp.add(1); addr = next;
        if addr == end || !pmd_none(core::ptr::read_volatile(pmdp)) { break; }
    }
}

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
unsafe fn kasan_pud_populate(p4dp: *mut p4d_t, mut addr: usize, end: usize, node: i32, early: bool) {
    let mut pudp = kasan_pud_offset(p4dp, addr, node, early);
    loop {
        let next = pud_addr_end(addr, end);
        kasan_pmd_populate(pudp, addr, next, node, early);
        pudp = pudp.add(1); addr = next;
        if addr == end || !pud_none(core::ptr::read_volatile(pudp)) { break; }
    }
}

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
unsafe fn kasan_p4d_populate(pgdp: *mut pgd_t, mut addr: usize, end: usize, node: i32, early: bool) {
    let mut p4dp = kasan_p4d_offset(pgdp, addr, node, early);
    loop {
        let next = p4d_addr_end(addr, end);
        kasan_pud_populate(p4dp, addr, next, node, early);
        p4dp = p4dp.add(1); addr = next;
        if addr == end || !p4d_none(core::ptr::read_volatile(p4dp)) { break; }
    }
}

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
unsafe fn kasan_pgd_populate(mut addr: usize, end: usize, node: i32, early: bool) {
    let mut pgdp = pgd_offset_k(addr);
    loop {
        let next = pgd_addr_end(addr, end);
        kasan_p4d_populate(pgdp, addr, next, node, early);
        pgdp = pgdp.add(1); addr = next;
        if addr == end { break; }
    }
}

#[cfg(any(CONFIG_ARM64_64K_PAGES, CONFIG_PGTABLE_LEVELS_GT_4))]
const SHADOW_ALIGN: usize = P4D_SIZE;
#[cfg(not(any(CONFIG_ARM64_64K_PAGES, CONFIG_PGTABLE_LEVELS_GT_4)))]
const SHADOW_ALIGN: usize = PUD_SIZE;

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
unsafe fn root_level_aligned(addr: u64) -> bool {
    let shift = (ARM64_HW_PGTABLE_LEVELS(vabits_actual) - 1) * PTDESC_TABLE_SHIFT;
    addr % ((PAGE_SIZE as u64) << shift) == 0
}

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
pub unsafe extern "C" fn kasan_early_init() {
    BUILD_BUG_ON!(KASAN_SHADOW_OFFSET != KASAN_SHADOW_END - (1usize << (64 - KASAN_SHADOW_SCALE_SHIFT)));
    BUILD_BUG_ON!(!IS_ALIGNED(_KASAN_SHADOW_START(VA_BITS), SHADOW_ALIGN));
    BUILD_BUG_ON!(!IS_ALIGNED(_KASAN_SHADOW_START(VA_BITS_MIN), SHADOW_ALIGN));
    BUILD_BUG_ON!(!IS_ALIGNED(KASAN_SHADOW_END, SHADOW_ALIGN));
    if !root_level_aligned(KASAN_SHADOW_START) {
        static mut TBL: [pte_t; PTRS_PER_PTE] = [core::mem::zeroed(); PTRS_PER_PTE];
        let pgdp = pgd_offset_k(KASAN_SHADOW_START);
        set_pgd(pgdp, __pgd(__pa_symbol(TBL) | PGD_TYPE_TABLE));
    }
    kasan_pgd_populate(KASAN_SHADOW_START, KASAN_SHADOW_END, NUMA_NO_NODE, true);
}

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
unsafe fn kasan_map_populate(start: usize, end: usize, node: i32) {
    kasan_pgd_populate(start & PAGE_MASK, PAGE_ALIGN(end), node, false);
}

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
unsafe fn root_level_idx(addr: u64) -> i32 {
    let vabits = if cfg!(CONFIG_ARM64_64K_PAGES) { VA_BITS } else { vabits_actual };
    let shift = (ARM64_HW_PGTABLE_LEVELS(vabits) - 1) * PTDESC_TABLE_SHIFT;
    ((addr & !_PAGE_OFFSET(vabits)) >> (shift + PAGE_SHIFT)) as i32
}

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
unsafe fn clone_next_level(addr: u64, tmp_pg_dir: *mut pgd_t, pud: *mut pud_t) {
    let idx = root_level_idx(addr);
    let pgd = core::ptr::read_volatile(swapper_pg_dir.add(idx as usize));
    let pudp = __phys_to_kimg(__pgd_to_phys(pgd)) as *const pud_t;
    core::ptr::copy_nonoverlapping(pudp, pud, PTRS_PER_PUD);
    *tmp_pg_dir.add(idx as usize) = __pgd(__phys_to_pgd_val(__pa_symbol(pud)) | PUD_TYPE_TABLE);
}

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
unsafe fn next_level_idx(addr: u64) -> i32 {
    let shift = (ARM64_HW_PGTABLE_LEVELS(vabits_actual) - 2) * PTDESC_TABLE_SHIFT;
    ((addr >> (shift + PAGE_SHIFT)) % PTRS_PER_PTE) as i32
}

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
unsafe fn clear_next_level(pgd_idx: i32, start: i32, end: i32) {
    let pgd = core::ptr::read_volatile(swapper_pg_dir.add(pgd_idx as usize));
    let pudp = __phys_to_kimg(__pgd_to_phys(pgd)) as *mut pud_t;
    core::ptr::write_bytes(pudp.add(start as usize), 0, (end - start) as usize);
}

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
unsafe fn clear_shadow(start: u64, end: u64) {
    let mut l = root_level_idx(start);
    let m = root_level_idx(end);
    if !root_level_aligned(start) { clear_next_level(l, next_level_idx(start), PTRS_PER_PTE as i32); l += 1; }
    if !root_level_aligned(end) { clear_next_level(m, 0, next_level_idx(end)); }
    core::ptr::write_bytes(swapper_pg_dir.add(l as usize), 0, (m - l) as usize);
}

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
unsafe fn kasan_init_shadow() {
    static mut PUD: [[pud_t; PTRS_PER_PUD]; 2] = [[core::mem::zeroed(); PTRS_PER_PUD]; 2];
    let kimg_shadow_start = (kasan_mem_to_shadow(KERNEL_START) as u64) & PAGE_MASK as u64;
    let kimg_shadow_end = PAGE_ALIGN(kasan_mem_to_shadow(KERNEL_END) as usize) as u64;
    let mod_shadow_start = kasan_mem_to_shadow(MODULES_VADDR) as u64;
    let vmalloc_shadow_end = kasan_mem_to_shadow(VMALLOC_END) as u64;
    core::ptr::copy_nonoverlapping(swapper_pg_dir, &mut TMP_PG_DIR as *mut _, PTRS_PER_PTE);
    if !root_level_aligned(KASAN_SHADOW_START) { clone_next_level(KASAN_SHADOW_START, TMP_PG_DIR.as_mut_ptr(), PUD[0].as_mut_ptr()); }
    if !root_level_aligned(KASAN_SHADOW_END) { clone_next_level(KASAN_SHADOW_END, TMP_PG_DIR.as_mut_ptr(), PUD[1].as_mut_ptr()); }
    dsb(ishst); cpu_replace_ttbr1(lm_alias(TMP_PG_DIR.as_mut_ptr()));
    clear_shadow(KASAN_SHADOW_START, KASAN_SHADOW_END);
    kasan_map_populate(kimg_shadow_start as usize, kimg_shadow_end as usize, early_pfn_to_nid(virt_to_pfn(lm_alias(KERNEL_START))));
    kasan_populate_early_shadow(kasan_mem_to_shadow(PAGE_END), mod_shadow_start as *mut _);
    BUILD_BUG_ON!(VMALLOC_START != MODULES_END);
    kasan_populate_early_shadow(vmalloc_shadow_end as *mut _, KASAN_SHADOW_END as *mut _);
    for_each_mem_range!(i, &mut pa_start, &mut pa_end, {
        let start = __phys_to_virt(pa_start) as *mut _;
        let end = __phys_to_virt(pa_end) as *mut _;
        kasan_map_populate(kasan_mem_to_shadow(start) as usize, kasan_mem_to_shadow(end) as usize, early_pfn_to_nid(virt_to_pfn(start)));
    });
    for i in 0..PTRS_PER_PTE { __set_pte(&mut kasan_early_shadow_pte[i], pfn_pte(sym_to_pfn(kasan_early_shadow_page), PAGE_KERNEL_RO)); }
    core::ptr::write_bytes(kasan_early_shadow_page, KASAN_SHADOW_INIT, PAGE_SIZE);
    cpu_replace_ttbr1(lm_alias(swapper_pg_dir));
}

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
unsafe fn kasan_init_depth() { init_task.kasan_depth = 0; }

#[cfg(all(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS), CONFIG_KASAN_VMALLOC))]
pub unsafe extern "C" fn kasan_populate_early_vm_area_shadow(start: *mut core::ffi::c_void, size: usize) {
    if !is_vmalloc_or_module_addr(start) { return; }
    let shadow_start = ALIGN_DOWN(kasan_mem_to_shadow(start) as usize, PAGE_SIZE);
    let shadow_end = ALIGN(kasan_mem_to_shadow(start.add(size)) as usize, PAGE_SIZE);
    kasan_map_populate(shadow_start, shadow_end, NUMA_NO_NODE);
}

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
pub unsafe extern "C" fn kasan_init() {
    kasan_init_shadow();
    kasan_init_depth();
    kasan_init_generic();
    /*
     * Generic KASAN is now fully initialized.
     * Software and Hardware Tag-Based modes still require
     * kasan_init_sw_tags() and kasan_init_hw_tags() correspondingly.
     */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
