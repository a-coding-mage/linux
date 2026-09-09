// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2008-2010, 2013 Dave Chinner
 * All Rights Reserved.
 */
// Dependencies supplied by the surrounding XFS/Rust translation environment.

static mut XFS_ICREATE_CACHE: *mut kmem_cache = core::ptr::null_mut(); // inode create item

#[inline]
unsafe fn icr_item(lip: *mut xfs_log_item) -> *mut xfs_icreate_item {
    container_of!(lip, xfs_icreate_item, ic_item)
}

/*
 * This returns the number of iovecs needed to log the given inode item.
 *
 * We only need one iovec for the icreate log structure.
 */
unsafe extern "C" fn xfs_icreate_item_size(
    _lip: *mut xfs_log_item,
    nvecs: *mut i32,
    nbytes: *mut i32,
) {
    *nvecs += 1;
    *nbytes += core::mem::size_of::<xfs_icreate_log>() as i32;
}

/*
 * This is called to fill in the vector of log iovecs for the
 * given inode create log item.
 */
unsafe extern "C" fn xfs_icreate_item_format(
    lip: *mut xfs_log_item,
    lfb: *mut xlog_format_buf,
) {
    let icp = icr_item(lip);
    xlog_format_copy(
        lfb,
        XLOG_REG_TYPE_ICREATE,
        &(*icp).ic_format as *const xfs_icreate_log as *const core::ffi::c_void,
        core::mem::size_of::<xfs_icreate_log>(),
    );
}

unsafe extern "C" fn xfs_icreate_item_release(lip: *mut xfs_log_item) {
    kvfree((*icr_item(lip)).ic_item.li_lv_shadow);
    kmem_cache_free(XFS_ICREATE_CACHE, icr_item(lip));
}

