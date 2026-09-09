// SPDX-License-Identifier: MIT
/*
 * VirtualBox Guest Shared Folders support: Utility functions.
 * Mainly conversion from/to VirtualBox/Linux data structures.
 *
 * Copyright (C) 2006-2018 Oracle Corporation
 */

// Kernel and vfsmod declarations are supplied by the surrounding translation unit.

pub unsafe fn vboxsf_new_inode(sb: *mut super_block) -> *mut inode {
    let sbi = VBOXSF_SBI(sb);
    let inode = new_inode(sb);
    if inode.is_null() { return ERR_PTR(-ENOMEM); }

    idr_preload(GFP_KERNEL);
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*sbi).ino_idr_lock, &mut flags);
    let cursor = idr_get_cursor(&mut (*sbi).ino_idr);
    let ret = idr_alloc_cyclic(&mut (*sbi).ino_idr, inode as *mut c_void, 1, 0, GFP_ATOMIC);
    if ret >= 0 && ret < cursor { (*sbi).next_generation = (*sbi).next_generation.wrapping_add(1); }
    let gen = (*sbi).next_generation;
    spin_unlock_irqrestore(&mut (*sbi).ino_idr_lock, flags);
    idr_preload_end();

    if ret < 0 { iput(inode); return ERR_PTR(ret); }
    (*inode).i_ino = ret as _;
    (*inode).i_generation = gen;
    inode
}

/* set [inode] attributes based on [info], uid/gid based on [sbi] */
pub unsafe fn vboxsf_init_inode(sbi: *mut vboxsf_sbi, inode: *mut inode,
                                info: *const shfl_fsobjinfo, reinit: bool) -> c_int {
    let attr = &(*info).attr;
    let mut mode: umode_t = if attr.mode & SHFL_UNIX_IRUSR != 0 { S_IRUSR } else { 0 };
    mode |= if attr.mode & SHFL_UNIX_IWUSR != 0 { S_IWUSR } else { 0 };
    mode |= if attr.mode & SHFL_UNIX_IXUSR != 0 { S_IXUSR } else { 0 };
    mode |= if attr.mode & SHFL_UNIX_IRGRP != 0 { S_IRGRP } else { 0 };
    mode |= if attr.mode & SHFL_UNIX_IWGRP != 0 { S_IWGRP } else { 0 };
    mode |= if attr.mode & SHFL_UNIX_IXGRP != 0 { S_IXGRP } else { 0 };
    mode |= if attr.mode & SHFL_UNIX_IROTH != 0 { S_IROTH } else { 0 };
    mode |= if attr.mode & SHFL_UNIX_IWOTH != 0 { S_IWOTH } else { 0 };
    mode |= if attr.mode & SHFL_UNIX_IXOTH != 0 { S_IXOTH } else { 0 };

    (*inode).i_flags |= S_NOATIME | S_NOCMTIME;
    (*inode).i_mapping.as_mut().unwrap().a_ops = &vboxsf_reg_aops;
    if SHFL_IS_DIRECTORY(attr.mode) {
        if (*sbi).o.dmode_set { mode = (*sbi).o.dmode; }
        mode = (mode & !(*sbi).o.dmask) | S_IFDIR;
        if !reinit { (*inode).i_op = &vboxsf_dir_iops; (*inode).i_fop = &vboxsf_dir_fops; set_nlink(inode, 1); }
        else if !S_ISDIR((*inode).i_mode) { return -ESTALE; }
        (*inode).i_mode = mode;
    } else if SHFL_IS_SYMLINK(attr.mode) {
        if (*sbi).o.fmode_set { mode = (*sbi).o.fmode; }
        mode = (mode & !(*sbi).o.fmask) | S_IFLNK;
        if !reinit { (*inode).i_op = &vboxsf_lnk_iops; set_nlink(inode, 1); }
        else if !S_ISLNK((*inode).i_mode) { return -ESTALE; }
        (*inode).i_mode = mode;
    } else {
        if (*sbi).o.fmode_set { mode = (*sbi).o.fmode; }
        mode = (mode & !(*sbi).o.fmask) | S_IFREG;
        if !reinit { (*inode).i_op = &vboxsf_reg_iops; (*inode).i_fop = &vboxsf_reg_fops; set_nlink(inode, 1); }
        else if !S_ISREG((*inode).i_mode) { return -ESTALE; }
        (*inode).i_mode = mode;
    }
    (*inode).i_uid = (*sbi).o.uid; (*inode).i_gid = (*sbi).o.gid;
    (*inode).i_size = (*info).size; (*inode).i_blkbits = 12;
    let allocated = ((*info).allocated + 511) / 512;
    (*inode).i_blocks = allocated;
    inode_set_atime_to_ts(inode, ns_to_timespec64((*info).access_time.ns_relative_to_unix_epoch));
    inode_set_ctime_to_ts(inode, ns_to_timespec64((*info).change_time.ns_relative_to_unix_epoch));
    inode_set_mtime_to_ts(inode, ns_to_timespec64((*info).modification_time.ns_relative_to_unix_epoch));
    0
}

