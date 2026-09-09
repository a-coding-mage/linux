/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2018 MediaTek Inc.
 * Author: Yong Wu <yong.wu@mediatek.com>
 */
// Dependency: MTK_M4U_ID is supplied by <dt-bindings/memory/mtk-memory-port.h>.

pub const M4U_LARB0_ID: u32 = 0;
pub const M4U_LARB1_ID: u32 = 1;
pub const M4U_LARB2_ID: u32 = 2;
pub const M4U_LARB3_ID: u32 = 3;
pub const M4U_LARB4_ID: u32 = 4;
pub const M4U_LARB5_ID: u32 = 5;
pub const M4U_LARB6_ID: u32 = 6;
pub const M4U_LARB7_ID: u32 = 7;

// MTK_M4U_ID(larb, port) == (larb << 5) | port.
pub const M4U_PORT_DISP_OVL0: u32 = (M4U_LARB0_ID << 5) | 0;
pub const M4U_PORT_DISP_2L_OVL0_LARB0: u32 = (M4U_LARB0_ID << 5) | 1;
pub const M4U_PORT_DISP_2L_OVL1_LARB0: u32 = (M4U_LARB0_ID << 5) | 2;
pub const M4U_PORT_DISP_RDMA0: u32 = (M4U_LARB0_ID << 5) | 3;
pub const M4U_PORT_DISP_RDMA1: u32 = (M4U_LARB0_ID << 5) | 4;
pub const M4U_PORT_DISP_WDMA0: u32 = (M4U_LARB0_ID << 5) | 5;
pub const M4U_PORT_MDP_RDMA0: u32 = (M4U_LARB0_ID << 5) | 6;
pub const M4U_PORT_MDP_WROT0: u32 = (M4U_LARB0_ID << 5) | 7;
pub const M4U_PORT_MDP_WDMA0: u32 = (M4U_LARB0_ID << 5) | 8;
pub const M4U_PORT_DISP_FAKE0: u32 = (M4U_LARB0_ID << 5) | 9;

pub const M4U_PORT_HW_VDEC_MC_EXT: u32 = (M4U_LARB1_ID << 5) | 0;
pub const M4U_PORT_HW_VDEC_PP_EXT: u32 = (M4U_LARB1_ID << 5) | 1;
pub const M4U_PORT_HW_VDEC_VLD_EXT: u32 = (M4U_LARB1_ID << 5) | 2;
pub const M4U_PORT_HW_VDEC_AVC_MV_EXT: u32 = (M4U_LARB1_ID << 5) | 3;
pub const M4U_PORT_HW_VDEC_PRED_RD_EXT: u32 = (M4U_LARB1_ID << 5) | 4;
pub const M4U_PORT_HW_VDEC_PRED_WR_EXT: u32 = (M4U_LARB1_ID << 5) | 5;
pub const M4U_PORT_HW_VDEC_PPWRAP_EXT: u32 = (M4U_LARB1_ID << 5) | 6;

pub const M4U_PORT_IMG_IPUO: u32 = (M4U_LARB2_ID << 5) | 0;
pub const M4U_PORT_IMG_IPU3O: u32 = (M4U_LARB2_ID << 5) | 1;
pub const M4U_PORT_IMG_IPUI: u32 = (M4U_LARB2_ID << 5) | 2;

pub const M4U_PORT_CAM_IPUO: u32 = (M4U_LARB3_ID << 5) | 0;
pub const M4U_PORT_CAM_IPU2O: u32 = (M4U_LARB3_ID << 5) | 1;
pub const M4U_PORT_CAM_IPU3O: u32 = (M4U_LARB3_ID << 5) | 2;
pub const M4U_PORT_CAM_IPUI: u32 = (M4U_LARB3_ID << 5) | 3;
pub const M4U_PORT_CAM_IPU2I: u32 = (M4U_LARB3_ID << 5) | 4;

