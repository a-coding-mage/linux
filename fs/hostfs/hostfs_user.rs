/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 * Licensed under the GPL
 */

// C headers and "hostfs.h" are external dependencies supplied by the surrounding build.

unsafe fn statx_to_hostfs(buf: *const statx, p: *mut hostfs_stat) {
    (*p).ino = (*buf).stx_ino;
    (*p).mode = (*buf).stx_mode;
    (*p).nlink = (*buf).stx_nlink;
    (*p).uid = (*buf).stx_uid;
    (*p).gid = (*buf).stx_gid;
    (*p).size = (*buf).stx_size;
    (*p).atime.tv_sec = (*buf).stx_atime.tv_sec;
    (*p).atime.tv_nsec = (*buf).stx_atime.tv_nsec;
    (*p).ctime.tv_sec = (*buf).stx_ctime.tv_sec;
    (*p).ctime.tv_nsec = (*buf).stx_ctime.tv_nsec;
    (*p).mtime.tv_sec = (*buf).stx_mtime.tv_sec;
    (*p).mtime.tv_nsec = (*buf).stx_mtime.tv_nsec;
    if ((*buf).stx_mask & STATX_BTIME) != 0 {
        (*p).btime.tv_sec = (*buf).stx_btime.tv_sec;
        (*p).btime.tv_nsec = (*buf).stx_btime.tv_nsec;
    } else {
        memset(&mut (*p).btime as *mut _, 0, core::mem::size_of_val(&(*p).btime));
    }
    (*p).blksize = (*buf).stx_blksize;
    (*p).blocks = (*buf).stx_blocks;
    (*p).rdev.maj = (*buf).stx_rdev_major;
    (*p).rdev.min = (*buf).stx_rdev_minor;
    (*p).dev.maj = (*buf).stx_dev_major;
    (*p).dev.min = (*buf).stx_dev_minor;
}

pub unsafe fn stat_file(path: *const c_char, p: *mut hostfs_stat, fd: c_int) -> c_int {
    let mut buf: statx = core::mem::zeroed();
    let mut flags: c_int = AT_SYMLINK_NOFOLLOW;
    if fd >= 0 {
        flags |= AT_EMPTY_PATH;
        path = b"\0".as_ptr() as *const c_char;
    }
    if statx(fd, path, flags, STATX_BASIC_STATS | STATX_BTIME, &mut buf) < 0 {
        return -errno;
    }
    statx_to_hostfs(&buf, p);
    0
}

pub unsafe fn access_file(path: *mut c_char, r: c_int, w: c_int, x: c_int) -> c_int {
    let mut mode = 0;
    if r != 0 { mode = R_OK; }
    if w != 0 { mode |= W_OK; }
    if x != 0 { mode |= X_OK; }
    if access(path, mode) != 0 { -errno } else { 0 }
}

pub unsafe fn open_file(path: *mut c_char, r: c_int, w: c_int, append: c_int) -> c_int {
    let mut mode: c_int;
    if r != 0 && w == 0 { mode = O_RDONLY; }
    else if r == 0 && w != 0 { mode = O_WRONLY; }
    else if r != 0 && w != 0 { mode = O_RDWR; }
    else { panic("Impossible mode in open_file"); }
    if append != 0 { mode |= O_APPEND; }
    let fd = open64(path, mode);
    if fd < 0 { -errno } else { fd }
}

pub unsafe fn open_dir(path: *mut c_char, err_out: *mut c_int) -> *mut DIR {
    let dir = opendir(path);
    *err_out = errno;
    dir
}

pub unsafe fn seek_dir(stream: *mut c_void, pos: c_ulonglong) { seekdir(stream as *mut DIR, pos); }

pub unsafe fn read_dir(stream: *mut c_void, pos_out: *mut c_ulonglong, ino_out: *mut c_ulonglong,
                       len_out: *mut c_int, type_out: *mut c_uint) -> *mut c_char {
    let ent = readdir(stream as *mut DIR);
    if ent.is_null() { return core::ptr::null_mut(); }
    *len_out = strlen((*ent).d_name) as c_int;
    *ino_out = (*ent).d_ino;
    *type_out = (*ent).d_type as c_uint;
    *pos_out = (*ent).d_off as c_ulonglong;
    (*ent).d_name.as_mut_ptr()
}