pub unsafe fn vboxsf_create_at_dentry(dentry: *mut dentry, params: *mut shfl_createparms) -> c_int {
    let sbi = VBOXSF_SBI((*dentry).d_sb);
    let path = vboxsf_path_from_dentry(sbi, dentry);
    if IS_ERR(path) { return PTR_ERR(path); }
    let err = vboxsf_create((*sbi).root, path, params);
    __putname(path as *mut c_void); err
}

pub unsafe fn vboxsf_stat(sbi: *mut vboxsf_sbi, path: *mut shfl_string, info: *mut shfl_fsobjinfo) -> c_int {
    let mut params: shfl_createparms = core::mem::zeroed();
    params.handle = SHFL_HANDLE_NIL; params.create_flags = SHFL_CF_LOOKUP | SHFL_CF_ACT_FAIL_IF_NEW;
    let err = vboxsf_create((*sbi).root, path, &mut params);
    if err != 0 { return err; }
    if params.result != SHFL_FILE_EXISTS { return -ENOENT; }
    if !info.is_null() { *info = params.info; } 0
}

pub unsafe fn vboxsf_stat_dentry(dentry: *mut dentry, info: *mut shfl_fsobjinfo) -> c_int {
    let sbi = VBOXSF_SBI((*dentry).d_sb); let path = vboxsf_path_from_dentry(sbi, dentry);
    if IS_ERR(path) { return PTR_ERR(path); }
    let err = vboxsf_stat(sbi, path, info); __putname(path as *mut c_void); err
}

pub unsafe fn vboxsf_inode_revalidate(dentry: *mut dentry) -> c_int {
    if dentry.is_null() || !d_really_is_positive(dentry) { return -EINVAL; }
    let inode = d_inode(dentry); let prev_mtime = inode_get_mtime(inode);
    let sf_i = VBOXSF_I(inode); let sbi = VBOXSF_SBI((*dentry).d_sb);
    if !(*sf_i).force_restat && time_before(jiffies, (*dentry).d_time + (*sbi).o.ttl) { return 0; }
    let mut info: shfl_fsobjinfo = core::mem::zeroed(); let err = vboxsf_stat_dentry(dentry, &mut info);
    if err != 0 { return err; }
    (*dentry).d_time = jiffies; (*sf_i).force_restat = 0;
    let err = vboxsf_init_inode(sbi, inode, &info, true); if err != 0 { return err; }
    let mtime = inode_get_mtime(inode);
    if timespec64_compare(&mtime, &prev_mtime) > 0 { invalidate_inode_pages2((*inode).i_mapping); }
    0
}

pub unsafe fn vboxsf_getattr(_idmap: *mut mnt_idmap, path: *const path, kstat: *mut kstat,
                             request_mask: u32, flags: c_uint) -> c_int {
    let dentry = (*path).dentry; let inode = d_inode(dentry); let sf_i = VBOXSF_I(inode);
    let err = match flags & AT_STATX_SYNC_TYPE { AT_STATX_DONT_SYNC => 0, AT_STATX_FORCE_SYNC => { (*sf_i).force_restat = 1; vboxsf_inode_revalidate(dentry) }, _ => vboxsf_inode_revalidate(dentry) };
    if err != 0 { return err; } generic_fillattr(&nop_mnt_idmap, request_mask, inode, kstat); 0
}

