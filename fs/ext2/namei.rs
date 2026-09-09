// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/ext2/namei.c
 *
 * Rewrite to pagecache. Almost all code had been changed, so blame me
 * if the things go wrong. Please, send bug reports to
 * viro@parcelfarce.linux.theplanet.co.uk
 *
 * Stuff here is basically a glue between the VFS and generic UNIXish
 * filesystem that keeps everything in pagecache. All knowledge of the
 * directory layout is in fs/ext2/dir.c - it turned out to be easily separatable
 * and it's easier to debug that way. In principle we might want to
 * generalize that a bit and turn it into a library. Or not.
 *
 * The only non-static object here is ext2_dir_inode_operations.
 *
 * Copyright (C) 1992, 1993, 1994, 1995
 * Remy Card (card@masi.ibp.fr)
 * Laboratoire MASI - Institut Blaise Pascal
 * Universite Pierre et Marie Curie (Paris VI)
 *
 *  from
 *
 *  linux/fs/minix/namei.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *
 *  Big-endian to little-endian byte-swapping/bitmaps by
 *        David S. Miller (davem@caip.rutgers.edu), 1995
 */

// Dependencies are supplied by the surrounding kernel translation unit.

unsafe fn ext2_add_nondir(dentry: *mut dentry, inode: *mut inode) -> i32 {
    let err = ext2_add_link(dentry, inode);
    if err == 0 {
        d_instantiate_new(dentry, inode);
        return 0;
    }
    inode_dec_link_count(inode);
    discard_new_inode(inode);
    err
}

/* Methods themselves. */

unsafe fn ext2_lookup(dir: *mut inode, dentry: *mut dentry, _flags: u32) -> *mut dentry {
    let mut inode: *mut inode;
    let mut ino: ino_t = 0;
    let res: i32;

    if (*dentry).d_name.len > EXT2_NAME_LEN {
        return ERR_PTR(-ENAMETOOLONG);
    }

    res = ext2_inode_by_name(dir, &(*dentry).d_name, &mut ino);
    if res != 0 {
        if res != -ENOENT {
            return ERR_PTR(res);
        }
        inode = core::ptr::null_mut();
    } else {
        inode = ext2_iget((*dir).i_sb, ino);
        if inode == ERR_PTR(-ESTALE) {
            ext2_error((*dir).i_sb, __func__, "deleted inode referenced: %lu", ino as c_ulong);
            return ERR_PTR(-EIO);
        }
    }
    d_splice_alias(inode, dentry)
}

unsafe fn ext2_get_parent(child: *mut dentry) -> *mut dentry {
    let mut ino: ino_t = 0;
    let res = ext2_inode_by_name(d_inode(child), &dotdot_name, &mut ino);
    if res != 0 {
        return ERR_PTR(res);
    }
    d_obtain_alias(ext2_iget((*child).d_sb, ino))
}

/*
 * By the time this is called, we already have created
 * the directory cache entry for the new file, but it
 * is so far negative - it has no inode.
 *
 * If the create succeeds, we fill in the inode information
 * with d_instantiate().
 */
