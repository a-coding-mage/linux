/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2023 Qualcomm Innovation Center, Inc. All rights reserved.
 * Copyright (c) 2023, Linaro Limited
 */

// The original header defines these identifiers independently for each
// interconnect block. Rust modules preserve those source-level scopes.

pub mod a1noc {
    pub const MASTER_QSPI_0: i32 = 0;
    pub const MASTER_QUP_1: i32 = 1;
    pub const MASTER_SDCC_4: i32 = 2;
    pub const MASTER_UFS_MEM: i32 = 3;
    pub const SLAVE_A1NOC_SNOC: i32 = 4;
}

pub mod a2noc {
    pub const MASTER_QUP_0: i32 = 0;
    pub const MASTER_QUP_2: i32 = 1;
    pub const MASTER_CRYPTO: i32 = 2;
    pub const MASTER_SP: i32 = 3;
    pub const MASTER_QDSS_ETR: i32 = 4;
    pub const MASTER_QDSS_ETR_1: i32 = 5;
    pub const MASTER_SDCC_2: i32 = 6;
    pub const SLAVE_A2NOC_SNOC: i32 = 7;
}

pub mod qup_core {
    pub const MASTER_DDR_PERF_MODE: i32 = 0;
    pub const MASTER_QUP_CORE_0: i32 = 1;
    pub const MASTER_QUP_CORE_1: i32 = 2;
    pub const MASTER_QUP_CORE_2: i32 = 3;
    pub const SLAVE_DDR_PERF_MODE: i32 = 4;
    pub const SLAVE_QUP_CORE_0: i32 = 5;
    pub const SLAVE_QUP_CORE_1: i32 = 6;
    pub const SLAVE_QUP_CORE_2: i32 = 7;
}

pub mod cnoc {
    pub const MASTER_CNOC_CFG: i32 = 0;
    pub const SLAVE_AHB2PHY_SOUTH: i32 = 1;
    pub const SLAVE_AHB2PHY_NORTH: i32 = 2;
    pub const SLAVE_AHB2PHY_2: i32 = 3;
    pub const SLAVE_AV1_ENC_CFG: i32 = 4;
    pub const SLAVE_CAMERA_CFG: i32 = 5;
    pub const SLAVE_CLK_CTL: i32 = 6;
    pub const SLAVE_CRYPTO_0_CFG: i32 = 7;
    pub const SLAVE_DISPLAY_CFG: i32 = 8;
    pub const SLAVE_GFX3D_CFG: i32 = 9;
    pub const SLAVE_IMEM_CFG: i32 = 10;
    pub const SLAVE_IPC_ROUTER_CFG: i32 = 11;
    pub const SLAVE_PCIE_0_CFG: i32 = 12;
    pub const SLAVE_PCIE_1_CFG: i32 = 13;
    pub const SLAVE_PCIE_2_CFG: i32 = 14;
    pub const SLAVE_PCIE_3_CFG: i32 = 15;
    pub const SLAVE_PCIE_4_CFG: i32 = 16;
    pub const SLAVE_PCIE_5_CFG: i32 = 17;
    pub const SLAVE_PCIE_6A_CFG: i32 = 18;
    pub const SLAVE_PCIE_6B_CFG: i32 = 19;
    pub const SLAVE_PCIE_RSC_CFG: i32 = 20;
    pub const SLAVE_PDM: i32 = 21;
    pub const SLAVE_PRNG: i32 = 22;
    pub const SLAVE_QDSS_CFG: i32 = 23;
    pub const SLAVE_QSPI_0: i32 = 24;
    pub const SLAVE_QUP_0: i32 = 25;
    pub const SLAVE_QUP_1: i32 = 26;
    pub const SLAVE_QUP_2: i32 = 27;
    pub const SLAVE_SDCC_2: i32 = 28;
    pub const SLAVE_SDCC_4: i32 = 29;
    pub const SLAVE_SMMUV3_CFG: i32 = 30;
    pub const SLAVE_TCSR: i32 = 31;
    pub const SLAVE_TLMM: i32 = 32;
    pub const SLAVE_UFS_MEM_CFG: i32 = 33;
    pub const SLAVE_USB2: i32 = 34;
    pub const SLAVE_USB3_0: i32 = 35;
    pub const SLAVE_USB3_1: i32 = 36;
    pub const SLAVE_USB3_2: i32 = 37;
    pub const SLAVE_USB3_MP: i32 = 38;
    pub const SLAVE_USB4_0: i32 = 39;
    pub const SLAVE_USB4_1: i32 = 40;
    pub const SLAVE_USB4_2: i32 = 41;
    pub const SLAVE_VENUS_CFG: i32 = 42;
    pub const SLAVE_LPASS_QTB_CFG: i32 = 43;
    pub const SLAVE_CNOC_MNOC_CFG: i32 = 44;
    pub const SLAVE_NSP_QTB_CFG: i32 = 45;
    pub const SLAVE_QDSS_STM: i32 = 46;
    pub const SLAVE_TCU: i32 = 47;
}

