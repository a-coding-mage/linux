/*
 * Copyright (c) 2006, 2018 Oracle and/or its affiliates. All rights reserved.
 *
 * This software is available under a choice of one of two licenses: GPL v2 or
 * the OpenIB.org BSD license. See the original source for the full terms.
 *
 * Direct Rust translation of ib_rdma.c. Kernel and project symbols are
 * supplied by external dependencies.
 */

pub static mut rds_ib_mr_wq: *mut workqueue_struct = core::ptr::null_mut();

unsafe extern "C" {
    fn rds_ib_odp_mr_worker(work: *mut work_struct);
}

pub unsafe extern "C" fn rds_ib_get_device(ipaddr: __be32) -> *mut rds_ib_device {
    let mut rds_ibdev: *mut rds_ib_device;
    let mut i_ipaddr: *mut rds_ib_ipaddr;

    rcu_read_lock();
    list_for_each_entry_rcu!(rds_ibdev, &raw mut rds_ib_devices, list, {
        list_for_each_entry_rcu!(i_ipaddr, &raw mut (*rds_ibdev).ipaddr_list, list, {
            if (*i_ipaddr).ipaddr == ipaddr {
                refcount_inc(&raw mut (*rds_ibdev).refcount);
                rcu_read_unlock();
                return rds_ibdev;
            }
        });
    });
    rcu_read_unlock();
    core::ptr::null_mut()
}

unsafe fn rds_ib_add_ipaddr(rds_ibdev: *mut rds_ib_device, ipaddr: __be32) -> c_int {
    let i_ipaddr = kmalloc_obj::<rds_ib_ipaddr>();
    if i_ipaddr.is_null() { return -ENOMEM; }
    (*i_ipaddr).ipaddr = ipaddr;
    spin_lock_irq(&raw mut (*rds_ibdev).spinlock);
    list_add_tail_rcu(&raw mut (*i_ipaddr).list, &raw mut (*rds_ibdev).ipaddr_list);
    spin_unlock_irq(&raw mut (*rds_ibdev).spinlock);
    0
}

unsafe fn rds_ib_remove_ipaddr(rds_ibdev: *mut rds_ib_device, ipaddr: __be32) {
    let mut to_free: *mut rds_ib_ipaddr = core::ptr::null_mut();
    let mut i_ipaddr: *mut rds_ib_ipaddr;
    spin_lock_irq(&raw mut (*rds_ibdev).spinlock);
    list_for_each_entry_rcu!(i_ipaddr, &raw mut (*rds_ibdev).ipaddr_list, list, {
        if (*i_ipaddr).ipaddr == ipaddr {
            list_del_rcu(&raw mut (*i_ipaddr).list);
            to_free = i_ipaddr;
            break;
        }
    });
    spin_unlock_irq(&raw mut (*rds_ibdev).spinlock);
    if !to_free.is_null() { kfree_rcu(to_free, rcu); }
}

pub unsafe extern "C" fn rds_ib_update_ipaddr(rds_ibdev: *mut rds_ib_device, ipaddr: *mut in6_addr) -> c_int {
    let addr = (*ipaddr).s6_addr32[3];
    let old = rds_ib_get_device(addr);
    if old.is_null() { return rds_ib_add_ipaddr(rds_ibdev, addr); }
    if old != rds_ibdev {
        rds_ib_remove_ipaddr(old, addr);
        rds_ib_dev_put(old);
        return rds_ib_add_ipaddr(rds_ibdev, addr);
    }
    rds_ib_dev_put(old);
    0
}

pub unsafe extern "C" fn rds_ib_add_conn(rds_ibdev: *mut rds_ib_device, conn: *mut rds_connection) {
    let ic = (*conn).c_transport_data as *mut rds_ib_connection;
    spin_lock_irq(&raw mut ib_nodev_conns_lock);
    BUG_ON(list_empty(&raw mut ib_nodev_conns));
    BUG_ON(list_empty(&raw mut (*ic).ib_node));
    list_del(&raw mut (*ic).ib_node);
    spin_lock(&raw mut (*rds_ibdev).spinlock);
    list_add_tail(&raw mut (*ic).ib_node, &raw mut (*rds_ibdev).conn_list);
    spin_unlock(&raw mut (*rds_ibdev).spinlock);
    spin_unlock_irq(&raw mut ib_nodev_conns_lock);
    (*ic).rds_ibdev = rds_ibdev;
    refcount_inc(&raw mut (*rds_ibdev).refcount);
}

