// SPDX-License-Identifier: MIT
/*
 * VirtualBox Guest Shared Folders support: Directory inode and file operations
 *
 * Copyright (C) 2006-2018 Oracle Corporation
 */

// Linux dependencies: namei.h, vbox_utils.h, and vfsmod.h.

unsafe fn vboxsf_dir_open(inode: *mut inode, file: *mut file) -> i32 {
    let sbi = VBOXSF_SBI((*inode).i_sb);
    let mut params: shfl_createparms = core::mem::zeroed();
    let sf_d = vboxsf_dir_info_alloc();
    if sf_d.is_null() { return -ENOMEM; }

    params.handle = SHFL_HANDLE_NIL;
    params.create_flags = SHFL_CF_DIRECTORY | SHFL_CF_ACT_OPEN_IF_EXISTS |
        SHFL_CF_ACT_FAIL_IF_NEW | SHFL_CF_ACCESS_READ;

    let mut err = vboxsf_create_at_dentry(file_dentry(file), &mut params);
    if err != 0 { vboxsf_dir_info_free(sf_d); return err; }
    if params.result != SHFL_FILE_EXISTS {
        err = -ENOENT;
        vboxsf_close((*sbi).root, params.handle);
        vboxsf_dir_info_free(sf_d);
        return err;
    }
    err = vboxsf_dir_read_all(sbi, sf_d, params.handle);
    if err != 0 {
        vboxsf_close((*sbi).root, params.handle);
        vboxsf_dir_info_free(sf_d);
        return err;
    }
    vboxsf_close((*sbi).root, params.handle);
    (*file).private_data = sf_d as *mut core::ffi::c_void;
    0
}

unsafe fn vboxsf_dir_release(_inode: *mut inode, file: *mut file) -> i32 {
    if !(*file).private_data.is_null() { vboxsf_dir_info_free((*file).private_data as *mut vboxsf_dir_info); }
    0
}

unsafe fn vboxsf_get_d_type(mode: u32) -> u32 {
    match mode & SHFL_TYPE_MASK {
        SHFL_TYPE_FIFO => DT_FIFO,
        SHFL_TYPE_DEV_CHAR => DT_CHR,
        SHFL_TYPE_DIRECTORY => DT_DIR,
        SHFL_TYPE_DEV_BLOCK => DT_BLK,
        SHFL_TYPE_FILE => DT_REG,
        SHFL_TYPE_SYMLINK => DT_LNK,
        SHFL_TYPE_SOCKET => DT_SOCK,
        SHFL_TYPE_WHITEOUT => DT_WHT,
        _ => DT_UNKNOWN,
    }
}

unsafe fn vboxsf_dir_emit(dir: *mut file, ctx: *mut dir_context) -> bool {
    let sbi = VBOXSF_SBI(file_inode(dir).read().i_sb);
    let sf_d = (*dir).private_data as *mut vboxsf_dir_info;
    let mut cur: i64 = 0;
    let mut b: *mut vboxsf_dir_buf;
    list_for_each_entry!(b, &mut (*sf_d).info_list, head) {
        'try_next_entry: loop {
            if (*ctx).pos >= cur + (*b).entries { cur += (*b).entries; break; }
            let mut info = (*b).buf as *mut shfl_dirinfo;
            let mut i: i64 = 0;
            while i < (*ctx).pos - cur {
                let end = (*info).name.string.utf8.as_mut_ptr().add((*info).name.size as usize);
                if WARN_ON(end > (*b).buf.add((*b).used as usize)) { return false; }
                info = end as *mut shfl_dirinfo; i += 1;
            }
            let end = (*info).name.string.utf8.as_mut_ptr().add((*info).name.size as usize);
            if WARN_ON(end > (*b).buf.add((*b).used as usize)) { return false; }
            let d_type = vboxsf_get_d_type((*info).info.attr.mode);
            if ( (*ctx).pos + 1) as ino_t != ((*ctx).pos + 1) as u64 { vbg_err(c"vboxsf: fake ino overflow, truncating dir\0".as_ptr()); return false; }
            let fake_ino = ((*ctx).pos + 1) as ino_t;
            if !(*sbi).nls.is_null() {
                let mut d_name = [0i8; NAME_MAX as usize];
                let err = vboxsf_nlscpy(sbi, d_name.as_mut_ptr(), NAME_MAX, (*info).name.string.utf8.as_mut_ptr(), (*info).name.length);
                if err != 0 { (*ctx).pos += 1; continue 'try_next_entry; }
                return dir_emit(ctx, d_name.as_ptr(), strlen(d_name.as_ptr()), fake_ino, d_type);
            }
            return dir_emit(ctx, (*info).name.string.utf8.as_mut_ptr(), (*info).name.length, fake_ino, d_type);
        }
    }
    false
}

