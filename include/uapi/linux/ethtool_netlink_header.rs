/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * include/uapi/linux/ethtool_netlink.h - netlink interface for ethtool
 *
 * See Documentation/networking/ethtool-netlink.rst in kernel source tree for
 * doucumentation of the interface.
 */

// Dependencies supplied by linux::ethtool and ethtool_netlink_generated.

pub const ETHTOOL_FLAG_ALL: u32 = ETHTOOL_FLAG_COMPACT_BITSETS
    | ETHTOOL_FLAG_OMIT_REPLY
    | ETHTOOL_FLAG_STATS;

/* CABLE TEST NOTIFY */
pub const ETHTOOL_A_CABLE_RESULT_CODE_UNSPEC: i32 = 0;
pub const ETHTOOL_A_CABLE_RESULT_CODE_OK: i32 = 1;
pub const ETHTOOL_A_CABLE_RESULT_CODE_OPEN: i32 = 2;
pub const ETHTOOL_A_CABLE_RESULT_CODE_SAME_SHORT: i32 = 3;
pub const ETHTOOL_A_CABLE_RESULT_CODE_CROSS_SHORT: i32 = 4;
/* detected reflection caused by the impedance discontinuity between
 * a regular 100 Ohm cable and a part with the abnormal impedance value
 */
pub const ETHTOOL_A_CABLE_RESULT_CODE_IMPEDANCE_MISMATCH: i32 = 5;
/* TDR not possible due to high noise level */
pub const ETHTOOL_A_CABLE_RESULT_CODE_NOISE: i32 = 6;
/* TDR resolution not possible / out of distance */
pub const ETHTOOL_A_CABLE_RESULT_CODE_RESOLUTION_NOT_POSSIBLE: i32 = 7;

pub const ETHTOOL_A_CABLE_PAIR_A: i32 = 0;
pub const ETHTOOL_A_CABLE_PAIR_B: i32 = 1;
pub const ETHTOOL_A_CABLE_PAIR_C: i32 = 2;
pub const ETHTOOL_A_CABLE_PAIR_D: i32 = 3;

/* Information source for specific results. */
pub const ETHTOOL_A_CABLE_INF_SRC_UNSPEC: i32 = 0;
/* Results provided by the Time Domain Reflectometry (TDR) */
pub const ETHTOOL_A_CABLE_INF_SRC_TDR: i32 = 1;
/* Results provided by the Active Link Cable Diagnostic (ALCD) */
pub const ETHTOOL_A_CABLE_INF_SRC_ALCD: i32 = 2;

pub const ETHTOOL_A_CABLE_TEST_NTF_STATUS_UNSPEC: i32 = 0;
pub const ETHTOOL_A_CABLE_TEST_NTF_STATUS_STARTED: i32 = 1;
pub const ETHTOOL_A_CABLE_TEST_NTF_STATUS_COMPLETED: i32 = 2;

/* CABLE TEST TDR NOTIFY */

pub const ETHTOOL_A_CABLE_AMPLITUDE_UNSPEC: i32 = 0;
pub const ETHTOOL_A_CABLE_AMPLITUDE_PAIR: i32 = 1; // u8
pub const ETHTOOL_A_CABLE_AMPLITUDE_mV: i32 = 2; // s16
pub const __ETHTOOL_A_CABLE_AMPLITUDE_CNT: i32 = 3;
pub const ETHTOOL_A_CABLE_AMPLITUDE_MAX: i32 = __ETHTOOL_A_CABLE_AMPLITUDE_CNT - 1;

pub const ETHTOOL_A_CABLE_PULSE_UNSPEC: i32 = 0;
pub const ETHTOOL_A_CABLE_PULSE_mV: i32 = 1; // s16
pub const __ETHTOOL_A_CABLE_PULSE_CNT: i32 = 2;
pub const ETHTOOL_A_CABLE_PULSE_MAX: i32 = __ETHTOOL_A_CABLE_PULSE_CNT - 1;

pub const ETHTOOL_A_CABLE_STEP_UNSPEC: i32 = 0;
pub const ETHTOOL_A_CABLE_STEP_FIRST_DISTANCE: i32 = 1; // u32
pub const ETHTOOL_A_CABLE_STEP_LAST_DISTANCE: i32 = 2; // u32
pub const ETHTOOL_A_CABLE_STEP_STEP_DISTANCE: i32 = 3; // u32
pub const __ETHTOOL_A_CABLE_STEP_CNT: i32 = 4;
pub const ETHTOOL_A_CABLE_STEP_MAX: i32 = __ETHTOOL_A_CABLE_STEP_CNT - 1;

