// SPDX-License-Identifier: LGPL-2.1
/* C kernel dependencies are supplied by the surrounding translation unit. */

#[repr(C)]
struct mext_data {
    orig_inode: *mut inode,
    donor_inode: *mut inode,
    orig_map: ext4_map_blocks,
    donor_lblk: ext4_lblk_t,
}

pub unsafe fn ext4_double_down_write_data_sem(first: *mut inode, second: *mut inode) {
    if first < second {
        down_write(&mut (*EXT4_I(first)).i_data_sem);
        down_write_nested(&mut (*EXT4_I(second)).i_data_sem, I_DATA_SEM_OTHER);
    } else {
        down_write(&mut (*EXT4_I(second)).i_data_sem);
        down_write_nested(&mut (*EXT4_I(first)).i_data_sem, I_DATA_SEM_OTHER);
    }
}

pub unsafe fn ext4_double_up_write_data_sem(orig_inode: *mut inode, donor_inode: *mut inode) {
    up_write(&mut (*EXT4_I(orig_inode)).i_data_sem);
    up_write(&mut (*EXT4_I(donor_inode)).i_data_sem);
}

unsafe fn mext_folio_double_lock(inode1: *mut inode, inode2: *mut inode, mut index1: pgoff_t,
    mut index2: pgoff_t, len: usize, folio: *mut [*mut folio; 2]) -> i32 {
    let mut mapping: [*mut address_space; 2] = [core::ptr::null_mut(); 2];
    let flags: u32;
    let mut fgp_flags: fgf_t = FGP_WRITEBEGIN;
    BUG_ON(!(!inode1 || !inode2));
    if inode1 < inode2 {
        mapping[0] = (*inode1).i_mapping; mapping[1] = (*inode2).i_mapping;
    } else {
        core::mem::swap(&mut index1, &mut index2);
        mapping[0] = (*inode2).i_mapping; mapping[1] = (*inode1).i_mapping;
    }
    flags = memalloc_nofs_save();
    fgp_flags |= fgf_set_order(len);
    (*folio)[0] = __filemap_get_folio(mapping[0], index1, fgp_flags, mapping_gfp_mask(mapping[0]));
    if IS_ERR((*folio)[0]) {
        memalloc_nofs_restore(flags); return PTR_ERR((*folio)[0]);
    }
    (*folio)[1] = __filemap_get_folio(mapping[1], index2, fgp_flags, mapping_gfp_mask(mapping[1]));
    memalloc_nofs_restore(flags);
    if IS_ERR((*folio)[1]) {
        folio_unlock((*folio)[0]); folio_put((*folio)[0]); return PTR_ERR((*folio)[1]);
    }
    folio_wait_writeback((*folio)[0]); folio_wait_writeback((*folio)[1]);
    if inode1 > inode2 { (*folio).swap(0, 1); }
    0
}

unsafe fn mext_folio_double_unlock(folio: *mut [*mut folio; 2]) {
    folio_unlock((*folio)[0]); folio_put((*folio)[0]);
    folio_unlock((*folio)[1]); folio_put((*folio)[1]);
}

unsafe fn mext_folio_mkuptodate(folio: *mut folio, from: usize, to: usize) -> i32 {
    let inode = (*(*folio).mapping).host;
    let mut block: sector_t;
    let (mut bh, head): (*mut buffer_head, *mut buffer_head);
    let blocksize = i_blocksize(inode);
    let (mut block_start, mut block_end): (u32, u32);
    let mut nr = 0; let mut partial = false;
    BUG_ON(!folio_test_locked(folio)); BUG_ON(folio_test_writeback(folio));
    if folio_test_uptodate(folio) { return 0; }
    head = { let h = folio_buffers(folio); if !h { create_empty_buffers(folio, blocksize, 0) } else { h } };
    block = folio_pos(folio) >> (*inode).i_blkbits; block_end = 0; bh = head;
    loop {
        block_start = block_end; block_end = block_start + blocksize;
        if block_end <= from as u32 || block_start >= to as u32 { if !buffer_uptodate(bh) { partial = true; } }
        else if !buffer_uptodate(bh) {
            if !buffer_mapped(bh) { let err = ext4_get_block(inode, block, bh, 0); if err != 0 { return err; } if !buffer_mapped(bh) { folio_zero_range(folio, block_start as usize, blocksize as usize); set_buffer_uptodate(bh); block += 1; bh = (*bh).b_this_page; if bh == head { break; } continue; } }
            lock_buffer(bh); if buffer_uptodate(bh) { unlock_buffer(bh); } else { ext4_read_bh_nowait(bh, 0, core::ptr::null_mut(), false); nr += 1; }
        }
        block += 1; bh = (*bh).b_this_page; if bh == head { break; }
    }
    if nr != 0 { bh = head; loop { if bh_offset(bh) + blocksize <= from as u64 { } else if bh_offset(bh) >= to as u64 { break; } else { wait_on_buffer(bh); if !buffer_uptodate(bh) { return -EIO; } } bh = (*bh).b_this_page; if bh == head { break; } } }
    if !partial { folio_mark_uptodate(folio); } 0
}

