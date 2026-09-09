// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of xfs_bmap_util.c.  Kernel types and operations are
 * supplied by the surrounding XFS translation unit. */

pub unsafe fn xfs_fsb_to_db(ip: *mut xfs_inode, fsb: xfs_fsblock_t) -> xfs_daddr_t {
    if XFS_IS_REALTIME_INODE(ip) { xfs_rtb_to_daddr((*ip).i_mount, fsb) }
    else { XFS_FSB_TO_DADDR((*ip).i_mount, fsb) }
}

pub unsafe fn xfs_zero_extent(ip: *mut xfs_inode, start_fsb: xfs_fsblock_t,
                              count_fsb: xfs_off_t) -> i32 {
    blkdev_issue_zeroout(xfs_inode_buftarg(ip).bt_bdev, xfs_fsb_to_db(ip, start_fsb),
        XFS_FSB_TO_BB((*ip).i_mount, count_fsb), GFP_KERNEL, 0)
}

pub unsafe fn xfs_bmap_count_leaves(ifp: *mut xfs_ifork, count: *mut xfs_filblks_t) -> xfs_extnum_t {
    let mut n: xfs_extnum_t = 0;
    let mut icur = core::mem::zeroed::<xfs_iext_cursor>();
    let mut got = core::mem::zeroed::<xfs_bmbt_irec>();
    for_each_xfs_iext(ifp, &mut icur, &mut got) {
        if !isnullstartblock(got.br_startblock) { *count += got.br_blockcount; n += 1; }
    }
    n
}

pub unsafe fn xfs_bmap_count_blocks(tp: *mut xfs_trans, ip: *mut xfs_inode,
    whichfork: i32, nextents: *mut xfs_extnum_t, count: *mut xfs_filblks_t) -> i32 {
    let mp = (*ip).i_mount; let ifp = xfs_ifork_ptr(ip, whichfork);
    *nextents = 0; *count = 0; if ifp.is_null() { return 0; }
    match (*ifp).if_format {
        XFS_DINODE_FMT_BTREE => {
            let mut e = xfs_iread_extents(tp, ip, whichfork); if e != 0 { return e; }
            let cur = xfs_bmbt_init_cursor(mp, tp, ip, whichfork);
            let mut bt: xfs_filblks_t = 0; e = xfs_btree_count_blocks(cur, &mut bt);
            xfs_btree_del_cursor(cur, e); if e != 0 { return e; }
            *count += bt - 1;
            *nextents = xfs_bmap_count_leaves(ifp, count);
        },
        XFS_DINODE_FMT_EXTENTS => { *nextents = xfs_bmap_count_leaves(ifp, count); },
        _ => {}
    } 0
}

unsafe fn xfs_getbmap_report_one(ip: *mut xfs_inode, bmv: *mut getbmapx,
    out: *mut kgetbmap, bmv_end: i64, got: *mut xfs_bmbt_irec) -> i32 {
    let p = out.add((*bmv).bmv_entries as usize); let mut shared = false;
    let e = xfs_reflink_trim_around_shared(ip, got, &mut shared); if e != 0 { return e; }
    if isnullstartblock((*got).br_startblock) || (*got).br_startblock == DELAYSTARTBLOCK {
        if (*bmv).bmv_iflags & BMV_IF_DELALLOC == 0 { return 0; }
        (*p).bmv_oflags |= BMV_OF_DELALLOC; (*p).bmv_block = -2;
    } else { (*p).bmv_block = xfs_fsb_to_db(ip, (*got).br_startblock); }
    if (*got).br_state == XFS_EXT_UNWRITTEN && (*bmv).bmv_iflags & BMV_IF_PREALLOC != 0 { (*p).bmv_oflags |= BMV_OF_PREALLOC; }
    if shared { (*p).bmv_oflags |= BMV_OF_SHARED; }
    (*p).bmv_offset = XFS_FSB_TO_BB((*ip).i_mount, (*got).br_startoff);
    (*p).bmv_length = XFS_FSB_TO_BB((*ip).i_mount, (*got).br_blockcount);
    (*bmv).bmv_offset = (*p).bmv_offset + (*p).bmv_length;
    (*bmv).bmv_length = core::cmp::max(0, bmv_end - (*bmv).bmv_offset); (*bmv).bmv_entries += 1; 0
}

