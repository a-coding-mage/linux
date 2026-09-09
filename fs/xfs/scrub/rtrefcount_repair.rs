// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2021-2024 Oracle. All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// External XFS declarations and macros are supplied by the surrounding translation unit.

#[repr(C)]
pub struct xrep_rtrefc {
    pub refcount_records: *mut xfarray,
    pub new_btree: xrep_newbt,
    pub old_rtrefcountbt_blocks: xfsb_bitmap,
    pub sc: *mut xfs_scrub,
    pub array_cur: xfarray_idx_t,
    pub btblocks: xfs_filblks_t,
}

pub unsafe fn xrep_setup_rtrefcountbt(sc: *mut xfs_scrub) -> i32 {
    xrep_setup_xfbtree(sc, "realtime rmap record bag\\0".as_ptr() as *const i8)
}

unsafe fn xrep_rtrefc_check_ext(sc: *mut xfs_scrub, rec: *const xfs_refcount_irec) -> i32 {
    if !xfs_rtrefcount_check_irec((*sc).sr.rtg, rec).is_null() { return -EFSCORRUPTED; }
    if xfs_rgbno_to_rtxoff((*sc).mp, (*rec).rc_startblock) != 0 { return -EFSCORRUPTED; }
    let last = (*rec).rc_startblock + (*rec).rc_blockcount - 1;
    if xfs_rgbno_to_rtxoff((*sc).mp, last) != (*(*sc).mp).m_sb.sb_rextsize - 1 { return -EFSCORRUPTED; }
    xrep_require_rtext_inuse(sc, (*rec).rc_startblock, (*rec).rc_blockcount)
}

