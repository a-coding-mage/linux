// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// Dependencies are supplied by the surrounding XFS translation.

#[repr(C)]
pub struct xchk_iallocbt {
    pub inodes: u64,
    pub next_startino: xfs_agino_t,
    pub next_cluster_ino: xfs_agino_t,
}

pub unsafe fn xchk_setup_ag_iallocbt(sc: *mut xfs_scrub) -> i32 {
    if xchk_need_intent_drain(sc) {
        xchk_fsgates_enable(sc, XCHK_FSGATES_DRAIN);
    }
    xchk_setup_ag_btree((*sc).flags & XCHK_TRY_HARDER)
}

unsafe fn xchk_inobt_xref_finobt(sc: *mut xfs_scrub, irec: *mut xfs_inobt_rec_incore,
        agino: xfs_agino_t, free: bool, hole: bool) -> i32 {
    let mut frec: xfs_inobt_rec_incore = core::mem::zeroed();
    let cur = (*sc).sa.fino_cur;
    let mut has_record: i32 = 0;
    let error = xfs_inobt_lookup(cur, agino, XFS_LOOKUP_LE, &mut has_record);
    if error != 0 { return error; }
    if has_record == 0 { return xchk_inobt_xref_finobt_no_record(sc, irec, cur, free, hole); }
    let error = xfs_inobt_get_rec(cur, &mut frec, &mut has_record);
    if has_record == 0 { return -EFSCORRUPTED; }
    if frec.ir_startino + XFS_INODES_PER_CHUNK <= agino {
        return xchk_inobt_xref_finobt_no_record(sc, irec, cur, free, hole);
    }
    let frec_idx = agino - frec.ir_startino;
    let ffree = (frec.ir_free & (1u64 << frec_idx)) != 0;
    let fhole_idx = frec_idx / XFS_INODES_PER_HOLEMASK_BIT;
    let fhole = (frec.ir_holemask & (1u32 << fhole_idx)) != 0;
    if ffree != free { xchk_btree_xref_set_corrupt(sc, cur, 0); }
    if fhole != hole { xchk_btree_xref_set_corrupt(sc, cur, 0); }
    0
}

unsafe fn xchk_inobt_xref_finobt_no_record(sc: *mut xfs_scrub,
        irec: *mut xfs_inobt_rec_incore, cur: *mut xfs_btree_cur,
        free: bool, hole: bool) -> i32 {
    if (*irec).ir_free == 0 || (*irec).ir_free == XFS_INOBT_ALL_FREE || hole || !free { return 0; }
    xchk_btree_xref_set_corrupt(sc, cur, 0);
    0
}

unsafe fn xchk_inobt_chunk_xref_finobt(sc: *mut xfs_scrub, irec: *mut xfs_inobt_rec_incore,
        agino: xfs_agino_t, nr_inodes: u32) {
    if (*sc).sa.fino_cur.is_null() || xchk_skip_xref((*sc).sm) { return; }
    let mut rec_idx = agino - (*irec).ir_startino;
    let mut i = agino;
    while i < agino + nr_inodes {
        let free = ((*irec).ir_free & (1u64 << rec_idx)) != 0;
        let hole_idx = rec_idx / XFS_INODES_PER_HOLEMASK_BIT;
        let hole = ((*irec).ir_holemask & (1u32 << hole_idx)) != 0;
        let mut error = xchk_inobt_xref_finobt(sc, irec, i, free, hole);
        if !xchk_should_check_xref(sc, &mut error, &mut (*sc).sa.fino_cur) { return; }
        i += 1; rec_idx += 1;
    }
}

