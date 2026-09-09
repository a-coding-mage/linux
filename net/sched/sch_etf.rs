// SPDX-License-Identifier: GPL-2.0
//
// net/sched/sch_etf.c  Earliest TxTime First queueing discipline.

// C kernel dependencies are supplied by the surrounding translation unit.

macro_rules! DEADLINE_MODE_IS_ON { ($x:expr) => { ($x).flags & TC_ETF_DEADLINE_MODE_ON }; }
macro_rules! OFFLOAD_IS_ON { ($x:expr) => { ($x).flags & TC_ETF_OFFLOAD_ON }; }
macro_rules! SKIP_SOCK_CHECK_IS_SET { ($x:expr) => { ($x).flags & TC_ETF_SKIP_SOCK_CHECK }; }

#[repr(C)]
struct etf_sched_data {
    offload: bool,
    deadline_mode: bool,
    skip_sock_check: bool,
    clockid: i32,
    queue: i32,
    delta: s32,
    last: ktime_t,
    head: rb_root_cached,
    watchdog: qdisc_watchdog,
    get_time: Option<unsafe extern "C" fn() -> ktime_t>,
}

static etf_policy: [nla_policy; TCA_ETF_MAX as usize + 1] = {
    let mut p = [nla_policy { len: 0 }; TCA_ETF_MAX as usize + 1];
    p[TCA_ETF_PARMS as usize].len = core::mem::size_of::<tc_etf_qopt>() as u16;
    p
};

unsafe extern "C" fn validate_input_params(qopt: *mut tc_etf_qopt, extack: *mut netlink_ext_ack) -> i32 {
    if (*qopt).clockid < 0 { NL_SET_ERR_MSG(extack, "Dynamic clockids are not supported"); return -ENOTSUPP; }
    if (*qopt).clockid != CLOCK_TAI { NL_SET_ERR_MSG(extack, "Invalid clockid. CLOCK_TAI must be used"); return -EINVAL; }
    if (*qopt).delta < 0 { NL_SET_ERR_MSG(extack, "Delta must be positive"); return -EINVAL; }
    0
}

unsafe extern "C" fn is_packet_valid(sch: *mut Qdisc, nskb: *mut sk_buff) -> bool {
    let q = qdisc_priv::<etf_sched_data>(sch);
    let txtime = (*nskb).tstamp;
    let sk = (*nskb).sk;
    if (*q).skip_sock_check { return !ktime_before(txtime, ((*q).get_time.unwrap())()) && !ktime_before(txtime, (*q).last); }
    if sk.is_null() || !sk_fullsock(sk) || !sock_flag(sk, SOCK_TXTIME) || (*sk).sk_clockid != (*q).clockid || (*sk).sk_txtime_deadline_mode != (*q).deadline_mode { return false; }
    let now = ((*q).get_time.unwrap())();
    !ktime_before(txtime, now) && !ktime_before(txtime, (*q).last)
}

unsafe extern "C" fn etf_peek_timesortedlist(sch: *mut Qdisc) -> *mut sk_buff {
    let q = qdisc_priv::<etf_sched_data>(sch);
    let p = rb_first_cached(&mut (*q).head);
    if p.is_null() { core::ptr::null_mut() } else { rb_to_skb(p) }
}

unsafe extern "C" fn reset_watchdog(sch: *mut Qdisc) {
    let q = qdisc_priv::<etf_sched_data>(sch); let skb = etf_peek_timesortedlist(sch);
    if skb.is_null() { qdisc_watchdog_cancel(&mut (*q).watchdog); return; }
    let next = ktime_sub_ns((*skb).tstamp, (*q).delta as i64);
    qdisc_watchdog_schedule_ns(&mut (*q).watchdog, ktime_to_ns(next));
}

unsafe extern "C" fn report_sock_error(skb: *mut sk_buff, err: u32, code: u8) {
    let sk = (*skb).sk;
    if sk.is_null() || !sk_fullsock(sk) || !(*sk).sk_txtime_report_errors { return; }
    let clone = skb_clone(skb, GFP_ATOMIC); if clone.is_null() { return; }
    let serr = SKB_EXT_ERR(clone); let txtime = (*skb).tstamp;
    (*serr).ee.ee_errno = err; (*serr).ee.ee_origin = SO_EE_ORIGIN_TXTIME; (*serr).ee.ee_type = 0; (*serr).ee.ee_code = code; (*serr).ee.ee_pad = 0;
    (*serr).ee.ee_data = (txtime >> 32) as u32; (*serr).ee.ee_info = txtime as u32;
    if sock_queue_err_skb(sk, clone) != 0 { kfree_skb(clone); }
}

