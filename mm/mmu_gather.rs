// SPDX-License-Identifier: GPL-2.0
// C dependencies supplied by the surrounding kernel translation unit.

#[cfg(not(CONFIG_MMU_GATHER_NO_GATHER))]
static mut MAX_NR_FOLIOS_PER_FREE: usize = 512;

#[cfg(not(CONFIG_MMU_GATHER_NO_GATHER))]
unsafe fn tlb_next_batch(tlb: *mut mmu_gather) -> bool {
    let mut batch: *mut mmu_gather_batch;

    /* Limit batching if we have delayed rmaps pending */
    if (*tlb).delayed_rmap && (*tlb).active != &mut (*tlb).local {
        return false;
    }

    batch = (*tlb).active;
    if !(*batch).next.is_null() {
        (*tlb).active = (*batch).next;
        return true;
    }

    if (*tlb).batch_count == MAX_GATHER_BATCH_COUNT {
        return false;
    }

    batch = __get_free_page(GFP_NOWAIT) as *mut mmu_gather_batch;
    if batch.is_null() {
        return false;
    }

    (*tlb).batch_count += 1;
    (*batch).next = core::ptr::null_mut();
    (*batch).nr = 0;
    (*batch).max = MAX_GATHER_BATCH;

    (*tlb).active.as_mut().unwrap().next = batch;
    (*tlb).active = batch;
    true
}

#[cfg(all(not(CONFIG_MMU_GATHER_NO_GATHER), CONFIG_SMP))]
unsafe fn tlb_flush_rmap_batch(batch: *mut mmu_gather_batch, vma: *mut vm_area_struct) {
    let pages = (*batch).encoded_pages;
    let mut i = 0;
    while i < (*batch).nr {
        let enc = *pages.add(i);
        if encoded_page_flags(enc) & ENCODED_PAGE_BIT_DELAY_RMAP != 0 {
            let page = encoded_page_ptr(enc);
            let mut nr_pages = 1;
            if encoded_page_flags(enc) & ENCODED_PAGE_BIT_NR_PAGES_NEXT != 0 {
                i += 1;
                nr_pages = encoded_nr_pages(*pages.add(i));
            }
            folio_remove_rmap_ptes(page_folio(page), page, nr_pages, vma);
        }
        i += 1;
    }
}

#[cfg(all(not(CONFIG_MMU_GATHER_NO_GATHER), CONFIG_SMP))]
pub unsafe fn tlb_flush_rmaps(tlb: *mut mmu_gather, vma: *mut vm_area_struct) {
    if (*tlb).delayed_rmap == 0 { return; }
    tlb_flush_rmap_batch(&mut (*tlb).local, vma);
    if (*tlb).active != &mut (*tlb).local {
        tlb_flush_rmap_batch((*tlb).active, vma);
    }
    (*tlb).delayed_rmap = 0;
}

#[cfg(not(CONFIG_MMU_GATHER_NO_GATHER))]
unsafe fn __tlb_batch_free_encoded_pages(batch: *mut mmu_gather_batch) {
    let mut pages = (*batch).encoded_pages;
    let mut nr: usize;
    let mut nr_pages: usize;
    while (*batch).nr != 0 {
        if !page_poisoning_enabled_static() && !want_init_on_free() {
            nr = core::cmp::min(MAX_NR_FOLIOS_PER_FREE, (*batch).nr);
            if encoded_page_flags(*pages.add(nr - 1)) & ENCODED_PAGE_BIT_NR_PAGES_NEXT != 0 { nr += 1; }
        } else {
            nr = 0; nr_pages = 0;
            while nr < (*batch).nr && nr_pages < MAX_NR_FOLIOS_PER_FREE {
                if encoded_page_flags(*pages.add(nr)) & ENCODED_PAGE_BIT_NR_PAGES_NEXT != 0 {
                    nr += 1; nr_pages += encoded_nr_pages(*pages.add(nr));
                } else { nr_pages += 1; }
                nr += 1;
            }
        }
        free_pages_and_swap_cache(pages, nr);
        pages = pages.add(nr);
        (*batch).nr -= nr;
        cond_resched();
    }
}

#[cfg(not(CONFIG_MMU_GATHER_NO_GATHER))]
unsafe fn tlb_batch_pages_flush(tlb: *mut mmu_gather) {
    let mut batch = &mut (*tlb).local as *mut mmu_gather_batch;
    while !batch.is_null() && (*batch).nr != 0 {
        __tlb_batch_free_encoded_pages(batch);
        batch = (*batch).next;
    }
    (*tlb).active = &mut (*tlb).local;
}

#[cfg(not(CONFIG_MMU_GATHER_NO_GATHER))]
unsafe fn tlb_batch_list_free(tlb: *mut mmu_gather) {
    let mut batch = (*tlb).local.next;
    while !batch.is_null() {
        let next = (*batch).next;
        free_pages(batch as usize, 0);
        batch = next;
    }
    (*tlb).local.next = core::ptr::null_mut();
}

