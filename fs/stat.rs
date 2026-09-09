// SPDX-License-Identifier: GPL-2.0
/* Literal Rust translation of linux/fs/stat.c.  Kernel-provided types,
 * constants, macros, and external functions are intentionally unresolved here. */

pub unsafe fn fill_mg_cmtime(stat: *mut kstat, request_mask: u32, inode: *mut inode) {
    let pcn = &mut (*inode).i_ctime_nsec as *mut _ as *mut atomic_t;
    if request_mask & (STATX_CTIME | STATX_MTIME) == 0 {
        (*stat).result_mask &= !(STATX_CTIME | STATX_MTIME);
        return;
    }
    (*stat).mtime = inode_get_mtime(inode);
    (*stat).ctime.tv_sec = inode_get_ctime_sec(inode);
    (*stat).ctime.tv_nsec = atomic_read(pcn) as u32;
    if (*stat).ctime.tv_nsec & I_CTIME_QUERIED == 0 {
        (*stat).ctime.tv_nsec = atomic_fetch_or(I_CTIME_QUERIED, pcn) as u32;
    }
    (*stat).ctime.tv_nsec &= !I_CTIME_QUERIED;
    trace_fill_mg_cmtime(inode, &mut (*stat).ctime, &mut (*stat).mtime);
}

pub unsafe fn generic_fillattr(idmap: *mut mnt_idmap, request_mask: u32,
                               inode: *mut inode, stat: *mut kstat) {
    let vfsuid = i_uid_into_vfsuid(idmap, inode);
    let vfsgid = i_gid_into_vfsgid(idmap, inode);
    (*stat).dev = (*(*inode).i_sb).s_dev;
    (*stat).ino = (*inode).i_ino;
    (*stat).mode = (*inode).i_mode;
    (*stat).nlink = (*inode).i_nlink;
    (*stat).uid = vfsuid_into_kuid(vfsuid);
    (*stat).gid = vfsgid_into_kgid(vfsgid);
    (*stat).rdev = (*inode).i_rdev;
    (*stat).size = i_size_read(inode);
    (*stat).atime = inode_get_atime(inode);
    if is_mgtime(inode) { fill_mg_cmtime(stat, request_mask, inode); }
    else { (*stat).ctime = inode_get_ctime(inode); (*stat).mtime = inode_get_mtime(inode); }
    (*stat).blksize = i_blocksize(inode);
    (*stat).blocks = (*inode).i_blocks;
    if request_mask & STATX_CHANGE_COOKIE != 0 && IS_I_VERSION(inode) {
        (*stat).result_mask |= STATX_CHANGE_COOKIE;
        (*stat).change_cookie = inode_query_iversion(inode);
    }
}

pub unsafe fn generic_fill_statx_attr(inode: *mut inode, stat: *mut kstat) {
    if (*inode).i_flags & S_IMMUTABLE != 0 { (*stat).attributes |= STATX_ATTR_IMMUTABLE; }
    if (*inode).i_flags & S_APPEND != 0 { (*stat).attributes |= STATX_ATTR_APPEND; }
    (*stat).attributes_mask |= KSTAT_ATTR_VFS_FLAGS;
}

pub unsafe fn generic_fill_statx_atomic_writes(stat: *mut kstat, unit_min: u32,
                                                unit_max: u32, unit_max_opt: u32) {
    (*stat).result_mask |= STATX_WRITE_ATOMIC;
    (*stat).attributes_mask |= STATX_ATTR_WRITE_ATOMIC;
    if unit_min != 0 {
        (*stat).atomic_write_unit_min = unit_min;
        (*stat).atomic_write_unit_max = unit_max;
        (*stat).atomic_write_unit_max_opt = unit_max_opt;
        (*stat).atomic_write_segments_max = 1;
        (*stat).attributes |= STATX_ATTR_WRITE_ATOMIC;
    }
}