pub unsafe fn read_file(fd: c_int, offset: *mut c_ulonglong, buf: *mut c_char, len: c_int) -> c_int {
    let n = pread64(fd, buf as *mut c_void, len as usize, *offset);
    if n < 0 { return -errno; }
    *offset += n as c_ulonglong;
    n as c_int
}

pub unsafe fn write_file(fd: c_int, offset: *mut c_ulonglong, buf: *const c_char, len: c_int) -> c_int {
    let n = pwrite64(fd, buf as *const c_void, len as usize, *offset);
    if n < 0 { return -errno; }
    *offset += n as c_ulonglong;
    n as c_int
}

pub unsafe fn lseek_file(fd: c_int, offset: c_longlong, whence: c_int) -> c_int {
    if lseek64(fd, offset, whence) < 0 { -errno } else { 0 }
}

pub unsafe fn fsync_file(fd: c_int, datasync: c_int) -> c_int {
    let ret = if datasync != 0 { fdatasync(fd) } else { fsync(fd) };
    if ret < 0 { -errno } else { 0 }
}

pub unsafe fn replace_file(oldfd: c_int, fd: c_int) -> c_int { dup2(oldfd, fd) }
pub unsafe fn close_file(stream: *mut c_void) { close(*(stream as *mut c_int)); }
pub unsafe fn close_dir(stream: *mut c_void) { closedir(stream as *mut DIR); }

pub unsafe fn file_create(name: *mut c_char, mode: c_int) -> c_int {
    let fd = open64(name, O_CREAT | O_RDWR, mode);
    if fd < 0 { -errno } else { fd }
}

pub unsafe fn set_attr(file: *const c_char, attrs: *mut hostfs_iattr, fd: c_int) -> c_int {
    let mut st: hostfs_stat = core::mem::zeroed();
    let mut times: [timeval; 2] = core::mem::zeroed();
    let mut err: c_int;
    let ma = HOSTFS_ATTR_ATIME_SET | HOSTFS_ATTR_MTIME_SET;
    if (*attrs).ia_valid & HOSTFS_ATTR_MODE != 0 {
        if fd >= 0 { if fchmod(fd, (*attrs).ia_mode) != 0 { return -errno; } }
        else if chmod(file, (*attrs).ia_mode) != 0 { return -errno; }
    }
    if (*attrs).ia_valid & HOSTFS_ATTR_UID != 0 {
        if fd >= 0 { if fchown(fd, (*attrs).ia_uid, -1) != 0 { return -errno; } }
        else if chown(file, (*attrs).ia_uid, -1) != 0 { return -errno; }
    }
    if (*attrs).ia_valid & HOSTFS_ATTR_GID != 0 {
        if fd >= 0 { if fchown(fd, -1, (*attrs).ia_gid) != 0 { return -errno; } }
        else if chown(file, -1, (*attrs).ia_gid) != 0 { return -errno; }
    }
    if (*attrs).ia_valid & HOSTFS_ATTR_SIZE != 0 {
        if fd >= 0 { if ftruncate(fd, (*attrs).ia_size) != 0 { return -errno; } }
        else if truncate(file, (*attrs).ia_size) != 0 { return -errno; }
    }
    /* Update accessed and/or modified time in two parts, then apply them. */
    if (*attrs).ia_valid & ma != 0 {
        err = stat_file(file, &mut st, fd); if err != 0 { return err; }
        times[0].tv_sec = st.atime.tv_sec; times[0].tv_usec = st.atime.tv_nsec / 1000;
        times[1].tv_sec = st.mtime.tv_sec; times[1].tv_usec = st.mtime.tv_nsec / 1000;
        if (*attrs).ia_valid & HOSTFS_ATTR_ATIME_SET != 0 { times[0].tv_sec = (*attrs).ia_atime.tv_sec; times[0].tv_usec = (*attrs).ia_atime.tv_nsec / 1000; }
        if (*attrs).ia_valid & HOSTFS_ATTR_MTIME_SET != 0 { times[1].tv_sec = (*attrs).ia_mtime.tv_sec; times[1].tv_usec = (*attrs).ia_mtime.tv_nsec / 1000; }
        if fd >= 0 { if futimes(fd, times.as_ptr()) != 0 { return -errno; } }
        else if utimes(file, times.as_ptr()) != 0 { return -errno; }
    }
    /* Note: ctime is not handled */
    if (*attrs).ia_valid & (HOSTFS_ATTR_ATIME | HOSTFS_ATTR_MTIME) != 0 {
        err = stat_file(file, &mut st, fd);
        (*attrs).ia_atime = st.atime; (*attrs).ia_mtime = st.mtime;
        if err != 0 { return err; }
    }
    0
}

