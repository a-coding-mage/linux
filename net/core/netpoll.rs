// SPDX-License-Identifier: GPL-2.0-only
/*
 * Common framework for low-level network console, dump, and debugger code
 *
 * Sep 8 2003  Matt Mackall <mpm@selenic.com>
 *
 * based on the netconsole code from:
 *
 * Copyright (C) 2001  Ingo Molnar <mingo@redhat.com>
 * Copyright (C) 2002  Red Hat, Inc.
 */

// C kernel headers and build-time configuration are supplied by the surrounding
// translation unit.

const USEC_PER_POLL: c_ulong = 50;

/*
 * carrier_timeout is netconsole-specific and only kept here to preserve the
 * netpoll.carrier_timeout module-parameter ABI. Its value is exposed to
 * netconsole through netpoll_get_carrier_timeout().
 */
static mut carrier_timeout: c_uint = 4;

pub unsafe extern "C" fn netpoll_get_carrier_timeout() -> c_uint {
    carrier_timeout
}

unsafe fn netpoll_start_xmit(
    mut skb: *mut sk_buff,
    dev: *mut net_device,
    txq: *mut netdev_queue,
) -> netdev_tx_t {
    let mut status: netdev_tx_t = NETDEV_TX_OK;
    let features: netdev_features_t;

    features = netif_skb_features(skb);

    if skb_vlan_tag_present(skb) && !vlan_hw_offload_capable(features, (*skb).vlan_proto) {
        skb = __vlan_hwaccel_push_inside(skb);
        if skb.is_null() {
            /* This is actually a packet drop, but we do not want the code
             * that calls this function to try and operate on a NULL skb. */
            return status;
        }
    }

    status = netdev_start_xmit(skb, dev, txq, false);
    status
}

unsafe extern "C" fn queue_process(work: *mut work_struct) {
    let npinfo = container_of!(work, netpoll_info, tx_work.work);
    let mut skb: *mut sk_buff;
    let mut flags: c_ulong = 0;

    while {
        skb = skb_dequeue(&mut (*npinfo).txq);
        !skb.is_null()
    } {
        let dev = (*skb).dev;
        let txq: *mut netdev_queue;
        let mut q_index: c_uint;

        if !netif_device_present(dev) || !netif_running(dev) {
            kfree_skb(skb);
            continue;
        }

        local_irq_save(&mut flags);
        q_index = skb_get_queue_mapping(skb);
        if q_index >= (*dev).real_num_tx_queues {
            q_index %= (*dev).real_num_tx_queues;
            skb_set_queue_mapping(skb, q_index);
        }
        txq = netdev_get_tx_queue(dev, q_index);
        HARD_TX_LOCK(dev, txq, smp_processor_id());
        if netif_xmit_frozen_or_stopped(txq)
            || !dev_xmit_complete(netpoll_start_xmit(skb, dev, txq))
        {
            skb_queue_head(&mut (*npinfo).txq, skb);
            HARD_TX_UNLOCK(dev, txq);
            local_irq_restore(flags);
            schedule_delayed_work(&mut (*npinfo).tx_work, HZ / 10);
            return;
        }
        HARD_TX_UNLOCK(dev, txq);
        local_irq_restore(flags);
    }
}

unsafe fn netif_local_xmit_active(dev: *mut net_device) -> c_int {
    for i in 0..(*dev).num_tx_queues {
        let txq = netdev_get_tx_queue(dev, i);
        if netif_tx_owned(txq, smp_processor_id()) {
            return 1;
        }
    }
    0
}

unsafe fn poll_one_napi(napi: *mut napi_struct) {
    if test_and_set_bit(NAPI_STATE_NPSVC, &mut (*napi).state) != 0 {
        return;
    }
    let work = ((*napi).poll)(napi, 0);
    WARN_ONCE(work != 0, "%pS exceeded budget in poll\n", (*napi).poll);
    trace_napi_poll(napi, work, 0);
    clear_bit(NAPI_STATE_NPSVC, &mut (*napi).state);
}

