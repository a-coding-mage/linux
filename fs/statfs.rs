// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies corresponding to the original C includes are supplied externally.

unsafe fn flags_by_mnt(mnt_flags: i32) -> i32 {
    let mut flags = 0;
    if mnt_flags & MNT_READONLY != 0 { flags |= ST_RDONLY; }
    if mnt_flags & MNT_NOSUID != 0 { flags |= ST_NOSUID; }
    if mnt_flags & MNT_NODEV != 0 { flags |= ST_NODEV; }
    if mnt_flags & MNT_NOEXEC != 0 { flags |= ST_NOEXEC; }
    if mnt_flags & MNT_NOATIME != 0 { flags |= ST_NOATIME; }
    if mnt_flags & MNT_NODIRATIME != 0 { flags |= ST_NODIRATIME; }
    if mnt_flags & MNT_RELATIME != 0 { flags |= ST_RELATIME; }
    if mnt_flags & MNT_NOSYMFOLLOW != 0 { flags |= ST_NOSYMFOLLOW; }
    flags
}

unsafe fn flags_by_sb(s_flags: i32) -> i32 {
    let mut flags = 0;
    if s_flags & SB_SYNCHRONOUS != 0 { flags |= ST_SYNCHRONOUS; }
    if s_flags & SB_MANDLOCK != 0 { flags |= ST_MANDLOCK; }
    if s_flags & SB_RDONLY != 0 { flags |= ST_RDONLY; }
    flags
}

unsafe fn calculate_f_flags(mnt: *mut vfsmount) -> i32 {
    ST_VALID | flags_by_mnt((*mnt).mnt_flags) | flags_by_sb((*(*mnt).mnt_sb).s_flags)
}

unsafe fn statfs_by_dentry(dentry: *mut dentry, buf: *mut kstatfs) -> i32 {
    if (*(*dentry).d_sb).s_op.statfs.is_none() { return -ENOSYS; }
    core::ptr::write_bytes(buf, 0, 1);
    let mut retval = security_sb_statfs(dentry);
    if retval != 0 { return retval; }
    retval = ((*(*(*dentry).d_sb).s_op).statfs.unwrap())(dentry, buf);
    if retval == 0 && (*buf).f_frsize == 0 { (*buf).f_frsize = (*buf).f_bsize; }
    retval
}

#[no_mangle]
pub unsafe extern "C" fn vfs_get_fsid(dentry: *mut dentry, fsid: *mut __kernel_fsid_t) -> i32 {
    let mut st = core::mem::MaybeUninit::<kstatfs>::uninit();
    let error = statfs_by_dentry(dentry, st.as_mut_ptr());
    if error != 0 { return error; }
    *fsid = (*st.as_ptr()).f_fsid;
    0
}

#[no_mangle]
pub unsafe extern "C" fn vfs_statfs(path: *const path, buf: *mut kstatfs) -> i32 {
    let error = statfs_by_dentry((*path).dentry, buf);
    if error == 0 { (*buf).f_flags = calculate_f_flags((*path).mnt); }
    error
}

#[no_mangle]
pub unsafe extern "C" fn user_statfs(pathname: *const core::ffi::c_char, st: *mut kstatfs) -> i32 {
    let mut path = core::mem::MaybeUninit::<path>::uninit();
    let mut lookup_flags = LOOKUP_FOLLOW | LOOKUP_AUTOMOUNT;
    let name = filename_class(pathname);
    loop {
        let error = filename_lookup(AT_FDCWD, name, lookup_flags, path.as_mut_ptr(), core::ptr::null_mut());
        if error == 0 {
            let mut error = vfs_statfs(path.as_mut_ptr(), st);
            path_put(path.as_mut_ptr());
            if retry_estale(error, lookup_flags) {
                lookup_flags |= LOOKUP_REVAL;
                continue;
            }
            return error;
        }
        return error;
    }
}

#[no_mangle]
pub unsafe extern "C" fn fd_statfs(fd: i32, st: *mut kstatfs) -> i32 {
    let f = fd_raw_class(fd);
    if fd_empty(f) { return -EBADF; }
    vfs_statfs(&(*fd_file(f)).f_path, st)
}

