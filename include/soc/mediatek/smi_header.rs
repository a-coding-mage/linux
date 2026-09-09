/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2015-2016 MediaTek Inc.
 * Author: Yong Wu <yong.wu@mediatek.com>
 */

// The original declarations are enabled when CONFIG_MTK_SMI is enabled.

#[repr(i32)]
pub enum iommu_atf_cmd {
    IOMMU_ATF_CMD_CONFIG_SMI_LARB,   /* For mm master to en/disable iommu */
    IOMMU_ATF_CMD_CONFIG_INFRA_IOMMU, /* For infra master to enable iommu */
    IOMMU_ATF_CMD_MAX,
}

#[macro_export]
macro_rules! MTK_SMI_MMU_EN {
    ($port:expr) => {
        1u32 << ($port)
    };
}

// Defined by the Linux device subsystem included by the original header.
pub enum device {}

#[repr(C)]
pub struct mtk_smi_larb_iommu {
    pub dev: *mut device,
    pub mmu: core::ffi::c_uint,
    pub bank: [u8; 32],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