pub unsafe fn vboxsf_setattr(_idmap: *mut mnt_idmap, dentry: *mut dentry, iattr: *mut iattr) -> c_int {
    let sf_i = VBOXSF_I(d_inode(dentry)); let sbi = VBOXSF_SBI((*dentry).d_sb);
    let mut params: shfl_createparms = core::mem::zeroed(); let mut info: shfl_fsobjinfo = core::mem::zeroed();
    params.handle = SHFL_HANDLE_NIL; params.create_flags = SHFL_CF_ACT_OPEN_IF_EXISTS | SHFL_CF_ACT_FAIL_IF_NEW | SHFL_CF_ACCESS_ATTR_WRITE;
    if (*iattr).ia_valid & ATTR_SIZE != 0 { params.create_flags |= SHFL_CF_ACCESS_WRITE; }
    let err = vboxsf_create_at_dentry(dentry, &mut params); if err != 0 || params.result != SHFL_FILE_EXISTS { return if err != 0 { err } else { -ENOENT }; }
    if (*iattr).ia_valid & (ATTR_MODE | ATTR_ATIME | ATTR_MTIME) != 0 {
        if (*iattr).ia_valid & ATTR_MODE != 0 {
            info.attr.mode = if (*iattr).ia_mode & S_IRUSR != 0 { SHFL_UNIX_IRUSR } else { 0 } | if (*iattr).ia_mode & S_IWUSR != 0 { SHFL_UNIX_IWUSR } else { 0 } | if (*iattr).ia_mode & S_IXUSR != 0 { SHFL_UNIX_IXUSR } else { 0 } | if (*iattr).ia_mode & S_IRGRP != 0 { SHFL_UNIX_IRGRP } else { 0 } | if (*iattr).ia_mode & S_IWGRP != 0 { SHFL_UNIX_IWGRP } else { 0 } | if (*iattr).ia_mode & S_IXGRP != 0 { SHFL_UNIX_IXGRP } else { 0 } | if (*iattr).ia_mode & S_IROTH != 0 { SHFL_UNIX_IROTH } else { 0 } | if (*iattr).ia_mode & S_IWOTH != 0 { SHFL_UNIX_IWOTH } else { 0 } | if (*iattr).ia_mode & S_IXOTH != 0 { SHFL_UNIX_IXOTH } else { 0 };
            info.attr.mode |= if (*iattr).ia_mode & S_IFDIR != 0 { SHFL_TYPE_DIRECTORY } else { SHFL_TYPE_FILE };
        }
        if (*iattr).ia_valid & ATTR_ATIME != 0 { info.access_time.ns_relative_to_unix_epoch = timespec64_to_ns(&(*iattr).ia_atime); }
        if (*iattr).ia_valid & ATTR_MTIME != 0 { info.modification_time.ns_relative_to_unix_epoch = timespec64_to_ns(&(*iattr).ia_mtime); }
        let mut buf_len = core::mem::size_of::<shfl_fsobjinfo>() as u32;
        let err = vboxsf_fsinfo((*sbi).root, params.handle, SHFL_INFO_SET | SHFL_INFO_FILE, &mut buf_len, &mut info);
        if err != 0 { vboxsf_close((*sbi).root, params.handle); return err; } (*sf_i).force_restat = 1;
    }
    if (*iattr).ia_valid & ATTR_SIZE != 0 {
        core::ptr::write_bytes(&mut info, 0, 1); info.size = (*iattr).ia_size;
        let mut buf_len = core::mem::size_of::<shfl_fsobjinfo>() as u32;
        let err = vboxsf_fsinfo((*sbi).root, params.handle, SHFL_INFO_SET | SHFL_INFO_SIZE, &mut buf_len, &mut info);
        if err != 0 { vboxsf_close((*sbi).root, params.handle); return err; } (*sf_i).force_restat = 1;
    }
    vboxsf_close((*sbi).root, params.handle); if (*sf_i).force_restat { vboxsf_inode_revalidate(dentry); } 0
}

/* dentry_path conversion and directory helpers retain the kernel ABI and external helpers. */
pub unsafe fn vboxsf_path_from_dentry(sbi: *mut vboxsf_sbi, dentry: *mut dentry) -> *mut shfl_string {
    let buf = __getname(); if buf.is_null() { return ERR_PTR(-ENOMEM); }
    let path = dentry_path_raw(dentry, buf, PATH_MAX); if IS_ERR(path) { __putname(buf); return ERR_CAST(path); }
    let path_len = strlen(path);
    if !(*sbi).nls.is_null() {
        let shfl_path = __getname() as *mut shfl_string; if shfl_path.is_null() { __putname(buf); return ERR_PTR(-ENOMEM); }
        let mut out = (*shfl_path).string.utf8; let mut out_len = PATH_MAX - SHFLSTRING_HEADER_SIZE - 1; let mut p = path; let mut left = path_len;
        while left != 0 { let mut uni: wchar_t = 0; let nb = (*(*sbi).nls).char2uni(p, left, &mut uni); if nb < 0 { __putname(shfl_path as *mut c_void); __putname(buf); return ERR_PTR(-EINVAL); } p = p.add(nb as usize); left -= nb as usize; let nb = utf32_to_utf8(uni, out, out_len); if nb < 0 { __putname(shfl_path as *mut c_void); __putname(buf); return ERR_PTR(-ENAMETOOLONG); } out = out.add(nb as usize); out_len -= nb as usize; }
        *out = 0; (*shfl_path).length = out.offset_from((*shfl_path).string.utf8) as _; (*shfl_path).size = (*shfl_path).length + 1; __putname(buf); shfl_path
    } else { if SHFLSTRING_HEADER_SIZE + path_len + 1 > PATH_MAX { __putname(buf); return ERR_PTR(-ENAMETOOLONG); } let shfl_path = buf as *mut shfl_string; memmove((*shfl_path).string.utf8 as *mut c_void, path as *const c_void, path_len); *(*shfl_path).string.utf8.add(path_len) = 0; (*shfl_path).length = path_len as _; (*shfl_path).size = path_len as _ + 1; shfl_path }
}

