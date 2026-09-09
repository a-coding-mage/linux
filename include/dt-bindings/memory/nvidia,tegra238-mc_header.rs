/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/* Copyright (c) 2026, NVIDIA CORPORATION. All rights reserved. */

/* special clients */
pub const TEGRA238_SID_INVALID: u32 = 0x0;
pub const TEGRA238_SID_PASSTHROUGH: u32 = 0x7f;

/* ISO stream IDs */
pub const TEGRA238_SID_ISO_NVDISPLAY: u32 = 0x1;
pub const TEGRA238_SID_ISO_APE0: u32 = 0x2;
pub const TEGRA238_SID_ISO_APE1: u32 = 0x3;

/* NISO stream IDs */
pub const TEGRA238_SID_AON: u32 = 0x1;
pub const TEGRA238_SID_BPMP: u32 = 0x2;
pub const TEGRA238_SID_ETR: u32 = 0x3;
pub const TEGRA238_SID_FDE: u32 = 0x4;
pub const TEGRA238_SID_HC: u32 = 0x5;
pub const TEGRA238_SID_HDA: u32 = 0x6;
pub const TEGRA238_SID_NVDEC: u32 = 0x7;
pub const TEGRA238_SID_NVDISPLAY: u32 = 0x8;
pub const TEGRA238_SID_NVENC: u32 = 0x9;
pub const TEGRA238_SID_OFA: u32 = 0xa;
pub const TEGRA238_SID_PCIE0: u32 = 0xb;
pub const TEGRA238_SID_PCIE1: u32 = 0xc;
pub const TEGRA238_SID_PCIE2: u32 = 0xd;
pub const TEGRA238_SID_PCIE3: u32 = 0xe;
pub const TEGRA238_SID_HWMP_PMA: u32 = 0xf;
pub const TEGRA238_SID_PSC: u32 = 0x10;
pub const TEGRA238_SID_SDMMC1A: u32 = 0x11;
pub const TEGRA238_SID_SDMMC4A: u32 = 0x12;
pub const TEGRA238_SID_SES_SE0: u32 = 0x13;
pub const TEGRA238_SID_SES_SE1: u32 = 0x14;
pub const TEGRA238_SID_SES_SE2: u32 = 0x15;
pub const TEGRA238_SID_SEU1_SE0: u32 = 0x16;
pub const TEGRA238_SID_SEU1_SE1: u32 = 0x17;
pub const TEGRA238_SID_SEU1_SE2: u32 = 0x18;
pub const TEGRA238_SID_TSEC: u32 = 0x19;
pub const TEGRA238_SID_UFSHC: u32 = 0x1a;
pub const TEGRA238_SID_VIC: u32 = 0x1b;
pub const TEGRA238_SID_XUSB_HOST: u32 = 0x1c;
pub const TEGRA238_SID_XUSB_DEV: u32 = 0x1d;
pub const TEGRA238_SID_GPCDMA_0: u32 = 0x1e;
pub const TEGRA238_SID_SMMU_TEST: u32 = 0x1f;

/* Host1x virtualization clients. */
pub const TEGRA238_SID_HOST1X_CTX0: u32 = 0x20;
pub const TEGRA238_SID_HOST1X_CTX1: u32 = 0x21;
pub const TEGRA238_SID_HOST1X_CTX2: u32 = 0x22;
pub const TEGRA238_SID_HOST1X_CTX3: u32 = 0x23;
pub const TEGRA238_SID_HOST1X_CTX4: u32 = 0x24;
pub const TEGRA238_SID_HOST1X_CTX5: u32 = 0x25;
pub const TEGRA238_SID_HOST1X_CTX6: u32 = 0x26;
pub const TEGRA238_SID_HOST1X_CTX7: u32 = 0x27;

pub const TEGRA238_SID_XUSB_VF0: u32 = 0x28;
pub const TEGRA238_SID_XUSB_VF1: u32 = 0x29;
pub const TEGRA238_SID_XUSB_VF2: u32 = 0x2a;
pub const TEGRA238_SID_XUSB_VF3: u32 = 0x2b;

/* Host1x command buffers */
pub const TEGRA238_SID_HC_VM0: u32 = 0x2c;
pub const TEGRA238_SID_HC_VM1: u32 = 0x2d;
pub const TEGRA238_SID_HC_VM2: u32 = 0x2e;
pub const TEGRA238_SID_HC_VM3: u32 = 0x2f;
pub const TEGRA238_SID_HC_VM4: u32 = 0x30;
pub const TEGRA238_SID_HC_VM5: u32 = 0x31;
pub const TEGRA238_SID_HC_VM6: u32 = 0x32;
pub const TEGRA238_SID_HC_VM7: u32 = 0x33;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
