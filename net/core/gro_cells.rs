// SPDX-License-Identifier: GPL-2.0
// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
struct gro_cell {
    napi_skbs: sk_buff_head,
    napi: napi_struct,
    bh_lock: local_lock_t,
}

pub unsafe fn gro_cells_receive(gcells: *mut gro_cells, skb: *mut sk_buff) -> c_int {
    let dev = (*skb).dev;
    let mut have_bh_lock = false;
    let mut cell: *mut gro_cell;
    let res: c_int;

    rcu_read_lock();
    if unlikely((*dev).flags & IFF_UP == 0) {
        dev_core_stats_rx_dropped_inc(dev);
        kfree_skb(skb);
        res = NET_RX_DROP;
    } else if (*gcells).cells.is_null() || skb_cloned(skb) || netif_elide_gro(dev) {
        res = netif_rx(skb);
    } else {
        local_lock_nested_bh(&mut (*(*gcells).cells).bh_lock);
        have_bh_lock = true;
        cell = this_cpu_ptr((*gcells).cells);

        if skb_queue_len(&(*cell).napi_skbs) > READ_ONCE(net_hotdata.max_backlog) {
            dev_core_stats_rx_dropped_inc(dev);
            kfree_skb(skb);
            res = NET_RX_DROP;
        } else {
            __skb_queue_tail(&mut (*cell).napi_skbs, skb);
            if skb_queue_len(&(*cell).napi_skbs) == 1 {
                napi_schedule(&mut (*cell).napi);
            }
            res = NET_RX_SUCCESS;
        }
    }

    if have_bh_lock {
        local_unlock_nested_bh(&mut (*(*gcells).cells).bh_lock);
    }
    rcu_read_unlock();
    res
}

// called under BH context
unsafe fn gro_cell_poll(napi: *mut napi_struct, budget: c_int) -> c_int {
    let cell = container_of!(napi, gro_cell, napi);
    let mut skb: *mut sk_buff;
    let mut work_done: c_int = 0;

    while work_done < budget {
        __local_lock_nested_bh(&mut (*cell).bh_lock);
        skb = __skb_dequeue(&mut (*cell).napi_skbs);
        __local_unlock_nested_bh(&mut (*cell).bh_lock);
        if skb.is_null() {
            break;
        }
        napi_gro_receive(napi, skb);
        work_done += 1;
    }

    if work_done < budget {
        napi_complete_done(napi, work_done);
    }
    work_done
}

pub unsafe fn gro_cells_init(gcells: *mut gro_cells, dev: *mut net_device) -> c_int {
    let mut i: c_int;

    (*gcells).cells = alloc_percpu!(gro_cell);
    if (*gcells).cells.is_null() {
        return -ENOMEM;
    }

    for_each_possible_cpu!(i) {
        let cell = per_cpu_ptr((*gcells).cells, i);

        __skb_queue_head_init(&mut (*cell).napi_skbs);
        local_lock_init(&mut (*cell).bh_lock);

        set_bit(NAPI_STATE_NO_BUSY_POLL, &mut (*cell).napi.state);

        netif_napi_add(dev, &mut (*cell).napi, gro_cell_poll);
        napi_enable(&mut (*cell).napi);
    }
    0
}

#[repr(C)]
struct percpu_free_defer {
    rcu: rcu_head,
    ptr: *mut core::ffi::c_void,
}

unsafe fn percpu_free_defer_callback(head: *mut rcu_head) {
    let defer = container_of!(head, percpu_free_defer, rcu);
    free_percpu((*defer).ptr);
    kfree(defer);
}

pub unsafe fn gro_cells_destroy(gcells: *mut gro_cells) {
    let defer: *mut percpu_free_defer;
    let mut i: c_int;

    if (*gcells).cells.is_null() {
        return;
    }
    for_each_possible_cpu!(i) {
        let cell = per_cpu_ptr((*gcells).cells, i);

        napi_disable(&mut (*cell).napi);
        __netif_napi_del(&mut (*cell).napi);
        __skb_queue_purge(&mut (*cell).napi_skbs);
    }
    /* We need to observe an rcu grace period before freeing ->cells,
     * because netpoll could access dev->napi_list under rcu protection.
     * Try hard using call_rcu() instead of synchronize_rcu(),
     * because we might be called from cleanup_net(), and we
     * definitely do not want to block this critical task.
     */
    defer = kmalloc_obj!(percpu_free_defer, GFP_KERNEL | __GFP_NOWARN);
    if likely(!defer.is_null()) {
        (*defer).ptr = (*gcells).cells as *mut core::ffi::c_void;
        call_rcu(&mut (*defer).rcu, percpu_free_defer_callback);
    } else {
        /* We do not hold RTNL at this point, synchronize_net()
         * would not be able to expedite this sync.
         */
        synchronize_rcu_expedited();
        free_percpu((*gcells).cells as *mut core::ffi::c_void);
    }
    (*gcells).cells = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
