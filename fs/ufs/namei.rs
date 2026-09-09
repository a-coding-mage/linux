// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/ufs/namei.c
 *
 * Migration to usage of "page cache" on May 2006 by
 * Evgeniy Dushistov <dushistov@mail.ru> based on ext2 code base.
 *
 * Copyright (C) 1998
 * Daniel Pirkl <daniel.pirkl@email.cz>
 * Charles University, Faculty of Mathematics and Physics
 *
 *  from linux/fs/ext2/namei.c
 * Copyright (C) 1992, 1993, 1994, 1995
 * Remy Card (card@masi.ibp.fr)
 * Laboratoire MASI - Institut Blaise Pascal
 * Universite Pierre et Marie Curie (Paris VI)
 *
 *  from linux/fs/minix/namei.c
 * Copyright (C) 1991, 1992  Linus Torvalds
 *
 *  Big-endian to little-endian byte-swapping/bitmaps by
 *        David S. Miller (davem@caip.rutgers.edu), 1995
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn ufs_add_nondir(dentry: *mut dentry, inode: *mut inode) -> c_int {
    let err = ufs_add_link(dentry, inode);
    if err == 0 {
        d_instantiate_new(dentry, inode);
        return 0;
    }
    inode_dec_link_count(inode);
    discard_new_inode(inode);
    err
}

unsafe fn ufs_lookup(dir: *mut inode, dentry: *mut dentry, _flags: c_uint) -> *mut dentry {
    let mut inode: *mut inode = core::ptr::null_mut();
    let ino: ino_t;
    if (*dentry).d_name.len > UFS_MAXNAMLEN {
        return ERR_PTR(-ENAMETOOLONG);
    }
    ino = ufs_inode_by_name(dir, &(*dentry).d_name);
    if ino != 0 {
        inode = ufs_iget((*dir).i_sb, ino);
    }
    d_splice_alias(inode, dentry)
}

/* By the time this is called, the directory cache entry is negative. */
unsafe fn ufs_create(_idmap: *mut mnt_idmap, dir: *mut inode,
                     dentry: *mut dentry, mode: umode_t) -> c_int {
    let inode = ufs_new_inode(dir, mode);
    if IS_ERR(inode) { return PTR_ERR(inode); }
    (*inode).i_op = &ufs_file_inode_operations;
    (*inode).i_fop = &ufs_file_operations;
    (*(*inode).i_mapping).a_ops = &ufs_aops;
    mark_inode_dirty(inode);
    ufs_add_nondir(dentry, inode)
}

unsafe fn ufs_mknod(_idmap: *mut mnt_idmap, dir: *mut inode,
                    dentry: *mut dentry, mode: umode_t, rdev: dev_t) -> c_int {
    if !old_valid_dev(rdev) { return -EINVAL; }
    let inode = ufs_new_inode(dir, mode);
    let mut err = PTR_ERR(inode);
    if !IS_ERR(inode) {
        init_special_inode(inode, mode, rdev);
        ufs_set_inode_dev((*inode).i_sb, UFS_I(inode), rdev);
        mark_inode_dirty(inode);
        err = ufs_add_nondir(dentry, inode);
    }
    err
}

unsafe fn ufs_symlink(_idmap: *mut mnt_idmap, dir: *mut inode,
                      dentry: *mut dentry, symname: *const c_char) -> c_int {
    let sb = (*dir).i_sb;
    let l = strlen(symname) + 1;
    let inode = ufs_new_inode(dir, S_IFLNK | S_IRWXUGO);
    let mut err = PTR_ERR(inode);
    if l > (*sb).s_blocksize { return -ENAMETOOLONG; }
    if IS_ERR(inode) { return err; }
    if l > (*(*UFS_SB(sb)).s_uspi).s_maxsymlinklen {
        (*inode).i_op = &page_symlink_inode_operations;
        inode_nohighmem(inode);
        (*(*inode).i_mapping).a_ops = &ufs_aops;
        err = page_symlink(inode, symname, l);
        if err != 0 { inode_dec_link_count(inode); discard_new_inode(inode); return err; }
    } else {
        (*inode).i_op = &simple_symlink_inode_operations;
        (*inode).i_link = UFS_I(inode).as_ref().unwrap().i_u1.i_symlink as *mut c_char;
        memcpy((*inode).i_link as *mut c_void, symname as *const c_void, l);
        (*inode).i_size = l - 1;
    }
    mark_inode_dirty(inode);
    ufs_add_nondir(dentry, inode)
}

unsafe fn ufs_link(old_dentry: *mut dentry, dir: *mut inode, dentry: *mut dentry) -> c_int {
    let inode = d_inode(old_dentry);
    inode_set_ctime_current(inode); inode_inc_link_count(inode); ihold(inode);
    let error = ufs_add_link(dentry, inode);
    if error != 0 { inode_dec_link_count(inode); iput(inode); }
    else { d_instantiate(dentry, inode); }
    error
}

