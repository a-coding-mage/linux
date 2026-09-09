// SPDX-License-Identifier: GPL-2.0

// External Linux/Btrfs declarations supplied by other translation units.

const MERKLE_START_ALIGN: u64 = 65536;

unsafe fn merkle_file_pos(inode: *const inode) -> loff_t {
    let sz: u64 = (*inode).i_size;
    let rounded = (sz + MERKLE_START_ALIGN - 1) & !(MERKLE_START_ALIGN - 1);
    if rounded > (*(*inode).i_sb).s_maxbytes { return -EFBIG; }
    rounded as loff_t
}

unsafe fn drop_verity_items(inode: *mut btrfs_inode, key_type: u8) -> c_int {
    let root = (*inode).root;
    let mut path = btrfs_alloc_path();
    let mut key: btrfs_key = core::mem::zeroed();
    let mut count = 0;
    if path.is_null() { return -ENOMEM; }
    loop {
        let trans = btrfs_start_transaction(root, 1);
        if IS_ERR(trans) { return PTR_ERR(trans); }
        key.objectid = btrfs_ino(inode);
        key.type_ = key_type;
        key.offset = u64::MAX;
        let mut ret = btrfs_search_slot(trans, root, &mut key, path, -1, 1);
        if ret > 0 {
            ret = 0;
            if (*path).slots[0] == 0 { btrfs_end_transaction(trans); break; }
            (*path).slots[0] -= 1;
        } else if ret < 0 {
            btrfs_end_transaction(trans); return ret;
        }
        btrfs_item_key_to_cpu((*path).nodes[0], &mut key, (*path).slots[0]);
        if key.objectid != btrfs_ino(inode) || key.type_ != key_type {
            btrfs_end_transaction(trans); break;
        }
        ret = btrfs_del_items(trans, root, path, (*path).slots[0], 1);
        if ret != 0 { btrfs_end_transaction(trans); return ret; }
        count += 1;
        btrfs_release_path(path);
        btrfs_end_transaction(trans);
    }
    count
}

pub unsafe fn btrfs_drop_verity_items(inode: *mut btrfs_inode) -> c_int {
    let mut ret = drop_verity_items(inode, BTRFS_VERITY_DESC_ITEM_KEY);
    if ret < 0 { return ret; }
    ret = drop_verity_items(inode, BTRFS_VERITY_MERKLE_ITEM_KEY);
    if ret < 0 { return ret; }
    0
}

unsafe fn write_key_bytes(inode: *mut btrfs_inode, key_type: u8, mut offset: u64,
                          src: *const c_char, mut len: u64) -> c_int {
    let root = (*inode).root;
    let mut path = btrfs_alloc_path();
    let mut src_offset: usize = 0;
    let mut ret = 0;
    if path.is_null() { return -ENOMEM; }
    while len > 0 {
        let trans = btrfs_start_transaction(root, 1);
        if IS_ERR(trans) { return PTR_ERR(trans); }
        let key = btrfs_key { objectid: btrfs_ino(inode), type_: key_type, offset };
        let copy_bytes = core::cmp::min(len, 2048) as usize;
        ret = btrfs_insert_empty_item(trans, root, path, &key, copy_bytes);
        if ret != 0 { btrfs_end_transaction(trans); break; }
        let leaf = (*path).nodes[0];
        let data = btrfs_item_ptr(leaf, (*path).slots[0]);
        write_extent_buffer(leaf, src.add(src_offset), data as usize, copy_bytes);
        offset += copy_bytes as u64; src_offset += copy_bytes; len -= copy_bytes as u64;
        btrfs_release_path(path);
        btrfs_end_transaction(trans);
    }
    ret
}

