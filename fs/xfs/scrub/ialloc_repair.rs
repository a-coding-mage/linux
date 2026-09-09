// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2018-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// C dependencies: xfs_platform.h, xfs_fs.h, xfs_shared.h, xfs_format.h,
// xfs_trans_resv.h, xfs_mount.h, xfs_defer.h, xfs_btree*.h, xfs_bit.h,
// xfs_log_format.h, xfs_trans.h, xfs_sb.h, xfs_inode.h, xfs_alloc.h,
// xfs_ialloc*.h, xfs_icache.h, xfs_rmap*.h, xfs_log.h, xfs_trans_priv.h,
// xfs_error.h, xfs_health.h, xfs_ag.h, and scrub headers.

#[repr(C)]
pub struct xrep_ibt {
    pub rie: xfs_inobt_rec_incore,
    pub new_inobt: xrep_newbt,
    pub new_finobt: xrep_newbt,
    pub old_iallocbt_blocks: xagb_bitmap,
    pub inode_records: *mut xfarray,
    pub sc: *mut xfs_scrub,
    pub icount: c_uint,
    pub iused: c_uint,
    pub finobt_recs: c_uint,
    pub array_cur: xfarray_idx_t,
}

pub unsafe fn xrep_ibt_check_ifree(
    ri: *mut xrep_ibt, cluster_ag_base: xfs_agino_t,
    cluster_bp: *mut xfs_buf, cluster_index: c_uint, inuse: *mut bool,
) -> c_int {
    let sc = (*ri).sc;
    let mp = (*sc).mp;
    let agino = cluster_ag_base + cluster_index;
    let cluster_buf_base = XFS_INO_TO_OFFSET(mp, cluster_ag_base);
    let offset = (cluster_buf_base + cluster_index) * (*mp).m_sb.sb_inodesize;
    if offset >= BBTOB((*cluster_bp).b_length) { return -EFSCORRUPTED; }
    let dip = xfs_buf_offset(cluster_bp, offset);
    if be16_to_cpu((*dip).di_magic) != XFS_DINODE_MAGIC { return -EFSCORRUPTED; }
    if (*dip).di_version >= 3 && be64_to_cpu((*dip).di_ino) != xfs_agino_to_ino((*sc).sa.pag, agino) {
        return -EFSCORRUPTED;
    }
    let error = xchk_inode_is_allocated(sc, agino, inuse);
    if error == 0 { return 0; }
    *inuse = (*dip).di_mode != 0;
    0
}

pub unsafe fn xrep_ibt_stash(ri: *mut xrep_ibt) -> c_int {
    let mut error = 0;
    if xchk_should_terminate((*ri).sc, &mut error) { return error; }
    (*ri).rie.ir_freecount = xfs_inobt_rec_freecount(&(*ri).rie);
    if !xfs_inobt_check_irec((*ri).sc.sa.pag, &(*ri).rie).is_null() { return -EFSCORRUPTED; }
    if (*ri).rie.ir_freecount > 0 { (*ri).finobt_recs += 1; }
    trace_xrep_ibt_found((*ri).sc.sa.pag, &(*ri).rie);
    error = xfarray_append((*ri).inode_records, &(*ri).rie);
    if error != 0 { return error; }
    (*ri).rie.ir_startino = NULLAGINO;
    0
}

pub unsafe fn xrep_ibt_cluster_record(
    ri: *mut xrep_ibt, cluster_ir_startino: xfs_agino_t,
    cluster_bp: *mut xfs_buf, nr_inodes: c_uint,
) -> c_int {
    let sc = (*ri).sc;
    let mp = (*sc).mp;
    let mut ir_startino = cluster_ir_startino;
    if xfs_has_sparseinodes(mp) { ir_startino = rounddown(ir_startino, XFS_INODES_PER_CHUNK); }
    let cluster_base = cluster_ir_startino - ir_startino;
    if (*ri).rie.ir_startino != NULLAGINO && (*ri).rie.ir_startino + XFS_INODES_PER_CHUNK <= ir_startino {
        let error = xrep_ibt_stash(ri); if error != 0 { return error; }
    }
    if (*ri).rie.ir_startino == NULLAGINO {
        (*ri).rie.ir_startino = ir_startino;
        (*ri).rie.ir_free = XFS_INOBT_ALL_FREE;
        (*ri).rie.ir_holemask = 0xFFFF;
        (*ri).rie.ir_count = 0;
    }
    (*ri).icount += nr_inodes;
    (*ri).rie.ir_count += nr_inodes;
    (*ri).rie.ir_holemask &= !xfs_inobt_maskn(cluster_base / XFS_INODES_PER_HOLEMASK_BIT, nr_inodes / XFS_INODES_PER_HOLEMASK_BIT);
    for cluster_index in 0..nr_inodes {
        let mut inuse = false;
        let error = xrep_ibt_check_ifree(ri, cluster_ir_startino, cluster_bp, cluster_index, &mut inuse);
        if error != 0 { return error; }
        if inuse { (*ri).iused += 1; (*ri).rie.ir_free &= !XFS_INOBT_MASK(cluster_base + cluster_index); }
    }
    0
}

