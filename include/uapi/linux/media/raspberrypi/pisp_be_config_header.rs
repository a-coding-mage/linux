/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/* PiSP Back End configuration definitions. */

// C dependencies supplied by the surrounding UAPI translation.
use crate::{pisp_bla_config, pisp_compress_config, pisp_decompress_config,
    pisp_image_format_config, pisp_wbg_config};

pub const PISP_BACK_END_INPUT_ALIGN: u32 = 4;
pub const PISP_BACK_END_COMPRESSED_ALIGN: u32 = 8;
pub const PISP_BACK_END_OUTPUT_MIN_ALIGN: u32 = 16;
pub const PISP_BACK_END_OUTPUT_MAX_ALIGN: u32 = 64;
pub const PISP_BACK_END_MIN_TILE_WIDTH: u32 = 16;
pub const PISP_BACK_END_MIN_TILE_HEIGHT: u32 = 16;
pub const PISP_BACK_END_MAX_TILE_WIDTH: u32 = 65536;
pub const PISP_BACK_END_MAX_TILE_HEIGHT: u32 = 65536;
pub const PISP_BACK_END_NUM_OUTPUTS: usize = 2;
pub const PISP_BACK_END_HOG_OUTPUT: usize = 1;
pub const PISP_BACK_END_NUM_TILES: usize = 64;

#[repr(u32)] pub enum pisp_be_bayer_enable { PISP_BE_BAYER_ENABLE_INPUT=0x000001, PISP_BE_BAYER_ENABLE_DECOMPRESS=0x000002, PISP_BE_BAYER_ENABLE_DPC=0x000004, PISP_BE_BAYER_ENABLE_GEQ=0x000008, PISP_BE_BAYER_ENABLE_TDN_INPUT=0x000010, PISP_BE_BAYER_ENABLE_TDN_DECOMPRESS=0x000020, PISP_BE_BAYER_ENABLE_TDN=0x000040, PISP_BE_BAYER_ENABLE_TDN_COMPRESS=0x000080, PISP_BE_BAYER_ENABLE_TDN_OUTPUT=0x000100, PISP_BE_BAYER_ENABLE_SDN=0x000200, PISP_BE_BAYER_ENABLE_BLC=0x000400, PISP_BE_BAYER_ENABLE_STITCH_INPUT=0x000800, PISP_BE_BAYER_ENABLE_STITCH_DECOMPRESS=0x001000, PISP_BE_BAYER_ENABLE_STITCH=0x002000, PISP_BE_BAYER_ENABLE_STITCH_COMPRESS=0x004000, PISP_BE_BAYER_ENABLE_STITCH_OUTPUT=0x008000, PISP_BE_BAYER_ENABLE_WBG=0x010000, PISP_BE_BAYER_ENABLE_CDN=0x020000, PISP_BE_BAYER_ENABLE_LSC=0x040000, PISP_BE_BAYER_ENABLE_TONEMAP=0x080000, PISP_BE_BAYER_ENABLE_CAC=0x100000, PISP_BE_BAYER_ENABLE_DEBIN=0x200000, PISP_BE_BAYER_ENABLE_DEMOSAIC=0x400000 }
#[repr(u32)] pub enum pisp_be_rgb_enable { PISP_BE_RGB_ENABLE_INPUT=1, PISP_BE_RGB_ENABLE_CCM=2, PISP_BE_RGB_ENABLE_SAT_CONTROL=4, PISP_BE_RGB_ENABLE_YCBCR=8, PISP_BE_RGB_ENABLE_FALSE_COLOUR=0x10, PISP_BE_RGB_ENABLE_SHARPEN=0x20, PISP_BE_RGB_ENABLE_YCBCR_INVERSE=0x80, PISP_BE_RGB_ENABLE_GAMMA=0x100, PISP_BE_RGB_ENABLE_CSC0=0x200, PISP_BE_RGB_ENABLE_CSC1=0x400, PISP_BE_RGB_ENABLE_DOWNSCALE0=0x1000, PISP_BE_RGB_ENABLE_DOWNSCALE1=0x2000, PISP_BE_RGB_ENABLE_RESAMPLE0=0x8000, PISP_BE_RGB_ENABLE_RESAMPLE1=0x10000, PISP_BE_RGB_ENABLE_OUTPUT0=0x40000, PISP_BE_RGB_ENABLE_OUTPUT1=0x80000, PISP_BE_RGB_ENABLE_HOG=0x200000 }
pub const fn PISP_BE_RGB_ENABLE_CSC(i: u32) -> u32 { 0x200 << i }
pub const fn PISP_BE_RGB_ENABLE_DOWNSCALE(i: u32) -> u32 { 0x1000 << i }
pub const fn PISP_BE_RGB_ENABLE_RESAMPLE(i: u32) -> u32 { 0x8000 << i }
pub const fn PISP_BE_RGB_ENABLE_OUTPUT(i: u32) -> u32 { 0x40000 << i }
#[repr(u32)] pub enum pisp_be_dirty { PISP_BE_DIRTY_GLOBAL=1, PISP_BE_DIRTY_SH_FC_COMBINE=2, PISP_BE_DIRTY_CROP=4 }