unsafe extern "C" fn etf_enqueue_timesortedlist(nskb: *mut sk_buff, sch: *mut Qdisc, to_free: *mut *mut sk_buff) -> i32 {
    let q = qdisc_priv::<etf_sched_data>(sch); let mut p = &mut (*q).head.rb_root.rb_node as *mut *mut rb_node; let mut parent = core::ptr::null_mut(); let mut leftmost = true; let txtime = (*nskb).tstamp;
    if !is_packet_valid(sch, nskb) { report_sock_error(nskb, EINVAL as u32, SO_EE_CODE_TXTIME_INVALID_PARAM); return qdisc_drop(nskb, sch, to_free); }
    while !(*p).is_null() { parent = *p; let skb = rb_to_skb(parent); if ktime_compare(txtime, (*skb).tstamp) >= 0 { p = &mut (*parent).rb_right; leftmost = false; } else { p = &mut (*parent).rb_left; } }
    rb_link_node(&mut (*nskb).rbnode, parent, p); rb_insert_color_cached(&mut (*nskb).rbnode, &mut (*q).head, leftmost); qdisc_qstats_backlog_inc(sch, nskb); qdisc_qlen_inc(sch); reset_watchdog(sch); NET_XMIT_SUCCESS
}

unsafe extern "C" fn timesortedlist_drop(sch: *mut Qdisc, mut skb: *mut sk_buff, now: ktime_t) {
    let q = qdisc_priv::<etf_sched_data>(sch); let mut to_free = core::ptr::null_mut(); let mut tmp = core::ptr::null_mut();
    skb_rbtree_walk_from_safe!(skb, tmp) { if ktime_after((*skb).tstamp, now) { break; } rb_erase_cached(&mut (*skb).rbnode, &mut (*q).head); (*skb).next = core::ptr::null_mut(); (*skb).prev = core::ptr::null_mut(); (*skb).dev = qdisc_dev(sch); report_sock_error(skb, ECANCELED as u32, SO_EE_CODE_TXTIME_MISSED); qdisc_qstats_backlog_dec(sch, skb); qdisc_drop(skb, sch, &mut to_free); qdisc_qstats_overlimit(sch); qdisc_qlen_dec(sch); }
    kfree_skb_list(to_free);
}

unsafe extern "C" fn timesortedlist_remove(sch: *mut Qdisc, skb: *mut sk_buff) { let q = qdisc_priv::<etf_sched_data>(sch); rb_erase_cached(&mut (*skb).rbnode, &mut (*q).head); (*skb).next = core::ptr::null_mut(); (*skb).prev = core::ptr::null_mut(); (*skb).dev = qdisc_dev(sch); qdisc_qstats_backlog_dec(sch, skb); qdisc_bstats_update(sch, skb); (*q).last = (*skb).tstamp; qdisc_qlen_dec(sch); }

unsafe extern "C" fn etf_dequeue_timesortedlist(sch: *mut Qdisc) -> *mut sk_buff {
    let q = qdisc_priv::<etf_sched_data>(sch); let mut skb = etf_peek_timesortedlist(sch); if skb.is_null() { return skb; } let now = ((*q).get_time.unwrap())();
    if ktime_before((*skb).tstamp, now) { timesortedlist_drop(sch, skb, now); skb = core::ptr::null_mut(); } else if (*q).deadline_mode { timesortedlist_remove(sch, skb); (*skb).tstamp = now; } else { let next = ktime_sub_ns((*skb).tstamp, (*q).delta as i64); if ktime_after(now, next) { timesortedlist_remove(sch, skb); } else { skb = core::ptr::null_mut(); } }
    reset_watchdog(sch); skb
}

unsafe extern "C" fn etf_disable_offload(dev: *mut net_device, q: *mut etf_sched_data) { if !(*q).offload { return; } let ops = (*dev).netdev_ops; if ops.is_null() || (*ops).ndo_setup_tc.is_none() { return; } let mut etf = tc_etf_qopt_offload::default(); etf.queue = (*q).queue; if ((*ops).ndo_setup_tc.unwrap())(dev, TC_SETUP_QDISC_ETF, &mut etf as *mut _ as *mut _) < 0 { pr_warn!("Couldn't disable ETF offload for queue {}\n", etf.queue); } }
unsafe extern "C" fn etf_enable_offload(dev: *mut net_device, q: *mut etf_sched_data, extack: *mut netlink_ext_ack) -> i32 { let ops = (*dev).netdev_ops; if ops.is_null() || (*ops).ndo_setup_tc.is_none() { NL_SET_ERR_MSG(extack, "Specified device does not support ETF offload"); return -EOPNOTSUPP; } let mut etf = tc_etf_qopt_offload::default(); etf.queue = (*q).queue; etf.enable = 1; let err = ((*ops).ndo_setup_tc.unwrap())(dev, TC_SETUP_QDISC_ETF, &mut etf as *mut _ as *mut _); if err < 0 { NL_SET_ERR_MSG(extack, "Specified device failed to setup ETF hardware offload"); } err }

