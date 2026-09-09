// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2014 Red Hat, Inc.
 * All Rights Reserved.
 */

// Dependencies are supplied by the surrounding XFS Rust translation.

static mut xfs_rmapbt_cur_cache: *mut kmem_cache = core::ptr::null_mut();

static mut xfs_rmapbt_buf_ops: xfs_buf_ops = xfs_buf_ops {
    name: "xfs_rmapbt", magic: [0, cpu_to_be32(XFS_RMAP_CRC_MAGIC)],
    verify_read: xfs_rmapbt_read_verify, verify_write: xfs_rmapbt_write_verify,
    verify_struct: xfs_rmapbt_verify,
};

unsafe fn xfs_rmapbt_dup_cursor(cur: *mut xfs_btree_cur) -> *mut xfs_btree_cur {
    xfs_rmapbt_init_cursor((*cur).bc_mp, (*cur).bc_tp, (*cur).bc_ag.agbp,
                           to_perag((*cur).bc_group))
}

unsafe fn xfs_rmapbt_set_root(cur: *mut xfs_btree_cur, ptr: *const xfs_btree_ptr, inc: i32) {
    let agbp = (*cur).bc_ag.agbp;
    let agf = (*agbp).b_addr as *mut xfs_agf;
    let pag = to_perag((*cur).bc_group);
    ASSERT((*ptr).s != 0);
    (*agf).agf_rmap_root = (*ptr).s;
    be32_add_cpu(&mut (*agf).agf_rmap_level, inc);
    (*pag).pagf_rmap_level += inc as u32;
    xfs_alloc_log_agf((*cur).bc_tp, agbp, XFS_AGF_ROOTS | XFS_AGF_LEVELS);
}

unsafe fn xfs_rmapbt_alloc_block(cur: *mut xfs_btree_cur, _start: *const xfs_btree_ptr,
                                 new: *mut xfs_btree_ptr, stat: *mut i32) -> i32 {
    let agbp = (*cur).bc_ag.agbp;
    let agf = (*agbp).b_addr as *mut xfs_agf;
    let pag = to_perag((*cur).bc_group);
    let mut args = xfs_alloc_arg { len: 1, ..core::mem::zeroed() };
    let mut bno: xfs_agblock_t = 0;
    let error = xfs_alloc_get_freelist(pag, (*cur).bc_tp, agbp, &mut bno, 1);
    if error != 0 { return error; }
    if bno == NULLAGBLOCK { *stat = 0; return 0; }
    xfs_extent_busy_reuse(pag_group(pag), bno, 1, false);
    (*new).s = cpu_to_be32(bno);
    be32_add_cpu(&mut (*agf).agf_rmap_blocks, 1);
    xfs_alloc_log_agf((*cur).bc_tp, agbp, XFS_AGF_RMAP_BLOCKS);
    xfs_ag_resv_alloc_extent(pag, XFS_AG_RESV_RMAPBT, &mut args);
    *stat = 1;
    0
}

unsafe fn xfs_rmapbt_free_block(cur: *mut xfs_btree_cur, bp: *mut xfs_buf) -> i32 {
    let agbp = (*cur).bc_ag.agbp;
    let agf = (*agbp).b_addr as *mut xfs_agf;
    let pag = to_perag((*cur).bc_group);
    let bno = xfs_daddr_to_agbno((*cur).bc_mp, xfs_buf_daddr(bp));
    be32_add_cpu(&mut (*agf).agf_rmap_blocks, -1);
    xfs_alloc_log_agf((*cur).bc_tp, agbp, XFS_AGF_RMAP_BLOCKS);
    let error = xfs_alloc_put_freelist(pag, (*cur).bc_tp, agbp, core::ptr::null_mut(), bno, 1);
    if error != 0 { return error; }
    xfs_extent_busy_insert((*cur).bc_tp, pag_group(pag), bno, 1, XFS_EXTENT_BUSY_SKIP_DISCARD);
    xfs_ag_resv_free_extent(pag, XFS_AG_RESV_RMAPBT, core::ptr::null_mut(), 1);
    0
}

