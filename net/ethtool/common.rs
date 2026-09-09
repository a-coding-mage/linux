// SPDX-License-Identifier: GPL-2.0-only
//
// Rust translation of common.c. Kernel and ethtool types/functions are
// supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

/* C designated initializers are represented as sparse name/value tables.
 * The numeric indices and constants are provided by the kernel bindings. */
pub static netdev_features_strings: &[(&str, &str)] = &[
    ("NETIF_F_SG_BIT", "tx-scatter-gather"), ("NETIF_F_IP_CSUM_BIT", "tx-checksum-ipv4"),
    ("NETIF_F_HW_CSUM_BIT", "tx-checksum-ip-generic"), ("NETIF_F_IPV6_CSUM_BIT", "tx-checksum-ipv6"),
    ("NETIF_F_HIGHDMA_BIT", "highdma"), ("NETIF_F_FRAGLIST_BIT", "tx-scatter-gather-fraglist"),
    ("NETIF_F_HW_VLAN_CTAG_TX_BIT", "tx-vlan-hw-insert"), ("NETIF_F_HW_VLAN_CTAG_RX_BIT", "rx-vlan-hw-parse"),
    ("NETIF_F_HW_VLAN_CTAG_FILTER_BIT", "rx-vlan-filter"), ("NETIF_F_HW_VLAN_STAG_TX_BIT", "tx-vlan-stag-hw-insert"),
    ("NETIF_F_HW_VLAN_STAG_RX_BIT", "rx-vlan-stag-hw-parse"), ("NETIF_F_HW_VLAN_STAG_FILTER_BIT", "rx-vlan-stag-filter"),
    ("NETIF_F_VLAN_CHALLENGED_BIT", "vlan-challenged"), ("NETIF_F_GSO_BIT", "tx-generic-segmentation"),
    ("NETIF_F_GRO_BIT", "rx-gro"), ("NETIF_F_GRO_HW_BIT", "rx-gro-hw"), ("NETIF_F_LRO_BIT", "rx-lro"),
    ("NETIF_F_TSO_BIT", "tx-tcp-segmentation"), ("NETIF_F_GSO_ROBUST_BIT", "tx-gso-robust"),
    ("NETIF_F_TSO_ECN_BIT", "tx-tcp-ecn-segmentation"), ("NETIF_F_GSO_ACCECN_BIT", "tx-tcp-accecn-segmentation"),
    ("NETIF_F_TSO_MANGLEID_BIT", "tx-tcp-mangleid-segmentation"), ("NETIF_F_TSO6_BIT", "tx-tcp6-segmentation"),
    ("NETIF_F_FSO_BIT", "tx-fcoe-segmentation"), ("NETIF_F_GSO_GRE_BIT", "tx-gre-segmentation"),
    ("NETIF_F_GSO_GRE_CSUM_BIT", "tx-gre-csum-segmentation"), ("NETIF_F_GSO_IPXIP4_BIT", "tx-ipxip4-segmentation"),
    ("NETIF_F_GSO_IPXIP6_BIT", "tx-ipxip6-segmentation"), ("NETIF_F_GSO_UDP_TUNNEL_BIT", "tx-udp_tnl-segmentation"),
    ("NETIF_F_GSO_UDP_TUNNEL_CSUM_BIT", "tx-udp_tnl-csum-segmentation"), ("NETIF_F_GSO_PARTIAL_BIT", "tx-gso-partial"),
    ("NETIF_F_GSO_TUNNEL_REMCSUM_BIT", "tx-tunnel-remcsum-segmentation"), ("NETIF_F_GSO_SCTP_BIT", "tx-sctp-segmentation"),
    ("NETIF_F_GSO_ESP_BIT", "tx-esp-segmentation"), ("NETIF_F_GSO_UDP_L4_BIT", "tx-udp-segmentation"),
    ("NETIF_F_GSO_FRAGLIST_BIT", "tx-gso-list"), ("NETIF_F_FCOE_CRC_BIT", "tx-checksum-fcoe-crc"),
    ("NETIF_F_SCTP_CRC_BIT", "tx-checksum-sctp"), ("NETIF_F_NTUPLE_BIT", "rx-ntuple-filter"),
    ("NETIF_F_RXHASH_BIT", "rx-hashing"), ("NETIF_F_RXCSUM_BIT", "rx-checksum"),
    ("NETIF_F_NOCACHE_COPY_BIT", "tx-nocache-copy"), ("NETIF_F_LOOPBACK_BIT", "loopback"),
    ("NETIF_F_RXFCS_BIT", "rx-fcs"), ("NETIF_F_RXALL_BIT", "rx-all"),
    ("NETIF_F_HW_L2FW_DOFFLOAD_BIT", "l2-fwd-offload"), ("NETIF_F_HW_TC_BIT", "hw-tc-offload"),
    ("NETIF_F_HW_ESP_BIT", "esp-hw-offload"), ("NETIF_F_HW_ESP_TX_CSUM_BIT", "esp-tx-csum-hw-offload"),
    ("NETIF_F_RX_UDP_TUNNEL_PORT_BIT", "rx-udp_tunnel-port-offload"), ("NETIF_F_HW_TLS_TX_BIT", "tls-hw-tx-offload"),
    ("NETIF_F_HW_TLS_RX_BIT", "tls-hw-rx-offload"), ("NETIF_F_GRO_FRAGLIST_BIT", "rx-gro-list"),
    ("NETIF_F_HW_MACSEC_BIT", "macsec-hw-offload"), ("NETIF_F_GRO_UDP_FWD_BIT", "rx-udp-gro-forwarding"),
    ("NETIF_F_HW_HSR_TAG_INS_BIT", "hsr-tag-ins-offload"), ("NETIF_F_HW_HSR_TAG_RM_BIT", "hsr-tag-rm-offload"),
    ("NETIF_F_HW_HSR_FWD_BIT", "hsr-fwd-offload"), ("NETIF_F_HW_HSR_DUP_BIT", "hsr-dup-offload"),
];

