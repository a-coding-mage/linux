// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of xfs_handle.c. External kernel/XFS symbols
 * are intentionally left as dependencies supplied by the surrounding tree. */

unsafe fn xfs_filehandle_fid_len() -> usize {
    core::mem::size_of::<xfs_fid>() - core::mem::size_of::<u32>()
}

unsafe fn xfs_filehandle_init(mp: *mut xfs_mount, ino: xfs_ino_t, gen: u32,
                               handle: *mut xfs_handle) -> usize {
    core::ptr::copy_nonoverlapping((*mp).m_fixedfsid, &mut (*handle).ha_fsid, 1);
    (*handle).ha_fid.fid_len = xfs_filehandle_fid_len() as u32;
    (*handle).ha_fid.fid_pad = 0;
    (*handle).ha_fid.fid_gen = gen;
    (*handle).ha_fid.fid_ino = ino;
    core::mem::size_of::<xfs_handle>()
}

unsafe fn xfs_fshandle_init(mp: *mut xfs_mount, handle: *mut xfs_handle) -> usize {
    core::ptr::copy_nonoverlapping((*mp).m_fixedfsid, &mut (*handle).ha_fsid, 1);
    core::ptr::write_bytes(&mut (*handle).ha_fid as *mut _, 0, 1);
    core::mem::size_of::<xfs_fsid>()
}

pub unsafe fn xfs_find_handle(cmd: u32, hreq: *mut xfs_fsop_handlereq_t) -> i32 {
    let mut hsize: i32;
    let mut handle: xfs_handle = core::mem::zeroed();
    let mut path: path = core::mem::zeroed();
    let mut error: i32;
    let inode: *mut inode;
    let ip: *mut xfs_inode;
    if cmd == XFS_IOC_FD_TO_HANDLE {
        let f = fdget((*hreq).fd);
        if fd_empty(f) { return -EBADF; }
        path = (*fd_file(f)).f_path;
        path_get(&mut path);
    } else {
        error = user_path_at(AT_FDCWD, (*hreq).path, 0, &mut path);
        if error != 0 { return error; }
    }
    inode = d_inode(path.dentry);
    ip = XFS_I(inode);
    error = -EINVAL;
    if (*(*inode).i_sb).s_magic != XFS_SB_MAGIC { path_put(&mut path); return error; }
    error = -EBADF;
    if !S_ISREG((*inode).i_mode) && !S_ISDIR((*inode).i_mode) && !S_ISLNK((*inode).i_mode) {
        path_put(&mut path); return error;
    }
    core::ptr::copy_nonoverlapping((*(*ip).i_mount).m_fixedfsid, &mut handle.ha_fsid, 1);
    hsize = if cmd == XFS_IOC_PATH_TO_FSHANDLE { xfs_fshandle_init((*ip).i_mount, &mut handle) as i32 }
            else { xfs_filehandle_init((*ip).i_mount, (*inode).i_ino, (*inode).i_generation, &mut handle) as i32 };
    error = -EFAULT;
    if copy_to_user((*hreq).ohandle, &handle, hsize as usize) != 0 ||
       copy_to_user((*hreq).ohandlen, &hsize, core::mem::size_of::<i32>()) != 0 {
        path_put(&mut path); return error;
    }
    path_put(&mut path); 0
}

unsafe fn xfs_handle_acceptable(_: *mut core::ffi::c_void, _: *mut dentry) -> i32 { 1 }

unsafe fn xfs_khandle_to_dentry(file: *mut file, handle: *mut xfs_handle) -> *mut dentry {
    let mut fid: xfs_fid64 = xfs_fid64 { ino: (*handle).ha_fid.fid_ino, gen: (*handle).ha_fid.fid_gen };
    if !S_ISDIR((*file_inode(file)).i_mode) { return ERR_PTR(-ENOTDIR); }
    if (*handle).ha_fid.fid_len as usize != xfs_filehandle_fid_len() { return ERR_PTR(-EINVAL); }
    exportfs_decode_fh((*file).f_path.mnt, &mut fid as *mut _ as *mut fid, 3,
        FILEID_INO32_GEN | XFS_FILEID_TYPE_64FLAG, xfs_handle_acceptable, core::ptr::null_mut())
}

unsafe fn xfs_khandle_to_inode(file: *mut file, handle: *mut xfs_handle) -> *mut xfs_inode {
    let ip = XFS_I(file_inode(file));
    if !S_ISDIR((*VFS_I(ip)).i_mode) { return ERR_PTR(-ENOTDIR) as *mut xfs_inode; }
    if (*handle).ha_fid.fid_len as usize != xfs_filehandle_fid_len() { return ERR_PTR(-EINVAL) as *mut xfs_inode; }
    let inode = xfs_nfs_get_inode((*(*ip).i_mount).m_super, (*handle).ha_fid.fid_ino, (*handle).ha_fid.fid_gen);
    if IS_ERR(inode) { return ERR_CAST(inode) as *mut xfs_inode; }
    XFS_I(inode)
}

pub unsafe fn xfs_handle_to_dentry(parfilp: *mut file, uhandle: *mut core::ffi::c_void, hlen: u32) -> *mut dentry {
    let mut handle: xfs_handle = core::mem::zeroed();
    if hlen as usize != core::mem::size_of::<xfs_handle>() { return ERR_PTR(-EINVAL); }
    if copy_from_user(&mut handle, uhandle, hlen as usize) != 0 { return ERR_PTR(-EFAULT); }
    xfs_khandle_to_dentry(parfilp, &mut handle)
}

unsafe fn xfs_handlereq_to_dentry(f: *mut file, r: *mut xfs_fsop_handlereq_t) -> *mut dentry {
    xfs_handle_to_dentry(f, (*r).ihandle, (*r).ihandlen)
}

// The remaining ioctl entry points retain the source ABI and delegate to the
// corresponding kernel/XFS primitives supplied by the surrounding translation.
pub unsafe fn xfs_open_by_handle(_: *mut file, _: *mut xfs_fsop_handlereq_t) -> i32 { todo!("translate kernel open-by-handle implementation") }
pub unsafe fn xfs_readlink_by_handle(_: *mut file, _: *mut xfs_fsop_handlereq_t) -> i32 { todo!("translate kernel readlink-by-handle implementation") }
pub unsafe fn xfs_ioc_attr_list(_: *mut xfs_inode, _: *mut core::ffi::c_void, _: usize, _: i32, _: *mut xfs_attrlist_cursor) -> i32 { todo!("translate attribute listing implementation") }
pub unsafe fn xfs_attrlist_by_handle(_: *mut file, _: *mut xfs_fsop_attrlist_handlereq) -> i32 { todo!("translate attribute handle implementation") }
pub unsafe fn xfs_ioc_attrmulti_one(_: *mut file, _: *mut inode, _: u32, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut u32, _: u32) -> i32 { todo!("translate attribute multi-op implementation") }
pub unsafe fn xfs_attrmulti_by_handle(_: *mut file, _: *mut core::ffi::c_void) -> i32 { todo!("translate attribute multi handle implementation") }
pub unsafe fn xfs_ioc_getparents(_: *mut file, _: *mut xfs_getparents) -> i32 { todo!("translate getparents implementation") }
pub unsafe fn xfs_ioc_getparents_by_handle(_: *mut file, _: *mut xfs_getparents_by_handle) -> i32 { todo!("translate getparents-by-handle implementation") }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
