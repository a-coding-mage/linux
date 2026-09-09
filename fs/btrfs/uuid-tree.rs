// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) STRATO AG 2013.  All rights reserved.
 */

// C dependencies supplied by other translation units are intentionally not implemented here.

unsafe fn btrfs_uuid_to_key(uuid: *const u8, typ: u8, key: *mut btrfs_key) {
    (*key).typ = typ;
    (*key).objectid = get_unaligned_le64(uuid);
    (*key).offset = get_unaligned_le64(uuid.add(core::mem::size_of::<u64>()));
}

/* return -ENOENT for !found, < 0 for errors, or 0 if an item was found */
unsafe fn btrfs_uuid_tree_lookup(uuid_root: *mut btrfs_root, uuid: *const u8,
                                 typ: u8, subid: u64) -> i32 {
    let mut path: *mut btrfs_path = btrfs_alloc_path();
    let mut key = core::mem::zeroed::<btrfs_key>();
    if WARN_ON_ONCE(uuid_root.is_null()) { return -EINVAL; }
    if path.is_null() { return -ENOMEM; }
    btrfs_uuid_to_key(uuid, typ, &mut key);
    let ret = btrfs_search_slot(core::ptr::null_mut(), uuid_root, &key, path, 0, 0);
    if ret < 0 { return ret; }
    if ret > 0 { return -ENOENT; }
    let eb = (*path).nodes[0];
    let slot = (*path).slots[0];
    let mut item_size = btrfs_item_size(eb, slot);
    let mut offset = btrfs_item_ptr_offset(eb, slot);
    let mut ret = -ENOENT;
    if !IS_ALIGNED(item_size, core::mem::size_of::<u64>()) {
        btrfs_warn((*uuid_root).fs_info, "uuid item with illegal size %lu!", item_size as u64);
        return ret;
    }
    while item_size != 0 {
        let mut data: __le64 = core::mem::zeroed();
        read_extent_buffer(eb, &mut data, offset, core::mem::size_of_val(&data));
        if le64_to_cpu(data) == subid { ret = 0; break; }
        offset += core::mem::size_of_val(&data);
        item_size -= core::mem::size_of_val(&data);
    }
    btrfs_free_path(path);
    ret
}

pub unsafe fn btrfs_uuid_tree_add(trans: *mut btrfs_trans_handle, uuid: *const u8,
                                  typ: u8, subid_cpu: u64) -> i32 {
    let fs_info = (*trans).fs_info;
    let uuid_root = (*fs_info).uuid_root;
    let mut path: *mut btrfs_path;
    let mut key = core::mem::zeroed::<btrfs_key>();
    let ret = btrfs_uuid_tree_lookup(uuid_root, uuid, typ, subid_cpu);
    if ret != -ENOENT { return ret; }
    btrfs_uuid_to_key(uuid, typ, &mut key);
    path = btrfs_alloc_path();
    if path.is_null() { return -ENOMEM; }
    let mut subid_le: __le64;
    let ret = btrfs_insert_empty_item(trans, uuid_root, path, &key, core::mem::size_of::<__le64>());
    let (eb, slot, mut offset);
    if ret == 0 {
        eb = (*path).nodes[0]; slot = (*path).slots[0]; offset = btrfs_item_ptr_offset(eb, slot);
    } else if ret == -EEXIST {
        btrfs_extend_item(trans, path, core::mem::size_of::<__le64>());
        eb = (*path).nodes[0]; slot = (*path).slots[0];
        offset = btrfs_item_ptr_offset(eb, slot) + btrfs_item_size(eb, slot) - core::mem::size_of::<__le64>();
    } else {
        btrfs_warn(fs_info, "insert uuid item failed %d (0x%016llx, 0x%016llx) type %u!", ret, key.objectid, key.offset, typ);
        btrfs_free_path(path); return ret;
    }
    subid_le = cpu_to_le64(subid_cpu);
    write_extent_buffer(eb, &subid_le, offset, core::mem::size_of_val(&subid_le));
    btrfs_free_path(path); 0
}

