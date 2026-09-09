// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of file-item.c. External kernel types and
 * functions are supplied by the surrounding translated tree. */

const __MAX_CSUM_ITEMS = |r: &btrfs_root, size: usize| -> usize {
    ((BTRFS_LEAF_DATA_SIZE(r) - core::mem::size_of::<btrfs_item>() * 2) / size) - 1
};
const MAX_CSUM_ITEMS = |r: &btrfs_fs_info, size: usize| -> u32 {
    core::cmp::min(__MAX_CSUM_ITEMS(r.root, size) as u32, PAGE_SIZE as u32)
};

pub unsafe fn btrfs_inode_safe_disk_i_size_write(inode: *mut btrfs_inode, new_i_size: u64) {
    spin_lock(&mut (*inode).lock);
    let mut i_size = if new_i_size != 0 { new_i_size } else { i_size_read(&(*inode).vfs_inode) };
    if (*inode).file_extent_tree.is_null() { (*inode).disk_i_size = i_size; spin_unlock(&mut (*inode).lock); return; }
    let mut start = 0; let mut end = 0;
    let found = btrfs_find_contiguous_extent_bit((*inode).file_extent_tree, 0, &mut start, &mut end, EXTENT_DIRTY);
    i_size = if found && start == 0 { core::cmp::min(i_size, end.wrapping_add(1)) } else { 0 };
    (*inode).disk_i_size = i_size;
    spin_unlock(&mut (*inode).lock);
}

pub unsafe fn btrfs_inode_set_file_extent_range(inode: *mut btrfs_inode, start: u64, len: u64) -> i32 {
    if (*inode).file_extent_tree.is_null() || len == 0 { return 0; }
    ASSERT(IS_ALIGNED(start.wrapping_add(len), (*(*inode).root).fs_info.sectorsize));
    btrfs_set_extent_bit((*inode).file_extent_tree, start, start + len - 1, EXTENT_DIRTY, core::ptr::null_mut())
}

pub unsafe fn btrfs_inode_clear_file_extent_range(inode: *mut btrfs_inode, start: u64, len: u64) -> i32 {
    if (*inode).file_extent_tree.is_null() || len == 0 { return 0; }
    ASSERT(IS_ALIGNED(start.wrapping_add(len), (*(*inode).root).fs_info.sectorsize) || len == u64::MAX);
    btrfs_clear_extent_bit((*inode).file_extent_tree, start, start + len - 1, EXTENT_DIRTY, core::ptr::null_mut())
}

unsafe fn bytes_to_csum_size(f: *const btrfs_fs_info, bytes: u32) -> usize {
    ASSERT(IS_ALIGNED(bytes, (*f).sectorsize));
    ((bytes >> (*f).sectorsize_bits) * (*f).csum_size) as usize
}
unsafe fn csum_size_to_bytes(f: *const btrfs_fs_info, size: u32) -> u32 {
    ASSERT(IS_ALIGNED(size, (*f).csum_size));
    (size / (*f).csum_size) << (*f).sectorsize_bits
}
unsafe fn max_ordered_sum_bytes(f: *const btrfs_fs_info) -> u32 {
    csum_size_to_bytes(f, round_down(PAGE_SIZE - core::mem::size_of::<btrfs_ordered_sum>(), (*f).csum_size))
}
unsafe fn btrfs_ordered_sum_size(f: *const btrfs_fs_info, bytes: usize) -> usize {
    core::mem::size_of::<btrfs_ordered_sum>() + bytes_to_csum_size(f, bytes as u32)
}

