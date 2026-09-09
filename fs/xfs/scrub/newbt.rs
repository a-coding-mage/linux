// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2022-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* Translated from newbt.c; declarations supplied by the surrounding XFS codebase are external. */

const XREP_MAX_ITRUNCATE_EFIS: u32 = 128;

unsafe fn xrep_newbt_estimate_slack(xnr: *mut xrep_newbt) {
    let sc = (*xnr).sc;
    let bload = &mut (*xnr).bload;
    let (free, sz): (u64, u64);

    bload.leaf_slack = xfs_globals.bload_leaf_slack;
    bload.node_slack = xfs_globals.bload_node_slack;

    if (*(*sc).ops).type_ == ST_PERAG {
        free = (*(*sc).sa.pag).pagf_freeblks;
        sz = xfs_ag_block_count((*sc).mp, pag_agno((*sc).sa.pag));
    } else {
        free = xfs_sum_freecounter_raw((*sc).mp, XC_FREE_BLOCKS);
        sz = (*(*sc).mp).m_sb.sb_dblocks;
    }

    if free >= sz / 10 { return; }
    if bload.leaf_slack < 0 { bload.leaf_slack = 2; }
    if bload.node_slack < 0 { bload.node_slack = 2; }
}

pub unsafe fn xrep_newbt_init_ag(xnr: *mut xrep_newbt, sc: *mut xfs_scrub,
        oinfo: *const xfs_owner_info, alloc_hint: xfs_fsblock_t,
        resv: xfs_ag_resv_type) {
    core::ptr::write_bytes(xnr, 0, 1);
    (*xnr).sc = sc;
    (*xnr).oinfo = *oinfo;
    (*xnr).alloc_hint = alloc_hint;
    (*xnr).resv = resv;
    INIT_LIST_HEAD(&mut (*xnr).resv_list);
    (*xnr).bload.max_dirty = XFS_B_TO_FSBT((*sc).mp, 256u32 << 10);
    xrep_newbt_estimate_slack(xnr);
}

pub unsafe fn xrep_newbt_init_inode(xnr: *mut xrep_newbt, sc: *mut xfs_scrub,
        whichfork: i32, oinfo: *const xfs_owner_info) -> i32 {
    let ifp = kmem_cache_zalloc(xfs_ifork_cache, XCHK_GFP_FLAGS);
    if ifp.is_null() { return -ENOMEM; }
    xrep_newbt_init_ag(xnr, sc, oinfo, XFS_INODE_TO_FSB((*sc).ip), XFS_AG_RESV_NONE);
    (*xnr).ifake.if_fork = ifp;
    (*xnr).ifake.if_fork_size = xfs_inode_fork_size((*sc).ip, whichfork);
    0
}

pub unsafe fn xrep_newbt_init_metadir_inode(xnr: *mut xrep_newbt, sc: *mut xfs_scrub) -> i32 {
    let mut oinfo: xfs_owner_info = core::mem::zeroed();
    ASSERT(xfs_is_metadir_inode((*sc).ip));
    xfs_rmap_inode_bmbt_owner(&mut oinfo, (*sc).ip, XFS_DATA_FORK);
    let ifp = kmem_cache_zalloc(xfs_ifork_cache, XCHK_GFP_FLAGS);
    if ifp.is_null() { return -ENOMEM; }
    xrep_newbt_init_ag(xnr, sc, &oinfo, XFS_INODE_TO_FSB((*sc).ip), XFS_AG_RESV_NONE);
    (*xnr).ifake.if_fork = ifp;
    (*xnr).ifake.if_fork_size = xfs_inode_fork_size((*sc).ip, XFS_DATA_FORK);
    0
}

pub unsafe fn xrep_newbt_init_bare(xnr: *mut xrep_newbt, sc: *mut xfs_scrub) {
    xrep_newbt_init_ag(xnr, sc, &XFS_RMAP_OINFO_ANY_OWNER, NULLFSBLOCK, XFS_AG_RESV_NONE);
}

unsafe fn xrep_newbt_add_blocks(xnr: *mut xrep_newbt, pag: *mut xfs_perag,
        args: *const xfs_alloc_arg) -> i32 {
    let resv = kmalloc_obj::<xrep_newbt_resv>(XCHK_GFP_FLAGS);
    if resv.is_null() { return -ENOMEM; }
    INIT_LIST_HEAD(&mut (*resv).list);
    (*resv).agbno = XFS_FSB_TO_AGBNO((*(*xnr).sc).mp, (*args).fsbno);
    (*resv).len = (*args).len;
    (*resv).used = 0;
    (*resv).pag = xfs_perag_hold(pag);
    if !(*args).tp.is_null() {
        ASSERT((*xnr).oinfo.oi_offset == 0);
        let error = xfs_alloc_schedule_autoreap(args, XFS_FREE_EXTENT_SKIP_DISCARD, &mut (*resv).autoreap);
        if error != 0 { xfs_perag_put((*resv).pag); kfree(resv); return error; }
    }
    list_add_tail(&mut (*resv).list, &mut (*xnr).resv_list);
    0
}

