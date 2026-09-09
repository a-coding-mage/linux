// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/minix/namei.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 */

// Declarations supplied by minix.h and the surrounding kernel translation.

unsafe fn add_nondir(dentry: *mut dentry, inode: *mut inode) -> i32 {
    let err = minix_add_link(dentry, inode);
    if err == 0 {
        d_instantiate(dentry, inode);
        return 0;
    }
    inode_dec_link_count(inode);
    iput(inode);
    err
}

unsafe fn minix_lookup(dir: *mut inode, dentry: *mut dentry, _flags: u32) -> *mut dentry {
    let mut inode: *mut inode = core::ptr::null_mut();
    let ino: ino_t;

    if (*dentry).d_name.len > (*minix_sb((*dir).i_sb)).s_namelen {
        return ERR_PTR(-ENAMETOOLONG);
    }

    ino = minix_inode_by_name(dentry);
    if ino != 0 {
        inode = minix_iget((*dir).i_sb, ino);
    }
    d_splice_alias(inode, dentry)
}

unsafe fn minix_mknod(_idmap: *mut mnt_idmap, dir: *mut inode,
                      dentry: *mut dentry, mode: umode_t, rdev: dev_t) -> i32 {
    if !old_valid_dev(rdev) {
        return -EINVAL;
    }

    let inode = minix_new_inode(dir, mode);
    if IS_ERR(inode) {
        return PTR_ERR(inode);
    }

    minix_set_inode(inode, rdev);
    mark_inode_dirty(inode);
    add_nondir(dentry, inode)
}

unsafe fn minix_tmpfile(_idmap: *mut mnt_idmap, dir: *mut inode,
                        file: *mut file, mode: umode_t) -> i32 {
    let inode = minix_new_inode(dir, mode);

    if IS_ERR(inode) {
        return finish_open_simple(file, PTR_ERR(inode));
    }
    minix_set_inode(inode, 0);
    mark_inode_dirty(inode);
    d_tmpfile(file, inode);
    finish_open_simple(file, 0)
}

unsafe fn minix_create(_idmap: *mut mnt_idmap, dir: *mut inode,
                       dentry: *mut dentry, mode: umode_t) -> i32 {
    minix_mknod(&mut nop_mnt_idmap, dir, dentry, mode, 0)
}

unsafe fn minix_symlink(_idmap: *mut mnt_idmap, dir: *mut inode,
                        dentry: *mut dentry, symname: *const c_char) -> i32 {
    let i = strlen(symname) as i32 + 1;
    let inode;
    let err: i32;

    if i > (*(*dir).i_sb).s_blocksize {
        return -ENAMETOOLONG;
    }

    inode = minix_new_inode(dir, S_IFLNK | 0o777);
    if IS_ERR(inode) {
        return PTR_ERR(inode);
    }

    minix_set_inode(inode, 0);
    err = page_symlink(inode, symname, i);
    if unlikely(err != 0) {
        inode_dec_link_count(inode);
        iput(inode);
        return err;
    }
    add_nondir(dentry, inode)
}

unsafe fn minix_link(old_dentry: *mut dentry, dir: *mut inode,
                     dentry: *mut dentry) -> i32 {
    let inode = d_inode(old_dentry);

    inode_set_ctime_current(inode);
    inode_inc_link_count(inode);
    ihold(inode);
    add_nondir(dentry, inode)
}

unsafe fn minix_mkdir(_idmap: *mut mnt_idmap, dir: *mut inode,
                      dentry: *mut dentry, mode: umode_t) -> *mut dentry {
    let inode;
    let mut err: i32;

    inode = minix_new_inode(dir, mode);
    if IS_ERR(inode) {
        return ERR_CAST(inode);
    }

    inode_inc_link_count(dir);
    minix_set_inode(inode, 0);
    inode_inc_link_count(inode);

    err = minix_make_empty(inode, dir);
    if err != 0 { goto_out_fail!(out_fail); }

    err = minix_add_link(dentry, inode);
    if err != 0 { goto_out_fail!(out_fail); }

    d_instantiate(dentry, inode);
    return ERR_PTR(err);

out_fail:
    inode_dec_link_count(inode);
    inode_dec_link_count(inode);
    iput(inode);
    inode_dec_link_count(dir);
    ERR_PTR(err)
}

unsafe fn minix_unlink(dir: *mut inode, dentry: *mut dentry) -> i32 {
    let inode = d_inode(dentry);
    let mut folio: *mut folio = core::ptr::null_mut();
    let de: *mut minix_dir_entry;
    let err: i32;

    if (*inode).i_nlink == 0 {
        minix_error_inode(inode, "inode has corrupted nlink\0".as_ptr() as *const c_char);
        return -EFSCORRUPTED;
    }

    de = minix_find_entry(dentry, &mut folio);
    if de.is_null() {
        return -ENOENT;
    }
    err = minix_delete_entry(de, folio);
    folio_release_kmap(folio, de);

    if err != 0 { return err; }
    inode_set_ctime_to_ts(inode, inode_get_ctime(dir));
    inode_dec_link_count(inode);
    0
}

