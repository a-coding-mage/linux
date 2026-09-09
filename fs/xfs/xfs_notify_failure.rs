// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2022 Fujitsu.  All Rights Reserved.
 */

// External declarations and constants are supplied by the corresponding XFS
// and Linux bindings.

#[repr(C)]
struct xfs_failure_info {
    startblock: xfs_agblock_t,
    blockcount: xfs_extlen_t,
    mf_flags: i32,
    want_shutdown: bool,
}

unsafe fn xfs_failure_pgoff(
    mp: *mut xfs_mount,
    rec: *const xfs_rmap_irec,
    notify: *const xfs_failure_info,
) -> pgoff_t {
    let mut pos: loff_t = XFS_FSB_TO_B(unsafe { (*mp).m_sb }, unsafe { (*rec).rm_offset });

    if unsafe { (*notify).startblock > (*rec).rm_startblock } {
        pos += XFS_FSB_TO_B(
            unsafe { (*mp).m_sb },
            unsafe { (*notify).startblock - (*rec).rm_startblock },
        );
    }
    (pos >> PAGE_SHIFT) as pgoff_t
}

unsafe fn xfs_failure_pgcnt(
    mp: *mut xfs_mount,
    rec: *const xfs_rmap_irec,
    notify: *const xfs_failure_info,
) -> c_ulong {
    let end_rec = unsafe { (*rec).rm_startblock + (*rec).rm_blockcount };
    let end_notify = unsafe { (*notify).startblock + (*notify).blockcount };
    let start_cross = core::cmp::max(unsafe { (*rec).rm_startblock }, unsafe { (*notify).startblock });
    let end_cross = core::cmp::min(end_rec, end_notify);

    (XFS_FSB_TO_B(unsafe { (*mp).m_sb }, end_cross - start_cross) >> PAGE_SHIFT) as c_ulong
}

unsafe extern "C" fn xfs_dax_failure_fn(
    cur: *mut xfs_btree_cur,
    rec: *const xfs_rmap_irec,
    data: *mut c_void,
) -> i32 {
    let mp = unsafe { (*cur).bc_mp };
    let notify = data as *mut xfs_failure_info;
    let mut ip: *mut xfs_inode = core::ptr::null_mut();
    let mut mapping: *mut address_space;
    let mut error: i32 = 0;

    if unsafe { XFS_RMAP_NON_INODE_OWNER((*rec).rm_owner) }
        || unsafe { ((*rec).rm_flags & (XFS_RMAP_ATTR_FORK | XFS_RMAP_BMBT_BLOCK)) != 0 }
    {
        /* Continue the query because this isn't a failure. */
        if unsafe { ((*notify).mf_flags & MF_MEM_PRE_REMOVE) != 0 } {
            return 0;
        }
        unsafe { (*notify).want_shutdown = true };
        return 0;
    }

    /* Get files that incore, filter out others that are not in use. */
    error = unsafe {
        xfs_iget(
            mp,
            (*cur).bc_tp,
            (*rec).rm_owner,
            XFS_IGET_INCORE,
            0,
            &mut ip,
        )
    };
    /* Continue the rmap query if the inode isn't incore */
    if error == -ENODATA {
        return 0;
    }
    if error != 0 {
        unsafe { (*notify).want_shutdown = true };
        return 0;
    }

    mapping = unsafe { VFS_I(ip).i_mapping };
    let pgoff = unsafe { xfs_failure_pgoff(mp, rec, notify) };
    let pgcnt = unsafe { xfs_failure_pgcnt(mp, rec, notify) };

    /* Continue the rmap query if the inode isn't a dax file. */
    if unsafe { dax_mapping(mapping) } {
        error = unsafe { mf_dax_kill_procs(mapping, pgoff, pgcnt, (*notify).mf_flags) };
    }

    /* Invalidate the cache in dax pages. */
    if unsafe { ((*notify).mf_flags & MF_MEM_PRE_REMOVE) != 0 } {
        unsafe { invalidate_inode_pages2_range(mapping, pgoff, pgoff + pgcnt - 1) };
    }

    unsafe {
        fserror_report_data_lost(
            VFS_I(ip),
            (pgoff as u64) << PAGE_SHIFT,
            (pgcnt as u64) << PAGE_SHIFT,
            GFP_NOFS,
        );
        xfs_irele(ip);
    }
    error
}

unsafe fn xfs_dax_notify_failure_freeze(mp: *mut xfs_mount) -> i32 {
    let sb = unsafe { (*mp).m_super };
    let error = unsafe { freeze_super(sb, FREEZE_HOLDER_KERNEL, core::ptr::null_mut()) };
    if error != 0 {
        unsafe { xfs_emerg(mp, c"already frozen by kernel, err=%d".as_ptr(), error) };
    }
    error
}

