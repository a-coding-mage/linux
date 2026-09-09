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

// Dependency declarations supplied by hw_sequencer_private.h and other units.
pub struct dc;
pub struct dce_hwseq;
pub struct dc_phy_addr_space_config;
pub struct dc_state;
pub struct pipe_ctx;
pub struct abm;
pub struct set_backlight_level_params;
pub struct dc_stream_state;

extern "C" {
    pub fn dcn21_init_sys_ctx(
        hws: *mut dce_hwseq,
        dc: *mut dc,
        pa_config: *mut dc_phy_addr_space_config,
    ) -> i32;

    pub fn dcn21_s0i3_golden_init_wa(dc: *mut dc) -> bool;

    pub fn dcn21_exit_optimized_pwr_state(
        dc: *const dc,
        context: *mut dc_state,
    );

    pub fn dcn21_optimize_pwr_state(
        dc: *const dc,
        context: *mut dc_state,
    );

    pub fn dcn21_PLAT_58856_wa(
        context: *mut dc_state,
        pipe_ctx: *mut pipe_ctx,
    );

    pub fn dcn21_dmub_cacp_set_pipe(
        abm: *mut abm,
        otg_inst: u32,
        option: u32,
        panel_inst: u32,
        pwrseq_inst: u32,
    ) -> bool;

    pub fn dcn21_dmub_abm_set_pipe(
        abm: *mut abm,
        otg_inst: u32,
        option: u32,
        panel_inst: u32,
        pwrseq_inst: u32,
    ) -> bool;

    pub fn dcn21_set_pipe(pipe_ctx: *mut pipe_ctx);

    pub fn dcn21_set_abm_immediate_disable(pipe_ctx: *mut pipe_ctx);

    pub fn dcn21_set_backlight_level(
        pipe_ctx: *mut pipe_ctx,
        params: *mut set_backlight_level_params,
    ) -> bool;

    pub fn dcn21_is_abm_supported(
        dc: *mut dc,
        context: *mut dc_state,
        stream: *mut dc_stream_state,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
