// SPDX-License-Identifier: GPL-2.0-only
/* Event cache for netfilter. */

/*
 * (C) 2005 Harald Welte <laforge@gnumonks.org>
 * (C) 2005 Patrick McHardy <kaber@trash.net>
 * (C) 2005-2006 Netfilter Core Team <coreteam@netfilter.org>
 * (C) 2005 USAGI/WIDE Project <http://www.linux-ipv6.org>
 */

// C kernel headers and build-time configuration are supplied by the surrounding crate.

static mut NF_CT_ECACHE_MUTEX: Mutex = DEFINE_MUTEX!();

const DYING_NULLS_VAL: usize = (1usize << 30) + 1;
const ECACHE_MAX_JIFFIES: u64 = msecs_to_jiffies(10);
const ECACHE_RETRY_JIFFIES: u64 = msecs_to_jiffies(10);

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum RetryState {
    StateCongested,
    StateRestart,
    StateDone,
}

pub unsafe fn nf_conn_pernet_ecache(net: *const net) -> *mut nf_conntrack_net_ecache {
    let cnet: *mut nf_conntrack_net = nf_ct_pernet(net);
    &mut (*cnet).ecache
}

unsafe fn ecache_work_evict_list(cnet: *mut nf_conntrack_net) -> RetryState {
    let stop = jiffies().wrapping_add(ECACHE_MAX_JIFFIES);
    let mut evicted_list: hlist_nulls_head = core::mem::zeroed();
    let mut ret = RetryState::StateDone;
    let mut sent: u32;

    INIT_HLIST_NULLS_HEAD(&mut evicted_list, DYING_NULLS_VAL);

    'next: loop {
        sent = 0;
        spin_lock_bh(&mut (*cnet).ecache.dying_lock);

        let mut h: *mut nf_conntrack_tuple_hash = core::ptr::null_mut();
        let mut n: *mut hlist_nulls_node = core::ptr::null_mut();
        hlist_nulls_for_each_entry_safe!(h, n, &mut (*cnet).ecache.dying_list, hnnode, {
            let ct: *mut nf_conn = nf_ct_tuplehash_to_ctrack(h);
            // The worker owns all entries; ct remains valid until nf_ct_put below.
            if nf_conntrack_event(IPCT_DESTROY, ct) != 0 {
                ret = RetryState::StateCongested;
                break;
            }
            hlist_nulls_del_rcu(&mut (*ct).tuplehash[IP_CT_DIR_ORIGINAL].hnnode);
            hlist_nulls_add_head(&mut (*ct).tuplehash[IP_CT_DIR_REPLY].hnnode, &mut evicted_list);
            if time_after(jiffies(), stop) {
                ret = RetryState::StateRestart;
                break;
            }
            sent = sent.wrapping_add(1);
            if sent > 16 {
                spin_unlock_bh(&mut (*cnet).ecache.dying_lock);
                cond_resched();
                continue 'next;
            }
        });
        spin_unlock_bh(&mut (*cnet).ecache.dying_lock);

        let mut h2: *mut nf_conntrack_tuple_hash = core::ptr::null_mut();
        let mut n2: *mut hlist_nulls_node = core::ptr::null_mut();
        hlist_nulls_for_each_entry_safe!(h2, n2, &mut evicted_list, hnnode, {
            let ct: *mut nf_conn = nf_ct_tuplehash_to_ctrack(h2);
            hlist_nulls_del_rcu(&mut (*ct).tuplehash[IP_CT_DIR_REPLY].hnnode);
            nf_ct_put(ct);
            cond_resched();
        });
        return ret;
    }
}

unsafe fn ecache_work(work: *mut work_struct) {
    let cnet = container_of!(work, nf_conntrack_net, ecache.dwork.work);
    let ret = ecache_work_evict_list(cnet);
    let delay: i32 = match ret {
        RetryState::StateCongested => ECACHE_RETRY_JIFFIES as i32,
        RetryState::StateRestart => 0,
        RetryState::StateDone => -1,
    };
    if delay >= 0 {
        schedule_delayed_work(&mut (*cnet).ecache.dwork, delay as u64);
    }
}

unsafe fn __nf_conntrack_eventmask_report(
    e: *mut nf_conntrack_ecache, events: u32, missed: u32, item: *const nf_ct_event,
) -> i32 {
    let net = nf_ct_net((*item).ct);
    if ((events | missed) & (*e).ctmask as u32) == 0 { return 0; }
    rcu_read_lock();
    let notify = rcu_dereference!((*net).ct.nf_conntrack_event_cb);
    if notify.is_null() { rcu_read_unlock(); return 0; }
    let ret = ((*notify).ct_event)(events | missed, item);
    rcu_read_unlock();
    if ret >= 0 && missed == 0 { return 0; }
    loop {
        let old = READ_ONCE((*e).missed);
        let want = if ret < 0 { old | events } else { old & !missed };
        if cmpxchg(&mut (*e).missed, old, want) == old { break; }
    }
    ret
}

unsafe fn nf_ct_ecache_tstamp_refresh(e: *mut nf_conntrack_ecache) {
    #[cfg(CONFIG_NF_CONNTRACK_TIMESTAMP)]
    if local64_read(&(*e).timestamp) != 0 { local64_set(&mut (*e).timestamp, ktime_get_real_ns()); }
}