unsafe fn xrep_rtrefc_stash(rr: *mut xrep_rtrefc, domain: xfs_refc_domain, bno: xfs_rgblock_t, len: xfs_extlen_t, refcount: u64) -> i32 {
    let mut irec = xfs_refcount_irec { rc_startblock:bno, rc_blockcount:len, rc_refcount:refcount, rc_domain:domain };
    let mut error=0;
    if xchk_should_terminate((*rr).sc, &mut error) { return error; }
    irec.rc_refcount=core::cmp::min(XFS_REFC_REFCOUNT_MAX, refcount);
    error=xrep_rtrefc_check_ext((*rr).sc, &irec); if error != 0 { return error; }
    trace_xrep_refc_found(rtg_group((*(*rr).sc).sr.rtg), &irec);
    xfarray_append((*rr).refcount_records, &irec)
}
unsafe fn xrep_rtrefc_stash_cow(rr:*mut xrep_rtrefc,bno:xfs_rgblock_t,len:xfs_extlen_t)->i32{xrep_rtrefc_stash(rr,XFS_REFC_DOMAIN_COW,bno,len,1)}
#[inline] unsafe fn xrep_rtrefc_rmap_shareable(r:*const xfs_rmap_irec)->bool{!XFS_RMAP_NON_INODE_OWNER((*r).rm_owner)&&((*r).rm_flags&XFS_RMAP_UNWRITTEN)==0}
unsafe fn xrep_rtrefc_walk_rmaps(rr:*mut xrep_rtrefc,r:*mut xfs_rmap_irec,have:*mut bool)->i32{
 let c=(*(*rr).sc).sr.rmap_cur;let mp=(*c).bc_mp;let mut gt=0;let mut e=0;*have=false;
 loop{if xchk_should_terminate((*rr).sc,&mut e){return e} e=xfs_btree_increment(c,0,&mut gt);if e!=0{return e}if gt==0{return 0} e=xfs_rmap_get_rec(c,r,&mut gt);if e!=0{return e}
 if XFS_IS_CORRUPT(mp,gt==0){xfs_btree_mark_sick(c);return -EFSCORRUPTED}
 if (*r).rm_owner==XFS_RMAP_OWN_COW{e=xrep_rtrefc_stash_cow(rr,(*r).rm_startblock,(*r).rm_blockcount);if e!=0{return e}}
 else if xfs_is_sb_inum(mp,(*r).rm_owner)||((*r).rm_flags&(XFS_RMAP_ATTR_FORK|XFS_RMAP_BMBT_BLOCK))!=0{xfs_btree_mark_sick(c);return -EFSCORRUPTED}
 if xrep_rtrefc_rmap_shareable(r){break}}
 *have=true;0
}
#[inline] unsafe fn xrep_rtrefc_encode_startblock(r:*const xfs_refcount_irec)->u32{let mut s=(*r).rc_startblock&!XFS_REFC_COWFLAG;if (*r).rc_domain==XFS_REFC_DOMAIN_COW{s|=XFS_REFC_COWFLAG}s}
unsafe fn xrep_rtrefc_extent_cmp(a:*const core::ffi::c_void,b:*const core::ffi::c_void)->i32{let x=xrep_rtrefc_encode_startblock(a as _);let y=xrep_rtrefc_encode_startblock(b as _);if x>y{1}else if x<y{-1}else{0}}
unsafe fn xrep_rtrefc_sort_records(rr:*mut xrep_rtrefc)->i32{
 let mut e=xfarray_sort((*rr).refcount_records,xrep_rtrefc_extent_cmp,XFARRAY_SORT_KILLABLE);if e!=0{return e}
 let mut cur=XFARRAY_CURSOR_INIT;let mut dom=XFS_REFC_DOMAIN_SHARED;let mut next=0;
 while xfarray_next((*rr).refcount_records,&mut cur){let mut r=core::mem::zeroed();e=xfarray_load((*rr).refcount_records,cur,&mut r);if e!=0{return e}if dom==XFS_REFC_DOMAIN_SHARED&&r.rc_domain==XFS_REFC_DOMAIN_COW{dom=r.rc_domain;next=0}if dom!=r.rc_domain||r.rc_startblock<next{return -EFSCORRUPTED}next=r.rc_startblock+r.rc_blockcount} e
}
unsafe fn xrep_rtrefc_walk_rmap(c:*mut xfs_btree_cur,r:*const xfs_rmap_irec,p:*mut core::ffi::c_void)->i32{let rr=p as *mut xrep_rtrefc;let mut e=0;if xchk_should_terminate((*rr).sc,&mut e){return e}if (*r).rm_owner!=I_INO((*(*rr).sc).ip){return 0}e=xrep_check_ino_btree_mapping((*rr).sc,r);if e!=0{return e}xfsb_bitmap_set(&mut (*rr).old_rtrefcountbt_blocks,xfs_gbno_to_fsb((*c).bc_group,(*r).rm_startblock),(*r).rm_blockcount)}
unsafe fn xrep_rtrefc_scan_ag(rr:*mut xrep_rtrefc,pag:*mut xfs_perag)->i32{let sc=(*rr).sc;let mut e=xrep_ag_init(sc,pag,&mut (*sc).sa);if e==0{e=xfs_rmap_query_all((*sc).sa.rmap_cur,xrep_rtrefc_walk_rmap,rr as _);xchk_ag_free(sc,&mut (*sc).sa)}e}
unsafe fn xrep_rtrefc_find_refcounts(rr:*mut xrep_rtrefc)->i32{
 let sc=(*rr).sc;let mut p=core::ptr::null_mut();let mut e=0;while{p=xfs_perag_next((*sc).mp,p);!p.is_null()}{e=xrep_rtrefc_scan_ag(rr,p);if e!=0{xfs_perag_rele(p);return e}}
 xrep_rtgroup_btcur_init(sc,&mut (*sc).sr);let mut bag=core::ptr::null_mut();e=rcbag_init((*sc).mp,(*sc).xmbtp,&mut bag);if e!=0{ xchk_rtgroup_btcur_free(&mut (*sc).sr);return e}
 e=xfs_btree_goto_left_edge((*sc).sr.rmap_cur);if e==0{while xfs_btree_has_more_records((*sc).sr.rmap_cur){let mut r=core::mem::zeroed();let mut have=false;e=xrep_rtrefc_walk_rmaps(rr,&mut r,&mut have);if e!=0||!have{break}let mut sb=r.rm_startblock;let mut cb=sb;let mut nb=0;e=rcbag_next_edge(bag,(*sc).tp,&mut r,have,&mut nb);if e!=0{break}while rcbag_count(bag)>0{e=rcbag_remove_ending_at(bag,(*sc).tp,nb);if e!=0{break}e=xrep_rtrefc_walk_rmaps(rr,&mut r,&mut have);if e!=0{break}if have{e=rcbag_add(bag,(*sc).tp,&mut r);if e!=0{break}}if rcbag_count(bag)>1{e=xrep_rtrefc_stash(rr,XFS_REFC_DOMAIN_SHARED,cb,nb-cb,rcbag_count(bag));if e!=0{break}}cb=nb;if rcbag_count(bag)==0{break}sb=nb;e=rcbag_next_edge(bag,(*sc).tp,&mut r,have,&mut nb);if e!=0{break}}if e!=0{break}}}
 rcbag_free(&mut bag);xchk_rtgroup_btcur_free(&mut (*sc).sr);e
}
unsafe fn xrep_rtrefc_get_records(c:*mut xfs_btree_cur,idx:u32,_b:*mut xfs_btree_block,n:u32,p:*mut core::ffi::c_void)->u32{let rr=p as *mut xrep_rtrefc;for i in 0..n{let e=xfarray_load((*rr).refcount_records,(*rr).array_cur,&mut (*c).bc_rec.rc);if e!=0{return e as u32}(*rr).array_cur+=1;((*c).bc_ops).init_rec_from_cur(c,xfs_btree_rec_addr(c,idx+i,_b));}n}
unsafe fn xrep_rtrefc_claim_block(c:*mut xfs_btree_cur,p:*mut xfs_btree_ptr,v:*mut core::ffi::c_void)->i32{xrep_newbt_claim_block(c,&mut (*(v as *mut xrep_rtrefc)).new_btree,p)}
unsafe fn xrep_rtrefc_iroot_size(c:*mut xfs_btree_cur,l:u32,n:u32,_:*mut core::ffi::c_void)->usize{xfs_rtrefcount_broot_space_calc((*c).bc_mp,l,n)}
unsafe fn xrep_rtrefc_build_new_tree(rr:*mut xrep_rtrefc)->i32{
 let sc=(*rr).sc;let mut e=xrep_rtrefc_sort_records(rr);if e!=0{return e}e=xrep_newbt_init_metadir_inode(&mut (*rr).new_btree,sc);if e!=0{return e}
 (*rr).new_btree.bload.get_records=Some(xrep_rtrefc_get_records);(*rr).new_btree.bload.claim_block=Some(xrep_rtrefc_claim_block);(*rr).new_btree.bload.iroot_size=Some(xrep_rtrefc_iroot_size);
 let c=xfs_rtrefcountbt_init_cursor(core::ptr::null_mut(),(*sc).sr.rtg);xfs_btree_stage_ifakeroot(c,&mut (*rr).new_btree.ifake);e=xfs_btree_bload_compute_geometry(c,&mut (*rr).new_btree.bload,xfarray_length((*rr).refcount_records));if e==0&&!xchk_should_terminate(sc,&mut e){e=xfs_trans_reserve_more_inode((*sc).tp,rtg_refcount((*sc).sr.rtg),(*rr).new_btree.bload.nr_blocks,0,true)}if e==0{e=xrep_newbt_alloc_blocks(&mut (*rr).new_btree,(*rr).new_btree.bload.nr_blocks)}if e==0{(*rr).array_cur=XFARRAY_CURSOR_INIT;e=xfs_btree_bload(c,&mut (*rr).new_btree.bload,rr as _)}if e==0{xfs_rtrefcountbt_commit_staged_btree(c,(*sc).tp);xrep_inode_set_nblocks(sc,(*rr).new_btree.ifake.if_blocks);xfs_btree_del_cursor(c,0);e=xrep_newbt_commit(&mut (*rr).new_btree);if e==0{e=xrep_roll_trans(sc)}}else{xfs_btree_del_cursor(c,e);xrep_newbt_cancel(&mut (*rr).new_btree)}e
}
pub unsafe fn xrep_rtrefcountbt(sc:*mut xfs_scrub)->i32{
 let mp=(*sc).mp;if !xfs_has_rtrmapbt(mp){return -EOPNOTSUPP}let mut e=xrep_metadata_inode_forks(sc);if e!=0{return e}let rr=kzalloc_obj::<xrep_rtrefc>(XCHK_GFP_FLAGS);if rr.is_null(){return -ENOMEM}(*rr).sc=sc;e=xfarray_create("realtime reference count records\\0".as_ptr() as _,(*mp).m_sb.sb_rextents,core::mem::size_of::<xfs_refcount_irec>(),&mut (*rr).refcount_records);if e==0{xfsb_bitmap_init(&mut (*rr).old_rtrefcountbt_blocks);e=xrep_rtrefc_find_refcounts(rr)}if e==0{xfs_trans_ijoin((*sc).tp,(*sc).ip,0);e=xrep_rtrefc_build_new_tree(rr)}if e==0{e=xrep_reap_metadir_fsblocks(sc,&mut (*rr).old_rtrefcountbt_blocks)}xfsb_bitmap_destroy(&mut (*rr).old_rtrefcountbt_blocks);if !(*rr).refcount_records.is_null(){xfarray_destroy((*rr).refcount_records)}kfree(rr);e
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
