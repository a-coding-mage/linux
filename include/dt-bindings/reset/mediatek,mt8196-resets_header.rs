/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2025 Collabora Ltd.
 * Author: AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

/* Original C header guard: _DT_BINDINGS_RESET_CONTROLLER_MT8196 */

/* PEXTP0 resets */
pub const MT8196_PEXTP0_RST0_PCIE0_MAC: u32 = 0;
pub const MT8196_PEXTP0_RST0_PCIE0_PHY: u32 = 1;

/* PEXTP1 resets */
pub const MT8196_PEXTP1_RST0_PCIE1_MAC: u32 = 0;
pub const MT8196_PEXTP1_RST0_PCIE1_PHY: u32 = 1;
pub const MT8196_PEXTP1_RST0_PCIE2_MAC: u32 = 2;
pub const MT8196_PEXTP1_RST0_PCIE2_PHY: u32 = 3;

/* UFS resets */
pub const MT8196_UFSAO_RST0_UFS_MPHY: u32 = 0;
pub const MT8196_UFSAO_RST1_UFS_UNIPRO: u32 = 1;
pub const MT8196_UFSAO_RST1_UFS_CRYPTO: u32 = 2;
pub const MT8196_UFSAO_RST1_UFSHCI: u32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