pub unsafe extern "C" fn rds_ib_remove_conn(rds_ibdev: *mut rds_ib_device, conn: *mut rds_connection) {
    let ic = (*conn).c_transport_data as *mut rds_ib_connection;
    spin_lock(&raw mut ib_nodev_conns_lock);
    spin_lock_irq(&raw mut (*rds_ibdev).spinlock);
    BUG_ON(list_empty(&raw mut (*ic).ib_node));
    list_del(&raw mut (*ic).ib_node);
    spin_unlock_irq(&raw mut (*rds_ibdev).spinlock);
    list_add_tail(&raw mut (*ic).ib_node, &raw mut ib_nodev_conns);
    spin_unlock(&raw mut ib_nodev_conns_lock);
    (*ic).rds_ibdev = core::ptr::null_mut();
    rds_ib_dev_put(rds_ibdev);
}

pub unsafe extern "C" fn rds_ib_destroy_nodev_conns() {
    let mut ic: *mut rds_ib_connection;
    let mut _ic: *mut rds_ib_connection;
    let mut tmp_list: list_head = LIST_HEAD_INIT!();
    spin_lock_irq(&raw mut ib_nodev_conns_lock);
    list_splice(&raw mut ib_nodev_conns, &raw mut tmp_list);
    spin_unlock_irq(&raw mut ib_nodev_conns_lock);
    list_for_each_entry_safe!(ic, _ic, &raw mut tmp_list, ib_node, {
        rds_conn_destroy((*ic).conn);
    });
}

pub unsafe extern "C" fn rds_ib_get_mr_info(rds_ibdev: *mut rds_ib_device, iinfo: *mut rds_info_rdma_connection) {
    let pool = (*rds_ibdev).mr_1m_pool;
    (*iinfo).rdma_mr_max = (*pool).max_items;
    (*iinfo).rdma_mr_size = (*pool).max_pages;
}

#[cfg(CONFIG_IPV6)]
pub unsafe extern "C" fn rds6_ib_get_mr_info(rds_ibdev: *mut rds_ib_device, iinfo6: *mut rds6_info_rdma_connection) {
    let pool = (*rds_ibdev).mr_1m_pool;
    (*iinfo6).rdma_mr_max = (*pool).max_items;
    (*iinfo6).rdma_mr_size = (*pool).max_pages;
}

pub unsafe extern "C" fn rds_ib_reuse_mr(pool: *mut rds_ib_mr_pool) -> *mut rds_ib_mr {
    let mut ibmr: *mut rds_ib_mr = core::ptr::null_mut();
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&raw mut (*pool).clean_lock, &mut flags);
    let ret = llist_del_first(&raw mut (*pool).clean_list);
    spin_unlock_irqrestore(&raw mut (*pool).clean_lock, flags);
    if !ret.is_null() {
        ibmr = llist_entry!(ret, rds_ib_mr, llnode);
        if (*pool).pool_type == RDS_IB_MR_8K_POOL { rds_ib_stats_inc(s_ib_rdma_mr_8k_reused); }
        else { rds_ib_stats_inc(s_ib_rdma_mr_1m_reused); }
    }
    ibmr
}

pub unsafe extern "C" fn rds_ib_sync_mr(trans_private: *mut c_void, direction: c_int) {
    let ibmr = trans_private as *mut rds_ib_mr;
    let dev = (*ibmr).device;
    if (*ibmr).odp != 0 { return; }
    match direction {
        DMA_FROM_DEVICE => ib_dma_sync_sg_for_cpu((*dev).dev, (*ibmr).sg, (*ibmr).sg_dma_len, DMA_BIDIRECTIONAL),
        DMA_TO_DEVICE => ib_dma_sync_sg_for_device((*dev).dev, (*ibmr).sg, (*ibmr).sg_dma_len, DMA_BIDIRECTIONAL),
        _ => {}
    }
}

pub unsafe extern "C" fn __rds_ib_teardown_mr(ibmr: *mut rds_ib_mr) {
    let dev = (*ibmr).device;
    if (*ibmr).sg_dma_len != 0 {
        ib_dma_unmap_sg((*dev).dev, (*ibmr).sg, (*ibmr).sg_len, DMA_BIDIRECTIONAL);
        (*ibmr).sg_dma_len = 0;
    }
    if (*ibmr).sg_len != 0 {
        for i in 0..(*ibmr).sg_len {
            let page = sg_page((*ibmr).sg.add(i));
            unpin_user_pages_dirty_lock(&page, 1, true);
        }
        kfree((*ibmr).sg as *mut c_void);
        (*ibmr).sg = core::ptr::null_mut();
        (*ibmr).sg_len = 0;
    }
}

