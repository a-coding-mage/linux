// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2007 Red Hat.  All rights reserved.
 */

// Linux and btrfs headers from the original implementation provide the
// external types, constants, macros, and functions referenced below.

pub unsafe fn btrfs_getxattr(inode: *const inode, name: *const c_char,
                             buffer: *mut c_void, size: size_t) -> c_int {
    let mut di: *mut btrfs_dir_item;
    let root: *mut btrfs_root = BTRFS_I(inode).root;
    let mut path: *mut btrfs_path;
    let leaf: *mut extent_buffer;
    let data_ptr: c_ulong;

    path = btrfs_alloc_path();
    if path.is_null() { return -ENOMEM; }

    di = btrfs_lookup_xattr(ptr::null_mut(), root, path,
        btrfs_ino(BTRFS_I(inode)), name, strlen(name), 0);
    if di.is_null() { return -ENODATA; }
    if IS_ERR(di) { return PTR_ERR(di); }

    leaf = (*path).nodes[0];
    if size == 0 { return btrfs_dir_data_len(leaf, di) as c_int; }
    if btrfs_dir_data_len(leaf, di) > size { return -ERANGE; }

    data_ptr = (((di.offset(1)) as *mut c_char).add(
        btrfs_dir_name_len(leaf, di) as usize)) as c_ulong;
    read_extent_buffer(leaf, buffer, data_ptr,
                       btrfs_dir_data_len(leaf, di));
    btrfs_dir_data_len(leaf, di) as c_int
}

pub unsafe fn btrfs_setxattr(trans: *mut btrfs_trans_handle, inode: *mut inode,
    name: *const c_char, value: *const c_void, size: size_t, flags: c_int) -> c_int {
    let mut di: *mut btrfs_dir_item = ptr::null_mut();
    let root: *mut btrfs_root = BTRFS_I(inode).root;
    let mut path: *mut btrfs_path;
    let name_len = strlen(name);
    let mut ret: c_int = 0;

    ASSERT(!trans.is_null());
    if name_len + size > BTRFS_MAX_XATTR_SIZE((*root).fs_info) { return -ENOSPC; }
    path = btrfs_alloc_path();
    if path.is_null() { return -ENOMEM; }
    (*path).skip_release_on_error = true;

    if value.is_null() {
        di = btrfs_lookup_xattr(trans, root, path, btrfs_ino(BTRFS_I(inode)), name, name_len, -1);
        if di.is_null() && (flags & XATTR_REPLACE) != 0 { ret = -ENODATA; }
        else if IS_ERR(di) { ret = PTR_ERR(di); }
        else if !di.is_null() { ret = btrfs_delete_one_dir_name(trans, root, path, di); }
        return btrfs_setxattr_out(ret, inode);
    }

    if (flags & XATTR_REPLACE) != 0 {
        btrfs_assert_inode_locked(BTRFS_I(inode));
        di = btrfs_lookup_xattr(ptr::null_mut(), root, path, btrfs_ino(BTRFS_I(inode)), name, name_len, 0);
        if di.is_null() { ret = -ENODATA; }
        else if IS_ERR(di) { ret = PTR_ERR(di); }
        if ret != 0 { return btrfs_setxattr_out(ret, inode); }
        btrfs_release_path(path);
        di = ptr::null_mut();
    }

    ret = btrfs_insert_xattr_item(trans, root, path, btrfs_ino(BTRFS_I(inode)), name, name_len, value, size);
    if ret == -EOVERFLOW {
        ret = 0;
        btrfs_assert_tree_write_locked((*path).nodes[0]);
        di = btrfs_match_dir_item_name(path, name, name_len);
        if di.is_null() && (flags & XATTR_REPLACE) == 0 { ret = -ENOSPC; return btrfs_setxattr_out(ret, inode); }
    } else if ret == -EEXIST {
        ret = 0;
        di = btrfs_match_dir_item_name(path, name, name_len);
        ASSERT(!di.is_null());
    } else if ret != 0 { return btrfs_setxattr_out(ret, inode); }

    if !di.is_null() && (flags & XATTR_CREATE) != 0 { return btrfs_setxattr_out(-EEXIST, inode); }
    if !di.is_null() {
        let slot = (*path).slots[0];
        let leaf = (*path).nodes[0];
        let old_data_len = btrfs_dir_data_len(leaf, di) as u16;
        let item_size = btrfs_item_size(leaf, slot);
        let data_size = size_of::<btrfs_dir_item>() + name_len + size;
        let data_ptr: c_ulong;
        if size > old_data_len as usize && btrfs_leaf_free_space(leaf) < size - old_data_len as usize { return btrfs_setxattr_out(-ENOSPC, inode); }
        if old_data_len as usize + name_len + size_of::<btrfs_dir_item>() == item_size {
            if size > old_data_len as usize { btrfs_extend_item(trans, path, size - old_data_len as usize); }
            else if size < old_data_len as usize { btrfs_truncate_item(trans, path, data_size, 1); }
        } else {
            ret = btrfs_delete_one_dir_name(trans, root, path, di);
            if ret != 0 { return btrfs_setxattr_out(ret, inode); }
            btrfs_extend_item(trans, path, data_size);
        }
        let ptr = btrfs_item_ptr(leaf, slot, c_char).add(btrfs_item_size(leaf, slot) - data_size);
        di = ptr as *mut btrfs_dir_item;
        btrfs_set_dir_data_len(leaf, di, size as u16);
        data_ptr = (di.add(1) as usize + name_len) as c_ulong;
        write_extent_buffer(leaf, value, data_ptr, size);
    }
    btrfs_setxattr_out(ret, inode)
}