pub unsafe fn xrep_newbt_add_extent(xnr: *mut xrep_newbt, pag: *mut xfs_perag,
        agbno: xfs_agblock_t, len: xfs_extlen_t) -> i32 {
    let args = xfs_alloc_arg { tp: core::ptr::null_mut(), oinfo: (*xnr).oinfo,
        fsbno: xfs_agbno_to_fsb(pag, agbno), len, resv: (*xnr).resv, ..core::mem::zeroed() };
    xrep_newbt_add_blocks(xnr, pag, &args)
}

unsafe fn xrep_newbt_validate_ag_alloc_hint(xnr: *mut xrep_newbt) {
    let sc = (*xnr).sc;
    let agno = XFS_FSB_TO_AGNO((*sc).mp, (*xnr).alloc_hint);
    if agno == pag_agno((*sc).sa.pag) && xfs_verify_fsbno((*sc).mp, (*xnr).alloc_hint) { return; }
    (*xnr).alloc_hint = xfs_agbno_to_fsb((*sc).sa.pag, XFS_AGFL_BLOCK((*sc).mp) + 1);
}

unsafe fn xrep_newbt_validate_file_alloc_hint(xnr: *mut xrep_newbt) {
    let sc = (*xnr).sc;
    if xfs_verify_fsbno((*sc).mp, (*xnr).alloc_hint) { return; }
    (*xnr).alloc_hint = XFS_AGB_TO_FSB((*sc).mp, 0, XFS_AGFL_BLOCK((*sc).mp) + 1);
}

unsafe fn xrep_newbt_alloc_ag_blocks(xnr: *mut xrep_newbt, mut nr_blocks: u64) -> i32 {
    let sc = (*xnr).sc; let mp = (*sc).mp;
    ASSERT(!(*sc).sa.pag.is_null()); ASSERT((*xnr).resv != XFS_AG_RESV_METAFILE);
    while nr_blocks > 0 {
        let mut args: xfs_alloc_arg = core::mem::zeroed();
        args.tp = (*sc).tp; args.mp = mp; args.oinfo = (*xnr).oinfo; args.minlen = 1;
        args.maxlen = nr_blocks; args.prod = 1; args.resv = (*xnr).resv;
        xrep_newbt_validate_ag_alloc_hint(xnr);
        let error = if let Some(f) = (*xnr).alloc_vextent { f(sc, &mut args, (*xnr).alloc_hint) }
            else { xfs_alloc_vextent_near_bno(&mut args, (*xnr).alloc_hint) };
        if error != 0 { return error; }
        if args.fsbno == NULLFSBLOCK { return -ENOSPC; }
        if XFS_FSB_TO_AGNO(mp, args.fsbno) != pag_agno((*sc).sa.pag) { ASSERT(false); return -EFSCORRUPTED; }
        trace_xrep_newbt_alloc_ag_blocks((*sc).sa.pag, XFS_FSB_TO_AGBNO(mp, args.fsbno), args.len, (*xnr).oinfo.oi_owner);
        let error = xrep_newbt_add_blocks(xnr, (*sc).sa.pag, &args); if error != 0 { return error; }
        nr_blocks -= args.len; (*xnr).alloc_hint = args.fsbno + args.len;
        let error = xrep_defer_finish(sc); if error != 0 { return error; }
    } 0
}

unsafe fn xrep_newbt_alloc_file_blocks(xnr: *mut xrep_newbt, mut nr_blocks: u64) -> i32 {
    let sc = (*xnr).sc; let mp = (*sc).mp; ASSERT((*xnr).resv != XFS_AG_RESV_METAFILE);
    while nr_blocks > 0 {
        let mut args: xfs_alloc_arg = core::mem::zeroed(); args.tp=(*sc).tp; args.mp=mp; args.oinfo=(*xnr).oinfo;
        args.minlen=1; args.maxlen=nr_blocks; args.prod=1; args.resv=(*xnr).resv;
        xrep_newbt_validate_file_alloc_hint(xnr);
        let error = if let Some(f)=(*xnr).alloc_vextent { f(sc,&mut args,(*xnr).alloc_hint) } else { xfs_alloc_vextent_start_ag(&mut args,(*xnr).alloc_hint) };
        if error != 0 { return error; } if args.fsbno==NULLFSBLOCK { return -ENOSPC; }
        let pag=xfs_perag_get(mp,XFS_FSB_TO_AGNO(mp,args.fsbno)); if pag.is_null() { ASSERT(false); return -EFSCORRUPTED; }
        trace_xrep_newbt_alloc_file_blocks(pag,XFS_FSB_TO_AGBNO(mp,args.fsbno),args.len,(*xnr).oinfo.oi_owner);
        let error=xrep_newbt_add_blocks(xnr,pag,&args); xfs_perag_put(pag); if error!=0{return error;}
        nr_blocks-=args.len; (*xnr).alloc_hint=args.fsbno+args.len;
        let error=xrep_defer_finish(sc); if error!=0{return error;}
    } 0
}

