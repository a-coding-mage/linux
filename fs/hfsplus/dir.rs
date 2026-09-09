// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/hfsplus/dir.c
 *
 * Copyright (C) 2001
 * Brad Boyer (flar@allandria.com)
 * (C) 2003 Ardis Technologies <roman@ardistech.com>
 *
 * Handling of directories
 */

// Linux kernel dependencies and local HFS+ headers are supplied by the surrounding translation unit.

#[inline]
unsafe fn hfsplus_instantiate(dentry: *mut dentry, inode: *mut inode, cnid: u32) {
    (*dentry).d_fsdata = cnid as usize as *mut core::ffi::c_void;
    d_instantiate(dentry, inode);
}

/* Find the entry inside dir named dentry->d_name */
unsafe fn hfsplus_lookup(dir: *mut inode, dentry: *mut dentry, _flags: u32) -> *mut dentry {
    let mut inode: *mut inode = core::ptr::null_mut();
    let mut fd: hfs_find_data = core::mem::zeroed();
    let sb = (*dir).i_sb;
    let mut entry: hfsplus_cat_entry = core::mem::zeroed();
    let mut linkid: u32 = 0;
    let mut err: i32;
    let mut cnid: u32;
    (*dentry).d_fsdata = core::ptr::null_mut();
    err = hfs_find_init((*HFSPLUS_SB(sb)).cat_tree, &mut fd);
    if err != 0 { return ERR_PTR(err); }
    err = hfsplus_cat_build_key(sb, fd.search_key, (*dir).i_ino, &(*dentry).d_name);
    if err < 0 { hfs_find_exit(&mut fd); return ERR_PTR(err); }
    loop {
        err = hfsplus_brec_read_cat(&mut fd, &mut entry);
        if err != 0 {
            if err == -ENOENT { hfs_find_exit(&mut fd); return d_splice_alias(core::ptr::null_mut(), dentry); }
            hfs_find_exit(&mut fd); return ERR_PTR(err);
        }
        let typ = be16_to_cpu(entry.type_);
        if typ == HFSPLUS_FOLDER {
            if fd.entrylength < core::mem::size_of::<hfsplus_cat_folder>() as i32 { hfs_find_exit(&mut fd); return ERR_PTR(-EIO); }
            cnid = be32_to_cpu(entry.folder.id);
            (*dentry).d_fsdata = cnid as usize as *mut core::ffi::c_void;
        } else if typ == HFSPLUS_FILE {
            if fd.entrylength < core::mem::size_of::<hfsplus_cat_file>() as i32 { hfs_find_exit(&mut fd); return ERR_PTR(-EIO); }
            cnid = be32_to_cpu(entry.file.id);
            if entry.file.user_info.fdType == cpu_to_be32(HFSP_HARDLINK_TYPE) && entry.file.user_info.fdCreator == cpu_to_be32(HFSP_HFSPLUS_CREATOR) && !(*HFSPLUS_SB(sb)).hidden_dir.is_null() && (entry.file.create_date == (*HFSPLUS_I((*HFSPLUS_SB(sb)).hidden_dir)).create_date || entry.file.create_date == (*HFSPLUS_I(d_inode((*sb).s_root))).create_date) {
                let mut name = [0i8; 32];
                if !(*dentry).d_fsdata.is_null() {
                    cnid = (*dentry).d_fsdata as usize as u32; linkid = 0;
                } else {
                    (*dentry).d_fsdata = cnid as usize as *mut core::ffi::c_void;
                    linkid = be32_to_cpu(entry.file.permissions.dev);
                    let mut str_: qstr = core::mem::zeroed();
                    str_.len = sprintf(name.as_mut_ptr(), b"iNode%d\0".as_ptr() as *const i8, linkid) as u32;
                    str_.name = name.as_ptr();
                    err = hfsplus_cat_build_key(sb, fd.search_key, (*HFSPLUS_SB(sb)).hidden_dir.as_ref().unwrap().i_ino, &str_);
                    if err < 0 { hfs_find_exit(&mut fd); return ERR_PTR(err); }
                    continue;
                }
            } else if (*dentry).d_fsdata.is_null() { (*dentry).d_fsdata = cnid as usize as *mut core::ffi::c_void; }
        } else { pr_err(b"invalid catalog entry type in lookup\n\0".as_ptr()); hfs_find_exit(&mut fd); return ERR_PTR(-EIO); }
        hfs_find_exit(&mut fd);
        inode = hfsplus_iget((*dir).i_sb, cnid);
        if IS_ERR(inode) { return ERR_CAST(inode); }
        if S_ISREG((*inode).i_mode) { (*HFSPLUS_I(inode)).linkid = linkid; }
        return d_splice_alias(inode, dentry);
    }
}

