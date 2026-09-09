// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 2019 Texas Instruments Incorporated - http://www.ti.com
 *  Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

// Definitions supplied by the corresponding kernel headers are external dependencies.

macro_rules! psil_pdma_xy_tr {
    ($x:expr) => {
        psil_ep {
            thread_id: $x,
            ep_config: psil_ep_config {
                ep_type: PSIL_EP_PDMA_XY,
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
                pkt_mode: 1,
                ..Default::default()
            },
            ..Default::default()
        }
    };
}

macro_rules! psil_ethernet {
    ($x:expr) => {
        psil_ep {
            thread_id: $x,
            ep_config: psil_ep_config {
                ep_type: PSIL_EP_NATIVE,
                pkt_mode: 1,
                needs_epib: 1,
                psd_size: 16,
                ..Default::default()
            },
            ..Default::default()
        }
    };
}

macro_rules! psil_sa2ul {
    ($x:expr, $tx:expr) => {
        psil_ep {
            thread_id: $x,
            ep_config: psil_ep_config {
                ep_type: PSIL_EP_NATIVE,
                pkt_mode: 1,
                needs_epib: 1,
                psd_size: 64,
                notdpkt: $tx,
                ..Default::default()
            },
            ..Default::default()
        }
    };
}

/* PSI-L source thread IDs, used for RX (DMA_DEV_TO_MEM) */
static mut am654_src_ep_map: [psil_ep; 60] = [
    psil_sa2ul!(0x4000, 0), psil_sa2ul!(0x4001, 0), psil_sa2ul!(0x4002, 0), psil_sa2ul!(0x4003, 0),
    psil_ethernet!(0x4100), psil_ethernet!(0x4101), psil_ethernet!(0x4102), psil_ethernet!(0x4103),
    psil_ethernet!(0x4200), psil_ethernet!(0x4201), psil_ethernet!(0x4202), psil_ethernet!(0x4203),
    psil_ethernet!(0x4300), psil_ethernet!(0x4301), psil_ethernet!(0x4302), psil_ethernet!(0x4303),
    psil_pdma_xy_tr!(0x4400), psil_pdma_xy_tr!(0x4401), psil_pdma_xy_tr!(0x4402),
    psil_pdma_xy_pkt!(0x4500), psil_pdma_xy_pkt!(0x4501), psil_pdma_xy_pkt!(0x4502), psil_pdma_xy_pkt!(0x4503),
    psil_pdma_xy_pkt!(0x4504), psil_pdma_xy_pkt!(0x4505), psil_pdma_xy_pkt!(0x4506), psil_pdma_xy_pkt!(0x4507),
    psil_pdma_xy_pkt!(0x4508), psil_pdma_xy_pkt!(0x4509), psil_pdma_xy_pkt!(0x450a), psil_pdma_xy_pkt!(0x450b),
    psil_pdma_xy_pkt!(0x450c), psil_pdma_xy_pkt!(0x450d), psil_pdma_xy_pkt!(0x450e), psil_pdma_xy_pkt!(0x450f),
    psil_pdma_xy_pkt!(0x4510), psil_pdma_xy_pkt!(0x4511), psil_pdma_xy_pkt!(0x4512), psil_pdma_xy_pkt!(0x4513),
    psil_pdma_xy_pkt!(0x4514), psil_pdma_xy_pkt!(0x4515), psil_pdma_xy_pkt!(0x4516),
    psil_ethernet!(0x7000),
    psil_pdma_xy_tr!(0x7100), psil_pdma_xy_tr!(0x7101), psil_pdma_xy_tr!(0x7102), psil_pdma_xy_tr!(0x7103),
    psil_pdma_xy_pkt!(0x7200), psil_pdma_xy_pkt!(0x7201), psil_pdma_xy_pkt!(0x7202), psil_pdma_xy_pkt!(0x7203),
    psil_pdma_xy_pkt!(0x7204), psil_pdma_xy_pkt!(0x7205), psil_pdma_xy_pkt!(0x7206), psil_pdma_xy_pkt!(0x7207),
    psil_pdma_xy_pkt!(0x7208), psil_pdma_xy_pkt!(0x7209), psil_pdma_xy_pkt!(0x720a), psil_pdma_xy_pkt!(0x720b),
    psil_pdma_xy_pkt!(0x7212),
];

/* PSI-L destination thread IDs, used for TX (DMA_MEM_TO_DEV) */
static mut am654_dst_ep_map: [psil_ep; 34] = [
    psil_sa2ul!(0xc000, 1), psil_sa2ul!(0xc001, 1),
    psil_ethernet!(0xc100), psil_ethernet!(0xc101), psil_ethernet!(0xc102), psil_ethernet!(0xc103),
    psil_ethernet!(0xc104), psil_ethernet!(0xc105), psil_ethernet!(0xc106), psil_ethernet!(0xc107),
    psil_ethernet!(0xc200), psil_ethernet!(0xc201), psil_ethernet!(0xc202), psil_ethernet!(0xc203),
    psil_ethernet!(0xc204), psil_ethernet!(0xc205), psil_ethernet!(0xc206), psil_ethernet!(0xc207),
    psil_ethernet!(0xc300), psil_ethernet!(0xc301), psil_ethernet!(0xc302), psil_ethernet!(0xc303),
    psil_ethernet!(0xc304), psil_ethernet!(0xc305), psil_ethernet!(0xc306), psil_ethernet!(0xc307),
    psil_ethernet!(0xf000), psil_ethernet!(0xf001), psil_ethernet!(0xf002), psil_ethernet!(0xf003),
    psil_ethernet!(0xf004), psil_ethernet!(0xf005), psil_ethernet!(0xf006), psil_ethernet!(0xf007),
];

static mut am654_ep_map: psil_ep_map = psil_ep_map {
    name: "am654",
    src: am654_src_ep_map.as_ptr(),
    src_count: am654_src_ep_map.len(),
    dst: am654_dst_ep_map.as_ptr(),
    dst_count: am654_dst_ep_map.len(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
