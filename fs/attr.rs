// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/attr.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *  changes by Thomas Schoebel-Theuer
 */

/* Linux dependencies are supplied by the surrounding translation unit. */

pub unsafe fn setattr_should_drop_sgid(
    idmap: *mut mnt_idmap,
    inode: *const inode,
) -> i32 {
    let mode: umode_t = (*inode).i_mode;

    if (mode & S_ISGID) == 0 { return 0; }
    if (mode & S_IXGRP) != 0 { return ATTR_KILL_SGID; }
    if !in_group_or_capable(idmap, inode, i_gid_into_vfsgid(idmap, inode)) {
        return ATTR_KILL_SGID;
    }
    0
}

pub unsafe fn setattr_should_drop_suidgid(
    idmap: *mut mnt_idmap,
    inode: *mut inode,
) -> i32 {
    let mode: umode_t = (*inode).i_mode;
    let mut kill: i32 = 0;

    /* suid always must be killed */
    if unlikely((mode & S_ISUID) != 0) { kill = ATTR_KILL_SUID; }
    kill |= setattr_should_drop_sgid(idmap, inode);
    if unlikely(kill != 0 && !capable(CAP_FSETID) && S_ISREG(mode)) { return kill; }
    0
}

unsafe fn chown_ok(idmap: *mut mnt_idmap, inode: *const inode, ia_vfsuid: vfsuid_t) -> bool {
    let vfsuid = i_uid_into_vfsuid(idmap, inode);
    if vfsuid_eq_kuid(vfsuid, current_fsuid()) && vfsuid_eq(ia_vfsuid, vfsuid) { return true; }
    if capable_wrt_inode_uidgid(idmap, inode, CAP_CHOWN) { return true; }
    if !vfsuid_valid(vfsuid) && ns_capable((*(*inode).i_sb).s_user_ns, CAP_CHOWN) { return true; }
    false
}

unsafe fn chgrp_ok(idmap: *mut mnt_idmap, inode: *const inode, ia_vfsgid: vfsgid_t) -> bool {
    let vfsgid = i_gid_into_vfsgid(idmap, inode);
    let vfsuid = i_uid_into_vfsuid(idmap, inode);
    if vfsuid_eq_kuid(vfsuid, current_fsuid()) {
        if vfsgid_eq(ia_vfsgid, vfsgid) || vfsgid_in_group_p(ia_vfsgid) { return true; }
    }
    if capable_wrt_inode_uidgid(idmap, inode, CAP_CHOWN) { return true; }
    if !vfsgid_valid(vfsgid) && ns_capable((*(*inode).i_sb).s_user_ns, CAP_CHOWN) { return true; }
    false
}