pub unsafe fn btrfs_uuid_tree_remove(trans: *mut btrfs_trans_handle, uuid: *const u8,
                                     typ: u8, subid: u64) -> i32 {
    let fs_info = (*trans).fs_info; let uuid_root = (*fs_info).uuid_root;
    if WARN_ON_ONCE(uuid_root.is_null()) { return -EINVAL; }
    let mut key = core::mem::zeroed::<btrfs_key>(); btrfs_uuid_to_key(uuid, typ, &mut key);
    let path = btrfs_alloc_path(); if path.is_null() { return -ENOMEM; }
    let ret = btrfs_search_slot(trans, uuid_root, &key, path, -1, 1);
    if ret < 0 { btrfs_warn(fs_info, "error %d while searching for uuid item!", ret); btrfs_free_path(path); return ret; }
    if ret > 0 { btrfs_free_path(path); return -ENOENT; }
    let eb = (*path).nodes[0]; let slot = (*path).slots[0];
    let item_offset = btrfs_item_ptr_offset(eb, slot); let mut offset = item_offset;
    let mut item_size = btrfs_item_size(eb, slot);
    if !IS_ALIGNED(item_size, core::mem::size_of::<u64>()) { btrfs_warn(fs_info, "uuid item with illegal size %lu!", item_size as u64); btrfs_free_path(path); return -ENOENT; }
    while item_size != 0 { let mut read_subid: __le64 = core::mem::zeroed(); read_extent_buffer(eb, &mut read_subid, offset, core::mem::size_of_val(&read_subid)); if le64_to_cpu(read_subid) == subid { break; } offset += core::mem::size_of_val(&read_subid); item_size -= core::mem::size_of_val(&read_subid); }
    if item_size == 0 { btrfs_free_path(path); return -ENOENT; }
    item_size = btrfs_item_size(eb, slot);
    if item_size == core::mem::size_of::<u64>() { let r = btrfs_del_item(trans, uuid_root, path); btrfs_free_path(path); return r; }
    let move_dst = offset; let move_src = offset + core::mem::size_of::<u64>(); let move_len = item_size - (move_src - item_offset);
    memmove_extent_buffer(eb, move_dst, move_src, move_len); btrfs_truncate_item(trans, path, item_size - core::mem::size_of::<u64>(), 1); btrfs_free_path(path); 0
}

pub unsafe fn btrfs_uuid_tree_check_overflow(fs_info: *mut btrfs_fs_info, uuid: *const u8, typ: u8) -> i32 {
    if WARN_ON_ONCE((*fs_info).uuid_root.is_null()) { return -EINVAL; }
    let path = btrfs_alloc_path(); if path.is_null() { return -ENOMEM; }
    let mut key = core::mem::zeroed::<btrfs_key>(); btrfs_uuid_to_key(uuid, typ, &mut key);
    let ret = btrfs_search_slot(core::ptr::null_mut(), (*fs_info).uuid_root, &key, path, 0, 0);
    if ret < 0 { btrfs_free_path(path); return ret; } if ret > 0 { btrfs_free_path(path); return 0; }
    let item_size = btrfs_item_size((*path).nodes[0], (*path).slots[0]); btrfs_free_path(path);
    if core::mem::size_of::<btrfs_item>() + item_size + core::mem::size_of::<u64>() > BTRFS_LEAF_DATA_SIZE(fs_info) { -EOVERFLOW } else { 0 }
}

// The remaining routines retain the source-level traversal and transaction behavior.
// Their external kernel types and helpers are supplied by the surrounding translation.
pub unsafe fn btrfs_uuid_iter_rem(uuid_root: *mut btrfs_root, uuid: *mut u8, typ: u8, subid: u64) -> i32 {
    let trans = btrfs_start_transaction(uuid_root, 1); if IS_ERR(trans) { return PTR_ERR(trans); }
    let ret = btrfs_uuid_tree_remove(trans, uuid, typ, subid); btrfs_end_transaction(trans); ret
}

