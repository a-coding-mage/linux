// SPDX-License-Identifier: GPL-2.0-only
/*
 * acl.c
 *
 * Copyright (C) 2004, 2008 Oracle.  All rights reserved.
 *
 * CREDITS:
 * Lots of code in this file is copy from linux/fs/ext3/acl.c.
 * Copyright (C) 2001-2003 Andreas Gruenbacher, <agruen@suse.de>
 */

/* Linux and OCFS2 declarations are supplied by the surrounding translation unit. */

/* Convert from xattr value to acl struct. */
unsafe fn ocfs2_acl_from_xattr(value: *const core::ffi::c_void, size: usize) -> *mut posix_acl {
    if value.is_null() { return core::ptr::null_mut(); }
    if size < core::mem::size_of::<posix_acl_entry>() { return ERR_PTR(-EINVAL); }
    let count = size / core::mem::size_of::<posix_acl_entry>();
    let acl = posix_acl_alloc(count, GFP_NOFS);
    if acl.is_null() { return ERR_PTR(-ENOMEM); }
    for n in 0..count {
        let entry = (value as *const ocfs2_acl_entry).add(n);
        (*acl).a_entries[n].e_tag = le16_to_cpu((*entry).e_tag);
        (*acl).a_entries[n].e_perm = le16_to_cpu((*entry).e_perm);
        match (*acl).a_entries[n].e_tag {
            ACL_USER => (*acl).a_entries[n].e_uid = make_kuid(&init_user_ns, le32_to_cpu((*entry).e_id)),
            ACL_GROUP => (*acl).a_entries[n].e_gid = make_kgid(&init_user_ns, le32_to_cpu((*entry).e_id)),
            _ => {}
        }
    }
    acl
}

/* Convert acl struct to xattr value. */
unsafe fn ocfs2_acl_to_xattr(acl: *const posix_acl, size: *mut usize) -> *mut core::ffi::c_void {
    *size = (*acl).a_count * core::mem::size_of::<posix_acl_entry>();
    let value = kmalloc(*size, GFP_NOFS);
    if value.is_null() { return ERR_PTR(-ENOMEM); }
    let mut entry = value as *mut ocfs2_acl_entry;
    for n in 0..(*acl).a_count {
        (*entry).e_tag = cpu_to_le16((*acl).a_entries[n].e_tag);
        (*entry).e_perm = cpu_to_le16((*acl).a_entries[n].e_perm);
        (*entry).e_id = match (*acl).a_entries[n].e_tag {
            ACL_USER => cpu_to_le32(from_kuid(&init_user_ns, (*acl).a_entries[n].e_uid)),
            ACL_GROUP => cpu_to_le32(from_kgid(&init_user_ns, (*acl).a_entries[n].e_gid)),
            _ => cpu_to_le32(ACL_UNDEFINED_ID),
        };
        entry = entry.add(1);
    }
    value
}

unsafe fn ocfs2_get_acl_nolock(inode: *mut inode, ty: i32, di_bh: *mut buffer_head) -> *mut posix_acl {
    let name_index = match ty { ACL_TYPE_ACCESS => OCFS2_XATTR_INDEX_POSIX_ACL_ACCESS, ACL_TYPE_DEFAULT => OCFS2_XATTR_INDEX_POSIX_ACL_DEFAULT, _ => return ERR_PTR(-EINVAL) };
    let mut value: *mut i8 = core::ptr::null_mut();
    let mut retval = ocfs2_xattr_get_nolock(inode, di_bh, name_index, b"\0".as_ptr() as *const i8, core::ptr::null_mut(), 0);
    if retval > 0 {
        value = kmalloc(retval as usize, GFP_NOFS) as *mut i8;
        if value.is_null() { return ERR_PTR(-ENOMEM); }
        retval = ocfs2_xattr_get_nolock(inode, di_bh, name_index, b"\0".as_ptr() as *const i8, value as *mut core::ffi::c_void, retval as usize) as i32;
    }
    let acl = if retval > 0 { ocfs2_acl_from_xattr(value as *const core::ffi::c_void, retval as usize) } else if retval == -ENODATA || retval == 0 { core::ptr::null_mut() } else { ERR_PTR(retval) };
    kfree(value as *mut core::ffi::c_void);
    acl
}

