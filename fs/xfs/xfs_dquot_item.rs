// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2003 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// C dependencies supplied by the surrounding XFS translation unit.

#[inline]
unsafe fn dquot_item(lip: *mut xfs_log_item) -> *mut xfs_dq_logitem {
    // Equivalent to container_of(lip, struct xfs_dq_logitem, qli_item).
    lip as *mut xfs_dq_logitem
}

/* returns the number of iovecs needed to log the given dquot item. */
unsafe fn xfs_qm_dquot_logitem_size(
    _lip: *mut xfs_log_item,
    nvecs: *mut ::core::ffi::c_int,
    nbytes: *mut ::core::ffi::c_int,
) {
    *nvecs += 2;
    *nbytes += (core::mem::size_of::<xfs_dq_logformat>()
        + core::mem::size_of::<xfs_disk_dquot>()) as ::core::ffi::c_int;
}

/* fills in the vector of log iovecs for the given dquot log item. */
unsafe fn xfs_qm_dquot_logitem_format(
    lip: *mut xfs_log_item,
    lfb: *mut xlog_format_buf,
) {
    let mut ddq: xfs_disk_dquot = core::mem::zeroed();
    let qlip = dquot_item(lip);
    let qlf: *mut xfs_dq_logformat;

    qlf = xlog_format_start(lfb, XLOG_REG_TYPE_QFORMAT);
    (*qlf).qlf_type = XFS_LI_DQUOT;
    (*qlf).qlf_size = 2;
    (*qlf).qlf_id = (*(*qlip).qli_dquot).q_id;
    (*qlf).qlf_blkno = (*(*qlip).qli_dquot).q_blkno;
    (*qlf).qlf_len = 1;
    (*qlf).qlf_boffset = (*(*qlip).qli_dquot).q_bufoffset;
    xlog_format_commit(lfb, core::mem::size_of::<xfs_dq_logformat>());

    xfs_dquot_to_disk(&mut ddq, (*qlip).qli_dquot);
    xlog_format_copy(
        lfb,
        XLOG_REG_TYPE_DQUOT,
        &ddq,
        core::mem::size_of::<xfs_disk_dquot>(),
    );
}

/* Increment the pin count of the given dquot. */
unsafe fn xfs_qm_dquot_logitem_pin(lip: *mut xfs_log_item) {
    let dqp = (*dquot_item(lip)).qli_dquot;
    ASSERT(XFS_DQ_IS_LOCKED(dqp));
    atomic_inc(&mut (*dqp).q_pincount);
}

/* Decrement the pin count of the given dquot. */
unsafe fn xfs_qm_dquot_logitem_unpin(lip: *mut xfs_log_item, _remove: ::core::ffi::c_int) {
    let dqp = (*dquot_item(lip)).qli_dquot;
    ASSERT(atomic_read(&(*dqp).q_pincount) > 0);
    if atomic_dec_and_test(&mut (*dqp).q_pincount) {
        wake_up(&mut (*dqp).q_pinwait);
    }
}

/* This is called to wait for the given dquot to be unpinned. */
pub unsafe fn xfs_qm_dqunpin_wait(dqp: *mut xfs_dquot) {
    ASSERT(XFS_DQ_IS_LOCKED(dqp));
    if atomic_read(&(*dqp).q_pincount) == 0 {
        return;
    }
    xfs_log_force((*dqp).q_mount, 0);
    wait_event(&mut (*dqp).q_pinwait, atomic_read(&(*dqp).q_pincount) == 0);
}

