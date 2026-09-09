// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2007 Oracle.  All rights reserved.
 */

// Dependencies supplied by the surrounding translated kernel sources.

unsafe fn btrfs_read_root_item(eb: *mut extent_buffer, slot: i32,
                               item: *mut btrfs_root_item) {
    let len: u32;
    let mut need_reset = false;

    len = btrfs_item_size(eb, slot);
    read_extent_buffer(eb, item as *mut _, btrfs_item_ptr_offset(eb, slot),
                       core::cmp::min(len, core::mem::size_of::<btrfs_root_item>() as u32));
    if len < core::mem::size_of::<btrfs_root_item>() as u32 {
        need_reset = true;
    }
    if !need_reset && btrfs_root_generation(item) != btrfs_root_generation_v2(item) {
        if btrfs_root_generation_v2(item) != 0 {
            btrfs_warn((*eb).fs_info,
                       "mismatching generation and generation_v2 found in root item. This root was probably mounted with an older kernel. Resetting all new fields.");
        }
        need_reset = true;
    }
    if need_reset {
        // Clear all members from generation_v2 onwards.
        memset_startat(item, 0, generation_v2);
        generate_random_guid((*item).uuid.as_mut_ptr());
    }
}

pub unsafe fn btrfs_find_root(root: *mut btrfs_root, search_key: *const btrfs_key,
                              path: *mut btrfs_path, root_item: *mut btrfs_root_item,
                              root_key: *mut btrfs_key) -> i32 {
    let mut found_key: btrfs_key = core::mem::zeroed();
    let l: *mut extent_buffer;
    let mut ret: i32;
    let slot: i32;

    ret = btrfs_search_slot(core::ptr::null_mut(), root, search_key, path, 0, 0);
    if ret < 0 { return ret; }
    if (*search_key).offset != u64::MAX {
        if ret > 0 { btrfs_release_path(path); return ret; }
    } else {
        if ret == 0 { ret = -EUCLEAN; btrfs_release_path(path); return ret; }
        if (*path).slots[0] == 0 { btrfs_release_path(path); return ret; }
        (*path).slots[0] -= 1;
        ret = 0;
    }
    l = (*path).nodes[0];
    slot = (*path).slots[0];
    btrfs_item_key_to_cpu(l, &mut found_key, slot);
    if found_key.objectid != (*search_key).objectid || found_key.type_ != BTRFS_ROOT_ITEM_KEY {
        ret = 1;
        btrfs_release_path(path);
        return ret;
    }
    if !root_item.is_null() { btrfs_read_root_item(l, slot, root_item); }
    if !root_key.is_null() { core::ptr::copy_nonoverlapping(&found_key, root_key, 1); }
    btrfs_release_path(path);
    ret
}

pub unsafe fn btrfs_set_root_node(item: *mut btrfs_root_item, node: *mut extent_buffer) {
    btrfs_set_root_bytenr(item, (*node).start);
    btrfs_set_root_level(item, btrfs_header_level(node));
    btrfs_set_root_generation(item, btrfs_header_generation(node));
}

pub unsafe fn btrfs_update_root(trans: *mut btrfs_trans_handle, root: *mut btrfs_root,
                                key: *mut btrfs_key, item: *mut btrfs_root_item) -> i32 {
    let fs_info = (*root).fs_info;
    let path = btrfs_alloc_path();
    let l: *mut extent_buffer;
    let slot: i32;
    let ptr: usize;
    let old_len: u32;
    if path.is_null() { return -ENOMEM; }
    let mut ret = btrfs_search_slot(trans, root, key, path, 0, 1);
    if ret < 0 { return ret; }
    if ret > 0 {
        btrfs_crit(fs_info, "unable to find root key in tree", btrfs_root_id(root));
        ret = -EUCLEAN;
        btrfs_abort_transaction(trans, ret);
        return ret;
    }
    l = (*path).nodes[0]; slot = (*path).slots[0];
    ptr = btrfs_item_ptr_offset(l, slot) as usize;
    old_len = btrfs_item_size(l, slot);
    if old_len < core::mem::size_of::<btrfs_root_item>() as u32 {
        btrfs_release_path(path);
        ret = btrfs_search_slot(trans, root, key, path, -1, 1);
        if ret < 0 { btrfs_abort_transaction(trans, ret); return ret; }
        ret = btrfs_del_item(trans, root, path);
        if ret < 0 { btrfs_abort_transaction(trans, ret); return ret; }
        btrfs_release_path(path);
        ret = btrfs_insert_empty_item(trans, root, path, key,
                                      core::mem::size_of::<btrfs_root_item>() as u32);
        if ret < 0 { btrfs_abort_transaction(trans, ret); return ret; }
        l = (*path).nodes[0];
        let slot2 = (*path).slots[0];
        let ptr2 = btrfs_item_ptr_offset(l, slot2) as usize;
        btrfs_set_root_generation_v2(item, btrfs_root_generation(item));
        write_extent_buffer(l, item as *const _, ptr2, core::mem::size_of::<btrfs_root_item>());
        return ret;
    }
    btrfs_set_root_generation_v2(item, btrfs_root_generation(item));
    write_extent_buffer(l, item as *const _, ptr, core::mem::size_of::<btrfs_root_item>());
    ret
}

