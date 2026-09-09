// SPDX-License-Identifier: MIT
// Copyright 2024 Advanced Micro Devices, Inc.
// Translated from dc_spl_types.h. Included C-header dependencies are supplied externally.

#[repr(C)]
pub struct spl_size { pub width: u32, pub height: u32 }
#[repr(C)]
pub struct spl_rect { pub x: i32, pub y: i32, pub width: i32, pub height: i32 }
#[repr(C)]
pub struct spl_ratios { pub horz: spl_fixed31_32, pub vert: spl_fixed31_32, pub horz_c: spl_fixed31_32, pub vert_c: spl_fixed31_32 }
#[repr(C)]
pub struct spl_inits { pub h: spl_fixed31_32, pub h_c: spl_fixed31_32, pub v: spl_fixed31_32, pub v_c: spl_fixed31_32 }
#[repr(C)]
pub struct spl_taps { pub v_taps: u32, pub h_taps: u32, pub v_taps_c: u32, pub h_taps_c: u32, pub integer_scaling: bool }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum spl_view_3d { SPL_VIEW_3D_NONE = 0, SPL_VIEW_3D_FRAME_SEQUENTIAL, SPL_VIEW_3D_SIDE_BY_SIDE, SPL_VIEW_3D_TOP_AND_BOTTOM, SPL_VIEW_3D_COUNT, SPL_VIEW_3D_FIRST = 1 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum spl_pixel_format { SPL_PIXEL_FORMAT_UNINITIALIZED, SPL_PIXEL_FORMAT_INDEX8, SPL_PIXEL_FORMAT_RGB565, SPL_PIXEL_FORMAT_ARGB8888, SPL_PIXEL_FORMAT_ARGB2101010, SPL_PIXEL_FORMAT_ARGB2101010_XRBIAS, SPL_PIXEL_FORMAT_FP16, SPL_PIXEL_FORMAT_420BPP8, SPL_PIXEL_FORMAT_420BPP10, SPL_PIXEL_FORMAT_422BPP8, SPL_PIXEL_FORMAT_422BPP10, SPL_PIXEL_FORMAT_422BPP12, SPL_PIXEL_FORMAT_444BPP8, SPL_PIXEL_FORMAT_444BPP10, SPL_PIXEL_FORMAT_GRPH_BEGIN = 1, SPL_PIXEL_FORMAT_GRPH_END = 6, SPL_PIXEL_FORMAT_SUBSAMPLED_BEGIN = 7, SPL_PIXEL_FORMAT_SUBSAMPLED_END = 11, SPL_PIXEL_FORMAT_VIDEO_BEGIN = 7, SPL_PIXEL_FORMAT_VIDEO_END = 13, SPL_PIXEL_FORMAT_INVALID, SPL_PIXEL_FORMAT_UNKNOWN }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum lb_memory_config { LB_MEMORY_CONFIG_0 = 0, LB_MEMORY_CONFIG_1 = 1, LB_MEMORY_CONFIG_2 = 2, LB_MEMORY_CONFIG_3 = 3 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum spl_rotation_angle { SPL_ROTATION_ANGLE_0 = 0, SPL_ROTATION_ANGLE_90, SPL_ROTATION_ANGLE_180, SPL_ROTATION_ANGLE_270, SPL_ROTATION_ANGLE_COUNT }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum spl_color_space { SPL_COLOR_SPACE_UNKNOWN, SPL_COLOR_SPACE_SRGB, SPL_COLOR_SPACE_XR_RGB, SPL_COLOR_SPACE_SRGB_LIMITED, SPL_COLOR_SPACE_MSREF_SCRGB, SPL_COLOR_SPACE_YCBCR601, SPL_COLOR_SPACE_YCBCR709, SPL_COLOR_SPACE_XV_YCC_709, SPL_COLOR_SPACE_XV_YCC_601, SPL_COLOR_SPACE_YCBCR601_LIMITED, SPL_COLOR_SPACE_YCBCR709_LIMITED, SPL_COLOR_SPACE_2020_RGB_FULLRANGE, SPL_COLOR_SPACE_2020_RGB_LIMITEDRANGE, SPL_COLOR_SPACE_2020_YCBCR, SPL_COLOR_SPACE_ADOBERGB, SPL_COLOR_SPACE_DCIP3, SPL_COLOR_SPACE_DISPLAYNATIVE, SPL_COLOR_SPACE_DOLBYVISION, SPL_COLOR_SPACE_APPCTRL, SPL_COLOR_SPACE_CUSTOMPOINTS, SPL_COLOR_SPACE_YCBCR709_BLACK }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum chroma_cositing { CHROMA_COSITING_NONE, CHROMA_COSITING_LEFT, CHROMA_COSITING_TOPLEFT, CHROMA_COSITING_COUNT }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum upsp_mode { UPSP_BYPASS = 0, UPSP_HORIZONTAL_UPSAMPLING_ONLY, UPSP_VERTICAL_UPSAMPLING_ONLY, UPSP_HORIZONTAL_VERTICAL_UPSAMPLING }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum upsp_num_taps { UPSP_2_TAPS, UPSP_4_TAPS }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum upsp_boundary_mode { UPSP_BOUNDARY_EDGE, UPSP_BOUNDARY_BLACK }

#[repr(C)]
pub struct spl_scaler_data { pub h_active: i32, pub v_active: i32, pub taps: spl_taps, pub viewport: spl_rect, pub viewport_c: spl_rect, pub recout: spl_rect, pub ratios: spl_ratios, pub recip_ratios: spl_ratios, pub inits: spl_inits }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum spl_transfer_func_type { SPL_TF_TYPE_PREDEFINED, SPL_TF_TYPE_DISTRIBUTED_POINTS, SPL_TF_TYPE_BYPASS, SPL_TF_TYPE_HWPWL }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum spl_transfer_func_predefined { SPL_TRANSFER_FUNCTION_SRGB, SPL_TRANSFER_FUNCTION_BT709, SPL_TRANSFER_FUNCTION_PQ, SPL_TRANSFER_FUNCTION_LINEAR, SPL_TRANSFER_FUNCTION_UNITY, SPL_TRANSFER_FUNCTION_HLG, SPL_TRANSFER_FUNCTION_HLG12, SPL_TRANSFER_FUNCTION_GAMMA22, SPL_TRANSFER_FUNCTION_GAMMA24, SPL_TRANSFER_FUNCTION_GAMMA26 }

#[repr(C)] pub struct mpc_size { pub width: u32, pub height: u32 }
#[repr(C)] #[derive(Copy, Clone)] pub enum scl_mode { SCL_MODE_SCALING_444_BYPASS=0, SCL_MODE_SCALING_444_RGB_ENABLE=1, SCL_MODE_SCALING_444_YCBCR_ENABLE=2, SCL_MODE_SCALING_420_YCBCR_ENABLE=3, SCL_MODE_SCALING_420_LUMA_BYPASS=4, SCL_MODE_SCALING_420_CHROMA_BYPASS=5, SCL_MODE_DSCL_BYPASS=6 }
#[repr(C)] pub struct scl_black_color { pub offset_rgb_y: u32, pub offset_rgb_cbcr: u32 }
#[repr(C)] pub struct ratio { pub h_scale_ratio:u32, pub v_scale_ratio:u32, pub h_scale_ratio_c:u32, pub v_scale_ratio_c:u32 }
#[repr(C)] pub struct init { pub h_filter_init_frac:u32, pub h_filter_init_int:u32, pub h_filter_init_frac_c:u32, pub h_filter_init_int_c:u32, pub v_filter_init_frac:u32, pub v_filter_init_int:u32, pub v_filter_init_frac_c:u32, pub v_filter_init_int_c:u32, pub v_filter_init_bot_frac:u32, pub v_filter_init_bot_int:u32, pub v_filter_init_bot_frac_c:u32, pub v_filter_init_bot_int_c:u32 }

#[repr(C)] pub struct isharp_noise_det { pub enable:u32, pub mode:u32, pub uthreshold:u32, pub dthreshold:u32, pub pwl_start_in:u32, pub pwl_end_in:u32, pub pwl_slope:u32 }
#[repr(C)] pub struct isharp_lba { pub mode:u32, pub in_seg:[u32;6], pub base_seg:[u32;6], pub slope_seg:[u32;6] }
#[repr(C)] pub struct isharp_fmt { pub mode:u32, pub norm:u32 }
#[repr(C)] pub struct isharp_nldelta_sclip { pub enable_p:u32, pub pivot_p:u32, pub slope_p:u32, pub enable_n:u32, pub pivot_n:u32, pub slope_n:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub enum isharp_en { ISHARP_DISABLE, ISHARP_ENABLE }
pub const ISHARP_LUT_TABLE_SIZE: usize = 32;

// FILTER and register data are represented literally; repeated EASF register fields retain source order.
#[repr(C)]
pub struct dscl_prog_data {
    pub recout: spl_rect, pub mpc_size: mpc_size, pub dscl_mode:u32, pub scl_black_color:scl_black_color, pub ratios:ratio, pub init:init, pub taps:spl_taps, pub viewport:spl_rect, pub viewport_c:spl_rect,
    pub filter_h:*const u16, pub filter_v:*const u16, pub filter_h_c:*const u16, pub filter_v_c:*const u16,
    pub easf_matrix_mode:u32, pub easf_ltonl_en:u32, pub easf_v_en:u32, pub easf_v_sharp_factor:u32, pub easf_v_ring:u32, pub easf_v_bf1_en:u32, pub easf_v_bf2_mode:u32, pub easf_v_bf3_mode:u32, pub easf_v_bf2_flat1_gain:u32, pub easf_v_bf2_flat2_gain:u32, pub easf_v_bf2_roc_gain:u32, pub easf_v_ringest_3tap_dntilt_uptilt:u32, pub easf_v_ringest_3tap_uptilt_max:u32, pub easf_v_ringest_3tap_dntilt_slope:u32, pub easf_v_ringest_3tap_uptilt1_slope:u32, pub easf_v_ringest_3tap_uptilt2_slope:u32, pub easf_v_ringest_3tap_uptilt2_offset:u32, pub easf_v_ringest_eventap_reduceg1:u32, pub easf_v_ringest_eventap_reduceg2:u32, pub easf_v_ringest_eventap_gain1:u32, pub easf_v_ringest_eventap_gain2:u32, pub easf_v_bf_maxa:u32, pub easf_v_bf_maxb:u32, pub easf_v_bf_mina:u32, pub easf_v_bf_minb:u32,
    pub easf_v_bf1_pwl_in_seg0:u32, pub easf_v_bf1_pwl_base_seg0:u32, pub easf_v_bf1_pwl_slope_seg0:u32, pub easf_v_bf1_pwl_in_seg1:u32, pub easf_v_bf1_pwl_base_seg1:u32, pub easf_v_bf1_pwl_slope_seg1:u32, pub easf_v_bf1_pwl_in_seg2:u32, pub easf_v_bf1_pwl_base_seg2:u32, pub easf_v_bf1_pwl_slope_seg2:u32, pub easf_v_bf1_pwl_in_seg3:u32, pub easf_v_bf1_pwl_base_seg3:u32, pub easf_v_bf1_pwl_slope_seg3:u32, pub easf_v_bf1_pwl_in_seg4:u32, pub easf_v_bf1_pwl_base_seg4:u32, pub easf_v_bf1_pwl_slope_seg4:u32, pub easf_v_bf1_pwl_in_seg5:u32, pub easf_v_bf1_pwl_base_seg5:u32, pub easf_v_bf1_pwl_slope_seg5:u32, pub easf_v_bf1_pwl_in_seg6:u32, pub easf_v_bf1_pwl_base_seg6:u32, pub easf_v_bf1_pwl_slope_seg6:u32, pub easf_v_bf1_pwl_in_seg7:u32, pub easf_v_bf1_pwl_base_seg7:u32, pub easf_v_bf3_pwl_in_set0:u32, pub easf_v_bf3_pwl_base_set0:u32, pub easf_v_bf3_pwl_slope_set0:u32, pub easf_v_bf3_pwl_in_set1:u32, pub easf_v_bf3_pwl_base_set1:u32, pub easf_v_bf3_pwl_slope_set1:u32, pub easf_v_bf3_pwl_in_set2:u32, pub easf_v_bf3_pwl_base_set2:u32, pub easf_v_bf3_pwl_slope_set2:u32, pub easf_v_bf3_pwl_in_set3:u32, pub easf_v_bf3_pwl_base_set3:u32, pub easf_v_bf3_pwl_slope_set3:u32, pub easf_v_bf3_pwl_in_set4:u32, pub easf_v_bf3_pwl_base_set4:u32, pub easf_v_bf3_pwl_slope_set4:u32, pub easf_v_bf3_pwl_in_set5:u32, pub easf_v_bf3_pwl_base_set5:u32, pub easf_v_bf3_pwl_slope_set5:u32,
    pub easf_h_en:u32, pub easf_h_sharp_factor:u32, pub easf_h_ring:u32, pub easf_h_bf1_en:u32, pub easf_h_bf2_mode:u32, pub easf_h_bf3_mode:u32, pub easf_h_bf2_flat1_gain:u32, pub easf_h_bf2_flat2_gain:u32, pub easf_h_bf2_roc_gain:u32, pub easf_h_ringest_eventap_reduceg1:u32, pub easf_h_ringest_eventap_reduceg2:u32, pub easf_h_ringest_eventap_gain1:u32, pub easf_h_ringest_eventap_gain2:u32, pub easf_h_bf_maxa:u32, pub easf_h_bf_maxb:u32, pub easf_h_bf_mina:u32, pub easf_h_bf_minb:u32,
    pub easf_h_bf1_pwl_in_seg0:u32, pub easf_h_bf1_pwl_base_seg0:u32, pub easf_h_bf1_pwl_slope_seg0:u32, pub easf_h_bf1_pwl_in_seg1:u32, pub easf_h_bf1_pwl_base_seg1:u32, pub easf_h_bf1_pwl_slope_seg1:u32, pub easf_h_bf1_pwl_in_seg2:u32, pub easf_h_bf1_pwl_base_seg2:u32, pub easf_h_bf1_pwl_slope_seg2:u32, pub easf_h_bf1_pwl_in_seg3:u32, pub easf_h_bf1_pwl_base_seg3:u32, pub easf_h_bf1_pwl_slope_seg3:u32, pub easf_h_bf1_pwl_in_seg4:u32, pub easf_h_bf1_pwl_base_seg4:u32, pub easf_h_bf1_pwl_slope_seg4:u32, pub easf_h_bf1_pwl_in_seg5:u32, pub easf_h_bf1_pwl_base_seg5:u32, pub easf_h_bf1_pwl_slope_seg5:u32, pub easf_h_bf1_pwl_in_seg6:u32, pub easf_h_bf1_pwl_base_seg6:u32, pub easf_h_bf1_pwl_slope_seg6:u32, pub easf_h_bf1_pwl_in_seg7:u32, pub easf_h_bf1_pwl_base_seg7:u32, pub easf_h_bf3_pwl_in_set0:u32, pub easf_h_bf3_pwl_base_set0:u32, pub easf_h_bf3_pwl_slope_set0:u32, pub easf_h_bf3_pwl_in_set1:u32, pub easf_h_bf3_pwl_base_set1:u32, pub easf_h_bf3_pwl_slope_set1:u32, pub easf_h_bf3_pwl_in_set2:u32, pub easf_h_bf3_pwl_base_set2:u32, pub easf_h_bf3_pwl_slope_set2:u32, pub easf_h_bf3_pwl_in_set3:u32, pub easf_h_bf3_pwl_base_set3:u32, pub easf_h_bf3_pwl_slope_set3:u32, pub easf_h_bf3_pwl_in_set4:u32, pub easf_h_bf3_pwl_base_set4:u32, pub easf_h_bf3_pwl_slope_set4:u32, pub easf_h_bf3_pwl_in_set5:u32, pub easf_h_bf3_pwl_base_set5:u32, pub easf_h_bf3_pwl_slope_set5:u32, pub easf_matrix_c0:u32, pub easf_matrix_c1:u32, pub easf_matrix_c2:u32, pub easf_matrix_c3:u32,
    pub upsp_mode:u32, pub upsp_v_num_taps:u32, pub upsp_v_init_int:u32, pub upsp_v_init_frac:u32, pub upsp_h_num_taps:u32, pub upsp_h_init_int:u32, pub upsp_h_init_frac:u32, pub upsp_boundary_mode:u32, pub upsp_v_coef_tap0_p0:u32, pub upsp_v_coef_tap1_p0:u32, pub upsp_v_coef_tap2_p0:u32, pub upsp_v_coef_tap3_p0:u32, pub upsp_v_coef_tap0_p1:u32, pub upsp_v_coef_tap1_p1:u32, pub upsp_v_coef_tap2_p1:u32, pub upsp_v_coef_tap3_p1:u32, pub upsp_h_coef_tap0_p0:u32, pub upsp_h_coef_tap1_p0:u32, pub upsp_h_coef_tap2_p0:u32, pub upsp_h_coef_tap3_p0:u32, pub upsp_h_coef_tap0_p1:u32, pub upsp_h_coef_tap1_p1:u32, pub upsp_h_coef_tap2_p1:u32, pub upsp_h_coef_tap3_p1:u32, pub upsp_clamp_max:u32, pub upsp_clamp_min:u32,
    pub isharp_en:u32, pub isharp_noise_det:isharp_noise_det, pub isharp_nl_en:u32, pub isharp_lba:isharp_lba, pub isharp_fmt:isharp_fmt, pub isharp_delta:[u32;ISHARP_LUT_TABLE_SIZE], pub isharp_nldelta_sclip:isharp_nldelta_sclip, pub filter_blur_scale_v:*const u16, pub filter_blur_scale_h:*const u16, pub sharpness_level:i32
}

#[repr(C)] pub struct spl_scratch { pub scl_data:spl_scaler_data }
#[repr(C)] pub struct spl_out { pub dscl_prog_data:*mut dscl_prog_data }
#[repr(C)] pub struct spl_opp_adjust { pub x:i32, pub y:i32, pub width:i32, pub height:i32 }
#[repr(C)] pub union num_slices_recout_width { pub mpc_num_h_slices:i32, pub mpc_recout_width_align:i32 }
#[repr(C)] pub struct basic_in { pub format:spl_pixel_format, pub cositing:chroma_cositing, pub src_rect:spl_rect, pub dst_rect:spl_rect, pub clip_rect:spl_rect, pub rotation:spl_rotation_angle, pub horizontal_mirror:bool, pub num_h_slices_recout_width_align: num_h_slices_recout_width_align, pub mpc_h_slice_index:i32, pub opp_recout_adjust:spl_opp_adjust, pub tf_type:spl_transfer_func_type, pub tf_predefined_type:spl_transfer_func_predefined, pub color_space:spl_color_space, pub max_luminance:u32, pub film_grain_applied:bool, pub custom_width:i32, pub custom_x:i32 }
#[repr(C)] pub struct num_h_slices_recout_width_align { pub use_recout_width_aligned:bool, pub num_slices_recout_width:num_slices_recout_width }
#[repr(C)] pub struct basic_out { pub output_size:spl_size, pub dst_rect:spl_rect, pub src_rect:spl_rect, pub odm_combine_factor:i32, pub odm_slice_rect:spl_rect, pub view_format:spl_view_3d, pub always_scale:bool, pub max_downscale_src_width:i32, pub alpha_en:bool, pub use_two_pixels_per_container:bool }
#[repr(C)] #[derive(Copy,Clone)] pub enum sharpness_setting { SHARPNESS_HW_OFF=0, SHARPNESS_ZERO, SHARPNESS_CUSTOM }
#[repr(C)] #[derive(Copy,Clone)] pub enum sharpness_range_source { SHARPNESS_RANGE_DCN=0, SHARPNESS_RANGE_DCN_OVERRIDE }
#[repr(C)] pub struct spl_sharpness_range { pub sdr_rgb_min:i32,pub sdr_rgb_max:i32,pub sdr_rgb_mid:i32,pub sdr_yuv_min:i32,pub sdr_yuv_max:i32,pub sdr_yuv_mid:i32,pub hdr_rgb_min:i32,pub hdr_rgb_max:i32,pub hdr_rgb_mid:i32 }
#[repr(C)] pub struct adaptive_sharpness { pub enable:bool, pub sharpness_level:u32, pub sharpness_range:spl_sharpness_range }
#[repr(C)] #[derive(Copy,Clone)] pub enum linear_light_scaling { LLS_PREF_DONT_CARE=0, LLS_PREF_YES, LLS_PREF_NO }
#[repr(C)] #[derive(Copy,Clone)] pub enum sharpen_policy { SHARPEN_ALWAYS=0, SHARPEN_YUV=1, SHARPEN_RGB_FULLSCREEN_YUV=2, SHARPEN_FULLSCREEN_ALL=3 }
#[repr(C)] #[derive(Copy,Clone)] pub enum scale_to_sharpness_policy { NO_SCALE_TO_SHARPNESS_ADJ=0, SCALE_TO_SHARPNESS_ADJ_YUV=1, SCALE_TO_SHARPNESS_ADJ_ALL=2 }
#[repr(C)] pub struct spl_callbacks { pub spl_calc_lb_num_partitions: Option<unsafe extern "C" fn(bool,*const spl_scaler_data,lb_memory_config,*mut i32,*mut i32)> }
#[repr(C)] pub struct spl_debug { pub visual_confirm_base_offset:i32, pub visual_confirm_dpp_offset:i32, pub scale_to_sharpness_policy:scale_to_sharpness_policy }
#[repr(C)] pub struct spl_in { pub basic_out:basic_out, pub basic_in:basic_in, pub odm_slice_index:i32, pub scaling_quality:spl_taps, pub callbacks:spl_callbacks, pub adaptive_sharpness:adaptive_sharpness, pub lls_pref:linear_light_scaling, pub prefer_easf:bool, pub disable_easf:bool, pub override_easf:bool, pub debug:spl_debug, pub is_fullscreen:bool, pub is_hdr_on:bool, pub h_active:i32, pub v_active:i32, pub min_viewport_size:i32, pub sdr_white_level_nits:i32, pub sharpen_policy:sharpen_policy, pub upsp_mode:upsp_mode }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
