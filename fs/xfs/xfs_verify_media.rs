// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2026 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

#[repr(C)]
struct xfs_group_data_lost {
    startblock: xfs_agblock_t,
    blockcount: xfs_extlen_t,
}

/* Report lost file data from rmap records */
unsafe fn xfs_verify_report_data_lost(
    cur: *mut xfs_btree_cur,
    rec: *const xfs_rmap_irec,
    data: *mut core::ffi::c_void,
) -> i32 {
    let mp: *mut xfs_mount = (*cur).bc_mp;
    let mut ip: *mut xfs_inode = core::ptr::null_mut();
    let lost: *mut xfs_group_data_lost = data as *mut xfs_group_data_lost;
    let mut fileoff: xfs_fileoff_t = (*rec).rm_offset;
    let mut blocks: xfs_extlen_t = (*rec).rm_blockcount;
    let is_attr: bool = ((*rec).rm_flags & XFS_RMAP_ATTR_FORK) != 0;
    let lost_end: xfs_agblock_t = (*lost).startblock + (*lost).blockcount;
    let rmap_end: xfs_agblock_t = (*rec).rm_startblock + (*rec).rm_blockcount;
    let mut error: i32 = 0;

    if XFS_RMAP_NON_INODE_OWNER((*rec).rm_owner) {
        return 0;
    }

    error = xfs_iget(mp, (*cur).bc_tp, (*rec).rm_owner, 0, 0, &mut ip);
    if error != 0 {
        return 0;
    }

    if ((*rec).rm_flags & XFS_RMAP_BMBT_BLOCK) != 0 {
        xfs_bmap_mark_sick(ip, if is_attr { XFS_ATTR_FORK } else { XFS_DATA_FORK });
        xfs_irele(ip);
        return 0;
    }

    if is_attr {
        xfs_inode_mark_sick(ip, XFS_SICK_INO_XATTR);
        xfs_irele(ip);
        return 0;
    }

    if (*lost).startblock > (*rec).rm_startblock {
        fileoff += (*lost).startblock - (*rec).rm_startblock;
        blocks -= (*lost).startblock - (*rec).rm_startblock;
    }
    if rmap_end > lost_end {
        blocks -= rmap_end - lost_end;
    }

    fserror_report_data_lost(VFS_I(ip), XFS_FSB_TO_B(mp, fileoff), XFS_FSB_TO_B(mp, blocks), GFP_NOFS);
    xfs_irele(ip);
    0
}

