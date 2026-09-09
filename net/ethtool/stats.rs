// SPDX-License-Identifier: GPL-2.0-only

// Translated from stats.c. Kernel and ethtool types/functions are supplied by
// the surrounding crate.

#[repr(C)]
pub struct stats_req_info {
    pub base: ethnl_req_info,
    pub stat_mask: [usize; 1],
    pub src: ethtool_mac_stats_src,
}

#[repr(C)]
pub struct stats_reply_data {
    pub base: ethnl_reply_data,
    pub phy_stats: ethtool_eth_phy_stats,
    pub mac_stats: ethtool_eth_mac_stats,
    pub ctrl_stats: ethtool_eth_ctrl_stats,
    pub rmon_stats: ethtool_rmon_stats,
    pub phydev_stats: ethtool_phy_stats,
    pub rmon_ranges: *const ethtool_rmon_hist_range,
}

pub static mut stats_std_names: [[u8; ETH_GSTRING_LEN]; __ETHTOOL_STATS_CNT] = {
    let mut a = [[0; ETH_GSTRING_LEN]; __ETHTOOL_STATS_CNT];
    a[ETHTOOL_STATS_ETH_PHY] = *b"eth-phy\0";
    a[ETHTOOL_STATS_ETH_MAC] = *b"eth-mac\0";
    a[ETHTOOL_STATS_ETH_CTRL] = *b"eth-ctrl\0";
    a[ETHTOOL_STATS_RMON] = *b"rmon\0";
    a[ETHTOOL_STATS_PHY] = *b"phydev\0";
    a
};

pub static mut stats_eth_phy_names: [[u8; ETH_GSTRING_LEN]; __ETHTOOL_A_STATS_ETH_PHY_CNT] = {
    let mut a = [[0; ETH_GSTRING_LEN]; __ETHTOOL_A_STATS_ETH_PHY_CNT];
    a[ETHTOOL_A_STATS_ETH_PHY_5_SYM_ERR] = *b"SymbolErrorDuringCarrier\0"; a
};

pub static mut stats_eth_mac_names: [[u8; ETH_GSTRING_LEN]; __ETHTOOL_A_STATS_ETH_MAC_CNT] = {
    let mut a = [[0; ETH_GSTRING_LEN]; __ETHTOOL_A_STATS_ETH_MAC_CNT];
    a[ETHTOOL_A_STATS_ETH_MAC_2_TX_PKT] = *b"FramesTransmittedOK\0";
    a[ETHTOOL_A_STATS_ETH_MAC_3_SINGLE_COL] = *b"SingleCollisionFrames\0";
    a[ETHTOOL_A_STATS_ETH_MAC_4_MULTI_COL] = *b"MultipleCollisionFrames\0";
    a[ETHTOOL_A_STATS_ETH_MAC_5_RX_PKT] = *b"FramesReceivedOK\0";
    a[ETHTOOL_A_STATS_ETH_MAC_6_FCS_ERR] = *b"FrameCheckSequenceErrors\0";
    a[ETHTOOL_A_STATS_ETH_MAC_7_ALIGN_ERR] = *b"AlignmentErrors\0";
    a[ETHTOOL_A_STATS_ETH_MAC_8_TX_BYTES] = *b"OctetsTransmittedOK\0";
    a[ETHTOOL_A_STATS_ETH_MAC_9_TX_DEFER] = *b"FramesWithDeferredXmissions\0";
    a[ETHTOOL_A_STATS_ETH_MAC_10_LATE_COL] = *b"LateCollisions\0";
    a[ETHTOOL_A_STATS_ETH_MAC_11_XS_COL] = *b"FramesAbortedDueToXSColls\0";
    a[ETHTOOL_A_STATS_ETH_MAC_12_TX_INT_ERR] = *b"FramesLostDueToIntMACXmitError\0";
    a[ETHTOOL_A_STATS_ETH_MAC_13_CS_ERR] = *b"CarrierSenseErrors\0";
    a[ETHTOOL_A_STATS_ETH_MAC_14_RX_BYTES] = *b"OctetsReceivedOK\0";
    a[ETHTOOL_A_STATS_ETH_MAC_15_RX_INT_ERR] = *b"FramesLostDueToIntMACRcvError\0";
    a[ETHTOOL_A_STATS_ETH_MAC_18_TX_MCAST] = *b"MulticastFramesXmittedOK\0";
    a[ETHTOOL_A_STATS_ETH_MAC_19_TX_BCAST] = *b"BroadcastFramesXmittedOK\0";
    a[ETHTOOL_A_STATS_ETH_MAC_20_XS_DEFER] = *b"FramesWithExcessiveDeferral\0";
    a[ETHTOOL_A_STATS_ETH_MAC_21_RX_MCAST] = *b"MulticastFramesReceivedOK\0";
    a[ETHTOOL_A_STATS_ETH_MAC_22_RX_BCAST] = *b"BroadcastFramesReceivedOK\0";
    a[ETHTOOL_A_STATS_ETH_MAC_23_IR_LEN_ERR] = *b"InRangeLengthErrors\0";
    a[ETHTOOL_A_STATS_ETH_MAC_24_OOR_LEN] = *b"OutOfRangeLengthField\0";
    a[ETHTOOL_A_STATS_ETH_MAC_25_TOO_LONG_ERR] = *b"FrameTooLongErrors\0"; a
};

