// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/ext4/acl.c
 *
 * Copyright (C) 2001-2003 Andreas Gruenbacher, <agruen@suse.de>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/quotaops.h, ext4_jbd2.h, ext4.h, xattr.h, and acl.h.

/* Convert from filesystem to in-memory representation. */
unsafe fn ext4_acl_from_disk(value: *const core::ffi::c_void, size: usize) -> *mut posix_acl {
    let end = (value as *const u8).add(size);
    let mut n: i32;
    let count: i32;
    let acl: *mut posix_acl;

    if value.is_null() {
        return core::ptr::null_mut();
    }
    if size < core::mem::size_of::<ext4_acl_header>() {
        return ERR_PTR(-EINVAL);
    }
    if (*(value as *const ext4_acl_header)).a_version != cpu_to_le32(EXT4_ACL_VERSION) {
        return ERR_PTR(-EINVAL);
    }
    let mut value = (value as *const u8).add(core::mem::size_of::<ext4_acl_header>());
    count = ext4_acl_count(size);
    if count < 0 {
        return ERR_PTR(-EINVAL);
    }
    if count == 0 {
        return core::ptr::null_mut();
    }
    acl = posix_acl_alloc(count, GFP_NOFS);
    if acl.is_null() {
        return ERR_PTR(-ENOMEM);
    }
    n = 0;
    while n < count {
        let entry = value as *const ext4_acl_entry;
        if value.add(core::mem::size_of::<ext4_acl_entry_short>()) > end {
            posix_acl_release(acl);
            return ERR_PTR(-EINVAL);
        }
        (*acl).a_entries[n as usize].e_tag = le16_to_cpu((*entry).e_tag);
        (*acl).a_entries[n as usize].e_perm = le16_to_cpu((*entry).e_perm);

        match (*acl).a_entries[n as usize].e_tag {
            ACL_USER_OBJ | ACL_GROUP_OBJ | ACL_MASK | ACL_OTHER => {
                value = value.add(core::mem::size_of::<ext4_acl_entry_short>());
            }
            ACL_USER => {
                value = value.add(core::mem::size_of::<ext4_acl_entry>());
                if value > end {
                    posix_acl_release(acl);
                    return ERR_PTR(-EINVAL);
                }
                (*acl).a_entries[n as usize].e_uid =
                    make_kuid(&init_user_ns, le32_to_cpu((*entry).e_id));
            }
            ACL_GROUP => {
                value = value.add(core::mem::size_of::<ext4_acl_entry>());
                if value > end {
                    posix_acl_release(acl);
                    return ERR_PTR(-EINVAL);
                }
                (*acl).a_entries[n as usize].e_gid =
                    make_kgid(&init_user_ns, le32_to_cpu((*entry).e_id));
            }
            _ => {
                posix_acl_release(acl);
                return ERR_PTR(-EINVAL);
            }
        }
        n += 1;
    }
    if value != end {
        posix_acl_release(acl);
        return ERR_PTR(-EINVAL);
    }
    acl
}

/* Convert from in-memory to filesystem representation. */
unsafe fn ext4_acl_to_disk(acl: *const posix_acl, size: *mut usize) -> *mut core::ffi::c_void {
    let ext_acl: *mut ext4_acl_header;
    let mut e: *mut u8;

    *size = ext4_acl_size((*acl).a_count);
    ext_acl = kmalloc(
        core::mem::size_of::<ext4_acl_header>() +
            (*acl).a_count as usize * core::mem::size_of::<ext4_acl_entry>(),
        GFP_NOFS,
    ) as *mut ext4_acl_header;
    if ext_acl.is_null() {
        return ERR_PTR(-ENOMEM);
    }
    (*ext_acl).a_version = cpu_to_le32(EXT4_ACL_VERSION);
    e = (ext_acl as *mut u8).add(core::mem::size_of::<ext4_acl_header>());
    let mut n = 0usize;
    while n < (*acl).a_count as usize {
        let acl_e = &(*acl).a_entries[n];
        let entry = e as *mut ext4_acl_entry;
        (*entry).e_tag = cpu_to_le16(acl_e.e_tag);
        (*entry).e_perm = cpu_to_le16(acl_e.e_perm);
        match acl_e.e_tag {
            ACL_USER => {
                (*entry).e_id = cpu_to_le32(from_kuid(&init_user_ns, acl_e.e_uid));
                e = e.add(core::mem::size_of::<ext4_acl_entry>());
            }
            ACL_GROUP => {
                (*entry).e_id = cpu_to_le32(from_kgid(&init_user_ns, acl_e.e_gid));
                e = e.add(core::mem::size_of::<ext4_acl_entry>());
            }
            ACL_USER_OBJ | ACL_GROUP_OBJ | ACL_MASK | ACL_OTHER => {
                e = e.add(core::mem::size_of::<ext4_acl_entry_short>());
            }
            _ => {
                kfree(ext_acl as *mut core::ffi::c_void);
                return ERR_PTR(-EINVAL);
            }
        }
        n += 1;
    }
    ext_acl as *mut core::ffi::c_void
}