unsafe fn btrfs_setxattr_out(ret: c_int, inode: *mut inode) -> c_int {
    if ret == 0 {
        set_bit(BTRFS_INODE_COPY_EVERYTHING, &mut BTRFS_I(inode).runtime_flags);
        clear_bit(BTRFS_INODE_NO_XATTRS, &mut BTRFS_I(inode).runtime_flags);
    }
    ret
}

/* @value: "" makes the attribute empty, NULL removes it. */
pub unsafe fn btrfs_setxattr_trans(inode: *mut inode, name: *const c_char,
    value: *const c_void, size: size_t, flags: c_int) -> c_int {
    let root = BTRFS_I(inode).root;
    let start_trans = (*current).journal_info.is_null();
    let trans: *mut btrfs_trans_handle;
    let mut ret: c_int;
    if start_trans {
        trans = btrfs_start_transaction(root, 2);
        if IS_ERR(trans) { return PTR_ERR(trans); }
    } else {
        ASSERT(strncmp(name, XATTR_SECURITY_PREFIX, XATTR_SECURITY_PREFIX_LEN) == 0);
        trans = (*current).journal_info as *mut btrfs_trans_handle;
    }
    ret = btrfs_setxattr(trans, inode, name, value, size, flags);
    if ret == 0 {
        inode_inc_iversion(inode); inode_set_ctime_current(inode);
        ret = btrfs_update_inode(trans, BTRFS_I(inode));
        if ret != 0 { btrfs_abort_transaction(trans, ret); }
    }
    if start_trans { btrfs_end_transaction(trans); }
    ret
}

pub unsafe fn btrfs_listxattr(dentry: *mut dentry, buffer: *mut c_char, size: size_t) -> ssize_t {
    let mut found_key = btrfs_key::default(); let mut key = btrfs_key::default();
    let inode = d_inode(dentry); let root = BTRFS_I(inode).root;
    let mut path: *mut btrfs_path; let mut iter_ret = 0; let mut ret = 0;
    let mut total_size: size_t = 0; let mut size_left = size;
    key.objectid = btrfs_ino(BTRFS_I(inode)); key.type_ = BTRFS_XATTR_ITEM_KEY; key.offset = 0;
    path = btrfs_alloc_path(); if path.is_null() { return -ENOMEM as ssize_t; }
    (*path).reada = READA_FORWARD;
    btrfs_for_each_slot!(root, &key, &mut found_key, path, iter_ret, {
        let leaf = (*path).nodes[0]; let slot = (*path).slots[0];
        if found_key.objectid != key.objectid || found_key.type_ > BTRFS_XATTR_ITEM_KEY { break; }
        if found_key.type_ < BTRFS_XATTR_ITEM_KEY { continue; }
        let mut di = btrfs_item_ptr(leaf, slot, btrfs_dir_item) as *mut btrfs_dir_item;
        let item_size = btrfs_item_size(leaf, slot); let mut cur = 0;
        while cur < item_size {
            let name_len = btrfs_dir_name_len(leaf, di); let data_len = btrfs_dir_data_len(leaf, di);
            let this_len = size_of::<btrfs_dir_item>() + name_len as usize + data_len as usize;
            let name_ptr = di.add(1) as c_ulong; total_size += name_len as usize + 1;
            if size == 0 { cur += this_len; di = (di as *mut c_char).add(this_len) as *mut btrfs_dir_item; continue; }
            if buffer.is_null() || name_len as usize + 1 > size_left { iter_ret = -ERANGE; break; }
            read_extent_buffer(leaf, buffer as *mut c_void, name_ptr, name_len as usize);
            *buffer.add(name_len as usize) = 0; size_left -= name_len as usize + 1; buffer = buffer.add(name_len as usize + 1);
            cur += this_len; di = (di as *mut c_char).add(this_len) as *mut btrfs_dir_item;
        }
    });
    if iter_ret < 0 { ret = iter_ret; } else { ret = total_size as c_int; } ret as ssize_t
}

