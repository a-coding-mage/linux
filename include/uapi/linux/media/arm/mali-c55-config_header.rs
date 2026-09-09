/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* ARM Mali-C55 ISP Driver - Userspace API */

// C dependencies: linux/types.h, linux/v4l2-controls.h, linux/media/v4l2-isp.h.

pub const V4L2_CID_MALI_C55_CAPABILITIES: u32 = V4L2_CID_USER_MALI_C55_BASE + 0x0;
pub const MALI_C55_GPS_PONG: u32 = 1u32 << 0;
pub const MALI_C55_GPS_WDR: u32 = 1u32 << 1;
pub const MALI_C55_GPS_COMPRESSION: u32 = 1u32 << 2;
pub const MALI_C55_GPS_TEMPER: u32 = 1u32 << 3;
pub const MALI_C55_GPS_SINTER_LITE: u32 = 1u32 << 4;
pub const MALI_C55_GPS_SINTER: u32 = 1u32 << 5;
pub const MALI_C55_GPS_IRIDIX_LTM: u32 = 1u32 << 6;
pub const MALI_C55_GPS_IRIDIX_GTM: u32 = 1u32 << 7;
pub const MALI_C55_GPS_CNR: u32 = 1u32 << 8;
pub const MALI_C55_GPS_FRSCALER: u32 = 1u32 << 9;
pub const MALI_C55_GPS_DS_PIPE: u32 = 1u32 << 10;
pub const MALI_C55_MAX_ZONES: usize = 15 * 15;
pub const MALI_C55_NUM_GAMMA_LUT_ELEMENTS: usize = 129;

#[repr(C, packed)]
pub struct mali_c55_ae_1024bin_hist { pub bins: [u16; 1024] }

#[repr(C, packed)]
pub struct mali_c55_ae_5bin_hist { pub hist0: u16, pub hist1: u16, pub hist3: u16, pub hist4: u16 }

#[repr(C, packed)]
pub struct mali_c55_awb_average_ratios { pub avg_rg_gr: u16, pub avg_bg_br: u16, pub num_pixels: u32 }

#[repr(C, packed)]
pub struct mali_c55_af_statistics { pub intensity_stats: u16, pub edge_stats: u16 }

