/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2020 MediaTek Inc.
 *
 * Author: Chao Hao <chao.hao@mediatek.com>
 * Author: Yong Wu <yong.wu@mediatek.com>
 */

// Dependency: MTK_M4U_ID is supplied by <dt-bindings/memory/mtk-memory-port.h>.

/*
 * MM IOMMU supports 16GB dma address.
 *
 * The address will preassign like this:
 *
 * modules    dma-address-region      larbs-ports
 * disp         0 ~ 4G                larb0/1
 * vcodec      4G ~ 8G                larb4/5/7
 * cam/mdp     8G ~ 12G               larb2/9/11/13/14/16/17/18/19/20
 * CCU0    0x4000_0000 ~ 0x43ff_ffff  larb13: port 9/10
 * CCU1    0x4400_0000 ~ 0x47ff_ffff  larb14: port 4/5
 *
 * larb3/6/8/10/12/15 is null.
 */

/* larb0 */
pub const M4U_PORT_L0_DISP_POSTMASK0: u32 = MTK_M4U_ID(0, 0);
pub const M4U_PORT_L0_OVL_RDMA0_HDR: u32 = MTK_M4U_ID(0, 1);
pub const M4U_PORT_L0_OVL_RDMA0: u32 = MTK_M4U_ID(0, 2);
pub const M4U_PORT_L0_DISP_RDMA0: u32 = MTK_M4U_ID(0, 3);
pub const M4U_PORT_L0_DISP_WDMA0: u32 = MTK_M4U_ID(0, 4);
pub const M4U_PORT_L0_DISP_FAKE0: u32 = MTK_M4U_ID(0, 5);

/* larb1 */
pub const M4U_PORT_L1_OVL_2L_RDMA0_HDR: u32 = MTK_M4U_ID(1, 0);
pub const M4U_PORT_L1_OVL_2L_RDMA2_HDR: u32 = MTK_M4U_ID(1, 1);
pub const M4U_PORT_L1_OVL_2L_RDMA0: u32 = MTK_M4U_ID(1, 2);
pub const M4U_PORT_L1_OVL_2L_RDMA2: u32 = MTK_M4U_ID(1, 3);
pub const M4U_PORT_L1_DISP_MDP_RDMA4: u32 = MTK_M4U_ID(1, 4);
pub const M4U_PORT_L1_DISP_RDMA4: u32 = MTK_M4U_ID(1, 5);
pub const M4U_PORT_L1_DISP_UFBC_WDMA0: u32 = MTK_M4U_ID(1, 6);
pub const M4U_PORT_L1_DISP_FAKE1: u32 = MTK_M4U_ID(1, 7);

/* larb2 */
pub const M4U_PORT_L2_MDP_RDMA0: u32 = MTK_M4U_ID(2, 0);
pub const M4U_PORT_L2_MDP_RDMA1: u32 = MTK_M4U_ID(2, 1);
pub const M4U_PORT_L2_MDP_WROT0: u32 = MTK_M4U_ID(2, 2);
pub const M4U_PORT_L2_MDP_WROT1: u32 = MTK_M4U_ID(2, 3);
pub const M4U_PORT_L2_MDP_DISP_FAKE0: u32 = MTK_M4U_ID(2, 4);

/* larb3: null */

/* larb4 */
pub const M4U_PORT_L4_VDEC_MC_EXT: u32 = MTK_M4U_ID(4, 0);
pub const M4U_PORT_L4_VDEC_UFO_EXT: u32 = MTK_M4U_ID(4, 1);
pub const M4U_PORT_L4_VDEC_PP_EXT: u32 = MTK_M4U_ID(4, 2);
pub const M4U_PORT_L4_VDEC_PRED_RD_EXT: u32 = MTK_M4U_ID(4, 3);
pub const M4U_PORT_L4_VDEC_PRED_WR_EXT: u32 = MTK_M4U_ID(4, 4);
pub const M4U_PORT_L4_VDEC_PPWRAP_EXT: u32 = MTK_M4U_ID(4, 5);
pub const M4U_PORT_L4_VDEC_TILE_EXT: u32 = MTK_M4U_ID(4, 6);
pub const M4U_PORT_L4_VDEC_VLD_EXT: u32 = MTK_M4U_ID(4, 7);
pub const M4U_PORT_L4_VDEC_VLD2_EXT: u32 = MTK_M4U_ID(4, 8);
pub const M4U_PORT_L4_VDEC_AVC_MV_EXT: u32 = MTK_M4U_ID(4, 9);
pub const M4U_PORT_L4_VDEC_RG_CTRL_DMA_EXT: u32 = MTK_M4U_ID(4, 10);