pub unsafe fn btrfs_insert_root(trans: *mut btrfs_trans_handle, root: *mut btrfs_root,
                                key: *const btrfs_key, item: *mut btrfs_root_item) -> i32 {
    btrfs_set_root_generation_v2(item, btrfs_root_generation(item));
    btrfs_insert_item(trans, root, key, item as *const _, core::mem::size_of::<btrfs_root_item>())
}

pub unsafe fn btrfs_find_orphan_roots(fs_info: *mut btrfs_fs_info) -> i32 {
    let tree_root = (*fs_info).tree_root;
    let path = btrfs_alloc_path();
    let mut key: btrfs_key = btrfs_key { objectid: BTRFS_ORPHAN_OBJECTID, type_: BTRFS_ORPHAN_ITEM_KEY, offset: 0 };
    if path.is_null() { return -ENOMEM; }
    loop {
        let ret = btrfs_search_slot(core::ptr::null_mut(), tree_root, &key, path, 0, 0);
        if ret < 0 { return ret; }
        let mut leaf = (*path).nodes[0];
        if (*path).slots[0] >= btrfs_header_nritems(leaf) {
            let next = btrfs_next_leaf(tree_root, path);
            if next < 0 { return next; }
            if next > 0 { return 0; }
            leaf = (*path).nodes[0];
        }
        btrfs_item_key_to_cpu(leaf, &mut key, (*path).slots[0]);
        btrfs_release_path(path);
        if key.objectid != BTRFS_ORPHAN_OBJECTID || key.type_ != BTRFS_ORPHAN_ITEM_KEY { return 0; }
        let root_objectid = key.offset;
        key.offset = key.offset.wrapping_add(1);
        let root = btrfs_get_fs_root(fs_info, root_objectid, false);
        let r = PTR_ERR_OR_ZERO(root);
        if r != 0 && r != -ENOENT { return r; }
        if r == -ENOENT {
            let trans = btrfs_join_transaction(tree_root);
            if IS_ERR(trans) { return PTR_ERR(trans); }
            let r = btrfs_del_orphan_item(trans, tree_root, root_objectid);
            btrfs_end_transaction(trans);
            if r != 0 { return r; }
            continue;
        }
        WARN_ON(!test_bit(BTRFS_ROOT_ORPHAN_ITEM_INSERTED, &mut (*root).state));
        if btrfs_root_refs(&(*root).root_item) == 0 {
            let mut drop_key: btrfs_key = core::mem::zeroed();
            btrfs_disk_key_to_cpu(&mut drop_key, &(*root).root_item.drop_progress);
            if drop_key.objectid != 0 || drop_key.type_ != 0 || drop_key.offset != 0 {
                set_bit(BTRFS_FS_UNFINISHED_DROPS, &mut (*fs_info).flags);
                set_bit(BTRFS_ROOT_UNFINISHED_DROP, &mut (*root).state);
            }
            set_bit(BTRFS_ROOT_DEAD_TREE, &mut (*root).state);
            btrfs_add_dead_root(root);
        }
        btrfs_put_root(root);
    }
}

pub unsafe fn btrfs_del_root(trans: *mut btrfs_trans_handle, key: *const btrfs_key) -> i32 {
    let root = (*trans).fs_info.tree_root;
    let path = btrfs_alloc_path();
    if path.is_null() { return -ENOMEM; }
    let ret = btrfs_search_slot(trans, root, key, path, -1, 1);
    if ret < 0 { return ret; }
    if ret > 0 { return -EUCLEAN; }
    btrfs_del_item(trans, root, path)
}

pub unsafe fn btrfs_del_root_ref(trans: *mut btrfs_trans_handle, root_id: u64, ref_id: u64,
                                 dirid: u64, sequence: *mut u64, name: *const fscrypt_str) -> i32 {
    let tree_root = (*trans).fs_info.tree_root;
    let path = btrfs_alloc_path();
    if path.is_null() { return -ENOMEM; }
    let mut key = btrfs_key { objectid: root_id, type_: BTRFS_ROOT_BACKREF_KEY, offset: ref_id };
    loop {
        let ret = btrfs_search_slot(trans, tree_root, &key, path, -1, 1);
        if ret < 0 { return ret; }
        if ret == 0 {
            let leaf = (*path).nodes[0];
            let ref_ = btrfs_item_ptr(leaf, (*path).slots[0], btrfs_root_ref);
            let ptr = (ref_ as usize + core::mem::size_of::<btrfs_root_ref>()) as usize;
            if btrfs_root_ref_dirid(leaf, ref_) != dirid || btrfs_root_ref_name_len(leaf, ref_) != (*name).len ||
               memcmp_extent_buffer(leaf, (*name).name, ptr, (*name).len) != 0 { return -ENOENT; }
            *sequence = btrfs_root_ref_sequence(leaf, ref_);
            let r = btrfs_del_item(trans, tree_root, path);
            if r != 0 { return r; }
        } else { return -ENOENT; }
        if key.type_ == BTRFS_ROOT_BACKREF_KEY {
            btrfs_release_path(path);
            key = btrfs_key { objectid: ref_id, type_: BTRFS_ROOT_REF_KEY, offset: root_id };
        } else { return ret; }
    }
}

