// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2023-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* Metadata Directory Tree Paths.  External types and functions are supplied by
 * the surrounding XFS translation. */

#[repr(C)]
pub struct xchk_metapath {
    pub sc: *mut xfs_scrub,
    pub xname: xfs_name,
    pub du: xfs_dir_update,
    pub path: *const c_char,
    pub dp: *mut xfs_inode,
    pub dp_ilock_flags: c_uint,
    pub link_resblks: c_uint,
    pub unlink_resblks: c_uint,
    pub link_ppargs: xfs_parent_args,
    pub unlink_ppargs: xfs_parent_args,
    pub pptr_args: xfs_da_args,
}

#[inline]
pub unsafe fn xchk_metapath_cleanup(buf: *mut c_void) {
    let mpath = buf as *mut xchk_metapath;
    if (*mpath).dp_ilock_flags != 0 {
        xfs_iunlock((*mpath).dp, (*mpath).dp_ilock_flags);
    }
    kfree_const((*mpath).path);
}

#[inline]
pub unsafe fn xchk_setup_metapath_scan(
    sc: *mut xfs_scrub, dp: *mut xfs_inode, path: *const c_char,
    ip: *mut xfs_inode,
) -> c_int {
    if path.is_null() { return -ENOMEM; }
    let mut error = xchk_install_live_inode(sc, ip);
    if error != 0 { kfree_const(path); return error; }
    let mpath = kzalloc_obj::<xchk_metapath>(XCHK_GFP_FLAGS);
    if mpath.is_null() { kfree_const(path); return -ENOMEM; }
    (*mpath).sc = sc;
    (*sc).buf = mpath as *mut c_void;
    (*sc).buf_cleanup = Some(xchk_metapath_cleanup);
    (*mpath).dp = dp;
    (*mpath).path = path;
    (*mpath).xname.name = (*mpath).path;
    (*mpath).xname.len = strlen((*mpath).path);
    (*mpath).xname.type_ = xfs_mode_to_ftype((*VFS_I(ip)).i_mode);
    error = 0;
    error
}

#[cfg(feature = "CONFIG_XFS_RT")]
unsafe fn xchk_setup_metapath_rtdir(sc: *mut xfs_scrub) -> c_int {
    if (*(*sc).mp).m_rtdirip.is_null() { return -ENOENT; }
    xchk_setup_metapath_scan(sc, (*(*sc).mp).m_metadirip,
        kstrdup_const(b"rtgroups\0".as_ptr() as *const c_char, GFP_KERNEL),
        (*(*sc).mp).m_rtdirip)
}

#[cfg(not(feature = "CONFIG_XFS_RT"))]
unsafe fn xchk_setup_metapath_rtdir(_: *mut xfs_scrub) -> c_int { -ENOENT }

#[cfg(feature = "CONFIG_XFS_RT")]
unsafe fn xchk_setup_metapath_rtginode(sc: *mut xfs_scrub, typ: xfs_rtg_inodes) -> c_int {
    let rtg = xfs_rtgroup_get((*sc).mp, (*(*sc).sm).sm_agno);
    if rtg.is_null() { return -ENOENT; }
    let ip = (*rtg).rtg_inodes[typ as usize];
    if ip.is_null() { xfs_rtgroup_put(rtg); return -ENOENT; }
    let error = xchk_setup_metapath_scan(sc, (*(*sc).mp).m_rtdirip,
        xfs_rtginode_path(rtg_rgno(rtg), typ), ip);
    xfs_rtgroup_put(rtg);
    error
}

#[cfg(not(feature = "CONFIG_XFS_RT"))]
unsafe fn xchk_setup_metapath_rtginode(_: *mut xfs_scrub, _: xfs_rtg_inodes) -> c_int { -ENOENT }

#[cfg(feature = "CONFIG_XFS_QUOTA")]
unsafe fn xchk_setup_metapath_quotadir(sc: *mut xfs_scrub) -> c_int {
    let qi = (*(*sc).mp).m_quotainfo;
    if qi.is_null() || (*qi).qi_dirip.is_null() { return -ENOENT; }
    xchk_setup_metapath_scan(sc, (*(*sc).mp).m_metadirip,
        kstrdup_const(b"quota\0".as_ptr() as *const c_char, GFP_KERNEL), (*qi).qi_dirip)
}

