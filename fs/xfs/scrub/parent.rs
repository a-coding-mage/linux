// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// Dependencies supplied by the surrounding XFS implementation are intentionally external.

pub unsafe fn xchk_setup_parent(sc: *mut xfs_scrub) -> i32 {
    let mut error: i32;
    if xchk_could_repair(sc) {
        error = xrep_setup_parent(sc);
        if error != 0 { return error; }
    }
    xchk_setup_inode_contents(sc, 0)
}

pub struct xchk_parent_ctx { pub sc: *mut xfs_scrub, pub nlink: xfs_nlink_t }

unsafe fn xchk_parent_actor(sc: *mut xfs_scrub, _dp: *mut xfs_inode, _dapos: xfs_dir2_dataptr_t, name: *const xfs_name, ino: xfs_ino_t, priv_: *mut core::ffi::c_void) -> i32 {
    let spc = &mut *(priv_ as *mut xchk_parent_ctx);
    let mut error = 0;
    if !xfs_dir2_namecheck((*name).name, (*name).len) { error = -EFSCORRUPTED; }
    if !xchk_fblock_xref_process_error(sc, XFS_DATA_FORK, 0, &mut error) { return error; }
    if I_INO((*sc).ip) == ino { spc.nlink += 1; }
    if xchk_should_terminate(spc.sc, &mut error) { return error; }
    0
}

unsafe fn xchk_parent_ilock_dir(dp: *mut xfs_inode) -> u32 {
    if !xfs_ilock_nowait(dp, XFS_ILOCK_SHARED) { return 0; }
    if !xfs_need_iread_extents(&(*dp).i_df) { return XFS_ILOCK_SHARED; }
    xfs_iunlock(dp, XFS_ILOCK_SHARED);
    if !xfs_ilock_nowait(dp, XFS_ILOCK_EXCL) { return 0; }
    XFS_ILOCK_EXCL
}

unsafe fn xchk_parent_validate(sc: *mut xfs_scrub, parent_ino: xfs_ino_t) -> i32 {
    let mut spc = xchk_parent_ctx { sc, nlink: 0 };
    let mp = (*sc).mp;
    let ino = I_INO((*sc).ip);
    let mut dp: *mut xfs_inode = core::ptr::null_mut();
    let mut error = 0;
    if (*sc).ip == (*mp).m_rootip {
        if ino != (*mp).m_sb.sb_rootino || ino != parent_ino { xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0); }
        return 0;
    }
    if (*sc).ip == (*mp).m_metadirip {
        if ino != (*mp).m_sb.sb_metadirino || ino != parent_ino { xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0); }
        return 0;
    }
    if ino == parent_ino { xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0); return 0; }
    let expected_nlink = if (*VFS_I((*sc).ip)).i_nlink == 0 { 0 } else { 1 };
    error = xchk_iget(sc, parent_ino, &mut dp);
    if error == -EINVAL || error == -ENOENT { error = -EFSCORRUPTED; xchk_fblock_process_error(sc, XFS_DATA_FORK, 0, &mut error); return error; }
    if !xchk_fblock_xref_process_error(sc, XFS_DATA_FORK, 0, &mut error) { return error; }
    if dp == (*sc).ip || xrep_is_tempfile(dp) || !S_ISDIR((*VFS_I(dp)).i_mode) { xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0); xchk_irele(sc, dp); return error; }
    let lock_mode = xchk_parent_ilock_dir(dp);
    if lock_mode == 0 { xchk_iunlock(sc, XFS_ILOCK_EXCL); xchk_ilock(sc, XFS_ILOCK_EXCL); xchk_irele(sc, dp); return -EAGAIN; }
    if xchk_dir_looks_zapped(dp) { xchk_set_incomplete(sc); xfs_iunlock(dp, lock_mode); xchk_irele(sc, dp); return -EBUSY; }
    if xfs_is_metadir_inode(dp) != xfs_is_metadir_inode((*sc).ip) { xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0); xfs_iunlock(dp, lock_mode); xchk_irele(sc, dp); return error; }
    error = xchk_dir_walk(sc, dp, Some(xchk_parent_actor), &mut spc as *mut _ as *mut _);
    if !xchk_fblock_xref_process_error(sc, XFS_DATA_FORK, 0, &mut error) { xfs_iunlock(dp, lock_mode); xchk_irele(sc, dp); return error; }
    if spc.nlink != expected_nlink { xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0); }
    xfs_iunlock(dp, lock_mode); xchk_irele(sc, dp); error
}

