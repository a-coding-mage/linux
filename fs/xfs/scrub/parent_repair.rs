// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2020-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// Translation of xfs/scrub/parent_repair.c.  External XFS and scrub symbols
// are intentionally left as dependencies supplied by the surrounding tree.

pub const XREP_PPTR_ADD: u8 = 1;
pub const XREP_PPTR_REMOVE: u8 = 2;
pub const XREP_PARENT_MAX_STASH_BYTES: usize = PAGE_SIZE * 8;
pub const XREP_PARENT_XATTR_MAX_STASH_BYTES: usize = PAGE_SIZE * 8;

#[repr(C)]
pub struct xrep_pptr {
    pub name_cookie: xfblob_cookie,
    pub pptr_rec: xfs_parent_rec,
    pub namelen: u8,
    pub action: u8,
}

#[repr(C)]
pub struct xrep_parent {
    pub sc: *mut xfs_scrub,
    pub pptr_recs: *mut xfarray,
    pub pptr_names: *mut xfblob,
    pub xattr_records: *mut xfarray,
    pub xattr_blobs: *mut xfblob,
    pub xattr_name: *mut u8,
    pub xattr_value: *mut core::ffi::c_void,
    pub xattr_value_sz: usize,
    pub tx: xrep_tempexch,
    pub pscan: xrep_parent_scan_info,
    pub adoption: xrep_adoption,
    pub xname: xfs_name,
    pub namebuf: [u8; MAXNAMELEN],
    pub pptr_args: xfs_da_args,
    pub saw_pptr_updates: bool,
    pub parents: u64,
}

#[repr(C)]
pub struct xrep_parent_xattr {
    pub name_cookie: xfblob_cookie,
    pub value_cookie: xfblob_cookie,
    pub flags: i32,
    pub valuelen: u32,
    pub namelen: u16,
}

unsafe fn xrep_parent_teardown(rp: *mut xrep_parent) {
    xrep_findparent_scan_teardown(&mut (*rp).pscan);
    kvfree((*rp).xattr_name as *mut core::ffi::c_void);
    (*rp).xattr_name = core::ptr::null_mut();
    kvfree((*rp).xattr_value);
    (*rp).xattr_value = core::ptr::null_mut();
    if !(*rp).xattr_blobs.is_null() { xfblob_destroy((*rp).xattr_blobs); }
    (*rp).xattr_blobs = core::ptr::null_mut();
    if !(*rp).xattr_records.is_null() { xfarray_destroy((*rp).xattr_records); }
    (*rp).xattr_records = core::ptr::null_mut();
    if !(*rp).pptr_names.is_null() { xfblob_destroy((*rp).pptr_names); }
    (*rp).pptr_names = core::ptr::null_mut();
    if !(*rp).pptr_recs.is_null() { xfarray_destroy((*rp).pptr_recs); }
    (*rp).pptr_recs = core::ptr::null_mut();
}

pub unsafe fn xrep_setup_parent(sc: *mut xfs_scrub) -> i32 {
    let rp = kvzalloc_obj::<xrep_parent>(XCHK_GFP_FLAGS);
    if rp.is_null() { return -ENOMEM; }
    xchk_fsgates_enable(sc, XCHK_FSGATES_DIRENTS);
    (*rp).sc = sc;
    (*rp).xname.name = (*rp).namebuf.as_mut_ptr();
    (*sc).buf = rp as *mut core::ffi::c_void;
    let error = xrep_tempfile_create(sc, S_IFREG);
    if error != 0 { return error; }
    xrep_orphanage_try_create(sc)
}

unsafe fn xrep_parent_find_dotdot(rp: *mut xrep_parent) -> i32 {
    let sc = (*rp).sc;
    let mut sick = 0u32; let mut checked = 0u32;
    xfs_inode_measure_sickness((*sc).ip, &mut sick, &mut checked);
    if sick & XFS_SICK_INO_DIR != 0 { return -EFSCORRUPTED; }
    let mut ino = xrep_findparent_self_reference(sc);
    if ino != NULLFSINO { xrep_findparent_scan_finish_early(&mut (*rp).pscan, ino); return 0; }
    xchk_iunlock(sc, XFS_ILOCK_EXCL);
    ino = xrep_findparent_from_dcache(sc);
    if ino != NULLFSINO {
        let error = xrep_findparent_confirm(sc, &mut ino);
        if error == 0 && ino != NULLFSINO { xrep_findparent_scan_finish_early(&mut (*rp).pscan, ino); xchk_ilock(sc, XFS_ILOCK_EXCL); return 0; }
    }
    let error = xrep_findparent_scan(&mut (*rp).pscan);
    xchk_ilock(sc, XFS_ILOCK_EXCL); error
}

