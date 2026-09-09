// SPDX-License-Identifier: GPL-2.0-only
/*
 * mm/truncate.c - code for taking down pages from address_spaces
 *
 * Copyright (C) 2002, Linus Torvalds
 */

// Kernel headers and `internal.h` are supplied by the surrounding translation.

unsafe fn clear_shadow_entries(mapping: *mut address_space, start: c_ulong, max: c_ulong) {
    let mut xas = XA_STATE!(&mut (*mapping).i_pages, start);
    let mut folio: *mut folio;
    if shmem_mapping(mapping) || dax_mapping(mapping) { return; }
    xas_set_update(&mut xas, workingset_update_node);
    spin_lock(&mut (*(*mapping).host).i_lock);
    xas_lock_irq(&mut xas);
    xas_for_each!(&mut xas, folio, max, { if xa_is_value(folio) { xas_store(&mut xas, core::ptr::null_mut()); } });
    xas_unlock_irq(&mut xas);
    if mapping_shrinkable(mapping) { inode_lru_list_add((*mapping).host); }
    spin_unlock(&mut (*(*mapping).host).i_lock);
}

unsafe fn truncate_folio_batch_exceptionals(mapping: *mut address_space, fbatch: *mut folio_batch, indices: *mut pgoff_t) {
    let mut xas = XA_STATE!(&mut (*mapping).i_pages, *indices);
    let nr = folio_batch_count(fbatch);
    let mut folio: *mut folio;
    let mut j = 0;
    if shmem_mapping(mapping) { return; }
    while j < nr && !xa_is_value((*fbatch).folios[j as usize]) { j += 1; }
    if j == nr { return; }
    if dax_mapping(mapping) {
        let mut i = j;
        while i < nr { if xa_is_value((*fbatch).folios[i as usize]) { WARN_ON_ONCE!(1); dax_delete_mapping_entry(mapping, *indices.add(i as usize)); } i += 1; }
    } else {
        xas_set(&mut xas, *indices.add(j as usize));
        xas_set_update(&mut xas, workingset_update_node);
        spin_lock(&mut (*(*mapping).host).i_lock);
        xas_lock_irq(&mut xas);
        xas_for_each!(&mut xas, folio, *indices.add((nr - 1) as usize), { if xa_is_value(folio) { xas_store(&mut xas, core::ptr::null_mut()); } });
        xas_unlock_irq(&mut xas);
        if mapping_shrinkable(mapping) { inode_lru_list_add((*mapping).host); }
        spin_unlock(&mut (*(*mapping).host).i_lock);
    }
    folio_batch_remove_exceptionals(fbatch);
}

pub unsafe fn folio_invalidate(folio: *mut folio, offset: usize, length: usize) {
    let aops = (*(*folio).mapping).a_ops;
    if !(*aops).invalidate_folio.is_none() { ((*aops).invalidate_folio.unwrap())(folio, offset, length); }
}

unsafe fn truncate_cleanup_folio(folio: *mut folio) {
    if folio_mapped(folio) { unmap_mapping_folio(folio); }
    if folio_needs_release(folio) { folio_invalidate(folio, 0, folio_size(folio)); }
    folio_cancel_dirty(folio);
}

pub unsafe fn truncate_inode_folio(mapping: *mut address_space, folio: *mut folio) -> c_int {
    if (*folio).mapping != mapping { return -EIO; }
    truncate_cleanup_folio(folio); filemap_remove_folio(folio); 0
}

unsafe fn folio_split_or_unmap(folio: *mut folio, split_at: *mut page, min_order: c_ulong) -> c_int {
    let flags = TTU_SYNC | TTU_SPLIT_HUGE_PMD | TTU_IGNORE_MLOCK;
    let ret = folio_split(folio, min_order, split_at, core::ptr::null_mut());
    if ret != 0 && !shmem_mapping((*folio).mapping) { try_to_unmap(folio, flags); WARN_ON!(folio_mapped(folio)); }
    ret
}