pub unsafe fn vfs_getattr_nosec(path: *const path, stat: *mut kstat,
                                request_mask: u32, mut query_flags: u32) -> i32 {
    let inode = d_backing_inode((*path).dentry);
    core::ptr::write_bytes(stat, 0, 1);
    (*stat).result_mask |= STATX_BASIC_STATS;
    query_flags &= AT_STATX_SYNC_TYPE;
    if (*(*inode).i_sb).s_flags & SB_NOATIME != 0 { (*stat).result_mask &= !STATX_ATIME; }
    if IS_AUTOMOUNT(inode) { (*stat).attributes |= STATX_ATTR_AUTOMOUNT; }
    if IS_DAX(inode) { (*stat).attributes |= STATX_ATTR_DAX; }
    (*stat).attributes_mask |= STATX_ATTR_AUTOMOUNT | STATX_ATTR_DAX;
    let idmap = mnt_idmap((*path).mnt);
    if !(*(*inode).i_op).getattr.is_null() {
        let ret = ((*(*inode).i_op).getattr)(idmap, path, stat, request_mask, query_flags);
        if ret != 0 { return ret; }
    } else { generic_fillattr(idmap, request_mask, inode, stat); }
    if S_ISBLK((*stat).mode) { bdev_statx(path, stat, request_mask); }
    0
}

pub unsafe fn vfs_getattr(path: *const path, stat: *mut kstat, request_mask: u32, query_flags: u32) -> i32 {
    let ret = security_inode_getattr(path);
    if ret != 0 { return ret; }
    vfs_getattr_nosec(path, stat, request_mask, query_flags)
}

pub unsafe fn vfs_fstat(fd: i32, stat: *mut kstat) -> i32 {
    let f = fd_file_raw(fd);
    if f.is_null() { return -EBADF; }
    vfs_getattr(&(*f).f_path, stat, STATX_BASIC_STATS, 0)
}

unsafe fn statx_lookup_flags(flags: i32) -> i32 {
    let mut lookup_flags = 0;
    if flags & AT_SYMLINK_NOFOLLOW == 0 { lookup_flags |= LOOKUP_FOLLOW; }
    if flags & AT_NO_AUTOMOUNT == 0 { lookup_flags |= LOOKUP_AUTOMOUNT; }
    lookup_flags
}

unsafe fn vfs_statx_path(path: *const path, flags: i32, stat: *mut kstat, request_mask: u32) -> i32 {
    let error = vfs_getattr(path, stat, request_mask, flags as u32);
    if error != 0 { return error; }
    if request_mask & STATX_MNT_ID_UNIQUE != 0 { (*stat).mnt_id = (*real_mount((*path).mnt)).mnt_id_unique; (*stat).result_mask |= STATX_MNT_ID_UNIQUE; }
    else { (*stat).mnt_id = (*real_mount((*path).mnt)).mnt_id; (*stat).result_mask |= STATX_MNT_ID; }
    if path_mounted(path) { (*stat).attributes |= STATX_ATTR_MOUNT_ROOT; }
    (*stat).attributes_mask |= STATX_ATTR_MOUNT_ROOT;
    0
}

unsafe fn vfs_statx_fd(fd: i32, flags: i32, stat: *mut kstat, request_mask: u32) -> i32 {
    let f = fd_file_raw(fd);
    if f.is_null() { return -EBADF; }
    vfs_statx_path(&(*f).f_path, flags, stat, request_mask)
}

unsafe fn vfs_statx(dfd: i32, filename: *mut filename, flags: i32, stat: *mut kstat, request_mask: u32) -> i32 {
    if flags & !(AT_SYMLINK_NOFOLLOW | AT_NO_AUTOMOUNT | AT_EMPTY_PATH | AT_STATX_SYNC_TYPE) != 0 { return -EINVAL; }
    let mut lookup_flags = statx_lookup_flags(flags);
    loop {
        let mut path = core::mem::MaybeUninit::<path>::uninit();
        let mut error = filename_lookup(dfd, filename, lookup_flags, path.as_mut_ptr(), core::ptr::null_mut());
        if error == 0 { error = vfs_statx_path(path.as_ptr(), flags, stat, request_mask); path_put(path.as_mut_ptr()); }
        if retry_estale(error, lookup_flags) { lookup_flags |= LOOKUP_REVAL; continue; }
        return error;
    }
}