unsafe fn minix_rmdir(dir: *mut inode, dentry: *mut dentry) -> i32 {
    let inode = d_inode(dentry);
    let mut err = -EFSCORRUPTED;

    if (*dir).i_nlink <= 2 {
        minix_error_inode(dir, "inode has corrupted nlink\0".as_ptr() as *const c_char);
        return err;
    }

    err = -ENOTEMPTY;
    if !minix_empty_dir(inode) { return err; }

    err = minix_unlink(dir, dentry);
    if err == 0 {
        inode_dec_link_count(dir);
        inode_dec_link_count(inode);
    }
    err
}

unsafe fn minix_rename(_idmap: *mut mnt_idmap, old_dir: *mut inode,
                       old_dentry: *mut dentry, new_dir: *mut inode,
                       new_dentry: *mut dentry, flags: u32) -> i32 {
    let old_inode = d_inode(old_dentry);
    let new_inode = d_inode(new_dentry);
    let mut dir_folio: *mut folio = core::ptr::null_mut();
    let mut dir_de: *mut minix_dir_entry = core::ptr::null_mut();
    let mut old_folio: *mut folio = core::ptr::null_mut();
    let old_de: *mut minix_dir_entry;
    let mut err = -ENOENT;

    if flags & !RENAME_NOREPLACE != 0 { return -EINVAL; }

    old_de = minix_find_entry(old_dentry, &mut old_folio);
    if old_de.is_null() { return err; }

    if S_ISDIR((*old_inode).i_mode) {
        err = -EIO;
        dir_de = minix_dotdot(old_inode, &mut dir_folio);
        if dir_de.is_null() { goto_out_old!(old_folio, old_de); }
    }

    if !new_inode.is_null() {
        let mut new_folio: *mut folio = core::ptr::null_mut();
        let new_de: *mut minix_dir_entry;

        err = -ENOTEMPTY;
        if !dir_de.is_null() && !minix_empty_dir(new_inode) { goto_out_dir!(dir_de, dir_folio, old_folio, old_de); }
        err = -EFSCORRUPTED;
        if (*new_inode).i_nlink == 0 || (!dir_de.is_null() && (*new_inode).i_nlink != 2) {
            minix_error_inode(new_inode, "inode has corrupted nlink\0".as_ptr() as *const c_char);
            goto_out_dir!(dir_de, dir_folio, old_folio, old_de);
        }
        if !dir_de.is_null() && (*old_dir).i_nlink <= 2 {
            minix_error_inode(old_dir, "inode has corrupted nlink\0".as_ptr() as *const c_char);
            goto_out_dir!(dir_de, dir_folio, old_folio, old_de);
        }
        err = -ENOENT;
        new_de = minix_find_entry(new_dentry, &mut new_folio);
        if new_de.is_null() { goto_out_dir!(dir_de, dir_folio, old_folio, old_de); }
        err = minix_set_link(new_de, new_folio, old_inode);
        folio_release_kmap(new_folio, new_de);
        if err != 0 { goto_out_dir!(dir_de, dir_folio, old_folio, old_de); }
        inode_set_ctime_current(new_inode);
        if !dir_de.is_null() { drop_nlink(new_inode); }
        inode_dec_link_count(new_inode);
    } else {
        err = minix_add_link(new_dentry, old_inode);
        if err != 0 { goto_out_dir!(dir_de, dir_folio, old_folio, old_de); }
        if !dir_de.is_null() { inode_inc_link_count(new_dir); }
    }

    err = minix_delete_entry(old_de, old_folio);
    if err != 0 { goto_out_dir!(dir_de, dir_folio, old_folio, old_de); }
    mark_inode_dirty(old_inode);
    if !dir_de.is_null() {
        err = minix_set_link(dir_de, dir_folio, new_dir);
        if err == 0 { inode_dec_link_count(old_dir); }
    }
    if !dir_de.is_null() { folio_release_kmap(dir_folio, dir_de); }
    folio_release_kmap(old_folio, old_de);
    err
}

/* directories can handle most operations... */
#[no_mangle]
pub static minix_dir_inode_operations: inode_operations = inode_operations {
    create: Some(minix_create), lookup: Some(minix_lookup), link: Some(minix_link),
    unlink: Some(minix_unlink), symlink: Some(minix_symlink), mkdir: Some(minix_mkdir),
    rmdir: Some(minix_rmdir), mknod: Some(minix_mknod), rename: Some(minix_rename),
    getattr: Some(minix_getattr), tmpfile: Some(minix_tmpfile),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
