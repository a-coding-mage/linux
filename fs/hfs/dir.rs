/*
 *  linux/fs/hfs/dir.c
 *
 * Rust translation of the HFS directory implementation.
 * C headers and externally supplied symbols are dependencies of this file.
 */

use core::ffi::c_void;

unsafe fn hfs_lookup(dir: *mut inode, dentry: *mut dentry, _flags: c_uint) -> *mut dentry {
    let mut rec: hfs_cat_rec = core::mem::zeroed();
    let mut fd: hfs_find_data = core::mem::zeroed();
    let mut inode: *mut inode = core::ptr::null_mut();
    let mut res: c_int;

    res = hfs_find_init((*HFS_SB((*dir).i_sb)).cat_tree, &mut fd);
    if res != 0 { return ERR_PTR(res); }
    hfs_cat_build_key((*dir).i_sb, fd.search_key, (*dir).i_ino, &(*dentry).d_name);
    res = hfs_brec_read(&mut fd, &mut rec as *mut _ as *mut c_void, core::mem::size_of::<hfs_cat_rec>());
    if res != 0 {
        if res != -ENOENT { inode = ERR_PTR(res); }
    } else {
        inode = hfs_iget((*dir).i_sb, &(*fd.search_key).cat, &rec);
        if inode.is_null() { inode = ERR_PTR(-EACCES); }
    }
    hfs_find_exit(&mut fd);
    d_splice_alias(inode, dentry)
}

unsafe fn hfs_readdir(file: *mut file, ctx: *mut dir_context) -> c_int {
    let inode = file_inode(file);
    let sb = (*inode).i_sb;
    let mut len: c_int;
    let mut err: c_int;
    let mut strbuf = [0i8; HFS_MAX_NAMELEN as usize];
    let mut entry: hfs_cat_rec_union = core::mem::zeroed();
    let mut fd: hfs_find_data = core::mem::zeroed();
    let mut rd: *mut hfs_readdir_data;
    let mut typ: u16;

    if (*ctx).pos >= (*inode).i_size { return 0; }
    err = hfs_find_init((*HFS_SB(sb)).cat_tree, &mut fd);
    if err != 0 { return err; }
    hfs_cat_build_key(sb, fd.search_key, (*inode).i_ino, core::ptr::null());
    err = hfs_brec_find(&mut fd);
    if err != 0 { hfs_find_exit(&mut fd); return err; }

    if (*ctx).pos == 0 {
        if !dir_emit_dot(file, ctx) { hfs_find_exit(&mut fd); return err; }
        (*ctx).pos = 1;
    }
    if (*ctx).pos == 1 {
        if fd.entrylength > core::mem::size_of_val(&entry) as i32 || fd.entrylength < 0 { err = -EIO; hfs_find_exit(&mut fd); return err; }
        hfs_bnode_read(fd.bnode, &mut entry as *mut _ as *mut c_void, fd.entryoffset, fd.entrylength);
        if entry.type_ != HFS_CDR_THD { pr_err(c"bad catalog folder thread\n"); hfs_find_exit(&mut fd); return -EIO; }
        if !dir_emit(ctx, c"..", 2, be32_to_cpu(entry.thread.ParID), DT_DIR) { hfs_find_exit(&mut fd); return err; }
        (*ctx).pos = 2;
    }
    if (*ctx).pos >= (*inode).i_size { hfs_find_exit(&mut fd); return err; }
    rd = (*file).private_data as *mut hfs_readdir_data;
    if !rd.is_null() && (*rd).pos == (*ctx).pos {
        core::ptr::copy_nonoverlapping(&(*rd).key, fd.search_key as *mut hfs_cat_key, 1);
        err = hfs_brec_find(&mut fd);
        if err == -ENOENT { err = hfs_brec_goto(&mut fd, 1); }
    } else { err = hfs_brec_goto(&mut fd, (*ctx).pos - 1); }
    if err != 0 { hfs_find_exit(&mut fd); return err; }

    loop {
        if be32_to_cpu((*fd.key).cat.ParID) != (*inode).i_ino { pr_err(c"walked past end of dir\n"); err = -EIO; break; }
        if fd.entrylength > core::mem::size_of_val(&entry) as i32 || fd.entrylength < 0 { err = -EIO; break; }
        hfs_bnode_read(fd.bnode, &mut entry as *mut _ as *mut c_void, fd.entryoffset, fd.entrylength);
        typ = entry.type_;
        len = hfs_mac2asc(sb, strbuf.as_mut_ptr(), &(*fd.key).cat.CName);
        if typ == HFS_CDR_DIR {
            if fd.entrylength < core::mem::size_of::<hfs_cat_dir>() as i32 { pr_err(c"small dir entry\n"); err = -EIO; break; }
            if !dir_emit(ctx, strbuf.as_ptr(), len, be32_to_cpu(entry.dir.DirID), DT_DIR) { break; }
        } else if typ == HFS_CDR_FIL {
            if fd.entrylength < core::mem::size_of::<hfs_cat_file>() as i32 { pr_err(c"small file entry\n"); err = -EIO; break; }
            if !dir_emit(ctx, strbuf.as_ptr(), len, be32_to_cpu(entry.file.FlNum), DT_REG) { break; }
        } else { pr_err(c"bad catalog entry type %d\n", typ); err = -EIO; break; }
        (*ctx).pos += 1;
        if (*ctx).pos >= (*inode).i_size { break; }
        err = hfs_brec_goto(&mut fd, 1);
        if err != 0 { break; }
    }
    if err == 0 {
        if rd.is_null() { rd = kmalloc_obj::<hfs_readdir_data>(); if rd.is_null() { err = -ENOMEM; } else { (*file).private_data = rd as *mut c_void; } }
        if !rd.is_null() { (*rd).pos = (*ctx).pos; core::ptr::copy_nonoverlapping(&(*fd.key).cat, &mut (*rd).key, 1); }
    }
    hfs_find_exit(&mut fd);
    err
}

