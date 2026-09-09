// SPDX-License-Identifier: GPL-2.0
/*
 * This file contains KASAN shadow initialization code.
 *
 * Copyright (c) 2015 Samsung Electronics Co., Ltd.
 * Author: Andrey Ryabinin <ryabinin.a.a@gmail.com>
 */

// C headers and build-provided symbols are supplied by the surrounding kernel.

/*
 * This page serves two purposes:
 *   - It used as early shadow memory. The entire shadow region populated
 *     with this page, before we will be able to setup normal shadow memory.
 *   - Latter it reused it as zero shadow to cover large ranges of memory
 *     that allowed to access, but not handled by kasan (vmalloc/vmemmap ...).
 */
#[no_mangle]
pub static mut kasan_early_shadow_page: [u8; PAGE_SIZE] = [0; PAGE_SIZE];

#[cfg(CONFIG_PGTABLE_LEVELS = "5")]
#[no_mangle]
pub static mut kasan_early_shadow_p4d: [p4d_t; MAX_PTRS_PER_P4D] = unsafe { core::mem::zeroed() };

#[cfg(CONFIG_PGTABLE_LEVELS = "5")]
#[inline]
unsafe fn kasan_p4d_table(pgd: pgd_t) -> bool {
    pgd_page(pgd) == virt_to_page(lm_alias(kasan_early_shadow_p4d.as_mut_ptr()))
}

#[cfg(not(CONFIG_PGTABLE_LEVELS = "5"))]
#[inline]
unsafe fn kasan_p4d_table(_pgd: pgd_t) -> bool { false }

#[cfg(any(CONFIG_PGTABLE_LEVELS = "4", CONFIG_PGTABLE_LEVELS = "5"))]
#[no_mangle]
pub static mut kasan_early_shadow_pud: [pud_t; MAX_PTRS_PER_PUD] = unsafe { core::mem::zeroed() };

#[cfg(any(CONFIG_PGTABLE_LEVELS = "4", CONFIG_PGTABLE_LEVELS = "5"))]
#[inline]
unsafe fn kasan_pud_table(p4d: p4d_t) -> bool {
    p4d_page(p4d) == virt_to_page(lm_alias(kasan_early_shadow_pud.as_mut_ptr()))
}

#[cfg(not(any(CONFIG_PGTABLE_LEVELS = "4", CONFIG_PGTABLE_LEVELS = "5")))]
#[inline]
unsafe fn kasan_pud_table(_p4d: p4d_t) -> bool { false }

#[cfg(any(CONFIG_PGTABLE_LEVELS = "3", CONFIG_PGTABLE_LEVELS = "4", CONFIG_PGTABLE_LEVELS = "5"))]
#[no_mangle]
pub static mut kasan_early_shadow_pmd: [pmd_t; MAX_PTRS_PER_PMD] = unsafe { core::mem::zeroed() };

#[cfg(any(CONFIG_PGTABLE_LEVELS = "3", CONFIG_PGTABLE_LEVELS = "4", CONFIG_PGTABLE_LEVELS = "5"))]
#[inline]
unsafe fn kasan_pmd_table(pud: pud_t) -> bool {
    pud_page(pud) == virt_to_page(lm_alias(kasan_early_shadow_pmd.as_mut_ptr()))
}

#[cfg(not(any(CONFIG_PGTABLE_LEVELS = "3", CONFIG_PGTABLE_LEVELS = "4", CONFIG_PGTABLE_LEVELS = "5")))]
#[inline]
unsafe fn kasan_pmd_table(_pud: pud_t) -> bool { false }

#[no_mangle]
pub static mut kasan_early_shadow_pte: [pte_t; MAX_PTRS_PER_PTE + PTE_HWTABLE_PTRS] = unsafe { core::mem::zeroed() };

#[inline]
unsafe fn kasan_pte_table(pmd: pmd_t) -> bool {
    pmd_page(pmd) == virt_to_page(lm_alias(kasan_early_shadow_pte.as_mut_ptr()))
}