unsafe fn vboxsf_dir_iterate(dir: *mut file, ctx: *mut dir_context) -> i32 {
    loop { if !vboxsf_dir_emit(dir, ctx) { break; } (*ctx).pos += 1; }
    0
}

WRAP_DIR_ITER!(vboxsf_dir_iterate); // FIXME!
const vboxsf_dir_fops: file_operations = file_operations {
    open: Some(vboxsf_dir_open), iterate_shared: Some(shared_vboxsf_dir_iterate),
    release: Some(vboxsf_dir_release), read: Some(generic_read_dir), llseek: Some(generic_file_llseek),
};

/*
 * This is called during name resolution/lookup to check if the @dentry in
 * the cache is still valid. the job is handled by vboxsf_inode_revalidate.
 */
unsafe fn vboxsf_dentry_revalidate(_dir: *mut inode, _name: *const qstr, dentry: *mut dentry, flags: u32) -> i32 {
    if flags & LOOKUP_RCU != 0 { return -ECHILD; }
    if d_really_is_positive(dentry) { (vboxsf_inode_revalidate(dentry) == 0) as i32 }
    else { (vboxsf_stat_dentry(dentry, core::ptr::null_mut()) == -ENOENT) as i32 }
}

const vboxsf_dentry_ops: dentry_operations = dentry_operations { d_revalidate: Some(vboxsf_dentry_revalidate) };

/* iops */

unsafe fn vboxsf_dir_lookup(parent: *mut inode, dentry: *mut dentry, _flags: u32) -> *mut dentry {
    let sbi = VBOXSF_SBI((*parent).i_sb); let mut fsinfo: shfl_fsobjinfo = core::mem::zeroed();
    (*dentry).d_time = jiffies;
    let err = vboxsf_stat_dentry(dentry, &mut fsinfo);
    let inode = if err != 0 { if err == -ENOENT { core::ptr::null_mut() } else { ERR_PTR(err) } }
    else { let i = vboxsf_new_inode((*parent).i_sb); if !IS_ERR(i) { vboxsf_init_inode(sbi, i, &fsinfo, false); } i };
    d_splice_alias(inode, dentry)
}

unsafe fn vboxsf_dir_instantiate(parent: *mut inode, dentry: *mut dentry, info: *mut shfl_fsobjinfo) -> i32 {
    let sbi = VBOXSF_SBI((*parent).i_sb); let inode = vboxsf_new_inode((*parent).i_sb);
    if IS_ERR(inode) { return PTR_ERR(inode); }
    (*VBOXSF_I(inode)).force_restat = 1; vboxsf_init_inode(sbi, inode, info, false); d_instantiate(dentry, inode); 0
}

unsafe fn vboxsf_dir_create(parent: *mut inode, dentry: *mut dentry, mode: umode_t, is_dir: bool, excl: bool, handle_ret: *mut u64) -> i32 {
    let sf_parent_i = VBOXSF_I(parent); let sbi = VBOXSF_SBI((*parent).i_sb); let mut params: shfl_createparms = core::mem::zeroed();
    params.handle = SHFL_HANDLE_NIL; params.create_flags = SHFL_CF_ACT_CREATE_IF_NEW | SHFL_CF_ACCESS_READWRITE;
    if is_dir { params.create_flags |= SHFL_CF_DIRECTORY; } if excl { params.create_flags |= SHFL_CF_ACT_FAIL_IF_EXISTS; }
    params.info.attr.mode = (mode & 0o777) | if is_dir { SHFL_TYPE_DIRECTORY } else { SHFL_TYPE_FILE }; params.info.attr.additional = SHFLFSOBJATTRADD_NOTHING;
    let mut err = vboxsf_create_at_dentry(dentry, &mut params); if err != 0 { return err; }
    if params.result != SHFL_FILE_CREATED { return -EPERM; }
    err = vboxsf_dir_instantiate(parent, dentry, &mut params.info); if err == 0 { (*sf_parent_i).force_restat = 1; }
    if err == 0 && !handle_ret.is_null() { *handle_ret = params.handle; } else { vboxsf_close((*sbi).root, params.handle); } err
}

unsafe fn vboxsf_dir_mkfile(_idmap: *mut mnt_idmap, parent: *mut inode, dentry: *mut dentry, mode: umode_t) -> i32 { vboxsf_dir_create(parent, dentry, mode, false, true, core::ptr::null_mut()) }
unsafe fn vboxsf_dir_mkdir(_idmap: *mut mnt_idmap, parent: *mut inode, dentry: *mut dentry, mode: umode_t) -> *mut dentry { ERR_PTR(vboxsf_dir_create(parent, dentry, mode, true, true, core::ptr::null_mut())) }