/* larb5 */
pub const M4U_PORT_L5_VDEC_LAT0_VLD_EXT: u32 = MTK_M4U_ID(5, 0);
pub const M4U_PORT_L5_VDEC_LAT0_VLD2_EXT: u32 = MTK_M4U_ID(5, 1);
pub const M4U_PORT_L5_VDEC_LAT0_AVC_MV_EXT: u32 = MTK_M4U_ID(5, 2);
pub const M4U_PORT_L5_VDEC_LAT0_PRED_RD_EXT: u32 = MTK_M4U_ID(5, 3);
pub const M4U_PORT_L5_VDEC_LAT0_TILE_EXT: u32 = MTK_M4U_ID(5, 4);
pub const M4U_PORT_L5_VDEC_LAT0_WDMA_EXT: u32 = MTK_M4U_ID(5, 5);
pub const M4U_PORT_L5_VDEC_LAT0_RG_CTRL_DMA_EXT: u32 = MTK_M4U_ID(5, 6);
pub const M4U_PORT_L5_VDEC_UFO_ENC_EXT: u32 = MTK_M4U_ID(5, 7);

/* larb6: null */

/* larb7 */
pub const M4U_PORT_L7_VENC_RCPU: u32 = MTK_M4U_ID(7, 0);
pub const M4U_PORT_L7_VENC_REC: u32 = MTK_M4U_ID(7, 1);
pub const M4U_PORT_L7_VENC_BSDMA: u32 = MTK_M4U_ID(7, 2);
pub const M4U_PORT_L7_VENC_SV_COMV: u32 = MTK_M4U_ID(7, 3);
pub const M4U_PORT_L7_VENC_RD_COMV: u32 = MTK_M4U_ID(7, 4);
pub const M4U_PORT_L7_VENC_CUR_LUMA: u32 = MTK_M4U_ID(7, 5);
pub const M4U_PORT_L7_VENC_CUR_CHROMA: u32 = MTK_M4U_ID(7, 6);
pub const M4U_PORT_L7_VENC_REF_LUMA: u32 = MTK_M4U_ID(7, 7);
pub const M4U_PORT_L7_VENC_REF_CHROMA: u32 = MTK_M4U_ID(7, 8);
pub const M4U_PORT_L7_JPGENC_Y_RDMA: u32 = MTK_M4U_ID(7, 9);
pub const M4U_PORT_L7_JPGENC_Q_RDMA: u32 = MTK_M4U_ID(7, 10);
pub const M4U_PORT_L7_JPGENC_C_TABLE: u32 = MTK_M4U_ID(7, 11);
pub const M4U_PORT_L7_JPGENC_BSDMA: u32 = MTK_M4U_ID(7, 12);
pub const M4U_PORT_L7_VENC_SUB_R_LUMA: u32 = MTK_M4U_ID(7, 13);
pub const M4U_PORT_L7_VENC_SUB_W_LUMA: u32 = MTK_M4U_ID(7, 14);

/* larb8: null */

/* larb9 */
pub const M4U_PORT_L9_IMG_IMGI_D1: u32 = MTK_M4U_ID(9, 0);
pub const M4U_PORT_L9_IMG_IMGBI_D1: u32 = MTK_M4U_ID(9, 1);
pub const M4U_PORT_L9_IMG_DMGI_D1: u32 = MTK_M4U_ID(9, 2);
pub const M4U_PORT_L9_IMG_DEPI_D1: u32 = MTK_M4U_ID(9, 3);
pub const M4U_PORT_L9_IMG_ICE_D1: u32 = MTK_M4U_ID(9, 4);
pub const M4U_PORT_L9_IMG_SMTI_D1: u32 = MTK_M4U_ID(9, 5);
pub const M4U_PORT_L9_IMG_SMTO_D2: u32 = MTK_M4U_ID(9, 6);
pub const M4U_PORT_L9_IMG_SMTO_D1: u32 = MTK_M4U_ID(9, 7);
pub const M4U_PORT_L9_IMG_CRZO_D1: u32 = MTK_M4U_ID(9, 8);
pub const M4U_PORT_L9_IMG_IMG3O_D1: u32 = MTK_M4U_ID(9, 9);
pub const M4U_PORT_L9_IMG_VIPI_D1: u32 = MTK_M4U_ID(9, 10);
pub const M4U_PORT_L9_IMG_SMTI_D5: u32 = MTK_M4U_ID(9, 11);
pub const M4U_PORT_L9_IMG_TIMGO_D1: u32 = MTK_M4U_ID(9, 12);
pub const M4U_PORT_L9_IMG_UFBC_W0: u32 = MTK_M4U_ID(9, 13);
pub const M4U_PORT_L9_IMG_UFBC_R0: u32 = MTK_M4U_ID(9, 14);

