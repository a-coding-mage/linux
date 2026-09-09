/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ipv6 in net namespaces
 */

// Dependency declarations from <net/inet_frag.h>, <net/dst_ops.h>, and
// <uapi/linux/icmpv6.h> are supplied by other translated units.

pub struct ctl_table_header;

#[repr(C)]
pub struct netns_sysctl_ipv6 {
    // CONFIG_SYSCTL fields are present when CONFIG_SYSCTL is enabled.
    #[cfg(feature = "CONFIG_SYSCTL")]
    pub hdr: *mut ctl_table_header,
    #[cfg(feature = "CONFIG_SYSCTL")]
    pub route_hdr: *mut ctl_table_header,
    #[cfg(feature = "CONFIG_SYSCTL")]
    pub icmp_hdr: *mut ctl_table_header,
    #[cfg(feature = "CONFIG_SYSCTL")]
    pub frags_hdr: *mut ctl_table_header,
    #[cfg(feature = "CONFIG_SYSCTL")]
    pub xfrm6_hdr: *mut ctl_table_header,
    pub flush_delay: ::core::ffi::c_int,
    pub ip6_rt_max_size: ::core::ffi::c_int,
    pub ip6_rt_gc_min_interval: ::core::ffi::c_int,
    pub ip6_rt_gc_timeout: ::core::ffi::c_int,
    pub ip6_rt_gc_interval: ::core::ffi::c_int,
    pub ip6_rt_gc_elasticity: ::core::ffi::c_int,
    pub ip6_rt_mtu_expires: ::core::ffi::c_int,
    pub ip6_rt_min_advmss: ::core::ffi::c_int,
    pub multipath_hash_fields: u32,
    pub multipath_hash_policy: u8,
    // __cacheline_group_begin(sysctl_ipv6_flowlabel)
    pub flowlabel_consistency: u8,
    pub auto_flowlabels: u8,
    pub flowlabel_state_ranges: u8,
    // __cacheline_group_end(sysctl_ipv6_flowlabel)
    pub icmpv6_echo_ignore_all: u8,
    pub icmpv6_echo_ignore_multicast: u8,
    pub icmpv6_echo_ignore_anycast: u8,
    pub icmpv6_time: ::core::ffi::c_int,
    // DECLARE_BITMAP(icmpv6_ratemask, ICMPV6_MSG_MAX + 1)
    pub icmpv6_ratemask: [::core::ffi::c_ulong; (ICMPV6_MSG_MAX as usize + 1 + (::core::mem::size_of::<::core::ffi::c_ulong>() * 8 - 1)) / (::core::mem::size_of::<::core::ffi::c_ulong>() * 8)],
    pub icmpv6_ratemask_ptr: *mut ::core::ffi::c_ulong,
    pub anycast_src_echo_reply: u8,
    pub bindv6only: u8,
    pub ip_nonlocal_bind: u8,
    pub fwmark_reflect: u8,
    pub idgen_retries: ::core::ffi::c_int,
    pub idgen_delay: ::core::ffi::c_int,
    pub flowlabel_reflect: ::core::ffi::c_int,
    pub max_dst_opts_cnt: ::core::ffi::c_int,
    pub max_hbh_opts_cnt: ::core::ffi::c_int,
    pub max_dst_opts_len: ::core::ffi::c_int,
    pub max_hbh_opts_len: ::core::ffi::c_int,
    pub seg6_flowlabel: ::core::ffi::c_int,
    pub ioam6_id: u32,
    pub ioam6_id_wide: u64,
    pub skip_notify_on_dev_down: u8,
    pub fib_notify_on_flag_change: u8,
    pub icmpv6_error_anycast_as_unicast: u8,
    pub icmpv6_errors_extension_mask: u8,
}