pub unsafe fn vboxsf_nlscpy(sbi: *mut vboxsf_sbi, name: *mut c_char, name_bound_len: usize, utf8_name: *const u8, utf8_len: usize) -> c_int {
    let mut input = utf8_name as *const c_char; let mut in_len = utf8_len; let mut output = name; let mut out_len = name_bound_len - 1;
    while in_len != 0 { let mut uni: unicode_t = 0; let nb = utf8_to_utf32(input, in_len, &mut uni); if nb < 0 { return -EINVAL; } input = input.add(nb as usize); in_len -= nb as usize; let nb = (*(*sbi).nls).uni2char(uni, output, out_len); if nb < 0 { return nb; } output = output.add(nb as usize); out_len -= nb as usize; } *output = 0; 0
}

unsafe fn vboxsf_dir_buf_alloc(list: *mut list_head) -> *mut vboxsf_dir_buf { let b = kmalloc_obj::<vboxsf_dir_buf>(); if b.is_null() { return core::ptr::null_mut(); } (*b).buf = kmalloc(DIR_BUFFER_SIZE, GFP_KERNEL); if (*b).buf.is_null() { kfree(b as *mut c_void); return core::ptr::null_mut(); } (*b).entries = 0; (*b).used = 0; (*b).free = DIR_BUFFER_SIZE; list_add(&mut (*b).head, list); b }
unsafe fn vboxsf_dir_buf_free(b: *mut vboxsf_dir_buf) { list_del(&mut (*b).head); kfree((*b).buf); kfree(b as *mut c_void); }
pub unsafe fn vboxsf_dir_info_alloc() -> *mut vboxsf_dir_info { let p = kmalloc_obj::<vboxsf_dir_info>(); if p.is_null() { return core::ptr::null_mut(); } INIT_LIST_HEAD(&mut (*p).info_list); p }
pub unsafe fn vboxsf_dir_info_free(p: *mut vboxsf_dir_info) { let mut pos: *mut list_head = core::ptr::null_mut(); let mut tmp: *mut list_head = core::ptr::null_mut(); list_for_each_safe(pos, tmp, &mut (*p).info_list); while !pos.is_null() { let b = list_entry(pos, vboxsf_dir_buf, head); vboxsf_dir_buf_free(b); pos = tmp; } kfree(p as *mut c_void); }

pub unsafe fn vboxsf_dir_read_all(sbi: *mut vboxsf_sbi, sf_d: *mut vboxsf_dir_info, handle: u64) -> c_int { let mut err = 0; let mut b: *mut vboxsf_dir_buf = core::ptr::null_mut(); while err == 0 { b = vboxsf_dir_buf_alloc(&mut (*sf_d).info_list); if b.is_null() { err = -ENOMEM; break; } let mut entries = 0; let mut size = (*b).free; err = vboxsf_dirinfo((*sbi).root, handle, core::ptr::null_mut(), 0, 0, &mut size, (*b).buf, &mut entries); if err < 0 { break; } (*b).entries += entries; (*b).free -= size; (*b).used += size; } if !b.is_null() && (*b).used == 0 { vboxsf_dir_buf_free(b); } if err > 0 || err == -EILSEQ { err = 0; } err }
pub unsafe fn vboxsf_query_case_sensitive(sbi: *mut vboxsf_sbi) -> c_int { let mut volinfo: shfl_volinfo = core::mem::zeroed(); let mut len = core::mem::size_of::<shfl_volinfo>() as u32; let err = vboxsf_fsinfo((*sbi).root, 0, SHFL_INFO_GET | SHFL_INFO_VOLUME, &mut len, &mut volinfo); if err != 0 { return err; } if (len as usize) < core::mem::size_of::<shfl_volinfo>() { return 0; } (*sbi).case_insensitive = !volinfo.properties.case_sensitive; 0 }
pub unsafe fn vboxsf_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int { let sbi = VBOXSF_SBI((*dentry).d_sb); if (*sbi).case_insensitive { (*fa).fsx_xflags |= FS_XFLAG_CASEFOLD; (*fa).flags |= FS_CASEFOLD_FL; } 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