pub static mut stats_eth_ctrl_names: [[u8; ETH_GSTRING_LEN]; __ETHTOOL_A_STATS_ETH_CTRL_CNT] = {
    let mut a = [[0; ETH_GSTRING_LEN]; __ETHTOOL_A_STATS_ETH_CTRL_CNT];
    a[ETHTOOL_A_STATS_ETH_CTRL_3_TX] = *b"MACControlFramesTransmitted\0";
    a[ETHTOOL_A_STATS_ETH_CTRL_4_RX] = *b"MACControlFramesReceived\0";
    a[ETHTOOL_A_STATS_ETH_CTRL_5_RX_UNSUP] = *b"UnsupportedOpcodesReceived\0"; a
};
pub static mut stats_rmon_names: [[u8; ETH_GSTRING_LEN]; __ETHTOOL_A_STATS_RMON_CNT] = {
    let mut a = [[0; ETH_GSTRING_LEN]; __ETHTOOL_A_STATS_RMON_CNT];
    a[ETHTOOL_A_STATS_RMON_UNDERSIZE] = *b"etherStatsUndersizePkts\0";
    a[ETHTOOL_A_STATS_RMON_OVERSIZE] = *b"etherStatsOversizePkts\0";
    a[ETHTOOL_A_STATS_RMON_FRAG] = *b"etherStatsFragments\0";
    a[ETHTOOL_A_STATS_RMON_JABBER] = *b"etherStatsJabbers\0"; a
};
pub static mut stats_phy_names: [[u8; ETH_GSTRING_LEN]; __ETHTOOL_A_STATS_PHY_CNT] = {
    let mut a = [[0; ETH_GSTRING_LEN]; __ETHTOOL_A_STATS_PHY_CNT];
    a[ETHTOOL_A_STATS_PHY_RX_PKTS] = *b"RxFrames\0";
    a[ETHTOOL_A_STATS_PHY_RX_BYTES] = *b"RxOctets\0";
    a[ETHTOOL_A_STATS_PHY_RX_ERRORS] = *b"RxErrors\0";
    a[ETHTOOL_A_STATS_PHY_TX_PKTS] = *b"TxFrames\0";
    a[ETHTOOL_A_STATS_PHY_TX_BYTES] = *b"TxOctets\0";
    a[ETHTOOL_A_STATS_PHY_TX_ERRORS] = *b"TxErrors\0"; a
};

unsafe fn ethtool_stats_sum(a: u64, b: u64) -> u64 {
    if a == ETHTOOL_STAT_NOT_SET { b } else if b == ETHTOOL_STAT_NOT_SET { a } else { a.wrapping_add(b) }
}

unsafe fn ethtool_aggregate_stats(aggr_stats: *mut u8, emac_stats: *const u8, pmac_stats: *const u8, stats_size: usize, stats_offset: usize) {
    let n = stats_size / core::mem::size_of::<u64>();
    let s1 = emac_stats.add(stats_offset) as *const u64;
    let s2 = pmac_stats.add(stats_offset) as *const u64;
    let s = aggr_stats.add(stats_offset) as *mut u64;
    for i in 0..n { *s.add(i) = ethtool_stats_sum(*s1.add(i), *s2.add(i)); }
}

pub unsafe fn ethtool_aggregate_mac_stats(dev: *mut net_device, mac_stats: *mut ethtool_eth_mac_stats) {
    let ops = (*dev).ethtool_ops; let mut pmac: ethtool_eth_mac_stats = core::mem::zeroed(); let mut emac: ethtool_eth_mac_stats = core::mem::zeroed();
    core::ptr::write_bytes(&mut emac as *mut _ as *mut u8, 0xff, core::mem::size_of_val(&emac)); core::ptr::write_bytes(&mut pmac as *mut _ as *mut u8, 0xff, core::mem::size_of_val(&pmac));
    emac.src = ETHTOOL_MAC_STATS_SRC_EMAC; pmac.src = ETHTOOL_MAC_STATS_SRC_PMAC;
    ((*ops).get_eth_mac_stats.unwrap())(dev, &mut emac); ((*ops).get_eth_mac_stats.unwrap())(dev, &mut pmac);
    ethtool_aggregate_stats(mac_stats as *mut u8, &emac as *const _ as *const u8, &pmac as *const _ as *const u8, core::mem::size_of_val(&(*mac_stats).stats), core::mem::offset_of!(ethtool_eth_mac_stats, stats));
}