unsafe fn ext2_create(_idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> i32 {
    let err = dquot_initialize(dir);
    if err != 0 { return err; }
    let inode = ext2_new_inode(dir, mode, &(*dentry).d_name);
    if IS_ERR(inode) { return PTR_ERR(inode); }
    ext2_set_file_ops(inode);
    mark_inode_dirty(inode);
    ext2_add_nondir(dentry, inode)
}

unsafe fn ext2_tmpfile(_idmap: *mut mnt_idmap, dir: *mut inode, file: *mut file, mode: umode_t) -> i32 {
    let inode = ext2_new_inode(dir, mode, core::ptr::null());
    if IS_ERR(inode) { return PTR_ERR(inode); }
    ext2_set_file_ops(inode);
    mark_inode_dirty(inode);
    d_tmpfile(file, inode);
    unlock_new_inode(inode);
    finish_open_simple(file, 0)
}

unsafe fn ext2_mknod(_idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t, rdev: dev_t) -> i32 {
    let err = dquot_initialize(dir);
    if err != 0 { return err; }
    let inode = ext2_new_inode(dir, mode, &(*dentry).d_name);
    let mut err = PTR_ERR(inode);
    if !IS_ERR(inode) {
        init_special_inode(inode, (*inode).i_mode, rdev);
        (*inode).i_op = &ext2_special_inode_operations;
        mark_inode_dirty(inode);
        err = ext2_add_nondir(dentry, inode);
    }
    err
}

unsafe fn ext2_symlink(_idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, symname: *const c_char) -> i32 {
    let sb = (*dir).i_sb;
    let mut err = -ENAMETOOLONG;
    let l = strlen(symname) + 1;
    let inode;
    if l > (*sb).s_blocksize { return err; }
    err = dquot_initialize(dir);
    if err != 0 { return err; }
    inode = ext2_new_inode(dir, S_IFLNK | S_IRWXUGO, &(*dentry).d_name);
    err = PTR_ERR(inode);
    if IS_ERR(inode) { return err; }
    if l > core::mem::size_of_val(&(*EXT2_I(inode)).i_data) {
        (*inode).i_op = &ext2_symlink_inode_operations;
        inode_nohighmem(inode);
        (*(*inode).i_mapping).a_ops = &ext2_aops;
        err = page_symlink(inode, symname, l);
        if err != 0 { inode_dec_link_count(inode); discard_new_inode(inode); return err; }
    } else {
        (*inode).i_op = &ext2_fast_symlink_inode_operations;
        (*inode).i_link = (*EXT2_I(inode)).i_data.as_mut_ptr() as *mut c_char;
        memcpy((*inode).i_link, symname, l);
        (*inode).i_size = l - 1;
    }
    mark_inode_dirty(inode);
    ext2_add_nondir(dentry, inode)
}

unsafe fn ext2_link(old_dentry: *mut dentry, dir: *mut inode, dentry: *mut dentry) -> i32 {
    let inode = d_inode(old_dentry);
    let err = dquot_initialize(dir);
    if err != 0 { return err; }
    inode_set_ctime_current(inode);
    inode_inc_link_count(inode);
    ihold(inode);
    let err = ext2_add_link(dentry, inode);
    if err == 0 { d_instantiate(dentry, inode); return 0; }
    inode_dec_link_count(inode);
    iput(inode);
    err
}

unsafe fn ext2_mkdir(_idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> *mut dentry {
    let mut err = dquot_initialize(dir);
    if err != 0 { return ERR_PTR(err); }
    inode_inc_link_count(dir);
    let inode = ext2_new_inode(dir, mode, &(*dentry).d_name);
    err = PTR_ERR(inode);
    if IS_ERR(inode) { inode_dec_link_count(dir); return ERR_PTR(err); }
    (*inode).i_op = &ext2_dir_inode_operations;
    (*inode).i_fop = &ext2_dir_operations;
    (*(*inode).i_mapping).a_ops = &ext2_aops;
    inode_inc_link_count(inode);
    err = ext2_make_empty(inode, dir);
    if err != 0 { inode_dec_link_count(inode); inode_dec_link_count(inode); discard_new_inode(inode); inode_dec_link_count(dir); return ERR_PTR(err); }
    err = ext2_add_link(dentry, inode);
    if err != 0 { inode_dec_link_count(inode); inode_dec_link_count(inode); discard_new_inode(inode); inode_dec_link_count(dir); return ERR_PTR(err); }
    d_instantiate_new(dentry, inode);
    ERR_PTR(err)
}

unsafe fn ext2_unlink(dir: *mut inode, dentry: *mut dentry) -> i32 {
    let inode = d_inode(dentry);
    let mut folio: *mut folio = core::ptr::null_mut();
    let err = dquot_initialize(dir);
    if err != 0 { return err; }
    let de = ext2_find_entry(dir, &(*dentry).d_name, &mut folio);
    if IS_ERR(de) { return PTR_ERR(de); }
    let err = ext2_delete_entry(de, folio);
    folio_release_kmap(folio, de);
    if err != 0 { return err; }
    inode_set_ctime_to_ts(inode, inode_get_ctime(dir));
    if (*inode).i_nlink != 0 { inode_dec_link_count(inode); }
    0
}

unsafe fn ext2_rmdir(dir: *mut inode, dentry: *mut dentry) -> i32 {
    let inode = d_inode(dentry);
    let mut err = -ENOTEMPTY;
    if ext2_empty_dir(inode) {
        err = ext2_unlink(dir, dentry);
        if err == 0 { (*inode).i_size = 0; inode_dec_link_count(inode); inode_dec_link_count(dir); }
    }
    err
}

unsafe fn ext2_rename(_idmap: *mut mnt_idmap, old_dir: *mut inode, old_dentry: *mut dentry, new_dir: *mut inode, new_dentry: *mut dentry, flags: u32) -> i32 {
    let old_inode = d_inode(old_dentry);
    let new_inode = d_inode(new_dentry);
    let mut dir_folio: *mut folio = core::ptr::null_mut();
    let mut dir_de: *mut ext2_dir_entry_2 = core::ptr::null_mut();
    let mut old_folio: *mut folio = core::ptr::null_mut();
    let old_is_dir = S_ISDIR((*old_inode).i_mode);
    if flags & !RENAME_NOREPLACE != 0 { return -EINVAL; }
    let mut err = dquot_initialize(old_dir); if err != 0 { return err; }
    err = dquot_initialize(new_dir); if err != 0 { return err; }
    let old_de = ext2_find_entry(old_dir, &(*old_dentry).d_name, &mut old_folio);
    if IS_ERR(old_de) { return PTR_ERR(old_de); }
    if old_is_dir && old_dir != new_dir {
        err = -EIO; dir_de = ext2_dotdot(old_inode, &mut dir_folio); if dir_de.is_null() { folio_release_kmap(old_folio, old_de); return err; }
    }
    if !new_inode.is_null() {
        let mut new_folio: *mut folio = core::ptr::null_mut();
        let new_de;
        if old_is_dir && !ext2_empty_dir(new_inode) { err = -ENOTEMPTY; if !dir_de.is_null() { folio_release_kmap(dir_folio, dir_de); } folio_release_kmap(old_folio, old_de); return err; }
        new_de = ext2_find_entry(new_dir, &(*new_dentry).d_name, &mut new_folio);
        if IS_ERR(new_de) { err = PTR_ERR(new_de); if !dir_de.is_null() { folio_release_kmap(dir_folio, dir_de); } folio_release_kmap(old_folio, old_de); return err; }
        err = ext2_set_link(new_dir, new_de, new_folio, old_inode, true);
        folio_release_kmap(new_folio, new_de); if err != 0 { if !dir_de.is_null() { folio_release_kmap(dir_folio, dir_de); } folio_release_kmap(old_folio, old_de); return err; }
        inode_set_ctime_current(new_inode); if old_is_dir { drop_nlink(new_inode); } inode_dec_link_count(new_inode);
    } else {
        err = ext2_add_link(new_dentry, old_inode); if err != 0 { if !dir_de.is_null() { folio_release_kmap(dir_folio, dir_de); } folio_release_kmap(old_folio, old_de); return err; }
        if old_is_dir { inode_inc_link_count(new_dir); }
    }
    /* Like most other Unix systems, set the ctime for inodes on a rename. */
    inode_set_ctime_current(old_inode); mark_inode_dirty(old_inode);
    err = ext2_delete_entry(old_de, old_folio);
    if err == 0 && old_is_dir { if old_dir != new_dir { err = ext2_set_link(old_inode, dir_de, dir_folio, new_dir, false); } inode_dec_link_count(old_dir); }
    if !dir_de.is_null() { folio_release_kmap(dir_folio, dir_de); }
    folio_release_kmap(old_folio, old_de);
    err
}

const ext2_dir_inode_operations: inode_operations = inode_operations {
    create: Some(ext2_create), lookup: Some(ext2_lookup), link: Some(ext2_link), unlink: Some(ext2_unlink), symlink: Some(ext2_symlink), mkdir: Some(ext2_mkdir), rmdir: Some(ext2_rmdir), mknod: Some(ext2_mknod), rename: Some(ext2_rename), listxattr: Some(ext2_listxattr), getattr: Some(ext2_getattr), setattr: Some(ext2_setattr), get_inode_acl: Some(ext2_get_acl), set_acl: Some(ext2_set_acl), tmpfile: Some(ext2_tmpfile), fileattr_get: Some(ext2_fileattr_get), fileattr_set: Some(ext2_fileattr_set),
};

const ext2_special_inode_operations: inode_operations = inode_operations {
    listxattr: Some(ext2_listxattr), getattr: Some(ext2_getattr), setattr: Some(ext2_setattr), get_inode_acl: Some(ext2_get_acl), set_acl: Some(ext2_set_acl),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
