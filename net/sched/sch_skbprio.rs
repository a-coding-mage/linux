// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/sch_skbprio.c  SKB Priority Queue.
 *
 * Authors: Nishanth Devarajan, <ndev2021@gmail.com>
 *          Cody Doucette, <doucette@bu.edu>
 *          original idea by Michel Machado, Cody Doucette, and Qiaobin Fu
 */

// Linux kernel dependencies supplied by other translation units.

struct skbprio_sched_data {
    /* Queue state. */
    qdiscs: [sk_buff_head; SKBPRIO_MAX_PRIORITY],
    qstats: [gnet_stats_queue; SKBPRIO_MAX_PRIORITY],
    highest_prio: u16,
    lowest_prio: u16,
}

unsafe fn calc_new_high_prio(q: *const skbprio_sched_data) -> u16 {
    let mut prio: i32 = (*q).highest_prio as i32 - 1;
    while prio >= (*q).lowest_prio as i32 {
        if !skb_queue_empty(&(*q).qdiscs[prio as usize]) {
            return prio as u16;
        }
        prio -= 1;
    }
    /* SKB queue is empty, return 0 (default highest priority setting). */
    0
}

unsafe fn calc_new_low_prio(q: *const skbprio_sched_data) -> u16 {
    let mut prio: i32 = (*q).lowest_prio as i32 + 1;
    while prio <= (*q).highest_prio as i32 {
        if !skb_queue_empty(&(*q).qdiscs[prio as usize]) {
            return prio as u16;
        }
        prio += 1;
    }
    /* SKB queue is empty, return SKBPRIO_MAX_PRIORITY - 1
     * (default lowest priority setting).
     */
    SKBPRIO_MAX_PRIORITY - 1
}

unsafe fn skbprio_enqueue(skb: *mut sk_buff, sch: *mut Qdisc,
                          to_free: *mut *mut sk_buff) -> i32 {
    let max_priority: u32 = SKBPRIO_MAX_PRIORITY - 1;
    let q = qdisc_priv::<skbprio_sched_data>(sch);
    let prio: u16 = core::cmp::min((*skb).priority, max_priority) as u16;
    let qdisc = &mut (*q).qdiscs[prio as usize];

    if (*sch).q.qlen < READ_ONCE((*sch).limit) {
        __skb_queue_tail(qdisc, skb);
        qdisc_qstats_backlog_inc(sch, skb);
        (*q).qstats[prio as usize].backlog += qdisc_pkt_len(skb);
        if prio > (*q).highest_prio { (*q).highest_prio = prio; }
        if prio < (*q).lowest_prio { (*q).lowest_prio = prio; }
        qdisc_qlen_inc(sch);
        return NET_XMIT_SUCCESS;
    }

    let lp = (*q).lowest_prio;
    if prio <= lp {
        (*q).qstats[prio as usize].drops += 1;
        (*q).qstats[prio as usize].overlimits += 1;
        return qdisc_drop(skb, sch, to_free);
    }

    __skb_queue_tail(qdisc, skb);
    qdisc_qstats_backlog_inc(sch, skb);
    (*q).qstats[prio as usize].backlog += qdisc_pkt_len(skb);
    let lp_qdisc = &mut (*q).qdiscs[lp as usize];
    let to_drop = __skb_dequeue_tail(lp_qdisc);
    BUG_ON(to_drop.is_null());
    qdisc_qstats_backlog_dec(sch, to_drop);
    qdisc_drop(to_drop, sch, to_free);
    (*q).qstats[lp as usize].backlog -= qdisc_pkt_len(to_drop);
    (*q).qstats[lp as usize].drops += 1;
    (*q).qstats[lp as usize].overlimits += 1;

    if skb_queue_empty(lp_qdisc) {
        if (*q).lowest_prio == (*q).highest_prio {
            (*q).lowest_prio = prio;
            (*q).highest_prio = prio;
        } else {
            (*q).lowest_prio = calc_new_low_prio(q);
        }
    }
    if prio > (*q).highest_prio { (*q).highest_prio = prio; }
    NET_XMIT_CN
}

unsafe fn skbprio_dequeue(sch: *mut Qdisc) -> *mut sk_buff {
    let q = qdisc_priv::<skbprio_sched_data>(sch);
    let hpq = &mut (*q).qdiscs[(*q).highest_prio as usize];
    let skb = __skb_dequeue(hpq);
    if unlikely(skb.is_null()) { return core::ptr::null_mut(); }
    qdisc_qlen_dec(sch);
    qdisc_qstats_backlog_dec(sch, skb);
    qdisc_bstats_update(sch, skb);
    (*q).qstats[(*q).highest_prio as usize].backlog -= qdisc_pkt_len(skb);
    if skb_queue_empty(hpq) {
        if (*q).lowest_prio == (*q).highest_prio {
            (*q).highest_prio = 0;
            (*q).lowest_prio = SKBPRIO_MAX_PRIORITY - 1;
        } else {
            (*q).highest_prio = calc_new_high_prio(q);
        }
    }
    skb
}

