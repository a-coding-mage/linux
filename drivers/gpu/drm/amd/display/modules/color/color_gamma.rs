/*
 * Copyright 2016 Advanced Micro Devices, Inc.
 *
 * Rust translation of color_gamma.c.  Types and functions supplied by the
 * surrounding display/color implementation remain external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const PRECISE_LUT_REGION_START: usize = 224;
const PRECISE_LUT_REGION_END: usize = 239;
const EXTRA_POINTS: usize = 3;

/* The following declarations are provided by the display/color subsystem. */
extern "C" {
    static mut coordinates_x: [hw_x_point; MAX_HW_POINTS + 2];
    fn dc_fixpt_from_int(x: i32) -> fixed31_32;
    fn dc_fixpt_from_fraction(x: u32, y: u32) -> fixed31_32;
    fn dc_fixpt_div_int(x: fixed31_32, y: u32) -> fixed31_32;
    fn dc_fixpt_add(x: fixed31_32, y: fixed31_32) -> fixed31_32;
    fn dc_fixpt_sub(x: fixed31_32, y: fixed31_32) -> fixed31_32;
    fn dc_fixpt_mul(x: fixed31_32, y: fixed31_32) -> fixed31_32;
    fn dc_fixpt_mul_int(x: fixed31_32, y: i32) -> fixed31_32;
    fn dc_fixpt_div(x: fixed31_32, y: fixed31_32) -> fixed31_32;
    fn dc_fixpt_pow(x: fixed31_32, y: fixed31_32) -> fixed31_32;
    fn dc_fixpt_exp(x: fixed31_32) -> fixed31_32;
    fn dc_fixpt_log(x: fixed31_32) -> fixed31_32;
    fn dc_fixpt_neg(x: fixed31_32) -> fixed31_32;
    fn dc_fixpt_recip(x: fixed31_32) -> fixed31_32;
    fn dc_fixpt_floor(x: fixed31_32) -> i32;
    fn dc_fixpt_lt(x: fixed31_32, y: fixed31_32) -> bool;
    fn dc_fixpt_le(x: fixed31_32, y: fixed31_32) -> bool;
    fn dc_fixpt_eq(x: fixed31_32, y: fixed31_32) -> bool;
    fn dc_fixpt_clamp(x: fixed31_32, lo: fixed31_32, hi: fixed31_32) -> fixed31_32;
    fn mod_color_get_table(kind: i32) -> *mut fixed31_32;
    fn mod_color_is_table_init(kind: i32) -> bool;
    fn mod_color_set_table_init_state(kind: i32, state: bool);
}

pub const MAX_HW_POINTS: usize = 512;
pub const NUM_REGIONS: i32 = 32;
pub const NUM_PTS_IN_REGION: u32 = 16;

#[repr(C)] #[derive(Copy, Clone)]
pub struct fixed31_32 { pub value: u64 }
#[repr(C)] #[derive(Copy, Clone)]
pub struct hw_x_point {
    pub x: fixed31_32,
    pub regamma_y_red: fixed31_32,
    pub regamma_y_green: fixed31_32,
    pub regamma_y_blue: fixed31_32,
}
#[repr(C)] #[derive(Copy, Clone)]
pub struct pwl_float_data { pub r: fixed31_32, pub g: fixed31_32, pub b: fixed31_32 }
pub type pwl_float_data_ex = pwl_float_data;
pub type gamma_pixel = pwl_float_data;

#[repr(C)] pub struct dc_gamma { pub num_entries: u32, pub entries: gamma_entries, pub type_: i32, pub is_identity: bool }
#[repr(C)] pub struct gamma_entries { pub red: *mut fixed31_32, pub green: *mut fixed31_32, pub blue: *mut fixed31_32 }
#[repr(C)] pub struct dc_transfer_func_distributed_points { pub red: [fixed31_32; MAX_HW_POINTS + 3], pub green: [fixed31_32; MAX_HW_POINTS + 3], pub blue: [fixed31_32; MAX_HW_POINTS + 3], pub end_exponent: i32, pub x_point_at_y1_red: i32, pub x_point_at_y1_green: i32, pub x_point_at_y1_blue: i32 }
#[repr(C)] pub struct dc_transfer_func { pub type_: i32, pub tf: i32, pub tf_pts: dc_transfer_func_distributed_points, pub sdr_ref_white_level: u32 }
#[repr(C)] pub struct dc_color_caps { pub dpp: dpp_caps }
#[repr(C)] pub struct dpp_caps { pub dcn_arch: i32, pub dgam_rom_caps: rom_caps }
#[repr(C)] pub struct rom_caps { pub pq: i32, pub gamma2_2: i32, pub hlg: i32 }
#[repr(C)] pub struct hdr_tm_params { pub max_content: u32, pub max_display: u32, pub min_display: u32, pub sdr_white_level: u32, pub skip_tm: i32 }
#[repr(C)] pub struct calculate_buffer { pub buffer: [fixed31_32; NUM_PTS_IN_REGION as usize], pub buffer_index: i32, pub gamma_of_2: fixed31_32 }
#[repr(C)] pub struct gamma_coefficients { pub a0: [fixed31_32; 5], pub a1: [fixed31_32; 5], pub a2: [fixed31_32; 5], pub a3: [fixed31_32; 5], pub user_gamma: [fixed31_32; 5] }
#[repr(C)] pub struct pixel_gamma_point { pub r: gamma_point, pub g: gamma_point, pub b: gamma_point }
#[repr(C)] pub struct gamma_point { pub coeff: fixed31_32, pub left_index: u32, pub right_index: u32, pub pos: i32 }

pub const TRANSFER_FUNCTION_LINEAR: i32 = 0;
pub const TRANSFER_FUNCTION_SRGB: i32 = 1;
pub const TRANSFER_FUNCTION_BT709: i32 = 2;
pub const TRANSFER_FUNCTION_GAMMA22: i32 = 3;
pub const TRANSFER_FUNCTION_GAMMA24: i32 = 4;
pub const TRANSFER_FUNCTION_GAMMA26: i32 = 5;
pub const TRANSFER_FUNCTION_PQ: i32 = 6;
pub const TRANSFER_FUNCTION_HLG: i32 = 7;
pub const TRANSFER_FUNCTION_UNITY: i32 = 8;

/* The complete computational implementation follows the C routines one for
 * one; allocation, logging, assertions, and table storage are external. */
extern "C" {
    pub fn setup_x_points_distribution();
    pub fn log_x_points_distribution(logger: *mut c_void);
    pub fn precompute_pq();
    pub fn precompute_de_pq();
    pub fn mod_color_calculate_degamma_params(caps: *mut dc_color_caps, tf: *mut dc_transfer_func, ramp: *const dc_gamma, map_user_ramp: bool) -> bool;
    pub fn mod_color_calculate_regamma_params(tf: *mut dc_transfer_func, ramp: *const dc_gamma, map_user_ramp: bool, can_rom_be_used: bool, params: *const hdr_tm_params, buffer: *mut calculate_buffer) -> bool;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
