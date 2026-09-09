// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023 Western Digital Corporation or its affiliates. */

// C dependencies are supplied by the surrounding translation unit.

unsafe fn btrfs_partially_delete_raid_extent(
    trans: *mut btrfs_trans_handle, path: *mut btrfs_path,
    oldkey: *const btrfs_key, newlen: u64, frontpad: u64,
) -> i32 {
    let stripe_root = (*(*trans).fs_info).stripe_root;
    let leaf = (*path).nodes[0];
    let slot = (*path).slots[0];
    let item_size = btrfs_item_size(leaf, slot);
    let newitem = kzalloc(item_size, GFP_NOFS);
    if newitem.is_null() { return -ENOMEM; }
    let newkey = btrfs_key { objectid: (*oldkey).objectid + frontpad,
        type_: BTRFS_RAID_STRIPE_KEY, offset: newlen };
    ASSERT(newlen > 0);
    ASSERT((*oldkey).type_ == BTRFS_RAID_STRIPE_KEY);
    let extent = btrfs_item_ptr::<btrfs_stripe_extent>(leaf, slot);
    for i in 0..btrfs_num_raid_stripes(item_size) {
        let stride = &(*extent).strides[i];
        let devid = btrfs_raid_stride_devid(leaf, stride);
        btrfs_set_stack_raid_stride_devid(&mut (*newitem).strides[i], devid);
        let phys = btrfs_raid_stride_physical(leaf, stride) + frontpad;
        btrfs_set_stack_raid_stride_physical(&mut (*newitem).strides[i], phys);
    }
    let ret = btrfs_del_item(trans, stripe_root, path);
    if ret != 0 { return ret; }
    btrfs_release_path(path);
    btrfs_insert_item(trans, stripe_root, &newkey, newitem, item_size)
}

pub unsafe fn btrfs_delete_raid_extent(trans: *mut btrfs_trans_handle, mut start: u64, mut length: u64) -> i32 {
    let fs_info = (*trans).fs_info;
    let stripe_root = (*fs_info).stripe_root;
    if !btrfs_fs_incompat(fs_info, RAID_STRIPE_TREE) || stripe_root.is_null() { return 0; }
    if !btrfs_is_testing(fs_info) {
        let map = btrfs_find_chunk_map(fs_info, start, length);
        if map.is_null() { return -EINVAL; }
        let use_rst = btrfs_need_stripe_tree_update(fs_info, (*map).type_);
        btrfs_free_chunk_map(map);
        if !use_rst { return 0; }
    }
    let path = btrfs_alloc_path();
    if path.is_null() { return -ENOMEM; }
    let mut ret = 0;
    loop {
        let mut key = btrfs_key { objectid: start, type_: BTRFS_RAID_STRIPE_KEY, offset: u64::MAX };
        ret = btrfs_search_slot(trans, stripe_root, &key, path, -1, 1);
        if ret < 0 { break; }
        if (*path).slots[0] == 0 { ret = 0; break; }
        (*path).slots[0] -= 1;
        let mut leaf = (*path).nodes[0];
        let mut slot = (*path).slots[0];
        btrfs_item_key_to_cpu(leaf, &mut key, slot);
        let mut found_start = key.objectid;
        let mut found_end = found_start + key.offset;
        if found_start > start {
            if slot == 0 { ret = btrfs_previous_item(stripe_root, path, 0, BTRFS_RAID_STRIPE_KEY); if ret != 0 { if ret > 0 { ret = -ENOENT; } break; } }
            else { (*path).slots[0] -= 1; }
            leaf = (*path).nodes[0]; slot = (*path).slots[0];
            btrfs_item_key_to_cpu(leaf, &mut key, slot);
            found_start = key.objectid; found_end = found_start + key.offset;
            if found_start > start || found_end <= start { ret = -ENOENT; break; }
        }
        if key.type_ != BTRFS_RAID_STRIPE_KEY || found_end <= start { break; }
        trace_btrfs_raid_extent_delete(fs_info, start, start + length, found_start, found_end);
        let end = start + length;
        if found_start < start && found_end > end {
            let diff_start = start - found_start; let diff_end = found_end - end;
            let newkey = btrfs_key { objectid: end, type_: BTRFS_RAID_STRIPE_KEY, offset: diff_end };
            ret = btrfs_duplicate_item(trans, stripe_root, path, &newkey);
            if ret == -EAGAIN { btrfs_release_path(path); continue; }
            if ret != 0 { break; }
            leaf = (*path).nodes[0]; let item_size = btrfs_item_size(leaf, (*path).slots[0]);
            let extent = btrfs_item_ptr::<btrfs_stripe_extent>(leaf, (*path).slots[0]);
            for i in 0..btrfs_num_raid_stripes(item_size) { let stride = &mut (*extent).strides[i]; let phys = btrfs_raid_stride_physical(leaf, stride) + diff_start + length; btrfs_set_raid_stride_physical(leaf, stride, phys); }
            (*path).slots[0] -= 1; btrfs_item_key_to_cpu(leaf, &mut key, (*path).slots[0]);
            ret = btrfs_partially_delete_raid_extent(trans, path, &key, diff_start, 0); break;
        }
        if found_start < start {
            let diff_start = start - found_start;
            ret = btrfs_partially_delete_raid_extent(trans, path, &key, diff_start, 0);
            if ret != 0 { break; }
            start += key.offset - diff_start; length -= key.offset - diff_start;
            if length == 0 { break; } btrfs_release_path(path); continue;
        }
        if found_end > end {
            let diff_end = found_end - end;
            ret = btrfs_partially_delete_raid_extent(trans, path, &key, key.offset - length, length);
            ASSERT(key.offset - diff_end == length); break;
        }
        ret = btrfs_del_item(trans, stripe_root, path); if ret != 0 { break; }
        start += key.offset; length -= key.offset; if length == 0 { break; }
        btrfs_release_path(path);
    }
    ret
}

