/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Interconnect framework driver for i.MX SoC
 *
 * Copyright (c) 2019, BayLibre
 * Copyright (c) 2019-2020, NXP
 * Author: Alexandre Bailon <abailon@baylibre.com>
 */

// Translated from the C header guard:
// #ifndef __DT_BINDINGS_INTERCONNECT_IMX8MM_H

pub const IMX8MM_ICN_NOC: i32 = 1;
pub const IMX8MM_ICS_DRAM: i32 = 2;
pub const IMX8MM_ICS_OCRAM: i32 = 3;
pub const IMX8MM_ICM_A53: i32 = 4;

pub const IMX8MM_ICM_VPU_H1: i32 = 5;
pub const IMX8MM_ICM_VPU_G1: i32 = 6;
pub const IMX8MM_ICM_VPU_G2: i32 = 7;
pub const IMX8MM_ICN_VIDEO: i32 = 8;

pub const IMX8MM_ICM_GPU2D: i32 = 9;
pub const IMX8MM_ICM_GPU3D: i32 = 10;
pub const IMX8MM_ICN_GPU: i32 = 11;

pub const IMX8MM_ICM_CSI: i32 = 12;
pub const IMX8MM_ICM_LCDIF: i32 = 13;
pub const IMX8MM_ICN_MIPI: i32 = 14;

pub const IMX8MM_ICM_USB1: i32 = 15;
pub const IMX8MM_ICM_USB2: i32 = 16;
pub const IMX8MM_ICM_PCIE: i32 = 17;
pub const IMX8MM_ICN_HSIO: i32 = 18;

pub const IMX8MM_ICM_SDMA2: i32 = 19;
pub const IMX8MM_ICM_SDMA3: i32 = 20;
pub const IMX8MM_ICN_AUDIO: i32 = 21;

pub const IMX8MM_ICN_ENET: i32 = 22;
pub const IMX8MM_ICM_ENET: i32 = 23;

pub const IMX8MM_ICN_MAIN: i32 = 24;
pub const IMX8MM_ICM_NAND: i32 = 25;
pub const IMX8MM_ICM_SDMA1: i32 = 26;
pub const IMX8MM_ICM_USDHC1: i32 = 27;
pub const IMX8MM_ICM_USDHC2: i32 = 28;
pub const IMX8MM_ICM_USDHC3: i32 = 29;

// #endif /* __DT_BINDINGS_INTERCONNECT_IMX8MM_H */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
