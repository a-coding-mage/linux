// SPDX-License-Identifier: GPL-2.0
// External kernel declarations and constants are supplied by the surrounding kernel translation.

/**
 * fileattr_fill_xflags - initialize fileattr with xflags
 * @fa: fileattr pointer
 * @xflags: FS_XFLAG_* flags
 *
 * Set ->fsx_xflags, ->fsx_valid and ->flags (translated xflags).
 */
pub unsafe fn fileattr_fill_xflags(fa: *mut file_kattr, xflags: u32) {
    (*fa).fsx_valid = true;
    (*fa).fsx_xflags = xflags;
    if (*fa).fsx_xflags & FS_XFLAG_IMMUTABLE != 0 { (*fa).flags |= FS_IMMUTABLE_FL; }
    if (*fa).fsx_xflags & FS_XFLAG_APPEND != 0 { (*fa).flags |= FS_APPEND_FL; }
    if (*fa).fsx_xflags & FS_XFLAG_SYNC != 0 { (*fa).flags |= FS_SYNC_FL; }
    if (*fa).fsx_xflags & FS_XFLAG_NOATIME != 0 { (*fa).flags |= FS_NOATIME_FL; }
    if (*fa).fsx_xflags & FS_XFLAG_NODUMP != 0 { (*fa).flags |= FS_NODUMP_FL; }
    if (*fa).fsx_xflags & FS_XFLAG_DAX != 0 { (*fa).flags |= FS_DAX_FL; }
    if (*fa).fsx_xflags & FS_XFLAG_PROJINHERIT != 0 { (*fa).flags |= FS_PROJINHERIT_FL; }
    if (*fa).fsx_xflags & FS_XFLAG_VERITY != 0 { (*fa).flags |= FS_VERITY_FL; }
    if (*fa).fsx_xflags & FS_XFLAG_CASEFOLD != 0 { (*fa).flags |= FS_CASEFOLD_FL; }
}

pub unsafe fn fileattr_fill_flags(fa: *mut file_kattr, flags: u32) {
    (*fa).flags_valid = true;
    (*fa).flags = flags;
    if flags & FS_SYNC_FL != 0 { (*fa).fsx_xflags |= FS_XFLAG_SYNC; }
    if flags & FS_IMMUTABLE_FL != 0 { (*fa).fsx_xflags |= FS_XFLAG_IMMUTABLE; }
    if flags & FS_APPEND_FL != 0 { (*fa).fsx_xflags |= FS_XFLAG_APPEND; }
    if flags & FS_NODUMP_FL != 0 { (*fa).fsx_xflags |= FS_XFLAG_NODUMP; }
    if flags & FS_NOATIME_FL != 0 { (*fa).fsx_xflags |= FS_XFLAG_NOATIME; }
    if flags & FS_DAX_FL != 0 { (*fa).fsx_xflags |= FS_XFLAG_DAX; }
    if flags & FS_PROJINHERIT_FL != 0 { (*fa).fsx_xflags |= FS_XFLAG_PROJINHERIT; }
    if flags & FS_VERITY_FL != 0 { (*fa).fsx_xflags |= FS_XFLAG_VERITY; }
    if flags & FS_CASEFOLD_FL != 0 { (*fa).fsx_xflags |= FS_XFLAG_CASEFOLD; }
}

pub unsafe fn vfs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> i32 {
    let inode = d_inode(dentry);
    if (*(*inode).i_op).fileattr_get.is_none() { return -ENOIOCTLCMD; }
    let error = security_inode_file_getattr(dentry, fa);
    if error != 0 { return error; }
    ((*(*inode).i_op).fileattr_get.unwrap())(dentry, fa)
}

unsafe fn fileattr_to_file_attr(fa: *const file_kattr, fattr: *mut file_attr) {
    core::ptr::write_bytes(fattr, 0, 1);
    (*fattr).fa_xflags = (*fa).fsx_xflags & FS_XFLAGS_MASK;
    (*fattr).fa_extsize = (*fa).fsx_extsize;
    (*fattr).fa_nextents = (*fa).fsx_nextents;
    (*fattr).fa_projid = (*fa).fsx_projid;
    (*fattr).fa_cowextsize = (*fa).fsx_cowextsize;
}

