/*
 *  linux/fs/hfs/catalog.c
 *
 * Copyright (C) 1995-1997  Paul H. Hargrove
 * (C) 2003 Ardis Technologies <roman@ardistech.com>
 * This file may be distributed under the terms of the GNU General Public License.
 *
 * This file contains the functions related to the catalog B-tree.
 *
 * Cache code shamelessly stolen from
 *     linux/fs/inode.c Copyright (C) 1991, 1992  Linus Torvalds
 *     re-shamelessly stolen Copyright (C) 1997 Linus Torvalds
 */

// Dependencies supplied by the surrounding HFS/kernel translation.

pub unsafe fn hfs_cat_build_key(sb: *mut super_block, key: *mut btree_key, parent: u32, name: *const qstr) {
    (*key).cat.reserved = 0;
    (*key).cat.ParID = cpu_to_be32(parent);
    if !name.is_null() {
        hfs_asc2mac(sb, &mut (*key).cat.CName, name);
        (*key).key_len = 6 + (*key).cat.CName.len;
    } else {
        core::ptr::write_bytes(
            &mut (*key).cat.CName as *mut hfs_name as *mut u8,
            0,
            core::mem::size_of::<hfs_name>(),
        );
        (*key).key_len = 6;
    }
}

unsafe fn hfs_cat_build_record(rec: *mut hfs_cat_rec, cnid: u32, inode: *mut inode) -> i32 {
    let mtime: __be32 = hfs_mtime();

    core::ptr::write_bytes(rec as *mut u8, 0, core::mem::size_of::<hfs_cat_rec>());
    if S_ISDIR((*inode).i_mode) {
        (*rec).type_ = HFS_CDR_DIR;
        (*rec).dir.DirID = cpu_to_be32(cnid);
        (*rec).dir.CrDat = mtime;
        (*rec).dir.MdDat = mtime;
        (*rec).dir.BkDat = 0;
        (*rec).dir.UsrInfo.frView = cpu_to_be16(0xff);
        core::mem::size_of::<hfs_cat_dir>() as i32
    } else {
        /* init some fields for the file record */
        (*rec).type_ = HFS_CDR_FIL;
        (*rec).file.Flags = HFS_FIL_USED | HFS_FIL_THD;
        if ((*inode).i_mode & S_IWUSR) == 0 {
            (*rec).file.Flags |= HFS_FIL_LOCK;
        }
        (*rec).file.FlNum = cpu_to_be32(cnid);
        (*rec).file.CrDat = mtime;
        (*rec).file.MdDat = mtime;
        (*rec).file.BkDat = 0;
        (*rec).file.UsrWds.fdType = HFS_SB((*inode).i_sb).s_type;
        (*rec).file.UsrWds.fdCreator = HFS_SB((*inode).i_sb).s_creator;
        core::mem::size_of::<hfs_cat_file>() as i32
    }
}

unsafe fn hfs_cat_build_thread(
    sb: *mut super_block,
    rec: *mut hfs_cat_rec,
    type_: i32,
    parentid: u32,
    name: *const qstr,
) -> i32 {
    (*rec).type_ = type_;
    core::ptr::write_bytes(
        (*rec).thread.reserved.as_mut_ptr(),
        0,
        core::mem::size_of_val(&(*rec).thread.reserved),
    );
    (*rec).thread.ParID = cpu_to_be32(parentid);
    hfs_asc2mac(sb, &mut (*rec).thread.CName, name);
    core::mem::size_of::<hfs_cat_thread>() as i32
}

