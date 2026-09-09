// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2008, Christoph Hellwig
 * All Rights Reserved.
 */

// Locking scheme:
//  - all ACL updates are protected by inode->i_mutex, which is taken before
//    calling into this file.

unsafe fn xfs_acl_from_disk(
    mp: *mut xfs_mount,
    aclp: *const xfs_acl,
    len: i32,
    max_entries: i32,
) -> *mut posix_acl {
    let mut acl_e: *mut posix_acl_entry;
    let acl: *mut posix_acl;
    let mut ace: *const xfs_acl_entry;
    let mut count: u32;
    let mut i: u32;

    if len < core::mem::size_of::<xfs_acl>() as i32 {
        XFS_CORRUPTION_ERROR(__func__, XFS_ERRLEVEL_LOW, mp, aclp, len);
        return ERR_PTR(-EFSCORRUPTED);
    }

    count = be32_to_cpu((*aclp).acl_cnt);
    if count > max_entries as u32 || XFS_ACL_SIZE(count) != len {
        XFS_CORRUPTION_ERROR(__func__, XFS_ERRLEVEL_LOW, mp, aclp, len);
        return ERR_PTR(-EFSCORRUPTED);
    }

    acl = posix_acl_alloc(count, GFP_KERNEL);
    if acl.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    i = 0;
    while i < count {
        acl_e = &mut (*acl).a_entries[i as usize];
        ace = &(*aclp).acl_entry[i as usize];

        // The tag is 32 bits on disk and 16 bits in core.
        // Because every access to it goes through the core format first this
        // is not a problem.
        (*acl_e).e_tag = be32_to_cpu((*ace).ae_tag) as _;
        (*acl_e).e_perm = be16_to_cpu((*ace).ae_perm) as _;

        match (*acl_e).e_tag {
            ACL_USER => (*acl_e).e_uid = make_kuid(&init_user_ns, be32_to_cpu((*ace).ae_id)),
            ACL_GROUP => (*acl_e).e_gid = make_kgid(&init_user_ns, be32_to_cpu((*ace).ae_id)),
            ACL_USER_OBJ | ACL_GROUP_OBJ | ACL_MASK | ACL_OTHER => {}
            _ => {
                posix_acl_release(acl);
                return ERR_PTR(-EINVAL);
            }
        }
        i += 1;
    }
    acl
}

unsafe fn xfs_acl_to_disk(aclp: *mut xfs_acl, acl: *const posix_acl) {
    let mut i: i32 = 0;

    (*aclp).acl_cnt = cpu_to_be32((*acl).a_count);
    while i < (*acl).a_count as i32 {
        let ace = &mut (*aclp).acl_entry[i as usize];
        let acl_e = &(*acl).a_entries[i as usize];

        (*ace).ae_tag = cpu_to_be32((*acl_e).e_tag as u32);
        (*ace).ae_id = match (*acl_e).e_tag {
            ACL_USER => cpu_to_be32(from_kuid(&init_user_ns, (*acl_e).e_uid)),
            ACL_GROUP => cpu_to_be32(from_kgid(&init_user_ns, (*acl_e).e_gid)),
            _ => cpu_to_be32(ACL_UNDEFINED_ID),
        };
        (*ace).ae_perm = cpu_to_be16((*acl_e).e_perm as u16);
        i += 1;
    }
}

pub unsafe fn xfs_get_acl(inode: *mut inode, r#type: i32, rcu: bool) -> *mut posix_acl {
    let ip = XFS_I(inode);
    let mp = (*ip).i_mount;
    let mut acl: *mut posix_acl = core::ptr::null_mut();
    let mut args = xfs_da_args {
        dp: ip,
        attr_filter: XFS_ATTR_ROOT,
        valuelen: XFS_ACL_MAX_SIZE(mp),
        ..core::mem::zeroed()
    };
    let error: i32;

    if rcu {
        return ERR_PTR(-ECHILD);
    }
    match r#type {
        ACL_TYPE_ACCESS => args.name = SGI_ACL_FILE,
        ACL_TYPE_DEFAULT => args.name = SGI_ACL_DEFAULT,
        _ => BUG(),
    }
    args.namelen = strlen(args.name);

    error = xfs_attr_get(&mut args);
    if error == 0 {
        acl = xfs_acl_from_disk(mp, args.value, args.valuelen, XFS_ACL_MAX_ENTRIES(mp));
    } else if error != -ENOATTR {
        acl = ERR_PTR(error);
    }
    kvfree(args.value);
    acl
}

