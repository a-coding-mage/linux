// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IPVS:        Weighted Fail Over module
 *
 * Authors:     Kenny Mathis <kmathis@chokepoint.net>
 *
 * Changes:
 *     Kenny Mathis            :     added initial functionality based on weight
 */

// pr_fmt(fmt) is "IPVS: " fmt.
// C dependencies supplied by the surrounding kernel/IPVS translation unit:
// linux/module.h, linux/kernel.h, and net/ip_vs.h.

use crate::linux::module::{module_exit, module_init, synchronize_rcu, THIS_MODULE};
use crate::linux::types::{atomic_read, AtomicT, SkBuff};
use crate::net::ip_vs::{
    ip_vs_scheduler_err, register_ip_vs_scheduler, unregister_ip_vs_scheduler,
    IpVsDest, IpVsIphdr, IpVsScheduler, IpVsService, IP_VS_DEST_F_OVERLOAD,
};

/* Weighted Fail Over Module */
unsafe fn ip_vs_fo_schedule(
    svc: *mut IpVsService,
    _skb: *const SkBuff,
    _iph: *mut IpVsIphdr,
) -> *mut IpVsDest {
    let mut dest: *mut IpVsDest;
    let mut hweight: *mut IpVsDest = core::ptr::null_mut();
    let mut hw: i32 = 0; /* Track highest weight */

    // IP_VS_DBG(6, "ip_vs_fo_schedule(): Scheduling...\n");
    crate::net::ip_vs::IP_VS_DBG!(6, "ip_vs_fo_schedule(): Scheduling...\n");

    /* Basic failover functionality
     * Find virtual server with highest weight and send it traffic
     */
    // list_for_each_entry_rcu(dest, &svc->destinations, n_list)
    crate::linux::list::list_for_each_entry_rcu!(dest, (*svc).destinations, n_list, {
        if ((*dest).flags & IP_VS_DEST_F_OVERLOAD) == 0
            && atomic_read(&(*dest).weight) > hw
        {
            hweight = dest;
            hw = atomic_read(&(*dest).weight);
        }
    });

    if !hweight.is_null() {
        // IP_VS_DBG_BUF(6, "FO: server %s:%u activeconns %d weight %d\n", ...);
        crate::net::ip_vs::IP_VS_DBG_BUF!(
            6,
            "FO: server %s:%u activeconns %d weight %d\n",
            crate::net::ip_vs::IP_VS_DBG_ADDR!((*hweight).af, &(*hweight).addr),
            u16::from_be((*hweight).port),
            atomic_read(&(*hweight).activeconns),
            atomic_read(&(*hweight).weight)
        );
        return hweight;
    }

    ip_vs_scheduler_err(svc, "no destination available");
    core::ptr::null_mut()
}

static mut ip_vs_fo_scheduler: IpVsScheduler = IpVsScheduler {
    name: b"fo\0".as_ptr() as *const core::ffi::c_char,
    refcnt: AtomicT::init(0),
    module: THIS_MODULE,
    n_list: crate::linux::list::ListHead::new(),
    schedule: Some(ip_vs_fo_schedule),
};

unsafe fn ip_vs_fo_init() -> i32 {
    register_ip_vs_scheduler(&raw mut ip_vs_fo_scheduler)
}

unsafe fn ip_vs_fo_cleanup() {
    unregister_ip_vs_scheduler(&raw mut ip_vs_fo_scheduler);
    synchronize_rcu();
}

module_init!(ip_vs_fo_init);
module_exit!(ip_vs_fo_cleanup);
crate::linux::module::MODULE_LICENSE!("GPL");
crate::linux::module::MODULE_DESCRIPTION!("ipvs weighted failover scheduler");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
