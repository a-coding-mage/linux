/* Rust translation of mpc.h. External types are supplied by dependent headers. */

pub const MAX_MPCC: usize = 6;
pub const MAX_OPP: usize = 6;
pub const MAX_DWB: usize = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mpc_output_csc_mode { MPC_OUTPUT_CSC_DISABLE = 0, MPC_OUTPUT_CSC_COEF_A, MPC_OUTPUT_CSC_COEF_B }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum mpcc_blend_mode { MPCC_BLEND_MODE_BYPASS, MPCC_BLEND_MODE_TOP_LAYER_PASSTHROUGH, MPCC_BLEND_MODE_TOP_LAYER_ONLY, MPCC_BLEND_MODE_TOP_BOT_BLENDING }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum mpcc_alpha_blend_mode { MPCC_ALPHA_BLEND_MODE_PER_PIXEL_ALPHA, MPCC_ALPHA_BLEND_MODE_PER_PIXEL_ALPHA_COMBINED_GLOBAL_GAIN, MPCC_ALPHA_BLEND_MODE_GLOBAL_ALPHA }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum mpcc_movable_cm_location { MPCC_MOVABLE_CM_LOCATION_BEFORE, MPCC_MOVABLE_CM_LOCATION_AFTER }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum MCM_LUT_ID { MCM_LUT_3DLUT, MCM_LUT_1DLUT, MCM_LUT_SHAPER }

#[repr(C)] pub struct mpc_fl_3dlut_config { pub enabled: bool, pub size: dc_cm_lut_size, pub select_lut_bank_a: bool, pub bit_depth: u16, pub hubp_index: i32, pub bias: u16, pub scale: u16 }
#[repr(C)] pub union mcm_lut_params { pub pwl: *const pwl_params, pub lut3d: *const tetrahedral_params }
#[repr(C)] pub struct mpcc_blnd_cfg { pub black_color: tg_color, pub alpha_mode: mpcc_alpha_blend_mode, pub pre_multiplied_alpha: bool, pub global_gain: i32, pub global_alpha: i32, pub overlap_only: bool, pub bottom_gain_mode: i32, pub background_color_bpc: i32, pub top_gain: i32, pub bottom_inside_gain: i32, pub bottom_outside_gain: i32 }
#[repr(C)] pub struct mpc_grph_gamut_adjustment { pub temperature_matrix: [fixed31_32; CSC_TEMPERATURE_MATRIX_SIZE], pub gamut_adjust_type: graphics_gamut_adjust_type, pub mpcc_gamut_remap_block_id: mpcc_gamut_remap_id }
#[repr(C)] pub struct mpc_rmcm_regs { pub rmcm_3dlut_mem_pwr_state: u32, pub rmcm_3dlut_mem_pwr_force: u32, pub rmcm_3dlut_mem_pwr_dis: u32, pub rmcm_3dlut_mem_pwr_mode: u32, pub rmcm_3dlut_size: u32, pub rmcm_3dlut_mode: u32, pub rmcm_3dlut_mode_cur: u32, pub rmcm_3dlut_read_sel: u32, pub rmcm_3dlut_30bit_en: u32, pub rmcm_3dlut_wr_en_mask: u32, pub rmcm_3dlut_ram_sel: u32, pub rmcm_3dlut_out_norm_factor: u32, pub rmcm_3dlut_fl_sel: u32, pub rmcm_3dlut_out_offset_r: u32, pub rmcm_3dlut_out_scale_r: u32, pub rmcm_3dlut_fl_done: u32, pub rmcm_3dlut_fl_soft_underflow: u32, pub rmcm_3dlut_fl_hard_underflow: u32, pub rmcm_cntl: u32, pub rmcm_shaper_mem_pwr_state: u32, pub rmcm_shaper_mem_pwr_force: u32, pub rmcm_shaper_mem_pwr_dis: u32, pub rmcm_shaper_mem_pwr_mode: u32, pub rmcm_shaper_lut_mode: u32, pub rmcm_shaper_mode_cur: u32, pub rmcm_shaper_lut_write_en_mask: u32, pub rmcm_shaper_lut_write_sel: u32, pub rmcm_shaper_offset_b: u32, pub rmcm_shaper_scale_b: u32, pub rmcm_shaper_rama_exp_region_start_b: u32, pub rmcm_shaper_rama_exp_region_start_seg_b: u32, pub rmcm_shaper_rama_exp_region_end_b: u32, pub rmcm_shaper_rama_exp_region_end_base_b: u32 }
#[repr(C)] pub struct mpc_sm_cfg { pub enable: bool, pub sm_mode: i32, pub frame_alt: bool, pub field_alt: bool, pub force_next_frame_porlarity: i32, pub force_next_field_polarity: i32 }
#[repr(C)] pub struct mpc_denorm_clamp { pub clamp_max_r_cr: i32, pub clamp_min_r_cr: i32, pub clamp_max_g_y: i32, pub clamp_min_g_y: i32, pub clamp_max_b_cb: i32, pub clamp_min_b_cb: i32 }
#[repr(C)] pub struct mpc_dwb_flow_control { pub flow_ctrl_mode: i32, pub flow_ctrl_cnt0: i32, pub flow_ctrl_cnt1: i32 }
#[repr(C)] pub struct mpcc { pub mpcc_id: i32, pub dpp_id: i32, pub mpcc_bot: *mut mpcc, pub blnd_cfg: mpcc_blnd_cfg, pub sm_cfg: mpc_sm_cfg, pub shared_bottom: bool }
#[repr(C)] pub struct mpc_tree { pub opp_id: i32, pub opp_list: *mut mpcc }
#[repr(C)] pub struct mpc { pub funcs: *const mpc_funcs, pub ctx: *mut dc_context, pub mpcc_array: [mpcc; MAX_MPCC], pub blender_params: pwl_params, pub cm_bypass_mode: bool }
#[repr(C)] pub struct mpcc_state { pub opp_id:u32,pub dpp_id:u32,pub bot_mpcc_id:u32,pub mode:u32,pub alpha_mode:u32,pub pre_multiplied_alpha:u32,pub overlap_only:u32,pub idle:u32,pub busy:u32,pub shaper_lut_mode:u32,pub lut3d_mode:u32,pub lut3d_bit_depth:u32,pub lut3d_size:u32,pub rgam_mode:u32,pub rgam_lut:u32,pub gamut_remap:mpc_grph_gamut_adjustment,pub rmcm_regs:mpc_rmcm_regs }
#[repr(C)] pub struct dcn_mpc_reg_state { pub mpcc_bot_sel:u32,pub mpcc_control:u32,pub mpcc_status:u32,pub mpcc_top_sel:u32,pub mpcc_opp_id:u32,pub mpcc_ogam_control:u32 }

