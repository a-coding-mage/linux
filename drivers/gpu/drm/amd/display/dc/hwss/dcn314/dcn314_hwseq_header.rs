/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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

// Dependency supplied by hw_sequencer_private.h.

#[repr(C)]
pub struct dc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pipe_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dce_hwseq {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_stream_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct link_resource {
    _private: [u8; 0],
}

#[repr(C)]
pub enum signal_type {}

extern "C" {
    pub fn dcn314_update_odm(
        dc: *mut dc,
        context: *mut dc_state,
        pipe_ctx: *mut pipe_ctx,
    );

    pub fn dcn314_dsc_pg_control(
        hws: *mut dce_hwseq,
        dsc_inst: u32,
        power_on: bool,
    );

    pub fn dcn314_enable_power_gating_plane(hws: *mut dce_hwseq, enable: bool);

    pub fn dcn314_calculate_dccg_k1_k2_values(
        pipe_ctx: *mut pipe_ctx,
        k1_div: *mut u32,
        k2_div: *mut u32,
    ) -> u32;

    pub fn dcn314_calculate_pix_rate_divider(
        dc: *mut dc,
        context: *mut dc_state,
        stream: *const dc_stream_state,
    );

    pub fn dcn314_resync_fifo_dccg_dio(
        hws: *mut dce_hwseq,
        dc: *mut dc,
        context: *mut dc_state,
        current_pipe_idx: u32,
    );

    pub fn dcn314_dpp_root_clock_control(
        hws: *mut dce_hwseq,
        dpp_inst: u32,
        clock_on: bool,
    );

    pub fn dcn314_disable_link_output(
        link: *mut dc_link,
        link_res: *const link_resource,
        signal: signal_type,
    );

    pub fn dcn314_dpp_pg_control(
        hws: *mut dce_hwseq,
        dpp_inst: u32,
        power_on: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
