// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2006 Silicon Graphics, Inc.
 * All Rights Reserved.
 */
// Dependencies supplied by the surrounding XFS translation.

unsafe fn xlog_recover_inode_ra_pass2(log: *mut xlog, item: *mut xlog_recover_item) {
    unsafe {
        if (*item).ri_buf[0].iov_len == core::mem::size_of::<xfs_inode_log_format>() {
            let ilfp = (*item).ri_buf[0].iov_base as *mut xfs_inode_log_format;
            xlog_buf_readahead(log, (*ilfp).ilf_blkno, (*ilfp).ilf_len, &xfs_inode_buf_ra_ops);
        } else {
            let ilfp = (*item).ri_buf[0].iov_base as *mut xfs_inode_log_format_32;
            xlog_buf_readahead(log, (*ilfp).ilf_blkno, (*ilfp).ilf_len, &xfs_inode_buf_ra_ops);
        }
    }
}

/*
 * Inode fork owner changes
 *
 * If we have been told that we have to reparent the inode fork, it's because an
 * extent swap operation on a CRC enabled filesystem has been done and we are
 * replaying it. We need to walk the BMBT of the appropriate fork and change
 * the owners of it.
 *
 * The complexity here is that we don't have an inode context to work with, so
 * after we've replayed the inode we need to instantiate one. This is where the
 * fun begins.
 *
 * We are in the middle of log recovery, so we can't run transactions. That
 * means we cannot use cache coherent inode instantiation via xfs_iget(), as
 * that will result in the corresponding iput() running the inode through
 * xfs_inactive(). If we've just replayed an inode core that changes the link
 * count to zero (i.e. it's been unlinked), then xfs_inactive() will run
 * transactions (bad!).
 *
 * So, to avoid this, we instantiate an inode directly from the inode core we've
 * just recovered. We have the buffer still locked, and all we really need to
 * instantiate is the inode core and the forks being modified. We can do this
 * manually, then run the inode btree owner change, and then tear down the
 * xfs_inode without having to run any transactions at all.
 *
 * Also, because we don't have a transaction context available here but need to
 * gather all the buffers we modify for writeback so we pass the buffer_list
 * instead for the operation to use.
 */
unsafe fn xfs_recover_inode_owner_change(
    mp: *mut xfs_mount,
    dip: *mut xfs_dinode,
    in_f: *mut xfs_inode_log_format,
    buffer_list: *mut list_head,
) -> i32 {
    unsafe {
        ASSERT((*in_f).ilf_fields & (XFS_ILOG_DOWNER | XFS_ILOG_AOWNER) != 0);
        let ip = xfs_inode_alloc(mp, (*in_f).ilf_ino);
        if ip.is_null() { return -ENOMEM; }
        ASSERT((*dip).di_version >= 3);
        let mut error = xfs_inode_from_disk(ip, dip);
        if error != 0 { xfs_inode_free(ip); return error; }
        if (*in_f).ilf_fields & XFS_ILOG_DOWNER != 0 {
            ASSERT((*in_f).ilf_fields & XFS_ILOG_DBROOT != 0);
            error = xfs_bmbt_change_owner(core::ptr::null_mut(), ip, XFS_DATA_FORK, I_INO(ip), buffer_list);
            if error != 0 { xfs_inode_free(ip); return error; }
        }
        if (*in_f).ilf_fields & XFS_ILOG_AOWNER != 0 {
            ASSERT((*in_f).ilf_fields & XFS_ILOG_ABROOT != 0);
            error = xfs_bmbt_change_owner(core::ptr::null_mut(), ip, XFS_ATTR_FORK, I_INO(ip), buffer_list);
        }
        xfs_inode_free(ip);
        error
    }
}

#[inline]
unsafe fn xfs_log_dinode_has_bigtime(ld: *const xfs_log_dinode) -> bool {
    unsafe { (*ld).di_version >= 3 && ((*ld).di_flags2 & XFS_DIFLAG2_BIGTIME) != 0 }
}

