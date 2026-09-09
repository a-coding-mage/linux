/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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

// Dependency equivalent of: #include "hw_sequencer_private.h"

extern "C" {
    pub fn dcn35_update_odm(dc: *mut dc, context: *mut dc_state, pipe_ctx: *mut pipe_ctx);

    pub fn dcn35_dsc_pg_control(hws: *mut dce_hwseq, dsc_inst: ::core::ffi::c_uint, power_on: bool);

    pub fn dcn35_dpp_root_clock_control(hws: *mut dce_hwseq, dpp_inst: ::core::ffi::c_uint, clock_on: bool);

    pub fn dcn35_dpstream_root_clock_control(hws: *mut dce_hwseq, dp_hpo_inst: ::core::ffi::c_uint, clock_on: bool);

    pub fn dcn35_hdmistream_root_clock_control(hws: *mut dce_hwseq, clock_on: bool);

    pub fn dcn35_physymclk_root_clock_control(hws: *mut dce_hwseq, phy_inst: ::core::ffi::c_uint, clock_on: bool);

    pub fn dcn35_enable_power_gating_plane(hws: *mut dce_hwseq, enable: bool);

    pub fn dcn35_set_dmu_fgcg(hws: *mut dce_hwseq, enable: bool);

    pub fn dcn35_init_hw(dc: *mut dc);

    pub fn dcn35_disable_link_output(
        link: *mut dc_link,
        link_res: *const link_resource,
        signal: signal_type,
    );

    pub fn dcn35_power_down_on_boot(dc: *mut dc);

    pub fn dcn35_apply_idle_power_optimizations(dc: *mut dc, enable: bool) -> bool;

    pub fn dcn35_z10_restore(dc: *const dc);

    pub fn dcn35_init_pipes(dc: *mut dc, context: *mut dc_state);
    pub fn dcn35_plane_atomic_disable(dc: *mut dc, pipe_ctx: *mut pipe_ctx);
    pub fn dcn35_enable_plane(dc: *mut dc, pipe_ctx: *mut pipe_ctx, context: *mut dc_state);
    pub fn dcn35_disable_plane(dc: *mut dc, state: *mut dc_state, pipe_ctx: *mut pipe_ctx);

    pub fn dcn35_calc_blocks_to_gate(
        dc: *mut dc,
        context: *mut dc_state,
        update_state: *mut pg_block_update,
    );
    pub fn dcn35_calc_blocks_to_ungate(
        dc: *mut dc,
        context: *mut dc_state,
        update_state: *mut pg_block_update,
    );
    pub fn dcn35_hw_block_power_up(dc: *mut dc, update_state: *mut pg_block_update);
    pub fn dcn35_hw_block_power_down(dc: *mut dc, update_state: *mut pg_block_update);
    pub fn dcn35_root_clock_control(
        dc: *mut dc,
        update_state: *mut pg_block_update,
        power_on: bool,
    );

    pub fn dcn35_prepare_bandwidth(dc: *mut dc, context: *mut dc_state);

    pub fn dcn35_optimize_bandwidth(dc: *mut dc, context: *mut dc_state);

    pub fn dcn35_setup_hpo_hw_control(hws: *const dce_hwseq, enable: bool);
    pub fn dcn35_dsc_pg_control(
        hws: *mut dce_hwseq,
        dsc_inst: ::core::ffi::c_uint,
        power_on: bool,
    );

    pub fn dcn35_set_drr(
        pipe_ctx: *mut *mut pipe_ctx,
        num_pipes: i32,
        adjust: dc_crtc_timing_adjust,
    );

    pub fn dcn35_set_static_screen_control(
        pipe_ctx: *mut *mut pipe_ctx,
        num_pipes: i32,
        params: *const dc_static_screen_params,
    );

    pub fn dcn35_set_long_vblank(
        pipe_ctx: *mut *mut pipe_ctx,
        num_pipes: i32,
        v_total_min: u32,
        v_total_max: u32,
    );

    pub fn dcn35_is_dp_dig_pixel_rate_div_policy(pipe_ctx: *mut pipe_ctx) -> bool;

    pub fn dcn35_hardware_release(dc: *mut dc);

    pub fn dcn35_abort_cursor_offload_update(dc: *mut dc, pipe: *const pipe_ctx);
    pub fn dcn35_begin_cursor_offload_update(dc: *mut dc, pipe: *const pipe_ctx);
    pub fn dcn35_commit_cursor_offload_update(dc: *mut dc, pipe: *const pipe_ctx);
    pub fn dcn35_update_cursor_offload_pipe(dc: *mut dc, pipe: *const pipe_ctx);
    pub fn dcn35_notify_cursor_offload_drr_update(
        dc: *mut dc,
        context: *mut dc_state,
        stream: *const dc_stream_state,
    );
    pub fn dcn35_program_cursor_offload_now(dc: *mut dc, pipe: *const pipe_ctx);
    pub fn dcn35_disable_link_output(
        link: *mut dc_link,
        link_res: *const link_resource,
        signal: signal_type,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