/* Function-pointer declarations and nested RMCM interface. */
#[repr(C)] pub struct mpc_funcs {
    pub read_mpcc_state: Option<unsafe extern "C" fn(*mut mpc,i32,*mut mpcc_state)>,
    pub mpc_read_reg_state: Option<unsafe extern "C" fn(*mut mpc,i32,*mut dcn_mpc_reg_state)>,
    pub insert_plane: Option<unsafe extern "C" fn(*mut mpc,*mut mpc_tree,*mut mpcc_blnd_cfg,*mut mpc_sm_cfg,*mut mpcc,i32,i32)->*mut mpcc>,
    pub remove_mpcc: Option<unsafe extern "C" fn(*mut mpc,*mut mpc_tree,*mut mpcc)>,
    pub mpc_init: Option<unsafe extern "C" fn(*mut mpc)>,
    pub mpc_init_single_inst: Option<unsafe extern "C" fn(*mut mpc,u32)>,
    pub update_blending: Option<unsafe extern "C" fn(*mut mpc,*mut mpcc_blnd_cfg,i32)>,
    pub cursor_lock: Option<unsafe extern "C" fn(*mut mpc,i32,bool)>,
    pub insert_plane_to_secondary: Option<unsafe extern "C" fn(*mut mpc,*mut mpc_tree,*mut mpcc_blnd_cfg,*mut mpc_sm_cfg,*mut mpcc,i32,i32)->*mut mpcc>,
    pub remove_mpcc_from_secondary: Option<unsafe extern "C" fn(*mut mpc,*mut mpc_tree,*mut mpcc)>,
    pub get_mpcc_for_dpp_from_secondary: Option<unsafe extern "C" fn(*mut mpc_tree,i32)->*mut mpcc>,
    pub get_mpcc_for_dpp: Option<unsafe extern "C" fn(*mut mpc_tree,i32)->*mut mpcc>,
    pub wait_for_idle: Option<unsafe extern "C" fn(*mut mpc,i32)>,
    pub assert_mpcc_idle_before_connect: Option<unsafe extern "C" fn(*mut mpc,i32)>,
    pub init_mpcc_list_from_hw: Option<unsafe extern "C" fn(*mut mpc,*mut mpc_tree)>,
    pub set_denorm: Option<unsafe extern "C" fn(*mut mpc,i32,dc_color_depth)>,
    pub set_denorm_clamp: Option<unsafe extern "C" fn(*mut mpc,i32,mpc_denorm_clamp)>,
    pub set_output_csc: Option<unsafe extern "C" fn(*mut mpc,i32,*const u16,mpc_output_csc_mode)>,
    pub set_ocsc_default: Option<unsafe extern "C" fn(*mut mpc,i32,dc_color_space,mpc_output_csc_mode)>,
    pub set_output_gamma: Option<unsafe extern "C" fn(*mut mpc,i32,*const pwl_params)>,
    pub power_on_mpc_mem_pwr: Option<unsafe extern "C" fn(*mut mpc,i32,bool)>,
    pub set_dwb_mux: Option<unsafe extern "C" fn(*mut mpc,i32,i32)>, pub disable_dwb_mux: Option<unsafe extern "C" fn(*mut mpc,i32)>,
    pub is_dwb_idle: Option<unsafe extern "C" fn(*mut mpc,i32)->bool>,
    pub set_out_rate_control: Option<unsafe extern "C" fn(*mut mpc,i32,bool,bool,*mut mpc_dwb_flow_control)>,
    pub set_gamut_remap: Option<unsafe extern "C" fn(*mut mpc,i32,*const mpc_grph_gamut_adjustment)>,
    pub program_1dlut: Option<unsafe extern "C" fn(*mut mpc,*const pwl_params,u32)->bool>, pub program_shaper: Option<unsafe extern "C" fn(*mut mpc,*const pwl_params,u32)->bool>,
    pub acquire_rmu: Option<unsafe extern "C" fn(*mut mpc,i32,i32)->u32>, pub program_3dlut: Option<unsafe extern "C" fn(*mut mpc,*const tetrahedral_params,i32)->bool>, pub release_rmu: Option<unsafe extern "C" fn(*mut mpc,i32)->i32>,
    pub get_mpc_out_mux: Option<unsafe extern "C" fn(*mut mpc,i32)->u32>, pub set_bg_color: Option<unsafe extern "C" fn(*mut mpc,*mut tg_color,i32)>, pub set_mpc_mem_lp_mode: Option<unsafe extern "C" fn(*mut mpc)>,
    pub set_movable_cm_location: Option<unsafe extern "C" fn(*mut mpc,mpcc_movable_cm_location,i32)>, pub update_3dlut_fast_load_select: Option<unsafe extern "C" fn(*mut mpc,i32,i32)>,
    pub get_3dlut_fast_load_status: Option<unsafe extern "C" fn(*mut mpc,i32,*mut u32,*mut u32,*mut u32)>,
    pub populate_lut: Option<unsafe extern "C" fn(*mut mpc,MCM_LUT_ID,*const mcm_lut_params,bool,i32)>, pub program_lut_read_write_control: Option<unsafe extern "C" fn(*mut mpc,MCM_LUT_ID,bool,u32,i32)>,
    pub program_lut_mode: Option<unsafe extern "C" fn(*mut mpc,MCM_LUT_ID,bool,bool,dc_cm_lut_size,i32)>, pub get_lut_mode: Option<unsafe extern "C" fn(*mut mpc,MCM_LUT_ID,i32,*mut bool,*mut bool)>,
    pub rmcm: mpc_rmcm_funcs,
}

