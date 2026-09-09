/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants clk index STMicroelectronics
 * STiH418 SoC.
 */

// Dependency: declarations from "stih410-clks.h" are supplied externally.

/* STiH418 introduces new clock outputs compared to STiH410 */

/* CLOCKGEN C0 */
pub const CLK_PROC_BDISP_0: i32 = 14;
pub const CLK_PROC_BDISP_1: i32 = 15;
pub const CLK_TX_ICN_1: i32 = 23;
pub const CLK_ETH_PHYREF: i32 = 27;
pub const CLK_PP_HEVC: i32 = 35;
pub const CLK_CLUST_HEVC: i32 = 36;
pub const CLK_HWPE_HEVC: i32 = 37;
pub const CLK_FC_HEVC: i32 = 38;
pub const CLK_PROC_MIXER: i32 = 39;
pub const CLK_PROC_SC: i32 = 40;
pub const CLK_AVSP_HEVC: i32 = 41;

/* CLOCKGEN D2 */
// C preprocessor undefinitions: CLK_PIX_PIP, CLK_PIX_GDP1, CLK_PIX_GDP2,
// CLK_PIX_GDP3, and CLK_PIX_GDP4 are intentionally replaced by the including
// translation unit's definitions.

pub const CLK_TMDS_HDMI_DIV2: i32 = 5;
pub const CLK_VP9: i32 = 47;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