pub const M4U_PORT_VENC_RCPU: u32 = (M4U_LARB4_ID << 5) | 0;
pub const M4U_PORT_VENC_REC: u32 = (M4U_LARB4_ID << 5) | 1;
pub const M4U_PORT_VENC_BSDMA: u32 = (M4U_LARB4_ID << 5) | 2;
pub const M4U_PORT_VENC_SV_COMV: u32 = (M4U_LARB4_ID << 5) | 3;
pub const M4U_PORT_VENC_RD_COMV: u32 = (M4U_LARB4_ID << 5) | 4;
pub const M4U_PORT_JPGENC_RDMA: u32 = (M4U_LARB4_ID << 5) | 5;
pub const M4U_PORT_JPGENC_BSDMA: u32 = (M4U_LARB4_ID << 5) | 6;
pub const M4U_PORT_VENC_CUR_LUMA: u32 = (M4U_LARB4_ID << 5) | 7;
pub const M4U_PORT_VENC_CUR_CHROMA: u32 = (M4U_LARB4_ID << 5) | 8;
pub const M4U_PORT_VENC_REF_LUMA: u32 = (M4U_LARB4_ID << 5) | 9;
pub const M4U_PORT_VENC_REF_CHROMA: u32 = (M4U_LARB4_ID << 5) | 10;

pub const M4U_PORT_CAM_IMGI: u32 = (M4U_LARB5_ID << 5) | 0;
pub const M4U_PORT_CAM_IMG2O: u32 = (M4U_LARB5_ID << 5) | 1;
pub const M4U_PORT_CAM_IMG3O: u32 = (M4U_LARB5_ID << 5) | 2;
pub const M4U_PORT_CAM_VIPI: u32 = (M4U_LARB5_ID << 5) | 3;
pub const M4U_PORT_CAM_LCEI: u32 = (M4U_LARB5_ID << 5) | 4;
pub const M4U_PORT_CAM_SMXI: u32 = (M4U_LARB5_ID << 5) | 5;
pub const M4U_PORT_CAM_SMXO: u32 = (M4U_LARB5_ID << 5) | 6;
pub const M4U_PORT_CAM_WPE0_RDMA1: u32 = (M4U_LARB5_ID << 5) | 7;
pub const M4U_PORT_CAM_WPE0_RDMA0: u32 = (M4U_LARB5_ID << 5) | 8;
pub const M4U_PORT_CAM_WPE0_WDMA: u32 = (M4U_LARB5_ID << 5) | 9;
pub const M4U_PORT_CAM_FDVT_RP: u32 = (M4U_LARB5_ID << 5) | 10;
pub const M4U_PORT_CAM_FDVT_WR: u32 = (M4U_LARB5_ID << 5) | 11;
pub const M4U_PORT_CAM_FDVT_RB: u32 = (M4U_LARB5_ID << 5) | 12;
pub const M4U_PORT_CAM_WPE1_RDMA0: u32 = (M4U_LARB5_ID << 5) | 13;
pub const M4U_PORT_CAM_WPE1_RDMA1: u32 = (M4U_LARB5_ID << 5) | 14;
pub const M4U_PORT_CAM_WPE1_WDMA: u32 = (M4U_LARB5_ID << 5) | 15;
pub const M4U_PORT_CAM_DPE_RDMA: u32 = (M4U_LARB5_ID << 5) | 16;
pub const M4U_PORT_CAM_DPE_WDMA: u32 = (M4U_LARB5_ID << 5) | 17;
pub const M4U_PORT_CAM_MFB_RDMA0: u32 = (M4U_LARB5_ID << 5) | 18;
pub const M4U_PORT_CAM_MFB_RDMA1: u32 = (M4U_LARB5_ID << 5) | 19;
pub const M4U_PORT_CAM_MFB_WDMA: u32 = (M4U_LARB5_ID << 5) | 20;
pub const M4U_PORT_CAM_RSC_RDMA0: u32 = (M4U_LARB5_ID << 5) | 21;
pub const M4U_PORT_CAM_RSC_WDMA: u32 = (M4U_LARB5_ID << 5) | 22;
pub const M4U_PORT_CAM_OWE_RDMA: u32 = (M4U_LARB5_ID << 5) | 23;
pub const M4U_PORT_CAM_OWE_WDMA: u32 = (M4U_LARB5_ID << 5) | 24;

