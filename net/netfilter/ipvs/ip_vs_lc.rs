// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IPVS:        Least-Connection Scheduling module
 *
 * Authors:     Wensong Zhang <wensong@linuxvirtualserver.org>
 *
 * Changes:
 *     Wensong Zhang            :     added the ip_vs_lc_update_svc
 *     Wensong Zhang            :     added any dest with weight=0 is quiesced
 */

// pr_fmt(fmt) "IPVS: " fmt
// Dependencies are supplied by the surrounding IPVS implementation.

/*
 *	Least Connection scheduling
 */
unsafe fn ip_vs_lc_schedule(
    svc: *mut ip_vs_service,
    skb: *const sk_buff,
    iph: *mut ip_vs_iphdr,
) -> *mut ip_vs_dest {
    let mut dest: *mut ip_vs_dest;
    let mut least: *mut ip_vs_dest = core::ptr::null_mut();
    let mut loh: u32 = 0;
    let mut doh: u32;

    IP_VS_DBG!(6, "%s(): Scheduling...\n", "ip_vs_lc_schedule");

    /*
     * Simply select the server with the least number of
     *        (activeconns*256) + totalconns
     * Except whose weight is equal to zero.
     * If the weight is equal to zero, it means that the server is
     * quiesced, the existing connections to the server still get
     * served, but no new connection is assigned to the server.
     */

    list_for_each_entry_rcu!(dest, (*svc).destinations, n_list) {
        if (((*dest).flags & IP_VS_DEST_F_OVERLOAD) != 0
            || atomic_read!(&(*dest).weight) == 0)
        {
            continue;
        }
        doh = ip_vs_dest_conn_overhead(dest);
        if least.is_null() || doh < loh {
            least = dest;
            loh = doh;
        }
    }

    if least.is_null() {
        ip_vs_scheduler_err(svc, "no destination available");
    } else {
        IP_VS_DBG_BUF!(
            6,
            "LC: server %s:%u activeconns %d inactconns %d\n",
            IP_VS_DBG_ADDR!((*least).af, &(*least).addr),
            ntohs((*least).port),
            atomic_read!(&(*least).activeconns),
            ip_vs_dest_inactconns(least),
        );
    }

    least
}

static mut ip_vs_lc_scheduler: ip_vs_scheduler = ip_vs_scheduler {
    name: "lc",
    refcnt: ATOMIC_INIT!(0),
    module: THIS_MODULE,
    n_list: LIST_HEAD_INIT!(ip_vs_lc_scheduler.n_list),
    schedule: Some(ip_vs_lc_schedule),
};

unsafe fn ip_vs_lc_init() -> i32 {
    register_ip_vs_scheduler(&raw mut ip_vs_lc_scheduler)
}

unsafe fn ip_vs_lc_cleanup() {
    unregister_ip_vs_scheduler(&raw mut ip_vs_lc_scheduler);
    synchronize_rcu();
}

module_init!(ip_vs_lc_init);
module_exit!(ip_vs_lc_cleanup);
MODULE_LICENSE!("GPL");
MODULE_DESCRIPTION!("ipvs least connection scheduler");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
