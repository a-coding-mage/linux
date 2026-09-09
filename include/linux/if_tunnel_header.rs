/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux networking declarations:
// <linux/ip.h>, <linux/in6.h>, <uapi/linux/if_tunnel.h>,
// and <linux/u64_stats_sync.h>.

/*
 * Locking : hash tables are protected by RCU and RTNL
 */

/// C equivalent:
///
/// ```c
/// #define for_each_ip_tunnel_rcu(pos, start) \\
///     for (pos = rcu_dereference(start); pos; pos = rcu_dereference(pos->next))
/// ```
#[macro_export]
macro_rules! for_each_ip_tunnel_rcu {
    ($pos:ident, $start:expr) => {
        for $pos in unsafe { rcu_dereference($start) } {
            unsafe {
                $pos = rcu_dereference((*$pos).next);
            }
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
