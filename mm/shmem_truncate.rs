// SPDX-License-Identifier: GPL-2.0-only
// C:1120-1293. Preserve the two-pass truncate algorithm and races with swapout.
unsafe fn shmem_undo_range(inode: *mut inode, lstart: loff_t, lend: uoff_t, unfalloc: bool) {
    let mapping = (*inode).i_mapping;
    let info = SHMEM_I(inode);
    let mut start = (lstart as u64)
        .wrapping_add(RUST_SHMEM_PAGE_SIZE as u64 - 1)
        .wrapping_shr(RUST_SHMEM_PAGE_SHIFT as u32) as Pgoff;
    let mut end = lend
        .wrapping_add(1)
        .wrapping_shr(RUST_SHMEM_PAGE_SHIFT as u32) as Pgoff;
    if lend == uoff_t::MAX {
        end = Pgoff::MAX;
    }
    if (*info).fallocend > start && (*info).fallocend <= end && !unfalloc {
        (*info).fallocend = start;
    }
    let mut batch = MaybeUninit::<folio_batch>::uninit();
    let fb = batch.as_mut_ptr();
    let mut indices = [0 as Pgoff; RUST_SHMEM_FOLIO_BATCH_SIZE as usize];
    let mut swaps: c_long = 0;
    let mut index = start;
    folio_batch_init(fb);
    while index < end
        && find_lock_entries(
            mapping,
            &mut index,
            end.wrapping_sub(1),
            fb,
            indices.as_mut_ptr(),
        ) != 0
    {
        for i in 0..folio_batch_count(fb) as usize {
            let f = (*fb).folios[i];
            if xa_is_value(f.cast()) {
                if !unfalloc {
                    swaps = swaps.wrapping_add(shmem_free_swap(
                        mapping,
                        indices[i],
                        end.wrapping_sub(1),
                        f.cast(),
                    ));
                }
                continue;
            }
            if !unfalloc || !folio_test_uptodate(f) {
                truncate_inode_folio(mapping, f);
            }
            folio_unlock(f);
        }
        folio_batch_remove_exceptionals(fb);
        folio_batch_release(fb);
        rust_shmem_cond_resched();
    }
    if !unfalloc {
        let mut same = (lstart as u64 >> RUST_SHMEM_PAGE_SHIFT) == (lend >> RUST_SHMEM_PAGE_SHIFT);
        let mut f =
            shmem_get_partial_folio(inode, (lstart as u64 >> RUST_SHMEM_PAGE_SHIFT) as Pgoff);
        if !f.is_null() {
            same = lend < folio_next_pos(f) as uoff_t;
            folio_mark_dirty(f);
            if !truncate_inode_partial_folio(f, lstart, lend) {
                start = folio_next_index(f);
                if same {
                    end = (*rust_shmem_folio_index_ptr(f));
                }
            }
            folio_unlock(f);
            folio_put(f);
            f = null_mut();
        }
        if !same {
            f = shmem_get_partial_folio(inode, (lend >> RUST_SHMEM_PAGE_SHIFT) as Pgoff);
        }
        if !f.is_null() {
            folio_mark_dirty(f);
            if !truncate_inode_partial_folio(f, lstart, lend) {
                end = (*rust_shmem_folio_index_ptr(f));
            }
            folio_unlock(f);
            folio_put(f);
        }
    }
    index = start;
    while index < end {
        rust_shmem_cond_resched();
        if find_get_entries(
            mapping,
            &mut index,
            end.wrapping_sub(1),
            fb,
            indices.as_mut_ptr(),
        ) == 0
        {
            if index == start || end != Pgoff::MAX {
                break;
            }
            index = start;
            continue;
        }
        for i in 0..folio_batch_count(fb) as usize {
            let f = (*fb).folios[i];
            if xa_is_value(f.cast()) {
                if unfalloc {
                    continue;
                }
                let freed = shmem_free_swap(mapping, indices[i], end.wrapping_sub(1), f.cast());
                if freed == 0 {
                    let mut base = indices[i];
                    let order = shmem_confirm_swap(mapping, base, radix_to_swp_entry(f.cast()));
                    if order > 0 {
                        base = round_down(base, 1 << order);
                        if base < start || base.wrapping_add(1 << order) > end {
                            continue;
                        }
                    }
                    index = base;
                    break;
                }
                swaps = swaps.wrapping_add(freed);
                continue;
            }
            folio_lock(f);
            if !unfalloc || !folio_test_uptodate(f) {
                if folio_mapping(f) != mapping {
                    folio_unlock(f);
                    index = indices[i];
                    break;
                }
                vm_bug_folio!(folio_test_writeback(f), f);
                if !folio_test_large(f) {
                    truncate_inode_folio(mapping, f);
                } else if truncate_inode_partial_folio(f, lstart, lend) && !folio_test_large(f) {
                    folio_unlock(f);
                    index = start;
                    break;
                }
            }
            folio_unlock(f);
        }
        folio_batch_remove_exceptionals(fb);
        folio_batch_release(fb);
    }
    shmem_recalc_inode(inode, 0, swaps.wrapping_neg());
}
#[no_mangle]
pub unsafe extern "C" fn shmem_truncate_range(inode: *mut inode, start: loff_t, end: uoff_t) {
    shmem_undo_range(inode, start, end, false);
    inode_set_mtime_to_ts(inode, inode_set_ctime_current(inode));
    inode_inc_iversion(inode);
}
#[export_name = "rust_shmem_owner_shmem_getattr"]
unsafe extern "C" fn shmem_getattr(
    idmap: *mut mnt_idmap,
    path: *const path,
    stat: *mut kstat,
    mask: u32,
    _flags: c_uint,
) -> c_int {
    let inode = (*(*path).dentry).d_inode;
    let info = SHMEM_I(inode);
    if rust_shmem_accounting_stale(inode) {
        shmem_recalc_inode(inode, 0, 0);
    }
    if (*info).fsflags & RUST_SHMEM_FS_APPEND_FL as c_uint != 0 {
        (*stat).attributes |= RUST_SHMEM_STATX_ATTR_APPEND as u64;
    }
    if (*info).fsflags & RUST_SHMEM_FS_IMMUTABLE_FL as c_uint != 0 {
        (*stat).attributes |= RUST_SHMEM_STATX_ATTR_IMMUTABLE as u64;
    }
    if (*info).fsflags & RUST_SHMEM_FS_NODUMP_FL as c_uint != 0 {
        (*stat).attributes |= RUST_SHMEM_STATX_ATTR_NODUMP as u64;
    }
    (*stat).attributes_mask |= (RUST_SHMEM_STATX_ATTR_APPEND
        | RUST_SHMEM_STATX_ATTR_IMMUTABLE
        | RUST_SHMEM_STATX_ATTR_NODUMP) as u64;
    generic_fillattr(idmap, mask, inode, stat);
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    if shmem_huge_global_enabled(inode, 0, 0, false, null_mut(), 0) != 0 {
        (*stat).blksize = RUST_SHMEM_HPAGE_PMD_SIZE as _;
    }
    if mask & RUST_SHMEM_STATX_BTIME as u32 != 0 {
        (*stat).result_mask |= RUST_SHMEM_STATX_BTIME as u32;
        (*stat).btime = (*info).i_crtime;
    }
    0
}
#[export_name = "rust_shmem_owner_shmem_setattr"]
unsafe extern "C" fn shmem_setattr(
    idmap: *mut mnt_idmap,
    dentry: *mut dentry,
    attr: *mut iattr,
) -> c_int {
    let inode = d_inode(dentry);
    let info = SHMEM_I(inode);
    let mut error = setattr_prepare(idmap, dentry, attr);
    let mut mtime = false;
    let mut ctime = true;
    if error != 0 {
        return error;
    }
    if (*info).seals & RUST_SHMEM_F_SEAL_EXEC as c_uint != 0
        && (*attr).ia_valid & RUST_SHMEM_ATTR_MODE as c_uint != 0
        && ((*inode).i_mode ^ (*attr).ia_mode) & 0o111 != 0
    {
        return -(RUST_SHMEM_EPERM as c_int);
    }
    if rust_shmem_isreg((*inode).i_mode) && (*attr).ia_valid & RUST_SHMEM_ATTR_SIZE as c_uint != 0 {
        let old = (*inode).i_size;
        let new = (*attr).ia_size;
        if (new < old && (*info).seals & RUST_SHMEM_F_SEAL_SHRINK as c_uint != 0)
            || (new > old && (*info).seals & RUST_SHMEM_F_SEAL_GROW as c_uint != 0)
        {
            return -(RUST_SHMEM_EPERM as c_int);
        }
        if new != old {
            if (*info).flags & RUST_SHMEM_SHMEM_F_MAPPING_FROZEN as c_ulong != 0 {
                return -(RUST_SHMEM_EPERM as c_int);
            }
            error = shmem_reacct_size((*info).flags, old, new);
            if error != 0 {
                return error;
            }
            i_size_write(inode, new);
            mtime = true;
        } else {
            ctime = false;
        }
        if new <= old {
            let hole = round_up_u64(new as u64, RUST_SHMEM_PAGE_SIZE as u64) as loff_t;
            if old > hole {
                unmap_mapping_range((*inode).i_mapping, hole, 0, 1);
            }
            if (*info).alloced != 0 {
                shmem_truncate_range(inode, new, uoff_t::MAX);
            }
            if old > hole {
                unmap_mapping_range((*inode).i_mapping, hole, 0, 1);
            }
        }
    }
    if is_quota_modification(idmap, inode, attr) {
        error = dquot_initialize(inode);
        if error != 0 {
            return error;
        }
    }
    if i_uid_needs_update(idmap, attr, inode) || i_gid_needs_update(idmap, attr, inode) {
        error = dquot_transfer(idmap, inode, attr);
        if error != 0 {
            return error;
        }
    }
    setattr_copy(idmap, inode, attr);
    if (*attr).ia_valid & RUST_SHMEM_ATTR_MODE as c_uint != 0 {
        error = posix_acl_chmod(idmap, dentry, (*inode).i_mode);
    }
    if error == 0 && ctime {
        inode_set_ctime_current(inode);
        if mtime {
            inode_set_mtime_to_ts(inode, inode_get_ctime(inode));
        }
        inode_inc_iversion(inode);
    }
    error
}
#[export_name = "rust_shmem_owner_shmem_evict_inode"]
unsafe extern "C" fn shmem_evict_inode(inode: *mut inode) {
    let info = SHMEM_I(inode);
    let sb = SHMEM_SB((*inode).i_sb);
    let mut freed: usize = 0;
    if shmem_mapping((*inode).i_mapping) {
        shmem_unacct_size((*info).flags, (*inode).i_size);
        (*inode).i_size = 0;
        mapping_set_exiting((*inode).i_mapping);
        shmem_truncate_range(inode, 0, uoff_t::MAX);
        let shrink = rust_shmem_inode_shrinklist(info);
        let swap = rust_shmem_inode_swaplist(info);
        if !rust_shmem_list_empty(shrink) {
            spin_lock(addr_of_mut!((*sb).shrinklist_lock));
            if !rust_shmem_list_empty(shrink) {
                list_del_init(shrink);
                (*sb).shrinklist_len = (*sb).shrinklist_len.wrapping_sub(1);
            }
            spin_unlock(addr_of_mut!((*sb).shrinklist_lock));
        }
        while !rust_shmem_list_empty(swap) {
            rust_shmem_wait_eviction(info);
            spin_lock(addr_of_mut!(rust_shmem_data_shmem_swaplist_lock));
            if atomic_read(addr_of!((*info).stop_eviction)) == 0 {
                list_del_init(swap);
            }
            spin_unlock(addr_of_mut!(rust_shmem_data_shmem_swaplist_lock));
        }
    }
    simple_xattrs_free(
        addr_of_mut!((*sb).xa_cache),
        addr_of_mut!((*info).xattrs),
        if (*sb).max_inodes != 0 {
            &mut freed
        } else {
            null_mut()
        },
    );
    shmem_free_inode((*inode).i_sb, freed);
    if (*inode).i_blocks != 0 {
        rust_shmem_eviction_warning(inode);
    }
    clear_inode(inode);
    #[cfg(CONFIG_TMPFS_QUOTA)]
    {
        dquot_free_inode(inode);
        dquot_drop(inode);
    }
}