#[repr(C)] pub struct mpc_rmcm_funcs {
    pub fl_3dlut_configure: Option<unsafe extern "C" fn(*mut mpc,*mut mpc_fl_3dlut_config,i32)>, pub enable_3dlut_fl: Option<unsafe extern "C" fn(*mut mpc,bool,i32)>,
    pub update_3dlut_fast_load_select: Option<unsafe extern "C" fn(*mut mpc,i32,i32)>, pub program_lut_read_write_control: Option<unsafe extern "C" fn(*mut mpc,MCM_LUT_ID,bool,bool,i32)>,
    pub program_lut_mode: Option<unsafe extern "C" fn(*mut mpc,bool,bool,i32)>, pub program_3dlut_size: Option<unsafe extern "C" fn(*mut mpc,dc_cm_lut_size,i32)>,
    pub program_bias_scale: Option<unsafe extern "C" fn(*mut mpc,u16,u16,i32)>, pub program_bit_depth: Option<unsafe extern "C" fn(*mut mpc,u16,i32)>, pub is_config_supported: Option<unsafe extern "C" fn(u32)->bool>,
    pub power_on_shaper_3dlut: Option<unsafe extern "C" fn(*mut mpc,u32,bool)>, pub populate_lut: Option<unsafe extern "C" fn(*mut mpc,mcm_lut_params,bool,i32)>, pub get_3dlut_mode: Option<unsafe extern "C" fn(*mut mpc,i32,*mut bool,*mut bool)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
