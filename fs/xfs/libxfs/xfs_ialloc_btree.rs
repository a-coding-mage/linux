// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of xfs_ialloc_btree.c; external definitions are supplied elsewhere. */

static mut XFS_INOBT_CUR_CACHE: *mut kmem_cache = core::ptr::null_mut();

unsafe fn xfs_inobt_get_minrecs(cur: *mut xfs_btree_cur, level: c_int) -> c_int {
    (*M_IGEO((*cur).bc_mp)).inobt_mnr[(level != 0) as usize]
}
unsafe fn xfs_inobt_dup_cursor(cur: *mut xfs_btree_cur) -> *mut xfs_btree_cur {
    xfs_inobt_init_cursor(to_perag((*cur).bc_group), (*cur).bc_tp, (*cur).bc_ag.agbp)
}
unsafe fn xfs_finobt_dup_cursor(cur: *mut xfs_btree_cur) -> *mut xfs_btree_cur {
    xfs_finobt_init_cursor(to_perag((*cur).bc_group), (*cur).bc_tp, (*cur).bc_ag.agbp)
}
unsafe fn xfs_inobt_set_root(cur: *mut xfs_btree_cur, nptr: *const xfs_btree_ptr, inc: c_int) {
    let agbp = (*cur).bc_ag.agbp; let agi = (*agbp).b_addr as *mut xfs_agi;
    (*agi).agi_root = (*nptr).s; be32_add_cpu(&mut (*agi).agi_level, inc);
    xfs_ialloc_log_agi((*cur).bc_tp, agbp, XFS_AGI_ROOT | XFS_AGI_LEVEL);
}
unsafe fn xfs_finobt_set_root(cur: *mut xfs_btree_cur, nptr: *const xfs_btree_ptr, inc: c_int) {
    let agbp = (*cur).bc_ag.agbp; let agi = (*agbp).b_addr as *mut xfs_agi;
    (*agi).agi_free_root = (*nptr).s; be32_add_cpu(&mut (*agi).agi_free_level, inc);
    xfs_ialloc_log_agi((*cur).bc_tp, agbp, XFS_AGI_FREE_ROOT | XFS_AGI_FREE_LEVEL);
}
unsafe fn xfs_inobt_mod_blockcount(cur: *mut xfs_btree_cur, howmuch: c_int) {
    let agbp = (*cur).bc_ag.agbp; let agi = (*agbp).b_addr as *mut xfs_agi;
    if !xfs_has_inobtcounts((*cur).bc_mp) { return; }
    if xfs_btree_is_fino((*cur).bc_ops) { be32_add_cpu(&mut (*agi).agi_fblocks, howmuch); }
    else { be32_add_cpu(&mut (*agi).agi_iblocks, howmuch); }
    xfs_ialloc_log_agi((*cur).bc_tp, agbp, XFS_AGI_IBLOCKS);
}
unsafe fn __xfs_inobt_alloc_block(cur: *mut xfs_btree_cur, start: *const xfs_btree_ptr, new: *mut xfs_btree_ptr, stat: *mut c_int, resv: xfs_ag_resv_type) -> c_int {
    let mut args: xfs_alloc_arg_t = core::mem::zeroed();
    let sbno = be32_to_cpu((*start).s);
    args.tp=(*cur).bc_tp; args.mp=(*cur).bc_mp; args.pag=to_perag((*cur).bc_group);
    args.oinfo=XFS_RMAP_OINFO_INOBT; args.minlen=1; args.maxlen=1; args.prod=1; args.resv=resv;
    let error=xfs_alloc_vextent_near_bno(&mut args, xfs_agbno_to_fsb(args.pag,sbno));
    if error != 0 { return error; }
    if args.fsbno == NULLFSBLOCK { *stat=0; return 0; }
    ASSERT(args.len == 1); (*new).s=cpu_to_be32(XFS_FSB_TO_AGBNO(args.mp,args.fsbno)); *stat=1;
    xfs_inobt_mod_blockcount(cur,1); 0
}
unsafe fn xfs_inobt_alloc_block(c:*mut xfs_btree_cur,s:*const xfs_btree_ptr,n:*mut xfs_btree_ptr,st:*mut c_int)->c_int { __xfs_inobt_alloc_block(c,s,n,st,XFS_AG_RESV_NONE) }
unsafe fn xfs_finobt_alloc_block(c:*mut xfs_btree_cur,s:*const xfs_btree_ptr,n:*mut xfs_btree_ptr,st:*mut c_int)->c_int { if (*(*c).bc_mp).m_finobt_nores { xfs_inobt_alloc_block(c,s,n,st) } else { __xfs_inobt_alloc_block(c,s,n,st,XFS_AG_RESV_METADATA) } }
unsafe fn __xfs_inobt_free_block(c:*mut xfs_btree_cur,bp:*mut xfs_buf,resv:xfs_ag_resv_type)->c_int { xfs_inobt_mod_blockcount(c,-1); let fsbno=XFS_DADDR_TO_FSB((*c).bc_mp,xfs_buf_daddr(bp)); xfs_free_extent_later((*c).bc_tp,fsbno,1,&XFS_RMAP_OINFO_INOBT,resv,0) }
unsafe fn xfs_inobt_free_block(c:*mut xfs_btree_cur,b:*mut xfs_buf)->c_int { __xfs_inobt_free_block(c,b,XFS_AG_RESV_NONE) }
unsafe fn xfs_finobt_free_block(c:*mut xfs_btree_cur,b:*mut xfs_buf)->c_int { if (*(*c).bc_mp).m_finobt_nores { xfs_inobt_free_block(c,b) } else { __xfs_inobt_free_block(c,b,XFS_AG_RESV_METADATA) } }
unsafe fn xfs_inobt_get_maxrecs(c:*mut xfs_btree_cur,l:c_int)->c_int { (*M_IGEO((*c).bc_mp)).inobt_mxr[(l!=0) as usize] }
unsafe fn xfs_inobt_init_key_from_rec(k:*mut xfs_btree_key,r:*const xfs_btree_rec) { (*k).inobt.ir_startino=(*r).inobt.ir_startino; }
unsafe fn xfs_inobt_init_high_key_from_rec(k:*mut xfs_btree_key,r:*const xfs_btree_rec) { let x=be32_to_cpu((*r).inobt.ir_startino)+XFS_INODES_PER_CHUNK-1; (*k).inobt.ir_startino=cpu_to_be32(x); }
unsafe fn xfs_inobt_init_rec_from_cur(c:*mut xfs_btree_cur,r:*mut xfs_btree_rec) { (*r).inobt.ir_startino=cpu_to_be32((*c).bc_rec.i.ir_startino); if xfs_has_sparseinodes((*c).bc_mp) { (*r).inobt.ir_u.sp.ir_holemask=cpu_to_be16((*c).bc_rec.i.ir_holemask); (*r).inobt.ir_u.sp.ir_count=(*c).bc_rec.i.ir_count; (*r).inobt.ir_u.sp.ir_freecount=(*c).bc_rec.i.ir_freecount; } else { (*r).inobt.ir_u.f.ir_freecount=cpu_to_be32((*c).bc_rec.i.ir_freecount); } (*r).inobt.ir_free=cpu_to_be64((*c).bc_rec.i.ir_free); }
unsafe fn xfs_inobt_init_ptr_from_cur(c:*mut xfs_btree_cur,p:*mut xfs_btree_ptr) { let a=(*(*c).bc_ag.agbp).b_addr as *mut xfs_agi; ASSERT((*c).bc_group.xg_gno==be32_to_cpu((*a).agi_seqno)); (*p).s=(*a).agi_root; }
unsafe fn xfs_finobt_init_ptr_from_cur(c:*mut xfs_btree_cur,p:*mut xfs_btree_ptr) { let a=(*(*c).bc_ag.agbp).b_addr as *mut xfs_agi; ASSERT((*c).bc_group.xg_gno==be32_to_cpu((*a).agi_seqno)); (*p).s=(*a).agi_free_root; }
unsafe fn xfs_inobt_cmp_key_with_cur(c:*mut xfs_btree_cur,k:*const xfs_btree_key)->c_int { cmp_int(be32_to_cpu((*k).inobt.ir_startino),(*c).bc_rec.i.ir_startino) }
unsafe fn xfs_inobt_cmp_two_keys(_: *mut xfs_btree_cur,a:*const xfs_btree_key,b:*const xfs_btree_key,m:*const xfs_btree_key)->c_int { ASSERT(m.is_null() || (*m).inobt.ir_startino!=0); cmp_int(be32_to_cpu((*a).inobt.ir_startino),be32_to_cpu((*b).inobt.ir_startino)) }
unsafe fn xfs_inobt_verify(bp:*mut xfs_buf)->xfs_failaddr_t { let mp=(*bp).b_mount; let block=XFS_BUF_TO_BLOCK(bp); if !xfs_verify_magic(bp,(*block).bb_magic) { return __this_address!(); } if xfs_has_crc(mp) { let fa=xfs_btree_agblock_v5hdr_verify(bp); if !fa.is_null(){return fa;} } let level=be16_to_cpu((*block).bb_level); if level>=(*M_IGEO(mp)).inobt_maxlevels{return __this_address!();} xfs_btree_agblock_verify(bp,(*M_IGEO(mp)).inobt_mxr[(level!=0) as usize]) }
unsafe fn xfs_inobt_read_verify(bp:*mut xfs_buf) { if !xfs_btree_agblock_verify_crc(bp) { xfs_verifier_error(bp,-EFSBADCRC,__this_address!()); } else { let fa=xfs_inobt_verify(bp); if !fa.is_null(){xfs_verifier_error(bp,-EFSCORRUPTED,fa);} } if (*bp).b_error!=0 { trace_xfs_btree_corrupt(bp,_RET_IP_); } }
unsafe fn xfs_inobt_write_verify(bp:*mut xfs_buf) { let fa=xfs_inobt_verify(bp); if !fa.is_null(){trace_xfs_btree_corrupt(bp,_RET_IP_); xfs_verifier_error(bp,-EFSCORRUPTED,fa); return;} xfs_btree_agblock_calc_crc(bp); }