/* Convert a log timestamp to an ondisk timestamp. */
#[inline]
unsafe fn xfs_log_dinode_to_disk_ts(from: *mut xfs_log_dinode, its: xfs_log_timestamp_t) -> xfs_timestamp_t {
    unsafe {
        if xfs_log_dinode_has_bigtime(from) { return cpu_to_be64(its); }
        let mut ts: xfs_timestamp_t = core::mem::zeroed();
        let lts = &mut *(&mut ts as *mut _ as *mut xfs_legacy_timestamp);
        let lits = &*(&its as *const _ as *const xfs_log_legacy_timestamp);
        lts.t_sec = cpu_to_be32(lits.t_sec);
        lts.t_nsec = cpu_to_be32(lits.t_nsec);
        ts
    }
}

#[inline]
unsafe fn xfs_log_dinode_has_large_extent_counts(ld: *const xfs_log_dinode) -> bool {
    unsafe { (*ld).di_version >= 3 && ((*ld).di_flags2 & XFS_DIFLAG2_NREXT64) != 0 }
}

#[inline]
unsafe fn xfs_log_dinode_to_disk_iext_counters(from: *mut xfs_log_dinode, to: *mut xfs_dinode) {
    unsafe {
        if xfs_log_dinode_has_large_extent_counts(from) {
            (*to).di_big_nextents = cpu_to_be64((*from).di_big_nextents);
            (*to).di_big_anextents = cpu_to_be32((*from).di_big_anextents);
            (*to).di_nrext64_pad = cpu_to_be16((*from).di_nrext64_pad);
        } else {
            (*to).di_nextents = cpu_to_be32((*from).di_nextents);
            (*to).di_anextents = cpu_to_be16((*from).di_anextents);
        }
    }
}

unsafe fn xfs_log_dinode_to_disk(from: *mut xfs_log_dinode, to: *mut xfs_dinode, lsn: xfs_lsn_t) {
    unsafe {
        (*to).di_magic = cpu_to_be16((*from).di_magic); (*to).di_mode = cpu_to_be16((*from).di_mode);
        (*to).di_version = (*from).di_version; (*to).di_format = (*from).di_format;
        (*to).di_metatype = cpu_to_be16((*from).di_metatype); (*to).di_uid = cpu_to_be32((*from).di_uid);
        (*to).di_gid = cpu_to_be32((*from).di_gid); (*to).di_nlink = cpu_to_be32((*from).di_nlink);
        (*to).di_projid_lo = cpu_to_be16((*from).di_projid_lo); (*to).di_projid_hi = cpu_to_be16((*from).di_projid_hi);
        (*to).di_atime = xfs_log_dinode_to_disk_ts(from, (*from).di_atime); (*to).di_mtime = xfs_log_dinode_to_disk_ts(from, (*from).di_mtime); (*to).di_ctime = xfs_log_dinode_to_disk_ts(from, (*from).di_ctime);
        (*to).di_size = cpu_to_be64((*from).di_size); (*to).di_nblocks = cpu_to_be64((*from).di_nblocks); (*to).di_extsize = cpu_to_be32((*from).di_extsize);
        (*to).di_forkoff = (*from).di_forkoff; (*to).di_aformat = (*from).di_aformat; (*to).di_dmevmask = cpu_to_be32((*from).di_dmevmask); (*to).di_dmstate = cpu_to_be16((*from).di_dmstate); (*to).di_flags = cpu_to_be16((*from).di_flags); (*to).di_gen = cpu_to_be32((*from).di_gen);
        if (*from).di_version == 3 {
            (*to).di_changecount = cpu_to_be64((*from).di_changecount); (*to).di_crtime = xfs_log_dinode_to_disk_ts(from, (*from).di_crtime); (*to).di_flags2 = cpu_to_be64((*from).di_flags2); (*to).di_cowextsize = cpu_to_be32((*from).di_cowextsize); (*to).di_ino = cpu_to_be64((*from).di_ino); (*to).di_lsn = cpu_to_be64(lsn); memset((*to).di_pad2.as_mut_ptr(), 0, core::mem::size_of_val(&(*to).di_pad2)); uuid_copy(&mut (*to).di_uuid, &(*from).di_uuid); (*to).di_v3_pad = 0;
        } else { (*to).di_flushiter = cpu_to_be16((*from).di_flushiter); memset((*to).di_v2_pad.as_mut_ptr(), 0, core::mem::size_of_val(&(*to).di_v2_pad)); }
        xfs_log_dinode_to_disk_iext_counters(from, to);
    }
}

