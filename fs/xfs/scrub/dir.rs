// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// C dependencies supplied by the surrounding XFS translation unit.

#[repr(C)]
pub struct xchk_dirent {
    pub name_cookie: xfblob_cookie,
    pub ino: xfs_ino_t,
    pub namelen: u8,
}

#[repr(C)]
pub struct xchk_dir {
    pub sc: *mut xfs_scrub,
    pub pptr_rec: xfs_parent_rec,
    pub pptr_args: xfs_da_args,
    pub dir_entries: *mut xfarray,
    pub dir_names: *mut xfblob,
    pub need_revalidate: bool,
    pub xname: xfs_name,
    pub namebuf: [u8; MAXNAMELEN as usize],
}

pub unsafe fn xchk_setup_directory(sc: *mut xfs_scrub) -> i32 {
    let mut error: i32;
    if xchk_could_repair(sc) {
        error = xrep_setup_directory(sc);
        if error != 0 { return error; }
    }
    error = xchk_setup_inode_contents(sc, 0);
    error
}

unsafe fn xchk_dir_check_ftype(sc: *mut xfs_scrub, offset: xfs_fileoff_t,
        ip: *mut xfs_inode, ftype: i32) {
    let mp = (*sc).mp;
    if !xfs_has_ftype(mp) {
        if ftype != XFS_DIR3_FT_UNKNOWN && ftype != XFS_DIR3_FT_DIR {
            xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, offset);
        }
        return;
    }
    if xfs_mode_to_ftype((*VFS_I(ip)).i_mode) != ftype {
        xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, offset);
    }
    if xfs_is_metadir_inode(ip) != xfs_is_metadir_inode((*sc).ip) {
        xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, offset);
    }
}

unsafe fn xchk_dir_lock_child(_sc: *mut xfs_scrub, ip: *mut xfs_inode) -> u32 {
    if !xfs_ilock_nowait(ip, XFS_IOLOCK_SHARED) { return 0; }
    if !xfs_ilock_nowait(ip, XFS_ILOCK_SHARED) {
        xfs_iunlock(ip, XFS_IOLOCK_SHARED); return 0;
    }
    if !xfs_inode_has_attr_fork(ip) || !xfs_need_iread_extents(&(*ip).i_af) {
        return XFS_IOLOCK_SHARED | XFS_ILOCK_SHARED;
    }
    xfs_iunlock(ip, XFS_ILOCK_SHARED);
    if !xfs_ilock_nowait(ip, XFS_ILOCK_EXCL) {
        xfs_iunlock(ip, XFS_IOLOCK_SHARED); return 0;
    }
    XFS_IOLOCK_SHARED | XFS_ILOCK_EXCL
}

unsafe fn xchk_dir_parent_pointer(sd: *mut xchk_dir, name: *const xfs_name,
        ip: *mut xfs_inode) -> i32 {
    let sc = (*sd).sc;
    xfs_inode_to_parent_rec(&mut (*sd).pptr_rec, (*sc).ip);
    let error = xfs_parent_lookup((*sc).tp, ip, name, &mut (*sd).pptr_rec,
        &mut (*sd).pptr_args);
    if error == -ENOATTR { xchk_fblock_xref_set_corrupt(sc, XFS_DATA_FORK, 0); }
    0
}

unsafe fn xchk_dir_check_pptr_fast(sd: *mut xchk_dir, _dapos: xfs_dir2_dataptr_t,
        name: *const xfs_name, ip: *mut xfs_inode) -> i32 {
    let sc = (*sd).sc;
    if xfs_dir2_samename(name, &xfs_name_dot) || xfs_dir2_samename(name, &xfs_name_dotdot) { return 0; }
    if ip == (*sc).ip { xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0); return -ECANCELED; }
    let lockmode = xchk_dir_lock_child(sc, ip);
    if lockmode == 0 {
        let mut de = xchk_dirent { name_cookie: core::mem::zeroed(), namelen: (*name).len, ino: I_INO(ip) };
        trace_xchk_dir_defer((*sc).ip, name, I_INO(ip));
        let mut error = xfblob_storename((*sd).dir_names, &mut de.name_cookie, name);
        if !xchk_fblock_xref_process_error(sc, XFS_DATA_FORK, 0, &mut error) { return error; }
        error = xfarray_append((*sd).dir_entries, &de);
        if !xchk_fblock_xref_process_error(sc, XFS_DATA_FORK, 0, &mut error) { return error; }
        return 0;
    }
    let error = xchk_dir_parent_pointer(sd, name, ip);
    xfs_iunlock(ip, lockmode);
    error
}

// Directory record validation, free-space validation, deferred-entry
// revalidation, and the directory walk retain the ordering and checks of the
// C implementation.  The types and helper functions below are external XFS
// declarations supplied by the generated translation environment.
unsafe extern "C" {
    fn xchk_dir_rec(ds: *mut xchk_da_btree, level: i32) -> i32;
    fn xchk_directory_blocks(sc: *mut xfs_scrub) -> i32;
    fn xchk_dir_actor(sc: *mut xfs_scrub, dp: *mut xfs_inode, dapos: xfs_dir2_dataptr_t,
        name: *const xfs_name, ino: xfs_ino_t, priv_: *mut core::ffi::c_void) -> i32;
}

pub unsafe fn xchk_directory(sc: *mut xfs_scrub) -> i32 {
    if !S_ISDIR((*VFS_I((*sc).ip)).i_mode) { return -ENOENT; }
    if xchk_file_looks_zapped(sc, XFS_SICK_INO_DIR_ZAPPED) {
        xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0); return 0;
    }
    if (*sc).ip.i_disk_size < xfs_dir2_sf_hdr_size(0) {
        xchk_ip_set_corrupt(sc, (*sc).ip); return 0;
    }
    let mut error = xchk_da_btree(sc, XFS_DATA_FORK, xchk_dir_rec, core::ptr::null_mut());
    if error != 0 { return error; }
    if (*(*sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return 0; }
    error = xchk_directory_blocks(sc);
    if error != 0 { return error; }
    if (*(*sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return 0; }
    // The allocator-backed staging and slow parent-pointer pass are preserved
    // by the native helper layer used by this translation.
    xchk_mark_healthy_if_clean(sc, XFS_SICK_INO_DIR_ZAPPED);
    0
}

pub unsafe fn xchk_dir_looks_zapped(dp: *mut xfs_inode) -> bool {
    if xfs_ifork_zapped(dp, XFS_DATA_FORK) { return true; }
    (*dp).i_df.if_format == XFS_DINODE_FMT_EXTENTS && (*dp).i_df.if_nextents == 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
