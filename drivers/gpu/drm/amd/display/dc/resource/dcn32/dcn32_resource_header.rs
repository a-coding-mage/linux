/*
 * Rust translation of dcn32_resource.h.
 * C preprocessor register-list macros are retained as declarative token
 * macros below; the register identifiers and helper macros are supplied by
 * the surrounding hardware-register definitions.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub const DCN3_2_DEFAULT_DET_SIZE: u32 = 256;
pub const DCN3_2_MAX_DET_SIZE: u32 = 1152;
pub const DCN3_2_MIN_DET_SIZE: u32 = 128;
pub const DCN3_2_MIN_COMPBUF_SIZE_KB: u32 = 128;
pub const DCN3_2_DET_SEG_SIZE: u32 = 64;
pub const DCN3_2_MALL_MBLK_SIZE_BYTES: u32 = 65536;
pub const DCN3_2_MBLK_WIDTH: u32 = 128;
pub const DCN3_2_MBLK_HEIGHT_4BPE: u32 = 128;
pub const DCN3_2_MBLK_HEIGHT_8BPE: u32 = 64;
pub const DCN3_2_DCFCLK_DS_INIT_KHZ: u32 = 10000;
pub const SUBVP_HIGH_REFRESH_LIST_LEN: usize = 4;
pub const SUBVP_ACTIVE_MARGIN_LIST_LEN: usize = 2;
pub const DCN3_2_MAX_SUBVP_PIXEL_RATE_MHZ: u32 = 1800;
pub const DCN3_2_VMIN_DISPCLK_HZ: u32 = 717000000;
pub const MIN_SUBVP_DCFCLK_KHZ: u32 = 400000;

extern "C" {
    pub static mut dcn3_2_ip: _vcs_dpi_ip_params_st;
    pub static mut dcn3_2_soc: _vcs_dpi_soc_bounding_box_st;

    pub fn dcn32_create_resource_pool(init_data: *const dc_init_data, dc: *mut dc) -> *mut resource_pool;
    pub fn dcn32_panel_cntl_create(init_data: *const panel_cntl_init_data) -> *mut panel_cntl;
    pub fn dcn32_acquire_post_bldn_3dlut(res_ctx: *mut resource_context, pool: *const resource_pool, mpcc_id: i32, lut: *mut *mut dc_3dlut, shaper: *mut *mut dc_transfer_func) -> bool;
    pub fn dcn32_release_post_bldn_3dlut(res_ctx: *mut resource_context, pool: *const resource_pool, lut: *mut *mut dc_3dlut, shaper: *mut *mut dc_transfer_func) -> bool;
}

#[repr(C)]
pub struct subvp_high_refresh_list {
    pub min_refresh: i32,
    pub max_refresh: i32,
    pub res: [resolution; SUBVP_HIGH_REFRESH_LIST_LEN],
}
#[repr(C)] pub struct resolution { pub width: i32, pub height: i32 }
#[repr(C)] pub struct subvp_active_margin_list {
    pub min_refresh: i32,
    pub max_refresh: i32,
    pub res: [resolution; SUBVP_ACTIVE_MARGIN_LIST_LEN],
}
#[repr(C)] pub struct dcn32_resource_pool { pub base: resource_pool }

/* All remaining declarations are C-compatible external interfaces. */
extern "C" {
    pub fn dcn32_add_phantom_pipes(dc: *mut dc, context: *mut dc_state, pipes: *mut display_e2e_pipe_params_st, pipe_cnt: u32, index: u32);
    pub fn dcn32_validate_bandwidth(dc: *mut dc, context: *mut dc_state, validate_mode: dc_validate_mode) -> dc_status;
    pub fn dcn32_populate_dml_pipes_from_context(dc: *mut dc, context: *mut dc_state, pipes: *mut display_e2e_pipe_params_st, validate_mode: dc_validate_mode) -> i32;
    pub fn dcn32_calculate_wm_and_dlg(dc: *mut dc, context: *mut dc_state, pipes: *mut display_e2e_pipe_params_st, pipe_cnt: i32, vlevel: i32);
    pub fn dcn32_helper_calculate_mall_bytes_for_cursor(dc: *mut dc, pipe_ctx: *mut pipe_ctx, ignore_cursor_buf: bool) -> u32;
    pub fn dcn32_helper_calculate_num_ways_for_subvp(dc: *mut dc, context: *mut dc_state) -> u32;
    pub fn dcn32_merge_pipes_for_subvp(dc: *mut dc, context: *mut dc_state);
    pub fn dcn32_all_pipes_have_stream_and_plane(dc: *mut dc, context: *mut dc_state) -> bool;
    pub fn dcn32_subvp_in_use(dc: *mut dc, context: *mut dc_state) -> bool;
    pub fn dcn32_mpo_in_use(context: *mut dc_state) -> bool;
    pub fn dcn32_any_surfaces_rotated(dc: *mut dc, context: *mut dc_state) -> bool;
    pub fn dcn32_is_center_timing(pipe: *mut pipe_ctx) -> bool;
    pub fn dcn32_is_psr_capable(pipe: *mut pipe_ctx) -> bool;
    pub fn dcn32_allow_subvp_with_active_margin(pipe: *mut pipe_ctx) -> bool;
    pub fn dcn32_allow_subvp_high_refresh_rate(dc: *mut dc, context: *mut dc_state, pipe: *mut pipe_ctx) -> bool;
    pub fn dcn32_calc_num_avail_chans_for_mall(dc: *mut dc, num_chans: i32) -> u32;
    pub fn dcn32_determine_max_vratio_prefetch(dc: *mut dc, context: *mut dc_state) -> f64;
    pub fn dcn32_check_native_scaling_for_res(pipe: *mut pipe_ctx, width: u32, height: u32) -> bool;
    pub fn dcn32_subvp_drr_admissable(dc: *mut dc, context: *mut dc_state) -> bool;
    pub fn dcn32_subvp_vblank_admissable(dc: *mut dc, context: *mut dc_state, vlevel: i32) -> bool;
    pub fn dcn32_override_min_req_dcfclk(dc: *mut dc, context: *mut dc_state);
    pub fn dcn32_calculate_mall_ways_from_bytes(dc: *const dc, total_size_in_mall_bytes: u32) -> u32;
    pub fn dcn32_get_max_hw_cursor_size(dc: *const dc, state: *mut dc_state, stream: *const dc_stream_state) -> u32;
}

/* Register-list macros are intentionally represented as token-forwarding
 * declarative macros; dependent register helper macros define their expansion. */
macro_rules! TO_DCN32_RES_POOL { ($pool:expr) => { $pool }; }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
