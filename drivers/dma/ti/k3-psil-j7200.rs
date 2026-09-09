// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 2019 Texas Instruments Incorporated - http://www.ti.com
 *  Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

// Dependency declarations are provided by k3-psil-priv.h in the original source.

macro_rules! psil_pdma_xy_tr {
    ($x:expr) => { psil_ep { thread_id: $x, ep_config: psil_ep_config { ep_type: PSIL_EP_PDMA_XY } } };
}
macro_rules! psil_pdma_xy_pkt {
    ($x:expr) => { psil_ep { thread_id: $x, ep_config: psil_ep_config { ep_type: PSIL_EP_PDMA_XY, pkt_mode: 1 } } };
}
macro_rules! psil_pdma_mcasp {
    ($x:expr) => { psil_ep { thread_id: $x, ep_config: psil_ep_config { ep_type: PSIL_EP_PDMA_XY, pdma_acc32: 1, pdma_burst: 1 } } };
}
macro_rules! psil_ethernet {
    ($x:expr) => { psil_ep { thread_id: $x, ep_config: psil_ep_config { ep_type: PSIL_EP_NATIVE, pkt_mode: 1, needs_epib: 1, psd_size: 16 } } };
}
macro_rules! psil_sa2ul {
    ($x:expr, $tx:expr) => { psil_ep { thread_id: $x, ep_config: psil_ep_config { ep_type: PSIL_EP_NATIVE, pkt_mode: 1, needs_epib: 1, psd_size: 64, notdpkt: $tx } } };
}

/* PSI-L source thread IDs, used for RX (DMA_DEV_TO_MEM) */
static mut j7200_src_ep_map: [psil_ep; 64] = [
    psil_pdma_mcasp!(0x4400), psil_pdma_mcasp!(0x4401), psil_pdma_mcasp!(0x4402),
    psil_pdma_xy_pkt!(0x4600), psil_pdma_xy_pkt!(0x4601), psil_pdma_xy_pkt!(0x4602), psil_pdma_xy_pkt!(0x4603), psil_pdma_xy_pkt!(0x4604), psil_pdma_xy_pkt!(0x4605), psil_pdma_xy_pkt!(0x4606), psil_pdma_xy_pkt!(0x4607), psil_pdma_xy_pkt!(0x4608), psil_pdma_xy_pkt!(0x4609), psil_pdma_xy_pkt!(0x460a), psil_pdma_xy_pkt!(0x460b), psil_pdma_xy_pkt!(0x460c), psil_pdma_xy_pkt!(0x460d), psil_pdma_xy_pkt!(0x460e), psil_pdma_xy_pkt!(0x460f),
    psil_pdma_xy_pkt!(0x4610), psil_pdma_xy_pkt!(0x4611), psil_pdma_xy_pkt!(0x4612), psil_pdma_xy_pkt!(0x4613), psil_pdma_xy_pkt!(0x4614), psil_pdma_xy_pkt!(0x4615), psil_pdma_xy_pkt!(0x4616), psil_pdma_xy_pkt!(0x4617), psil_pdma_xy_pkt!(0x4618), psil_pdma_xy_pkt!(0x4619), psil_pdma_xy_pkt!(0x461a), psil_pdma_xy_pkt!(0x461b), psil_pdma_xy_pkt!(0x461c), psil_pdma_xy_pkt!(0x461d), psil_pdma_xy_pkt!(0x461e), psil_pdma_xy_pkt!(0x461f),
    psil_pdma_xy_pkt!(0x4700), psil_pdma_xy_pkt!(0x4701), psil_pdma_xy_pkt!(0x4702), psil_pdma_xy_pkt!(0x4703), psil_pdma_xy_pkt!(0x4704), psil_pdma_xy_pkt!(0x4705), psil_pdma_xy_pkt!(0x4706), psil_pdma_xy_pkt!(0x4707), psil_pdma_xy_pkt!(0x4708), psil_pdma_xy_pkt!(0x4709),
    psil_ethernet!(0x4a00), psil_ethernet!(0x7000),
    psil_pdma_xy_pkt!(0x7100), psil_pdma_xy_pkt!(0x7101), psil_pdma_xy_pkt!(0x7102), psil_pdma_xy_pkt!(0x7103),
    psil_pdma_xy_pkt!(0x7200), psil_pdma_xy_pkt!(0x7201), psil_pdma_xy_pkt!(0x7202), psil_pdma_xy_pkt!(0x7203), psil_pdma_xy_pkt!(0x7204), psil_pdma_xy_pkt!(0x7205), psil_pdma_xy_pkt!(0x7206), psil_pdma_xy_pkt!(0x7207), psil_pdma_xy_pkt!(0x7300),
    psil_pdma_xy_tr!(0x7400), psil_pdma_xy_tr!(0x7401), psil_sa2ul!(0x7500, 0), psil_sa2ul!(0x7501, 0), psil_sa2ul!(0x7502, 0), psil_sa2ul!(0x7503, 0),
];

