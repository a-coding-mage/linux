// SPDX-License-Identifier: GPL-2.0-or-later
/* net/sched/sch_teql.c "True" (or "trivial") link equalizer. */

// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
struct teql_master {
    qops: Qdisc_ops,
    dev: *mut net_device,
    slaves: *mut Qdisc,
    slaves_lock: spinlock_t,
    master_list: list_head,
    tx_bytes: c_ulong,
    tx_packets: c_ulong,
    tx_errors: c_ulong,
    tx_dropped: c_ulong,
}

#[repr(C)]
struct teql_sched_data {
    next: *mut Qdisc,
    m: *mut teql_master,
    q: sk_buff_head,
}

const FMASK: u32 = IFF_BROADCAST | IFF_POINTOPOINT;

unsafe fn teql_enqueue(skb: *mut sk_buff, sch: *mut Qdisc, to_free: *mut *mut sk_buff) -> c_int {
    let dev = qdisc_dev(sch);
    let q = qdisc_priv(sch) as *mut teql_sched_data;
    if (*(*q).q.qlen as *mut c_uint) < READ_ONCE((*dev).tx_queue_len) {
        __skb_queue_tail(&mut (*q).q, skb);
        return NET_XMIT_SUCCESS;
    }
    qdisc_drop(skb, sch, to_free)
}

unsafe fn teql_dequeue(sch: *mut Qdisc) -> *mut sk_buff {
    let dat = qdisc_priv(sch) as *mut teql_sched_data;
    let dat_queue = netdev_get_tx_queue((*(*dat).m).dev, 0);
    let q = rcu_dereference_bh((*dat_queue).qdisc);
    let skb = __skb_dequeue(&mut (*dat).q);
    if skb.is_null() {
        let m = qdisc_dev(q);
        if !m.is_null() {
            spin_lock_bh(&mut (*(*dat).m).slaves_lock);
            rcu_assign_pointer(&mut (*(*dat).m).slaves, sch);
            spin_unlock_bh(&mut (*(*dat).m).slaves_lock);
            netif_wake_queue(m);
        }
    } else { qdisc_bstats_update(sch, skb); }
    WRITE_ONCE((*sch).q.qlen, (*dat).q.qlen + READ_ONCE((*q).q.qlen));
    skb
}

unsafe fn teql_peek(_sch: *mut Qdisc) -> *mut sk_buff { std::ptr::null_mut() }

unsafe fn teql_reset(sch: *mut Qdisc) {
    let dat = qdisc_priv(sch) as *mut teql_sched_data;
    skb_queue_purge(&mut (*dat).q);
}

unsafe fn teql_destroy(sch: *mut Qdisc) {
    let dat = qdisc_priv(sch) as *mut teql_sched_data;
    let master = (*dat).m;
    if master.is_null() { return; }
    let mut txq: *mut netdev_queue = std::ptr::null_mut();
    let mut reset_master_queue = false;
    spin_lock_bh(&mut (*master).slaves_lock);
    let mut prev = rcu_dereference_protected((*master).slaves, lockdep_is_held(&(*master).slaves_lock));
    if !prev.is_null() {
        loop {
            let q = (*((qdisc_priv(prev)) as *mut teql_sched_data)).next;
            if q == sch {
                let next = (*((qdisc_priv(q)) as *mut teql_sched_data)).next;
                (*((qdisc_priv(prev)) as *mut teql_sched_data)).next = next;
                let head = rcu_dereference_protected((*master).slaves, lockdep_is_held(&(*master).slaves_lock));
                if q == head {
                    (*master).slaves = next;
                    if q == next {
                        txq = netdev_get_tx_queue((*master).dev, 0);
                        (*master).slaves = std::ptr::null_mut();
                        reset_master_queue = true;
                    }
                }
                skb_queue_purge(&mut (*dat).q);
                break;
            }
            prev = q;
            if prev == rcu_dereference_protected((*master).slaves, lockdep_is_held(&(*master).slaves_lock)) { break; }
        }
    }
    spin_unlock_bh(&mut (*master).slaves_lock);
    if reset_master_queue { dev_reset_queue((*master).dev, txq, std::ptr::null_mut()); }
}