unsafe fn read_key_bytes(inode: *mut btrfs_inode, key_type: u8, mut offset: u64,
                         dest: *mut c_char, mut len: u64, dest_folio: *mut folio) -> c_int {
    let root = (*inode).root;
    let mut path = btrfs_alloc_path();
    let mut key: btrfs_key = core::mem::zeroed();
    let mut copied: u64 = 0;
    let mut dest_offset: usize = 0;
    if path.is_null() { return -ENOMEM; }
    if !dest_folio.is_null() { (*path).reada = READA_FORWARD; }
    key.objectid = btrfs_ino(inode); key.type_ = key_type; key.offset = offset;
    let mut ret = btrfs_search_slot(core::ptr::null_mut(), root, &mut key, path, 0, 0);
    if ret < 0 { return ret; }
    if ret > 0 { if (*path).slots[0] == 0 { return 0; } (*path).slots[0] -= 1; }
    while len > 0 {
        let leaf = (*path).nodes[0];
        btrfs_item_key_to_cpu(leaf, &mut key, (*path).slots[0]);
        if key.objectid != btrfs_ino(inode) || key.type_ != key_type { break; }
        let item_end = btrfs_item_size(leaf, (*path).slots[0]) as u64 + key.offset;
        if copied > 0 { if key.offset != offset { break; } }
        else if key.offset > offset || item_end <= offset { break; }
        let copy_end = if dest.is_null() { item_end } else { core::cmp::min(offset + len, item_end) };
        let copy_bytes = (copy_end - offset) as usize;
        let copy_offset = (offset - key.offset) as usize;
        if !dest.is_null() {
            let kaddr = if !dest_folio.is_null() { kmap_local_folio(dest_folio, 0) } else { dest };
            let data = btrfs_item_ptr(leaf, (*path).slots[0]);
            read_extent_buffer(leaf, kaddr.add(dest_offset), (data as usize + copy_offset) as *const _, copy_bytes);
            if !dest_folio.is_null() { kunmap_local(kaddr); }
        }
        offset += copy_bytes as u64; dest_offset += copy_bytes; len -= copy_bytes as u64; copied += copy_bytes as u64;
        (*path).slots[0] += 1;
        if (*path).slots[0] >= btrfs_header_nritems((*path).nodes[0]) {
            ret = btrfs_next_leaf(root, path);
            if ret < 0 || ret > 0 { if ret > 0 { ret = 0; } break; }
        }
    }
    if ret == 0 { copied as c_int } else { ret }
}

unsafe fn del_orphan(trans: *mut btrfs_trans_handle, inode: *mut btrfs_inode) -> c_int {
    if (*inode).vfs_inode.i_nlink == 0 { return 0; }
    let mut ret = btrfs_del_orphan_item(trans, (*inode).root, btrfs_ino(inode));
    if ret == -ENOENT { ret = 0; } ret
}

unsafe fn rollback_verity(inode: *mut btrfs_inode) -> c_int {
    let root = (*inode).root; let mut trans = core::ptr::null_mut();
    btrfs_assert_inode_locked(inode);
    truncate_inode_pages((*inode).vfs_inode.i_mapping, (*inode).vfs_inode.i_size);
    clear_bit(BTRFS_INODE_VERITY_IN_PROGRESS, &mut (*inode).runtime_flags);
    let mut ret = btrfs_drop_verity_items(inode);
    if ret != 0 { btrfs_handle_fs_error((*root).fs_info, ret, c"failed to drop verity items in rollback %llu", (*inode).vfs_inode.i_ino); return ret; }
    trans = btrfs_start_transaction(root, 2);
    if IS_ERR(trans) { ret = PTR_ERR(trans); btrfs_handle_fs_error((*root).fs_info, ret, c"failed to start transaction in verity rollback %llu", (*inode).vfs_inode.i_ino); return ret; }
    (*inode).ro_flags &= !BTRFS_INODE_RO_VERITY;
    btrfs_sync_inode_flags_to_i_flags(inode); ret = btrfs_update_inode(trans, inode);
    if ret != 0 { btrfs_abort_transaction(trans, ret); btrfs_end_transaction(trans); return ret; }
    ret = del_orphan(trans, inode); if ret != 0 { btrfs_abort_transaction(trans, ret); }
    btrfs_end_transaction(trans); ret
}

unsafe fn finish_verity(inode: *mut btrfs_inode, desc: *const c_void, desc_size: usize) -> c_int {
    let root = (*inode).root; let mut item: btrfs_verity_descriptor_item = core::mem::zeroed();
    btrfs_set_stack_verity_descriptor_size(&mut item, desc_size);
    let mut ret = write_key_bytes(inode, BTRFS_VERITY_DESC_ITEM_KEY, 0, &item as *const _ as *const c_char, core::mem::size_of_val(&item) as u64);
    if ret != 0 { return ret; }
    ret = write_key_bytes(inode, BTRFS_VERITY_DESC_ITEM_KEY, 1, desc as *const c_char, desc_size as u64); if ret != 0 { return ret; }
    let trans = btrfs_start_transaction(root, 2); if IS_ERR(trans) { return PTR_ERR(trans); }
    (*inode).ro_flags |= BTRFS_INODE_RO_VERITY; btrfs_sync_inode_flags_to_i_flags(inode);
    ret = btrfs_update_inode(trans, inode); if ret == 0 { ret = del_orphan(trans, inode); }
    if ret == 0 { clear_bit(BTRFS_INODE_VERITY_IN_PROGRESS, &mut (*inode).runtime_flags); btrfs_set_fs_compat_ro((*root).fs_info, VERITY); }
    btrfs_end_transaction(trans); ret
}