pub unsafe fn btrfs_add_root_ref(trans: *mut btrfs_trans_handle, root_id: u64, ref_id: u64,
                                 dirid: u64, sequence: u64, name: *const fscrypt_str) -> i32 {
    let tree_root = (*trans).fs_info.tree_root;
    let path = btrfs_alloc_path();
    if path.is_null() { return -ENOMEM; }
    let mut key = btrfs_key { objectid: root_id, type_: BTRFS_ROOT_BACKREF_KEY, offset: ref_id };
    loop {
        let ret = btrfs_insert_empty_item(trans, tree_root, path, &key,
                                          core::mem::size_of::<btrfs_root_ref>() as u32 + (*name).len);
        if ret != 0 { btrfs_abort_transaction(trans, ret); return ret; }
        let leaf = (*path).nodes[0];
        let ref_ = btrfs_item_ptr(leaf, (*path).slots[0], btrfs_root_ref);
        btrfs_set_root_ref_dirid(leaf, ref_, dirid);
        btrfs_set_root_ref_sequence(leaf, ref_, sequence);
        btrfs_set_root_ref_name_len(leaf, ref_, (*name).len);
        let ptr = ref_ as usize + core::mem::size_of::<btrfs_root_ref>();
        write_extent_buffer(leaf, (*name).name, ptr, (*name).len as usize);
        if key.type_ == BTRFS_ROOT_BACKREF_KEY {
            btrfs_release_path(path);
            key = btrfs_key { objectid: ref_id, type_: BTRFS_ROOT_REF_KEY, offset: root_id };
        } else { return 0; }
    }
}

pub unsafe fn btrfs_check_and_init_root_item(root_item: *mut btrfs_root_item) {
    let mut inode_flags = btrfs_stack_inode_flags(&(*root_item).inode);
    if inode_flags & BTRFS_INODE_ROOT_ITEM_INIT == 0 {
        inode_flags |= BTRFS_INODE_ROOT_ITEM_INIT;
        btrfs_set_stack_inode_flags(&mut (*root_item).inode, inode_flags);
        btrfs_set_root_flags(root_item, 0);
        btrfs_set_root_limit(root_item, 0);
    }
}

pub unsafe fn btrfs_update_root_times(trans: *mut btrfs_trans_handle, root: *mut btrfs_root) {
    let item = &mut (*root).root_item;
    let mut ct: timespec64 = core::mem::zeroed();
    ktime_get_real_ts64(&mut ct);
    spin_lock(&mut (*root).root_item_lock);
    btrfs_set_root_ctransid(item, (*trans).transid);
    btrfs_set_stack_timespec_sec(&mut item.ctime, ct.tv_sec);
    btrfs_set_stack_timespec_nsec(&mut item.ctime, ct.tv_nsec);
    spin_unlock(&mut (*root).root_item_lock);
}

pub unsafe fn btrfs_subvolume_reserve_metadata(root: *mut btrfs_root, rsv: *mut btrfs_block_rsv,
                                               items: i32, use_global_rsv: bool) -> i32 {
    let mut qgroup_num_bytes = 0u64;
    let fs_info = (*root).fs_info;
    let global_rsv = &mut (*fs_info).global_block_rsv;
    if btrfs_qgroup_enabled(fs_info) {
        qgroup_num_bytes = 3 * (*fs_info).nodesize as u64;
        let ret = btrfs_qgroup_reserve_meta_prealloc(root, qgroup_num_bytes, true, false);
        if ret != 0 { return ret; }
    }
    let num_bytes = btrfs_calc_insert_metadata_size(fs_info, items);
    (*rsv).space_info = btrfs_find_space_info(fs_info, BTRFS_BLOCK_GROUP_METADATA);
    let mut ret = btrfs_block_rsv_add(fs_info, rsv, num_bytes, BTRFS_RESERVE_FLUSH_ALL);
    if ret == -ENOSPC && use_global_rsv {
        ret = btrfs_block_rsv_migrate(global_rsv, rsv, num_bytes, true);
    }
    if ret != 0 && qgroup_num_bytes != 0 { btrfs_qgroup_free_meta_prealloc(root, qgroup_num_bytes); }
    if ret == 0 {
        spin_lock(&mut (*rsv).lock);
        (*rsv).qgroup_rsv_reserved += qgroup_num_bytes;
        spin_unlock(&mut (*rsv).lock);
    }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