pub unsafe fn ethtool_aggregate_phy_stats(dev: *mut net_device, out: *mut ethtool_eth_phy_stats) { let ops=(*dev).ethtool_ops; let mut a: ethtool_eth_phy_stats=core::mem::zeroed(); let mut b: ethtool_eth_phy_stats=core::mem::zeroed(); core::ptr::write_bytes(&mut a as *mut _ as *mut u8,0xff,core::mem::size_of_val(&a)); core::ptr::write_bytes(&mut b as *mut _ as *mut u8,0xff,core::mem::size_of_val(&b)); a.src=ETHTOOL_MAC_STATS_SRC_EMAC;b.src=ETHTOOL_MAC_STATS_SRC_PMAC;((*ops).get_eth_phy_stats.unwrap())(dev,&mut a);((*ops).get_eth_phy_stats.unwrap())(dev,&mut b);ethtool_aggregate_stats(out as *mut u8,&a as *const _ as *const u8,&b as *const _ as *const u8,core::mem::size_of_val(&(*out).stats),core::mem::offset_of!(ethtool_eth_phy_stats,stats)); }

pub unsafe fn ethtool_aggregate_ctrl_stats(dev: *mut net_device, out: *mut ethtool_eth_ctrl_stats) { let ops=(*dev).ethtool_ops; let mut a: ethtool_eth_ctrl_stats=core::mem::zeroed(); let mut b: ethtool_eth_ctrl_stats=core::mem::zeroed(); core::ptr::write_bytes(&mut a as *mut _ as *mut u8,0xff,core::mem::size_of_val(&a));core::ptr::write_bytes(&mut b as *mut _ as *mut u8,0xff,core::mem::size_of_val(&b));a.src=ETHTOOL_MAC_STATS_SRC_EMAC;b.src=ETHTOOL_MAC_STATS_SRC_PMAC;((*ops).get_eth_ctrl_stats.unwrap())(dev,&mut a);((*ops).get_eth_ctrl_stats.unwrap())(dev,&mut b);ethtool_aggregate_stats(out as *mut u8,&a as *const _ as *const u8,&b as *const _ as *const u8,core::mem::size_of_val(&(*out).stats),core::mem::offset_of!(ethtool_eth_ctrl_stats,stats)); }

pub unsafe fn ethtool_aggregate_pause_stats(dev: *mut net_device, out: *mut ethtool_pause_stats) { let ops=(*dev).ethtool_ops; let mut a: ethtool_pause_stats=core::mem::zeroed(); let mut b: ethtool_pause_stats=core::mem::zeroed(); core::ptr::write_bytes(&mut a as *mut _ as *mut u8,0xff,core::mem::size_of_val(&a));core::ptr::write_bytes(&mut b as *mut _ as *mut u8,0xff,core::mem::size_of_val(&b));a.src=ETHTOOL_MAC_STATS_SRC_EMAC;b.src=ETHTOOL_MAC_STATS_SRC_PMAC;((*ops).get_pause_stats.unwrap())(dev,&mut a);((*ops).get_pause_stats.unwrap())(dev,&mut b);ethtool_aggregate_stats(out as *mut u8,&a as *const _ as *const u8,&b as *const _ as *const u8,core::mem::size_of_val(&(*out).stats),core::mem::offset_of!(ethtool_pause_stats,stats)); }

pub unsafe fn ethtool_aggregate_rmon_stats(dev: *mut net_device, out: *mut ethtool_rmon_stats) { let ops=(*dev).ethtool_ops; let mut a: ethtool_rmon_stats=core::mem::zeroed(); let mut b: ethtool_rmon_stats=core::mem::zeroed(); let mut dummy: *const ethtool_rmon_hist_range=core::ptr::null(); core::ptr::write_bytes(&mut a as *mut _ as *mut u8,0xff,core::mem::size_of_val(&a));core::ptr::write_bytes(&mut b as *mut _ as *mut u8,0xff,core::mem::size_of_val(&b));a.src=ETHTOOL_MAC_STATS_SRC_EMAC;b.src=ETHTOOL_MAC_STATS_SRC_PMAC;((*ops).get_rmon_stats.unwrap())(dev,&mut a,&mut dummy);((*ops).get_rmon_stats.unwrap())(dev,&mut b,&mut dummy);ethtool_aggregate_stats(out as *mut u8,&a as *const _ as *const u8,&b as *const _ as *const u8,core::mem::size_of_val(&(*out).stats),core::mem::offset_of!(ethtool_rmon_stats,stats)); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