// Callback tables preserve the C ABI layout and are initialized with the corresponding translated functions.
#[no_mangle] pub static mut xfs_inobt_buf_ops: xfs_buf_ops = xfs_buf_ops { name:"xfs_inobt\0", magic:[cpu_to_be32(XFS_IBT_MAGIC),cpu_to_be32(XFS_IBT_CRC_MAGIC)], verify_read:Some(xfs_inobt_read_verify), verify_write:Some(xfs_inobt_write_verify), verify_struct:Some(xfs_inobt_verify) };
#[no_mangle] pub static mut xfs_finobt_buf_ops: xfs_buf_ops = xfs_buf_ops { name:"xfs_finobt\0", magic:[cpu_to_be32(XFS_FIBT_MAGIC),cpu_to_be32(XFS_FIBT_CRC_MAGIC)], verify_read:Some(xfs_inobt_read_verify), verify_write:Some(xfs_inobt_write_verify), verify_struct:Some(xfs_inobt_verify) };

unsafe fn xfs_inobt_keys_inorder(_: *mut xfs_btree_cur,a:*const xfs_btree_key,b:*const xfs_btree_key)->bool { be32_to_cpu((*a).inobt.ir_startino)<be32_to_cpu((*b).inobt.ir_startino) }
unsafe fn xfs_inobt_recs_inorder(_: *mut xfs_btree_cur,a:*const xfs_btree_rec,b:*const xfs_btree_rec)->bool { be32_to_cpu((*a).inobt.ir_startino)+XFS_INODES_PER_CHUNK<=be32_to_cpu((*b).inobt.ir_startino) }
unsafe fn xfs_inobt_keys_contiguous(_: *mut xfs_btree_cur,a:*const xfs_btree_key,b:*const xfs_btree_key,m:*const xfs_btree_key)->xbtree_key_contig { ASSERT(m.is_null()||(*m).inobt.ir_startino!=0); xbtree_key_contig(be32_to_cpu((*a).inobt.ir_startino),be32_to_cpu((*b).inobt.ir_startino)) }