pub unsafe fn setattr_prepare(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> i32 {
    let inode = d_inode(dentry);
    let ia_valid = (*attr).ia_valid;
    if (ia_valid & ATTR_SIZE) != 0 {
        if IS_VERITY(inode) { return -EPERM; }
        let error = inode_newsize_ok(inode, (*attr).ia_size);
        if error != 0 { return error; }
    }
    if (ia_valid & ATTR_FORCE) != 0 { return setattr_prepare_kill_priv(idmap, dentry, attr); }
    if (ia_valid & ATTR_UID) != 0 && !chown_ok(idmap, inode, (*attr).ia_vfsuid) { return -EPERM; }
    if (ia_valid & ATTR_GID) != 0 && !chgrp_ok(idmap, inode, (*attr).ia_vfsgid) { return -EPERM; }
    if (ia_valid & ATTR_MODE) != 0 {
        if !inode_owner_or_capable(idmap, inode) { return -EPERM; }
        let vfsgid = if (ia_valid & ATTR_GID) != 0 { (*attr).ia_vfsgid } else { i_gid_into_vfsgid(idmap, inode) };
        if !in_group_or_capable(idmap, inode, vfsgid) { (*attr).ia_mode &= !S_ISGID; }
    }
    if (ia_valid & (ATTR_MTIME_SET | ATTR_ATIME_SET | ATTR_TIMES_SET)) != 0 && !inode_owner_or_capable(idmap, inode) { return -EPERM; }
    setattr_prepare_kill_priv(idmap, dentry, attr)
}

unsafe fn setattr_prepare_kill_priv(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> i32 {
    if ((*attr).ia_valid & ATTR_KILL_PRIV) != 0 { return security_inode_killpriv(idmap, dentry); }
    0
}

pub unsafe fn inode_newsize_ok(inode: *const inode, offset: loff_t) -> i32 {
    if offset < 0 { return -EINVAL; }
    if (*inode).i_size < offset {
        let limit = rlimit(RLIMIT_FSIZE);
        if limit != RLIM_INFINITY && offset > limit { send_sig(SIGXFSZ, current, 0); return -EFBIG; }
        if offset > (*(*inode).i_sb).s_maxbytes { return -EFBIG; }
    } else if IS_SWAPFILE(inode) { return -ETXTBSY; }
    0
}

unsafe fn setattr_copy_mgtime(inode: *mut inode, attr: *const iattr) {
    let valid = (*attr).ia_valid;
    let now = if (valid & ATTR_CTIME_SET) != 0 { inode_set_ctime_deleg(inode, (*attr).ia_ctime) }
        else if (valid & ATTR_CTIME) != 0 { inode_set_ctime_current(inode) } else { current_time(inode) };
    if (valid & ATTR_ATIME_SET) != 0 { inode_set_atime_to_ts(inode, (*attr).ia_atime); }
    else if (valid & ATTR_ATIME) != 0 { inode_set_atime_to_ts(inode, now); }
    if (valid & ATTR_MTIME_SET) != 0 { inode_set_mtime_to_ts(inode, (*attr).ia_mtime); }
    else if (valid & ATTR_MTIME) != 0 { inode_set_mtime_to_ts(inode, now); }
}

pub unsafe fn setattr_copy(idmap: *mut mnt_idmap, inode: *mut inode, attr: *const iattr) {
    let valid = (*attr).ia_valid;
    i_uid_update(idmap, attr, inode); i_gid_update(idmap, attr, inode);
    if (valid & ATTR_MODE) != 0 {
        let mut mode = (*attr).ia_mode;
        if !in_group_or_capable(idmap, inode, i_gid_into_vfsgid(idmap, inode)) { mode &= !S_ISGID; }
        (*inode).i_mode = mode;
    }
    if is_mgtime(inode) { setattr_copy_mgtime(inode, attr); return; }
    if (valid & ATTR_ATIME) != 0 { inode_set_atime_to_ts(inode, (*attr).ia_atime); }
    if (valid & ATTR_MTIME) != 0 { inode_set_mtime_to_ts(inode, (*attr).ia_mtime); }
    if (valid & ATTR_CTIME_SET) != 0 { inode_set_ctime_deleg(inode, (*attr).ia_ctime); }
    else if (valid & ATTR_CTIME) != 0 { inode_set_ctime_to_ts(inode, (*attr).ia_ctime); }
}

pub unsafe fn may_setattr(idmap: *mut mnt_idmap, inode: *mut inode, ia_valid: u32) -> i32 {
    if (ia_valid & (ATTR_MODE | ATTR_UID | ATTR_GID | ATTR_TIMES_SET)) != 0 && (IS_IMMUTABLE(inode) || IS_APPEND(inode)) { return -EPERM; }
    if (ia_valid & ATTR_TOUCH) != 0 {
        if IS_IMMUTABLE(inode) { return -EPERM; }
        if !inode_owner_or_capable(idmap, inode) { let error = inode_permission(idmap, inode, MAY_WRITE); if error != 0 { return error; } }
    }
    0
}

pub unsafe fn notify_change(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr, delegated_inode: *mut delegated_inode) -> i32 {
    let inode = (*dentry).d_inode;
    let mode = (*inode).i_mode;
    let mut ia_valid = (*attr).ia_valid;
    let mut error = may_setattr(idmap, inode, ia_valid); if error != 0 { return error; }
    if (ia_valid & ATTR_MODE) != 0 {
        if S_ISLNK((*inode).i_mode) { return -EOPNOTSUPP; }
        if is_sxid((*attr).ia_mode) { (*inode).i_flags &= !S_NOSEC; }
    }
    let now = current_time(inode);
    (*attr).ia_atime = if (ia_valid & ATTR_ATIME_SET) != 0 { timestamp_truncate((*attr).ia_atime, inode) } else { now };
    (*attr).ia_ctime = if (ia_valid & ATTR_CTIME_SET) != 0 { timestamp_truncate((*attr).ia_ctime, inode) } else { now };
    (*attr).ia_mtime = if (ia_valid & ATTR_MTIME_SET) != 0 { timestamp_truncate((*attr).ia_mtime, inode) } else { now };
    if (ia_valid & ATTR_KILL_PRIV) != 0 {
        error = security_inode_need_killpriv(dentry); if error < 0 { return error; }
        if error == 0 { ia_valid = (*attr).ia_valid &= !ATTR_KILL_PRIV; }
    }
    if (ia_valid & (ATTR_KILL_SUID | ATTR_KILL_SGID)) != 0 && (ia_valid & ATTR_MODE) != 0 { BUG(); }
    if (ia_valid & ATTR_KILL_SUID) != 0 && (mode & S_ISUID) != 0 { ia_valid = (*attr).ia_valid |= ATTR_MODE; (*attr).ia_mode = (*inode).i_mode & !S_ISUID; }
    if (ia_valid & ATTR_KILL_SGID) != 0 && (mode & S_ISGID) != 0 { if (ia_valid & ATTR_MODE) == 0 { ia_valid = (*attr).ia_valid |= ATTR_MODE; (*attr).ia_mode = (*inode).i_mode; } (*attr).ia_mode &= !S_ISGID; }
    if ((*attr).ia_valid & !(ATTR_KILL_SUID | ATTR_KILL_SGID)) == 0 { return 0; }
    if (ia_valid & ATTR_UID) != 0 && !vfsuid_has_fsmapping(idmap, (*(*inode).i_sb).s_user_ns, (*attr).ia_vfsuid) { return -EOVERFLOW; }
    if (ia_valid & ATTR_GID) != 0 && !vfsgid_has_fsmapping(idmap, (*(*inode).i_sb).s_user_ns, (*attr).ia_vfsgid) { return -EOVERFLOW; }
    if (ia_valid & ATTR_UID) == 0 && !vfsuid_valid(i_uid_into_vfsuid(idmap, inode)) { return -EOVERFLOW; }
    if (ia_valid & ATTR_GID) == 0 && !vfsgid_valid(i_gid_into_vfsgid(idmap, inode)) { return -EOVERFLOW; }
    error = security_inode_setattr(idmap, dentry, attr); if error != 0 { return error; }
    if (ia_valid & ATTR_DELEG) == 0 { error = try_break_deleg(inode, 0, delegated_inode); if error != 0 { return error; } }
    error = if (*(*inode).i_op).setattr.is_some() { (*(*inode).i_op).setattr(idmap, dentry, attr) } else { simple_setattr(idmap, dentry, attr) };
    if error == 0 { fsnotify_change(dentry, ia_valid); security_inode_post_setattr(idmap, dentry, ia_valid); }
    error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
