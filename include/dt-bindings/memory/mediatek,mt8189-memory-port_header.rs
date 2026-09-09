/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2025 MediaTek Inc.
 * Author: Zhengnan chen <zhengnan.chen@mediatek.com>
 */

// External dependency: #include <dt-bindings/memory/mtk-memory-port.h>

pub const SMI_L0_ID: u32 = (0);
pub const SMI_L1_ID: u32 = (1);
pub const SMI_L2_ID: u32 = (2);
pub const SMI_L4_ID: u32 = (3);
pub const SMI_L7_ID: u32 = (4);
pub const SMI_L9_ID: u32 = (5);
pub const SMI_L11_ID: u32 = (6);
pub const SMI_L13_ID: u32 = (7);
pub const SMI_L14_ID: u32 = (8);
pub const SMI_L16_ID: u32 = (9);
pub const SMI_L17_ID: u32 = (10);
pub const SMI_L19_ID: u32 = (11);
pub const SMI_L20_ID: u32 = (12);

/*
 * MM IOMMU supports 16GB dma address. We separate it to four ranges:
 * 0 ~ 4G; 4G ~ 8G; 8G ~ 12G; 12G ~ 16G, we could adjust these masters
 * locate in anyone region. BUT:
 * a) Make sure all the ports inside a larb are in one range.
 * b) The iova of any master can NOT cross the 4G/8G/12G boundary.
 *
 * This is the suggested mapping in this SoC:
 *
 * modules		dma-address-region	larbs-ports
 * disp/mdp		0 ~ 4G			larb0/1/2
 * vcodec		4G ~ 8G                 larb4/7
 * imgsys/cam/ipesys	8G ~ 12G                the other larbs.
 * N/A			12G ~ 16G
 */

/* Larb0 -- disp */
pub const M4U_L0_P0_DISP_OVL0_4L_HDR: u32 = MTK_M4U_ID(SMI_L0_ID, 0);
pub const M4U_L0_P1_DISP_OVL0_4L_RDMA0: u32 = MTK_M4U_ID(SMI_L0_ID, 1);
pub const M4U_L0_P2_DISP_OVL1_4L_RDMA1: u32 = MTK_M4U_ID(SMI_L0_ID, 2);
pub const M4U_L0_P3_DISP_OVL0_4L_RDMA2: u32 = MTK_M4U_ID(SMI_L0_ID, 3);
pub const M4U_L0_P4_DISP_OVL1_4L_RDMA3: u32 = MTK_M4U_ID(SMI_L0_ID, 4);
pub const M4U_L0_P5_DISP_RDMA0: u32 = MTK_M4U_ID(SMI_L0_ID, 5);
pub const M4U_L0_P6_DISP_WDMA0: u32 = MTK_M4U_ID(SMI_L0_ID, 6);
pub const M4U_L0_P7_DISP_FAKE_ENG0: u32 = MTK_M4U_ID(SMI_L0_ID, 7);

/* Larb1 -- disp */
pub const M4U_L1_P0_DISP_OVL1_4L_HDR: u32 = MTK_M4U_ID(SMI_L1_ID, 0);
pub const M4U_L1_P1_DISP_OVL1_4L_RDMA0: u32 = MTK_M4U_ID(SMI_L1_ID, 1);
pub const M4U_L1_P2_DISP_OVL0_4L_RDMA1: u32 = MTK_M4U_ID(SMI_L1_ID, 2);
pub const M4U_L1_P3_DISP_OVL1_4L_RDMA2: u32 = MTK_M4U_ID(SMI_L1_ID, 3);
pub const M4U_L1_P4_DISP_OVL0_4L_RDMA3: u32 = MTK_M4U_ID(SMI_L1_ID, 4);
pub const M4U_L1_P5_DISP_RDMA1: u32 = MTK_M4U_ID(SMI_L1_ID, 5);
pub const M4U_L1_P6_DISP_WDMA1: u32 = MTK_M4U_ID(SMI_L1_ID, 6);
pub const M4U_L1_P7_DISP_FAKE_ENG1: u32 = MTK_M4U_ID(SMI_L1_ID, 7);

