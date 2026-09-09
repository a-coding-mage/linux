// SPDX-License-Identifier: GPL-2.0
/*
 * fs/f2fs/acl.c
 *
 * Copyright (c) 2012 Samsung Electronics Co., Ltd.
 *             http://www.samsung.com/
 *
 * Portions of this code from linux/fs/ext2/acl.c
 *
 * Copyright (C) 2001-2003 Andreas Gruenbacher, <agruen@suse.de>
 */
// External Linux/F2FS declarations are supplied by the surrounding crate.

#[inline]
unsafe fn f2fs_acl_size(count: i32) -> usize {
    if count <= 4 {
        core::mem::size_of::<f2fs_acl_header>() +
            (count as usize) * core::mem::size_of::<f2fs_acl_entry_short>()
    } else {
        core::mem::size_of::<f2fs_acl_header>() +
            4 * core::mem::size_of::<f2fs_acl_entry_short>() +
            ((count - 4) as usize) * core::mem::size_of::<f2fs_acl_entry>()
    }
}

#[inline]
unsafe fn f2fs_acl_count(mut size: usize) -> i32 {
    let s: isize;
    size -= core::mem::size_of::<f2fs_acl_header>();
    s = size as isize - 4 * core::mem::size_of::<f2fs_acl_entry_short>() as isize;
    if s < 0 {
        if size % core::mem::size_of::<f2fs_acl_entry_short>() != 0 { return -1; }
        (size / core::mem::size_of::<f2fs_acl_entry_short>()) as i32
    } else {
        if (s as usize) % core::mem::size_of::<f2fs_acl_entry>() != 0 { return -1; }
        (s as usize / core::mem::size_of::<f2fs_acl_entry>() + 4) as i32
    }
}

unsafe fn f2fs_acl_from_disk(value: *const i8, size: usize) -> *mut posix_acl {
    let mut i: i32;
    let mut count: i32;
    let mut err: i32 = -EINVAL;
    let acl: *mut posix_acl;
    let hdr = value as *mut f2fs_acl_header;
    let mut entry = hdr.add(1) as *mut f2fs_acl_entry;
    let end = value.add(size);

    if size < core::mem::size_of::<f2fs_acl_header>() { return ERR_PTR(-EINVAL); }
    if (*hdr).a_version != cpu_to_le32(F2FS_ACL_VERSION) { return ERR_PTR(-EINVAL); }
    count = f2fs_acl_count(size);
    if count < 0 { return ERR_PTR(-EINVAL); }
    if count == 0 { return core::ptr::null_mut(); }
    acl = posix_acl_alloc(count, GFP_NOFS);
    if acl.is_null() { return ERR_PTR(-ENOMEM); }

    i = 0;
    while i < count {
        if (entry as *mut i8).add(core::mem::size_of::<f2fs_acl_entry_short>()) > end {
            err = -EFSCORRUPTED; goto fail;
        }
        (*acl).a_entries.add(i as usize).write(posix_acl_entry {
            e_tag: le16_to_cpu((*entry).e_tag), e_perm: le16_to_cpu((*entry).e_perm), ..core::mem::zeroed()
        });
        match (*acl).a_entries.add(i as usize).read().e_tag {
            ACL_USER_OBJ | ACL_GROUP_OBJ | ACL_MASK | ACL_OTHER => {
                entry = (entry as *mut i8).add(core::mem::size_of::<f2fs_acl_entry_short>()) as *mut f2fs_acl_entry;
            }
            ACL_USER => {
                if (entry as *mut i8).add(core::mem::size_of::<f2fs_acl_entry>()) > end { err = -EFSCORRUPTED; goto fail; }
                (*acl).a_entries.add(i as usize).as_mut().unwrap_unchecked().e_uid = make_kuid(&init_user_ns, le32_to_cpu((*entry).e_id));
                entry = (entry as *mut i8).add(core::mem::size_of::<f2fs_acl_entry>()) as *mut f2fs_acl_entry;
            }
            ACL_GROUP => {
                if (entry as *mut i8).add(core::mem::size_of::<f2fs_acl_entry>()) > end { err = -EFSCORRUPTED; goto fail; }
                (*acl).a_entries.add(i as usize).as_mut().unwrap_unchecked().e_gid = make_kgid(&init_user_ns, le32_to_cpu((*entry).e_id));
                entry = (entry as *mut i8).add(core::mem::size_of::<f2fs_acl_entry>()) as *mut f2fs_acl_entry;
            }
            _ => { goto fail; }
        }
        i += 1;
    }
    if entry as *mut i8 != end { goto fail; }
    return acl;
fail:
    posix_acl_release(acl);
    ERR_PTR(err)
}

