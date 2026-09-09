// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 2021 Texas Instruments Incorporated - https://www.ti.com
 */

// Linux kernel dependency: `psil_ep`, `psil_ep_map`, `PSIL_EP_PDMA_XY`,
// `PSIL_EP_NATIVE`, and `ARRAY_SIZE` are supplied by the translated headers.

macro_rules! psil_pdma_xy_tr {
    ($x:expr) => { psil_ep { thread_id: $x, ep_config: psil_ep_config { ep_type: PSIL_EP_PDMA_XY, ..Default::default() } } };
}
macro_rules! psil_pdma_xy_pkt {
    ($x:expr) => { psil_ep { thread_id: $x, ep_config: psil_ep_config { ep_type: PSIL_EP_PDMA_XY, pkt_mode: 1, ..Default::default() } } };
}
macro_rules! psil_pdma_mcasp {
    ($x:expr) => { psil_ep { thread_id: $x, ep_config: psil_ep_config { ep_type: PSIL_EP_PDMA_XY, pdma_acc32: 1, pdma_burst: 1, ..Default::default() } } };
}
macro_rules! psil_ethernet {
    ($x:expr) => { psil_ep { thread_id: $x, ep_config: psil_ep_config { ep_type: PSIL_EP_NATIVE, pkt_mode: 1, needs_epib: 1, psd_size: 16, ..Default::default() } } };
}
macro_rules! psil_sa2ul {
    ($x:expr, $tx:expr) => { psil_ep { thread_id: $x, ep_config: psil_ep_config { ep_type: PSIL_EP_NATIVE, pkt_mode: 1, needs_epib: 1, psd_size: 64, notdpkt: $tx, ..Default::default() } } };
}
macro_rules! psil_csi2rx {
    ($x:expr) => { psil_ep { thread_id: $x, ep_config: psil_ep_config { ep_type: PSIL_EP_NATIVE, ..Default::default() } } };
}