unsafe fn xfs_inobt_init_cursor(pag:*mut xfs_perag,tp:*mut xfs_trans,agbp:*mut xfs_buf)->*mut xfs_btree_cur { let mp=pag_mount(pag); let c=xfs_btree_alloc_cursor(mp,tp,&xfs_inobt_ops,(*M_IGEO(mp)).inobt_maxlevels,XFS_INOBT_CUR_CACHE); (*c).bc_group=xfs_group_hold(pag_group(pag)); (*c).bc_ag.agbp=agbp; if !agbp.is_null(){(*c).bc_nlevels=be32_to_cpu(((*agbp).b_addr as *mut xfs_agi).as_ref().unwrap().agi_level);} c }
unsafe fn xfs_finobt_init_cursor(pag:*mut xfs_perag,tp:*mut xfs_trans,agbp:*mut xfs_buf)->*mut xfs_btree_cur { let mp=pag_mount(pag); let c=xfs_btree_alloc_cursor(mp,tp,&xfs_finobt_ops,(*M_IGEO(mp)).inobt_maxlevels,XFS_INOBT_CUR_CACHE); (*c).bc_group=xfs_group_hold(pag_group(pag)); (*c).bc_ag.agbp=agbp; if !agbp.is_null(){(*c).bc_nlevels=be32_to_cpu(((*agbp).b_addr as *mut xfs_agi).as_ref().unwrap().agi_free_level);} c }