unsafe fn xfs_dax_notify_failure_thaw(mp: *mut xfs_mount, kernel_frozen: bool) {
    let sb = unsafe { (*mp).m_super };
    if kernel_frozen {
        let error = unsafe { thaw_super(sb, FREEZE_HOLDER_KERNEL, core::ptr::null_mut()) };
        if error != 0 {
            unsafe { xfs_emerg(mp, c"still frozen after notify failure, err=%d".as_ptr(), error) };
        }
    }

    /*
     * Also thaw userspace call anyway because the device is about to be
     * removed immediately.
     */
    unsafe { thaw_super(sb, FREEZE_HOLDER_USERSPACE, core::ptr::null_mut()) };
}

unsafe fn xfs_dax_translate_range(
    btp: *mut xfs_buftarg,
    mut offset: u64,
    mut len: u64,
    daddr: *mut xfs_daddr_t,
    bblen: *mut u64,
) -> i32 {
    let dev_start = unsafe { (*btp).bt_dax_part_off };
    let dev_len = BBTOB(unsafe { (*btp).bt_nr_sectors });
    let dev_end = dev_start + dev_len - 1;

    /* Notify failure on the whole device. */
    if offset == 0 && len == u64::MAX {
        offset = dev_start;
        len = dev_len;
    }

    /* Ignore the range out of filesystem area */
    if offset + len - 1 < dev_start || offset > dev_end {
        return -ENXIO;
    }

    /* Calculate the real range when it touches the boundary */
    if offset > dev_start {
        offset -= dev_start;
    } else {
        len -= dev_start - offset;
        offset = 0;
    }
    if offset + len - 1 > dev_end {
        len = dev_end - offset + 1;
    }

    unsafe {
        *daddr = BTOBB(offset);
        *bblen = BTOBB(len);
    }
    0
}

unsafe fn xfs_dax_notify_logdev_failure(mp: *mut xfs_mount, offset: u64, len: u64, mf_flags: i32) -> i32 {
    let mut daddr = 0;
    let mut bblen = 0;
    let error = unsafe { xfs_dax_translate_range((*mp).m_logdev_targp, offset, len, &mut daddr, &mut bblen) };
    if error != 0 { return error; }
    unsafe { xfs_healthmon_report_media(mp, XFS_DEV_LOG, daddr, bblen) };
    if (mf_flags & MF_MEM_PRE_REMOVE) != 0 { return 0; }
    unsafe { xfs_err(mp, c"ondisk log corrupt, shutting down fs!".as_ptr()) };
    unsafe { xfs_force_shutdown(mp, SHUTDOWN_CORRUPT_ONDISK) };
    -EFSCORRUPTED
}

