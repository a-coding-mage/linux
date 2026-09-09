// SPDX-License-Identifier: GPL-2.0
/* Translation of linux/fs/hfsplus/catalog.c. */

pub unsafe fn hfsplus_cat_case_cmp_key(k1: *const hfsplus_btree_key, k2: *const hfsplus_btree_key) -> i32 {
    let k1p = (*k1).cat.parent;
    let k2p = (*k2).cat.parent;
    if k1p != k2p { return if be32_to_cpu(k1p) < be32_to_cpu(k2p) { -1 } else { 1 }; }
    hfsplus_strcasecmp(&(*k1).cat.name, &(*k2).cat.name)
}

pub unsafe fn hfsplus_cat_bin_cmp_key(k1: *const hfsplus_btree_key, k2: *const hfsplus_btree_key) -> i32 {
    let k1p = (*k1).cat.parent;
    let k2p = (*k2).cat.parent;
    if k1p != k2p { return if be32_to_cpu(k1p) < be32_to_cpu(k2p) { -1 } else { 1 }; }
    hfsplus_strcmp(&(*k1).cat.name, &(*k2).cat.name)
}

/* Generates key for catalog file/folders record. */
pub unsafe fn hfsplus_cat_build_key(sb: *mut super_block, key: *mut hfsplus_btree_key, parent: u32, str_: *const qstr) -> i32 {
    (*key).cat.parent = cpu_to_be32(parent);
    let err = hfsplus_asc2uni(sb, &mut (*key).cat.name, HFSPLUS_MAX_STRLEN, (*str_).name, (*str_).len, HFS_REGULAR_NAME);
    if err < 0 { return err; }
    let len = be16_to_cpu((*key).cat.name.length);
    (*key).key_len = cpu_to_be16(6 + 2 * len);
    0
}

/* Generates key for catalog thread record. */
pub unsafe fn hfsplus_cat_build_key_with_cnid(_sb: *mut super_block, key: *mut hfsplus_btree_key, parent: u32) {
    (*key).cat.parent = cpu_to_be32(parent);
    (*key).cat.name.length = 0;
    (*key).key_len = cpu_to_be16(6);
}

unsafe fn hfsplus_cat_build_key_uni(key: *mut hfsplus_btree_key, parent: u32, name: *mut hfsplus_unistr) {
    let mut ustrlen = be16_to_cpu((*name).length);
    (*key).cat.parent = cpu_to_be32(parent);
    (*key).cat.name.length = cpu_to_be16(ustrlen);
    ustrlen *= 2;
    memcpy((*key).cat.name.unicode.as_mut_ptr(), (*name).unicode.as_ptr(), ustrlen as usize);
    (*key).key_len = cpu_to_be16(6 + ustrlen);
}

pub unsafe fn hfsplus_cat_set_perms(inode: *mut inode, perms: *mut hfsplus_perm) {
    if (*inode).i_flags & S_IMMUTABLE != 0 { (*perms).rootflags |= HFSPLUS_FLG_IMMUTABLE; } else { (*perms).rootflags &= !HFSPLUS_FLG_IMMUTABLE; }
    if (*inode).i_flags & S_APPEND != 0 { (*perms).rootflags |= HFSPLUS_FLG_APPEND; } else { (*perms).rootflags &= !HFSPLUS_FLG_APPEND; }
    (*perms).userflags = HFSPLUS_I(inode).userflags;
    (*perms).mode = cpu_to_be16((*inode).i_mode);
    (*perms).owner = cpu_to_be32(i_uid_read(inode));
    (*perms).group = cpu_to_be32(i_gid_read(inode));
    if S_ISREG((*inode).i_mode) { (*perms).dev = cpu_to_be32((*inode).i_nlink); }
    else if S_ISBLK((*inode).i_mode) || S_ISCHR((*inode).i_mode) { (*perms).dev = cpu_to_be32((*inode).i_rdev); }
    else { (*perms).dev = 0; }
}