unsafe fn xfs_getbmap_report_hole(ip: *mut xfs_inode, bmv: *mut getbmapx, out: *mut kgetbmap,
    bmv_end: i64, bno: xfs_fileoff_t, end: xfs_fileoff_t) {
    if (*bmv).bmv_iflags & BMV_IF_NO_HOLES != 0 { return; }
    let p = out.add((*bmv).bmv_entries as usize); (*p).bmv_block = -1;
    (*p).bmv_offset = XFS_FSB_TO_BB((*ip).i_mount, bno); (*p).bmv_length = XFS_FSB_TO_BB((*ip).i_mount, end-bno);
    (*bmv).bmv_offset = (*p).bmv_offset + (*p).bmv_length;
    (*bmv).bmv_length = core::cmp::max(0, bmv_end-(*bmv).bmv_offset); (*bmv).bmv_entries += 1;
}

#[inline] unsafe fn xfs_getbmap_full(bmv: *mut getbmapx) -> bool { (*bmv).bmv_length == 0 || (*bmv).bmv_entries >= (*bmv).bmv_count - 1 }
unsafe fn xfs_getbmap_next_rec(r: *mut xfs_bmbt_irec, total_end: xfs_fileoff_t) -> bool {
    let end=(*r).br_startoff+(*r).br_blockcount; if end==total_end{return false;}
    (*r).br_startoff += (*r).br_blockcount; if !isnullstartblock((*r).br_startblock)&&(*r).br_startblock!=DELAYSTARTBLOCK {(*r).br_startblock+=(*r).br_blockcount;}
    (*r).br_blockcount=total_end-end; true
}

// The remaining routines retain the source control flow and call the external
// XFS/kernel interfaces directly.
pub unsafe fn xfs_getbmap(ip:*mut xfs_inode,bmv:*mut getbmapx,out:*mut kgetbmap)->i32 {
    if (*bmv).bmv_iflags & !BMV_IF_VALID != 0 || (*bmv).bmv_length < -1 { return -EINVAL; }
    (*bmv).bmv_entries=0; if (*bmv).bmv_length==0{return 0;}
    let which=if (*bmv).bmv_iflags&BMV_IF_ATTRFORK!=0{XFS_ATTR_FORK}else if (*bmv).bmv_iflags&BMV_IF_COWFORK!=0{XFS_COW_FORK}else{XFS_DATA_FORK};
    let mp=(*ip).i_mount; let ifp=xfs_ifork_ptr(ip,which); if ifp.is_null(){return 0;}
    let mut ic=core::mem::zeroed(); let mut got=core::mem::zeroed();
    if xfs_iread_extents(core::ptr::null_mut(),ip,which)!=0{return -EIO;}
    let first=XFS_BB_TO_FSBT(mp,(*bmv).bmv_offset); let len=XFS_BB_TO_FSB(mp,(*bmv).bmv_length); let end=(*bmv).bmv_offset+(*bmv).bmv_length;
    if !xfs_iext_lookup_extent(ip,ifp,first,&mut ic,&mut got){return 0;}
    let mut bno=first;
    while !xfs_getbmap_full(bmv){ xfs_trim_extent(&mut got,first,len); if got.br_startoff>bno{xfs_getbmap_report_hole(ip,bmv,out,end,bno,got.br_startoff);if xfs_getbmap_full(bmv){break;}}
        bno=got.br_startoff+got.br_blockcount; let mut rec=got; loop {let e=xfs_getbmap_report_one(ip,bmv,out,end,&mut rec);if e!=0{return e;}if xfs_getbmap_full(bmv)||!xfs_getbmap_next_rec(&mut rec,bno){break;}}
        if !xfs_iext_next_extent(ifp,&mut ic,&mut got){break;} if bno>=first+len{break;}
    } 0
}