#[repr(C)]
enum mext_move_type { MEXT_SKIP_EXTENT, MEXT_MOVE_EXTENT, MEXT_COPY_DATA }

unsafe fn mext_move_begin(mext: *mut mext_data, folio: *mut [*mut folio; 2], move_type: *mut mext_move_type) -> i32 {
    let orig_inode = (*mext).orig_inode; let donor_inode = (*mext).donor_inode; let blkbits = (*orig_inode).i_blkbits;
    let orig_pos = ((*mext).orig_map.m_lblk as i64) << blkbits; let donor_pos = ((*mext).donor_lblk as i64) << blkbits;
    let ret = mext_folio_double_lock(orig_inode, donor_inode, (orig_pos >> PAGE_SHIFT) as pgoff_t, (donor_pos >> PAGE_SHIFT) as pgoff_t, ((*mext).orig_map.m_len << blkbits) as usize, folio); if ret != 0 { return ret; }
    if (*mext).orig_map.m_seq != READ_ONCE((*EXT4_I(orig_inode)).i_es_seq) { mext_folio_double_unlock(folio); return -ESTALE; }
    let move_len = (umin(folio_next_pos((*folio)[0]) - orig_pos as u64, folio_next_pos((*folio)[1]) - donor_pos as u64) >> blkbits) as u32;
    if move_len < (*mext).orig_map.m_len { (*mext).orig_map.m_len = move_len; }
    let mut donor_map: ext4_map_blocks = core::mem::zeroed(); donor_map.m_lblk = (*mext).donor_lblk; donor_map.m_len = (*mext).orig_map.m_len; donor_map.m_flags = 0;
    let ret = ext4_map_blocks(core::ptr::null_mut(), donor_inode, &mut donor_map, 0); if ret < 0 { mext_folio_double_unlock(folio); return ret; }
    (*mext).orig_map.m_len = donor_map.m_len;
    *move_type = if donor_map.m_flags & (EXT4_MAP_MAPPED | EXT4_MAP_UNWRITTEN) == 0 { MEXT_SKIP_EXTENT } else if (*mext).orig_map.m_flags & EXT4_MAP_UNWRITTEN != 0 && donor_map.m_flags & EXT4_MAP_UNWRITTEN != 0 { MEXT_MOVE_EXTENT } else { MEXT_COPY_DATA }; 0
}

unsafe fn mext_folio_mkwrite(inode: *mut inode, folio: *mut folio, from: usize, to: usize) -> i32 {
    let blocksize = i_blocksize(inode); let head = { let h = folio_buffers(folio); if !h { create_empty_buffers(folio, blocksize, 0) } else { h } }; let mut block = folio_pos(folio) >> (*inode).i_blkbits; let mut end = 0; let mut bh = head;
    loop { let start = end; end += blocksize; if end > from as u32 && start < to as u32 { let ret = ext4_get_block(inode, block, bh, 0); if ret != 0 { return ret; } } block += 1; bh = (*bh).b_this_page; if bh == head { break; } } block_commit_write(folio, from, to); 0
}

