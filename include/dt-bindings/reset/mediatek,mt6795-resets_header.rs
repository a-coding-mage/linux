/* SPDX-License-Identifier: (GPL-2.0+ OR BSD-2-Clause) */
/*
 * Copyright (c) 2022 Collabora Ltd.
 * Author: AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

/* INFRACFG resets */
pub const MT6795_INFRA_RST0_SCPSYS_RST: i32 = 0;
pub const MT6795_INFRA_RST0_PMIC_WRAP_RST: i32 = 1;
pub const MT6795_INFRA_RST1_MIPI_DSI_RST: i32 = 2;
pub const MT6795_INFRA_RST1_MIPI_CSI_RST: i32 = 3;
pub const MT6795_INFRA_RST1_MM_IOMMU_RST: i32 = 4;

/* MMSYS resets */
pub const MT6795_MMSYS_SW0_RST_B_SMI_COMMON: i32 = 0;
pub const MT6795_MMSYS_SW0_RST_B_SMI_LARB: i32 = 1;
pub const MT6795_MMSYS_SW0_RST_B_CAM_MDP: i32 = 2;
pub const MT6795_MMSYS_SW0_RST_B_MDP_RDMA0: i32 = 3;
pub const MT6795_MMSYS_SW0_RST_B_MDP_RDMA1: i32 = 4;
pub const MT6795_MMSYS_SW0_RST_B_MDP_RSZ0: i32 = 5;
pub const MT6795_MMSYS_SW0_RST_B_MDP_RSZ1: i32 = 6;
pub const MT6795_MMSYS_SW0_RST_B_MDP_RSZ2: i32 = 7;
pub const MT6795_MMSYS_SW0_RST_B_MDP_TDSHP0: i32 = 8;
pub const MT6795_MMSYS_SW0_RST_B_MDP_TDSHP1: i32 = 9;
pub const MT6795_MMSYS_SW0_RST_B_MDP_WDMA: i32 = 10;
pub const MT6795_MMSYS_SW0_RST_B_MDP_WROT0: i32 = 11;
pub const MT6795_MMSYS_SW0_RST_B_MDP_WROT1: i32 = 12;
pub const MT6795_MMSYS_SW0_RST_B_MDP_CROP: i32 = 13;

/* PERICFG resets */
pub const MT6795_PERI_NFI_SW_RST: i32 = 0;
pub const MT6795_PERI_THERM_SW_RST: i32 = 1;
pub const MT6795_PERI_MSDC1_SW_RST: i32 = 2;

/* TOPRGU resets */
pub const MT6795_TOPRGU_INFRA_SW_RST: i32 = 0;
pub const MT6795_TOPRGU_MM_SW_RST: i32 = 1;
pub const MT6795_TOPRGU_MFG_SW_RST: i32 = 2;
pub const MT6795_TOPRGU_VENC_SW_RST: i32 = 3;
pub const MT6795_TOPRGU_VDEC_SW_RST: i32 = 4;
pub const MT6795_TOPRGU_IMG_SW_RST: i32 = 5;
pub const MT6795_TOPRGU_DDRPHY_SW_RST: i32 = 6;
pub const MT6795_TOPRGU_MD_SW_RST: i32 = 7;
pub const MT6795_TOPRGU_INFRA_AO_SW_RST: i32 = 8;
pub const MT6795_TOPRGU_MD_LITE_SW_RST: i32 = 9;
pub const MT6795_TOPRGU_APMIXED_SW_RST: i32 = 10;
pub const MT6795_TOPRGU_PWRAP_SPI_CTL_RST: i32 = 11;
pub const MT6795_TOPRGU_SW_RST_NUM: i32 = 12;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