pub unsafe fn vfs_fstatat(dfd: i32, filename: *const core::ffi::c_char, stat: *mut kstat, flags: i32) -> i32 {
    if filename.is_null() && dfd >= 0 { return vfs_fstat(dfd, stat); }
    vfs_statx(dfd, filename_maybe_null(filename, flags), flags | AT_NO_AUTOMOUNT, stat, STATX_BASIC_STATS)
}

unsafe fn do_readlinkat(dfd: i32, pathname: *const core::ffi::c_char, buf: *mut core::ffi::c_char, bufsiz: i32) -> i32 {
    if bufsiz <= 0 { return -EINVAL; }
    let name = filename_flags(pathname, LOOKUP_EMPTY);
    let mut lookup_flags = 0;
    loop {
        let mut p = core::mem::MaybeUninit::<path>::uninit();
        let mut error = filename_lookup(dfd, name, lookup_flags, p.as_mut_ptr(), core::ptr::null_mut());
        if error != 0 { return error; }
        let pp = p.as_mut_ptr();
        if d_is_symlink((*pp).dentry) || !(*(*d_backing_inode((*pp).dentry)).i_op).readlink.is_null() {
            error = security_inode_readlink((*pp).dentry);
            if error == 0 { touch_atime(pp); error = vfs_readlink((*pp).dentry, buf, bufsiz); }
        } else { error = if (*name).name[0] == 0 { -ENOENT } else { -EINVAL }; }
        path_put(pp);
        if retry_estale(error, lookup_flags) { lookup_flags |= LOOKUP_REVAL; continue; }
        return error;
    }
}

pub unsafe fn readlinkat(dfd: i32, pathname: *const core::ffi::c_char, buf: *mut core::ffi::c_char, bufsiz: i32) -> i32 { do_readlinkat(dfd, pathname, buf, bufsiz) }
pub unsafe fn readlink(pathname: *const core::ffi::c_char, buf: *mut core::ffi::c_char, bufsiz: i32) -> i32 { do_readlinkat(AT_FDCWD, pathname, buf, bufsiz) }

unsafe fn cp_statx(stat: *const kstat, buffer: *mut statx) -> i32 {
    let mut tmp: statx = core::mem::zeroed();
    tmp.stx_mask = (*stat).result_mask & !STATX_CHANGE_COOKIE;
    tmp.stx_blksize = (*stat).blksize;
    tmp.stx_attributes = (*stat).attributes & !STATX_ATTR_CHANGE_MONOTONIC;
    tmp.stx_nlink = (*stat).nlink; tmp.stx_uid = from_kuid_munged(current_user_ns(), (*stat).uid); tmp.stx_gid = from_kgid_munged(current_user_ns(), (*stat).gid);
    tmp.stx_mode = (*stat).mode; tmp.stx_ino = (*stat).ino; tmp.stx_size = (*stat).size; tmp.stx_blocks = (*stat).blocks; tmp.stx_attributes_mask = (*stat).attributes_mask;
    tmp.stx_atime.tv_sec = (*stat).atime.tv_sec; tmp.stx_atime.tv_nsec = (*stat).atime.tv_nsec; tmp.stx_btime.tv_sec = (*stat).btime.tv_sec; tmp.stx_btime.tv_nsec = (*stat).btime.tv_nsec;
    tmp.stx_ctime.tv_sec = (*stat).ctime.tv_sec; tmp.stx_ctime.tv_nsec = (*stat).ctime.tv_nsec; tmp.stx_mtime.tv_sec = (*stat).mtime.tv_sec; tmp.stx_mtime.tv_nsec = (*stat).mtime.tv_nsec;
    tmp.stx_rdev_major = MAJOR((*stat).rdev); tmp.stx_rdev_minor = MINOR((*stat).rdev); tmp.stx_dev_major = MAJOR((*stat).dev); tmp.stx_dev_minor = MINOR((*stat).dev);
    tmp.stx_mnt_id = (*stat).mnt_id; tmp.stx_dio_mem_align = (*stat).dio_mem_align; tmp.stx_dio_offset_align = (*stat).dio_offset_align; tmp.stx_dio_read_offset_align = (*stat).dio_read_offset_align; tmp.stx_subvol = (*stat).subvol;
    tmp.stx_atomic_write_unit_min = (*stat).atomic_write_unit_min; tmp.stx_atomic_write_unit_max = (*stat).atomic_write_unit_max; tmp.stx_atomic_write_segments_max = (*stat).atomic_write_segments_max; tmp.stx_atomic_write_unit_max_opt = (*stat).atomic_write_unit_max_opt;
    if copy_to_user(buffer, &tmp, core::mem::size_of::<statx>()) != 0 { -EFAULT } else { 0 }
}

