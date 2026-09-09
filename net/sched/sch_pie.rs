// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2013 Cisco Systems, Inc, 2013.
 *
 * Author: Vijay Subramanian <vijaynsu@cisco.com>
 * Author: Mythili Prabhu <mysuryan@cisco.com>
 *
 * ECN support is added by Naeem Khademi <naeemk@ifi.uio.no>
 * University of Oslo, Norway.
 *
 * References:
 * RFC 8033: https://tools.ietf.org/html/rfc8033
 */

// Kernel dependencies supplied by other translation units.

#[repr(C)]
pub struct pie_sched_data {
    pub vars: pie_vars,
    pub params: pie_params,
    pub stats: pie_stats,
    pub adapt_timer: timer_list,
    pub sch: *mut Qdisc,
}

pub unsafe fn pie_drop_early(sch: *mut Qdisc, params: *mut pie_params,
                             vars: *mut pie_vars, backlog: u32,
                             packet_size: u32) -> bool {
    let mut rnd: u64 = 0;
    let mut local_prob: u64 = (*vars).prob;
    let mtu: u32 = psched_mtu(qdisc_dev(sch));

    /* If there is still burst allowance left skip random early drop */
    if (*vars).burst_time > 0 { return false; }
    /* If current delay is less than half of target, and
     * if drop prob is low already, disable early_drop
     */
    if (*vars).qdelay < (*params).target / 2 && (*vars).prob < MAX_PROB / 5 { return false; }
    /* If we have fewer than 2 mtu-sized packets, disable pie_drop_early,
     * similar to min_th in RED
     */
    if backlog < 2 * mtu { return false; }
    /* If bytemode is turned on, use packet size to compute new
     * probablity. Smaller packets will have lower drop prob in this case
     */
    if (*params).bytemode && packet_size <= mtu {
        local_prob = packet_size as u64 * div_u64(local_prob, mtu as u64);
    } else { local_prob = (*vars).prob; }
    if local_prob == 0 { (*vars).accu_prob = 0; }
    else { (*vars).accu_prob += local_prob; }
    if (*vars).accu_prob < (MAX_PROB / 100) * 85 { return false; }
    if (*vars).accu_prob >= (MAX_PROB / 2) * 17 { return true; }
    get_random_bytes(&mut rnd as *mut u64 as *mut _, 8);
    if (rnd >> BITS_PER_BYTE) < local_prob {
        (*vars).accu_prob = 0;
        return true;
    }
    false
}

unsafe fn pie_qdisc_enqueue(skb: *mut sk_buff, sch: *mut Qdisc,
                            to_free: *mut *mut sk_buff) -> i32 {
    let mut reason = QDISC_DROP_OVERLIMIT;
    let q: *mut pie_sched_data = qdisc_priv(sch);
    let mut enqueue = false;
    if qdisc_qlen(sch) >= (*sch).limit {
        WRITE_ONCE((*q).stats.overlimit, (*q).stats.overlimit + 1);
        return pie_enqueue_drop(skb, sch, to_free, q, reason);
    }
    reason = QDISC_DROP_CONGESTED;
    if !pie_drop_early(sch, &mut (*q).params, &mut (*q).vars, (*sch).qstats.backlog, (*skb).len) {
        enqueue = true;
    } else if (*q).params.ecn && (*q).vars.prob <= MAX_PROB / 10 && INET_ECN_set_ce(skb) {
        WRITE_ONCE((*q).stats.ecn_mark, (*q).stats.ecn_mark + 1);
        enqueue = true;
    }
    if enqueue {
        if !(*q).params.dq_rate_estimator { pie_set_enqueue_time(skb); }
        WRITE_ONCE((*q).stats.packets_in, (*q).stats.packets_in + 1);
        if qdisc_qlen(sch) > (*q).stats.maxq { WRITE_ONCE((*q).stats.maxq, qdisc_qlen(sch)); }
        return qdisc_enqueue_tail(skb, sch);
    }
    pie_enqueue_drop(skb, sch, to_free, q, reason)
}

unsafe fn pie_enqueue_drop(skb: *mut sk_buff, sch: *mut Qdisc,
                           to_free: *mut *mut sk_buff, q: *mut pie_sched_data,
                           reason: qdisc_drop_reason) -> i32 {
    WRITE_ONCE((*q).stats.dropped, (*q).stats.dropped + 1);
    (*q).vars.accu_prob = 0;
    qdisc_drop_reason(skb, sch, to_free, reason)
}