#[repr(C)] pub struct xchk_pptr { pub name_cookie: xfblob_cookie, pub pptr_rec: xfs_parent_rec, pub namelen: u8 }
#[repr(C)] pub struct xchk_pptrs {
    pub sc: *mut xfs_scrub, pub pptrs_found: u64, pub parent_ino: xfs_ino_t,
    pub pptr_entries: *mut xfarray, pub pptr_names: *mut xfblob, pub pptr_args: xfs_da_args,
    pub need_revalidate: bool, pub xname: xfs_name, pub namebuf: [i8; MAXNAMELEN],
}

unsafe fn xchk_parent_scan_dotdot(sc: *mut xfs_scrub, _ip: *mut xfs_inode, attr_flags: u32, name: *const u8, namelen: u32, value: *const core::ffi::c_void, valuelen: u32, priv_: *mut core::ffi::c_void) -> i32 {
    let pp = &mut *(priv_ as *mut xchk_pptrs); if attr_flags & XFS_ATTR_PARENT == 0 { return 0; }
    let mut parent_ino = 0; let error = xfs_parent_from_attr((*sc).mp, attr_flags, name, namelen, value, valuelen, &mut parent_ino, core::ptr::null_mut());
    if error != 0 { return error; } if pp.parent_ino == parent_ino { return -ECANCELED; } 0
}

unsafe fn xchk_parent_lock_dir(_sc: *mut xfs_scrub, dp: *mut xfs_inode) -> u32 {
    if !xfs_ilock_nowait(dp, XFS_IOLOCK_SHARED) { return 0; }
    if !xfs_ilock_nowait(dp, XFS_ILOCK_SHARED) { xfs_iunlock(dp, XFS_IOLOCK_SHARED); return 0; }
    if !xfs_need_iread_extents(&(*dp).i_df) { return XFS_IOLOCK_SHARED | XFS_ILOCK_SHARED; }
    xfs_iunlock(dp, XFS_ILOCK_SHARED);
    if !xfs_ilock_nowait(dp, XFS_ILOCK_EXCL) { xfs_iunlock(dp, XFS_IOLOCK_SHARED); return 0; }
    XFS_IOLOCK_SHARED | XFS_ILOCK_EXCL
}

unsafe fn xchk_parent_dirent(pp: *mut xchk_pptrs, xname: *const xfs_name, dp: *mut xfs_inode) -> i32 {
    let sc = (*pp).sc; let mut child_ino = 0; let mut error = xchk_dir_lookup(sc, dp, xname, &mut child_ino);
    if error == -ENOENT { xchk_fblock_xref_set_corrupt(sc, XFS_ATTR_FORK, 0); return 0; }
    if !xchk_fblock_xref_process_error(sc, XFS_ATTR_FORK, 0, &mut error) { return error; }
    if child_ino != I_INO((*sc).ip) { xchk_fblock_xref_set_corrupt(sc, XFS_ATTR_FORK, 0); } 0
}

