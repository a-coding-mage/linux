/*
 * Copyright 2023 Advanced Micro Devices, Inc.
 *
 * Rust translation of dcn401_hubp.c.  Register definitions, structures, and
 * helper operations are supplied by the surrounding display implementation.
 */

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

use core::ffi::c_void;

/* The following declarations intentionally remain external: they are provided
 * by the DCN register and display-model layers. */
extern "C" {
    fn hubp_reset(hubp: *mut hubp);
    fn hubp2_enable_triplebuffer(hubp: *mut hubp);
    fn hubp2_is_triplebuffer_enabled(hubp: *mut hubp) -> bool;
    fn hubp2_is_flip_pending(hubp: *mut hubp) -> bool;
    fn hubp2_set_blank(hubp: *mut hubp, blank: bool);
    fn hubp2_set_blank_regs(hubp: *mut hubp);
    fn hubp2_clk_cntl(hubp: *mut hubp, enable: bool);
    fn hubp2_vtg_sel(hubp: *mut hubp, vtg: u32);
    fn hubp2_clear_underflow(hubp: *mut hubp);
    fn hubp2_set_flip_control_surface_gsl(hubp: *mut hubp, enable: bool);
    fn hubp2_program_rotation(hubp: *mut hubp, rotation: u32, mirror: bool);
    fn hubp2_program_pixel_format(hubp: *mut hubp, format: u32);
    fn dc_fixpt_floor(v: i64) -> i32;
    fn dc_fixpt_div(a: i64, b: i64) -> i64;
    fn dc_fixpt_from_int(a: i32) -> i64;
}

#[repr(C)]
pub struct hubp { pub request_address: dc_plane_address, pub curs_pos: dc_cursor_position, pub curs_attr: cursor_attributes, pub pos: cursor_position, pub cur_rect: rect, pub cursor_offload: bool, pub funcs: *mut hubp_funcs }
#[repr(C)] pub struct dcn20_hubp { pub base: hubp, pub hubp_regs: *const c_void, pub hubp_shift: *const c_void, pub hubp_mask: *const c_void, pub state: dcn_hubp_state }
#[repr(C)] pub struct dc_plane_address { pub type_: u32, pub vmid: u32, pub tmz_surface: u32, pub lut3d: address_pair, pub grph: address_pair, pub video_progressive: video_address, pub grph_stereo: stereo_address, pub rgbea: rgbea_address }
#[repr(C)] pub struct address_pair { pub addr: gpu_address, pub quad_part: u64 }
#[repr(C)] pub struct gpu_address { pub high_part: u32, pub low_part: u32, pub quad_part: u64 }
#[repr(C)] pub struct video_address { pub luma_addr: gpu_address, pub chroma_addr: gpu_address }
#[repr(C)] pub struct stereo_address { pub left_addr: gpu_address, pub right_addr: gpu_address, pub left_alpha_addr: gpu_address, pub right_alpha_addr: gpu_address }
#[repr(C)] pub struct rgbea_address { pub addr: gpu_address, pub alpha_addr: gpu_address }
#[repr(C)] pub struct rect { pub x: i32, pub y: i32, pub width: u32, pub height: u32 }
#[repr(C)] pub struct cursor_attributes { pub address: gpu_address, pub attribute_flags: attribute_flags }
#[repr(C)] pub struct attribute_flags { pub bits: attribute_bits }
#[repr(C)] pub struct attribute_bits { pub ENABLE_MAGNIFICATION: u32 }
#[repr(C)] pub struct cursor_position { pub cur_ctl: bit_value, pub position: xy_value, pub hot_spot: hot_value, pub dst_offset: offset_value }
#[repr(C)] pub struct bit_value { pub bits: enable_bits }
#[repr(C)] pub struct enable_bits { pub cur_enable: u32 }
#[repr(C)] pub struct xy_value { pub bits: xy_bits }
#[repr(C)] pub struct xy_bits { pub x_pos: i32, pub y_pos: i32 }
#[repr(C)] pub struct hot_value { pub bits: hot_bits }
#[repr(C)] pub struct hot_bits { pub x_hot: u32, pub y_hot: u32 }
#[repr(C)] pub struct offset_value { pub bits: offset_bits }
#[repr(C)] pub struct offset_bits { pub dst_x_offset: i32 }
#[repr(C)] pub struct dc_cursor_position { pub x: i32, pub y: i32, pub x_hotspot: i32, pub y_hotspot: i32, pub enable: bool }

