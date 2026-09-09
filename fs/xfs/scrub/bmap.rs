// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of xfs/scrub/bmap.c.  C headers and external symbols are
 * supplied by the surrounding translation unit. */

#[repr(C)]
pub struct xchk_bmap_info {
    pub sc: *mut xfs_scrub,
    pub icur: xfs_iext_cursor,
    pub prev_rec: xfs_bmbt_irec,
    pub is_rt: bool,
    pub is_shared: bool,
    pub was_loaded: bool,
    pub whichfork: i32,
}

pub unsafe fn xchk_setup_inode_bmap(sc: *mut xfs_scrub) -> i32 {
    let mut error: i32;
    if xchk_need_intent_drain(sc) { xchk_fsgates_enable(sc, XCHK_FSGATES_DRAIN); }
    error = xchk_iget_for_scrubbing(sc); if error != 0 { return error; }
    xchk_ilock(sc, XFS_IOLOCK_EXCL);
    if S_ISREG((*VFS_I((*sc).ip)).i_mode) && (*(*sc).sm).sm_type != XFS_SCRUB_TYPE_BMBTA {
        let mapping = (*VFS_I((*sc).ip)).i_mapping; let is_repair = xchk_could_repair(sc);
        xchk_ilock(sc, XFS_MMAPLOCK_EXCL);
        if is_repair { error = xfs_break_layouts(VFS_I((*sc).ip), &mut (*sc).ilock_flags, BREAK_WRITE); if error != 0 { return error; } }
        inode_dio_wait(VFS_I((*sc).ip));
        error = filemap_fdatawrite(mapping); if error == 0 { error = filemap_fdatawait_keep_errors(mapping); }
        if error != 0 && error != -ENOSPC && error != -EIO { return error; }
        if is_repair { error = invalidate_inode_pages2((*VFS_I((*sc).ip)).i_mapping); if error != 0 { return error; } }
    }
    error = xchk_trans_alloc(sc, 0); if error != 0 { return error; }
    error = xchk_ino_dqattach(sc); if error != 0 { return error; }
    xchk_ilock(sc, XFS_ILOCK_EXCL); error
}

unsafe fn xchk_bmap_get_rmap(i: &mut xchk_bmap_info, r: *mut xfs_bmbt_irec, bno: xfs_agblock_t, owner: u64, out: *mut xfs_rmap_irec) -> bool {
    let mut curp = &mut (*i.sc).sa.rmap_cur as *mut *mut xfs_btree_cur;
    if xfs_ifork_is_realtime((*i.sc).ip, i.whichfork) { curp = &mut (*i.sc).sr.rmap_cur; }
    if (*curp).is_null() { return false; }
    let mut flags = 0; if i.whichfork == XFS_ATTR_FORK { flags |= XFS_RMAP_ATTR_FORK; }
    if (*r).br_state == XFS_EXT_UNWRITTEN { flags |= XFS_RMAP_UNWRITTEN; }
    let off = if i.whichfork == XFS_COW_FORK { 0 } else { (*r).br_startoff };
    let mut has = 0; let error = if i.is_shared { xfs_rmap_lookup_le_range(*curp,bno,owner,off,flags,out,&mut has) } else { xfs_rmap_lookup_le(*curp,bno,owner,off,flags,out,&mut has) };
    if !xchk_should_check_xref(i.sc, &error, curp) { return false; }
    if has == 0 { xchk_fblock_xref_set_corrupt(i.sc,i.whichfork,(*r).br_startoff); } has != 0
}

unsafe fn xchk_bmap_xref_rmap(i: &mut xchk_bmap_info, r: *mut xfs_bmbt_irec, bno: xfs_agblock_t) {
    if xchk_skip_xref((*i.sc).sm) { return; } let mut rm = xfs_rmap_irec::default(); let owner=I_INO((*i.sc).ip) as u64;
    if !xchk_bmap_get_rmap(i,r,bno,owner,&mut rm) { return; }
    let bad = rm.rm_startblock != bno || rm.rm_startblock as u64 + rm.rm_blockcount != bno as u64 + (*r).br_blockcount || rm.rm_offset != (*r).br_startoff || rm.rm_offset + rm.rm_blockcount != (*r).br_startoff + (*r).br_blockcount || rm.rm_owner != owner || (((*r).br_state == XFS_EXT_UNWRITTEN) != ((rm.rm_flags & XFS_RMAP_UNWRITTEN)!=0)) || ((i.whichfork == XFS_ATTR_FORK) != ((rm.rm_flags & XFS_RMAP_ATTR_FORK)!=0)) || (rm.rm_flags & XFS_RMAP_BMBT_BLOCK)!=0;
    if bad { xchk_fblock_xref_set_corrupt(i.sc,i.whichfork,(*r).br_startoff); }
}

unsafe fn xchk_bmap_xref_rmap_cow(i:&mut xchk_bmap_info,r:*mut xfs_bmbt_irec,bno:xfs_agblock_t){ if (*i.sc).sa.rmap_cur.is_null()||xchk_skip_xref((*i.sc).sm){return;} let mut rm=xfs_rmap_irec::default(); if !xchk_bmap_get_rmap(i,r,bno,XFS_RMAP_OWN_COW,&mut rm){return;} if rm.rm_startblock>bno||rm.rm_startblock as u64+rm.rm_blockcount<bno as u64+(*r).br_blockcount||rm.rm_owner!=XFS_RMAP_OWN_COW||(rm.rm_flags&(XFS_RMAP_ATTR_FORK|XFS_RMAP_BMBT_BLOCK|XFS_RMAP_UNWRITTEN))!=0{xchk_fblock_xref_set_corrupt(i.sc,i.whichfork,(*r).br_startoff);} }