#[cfg(not(CONFIG_MMU_GATHER_NO_GATHER))]
unsafe fn __tlb_remove_folio_pages_size(tlb: *mut mmu_gather, page: *mut page, nr_pages: u32, delay_rmap: bool, page_size: i32) -> bool {
    let mut flags = if delay_rmap { ENCODED_PAGE_BIT_DELAY_RMAP } else { 0 };
    let mut batch = (*tlb).active;
    VM_BUG_ON(!(*tlb).end);
    #[cfg(CONFIG_MMU_GATHER_PAGE_SIZE)] {
        VM_WARN_ON((*tlb).page_size != page_size);
        VM_WARN_ON_ONCE(nr_pages != 1 && page_size != PAGE_SIZE);
        VM_WARN_ON_ONCE(page_folio(page) != page_folio(page.add((nr_pages - 1) as usize)));
    }
    if nr_pages == 1 {
        (*batch).encoded_pages[(*batch).nr] = encode_page(page, flags);
        (*batch).nr += 1;
    } else {
        flags |= ENCODED_PAGE_BIT_NR_PAGES_NEXT;
        (*batch).encoded_pages[(*batch).nr] = encode_page(page, flags);
        (*batch).nr += 1;
        (*batch).encoded_pages[(*batch).nr] = encode_nr_pages(nr_pages);
        (*batch).nr += 1;
    }
    if (*batch).nr >= (*batch).max - 1 {
        if !tlb_next_batch(tlb) { return true; }
        batch = (*tlb).active;
    }
    VM_BUG_ON_PAGE((*batch).nr > (*batch).max - 1, page);
    false
}

#[cfg(not(CONFIG_MMU_GATHER_NO_GATHER))]
pub unsafe fn __tlb_remove_folio_pages(tlb: *mut mmu_gather, page: *mut page, nr_pages: u32, delay_rmap: bool) -> bool { __tlb_remove_folio_pages_size(tlb, page, nr_pages, delay_rmap, PAGE_SIZE) }

#[cfg(not(CONFIG_MMU_GATHER_NO_GATHER))]
pub unsafe fn __tlb_remove_page_size(tlb: *mut mmu_gather, page: *mut page, page_size: i32) -> bool { __tlb_remove_folio_pages_size(tlb, page, 1, false, page_size) }

#[cfg(CONFIG_MMU_GATHER_TABLE_FREE)]
unsafe fn __tlb_remove_table_free(batch: *mut mmu_table_batch) {
    for i in 0..(*batch).nr { __tlb_remove_table(*(*batch).tables.add(i)); }
    free_page(batch as usize);
}

#[cfg(all(CONFIG_MMU_GATHER_TABLE_FREE, CONFIG_MMU_GATHER_RCU_TABLE_FREE))]
unsafe fn tlb_remove_table_smp_sync(_arg: *mut core::ffi::c_void) {}

#[cfg(all(CONFIG_MMU_GATHER_TABLE_FREE, CONFIG_MMU_GATHER_RCU_TABLE_FREE))]
pub unsafe fn tlb_remove_table_sync_one() { smp_call_function(tlb_remove_table_smp_sync, core::ptr::null_mut(), 1); }

#[cfg(all(CONFIG_MMU_GATHER_TABLE_FREE, CONFIG_MMU_GATHER_RCU_TABLE_FREE))]
unsafe fn tlb_remove_table_rcu(head: *mut rcu_head) { __tlb_remove_table_free(container_of!(head, mmu_table_batch, rcu)); }

#[cfg(all(CONFIG_MMU_GATHER_TABLE_FREE, CONFIG_MMU_GATHER_RCU_TABLE_FREE))]
unsafe fn tlb_remove_table_free(batch: *mut mmu_table_batch) { call_rcu(&mut (*batch).rcu, tlb_remove_table_rcu); }

#[cfg(all(CONFIG_MMU_GATHER_TABLE_FREE, CONFIG_MMU_GATHER_RCU_TABLE_FREE))]
pub unsafe fn tlb_remove_table_sync_rcu() { synchronize_rcu(); }

#[cfg(all(CONFIG_MMU_GATHER_TABLE_FREE, not(CONFIG_MMU_GATHER_RCU_TABLE_FREE)))]
unsafe fn tlb_remove_table_free(batch: *mut mmu_table_batch) { __tlb_remove_table_free(batch); }

#[cfg(CONFIG_MMU_GATHER_TABLE_FREE)]
#[inline]
unsafe fn tlb_table_invalidate(tlb: *mut mmu_gather) { if tlb_needs_table_invalidate() { tlb_flush_mmu_tlbonly(tlb); } }

#[cfg(all(CONFIG_MMU_GATHER_TABLE_FREE, CONFIG_PT_RECLAIM))]
unsafe fn __tlb_remove_table_one_rcu(head: *mut rcu_head) { let ptdesc = container_of!(head, ptdesc, pt_rcu_head); __tlb_remove_table(ptdesc); }

#[cfg(all(CONFIG_MMU_GATHER_TABLE_FREE, CONFIG_PT_RECLAIM))]
unsafe fn __tlb_remove_table_one(table: *mut core::ffi::c_void) { let ptdesc = table as *mut ptdesc; call_rcu(&mut (*ptdesc).pt_rcu_head, __tlb_remove_table_one_rcu); }