#[inline]
unsafe fn kasan_early_shadow_page_entry(pte: pte_t) -> bool {
    pte_page(pte) == virt_to_page(lm_alias(kasan_early_shadow_page.as_mut_ptr()))
}

#[inline]
unsafe fn early_alloc(size: usize, node: i32) -> *mut core::ffi::c_void {
    let ptr = memblock_alloc_try_nid(size, size, __pa(MAX_DMA_ADDRESS), MEMBLOCK_ALLOC_ACCESSIBLE, node);
    if ptr.is_null() {
        panic!("{}: Failed to allocate {} bytes align={:x} nid={} from={:x}\n", "early_alloc", size, size, node, __pa(MAX_DMA_ADDRESS) as u64);
    }
    ptr
}

unsafe fn zero_pte_populate(pmd: *mut pmd_t, mut addr: usize, end: usize) {
    let mut pte = pte_offset_kernel(pmd, addr);
    let mut zero_pte = pte_wrprotect(pfn_pte(PFN_DOWN(__pa_symbol(kasan_early_shadow_page.as_mut_ptr())), PAGE_KERNEL));
    while addr.wrapping_add(PAGE_SIZE) <= end {
        set_pte_at(&mut init_mm, addr, pte, zero_pte);
        addr = addr.wrapping_add(PAGE_SIZE);
        pte = pte_offset_kernel(pmd, addr);
    }
}

unsafe fn zero_pmd_populate(pud: *mut pud_t, mut addr: usize, end: usize) -> i32 {
    let mut pmd = pmd_offset(pud, addr);
    loop {
        let next = pmd_addr_end(addr, end);
        if IS_ALIGNED(addr, PMD_SIZE) && end - addr >= PMD_SIZE {
            pmd_populate_kernel(&mut init_mm, pmd, lm_alias(kasan_early_shadow_pte.as_mut_ptr()));
        } else {
            if pmd_none(*pmd) {
                let p = if slab_is_available() { pte_alloc_one_kernel(&mut init_mm) } else {
                    let p = early_alloc(PAGE_SIZE, NUMA_NO_NODE) as *mut pte_t;
                    kernel_pte_init(p); p
                };
                if p.is_null() { return -ENOMEM; }
                pmd_populate_kernel(&mut init_mm, pmd, p);
            }
            zero_pte_populate(pmd, addr, next);
        }
        addr = next; pmd = pmd.add(1);
        if addr == end { break; }
    }
    0
}

unsafe fn zero_pud_populate(p4d: *mut p4d_t, mut addr: usize, end: usize) -> i32 {
    let mut pud = pud_offset(p4d, addr);
    loop {
        let next = pud_addr_end(addr, end);
        if IS_ALIGNED(addr, PUD_SIZE) && end - addr >= PUD_SIZE {
            pud_populate(&mut init_mm, pud, lm_alias(kasan_early_shadow_pmd.as_mut_ptr()));
            let pmd = pmd_offset(pud, addr);
            pmd_populate_kernel(&mut init_mm, pmd, lm_alias(kasan_early_shadow_pte.as_mut_ptr()));
        } else {
            if pud_none(*pud) {
                if slab_is_available() {
                    let p = pmd_alloc(&mut init_mm, pud, addr); if p.is_null() { return -ENOMEM; }
                } else { let p = early_alloc(PAGE_SIZE, NUMA_NO_NODE) as *mut pmd_t; pmd_init(p); pud_populate(&mut init_mm, pud, p); }
            }
            if zero_pmd_populate(pud, addr, next) != 0 { return -ENOMEM; }
        }
        addr = next; pud = pud.add(1); if addr == end { break; }
    }
    0
}

