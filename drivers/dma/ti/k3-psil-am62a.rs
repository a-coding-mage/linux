// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 2022 Texas Instruments Incorporated - https://www.ti.com
 */

// Dependency declarations supplied by the surrounding kernel translation.

macro_rules! psil_pdma_xy_tr {
    ($x:expr) => {
        psil_ep {
            thread_id: $x,
            ep_config: psil_ep_config {
                ep_type: PSIL_EP_PDMA_XY,
                mapped_channel_id: -1,
                default_flow_id: -1,
                ..Default::default()
            },
        }
    };
}

macro_rules! psil_pdma_xy_pkt {
    ($x:expr) => {
        psil_ep {
            thread_id: $x,
            ep_config: psil_ep_config {
                ep_type: PSIL_EP_PDMA_XY,
                mapped_channel_id: -1,
                default_flow_id: -1,
                pkt_mode: 1,
                ..Default::default()
            },
        }
    };
}

macro_rules! psil_ethernet {
    ($x:expr, $ch:expr, $flow_base:expr, $flow_cnt:expr) => {
        psil_ep {
            thread_id: $x,
            ep_config: psil_ep_config {
                ep_type: PSIL_EP_NATIVE,
                pkt_mode: 1,
                needs_epib: 1,
                psd_size: 16,
                mapped_channel_id: $ch,
                flow_start: $flow_base,
                flow_num: $flow_cnt,
                default_flow_id: $flow_base,
                ..Default::default()
            },
        }
    };
}

macro_rules! psil_saul {
    ($x:expr, $ch:expr, $flow_base:expr, $flow_cnt:expr, $default_flow:expr, $tx:expr) => {
        psil_ep {
            thread_id: $x,
            ep_config: psil_ep_config {
                ep_type: PSIL_EP_NATIVE,
                pkt_mode: 1,
                needs_epib: 1,
                psd_size: 64,
                mapped_channel_id: $ch,
                flow_start: $flow_base,
                flow_num: $flow_cnt,
                default_flow_id: $default_flow,
                notdpkt: $tx,
                ..Default::default()
            },
        }
    };
}

macro_rules! psil_pdma_mcasp {
    ($x:expr) => {
        psil_ep {
            thread_id: $x,
            ep_config: psil_ep_config {
                ep_type: PSIL_EP_PDMA_XY,
                pdma_acc32: 1,
                pdma_burst: 1,
                ..Default::default()
            },
        }
    };
}

macro_rules! psil_csi2rx {
    ($x:expr) => {
        psil_ep {
            thread_id: $x,
            ep_config: psil_ep_config {
                ep_type: PSIL_EP_NATIVE,
                ..Default::default()
            },
        }
    };
}

