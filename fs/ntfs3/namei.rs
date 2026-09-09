// SPDX-License-Identifier: GPL-2.0
/*
 *
 * Copyright (C) 2019-2021 Paragon Software GmbH, All rights reserved.
 *
 */

/* Linux and ntfs3 dependencies are supplied by the surrounding translation unit. */

/*
 * fill_name_de - Format NTFS_DE in @buf.
 */
pub unsafe fn fill_name_de(sbi: *mut ntfs_sb_info, buf: *mut core::ffi::c_void,
                           name: *const qstr, uni: *const cpu_str) -> i32 {
    let e = buf as *mut NTFS_DE;
    let fname = (e.add(1)) as *mut ATTR_FILE_NAME;
    let mut data_size: u16;
    let mut real_size: u16;
    let mut aligned_size: u16;

    #[cfg(not(CONFIG_NTFS3_64BIT_CLUSTER))]
    {
        (*e).ref_.high = 0;
        (*fname).home.high = 0;
    }

    if !uni.is_null() {
        #[cfg(target_endian = "big")]
        {
            let mut ulen = (*uni).len;
            let mut uname = (*fname).name.as_mut_ptr();
            let mut name_cpu = (*uni).name;
            while ulen != 0 {
                *uname = cpu_to_le16(*name_cpu);
                uname = uname.add(1);
                name_cpu = name_cpu.add(1);
                ulen -= 1;
            }
        }
        #[cfg(target_endian = "little")]
        core::ptr::copy_nonoverlapping((*uni).name, (*fname).name.as_mut_ptr(),
                                       (*uni).len as usize);
        (*fname).name_len = (*uni).len;
    } else {
        let err = ntfs_nls_to_utf16(sbi, (*name).name, (*name).len,
                                    &mut (*fname).name_len as *mut _ as *mut cpu_str,
                                    NTFS_NAME_LEN, UTF16_LITTLE_ENDIAN);
        if err < 0 { return err; }
    }

    (*fname).type_ = FILE_NAME_POSIX;
    data_size = fname_full_size(fname);
    real_size = data_size.wrapping_add(core::mem::size_of::<NTFS_DE>() as u16);
    aligned_size = ((data_size as u32 + 7) & !7) as u16;
    aligned_size = aligned_size.wrapping_add(core::mem::size_of::<NTFS_DE>() as u16);
    if aligned_size > real_size {
        core::ptr::write_bytes((buf as *mut u8).add(real_size as usize), 0,
                               (aligned_size - real_size) as usize);
    }
    (*e).size = cpu_to_le16(aligned_size);
    (*e).key_size = cpu_to_le16(data_size);
    (*e).flags = 0;
    (*e).res = 0;
    0
}

/* ntfs_lookup - inode_operations::lookup */
unsafe fn ntfs_lookup(dir: *mut inode, dentry: *mut dentry, flags: u32) -> *mut dentry {
    let ni = ntfs_i(dir);
    let uni = kmalloc(PATH_MAX, GFP_KERNEL) as *mut cpu_str;
    if uni.is_null() { return ERR_PTR(-ENOMEM); }
    let mut err = ntfs_nls_to_utf16((*ni).mi.sbi, (*dentry).d_name.name,
                                    (*dentry).d_name.len, uni, NTFS_NAME_LEN,
                                    UTF16_HOST_ENDIAN);
    if err < 0 { kfree(uni as *mut _); return ERR_PTR(err); }
    ni_lock_dir(ni);
    let inode = dir_search_flags(dir, uni, core::ptr::null_mut(), flags);
    ni_unlock(ni);
    kfree(uni as *mut _);
    if !IS_ERR_OR_NULL(inode) && (*inode).i_op.is_null() {
        iput(inode); return ERR_PTR(-EINVAL);
    }
    d_splice_alias(inode, dentry)
}

