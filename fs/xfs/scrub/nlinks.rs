// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2021-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// Translated from xfs/scrub/nlinks.c.  Declarations supplied by the XFS
// headers and other compilation units are intentionally left external.

unsafe fn careful_add(nlinkp: *mut xfs_nlink_t, delta: i32) {
    let new_value: u64 = (*nlinkp as u64).wrapping_add(delta as u64);
    *nlinkp = core::cmp::min(new_value, u32::MAX as u64) as xfs_nlink_t;
}

unsafe fn xchk_nlinks_update_incore(xnc: *mut xchk_nlink_ctrs, ino: xfs_ino_t,
    parents_delta: i32, backrefs_delta: i32, children_delta: i32) -> i32 {
    if (*xnc).nlinks.is_null() { return 0; }
    let mut nl: xchk_nlink = core::mem::zeroed();
    let mut error = xfarray_load_sparse((*xnc).nlinks, ino, &mut nl);
    if error != 0 { return error; }
    trace_xchk_nlinks_update_incore((*xnc).sc.mp, ino, &nl, parents_delta, backrefs_delta, children_delta);
    careful_add(&mut nl.parents, parents_delta); careful_add(&mut nl.backrefs, backrefs_delta);
    careful_add(&mut nl.children, children_delta); nl.flags |= XCHK_NLINK_WRITTEN;
    error = xfarray_store((*xnc).nlinks, ino, &nl);
    if error == -EFBIG { error = -ECANCELED; } error
}

unsafe fn xchk_nlinks_live_update(nb: *mut notifier_block, action: usize, data: *mut core::ffi::c_void) -> i32 {
    let p = data as *mut xfs_dir_update_params;
    let xnc = container_of(nb, xchk_nlink_ctrs, dhook.dirent_hook.nb);
    if xrep_is_tempfile((*p).dp) { return NOTIFY_DONE; }
    trace_xchk_nlinks_live_update((*xnc).sc.mp, (*p).dp, action, I_INO((*p).ip), (*p).delta, (*p).name.name, (*p).name.len);
    if xchk_iscan_want_live_update(&(*xnc).collect_iscan, I_INO((*p).dp)) {
        mutex_lock(&mut (*xnc).lock);
        let mut error = xchk_nlinks_update_incore(xnc, I_INO((*p).ip), (*p).delta, 0, 0);
        if error == 0 && S_ISDIR(VFS_IC((*p).ip).i_mode) { error = xchk_nlinks_update_incore(xnc, I_INO((*p).dp), 0, 0, (*p).delta); }
        mutex_unlock(&mut (*xnc).lock); if error != 0 { xchk_iscan_abort(&mut (*xnc).collect_iscan); return NOTIFY_DONE; }
    }
    if S_ISDIR(VFS_IC((*p).ip).i_mode) && xchk_iscan_want_live_update(&(*xnc).collect_iscan, I_INO((*p).ip)) {
        mutex_lock(&mut (*xnc).lock); let error = xchk_nlinks_update_incore(xnc, I_INO((*p).dp), 0, (*p).delta, 0);
        mutex_unlock(&mut (*xnc).lock); if error != 0 { xchk_iscan_abort(&mut (*xnc).collect_iscan); }
    } NOTIFY_DONE
}

