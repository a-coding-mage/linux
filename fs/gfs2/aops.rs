// SPDX-License-Identifier: GPL-2.0-only
// Direct Rust translation of aops.c. Kernel and GFS2 dependencies are external.

unsafe fn gfs2_get_block_noalloc(inode: *mut inode, lblock: sector_t,
    bh_result: *mut buffer_head, _create: c_int) -> c_int {
    let error = gfs2_block_map(inode, lblock, bh_result, 0);
    if error != 0 { return error; }
    if !buffer_mapped(bh_result) { return -ENODATA; }
    0
}

unsafe fn gfs2_write_jdata_folio(folio: *mut folio, wbc: *mut writeback_control) -> c_int {
    let inode = (*(*folio).mapping).host;
    let i_size = i_size_read(inode);
    if folio_pos(folio) < i_size && i_size < folio_next_pos(folio) {
        folio_zero_segment(folio, offset_in_folio(folio, i_size), folio_size(folio));
    }
    __block_write_full_folio(inode, folio, gfs2_get_block_noalloc, wbc)
}

unsafe fn __gfs2_jdata_write_folio(folio: *mut folio, wbc: *mut writeback_control) -> c_int {
    let inode = (*(*folio).mapping).host;
    let ip = GFS2_I(inode);
    if folio_test_checked(folio) {
        folio_clear_checked(folio);
        if folio_buffers(folio).is_null() {
            create_empty_buffers(folio, (*(*inode).i_sb).s_blocksize,
                BIT(BH_Dirty) | BIT(BH_Uptodate));
        }
        gfs2_trans_add_databufs((*ip).i_gl, folio, 0, folio_size(folio));
    }
    gfs2_write_jdata_folio(folio, wbc)
}

pub unsafe fn gfs2_jdata_writeback(mapping: *mut address_space,
    wbc: *mut writeback_control) -> c_int {
    let inode = (*mapping).host;
    let ip = GFS2_I(inode);
    let sdp = GFS2_SB((*mapping).host);
    let mut folio: *mut folio = core::ptr::null_mut();
    let mut error = 0;
    BUG_ON(!(*current).journal_info.is_null());
    if gfs2_assert_withdraw(sdp, (*(*ip).i_gl).gl_state == LM_ST_EXCLUSIVE) != 0 { return 0; }
    while { folio = writeback_iter(mapping, wbc, folio, &mut error); !folio.is_null() } {
        if folio_test_checked(folio) { folio_redirty_for_writepage(wbc, folio); folio_unlock(folio); continue; }
        error = __gfs2_jdata_write_folio(folio, wbc);
    }
    error
}

unsafe fn gfs2_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> c_int {
    let sdp = gfs2_mapping2sbd(mapping);
    let initial_nr_to_write = (*wbc).nr_to_write;
    let mut wpc = iomap_writepage_ctx { inode: (*mapping).host, wbc, ops: &gfs2_writeback_ops };
    let ret = iomap_writepages(&mut wpc);
    if ret == 0 && (*wbc).nr_to_write == initial_nr_to_write { set_bit(SDF_FORCE_AIL_FLUSH, &mut (*sdp).sd_flags); }
    ret
}

unsafe fn gfs2_write_jdata_batch(mapping: *mut address_space, wbc: *mut writeback_control,
    fbatch: *mut folio_batch, done_index: *mut pgoff_t) -> c_int {
    let inode = (*mapping).host; let sdp = GFS2_SB(inode); let mut size = 0usize;
    let nr_folios = folio_batch_count(fbatch); let mut ret;
    for i in 0..nr_folios { size += folio_size((*fbatch).folios[i as usize]); }
    let nrblocks = size >> (*inode).i_blkbits;
    ret = gfs2_trans_begin(sdp, nrblocks, nrblocks); if ret < 0 { return ret; }
    for i in 0..nr_folios {
        let folio = (*fbatch).folios[i as usize]; *done_index = (*folio).index; folio_lock(folio);
        if (*folio).mapping != mapping { folio_unlock(folio); continue; }
        if !folio_test_dirty(folio) { folio_unlock(folio); continue; }
        if folio_test_writeback(folio) { if (*wbc).sync_mode != WB_SYNC_NONE { folio_wait_writeback(folio); } else { folio_unlock(folio); continue; } }
        BUG_ON(folio_test_writeback(folio)); if !folio_clear_dirty_for_io(folio) { folio_unlock(folio); continue; }
        trace_wbc_writepage(wbc, inode_to_bdi(inode)); ret = __gfs2_jdata_write_folio(folio, wbc);
        if ret != 0 { *done_index = folio_next_index(folio); ret = 1; break; }
        if { (*wbc).nr_to_write -= 1; (*wbc).nr_to_write <= 0 } && (*wbc).sync_mode == WB_SYNC_NONE { ret = 1; break; }
    }
    gfs2_trans_end(sdp); ret
}

