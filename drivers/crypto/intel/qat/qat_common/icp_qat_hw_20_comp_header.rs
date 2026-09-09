/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2022 Intel Corporation */

// Translated from icp_qat_hw_20_comp.h.
// The enum types and configuration constants are supplied by the corresponding
// dependency headers in the surrounding translation unit.

#[repr(C)]
pub struct icp_qat_hw_comp_20_config_csr_lower {
    pub edmm: icp_qat_hw_comp_20_extended_delay_match_mode,
    pub algo: icp_qat_hw_comp_20_hw_comp_format,
    pub sd: icp_qat_hw_comp_20_search_depth,
    pub hbs: icp_qat_hw_comp_20_hbs_control,
    pub abd: icp_qat_hw_comp_20_abd,
    pub lllbd: icp_qat_hw_comp_20_lllbd_ctrl,
    pub mmctrl: icp_qat_hw_comp_20_min_match_control,
    pub hash_col: icp_qat_hw_comp_20_skip_hash_collision,
    pub hash_update: icp_qat_hw_comp_20_skip_hash_update,
    pub skip_ctrl: icp_qat_hw_comp_20_byte_skip,
}

#[inline]
pub fn ICP_QAT_FW_COMP_20_BUILD_CONFIG_LOWER(
    csr: icp_qat_hw_comp_20_config_csr_lower,
) -> u32 {
    let mut val32: u32 = 0;
    val32 |= ((csr.algo as u32 & ICP_QAT_HW_COMP_20_CONFIG_CSR_HW_COMP_FORMAT_MASK as u32)
        << ICP_QAT_HW_COMP_20_CONFIG_CSR_HW_COMP_FORMAT_BITPOS);
    val32 |= ((csr.sd as u32 & ICP_QAT_HW_COMP_20_CONFIG_CSR_SEARCH_DEPTH_MASK as u32)
        << ICP_QAT_HW_COMP_20_CONFIG_CSR_SEARCH_DEPTH_BITPOS);
    val32 |= ((csr.edmm as u32
        & ICP_QAT_HW_COMP_20_CONFIG_CSR_EXTENDED_DELAY_MATCH_MODE_MASK as u32)
        << ICP_QAT_HW_COMP_20_CONFIG_CSR_EXTENDED_DELAY_MATCH_MODE_BITPOS);
    val32 |= ((csr.hbs as u32 & ICP_QAT_HW_COMP_20_CONFIG_CSR_HBS_CONTROL_MASK as u32)
        << ICP_QAT_HW_COMP_20_CONFIG_CSR_HBS_CONTROL_BITPOS);
    val32 |= ((csr.lllbd as u32 & ICP_QAT_HW_COMP_20_CONFIG_CSR_LLLBD_CTRL_MASK as u32)
        << ICP_QAT_HW_COMP_20_CONFIG_CSR_LLLBD_CTRL_BITPOS);
    val32 |= ((csr.mmctrl as u32
        & ICP_QAT_HW_COMP_20_CONFIG_CSR_MIN_MATCH_CONTROL_MASK as u32)
        << ICP_QAT_HW_COMP_20_CONFIG_CSR_MIN_MATCH_CONTROL_BITPOS);
    val32 |= ((csr.hash_col as u32
        & ICP_QAT_HW_COMP_20_CONFIG_CSR_SKIP_HASH_COLLISION_MASK as u32)
        << ICP_QAT_HW_COMP_20_CONFIG_CSR_SKIP_HASH_COLLISION_BITPOS);
    val32 |= ((csr.hash_update as u32
        & ICP_QAT_HW_COMP_20_CONFIG_CSR_SKIP_HASH_UPDATE_MASK as u32)
        << ICP_QAT_HW_COMP_20_CONFIG_CSR_SKIP_HASH_UPDATE_BITPOS);
    val32 |= ((csr.skip_ctrl as u32 & ICP_QAT_HW_COMP_20_CONFIG_CSR_BYTE_SKIP_MASK as u32)
        << ICP_QAT_HW_COMP_20_CONFIG_CSR_BYTE_SKIP_BITPOS);
    val32 |= ((csr.abd as u32 & ICP_QAT_HW_COMP_20_CONFIG_CSR_ABD_MASK as u32)
        << ICP_QAT_HW_COMP_20_CONFIG_CSR_ABD_BITPOS);
    val32.swap_bytes()
}

