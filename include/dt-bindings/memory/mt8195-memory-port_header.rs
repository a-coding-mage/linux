// Faithful Rust translation of mt8195-memory-port.h.
// The MTK_M4U_ID and MTK_IFAIOMMU_PERI_ID macros are supplied by the dependency header.
/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Yong Wu <yong.wu@mediatek.com>
 */
// Header guard preserved as a comment.


// Dependency: <dt-bindings/memory/mtk-memory-port.h>

/*
 * MM IOMMU supports 16GB dma address. We separate it to four ranges:
 * 0 ~ 4G; 4G ~ 8G; 8G ~ 12G; 12G ~ 16G, we could adjust these masters
 * locate in anyone region. BUT:
 * a) Make sure all the ports inside a larb are in one range.
 * b) The iova of any master can NOT cross the 4G/8G/12G boundary.
 *
 * This is the suggested mapping in this SoC:
 *
 * modules    dma-address-region	larbs-ports
 * disp         0 ~ 4G                  larb0/1/2/3
 * vcodec      4G ~ 8G                  larb19/20/21/22/23/24
 * cam/mdp     8G ~ 12G                 the other larbs.
 * N/A         12G ~ 16G
 * CCU0   0x24000_0000 ~ 0x243ff_ffff   larb18: port 0/1
 * CCU1   0x24400_0000 ~ 0x247ff_ffff   larb18: port 2/3
 *
 * This SoC have two IOMMU HWs, this is the detailed connected information:
 * iommu-vdo: larb0/2/5/7/9/10/11/13/17/19/21/24/25/28
 * iommu-vpp: larb1/3/4/6/8/12/14/16/18/20/22/23/26/27
 */

/* MM IOMMU ports */
/* larb0 */
pub const M4U_PORT_L0_DISP_RDMA0: u32 = mtk_m4u_id!(0, 0);
pub const M4U_PORT_L0_DISP_WDMA0: u32 = mtk_m4u_id!(0, 1);
pub const M4U_PORT_L0_DISP_OVL0_RDMA0: u32 = mtk_m4u_id!(0, 2);
pub const M4U_PORT_L0_DISP_OVL0_RDMA1: u32 = mtk_m4u_id!(0, 3);
pub const M4U_PORT_L0_DISP_OVL0_HDR: u32 = mtk_m4u_id!(0, 4);
pub const M4U_PORT_L0_DISP_FAKE0: u32 = mtk_m4u_id!(0, 5);

/* larb1 */
pub const M4U_PORT_L1_DISP_RDMA0: u32 = mtk_m4u_id!(1, 0);
pub const M4U_PORT_L1_DISP_WDMA0: u32 = mtk_m4u_id!(1, 1);
pub const M4U_PORT_L1_DISP_OVL0_RDMA0: u32 = mtk_m4u_id!(1, 2);
pub const M4U_PORT_L1_DISP_OVL0_RDMA1: u32 = mtk_m4u_id!(1, 3);
pub const M4U_PORT_L1_DISP_OVL0_HDR: u32 = mtk_m4u_id!(1, 4);
pub const M4U_PORT_L1_DISP_FAKE0: u32 = mtk_m4u_id!(1, 5);

/* larb2 */
pub const M4U_PORT_L2_MDP_RDMA0: u32 = mtk_m4u_id!(2, 0);
pub const M4U_PORT_L2_MDP_RDMA2: u32 = mtk_m4u_id!(2, 1);
pub const M4U_PORT_L2_MDP_RDMA4: u32 = mtk_m4u_id!(2, 2);
pub const M4U_PORT_L2_MDP_RDMA6: u32 = mtk_m4u_id!(2, 3);
pub const M4U_PORT_L2_DISP_FAKE1: u32 = mtk_m4u_id!(2, 4);

/* larb3 */
pub const M4U_PORT_L3_MDP_RDMA1: u32 = mtk_m4u_id!(3, 0);
pub const M4U_PORT_L3_MDP_RDMA3: u32 = mtk_m4u_id!(3, 1);
pub const M4U_PORT_L3_MDP_RDMA5: u32 = mtk_m4u_id!(3, 2);
pub const M4U_PORT_L3_MDP_RDMA7: u32 = mtk_m4u_id!(3, 3);
pub const M4U_PORT_L3_HDR_DS: u32 = mtk_m4u_id!(3, 4);
pub const M4U_PORT_L3_HDR_ADL: u32 = mtk_m4u_id!(3, 5);
pub const M4U_PORT_L3_DISP_FAKE1: u32 = mtk_m4u_id!(3, 6);

/* larb4 */
pub const M4U_PORT_L4_MDP_RDMA: u32 = mtk_m4u_id!(4, 0);
pub const M4U_PORT_L4_MDP_FG: u32 = mtk_m4u_id!(4, 1);
pub const M4U_PORT_L4_MDP_OVL: u32 = mtk_m4u_id!(4, 2);
pub const M4U_PORT_L4_MDP_WROT: u32 = mtk_m4u_id!(4, 3);
pub const M4U_PORT_L4_FAKE: u32 = mtk_m4u_id!(4, 4);