unsafe fn xfs_inobt_block_maxrecs(blocklen:c_uint,leaf:bool)->c_uint { if leaf {blocklen/(core::mem::size_of::<xfs_inobt_rec_t>() as u32)} else {blocklen/((core::mem::size_of::<xfs_inobt_key_t>()+core::mem::size_of::<xfs_inobt_ptr_t>()) as u32)} }
pub unsafe fn xfs_inobt_maxrecs(mp:*mut xfs_mount,mut blocklen:c_uint,leaf:bool)->c_uint { blocklen-=XFS_INOBT_BLOCK_LEN(mp); xfs_inobt_block_maxrecs(blocklen,leaf) }
pub unsafe fn xfs_iallocbt_maxlevels_ondisk()->c_uint { max(xfs_inobt_maxlevels_ondisk(),xfs_finobt_maxlevels_ondisk()) }
unsafe fn xfs_inobt_maxlevels_ondisk()->c_uint { let b=min(XFS_MIN_BLOCKSIZE-XFS_BTREE_SBLOCK_LEN,XFS_MIN_CRC_BLOCKSIZE-XFS_BTREE_SBLOCK_CRC_LEN); let m=[xfs_inobt_block_maxrecs(b,true)/2,xfs_inobt_block_maxrecs(b,false)/2]; xfs_btree_compute_maxlevels(m.as_ptr(),XFS_MAX_INODE_RECORDS) }
unsafe fn xfs_finobt_maxlevels_ondisk()->c_uint { let b=XFS_MIN_CRC_BLOCKSIZE-XFS_BTREE_SBLOCK_CRC_LEN; let m=[xfs_inobt_block_maxrecs(b,true)/2,xfs_inobt_block_maxrecs(b,false)/2]; xfs_btree_compute_maxlevels(m.as_ptr(),XFS_MAX_INODE_RECORDS) }

