/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2014 - 2020 Intel Corporation */

/* Dependency: HZ_PER_MHZ and adf_hw_device_data are supplied externally. */

/* PCIe configuration space */
pub const ADF_DH895XCC_SRAM_BAR: u32 = 0;
pub const ADF_DH895XCC_PMISC_BAR: u32 = 1;
pub const ADF_DH895XCC_ETR_BAR: u32 = 2;
pub const ADF_DH895XCC_FUSECTL_SKU_MASK: u32 = 0x300000;
pub const ADF_DH895XCC_FUSECTL_SKU_SHIFT: u32 = 20;
pub const ADF_DH895XCC_FUSECTL_SKU_1: u32 = 0x0;
pub const ADF_DH895XCC_FUSECTL_SKU_2: u32 = 0x1;
pub const ADF_DH895XCC_FUSECTL_SKU_3: u32 = 0x2;
pub const ADF_DH895XCC_FUSECTL_SKU_4: u32 = 0x3;
pub const ADF_DH895XCC_MAX_ACCELERATORS: u32 = 6;
pub const ADF_DH895XCC_MAX_ACCELENGINES: u32 = 12;
pub const ADF_DH895XCC_ACCELERATORS_REG_OFFSET: u32 = 13;
pub const ADF_DH895XCC_ACCELERATORS_MASK: u32 = 0x3F;
pub const ADF_DH895XCC_ACCELENGINES_MASK: u32 = 0xFFF;
pub const ADF_DH895XCC_ETR_MAX_BANKS: u32 = 32;

/* Masks for VF2PF interrupts */
#[inline]
pub const fn ADF_DH895XCC_ERR_REG_VF2PF_L(vf_src: u32) -> u32 {
    (vf_src & 0x01FFFE00) >> 9
}

#[inline]
pub const fn ADF_DH895XCC_ERR_MSK_VF2PF_L(vf_mask: u32) -> u32 {
    (vf_mask & 0xFFFF) << 9
}

#[inline]
pub const fn ADF_DH895XCC_ERR_REG_VF2PF_U(vf_src: u32) -> u32 {
    (vf_src & 0x0000FFFF) << 16
}

#[inline]
pub const fn ADF_DH895XCC_ERR_MSK_VF2PF_U(vf_mask: u32) -> u32 {
    vf_mask >> 16
}

/* AE to function mapping */
pub const ADF_DH895XCC_AE2FUNC_MAP_GRP_A_NUM_REGS: u32 = 96;
pub const ADF_DH895XCC_AE2FUNC_MAP_GRP_B_NUM_REGS: u32 = 12;

/* Clocks frequency */
pub const ADF_DH895X_AE_FREQ: u32 = 933 * HZ_PER_MHZ;

/* FW names */
pub const ADF_DH895XCC_FW: &str = "qat_895xcc.bin";
pub const ADF_DH895XCC_MMP: &str = "qat_895xcc_mmp.bin";

unsafe extern "C" {
    pub fn adf_init_hw_data_dh895xcc(hw_data: *mut adf_hw_device_data);
    pub fn adf_clean_hw_data_dh895xcc(hw_data: *mut adf_hw_device_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
