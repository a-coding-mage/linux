/*
 * Copyright (c) 2016 Oracle.  All rights reserved.
 *
 * This software is available to you under a choice of one of two
 * licenses.  You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the
 * OpenIB.org BSD license below:
 */

#[inline]
unsafe fn rds_transition_frwr_state(
    ibmr: *mut rds_ib_mr,
    old_state: rds_ib_fr_state,
    new_state: rds_ib_fr_state,
) {
    if cmpxchg(&mut (*ibmr).u.frmr.fr_state, old_state, new_state) == old_state
        && old_state == FRMR_IS_INUSE
    {
        /* enforce order of ibmr->u.frmr.fr_state update
         * before decrementing i_fastreg_inuse_count
         */
        smp_mb__before_atomic();
        atomic_dec(&mut (*(*ibmr).ic).i_fastreg_inuse_count);
        if waitqueue_active(&rds_ib_ring_empty_wait) {
            wake_up(&rds_ib_ring_empty_wait);
        }
    }
}

unsafe fn rds_ib_alloc_frmr(
    rds_ibdev: *mut rds_ib_device,
    npages: i32,
) -> *mut rds_ib_mr {
    let pool: *mut rds_ib_mr_pool;
    let mut ibmr: *mut rds_ib_mr = core::ptr::null_mut();
    let frmr: *mut rds_ib_frmr;
    let mut err: i32 = 0;

    if npages <= RDS_MR_8K_MSG_SIZE {
        pool = (*rds_ibdev).mr_8k_pool;
    } else {
        pool = (*rds_ibdev).mr_1m_pool;
    }

    ibmr = rds_ib_try_reuse_ibmr(pool);
    if !ibmr.is_null() {
        return ibmr;
    }

    ibmr = kzalloc_node(
        core::mem::size_of::<rds_ib_mr>(),
        GFP_KERNEL,
        rdsibdev_to_node(rds_ibdev),
    );
    if ibmr.is_null() {
        err = -ENOMEM;
        goto out_no_cigar;
    }

    frmr = &mut (*ibmr).u.frmr;
    (*frmr).mr = ib_alloc_mr((*rds_ibdev).pd, IB_MR_TYPE_MEM_REG, (*pool).max_pages);
    if IS_ERR((*frmr).mr) {
        pr_warn("RDS/IB: {} failed to allocate MR", __func__);
        err = PTR_ERR((*frmr).mr);
        goto out_no_cigar;
    }

    (*ibmr).pool = pool;
    if (*pool).pool_type == RDS_IB_MR_8K_POOL {
        rds_ib_stats_inc(s_ib_rdma_mr_8k_alloc);
    } else {
        rds_ib_stats_inc(s_ib_rdma_mr_1m_alloc);
    }

    if atomic_read(&(*pool).item_count) > (*pool).max_items_soft {
        (*pool).max_items_soft = (*pool).max_items;
    }

    (*frmr).fr_state = FRMR_IS_FREE;
    init_waitqueue_head(&mut (*frmr).fr_inv_done);
    init_waitqueue_head(&mut (*frmr).fr_reg_done);
    return ibmr;

    out_no_cigar:
    kfree(ibmr);
    atomic_dec(&mut (*pool).item_count);
    ERR_PTR(err)
}

unsafe fn rds_ib_free_frmr(ibmr: *mut rds_ib_mr, drop: bool) {
    let pool = (*ibmr).pool;

    if drop {
        llist_add(&mut (*ibmr).llnode, &mut (*pool).drop_list);
    } else {
        llist_add(&mut (*ibmr).llnode, &mut (*pool).free_list);
    }
    atomic_add((*ibmr).sg_len, &mut (*pool).free_pinned);
    atomic_inc(&mut (*pool).dirty_count);

    /* If we've pinned too many pages, request a flush */
    if atomic_read(&(*pool).free_pinned) >= (*pool).max_free_pinned
        || atomic_read(&(*pool).dirty_count) >= (*pool).max_items / 5
    {
        queue_delayed_work(rds_ib_mr_wq, &mut (*pool).flush_worker, 10);
    }
}