unsafe fn hfs_dir_release(_inode: *mut inode, file: *mut file) -> c_int { kfree((*file).private_data); 0 }

unsafe fn hfs_create(_idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> c_int {
    let inode = hfs_new_inode(dir, &(*dentry).d_name, mode);
    if IS_ERR(inode) { return PTR_ERR(inode); }
    let res = hfs_cat_create((*inode).i_ino, dir, &(*dentry).d_name, inode);
    if res != 0 { clear_nlink(inode); hfs_delete_inode(inode); iput(inode); return res; }
    d_instantiate(dentry, inode); mark_inode_dirty(inode); 0
}

unsafe fn hfs_mkdir(_idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> *mut dentry {
    let inode = hfs_new_inode(dir, &(*dentry).d_name, mode);
    if IS_ERR(inode) { return ERR_CAST(inode); }
    let res = hfs_cat_create((*inode).i_ino, dir, &(*dentry).d_name, inode);
    if res != 0 { clear_nlink(inode); hfs_delete_inode(inode); iput(inode); return ERR_PTR(res); }
    d_instantiate(dentry, inode); mark_inode_dirty(inode); core::ptr::null_mut()
}

unsafe fn hfs_remove(dir: *mut inode, dentry: *mut dentry) -> c_int {
    let sb = (*dir).i_sb; let inode = d_inode(dentry);
    if S_ISDIR((*inode).i_mode) && (*inode).i_size != 2 { return -ENOTEMPTY; }
    if !is_hfs_cnid_counts_valid(sb) { pr_err(c"cannot remove file/folder\n"); return -ERANGE; }
    let res = hfs_cat_delete((*inode).i_ino, dir, &(*dentry).d_name); if res != 0 { return res; }
    clear_nlink(inode); inode_set_ctime_current(inode); hfs_delete_inode(inode); mark_inode_dirty(inode); 0
}

unsafe fn hfs_rename(_idmap: *mut mnt_idmap, old_dir: *mut inode, old_dentry: *mut dentry, new_dir: *mut inode, new_dentry: *mut dentry, flags: c_uint) -> c_int {
    if flags & !RENAME_NOREPLACE != 0 { return -EINVAL; }
    if d_really_is_positive(new_dentry) { let res = hfs_remove(new_dir, new_dentry); if res != 0 { return res; } }
    let res = hfs_cat_move((*d_inode(old_dentry)).i_ino, old_dir, &(*old_dentry).d_name, new_dir, &(*new_dentry).d_name);
    if res == 0 { let inode = d_inode(old_dentry); hfs_cat_build_key((*old_dir).i_sb, &mut (*HFS_I(inode)).cat_key as *mut _ as *mut btree_key, (*new_dir).i_ino, &(*new_dentry).d_name); inode_set_ctime_current(inode); mark_inode_dirty(inode); } res
}

#[repr(C)]
pub struct file_operations {
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_void, usize, *mut i64) -> isize>,
    pub iterate_shared: Option<unsafe fn(*mut file, *mut dir_context) -> c_int>,
    pub llseek: Option<unsafe fn(*mut file, i64, c_int) -> i64>,
    pub release: Option<unsafe fn(*mut inode, *mut file) -> c_int>,
}

#[no_mangle]
pub static hfs_dir_operations: file_operations = file_operations {
    read: Some(generic_read_dir),
    iterate_shared: Some(hfs_readdir),
    llseek: Some(generic_file_llseek),
    release: Some(hfs_dir_release),
};

#[repr(C)]
pub struct inode_operations {
    pub create: Option<unsafe fn(*mut mnt_idmap, *mut inode, *mut dentry, umode_t) -> c_int>,
    pub lookup: Option<unsafe fn(*mut inode, *mut dentry, c_uint) -> *mut dentry>,
    pub unlink: Option<unsafe fn(*mut inode, *mut dentry) -> c_int>,
    pub mkdir: Option<unsafe fn(*mut mnt_idmap, *mut inode, *mut dentry, umode_t) -> *mut dentry>,
    pub rmdir: Option<unsafe fn(*mut inode, *mut dentry) -> c_int>,
    pub rename: Option<unsafe fn(*mut mnt_idmap, *mut inode, *mut dentry, *mut inode, *mut dentry, c_uint) -> c_int>,
    pub setattr: Option<unsafe fn()>,
    pub fileattr_get: Option<unsafe fn()>,
}

#[no_mangle]
pub static hfs_dir_inode_operations: inode_operations = inode_operations {
    create: Some(hfs_create),
    lookup: Some(hfs_lookup),
    unlink: Some(hfs_remove),
    mkdir: Some(hfs_mkdir),
    rmdir: Some(hfs_remove),
    rename: Some(hfs_rename),
    setattr: Some(hfs_inode_setattr),
    fileattr_get: Some(hfs_fileattr_get),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