pub unsafe fn __xfs_set_acl(inode: *mut inode, acl: *mut posix_acl, r#type: i32) -> i32 {
    let ip = XFS_I(inode);
    let mut args = xfs_da_args {
        dp: ip,
        attr_filter: XFS_ATTR_ROOT,
        ..core::mem::zeroed()
    };
    let mut error: i32;

    match r#type {
        ACL_TYPE_ACCESS => args.name = SGI_ACL_FILE,
        ACL_TYPE_DEFAULT => {
            if !S_ISDIR((*inode).i_mode) {
                return if !acl.is_null() { -EACCES } else { 0 };
            }
            args.name = SGI_ACL_DEFAULT;
        }
        _ => return -EINVAL,
    }
    args.namelen = strlen(args.name);

    if !acl.is_null() {
        args.valuelen = XFS_ACL_SIZE((*acl).a_count);
        args.value = kvzalloc(args.valuelen, GFP_KERNEL);
        if args.value.is_null() { return -ENOMEM; }
        xfs_acl_to_disk(args.value as *mut xfs_acl, acl);
        error = xfs_attr_change(&mut args, XFS_ATTRUPDATE_UPSERT);
        kvfree(args.value);
    } else {
        error = xfs_attr_change(&mut args, XFS_ATTRUPDATE_REMOVE);
        if error == -ENOATTR { error = 0; }
    }
    if error == 0 { set_cached_acl(inode, r#type, acl); }
    error
}

unsafe fn xfs_acl_set_mode(inode: *mut inode, mode: umode_t) -> i32 {
    let ip = XFS_I(inode);
    let mp = (*ip).i_mount;
    let mut tp: *mut xfs_trans = core::ptr::null_mut();
    let error = xfs_trans_alloc(mp, &M_RES(mp).tr_ichange, 0, 0, 0, &mut tp);
    if error != 0 { return error; }
    xfs_ilock(ip, XFS_ILOCK_EXCL);
    xfs_trans_ijoin(tp, ip, XFS_ILOCK_EXCL);
    (*inode).i_mode = mode;
    inode_set_ctime_current(inode);
    xfs_trans_log_inode(tp, ip, XFS_ILOG_CORE);
    if xfs_has_wsync(mp) { xfs_trans_set_sync(tp); }
    xfs_trans_commit(tp)
}

pub unsafe fn xfs_set_acl(idmap: *mut mnt_idmap, dentry: *mut dentry, acl: *mut posix_acl, r#type: i32) -> i32 {
    let mut mode: umode_t = 0;
    let mut set_mode = false;
    let mut error = 0;
    let inode = d_inode(dentry);

    if acl.is_null() { return __xfs_set_acl(inode, acl, r#type); }
    error = -E2BIG;
    if (*acl).a_count > XFS_ACL_MAX_ENTRIES(XFS_M((*inode).i_sb)) { return error; }
    if r#type == ACL_TYPE_ACCESS {
        error = posix_acl_update_mode(idmap, inode, &mut mode, &mut (acl as *mut _));
        if error != 0 { return error; }
        set_mode = true;
    }
    error = __xfs_set_acl(inode, acl, r#type);
    if error == 0 && set_mode && mode != (*inode).i_mode { error = xfs_acl_set_mode(inode, mode); }
    error
}

pub unsafe fn xfs_forget_acl(inode: *mut inode, name: *const i8) {
    if !strcmp(name, SGI_ACL_FILE) { forget_cached_acl(inode, ACL_TYPE_ACCESS); }
    else if !strcmp(name, SGI_ACL_DEFAULT) { forget_cached_acl(inode, ACL_TYPE_DEFAULT); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
