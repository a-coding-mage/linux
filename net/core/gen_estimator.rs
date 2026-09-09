// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/gen_estimator.c\tSimple rate estimator.
 *
 * Authors:\tAlexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
 *\t\tEric Dumazet <edumazet@google.com>
 *
 * Changes:
 *              Jamal Hadi Salim - moved it to net/core and reshulfed
 *              names to make it usable in general net subsystem.
 */

// Linux kernel dependencies supplied by other translation units.

/* This code is NOT intended to be used for statistics collection,
 * its purpose is to provide a base for statistical multiplexing
 * for controlled load service.
 * If you need only statistics, run a user level daemon which
 * periodically reads byte counters.
 */

#[repr(C)]
pub struct net_rate_estimator {
    pub bstats: *mut gnet_stats_basic_sync,
    pub stats_lock: *mut spinlock_t,
    pub running: bool,
    pub cpu_bstats: *mut gnet_stats_basic_sync,
    pub ewma_log: u8,
    pub intvl_log: u8, /* period : (250ms << intvl_log) */

    pub seq: seqcount_t,
    pub last_packets: u64,
    pub last_bytes: u64,

    pub avpps: u64,
    pub avbps: u64,

    pub next_jiffies: c_ulong,
    pub timer: timer_list,
    pub rcu: rcu_head,
}

unsafe fn est_fetch_counters(e: *mut net_rate_estimator,
                             b: *mut gnet_stats_basic_sync) {
    gnet_stats_basic_sync_init(b);
    if !(*e).stats_lock.is_null() {
        spin_lock((*e).stats_lock);
    }

    gnet_stats_add_basic(b, (*e).cpu_bstats, (*e).bstats, (*e).running);

    if !(*e).stats_lock.is_null() {
        spin_unlock((*e).stats_lock);
    }
}

unsafe extern "C" fn est_timer(t: *mut timer_list) {
    let est = timer_container_of::<net_rate_estimator>(t, "timer");
    let mut b: gnet_stats_basic_sync = core::mem::zeroed();
    let b_bytes: u64;
    let b_packets: u64;
    let mut rate: u64;
    let mut brate: u64;

    est_fetch_counters(est, &mut b);
    b_bytes = u64_stats_read(&b.bytes);
    b_packets = u64_stats_read(&b.packets);

    brate = (b_bytes.wrapping_sub((*est).last_bytes)) << (10 - (*est).intvl_log);
    brate = (brate >> (*est).ewma_log).wrapping_sub((*est).avbps >> (*est).ewma_log);

    rate = (b_packets.wrapping_sub((*est).last_packets)) << (10 - (*est).intvl_log);
    rate = (rate >> (*est).ewma_log).wrapping_sub((*est).avpps >> (*est).ewma_log);

    preempt_disable_nested();
    write_seqcount_begin(&mut (*est).seq);
    (*est).avbps = (*est).avbps.wrapping_add(brate);
    (*est).avpps = (*est).avpps.wrapping_add(rate);
    write_seqcount_end(&mut (*est).seq);
    preempt_enable_nested();

    (*est).last_bytes = b_bytes;
    (*est).last_packets = b_packets;

    (*est).next_jiffies = (*est).next_jiffies.wrapping_add(((HZ / 4) << (*est).intvl_log) as c_ulong);

    if time_after_eq(jiffies, (*est).next_jiffies) {
        /* Ouch... timer was delayed. */
        (*est).next_jiffies = jiffies.wrapping_add(1);
    }
    mod_timer(&mut (*est).timer, (*est).next_jiffies);
}

/**
 * gen_new_estimator - create a new rate estimator
 * @bstats: basic statistics
 * @cpu_bstats: bstats per cpu
 * @rate_est: rate estimator statistics
 * @lock: lock for statistics and control path
 * @running: true if @bstats represents a running qdisc, thus @bstats'
 *           internal values might change during basic reads. Only used
 *           if @bstats_cpu is NULL
 * @opt: rate estimator configuration TLV
 *
 * Creates a new rate estimator with &bstats as source and &rate_est
 * as destination. A new timer with the interval specified in the
 * configuration TLV is created. Upon each interval, the latest statistics
 * will be read from &bstats and the estimated rate will be stored in
 * &rate_est with the statistics lock grabbed during this period.
 *
 * Returns 0 on success or a negative error code.
 *
 */