unsafe fn gfs2_write_cache_jdata(mapping: *mut address_space, wbc: *mut writeback_control) -> c_int {
    let mut ret = 0; let mut done = false; let mut fbatch = core::mem::MaybeUninit::<folio_batch>::uninit();
    folio_batch_init(fbatch.as_mut_ptr()); let fbatch = fbatch.as_mut_ptr(); let mut writeback_index; let mut index; let mut end; let mut cycled; let mut range_whole = false; let tag;
    if (*wbc).range_cyclic { writeback_index = (*mapping).writeback_index; index = writeback_index; cycled = index == 0; end = -1; }
    else { index = (*wbc).range_start >> PAGE_SHIFT; end = (*wbc).range_end >> PAGE_SHIFT; range_whole = (*wbc).range_start == 0 && (*wbc).range_end == LLONG_MAX; cycled = true; }
    tag = wbc_to_tag(wbc);
    'retry: loop {
        if (*wbc).sync_mode == WB_SYNC_ALL || (*wbc).tagged_writepages { tag_pages_for_writeback(mapping, index, end); }
        let mut done_index = index;
        while !done && index <= end {
            let nr = filemap_get_folios_tag(mapping, &mut index, end, tag, fbatch); if nr == 0 { break; }
            ret = gfs2_write_jdata_batch(mapping, wbc, fbatch, &mut done_index); if ret != 0 { done = true; } if ret > 0 { ret = 0; }
            folio_batch_release(fbatch); cond_resched();
        }
        if !cycled && !done { cycled = true; index = 0; end = writeback_index - 1; continue 'retry; }
        if (*wbc).range_cyclic || (range_whole && (*wbc).nr_to_write > 0) { (*mapping).writeback_index = done_index; }
        return ret;
    }
}

unsafe fn gfs2_jdata_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> c_int {
    let ip = GFS2_I((*mapping).host); let sdp = GFS2_SB((*mapping).host); let mut ret = gfs2_write_cache_jdata(mapping, wbc);
    if ret == 0 && (*wbc).sync_mode == WB_SYNC_ALL { gfs2_log_flush(sdp, (*ip).i_gl, GFS2_LOG_HEAD_FLUSH_NORMAL | GFS2_LFC_JDATA_WPAGES); ret = gfs2_write_cache_jdata(mapping, wbc); } ret
}

unsafe fn stuffed_read_folio(ip: *mut gfs2_inode, folio: *mut folio) -> c_int {
    let mut dibh = core::ptr::null_mut(); let mut dsize = i_size_read(&mut (*ip).i_inode); let mut from = core::ptr::null_mut(); let mut error = 0;
    if (*folio).index != 0 { dsize = 0; } else { error = gfs2_meta_inode_buffer(ip, &mut dibh); if error != 0 { folio_end_read(folio, false); return error; } from = (*dibh).b_data.add(core::mem::size_of::<gfs2_dinode>()); }
    folio_fill_tail(folio, 0, from, dsize); brelse(dibh); folio_end_read(folio, error == 0); error
}

unsafe fn gfs2_read_folio(_file: *mut file, folio: *mut folio) -> c_int {
    let inode = (*(*folio).mapping).host; let ip = GFS2_I(inode); let sdp = GFS2_SB(inode); let error;
    if !gfs2_is_jdata(ip) || (i_blocksize(inode) == PAGE_SIZE && folio_buffers(folio).is_null()) { iomap_bio_read_folio(folio, &gfs2_iomap_ops); error = 0; }
    else if gfs2_is_stuffed(ip) { error = stuffed_read_folio(ip, folio); } else { error = mpage_read_folio(folio, gfs2_block_map); }
    if gfs2_withdrawn(sdp) { return -EIO; } error
}

