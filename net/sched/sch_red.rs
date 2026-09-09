// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/sch_red.c	Random Early Detection queue.
 *
 * Authors:	Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
 */

// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct red_sched_data {
    pub limit: u32,
    pub flags: u8,
    pub userbits: u8,
    pub adapt_timer: timer_list,
    pub sch: *mut Qdisc,
    pub parms: red_parms,
    pub vars: red_vars,
    pub stats: red_stats,
    pub qdisc: *mut Qdisc,
    pub qe_early_drop: tcf_qevent,
    pub qe_mark: tcf_qevent,
}

pub const TC_RED_SUPPORTED_FLAGS: u32 = TC_RED_HISTORIC_FLAGS | TC_RED_NODROP;

#[inline]
unsafe fn red_use_ecn(q: *mut red_sched_data) -> i32 { ((*q).flags as u32 & TC_RED_ECN) as i32 }

#[inline]
unsafe fn red_use_harddrop(q: *mut red_sched_data) -> i32 { ((*q).flags as u32 & TC_RED_HARDDROP) as i32 }

unsafe fn red_use_nodrop(q: *mut red_sched_data) -> i32 { ((*q).flags as u32 & TC_RED_NODROP) as i32 }

unsafe fn red_enqueue(mut skb: *mut sk_buff, sch: *mut Qdisc, to_free: *mut *mut sk_buff) -> i32 {
    let mut reason: qdisc_drop_reason = QDISC_DROP_CONGESTED;
    let q = qdisc_priv::<red_sched_data>(sch);
    let child = (*q).qdisc;
    let len: u32;
    let mut ret: i32;
    (*q).vars.qavg = red_calc_qavg(&mut (*q).parms, &mut (*q).vars, (*(*child).qstats).backlog);
    if red_is_idling(&(*q).vars) { red_end_of_idle_period(&mut (*q).vars); }
    match red_action(&(*q).parms, &(*q).vars, (*q).vars.qavg) {
        RED_DONT_MARK => {},
        RED_PROB_MARK => {
            qdisc_qstats_overlimit(sch);
            if red_use_ecn(q) == 0 { (*q).stats.prob_drop += 1; return congestion_drop(skb, sch, to_free, &mut ret, reason, &mut skb); }
            if INET_ECN_set_ce(skb) != 0 {
                (*q).stats.prob_mark += 1;
                skb = tcf_qevent_handle(&mut (*q).qe_mark, sch, skb, to_free, &mut ret);
                if skb.is_null() { return NET_XMIT_CN | ret; }
            } else if red_use_nodrop(q) == 0 {
                (*q).stats.prob_drop += 1;
                return congestion_drop(skb, sch, to_free, &mut ret, reason, &mut skb);
            }
        },
        RED_HARD_MARK => {
            reason = QDISC_DROP_OVERLIMIT;
            qdisc_qstats_overlimit(sch);
            if red_use_harddrop(q) != 0 || red_use_ecn(q) == 0 {
                (*q).stats.forced_drop += 1;
                return congestion_drop(skb, sch, to_free, &mut ret, reason, &mut skb);
            }
            if INET_ECN_set_ce(skb) != 0 {
                (*q).stats.forced_mark += 1;
                skb = tcf_qevent_handle(&mut (*q).qe_mark, sch, skb, to_free, &mut ret);
                if skb.is_null() { return NET_XMIT_CN | ret; }
            } else if red_use_nodrop(q) == 0 {
                (*q).stats.forced_drop += 1;
                return congestion_drop(skb, sch, to_free, &mut ret, reason, &mut skb);
            }
        },
        _ => {}
    }
    len = qdisc_pkt_len(skb);
    ret = qdisc_enqueue(skb, child, to_free);
    if ret == NET_XMIT_SUCCESS { qstats_backlog_add(sch, len); qdisc_qlen_inc(sch); }
    else if net_xmit_drop_count(ret) != 0 { (*q).stats.pdrop += 1; qdisc_qstats_drop(sch); }
    ret
}

unsafe fn congestion_drop(skb: *mut sk_buff, sch: *mut Qdisc, to_free: *mut *mut sk_buff, ret: &mut i32, reason: qdisc_drop_reason, out: &mut *mut sk_buff) -> i32 {
    let q = qdisc_priv::<red_sched_data>(sch);
    *out = tcf_qevent_handle(&mut (*q).qe_early_drop, sch, skb, to_free, ret);
    if (*out).is_null() { return NET_XMIT_CN | *ret; }
    qdisc_drop_reason(*out, sch, to_free, reason);
    NET_XMIT_CN
}

unsafe fn red_dequeue(sch: *mut Qdisc) -> *mut sk_buff {
    let q = qdisc_priv::<red_sched_data>(sch);
    let skb = qdisc_dequeue_peeked((*q).qdisc);
    if !skb.is_null() { qdisc_bstats_update(sch, skb); qdisc_qstats_backlog_dec(sch, skb); qdisc_qlen_dec(sch); }
    else if !red_is_idling(&(*q).vars) { red_start_of_idle_period(&mut (*q).vars); }
    skb
}

unsafe fn red_peek(sch: *mut Qdisc) -> *mut sk_buff { let q = qdisc_priv::<red_sched_data>(sch); ((*(*q).qdisc).ops).peek.unwrap()((*q).qdisc) }
unsafe fn red_reset(sch: *mut Qdisc) { let q = qdisc_priv::<red_sched_data>(sch); qdisc_reset((*q).qdisc); red_restart(&mut (*q).vars); }

// The remaining qdisc lifecycle and netlink callbacks retain their C ABI-facing signatures.
extern "C" {
    fn red_offload(sch: *mut Qdisc, enable: bool) -> i32;
    fn red_init(sch: *mut Qdisc, opt: *mut nlattr, extack: *mut netlink_ext_ack) -> i32;
    fn red_change(sch: *mut Qdisc, opt: *mut nlattr, extack: *mut netlink_ext_ack) -> i32;
    fn red_destroy(sch: *mut Qdisc);
    fn red_dump(sch: *mut Qdisc, skb: *mut sk_buff) -> i32;
    fn red_dump_stats(sch: *mut Qdisc, d: *mut gnet_dump) -> i32;
    fn red_graft(sch: *mut Qdisc, arg: usize, new: *mut Qdisc, old: *mut *mut Qdisc, extack: *mut netlink_ext_ack) -> i32;
    fn red_leaf(sch: *mut Qdisc, arg: usize) -> *mut Qdisc;
    fn red_find(sch: *mut Qdisc, classid: u32) -> usize;
    fn red_walk(sch: *mut Qdisc, walker: *mut qdisc_walker);
}

#[no_mangle]
pub unsafe extern "C" fn red_module_init() -> i32 { register_qdisc(&mut red_qdisc_ops) }
#[no_mangle]
pub unsafe extern "C" fn red_module_exit() { unregister_qdisc(&mut red_qdisc_ops); }

// External declarations and ABI types are provided by the kernel translation environment.
extern "C" {
    static mut red_qdisc_ops: Qdisc_ops;
    fn register_qdisc(ops: *mut Qdisc_ops) -> i32;
    fn unregister_qdisc(ops: *mut Qdisc_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