/* larb5 */
pub const M4U_PORT_L5_SVPP1_MDP_RDMA: u32 = mtk_m4u_id!(5, 0);
pub const M4U_PORT_L5_SVPP1_MDP_FG: u32 = mtk_m4u_id!(5, 1);
pub const M4U_PORT_L5_SVPP1_MDP_OVL: u32 = mtk_m4u_id!(5, 2);
pub const M4U_PORT_L5_SVPP1_MDP_WROT: u32 = mtk_m4u_id!(5, 3);
pub const M4U_PORT_L5_SVPP2_MDP_RDMA: u32 = mtk_m4u_id!(5, 4);
pub const M4U_PORT_L5_SVPP2_MDP_FG: u32 = mtk_m4u_id!(5, 5);
pub const M4U_PORT_L5_SVPP2_MDP_WROT: u32 = mtk_m4u_id!(5, 6);
pub const M4U_PORT_L5_FAKE: u32 = mtk_m4u_id!(5, 7);

/* larb6 */
pub const M4U_PORT_L6_SVPP3_MDP_RDMA: u32 = mtk_m4u_id!(6, 0);
pub const M4U_PORT_L6_SVPP3_MDP_FG: u32 = mtk_m4u_id!(6, 1);
pub const M4U_PORT_L6_SVPP3_MDP_WROT: u32 = mtk_m4u_id!(6, 2);
pub const M4U_PORT_L6_FAKE: u32 = mtk_m4u_id!(6, 3);

/* larb7 */
pub const M4U_PORT_L7_IMG_WPE_RDMA0: u32 = mtk_m4u_id!(7, 0);
pub const M4U_PORT_L7_IMG_WPE_RDMA1: u32 = mtk_m4u_id!(7, 1);
pub const M4U_PORT_L7_IMG_WPE_WDMA0: u32 = mtk_m4u_id!(7, 2);

/* larb8 */
pub const M4U_PORT_L8_IMG_WPE_RDMA0: u32 = mtk_m4u_id!(8, 0);
pub const M4U_PORT_L8_IMG_WPE_RDMA1: u32 = mtk_m4u_id!(8, 1);
pub const M4U_PORT_L8_IMG_WPE_WDMA0: u32 = mtk_m4u_id!(8, 2);

/* larb9 */
pub const M4U_PORT_L9_IMG_IMGI_T1_A: u32 = mtk_m4u_id!(9, 0);
pub const M4U_PORT_L9_IMG_IMGBI_T1_A: u32 = mtk_m4u_id!(9, 1);
pub const M4U_PORT_L9_IMG_IMGCI_T1_A: u32 = mtk_m4u_id!(9, 2);
pub const M4U_PORT_L9_IMG_SMTI_T1_A: u32 = mtk_m4u_id!(9, 3);
pub const M4U_PORT_L9_IMG_TNCSTI_T1_A: u32 = mtk_m4u_id!(9, 4);
pub const M4U_PORT_L9_IMG_TNCSTI_T4_A: u32 = mtk_m4u_id!(9, 5);
pub const M4U_PORT_L9_IMG_YUVO_T1_A: u32 = mtk_m4u_id!(9, 6);
pub const M4U_PORT_L9_IMG_TIMGO_T1_A: u32 = mtk_m4u_id!(9, 7);
pub const M4U_PORT_L9_IMG_YUVO_T2_A: u32 = mtk_m4u_id!(9, 8);
pub const M4U_PORT_L9_IMG_IMGI_T1_B: u32 = mtk_m4u_id!(9, 9);
pub const M4U_PORT_L9_IMG_IMGBI_T1_B: u32 = mtk_m4u_id!(9, 10);
pub const M4U_PORT_L9_IMG_IMGCI_T1_B: u32 = mtk_m4u_id!(9, 11);
pub const M4U_PORT_L9_IMG_YUVO_T5_A: u32 = mtk_m4u_id!(9, 12);
pub const M4U_PORT_L9_IMG_SMTI_T1_B: u32 = mtk_m4u_id!(9, 13);
pub const M4U_PORT_L9_IMG_TNCSO_T1_A: u32 = mtk_m4u_id!(9, 14);
pub const M4U_PORT_L9_IMG_SMTO_T1_A: u32 = mtk_m4u_id!(9, 15);
pub const M4U_PORT_L9_IMG_TNCSTO_T1_A: u32 = mtk_m4u_id!(9, 16);
pub const M4U_PORT_L9_IMG_YUVO_T2_B: u32 = mtk_m4u_id!(9, 17);
pub const M4U_PORT_L9_IMG_YUVO_T5_B: u32 = mtk_m4u_id!(9, 18);
pub const M4U_PORT_L9_IMG_SMTO_T1_B: u32 = mtk_m4u_id!(9, 19);

