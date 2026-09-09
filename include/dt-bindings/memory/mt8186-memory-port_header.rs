/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of mt8186-memory-port.h. The external MTK_M4U_ID macro is represented by its direct ID calculation. */

/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2022 MediaTek Inc.
 *
 * Author: Anan Sun <anan.sun@mediatek.com>
 * Author: Yong Wu <yong.wu@mediatek.com>
 */


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
 * disp         0 ~ 4G                  larb0/1/2
 * vcodec      4G ~ 8G                  larb4/7
 * cam/mdp     8G ~ 12G                 the other larbs.
 * N/A         12G ~ 16G
 * CCU0   0x24000_0000 ~ 0x243ff_ffff   larb13: port 9/10
 * CCU1   0x24400_0000 ~ 0x247ff_ffff   larb14: port 4/5
 */

/* MM IOMMU ports */
/* LARB 0 -- MMSYS */
pub const IOMMU_PORT_L0_DISP_POSTMASK0: u32 = ((0u32) << 5) | (u32);
pub const IOMMU_PORT_L0_REVERSED: u32 = ((0u32) << 5) | (u32);
pub const IOMMU_PORT_L0_OVL_RDMA0: u32 = ((0u32) << 5) | (u32);
pub const IOMMU_PORT_L0_DISP_FAKE0: u32 = ((0u32) << 5) | (u32);

/* LARB 1 -- MMSYS */
pub const IOMMU_PORT_L1_DISP_RDMA1: u32 = ((1u32) << 5) | (u32);
pub const IOMMU_PORT_L1_OVL_2L_RDMA0: u32 = ((1u32) << 5) | (u32);
pub const IOMMU_PORT_L1_DISP_RDMA0: u32 = ((1u32) << 5) | (u32);
pub const IOMMU_PORT_L1_DISP_WDMA0: u32 = ((1u32) << 5) | (u32);
pub const IOMMU_PORT_L1_DISP_FAKE1: u32 = ((1u32) << 5) | (u32);

/* LARB 2 -- MMSYS */
pub const IOMMU_PORT_L2_MDP_RDMA0: u32 = ((2u32) << 5) | (u32);
pub const IOMMU_PORT_L2_MDP_RDMA1: u32 = ((2u32) << 5) | (u32);
pub const IOMMU_PORT_L2_MDP_WROT0: u32 = ((2u32) << 5) | (u32);
pub const IOMMU_PORT_L2_MDP_WROT1: u32 = ((2u32) << 5) | (u32);
pub const IOMMU_PORT_L2_DISP_FAKE0: u32 = ((2u32) << 5) | (u32);

/* LARB 4 -- VDEC */
pub const IOMMU_PORT_L4_HW_VDEC_MC_EXT: u32 = ((4u32) << 5) | (u32);
pub const IOMMU_PORT_L4_HW_VDEC_UFO_EXT: u32 = ((4u32) << 5) | (u32);
pub const IOMMU_PORT_L4_HW_VDEC_PP_EXT: u32 = ((4u32) << 5) | (u32);
pub const IOMMU_PORT_L4_HW_VDEC_PRED_RD_EXT: u32 = ((4u32) << 5) | (u32);
pub const IOMMU_PORT_L4_HW_VDEC_PRED_WR_EXT: u32 = ((4u32) << 5) | (u32);
pub const IOMMU_PORT_L4_HW_VDEC_PPWRAP_EXT: u32 = ((4u32) << 5) | (u32);
pub const IOMMU_PORT_L4_HW_VDEC_TILE_EXT: u32 = ((4u32) << 5) | (u32);
pub const IOMMU_PORT_L4_HW_VDEC_VLD_EXT: u32 = ((4u32) << 5) | (u32);
pub const IOMMU_PORT_L4_HW_VDEC_VLD2_EXT: u32 = ((4u32) << 5) | (u32);
pub const IOMMU_PORT_L4_HW_VDEC_AVC_MV_EXT: u32 = ((4u32) << 5) | (u32);
pub const IOMMU_PORT_L4_HW_VDEC_UFO_ENC_EXT: u32 = ((4u32) << 5) | (u32);
pub const IOMMU_PORT_L4_HW_VDEC_RG_CTRL_DMA_EXT: u32 = ((4u32) << 5) | (u32);
pub const IOMMU_PORT_L4_HW_MINI_MDP_R0_EXT: u32 = ((4u32) << 5) | (u32);
pub const IOMMU_PORT_L4_HW_MINI_MDP_W0_EXT: u32 = ((4u32) << 5) | (u32);