unsafe fn btrfs_xattr_handler_get(handler: *const xattr_handler, _unused: *mut dentry, inode: *mut inode, name: *const c_char, buffer: *mut c_void, size: size_t) -> c_int { btrfs_getxattr(inode, xattr_full_name(handler, name), buffer, size) }
unsafe fn btrfs_xattr_handler_set(handler: *const xattr_handler, _idmap: *mut mnt_idmap, _unused: *mut dentry, inode: *mut inode, name: *const c_char, buffer: *const c_void, size: size_t, flags: c_int) -> c_int { if btrfs_root_readonly(BTRFS_I(inode).root) { return -EROFS; } btrfs_setxattr_trans(inode, xattr_full_name(handler, name), buffer, size, flags) }
unsafe fn btrfs_xattr_handler_get_security(handler: *const xattr_handler, _unused: *mut dentry, inode: *mut inode, name: *const c_char, buffer: *mut c_void, size: size_t) -> c_int { let name = xattr_full_name(handler, name); let is_cap = strcmp(name, XATTR_NAME_CAPS) == 0; if is_cap && test_bit(BTRFS_INODE_NO_CAP_XATTR, &BTRFS_I(inode).runtime_flags) { return -ENODATA; } let ret = btrfs_getxattr(inode, name, buffer, size); if ret == -ENODATA && is_cap { set_bit(BTRFS_INODE_NO_CAP_XATTR, &mut BTRFS_I(inode).runtime_flags); } ret }
unsafe fn btrfs_xattr_handler_set_security(handler: *const xattr_handler, _idmap: *mut mnt_idmap, _unused: *mut dentry, inode: *mut inode, name: *const c_char, buffer: *const c_void, size: size_t, flags: c_int) -> c_int { if btrfs_root_readonly(BTRFS_I(inode).root) { return -EROFS; } let name = xattr_full_name(handler, name); if strcmp(name, XATTR_NAME_CAPS) == 0 { clear_bit(BTRFS_INODE_NO_CAP_XATTR, &mut BTRFS_I(inode).runtime_flags); } btrfs_setxattr_trans(inode, name, buffer, size, flags) }
unsafe fn btrfs_xattr_handler_set_prop(handler: *const xattr_handler, _idmap: *mut mnt_idmap, _unused: *mut dentry, inode: *mut inode, name: *const c_char, value: *const c_void, size: size_t, flags: c_int) -> c_int { let name = xattr_full_name(handler, name); let root = BTRFS_I(inode).root; let mut ret = btrfs_validate_prop(BTRFS_I(inode), name, value, size); if ret != 0 { return ret; } if btrfs_ignore_prop(BTRFS_I(inode), name) { return 0; } let trans = btrfs_start_transaction(root, 2); if IS_ERR(trans) { return PTR_ERR(trans); } ret = btrfs_set_prop(trans, BTRFS_I(inode), name, value, size, flags); if ret == 0 { inode_inc_iversion(inode); inode_set_ctime_current(inode); ret = btrfs_update_inode(trans, BTRFS_I(inode)); if ret != 0 { btrfs_abort_transaction(trans, ret); } } btrfs_end_transaction(trans); ret }

pub static btrfs_security_xattr_handler: xattr_handler = xattr_handler { prefix: XATTR_SECURITY_PREFIX, get: Some(btrfs_xattr_handler_get_security), set: Some(btrfs_xattr_handler_set_security) };
pub static btrfs_trusted_xattr_handler: xattr_handler = xattr_handler { prefix: XATTR_TRUSTED_PREFIX, get: Some(btrfs_xattr_handler_get), set: Some(btrfs_xattr_handler_set) };
pub static btrfs_user_xattr_handler: xattr_handler = xattr_handler { prefix: XATTR_USER_PREFIX, get: Some(btrfs_xattr_handler_get), set: Some(btrfs_xattr_handler_set) };
pub static btrfs_btrfs_xattr_handler: xattr_handler = xattr_handler { prefix: XATTR_BTRFS_PREFIX, get: Some(btrfs_xattr_handler_get), set: Some(btrfs_xattr_handler_set_prop) };
pub static btrfs_xattr_handlers: [*const xattr_handler; 5] = [&btrfs_security_xattr_handler, &btrfs_trusted_xattr_handler, &btrfs_user_xattr_handler, &btrfs_btrfs_xattr_handler, ptr::null()];

unsafe fn btrfs_initxattrs(inode: *mut inode, xattr_array: *const xattr, fs_private: *mut c_void) -> c_int {
    let trans = fs_private as *mut btrfs_trans_handle; let mut ret = 0; let nofs_flag = memalloc_nofs_save(); let mut xattr = xattr_array;
    while !(*xattr).name.is_null() { let name_len = XATTR_SECURITY_PREFIX_LEN + strlen((*xattr).name) + 1; let name = kmalloc(name_len, GFP_KERNEL); if name.is_null() { ret = -ENOMEM; break; } scnprintf(name as *mut c_char, name_len, cstr!("%s%s"), XATTR_SECURITY_PREFIX, (*xattr).name); if strcmp(name as *const c_char, XATTR_NAME_CAPS) == 0 { clear_bit(BTRFS_INODE_NO_CAP_XATTR, &mut BTRFS_I(inode).runtime_flags); } ret = btrfs_setxattr(trans, inode, name as *const c_char, (*xattr).value, (*xattr).value_len, 0); kfree(name); if ret < 0 { break; } xattr = xattr.add(1); }
    memalloc_nofs_restore(nofs_flag); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