pub unsafe fn xrep_newbt_alloc_blocks(xnr: *mut xrep_newbt, nr_blocks: u64) -> i32 {
    if !(*xnr).sc.ip.is_null() { xrep_newbt_alloc_file_blocks(xnr,nr_blocks) } else { xrep_newbt_alloc_ag_blocks(xnr,nr_blocks) }
}

unsafe fn xrep_newbt_free_extent(xnr:*mut xrep_newbt,resv:*mut xrep_newbt_resv,btree_committed:bool)->i32 {
    let sc=(*xnr).sc; let mut free_agbno=(*resv).agbno; let mut free_aglen=(*resv).len;
    if !btree_committed || (*resv).used==0 { trace_xrep_newbt_free_blocks((*resv).pag,free_agbno,free_aglen,(*xnr).oinfo.oi_owner); xfs_alloc_commit_autoreap((*sc).tp,&mut (*resv).autoreap); return 1; }
    xfs_alloc_cancel_autoreap((*sc).tp,&mut (*resv).autoreap); free_agbno+=(*resv).used; free_aglen-=(*resv).used; if free_aglen==0{return 0;}
    trace_xrep_newbt_free_blocks((*resv).pag,free_agbno,free_aglen,(*xnr).oinfo.oi_owner); ASSERT((*xnr).resv!=XFS_AG_RESV_AGFL); ASSERT((*xnr).resv!=XFS_AG_RESV_IGNORE);
    let error=xfs_free_extent_later((*sc).tp,xfs_agbno_to_fsb((*resv).pag,free_agbno),free_aglen,&(*xnr).oinfo,(*xnr).resv,XFS_FREE_EXTENT_SKIP_DISCARD); if error!=0{return error;} 1
}

unsafe fn xrep_newbt_free(xnr:*mut xrep_newbt,btree_committed:bool)->i32 {
    let sc=(*xnr).sc; let mut freed=0u32; let mut error=0;
    if !xfs_is_shutdown((*sc).mp) {
        let mut resv=(*xnr).resv_list.next; while resv != &mut (*xnr).resv_list as *mut _ { let next=(*resv).next; let ret=xrep_newbt_free_extent(xnr,resv as *mut xrep_newbt_resv,btree_committed); list_del(resv); xfs_perag_put((*resv.as_mut()).pag); kfree(resv as *mut xrep_newbt_resv); if ret<0{error=ret;break;} freed+=ret as u32; if freed>=XREP_MAX_ITRUNCATE_EFIS{error=xrep_defer_finish(sc);if error!=0{break;}freed=0;} resv=next; }
        if error==0 && freed!=0 {error=xrep_defer_finish(sc);}
    }
    while (*xnr).resv_list.next != &mut (*xnr).resv_list as *mut _ { let resv=(*xnr).resv_list.next; xfs_alloc_commit_autoreap((*sc).tp,&mut (*(resv as *mut xrep_newbt_resv)).autoreap); list_del(resv); xfs_perag_put((*(resv as *mut xrep_newbt_resv)).pag); kfree(resv as *mut xrep_newbt_resv); }
    if !(*sc).ip.is_null(){kmem_cache_free(xfs_ifork_cache,(*xnr).ifake.if_fork);(*xnr).ifake.if_fork=core::ptr::null_mut();} error
}

pub unsafe fn xrep_newbt_commit(xnr:*mut xrep_newbt)->i32{xrep_newbt_free(xnr,true)}
pub unsafe fn xrep_newbt_cancel(xnr:*mut xrep_newbt){xrep_newbt_free(xnr,false);}

pub unsafe fn xrep_newbt_claim_block(cur:*mut xfs_btree_cur,xnr:*mut xrep_newbt,ptr:*mut xfs_btree_ptr)->i32{
    let resv=(*xnr).resv_list.next as *mut xrep_newbt_resv; if (*resv).used==(*resv).len{return -ENOSPC;}
    let agbno=(*resv).agbno+(*resv).used;(*resv).used+=1;if (*resv).used==(*resv).len{list_move_tail(&mut (*resv).list,&mut (*xnr).resv_list);}
    trace_xrep_newbt_claim_block((*resv).pag,agbno,1,(*xnr).oinfo.oi_owner);
    if (*(*cur).bc_ops).ptr_len==XFS_BTREE_LONG_PTR_LEN{(*ptr).l=cpu_to_be64(xfs_agbno_to_fsb((*resv).pag,agbno));}else{(*ptr).s=cpu_to_be32(agbno);}
    xrep_defer_finish((*xnr).sc)
}

pub unsafe fn xrep_newbt_unused_blocks(xnr:*mut xrep_newbt)->u32{let mut unused=0;let mut p=(*xnr).resv_list.next;while p!=&mut (*xnr).resv_list as *mut _{let r=p as *mut xrep_newbt_resv;unused+=(*r).len-(*r).used;p=(*p).next;}unused}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
