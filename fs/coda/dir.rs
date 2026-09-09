// SPDX-License-Identifier: GPL-2.0

/*
 * Directory operations for Coda filesystem
 * Original version: (C) 1996 P. Braam and M. Callahan
 * Rewritten for Linux 2.1. (C) 1997 Carnegie Mellon University
 *
 * Carnegie Mellon encourages users to contribute improvements to
 * the Coda project. Contact Peter Braam (coda@cs.cmu.edu).
 */

/* Linux and Coda headers are supplied by the surrounding translation unit. */

/* same as fs/bad_inode.c */
unsafe fn coda_return_EIO() -> i32 {
    -EIO
}
const CODA_EIO_ERROR: *const core::ffi::c_void = coda_return_EIO as *const core::ffi::c_void;

/* inode operations for directories */
/* access routines: lookup, readlink, permission */
unsafe fn coda_lookup(dir: *mut inode, entry: *mut dentry, _flags: u32) -> *mut dentry {
    let sb = (*dir).i_sb;
    let name = (*entry).d_name.name;
    let length = (*entry).d_name.len;
    let mut inode: *mut inode;
    let mut type_: i32 = 0;

    if length > CODA_MAXNAMLEN {
        pr_err!("name too long: lookup, %s %zu\n", coda_i2s(dir), length);
        return ERR_PTR(-ENAMETOOLONG);
    }

    if is_root_inode(dir) && coda_iscontrol(name, length) {
        inode = coda_cnode_makectl(sb);
        type_ = CODA_NOCACHE;
    } else {
        let mut fid = CodaFid { opaque: [0; _] };
        let error = venus_lookup(sb, coda_i2f(dir), name, length, &mut type_, &mut fid);
        inode = if error == 0 { coda_cnode_make(&fid, sb) } else { ERR_PTR(error) };
    }

    if !IS_ERR(inode) && (type_ & CODA_NOCACHE) != 0 {
        coda_flag_inode(inode, C_VATTR | C_PURGE);
    }
    if inode == ERR_PTR(-ENOENT) { inode = core::ptr::null_mut(); }
    d_splice_alias(inode, entry)
}

unsafe fn coda_permission(_idmap: *mut mnt_idmap, inode: *mut inode, mut mask: i32) -> i32 {
    if (mask & MAY_NOT_BLOCK) != 0 { return -ECHILD; }
    mask &= MAY_READ | MAY_WRITE | MAY_EXEC;
    if mask == 0 { return 0; }
    if (mask & MAY_EXEC) != 0 && !execute_ok(inode) { return -EACCES; }
    if coda_cache_check(inode, mask) != 0 { return 0; }
    let error = venus_access((*inode).i_sb, coda_i2f(inode), mask);
    if error == 0 { coda_cache_enter(inode, mask); }
    error
}

unsafe fn coda_dir_update_mtime(dir: *mut inode) {
    /* REQUERY_VENUS_FOR_MTIME is a build-time configuration condition. */
    #[cfg(REQUERY_VENUS_FOR_MTIME)]
    { coda_flag_inode(dir, C_VATTR); }
    #[cfg(not(REQUERY_VENUS_FOR_MTIME))]
    { inode_set_mtime_to_ts(dir, inode_set_ctime_current(dir)); }
}

unsafe fn coda_dir_inc_nlink(dir: *mut inode) { if (*dir).i_nlink >= 2 { inc_nlink(dir); } }
unsafe fn coda_dir_drop_nlink(dir: *mut inode) { if (*dir).i_nlink > 2 { drop_nlink(dir); } }