#[repr(C)]
pub struct icp_qat_hw_comp_20_config_csr_upper {
    pub scb_ctrl: icp_qat_hw_comp_20_scb_control,
    pub rmb_ctrl: icp_qat_hw_comp_20_rmb_control,
    pub som_ctrl: icp_qat_hw_comp_20_som_control,
    pub skip_hash_ctrl: icp_qat_hw_comp_20_skip_hash_rd_control,
    pub scb_unload_ctrl: icp_qat_hw_comp_20_scb_unload_control,
    pub disable_token_fusion_ctrl: icp_qat_hw_comp_20_disable_token_fusion_control,
    pub lbms: icp_qat_hw_comp_20_lbms,
    pub scb_mode_reset: icp_qat_hw_comp_20_scb_mode_reset_mask,
    pub lazy: u16,
    pub nice: u16,
}

#[inline]
pub fn ICP_QAT_FW_COMP_20_BUILD_CONFIG_UPPER(
    csr: icp_qat_hw_comp_20_config_csr_upper,
) -> u32 {
    let mut val32: u32 = 0;
    val32 |= ((csr.scb_ctrl as u32 & ICP_QAT_HW_COMP_20_CONFIG_CSR_SCB_CONTROL_MASK as u32) << ICP_QAT_HW_COMP_20_CONFIG_CSR_SCB_CONTROL_BITPOS);
    val32 |= ((csr.rmb_ctrl as u32 & ICP_QAT_HW_COMP_20_CONFIG_CSR_RMB_CONTROL_MASK as u32) << ICP_QAT_HW_COMP_20_CONFIG_CSR_RMB_CONTROL_BITPOS);
    val32 |= ((csr.som_ctrl as u32 & ICP_QAT_HW_COMP_20_CONFIG_CSR_SOM_CONTROL_MASK as u32) << ICP_QAT_HW_COMP_20_CONFIG_CSR_SOM_CONTROL_BITPOS);
    val32 |= ((csr.skip_hash_ctrl as u32 & ICP_QAT_HW_COMP_20_CONFIG_CSR_SKIP_HASH_RD_CONTROL_MASK as u32) << ICP_QAT_HW_COMP_20_CONFIG_CSR_SKIP_HASH_RD_CONTROL_BITPOS);
    val32 |= ((csr.scb_unload_ctrl as u32 & ICP_QAT_HW_COMP_20_CONFIG_CSR_SCB_UNLOAD_CONTROL_MASK as u32) << ICP_QAT_HW_COMP_20_CONFIG_CSR_SCB_UNLOAD_CONTROL_BITPOS);
    val32 |= ((csr.disable_token_fusion_ctrl as u32 & ICP_QAT_HW_COMP_20_CONFIG_CSR_DISABLE_TOKEN_FUSION_CONTROL_MASK as u32) << ICP_QAT_HW_COMP_20_CONFIG_CSR_DISABLE_TOKEN_FUSION_CONTROL_BITPOS);
    val32 |= ((csr.lbms as u32 & ICP_QAT_HW_COMP_20_CONFIG_CSR_LBMS_MASK as u32) << ICP_QAT_HW_COMP_20_CONFIG_CSR_LBMS_BITPOS);
    val32 |= ((csr.scb_mode_reset as u32 & ICP_QAT_HW_COMP_20_CONFIG_CSR_SCB_MODE_RESET_MASK_MASK as u32) << ICP_QAT_HW_COMP_20_CONFIG_CSR_SCB_MODE_RESET_MASK_BITPOS);
    val32 |= ((csr.lazy as u32 & ICP_QAT_HW_COMP_20_CONFIG_CSR_LAZY_PARAM_MASK as u32) << ICP_QAT_HW_COMP_20_CONFIG_CSR_LAZY_PARAM_BITPOS);
    val32 |= ((csr.nice as u32 & ICP_QAT_HW_COMP_20_CONFIG_CSR_NICE_PARAM_MASK as u32) << ICP_QAT_HW_COMP_20_CONFIG_CSR_NICE_PARAM_BITPOS);
    val32.swap_bytes()
}