unsafe fn poll_napi(dev: *mut net_device) {
    let cpu = smp_processor_id();
    list_for_each_entry_rcu!(napi, (*dev).napi_list, dev_list, {
        if cmpxchg(&mut (*napi).poll_owner, -1, cpu) == -1 {
            poll_one_napi(napi);
            smp_store_release(&mut (*napi).poll_owner, -1);
        }
    });
}

pub unsafe extern "C" fn netpoll_poll_dev(dev: *mut net_device) {
    let ni = rcu_dereference_bh((*dev).npinfo);
    if ni.is_null() || down_trylock(&mut (*ni).dev_lock) != 0 {
        return;
    }
    if !netif_running(dev) || netif_local_xmit_active(dev) != 0 {
        up(&mut (*ni).dev_lock);
        return;
    }
    let ops = (*dev).netdev_ops;
    if !(*ops).ndo_poll_controller.is_none() {
        ((*ops).ndo_poll_controller)(dev);
    }
    poll_napi(dev);
    up(&mut (*ni).dev_lock);
    netpoll_zap_completion_queue();
}

pub unsafe extern "C" fn netpoll_poll_disable(dev: *mut net_device) {
    might_sleep();
    let ni = rtnl_dereference((*dev).npinfo);
    if !ni.is_null() {
        down(&mut (*ni).dev_lock);
    }
}

pub unsafe extern "C" fn netpoll_poll_enable(dev: *mut net_device) {
    let ni = rtnl_dereference((*dev).npinfo);
    if !ni.is_null() {
        up(&mut (*ni).dev_lock);
    }
}

pub unsafe extern "C" fn netpoll_zap_completion_queue() {
    let mut flags: c_ulong = 0;
    let sd = get_cpu_var(softnet_data);
    if !(*sd).completion_queue.is_null() {
        local_irq_save(&mut flags);
        let mut clist = (*sd).completion_queue;
        (*sd).completion_queue = core::ptr::null_mut();
        local_irq_restore(flags);
        while !clist.is_null() {
            let skb = clist;
            clist = (*skb).next;
            if !skb_irq_freeable(skb) {
                refcount_set(&mut (*skb).users, 1);
                dev_kfree_skb_any(skb);
            } else {
                __kfree_skb(skb);
            }
        }
    }
    put_cpu_var(softnet_data);
}

unsafe fn netpoll_owner_active(dev: *mut net_device) -> c_int {
    list_for_each_entry_rcu!(napi, (*dev).napi_list, dev_list, {
        if READ_ONCE((*napi).poll_owner) == smp_processor_id() {
            return 1;
        }
    });
    0
}

unsafe fn __netpoll_send_skb(np: *mut netpoll, skb: *mut sk_buff) -> netdev_tx_t {
    let mut status = NETDEV_TX_BUSY;
    let mut ret = NET_XMIT_DROP;
    let dev = (*np).dev;
    let npinfo: *mut netpoll_info;
    (*skb).dev = dev;
    rcu_read_lock();
    npinfo = rcu_dereference_bh((*dev).npinfo);
    if npinfo.is_null() || !netif_running(dev) || !netif_device_present(dev) {
        dev_kfree_skb_irq(skb);
        rcu_read_unlock();
        return ret;
    }
    if skb_queue_len(&(*npinfo).txq) == 0 && netpoll_owner_active(dev) == 0 {
        let txq = netdev_core_pick_tx(dev, skb, core::ptr::null_mut());
        let mut tries = jiffies_to_usecs(1) / USEC_PER_POLL;
        while tries > 0 {
            if HARD_TX_TRYLOCK(dev, txq) {
                if !netif_xmit_stopped(txq) {
                    status = netpoll_start_xmit(skb, dev, txq);
                }
                HARD_TX_UNLOCK(dev, txq);
                if dev_xmit_complete(status) { break; }
            }
            netpoll_poll_dev((*np).dev);
            udelay(USEC_PER_POLL);
            tries -= 1;
        }
        WARN_ONCE(!irqs_disabled(),
            "netpoll_send_skb_on_dev(): %s enabled interrupts in poll (%pS)\n",
            (*dev).name, (*dev).netdev_ops.ndo_start_xmit);
    }
    if !dev_xmit_complete(status) {
        skb_queue_tail(&mut (*npinfo).txq, skb);
        schedule_delayed_work(&mut (*npinfo).tx_work, 0);
    }
    ret = NETDEV_TX_OK;
    rcu_read_unlock();
    ret
}