pub const M4U_PORT_CAM_IMGO: u32 = (M4U_LARB6_ID << 5) | 0;
pub const M4U_PORT_CAM_RRZO: u32 = (M4U_LARB6_ID << 5) | 1;
pub const M4U_PORT_CAM_AAO: u32 = (M4U_LARB6_ID << 5) | 2;
pub const M4U_PORT_CAM_AFO: u32 = (M4U_LARB6_ID << 5) | 3;
pub const M4U_PORT_CAM_LSCI0: u32 = (M4U_LARB6_ID << 5) | 4;
pub const M4U_PORT_CAM_LSCI1: u32 = (M4U_LARB6_ID << 5) | 5;
pub const M4U_PORT_CAM_PDO: u32 = (M4U_LARB6_ID << 5) | 6;
pub const M4U_PORT_CAM_BPCI: u32 = (M4U_LARB6_ID << 5) | 7;
pub const M4U_PORT_CAM_LCSO: u32 = (M4U_LARB6_ID << 5) | 8;
pub const M4U_PORT_CAM_CAM_RSSO_A: u32 = (M4U_LARB6_ID << 5) | 9;
pub const M4U_PORT_CAM_UFEO: u32 = (M4U_LARB6_ID << 5) | 10;
pub const M4U_PORT_CAM_SOCO: u32 = (M4U_LARB6_ID << 5) | 11;
pub const M4U_PORT_CAM_SOC1: u32 = (M4U_LARB6_ID << 5) | 12;
pub const M4U_PORT_CAM_SOC2: u32 = (M4U_LARB6_ID << 5) | 13;
pub const M4U_PORT_CAM_CCUI: u32 = (M4U_LARB6_ID << 5) | 14;
pub const M4U_PORT_CAM_CCUO: u32 = (M4U_LARB6_ID << 5) | 15;
pub const M4U_PORT_CAM_RAWI_A: u32 = (M4U_LARB6_ID << 5) | 16;
pub const M4U_PORT_CAM_CCUG: u32 = (M4U_LARB6_ID << 5) | 17;
pub const M4U_PORT_CAM_PSO: u32 = (M4U_LARB6_ID << 5) | 18;
pub const M4U_PORT_CAM_AFO_1: u32 = (M4U_LARB6_ID << 5) | 19;
pub const M4U_PORT_CAM_LSCI_2: u32 = (M4U_LARB6_ID << 5) | 20;
pub const M4U_PORT_CAM_PDI: u32 = (M4U_LARB6_ID << 5) | 21;
pub const M4U_PORT_CAM_FLKO: u32 = (M4U_LARB6_ID << 5) | 22;
pub const M4U_PORT_CAM_LMVO: u32 = (M4U_LARB6_ID << 5) | 23;
pub const M4U_PORT_CAM_UFGO: u32 = (M4U_LARB6_ID << 5) | 24;
pub const M4U_PORT_CAM_SPARE: u32 = (M4U_LARB6_ID << 5) | 25;
pub const M4U_PORT_CAM_SPARE_2: u32 = (M4U_LARB6_ID << 5) | 26;
pub const M4U_PORT_CAM_SPARE_3: u32 = (M4U_LARB6_ID << 5) | 27;
pub const M4U_PORT_CAM_SPARE_4: u32 = (M4U_LARB6_ID << 5) | 28;
pub const M4U_PORT_CAM_SPARE_5: u32 = (M4U_LARB6_ID << 5) | 29;
pub const M4U_PORT_CAM_SPARE_6: u32 = (M4U_LARB6_ID << 5) | 30;

pub const M4U_PORT_CCU0: u32 = (M4U_LARB7_ID << 5) | 0;
pub const M4U_PORT_CCU1: u32 = (M4U_LARB7_ID << 5) | 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