/* LARB 7 -- VENC */
pub const IOMMU_PORT_L7_VENC_RCPU: u32 = ((7u32) << 5) | (u32);
pub const IOMMU_PORT_L7_VENC_REC: u32 = ((7u32) << 5) | (u32);
pub const IOMMU_PORT_L7_VENC_BSDMA: u32 = ((7u32) << 5) | (u32);
pub const IOMMU_PORT_L7_VENC_SV_COMV: u32 = ((7u32) << 5) | (u32);
pub const IOMMU_PORT_L7_VENC_RD_COMV: u32 = ((7u32) << 5) | (u32);
pub const IOMMU_PORT_L7_VENC_CUR_LUMA: u32 = ((7u32) << 5) | (u32);
pub const IOMMU_PORT_L7_VENC_CUR_CHROMA: u32 = ((7u32) << 5) | (u32);
pub const IOMMU_PORT_L7_VENC_REF_LUMA: u32 = ((7u32) << 5) | (u32);
pub const IOMMU_PORT_L7_VENC_REF_CHROMA: u32 = ((7u32) << 5) | (u32);
pub const IOMMU_PORT_L7_JPGENC_Y_RDMA: u32 = ((7u32) << 5) | (u32);
pub const IOMMU_PORT_L7_JPGENC_C_RDMA: u32 = ((7u32) << 5) | (u32);
pub const IOMMU_PORT_L7_JPGENC_Q_TABLE: u32 = ((7u32) << 5) | (u32);
pub const IOMMU_PORT_L7_JPGENC_BSDMA: u32 = ((7u32) << 5) | (u32);

/* LARB 8 -- WPE */
pub const IOMMU_PORT_L8_WPE_RDMA_0: u32 = ((8u32) << 5) | (u32);
pub const IOMMU_PORT_L8_WPE_RDMA_1: u32 = ((8u32) << 5) | (u32);
pub const IOMMU_PORT_L8_WPE_WDMA_0: u32 = ((8u32) << 5) | (u32);

/* LARB 9 -- IMG-1 */
pub const IOMMU_PORT_L9_IMG_IMGI_D1: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_IMGBI_D1: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_DMGI_D1: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_DEPI_D1: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_LCE_D1: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_SMTI_D1: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_SMTO_D2: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_SMTO_D1: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_CRZO_D1: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_IMG3O_D1: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_VIPI_D1: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_SMTI_D5: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_TIMGO_D1: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_UFBC_W0: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_UFBC_R0: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_WPE_RDMA1: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_WPE_RDMA0: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_WPE_WDMA: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_MFB_RDMA0: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_MFB_RDMA1: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_MFB_RDMA2: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_MFB_RDMA3: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_MFB_RDMA4: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_MFB_RDMA5: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_MFB_WDMA0: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_MFB_WDMA1: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_RESERVE6: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_RESERVE7: u32 = ((9u32) << 5) | (u32);
pub const IOMMU_PORT_L9_IMG_RESERVE8: u32 = ((9u32) << 5) | (u32);

