/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Interconnect framework driver for i.MX SoC
 *
 * Copyright (c) 2019-2020, NXP
 */

// Translated from the C header. The original include guard is omitted because
// Rust items are conventionally protected by their module boundary.

pub const IMX8MQ_ICN_NOC: u32 = 1;
pub const IMX8MQ_ICS_DRAM: u32 = 2;
pub const IMX8MQ_ICS_OCRAM: u32 = 3;
pub const IMX8MQ_ICM_A53: u32 = 4;

pub const IMX8MQ_ICM_VPU: u32 = 5;
pub const IMX8MQ_ICN_VIDEO: u32 = 6;

pub const IMX8MQ_ICM_GPU: u32 = 7;
pub const IMX8MQ_ICN_GPU: u32 = 8;

pub const IMX8MQ_ICM_DCSS: u32 = 9;
pub const IMX8MQ_ICN_DCSS: u32 = 10;

pub const IMX8MQ_ICM_USB1: u32 = 11;
pub const IMX8MQ_ICM_USB2: u32 = 12;
pub const IMX8MQ_ICN_USB: u32 = 13;

pub const IMX8MQ_ICM_CSI1: u32 = 14;
pub const IMX8MQ_ICM_CSI2: u32 = 15;
pub const IMX8MQ_ICM_LCDIF: u32 = 16;
pub const IMX8MQ_ICN_DISPLAY: u32 = 17;

pub const IMX8MQ_ICM_SDMA2: u32 = 18;
pub const IMX8MQ_ICN_AUDIO: u32 = 19;

pub const IMX8MQ_ICN_ENET: u32 = 20;
pub const IMX8MQ_ICM_ENET: u32 = 21;

pub const IMX8MQ_ICM_SDMA1: u32 = 22;
pub const IMX8MQ_ICM_NAND: u32 = 23;
pub const IMX8MQ_ICM_USDHC1: u32 = 24;
pub const IMX8MQ_ICM_USDHC2: u32 = 25;
pub const IMX8MQ_ICM_PCIE1: u32 = 26;
pub const IMX8MQ_ICM_PCIE2: u32 = 27;
pub const IMX8MQ_ICN_MAIN: u32 = 28;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
