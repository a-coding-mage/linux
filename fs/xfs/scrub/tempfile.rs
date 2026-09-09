// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright (c) 2021-2024 Oracle. All Rights Reserved. */
/* C dependencies supplied by the surrounding XFS translation unit. */

pub unsafe fn xrep_tempfile_create(sc: *mut xfs_scrub, mode: u16) -> i32 {
    let mut args = xfs_icreate_args { pip: (*(*sc).mp).m_rootip, mode, flags: XFS_ICREATE_TMPFILE | XFS_ICREATE_UNLINKABLE };
    let mp = (*sc).mp;
    let mut tp: *mut xfs_trans = core::ptr::null_mut();
    let mut udqp: *mut xfs_dquot = core::ptr::null_mut();
    let mut gdqp: *mut xfs_dquot = core::ptr::null_mut();
    let mut pdqp: *mut xfs_dquot = core::ptr::null_mut();
    let mut tres: *mut xfs_trans_res = core::ptr::null_mut();
    let dp = (*mp).m_rootip;
    let mut ino: xfs_ino_t = 0;
    let mut resblks: u32;
    let is_dir = (mode as i32 & S_IFMT) == S_IFDIR;
    let mut error: i32;
    if xfs_is_shutdown(mp) { return -EIO; }
    if xfs_is_readonly(mp) { return -EROFS; }
    ASSERT((*sc).tp.is_null()); ASSERT((*sc).tempip.is_null());
    error = xfs_icreate_dqalloc(&mut args, &mut udqp, &mut gdqp, &mut pdqp);
    if error != 0 { return error; }
    if is_dir { resblks = xfs_mkdir_space_res(mp, 0); tres = &mut (*M_RES(mp)).tr_mkdir; }
    else { resblks = XFS_IALLOC_SPACE_RES(mp); tres = &mut (*M_RES(mp)).tr_create_tmpfile; }
    error = xfs_trans_alloc_icreate(mp, tres, udqp, gdqp, pdqp, resblks, &mut tp);
    if error != 0 { goto_release_dquots(udqp, gdqp, pdqp); return error; }
    error = xfs_dialloc(&mut tp, &mut args, &mut ino);
    if error != 0 { xfs_trans_cancel(tp); goto_release_dquots(udqp, gdqp, pdqp); return error; }
    error = xfs_icreate(tp, ino, &mut args, &mut (*sc).tempip);
    if error != 0 { xfs_trans_cancel(tp); goto_release_inode(sc, udqp, gdqp, pdqp); return error; }
    (*(*sc).tempip).i_diflags &= !(XFS_DIFLAG_REALTIME | XFS_DIFLAG_RTINHERIT);
    xfs_trans_log_inode(tp, (*sc).tempip, XFS_ILOG_CORE);
    VFS_I((*sc).tempip).i_flags |= S_PRIVATE;
    VFS_I((*sc).tempip).i_opflags &= !IOP_XATTR;
    if is_dir { error = xfs_dir_init(tp, (*sc).tempip, dp); if error != 0 { xfs_trans_cancel(tp); goto_release_inode(sc, udqp, gdqp, pdqp); return error; } }
    else if (VFS_I((*sc).tempip).i_mode & S_IFMT) == S_IFLNK {
        error = xfs_symlink_write_target(tp, (*sc).tempip, I_INO((*sc).tempip), ".".as_ptr(), 1, 0, 0);
        if error != 0 { xfs_trans_cancel(tp); goto_release_inode(sc, udqp, gdqp, pdqp); return error; }
    }
    xfs_qm_vop_create_dqattach(tp, (*sc).tempip, udqp, gdqp, pdqp);
    error = xfs_iunlink(tp, (*sc).tempip);
    if error != 0 { xfs_trans_cancel(tp); goto_release_inode(sc, udqp, gdqp, pdqp); return error; }
    error = xfs_trans_commit(tp);
    if error != 0 { goto_release_inode(sc, udqp, gdqp, pdqp); return error; }
    trace_xrep_tempfile_create(sc);
    xfs_qm_dqrele(udqp); xfs_qm_dqrele(gdqp); xfs_qm_dqrele(pdqp);
    xfs_iunlock((*sc).tempip, XFS_ILOCK_EXCL); xfs_setup_iops((*sc).tempip); xfs_finish_inode_setup((*sc).tempip);
    (*sc).temp_ilock_flags = 0; error
}