unsafe fn zero_p4d_populate(pgd: *mut pgd_t, mut addr: usize, end: usize) -> i32 {
    let mut p4d = p4d_offset(pgd, addr);
    loop {
        let next = p4d_addr_end(addr, end);
        if IS_ALIGNED(addr, P4D_SIZE) && end - addr >= P4D_SIZE {
            p4d_populate_kernel(addr, p4d, lm_alias(kasan_early_shadow_pud.as_mut_ptr()));
            let pud = pud_offset(p4d, addr); pud_populate(&mut init_mm, pud, lm_alias(kasan_early_shadow_pmd.as_mut_ptr()));
            let pmd = pmd_offset(pud, addr); pmd_populate_kernel(&mut init_mm, pmd, lm_alias(kasan_early_shadow_pte.as_mut_ptr()));
        } else {
            if p4d_none(*p4d) { if slab_is_available() { let p = pud_alloc(&mut init_mm, p4d, addr); if p.is_null() { return -ENOMEM; } } else { let p = early_alloc(PAGE_SIZE, NUMA_NO_NODE) as *mut pud_t; pud_init(p); p4d_populate_kernel(addr, p4d, p); } }
            if zero_pud_populate(p4d, addr, next) != 0 { return -ENOMEM; }
        }
        addr = next; p4d = p4d.add(1); if addr == end { break; }
    }
    0
}

pub unsafe fn kasan_populate_early_shadow(shadow_start: *const core::ffi::c_void, shadow_end: *const core::ffi::c_void) -> i32 {
    let mut addr = shadow_start as usize; let end = shadow_end as usize; let mut pgd = pgd_offset_k(addr);
    loop {
        let next = pgd_addr_end(addr, end);
        if IS_ALIGNED(addr, PGDIR_SIZE) && end - addr >= PGDIR_SIZE {
            pgd_populate_kernel(addr, pgd, lm_alias(kasan_early_shadow_p4d.as_mut_ptr()));
            let p4d = p4d_offset(pgd, addr); p4d_populate_kernel(addr, p4d, lm_alias(kasan_early_shadow_pud.as_mut_ptr()));
            let pud = pud_offset(p4d, addr); pud_populate(&mut init_mm, pud, lm_alias(kasan_early_shadow_pmd.as_mut_ptr()));
            let pmd = pmd_offset(pud, addr); pmd_populate_kernel(&mut init_mm, pmd, lm_alias(kasan_early_shadow_pte.as_mut_ptr()));
        } else {
            if pgd_none(*pgd) { if slab_is_available() { if p4d_alloc(&mut init_mm, pgd, addr).is_null() { return -ENOMEM; } } else { pgd_populate_kernel(addr, pgd, early_alloc(PAGE_SIZE, NUMA_NO_NODE)); } }
            if zero_p4d_populate(pgd, addr, next) != 0 { return -ENOMEM; }
        }
        addr = next; pgd = pgd.add(1); if addr == end { break; }
    }
    0
}

unsafe fn kasan_free_pte(pte_start: *mut pte_t, pmd: *mut pmd_t) { for i in 0..PTRS_PER_PTE { if !pte_none(ptep_get(pte_start.add(i))) { return; } } pte_free_kernel(&mut init_mm, pte_start); pmd_clear(pmd); }
unsafe fn kasan_free_pmd(pmd_start: *mut pmd_t, pud: *mut pud_t) { for i in 0..PTRS_PER_PMD { if !pmd_none(*pmd_start.add(i)) { return; } } pmd_free(&mut init_mm, pmd_start); pud_clear(pud); }
unsafe fn kasan_free_pud(pud_start: *mut pud_t, p4d: *mut p4d_t) { for i in 0..PTRS_PER_PUD { if !pud_none(*pud_start.add(i)) { return; } } pud_free(&mut init_mm, pud_start); p4d_clear(p4d); }
unsafe fn kasan_free_p4d(p4d_start: *mut p4d_t, pgd: *mut pgd_t) { for i in 0..PTRS_PER_P4D { if !p4d_none(*p4d_start.add(i)) { return; } } p4d_free(&mut init_mm, p4d_start); pgd_clear(pgd); }