#[repr(C, packed)]
pub struct mali_c55_stats_buffer {
    pub ae_1024bin_hist: mali_c55_ae_1024bin_hist,
    pub iridix_1024bin_hist: mali_c55_ae_1024bin_hist,
    pub ae_5bin_hists: [mali_c55_ae_5bin_hist; MALI_C55_MAX_ZONES],
    pub reserved1: [u32; 14],
    pub awb_ratios: [mali_c55_awb_average_ratios; MALI_C55_MAX_ZONES],
    pub reserved2: [u32; 14],
    pub af_statistics: [mali_c55_af_statistics; MALI_C55_MAX_ZONES],
    pub reserved3: [u32; 15],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mali_c55_param_block_type {
    MALI_C55_PARAM_BLOCK_SENSOR_OFFS,
    MALI_C55_PARAM_BLOCK_AEXP_HIST,
    MALI_C55_PARAM_BLOCK_AEXP_IHIST,
    MALI_C55_PARAM_BLOCK_AEXP_HIST_WEIGHTS,
    MALI_C55_PARAM_BLOCK_AEXP_IHIST_WEIGHTS,
    MALI_C55_PARAM_BLOCK_DIGITAL_GAIN,
    MALI_C55_PARAM_BLOCK_AWB_GAINS,
    MALI_C55_PARAM_BLOCK_AWB_CONFIG,
    MALI_C55_PARAM_BLOCK_AWB_GAINS_AEXP,
    MALI_C55_PARAM_MESH_SHADING_CONFIG,
    MALI_C55_PARAM_MESH_SHADING_SELECTION,
    MALI_C55_PARAM_BLOCK_CCM,
    MALI_C55_PARAM_BLOCK_GAMMA_FR,
    MALI_C55_PARAM_BLOCK_GAMMA_DS,
}

#[repr(C)]
pub struct mali_c55_params_sensor_off_preshading {
    pub header: v4l2_isp_params_block_header,
    pub chan00: u32, pub chan01: u32, pub chan10: u32, pub chan11: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mali_c55_aexp_hist_tap_points { MALI_C55_AEXP_HIST_TAP_WB = 0, MALI_C55_AEXP_HIST_TAP_FS, MALI_C55_AEXP_HIST_TAP_TPG }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mali_c55_aexp_skip_x { MALI_C55_AEXP_SKIP_X_EVERY_2ND, MALI_C55_AEXP_SKIP_X_EVERY_3RD, MALI_C55_AEXP_SKIP_X_EVERY_4TH, MALI_C55_AEXP_SKIP_X_EVERY_5TH, MALI_C55_AEXP_SKIP_X_EVERY_8TH, MALI_C55_AEXP_SKIP_X_EVERY_9TH }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mali_c55_aexp_skip_y { MALI_C55_AEXP_SKIP_Y_ALL, MALI_C55_AEXP_SKIP_Y_EVERY_2ND, MALI_C55_AEXP_SKIP_Y_EVERY_3RD, MALI_C55_AEXP_SKIP_Y_EVERY_4TH, MALI_C55_AEXP_SKIP_Y_EVERY_5TH, MALI_C55_AEXP_SKIP_Y_EVERY_8TH, MALI_C55_AEXP_SKIP_Y_EVERY_9TH }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mali_c55_aexp_row_column_offset { MALI_C55_AEXP_FIRST_ROW_OR_COL = 1, MALI_C55_AEXP_SECOND_ROW_OR_COL = 2 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mali_c55_aexp_hist_plane_mode { MALI_C55_AEXP_HIST_COMBINED = 0, MALI_C55_AEXP_HIST_SEPARATE = 1, MALI_C55_AEXP_HIST_FOCUS_00 = 4, MALI_C55_AEXP_HIST_FOCUS_01 = 5, MALI_C55_AEXP_HIST_FOCUS_10 = 6, MALI_C55_AEXP_HIST_FOCUS_11 = 7 }

#[repr(C)]
pub struct mali_c55_params_aexp_hist {
    pub header: v4l2_isp_params_block_header,
    pub skip_x: u8, pub offset_x: u8, pub skip_y: u8, pub offset_y: u8,
    pub scale_bottom: u8, pub scale_top: u8, pub plane_mode: u8, pub tap_point: u8,
}

#[repr(C)]
pub struct mali_c55_params_aexp_weights {
    pub header: v4l2_isp_params_block_header,
    pub nodes_used_horiz: u8, pub nodes_used_vert: u8,
    pub zone_weights: [u8; MALI_C55_MAX_ZONES],
}

#[repr(C)]
pub struct mali_c55_params_digital_gain { pub header: v4l2_isp_params_block_header, pub gain: u16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mali_c55_awb_stats_mode { MALI_C55_AWB_MODE_GRBR = 0, MALI_C55_AWB_MODE_RGBG }

#[repr(C)]
pub struct mali_c55_params_awb_gains {
    pub header: v4l2_isp_params_block_header,
    pub gain00: u16, pub gain01: u16, pub gain10: u16, pub gain11: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mali_c55_params_awb_tap_points { MALI_C55_AWB_STATS_TAP_PF = 0, MALI_C55_AWB_STATS_TAP_CNR }

#[repr(C)]
pub struct mali_c55_params_awb_config {
    pub header: v4l2_isp_params_block_header,
    pub tap_point: u8, pub stats_mode: u8,
    pub white_level: u16, pub black_level: u16,
    pub cr_max: u16, pub cr_min: u16, pub cb_max: u16, pub cb_min: u16,
    pub nodes_used_horiz: u8, pub nodes_used_vert: u8,
    pub cr_high: u16, pub cr_low: u16, pub cb_high: u16, pub cb_low: u16,
}

pub const MALI_C55_NUM_MESH_SHADING_ELEMENTS: usize = 3072;

#[repr(C)]
pub struct mali_c55_params_mesh_shading_config {
    pub header: v4l2_isp_params_block_header,
    pub mesh_show: u8, pub mesh_scale: u8, pub mesh_page_r: u8, pub mesh_page_g: u8,
    pub mesh_page_b: u8, pub mesh_width: u8, pub mesh_height: u8,
    pub mesh: [u32; MALI_C55_NUM_MESH_SHADING_ELEMENTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mali_c55_params_mesh_alpha_bank { MALI_C55_MESH_ALPHA_BANK_LS0_AND_LS1 = 0, MALI_C55_MESH_ALPHA_BANK_LS1_AND_LS2 = 1, MALI_C55_MESH_ALPHA_BANK_LS0_AND_LS2 = 4 }

#[repr(C)]
pub struct mali_c55_params_mesh_shading_selection {
    pub header: v4l2_isp_params_block_header,
    pub mesh_alpha_bank_r: u8, pub mesh_alpha_bank_g: u8, pub mesh_alpha_bank_b: u8,
    pub mesh_alpha_r: u8, pub mesh_alpha_g: u8, pub mesh_alpha_b: u8,
    pub mesh_strength: u16,
}

#[repr(C)]
pub struct mali_c55_params_ccm {
    pub header: v4l2_isp_params_block_header,
    pub coeffs: [[u16; 3]; 3], pub gains: [u16; 3], pub offs: [u16; 3],
}

#[repr(C)]
pub struct mali_c55_params_gamma {
    pub header: v4l2_isp_params_block_header,
    pub gains: [u16; 3], pub offs: [u16; 3],
    pub lut: [u32; MALI_C55_NUM_GAMMA_LUT_ELEMENTS],
}

pub const MALI_C55_PARAMS_MAX_SIZE: usize =
    core::mem::size_of::<mali_c55_params_sensor_off_preshading>() +
    core::mem::size_of::<mali_c55_params_aexp_hist>() +
    core::mem::size_of::<mali_c55_params_aexp_weights>() +
    core::mem::size_of::<mali_c55_params_aexp_hist>() +
    core::mem::size_of::<mali_c55_params_aexp_weights>() +
    core::mem::size_of::<mali_c55_params_digital_gain>() +
    core::mem::size_of::<mali_c55_params_awb_gains>() +
    core::mem::size_of::<mali_c55_params_awb_config>() +
    core::mem::size_of::<mali_c55_params_awb_gains>() +
    core::mem::size_of::<mali_c55_params_mesh_shading_config>() +
    core::mem::size_of::<mali_c55_params_mesh_shading_selection>() +
    core::mem::size_of::<mali_c55_params_ccm>() +
    core::mem::size_of::<mali_c55_params_gamma>() +
    core::mem::size_of::<mali_c55_params_gamma>();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