unsafe fn do_statfs_native(st: *mut kstatfs, p: *mut statfs) -> i32 {
    let mut buf = core::mem::MaybeUninit::<statfs>::zeroed().assume_init();
    if core::mem::size_of::<statfs>() == core::mem::size_of::<kstatfs>() {
        core::ptr::copy_nonoverlapping(st as *const u8, &mut buf as *mut _ as *mut u8, core::mem::size_of::<kstatfs>());
    } else {
        if core::mem::size_of_val(&buf.f_blocks) == 4 {
            if ((*st).f_blocks | (*st).f_bfree | (*st).f_bavail | (*st).f_bsize | (*st).f_frsize) & 0xffffffff00000000 != 0 { return -EOVERFLOW; }
            if (*st).f_files != -1 && (*st).f_files & 0xffffffff00000000 != 0 { return -EOVERFLOW; }
            if (*st).f_ffree != -1 && (*st).f_ffree & 0xffffffff00000000 != 0 { return -EOVERFLOW; }
        }
        buf.f_type = (*st).f_type; buf.f_bsize = (*st).f_bsize; buf.f_blocks = (*st).f_blocks;
        buf.f_bfree = (*st).f_bfree; buf.f_bavail = (*st).f_bavail; buf.f_files = (*st).f_files;
        buf.f_ffree = (*st).f_ffree; buf.f_fsid = (*st).f_fsid; buf.f_namelen = (*st).f_namelen;
        buf.f_frsize = (*st).f_frsize; buf.f_flags = (*st).f_flags;
    }
    if copy_to_user(p, &buf, core::mem::size_of::<statfs>()) != 0 { return -EFAULT; }
    0
}

unsafe fn do_statfs64(st: *mut kstatfs, p: *mut statfs64) -> i32 {
    let mut buf = core::mem::MaybeUninit::<statfs64>::zeroed().assume_init();
    if core::mem::size_of::<statfs64>() == core::mem::size_of::<kstatfs>() {
        core::ptr::copy_nonoverlapping(st as *const u8, &mut buf as *mut _ as *mut u8, core::mem::size_of::<kstatfs>());
    } else {
        buf.f_type = (*st).f_type; buf.f_bsize = (*st).f_bsize; buf.f_blocks = (*st).f_blocks;
        buf.f_bfree = (*st).f_bfree; buf.f_bavail = (*st).f_bavail; buf.f_files = (*st).f_files;
        buf.f_ffree = (*st).f_ffree; buf.f_fsid = (*st).f_fsid; buf.f_namelen = (*st).f_namelen;
        buf.f_frsize = (*st).f_frsize; buf.f_flags = (*st).f_flags;
    }
    if copy_to_user(p, &buf, core::mem::size_of::<statfs64>()) != 0 { return -EFAULT; }
    0
}

// The syscall macro expansions retain their C ABI and are provided by the kernel integration.
pub unsafe fn sys_statfs(pathname: *const core::ffi::c_char, buf: *mut statfs) -> i32 {
    let mut st = core::mem::MaybeUninit::<kstatfs>::uninit();
    let error = user_statfs(pathname, st.as_mut_ptr());
    if error == 0 { do_statfs_native(st.as_mut_ptr(), buf) } else { error }
}

pub unsafe fn sys_statfs64(pathname: *const core::ffi::c_char, sz: usize, buf: *mut statfs64) -> i32 {
    if sz != core::mem::size_of::<statfs64>() { return -EINVAL; }
    let mut st = core::mem::MaybeUninit::<kstatfs>::uninit();
    let error = user_statfs(pathname, st.as_mut_ptr());
    if error == 0 { do_statfs64(st.as_mut_ptr(), buf) } else { error }
}

pub unsafe fn sys_fstatfs(fd: u32, buf: *mut statfs) -> i32 {
    let mut st = core::mem::MaybeUninit::<kstatfs>::uninit();
    let error = fd_statfs(fd as i32, st.as_mut_ptr());
    if error == 0 { do_statfs_native(st.as_mut_ptr(), buf) } else { error }
}

