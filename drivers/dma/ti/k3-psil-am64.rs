// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 2020 Texas Instruments Incorporated - https://www.ti.com
 *  Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

// Definitions below are supplied by the corresponding PSIL private header.

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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
        }
    };
}

/* PSI-L source thread IDs, used for RX (DMA_DEV_TO_MEM) */
static mut am64_src_ep_map: [psil_ep; 42] = [
    /* SAUL */
    psil_saul!(0x4000, 17, 32, 8, 32, 0),
    psil_saul!(0x4001, 18, 32, 8, 33, 0),
    psil_saul!(0x4002, 19, 40, 8, 40, 0),
    psil_saul!(0x4003, 20, 40, 8, 41, 0),
    /* ICSS_G0 */
    psil_ethernet!(0x4100, 21, 48, 16), psil_ethernet!(0x4101, 22, 64, 16),
    psil_ethernet!(0x4102, 23, 80, 16), psil_ethernet!(0x4103, 24, 96, 16),
    /* ICSS_G1 */
    psil_ethernet!(0x4200, 25, 112, 16), psil_ethernet!(0x4201, 26, 128, 16),
    psil_ethernet!(0x4202, 27, 144, 16), psil_ethernet!(0x4203, 28, 160, 16),
    /* PDMA_MAIN0 - SPI0-3 */
    psil_pdma_xy_pkt!(0x4300), psil_pdma_xy_pkt!(0x4301), psil_pdma_xy_pkt!(0x4302), psil_pdma_xy_pkt!(0x4303),
    psil_pdma_xy_pkt!(0x4304), psil_pdma_xy_pkt!(0x4305), psil_pdma_xy_pkt!(0x4306), psil_pdma_xy_pkt!(0x4307),
    psil_pdma_xy_pkt!(0x4308), psil_pdma_xy_pkt!(0x4309), psil_pdma_xy_pkt!(0x430a), psil_pdma_xy_pkt!(0x430b),
    psil_pdma_xy_pkt!(0x430c), psil_pdma_xy_pkt!(0x430d), psil_pdma_xy_pkt!(0x430e), psil_pdma_xy_pkt!(0x430f),
    /* PDMA_MAIN0 - USART0-1 */
    psil_pdma_xy_pkt!(0x4310), psil_pdma_xy_pkt!(0x4311),
    /* PDMA_MAIN1 - SPI4 */
    psil_pdma_xy_pkt!(0x4400), psil_pdma_xy_pkt!(0x4401), psil_pdma_xy_pkt!(0x4402), psil_pdma_xy_pkt!(0x4403),
    /* PDMA_MAIN1 - USART2-6 */
    psil_pdma_xy_pkt!(0x4404), psil_pdma_xy_pkt!(0x4405), psil_pdma_xy_pkt!(0x4406), psil_pdma_xy_pkt!(0x4407), psil_pdma_xy_pkt!(0x4408),
    /* PDMA_MAIN1 - ADCs */
    psil_pdma_xy_tr!(0x440f), psil_pdma_xy_tr!(0x4410),
    /* CPSW2 */
    psil_ethernet!(0x4500, 16, 16, 16),
];

/* PSI-L destination thread IDs, used for TX (DMA_MEM_TO_DEV) */
static mut am64_dst_ep_map: [psil_ep; 26] = [
    /* SAUL */
    psil_saul!(0xc000, 24, 80, 8, 80, 1), psil_saul!(0xc001, 25, 88, 8, 88, 1),
    /* ICSS_G0 */
    psil_ethernet!(0xc100, 26, 96, 1), psil_ethernet!(0xc101, 27, 97, 1), psil_ethernet!(0xc102, 28, 98, 1), psil_ethernet!(0xc103, 29, 99, 1),
    psil_ethernet!(0xc104, 30, 100, 1), psil_ethernet!(0xc105, 31, 101, 1), psil_ethernet!(0xc106, 32, 102, 1), psil_ethernet!(0xc107, 33, 103, 1),
    /* ICSS_G1 */
    psil_ethernet!(0xc200, 34, 104, 1), psil_ethernet!(0xc201, 35, 105, 1), psil_ethernet!(0xc202, 36, 106, 1), psil_ethernet!(0xc203, 37, 107, 1),
    psil_ethernet!(0xc204, 38, 108, 1), psil_ethernet!(0xc205, 39, 109, 1), psil_ethernet!(0xc206, 40, 110, 1), psil_ethernet!(0xc207, 41, 111, 1),
    /* CPSW2 */
    psil_ethernet!(0xc500, 16, 16, 8), psil_ethernet!(0xc501, 17, 24, 8), psil_ethernet!(0xc502, 18, 32, 8), psil_ethernet!(0xc503, 19, 40, 8),
    psil_ethernet!(0xc504, 20, 48, 8), psil_ethernet!(0xc505, 21, 56, 8), psil_ethernet!(0xc506, 22, 64, 8), psil_ethernet!(0xc507, 23, 72, 8),
];

static mut am64_ep_map: psil_ep_map = psil_ep_map {
    name: "am64",
    src: am64_src_ep_map.as_ptr(),
    src_count: am64_src_ep_map.len(),
    dst: am64_dst_ep_map.as_ptr(),
    dst_count: am64_dst_ep_map.len(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