pub unsafe fn truncate_inode_partial_folio(folio: *mut folio, start: loff_t, end: loff_t) -> bool {
    let pos = folio_pos(folio); let size = folio_size(folio);
    let offset = if pos < start { (start - pos) as usize } else { 0 };
    let length = if pos + size <= end as u64 { size - offset } else { (end + 1 - pos) as usize - offset };
    folio_wait_writeback(folio);
    if length == size { truncate_inode_folio((*folio).mapping, folio); return true; }
    if !mapping_inaccessible((*folio).mapping) { folio_zero_range(folio, offset, length); }
    if folio_needs_release(folio) { folio_invalidate(folio, offset, length); }
    if !folio_test_large(folio) { return true; }
    let min_order = mapping_min_folio_order((*folio).mapping);
    let split_at = folio_page(folio, PAGE_ALIGN_DOWN!(offset) / PAGE_SIZE);
    if folio_split_or_unmap(folio, split_at, min_order) == 0 {
        if offset + length == size { return true; }
        let split_at2 = folio_page(folio, PAGE_ALIGN_DOWN!(offset + length) / PAGE_SIZE);
        let folio2 = page_folio(split_at2);
        if !folio_try_get(folio2) { return true; }
        if !folio_test_large(folio2) { folio_put(folio2); return true; }
        if folio_trylock(folio2) {
            if folio_test_large(folio2) && (*folio2).mapping == (*folio).mapping { folio_split_or_unmap(folio2, split_at2, min_order); }
            folio_unlock(folio2);
        }
        folio_put(folio2); return true;
    }
    if folio_test_dirty(folio) { return false; }
    truncate_inode_folio((*folio).mapping, folio); true
}

pub unsafe fn generic_error_remove_folio(mapping: *mut address_space, folio: *mut folio) -> c_int {
    if mapping.is_null() { return -EINVAL; }
    if !S_ISREG!((*(*mapping).host).i_mode) { return -EIO; }
    truncate_inode_folio(mapping, folio)
}

pub unsafe fn mapping_evict_folio(mapping: *mut address_space, folio: *mut folio) -> c_long {
    if mapping.is_null() || folio_test_dirty(folio) || folio_test_writeback(folio) { return 0; }
    if folio_ref_count(folio) > folio_nr_pages(folio) + folio_has_private(folio) as usize + 1 { return 0; }
    if !filemap_release_folio(folio, 0) { return 0; }
    remove_mapping(mapping, folio)
}

pub unsafe fn truncate_inode_pages_range(mapping: *mut address_space, lstart: loff_t, lend: uoff_t) {
    if mapping_empty(mapping) { return; }
    let mut start = (lstart + PAGE_SIZE - 1) >> PAGE_SHIFT;
    let mut end = if lend == -1 { -1 } else { (lend + 1) >> PAGE_SHIFT };
    let mut fbatch = core::mem::zeroed::<folio_batch>(); let mut indices = [0 as pgoff_t; FOLIO_BATCH_SIZE];
    folio_batch_init(&mut fbatch); let mut index = start;
    while index < end && find_lock_entries(mapping, &mut index, end - 1, &mut fbatch, indices.as_mut_ptr()) {
        truncate_folio_batch_exceptionals(mapping, &mut fbatch, indices.as_mut_ptr());
        for i in 0..folio_batch_count(&fbatch) { truncate_cleanup_folio(fbatch.folios[i as usize]); }
        delete_from_page_cache_batch(mapping, &mut fbatch);
        for i in 0..folio_batch_count(&fbatch) { folio_unlock(fbatch.folios[i as usize]); }
        folio_batch_release(&mut fbatch); cond_resched();
    }
    let mut same_folio = (lstart >> PAGE_SHIFT) == (lend >> PAGE_SHIFT);
    let mut folio = __filemap_get_folio(mapping, lstart >> PAGE_SHIFT, FGP_LOCK, 0);
    if !IS_ERR!(folio) {
        same_folio = lend < folio_next_pos(folio);
        if !truncate_inode_partial_folio(folio, lstart, lend as loff_t) { start = folio_next_index(folio); if same_folio { end = (*folio).index; } }
        folio_unlock(folio); folio_put(folio); folio = core::ptr::null_mut();
    }
    if !same_folio { folio = __filemap_get_folio(mapping, lend >> PAGE_SHIFT, FGP_LOCK, 0); if !IS_ERR!(folio) { if !truncate_inode_partial_folio(folio, lstart, lend as loff_t) { end = (*folio).index; } folio_unlock(folio); folio_put(folio); } }
    index = start;
    while index < end {
        cond_resched();
        if !find_get_entries(mapping, &mut index, end - 1, &mut fbatch, indices.as_mut_ptr()) { if index == start { break; } index = start; continue; }
        for i in 0..folio_batch_count(&fbatch) { folio = fbatch.folios[i as usize]; if xa_is_value(folio) { continue; } folio_lock(folio); VM_BUG_ON_FOLIO!(!folio_contains(folio, indices[i as usize]), folio); folio_wait_writeback(folio); truncate_inode_folio(mapping, folio); folio_unlock(folio); }
        truncate_folio_batch_exceptionals(mapping, &mut fbatch, indices.as_mut_ptr()); folio_batch_release(&mut fbatch);
    }
}