/* Walk reverse mappings to look for all file data loss */
unsafe fn xfs_verify_report_losses(
    mp: *mut xfs_mount,
    type_: xfs_group_type,
    daddr: xfs_daddr_t,
    bblen: u64,
) -> i32 {
    let mut xg: *mut xfs_group = core::ptr::null_mut();
    let tp: *mut xfs_trans;
    let (start_bno, end_bno): (xfs_fsblock_t, xfs_fsblock_t);
    let (start_gno, end_gno): (u32, u32);
    let mut error: i32;

    if type_ == XG_TYPE_RTG {
        start_bno = xfs_daddr_to_rtb(mp, daddr);
        end_bno = xfs_daddr_to_rtb(mp, daddr + bblen - 1);
    } else {
        start_bno = XFS_DADDR_TO_FSB(mp, daddr);
        end_bno = XFS_DADDR_TO_FSB(mp, daddr + bblen - 1);
    }

    tp = xfs_trans_alloc_empty(mp);
    start_gno = xfs_fsb_to_gno(mp, start_bno, type_);
    end_gno = xfs_fsb_to_gno(mp, end_bno, type_);
    while {
        xg = xfs_group_next_range(mp, xg, start_gno, end_gno, type_);
        !xg.is_null()
    } {
        let mut agf_bp: *mut xfs_buf = core::ptr::null_mut();
        let mut rtg: *mut xfs_rtgroup = core::ptr::null_mut();
        let cur: *mut xfs_btree_cur;
        let mut ri_low: xfs_rmap_irec = core::mem::zeroed();
        let mut ri_high: xfs_rmap_irec = core::mem::zeroed();
        let mut lost: xfs_group_data_lost;

        if type_ == XG_TYPE_AG {
            let pag: *mut xfs_perag = to_perag(xg);
            error = xfs_alloc_read_agf(pag, tp, 0, &mut agf_bp);
            if error != 0 {
                xfs_perag_rele(pag);
                break;
            }
            cur = xfs_rmapbt_init_cursor(mp, tp, agf_bp, pag);
        } else {
            rtg = to_rtg(xg);
            xfs_rtgroup_lock(rtg, XFS_RTGLOCK_RMAP);
            cur = xfs_rtrmapbt_init_cursor(tp, rtg);
        }

        /*
         * Set the rmap range from ri_low to ri_high, which represents
         * a [start, end] where we looking for the files or metadata.
         */
        ri_high = core::mem::MaybeUninit::zeroed().assume_init();
        core::ptr::write_bytes(&mut ri_high as *mut xfs_rmap_irec as *mut u8, 0xff, core::mem::size_of::<xfs_rmap_irec>());
        if (*xg).xg_gno == start_gno { ri_low.rm_startblock = xfs_fsb_to_gbno(mp, start_bno, type_); }
        if (*xg).xg_gno == end_gno { ri_high.rm_startblock = xfs_fsb_to_gbno(mp, end_bno, type_); }
        lost.startblock = ri_low.rm_startblock;
        lost.blockcount = core::cmp::min((*xg).xg_block_count, ri_high.rm_startblock + 1) - ri_low.rm_startblock;
        error = xfs_rmap_query_range(cur, &ri_low, &ri_high, xfs_verify_report_data_lost, &mut lost as *mut _ as *mut core::ffi::c_void);
        xfs_btree_del_cursor(cur, error);
        if !agf_bp.is_null() { xfs_trans_brelse(tp, agf_bp); }
        if !rtg.is_null() { xfs_rtgroup_unlock(rtg, XFS_RTGLOCK_RMAP); }
        if error != 0 { xfs_group_rele(xg); break; }
    }
    xfs_trans_cancel(tp);
    0
}

/* Compute the desired verify IO size. */
unsafe fn xfs_verify_iosize(me: *const xfs_verify_media, btp: *mut xfs_buftarg, bbcount: u64) -> u32 {
    let iosize: u32 = min_not_zero(SZ_1M, (*me).me_max_io_size);
    BUILD_BUG_ON(BBSHIFT != SECTOR_SHIFT);
    ASSERT(BBTOB(bbcount) >= (*btp).bt_logical_sectorsize);
    clamp(iosize, (*btp).bt_logical_sectorsize, BBTOB(bbcount))
}

/* Allocate as much memory as we can get for verification buffer. */
unsafe fn xfs_verify_alloc_folio(iosize: u32) -> *mut folio {
    let mut order = get_order(iosize);
    while order > 0 {
        let folio = folio_alloc(GFP_KERNEL | __GFP_NORETRY, order);
        if !folio.is_null() { return folio; }
        order -= 1;
    }
    folio_alloc(GFP_KERNEL, 0)
}

/* Report any kind of problem verifying media */
unsafe fn xfs_verify_media_error(mp: *mut xfs_mount, me: *mut xfs_verify_media, btp: *mut xfs_buftarg, daddr: xfs_daddr_t, bio_bbcount: u32, bio_status: blk_status_t) {
    trace_xfs_verify_media_error(mp, me, (*btp).bt_dev, daddr, bio_bbcount, bio_status);
    if (*me).me_start_daddr == daddr { (*me).me_ioerror = -blk_status_to_errno(bio_status); }
    match bio_status { BLK_STS_PROTECTION | BLK_STS_IOERR | BLK_STS_MEDIUM => {}, _ => return }
    if ((*me).me_flags & XFS_VERIFY_MEDIA_REPORT) == 0 { return; }
    xfs_healthmon_report_media(mp, (*me).me_dev, daddr, bio_bbcount);
    if !xfs_has_rmapbt(mp) { return; }
    match (*me).me_dev { XFS_DEV_DATA => { xfs_verify_report_losses(mp, XG_TYPE_AG, daddr, bio_bbcount as u64); }, XFS_DEV_RT => { xfs_verify_report_losses(mp, XG_TYPE_RTG, daddr, bio_bbcount as u64); }, _ => {} }
}