/* larb10: null */

/* larb11 */
pub const M4U_PORT_L11_IMG_IMGI_D1: u32 = MTK_M4U_ID(11, 0);
pub const M4U_PORT_L11_IMG_IMGBI_D1: u32 = MTK_M4U_ID(11, 1);
pub const M4U_PORT_L11_IMG_DMGI_D1: u32 = MTK_M4U_ID(11, 2);
pub const M4U_PORT_L11_IMG_DEPI_D1: u32 = MTK_M4U_ID(11, 3);
pub const M4U_PORT_L11_IMG_ICE_D1: u32 = MTK_M4U_ID(11, 4);
pub const M4U_PORT_L11_IMG_SMTI_D1: u32 = MTK_M4U_ID(11, 5);
pub const M4U_PORT_L11_IMG_SMTO_D2: u32 = MTK_M4U_ID(11, 6);
pub const M4U_PORT_L11_IMG_SMTO_D1: u32 = MTK_M4U_ID(11, 7);
pub const M4U_PORT_L11_IMG_CRZO_D1: u32 = MTK_M4U_ID(11, 8);
pub const M4U_PORT_L11_IMG_IMG3O_D1: u32 = MTK_M4U_ID(11, 9);
pub const M4U_PORT_L11_IMG_VIPI_D1: u32 = MTK_M4U_ID(11, 10);
pub const M4U_PORT_L11_IMG_SMTI_D5: u32 = MTK_M4U_ID(11, 11);
pub const M4U_PORT_L11_IMG_TIMGO_D1: u32 = MTK_M4U_ID(11, 12);
pub const M4U_PORT_L11_IMG_UFBC_W0: u32 = MTK_M4U_ID(11, 13);
pub const M4U_PORT_L11_IMG_UFBC_R0: u32 = MTK_M4U_ID(11, 14);
pub const M4U_PORT_L11_IMG_WPE_RDMA1: u32 = MTK_M4U_ID(11, 15);
pub const M4U_PORT_L11_IMG_WPE_RDMA0: u32 = MTK_M4U_ID(11, 16);
pub const M4U_PORT_L11_IMG_WPE_WDMA: u32 = MTK_M4U_ID(11, 17);
pub const M4U_PORT_L11_IMG_MFB_RDMA0: u32 = MTK_M4U_ID(11, 18);
pub const M4U_PORT_L11_IMG_MFB_RDMA1: u32 = MTK_M4U_ID(11, 19);
pub const M4U_PORT_L11_IMG_MFB_RDMA2: u32 = MTK_M4U_ID(11, 20);
pub const M4U_PORT_L11_IMG_MFB_RDMA3: u32 = MTK_M4U_ID(11, 21);
pub const M4U_PORT_L11_IMG_MFB_RDMA4: u32 = MTK_M4U_ID(11, 22);
pub const M4U_PORT_L11_IMG_MFB_RDMA5: u32 = MTK_M4U_ID(11, 23);
pub const M4U_PORT_L11_IMG_MFB_WDMA0: u32 = MTK_M4U_ID(11, 24);
pub const M4U_PORT_L11_IMG_MFB_WDMA1: u32 = MTK_M4U_ID(11, 25);

/* larb12: null */

/* larb13 */
pub const M4U_PORT_L13_CAM_MRAWI: u32 = MTK_M4U_ID(13, 0);
pub const M4U_PORT_L13_CAM_MRAWO0: u32 = MTK_M4U_ID(13, 1);
pub const M4U_PORT_L13_CAM_MRAWO1: u32 = MTK_M4U_ID(13, 2);
pub const M4U_PORT_L13_CAM_CAMSV1: u32 = MTK_M4U_ID(13, 3);
pub const M4U_PORT_L13_CAM_CAMSV2: u32 = MTK_M4U_ID(13, 4);
pub const M4U_PORT_L13_CAM_CAMSV3: u32 = MTK_M4U_ID(13, 5);
pub const M4U_PORT_L13_CAM_CAMSV4: u32 = MTK_M4U_ID(13, 6);
pub const M4U_PORT_L13_CAM_CAMSV5: u32 = MTK_M4U_ID(13, 7);
pub const M4U_PORT_L13_CAM_CAMSV6: u32 = MTK_M4U_ID(13, 8);
pub const M4U_PORT_L13_CAM_CCUI: u32 = MTK_M4U_ID(13, 9);
pub const M4U_PORT_L13_CAM_CCUO: u32 = MTK_M4U_ID(13, 10);
pub const M4U_PORT_L13_CAM_FAKE: u32 = MTK_M4U_ID(13, 11);