unsafe fn hfsplus_readdir(file: *mut file, ctx: *mut dir_context) -> i32 {
    let inode = file_inode(file); let sb = (*inode).i_sb;
    if (*file).f_pos >= (*inode).i_size { return 0; }
    let mut fd: hfs_find_data = core::mem::zeroed(); let mut entry: hfsplus_cat_entry = core::mem::zeroed();
    let mut err = hfs_find_init((*HFSPLUS_SB(sb)).cat_tree, &mut fd); if err != 0 { return err; }
    let strbuf = kmalloc(NLS_MAX_CHARSET_SIZE * HFSPLUS_MAX_STRLEN + 1, GFP_KERNEL); if strbuf.is_null() { hfs_find_exit(&mut fd); return -ENOMEM; }
    hfsplus_cat_build_key_with_cnid(sb, fd.search_key, (*inode).i_ino); err = hfs_brec_find(&mut fd, hfs_find_rec_by_key); if err != 0 { kfree(strbuf); hfs_find_exit(&mut fd); return err; }
    if (*ctx).pos == 0 { if !dir_emit_dot(file, ctx) { kfree(strbuf); hfs_find_exit(&mut fd); return 0; } (*ctx).pos = 1; }
    if (*ctx).pos == 1 { if fd.entrylength > core::mem::size_of::<hfsplus_cat_entry>() as i32 || fd.entrylength < 0 { err = -EIO; } else { hfs_bnode_read(fd.bnode, &mut entry, fd.entryoffset, fd.entrylength); if be16_to_cpu(entry.type_) != HFSPLUS_FOLDER_THREAD || fd.entrylength < HFSPLUS_MIN_THREAD_SZ { err = -EIO; } else if !dir_emit(ctx, b"..\0".as_ptr() as *const i8, 2, be32_to_cpu(entry.thread.parentID), DT_DIR) { err = 0; } else { (*ctx).pos = 2; } } if err != 0 { kfree(strbuf); hfs_find_exit(&mut fd); return err; } }
    if (*ctx).pos >= (*inode).i_size { kfree(strbuf); hfs_find_exit(&mut fd); return 0; }
    err = hfs_brec_goto(&mut fd, (*ctx).pos - 1); if err != 0 { kfree(strbuf); hfs_find_exit(&mut fd); return err; }
    loop { if be32_to_cpu((*fd.key).cat.parent) != (*inode).i_ino { err = -EIO; break; } if fd.entrylength > core::mem::size_of::<hfsplus_cat_entry>() as i32 || fd.entrylength < 0 { err = -EIO; break; } hfs_bnode_read(fd.bnode, &mut entry, fd.entryoffset, fd.entrylength); let typ = be16_to_cpu(entry.type_); let mut len = NLS_MAX_CHARSET_SIZE * HFSPLUS_MAX_STRLEN; err = hfsplus_uni2asc_str(sb, &(*fd.key).cat.name, strbuf, &mut len); if err != 0 { break; } let id; let dt; if typ == HFSPLUS_FOLDER { id = be32_to_cpu(entry.folder.id); dt = DT_DIR; } else if typ == HFSPLUS_FILE { id = be32_to_cpu(entry.file.id); let mode = be16_to_cpu(entry.file.permissions.mode); dt = if S_ISREG(mode) { DT_REG } else if S_ISLNK(mode) { DT_LNK } else if S_ISFIFO(mode) { DT_FIFO } else if S_ISCHR(mode) { DT_CHR } else if S_ISBLK(mode) { DT_BLK } else if S_ISSOCK(mode) { DT_SOCK } else { DT_UNKNOWN }; } else { err = -EIO; break; } if !dir_emit(ctx, strbuf, len, id, dt) { break; } (*ctx).pos += 1; if (*ctx).pos >= (*inode).i_size { break; } err = hfs_brec_goto(&mut fd, 1); if err != 0 { break; } }
    kfree(strbuf); hfs_find_exit(&mut fd); err
}

unsafe fn hfsplus_dir_release(_inode: *mut inode, file: *mut file) -> i32 { kfree((*file).private_data); 0 }

// Remaining directory mutation entry points retain the C implementation's ABI and are declared for linkage.
unsafe extern "C" {
    fn hfsplus_link(src_dentry: *mut dentry, dst_dir: *mut inode, dst_dentry: *mut dentry) -> i32;
    fn hfsplus_unlink(dir: *mut inode, dentry: *mut dentry) -> i32;
    fn hfsplus_rmdir(dir: *mut inode, dentry: *mut dentry) -> i32;
    fn hfsplus_symlink(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, symname: *const i8) -> i32;
    fn hfsplus_mknod(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t, rdev: dev_t) -> i32;
    fn hfsplus_create(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> i32;
    fn hfsplus_mkdir(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> *mut dentry;
    fn hfsplus_rename(idmap: *mut mnt_idmap, old_dir: *mut inode, old_dentry: *mut dentry, new_dir: *mut inode, new_dentry: *mut dentry, flags: u32) -> i32;
}

#[no_mangle]
pub static hfsplus_dir_inode_operations: inode_operations = inode_operations { lookup: Some(hfsplus_lookup), create: Some(hfsplus_create), link: Some(hfsplus_link), unlink: Some(hfsplus_unlink), mkdir: Some(hfsplus_mkdir), rmdir: Some(hfsplus_rmdir), symlink: Some(hfsplus_symlink), mknod: Some(hfsplus_mknod), rename: Some(hfsplus_rename), getattr: Some(hfsplus_getattr), listxattr: Some(hfsplus_listxattr), fileattr_get: Some(hfsplus_fileattr_get), fileattr_set: Some(hfsplus_fileattr_set) };

#[no_mangle]
pub static hfsplus_dir_operations: file_operations = file_operations { fsync: Some(hfsplus_file_fsync), read: Some(generic_read_dir), iterate_shared: Some(hfsplus_readdir), unlocked_ioctl: Some(hfsplus_ioctl), llseek: Some(generic_file_llseek), release: Some(hfsplus_dir_release) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
