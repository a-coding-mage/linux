/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Userspace API for hardware time stamping of network packets
 *
 * Copyright (C) 2008,2009 Intel Corporation
 * Author: Patrick Ohly <patrick.ohly@intel.com>
 */

/* Possible type of hwtstamp provider. Mainly "precise" the default one
 * is for IEEE 1588 quality and "approx" is for NICs DMA point.
 */
#[repr(i32)]
pub enum hwtstamp_provider_qualifier {
    HWTSTAMP_PROVIDER_QUALIFIER_PRECISE,
    HWTSTAMP_PROVIDER_QUALIFIER_APPROX,
    HWTSTAMP_PROVIDER_QUALIFIER_CNT,
}

/* SO_TIMESTAMPING flags */
pub const SOF_TIMESTAMPING_TX_HARDWARE: i32 = 1 << 0;
pub const SOF_TIMESTAMPING_TX_SOFTWARE: i32 = 1 << 1;
pub const SOF_TIMESTAMPING_RX_HARDWARE: i32 = 1 << 2;
pub const SOF_TIMESTAMPING_RX_SOFTWARE: i32 = 1 << 3;
pub const SOF_TIMESTAMPING_SOFTWARE: i32 = 1 << 4;
pub const SOF_TIMESTAMPING_SYS_HARDWARE: i32 = 1 << 5;
pub const SOF_TIMESTAMPING_RAW_HARDWARE: i32 = 1 << 6;
pub const SOF_TIMESTAMPING_OPT_ID: i32 = 1 << 7;
pub const SOF_TIMESTAMPING_TX_SCHED: i32 = 1 << 8;
pub const SOF_TIMESTAMPING_TX_ACK: i32 = 1 << 9;
pub const SOF_TIMESTAMPING_OPT_CMSG: i32 = 1 << 10;
pub const SOF_TIMESTAMPING_OPT_TSONLY: i32 = 1 << 11;
pub const SOF_TIMESTAMPING_OPT_STATS: i32 = 1 << 12;
pub const SOF_TIMESTAMPING_OPT_PKTINFO: i32 = 1 << 13;
pub const SOF_TIMESTAMPING_OPT_TX_SWHW: i32 = 1 << 14;
pub const SOF_TIMESTAMPING_BIND_PHC: i32 = 1 << 15;
pub const SOF_TIMESTAMPING_OPT_ID_TCP: i32 = 1 << 16;
pub const SOF_TIMESTAMPING_OPT_RX_FILTER: i32 = 1 << 17;
pub const SOF_TIMESTAMPING_TX_COMPLETION: i32 = 1 << 18;
pub const SOF_TIMESTAMPING_LAST: i32 = SOF_TIMESTAMPING_TX_COMPLETION;
pub const SOF_TIMESTAMPING_MASK: i32 = (SOF_TIMESTAMPING_LAST - 1) | SOF_TIMESTAMPING_LAST;

/* Recording flags can be set both via socket options and control messages. */
pub const SOF_TIMESTAMPING_TX_RECORD_MASK: i32 = SOF_TIMESTAMPING_TX_HARDWARE
    | SOF_TIMESTAMPING_TX_SOFTWARE
    | SOF_TIMESTAMPING_TX_SCHED
    | SOF_TIMESTAMPING_TX_ACK
    | SOF_TIMESTAMPING_TX_COMPLETION;

#[repr(C)]
pub struct so_timestamping {
    pub flags: i32,
    pub bind_phc: i32,
}

#[repr(C)]
pub struct hwtstamp_config {
    pub flags: i32,
    pub tx_type: i32,
    pub rx_filter: i32,
}

/* possible values for hwtstamp_config->flags */
pub const HWTSTAMP_FLAG_BONDED_PHC_INDEX: i32 = 1 << 0;
pub const HWTSTAMP_FLAG_LAST: i32 = HWTSTAMP_FLAG_BONDED_PHC_INDEX;
pub const HWTSTAMP_FLAG_MASK: i32 = (HWTSTAMP_FLAG_LAST - 1) | HWTSTAMP_FLAG_LAST;

/* possible values for hwtstamp_config->tx_type */
#[repr(i32)]
pub enum hwtstamp_tx_types {
    HWTSTAMP_TX_OFF,
    HWTSTAMP_TX_ON,
    HWTSTAMP_TX_ONESTEP_SYNC,
    HWTSTAMP_TX_ONESTEP_P2P,
    __HWTSTAMP_TX_CNT,
}

/* possible values for hwtstamp_config->rx_filter */
#[repr(i32)]
pub enum hwtstamp_rx_filters {
    HWTSTAMP_FILTER_NONE,
    HWTSTAMP_FILTER_ALL,
    HWTSTAMP_FILTER_SOME,
    HWTSTAMP_FILTER_PTP_V1_L4_EVENT,
    HWTSTAMP_FILTER_PTP_V1_L4_SYNC,
    HWTSTAMP_FILTER_PTP_V1_L4_DELAY_REQ,
    HWTSTAMP_FILTER_PTP_V2_L4_EVENT,
    HWTSTAMP_FILTER_PTP_V2_L4_SYNC,
    HWTSTAMP_FILTER_PTP_V2_L4_DELAY_REQ,
    HWTSTAMP_FILTER_PTP_V2_L2_EVENT,
    HWTSTAMP_FILTER_PTP_V2_L2_SYNC,
    HWTSTAMP_FILTER_PTP_V2_L2_DELAY_REQ,
    HWTSTAMP_FILTER_PTP_V2_EVENT,
    HWTSTAMP_FILTER_PTP_V2_SYNC,
    HWTSTAMP_FILTER_PTP_V2_DELAY_REQ,
    HWTSTAMP_FILTER_NTP_ALL,
    __HWTSTAMP_FILTER_CNT,
}

/* SCM_TIMESTAMPING_PKTINFO control message */
#[repr(C)]
pub struct scm_ts_pktinfo {
    pub if_index: __u32,
    pub pkt_length: __u32,
    pub reserved: [__u32; 2],
}

/* SO_TXTIME gets a struct sock_txtime with flags being an integer bit field. */
pub const SOF_TXTIME_DEADLINE_MODE: i32 = 1 << 0;
pub const SOF_TXTIME_REPORT_ERRORS: i32 = 1 << 1;
pub const SOF_TXTIME_FLAGS_LAST: i32 = SOF_TXTIME_REPORT_ERRORS;
pub const SOF_TXTIME_FLAGS_MASK: i32 = (SOF_TXTIME_FLAGS_LAST - 1) | SOF_TXTIME_FLAGS_LAST;

#[repr(C)]
pub struct sock_txtime {
    pub clockid: __kernel_clockid_t,
    pub flags: __u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