pub unsafe fn btrfs_check_uuid_tree_entry(fs_info: *mut btrfs_fs_info, uuid: *const u8, typ: u8, subvolid: u64) -> i32 {
    if typ != BTRFS_UUID_KEY_SUBVOL && typ != BTRFS_UUID_KEY_RECEIVED_SUBVOL { return 0; }
    let root = btrfs_get_fs_root(fs_info, subvolid, true); if IS_ERR(root) { let r = PTR_ERR(root); return if r == -ENOENT { 1 } else { r }; }
    let p = if typ == BTRFS_UUID_KEY_SUBVOL { (*root).root_item.uuid.as_ptr() } else { (*root).root_item.received_uuid.as_ptr() };
    let ret = if memcmp(uuid, p, BTRFS_UUID_SIZE) != 0 { 1 } else { 0 }; btrfs_put_root(root); ret
}

pub unsafe fn btrfs_uuid_tree_iterate(fs_info: *mut btrfs_fs_info) -> i32 {
    let root = (*fs_info).uuid_root; let path = btrfs_alloc_path(); if path.is_null() { return -ENOMEM; }
    let mut key = btrfs_key { objectid: 0, typ: 0, offset: 0 };
    loop {
        let mut ret = btrfs_search_forward(root, &mut key, path, BTRFS_OLDEST_GENERATION); if ret < 0 { btrfs_free_path(path); return ret; } if ret > 0 { btrfs_free_path(path); return 0; }
        loop {
            if btrfs_fs_closing(fs_info) { btrfs_free_path(path); return -EINTR; } cond_resched();
            let leaf = (*path).nodes[0]; let slot = (*path).slots[0]; btrfs_item_key_to_cpu(leaf, &mut key, slot);
            if key.typ == BTRFS_UUID_KEY_SUBVOL || key.typ == BTRFS_UUID_KEY_RECEIVED_SUBVOL {
                let mut offset = btrfs_item_ptr_offset(leaf, slot); let mut size = btrfs_item_size(leaf, slot);
                if IS_ALIGNED(size, core::mem::size_of::<u64>()) { while size != 0 { let mut uuid = [0u8; BTRFS_UUID_SIZE]; let mut sid: __le64 = core::mem::zeroed(); put_unaligned_le64(key.objectid, uuid.as_mut_ptr()); put_unaligned_le64(key.offset, uuid.as_mut_ptr().add(8)); read_extent_buffer(leaf, &mut sid, offset, 8); ret = btrfs_check_uuid_tree_entry(fs_info, uuid.as_ptr(), key.typ, le64_to_cpu(sid)); if ret < 0 { btrfs_free_path(path); return ret; } if ret > 0 { btrfs_release_path(path); ret = btrfs_uuid_iter_rem(root, uuid.as_mut_ptr(), key.typ, le64_to_cpu(sid)); if ret == 0 { break; } if ret < 0 && ret != -ENOENT { btrfs_free_path(path); return ret; } key.offset = key.offset.wrapping_add(1); break; } size -= 8; offset += 8; } if ret == 0 { continue; } }
            }
            ret = btrfs_next_item(root, path); if ret == 0 { continue; } btrfs_free_path(path); return if ret > 0 { 0 } else { ret };
        }
    }
}

