// SPDX-License-Identifier: GPL-2.0+
/*
 * Meta data file for NILFS
 *
 * Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Written by Ryusuke Konishi.
 */

// Dependencies supplied by the surrounding kernel/NILFS translation.

const NILFS_MDT_MAX_RA_BLOCKS: i32 = 16 - 1;

unsafe fn nilfs_mdt_insert_new_block(
    inode: *mut inode,
    block: c_ulong,
    bh: *mut buffer_head,
    init_block: Option<unsafe extern "C" fn(*mut inode, *mut buffer_head, *mut c_void)>,
) -> c_int {
    let ii = NILFS_I(inode);
    let folio = (*bh).b_folio;
    let from: *mut c_void;
    let mut ret: c_int;

    // Caller exclude read accesses using page lock
    (*bh).b_blocknr = 0;

    ret = nilfs_bmap_insert((*ii).i_bmap, block, bh as c_ulong);
    if unlikely(ret != 0) { return ret; }

    set_buffer_mapped(bh);
    from = kmap_local_folio(folio, offset_in_folio(folio, (*bh).b_data));
    memset(from, 0, (*bh).b_size as usize);
    if let Some(f) = init_block { f(inode, bh, from); }
    kunmap_local(from);
    flush_dcache_folio(folio);
    set_buffer_uptodate(bh);
    mark_buffer_dirty(bh);
    nilfs_mdt_mark_dirty(inode);
    trace_nilfs2_mdt_insert_new_block(inode, (*inode).i_ino, block);
    0
}

unsafe fn nilfs_mdt_create_block(
    inode: *mut inode, block: c_ulong, out_bh: *mut *mut buffer_head,
    init_block: Option<unsafe extern "C" fn(*mut inode, *mut buffer_head, *mut c_void)>,
) -> c_int {
    let sb = (*inode).i_sb;
    let mut ti: nilfs_transaction_info = core::mem::zeroed();
    let bh: *mut buffer_head;
    let mut err: c_int;
    nilfs_transaction_begin(sb, &mut ti, 0);
    err = -ENOMEM;
    bh = nilfs_grab_buffer(inode, (*inode).i_mapping, block, 0);
    if unlikely(bh.is_null()) { return nilfs_transaction_abort(sb); }
    err = -EEXIST;
    if buffer_uptodate(bh) { goto_failed_bh(inode, bh, sb, err); }
    wait_on_buffer(bh);
    if buffer_uptodate(bh) { goto_failed_bh(inode, bh, sb, err); }
    err = nilfs_mdt_insert_new_block(inode, block, bh, init_block);
    if likely(err == 0) { get_bh(bh); *out_bh = bh; }
    folio_unlock((*bh).b_folio); folio_put((*bh).b_folio); brelse(bh);
    if likely(err == 0) { nilfs_transaction_commit(sb) } else { nilfs_transaction_abort(sb) }
}

// C's shared cleanup labels are represented by this local helper.
unsafe fn goto_failed_bh(inode: *mut inode, bh: *mut buffer_head, sb: *mut super_block, mut err: c_int) -> c_int {
    folio_unlock((*bh).b_folio); folio_put((*bh).b_folio); brelse(bh);
    if likely(err == 0) { err = nilfs_transaction_commit(sb); } else { nilfs_transaction_abort(sb); }
    err
}

unsafe fn nilfs_mdt_submit_block(inode: *mut inode, blkoff: c_ulong, opf: blk_opf_t, out_bh: *mut *mut buffer_head) -> c_int {
    let mut bh: *mut buffer_head;
    let mut blknum: u64 = 0;
    let mut ret = -ENOMEM;
    bh = nilfs_grab_buffer(inode, (*inode).i_mapping, blkoff, 0);
    if unlikely(bh.is_null()) { return ret; }
    ret = -EEXIST;
    if buffer_uptodate(bh) { get_bh(bh); *out_bh = bh; folio_unlock((*bh).b_folio); folio_put((*bh).b_folio); brelse(bh); return ret; }
    if opf & REQ_RAHEAD != 0 { if !trylock_buffer(bh) { ret = -EBUSY; goto submit_failed; } } else { lock_buffer(bh); }
    if buffer_uptodate(bh) { unlock_buffer(bh); ret = -EEXIST; goto submit_out; }
    ret = nilfs_bmap_lookup((*NILFS_I(inode)).i_bmap, blkoff, &mut blknum);
    if unlikely(ret != 0) { unlock_buffer(bh); goto submit_failed; }
    map_bh(bh, (*inode).i_sb, blknum as sector_t);
    bh_submit(bh, opf, bh_end_read); ret = 0;
    trace_nilfs2_mdt_submit_block(inode, (*inode).i_ino, blkoff, opf & REQ_OP_MASK);
submit_out:
    get_bh(bh); *out_bh = bh;
submit_failed:
    folio_unlock((*bh).b_folio); folio_put((*bh).b_folio); brelse(bh); ret
}