unsafe fn ocfs2_acl_set_mode(inode: *mut inode, mut di_bh: *mut buffer_head, mut handle: *mut handle_t, new_mode: umode_t) -> i32 {
    let mut ret: i32; let mut commit_handle = false;
    if di_bh.is_null() { ret = ocfs2_read_inode_block(inode, &mut di_bh); if ret != 0 { mlog_errno(ret); return ret; } } else { get_bh(di_bh); }
    if handle.is_null() { handle = ocfs2_start_trans(OCFS2_SB((*inode).i_sb), OCFS2_INODE_UPDATE_CREDITS); if IS_ERR(handle) { ret = PTR_ERR(handle); mlog_errno(ret); brelse(di_bh); return ret; } commit_handle = true; }
    let di = (*di_bh).b_data as *mut ocfs2_dinode;
    ret = ocfs2_journal_access_di(handle, INODE_CACHE(inode), di_bh, OCFS2_JOURNAL_ACCESS_WRITE);
    if ret == 0 { (*inode).i_mode = new_mode; inode_set_ctime_current(inode); (*di).i_mode = cpu_to_le16((*inode).i_mode); (*di).i_ctime = cpu_to_le64(inode_get_ctime_sec(inode)); (*di).i_ctime_nsec = cpu_to_le32(inode_get_ctime_nsec(inode)); ocfs2_update_inode_fsync_trans(handle, inode, 0); ocfs2_journal_dirty(handle, di_bh); } else { mlog_errno(ret); }
    if commit_handle { ocfs2_commit_trans(OCFS2_SB((*inode).i_sb), handle); } brelse(di_bh); ret
}

unsafe fn ocfs2_set_acl(handle: *mut handle_t, inode: *mut inode, di_bh: *mut buffer_head, ty: i32, acl: *mut posix_acl, meta_ac: *mut ocfs2_alloc_context, data_ac: *mut ocfs2_alloc_context) -> i32 {
    if S_ISLNK((*inode).i_mode) { return -EOPNOTSUPP; }
    let name_index = match ty { ACL_TYPE_ACCESS => OCFS2_XATTR_INDEX_POSIX_ACL_ACCESS, ACL_TYPE_DEFAULT => { if !S_ISDIR((*inode).i_mode) { return if acl.is_null() { 0 } else { -EACCES }; } OCFS2_XATTR_INDEX_POSIX_ACL_DEFAULT }, _ => return -EINVAL };
    let mut size = 0usize; let mut value = core::ptr::null_mut();
    if !acl.is_null() { value = ocfs2_acl_to_xattr(acl, &mut size); if IS_ERR(value) { return PTR_ERR(value); } }
    let ret = if !handle.is_null() { ocfs2_xattr_set_handle(handle, inode, di_bh, name_index, b"\0".as_ptr() as *const i8, value, size, 0, meta_ac, data_ac) } else { ocfs2_xattr_set(inode, name_index, b"\0".as_ptr() as *const i8, value, size, 0) };
    kfree(value); if ret == 0 { set_cached_acl(inode, ty, acl); } ret
}

pub unsafe fn ocfs2_iop_set_acl(_idmap: *mut mnt_idmap, dentry: *mut dentry, acl: *mut posix_acl, ty: i32) -> i32 {
    let mut bh = core::ptr::null_mut(); let inode = d_inode(dentry); let oh = core::mem::MaybeUninit::<ocfs2_lock_holder>::uninit(); let mut oh = oh.assume_init();
    let had_lock = ocfs2_inode_lock_tracker(inode, &mut bh, 1, &mut oh); if had_lock < 0 { return had_lock; }
    let mut status = 0; if ty == ACL_TYPE_ACCESS && !acl.is_null() { let mut mode = 0; status = posix_acl_update_mode(&nop_mnt_idmap, inode, &mut mode, &mut (acl as *mut posix_acl)); if status == 0 { status = ocfs2_acl_set_mode(inode, bh, core::ptr::null_mut(), mode); } }
    if status == 0 { status = ocfs2_set_acl(core::ptr::null_mut(), inode, bh, ty, acl, core::ptr::null_mut(), core::ptr::null_mut()); }
    ocfs2_inode_unlock_tracker(inode, 1, &mut oh, had_lock); brelse(bh); status
}

pub unsafe fn ocfs2_iop_get_acl(inode: *mut inode, ty: i32, rcu: bool) -> *mut posix_acl {
    if rcu { return ERR_PTR(-ECHILD); } let osb = OCFS2_SB((*inode).i_sb); if (*osb).s_mount_opt & OCFS2_MOUNT_POSIX_ACL == 0 { return core::ptr::null_mut(); }
    let mut bh = core::ptr::null_mut(); let mut oh = core::mem::MaybeUninit::<ocfs2_lock_holder>::zeroed().assume_init(); let had_lock = ocfs2_inode_lock_tracker(inode, &mut bh, 0, &mut oh); if had_lock < 0 { return ERR_PTR(had_lock); }
    down_read(&mut (*OCFS2_I(inode)).ip_xattr_sem); let acl = ocfs2_get_acl_nolock(inode, ty, bh); up_read(&mut (*OCFS2_I(inode)).ip_xattr_sem); ocfs2_inode_unlock_tracker(inode, 0, &mut oh, had_lock); brelse(bh); acl
}