unsafe fn xfs_rmapbt_get_minrecs(cur: *mut xfs_btree_cur, level: i32) -> i32 {
    (*(*cur).bc_mp).m_rmap_mnr[(level != 0) as usize]
}
unsafe fn xfs_rmapbt_get_maxrecs(cur: *mut xfs_btree_cur, level: i32) -> i32 {
    (*(*cur).bc_mp).m_rmap_mxr[(level != 0) as usize]
}

#[inline]
unsafe fn ondisk_rec_offset_to_key(rec: *const xfs_btree_rec) -> __be64 {
    (*rec).rmap.rm_offset & !cpu_to_be64(XFS_RMAP_OFF_UNWRITTEN)
}

unsafe fn xfs_rmapbt_init_key_from_rec(key: *mut xfs_btree_key, rec: *const xfs_btree_rec) {
    (*key).rmap.rm_startblock = (*rec).rmap.rm_startblock;
    (*key).rmap.rm_owner = (*rec).rmap.rm_owner;
    (*key).rmap.rm_offset = ondisk_rec_offset_to_key(rec);
}

unsafe fn xfs_rmapbt_init_high_key_from_rec(key: *mut xfs_btree_key, rec: *const xfs_btree_rec) {
    let adj = be32_to_cpu((*rec).rmap.rm_blockcount) - 1;
    (*key).rmap.rm_startblock = (*rec).rmap.rm_startblock;
    be32_add_cpu(&mut (*key).rmap.rm_startblock, adj as i32);
    (*key).rmap.rm_owner = (*rec).rmap.rm_owner;
    (*key).rmap.rm_offset = ondisk_rec_offset_to_key(rec);
    if XFS_RMAP_NON_INODE_OWNER(be64_to_cpu((*rec).rmap.rm_owner)) ||
       XFS_RMAP_IS_BMBT_BLOCK(be64_to_cpu((*rec).rmap.rm_offset)) { return; }
    let off = be64_to_cpu((*key).rmap.rm_offset);
    (*key).rmap.rm_offset = cpu_to_be64((XFS_RMAP_OFF(off) + adj as u64) | (off & !XFS_RMAP_OFF_MASK));
}

unsafe fn xfs_rmapbt_init_rec_from_cur(cur: *mut xfs_btree_cur, rec: *mut xfs_btree_rec) {
    (*rec).rmap.rm_startblock = cpu_to_be32((*cur).bc_rec.r.rm_startblock);
    (*rec).rmap.rm_blockcount = cpu_to_be32((*cur).bc_rec.r.rm_blockcount);
    (*rec).rmap.rm_owner = cpu_to_be64((*cur).bc_rec.r.rm_owner);
    (*rec).rmap.rm_offset = cpu_to_be64(xfs_rmap_irec_offset_pack(&(*cur).bc_rec.r));
}

unsafe fn xfs_rmapbt_init_ptr_from_cur(cur: *mut xfs_btree_cur, ptr: *mut xfs_btree_ptr) {
    let agf = (*(*cur).bc_ag.agbp).b_addr as *mut xfs_agf;
    ASSERT((*(*cur).bc_group).xg_gno == be32_to_cpu((*agf).agf_seqno));
    (*ptr).s = (*agf).agf_rmap_root;
}

#[inline] unsafe fn offset_keymask(offset: u64) -> u64 { offset & !XFS_RMAP_OFF_UNWRITTEN }

unsafe fn xfs_rmapbt_cmp_key_with_cur(cur: *mut xfs_btree_cur, key: *const xfs_btree_key) -> i32 {
    let rec = &(*cur).bc_rec.r;
    let kp = &(*key).rmap;
    cmp_int(be32_to_cpu(kp.rm_startblock), rec.rm_startblock).or_else(|| cmp_int(be64_to_cpu(kp.rm_owner), rec.rm_owner)).unwrap_or_else(|| cmp_int(offset_keymask(be64_to_cpu(kp.rm_offset)), offset_keymask(xfs_rmap_irec_offset_pack(rec))))
}

