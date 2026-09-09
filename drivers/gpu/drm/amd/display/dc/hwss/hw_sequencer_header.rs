/*
 * Copyright 2015-2026 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 *
 */


struct pipe_ctx;
struct dc_state;
struct dc_stream_status;
struct dc_writeback_info;
struct dchub_init_data;
struct dc_static_screen_params;
struct resource_pool;
struct dc_phy_addr_space_config;
struct dc_virtual_addr_space_config;
struct dpp;
struct dce_hwseq;
struct link_resource;
struct dc_dmub_cmd;
struct pg_block_update;
struct drr_params;
struct dc_underflow_debug_data;
struct dsc_optc_config;
struct vm_system_aperture_param;
struct stream_encoder;
struct hpo_dp_stream_encoder;
struct hpo_frl_stream_encoder;
struct link_training_settings;
struct dc_link;
struct dc_crtc_timing;
#[repr(C)]
pub struct subvp_pipe_control_lock_fast_params {
	*mut dcdc;
	bool lock;
	bool subvp_immediate_flip;
};

#[repr(C)]
pub struct pipe_control_lock_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
	bool lock;
};

#[repr(C)]
pub struct set_flip_control_gsl_params {
	*mut hubphubp;
	bool flip_immediate;
};

#[repr(C)]
pub struct program_triplebuffer_params {
	const *mut dcdc;
	*mut pipe_ctxpipe_ctx;
	bool enableTripleBuffer;
};

#[repr(C)]
pub struct update_plane_addr_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct set_input_transfer_func_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
	*mut dc_plane_stateplane_state;
};

#[repr(C)]
pub struct program_gamut_remap_params {
	*mut transformxfm;
	*mut dppdpp;
	*mut mpcmpc;
	int mpcc_id;
	const *mut dc_stream_statestream;
	const *mut dc_plane_stateplane;
	bool is_top_pipe;
};

#[repr(C)]
pub struct hubp_enable_3dlut_fl_params {
	*mut hubphubp;
};

#[repr(C)]
pub struct tg_setup_vertical_interrupt0_params {
	*mut timing_generatortg;
	u32 start_line;
	u32 end_line;
};

#[repr(C)]
pub struct update_info_frame_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct program_manual_trigger_params {
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct send_dmcub_cmd_params {
	*mut dc_contextctx;
	*mut dmub_rb_cmdcmd;
	enum dm_dmub_wait_type wait_type;
};

#[repr(C)]
pub struct setup_dpp_params {
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct program_bias_and_scale_params {
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct set_output_transfer_func_params {
	*mut transformxfm;
	*mut dppdpp;
	*mut mpcmpc;
	int mpcc_id;
	bool is_top_pipe;
	const *mut dc_stream_statestream;
};
#[repr(C)]
pub struct program_upsp_params {
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct update_visual_confirm_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
	int mpcc_id;
};

#[repr(C)]
pub struct power_on_mpc_mem_pwr_params {
	*mut mpcmpc;
	int mpcc_id;
	bool power_on;
};

#[repr(C)]
pub struct set_output_csc_params {
	*mut mpcmpc;
	int opp_id;
	const u16 *regval;
	enum mpc_output_csc_mode ocsc_mode;
};

#[repr(C)]
pub struct set_ocsc_default_params {
	*mut mpcmpc;
	int opp_id;
	enum dc_color_space color_space;
	enum mpc_output_csc_mode ocsc_mode;
};

#[repr(C)]
pub struct subvp_save_surf_addr {
	*mut dc_dmub_srvdc_dmub_srv;
	const *mut dc_plane_addressaddr;
	u8 subvp_index;
};

#[repr(C)]
pub struct wait_for_dcc_meta_propagation_params {
	const *mut dcdc;
	const *mut pipe_ctxtop_pipe_to_program;
};

#[repr(C)]
pub struct dmub_hw_control_lock_fast_params {
	*mut dcdc;
	bool is_required;
	bool lock;
};

#[repr(C)]
pub struct program_surface_config_params {
	*mut hubphubp;
	enum surface_pixel_format format;
	*mut dc_tiling_infotiling_info;
	struct plane_size plane_size;
	enum dc_rotation_angle rotation;
	*mut dc_plane_dcc_paramdcc;
	bool horizontal_mirror;
	int compat_level;
};

#[repr(C)]
pub struct program_mcache_id_and_split_coordinate {
	*mut hubphubp;
	*mut dml2_hubp_pipe_mcache_regsmcache_regs;
};

#[repr(C)]
pub struct control_cm_hist_params {
	*mut dppdpp;
	struct cm_hist_control cm_hist_control;
	enum dc_color_space color_space;
};

#[repr(C)]
pub struct program_cursor_update_now_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct hubp_wait_pipe_read_start_params {
	*mut hubphubp;
};

#[repr(C)]
pub struct apply_update_flags_for_phantom_params {
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct update_phantom_vp_position_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
	*mut dc_statecontext;
};

#[repr(C)]
pub struct set_odm_combine_params {
	*mut timing_generatortg;
	int opp_inst[MAX_PIPES];
	int opp_head_count;
	int odm_slice_width;
	int last_odm_slice_width;
};

#[repr(C)]
pub struct set_odm_bypass_params {
	*mut timing_generatortg;
	const *mut dc_crtc_timingtiming;
};

#[repr(C)]
pub struct opp_pipe_clock_control_params {
	*mut output_pixel_processoropp;
	bool enable;
};

#[repr(C)]
pub struct opp_program_left_edge_extra_pixel_params {
	*mut output_pixel_processoropp;
	enum dc_pixel_encoding pixel_encoding;
	bool is_otg_master;
};

#[repr(C)]
pub struct dccg_set_dto_dscclk_params {
	*mut dccgdccg;
	int inst;
	int num_slices_h;
};

#[repr(C)]
pub struct dsc_set_config_params {
	*mut display_stream_compressordsc;
	*mut dsc_configdsc_cfg;
	*mut dsc_optc_configdsc_optc_cfg;
};

#[repr(C)]
pub struct dsc_enable_params {
	*mut display_stream_compressordsc;
	int opp_inst;
};

#[repr(C)]
pub struct tg_set_dsc_config_params {
	*mut timing_generatortg;
	*mut dsc_optc_configdsc_optc_cfg;
	bool enable;
};

#[repr(C)]
pub struct dsc_disconnect_params {
	*mut display_stream_compressordsc;
};

#[repr(C)]
pub struct dsc_read_state_params {
	*mut display_stream_compressordsc;
	*mut dcn_dsc_statedsc_state;
};

#[repr(C)]
pub struct dsc_calculate_and_set_config_params {
	*mut pipe_ctxpipe_ctx;
	struct dsc_optc_config dsc_optc_cfg;
	bool enable;
	int opp_cnt;
};

#[repr(C)]
pub struct dsc_enable_with_opp_params {
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct program_tg_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
	*mut dc_statecontext;
};

#[repr(C)]
pub struct tg_program_global_sync_params {
	*mut timing_generatortg;
	int vready_offset;
	u32 vstartup_lines;
	u32 vupdate_offset_pixels;
	u32 vupdate_vupdate_width_pixels;
	u32 pstate_keepout_start_lines;
};

#[repr(C)]
pub struct tg_wait_for_state_params {
	*mut timing_generatortg;
	enum crtc_state state;
};

#[repr(C)]
pub struct tg_set_vtg_params_params {
	*mut timing_generatortg;
	*mut dc_crtc_timingtiming;
	bool program_fp2;
};

#[repr(C)]
pub struct tg_set_gsl_params {
	*mut timing_generatortg;
	struct gsl_params gsl;
};

#[repr(C)]
pub struct tg_set_gsl_source_select_params {
	*mut timing_generatortg;
	int group_idx;
	u32 gsl_ready_signal;
};

#[repr(C)]
pub struct setup_vupdate_interrupt_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct tg_setup_vertical_interrupt2_params {
	*mut timing_generatortg;
	int start_line;
};

#[repr(C)]
pub struct dpp_set_hdr_multiplier_params {
	*mut dppdpp;
	u32 hw_mult;
};

#[repr(C)]
pub struct program_det_size_params {
	*mut hubbubhubbub;
	u32 hubp_inst;
	u32 det_buffer_size_kb;
};

#[repr(C)]
pub struct program_det_segments_params {
	*mut hubbubhubbub;
	u32 hubp_inst;
	u32 det_size;
};

#[repr(C)]
pub struct update_dchubp_dpp_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
	*mut dc_statecontext;
};

#[repr(C)]
pub struct opp_set_dyn_expansion_params {
	*mut output_pixel_processoropp;
	enum dc_color_space color_space;
	enum dc_color_depth color_depth;
	enum signal_type signal;
};

#[repr(C)]
pub struct opp_program_fmt_params {
	*mut output_pixel_processoropp;
	*mut bit_depth_reduction_paramsfmt_bit_depth;
	*mut clamping_and_pixel_encoding_paramsclamping;
};

#[repr(C)]
pub struct opp_program_bit_depth_reduction_params {
	*mut output_pixel_processoropp;
	bool use_default_params;
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct opp_set_disp_pattern_generator_params {
	*mut output_pixel_processoropp;
	enum controller_dp_test_pattern test_pattern;
	enum controller_dp_color_space color_space;
	enum dc_color_depth color_depth;
	struct tg_color solid_color;
	bool use_solid_color;
	int width;
	int height;
	int offset;
};

#[repr(C)]
pub struct set_abm_pipe_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct set_abm_level_params {
	*mut abmabm;
	u32 abm_level;
};

#[repr(C)]
pub struct set_abm_immediate_disable_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct set_disp_pattern_generator_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
	enum controller_dp_test_pattern test_pattern;
	enum controller_dp_color_space color_space;
	enum dc_color_depth color_depth;
	const *mut tg_colorsolid_color;
	int width;
	int height;
	int offset;
};

#[repr(C)]
pub struct mpc_update_blending_params {
	*mut mpcmpc;
	struct mpcc_blnd_cfg blnd_cfg;
	int mpcc_id;
};

#[repr(C)]
pub struct mpc_assert_idle_mpcc_params {
	*mut mpcmpc;
	int mpcc_id;
};

#[repr(C)]
pub struct mpc_insert_plane_params {
	*mut mpcmpc;
	*mut mpc_treempc_tree_params;
	struct mpcc_blnd_cfg blnd_cfg;
	*mut mpcc_sm_cfgsm_cfg;
	*mut mpccinsert_above_mpcc;
	int dpp_id;
	int mpcc_id;
};

#[repr(C)]
pub struct mpc_remove_mpcc_params {
	*mut mpcmpc;
	*mut mpc_treempc_tree_params;
	*mut mpccmpcc_to_remove;
};

#[repr(C)]
pub struct opp_set_mpcc_disconnect_pending_params {
	*mut output_pixel_processoropp;
	int mpcc_inst;
	bool pending;
};

#[repr(C)]
pub struct dc_set_optimized_required_params {
	*mut dcdc;
	bool optimized_required;
};

#[repr(C)]
pub struct hubp_disconnect_params {
	*mut hubphubp;
};

#[repr(C)]
pub struct hubbub_force_pstate_change_control_params {
	*mut hubbubhubbub;
	bool enable;
	bool wait;
};

#[repr(C)]
pub struct tg_enable_crtc_params {
	*mut timing_generatortg;
};

#[repr(C)]
pub struct hubp_wait_flip_pending_params {
	*mut hubphubp;
	u32 timeout_us;
	u32 polling_interval_us;
};

#[repr(C)]
pub struct tg_wait_double_buffer_pending_params {
	*mut timing_generatortg;
	u32 timeout_us;
	u32 polling_interval_us;
};

#[repr(C)]
pub struct update_force_pstate_params {
	*mut dcdc;
	*mut dc_statecontext;
};

#[repr(C)]
pub struct hubbub_apply_dedcn21_147_wa_params {
	*mut hubbubhubbub;
};

#[repr(C)]
pub struct hubbub_allow_self_refresh_control_params {
	*mut hubbubhubbub;
	bool allow;
	bool *disallow_self_refresh_applied;
};

#[repr(C)]
pub struct tg_get_frame_count_params {
	*mut timing_generatortg;
	u32 *frame_count;
};

#[repr(C)]
pub struct mpc_set_dwb_mux_params {
	*mut mpcmpc;
	int dwb_id;
	int mpcc_id;
};

#[repr(C)]
pub struct mpc_disable_dwb_mux_params {
	*mut mpcmpc;
	u32 dwb_id;
};

#[repr(C)]
pub struct mcif_wb_config_buf_params {
	*mut mcif_wbmcif_wb;
	*mut mcif_buf_paramsmcif_buf_params;
	u32 dest_height;
};

#[repr(C)]
pub struct mcif_wb_config_arb_params {
	*mut mcif_wbmcif_wb;
	*mut mcif_arb_paramsmcif_arb_params;
};

#[repr(C)]
pub struct mcif_wb_enable_params {
	*mut mcif_wbmcif_wb;
};

#[repr(C)]
pub struct mcif_wb_disable_params {
	*mut mcif_wbmcif_wb;
};

#[repr(C)]
pub struct dwbc_enable_params {
	*mut dwbcdwb;
	*mut dc_dwb_paramsdwb_params;
};

#[repr(C)]
pub struct dwbc_disable_params {
	*mut dwbcdwb;
};

#[repr(C)]
pub struct dwbc_update_params {
	*mut dwbcdwb;
	*mut dc_dwb_paramsdwb_params;
};

#[repr(C)]
pub struct hubp_update_mall_sel_params {
	*mut hubphubp;
	u32 mall_sel;
	bool cache_cursor;
};

#[repr(C)]
pub struct hubp_prepare_subvp_buffering_params {
	*mut hubphubp;
	bool enable;
};

#[repr(C)]
pub struct hubp_set_blank_en_params {
	*mut hubphubp;
	bool enable;
};

#[repr(C)]
pub struct hubp_disable_control_params {
	*mut hubphubp;
	bool disable;
};

#[repr(C)]
pub struct hubbub_soft_reset_params {
	*mut hubbubhubbub;
	void (*hubbub_soft_reset)(*mut hubbubhubbub, bool reset);
	bool reset;
};

#[repr(C)]
pub struct hubbub_perfmon_reset_params {
	*mut hubbubhubbub;
};

#[repr(C)]
pub struct hubbub_perfmon_arm_out_of_order_bw_params {
	*mut hubbubhubbub;
};

#[repr(C)]
pub struct hubbub_perfmon_start_out_of_order_bw_params {
	*mut hubbubhubbub;
};

#[repr(C)]
pub struct hubbub_perfmon_start_in_order_bw_params {
	*mut hubbubhubbub;
};

#[repr(C)]
pub struct hubbub_perfmon_start_memory_latencies_params {
	*mut hubbubhubbub;
};

#[repr(C)]
pub struct hubbub_perfmon_start_urgent_assertion_count_params {
	*mut hubbubhubbub;
};

#[repr(C)]
pub struct hubbub_perfmon_start_urgent_ramp_latency_params {
	*mut hubbubhubbub;
	struct hubbub_urgent_latency_params latency_params;
};

#[repr(C)]
pub struct hubbub_perfmon_start_prefetch_data_size_params {
	*mut hubbubhubbub;
};

#[repr(C)]
pub struct hubbub_perfmon_get_out_of_order_bw_params {
	*mut hubbubhubbub;
	u32       refclk_mhz;
	u32      *bandwidth_mbps;
	u32      *duration_ns;
};

#[repr(C)]
pub struct hubbub_perfmon_get_in_order_bw_params {
	*mut hubbubhubbub;
	u32       refclk_mhz;
	u32       min_duration_ns;
	u32      *bandwidth_mbps;
	u32      *duration_ns;
};

#[repr(C)]
pub struct hubbub_perfmon_get_memory_latencies_params {
	*mut hubbubhubbub;
	u32                              refclk_mhz;
	*mut dc_probe_latenciesresult;
};

#[repr(C)]
pub struct hubbub_perfmon_get_urgent_assertion_count_params {
	*mut hubbubhubbub;
	u32       refclk_mhz;
	u32      *assertion_count;
};

#[repr(C)]
pub struct hubbub_perfmon_get_prefetch_data_size_params {
	*mut hubbubhubbub;
	u32      *prefetch_data_size;
};

#[repr(C)]
pub struct hubbub_perfmon_get_urgent_ramp_latency_params {
	*mut hubbubhubbub;
	u32       refclk_mhz;
	u32      *latency_ns;
};

#[repr(C)]
pub struct hubp_clk_cntl_params {
	*mut hubphubp;
	bool enable;
};

#[repr(C)]
pub struct hubp_init_params {
	*mut hubphubp;
};

#[repr(C)]
pub struct hubp_set_vm_system_aperture_settings_params {
	*mut hubphubp;
	//struct vm_system_aperture_param apt;
	u64 sys_default;
	u64 sys_low;
	u64 sys_high;
};

#[repr(C)]
pub struct hubp_set_flip_int_params {
	*mut hubphubp;
};

#[repr(C)]
pub struct dpp_dppclk_control_params {
	*mut dppdpp;
	bool dppclk_div;
	bool enable;
};

#[repr(C)]
pub struct disable_phantom_crtc_params {
	*mut timing_generatortg;
};

#[repr(C)]
pub struct dpp_pg_control_params {
	*mut dce_hwseqhws;
	u32 dpp_inst;
	bool power_on;
};

#[repr(C)]
pub struct hubp_pg_control_params {
	*mut dce_hwseqhws;
	u32 hubp_inst;
	bool power_on;
};

#[repr(C)]
pub struct hubp_reset_params {
	*mut hubphubp;
};

#[repr(C)]
pub struct dpp_reset_params {
	*mut dppdpp;
};

#[repr(C)]
pub struct dpp_root_clock_control_params {
	*mut dce_hwseqhws;
	u32 dpp_inst;
	bool clock_on;
};

#[repr(C)]
pub struct dc_ip_request_cntl_params {
	*mut dcdc;
	bool enable;
};

#[repr(C)]
pub struct dsc_pg_status_params {
	*mut dce_hwseqhws;
	int dsc_inst;
	bool is_ungated;
};

#[repr(C)]
pub struct dsc_wait_disconnect_pending_clear_params {
	*mut display_stream_compressordsc;
	bool *is_ungated;
};

#[repr(C)]
pub struct dsc_disable_params {
	*mut display_stream_compressordsc;
	bool *is_ungated;
};

#[repr(C)]
pub struct dccg_set_ref_dscclk_params {
	*mut dccgdccg;
	int dsc_inst;
	bool *is_ungated;
};

#[repr(C)]
pub struct dccg_update_dpp_dto_params {
	*mut dccgdccg;
	int dpp_inst;
	int dppclk_khz;
};

#[repr(C)]
pub struct hubp_vtg_sel_params {
	*mut hubphubp;
	u32 otg_inst;
};

#[repr(C)]
pub struct hubp_setup2_params {
	*mut hubphubp;
	*mut dml2_dchub_per_pipe_register_sethubp_regs;
	*mut dml2_global_sync_programmingglobal_sync;
	*mut dc_crtc_timingtiming;
};

#[repr(C)]
pub struct hubp_setup_params {
	*mut hubphubp;
	*mut _vcs_dpi_display_dlg_regs_stdlg_regs;
	*mut _vcs_dpi_display_ttu_regs_stttu_regs;
	*mut _vcs_dpi_display_rq_regs_strq_regs;
	*mut _vcs_dpi_display_pipe_dest_params_stpipe_dest;
};

#[repr(C)]
pub struct hubp_set_unbounded_requesting_params {
	*mut hubphubp;
	bool unbounded_req;
};

#[repr(C)]
pub struct hubp_setup_interdependent2_params {
	*mut hubphubp;
	*mut dml2_dchub_per_pipe_register_sethubp_regs;
};

#[repr(C)]
pub struct hubp_setup_interdependent_params {
	*mut hubphubp;
	*mut _vcs_dpi_display_dlg_regs_stdlg_regs;
	*mut _vcs_dpi_display_ttu_regs_stttu_regs;
};

#[repr(C)]
pub struct dpp_set_cursor_matrix_params {
	*mut dppdpp;
	enum dc_color_space color_space;
	*mut dc_csc_transformcursor_csc_color_matrix;
};

#[repr(C)]
pub struct mpc_update_mpcc_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct dpp_set_scaler_params {
	*mut dppdpp;
	const *mut scaler_datascl_data;
};

#[repr(C)]
pub struct hubp_mem_program_viewport_params {
	*mut hubphubp;
	const *mut rectviewport;
	const *mut rectviewport_c;
};

#[repr(C)]
pub struct hubp_program_mcache_id_and_split_coordinate_params {
	*mut hubphubp;
	*mut mcache_regs_structmcache_regs;
};

#[repr(C)]
pub struct abort_cursor_offload_update_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct cursor_lock_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
	bool lock;
};

#[repr(C)]
pub struct setup_periodic_interrupt_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct send_cursor_info_to_dmu_params {
	*mut pipe_ctxpipe_ctx;
	int pipe_idx;
};

#[repr(C)]
pub struct set_cursor_attribute_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct hubp_set_cursor_attributes_params {
	*mut hubphubp;
	const *mut dc_cursor_attributesattributes;
};

#[repr(C)]
pub struct dpp_set_cursor_attributes_params {
	*mut dppdpp;
	*mut dc_cursor_attributesattributes;
};

#[repr(C)]
pub struct set_cursor_position_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct set_cursor_sdr_white_level_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct program_output_csc_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
	enum dc_color_space colorspace;
	u16 *matrix;
	int opp_id;
};