unsafe fn nilfs_mdt_read_block(inode: *mut inode, block: c_ulong, readahead: c_int, out_bh: *mut *mut buffer_head) -> c_int {
    let mut first_bh: *mut buffer_head = core::ptr::null_mut();
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    let mut err = nilfs_mdt_submit_block(inode, block, REQ_OP_READ, &mut first_bh);
    if err == -EEXIST { *out_bh = first_bh; return 0; }
    if unlikely(err != 0) { return err; }
    if readahead != 0 { for i in 0..NILFS_MDT_MAX_RA_BLOCKS { err = nilfs_mdt_submit_block(inode, block + i as c_ulong + 1, REQ_OP_READ | REQ_RAHEAD, &mut bh); if likely(err == 0 || err == -EEXIST) { brelse(bh); } else if err != -EBUSY { break; } if !buffer_locked(first_bh) { break; } } }
    wait_on_buffer(first_bh);
    if !buffer_uptodate(first_bh) { nilfs_err((*inode).i_sb, "I/O error reading meta-data file (ino=%llu, block-offset=%lu)", (*inode).i_ino, block); brelse(first_bh); return -EIO; }
    *out_bh = first_bh; 0
}

pub unsafe fn nilfs_mdt_get_block(inode: *mut inode, blkoff: c_ulong, create: c_int, init_block: Option<unsafe extern "C" fn(*mut inode, *mut buffer_head, *mut c_void)>, out_bh: *mut *mut buffer_head) -> c_int {
    loop { let ret = nilfs_mdt_read_block(inode, blkoff, if create != 0 { 0 } else { 1 }, out_bh); if create == 0 || ret != -ENOENT { return ret; } let ret = nilfs_mdt_create_block(inode, blkoff, out_bh, init_block); if ret != -EEXIST { return ret; } }
}

pub unsafe fn nilfs_mdt_find_block(inode: *mut inode, start: c_ulong, end: c_ulong, blkoff: *mut c_ulong, out_bh: *mut *mut buffer_head) -> c_int {
    if unlikely(start > end) { return -ENOENT; }
    let mut ret = nilfs_mdt_read_block(inode, start, 1, out_bh);
    if ret == 0 { *blkoff = start; return ret; }
    if unlikely(ret != -ENOENT || start == ULONG_MAX) { return ret; }
    let mut next: u64 = 0;
    ret = nilfs_bmap_seek_key((*NILFS_I(inode)).i_bmap, start + 1, &mut next);
    if ret == 0 { if next <= end as u64 { ret = nilfs_mdt_read_block(inode, next as c_ulong, 1, out_bh); if ret == 0 { *blkoff = next as c_ulong; } } else { ret = -ENOENT; } }
    ret
}

pub unsafe fn nilfs_mdt_delete_block(inode: *mut inode, block: c_ulong) -> c_int {
    let err = nilfs_bmap_delete((*NILFS_I(inode)).i_bmap, block);
    if err == 0 || err == -ENOENT { nilfs_mdt_mark_dirty(inode); nilfs_mdt_forget_block(inode, block); } err
}

pub unsafe fn nilfs_mdt_forget_block(inode: *mut inode, block: c_ulong) -> c_int {
    let index = block >> (PAGE_SHIFT - (*inode).i_blkbits);
    let folio = filemap_lock_folio((*inode).i_mapping, index);
    if IS_ERR(folio) { return -ENOENT; }
    folio_wait_writeback(folio);
    let mut bh = folio_buffers(folio);
    if !bh.is_null() { let first_block = index << (PAGE_SHIFT - (*inode).i_blkbits); bh = get_nth_bh(bh, block - first_block); nilfs_forget_buffer(bh); }
    let still_dirty = folio_test_dirty(folio); folio_unlock(folio); folio_put(folio);
    if still_dirty || invalidate_inode_pages2_range((*inode).i_mapping, index, index) != 0 { -EBUSY } else { 0 }
}