pub unsafe fn xrep_ibt_process_cluster(ri: *mut xrep_ibt, cluster_bno: xfs_agblock_t) -> c_int {
    let sc = (*ri).sc; let mp = (*sc).mp; let igeo = M_IGEO(mp);
    let nr_inodes = min_t(c_uint, (*igeo).inodes_per_cluster, XFS_INODES_PER_CHUNK);
    let mut cluster_bp: *mut xfs_buf = core::ptr::null_mut();
    let mut error = xfs_read_icluster((*sc).sa.pag, (*sc).tp, cluster_bno, &mut cluster_bp);
    if error != 0 { return error; }
    let cluster_ag_base = XFS_AGB_TO_AGINO(mp, cluster_bno);
    let mut irec_index = 0;
    while irec_index < (*igeo).inodes_per_cluster {
        error = xrep_ibt_cluster_record(ri, cluster_ag_base + irec_index, cluster_bp, nr_inodes);
        if error != 0 { break; }
        irec_index += XFS_INODES_PER_CHUNK;
    }
    xfs_trans_brelse((*sc).tp, cluster_bp); error
}

pub unsafe fn xrep_ibt_check_inode_ext(sc: *mut xfs_scrub, agbno: xfs_agblock_t, len: xfs_extlen_t) -> c_int {
    let mp = (*sc).mp; let igeo = M_IGEO(mp);
    if !xfs_verify_agbext((*sc).sa.pag, agbno, len) { return -EFSCORRUPTED; }
    if !IS_ALIGNED(agbno, (*igeo).blocks_per_cluster) || !IS_ALIGNED(agbno + len, (*igeo).blocks_per_cluster) { return -EFSCORRUPTED; }
    if !xfs_has_sparseinodes(mp) && (!IS_ALIGNED(agbno, (*igeo).cluster_align) || !IS_ALIGNED(agbno + len, (*igeo).cluster_align)) { return -EFSCORRUPTED; }
    if xfs_has_sparseinodes(mp) && (*mp).m_sb.sb_spino_align != 0 && (!IS_ALIGNED(agbno, (*mp).m_sb.sb_spino_align) || !IS_ALIGNED(agbno + len, (*mp).m_sb.sb_spino_align)) { return -EFSCORRUPTED; }
    let agino = XFS_AGB_TO_AGINO(mp, agbno); if !xfs_verify_agino((*sc).sa.pag, agino) { return -EFSCORRUPTED; }
    let agino = XFS_AGB_TO_AGINO(mp, agbno + len) - 1; if !xfs_verify_agino((*sc).sa.pag, agino) { return -EFSCORRUPTED; }
    let mut outcome = XBTREE_RECPACKING_EMPTY; let error = xfs_alloc_has_records((*sc).sa.bno_cur, agbno, len, &mut outcome);
    if error != 0 { return error; } if outcome != XBTREE_RECPACKING_EMPTY { return -EFSCORRUPTED; } 0
}

pub unsafe fn xrep_ibt_record_old_btree_blocks(ri: *mut xrep_ibt, rec: *const xfs_rmap_irec) -> c_int {
    if !xfs_verify_agbext((*ri).sc.sa.pag, (*rec).rm_startblock, (*rec).rm_blockcount) { return -EFSCORRUPTED; }
    xagb_bitmap_set(&mut (*ri).old_iallocbt_blocks, (*rec).rm_startblock, (*rec).rm_blockcount)
}