#[repr(C, packed)] pub struct pisp_be_global_config { pub bayer_enables:u32, pub rgb_enables:u32, pub bayer_order:u8, pub pad:[u8;3] }
#[repr(C, packed)] pub struct pisp_be_input_buffer_config { pub addr:[[u32;2];3] }
pub const PISP_BE_DPC_FLAG_FOLDBACK: u8 = 1;
#[repr(C, packed)] pub struct pisp_be_dpc_config { pub coeff_level:u8, pub coeff_range:u8, pub pad:u8, pub flags:u8 }
pub const PISP_BE_GEQ_SHARPER: u16 = 1 << 15;
pub const PISP_BE_GEQ_SLOPE: u16 = (1 << 10) - 1;
#[repr(C, packed)] pub struct pisp_be_geq_config { pub offset:u16, pub slope_sharper:u16, pub min:u16, pub max:u16 }
#[repr(C, packed)] pub struct pisp_be_tdn_input_buffer_config { pub addr:[u32;2] }
#[repr(C, packed)] pub struct pisp_be_tdn_config { pub black_level:u16, pub ratio:u16, pub noise_constant:u16, pub noise_slope:u16, pub threshold:u16, pub reset:u8, pub pad:u8 }
#[repr(C, packed)] pub struct pisp_be_tdn_output_buffer_config { pub addr:[u32;2] }
#[repr(C, packed)] pub struct pisp_be_sdn_config { pub black_level:u16, pub leakage:u8, pub pad:u8, pub noise_constant:u16, pub noise_slope:u16, pub noise_constant2:u16, pub noise_slope2:u16 }
#[repr(C, packed)] pub struct pisp_be_stitch_input_buffer_config { pub addr:[u32;2] }
pub const PISP_BE_STITCH_STREAMING_LONG:u16=0x8000; pub const PISP_BE_STITCH_EXPOSURE_RATIO_MASK:u16=0x7fff;
#[repr(C, packed)] pub struct pisp_be_stitch_config { pub threshold_lo:u16, pub threshold_diff_power:u8, pub pad:u8, pub exposure_ratio:u16, pub motion_threshold_256:u8, pub motion_threshold_recip:u8 }
#[repr(C, packed)] pub struct pisp_be_stitch_output_buffer_config { pub addr:[u32;2] }
#[repr(C, packed)] pub struct pisp_be_cdn_config { pub thresh:u16, pub iir_strength:u8, pub g_adjust:u8 }
pub const PISP_BE_LSC_LOG_GRID_SIZE:usize=5; pub const PISP_BE_LSC_GRID_SIZE:usize=1<<PISP_BE_LSC_LOG_GRID_SIZE; pub const PISP_BE_LSC_STEP_PRECISION:usize=18; pub const PISP_BE_LSC_LUT_SIZE:usize=PISP_BE_LSC_GRID_SIZE+1;
#[repr(C, packed)] pub struct pisp_be_lsc_config { pub grid_step_x:u16, pub grid_step_y:u16, pub lut_packed:[[u32;PISP_BE_LSC_LUT_SIZE];PISP_BE_LSC_LUT_SIZE] }
#[repr(C, packed)] pub struct pisp_be_lsc_extra { pub offset_x:u16, pub offset_y:u16 }
pub const PISP_BE_CAC_LOG_GRID_SIZE:usize=3; pub const PISP_BE_CAC_GRID_SIZE:usize=1<<PISP_BE_CAC_LOG_GRID_SIZE; pub const PISP_BE_CAC_STEP_PRECISION:usize=20; pub const PISP_BE_CAC_LUT_SIZE:usize=PISP_BE_CAC_GRID_SIZE+1;
#[repr(C, packed)] pub struct pisp_be_cac_config { pub grid_step_x:u16, pub grid_step_y:u16, pub lut:[[[[i8;2];2];PISP_BE_CAC_LUT_SIZE];PISP_BE_CAC_LUT_SIZE] }
#[repr(C, packed)] pub struct pisp_be_cac_extra { pub offset_x:u16, pub offset_y:u16 }
pub const PISP_BE_DEBIN_NUM_COEFFS:usize=4;
#[repr(C, packed)] pub struct pisp_be_debin_config { pub coeffs:[i8;PISP_BE_DEBIN_NUM_COEFFS], pub h_enable:i8, pub v_enable:i8, pub pad:[i8;2] }
pub const PISP_BE_TONEMAP_LUT_SIZE:usize=64;
#[repr(C, packed)] pub struct pisp_be_tonemap_config { pub detail_constant:u16, pub detail_slope:u16, pub iir_strength:u16, pub strength:u16, pub lut:[u32;PISP_BE_TONEMAP_LUT_SIZE] }
#[repr(C, packed)] pub struct pisp_be_demosaic_config { pub sharper:u8, pub fc_mode:u8, pub pad:[u8;2] }
#[repr(C, packed)] pub struct pisp_be_ccm_config { pub coeffs:[i16;9], pub pad:[u8;2], pub offsets:[i32;3] }
#[repr(C, packed)] pub struct pisp_be_sat_control_config { pub shift_r:u8, pub shift_g:u8, pub shift_b:u8, pub pad:u8 }
#[repr(C, packed)] pub struct pisp_be_false_colour_config { pub distance:u8, pub pad:[u8;3] }
pub const PISP_BE_SHARPEN_SIZE:usize=5; pub const PISP_BE_SHARPEN_FUNC_NUM_POINTS:usize=9;
#[repr(C, packed)] pub struct pisp_be_sharpen_config { pub kernel0:[i8;25],pub pad0:[i8;3],pub kernel1:[i8;25],pub pad1:[i8;3],pub kernel2:[i8;25],pub pad2:[i8;3],pub kernel3:[i8;25],pub pad3:[i8;3],pub kernel4:[i8;25],pub pad4:[i8;3], pub threshold_offset0:u16,pub threshold_slope0:u16,pub scale0:u16,pub pad5:u16,pub threshold_offset1:u16,pub threshold_slope1:u16,pub scale1:u16,pub pad6:u16,pub threshold_offset2:u16,pub threshold_slope2:u16,pub scale2:u16,pub pad7:u16,pub threshold_offset3:u16,pub threshold_slope3:u16,pub scale3:u16,pub pad8:u16,pub threshold_offset4:u16,pub threshold_slope4:u16,pub scale4:u16,pub pad9:u16,pub positive_strength:u16,pub positive_pre_limit:u16,pub positive_func:[u16;9],pub positive_limit:u16,pub negative_strength:u16,pub negative_pre_limit:u16,pub negative_func:[u16;9],pub negative_limit:u16,pub enables:u8,pub white:u8,pub black:u8,pub grey:u8 }
#[repr(C, packed)] pub struct pisp_be_sh_fc_combine_config { pub y_factor:u8,pub c1_factor:u8,pub c2_factor:u8,pub pad:u8 }
pub const PISP_BE_GAMMA_LUT_SIZE:usize=64;
#[repr(C, packed)] pub struct pisp_be_gamma_config { pub lut:[u32;PISP_BE_GAMMA_LUT_SIZE] }
#[repr(C, packed)] pub struct pisp_be_crop_config { pub offset_x:u16,pub offset_y:u16,pub width:u16,pub height:u16 }
pub const PISP_BE_RESAMPLE_FILTER_SIZE:usize=96;
#[repr(C, packed)] pub struct pisp_be_resample_config { pub scale_factor_h:u16,pub scale_factor_v:u16,pub coef:[i16;PISP_BE_RESAMPLE_FILTER_SIZE] }
#[repr(C, packed)] pub struct pisp_be_resample_extra { pub scaled_width:u16,pub scaled_height:u16,pub initial_phase_h:[i16;3],pub initial_phase_v:[i16;3] }
#[repr(C, packed)] pub struct pisp_be_downscale_config { pub scale_factor_h:u16,pub scale_factor_v:u16,pub scale_recip_h:u16,pub scale_recip_v:u16 }
#[repr(C, packed)] pub struct pisp_be_downscale_extra { pub scaled_width:u16,pub scaled_height:u16 }
#[repr(C, packed)] pub struct pisp_be_hog_config { pub compute_signed:u8,pub channel_mix:[u8;3],pub stride:u32 }
#[repr(C, packed)] pub struct pisp_be_axi_config { pub r_qos:u8,pub r_cache_prot:u8,pub w_qos:u8,pub w_cache_prot:u8 }
#[repr(u32)] pub enum pisp_be_transform { PISP_BE_TRANSFORM_NONE=0, PISP_BE_TRANSFORM_HFLIP=1, PISP_BE_TRANSFORM_VFLIP=2, PISP_BE_TRANSFORM_ROT180=3 }
#[repr(C, packed)] pub struct pisp_be_output_format_config { pub image:pisp_image_format_config,pub transform:u8,pub pad:[u8;3],pub lo:u16,pub hi:u16,pub lo2:u16,pub hi2:u16 }
#[repr(C, packed)] pub struct pisp_be_output_buffer_config { pub addr:[[u32;2];3] }
#[repr(C, packed)] pub struct pisp_be_hog_buffer_config { pub addr:[u32;2] }

