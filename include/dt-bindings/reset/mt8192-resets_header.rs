/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2020 MediaTek Inc.
 * Author: Yong Liang <yong.liang@mediatek.com>
 */

/* TOPRGU resets */
pub const MT8192_TOPRGU_MM_SW_RST: u32 = 1;
pub const MT8192_TOPRGU_MFG_SW_RST: u32 = 2;
pub const MT8192_TOPRGU_VENC_SW_RST: u32 = 3;
pub const MT8192_TOPRGU_VDEC_SW_RST: u32 = 4;
pub const MT8192_TOPRGU_IMG_SW_RST: u32 = 5;
pub const MT8192_TOPRGU_MD_SW_RST: u32 = 7;
pub const MT8192_TOPRGU_CONN_SW_RST: u32 = 9;
pub const MT8192_TOPRGU_CONN_MCU_SW_RST: u32 = 12;
pub const MT8192_TOPRGU_IPU0_SW_RST: u32 = 14;
pub const MT8192_TOPRGU_IPU1_SW_RST: u32 = 15;
pub const MT8192_TOPRGU_AUDIO_SW_RST: u32 = 17;
pub const MT8192_TOPRGU_CAMSYS_SW_RST: u32 = 18;
pub const MT8192_TOPRGU_MJC_SW_RST: u32 = 19;
pub const MT8192_TOPRGU_C2K_S2_SW_RST: u32 = 20;
pub const MT8192_TOPRGU_C2K_SW_RST: u32 = 21;
pub const MT8192_TOPRGU_PERI_SW_RST: u32 = 22;
pub const MT8192_TOPRGU_PERI_AO_SW_RST: u32 = 23;

pub const MT8192_TOPRGU_SW_RST_NUM: u32 = 23;

/* MMSYS resets */
pub const MT8192_MMSYS_SW0_RST_B_DISP_DSI0: u32 = 15;

/* INFRA resets */
pub const MT8192_INFRA_RST0_THERM_CTRL_SWRST: u32 = 0;
pub const MT8192_INFRA_RST2_PEXTP_PHY_SWRST: u32 = 1;
pub const MT8192_INFRA_RST3_THERM_CTRL_PTP_SWRST: u32 = 2;
pub const MT8192_INFRA_RST4_PCIE_TOP_SWRST: u32 = 3;
pub const MT8192_INFRA_RST4_THERM_CTRL_MCU_SWRST: u32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