pub unsafe fn nilfs_mdt_fetch_dirty(inode: *mut inode) -> c_int { let ii = NILFS_I(inode); if nilfs_bmap_test_and_clear_dirty((*ii).i_bmap) { set_bit(NILFS_I_DIRTY, &mut (*ii).i_state); return 1; } test_bit(NILFS_I_DIRTY, &(*ii).i_state) }

unsafe fn nilfs_mdt_write_folio(folio: *mut folio, wbc: *mut writeback_control) -> c_int {
    let inode = (*(*folio).mapping).host;
    if !inode.is_null() && sb_rdonly((*inode).i_sb) { nilfs_clear_folio_dirty(folio); folio_unlock(folio); return -EROFS; }
    folio_redirty_for_writepage(wbc, folio); folio_unlock(folio); if inode.is_null() { return 0; }
    if (*wbc).sync_mode == WB_SYNC_ALL { nilfs_construct_segment((*inode).i_sb) } else { 0 }
}

unsafe fn nilfs_mdt_writeback(mapping: *mut address_space, wbc: *mut writeback_control) -> c_int {
    let mut folio: *mut folio = core::ptr::null_mut(); let mut error = 0;
    while { folio = writeback_iter(mapping, wbc, folio, &mut error); !folio.is_null() } { error = nilfs_mdt_write_folio(folio, wbc); } error
}

static def_mdt_aops: address_space_operations = address_space_operations { dirty_folio: Some(block_dirty_folio), invalidate_folio: Some(block_invalidate_folio), writepages: Some(nilfs_mdt_writeback), migrate_folio: Some(buffer_migrate_folio_norefs) };
static def_mdt_iops: inode_operations = unsafe { core::mem::zeroed() };
static def_mdt_fops: file_operations = unsafe { core::mem::zeroed() };

pub unsafe fn nilfs_mdt_init(inode: *mut inode, gfp_mask: gfp_t, objsz: usize) -> c_int {
    let mi = kzalloc(core::cmp::max(core::mem::size_of::<nilfs_mdt_info>(), objsz), GFP_NOFS); if mi.is_null() { return -ENOMEM; }
    init_rwsem(&mut (*mi).mi_sem); (*inode).i_private = mi as *mut c_void; (*inode).i_mode = S_IFREG; mapping_set_gfp_mask((*inode).i_mapping, gfp_mask); (*inode).i_op = &def_mdt_iops; (*inode).i_fop = &def_mdt_fops; (*inode).i_mapping.a_ops = &def_mdt_aops; 0
}

pub unsafe fn nilfs_mdt_clear(inode: *mut inode) { let mdi = NILFS_MDT(inode); if !(*mdi).mi_palloc_cache.is_null() { nilfs_palloc_destroy_cache(inode); } let shadow = (*mdi).mi_shadow; if !shadow.is_null() { let s_inode = (*shadow).inode; (*shadow).inode = core::ptr::null_mut(); iput(s_inode); (*mdi).mi_shadow = core::ptr::null_mut(); } }
pub unsafe fn nilfs_mdt_destroy(inode: *mut inode) { let mdi = NILFS_MDT(inode); kfree((*mdi).mi_bgl as *mut c_void); kfree(mdi as *mut c_void); }
pub unsafe fn nilfs_mdt_set_entry_size(inode: *mut inode, entry_size: c_uint, header_size: c_uint) { let mi = NILFS_MDT(inode); (*mi).mi_entry_size = entry_size; (*mi).mi_entries_per_block = i_blocksize(inode) / entry_size; (*mi).mi_first_entry_offset = DIV_ROUND_UP(header_size, entry_size); }

