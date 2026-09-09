/* SPDX-License-Identifier: GPL-2.0 */
/*
 * 25-Jul-1998 Major changes to allow for ip chain table
 *
 * 3-Jan-2000 Named tables to allow packet selection for different uses.
 */

/*
 *  Format of an IP6 firewall descriptor
 *
 *  src, dst, src_mask, dst_mask are always stored in network byte order.
 *  flags are stored in host byte order (of course).
 *  Port numbers are stored in HOST byte order.
 */

// C header dependencies: linux/if.h, linux/in6.h, linux/init.h,
// linux/ipv6.h, linux/skbuff.h, and uapi/linux/netfilter_ipv6/ip6_tables.h.

extern "C" {
    pub fn ip6t_alloc_initial_table(table: *const xt_table) -> *mut core::ffi::c_void;

    pub fn ip6t_register_table(
        net: *mut net,
        table: *const xt_table,
        repl: *const ip6t_replace,
        ops: *const nf_hook_ops,
    ) -> core::ffi::c_int;

    pub fn ip6t_unregister_table_exit(net: *mut net, name: *const core::ffi::c_char);

    pub fn ip6t_do_table(
        priv_: *mut core::ffi::c_void,
        skb: *mut sk_buff,
        state: *const nf_hook_state,
    ) -> core::ffi::c_uint;
}

// Preserved from CONFIG_NETFILTER_XTABLES_COMPAT.  This block is compiled
// only when the corresponding compatibility configuration is enabled.
#[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
#[repr(C)]
pub struct compat_ip6t_entry {
    pub ipv6: ip6t_ip6,
    pub nfcache: compat_uint_t,
    pub target_offset: u16,
    pub next_offset: u16,
    pub comefrom: compat_uint_t,
    pub counters: compat_xt_counters,
    // C flexible array member: unsigned char elems[];
    pub elems: [u8; 0],
}

#[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
#[inline]
pub unsafe fn compat_ip6t_get_target(e: *mut compat_ip6t_entry) -> *mut xt_entry_target {
    (e as *mut u8).add((*e).target_offset as usize) as *mut xt_entry_target
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