pub unsafe fn copy_fsxattr_to_user(fa: *const file_kattr, ufa: *mut fsxattr) -> i32 {
    let mut xfa: fsxattr = core::mem::zeroed();
    xfa.fsx_xflags = (*fa).fsx_xflags & FS_XFLAGS_MASK;
    xfa.fsx_extsize = (*fa).fsx_extsize;
    xfa.fsx_nextents = (*fa).fsx_nextents;
    xfa.fsx_projid = (*fa).fsx_projid;
    xfa.fsx_cowextsize = (*fa).fsx_cowextsize;
    if copy_to_user(ufa, &xfa, core::mem::size_of::<fsxattr>()) != 0 { return -EFAULT; }
    0
}

unsafe fn file_attr_to_fileattr(fattr: *const file_attr, fa: *mut file_kattr) -> i32 {
    let mask: u64 = FS_XFLAGS_MASK as u64;
    if ((*fattr).fa_xflags as u64) & !mask != 0 { return -EINVAL; }
    fileattr_fill_xflags(fa, (*fattr).fa_xflags & !(FS_XFLAG_RDONLY_MASK as u32));
    (*fa).fsx_extsize = (*fattr).fa_extsize;
    (*fa).fsx_projid = (*fattr).fa_projid;
    (*fa).fsx_cowextsize = (*fattr).fa_cowextsize;
    0
}

unsafe fn copy_fsxattr_from_user(fa: *mut file_kattr, ufa: *mut fsxattr) -> i32 {
    let mut xfa: fsxattr = core::mem::zeroed();
    if copy_from_user(&mut xfa, ufa, core::mem::size_of::<fsxattr>()) != 0 { return -EFAULT; }
    if xfa.fsx_xflags & !FS_XFLAGS_MASK != 0 { return -EOPNOTSUPP; }
    fileattr_fill_xflags(fa, xfa.fsx_xflags & !(FS_XFLAG_RDONLY_MASK as u32));
    (*fa).fsx_extsize = xfa.fsx_extsize;
    (*fa).fsx_nextents = xfa.fsx_nextents;
    (*fa).fsx_projid = xfa.fsx_projid;
    (*fa).fsx_cowextsize = xfa.fsx_cowextsize;
    0
}

// Generic validation and syscall wrappers retain the kernel's external types and helpers.
pub unsafe fn vfs_fileattr_set(idmap: *mut mnt_idmap, dentry: *mut dentry, fa: *mut file_kattr) -> i32 {
    let inode = d_inode(dentry);
    let mut old_ma: file_kattr = core::mem::zeroed();
    if (*(*inode).i_op).fileattr_set.is_none() { return -ENOIOCTLCMD; }
    if !inode_owner_or_capable(idmap, inode) { return -EPERM; }
    inode_lock(inode);
    let mut err = vfs_fileattr_get(dentry, &mut old_ma);
    if err == 0 {
        if (*fa).flags_valid {
            (*fa).fsx_xflags |= old_ma.fsx_xflags & !FS_XFLAG_COMMON;
            (*fa).fsx_extsize = old_ma.fsx_extsize;
            (*fa).fsx_nextents = old_ma.fsx_nextents;
            (*fa).fsx_projid = old_ma.fsx_projid;
            (*fa).fsx_cowextsize = old_ma.fsx_cowextsize;
        } else { (*fa).flags |= old_ma.flags & !FS_COMMON_FL; }
        err = fileattr_set_prepare(inode, &old_ma, fa);
        if err == 0 { err = security_inode_file_setattr(dentry, fa); }
        if err == 0 { err = ((*(*inode).i_op).fileattr_set.unwrap())(idmap, dentry, fa); }
        if err == 0 { fsnotify_xattr(dentry); }
    }
    inode_unlock(inode);
    err
}