unsafe fn kasan_remove_pte_table(mut pte: *mut pte_t, mut addr: usize, end: usize) { while addr < end { let mut next = (addr + PAGE_SIZE) & PAGE_MASK; if next > end { next = end; } let ptent = ptep_get(pte); if pte_present(ptent) { if !WARN_ON(!kasan_early_shadow_page_entry(ptent)) { pte_clear(&mut init_mm, addr, pte); } } addr = next; pte = pte.add(1); } }
unsafe fn kasan_remove_pmd_table(mut pmd: *mut pmd_t, mut addr: usize, end: usize) { while addr < end { let next = pmd_addr_end(addr, end); if pmd_present(*pmd) { if kasan_pte_table(*pmd) && IS_ALIGNED(addr, PMD_SIZE) && IS_ALIGNED(next, PMD_SIZE) { pmd_clear(pmd); } else { let pte = pte_offset_kernel(pmd, addr); kasan_remove_pte_table(pte, addr, next); kasan_free_pte(pte_offset_kernel(pmd, 0), pmd); } } addr = next; pmd = pmd.add(1); } }
unsafe fn kasan_remove_pud_table(mut pud: *mut pud_t, mut addr: usize, end: usize) { while addr < end { let next = pud_addr_end(addr, end); if pud_present(*pud) { if kasan_pmd_table(*pud) && IS_ALIGNED(addr, PUD_SIZE) && IS_ALIGNED(next, PUD_SIZE) { pud_clear(pud); } else { let pmd = pmd_offset(pud, addr); kasan_remove_pmd_table(pmd, addr, next); kasan_free_pmd(pmd_offset(pud, 0), pud); } } addr = next; pud = pud.add(1); } }
unsafe fn kasan_remove_p4d_table(mut p4d: *mut p4d_t, mut addr: usize, end: usize) { while addr < end { let next = p4d_addr_end(addr, end); if p4d_present(*p4d) { if kasan_pud_table(*p4d) && IS_ALIGNED(addr, P4D_SIZE) && IS_ALIGNED(next, P4D_SIZE) { p4d_clear(p4d); } else { let pud = pud_offset(p4d, addr); kasan_remove_pud_table(pud, addr, next); kasan_free_pud(pud_offset(p4d, 0), p4d); } } addr = next; p4d = p4d.add(1); } }

pub unsafe fn kasan_remove_zero_shadow(start: *mut core::ffi::c_void, size: usize) { let mut addr = kasan_mem_to_shadow(start) as usize; let end = addr + (size >> KASAN_SHADOW_SCALE_SHIFT); if WARN_ON(start as usize % KASAN_MEMORY_PER_SHADOW_PAGE != 0) || WARN_ON(size % KASAN_MEMORY_PER_SHADOW_PAGE != 0) { return; } while addr < end { let next = pgd_addr_end(addr, end); let pgd = pgd_offset_k(addr); if pgd_present(*pgd) { if kasan_p4d_table(*pgd) && IS_ALIGNED(addr, PGDIR_SIZE) && IS_ALIGNED(next, PGDIR_SIZE) { pgd_clear(pgd); } else { let p4d = p4d_offset(pgd, addr); kasan_remove_p4d_table(p4d, addr, next); kasan_free_p4d(p4d_offset(pgd, 0), pgd); } } addr = next; } }

pub unsafe fn kasan_add_zero_shadow(start: *mut core::ffi::c_void, size: usize) -> i32 { let shadow_start = kasan_mem_to_shadow(start); let shadow_end = (shadow_start as usize + (size >> KASAN_SHADOW_SCALE_SHIFT)) as *const core::ffi::c_void; if WARN_ON(start as usize % KASAN_MEMORY_PER_SHADOW_PAGE != 0) || WARN_ON(size % KASAN_MEMORY_PER_SHADOW_PAGE != 0) { return -EINVAL; } let ret = kasan_populate_early_shadow(shadow_start, shadow_end); if ret != 0 { kasan_remove_zero_shadow(start, size); } ret }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