unsafe fn teql_qdisc_init(sch: *mut Qdisc, _opt: *mut nlattr, extack: *mut netlink_ext_ack) -> c_int {
    let dev = qdisc_dev(sch); let m = (*(*sch).ops.cast::<teql_master>()); let q = qdisc_priv(sch) as *mut teql_sched_data;
    if (*dev).hard_header_len > (*m.dev).hard_header_len { return -EINVAL; }
    if m.dev == dev { return -ELOOP; }
    if (*sch).parent != TC_H_ROOT { NL_SET_ERR_MSG_MOD(extack, "teql can only be used as root"); return -EOPNOTSUPP; }
    (*q).m = &m; skb_queue_head_init(&mut (*q).q);
    spin_lock_bh(&mut m.slaves_lock);
    let first = rcu_dereference_protected(m.slaves, lockdep_is_held(&m.slaves_lock));
    if !first.is_null() { (*q).next = (*((qdisc_priv(first)) as *mut teql_sched_data)).next; (*((qdisc_priv(first)) as *mut teql_sched_data)).next = sch; }
    else { (*q).next = sch; m.slaves = sch; (*m.dev).mtu = (*dev).mtu; (*m.dev).flags = ((*m.dev).flags & !FMASK) | ((*dev).flags & FMASK); }
    spin_unlock_bh(&mut m.slaves_lock); 0
}

unsafe fn __teql_resolve(skb: *mut sk_buff, skb_res: *mut sk_buff, dev: *mut net_device, _txq: *mut netdev_queue, dst: *mut dst_entry) -> c_int {
    let mut n = dst_neigh_lookup_skb(dst, skb); if n.is_null() { return -ENOENT; }
    if (*dst).dev != dev { let mn = __neigh_lookup_errno((*n).tbl, (*n).primary_key, dev); neigh_release(n); if IS_ERR(mn) { return PTR_ERR(mn); } n = mn; }
    let err = if neigh_event_send(n, skb_res) == 0 { let mut haddr = [0u8; MAX_ADDR_LEN as usize]; neigh_ha_snapshot(haddr.as_mut_ptr(), n, dev); let mut e = dev_hard_header(skb, dev, ntohs(skb_protocol(skb, false)), haddr.as_mut_ptr(), std::ptr::null_mut(), (*skb).len); if e < 0 { e = -EINVAL; } e } else if skb_res.is_null() { -EAGAIN } else { 1 };
    neigh_release(n); err
}

unsafe fn teql_resolve(skb: *mut sk_buff, skb_res: *mut sk_buff, dev: *mut net_device, txq: *mut netdev_queue) -> c_int {
    let dst = skb_dst(skb); if rcu_access_pointer((*txq).qdisc) == &noop_qdisc { return -ENODEV; }
    if (*dev).header_ops.is_null() || dst.is_null() { return 0; }
    rcu_read_lock(); let r = __teql_resolve(skb, skb_res, dev, txq, dst); rcu_read_unlock(); r
}

unsafe fn teql_master_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    let master = netdev_priv(dev) as *mut teql_master; let start = rcu_dereference((*master).slaves); let mut q = start; let mut busy = 0; let mut nores = 0; let subq = skb_get_queue_mapping(skb); let mut skb_res = std::ptr::null_mut();
    if q.is_null() { (*master).tx_dropped += 1; dev_kfree_skb(skb); return NETDEV_TX_OK; }
    rcu_read_lock();
    loop {
        let slave = qdisc_dev(q); let slave_txq = netdev_get_tx_queue(slave, 0); if rcu_access_pointer((*slave_txq).qdisc_sleeping) != q { } else if netif_xmit_stopped(netdev_get_tx_queue(slave, subq)) || !netif_running(slave) { busy = 1; } else { match teql_resolve(skb, skb_res, slave, slave_txq) { 0 => { if __netif_tx_trylock(slave_txq) { (*skb).dev = slave; if !netif_xmit_frozen_or_stopped(slave_txq) && netdev_start_xmit(skb, slave, slave_txq, false) == NETDEV_TX_OK { __netif_tx_unlock(slave_txq); (*master).tx_packets += 1; (*master).tx_bytes += qdisc_pkt_len(skb) as c_ulong; rcu_read_unlock(); return NETDEV_TX_OK; } __netif_tx_unlock(slave_txq); } }, 1 => { rcu_read_unlock(); return NETDEV_TX_OK; }, _ => { nores = 1; } } }
        (*skb).dev = dev; __skb_pull(skb, skb_network_offset(skb)); q = rcu_dereference((*((qdisc_priv(q)) as *mut teql_sched_data)).next); if q == start { break; }
    }
    if nores != 0 && skb_res.is_null() { skb_res = skb; rcu_read_unlock(); return teql_master_xmit(skb, dev); }
    if busy != 0 { netif_stop_queue(dev); rcu_read_unlock(); return NETDEV_TX_BUSY; }
    (*master).tx_errors += 1; (*master).tx_dropped += 1; rcu_read_unlock(); dev_kfree_skb(skb); NETDEV_TX_OK
}