/* Larb2 -- mmlsys(mdp) */
pub const M4U_L2_P0_MDP_RDMA0: u32 = MTK_M4U_ID(SMI_L2_ID, 0);
pub const M4U_L2_P1_MDP_RDMA1: u32 = MTK_M4U_ID(SMI_L2_ID, 1);
pub const M4U_L2_P2_MDP_WROT0: u32 = MTK_M4U_ID(SMI_L2_ID, 2);
pub const M4U_L2_P3_MDP_WROT1: u32 = MTK_M4U_ID(SMI_L2_ID, 3);
pub const M4U_L2_P4_MDP_DUMMY0: u32 = MTK_M4U_ID(SMI_L2_ID, 4);
pub const M4U_L2_P5_MDP_DUMMY1: u32 = MTK_M4U_ID(SMI_L2_ID, 5);
pub const M4U_L2_P6_MDP_RDMA2: u32 = MTK_M4U_ID(SMI_L2_ID, 6);
pub const M4U_L2_P7_MDP_RDMA3: u32 = MTK_M4U_ID(SMI_L2_ID, 7);
pub const M4U_L2_P8_MDP_WROT2: u32 = MTK_M4U_ID(SMI_L2_ID, 8);
pub const M4U_L2_P9_MDP_WROT3: u32 = MTK_M4U_ID(SMI_L2_ID, 9);
pub const M4U_L2_P10_DISP_FAKE0: u32 = MTK_M4U_ID(SMI_L2_ID, 10);

/* Larb3: null */

/* Larb4 -- vdec */
pub const M4U_L4_P0_HW_VDEC_MC_EXT: u32 = MTK_M4U_ID(SMI_L4_ID, 0);
pub const M4U_L4_P1_HW_VDEC_UFO_EXT: u32 = MTK_M4U_ID(SMI_L4_ID, 1);
pub const M4U_L4_P2_HW_VDEC_PP_EXT: u32 = MTK_M4U_ID(SMI_L4_ID, 2);
pub const M4U_L4_P3_HW_VDEC_PRED_RD_EXT: u32 = MTK_M4U_ID(SMI_L4_ID, 3);
pub const M4U_L4_P4_HW_VDEC_PRED_WR_EXT: u32 = MTK_M4U_ID(SMI_L4_ID, 4);
pub const M4U_L4_P5_HW_VDEC_PPWRAP_EXT: u32 = MTK_M4U_ID(SMI_L4_ID, 5);
pub const M4U_L4_P6_HW_VDEC_TILE_EXT: u32 = MTK_M4U_ID(SMI_L4_ID, 6);
pub const M4U_L4_P7_HW_VDEC_VLD_EXT: u32 = MTK_M4U_ID(SMI_L4_ID, 7);
pub const M4U_L4_P8_HW_VDEC_VLD2_EXT: u32 = MTK_M4U_ID(SMI_L4_ID, 8);
pub const M4U_L4_P9_HW_VDEC_AVC_MV_EXT: u32 = MTK_M4U_ID(SMI_L4_ID, 9);
pub const M4U_L4_P10_HW_VDEC_RG_CTRL_DMA_EXT: u32 = MTK_M4U_ID(SMI_L4_ID, 10);
pub const M4U_L4_P11_HW_VDEC_UFO_ENC_EXT: u32 = MTK_M4U_ID(SMI_L4_ID, 11);

/* Larb5: null */

/* Larb6: null */

