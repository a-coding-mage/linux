// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// C dependencies: inc/core_types.h, dc.h, dc_stream.h, hw_sequencer_private.h,
// hwss/hw_sequencer.h, and dcn401/dcn401_dccg.h.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ips_ono_state {
    ONO_ON = 0,
    ONO_ON_IN_PROGRESS = 1,
    ONO_OFF = 2,
    ONO_OFF_IN_PROGRESS = 3,
}

#[repr(C)]
pub struct ips_ono_region_state {
    /// @desire_pwr_state: desired power state based on configured value
    pub desire_pwr_state: u32,
    /// @current_pwr_state: current power gate status
    pub current_pwr_state: u32,
}

// Opaque declarations supplied by included headers or other translation units.
#[repr(C)] pub struct dc { _private: [u8; 0] }
#[repr(C)] pub struct program_gamut_remap_params { _private: [u8; 0] }
#[repr(C)] pub struct pipe_ctx { _private: [u8; 0] }
#[repr(C)] pub struct dc_plane_state { _private: [u8; 0] }
#[repr(C)] pub struct set_output_transfer_func_params { _private: [u8; 0] }
#[repr(C)] pub struct dce_hwseq { _private: [u8; 0] }
#[repr(C)] pub struct dc_link { _private: [u8; 0] }
#[repr(C)] pub struct link_resource { _private: [u8; 0] }
#[repr(C)] pub struct dc_cursor_position { _private: [u8; 0] }
#[repr(C)] pub struct dc_state { _private: [u8; 0] }
#[repr(C)] pub struct dc_link_settings { _private: [u8; 0] }
#[repr(C)] pub struct dpp { _private: [u8; 0] }
#[repr(C)] pub struct hubp { _private: [u8; 0] }
#[repr(C)] pub struct dc_stream_state { _private: [u8; 0] }
#[repr(C)] pub struct dc_writeback_info { _private: [u8; 0] }
#[repr(C)] pub struct resource_pool { _private: [u8; 0] }
#[repr(C)] pub struct block_sequence_state { _private: [u8; 0] }
#[repr(C)] pub union block_sequence_params { _private: [u8; 0] }

// enum signal_type and enum dc_status are supplied by the C dependencies.
pub type signal_type = i32;
pub type dc_status = i32;

