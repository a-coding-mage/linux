// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2008 Red Hat.  All rights reserved.
 */

// Dependencies supplied by the surrounding translation unit.

pub unsafe fn btrfs_insert_orphan_item(
    trans: *mut btrfs_trans_handle,
    root: *mut btrfs_root,
    offset: u64,
) -> i32 {
    // BTRFS_PATH_AUTO_FREE(path)
    let path = btrfs_alloc_path();
    let mut key: btrfs_key = core::mem::zeroed();

    key.objectid = BTRFS_ORPHAN_OBJECTID;
    key.type_ = BTRFS_ORPHAN_ITEM_KEY;
    key.offset = offset;

    if path.is_null() {
        return -libc::ENOMEM;
    }

    btrfs_insert_empty_item(trans, root, path, &key, 0)
}

pub unsafe fn btrfs_del_orphan_item(
    trans: *mut btrfs_trans_handle,
    root: *mut btrfs_root,
    offset: u64,
) -> i32 {
    // BTRFS_PATH_AUTO_FREE(path)
    let path = btrfs_alloc_path();
    let mut key: btrfs_key = core::mem::zeroed();
    let mut ret: i32 = 0;

    key.objectid = BTRFS_ORPHAN_OBJECTID;
    key.type_ = BTRFS_ORPHAN_ITEM_KEY;
    key.offset = offset;

    if path.is_null() {
        return -libc::ENOMEM;
    }

    ret = btrfs_search_slot(trans, root, &key, path, -1, 1);
    if ret < 0 {
        return ret;
    }
    if ret != 0 {
        return -libc::ENOENT;
    }

    btrfs_del_item(trans, root, path)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
