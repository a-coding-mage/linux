/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2025 Intel Corporation */

// Dependencies supplied by the corresponding QAT headers are intentionally
// referenced here rather than redefined.

#[repr(C)]
pub struct icp_qat_hw_comp_51_config_csr_lower {
    pub abd: icp_qat_hw_comp_51_abd,
    pub lllbd: icp_qat_hw_comp_51_lllbd_ctrl,
    pub sd: icp_qat_hw_comp_51_search_depth,
    pub mmctrl: icp_qat_hw_comp_51_min_match_control,
    pub lbc: icp_qat_hw_comp_51_lz4_block_checksum,
}

#[inline]
pub fn ICP_QAT_FW_COMP_51_BUILD_CONFIG_LOWER(
    csr: icp_qat_hw_comp_51_config_csr_lower,
) -> u32 {
    let mut val32: u32 = 0;

    QAT_FIELD_SET!(val32, csr.abd,
        ICP_QAT_HW_COMP_51_CONFIG_CSR_ABD_BITPOS,
        ICP_QAT_HW_COMP_51_CONFIG_CSR_ABD_MASK);
    QAT_FIELD_SET!(val32, csr.lllbd,
        ICP_QAT_HW_COMP_51_CONFIG_CSR_LLLBD_CTRL_BITPOS,
        ICP_QAT_HW_COMP_51_CONFIG_CSR_LLLBD_CTRL_MASK);
    QAT_FIELD_SET!(val32, csr.sd,
        ICP_QAT_HW_COMP_51_CONFIG_CSR_SEARCH_DEPTH_BITPOS,
        ICP_QAT_HW_COMP_51_CONFIG_CSR_SEARCH_DEPTH_MASK);
    QAT_FIELD_SET!(val32, csr.mmctrl,
        ICP_QAT_HW_COMP_51_CONFIG_CSR_MIN_MATCH_CONTROL_BITPOS,
        ICP_QAT_HW_COMP_51_CONFIG_CSR_MIN_MATCH_CONTROL_MASK);
    QAT_FIELD_SET!(val32, csr.lbc,
        ICP_QAT_HW_COMP_51_CONFIG_CSR_LZ4_BLOCK_CHECKSUM_BITPOS,
        ICP_QAT_HW_COMP_51_CONFIG_CSR_LZ4_BLOCK_CHECKSUM_MASK);

    val32
}

#[repr(C)]
pub struct icp_qat_hw_comp_51_config_csr_upper {
    pub edmm: icp_qat_hw_comp_51_dmm_algorithm,
    pub bms: icp_qat_hw_comp_51_bms,
    pub scb_mode_reset: icp_qat_hw_comp_51_scb_mode_reset_mask,
}

#[inline]
pub fn ICP_QAT_FW_COMP_51_BUILD_CONFIG_UPPER(
    csr: icp_qat_hw_comp_51_config_csr_upper,
) -> u32 {
    let mut val32: u32 = 0;

    QAT_FIELD_SET!(val32, csr.edmm,
        ICP_QAT_HW_COMP_51_CONFIG_CSR_DMM_ALGORITHM_BITPOS,
        ICP_QAT_HW_COMP_51_CONFIG_CSR_DMM_ALGORITHM_MASK);
    QAT_FIELD_SET!(val32, csr.bms,
        ICP_QAT_HW_COMP_51_CONFIG_CSR_BMS_BITPOS,
        ICP_QAT_HW_COMP_51_CONFIG_CSR_BMS_MASK);
    QAT_FIELD_SET!(val32, csr.scb_mode_reset,
        ICP_QAT_HW_COMP_51_CONFIG_CSR_SCB_MODE_RESET_MASK_BITPOS,
        ICP_QAT_HW_COMP_51_CONFIG_CSR_SCB_MODE_RESET_MASK_MASK);

    val32
}

#[repr(C)]
pub struct icp_qat_hw_decomp_51_config_csr_lower {
    pub lbc: icp_qat_hw_decomp_51_lz4_block_checksum,
}

#[inline]
pub fn ICP_QAT_FW_DECOMP_51_BUILD_CONFIG_LOWER(
    csr: icp_qat_hw_decomp_51_config_csr_lower,
) -> u32 {
    let mut val32: u32 = 0;

    QAT_FIELD_SET!(val32, csr.lbc,
        ICP_QAT_HW_DECOMP_51_CONFIG_CSR_LZ4_BLOCK_CHECKSUM_BITPOS,
        ICP_QAT_HW_DECOMP_51_CONFIG_CSR_LZ4_BLOCK_CHECKSUM_MASK);

    val32
}

#[repr(C)]
pub struct icp_qat_hw_decomp_51_config_csr_upper {
    pub bms: icp_qat_hw_decomp_51_bms,
}

#[inline]
pub fn ICP_QAT_FW_DECOMP_51_BUILD_CONFIG_UPPER(
    csr: icp_qat_hw_decomp_51_config_csr_upper,
) -> u32 {
    let mut val32: u32 = 0;

    QAT_FIELD_SET!(val32, csr.bms,
        ICP_QAT_HW_DECOMP_51_CONFIG_CSR_BMS_BITPOS,
        ICP_QAT_HW_DECOMP_51_CONFIG_CSR_BMS_MASK);

    val32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