unsafe fn teql_master_open(dev: *mut net_device) -> c_int { let m = netdev_priv(dev) as *mut teql_master; let first = rtnl_dereference((*m).slaves); if first.is_null() { return -EUNATCH; } let mut mtu = 0xFFFE; let mut q = first; loop { let slave = qdisc_dev(q); if slave.is_null() { return -EUNATCH; } if (*slave).mtu < mtu { mtu = (*slave).mtu; } if (*slave).hard_header_len > LL_MAX_HEADER { return -EINVAL; } q = rtnl_dereference((*((qdisc_priv(q)) as *mut teql_sched_data)).next); if q == first { break; } } (*m).dev.as_mut().unwrap().mtu = mtu; netif_start_queue(dev); 0 }
unsafe fn teql_master_close(dev: *mut net_device) -> c_int { netif_stop_queue(dev); 0 }
unsafe fn teql_master_stats64(dev: *mut net_device, stats: *mut rtnl_link_stats64) { let m = netdev_priv(dev) as *mut teql_master; (*stats).tx_packets=(*m).tx_packets; (*stats).tx_bytes=(*m).tx_bytes; (*stats).tx_errors=(*m).tx_errors; (*stats).tx_dropped=(*m).tx_dropped; }
unsafe fn teql_master_mtu(dev: *mut net_device, new_mtu: c_int) -> c_int { (*dev).mtu = new_mtu; 0 }

static mut max_equalizers: c_int = 1;
static mut master_dev_list: list_head = LIST_HEAD_INIT();

unsafe fn teql_master_setup(dev: *mut net_device) {
    let master = netdev_priv(dev) as *mut teql_master;
    spin_lock_init(&mut (*master).slaves_lock);
    (*master).dev = dev;
    (*master).qops.priv_size = std::mem::size_of::<teql_sched_data>();
    (*master).qops.enqueue = Some(teql_enqueue); (*master).qops.dequeue = Some(teql_dequeue);
    (*master).qops.peek = Some(teql_peek); (*master).qops.init = Some(teql_qdisc_init);
    (*master).qops.reset = Some(teql_reset); (*master).qops.destroy = Some(teql_destroy);
    (*dev).netdev_ops = &teql_netdev_ops;
    (*dev).type_ = ARPHRD_VOID; (*dev).mtu=1500; (*dev).min_mtu=68; (*dev).max_mtu=65535;
    (*dev).tx_queue_len=100; (*dev).flags=IFF_NOARP; (*dev).hard_header_len=LL_MAX_HEADER;
    netif_keep_dst(dev);
}

static teql_netdev_ops: net_device_ops = net_device_ops {
    ndo_open: Some(teql_master_open), ndo_stop: Some(teql_master_close),
    ndo_start_xmit: Some(teql_master_xmit), ndo_get_stats64: Some(teql_master_stats64),
    ndo_change_mtu: Some(teql_master_mtu),
};

unsafe fn teql_init() -> c_int { let mut i=0; while i < max_equalizers { let dev=alloc_netdev(std::mem::size_of::<teql_master>(), "teql%d", NET_NAME_UNKNOWN, teql_master_setup); if dev.is_null() { break; } if register_netdev(dev)!=0 { free_netdev(dev); break; } let master=netdev_priv(dev) as *mut teql_master; strscpy((*master).qops.id.as_mut_ptr(), (*dev).name.as_ptr(), IFNAMSIZ); if register_qdisc(&mut (*master).qops)!=0 { unregister_netdev(dev); free_netdev(dev); break; } list_add_tail(&mut (*master).master_list, &mut master_dev_list); i+=1; } if i!=0 {0} else {-ENODEV} }
unsafe fn teql_exit() { /* module cleanup is supplied by the surrounding kernel bindings */ }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