unsafe fn ufs_mkdir(_idmap: *mut mnt_idmap, dir: *mut inode,
                    dentry: *mut dentry, mode: umode_t) -> *mut dentry {
    inode_inc_link_count(dir);
    let inode = ufs_new_inode(dir, mode);
    let mut err = PTR_ERR(inode);
    if IS_ERR(inode) { inode_dec_link_count(dir); return ERR_PTR(err); }
    (*inode).i_op = &ufs_dir_inode_operations; (*inode).i_fop = &ufs_dir_operations;
    (*(*inode).i_mapping).a_ops = &ufs_aops; inode_inc_link_count(inode);
    err = ufs_make_empty(inode, dir);
    if err == 0 { err = ufs_add_link(dentry, inode); }
    if err != 0 { inode_dec_link_count(inode); inode_dec_link_count(inode); discard_new_inode(inode); inode_dec_link_count(dir); return ERR_PTR(err); }
    d_instantiate_new(dentry, inode); core::ptr::null_mut()
}

unsafe fn ufs_unlink(dir: *mut inode, dentry: *mut dentry) -> c_int {
    let inode = d_inode(dentry); let mut folio: *mut folio = core::ptr::null_mut();
    let de = ufs_find_entry(dir, &(*dentry).d_name, &mut folio);
    if de.is_null() { return -ENOENT; }
    let err = ufs_delete_entry(dir, de, folio);
    if err == 0 { inode_set_ctime_to_ts(inode, inode_get_ctime(dir)); inode_dec_link_count(inode); }
    folio_release_kmap(folio, de); err
}

unsafe fn ufs_rmdir(dir: *mut inode, dentry: *mut dentry) -> c_int {
    let inode = d_inode(dentry); let mut err = -ENOTEMPTY;
    if ufs_empty_dir(inode) { err = ufs_unlink(dir, dentry); if err == 0 { (*inode).i_size = 0; inode_dec_link_count(inode); inode_dec_link_count(dir); } }
    err
}

unsafe fn ufs_rename(_idmap: *mut mnt_idmap, old_dir: *mut inode, old_dentry: *mut dentry,
                     new_dir: *mut inode, new_dentry: *mut dentry, flags: c_uint) -> c_int {
    let old_inode = d_inode(old_dentry); let new_inode = d_inode(new_dentry);
    if flags & !RENAME_NOREPLACE != 0 { return -EINVAL; }
    let mut old_folio: *mut folio = core::ptr::null_mut();
    let old_de = ufs_find_entry(old_dir, &(*old_dentry).d_name, &mut old_folio);
    if old_de.is_null() { return -ENOENT; }
    let mut dir_folio: *mut folio = core::ptr::null_mut(); let mut dir_de: *mut ufs_dir_entry = core::ptr::null_mut();
    if S_ISDIR((*old_inode).i_mode) { dir_de = ufs_dotdot(old_inode, &mut dir_folio); if dir_de.is_null() { folio_release_kmap(old_folio, old_de); return -EIO; } }
    let mut err;
    if !new_inode.is_null() {
        if !dir_de.is_null() && !ufs_empty_dir(new_inode) { err = -ENOTEMPTY; }
        else { let mut nf: *mut folio = core::ptr::null_mut(); let nd = ufs_find_entry(new_dir, &(*new_dentry).d_name, &mut nf); if nd.is_null() { err = -ENOENT; } else { err = ufs_set_link(new_dir, nd, nf, old_inode, 1); folio_release_kmap(nf, nd); if err == 0 { inode_set_ctime_current(new_inode); if !dir_de.is_null() { drop_nlink(new_inode); } inode_dec_link_count(new_inode); } } }
    } else { err = ufs_add_link(new_dentry, old_inode); if err == 0 && !dir_de.is_null() { inode_inc_link_count(new_dir); } }
    if err == 0 { inode_set_ctime_current(old_inode); mark_inode_dirty(old_inode); err = ufs_delete_entry(old_dir, old_de, old_folio); if err == 0 && !dir_de.is_null() { if old_dir != new_dir { err = ufs_set_link(old_inode, dir_de, dir_folio, new_dir, 0); } inode_dec_link_count(old_dir); } }
    if !dir_de.is_null() { folio_release_kmap(dir_folio, dir_de); } folio_release_kmap(old_folio, old_de); err
}

#[allow(non_upper_case_globals)]
pub static ufs_dir_inode_operations: inode_operations = inode_operations {
    create: Some(ufs_create), lookup: Some(ufs_lookup), link: Some(ufs_link),
    unlink: Some(ufs_unlink), symlink: Some(ufs_symlink), mkdir: Some(ufs_mkdir),
    rmdir: Some(ufs_rmdir), mknod: Some(ufs_mknod), rename: Some(ufs_rename),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