unsafe fn skbprio_change(sch: *mut Qdisc, opt: *mut nlattr,
                         _extack: *mut netlink_ext_ack) -> i32 {
    let ctl = nla_data::<tc_skbprio_qopt>(opt);
    if (*opt).nla_len != nla_attr_size(core::mem::size_of::<tc_skbprio_qopt>()) { return -EINVAL; }
    WRITE_ONCE((*sch).limit, (*ctl).limit);
    0
}

unsafe fn skbprio_init(sch: *mut Qdisc, opt: *mut nlattr,
                       extack: *mut netlink_ext_ack) -> i32 {
    let q = qdisc_priv::<skbprio_sched_data>(sch);
    for prio in 0..SKBPRIO_MAX_PRIORITY { __skb_queue_head_init(&mut (*q).qdiscs[prio as usize]); }
    memset((*q).qstats.as_mut_ptr(), 0, core::mem::size_of_val(&(*q).qstats));
    (*q).highest_prio = 0;
    (*q).lowest_prio = SKBPRIO_MAX_PRIORITY - 1;
    (*sch).limit = 64;
    if opt.is_null() { return 0; }
    skbprio_change(sch, opt, extack)
}

unsafe fn skbprio_dump(sch: *mut Qdisc, skb: *mut sk_buff) -> i32 {
    let mut opt: tc_skbprio_qopt = core::mem::zeroed();
    opt.limit = READ_ONCE((*sch).limit);
    if nla_put(skb, TCA_OPTIONS, core::mem::size_of::<tc_skbprio_qopt>(), &opt as *const _ as *const core::ffi::c_void) { return -1; }
    (*skb).len as i32
}

unsafe fn skbprio_reset(sch: *mut Qdisc) {
    let q = qdisc_priv::<skbprio_sched_data>(sch);
    for prio in 0..SKBPRIO_MAX_PRIORITY { __skb_queue_purge(&mut (*q).qdiscs[prio as usize]); }
    memset((*q).qstats.as_mut_ptr(), 0, core::mem::size_of_val(&(*q).qstats));
    (*q).highest_prio = 0;
    (*q).lowest_prio = SKBPRIO_MAX_PRIORITY - 1;
}

unsafe fn skbprio_destroy(sch: *mut Qdisc) {
    let q = qdisc_priv::<skbprio_sched_data>(sch);
    for prio in 0..SKBPRIO_MAX_PRIORITY { __skb_queue_purge(&mut (*q).qdiscs[prio as usize]); }
}

unsafe fn skbprio_leaf(_sch: *mut Qdisc, _arg: usize) -> *mut Qdisc { core::ptr::null_mut() }
unsafe fn skbprio_find(_sch: *mut Qdisc, _classid: u32) -> usize { 0 }

unsafe fn skbprio_dump_class(_sch: *mut Qdisc, cl: usize, _skb: *mut sk_buff, tcm: *mut tcmsg) -> i32 {
    (*tcm).tcm_handle |= TC_H_MIN(cl as u32); 0
}

unsafe fn skbprio_dump_class_stats(sch: *mut Qdisc, cl: usize, d: *mut gnet_dump) -> i32 {
    let q = qdisc_priv::<skbprio_sched_data>(sch);
    if gnet_stats_copy_queue(d, core::ptr::null_mut(), &(*q).qstats[cl - 1], (*q).qstats[cl - 1].qlen) < 0 { return -1; }
    0
}

unsafe fn skbprio_walk(sch: *mut Qdisc, arg: *mut qdisc_walker) {
    if (*arg).stop { return; }
    for i in 0..SKBPRIO_MAX_PRIORITY {
        if !tc_qdisc_stats_dump(sch, i + 1, arg) { break; }
    }
}

static skbprio_class_ops: Qdisc_class_ops = Qdisc_class_ops {
    leaf: Some(skbprio_leaf), find: Some(skbprio_find), dump: Some(skbprio_dump_class),
    dump_stats: Some(skbprio_dump_class_stats), walk: Some(skbprio_walk),
};

static mut skbprio_qdisc_ops: Qdisc_ops = Qdisc_ops {
    cl_ops: &skbprio_class_ops, id: "skbprio", priv_size: core::mem::size_of::<skbprio_sched_data>(),
    enqueue: Some(skbprio_enqueue), dequeue: Some(skbprio_dequeue), peek: Some(qdisc_peek_dequeued),
    init: Some(skbprio_init), reset: Some(skbprio_reset), change: Some(skbprio_change),
    dump: Some(skbprio_dump), destroy: Some(skbprio_destroy), owner: THIS_MODULE,
};

unsafe fn skbprio_module_init() -> i32 { register_qdisc(&mut skbprio_qdisc_ops) }
unsafe fn skbprio_module_exit() { unregister_qdisc(&mut skbprio_qdisc_ops); }

// MODULE_ALIAS_NET_SCH("skbprio");
// module_init(skbprio_module_init)
// module_exit(skbprio_module_exit)
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("SKB priority based scheduling qdisc");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