unsafe fn hfsplus_cat_build_record(entry: *mut hfsplus_cat_entry, cnid: u32, inode: *mut inode) -> i32 {
    let sbi = HFSPLUS_SB((*inode).i_sb);
    if S_ISDIR((*inode).i_mode) {
        let folder = &mut (*entry).folder;
        memset(folder as *mut _ as *mut core::ffi::c_void, 0, core::mem::size_of_val(folder));
        folder.type_ = cpu_to_be16(HFSPLUS_FOLDER);
        if test_bit(HFSPLUS_SB_HFSX, &sbi.flags) { folder.flags |= cpu_to_be16(HFSPLUS_HAS_FOLDER_COUNT); }
        folder.id = cpu_to_be32((*inode).i_ino);
        let now = hfsp_now2mt(); HFSPLUS_I(inode).create_date = now; folder.create_date = now; folder.content_mod_date = now; folder.attribute_mod_date = now; folder.access_date = now;
        hfsplus_cat_set_perms(inode, &mut folder.permissions);
        if inode == sbi.hidden_dir { folder.user_info.frFlags = cpu_to_be16(0x5000); }
        core::mem::size_of::<hfsplus_cat_folder>() as i32
    } else {
        let file = &mut (*entry).file;
        memset(file as *mut _ as *mut core::ffi::c_void, 0, core::mem::size_of_val(file));
        file.type_ = cpu_to_be16(HFSPLUS_FILE); file.flags = cpu_to_be16(HFSPLUS_FILE_THREAD_EXISTS); file.id = cpu_to_be32(cnid);
        let now = hfsp_now2mt(); HFSPLUS_I(inode).create_date = now; file.create_date = now; file.content_mod_date = now; file.attribute_mod_date = now; file.access_date = now;
        if cnid == (*inode).i_ino {
            hfsplus_cat_set_perms(inode, &mut file.permissions);
            if S_ISLNK((*inode).i_mode) { file.user_info.fdType = cpu_to_be32(HFSP_SYMLINK_TYPE); file.user_info.fdCreator = cpu_to_be32(HFSP_SYMLINK_CREATOR); } else { file.user_info.fdType = cpu_to_be32(sbi.type_); file.user_info.fdCreator = cpu_to_be32(sbi.creator); }
            if HFSPLUS_FLG_IMMUTABLE & (file.permissions.rootflags | file.permissions.userflags) != 0 { file.flags |= cpu_to_be16(HFSPLUS_FILE_LOCKED); }
        } else {
            file.user_info.fdType = cpu_to_be32(HFSP_HARDLINK_TYPE); file.user_info.fdCreator = cpu_to_be32(HFSP_HFSPLUS_CREATOR); file.user_info.fdFlags = cpu_to_be16(0x100);
            file.create_date = HFSPLUS_I(sbi.hidden_dir).create_date; file.permissions.dev = cpu_to_be32(HFSPLUS_I(inode).linkid);
        }
        core::mem::size_of::<hfsplus_cat_file>() as i32
    }
}

unsafe fn hfsplus_fill_cat_thread(sb: *mut super_block, entry: *mut hfsplus_cat_entry, type_: i32, parentid: u32, str_: *const qstr) -> i32 {
    (*entry).type_ = cpu_to_be16(type_ as u16); (*entry).thread.reserved = 0; (*entry).thread.parentID = cpu_to_be32(parentid);
    let err = hfsplus_asc2uni(sb, &mut (*entry).thread.nodeName, HFSPLUS_MAX_STRLEN, (*str_).name, (*str_).len, HFS_REGULAR_NAME);
    if err < 0 { return err; }
    10 + be16_to_cpu((*entry).thread.nodeName.length) as i32 * 2
}

unsafe fn hfsplus_subfolders_inc(dir: *mut inode) { let sbi = HFSPLUS_SB((*dir).i_sb); if test_bit(HFSPLUS_SB_HFSX, &sbi.flags) { HFSPLUS_I(dir).subfolders += 1; } }
unsafe fn hfsplus_subfolders_dec(dir: *mut inode) { let sbi = HFSPLUS_SB((*dir).i_sb); if test_bit(HFSPLUS_SB_HFSX, &sbi.flags) && HFSPLUS_I(dir).subfolders != 0 { HFSPLUS_I(dir).subfolders -= 1; } }

pub unsafe fn hfsplus_find_cat(sb: *mut super_block, cnid: u32, fd: *mut hfs_find_data) -> i32 {
    let mut tmp: hfsplus_cat_entry = core::mem::zeroed();
    hfsplus_cat_build_key_with_cnid(sb, (*fd).search_key, cnid);
    let err = hfsplus_brec_read_cat(fd, &mut tmp);
    if err != 0 { return err; }
    let type_ = be16_to_cpu(tmp.type_);
    if !is_hfs_thread_record_type(type_) || be16_to_cpu(tmp.thread.nodeName.length) > 255 { return -EIO; }
    hfsplus_cat_build_key_uni((*fd).search_key, be32_to_cpu(tmp.thread.parentID), &mut tmp.thread.nodeName);
    hfs_brec_find(fd, hfs_find_rec_by_key)
}