pub unsafe fn btrfs_insert_hole_extent(trans: *mut btrfs_trans_handle, root: *mut btrfs_root, objectid: u64, pos: u64, num_bytes: u64) -> i32 {
    let path = btrfs_alloc_path(); if path.is_null() { return -ENOMEM; }
    let mut key = btrfs_key { objectid, type_: BTRFS_EXTENT_DATA_KEY, offset: pos };
    let ret = btrfs_insert_empty_item(trans, root, path, &mut key, core::mem::size_of::<btrfs_file_extent_item>() as u32);
    if ret < 0 { return ret; }
    let leaf = (*path).nodes[0]; let item = btrfs_item_ptr(leaf, (*path).slots[0]);
    btrfs_set_file_extent_disk_bytenr(leaf, item, 0); btrfs_set_file_extent_disk_num_bytes(leaf, item, 0);
    btrfs_set_file_extent_offset(leaf, item, 0); btrfs_set_file_extent_num_bytes(leaf, item, num_bytes);
    btrfs_set_file_extent_ram_bytes(leaf, item, num_bytes); btrfs_set_file_extent_generation(leaf, item, (*trans).transid);
    btrfs_set_file_extent_type(leaf, item, BTRFS_FILE_EXTENT_REG); btrfs_set_file_extent_compression(leaf, item, 0);
    btrfs_set_file_extent_encryption(leaf, item, 0); btrfs_set_file_extent_other_encoding(leaf, item, 0); ret
}

unsafe fn btrfs_lookup_csum(trans: *mut btrfs_trans_handle, root: *mut btrfs_root, path: *mut btrfs_path, bytenr: u64, cow: i32) -> *mut btrfs_csum_item {
    let f = (*root).fs_info; let mut key = btrfs_key { objectid: BTRFS_EXTENT_CSUM_OBJECTID, type_: BTRFS_EXTENT_CSUM_KEY, offset: bytenr }; let mut found = btrfs_key::default();
    let mut ret = btrfs_search_slot(trans, root, &mut key, path, 0, cow); if ret < 0 { return ERR_PTR(ret); }
    let leaf = (*path).nodes[0]; let mut off = 0u64;
    if ret > 0 { ret = 1; if (*path).slots[0] == 0 { return ERR_PTR(-ENOENT); } (*path).slots[0] -= 1; btrfs_item_key_to_cpu(leaf, &mut found, (*path).slots[0]); if found.type_ != BTRFS_EXTENT_CSUM_KEY { return ERR_PTR(-ENOENT); }
        off = (bytenr - found.offset) >> (*f).sectorsize_bits; let n = btrfs_item_size(leaf, (*path).slots[0]) / (*f).csum_size; if off == n as u64 { return ERR_PTR(-EFBIG); } if off > n as u64 { return ERR_PTR(-ENOENT); }
    }
    (btrfs_item_ptr(leaf, (*path).slots[0]) as *mut u8).add((off * (*f).csum_size as u64) as usize) as *mut btrfs_csum_item
}

pub unsafe fn btrfs_lookup_file_extent(trans: *mut btrfs_trans_handle, root: *mut btrfs_root, path: *mut btrfs_path, objectid: u64, offset: u64, mod_: i32) -> i32 {
    let mut key = btrfs_key { objectid, type_: BTRFS_EXTENT_DATA_KEY, offset };
    btrfs_search_slot(trans, root, &mut key, path, if mod_ < 0 { -1 } else { 0 }, if mod_ != 0 { 1 } else { 0 })
}

/* The remaining routines retain the C control flow and use the translated
 * kernel accessors supplied by dependent files. */
pub unsafe fn btrfs_file_extent_end(path: *const btrfs_path) -> u64 {
    let leaf = (*path).nodes[0]; let slot = (*path).slots[0]; let mut key = btrfs_key::default();
    btrfs_item_key_to_cpu(leaf, &mut key, slot); let fi = btrfs_item_ptr(leaf, slot);
    ASSERT(key.type_ == BTRFS_EXTENT_DATA_KEY);
    if btrfs_file_extent_type(leaf, fi) == BTRFS_FILE_EXTENT_INLINE { (*leaf).fs_info.sectorsize as u64 } else { key.offset + btrfs_file_extent_num_bytes(leaf, fi) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
