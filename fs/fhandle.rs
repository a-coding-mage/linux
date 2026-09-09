// SPDX-License-Identifier: GPL-2.0
// Kernel headers and local headers from the original source are external dependencies.

unsafe fn do_sys_name_to_handle(
    path: *const struct_path,
    ufh: *mut struct_file_handle,
    mnt_id: *mut core::ffi::c_void,
    unique_mntid: bool,
    fh_flags: i32,
) -> isize {
    let mut retval: isize;
    let f_handle: struct_file_handle;
    let mut handle_dwords: i32;
    let mut handle_bytes: i32;
    let mut handle: *mut struct_file_handle = core::ptr::null_mut();

    if !exportfs_can_encode_fh((*(*path).dentry).d_sb.s_export_op, fh_flags) { return -EOPNOTSUPP as isize; }
    if (fh_flags & EXPORT_FH_CONNECTABLE) != 0 && WARN_ON(((*(*path).dentry).d_flags & DCACHE_DISCONNECTED) != 0) { return -EINVAL as isize; }
    if copy_from_user(&mut f_handle, ufh, core::mem::size_of::<struct_file_handle>()) != 0 { return -EFAULT as isize; }
    if f_handle.handle_bytes > MAX_HANDLE_SZ { return -EINVAL as isize; }
    handle = kzalloc_flex(f_handle.handle_bytes);
    if handle.is_null() { return -ENOMEM as isize; }
    handle_dwords = f_handle.handle_bytes >> 2;
    retval = exportfs_encode_fh((*path).dentry, (*handle).f_handle as *mut struct_fid, &mut handle_dwords, fh_flags) as isize;
    (*handle).handle_type = retval as i32;
    handle_bytes = handle_dwords * core::mem::size_of::<u32>() as i32;
    (*handle).handle_bytes = handle_bytes;
    if handle_bytes > f_handle.handle_bytes || retval == FILEID_INVALID as isize || retval < 0 {
        if retval == FILEID_INVALID as isize || retval == -(ENOSPC as isize) { retval = -EOVERFLOW as isize; }
        handle_bytes = 0;
    } else {
        if (fh_flags & EXPORT_FH_CONNECTABLE) != 0 {
            (*handle).handle_type |= FILEID_IS_CONNECTABLE;
            if d_is_dir((*path).dentry) { (*handle).handle_type |= FILEID_IS_DIR; }
        }
        retval = 0;
    }
    if unique_mntid {
        if put_user(real_mount((*path).mnt).mnt_id_unique, mnt_id as *mut u64) != 0 { retval = -EFAULT as isize; }
    } else if put_user(real_mount((*path).mnt).mnt_id, mnt_id as *mut i32) != 0 { retval = -EFAULT as isize; }
    if retval != -EFAULT as isize && copy_to_user(ufh, handle, struct_size(handle_bytes)) != 0 { retval = -EFAULT as isize; }
    kfree(handle);
    retval
}

unsafe fn sys_name_to_handle_at(dfd: i32, name: *const u8, handle: *mut struct_file_handle, mnt_id: *mut core::ffi::c_void, flag: i32) -> i32 {
    let mut path = core::mem::zeroed::<struct_path>();
    let mut fh_flags = 0;
    if (flag & !(AT_SYMLINK_FOLLOW | AT_EMPTY_PATH | AT_HANDLE_FID | AT_HANDLE_MNT_ID_UNIQUE | AT_HANDLE_CONNECTABLE)) != 0 { return -EINVAL; }
    if (flag & AT_HANDLE_CONNECTABLE) != 0 && (flag & (AT_HANDLE_FID | AT_EMPTY_PATH)) != 0 { return -EINVAL; }
    else if (flag & AT_HANDLE_FID) != 0 { fh_flags |= EXPORT_FH_FID; }
    else if (flag & AT_HANDLE_CONNECTABLE) != 0 { fh_flags |= EXPORT_FH_CONNECTABLE; }
    let lookup_flags = if (flag & AT_SYMLINK_FOLLOW) != 0 { LOOKUP_FOLLOW } else { 0 };
    let filename = filename_lookup_name(name, flag);
    let mut err = filename_lookup(dfd, filename, lookup_flags, &mut path, core::ptr::null_mut());
    if err == 0 { err = do_sys_name_to_handle(&path, handle, mnt_id, (flag & AT_HANDLE_MNT_ID_UNIQUE) != 0, fh_flags) as i32; path_put(&mut path); }
    err
}