/* PSI-L source thread IDs, used for RX (DMA_DEV_TO_MEM) */
static mut am62a_src_ep_map: [psil_ep; 59] = [
    psil_saul!(0x7504, 20, 35, 8, 35, 0), psil_saul!(0x7505, 21, 35, 8, 36, 0),
    psil_saul!(0x7506, 22, 43, 8, 43, 0), psil_saul!(0x7507, 23, 43, 8, 44, 0),
    psil_pdma_xy_pkt!(0x4300), psil_pdma_xy_pkt!(0x4301), psil_pdma_xy_pkt!(0x4302),
    psil_pdma_xy_pkt!(0x4303), psil_pdma_xy_pkt!(0x4304), psil_pdma_xy_pkt!(0x4305),
    psil_pdma_xy_pkt!(0x4306), psil_pdma_xy_pkt!(0x4307), psil_pdma_xy_pkt!(0x4308),
    psil_pdma_xy_pkt!(0x4309), psil_pdma_xy_pkt!(0x430a), psil_pdma_xy_pkt!(0x430b),
    psil_pdma_xy_pkt!(0x4400), psil_pdma_xy_pkt!(0x4401), psil_pdma_xy_pkt!(0x4402),
    psil_pdma_xy_pkt!(0x4403), psil_pdma_xy_pkt!(0x4404), psil_pdma_xy_pkt!(0x4405),
    psil_pdma_xy_pkt!(0x4406), psil_pdma_mcasp!(0x4500), psil_pdma_mcasp!(0x4501),
    psil_pdma_mcasp!(0x4502), psil_ethernet!(0x4600, 19, 19, 16),
    psil_csi2rx!(0x5000), psil_csi2rx!(0x5001), psil_csi2rx!(0x5002), psil_csi2rx!(0x5003),
    psil_csi2rx!(0x5004), psil_csi2rx!(0x5005), psil_csi2rx!(0x5006), psil_csi2rx!(0x5007),
    psil_csi2rx!(0x5008), psil_csi2rx!(0x5009), psil_csi2rx!(0x500a), psil_csi2rx!(0x500b),
    psil_csi2rx!(0x500c), psil_csi2rx!(0x500d), psil_csi2rx!(0x500e), psil_csi2rx!(0x500f),
    psil_csi2rx!(0x5010), psil_csi2rx!(0x5011), psil_csi2rx!(0x5012), psil_csi2rx!(0x5013),
    psil_csi2rx!(0x5014), psil_csi2rx!(0x5015), psil_csi2rx!(0x5016), psil_csi2rx!(0x5017),
    psil_csi2rx!(0x5018), psil_csi2rx!(0x5019), psil_csi2rx!(0x501a), psil_csi2rx!(0x501b),
    psil_csi2rx!(0x501c), psil_csi2rx!(0x501d), psil_csi2rx!(0x501e), psil_csi2rx!(0x501f),
];

/* PSI-L destination thread IDs, used for TX (DMA_MEM_TO_DEV) */
static mut am62a_dst_ep_map: [psil_ep; 32] = [
    psil_saul!(0xf500, 27, 83, 8, 83, 1), psil_saul!(0xf501, 28, 91, 8, 91, 1),
    psil_pdma_xy_pkt!(0xc300), psil_pdma_xy_pkt!(0xc301), psil_pdma_xy_pkt!(0xc302),
    psil_pdma_xy_pkt!(0xc303), psil_pdma_xy_pkt!(0xc304), psil_pdma_xy_pkt!(0xc305),
    psil_pdma_xy_pkt!(0xc306), psil_pdma_xy_pkt!(0xc307), psil_pdma_xy_pkt!(0xc308),
    psil_pdma_xy_pkt!(0xc309), psil_pdma_xy_pkt!(0xc30a), psil_pdma_xy_pkt!(0xc30b),
    psil_pdma_xy_pkt!(0xc400), psil_pdma_xy_pkt!(0xc401), psil_pdma_xy_pkt!(0xc402),
    psil_pdma_xy_pkt!(0xc403), psil_pdma_xy_pkt!(0xc404), psil_pdma_xy_pkt!(0xc405),
    psil_pdma_xy_pkt!(0xc406), psil_pdma_mcasp!(0xc500), psil_pdma_mcasp!(0xc501),
    psil_pdma_mcasp!(0xc502), psil_ethernet!(0xc600, 19, 19, 8),
    psil_ethernet!(0xc601, 20, 27, 8), psil_ethernet!(0xc602, 21, 35, 8),
    psil_ethernet!(0xc603, 22, 43, 8), psil_ethernet!(0xc604, 23, 51, 8),
    psil_ethernet!(0xc605, 24, 59, 8), psil_ethernet!(0xc606, 25, 67, 8),
    psil_ethernet!(0xc607, 26, 75, 8),
];

static mut am62a_ep_map: psil_ep_map = psil_ep_map {
    name: "am62a",
    src: am62a_src_ep_map.as_ptr(),
    src_count: am62a_src_ep_map.len(),
    dst: am62a_dst_ep_map.as_ptr(),
    dst_count: am62a_dst_ep_map.len(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