/* larb10 */
pub const M4U_PORT_L10_IMG_IMGI_D1_A: u32 = mtk_m4u_id!(10, 0);
pub const M4U_PORT_L10_IMG_IMGCI_D1_A: u32 = mtk_m4u_id!(10, 1);
pub const M4U_PORT_L10_IMG_DEPI_D1_A: u32 = mtk_m4u_id!(10, 2);
pub const M4U_PORT_L10_IMG_DMGI_D1_A: u32 = mtk_m4u_id!(10, 3);
pub const M4U_PORT_L10_IMG_VIPI_D1_A: u32 = mtk_m4u_id!(10, 4);
pub const M4U_PORT_L10_IMG_TNRWI_D1_A: u32 = mtk_m4u_id!(10, 5);
pub const M4U_PORT_L10_IMG_RECI_D1_A: u32 = mtk_m4u_id!(10, 6);
pub const M4U_PORT_L10_IMG_SMTI_D1_A: u32 = mtk_m4u_id!(10, 7);
pub const M4U_PORT_L10_IMG_SMTI_D6_A: u32 = mtk_m4u_id!(10, 8);
pub const M4U_PORT_L10_IMG_PIMGI_P1_A: u32 = mtk_m4u_id!(10, 9);
pub const M4U_PORT_L10_IMG_PIMGBI_P1_A: u32 = mtk_m4u_id!(10, 10);
pub const M4U_PORT_L10_IMG_PIMGCI_P1_A: u32 = mtk_m4u_id!(10, 11);
pub const M4U_PORT_L10_IMG_PIMGI_P1_B: u32 = mtk_m4u_id!(10, 12);
pub const M4U_PORT_L10_IMG_PIMGBI_P1_B: u32 = mtk_m4u_id!(10, 13);
pub const M4U_PORT_L10_IMG_PIMGCI_P1_B: u32 = mtk_m4u_id!(10, 14);
pub const M4U_PORT_L10_IMG_IMG3O_D1_A: u32 = mtk_m4u_id!(10, 15);
pub const M4U_PORT_L10_IMG_IMG4O_D1_A: u32 = mtk_m4u_id!(10, 16);
pub const M4U_PORT_L10_IMG_IMG3CO_D1_A: u32 = mtk_m4u_id!(10, 17);
pub const M4U_PORT_L10_IMG_FEO_D1_A: u32 = mtk_m4u_id!(10, 18);
pub const M4U_PORT_L10_IMG_IMG2O_D1_A: u32 = mtk_m4u_id!(10, 19);
pub const M4U_PORT_L10_IMG_TNRWO_D1_A: u32 = mtk_m4u_id!(10, 20);
pub const M4U_PORT_L10_IMG_SMTO_D1_A: u32 = mtk_m4u_id!(10, 21);
pub const M4U_PORT_L10_IMG_WROT_P1_A: u32 = mtk_m4u_id!(10, 22);
pub const M4U_PORT_L10_IMG_WROT_P1_B: u32 = mtk_m4u_id!(10, 23);

/* larb11 */
pub const M4U_PORT_L11_IMG_WPE_EIS_RDMA0_A: u32 = mtk_m4u_id!(11, 0);
pub const M4U_PORT_L11_IMG_WPE_EIS_RDMA1_A: u32 = mtk_m4u_id!(11, 1);
pub const M4U_PORT_L11_IMG_WPE_EIS_WDMA0_A: u32 = mtk_m4u_id!(11, 2);
pub const M4U_PORT_L11_IMG_WPE_TNR_RDMA0_A: u32 = mtk_m4u_id!(11, 3);
pub const M4U_PORT_L11_IMG_WPE_TNR_RDMA1_A: u32 = mtk_m4u_id!(11, 4);
pub const M4U_PORT_L11_IMG_WPE_TNR_WDMA0_A: u32 = mtk_m4u_id!(11, 5);
pub const M4U_PORT_L11_IMG_WPE_EIS_CQ0_A: u32 = mtk_m4u_id!(11, 6);
pub const M4U_PORT_L11_IMG_WPE_EIS_CQ1_A: u32 = mtk_m4u_id!(11, 7);
pub const M4U_PORT_L11_IMG_WPE_TNR_CQ0_A: u32 = mtk_m4u_id!(11, 8);
pub const M4U_PORT_L11_IMG_WPE_TNR_CQ1_A: u32 = mtk_m4u_id!(11, 9);

/* larb12 */
pub const M4U_PORT_L12_IMG_FDVT_RDA: u32 = mtk_m4u_id!(12, 0);
pub const M4U_PORT_L12_IMG_FDVT_RDB: u32 = mtk_m4u_id!(12, 1);
pub const M4U_PORT_L12_IMG_FDVT_WRA: u32 = mtk_m4u_id!(12, 2);
pub const M4U_PORT_L12_IMG_FDVT_WRB: u32 = mtk_m4u_id!(12, 3);
pub const M4U_PORT_L12_IMG_ME_RDMA: u32 = mtk_m4u_id!(12, 4);
pub const M4U_PORT_L12_IMG_ME_WDMA: u32 = mtk_m4u_id!(12, 5);
pub const M4U_PORT_L12_IMG_DVS_RDMA: u32 = mtk_m4u_id!(12, 6);
pub const M4U_PORT_L12_IMG_DVS_WDMA: u32 = mtk_m4u_id!(12, 7);
pub const M4U_PORT_L12_IMG_DVP_RDMA: u32 = mtk_m4u_id!(12, 8);
pub const M4U_PORT_L12_IMG_DVP_WDMA: u32 = mtk_m4u_id!(12, 9);