unsafe fn xchk_parent_iget(pp: *mut xchk_pptrs, pptr: *const xfs_parent_rec, dpp: *mut *mut xfs_inode) -> i32 {
    let sc = (*pp).sc; let parent_ino = be64_to_cpu((*pptr).p_ino); let mut ip = core::ptr::null_mut();
    let mut error = xfs_dir_ino_validate((*sc).mp, parent_ino); if error != 0 { xchk_fblock_set_corrupt(sc, XFS_ATTR_FORK, 0); return -ECANCELED; }
    error = xchk_iget(sc, parent_ino, &mut ip); if error == -EINVAL || error == -ENOENT { xchk_fblock_set_corrupt(sc, XFS_ATTR_FORK, 0); return -ECANCELED; }
    if !xchk_fblock_xref_process_error(sc, XFS_ATTR_FORK, 0, &mut error) { return error; }
    if !S_ISDIR((*VFS_I(ip)).i_mode) || (*VFS_I(ip)).i_generation != be32_to_cpu((*pptr).p_gen) { xchk_fblock_xref_set_corrupt(sc, XFS_ATTR_FORK, 0); xchk_irele(sc, ip); return 0; }
    *dpp = ip; 0
}

unsafe fn xchk_parent_scan_attr(sc: *mut xfs_scrub, _ip: *mut xfs_inode, attr_flags: u32, name: *const u8, namelen: u32, value: *const core::ffi::c_void, valuelen: u32, priv_: *mut core::ffi::c_void) -> i32 {
    let pp = &mut *(priv_ as *mut xchk_pptrs); if attr_flags & XFS_ATTR_PARENT == 0 { return 0; }
    let xname = xfs_name { name, len: namelen }; let pptr_rec = value as *const xfs_parent_rec; let mut parent_ino = 0;
    let mut error = xfs_parent_from_attr((*sc).mp, attr_flags, name, namelen, value, valuelen, &mut parent_ino, core::ptr::null_mut());
    if error != 0 || parent_ino == I_INO((*sc).ip) { xchk_fblock_set_corrupt(sc, XFS_ATTR_FORK, 0); return -ECANCELED; }
    pp.pptrs_found += 1; let mut dp = core::ptr::null_mut(); error = xchk_parent_iget(pp, pptr_rec, &mut dp); if error != 0 || dp.is_null() { return error; }
    let lockmode = xchk_parent_lock_dir(sc, dp); if lockmode == 0 { xchk_irele(sc, dp); return 0; }
    error = xchk_parent_dirent(pp, &xname, dp); xfs_iunlock(dp, lockmode); xchk_irele(sc, dp); error
}

pub unsafe fn xchk_parent(sc: *mut xfs_scrub) -> i32 {
    let mp = (*sc).mp; if xfs_has_parent(mp) { return xchk_parent_pptr(sc); }
    if !S_ISDIR((*VFS_I((*sc).ip)).i_mode) { return -ENOENT; }
    if !xfs_verify_dir_ino(mp, I_INO((*sc).ip)) { xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0); return 0; }
    let mut error = 0; loop {
        if xchk_should_terminate(sc, &mut error) { break; }
        let mut parent_ino = 0; error = xchk_dir_lookup(sc, (*sc).ip, &xfs_name_dotdot, &mut parent_ino);
        if !xchk_fblock_process_error(sc, XFS_DATA_FORK, 0, &mut error) { return error; }
        if !xfs_verify_dir_ino(mp, parent_ino) { xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0); return 0; }
        error = xchk_parent_validate(sc, parent_ino);
        if error != -EAGAIN { break; }
    }
    if error == -EBUSY { return 0; } error
}

pub unsafe fn xchk_pptr_looks_zapped(ip: *mut xfs_inode) -> bool {
    let inode = VFS_I(ip); ASSERT(xfs_has_parent((*ip).i_mount));
    if (*inode).i_nlink == 0 && (inode_state_read_once(inode) & I_LINKABLE) == 0 { return false; }
    if xchk_inode_is_dirtree_root(ip) || xchk_inode_is_sb_rooted(ip) { return false; }
    if !xfs_inode_has_attr_fork(ip) { return true; }
    if xfs_ifork_zapped(ip, XFS_ATTR_FORK) { return true; }
    (*ip).i_af.if_format == XFS_DINODE_FMT_EXTENTS && (*ip).i_af.if_nextents == 0
}