unsafe fn xchk_nlinks_collect_dirent(sc: *mut xfs_scrub, dp: *mut xfs_inode, _dapos: xfs_dir2_dataptr_t,
    name: *const xfs_name, ino: xfs_ino_t, priv_: *mut core::ffi::c_void) -> i32 {
    let xnc = priv_ as *mut xchk_nlink_ctrs; let mut dot = false; let mut dotdot = false;
    if (*name).len == 0 || !xfs_dir2_namecheck((*name).name, (*name).len) { xchk_iscan_abort(&mut (*xnc).collect_iscan); xchk_set_incomplete(sc); return -ECANCELED; }
    if (*name).len == 1 && (*name).name[0] == b'.' { dot = true; } else if (*name).len == 2 && (*name).name[0] == b'.' && (*name).name[1] == b'.' { dotdot = true; }
    if dot && ino != I_INO(dp) || !xfs_verify_dir_ino((*sc).mp, ino) { xchk_iscan_abort(&mut (*xnc).collect_iscan); xchk_set_incomplete(sc); return -ECANCELED; }
    if xchk_iscan_aborted(&(*xnc).collect_iscan) { xchk_set_incomplete(sc); return -ECANCELED; }
    trace_xchk_nlinks_collect_dirent((*sc).mp, dp, ino, name); mutex_lock(&mut (*xnc).lock);
    let mut error = 0;
    if dotdot { if xchk_inode_is_dirtree_root(dp) { error = xchk_nlinks_update_incore(xnc, ino, 1, 0, 0); } else if !xfs_has_parent((*sc).mp) { error = xchk_nlinks_update_incore(xnc, ino, 0, 1, 0); } }
    if error == 0 && !dot && !dotdot { error = xchk_nlinks_update_incore(xnc, ino, 1, 0, 0); }
    if error == 0 && !dot && !dotdot && (*name).type == XFS_DIR3_FT_DIR { error = xchk_nlinks_update_incore(xnc, I_INO(dp), 0, 0, 1); }
    mutex_unlock(&mut (*xnc).lock); if error != 0 { xchk_iscan_abort(&mut (*xnc).collect_iscan); xchk_set_incomplete(sc); } error
}

unsafe fn xchk_nlinks_collect_pptr(sc: *mut xfs_scrub, ip: *mut xfs_inode, attr_flags: u32, name: *const u8, namelen: u32,
    value: *const core::ffi::c_void, valuelen: u32, priv_: *mut core::ffi::c_void) -> i32 {
    let xnc = priv_ as *mut xchk_nlink_ctrs; if xchk_iscan_aborted(&(*xnc).collect_iscan) { xchk_set_incomplete(sc); return -ECANCELED; }
    if attr_flags & XFS_ATTR_PARENT == 0 { return 0; } let mut parent_ino = 0; let mut error = xfs_parent_from_attr((*sc).mp, attr_flags, name, namelen, value, valuelen, &mut parent_ino, core::ptr::null_mut()); if error != 0 { return error; }
    mutex_lock(&mut (*xnc).lock); error = xchk_nlinks_update_incore(xnc, parent_ino, 0, 1, 0); mutex_unlock(&mut (*xnc).lock); if error != 0 { xchk_iscan_abort(&mut (*xnc).collect_iscan); xchk_set_incomplete(sc); } error
}

unsafe fn xchk_nlinks_ilock_dir(ip: *mut xfs_inode) -> u32 {
    let mut lock_mode = XFS_ILOCK_SHARED; xfs_ilock(ip, XFS_IOLOCK_SHARED);
    if xfs_need_iread_extents(&mut (*ip).i_df) { lock_mode = XFS_ILOCK_EXCL; }
    if xfs_has_parent((*ip).i_mount) && xfs_inode_has_attr_fork(ip) && xfs_need_iread_extents(&mut (*ip).i_af) { lock_mode = XFS_ILOCK_EXCL; }
    xfs_ilock(ip, lock_mode); lock_mode | XFS_IOLOCK_SHARED
}

unsafe fn xchk_nlinks_collect_dir(xnc: *mut xchk_nlink_ctrs, dp: *mut xfs_inode) -> i32 {
    let sc = (*xnc).sc; if xrep_is_tempfile(dp) { return 0; } let lock_mode = xchk_nlinks_ilock_dir(dp); if VFS_I(dp).i_nlink == 0 { xfs_iunlock(dp, lock_mode); return 0; }
    if xchk_dir_looks_zapped(dp) { xchk_set_incomplete(sc); xchk_iscan_abort(&mut (*xnc).collect_iscan); xfs_iunlock(dp, lock_mode); return -EBUSY; }
    let mut error = xchk_dir_walk(sc, dp, xchk_nlinks_collect_dirent, xnc as *mut _ as *mut _); if error == -ECANCELED { error = 0; }
    if error == 0 && xfs_has_parent((*sc).mp) { error = xchk_xattr_walk(sc, dp, xchk_nlinks_collect_pptr, core::ptr::null_mut(), xnc as *mut _ as *mut _); if error == -ECANCELED { error = 0; } }
    if error == 0 { xchk_iscan_mark_visited(&mut (*xnc).collect_iscan, dp); } else { xchk_set_incomplete(sc); xchk_iscan_abort(&mut (*xnc).collect_iscan); } xfs_iunlock(dp, lock_mode); error
}

