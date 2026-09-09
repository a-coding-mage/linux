// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright (c) 2018-2024 Oracle. All Rights Reserved. */

// C dependencies supplied by the surrounding XFS translation unit.

#[inline]
unsafe fn xfs_metadir_set_xname(xname: *mut xfs_name, path: *const core::ffi::c_char, ftype: u8) {
    (*xname).name = path as *const u8;
    (*xname).len = strlen(path);
    (*xname).type_ = ftype;
}

#[inline]
unsafe fn xfs_metadir_lookup(tp: *mut xfs_trans, dp: *mut xfs_inode,
    xname: *mut xfs_name, ino: *mut xfs_ino_t) -> i32 {
    let mp = (*dp).i_mount;
    let mut args = xfs_da_args {
        trans: tp, dp, geo: (*mp).m_dir_geo, name: (*xname).name,
        namelen: (*xname).len, hashval: xfs_dir2_hashname(mp, xname),
        whichfork: XFS_DATA_FORK, op_flags: XFS_DA_OP_OKNOENT,
        owner: I_INO(dp), ..core::mem::zeroed()
    };
    if !S_ISDIR((*VFS_I(dp)).i_mode) {
        xfs_fs_mark_sick(mp, XFS_SICK_FS_METADIR);
        return -EFSCORRUPTED;
    }
    if xfs_is_shutdown(mp) { return -EIO; }
    let error = xfs_dir_lookup_args(&mut args);
    if error != 0 { return error; }
    if !xfs_verify_ino(mp, args.inumber) {
        xfs_fs_mark_sick(mp, XFS_SICK_FS_METADIR);
        return -EFSCORRUPTED;
    }
    if (*xname).type_ != XFS_DIR3_FT_UNKNOWN && (*xname).type_ != args.filetype {
        xfs_fs_mark_sick(mp, XFS_SICK_FS_METADIR);
        return -EFSCORRUPTED;
    }
    trace_xfs_metadir_lookup(dp, xname, args.inumber);
    *ino = args.inumber;
    0
}

pub unsafe fn xfs_metadir_load(tp: *mut xfs_trans, dp: *mut xfs_inode,
    path: *const core::ffi::c_char, metafile_type: xfs_metafile_type,
    ipp: *mut *mut xfs_inode) -> i32 {
    let mut xname: xfs_name = core::mem::zeroed();
    let mut ino = 0;
    xfs_metadir_set_xname(&mut xname, path, XFS_DIR3_FT_UNKNOWN);
    xfs_ilock(dp, XFS_ILOCK_EXCL);
    let error = xfs_metadir_lookup(tp, dp, &mut xname, &mut ino);
    xfs_iunlock(dp, XFS_ILOCK_EXCL);
    if error != 0 { return error; }
    xfs_trans_metafile_iget(tp, ino, metafile_type, ipp)
}

#[inline]
unsafe fn xfs_metadir_teardown(upd: *mut xfs_metadir_update, error: i32) {
    trace_xfs_metadir_teardown(upd, error);
    if !(*upd).ppargs.is_null() {
        xfs_parent_finish((*(*upd).dp).i_mount, (*upd).ppargs);
        (*upd).ppargs = core::ptr::null_mut();
    }
    if !(*upd).ip.is_null() {
        if (*upd).ip_locked { xfs_iunlock((*upd).ip, XFS_ILOCK_EXCL); }
        (*upd).ip_locked = false;
    }
    if (*upd).dp_locked { xfs_iunlock((*upd).dp, XFS_ILOCK_EXCL); }
    (*upd).dp_locked = false;
}

unsafe fn xfs_metadir_start_create(upd: *mut xfs_metadir_update) -> i32 {
    let mp = (*(*upd).dp).i_mount;
    let mut error;
    ASSERT(!(*upd).dp.is_null());
    ASSERT((*upd).ip.is_null());
    ASSERT(xfs_has_metadir(mp));
    ASSERT((*upd).metafile_type != XFS_METAFILE_UNKNOWN);
    error = xfs_parent_start(mp, &mut (*upd).ppargs);
    if error != 0 { return error; }
    error = xfs_trans_alloc(mp, &mut (*M_RES(mp)).tr_create,
        xfs_create_space_res(mp, MAXNAMELEN), 0, 0, &mut (*upd).tp);
    if error != 0 { xfs_metadir_teardown(upd, error); return error; }
    xfs_ilock((*upd).dp, XFS_ILOCK_EXCL | XFS_ILOCK_PARENT);
    (*upd).dp_locked = true;
    trace_xfs_metadir_start_create(upd);
    0
}

unsafe fn xfs_metadir_create(upd: *mut xfs_metadir_update, mode: umode_t) -> i32 {
    let mut args: xfs_icreate_args = core::mem::zeroed();
    args.pip = (*upd).dp; args.mode = mode;
    let mut xname: xfs_name = core::mem::zeroed();
    let mut du: xfs_dir_update = core::mem::zeroed();
    du.dp = (*upd).dp; du.name = &mut xname; du.ppargs = (*upd).ppargs;
    let mp = (*(*upd).dp).i_mount;
    let mut ino = 0; let mut resblks; let mut error;
    xfs_assert_ilocked((*upd).dp, XFS_ILOCK_EXCL);
    xfs_metadir_set_xname(&mut xname, (*upd).path, XFS_DIR3_FT_UNKNOWN);
    error = xfs_metadir_lookup((*upd).tp, (*upd).dp, &mut xname, &mut ino);
    if error == 0 { return -EEXIST; }
    if error != -ENOENT { return error; }
    error = xfs_dialloc(&mut (*upd).tp, &mut args, &mut ino);
    if error != 0 { return error; }
    error = xfs_icreate((*upd).tp, ino, &mut args, &mut (*upd).ip);
    if error != 0 { return error; }
    du.ip = (*upd).ip;
    xfs_metafile_set_iflag((*upd).tp, (*upd).ip, (*upd).metafile_type);
    (*upd).ip_locked = true;
    xfs_trans_ijoin((*upd).tp, (*upd).dp, 0);
    if S_ISDIR(args.mode) { resblks = xfs_mkdir_space_res(mp, xname.len); }
    else { resblks = xfs_create_space_res(mp, xname.len); }
    xname.type_ = xfs_mode_to_ftype(args.mode);
    trace_xfs_metadir_try_create(upd);
    error = xfs_dir_create_child((*upd).tp, resblks, &mut du);
    if error != 0 { return error; }
    trace_xfs_metadir_create(upd);
    0
}