unsafe fn get_path_anchor(fd: i32, root: *mut struct_path) -> i32 {
    if fd >= 0 { let f = fdget(fd); if fd_empty(f) { return -EBADF; } *root = fd_file(f).f_path; path_get(root); return 0; }
    if fd == AT_FDCWD { get_fs_pwd(current_fs(), root); return 0; }
    if fd == FD_PIDFS_ROOT { pidfs_get_root(root); return 0; }
    if fd == FD_NSFS_ROOT { nsfs_get_root(root); return 0; }
    -EBADF
}

unsafe fn vfs_dentry_acceptable(context: *mut core::ffi::c_void, dentry: *mut struct_dentry) -> i32 {
    let ctx = &mut *(context as *mut struct_handle_to_path_ctx);
    let user_ns = current_user_ns(); let root = ctx.root.dentry; let idmap = mnt_idmap(ctx.root.mnt);
    if root.is_null() || ctx.flags == 0 { return 1; }
    if !privileged_wrt_inode_uidgid(user_ns, idmap, d_inode(dentry)) { return 0; }
    let mut d = dget(dentry); while d != root && !IS_ROOT(d) { let parent = dget_parent(d); if !privileged_wrt_inode_uidgid(user_ns, idmap, d_inode(parent)) { dput(d); dput(parent); return 0; } dput(d); d = parent; }
    let retval = if (ctx.flags & HANDLE_CHECK_SUBTREE) == 0 || d == root { 1 } else { 0 };
    if (ctx.fh_flags & EXPORT_FH_DIR_ONLY) != 0 { WARN_ON_ONCE(d != root && d != (*root).d_sb.s_root); }
    dput(d); retval
}

unsafe fn do_handle_to_path(handle: *mut struct_file_handle, path: *mut struct_path, ctx: *mut struct_handle_to_path_ctx) -> i32 {
    let mnt = (*ctx).root.mnt;
    let dentry = exportfs_decode_fh_raw(mnt, (*handle).f_handle as *mut struct_fid, (*handle).handle_bytes >> 2, (*handle).handle_type, (*ctx).fh_flags, vfs_dentry_acceptable, ctx as *mut _);
    if dentry.is_null() || IS_ERR(dentry) { return if dentry == ERR_PTR(-ENOMEM) { -ENOMEM } else { -ESTALE }; }
    (*path).dentry = dentry; (*path).mnt = mntget(mnt); 0
}

unsafe fn capable_wrt_mount(mount: *mut struct_mount) -> bool { let ns = READ_ONCE((*mount).mnt_ns); !ns.is_null() && ns_capable((*ns).user_ns, CAP_SYS_ADMIN) }

unsafe fn may_decode_fh(ctx: *mut struct_handle_to_path_ctx, o_flags: u32) -> i32 {
    let root = &mut (*ctx).root;
    if capable(CAP_DAC_READ_SEARCH) { return 0; }
    if (o_flags & O_DIRECTORY as u32) == 0 { return -EPERM; }
    if ns_capable((*root.mnt).mnt_sb.s_user_ns, CAP_SYS_ADMIN) { (*ctx).flags = HANDLE_CHECK_PERMS; }
    else if is_mounted(root.mnt) && capable_wrt_mount(real_mount(root.mnt)) && !has_locked_children(real_mount(root.mnt), root.dentry) { (*ctx).flags = HANDLE_CHECK_PERMS | HANDLE_CHECK_SUBTREE; }
    else { return -EPERM; }
    if !ns_capable(current_user_ns(), CAP_DAC_READ_SEARCH) { return -EPERM; }
    (*ctx).fh_flags = EXPORT_FH_DIR_ONLY; 0
}