#[repr(u32)] pub enum pisp_tile_edge { PISP_LEFT_EDGE=1, PISP_RIGHT_EDGE=2, PISP_TOP_EDGE=4, PISP_BOTTOM_EDGE=8 }
#[repr(C, packed)] pub struct pisp_tile { pub edge:u8,pub pad0:[u8;3],pub input_addr_offset:u32,pub input_addr_offset2:u32,pub input_offset_x:u16,pub input_offset_y:u16,pub input_width:u16,pub input_height:u16,pub tdn_input_addr_offset:u32,pub tdn_output_addr_offset:u32,pub stitch_input_addr_offset:u32,pub stitch_output_addr_offset:u32,pub lsc_grid_offset_x:u32,pub lsc_grid_offset_y:u32,pub cac_grid_offset_x:u32,pub cac_grid_offset_y:u32,pub crop_x_start:[u16;2],pub crop_x_end:[u16;2],pub crop_y_start:[u16;2],pub crop_y_end:[u16;2],pub downscale_phase_x:[u16;6],pub downscale_phase_y:[u16;6],pub resample_in_width:[u16;2],pub resample_in_height:[u16;2],pub resample_phase_x:[u16;6],pub resample_phase_y:[u16;6],pub output_offset_x:[u16;2],pub output_offset_y:[u16;2],pub output_width:[u16;2],pub output_height:[u16;2],pub output_addr_offset:[u32;2],pub output_addr_offset2:[u32;2],pub output_hog_addr_offset:u32 }