unsafe fn rds_ib_post_reg_frmr(ibmr: *mut rds_ib_mr) -> i32 {
    let frmr = &mut (*ibmr).u.frmr;
    let mut reg_wr: ib_reg_wr = core::mem::zeroed();
    let mut ret: i32;
    let mut off: i32 = 0;

    while atomic_dec_return(&mut (*(*ibmr).ic).i_fastreg_wrs) <= 0 {
        atomic_inc(&mut (*(*ibmr).ic).i_fastreg_wrs);
        cpu_relax();
    }

    ret = ib_map_mr_sg_zbva((*frmr).mr, (*ibmr).sg, (*ibmr).sg_dma_len, &mut off, PAGE_SIZE);
    if unlikely(ret != (*ibmr).sg_dma_len) {
        ret = if ret < 0 { ret } else { -EINVAL };
        goto out_inc;
    }

    if cmpxchg(&mut (*frmr).fr_state, FRMR_IS_FREE, FRMR_IS_INUSE) != FRMR_IS_FREE {
        ret = -EBUSY;
        goto out_inc;
    }

    atomic_inc(&mut (*(*ibmr).ic).i_fastreg_inuse_count);

    /* Perform a WR for the fast_reg_mr. */
    ib_update_fast_reg_key((*frmr).mr, (*ibmr).remap_count);
    (*ibmr).remap_count = (*ibmr).remap_count.wrapping_add(1);
    (*frmr).fr_reg = true;

    reg_wr = core::mem::zeroed();
    reg_wr.wr.wr_id = ibmr as usize as u64;
    reg_wr.wr.opcode = IB_WR_REG_MR;
    reg_wr.wr.num_sge = 0;
    reg_wr.mr = (*frmr).mr;
    reg_wr.key = (*frmr).mr.as_ref().unwrap().rkey;
    reg_wr.access = IB_ACCESS_LOCAL_WRITE | IB_ACCESS_REMOTE_READ | IB_ACCESS_REMOTE_WRITE;
    reg_wr.wr.send_flags = IB_SEND_SIGNALED;

    ret = ib_post_send((*(*(*ibmr).ic).i_cm_id).qp, &mut reg_wr.wr, core::ptr::null_mut());
    if unlikely(ret != 0) {
        rds_transition_frwr_state(ibmr, FRMR_IS_INUSE, FRMR_IS_STALE);
        if printk_ratelimit() {
            pr_warn("RDS/IB: {} returned error({})\n", __func__, ret);
        }
        goto out_inc;
    }

    wait_event(&mut (*frmr).fr_reg_done, !(*frmr).fr_reg);
    return ret;

    out_inc:
    atomic_inc(&mut (*(*ibmr).ic).i_fastreg_wrs);
    ret
}