pub unsafe fn sys_fstatfs64(fd: u32, sz: usize, buf: *mut statfs64) -> i32 {
    if sz != core::mem::size_of::<statfs64>() { return -EINVAL; }
    let mut st = core::mem::MaybeUninit::<kstatfs>::uninit();
    let error = fd_statfs(fd as i32, st.as_mut_ptr());
    if error == 0 { do_statfs64(st.as_mut_ptr(), buf) } else { error }
}

unsafe fn vfs_ustat(dev: dev_t, sbuf: *mut kstatfs) -> i32 {
    let s = user_get_super(dev, false);
    if s.is_null() { return -EINVAL; }
    let err = statfs_by_dentry((*s).s_root, sbuf);
    drop_super(s);
    err
}

pub unsafe fn sys_ustat(dev: u32, ubuf: *mut ustat) -> i32 {
    let mut sbuf = core::mem::MaybeUninit::<kstatfs>::uninit();
    let err = vfs_ustat(new_decode_dev(dev), sbuf.as_mut_ptr());
    if err != 0 { return err; }
    let mut tmp = core::mem::MaybeUninit::<ustat>::zeroed().assume_init();
    tmp.f_tfree = (*sbuf.as_ptr()).f_bfree;
    tmp.f_tinode = (*sbuf.as_ptr()).f_ffree;
    if copy_to_user(ubuf, &tmp, core::mem::size_of::<ustat>()) != 0 { -EFAULT } else { 0 }
}

// CONFIG_COMPAT: these definitions are present when the kernel compatibility ABI is enabled.
#[cfg(feature = "CONFIG_COMPAT")]
unsafe fn put_compat_statfs(ubuf: *mut compat_statfs, kbuf: *mut kstatfs) -> i32 {
    if core::mem::size_of_val(&(*ubuf).f_blocks) == 4 {
        if ((*kbuf).f_blocks | (*kbuf).f_bfree | (*kbuf).f_bavail | (*kbuf).f_bsize | (*kbuf).f_frsize) & 0xffffffff00000000 != 0 { return -EOVERFLOW; }
        if (*kbuf).f_files != 0xffffffffffffffff && (*kbuf).f_files & 0xffffffff00000000 != 0 { return -EOVERFLOW; }
        if (*kbuf).f_ffree != 0xffffffffffffffff && (*kbuf).f_ffree & 0xffffffff00000000 != 0 { return -EOVERFLOW; }
    }
    let mut buf = core::mem::MaybeUninit::<compat_statfs>::zeroed().assume_init();
    buf.f_type = (*kbuf).f_type; buf.f_bsize = (*kbuf).f_bsize; buf.f_blocks = (*kbuf).f_blocks;
    buf.f_bfree = (*kbuf).f_bfree; buf.f_bavail = (*kbuf).f_bavail; buf.f_files = (*kbuf).f_files;
    buf.f_ffree = (*kbuf).f_ffree; buf.f_namelen = (*kbuf).f_namelen;
    buf.f_fsid.val[0] = (*kbuf).f_fsid.val[0]; buf.f_fsid.val[1] = (*kbuf).f_fsid.val[1];
    buf.f_frsize = (*kbuf).f_frsize; buf.f_flags = (*kbuf).f_flags;
    if copy_to_user(ubuf, &buf, core::mem::size_of::<compat_statfs>()) != 0 { -EFAULT } else { 0 }
}

#[cfg(feature = "CONFIG_COMPAT")]
pub unsafe fn compat_sys_statfs(pathname: *const core::ffi::c_char, buf: *mut compat_statfs) -> i32 {
    let mut tmp = core::mem::MaybeUninit::<kstatfs>::uninit();
    let error = user_statfs(pathname, tmp.as_mut_ptr());
    if error == 0 { put_compat_statfs(buf, tmp.as_mut_ptr()) } else { error }
}

