// SPDX-License-Identifier: GPL-2.0-only
// Translated from linux/fs/open.c. Kernel types, constants, macros, and
// external functions are supplied by the surrounding kernel translation.

#[allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]
extern "C" {
    // External kernel declarations are intentionally not redefined here.
}

pub unsafe fn do_truncate(idmap: *mut mnt_idmap, dentry: *mut dentry,
                          length: loff_t, time_attrs: c_uint,
                          filp: *mut file) -> c_int {
    if length < 0 { return -EINVAL; }
    let mut newattrs: iattr = core::mem::zeroed();
    newattrs.ia_size = length;
    newattrs.ia_valid = ATTR_SIZE | time_attrs;
    if !filp.is_null() { newattrs.ia_file = filp; newattrs.ia_valid |= ATTR_FILE; }
    let mut ret = dentry_needs_remove_privs(idmap, dentry);
    if ret < 0 { return ret; }
    if ret != 0 { newattrs.ia_valid |= ret | ATTR_FORCE; }
    ret = inode_lock_killable((*dentry).d_inode);
    if ret != 0 { return ret; }
    ret = notify_change(idmap, dentry, &mut newattrs, core::ptr::null_mut());
    inode_unlock((*dentry).d_inode);
    ret
}

pub unsafe fn vfs_truncate(path: *const path, length: loff_t) -> c_int {
    let inode = (*(*path).dentry).d_inode;
    if S_ISDIR((*inode).i_mode) { return -EISDIR; }
    if !S_ISREG((*inode).i_mode) { return -EINVAL; }
    let idmap = mnt_idmap((*path).mnt);
    let mut error = inode_permission(idmap, inode, MAY_WRITE);
    if error != 0 { return error; }
    error = fsnotify_truncate_perm(path, length); if error != 0 { return error; }
    error = mnt_want_write((*path).mnt); if error != 0 { return error; }
    error = -EPERM;
    if IS_APPEND(inode) { mnt_drop_write((*path).mnt); return error; }
    error = get_write_access(inode);
    if error != 0 { mnt_drop_write((*path).mnt); return error; }
    error = break_lease(inode, O_WRONLY);
    if error == 0 { error = security_path_truncate(path); }
    if error == 0 { error = do_truncate(idmap, (*path).dentry, length, 0, core::ptr::null_mut()); }
    put_write_access(inode); mnt_drop_write((*path).mnt); error
}

pub unsafe fn ksys_truncate(pathname: *const c_char, length: loff_t) -> c_int {
    if length < 0 { return -EINVAL; }
    let lookup_flags = LOOKUP_FOLLOW;
    let mut path: path = core::mem::zeroed();
    let name = filename_kernel_from_user(pathname);
    let mut error = filename_lookup(AT_FDCWD, name, lookup_flags, &mut path, core::ptr::null_mut());
    if error == 0 { error = vfs_truncate(&path, length); path_put(&mut path); }
    error
}

pub unsafe fn do_ftruncate(file: *mut file, length: loff_t, flags: c_uint) -> c_int {
    let dentry = (*file).f_path.dentry; let inode = (*dentry).d_inode;
    if !S_ISREG((*inode).i_mode) || ((*file).f_mode & FMODE_WRITE) == 0 { return -EINVAL; }
    if length > MAX_NON_LFS && ((*file).f_flags & O_LARGEFILE) == 0 && (flags & FTRUNCATE_LFS) == 0 { return -EINVAL; }
    if IS_APPEND(file_inode(file)) { return -EPERM; }
    let mut error = security_file_truncate(file); if error != 0 { return error; }
    error = fsnotify_truncate_perm(&(*file).f_path, length); if error != 0 { return error; }
    error = do_truncate(file_mnt_idmap(file), dentry, length, ATTR_MTIME | ATTR_CTIME, file); error
}

pub unsafe fn ksys_ftruncate(fd: c_uint, length: loff_t, flags: c_uint) -> c_int {
    if length < 0 { return -EINVAL; }
    let f = fd_get(fd); if f.is_null() { return -EBADF; }
    do_ftruncate(fd_file(f), length, flags)
}