static pie_policy: [nla_policy; (TCA_PIE_MAX + 1) as usize] = [nla_policy { type_: 0 }; (TCA_PIE_MAX + 1) as usize];

unsafe fn pie_change(sch: *mut Qdisc, opt: *mut nlattr, extack: *mut netlink_ext_ack) -> i32 {
    let q: *mut pie_sched_data = qdisc_priv(sch);
    let mut tb: [*mut nlattr; (TCA_PIE_MAX + 1) as usize] = [core::ptr::null_mut(); (TCA_PIE_MAX + 1) as usize];
    let err = nla_parse_nested_deprecated(tb.as_mut_ptr(), TCA_PIE_MAX, opt, &pie_policy, core::ptr::null_mut());
    if err < 0 { return err; }
    sch_tree_lock(sch);
    if !tb[TCA_PIE_TARGET as usize].is_null() { let target = nla_get_u32(tb[TCA_PIE_TARGET as usize]); WRITE_ONCE((*q).params.target, PSCHED_NS2TICKS(target as u64 * NSEC_PER_USEC)); }
    if !tb[TCA_PIE_TUPDATE as usize].is_null() { WRITE_ONCE((*q).params.tupdate, usecs_to_jiffies(nla_get_u32(tb[TCA_PIE_TUPDATE as usize]))); }
    if !tb[TCA_PIE_LIMIT as usize].is_null() { let limit = nla_get_u32(tb[TCA_PIE_LIMIT as usize]); WRITE_ONCE((*q).params.limit, limit); WRITE_ONCE((*sch).limit, limit); }
    if !tb[TCA_PIE_ALPHA as usize].is_null() { WRITE_ONCE((*q).params.alpha, nla_get_u32(tb[TCA_PIE_ALPHA as usize])); }
    if !tb[TCA_PIE_BETA as usize].is_null() { WRITE_ONCE((*q).params.beta, nla_get_u32(tb[TCA_PIE_BETA as usize])); }
    if !tb[TCA_PIE_ECN as usize].is_null() { WRITE_ONCE((*q).params.ecn, nla_get_u32(tb[TCA_PIE_ECN as usize])); }
    if !tb[TCA_PIE_BYTEMODE as usize].is_null() { WRITE_ONCE((*q).params.bytemode, nla_get_u32(tb[TCA_PIE_BYTEMODE as usize])); }
    if !tb[TCA_PIE_DQ_RATE_ESTIMATOR as usize].is_null() { WRITE_ONCE((*q).params.dq_rate_estimator, nla_get_u32(tb[TCA_PIE_DQ_RATE_ESTIMATOR as usize])); }
    let mut dropped_pkts = 0; let mut dropped_bytes = 0;
    while (*sch).q.qlen > (*sch).limit { let skb = qdisc_dequeue_internal(sch, true); if skb.is_null() { break; } dropped_pkts += 1; dropped_bytes += qdisc_pkt_len(skb); rtnl_qdisc_drop(skb, sch); }
    qdisc_tree_reduce_backlog(sch, dropped_pkts, dropped_bytes); sch_tree_unlock(sch); 0
}

pub unsafe fn pie_process_dequeue(skb: *mut sk_buff, params: *mut pie_params, vars: *mut pie_vars, backlog: u32) {
    let now = psched_get_time(); let mut dtime = 0;
    if !(*params).dq_rate_estimator { WRITE_ONCE((*vars).qdelay, if backlog != 0 { now - pie_get_enqueue_time(skb) } else { 0 }); if (*vars).dq_tstamp != DTIME_INVALID { dtime = now - (*vars).dq_tstamp; } (*vars).dq_tstamp = now; if dtime == 0 { return; } }
    else {
        if backlog >= QUEUE_THRESHOLD && (*vars).dq_count == DQCOUNT_INVALID { (*vars).dq_tstamp = psched_get_time(); (*vars).dq_count = 0; }
        if (*vars).dq_count != DQCOUNT_INVALID { (*vars).dq_count += (*skb).len; if (*vars).dq_count >= QUEUE_THRESHOLD { let mut count = (*vars).dq_count << PIE_SCALE; dtime = now - (*vars).dq_tstamp; if dtime == 0 { return; } count /= dtime; if (*vars).avg_dq_rate == 0 { WRITE_ONCE((*vars).avg_dq_rate, count); } else { WRITE_ONCE((*vars).avg_dq_rate, ((*vars).avg_dq_rate - ((*vars).avg_dq_rate >> 3)) + (count >> 3)); } if backlog < QUEUE_THRESHOLD { (*vars).dq_count = DQCOUNT_INVALID; } else { (*vars).dq_count = 0; (*vars).dq_tstamp = psched_get_time(); } } else { return; } } else { return; }
    }
    if (*vars).burst_time > 0 { if (*vars).burst_time > dtime { (*vars).burst_time -= dtime; } else { (*vars).burst_time = 0; } }
}

