/* Direct low-level Rust translation of core_types.h. Included dependency types
 * are intentionally left to the surrounding translation unit. */

use core::ffi::{c_char, c_void};

pub const MAX_CLOCK_SOURCES: usize = 7;
pub const MAX_SVP_PHANTOM_STREAMS: usize = 2;
pub const MAX_SVP_PHANTOM_PLANES: usize = 2;
pub const MAX_RMCM_INST: usize = 2;
pub const NO_UNDERLAY_PIPE: i32 = -1;
pub const LINK_RES_HPO_DP_REC_MAP__MASK: u32 = 0xffff;
pub const LINK_RES_HPO_DP_REC_MAP__SHIFT: u32 = 0;

#[repr(C)]
pub struct resource_funcs {
    pub get_preferred_eng_id_dpia: *mut c_void, pub destroy: *mut c_void,
    pub link_init: *mut c_void, pub panel_cntl_create: *mut c_void,
    pub link_enc_create: *mut c_void, pub link_enc_create_minimal: *mut c_void,
    pub hpo_frl_link_enc_create: *mut c_void, pub validate_bandwidth: *mut c_void,
    pub calculate_wm_and_dlg: *mut c_void, pub update_soc_for_wm_a: *mut c_void,
    pub calculate_mall_ways_from_bytes: *mut c_void, pub prepare_mcache_programming: *mut c_void,
    pub populate_dml_pipes: *mut c_void, pub link_encs_assign: *mut c_void,
    pub link_enc_unassign: *mut c_void, pub validate_global: *mut c_void,
    pub acquire_free_pipe_as_secondary_dpp_pipe: *mut c_void,
    pub acquire_free_pipe_as_secondary_opp_head: *mut c_void, pub release_pipe: *mut c_void,
    pub validate_plane: *mut c_void, pub add_stream_to_ctx: *mut c_void,
    pub remove_stream_from_ctx: *mut c_void, pub patch_unknown_plane_state: *mut c_void,
    pub find_first_free_match_stream_enc_for_link: *mut c_void,
    pub populate_dml_writeback_from_context: *mut c_void, pub set_mcif_arb_params: *mut c_void,
    pub update_bw_bounding_box: *mut c_void, pub acquire_post_bldn_3dlut: *mut c_void,
    pub release_post_bldn_3dlut: *mut c_void, pub add_dsc_to_stream_resource: *mut c_void,
    pub add_phantom_pipes: *mut c_void, pub get_panel_config_defaults: *mut c_void,
    pub get_default_tiling_info: *mut c_void, pub build_pipe_pix_clk_params: *mut c_void,
    pub get_power_profile: *mut c_void, pub get_det_buffer_size: *mut c_void,
    pub get_vstartup_for_pipe: *mut c_void, pub get_max_hw_cursor_size: *mut c_void,
    pub program_mcache_pipe_config: *mut c_void, pub update_dc_state_for_encoder_switch: *mut c_void,
}

#[repr(C)] pub struct audio_support { pub dp_audio: bool, pub hdmi_audio_on_dongle: bool, pub hdmi_audio_native: bool }