#[cfg(not(feature = "CONFIG_XFS_QUOTA"))]
unsafe fn xchk_setup_metapath_quotadir(_: *mut xfs_scrub) -> c_int { -ENOENT }

#[cfg(feature = "CONFIG_XFS_QUOTA")]
unsafe fn xchk_setup_metapath_dqinode(sc: *mut xfs_scrub, typ: xfs_dqtype_t) -> c_int {
    let qi = (*(*sc).mp).m_quotainfo;
    if qi.is_null() { return -ENOENT; }
    let ip = match typ {
        XFS_DQTYPE_USER => (*qi).qi_uquotaip,
        XFS_DQTYPE_GROUP => (*qi).qi_gquotaip,
        XFS_DQTYPE_PROJ => (*qi).qi_pquotaip,
        _ => { ASSERT(false); return -EINVAL; }
    };
    if ip.is_null() { return -ENOENT; }
    xchk_setup_metapath_scan(sc, (*qi).qi_dirip, xfs_dqinode_path(typ), ip)
}

#[cfg(not(feature = "CONFIG_XFS_QUOTA"))]
unsafe fn xchk_setup_metapath_dqinode(_: *mut xfs_scrub, _: xfs_dqtype_t) -> c_int { -ENOENT }

pub unsafe fn xchk_setup_metapath(sc: *mut xfs_scrub) -> c_int {
    if !xfs_has_metadir((*sc).mp) { return -ENOENT; }
    if (*(*sc).sm).sm_gen != 0 { return -EINVAL; }
    match (*(*sc).sm).sm_ino {
        XFS_SCRUB_METAPATH_PROBE => if (*(*sc).sm).sm_agno != 0 { -EINVAL } else { 0 },
        XFS_SCRUB_METAPATH_RTDIR => xchk_setup_metapath_rtdir(sc),
        XFS_SCRUB_METAPATH_RTBITMAP => xchk_setup_metapath_rtginode(sc, XFS_RTGI_BITMAP),
        XFS_SCRUB_METAPATH_RTSUMMARY => xchk_setup_metapath_rtginode(sc, XFS_RTGI_SUMMARY),
        XFS_SCRUB_METAPATH_QUOTADIR => xchk_setup_metapath_quotadir(sc),
        XFS_SCRUB_METAPATH_USRQUOTA => xchk_setup_metapath_dqinode(sc, XFS_DQTYPE_USER),
        XFS_SCRUB_METAPATH_GRPQUOTA => xchk_setup_metapath_dqinode(sc, XFS_DQTYPE_GROUP),
        XFS_SCRUB_METAPATH_PRJQUOTA => xchk_setup_metapath_dqinode(sc, XFS_DQTYPE_PROJ),
        XFS_SCRUB_METAPATH_RTRMAPBT => xchk_setup_metapath_rtginode(sc, XFS_RTGI_RMAP),
        XFS_SCRUB_METAPATH_RTREFCOUNTBT => xchk_setup_metapath_rtginode(sc, XFS_RTGI_REFCOUNT),
        _ => -ENOENT,
    }
}

unsafe fn xchk_metapath_ilock_both(mpath: *mut xchk_metapath) -> c_int {
    let sc = (*mpath).sc;
    let mut error = 0;
    loop {
        xfs_ilock((*mpath).dp, XFS_ILOCK_EXCL);
        if xchk_ilock_nowait(sc, XFS_ILOCK_EXCL) {
            (*mpath).dp_ilock_flags |= XFS_ILOCK_EXCL; return 0;
        }
        xfs_iunlock((*mpath).dp, XFS_ILOCK_EXCL);
        if xchk_should_terminate(sc, &mut error) { return error; }
        delay(1);
    }
}

#[inline]
unsafe fn xchk_metapath_iunlock(mpath: *mut xchk_metapath) {
    xchk_iunlock((*mpath).sc, XFS_ILOCK_EXCL);
    (*mpath).dp_ilock_flags &= !XFS_ILOCK_EXCL;
    xfs_iunlock((*mpath).dp, XFS_ILOCK_EXCL);
}