pub unsafe fn nilfs_mdt_setup_shadow_map(inode: *mut inode, shadow: *mut nilfs_shadow_map) -> c_int { INIT_LIST_HEAD(&mut (*shadow).frozen_buffers); let s_inode = nilfs_iget_for_shadow(inode); if IS_ERR(s_inode) { return PTR_ERR(s_inode); } (*shadow).inode = s_inode; (*NILFS_MDT(inode)).mi_shadow = shadow; 0 }
pub unsafe fn nilfs_mdt_save_to_shadow_map(inode: *mut inode) -> c_int { let mi = NILFS_MDT(inode); let ii = NILFS_I(inode); let shadow = (*mi).mi_shadow; let s_inode = (*shadow).inode; let mut ret = nilfs_copy_dirty_pages((*s_inode).i_mapping, (*inode).i_mapping); if ret != 0 { return ret; } ret = nilfs_copy_dirty_pages((*(*NILFS_I(s_inode)).i_assoc_inode).i_mapping, (*(*ii).i_assoc_inode).i_mapping); if ret == 0 { nilfs_bmap_save((*ii).i_bmap, &mut (*shadow).bmap_store); } ret }

pub unsafe fn nilfs_mdt_freeze_buffer(inode: *mut inode, bh: *mut buffer_head) -> c_int { let shadow = (*NILFS_MDT(inode)).mi_shadow; let folio = filemap_grab_folio((*(*shadow).inode).i_mapping, (*(*bh).b_folio).index); if IS_ERR(folio) { return PTR_ERR(folio); } let mut frozen = folio_buffers(folio); if frozen.is_null() { frozen = create_empty_buffers(folio, 1 << (*inode).i_blkbits, 0); } frozen = get_nth_bh(frozen, offset_in_folio(folio, (*bh).b_data) >> (*inode).i_blkbits); if !buffer_uptodate(frozen) { nilfs_copy_buffer(frozen, bh); } if list_empty(&(*frozen).b_assoc_buffers) { list_add_tail(&mut (*frozen).b_assoc_buffers, &mut (*shadow).frozen_buffers); set_buffer_nilfs_redirected(bh); } else { brelse(frozen); } folio_unlock(folio); folio_put(folio); 0 }
pub unsafe fn nilfs_mdt_get_frozen_buffer(inode: *mut inode, bh: *mut buffer_head) -> *mut buffer_head { let shadow = (*NILFS_MDT(inode)).mi_shadow; let folio = filemap_lock_folio((*(*shadow).inode).i_mapping, (*(*bh).b_folio).index); if IS_ERR(folio) { return core::ptr::null_mut(); } let mut frozen = folio_buffers(folio); if !frozen.is_null() { frozen = get_nth_bh(frozen, offset_in_folio(folio, (*bh).b_data) >> (*inode).i_blkbits); } folio_unlock(folio); folio_put(folio); frozen }
unsafe fn nilfs_release_frozen_buffers(shadow: *mut nilfs_shadow_map) { let head = &mut (*shadow).frozen_buffers; while !list_empty(head) { let bh = list_first_entry(head, buffer_head, b_assoc_buffers); list_del_init(&mut (*bh).b_assoc_buffers); brelse(bh); } }
pub unsafe fn nilfs_mdt_restore_from_shadow_map(inode: *mut inode) { let mi = NILFS_MDT(inode); let ii = NILFS_I(inode); let shadow = (*mi).mi_shadow; down_write(&mut (*mi).mi_sem); if !(*mi).mi_palloc_cache.is_null() { nilfs_palloc_clear_cache(inode); } nilfs_clear_dirty_pages((*inode).i_mapping); nilfs_copy_back_pages((*inode).i_mapping, (*(*shadow).inode).i_mapping); nilfs_clear_dirty_pages((*(*ii).i_assoc_inode).i_mapping); nilfs_copy_back_pages((*(*ii).i_assoc_inode).i_mapping, (*(*NILFS_I((*shadow).inode)).i_assoc_inode).i_mapping); nilfs_bmap_restore((*ii).i_bmap, &(*shadow).bmap_store); up_write(&mut (*mi).mi_sem); }
pub unsafe fn nilfs_mdt_clear_shadow_map(inode: *mut inode) { let mi = NILFS_MDT(inode); let shadow = (*mi).mi_shadow; let btnc = (*NILFS_I((*shadow).inode)).i_assoc_inode; down_write(&mut (*mi).mi_sem); nilfs_release_frozen_buffers(shadow); truncate_inode_pages((*(*shadow).inode).i_mapping, 0); truncate_inode_pages((*btnc).i_mapping, 0); up_write(&mut (*mi).mi_sem); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