unsafe fn xfs_rmapbt_cmp_two_keys(_cur: *mut xfs_btree_cur, k1: *const xfs_btree_key,
                                  k2: *const xfs_btree_key, mask: *const xfs_btree_key) -> i32 {
    ASSERT(mask.is_null() || (*mask).rmap.rm_startblock != 0);
    let a = &(*k1).rmap; let b = &(*k2).rmap;
    let mut d = cmp_int(be32_to_cpu(a.rm_startblock), be32_to_cpu(b.rm_startblock));
    if d != 0 { return d; }
    if mask.is_null() || (*mask).rmap.rm_owner != 0 {
        d = cmp_int(be64_to_cpu(a.rm_owner), be64_to_cpu(b.rm_owner)); if d != 0 { return d; }
    }
    if mask.is_null() || (*mask).rmap.rm_offset != 0 {
        ASSERT(mask.is_null() || (*mask).rmap.rm_owner != 0);
        d = cmp_int(offset_keymask(be64_to_cpu(a.rm_offset)), offset_keymask(be64_to_cpu(b.rm_offset)));
    }
    d
}

unsafe fn xfs_rmapbt_verify(bp: *mut xfs_buf) -> xfs_failaddr_t {
    let mp = (*bp).b_mount; let block = XFS_BUF_TO_BLOCK(bp); let pag = (*bp).b_pag;
    if !xfs_verify_magic(bp, (*block).bb_magic) || !xfs_has_rmapbt(mp) { return __this_address; }
    let fa = xfs_btree_agblock_v5hdr_verify(bp); if !fa.is_null() { return fa; }
    let level = be16_to_cpu((*block).bb_level) as usize;
    if !pag.is_null() && xfs_perag_initialised_agf(pag) {
        if level >= (*pag).pagf_rmap_level as usize { return __this_address; }
    } else if level >= (*mp).m_rmap_maxlevels as usize { return __this_address; }
    xfs_btree_agblock_verify(bp, (*mp).m_rmap_mxr[level != 0])
}

unsafe fn xfs_rmapbt_read_verify(bp: *mut xfs_buf) {
    if !xfs_btree_agblock_verify_crc(bp) { xfs_verifier_error(bp, -EFSBADCRC, __this_address); }
    else { let fa = xfs_rmapbt_verify(bp); if !fa.is_null() { xfs_verifier_error(bp, -EFSCORRUPTED, fa); } }
    if (*bp).b_error != 0 { trace_xfs_btree_corrupt(bp, _RET_IP_); }
}
unsafe fn xfs_rmapbt_write_verify(bp: *mut xfs_buf) {
    let fa = xfs_rmapbt_verify(bp); if !fa.is_null() { trace_xfs_btree_corrupt(bp, _RET_IP_); xfs_verifier_error(bp, -EFSCORRUPTED, fa); return; }
    xfs_btree_agblock_calc_crc(bp);
}