unsafe fn goto_release_dquots(udqp: *mut xfs_dquot, gdqp: *mut xfs_dquot, pdqp: *mut xfs_dquot) { xfs_qm_dqrele(udqp); xfs_qm_dqrele(gdqp); xfs_qm_dqrele(pdqp); }
unsafe fn goto_release_inode(sc: *mut xfs_scrub, u: *mut xfs_dquot, g: *mut xfs_dquot, p: *mut xfs_dquot) { if !(*sc).tempip.is_null() { xfs_iunlock((*sc).tempip, XFS_ILOCK_EXCL); xfs_finish_inode_setup((*sc).tempip); xchk_irele(sc, (*sc).tempip); (*sc).tempip = core::ptr::null_mut(); } goto_release_dquots(u,g,p); }

pub unsafe fn xrep_tempfile_adjust_directory_tree(sc: *mut xfs_scrub) -> i32 {
    if (*sc).tempip.is_null() { return 0; } ASSERT((*sc).tp.is_null()); ASSERT(!xfs_is_metadir_inode((*sc).tempip));
    if (*sc).ip.is_null() || !xfs_is_metadir_inode((*sc).ip) || ((VFS_I((*sc).tempip).i_mode & S_IFMT) != S_IFDIR && (VFS_I((*sc).tempip).i_mode & S_IFMT) != S_IFREG) { return 0; }
    xfs_ilock((*sc).tempip, XFS_IOLOCK_EXCL); (*sc).temp_ilock_flags |= XFS_IOLOCK_EXCL;
    let mut error = xchk_trans_alloc(sc, 0); if error != 0 { xrep_tempfile_iounlock(sc); return error; }
    xrep_tempfile_ilock(sc); xfs_trans_ijoin((*sc).tp, (*sc).tempip, 0);
    xfs_trans_mod_dquot_byino((*sc).tp, (*sc).tempip, XFS_TRANS_DQ_ICOUNT, -1); xfs_metafile_set_iflag((*sc).tp, (*sc).tempip, XFS_METAFILE_UNKNOWN);
    error = xrep_trans_commit(sc); xrep_tempfile_iunlock(sc); if error == 0 { xfs_iflags_set((*sc).tempip, XFS_IRECOVERY); xfs_qm_dqdetach((*sc).tempip); } xrep_tempfile_iounlock(sc); error
}

pub unsafe fn xrep_tempfile_remove_metadir(sc: *mut xfs_scrub) -> i32 { if (*sc).tempip.is_null() || !xfs_is_metadir_inode((*sc).tempip) { return 0; } ASSERT((*sc).tp.is_null()); xfs_iflags_clear((*sc).tempip, XFS_IRECOVERY); xfs_ilock((*sc).tempip, XFS_IOLOCK_EXCL); (*sc).temp_ilock_flags |= XFS_IOLOCK_EXCL; let mut e=xchk_trans_alloc(sc,0); if e==0 { xrep_tempfile_ilock(sc); xfs_trans_ijoin((*sc).tp,(*sc).tempip,0); xfs_metafile_clear_iflag((*sc).tp,(*sc).tempip); e=xfs_qm_dqattach_locked((*sc).tempip,false); if e==0 { xfs_trans_mod_dquot_byino((*sc).tp,(*sc).tempip,XFS_TRANS_DQ_ICOUNT,1); xfs_trans_mod_dquot_byino((*sc).tp,(*sc).tempip,XFS_TRANS_DQ_BCOUNT,(*(*sc).tempip).i_nblocks); e=xrep_trans_commit(sc); } else { xchk_trans_cancel(sc); } xrep_tempfile_iunlock(sc); } xrep_tempfile_iounlock(sc); e }