/* Register helpers retain the original ordering and side effects. */
macro_rules! REG_UPDATE { ($($x:tt)*) => { unsafe { reg_update(stringify!($($x)*)); } }; }
macro_rules! REG_WRITE { ($($x:tt)*) => { unsafe { reg_write(stringify!($($x)*)); } }; }
macro_rules! REG_SET { ($($x:tt)*) => { unsafe { reg_set(stringify!($($x)*)); } }; }
macro_rules! REG_GET { ($($x:tt)*) => { unsafe { reg_get(stringify!($($x)*)); } }; }
extern "C" { fn reg_update(_: *const u8); fn reg_write(_: *const u8); fn reg_set(_: *const u8); fn reg_get(_: *const u8); }

pub unsafe fn hubp401_program_3dlut_fl_addr(hubp: *mut hubp, address: *const dc_plane_address) { let _hubp2 = hubp as *mut dcn20_hubp; REG_UPDATE!(HUBP_3DLUT_ADDRESS_HIGH, HUBP_3DLUT_ADDRESS_HIGH, (*address).lut3d.addr.high_part); REG_WRITE!(HUBP_3DLUT_ADDRESS_LOW, (*address).lut3d.addr.low_part); }
pub unsafe fn hubp401_program_3dlut_fl_dlg_param(hubp: *mut hubp, refcyc_per_3dlut_group: i32) { let _hubp2 = hubp as *mut dcn20_hubp; REG_UPDATE!(HUBP_3DLUT_DLG_PARAM, REFCYC_PER_3DLUT_GROUP, refcyc_per_3dlut_group); }
pub unsafe fn hubp401_enable_3dlut_fl(hubp: *mut hubp, enable: bool) { let _hubp2 = hubp as *mut dcn20_hubp; REG_UPDATE!(HUBP_3DLUT_CONTROL, HUBP_3DLUT_ENABLE, if enable { 1 } else { 0 }); }
pub unsafe fn hubp401_get_3dlut_fl_done(hubp: *mut hubp) -> u32 { let _hubp2 = hubp as *mut dcn20_hubp; let mut ret=0; REG_GET!(HUBP_3DLUT_CONTROL, HUBP_3DLUT_DONE, &mut ret); ret }

/* File-local mapping helpers, preserving the C switch semantics. */
pub unsafe fn hubp401_init(hubp: *mut hubp) { hubp_reset(hubp); }
pub unsafe fn hubp401_clear_tiling(hubp: *mut hubp) { let _hubp2=hubp as *mut dcn20_hubp; REG_UPDATE!(DCHUBP_REQ_SIZE_CONFIG, SWATH_HEIGHT, 0); REG_UPDATE!(DCSURF_TILING_CONFIG, SW_MODE, DC_SW_LINEAR); REG_UPDATE!(DCSURF_SURFACE_CONTROL, PRIMARY_SURFACE_DCC_EN, 0, SECONDARY_SURFACE_DCC_EN, 0); }
pub unsafe fn hubp401_set_flip_int(hubp: *mut hubp) { let _hubp2=hubp as *mut dcn20_hubp; REG_UPDATE!(DCSURF_SURFACE_FLIP_INTERRUPT, SURFACE_FLIP_INT_MASK, 1); }
pub unsafe fn hubp401_in_blank(hubp: *mut hubp) -> bool { let _hubp2=hubp as *mut dcn20_hubp; let mut v=0; REG_GET!(DCHUBP_CNTL, HUBP_IN_BLANK, &mut v); v != 0 }

/* Remaining implementation is represented as external declarations until the
 * shared DCN type layer supplies the exact register and structure definitions. */
extern "C" {
    pub fn hubp401_program_surface_flip_and_addr(hubp: *mut hubp, address: *const dc_plane_address, flip_immediate: bool) -> bool;
    pub fn hubp401_setup(hubp: *mut hubp, pipe_regs: *mut c_void, pipe_global_sync: *mut c_void, timing: *mut c_void);
    pub fn hubp401_read_state(hubp: *mut hubp);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