unsafe fn fileattr_set_prepare(inode: *mut inode, old_ma: *const file_kattr, fa: *mut file_kattr) -> i32 {
    if ((*fa).flags ^ (*old_ma).flags) & (FS_APPEND_FL | FS_IMMUTABLE_FL) != 0 && !capable(CAP_LINUX_IMMUTABLE) { return -EPERM; }
    let mut err = fscrypt_prepare_setflags(inode, (*old_ma).flags, (*fa).flags);
    if err != 0 { return err; }
    if current_user_ns() != &init_user_ns {
        if (*old_ma).fsx_projid != (*fa).fsx_projid || ((*old_ma).fsx_xflags ^ (*fa).fsx_xflags) & FS_XFLAG_PROJINHERIT != 0 { return -EINVAL; }
    } else if (*old_ma).fsx_projid != (*fa).fsx_projid && !projid_valid(make_kprojid(&init_user_ns, (*fa).fsx_projid)) { return -EINVAL; }
    if (*fa).fsx_xflags & FS_XFLAG_EXTSIZE != 0 && !S_ISREG((*inode).i_mode) { return -EINVAL; }
    if (*fa).fsx_xflags & FS_XFLAG_EXTSZINHERIT != 0 && !S_ISDIR((*inode).i_mode) { return -EINVAL; }
    if (*fa).fsx_xflags & FS_XFLAG_COWEXTSIZE != 0 && !S_ISREG((*inode).i_mode) && !S_ISDIR((*inode).i_mode) { return -EINVAL; }
    if (*fa).fsx_xflags & FS_XFLAG_DAX != 0 && !S_ISREG((*inode).i_mode) && !S_ISDIR((*inode).i_mode) { return -EINVAL; }
    if (*fa).fsx_extsize == 0 { (*fa).fsx_xflags &= !(FS_XFLAG_EXTSIZE | FS_XFLAG_EXTSZINHERIT); }
    if (*fa).fsx_cowextsize == 0 { (*fa).fsx_xflags &= !FS_XFLAG_COWEXTSIZE; }
    err
}

pub unsafe fn ioctl_getflags(file: *mut file, argp: *mut u32) -> i32 {
    let mut fa: file_kattr = core::mem::zeroed(); fa.flags_valid = true;
    let err = vfs_fileattr_get((*file).f_path.dentry, &mut fa);
    if err == 0 { put_user(fa.flags, argp) } else { err }
}

pub unsafe fn ioctl_setflags(file: *mut file, argp: *mut u32) -> i32 {
    let mut flags = 0u32; let mut fa: file_kattr = core::mem::zeroed();
    let mut err = get_user(&mut flags, argp);
    if err == 0 { err = mnt_want_write_file(file); }
    if err == 0 { fileattr_fill_flags(&mut fa, flags); err = vfs_fileattr_set(file_mnt_idmap(file), (*file).f_path.dentry, &mut fa); mnt_drop_write_file(file); }
    err
}

pub unsafe fn ioctl_fsgetxattr(file: *mut file, argp: *mut core::ffi::c_void) -> i32 {
    let mut fa: file_kattr = core::mem::zeroed(); fa.fsx_valid = true;
    let err = vfs_fileattr_get((*file).f_path.dentry, &mut fa);
    if err == 0 { copy_fsxattr_to_user(&fa, argp as *mut fsxattr) } else { err }
}

pub unsafe fn ioctl_fssetxattr(file: *mut file, argp: *mut core::ffi::c_void) -> i32 {
    let mut fa: file_kattr = core::mem::zeroed(); let mut err = copy_fsxattr_from_user(&mut fa, argp as *mut fsxattr);
    if err == 0 { err = mnt_want_write_file(file); }
    if err == 0 { err = vfs_fileattr_set(file_mnt_idmap(file), (*file).f_path.dentry, &mut fa); mnt_drop_write_file(file); }
    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