#[cfg(all(CONFIG_MMU_GATHER_TABLE_FREE, not(CONFIG_PT_RECLAIM)))]
unsafe fn __tlb_remove_table_one(table: *mut core::ffi::c_void) { tlb_remove_table_sync_rcu(); __tlb_remove_table(table); }

#[cfg(CONFIG_MMU_GATHER_TABLE_FREE)]
unsafe fn tlb_remove_table_one(table: *mut core::ffi::c_void) { __tlb_remove_table_one(table); }

#[cfg(CONFIG_MMU_GATHER_TABLE_FREE)]
unsafe fn tlb_table_flush(tlb: *mut mmu_gather) {
    let batch = &mut (*tlb).batch;
    if !(*batch).is_null() { tlb_table_invalidate(tlb); tlb_remove_table_free(*batch); *batch = core::ptr::null_mut(); }
}

#[cfg(CONFIG_MMU_GATHER_TABLE_FREE)]
pub unsafe fn tlb_remove_table(tlb: *mut mmu_gather, table: *mut core::ffi::c_void) {
    let batch = &mut (*tlb).batch;
    if (*batch).is_null() {
        *batch = __get_free_page(GFP_NOWAIT) as *mut mmu_table_batch;
        if (*batch).is_null() { tlb_table_invalidate(tlb); tlb_remove_table_one(table); return; }
        (**batch).nr = 0;
    }
    let n = (**batch).nr;
    (**batch).tables.add(n).write(table);
    (**batch).nr = n + 1;
    if (**batch).nr == MAX_TABLE_BATCH { tlb_table_flush(tlb); }
}

#[cfg(CONFIG_MMU_GATHER_TABLE_FREE)]
#[inline] unsafe fn tlb_table_init(tlb: *mut mmu_gather) { (*tlb).batch = core::ptr::null_mut(); }
#[cfg(not(CONFIG_MMU_GATHER_TABLE_FREE))]
#[inline] unsafe fn tlb_table_flush(_tlb: *mut mmu_gather) {}
#[cfg(not(CONFIG_MMU_GATHER_TABLE_FREE))]
#[inline] unsafe fn tlb_table_init(_tlb: *mut mmu_gather) {}

unsafe fn tlb_flush_mmu_free(tlb: *mut mmu_gather) { tlb_table_flush(tlb); #[cfg(not(CONFIG_MMU_GATHER_NO_GATHER))] tlb_batch_pages_flush(tlb); }
pub unsafe fn tlb_flush_mmu(tlb: *mut mmu_gather) { tlb_flush_mmu_tlbonly(tlb); tlb_flush_mmu_free(tlb); }

unsafe fn __tlb_gather_mmu(tlb: *mut mmu_gather, mm: *mut mm_struct, fullmm: bool) {
    (*tlb).mm = mm; (*tlb).fullmm = fullmm;
    #[cfg(not(CONFIG_MMU_GATHER_NO_GATHER))] {
        (*tlb).need_flush_all = 0; (*tlb).local.next = core::ptr::null_mut(); (*tlb).local.nr = 0; (*tlb).local.max = (*tlb).__pages.len(); (*tlb).active = &mut (*tlb).local; (*tlb).batch_count = 0;
    }
    (*tlb).delayed_rmap = 0; tlb_table_init(tlb);
    #[cfg(CONFIG_MMU_GATHER_PAGE_SIZE)] { (*tlb).page_size = 0; }
    (*tlb).vma_pfn = 0; (*tlb).fully_unshared_tables = 0; __tlb_reset_range(tlb); inc_tlb_flush_pending((*tlb).mm);
}

pub unsafe fn tlb_gather_mmu(tlb: *mut mmu_gather, mm: *mut mm_struct) { __tlb_gather_mmu(tlb, mm, false); }
pub unsafe fn tlb_gather_mmu_fullmm(tlb: *mut mmu_gather, mm: *mut mm_struct) { __tlb_gather_mmu(tlb, mm, true); }
pub unsafe fn tlb_gather_mmu_vma(tlb: *mut mmu_gather, vma: *mut vm_area_struct) { tlb_gather_mmu(tlb, (*vma).vm_mm); tlb_update_vma_flags(tlb, vma); if is_vm_hugetlb_page(vma) { tlb_change_page_size(tlb, huge_page_size(hstate_vma(vma))); } }

pub unsafe fn tlb_finish_mmu(tlb: *mut mmu_gather) {
    VM_WARN_ON_ONCE((*tlb).fully_unshared_tables);
    if mm_tlb_flush_nested((*tlb).mm) { (*tlb).fullmm = true; __tlb_reset_range(tlb); (*tlb).freed_tables = 1; }
    tlb_flush_mmu(tlb);
    #[cfg(not(CONFIG_MMU_GATHER_NO_GATHER))] tlb_batch_list_free(tlb);
    dec_tlb_flush_pending((*tlb).mm);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