pub unsafe fn xfs_inobt_irec_to_allocmask(rec:*const xfs_inobt_rec_incore)->u64 { let mut bitmap=0u64; let inodespbit=(1u64<<XFS_INODES_PER_HOLEMASK_BIT)-1; let mut allocbitmap=(!(*rec).ir_holemask)&((1u32<<XFS_INOBT_HOLEMASK_BITS)-1); let mut next=xfs_next_bit(&mut allocbitmap,1,0); while next!=-1 { ASSERT(next<(core::mem::size_of::<u16>()*NBBY) as c_int); bitmap|=inodespbit<<(next*XFS_INODES_PER_HOLEMASK_BIT); next=xfs_next_bit(&mut allocbitmap,1,next+1); } bitmap }
pub unsafe fn xfs_iallocbt_calc_size(mp:*mut xfs_mount,len:c_ulonglong)->xfs_extlen_t { xfs_btree_calc_size((*M_IGEO(mp)).inobt_mnr,len) }
pub unsafe fn xfs_inobt_commit_staged_btree(cur:*mut xfs_btree_cur,tp:*mut xfs_trans,agbp:*mut xfs_buf) { let agi=(*agbp).b_addr as *mut xfs_agi; let af=(*cur).bc_ag.afake; let mut fields; ASSERT((*cur).bc_flags&XFS_BTREE_STAGING!=0); if xfs_btree_is_ino((*cur).bc_ops){fields=XFS_AGI_ROOT|XFS_AGI_LEVEL;(*agi).agi_root=cpu_to_be32((*af).af_root);(*agi).agi_level=cpu_to_be32((*af).af_levels);if xfs_has_inobtcounts((*cur).bc_mp){(*agi).agi_iblocks=cpu_to_be32((*af).af_blocks);fields|=XFS_AGI_IBLOCKS;}xfs_ialloc_log_agi(tp,agbp,fields);xfs_btree_commit_afakeroot(cur,tp,agbp);}else{fields=XFS_AGI_FREE_ROOT|XFS_AGI_FREE_LEVEL;(*agi).agi_free_root=cpu_to_be32((*af).af_root);(*agi).agi_free_level=cpu_to_be32((*af).af_levels);if xfs_has_inobtcounts((*cur).bc_mp){(*agi).agi_fblocks=cpu_to_be32((*af).af_blocks);fields|=XFS_AGI_IBLOCKS;}xfs_ialloc_log_agi(tp,agbp,fields);xfs_btree_commit_afakeroot(cur,tp,agbp);} }
pub unsafe fn xfs_finobt_calc_reserves(pag:*mut xfs_perag,tp:*mut xfs_trans,ask:*mut xfs_extlen_t,used:*mut xfs_extlen_t)->c_int { if !xfs_has_finobt(pag_mount(pag)){return 0;} let mut agbp=core::ptr::null_mut(); let mut tree=0; let e=xfs_ialloc_read_agi(pag,tp,0,&mut agbp); if e!=0{return e;} if xfs_has_inobtcounts(pag_mount(pag)){tree=be32_to_cpu(((*agbp).b_addr as *mut xfs_agi).as_ref().unwrap().agi_fblocks);}else{let c=xfs_finobt_init_cursor(pag,tp,agbp);let e=xfs_btree_count_blocks(c,&mut tree);xfs_btree_del_cursor(c,e);if e!=0{xfs_trans_brelse(tp,agbp);return e;}}xfs_trans_brelse(tp,agbp);*ask+=xfs_inobt_max_size(pag);*used+=tree;0 }
unsafe fn xfs_inobt_max_size(pag:*mut xfs_perag)->xfs_extlen_t { let mp=pag_mount(pag);let mut blocks=pag_group(pag).xg_block_count;if (*M_IGEO(mp)).inobt_mxr[0]==0{return 0;}if xfs_ag_contains_log(mp,pag_agno(pag)){blocks-=(*mp).m_sb.sb_logblocks;}xfs_btree_calc_size((*M_IGEO(mp)).inobt_mnr,(blocks as u64)*(*mp).m_sb.sb_inopblock/XFS_INODES_PER_CHUNK) }
pub unsafe fn xfs_inobt_init_cur_cache()->c_int { XFS_INOBT_CUR_CACHE=kmem_cache_create("xfs_inobt_cur\0",xfs_btree_cur_sizeof(xfs_inobt_maxlevels_ondisk()),0,0,core::ptr::null_mut()); if XFS_INOBT_CUR_CACHE.is_null(){-ENOMEM}else{0} }
pub unsafe fn xfs_inobt_destroy_cur_cache(){ kmem_cache_destroy(XFS_INOBT_CUR_CACHE); XFS_INOBT_CUR_CACHE=core::ptr::null_mut(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