/* larb13 */
pub const M4U_PORT_L13_CAM_CAMSV_CQI_E1: u32 = mtk_m4u_id!(13, 0);
pub const M4U_PORT_L13_CAM_CAMSV_CQI_E2: u32 = mtk_m4u_id!(13, 1);
pub const M4U_PORT_L13_CAM_GCAMSV_A_IMGO_0: u32 = mtk_m4u_id!(13, 2);
pub const M4U_PORT_L13_CAM_SCAMSV_A_IMGO_0: u32 = mtk_m4u_id!(13, 3);
pub const M4U_PORT_L13_CAM_GCAMSV_B_IMGO_0: u32 = mtk_m4u_id!(13, 4);
pub const M4U_PORT_L13_CAM_GCAMSV_B_IMGO_1: u32 = mtk_m4u_id!(13, 5);
pub const M4U_PORT_L13_CAM_GCAMSV_A_UFEO_0: u32 = mtk_m4u_id!(13, 6);
pub const M4U_PORT_L13_CAM_GCAMSV_B_UFEO_0: u32 = mtk_m4u_id!(13, 7);
pub const M4U_PORT_L13_CAM_PDAI_0: u32 = mtk_m4u_id!(13, 8);
pub const M4U_PORT_L13_CAM_FAKE: u32 = mtk_m4u_id!(13, 9);

/* larb14 */
pub const M4U_PORT_L14_CAM_GCAMSV_A_IMGO_1: u32 = mtk_m4u_id!(14, 0);
pub const M4U_PORT_L14_CAM_SCAMSV_A_IMGO_1: u32 = mtk_m4u_id!(14, 1);
pub const M4U_PORT_L14_CAM_GCAMSV_B_IMGO_0: u32 = mtk_m4u_id!(14, 2);
pub const M4U_PORT_L14_CAM_GCAMSV_B_IMGO_1: u32 = mtk_m4u_id!(14, 3);
pub const M4U_PORT_L14_CAM_SCAMSV_B_IMGO_0: u32 = mtk_m4u_id!(14, 4);
pub const M4U_PORT_L14_CAM_SCAMSV_B_IMGO_1: u32 = mtk_m4u_id!(14, 5);
pub const M4U_PORT_L14_CAM_IPUI: u32 = mtk_m4u_id!(14, 6);
pub const M4U_PORT_L14_CAM_IPU2I: u32 = mtk_m4u_id!(14, 7);
pub const M4U_PORT_L14_CAM_IPUO: u32 = mtk_m4u_id!(14, 8);
pub const M4U_PORT_L14_CAM_IPU2O: u32 = mtk_m4u_id!(14, 9);
pub const M4U_PORT_L14_CAM_IPU3O: u32 = mtk_m4u_id!(14, 10);
pub const M4U_PORT_L14_CAM_GCAMSV_A_UFEO_1: u32 = mtk_m4u_id!(14, 11);
pub const M4U_PORT_L14_CAM_GCAMSV_B_UFEO_1: u32 = mtk_m4u_id!(14, 12);
pub const M4U_PORT_L14_CAM_PDAI_1: u32 = mtk_m4u_id!(14, 13);
pub const M4U_PORT_L14_CAM_PDAO: u32 = mtk_m4u_id!(14, 14);

/* larb15: null */

/* larb16 */
pub const M4U_PORT_L16_CAM_IMGO_R1: u32 = mtk_m4u_id!(16, 0);
pub const M4U_PORT_L16_CAM_CQI_R1: u32 = mtk_m4u_id!(16, 1);
pub const M4U_PORT_L16_CAM_CQI_R2: u32 = mtk_m4u_id!(16, 2);
pub const M4U_PORT_L16_CAM_BPCI_R1: u32 = mtk_m4u_id!(16, 3);
pub const M4U_PORT_L16_CAM_LSCI_R1: u32 = mtk_m4u_id!(16, 4);
pub const M4U_PORT_L16_CAM_RAWI_R2: u32 = mtk_m4u_id!(16, 5);
pub const M4U_PORT_L16_CAM_RAWI_R3: u32 = mtk_m4u_id!(16, 6);
pub const M4U_PORT_L16_CAM_UFDI_R2: u32 = mtk_m4u_id!(16, 7);
pub const M4U_PORT_L16_CAM_UFDI_R3: u32 = mtk_m4u_id!(16, 8);
pub const M4U_PORT_L16_CAM_RAWI_R4: u32 = mtk_m4u_id!(16, 9);
pub const M4U_PORT_L16_CAM_RAWI_R5: u32 = mtk_m4u_id!(16, 10);
pub const M4U_PORT_L16_CAM_AAI_R1: u32 = mtk_m4u_id!(16, 11);
pub const M4U_PORT_L16_CAM_FHO_R1: u32 = mtk_m4u_id!(16, 12);
pub const M4U_PORT_L16_CAM_AAO_R1: u32 = mtk_m4u_id!(16, 13);
pub const M4U_PORT_L16_CAM_TSFSO_R1: u32 = mtk_m4u_id!(16, 14);
pub const M4U_PORT_L16_CAM_FLKO_R1: u32 = mtk_m4u_id!(16, 15);