unsafe fn mext_move_extent(mext: *mut mext_data, m_len: *mut u64) -> i32 {
    let orig_inode = (*mext).orig_inode; let donor_inode = (*mext).donor_inode; let orig_map = &mut (*mext).orig_map; let blkbits = (*orig_inode).i_blkbits; let mut folio = [core::ptr::null_mut(); 2]; let mut move_type = MEXT_SKIP_EXTENT; let mut r_len = 0u64; let mut ret; let credits = ext4_chunk_trans_extent(orig_inode, 0) * 2; let handle = ext4_journal_start(orig_inode, EXT4_HT_MOVE_EXTENTS, credits); *m_len = 0; if IS_ERR(handle) { return PTR_ERR(handle); } ext4_fc_mark_ineligible((*orig_inode).i_sb, EXT4_FC_REASON_MOVE_EXT, handle);
    ret = mext_move_begin(mext, &mut folio, &mut move_type); if ret != 0 { ext4_journal_stop(handle); return ret; } if let MEXT_SKIP_EXTENT = move_type { mext_folio_double_unlock(&mut folio); ext4_journal_stop(handle); return 0; }
    let mut from = 0usize; if let MEXT_COPY_DATA = move_type { from = offset_in_folio(folio[0], ((orig_map.m_lblk as i64) << blkbits) as u64); let length = (orig_map.m_len as usize) << blkbits; ret = mext_folio_mkuptodate(folio[0], from, from + length); if ret != 0 { mext_folio_double_unlock(&mut folio); ext4_journal_stop(handle); return ret; } }
    if !filemap_release_folio(folio[0], 0) || !filemap_release_folio(folio[1], 0) { mext_folio_double_unlock(&mut folio); ext4_journal_stop(handle); return -EBUSY; }
    ext4_double_down_write_data_sem(orig_inode, donor_inode); *m_len = ext4_swap_extents(handle, orig_inode, donor_inode, orig_map.m_lblk, (*mext).donor_lblk, orig_map.m_len, 1, &mut ret); ext4_double_up_write_data_sem(orig_inode, donor_inode);
    if ret == 0 && *m_len != orig_map.m_len { ret = -EIO; } if *m_len != 0 && !matches!(move_type, MEXT_MOVE_EXTENT) { let length = *m_len << blkbits; let ret2 = mext_folio_mkwrite(orig_inode, folio[0], from, from + length as usize); if ret2 != 0 { if ret == 0 { ret = ret2; } ext4_double_down_write_data_sem(orig_inode, donor_inode); r_len = ext4_swap_extents(handle, donor_inode, orig_inode, (*mext).donor_lblk, orig_map.m_lblk, *m_len, 0, &mut ret); ext4_double_up_write_data_sem(orig_inode, donor_inode); if ret != 0 || r_len != *m_len { ext4_error_inode_block(orig_inode, orig_map.m_lblk as sector_t, EIO, "Unable to copy data block, data will be lost!"); ret = -EIO; } *m_len = 0; } else { let ret2 = ext4_jbd2_inode_add_write(handle, orig_inode, ((orig_map.m_lblk as i64) << blkbits) as u64, length); if ret == 0 { ret = ret2; } } }
    mext_folio_double_unlock(&mut folio); ext4_journal_stop(handle); ret
}

unsafe fn mext_check_validity(orig_inode: *mut inode, donor_inode: *mut inode) -> i32 {
    let sb = (*orig_inode).i_sb;
    if orig_inode == donor_inode || (*orig_inode).i_sb != (*donor_inode).i_sb || !S_ISREG((*orig_inode).i_mode) || !S_ISREG((*donor_inode).i_mode) { return -EINVAL; }
    if ext4_has_feature_bigalloc(sb) || IS_DAX(orig_inode) || ext4_should_journal_data(orig_inode) || ext4_should_journal_data(donor_inode) || IS_ENCRYPTED(orig_inode) || IS_ENCRYPTED(donor_inode) || !ext4_test_inode_flag(orig_inode, EXT4_INODE_EXTENTS) || !ext4_test_inode_flag(donor_inode, EXT4_INODE_EXTENTS) { return -EOPNOTSUPP; }
    if (*donor_inode).i_mode & (S_ISUID | S_ISGID) != 0 { return -EINVAL; } if IS_IMMUTABLE(donor_inode) || IS_APPEND(donor_inode) { return -EPERM; } if IS_SWAPFILE(orig_inode) || IS_SWAPFILE(donor_inode) { return -ETXTBSY; } if ext4_is_quota_file(orig_inode) || ext4_is_quota_file(donor_inode) { return -EOPNOTSUPP; } if (*orig_inode).i_size == 0 || (*donor_inode).i_size == 0 { return -EINVAL; } 0
}

