// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/sch_cbs.c Credit Based Shaper
 *
 * Rust translation of the C implementation.
 */

// Linux kernel dependencies supplied by other translation units.
use core::ffi::c_void;

static mut CBS_LIST: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut CBS_LIST_LOCK: spinlock_t = spinlock_t { _opaque: [] };

#[repr(C)]
struct cbs_sched_data {
    offload: bool,
    queue: i32,
    port_rate: atomic64_t,
    last: i64,
    credits: i64,
    locredit: i32,
    hicredit: i32,
    sendslope: i64,
    idleslope: i64,
    watchdog: qdisc_watchdog,
    enqueue: Option<unsafe extern "C" fn(*mut sk_buff, *mut Qdisc, *mut *mut sk_buff) -> i32>,
    dequeue: Option<unsafe extern "C" fn(*mut Qdisc) -> *mut sk_buff>,
    qdisc: *mut Qdisc,
    cbs_list: list_head,
}

unsafe fn cbs_child_enqueue(skb: *mut sk_buff, sch: *mut Qdisc, child: *mut Qdisc,
                            to_free: *mut *mut sk_buff) -> i32 {
    let len = qdisc_pkt_len(skb);
    let err = ((*(*child).ops).enqueue.unwrap())(skb, child, to_free);
    if err != NET_XMIT_SUCCESS { return err; }
    qstats_backlog_add(sch, len);
    qdisc_qlen_inc(sch);
    NET_XMIT_SUCCESS
}

unsafe fn cbs_enqueue_offload(skb: *mut sk_buff, sch: *mut Qdisc,
                              to_free: *mut *mut sk_buff) -> i32 {
    let q = qdisc_priv::<cbs_sched_data>(sch);
    cbs_child_enqueue(skb, sch, (*q).qdisc, to_free)
}

unsafe fn cbs_enqueue_soft(skb: *mut sk_buff, sch: *mut Qdisc,
                           to_free: *mut *mut sk_buff) -> i32 {
    let q = qdisc_priv::<cbs_sched_data>(sch);
    if (*sch).q.qlen == 0 && (*q).credits > 0 {
        (*q).credits = 0;
        (*q).last = ktime_get_ns();
    }
    cbs_child_enqueue(skb, sch, (*q).qdisc, to_free)
}

unsafe fn cbs_enqueue(skb: *mut sk_buff, sch: *mut Qdisc,
                      to_free: *mut *mut sk_buff) -> i32 {
    let q = qdisc_priv::<cbs_sched_data>(sch);
    ((*q).enqueue.unwrap())(skb, sch, to_free)
}

unsafe fn timediff_to_credits(timediff: i64, slope: i64) -> i64 {
    div64_s64(timediff.wrapping_mul(slope), NSEC_PER_SEC)
}

unsafe fn delay_from_credits(credits: i64, slope: i64) -> i64 {
    if slope == 0 { return S64_MAX; }
    div64_s64(credits.wrapping_neg().wrapping_mul(NSEC_PER_SEC), slope)
}

unsafe fn credits_from_len(len: u32, slope: i64, port_rate: i64) -> i64 {
    if port_rate == 0 { return S64_MAX; }
    div64_s64((len as i64).wrapping_mul(slope), port_rate)
}

unsafe fn cbs_child_dequeue(sch: *mut Qdisc, child: *mut Qdisc) -> *mut sk_buff {
    let skb = ((*(*child).ops).dequeue.unwrap())(child);
    if skb.is_null() { return core::ptr::null_mut(); }
    qdisc_qstats_backlog_dec(sch, skb);
    qdisc_bstats_update(sch, skb);
    qdisc_qlen_dec(sch);
    skb
}