/* larb17 */
pub const M4U_PORT_L17_CAM_YUVO_R1: u32 = mtk_m4u_id!(17, 0);
pub const M4U_PORT_L17_CAM_YUVO_R3: u32 = mtk_m4u_id!(17, 1);
pub const M4U_PORT_L17_CAM_YUVCO_R1: u32 = mtk_m4u_id!(17, 2);
pub const M4U_PORT_L17_CAM_YUVO_R2: u32 = mtk_m4u_id!(17, 3);
pub const M4U_PORT_L17_CAM_RZH1N2TO_R1: u32 = mtk_m4u_id!(17, 4);
pub const M4U_PORT_L17_CAM_DRZS4NO_R1: u32 = mtk_m4u_id!(17, 5);
pub const M4U_PORT_L17_CAM_TNCSO_R1: u32 = mtk_m4u_id!(17, 6);

/* larb18 */
pub const M4U_PORT_L18_CAM_CCUI: u32 = mtk_m4u_id!(18, 0);
pub const M4U_PORT_L18_CAM_CCUO: u32 = mtk_m4u_id!(18, 1);
pub const M4U_PORT_L18_CAM_CCUI2: u32 = mtk_m4u_id!(18, 2);
pub const M4U_PORT_L18_CAM_CCUO2: u32 = mtk_m4u_id!(18, 3);

/* larb19 */
pub const M4U_PORT_L19_VENC_RCPU: u32 = mtk_m4u_id!(19, 0);
pub const M4U_PORT_L19_VENC_REC: u32 = mtk_m4u_id!(19, 1);
pub const M4U_PORT_L19_VENC_BSDMA: u32 = mtk_m4u_id!(19, 2);
pub const M4U_PORT_L19_VENC_SV_COMV: u32 = mtk_m4u_id!(19, 3);
pub const M4U_PORT_L19_VENC_RD_COMV: u32 = mtk_m4u_id!(19, 4);
pub const M4U_PORT_L19_VENC_NBM_RDMA: u32 = mtk_m4u_id!(19, 5);
pub const M4U_PORT_L19_VENC_NBM_RDMA_LITE: u32 = mtk_m4u_id!(19, 6);
pub const M4U_PORT_L19_JPGENC_Y_RDMA: u32 = mtk_m4u_id!(19, 7);
pub const M4U_PORT_L19_JPGENC_C_RDMA: u32 = mtk_m4u_id!(19, 8);
pub const M4U_PORT_L19_JPGENC_Q_TABLE: u32 = mtk_m4u_id!(19, 9);
pub const M4U_PORT_L19_VENC_SUB_W_LUMA: u32 = mtk_m4u_id!(19, 10);
pub const M4U_PORT_L19_VENC_FCS_NBM_RDMA: u32 = mtk_m4u_id!(19, 11);
pub const M4U_PORT_L19_JPGENC_BSDMA: u32 = mtk_m4u_id!(19, 12);
pub const M4U_PORT_L19_JPGDEC_WDMA0: u32 = mtk_m4u_id!(19, 13);
pub const M4U_PORT_L19_JPGDEC_BSDMA0: u32 = mtk_m4u_id!(19, 14);
pub const M4U_PORT_L19_VENC_NBM_WDMA: u32 = mtk_m4u_id!(19, 15);
pub const M4U_PORT_L19_VENC_NBM_WDMA_LITE: u32 = mtk_m4u_id!(19, 16);
pub const M4U_PORT_L19_VENC_FCS_NBM_WDMA: u32 = mtk_m4u_id!(19, 17);
pub const M4U_PORT_L19_JPGDEC_WDMA1: u32 = mtk_m4u_id!(19, 18);
pub const M4U_PORT_L19_JPGDEC_BSDMA1: u32 = mtk_m4u_id!(19, 19);
pub const M4U_PORT_L19_JPGDEC_BUFF_OFFSET1: u32 = mtk_m4u_id!(19, 20);
pub const M4U_PORT_L19_JPGDEC_BUFF_OFFSET0: u32 = mtk_m4u_id!(19, 21);
pub const M4U_PORT_L19_VENC_CUR_LUMA: u32 = mtk_m4u_id!(19, 22);
pub const M4U_PORT_L19_VENC_CUR_CHROMA: u32 = mtk_m4u_id!(19, 23);
pub const M4U_PORT_L19_VENC_REF_LUMA: u32 = mtk_m4u_id!(19, 24);
pub const M4U_PORT_L19_VENC_REF_CHROMA: u32 = mtk_m4u_id!(19, 25);
pub const M4U_PORT_L19_VENC_SUB_R_CHROMA: u32 = mtk_m4u_id!(19, 26);