unsafe fn xchk_parent_scan_dotdot_and_check(pp: *mut xchk_pptrs) -> i32 {
    let sc = (*pp).sc; let mut error = xchk_dir_lookup(sc, (*sc).ip, &xfs_name_dotdot, &mut (*pp).parent_ino);
    if !xchk_fblock_process_error(sc, XFS_DATA_FORK, 0, &mut error) { return error; }
    if !xfs_verify_dir_ino((*sc).mp, (*pp).parent_ino) { xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0); return 0; }
    if xchk_inode_is_dirtree_root((*sc).ip) { if I_INO((*sc).ip) != (*pp).parent_ino { xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0); } return 0; }
    if (*VFS_I((*sc).ip)).i_nlink == 0 || ((*sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return 0; }
    error = xchk_xattr_walk(sc, (*sc).ip, Some(xchk_parent_scan_dotdot), core::ptr::null_mut(), pp);
    if error == -ECANCELED { return 0; }
    if error == 0 || error == -EFSCORRUPTED { xchk_fblock_set_corrupt(sc, XFS_ATTR_FORK, 0); return 0; } error
}

unsafe fn xchk_parent_count_pptr(sc: *mut xfs_scrub, _ip: *mut xfs_inode, flags: u32, name: *const u8, len: u32, value: *const core::ffi::c_void, vlen: u32, priv_: *mut core::ffi::c_void) -> i32 {
    if flags & XFS_ATTR_PARENT == 0 { return 0; }
    let pp = &mut *(priv_ as *mut xchk_pptrs); let error = xfs_parent_from_attr((*sc).mp, flags, name, len, value, vlen, core::ptr::null_mut(), core::ptr::null_mut());
    if error == 0 { pp.pptrs_found += 1; } error
}

unsafe fn xchk_parent_count_pptrs(pp: *mut xchk_pptrs) -> i32 {
    let sc = (*pp).sc;
    if (*pp).need_revalidate { (*pp).pptrs_found = 0; let error = xchk_xattr_walk(sc, (*sc).ip, Some(xchk_parent_count_pptr), core::ptr::null_mut(), pp); if error == -EFSCORRUPTED { xchk_fblock_set_corrupt(sc, XFS_ATTR_FORK, 0); return 0; } if error != 0 { return error; } }
    if S_ISDIR((*VFS_I((*sc).ip)).i_mode) { if xchk_inode_is_dirtree_root((*sc).ip) { (*pp).pptrs_found += 1; } if ((*VFS_I((*sc).ip)).i_nlink == 0 && (*pp).pptrs_found > 0) || ((*VFS_I((*sc).ip)).i_nlink > 0 && (*pp).pptrs_found == 0) { xchk_ip_set_corrupt(sc, (*sc).ip); } }
    else { if xfs_has_metadir((*sc).mp) && xchk_inode_is_sb_rooted((*sc).ip) { (*pp).pptrs_found += 1; } if (*VFS_I((*sc).ip)).i_nlink as u64 != (*pp).pptrs_found { xchk_ip_set_corrupt(sc, (*sc).ip); } } 0
}

unsafe fn xchk_parent_pptr(sc: *mut xfs_scrub) -> i32 {
    let mut pp = xchk_pptrs { sc, pptrs_found: 0, parent_ino: 0, pptr_entries: core::ptr::null_mut(), pptr_names: core::ptr::null_mut(), pptr_args: core::mem::zeroed(), need_revalidate: false, xname: core::mem::zeroed(), namebuf: [0; MAXNAMELEN] };
    pp.xname.name = pp.namebuf.as_mut_ptr() as *const u8;
    let mut error = xchk_xattr_walk(sc, (*sc).ip, Some(xchk_parent_scan_attr), core::ptr::null_mut(), &mut pp as *mut _ as *mut _);
    if error == -ECANCELED { error = 0; } if error != 0 { return error; }
    if S_ISDIR((*VFS_I((*sc).ip)).i_mode) { error = xchk_parent_scan_dotdot_and_check(&mut pp); if error != 0 { return error; } }
    xchk_parent_count_pptrs(&mut pp)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