pub unsafe fn hfsplus_create_cat(cnid: u32, dir: *mut inode, str_: *const qstr, inode: *mut inode) -> i32 {
    let sb = (*dir).i_sb; let mut fd: hfs_find_data = core::mem::zeroed(); let mut entry: hfsplus_cat_entry = core::mem::zeroed();
    let mut err = hfs_find_init(HFSPLUS_SB(sb).cat_tree, &mut fd); if err != 0 { return err; }
    err = hfs_bmap_reserve(fd.tree, 2 * (*fd.tree).depth); if err != 0 { hfs_find_exit(&mut fd); return err; }
    hfsplus_cat_build_key_with_cnid(sb, fd.search_key, cnid);
    let mut entry_size = hfsplus_fill_cat_thread(sb, &mut entry, if S_ISDIR((*inode).i_mode) { HFSPLUS_FOLDER_THREAD } else { HFSPLUS_FILE_THREAD }, (*dir).i_ino, str_);
    if entry_size < 0 { hfs_find_exit(&mut fd); return entry_size; }
    err = hfs_brec_find(&mut fd, hfs_find_rec_by_key); if err != -ENOENT { if err == 0 { err = -EEXIST; } hfs_find_exit(&mut fd); return err; }
    err = hfs_brec_insert(&mut fd, &entry, entry_size); if err != 0 { hfs_find_exit(&mut fd); return err; }
    err = hfsplus_cat_build_key(sb, fd.search_key, (*dir).i_ino, str_); if err != 0 { hfs_find_exit(&mut fd); return err; }
    entry_size = hfsplus_cat_build_record(&mut entry, cnid, inode);
    err = hfs_brec_find(&mut fd, hfs_find_rec_by_key); if err != -ENOENT { if err == 0 { err = -EEXIST; } hfs_find_exit(&mut fd); return err; }
    err = hfs_brec_insert(&mut fd, &entry, entry_size); if err != 0 { hfs_find_exit(&mut fd); return err; }
    (*dir).i_size += 1; if S_ISDIR((*inode).i_mode) { hfsplus_subfolders_inc(dir); }
    inode_set_mtime_to_ts(dir, inode_set_ctime_current(dir)); hfsplus_mark_inode_dirty(HFSPLUS_CAT_TREE_I(sb), HFSPLUS_I_CAT_DIRTY); hfsplus_mark_inode_dirty(dir, HFSPLUS_I_CAT_DIRTY);
    hfs_find_exit(&mut fd); 0
}

pub unsafe fn hfsplus_delete_cat(cnid: u32, dir: *mut inode, str_: *const qstr) -> i32 {
    let sb = (*dir).i_sb; let mut fd: hfs_find_data = core::mem::zeroed(); let mut err = hfs_find_init(HFSPLUS_SB(sb).cat_tree, &mut fd); if err != 0 { return err; }
    err = hfs_bmap_reserve(fd.tree, 2 * (*fd.tree).depth - 2); if err != 0 { hfs_find_exit(&mut fd); return err; }
    if str_.is_null() { let mut entry: hfsplus_cat_entry = core::mem::zeroed(); hfsplus_cat_build_key_with_cnid(sb, fd.search_key, cnid); err = hfsplus_brec_read_cat(&mut fd, &mut entry); if err != 0 { hfs_find_exit(&mut fd); return err; } hfsplus_cat_build_key_uni(fd.search_key, (*dir).i_ino, &mut entry.thread.nodeName); } else { err = hfsplus_cat_build_key(sb, fd.search_key, (*dir).i_ino, str_); if err != 0 { hfs_find_exit(&mut fd); return err; } }
    err = hfs_brec_find(&mut fd, hfs_find_rec_by_key); if err != 0 { hfs_find_exit(&mut fd); return err; }
    let type_ = hfs_bnode_read_u16(fd.bnode, fd.entryoffset); err = hfs_brec_remove(&mut fd); if err != 0 { hfs_find_exit(&mut fd); return err; }
    hfsplus_cat_build_key_with_cnid(sb, fd.search_key, cnid); err = hfs_brec_find(&mut fd, hfs_find_rec_by_key); if err == 0 { err = hfs_brec_remove(&mut fd); }
    if err == 0 { (*dir).i_size -= 1; if type_ == HFSPLUS_FOLDER { hfsplus_subfolders_dec(dir); } inode_set_mtime_to_ts(dir, inode_set_ctime_current(dir)); hfsplus_mark_inode_dirty(HFSPLUS_CAT_TREE_I(sb), HFSPLUS_I_CAT_DIRTY); hfsplus_mark_inode_dirty(dir, HFSPLUS_I_CAT_DIRTY); }
    hfs_find_exit(&mut fd); err
}