unsafe fn coda_create(_idmap: *mut mnt_idmap, dir: *mut inode, de: *mut dentry, mode: umode_t) -> i32 {
    let name = (*de).d_name.name;
    let length = (*de).d_name.len;
    let mut newfid: CodaFid = core::mem::zeroed();
    let mut attrs: coda_vattr = core::mem::zeroed();
    if is_root_inode(dir) && coda_iscontrol(name, length) { return -EPERM; }
    let mut error = venus_create((*dir).i_sb, coda_i2f(dir), name, length, 0, mode, &mut newfid, &mut attrs);
    if error != 0 { d_drop(de); return error; }
    let inode = coda_iget((*dir).i_sb, &newfid, &attrs);
    if IS_ERR(inode) { error = PTR_ERR(inode); d_drop(de); return error; }
    coda_dir_update_mtime(dir); d_instantiate(de, inode); 0
}

unsafe fn coda_mkdir(_idmap: *mut mnt_idmap, dir: *mut inode, de: *mut dentry, mode: umode_t) -> *mut dentry {
    let name = (*de).d_name.name; let len = (*de).d_name.len;
    let mut attrs: coda_vattr = core::mem::zeroed(); let mut newfid: CodaFid = core::mem::zeroed();
    if is_root_inode(dir) && coda_iscontrol(name, len) { return ERR_PTR(-EPERM); }
    attrs.va_mode = mode & !S_IFDIR;
    let error = venus_mkdir((*dir).i_sb, coda_i2f(dir), name, len, &mut newfid, &mut attrs);
    if error != 0 { d_drop(de); return ERR_PTR(error); }
    let inode = coda_iget((*dir).i_sb, &newfid, &attrs);
    if IS_ERR(inode) { let error = PTR_ERR(inode); d_drop(de); return ERR_PTR(error); }
    coda_dir_inc_nlink(dir); coda_dir_update_mtime(dir); d_instantiate(de, inode); core::ptr::null_mut()
}

unsafe fn coda_link(source_de: *mut dentry, dir_inode: *mut inode, de: *mut dentry) -> i32 {
    let inode = d_inode(source_de); let name = (*de).d_name.name; let len = (*de).d_name.len;
    if is_root_inode(dir_inode) && coda_iscontrol(name, len) { return -EPERM; }
    let error = venus_link((*dir_inode).i_sb, coda_i2f(inode), coda_i2f(dir_inode), name, len);
    if error != 0 { d_drop(de); return error; }
    coda_dir_update_mtime(dir_inode); ihold(inode); d_instantiate(de, inode); inc_nlink(inode); 0
}

unsafe fn coda_symlink(_idmap: *mut mnt_idmap, dir_inode: *mut inode, de: *mut dentry, symname: *const i8) -> i32 {
    let name = (*de).d_name.name; let len = (*de).d_name.len; let symlen = strlen(symname);
    if is_root_inode(dir_inode) && coda_iscontrol(name, len) { return -EPERM; }
    if symlen > CODA_MAXPATHLEN { return -ENAMETOOLONG; }
    d_drop(de);
    let error = venus_symlink((*dir_inode).i_sb, coda_i2f(dir_inode), name, len, symname, symlen);
    if error == 0 { coda_dir_update_mtime(dir_inode); } error
}

unsafe fn coda_unlink(dir: *mut inode, de: *mut dentry) -> i32 {
    let error = venus_remove((*dir).i_sb, coda_i2f(dir), (*de).d_name.name, (*de).d_name.len);
    if error != 0 { return error; } coda_dir_update_mtime(dir); drop_nlink(d_inode(de)); 0
}

unsafe fn coda_rmdir(dir: *mut inode, de: *mut dentry) -> i32 {
    let error = venus_rmdir((*dir).i_sb, coda_i2f(dir), (*de).d_name.name, (*de).d_name.len);
    if error == 0 { if d_really_is_positive(de) { clear_nlink(d_inode(de)); } coda_dir_drop_nlink(dir); coda_dir_update_mtime(dir); } error
}