pub unsafe fn gen_new_estimator(
    bstats: *mut gnet_stats_basic_sync,
    cpu_bstats: *mut gnet_stats_basic_sync,
    rate_est: *mut *mut net_rate_estimator,
    lock: *mut spinlock_t,
    running: bool,
    opt: *mut nlattr,
) -> c_int {
    let parm = nla_data::<gnet_estimator>(opt);
    let mut old: *mut net_rate_estimator;
    let est: *mut net_rate_estimator;
    let mut b: gnet_stats_basic_sync = core::mem::zeroed();
    let intvl_log: c_int;

    if nla_len(opt) < core::mem::size_of::<gnet_estimator>() {
        return -EINVAL;
    }
    if (*parm).interval < -2 || (*parm).interval > 3 {
        return -EINVAL;
    }
    if (*parm).ewma_log == 0 || (*parm).ewma_log >= 31 {
        return -EINVAL;
    }

    est = kzalloc_obj::<net_rate_estimator>();
    if est.is_null() {
        return -ENOBUFS;
    }

    seqcount_init(&mut (*est).seq);
    intvl_log = (*parm).interval + 2;
    (*est).bstats = bstats;
    (*est).stats_lock = lock;
    (*est).running = running;
    (*est).ewma_log = (*parm).ewma_log;
    (*est).intvl_log = intvl_log as u8;
    (*est).cpu_bstats = cpu_bstats;

    if !lock.is_null() { local_bh_disable(); }
    est_fetch_counters(est, &mut b);
    if !lock.is_null() { local_bh_enable(); }
    (*est).last_bytes = u64_stats_read(&b.bytes);
    (*est).last_packets = u64_stats_read(&b.packets);

    if !lock.is_null() { spin_lock_bh(lock); }
    old = rcu_dereference_protected(*rate_est, 1);
    if !old.is_null() {
        timer_delete_sync(&mut (*old).timer);
        (*est).avbps = (*old).avbps;
        (*est).avpps = (*old).avpps;
    }

    (*est).next_jiffies = jiffies + ((HZ / 4) << intvl_log) as c_ulong;
    timer_setup(&mut (*est).timer, est_timer, 0);
    mod_timer(&mut (*est).timer, (*est).next_jiffies);

    rcu_assign_pointer(rate_est, est);
    if !lock.is_null() { spin_unlock_bh(lock); }
    if !old.is_null() { kfree_rcu(old, rcu); }
    0
}

/** Remove a rate estimator. */
pub unsafe fn gen_kill_estimator(rate_est: *mut *mut net_rate_estimator) {
    let est = unrcu_pointer(xchg(rate_est, core::ptr::null_mut()));
    if !est.is_null() {
        timer_shutdown_sync(&mut (*est).timer);
        kfree_rcu(est, rcu);
    }
}

pub unsafe fn gen_replace_estimator(
    bstats: *mut gnet_stats_basic_sync,
    cpu_bstats: *mut gnet_stats_basic_sync,
    rate_est: *mut *mut net_rate_estimator,
    lock: *mut spinlock_t,
    running: bool,
    opt: *mut nlattr,
) -> c_int {
    gen_new_estimator(bstats, cpu_bstats, rate_est, lock, running, opt)
}

pub unsafe fn gen_estimator_active(rate_est: *mut *mut net_rate_estimator) -> bool {
    !rcu_access_pointer(*rate_est).is_null()
}

pub unsafe fn gen_estimator_read(
    rate_est: *mut *mut net_rate_estimator,
    sample: *mut gnet_stats_rate_est64,
) -> bool {
    let est: *mut net_rate_estimator;
    let seq: c_uint;

    rcu_read_lock();
    est = rcu_dereference(*rate_est);
    if est.is_null() {
        rcu_read_unlock();
        return false;
    }

    loop {
        seq = read_seqcount_begin(&(*est).seq);
        (*sample).bps = (*est).avbps >> 8;
        (*sample).pps = (*est).avpps >> 8;
        if !read_seqcount_retry(&(*est).seq, seq) { break; }
    }

    rcu_read_unlock();
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
