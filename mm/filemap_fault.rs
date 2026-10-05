// SPDX-License-Identifier: GPL-2.0-only
// Original mm/filemap.c:3275-4080, CONFIG_MMU.
const MMAP_LOTSAMISS: u16 = 100;
unsafe fn lock_folio_maybe_drop_mmap(
    vmf: *mut vm_fault,
    folio: *mut folio,
    fpin: *mut *mut file,
) -> c_int {
    if folio_trylock(folio) {
        return 1;
    }
    if (*vmf).flags & RUST_FILEMAP_FAULT_FLAG_RETRY_NOWAIT != 0 {
        return 0;
    }
    *fpin = maybe_unlock_mmap_for_io(vmf, *fpin);
    if (*vmf).flags & RUST_FILEMAP_FAULT_FLAG_KILLABLE != 0 {
        if __folio_lock_killable(folio) != 0 {
            if (*fpin).is_null() {
                release_fault_lock(vmf);
            }
            return 0;
        }
    } else {
        __folio_lock(folio);
    }
    1
}
unsafe fn do_sync_mmap_readahead(vmf: *mut vm_fault) -> *mut file {
    let vma = vmf_vma(vmf);
    let file = vma_file(vma);
    let ra = file_ra(file);
    let mapping = file_mapping(file);
    let mut ractl = rust_filemap_readahead_init(file, ra, mapping, vmf_pgoff(vmf));
    let mut fpin = null_mut();
    let vm_flags = vma_vm_flags(vma);
    let (force_thp_readahead, thp_order) = {
        #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
        {
            if vm_flags & RUST_FILEMAP_VM_HUGEPAGE != 0 && mapping_large_folio_support(mapping) {
                (
                    true,
                    core::cmp::min(
                        mapping_max_folio_order(mapping),
                        get_order(RUST_FILEMAP_SZ_2M as c_ulong) as c_uint,
                    ),
                )
            } else {
                (false, 0)
            }
        }
        #[cfg(not(CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            (false, 0)
        }
    };
    if !force_thp_readahead {
        if vm_flags & (RUST_FILEMAP_VM_RAND_READ | RUST_FILEMAP_VM_EXEC)
            == RUST_FILEMAP_VM_RAND_READ
            || (*ra).ra_pages == 0
        {
            return fpin;
        }
        if vm_flags & RUST_FILEMAP_VM_SEQ_READ != 0 {
            fpin = maybe_unlock_mmap_for_io(vmf, fpin);
            page_cache_sync_ra(&mut ractl, (*ra).ra_pages as c_ulong);
            return fpin;
        }
    }
    if vm_flags & (RUST_FILEMAP_VM_SEQ_READ | RUST_FILEMAP_VM_EXEC) == 0 {
        let mut mmap_miss = ra_mmap_miss_read_once(ra);
        if mmap_miss < MMAP_LOTSAMISS * 10 {
            mmap_miss = mmap_miss.wrapping_add(1);
            ra_mmap_miss_write_once(ra, mmap_miss);
        }
        if mmap_miss > MMAP_LOTSAMISS {
            return fpin;
        }
    }
    if force_thp_readahead {
        let nr = (1 as c_ulong).wrapping_shl(thp_order);
        fpin = maybe_unlock_mmap_for_io(vmf, fpin);
        ractl._index &= !nr.wrapping_sub(1);
        (*ra).size = nr as c_uint;
        if vm_flags & RUST_FILEMAP_VM_RAND_READ == 0 {
            (*ra).size = (*ra).size.wrapping_mul(2);
        }
        (*ra).async_size = nr as c_uint;
        (*ra).order = thp_order as u16;
        page_cache_ra_order(&mut ractl, ra);
        return fpin;
    }
    if vm_flags & RUST_FILEMAP_VM_EXEC != 0 {
        let start = vma_start_pgoff(vma);
        let end = vma_end_pgoff(vma);
        (*ra).order = exec_folio_order() as u16;
        let alignment = (1 as c_ulong).wrapping_shl((*ra).order as c_uint);
        (*ra).start = vmf_pgoff(vmf) & !alignment.wrapping_sub(1);
        (*ra).start = core::cmp::max((*ra).start, start);
        let ra_end = ((*ra)
            .start
            .wrapping_add((*ra).ra_pages as c_ulong)
            .wrapping_sub(1)
            | alignment.wrapping_sub(1))
        .wrapping_add(1);
        let ra_end = core::cmp::min(ra_end, end);
        (*ra).size = ra_end.wrapping_sub((*ra).start) as c_uint;
        (*ra).async_size = 0;
    } else {
        (*ra).start = core::cmp::max(
            0,
            vmf_pgoff(vmf).wrapping_sub(((*ra).ra_pages / 2) as c_ulong) as c_long,
        ) as Pgoff;
        (*ra).size = (*ra).ra_pages;
        (*ra).async_size = (*ra).ra_pages / 4;
        (*ra).order = 0;
    }
    fpin = maybe_unlock_mmap_for_io(vmf, fpin);
    ractl._index = (*ra).start;
    page_cache_ra_order(&mut ractl, ra);
    fpin
}
unsafe fn do_async_mmap_readahead(vmf: *mut vm_fault, folio: *mut folio) -> *mut file {
    let vma = vmf_vma(vmf);
    let file = vma_file(vma);
    let ra = file_ra(file);
    let mut ractl = rust_filemap_readahead_init(file, ra, file_mapping(file), vmf_pgoff(vmf));
    let mut fpin = null_mut();
    if vma_vm_flags(vma) & RUST_FILEMAP_VM_RAND_READ != 0 || (*ra).ra_pages == 0 {
        return fpin;
    }
    if !folio_test_locked(folio)
        && vma_vm_flags(vma) & (RUST_FILEMAP_VM_SEQ_READ | RUST_FILEMAP_VM_EXEC) == 0
    {
        let mmap_miss = ra_mmap_miss_read_once(ra);
        if mmap_miss != 0 {
            ra_mmap_miss_write_once(ra, mmap_miss.wrapping_sub(1));
        }
    }
    if folio_test_readahead(folio) {
        fpin = maybe_unlock_mmap_for_io(vmf, fpin);
        page_cache_async_ra(&mut ractl, folio, (*ra).ra_pages as c_ulong);
    }
    fpin
}
unsafe fn filemap_fault_recheck_pte_none(vmf: *mut vm_fault) -> vm_fault_t {
    let vma = vmf_vma(vmf);
    if vma_vm_flags(vma) & RUST_FILEMAP_VM_LOCKED == 0
        || (*vmf).flags & RUST_FILEMAP_FAULT_FLAG_ORIG_PTE_VALID == 0
    {
        return 0;
    }
    let ptep = pte_offset_map_ro_nolock(
        vma_mm(vma),
        (*vmf).pmd,
        vmf_address(vmf),
        addr_of_mut!((*vmf).ptl),
    );
    if ptep.is_null() {
        return RUST_FILEMAP_VM_FAULT_NOPAGE;
    }
    let mut ret = 0;
    if !pte_none(ptep_get_lockless(ptep)) {
        ret = RUST_FILEMAP_VM_FAULT_NOPAGE;
    } else {
        spin_lock((*vmf).ptl);
        if !pte_none(ptep_get(ptep)) {
            ret = RUST_FILEMAP_VM_FAULT_NOPAGE;
        }
        spin_unlock((*vmf).ptl);
    }
    pte_unmap(ptep);
    ret
}
#[no_mangle]
pub unsafe extern "C" fn filemap_fault(vmf: *mut vm_fault) -> vm_fault_t {
    let file = vma_file(vmf_vma(vmf));
    let mut fpin = null_mut();
    let mapping = file_mapping(file);
    let inode = mapping_host(mapping);
    let index = vmf_pgoff(vmf);
    let mut ret = 0;
    let mut mapping_locked = false;
    let max_idx = div_round_up_file_bytes(i_size_read(inode));
    if index >= max_idx {
        return RUST_FILEMAP_VM_FAULT_SIGBUS;
    }
    trace_mm_filemap_fault(mapping, index);
    let mut folio = filemap_get_folio(mapping, index);
    let mut retry_find = IS_ERR(folio.cast());
    if !retry_find {
        if (*vmf).flags & RUST_FILEMAP_FAULT_FLAG_TRIED == 0 {
            fpin = do_async_mmap_readahead(vmf, folio);
        }
        if !folio_test_uptodate(folio) {
            filemap_invalidate_lock_shared(mapping);
            mapping_locked = true;
        }
    } else {
        ret = filemap_fault_recheck_pte_none(vmf);
        if ret != 0 {
            return ret;
        }
        count_vm_event(PGMAJFAULT);
        count_memcg_event_mm(vma_mm(vmf_vma(vmf)), PGMAJFAULT);
        ret = RUST_FILEMAP_VM_FAULT_MAJOR;
        fpin = do_sync_mmap_readahead(vmf);
    }
    loop {
        if retry_find {
            if !mapping_locked {
                filemap_invalidate_lock_shared(mapping);
                mapping_locked = true;
            }
            folio = __filemap_get_folio(
                mapping,
                index,
                RUST_FILEMAP_FGP_CREAT | RUST_FILEMAP_FGP_FOR_MMAP,
                vmf_gfp_mask(vmf),
            );
            if IS_ERR(folio.cast()) {
                if !fpin.is_null() {
                    break;
                }
                filemap_invalidate_unlock_shared(mapping);
                return RUST_FILEMAP_VM_FAULT_OOM;
            }
        }
        retry_find = true;
        if lock_folio_maybe_drop_mmap(vmf, folio, &mut fpin) == 0 {
            break;
        }
        if folio_mapping_field(folio) != mapping {
            folio_unlock(folio);
            folio_put(folio);
            continue;
        }
        vm_bug_folio!((!folio_contains(folio, index)) as c_int, folio);
        if !folio_test_uptodate(folio) {
            if !mapping_locked {
                folio_unlock(folio);
                folio_put(folio);
                continue;
            }
            fpin = maybe_unlock_mmap_for_io(vmf, fpin);
            let error = filemap_read_folio(file, (*mapping_aops(mapping)).read_folio, folio);
            if !fpin.is_null() {
                break;
            }
            folio_put(folio);
            if error == 0 || error == RUST_FILEMAP_AOP_TRUNCATED_PAGE as c_int {
                continue;
            }
            filemap_invalidate_unlock_shared(mapping);
            return RUST_FILEMAP_VM_FAULT_SIGBUS;
        }
        if !fpin.is_null() {
            folio_unlock(folio);
            break;
        }
        if mapping_locked {
            filemap_invalidate_unlock_shared(mapping);
        }
        let max_idx = div_round_up_file_bytes(i_size_read(inode));
        if index >= max_idx {
            folio_unlock(folio);
            folio_put(folio);
            return RUST_FILEMAP_VM_FAULT_SIGBUS;
        }
        (*vmf).page = folio_file_page(folio, index);
        return ret | RUST_FILEMAP_VM_FAULT_LOCKED;
    }
    if !IS_ERR(folio.cast()) {
        folio_put(folio);
    }
    if mapping_locked {
        filemap_invalidate_unlock_shared(mapping);
    }
    if !fpin.is_null() {
        fput(fpin);
    }
    ret | RUST_FILEMAP_VM_FAULT_RETRY
}
unsafe fn filemap_map_pmd(vmf: *mut vm_fault, folio: *mut folio, start: Pgoff) -> bool {
    let mm = vma_mm(vmf_vma(vmf));
    if pmd_trans_huge(*(*vmf).pmd) != 0 {
        folio_unlock(folio);
        folio_put(folio);
        return true;
    }
    if pmd_none(*(*vmf).pmd) != 0 && folio_test_pmd_mappable(folio) {
        let page = folio_file_page(folio, start);
        if do_set_pmd(vmf, folio, page) == 0 {
            folio_unlock(folio);
            return true;
        }
    }
    if pmd_none(*(*vmf).pmd) != 0 && vmf_has_prealloc_pte(vmf) {
        pmd_install_vmf(mm, vmf);
    }
    false
}
unsafe fn next_uptodate_folio(
    xas: *mut xa_state,
    mapping: *mut address_space,
    end_pgoff: Pgoff,
) -> *mut folio {
    let mut folio = xas_next_entry(xas, end_pgoff) as *mut folio;
    while !folio.is_null() {
        if !xas_retry(xas, folio.cast()) && !xa_is_value(folio.cast()) && folio_try_get(folio) {
            if !folio_test_locked(folio)
                && folio.cast() == xas_reload(xas)
                && folio_test_uptodate(folio)
                && !folio_test_readahead(folio)
                && folio_trylock(folio)
            {
                if folio_mapping_field(folio) == mapping && folio_test_uptodate(folio) {
                    let max_idx = div_round_up_file_bytes(i_size_read(mapping_host(mapping)));
                    if (*xas).xa_index < max_idx {
                        return folio;
                    }
                }
                folio_unlock(folio);
            }
            folio_put(folio);
        }
        folio = xas_next_entry(xas, end_pgoff) as *mut folio;
    }
    null_mut()
}
unsafe fn filemap_map_folio_range(
    vmf: *mut vm_fault,
    folio: *mut folio,
    start: c_ulong,
    mut addr: c_ulong,
    mut nr_pages: c_uint,
    rss: *mut c_ulong,
    file_end: Pgoff,
) -> vm_fault_t {
    let mapping = folio_mapping_field(folio);
    let mut ref_from_caller: c_uint = 1;
    let mut ret = 0;
    let mut page = folio_page(folio, start);
    let mut count: c_uint = 0;
    let old_ptep = (*vmf).pte;
    let addr0 = addr.wrapping_sub(start.wrapping_mul(PAGE_SIZE));
    if (file_end >= folio_next_index(folio) || shmem_mapping(mapping))
        && folio_within_vma(folio, vmf_vma(vmf))
        && addr0 & RUST_FILEMAP_PMD_MASK
            == addr0
                .wrapping_add(folio_size(folio) as c_ulong)
                .wrapping_sub(1)
                & RUST_FILEMAP_PMD_MASK
    {
        (*vmf).pte = (*vmf).pte.sub(start as usize);
        page = page_nth(page, (start as c_long).wrapping_neg());
        addr = addr0;
        nr_pages = folio_nr_pages(folio) as c_uint;
    }
    loop {
        if !PageHWPoison(page_nth(page, count as c_long))
            && pte_none(ptep_get((*vmf).pte.add(count as usize)))
        {
            count = count.wrapping_add(1);
        } else {
            if count != 0 {
                set_pte_range(vmf, folio, page, count, addr);
                *rss = (*rss).wrapping_add(count as c_ulong);
                folio_ref_add(folio, count.wrapping_sub(ref_from_caller) as c_int);
                ref_from_caller = 0;
                if vmf_address(vmf).wrapping_sub(addr) < (count as c_ulong).wrapping_mul(PAGE_SIZE)
                {
                    ret = RUST_FILEMAP_VM_FAULT_NOPAGE;
                }
            }
            count = count.wrapping_add(1);
            page = page_nth(page, count as c_long);
            (*vmf).pte = (*vmf).pte.add(count as usize);
            addr = addr.wrapping_add((count as c_ulong).wrapping_mul(PAGE_SIZE));
            count = 0;
        }
        nr_pages = nr_pages.wrapping_sub(1);
        if nr_pages == 0 {
            break;
        }
    }
    if count != 0 {
        set_pte_range(vmf, folio, page, count, addr);
        *rss = (*rss).wrapping_add(count as c_ulong);
        folio_ref_add(folio, count.wrapping_sub(ref_from_caller) as c_int);
        ref_from_caller = 0;
        if vmf_address(vmf).wrapping_sub(addr) < (count as c_ulong).wrapping_mul(PAGE_SIZE) {
            ret = RUST_FILEMAP_VM_FAULT_NOPAGE;
        }
    }
    (*vmf).pte = old_ptep;
    if ref_from_caller != 0 {
        folio_ref_dec(folio);
    }
    ret
}
unsafe fn filemap_map_order0_folio(
    vmf: *mut vm_fault,
    folio: *mut folio,
    addr: c_ulong,
    rss: *mut c_ulong,
) -> vm_fault_t {
    let page = folio_page0(folio);
    if PageHWPoison(page) || !pte_none(ptep_get((*vmf).pte)) {
        folio_ref_dec(folio);
        return 0;
    }
    let ret = if vmf_address(vmf) == addr {
        RUST_FILEMAP_VM_FAULT_NOPAGE
    } else {
        0
    };
    set_pte_range(vmf, folio, page, 1, addr);
    *rss = (*rss).wrapping_add(1);
    ret
}
#[no_mangle]
pub unsafe extern "C" fn filemap_map_pages(
    vmf: *mut vm_fault,
    start_pgoff: Pgoff,
    mut end_pgoff: Pgoff,
) -> vm_fault_t {
    let vma = vmf_vma(vmf);
    let file = vma_file(vma);
    let mapping = file_mapping(file);
    let mut last_pgoff = start_pgoff;
    let mut xas = rust_filemap_xa_state(mapping_i_pages(mapping), start_pgoff);
    let mut ret = 0;
    let mut rss: c_ulong = 0;
    let file_end = div_round_up_file_bytes(i_size_read(mapping_host(mapping))).wrapping_sub(1);
    end_pgoff = core::cmp::min(end_pgoff, file_end);
    rcu_read_lock();
    'out: {
        let mut folio = next_uptodate_folio(&mut xas, mapping, end_pgoff);
        if folio.is_null() {
            break 'out;
        }
        if (file_end >= folio_next_index(folio) || shmem_mapping(mapping))
            && filemap_map_pmd(vmf, folio, start_pgoff)
        {
            ret = RUST_FILEMAP_VM_FAULT_NOPAGE;
            break 'out;
        }
        let mut addr = vma_start(vma).wrapping_add(
            start_pgoff
                .wrapping_sub(vma_start_pgoff(vma))
                .wrapping_shl(PAGE_SHIFT),
        );
        (*vmf).pte = pte_offset_map_lock(vma_mm(vma), (*vmf).pmd, addr, addr_of_mut!((*vmf).ptl));
        if (*vmf).pte.is_null() {
            folio_unlock(folio);
            folio_put(folio);
            break 'out;
        }
        let folio_type = mm_counter_file(folio);
        loop {
            let delta = xas.xa_index.wrapping_sub(last_pgoff);
            addr = addr.wrapping_add(delta.wrapping_shl(PAGE_SHIFT));
            (*vmf).pte = (*vmf).pte.add(delta as usize);
            last_pgoff = xas.xa_index;
            let end = folio_next_index(folio).wrapping_sub(1);
            let nr_pages = core::cmp::min(end, end_pgoff)
                .wrapping_sub(xas.xa_index)
                .wrapping_add(1) as c_uint;
            let map_ret = if !folio_test_large(folio) {
                filemap_map_order0_folio(vmf, folio, addr, &mut rss)
            } else {
                filemap_map_folio_range(
                    vmf,
                    folio,
                    xas.xa_index.wrapping_sub(folio_index(folio)),
                    addr,
                    nr_pages,
                    &mut rss,
                    file_end,
                )
            };
            ret |= map_ret;
            if map_ret & RUST_FILEMAP_VM_FAULT_NOPAGE != 0
                && (*vmf).flags & RUST_FILEMAP_FAULT_FLAG_TRIED == 0
                && !folio_test_workingset(folio)
                && vma_vm_flags(vma) & (RUST_FILEMAP_VM_SEQ_READ | RUST_FILEMAP_VM_EXEC) == 0
            {
                let mmap_miss = ra_mmap_miss_read_once(file_ra(file));
                if mmap_miss != 0 {
                    ra_mmap_miss_write_once(file_ra(file), mmap_miss.wrapping_sub(1));
                }
            }
            folio_unlock(folio);
            folio = next_uptodate_folio(&mut xas, mapping, end_pgoff);
            if folio.is_null() {
                break;
            }
        }
        add_mm_counter(vma_mm(vma), folio_type as c_int, rss as c_long);
        pte_unmap_unlock((*vmf).pte, (*vmf).ptl);
        trace_mm_filemap_map_pages(mapping, start_pgoff, end_pgoff);
    }
    rcu_read_unlock();
    ret
}
#[no_mangle]
pub unsafe extern "C" fn filemap_page_mkwrite(vmf: *mut vm_fault) -> vm_fault_t {
    let file = vma_file(vmf_vma(vmf));
    let mapping = file_mapping(file);
    let folio = page_folio((*vmf).page);
    let mut ret = RUST_FILEMAP_VM_FAULT_LOCKED;
    sb_start_pagefault(inode_superblock(mapping_host(mapping)));
    file_update_time(file);
    folio_lock(folio);
    if folio_mapping_field(folio) != mapping {
        folio_unlock(folio);
        ret = RUST_FILEMAP_VM_FAULT_NOPAGE;
    } else {
        folio_mark_dirty(folio);
        folio_wait_stable(folio);
    }
    sb_end_pagefault(inode_superblock(mapping_host(mapping)));
    ret
}
// Immutable callbacks and zero native fields: safe to share read-only.
unsafe impl Sync for vm_operations_struct {}
#[no_mangle]
pub static generic_file_vm_ops: vm_operations_struct = unsafe {
    let mut ops: vm_operations_struct = zeroed();
    ops.fault = Some(filemap_fault);
    ops.map_pages = Some(filemap_map_pages);
    ops.page_mkwrite = Some(filemap_page_mkwrite);
    ops
};
#[no_mangle]
pub unsafe extern "C" fn generic_file_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    if (*mapping_aops(file_mapping(file))).read_folio.is_none() {
        return -(ENOEXEC as c_int);
    }
    file_accessed(file);
    vma_set_vm_ops(vma, addr_of!(generic_file_vm_ops));
    0
}
#[no_mangle]
pub unsafe extern "C" fn generic_file_mmap_prepare(desc: *mut vm_area_desc) -> c_int {
    let file = (*desc).file;
    if (*mapping_aops(file_mapping(file))).read_folio.is_none() {
        return -(ENOEXEC as c_int);
    }
    file_accessed(file);
    (*desc).vm_ops = addr_of!(generic_file_vm_ops);
    0
}
#[no_mangle]
pub unsafe extern "C" fn generic_file_readonly_mmap(
    file: *mut file,
    vma: *mut vm_area_struct,
) -> c_int {
    if vma_is_shared_maywrite(vma) {
        return -(EINVAL as c_int);
    }
    generic_file_mmap(file, vma)
}
#[no_mangle]
pub unsafe extern "C" fn generic_file_readonly_mmap_prepare(desc: *mut vm_area_desc) -> c_int {
    if is_shared_maywrite(addr_of!((*desc).vma_flags)) {
        return -(EINVAL as c_int);
    }
    generic_file_mmap_prepare(desc)
}
