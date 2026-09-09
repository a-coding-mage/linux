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

// Dependency equivalent of: #include "hw_sequencer_private.h"

extern "C" {
    pub fn dcn20_log_color_state(dc: *mut dc, log_ctx: *mut dc_log_buffer_ctx);
    pub fn dcn20_set_blend_lut(
        pipe_ctx: *mut pipe_ctx,
        plane_state: *const dc_plane_state,
    ) -> bool;
    pub fn dcn20_set_shaper_3dlut(
        pipe_ctx: *mut pipe_ctx,
        plane_state: *const dc_plane_state,
    ) -> bool;
    pub fn dcn20_program_front_end_for_ctx(dc: *mut dc, context: *mut dc_state);
    pub fn dcn20_post_unlock_program_front_end(dc: *mut dc, context: *mut dc_state);
    pub fn dcn20_update_plane_addr(dc: *const dc, pipe_ctx: *mut pipe_ctx);
    pub fn dcn20_update_mpcc(dc: *mut dc, pipe_ctx: *mut pipe_ctx);
    pub fn dcn20_set_input_transfer_func(
        dc: *mut dc,
        pipe_ctx: *mut pipe_ctx,
        plane_state: *const dc_plane_state,
    ) -> bool;
    pub fn dcn20_set_output_transfer_func(params: *mut set_output_transfer_func_params) -> bool;
    pub fn dcn20_program_output_csc(
        dc: *mut dc,
        pipe_ctx: *mut pipe_ctx,
        colorspace: dc_color_space,
        matrix: *mut u16,
        opp_id: i32,
    );
    pub fn dcn20_enable_stream(pipe_ctx: *mut pipe_ctx);
    pub fn dcn20_unblank_stream(pipe_ctx: *mut pipe_ctx, link_settings: *mut dc_link_settings);
    pub fn dcn20_disable_plane(dc: *mut dc, state: *mut dc_state, pipe_ctx: *mut pipe_ctx);
    pub fn dcn20_disable_pixel_data(dc: *mut dc, pipe_ctx: *mut pipe_ctx, blank: bool);
    pub fn dcn20_blank_pixel_data(dc: *mut dc, pipe_ctx: *mut pipe_ctx, blank: bool);
    pub fn dcn20_pipe_control_lock(dc: *mut dc, pipe: *mut pipe_ctx, lock: bool);
    pub fn dcn20_prepare_bandwidth(dc: *mut dc, context: *mut dc_state);
    pub fn dcn20_optimize_bandwidth(dc: *mut dc, context: *mut dc_state);
    pub fn dcn20_update_bandwidth(dc: *mut dc, context: *mut dc_state) -> bool;
    pub fn dcn20_reset_hw_ctx_wrap(dc: *mut dc, context: *mut dc_state);
    pub fn dcn20_enable_stream_timing(
        pipe_ctx: *mut pipe_ctx,
        context: *mut dc_state,
        dc: *mut dc,
    ) -> dc_status;
    pub fn dcn20_disable_stream_gating(dc: *mut dc, pipe_ctx: *mut pipe_ctx);
    pub fn dcn20_enable_stream_gating(dc: *mut dc, pipe_ctx: *mut pipe_ctx);
    pub fn dcn20_setup_vupdate_interrupt(dc: *mut dc, pipe_ctx: *mut pipe_ctx);
    pub fn dcn20_reset_back_end_for_pipe(
        dc: *mut dc,
        pipe_ctx: *mut pipe_ctx,
        context: *mut dc_state,
    );
    pub fn dcn20_init_blank(dc: *mut dc, tg: *mut timing_generator);
    pub fn dcn20_disable_vga(hws: *mut dce_hwseq);
    pub fn dcn20_plane_atomic_disable(dc: *mut dc, pipe_ctx: *mut pipe_ctx);
    pub fn dcn20_enable_power_gating_plane(hws: *mut dce_hwseq, enable: bool);
    pub fn dcn20_dpp_pg_control(hws: *mut dce_hwseq, dpp_inst: u32, power_on: bool);
    pub fn dcn20_hubp_pg_control(hws: *mut dce_hwseq, hubp_inst: u32, power_on: bool);
    pub fn dcn20_program_triple_buffer(
        dc: *const dc,
        pipe_ctx: *mut pipe_ctx,
        enable_triple_buffer: bool,
    );
    pub fn dcn20_enable_writeback(
        dc: *mut dc,
        wb_info: *mut dc_writeback_info,
        context: *mut dc_state,
    );
    pub fn dcn20_disable_writeback(dc: *mut dc, dwb_pipe_inst: u32);
    pub fn dcn20_update_odm(dc: *mut dc, context: *mut dc_state, pipe_ctx: *mut pipe_ctx);
    pub fn dcn20_dmdata_status_done(pipe_ctx: *mut pipe_ctx) -> bool;
    pub fn dcn20_program_dmdata_engine(pipe_ctx: *mut pipe_ctx);
    pub fn dcn20_set_dmdata_attributes(pipe_ctx: *mut pipe_ctx);
    pub fn dcn20_init_vm_ctx(
        hws: *mut dce_hwseq,
        dc: *mut dc,
        va_config: *mut dc_virtual_addr_space_config,
        vmid: i32,
    );
    pub fn dcn20_set_flip_control_gsl(pipe_ctx: *mut pipe_ctx, flip_immediate: bool);
    pub fn dcn20_dsc_pg_control(hws: *mut dce_hwseq, dsc_inst: u32, power_on: bool);
    pub fn dcn20_fpga_init_hw(dc: *mut dc);
    pub fn dcn20_wait_for_blank_complete(opp: *mut output_pixel_processor) -> bool;
    pub fn dcn20_dccg_init(hws: *mut dce_hwseq);
    pub fn dcn20_init_sys_ctx(
        hws: *mut dce_hwseq,
        dc: *mut dc,
        pa_config: *mut dc_phy_addr_space_config,
    ) -> i32;
    pub fn dcn20_set_disp_pattern_generator(
        dc: *const dc,
        pipe_ctx: *mut pipe_ctx,
        test_pattern: controller_dp_test_pattern,
        color_space: controller_dp_color_space,
        color_depth: dc_color_depth,
        solid_color: *const tg_color,
        width: i32,
        height: i32,
        offset: i32,
    );
    pub fn dcn20_setup_gsl_group_as_lock(
        dc: *const dc,
        pipe_ctx: *mut pipe_ctx,
        enable: bool,
    );
    pub fn dcn20_detect_pipe_changes(
        old_state: *mut dc_state,
        new_state: *mut dc_state,
        old_pipe: *mut pipe_ctx,
        new_pipe: *mut pipe_ctx,
    );
    pub fn dcn20_enable_plane(dc: *mut dc, pipe_ctx: *mut pipe_ctx, context: *mut dc_state);
    pub fn dcn20_update_dchubp_dpp(
        dc: *mut dc,
        pipe_ctx: *mut pipe_ctx,
        context: *mut dc_state,
    );
    pub fn dcn20_post_unlock_reset_opp(dc: *mut dc, opp_head: *mut pipe_ctx);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