unsafe fn xfs_rmapbt_keys_inorder(_cur: *mut xfs_btree_cur, k1: *const xfs_btree_key, k2: *const xfs_btree_key) -> i32 {
    let a=&(*k1).rmap; let b=&(*k2).rmap;
    if be32_to_cpu(a.rm_startblock) < be32_to_cpu(b.rm_startblock) { return 1; }
    if be32_to_cpu(a.rm_startblock) > be32_to_cpu(b.rm_startblock) { return 0; }
    if be64_to_cpu(a.rm_owner) < be64_to_cpu(b.rm_owner) { return 1; }
    if be64_to_cpu(a.rm_owner) > be64_to_cpu(b.rm_owner) { return 0; }
    (offset_keymask(be64_to_cpu(a.rm_offset)) <= offset_keymask(be64_to_cpu(b.rm_offset))) as i32
}
unsafe fn xfs_rmapbt_recs_inorder(_cur:*mut xfs_btree_cur,r1:*const xfs_btree_rec,r2:*const xfs_btree_rec)->i32 {
    let a=&(*r1).rmap; let b=&(*r2).rmap;
    if be32_to_cpu(a.rm_startblock) != be32_to_cpu(b.rm_startblock) { return (be32_to_cpu(a.rm_startblock)<be32_to_cpu(b.rm_startblock)) as i32; }
    if be64_to_cpu(a.rm_owner) != be64_to_cpu(b.rm_owner) { return (be64_to_cpu(a.rm_owner)<be64_to_cpu(b.rm_owner)) as i32; }
    (offset_keymask(be64_to_cpu(a.rm_offset)) <= offset_keymask(be64_to_cpu(b.rm_offset))) as i32
}
unsafe fn xfs_rmapbt_keys_contiguous(_cur:*mut xfs_btree_cur,k1:*const xfs_btree_key,k2:*const xfs_btree_key,mask:*const xfs_btree_key)->xbtree_key_contig {
    ASSERT(mask.is_null() || (*mask).rmap.rm_startblock != 0);
    ASSERT(mask.is_null() || ((*mask).rmap.rm_owner == 0 && (*mask).rmap.rm_offset == 0));
    xbtree_key_contig(be32_to_cpu((*k1).rmap.rm_startblock),be32_to_cpu((*k2).rmap.rm_startblock))
}

pub static mut xfs_rmapbt_ops: xfs_btree_ops = xfs_btree_ops {
    name:"rmap", type_:XFS_BTREE_TYPE_AG, geom_flags:XFS_BTGEO_OVERLAPPING,
    rec_len:core::mem::size_of::<xfs_rmap_rec>(), key_len:2*core::mem::size_of::<xfs_rmap_key>(), ptr_len:XFS_BTREE_SHORT_PTR_LEN,
    lru_refs:XFS_RMAP_BTREE_REF, statoff:XFS_STATS_CALC_INDEX(xs_rmap_2), sick_mask:XFS_SICK_AG_RMAPBT,
    dup_cursor:xfs_rmapbt_dup_cursor,set_root:xfs_rmapbt_set_root,alloc_block:xfs_rmapbt_alloc_block,free_block:xfs_rmapbt_free_block,
    get_minrecs:xfs_rmapbt_get_minrecs,get_maxrecs:xfs_rmapbt_get_maxrecs,init_key_from_rec:xfs_rmapbt_init_key_from_rec,
    init_high_key_from_rec:xfs_rmapbt_init_high_key_from_rec,init_rec_from_cur:xfs_rmapbt_init_rec_from_cur,init_ptr_from_cur:xfs_rmapbt_init_ptr_from_cur,
    cmp_key_with_cur:xfs_rmapbt_cmp_key_with_cur,buf_ops:&mut xfs_rmapbt_buf_ops,cmp_two_keys:xfs_rmapbt_cmp_two_keys,
    keys_inorder:xfs_rmapbt_keys_inorder,recs_inorder:xfs_rmapbt_recs_inorder,keys_contiguous:xfs_rmapbt_keys_contiguous,
};

pub unsafe fn xfs_rmapbt_init_cursor(mp:*mut xfs_mount,tp:*mut xfs_trans,agbp:*mut xfs_buf,pag:*mut xfs_perag)->*mut xfs_btree_cur {
    let cur=xfs_btree_alloc_cursor(mp,tp,&xfs_rmapbt_ops,(*mp).m_rmap_maxlevels,xfs_rmapbt_cur_cache);
    (*cur).bc_group=xfs_group_hold(pag_group(pag)); (*cur).bc_ag.agbp=agbp;
    if !agbp.is_null() { (*cur).bc_nlevels=be32_to_cpu((*(*agbp).b_addr.cast::<xfs_agf>()).agf_rmap_level); } cur
}