unsafe fn btrfs_begin_enable_verity(filp: *mut file) -> c_int {
    let inode = BTRFS_I(file_inode(filp)); let root = (*inode).root;
    btrfs_assert_inode_locked(inode);
    if IS_ENCRYPTED(&(*inode).vfs_inode) { return -EOPNOTSUPP; }
    if test_bit(BTRFS_INODE_VERITY_IN_PROGRESS, &(*inode).runtime_flags) { return -EBUSY; }
    let ret = btrfs_drop_verity_items(inode); if ret != 0 { return ret; }
    let trans = btrfs_start_transaction(root, 1); if IS_ERR(trans) { return PTR_ERR(trans); }
    let ret = btrfs_orphan_add(trans, inode); if ret == 0 { set_bit(BTRFS_INODE_VERITY_IN_PROGRESS, &mut (*inode).runtime_flags); }
    btrfs_end_transaction(trans); 0
}

unsafe fn btrfs_end_enable_verity(filp: *mut file, desc: *const c_void, desc_size: usize, _merkle_tree_size: u64) -> c_int {
    let inode = BTRFS_I(file_inode(filp)); btrfs_assert_inode_locked(inode);
    let mut ret = 0;
    if !desc.is_null() { ret = finish_verity(inode, desc, desc_size); if ret == 0 { return ret; } }
    let rollback_ret = rollback_verity(inode);
    if rollback_ret != 0 { btrfs_err((*inode).root.fs_info, c"failed to rollback verity items: %pe", ERR_PTR(rollback_ret)); }
    ret
}

pub unsafe fn btrfs_get_verity_descriptor(inode: *mut inode, buf: *mut c_void, buf_size: usize) -> c_int {
    let mut item: btrfs_verity_descriptor_item = core::mem::zeroed();
    let ret = read_key_bytes(BTRFS_I(inode), BTRFS_VERITY_DESC_ITEM_KEY, 0, &mut item as *mut _ as *mut c_char, core::mem::size_of_val(&item) as u64, core::ptr::null_mut());
    if ret < 0 { return ret; }
    if item.reserved[0] != 0 || item.reserved[1] != 0 { return -EUCLEAN; }
    let true_size = btrfs_stack_verity_descriptor_size(&item); if true_size > INT_MAX as u64 { return -EUCLEAN; }
    if buf_size == 0 { return true_size as c_int; } if buf_size < true_size as usize { return -ERANGE; }
    let ret = read_key_bytes(BTRFS_I(inode), BTRFS_VERITY_DESC_ITEM_KEY, 1, buf as *mut c_char, buf_size as u64, core::ptr::null_mut());
    if ret < 0 { return ret; } if ret as u64 != true_size { return -EIO; } true_size as c_int
}

unsafe fn btrfs_read_merkle_tree_page(inode: *mut inode, mut index: pgoff_t) -> *mut page {
    let off = (index as u64) << PAGE_SHIFT; let merkle_pos = merkle_file_pos(inode); if merkle_pos < 0 { return ERR_PTR(merkle_pos); }
    if merkle_pos > (*(*inode).i_sb).s_maxbytes - off as loff_t - PAGE_SIZE as loff_t { return ERR_PTR(-EFBIG); }
    index += (merkle_pos as u64 >> PAGE_SHIFT) as pgoff_t;
    let folio = __filemap_get_folio((*inode).i_mapping, index, FGP_ACCESSED, 0); if IS_ERR(folio) { return ERR_PTR(PTR_ERR(folio)); }
    folio_lock(folio);
    let ret = read_key_bytes(BTRFS_I(inode), BTRFS_VERITY_MERKLE_ITEM_KEY, off, folio_address(folio), PAGE_SIZE as u64, folio);
    if ret < 0 { folio_unlock(folio); folio_put(folio); return ERR_PTR(ret); }
    if ret < PAGE_SIZE as c_int { folio_zero_segment(folio, ret as usize, PAGE_SIZE); }
    folio_mark_uptodate(folio); folio_unlock(folio); folio_file_page(folio, index)
}

unsafe fn btrfs_write_merkle_tree_block(file: *mut file, buf: *const c_void, pos: u64, size: u32) -> c_int {
    let inode = file_inode(file); let merkle_pos = merkle_file_pos(inode); if merkle_pos < 0 { return merkle_pos; }
    if merkle_pos > (*(*inode).i_sb).s_maxbytes - pos as loff_t - size as loff_t { return -EFBIG; }
    write_key_bytes(BTRFS_I(inode), BTRFS_VERITY_MERKLE_ITEM_KEY, pos, buf as *const c_char, size as u64)
}

pub static btrfs_verityops: fsverity_operations = fsverity_operations {
    begin_enable_verity: Some(btrfs_begin_enable_verity), end_enable_verity: Some(btrfs_end_enable_verity),
    get_verity_descriptor: Some(btrfs_get_verity_descriptor), read_merkle_tree_page: Some(btrfs_read_merkle_tree_page),
    write_merkle_tree_block: Some(btrfs_write_merkle_tree_block),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
