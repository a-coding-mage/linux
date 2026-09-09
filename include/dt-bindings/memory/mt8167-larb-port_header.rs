/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2020 MediaTek Inc.
 * Copyright (c) 2020 BayLibre, SAS
 * Author: Honghui Zhang <honghui.zhang@mediatek.com>
 * Author: Fabien Parent <fparent@baylibre.com>
 */

// Dependency supplied by the corresponding memory-port bindings.

pub const M4U_LARB0_ID: u32 = 0;
pub const M4U_LARB1_ID: u32 = 1;
pub const M4U_LARB2_ID: u32 = 2;

/* larb0 */
pub const M4U_PORT_DISP_OVL0: u32 = MTK_M4U_ID!(M4U_LARB0_ID, 0);
pub const M4U_PORT_DISP_RDMA0: u32 = MTK_M4U_ID!(M4U_LARB0_ID, 1);
pub const M4U_PORT_DISP_WDMA0: u32 = MTK_M4U_ID!(M4U_LARB0_ID, 2);
pub const M4U_PORT_DISP_RDMA1: u32 = MTK_M4U_ID!(M4U_LARB0_ID, 3);
pub const M4U_PORT_MDP_RDMA: u32 = MTK_M4U_ID!(M4U_LARB0_ID, 4);
pub const M4U_PORT_MDP_WDMA: u32 = MTK_M4U_ID!(M4U_LARB0_ID, 5);
pub const M4U_PORT_MDP_WROT: u32 = MTK_M4U_ID!(M4U_LARB0_ID, 6);
pub const M4U_PORT_DISP_FAKE: u32 = MTK_M4U_ID!(M4U_LARB0_ID, 7);

/* larb1 */
pub const M4U_PORT_CAM_IMGO: u32 = MTK_M4U_ID!(M4U_LARB1_ID, 0);
pub const M4U_PORT_CAM_IMG2O: u32 = MTK_M4U_ID!(M4U_LARB1_ID, 1);
pub const M4U_PORT_CAM_LSCI: u32 = MTK_M4U_ID!(M4U_LARB1_ID, 2);
pub const M4U_PORT_CAM_ESFKO: u32 = MTK_M4U_ID!(M4U_LARB1_ID, 3);
pub const M4U_PORT_CAM_AAO: u32 = MTK_M4U_ID!(M4U_LARB1_ID, 4);
pub const M4U_PORT_VENC_REC: u32 = MTK_M4U_ID!(M4U_LARB1_ID, 5);
pub const M4U_PORT_VENC_BSDMA: u32 = MTK_M4U_ID!(M4U_LARB1_ID, 6);
pub const M4U_PORT_VENC_RD_COMV: u32 = MTK_M4U_ID!(M4U_LARB1_ID, 7);
pub const M4U_PORT_CAM_IMGI: u32 = MTK_M4U_ID!(M4U_LARB1_ID, 8);
pub const M4U_PORT_VENC_CUR_LUMA: u32 = MTK_M4U_ID!(M4U_LARB1_ID, 9);
pub const M4U_PORT_VENC_CUR_CHROMA: u32 = MTK_M4U_ID!(M4U_LARB1_ID, 10);
pub const M4U_PORT_VENC_REF_LUMA: u32 = MTK_M4U_ID!(M4U_LARB1_ID, 11);
pub const M4U_PORT_VENC_REF_CHROMA: u32 = MTK_M4U_ID!(M4U_LARB1_ID, 12);

/* larb2 */
pub const M4U_PORT_HW_VDEC_MC_EXT: u32 = MTK_M4U_ID!(M4U_LARB2_ID, 0);
pub const M4U_PORT_HW_VDEC_PP_EXT: u32 = MTK_M4U_ID!(M4U_LARB2_ID, 1);
pub const M4U_PORT_HW_VDEC_VLD_EXT: u32 = MTK_M4U_ID!(M4U_LARB2_ID, 2);
pub const M4U_PORT_HW_VDEC_AVC_MV_EXT: u32 = MTK_M4U_ID!(M4U_LARB2_ID, 3);
pub const M4U_PORT_HW_VDEC_PRED_RD_EXT: u32 = MTK_M4U_ID!(M4U_LARB2_ID, 4);
pub const M4U_PORT_HW_VDEC_PRED_WR_EXT: u32 = MTK_M4U_ID!(M4U_LARB2_ID, 5);
pub const M4U_PORT_HW_VDEC_PPWRAP_EXT: u32 = MTK_M4U_ID!(M4U_LARB2_ID, 6);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