pub static rss_hash_func_strings: &[(&str, &str)] = &[("ETH_RSS_HASH_TOP_BIT", "toeplitz"), ("ETH_RSS_HASH_XOR_BIT", "xor"), ("ETH_RSS_HASH_CRC32_BIT", "crc32")];
pub static tunable_strings: &[(&str, &str)] = &[("ETHTOOL_ID_UNSPEC", "Unspec"), ("ETHTOOL_RX_COPYBREAK", "rx-copybreak"), ("ETHTOOL_TX_COPYBREAK", "tx-copybreak"), ("ETHTOOL_PFC_PREVENTION_TOUT", "pfc-prevention-tout"), ("ETHTOOL_TX_COPYBREAK_BUF_SIZE", "tx-copybreak-buf-size")];
pub static phy_tunable_strings: &[(&str, &str)] = &[("ETHTOOL_ID_UNSPEC", "Unspec"), ("ETHTOOL_PHY_DOWNSHIFT", "phy-downshift"), ("ETHTOOL_PHY_FAST_LINK_DOWN", "phy-fast-link-down"), ("ETHTOOL_PHY_EDPD", "phy-energy-detect-power-down"), ("ETHTOOL_PHY_SHORT_CABLE_PRESET", "phy-short-cable-preset"), ("ETHTOOL_PHY_LPF_BW", "phy-lpf-bandwidth"), ("ETHTOOL_PHY_DSP_EQ_INIT_VALUE", "phy-dsp-eq-init-value")];

pub static link_mode_names: &[(&str, &str)] = &[
    ("10baseT/Half", "10baseT/Half"), ("10baseT/Full", "10baseT/Full"), ("100baseT/Half", "100baseT/Half"), ("100baseT/Full", "100baseT/Full"),
    ("1000baseT/Half", "1000baseT/Half"), ("1000baseT/Full", "1000baseT/Full"), ("Autoneg", "Autoneg"), ("TP", "TP"), ("AUI", "AUI"), ("MII", "MII"), ("FIBRE", "FIBRE"), ("BNC", "BNC"),
    ("10000baseT/Full", "10000baseT/Full"), ("Pause", "Pause"), ("Asym_Pause", "Asym_Pause"), ("2500baseX/Full", "2500baseX/Full"), ("Backplane", "Backplane"),
    ("1000baseKX/Full", "1000baseKX/Full"), ("10000baseKX4/Full", "10000baseKX4/Full"), ("10000baseKR/Full", "10000baseKR/Full"), ("10000baseR_FEC", "10000baseR_FEC"),
    ("20000baseMLD2/Full", "20000baseMLD2/Full"), ("20000baseKR2/Full", "20000baseKR2/Full"), ("40000baseKR4/Full", "40000baseKR4/Full"), ("40000baseCR4/Full", "40000baseCR4/Full"), ("40000baseSR4/Full", "40000baseSR4/Full"), ("40000baseLR4/Full", "40000baseLR4/Full"),
    ("25000baseCR/Full", "25000baseCR/Full"), ("25000baseKR/Full", "25000baseKR/Full"), ("25000baseSR/Full", "25000baseSR/Full"), ("50000baseCR2/Full", "50000baseCR2/Full"), ("50000baseKR2/Full", "50000baseKR2/Full"),
    ("FEC_NONE", "None"), ("FEC_RS", "RS"), ("FEC_BASER", "BASER"), ("FEC_LLRS", "LLRS"),
];