#[repr(C)]
pub struct hubp_set_blank_params {
	*mut hubphubp;
	bool blank;
};

#[repr(C)]
pub struct phantom_hubp_post_enable_params {
	*mut hubphubp;
};

#[repr(C)]
pub struct begin_cursor_offload_update_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct update_cursor_offload_pipe_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct commit_cursor_offload_update_params {
	*mut dcdc;
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct stream_enc_update_hdmi_info_packets_params {
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct hpo_frl_stream_enc_update_hdmi_info_packets_params {
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct hpo_dp_stream_enc_update_dp_info_packets_sdp_line_num_params {
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct hpo_dp_stream_enc_update_dp_info_packets_params {
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct stream_enc_update_dp_info_packets_sdp_line_num_params {
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct stream_enc_update_dp_info_packets_params {
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct dsc_set_config_simple_params {
	*mut display_stream_compressordsc;
	struct dsc_config dsc_cfg;
	struct dsc_optc_config dsc_optc_cfg;
};

#[repr(C)]
pub struct stream_enc_dp_set_dsc_config_params {
	*mut stream_encoderstream_enc;
	const *mut dsc_optc_configdsc_optc_cfg;
};

#[repr(C)]
pub struct hpo_dp_stream_enc_dp_set_dsc_pps_info_packet_params {
	*mut hpo_dp_stream_encoderhpo_dp_stream_enc;
	bool immediate_update;
	u8 *dsc_packed_pps;
	bool pps_sdp_stream;
};

#[repr(C)]
pub struct stream_enc_dp_set_dsc_pps_info_packet_params {
	*mut stream_encoderstream_enc;
	bool immediate_update;
	u8 *dsc_packed_pps;
	bool pps_sdp_stream;
};

#[repr(C)]
pub struct hpo_frl_stream_enc_set_dsc_config_params {
	*mut hpo_frl_stream_encoderhpo_frl_stream_enc;
	const *mut dc_crtc_timingtiming;
	u8 *dsc_packed_pps;
};

#[repr(C)]
pub struct dp_trace_source_sequence_params {
	*mut dc_linklink;
	enum dpcd_source_sequence source;
};

#[repr(C)]
pub struct set_dmdata_attributes_params {
	*mut hubphubp;
	struct dc_dmdata_attributes attr;
};

#[repr(C)]
pub struct link_increase_mst_payload_params {
	*mut pipe_ctxpipe_ctx;
	u32 mst_stream_bw;
};

#[repr(C)]
pub struct link_reduce_mst_payload_params {
	*mut pipe_ctxpipe_ctx;
	u32 mst_stream_bw;
};

#[repr(C)]
pub struct dp_set_test_pattern_params {
	*mut dc_linklink;
	enum dp_test_pattern test_pattern;
	enum dp_test_pattern_color_space test_pattern_color_space;
	const *mut link_training_settingsp_link_settings;
	const u8 *p_custom_pattern;
	u32 cust_pattern_size;
};

#[repr(C)]
pub struct link_set_dpms_off_params {
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct disable_audio_stream_params {
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub struct clk_mgr_set_max_memclk_params {
	*mut clk_mgrclk_mgr;
	u32 memclk_mhz;
};

#[repr(C)]
pub struct clk_mgr_update_clocks_params {
	*mut clk_mgrclk_mgr;
};

#[repr(C)]
pub struct hubbub_program_watermarks_params {
	*mut dcdc;
	*mut hubbubhubbub;
	*mut dcn_watermark_setwatermarks;
	u32 refclk_mhz;
	bool safe_to_lower;
};

#[repr(C)]
pub struct hubbub_program_arbiter_params {
	*mut dcdc;
	*mut hubbubhubbub;
	*mut dml2_display_arb_regsarb_regs;
	bool safe_to_lower;
};

#[repr(C)]
pub struct hubbub_program_compbuf_segments_params {
	*mut hubbubhubbub;
	u32 compbuf_size;
	bool safe_to_lower;
};

#[repr(C)]
pub struct prepare_bandwidth_params {
	*mut dcdc;
	*mut dc_statecontext;
};

#[repr(C)]
pub struct link_set_dpms_on_params {
	*mut dc_statestate;
	*mut pipe_ctxpipe_ctx;
};

#[repr(C)]
pub union block_sequence_params {
	struct update_plane_addr_params update_plane_addr_params;
	struct subvp_pipe_control_lock_fast_params subvp_pipe_control_lock_fast_params;
	struct pipe_control_lock_params pipe_control_lock_params;
	struct set_flip_control_gsl_params set_flip_control_gsl_params;
	struct program_triplebuffer_params program_triplebuffer_params;
	struct set_input_transfer_func_params set_input_transfer_func_params;
	struct program_gamut_remap_params program_gamut_remap_params;
	struct hubp_enable_3dlut_fl_params hubp_enable_3dlut_fl_params;
	struct tg_setup_vertical_interrupt0_params tg_setup_vertical_interrupt0_params;
	struct update_info_frame_params update_info_frame_params;
	struct program_manual_trigger_params program_manual_trigger_params;
	struct send_dmcub_cmd_params send_dmcub_cmd_params;
	struct setup_dpp_params setup_dpp_params;
	struct program_bias_and_scale_params program_bias_and_scale_params;
	struct set_output_transfer_func_params set_output_transfer_func_params;
	struct program_upsp_params program_upsp_params;
	struct update_visual_confirm_params update_visual_confirm_params;
	struct power_on_mpc_mem_pwr_params power_on_mpc_mem_pwr_params;
	struct set_output_csc_params set_output_csc_params;
	struct set_ocsc_default_params set_ocsc_default_params;
	struct subvp_save_surf_addr subvp_save_surf_addr;
	struct wait_for_dcc_meta_propagation_params wait_for_dcc_meta_propagation_params;
	struct dmub_hw_control_lock_fast_params dmub_hw_control_lock_fast_params;
	struct program_surface_config_params program_surface_config_params;
	struct program_mcache_id_and_split_coordinate program_mcache_id_and_split_coordinate;
	struct control_cm_hist_params control_cm_hist_params;
	struct program_cursor_update_now_params program_cursor_update_now_params;
	struct hubp_wait_pipe_read_start_params hubp_wait_pipe_read_start_params;
	struct apply_update_flags_for_phantom_params apply_update_flags_for_phantom_params;
	struct update_phantom_vp_position_params update_phantom_vp_position_params;
	struct set_odm_combine_params set_odm_combine_params;
	struct set_odm_bypass_params set_odm_bypass_params;
	struct opp_pipe_clock_control_params opp_pipe_clock_control_params;
	struct opp_program_left_edge_extra_pixel_params opp_program_left_edge_extra_pixel_params;
	struct dccg_set_dto_dscclk_params dccg_set_dto_dscclk_params;
	struct dsc_set_config_params dsc_set_config_params;
	struct dsc_enable_params dsc_enable_params;
	struct tg_set_dsc_config_params tg_set_dsc_config_params;
	struct dsc_disconnect_params dsc_disconnect_params;
	struct dsc_read_state_params dsc_read_state_params;
	struct dsc_calculate_and_set_config_params dsc_calculate_and_set_config_params;
	struct dsc_enable_with_opp_params dsc_enable_with_opp_params;
	struct program_tg_params program_tg_params;
	struct tg_program_global_sync_params tg_program_global_sync_params;
	struct tg_wait_for_state_params tg_wait_for_state_params;
	struct tg_set_vtg_params_params tg_set_vtg_params_params;
	struct tg_setup_vertical_interrupt2_params tg_setup_vertical_interrupt2_params;
	struct dpp_set_hdr_multiplier_params dpp_set_hdr_multiplier_params;
	struct tg_set_gsl_params tg_set_gsl_params;
	struct tg_set_gsl_source_select_params tg_set_gsl_source_select_params;
	struct setup_vupdate_interrupt_params setup_vupdate_interrupt_params;
	struct program_det_size_params program_det_size_params;
	struct program_det_segments_params program_det_segments_params;
	struct update_dchubp_dpp_params update_dchubp_dpp_params;
	struct opp_set_dyn_expansion_params opp_set_dyn_expansion_params;
	struct opp_program_fmt_params opp_program_fmt_params;
	struct opp_program_bit_depth_reduction_params opp_program_bit_depth_reduction_params;
	struct opp_set_disp_pattern_generator_params opp_set_disp_pattern_generator_params;
	struct set_abm_pipe_params set_abm_pipe_params;
	struct set_abm_level_params set_abm_level_params;
	struct set_abm_immediate_disable_params set_abm_immediate_disable_params;
	struct set_disp_pattern_generator_params set_disp_pattern_generator_params;
	struct mpc_remove_mpcc_params mpc_remove_mpcc_params;
	struct opp_set_mpcc_disconnect_pending_params opp_set_mpcc_disconnect_pending_params;
	struct dc_set_optimized_required_params dc_set_optimized_required_params;
	struct hubp_disconnect_params hubp_disconnect_params;
	struct hubbub_force_pstate_change_control_params hubbub_force_pstate_change_control_params;
	struct tg_enable_crtc_params tg_enable_crtc_params;
	struct hubp_wait_flip_pending_params hubp_wait_flip_pending_params;
	struct tg_wait_double_buffer_pending_params tg_wait_double_buffer_pending_params;
	struct update_force_pstate_params update_force_pstate_params;
	struct hubbub_apply_dedcn21_147_wa_params hubbub_apply_dedcn21_147_wa_params;
	struct hubbub_allow_self_refresh_control_params hubbub_allow_self_refresh_control_params;
	struct tg_get_frame_count_params tg_get_frame_count_params;
	struct mpc_set_dwb_mux_params mpc_set_dwb_mux_params;
	struct mpc_disable_dwb_mux_params mpc_disable_dwb_mux_params;
	struct mcif_wb_config_buf_params mcif_wb_config_buf_params;
	struct mcif_wb_config_arb_params mcif_wb_config_arb_params;
	struct mcif_wb_enable_params mcif_wb_enable_params;
	struct mcif_wb_disable_params mcif_wb_disable_params;
	struct dwbc_enable_params dwbc_enable_params;
	struct dwbc_disable_params dwbc_disable_params;
	struct dwbc_update_params dwbc_update_params;
	struct hubp_update_mall_sel_params hubp_update_mall_sel_params;
	struct hubp_prepare_subvp_buffering_params hubp_prepare_subvp_buffering_params;
	struct hubp_set_blank_en_params hubp_set_blank_en_params;
	struct hubp_disable_control_params hubp_disable_control_params;
	struct hubbub_soft_reset_params hubbub_soft_reset_params;
	struct hubbub_perfmon_reset_params hubbub_perfmon_reset_params;
	struct hubbub_perfmon_arm_out_of_order_bw_params hubbub_perfmon_arm_out_of_order_bw_params;
	struct hubbub_perfmon_start_out_of_order_bw_params hubbub_perfmon_start_out_of_order_bw_params;
	struct hubbub_perfmon_start_in_order_bw_params hubbub_perfmon_start_in_order_bw_params;
	struct hubbub_perfmon_start_memory_latencies_params hubbub_perfmon_start_memory_latencies_params;
	struct hubbub_perfmon_start_urgent_assertion_count_params hubbub_perfmon_start_urgent_assertion_count_params;
	struct hubbub_perfmon_start_urgent_ramp_latency_params hubbub_perfmon_start_urgent_ramp_latency_params;
	struct hubbub_perfmon_start_prefetch_data_size_params hubbub_perfmon_start_prefetch_data_size_params;
	struct hubbub_perfmon_get_out_of_order_bw_params hubbub_perfmon_get_out_of_order_bw_params;
	struct hubbub_perfmon_get_in_order_bw_params hubbub_perfmon_get_in_order_bw_params;
	struct hubbub_perfmon_get_memory_latencies_params hubbub_perfmon_get_memory_latencies_params;
	struct hubbub_perfmon_get_urgent_assertion_count_params hubbub_perfmon_get_urgent_assertion_count_params;
	struct hubbub_perfmon_get_prefetch_data_size_params hubbub_perfmon_get_prefetch_data_size_params;
	struct hubbub_perfmon_get_urgent_ramp_latency_params hubbub_perfmon_get_urgent_ramp_latency_params;
	struct hubp_clk_cntl_params hubp_clk_cntl_params;
	struct hubp_init_params hubp_init_params;
	struct hubp_set_vm_system_aperture_settings_params hubp_set_vm_system_aperture_settings_params;
	struct hubp_set_flip_int_params hubp_set_flip_int_params;
	struct dpp_dppclk_control_params dpp_dppclk_control_params;
	struct disable_phantom_crtc_params disable_phantom_crtc_params;
	struct dpp_pg_control_params dpp_pg_control_params;
	struct hubp_pg_control_params hubp_pg_control_params;
	struct hubp_reset_params hubp_reset_params;
	struct dpp_reset_params dpp_reset_params;
	struct dpp_root_clock_control_params dpp_root_clock_control_params;
	struct dc_ip_request_cntl_params dc_ip_request_cntl_params;
	struct dsc_pg_status_params dsc_pg_status_params;
	struct dsc_wait_disconnect_pending_clear_params dsc_wait_disconnect_pending_clear_params;
	struct dsc_disable_params dsc_disable_params;
	struct dccg_set_ref_dscclk_params dccg_set_ref_dscclk_params;
	struct dccg_update_dpp_dto_params dccg_update_dpp_dto_params;
	struct hubp_vtg_sel_params hubp_vtg_sel_params;
	struct hubp_setup2_params hubp_setup2_params;
	struct hubp_setup_params hubp_setup_params;
	struct hubp_set_unbounded_requesting_params hubp_set_unbounded_requesting_params;
	struct hubp_setup_interdependent2_params hubp_setup_interdependent2_params;
	struct hubp_setup_interdependent_params hubp_setup_interdependent_params;
	struct dpp_set_cursor_matrix_params dpp_set_cursor_matrix_params;
	struct mpc_update_mpcc_params mpc_update_mpcc_params;
	struct mpc_update_blending_params mpc_update_blending_params;
	struct mpc_assert_idle_mpcc_params mpc_assert_idle_mpcc_params;
	struct mpc_insert_plane_params mpc_insert_plane_params;
	struct dpp_set_scaler_params dpp_set_scaler_params;
	struct hubp_mem_program_viewport_params hubp_mem_program_viewport_params;
	struct abort_cursor_offload_update_params abort_cursor_offload_update_params;
	struct cursor_lock_params cursor_lock_params;
	struct setup_periodic_interrupt_params setup_periodic_interrupt_params;
	struct send_cursor_info_to_dmu_params send_cursor_info_to_dmu_params;
	struct set_cursor_attribute_params set_cursor_attribute_params;
	struct hubp_set_cursor_attributes_params hubp_set_cursor_attributes_params;
	struct dpp_set_cursor_attributes_params dpp_set_cursor_attributes_params;
	struct set_cursor_position_params set_cursor_position_params;
	struct set_cursor_sdr_white_level_params set_cursor_sdr_white_level_params;
	struct program_output_csc_params program_output_csc_params;
	struct hubp_set_blank_params hubp_set_blank_params;
	struct phantom_hubp_post_enable_params phantom_hubp_post_enable_params;
	struct begin_cursor_offload_update_params begin_cursor_offload_update_params;
	struct update_cursor_offload_pipe_params update_cursor_offload_pipe_params;
	struct commit_cursor_offload_update_params commit_cursor_offload_update_params;
	struct stream_enc_update_hdmi_info_packets_params stream_enc_update_hdmi_info_packets_params;
	struct hpo_frl_stream_enc_update_hdmi_info_packets_params hpo_frl_stream_enc_update_hdmi_info_packets_params;
	struct hpo_dp_stream_enc_update_dp_info_packets_sdp_line_num_params hpo_dp_stream_enc_update_dp_info_packets_sdp_line_num_params;
	struct hpo_dp_stream_enc_update_dp_info_packets_params hpo_dp_stream_enc_update_dp_info_packets_params;
	struct stream_enc_update_dp_info_packets_sdp_line_num_params stream_enc_update_dp_info_packets_sdp_line_num_params;
	struct stream_enc_update_dp_info_packets_params stream_enc_update_dp_info_packets_params;
	struct dsc_set_config_simple_params dsc_set_config_simple_params;
	struct stream_enc_dp_set_dsc_config_params stream_enc_dp_set_dsc_config_params;
	struct hpo_dp_stream_enc_dp_set_dsc_pps_info_packet_params hpo_dp_stream_enc_dp_set_dsc_pps_info_packet_params;
	struct stream_enc_dp_set_dsc_pps_info_packet_params stream_enc_dp_set_dsc_pps_info_packet_params;
	struct hpo_frl_stream_enc_set_dsc_config_params hpo_frl_stream_enc_set_dsc_config_params;
	struct dp_trace_source_sequence_params dp_trace_source_sequence_params;
	struct set_dmdata_attributes_params set_dmdata_attributes_params;
	struct link_increase_mst_payload_params link_increase_mst_payload_params;
	struct link_reduce_mst_payload_params link_reduce_mst_payload_params;
	struct dp_set_test_pattern_params dp_set_test_pattern_params;
	struct link_set_dpms_off_params link_set_dpms_off_params;
	struct disable_audio_stream_params disable_audio_stream_params;
	struct prepare_bandwidth_params prepare_bandwidth_params;
	struct link_set_dpms_on_params link_set_dpms_on_params;
	struct clk_mgr_set_max_memclk_params clk_mgr_set_max_memclk_params;
	struct clk_mgr_update_clocks_params clk_mgr_update_clocks_params;
	struct hubbub_program_watermarks_params hubbub_program_watermarks_params;
	struct hubbub_program_arbiter_params hubbub_program_arbiter_params;
	struct hubbub_program_compbuf_segments_params hubbub_program_compbuf_segments_params;
};

#[repr(C)]
pub enum block_sequence_func {
	DMUB_SUBVP_PIPE_CONTROL_LOCK_FAST = 0,
	OPTC_PIPE_CONTROL_LOCK,
	HUBP_SET_FLIP_CONTROL_GSL,
	HUBP_PROGRAM_TRIPLEBUFFER,
	HUBP_UPDATE_PLANE_ADDR,
	DPP_SET_INPUT_TRANSFER_FUNC,
	DPP_PROGRAM_GAMUT_REMAP,
	HUBP_ENABLE_3DLUT_FL,
	OTG_SETUP_VERTICAL_INTERRUPT,
	HWSS_SETUP_PERIODIC_INTERRUPT,
	HWSS_UPDATE_INFO_FRAME,
	HUBP_SET_DMDATA_ATTRIBUTES,
	OPTC_PROGRAM_MANUAL_TRIGGER,
	DMUB_SEND_DMCUB_CMD,
	DPP_SETUP_DPP,
	DPP_PROGRAM_BIAS_AND_SCALE,
	DPP_SET_OUTPUT_TRANSFER_FUNC,
	DPP_SET_HDR_MULTIPLIER,
	DPP_PROGRAM_UPSP,
	MPC_UPDATE_VISUAL_CONFIRM,
	MPC_POWER_ON_MPC_MEM_PWR,
	MPC_SET_OUTPUT_CSC,
	MPC_SET_OCSC_DEFAULT,
	DMUB_SUBVP_SAVE_SURF_ADDR,
	HUBP_WAIT_FOR_DCC_META_PROP,
	DMUB_HW_CONTROL_LOCK_FAST,
	HUBP_PROGRAM_SURFACE_CONFIG,
	HUBP_PROGRAM_MCACHE_ID,
	DPP_PROGRAM_CM_HIST,
	PROGRAM_CURSOR_UPDATE_NOW,
	HUBP_WAIT_PIPE_READ_START,
	HWS_APPLY_UPDATE_FLAGS_FOR_PHANTOM,
	HWS_UPDATE_PHANTOM_VP_POSITION,
	OPTC_SET_ODM_COMBINE,
	OPTC_SET_ODM_BYPASS,
	OPP_PIPE_CLOCK_CONTROL,
	OPP_PROGRAM_LEFT_EDGE_EXTRA_PIXEL,
	DCCG_SET_DTO_DSCCLK,
	DSC_SET_CONFIG,
	DSC_ENABLE,
	TG_SET_DSC_CONFIG,
	DSC_DISCONNECT,
	DSC_READ_STATE,
	DSC_CALCULATE_AND_SET_CONFIG,
	DSC_ENABLE_WITH_OPP,
	TG_PROGRAM_GLOBAL_SYNC,
	TG_WAIT_FOR_STATE,
	TG_SET_VTG_PARAMS,
	TG_SETUP_VERTICAL_INTERRUPT2,
	HUBP_PROGRAM_DET_SIZE,
	HUBP_PROGRAM_DET_SEGMENTS,
	OPP_SET_DYN_EXPANSION,
	OPP_PROGRAM_FMT,
	OPP_PROGRAM_BIT_DEPTH_REDUCTION,
	OPP_SET_DISP_PATTERN_GENERATOR,
	ABM_SET_PIPE,
	ABM_SET_LEVEL,
	ABM_SET_IMMEDIATE_DISABLE,
	MPC_REMOVE_MPCC,
	OPP_SET_MPCC_DISCONNECT_PENDING,
	DC_SET_OPTIMIZED_REQUIRED,
	HUBP_DISCONNECT,
	HUBBUB_FORCE_PSTATE_CHANGE_CONTROL,
	TG_ENABLE_CRTC,
	TG_SET_GSL,
	TG_SET_GSL_SOURCE_SELECT,
	HUBP_WAIT_FLIP_PENDING,
	TG_WAIT_DOUBLE_BUFFER_PENDING,
	UPDATE_FORCE_PSTATE,
	PROGRAM_MALL_PIPE_CONFIG,
	HUBBUB_APPLY_DEDCN21_147_WA,
	HUBBUB_ALLOW_SELF_REFRESH_CONTROL,
	TG_GET_FRAME_COUNT,
	MPC_SET_DWB_MUX,
	MPC_DISABLE_DWB_MUX,
	MCIF_WB_CONFIG_BUF,
	MCIF_WB_CONFIG_ARB,
	MCIF_WB_ENABLE,
	MCIF_WB_DISABLE,
	DWBC_ENABLE,
	DWBC_DISABLE,
	DWBC_UPDATE,
	HUBP_UPDATE_MALL_SEL,
	HUBP_PREPARE_SUBVP_BUFFERING,
	HUBP_SET_BLANK_EN,
	HUBP_DISABLE_CONTROL,
	HUBBUB_SOFT_RESET,
	HUBP_CLK_CNTL,
	HUBP_INIT,
	HUBP_SET_VM_SYSTEM_APERTURE_SETTINGS,
	HUBP_SET_FLIP_INT,
	DPP_DPPCLK_CONTROL,
	DISABLE_PHANTOM_CRTC,
	DSC_PG_STATUS,
	DSC_WAIT_DISCONNECT_PENDING_CLEAR,
	DSC_DISABLE,
	DCCG_SET_REF_DSCCLK,
	DPP_PG_CONTROL,
	HUBP_PG_CONTROL,
	HUBP_RESET,
	DPP_RESET,
	DPP_ROOT_CLOCK_CONTROL,
	DC_IP_REQUEST_CNTL,
	DCCG_UPDATE_DPP_DTO,
	HUBP_VTG_SEL,
	HUBP_SETUP2,
	HUBP_SETUP,
	HUBP_SET_UNBOUNDED_REQUESTING,
	HUBP_SETUP_INTERDEPENDENT2,
	HUBP_SETUP_INTERDEPENDENT,
	DPP_SET_CURSOR_MATRIX,
	MPC_UPDATE_BLENDING,
	MPC_ASSERT_IDLE_MPCC,
	MPC_INSERT_PLANE,
	DPP_SET_SCALER,
	HUBP_MEM_PROGRAM_VIEWPORT,
	ABORT_CURSOR_OFFLOAD_UPDATE,
	HWSS_CURSOR_LOCK,
	HWSS_BEGIN_CURSOR_OFFLOAD_UPDATE,
	HWSS_COMMIT_CURSOR_OFFLOAD_UPDATE,
	HWSS_UPDATE_CURSOR_OFFLOAD_PIPE,
	DC_SEND_CURSOR_INFO_TO_DMU,
	SET_CURSOR_ATTRIBUTE,
	HUBP_SET_CURSOR_ATTRIBUTES,
	DPP_SET_CURSOR_ATTRIBUTES,
	SET_CURSOR_POSITION,
	SET_CURSOR_SDR_WHITE_LEVEL,
	PROGRAM_OUTPUT_CSC,
	HUBP_SET_LEGACY_TILING_COMPAT_LEVEL,
	HUBP_SET_BLANK,
	PHANTOM_HUBP_POST_ENABLE,
	STREAM_ENC_UPDATE_HDMI_INFO_PACKETS,
	HPO_FRL_STREAM_ENC_UPDATE_HDMI_INFO_PACKETS,
	HPO_DP_STREAM_ENC_UPDATE_DP_INFO_PACKETS_SDP_LINE_NUM,
	HPO_DP_STREAM_ENC_UPDATE_DP_INFO_PACKETS,
	STREAM_ENC_UPDATE_DP_INFO_PACKETS_SDP_LINE_NUM,
	STREAM_ENC_UPDATE_DP_INFO_PACKETS,
	DSC_SET_CONFIG_SIMPLE,
	STREAM_ENC_DP_SET_DSC_CONFIG,
	HPO_DP_STREAM_ENC_DP_SET_DSC_PPS_INFO_PACKET,
	STREAM_ENC_DP_SET_DSC_PPS_INFO_PACKET,
	HPO_FRL_STREAM_ENC_SET_DSC_CONFIG,
	LINK_INCREASE_MST_PAYLOAD,
	LINK_REDUCE_MST_PAYLOAD,
	DP_TRACE_SOURCE_SEQUENCE,
	DP_SET_TEST_PATTERN,
	LINK_SET_DPMS_OFF,
	DISABLE_AUDIO_STREAM,
	PREPARE_BANDWIDTH,
	LINK_SET_DPMS_ON,
	CLK_MGR_SET_MAX_MEMCLK,
	CLK_MGR_UPDATE_CLOCKS,
	HUBBUB_PROGRAM_WATERMARKS,
	HUBBUB_PROGRAM_ARBITER,
	HUBBUB_PROGRAM_COMPBUF_SEGMENTS,
	HUBBUB_PERFMON_RESET,
	HUBBUB_PERFMON_ARM_OUT_OF_ORDER_BW,
	HUBBUB_PERFMON_START_OUT_OF_ORDER_BW,
	HUBBUB_PERFMON_START_IN_ORDER_BW,
	HUBBUB_PERFMON_START_MEMORY_LATENCIES,
	HUBBUB_PERFMON_START_URGENT_ASSERTION_COUNT,
	HUBBUB_PERFMON_START_URGENT_RAMP_LATENCY,
	HUBBUB_PERFMON_START_PREFETCH_DATA_SIZE,
	HUBBUB_PERFMON_GET_OUT_OF_ORDER_BW,
	HUBBUB_PERFMON_GET_IN_ORDER_BW,
	HUBBUB_PERFMON_GET_MEMORY_LATENCIES,
	HUBBUB_PERFMON_GET_URGENT_ASSERTION_COUNT,
	HUBBUB_PERFMON_GET_PREFETCH_DATA_SIZE,
	HUBBUB_PERFMON_GET_URGENT_RAMP_LATENCY,
	/* This must be the last value in this enum, add new ones above */
	HWSS_BLOCK_SEQUENCE_FUNC_COUNT
};

#[repr(C)]
pub struct block_sequence {
	union block_sequence_params params;
	enum block_sequence_func func;
};

#[repr(C)]
pub struct block_sequence_state {
	*mut block_sequencesteps;
	u32 *num_steps;
};

pub const MAX_HWSS_BLOCK_SEQUENCE_SIZE: usize = HWSS_BLOCK_SEQUENCE_FUNC_COUNT * MAX_PIPES;

#[repr(C)]
pub struct hw_sequencer_funcs {
	void (*hardware_release)(*mut dcdc);
	/* Embedded Display Related */
	void (*edp_power_control)(*mut dc_linklink, bool enable);
	void (*edp_wait_for_hpd_ready)(*mut dc_linklink, bool power_up);
	void (*edp_wait_for_T12)(*mut dc_linklink);

	/* Pipe Programming Related */
	void (*init_hw)(*mut dcdc);
	void (*power_down_on_boot)(*mut dcdc);
	void (*enable_accelerated_mode)(*mut dcdc,
			*mut dc_statecontext);
	enum dc_status (*apply_ctx_to_hw)(*mut dcdc,
			*mut dc_statecontext);
	void (*disable_plane)(*mut dcdc, *mut dc_statestate, *mut pipe_ctxpipe_ctx);
	void (*disable_plane_sequence)(*mut dcdc, *mut dc_statestate, *mut pipe_ctxpipe_ctx,
		*mut block_sequence_stateseq_state);
	void (*disable_pixel_data)(*mut dcdc, *mut pipe_ctxpipe_ctx, bool blank);
	void (*apply_ctx_for_surface)(*mut dcdc,
			const *mut dc_stream_statestream,
			int num_planes, *mut dc_statecontext);
	void (*program_front_end_for_ctx)(*mut dcdc,
			*mut dc_statecontext);
	void (*wait_for_pending_cleared)(*mut dcdc,
			*mut dc_statecontext);
	void (*post_unlock_program_front_end)(*mut dcdc,
			*mut dc_statecontext);
	void (*update_plane_addr)(const *mut dcdc,
			*mut pipe_ctxpipe_ctx);
	void (*update_dchub)(*mut dce_hwseqhws,
			*mut dchub_init_datadh_data);
	void (*wait_for_mpcc_disconnect)(*mut dcdc,
			*mut resource_poolres_pool,
			*mut pipe_ctxpipe_ctx);
	void (*wait_for_mpcc_disconnect_sequence)(*mut dcdc,
			*mut resource_poolres_pool,
			*mut pipe_ctxpipe_ctx,
			*mut block_sequence_stateseq_state);
	void (*edp_backlight_control)(
			*mut dc_linklink,
			bool enable);
	void (*program_triplebuffer)(const *mut dcdc,
		*mut pipe_ctxpipe_ctx, bool enableTripleBuffer);
	void (*update_pending_status)(*mut pipe_ctxpipe_ctx);
	void (*update_dsc_pg)(*mut dcdc, *mut dc_statecontext, bool safe_to_disable);
	void (*clear_surface_dcc_and_tiling)(*mut pipe_ctxpipe_ctx, *mut dc_plane_stateplane_state, bool clear_tiling);

	/* Pipe Lock Related */
	void (*pipe_control_lock)(*mut dcdc,
			*mut pipe_ctxpipe, bool lock);
	void (*interdependent_update_lock)(*mut dcdc,
			*mut dc_statecontext, bool lock);
	void (*set_flip_control_gsl)(*mut pipe_ctxpipe_ctx,
			bool flip_immediate);
	void (*cursor_lock)(*mut dcdc, *mut pipe_ctxpipe, bool lock);

	/* Timing Related */
	void (*get_position)(*mut pipe_ctx*pipe_ctx, int num_pipes,
			*mut crtc_positionposition);
	int (*get_vupdate_offset_from_vsync)(*mut pipe_ctxpipe_ctx);
	void (*calc_vupdate_position)(
			*mut dcdc,
			*mut pipe_ctxpipe_ctx,
			u32 *start_line,
			u32 *end_line);
	void (*enable_per_frame_crtc_position_reset)(*mut dcdc,
			int group_size, *mut pipe_ctxgrouped_pipes[]);
	void (*enable_timing_synchronization)(*mut dcdc,
			*mut dc_statestate,
			int group_index, int group_size,
			*mut pipe_ctxgrouped_pipes[]);
	void (*enable_vblanks_synchronization)(*mut dcdc,
			int group_index, int group_size,
			*mut pipe_ctxgrouped_pipes[]);
	void (*setup_periodic_interrupt)(*mut dcdc,
			*mut pipe_ctxpipe_ctx);
	void (*set_drr)(*mut pipe_ctx*pipe_ctx, int num_pipes,
			struct dc_crtc_timing_adjust adjust);
	void (*set_static_screen_control)(*mut pipe_ctx*pipe_ctx,
			int num_pipes,
			const *mut dc_static_screen_paramsevents);

	/* Stream Related */
	void (*enable_stream)(*mut pipe_ctxpipe_ctx);
	void (*disable_stream)(*mut pipe_ctxpipe_ctx);
	void (*blank_stream)(*mut pipe_ctxpipe_ctx);
	void (*unblank_stream)(*mut pipe_ctxpipe_ctx,
			*mut dc_link_settingslink_settings);

	/* Bandwidth Related */
	void (*prepare_bandwidth)(*mut dcdc, *mut dc_statecontext);
	void (*prepare_bandwidth_sequence)(*mut dcdc,
			*mut dc_statecontext,
			*mut block_sequence_stateseq_state);
	bool (*update_bandwidth)(*mut dcdc, *mut dc_statecontext);
	void (*optimize_bandwidth)(*mut dcdc, *mut dc_statecontext);
	void (*optimize_bandwidth_sequence)(*mut dcdc,
			*mut dc_statecontext,
			*mut block_sequence_stateseq_state);

	/* Infopacket Related */
	void (*set_avmute)(*mut pipe_ctxpipe_ctx, bool enable);
	void (*send_immediate_sdp_message)(
			*mut pipe_ctxpipe_ctx,
			const u8 *custom_sdp_message,
			u32 sdp_message_size);
	void (*update_info_frame)(*mut pipe_ctxpipe_ctx);
	void (*set_dmdata_attributes)(*mut pipe_ctxpipe);
	void (*program_dmdata_engine)(*mut pipe_ctxpipe_ctx);
	bool (*dmdata_status_done)(*mut pipe_ctxpipe_ctx);

	/* Cursor Related */
	void (*set_cursor_position)(*mut pipe_ctxpipe);
	void (*set_cursor_attribute)(*mut pipe_ctxpipe);
	void (*set_cursor_sdr_white_level)(*mut pipe_ctxpipe);
	void (*abort_cursor_offload_update)(*mut dcdc, const *mut pipe_ctxpipe);
	void (*begin_cursor_offload_update)(*mut dcdc, const *mut pipe_ctxpipe);
	void (*commit_cursor_offload_update)(*mut dcdc, const *mut pipe_ctxpipe);
	void (*update_cursor_offload_pipe)(*mut dcdc, const *mut pipe_ctxpipe);
	void (*notify_cursor_offload_drr_update)(*mut dcdc, *mut dc_statecontext,
						 const *mut dc_stream_statestream);
	void (*program_cursor_offload_now)(*mut dcdc, const *mut pipe_ctxpipe);

	/* Colour Related */
	void (*program_gamut_remap)(*mut program_gamut_remap_paramsparams);
	void (*program_output_csc)(*mut dcdc, *mut pipe_ctxpipe_ctx,
			enum dc_color_space colorspace,
			u16 *matrix, int opp_id);
	void (*trigger_3dlut_dma_load)(*mut pipe_ctxpipe_ctx);

	/* VM Related */
	int (*init_sys_ctx)(*mut dce_hwseqhws,
			*mut dcdc,
			*mut dc_phy_addr_space_configpa_config);
	void (*init_vm_ctx)(*mut dce_hwseqhws,
			*mut dcdc,
			*mut dc_virtual_addr_space_configva_config,
			int vmid);

	/* Writeback Related */
	void (*update_writeback)(*mut dcdc,
			*mut dc_writeback_infowb_info,
			*mut dc_statecontext);
	void (*enable_writeback)(*mut dcdc,
			*mut dc_writeback_infowb_info,
			*mut dc_statecontext);
	void (*disable_writeback)(*mut dcdc,
			u32 dwb_pipe_inst);

	/* Clock Related */
	enum dc_status (*set_clock)(*mut dcdc,
			enum dc_clock_type clock_type,
			u32 clk_khz, u32 stepping);
	void (*get_clock)(*mut dcdc, enum dc_clock_type clock_type,
			*mut dc_clock_configclock_cfg);
	void (*optimize_pwr_state)(const *mut dcdc,
			*mut dc_statecontext);
	void (*exit_optimized_pwr_state)(const *mut dcdc,
			*mut dc_statecontext);
	void (*calculate_pix_rate_divider)(*mut dcdc,
			*mut dc_statecontext,
			const *mut dc_stream_statestream);

	/* Audio Related */
	void (*enable_audio_stream)(*mut pipe_ctxpipe_ctx);
	void (*disable_audio_stream)(*mut pipe_ctxpipe_ctx);

	/* Stereo 3D Related */
	void (*setup_stereo)(*mut pipe_ctxpipe_ctx, *mut dcdc);

	/* HW State Logging Related */
	void (*log_hw_state)(*mut dcdc, *mut dc_log_buffer_ctxlog_ctx);
	void (*log_color_state)(*mut dcdc,
				*mut dc_log_buffer_ctxlog_ctx);
	void (*get_hw_state)(*mut dcdc, char *pBuf,
			u32 bufSize, u32 mask);
	void (*clear_status_bits)(*mut dcdc, u32 mask);

	bool (*set_backlight_level)(*mut pipe_ctxpipe_ctx,
		*mut set_backlight_level_paramsparams);

	void (*set_abm_immediate_disable)(*mut pipe_ctxpipe_ctx);

	void (*set_pipe)(*mut pipe_ctxpipe_ctx);

	void (*enable_dp_link_output)(*mut dc_linklink,
			const *mut link_resourcelink_res,
			enum signal_type signal,
			enum clock_source_id clock_source,
			const *mut dc_link_settingslink_settings);
	void (*enable_tmds_link_output)(*mut dc_linklink,
			const *mut link_resourcelink_res,
			enum signal_type signal,
			enum clock_source_id clock_source,
			enum dc_color_depth color_depth,
			u32 pixel_clock);
	void (*enable_lvds_link_output)(*mut dc_linklink,
			const *mut link_resourcelink_res,
			enum clock_source_id clock_source,
			u32 pixel_clock);
	void (*enable_analog_link_output)(*mut dc_linklink,
			u32 pixel_clock);
	void (*disable_link_output)(*mut dc_linklink,
			const *mut link_resourcelink_res,
			enum signal_type signal);
	bool (*dac_load_detect)(*mut dc_linklink);
	void (*prepare_ddc)(*mut dc_linklink);

	void (*get_dcc_en_bits)(*mut dcdc, int *dcc_en_bits);

	enum dc_status (*setup_hdmi_frl_link)(
			*mut dc_linklink,
			int hpo_inst,
			enum clock_source_id frl_phy_clock_source_id);

	u32 (*get_max_dispclk_mhz)(*mut dcdc,
			*mut dc_statecontext);

	/* Idle Optimization Related */
	bool (*apply_idle_power_optimizations)(*mut dcdc, bool enable);

	bool (*does_plane_fit_in_mall)(*mut dcdc,
			u32 pitch,
			u32 height,
			enum surface_pixel_format format,
			*mut dc_cursor_attributescursor_attr);
	void (*commit_subvp_config)(*mut dcdc, *mut dc_statecontext);
	void (*enable_phantom_streams)(*mut dcdc, *mut dc_statecontext);
	void (*disable_phantom_streams)(*mut dcdc, *mut dc_statecontext);
	void (*subvp_pipe_control_lock)(*mut dcdc,
			*mut dc_statecontext,
			bool lock,
			bool should_lock_all_pipes,
			*mut pipe_ctxtop_pipe_to_program,
			bool subvp_prev_use);
	void (*subvp_pipe_control_lock_fast)(*mut block_sequence_paramsparams);

	void (*z10_restore)(const *mut dcdc);
	void (*z10_save_init)(*mut dcdc);
	bool (*is_abm_supported)(*mut dcdc,
			*mut dc_statecontext, *mut dc_stream_statestream);

	void (*set_disp_pattern_generator)(const *mut dcdc,
			*mut pipe_ctxpipe_ctx,
			enum controller_dp_test_pattern test_pattern,
			enum controller_dp_color_space color_space,
			enum dc_color_depth color_depth,
			const *mut tg_colorsolid_color,
			int width, int height, int offset);
	void (*blank_phantom)(*mut dcdc,
			*mut timing_generatortg,
			int width,
			int height);
	void (*update_visual_confirm_color)(*mut dcdc,
			*mut pipe_ctxpipe_ctx,
			int mpcc_id);
	void (*update_phantom_vp_position)(*mut dcdc,
			*mut dc_statecontext,
			*mut pipe_ctxphantom_pipe);
	void (*apply_update_flags_for_phantom)(*mut pipe_ctxphantom_pipe);

	void (*calc_blocks_to_gate)(*mut dcdc, *mut dc_statecontext,
		*mut pg_block_updateupdate_state);
	void (*calc_blocks_to_ungate)(*mut dcdc, *mut dc_statecontext,
		*mut pg_block_updateupdate_state);
	void (*hw_block_power_up)(*mut dcdc,
		*mut pg_block_updateupdate_state);
	void (*hw_block_power_down)(*mut dcdc,
		*mut pg_block_updateupdate_state);
	void (*root_clock_control)(*mut dcdc,
		*mut pg_block_updateupdate_state, bool power_on);
	bool (*is_pipe_topology_transition_seamless)(*mut dcdc,
			const *mut dc_statecur_ctx,
			const *mut dc_statenew_ctx);
	void (*wait_for_dcc_meta_propagation)(const *mut dcdc,
		const *mut pipe_ctxtop_pipe_to_program);
	void (*dmub_hw_control_lock)(*mut dcdc,
			*mut dc_statecontext,
			bool lock);
	void (*fams2_update_config)(*mut dcdc,
			*mut dc_statecontext,
			bool enable);
	void (*dmub_hw_control_lock_fast)(*mut block_sequence_paramsparams);
	void (*set_long_vtotal)(*mut pipe_ctx*pipe_ctx, int num_pipes, u32 v_total_min, u32 v_total_max);
	void (*program_outstanding_updates)(*mut dcdc,
			*mut dc_statecontext);
	void (*setup_hpo_hw_control)(const *mut dce_hwseqhws, bool enable);
	void (*wait_for_all_pending_updates)(const *mut pipe_ctxpipe_ctx);
	void (*detect_pipe_changes)(*mut dc_stateold_state,
			*mut dc_statenew_state,
			*mut pipe_ctxold_pipe,
			*mut pipe_ctxnew_pipe);
	void (*enable_plane)(*mut dcdc,
			*mut pipe_ctxpipe_ctx,
			*mut dc_statecontext);
	void (*enable_plane_sequence)(*mut dcdc,
			*mut pipe_ctxpipe_ctx,
			*mut dc_statecontext,
			*mut block_sequence_stateseq_state);
	void (*update_dchubp_dpp)(*mut dcdc,
			*mut pipe_ctxpipe_ctx,
			*mut dc_statecontext);
	void (*update_dchubp_dpp_sequence)(*mut dcdc,
			*mut pipe_ctxpipe_ctx,
			*mut dc_statecontext,
			*mut block_sequence_stateseq_state);
	void (*post_unlock_reset_opp)(*mut dcdc,
			*mut pipe_ctxopp_head);
	void (*post_unlock_reset_opp_sequence)(
			*mut dcdc,
			*mut pipe_ctxopp_head,
			*mut block_sequence_stateseq_state);
	void (*get_underflow_debug_data)(const *mut dcdc,
			*mut timing_generatortg,
			*mut dc_underflow_debug_dataout_data);

	/**
	 * program_perfmon - Program/transition perfmon probes for a commit.
	 * @dc:      DC structure
	 * @context: target state; probes, probe_count, and probe_status are
	 *           read from and written to this object
	 *
	 * Invoked during the execute phase of dc_update_state. The hook resolves
	 * each probe's transition by diffing @context against dc->current_state
	 * and latches MEASURED results into @context->probe_status.
	 */
	void (*program_perfmon)(*mut dcdc, *mut dc_statecontext);

};

void color_space_to_black_color(
	const *mut dcdc,
	enum dc_color_space colorspace,
	*mut tg_colorblack_color);

bool hwss_wait_for_blank_complete(
		*mut timing_generatortg);

const u16 *find_color_matrix(
		enum dc_color_space color_space,
		u32 *array_size);

void get_surface_tile_visual_confirm_color(
		*mut pipe_ctxpipe_ctx,
		*mut tg_colorcolor);
void get_surface_visual_confirm_color(
		const *mut pipe_ctxpipe_ctx,
		*mut tg_colorcolor);

void get_hdr_visual_confirm_color(
		*mut pipe_ctxpipe_ctx,
		*mut tg_colorcolor);
void get_mpctree_visual_confirm_color(
		*mut pipe_ctxpipe_ctx,
		*mut tg_colorcolor);
void get_smartmux_visual_confirm_color(
	*mut dcdc,
	*mut tg_colorcolor);
void get_vabc_visual_confirm_color(
	*mut pipe_ctxpipe_ctx,
	*mut tg_colorcolor);
void get_subvp_visual_confirm_color(
	*mut pipe_ctxpipe_ctx,
	*mut tg_colorcolor);
void get_fams2_visual_confirm_color(
	*mut dcdc,
	*mut dc_statecontext,
	*mut pipe_ctxpipe_ctx,
	*mut tg_colorcolor);

void get_mclk_switch_visual_confirm_color(
		*mut pipe_ctxpipe_ctx,
		*mut tg_colorcolor);

void get_cursor_visual_confirm_color(
		*mut pipe_ctxpipe_ctx,
		*mut tg_colorcolor);

void get_dcc_visual_confirm_color(
	*mut dcdc,
	*mut pipe_ctxpipe_ctx,
	*mut tg_colorcolor);

void get_refresh_rate_confirm_color(
		*mut pipe_ctxpipe_ctx,
		*mut tg_colorcolor);

void set_p_state_switch_method(
		*mut dcdc,
		*mut dc_statecontext,
		*mut pipe_ctxpipe_ctx);

void set_drr_and_clear_adjust_pending(
		*mut pipe_ctxpipe_ctx,
		*mut dc_stream_statestream,
		*mut drr_paramsparams);

void hwss_execute_sequence(*mut dcdc,
		struct block_sequence block_sequence[MAX_HWSS_BLOCK_SEQUENCE_SIZE],
		int num_steps);

void hwss_build_fast_sequence(*mut dcdc,
		*mut dc_dmub_cmddc_dmub_cmd,
		u32 dmub_cmd_count,
		struct block_sequence block_sequence[MAX_HWSS_BLOCK_SEQUENCE_SIZE],
		u32 *num_steps,
		*mut pipe_ctxpipe_ctx,
		*mut dc_stream_statusstream_status,
		*mut dc_statecontext);

void hwss_build_full_sequence(*mut dcdc,
	struct block_sequence block_sequence[MAX_HWSS_BLOCK_SEQUENCE_SIZE],
	u32 *num_steps,
	*mut dc_statecontext, bool program_phantom_pipe);

void hwss_build_post_unlock_full_sequence(*mut dcdc,
	struct block_sequence block_sequence[MAX_HWSS_BLOCK_SEQUENCE_SIZE],
	u32 *num_steps,
	*mut dc_statecontext);

void hwss_wait_for_all_blank_complete(*mut dcdc,
		*mut dc_statecontext);

void hwss_wait_for_odm_update_pending_complete(*mut dcdc,
		*mut dc_statecontext);

void hwss_wait_for_no_pipes_pending(*mut dcdc,
		*mut dc_statecontext);

void hwss_wait_for_outstanding_hw_updates(*mut dcdc,
		*mut dc_statedc_context);

void hwss_process_outstanding_hw_updates(*mut dcdc,
		*mut dc_statedc_context);

void hwss_send_dmcub_cmd(*mut block_sequence_paramsparams);

void hwss_program_manual_trigger(*mut block_sequence_paramsparams);

void hwss_setup_dpp(*mut block_sequence_paramsparams);

void hwss_program_bias_and_scale(*mut block_sequence_paramsparams);

void hwss_program_upsp(*mut block_sequence_paramsparams);

void hwss_power_on_mpc_mem_pwr(*mut block_sequence_paramsparams);

void hwss_set_output_csc(*mut block_sequence_paramsparams);

void hwss_set_ocsc_default(*mut block_sequence_paramsparams);

void hwss_subvp_save_surf_addr(*mut block_sequence_paramsparams);

void hwss_program_surface_config(*mut block_sequence_paramsparams);

void hwss_program_mcache_id_and_split_coordinate(*mut block_sequence_paramsparams);

void hwss_program_cm_hist(*mut block_sequence_paramsparams);

void hwss_set_odm_combine(*mut block_sequence_paramsparams);

void hwss_set_odm_bypass(*mut block_sequence_paramsparams);

void hwss_opp_pipe_clock_control(*mut block_sequence_paramsparams);

void hwss_opp_program_left_edge_extra_pixel(*mut block_sequence_paramsparams);

void hwss_blank_pixel_data(*mut block_sequence_paramsparams);

void hwss_dccg_set_dto_dscclk(*mut block_sequence_paramsparams);

void hwss_dsc_set_config(*mut block_sequence_paramsparams);

void hwss_dsc_enable(*mut block_sequence_paramsparams);

void hwss_tg_set_dsc_config(*mut block_sequence_paramsparams);

void hwss_dsc_disconnect(*mut block_sequence_paramsparams);

void hwss_dsc_read_state(*mut block_sequence_paramsparams);

void hwss_dsc_calculate_and_set_config(*mut block_sequence_paramsparams);

void hwss_dsc_enable_with_opp(*mut block_sequence_paramsparams);

void hwss_dsc_set_config_simple(*mut block_sequence_paramsparams);

void hwss_stream_enc_update_hdmi_info_packets(*mut block_sequence_paramsparams);

void hwss_hpo_frl_stream_enc_update_hdmi_info_packets(*mut block_sequence_paramsparams);

void hwss_hpo_dp_stream_enc_update_dp_info_packets_sdp_line_num(*mut block_sequence_paramsparams);

void hwss_hpo_dp_stream_enc_update_dp_info_packets(*mut block_sequence_paramsparams);

void hwss_stream_enc_update_dp_info_packets_sdp_line_num(*mut block_sequence_paramsparams);

void hwss_stream_enc_update_dp_info_packets(*mut block_sequence_paramsparams);

void hwss_stream_enc_dp_set_dsc_config(*mut block_sequence_paramsparams);

void hwss_hpo_dp_stream_enc_dp_set_dsc_pps_info_packet(*mut block_sequence_paramsparams);

void hwss_stream_enc_dp_set_dsc_pps_info_packet(*mut block_sequence_paramsparams);

void hwss_hpo_frl_stream_enc_set_dsc_config(*mut block_sequence_paramsparams);

void hwss_set_dmdata_attributes(*mut block_sequence_paramsparams);

void hwss_dp_trace_source_sequence(*mut block_sequence_paramsparams);

void hwss_link_increase_mst_payload(*mut block_sequence_paramsparams);

void hwss_link_reduce_mst_payload(*mut block_sequence_paramsparams);

void hwss_dp_set_test_pattern(*mut block_sequence_paramsparams);

void hwss_link_set_dpms_off(*mut block_sequence_paramsparams);

void hwss_prepare_bandwidth(*mut dcdc, *mut block_sequence_paramsparams);

void hwss_link_set_dpms_on(*mut block_sequence_paramsparams);

void hwss_program_tg(*mut block_sequence_paramsparams);

void hwss_tg_program_global_sync(*mut block_sequence_paramsparams);

void hwss_tg_wait_for_state(*mut block_sequence_paramsparams);

void hwss_tg_set_vtg_params(*mut block_sequence_paramsparams);

void hwss_hubp_enable_3dlut_fl(*mut block_sequence_paramsparams);

void hwss_update_info_frame(*mut dcdc, *mut block_sequence_paramsparams);

void hwss_tg_setup_vertical_interrupt0(*mut block_sequence_paramsparams);

void hwss_tg_setup_vertical_interrupt2(*mut block_sequence_paramsparams);

void hwss_dpp_set_hdr_multiplier(*mut block_sequence_paramsparams);

void hwss_program_det_size(*mut block_sequence_paramsparams);

void hwss_program_det_segments(*mut block_sequence_paramsparams);

void hwss_opp_set_dyn_expansion(*mut block_sequence_paramsparams);

void hwss_opp_program_fmt(*mut block_sequence_paramsparams);

void hwss_opp_program_bit_depth_reduction(*mut block_sequence_paramsparams);

void hwss_opp_set_disp_pattern_generator(*mut block_sequence_paramsparams);

void hwss_set_abm_pipe(*mut block_sequence_paramsparams);

void hwss_set_abm_level(*mut block_sequence_paramsparams);

void hwss_set_abm_immediate_disable(*mut block_sequence_paramsparams);

void hwss_mpc_remove_mpcc(*mut block_sequence_paramsparams);

void hwss_opp_set_mpcc_disconnect_pending(*mut block_sequence_paramsparams);

void hwss_dc_set_optimized_required(*mut block_sequence_paramsparams);

void hwss_hubp_disconnect(*mut block_sequence_paramsparams);

void hwss_hubbub_force_pstate_change_control(*mut block_sequence_paramsparams);

void hwss_tg_enable_crtc(*mut block_sequence_paramsparams);

void hwss_tg_set_gsl(*mut block_sequence_paramsparams);

void hwss_tg_set_gsl_source_select(*mut block_sequence_paramsparams);

void hwss_hubp_wait_flip_pending(*mut block_sequence_paramsparams);

void hwss_tg_wait_double_buffer_pending(*mut block_sequence_paramsparams);

void hwss_update_force_pstate(*mut block_sequence_paramsparams);

void hwss_hubbub_apply_dedcn21_147_wa(*mut block_sequence_paramsparams);

void hwss_hubbub_allow_self_refresh_control(*mut block_sequence_paramsparams);

void hwss_tg_get_frame_count(*mut block_sequence_paramsparams);

void hwss_mpc_set_dwb_mux(*mut block_sequence_paramsparams);

void hwss_mpc_disable_dwb_mux(*mut block_sequence_paramsparams);

void hwss_mcif_wb_config_buf(*mut block_sequence_paramsparams);

void hwss_mcif_wb_config_arb(*mut block_sequence_paramsparams);

void hwss_mcif_wb_enable(*mut block_sequence_paramsparams);

void hwss_mcif_wb_disable(*mut block_sequence_paramsparams);

void hwss_dwbc_enable(*mut block_sequence_paramsparams);

void hwss_dwbc_disable(*mut block_sequence_paramsparams);

void hwss_dwbc_update(*mut block_sequence_paramsparams);

void hwss_hubp_update_mall_sel(*mut block_sequence_paramsparams);

void hwss_hubp_prepare_subvp_buffering(*mut block_sequence_paramsparams);

void hwss_hubp_set_blank_en(*mut block_sequence_paramsparams);

void hwss_hubp_disable_control(*mut block_sequence_paramsparams);

void hwss_hubbub_soft_reset(*mut block_sequence_paramsparams);

void hwss_hubbub_perfmon_reset(*mut block_sequence_paramsparams);

void hwss_hubbub_perfmon_arm_out_of_order_bw(*mut block_sequence_paramsparams);

void hwss_hubbub_perfmon_start_out_of_order_bw(*mut block_sequence_paramsparams);

void hwss_hubbub_perfmon_start_in_order_bw(*mut block_sequence_paramsparams);

void hwss_hubbub_perfmon_start_memory_latencies(*mut block_sequence_paramsparams);

void hwss_hubbub_perfmon_start_urgent_assertion_count(*mut block_sequence_paramsparams);

void hwss_hubbub_perfmon_start_urgent_ramp_latency(*mut block_sequence_paramsparams);

void hwss_hubbub_perfmon_start_prefetch_data_size(*mut block_sequence_paramsparams);

void hwss_hubbub_perfmon_get_out_of_order_bw(*mut block_sequence_paramsparams);

void hwss_hubbub_perfmon_get_in_order_bw(*mut block_sequence_paramsparams);

void hwss_hubbub_perfmon_get_memory_latencies(*mut block_sequence_paramsparams);

void hwss_hubbub_perfmon_get_urgent_assertion_count(*mut block_sequence_paramsparams);

void hwss_hubbub_perfmon_get_prefetch_data_size(*mut block_sequence_paramsparams);

void hwss_hubbub_perfmon_get_urgent_ramp_latency(*mut block_sequence_paramsparams);

void hwss_hubp_clk_cntl(*mut block_sequence_paramsparams);

void hwss_hubp_init(*mut block_sequence_paramsparams);

void hwss_hubp_set_vm_system_aperture_settings(*mut block_sequence_paramsparams);

void hwss_hubp_set_flip_int(*mut block_sequence_paramsparams);

void hwss_dpp_dppclk_control(*mut block_sequence_paramsparams);

void hwss_disable_phantom_crtc(*mut block_sequence_paramsparams);

void hwss_dsc_pg_status(*mut block_sequence_paramsparams);

void hwss_dsc_wait_disconnect_pending_clear(*mut block_sequence_paramsparams);

void hwss_dsc_disable(*mut block_sequence_paramsparams);

void hwss_dccg_set_ref_dscclk(*mut block_sequence_paramsparams);

void hwss_dpp_pg_control(*mut block_sequence_paramsparams);

void hwss_hubp_pg_control(*mut block_sequence_paramsparams);

void hwss_hubp_reset(*mut block_sequence_paramsparams);

void hwss_dpp_reset(*mut block_sequence_paramsparams);

void hwss_dpp_root_clock_control(*mut block_sequence_paramsparams);

void hwss_dc_ip_request_cntl(*mut block_sequence_paramsparams);

void hwss_dccg_update_dpp_dto(*mut block_sequence_paramsparams);

void hwss_hubp_vtg_sel(*mut block_sequence_paramsparams);

void hwss_hubp_setup2(*mut block_sequence_paramsparams);

void hwss_hubp_setup(*mut block_sequence_paramsparams);

void hwss_hubp_set_unbounded_requesting(*mut block_sequence_paramsparams);

void hwss_hubp_setup_interdependent2(*mut block_sequence_paramsparams);

void hwss_hubp_setup_interdependent(*mut block_sequence_paramsparams);

void hwss_dpp_set_cursor_matrix(*mut block_sequence_paramsparams);

void hwss_mpc_update_mpcc(*mut block_sequence_paramsparams);

void hwss_mpc_update_blending(*mut block_sequence_paramsparams);

void hwss_mpc_assert_idle_mpcc(*mut block_sequence_paramsparams);

void hwss_mpc_insert_plane(*mut block_sequence_paramsparams);

void hwss_dpp_set_scaler(*mut block_sequence_paramsparams);

void hwss_hubp_mem_program_viewport(*mut block_sequence_paramsparams);

void hwss_abort_cursor_offload_update(*mut block_sequence_paramsparams);

void hwss_send_cursor_info_to_dmu(*mut block_sequence_paramsparams);

void hwss_set_cursor_attribute(*mut block_sequence_paramsparams);

void hwss_hubp_set_cursor_attributes(*mut block_sequence_paramsparams);

void hwss_dpp_set_cursor_attributes(*mut block_sequence_paramsparams);

void hwss_set_cursor_position(*mut block_sequence_paramsparams);

void hwss_set_cursor_sdr_white_level(*mut block_sequence_paramsparams);

void hwss_program_gamut_remap(*mut pipe_ctxpipe_ctx);

void hwss_program_output_csc(*mut block_sequence_paramsparams);

void hwss_hubp_set_legacy_tiling_compat_level(*mut block_sequence_paramsparams);

void hwss_hubp_set_blank(*mut block_sequence_paramsparams);

void hwss_phantom_hubp_post_enable(*mut block_sequence_paramsparams);

void hwss_cursor_lock(*mut block_sequence_paramsparams);

void hwss_begin_cursor_offload_update(*mut block_sequence_paramsparams);

void hwss_commit_cursor_offload_update(*mut block_sequence_paramsparams);

void hwss_update_cursor_offload_pipe(*mut block_sequence_paramsparams);

void hwss_setup_periodic_interrupt(*mut dcdc, *mut block_sequence_paramsparams);

void hwss_disable_audio_stream(*mut dcdc, *mut block_sequence_paramsparams);

void hwss_add_optc_pipe_control_lock(*mut block_sequence_stateseq_state,
		*mut dcdc, *mut pipe_ctxpipe_ctx, bool lock);

void hwss_add_hubp_set_flip_control_gsl(*mut block_sequence_stateseq_state,
		*mut hubphubp, bool flip_immediate);

void hwss_add_hubp_program_triplebuffer(*mut block_sequence_stateseq_state,
		*mut dcdc, *mut pipe_ctxpipe_ctx, bool enableTripleBuffer);

void hwss_add_hubp_update_plane_addr(*mut block_sequence_stateseq_state,
		*mut dcdc, *mut pipe_ctxpipe_ctx);

void hwss_add_dpp_set_input_transfer_func(*mut block_sequence_stateseq_state,
		*mut dcdc, *mut pipe_ctxpipe_ctx, *mut dc_plane_stateplane_state);

void hwss_add_dpp_program_gamut_remap(*mut block_sequence_stateseq_state,
		*mut pipe_ctxpipe_ctx);

void hwss_add_dpp_program_bias_and_scale(*mut block_sequence_stateseq_state,
		*mut pipe_ctxpipe_ctx);

void hwss_add_optc_program_manual_trigger(*mut block_sequence_stateseq_state,
		*mut pipe_ctxpipe_ctx);

void hwss_add_dpp_set_output_transfer_func(*mut block_sequence_stateseq_state,
		*mut dcdc, *mut pipe_ctxpipe_ctx);

void hwss_set_output_transfer_func(*mut dcdc, *mut pipe_ctxpipe_ctx);

void hwss_add_mpc_update_visual_confirm(*mut block_sequence_stateseq_state,
		*mut dcdc, *mut pipe_ctxpipe_ctx, int mpcc_id);

void hwss_add_mpc_power_on_mpc_mem_pwr(*mut block_sequence_stateseq_state,
		*mut mpcmpc, int mpcc_id, bool power_on);

void hwss_add_mpc_set_output_csc(*mut block_sequence_stateseq_state,
		*mut mpcmpc, int opp_id, const u16 *regval, enum mpc_output_csc_mode ocsc_mode);

void hwss_add_mpc_set_ocsc_default(*mut block_sequence_stateseq_state,
		*mut mpcmpc, int opp_id, enum dc_color_space colorspace, enum mpc_output_csc_mode ocsc_mode);

void hwss_add_dmub_send_dmcub_cmd(*mut block_sequence_stateseq_state,
		*mut dc_contextctx, *mut dmub_rb_cmdcmd, enum dm_dmub_wait_type wait_type);

void hwss_add_dmub_subvp_save_surf_addr(*mut block_sequence_stateseq_state,
		*mut dc_dmub_srvdc_dmub_srv, *mut dc_plane_addressaddr, u8 subvp_index);

void hwss_add_hubp_wait_for_dcc_meta_prop(*mut block_sequence_stateseq_state,
		*mut dcdc, *mut pipe_ctxtop_pipe_to_program);

void hwss_add_hubp_wait_pipe_read_start(*mut block_sequence_stateseq_state,
		*mut hubphubp);

void hwss_add_hws_apply_update_flags_for_phantom(*mut block_sequence_stateseq_state,
		*mut pipe_ctxpipe_ctx);

void hwss_add_hws_update_phantom_vp_position(*mut block_sequence_stateseq_state,
		*mut dcdc, *mut dc_statecontext, *mut pipe_ctxpipe_ctx);

void hwss_add_optc_set_odm_combine(*mut block_sequence_stateseq_state,
		*mut timing_generatortg, int opp_inst[MAX_PIPES], int opp_head_count,
		int odm_slice_width, int last_odm_slice_width);

void hwss_add_optc_set_odm_bypass(*mut block_sequence_stateseq_state,
		*mut timing_generatoroptc, *mut dc_crtc_timingtiming);

void hwss_add_tg_program_global_sync(*mut block_sequence_stateseq_state,
		*mut timing_generatortg,
		int vready_offset,
		u32 vstartup_lines,
		u32 vupdate_offset_pixels,
		u32 vupdate_vupdate_width_pixels,
		u32 pstate_keepout_start_lines);

void hwss_add_tg_wait_for_state(*mut block_sequence_stateseq_state,
		*mut timing_generatortg, enum crtc_state state);

void hwss_add_tg_set_vtg_params(*mut block_sequence_stateseq_state,
		*mut timing_generatortg, *mut dc_crtc_timingdc_crtc_timing, bool program_fp2);

void hwss_add_vertical_interrupt_setup(*mut block_sequence_stateseq_state,
		*mut timing_generatortg, u32 start_line, u32 end_line);

void hwss_add_tg_setup_vertical_interrupt2(*mut block_sequence_stateseq_state,
		*mut timing_generatortg, int start_line);

void hwss_add_dpp_set_hdr_multiplier(*mut block_sequence_stateseq_state,
		*mut dppdpp, u32 hw_mult);

void hwss_add_hubp_program_det_size(*mut block_sequence_stateseq_state,
		*mut hubbubhubbub, u32 hubp_inst, u32 det_buffer_size_kb);

void hwss_add_hubp_program_mcache_id(*mut block_sequence_stateseq_state,
		*mut hubphubp, *mut dml2_hubp_pipe_mcache_regsmcache_regs);

void hwss_add_hubbub_force_pstate_change_control(*mut block_sequence_stateseq_state,
		*mut hubbubhubbub, bool enable, bool wait);

void hwss_add_hubp_program_det_segments(*mut block_sequence_stateseq_state,
		*mut hubbubhubbub, u32 hubp_inst, u32 det_size);

void hwss_add_opp_set_dyn_expansion(*mut block_sequence_stateseq_state,
		*mut output_pixel_processoropp, enum dc_color_space color_sp,
		enum dc_color_depth color_dpth, enum signal_type signal);

void hwss_add_opp_program_fmt(*mut block_sequence_stateseq_state,
		*mut output_pixel_processoropp, *mut bit_depth_reduction_paramsfmt_bit_depth,
		*mut clamping_and_pixel_encoding_paramsclamping);

void hwss_add_abm_set_pipe(*mut block_sequence_stateseq_state,
		*mut dcdc, *mut pipe_ctxpipe_ctx);

void hwss_add_abm_set_level(*mut block_sequence_stateseq_state,
		*mut abmabm, u32 abm_level);

void hwss_add_tg_enable_crtc(*mut block_sequence_stateseq_state,
		*mut timing_generatortg);

void hwss_add_hubp_wait_flip_pending(*mut block_sequence_stateseq_state,
		*mut hubphubp, u32 timeout_us, u32 polling_interval_us);

void hwss_add_tg_wait_double_buffer_pending(*mut block_sequence_stateseq_state,
		*mut timing_generatortg, u32 timeout_us, u32 polling_interval_us);

void hwss_add_dccg_set_dto_dscclk(*mut block_sequence_stateseq_state,
		*mut dccgdccg, int inst, int num_slices_h);

void hwss_add_dsc_calculate_and_set_config(*mut block_sequence_stateseq_state,
		*mut pipe_ctxpipe_ctx, bool enable, int opp_cnt);

void hwss_add_mpc_remove_mpcc(*mut block_sequence_stateseq_state,
		*mut mpcmpc, *mut mpc_treempc_tree_params, *mut mpccmpcc_to_remove);

void hwss_add_opp_set_mpcc_disconnect_pending(*mut block_sequence_stateseq_state,
		*mut output_pixel_processoropp, int mpcc_inst, bool pending);

void hwss_add_hubp_disconnect(*mut block_sequence_stateseq_state,
		*mut hubphubp);

void hwss_add_dsc_enable_with_opp(*mut block_sequence_stateseq_state,
		*mut pipe_ctxpipe_ctx);

void hwss_add_dsc_disconnect(*mut block_sequence_stateseq_state,
		*mut display_stream_compressordsc);

void hwss_add_dc_set_optimized_required(*mut block_sequence_stateseq_state,
		*mut dcdc, bool optimized_required);

void hwss_add_abm_set_immediate_disable(*mut block_sequence_stateseq_state,
		*mut dcdc, *mut pipe_ctxpipe_ctx);

void hwss_add_opp_set_disp_pattern_generator(*mut block_sequence_stateseq_state,
		*mut output_pixel_processoropp,
		enum controller_dp_test_pattern test_pattern,
		enum controller_dp_color_space color_space,
		enum dc_color_depth color_depth,
		struct tg_color solid_color,
		bool use_solid_color,
		int width,
		int height,
		int offset);

void hwss_add_opp_program_bit_depth_reduction(*mut block_sequence_stateseq_state,
		*mut output_pixel_processoropp,
		bool use_default_params,
		*mut pipe_ctxpipe_ctx);

void hwss_add_dpp_program_cm_hist(*mut block_sequence_stateseq_state,
		*mut dppdpp,
		struct cm_hist_control cm_hist_control,
		enum dc_color_space color_space);

void hwss_add_dc_ip_request_cntl(*mut block_sequence_stateseq_state,
		*mut dcdc,
		bool enable);

void hwss_add_dwbc_update(*mut block_sequence_stateseq_state,
		*mut dwbcdwb,
		*mut dc_dwb_paramsdwb_params);

void hwss_add_mcif_wb_config_buf(*mut block_sequence_stateseq_state,
		*mut mcif_wbmcif_wb,
		*mut mcif_buf_paramsmcif_buf_params,
		u32 dest_height);

void hwss_add_mcif_wb_config_arb(*mut block_sequence_stateseq_state,
		*mut mcif_wbmcif_wb,
		*mut mcif_arb_paramsmcif_arb_params);

void hwss_add_mcif_wb_enable(*mut block_sequence_stateseq_state,
		*mut mcif_wbmcif_wb);

void hwss_add_mcif_wb_disable(*mut block_sequence_stateseq_state,
		*mut mcif_wbmcif_wb);

void hwss_add_mpc_set_dwb_mux(*mut block_sequence_stateseq_state,
		*mut mpcmpc,
		int dwb_id,
		int mpcc_id);

void hwss_add_mpc_disable_dwb_mux(*mut block_sequence_stateseq_state,
		*mut mpcmpc,
		u32 dwb_id);

void hwss_add_dwbc_enable(*mut block_sequence_stateseq_state,
		*mut dwbcdwb,
		*mut dc_dwb_paramsdwb_params);

void hwss_add_dwbc_disable(*mut block_sequence_stateseq_state,
		*mut dwbcdwb);

void hwss_add_tg_set_gsl(*mut block_sequence_stateseq_state,
		*mut timing_generatortg,
		struct gsl_params gsl);

void hwss_add_tg_set_gsl_source_select(*mut block_sequence_stateseq_state,
		*mut timing_generatortg,
		int group_idx,
		u32 gsl_ready_signal);

void hwss_add_hubp_update_mall_sel(*mut block_sequence_stateseq_state,
		*mut hubphubp,
		u32 mall_sel,
		bool cache_cursor);

void hwss_add_hubp_prepare_subvp_buffering(*mut block_sequence_stateseq_state,
		*mut hubphubp,
		bool enable);

void hwss_add_hubp_set_blank_en(*mut block_sequence_stateseq_state,
		*mut hubphubp,
		bool enable);

void hwss_add_hubp_disable_control(*mut block_sequence_stateseq_state,
		*mut hubphubp,
		bool disable);

void hwss_add_hubbub_soft_reset(*mut block_sequence_stateseq_state,
		*mut hubbubhubbub,
		void (*hubbub_soft_reset)(*mut hubbubhubbub, bool reset),
		bool reset);

void hwss_add_hubbub_perfmon_reset(*mut block_sequence_stateseq_state,
		*mut hubbubhubbub);

void hwss_add_hubbub_perfmon_arm_out_of_order_bw(*mut block_sequence_stateseq_state,
		*mut hubbubhubbub);

void hwss_add_hubbub_perfmon_start_out_of_order_bw(*mut block_sequence_stateseq_state,
		*mut hubbubhubbub);

void hwss_add_hubbub_perfmon_start_in_order_bw(*mut block_sequence_stateseq_state,
		*mut hubbubhubbub);

void hwss_add_hubbub_perfmon_start_memory_latencies(*mut block_sequence_stateseq_state,
		*mut hubbubhubbub);

void hwss_add_hubbub_perfmon_start_urgent_assertion_count(*mut block_sequence_stateseq_state,
		*mut hubbubhubbub);

void hwss_add_hubbub_perfmon_start_urgent_ramp_latency(*mut block_sequence_stateseq_state,
		*mut hubbubhubbub,
		const *mut hubbub_urgent_latency_paramslatency_params);

void hwss_add_hubbub_perfmon_start_prefetch_data_size(*mut block_sequence_stateseq_state,
		*mut hubbubhubbub);

void hwss_add_hubbub_perfmon_get_out_of_order_bw(*mut block_sequence_stateseq_state,
		*mut hubbubhubbub, u32 refclk_mhz,
		u32 *bandwidth_mbps, u32 *duration_ns);

void hwss_add_hubbub_perfmon_get_in_order_bw(*mut block_sequence_stateseq_state,
		*mut hubbubhubbub, u32 refclk_mhz, u32 min_duration_ns,
		u32 *bandwidth_mbps, u32 *duration_ns);

void hwss_add_hubbub_perfmon_get_memory_latencies(*mut block_sequence_stateseq_state,
		*mut hubbubhubbub, u32 refclk_mhz,
		*mut dc_probe_latenciesresult);

void hwss_add_hubbub_perfmon_get_urgent_assertion_count(*mut block_sequence_stateseq_state,
		*mut hubbubhubbub, u32 refclk_mhz,
		u32 *assertion_count);

void hwss_add_hubbub_perfmon_get_prefetch_data_size(*mut block_sequence_stateseq_state,
		*mut hubbubhubbub, u32 *prefetch_data_size);

void hwss_add_hubbub_perfmon_get_urgent_ramp_latency(*mut block_sequence_stateseq_state,
		*mut hubbubhubbub, u32 refclk_mhz,
		u32 *latency_ns);

void hwss_add_hubp_clk_cntl(*mut block_sequence_stateseq_state,
		*mut hubphubp,
		bool enable);

void hwss_add_dpp_dppclk_control(*mut block_sequence_stateseq_state,
		*mut dppdpp,
		bool dppclk_div,
		bool enable);

void hwss_add_disable_phantom_crtc(*mut block_sequence_stateseq_state,
		*mut timing_generatortg);

void hwss_add_dsc_pg_status(*mut block_sequence_stateseq_state,
		*mut dce_hwseqhws,
		int dsc_inst,
		bool is_ungated);

void hwss_add_dsc_wait_disconnect_pending_clear(*mut block_sequence_stateseq_state,
		*mut display_stream_compressordsc,
		bool *is_ungated);

void hwss_add_dsc_disable(*mut block_sequence_stateseq_state,
		*mut display_stream_compressordsc,
		bool *is_ungated);

void hwss_add_dccg_set_ref_dscclk(*mut block_sequence_stateseq_state,
		*mut dccgdccg,
		int dsc_inst,
		bool *is_ungated);

void hwss_add_dpp_root_clock_control(*mut block_sequence_stateseq_state,
		*mut dce_hwseqhws,
		u32 dpp_inst,
		bool clock_on);

void hwss_add_dpp_pg_control(*mut block_sequence_stateseq_state,
		*mut dce_hwseqhws,
		u32 dpp_inst,
		bool power_on);

void hwss_add_hubp_pg_control(*mut block_sequence_stateseq_state,
		*mut dce_hwseqhws,
		u32 hubp_inst,
		bool power_on);

void hwss_add_hubp_set_blank(*mut block_sequence_stateseq_state,
		*mut hubphubp,
		bool blank);

void hwss_add_hubp_init(*mut block_sequence_stateseq_state,
		*mut hubphubp);

void hwss_add_hubp_reset(*mut block_sequence_stateseq_state,
		*mut hubphubp);

void hwss_add_dpp_reset(*mut block_sequence_stateseq_state,
		*mut dppdpp);

void hwss_add_opp_pipe_clock_control(*mut block_sequence_stateseq_state,
		*mut output_pixel_processoropp,
		bool enable);

void hwss_add_hubp_set_vm_system_aperture_settings(*mut block_sequence_stateseq_state,
		*mut hubphubp,
		uint64_t sys_default,
		uint64_t sys_low,
		uint64_t sys_high);

void hwss_add_hubp_set_flip_int(*mut block_sequence_stateseq_state,
		*mut hubphubp);

void hwss_add_dccg_update_dpp_dto(*mut block_sequence_stateseq_state,
		*mut dccgdccg,
		int dpp_inst,
		int dppclk_khz);

void hwss_add_hubp_vtg_sel(*mut block_sequence_stateseq_state,
		*mut hubphubp,
		u32 otg_inst);

void hwss_add_hubp_setup2(*mut block_sequence_stateseq_state,
		*mut hubphubp,
		*mut dml2_dchub_per_pipe_register_sethubp_regs,
		*mut dml2_global_sync_programmingglobal_sync,
		*mut dc_crtc_timingtiming);

void hwss_add_hubp_setup(*mut block_sequence_stateseq_state,
		*mut hubphubp,
		*mut _vcs_dpi_display_dlg_regs_stdlg_regs,
		*mut _vcs_dpi_display_ttu_regs_stttu_regs,
		*mut _vcs_dpi_display_rq_regs_strq_regs,
		*mut _vcs_dpi_display_pipe_dest_params_stpipe_dest);

void hwss_add_hubp_set_unbounded_requesting(*mut block_sequence_stateseq_state,
		*mut hubphubp,
		bool unbounded_req);

void hwss_add_hubp_setup_interdependent2(*mut block_sequence_stateseq_state,
		*mut hubphubp,
		*mut dml2_dchub_per_pipe_register_sethubp_regs);

void hwss_add_hubp_setup_interdependent(*mut block_sequence_stateseq_state,
		*mut hubphubp,
		*mut _vcs_dpi_display_dlg_regs_stdlg_regs,
		*mut _vcs_dpi_display_ttu_regs_stttu_regs);
void hwss_add_hubp_program_surface_config(*mut block_sequence_stateseq_state,
		*mut hubphubp,
		enum surface_pixel_format format,
		*mut dc_tiling_infotiling_info,
		struct plane_size plane_size,
		enum dc_rotation_angle rotation,
		*mut dc_plane_dcc_paramdcc,
		bool horizontal_mirror,
		int compat_level);

void hwss_add_dpp_setup_dpp(*mut block_sequence_stateseq_state,
		*mut pipe_ctxpipe_ctx);

void hwss_add_dpp_set_cursor_matrix(*mut block_sequence_stateseq_state,
		*mut dppdpp,
		enum dc_color_space color_space,
		*mut dc_csc_transformcursor_csc_color_matrix);

void hwss_add_mpc_update_blending(*mut block_sequence_stateseq_state,
		*mut mpcmpc,
		struct mpcc_blnd_cfg blnd_cfg,
		int mpcc_id);

void hwss_add_mpc_assert_idle_mpcc(*mut block_sequence_stateseq_state,
		*mut mpcmpc,
		int mpcc_id);

void hwss_add_mpc_insert_plane(*mut block_sequence_stateseq_state,
		*mut mpcmpc,
		*mut mpc_treempc_tree_params,
		struct mpcc_blnd_cfg blnd_cfg,
		*mut mpcc_sm_cfgsm_cfg,
		*mut mpccinsert_above_mpcc,
		int dpp_id,
		int mpcc_id);

void hwss_add_dpp_set_scaler(*mut block_sequence_stateseq_state,
		*mut dppdpp,
		const *mut scaler_datascl_data);

void hwss_add_hubp_mem_program_viewport(*mut block_sequence_stateseq_state,
		*mut hubphubp,
		const *mut rectviewport,
		const *mut rectviewport_c);

void hwss_add_abort_cursor_offload_update(*mut block_sequence_stateseq_state,
		*mut dcdc,
		*mut pipe_ctxpipe_ctx);

void hwss_add_set_cursor_attribute(*mut block_sequence_stateseq_state,
		*mut dcdc,
		*mut pipe_ctxpipe_ctx);

void hwss_add_hubp_set_cursor_attributes(*mut block_sequence_stateseq_state,
		*mut hubphubp,
		const *mut dc_cursor_attributesattributes);

void hwss_add_dpp_set_cursor_attributes(*mut block_sequence_stateseq_state,
		*mut dppdpp,
		*mut dc_cursor_attributesattributes);

void hwss_add_set_cursor_position(*mut block_sequence_stateseq_state,
		*mut dcdc,
		*mut pipe_ctxpipe_ctx);

void hwss_add_set_cursor_sdr_white_level(*mut block_sequence_stateseq_state,
		*mut dcdc,
		*mut pipe_ctxpipe_ctx);

void hwss_add_program_output_csc(*mut block_sequence_stateseq_state,
		*mut dcdc,
		*mut pipe_ctxpipe_ctx,
		enum dc_color_space colorspace,
		u16 *matrix,
		int opp_id);

void hwss_add_phantom_hubp_post_enable(*mut block_sequence_stateseq_state,
		*mut hubphubp);

void hwss_add_update_force_pstate(*mut block_sequence_stateseq_state,
		*mut dcdc,
		*mut dc_statecontext);

void hwss_add_hubbub_apply_dedcn21_147_wa(*mut block_sequence_stateseq_state,
		*mut hubbubhubbub);

void hwss_add_hubbub_allow_self_refresh_control(*mut block_sequence_stateseq_state,
		*mut hubbubhubbub,
		bool allow,
		bool *disallow_self_refresh_applied);

void hwss_add_tg_get_frame_count(*mut block_sequence_stateseq_state,
		*mut timing_generatortg,
		u32 *frame_count);

void hwss_add_tg_set_dsc_config(*mut block_sequence_stateseq_state,
		*mut timing_generatortg,
		*mut dsc_optc_configdsc_optc_cfg,
		bool enable);

void hwss_add_opp_program_left_edge_extra_pixel(*mut block_sequence_stateseq_state,
		*mut output_pixel_processoropp,
		enum dc_pixel_encoding pixel_encoding,
		bool is_otg_master);

void hwss_add_hubp_enable_3dlut_fl(*mut block_sequence_stateseq_state,
		*mut hubphubp);

void hwss_add_begin_cursor_offload_update(*mut block_sequence_stateseq_state,
		*mut dcdc,
		*mut pipe_ctxpipe_ctx);

void hwss_add_cursor_lock(*mut block_sequence_stateseq_state,
		*mut dcdc,
		*mut pipe_ctxpipe_ctx,
		bool lock);

void hwss_add_send_update_cursor_info_to_dmu(*mut block_sequence_stateseq_state,
		*mut pipe_ctxpipe_ctx,
		int index);

void hwss_add_update_cursor_offload_pipe(*mut block_sequence_stateseq_state,
		*mut dcdc,
		*mut pipe_ctxpipe_ctx);

void hwss_add_commit_cursor_offload_update(*mut block_sequence_stateseq_state,
		*mut dcdc,
		*mut pipe_ctxpipe_ctx);

void hwss_add_stream_enc_update_hdmi_info_packets(*mut block_sequence_stateseq_state,
		*mut pipe_ctxpipe_ctx);

void hwss_add_hpo_frl_stream_enc_update_hdmi_info_packets(*mut block_sequence_stateseq_state,
		*mut pipe_ctxpipe_ctx);

void hwss_add_hpo_dp_stream_enc_update_dp_info_packets_sdp_line_num(*mut block_sequence_stateseq_state,
		*mut pipe_ctxpipe_ctx);

void hwss_add_hpo_dp_stream_enc_update_dp_info_packets(*mut block_sequence_stateseq_state,
		*mut pipe_ctxpipe_ctx);

void hwss_add_stream_enc_update_dp_info_packets_sdp_line_num(*mut block_sequence_stateseq_state,
		*mut pipe_ctxpipe_ctx);

void hwss_add_stream_enc_update_dp_info_packets(*mut block_sequence_stateseq_state,
		*mut pipe_ctxpipe_ctx);

void hwss_add_dsc_set_config(*mut block_sequence_stateseq_state,
		*mut display_stream_compressordsc,
		const *mut dsc_configdsc_cfg,
		const *mut dsc_optc_configdsc_optc_cfg);

void hwss_add_stream_enc_dp_set_dsc_config(*mut block_sequence_stateseq_state,
		*mut stream_encoderstream_enc,
		const *mut dsc_optc_configdsc_optc_cfg);

void hwss_add_hpo_dp_stream_enc_dp_set_dsc_pps_info_packet(*mut block_sequence_stateseq_state,
		*mut hpo_dp_stream_encoderhpo_dp_stream_enc,
		bool immediate_update,
		u8 *dsc_packed_pps,
		bool pps_sdp_stream);

void hwss_add_stream_enc_dp_set_dsc_pps_info_packet(*mut block_sequence_stateseq_state,
		*mut stream_encoderstream_enc,
		bool immediate_update,
		u8 *dsc_packed_pps,
		bool pps_sdp_stream);

void hwss_add_hpo_frl_stream_enc_set_dsc_config(*mut block_sequence_stateseq_state,
		*mut hpo_frl_stream_encoderhpo_frl_stream_enc,
		const *mut dc_crtc_timingtiming,
		u8 *dsc_packed_pps);

void hwss_add_setup_periodic_interrupt(*mut block_sequence_stateseq_state,
		*mut dcdc,
		*mut pipe_ctxpipe_ctx);

void hwss_add_dp_trace_source_sequence(*mut block_sequence_stateseq_state,
		*mut dc_linklink,
		enum dpcd_source_sequence source);

void hwss_add_set_dmdata_attributes(*mut block_sequence_stateseq_state,
		*mut pipe_ctxpipe_ctx);

void hwss_add_link_increase_mst_payload(*mut block_sequence_stateseq_state,
		*mut pipe_ctxpipe_ctx,
		u32 mst_stream_bw);

void hwss_add_link_reduce_mst_payload(*mut block_sequence_stateseq_state,
		*mut pipe_ctxpipe_ctx,
		u32 mst_stream_bw);

void hwss_add_dp_set_test_pattern(*mut block_sequence_stateseq_state,
		*mut dc_linklink,
		enum dp_test_pattern test_pattern,
		enum dp_test_pattern_color_space test_pattern_color_space,
		const *mut link_training_settingsp_link_settings,
		const u8 *p_custom_pattern,
		u32 cust_pattern_size);

void hwss_add_link_set_dpms_off(*mut block_sequence_stateseq_state,
		*mut pipe_ctxpipe_ctx);

void hwss_add_disable_audio_stream(*mut block_sequence_stateseq_state,
		*mut pipe_ctxpipe_ctx);

void hwss_add_prepare_bandwidth(*mut block_sequence_stateseq_state,
		*mut dcdc,
		*mut dc_statecontext);

void hwss_add_link_set_dpms_on(*mut block_sequence_stateseq_state,
		*mut dc_statestate,
		*mut pipe_ctxpipe_ctx);

/* Clock manager BLS executor functions */
void hwss_clk_mgr_set_max_memclk(*mut block_sequence_paramsparams);
void hwss_clk_mgr_update_clocks(*mut block_sequence_paramsparams);

void hwss_hubbub_program_watermarks(*mut block_sequence_paramsparams);

void hwss_hubbub_program_arbiter(*mut block_sequence_paramsparams);

void hwss_hubbub_program_compbuf_segments(*mut block_sequence_paramsparams);

/* Clock manager BLS add-helper functions */
void hwss_add_clk_mgr_set_max_memclk(*mut block_sequence_stateseq_state,
		*mut clk_mgrclk_mgr,
		u32 memclk_mhz);

void hwss_add_clk_mgr_update_clocks(*mut block_sequence_stateseq_state,
		*mut clk_mgrclk_mgr);

void hwss_add_hubbub_program_watermarks(*mut block_sequence_stateseq_state,
		*mut dcdc,
		*mut hubbubhubbub,
		*mut dcn_watermark_setwatermarks,
		u32 refclk_mhz,
		bool safe_to_lower);

void hwss_add_hubbub_program_arbiter(*mut block_sequence_stateseq_state,
		*mut dcdc,
		*mut hubbubhubbub,
		*mut dml2_display_arb_regsarb_regs,
		bool safe_to_lower);

void hwss_add_hubbub_program_compbuf_segments(*mut block_sequence_stateseq_state,
		*mut hubbubhubbub,
		u32 compbuf_size,
		bool safe_to_lower);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
