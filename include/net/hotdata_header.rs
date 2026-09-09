/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependencies supplied by the corresponding kernel headers are intentionally
// referenced here rather than redefined.

#[repr(C)]
pub struct skb_defer_node {
    pub defer_list: llist_head,
    pub defer_count: atomic_long_t,
}

/* Read mostly data used in network fast paths. */
#[repr(C)]
pub struct net_hotdata {
    // Preserved from IS_ENABLED(CONFIG_INET).
    #[cfg(feature = "CONFIG_INET")]
    pub ip_packet_offload: packet_offload,
    #[cfg(feature = "CONFIG_INET")]
    pub tcpv4_offload: net_offload,
    #[cfg(feature = "CONFIG_INET")]
    pub tcp_protocol: net_protocol,
    #[cfg(feature = "CONFIG_INET")]
    pub udpv4_offload: net_offload,
    #[cfg(feature = "CONFIG_INET")]
    pub udp_protocol: net_protocol,
    #[cfg(feature = "CONFIG_INET")]
    pub ipv6_packet_offload: packet_offload,
    #[cfg(feature = "CONFIG_INET")]
    pub tcpv6_offload: net_offload,
    // Preserved from IS_ENABLED(CONFIG_IPV6).
    #[cfg(all(feature = "CONFIG_INET", feature = "CONFIG_IPV6"))]
    pub tcpv6_protocol: inet6_protocol,
    #[cfg(all(feature = "CONFIG_INET", feature = "CONFIG_IPV6"))]
    pub udpv6_protocol: inet6_protocol,
    #[cfg(feature = "CONFIG_INET")]
    pub udpv6_offload: net_offload,

    pub offload_base: list_head,
    pub skbuff_cache: *mut kmem_cache,
    pub skbuff_fclone_cache: *mut kmem_cache,
    pub skb_small_head_cache: *mut kmem_cache,
    // Preserved from CONFIG_RPS.
    #[cfg(feature = "CONFIG_RPS")]
    pub rps_sock_flow_table: rps_tag_ptr,
    #[cfg(feature = "CONFIG_RPS")]
    pub rps_cpu_mask: u32,
    pub skb_defer_nodes: *mut skb_defer_node,
    pub gro_normal_batch: i32,
    pub netdev_budget: i32,
    pub netdev_budget_usecs: i32,
    pub tstamp_prequeue: i32,
    pub max_backlog: i32,
    pub qdisc_max_burst: i32,
    pub dev_tx_weight: i32,
    pub dev_rx_weight: i32,
    pub sysctl_max_skb_frags: i32,
    pub sysctl_skb_defer_max: i32,
    pub sysctl_mem_pcpu_rsv: i32,
}

macro_rules! inet_ehash_secret {
    () => { net_hotdata.tcp_protocol.secret };
}
macro_rules! udp_ehash_secret {
    () => { net_hotdata.udp_protocol.secret };
}
macro_rules! inet6_ehash_secret {
    () => { net_hotdata.tcpv6_protocol.secret };
}
macro_rules! tcp_ipv6_hash_secret {
    () => { net_hotdata.tcpv6_offload.secret };
}
macro_rules! udp6_ehash_secret {
    () => { net_hotdata.udpv6_protocol.secret };
}
macro_rules! udp_ipv6_hash_secret {
    () => { net_hotdata.udpv6_offload.secret };
}

extern "C" {
    pub static mut net_hotdata: net_hotdata;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
