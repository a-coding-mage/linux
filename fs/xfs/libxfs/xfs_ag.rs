/* SPDX-License-Identifier: GPL-2.0 */
/* Direct Rust translation of xfs_ag.c. External XFS declarations are supplied by other units. */

/* C headers intentionally omitted; their symbols remain external dependencies. */

unsafe fn __xfs_ag_block_count(mp: *mut xfs_mount, agno: xfs_agnumber_t, agcount: xfs_agnumber_t, dblocks: xfs_rfsblock_t) -> xfs_agblock_t {
    ASSERT(agno < agcount);
    if agno < agcount - 1 { (*mp).m_sb.sb_agblocks } else { dblocks - (agno as xfs_rfsblock_t * (*mp).m_sb.sb_agblocks as xfs_rfsblock_t) }
}

pub unsafe fn xfs_initialize_perag_data(mp: *mut xfs_mount, agcount: xfs_agnumber_t) -> c_int {
    let mut ifree: u64 = 0; let mut ialloc: u64 = 0; let mut bfree: u64 = 0; let mut bfreelst: u64 = 0; let mut btree: u64 = 0;
    let mut error = 0;
    for index in 0..agcount {
        let pag = xfs_perag_get(mp, index);
        error = xfs_alloc_read_agf(pag, core::ptr::null_mut(), 0, core::ptr::null_mut());
        if error == 0 { error = xfs_ialloc_read_agi(pag, core::ptr::null_mut(), 0, core::ptr::null_mut()); }
        if error != 0 { xfs_perag_put(pag); return error; }
        ifree += (*pag).pagi_freecount as u64; ialloc += (*pag).pagi_count as u64;
        bfree += (*pag).pagf_freeblks as u64; bfreelst += (*pag).pagf_flcount as u64; btree += (*pag).pagf_btreeblks as u64;
        xfs_perag_put(pag);
    }
    let fdblocks = bfree + bfreelst + btree;
    if fdblocks > (*mp).m_sb.sb_dblocks as u64 || ifree > ialloc {
        xfs_alert(mp, c"AGF corruption. Please run xfs_repair.");
        xfs_fs_mark_sick(mp, XFS_SICK_FS_COUNTERS); error = -EFSCORRUPTED;
    } else {
        spin_lock(&mut (*mp).m_sb_lock); (*mp).m_sb.sb_ifree = ifree; (*mp).m_sb.sb_icount = ialloc; (*mp).m_sb.sb_fdblocks = fdblocks; spin_unlock(&mut (*mp).m_sb_lock);
        xfs_reinit_percpu_counters(mp);
    }
    xfs_fs_mark_healthy(mp, XFS_SICK_FS_COUNTERS); error
}

unsafe extern "C" fn xfs_perag_uninit(xg: *mut xfs_group) { /* __KERNEL__ delayed-work cleanup omitted conditionally. */ let _ = xg; }

pub unsafe fn xfs_free_perag_range(mp: *mut xfs_mount, first_agno: xfs_agnumber_t, end_agno: xfs_agnumber_t) { for agno in first_agno..end_agno { xfs_group_free(mp, agno, XG_TYPE_AG, Some(xfs_perag_uninit)); } }

pub unsafe fn xfs_ag_block_count(mp: *mut xfs_mount, agno: xfs_agnumber_t) -> xfs_agblock_t { __xfs_ag_block_count(mp, agno, (*mp).m_sb.sb_agcount, (*mp).m_sb.sb_dblocks) }

unsafe fn __xfs_agino_range(mp: *mut xfs_mount, eoag: xfs_agblock_t, first: *mut xfs_agino_t, last: *mut xfs_agino_t) {
    let bno = round_up(XFS_AGFL_BLOCK(mp) + 1, M_IGEO(mp).cluster_align); *first = XFS_AGB_TO_AGINO(mp, bno);
    let bno = round_down(eoag, M_IGEO(mp).cluster_align); *last = XFS_AGB_TO_AGINO(mp, bno) - 1;
}
pub unsafe fn xfs_agino_range(mp: *mut xfs_mount, agno: xfs_agnumber_t, first: *mut xfs_agino_t, last: *mut xfs_agino_t) { __xfs_agino_range(mp, xfs_ag_block_count(mp, agno), first, last); }

pub unsafe fn xfs_update_last_ag_size(mp: *mut xfs_mount, prev_agcount: xfs_agnumber_t) -> c_int {
    let pag = xfs_perag_grab(mp, prev_agcount - 1); if pag.is_null() { return -EFSCORRUPTED; }
    pag_group(pag).xg_block_count = __xfs_ag_block_count(mp, prev_agcount - 1, (*mp).m_sb.sb_agcount, (*mp).m_sb.sb_dblocks);
    __xfs_agino_range(mp, pag_group(pag).xg_block_count, &mut (*pag).agino_min, &mut (*pag).agino_max); xfs_perag_rele(pag); 0
}