unsafe fn xchk_finobt_xref_inobt(sc: *mut xfs_scrub, frec: *mut xfs_inobt_rec_incore,
        agino: xfs_agino_t, ffree: bool, fhole: bool) -> i32 {
    let mut irec: xfs_inobt_rec_incore = core::mem::zeroed();
    let cur = (*sc).sa.ino_cur;
    let mut has_record = 0;
    let error = xfs_inobt_lookup(cur, agino, XFS_LOOKUP_LE, &mut has_record);
    if error != 0 { return error; }
    if has_record == 0 { xchk_btree_xref_set_corrupt(sc, cur, 0); return 0; }
    let error = xfs_inobt_get_rec(cur, &mut irec, &mut has_record);
    if has_record == 0 { return -EFSCORRUPTED; }
    if irec.ir_startino + XFS_INODES_PER_CHUNK <= agino { xchk_btree_xref_set_corrupt(sc, cur, 0); return 0; }
    let rec_idx = agino - irec.ir_startino;
    let free = (irec.ir_free & (1u64 << rec_idx)) != 0;
    let hole = (irec.ir_holemask & (1u32 << (rec_idx / XFS_INODES_PER_HOLEMASK_BIT))) != 0;
    if ffree != free { xchk_btree_xref_set_corrupt(sc, cur, 0); }
    if fhole != hole { xchk_btree_xref_set_corrupt(sc, cur, 0); }
    0
}

unsafe fn xchk_finobt_chunk_xref_inobt(sc: *mut xfs_scrub, frec: *mut xfs_inobt_rec_incore,
        agino: xfs_agino_t, nr_inodes: u32) {
    if (*sc).sa.ino_cur.is_null() || xchk_skip_xref((*sc).sm) { return; }
    let mut rec_idx = agino - (*frec).ir_startino;
    let mut i = agino;
    while i < agino + nr_inodes {
        let ffree = ((*frec).ir_free & (1u64 << rec_idx)) != 0;
        let fhole = ((*frec).ir_holemask & (1u32 << (rec_idx / XFS_INODES_PER_HOLEMASK_BIT))) != 0;
        let mut error = xchk_finobt_xref_inobt(sc, frec, i, ffree, fhole);
        if !xchk_should_check_xref(sc, &mut error, &mut (*sc).sa.ino_cur) { return; }
        i += 1; rec_idx += 1;
    }
}

unsafe fn xchk_iallocbt_chunk(bs: *mut xchk_btree, irec: *mut xfs_inobt_rec_incore,
        agino: xfs_agino_t, nr_inodes: u32) -> bool {
    let sc = (*bs).sc; let mp = (*(*bs).cur).bc_mp;
    let pag = to_perag((*(*bs).cur).bc_group);
    let agbno = XFS_AGINO_TO_AGBNO(mp, agino);
    let len = XFS_B_TO_FSB(mp, nr_inodes * (*mp).m_sb.sb_inodesize);
    if !xfs_verify_agbext(pag, agbno, len) { xchk_btree_set_corrupt(sc, (*bs).cur, 0); }
    if (*sc).sm.sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return false; }
    xchk_xref_is_used_space(sc, agbno, len);
    if (*sc).sm.sm_type == XFS_SCRUB_TYPE_INOBT { xchk_inobt_chunk_xref_finobt(sc, irec, agino, nr_inodes); }
    else { xchk_finobt_chunk_xref_inobt(sc, irec, agino, nr_inodes); }
    xchk_xref_is_only_owned_by(sc, agbno, len, &XFS_RMAP_OINFO_INODES);
    xchk_xref_is_not_shared(sc, agbno, len); xchk_xref_is_not_cow_staging(sc, agbno, len); true
}

unsafe fn xchk_iallocbt_check_cluster_ifree(bs: *mut xchk_btree, irec: *mut xfs_inobt_rec_incore,
        irec_ino: u32, dip: *mut xfs_dinode) -> i32 {
    let mut error = 0;
    if xchk_should_terminate((*bs).sc, &mut error) { return error; }
    let agino = (*irec).ir_startino + irec_ino;
    let fsino = xfs_agino_to_ino(to_perag((*(*bs).cur).bc_group), agino);
    let irec_free = ((*irec).ir_free & XFS_INOBT_MASK(irec_ino)) != 0;
    if be16_to_cpu((*dip).di_magic) != XFS_DINODE_MAGIC ||
       ((*dip).di_version >= 3 && be64_to_cpu((*dip).di_ino) != fsino) {
        xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0); return 0;
    }
    let mut ino_inuse = false;
    error = xchk_inode_is_allocated((*bs).sc, agino, &mut ino_inuse);
    let freemask_ok;
    if error == -ENODATA {
        freemask_ok = irec_free ^ ((*dip).di_mode != 0);
        if (*(*bs).sc).flags & XCHK_TRY_HARDER == 0 && !freemask_ok { return -EDEADLOCK; }
    } else if error < 0 { return 0; }
    else { freemask_ok = irec_free ^ ino_inuse; }
    if !freemask_ok { xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0); } 0
}