/* PSI-L source thread IDs, used for RX (DMA_DEV_TO_MEM) */
static mut j784s4_src_ep_map: [psil_ep; 139] = [
    psil_pdma_mcasp!(0x4400), psil_pdma_mcasp!(0x4401), psil_pdma_mcasp!(0x4402), psil_pdma_mcasp!(0x4403), psil_pdma_mcasp!(0x4404),
    psil_pdma_xy_pkt!(0x4600), psil_pdma_xy_pkt!(0x4601), psil_pdma_xy_pkt!(0x4602), psil_pdma_xy_pkt!(0x4603), psil_pdma_xy_pkt!(0x4604), psil_pdma_xy_pkt!(0x4605), psil_pdma_xy_pkt!(0x4606), psil_pdma_xy_pkt!(0x4607), psil_pdma_xy_pkt!(0x4608), psil_pdma_xy_pkt!(0x4609), psil_pdma_xy_pkt!(0x460a), psil_pdma_xy_pkt!(0x460b), psil_pdma_xy_pkt!(0x460c), psil_pdma_xy_pkt!(0x460d), psil_pdma_xy_pkt!(0x460e), psil_pdma_xy_pkt!(0x460f),
    psil_pdma_xy_pkt!(0x4620), psil_pdma_xy_pkt!(0x4621), psil_pdma_xy_pkt!(0x4622), psil_pdma_xy_pkt!(0x4623), psil_pdma_xy_pkt!(0x4624), psil_pdma_xy_pkt!(0x4625), psil_pdma_xy_pkt!(0x4626), psil_pdma_xy_pkt!(0x4627), psil_pdma_xy_pkt!(0x4628), psil_pdma_xy_pkt!(0x4629), psil_pdma_xy_pkt!(0x462a), psil_pdma_xy_pkt!(0x462b), psil_pdma_xy_pkt!(0x462c), psil_pdma_xy_pkt!(0x462d), psil_pdma_xy_pkt!(0x462e), psil_pdma_xy_pkt!(0x462f),
    psil_ethernet!(0x4640),
    psil_pdma_xy_pkt!(0x4700), psil_pdma_xy_pkt!(0x4701), psil_pdma_xy_pkt!(0x4702), psil_pdma_xy_pkt!(0x4703), psil_pdma_xy_pkt!(0x4704), psil_pdma_xy_pkt!(0x4705), psil_pdma_xy_pkt!(0x4706), psil_pdma_xy_pkt!(0x4707), psil_pdma_xy_pkt!(0x4708), psil_pdma_xy_pkt!(0x4709),
    psil_csi2rx!(0x4900), psil_csi2rx!(0x4901), psil_csi2rx!(0x4902), psil_csi2rx!(0x4903),
    psil_csi2rx!(0x4940), psil_csi2rx!(0x4941), psil_csi2rx!(0x4942), psil_csi2rx!(0x4943), psil_csi2rx!(0x4944), psil_csi2rx!(0x4945), psil_csi2rx!(0x4946), psil_csi2rx!(0x4947), psil_csi2rx!(0x4948), psil_csi2rx!(0x4949), psil_csi2rx!(0x494a), psil_csi2rx!(0x494b), psil_csi2rx!(0x494c), psil_csi2rx!(0x494d), psil_csi2rx!(0x494e), psil_csi2rx!(0x494f),
    psil_csi2rx!(0x4950), psil_csi2rx!(0x4951), psil_csi2rx!(0x4952), psil_csi2rx!(0x4953), psil_csi2rx!(0x4954), psil_csi2rx!(0x4955), psil_csi2rx!(0x4956), psil_csi2rx!(0x4957), psil_csi2rx!(0x4958), psil_csi2rx!(0x4959), psil_csi2rx!(0x495a), psil_csi2rx!(0x495b), psil_csi2rx!(0x495c), psil_csi2rx!(0x495d), psil_csi2rx!(0x495e), psil_csi2rx!(0x495f),
    psil_csi2rx!(0x4960), psil_csi2rx!(0x4961), psil_csi2rx!(0x4962), psil_csi2rx!(0x4963), psil_csi2rx!(0x4964), psil_csi2rx!(0x4965), psil_csi2rx!(0x4966), psil_csi2rx!(0x4967), psil_csi2rx!(0x4968), psil_csi2rx!(0x4969), psil_csi2rx!(0x496a), psil_csi2rx!(0x496b), psil_csi2rx!(0x496c), psil_csi2rx!(0x496d), psil_csi2rx!(0x496e), psil_csi2rx!(0x496f),
    psil_csi2rx!(0x4970), psil_csi2rx!(0x4971), psil_csi2rx!(0x4972), psil_csi2rx!(0x4973), psil_csi2rx!(0x4974), psil_csi2rx!(0x4975), psil_csi2rx!(0x4976), psil_csi2rx!(0x4977), psil_csi2rx!(0x4978), psil_csi2rx!(0x4979), psil_csi2rx!(0x497a), psil_csi2rx!(0x497b), psil_csi2rx!(0x497c), psil_csi2rx!(0x497d), psil_csi2rx!(0x497e), psil_csi2rx!(0x497f),
    psil_csi2rx!(0x4980), psil_csi2rx!(0x4981), psil_csi2rx!(0x4982), psil_csi2rx!(0x4983), psil_csi2rx!(0x4984), psil_csi2rx!(0x4985), psil_csi2rx!(0x4986), psil_csi2rx!(0x4987), psil_csi2rx!(0x4988), psil_csi2rx!(0x4989), psil_csi2rx!(0x498a), psil_csi2rx!(0x498b), psil_csi2rx!(0x498c), psil_csi2rx!(0x498d), psil_csi2rx!(0x498e), psil_csi2rx!(0x498f),
    psil_csi2rx!(0x4990), psil_csi2rx!(0x4991), psil_csi2rx!(0x4992), psil_csi2rx!(0x4993), psil_csi2rx!(0x4994), psil_csi2rx!(0x4995), psil_csi2rx!(0x4996), psil_csi2rx!(0x4997), psil_csi2rx!(0x4998), psil_csi2rx!(0x4999), psil_csi2rx!(0x499a), psil_csi2rx!(0x499b), psil_csi2rx!(0x499c), psil_csi2rx!(0x499d), psil_csi2rx!(0x499e), psil_csi2rx!(0x499f),
    psil_ethernet!(0x4a00),
    psil_sa2ul!(0x4a40, 0), psil_sa2ul!(0x4a41, 0), psil_sa2ul!(0x4a42, 0), psil_sa2ul!(0x4a43, 0),
    psil_ethernet!(0x7000),
    psil_pdma_xy_pkt!(0x7100), psil_pdma_xy_pkt!(0x7101), psil_pdma_xy_pkt!(0x7102), psil_pdma_xy_pkt!(0x7103),
    psil_pdma_xy_pkt!(0x7200), psil_pdma_xy_pkt!(0x7201), psil_pdma_xy_pkt!(0x7202), psil_pdma_xy_pkt!(0x7203), psil_pdma_xy_pkt!(0x7204), psil_pdma_xy_pkt!(0x7205), psil_pdma_xy_pkt!(0x7206), psil_pdma_xy_pkt!(0x7207),
    psil_pdma_xy_pkt!(0x7300),
    psil_pdma_xy_tr!(0x7400), psil_pdma_xy_tr!(0x7401), psil_pdma_xy_tr!(0x7402), psil_pdma_xy_tr!(0x7403),
    psil_sa2ul!(0x7500, 0), psil_sa2ul!(0x7501, 0), psil_sa2ul!(0x7502, 0), psil_sa2ul!(0x7503, 0),
]; 