unsafe fn f2fs_acl_to_disk(sbi: *mut f2fs_sb_info, acl: *const posix_acl, size: *mut usize) -> *mut core::ffi::c_void {
    let f2fs_acl = f2fs_kmalloc(sbi, core::mem::size_of::<f2fs_acl_header>() + (*acl).a_count as usize * core::mem::size_of::<f2fs_acl_entry>(), GFP_NOFS) as *mut f2fs_acl_header;
    if f2fs_acl.is_null() { return ERR_PTR(-ENOMEM); }
    (*f2fs_acl).a_version = cpu_to_le32(F2FS_ACL_VERSION);
    let mut entry = f2fs_acl.add(1) as *mut f2fs_acl_entry;
    let mut i = 0;
    while i < (*acl).a_count {
        (*entry).e_tag = cpu_to_le16((*acl).a_entries.add(i as usize).read().e_tag);
        (*entry).e_perm = cpu_to_le16((*acl).a_entries.add(i as usize).read().e_perm);
        match (*acl).a_entries.add(i as usize).read().e_tag {
            ACL_USER => { (*entry).e_id = cpu_to_le32(from_kuid(&init_user_ns, (*acl).a_entries.add(i as usize).read().e_uid)); entry = (entry as *mut i8).add(core::mem::size_of::<f2fs_acl_entry>()) as *mut f2fs_acl_entry; }
            ACL_GROUP => { (*entry).e_id = cpu_to_le32(from_kgid(&init_user_ns, (*acl).a_entries.add(i as usize).read().e_gid)); entry = (entry as *mut i8).add(core::mem::size_of::<f2fs_acl_entry>()) as *mut f2fs_acl_entry; }
            ACL_USER_OBJ | ACL_GROUP_OBJ | ACL_MASK | ACL_OTHER => { entry = (entry as *mut i8).add(core::mem::size_of::<f2fs_acl_entry_short>()) as *mut f2fs_acl_entry; }
            _ => { kfree(f2fs_acl as *mut _); return ERR_PTR(-EINVAL); }
        }
        i += 1;
    }
    *size = f2fs_acl_size((*acl).a_count);
    f2fs_acl as *mut core::ffi::c_void
}

// The remaining ACL entry points retain the C implementation's external kernel calls and control flow.
unsafe fn __f2fs_get_acl(inode: *mut inode, typ: i32, dfolio: *mut folio) -> *mut posix_acl {
    let mut name_index = F2FS_XATTR_INDEX_POSIX_ACL_DEFAULT;
    if typ == ACL_TYPE_ACCESS { name_index = F2FS_XATTR_INDEX_POSIX_ACL_ACCESS; }
    let mut value: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut retval = f2fs_getxattr(inode, name_index, b"\0".as_ptr() as *const i8, core::ptr::null_mut(), 0, dfolio);
    if retval > 0 { value = f2fs_kmalloc(F2FS_I_SB(inode), retval as usize, GFP_F2FS_ZERO); if value.is_null() { return ERR_PTR(-ENOMEM); } retval = f2fs_getxattr(inode, name_index, b"\0".as_ptr() as *const i8, value, retval, dfolio); }
    let acl = if retval > 0 { f2fs_acl_from_disk(value as *const i8, retval as usize) } else if retval == -ENODATA { core::ptr::null_mut() } else { ERR_PTR(retval) };
    kfree(value); acl
}

pub unsafe fn f2fs_get_acl(inode: *mut inode, typ: i32, rcu: bool) -> *mut posix_acl { if rcu { ERR_PTR(-ECHILD) } else { __f2fs_get_acl(inode, typ, core::ptr::null_mut()) } }

unsafe fn f2fs_acl_update_mode(idmap: *mut mnt_idmap, inode: *mut inode, mode_p: *mut umode_t, acl: *mut *mut posix_acl) -> i32 {
    let mut mode = (*inode).i_mode;
    if is_inode_flag_set(inode, FI_ACL_MODE) { mode = (*F2FS_I(inode)).i_acl_mode; }
    let error = posix_acl_equiv_mode(*acl, &mut mode);
    if error < 0 { return error; }
    if error == 0 { *acl = core::ptr::null_mut(); }
    if !in_group_or_capable(idmap, inode, i_gid_into_vfsgid(idmap, inode)) { mode &= !S_ISGID; }
    *mode_p = mode; 0
}

unsafe fn __f2fs_set_acl(idmap: *mut mnt_idmap, inode: *mut inode, typ: i32, acl: *mut posix_acl, ifolio: *mut folio) -> i32 {
    let name_index: i32;
    let mut value: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut size = 0usize;
    let mut mode = (*inode).i_mode;
    match typ {
        ACL_TYPE_ACCESS => { name_index = F2FS_XATTR_INDEX_POSIX_ACL_ACCESS; if !acl.is_null() && ifolio.is_null() { let error = f2fs_acl_update_mode(idmap, inode, &mut mode, &mut (acl as *mut _)); if error != 0 { return error; } set_acl_inode(inode, mode); } }
        ACL_TYPE_DEFAULT => { name_index = F2FS_XATTR_INDEX_POSIX_ACL_DEFAULT; if !S_ISDIR((*inode).i_mode) { return if !acl.is_null() { -EACCES } else { 0 }; } }
        _ => return -EINVAL,
    }
    if !acl.is_null() { value = f2fs_acl_to_disk(F2FS_I_SB(inode), acl, &mut size); if IS_ERR(value) { clear_inode_flag(inode, FI_ACL_MODE); return PTR_ERR(value); } }
    let error = f2fs_setxattr(inode, name_index, b"\0".as_ptr() as *const i8, value, size, ifolio, 0);
    kfree(value); if error == 0 { set_cached_acl(inode, typ, acl); } clear_inode_flag(inode, FI_ACL_MODE); error
}