unsafe fn update_raid_extent_item(trans: *mut btrfs_trans_handle, key: *mut btrfs_key, extent: *mut btrfs_stripe_extent, item_size: usize) -> i32 {
    let path = btrfs_alloc_path(); if path.is_null() { return -ENOMEM; }
    let ret = btrfs_search_slot(trans, (*trans).fs_info.stripe_root, key, path, 0, 1);
    if ret != 0 { return if ret == 1 { ret } else { -EINVAL }; }
    write_extent_buffer((*path).nodes[0], extent, btrfs_item_ptr_offset((*path).nodes[0], (*path).slots[0]), item_size); ret
}

pub unsafe fn btrfs_insert_one_raid_extent(trans: *mut btrfs_trans_handle, bioc: *mut btrfs_io_context) -> i32 {
    let fs_info = (*trans).fs_info; let root = (*fs_info).stripe_root;
    let n = btrfs_bg_type_to_factor((*bioc).map_type) as usize; let size = struct_size::<btrfs_stripe_extent>(n);
    let extent = kzalloc(size, GFP_NOFS); if extent.is_null() { btrfs_abort_transaction(trans, -ENOMEM); btrfs_end_transaction(trans); return -ENOMEM; }
    trace_btrfs_insert_one_raid_extent(fs_info, (*bioc).logical, (*bioc).size, n as i32);
    for i in 0..n { let s = &(*bioc).stripes[i]; btrfs_set_stack_raid_stride_devid(&mut (*extent).strides[i], (*s.dev).devid); btrfs_set_stack_raid_stride_physical(&mut (*extent).strides[i], s.physical); }
    let key = btrfs_key { objectid: (*bioc).logical, type_: BTRFS_RAID_STRIPE_KEY, offset: (*bioc).size };
    let mut ret = btrfs_insert_item(trans, root, &key, extent, size);
    if ret == -EEXIST { ret = update_raid_extent_item(trans, &key as *const _ as *mut _, extent, size); if ret != 0 { btrfs_abort_transaction(trans, ret); } }
    else if ret != 0 { btrfs_abort_transaction(trans, ret); } ret
}

pub unsafe fn btrfs_insert_raid_extent(trans: *mut btrfs_trans_handle, ordered: *mut btrfs_ordered_extent) -> i32 {
    if !btrfs_fs_incompat((*trans).fs_info, RAID_STRIPE_TREE) { return 0; }
    let mut bioc = (*ordered).bioc_list; list_for_each_entry!(bioc, &mut (*ordered).bioc_list, rst_ordered_entry) { let ret = btrfs_insert_one_raid_extent(trans, bioc); if ret != 0 { return ret; } }
    while !list_empty(&(*ordered).bioc_list) { bioc = list_first_entry!(&(*ordered).bioc_list, btrfs_io_context, rst_ordered_entry); list_del(&mut (*bioc).rst_ordered_entry); btrfs_put_bioc(bioc); } 0
}

pub unsafe fn btrfs_get_raid_extent_offset(fs_info: *mut btrfs_fs_info, logical: u64, length: *mut u64, map_type: u64, stripe_index: u32, stripe: *mut btrfs_io_stripe) -> i32 {
    let root = (*fs_info).stripe_root; if root.is_null() { btrfs_err_rl(fs_info, "missing raid stripe tree root for logical %llu", logical); return -EUCLEAN; }
    let path = btrfs_alloc_path(); if path.is_null() { return -ENOMEM; }
    let key = btrfs_key { objectid: logical, type_: BTRFS_RAID_STRIPE_KEY, offset: 0 };
    if (*stripe).rst_search_commit_root { (*path).skip_locking = true; (*path).search_commit_root = true; }
    let mut ret = btrfs_search_slot(core::ptr::null_mut(), root, &key, path, 0, 0); if ret < 0 { return ret; } if ret != 0 && (*path).slots[0] != 0 { (*path).slots[0] -= 1; }
    let end = logical + *length;
    loop { let leaf = (*path).nodes[0]; let slot = (*path).slots[0]; let mut found = btrfs_key::default(); btrfs_item_key_to_cpu(leaf, &mut found, slot); let found_end = found.objectid + found.offset; if found.objectid > end { ret = -ENODATA; break; } if in_range(logical, found.objectid, found.offset) { let off = logical - found.objectid; if end > found_end { *length -= end - found_end; } let n = btrfs_num_raid_stripes(btrfs_item_size(leaf, slot)); let extent = btrfs_item_ptr::<btrfs_stripe_extent>(leaf, slot); for i in 0..n { let s = &(*extent).strides[i]; let devid = btrfs_raid_stride_devid(leaf, s); if devid == (*(*stripe).dev).devid && (!(map_type & BTRFS_BLOCK_GROUP_DUP != 0) || stripe_index == i as u32) { (*stripe).physical = btrfs_raid_stride_physical(leaf, s) + off; trace_btrfs_get_raid_extent_offset(fs_info, logical, *length, (*stripe).physical, devid); return 0; } } ret = -ENODATA; break; } ret = btrfs_next_item(root, path); if ret != 0 { break; } }
    if ret > 0 { ret = -ENODATA; } if ret != 0 && ret != -EIO && !(*stripe).rst_search_commit_root { btrfs_debug(fs_info, "cannot find raid-stripe", logical, logical + *length, (*(*stripe).dev).devid, btrfs_bg_type_to_raid_name(map_type)); } ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