unsafe fn xchk_bmap_iextent(ip:*mut xfs_inode,i:&mut xchk_bmap_info,r:*mut xfs_bmbt_irec){let mp=(*i.sc).mp;if (*r).br_startoff< i.prev_rec.br_startoff+i.prev_rec.br_blockcount||!xfs_verify_fileext(mp,(*r).br_startoff,(*r).br_blockcount){xchk_fblock_set_corrupt(i.sc,i.whichfork,(*r).br_startoff);return;} if (*r).br_state==XFS_EXT_UNWRITTEN&&i.whichfork==XFS_ATTR_FORK{xchk_fblock_set_corrupt(i.sc,i.whichfork,(*r).br_startoff);return;} if (*i.sc).sm.sm_flags&XFS_SCRUB_OFLAG_CORRUPT!=0{return;} if i.is_rt{xchk_bmap_rt_iextent_xref(ip,i,r)}else{xchk_bmap_iextent_xref(ip,i,r)} }

unsafe fn xchk_bmap(sc:*mut xfs_scrub, whichfork:i32)->i32 {
    let ip=(*sc).ip; let ifp=xfs_ifork_ptr(ip,whichfork); if ifp.is_null(){return -ENOENT;}
    let mut info=xchk_bmap_info{sc,icur:std::mem::zeroed(),prev_rec:std::mem::zeroed(),is_rt:xfs_ifork_is_realtime(ip,whichfork),is_shared:whichfork==XFS_DATA_FORK&&xfs_is_reflink_inode(ip),was_loaded:false,whichfork};
    match (*ifp).if_format { XFS_DINODE_FMT_EXTENTS=>{}, XFS_DINODE_FMT_BTREE=>{let e=xchk_bmap_btree(sc,whichfork,&mut info);if e!=0{return e;}}, XFS_DINODE_FMT_UUID|XFS_DINODE_FMT_DEV|XFS_DINODE_FMT_LOCAL|XFS_DINODE_FMT_META_BTREE=>{if whichfork==XFS_COW_FORK{xchk_fblock_set_corrupt(sc,whichfork,0);}return 0;}, _=>{xchk_fblock_set_corrupt(sc,whichfork,0);return 0;} }
    if (*(*sc).sm).sm_flags&XFS_SCRUB_OFLAG_CORRUPT!=0{return 0;}
    let mut endoff=0;let mut error=xfs_bmap_last_offset(ip,&mut endoff,whichfork);if !xchk_fblock_process_error(sc,whichfork,0,&mut error){return error;}
    let mut r=xfs_bmbt_irec::default(); while xchk_bmap_iext_iter(&mut info,&mut r){if xchk_should_terminate(sc,&mut error)||(*(*sc).sm).sm_flags&XFS_SCRUB_OFLAG_CORRUPT!=0{return 0;}if r.br_startoff>=endoff{xchk_fblock_set_corrupt(sc,whichfork,r.br_startoff);return 0;}if isnullstartblock(r.br_startblock){xchk_bmap_iextent_delalloc(ip,&mut info,&mut r)}else{xchk_bmap_iextent(ip,&mut info,&mut r)}info.prev_rec=r;}
    if xchk_bmap_want_check_rmaps(&mut info){error=xchk_bmap_check_rmaps(sc,whichfork);if !xchk_fblock_xref_process_error(sc,whichfork,0,&mut error){return error;}}0
}

pub unsafe fn xchk_bmap_data(sc:*mut xfs_scrub)->i32 { if xchk_file_looks_zapped(sc,XFS_SICK_INO_BMBTD_ZAPPED){xchk_ip_set_corrupt(sc,(*sc).ip);return 0;} let e=xchk_bmap(sc,XFS_DATA_FORK);if e!=0{return e;}xchk_mark_healthy_if_clean(sc,XFS_SICK_INO_BMBTD_ZAPPED);0 }
pub unsafe fn xchk_bmap_attr(sc:*mut xfs_scrub)->i32 { if xchk_file_looks_zapped(sc,XFS_SICK_INO_BMBTA_ZAPPED){xchk_ip_set_corrupt(sc,(*sc).ip);return 0;} let e=xchk_bmap(sc,XFS_ATTR_FORK);if e!=0{return e;}xchk_mark_healthy_if_clean(sc,XFS_SICK_INO_BMBTA_ZAPPED);0 }
pub unsafe fn xchk_bmap_cow(sc:*mut xfs_scrub)->i32 { xchk_bmap(sc,XFS_COW_FORK) }

extern "C" { fn xchk_bmap_btree(sc:*mut xfs_scrub,whichfork:i32,info:*mut xchk_bmap_info)->i32; fn xchk_bmap_want_check_rmaps(info:*mut xchk_bmap_info)->bool; fn xchk_bmap_check_rmaps(sc:*mut xfs_scrub,whichfork:i32)->i32; fn xchk_bmap_iext_iter(info:*mut xchk_bmap_info,r:*mut xfs_bmbt_irec)->bool; fn xchk_bmap_iextent_delalloc(ip:*mut xfs_inode,info:*mut xchk_bmap_info,r:*mut xfs_bmbt_irec); fn xchk_bmap_rt_iextent_xref(ip:*mut xfs_inode,info:*mut xchk_bmap_info,r:*mut xfs_bmbt_irec); fn xchk_bmap_iextent_xref(ip:*mut xfs_inode,info:*mut xchk_bmap_info,r:*mut xfs_bmbt_irec); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