/* LARB 11 -- IMG-2 */
pub const IOMMU_PORT_L11_IMG_IMGI_D1: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_IMGBI_D1: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_DMGI_D1: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_DEPI_D1: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_LCE_D1: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_SMTI_D1: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_SMTO_D2: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_SMTO_D1: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_CRZO_D1: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_IMG3O_D1: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_VIPI_D1: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_SMTI_D5: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_TIMGO_D1: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_UFBC_W0: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_UFBC_R0: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_WPE_RDMA1: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_WPE_RDMA0: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_WPE_WDMA: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_MFB_RDMA0: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_MFB_RDMA1: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_MFB_RDMA2: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_MFB_RDMA3: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_MFB_RDMA4: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_MFB_RDMA5: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_MFB_WDMA0: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_MFB_WDMA1: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_RESERVE6: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_RESERVE7: u32 = ((11u32) << 5) | (u32);
pub const IOMMU_PORT_L11_IMG_RESERVE8: u32 = ((11u32) << 5) | (u32);

/* LARB 13 -- CAM */
pub const IOMMU_PORT_L13_CAM_MRAWI: u32 = ((13u32) << 5) | (u32);
pub const IOMMU_PORT_L13_CAM_MRAWO_0: u32 = ((13u32) << 5) | (u32);
pub const IOMMU_PORT_L13_CAM_MRAWO_1: u32 = ((13u32) << 5) | (u32);
pub const IOMMU_PORT_L13_CAM_CAMSV_4: u32 = ((13u32) << 5) | (u32);
pub const IOMMU_PORT_L13_CAM_CAMSV_5: u32 = ((13u32) << 5) | (u32);
pub const IOMMU_PORT_L13_CAM_CAMSV_6: u32 = ((13u32) << 5) | (u32);
pub const IOMMU_PORT_L13_CAM_CCUI: u32 = ((13u32) << 5) | (u32);
pub const IOMMU_PORT_L13_CAM_CCUO: u32 = ((13u32) << 5) | (u32);
pub const IOMMU_PORT_L13_CAM_FAKE: u32 = ((13u32) << 5) | (u32);

/* LARB 14 -- CAM */
pub const IOMMU_PORT_L14_CAM_CCUI: u32 = ((14u32) << 5) | (u32);
pub const IOMMU_PORT_L14_CAM_CCUO: u32 = ((14u32) << 5) | (u32);

/* LARB 16 -- RAW-A */
pub const IOMMU_PORT_L16_CAM_IMGO_R1_A: u32 = ((16u32) << 5) | (u32);
pub const IOMMU_PORT_L16_CAM_RRZO_R1_A: u32 = ((16u32) << 5) | (u32);
pub const IOMMU_PORT_L16_CAM_CQI_R1_A: u32 = ((16u32) << 5) | (u32);
pub const IOMMU_PORT_L16_CAM_BPCI_R1_A: u32 = ((16u32) << 5) | (u32);
pub const IOMMU_PORT_L16_CAM_YUVO_R1_A: u32 = ((16u32) << 5) | (u32);
pub const IOMMU_PORT_L16_CAM_UFDI_R2_A: u32 = ((16u32) << 5) | (u32);
pub const IOMMU_PORT_L16_CAM_RAWI_R2_A: u32 = ((16u32) << 5) | (u32);
pub const IOMMU_PORT_L16_CAM_RAWI_R3_A: u32 = ((16u32) << 5) | (u32);
pub const IOMMU_PORT_L16_CAM_AAO_R1_A: u32 = ((16u32) << 5) | (u32);
pub const IOMMU_PORT_L16_CAM_AFO_R1_A: u32 = ((16u32) << 5) | (u32);
pub const IOMMU_PORT_L16_CAM_FLKO_R1_A: u32 = ((16u32) << 5) | (u32);
pub const IOMMU_PORT_L16_CAM_LCESO_R1_A: u32 = ((16u32) << 5) | (u32);
pub const IOMMU_PORT_L16_CAM_CRZO_R1_A: u32 = ((16u32) << 5) | (u32);
pub const IOMMU_PORT_L16_CAM_LTMSO_R1_A: u32 = ((16u32) << 5) | (u32);
pub const IOMMU_PORT_L16_CAM_RSSO_R1_A: u32 = ((16u32) << 5) | (u32);
pub const IOMMU_PORT_L16_CAM_AAHO_R1_A: u32 = ((16u32) << 5) | (u32);
pub const IOMMU_PORT_L16_CAM_LSCI_R1_A: u32 = ((16u32) << 5) | (u32);

