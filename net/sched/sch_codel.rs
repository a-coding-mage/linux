// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
/*
 * Codel - The Controlled-Delay Active Queue Management algorithm
 *
 *  Copyright (C) 2011-2012 Kathleen Nichols <nichols@pollere.com>
 *  Copyright (C) 2011-2012 Van Jacobson <van@pollere.net>
 *
 *  Implemented on linux by :
 *  Copyright (C) 2012 Michael D. Taht <dave.taht@bufferbloat.net>
 *  Copyright (C) 2012,2015 Eric Dumazet <edumazet@google.com>
 */

// Linux dependencies supplied by other translation units.

const DEFAULT_CODEL_LIMIT: u32 = 1000;

#[repr(C)]
struct codel_sched_data {
    params: codel_params,
    vars: codel_vars,
    stats: codel_stats,
    drop_overlimit: u32,
}

/* This is the specific function called from codel_dequeue()
 * to dequeue a packet from queue. Note: backlog is handled in
 * codel, we dont need to reduce it here.
 */
unsafe fn dequeue_func(_vars: *mut codel_vars, ctx: *mut core::ffi::c_void) -> *mut sk_buff {
    let sch = ctx as *mut Qdisc;
    let skb = __qdisc_dequeue_head(&mut (*sch).q);

    if !skb.is_null() {
        qstats_backlog_sub(sch, qdisc_pkt_len(skb));
        prefetch(&(*skb).end); /* we'll need skb_shinfo() */
    }
    skb
}

unsafe fn drop_func(skb: *mut sk_buff, ctx: *mut core::ffi::c_void) {
    let sch = ctx as *mut Qdisc;
    qdisc_dequeue_drop(sch, skb, QDISC_DROP_CONGESTED);
    qdisc_qstats_drop(sch);
}

unsafe fn __codel_qdisc_dequeue(sch: *mut Qdisc) -> *mut sk_buff {
    let q = qdisc_priv(sch) as *mut codel_sched_data;
    let skb = codel_dequeue(
        sch,
        &mut (*sch).qstats.backlog,
        &mut (*q).params,
        &mut (*q).vars,
        &mut (*q).stats,
        qdisc_pkt_len,
        codel_get_enqueue_time,
        drop_func,
        dequeue_func,
    );

    if !skb.is_null() {
        qdisc_bstats_update(sch, skb);
    }
    skb
}

unsafe fn codel_dequeue_drop(sch: *mut Qdisc) {
    let q = qdisc_priv(sch) as *mut codel_sched_data;
    if (*q).stats.drop_count != 0 {
        qdisc_tree_reduce_backlog(sch, (*q).stats.drop_count, (*q).stats.drop_len);
        (*q).stats.drop_count = 0;
        (*q).stats.drop_len = 0;
    }
}

unsafe fn codel_qdisc_dequeue(sch: *mut Qdisc) -> *mut sk_buff {
    let skb = __codel_qdisc_dequeue(sch);
    codel_dequeue_drop(sch);
    skb
}

unsafe fn codel_peek(sch: *mut Qdisc) -> *mut sk_buff {
    let mut skb = skb_peek(&mut (*sch).gso_skb);
    if skb.is_null() {
        skb = __codel_qdisc_dequeue(sch);
        if !skb.is_null() {
            __skb_queue_head(&mut (*sch).gso_skb, skb);
            /* it's still part of the queue */
            qdisc_qstats_backlog_inc(sch, skb);
            (*sch).q.qlen += 1;
        }
        codel_dequeue_drop(sch);
    }
    skb
}

unsafe fn codel_qdisc_enqueue(skb: *mut sk_buff, sch: *mut Qdisc, to_free: *mut *mut sk_buff) -> i32 {
    if qdisc_qlen(sch) < (*sch).limit {
        codel_set_enqueue_time(skb);
        return qdisc_enqueue_tail(skb, sch);
    }
    let q = qdisc_priv(sch) as *mut codel_sched_data;
    (*q).drop_overlimit = (*q).drop_overlimit.wrapping_add(1);
    qdisc_drop_reason(skb, sch, to_free, QDISC_DROP_OVERLIMIT)
}

static mut codel_policy: [nla_policy; (TCA_CODEL_MAX + 1) as usize] = [nla_policy { type_: NLA_UNSPEC }; (TCA_CODEL_MAX + 1) as usize];