unsafe fn coda_rename(_idmap: *mut mnt_idmap, old_dir: *mut inode, old_dentry: *mut dentry, new_dir: *mut inode, new_dentry: *mut dentry, flags: u32) -> i32 {
    if flags != 0 { return -EINVAL; }
    let error = venus_rename((*old_dir).i_sb, coda_i2f(old_dir), coda_i2f(new_dir), (*old_dentry).d_name.len, (*new_dentry).d_name.len, (*old_dentry).d_name.name, (*new_dentry).d_name.name);
    if error == 0 {
        if d_really_is_positive(new_dentry) { if d_is_dir(new_dentry) { coda_dir_drop_nlink(old_dir); coda_dir_inc_nlink(new_dir); } coda_flag_inode(d_inode(new_dentry), C_VATTR); }
        coda_dir_update_mtime(old_dir); coda_dir_update_mtime(new_dir);
    } error
}

unsafe fn CDT2DT(cdt: u8) -> u32 {
    match cdt { CDT_UNKNOWN => DT_UNKNOWN, CDT_FIFO => DT_FIFO, CDT_CHR => DT_CHR, CDT_DIR => DT_DIR, CDT_BLK => DT_BLK, CDT_REG => DT_REG, CDT_LNK => DT_LNK, CDT_SOCK => DT_SOCK, CDT_WHT => DT_WHT, _ => DT_UNKNOWN }
}

/* support routines */
unsafe fn coda_venus_readdir(coda_file: *mut file, ctx: *mut dir_context) -> i32 {
    let cfi = coda_ftoc(coda_file); let host_file = (*cfi).cfi_container; let cii = ITOC(file_inode(coda_file));
    let vdir = kmalloc_obj::<venus_dirent>(); if vdir.is_null() { return -ENOMEM; }
    if !dir_emit_dots(coda_file, ctx) { kfree(vdir); return 0; }
    let vdir_size = core::mem::offset_of!(venus_dirent, d_name);
    loop {
        let mut pos = (*ctx).pos - 2; let ret = kernel_read(host_file, vdir as *mut core::ffi::c_void, core::mem::size_of::<venus_dirent>(), &mut pos);
        if ret < 0 || ret == 0 { break; }
        if ret < vdir_size as isize || ret < (vdir_size + (*vdir).d_namlen as usize) as isize { break; }
        if (*vdir).d_reclen < (vdir_size + (*vdir).d_namlen as usize) as u16 { break; }
        let mut len = (*vdir).d_namlen; let name = (*vdir).d_name;
        if name[0] == b'.' && (len == 1 || (name[1] == b'.' && len == 2)) { (*vdir).d_fileno = 0; len = 0; }
        if (*vdir).d_fileno != 0 && len != 0 && !dir_emit(ctx, name.as_ptr(), len as usize, (*vdir).d_fileno, CDT2DT((*vdir).d_type)) { break; }
        (*ctx).pos += (*vdir).d_reclen as i64;
    }
    kfree(vdir); 0
}

unsafe fn coda_readdir(coda_file: *mut file, ctx: *mut dir_context) -> i32 {
    let host_file = (*coda_ftoc(coda_file)).cfi_container; let ret = iterate_dir(host_file, ctx);
    if ret != -ENOTDIR { ret } else { coda_venus_readdir(coda_file, ctx) }
}

unsafe fn coda_dentry_revalidate(dir: *mut inode, name: *const qstr, de: *mut dentry, flags: u32) -> i32 {
    if (flags & LOOKUP_RCU) != 0 { return -ECHILD; }
    let inode = d_inode(de); if inode.is_null() || is_root_inode(inode) { return 1; }
    let cii = ITOC(inode); if ((*cii).c_flags & (C_PURGE | C_FLUSH)) == 0 { return 1; }
    shrink_dcache_parent(de); if ((*cii).c_flags & C_FLUSH) != 0 { coda_flag_inode_children(inode, C_FLUSH); }
    if d_count(de) > 1 { return 1; }
    spin_lock(&mut (*cii).c_lock); (*cii).c_flags &= !(C_VATTR | C_PURGE | C_FLUSH); spin_unlock(&mut (*cii).c_lock); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