pub unsafe fn hfsplus_rename_cat(cnid: u32, src_dir: *mut inode, src_name: *const qstr, dst_dir: *mut inode, dst_name: *const qstr) -> i32 {
    let sb = (*src_dir).i_sb; let mut src_fd: hfs_find_data = core::mem::zeroed(); let mut dst_fd: hfs_find_data = core::mem::zeroed(); let mut entry: hfsplus_cat_entry = core::mem::zeroed();
    let mut err = hfs_find_init(HFSPLUS_SB(sb).cat_tree, &mut src_fd); if err != 0 { return err; }
    core::ptr::copy_nonoverlapping(&src_fd, &mut dst_fd, 1);
    err = hfs_bmap_reserve(src_fd.tree, 4 * (*src_fd.tree).depth - 1); if err != 0 { hfs_bnode_put(dst_fd.bnode); hfs_find_exit(&mut src_fd); return err; }
    err = hfsplus_cat_build_key(sb, src_fd.search_key, (*src_dir).i_ino, src_name); if err != 0 { hfs_bnode_put(dst_fd.bnode); hfs_find_exit(&mut src_fd); return err; }
    err = hfs_brec_find(&mut src_fd, hfs_find_rec_by_key); if err != 0 { hfs_bnode_put(dst_fd.bnode); hfs_find_exit(&mut src_fd); return err; }
    if src_fd.entrylength > core::mem::size_of::<hfsplus_cat_entry>() as i32 || src_fd.entrylength < 0 { hfs_bnode_put(dst_fd.bnode); hfs_find_exit(&mut src_fd); return -EIO; }
    hfs_bnode_read(src_fd.bnode, &mut entry, src_fd.entryoffset, src_fd.entrylength); let type_ = be16_to_cpu(entry.type_);
    err = hfsplus_cat_build_key(sb, dst_fd.search_key, (*dst_dir).i_ino, dst_name); if err != 0 { hfs_bnode_put(dst_fd.bnode); hfs_find_exit(&mut src_fd); return err; }
    err = hfs_brec_find(&mut dst_fd, hfs_find_rec_by_key); if err != -ENOENT { if err == 0 { err = -EEXIST; } hfs_bnode_put(dst_fd.bnode); hfs_find_exit(&mut src_fd); return err; }
    err = hfs_brec_insert(&mut dst_fd, &entry, src_fd.entrylength); if err != 0 { hfs_bnode_put(dst_fd.bnode); hfs_find_exit(&mut src_fd); return err; }
    (*dst_dir).i_size += 1; if type_ == HFSPLUS_FOLDER { hfsplus_subfolders_inc(dst_dir); } inode_set_mtime_to_ts(dst_dir, inode_set_ctime_current(dst_dir));
    err = hfsplus_cat_build_key(sb, src_fd.search_key, (*src_dir).i_ino, src_name); if err == 0 { err = hfs_brec_find(&mut src_fd, hfs_find_rec_by_key); } if err == 0 { err = hfs_brec_remove(&mut src_fd); }
    if err == 0 { (*src_dir).i_size -= 1; if type_ == HFSPLUS_FOLDER { hfsplus_subfolders_dec(src_dir); } inode_set_mtime_to_ts(src_dir, inode_set_ctime_current(src_dir)); hfsplus_cat_build_key_with_cnid(sb, src_fd.search_key, cnid); err = hfs_brec_find(&mut src_fd, hfs_find_rec_by_key); if err == 0 { type_ = hfs_bnode_read_u16(src_fd.bnode, src_fd.entryoffset); err = hfs_brec_remove(&mut src_fd); } }
    if err == 0 { hfsplus_cat_build_key_with_cnid(sb, dst_fd.search_key, cnid); let n = hfsplus_fill_cat_thread(sb, &mut entry, type_ as i32, (*dst_dir).i_ino, dst_name); if n < 0 { err = n; } else { err = hfs_brec_find(&mut dst_fd, hfs_find_rec_by_key); if err == -ENOENT { err = hfs_brec_insert(&mut dst_fd, &entry, n); } else if err == 0 { err = -EEXIST; } } }
    hfsplus_mark_inode_dirty(HFSPLUS_CAT_TREE_I(sb), HFSPLUS_I_CAT_DIRTY); hfsplus_mark_inode_dirty(dst_dir, HFSPLUS_I_CAT_DIRTY); hfsplus_mark_inode_dirty(src_dir, HFSPLUS_I_CAT_DIRTY);
    hfs_bnode_put(dst_fd.bnode); hfs_find_exit(&mut src_fd); err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