#[repr(C)] pub struct resource_pool {
    pub mis: [*mut mem_input; MAX_PIPES], pub hubps: [*mut hubp; MAX_PIPES],
    pub ipps: [*mut input_pixel_processor; MAX_PIPES], pub transforms: [*mut transform; MAX_PIPES],
    pub dpps: [*mut dpp; MAX_PIPES], pub opps: [*mut output_pixel_processor; MAX_PIPES],
    pub timing_generators: [*mut timing_generator; MAX_PIPES], pub stream_enc: [*mut stream_encoder; MAX_PIPES * 2],
    pub hubbub: *mut hubbub, pub dio: *mut dio, pub mpc: *mut mpc, pub pp_smu: *mut pp_smu_funcs,
    pub engines: [*mut dce_aux; MAX_PIPES], pub hw_i2cs: [*mut dce_i2c_hw; MAX_PIPES],
    pub sw_i2cs: [*mut dce_i2c_sw; MAX_PIPES], pub i2c_hw_buffer_in_use: bool,
    pub dwbc: [*mut dwbc; MAX_DWB_PIPES], pub mcif_wb: [*mut mcif_wb; MAX_DWB_PIPES],
    pub gsl_groups: u32, pub dscs: [*mut display_stream_compressor; MAX_PIPES],
    pub pipe_count: u32, pub underlay_pipe_index: u32, pub stream_enc_count: u32,
    pub link_encoders: [*mut link_encoder; MAX_LINK_ENCODERS], pub dig_link_enc_count: u32,
    pub usb4_dpia_count: u32, pub hpo_frl_stream_enc_count: u32,
    pub hpo_frl_stream_enc: [*mut hpo_frl_stream_encoder; MAX_HDMI_FRL_ENCODERS],
    pub hpo_frl_link_enc_count: u32, pub hpo_frl_link_enc: [*mut hpo_frl_link_encoder; MAX_HDMI_FRL_ENCODERS],
    pub hpo_dp_stream_enc_count: u32, pub hpo_dp_stream_enc: [*mut hpo_dp_stream_encoder; MAX_HPO_DP2_ENCODERS],
    pub hpo_dp_link_enc_count: u32, pub hpo_dp_link_enc: [*mut hpo_dp_link_encoder; MAX_HPO_DP2_LINK_ENCODERS],
    pub mpc_lut: [*mut dc_3dlut; MAX_PIPES], pub mpc_shaper: [*mut dc_transfer_func; MAX_PIPES],
    pub rmcm_3dlut: [dc_rmcm_3dlut; MAX_RMCM_INST], pub ref_clocks: ref_clocks,
    pub timing_generator_count: u32, pub mpcc_count: u32, pub writeback_pipe_count: u32,
    pub dp_clock_source: *mut clock_source, pub clock_sources: [*mut clock_source; MAX_CLOCK_SOURCES],
    pub clk_src_count: u32, pub audios: [*mut audio; MAX_AUDIOS], pub audio_count: u32,
    pub audio_support: audio_support, pub dccg: *mut dccg, pub pg_cntl: *mut pg_cntl,
    pub irqs: *mut irq_service, pub abm: *mut abm, pub dmcu: *mut dmcu, pub psr: *mut dmub_psr,
    pub replay: *mut dmub_replay, pub multiple_abms: [*mut abm; MAX_PIPES],
    pub funcs: *const resource_funcs, pub res_cap: *const resource_caps, pub oem_device: *mut ddc_service,
}
#[repr(C)] pub struct ref_clocks { pub xtalin_clock_inKhz: u32, pub dccg_ref_clock_inKhz: u32, pub dchub_ref_clock_inKhz: u32 }
#[repr(C)] pub struct dcn_fe_bandwidth { pub dppclk_khz: i32 }
#[repr(C)] pub struct test_pattern_params { pub test_pattern: controller_dp_test_pattern, pub color_space: controller_dp_color_space, pub color_depth: dc_color_depth, pub width: i32, pub height: i32, pub offset: i32 }
#[repr(C)] pub struct stream_resource { pub opp: *mut output_pixel_processor, pub dsc: *mut display_stream_compressor, pub tg: *mut timing_generator, pub stream_enc: *mut stream_encoder, pub hpo_frl_stream_enc: *mut hpo_frl_stream_encoder, pub hpo_dp_stream_enc: *mut hpo_dp_stream_encoder, pub audio: *mut audio, pub pix_clk_params: pixel_clk_params, pub encoder_info_frame: encoder_info_frame, pub abm: *mut abm, pub gsl_group: u8, pub test_pattern_params: test_pattern_params }
#[repr(C)] pub struct plane_resource { pub scl_data: scaler_data, pub spl_in: spl_in, pub spl_out: spl_out, pub hubp: *mut hubp, pub mi: *mut mem_input, pub ipp: *mut input_pixel_processor, pub xfm: *mut transform, pub dpp: *mut dpp, pub mpcc_inst: u8, pub bw: dcn_fe_bandwidth }
#[repr(C)] pub struct link_resource { pub dio_link_enc: *mut link_encoder, pub hpo_dp_link_enc: *mut hpo_dp_link_encoder, pub hpo_frl_link_enc: *mut hpo_frl_link_encoder }
#[repr(C)] pub struct link_config { pub dp_link_settings: dc_link_settings, pub dp_tunnel_settings: dc_tunnel_settings }
#[repr(C)] pub union pipe_update_flags { pub bits: pipe_update_bits, pub raw: u32 }
#[repr(C)] pub struct pipe_update_bits { pub enable: u32, pub disable: u32, pub odm: u32, pub global_sync: u32, pub opp_changed: u32, pub tg_changed: u32, pub mpcc: u32, pub dppclk: u32, pub hubp_interdependent: u32, pub hubp_rq_dlg_ttu: u32, pub gamut_remap: u32, pub scaler: u32, pub viewport: u32, pub plane_changed: u32, pub det_size: u32, pub unbounded_req: u32, pub test_pattern_changed: u32 }
#[repr(C)] pub struct pixel_rate_divider { pub div_factor1: u32, pub div_factor2: u32 }
#[repr(C)] pub struct dsc_padding_params { pub dsc_hactive_padding: u8, pub dsc_htotal_padding: u32, pub dsc_pix_clk_100hz: u32 }
#[repr(C)] pub struct pipe_ctx { pub plane_state: *mut dc_plane_state, pub stream: *mut dc_stream_state, pub plane_res: plane_resource, pub stream_res: stream_resource, pub link_res: link_resource, pub clock_source: *mut clock_source, pub pll_settings: pll_settings, pub link_config: link_config, pub pipe_idx: u8, pub pipe_idx_syncd: u8, pub top_pipe: *mut pipe_ctx, pub bottom_pipe: *mut pipe_ctx, pub next_odm_pipe: *mut pipe_ctx, pub prev_odm_pipe: *mut pipe_ctx, pub dlg_regs: _vcs_dpi_display_dlg_regs_st, pub ttu_regs: _vcs_dpi_display_ttu_regs_st, pub rq_regs: _vcs_dpi_display_rq_regs_st, pub pipe_dlg_param: _vcs_dpi_display_pipe_dest_params_st, pub dml_rq_param: _vcs_dpi_display_rq_params_st, pub dml_dlg_sys_param: _vcs_dpi_display_dlg_sys_params_st, pub dml_input: _vcs_dpi_display_e2e_pipe_params_st, pub det_buffer_size_kb: i32, pub unbounded_req: bool, pub surface_size_in_mall_bytes: u32, pub hubp_regs: dml2_dchub_per_pipe_register_set, pub mcache_regs: dml2_hubp_pipe_mcache_regs, pub global_sync: dml2_global_sync_programming, pub dwbc: *mut dwbc, pub mcif_wb: *mut mcif_wb, pub update_flags: pipe_update_flags, pub p_state_type: p_state_switch_method, pub visual_confirm_color: tg_color, pub has_vactive_margin: bool, pub subvp_index: u8, pub pixel_rate_divider: pixel_rate_divider, pub dsc_padding_params: dsc_padding_params, pub next_vupdate: u32, pub wait_frame_count: u32, pub wait_is_required: bool }
#[repr(C)] pub enum p_state_switch_method { P_STATE_UNKNOWN=0, P_STATE_V_BLANK=1, P_STATE_FPO, P_STATE_V_ACTIVE, P_STATE_SUB_VP, P_STATE_DRR_SUB_VP, P_STATE_V_BLANK_SUB_VP, P_STATE_ALT }
#[repr(C)] pub struct link_enc_cfg_context { pub mode: link_enc_cfg_mode, pub link_enc_assignments: [link_enc_assignment; MAX_PIPES], pub link_enc_avail: [engine_id; MAX_LINK_ENCODERS], pub transient_assignments: [link_enc_assignment; MAX_PIPES] }
#[repr(C)] pub struct resource_context { pub pipe_ctx: [pipe_ctx; MAX_PIPES], pub is_stream_enc_acquired: [bool; MAX_PIPES*2], pub is_audio_acquired: [bool; MAX_PIPES], pub clock_source_ref_count: [u8; MAX_CLOCK_SOURCES], pub dp_clock_source_ref_count: u8, pub is_dsc_acquired: [bool; MAX_PIPES], pub link_enc_cfg_ctx: link_enc_cfg_context, pub dio_link_enc_to_link_idx: [u32; MAX_LINK_ENCODERS], pub dio_link_enc_ref_cnts: [i32; MAX_LINK_ENCODERS], pub is_hpo_frl_stream_enc_acquired: [bool; MAX_HDMI_FRL_ENCODERS], pub hpo_frl_link_enc_to_link_idx: [u32; MAX_HDMI_FRL_ENCODERS], pub hpo_frl_link_enc_ref_cnts: [i32; MAX_HDMI_FRL_ENCODERS], pub is_hpo_dp_stream_enc_acquired: [bool; MAX_HPO_DP2_ENCODERS], pub hpo_dp_link_enc_to_link_idx: [u32; MAX_HPO_DP2_LINK_ENCODERS], pub hpo_dp_link_enc_ref_cnts: [i32; MAX_HPO_DP2_LINK_ENCODERS], pub is_mpc_3dlut_acquired: [bool; MAX_PIPES], pub temp_pipe: pipe_ctx }
#[repr(C)] pub struct dc_bounding_box_max_clk { pub max_dcfclk_mhz: i32, pub max_dispclk_mhz: i32, pub max_dppclk_mhz: i32, pub max_phyclk_mhz: i32 }
#[repr(C)] pub struct dc_measured_memory_qos { pub peak_bw_mbps:u32, pub avg_bw_mbps:u32, pub max_latency_ns:u32, pub min_latency_ns:u32, pub avg_latency_ns:u32 }
#[repr(C)] pub struct dc_requested_memory_qos { pub bandwidth_lb_in_mbps:u32, pub calculated_avg_bw_in_mbps:u32, pub max_latency_ub_in_ns:u32, pub avg_latency_ub_in_ns:u32, pub max_bw_budget_in_mbps:u32 }
#[repr(C)] pub enum dc_replay_enable { DC_REPLAY_DISABLE=0, DC_REPLAY_ENABLE=1 }
#[repr(C)] pub struct dce_bw_output { pub cpuc_state_change_enable:bool, pub cpup_state_change_enable:bool, pub stutter_mode_enable:bool, pub nbp_state_change_enable:bool, pub all_displays_in_sync:bool, pub urgent_wm_ns:[dce_watermarks;MAX_PIPES], pub stutter_exit_wm_ns:[dce_watermarks;MAX_PIPES], pub stutter_entry_wm_ns:[dce_watermarks;MAX_PIPES], pub nbp_state_change_wm_ns:[dce_watermarks;MAX_PIPES], pub sclk_khz:i32, pub sclk_deep_sleep_khz:i32, pub yclk_khz:i32, pub dispclk_khz:i32, pub blackout_recovery_time_us:i32 }
#[repr(C)] pub struct dcn_bw_writeback { pub mcif_wb_arb:[mcif_arb_params;MAX_DWB_PIPES] }
#[repr(C)] pub struct dcn_bw_output { pub clk:dc_clocks, pub watermarks:dcn_watermark_set, pub bw_writeback:dcn_bw_writeback, pub compbuf_size_kb:i32, pub mall_ss_size_bytes:u32, pub mall_ss_psr_active_size_bytes:u32, pub mall_subvp_size_bytes:u32, pub legacy_svp_drr_stream_index:u32, pub legacy_svp_drr_stream_index_valid:bool, pub mcache_allocations:[dml2_mcache_surface_allocation;DML2_MAX_PLANES], pub fams2_global_config:dmub_cmd_fams2_global_config, pub fams2_stream_base_params:[dmub_cmd_fams2_config;DML2_MAX_PLANES], pub arb_regs:dml2_display_arb_regs }
#[repr(C)] pub union bw_output { pub dcn:dcn_bw_output, pub dce:dce_bw_output }
#[repr(C)] pub struct bw_context { pub bw:bw_output, pub dml:display_mode_lib, pub dml2:*mut dml2_context, pub dml2_dc_power_source:*mut dml2_context }
#[repr(C)] pub struct dc_dmub_cmd { pub dmub_cmd:dmub_rb_cmd, pub wait_type:dm_dmub_wait_type }
#[repr(C)] pub struct perf_params { pub stutter_period_us:u32 }
#[repr(C)] pub struct replay_context { pub aux_inst:channel_id, pub digbe_inst:transmitter, pub digfe_inst:engine_id, pub controllerId:controller_id, pub line_time_in_ns:u32, pub os_request_force_ffu:bool }
#[repr(C)] pub enum update_v3_flow { UPDATE_V3_FLOW_INVALID, UPDATE_V3_FLOW_NO_NEW_CONTEXT_CONTEXT_FAST, UPDATE_V3_FLOW_NO_NEW_CONTEXT_CONTEXT_FULL, UPDATE_V3_FLOW_NEW_CONTEXT_SEAMLESS, UPDATE_V3_FLOW_NEW_CONTEXT_MINIMAL_NEW, UPDATE_V3_FLOW_NEW_CONTEXT_MINIMAL_CURRENT }
#[repr(C)] pub struct pipe_split_policy_backup { pub dynamic_odm_policy: bool, pub subvp_policy: bool, pub mpc_policy: pipe_split_policy, pub force_odm: [c_char; MAX_PIPES] }
#[repr(C)] pub struct dc_update_scratch_space { pub dc:*mut dc, pub surface_updates:*mut dc_surface_update, pub surface_count:i32, pub stream:*mut dc_stream_state, pub stream_update:*mut dc_stream_update, pub probe_updates:*const dc_probe_updates, pub update_v3:bool, pub do_clear_update_bits:bool, pub update_type:dc_update_type, pub new_context:*mut dc_state, pub flow:update_v3_flow, pub backup_context:*mut dc_state, pub intermediate_context:*mut dc_state, pub intermediate_policy:pipe_split_policy_backup, pub intermediate_updates:[dc_surface_update; MAX_SURFACES], pub intermediate_count:i32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