/* The remaining string tables retain the exact C names and values. */
pub static ethtool_link_medium_names: &[&str] = &["BaseT", "BaseK", "BaseS", "BaseC", "BaseL", "BaseD", "BaseE", "BaseF", "BaseV", "BaseMLD", "None"];
pub static netif_msg_class_names: &[&str] = &["drv", "probe", "link", "timer", "ifdown", "ifup", "rx_err", "tx_err", "tx_queued", "intr", "tx_done", "rx_status", "pktdata", "hw", "wol"];
pub static wol_mode_names: &[&str] = &["phy", "ucast", "mcast", "bcast", "arp", "magic", "magicsecure", "filter"];
pub static sof_timestamping_names: &[&str] = &["hardware-transmit", "software-transmit", "hardware-receive", "software-receive", "software-system-clock", "hardware-legacy-clock", "hardware-raw-clock", "option-id", "sched-transmit", "ack-transmit", "option-cmsg", "option-tsonly", "option-stats", "option-pktinfo", "option-tx-swhw", "bind-phc", "option-id-tcp", "option-rx-filter", "tx-completion"];
pub static ts_tx_type_names: &[&str] = &["off", "on", "onestep-sync", "onestep-p2p"];
pub static ts_rx_filter_names: &[&str] = &["none", "all", "some", "ptpv1-l4-event", "ptpv1-l4-sync", "ptpv1-l4-delay-req", "ptpv2-l4-event", "ptpv2-l4-sync", "ptpv2-l4-delay-req", "ptpv2-l2-event", "ptpv2-l2-sync", "ptpv2-l2-delay-req", "ptpv2-event", "ptpv2-sync", "ptpv2-delay-req", "ntp-all"];
pub static ts_flags_names: &[&str] = &["bonded-phc-index"];
pub static udp_tunnel_type_names: &[&str] = &["vxlan", "geneve", "vxlan-gpe"];

/* File-local algorithms translated literally. */
pub unsafe fn ethtool_rxfh_is_periodic(tbl: *const u32, old_size: u32, new_size: u32) -> bool {
    let mut i = new_size;
    while i < old_size { if *tbl.add(i as usize) != *tbl.add((i % new_size) as usize) { return false; } i += 1; }
    true
}
pub unsafe fn ethtool_rxfh_can_resize(tbl: *const u32, old_size: u32, new_size: u32, user_size: u32) -> bool {
    if new_size == old_size || user_size == 0 { return true; }
    if new_size < old_size { return new_size >= user_size && old_size % new_size == 0 && ethtool_rxfh_is_periodic(tbl, old_size, new_size); }
    new_size % old_size == 0
}
pub unsafe fn ethtool_rxfh_resize(tbl: *mut u32, old_size: u32, new_size: u32) {
    let mut i = old_size; while i < new_size { *tbl.add(i as usize) = *tbl.add((i % old_size) as usize); i += 1; }
}

/* External kernel declarations used by the implementation below. */
extern "C" {
    fn ethtool_rss_notify(dev: *mut c_void, msg: u32, context: u32);
}

/* The structures and helper functions below are intentionally declared by
 * common.h and the kernel headers; their complete definitions are external. */
pub unsafe fn ethtool_rxfh_indir_can_resize(dev: *mut c_void, tbl: *const u32, old_size: u32, new_size: u32, user_size: u32) -> bool {
    let _ = dev;
    ethtool_rxfh_can_resize(tbl, old_size, new_size, user_size)
}
pub unsafe fn ethtool_rxfh_indir_resize(dev: *mut c_void, tbl: *mut u32, old_size: u32, new_size: u32, user_size: u32) {
    let _ = dev;
    if user_size != 0 { ethtool_rxfh_resize(tbl, old_size, new_size); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