pub unsafe fn vfs_fallocate(file: *mut file, mode: c_int, offset: loff_t, len: loff_t) -> c_int {
    let inode = file_inode(file); if offset < 0 || len <= 0 { return -EINVAL; }
    if (mode & !(FALLOC_FL_MODE_MASK | FALLOC_FL_KEEP_SIZE)) != 0 { return -EOPNOTSUPP; }
    match mode & FALLOC_FL_MODE_MASK {
        FALLOC_FL_ALLOCATE_RANGE | FALLOC_FL_UNSHARE_RANGE | FALLOC_FL_ZERO_RANGE => (),
        FALLOC_FL_PUNCH_HOLE if mode & FALLOC_FL_KEEP_SIZE != 0 => (),
        FALLOC_FL_COLLAPSE_RANGE | FALLOC_FL_INSERT_RANGE | FALLOC_FL_WRITE_ZEROES
            if mode & FALLOC_FL_KEEP_SIZE == 0 => (),
        _ => return -EOPNOTSUPP,
    }
    if (*file).f_mode & FMODE_WRITE == 0 { return -EBADF; }
    if mode & !FALLOC_FL_KEEP_SIZE != 0 && IS_APPEND(inode) { return -EPERM; }
    if IS_IMMUTABLE(inode) { return -EPERM; }
    if IS_SWAPFILE(inode) { return -ETXTBSY; }
    let mut ret = security_file_permission(file, MAY_WRITE); if ret != 0 { return ret; }
    ret = fsnotify_file_area_perm(file, MAY_WRITE, &offset, len); if ret != 0 { return ret; }
    if S_ISFIFO((*inode).i_mode) { return -ESPIPE; }
    if S_ISDIR((*inode).i_mode) { return -EISDIR; }
    if !S_ISREG((*inode).i_mode) && !S_ISBLK((*inode).i_mode) { return -ENODEV; }
    let sum = offset.checked_add(len).ok_or(-EFBIG); if sum.is_err() { return -EFBIG; }
    if sum.unwrap() > (*(*inode).i_sb).s_maxbytes { return -EFBIG; }
    if (*file).f_op.is_null() || (*(*file).f_op).fallocate.is_none() { return -EOPNOTSUPP; }
    file_start_write(file); ret = ((*(*file).f_op).fallocate.unwrap())(file, mode, offset, len);
    if ret == 0 { fsnotify_modify(file); } file_end_write(file); ret
}

pub unsafe fn chmod_common(path: *const path, mode: umode_t) -> c_int {
    let inode = (*(*path).dentry).d_inode; let mut delegated: delegated_inode = core::mem::zeroed();
    let mut attrs: iattr = core::mem::zeroed(); let mut error = mnt_want_write((*path).mnt);
    if error != 0 { return error; }
    error = inode_lock_killable(inode); if error == 0 { error = security_path_chmod(path, mode); }
    if error == 0 { attrs.ia_mode = (mode & S_IALLUGO) | ((*inode).i_mode & !S_IALLUGO); attrs.ia_valid = ATTR_MODE | ATTR_CTIME; error = notify_change(mnt_idmap((*path).mnt), (*path).dentry, &mut attrs, &mut delegated); }
    inode_unlock(inode); if is_delegated(&delegated) && error == 0 { error = break_deleg_wait(&mut delegated); }
    mnt_drop_write((*path).mnt); error
}

pub unsafe fn nonseekable_open(_inode: *mut inode, filp: *mut file) -> c_int { (*filp).f_mode &= !(FMODE_LSEEK|FMODE_PREAD|FMODE_PWRITE); 0 }
pub unsafe fn stream_open(_inode: *mut inode, filp: *mut file) -> c_int { (*filp).f_mode &= !(FMODE_LSEEK|FMODE_PREAD|FMODE_PWRITE|FMODE_ATOMIC_POS); (*filp).f_mode |= FMODE_STREAM; 0 }

pub unsafe fn generic_file_open(inode: *mut inode, filp: *mut file) -> c_int {
    if (*filp).f_flags & O_LARGEFILE == 0 && i_size_read(inode) > MAX_NON_LFS { return -EOVERFLOW; } 0
}

pub unsafe fn build_open_how(flags: c_int, mode: umode_t) -> open_how {
    let mut how = open_how { flags: (flags as u64) & VALID_OPEN_FLAGS, mode: mode & S_IALLUGO, resolve: 0 };
    if how.flags & O_PATH != 0 { how.flags &= O_PATH_FLAGS; }
    if how.flags & (O_CREAT | __O_TMPFILE) == 0 { how.mode = 0; } how
}