unsafe fn vboxsf_dir_atomic_open(parent: *mut inode, dentry: *mut dentry, file: *mut file, flags: u32, mode: umode_t) -> i32 {
    let sbi = VBOXSF_SBI((*parent).i_sb); let mut handle = 0u64;
    if d_in_lookup(dentry) { let res = vboxsf_dir_lookup(parent, dentry, 0); if !res.is_null() || d_really_is_positive(dentry) { return finish_no_open(file, res); } }
    if flags & O_CREAT == 0 { return finish_no_open(file, core::ptr::null_mut()); }
    let mut err = vboxsf_dir_create(parent, dentry, mode, false, flags & O_EXCL != 0, &mut handle); if err != 0 { return err; }
    let sf_handle = vboxsf_create_sf_handle(d_inode(dentry), handle, SHFL_CF_ACCESS_READWRITE); if IS_ERR(sf_handle) { vboxsf_close((*sbi).root, handle); return PTR_ERR(sf_handle); }
    err = finish_open(file, dentry, generic_file_open); if err != 0 { vboxsf_release_sf_handle(d_inode(dentry), sf_handle); return err; }
    (*file).private_data = sf_handle as *mut core::ffi::c_void; (*file).f_mode |= FMODE_CREATED; 0
}

unsafe fn vboxsf_dir_unlink(parent: *mut inode, dentry: *mut dentry) -> i32 {
    let sbi = VBOXSF_SBI((*parent).i_sb); let sf_parent_i = VBOXSF_I(parent); let inode = d_inode(dentry);
    let mut flags = if S_ISDIR((*inode).i_mode) { SHFL_REMOVE_DIR } else { SHFL_REMOVE_FILE }; if S_ISLNK((*inode).i_mode) { flags |= SHFL_REMOVE_SYMLINK; }
    let path = vboxsf_path_from_dentry(sbi, dentry); if IS_ERR(path) { return PTR_ERR(path); }
    let err = vboxsf_remove((*sbi).root, path, flags); __putname(path); if err != 0 { return err; } (*sf_parent_i).force_restat = 1; 0
}

unsafe fn vboxsf_dir_rename(_idmap: *mut mnt_idmap, old_parent: *mut inode, old_dentry: *mut dentry, new_parent: *mut inode, new_dentry: *mut dentry, flags: u32) -> i32 {
    if flags != 0 { return -EINVAL; }
    let sbi = VBOXSF_SBI((*old_parent).i_sb); let old_i = VBOXSF_I(old_parent); let new_i = VBOXSF_I(new_parent); let mut shfl_flags = SHFL_RENAME_FILE | SHFL_RENAME_REPLACE_IF_EXISTS;
    let old_path = vboxsf_path_from_dentry(sbi, old_dentry); if IS_ERR(old_path) { return PTR_ERR(old_path); }
    let new_path = vboxsf_path_from_dentry(sbi, new_dentry); if IS_ERR(new_path) { __putname(old_path); return PTR_ERR(new_path); }
    if (*d_inode(old_dentry)).i_mode & S_IFDIR != 0 { shfl_flags = 0; }
    let err = vboxsf_rename((*sbi).root, old_path, new_path, shfl_flags); if err == 0 { (*new_i).force_restat = 1; (*old_i).force_restat = 1; } __putname(new_path); __putname(old_path); err
}

unsafe fn vboxsf_dir_symlink(_idmap: *mut mnt_idmap, parent: *mut inode, dentry: *mut dentry, symname: *const i8) -> i32 {
    let sf_parent_i = VBOXSF_I(parent); let sbi = VBOXSF_SBI((*parent).i_sb); let symname_size = strlen(symname) + 1;
    let path = vboxsf_path_from_dentry(sbi, dentry); if IS_ERR(path) { return PTR_ERR(path); }
    let ssymname = kmalloc(SHFLSTRING_HEADER_SIZE + symname_size, GFP_KERNEL); if ssymname.is_null() { __putname(path); return -ENOMEM; }
    (*ssymname).length = symname_size - 1; (*ssymname).size = symname_size; memcpy((*ssymname).string.utf8.as_mut_ptr(), symname, symname_size);
    let mut info: shfl_fsobjinfo = core::mem::zeroed(); let err = vboxsf_symlink((*sbi).root, path, ssymname, &mut info); kfree(ssymname); __putname(path);
    if err != 0 { return if err == -EROFS { -EPERM } else { err }; }
    let err = vboxsf_dir_instantiate(parent, dentry, &mut info); if err != 0 { return err; } (*sf_parent_i).force_restat = 1; 0
}

const vboxsf_dir_iops: inode_operations = inode_operations {
    lookup: Some(vboxsf_dir_lookup), create: Some(vboxsf_dir_mkfile), mkdir: Some(vboxsf_dir_mkdir), atomic_open: Some(vboxsf_dir_atomic_open),
    rmdir: Some(vboxsf_dir_unlink), unlink: Some(vboxsf_dir_unlink), rename: Some(vboxsf_dir_rename), symlink: Some(vboxsf_dir_symlink),
    getattr: Some(vboxsf_getattr), setattr: Some(vboxsf_setattr), fileattr_get: Some(vboxsf_fileattr_get),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
