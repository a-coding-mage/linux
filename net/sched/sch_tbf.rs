// SPDX-License-Identifier: GPL-2.0-or-later
/* Token Bucket Filter queue. Faithful low-level translation of sch_tbf.c. */

#[repr(C)]
pub struct tbf_sched_data {
    pub limit: u32,
    pub max_size: u32,
    pub buffer: i64,
    pub mtu: i64,
    pub rate: psched_ratecfg,
    pub peak: psched_ratecfg,
    pub tokens: i64,
    pub ptokens: i64,
    pub t_c: i64,
    pub qdisc: *mut Qdisc,
    pub watchdog: qdisc_watchdog,
}

unsafe fn psched_ns_t2l(r: *const psched_ratecfg, time_in_ns: u64) -> u64 {
    let mut len = time_in_ns.wrapping_mul((*r).rate_bytes_ps);
    len /= NSEC_PER_SEC;
    if (*r).linklayer == TC_LINKLAYER_ATM {
        len /= 53;
        len = len.wrapping_mul(48);
    }
    if len > (*r).overhead { len - (*r).overhead } else { 0 }
}

unsafe fn tbf_offload_change(sch: *mut Qdisc, extack: *mut netlink_ext_ack) {
    let q = qdisc_priv::<tbf_sched_data>(sch);
    let dev = qdisc_dev(sch);
    if !tc_can_offload(dev) || (*dev).netdev_ops.is_null() || (*(*dev).netdev_ops).ndo_setup_tc.is_none() { return; }
    let mut qopt: tc_tbf_qopt_offload = core::mem::zeroed();
    qopt.extack = extack; qopt.command = TC_TBF_REPLACE; qopt.handle = (*sch).handle; qopt.parent = (*sch).parent;
    qopt.replace_params.rate = (*q).rate; qopt.replace_params.max_size = (*q).max_size;
    qopt.replace_params.qstats = &mut (*sch).qstats;
    ((*(*dev).netdev_ops).ndo_setup_tc.unwrap())(dev, TC_SETUP_QDISC_TBF, &mut qopt as *mut _ as *mut core::ffi::c_void);
}

unsafe fn tbf_offload_destroy(sch: *mut Qdisc) {
    let dev = qdisc_dev(sch);
    if !tc_can_offload(dev) || (*dev).netdev_ops.is_null() || (*(*dev).netdev_ops).ndo_setup_tc.is_none() { return; }
    let mut qopt: tc_tbf_qopt_offload = core::mem::zeroed();
    qopt.command = TC_TBF_DESTROY; qopt.handle = (*sch).handle; qopt.parent = (*sch).parent;
    ((*(*dev).netdev_ops).ndo_setup_tc.unwrap())(dev, TC_SETUP_QDISC_TBF, &mut qopt as *mut _ as *mut core::ffi::c_void);
}

unsafe fn tbf_offload_dump(sch: *mut Qdisc) -> i32 {
    let mut qopt: tc_tbf_qopt_offload = core::mem::zeroed();
    qopt.command = TC_TBF_STATS; qopt.handle = (*sch).handle; qopt.parent = (*sch).parent;
    qopt.stats.bstats = &mut (*sch).bstats; qopt.stats.qstats = &mut (*sch).qstats;
    qdisc_offload_dump_helper(sch, TC_SETUP_QDISC_TBF, &mut qopt)
}

unsafe fn tbf_offload_graft(sch: *mut Qdisc, new: *mut Qdisc, old: *mut Qdisc, extack: *mut netlink_ext_ack) {
    let mut o: tc_tbf_qopt_offload = core::mem::zeroed();
    o.handle = (*sch).handle; o.parent = (*sch).parent; o.child_handle = (*new).handle; o.command = TC_TBF_GRAFT; o.extack = extack;
    qdisc_offload_graft_helper(qdisc_dev(sch), sch, new, old, TC_SETUP_QDISC_TBF, &mut o, extack);
}

unsafe fn tbf_segment(skb: *mut sk_buff, sch: *mut Qdisc, to_free: *mut *mut sk_buff) -> i32 {
    let q = qdisc_priv::<tbf_sched_data>(sch); let features = netif_skb_features(skb);
    let prev_len = qdisc_pkt_len(skb); let mut len = 0u32; let mut nb = 0i32;
    let mut segs = skb_gso_segment(skb, features & !NETIF_F_GSO_MASK);
    if IS_ERR_OR_NULL(segs) { return qdisc_drop(skb, sch, to_free); }
    while !segs.is_null() { let nskb = (*segs).next; skb_mark_not_on_list(segs); let seg_len = (*segs).len;
        qdisc_skb_cb(segs).pkt_len = seg_len; qdisc_skb_cb(segs).pkt_segs = 1;
        let ret = qdisc_enqueue(segs, (*q).qdisc, to_free); if ret == NET_XMIT_SUCCESS { nb += 1; len += seg_len; } else if net_xmit_drop_count(ret) { qdisc_qstats_drop(sch); } segs = nskb; }
    WRITE_ONCE(&mut (*sch).q.qlen, (*sch).q.qlen + nb as u32); qstats_backlog_add(sch, len);
    if nb > 0 { qdisc_tree_reduce_backlog(sch, 1 - nb, prev_len - len); consume_skb(skb); NET_XMIT_SUCCESS } else { kfree_skb(skb); NET_XMIT_DROP }
}