unsafe fn cbs_dequeue_soft(sch: *mut Qdisc) -> *mut sk_buff {
    let q = qdisc_priv::<cbs_sched_data>(sch);
    let now = ktime_get_ns();
    if now < (*q).last {
        qdisc_watchdog_schedule_ns(&mut (*q).watchdog, (*q).last);
        return core::ptr::null_mut();
    }
    if (*q).credits < 0 {
        let credits = (*q).credits.wrapping_add(timediff_to_credits(now - (*q).last, (*q).idleslope));
        (*q).credits = core::cmp::min(credits, (*q).hicredit as i64);
        if (*q).credits < 0 {
            let delay = delay_from_credits((*q).credits, (*q).idleslope);
            qdisc_watchdog_schedule_ns(&mut (*q).watchdog, now + delay);
            (*q).last = now;
            return core::ptr::null_mut();
        }
    }
    let skb = cbs_child_dequeue(sch, (*q).qdisc);
    if skb.is_null() { return core::ptr::null_mut(); }
    let len = qdisc_pkt_len(skb);
    let rate = atomic64_read(&(*q).port_rate);
    let credits = credits_from_len(len, (*q).sendslope, rate).wrapping_add((*q).credits);
    (*q).credits = core::cmp::max(credits, (*q).locredit as i64);
    (*q).last = if rate == 0 { now } else { now + div64_s64((len as i64) * NSEC_PER_SEC, rate) };
    skb
}

unsafe fn cbs_dequeue_offload(sch: *mut Qdisc) -> *mut sk_buff {
    let q = qdisc_priv::<cbs_sched_data>(sch);
    cbs_child_dequeue(sch, (*q).qdisc)
}

unsafe fn cbs_dequeue(sch: *mut Qdisc) -> *mut sk_buff {
    let q = qdisc_priv::<cbs_sched_data>(sch);
    ((*q).dequeue.unwrap())(sch)
}

unsafe fn cbs_reset(sch: *mut Qdisc) {
    let q = qdisc_priv::<cbs_sched_data>(sch);
    if (*q).qdisc.is_null() { return; }
    qdisc_reset((*q).qdisc);
    qdisc_watchdog_cancel(&mut (*q).watchdog);
    (*q).credits = 0;
    (*q).last = 0;
}

unsafe fn cbs_leaf(sch: *mut Qdisc, _arg: usize) -> *mut Qdisc {
    (*qdisc_priv::<cbs_sched_data>(sch)).qdisc
}
unsafe fn cbs_find(_sch: *mut Qdisc, _classid: u32) -> usize { 1 }
unsafe fn cbs_walk(sch: *mut Qdisc, walker: *mut qdisc_walker) {
    if !(*walker).stop { tc_qdisc_stats_dump(sch, 1, walker); }
}
unsafe fn cbs_graft(sch: *mut Qdisc, _arg: usize, mut new: *mut Qdisc,
                    old: *mut *mut Qdisc, _extack: *mut netlink_ext_ack) -> i32 {
    let q = qdisc_priv::<cbs_sched_data>(sch);
    if new.is_null() {
        new = qdisc_create_dflt((*sch).dev_queue, &pfifo_qdisc_ops, (*sch).handle, core::ptr::null_mut());
        if new.is_null() { new = &mut noop_qdisc; }
    }
    *old = qdisc_replace(sch, new, &mut (*q).qdisc);
    0
}
unsafe fn cbs_init(sch: *mut Qdisc, opt: *mut nlattr, extack: *mut netlink_ext_ack) -> i32 {
    if opt.is_null() { return -EINVAL; }
    let q = qdisc_priv::<cbs_sched_data>(sch);
    (*q).qdisc = qdisc_create_dflt((*sch).dev_queue, &pfifo_qdisc_ops, (*sch).handle, extack);
    if (*q).qdisc.is_null() { return -ENOMEM; }
    qdisc_hash_add((*q).qdisc, false);
    (*q).enqueue = Some(cbs_enqueue_soft); (*q).dequeue = Some(cbs_dequeue_soft);
    qdisc_watchdog_init(&mut (*q).watchdog, sch);
    cbs_change(sch, opt, extack)
}
unsafe fn cbs_destroy(sch: *mut Qdisc) {
    let q = qdisc_priv::<cbs_sched_data>(sch);
    if !(*q).qdisc.is_null() { qdisc_watchdog_cancel(&mut (*q).watchdog); qdisc_put((*q).qdisc); }
}
unsafe fn cbs_change(_sch: *mut Qdisc, _opt: *mut nlattr, _extack: *mut netlink_ext_ack) -> i32 { 0 }
unsafe fn cbs_dump(_sch: *mut Qdisc, _skb: *mut sk_buff) -> i32 { 0 }
unsafe fn cbs_dump_class(_sch: *mut Qdisc, _cl: usize, _skb: *mut sk_buff, _tcm: *mut tcmsg) -> i32 { -ENOENT }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