pub unsafe fn truncate_inode_pages(mapping: *mut address_space, lstart: loff_t) { truncate_inode_pages_range(mapping, lstart, -1); }
pub unsafe fn truncate_inode_pages_final(mapping: *mut address_space) { mapping_set_exiting(mapping); if !mapping_empty(mapping) { xa_lock_irq(&mut (*mapping).i_pages); xa_unlock_irq(&mut (*mapping).i_pages); } truncate_inode_pages(mapping, 0); }

pub unsafe fn mapping_try_invalidate(mapping: *mut address_space, start: pgoff_t, end: pgoff_t, nr_failed: *mut c_ulong) -> c_ulong {
    let mut indices = [0 as pgoff_t; FOLIO_BATCH_SIZE]; let mut fbatch = core::mem::zeroed::<folio_batch>(); let mut index = start; let mut count = 0; folio_batch_init(&mut fbatch);
    while find_lock_entries(mapping, &mut index, end, &mut fbatch, indices.as_mut_ptr()) { let nr = folio_batch_count(&fbatch); let mut values = false; for i in 0..nr { let f = fbatch.folios[i as usize]; if xa_is_value(f) { values = true; count += 1; continue; } let ret = mapping_evict_folio(mapping, f); folio_unlock(f); if ret == 0 { deactivate_file_folio(f); if !nr_failed.is_null() { *nr_failed += 1; } } count += ret as c_ulong; } if values { clear_shadow_entries(mapping, indices[0], indices[nr as usize - 1]); } folio_batch_remove_exceptionals(&mut fbatch); folio_batch_release(&mut fbatch); cond_resched(); } count
}
pub unsafe fn invalidate_mapping_pages(mapping: *mut address_space, start: pgoff_t, end: pgoff_t) -> c_ulong { mapping_try_invalidate(mapping, start, end, core::ptr::null_mut()) }

unsafe fn folio_launder(mapping: *mut address_space, folio: *mut folio) -> c_int { if !folio_test_dirty(folio) || (*folio).mapping != mapping { return 0; } match (*(*mapping).a_ops).launder_folio { Some(f) => f(folio), None => 0 } }
pub unsafe fn folio_unmap_invalidate(mapping: *mut address_space, folio: *mut folio, gfp: gfp_t) -> c_int { VM_BUG_ON_FOLIO!(!folio_test_locked(folio), folio); if folio_mapped(folio) { unmap_mapping_folio(folio); } BUG_ON!(folio_mapped(folio)); let ret = folio_launder(mapping, folio); if ret != 0 { return ret; } if (*folio).mapping != mapping || !filemap_release_folio(folio, gfp) { return -EBUSY; } spin_lock(&mut (*(*mapping).host).i_lock); xa_lock_irq(&mut (*mapping).i_pages); if folio_test_dirty(folio) { xa_unlock_irq(&mut (*mapping).i_pages); spin_unlock(&mut (*(*mapping).host).i_lock); return -EBUSY; } BUG_ON!(folio_has_private(folio)); __filemap_remove_folio(folio, core::ptr::null_mut()); xa_unlock_irq(&mut (*mapping).i_pages); if mapping_shrinkable(mapping) { inode_lru_list_add((*mapping).host); } let free = (*(*mapping).a_ops).free_folio; spin_unlock(&mut (*(*mapping).host).i_lock); if let Some(f) = free { f(folio); } folio_put_refs(folio, folio_nr_pages(folio)); 1 }