unsafe fn codel_change(sch: *mut Qdisc, opt: *mut nlattr, extack: *mut netlink_ext_ack) -> i32 {
    let mut dropped_pkts: u32 = 0;
    let mut dropped_bytes: u32 = 0;
    let q = qdisc_priv(sch) as *mut codel_sched_data;
    let mut tb: [*mut nlattr; (TCA_CODEL_MAX + 1) as usize] = [core::ptr::null_mut(); (TCA_CODEL_MAX + 1) as usize];
    let err = nla_parse_nested_deprecated(tb.as_mut_ptr(), TCA_CODEL_MAX, opt, codel_policy.as_ptr(), core::ptr::null_mut());
    if err < 0 { return err; }
    sch_tree_lock(sch);
    if !tb[TCA_CODEL_TARGET as usize].is_null() {
        let target = nla_get_u32(tb[TCA_CODEL_TARGET as usize]);
        (*q).params.target = (((target as u64) * NSEC_PER_USEC as u64) >> CODEL_SHIFT) as codel_time_t;
    }
    if !tb[TCA_CODEL_CE_THRESHOLD as usize].is_null() {
        let val = nla_get_u32(tb[TCA_CODEL_CE_THRESHOLD as usize]) as u64;
        (*q).params.ce_threshold = ((val * NSEC_PER_USEC as u64) >> CODEL_SHIFT) as codel_time_t;
    }
    if !tb[TCA_CODEL_INTERVAL as usize].is_null() {
        let interval = nla_get_u32(tb[TCA_CODEL_INTERVAL as usize]);
        (*q).params.interval = (((interval as u64) * NSEC_PER_USEC as u64) >> CODEL_SHIFT) as codel_time_t;
    }
    if !tb[TCA_CODEL_LIMIT as usize].is_null() { (*sch).limit = nla_get_u32(tb[TCA_CODEL_LIMIT as usize]); }
    if !tb[TCA_CODEL_ECN as usize].is_null() { (*q).params.ecn = (nla_get_u32(tb[TCA_CODEL_ECN as usize]) != 0) as u32; }
    while (*sch).q.qlen > (*sch).limit {
        let skb = qdisc_dequeue_internal(sch, true);
        if skb.is_null() { break; }
        dropped_pkts += 1;
        dropped_bytes += qdisc_pkt_len(skb);
        rtnl_qdisc_drop(skb, sch);
    }
    qdisc_tree_reduce_backlog(sch, dropped_pkts, dropped_bytes);
    sch_tree_unlock(sch);
    0
}

unsafe fn codel_init(sch: *mut Qdisc, opt: *mut nlattr, extack: *mut netlink_ext_ack) -> i32 {
    let q = qdisc_priv(sch) as *mut codel_sched_data;
    (*sch).limit = DEFAULT_CODEL_LIMIT;
    codel_params_init(&mut (*q).params);
    codel_vars_init(&mut (*q).vars);
    codel_stats_init(&mut (*q).stats);
    (*q).params.mtu = clamp_t(psched_mtu(qdisc_dev(sch)), 256, 1 << 20);
    if !opt.is_null() { let err = codel_change(sch, opt, extack); if err != 0 { return err; } }
    if (*sch).limit >= 1 { (*sch).flags |= TCQ_F_CAN_BYPASS; } else { (*sch).flags &= !TCQ_F_CAN_BYPASS; }
    (*sch).flags |= TCQ_F_DEQUEUE_DROPS;
    0
}

unsafe fn codel_dump(sch: *mut Qdisc, skb: *mut sk_buff) -> i32 {
    let q = qdisc_priv(sch) as *mut codel_sched_data;
    let ce_threshold = (*q).params.ce_threshold;
    let opts = nla_nest_start_noflag(skb, TCA_OPTIONS);
    if opts.is_null() { return -1; }
    if nla_put_u32(skb, TCA_CODEL_TARGET, codel_time_to_us((*q).params.target)) != 0 || nla_put_u32(skb, TCA_CODEL_LIMIT, (*sch).limit) != 0 || nla_put_u32(skb, TCA_CODEL_INTERVAL, codel_time_to_us((*q).params.interval)) != 0 || nla_put_u32(skb, TCA_CODEL_ECN, (*q).params.ecn) != 0 { nla_nest_cancel(skb, opts); return -1; }
    if ce_threshold != CODEL_DISABLED_THRESHOLD && nla_put_u32(skb, TCA_CODEL_CE_THRESHOLD, codel_time_to_us(ce_threshold)) != 0 { nla_nest_cancel(skb, opts); return -1; }
    nla_nest_end(skb, opts)
}

unsafe fn codel_dump_stats(sch: *mut Qdisc, d: *mut gnet_dump) -> i32 {
    let q = qdisc_priv(sch) as *const codel_sched_data;
    let mut st = tc_codel_xstats { maxpacket: (*q).stats.maxpacket, count: (*q).vars.count, lastcount: (*q).vars.lastcount, drop_overlimit: (*q).drop_overlimit, ldelay: codel_time_to_us((*q).vars.ldelay), dropping: (*q).vars.dropping, ecn_mark: (*q).stats.ecn_mark, ce_mark: (*q).stats.ce_mark, drop_next: 0 };
    if st.dropping { let delta = (*q).vars.drop_next - codel_get_time(); st.drop_next = if delta >= 0 { codel_time_to_us(delta) } else { -codel_time_to_us(-delta) }; }
    gnet_stats_copy_app(d, &st, core::mem::size_of::<tc_codel_xstats>())
}

unsafe fn codel_reset(sch: *mut Qdisc) { let q = qdisc_priv(sch) as *mut codel_sched_data; qdisc_reset_queue(sch); codel_vars_init(&mut (*q).vars); }

// The Qdisc operation table and module registration are supplied in the kernel-facing Rust bindings.
extern "C" {
    static mut codel_qdisc_ops: Qdisc_ops;
    fn register_qdisc(ops: *mut Qdisc_ops) -> i32;
    fn unregister_qdisc(ops: *mut Qdisc_ops);
}

unsafe fn codel_module_init() -> i32 { register_qdisc(&mut codel_qdisc_ops) }
unsafe fn codel_module_exit() { unregister_qdisc(&mut codel_qdisc_ops); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