/* larb14 */
pub const M4U_PORT_L14_CAM_RESERVE1: u32 = MTK_M4U_ID(14, 0);
pub const M4U_PORT_L14_CAM_RESERVE2: u32 = MTK_M4U_ID(14, 1);
pub const M4U_PORT_L14_CAM_RESERVE3: u32 = MTK_M4U_ID(14, 2);
pub const M4U_PORT_L14_CAM_CAMSV0: u32 = MTK_M4U_ID(14, 3);
pub const M4U_PORT_L14_CAM_CCUI: u32 = MTK_M4U_ID(14, 4);
pub const M4U_PORT_L14_CAM_CCUO: u32 = MTK_M4U_ID(14, 5);

/* larb15: null */

/* larb16 */
pub const M4U_PORT_L16_CAM_IMGO_R1_A: u32 = MTK_M4U_ID(16, 0);
pub const M4U_PORT_L16_CAM_RRZO_R1_A: u32 = MTK_M4U_ID(16, 1);
pub const M4U_PORT_L16_CAM_CQI_R1_A: u32 = MTK_M4U_ID(16, 2);
pub const M4U_PORT_L16_CAM_BPCI_R1_A: u32 = MTK_M4U_ID(16, 3);
pub const M4U_PORT_L16_CAM_YUVO_R1_A: u32 = MTK_M4U_ID(16, 4);
pub const M4U_PORT_L16_CAM_UFDI_R2_A: u32 = MTK_M4U_ID(16, 5);
pub const M4U_PORT_L16_CAM_RAWI_R2_A: u32 = MTK_M4U_ID(16, 6);
pub const M4U_PORT_L16_CAM_RAWI_R3_A: u32 = MTK_M4U_ID(16, 7);
pub const M4U_PORT_L16_CAM_AAO_R1_A: u32 = MTK_M4U_ID(16, 8);
pub const M4U_PORT_L16_CAM_AFO_R1_A: u32 = MTK_M4U_ID(16, 9);
pub const M4U_PORT_L16_CAM_FLKO_R1_A: u32 = MTK_M4U_ID(16, 10);
pub const M4U_PORT_L16_CAM_LCESO_R1_A: u32 = MTK_M4U_ID(16, 11);
pub const M4U_PORT_L16_CAM_CRZO_R1_A: u32 = MTK_M4U_ID(16, 12);
pub const M4U_PORT_L16_CAM_LTMSO_R1_A: u32 = MTK_M4U_ID(16, 13);
pub const M4U_PORT_L16_CAM_RSSO_R1_A: u32 = MTK_M4U_ID(16, 14);
pub const M4U_PORT_L16_CAM_AAHO_R1_A: u32 = MTK_M4U_ID(16, 15);
pub const M4U_PORT_L16_CAM_LSCI_R1_A: u32 = MTK_M4U_ID(16, 16);