#[repr(C)]
pub struct netns_ipv6 {
    /* Keep ip6_dst_ops at the beginning of netns_sysctl_ipv6 */
    pub ip6_dst_ops: dst_ops,
    pub sysctl: netns_sysctl_ipv6,
    pub devconf_all: *mut ipv6_devconf,
    pub devconf_dflt: *mut ipv6_devconf,
    pub peers: *mut inet_peer_base,
    pub fqdir: *mut fqdir,
    pub fib6_null_entry: *mut fib6_info,
    pub ip6_null_entry: *mut rt6_info,
    pub rt6_stats: *mut rt6_statistics,
    pub ip6_fib_timer: timer_list,
    pub fib_table_hash: *mut hlist_head,
    pub fib_table_hash_lock: spinlock_t,
    pub fib6_main_tbl: *mut fib6_table,
    pub fib6_walkers: list_head,
    pub fib6_walker_lock: rwlock_t,
    pub fib6_gc_lock: spinlock_t,
    pub ip6_rt_gc_expire: atomic_t,
    pub ip6_rt_last_gc: ::core::ffi::c_ulong,
    pub flowlabel_has_excl: u8,
    // CONFIG_IPV6_MULTIPLE_TABLES fields are present when enabled.
    #[cfg(feature = "CONFIG_IPV6_MULTIPLE_TABLES")]
    pub fib6_has_custom_rules: bool,
    #[cfg(feature = "CONFIG_IPV6_MULTIPLE_TABLES")]
    pub fib6_rules_require_fldissect: ::core::ffi::c_uint,
    #[cfg(all(feature = "CONFIG_IPV6_MULTIPLE_TABLES", feature = "CONFIG_IPV6_SUBTREES"))]
    pub fib6_routes_require_src: ::core::ffi::c_uint,
    #[cfg(feature = "CONFIG_IPV6_MULTIPLE_TABLES")]
    pub ip6_prohibit_entry: *mut rt6_info,
    #[cfg(feature = "CONFIG_IPV6_MULTIPLE_TABLES")]
    pub ip6_blk_hole_entry: *mut rt6_info,
    #[cfg(feature = "CONFIG_IPV6_MULTIPLE_TABLES")]
    pub fib6_local_tbl: *mut fib6_table,
    #[cfg(feature = "CONFIG_IPV6_MULTIPLE_TABLES")]
    pub fib6_rules_ops: *mut fib_rules_ops,
    pub ndisc_sk: *mut sock,
    pub tcp_sk: *mut sock,
    pub igmp_sk: *mut sock,
    pub mc_autojoin_sk: *mut sock,
    pub inet6_addr_lst: *mut hlist_head,
    pub addrconf_hash_lock: spinlock_t,
    pub addr_chk_work: delayed_work,
    // CONFIG_IPV6_MROUTE fields are present when enabled.
    #[cfg(all(feature = "CONFIG_IPV6_MROUTE", not(feature = "CONFIG_IPV6_MROUTE_MULTIPLE_TABLES")))]
    pub mrt6: *mut mr_table,
    #[cfg(all(feature = "CONFIG_IPV6_MROUTE", feature = "CONFIG_IPV6_MROUTE_MULTIPLE_TABLES"))]
    pub mr6_tables: list_head,
    #[cfg(all(feature = "CONFIG_IPV6_MROUTE", feature = "CONFIG_IPV6_MROUTE_MULTIPLE_TABLES"))]
    pub mr6_rules_ops: *mut fib_rules_ops,
    #[cfg(feature = "CONFIG_IPV6_MROUTE")]
    pub ip6mr_notifier_ops: *mut fib_notifier_ops,
    #[cfg(feature = "CONFIG_IPV6_MROUTE")]
    pub ipmr_seq: atomic_t,
    #[cfg(feature = "CONFIG_IPV6_MROUTE")]
    pub mfc_mutex: mutex,
    pub dev_addr_genid: atomic_t,
    pub fib6_sernum: atomic_t,
    pub seg6_data: *mut seg6_pernet_data,
    pub notifier_ops: *mut fib_notifier_ops,
    pub flowlabel_count: ::core::ffi::c_int,
    pub ip6addrlbl_table: ip6addrlbl_table,
    pub ioam6_data: *mut ioam6_pernet_data,
}

#[repr(C)]
pub struct ip6addrlbl_table {
    pub head: hlist_head,
    pub lock: spinlock_t,
    pub seq: u32,
}

#[cfg(feature = "CONFIG_NF_DEFRAG_IPV6")]
#[repr(C)]
pub struct netns_nf_frag {
    pub fqdir: *mut fqdir,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