#[cfg(feature = "CONFIG_COMPAT")]
pub unsafe fn compat_sys_fstatfs(fd: u32, buf: *mut compat_statfs) -> i32 {
    let mut tmp = core::mem::MaybeUninit::<kstatfs>::uninit();
    let error = fd_statfs(fd as i32, tmp.as_mut_ptr());
    if error == 0 { put_compat_statfs(buf, tmp.as_mut_ptr()) } else { error }
}

#[cfg(feature = "CONFIG_COMPAT")]
unsafe fn put_compat_statfs64(ubuf: *mut compat_statfs64, kbuf: *mut kstatfs) -> i32 {
    if ((*kbuf).f_bsize | (*kbuf).f_frsize) & 0xffffffff00000000 != 0 { return -EOVERFLOW; }
    let mut buf = core::mem::MaybeUninit::<compat_statfs64>::zeroed().assume_init();
    buf.f_type = (*kbuf).f_type; buf.f_bsize = (*kbuf).f_bsize; buf.f_blocks = (*kbuf).f_blocks;
    buf.f_bfree = (*kbuf).f_bfree; buf.f_bavail = (*kbuf).f_bavail; buf.f_files = (*kbuf).f_files;
    buf.f_ffree = (*kbuf).f_ffree; buf.f_namelen = (*kbuf).f_namelen;
    buf.f_fsid.val[0] = (*kbuf).f_fsid.val[0]; buf.f_fsid.val[1] = (*kbuf).f_fsid.val[1];
    buf.f_frsize = (*kbuf).f_frsize; buf.f_flags = (*kbuf).f_flags;
    if copy_to_user(ubuf, &buf, core::mem::size_of::<compat_statfs64>()) != 0 { -EFAULT } else { 0 }
}

#[cfg(feature = "CONFIG_COMPAT")]
pub unsafe fn kcompat_sys_statfs64(pathname: *const core::ffi::c_char, sz: compat_size_t, buf: *mut compat_statfs64) -> i32 {
    if sz as usize != core::mem::size_of::<compat_statfs64>() { return -EINVAL; }
    let mut tmp = core::mem::MaybeUninit::<kstatfs>::uninit();
    let error = user_statfs(pathname, tmp.as_mut_ptr());
    if error == 0 { put_compat_statfs64(buf, tmp.as_mut_ptr()) } else { error }
}

#[cfg(feature = "CONFIG_COMPAT")]
pub unsafe fn compat_sys_statfs64(pathname: *const core::ffi::c_char, sz: compat_size_t, buf: *mut compat_statfs64) -> i32 {
    kcompat_sys_statfs64(pathname, sz, buf)
}

#[cfg(feature = "CONFIG_COMPAT")]
pub unsafe fn kcompat_sys_fstatfs64(fd: u32, sz: compat_size_t, buf: *mut compat_statfs64) -> i32 {
    if sz as usize != core::mem::size_of::<compat_statfs64>() { return -EINVAL; }
    let mut tmp = core::mem::MaybeUninit::<kstatfs>::uninit();
    let error = fd_statfs(fd as i32, tmp.as_mut_ptr());
    if error == 0 { put_compat_statfs64(buf, tmp.as_mut_ptr()) } else { error }
}

#[cfg(feature = "CONFIG_COMPAT")]
pub unsafe fn compat_sys_fstatfs64(fd: u32, sz: compat_size_t, buf: *mut compat_statfs64) -> i32 {
    kcompat_sys_fstatfs64(fd, sz, buf)
}

#[cfg(feature = "CONFIG_COMPAT")]
pub unsafe fn compat_sys_ustat(dev: u32, u: *mut compat_ustat) -> i32 {
    let mut sbuf = core::mem::MaybeUninit::<kstatfs>::uninit();
    let err = vfs_ustat(new_decode_dev(dev), sbuf.as_mut_ptr());
    if err != 0 { return err; }
    let mut tmp = core::mem::MaybeUninit::<compat_ustat>::zeroed().assume_init();
    tmp.f_tfree = (*sbuf.as_ptr()).f_bfree; tmp.f_tinode = (*sbuf.as_ptr()).f_ffree;
    if copy_to_user(u, &tmp, core::mem::size_of::<compat_ustat>()) != 0 { -EFAULT } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