/* PSI-L destination thread IDs, used for TX (DMA_MEM_TO_DEV) */
static mut j7200_dst_ep_map: [psil_ep; 74] = [
    psil_pdma_mcasp!(0xc400), psil_pdma_mcasp!(0xc401), psil_pdma_mcasp!(0xc402),
    psil_pdma_xy_pkt!(0xc600), psil_pdma_xy_pkt!(0xc601), psil_pdma_xy_pkt!(0xc602), psil_pdma_xy_pkt!(0xc603), psil_pdma_xy_pkt!(0xc604), psil_pdma_xy_pkt!(0xc605), psil_pdma_xy_pkt!(0xc606), psil_pdma_xy_pkt!(0xc607), psil_pdma_xy_pkt!(0xc608), psil_pdma_xy_pkt!(0xc609), psil_pdma_xy_pkt!(0xc60a), psil_pdma_xy_pkt!(0xc60b), psil_pdma_xy_pkt!(0xc60c), psil_pdma_xy_pkt!(0xc60d), psil_pdma_xy_pkt!(0xc60e), psil_pdma_xy_pkt!(0xc60f),
    psil_pdma_xy_pkt!(0xc610), psil_pdma_xy_pkt!(0xc611), psil_pdma_xy_pkt!(0xc612), psil_pdma_xy_pkt!(0xc613), psil_pdma_xy_pkt!(0xc614), psil_pdma_xy_pkt!(0xc615), psil_pdma_xy_pkt!(0xc616), psil_pdma_xy_pkt!(0xc617), psil_pdma_xy_pkt!(0xc618), psil_pdma_xy_pkt!(0xc619), psil_pdma_xy_pkt!(0xc61a), psil_pdma_xy_pkt!(0xc61b), psil_pdma_xy_pkt!(0xc61c), psil_pdma_xy_pkt!(0xc61d), psil_pdma_xy_pkt!(0xc61e), psil_pdma_xy_pkt!(0xc61f),
    psil_pdma_xy_pkt!(0xc700), psil_pdma_xy_pkt!(0xc701), psil_pdma_xy_pkt!(0xc702), psil_pdma_xy_pkt!(0xc703), psil_pdma_xy_pkt!(0xc704), psil_pdma_xy_pkt!(0xc705), psil_pdma_xy_pkt!(0xc706), psil_pdma_xy_pkt!(0xc707), psil_pdma_xy_pkt!(0xc708), psil_pdma_xy_pkt!(0xc709),
    psil_ethernet!(0xca00), psil_ethernet!(0xca01), psil_ethernet!(0xca02), psil_ethernet!(0xca03), psil_ethernet!(0xca04), psil_ethernet!(0xca05), psil_ethernet!(0xca06), psil_ethernet!(0xca07),
    psil_ethernet!(0xf000), psil_ethernet!(0xf001), psil_ethernet!(0xf002), psil_ethernet!(0xf003), psil_ethernet!(0xf004), psil_ethernet!(0xf005), psil_ethernet!(0xf006), psil_ethernet!(0xf007),
    psil_pdma_xy_pkt!(0xf100), psil_pdma_xy_pkt!(0xf101), psil_pdma_xy_pkt!(0xf102), psil_pdma_xy_pkt!(0xf103),
    psil_pdma_xy_pkt!(0xf200), psil_pdma_xy_pkt!(0xf201), psil_pdma_xy_pkt!(0xf202), psil_pdma_xy_pkt!(0xf203), psil_pdma_xy_pkt!(0xf204), psil_pdma_xy_pkt!(0xf205), psil_pdma_xy_pkt!(0xf206), psil_pdma_xy_pkt!(0xf207), psil_pdma_xy_pkt!(0xf300),
    psil_sa2ul!(0xf500, 1), psil_sa2ul!(0xf501, 1),
];

static mut j7200_ep_map: psil_ep_map = psil_ep_map {
    name: "j7200",
    src: j7200_src_ep_map.as_ptr() as *mut psil_ep,
    src_count: j7200_src_ep_map.len(),
    dst: j7200_dst_ep_map.as_ptr() as *mut psil_ep,
    dst_count: j7200_dst_ep_map.len(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
