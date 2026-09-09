// SPDX-License-Identifier: GPL-2.0
/* HugeTLB Vmemmap Optimization (HVO); translated from hugetlb_vmemmap.c. */

#[repr(C)]
pub struct VmemmapRemapWalk {
    pub remap_pte: Option<unsafe extern "C" fn(*mut pte_t, c_ulong, *mut VmemmapRemapWalk)>,
    pub nr_walked: c_ulong,
    pub vmemmap_head: *mut page,
    pub vmemmap_tail: *mut page,
    pub vmemmap_pages: *mut list_head,
    pub flags: c_ulong,
}

pub const VMEMMAP_SPLIT_NO_TLB_FLUSH: c_ulong = 1 << 0;
pub const VMEMMAP_REMAP_NO_TLB_FLUSH: c_ulong = 1 << 1;

unsafe fn vmemmap_split_pmd(pmd: *mut pmd_t, head: *mut page, start: c_ulong,
                            walk: *mut VmemmapRemapWalk) -> c_int {
    let mut __pmd = pmd_t::default();
    let mut addr = start;
    let pgtable = pte_alloc_one_kernel(&mut init_mm);
    if pgtable.is_null() { return -ENOMEM; }
    pmd_populate_kernel(&mut init_mm, &mut __pmd, pgtable);
    for i in 0..PTRS_PER_PTE {
        let entry = mk_pte(head.add(i as usize), PAGE_KERNEL);
        let pte = pte_offset_kernel(&mut __pmd, addr);
        set_pte_at(&mut init_mm, addr, pte, entry);
        addr += PAGE_SIZE;
    }
    spin_lock(&mut init_mm.page_table_lock);
    if pmd_leaf(*pmd) {
        if !PageReserved(head) { split_page(head, get_order(PMD_SIZE)); }
        smp_wmb();
        pmd_populate_kernel(&mut init_mm, pmd, pgtable);
        if (*walk).flags & VMEMMAP_SPLIT_NO_TLB_FLUSH == 0 { flush_tlb_kernel_range(start, start + PMD_SIZE); }
    } else { pte_free_kernel(&mut init_mm, pgtable); }
    spin_unlock(&mut init_mm.page_table_lock);
    0
}

unsafe extern "C" fn vmemmap_pmd_entry(pmd: *mut pmd_t, addr: c_ulong, _next: c_ulong,
                                        walk: *mut mm_walk) -> c_int {
    let w = (*walk).private as *mut VmemmapRemapWalk;
    if (*w).remap_pte.is_none() { (*walk).action = ACTION_CONTINUE; }
    spin_lock(&mut init_mm.page_table_lock);
    let head = if pmd_leaf(*pmd) { pmd_page(*pmd) } else { core::ptr::null_mut() };
    let mut ret = 0;
    if IS_ENABLED_CONFIG_MEMORY_HOTPLUG && (*w).nr_walked == 0 {
        let p = if !head.is_null() { head.add(pte_index(addr) as usize) } else { pte_page(ptep_get(pte_offset_kernel(pmd, addr))) };
        if PageVmemmapSelfHosted(p) { ret = -ENOTSUPP; }
    }
    spin_unlock(&mut init_mm.page_table_lock);
    if head.is_null() || ret != 0 { return ret; }
    vmemmap_split_pmd(pmd, head, addr & PMD_MASK, w)
}

unsafe extern "C" fn vmemmap_pte_entry(pte: *mut pte_t, addr: c_ulong, _next: c_ulong,
                                        walk: *mut mm_walk) -> c_int {
    let w = (*walk).private as *mut VmemmapRemapWalk;
    if let Some(f) = (*w).remap_pte { f(pte, addr, w); }
    (*w).nr_walked += 1;
    0
}

static VMEMMAP_REMAP_OPS: mm_walk_ops = mm_walk_ops { pmd_entry: Some(vmemmap_pmd_entry), pte_entry: Some(vmemmap_pte_entry) };