/* larb20 */
pub const M4U_PORT_L20_VENC_RCPU: u32 = mtk_m4u_id!(20, 0);
pub const M4U_PORT_L20_VENC_REC: u32 = mtk_m4u_id!(20, 1);
pub const M4U_PORT_L20_VENC_BSDMA: u32 = mtk_m4u_id!(20, 2);
pub const M4U_PORT_L20_VENC_SV_COMV: u32 = mtk_m4u_id!(20, 3);
pub const M4U_PORT_L20_VENC_RD_COMV: u32 = mtk_m4u_id!(20, 4);
pub const M4U_PORT_L20_VENC_NBM_RDMA: u32 = mtk_m4u_id!(20, 5);
pub const M4U_PORT_L20_VENC_NBM_RDMA_LITE: u32 = mtk_m4u_id!(20, 6);
pub const M4U_PORT_L20_JPGENC_Y_RDMA: u32 = mtk_m4u_id!(20, 7);
pub const M4U_PORT_L20_JPGENC_C_RDMA: u32 = mtk_m4u_id!(20, 8);
pub const M4U_PORT_L20_JPGENC_Q_TABLE: u32 = mtk_m4u_id!(20, 9);
pub const M4U_PORT_L20_VENC_SUB_W_LUMA: u32 = mtk_m4u_id!(20, 10);
pub const M4U_PORT_L20_VENC_FCS_NBM_RDMA: u32 = mtk_m4u_id!(20, 11);
pub const M4U_PORT_L20_JPGENC_BSDMA: u32 = mtk_m4u_id!(20, 12);
pub const M4U_PORT_L20_JPGDEC_WDMA0: u32 = mtk_m4u_id!(20, 13);
pub const M4U_PORT_L20_JPGDEC_BSDMA0: u32 = mtk_m4u_id!(20, 14);
pub const M4U_PORT_L20_VENC_NBM_WDMA: u32 = mtk_m4u_id!(20, 15);
pub const M4U_PORT_L20_VENC_NBM_WDMA_LITE: u32 = mtk_m4u_id!(20, 16);
pub const M4U_PORT_L20_VENC_FCS_NBM_WDMA: u32 = mtk_m4u_id!(20, 17);
pub const M4U_PORT_L20_JPGDEC_WDMA1: u32 = mtk_m4u_id!(20, 18);
pub const M4U_PORT_L20_JPGDEC_BSDMA1: u32 = mtk_m4u_id!(20, 19);
pub const M4U_PORT_L20_JPGDEC_BUFF_OFFSET1: u32 = mtk_m4u_id!(20, 20);
pub const M4U_PORT_L20_JPGDEC_BUFF_OFFSET0: u32 = mtk_m4u_id!(20, 21);
pub const M4U_PORT_L20_VENC_CUR_LUMA: u32 = mtk_m4u_id!(20, 22);
pub const M4U_PORT_L20_VENC_CUR_CHROMA: u32 = mtk_m4u_id!(20, 23);
pub const M4U_PORT_L20_VENC_REF_LUMA: u32 = mtk_m4u_id!(20, 24);
pub const M4U_PORT_L20_VENC_REF_CHROMA: u32 = mtk_m4u_id!(20, 25);
pub const M4U_PORT_L20_VENC_SUB_R_CHROMA: u32 = mtk_m4u_id!(20, 26);

/* larb21 */
pub const M4U_PORT_L21_VDEC_MC_EXT: u32 = mtk_m4u_id!(21, 0);
pub const M4U_PORT_L21_VDEC_UFO_EXT: u32 = mtk_m4u_id!(21, 1);
pub const M4U_PORT_L21_VDEC_PP_EXT: u32 = mtk_m4u_id!(21, 2);
pub const M4U_PORT_L21_VDEC_PRED_RD_EXT: u32 = mtk_m4u_id!(21, 3);
pub const M4U_PORT_L21_VDEC_PRED_WR_EXT: u32 = mtk_m4u_id!(21, 4);
pub const M4U_PORT_L21_VDEC_PPWRAP_EXT: u32 = mtk_m4u_id!(21, 5);
pub const M4U_PORT_L21_VDEC_TILE_EXT: u32 = mtk_m4u_id!(21, 6);
pub const M4U_PORT_L21_VDEC_VLD_EXT: u32 = mtk_m4u_id!(21, 7);
pub const M4U_PORT_L21_VDEC_VLD2_EXT: u32 = mtk_m4u_id!(21, 8);
pub const M4U_PORT_L21_VDEC_AVC_MV_EXT: u32 = mtk_m4u_id!(21, 9);

/* larb22 */
pub const M4U_PORT_L22_VDEC_MC_EXT: u32 = mtk_m4u_id!(22, 0);
pub const M4U_PORT_L22_VDEC_UFO_EXT: u32 = mtk_m4u_id!(22, 1);
pub const M4U_PORT_L22_VDEC_PP_EXT: u32 = mtk_m4u_id!(22, 2);
pub const M4U_PORT_L22_VDEC_PRED_RD_EXT: u32 = mtk_m4u_id!(22, 3);
pub const M4U_PORT_L22_VDEC_PRED_WR_EXT: u32 = mtk_m4u_id!(22, 4);
pub const M4U_PORT_L22_VDEC_PPWRAP_EXT: u32 = mtk_m4u_id!(22, 5);
pub const M4U_PORT_L22_VDEC_TILE_EXT: u32 = mtk_m4u_id!(22, 6);
pub const M4U_PORT_L22_VDEC_VLD_EXT: u32 = mtk_m4u_id!(22, 7);
pub const M4U_PORT_L22_VDEC_VLD2_EXT: u32 = mtk_m4u_id!(22, 8);
pub const M4U_PORT_L22_VDEC_AVC_MV_EXT: u32 = mtk_m4u_id!(22, 9);