/* larb17 */
pub const M4U_PORT_L17_CAM_IMGO_R1_B: u32 = MTK_M4U_ID(17, 0);
pub const M4U_PORT_L17_CAM_RRZO_R1_B: u32 = MTK_M4U_ID(17, 1);
pub const M4U_PORT_L17_CAM_CQI_R1_B: u32 = MTK_M4U_ID(17, 2);
pub const M4U_PORT_L17_CAM_BPCI_R1_B: u32 = MTK_M4U_ID(17, 3);
pub const M4U_PORT_L17_CAM_YUVO_R1_B: u32 = MTK_M4U_ID(17, 4);
pub const M4U_PORT_L17_CAM_UFDI_R2_B: u32 = MTK_M4U_ID(17, 5);
pub const M4U_PORT_L17_CAM_RAWI_R2_B: u32 = MTK_M4U_ID(17, 6);
pub const M4U_PORT_L17_CAM_RAWI_R3_B: u32 = MTK_M4U_ID(17, 7);
pub const M4U_PORT_L17_CAM_AAO_R1_B: u32 = MTK_M4U_ID(17, 8);
pub const M4U_PORT_L17_CAM_AFO_R1_B: u32 = MTK_M4U_ID(17, 9);
pub const M4U_PORT_L17_CAM_FLKO_R1_B: u32 = MTK_M4U_ID(17, 10);
pub const M4U_PORT_L17_CAM_LCESO_R1_B: u32 = MTK_M4U_ID(17, 11);
pub const M4U_PORT_L17_CAM_CRZO_R1_B: u32 = MTK_M4U_ID(17, 12);
pub const M4U_PORT_L17_CAM_LTMSO_R1_B: u32 = MTK_M4U_ID(17, 13);
pub const M4U_PORT_L17_CAM_RSSO_R1_B: u32 = MTK_M4U_ID(17, 14);
pub const M4U_PORT_L17_CAM_AAHO_R1_B: u32 = MTK_M4U_ID(17, 15);
pub const M4U_PORT_L17_CAM_LSCI_R1_B: u32 = MTK_M4U_ID(17, 16);

/* larb18 */
pub const M4U_PORT_L18_CAM_IMGO_R1_C: u32 = MTK_M4U_ID(18, 0);
pub const M4U_PORT_L18_CAM_RRZO_R1_C: u32 = MTK_M4U_ID(18, 1);
pub const M4U_PORT_L18_CAM_CQI_R1_C: u32 = MTK_M4U_ID(18, 2);
pub const M4U_PORT_L18_CAM_BPCI_R1_C: u32 = MTK_M4U_ID(18, 3);
pub const M4U_PORT_L18_CAM_YUVO_R1_C: u32 = MTK_M4U_ID(18, 4);
pub const M4U_PORT_L18_CAM_UFDI_R2_C: u32 = MTK_M4U_ID(18, 5);
pub const M4U_PORT_L18_CAM_RAWI_R2_C: u32 = MTK_M4U_ID(18, 6);
pub const M4U_PORT_L18_CAM_RAWI_R3_C: u32 = MTK_M4U_ID(18, 7);
pub const M4U_PORT_L18_CAM_AAO_R1_C: u32 = MTK_M4U_ID(18, 8);
pub const M4U_PORT_L18_CAM_AFO_R1_C: u32 = MTK_M4U_ID(18, 9);
pub const M4U_PORT_L18_CAM_FLKO_R1_C: u32 = MTK_M4U_ID(18, 10);
pub const M4U_PORT_L18_CAM_LCESO_R1_C: u32 = MTK_M4U_ID(18, 11);
pub const M4U_PORT_L18_CAM_CRZO_R1_C: u32 = MTK_M4U_ID(18, 12);
pub const M4U_PORT_L18_CAM_LTMSO_R1_C: u32 = MTK_M4U_ID(18, 13);
pub const M4U_PORT_L18_CAM_RSSO_R1_C: u32 = MTK_M4U_ID(18, 14);
pub const M4U_PORT_L18_CAM_AAHO_R1_C: u32 = MTK_M4U_ID(18, 15);
pub const M4U_PORT_L18_CAM_LSCI_R1_C: u32 = MTK_M4U_ID(18, 16);

/* larb19 */
pub const M4U_PORT_L19_IPE_DVS_RDMA: u32 = MTK_M4U_ID(19, 0);
pub const M4U_PORT_L19_IPE_DVS_WDMA: u32 = MTK_M4U_ID(19, 1);
pub const M4U_PORT_L19_IPE_DVP_RDMA: u32 = MTK_M4U_ID(19, 2);
pub const M4U_PORT_L19_IPE_DVP_WDMA: u32 = MTK_M4U_ID(19, 3);

/* larb20 */
pub const M4U_PORT_L20_IPE_FDVT_RDA: u32 = MTK_M4U_ID(20, 0);
pub const M4U_PORT_L20_IPE_FDVT_RDB: u32 = MTK_M4U_ID(20, 1);
pub const M4U_PORT_L20_IPE_FDVT_WRA: u32 = MTK_M4U_ID(20, 2);
pub const M4U_PORT_L20_IPE_FDVT_WRB: u32 = MTK_M4U_ID(20, 3);
pub const M4U_PORT_L20_IPE_RSC_RDMA0: u32 = MTK_M4U_ID(20, 4);
pub const M4U_PORT_L20_IPE_RSC_WDMA: u32 = MTK_M4U_ID(20, 5);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