unsafe fn xlog_dinode_verify_extent_counts(mp: *mut xfs_mount, ldip: *mut xfs_log_dinode) -> i32 {
    unsafe {
        let (nextents, anextents);
        if xfs_log_dinode_has_large_extent_counts(ldip) {
            if !xfs_has_large_extent_counts(mp) || (*ldip).di_nrext64_pad != 0 { XFS_CORRUPTION_ERROR!("Bad log dinode large extent count format", XFS_ERRLEVEL_LOW, mp, ldip, core::mem::size_of::<xfs_log_dinode>()); xfs_alert(mp, "Bad inode 0x%llx, large extent counts %d, padding 0x%x", (*ldip).di_ino, xfs_has_large_extent_counts(mp), (*ldip).di_nrext64_pad); return -EFSCORRUPTED; }
            nextents = (*ldip).di_big_nextents; anextents = (*ldip).di_big_anextents;
        } else {
            if (*ldip).di_version == 3 && (*ldip).di_v3_pad != 0 { XFS_CORRUPTION_ERROR!("Bad log dinode di_v3_pad", XFS_ERRLEVEL_LOW, mp, ldip, core::mem::size_of::<xfs_log_dinode>()); xfs_alert(mp, "Bad inode 0x%llx, di_v3_pad 0x%llx", (*ldip).di_ino, (*ldip).di_v3_pad); return -EFSCORRUPTED; }
            nextents = (*ldip).di_nextents; anextents = (*ldip).di_anextents;
        }
        if nextents + anextents > (*ldip).di_nblocks { XFS_CORRUPTION_ERROR!("Bad log dinode extent counts", XFS_ERRLEVEL_LOW, mp, ldip, core::mem::size_of::<xfs_log_dinode>()); xfs_alert(mp, "Bad inode 0x%llx, large extent counts %d, nextents 0x%llx, anextents 0x%x, nblocks 0x%llx", (*ldip).di_ino, xfs_has_large_extent_counts(mp), nextents, anextents, (*ldip).di_nblocks); return -EFSCORRUPTED; }
        0
    }
}

#[inline]
unsafe fn xlog_recover_inode_dbroot(mp: *mut xfs_mount, src: *mut core::ffi::c_void, len: u32, dip: *mut xfs_dinode) -> i32 {
    unsafe { let dfork = XFS_DFORK_DPTR(dip); let dsize = XFS_DFORK_DSIZE(dip, mp); match (*dip).di_format { XFS_DINODE_FMT_BTREE => { xfs_bmbt_to_bmdr(mp, src, len, dfork, dsize); }, XFS_DINODE_FMT_META_BTREE => match be16_to_cpu((*dip).di_metatype) { XFS_METAFILE_RTRMAP => { xfs_rtrmapbt_to_disk(mp, src, len, dfork, dsize); return 0; }, XFS_METAFILE_RTREFCOUNT => { xfs_rtrefcountbt_to_disk(mp, src, len, dfork, dsize); return 0; }, _ => { ASSERT(false); return -EFSCORRUPTED; } }, _ => { ASSERT(false); return -EFSCORRUPTED; } } 0 }
}