static XFS_ICREATE_ITEM_OPS: xfs_item_ops = xfs_item_ops {
    flags: XFS_ITEM_RELEASE_WHEN_COMMITTED,
    iop_size: Some(xfs_icreate_item_size),
    iop_format: Some(xfs_icreate_item_format),
    iop_release: Some(xfs_icreate_item_release),
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
 * caller need to know anything about the icreate item.
 */
pub unsafe extern "C" fn xfs_icreate_log(
    tp: *mut xfs_trans,
    agno: xfs_agnumber_t,
    agbno: xfs_agblock_t,
    count: u32,
    inode_size: u32,
    length: xfs_agblock_t,
    generation: u32,
) {
    let icp = kmem_cache_zalloc(XFS_ICREATE_CACHE, GFP_KERNEL | __GFP_NOFAIL)
        as *mut xfs_icreate_item;

    xfs_log_item_init(
        (*tp).t_mountp,
        &mut (*icp).ic_item,
        XFS_LI_ICREATE,
        &XFS_ICREATE_ITEM_OPS,
    );

    (*icp).ic_format.icl_type = XFS_LI_ICREATE;
    (*icp).ic_format.icl_size = 1; // single vector
    (*icp).ic_format.icl_ag = cpu_to_be32(agno);
    (*icp).ic_format.icl_agbno = cpu_to_be32(agbno);
    (*icp).ic_format.icl_count = cpu_to_be32(count);
    (*icp).ic_format.icl_isize = cpu_to_be32(inode_size);
    (*icp).ic_format.icl_length = cpu_to_be32(length);
    (*icp).ic_format.icl_gen = cpu_to_be32(generation);

    xfs_trans_add_item(tp, &mut (*icp).ic_item);
    (*tp).t_flags |= XFS_TRANS_DIRTY;
    set_bit(XFS_LI_DIRTY, &mut (*icp).ic_item.li_flags);
}

unsafe extern "C" fn xlog_recover_icreate_reorder(
    _item: *mut xlog_recover_item,
) -> xlog_recover_reorder {
    /*
     * Inode allocation buffers must be replayed before subsequent inode
     * items try to modify those buffers. ICREATE items are the logical
     * equivalent of logging a newly initialized inode buffer, so recover
     * these at the same time that we recover logged buffers.
     */
    XLOG_REORDER_BUFFER_LIST
}

/*
 * This routine is called when an inode create format structure is found in a
 * committed transaction in the log. It's purpose is to initialise the inodes
 * being allocated on disk. This requires us to get inode cluster buffers that
 * match the range to be initialised, stamped with inode templates and written
 * by delayed write so that subsequent modifications will hit the cached buffer
 * and only need writing out at the end of recovery.
 */
unsafe extern "C" fn xlog_recover_icreate_commit_pass2(
    log: *mut xlog,
    buffer_list: *mut list_head,
    item: *mut xlog_recover_item,
    _lsn: xfs_lsn_t,
) -> i32 {
    let mp = (*log).l_mp;
    let icl = (*item).ri_buf[0].iov_base as *mut xfs_icreate_log;
    let igeo = M_IGEO(mp);
    let agno: xfs_agnumber_t;
    let agbno: xfs_agblock_t;
    let count: u32;
    let isize: u32;
    let length: xfs_agblock_t;
    let bb_per_cluster: i32;
    let mut cancel_count: i32;
    let nbufs: i32;

    if (*icl).icl_type != XFS_LI_ICREATE {
        xfs_warn((*log).l_mp, "xlog_recover_do_icreate_trans: bad type");
        return -EINVAL;
    }
    if (*icl).icl_size != 1 {
        xfs_warn((*log).l_mp, "xlog_recover_do_icreate_trans: bad icl size");
        return -EINVAL;
    }
    agno = be32_to_cpu((*icl).icl_ag);
    if agno >= (*mp).m_sb.sb_agcount {
        xfs_warn((*log).l_mp, "xlog_recover_do_icreate_trans: bad agno");
        return -EINVAL;
    }
    agbno = be32_to_cpu((*icl).icl_agbno);
    if agbno == 0 || agbno == NULLAGBLOCK || agbno >= (*mp).m_sb.sb_agblocks {
        xfs_warn((*log).l_mp, "xlog_recover_do_icreate_trans: bad agbno");
        return -EINVAL;
    }
    isize = be32_to_cpu((*icl).icl_isize);
    if isize != (*mp).m_sb.sb_inodesize {
        xfs_warn((*log).l_mp, "xlog_recover_do_icreate_trans: bad isize");
        return -EINVAL;
    }
    count = be32_to_cpu((*icl).icl_count);
    if count == 0 {
        xfs_warn((*log).l_mp, "xlog_recover_do_icreate_trans: bad count");
        return -EINVAL;
    }
    length = be32_to_cpu((*icl).icl_length);
    if length == 0 || length >= (*mp).m_sb.sb_agblocks {
        xfs_warn((*log).l_mp, "xlog_recover_do_icreate_trans: bad length");
        return -EINVAL;
    }
    if length != (*igeo).ialloc_blks && length != (*igeo).ialloc_min_blks {
        xfs_warn((*log).l_mp, "%s: unsupported chunk length", __func__);
        return -EINVAL;
    }
    if (count >> (*mp).m_sb.sb_inopblog) != length {
        xfs_warn((*log).l_mp, "%s: inconsistent inode count and chunk length", __func__);
        return -EINVAL;
    }

    bb_per_cluster = XFS_FSB_TO_BB(mp, (*igeo).blocks_per_cluster);
    nbufs = length / (*igeo).blocks_per_cluster;
    cancel_count = 0;
    for i in 0..nbufs {
        let daddr = XFS_AGB_TO_DADDR(mp, agno, agbno + i * (*igeo).blocks_per_cluster);
        if xlog_is_buffer_cancelled(log, daddr, bb_per_cluster) {
            cancel_count += 1;
        }
    }
    ASSERT!(cancel_count == 0 || cancel_count == nbufs);
    if cancel_count != 0 {
        if cancel_count != nbufs {
            xfs_warn(mp, "WARNING: partial inode chunk cancellation, skipped icreate.");
        }
        trace_xfs_log_recover_icreate_cancel(log, icl);
        return 0;
    }
    trace_xfs_log_recover_icreate_recover(log, icl);
    xfs_ialloc_inode_init(mp, core::ptr::null_mut(), buffer_list, count, agno, agbno,
                          length, be32_to_cpu((*icl).icl_gen))
}

pub static XLOG_ICREATE_ITEM_OPS: xlog_recover_item_ops = xlog_recover_item_ops {
    item_type: XFS_LI_ICREATE,
    reorder: Some(xlog_recover_icreate_reorder),
    commit_pass2: Some(xlog_recover_icreate_commit_pass2),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