pub const ETHTOOL_A_CABLE_TDR_NEST_UNSPEC: i32 = 0;
pub const ETHTOOL_A_CABLE_TDR_NEST_STEP: i32 = 1; // nest - ETHTTOOL_A_CABLE_STEP
pub const ETHTOOL_A_CABLE_TDR_NEST_AMPLITUDE: i32 = 2; // nest - ETHTOOL_A_CABLE_AMPLITUDE
pub const ETHTOOL_A_CABLE_TDR_NEST_PULSE: i32 = 3; // nest - ETHTOOL_A_CABLE_PULSE
pub const __ETHTOOL_A_CABLE_TDR_NEST_CNT: i32 = 4;
pub const ETHTOOL_A_CABLE_TDR_NEST_MAX: i32 = __ETHTOOL_A_CABLE_TDR_NEST_CNT - 1;

pub const ETHTOOL_STATS_ETH_PHY: i32 = 0;
pub const ETHTOOL_STATS_ETH_MAC: i32 = 1;
pub const ETHTOOL_STATS_ETH_CTRL: i32 = 2;
pub const ETHTOOL_STATS_RMON: i32 = 3;
pub const ETHTOOL_STATS_PHY: i32 = 4;
/* add new constants above here */
pub const __ETHTOOL_STATS_CNT: i32 = 5;

/* 30.3.2.1.5 aSymbolErrorDuringCarrier */
pub const ETHTOOL_A_STATS_ETH_PHY_5_SYM_ERR: i32 = 0;
/* add new constants above here */
pub const __ETHTOOL_A_STATS_ETH_PHY_CNT: i32 = 1;
pub const ETHTOOL_A_STATS_ETH_PHY_MAX: i32 = __ETHTOOL_A_STATS_ETH_PHY_CNT - 1;

/* 30.3.1.1.2 aFramesTransmittedOK */
pub const ETHTOOL_A_STATS_ETH_MAC_2_TX_PKT: i32 = 0;
/* 30.3.1.1.3 aSingleCollisionFrames */
pub const ETHTOOL_A_STATS_ETH_MAC_3_SINGLE_COL: i32 = 1;
/* 30.3.1.1.4 aMultipleCollisionFrames */
pub const ETHTOOL_A_STATS_ETH_MAC_4_MULTI_COL: i32 = 2;
/* 30.3.1.1.5 aFramesReceivedOK */
pub const ETHTOOL_A_STATS_ETH_MAC_5_RX_PKT: i32 = 3;
/* 30.3.1.1.6 aFrameCheckSequenceErrors */
pub const ETHTOOL_A_STATS_ETH_MAC_6_FCS_ERR: i32 = 4;
/* 30.3.1.1.7 aAlignmentErrors */
pub const ETHTOOL_A_STATS_ETH_MAC_7_ALIGN_ERR: i32 = 5;
/* 30.3.1.1.8 aOctetsTransmittedOK */
pub const ETHTOOL_A_STATS_ETH_MAC_8_TX_BYTES: i32 = 6;
/* 30.3.1.1.9 aFramesWithDeferredXmissions */
pub const ETHTOOL_A_STATS_ETH_MAC_9_TX_DEFER: i32 = 7;
/* 30.3.1.1.10 aLateCollisions */
pub const ETHTOOL_A_STATS_ETH_MAC_10_LATE_COL: i32 = 8;
/* 30.3.1.1.11 aFramesAbortedDueToXSColls */
pub const ETHTOOL_A_STATS_ETH_MAC_11_XS_COL: i32 = 9;
/* 30.3.1.1.12 aFramesLostDueToIntMACXmitError */
pub const ETHTOOL_A_STATS_ETH_MAC_12_TX_INT_ERR: i32 = 10;
/* 30.3.1.1.13 aCarrierSenseErrors */
pub const ETHTOOL_A_STATS_ETH_MAC_13_CS_ERR: i32 = 11;
/* 30.3.1.1.14 aOctetsReceivedOK */
pub const ETHTOOL_A_STATS_ETH_MAC_14_RX_BYTES: i32 = 12;
/* 30.3.1.1.15 aFramesLostDueToIntMACRcvError */
pub const ETHTOOL_A_STATS_ETH_MAC_15_RX_INT_ERR: i32 = 13;
/* 30.3.1.1.18 aMulticastFramesXmittedOK */
pub const ETHTOOL_A_STATS_ETH_MAC_18_TX_MCAST: i32 = 14;
/* 30.3.1.1.19 aBroadcastFramesXmittedOK */
pub const ETHTOOL_A_STATS_ETH_MAC_19_TX_BCAST: i32 = 15;
/* 30.3.1.1.20 aFramesWithExcessiveDeferral */
pub const ETHTOOL_A_STATS_ETH_MAC_20_XS_DEFER: i32 = 16;
/* 30.3.1.1.21 aMulticastFramesReceivedOK */
pub const ETHTOOL_A_STATS_ETH_MAC_21_RX_MCAST: i32 = 17;
/* 30.3.1.1.22 aBroadcastFramesReceivedOK */
pub const ETHTOOL_A_STATS_ETH_MAC_22_RX_BCAST: i32 = 18;
/* 30.3.1.1.23 aInRangeLengthErrors */
pub const ETHTOOL_A_STATS_ETH_MAC_23_IR_LEN_ERR: i32 = 19;
/* 30.3.1.1.24 aOutOfRangeLengthField */
pub const ETHTOOL_A_STATS_ETH_MAC_24_OOR_LEN: i32 = 20;
/* 30.3.1.1.25 aFrameTooLongErrors */
pub const ETHTOOL_A_STATS_ETH_MAC_25_TOO_LONG_ERR: i32 = 21;
/* add new constants above here */
pub const __ETHTOOL_A_STATS_ETH_MAC_CNT: i32 = 22;
pub const ETHTOOL_A_STATS_ETH_MAC_MAX: i32 = __ETHTOOL_A_STATS_ETH_MAC_CNT - 1;