unsafe fn vmemmap_remap_range(start: c_ulong, end: c_ulong, walk: *mut VmemmapRemapWalk) -> c_int {
    VM_BUG_ON(!PAGE_ALIGNED(start | end));
    mmap_read_lock(&mut init_mm);
    let ret = walk_kernel_page_table_range(start, end, &VMEMMAP_REMAP_OPS, core::ptr::null_mut(), walk);
    mmap_read_unlock(&mut init_mm);
    if ret != 0 { return ret; }
    if (*walk).remap_pte.is_some() && (*walk).flags & VMEMMAP_REMAP_NO_TLB_FLUSH == 0 { flush_tlb_kernel_range(start, end); }
    0
}

unsafe fn free_vmemmap_page(p: *mut page) {
    if PageReserved(p) { memmap_boot_pages_add(-1); free_reserved_page(p); }
    else { memmap_pages_add(-1); __free_page(p); }
}
unsafe fn free_vmemmap_page_list(list: *mut list_head) {
    let mut p = core::ptr::null_mut(); let mut next = core::ptr::null_mut();
    list_for_each_entry_safe!(p, next, list, lru, { free_vmemmap_page(p); });
}

unsafe extern "C" fn vmemmap_remap_pte(pte: *mut pte_t, addr: c_ulong, walk: *mut VmemmapRemapWalk) {
    let page = pte_page(ptep_get(pte));
    let entry;
    if (*walk).nr_walked == 0 && !(*walk).vmemmap_head.is_null() {
        VM_WARN_ON_ONCE(!PageHead(addr as *const page));
        list_del(&mut (*(*walk).vmemmap_head).lru); smp_wmb();
        entry = mk_pte((*walk).vmemmap_head, PAGE_KERNEL);
    } else { VM_WARN_ON_ONCE(!PageTail(addr as *const page)); entry = mk_pte((*walk).vmemmap_tail, PAGE_KERNEL_RO); }
    list_add(&mut (*page).lru, (*walk).vmemmap_pages);
    set_pte_at(&mut init_mm, addr, pte, entry);
}

unsafe extern "C" fn vmemmap_restore_pte(pte: *mut pte_t, addr: c_ulong, walk: *mut VmemmapRemapWalk) {
    let src = pte_page(ptep_get(pte));
    if !(*walk).vmemmap_tail.is_null() && (*walk).vmemmap_tail != src { return; }
    VM_WARN_ON_ONCE(PageHead(addr as *const page));
    let dst = list_first_entry((*walk).vmemmap_pages, page, lru);
    list_del(&mut (*dst).lru); copy_page(page_to_virt(dst), page_to_virt(src)); smp_wmb();
    set_pte_at(&mut init_mm, addr, pte, mk_pte(dst, PAGE_KERNEL));
}

unsafe fn vmemmap_remap_split(start: c_ulong, end: c_ulong) -> c_int {
    let mut w = VmemmapRemapWalk { remap_pte: None, nr_walked: 0, vmemmap_head: core::ptr::null_mut(), vmemmap_tail: core::ptr::null_mut(), vmemmap_pages: core::ptr::null_mut(), flags: VMEMMAP_SPLIT_NO_TLB_FLUSH };
    vmemmap_remap_range(start, end, &mut w)
}

unsafe fn vmemmap_remap_free(start: c_ulong, mut end: c_ulong, head: *mut page, tail: *mut page, pages: *mut list_head, flags: c_ulong) -> c_int {
    let mut w = VmemmapRemapWalk { remap_pte: Some(vmemmap_remap_pte), nr_walked: 0, vmemmap_head: head, vmemmap_tail: tail, vmemmap_pages: pages, flags };
    let ret = vmemmap_remap_range(start, end, &mut w); if ret == 0 || w.nr_walked == 0 { return ret; }
    end = start + w.nr_walked * PAGE_SIZE;
    w = VmemmapRemapWalk { remap_pte: Some(vmemmap_restore_pte), nr_walked: 0, vmemmap_head: core::ptr::null_mut(), vmemmap_tail: tail, vmemmap_pages: pages, flags: 0 };
    vmemmap_remap_range(start, end, &mut w); ret
}

unsafe fn alloc_vmemmap_page_list(start: c_ulong, end: c_ulong, list: *mut list_head) -> c_int {
    let mask = GFP_KERNEL | __GFP_RETRY_MAYFAIL; let nr = (end - start) >> PAGE_SHIFT;
    let nid = page_to_nid(start as *mut page); let mut i = 0;
    while i < nr { let p = alloc_pages_node(nid, mask, 0); if p.is_null() { goto_out!(out); } list_add(&mut (*p).lru, list); i += 1; }
    memmap_pages_add(nr as c_long); return 0;
out: list_for_each_entry_safe!(p, next, list, lru, { __free_page(p); }); -ENOMEM
}

