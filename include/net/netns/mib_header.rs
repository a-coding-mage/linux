/* SPDX-License-Identifier: GPL-2.0 */

// Translated from <net/snmp.h>; the C preprocessor configuration is retained
// as Rust configuration attributes where applicable.

#[repr(C)]
pub struct netns_mib {
    // DEFINE_SNMP_STAT(struct ipstats_mib, ip_statistics);
    pub ip_statistics: *mut ipstats_mib,

    #[cfg(CONFIG_IPV6)]
    // DEFINE_SNMP_STAT(struct ipstats_mib, ipv6_statistics);
    pub ipv6_statistics: *mut ipstats_mib,

    // DEFINE_SNMP_STAT(struct tcp_mib, tcp_statistics);
    pub tcp_statistics: *mut tcp_mib,
    // DEFINE_SNMP_STAT(struct linux_mib, net_statistics);
    pub net_statistics: *mut linux_mib,

    // DEFINE_SNMP_STAT(struct udp_mib, udp_statistics);
    pub udp_statistics: *mut udp_mib,
    #[cfg(CONFIG_IPV6)]
    // DEFINE_SNMP_STAT(struct udp_mib, udp_stats_in6);
    pub udp_stats_in6: *mut udp_mib,

    #[cfg(CONFIG_XFRM_STATISTICS)]
    // DEFINE_SNMP_STAT(struct linux_xfrm_mib, xfrm_statistics);
    pub xfrm_statistics: *mut linux_xfrm_mib,
    #[cfg(CONFIG_TLS)]
    // DEFINE_SNMP_STAT(struct linux_tls_mib, tls_statistics);
    pub tls_statistics: *mut linux_tls_mib,
    #[cfg(CONFIG_MPTCP)]
    // DEFINE_SNMP_STAT(struct mptcp_mib, mptcp_statistics);
    pub mptcp_statistics: *mut mptcp_mib,

    // DEFINE_SNMP_STAT(struct icmp_mib, icmp_statistics);
    pub icmp_statistics: *mut icmp_mib,
    // DEFINE_SNMP_STAT_ATOMIC(struct icmpmsg_mib, icmpmsg_statistics);
    pub icmpmsg_statistics: *mut icmpmsg_mib,
    #[cfg(CONFIG_IPV6)]
    // DEFINE_SNMP_STAT(struct icmpv6_mib, icmpv6_statistics);
    pub icmpv6_statistics: *mut icmpv6_mib,
    #[cfg(CONFIG_IPV6)]
    // DEFINE_SNMP_STAT_ATOMIC(struct icmpv6msg_mib, icmpv6msg_statistics);
    pub icmpv6msg_statistics: *mut icmpv6msg_mib,
    #[cfg(CONFIG_IPV6)]
    pub proc_net_devsnmp6: *mut proc_dir_entry,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