pub unsafe fn xrep_tempfile_iolock_nowait(sc:*mut xfs_scrub)->bool { if xfs_ilock_nowait((*sc).tempip,XFS_IOLOCK_EXCL) { (*sc).temp_ilock_flags|=XFS_IOLOCK_EXCL; true } else { false } }
pub unsafe fn xrep_tempfile_iolock_polled(sc:*mut xfs_scrub)->i32 { let mut e=0; while !xrep_tempfile_iolock_nowait(sc) { if xchk_should_terminate(sc,&mut e) { return e; } delay(1); } 0 }
pub unsafe fn xrep_tempfile_iounlock(sc:*mut xfs_scrub){ xfs_iunlock((*sc).tempip,XFS_IOLOCK_EXCL); (*sc).temp_ilock_flags &= !XFS_IOLOCK_EXCL; }
pub unsafe fn xrep_tempfile_ilock(sc:*mut xfs_scrub){ (*sc).temp_ilock_flags|=XFS_ILOCK_EXCL; xfs_ilock((*sc).tempip,XFS_ILOCK_EXCL); }
pub unsafe fn xrep_tempfile_ilock_nowait(sc:*mut xfs_scrub)->bool { if xfs_ilock_nowait((*sc).tempip,XFS_ILOCK_EXCL){(*sc).temp_ilock_flags|=XFS_ILOCK_EXCL;true}else{false} }
pub unsafe fn xrep_tempfile_iunlock(sc:*mut xfs_scrub){ xfs_iunlock((*sc).tempip,XFS_ILOCK_EXCL); (*sc).temp_ilock_flags &= !XFS_ILOCK_EXCL; }
pub unsafe fn xrep_tempfile_ilock_both(sc:*mut xfs_scrub){ xfs_lock_two_inodes((*sc).ip,XFS_ILOCK_EXCL,(*sc).tempip,XFS_ILOCK_EXCL); (*sc).ilock_flags|=XFS_ILOCK_EXCL; (*sc).temp_ilock_flags|=XFS_ILOCK_EXCL; }
pub unsafe fn xrep_tempfile_iunlock_both(sc:*mut xfs_scrub){ xrep_tempfile_iunlock(sc); xchk_iunlock(sc,XFS_ILOCK_EXCL); }
pub unsafe fn xrep_tempfile_rele(sc:*mut xfs_scrub){ if (*sc).tempip.is_null(){return;} if (*sc).temp_ilock_flags!=0{xfs_iunlock((*sc).tempip,(*sc).temp_ilock_flags);(*sc).temp_ilock_flags=0;} xrep_tempfile_remove_metadir(sc); xchk_irele(sc,(*sc).tempip); (*sc).tempip=core::ptr::null_mut(); }

pub unsafe fn xrep_tempfile_prealloc(sc:*mut xfs_scrub,mut off:xfs_fileoff_t,len:xfs_filblks_t)->i32 { let end=off+len; ASSERT(!(*sc).tempip.is_null()); ASSERT(!XFS_NOT_DQATTACHED((*sc).mp,(*sc).tempip)); while off<end { let mut map=xfs_bmbt_irec::default(); let mut n=1; let mut e=xfs_bmapi_read((*sc).tempip,off,end-off,&mut map,&mut n,XFS_DATA_FORK); if e!=0{return e;} if n==0{return -EFSCORRUPTED;} if !xfs_bmap_is_written_extent(&map){if map.br_startblock==DELAYSTARTBLOCK{return -EFSCORRUPTED;} n=1;e=xfs_bmapi_write((*sc).tp,(*sc).tempip,off,end-off,XFS_BMAPI_CONVERT|XFS_BMAPI_ZERO,0,&mut map,&mut n);if e!=0{return e;}if n!=1{return -EFSCORRUPTED;}trace_xrep_tempfile_prealloc(sc,XFS_DATA_FORK,&map);e=xfs_defer_finish(&mut (*sc).tp);if e!=0{return e;}} off=map.br_startoff+map.br_blockcount;} 0 }

pub unsafe fn xrep_tempfile_set_isize(sc:*mut xfs_scrub,isize:u64)->i32 { if (*(*sc).tempip).i_disk_size==isize{return 0;} (*(*sc).tempip).i_disk_size=isize;i_size_write(VFS_I((*sc).tempip),isize);xrep_tempfile_roll_trans(sc) }
pub unsafe fn xrep_tempfile_roll_trans(sc:*mut xfs_scrub)->i32 { xfs_trans_log_inode((*sc).tp,(*sc).tempip,XFS_ILOG_CORE);let e=xrep_roll_trans(sc);if e!=0{return e;}xfs_trans_ijoin((*sc).tp,(*sc).tempip,0);0 }