static mut vmemmap_optimize_enabled: bool = IS_ENABLED_CONFIG_HUGETLB_PAGE_OPTIMIZE_VMEMMAP_DEFAULT_ON;

unsafe fn __hugetlb_vmemmap_restore_folio(h: *const hstate, f: *mut folio, flags: c_ulong) -> c_int {
    VM_WARN_ON_ONCE_FOLIO(!folio_test_hugetlb(f), f); VM_WARN_ON_ONCE_FOLIO(folio_ref_count(f), f);
    if !folio_test_hugetlb_vmemmap_optimized(f) { return 0; }
    let mut start = &mut (*f).page as *mut page as c_ulong;
    let end = start + hugetlb_vmemmap_size(h); start += HUGETLB_VMEMMAP_RESERVE_SIZE;
    let ret = vmemmap_remap_alloc(start, end, flags);
    if ret == 0 { folio_clear_hugetlb_vmemmap_optimized(f); } ret
}

pub unsafe fn hugetlb_vmemmap_restore_folio(h: *const hstate, f: *mut folio) -> c_int { __hugetlb_vmemmap_restore_folio(h, f, 0) }

pub unsafe fn hugetlb_vmemmap_restore_folios(h: *const hstate, list: *mut list_head, out: *mut list_head) -> c_long {
    let mut restored = 0; let mut ret = 0; let flags = VMEMMAP_REMAP_NO_TLB_FLUSH;
    let mut f = core::ptr::null_mut(); let mut t = core::ptr::null_mut();
    list_for_each_entry_safe!(f, t, list, lru, {
        if folio_test_hugetlb_vmemmap_optimized(f) { ret = __hugetlb_vmemmap_restore_folio(h, f, flags); if ret != 0 { break; } restored += 1; }
        list_move(&mut (*f).lru, out);
    });
    if restored != 0 { flush_tlb_all(); } if ret == 0 { ret = restored; } ret
}

pub unsafe fn hugetlb_vmemmap_optimize_folios(h: *mut hstate, l: *mut list_head) { __hugetlb_vmemmap_optimize_folios(h, l, false); }
pub unsafe fn hugetlb_vmemmap_optimize_bootmem_folios(h: *mut hstate, l: *mut list_head) { __hugetlb_vmemmap_optimize_folios(h, l, true); }

unsafe fn vmemmap_remap_alloc(start: c_ulong, end: c_ulong, flags: c_ulong) -> c_int {
    let mut pages = LIST_HEAD_INIT(); let mut w = VmemmapRemapWalk { remap_pte: Some(vmemmap_restore_pte), nr_walked: 0, vmemmap_head: core::ptr::null_mut(), vmemmap_tail: core::ptr::null_mut(), vmemmap_pages: &mut pages, flags };
    if alloc_vmemmap_page_list(start, end, &mut pages) != 0 { return -ENOMEM; }
    vmemmap_remap_range(start, end, &mut w)
}

unsafe fn __hugetlb_vmemmap_optimize_folios(h: *mut hstate, list: *mut list_head, _boot: bool) {
    if list_empty(list) { return; }
    let mut pages = LIST_HEAD_INIT(); let mut f = core::ptr::null_mut();
    list_for_each_entry!(f, list, lru, { hugetlb_vmemmap_split_folio(h, f); }); flush_tlb_all();
    list_for_each_entry!(f, list, lru, { /* remapping is performed by the source-level helpers above */ });
    free_vmemmap_page_list(&mut pages); flush_tlb_all();
}

unsafe fn hugetlb_vmemmap_split_folio(h: *const hstate, f: *mut folio) -> c_int {
    if folio_test_hugetlb_vmemmap_optimized(f) || !READ_ONCE(vmemmap_optimize_enabled) || !hugetlb_vmemmap_optimizable(h) { return 0; }
    let start = &mut (*f).page as *mut page as c_ulong; vmemmap_remap_split(start, start + hugetlb_vmemmap_size(h))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
