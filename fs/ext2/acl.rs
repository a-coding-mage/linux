// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/ext2/acl.c
 *
 * Copyright (C) 2001-2003 Andreas Gruenbacher, <agruen@suse.de>
 */

// Dependencies are supplied by the surrounding kernel translation unit.

/*
 * Convert from filesystem to in-memory representation.
 */
unsafe fn ext2_acl_from_disk(value: *const core::ffi::c_void, size: usize) -> *mut posix_acl {
    let mut value = value as *const u8;
    let end = value.add(size);
    let mut n: i32;
    let count: i32;
    let acl: *mut posix_acl;

    if value.is_null() {
        return core::ptr::null_mut();
    }
    if size < core::mem::size_of::<ext2_acl_header>() {
        return ERR_PTR(-EINVAL);
    }
    if (*((value) as *const ext2_acl_header)).a_version != cpu_to_le32(EXT2_ACL_VERSION) {
        return ERR_PTR(-EINVAL);
    }
    value = value.add(core::mem::size_of::<ext2_acl_header>());
    count = ext2_acl_count(size);
    if count < 0 {
        return ERR_PTR(-EINVAL);
    }
    if count == 0 {
        return core::ptr::null_mut();
    }
    acl = posix_acl_alloc(count, GFP_KERNEL);
    if acl.is_null() {
        return ERR_PTR(-ENOMEM);
    }
    n = 0;
    while n < count {
        let entry = value as *const ext2_acl_entry;
        if value.add(core::mem::size_of::<ext2_acl_entry_short>()) > end {
            posix_acl_release(acl);
            return ERR_PTR(-EINVAL);
        }
        (*acl).a_entries[n as usize].e_tag = le16_to_cpu((*entry).e_tag);
        (*acl).a_entries[n as usize].e_perm = le16_to_cpu((*entry).e_perm);
        match (*acl).a_entries[n as usize].e_tag {
            ACL_USER_OBJ | ACL_GROUP_OBJ | ACL_MASK | ACL_OTHER => {
                value = value.add(core::mem::size_of::<ext2_acl_entry_short>());
            }
            ACL_USER => {
                value = value.add(core::mem::size_of::<ext2_acl_entry>());
                if value > end {
                    posix_acl_release(acl);
                    return ERR_PTR(-EINVAL);
                }
                (*acl).a_entries[n as usize].e_uid =
                    make_kuid(&init_user_ns, le32_to_cpu((*entry).e_id));
            }
            ACL_GROUP => {
                value = value.add(core::mem::size_of::<ext2_acl_entry>());
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

/*
 * Convert from in-memory to filesystem representation.
 */
unsafe fn ext2_acl_to_disk(acl: *const posix_acl, size: *mut usize) -> *mut core::ffi::c_void {
    let mut ext_acl: *mut ext2_acl_header;
    let mut e: *mut u8;
    let mut n: usize;

    *size = ext2_acl_size((*acl).a_count);
    ext_acl = kmalloc(
        core::mem::size_of::<ext2_acl_header>()
            + (*acl).a_count as usize * core::mem::size_of::<ext2_acl_entry>(),
        GFP_KERNEL,
    ) as *mut ext2_acl_header;
    if ext_acl.is_null() {
        return ERR_PTR(-ENOMEM);
    }
    (*ext_acl).a_version = cpu_to_le32(EXT2_ACL_VERSION);
    e = (ext_acl as *mut u8).add(core::mem::size_of::<ext2_acl_header>());
    n = 0;
    while n < (*acl).a_count as usize {
        let acl_e = &(*acl).a_entries[n];
        let entry = e as *mut ext2_acl_entry;
        (*entry).e_tag = cpu_to_le16(acl_e.e_tag);
        (*entry).e_perm = cpu_to_le16(acl_e.e_perm);
        match acl_e.e_tag {
            ACL_USER => {
                (*entry).e_id = cpu_to_le32(from_kuid(&init_user_ns, acl_e.e_uid));
                e = e.add(core::mem::size_of::<ext2_acl_entry>());
            }
            ACL_GROUP => {
                (*entry).e_id = cpu_to_le32(from_kgid(&init_user_ns, acl_e.e_gid));
                e = e.add(core::mem::size_of::<ext2_acl_entry>());
            }
            ACL_USER_OBJ | ACL_GROUP_OBJ | ACL_MASK | ACL_OTHER => {
                e = e.add(core::mem::size_of::<ext2_acl_entry_short>());
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

/*
 * inode->i_mutex: don't care
 */
unsafe fn ext2_get_acl(inode: *mut inode, acl_type: i32, rcu: bool) -> *mut posix_acl {
    let name_index: i32;
    let mut value: *mut i8 = core::ptr::null_mut();
    let acl: *mut posix_acl;
    let mut retval: i32;

    if rcu {
        return ERR_PTR(-ECHILD);
    }
    name_index = match acl_type {
        ACL_TYPE_ACCESS => EXT2_XATTR_INDEX_POSIX_ACL_ACCESS,
        ACL_TYPE_DEFAULT => EXT2_XATTR_INDEX_POSIX_ACL_DEFAULT,
        _ => { BUG(); 0 }
    };
    retval = ext2_xattr_get(inode, name_index, "".as_ptr() as *const i8, core::ptr::null_mut(), 0);
    if retval > 0 {
        value = kmalloc(retval as usize, GFP_KERNEL) as *mut i8;
        if value.is_null() {
            return ERR_PTR(-ENOMEM);
        }
        retval = ext2_xattr_get(inode, name_index, "".as_ptr() as *const i8, value, retval);
    }
    acl = if retval > 0 {
        ext2_acl_from_disk(value as *const core::ffi::c_void, retval as usize)
    } else if retval == -ENODATA || retval == -ENOSYS {
        core::ptr::null_mut()
    } else {
        ERR_PTR(retval)
    };
    kfree(value as *mut core::ffi::c_void);
    acl
}

unsafe fn __ext2_set_acl(inode: *mut inode, acl: *mut posix_acl, acl_type: i32) -> i32 {
    let name_index: i32;
    let mut value: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut size: usize = 0;
    let error: i32;

    name_index = match acl_type {
        ACL_TYPE_ACCESS => EXT2_XATTR_INDEX_POSIX_ACL_ACCESS,
        ACL_TYPE_DEFAULT => {
            if !S_ISDIR((*inode).i_mode) { return if !acl.is_null() { -EACCES } else { 0 }; }
            EXT2_XATTR_INDEX_POSIX_ACL_DEFAULT
        }
        _ => return -EINVAL,
    };
    if !acl.is_null() {
        value = ext2_acl_to_disk(acl, &mut size);
        if IS_ERR(value) { return PTR_ERR(value) as i32; }
    }
    error = ext2_xattr_set(inode, name_index, "".as_ptr() as *const i8, value, size, 0);
    kfree(value);
    if error == 0 { set_cached_acl(inode, acl_type, acl); }
    error
}

/*
 * inode->i_mutex: down
 */
unsafe fn ext2_set_acl(_idmap: *mut mnt_idmap, dentry: *mut dentry, acl: *mut posix_acl, acl_type: i32) -> i32 {
    let mut error: i32;
    let mut update_mode = 0;
    let inode = d_inode(dentry);
    let mut mode = (*inode).i_mode;

    if acl_type == ACL_TYPE_ACCESS && !acl.is_null() {
        error = posix_acl_update_mode(&nop_mnt_idmap, inode, &mut mode, &mut (acl as *mut posix_acl));
        if error != 0 { return error; }
        update_mode = 1;
    }
    error = __ext2_set_acl(inode, acl, acl_type);
    if error == 0 && update_mode != 0 {
        (*inode).i_mode = mode;
        inode_set_ctime_current(inode);
        mark_inode_dirty(inode);
    }
    error
}

/*
 * Initialize the ACLs of a new inode. Called from ext2_new_inode.
 *
 * dir->i_mutex: down
 * inode->i_mutex: up (access to inode is still exclusive)
 */
unsafe fn ext2_init_acl(inode: *mut inode, dir: *mut inode) -> i32 {
    let mut default_acl: *mut posix_acl = core::ptr::null_mut();
    let mut acl: *mut posix_acl = core::ptr::null_mut();
    let mut error = posix_acl_create(dir, &mut (*inode).i_mode, &mut default_acl, &mut acl);
    if error != 0 { return error; }
    if !default_acl.is_null() {
        error = __ext2_set_acl(inode, default_acl, ACL_TYPE_DEFAULT);
        posix_acl_release(default_acl);
    } else {
        (*inode).i_default_acl = core::ptr::null_mut();
    }
    if !acl.is_null() {
        if error == 0 { error = __ext2_set_acl(inode, acl, ACL_TYPE_ACCESS); }
        posix_acl_release(acl);
    } else {
        (*inode).i_acl = core::ptr::null_mut();
    }
    error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