/* larb23 */
pub const M4U_PORT_L23_VDEC_UFO_ENC_EXT: u32 = mtk_m4u_id!(23, 0);
pub const M4U_PORT_L23_VDEC_RDMA_EXT: u32 = mtk_m4u_id!(23, 1);

/* larb24 */
pub const M4U_PORT_L24_VDEC_LAT0_VLD_EXT: u32 = mtk_m4u_id!(24, 0);
pub const M4U_PORT_L24_VDEC_LAT0_VLD2_EXT: u32 = mtk_m4u_id!(24, 1);
pub const M4U_PORT_L24_VDEC_LAT0_AVC_MC_EXT: u32 = mtk_m4u_id!(24, 2);
pub const M4U_PORT_L24_VDEC_LAT0_PRED_RD_EXT: u32 = mtk_m4u_id!(24, 3);
pub const M4U_PORT_L24_VDEC_LAT0_TILE_EXT: u32 = mtk_m4u_id!(24, 4);
pub const M4U_PORT_L24_VDEC_LAT0_WDMA_EXT: u32 = mtk_m4u_id!(24, 5);
pub const M4U_PORT_L24_VDEC_LAT1_VLD_EXT: u32 = mtk_m4u_id!(24, 6);
pub const M4U_PORT_L24_VDEC_LAT1_VLD2_EXT: u32 = mtk_m4u_id!(24, 7);
pub const M4U_PORT_L24_VDEC_LAT1_AVC_MC_EXT: u32 = mtk_m4u_id!(24, 8);
pub const M4U_PORT_L24_VDEC_LAT1_PRED_RD_EXT: u32 = mtk_m4u_id!(24, 9);
pub const M4U_PORT_L24_VDEC_LAT1_TILE_EXT: u32 = mtk_m4u_id!(24, 10);
pub const M4U_PORT_L24_VDEC_LAT1_WDMA_EXT: u32 = mtk_m4u_id!(24, 11);

/* larb25 */
pub const M4U_PORT_L25_CAM_MRAW0_LSCI_M1: u32 = mtk_m4u_id!(25, 0);
pub const M4U_PORT_L25_CAM_MRAW0_CQI_M1: u32 = mtk_m4u_id!(25, 1);
pub const M4U_PORT_L25_CAM_MRAW0_CQI_M2: u32 = mtk_m4u_id!(25, 2);
pub const M4U_PORT_L25_CAM_MRAW0_IMGO_M1: u32 = mtk_m4u_id!(25, 3);
pub const M4U_PORT_L25_CAM_MRAW0_IMGBO_M1: u32 = mtk_m4u_id!(25, 4);
pub const M4U_PORT_L25_CAM_MRAW2_LSCI_M1: u32 = mtk_m4u_id!(25, 5);
pub const M4U_PORT_L25_CAM_MRAW2_CQI_M1: u32 = mtk_m4u_id!(25, 6);
pub const M4U_PORT_L25_CAM_MRAW2_CQI_M2: u32 = mtk_m4u_id!(25, 7);
pub const M4U_PORT_L25_CAM_MRAW2_IMGO_M1: u32 = mtk_m4u_id!(25, 8);
pub const M4U_PORT_L25_CAM_MRAW2_IMGBO_M1: u32 = mtk_m4u_id!(25, 9);
pub const M4U_PORT_L25_CAM_MRAW0_AFO_M1: u32 = mtk_m4u_id!(25, 10);
pub const M4U_PORT_L25_CAM_MRAW2_AFO_M1: u32 = mtk_m4u_id!(25, 11);

/* larb26 */
pub const M4U_PORT_L26_CAM_MRAW1_LSCI_M1: u32 = mtk_m4u_id!(26, 0);
pub const M4U_PORT_L26_CAM_MRAW1_CQI_M1: u32 = mtk_m4u_id!(26, 1);
pub const M4U_PORT_L26_CAM_MRAW1_CQI_M2: u32 = mtk_m4u_id!(26, 2);
pub const M4U_PORT_L26_CAM_MRAW1_IMGO_M1: u32 = mtk_m4u_id!(26, 3);
pub const M4U_PORT_L26_CAM_MRAW1_IMGBO_M1: u32 = mtk_m4u_id!(26, 4);
pub const M4U_PORT_L26_CAM_MRAW3_LSCI_M1: u32 = mtk_m4u_id!(26, 5);
pub const M4U_PORT_L26_CAM_MRAW3_CQI_M1: u32 = mtk_m4u_id!(26, 6);
pub const M4U_PORT_L26_CAM_MRAW3_CQI_M2: u32 = mtk_m4u_id!(26, 7);
pub const M4U_PORT_L26_CAM_MRAW3_IMGO_M1: u32 = mtk_m4u_id!(26, 8);
pub const M4U_PORT_L26_CAM_MRAW3_IMGBO_M1: u32 = mtk_m4u_id!(26, 9);
pub const M4U_PORT_L26_CAM_MRAW1_AFO_M1: u32 = mtk_m4u_id!(26, 10);
pub const M4U_PORT_L26_CAM_MRAW3_AFO_M1: u32 = mtk_m4u_id!(26, 11);