pub unsafe fn do_statx(dfd: i32, filename: *mut filename, flags: u32, mut mask: u32, buffer: *mut statx) -> i32 {
    if mask & STATX__RESERVED != 0 || flags & AT_STATX_SYNC_TYPE == AT_STATX_SYNC_TYPE { return -EINVAL; }
    mask &= !STATX_CHANGE_COOKIE;
    let mut stat = core::mem::MaybeUninit::<kstat>::uninit();
    let error = vfs_statx(dfd, filename, flags as i32, stat.as_mut_ptr(), mask);
    if error != 0 { return error; }
    cp_statx(stat.as_ptr(), buffer)
}

pub unsafe fn do_statx_fd(fd: i32, flags: u32, mut mask: u32, buffer: *mut statx) -> i32 {
    if mask & STATX__RESERVED != 0 || flags & AT_STATX_SYNC_TYPE == AT_STATX_SYNC_TYPE { return -EINVAL; }
    mask &= !STATX_CHANGE_COOKIE;
    let mut stat = core::mem::MaybeUninit::<kstat>::uninit();
    let error = vfs_statx_fd(fd, flags as i32, stat.as_mut_ptr(), mask);
    if error != 0 { return error; }
    cp_statx(stat.as_ptr(), buffer)
}

pub unsafe fn statx(dfd: i32, filename: *const core::ffi::c_char, flags: u32, mask: u32, buffer: *mut statx) -> i32 {
    if filename.is_null() && dfd >= 0 { return do_statx_fd(dfd, flags & !AT_NO_AUTOMOUNT, mask, buffer); }
    do_statx(dfd, filename_maybe_null(filename, flags as i32), flags, mask, buffer)
}

pub unsafe fn __inode_add_bytes(inode: *mut inode, mut bytes: i64) {
    (*inode).i_blocks += bytes >> 9; bytes &= 511; (*inode).i_bytes += bytes;
    if (*inode).i_bytes >= 512 { (*inode).i_blocks += 1; (*inode).i_bytes -= 512; }
}
pub unsafe fn inode_add_bytes(inode: *mut inode, bytes: i64) { spin_lock(&mut (*inode).i_lock); __inode_add_bytes(inode, bytes); spin_unlock(&mut (*inode).i_lock); }
pub unsafe fn __inode_sub_bytes(inode: *mut inode, mut bytes: i64) {
    (*inode).i_blocks -= bytes >> 9; bytes &= 511;
    if (*inode).i_bytes < bytes { (*inode).i_blocks -= 1; (*inode).i_bytes += 512; }
    (*inode).i_bytes -= bytes;
}
pub unsafe fn inode_sub_bytes(inode: *mut inode, bytes: i64) { spin_lock(&mut (*inode).i_lock); __inode_sub_bytes(inode, bytes); spin_unlock(&mut (*inode).i_lock); }
pub unsafe fn inode_get_bytes(inode: *mut inode) -> i64 { spin_lock(&mut (*inode).i_lock); let ret = __inode_get_bytes(inode); spin_unlock(&mut (*inode).i_lock); ret }
pub unsafe fn inode_set_bytes(inode: *mut inode, bytes: i64) { (*inode).i_blocks = bytes >> 9; (*inode).i_bytes = bytes & 511; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