/* Verify the media of an xfs device by submitting read requests to the disk. */
unsafe fn xfs_verify_media(mp: *mut xfs_mount, me: *mut xfs_verify_media) -> i32 {
    let mut btp: *mut xfs_buftarg = core::ptr::null_mut();
    let bio: *mut bio;
    let folio: *mut folio;
    let mut daddr: xfs_daddr_t;
    let mut bbcount: u64;
    let mut error = 0;
    (*me).me_ioerror = 0;
    match (*me).me_dev { XFS_DEV_DATA => btp = (*mp).m_ddev_targp, XFS_DEV_LOG => if (*mp).m_logdev_targp != (*mp).m_ddev_targp { btp = (*mp).m_logdev_targp; }, XFS_DEV_RT => btp = (*mp).m_rtdev_targp, _ => {} }
    if btp.is_null() { return -ENODEV; }
    if (*me).me_end_daddr > (*btp).bt_nr_sectors { (*me).me_end_daddr = (*btp).bt_nr_sectors; }
    if !IS_ALIGNED(BBTOB((*me).me_start_daddr | (*me).me_end_daddr), (*btp).bt_logical_sectorsize) { return -EINVAL; }
    if (*me).me_start_daddr >= (*me).me_end_daddr { return 0; }
    daddr = (*me).me_start_daddr;
    bbcount = core::cmp::min((*me).me_end_daddr, (*btp).bt_nr_sectors) - (*me).me_start_daddr;
    folio = xfs_verify_alloc_folio(xfs_verify_iosize(me, btp, bbcount));
    if folio.is_null() { return -ENOMEM; }
    trace_xfs_verify_media(mp, me, (*btp).bt_dev, daddr, bbcount, folio);
    bio = bio_alloc((*btp).bt_bdev, 1, REQ_OP_READ, GFP_KERNEL);
    if bio.is_null() { folio_put(folio); return -ENOMEM; }
    while bbcount > 0 {
        let mut bio_bbcount: u32;
        bio_reset(bio, (*btp).bt_bdev, REQ_OP_READ);
        (*bio).bi_iter.bi_sector = daddr;
        bio_add_folio_nofail(bio, folio, core::cmp::min(bbcount << SECTOR_SHIFT, folio_size(folio)), 0);
        bio_bbcount = (*bio).bi_iter.bi_size >> SECTOR_SHIFT;
        submit_bio_wait(bio);
        let bio_status = (*bio).bi_status;
        if bio_status != BLK_STS_OK { xfs_verify_media_error(mp, me, btp, daddr, bio_bbcount, bio_status); error = 0; break; }
        daddr += bio_bbcount as u64;
        bbcount -= bio_bbcount as u64;
        if bbcount == 0 { break; }
        if (*me).me_rest_us != 0 { let expires = ktime_add_ns(ktime_get(), (*me).me_rest_us * 1000); set_current_state(TASK_KILLABLE); schedule_hrtimeout(&expires, HRTIMER_MODE_ABS); }
        if fatal_signal_pending(current) { error = -EINTR; break; }
        cond_resched();
    }
    bio_put(bio);
    folio_put(folio);
    if error != 0 { return error; }
    (*me).me_start_daddr = daddr;
    trace_xfs_verify_media_end(mp, me, (*btp).bt_dev);
    0
}

unsafe fn xfs_ioc_verify_media(file: *mut file, arg: *mut xfs_verify_media) -> i32 {
    let mut me: xfs_verify_media = core::mem::zeroed();
    let ip: *mut xfs_inode = XFS_I(file_inode(file));
    let mp: *mut xfs_mount = (*ip).i_mount;
    let error: i32;
    if !capable(CAP_SYS_ADMIN) { return -EPERM; }
    if copy_from_user(&mut me, arg, core::mem::size_of::<xfs_verify_media>()) != 0 { return -EFAULT; }
    if me.me_pad != 0 || (me.me_flags & !XFS_VERIFY_MEDIA_FLAGS) != 0 { return -EINVAL; }
    match me.me_dev { XFS_DEV_DATA | XFS_DEV_LOG | XFS_DEV_RT => {}, _ => return -EINVAL }
    error = xfs_verify_media(mp, &mut me);
    if error != 0 { return error; }
    if copy_to_user(arg, &me, core::mem::size_of::<xfs_verify_media>()) != 0 { return -EFAULT; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