unsafe fn xchk_iallocbt_check_clusters(bs: *mut xchk_btree, irec: *mut xfs_inobt_rec_incore) -> i32 {
    let mut error = 0; let mut cluster_base = 0;
    while cluster_base < XFS_INODES_PER_CHUNK {
        error = xchk_iallocbt_check_cluster(bs, irec, cluster_base);
        if error != 0 { break; }
        cluster_base += M_IGEO((*(*bs).sc).mp).inodes_per_cluster;
    } error
}

unsafe fn xchk_iallocbt_check_cluster(bs: *mut xchk_btree, irec: *mut xfs_inobt_rec_incore,
        cluster_base: u32) -> i32 {
    let mp = (*(*bs).cur).bc_mp; let pag = to_perag((*(*bs).cur).bc_group);
    let nr = core::cmp::min(XFS_INODES_PER_CHUNK, M_IGEO(mp).inodes_per_cluster);
    let agbno = XFS_AGINO_TO_AGBNO(mp, (*irec).ir_startino + cluster_base);
    let mut mask: u16 = 0; let mut j = 0;
    while j < nr { mask |= XFS_INOBT_MASK((cluster_base + j) / XFS_INODES_PER_HOLEMASK_BIT); j += XFS_INODES_PER_HOLEMASK_BIT; }
    let holemask = (*irec).ir_holemask & mask;
    if holemask != mask && holemask != 0 { xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0); return 0; }
    if holemask != 0 { xchk_xref_is_not_owned_by((*bs).sc, agbno, M_IGEO(mp).blocks_per_cluster, &XFS_RMAP_OINFO_INODES); return 0; }
    xchk_xref_is_only_owned_by((*bs).sc, agbno, M_IGEO(mp).blocks_per_cluster, &XFS_RMAP_OINFO_INODES);
    let mut imap: xfs_imap = core::mem::zeroed(); imap.im_agbno = agbno;
    imap.im_boffset = XFS_INO_TO_OFFSET(mp, (*irec).ir_startino) << (*mp).m_sb.sb_inodelog;
    let mut bp: *mut xfs_buf = core::ptr::null_mut();
    let mut error = xfs_read_icluster(pag, (*(*bs).cur).bc_tp, imap.im_agbno, &mut bp);
    if !xchk_btree_xref_process_error((*bs).sc, (*bs).cur, 0, &mut error) { return error; }
    j = 0;
    while j < nr { if imap.im_boffset >= BBTOB((*bp).b_length) { xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0); break; }
        let dip = xfs_buf_offset(bp, imap.im_boffset); error = xchk_iallocbt_check_cluster_ifree(bs, irec, cluster_base + j, dip); if error != 0 { break; }
        imap.im_boffset += (*mp).m_sb.sb_inodesize; j += 1; }
    xfs_trans_brelse((*(*bs).cur).bc_tp, bp); error
}

pub unsafe fn xchk_xref_is_not_inode_chunk(sc: *mut xfs_scrub, agbno: xfs_agblock_t, len: xfs_extlen_t) {
    xchk_xref_inode_check(sc, agbno, len, &mut (*sc).sa.ino_cur, XBTREE_RECPACKING_EMPTY);
    xchk_xref_inode_check(sc, agbno, len, &mut (*sc).sa.fino_cur, XBTREE_RECPACKING_EMPTY);
}
pub unsafe fn xchk_xref_is_inode_chunk(sc: *mut xfs_scrub, agbno: xfs_agblock_t, len: xfs_extlen_t) {
    xchk_xref_inode_check(sc, agbno, len, &mut (*sc).sa.ino_cur, XBTREE_RECPACKING_FULL);
}

unsafe fn xchk_xref_inode_check(sc: *mut xfs_scrub, agbno: xfs_agblock_t, len: xfs_extlen_t,
        icur: *mut *mut xfs_btree_cur, expected: enum_xbtree_recpacking) {
    if (*icur).is_null() || xchk_skip_xref((*sc).sm) { return; }
    let mut outcome = XBTREE_RECPACKING_EMPTY;
    let mut error = xfs_ialloc_has_inodes_at_extent(*icur, agbno, len, &mut outcome);
    if !xchk_should_check_xref(sc, &mut error, icur) { return; }
    if outcome != expected { xchk_btree_xref_set_corrupt(sc, *icur, 0); }
}