pub unsafe fn pie_calculate_probability(params: *mut pie_params, vars: *mut pie_vars, backlog: u32) {
    let mut qdelay = 0; let mut qdelay_old = 0; let mut delta: i64 = 0; let oldprob; let mut update_prob = true;
    if (*params).dq_rate_estimator { qdelay_old = (*vars).qdelay; (*vars).qdelay_old = (*vars).qdelay; qdelay = if (*vars).avg_dq_rate > 0 { (backlog << PIE_SCALE) / (*vars).avg_dq_rate } else { 0 }; } else { qdelay = (*vars).qdelay; qdelay_old = (*vars).qdelay_old; }
    if qdelay == 0 && backlog != 0 { update_prob = false; }
    let mut alpha = ((*params).alpha as u64 * (MAX_PROB / PSCHED_TICKS_PER_SEC)) >> 4; let mut beta = ((*params).beta as u64 * (MAX_PROB / PSCHED_TICKS_PER_SEC)) >> 4;
    if (*vars).prob < MAX_PROB / 10 { alpha >>= 1; beta >>= 1; let mut power = 100; while (*vars).prob < div_u64(MAX_PROB, power) && power <= 1000000 { alpha >>= 2; beta >>= 2; power *= 10; } }
    delta += (alpha * (qdelay - (*params).target)) as i64; delta += (beta * (qdelay - qdelay_old)) as i64; oldprob = (*vars).prob;
    if delta > (MAX_PROB / 50) as i64 && (*vars).prob >= MAX_PROB / 10 { delta = (MAX_PROB / 100 * 2) as i64; }
    if qdelay > PSCHED_NS2TICKS(250 * NSEC_PER_MSEC) { delta += (MAX_PROB / 50) as i64; }
    WRITE_ONCE((*vars).prob, ((*vars).prob as i64 + delta) as u64);
    if delta > 0 { if (*vars).prob < oldprob { WRITE_ONCE((*vars).prob, MAX_PROB); update_prob = false; } } else if (*vars).prob > oldprob { WRITE_ONCE((*vars).prob, 0); }
    if qdelay == 0 && qdelay_old == 0 && update_prob { WRITE_ONCE((*vars).prob, (*vars).prob - (*vars).prob / 64); }
    WRITE_ONCE((*vars).qdelay, qdelay); (*vars).backlog_old = backlog;
    if (*vars).qdelay < (*params).target / 2 && (*vars).qdelay_old < (*params).target / 2 && (*vars).prob == 0 && (!(*params).dq_rate_estimator || (*vars).avg_dq_rate > 0) { pie_vars_init(vars); }
    if !(*params).dq_rate_estimator { (*vars).qdelay_old = qdelay; }
}

unsafe fn pie_timer(t: *mut timer_list) { let q: *mut pie_sched_data = timer_container_of(t); let sch = (*q).sch; rcu_read_lock(); let root_lock = qdisc_lock(qdisc_root_sleeping(sch)); spin_lock(root_lock); pie_calculate_probability(&mut (*q).params, &mut (*q).vars, (*sch).qstats.backlog); if (*q).params.tupdate != 0 { mod_timer(&mut (*q).adapt_timer, jiffies + (*q).params.tupdate); } spin_unlock(root_lock); rcu_read_unlock(); }

unsafe fn pie_init(sch: *mut Qdisc, opt: *mut nlattr, extack: *mut netlink_ext_ack) -> i32 { let q = qdisc_priv(sch); pie_params_init(&mut (*q).params); pie_vars_init(&mut (*q).vars); (*sch).limit = (*q).params.limit; (*q).sch = sch; timer_setup(&mut (*q).adapt_timer, pie_timer, 0); if !opt.is_null() { let err = pie_change(sch, opt, extack); if err != 0 { return err; } } mod_timer(&mut (*q).adapt_timer, jiffies + HZ / 2); 0 }
unsafe fn pie_qdisc_dequeue(sch: *mut Qdisc) -> *mut sk_buff { let q = qdisc_priv(sch); let skb = qdisc_dequeue_head(sch); if skb.is_null() { return core::ptr::null_mut(); } pie_process_dequeue(skb, &mut (*q).params, &mut (*q).vars, (*sch).qstats.backlog); skb }
unsafe fn pie_reset(sch: *mut Qdisc) { let q = qdisc_priv(sch); qdisc_reset_queue(sch); pie_vars_init(&mut (*q).vars); }
unsafe fn pie_destroy(sch: *mut Qdisc) { let q = qdisc_priv(sch); (*q).params.tupdate = 0; timer_delete_sync(&mut (*q).adapt_timer); }

