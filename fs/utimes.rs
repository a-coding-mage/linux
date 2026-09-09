// SPDX-License-Identifier: GPL-2.0
// External Linux kernel declarations are supplied by other translation units.

unsafe fn nsec_valid(nsec: libc::c_long) -> bool {
    if nsec == UTIME_OMIT || nsec == UTIME_NOW {
        return true;
    }
    nsec >= 0 && nsec <= 999_999_999
}

pub unsafe fn vfs_utimes(path: *const struct_path, times: *mut timespec64) -> libc::c_int {
    let mut error: libc::c_int;
    let mut newattrs: iattr = core::mem::zeroed();
    let inode = (*(*path).dentry).d_inode;
    let mut delegated_inode: delegated_inode = core::mem::zeroed();

    if !times.is_null() {
        if !nsec_valid((*times.add(0)).tv_nsec) || !nsec_valid((*times.add(1)).tv_nsec) {
            return -EINVAL;
        }
        if (*times.add(0)).tv_nsec == UTIME_NOW && (*times.add(1)).tv_nsec == UTIME_NOW {
            times = core::ptr::null_mut();
        }
    }

    error = mnt_want_write((*path).mnt);
    if error != 0 {
        return error;
    }

    (*(&mut newattrs)).ia_valid = ATTR_CTIME | ATTR_MTIME | ATTR_ATIME;
    if !times.is_null() {
        if (*times.add(0)).tv_nsec == UTIME_OMIT {
            newattrs.ia_valid &= !ATTR_ATIME;
        } else if (*times.add(0)).tv_nsec != UTIME_NOW {
            newattrs.ia_atime = *times.add(0);
            newattrs.ia_valid |= ATTR_ATIME_SET;
        }

        if (*times.add(1)).tv_nsec == UTIME_OMIT {
            newattrs.ia_valid &= !ATTR_MTIME;
        } else if (*times.add(1)).tv_nsec != UTIME_NOW {
            newattrs.ia_mtime = *times.add(1);
            newattrs.ia_valid |= ATTR_MTIME_SET;
        }
        // Tell setattr_prepare(), that this is an explicit time update, even
        // if neither ATTR_ATIME_SET nor ATTR_MTIME_SET were used.
        newattrs.ia_valid |= ATTR_TIMES_SET;
    } else {
        newattrs.ia_valid |= ATTR_TOUCH;
    }

    loop {
        inode_lock(inode);
        error = notify_change(mnt_idmap((*path).mnt), (*path).dentry,
                              &mut newattrs, &mut delegated_inode);
        inode_unlock(inode);
        if is_delegated(&delegated_inode) {
            error = break_deleg_wait(&mut delegated_inode);
            if error == 0 {
                continue;
            }
        }
        break;
    }
    mnt_drop_write((*path).mnt);
    error
}

pub unsafe fn do_utimes_path(dfd: libc::c_int, filename: *const libc::c_char,
                             times: *mut timespec64, flags: libc::c_int) -> libc::c_int {
    let mut path: struct_path = core::mem::zeroed();
    let mut lookup_flags = 0;
    let mut error: libc::c_int;
    if flags & !(AT_SYMLINK_NOFOLLOW | AT_EMPTY_PATH) != 0 { return -EINVAL; }
    if flags & AT_SYMLINK_NOFOLLOW == 0 { lookup_flags |= LOOKUP_FOLLOW; }
    let name = filename_uflags_new(filename, flags);
    loop {
        error = filename_lookup(dfd, name, lookup_flags, &mut path, core::ptr::null_mut());
        if error != 0 { return error; }
        error = vfs_utimes(&path, times);
        path_put(&mut path);
        if retry_estale(error, lookup_flags) {
            lookup_flags |= LOOKUP_REVAL;
            continue;
        }
        return error;
    }
}

pub unsafe fn do_utimes_fd(fd: libc::c_int, times: *mut timespec64, flags: libc::c_int) -> libc::c_int {
    if flags != 0 { return -EINVAL; }
    let f = fd_class_new(fd);
    if fd_empty(f) { return -EBADF; }
    vfs_utimes(&(*fd_file(f)).f_path, times)
}

// do_utimes - change times on filename or file descriptor
// @dfd: open file descriptor, -1 or AT_FDCWD
// @filename: path name or NULL
// @times: new times or NULL
// @flags: zero or more flags (only AT_SYMLINK_NOFOLLOW for the moment)
pub unsafe fn do_utimes(dfd: libc::c_int, filename: *const libc::c_char,
                        times: *mut timespec64, flags: libc::c_int) -> libc::c_long {
    if filename.is_null() && dfd != AT_FDCWD {
        do_utimes_fd(dfd, times, flags) as libc::c_long
    } else {
        do_utimes_path(dfd, filename, times, flags) as libc::c_long
    }
}