pub unsafe fn make_symlink(from: *const c_char, to: *const c_char) -> c_int { if symlink(to, from) != 0 { -errno } else { 0 } }
pub unsafe fn unlink_file(file: *const c_char) -> c_int { if unlink(file) != 0 { -errno } else { 0 } }
pub unsafe fn do_mkdir(file: *const c_char, mode: c_int) -> c_int { if mkdir(file, mode) != 0 { -errno } else { 0 } }
pub unsafe fn hostfs_do_rmdir(file: *const c_char) -> c_int { if rmdir(file) != 0 { -errno } else { 0 } }
pub unsafe fn do_mknod(file: *const c_char, mode: c_int, major: c_uint, minor: c_uint) -> c_int { if mknod(file, mode, os_makedev(major, minor)) != 0 { -errno } else { 0 } }
pub unsafe fn link_file(to: *const c_char, from: *const c_char) -> c_int { if link(to, from) != 0 { -errno } else { 0 } }

pub unsafe fn hostfs_do_readlink(file: *mut c_char, buf: *mut c_char, size: c_int) -> c_int {
    let n = readlink(file, buf, size as usize);
    if n < 0 { return -errno; }
    if n < size as isize { *buf.add(n as usize) = 0; }
    n as c_int
}

pub unsafe fn rename_file(from: *mut c_char, to: *mut c_char) -> c_int { if rename(from, to) < 0 { -errno } else { 0 } }

pub unsafe fn rename2_file(from: *mut c_char, to: *mut c_char, flags: c_uint) -> c_int {
    // SYS_renameat2 is supplied by the target platform; x86 fallback values are preserved from C.
    #[cfg(target_arch = "x86_64")] const SYS_RENAMEAT2: c_long = 316;
    #[cfg(target_arch = "x86")] const SYS_RENAMEAT2: c_long = 353;
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        let err = syscall(SYS_RENAMEAT2, AT_FDCWD, from, AT_FDCWD, to, flags);
        if err < 0 { if errno != ENOSYS { return -errno; } else { return -EINVAL; } }
        return 0;
    }
    -EINVAL
}

pub unsafe fn do_statfs(root: *mut c_char, bsize_out: *mut c_long, blocks_out: *mut c_longlong,
                        bfree_out: *mut c_longlong, bavail_out: *mut c_longlong,
                        files_out: *mut c_longlong, ffree_out: *mut c_longlong,
                        fsid_out: *mut c_void, fsid_size: c_int, namelen_out: *mut c_long) -> c_int {
    let mut buf: statfs64 = core::mem::zeroed();
    if statfs64(root, &mut buf) < 0 { return -errno; }
    *bsize_out = buf.f_bsize; *blocks_out = buf.f_blocks; *bfree_out = buf.f_bfree;
    *bavail_out = buf.f_bavail; *files_out = buf.f_files; *ffree_out = buf.f_ffree;
    core::ptr::copy_nonoverlapping(&buf.f_fsid as *const _, fsid_out as *mut _,
        core::cmp::min(core::mem::size_of_val(&buf.f_fsid), fsid_size as usize));
    *namelen_out = buf.f_namelen;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