pub unsafe fn hfs_cat_create(cnid: u32, dir: *mut inode, str_: *const qstr, inode_: *mut inode) -> i32 {
    let mut fd: hfs_find_data = core::mem::zeroed();
    let sb: *mut super_block;
    let mut entry: hfs_cat_rec = core::mem::zeroed();
    let mut entry_size: i32;
    let mut err: i32;

    hfs_dbg!("name %s, cnid %u, i_nlink %d\n", (*str_).name, cnid, (*inode_).i_nlink);
    if (*dir).i_size >= HFS_MAX_VALENCE { return -ENOSPC; }
    sb = (*dir).i_sb;
    err = hfs_find_init(HFS_SB((*sb).s_fs_info).cat_tree, &mut fd);
    if err != 0 { return err; }
    err = hfs_bmap_reserve((*fd.tree).depth * 2);
    if err != 0 { hfs_find_exit(&mut fd); return err; }

    hfs_cat_build_key(sb, fd.search_key, cnid, core::ptr::null());
    entry_size = hfs_cat_build_thread(sb, &mut entry, if S_ISDIR((*inode_).i_mode) { HFS_CDR_THD } else { HFS_CDR_FTH }, (*dir).i_ino, str_);
    err = hfs_brec_find(&mut fd);
    if err != -ENOENT { if err == 0 { err = -EEXIST; } hfs_find_exit(&mut fd); return err; }
    err = hfs_brec_insert(&mut fd, &mut entry, entry_size);
    if err != 0 { hfs_find_exit(&mut fd); return err; }

    hfs_cat_build_key(sb, fd.search_key, (*dir).i_ino, str_);
    entry_size = hfs_cat_build_record(&mut entry, cnid, inode_);
    err = hfs_brec_find(&mut fd);
    if err != -ENOENT { if err == 0 { err = -EEXIST; } hfs_cat_build_key(sb, fd.search_key, cnid, core::ptr::null()); if hfs_brec_find(&mut fd) == 0 { hfs_brec_remove(&mut fd); } hfs_find_exit(&mut fd); return err; }
    err = hfs_brec_insert(&mut fd, &mut entry, entry_size);
    if err != 0 { hfs_cat_build_key(sb, fd.search_key, cnid, core::ptr::null()); if hfs_brec_find(&mut fd) == 0 { hfs_brec_remove(&mut fd); } hfs_find_exit(&mut fd); return err; }

    (*dir).i_size += 1;
    inode_set_mtime_to_ts(dir, inode_set_ctime_current(dir));
    mark_inode_dirty(dir);
    hfs_find_exit(&mut fd);
    0
}

pub unsafe fn hfs_cat_keycmp(key1: *const btree_key, key2: *const btree_key) -> i32 {
    let k1p = (*key1).cat.ParID;
    let k2p = (*key2).cat.ParID;
    if k1p != k2p { return if be32_to_cpu(k1p) < be32_to_cpu(k2p) { -1 } else { 1 }; }
    hfs_strcmp((*key1).cat.CName.name.as_ptr(), (*key1).cat.CName.len, (*key2).cat.CName.name.as_ptr(), (*key2).cat.CName.len)
}

unsafe fn hfs_cat_validate_found_cnid(fd: *mut hfs_find_data, cnid: u32) -> i32 {
    let mut rec: hfs_cat_rec = core::mem::zeroed();
    let rec_len = (*fd).entrylength;
    if rec_len <= 0 || rec_len as usize > core::mem::size_of::<hfs_cat_rec>() { return -EIO; }
    hfs_bnode_read((*fd).bnode, &mut rec, (*fd).entryoffset, rec_len);
    let found_cnid = match rec.type_ {
        HFS_CDR_FIL => { if rec_len as usize != core::mem::size_of::<hfs_cat_file>() { return -EIO; } be32_to_cpu(rec.file.FlNum) },
        HFS_CDR_DIR => { if rec_len as usize != core::mem::size_of::<hfs_cat_dir>() { return -EIO; } be32_to_cpu(rec.dir.DirID) },
        _ => return -EIO,
    };
    if found_cnid != cnid { -EIO } else { 0 }
}

pub unsafe fn hfs_cat_find_brec(sb: *mut super_block, cnid: u32, fd: *mut hfs_find_data) -> i32 {
    let mut rec: hfs_cat_rec = core::mem::zeroed();
    hfs_cat_build_key(sb, (*fd).search_key, cnid, core::ptr::null());
    let mut res = hfs_brec_read(fd, &mut rec, core::mem::size_of::<hfs_cat_rec>() as i32);
    if res != 0 { return res; }
    let type_ = rec.type_;
    if type_ != HFS_CDR_THD && type_ != HFS_CDR_FTH { pr_err!("found bad thread record in catalog\n"); return -EIO; }
    (*fd).search_key.cat.ParID = rec.thread.ParID;
    let len = rec.thread.CName.len;
    (*fd).search_key.cat.CName.len = len;
    if len > HFS_NAMELEN { pr_err!("bad catalog namelength\n"); return -EIO; }
    core::ptr::copy_nonoverlapping(rec.thread.CName.name.as_ptr(), (*fd).search_key.cat.CName.name.as_mut_ptr(), len as usize);
    res = hfs_brec_find(fd);
    if res != 0 { return res; }
    hfs_cat_validate_found_cnid(fd, cnid)
}