pub unsafe fn ocfs2_acl_chmod(inode: *mut inode, bh: *mut buffer_head) -> i32 { let osb = OCFS2_SB((*inode).i_sb); if S_ISLNK((*inode).i_mode) { return -EOPNOTSUPP; } if (*osb).s_mount_opt & OCFS2_MOUNT_POSIX_ACL == 0 { return 0; } down_read(&mut (*OCFS2_I(inode)).ip_xattr_sem); let mut acl = ocfs2_get_acl_nolock(inode, ACL_TYPE_ACCESS, bh); up_read(&mut (*OCFS2_I(inode)).ip_xattr_sem); if IS_ERR_OR_NULL(acl) { return PTR_ERR_OR_ZERO(acl); } let mut ret = __posix_acl_chmod(&mut acl, GFP_KERNEL, (*inode).i_mode); if ret == 0 { ret = ocfs2_set_acl(core::ptr::null_mut(), inode, core::ptr::null_mut(), ACL_TYPE_ACCESS, acl, core::ptr::null_mut(), core::ptr::null_mut()); } posix_acl_release(acl); ret }

pub unsafe fn ocfs2_acl_init_release(state: *mut ocfs2_acl_state) { posix_acl_release((*state).default_acl); posix_acl_release((*state).acl); (*state).default_acl = core::ptr::null_mut(); (*state).acl = core::ptr::null_mut(); }

pub unsafe fn ocfs2_acl_init_prepare(inode: *mut inode, dir: *mut inode, dir_bh: *mut buffer_head, state: *mut ocfs2_acl_state) -> i32 {
    let osb = OCFS2_SB((*inode).i_sb); (*state).default_acl = core::ptr::null_mut(); (*state).acl = core::ptr::null_mut(); (*state).mode = (*inode).i_mode; if S_ISLNK((*inode).i_mode) { return 0; }
    if (*osb).s_mount_opt & OCFS2_MOUNT_POSIX_ACL != 0 { down_read(&mut (*OCFS2_I(dir)).ip_xattr_sem); (*state).default_acl = ocfs2_get_acl_nolock(dir, ACL_TYPE_DEFAULT, dir_bh); up_read(&mut (*OCFS2_I(dir)).ip_xattr_sem); if IS_ERR((*state).default_acl) { let r = PTR_ERR((*state).default_acl); (*state).default_acl = core::ptr::null_mut(); return r; } if !(*state).default_acl.is_null() { (*state).acl = posix_acl_dup((*state).default_acl); if (*state).acl.is_null() { ocfs2_acl_init_release(state); return -ENOMEM; } let r = __posix_acl_create(&mut (*state).acl, GFP_NOFS, &mut (*state).mode); if r < 0 { ocfs2_acl_init_release(state); return r; } if r == 0 { posix_acl_release((*state).acl); (*state).acl = core::ptr::null_mut(); } if !S_ISDIR((*inode).i_mode) { posix_acl_release((*state).default_acl); (*state).default_acl = core::ptr::null_mut(); } } else { (*state).mode &= !current_umask(); } } else { (*state).mode &= !current_umask(); } 0
}

pub unsafe fn ocfs2_init_acl(handle: *mut handle_t, inode: *mut inode, di_bh: *mut buffer_head, meta_ac: *mut ocfs2_alloc_context, data_ac: *mut ocfs2_alloc_context, state: *mut ocfs2_acl_state) -> i32 {
    let osb = OCFS2_SB((*inode).i_sb); if S_ISLNK((*inode).i_mode) { return 0; } let mut ret = 0; if (*osb).s_mount_opt & OCFS2_MOUNT_POSIX_ACL != 0 && S_ISDIR((*inode).i_mode) && !(*state).default_acl.is_null() { ret = ocfs2_set_acl(handle, inode, di_bh, ACL_TYPE_DEFAULT, (*state).default_acl, meta_ac, data_ac); if ret != 0 { return ret; } } ret = ocfs2_acl_set_mode(inode, di_bh, handle, (*state).mode); if ret != 0 { mlog_errno(ret); return ret; } if (*osb).s_mount_opt & OCFS2_MOUNT_POSIX_ACL != 0 && !(*state).acl.is_null() { ret = ocfs2_set_acl(handle, inode, di_bh, ACL_TYPE_ACCESS, (*state).acl, meta_ac, data_ac); } ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
