/* SPDX-License-Identifier: MIT
 * Copyright (C) 2018 Intel Corp.
 *
 * Authors:
 * Manasi Navare <manasi.d.navare@intel.com>
 */

// Dependency: struct dp_sdp_header is supplied by drm/display/drm_dp.h.

/* VESA Display Stream Compression DSC 1.2 constants */
pub const DSC_NUM_BUF_RANGES: usize = 15;
pub const DSC_MUX_WORD_SIZE_8_10_BPC: u32 = 48;
pub const DSC_MUX_WORD_SIZE_12_BPC: u32 = 64;
pub const DSC_RC_PIXELS_PER_GROUP: u32 = 3;
pub const DSC_SCALE_DECREMENT_INTERVAL_MAX: u32 = 4095;
pub const DSC_RANGE_BPG_OFFSET_MASK: u32 = 0x3f;

/* DSC Rate Control Constants */
pub const DSC_RC_MODEL_SIZE_CONST: u32 = 8192;
pub const DSC_RC_EDGE_FACTOR_CONST: u32 = 6;
pub const DSC_RC_TGT_OFFSET_HI_CONST: u32 = 3;
pub const DSC_RC_TGT_OFFSET_LO_CONST: u32 = 3;

/* DSC PPS constants and macros */
pub const DSC_PPS_VERSION_MAJOR_SHIFT: u32 = 4;
pub const DSC_PPS_BPC_SHIFT: u32 = 4;
pub const DSC_PPS_MSB_SHIFT: u32 = 8;
pub const DSC_PPS_LSB_MASK: u32 = 0xFF << 0;
pub const DSC_PPS_BPP_HIGH_MASK: u32 = 0x3 << 8;
pub const DSC_PPS_VBR_EN_SHIFT: u32 = 2;
pub const DSC_PPS_SIMPLE422_SHIFT: u32 = 3;
pub const DSC_PPS_CONVERT_RGB_SHIFT: u32 = 4;
pub const DSC_PPS_BLOCK_PRED_EN_SHIFT: u32 = 5;
pub const DSC_PPS_INIT_XMIT_DELAY_HIGH_MASK: u32 = 0x3 << 8;
pub const DSC_PPS_SCALE_DEC_INT_HIGH_MASK: u32 = 0xF << 8;
pub const DSC_PPS_RC_TGT_OFFSET_HI_SHIFT: u32 = 4;
pub const DSC_PPS_RC_RANGE_MINQP_SHIFT: u32 = 11;
pub const DSC_PPS_RC_RANGE_MAXQP_SHIFT: u32 = 6;
pub const DSC_PPS_NATIVE_420_SHIFT: u32 = 1;

#[repr(C)]
pub struct drm_dsc_rc_range_parameters {
    pub range_min_qp: u8,
    pub range_max_qp: u8,
    pub range_bpg_offset: u8,
}

#[repr(C)]
pub struct drm_dsc_config {
    pub line_buf_depth: u8,
    pub bits_per_component: u8,
    pub convert_rgb: bool,
    pub slice_count: u8,
    pub slice_width: u16,
    pub slice_height: u16,
    pub simple_422: bool,
    pub pic_width: u16,
    pub pic_height: u16,
    pub rc_tgt_offset_high: u8,
    pub rc_tgt_offset_low: u8,
    pub bits_per_pixel: u16,
    pub rc_edge_factor: u8,
    pub rc_quant_incr_limit1: u8,
    pub rc_quant_incr_limit0: u8,
    pub initial_xmit_delay: u16,
    pub initial_dec_delay: u16,
    pub block_pred_enable: bool,
    pub first_line_bpg_offset: u8,
    pub initial_offset: u16,
    pub rc_buf_thresh: [u16; DSC_NUM_BUF_RANGES - 1],
    pub rc_range_params: [drm_dsc_rc_range_parameters; DSC_NUM_BUF_RANGES],
    pub rc_model_size: u16,
    pub flatness_min_qp: u8,
    pub flatness_max_qp: u8,
    pub initial_scale_value: u8,
    pub scale_decrement_interval: u16,
    pub scale_increment_interval: u16,
    pub nfl_bpg_offset: u16,
    pub slice_bpg_offset: u16,
    pub final_offset: u16,
    pub vbr_enable: bool,
    pub mux_word_size: u8,
    pub slice_chunk_size: u16,
    pub rc_bits: u16,
    pub dsc_version_minor: u8,
    pub dsc_version_major: u8,
    pub native_422: bool,
    pub native_420: bool,
    pub second_line_bpg_offset: u8,
    pub nsl_bpg_offset: u16,
    pub second_line_offset_adj: u16,
}

#[repr(C, packed)]
pub struct drm_dsc_picture_parameter_set {
    pub dsc_version: u8,
    pub pps_identifier: u8,
    pub pps_reserved: u8,
    pub pps_3: u8,
    pub pps_4: u8,
    pub bits_per_pixel_low: u8,
    pub pic_height: u16,
    pub pic_width: u16,
    pub slice_height: u16,
    pub slice_width: u16,
    pub chunk_size: u16,
    pub initial_xmit_delay_high: u8,
    pub initial_xmit_delay_low: u8,
    pub initial_dec_delay: u16,
    pub pps20_reserved: u8,
    pub initial_scale_value: u8,
    pub scale_increment_interval: u16,
    pub scale_decrement_interval_high: u8,
    pub scale_decrement_interval_low: u8,
    pub pps26_reserved: u8,
    pub first_line_bpg_offset: u8,
    pub nfl_bpg_offset: u16,
    pub slice_bpg_offset: u16,
    pub initial_offset: u16,
    pub final_offset: u16,
    pub flatness_min_qp: u8,
    pub flatness_max_qp: u8,
    pub rc_model_size: u16,
    pub rc_edge_factor: u8,
    pub rc_quant_incr_limit0: u8,
    pub rc_quant_incr_limit1: u8,
    pub rc_tgt_offset: u8,
    pub rc_buf_thresh: [u8; DSC_NUM_BUF_RANGES - 1],
    pub rc_range_parameters: [u16; DSC_NUM_BUF_RANGES],
    pub native_422_420: u8,
    pub second_line_bpg_offset: u8,
    pub nsl_bpg_offset: u16,
    pub second_line_offset_adj: u16,
    pub pps_long_94_reserved: u32,
    pub pps_long_98_reserved: u32,
    pub pps_long_102_reserved: u32,
    pub pps_long_106_reserved: u32,
    pub pps_long_110_reserved: u32,
    pub pps_long_114_reserved: u32,
    pub pps_long_118_reserved: u32,
    pub pps_long_122_reserved: u32,
    pub pps_short_126_reserved: u16,
}

#[repr(C, packed)]
pub struct drm_dsc_pps_infoframe {
    pub pps_header: dp_sdp_header,
    pub pps_payload: drm_dsc_picture_parameter_set,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