unsafe extern "C" fn etf_init(sch: *mut Qdisc, opt: *mut nlattr, extack: *mut netlink_ext_ack) -> i32 {
    let q = qdisc_priv::<etf_sched_data>(sch); let dev = qdisc_dev(sch); let mut tb = [core::ptr::null_mut(); TCA_ETF_MAX as usize + 1];
    if opt.is_null() { NL_SET_ERR_MSG(extack, "Missing ETF qdisc options which are mandatory"); return -EINVAL; }
    let mut err = nla_parse_nested_deprecated(tb.as_mut_ptr(), TCA_ETF_MAX, opt, &etf_policy, extack); if err < 0 { return err; }
    if tb[TCA_ETF_PARMS as usize].is_null() { NL_SET_ERR_MSG(extack, "Missing mandatory ETF parameters"); return -EINVAL; }
    let qopt = nla_data::<tc_etf_qopt>(tb[TCA_ETF_PARMS as usize]); err = validate_input_params(qopt, extack); if err < 0 { return err; }
    (*q).queue = (*sch).dev_queue - netdev_get_tx_queue(dev, 0);
    if OFFLOAD_IS_ON!(*qopt) != 0 { err = etf_enable_offload(dev, q, extack); if err < 0 { return err; } }
    (*q).delta = (*qopt).delta; (*q).clockid = (*qopt).clockid; (*q).offload = OFFLOAD_IS_ON!(*qopt) != 0; (*q).deadline_mode = DEADLINE_MODE_IS_ON!(*qopt) != 0; (*q).skip_sock_check = SKIP_SOCK_CHECK_IS_SET!(*qopt) != 0;
    (*q).get_time = match (*q).clockid { CLOCK_REALTIME => Some(ktime_get_real), CLOCK_MONOTONIC => Some(ktime_get), CLOCK_BOOTTIME => Some(ktime_get_boottime), CLOCK_TAI => Some(ktime_get_clocktai), _ => { NL_SET_ERR_MSG(extack, "Clockid is not supported"); return -ENOTSUPP; } };
    qdisc_watchdog_init_clockid(&mut (*q).watchdog, sch, (*q).clockid); 0
}

unsafe extern "C" fn timesortedlist_clear(sch: *mut Qdisc) { let q = qdisc_priv::<etf_sched_data>(sch); let mut p = rb_first_cached(&mut (*q).head); while !p.is_null() { let skb = rb_to_skb(p); p = rb_next(p); rb_erase_cached(&mut (*skb).rbnode, &mut (*q).head); rtnl_kfree_skbs(skb, skb); qdisc_qlen_dec(sch); } }
unsafe extern "C" fn etf_reset(sch: *mut Qdisc) { let q = qdisc_priv::<etf_sched_data>(sch); if (*q).watchdog.qdisc == sch { qdisc_watchdog_cancel(&mut (*q).watchdog); } timesortedlist_clear(sch); __qdisc_reset_queue(&mut (*sch).q); (*q).last = 0; }
unsafe extern "C" fn etf_destroy(sch: *mut Qdisc) { let q = qdisc_priv::<etf_sched_data>(sch); if (*q).watchdog.qdisc == sch { qdisc_watchdog_cancel(&mut (*q).watchdog); } etf_disable_offload(qdisc_dev(sch), q); }
unsafe extern "C" fn etf_dump(sch: *mut Qdisc, skb: *mut sk_buff) -> i32 { let q = qdisc_priv::<etf_sched_data>(sch); let mut opt = tc_etf_qopt::default(); let nest = nla_nest_start_noflag(skb, TCA_OPTIONS); if nest.is_null() { return -1; } opt.delta = (*q).delta; opt.clockid = (*q).clockid; if (*q).offload { opt.flags |= TC_ETF_OFFLOAD_ON; } if (*q).deadline_mode { opt.flags |= TC_ETF_DEADLINE_MODE_ON; } if (*q).skip_sock_check { opt.flags |= TC_ETF_SKIP_SOCK_CHECK; } if nla_put(skb, TCA_ETF_PARMS, core::mem::size_of_val(&opt) as u16, &opt as *const _ as *const _) != 0 { nla_nest_cancel(skb, nest); return -1; } nla_nest_end(skb, nest) }

static mut etf_qdisc_ops: Qdisc_ops = Qdisc_ops { id: *b"etf\0", priv_size: core::mem::size_of::<etf_sched_data>(), enqueue: Some(etf_enqueue_timesortedlist), dequeue: Some(etf_dequeue_timesortedlist), peek: Some(etf_peek_timesortedlist), init: Some(etf_init), reset: Some(etf_reset), destroy: Some(etf_destroy), dump: Some(etf_dump), owner: THIS_MODULE };

unsafe extern "C" fn etf_module_init() -> i32 { register_qdisc(&mut etf_qdisc_ops) }
unsafe extern "C" fn etf_module_exit() { unregister_qdisc(&mut etf_qdisc_ops); }

// MODULE_ALIAS_NET_SCH("etf");
// module_init(etf_module_init); module_exit(etf_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Earliest TxTime First (ETF) qdisc");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
