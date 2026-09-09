// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IPVS:        Never Queue scheduling module
 *
 * Authors:     Wensong Zhang <wensong@linuxvirtualserver.org>
 *
 * The NQ algorithm adopts a two-speed model. When there is an idle server
 * available, the job will be sent to the idle server, instead of waiting
 * for a fast one. When there is no idle server available, the job will be
 * sent to the server that minimize its expected delay.
 */

// The following names are supplied by the surrounding IPVS implementation.

#[inline]
unsafe fn ip_vs_nq_dest_overhead(dest: *mut ip_vs_dest) -> i32 {
    /* We only use the active connection number in the cost calculation. */
    (*dest).activeconns.read() + 1
}

/* Weighted Least Connection scheduling. */
unsafe extern "C" fn ip_vs_nq_schedule(
    svc: *mut ip_vs_service,
    _skb: *const sk_buff,
    _iph: *mut ip_vs_iphdr,
) -> *mut ip_vs_dest {
    let mut dest: *mut ip_vs_dest;
    let mut least: *mut ip_vs_dest = core::ptr::null_mut();
    let mut loh: i32 = 0;
    let mut doh: i32;

    IP_VS_DBG!(6, "%s(): Scheduling...\n", "ip_vs_nq_schedule");

    /*
     * The load of each destination is (expected overhead) / weight.
     * Comparing h1*w2 > h2*w1 avoids floating point arithmetic.
     * A destination with weight zero is quiesced.
     */
    list_for_each_entry_rcu!(dest, (*svc).destinations, n_list, {
        if ((*dest).flags & IP_VS_DEST_F_OVERLOAD) != 0
            || (*dest).weight.read() == 0
        {
            continue;
        }

        doh = ip_vs_nq_dest_overhead(dest);

        /* Return the server directly if it is idle. */
        if (*dest).activeconns.read() == 0 {
            least = dest;
            loh = doh;
            goto_out!();
        }

        if least.is_null()
            || (i64::from(loh) * i64::from((*dest).weight.read())
                > i64::from(doh) * i64::from((*least).weight.read()))
        {
            least = dest;
            loh = doh;
        }
    });

    if least.is_null() {
        ip_vs_scheduler_err(svc, "no destination available");
        return core::ptr::null_mut();
    }

out:
    IP_VS_DBG_BUF!(
        6,
        "NQ: server %s:%u activeconns %d refcnt %d weight %d overhead %d\n",
        IP_VS_DBG_ADDR!((*least).af, &(*least).addr),
        ntohs((*least).port),
        (*least).activeconns.read(),
        (*least).refcnt.read(),
        (*least).weight.read(),
        loh
    );

    least
}

static mut ip_vs_nq_scheduler: ip_vs_scheduler = ip_vs_scheduler {
    name: "nq",
    refcnt: ATOMIC_INIT!(0),
    module: THIS_MODULE,
    n_list: LIST_HEAD_INIT!(ip_vs_nq_scheduler.n_list),
    schedule: Some(ip_vs_nq_schedule),
};

unsafe extern "C" fn ip_vs_nq_init() -> i32 {
    register_ip_vs_scheduler(&raw mut ip_vs_nq_scheduler)
}

unsafe extern "C" fn ip_vs_nq_cleanup() {
    unregister_ip_vs_scheduler(&raw mut ip_vs_nq_scheduler);
    synchronize_rcu();
}

module_init!(ip_vs_nq_init);
module_exit!(ip_vs_nq_cleanup);
MODULE_LICENSE!("GPL");
MODULE_DESCRIPTION!("ipvs never queue scheduler");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