/* Inode operation get_posix_acl(). inode->i_rwsem: don't care */
unsafe fn ext4_get_acl(inode: *mut inode, type_: i32, rcu: bool) -> *mut posix_acl {
    let name_index: i32;
    let mut value: *mut i8 = core::ptr::null_mut();
    let acl: *mut posix_acl;
    let mut retval: i32;

    if rcu { return ERR_PTR(-ECHILD); }
    name_index = match type_ {
        ACL_TYPE_ACCESS => EXT4_XATTR_INDEX_POSIX_ACL_ACCESS,
        ACL_TYPE_DEFAULT => EXT4_XATTR_INDEX_POSIX_ACL_DEFAULT,
        _ => { BUG(); 0 }
    };
    retval = ext4_xattr_get(inode, name_index, b"\0".as_ptr() as *const i8, core::ptr::null_mut(), 0);
    if retval > 0 {
        value = kmalloc(retval as usize, GFP_NOFS) as *mut i8;
        if value.is_null() { return ERR_PTR(-ENOMEM); }
        retval = ext4_xattr_get(inode, name_index, b"\0".as_ptr() as *const i8, value, retval as usize);
    }
    acl = if retval > 0 { ext4_acl_from_disk(value as *const core::ffi::c_void, retval as usize) }
        else if retval == -ENODATA || retval == -ENOSYS { core::ptr::null_mut() }
        else { ERR_PTR(retval) };
    kfree(value as *mut core::ffi::c_void);
    acl
}

/* Set the access or default ACL of an inode. inode->i_rwsem: down unless called from ext4_new_inode */
unsafe fn __ext4_set_acl(handle: *mut handle_t, inode: *mut inode, type_: i32,
                         acl: *mut posix_acl, xattr_flags: i32) -> i32 {
    let name_index: i32;
    let mut value: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut size = 0usize;
    let error: i32;
    match type_ {
        ACL_TYPE_ACCESS => name_index = EXT4_XATTR_INDEX_POSIX_ACL_ACCESS,
        ACL_TYPE_DEFAULT => {
            name_index = EXT4_XATTR_INDEX_POSIX_ACL_DEFAULT;
            if ((*inode).i_mode & S_IFMT) != S_IFDIR { return if !acl.is_null() { -EACCES } else { 0 }; }
        }
        _ => return -EINVAL,
    }
    if !acl.is_null() {
        value = ext4_acl_to_disk(acl, &mut size);
        if IS_ERR(value) { return PTR_ERR(value) as i32; }
    }
    error = ext4_xattr_set_handle(handle, inode, name_index, b"\0".as_ptr() as *const i8,
                                  value, size, xattr_flags);
    kfree(value);
    if error == 0 { set_cached_acl(inode, type_, acl); }
    error
}

unsafe fn ext4_set_acl(idmap: *mut mnt_idmap, dentry: *mut dentry,
                       acl: *mut posix_acl, type_: i32) -> i32 {
    let inode = d_inode(dentry);
    let acl_size = if !acl.is_null() { ext4_acl_size((*acl).a_count) } else { 0 };
    let mut retries = 0;
    let mut error = dquot_initialize(inode);
    if error != 0 { return error; }
    'retry: loop {
        let mut credits = 0;
        error = ext4_xattr_set_credits(inode, acl_size, false, &mut credits);
        if error != 0 { return error; }
        let handle = ext4_journal_start(inode, EXT4_HT_XATTR, credits);
        if IS_ERR(handle) { return PTR_ERR(handle) as i32; }
        let mut mode = (*inode).i_mode;
        let mut update_mode = false;
        if type_ == ACL_TYPE_ACCESS && !acl.is_null() {
            error = posix_acl_update_mode(idmap, inode, &mut mode, &mut (acl as *mut posix_acl));
            if error != 0 { ext4_journal_stop(handle); return error; }
            update_mode = mode != (*inode).i_mode;
        }
        error = __ext4_set_acl(handle, inode, type_, acl, 0);
        if error == 0 && update_mode {
            (*inode).i_mode = mode;
            inode_set_ctime_current(inode);
            error = ext4_mark_inode_dirty(handle, inode);
        }
        ext4_journal_stop(handle);
        if error == -ENOSPC && ext4_should_retry_alloc((*inode).i_sb, &mut retries) { continue 'retry; }
        return error;
    }
}

/* Initialize the ACLs of a new inode. Called from ext4_new_inode. */
unsafe fn ext4_init_acl(handle: *mut handle_t, inode: *mut inode, dir: *mut inode) -> i32 {
    let mut default_acl: *mut posix_acl = core::ptr::null_mut();
    let mut acl: *mut posix_acl = core::ptr::null_mut();
    let mut error = posix_acl_create(dir, &mut (*inode).i_mode, &mut default_acl, &mut acl);
    if error != 0 { return error; }
    if !default_acl.is_null() {
        error = __ext4_set_acl(handle, inode, ACL_TYPE_DEFAULT, default_acl, XATTR_CREATE);
        posix_acl_release(default_acl);
    } else { (*inode).i_default_acl = core::ptr::null_mut(); }
    if !acl.is_null() {
        if error == 0 { error = __ext4_set_acl(handle, inode, ACL_TYPE_ACCESS, acl, XATTR_CREATE); }
        posix_acl_release(acl);
    } else { (*inode).i_acl = core::ptr::null_mut(); }
    error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