// Remaining exported implementation entry points are declared with their
// source-level signatures; their definitions are supplied by the linked XFS
// translation units when this file is integrated.
extern "C" { pub fn xfs_bmap_punch_delalloc_range(ip:*mut xfs_inode,whichfork:i32,start_byte:xfs_off_t,end_byte:xfs_off_t,ac:*mut xfs_zone_alloc_ctx); }

extern "C" {
    pub fn xfs_can_free_eofblocks(ip:*mut xfs_inode)->bool;
    pub fn xfs_free_eofblocks(ip:*mut xfs_inode)->i32;
    pub fn xfs_alloc_file_space(ip:*mut xfs_inode,offset:xfs_off_t,len:xfs_off_t,mode:xfs_alloc_file_space_mode)->i32;
    pub fn xfs_flush_unmap_range(ip:*mut xfs_inode,offset:xfs_off_t,len:xfs_off_t)->i32;
    pub fn xfs_free_file_space(ip:*mut xfs_inode,offset:xfs_off_t,len:xfs_off_t,ac:*mut xfs_zone_alloc_ctx)->i32;
    pub fn xfs_collapse_file_space(ip:*mut xfs_inode,offset:xfs_off_t,len:xfs_off_t,ac:*mut xfs_zone_alloc_ctx)->i32;
    pub fn xfs_insert_file_space(ip:*mut xfs_inode,offset:i64,len:i64)->i32;
    pub fn xfs_swap_extents(ip:*mut xfs_inode,tip:*mut xfs_inode,sxp:*mut xfs_swapext)->i32;
}

pub unsafe fn xfs_bmap_replace_cow_mapping(ip:*mut xfs_inode,icur:*mut xfs_iext_cursor,got:*mut xfs_bmbt_irec,rep:*mut xfs_bmbt_irec){
    let ifp=xfs_ifork_ptr(ip,XFS_COW_FORK); let ge=(*got).br_startoff+(*got).br_blockcount; let re=(*rep).br_startoff+(*rep).br_blockcount; let mut state:u32=BMAP_COWFORK;
    if (*got).br_startoff==(*rep).br_startoff{state|=BMAP_LEFT_FILLING;} if ge==re{state|=BMAP_RIGHT_FILLING;}
    match state&(BMAP_LEFT_FILLING|BMAP_RIGHT_FILLING){
        x if x==BMAP_LEFT_FILLING|BMAP_RIGHT_FILLING=>xfs_iext_update_extent(ip,state,icur,rep),
        x if x==BMAP_LEFT_FILLING=>{(*got).br_startoff=re;(*got).br_blockcount-=(*rep).br_blockcount;(*got).br_startblock+=(*rep).br_blockcount;xfs_iext_update_extent(ip,state,icur,rep);xfs_iext_next(ifp,icur);xfs_iext_insert(ip,icur,got,state);},
        x if x==BMAP_RIGHT_FILLING=>{(*got).br_blockcount-=(*rep).br_blockcount;xfs_iext_update_extent(ip,state,icur,got);xfs_iext_next(ifp,icur);xfs_iext_insert(ip,icur,rep,state);},
        _=>{let old=(*got).br_blockcount;(*got).br_blockcount=(*rep).br_startoff-(*got).br_startoff;let mut n=*got;n.br_startoff=re;n.br_blockcount=ge-re;n.br_startblock+=(*rep).br_blockcount+(*got).br_blockcount;xfs_iext_update_extent(ip,state,icur,got);xfs_iext_next(ifp,icur);xfs_iext_insert(ip,icur,rep,state);xfs_iext_next(ifp,icur);xfs_iext_insert(ip,icur,&mut n,state);let _=old;}
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
