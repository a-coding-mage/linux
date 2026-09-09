/*
 * Rust source-level translation of dc.h.
 *
 * The original header's external dependencies are intentionally left as
 * external Rust types and declarations, to be supplied by the surrounding
 * translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub const DC_VER: &str = "3.2.392";
pub const MAX_SURFACES: usize = 4;
pub const MAX_PLANES: usize = 6;
pub const MAX_STREAMS: usize = 6;
pub const MIN_VIEWPORT_SIZE: usize = 12;
pub const MAX_NUM_EDP: usize = 2;
pub const MAX_SUPPORTED_FORMATS: usize = 7;
pub const MAX_HOST_ROUTERS_NUM: usize = 3;
pub const MAX_DPIA_PER_HOST_ROUTER: usize = 3;
pub const MAX_DPIA_NUM: usize = MAX_HOST_ROUTERS_NUM * MAX_DPIA_PER_HOST_ROUTER;
pub const NUM_FAST_FLIPS_TO_STEADY_STATE: usize = 20;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frl_cap_chk_intermediates_fixed31_32 {
    pub c_frl_sb: i32,
    pub overhead_sb: fixed31_32,
    pub overhead_rs: fixed31_32,
    pub overhead_map: fixed31_32,
    pub overhead_min: fixed31_32,
    pub overhead_max: fixed31_32,
    pub f_pixel_clock_max: fixed31_32,
    pub t_line: fixed31_32,
    pub r_bit_min: fixed31_32,
    pub r_frl_char_min: fixed31_32,
    pub c_frl_line: fixed31_32,
    pub ap: fixed31_32,
    pub r_ap: fixed31_32,
    pub avg_audio_packets_line: fixed31_32,
    pub margin: fixed31_32,
    pub audio_packets_line: i32,
    pub blank_audio_min: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frl_cap_chk_params_fixed31_32 {
    pub lanes: i32,
    pub f_pixel_clock_nominal: fixed31_32,
    pub r_bit_nominal: fixed31_32,
    pub audio_packet_type: i32,
    pub f_audio: fixed31_32,
    pub h_active: i32,
    pub h_blank: i32,
    pub bpc: i32,
    pub vic: i32,
    pub pixel_encoding: hdmi_frl_pixel_encoding,
    pub compressed: bool,
    pub bypass_hc_target_calc: bool,
    pub allow_all_bpp: bool,
    pub slices: i32,
    pub slice_width: i32,
    pub bpp_target: fixed31_32,
    pub layout: i32,
    pub acat: i32,
    pub borrow_params: frl_dml_borrow_params,
    pub average_tribyte_rate: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_versions {
    pub dc_ver: *const core::ffi::c_char,
    pub dmcu_version: dmcu_version,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_plane_cap {
    pub type_: dc_plane_type,
    pub per_pixel_alpha: u32,
    pub pixel_format_support: dc_plane_cap_pixel_format_support,
    pub max_upscale_factor: dc_plane_cap_scale,
    pub max_downscale_factor: dc_plane_cap_scale,
    pub min_width: u32,
    pub min_height: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_plane_cap_pixel_format_support {
    pub argb8888: u32, pub nv12: u32, pub fp16: u32, pub p010: u32,
    pub ayuv: u32, pub yuy2: u32, pub y210: u32, pub y212: u32,
    pub p208: u32, pub p210: u32, pub p212: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_plane_cap_scale { pub argb8888: u32, pub nv12: u32, pub fp16: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rom_curve_caps { pub srgb: u16, pub bt2020: u16, pub gamma2_2: u16, pub pq: u16, pub hlg: u16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_color_caps { pub dpp: dpp_color_caps, pub mpc: mpc_color_caps }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_dmub_caps { pub psr: bool, pub mclk_sw: bool, pub subvp_psr: bool, pub gecc_enable: bool, pub fams_ver: u8, pub aux_backlight_support: bool }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_scl_caps { pub sharpener_support: bool }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_check_config { pub max_optimizable_video_width: u32, pub enable_legacy_fast_update: bool, pub deferred_transition_state: bool, pub transition_countdown_to_steady_state: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_caps {
    pub max_streams: u32, pub max_links: u32, pub max_audios: u32, pub max_slave_planes: u32,
    pub max_slave_yuv_planes: u32, pub max_slave_rgb_planes: u32, pub max_planes: u32,
    pub max_downscale_ratio: u32, pub i2c_speed_in_khz: u32, pub i2c_speed_in_khz_hdcp: u32,
    pub dmdata_alloc_size: u32, pub max_cursor_size: u32, pub max_buffered_cursor_size: u32,
    pub max_video_width: u32, pub min_horizontal_blanking_period: u32, pub linear_pitch_alignment: i32,
    pub dcc_const_color: bool, pub dynamic_audio: bool, pub is_apu: bool, pub dual_link_dvi: bool,
    pub post_blend_color_processing: bool, pub force_dp_tps4_for_cp2520: bool,
    pub disable_dp_clk_share: bool, pub psp_setup_panel_mode: bool, pub extended_aux_timeout_support: bool,
    pub dmcub_support: bool, pub zstate_support: bool, pub ips_support: bool, pub ips_v2_support: bool,
    pub num_of_internal_disp: u32, pub max_dp_protocol_version: dp_protocol_version,
    pub hdmi_hpo: bool, pub mall_size_per_mem_channel: u32, pub mall_size_total: u32,
    pub cursor_cache_size: u32, pub planes: [dc_plane_cap; MAX_PLANES], pub color: dc_color_caps,
    pub dmub_caps: dc_dmub_caps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpp_color_caps { pub dcn_arch: u16, pub input_lut_shared: u16, pub icsc: u16, pub dgam_ram: u16, pub post_csc: u16, pub gamma_corr: u16, pub hw_3d_lut: u16, pub ogam_ram: u16, pub ocsc: u16, pub dgam_rom_for_yuv: u16, pub upsp_pre_scaler: u16, pub dgam_rom_caps: rom_curve_caps, pub ogam_rom_caps: rom_curve_caps }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_color_caps { pub gamut_remap: u16, pub ogam_ram: u16, pub ocsc: u16, pub num_3dluts: u16, pub num_rmcm_3dluts: u16, pub shared_3d_lut: u16, pub ogam_rom_caps: rom_curve_caps, pub preblend: bool, pub max_gamut_remap_coeff: fixed31_32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_update_descriptor { pub update_type: dc_update_type, pub lock_descriptor: dc_lock_descriptor }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fixed31_32 { pub value: i64 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmcu_version { pub major: u32, pub minor: u32, pub revision: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frl_dml_borrow_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_frl_pixel_encoding { pub _opaque: i32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpp_color_caps_placeholder { pub _opaque: [u8; 0] }

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_protocol_version { DP_VERSION_1_4 = 0, DP_VERSION_2_1, DP_VERSION_UNKNOWN }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_plane_type { DC_PLANE_TYPE_INVALID = 0, DC_PLANE_TYPE_DCE_RGB, DC_PLANE_TYPE_DCE_UNDERLAY, DC_PLANE_TYPE_DCN_UNIVERSAL }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_update_type { UPDATE_TYPE_FAST = 0, UPDATE_TYPE_MED, UPDATE_TYPE_FULL }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_lock_descriptor { LOCK_DESCRIPTOR_NONE = 0, LOCK_DESCRIPTOR_STREAM = 1, LOCK_DESCRIPTOR_LINK = 2, LOCK_DESCRIPTOR_GLOBAL = 4, LOCK_DESCRIPTOR_PROBE = 8 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