unsafe fn xchk_iallocbt_xref_rmap_btreeblks(sc: *mut xfs_scrub) {
    if (*sc).sa.ino_cur.is_null() || (*sc).sa.rmap_cur.is_null() ||
       (xfs_has_finobt((*sc).mp) && (*sc).sa.fino_cur.is_null()) || xchk_skip_xref((*sc).sm) { return; }
    let mut ib = 0; let mut fb = 0; let mut blocks = 0;
    let mut error = xfs_btree_count_blocks((*sc).sa.ino_cur, &mut ib);
    if !xchk_process_error(sc, 0, 0, &mut error) { return; }
    if !(*sc).sa.fino_cur.is_null() { error = xfs_btree_count_blocks((*sc).sa.fino_cur, &mut fb); if !xchk_process_error(sc, 0, 0, &mut error) { return; } }
    error = xchk_count_rmap_ownedby_ag(sc, (*sc).sa.rmap_cur, &XFS_RMAP_OINFO_INOBT, &mut blocks);
    if !xchk_should_check_xref(sc, &mut error, &mut (*sc).sa.rmap_cur) { return; }
    if blocks != ib + fb { xchk_btree_set_corrupt(sc, (*sc).sa.ino_cur, 0); }
}

unsafe fn xchk_iallocbt_rec(bs: *mut xchk_btree, rec: *const xfs_btree_rec) -> i32 {
    let mp = (*(*bs).cur).bc_mp; let mut irec: xfs_inobt_rec_incore = core::mem::zeroed();
    xfs_inobt_btrec_to_irec(mp, rec, &mut irec);
    if xfs_inobt_check_irec(to_perag((*(*bs).cur).bc_group), &irec) != 0 { xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0); return 0; }
    let mut agino = irec.ir_startino; let mut holes = !xfs_inobt_irec_to_allocmask(&irec);
    let mut holemask = irec.ir_holemask; let mut holecount = 0; let mut i = 0;
    if !xfs_inobt_issparse(irec.ir_holemask) {
        if irec.ir_count != XFS_INODES_PER_CHUNK { xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0); }
        if !xchk_iallocbt_chunk(bs, &mut irec, agino, XFS_INODES_PER_CHUNK) { return 0; }
    } else {
        if holes & irec.ir_free != holes || irec.ir_freecount > irec.ir_count { xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0); }
        while i < XFS_INOBT_HOLEMASK_BITS { if holemask & 1 != 0 { holecount += XFS_INODES_PER_HOLEMASK_BIT; }
            else if !xchk_iallocbt_chunk(bs, &mut irec, agino, XFS_INODES_PER_HOLEMASK_BIT) { return 0; }
            holemask >>= 1; agino += XFS_INODES_PER_HOLEMASK_BIT; i += 1; }
        if holecount > XFS_INODES_PER_CHUNK || holecount + irec.ir_count != XFS_INODES_PER_CHUNK { xchk_btree_set_corrupt((*bs).sc, (*bs).cur, 0); }
    }
    if (*(*bs).sc).sm.sm_flags & XFS_SCRUB_OFLAG_CORRUPT == 0 { xchk_iallocbt_check_clusters(bs, &mut irec) } else { 0 }
}

pub unsafe fn xchk_iallocbt(sc: *mut xfs_scrub) -> i32 {
    let mut iabt = xchk_iallocbt { inodes: 0, next_startino: NULLAGINO, next_cluster_ino: NULLAGINO };
    let cur = match (*sc).sm.sm_type { XFS_SCRUB_TYPE_INOBT => (*sc).sa.ino_cur, XFS_SCRUB_TYPE_FINOBT => (*sc).sa.fino_cur, _ => return -EIO };
    let error = xchk_btree(sc, cur, xchk_iallocbt_rec, &XFS_RMAP_OINFO_INOBT, &mut iabt);
    if error != 0 { return error; }
    xchk_iallocbt_xref_rmap_btreeblks(sc); error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