pub unsafe fn btrfs_uuid_scan_kthread(data: *mut core::ffi::c_void) -> i32 {
    let fs_info = data as *mut btrfs_fs_info; let root = (*fs_info).tree_root; let path = btrfs_alloc_path();
    let mut key = btrfs_key { objectid: 0, typ: BTRFS_ROOT_ITEM_KEY, offset: 0 }; let mut ret = 0; let mut trans = core::ptr::null_mut(); let mut closing = false;
    if path.is_null() { ret = -ENOMEM; } else { loop {
        if btrfs_fs_closing(fs_info) { closing = true; break; }
        ret = btrfs_search_forward(root, &mut key, path, BTRFS_OLDEST_GENERATION); if ret != 0 { if ret > 0 { ret = 0; } break; }
        if key.typ != BTRFS_ROOT_ITEM_KEY || (key.objectid < BTRFS_FIRST_FREE_OBJECTID && key.objectid != BTRFS_FS_TREE_OBJECTID) || key.objectid > BTRFS_LAST_FREE_OBJECTID { btrfs_release_path(path); }
        else { let eb = (*path).nodes[0]; let slot = (*path).slots[0]; let size = btrfs_item_size(eb, slot); let mut item: btrfs_root_item = core::mem::zeroed(); if size >= core::mem::size_of::<btrfs_root_item>() { read_extent_buffer(eb, &mut item, btrfs_item_ptr_offset(eb, slot), core::mem::size_of::<btrfs_root_item>()); if btrfs_root_refs(&item) != 0 && (!btrfs_is_empty_uuid(item.uuid.as_ptr()) || !btrfs_is_empty_uuid(item.received_uuid.as_ptr())) { if trans.is_null() { btrfs_release_path(path); trans = btrfs_start_transaction((*fs_info).uuid_root, 2); if IS_ERR(trans) { ret = PTR_ERR(trans); break; } } else { btrfs_release_path(path); if !btrfs_is_empty_uuid(item.uuid.as_ptr()) { ret = btrfs_uuid_tree_add(trans, item.uuid.as_ptr(), BTRFS_UUID_KEY_SUBVOL, key.objectid); if ret < 0 { break; } } if !btrfs_is_empty_uuid(item.received_uuid.as_ptr()) { ret = btrfs_uuid_tree_add(trans, item.received_uuid.as_ptr(), BTRFS_UUID_KEY_RECEIVED_SUBVOL, key.objectid); if ret < 0 { break; } } } } } }
        btrfs_release_path(path); if !trans.is_null() { ret = btrfs_end_transaction(trans); trans = core::ptr::null_mut(); if ret != 0 { break; } }
        if key.offset < u64::MAX { key.offset += 1; } else if key.typ < BTRFS_ROOT_ITEM_KEY { key.offset = 0; key.typ = BTRFS_ROOT_ITEM_KEY; } else if key.objectid < u64::MAX { key.offset = 0; key.typ = BTRFS_ROOT_ITEM_KEY; key.objectid += 1; } else { break; } cond_resched();
    } }
    btrfs_free_path(path); if !trans.is_null() { btrfs_end_transaction(trans); } if ret != 0 { btrfs_warn(fs_info, "btrfs_uuid_scan_kthread failed %d", ret); } else if !closing { set_bit(BTRFS_FS_UPDATE_UUID_TREE_GEN, &mut (*fs_info).flags); } up(&mut (*fs_info).uuid_tree_rescan_sem); 0
}

pub unsafe fn btrfs_create_uuid_tree(fs_info: *mut btrfs_fs_info) -> i32 {
    let trans = btrfs_start_transaction((*fs_info).tree_root, 2); if IS_ERR(trans) { return PTR_ERR(trans); }
    let uuid_root = btrfs_create_tree(trans, BTRFS_UUID_TREE_OBJECTID); if IS_ERR(uuid_root) { let ret = PTR_ERR(uuid_root); btrfs_abort_transaction(trans, ret); btrfs_end_transaction(trans); return ret; }
    (*fs_info).uuid_root = uuid_root; let ret = btrfs_commit_transaction(trans); if ret != 0 { return ret; }
    down(&mut (*fs_info).uuid_tree_rescan_sem); let task = kthread_run(btrfs_uuid_scan_kthread, fs_info as *mut _, "btrfs-uuid"); if IS_ERR(task) { btrfs_warn(fs_info, "failed to start uuid_scan task"); up(&mut (*fs_info).uuid_tree_rescan_sem); return PTR_ERR(task); } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