#[cfg(not(feature = "kernel"))]
pub unsafe fn xfs_metadir_start_link(upd: *mut xfs_metadir_update) -> i32 {
    let mp = (*(*upd).dp).i_mount;
    let mut resblks = xfs_link_space_res(mp, MAXNAMELEN);
    let mut nospace_error = 0; let error;
    ASSERT(!(*upd).dp.is_null()); ASSERT(!(*upd).ip.is_null()); ASSERT(xfs_has_metadir(mp));
    let mut e = xfs_parent_start(mp, &mut (*upd).ppargs); if e != 0 { return e; }
    e = xfs_trans_alloc_dir((*upd).dp, &mut (*M_RES(mp)).tr_link, (*upd).ip,
        &mut resblks, &mut (*upd).tp, &mut nospace_error);
    if e != 0 { xfs_metadir_teardown(upd, e); return e; }
    if resblks == 0 {
        xfs_trans_cancel((*upd).tp); (*upd).tp = core::ptr::null_mut();
        xfs_iunlock((*upd).dp, XFS_ILOCK_EXCL); xfs_iunlock((*upd).ip, XFS_ILOCK_EXCL);
        xfs_metadir_teardown(upd, nospace_error); return nospace_error;
    }
    (*upd).dp_locked = true; (*upd).ip_locked = true;
    trace_xfs_metadir_start_link(upd); 0
}

#[cfg(not(feature = "kernel"))]
pub unsafe fn xfs_metadir_link(upd: *mut xfs_metadir_update) -> i32 {
    let mut xname: xfs_name = core::mem::zeroed();
    let mut du: xfs_dir_update = core::mem::zeroed();
    du.dp = (*upd).dp; du.name = &mut xname; du.ip = (*upd).ip; du.ppargs = (*upd).ppargs;
    let mp = (*(*upd).dp).i_mount; let mut ino = 0;
    xfs_assert_ilocked((*upd).dp, XFS_ILOCK_EXCL); xfs_assert_ilocked((*upd).ip, XFS_ILOCK_EXCL);
    xfs_metadir_set_xname(&mut xname, (*upd).path, xfs_mode_to_ftype((*VFS_I((*upd).ip)).i_mode));
    let mut error = xfs_metadir_lookup((*upd).tp, (*upd).dp, &mut xname, &mut ino);
    if error == 0 { return -EEXIST; } if error != -ENOENT { return error; }
    error = xfs_dir_add_child((*upd).tp, xfs_link_space_res(mp, xname.len), &mut du);
    if error != 0 { return error; }
    trace_xfs_metadir_link(upd); 0
}

pub unsafe fn xfs_metadir_commit(upd: *mut xfs_metadir_update) -> i32 {
    trace_xfs_metadir_commit(upd);
    let error = xfs_trans_commit((*upd).tp); (*upd).tp = core::ptr::null_mut();
    xfs_metadir_teardown(upd, error); error
}

unsafe fn xfs_metadir_cancel(upd: *mut xfs_metadir_update, error: i32) {
    trace_xfs_metadir_cancel(upd); xfs_trans_cancel((*upd).tp);
    (*upd).tp = core::ptr::null_mut(); xfs_metadir_teardown(upd, error);
}

pub unsafe fn xfs_metadir_create_file(upd: *mut xfs_metadir_update, mode: umode_t,
    create: Option<unsafe extern "C" fn(*mut xfs_metadir_update, *mut core::ffi::c_void) -> i32>,
    priv_: *mut core::ffi::c_void, ipp: *mut *mut xfs_inode) -> i32 {
    if xfs_is_shutdown((*(*upd).dp).i_mount) { return -EIO; }
    let mut error = xfs_metadir_start_create(upd); if error != 0 { return error; }
    error = xfs_metadir_create(upd, mode);
    if error == 0 { if let Some(f) = create { error = f(upd, priv_); } }
    if error != 0 { xfs_metadir_cancel(upd, error); }
    else { error = xfs_metadir_commit(upd); }
    if error != 0 {
        if !(*upd).ip.is_null() { xfs_finish_inode_setup((*upd).ip); xfs_irele((*upd).ip); }
        return error;
    }
    xfs_finish_inode_setup((*upd).ip); *ipp = (*upd).ip; 0
}

pub unsafe fn xfs_metadir_mkdir(dp: *mut xfs_inode, path: *const core::ffi::c_char,
    ipp: *mut *mut xfs_inode) -> i32 {
    let mut upd: xfs_metadir_update = core::mem::zeroed();
    upd.dp = dp; upd.path = path; upd.metafile_type = XFS_METAFILE_DIR;
    xfs_metadir_create_file(&mut upd, S_IFDIR, None, core::ptr::null_mut(), ipp)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