/* 30.3.3.3 aMACControlFramesTransmitted */
pub const ETHTOOL_A_STATS_ETH_CTRL_3_TX: i32 = 0;
/* 30.3.3.4 aMACControlFramesReceived */
pub const ETHTOOL_A_STATS_ETH_CTRL_4_RX: i32 = 1;
/* 30.3.3.5 aUnsupportedOpcodesReceived */
pub const ETHTOOL_A_STATS_ETH_CTRL_5_RX_UNSUP: i32 = 2;
/* add new constants above here */
pub const __ETHTOOL_A_STATS_ETH_CTRL_CNT: i32 = 3;
pub const ETHTOOL_A_STATS_ETH_CTRL_MAX: i32 = __ETHTOOL_A_STATS_ETH_CTRL_CNT - 1;

/* etherStatsUndersizePkts */
pub const ETHTOOL_A_STATS_RMON_UNDERSIZE: i32 = 0;
/* etherStatsOversizePkts */
pub const ETHTOOL_A_STATS_RMON_OVERSIZE: i32 = 1;
/* etherStatsFragments */
pub const ETHTOOL_A_STATS_RMON_FRAG: i32 = 2;
/* etherStatsJabbers */
pub const ETHTOOL_A_STATS_RMON_JABBER: i32 = 3;
/* add new constants above here */
pub const __ETHTOOL_A_STATS_RMON_CNT: i32 = 4;
pub const ETHTOOL_A_STATS_RMON_MAX: i32 = __ETHTOOL_A_STATS_RMON_CNT - 1;

/* Basic packet counters if PHY has separate counters from the MAC */
pub const ETHTOOL_A_STATS_PHY_RX_PKTS: i32 = 0;
pub const ETHTOOL_A_STATS_PHY_RX_BYTES: i32 = 1;
pub const ETHTOOL_A_STATS_PHY_RX_ERRORS: i32 = 2;
pub const ETHTOOL_A_STATS_PHY_TX_PKTS: i32 = 3;
pub const ETHTOOL_A_STATS_PHY_TX_BYTES: i32 = 4;
pub const ETHTOOL_A_STATS_PHY_TX_ERRORS: i32 = 5;
/* add new constants above here */
pub const __ETHTOOL_A_STATS_PHY_CNT: i32 = 6;
pub const ETHTOOL_A_STATS_PHY_MAX: i32 = __ETHTOOL_A_STATS_PHY_CNT - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