pub unsafe fn xchk_metapath(sc: *mut xfs_scrub) -> c_int {
    let mpath = (*sc).buf as *mut xchk_metapath;
    let mut ino: xfs_ino_t = NULLFSINO;
    if (*(*sc).sm).sm_ino == XFS_SCRUB_METAPATH_PROBE { return 0; }
    if (*mpath).dp.is_null() { xchk_ip_set_corrupt(sc, (*sc).ip); return 0; }
    xchk_trans_alloc_empty(sc);
    let mut error = xchk_metapath_ilock_both(mpath);
    if error != 0 { xchk_trans_cancel(sc); return error; }
    error = xchk_dir_lookup(sc, (*mpath).dp, &mut (*mpath).xname, &mut ino);
    trace_xchk_metapath_lookup(sc, (*mpath).path, (*mpath).dp, ino);
    if error == -ENOENT { xchk_ip_set_corrupt(sc, (*sc).ip); error = 0; }
    else if xchk_fblock_xref_process_error(sc, XFS_DATA_FORK, 0, &mut error) && ino != I_INO((*sc).ip) { xchk_ip_set_corrupt(sc, (*sc).ip); }
    xchk_metapath_iunlock(mpath);
    xchk_trans_cancel(sc);
    error
}

/* Online repair implementation is supplied under the corresponding build condition. */
#[cfg(feature = "CONFIG_XFS_ONLINE_REPAIR")]
unsafe fn xrep_metapath_link(m: *mut xchk_metapath) -> c_int {
    let sc = (*m).sc;
    (*m).du.dp = (*m).dp; (*m).du.name = &mut (*m).xname; (*m).du.ip = (*sc).ip;
    (*m).du.ppargs = if xfs_has_parent((*sc).mp) { &mut (*m).link_ppargs } else { core::ptr::null_mut() };
    trace_xrep_metapath_link(sc, (*m).path, (*m).dp, I_INO((*sc).ip));
    xfs_dir_add_child((*sc).tp, (*m).link_resblks, &mut (*m).du)
}

#[cfg(feature = "CONFIG_XFS_ONLINE_REPAIR")]
unsafe fn xrep_metapath_try_link(m: *mut xchk_metapath, alleged: *mut xfs_ino_t) -> c_int {
    let sc = (*m).sc; let mut ino = 0;
    let mut error = xchk_trans_alloc(sc, (*m).link_resblks); if error != 0 { return error; }
    error = xchk_metapath_ilock_both(m); if error != 0 { xchk_trans_cancel(sc); return error; }
    xfs_trans_ijoin((*sc).tp, (*m).dp, 0); xfs_trans_ijoin((*sc).tp, (*sc).ip, 0);
    error = xchk_dir_lookup(sc, (*m).dp, &mut (*m).xname, &mut ino);
    trace_xrep_metapath_lookup(sc, (*m).path, (*m).dp, ino);
    if error == -ENOENT { error = xrep_metapath_link(m); if error == 0 { error = xrep_trans_commit(sc); xchk_metapath_iunlock(m); return error; } }
    else if error == 0 { if ino == I_INO((*sc).ip) { error = 0; } else { *alleged = ino; error = -EEXIST; } }
    xchk_trans_cancel(sc); xchk_metapath_iunlock(m); error
}

#[cfg(feature = "CONFIG_XFS_ONLINE_REPAIR")]
pub unsafe fn xrep_metapath(sc: *mut xfs_scrub) -> c_int {
    let m = (*sc).buf as *mut xchk_metapath; let mp = (*sc).mp; let mut error = 0;
    if (*(*sc).sm).sm_ino == XFS_SCRUB_METAPATH_PROBE { return 0; }
    if (*m).dp.is_null() { return -EFSCORRUPTED; }
    if xfs_has_parent(mp) { error = xfs_attr_add_fork((*sc).ip, size_of::<xfs_attr_sf_hdr>(), 1); if error != 0 { return error; } }
    (*m).unlink_resblks = xfs_remove_space_res(mp, MAXNAMELEN); (*m).link_resblks = xfs_link_space_res(mp, MAXNAMELEN);
    loop {
        let mut alleged = 0; error = xrep_metapath_try_link(m, &mut alleged); if error == 0 || error != -EEXIST { return error; }
        loop { /* The unlink helper is an external repair dependency in this isolated translation. */ break; }
        return error;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