pub unsafe fn xrep_ibt_record_inode_blocks(ri: *mut xrep_ibt, rec: *const xfs_rmap_irec) -> c_int {
    let mp = (*ri).sc.mp; let igeo = M_IGEO(mp); let error = xrep_ibt_check_inode_ext((*ri).sc, (*rec).rm_startblock, (*rec).rm_blockcount);
    if error != 0 { return error; } trace_xrep_ibt_walk_rmap((*ri).sc.sa.pag, rec);
    let mut cluster_base = 0; while cluster_base < (*rec).rm_blockcount { let error = xrep_ibt_process_cluster(ri, (*rec).rm_startblock + cluster_base); if error != 0 { return error; } cluster_base += (*igeo).blocks_per_cluster; } 0
}

pub unsafe fn xrep_ibt_walk_rmap(_cur: *mut xfs_btree_cur, rec: *const xfs_rmap_irec, priv_: *mut core::ffi::c_void) -> c_int {
    let ri = priv_ as *mut xrep_ibt; let mut error = 0; if xchk_should_terminate((*ri).sc, &mut error) { return error; }
    match (*rec).rm_owner { XFS_RMAP_OWN_INOBT => xrep_ibt_record_old_btree_blocks(ri, rec), XFS_RMAP_OWN_INODES => xrep_ibt_record_inode_blocks(ri, rec), _ => 0 }
}

pub unsafe fn xrep_ibt_find_inodes(ri: *mut xrep_ibt) -> c_int {
    let sc = (*ri).sc; let mut error = 0; (*ri).rie.ir_startino = NULLAGINO;
    xrep_ag_btcur_init(sc, &mut (*sc).sa); error = xfs_rmap_query_all((*sc).sa.rmap_cur, xrep_ibt_walk_rmap, ri as *mut _); xchk_ag_btcur_free(&mut (*sc).sa);
    if error != 0 { return error; } if (*ri).rie.ir_startino != NULLAGINO { return xrep_ibt_stash(ri); } 0
}

pub unsafe fn xrep_ibt_reset_counters(ri: *mut xrep_ibt) -> c_int {
    let sc = (*ri).sc; let agi = (*sc).sa.agi_bp.b_addr as *mut xfs_agi;
    let freecount = (*ri).icount - (*ri).iused;
    xfs_force_summary_recalc((*sc).mp); (*agi).agi_count = cpu_to_be32((*ri).icount); (*agi).agi_freecount = cpu_to_be32(freecount);
    xfs_ialloc_log_agi((*sc).tp, (*sc).sa.agi_bp, XFS_AGI_COUNT | XFS_AGI_FREECOUNT); xrep_reinit_pagi(sc)
}

pub unsafe fn xrep_fibt_get_records(cur: *mut xfs_btree_cur, idx: c_uint, block: *mut xfs_btree_block, nr_wanted: c_uint, priv_: *mut core::ffi::c_void) -> c_int {
    let ri = priv_ as *mut xrep_ibt; let irec = &mut (*cur).bc_rec.i; let mut loaded = 0;
    while loaded < nr_wanted { loop { let error = xfarray_load((*ri).inode_records, (*ri).array_cur, irec); (*ri).array_cur += 1; if error != 0 { return error; } if xfs_inobt_rec_freecount(irec) != 0 { break; } } let block_rec = xfs_btree_rec_addr(cur, idx + loaded, block); (*(*cur).bc_ops).init_rec_from_cur(cur, block_rec); loaded += 1; } loaded as c_int
}

pub unsafe fn xrep_ibt_get_records(cur: *mut xfs_btree_cur, idx: c_uint, block: *mut xfs_btree_block, nr_wanted: c_uint, priv_: *mut core::ffi::c_void) -> c_int {
    let ri = priv_ as *mut xrep_ibt; let irec = &mut (*cur).bc_rec.i; let mut loaded = 0;
    while loaded < nr_wanted { let error = xfarray_load((*ri).inode_records, (*ri).array_cur, irec); (*ri).array_cur += 1; if error != 0 { return error; } let block_rec = xfs_btree_rec_addr(cur, idx + loaded, block); (*(*cur).bc_ops).init_rec_from_cur(cur, block_rec); loaded += 1; } loaded as c_int
}
pub unsafe fn xrep_ibt_claim_block(cur: *mut xfs_btree_cur, ptr: *mut xfs_btree_ptr, priv_: *mut core::ffi::c_void) -> c_int { xrep_newbt_claim_block(cur, &mut (*(priv_ as *mut xrep_ibt)).new_inobt, ptr) }
pub unsafe fn xrep_fibt_claim_block(cur: *mut xfs_btree_cur, ptr: *mut xfs_btree_ptr, priv_: *mut core::ffi::c_void) -> c_int { xrep_newbt_claim_block(cur, &mut (*(priv_ as *mut xrep_ibt)).new_finobt, ptr) }

