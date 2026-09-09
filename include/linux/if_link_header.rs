/* SPDX-License-Identifier: GPL-2.0 */

// Corresponds to: #include <uapi/linux/if_link.h>

/* We don't want this structure exposed to user space */
#[repr(C)]
pub struct ifla_vf_stats {
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub broadcast: u64,
    pub multicast: u64,
    pub rx_dropped: u64,
    pub tx_dropped: u64,
}

#[repr(C)]
pub struct ifla_vf_info {
    pub vf: u32,
    pub mac: [u8; 32],
    pub vlan: u32,
    pub qos: u32,
    pub spoofchk: u32,
    pub linkstate: u32,
    pub min_tx_rate: u32,
    pub max_tx_rate: u32,
    pub rss_query_en: u32,
    pub trusted: u32,
    pub vlan_proto: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