unsafe fn rds_ib_map_frmr(
    rds_ibdev: *mut rds_ib_device,
    _pool: *mut rds_ib_mr_pool,
    ibmr: *mut rds_ib_mr,
    sg: *mut scatterlist,
    sg_len: u32,
) -> i32 {
    let dev = (*rds_ibdev).dev;
    let frmr = &mut (*ibmr).u.frmr;
    let mut len: u32;
    let mut ret: i32 = 0;

    rds_ib_teardown_mr(ibmr);
    (*ibmr).sg = sg;
    (*ibmr).sg_len = sg_len;
    (*ibmr).sg_dma_len = 0;
    (*frmr).sg_byte_len = 0;
    WARN_ON((*ibmr).sg_dma_len);
    (*ibmr).sg_dma_len = ib_dma_map_sg(dev, (*ibmr).sg, (*ibmr).sg_len, DMA_BIDIRECTIONAL);
    if unlikely((*ibmr).sg_dma_len == 0) {
        pr_warn("RDS/IB: {} failed!\n", __func__);
        return -EBUSY;
    }

    (*frmr).sg_byte_len = 0;
    (*frmr).dma_npages = 0;
    len = 0;
    ret = -EINVAL;
    for i in 0..(*ibmr).sg_dma_len {
        let dma_len = sg_dma_len(&mut (*ibmr).sg.add(i as usize));
        let dma_addr = sg_dma_address(&mut (*ibmr).sg.add(i as usize));
        (*frmr).sg_byte_len += dma_len;
        if dma_addr & !PAGE_MASK != 0 {
            if i > 0 { goto out_unmap; } else { (*frmr).dma_npages += 1; }
        }
        if (dma_addr + dma_len as u64) & !PAGE_MASK != 0 {
            if i < (*ibmr).sg_dma_len - 1 { goto out_unmap; } else { (*frmr).dma_npages += 1; }
        }
        len += dma_len;
    }
    (*frmr).dma_npages += len >> PAGE_SHIFT;
    if (*frmr).dma_npages > (*ibmr).pool.as_ref().unwrap().max_pages { ret = -EMSGSIZE; goto out_unmap; }
    ret = rds_ib_post_reg_frmr(ibmr);
    if ret != 0 { goto out_unmap; }
    if (*ibmr).pool.as_ref().unwrap().pool_type == RDS_IB_MR_8K_POOL { rds_ib_stats_inc(s_ib_rdma_mr_8k_used); } else { rds_ib_stats_inc(s_ib_rdma_mr_1m_used); }
    return ret;

    out_unmap:
    ib_dma_unmap_sg((*rds_ibdev).dev, (*ibmr).sg, (*ibmr).sg_len, DMA_BIDIRECTIONAL);
    (*ibmr).sg_dma_len = 0;
    ret
}

unsafe fn rds_ib_post_inv(ibmr: *mut rds_ib_mr) -> i32 {
    let frmr = &mut (*ibmr).u.frmr;
    let i_cm_id = (*(*ibmr).ic).i_cm_id;
    let mut ret = -EINVAL;
    if i_cm_id.is_null() || (*i_cm_id).qp.is_null() || (*frmr).mr.is_null() { return ret; }
    if (*frmr).fr_state != FRMR_IS_INUSE { return ret; }
    while atomic_dec_return(&mut (*(*ibmr).ic).i_fastreg_wrs) <= 0 { atomic_inc(&mut (*(*ibmr).ic).i_fastreg_wrs); cpu_relax(); }
    (*frmr).fr_inv = true;
    let s_wr = &mut (*frmr).fr_wr;
    *s_wr = core::mem::zeroed();
    (*s_wr).wr_id = ibmr as usize as u64;
    (*s_wr).opcode = IB_WR_LOCAL_INV;
    (*s_wr).ex.invalidate_rkey = (*frmr).mr.as_ref().unwrap().rkey;
    (*s_wr).send_flags = IB_SEND_SIGNALED;
    ret = ib_post_send((*i_cm_id).qp, s_wr, core::ptr::null_mut());
    if unlikely(ret != 0) {
        rds_transition_frwr_state(ibmr, FRMR_IS_INUSE, FRMR_IS_STALE);
        (*frmr).fr_inv = false;
        smp_mb__before_atomic();
        atomic_inc(&mut (*(*ibmr).ic).i_fastreg_wrs);
        pr_err("RDS/IB: {} returned error({})\n", __func__, ret);
        return ret;
    }
    wait_event(&mut (*frmr).fr_inv_done, (*frmr).fr_state != FRMR_IS_INUSE);
    ret
}