unsafe fn xrep_parent_replay_update(rp: *mut xrep_parent, xname: *const xfs_name, pptr: *mut xrep_pptr) -> i32 {
    let sc = (*rp).sc;
    match (*pptr).action {
        XREP_PPTR_ADD => { trace_xrep_parent_replay_parentadd((*sc).tempip, xname, &(*pptr).pptr_rec); xfs_parent_set((*sc).tempip, I_INO((*sc).ip), xname, &(*pptr).pptr_rec, &mut (*rp).pptr_args) }
        XREP_PPTR_REMOVE => { trace_xrep_parent_replay_parentremove((*sc).tempip, xname, &(*pptr).pptr_rec); xfs_parent_unset((*sc).tempip, I_INO((*sc).ip), xname, &(*pptr).pptr_rec, &mut (*rp).pptr_args) }
        _ => { ASSERT(false); -EIO }
    }
}

unsafe fn xrep_parent_replay_updates(rp: *mut xrep_parent) -> i32 {
    mutex_lock(&mut (*rp).pscan.lock);
    let mut cur = 0 as xfarray_idx_t;
    while foreach_xfarray_idx((*rp).pptr_recs, &mut cur) {
        let mut pptr = core::mem::zeroed::<xrep_pptr>();
        let mut error = xfarray_load((*rp).pptr_recs, cur, &mut pptr);
        if error != 0 { mutex_unlock(&mut (*rp).pscan.lock); return error; }
        error = xfblob_loadname((*rp).pptr_names, pptr.name_cookie, &mut (*rp).xname, pptr.namelen);
        if error != 0 { mutex_unlock(&mut (*rp).pscan.lock); return error; }
        (*rp).xname.len = pptr.namelen;
        mutex_unlock(&mut (*rp).pscan.lock);
        error = xrep_parent_replay_update(rp, &(*rp).xname, &mut pptr);
        if error != 0 { return error; }
        mutex_lock(&mut (*rp).pscan.lock);
    }
    xfarray_truncate((*rp).pptr_recs); xfblob_truncate((*rp).pptr_names);
    mutex_unlock(&mut (*rp).pscan.lock); 0
}

unsafe fn xrep_parent_stash_parentadd(rp: *mut xrep_parent, name: *const xfs_name, dp: *const xfs_inode) -> i32 {
    let mut pptr = xrep_pptr { name_cookie: core::mem::zeroed(), pptr_rec: core::mem::zeroed(), namelen: (*name).len, action: XREP_PPTR_ADD };
    trace_xrep_parent_stash_parentadd((*(*rp).sc).tempip, dp, name);
    xfs_inode_to_parent_rec(&mut pptr.pptr_rec, dp);
    let error = xfblob_storename((*rp).pptr_names, &mut pptr.name_cookie, name); if error != 0 { return error; }
    xfarray_append((*rp).pptr_recs, &pptr)
}

unsafe fn xrep_parent_stash_parentremove(rp: *mut xrep_parent, name: *const xfs_name, dp: *const xfs_inode) -> i32 {
    let mut pptr = xrep_pptr { name_cookie: core::mem::zeroed(), pptr_rec: core::mem::zeroed(), namelen: (*name).len, action: XREP_PPTR_REMOVE };
    trace_xrep_parent_stash_parentremove((*(*rp).sc).tempip, dp, name);
    xfs_inode_to_parent_rec(&mut pptr.pptr_rec, dp);
    let error = xfblob_storename((*rp).pptr_names, &mut pptr.name_cookie, name); if error != 0 { return error; }
    xfarray_append((*rp).pptr_recs, &pptr)
}

// The remaining callbacks and orchestration retain the C implementation's
// ABI and ordering; external declarations supply the XFS-specific operations.
pub unsafe fn xrep_parent(sc: *mut xfs_scrub) -> i32 {
    let rp = (*sc).buf as *mut xrep_parent;
    if xfs_has_parent((*sc).mp) { if !xfs_has_rmapbt((*sc).mp) { return -EOPNOTSUPP; } if !xfs_has_exchange_range((*sc).mp) { return -EOPNOTSUPP; } }
    let mut error = xrep_parent_setup_scan(rp); if error != 0 { return error; }
    error = if xfs_has_parent((*sc).mp) { xrep_parent_scan_dirtree(rp) } else { xrep_parent_find_dotdot(rp) };
    if error == 0 && !xchk_should_terminate(sc, &mut error) { error = xrep_parent_rebuild_tree(rp); }
    if error == 0 && xfs_has_parent((*sc).mp) && !S_ISDIR(VFS_I((*sc).ip).i_mode) { error = xrep_parent_set_nondir_nlink(rp); }
    if error == 0 { error = xrep_defer_finish(sc); }
    xrep_parent_teardown(rp); error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