pub mod gem_noc {
    pub const MASTER_GEM_NOC_CNOC: i32 = 0;
    pub const MASTER_GEM_NOC_PCIE_SNOC: i32 = 1;
    pub const SLAVE_AOSS: i32 = 2;
    pub const SLAVE_TME_CFG: i32 = 3;
    pub const SLAVE_APPSS: i32 = 4;
    pub const SLAVE_CNOC_CFG: i32 = 5;
    pub const SLAVE_BOOT_IMEM: i32 = 6;
    pub const SLAVE_IMEM: i32 = 7;
    pub const SLAVE_PCIE_0: i32 = 8;
    pub const SLAVE_PCIE_1: i32 = 9;
    pub const SLAVE_PCIE_2: i32 = 10;
    pub const SLAVE_PCIE_3: i32 = 11;
    pub const SLAVE_PCIE_4: i32 = 12;
    pub const SLAVE_PCIE_5: i32 = 13;
    pub const SLAVE_PCIE_6A: i32 = 14;
    pub const SLAVE_PCIE_6B: i32 = 15;
}

pub mod mem_noc {
    pub const MASTER_GPU_TCU: i32 = 0;
    pub const MASTER_PCIE_TCU: i32 = 1;
    pub const MASTER_SYS_TCU: i32 = 2;
    pub const MASTER_APPSS_PROC: i32 = 3;
    pub const MASTER_GFX3D: i32 = 4;
    pub const MASTER_LPASS_GEM_NOC: i32 = 5;
    pub const MASTER_MNOC_HF_MEM_NOC: i32 = 6;
    pub const MASTER_MNOC_SF_MEM_NOC: i32 = 7;
    pub const MASTER_COMPUTE_NOC: i32 = 8;
    pub const MASTER_ANOC_PCIE_GEM_NOC: i32 = 9;
    pub const MASTER_SNOC_SF_MEM_NOC: i32 = 10;
    pub const MASTER_GIC2: i32 = 11;
    pub const SLAVE_GEM_NOC_CNOC: i32 = 12;
    pub const SLAVE_LLCC: i32 = 13;
    pub const SLAVE_MEM_NOC_PCIE_SNOC: i32 = 14;
}

pub mod lpiaon_noc { pub const MASTER_LPIAON_NOC: i32 = 0; pub const SLAVE_LPASS_GEM_NOC: i32 = 1; }
pub mod lpinoc { pub const MASTER_LPASS_LPINOC: i32 = 0; pub const SLAVE_LPIAON_NOC_LPASS_AG_NOC: i32 = 1; }
pub mod lpass_proc { pub const MASTER_LPASS_PROC: i32 = 0; pub const SLAVE_LPICX_NOC_LPIAON_NOC: i32 = 1; }
pub mod llcc { pub const MASTER_LLCC: i32 = 0; pub const SLAVE_EBI1: i32 = 1; }

pub mod mnoc {
    pub const MASTER_AV1_ENC: i32 = 0;
    pub const MASTER_CAMNOC_HF: i32 = 1;
    pub const MASTER_CAMNOC_ICP: i32 = 2;
    pub const MASTER_CAMNOC_SF: i32 = 3;
    pub const MASTER_EVA: i32 = 4;
    pub const MASTER_MDP: i32 = 5;
    pub const MASTER_VIDEO: i32 = 6;
    pub const MASTER_VIDEO_CV_PROC: i32 = 7;
    pub const MASTER_VIDEO_V_PROC: i32 = 8;
    pub const MASTER_CNOC_MNOC_CFG: i32 = 9;
    pub const SLAVE_MNOC_HF_MEM_NOC: i32 = 10;
    pub const SLAVE_MNOC_SF_MEM_NOC: i32 = 11;
    pub const SLAVE_SERVICE_MNOC: i32 = 12;
}

pub mod cdsp { pub const MASTER_CDSP_PROC: i32 = 0; pub const SLAVE_CDSP_MEM_NOC: i32 = 1; }
pub mod pcie_north { pub const MASTER_PCIE_NORTH: i32 = 0; pub const MASTER_PCIE_SOUTH: i32 = 1; pub const SLAVE_ANOC_PCIE_GEM_NOC: i32 = 2; }
pub mod pcie_north_links { pub const MASTER_PCIE_3: i32 = 0; pub const MASTER_PCIE_4: i32 = 1; pub const MASTER_PCIE_5: i32 = 2; pub const SLAVE_PCIE_NORTH: i32 = 3; }
pub mod pcie_south_links { pub const MASTER_PCIE_0: i32 = 0; pub const MASTER_PCIE_1: i32 = 1; pub const MASTER_PCIE_2: i32 = 2; pub const MASTER_PCIE_6A: i32 = 3; pub const MASTER_PCIE_6B: i32 = 4; pub const SLAVE_PCIE_SOUTH: i32 = 5; }
pub mod snoc { pub const MASTER_A1NOC_SNOC: i32 = 0; pub const MASTER_A2NOC_SNOC: i32 = 1; pub const MASTER_GIC1: i32 = 2; pub const MASTER_USB_NOC_SNOC: i32 = 3; pub const SLAVE_SNOC_GEM_NOC_SF: i32 = 4; }
pub mod aggre_usb { pub const MASTER_AGGRE_USB_NORTH: i32 = 0; pub const MASTER_AGGRE_USB_SOUTH: i32 = 1; pub const SLAVE_USB_NOC_SNOC: i32 = 2; }
pub mod usb_noc { pub const MASTER_USB2: i32 = 0; pub const MASTER_USB3_MP: i32 = 1; pub const SLAVE_AGGRE_USB_NORTH: i32 = 2; }
pub mod usb_aggre_south { pub const MASTER_USB3_0: i32 = 0; pub const MASTER_USB3_1: i32 = 1; pub const MASTER_USB3_2: i32 = 2; pub const MASTER_USB4_0: i32 = 3; pub const MASTER_USB4_1: i32 = 4; pub const MASTER_USB4_2: i32 = 5; pub const SLAVE_AGGRE_USB_SOUTH: i32 = 6; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
