/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation:
// #include <net/gen_stats.h>

#[repr(C)]
pub struct xt_rateest {
    /* keep lock and bstats on same cache line to speedup xt_rateest_tg() */
    pub bstats: gnet_stats_basic_sync,
    pub lock: spinlock_t,

    /* following fields not accessed in hot path */
    pub refcnt: ::core::ffi::c_uint,
    pub list: hlist_node,
    pub name: [::core::ffi::c_char; IFNAMSIZ],
    pub params: gnet_estimator,
    pub rcu: rcu_head,

    /* keep this field far away to speedup xt_rateest_mt() */
    pub rate_est: *mut net_rate_estimator,
}

extern "C" {
    pub fn xt_rateest_lookup(
        net: *mut net,
        name: *const ::core::ffi::c_char,
    ) -> *mut xt_rateest;
    pub fn xt_rateest_put(net: *mut net, est: *mut xt_rateest);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