// The large pass-2 replay routine is retained as a direct unsafe translation.
unsafe fn xlog_recover_inode_commit_pass2(log: *mut xlog, buffer_list: *mut list_head, item: *mut xlog_recover_item, current_lsn: xfs_lsn_t) -> i32 {
    unsafe {
        let mp = (*log).l_mp;
        let mut in_f: *mut xfs_inode_log_format;
        let mut need_free = false;
        let mut error;
        if (*item).ri_buf[0].iov_len == core::mem::size_of::<xfs_inode_log_format>() { in_f = (*item).ri_buf[0].iov_base as *mut _; } else { in_f = kmalloc_obj::<xfs_inode_log_format>(GFP_KERNEL | __GFP_NOFAIL); need_free = true; error = xfs_inode_item_format_convert(&mut (*item).ri_buf[0], in_f); if error != 0 { if need_free { kfree(in_f); } return error; } }
        if xlog_is_buffer_cancelled(log, (*in_f).ilf_blkno, (*in_f).ilf_len) { trace_xfs_log_recover_inode_cancel(log, in_f); if need_free { kfree(in_f); } return 0; }
        trace_xfs_log_recover_inode_recover(log, in_f);
        let mut bp: *mut xfs_buf = core::ptr::null_mut(); error = xfs_buf_read((*mp).m_ddev_targp, (*in_f).ilf_blkno, (*in_f).ilf_len, 0, &mut bp, &xfs_inode_buf_ops); if error != 0 { if need_free { kfree(in_f); } return error; }
        ASSERT((*in_f).ilf_fields & XFS_ILOG_CORE != 0); let dip = xfs_buf_offset(bp, (*in_f).ilf_boffset) as *mut xfs_dinode;
        if XFS_IS_CORRUPT(mp, !xfs_verify_magic16(bp, (*dip).di_magic)) { xfs_alert(mp, "%s: Bad inode magic number, dip = %p, dino bp = %p, ino = %lld", "xlog_recover_inode_commit_pass2", dip, bp, (*in_f).ilf_ino); xfs_buf_relse(bp); if need_free { kfree(in_f); } return -EFSCORRUPTED; }
        let ldip = (*item).ri_buf[1].iov_base as *mut xfs_log_dinode; if XFS_IS_CORRUPT(mp, (*ldip).di_magic != XFS_DINODE_MAGIC) { xfs_buf_relse(bp); if need_free { kfree(in_f); } return -EFSCORRUPTED; }
        if (*dip).di_version >= 3 { let lsn = be64_to_cpu((*dip).di_lsn); if lsn != 0 && lsn != u64::MAX && XFS_LSN_CMP(lsn, current_lsn) > 0 { error = 0; } else { error = 1; } } else { error = 1; }
        if error == 0 { xfs_buf_relse(bp); if need_free { kfree(in_f); } return xfs_recover_inode_owner_change(mp, dip, in_f, buffer_list); }
        xfs_log_dinode_to_disk(ldip, dip, current_lsn);
        let fields = (*in_f).ilf_fields; if fields & XFS_ILOG_DEV != 0 { xfs_dinode_put_rdev(dip, (*in_f).ilf_u.ilfu_rdev); }
        if (*in_f).ilf_size != 2 { let len = (*item).ri_buf[2].iov_len; let src = (*item).ri_buf[2].iov_base; match fields & XFS_ILOG_DFORK { XFS_ILOG_DDATA | XFS_ILOG_DEXT => memcpy(XFS_DFORK_DPTR(dip), src, len), XFS_ILOG_DBROOT => { error = xlog_recover_inode_dbroot(mp, src, len, dip); if error != 0 { xfs_buf_relse(bp); if need_free { kfree(in_f); } return error; } }, _ => {} } }
        if fields & XFS_ILOG_AFORK != 0 { let attr_index = if fields & XFS_ILOG_DFORK != 0 { 3 } else { 2 }; let len = (*item).ri_buf[attr_index].iov_len; let src = (*item).ri_buf[attr_index].iov_base; match fields & XFS_ILOG_AFORK { XFS_ILOG_ADATA | XFS_ILOG_AEXT => memcpy(XFS_DFORK_APTR(dip), src, len), XFS_ILOG_ABROOT => xfs_bmbt_to_bmdr(mp, src, len, XFS_DFORK_APTR(dip), XFS_DFORK_ASIZE(dip, mp)), _ => return -EFSCORRUPTED } }
        if fields & (XFS_ILOG_DOWNER | XFS_ILOG_AOWNER) != 0 && (*dip).di_mode != 0 { error = xfs_recover_inode_owner_change(mp, dip, in_f, buffer_list); if error != 0 { xfs_buf_relse(bp); if need_free { kfree(in_f); } return error; } }
        xfs_dinode_calc_crc(mp, dip); if !xfs_dinode_verify(mp, (*in_f).ilf_ino, dip).is_null() { xfs_buf_relse(bp); if need_free { kfree(in_f); } return -EFSCORRUPTED; }
        ASSERT((*bp).b_mount == mp); xfs_buf_delwri_queue(bp, buffer_list); xfs_buf_relse(bp); if need_free { kfree(in_f); } 0
    }
}

#[no_mangle]
pub static mut xlog_inode_item_ops: xlog_recover_item_ops = xlog_recover_item_ops { item_type: XFS_LI_INODE, ra_pass2: Some(xlog_recover_inode_ra_pass2), commit_pass2: Some(xlog_recover_inode_commit_pass2) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