unsafe fn handle_to_path(mountdirfd: i32, ufh: *mut struct_file_handle, path: *mut struct_path, o_flags: u32) -> i32 {
    let mut f_handle = core::mem::zeroed::<struct_file_handle>();
    let mut ctx = core::mem::zeroed::<struct_handle_to_path_ctx>();
    if copy_from_user(&mut f_handle, ufh, core::mem::size_of::<struct_file_handle>()) != 0 { return -EFAULT; }
    if f_handle.handle_bytes > MAX_HANDLE_SZ || f_handle.handle_bytes == 0 || f_handle.handle_type < 0 || (FILEID_USER_FLAGS(f_handle.handle_type) & !FILEID_VALID_USER_FLAGS) != 0 { return -EINVAL; }
    let mut retval = get_path_anchor(mountdirfd, &mut ctx.root); if retval != 0 { return retval; }
    let eops = (*ctx.root.mnt).mnt_sb.s_export_op;
    retval = if !eops.is_null() && (*eops).permission.is_some() { ((*eops).permission.unwrap())(&mut ctx, o_flags) } else { may_decode_fh(&mut ctx, o_flags) };
    if retval != 0 { path_put(&mut ctx.root); return retval; }
    let handle = kmalloc_flex(f_handle.handle_bytes); if handle.is_null() { path_put(&mut ctx.root); return -ENOMEM; }
    *handle = f_handle;
    if copy_from_user((*handle).f_handle, (*ufh).f_handle, f_handle.handle_bytes as usize) != 0 { kfree(handle); path_put(&mut ctx.root); return -EFAULT; }
    if (f_handle.handle_type & FILEID_IS_CONNECTABLE) != 0 { ctx.fh_flags |= EXPORT_FH_CONNECTABLE; ctx.flags |= HANDLE_CHECK_SUBTREE; }
    if (f_handle.handle_type & FILEID_IS_DIR) != 0 { ctx.fh_flags |= EXPORT_FH_DIR_ONLY; }
    (*handle).handle_type &= !FILEID_USER_FLAGS_MASK;
    retval = do_handle_to_path(handle, path, &mut ctx); kfree(handle); path_put(&mut ctx.root); retval
}

unsafe fn file_open_handle(path: *mut struct_path, open_flag: i32) -> *mut struct_file {
    let eops = (*(*path).mnt).mnt_sb.s_export_op;
    if !eops.is_null() && (*eops).open.is_some() { return ((*eops).open.unwrap())(path, open_flag); }
    file_open_root(path, core::ptr::null(), open_flag, 0)
}

// The remaining handle decoding/opening path retains the kernel implementation's external helpers and flags.
unsafe fn do_handle_open(mountdirfd: i32, ufh: *mut struct_file_handle, open_flag: i32) -> isize {
    let mut path = core::mem::zeroed::<struct_path>();
    let retval = handle_to_path(mountdirfd, ufh, &mut path, open_flag as u32);
    if retval != 0 { return retval as isize; }
    FD_ADD(open_flag, file_open_handle(&mut path))
}

unsafe fn sys_open_by_handle_at(mountdirfd: i32, handle: *mut struct_file_handle, mut flags: i32) -> isize {
    if force_o_largefile() { flags |= O_LARGEFILE; }
    do_handle_open(mountdirfd, handle, flags)
}

#[cfg(CONFIG_COMPAT)]
unsafe fn compat_sys_open_by_handle_at(mountdirfd: i32, handle: *mut struct_file_handle, flags: i32) -> isize { do_handle_open(mountdirfd, handle, flags) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