/* Larb7 -- venc */
pub const M4U_L7_P0_VENC_RCPU: u32 = MTK_M4U_ID(SMI_L7_ID, 0);
pub const M4U_L7_P1_VENC_REC: u32 = MTK_M4U_ID(SMI_L7_ID, 1);
pub const M4U_L7_P2_VENC_BSDMA: u32 = MTK_M4U_ID(SMI_L7_ID, 2);
pub const M4U_L7_P3_VENC_SV_COMV: u32 = MTK_M4U_ID(SMI_L7_ID, 3);
pub const M4U_L7_P4_VENC_RD_COMV: u32 = MTK_M4U_ID(SMI_L7_ID, 4);
pub const M4U_L7_P5_JPGENC_Y_RDMA: u32 = MTK_M4U_ID(SMI_L7_ID, 5);
pub const M4U_L7_P6_JPGENC_C_RDMA: u32 = MTK_M4U_ID(SMI_L7_ID, 6);
pub const M4U_L7_P7_JPGENC_Q_RDMA: u32 = MTK_M4U_ID(SMI_L7_ID, 7);
pub const M4U_L7_P8_VENC_SUB_W_LUMA: u32 = MTK_M4U_ID(SMI_L7_ID, 8);
pub const M4U_L7_P9_JPGENC_BSDMA: u32 = MTK_M4U_ID(SMI_L7_ID, 9);
pub const M4U_L7_P10_VENC_CUR_LUMA: u32 = MTK_M4U_ID(SMI_L7_ID, 10);
pub const M4U_L7_P11_VENC_CUR_CHROMA: u32 = MTK_M4U_ID(SMI_L7_ID, 11);
pub const M4U_L7_P12_VENC_REF_LUMA: u32 = MTK_M4U_ID(SMI_L7_ID, 12);
pub const M4U_L7_P13_VENC_REF_CHROMA: u32 = MTK_M4U_ID(SMI_L7_ID, 13);
pub const M4U_L7_P14_VENC_SUB_R_LUMA: u32 = MTK_M4U_ID(SMI_L7_ID, 14);
pub const M4U_L7_P15_JPGDEC_WDMA: u32 = MTK_M4U_ID(SMI_L7_ID, 15);
pub const M4U_L7_P16_JPGDEC_BSDMA: u32 = MTK_M4U_ID(SMI_L7_ID, 16);
pub const M4U_L7_P17_JPGDEC_HUFF_OFFSET: u32 = MTK_M4U_ID(SMI_L7_ID, 17);

/* Larb8: null */