unsafe fn xfs_dax_notify_dev_failure(
    mp: *mut xfs_mount, offset: u64, len: u64, mf_flags: i32, type_: xfs_group_type,
) -> i32 {
    let mut notify = xfs_failure_info { startblock: 0, blockcount: 0, mf_flags, want_shutdown: false };
    let mut tp: *mut xfs_trans = core::ptr::null_mut();
    let mut cur: *mut xfs_btree_cur = core::ptr::null_mut();
    let mut error = 0;
    let mut kernel_frozen = false;
    let mut start_gno;
    let mut end_gno;
    let (mut start_bno, mut end_bno);
    let mut daddr = 0;
    let mut bblen = 0;
    let mut xg: *mut xfs_group = core::ptr::null_mut();

    error = unsafe { xfs_dax_translate_range(xfs_group_type_buftarg(mp, type_), offset, len, &mut daddr, &mut bblen) };
    if error != 0 { return error; }
    unsafe { xfs_healthmon_report_media(mp, if type_ == XG_TYPE_RTG { XFS_DEV_RT } else { XFS_DEV_DATA }, daddr, bblen) };
    if !unsafe { xfs_has_rmapbt(mp) } { unsafe { xfs_debug(mp, c"notify_failure() needs rmapbt enabled!".as_ptr()) }; return -EOPNOTSUPP; }

    if type_ == XG_TYPE_RTG {
        start_bno = unsafe { xfs_daddr_to_rtb(mp, daddr) };
        end_bno = unsafe { xfs_daddr_to_rtb(mp, daddr + bblen - 1) };
    } else {
        start_bno = XFS_DADDR_TO_FSB(mp, daddr);
        end_bno = XFS_DADDR_TO_FSB(mp, daddr + bblen - 1);
    }
    if (mf_flags & MF_MEM_PRE_REMOVE) != 0 {
        unsafe { xfs_info(mp, c"Device is about to be removed!".as_ptr()) };
        kernel_frozen = unsafe { xfs_dax_notify_failure_freeze(mp) == 0 };
    }

    tp = unsafe { xfs_trans_alloc_empty(mp) };
    start_gno = unsafe { xfs_fsb_to_gno(mp, start_bno, type_) };
    end_gno = unsafe { xfs_fsb_to_gno(mp, end_bno, type_) };
    while {
        xg = unsafe { xfs_group_next_range(mp, xg, start_gno, end_gno, type_) };
        !xg.is_null()
    } {
        let mut agf_bp: *mut xfs_buf = core::ptr::null_mut();
        let mut rtg: *mut xfs_rtgroup = core::ptr::null_mut();
        let mut ri_low = xfs_rmap_irec::default();
        let mut ri_high: xfs_rmap_irec = core::mem::zeroed();
        if type_ == XG_TYPE_AG {
            let pag = to_perag(xg);
            error = xfs_alloc_read_agf(pag, tp, 0, &mut agf_bp);
            if error != 0 { xfs_perag_rele(pag); break; }
            cur = xfs_rmapbt_init_cursor(mp, tp, agf_bp, pag);
        } else {
            rtg = to_rtg(xg);
            xfs_rtgroup_lock(rtg, XFS_RTGLOCK_RMAP);
            cur = xfs_rtrmapbt_init_cursor(tp, rtg);
        }
        core::ptr::write_bytes(&mut ri_high, 0xff, 1);
        if (*xg).xg_gno == start_gno { ri_low.rm_startblock = xfs_fsb_to_gbno(mp, start_bno, type_); }
        if (*xg).xg_gno == end_gno { ri_high.rm_startblock = xfs_fsb_to_gbno(mp, end_bno, type_); }
        notify.startblock = ri_low.rm_startblock;
        notify.blockcount = core::cmp::min((*xg).xg_block_count, ri_high.rm_startblock + 1) - ri_low.rm_startblock;
        error = xfs_rmap_query_range(cur, &ri_low, &ri_high, Some(xfs_dax_failure_fn), &mut notify as *mut _ as *mut c_void);
        xfs_btree_del_cursor(cur, error);
        if !agf_bp.is_null() { xfs_trans_brelse(tp, agf_bp); }
        if !rtg.is_null() { xfs_rtgroup_unlock(rtg, XFS_RTGLOCK_RMAP); }
        if error != 0 { xfs_group_rele(xg); break; }
    }
    xfs_trans_cancel(tp);
    if (mf_flags & MF_MEM_PRE_REMOVE) != 0 { xfs_force_shutdown(mp, SHUTDOWN_FORCE_UMOUNT); }
    else if error != 0 || notify.want_shutdown { xfs_force_shutdown(mp, SHUTDOWN_CORRUPT_ONDISK); if error == 0 { error = -EFSCORRUPTED; } }
    if (mf_flags & MF_MEM_PRE_REMOVE) != 0 { xfs_dax_notify_failure_thaw(mp, kernel_frozen); }
    error
}

unsafe extern "C" fn xfs_dax_notify_failure(dax_dev: *mut dax_device, offset: u64, len: u64, mf_flags: i32) -> i32 {
    let mp = dax_holder(dax_dev);
    if unsafe { ((*(*mp).m_super).s_flags & SB_BORN) == 0 } { unsafe { xfs_warn(mp, c"filesystem is not ready for notify_failure()!".as_ptr()) }; return -EIO; }
    if unsafe { (*mp).m_logdev_targp != (*mp).m_ddev_targp && (*(*mp).m_logdev_targp).bt_daxdev == dax_dev } {
        return unsafe { xfs_dax_notify_logdev_failure(mp, offset, len, mf_flags) };
    }
    unsafe { xfs_dax_notify_dev_failure(mp, offset, len, mf_flags, if !(*mp).m_rtdev_targp.is_null() && (*(*mp).m_rtdev_targp).bt_daxdev == dax_dev { XG_TYPE_RTG } else { XG_TYPE_AG }) }
}

#[repr(C)]
pub struct dax_holder_operations {
    pub notify_failure: Option<unsafe extern "C" fn(*mut dax_device, u64, u64, i32) -> i32>,
}

#[no_mangle]
pub static xfs_dax_holder_operations: dax_holder_operations = dax_holder_operations {
    notify_failure: Some(xfs_dax_notify_failure),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
