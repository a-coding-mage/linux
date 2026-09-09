/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2022-2023 NVIDIA CORPORATION.  All rights reserved.
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tegra_icc_client_type {
	TEGRA_ICC_NONE,
	TEGRA_ICC_NISO,
	TEGRA_ICC_ISO_DISPLAY,
	TEGRA_ICC_ISO_VI,
	TEGRA_ICC_ISO_AUDIO,
	TEGRA_ICC_ISO_VIFAL,
}

/* ICC ID's for MC client's used in BPMP */
pub const TEGRA_ICC_BPMP_DEBUG: i32 = 1;
pub const TEGRA_ICC_BPMP_CPU_CLUSTER0: i32 = 2;
pub const TEGRA_ICC_BPMP_CPU_CLUSTER1: i32 = 3;
pub const TEGRA_ICC_BPMP_CPU_CLUSTER2: i32 = 4;
pub const TEGRA_ICC_BPMP_GPU: i32 = 5;
pub const TEGRA_ICC_BPMP_CACTMON: i32 = 6;
pub const TEGRA_ICC_BPMP_DISPLAY: i32 = 7;
pub const TEGRA_ICC_BPMP_VI: i32 = 8;
pub const TEGRA_ICC_BPMP_EQOS: i32 = 9;
pub const TEGRA_ICC_BPMP_PCIE_0: i32 = 10;
pub const TEGRA_ICC_BPMP_PCIE_1: i32 = 11;
pub const TEGRA_ICC_BPMP_PCIE_2: i32 = 12;
pub const TEGRA_ICC_BPMP_PCIE_3: i32 = 13;
pub const TEGRA_ICC_BPMP_PCIE_4: i32 = 14;
pub const TEGRA_ICC_BPMP_PCIE_5: i32 = 15;
pub const TEGRA_ICC_BPMP_PCIE_6: i32 = 16;
pub const TEGRA_ICC_BPMP_PCIE_7: i32 = 17;
pub const TEGRA_ICC_BPMP_PCIE_8: i32 = 18;
pub const TEGRA_ICC_BPMP_PCIE_9: i32 = 19;
pub const TEGRA_ICC_BPMP_PCIE_10: i32 = 20;
pub const TEGRA_ICC_BPMP_DLA_0: i32 = 21;
pub const TEGRA_ICC_BPMP_DLA_1: i32 = 22;
pub const TEGRA_ICC_BPMP_SDMMC_1: i32 = 23;
pub const TEGRA_ICC_BPMP_SDMMC_2: i32 = 24;
pub const TEGRA_ICC_BPMP_SDMMC_3: i32 = 25;
pub const TEGRA_ICC_BPMP_SDMMC_4: i32 = 26;
pub const TEGRA_ICC_BPMP_NVDEC: i32 = 27;
pub const TEGRA_ICC_BPMP_NVENC: i32 = 28;
pub const TEGRA_ICC_BPMP_NVJPG_0: i32 = 29;
pub const TEGRA_ICC_BPMP_NVJPG_1: i32 = 30;
pub const TEGRA_ICC_BPMP_OFAA: i32 = 31;
pub const TEGRA_ICC_BPMP_XUSB_HOST: i32 = 32;
pub const TEGRA_ICC_BPMP_XUSB_DEV: i32 = 33;
pub const TEGRA_ICC_BPMP_TSEC: i32 = 34;
pub const TEGRA_ICC_BPMP_VIC: i32 = 35;
pub const TEGRA_ICC_BPMP_APE: i32 = 36;
pub const TEGRA_ICC_BPMP_APEDMA: i32 = 37;
pub const TEGRA_ICC_BPMP_SE: i32 = 38;
pub const TEGRA_ICC_BPMP_ISP: i32 = 39;
pub const TEGRA_ICC_BPMP_HDA: i32 = 40;
pub const TEGRA_ICC_BPMP_VIFAL: i32 = 41;
pub const TEGRA_ICC_BPMP_VI2FAL: i32 = 42;
pub const TEGRA_ICC_BPMP_VI2: i32 = 43;
pub const TEGRA_ICC_BPMP_RCE: i32 = 44;
pub const TEGRA_ICC_BPMP_PVA: i32 = 45;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
