// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2013 Fusion IO.  All rights reserved.
 */

// Linux and btrfs dependencies are supplied by the surrounding translation.

unsafe fn test_btrfs_split_item(sectorsize: u32, nodesize: u32) -> i32 {
    let mut fs_info: *mut btrfs_fs_info;
    let mut path: *mut btrfs_path = core::ptr::null_mut();
    let mut root: *mut btrfs_root = core::ptr::null_mut();
    let eb: *mut extent_buffer;
    let value = b"mary had a little lamb\0";
    let split1 = b"mary had a little\0";
    let split2 = b" lamb\0";
    let split3 = b"mary\0";
    let split4 = b" had a little\0";
    let mut buf = [0i8; 32];
    let mut key: btrfs_key;
    let value_len = libc::strlen(value.as_ptr() as *const libc::c_char) as u32;
    let mut ret: i32 = 0;

    test_msg(b"running btrfs_split_item tests\0".as_ptr() as *const libc::c_char);

    fs_info = btrfs_alloc_dummy_fs_info(nodesize, sectorsize);
    if fs_info.is_null() {
        test_std_err(TEST_ALLOC_FS_INFO);
        return -ENOMEM;
    }

    root = btrfs_alloc_dummy_root(fs_info);
    if IS_ERR(root) {
        test_std_err(TEST_ALLOC_ROOT);
        ret = PTR_ERR(root);
        return goto_out(path, root, fs_info, ret);
    }

    path = btrfs_alloc_path();
    if path.is_null() {
        test_std_err(TEST_ALLOC_PATH);
        ret = -ENOMEM;
        return goto_out(path, root, fs_info, ret);
    }

    eb = alloc_dummy_extent_buffer(fs_info, nodesize);
    (*path).nodes[0] = eb;
    if eb.is_null() {
        test_std_err(TEST_ALLOC_EXTENT_BUFFER);
        ret = -ENOMEM;
        return goto_out(path, root, fs_info, ret);
    }
    (*path).slots[0] = 0;

    key.objectid = 0;
    key.type = BTRFS_EXTENT_CSUM_KEY;
    key.offset = 0;

    /*
     * Passing a NULL trans handle is fine here, we have a dummy root eb
     * and the tree is a single node (level 0).
     */
    btrfs_setup_item_for_insert(core::ptr::null_mut(), root, path, &mut key, value_len);
    write_extent_buffer(eb, value.as_ptr() as *const libc::c_void,
                        btrfs_item_ptr_offset(eb, 0), value_len);

    key.offset = 3;

    /*
     * Passing NULL trans here should be safe because we have plenty of
     * space in this leaf to split the item without having to split the
     * leaf.
     */
    ret = btrfs_split_item(core::ptr::null_mut(), root, path, &key, 17);
    if ret != 0 {
        test_err(b"split item failed %d\0".as_ptr() as *const libc::c_char, ret);
        return goto_out(path, root, fs_info, ret);
    }

    /* Read the first slot, it should have the original key and contain only
     * 'mary had a little' */
    btrfs_item_key_to_cpu(eb, &mut key, 0);
    if key.objectid != 0 || key.type_ != BTRFS_EXTENT_CSUM_KEY || key.offset != 0 {
        test_err(b"invalid key at slot 0\0".as_ptr() as *const libc::c_char);
        ret = -EINVAL;
        return goto_out(path, root, fs_info, ret);
    }
    if btrfs_item_size(eb, 0) != libc::strlen(split1.as_ptr() as *const libc::c_char) as u32 {
        test_err(b"invalid len in the first split\0".as_ptr() as *const libc::c_char);
        ret = -EINVAL;
        goto_out(path, root, fs_info, ret);
    }
    read_extent_buffer(eb, buf.as_mut_ptr() as *mut libc::c_void, btrfs_item_ptr_offset(eb, 0),
                       libc::strlen(split1.as_ptr() as *const libc::c_char) as u32);
    if libc::memcmp(buf.as_ptr() as *const libc::c_void, split1.as_ptr() as *const libc::c_void,
                    libc::strlen(split1.as_ptr() as *const libc::c_char)) != 0 {
        test_err(b"data in the buffer doesn't match what it should in the first split\0".as_ptr() as *const libc::c_char);
        ret = -EINVAL;
        goto_out(path, root, fs_info, ret);
    }

    btrfs_item_key_to_cpu(eb, &mut key, 1);
    if key.objectid != 0 || key.type_ != BTRFS_EXTENT_CSUM_KEY || key.offset != 3 {
        test_err(b"invalid key at slot 1\0".as_ptr() as *const libc::c_char);
        ret = -EINVAL;
        return goto_out(path, root, fs_info, ret);
    }
    if btrfs_item_size(eb, 1) != libc::strlen(split2.as_ptr() as *const libc::c_char) as u32 {
        test_err(b"invalid len in the second split\0".as_ptr() as *const libc::c_char);
        ret = -EINVAL;
        return goto_out(path, root, fs_info, ret);
    }
    read_extent_buffer(eb, buf.as_mut_ptr() as *mut libc::c_void, btrfs_item_ptr_offset(eb, 1),
                       libc::strlen(split2.as_ptr() as *const libc::c_char) as u32);
    if libc::memcmp(buf.as_ptr() as *const libc::c_void, split2.as_ptr() as *const libc::c_void,
                    libc::strlen(split2.as_ptr() as *const libc::c_char)) != 0 {
        test_err(b"data in the buffer doesn't match what it should in the second split\0".as_ptr() as *const libc::c_char);
        ret = -EINVAL;
        return goto_out(path, root, fs_info, ret);
    }

    key.offset = 1;
    ret = btrfs_split_item(core::ptr::null_mut(), root, path, &key, 4);
    if ret != 0 {
        test_err(b"second split item failed %d\0".as_ptr() as *const libc::c_char, ret);
        return goto_out(path, root, fs_info, ret);
    }

    btrfs_item_key_to_cpu(eb, &mut key, 0);
    if key.objectid != 0 || key.type_ != BTRFS_EXTENT_CSUM_KEY || key.offset != 0 {
        test_err(b"invalid key at slot 0\0".as_ptr() as *const libc::c_char);
        ret = -EINVAL;
        return goto_out(path, root, fs_info, ret);
    }

    if btrfs_item_size(eb, 0) != libc::strlen(split3.as_ptr() as *const libc::c_char) as u32 {
        test_err(b"invalid len in the first split\0".as_ptr() as *const libc::c_char);
        ret = -EINVAL;
        return goto_out(path, root, fs_info, ret);
    }
    read_extent_buffer(eb, buf.as_mut_ptr() as *mut libc::c_void, btrfs_item_ptr_offset(eb, 0),
                       libc::strlen(split3.as_ptr() as *const libc::c_char) as u32);
    if libc::memcmp(buf.as_ptr() as *const libc::c_void, split3.as_ptr() as *const libc::c_void,
                    libc::strlen(split3.as_ptr() as *const libc::c_char)) != 0 {
        test_err(b"data in the buffer doesn't match what it should in the third split\0".as_ptr() as *const libc::c_char);
        ret = -EINVAL;
        return goto_out(path, root, fs_info, ret);
    }
    btrfs_item_key_to_cpu(eb, &mut key, 1);
    if key.objectid != 0 || key.type_ != BTRFS_EXTENT_CSUM_KEY || key.offset != 1 {
        test_err(b"invalid key at slot 1\0".as_ptr() as *const libc::c_char);
        ret = -EINVAL;
        return goto_out(path, root, fs_info, ret);
    }
    if btrfs_item_size(eb, 1) != libc::strlen(split4.as_ptr() as *const libc::c_char) as u32 {
        test_err(b"invalid len in the second split\0".as_ptr() as *const libc::c_char);
        ret = -EINVAL;
        return goto_out(path, root, fs_info, ret);
    }
    read_extent_buffer(eb, buf.as_mut_ptr() as *mut libc::c_void, btrfs_item_ptr_offset(eb, 1),
                       libc::strlen(split4.as_ptr() as *const libc::c_char) as u32);
    if libc::memcmp(buf.as_ptr() as *const libc::c_void, split4.as_ptr() as *const libc::c_void,
                    libc::strlen(split4.as_ptr() as *const libc::c_char)) != 0 {
        test_err(b"data in the buffer doesn't match what it should in the fourth split\0".as_ptr() as *const libc::c_char);
        ret = -EINVAL;
        return goto_out(path, root, fs_info, ret);
    }
    btrfs_item_key_to_cpu(eb, &mut key, 2);
    if key.objectid != 0 || key.type_ != BTRFS_EXTENT_CSUM_KEY || key.offset != 3 {
        test_err(b"invalid key at slot 2\0".as_ptr() as *const libc::c_char);
        ret = -EINVAL;
        return goto_out(path, root, fs_info, ret);
    }
    if btrfs_item_size(eb, 2) != libc::strlen(split2.as_ptr() as *const libc::c_char) as u32 {
        test_err(b"invalid len in the second split\0".as_ptr() as *const libc::c_char);
        ret = -EINVAL;
        return goto_out(path, root, fs_info, ret);
    }
    read_extent_buffer(eb, buf.as_mut_ptr() as *mut libc::c_void, btrfs_item_ptr_offset(eb, 2),
                       libc::strlen(split2.as_ptr() as *const libc::c_char) as u32);
    if libc::memcmp(buf.as_ptr() as *const libc::c_void, split2.as_ptr() as *const libc::c_void,
                    libc::strlen(split2.as_ptr() as *const libc::c_char)) != 0 {
        test_err(b"data in the buffer doesn't match what it should in the last chunk\0".as_ptr() as *const libc::c_char);
        ret = -EINVAL;
        return goto_out(path, root, fs_info, ret);
    }

    return goto_out(path, root, fs_info, ret);
}

pub unsafe fn btrfs_test_extent_buffer_operations(sectorsize: u32, nodesize: u32) -> i32 {
    test_msg(b"running extent buffer operation tests\0".as_ptr() as *const libc::c_char);
    test_btrfs_split_item(sectorsize, nodesize)
}

// Local equivalent of the C cleanup label; the surrounding translation supplies these functions.
unsafe fn goto_out(path: *mut btrfs_path, root: *mut btrfs_root,
                   fs_info: *mut btrfs_fs_info, ret: i32) -> i32 {
    btrfs_free_path(path);
    btrfs_free_dummy_root(root);
    btrfs_free_dummy_fs_info(fs_info);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