#[inline]
unsafe fn hfs_set_next_unused_CNID(sb: *mut super_block, deleted_cnid: u32, found_cnid: u32) {
    atomic64_cmpxchg(&mut HFS_SB((*sb).s_fs_info).next_id, deleted_cnid as i64 + 1, if found_cnid < HFS_FIRSTUSER_CNID { HFS_FIRSTUSER_CNID as i64 } else { found_cnid as i64 + 1 });
}

unsafe fn hfs_correct_next_unused_CNID(sb: *mut super_block, cnid: u32) -> i32 {
    if (cnid + 1) as i64 < atomic64_read(&HFS_SB((*sb).s_fs_info).next_id) { return 0; }
    let cat_tree = HFS_SB((*sb).s_fs_info).cat_tree;
    let leaf_head = (*cat_tree).leaf_head;
    let leaf_tail = (*cat_tree).leaf_tail;
    if leaf_head > leaf_tail { pr_err!("node is corrupted: leaf_head %lld, leaf_tail %lld\n", leaf_head, leaf_tail); return -ERANGE; }
    let mut node_id = leaf_tail;
    let mut node = hfs_bnode_find(cat_tree, node_id);
    if IS_ERR(node) { return -ENOENT; }
    loop {
        if node_id != leaf_tail { node = hfs_bnode_find(cat_tree, node_id); if IS_ERR(node) { return -ENOENT; } }
        for i in (0..(*node).num_recs as i32).rev() {
            let mut rec: hfs_cat_rec = core::mem::zeroed(); let mut off: u16 = 0;
            let len = hfs_brec_lenoff(node, i, &mut off); let keylen = hfs_brec_keylen(node, i);
            if keylen == 0 { return -EINVAL; }
            let entryoffset = off as i32 + keylen as i32; let entrylength = len as i32 - keylen as i32;
            if entrylength > core::mem::size_of::<hfs_cat_rec>() as i32 { return -EINVAL; }
            hfs_bnode_read(node, &mut rec, entryoffset, entrylength);
            if rec.type_ == HFS_CDR_DIR { hfs_set_next_unused_CNID(sb, cnid, be32_to_cpu(rec.dir.DirID)); hfs_bnode_put(node); return 0; }
            if rec.type_ == HFS_CDR_FIL { hfs_set_next_unused_CNID(sb, cnid, be32_to_cpu(rec.file.FlNum)); hfs_bnode_put(node); return 0; }
        }
        node_id = (*node).prev; hfs_bnode_put(node);
        if node_id < leaf_head { break; }
    }
    -ENOENT
}

pub unsafe fn hfs_cat_delete(cnid: u32, dir: *mut inode, str_: *const qstr) -> i32 {
    let sb = (*dir).i_sb; let mut fd: hfs_find_data = core::mem::zeroed(); let mut res = hfs_find_init(HFS_SB((*sb).s_fs_info).cat_tree, &mut fd); if res != 0 { return res; }
    hfs_cat_build_key(sb, fd.search_key, (*dir).i_ino, str_); res = hfs_brec_find(&mut fd); if res != 0 { hfs_find_exit(&mut fd); return res; }
    let type_ = hfs_bnode_read_u8(fd.bnode, fd.entryoffset);
    if type_ == HFS_CDR_FIL { let mut file: hfs_cat_file = core::mem::zeroed(); hfs_bnode_read(fd.bnode, &mut file, fd.entryoffset, core::mem::size_of::<hfs_cat_file>() as i32); if be32_to_cpu(file.FlNum) == cnid { hfs_free_fork(sb, &mut file, HFS_FK_RSRC); } }
    res = hfs_brec_remove(&mut fd); if res != 0 { hfs_find_exit(&mut fd); return res; }
    hfs_cat_build_key(sb, fd.search_key, cnid, core::ptr::null()); res = hfs_brec_find(&mut fd); if res == 0 { res = hfs_brec_remove(&mut fd); if res != 0 { hfs_find_exit(&mut fd); return res; } }
    (*dir).i_size -= 1; inode_set_mtime_to_ts(dir, inode_set_ctime_current(dir)); mark_inode_dirty(dir);
    res = hfs_correct_next_unused_CNID(sb, cnid); hfs_find_exit(&mut fd); res
}

