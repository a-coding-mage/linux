// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 2023 Texas Instruments Incorporated - https://www.ti.com
 */

// Definitions supplied by k3-psil-priv.h are external to this translation.

macro_rules! PSIL_PDMA_XY_TR {
    ($x:expr) => { psil_ep { thread_id: $x, ep_config: psil_ep_config { ep_type: PSIL_EP_PDMA_XY, mapped_channel_id: -1, default_flow_id: -1, ..Default::default() } } };
}
macro_rules! PSIL_PDMA_XY_PKT {
    ($x:expr) => { psil_ep { thread_id: $x, ep_config: psil_ep_config { ep_type: PSIL_EP_PDMA_XY, mapped_channel_id: -1, default_flow_id: -1, pkt_mode: 1, ..Default::default() } } };
}
macro_rules! PSIL_ETHERNET {
    ($x:expr, $ch:expr, $flow_base:expr, $flow_cnt:expr) => { psil_ep { thread_id: $x, ep_config: psil_ep_config { ep_type: PSIL_EP_NATIVE, pkt_mode: 1, needs_epib: 1, psd_size: 16, mapped_channel_id: $ch, flow_start: $flow_base, flow_num: $flow_cnt, default_flow_id: $flow_base, ..Default::default() } } };
}
macro_rules! PSIL_SAUL {
    ($x:expr, $ch:expr, $flow_base:expr, $flow_cnt:expr, $default_flow:expr, $tx:expr) => { psil_ep { thread_id: $x, ep_config: psil_ep_config { ep_type: PSIL_EP_NATIVE, pkt_mode: 1, needs_epib: 1, psd_size: 64, mapped_channel_id: $ch, flow_start: $flow_base, flow_num: $flow_cnt, default_flow_id: $default_flow, notdpkt: $tx, ..Default::default() } } };
}
macro_rules! PSIL_PDMA_MCASP {
    ($x:expr) => { psil_ep { thread_id: $x, ep_config: psil_ep_config { ep_type: PSIL_EP_PDMA_XY, pdma_acc32: 1, pdma_burst: 1, ..Default::default() } } };
}
macro_rules! PSIL_CSI2RX {
    ($x:expr) => { psil_ep { thread_id: $x, ep_config: psil_ep_config { ep_type: PSIL_EP_NATIVE, ..Default::default() } } };
}
macro_rules! PSIL_CSI_BLOCK {
    ($b:expr) => { PSIL_CSI2RX!($b+0),PSIL_CSI2RX!($b+1),PSIL_CSI2RX!($b+2),PSIL_CSI2RX!($b+3),PSIL_CSI2RX!($b+4),PSIL_CSI2RX!($b+5),PSIL_CSI2RX!($b+6),PSIL_CSI2RX!($b+7),PSIL_CSI2RX!($b+8),PSIL_CSI2RX!($b+9),PSIL_CSI2RX!($b+10),PSIL_CSI2RX!($b+11),PSIL_CSI2RX!($b+12),PSIL_CSI2RX!($b+13),PSIL_CSI2RX!($b+14),PSIL_CSI2RX!($b+15),PSIL_CSI2RX!($b+16),PSIL_CSI2RX!($b+17),PSIL_CSI2RX!($b+18),PSIL_CSI2RX!($b+19),PSIL_CSI2RX!($b+20),PSIL_CSI2RX!($b+21),PSIL_CSI2RX!($b+22),PSIL_CSI2RX!($b+23),PSIL_CSI2RX!($b+24),PSIL_CSI2RX!($b+25),PSIL_CSI2RX!($b+26),PSIL_CSI2RX!($b+27),PSIL_CSI2RX!($b+28),PSIL_CSI2RX!($b+29),PSIL_CSI2RX!($b+30),PSIL_CSI2RX!($b+31) };
}

/* PSI-L source thread IDs, used for RX (DMA_DEV_TO_MEM) */
static mut am62p_src_ep_map: [psil_ep; 0] = [];

/* PSI-L destination thread IDs, used for TX (DMA_MEM_TO_DEV) */
static mut am62p_dst_ep_map: [psil_ep; 0] = [];

