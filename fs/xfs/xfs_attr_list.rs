// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * Copyright (c) 2013 Red Hat, Inc.
 * All Rights Reserved.
 */

// Dependencies supplied by the surrounding XFS implementation are intentionally external.

unsafe fn xfs_attr_shortform_compare(a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> i32 {
    let sa = a as *const xfs_attr_sf_sort_t;
    let sb = b as *const xfs_attr_sf_sort_t;
    if (*sa).hash < (*sb).hash { -1 }
    else if (*sa).hash > (*sb).hash { 1 }
    else { (*sa).entno - (*sb).entno }
}

#[inline]
unsafe fn xfs_isreset_cursor(cursor: *const xfs_attrlist_cursor_kern) -> bool {
    !(*cursor).initted && (*cursor).hashval == 0 && (*cursor).blkno == 0 && (*cursor).offset == 0
}

/*
 * Copy out entries of shortform attribute lists for attr_list().
 * Shortform attribute lists are not stored in hashval sorted order.
 * If the output buffer is not large enough to hold them all, then we have to
 * calculate each entries' hashvalue and sort them before returning them.
 */
unsafe fn xfs_attr_shortform_list(context: *mut xfs_attr_list_context) -> i32 {
    let cursor = &mut (*context).cursor;
    let dp = (*context).dp;
    let sf = (*dp).i_af.if_data as *mut xfs_attr_sf_hdr;
    let mut error = 0;
    assert!(!sf.is_null());
    if (*sf).count == 0 { return 0; }
    trace_xfs_attr_list_sf(context);
    if (*context).bufsize == 0 || (xfs_isreset_cursor(cursor) && ((*dp).i_af.if_bytes + (*sf).count as usize * 16) < (*context).bufsize as usize) {
        let mut sfe = xfs_attr_sf_firstentry(sf);
        for i in 0..(*sf).count {
            if XFS_IS_CORRUPT((*context).dp).call((*(*context).dp).i_mount, !xfs_attr_namecheck((*sfe).flags, (*sfe).nameval, (*sfe).namelen)) {
                xfs_dirattr_mark_sick((*context).dp, XFS_ATTR_FORK); return -EFSCORRUPTED;
            }
            ((*context).put_listent)(context, (*sfe).flags, (*sfe).nameval, (*sfe).namelen as i32, (*sfe).nameval.add((*sfe).namelen as usize), (*sfe).valuelen as i32);
            if (*context).seen_enough { break; }
            sfe = xfs_attr_sf_nextentry(sfe);
            let _ = i;
        }
        trace_xfs_attr_list_sf_all(context); return 0;
    }
    if (*context).bufsize == 0 { return 0; }
    let sbsize = (*sf).count as usize * core::mem::size_of::<xfs_attr_sf_sort_t>();
    let mut sbuf = kmalloc(sbsize, GFP_KERNEL | __GFP_NOLOCKDEP | __GFP_NOFAIL) as *mut xfs_attr_sf_sort_t;
    let mut sbp = sbuf;
    let mut sfe = xfs_attr_sf_firstentry(sf);
    let mut nsbuf = 0i32;
    for i in 0..(*sf).count {
        if ((sfe as *mut i8) < (sf as *mut i8)) || ((sfe as *mut i8) >= (sf as *mut i8).add((*dp).i_af.if_bytes)) || !xfs_attr_check_namespace((*sfe).flags) {
            XFS_CORRUPTION_ERROR("xfs_attr_shortform_list", XFS_ERRLEVEL_LOW, (*context).dp as *mut _, sfe as *mut _, core::mem::size_of::<xfs_attr_sf_entry>());
            kfree(sbuf as *mut _); xfs_dirattr_mark_sick(dp, XFS_ATTR_FORK); return -EFSCORRUPTED;
        }
        (*sbp).entno = i as i32; (*sbp).name = (*sfe).nameval; (*sbp).namelen = (*sfe).namelen;
        (*sbp).value = (*sfe).nameval.add((*sfe).namelen as usize); (*sbp).valuelen = (*sfe).valuelen; (*sbp).flags = (*sfe).flags;
        (*sbp).hash = xfs_attr_hashval((*dp).i_mount, (*sfe).flags, (*sfe).nameval, (*sfe).namelen, (*sfe).nameval.add((*sfe).namelen as usize), (*sfe).valuelen);
        sfe = xfs_attr_sf_nextentry(sfe); sbp = sbp.add(1); nsbuf += 1;
    }
    xfs_sort(sbuf as *mut _, nsbuf as usize, core::mem::size_of::<xfs_attr_sf_sort_t>(), Some(xfs_attr_shortform_compare));
    let mut count = 0i32; cursor.initted = true; cursor.blkno = 0;
    sbp = sbuf;
    let mut i = 0;
    while i < nsbuf { if (*sbp).hash == cursor.hashval { if cursor.offset == count { break; } count += 1; } else if (*sbp).hash > cursor.hashval { break; } i += 1; sbp = sbp.add(1); }
    if i != nsbuf { while i < nsbuf {
        if cursor.hashval != (*sbp).hash { cursor.hashval = (*sbp).hash; cursor.offset = 0; }
        if XFS_IS_CORRUPT((*context).dp).call((*(*context).dp).i_mount, !xfs_attr_namecheck((*sbp).flags, (*sbp).name, (*sbp).namelen)) { xfs_dirattr_mark_sick((*context).dp, XFS_ATTR_FORK); error = -EFSCORRUPTED; break; }
        ((*context).put_listent)(context, (*sbp).flags, (*sbp).name, (*sbp).namelen, (*sbp).value, (*sbp).valuelen);
        if (*context).seen_enough { break; } cursor.offset += 1; i += 1; sbp = sbp.add(1);
    }}
    kfree(sbuf as *mut _); error
}

/* The remaining routines preserve the original XFS leaf/node traversal API. */
unsafe fn xfs_attr_node_list_lookup(context: *mut xfs_attr_list_context, cursor: *mut xfs_attrlist_cursor_kern, pbp: *mut *mut xfs_buf) -> i32 {
    let dp = (*context).dp; let mp = (*dp).i_mount; let tp = (*context).tp; let mut bp: *mut xfs_buf = core::ptr::null_mut(); let mut expected_level = 0u32;
    assert!((*pbp).is_null()); (*cursor).blkno = 0;
    loop { let error = xfs_da3_node_read(tp, dp, (*cursor).blkno, &mut bp, XFS_ATTR_FORK); if error != 0 { return error; }
        let node = (*bp).b_addr as *mut xfs_da_intnode; let magic = be16_to_cpu((*node).hdr.info.magic);
        if magic == XFS_ATTR_LEAF_MAGIC || magic == XFS_ATTR3_LEAF_MAGIC { break; }
        if magic != XFS_DA_NODE_MAGIC && magic != XFS_DA3_NODE_MAGIC { XFS_CORRUPTION_ERROR(__func__, XFS_ERRLEVEL_LOW, mp, node as *mut _, core::mem::size_of::<xfs_da_intnode>()); goto_corrupt!(bp, tp, dp); }
        let fa = xfs_da3_node_header_check(bp, I_INO(dp)); if !fa.is_null() { goto_corrupt!(bp, tp, dp); }
        let mut nodehdr = core::mem::MaybeUninit::<xfs_da3_icnode_hdr>::uninit(); xfs_da3_node_hdr_from_disk(mp, nodehdr.as_mut_ptr(), node); let nodehdr = nodehdr.assume_init();
        if nodehdr.level >= XFS_DA_NODE_MAXDEPTH { goto_corrupt!(bp, tp, dp); }
        if (*cursor).blkno == 0 { expected_level = nodehdr.level - 1; } else if expected_level != nodehdr.level { goto_corrupt!(bp, tp, dp); } else { expected_level -= 1; }
        let mut btree = nodehdr.btree; let mut i = 0; while i < nodehdr.count { if (*cursor).hashval <= be32_to_cpu((*btree).hashval) { (*cursor).blkno = be32_to_cpu((*btree).before); trace_xfs_attr_list_node_descend(context, btree); break; } btree = btree.add(1); i += 1; }
        xfs_trans_brelse(tp, bp); if i == nodehdr.count { return 0; } if XFS_IS_CORRUPT(mp).call((*cursor).blkno == 0) { xfs_dirattr_mark_sick(dp, XFS_ATTR_FORK); return -EFSCORRUPTED; }
    }
    let fa = xfs_attr3_leaf_header_check(bp, I_INO(dp)); if !fa.is_null() { __xfs_buf_mark_corrupt(bp, fa); xfs_trans_brelse(tp, bp); xfs_dirattr_mark_sick(dp, XFS_ATTR_FORK); return -EFSCORRUPTED; }
    if expected_level != 0 { xfs_buf_mark_corrupt(bp); xfs_trans_brelse(tp, bp); xfs_dirattr_mark_sick(dp, XFS_ATTR_FORK); return -EFSCORRUPTED; }
    *pbp = bp; 0
}

// The following leaf-list routines are direct unsafe translations of the C interfaces.
unsafe fn xfs_attr_node_list(context: *mut xfs_attr_list_context) -> i32 { let cursor = &mut (*context).cursor; let dp = (*context).dp; let mp = (*dp).i_mount; let mut bp = core::ptr::null_mut(); trace_xfs_attr_node_list(context); cursor.initted = true; if cursor.blkno > 0 { let e = xfs_da3_node_read((*context).tp, dp, cursor.blkno, &mut bp, XFS_ATTR_FORK); if e != 0 && e != -EFSCORRUPTED { return e; } } if bp.is_null() { let e = xfs_attr_node_list_lookup(context, cursor, &mut bp); if e != 0 || bp.is_null() { return e; } } loop { let e = xfs_attr3_leaf_list_int(bp, context); if e != 0 { xfs_trans_brelse((*context).tp,bp); return e; } let mut h = core::mem::MaybeUninit::<xfs_attr3_icleaf_hdr>::uninit(); xfs_attr3_leaf_hdr_from_disk((*mp).m_attr_geo,h.as_mut_ptr(),(*bp).b_addr as *mut _); let h=h.assume_init(); if (*context).seen_enough || h.forw == 0 { xfs_trans_brelse((*context).tp,bp); return 0; } cursor.blkno=h.forw; xfs_trans_brelse((*context).tp,bp); let e=xfs_attr3_leaf_read((*context).tp,dp,I_INO(dp),cursor.blkno,&mut bp); if e!=0{return e;} }}

unsafe fn xfs_attr3_leaf_list_int(bp: *mut xfs_buf, context: *mut xfs_attr_list_context) -> i32 { let cursor=&mut (*context).cursor; let leaf=(*bp).b_addr as *mut xfs_attr_leafblock; let mp=(*context).dp as *mut xfs_inode; let mut hdr=core::mem::MaybeUninit::<xfs_attr3_icleaf_hdr>::uninit(); xfs_attr3_leaf_hdr_from_disk((*(*mp).i_mount).m_attr_geo,hdr.as_mut_ptr(),leaf); let hdr=hdr.assume_init(); let entries=xfs_attr3_leaf_entryp(leaf); cursor.initted=true; let mut i=0; let mut entry=entries; if (*context).resynch { while i<hdr.count { if be32_to_cpu((*entry).hashval)==cursor.hashval { if cursor.offset==(*context).dupcnt { (*context).dupcnt=0; break; } (*context).dupcnt+=1; } else if be32_to_cpu((*entry).hashval)>cursor.hashval { (*context).dupcnt=0; break; } i+=1; entry=entry.add(1); } if i==hdr.count { trace_xfs_attr_list_notfound(context); return 0; } } (*context).resynch=false; while i<hdr.count { let hash=be32_to_cpu((*entry).hashval); if hash!=cursor.hashval {cursor.hashval=hash;cursor.offset=0;} if ((*entry).flags & XFS_ATTR_INCOMPLETE)!=0 && !(*context).allow_incomplete {i+=1;entry=entry.add(1);continue;} let (name,namelen,value,valuelen)=if ((*entry).flags&XFS_ATTR_LOCAL)!=0 {let n=xfs_attr3_leaf_name_local(leaf,i);((*n).nameval,(*n).namelen,(*n).nameval.add((*n).namelen as usize),be16_to_cpu((*n).valuelen) as i32)} else {let n=xfs_attr3_leaf_name_remote(leaf,i);((*n).name,(*n).namelen,core::ptr::null_mut(),be32_to_cpu((*n).valuelen) as i32)}; if XFS_IS_CORRUPT((*context).dp).call((*(*context).dp).i_mount,!xfs_attr_namecheck((*entry).flags,name,namelen)){xfs_dirattr_mark_sick((*context).dp,XFS_ATTR_FORK);return -EFSCORRUPTED;}((*context).put_listent)(context,(*entry).flags,name,namelen,value,valuelen);if (*context).seen_enough{break;}cursor.offset+=1;i+=1;entry=entry.add(1);}trace_xfs_attr_list_leaf_end(context);0 }

unsafe fn xfs_attr_leaf_list(context: *mut xfs_attr_list_context) -> i32 { trace_xfs_attr_leaf_list(context); (*context).cursor.blkno=0; let mut bp=core::ptr::null_mut(); let e=xfs_attr3_leaf_read((*context).tp,(*context).dp,I_INO((*context).dp),0,&mut bp); if e!=0{return e;} let e=xfs_attr3_leaf_list_int(bp,context);xfs_trans_brelse((*context).tp,bp);e }
unsafe fn xfs_attr_list_ilocked(context: *mut xfs_attr_list_context) -> i32 { let dp=(*context).dp;xfs_assert_ilocked(dp,XFS_ILOCK_SHARED|XFS_ILOCK_EXCL);if !xfs_inode_hasattr(dp){return 0;}if (*dp).i_af.if_format==XFS_DINODE_FMT_LOCAL{return xfs_attr_shortform_list(context);}let e=xfs_iread_extents(core::ptr::null_mut(),dp,XFS_ATTR_FORK);if e!=0{return e;}if xfs_attr_is_leaf(dp){xfs_attr_leaf_list(context)}else{xfs_attr_node_list(context)} }
unsafe fn xfs_attr_list(context: *mut xfs_attr_list_context) -> i32 { let dp=(*context).dp;XFS_STATS_INC((*dp).i_mount,xs_attr_list);if xfs_is_shutdown((*dp).i_mount){return -EIO;}let lock=xfs_ilock_attr_map_shared(dp);let e=xfs_attr_list_ilocked(context);xfs_iunlock(dp,lock);e }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