pub unsafe extern "C" fn netpoll_send_skb(np: *mut netpoll, skb: *mut sk_buff) -> netdev_tx_t {
    if np.is_null() {
        dev_kfree_skb_irq(skb);
        NET_XMIT_DROP
    } else {
        let mut flags = 0;
        local_irq_save(&mut flags);
        let ret = __netpoll_send_skb(np, skb);
        local_irq_restore(flags);
        ret
    }
}

pub unsafe extern "C" fn __netpoll_setup(np: *mut netpoll, ndev: *mut net_device) -> c_int {
    let mut npinfo = rtnl_dereference((*ndev).npinfo);
    if (*ndev).priv_flags & IFF_DISABLE_NETPOLL != 0 {
        np_err(np, "%s doesn't support polling, aborting\n", (*ndev).name);
        return -ENOTSUPP;
    }
    if npinfo.is_null() {
        npinfo = kmalloc_obj!(*npinfo);
        if npinfo.is_null() { return -ENOMEM; }
        sema_init(&mut (*npinfo).dev_lock, 1);
        skb_queue_head_init(&mut (*npinfo).txq);
        INIT_DELAYED_WORK!(&mut (*npinfo).tx_work, queue_process);
        refcount_set(&mut (*npinfo).refcnt, 1);
        let ops = (*ndev).netdev_ops;
        if !(*ops).ndo_netpoll_setup.is_none() {
            let err = ((*ops).ndo_netpoll_setup)(ndev);
            if err != 0 { kfree(npinfo); return err; }
        }
    } else {
        refcount_inc(&mut (*npinfo).refcnt);
    }
    (*np).dev = ndev;
    strscpy((*np).dev_name.as_mut_ptr(), (*ndev).name.as_ptr(), IFNAMSIZ);
    rcu_assign_pointer!((*ndev).npinfo, npinfo);
    0
}

unsafe extern "C" fn rcu_cleanup_netpoll_info(rcu_head: *mut rcu_head) {
    let npinfo = container_of!(rcu_head, netpoll_info, rcu);
    skb_queue_purge(&mut (*npinfo).txq);
    kfree(npinfo);
}

unsafe fn __netpoll_cleanup(np: *mut netpoll) {
    let npinfo = rtnl_dereference((*np).dev.npinfo);
    if npinfo.is_null() { return; }
    if refcount_dec_and_test(&mut (*npinfo).refcnt) {
        let ops = (*(*np).dev).netdev_ops;
        if !(*ops).ndo_netpoll_cleanup.is_none() {
            ((*ops).ndo_netpoll_cleanup)((*np).dev);
        }
        RCU_INIT_POINTER!((*(*np).dev).npinfo, core::ptr::null_mut());
        disable_delayed_work_sync(&mut (*npinfo).tx_work);
        call_rcu(&mut (*npinfo).rcu, rcu_cleanup_netpoll_info);
    }
}

pub unsafe extern "C" fn __netpoll_free(np: *mut netpoll) {
    ASSERT_RTNL!();
    synchronize_net();
    __netpoll_cleanup(np);
    kfree(np);
}

pub unsafe extern "C" fn do_netpoll_cleanup(np: *mut netpoll) {
    __netpoll_cleanup(np);
    netdev_put((*np).dev, &mut (*np).dev_tracker);
    (*np).dev = core::ptr::null_mut();
}

pub unsafe extern "C" fn netpoll_cleanup(np: *mut netpoll) {
    rtnl_lock();
    if (*np).dev.is_null() {
        rtnl_unlock();
        return;
    }
    do_netpoll_cleanup(np);
    rtnl_unlock();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