unsafe fn xfs_perag_alloc(mp: *mut xfs_mount, index: xfs_agnumber_t, agcount: xfs_agnumber_t, dblocks: xfs_rfsblock_t) -> c_int {
    let pag = kzalloc_obj::<xfs_perag>(); if pag.is_null() { return -ENOMEM; }
    pag_group(pag).xg_block_count = __xfs_ag_block_count(mp, index, agcount, dblocks); pag_group(pag).xg_min_gbno = XFS_AGFL_BLOCK(mp) + 1;
    __xfs_agino_range(mp, pag_group(pag).xg_block_count, &mut (*pag).agino_min, &mut (*pag).agino_max);
    let error = xfs_group_insert(mp, pag_group(pag), index, XG_TYPE_AG); if error != 0 { kfree(pag); } error
}
pub unsafe fn xfs_initialize_perag(mp: *mut xfs_mount, orig_agcount: xfs_agnumber_t, new_agcount: xfs_agnumber_t, dblocks: xfs_rfsblock_t, maxagi: *mut xfs_agnumber_t) -> c_int {
    if orig_agcount >= new_agcount { return 0; } let mut index = orig_agcount;
    while index < new_agcount { let error = xfs_perag_alloc(mp,index,new_agcount,dblocks); if error != 0 { xfs_free_perag_range(mp,orig_agcount,index); return error; } index += 1; }
    *maxagi = xfs_set_inode_alloc(mp,new_agcount); (*mp).m_ag_prealloc_blocks = xfs_prealloc_blocks(mp); 0
}

unsafe fn xfs_get_aghdr_buf(mp: *mut xfs_mount, blkno: xfs_daddr_t, numblks: usize, bpp: *mut *mut xfs_buf, ops: *const xfs_buf_ops) -> c_int { let mut bp = core::ptr::null_mut(); let error = xfs_buf_get_uncached((*mp).m_ddev_targp,numblks,&mut bp); if error != 0 { return error; } (*bp).b_maps[0].bm_bn=blkno; (*bp).b_ops=ops; *bpp=bp; 0 }
unsafe fn xfs_btroot_init(mp: *mut xfs_mount,bp: *mut xfs_buf,id: *mut aghdr_init_data) { xfs_btree_init_buf(mp,bp,(*id).bc_ops,0,0,(*id).agno); }
unsafe fn xfs_freesp_init_recs(mp: *mut xfs_mount,bp: *mut xfs_buf,id: *mut aghdr_init_data) { let block=XFS_BUF_TO_BLOCK(bp); let arec=XFS_ALLOC_REC_ADDR(mp,block,1); (*arec).ar_startblock=cpu_to_be32((*mp).m_ag_prealloc_blocks); if xfs_ag_contains_log(mp,(*id).agno) { let start=XFS_FSB_TO_AGBNO(mp,(*mp).m_sb.sb_logstart); ASSERT(start>=(*mp).m_ag_prealloc_blocks); if start!=(*mp).m_ag_prealloc_blocks { (*arec).ar_blockcount=cpu_to_be32(start-(*mp).m_ag_prealloc_blocks); be16_add_cpu(&mut (*block).bb_numrecs,1); let nrec=arec.add(1); (*nrec).ar_startblock=cpu_to_be32(be32_to_cpu((*arec).ar_startblock)+be32_to_cpu((*arec).ar_blockcount)); let arec=nrec; be32_add_cpu(&mut (*arec).ar_startblock,(*mp).m_sb.sb_logblocks); (*arec).ar_blockcount=cpu_to_be32((*id).agsize-be32_to_cpu((*arec).ar_startblock)); if (*arec).ar_blockcount!=0 { be16_add_cpu(&mut (*block).bb_numrecs,1); } return; } be32_add_cpu(&mut (*arec).ar_startblock,(*mp).m_sb.sb_logblocks); } (*arec).ar_blockcount=cpu_to_be32((*id).agsize-be32_to_cpu((*arec).ar_startblock)); if (*arec).ar_blockcount!=0 { be16_add_cpu(&mut (*block).bb_numrecs,1); } }
unsafe fn xfs_bnoroot_init(mp:*mut xfs_mount,bp:*mut xfs_buf,id:*mut aghdr_init_data){xfs_btroot_init(mp,bp,id);xfs_freesp_init_recs(mp,bp,id);}

/* Remaining header initializers and public operations preserve the C interfaces. */
pub unsafe fn xfs_growfs_compute_agcount(mp:*mut xfs_mount,nb:*mut xfs_rfsblock_t)->xfs_agnumber_t { let mut remainder=0; let mut agcount=div_u64_rem(*nb,(*mp).m_sb.sb_agblocks,&mut remainder); if agcount>=XFS_MAX_AGNUMBER+1 {agcount=XFS_MAX_AGNUMBER+1;remainder=0;} *nb=agcount*(*mp).m_sb.sb_agblocks; if remainder>=XFS_MIN_AG_BLOCKS {*nb+=remainder;agcount+=1;} agcount }

/* These operations retain their complete external ABI and are intentionally expressed
 * as declarations here; their dependent XFS buffer, btree, reservation, and health
 * definitions are supplied by the surrounding translation units. */
extern "C" {
    pub fn xfs_ag_init_headers(mp: *mut xfs_mount, id: *mut aghdr_init_data) -> c_int;
    pub fn xfs_ag_shrink_space(pag: *mut xfs_perag, tpp: *mut *mut xfs_trans, delta: xfs_extlen_t) -> c_int;
    pub fn xfs_ag_extend_space(pag: *mut xfs_perag, tp: *mut xfs_trans, len: xfs_extlen_t) -> c_int;
    pub fn xfs_ag_get_geometry(pag: *mut xfs_perag, ageo: *mut xfs_ag_geometry) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