#[repr(C)]
pub struct icp_qat_hw_decomp_20_config_csr_lower {
    pub hbs: icp_qat_hw_decomp_20_hbs_control,
    pub lbms: icp_qat_hw_decomp_20_lbms,
    pub algo: icp_qat_hw_decomp_20_hw_comp_format,
    pub mmctrl: icp_qat_hw_decomp_20_min_match_control,
    pub lbc: icp_qat_hw_decomp_20_lz4_block_checksum_present,
}

#[inline]
pub fn ICP_QAT_FW_DECOMP_20_BUILD_CONFIG_LOWER(
    csr: icp_qat_hw_decomp_20_config_csr_lower,
) -> u32 {
    let mut val32: u32 = 0;
    val32 |= ((csr.hbs as u32 & ICP_QAT_HW_DECOMP_20_CONFIG_CSR_HBS_CONTROL_MASK as u32) << ICP_QAT_HW_DECOMP_20_CONFIG_CSR_HBS_CONTROL_BITPOS);
    val32 |= ((csr.lbms as u32 & ICP_QAT_HW_DECOMP_20_CONFIG_CSR_LBMS_MASK as u32) << ICP_QAT_HW_DECOMP_20_CONFIG_CSR_LBMS_BITPOS);
    val32 |= ((csr.algo as u32 & ICP_QAT_HW_DECOMP_20_CONFIG_CSR_HW_DECOMP_FORMAT_MASK as u32) << ICP_QAT_HW_DECOMP_20_CONFIG_CSR_HW_DECOMP_FORMAT_BITPOS);
    val32 |= ((csr.mmctrl as u32 & ICP_QAT_HW_DECOMP_20_CONFIG_CSR_MIN_MATCH_CONTROL_MASK as u32) << ICP_QAT_HW_DECOMP_20_CONFIG_CSR_MIN_MATCH_CONTROL_BITPOS);
    val32 |= ((csr.lbc as u32 & ICP_QAT_HW_DECOMP_20_CONFIG_CSR_LZ4_BLOCK_CHECKSUM_PRESENT_MASK as u32) << ICP_QAT_HW_DECOMP_20_CONFIG_CSR_LZ4_BLOCK_CHECKSUM_PRESENT_BITPOS);
    val32.swap_bytes()
}

#[repr(C)]
pub struct icp_qat_hw_decomp_20_config_csr_upper {
    pub sdc: icp_qat_hw_decomp_20_speculative_decoder_control,
    pub mcc: icp_qat_hw_decomp_20_mini_cam_control,
}

#[inline]
pub fn ICP_QAT_FW_DECOMP_20_BUILD_CONFIG_UPPER(
    csr: icp_qat_hw_decomp_20_config_csr_upper,
) -> u32 {
    let mut val32: u32 = 0;
    val32 |= ((csr.sdc as u32 & ICP_QAT_HW_DECOMP_20_CONFIG_CSR_SPECULATIVE_DECODER_CONTROL_MASK as u32) << ICP_QAT_HW_DECOMP_20_CONFIG_CSR_SPECULATIVE_DECODER_CONTROL_BITPOS);
    val32 |= ((csr.mcc as u32 & ICP_QAT_HW_DECOMP_20_CONFIG_CSR_MINI_CAM_CONTROL_MASK as u32) << ICP_QAT_HW_DECOMP_20_CONFIG_CSR_MINI_CAM_CONTROL_BITPOS);
    val32.swap_bytes()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