pub unsafe fn gfs2_internal_read(ip: *mut gfs2_inode, buf: *mut c_char, pos: *mut loff_t, size: usize) -> isize {
    let mapping = (*ip).i_inode.i_mapping; let mut index = (*pos >> PAGE_SHIFT) as pgoff_t; let mut copied = 0usize;
    while copied < size { let folio = read_cache_folio(mapping, index, gfs2_read_folio, core::ptr::null_mut()); if IS_ERR(folio) { if PTR_ERR(folio) == -EINTR { continue; } return PTR_ERR(folio); } let offset = (*pos as usize) + copied - folio_pos(folio); let chunk = core::cmp::min(size - copied, folio_size(folio) - offset); memcpy_from_folio(buf.add(copied), folio, offset, chunk); index = folio_next_index(folio); folio_put(folio); copied += chunk; }
    *pos += size as loff_t; size as isize
}

unsafe fn gfs2_readahead(rac: *mut readahead_control) { let ip = GFS2_I((*(*rac).mapping).host); if gfs2_is_stuffed(ip) {} else if gfs2_is_jdata(ip) { mpage_readahead(rac, gfs2_block_map); } else { iomap_bio_readahead(rac, &gfs2_iomap_ops); } }

pub unsafe fn adjust_fs_space(inode: *mut inode) {
    let sdp = GFS2_SB(inode); let m_ip = GFS2_I((*sdp).sd_statfs_inode); let m_sc = &mut (*sdp).sd_statfs_master; let l_sc = &mut (*sdp).sd_statfs_local; let mut m_bh = core::ptr::null_mut();
    if gfs2_trans_begin(sdp, 2 * RES_STATFS, 0) != 0 { return; } let fs_total = gfs2_ri_total(sdp); if gfs2_meta_inode_buffer(m_ip, &mut m_bh) != 0 { (*sdp).sd_rindex_uptodate = 0; gfs2_trans_end(sdp); return; }
    spin_lock(&mut (*sdp).sd_statfs_spin); gfs2_statfs_change_in(m_sc, (*m_bh).b_data.add(core::mem::size_of::<gfs2_dinode>())); let new_free = if fs_total > m_sc.sc_total + l_sc.sc_total { fs_total - m_sc.sc_total - l_sc.sc_total } else { 0 }; spin_unlock(&mut (*sdp).sd_statfs_spin); fs_warn(sdp, "File system extended by %llu blocks.\n", new_free as u64); gfs2_statfs_change(sdp, new_free, new_free, 0); update_statfs(sdp, m_bh); brelse(m_bh); (*sdp).sd_rindex_uptodate = 0; gfs2_trans_end(sdp);
}

unsafe fn gfs2_jdata_dirty_folio(mapping: *mut address_space, folio: *mut folio) -> bool { if !(*current).journal_info.is_null() { folio_set_checked(folio); } block_dirty_folio(mapping, folio) }
unsafe fn gfs2_bmap(mapping: *mut address_space, lblock: sector_t) -> sector_t { let ip = GFS2_I((*mapping).host); let mut gh = core::mem::MaybeUninit::uninit(); if gfs2_glock_nq_init((*ip).i_gl, LM_ST_SHARED, LM_FLAG_ANY, gh.as_mut_ptr()) != 0 { return 0; } let d = if !gfs2_is_stuffed(ip) { iomap_bmap(mapping, lblock, &gfs2_iomap_ops) } else { 0 }; gfs2_glock_dq_uninit(gh.as_mut_ptr()); d }

// The remaining buffer invalidation/release callbacks retain the C kernel API shape.
unsafe fn gfs2_discard(sdp: *mut gfs2_sbd, bh: *mut buffer_head) { lock_buffer(bh); spin_lock(&mut (*sdp).sd_log_lock); clear_buffer_dirty(bh); let bd = (*bh).b_private as *mut gfs2_bufdata; if !bd.is_null() { if !list_empty(&(*bd).bd_list) && !buffer_pinned(bh) { list_del_init(&mut (*bd).bd_list); } else { spin_lock(&mut (*sdp).sd_ail_lock); gfs2_remove_from_journal(bh, REMOVE_JDATA); spin_unlock(&mut (*sdp).sd_ail_lock); } } (*bh).b_bdev = core::ptr::null_mut(); clear_buffer_mapped(bh); clear_buffer_req(bh); clear_buffer_new(bh); spin_unlock(&mut (*sdp).sd_log_lock); unlock_buffer(bh); }

