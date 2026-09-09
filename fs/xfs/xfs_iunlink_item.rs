// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2020-2022, Red Hat, Inc.
 * All Rights Reserved.
 */

// Dependencies are supplied by the surrounding XFS translation.

extern "C" {
    static mut xfs_iunlink_cache: *mut kmem_cache;
}

unsafe fn IUL_ITEM(lip: *mut xfs_log_item) -> *mut xfs_iunlink_item {
    container_of!(lip, xfs_iunlink_item, item)
}

unsafe fn xfs_iunlink_item_release(lip: *mut xfs_log_item) {
    let iup: *mut xfs_iunlink_item = IUL_ITEM(lip);

    xfs_perag_put((*iup).pag);
    kmem_cache_free(xfs_iunlink_cache, IUL_ITEM(lip));
}

unsafe fn xfs_iunlink_item_sort(lip: *mut xfs_log_item) -> u64 {
    I_INO((*IUL_ITEM(lip)).ip)
}

/*
 * Look up the inode cluster buffer and log the on-disk unlinked inode change
 * we need to make.
 */
unsafe fn xfs_iunlink_log_dinode(
    tp: *mut xfs_trans,
    iup: *mut xfs_iunlink_item,
) -> i32 {
    let ip: *mut xfs_inode = (*iup).ip;
    let mut dip: *mut xfs_dinode;
    let mut ibp: *mut xfs_buf = core::ptr::null_mut();
    let mut old_ptr: xfs_agino_t;
    let mut offset: i32;
    let mut error: i32;

    error = xfs_read_icluster(
        (*iup).pag,
        tp,
        (*ip).i_imap.im_agbno,
        &mut ibp,
    );
    if error != 0 {
        return error;
    }
    /*
     * Don't log the unlinked field on stale buffers as this may be the
     * transaction that frees the inode cluster and relogging the buffer
     * here will incorrectly remove the stale state.
     */
    if (*ibp).b_flags & XBF_STALE != 0 {
        xfs_trans_brelse(tp, ibp);
        return 0;
    }

    dip = xfs_buf_offset(ibp, (*ip).i_imap.im_boffset);

    /* Make sure the old pointer isn't garbage. */
    old_ptr = be32_to_cpu((*dip).di_next_unlinked);
    if old_ptr != (*iup).old_agino {
        xfs_inode_verifier_error(
            ip,
            -EFSCORRUPTED,
            __func__,
            dip,
            core::mem::size_of::<xfs_dinode>(),
            __this_address,
        );
        error = -EFSCORRUPTED;
        xfs_trans_brelse(tp, ibp);
        return error;
    }

    trace_xfs_iunlink_update_dinode(iup, old_ptr);

    (*dip).di_next_unlinked = cpu_to_be32((*iup).next_agino);
    offset = (*ip).i_imap.im_boffset
        + core::mem::offset_of!(xfs_dinode, di_next_unlinked) as i32;

    xfs_dinode_calc_crc((*tp).t_mountp, dip);
    xfs_trans_inode_buf(tp, ibp);
    xfs_trans_log_buf(
        tp,
        ibp,
        offset,
        offset + core::mem::size_of::<xfs_agino_t>() as i32 - 1,
    );
    0
}

/*
 * On precommit, we grab the inode cluster buffer for the inode number we were
 * passed, then update the next unlinked field for that inode in the buffer and
 * log the buffer. This ensures that the inode cluster buffer was logged in the
 * correct order w.r.t. other inode cluster buffers. We can then remove the
 * iunlink item from the transaction and release it as it is has now served it's
 * purpose.
 */
unsafe fn xfs_iunlink_item_precommit(
    tp: *mut xfs_trans,
    lip: *mut xfs_log_item,
) -> i32 {
    let iup: *mut xfs_iunlink_item = IUL_ITEM(lip);
    let error: i32;

    error = xfs_iunlink_log_dinode(tp, iup);
    list_del(&mut (*lip).li_trans);
    xfs_iunlink_item_release(lip);
    error
}

static mut xfs_iunlink_item_ops: xfs_item_ops = xfs_item_ops {
    iop_release: Some(xfs_iunlink_item_release),
    iop_sort: Some(xfs_iunlink_item_sort),
    iop_precommit: Some(xfs_iunlink_item_precommit),
};

/*
 * Initialize the inode log item for a newly allocated (in-core) inode.
 *
 * Inode extents can only reside within an AG. Hence specify the starting
 * block for the inode chunk by offset within an AG as well as the
 * length of the allocated extent.
 *
 * This joins the item to the transaction and marks it dirty so
 * that we don't need a separate call to do this, nor does the
 * caller need to know anything about the iunlink item.
 */
unsafe fn xfs_iunlink_log_inode(
    tp: *mut xfs_trans,
    ip: *mut xfs_inode,
    pag: *mut xfs_perag,
    next_agino: xfs_agino_t,
) -> i32 {
    let mp: *mut xfs_mount = (*tp).t_mountp;
    let iup: *mut xfs_iunlink_item;

    ASSERT(xfs_verify_agino_or_null(pag, next_agino));
    ASSERT(xfs_verify_agino_or_null(pag, (*ip).i_next_unlinked));

    /*
     * Since we're updating a linked list, we should never find that the
     * current pointer is the same as the new value, unless we're
     * terminating the list.
     */
    if (*ip).i_next_unlinked == next_agino {
        if next_agino != NULLAGINO {
            return -EFSCORRUPTED;
        }
        return 0;
    }

    iup = kmem_cache_zalloc(xfs_iunlink_cache, GFP_KERNEL | __GFP_NOFAIL);
    xfs_log_item_init(
        mp,
        &mut (*iup).item,
        XFS_LI_IUNLINK,
        &xfs_iunlink_item_ops,
    );

    (*iup).ip = ip;
    (*iup).next_agino = next_agino;
    (*iup).old_agino = (*ip).i_next_unlinked;
    (*iup).pag = xfs_perag_hold(pag);

    xfs_trans_add_item(tp, &mut (*iup).item);
    (*tp).t_flags |= XFS_TRANS_DIRTY;
    set_bit(XFS_LI_DIRTY, &mut (*iup).item.li_flags);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