pub unsafe fn rds_ib_mr_cqe_handler(ic: *mut rds_ib_connection, wc: *mut ib_wc) {
    let ibmr = (*wc).wr_id as usize as *mut rds_ib_mr;
    let frmr = &mut (*ibmr).u.frmr;
    if (*wc).status != IB_WC_SUCCESS {
        rds_transition_frwr_state(ibmr, FRMR_IS_INUSE, FRMR_IS_STALE);
        if rds_conn_up((*ic).conn) { rds_ib_conn_error((*ic).conn, "frmr completion status, disconnecting and reconnecting\n"); }
    }
    if (*frmr).fr_inv { rds_transition_frwr_state(ibmr, FRMR_IS_INUSE, FRMR_IS_FREE); (*frmr).fr_inv = false; wake_up(&(*frmr).fr_inv_done); }
    if (*frmr).fr_reg { (*frmr).fr_reg = false; wake_up(&(*frmr).fr_reg_done); }
    smp_mb__before_atomic();
    atomic_inc(&mut (*ic).i_fastreg_wrs);
}

pub unsafe fn rds_ib_unreg_frmr(list: *mut list_head, nfreed: *mut u32, unpinned: *mut usize, goal: u32) {
    let mut freed = *nfreed;
    let mut ret = 0;
    let mut ibmr: *mut rds_ib_mr = core::ptr::null_mut();
    list_for_each_entry(ibmr, list, unmap_list) { if (*ibmr).sg_dma_len != 0 { let ret2 = rds_ib_post_inv(ibmr); if ret2 != 0 && ret == 0 { ret = ret2; } } }
    if ret != 0 { pr_warn("RDS/IB: {} failed (err={})\n", __func__, ret); }
    list_for_each_entry_safe(ibmr, list, unmap_list) {
        *unpinned += (*ibmr).sg_len as usize;
        let frmr = &mut (*ibmr).u.frmr;
        __rds_ib_teardown_mr(ibmr);
        if freed < goal || (*frmr).fr_state == FRMR_IS_STALE {
            if (*frmr).fr_state == FRMR_IS_INUSE { continue; }
            if (*ibmr).pool.as_ref().unwrap().pool_type == RDS_IB_MR_8K_POOL { rds_ib_stats_inc(s_ib_rdma_mr_8k_free); } else { rds_ib_stats_inc(s_ib_rdma_mr_1m_free); }
            list_del(&mut (*ibmr).unmap_list);
            if !(*frmr).mr.is_null() { ib_dereg_mr((*frmr).mr); }
            kfree(ibmr);
            freed += 1;
        }
    }
    *nfreed = freed;
}

pub unsafe fn rds_ib_reg_frmr(rds_ibdev: *mut rds_ib_device, ic: *mut rds_ib_connection, sg: *mut scatterlist, nents: usize, key: *mut u32) -> *mut rds_ib_mr {
    let mut ibmr: *mut rds_ib_mr = core::ptr::null_mut();
    if ic.is_null() { return ERR_PTR(-EOPNOTSUPP); }
    loop {
        if !ibmr.is_null() { rds_ib_free_frmr(ibmr, true); }
        ibmr = rds_ib_alloc_frmr(rds_ibdev, nents as i32);
        if IS_ERR(ibmr) { return ibmr; }
        if (*ibmr).u.frmr.fr_state == FRMR_IS_FREE { break; }
    }
    (*ibmr).ic = ic;
    (*ibmr).device = rds_ibdev;
    let ret = rds_ib_map_frmr(rds_ibdev, (*ibmr).pool, ibmr, sg, nents as u32);
    if ret == 0 { *key = (*ibmr).u.frmr.mr.as_ref().unwrap().rkey; } else { rds_ib_free_frmr(ibmr, false); ibmr = ERR_PTR(ret); }
    ibmr
}

pub unsafe fn rds_ib_free_frmr_list(ibmr: *mut rds_ib_mr) {
    let pool = (*ibmr).pool;
    let frmr = &mut (*ibmr).u.frmr;
    if (*frmr).fr_state == FRMR_IS_STALE { llist_add(&mut (*ibmr).llnode, &mut (*pool).drop_list); } else { llist_add(&mut (*ibmr).llnode, &mut (*pool).free_list); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