#[repr(C, packed)] pub struct pisp_be_config { pub input_buffer:pisp_be_input_buffer_config,pub tdn_input_buffer:pisp_be_tdn_input_buffer_config,pub stitch_input_buffer:pisp_be_stitch_input_buffer_config,pub tdn_output_buffer:pisp_be_tdn_output_buffer_config,pub stitch_output_buffer:pisp_be_stitch_output_buffer_config,pub output_buffer:[pisp_be_output_buffer_config;2],pub hog_buffer:pisp_be_hog_buffer_config,pub global:pisp_be_global_config,pub input_format:pisp_image_format_config,pub decompress:pisp_decompress_config,pub dpc:pisp_be_dpc_config,pub geq:pisp_be_geq_config,pub tdn_input_format:pisp_image_format_config,pub tdn_decompress:pisp_decompress_config,pub tdn:pisp_be_tdn_config,pub tdn_compress:pisp_compress_config,pub tdn_output_format:pisp_image_format_config,pub sdn:pisp_be_sdn_config,pub blc:pisp_bla_config,pub stitch_compress:pisp_compress_config,pub stitch_output_format:pisp_image_format_config,pub stitch_input_format:pisp_image_format_config,pub stitch_decompress:pisp_decompress_config,pub stitch:pisp_be_stitch_config,pub lsc:pisp_be_lsc_config,pub wbg:pisp_wbg_config,pub cdn:pisp_be_cdn_config,pub cac:pisp_be_cac_config,pub debin:pisp_be_debin_config,pub tonemap:pisp_be_tonemap_config,pub demosaic:pisp_be_demosaic_config,pub ccm:pisp_be_ccm_config,pub sat_control:pisp_be_sat_control_config,pub ycbcr:pisp_be_ccm_config,pub sharpen:pisp_be_sharpen_config,pub false_colour:pisp_be_false_colour_config,pub sh_fc_combine:pisp_be_sh_fc_combine_config,pub ycbcr_inverse:pisp_be_ccm_config,pub gamma:pisp_be_gamma_config,pub csc:[pisp_be_ccm_config;2],pub downscale:[pisp_be_downscale_config;2],pub resample:[pisp_be_resample_config;2],pub output_format:[pisp_be_output_format_config;2],pub hog:pisp_be_hog_config,pub axi:pisp_be_axi_config,pub lsc_extra:pisp_be_lsc_extra,pub cac_extra:pisp_be_cac_extra,pub downscale_extra:[pisp_be_downscale_extra;2],pub resample_extra:[pisp_be_resample_extra;2],pub crop:pisp_be_crop_config,pub hog_format:pisp_image_format_config,pub dirty_flags_bayer:u32,pub dirty_flags_rgb:u32,pub dirty_flags_extra:u32 }
#[repr(C, packed)] pub struct pisp_be_tiles_config { pub config:pisp_be_config,pub tiles:[pisp_tile;64],pub num_tiles:u32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