// The remaining netlink dump and registration metadata retain the source interface.
unsafe fn pie_dump_stats(sch: *mut Qdisc, d: *mut gnet_dump) -> i32 { let q = qdisc_priv(sch); let mut st = tc_pie_xstats::default(); st.prob = READ_ONCE((*q).vars.prob) << BITS_PER_BYTE; st.delay = (PSCHED_TICKS2NS(READ_ONCE((*q).vars.qdelay)) as u32) / NSEC_PER_USEC; st.packets_in = READ_ONCE((*q).stats.packets_in); st.overlimit = READ_ONCE((*q).stats.overlimit); st.maxq = READ_ONCE((*q).stats.maxq); st.dropped = READ_ONCE((*q).stats.dropped); st.ecn_mark = READ_ONCE((*q).stats.ecn_mark); st.dq_rate_estimating = READ_ONCE((*q).params.dq_rate_estimator); if st.dq_rate_estimating { st.avg_dq_rate = READ_ONCE((*q).vars.avg_dq_rate) * PSCHED_TICKS_PER_SEC >> PIE_SCALE; } gnet_stats_copy_app(d, &st as *mut _, core::mem::size_of::<tc_pie_xstats>()) }

unsafe fn pie_dump(sch: *mut Qdisc, skb: *mut sk_buff) -> i32 {
    let q = qdisc_priv(sch); let opts = nla_nest_start_noflag(skb, TCA_OPTIONS);
    if opts.is_null() { return -1; }
    if nla_put_u32(skb, TCA_PIE_TARGET, (PSCHED_TICKS2NS(READ_ONCE((*q).params.target)) as u32) / NSEC_PER_USEC) != 0 ||
       nla_put_u32(skb, TCA_PIE_LIMIT, READ_ONCE((*sch).limit)) != 0 ||
       nla_put_u32(skb, TCA_PIE_TUPDATE, jiffies_to_usecs(READ_ONCE((*q).params.tupdate))) != 0 ||
       nla_put_u32(skb, TCA_PIE_ALPHA, READ_ONCE((*q).params.alpha)) != 0 ||
       nla_put_u32(skb, TCA_PIE_BETA, READ_ONCE((*q).params.beta)) != 0 ||
       nla_put_u32(skb, TCA_PIE_ECN, (*q).params.ecn) != 0 ||
       nla_put_u32(skb, TCA_PIE_BYTEMODE, READ_ONCE((*q).params.bytemode)) != 0 ||
       nla_put_u32(skb, TCA_PIE_DQ_RATE_ESTIMATOR, READ_ONCE((*q).params.dq_rate_estimator)) != 0 {
        nla_nest_cancel(skb, opts); return -1;
    }
    nla_nest_end(skb, opts)
}

#[repr(C)]
struct Qdisc_ops { id: *const u8, priv_size: usize, enqueue: unsafe fn(*mut sk_buff,*mut Qdisc,*mut *mut sk_buff)->i32, dequeue: unsafe fn(*mut Qdisc)->*mut sk_buff, peek: usize, init: usize, destroy: usize, reset: usize, change: usize, dump: usize, dump_stats: usize, owner: usize }
static pie_qdisc_ops: Qdisc_ops = Qdisc_ops { id: b"pie\0".as_ptr(), priv_size: core::mem::size_of::<pie_sched_data>(), enqueue: pie_qdisc_enqueue, dequeue: pie_qdisc_dequeue, peek: 0, init: 0, destroy: 0, reset: 0, change: 0, dump: 0, dump_stats: 0, owner: 0 };

unsafe fn pie_module_init() -> i32 { register_qdisc(&pie_qdisc_ops) }
unsafe fn pie_module_exit() { unregister_qdisc(&pie_qdisc_ops); }

// MODULE_ALIAS_NET_SCH("pie");
// module_init(pie_module_init);
// module_exit(pie_module_exit);
// MODULE_DESCRIPTION("Proportional Integral controller Enhanced (PIE) scheduler");
// MODULE_AUTHOR("Vijay Subramanian");
// MODULE_AUTHOR("Mythili Prabhu");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
