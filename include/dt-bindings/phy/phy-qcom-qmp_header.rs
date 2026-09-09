/* SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause) */
/*
 * Qualcomm QMP PHY constants
 *
 * Copyright (C) 2022 Linaro Limited
 */

/* Header guard: _DT_BINDINGS_PHY_QMP */

/* QMP USB4-USB3-DP clocks */
pub const QMP_USB43DP_USB3_PIPE_CLK: u32 = 0;
pub const QMP_USB43DP_DP_LINK_CLK: u32 = 1;
pub const QMP_USB43DP_DP_VCO_DIV_CLK: u32 = 2;

/* QMP USB4-USB3-DP PHYs */
pub const QMP_USB43DP_USB3_PHY: u32 = 0;
pub const QMP_USB43DP_DP_PHY: u32 = 1;

/* QMP PCIE PHYs */
pub const QMP_PCIE_PIPE_CLK: u32 = 0;
pub const QMP_PCIE_PHY_AUX_CLK: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