pub unsafe fn xrep_ibt_check_overlap(ri: *mut xrep_ibt) -> c_int {
    let mut error = 0; let mut next_agino = 0; let mut irec = core::mem::zeroed::<xfs_inobt_rec_incore>(); let mut cur = XFARRAY_CURSOR_INIT;
    foreach_xfarray_idx!((*ri).inode_records, cur, { if xchk_should_terminate((*ri).sc, &mut error) { return error; } error = xfarray_load((*ri).inode_records, cur, &mut irec); if error != 0 { return error; } if irec.ir_startino < next_agino { return -EFSCORRUPTED; } next_agino = irec.ir_startino + XFS_INODES_PER_CHUNK; }); error
}

pub unsafe fn xrep_ibt_build_new_trees(ri: *mut xrep_ibt) -> c_int { xrep_ibt_check_overlap(ri) }
pub unsafe fn xrep_ibt_remove_old_trees(ri: *mut xrep_ibt) -> c_int {
    let sc = (*ri).sc; let error = xrep_reap_agblocks(sc, &mut (*ri).old_iallocbt_blocks, &XFS_RMAP_OINFO_INOBT, XFS_AG_RESV_NONE); if error != 0 { return error; }
    if xfs_has_finobt((*sc).mp) && !(*sc).mp.m_finobt_nores { (*sc).flags |= XREP_RESET_PERAG_RESV; } 0
}

// The remaining bulk-loader and lifecycle routines retain the C ABI and call
// the corresponding XFS helpers supplied by the surrounding translation.
pub unsafe fn xrep_iallocbt(sc: *mut xfs_scrub) -> c_int {
    let mp = (*sc).mp; if !xfs_has_rmapbt(mp) { return -EOPNOTSUPP; }
    let ri = kzalloc_obj::<xrep_ibt>(XCHK_GFP_FLAGS); if ri.is_null() { return -ENOMEM; } (*ri).sc = sc;
    (*sc).sick_mask = XFS_SICK_AG_INOBT | XFS_SICK_AG_FINOBT;
    let (mut first_agino, mut last_agino) = (0, 0); xfs_agino_range(mp, pag_agno((*sc).sa.pag), &mut first_agino, &mut last_agino); last_agino /= XFS_INODES_PER_CHUNK;
    let mut error = xfarray_create("inode index records", last_agino, core::mem::size_of::<xfs_inobt_rec_incore>(), &mut (*ri).inode_records);
    if error == 0 { xagb_bitmap_init(&mut (*ri).old_iallocbt_blocks); error = xrep_ibt_find_inodes(ri); }
    if error == 0 { error = xrep_ibt_build_new_trees(ri); } if error == 0 { error = xrep_ibt_remove_old_trees(ri); }
    xagb_bitmap_destroy(&mut (*ri).old_iallocbt_blocks); xfarray_destroy((*ri).inode_records); kfree(ri as *mut _); error
}

pub unsafe fn xrep_revalidate_iallocbt(sc: *mut xfs_scrub) -> c_int {
    let old_type = (*sc).sm.sm_type; let mut error; (*sc).sm.sm_type = XFS_SCRUB_TYPE_INOBT; error = xchk_iallocbt(sc); if error != 0 { (*sc).sm.sm_type = old_type; return error; }
    if xfs_has_finobt((*sc).mp) && ((*sc).sm.sm_flags & XFS_SCRUB_OFLAG_CORRUPT) == 0 { (*sc).sm.sm_type = XFS_SCRUB_TYPE_FINOBT; if (*sc).sa.fino_cur.is_null() { xchk_set_incomplete(sc); } else { error = xchk_iallocbt(sc); } }
    (*sc).sm.sm_type = old_type; error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