pub unsafe fn nf_conntrack_eventmask_report(events: u32, ct: *mut nf_conn, portid: u32, report: i32) -> i32 {
    if !nf_ct_is_confirmed(ct) { return 0; }
    let e = nf_ct_ecache_find(ct);
    if e.is_null() { return 0; }
    let mut item: nf_ct_event = core::mem::zeroed();
    (*item).ct = ct;
    item.portid = if (*e).portid != 0 { (*e).portid } else { portid };
    item.report = report;
    let missed = if (*e).portid != 0 { 0 } else { (*e).missed };
    nf_ct_ecache_tstamp_refresh(e);
    let ret = __nf_conntrack_eventmask_report(e, events, missed, &item);
    if ret < 0 && (events & (1 << IPCT_DESTROY)) != 0 && (*e).portid == 0 && portid != 0 { (*e).portid = portid; }
    ret
}

pub unsafe fn nf_ct_deliver_cached_events(ct: *mut nf_conn) {
    if !nf_ct_is_confirmed(ct) || nf_ct_is_dying(ct) { return; }
    let e = nf_ct_ecache_find(ct);
    if e.is_null() { return; }
    let events = xchg(&mut (*e).cache, 0);
    let item = nf_ct_event { ct, portid: 0, report: 0 };
    __nf_conntrack_eventmask_report(e, events, (*e).missed, &item);
}

pub unsafe fn nf_ct_expect_event_report(event: ip_conntrack_expect_events, exp: *mut nf_conntrack_expect, portid: u32, report: i32) {
    let net = nf_ct_exp_net(exp);
    lockdep_nfct_expect_lock_held();
    rcu_read_lock();
    let notify = rcu_dereference!((*net).ct.nf_conntrack_event_cb);
    if !notify.is_null() && ((*exp).event_mask & (1 << event as u32)) != 0 {
        let item = nf_exp_event { exp, portid, report };
        ((*notify).exp_event)(1 << event as u32, &item);
    }
    rcu_read_unlock();
}

pub unsafe fn nf_conntrack_register_notifier(net: *mut net, new: *const nf_ct_event_notifier) {
    mutex_lock(&mut NF_CT_ECACHE_MUTEX);
    let notify = rcu_dereference_protected!((*net).ct.nf_conntrack_event_cb, lockdep_is_held(&NF_CT_ECACHE_MUTEX));
    WARN_ON_ONCE!(!notify.is_null());
    rcu_assign_pointer!((*net).ct.nf_conntrack_event_cb, new);
    mutex_unlock(&mut NF_CT_ECACHE_MUTEX);
}

pub unsafe fn nf_conntrack_unregister_notifier(net: *mut net) {
    mutex_lock(&mut NF_CT_ECACHE_MUTEX);
    RCU_INIT_POINTER!((*net).ct.nf_conntrack_event_cb, core::ptr::null());
    mutex_unlock(&mut NF_CT_ECACHE_MUTEX);
    // synchronize_rcu() is called after netns pre_exit.
}

pub unsafe fn nf_conntrack_ecache_work(net: *mut net, state: nf_ct_ecache_state) {
    let cnet = nf_ct_pernet(net);
    if state == NFCT_ECACHE_DESTROY_FAIL && !delayed_work_pending(&(*cnet).ecache.dwork) {
        schedule_delayed_work(&mut (*cnet).ecache.dwork, HZ);
        (*net).ct.ecache_dwork_pending = true;
    } else if state == NFCT_ECACHE_DESTROY_SENT {
        if !hlist_nulls_empty(&(*cnet).ecache.dying_list) { mod_delayed_work(system_percpu_wq, &mut (*cnet).ecache.dwork, 0); }
        else { (*net).ct.ecache_dwork_pending = false; }
    }
}

unsafe fn nf_ct_ecache_tstamp_new(ct: *const nf_conn, e: *mut nf_conntrack_ecache) {
    #[cfg(CONFIG_NF_CONNTRACK_TIMESTAMP)]
    { let mut ts = 0; if nf_ct_ext_exist(ct, NF_CT_EXT_TSTAMP) { ts = ktime_get_real_ns(); } local64_set(&mut (*e).timestamp, ts); }
}

pub unsafe fn nf_ct_ecache_ext_add(ct: *mut nf_conn, mut ctmask: u16, mut expmask: u16, gfp: gfp_t) -> bool {
    let net = nf_ct_net(ct);
    match (*net).ct.sysctl_events {
        0 => { if ctmask != 0 || expmask != 0 {} else { return true; } }
        2 => { if !READ_ONCE(nf_ctnetlink_has_listener) { return true; } }
        1 => { if ctmask == 0 && expmask == 0 { ctmask = u16::MAX; expmask = u16::MAX; } }
        _ => { WARN_ON_ONCE!(true); return true; }
    }
    let e = nf_ct_ext_add(ct, NF_CT_EXT_ECACHE, gfp);
    if !e.is_null() { nf_ct_ecache_tstamp_new(ct, e); (*e).ctmask = ctmask; (*e).expmask = expmask; }
    !e.is_null()
}

const NF_CT_EVENTS_DEFAULT: i32 = 2;
static mut NF_CT_EVENTS: i32 = NF_CT_EVENTS_DEFAULT;

pub unsafe fn nf_conntrack_ecache_pernet_init(net: *mut net) {
    let cnet = nf_ct_pernet(net);
    (*net).ct.sysctl_events = NF_CT_EVENTS;
    INIT_DELAYED_WORK!(&mut (*cnet).ecache.dwork, ecache_work);
    INIT_HLIST_NULLS_HEAD(&mut (*cnet).ecache.dying_list, DYING_NULLS_VAL);
    spin_lock_init(&mut (*cnet).ecache.dying_lock);
    BUILD_BUG_ON!(__IPCT_MAX >= 16);
}

pub unsafe fn nf_conntrack_ecache_pernet_fini(net: *mut net) {
    let cnet = nf_ct_pernet(net);
    cancel_delayed_work_sync(&mut (*cnet).ecache.dwork);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