pub unsafe fn utimensat(dfd: libc::c_int, filename: *const libc::c_char,
                        utimes: *mut kernel_timespec, flags: libc::c_int) -> libc::c_long {
    let mut tstimes: [timespec64; 2] = [core::mem::zeroed(), core::mem::zeroed()];
    if !utimes.is_null() {
        if get_timespec64(&mut tstimes[0], &*utimes.add(0)) != 0 ||
           get_timespec64(&mut tstimes[1], &*utimes.add(1)) != 0 { return -EFAULT as libc::c_long; }
        if tstimes[0].tv_nsec == UTIME_OMIT && tstimes[1].tv_nsec == UTIME_OMIT { return 0; }
    }
    do_utimes(dfd, filename, if utimes.is_null() { core::ptr::null_mut() } else { tstimes.as_mut_ptr() }, flags)
}

// The following compatibility syscall entry points are enabled by the
// external __ARCH_WANT_SYS_UTIME / CONFIG_COMPAT_32BIT_TIME conditions.
// Their declarations are retained here as source-level Rust interfaces.

pub unsafe fn futimesat(dfd: libc::c_int, filename: *const libc::c_char,
                        utimes: *mut kernel_old_timeval) -> libc::c_long {
    let mut times: [kernel_old_timeval; 2] = [core::mem::zeroed(), core::mem::zeroed()];
    if !utimes.is_null() {
        times[0] = *utimes.add(0); times[1] = *utimes.add(1);
        if times[0].tv_usec >= 1_000_000 || times[0].tv_usec < 0 || times[1].tv_usec >= 1_000_000 || times[1].tv_usec < 0 { return -EINVAL as libc::c_long; }
    }
    let mut tv: [timespec64; 2] = [core::mem::zeroed(), core::mem::zeroed()];
    if !utimes.is_null() { tv[0].tv_sec = times[0].tv_sec; tv[0].tv_nsec = 1000 * times[0].tv_usec; tv[1].tv_sec = times[1].tv_sec; tv[1].tv_nsec = 1000 * times[1].tv_usec; }
    do_utimes(dfd, filename, if utimes.is_null() { core::ptr::null_mut() } else { tv.as_mut_ptr() }, 0)
}

pub unsafe fn utimes(filename: *const libc::c_char, utimes: *mut kernel_old_timeval) -> libc::c_long {
    futimesat(AT_FDCWD, filename, utimes)
}

pub unsafe fn utime(filename: *const libc::c_char, times: *mut utimbuf) -> libc::c_long {
    let mut tv: [timespec64; 2] = [core::mem::zeroed(), core::mem::zeroed()];
    if !times.is_null() {
        tv[0].tv_sec = (*times).actime;
        tv[1].tv_sec = (*times).modtime;
    }
    do_utimes(AT_FDCWD, filename, if times.is_null() { core::ptr::null_mut() } else { tv.as_mut_ptr() }, 0)
}

pub unsafe fn utime32(filename: *const libc::c_char, t: *mut old_utimbuf32) -> libc::c_long {
    let mut tv: [timespec64; 2] = [core::mem::zeroed(), core::mem::zeroed()];
    if !t.is_null() {
        tv[0].tv_sec = (*t).actime;
        tv[1].tv_sec = (*t).modtime;
    }
    do_utimes(AT_FDCWD, filename, if t.is_null() { core::ptr::null_mut() } else { tv.as_mut_ptr() }, 0)
}

pub unsafe fn utimensat_time32(dfd: libc::c_uint, filename: *const libc::c_char,
                               t: *mut old_timespec32, flags: libc::c_int) -> libc::c_long {
    let mut tv: [timespec64; 2] = [core::mem::zeroed(), core::mem::zeroed()];
    if !t.is_null() {
        if get_old_timespec32(&mut tv[0], &*t.add(0)) != 0 || get_old_timespec32(&mut tv[1], &*t.add(1)) != 0 { return -EFAULT as libc::c_long; }
        if tv[0].tv_nsec == UTIME_OMIT && tv[1].tv_nsec == UTIME_OMIT { return 0; }
    }
    do_utimes(dfd as libc::c_int, filename, if t.is_null() { core::ptr::null_mut() } else { tv.as_mut_ptr() }, flags)
}

pub unsafe fn futimesat_time32(dfd: libc::c_uint, filename: *const libc::c_char,
                               t: *mut old_timeval32) -> libc::c_long {
    let mut tv: [timespec64; 2] = [core::mem::zeroed(), core::mem::zeroed()];
    if !t.is_null() {
        tv[0].tv_sec = (*t.add(0)).tv_sec; tv[0].tv_nsec = (*t.add(0)).tv_usec;
        tv[1].tv_sec = (*t.add(1)).tv_sec; tv[1].tv_nsec = (*t.add(1)).tv_usec;
        if tv[0].tv_nsec >= 1_000_000 || tv[0].tv_nsec < 0 || tv[1].tv_nsec >= 1_000_000 || tv[1].tv_nsec < 0 { return -EINVAL as libc::c_long; }
        tv[0].tv_nsec *= 1000; tv[1].tv_nsec *= 1000;
    }
    do_utimes(dfd as libc::c_int, filename, if t.is_null() { core::ptr::null_mut() } else { tv.as_mut_ptr() }, 0)
}

pub unsafe fn utimes_time32(filename: *const libc::c_char, t: *mut old_timeval32) -> libc::c_long {
    futimesat_time32(AT_FDCWD as libc::c_uint, filename, t)
}

// External types, constants, and functions referenced above are provided by
// the Linux kernel translation environment.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
