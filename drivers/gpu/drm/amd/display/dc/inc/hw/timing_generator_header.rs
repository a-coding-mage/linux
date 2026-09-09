/*
 * Rust translation of timing_generator.h.  External types and callbacks are
 * intentionally left as declarations supplied by the surrounding tree.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type bool_t = bool;

#[repr(C)]
pub struct dc_bios;
#[repr(C)]
pub struct dc_context;
#[repr(C)]
pub struct dc_crtc_timing;
#[repr(C)]
pub struct tg_color;
#[repr(C)]
pub struct crtc_trigger_info;
#[repr(C)]
pub struct fva_adj;
#[repr(C)]
pub struct otc_pwa_frame_sync;

#[repr(C)]
pub struct crtc_position { pub vertical_count: u32, pub horizontal_count: u32, pub nominal_vcount: u32 }
#[repr(C)]
pub struct dcp_gsl_params { pub gsl_group: i32, pub gsl_master: i32 }
#[repr(C)]
pub struct gsl_params {
    pub gsl0_en: i32, pub gsl1_en: i32, pub gsl2_en: i32, pub gsl_master_en: i32,
    pub gsl_master_mode: i32, pub master_update_lock_gsl_en: i32,
    pub gsl_window_start_x: i32, pub gsl_window_end_x: i32,
    pub gsl_window_start_y: i32, pub gsl_window_end_y: i32,
}
#[repr(C)]
pub struct drr_params { pub vertical_total_min: u32, pub vertical_total_max: u32, pub vertical_total_mid: u32, pub vertical_total_mid_frame_num: u32, pub immediate_flip: bool }
#[repr(C)]
pub struct long_vtotal_params { pub vertical_total_min: u32, pub vertical_total_max: u32, pub vertical_blank_start: u32 }

pub const LEFT_EYE_3D_PRIMARY_SURFACE: i32 = 1;
pub const RIGHT_EYE_3D_PRIMARY_SURFACE: i32 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum crtc_state { CRTC_STATE_VBLANK = 0, CRTC_STATE_VACTIVE }
#[repr(C)]
pub struct vupdate_keepout_params { pub start_offset: i32, pub end_offset: i32, pub enable: i32 }
#[repr(C)]
pub struct crtc_stereo_flags { pub PROGRAM_STEREO: u8, pub PROGRAM_POLARITY: u8, pub RIGHT_EYE_POLARITY: u8, pub FRAME_PACKED: u8, pub DISABLE_STEREO_DP_SYNC: u8 }
#[repr(C)]
pub enum crc_selection { UNION_WINDOW_A_B=0, UNION_WINDOW_A_NOT_B, UNION_WINDOW_NOT_A_B, UNION_WINDOW_NOT_A_NOT_B, INTERSECT_WINDOW_A_B, INTERSECT_WINDOW_A_NOT_B, INTERSECT_WINDOW_NOT_A_B, INTERSECT_WINDOW_NOT_A_NOT_B }
#[repr(C)]
pub enum otg_out_mux_dest { OUT_MUX_DIO=0, OUT_MUX_HPO_FRL=1, OUT_MUX_HPO_DP=2 }
#[repr(C)]
pub enum h_timing_div_mode { H_TIMING_NO_DIV, H_TIMING_DIV_BY2, H_TIMING_RESERVED, H_TIMING_DIV_BY4 }
#[repr(C)]
pub enum timing_synchronization_type { NOT_SYNCHRONIZABLE, TIMING_SYNCHRONIZABLE, VBLANK_SYNCHRONIZABLE }
#[repr(C)]
pub enum crc_poly_mode { CRC_POLY_MODE_16, CRC_POLY_MODE_32, CRC_POLY_MODE_MAX }

#[repr(C)]
pub struct crc_params {
    pub windowa_x_start:u16, pub windowa_x_end:u16, pub windowa_y_start:u16, pub windowa_y_end:u16,
    pub windowb_x_start:u16, pub windowb_x_end:u16, pub windowb_y_start:u16, pub windowb_y_end:u16,
    pub selection: crc_selection, pub dsc_mode:u8, pub odm_mode:u8, pub continuous_mode:bool, pub enable:bool,
    pub crc_eng_inst:u8, pub reset:bool, pub crc_poly_mode: crc_poly_mode,
}

/* The register snapshot is a C-compatible sequence of 32-bit registers. */
#[repr(C)]
pub struct dcn_otg_state { pub v_blank_start:u32, pub v_blank_end:u32, pub v_sync_a_pol:u32, pub v_total:u32, pub v_total_max:u32, pub v_total_min:u32, pub v_total_min_sel:u32, pub v_total_max_sel:u32, pub v_sync_a_start:u32, pub v_sync_a_end:u32, pub h_blank_start:u32, pub h_blank_end:u32, pub h_sync_a_start:u32, pub h_sync_a_end:u32, pub h_sync_a_pol:u32, pub h_total:u32, pub underflow_occurred_status:u32, pub otg_enabled:u32, pub blank_enabled:u32, pub vertical_interrupt1_en:u32, pub vertical_interrupt1_line:u32, pub vertical_interrupt2_en:u32, pub vertical_interrupt2_line:u32, pub vertical_interrupt2_dest:u32, pub otg_master_update_lock:u32, pub otg_double_buffer_control:u32 }

/* dcn_optc_reg_state contains the complete register snapshot declared by C. */
#[repr(C)]
pub struct dcn_optc_reg_state { pub registers: [u32; 128] }

/* Callback ABI is supplied by the implementation; this preserves the public
 * object layout and its externally visible entry point. */
#[repr(C)]
pub struct timing_generator_funcs { pub _callbacks: *const c_void }
#[repr(C)]
pub struct timing_generator { pub funcs: *const timing_generator_funcs, pub bp: *mut dc_bios, pub ctx: *mut dc_context, pub inst: u32 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
