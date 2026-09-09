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

// Dependency supplied by the included C header: hw_sequencer_private.h

#[repr(C)]
pub struct dc;
#[repr(C)]
pub struct dce_hwseq;
#[repr(C)]
pub struct pipe_ctx;
#[repr(C)]
pub struct dc_phy_addr_space_config;
#[repr(C)]
pub struct dc_state;
#[repr(C)]
pub struct set_backlight_level_params;
#[repr(C)]
pub struct dc_stream_state;
#[repr(C)]
pub struct dc_static_screen_params;

extern "C" {
    pub fn dcn31_init_hw(dc: *mut dc);

    pub fn dcn31_dsc_pg_control(
        hws: *mut dce_hwseq,
        dsc_inst: ::core::ffi::c_uint,
        power_on: bool,
    );

    pub fn dcn31_enable_power_gating_plane(
        hws: *mut dce_hwseq,
        enable: bool,
    );

    pub fn dcn31_update_info_frame(pipe_ctx: *mut pipe_ctx);

    pub fn dcn31_z10_restore(dc: *const dc);
    pub fn dcn31_z10_save_init(dc: *mut dc);

    pub fn dcn31_hubp_pg_control(
        hws: *mut dce_hwseq,
        hubp_inst: ::core::ffi::c_uint,
        power_on: bool,
    );

    pub fn dcn31_init_sys_ctx(
        hws: *mut dce_hwseq,
        dc: *mut dc,
        pa_config: *mut dc_phy_addr_space_config,
    ) -> ::core::ffi::c_int;

    pub fn dcn31_reset_hw_ctx_wrap(
        dc: *mut dc,
        context: *mut dc_state,
    );

    pub fn dcn31_set_backlight_level(
        pipe_ctx: *mut pipe_ctx,
        params: *mut set_backlight_level_params,
    ) -> bool;

    pub fn dcn31_is_abm_supported(
        dc: *mut dc,
        context: *mut dc_state,
        stream: *mut dc_stream_state,
    ) -> bool;

    pub fn dcn31_init_pipes(dc: *mut dc, context: *mut dc_state);
    pub fn dcn31_setup_hpo_hw_control(hws: *const dce_hwseq, enable: bool);

    pub fn dcn31_set_static_screen_control(
        pipe_ctx: *mut *mut pipe_ctx,
        num_pipes: ::core::ffi::c_int,
        params: *const dc_static_screen_params,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