pub unsafe fn build_open_flags(how: *const open_how, op: *mut open_flags) -> c_int {
    let mut flags = (*how).flags; let mut lookup_flags: c_int = 0; let mut acc_mode = ACC_MODE(flags);
    flags &= !O_CLOEXEC;
    if flags & !VALID_OPENAT2_FLAGS != 0 || (*how).resolve & !VALID_RESOLVE_FLAGS != 0 { return -EINVAL; }
    if (*how).resolve & RESOLVE_BENEATH != 0 && (*how).resolve & RESOLVE_IN_ROOT != 0 { return -EINVAL; }
    if flags & (O_CREAT | __O_TMPFILE) != 0 { if (*how).mode & !S_IALLUGO != 0 { return -EINVAL; } (*op).mode = (*how).mode | S_IFREG; }
    else { if (*how).mode != 0 { return -EINVAL; } (*op).mode = 0; }
    if flags & (O_DIRECTORY | O_CREAT) == (O_DIRECTORY | O_CREAT) { return -EINVAL; }
    if flags & __O_TMPFILE != 0 && (flags & O_DIRECTORY == 0 || acc_mode & MAY_WRITE == 0) { return -EINVAL; }
    if flags & O_PATH != 0 { if flags & !O_PATH_FLAGS != 0 { return -EINVAL; } acc_mode = 0; }
    if flags & __O_SYNC != 0 { flags |= O_DSYNC; }
    if flags & OPENAT2_REGULAR != 0 { flags = (flags & !OPENAT2_REGULAR) | __O_REGULAR; }
    (*op).open_flag = flags as c_int; if flags & O_TRUNC != 0 { acc_mode |= MAY_WRITE; }
    if flags & O_APPEND != 0 { acc_mode |= MAY_APPEND; } (*op).acc_mode = acc_mode;
    (*op).intent = if flags & O_PATH != 0 { 0 } else { LOOKUP_OPEN };
    if flags & O_CREAT != 0 { (*op).intent |= LOOKUP_CREATE; if flags & O_EXCL != 0 { (*op).intent |= LOOKUP_EXCL; flags |= O_NOFOLLOW; } }
    if flags & O_DIRECTORY != 0 { lookup_flags |= LOOKUP_DIRECTORY; } if flags & O_NOFOLLOW == 0 { lookup_flags |= LOOKUP_FOLLOW; }
    if flags & O_EMPTYPATH != 0 { lookup_flags |= LOOKUP_EMPTY; }
    if (*how).resolve & RESOLVE_NO_XDEV != 0 { lookup_flags |= LOOKUP_NO_XDEV; }
    if (*how).resolve & RESOLVE_NO_MAGICLINKS != 0 { lookup_flags |= LOOKUP_NO_MAGICLINKS; }
    if (*how).resolve & RESOLVE_NO_SYMLINKS != 0 { lookup_flags |= LOOKUP_NO_SYMLINKS; }
    if (*how).resolve & RESOLVE_BENEATH != 0 { lookup_flags |= LOOKUP_BENEATH; }
    if (*how).resolve & RESOLVE_IN_ROOT != 0 { lookup_flags |= LOOKUP_IN_ROOT; }
    if (*how).resolve & RESOLVE_CACHED != 0 { if flags & (O_TRUNC|O_CREAT|__O_TMPFILE) != 0 { return -EAGAIN; } lookup_flags |= LOOKUP_CACHED; }
    (*op).lookup_flags = lookup_flags; 0
}

pub unsafe fn do_sys_open(dfd: c_int, filename: *const c_char, flags: c_int, mode: umode_t) -> c_int {
    let mut how = build_open_how(flags, mode); if force_o_largefile() != 0 { how.flags |= O_LARGEFILE as u64; }
    let mut op: open_flags = core::mem::zeroed(); let err = build_open_flags(&how, &mut op); if err != 0 { return err; }
    let name = filename_kernel_from_user(filename); FD_ADD(how.flags, do_file_open(dfd, name, &mut op))
}

pub unsafe fn nonseekable_open_alias(inode: *mut inode, filp: *mut file) -> c_int { nonseekable_open(inode, filp) }

pub unsafe fn filp_close(filp: *mut file, id: fl_owner_t) -> c_int { let r = filp_flush(filp, id); fput_close(filp); r }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