unsafe fn xfs_rmapbt_block_maxrecs(blocklen:usize,leaf:bool)->usize { if leaf { blocklen/core::mem::size_of::<xfs_rmap_rec>() } else { blocklen/(2*core::mem::size_of::<xfs_rmap_key>()+core::mem::size_of::<xfs_rmap_ptr_t>()) } }
pub unsafe fn xfs_rmapbt_maxrecs(_mp:*mut xfs_mount,mut blocklen:usize,leaf:bool)->usize { blocklen-=XFS_RMAP_BLOCK_LEN; xfs_rmapbt_block_maxrecs(blocklen,leaf) }
pub unsafe fn xfs_rmapbt_maxlevels_ondisk()->u32 { let l=XFS_MIN_CRC_BLOCKSIZE-XFS_BTREE_SBLOCK_CRC_LEN; let m=[xfs_rmapbt_block_maxrecs(l,true)/2,xfs_rmapbt_block_maxrecs(l,false)/2]; max(xfs_btree_space_to_height(m,XFS_MAX_CRC_AG_BLOCKS),0) }
#[cfg(CONFIG_XFS_BTREE_IN_MEM)]
unsafe fn xfs_rmapbt_mem_block_maxrecs(blocklen:usize,leaf:bool)->usize { if leaf { blocklen/core::mem::size_of::<xfs_rmap_rec>() } else { blocklen/(2*core::mem::size_of::<xfs_rmap_key>()+core::mem::size_of::<__be64>()) } }
#[cfg(CONFIG_XFS_BTREE_IN_MEM)]
unsafe fn xfs_rmapbt_mem_verify(bp:*mut xfs_buf)->xfs_failaddr_t { let block=XFS_BUF_TO_BLOCK(bp); if !xfs_verify_magic(bp,(*block).bb_magic){return __this_address;} let fa=xfs_btree_fsblock_v5hdr_verify(bp,XFS_RMAP_OWN_UNKNOWN); if !fa.is_null(){return fa;} let level=be16_to_cpu((*block).bb_level) as usize; if level>=xfs_rmapbt_maxlevels_ondisk() as usize{return __this_address;} xfs_btree_memblock_verify(bp,xfs_rmapbt_mem_block_maxrecs(XFBNO_BLOCKSIZE-XFS_BTREE_LBLOCK_CRC_LEN,level==0)) }
#[cfg(CONFIG_XFS_BTREE_IN_MEM)]
unsafe fn xfs_rmapbt_mem_rw_verify(bp:*mut xfs_buf){let fa=xfs_rmapbt_mem_verify(bp);if !fa.is_null(){xfs_verifier_error(bp,-EFSCORRUPTED,fa);}}
#[cfg(CONFIG_XFS_BTREE_IN_MEM)]
pub unsafe fn xfs_rmapbt_mem_cursor(pag:*mut xfs_perag,tp:*mut xfs_trans,xfbt:*mut xfbtree)->*mut xfs_btree_cur { let cur=xfs_btree_alloc_cursor(pag_mount(pag),tp,&xfs_rmapbt_mem_ops,xfs_rmapbt_maxlevels_ondisk(),xfs_rmapbt_cur_cache);(*cur).bc_mem.xfbtree=xfbt;(*cur).bc_nlevels=(*xfbt).nlevels;(*cur).bc_group=xfs_group_hold(pag_group(pag));cur }
#[cfg(CONFIG_XFS_BTREE_IN_MEM)]
pub unsafe fn xfs_rmapbt_mem_init(mp:*mut xfs_mount,xfbt:*mut xfbtree,btp:*mut xfs_buftarg,agno:xfs_agnumber_t)->i32 {(*xfbt).owner=agno;xfbtree_init(mp,xfbt,btp,&xfs_rmapbt_mem_ops)}
#[cfg(CONFIG_XFS_BTREE_IN_MEM)]
unsafe fn xfs_rmapbt_mem_maxlevels()->u32 { let l=XFBNO_BLOCKSIZE-XFS_BTREE_LBLOCK_CRC_LEN; let m=[xfs_rmapbt_mem_block_maxrecs(l,true)/2,xfs_rmapbt_mem_block_maxrecs(l,false)/2]; xfs_btree_compute_maxlevels(m,XFS_MAX_AG_BYTES/core::mem::size_of::<xfs_rmap_rec>()) }
#[cfg(not(CONFIG_XFS_BTREE_IN_MEM))]
unsafe fn xfs_rmapbt_mem_maxlevels()->u32 { 0 }
pub unsafe fn xfs_rmapbt_compute_maxlevels(mp:*mut xfs_mount) { if !xfs_has_rmapbt(mp){(*mp).m_rmap_maxlevels=0;return;} (*mp).m_rmap_maxlevels=if xfs_has_reflink(mp){xfs_btree_space_to_height((*mp).m_rmap_mnr,(*mp).m_sb.sb_agblocks)}else{xfs_btree_compute_maxlevels((*mp).m_rmap_mnr,(*mp).m_sb.sb_agblocks)}; ASSERT((*mp).m_rmap_maxlevels<=xfs_rmapbt_maxlevels_ondisk()); }
pub unsafe fn xfs_rmapbt_calc_size(mp:*mut xfs_mount,len:u64)->xfs_extlen_t { xfs_btree_calc_size((*mp).m_rmap_mnr,len) }
pub unsafe fn xfs_rmapbt_max_size(mp:*mut xfs_mount,agblocks:xfs_agblock_t)->xfs_extlen_t { if (*mp).m_rmap_mxr[0]==0 {0}else{xfs_rmapbt_calc_size(mp,agblocks as u64)} }