/* Larb9 --imgsys */
pub const M4U_L9_P0_IMGI_D1: u32 = MTK_M4U_ID(SMI_L9_ID, 0);
pub const M4U_L9_P1_IMGBI_D1: u32 = MTK_M4U_ID(SMI_L9_ID, 1);
pub const M4U_L9_P2_DMGI_D1: u32 = MTK_M4U_ID(SMI_L9_ID, 2);
pub const M4U_L9_P3_DEPI_D1: u32 = MTK_M4U_ID(SMI_L9_ID, 3);
pub const M4U_L9_P4_LCE_D1: u32 = MTK_M4U_ID(SMI_L9_ID, 4);
pub const M4U_L9_P5_SMTI_D1: u32 = MTK_M4U_ID(SMI_L9_ID, 5);
pub const M4U_L9_P6_SMTO_D2: u32 = MTK_M4U_ID(SMI_L9_ID, 6);
pub const M4U_L9_P7_SMTO_D1: u32 = MTK_M4U_ID(SMI_L9_ID, 7);
pub const M4U_L9_P8_CRZO_D1: u32 = MTK_M4U_ID(SMI_L9_ID, 8);
pub const M4U_L9_P9_IMG3O_D1: u32 = MTK_M4U_ID(SMI_L9_ID, 9);
pub const M4U_L9_P10_VIPI_D1: u32 = MTK_M4U_ID(SMI_L9_ID, 10);
pub const M4U_L9_P11_SMTI_D5: u32 = MTK_M4U_ID(SMI_L9_ID, 11);
pub const M4U_L9_P12_TIMGO_D1: u32 = MTK_M4U_ID(SMI_L9_ID, 12);
pub const M4U_L9_P13_UFBC_W0: u32 = MTK_M4U_ID(SMI_L9_ID, 13);
pub const M4U_L9_P14_UFBC_R0: u32 = MTK_M4U_ID(SMI_L9_ID, 14);
pub const M4U_L9_P15_WPE_RDMA1: u32 = MTK_M4U_ID(SMI_L9_ID, 15);
pub const M4U_L9_P16_WPE_RDMA0: u32 = MTK_M4U_ID(SMI_L9_ID, 16);
pub const M4U_L9_P17_WPE_WDMA: u32 = MTK_M4U_ID(SMI_L9_ID, 17);
pub const M4U_L9_P18_MFB_RDMA0: u32 = MTK_M4U_ID(SMI_L9_ID, 18);
pub const M4U_L9_P19_MFB_RDMA1: u32 = MTK_M4U_ID(SMI_L9_ID, 19);
pub const M4U_L9_P20_MFB_RDMA2: u32 = MTK_M4U_ID(SMI_L9_ID, 20);
pub const M4U_L9_P21_MFB_RDMA3: u32 = MTK_M4U_ID(SMI_L9_ID, 21);
pub const M4U_L9_P22_MFB_RDMA4: u32 = MTK_M4U_ID(SMI_L9_ID, 22);
pub const M4U_L9_P23_MFB_RDMA5: u32 = MTK_M4U_ID(SMI_L9_ID, 23);
pub const M4U_L9_P24_MFB_WDMA0: u32 = MTK_M4U_ID(SMI_L9_ID, 24);
pub const M4U_L9_P25_MFB_WDMA1: u32 = MTK_M4U_ID(SMI_L9_ID, 25);
pub const M4U_L9_P26_RESERVE6: u32 = MTK_M4U_ID(SMI_L9_ID, 26);
pub const M4U_L9_P27_RESERVE7: u32 = MTK_M4U_ID(SMI_L9_ID, 27);
pub const M4U_L9_P28_RESERVE8: u32 = MTK_M4U_ID(SMI_L9_ID, 28);

/* Larb10: null */

/* Larb11 -- imgsys */
pub const M4U_L11_P0_IMGI_D1: u32 = MTK_M4U_ID(SMI_L11_ID, 0);
pub const M4U_L11_P1_IMGBI_D1: u32 = MTK_M4U_ID(SMI_L11_ID, 1);
pub const M4U_L11_P2_DMGI_D1: u32 = MTK_M4U_ID(SMI_L11_ID, 2);
pub const M4U_L11_P3_DEPI_D1: u32 = MTK_M4U_ID(SMI_L11_ID, 3);
pub const M4U_L11_P4_LCE_D1: u32 = MTK_M4U_ID(SMI_L11_ID, 4);
pub const M4U_L11_P5_SMTI_D1: u32 = MTK_M4U_ID(SMI_L11_ID, 5);
pub const M4U_L11_P6_SMTO_D2: u32 = MTK_M4U_ID(SMI_L11_ID, 6);
pub const M4U_L11_P7_SMTO_D1: u32 = MTK_M4U_ID(SMI_L11_ID, 7);
pub const M4U_L11_P8_CRZO_D1: u32 = MTK_M4U_ID(SMI_L11_ID, 8);
pub const M4U_L11_P9_IMG3O_D1: u32 = MTK_M4U_ID(SMI_L11_ID, 9);
pub const M4U_L11_P10_VIPI_D1: u32 = MTK_M4U_ID(SMI_L11_ID, 10);
pub const M4U_L11_P11_SMTI_D5: u32 = MTK_M4U_ID(SMI_L11_ID, 11);
pub const M4U_L11_P12_TIMGO_D1: u32 = MTK_M4U_ID(SMI_L11_ID, 12);
pub const M4U_L11_P13_UFBC_W0: u32 = MTK_M4U_ID(SMI_L11_ID, 13);
pub const M4U_L11_P14_UFBC_R0: u32 = MTK_M4U_ID(SMI_L11_ID, 14);
pub const M4U_L11_P15_WPE_RDMA1: u32 = MTK_M4U_ID(SMI_L11_ID, 15);
pub const M4U_L11_P16_WPE_RDMA0: u32 = MTK_M4U_ID(SMI_L11_ID, 16);
pub const M4U_L11_P17_WPE_WDMA: u32 = MTK_M4U_ID(SMI_L11_ID, 17);
pub const M4U_L11_P18_MFB_RDMA0: u32 = MTK_M4U_ID(SMI_L11_ID, 18);
pub const M4U_L11_P19_MFB_RDMA1: u32 = MTK_M4U_ID(SMI_L11_ID, 19);
pub const M4U_L11_P20_MFB_RDMA2: u32 = MTK_M4U_ID(SMI_L11_ID, 20);
pub const M4U_L11_P21_MFB_RDMA3: u32 = MTK_M4U_ID(SMI_L11_ID, 21);
pub const M4U_L11_P22_MFB_RDMA4: u32 = MTK_M4U_ID(SMI_L11_ID, 22);
pub const M4U_L11_P23_MFB_RDMA5: u32 = MTK_M4U_ID(SMI_L11_ID, 23);
pub const M4U_L11_P24_MFB_WDMA0: u32 = MTK_M4U_ID(SMI_L11_ID, 24);
pub const M4U_L11_P25_MFB_WDMA1: u32 = MTK_M4U_ID(SMI_L11_ID, 25);
pub const M4U_L11_P26_RESERVE6: u32 = MTK_M4U_ID(SMI_L11_ID, 26);
pub const M4U_L11_P27_RESERVE7: u32 = MTK_M4U_ID(SMI_L11_ID, 27);
pub const M4U_L11_P28_RESERVE8: u32 = MTK_M4U_ID(SMI_L11_ID, 28);

