// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IPVS:        Weighted Least-Connection Scheduling module
 *
 * Authors:     Wensong Zhang <wensong@linuxvirtualserver.org>
 *              Peter Kese <peter.kese@ijs.si>
 *
 * Changes:
 *     Wensong Zhang            :     changed the ip_vs_wlc_schedule to return dest
 *     Wensong Zhang            :     changed to use the inactconns in scheduling
 *     Wensong Zhang            :     changed some comestics things for debugging
 *     Wensong Zhang            :     changed for the d-linked destination list
 *     Wensong Zhang            :     added the ip_vs_wlc_update_svc
 *     Wensong Zhang            :     added any dest with weight=0 is quiesced
 */

/* C dependencies supplied by the surrounding kernel translation. */

/*
 *	Weighted Least Connection scheduling
 */
unsafe fn ip_vs_wlc_schedule(
    svc: *mut ip_vs_service,
    skb: *const sk_buff,
    iph: *mut ip_vs_iphdr,
) -> *mut ip_vs_dest {
    let _ = (skb, iph);
    let mut dest: *mut ip_vs_dest;
    let mut least: *mut ip_vs_dest;
    let mut loh: i32;
    let mut doh: i32;

    IP_VS_DBG!(6, "ip_vs_wlc_schedule(): Scheduling...\n");

    /*
     * We calculate the load of each dest server as follows:
     *             (dest overhead) / dest->weight
     *
     * Remember -- no floats in kernel mode!!!
     * The comparison of h1*w2 > h2*w1 is equivalent to that of
     *             h1/w1 > h2/w2
     * if every weight is larger than zero.
     *
     * The server with weight=0 is quiesced and will not receive any
     * new connections.
     */

    /* Equivalent of list_for_each_entry_rcu(dest, &svc->destinations, n_list). */
    list_for_each_entry_rcu!(dest, (*svc).destinations, n_list, {
        if ((*dest).flags & IP_VS_DEST_F_OVERLOAD) == 0
            && atomic_read!((*dest).weight) > 0
        {
            least = dest;
            loh = ip_vs_dest_conn_overhead(least);
            goto_nextstage!();
        }
    });

    ip_vs_scheduler_err(svc, "no destination available");
    return core::ptr::null_mut();

    /*
     *    Find the destination with the least load.
     */
    nextstage!();
    list_for_each_entry_continue_rcu!(dest, (*svc).destinations, n_list, {
        if ((*dest).flags & IP_VS_DEST_F_OVERLOAD) != 0 {
            continue;
        }
        doh = ip_vs_dest_conn_overhead(dest);
        if (loh as i64 * atomic_read!((*dest).weight) as i64
            > doh as i64 * atomic_read!((*least).weight) as i64)
        {
            least = dest;
            loh = doh;
        }
    });

    IP_VS_DBG_BUF!(
        6,
        "WLC: server %s:%u activeconns %d refcnt %d weight %d overhead %d\n",
        IP_VS_DBG_ADDR!((*least).af, &(*least).addr),
        ntohs!((*least).port),
        atomic_read!((*least).activeconns),
        refcount_read!((*least).refcnt),
        atomic_read!((*least).weight),
        loh,
    );

    least
}

static mut ip_vs_wlc_scheduler: ip_vs_scheduler = ip_vs_scheduler {
    name: "wlc",
    refcnt: ATOMIC_INIT!(0),
    module: THIS_MODULE,
    n_list: LIST_HEAD_INIT!(ip_vs_wlc_scheduler.n_list),
    schedule: Some(ip_vs_wlc_schedule),
};

unsafe fn ip_vs_wlc_init() -> i32 {
    register_ip_vs_scheduler(&raw mut ip_vs_wlc_scheduler)
}

unsafe fn ip_vs_wlc_cleanup() {
    unregister_ip_vs_scheduler(&raw mut ip_vs_wlc_scheduler);
    synchronize_rcu();
}

module_init!(ip_vs_wlc_init);
module_exit!(ip_vs_wlc_cleanup);
MODULE_LICENSE!("GPL");
MODULE_DESCRIPTION!("ipvs weighted least connection scheduler");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