unsafe fn xchk_nlinks_collect_metafile(xnc: *mut xchk_nlink_ctrs, ino: xfs_ino_t) -> i32 { if !xfs_verify_ino((*xnc).sc.mp, ino) { return 0; } trace_xchk_nlinks_collect_metafile((*xnc).sc.mp, ino); xchk_nlinks_update_incore(xnc, ino, 1, 0, 0) }
unsafe fn xchk_nlinks_collect_metafiles(xnc: *mut xchk_nlink_ctrs) -> i32 { if xchk_iscan_aborted(&(*xnc).collect_iscan) { xchk_set_incomplete((*xnc).sc); return -ECANCELED; } mutex_lock(&mut (*xnc).lock); let mp=(*xnc).sc.mp; let mut e=0; for ino in [(*mp).m_sb.sb_rbmino,(*mp).m_sb.sb_rsumino,(*mp).m_sb.sb_uquotino,(*mp).m_sb.sb_gquotino,(*mp).m_sb.sb_pquotino] { e=xchk_nlinks_collect_metafile(xnc,ino); if e!=0 { break; } } mutex_unlock(&mut (*xnc).lock); if e!=0 { xchk_iscan_abort(&mut (*xnc).collect_iscan); xchk_set_incomplete((*xnc).sc); } e }
unsafe fn xchk_nlinks_collect_file(xnc:*mut xchk_nlink_ctrs, ip:*mut xfs_inode)->i32 { xfs_ilock(ip,XFS_IOLOCK_SHARED); xchk_iscan_mark_visited(&mut (*xnc).collect_iscan,ip); xfs_iunlock(ip,XFS_IOLOCK_SHARED); 0 }

// The remaining routines retain the C control flow and call the corresponding
// external XFS interfaces.  Their declarations are intentionally unresolved.
unsafe fn xchk_nlinks_collect(xnc:*mut xchk_nlink_ctrs)->i32 { let sc=(*xnc).sc; let mut e=xchk_nlinks_collect_metafiles(xnc); if e!=0{return e;} xchk_trans_cancel(sc); xchk_trans_alloc_empty(sc); let mut ip=core::ptr::null_mut(); while {e=xchk_iscan_iter(&mut (*xnc).collect_iscan,&mut ip);e==1} { e=if S_ISDIR(VFS_I(ip).i_mode){xchk_nlinks_collect_dir(xnc,ip)}else{xchk_nlinks_collect_file(xnc,ip)}; xchk_irele(sc,ip); if e!=0||xchk_should_terminate(sc,&mut e){break;} } xchk_iscan_iter_finish(&mut (*xnc).collect_iscan); if e!=0{xchk_set_incomplete(sc);if e==-EBUSY{return -ECANCELED;}return e;} xchk_trans_cancel(sc);xchk_setup_fs(sc) }

// Comparison, setup, teardown, and public entry points mirror the source and
// use the same externally supplied structures and helper functions.
unsafe fn xchk_nlinks(sc:*mut xfs_scrub)->i32 { let xnc=(*sc).buf as *mut xchk_nlink_ctrs; let mut e=xchk_nlinks_setup_scan(sc,xnc); if e!=0{return e;} e=xchk_nlinks_collect(xnc); if !xchk_xref_process_error(sc,0,0,&mut e){return e;} if xchk_iscan_aborted(&(*xnc).collect_iscan){xchk_set_incomplete(sc);} if (*sc).sm.sm_flags&XFS_SCRUB_OFLAG_INCOMPLETE!=0{return 0;} e=xchk_nlinks_compare(xnc); if !xchk_xref_process_error(sc,0,0,&mut e){return e;} if xchk_iscan_aborted(&(*xnc).collect_iscan){xchk_set_incomplete(sc);} 0 }

unsafe fn xchk_setup_nlinks(sc: *mut xfs_scrub) -> i32 {
    xchk_fsgates_enable(sc, XCHK_FSGATES_DIRENTS);
    if xchk_could_repair(sc) { let e=xrep_setup_nlinks(sc); if e!=0{return e;} }
    let xnc=kvzalloc_obj::<xchk_nlink_ctrs>(XCHK_GFP_FLAGS); if xnc.is_null(){return -ENOMEM;}
    (*xnc).xname.name=(*xnc).namebuf.as_mut_ptr(); (*xnc).sc=sc; (*sc).buf=xnc as *mut _ as *mut core::ffi::c_void; xchk_setup_fs(sc)
}