unsafe fn gfs2_invalidate_folio(folio: *mut folio, offset: usize, length: usize) { let sdp = GFS2_SB((*(*folio).mapping).host); let stop = offset + length; let partial = offset != 0 || length < folio_size(folio); let mut bh = folio_buffers(folio); let head = bh; let mut pos = 0; BUG_ON(!folio_test_locked(folio)); if !partial { folio_clear_checked(folio); } if head.is_null() { if !partial { filemap_release_folio(folio, 0); } return; } loop { if pos + (*bh).b_size > stop { return; } if offset <= pos { gfs2_discard(sdp, bh); } pos += (*bh).b_size; bh = (*bh).b_this_page; if bh == head { break; } } if !partial { filemap_release_folio(folio, 0); } }

pub unsafe fn gfs2_release_folio(folio: *mut folio, _gfp_mask: gfp_t) -> bool { let mapping = (*folio).mapping; let sdp = gfs2_mapping2sbd(mapping); let head = folio_buffers(folio); if head.is_null() { return false; } spin_lock(&mut (*sdp).sd_log_lock); let mut bh = head; loop { let bd = (*bh).b_private as *mut gfs2_bufdata; if atomic_read(&(*bh).b_count) != 0 || (!bd.is_null() && !(*bd).bd_tr.is_null()) || buffer_dirty(bh) || WARN_ON(buffer_pinned(bh)) { spin_unlock(&mut (*sdp).sd_log_lock); return false; } bh = (*bh).b_this_page; if bh == head { break; } } bh = head; loop { let bd = (*bh).b_private as *mut gfs2_bufdata; if !bd.is_null() { gfs2_assert_warn(sdp, (*bd).bd_bh == bh); (*bd).bd_bh = core::ptr::null_mut(); (*bh).b_private = core::ptr::null_mut(); if (*bd).bd_blkno == 0 && !list_empty(&(*bd).bd_list) { list_del_init(&mut (*bd).bd_list); } if list_empty(&(*bd).bd_list) { kmem_cache_free(gfs2_bufdata_cachep, bd); } } bh = (*bh).b_this_page; if bh == head { break; } } spin_unlock(&mut (*sdp).sd_log_lock); try_to_free_buffers(folio) }

static gfs2_aops: address_space_operations = address_space_operations {
    writepages: Some(gfs2_writepages), read_folio: Some(gfs2_read_folio), readahead: Some(gfs2_readahead),
    dirty_folio: Some(iomap_dirty_folio), release_folio: Some(iomap_release_folio),
    invalidate_folio: Some(iomap_invalidate_folio), bmap: Some(gfs2_bmap),
    migrate_folio: Some(filemap_migrate_folio), is_partially_uptodate: Some(iomap_is_partially_uptodate),
    error_remove_folio: Some(generic_error_remove_folio),
};

static gfs2_jdata_aops: address_space_operations = address_space_operations {
    writepages: Some(gfs2_jdata_writepages), read_folio: Some(gfs2_read_folio), readahead: Some(gfs2_readahead),
    dirty_folio: Some(gfs2_jdata_dirty_folio), bmap: Some(gfs2_bmap),
    migrate_folio: Some(buffer_migrate_folio), invalidate_folio: Some(gfs2_invalidate_folio),
    release_folio: Some(gfs2_release_folio), is_partially_uptodate: Some(block_is_partially_uptodate),
    error_remove_folio: Some(generic_error_remove_folio),
};

pub unsafe fn gfs2_set_aops(inode: *mut inode) { if gfs2_is_jdata(GFS2_I(inode)) { (*(*inode).i_mapping).a_ops = &gfs2_jdata_aops; } else { (*(*inode).i_mapping).a_ops = &gfs2_aops; } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