pub unsafe fn hfs_cat_move(cnid: u32, src_dir: *mut inode, src_name: *const qstr, dst_dir: *mut inode, dst_name: *const qstr) -> i32 {
    let sb = (*src_dir).i_sb; let mut src_fd: hfs_find_data = core::mem::zeroed(); let mut dst_fd: hfs_find_data; let mut entry: hfs_cat_rec = core::mem::zeroed(); let mut err = hfs_find_init(HFS_SB((*sb).s_fs_info).cat_tree, &mut src_fd); if err != 0 { return err; } dst_fd = src_fd;
    err = hfs_bmap_reserve((*src_fd.tree).depth * 2); if err != 0 { hfs_bnode_put(dst_fd.bnode); hfs_find_exit(&mut src_fd); return err; }
    hfs_cat_build_key(sb, src_fd.search_key, (*src_dir).i_ino, src_name); err = hfs_brec_find(&mut src_fd); if err != 0 { hfs_bnode_put(dst_fd.bnode); hfs_find_exit(&mut src_fd); return err; }
    if src_fd.entrylength > core::mem::size_of::<hfs_cat_rec>() as i32 || src_fd.entrylength < 0 { hfs_bnode_put(dst_fd.bnode); hfs_find_exit(&mut src_fd); return -EIO; }
    hfs_bnode_read(src_fd.bnode, &mut entry, src_fd.entryoffset, src_fd.entrylength);
    hfs_cat_build_key(sb, dst_fd.search_key, (*dst_dir).i_ino, dst_name); err = hfs_brec_find(&mut dst_fd); if err != -ENOENT { if err == 0 { err = -EEXIST; } hfs_bnode_put(dst_fd.bnode); hfs_find_exit(&mut src_fd); return err; }
    err = hfs_brec_insert(&mut dst_fd, &mut entry, src_fd.entrylength); if err != 0 { hfs_bnode_put(dst_fd.bnode); hfs_find_exit(&mut src_fd); return err; }
    (*dst_dir).i_size += 1; inode_set_mtime_to_ts(dst_dir, inode_set_ctime_current(dst_dir)); mark_inode_dirty(dst_dir);
    hfs_cat_build_key(sb, src_fd.search_key, (*src_dir).i_ino, src_name); err = hfs_brec_find(&mut src_fd); if err == 0 { err = hfs_brec_remove(&mut src_fd); } if err != 0 { hfs_bnode_put(dst_fd.bnode); hfs_find_exit(&mut src_fd); return err; }
    (*src_dir).i_size -= 1; inode_set_mtime_to_ts(src_dir, inode_set_ctime_current(src_dir)); mark_inode_dirty(src_dir);
    let type_ = entry.type_; if type_ == HFS_CDR_FIL && (entry.file.Flags & HFS_FIL_THD) == 0 { hfs_bnode_put(dst_fd.bnode); hfs_find_exit(&mut src_fd); return 0; }
    hfs_cat_build_key(sb, src_fd.search_key, cnid, core::ptr::null()); err = hfs_brec_find(&mut src_fd); if err == 0 { err = hfs_brec_remove(&mut src_fd); } if err != 0 { hfs_bnode_put(dst_fd.bnode); hfs_find_exit(&mut src_fd); return err; }
    hfs_cat_build_key(sb, dst_fd.search_key, cnid, core::ptr::null()); let entry_size = hfs_cat_build_thread(sb, &mut entry, if type_ == HFS_CDR_FIL { HFS_CDR_FTH } else { HFS_CDR_THD }, (*dst_dir).i_ino, dst_name); err = hfs_brec_find(&mut dst_fd); if err != -ENOENT { if err == 0 { err = -EEXIST; } } else { err = hfs_brec_insert(&mut dst_fd, &mut entry, entry_size); }
    hfs_bnode_put(dst_fd.bnode); hfs_find_exit(&mut src_fd); err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