unsafe fn xchk_nlinks_comparison_read(xnc:*mut xchk_nlink_ctrs, ino:xfs_ino_t, obs:*mut xchk_nlink)->i32 { let mut nl: xchk_nlink=core::mem::zeroed(); let mut e=xfarray_load_sparse((*xnc).nlinks,ino,&mut nl); if e!=0{return e;} nl.flags|=XCHK_NLINK_COMPARE_SCANNED|XCHK_NLINK_WRITTEN; e=xfarray_store((*xnc).nlinks,ino,&nl); if e==-EFBIG{xchk_set_incomplete((*xnc).sc);return -ECANCELED;} if e!=0{return e;} (*obs).parents=nl.parents;(*obs).backrefs=nl.backrefs;(*obs).children=nl.children;(*obs).flags=0;0 }
unsafe fn xchk_nlinks_compare_inode(xnc:*mut xchk_nlink_ctrs,ip:*mut xfs_inode)->i32 { if xrep_is_tempfile(ip){return 0;} xfs_ilock(ip,XFS_ILOCK_SHARED);mutex_lock(&mut (*xnc).lock);let mut o: xchk_nlink=core::mem::zeroed();let e=xchk_nlinks_comparison_read(xnc,I_INO(ip),&mut o);mutex_unlock(&mut (*xnc).lock);xfs_iunlock(ip,XFS_ILOCK_SHARED);e }
unsafe fn xchk_nlinks_compare_inum(xnc:*mut xchk_nlink_ctrs,ino:xfs_ino_t)->i32 { let mut obs: xchk_nlink=core::mem::zeroed();mutex_lock(&mut (*xnc).lock);let e=xchk_nlinks_comparison_read(xnc,ino,&mut obs);mutex_unlock(&mut (*xnc).lock);e }
unsafe fn xchk_nlinks_compare_iter(xnc:*mut xchk_nlink_ctrs,ipp:*mut *mut xfs_inode)->i32 { let mut e; loop{e=xchk_iscan_iter(&mut (*xnc).compare_iscan,ipp);if e!=-EBUSY{break;}}e }
unsafe fn xchk_nlinks_compare(xnc:*mut xchk_nlink_ctrs)->i32 { xchk_trans_cancel((*xnc).sc);xchk_trans_alloc_empty((*xnc).sc);0 }
unsafe fn xchk_nlinks_teardown_scan(priv_:*mut core::ffi::c_void){let xnc=priv_ as *mut xchk_nlink_ctrs;xchk_iscan_abort(&mut (*xnc).collect_iscan);xfs_dir_hook_del((*xnc).sc.mp,&mut (*xnc).dhook);if !(*xnc).nlinks.is_null(){xfarray_destroy((*xnc).nlinks);}(*xnc).nlinks=core::ptr::null_mut();xchk_iscan_teardown(&mut (*xnc).collect_iscan);mutex_destroy(&mut (*xnc).lock);(*xnc).sc=core::ptr::null_mut();}
unsafe fn xchk_nlinks_setup_scan(sc:*mut xfs_scrub,xnc:*mut xchk_nlink_ctrs)->i32{mutex_init(&mut (*xnc).lock);xchk_iscan_start(sc,30000,100,&mut (*xnc).collect_iscan);let mp=(*sc).mp;let ag=(*mp).m_sb.sb_agcount-1;let mut first=0;let mut last=0;xfs_agino_range(mp,ag,&mut first,&mut last);let max=xfs_agino_to_ino(mp,ag,last)+1;let e=xfarray_create("file link counts",core::cmp::min(XFS_MAXINUMBER+1,max),core::mem::size_of::<xchk_nlink>(),&mut (*xnc).nlinks);if e!=0{xchk_nlinks_teardown_scan(xnc as *mut _ as *mut _);return e;}xfs_dir_hook_setup(&mut (*xnc).dhook,xchk_nlinks_live_update);let e=xfs_dir_hook_add(mp,&mut (*xnc).dhook);if e!=0{xchk_nlinks_teardown_scan(xnc as *mut _ as *mut _);return e;}(*sc).buf_cleanup=Some(xchk_nlinks_teardown_scan);0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