/* Larb12: null */

/* Larb13 -- cam */
pub const M4U_L13_P0_MRAWI: u32 = MTK_M4U_ID(SMI_L13_ID, 0);
pub const M4U_L13_P1_MRAWO_0: u32 = MTK_M4U_ID(SMI_L13_ID, 1);
pub const M4U_L13_P2_MRAWO_1: u32 = MTK_M4U_ID(SMI_L13_ID, 2);
pub const M4U_L13_P3_CAMSV_1: u32 = MTK_M4U_ID(SMI_L13_ID, 3);
pub const M4U_L13_P4_CAMSV_2: u32 = MTK_M4U_ID(SMI_L13_ID, 4);
pub const M4U_L13_P5_CAMSV_3: u32 = MTK_M4U_ID(SMI_L13_ID, 5);
pub const M4U_L13_P6_CAMSV_4: u32 = MTK_M4U_ID(SMI_L13_ID, 6);
pub const M4U_L13_P7_CAMSV_5: u32 = MTK_M4U_ID(SMI_L13_ID, 7);
pub const M4U_L13_P8_CAMSV_6: u32 = MTK_M4U_ID(SMI_L13_ID, 8);
pub const M4U_L13_P9_CCUI: u32 = MTK_M4U_ID(SMI_L13_ID, 9);
pub const M4U_L13_P10_CCUO: u32 = MTK_M4U_ID(SMI_L13_ID, 10);
pub const M4U_L13_P11_FAKE: u32 = MTK_M4U_ID(SMI_L13_ID, 11);
pub const M4U_L13_P12_PDAI_0: u32 = MTK_M4U_ID(SMI_L13_ID, 12);
pub const M4U_L13_P13_PDAI_1: u32 = MTK_M4U_ID(SMI_L13_ID, 13);
pub const M4U_L13_P14_PDAO: u32 = MTK_M4U_ID(SMI_L13_ID, 14);