/* ntfs_create - inode_operations::create */
unsafe fn ntfs_create(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> i32 {
    ntfs_create_inode(idmap, dir, dentry, core::ptr::null_mut(), S_IFREG | mode, 0, core::ptr::null(), 0, core::ptr::null_mut())
}

/* ntfs_mknod - inode_operations::mknod */
unsafe fn ntfs_mknod(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t, rdev: dev_t) -> i32 {
    ntfs_create_inode(idmap, dir, dentry, core::ptr::null_mut(), mode, rdev, core::ptr::null(), 0, core::ptr::null_mut())
}

/* ntfs_link - inode_operations::link */
unsafe fn ntfs_link(ode: *mut dentry, dir: *mut inode, de: *mut dentry) -> i32 {
    let inode = d_inode(ode); let ni = ntfs_i(inode);
    if S_ISDIR((*inode).i_mode) { return -EPERM; }
    if (*inode).i_nlink >= NTFS_LINK_MAX { return -EMLINK; }
    ni_lock_dir(ntfs_i(dir));
    if inode != dir { ni_lock(ni); }
    inc_nlink(inode); ihold(inode);
    let err = ntfs_link_inode(inode, de);
    if err == 0 {
        inode_set_ctime_current(inode);
        inode_set_mtime_to_ts(dir, inode_set_ctime_current(dir));
        mark_inode_dirty(inode); mark_inode_dirty(dir); d_instantiate(de, inode);
    } else { drop_nlink(inode); iput(inode); }
    if inode != dir { ni_unlock(ni); } ni_unlock(ntfs_i(dir)); err
}

/* ntfs_unlink - inode_operations::unlink */
unsafe fn ntfs_unlink(dir: *mut inode, dentry: *mut dentry) -> i32 {
    let dir_ni = ntfs_i(dir); let inode = d_inode(dentry); let ni = ntfs_i(inode);
    if unlikely(is_bad_ni(ni)) { return -EINVAL; }
    if unlikely(ntfs3_forced_shutdown((*dir).i_sb)) { return -EIO; }
    if likely(is_ni_base(ni)) {
        ni_lock_dir(dir_ni); let err = ntfs_unlink_inode(dir, dentry); ni_unlock(dir_ni); err
    } else {
        ni_lock(ni);
        let err = ni_remove_attr(ni, ATTR_DATA, (*ni).file.ads.name, (*ni).file.ads.len, false, core::ptr::null_mut());
        ni_unlock(ni); if err == 0 { drop_nlink(inode); } err
    }
}

/* ntfs_symlink - inode_operations::symlink */
unsafe fn ntfs_symlink(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, symname: *const i8) -> i32 {
    let size = strlen(symname) as u32;
    if unlikely(is_bad_ni(ntfs_i(dir))) { return -EINVAL; }
    if unlikely(ntfs3_forced_shutdown((*dir).i_sb)) { return -EIO; }
    ntfs_create_inode(idmap, dir, dentry, core::ptr::null_mut(), S_IFLNK | 0o777, 0, symname, size, core::ptr::null_mut())
}

/* ntfs_mkdir - inode_operations::mkdir */
unsafe fn ntfs_mkdir(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> *mut dentry {
    ERR_PTR(ntfs_create_inode(idmap, dir, dentry, core::ptr::null_mut(), mode, 0, core::ptr::null(), 0, core::ptr::null_mut()))
}

/* ntfs_rmdir - inode_operations::rmdir */
unsafe fn ntfs_rmdir(dir: *mut inode, dentry: *mut dentry) -> i32 {
    let ni = ntfs_i(dir);
    if unlikely(is_bad_ni(ni)) { return -EINVAL; }
    if unlikely(ntfs3_forced_shutdown((*dir).i_sb)) { return -EIO; }
    ni_lock_dir(ni); let err = ntfs_unlink_inode(dir, dentry); ni_unlock(ni); err
}

/* ntfs_rename - inode_operations::rename */
unsafe fn ntfs_rename(_idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry,
                     new_dir: *mut inode, new_dentry: *mut dentry, flags: u32) -> i32 {
    let sb = (*dir).i_sb; let sbi = (*sb).s_fs_info; let dir_ni = ntfs_i(dir); let new_dir_ni = ntfs_i(new_dir);
    let inode = d_inode(dentry); let ni = ntfs_i(inode); let new_inode = d_inode(new_dentry);
    if !is_ni_base(ni) { return -EOPNOTSUPP; }
    if unlikely(is_bad_ni(ni)) { return -EINVAL; }
    if unlikely(ntfs3_forced_shutdown(sb)) { return -EIO; }
    if flags & !RENAME_NOREPLACE != 0 { return -EINVAL; }
    let is_same = (*dentry).d_name.len == (*new_dentry).d_name.len &&
        !memcmp((*dentry).d_name.name, (*new_dentry).d_name.name, (*dentry).d_name.len as usize);
    if is_same && dir == new_dir { return 0; }
    if ntfs_is_meta_file(sbi, (*inode).i_ino) { return -EINVAL; }
    if !new_inode.is_null() {
        dget(new_dentry); ni_lock_dir(new_dir_ni); let err = ntfs_unlink_inode(new_dir, new_dentry); ni_unlock(new_dir_ni); dput(new_dentry); if err != 0 { return err; }
    }
    let de = kmalloc(PATH_MAX, GFP_KERNEL) as *mut NTFS_DE;
    if de.is_null() { return -ENOMEM; }
    let mut err = fill_name_de(sbi, de as *mut _, &(*dentry).d_name, core::ptr::null());
    if err < 0 { kfree(de as *mut _); return err; }
    let new_de = if is_same { de } else { let p = (de as *mut u8).add(2048) as *mut NTFS_DE; err = fill_name_de(sbi, p as *mut _, &(*new_dentry).d_name, core::ptr::null()); if err < 0 { kfree(de as *mut _); return err; } p };
    ni_lock_dir(dir_ni); ni_lock(ni); if dir_ni != new_dir_ni { ni_lock_dir2(new_dir_ni); }
    err = ni_rename(dir_ni, new_dir_ni, ni, de, new_de);
    if err == 0 { simple_rename_timestamp(dir, dentry, new_dir, new_dentry); mark_inode_dirty(inode); mark_inode_dirty(dir); if dir != new_dir { mark_inode_dirty(new_dir); } if IS_DIRSYNC(dir) { ntfs_sync_inode(dir); } if IS_DIRSYNC(new_dir) { ntfs_sync_inode(new_dir); } }
    if dir_ni != new_dir_ni { ni_unlock(new_dir_ni); } ni_unlock(ni); ni_unlock(dir_ni); kfree(de as *mut _); err
}

unsafe fn ntfs3_get_parent(child: *mut dentry) -> *mut dentry {
    let inode = d_inode(child); let ni = ntfs_i(inode); let mut le: *mut ATTR_LIST_ENTRY = core::ptr::null_mut(); let mut attr: *mut ATTRIB = core::ptr::null_mut();
    loop { attr = ni_find_attr(ni, attr, &mut le, ATTR_NAME, core::ptr::null(), 0, core::ptr::null_mut(), core::ptr::null_mut()); if attr.is_null() { break; } let fname = resident_data_ex(attr, SIZEOF_ATTRIBUTE_FILENAME); if fname.is_null() { continue; } return d_obtain_alias(ntfs_iget5((*inode).i_sb, &(*fname).home, core::ptr::null_mut())); }
    ERR_PTR(-ENOENT)
}

/* dentry_operations::d_hash */
unsafe fn ntfs_d_hash(dentry: *const dentry, name: *mut qstr) -> i32 {
    let mut hash = init_name_hash(dentry); let mut n = (*name).name; let mut len = (*name).len; let mut c: u32;
    loop { if len == 0 { (*name).hash = end_name_hash(hash); return 0; } len -= 1; c = *n as u8 as u32; n = n.add(1); if c >= 0x80 { break; } hash = partial_name_hash(toupper(c), hash); }
    let sbi = (*dentry).d_sb.as_ref().unwrap().s_fs_info; let uni = kmalloc(PATH_MAX, GFP_NOWAIT) as *mut cpu_str; if uni.is_null() { return -ENOMEM; }
    let mut err = ntfs_nls_to_utf16(sbi, (*name).name, (*name).len, uni, NTFS_NAME_LEN, UTF16_HOST_ENDIAN);
    if err < 0 { kfree(uni as *mut _); return err; } if err == 0 { kfree(uni as *mut _); return -EINVAL; }
    hash = ntfs_names_hash((*uni).name, (*uni).len, (*sbi).upcase, init_name_hash(dentry)); (*name).hash = end_name_hash(hash); kfree(uni as *mut _); 0
}

/* dentry_operations::d_compare */
unsafe fn ntfs_d_compare(dentry: *const dentry, len1: u32, str_: *const i8, name: *const qstr) -> i32 {
    let mut n1 = str_; let mut n2 = (*name).name; let mut lm = core::cmp::min(len1, (*name).len); let mut c1: u8; let mut c2: u8;
    loop { if lm == 0 { return (len1 != (*name).len) as i32; } lm -= 1; c1 = *n1 as u8; c2 = *n2 as u8; n1 = n1.add(1); n2 = n2.add(1); if c1 == c2 { continue; } if c1 >= 0x80 || c2 >= 0x80 { break; } if toupper(c1 as u32) != toupper(c2 as u32) { return 1; } }
    let sbi = (*dentry).d_sb.as_ref().unwrap().s_fs_info; let uni1 = kmalloc(PATH_MAX, GFP_NOWAIT) as *mut cpu_str; if uni1.is_null() { return -ENOMEM; }
    let mut ret = ntfs_nls_to_utf16(sbi, str_, len1, uni1, NTFS_NAME_LEN, UTF16_HOST_ENDIAN); if ret < 0 { kfree(uni1 as *mut _); return ret; } if ret == 0 { kfree(uni1 as *mut _); return -EINVAL; }
    let uni2 = (uni1 as *mut u8).add(2048) as *mut le_str; ret = ntfs_nls_to_utf16(sbi, (*name).name, (*name).len, uni2 as *mut cpu_str, NTFS_NAME_LEN, UTF16_LITTLE_ENDIAN); if ret < 0 { kfree(uni1 as *mut _); return ret; } if ret == 0 { kfree(uni1 as *mut _); return -EINVAL; }
    ret = (!ntfs_cmp_names_cpu(uni1, uni2, (*sbi).upcase, false)) as i32; kfree(uni1 as *mut _); ret
}

pub static mut ntfs_dir_inode_operations: inode_operations = inode_operations { lookup: Some(ntfs_lookup), create: Some(ntfs_create), link: Some(ntfs_link), unlink: Some(ntfs_unlink), symlink: Some(ntfs_symlink), mkdir: Some(ntfs_mkdir), rmdir: Some(ntfs_rmdir), mknod: Some(ntfs_mknod), rename: Some(ntfs_rename), get_acl: Some(ntfs_get_acl), set_acl: Some(ntfs_set_acl), setattr: Some(ntfs_setattr), getattr: Some(ntfs_getattr), listxattr: Some(ntfs_listxattr), fiemap: Some(ntfs_fiemap), fileattr_get: Some(ntfs_fileattr_get), fileattr_set: Some(ntfs_fileattr_set) };
pub static mut ntfs_special_inode_operations: inode_operations = inode_operations { setattr: Some(ntfs_setattr), getattr: Some(ntfs_getattr), listxattr: Some(ntfs_listxattr), get_acl: Some(ntfs_get_acl), set_acl: Some(ntfs_set_acl), fileattr_get: Some(ntfs_fileattr_get), fileattr_set: Some(ntfs_fileattr_set) };
pub static mut ntfs_dentry_ops: dentry_operations = dentry_operations { d_hash: Some(ntfs_d_hash), d_compare: Some(ntfs_d_compare) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