pub unsafe fn xrep_tempfile_copyin(sc:*mut xfs_scrub,mut off:xfs_fileoff_t,len:xfs_filblks_t,prep_fn:xrep_tempfile_copyin_fn,data:*mut core::ffi::c_void)->i32 { let mp=(*sc).mp;let end=off+len;let mut pos=XFS_FSB_TO_B(mp,off);let flush_mask=XFS_B_TO_FSBT(mp,1u64<<19)-1;let mut buffers_list=LIST_HEAD_INIT();while off<end {let mut map=xfs_bmbt_irec::default();let mut n=1;let mut e=xfs_bmapi_read((*sc).tempip,off,1,&mut map,&mut n,0);if e!=0{return e;}if n==0||!xfs_bmap_is_written_extent(&map){return -EFSCORRUPTED;}let mut bp: *mut xfs_buf=core::ptr::null_mut();e=xfs_trans_get_buf((*sc).tp,(*mp).m_ddev_targp,XFS_FSB_TO_DADDR(mp,map.br_startblock),(*mp).m_bsize,0,&mut bp);if e!=0{return e;}trace_xrep_tempfile_copyin(sc,XFS_DATA_FORK,&map);e=prep_fn(sc,bp,data);if e!=0{xfs_trans_brelse((*sc).tp,bp);return e;}xfs_buf_delwri_queue_here(bp,&mut buffers_list);xfs_trans_brelse((*sc).tp,bp);if (off&flush_mask)==0 {e=xfs_buf_delwri_submit(&mut buffers_list);if e!=0{return e;}}off+=1;pos+=(*mp).m_sb.sb_blocksize;}let e=xfs_buf_delwri_submit(&mut buffers_list);if e!=0{return e;}if !list_empty(&buffers_list){return -EIO;}0 }

pub unsafe fn xrep_tempexch_trans_reserve(sc:*mut xfs_scrub,whichfork:i32,off:xfs_fileoff_t,len:xfs_filblks_t,tx:*mut xrep_tempexch)->i32 { ASSERT(!(*sc).tp.is_null());xfs_assert_ilocked((*sc).ip,XFS_ILOCK_EXCL);xfs_assert_ilocked((*sc).tempip,XFS_ILOCK_EXCL);let e=xrep_tempexch_prep_request(sc,whichfork,off,len,tx);if e!=0{return e;}let e=xfs_exchmaps_estimate(&mut (*tx).req);if e!=0{return e;}let e=xfs_trans_reserve_more((*sc).tp,(*tx).req.resblks,0);if e!=0{return e;}xrep_tempexch_reserve_quota(sc,tx) }
pub unsafe fn xrep_tempexch_trans_alloc(sc:*mut xfs_scrub,whichfork:i32,tx:*mut xrep_tempexch)->i32 { ASSERT((*sc).tp.is_null());ASSERT(xfs_has_exchange_range((*sc).mp));let e=xrep_tempexch_prep_request(sc,whichfork,0,XFS_MAX_FILEOFF,tx);if e!=0{return e;}let e=xrep_tempexch_estimate(sc,tx);if e!=0{return e;}let mut flags=0;if xfs_has_lazysbcount((*sc).mp){flags|=XFS_TRANS_RES_FDBLKS;}let e=xfs_trans_alloc((*sc).mp,&mut (*M_RES((*sc).mp)).tr_itruncate,(*tx).req.resblks,0,flags,&mut (*sc).tp);if e!=0{return e;}(*sc).temp_ilock_flags|=XFS_ILOCK_EXCL;(*sc).ilock_flags|=XFS_ILOCK_EXCL;xfs_exchrange_ilock((*sc).tp,(*sc).ip,(*sc).tempip);xrep_tempexch_reserve_quota(sc,tx) }
pub unsafe fn xrep_tempexch_contents(sc:*mut xfs_scrub,tx:*mut xrep_tempexch)->i32 { ASSERT(xfs_has_exchange_range((*sc).mp));xfs_exchange_mappings((*sc).tp,&mut (*tx).req);let e=xfs_defer_finish(&mut (*sc).tp);if e!=0{return e;}if ((*tx).req.flags&XFS_EXCHMAPS_SET_SIZES)!=0{let t=i_size_read(VFS_I((*sc).ip));i_size_write(VFS_I((*sc).ip),i_size_read(VFS_I((*sc).tempip)));i_size_write(VFS_I((*sc).tempip),t);}0 }
pub unsafe fn xrep_tempfile_copyout_local(sc:*mut xfs_scrub,whichfork:i32){let tifp=xfs_ifork_ptr((*sc).tempip,whichfork);let ifp=xfs_ifork_ptr((*sc).ip,whichfork);ASSERT(!tifp.is_null());ASSERT(!ifp.is_null());ASSERT((*tifp).if_format==XFS_DINODE_FMT_LOCAL);ASSERT((*ifp).if_format==XFS_DINODE_FMT_LOCAL);if whichfork==XFS_DATA_FORK{i_size_write(VFS_I((*sc).ip),i_size_read(VFS_I((*sc).tempip)));(*(*sc).ip).i_disk_size=(*(*sc).tempip).i_disk_size;}else if whichfork!=XFS_ATTR_FORK{ASSERT(false);return;}xfs_idestroy_fork(ifp);xfs_init_local_fork((*sc).ip,whichfork,(*tifp).if_data,(*tifp).if_bytes);xfs_trans_log_inode((*sc).tp,(*sc).ip,XFS_ILOG_CORE|xfs_ilog_fdata(whichfork));}