/* Larb14 -- cam */
pub const M4U_L14_P0_RESERVE: u32 = MTK_M4U_ID(SMI_L14_ID, 0);
pub const M4U_L14_P1_RESERVE: u32 = MTK_M4U_ID(SMI_L14_ID, 1);
pub const M4U_L14_P2_RESERVE: u32 = MTK_M4U_ID(SMI_L14_ID, 2);
pub const M4U_L14_P3_CAMSV_0: u32 = MTK_M4U_ID(SMI_L14_ID, 3);
pub const M4U_L14_P4_CCUI: u32 = MTK_M4U_ID(SMI_L14_ID, 4);
pub const M4U_L14_P5_CCUO: u32 = MTK_M4U_ID(SMI_L14_ID, 5);
pub const M4U_L14_P6_CAMSV_7: u32 = MTK_M4U_ID(SMI_L14_ID, 6);
pub const M4U_L14_P7_CAMSV_8: u32 = MTK_M4U_ID(SMI_L14_ID, 7);
pub const M4U_L14_P8_CAMSV_9: u32 = MTK_M4U_ID(SMI_L14_ID, 8);
pub const M4U_L14_P9_CAMSV_10: u32 = MTK_M4U_ID(SMI_L14_ID, 9);

/* Larb15: null */

/* Larb16 -- cam */
pub const M4U_L16_P0_IMGO_R1_A: u32 = MTK_M4U_ID(SMI_L16_ID, 0);
pub const M4U_L16_P1_RRZO_R1_A: u32 = MTK_M4U_ID(SMI_L16_ID, 1);
pub const M4U_L16_P2_CQI_R1_A: u32 = MTK_M4U_ID(SMI_L16_ID, 2);
pub const M4U_L16_P3_BPCI_R1_A: u32 = MTK_M4U_ID(SMI_L16_ID, 3);
pub const M4U_L16_P4_YUVO_R1_A: u32 = MTK_M4U_ID(SMI_L16_ID, 4);
pub const M4U_L16_P5_UFDI_R2_A: u32 = MTK_M4U_ID(SMI_L16_ID, 5);
pub const M4U_L16_P6_RAWI_R2_A: u32 = MTK_M4U_ID(SMI_L16_ID, 6);
pub const M4U_L16_P7_RAWI_R3_A: u32 = MTK_M4U_ID(SMI_L16_ID, 7);
pub const M4U_L16_P8_AAO_R1_A: u32 = MTK_M4U_ID(SMI_L16_ID, 8);
pub const M4U_L16_P9_AFO_R1_A: u32 = MTK_M4U_ID(SMI_L16_ID, 9);
pub const M4U_L16_P10_FLKO_R1_A: u32 = MTK_M4U_ID(SMI_L16_ID, 10);
pub const M4U_L16_P11_LCESO_R1_A: u32 = MTK_M4U_ID(SMI_L16_ID, 11);
pub const M4U_L16_P12_CRZO_R1_A: u32 = MTK_M4U_ID(SMI_L16_ID, 12);
pub const M4U_L16_P13_LTMSO_R1_A: u32 = MTK_M4U_ID(SMI_L16_ID, 13);
pub const M4U_L16_P14_RSSO_R1_A: u32 = MTK_M4U_ID(SMI_L16_ID, 14);
pub const M4U_L16_P15_AAHO_R1_A: u32 = MTK_M4U_ID(SMI_L16_ID, 15);
pub const M4U_L16_P16_LSCI_R1_A: u32 = MTK_M4U_ID(SMI_L16_ID, 16);

