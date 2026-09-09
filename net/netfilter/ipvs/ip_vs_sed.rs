// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IPVS:        Shortest Expected Delay scheduling module
 *
 * Authors:     Wensong Zhang <wensong@linuxvirtualserver.org>
 *
 * Changes:
 */

/*
 * The SED algorithm attempts to minimize each job's expected delay until
 * completion. The expected delay that the job will experience is
 * (Ci + 1) / Ui if sent to the ith server, in which Ci is the number of
 * jobs on the ith server and Ui is the fixed service rate (weight) of the
 * ith server. The SED algorithm adopts a greedy policy that each does
 * what is in its own best interest, i.e. to join the queue which would
 * minimize its expected delay of completion.
 *
 * See the following paper for more information:
 * A. Weinrib and S. Shenker, Greed is not enough: Adaptive load sharing
 * in large heterogeneous systems. In Proceedings IEEE INFOCOM'88,
 * pages 986-994, 1988.
 *
 * Thanks must go to Marko Buuri <marko@buuri.name> for talking SED to me.
 *
 * The difference between SED and WLC is that SED includes the incoming
 * job in the cost function (the increment of 1). SED may outperform
 * WLC, while scheduling big jobs under larger heterogeneous systems
 * (the server weight varies a lot).
 */

// C headers provide the kernel and IPVS types, constants, and functions used below.

#[inline]
unsafe fn ip_vs_sed_dest_overhead(dest: *mut ip_vs_dest) -> i32 {
    /*
     * We only use the active connection number in the cost
     * calculation here.
     */
    atomic_read(unsafe { &(*dest).activeconns }) + 1
}

/*
 * Weighted Least Connection scheduling
 */
unsafe fn ip_vs_sed_schedule(
    svc: *mut ip_vs_service,
    _skb: *const sk_buff,
    _iph: *mut ip_vs_iphdr,
) -> *mut ip_vs_dest {
    let mut dest: *mut ip_vs_dest;
    let mut least: *mut ip_vs_dest;
    let mut loh: i32;
    let mut doh: i32;

    ip_vs_dbg!(6, "{}(): Scheduling...\n", "ip_vs_sed_schedule");

    /*
     * We calculate the load of each dest server as follows:
     *     (server expected overhead) / dest->weight
     *
     * Remember -- no floats in kernel mode!!!
     * The comparison of h1*w2 > h2*w1 is equivalent to that of
     *         h1/w1 > h2/w2
     * if every weight is larger than zero.
     *
     * The server with weight=0 is quiesced and will not receive any
     * new connections.
     */

    list_for_each_entry_rcu!(dest, &(*svc).destinations, n_list, {
        if ((*dest).flags & IP_VS_DEST_F_OVERLOAD) == 0
            && atomic_read(&(*dest).weight) > 0
        {
            least = dest;
            loh = ip_vs_sed_dest_overhead(least);
            goto nextstage;
        }
    });
    ip_vs_scheduler_err(svc, "no destination available");
    return core::ptr::null_mut();

    /*
     *    Find the destination with the least load.
     */
nextstage:
    list_for_each_entry_continue_rcu!(dest, &(*svc).destinations, n_list, {
        if ((*dest).flags & IP_VS_DEST_F_OVERLOAD) != 0 {
            continue;
        }
        doh = ip_vs_sed_dest_overhead(dest);
        if (loh as i64 * atomic_read(&(*dest).weight) as i64
            > doh as i64 * atomic_read(&(*least).weight) as i64)
        {
            least = dest;
            loh = doh;
        }
    });

    ip_vs_dbg_buf!(
        6,
        "SED: server %s:%u activeconns %d refcnt %d weight %d overhead %d\n",
        ip_vs_dbg_addr((*least).af, &(*least).addr),
        ntohs((*least).port),
        atomic_read(&(*least).activeconns),
        refcount_read(&(*least).refcnt),
        atomic_read(&(*least).weight),
        loh
    );

    least
}

static mut ip_vs_sed_scheduler: ip_vs_scheduler = ip_vs_scheduler {
    name: "sed",
    refcnt: ATOMIC_INIT!(0),
    module: THIS_MODULE,
    n_list: LIST_HEAD_INIT!(ip_vs_sed_scheduler.n_list),
    schedule: Some(ip_vs_sed_schedule),
};

unsafe fn ip_vs_sed_init() -> i32 {
    register_ip_vs_scheduler(&raw mut ip_vs_sed_scheduler)
}

unsafe fn ip_vs_sed_cleanup() {
    unregister_ip_vs_scheduler(&raw mut ip_vs_sed_scheduler);
    synchronize_rcu();
}

module_init!(ip_vs_sed_init);
module_exit!(ip_vs_sed_cleanup);
module_license!("GPL");
module_description!("ipvs shortest expected delay scheduler");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
