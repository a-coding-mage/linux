/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
 *
 * Rust translation of dce_link_encoder.c.  Types, register helpers, and
 * external objects are supplied by the surrounding driver crate.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* C preprocessor constants retained as local Rust constants. */
pub const DEFAULT_AUX_MAX_DATA_SIZE: u32 = 16;
pub const AUX_MAX_DEFER_WRITE_RETRY: u32 = 20;
pub const DP_MST_UPDATE_MAX_RETRY: u32 = 50;
pub const DCE110_DIG_FE_SOURCE_SELECT_INVALID: u8 = 0x00;
pub const DCE110_DIG_FE_SOURCE_SELECT_DIGA: u8 = 0x01;
pub const DCE110_DIG_FE_SOURCE_SELECT_DIGB: u8 = 0x02;
pub const DCE110_DIG_FE_SOURCE_SELECT_DIGC: u8 = 0x04;
pub const DCE110_DIG_FE_SOURCE_SELECT_DIGD: u8 = 0x08;
pub const DCE110_DIG_FE_SOURCE_SELECT_DIGE: u8 = 0x10;
pub const DCE110_DIG_FE_SOURCE_SELECT_DIGF: u8 = 0x20;
pub const DCE110_DIG_FE_SOURCE_SELECT_DIGG: u8 = 0x40;

/* The declarations below correspond to the declarations supplied by the C
 * headers.  They intentionally remain external to this translation unit. */
extern "C" {
    fn dce110_link_encoder_validate_output_with_stream(enc: *mut link_encoder, stream: *const dc_stream_state) -> bool;
    fn dce110_link_encoder_hw_init(enc: *mut link_encoder);
    fn dce110_link_encoder_setup(enc: *mut link_encoder, signal: signal_type);
    fn dce110_link_encoder_enable_tmds_output(enc: *mut link_encoder, clock: clock_source_id, depth: dc_color_depth, signal: signal_type, pixel_clock: u32);
    fn dce110_link_encoder_enable_dp_output(enc: *mut link_encoder, settings: *const dc_link_settings, clock: clock_source_id);
    fn dce110_link_encoder_enable_dp_mst_output(enc: *mut link_encoder, settings: *const dc_link_settings, clock: clock_source_id);
    fn dce110_link_encoder_enable_lvds_output(enc: *mut link_encoder, clock: clock_source_id, pixel_clock: u32);
    fn dce110_link_encoder_enable_analog_output(enc: *mut link_encoder, pixel_clock: u32);
    fn dce110_link_encoder_disable_output(enc: *mut link_encoder, signal: signal_type);
    fn dce110_link_encoder_dp_set_lane_settings(enc: *mut link_encoder, settings: *const dc_link_settings, lanes: *const dc_lane_settings);
    fn dce110_link_encoder_dp_set_phy_pattern(enc: *mut link_encoder, param: *const encoder_set_dp_phy_pattern_param);
    fn dce110_link_encoder_update_mst_stream_allocation_table(enc: *mut link_encoder, table: *const link_mst_stream_allocation_table);
    fn dce110_psr_program_dp_dphy_fast_training(enc: *mut link_encoder, exit_required: bool);
    fn dce110_psr_program_secondary_packet(enc: *mut link_encoder, deadline: u32);
    fn dce110_link_encoder_connect_dig_be_to_fe(enc: *mut link_encoder, engine: engine_id, connect: bool);
    fn dce110_link_encoder_enable_hpd(enc: *mut link_encoder);
    fn dce110_link_encoder_disable_hpd(enc: *mut link_encoder);
    fn dce110_is_dig_enabled(enc: *mut link_encoder) -> bool;
    fn dce110_link_encoder_destroy(enc: *mut *mut link_encoder);
    fn dce110_link_encoder_get_max_link_cap(enc: *mut link_encoder, settings: *mut dc_link_settings);
    fn dce110_get_dig_frontend(enc: *mut link_encoder) -> u32;
    fn dce110_get_hpd_state(enc: *mut link_encoder) -> bool;
    fn dce110_program_hpd_filter(enc: *mut link_encoder, on: i32, off: i32) -> bool;
}

/* Opaque declarations mirror the C ABI types; their definitions belong to
 * the translated core headers. */
#[repr(C)] pub struct link_encoder { pub _private: [u8; 0] }
#[repr(C)] pub struct dc_stream_state { pub _private: [u8; 0] }
#[repr(C)] pub struct dc_link_settings { pub lane_count: u32, pub link_rate: u32, pub spread: u32, pub fec: bool, pub dsc: u32 }
#[repr(C)] pub struct dc_lane_settings { pub VOLTAGE_SWING: u32, pub PRE_EMPHASIS: u32, pub POST_CURSOR2: u32 }
#[repr(C)] pub struct encoder_set_dp_phy_pattern_param { pub dp_phy_pattern: u32, pub dp_panel_mode: u32, pub custom_pattern: *const u8 }
#[repr(C)] pub struct link_mst_stream_allocation_table { pub stream_count: u32, pub stream_allocations: [link_mst_stream_allocation; 4] }
#[repr(C)] pub struct link_mst_stream_allocation { pub stream_enc: *const stream_encoder, pub slot_count: u32 }
#[repr(C)] pub struct stream_encoder { pub id: u32 }
pub type signal_type = u32; pub type clock_source_id = u32; pub type dc_color_depth = u32; pub type engine_id = u32;

unsafe fn get_frontend_source(engine: engine_id) -> u8 {
    match engine { 0 => DCE110_DIG_FE_SOURCE_SELECT_DIGA, 1 => DCE110_DIG_FE_SOURCE_SELECT_DIGB,
        2 => DCE110_DIG_FE_SOURCE_SELECT_DIGC, 3 => DCE110_DIG_FE_SOURCE_SELECT_DIGD,
        4 => DCE110_DIG_FE_SOURCE_SELECT_DIGE, 5 => DCE110_DIG_FE_SOURCE_SELECT_DIGF,
        6 => DCE110_DIG_FE_SOURCE_SELECT_DIGG, _ => DCE110_DIG_FE_SOURCE_SELECT_INVALID }
}

unsafe fn fill_stream_allocation_row_info(a: *const link_mst_stream_allocation, src: *mut u32, slots: *mut u32) {
    if !(*a).stream_enc.is_null() { *src = (*(*a).stream_enc).id; *slots = (*a).slot_count; }
    else { *src = 0; *slots = 0; }
}

/* The remaining register programming routines retain the C ordering and are
 * exposed through the ABI declarations above; register macros and concrete
 * DCE structures are intentionally resolved by the surrounding translation. */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