pub unsafe fn invalidate_inode_pages2_range(mapping: *mut address_space, start: pgoff_t, end: pgoff_t) -> c_int { let mut indices = [0 as pgoff_t; FOLIO_BATCH_SIZE]; let mut fb = core::mem::zeroed::<folio_batch>(); let mut index = start; let mut ret = 0; let mut did = 0; if mapping_empty(mapping) { return 0; } folio_batch_init(&mut fb); while find_get_entries(mapping, &mut index, end, &mut fb, indices.as_mut_ptr()) { let nr = folio_batch_count(&fb); let mut vals = false; for i in 0..nr { let f = fb.folios[i as usize]; if xa_is_value(f) { vals = true; if dax_mapping(mapping) && !dax_invalidate_mapping_entry_sync(mapping, indices[i as usize]) { ret = -EBUSY; } continue; } if did == 0 && folio_mapped(f) { unmap_mapping_pages(mapping, indices[i as usize], 1 + end - indices[i as usize], false); did = 1; } folio_lock(f); if (*f).mapping != mapping { folio_unlock(f); continue; } VM_BUG_ON_FOLIO!(!folio_contains(f, indices[i as usize]), f); folio_wait_writeback(f); let r = folio_unmap_invalidate(mapping, f, GFP_KERNEL); if r < 0 { ret = r; } folio_unlock(f); } if vals { clear_shadow_entries(mapping, indices[0], indices[nr as usize - 1]); } folio_batch_remove_exceptionals(&mut fb); folio_batch_release(&mut fb); cond_resched(); } if dax_mapping(mapping) { unmap_mapping_pages(mapping, start, end - start + 1, false); } ret }
pub unsafe fn invalidate_inode_pages2(mapping: *mut address_space) -> c_int { invalidate_inode_pages2_range(mapping, 0, -1) }

pub unsafe fn truncate_pagecache(inode: *mut inode, newsize: loff_t) { let mapping = (*inode).i_mapping; let holebegin = round_up(newsize, PAGE_SIZE); unmap_mapping_range(mapping, holebegin, 0, 1); truncate_inode_pages(mapping, newsize); unmap_mapping_range(mapping, holebegin, 0, 1); }
pub unsafe fn truncate_setsize(inode: *mut inode, newsize: loff_t) { let oldsize = (*inode).i_size; i_size_write(inode, newsize); if newsize > oldsize { pagecache_isize_extended(inode, oldsize, newsize); } truncate_pagecache(inode, newsize); }
pub unsafe fn pagecache_isize_extended(inode: *mut inode, from: loff_t, to: loff_t) { let bsize = i_blocksize(inode); WARN_ON!(to > (*inode).i_size); if from >= to || bsize >= PAGE_SIZE { return; } let rounded = round_up(from, bsize); if to <= rounded || rounded & (PAGE_SIZE - 1) == 0 { return; } let folio = filemap_lock_folio((*inode).i_mapping, from / PAGE_SIZE); if IS_ERR!(folio) { return; } if folio_mkclean(folio) { folio_mark_dirty(folio); } if folio_test_dirty(folio) { let offset = (from - folio_pos(folio)) as usize; let end = core::cmp::min((to - folio_pos(folio)) as usize, folio_size(folio)); folio_zero_segment(folio, offset, end); } folio_unlock(folio); folio_put(folio); }
pub unsafe fn truncate_pagecache_range(inode: *mut inode, lstart: loff_t, lend: loff_t) { let mapping = (*inode).i_mapping; let start = round_up(lstart, PAGE_SIZE); let end = round_down(1 + lend, PAGE_SIZE) - 1; if end as u64 > start as u64 { unmap_mapping_range(mapping, start, 1 + end - start, 0); } truncate_inode_pages_range(mapping, lstart, lend as uoff_t); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