unsafe fn xrep_tempexch_prep_request(sc:*mut xfs_scrub,whichfork:i32,off:xfs_fileoff_t,len:xfs_filblks_t,tx:*mut xrep_tempexch)->i32{core::ptr::write_bytes(tx as *mut u8,0,core::mem::size_of::<xrep_tempexch>());if whichfork==XFS_COW_FORK{return -EINVAL;}if xfs_ifork_ptr((*sc).ip,whichfork).is_null()||xfs_ifork_ptr((*sc).tempip,whichfork).is_null(){return -EINVAL;}(*tx).req.ip1=(*sc).tempip;(*tx).req.ip2=(*sc).ip;(*tx).req.startoff1=off;(*tx).req.startoff2=off;(*tx).req.blockcount=len;if whichfork==XFS_ATTR_FORK{(*tx).req.flags|=XFS_EXCHMAPS_ATTR_FORK;}else if whichfork==XFS_DATA_FORK&&off==0&&len==XFS_MAX_FILEOFF{(*tx).req.flags|=XFS_EXCHMAPS_SET_SIZES;}0}
unsafe fn xrep_tempexch_estimate(sc:*mut xfs_scrub,tx:*mut xrep_tempexch)->i32{let req=&mut (*tx).req;let a=xfs_ifork_ptr((*sc).ip,xfs_exchmaps_reqfork(req));let b=xfs_ifork_ptr((*sc).tempip,xfs_exchmaps_reqfork(req));let mut state=0;if (*a).if_format==XFS_DINODE_FMT_LOCAL{state|=1;}if (*b).if_format==XFS_DINODE_FMT_LOCAL{state|=2;}if state==0{return xfs_exchrange_estimate(req);}req.ip1_bcount=1;req.ip2_bcount=1;req.nr_exchanges=1;req.resblks=if state==3{2}else{1};xfs_exchmaps_estimate_overhead(req)}
unsafe fn xrep_tempexch_reserve_quota(_sc:*mut xfs_scrub,_tx:*const xrep_tempexch)->i32{0}

pub unsafe fn xrep_is_tempfile(ip:*const xfs_inode)->bool { let inode=&(*ip).i_vnode;let mp=(*ip).i_mount;if xfs_has_metadir(mp)&&((*ip).i_diflags2&XFS_DIFLAG2_METADATA)!=0{return __xfs_iflags_test(ip,XFS_IRECOVERY);} IS_PRIVATE(inode)&&((*inode).i_opflags&IOP_XATTR)==0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
