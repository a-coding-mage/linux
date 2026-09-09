// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mm/fault-armv.c
 *
 *  Copyright (C) 1995  Linus Torvalds
 *  Modifications for ARM processor (c) 1995-2002 Russell King
 */

// Dependencies are supplied by the surrounding Linux/Rust translation.

static mut shared_pte_mask: pteval_t = L_PTE_MT_BUFFERABLE;

#[cfg(__LINUX_ARM_ARCH_LT_6)]
static unsafe fn do_adjust_pte(
    vma: *mut vm_area_struct,
    address: c_ulong,
    pfn: c_ulong,
    ptep: *mut pte_t,
) -> c_int {
    let mut entry = *ptep;
    let ret = pte_present(entry);

    if ret != 0 && (pte_val(entry) & L_PTE_MT_MASK) != shared_pte_mask {
        flush_cache_page(vma, address, pfn);
        outer_flush_range(pfn << PAGE_SHIFT, (pfn << PAGE_SHIFT) + PAGE_SIZE);
        pte_val(entry) &= !L_PTE_MT_MASK;
        pte_val(entry) |= shared_pte_mask;
        set_pte_at((*vma).vm_mm, address, ptep, entry);
        flush_tlb_page(vma, address);
    }
    ret
}

#[cfg(__LINUX_ARM_ARCH_LT_6)]
static unsafe fn adjust_pte(
    vma: *mut vm_area_struct,
    address: c_ulong,
    pfn: c_ulong,
    need_lock: bool,
) -> c_int {
    let mut ptl: *mut spinlock_t = core::ptr::null_mut();
    let pgd = pgd_offset((*vma).vm_mm, address);
    if pgd_none_or_clear_bad(pgd) { return 0; }
    let p4d = p4d_offset(pgd, address);
    if p4d_none_or_clear_bad(p4d) { return 0; }
    let pud = pud_offset(p4d, address);
    if pud_none_or_clear_bad(pud) { return 0; }
    let pmd = pmd_offset(pud, address);
    if pmd_none_or_clear_bad(pmd) { return 0; }

    loop {
        let mut pmdval = core::mem::zeroed::<pmd_t>();
        let pte = pte_offset_map_rw_nolock((*vma).vm_mm, pmd, address, &mut pmdval, &mut ptl);
        if pte.is_null() { return 0; }
        if need_lock {
            spin_lock_nested(ptl, SINGLE_DEPTH_NESTING);
            if !pmd_same(pmdval, pmdp_get_lockless(pmd)) {
                pte_unmap_unlock(pte, ptl);
                continue;
            }
        }
        let ret = do_adjust_pte(vma, address, pfn, pte);
        if need_lock { spin_unlock(ptl); }
        pte_unmap(pte);
        return ret;
    }
}

#[cfg(__LINUX_ARM_ARCH_LT_6)]
static unsafe fn make_coherent(
    mapping: *mut address_space,
    vma: *mut vm_area_struct,
    addr: c_ulong,
    ptep: *mut pte_t,
    pfn: c_ulong,
) {
    let pmd_start_addr = ALIGN_DOWN(addr, PMD_SIZE);
    let pmd_end_addr = pmd_start_addr + PMD_SIZE;
    let mm = (*vma).vm_mm;
    let mut aliases = 0;
    let pgoff = linear_page_index(vma, addr);

    flush_dcache_mmap_lock(mapping);
    mapping_rmap_tree_foreach!(mpnt, mapping, pgoff, pgoff, {
        let mut need_lock = IS_ENABLED(CONFIG_SPLIT_PTE_PTLOCKS);
        if (*mpnt).vm_mm != mm || mpnt == vma { continue; }
        if (*mpnt).vm_flags & VM_MAYSHARE == 0 { continue; }
        let offset = (pgoff - (*mpnt).vm_pgoff) << PAGE_SHIFT;
        let mpnt_addr = (*mpnt).vm_start + offset;
        if mpnt_addr >= pmd_start_addr && mpnt_addr < pmd_end_addr { need_lock = false; }
        aliases += adjust_pte(mpnt, mpnt_addr, pfn, need_lock);
    });
    flush_dcache_mmap_unlock(mapping);
    if aliases != 0 { do_adjust_pte(vma, addr, pfn, ptep); }
}

#[cfg(__LINUX_ARM_ARCH_LT_6)]
unsafe fn update_mmu_cache_range(
    _vmf: *mut vm_fault,
    vma: *mut vm_area_struct,
    addr: c_ulong,
    ptep: *mut pte_t,
    _nr: c_uint,
) {
    let pfn = pte_pfn(*ptep);
    if !pfn_valid(pfn) || is_zero_pfn(pfn) { return; }
    let folio = page_folio(pfn_to_page(pfn));
    let mapping = folio_flush_mapping(folio);
    if !test_and_set_bit(PG_dcache_clean, &mut (*folio).flags.f) {
        __flush_dcache_folio(mapping, folio);
    }
    if !mapping.is_null() {
        if cache_is_vivt() { make_coherent(mapping, vma, addr, ptep, pfn); }
        else if (*vma).vm_flags & VM_EXEC != 0 { __flush_icache_all(); }
    }
}

static unsafe fn check_writebuffer(p1: *mut c_ulong, p2: *mut c_ulong) -> c_int {
    let zero: c_ulong = 0;
    let one: c_ulong = 1;
    local_irq_disable();
    mb(); *p1 = one;
    mb(); *p2 = zero;
    mb(); let val = *p1;
    mb(); local_irq_enable();
    (val != zero) as c_int
}

unsafe fn check_writebuffer_bugs() {
    let mut page: *mut page = core::ptr::null_mut();
    let mut reason: *const c_char;
    let mut v: c_ulong = 1;
    pr_info!("CPU: Testing write buffer coherency: ");
    page = alloc_page(GFP_KERNEL);
    if !page.is_null() {
        let prot = __pgprot_modify(PAGE_KERNEL, L_PTE_MT_MASK, L_PTE_MT_BUFFERABLE);
        let p1 = vmap(&page, 1, VM_IOREMAP, prot);
        let p2 = vmap(&page, 1, VM_IOREMAP, prot);
        if !p1.is_null() && !p2.is_null() {
            v = check_writebuffer(p1, p2);
            reason = c"enabling work-around".as_ptr();
        } else { reason = c"unable to map memory\n".as_ptr(); }
        vunmap(p1); vunmap(p2); put_page(page);
    } else { reason = c"unable to grab page\n".as_ptr(); }
    if v != 0 {
        pr_cont!("failed, %s\n", reason);
        shared_pte_mask = L_PTE_MT_UNCACHED;
    } else { pr_cont!("ok\n"); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