// The map contents are expressed through the source macros below.  The
// external psil_ep representation determines the concrete array element type.
const AM62P_SRC_ENTRIES: &[psil_ep] = &[
    PSIL_SAUL!(0x7504, 20, 35, 8, 35, 0), PSIL_SAUL!(0x7505, 21, 35, 8, 36, 0),
    PSIL_SAUL!(0x7506, 22, 43, 8, 43, 0), PSIL_SAUL!(0x7507, 23, 43, 8, 44, 0),
    PSIL_PDMA_XY_PKT!(0x4300), PSIL_PDMA_XY_PKT!(0x4301), PSIL_PDMA_XY_PKT!(0x4302), PSIL_PDMA_XY_PKT!(0x4303),
    PSIL_PDMA_XY_PKT!(0x4304), PSIL_PDMA_XY_PKT!(0x4305), PSIL_PDMA_XY_PKT!(0x4306), PSIL_PDMA_XY_PKT!(0x4307),
    PSIL_PDMA_XY_PKT!(0x4308), PSIL_PDMA_XY_PKT!(0x4309), PSIL_PDMA_XY_PKT!(0x430a), PSIL_PDMA_XY_PKT!(0x430b),
    PSIL_PDMA_XY_PKT!(0x4400), PSIL_PDMA_XY_PKT!(0x4401), PSIL_PDMA_XY_PKT!(0x4402), PSIL_PDMA_XY_PKT!(0x4403),
    PSIL_PDMA_XY_PKT!(0x4404), PSIL_PDMA_XY_PKT!(0x4405), PSIL_PDMA_XY_PKT!(0x4406),
    PSIL_PDMA_MCASP!(0x4500), PSIL_PDMA_MCASP!(0x4501), PSIL_PDMA_MCASP!(0x4502),
    PSIL_ETHERNET!(0x4600, 19, 19, 16),
    PSIL_CSI_BLOCK!(0x5000), PSIL_CSI_BLOCK!(0x5000),
    PSIL_CSI_BLOCK!(0x5100), PSIL_CSI_BLOCK!(0x5200), PSIL_CSI_BLOCK!(0x5300),
];

const AM62P_DST_ENTRIES: &[psil_ep] = &[
    PSIL_SAUL!(0xf500, 27, 83, 8, 83, 1), PSIL_SAUL!(0xf501, 28, 91, 8, 91, 1),
    PSIL_PDMA_XY_PKT!(0xc300), PSIL_PDMA_XY_PKT!(0xc301), PSIL_PDMA_XY_PKT!(0xc302), PSIL_PDMA_XY_PKT!(0xc303),
    PSIL_PDMA_XY_PKT!(0xc304), PSIL_PDMA_XY_PKT!(0xc305), PSIL_PDMA_XY_PKT!(0xc306), PSIL_PDMA_XY_PKT!(0xc307),
    PSIL_PDMA_XY_PKT!(0xc308), PSIL_PDMA_XY_PKT!(0xc309), PSIL_PDMA_XY_PKT!(0xc30a), PSIL_PDMA_XY_PKT!(0xc30b),
    PSIL_PDMA_XY_PKT!(0xc400), PSIL_PDMA_XY_PKT!(0xc401), PSIL_PDMA_XY_PKT!(0xc402), PSIL_PDMA_XY_PKT!(0xc403),
    PSIL_PDMA_XY_PKT!(0xc404), PSIL_PDMA_XY_PKT!(0xc405), PSIL_PDMA_XY_PKT!(0xc406),
    PSIL_PDMA_MCASP!(0xc500), PSIL_PDMA_MCASP!(0xc501), PSIL_PDMA_MCASP!(0xc502),
    PSIL_ETHERNET!(0xc600, 19, 19, 8), PSIL_ETHERNET!(0xc601, 20, 27, 8),
    PSIL_ETHERNET!(0xc602, 21, 35, 8), PSIL_ETHERNET!(0xc603, 22, 43, 8),
    PSIL_ETHERNET!(0xc604, 23, 51, 8), PSIL_ETHERNET!(0xc605, 24, 59, 8),
    PSIL_ETHERNET!(0xc606, 25, 67, 8), PSIL_ETHERNET!(0xc607, 26, 75, 8),
];

static mut am62p_ep_map: psil_ep_map = psil_ep_map {
    name: "am62p",
    src: AM62P_SRC_ENTRIES.as_ptr() as *mut psil_ep,
    src_count: AM62P_SRC_ENTRIES.len(),
    dst: AM62P_DST_ENTRIES.as_ptr() as *mut psil_ep,
    dst_count: AM62P_DST_ENTRIES.len(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