/* Larb17 -- cam */
pub const M4U_L17_P0_IMGO_R1_B: u32 = MTK_M4U_ID(SMI_L17_ID, 0);
pub const M4U_L17_P1_RRZO_R1_B: u32 = MTK_M4U_ID(SMI_L17_ID, 1);
pub const M4U_L17_P2_CQI_R1_B: u32 = MTK_M4U_ID(SMI_L17_ID, 2);
pub const M4U_L17_P3_BPCI_R1_B: u32 = MTK_M4U_ID(SMI_L17_ID, 3);
pub const M4U_L17_P4_YUVO_R1_B: u32 = MTK_M4U_ID(SMI_L17_ID, 4);
pub const M4U_L17_P5_UFDI_R2_B: u32 = MTK_M4U_ID(SMI_L17_ID, 5);
pub const M4U_L17_P6_RAWI_R2_B: u32 = MTK_M4U_ID(SMI_L17_ID, 6);
pub const M4U_L17_P7_RAWI_R3_B: u32 = MTK_M4U_ID(SMI_L17_ID, 7);
pub const M4U_L17_P8_AAO_R1_B: u32 = MTK_M4U_ID(SMI_L17_ID, 8);
pub const M4U_L17_P9_AFO_R1_B: u32 = MTK_M4U_ID(SMI_L17_ID, 9);
pub const M4U_L17_P10_FLKO_R1_B: u32 = MTK_M4U_ID(SMI_L17_ID, 10);
pub const M4U_L17_P11_LCESO_R1_B: u32 = MTK_M4U_ID(SMI_L17_ID, 11);
pub const M4U_L17_P12_CRZO_R1_B: u32 = MTK_M4U_ID(SMI_L17_ID, 12);
pub const M4U_L17_P13_LTMSO_R1_B: u32 = MTK_M4U_ID(SMI_L17_ID, 13);
pub const M4U_L17_P14_RSSO_R1_B: u32 = MTK_M4U_ID(SMI_L17_ID, 14);
pub const M4U_L17_P15_AAHO_R1_B: u32 = MTK_M4U_ID(SMI_L17_ID, 15);
pub const M4U_L17_P16_LSCI_R1_B: u32 = MTK_M4U_ID(SMI_L17_ID, 16);

/* Larb19 -- ipesys */
pub const M4U_L19_P0_DVS_RDMA: u32 = MTK_M4U_ID(SMI_L19_ID, 0);
pub const M4U_L19_P1_DVS_WDMA: u32 = MTK_M4U_ID(SMI_L19_ID, 1);
pub const M4U_L19_P2_DVP_RDMA: u32 = MTK_M4U_ID(SMI_L19_ID, 2);
pub const M4U_L19_P3_DVP_WDMA: u32 = MTK_M4U_ID(SMI_L19_ID, 3);

/* Larb20 -- ipesys */
pub const M4U_L20_P0_FDVT_RDA_0: u32 = MTK_M4U_ID(SMI_L20_ID, 0);
pub const M4U_L20_P1_FDVT_RDB_0: u32 = MTK_M4U_ID(SMI_L20_ID, 1);
pub const M4U_L20_P2_FDVT_WRA_0: u32 = MTK_M4U_ID(SMI_L20_ID, 2);
pub const M4U_L20_P3_FDVT_WRB_0: u32 = MTK_M4U_ID(SMI_L20_ID, 3);
pub const M4U_L20_P4_RSC_RDMA: u32 = MTK_M4U_ID(SMI_L20_ID, 4);
pub const M4U_L20_P5_RSC_WDMA: u32 = MTK_M4U_ID(SMI_L20_ID, 5);

/* fake larb21 for gce */
pub const M4U_L21_GCE_DM: u32 = MTK_M4U_ID(21, 0);
pub const M4U_L21_GCE_MM: u32 = MTK_M4U_ID(21, 1);

/* fake larb & port for svp and dual svp and wfd */
pub const M4U_PORT_SVP_HEAP: u32 = MTK_M4U_ID(22, 0);
pub const M4U_PORT_DUAL_SVP_HEAP: u32 = MTK_M4U_ID(22, 1);
pub const M4U_PORT_WFD_HEAP: u32 = MTK_M4U_ID(22, 2);

/* fake larb0 for apu */
pub const M4U_L0_APU_DATA: u32 = MTK_M4U_ID(0, 0);
pub const M4U_L0_APU_CODE: u32 = MTK_M4U_ID(0, 1);
pub const M4U_L0_APU_SECURE: u32 = MTK_M4U_ID(0, 2);
pub const M4U_L0_APU_VLM: u32 = MTK_M4U_ID(0, 3);

/* infra/peri */
pub const IFR_IOMMU_PORT_PCIE_0: u32 = MTK_IFAIOMMU_PERI_ID(0, 26);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