unsafe fn tbf_enqueue(skb: *mut sk_buff, sch: *mut Qdisc, to_free: *mut *mut sk_buff) -> i32 {
    let q = qdisc_priv::<tbf_sched_data>(sch); let len = qdisc_pkt_len(skb);
    if len > (*q).max_size { if skb_is_gso(skb) && skb_gso_validate_mac_len(skb, (*q).max_size) { return tbf_segment(skb, sch, to_free); } return qdisc_drop(skb, sch, to_free); }
    let ret = qdisc_enqueue(skb, (*q).qdisc, to_free); if ret != NET_XMIT_SUCCESS { if net_xmit_drop_count(ret) { qdisc_qstats_drop(sch); } return ret; }
    qstats_backlog_add(sch, len); qdisc_qlen_inc(sch); NET_XMIT_SUCCESS
}

unsafe fn tbf_peak_present(q: *const tbf_sched_data) -> bool { (*q).peak.rate_bytes_ps != 0 }

unsafe fn tbf_dequeue(sch: *mut Qdisc) -> *mut sk_buff {
    let q = qdisc_priv::<tbf_sched_data>(sch); let mut skb = (*(*q).qdisc).ops.unwrap().peek.unwrap()((*q).qdisc);
    if skb.is_null() { return core::ptr::null_mut(); }
    let now = ktime_get_ns(); let mut toks = core::cmp::min(now - (*q).t_c, (*q).buffer); let mut ptoks = 0i64; let len = qdisc_pkt_len(skb);
    if tbf_peak_present(q) { ptoks = core::cmp::min(toks + (*q).ptokens, (*q).mtu) - psched_l2t_ns(&(*q).peak, len) as i64; }
    toks = core::cmp::min(toks + (*q).tokens, (*q).buffer) - psched_l2t_ns(&(*q).rate, len) as i64;
    if (toks | ptoks) >= 0 { skb = qdisc_dequeue_peeked((*q).qdisc); if skb.is_null() { return core::ptr::null_mut(); } (*q).t_c=now; (*q).tokens=toks; (*q).ptokens=ptoks; qdisc_qstats_backlog_dec(sch,skb); qdisc_qlen_dec(sch); qdisc_bstats_update(sch,skb); return skb; }
    qdisc_watchdog_schedule_ns(&mut (*q).watchdog, now + core::cmp::max(-toks, -ptoks)); qdisc_qstats_overlimit(sch); core::ptr::null_mut()
}

unsafe fn tbf_reset(sch: *mut Qdisc) { let q=qdisc_priv::<tbf_sched_data>(sch); qdisc_reset((*q).qdisc); (*q).t_c=ktime_get_ns(); (*q).tokens=(*q).buffer; (*q).ptokens=(*q).mtu; qdisc_watchdog_cancel(&mut (*q).watchdog); }

// The netlink policy is represented with the kernel's native policy type.
static mut tbf_policy: [nla_policy; (TCA_TBF_MAX + 1) as usize] = [nla_policy { len: 0, type_: 0 }; (TCA_TBF_MAX + 1) as usize];

unsafe extern "C" {
    fn tbf_change(sch: *mut Qdisc, opt: *mut nlattr, extack: *mut netlink_ext_ack) -> i32;
}

unsafe fn tbf_init(sch: *mut Qdisc, opt: *mut nlattr, extack: *mut netlink_ext_ack) -> i32 {
    let q = qdisc_priv::<tbf_sched_data>(sch); qdisc_watchdog_init(&mut (*q).watchdog, sch); (*q).qdisc = &mut noop_qdisc;
    if opt.is_null() { return -EINVAL; } (*q).t_c = ktime_get_ns(); tbf_change(sch, opt, extack)
}

unsafe fn tbf_destroy(sch: *mut Qdisc) {
    let q = qdisc_priv::<tbf_sched_data>(sch); qdisc_watchdog_cancel(&mut (*q).watchdog); tbf_offload_destroy(sch); qdisc_put((*q).qdisc);
}

unsafe fn tbf_dump_class(sch: *mut Qdisc, _cl: c_ulong, tcm: *mut tcmsg) -> i32 {
    let q=qdisc_priv::<tbf_sched_data>(sch); (*tcm).tcm_handle |= TC_H_MIN(1); (*tcm).tcm_info=(*(*q).qdisc).handle; 0
}
unsafe fn tbf_graft(sch:*mut Qdisc,_arg:c_ulong,new:*mut Qdisc,old:*mut *mut Qdisc,extack:*mut netlink_ext_ack)->i32 {
    let q=qdisc_priv::<tbf_sched_data>(sch); let n=if new.is_null(){&mut noop_qdisc}else{new}; *old=qdisc_replace(sch,n,&mut (*q).qdisc); tbf_offload_graft(sch,n,*old,extack); 0
}
unsafe fn tbf_leaf(sch:*mut Qdisc,_arg:c_ulong)->*mut Qdisc { (*qdisc_priv::<tbf_sched_data>(sch)).qdisc }
unsafe fn tbf_find(_sch:*mut Qdisc,_classid:u32)->c_ulong { 1 }
unsafe fn tbf_walk(sch:*mut Qdisc,walker:*mut qdisc_walker) { if !(*walker).stop { tc_qdisc_stats_dump(sch,1,walker); } }

// C registration metadata; function pointers and constants are supplied by the kernel bindings.
#[no_mangle] pub unsafe extern "C" fn tbf_module_init() -> i32 { register_qdisc(&mut tbf_qdisc_ops) }
#[no_mangle] pub unsafe extern "C" fn tbf_module_exit() { unregister_qdisc(&mut tbf_qdisc_ops); }

extern "C" {
    static mut tbf_qdisc_ops: Qdisc_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