pub unsafe fn f2fs_set_acl(idmap: *mut mnt_idmap, dentry: *mut dentry, acl: *mut posix_acl, typ: i32) -> i32 {
    let inode = d_inode(dentry); if unlikely(f2fs_cp_error(F2FS_I_SB(inode))) { return -EIO; } __f2fs_set_acl(idmap, inode, typ, acl, core::ptr::null_mut())
}

/* Most part of f2fs_acl_clone, f2fs_acl_create_masq, f2fs_acl_create
 * are copied from posix_acl.c */
unsafe fn f2fs_acl_clone(acl: *const posix_acl, flags: gfp_t) -> *mut posix_acl {
    if acl.is_null() { return core::ptr::null_mut(); }
    let clone = kmemdup(acl as *const _, struct_size(acl, a_entries, (*acl).a_count), flags) as *mut posix_acl;
    if !clone.is_null() { refcount_set(&mut (*clone).a_refcount, 1); } clone
}

unsafe fn f2fs_acl_create_masq(acl: *mut posix_acl, mode_p: *mut umode_t) -> i32 {
    let mut pa: *mut posix_acl_entry; let mut group_obj = core::ptr::null_mut(); let mut mask_obj = core::ptr::null_mut(); let mut mode = *mode_p; let mut not_equiv = 0;
    let mut pe: *mut posix_acl_entry = core::ptr::null_mut();
    FOREACH_ACL_ENTRY(pa, pe, acl) {
        match (*pa).e_tag { ACL_USER_OBJ => { (*pa).e_perm &= (mode >> 6) | !S_IRWXO; mode &= ((*pa).e_perm << 6) | !S_IRWXU; }, ACL_USER | ACL_GROUP => not_equiv = 1, ACL_GROUP_OBJ => group_obj = pa, ACL_OTHER => { (*pa).e_perm &= mode | !S_IRWXO; mode &= (*pa).e_perm | !S_IRWXO; }, ACL_MASK => { mask_obj = pa; not_equiv = 1; }, _ => return -EIO }
    }
    let obj = if !mask_obj.is_null() { mask_obj } else if !group_obj.is_null() { group_obj } else { return -EIO };
    (*obj).e_perm &= (mode >> 3) | !S_IRWXO; mode &= ((*obj).e_perm << 3) | !S_IRWXG; *mode_p = (*mode_p & !S_IRWXUGO) | mode; not_equiv
}

unsafe fn f2fs_acl_create(dir: *mut inode, mode: *mut umode_t, default_acl: *mut *mut posix_acl, acl: *mut *mut posix_acl, dfolio: *mut folio) -> i32 {
    *acl = core::ptr::null_mut(); *default_acl = core::ptr::null_mut(); if S_ISLNK(*mode) || !IS_POSIXACL(dir) { return 0; }
    let p = __f2fs_get_acl(dir, ACL_TYPE_DEFAULT, dfolio); if p.is_null() || p == ERR_PTR(-EOPNOTSUPP) { *mode &= !current_umask(); return 0; } if IS_ERR(p) { return PTR_ERR(p); }
    let clone = f2fs_acl_clone(p, GFP_NOFS); if clone.is_null() { posix_acl_release(p); return -ENOMEM; }
    let ret = f2fs_acl_create_masq(clone, mode); if ret < 0 { posix_acl_release(clone); posix_acl_release(p); return ret; }
    if ret == 0 { posix_acl_release(clone); } else { *acl = clone; } if !S_ISDIR(*mode) { posix_acl_release(p); } else { *default_acl = p; } 0
}

pub unsafe fn f2fs_init_acl(inode: *mut inode, dir: *mut inode, ifolio: *mut folio, dfolio: *mut folio) -> i32 {
    let mut default_acl = core::ptr::null_mut(); let mut acl = core::ptr::null_mut(); let error = f2fs_acl_create(dir, &mut (*inode).i_mode, &mut default_acl, &mut acl, dfolio); if error != 0 { return error; }
    f2fs_mark_inode_dirty_sync(inode, true); let mut error = 0;
    if !default_acl.is_null() { error = __f2fs_set_acl(core::ptr::null_mut(), inode, ACL_TYPE_DEFAULT, default_acl, ifolio); posix_acl_release(default_acl); } else { (*inode).i_default_acl = core::ptr::null_mut(); }
    if !acl.is_null() { if error == 0 { error = __f2fs_set_acl(core::ptr::null_mut(), inode, ACL_TYPE_ACCESS, acl, ifolio); } posix_acl_release(acl); } else { (*inode).i_acl = core::ptr::null_mut(); } error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