/* LARB 17 -- RAW-B */
pub const IOMMU_PORT_L17_CAM_IMGO_R1_B: u32 = ((17u32) << 5) | (u32);
pub const IOMMU_PORT_L17_CAM_RRZO_R1_B: u32 = ((17u32) << 5) | (u32);
pub const IOMMU_PORT_L17_CAM_CQI_R1_B: u32 = ((17u32) << 5) | (u32);
pub const IOMMU_PORT_L17_CAM_BPCI_R1_B: u32 = ((17u32) << 5) | (u32);
pub const IOMMU_PORT_L17_CAM_YUVO_R1_B: u32 = ((17u32) << 5) | (u32);
pub const IOMMU_PORT_L17_CAM_UFDI_R2_B: u32 = ((17u32) << 5) | (u32);
pub const IOMMU_PORT_L17_CAM_RAWI_R2_B: u32 = ((17u32) << 5) | (u32);
pub const IOMMU_PORT_L17_CAM_RAWI_R3_B: u32 = ((17u32) << 5) | (u32);
pub const IOMMU_PORT_L17_CAM_AAO_R1_B: u32 = ((17u32) << 5) | (u32);
pub const IOMMU_PORT_L17_CAM_AFO_R1_B: u32 = ((17u32) << 5) | (u32);
pub const IOMMU_PORT_L17_CAM_FLKO_R1_B: u32 = ((17u32) << 5) | (u32);
pub const IOMMU_PORT_L17_CAM_LCESO_R1_B: u32 = ((17u32) << 5) | (u32);
pub const IOMMU_PORT_L17_CAM_CRZO_R1_B: u32 = ((17u32) << 5) | (u32);
pub const IOMMU_PORT_L17_CAM_LTMSO_R1_B: u32 = ((17u32) << 5) | (u32);
pub const IOMMU_PORT_L17_CAM_RSSO_R1_B: u32 = ((17u32) << 5) | (u32);
pub const IOMMU_PORT_L17_CAM_AAHO_R1_B: u32 = ((17u32) << 5) | (u32);
pub const IOMMU_PORT_L17_CAM_LSCI_R1_B: u32 = ((17u32) << 5) | (u32);

/* LARB 19 -- IPE */
pub const IOMMU_PORT_L19_IPE_DVS_RDMA: u32 = ((19u32) << 5) | (u32);
pub const IOMMU_PORT_L19_IPE_DVS_WDMA: u32 = ((19u32) << 5) | (u32);
pub const IOMMU_PORT_L19_IPE_DVP_RDMA: u32 = ((19u32) << 5) | (u32);
pub const IOMMU_PORT_L19_IPE_DVP_WDMA: u32 = ((19u32) << 5) | (u32);

/* LARB 20 -- IPE */
pub const IOMMU_PORT_L20_IPE_FDVT_RDA: u32 = ((20u32) << 5) | (u32);
pub const IOMMU_PORT_L20_IPE_FDVT_RDB: u32 = ((20u32) << 5) | (u32);
pub const IOMMU_PORT_L20_IPE_FDVT_WRA: u32 = ((20u32) << 5) | (u32);
pub const IOMMU_PORT_L20_IPE_FDVT_WRB: u32 = ((20u32) << 5) | (u32);
pub const IOMMU_PORT_L20_IPE_RSC_RDMA0: u32 = ((20u32) << 5) | (u32);
pub const IOMMU_PORT_L20_IPE_RSC_WDMA: u32 = ((20u32) << 5) | (u32);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
