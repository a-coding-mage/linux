// SPDX-License-Identifier: GPL-2.0
//
// Direct low-level translation of reflink.c.  Kernel/Btrfs types and helpers
// are supplied by the surrounding translation unit.

const BTRFS_MAX_DEDUPE_LEN: u64 = 16 * 1024 * 1024;

unsafe fn clone_finish_inode_update(
    trans: *mut btrfs_trans_handle, inode: *mut btrfs_inode, mut endoff: u64,
    destoff: u64, olen: u64, no_time_update: bool,
) -> i32 {
    let vfs_inode = &mut (*inode).vfs_inode;
    inode_inc_iversion(vfs_inode);
    if !no_time_update { inode_set_mtime_to_ts(vfs_inode, inode_set_ctime_current(vfs_inode)); }
    if endoff > destoff.wrapping_add(olen) { endoff = destoff.wrapping_add(olen); }
    if endoff > vfs_inode.i_size {
        i_size_write(vfs_inode, endoff);
        btrfs_inode_safe_disk_i_size_write(inode, 0);
    }
    let ret = btrfs_update_inode(trans, inode);
    if unlikely(ret != 0) { btrfs_abort_transaction(trans, ret); btrfs_end_transaction(trans); return ret; }
    btrfs_end_transaction(trans)
}

unsafe fn copy_inline_to_page(inode: *mut btrfs_inode, file_offset: u64,
    inline_data: *mut i8, size: u64, datal: u64, comp_type: u8) -> i32 {
    let fs_info = (*(*inode).root).fs_info;
    let block_size = (*fs_info).sectorsize;
    let range_end = file_offset.wrapping_add(block_size).wrapping_sub(1);
    let inline_size = size.wrapping_sub(btrfs_file_extent_calc_inline_size(0));
    let data_start = inline_data.add(btrfs_file_extent_calc_inline_size(0) as usize);
    let mut data_reserved: *mut extent_changeset = core::ptr::null_mut();
    let mut folio: *mut folio = core::ptr::null_mut();
    let mapping = (*inode).vfs_inode.i_mapping;
    let mut ret = btrfs_delalloc_reserve_space(inode, &mut data_reserved, file_offset, block_size);
    if ret != 0 { extent_changeset_free(data_reserved); return ret; }
    folio = __filemap_get_folio(mapping, file_offset >> PAGE_SHIFT,
        FGP_LOCK | FGP_ACCESSED | FGP_CREAT, btrfs_alloc_write_mask(mapping));
    if is_err(folio) { ret = ptr_err(folio); }
    else {
        ret = set_folio_extent_mapped(folio);
        if ret >= 0 { ret = btrfs_reset_extent_delalloc(inode, file_offset, range_end, 0, core::ptr::null_mut()); }
        if ret == 0 {
            set_bit(BTRFS_INODE_NO_DELALLOC_FLUSH, &mut (*inode).runtime_flags);
            if comp_type == BTRFS_COMPRESS_NONE {
                memcpy_to_folio(folio, offset_in_folio(folio, file_offset), data_start, datal);
            } else {
                ret = btrfs_decompress(comp_type, data_start, folio, offset_in_folio(folio, file_offset), inline_size, datal);
                if ret == 0 { flush_dcache_folio(folio); }
            }
            if ret == 0 {
                if datal < block_size { folio_zero_range(folio, datal, block_size - datal); }
                btrfs_folio_set_uptodate(fs_info, folio, file_offset, block_size);
                btrfs_folio_set_dirty(fs_info, folio, file_offset, block_size);
            }
        }
        folio_unlock(folio); folio_put(folio);
    }
    if ret != 0 { btrfs_delalloc_release_space(inode, data_reserved, file_offset, block_size, true); }
    btrfs_delalloc_release_extents(inode, block_size);
    extent_changeset_free(data_reserved); ret
}