pub unsafe extern "C" fn rds_ib_teardown_mr(ibmr: *mut rds_ib_mr) {
    let pinned = (*ibmr).sg_len;
    __rds_ib_teardown_mr(ibmr);
    if pinned != 0 { atomic_sub(pinned, &raw mut (*(*ibmr).pool).free_pinned); }
}

unsafe fn rds_ib_flush_goal(pool: *mut rds_ib_mr_pool, free_all: c_int) -> c_uint {
    let count = atomic_read(&raw mut (*pool).item_count);
    if free_all != 0 { count } else { 0 }
}

unsafe fn llist_append_to_list(llist: *mut llist_head, list: *mut list_head) -> c_uint {
    let mut count = 0;
    let mut node = llist_del_all(llist);
    while !node.is_null() {
        let next = (*node).next;
        let ibmr = llist_entry!(node, rds_ib_mr, llnode);
        list_add_tail(&raw mut (*ibmr).unmap_list, list);
        node = next;
        count += 1;
    }
    count
}

unsafe fn list_to_llist_nodes(list: *mut list_head, nodes_head: *mut *mut llist_node, nodes_tail: *mut *mut llist_node) {
    let mut cur: *mut llist_node = core::ptr::null_mut();
    let mut next = nodes_head;
    let mut ibmr: *mut rds_ib_mr;
    list_for_each_entry!(ibmr, list, unmap_list, {
        cur = &raw mut (*ibmr).llnode;
        *next = cur;
        next = &mut (*cur).next;
    });
    *next = core::ptr::null_mut();
    *nodes_tail = cur;
}

pub unsafe extern "C" fn rds_ib_flush_mr_pool(pool: *mut rds_ib_mr_pool, free_all: c_int, ibmr_ret: *mut *mut rds_ib_mr) -> c_int {
    let mut ibmr: *mut rds_ib_mr;
    let mut clean_nodes: *mut llist_node = core::ptr::null_mut();
    let mut clean_tail: *mut llist_node = core::ptr::null_mut();
    let mut unmap_list: list_head = LIST_HEAD_INIT!();
    let mut unpinned: c_ulong = 0;
    let mut nfreed: c_uint = 0;
    let mut dirty_to_clean: c_uint = 0;
    if (*pool).pool_type == RDS_IB_MR_8K_POOL { rds_ib_stats_inc(s_ib_rdma_mr_8k_pool_flush); } else { rds_ib_stats_inc(s_ib_rdma_mr_1m_pool_flush); }
    mutex_lock(&raw mut (*pool).flush_lock);
    if !ibmr_ret.is_null() { ibmr = rds_ib_reuse_mr(pool); if !ibmr.is_null() { *ibmr_ret = ibmr; mutex_unlock(&raw mut (*pool).flush_lock); return 0; } }
    dirty_to_clean = llist_append_to_list(&raw mut (*pool).drop_list, &raw mut unmap_list);
    dirty_to_clean += llist_append_to_list(&raw mut (*pool).free_list, &raw mut unmap_list);
    if free_all != 0 { let mut flags = 0; spin_lock_irqsave(&raw mut (*pool).clean_lock, &mut flags); llist_append_to_list(&raw mut (*pool).clean_list, &raw mut unmap_list); spin_unlock_irqrestore(&raw mut (*pool).clean_lock, flags); }
    let free_goal = rds_ib_flush_goal(pool, free_all);
    if !list_empty(&raw mut unmap_list) {
        rds_ib_unreg_frmr(&mut unmap_list, &mut nfreed, &mut unpinned, free_goal);
        if !list_empty(&raw mut unmap_list) {
            list_to_llist_nodes(&mut unmap_list, &mut clean_nodes, &mut clean_tail);
            if !ibmr_ret.is_null() { *ibmr_ret = llist_entry!(clean_nodes, rds_ib_mr, llnode); clean_nodes = (*clean_nodes).next; }
            if !clean_nodes.is_null() { let mut flags = 0; spin_lock_irqsave(&raw mut (*pool).clean_lock, &mut flags); llist_add_batch(clean_nodes, clean_tail, &raw mut (*pool).clean_list); spin_unlock_irqrestore(&raw mut (*pool).clean_lock, flags); }
        }
        atomic_sub(unpinned, &raw mut (*pool).free_pinned);
        atomic_sub(dirty_to_clean, &raw mut (*pool).dirty_count);
        atomic_sub(nfreed, &raw mut (*pool).item_count);
    }
    mutex_unlock(&raw mut (*pool).flush_lock);
    if waitqueue_active(&raw mut (*pool).flush_wait) { wake_up(&raw mut (*pool).flush_wait); }
    0
}