pub unsafe fn xfs_rmapbt_commit_staged_btree(cur:*mut xfs_btree_cur,tp:*mut xfs_trans,agbp:*mut xfs_buf) { let agf=(*agbp).b_addr as *mut xfs_agf; let afake=(*cur).bc_ag.afake; ASSERT((*cur).bc_flags&XFS_BTREE_STAGING!=0); (*agf).agf_rmap_root=cpu_to_be32((*afake).af_root); (*agf).agf_rmap_level=cpu_to_be32((*afake).af_levels); (*agf).agf_rmap_blocks=cpu_to_be32((*afake).af_blocks); xfs_alloc_log_agf(tp,agbp,XFS_AGF_ROOTS|XFS_AGF_LEVELS|XFS_AGF_RMAP_BLOCKS); xfs_btree_commit_afakeroot(cur,tp,agbp); }

pub unsafe fn xfs_rmapbt_calc_reserves(mp:*mut xfs_mount,tp:*mut xfs_trans,pag:*mut xfs_perag,ask:*mut xfs_extlen_t,used:*mut xfs_extlen_t)->i32 { if !xfs_has_rmapbt(mp){return 0;} let mut agbp=core::ptr::null_mut(); let e=xfs_alloc_read_agf(pag,tp,0,&mut agbp); if e!=0{return e;} let agf=(*agbp).b_addr as *mut xfs_agf; let mut blocks=be32_to_cpu((*agf).agf_length); let tree=be32_to_cpu((*agf).agf_rmap_blocks); xfs_trans_brelse(tp,agbp); if xfs_ag_contains_log(mp,pag_agno(pag)){blocks-=(*mp).m_sb.sb_logblocks;} *ask+=max(blocks/100,xfs_rmapbt_max_size(mp,blocks)); *used+=tree; e }

pub unsafe fn xfs_rmapbt_init_cur_cache()->i32 { xfs_rmapbt_cur_cache=kmem_cache_create("xfs_rmapbt_cur",xfs_btree_cur_sizeof(xfs_rmapbt_maxlevels_ondisk()),0,0,core::ptr::null_mut()); if xfs_rmapbt_cur_cache.is_null(){-ENOMEM}else{0} }
pub unsafe fn xfs_rmapbt_destroy_cur_cache(){kmem_cache_destroy(xfs_rmapbt_cur_cache);xfs_rmapbt_cur_cache=core::ptr::null_mut();}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
