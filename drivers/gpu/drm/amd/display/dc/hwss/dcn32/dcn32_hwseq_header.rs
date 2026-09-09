/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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
 */

// C dependency: "hw_sequencer_private.h"

extern "C" {

    pub fn dcn32_dsc_pg_control(
        hws: *mut dce_hwseq,
        dsc_inst: ::core::ffi::c_uint,
        power_on: bool,
    );

    pub fn dcn32_enable_power_gating_plane(hws: *mut dce_hwseq, enable: bool);

    pub fn dcn32_hubp_pg_control(
        hws: *mut dce_hwseq,
        hubp_inst: ::core::ffi::c_uint,
        power_on: bool,
    );

    pub fn dcn32_apply_idle_power_optimizations(dc: *mut dc, enable: bool) -> bool;

    pub fn dcn32_cab_for_ss_control(dc: *mut dc, enable: bool);

    pub fn dcn32_commit_subvp_config(dc: *mut dc, context: *mut dc_state);

    pub fn dcn32_set_mcm_luts(
        pipe_ctx: *mut pipe_ctx,
        plane_state: *const dc_plane_state,
    ) -> bool;

    pub fn dcn32_set_input_transfer_func(
        dc: *mut dc,
        pipe_ctx: *mut pipe_ctx,
        plane_state: *const dc_plane_state,
    ) -> bool;

    pub fn dcn32_set_mpc_shaper_3dlut(
        dpp_base: *mut dpp,
        mpc: *mut mpc,
        mpcc_id: ::core::ffi::c_int,
        stream: *const dc_stream_state,
    ) -> bool;

    pub fn dcn32_set_output_transfer_func(params: *mut set_output_transfer_func_params) -> bool;

    pub fn dcn32_init_hw(dc: *mut dc);

    pub fn dcn32_program_mall_pipe_config(dc: *mut dc, context: *mut dc_state);

    pub fn dcn32_update_mall_sel(dc: *mut dc, context: *mut dc_state);

    pub fn dcn32_update_force_pstate(dc: *mut dc, context: *mut dc_state);

    pub fn dcn32_update_odm(
        dc: *mut dc,
        context: *mut dc_state,
        pipe_ctx: *mut pipe_ctx,
    );

    pub fn dcn32_update_dsc_on_stream(pipe_ctx: *mut pipe_ctx, enable: bool);

    pub fn dcn32_calculate_dccg_k1_k2_values(
        pipe_ctx: *mut pipe_ctx,
        k1_div: *mut ::core::ffi::c_uint,
        k2_div: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_uint;

    pub fn dcn32_resync_fifo_dccg_dio(
        hws: *mut dce_hwseq,
        dc: *mut dc,
        context: *mut dc_state,
        current_pipe_idx: ::core::ffi::c_uint,
    );

    pub fn dcn32_subvp_pipe_control_lock(
        dc: *mut dc,
        context: *mut dc_state,
        lock: bool,
        should_lock_all_pipes: bool,
        top_pipe_to_program: *mut pipe_ctx,
        subvp_prev_use: bool,
    );

    pub fn dcn32_subvp_pipe_control_lock_fast(params: *mut block_sequence_params);

    pub fn dcn32_unblank_stream(
        pipe_ctx: *mut pipe_ctx,
        link_settings: *mut dc_link_settings,
    );

    pub fn dcn32_is_dp_dig_pixel_rate_div_policy(pipe_ctx: *mut pipe_ctx) -> bool;

    pub fn dcn32_calculate_pix_rate_divider(
        dc: *mut dc,
        context: *mut dc_state,
        stream: *const dc_stream_state,
    );

    pub fn dcn32_disable_link_output(
        link: *mut dc_link,
        link_res: *const link_resource,
        signal: signal_type,
    );

    pub fn dcn32_update_phantom_vp_position(
        dc: *mut dc,
        context: *mut dc_state,
        phantom_pipe: *mut pipe_ctx,
    );

    pub fn dcn32_apply_update_flags_for_phantom(phantom_pipe: *mut pipe_ctx);

    pub fn dcn32_dsc_pg_status(
        hws: *mut dce_hwseq,
        dsc_inst: ::core::ffi::c_uint,
    ) -> bool;

    pub fn dcn32_update_dsc_pg(
        dc: *mut dc,
        context: *mut dc_state,
        safe_to_disable: bool,
    );

    pub fn dcn32_enable_phantom_streams(dc: *mut dc, context: *mut dc_state);

    pub fn dcn32_disable_phantom_streams(dc: *mut dc, context: *mut dc_state);

    pub fn dcn32_init_blank(dc: *mut dc, tg: *mut timing_generator);

    pub fn dcn32_is_pipe_topology_transition_seamless(
        dc: *mut dc,
        cur_ctx: *const dc_state,
        new_ctx: *const dc_state,
    ) -> bool;

    pub fn dcn32_prepare_bandwidth(dc: *mut dc, context: *mut dc_state);

    pub fn dcn32_interdependent_update_lock(
        dc: *mut dc,
        context: *mut dc_state,
        lock: bool,
    );

    pub fn dcn32_program_outstanding_updates(dc: *mut dc, context: *mut dc_state);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