pub unsafe extern "C" fn rds_ib_try_reuse_ibmr(pool: *mut rds_ib_mr_pool) -> *mut rds_ib_mr {
    let mut iter = 0;
    loop {
        let ibmr = rds_ib_reuse_mr(pool);
        if !ibmr.is_null() { return ibmr; }
        if atomic_inc_return(&raw mut (*pool).item_count) <= (*pool).max_items { break; }
        atomic_dec(&raw mut (*pool).item_count);
        iter += 1;
        if iter > 2 { if (*pool).pool_type == RDS_IB_MR_8K_POOL { rds_ib_stats_inc(s_ib_rdma_mr_8k_pool_depleted); } else { rds_ib_stats_inc(s_ib_rdma_mr_1m_pool_depleted); } break; }
        if (*pool).pool_type == RDS_IB_MR_8K_POOL { rds_ib_stats_inc(s_ib_rdma_mr_8k_pool_wait); } else { rds_ib_stats_inc(s_ib_rdma_mr_1m_pool_wait); }
        let mut out = core::ptr::null_mut(); rds_ib_flush_mr_pool(pool, 0, &mut out); if !out.is_null() { return out; }
    }
    core::ptr::null_mut()
}

unsafe extern "C" fn rds_ib_mr_pool_flush_worker(work: *mut work_struct) {
    let pool = container_of!(work, rds_ib_mr_pool, flush_worker.work);
    rds_ib_flush_mr_pool(pool, 0, core::ptr::null_mut());
}

pub unsafe extern "C" fn rds_ib_free_mr(trans_private: *mut c_void, invalidate: c_int) {
    let ibmr = trans_private as *mut rds_ib_mr;
    let pool = (*ibmr).pool;
    if (*ibmr).odp != 0 { INIT_DELAYED_WORK!(&mut (*ibmr).work, rds_ib_odp_mr_worker); queue_delayed_work(rds_ib_mr_wq, &mut (*ibmr).work, 0); return; }
    rds_ib_free_frmr_list(ibmr);
    atomic_add((*ibmr).sg_len, &raw mut (*pool).free_pinned);
    atomic_inc(&raw mut (*pool).dirty_count);
    if atomic_read(&raw mut (*pool).free_pinned) >= (*pool).max_free_pinned || atomic_read(&raw mut (*pool).dirty_count) >= (*pool).max_items / 5 { queue_delayed_work(rds_ib_mr_wq, &mut (*pool).flush_worker, 10); }
    if invalidate != 0 { if likely(!in_interrupt()) { rds_ib_flush_mr_pool(pool, 0, core::ptr::null_mut()); } else { queue_delayed_work(rds_ib_mr_wq, &mut (*pool).flush_worker, 10); } }
    rds_ib_dev_put((*ibmr).device);
}

pub unsafe extern "C" fn rds_ib_flush_mrs() {
    let mut dev: *mut rds_ib_device;
    down_read(&raw mut rds_ib_devices_lock);
    list_for_each_entry!(dev, &raw mut rds_ib_devices, list, {
        if !(*dev).mr_8k_pool.is_null() { rds_ib_flush_mr_pool((*dev).mr_8k_pool, 0, core::ptr::null_mut()); }
        if !(*dev).mr_1m_pool.is_null() { rds_ib_flush_mr_pool((*dev).mr_1m_pool, 0, core::ptr::null_mut()); }
    });
    up_read(&raw mut rds_ib_devices_lock);
}

pub unsafe extern "C" fn rds_ib_get_lkey(trans_private: *mut c_void) -> u32 { (*((trans_private as *mut rds_ib_mr))).u.mr.lkey }