extern "C" {
    pub fn dcn401_program_gamut_remap(params: *mut program_gamut_remap_params);
    pub fn dcn401_init_hw(dc: *mut dc);
    pub fn dcn401_set_mcm_luts(pipe_ctx: *mut pipe_ctx, plane_state: *const dc_plane_state) -> bool;
    pub fn dcn401_set_output_transfer_func(params: *mut set_output_transfer_func_params) -> bool;
    pub fn dcn401_trigger_3dlut_dma_load(pipe_ctx: *mut pipe_ctx);
    pub fn dcn401_calculate_dccg_tmds_div_value(pipe_ctx: *mut pipe_ctx, tmds_div: *mut u32);
    pub fn dcn401_enable_stream_timing(pipe_ctx: *mut pipe_ctx, context: *mut dc_state, dc: *mut dc) -> dc_status;
    pub fn dcn401_enable_stream(pipe_ctx: *mut pipe_ctx);
    pub fn dcn401_setup_hpo_hw_control(hws: *const dce_hwseq, enable: bool);
    pub fn dcn401_disable_link_output(link: *mut dc_link, link_res: *const link_resource, signal: signal_type);
    pub fn dcn401_set_cursor_position(pipe_ctx: *mut pipe_ctx);
    pub fn dcn401_apply_idle_power_optimizations(dc: *mut dc, enable: bool) -> bool;
    pub fn dcn401_wait_for_dcc_meta_propagation(dc: *const dc, top_pipe_to_program: *const pipe_ctx);
    pub fn dcn401_prepare_bandwidth(dc: *mut dc, context: *mut dc_state);
    pub fn dcn401_prepare_bandwidth_sequence(dc: *mut dc, context: *mut dc_state, seq_state: *mut block_sequence_state);
    pub fn dcn401_optimize_bandwidth(dc: *mut dc, context: *mut dc_state);
    pub fn dcn401_optimize_bandwidth_sequence(dc: *mut dc, context: *mut dc_state, seq_state: *mut block_sequence_state);
    pub fn dcn401_dmub_hw_control_lock(dc: *mut dc, context: *mut dc_state, lock: bool);
    pub fn dcn401_fams2_update_config(dc: *mut dc, context: *mut dc_state, enable: bool);
    pub fn dcn401_dmub_hw_control_lock_fast(params: *mut block_sequence_params);
    pub fn dcn401_unblank_stream(pipe_ctx: *mut pipe_ctx, link_settings: *mut dc_link_settings);
    pub fn dcn401_hardware_release(dc: *mut dc);
    pub fn dcn401_update_odm(dc: *mut dc, context: *mut dc_state, otg_master: *mut pipe_ctx);
    pub fn dcn401_update_odm_sequence(dc: *mut dc, context: *mut dc_state, otg_master: *mut pipe_ctx, seq_state: *mut block_sequence_state);
    pub fn adjust_hotspot_between_slices_for_2x_magnify(cursor_width: u32, pos_cpy: *mut dc_cursor_position);
    pub fn dcn401_wait_for_det_buffer_update_under_otg_master(dc: *mut dc, context: *mut dc_state, otg_master: *mut pipe_ctx);
    pub fn dcn401_interdependent_update_lock(dc: *mut dc, context: *mut dc_state, lock: bool);
    pub fn dcn401_program_outstanding_updates(dc: *mut dc, context: *mut dc_state);
    pub fn dcn401_reset_back_end_for_pipe(dc: *mut dc, pipe_ctx: *mut pipe_ctx, context: *mut dc_state);
    pub fn dcn401_reset_hw_ctx_wrap(dc: *mut dc, context: *mut dc_state);
    pub fn dcn401_program_pipe(dc: *mut dc, pipe_ctx: *mut pipe_ctx, context: *mut dc_state);
    pub fn dcn401_program_pipe_sequence(dc: *mut dc, pipe_ctx: *mut pipe_ctx, context: *mut dc_state, seq_state: *mut block_sequence_state);
    pub fn dcn401_perform_3dlut_wa_unlock(pipe_ctx: *mut pipe_ctx);
    pub fn dcn401_program_front_end_for_ctx(dc: *mut dc, context: *mut dc_state);
    pub fn dcn401_post_unlock_program_front_end(dc: *mut dc, context: *mut dc_state);
    pub fn dcn401_update_bandwidth(dc: *mut dc, context: *mut dc_state) -> bool;
    pub fn dcn401_detect_pipe_changes(old_state: *mut dc_state, new_state: *mut dc_state, old_pipe: *mut pipe_ctx, new_pipe: *mut pipe_ctx);
    pub fn dcn401_plane_atomic_power_down(dc: *mut dc, dpp: *mut dpp, hubp: *mut hubp);
    pub fn dcn401_plane_atomic_power_down_sequence(dc: *mut dc, dpp: *mut dpp, hubp: *mut hubp, seq_state: *mut block_sequence_state);
    pub fn dcn401_plane_atomic_disconnect_sequence(dc: *mut dc, state: *mut dc_state, pipe_ctx: *mut pipe_ctx, seq_state: *mut block_sequence_state);
    pub fn dcn401_blank_pixel_data_sequence(dc: *mut dc, pipe_ctx: *mut pipe_ctx, blank: bool, seq_state: *mut block_sequence_state);
    pub fn dcn401_initialize_min_clocks(dc: *mut dc);
    pub fn dcn401_update_cursor_offload_pipe(dc: *mut dc, pipe: *const pipe_ctx);
    pub fn dcn401_program_all_writeback_pipes_in_tree_sequence(dc: *mut dc, stream: *const dc_stream_state, context: *mut dc_state, seq_state: *mut block_sequence_state);
    pub fn dcn401_enable_writeback_sequence(dc: *mut dc, wb_info: *mut dc_writeback_info, context: *mut dc_state, mpcc_inst: i32, seq_state: *mut block_sequence_state);
    pub fn dcn401_disable_writeback_sequence(dc: *mut dc, wb_info: *mut dc_writeback_info, seq_state: *mut block_sequence_state);
    pub fn dcn401_update_writeback_sequence(dc: *mut dc, wb_info: *mut dc_writeback_info, context: *mut dc_state, seq_state: *mut block_sequence_state);
    pub fn dcn401_setup_gsl_group_as_lock_sequence(dc: *const dc, pipe_ctx: *mut pipe_ctx, enable: bool, seq_state: *mut block_sequence_state);
    pub fn dcn401_disable_plane_sequence(dc: *mut dc, state: *mut dc_state, pipe_ctx: *mut pipe_ctx, seq_state: *mut block_sequence_state);
    pub fn dcn401_post_unlock_reset_opp_sequence(dc: *mut dc, opp_head: *mut pipe_ctx, seq_state: *mut block_sequence_state);
    pub fn dcn401_dc_ip_request_cntl(dc: *mut dc, enable: bool);
    pub fn dcn401_enable_plane_sequence(dc: *mut dc, pipe_ctx: *mut pipe_ctx, context: *mut dc_state, seq_state: *mut block_sequence_state);
    pub fn dcn401_update_dchubp_dpp_sequence(dc: *mut dc, pipe_ctx: *mut pipe_ctx, context: *mut dc_state, seq_state: *mut block_sequence_state);
    pub fn dcn401_update_mpcc_sequence(dc: *mut dc, pipe_ctx: *mut pipe_ctx, seq_state: *mut block_sequence_state);
    pub fn dcn401_wait_for_mpcc_disconnect_sequence(dc: *mut dc, res_pool: *mut resource_pool, pipe_ctx: *mut pipe_ctx, seq_state: *mut block_sequence_state);
    pub fn dcn401_setup_vupdate_interrupt_sequence(dc: *mut dc, pipe_ctx: *mut pipe_ctx, seq_state: *mut block_sequence_state);
    pub fn dcn401_set_hdr_multiplier_sequence(pipe_ctx: *mut pipe_ctx, seq_state: *mut block_sequence_state);
    pub fn dcn401_program_mall_pipe_config_sequence(dc: *mut dc, context: *mut dc_state, seq_state: *mut block_sequence_state);
    pub fn dcn401_verify_allow_pstate_change_high_sequence(dc: *mut dc, seq_state: *mut block_sequence_state);
    pub fn dcn401_hw_wa_force_recovery_sequence(dc: *mut dc, seq_state: *mut block_sequence_state) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