/* larb27 */
pub const M4U_PORT_L27_CAM_IMGO_R1: u32 = mtk_m4u_id!(27, 0);
pub const M4U_PORT_L27_CAM_CQI_R1: u32 = mtk_m4u_id!(27, 1);
pub const M4U_PORT_L27_CAM_CQI_R2: u32 = mtk_m4u_id!(27, 2);
pub const M4U_PORT_L27_CAM_BPCI_R1: u32 = mtk_m4u_id!(27, 3);
pub const M4U_PORT_L27_CAM_LSCI_R1: u32 = mtk_m4u_id!(27, 4);
pub const M4U_PORT_L27_CAM_RAWI_R2: u32 = mtk_m4u_id!(27, 5);
pub const M4U_PORT_L27_CAM_RAWI_R3: u32 = mtk_m4u_id!(27, 6);
pub const M4U_PORT_L27_CAM_UFDI_R2: u32 = mtk_m4u_id!(27, 7);
pub const M4U_PORT_L27_CAM_UFDI_R3: u32 = mtk_m4u_id!(27, 8);
pub const M4U_PORT_L27_CAM_RAWI_R4: u32 = mtk_m4u_id!(27, 9);
pub const M4U_PORT_L27_CAM_RAWI_R5: u32 = mtk_m4u_id!(27, 10);
pub const M4U_PORT_L27_CAM_AAI_R1: u32 = mtk_m4u_id!(27, 11);
pub const M4U_PORT_L27_CAM_FHO_R1: u32 = mtk_m4u_id!(27, 12);
pub const M4U_PORT_L27_CAM_AAO_R1: u32 = mtk_m4u_id!(27, 13);
pub const M4U_PORT_L27_CAM_TSFSO_R1: u32 = mtk_m4u_id!(27, 14);
pub const M4U_PORT_L27_CAM_FLKO_R1: u32 = mtk_m4u_id!(27, 15);

/* larb28 */
pub const M4U_PORT_L28_CAM_YUVO_R1: u32 = mtk_m4u_id!(28, 0);
pub const M4U_PORT_L28_CAM_YUVO_R3: u32 = mtk_m4u_id!(28, 1);
pub const M4U_PORT_L28_CAM_YUVCO_R1: u32 = mtk_m4u_id!(28, 2);
pub const M4U_PORT_L28_CAM_YUVO_R2: u32 = mtk_m4u_id!(28, 3);
pub const M4U_PORT_L28_CAM_RZH1N2TO_R1: u32 = mtk_m4u_id!(28, 4);
pub const M4U_PORT_L28_CAM_DRZS4NO_R1: u32 = mtk_m4u_id!(28, 5);
pub const M4U_PORT_L28_CAM_TNCSO_R1: u32 = mtk_m4u_id!(28, 6);

/* Infra iommu ports */
/* PCIe1: read: BIT16; write BIT17. */
pub const IOMMU_PORT_INFRA_PCIE1: u32 = mtk_ifaiommu_peri_id!(16);
/* PCIe0: read: BIT18; write BIT19. */
pub const IOMMU_PORT_INFRA_PCIE0: u32 = mtk_ifaiommu_peri_id!(18);
pub const IOMMU_PORT_INFRA_SSUSB_P3_R: u32 = mtk_ifaiommu_peri_id!(20);
pub const IOMMU_PORT_INFRA_SSUSB_P3_W: u32 = mtk_ifaiommu_peri_id!(21);
pub const IOMMU_PORT_INFRA_SSUSB_P2_R: u32 = mtk_ifaiommu_peri_id!(22);
pub const IOMMU_PORT_INFRA_SSUSB_P2_W: u32 = mtk_ifaiommu_peri_id!(23);
pub const IOMMU_PORT_INFRA_SSUSB_P1_1_R: u32 = mtk_ifaiommu_peri_id!(24);
pub const IOMMU_PORT_INFRA_SSUSB_P1_1_W: u32 = mtk_ifaiommu_peri_id!(25);
pub const IOMMU_PORT_INFRA_SSUSB_P1_0_R: u32 = mtk_ifaiommu_peri_id!(26);
pub const IOMMU_PORT_INFRA_SSUSB_P1_0_W: u32 = mtk_ifaiommu_peri_id!(27);
pub const IOMMU_PORT_INFRA_SSUSB2_R: u32 = mtk_ifaiommu_peri_id!(28);
pub const IOMMU_PORT_INFRA_SSUSB2_W: u32 = mtk_ifaiommu_peri_id!(29);
pub const IOMMU_PORT_INFRA_SSUSB_R: u32 = mtk_ifaiommu_peri_id!(30);
pub const IOMMU_PORT_INFRA_SSUSB_W: u32 = mtk_ifaiommu_peri_id!(31);




// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