/* PSI-L destination thread IDs, used for TX (DMA_MEM_TO_DEV) */
static mut j784s4_dst_ep_map: [psil_ep; 72] = [
    psil_ethernet!(0xc640), psil_ethernet!(0xc641), psil_ethernet!(0xc642), psil_ethernet!(0xc643), psil_ethernet!(0xc644), psil_ethernet!(0xc645), psil_ethernet!(0xc646), psil_ethernet!(0xc647),
    psil_ethernet!(0xca00), psil_ethernet!(0xca01), psil_ethernet!(0xca02), psil_ethernet!(0xca03), psil_ethernet!(0xca04), psil_ethernet!(0xca05), psil_ethernet!(0xca06), psil_ethernet!(0xca07),
    psil_sa2ul!(0xca40, 1), psil_sa2ul!(0xca41, 1),
    psil_pdma_xy_pkt!(0xc600), psil_pdma_xy_pkt!(0xc601), psil_pdma_xy_pkt!(0xc602), psil_pdma_xy_pkt!(0xc603), psil_pdma_xy_pkt!(0xc604), psil_pdma_xy_pkt!(0xc605), psil_pdma_xy_pkt!(0xc606), psil_pdma_xy_pkt!(0xc607), psil_pdma_xy_pkt!(0xc608), psil_pdma_xy_pkt!(0xc609), psil_pdma_xy_pkt!(0xc60a), psil_pdma_xy_pkt!(0xc60b), psil_pdma_xy_pkt!(0xc60c), psil_pdma_xy_pkt!(0xc60d), psil_pdma_xy_pkt!(0xc60e), psil_pdma_xy_pkt!(0xc60f),
    psil_pdma_xy_pkt!(0xc620), psil_pdma_xy_pkt!(0xc621), psil_pdma_xy_pkt!(0xc622), psil_pdma_xy_pkt!(0xc623), psil_pdma_xy_pkt!(0xc624), psil_pdma_xy_pkt!(0xc625), psil_pdma_xy_pkt!(0xc626), psil_pdma_xy_pkt!(0xc627), psil_pdma_xy_pkt!(0xc628), psil_pdma_xy_pkt!(0xc629), psil_pdma_xy_pkt!(0xc62a), psil_pdma_xy_pkt!(0xc62b), psil_pdma_xy_pkt!(0xc62c), psil_pdma_xy_pkt!(0xc62d), psil_pdma_xy_pkt!(0xc62e), psil_pdma_xy_pkt!(0xc62f),
    psil_ethernet!(0xf000), psil_ethernet!(0xf001), psil_ethernet!(0xf002), psil_ethernet!(0xf003), psil_ethernet!(0xf004), psil_ethernet!(0xf005), psil_ethernet!(0xf006), psil_ethernet!(0xf007),
    psil_pdma_xy_pkt!(0xf100), psil_pdma_xy_pkt!(0xf101), psil_pdma_xy_pkt!(0xf102), psil_pdma_xy_pkt!(0xf103),
    psil_pdma_xy_pkt!(0xf200), psil_pdma_xy_pkt!(0xf201), psil_pdma_xy_pkt!(0xf202), psil_pdma_xy_pkt!(0xf203), psil_pdma_xy_pkt!(0xf204), psil_pdma_xy_pkt!(0xf205), psil_pdma_xy_pkt!(0xf206), psil_pdma_xy_pkt!(0xf207),
    psil_sa2ul!(0xf500, 1), psil_sa2ul!(0xf501, 1),
];

static mut j784s4_ep_map: psil_ep_map = psil_ep_map {
    name: "j784s4",
    src: j784s4_src_ep_map.as_ptr(),
    src_count: j784s4_src_ep_map.len(),
    dst: j784s4_dst_ep_map.as_ptr(),
    dst_count: j784s4_dst_ep_map.len(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