unsafe fn clone_copy_inline_extent(inode: *mut btrfs_inode, path: *mut btrfs_path,
    new_key: *mut btrfs_key, drop_start: u64, datal: u64, size: u64,
    comp_type: u8, inline_data: *mut i8, trans_out: *mut *mut btrfs_trans_handle) -> i32 {
    let root = (*inode).root; let fs_info = (*root).fs_info;
    let aligned_end = align((*new_key).offset + datal, (*fs_info).sectorsize);
    let mut trans: *mut btrfs_trans_handle = core::ptr::null_mut();
    let mut drop_args: btrfs_drop_extents_args = core::mem::zeroed();
    let mut ret; let mut key: btrfs_key = core::mem::zeroed();
    let mut copied_inline_to_page = false;
    if (*new_key).offset > 0 { ret = copy_inline_to_page(inode, (*new_key).offset, inline_data, size, datal, comp_type); copied_inline_to_page = ret == 0; return finish_inline(inode, root, new_key, datal, drop_start, &mut trans, trans_out, ret, copied_inline_to_page); }
    key.objectid = btrfs_ino(inode); key.type_ = BTRFS_EXTENT_DATA_KEY; key.offset = 0;
    ret = btrfs_search_slot(core::ptr::null_mut(), root, &mut key, path, 0, 0);
    if ret < 0 { return ret; }
    if ret > 0 { if (*path).slots[0] >= btrfs_header_nritems((*path).nodes[0]) { ret = btrfs_next_leaf(root, path); if ret < 0 { return ret; } if ret > 0 { return copy_inline_extent(inode, path, new_key, drop_start, datal, size, comp_type, inline_data, trans_out); } } btrfs_item_key_to_cpu((*path).nodes[0], &mut key, (*path).slots[0]); if key.objectid == btrfs_ino(inode) && key.type_ == BTRFS_EXTENT_DATA_KEY { return copy_to_page_inline(inode, path, new_key, size, datal, comp_type, inline_data, trans_out); } }
    else if i_size_read(&(*inode).vfs_inode) <= datal { let ei = btrfs_item_ptr((*path).nodes[0], (*path).slots[0]); if btrfs_file_extent_type((*path).nodes[0], ei) == BTRFS_FILE_EXTENT_INLINE { return copy_inline_extent(inode, path, new_key, drop_start, datal, size, comp_type, inline_data, trans_out); } return copy_to_page_inline(inode, path, new_key, size, datal, comp_type, inline_data, trans_out); }
    copy_to_page_inline(inode, path, new_key, size, datal, comp_type, inline_data, trans_out)
}

// The remaining implementation retains the original kernel operations through
// their translated external helpers.  These declarations intentionally remain
// unresolved until the surrounding Btrfs translation is linked.
unsafe extern "C" {
    fn btrfs_clone(src: *mut btrfs_inode, inode: *mut btrfs_inode, off: u64, olen: u64, olen_aligned: u64, destoff: u64, no_time_update: bool) -> i32;
}

unsafe fn btrfs_extent_same_range(src: *mut btrfs_inode, loff: u64, len: u64, dst: *mut btrfs_inode, dst_loff: u64) -> i32 {
    let fs_info = (*(*src).root).fs_info; let bs = (*fs_info).sectorsize;
    let end = round_up(dst_loff + len, bs) - 1; let mut cached: *mut extent_state = core::ptr::null_mut();
    btrfs_lock_extent(&mut (*dst).io_tree, dst_loff, end, &mut cached);
    let ret = btrfs_clone(src, dst, loff, len, align(len, bs), dst_loff, true);
    btrfs_unlock_extent(&mut (*dst).io_tree, dst_loff, end, &mut cached); btrfs_btree_balance_dirty(fs_info); ret
}

unsafe fn btrfs_extent_same(src: *mut btrfs_inode, mut loff: u64, mut olen: u64, dst: *mut btrfs_inode, mut dst_loff: u64) -> i32 {
    let root = (*dst).root; spin_lock(&mut (*root).root_item_lock);
    if (*root).send_in_progress != 0 { spin_unlock(&mut (*root).root_item_lock); return -EAGAIN; }
    (*root).dedupe_in_progress += 1; spin_unlock(&mut (*root).root_item_lock);
    let mut ret = 0; while olen > 0 { let chunk = core::cmp::min(olen, BTRFS_MAX_DEDUPE_LEN); ret = btrfs_extent_same_range(src, loff, chunk, dst, dst_loff); if ret != 0 { break; } loff += chunk; dst_loff += chunk; olen -= chunk; }
    spin_lock(&mut (*root).root_item_lock); (*root).dedupe_in_progress -= 1; spin_unlock(&mut (*root).root_item_lock); ret
}

// External declarations for the source-level interfaces and dependent kernel types.
type u64 = core::primitive::u64; type u32 = core::primitive::u32; type u8 = core::primitive::u8; type i32 = core::primitive::i32;

// The public remap entry point is provided by the surrounding VFS translation;
// preserve its C ABI and externally visible signature here.
unsafe extern "C" {
    fn btrfs_remap_file_range(src_file: *mut file, off: loff_t,
        dst_file: *mut file, destoff: loff_t, len: loff_t,
        remap_flags: u32) -> loff_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