pub unsafe extern "C" fn rds_ib_get_mr(sg: *mut scatterlist, nents: c_ulong, rs: *mut rds_sock, key_ret: *mut u32, conn: *mut rds_connection, start: u64, length: u64, need_odp: c_int) -> *mut c_void {
    let dev = rds_ib_get_device((*rs).rs_bound_addr.s6_addr32[3]);
    if dev.is_null() { return ERR_PTR!(-ENODEV); }
    if need_odp == ODP_ZEROBASED || need_odp == ODP_VIRTUAL {
        let virt_addr = if need_odp == ODP_ZEROBASED { 0 } else { start };
        let flags = IB_ACCESS_LOCAL_WRITE | IB_ACCESS_REMOTE_READ | IB_ACCESS_REMOTE_WRITE | IB_ACCESS_REMOTE_ATOMIC | IB_ACCESS_ON_DEMAND;
        if !(*dev).odp_capable { rds_ib_dev_put(dev); return ERR_PTR!(-EOPNOTSUPP); }
        let mr = ib_reg_user_mr((*dev).pd, start, length, virt_addr, flags);
        if IS_ERR!(mr) { let e = PTR_ERR!(mr); rds_ib_dev_put(dev); return ERR_PTR!(e); }
        if !key_ret.is_null() { *key_ret = (*mr).rkey; }
        let ibmr = kzalloc_obj::<rds_ib_mr>();
        if ibmr.is_null() { ib_dereg_mr(mr); rds_ib_dev_put(dev); return ERR_PTR!(-ENOMEM); }
        (*ibmr).u.mr = mr; (*ibmr).odp = 1;
        let mut sge: ib_sge = core::mem::zeroed(); sge.addr = virt_addr; sge.length = length; sge.lkey = (*mr).lkey;
        ib_advise_mr((*dev).pd, IB_UVERBS_ADVISE_MR_ADVICE_PREFETCH_WRITE, IB_UVERBS_ADVISE_MR_FLAG_FLUSH, &mut sge, 1);
        return ibmr as *mut c_void;
    }
    let mut ic: *mut rds_ib_connection = core::ptr::null_mut();
    if !conn.is_null() { ic = (*conn).c_transport_data as *mut rds_ib_connection; if ic.is_null() || (*ic).i_cm_id.is_null() || (*(*ic).i_cm_id).qp.is_null() { rds_ib_dev_put(dev); return ERR_PTR!(-ENODEV); } }
    if (*dev).mr_8k_pool.is_null() || (*dev).mr_1m_pool.is_null() { rds_ib_dev_put(dev); return ERR_PTR!(-ENODEV); }
    let ibmr = rds_ib_reg_frmr(dev, ic, sg, nents, key_ret);
    if IS_ERR!(ibmr) { let e = PTR_ERR!(ibmr); pr_warn!("RDS/IB: rds_ib_get_mr failed (errno={})\n", e); rds_ib_dev_put(dev); ERR_PTR!(e) } else { ibmr as *mut c_void }
}

pub unsafe extern "C" fn rds_ib_destroy_mr_pool(pool: *mut rds_ib_mr_pool) { cancel_delayed_work_sync(&mut (*pool).flush_worker); rds_ib_flush_mr_pool(pool, 1, core::ptr::null_mut()); WARN_ON!(atomic_read(&raw mut (*pool).item_count)); WARN_ON!(atomic_read(&raw mut (*pool).free_pinned)); kfree(pool as *mut c_void); }

pub unsafe extern "C" fn rds_ib_create_mr_pool(dev: *mut rds_ib_device, pool_type: c_int) -> *mut rds_ib_mr_pool {
    let pool = kzalloc_obj::<rds_ib_mr_pool>(); if pool.is_null() { return ERR_PTR!(-ENOMEM); }
    (*pool).pool_type = pool_type; init_llist_head(&mut (*pool).free_list); init_llist_head(&mut (*pool).drop_list); init_llist_head(&mut (*pool).clean_list); spin_lock_init(&mut (*pool).clean_lock); mutex_init(&mut (*pool).flush_lock); init_waitqueue_head(&mut (*pool).flush_wait); INIT_DELAYED_WORK!(&mut (*pool).flush_worker, rds_ib_mr_pool_flush_worker);
    if pool_type == RDS_IB_MR_1M_POOL { (*pool).max_pages = RDS_MR_1M_MSG_SIZE + 1; (*pool).max_items = (*dev).max_1m_mrs; } else { (*pool).max_pages = RDS_MR_8K_MSG_SIZE + 1; (*pool).max_items = (*dev).max_8k_mrs; }
    (*pool).max_free_pinned = (*pool).max_items * (*pool).max_pages / 4; (*pool).max_items_soft = (*dev).max_mrs * 3 / 4; pool
}

pub unsafe extern "C" fn rds_ib_mr_init() -> c_int { rds_ib_mr_wq = alloc_workqueue!("rds_mr_flushd", WQ_MEM_RECLAIM | WQ_PERCPU, 0); if rds_ib_mr_wq.is_null() { -ENOMEM } else { 0 } }

pub unsafe extern "C" fn rds_ib_mr_exit() { destroy_workqueue(rds_ib_mr_wq); }

unsafe extern "C" fn rds_ib_odp_mr_worker(work: *mut work_struct) { let ibmr = container_of!(work, rds_ib_mr, work.work); ib_dereg_mr((*ibmr).u.mr); kfree(ibmr as *mut c_void); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
