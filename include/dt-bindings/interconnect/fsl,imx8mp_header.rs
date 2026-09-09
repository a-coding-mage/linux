/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * Interconnect framework driver for i.MX SoC
 *
 * Copyright 2022 NXP
 * Peng Fan <peng.fan@nxp.com>
 */

pub const IMX8MP_ICN_NOC: u32 = 0;
pub const IMX8MP_ICN_MAIN: u32 = 1;
pub const IMX8MP_ICS_DRAM: u32 = 2;
pub const IMX8MP_ICS_OCRAM: u32 = 3;
pub const IMX8MP_ICM_A53: u32 = 4;
pub const IMX8MP_ICM_SUPERMIX: u32 = 5;
pub const IMX8MP_ICM_GIC: u32 = 6;
pub const IMX8MP_ICM_MLMIX: u32 = 7;

pub const IMX8MP_ICN_AUDIO: u32 = 8;
pub const IMX8MP_ICM_DSP: u32 = 9;
pub const IMX8MP_ICM_SDMA2PER: u32 = 10;
pub const IMX8MP_ICM_SDMA2BURST: u32 = 11;
pub const IMX8MP_ICM_SDMA3PER: u32 = 12;
pub const IMX8MP_ICM_SDMA3BURST: u32 = 13;
pub const IMX8MP_ICM_EDMA: u32 = 14;

pub const IMX8MP_ICN_GPU: u32 = 15;
pub const IMX8MP_ICM_GPU2D: u32 = 16;
pub const IMX8MP_ICM_GPU3D: u32 = 17;

pub const IMX8MP_ICN_HDMI: u32 = 18;
pub const IMX8MP_ICM_HRV: u32 = 19;
pub const IMX8MP_ICM_LCDIF_HDMI: u32 = 20;
pub const IMX8MP_ICM_HDCP: u32 = 21;

pub const IMX8MP_ICN_HSIO: u32 = 22;
pub const IMX8MP_ICM_NOC_PCIE: u32 = 23;
pub const IMX8MP_ICM_USB1: u32 = 24;
pub const IMX8MP_ICM_USB2: u32 = 25;
pub const IMX8MP_ICM_PCIE: u32 = 26;

pub const IMX8MP_ICN_MEDIA: u32 = 27;
pub const IMX8MP_ICM_LCDIF_RD: u32 = 28;
pub const IMX8MP_ICM_LCDIF_WR: u32 = 29;
pub const IMX8MP_ICM_ISI0: u32 = 30;
pub const IMX8MP_ICM_ISI1: u32 = 31;
pub const IMX8MP_ICM_ISI2: u32 = 32;
pub const IMX8MP_ICM_ISP0: u32 = 33;
pub const IMX8MP_ICM_ISP1: u32 = 34;
pub const IMX8MP_ICM_DWE: u32 = 35;

pub const IMX8MP_ICN_VIDEO: u32 = 36;
pub const IMX8MP_ICM_VPU_G1: u32 = 37;
pub const IMX8MP_ICM_VPU_G2: u32 = 38;
pub const IMX8MP_ICM_VPU_H1: u32 = 39;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
