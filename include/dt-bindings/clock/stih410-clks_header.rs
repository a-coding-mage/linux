/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants clk index STMicroelectronics
 * STiH410 SoC.
 */

// Dependency supplied by the translated stih407 clock bindings.

/* STiH410 introduces new clock outputs compared to STiH407 */

/* CLOCKGEN C0 */
pub const CLK_TX_ICN_HADES: u32 = 32;
pub const CLK_RX_ICN_HADES: u32 = 33;
pub const CLK_ICN_REG_16: u32 = 34;
pub const CLK_PP_HADES: u32 = 35;
pub const CLK_CLUST_HADES: u32 = 36;
pub const CLK_HWPE_HADES: u32 = 37;
pub const CLK_FC_HADES: u32 = 38;

/* CLOCKGEN D0 */
pub const CLK_PCMR10_MASTER: u32 = 4;
pub const CLK_USB2_PHY: u32 = 5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