unsafe fn xfs_qm_dquot_logitem_push(
    lip: *mut xfs_log_item,
    buffer_list: *mut list_head,
) -> uint {
    let qlip = dquot_item(lip);
    let dqp = (*qlip).qli_dquot;
    let mut bp: *mut xfs_buf = core::ptr::null_mut();
    let ailp = (*lip).li_ailp;
    let mut rval = XFS_ITEM_SUCCESS;
    let mut error: ::core::ffi::c_int;

    if atomic_read(&(*dqp).q_pincount) > 0 { return XFS_ITEM_PINNED; }
    if !mutex_trylock(&mut (*dqp).q_qlock) { return XFS_ITEM_LOCKED; }
    if atomic_read(&(*dqp).q_pincount) > 0 {
        rval = XFS_ITEM_PINNED;
        goto_out_unlock!();
    }
    if !xfs_dqflock_nowait(dqp) {
        rval = XFS_ITEM_FLUSHING;
        goto_out_unlock!();
    }

    spin_unlock(&mut (*ailp).ail_lock);
    error = xfs_dquot_use_attached_buf(dqp, &mut bp);
    if error == -EAGAIN {
        xfs_dqfunlock(dqp);
        rval = XFS_ITEM_LOCKED;
        goto_relock_ail!();
    }
    error = xfs_qm_dqflush(dqp, bp);
    if error == 0 && !xfs_buf_delwri_queue(bp, buffer_list) {
        rval = XFS_ITEM_FLUSHING;
    }
    xfs_buf_relse(bp);

    goto_relock_ail!();
    spin_lock(&mut (*ailp).ail_lock);
    goto_out_unlock!();
    mutex_unlock(&mut (*dqp).q_qlock);
    rval
}

unsafe fn xfs_qm_dquot_logitem_release(lip: *mut xfs_log_item) {
    let dqp = (*dquot_item(lip)).qli_dquot;
    ASSERT(XFS_DQ_IS_LOCKED(dqp));
    mutex_unlock(&mut (*dqp).q_qlock);
}

unsafe fn xfs_qm_dquot_logitem_committing(lip: *mut xfs_log_item, _seq: xfs_csn_t) {
    xfs_qm_dquot_logitem_release(lip);
}

#[cfg(feature = "DEBUG_EXPENSIVE")]
unsafe fn xfs_qm_dquot_logitem_precommit_check(dqp: *mut xfs_dquot) {
    let mp = (*dqp).q_mount;
    let mut ddq: xfs_disk_dquot = core::mem::zeroed();
    xfs_dquot_to_disk(&mut ddq, dqp);
    let fa = xfs_dquot_verify(mp, &ddq, (*dqp).q_id);
    if !fa.is_null() {
        XFS_CORRUPTION_ERROR!("Bad dquot during logging", XFS_ERRLEVEL_LOW, mp, &ddq, core::mem::size_of::<xfs_disk_dquot>());
        xfs_alert(mp, "Metadata corruption detected at %pS, dquot 0x%x", fa, (*dqp).q_id);
        xfs_force_shutdown(mp, SHUTDOWN_CORRUPT_INCORE);
        ASSERT(fa.is_null());
    }
}

#[cfg(not(feature = "DEBUG_EXPENSIVE"))]
unsafe fn xfs_qm_dquot_logitem_precommit_check(_dqp: *mut xfs_dquot) {}

unsafe fn xfs_qm_dquot_logitem_precommit(
    tp: *mut xfs_trans,
    lip: *mut xfs_log_item,
) -> ::core::ffi::c_int {
    let qlip = dquot_item(lip);
    let dqp = (*qlip).qli_dquot;
    xfs_qm_dquot_logitem_precommit_check(dqp);
    xfs_dquot_attach_buf(tp, dqp)
}

static xfs_dquot_item_ops: xfs_item_ops = xfs_item_ops {
    iop_size: Some(xfs_qm_dquot_logitem_size),
    iop_precommit: Some(xfs_qm_dquot_logitem_precommit),
    iop_format: Some(xfs_qm_dquot_logitem_format),
    iop_pin: Some(xfs_qm_dquot_logitem_pin),
    iop_unpin: Some(xfs_qm_dquot_logitem_unpin),
    iop_release: Some(xfs_qm_dquot_logitem_release),
    iop_committing: Some(xfs_qm_dquot_logitem_committing),
    iop_push: Some(xfs_qm_dquot_logitem_push),
};

/* Initialize the dquot log item for a newly allocated dquot. */
pub unsafe fn xfs_qm_dquot_logitem_init(dqp: *mut xfs_dquot) {
    let lp = &mut (*dqp).q_logitem;
    xfs_log_item_init((*dqp).q_mount, &mut lp.qli_item, XFS_LI_DQUOT, &xfs_dquot_item_ops);
    spin_lock_init(&mut lp.qli_lock);
    lp.qli_dquot = dqp;
    lp.qli_dirty = false;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