unsafe fn mext_check_adjust_range(orig_inode: *mut inode, donor_inode: *mut inode, orig_start: u64, donor_start: u64, len: *mut u64) -> i32 {
    if (orig_start & !(PAGE_MASK >> (*orig_inode).i_blkbits)) != (donor_start & !(PAGE_MASK >> (*orig_inode).i_blkbits)) { return -EINVAL; }
    if orig_start >= EXT_MAX_BLOCKS as u64 || donor_start >= EXT_MAX_BLOCKS as u64 || *len > EXT_MAX_BLOCKS as u64 || donor_start + *len >= EXT_MAX_BLOCKS as u64 || orig_start + *len >= EXT_MAX_BLOCKS as u64 { return -EINVAL; }
    let oe = EXT4_B_TO_LBLK(orig_inode, i_size_read(orig_inode)); let de = EXT4_B_TO_LBLK(donor_inode, i_size_read(donor_inode)); if oe <= orig_start { *len = 0; } else if oe < orig_start + *len - 1 { *len = oe - orig_start; } if de <= donor_start { *len = 0; } else if de < donor_start + *len - 1 { *len = de - donor_start; } if *len == 0 { return -EINVAL; } 0
}

pub unsafe fn ext4_move_extents(o_filp: *mut file, d_filp: *mut file, mut orig_blk: u64, mut donor_blk: u64, mut len: u64, moved_len: *mut u64) -> i32 {
    let orig_inode = file_inode(o_filp); let donor_inode = file_inode(d_filp); let sb = (*orig_inode).i_sb; let sbi = EXT4_SB(sb); let mut retries = 0; let mut ret = 0; *moved_len = 0; lock_two_nondirectories(orig_inode, donor_inode); ret = mext_check_validity(orig_inode, donor_inode); if ret != 0 { unlock_two_nondirectories(orig_inode, donor_inode); return ret; } inode_dio_wait(orig_inode); inode_dio_wait(donor_inode); ret = mext_check_adjust_range(orig_inode, donor_inode, orig_blk, donor_blk, &mut len); if ret != 0 { unlock_two_nondirectories(orig_inode, donor_inode); return ret; }
    let mut mext: mext_data = core::mem::zeroed(); mext.orig_inode = orig_inode; mext.donor_inode = donor_inode; while len != 0 { mext.orig_map.m_lblk = orig_blk; mext.orig_map.m_len = len as u32; mext.orig_map.m_flags = 0; mext.donor_lblk = donor_blk; ret = ext4_map_blocks(core::ptr::null_mut(), orig_inode, &mut mext.orig_map, 0); if ret < 0 { break; } if mext.orig_map.m_flags & (EXT4_MAP_MAPPED | EXT4_MAP_UNWRITTEN) != 0 { let mut ml = 0; ret = mext_move_extent(&mut mext, &mut ml); *moved_len += ml; if ret != 0 { if ml != 0 { orig_blk += ml; donor_blk += ml; len -= ml; } if ret == -ESTALE { continue; } if ret == -ENOSPC && ext4_should_retry_alloc(sb, &mut retries) { continue; } if ret == -EBUSY && (*sbi).s_journal != core::ptr::null_mut() && { retries += 1; retries <= 4 } && jbd2_journal_force_commit_nested((*sbi).s_journal) { continue; } break; } } orig_blk += mext.orig_map.m_len as u64; donor_blk += mext.orig_map.m_len as u64; len -= mext.orig_map.m_len as u64; retries = 0; }
    if *moved_len != 0 { ext4_discard_preallocations(orig_inode); ext4_discard_preallocations(donor_inode); } unlock_two_nondirectories(orig_inode, donor_inode); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
