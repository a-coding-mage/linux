/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2014 Ulrich Hecht
 */

// Translated from the C header; the original include guard is omitted.

/* CPG */
pub const SH73A0_CLK_MAIN: u32 = 0;
pub const SH73A0_CLK_PLL0: u32 = 1;
pub const SH73A0_CLK_PLL1: u32 = 2;
pub const SH73A0_CLK_PLL2: u32 = 3;
pub const SH73A0_CLK_PLL3: u32 = 4;
pub const SH73A0_CLK_DSI0PHY: u32 = 5;
pub const SH73A0_CLK_DSI1PHY: u32 = 6;
pub const SH73A0_CLK_ZG: u32 = 7;
pub const SH73A0_CLK_M3: u32 = 8;
pub const SH73A0_CLK_B: u32 = 9;
pub const SH73A0_CLK_M1: u32 = 10;
pub const SH73A0_CLK_M2: u32 = 11;
pub const SH73A0_CLK_Z: u32 = 12;
pub const SH73A0_CLK_ZX: u32 = 13;
pub const SH73A0_CLK_HP: u32 = 14;

/* MSTP0 */
pub const SH73A0_CLK_IIC2: u32 = 1;
pub const SH73A0_CLK_MSIOF0: u32 = 0;

/* MSTP1 */
pub const SH73A0_CLK_CEU1: u32 = 29;
pub const SH73A0_CLK_CSI2_RX1: u32 = 28;
pub const SH73A0_CLK_CEU0: u32 = 27;
pub const SH73A0_CLK_CSI2_RX0: u32 = 26;
pub const SH73A0_CLK_TMU0: u32 = 25;
pub const SH73A0_CLK_DSITX0: u32 = 18;
pub const SH73A0_CLK_IIC0: u32 = 16;
pub const SH73A0_CLK_SGX: u32 = 12;
pub const SH73A0_CLK_LCDC0: u32 = 0;

/* MSTP2 */
pub const SH73A0_CLK_SCIFA7: u32 = 19;
pub const SH73A0_CLK_SY_DMAC: u32 = 18;
pub const SH73A0_CLK_MP_DMAC: u32 = 17;
pub const SH73A0_CLK_MSIOF3: u32 = 15;
pub const SH73A0_CLK_MSIOF1: u32 = 8;
pub const SH73A0_CLK_SCIFA5: u32 = 7;
pub const SH73A0_CLK_SCIFB: u32 = 6;
pub const SH73A0_CLK_MSIOF2: u32 = 5;
pub const SH73A0_CLK_SCIFA0: u32 = 4;
pub const SH73A0_CLK_SCIFA1: u32 = 3;
pub const SH73A0_CLK_SCIFA2: u32 = 2;
pub const SH73A0_CLK_SCIFA3: u32 = 1;
pub const SH73A0_CLK_SCIFA4: u32 = 0;

/* MSTP3 */
pub const SH73A0_CLK_SCIFA6: u32 = 31;
pub const SH73A0_CLK_CMT1: u32 = 29;
pub const SH73A0_CLK_FSI: u32 = 28;
pub const SH73A0_CLK_IRDA: u32 = 25;
pub const SH73A0_CLK_IIC1: u32 = 23;
pub const SH73A0_CLK_USB: u32 = 22;
pub const SH73A0_CLK_FLCTL: u32 = 15;
pub const SH73A0_CLK_SDHI0: u32 = 14;
pub const SH73A0_CLK_SDHI1: u32 = 13;
pub const SH73A0_CLK_MMCIF0: u32 = 12;
pub const SH73A0_CLK_SDHI2: u32 = 11;
pub const SH73A0_CLK_TPU0: u32 = 4;
pub const SH73A0_CLK_TPU1: u32 = 3;
pub const SH73A0_CLK_TPU2: u32 = 2;
pub const SH73A0_CLK_TPU3: u32 = 1;
pub const SH73A0_CLK_TPU4: u32 = 0;

/* MSTP4 */
pub const SH73A0_CLK_IIC3: u32 = 11;
pub const SH73A0_CLK_IIC4: u32 = 10;
pub const SH73A0_CLK_KEYSC: u32 = 3;

/* MSTP5 */
pub const SH73A0_CLK_INTCA0: u32 = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
