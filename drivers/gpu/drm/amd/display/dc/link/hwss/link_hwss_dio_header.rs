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

// Dependencies supplied by the corresponding translated headers:
// link_hwss.h, link_service.h

extern "C" {
    pub fn get_dio_link_hwss() -> *const link_hwss;
    pub fn can_use_dio_link_hwss(
        link: *const dc_link,
        link_res: *const link_resource,
    ) -> bool;
    pub fn set_dio_throttled_vcp_size(
        pipe_ctx: *mut pipe_ctx,
        throttled_vcp_size: fixed31_32,
    );
    pub fn setup_dio_stream_encoder(pipe_ctx: *mut pipe_ctx);
    pub fn reset_dio_stream_encoder(pipe_ctx: *mut pipe_ctx);
    pub fn setup_dio_stream_attribute(pipe_ctx: *mut pipe_ctx);
    pub fn enable_dio_dp_link_output(
        link: *mut dc_link,
        link_res: *const link_resource,
        signal: signal_type,
        clock_source: clock_source_id,
        link_settings: *const dc_link_settings,
    );
    pub fn disable_dio_link_output(
        link: *mut dc_link,
        link_res: *const link_resource,
        signal: signal_type,
    );
    pub fn set_dio_dp_link_test_pattern(
        link: *mut dc_link,
        link_res: *const link_resource,
        tp_params: *mut encoder_set_dp_phy_pattern_param,
    );
    pub fn set_dio_dp_lane_settings(
        link: *mut dc_link,
        link_res: *const link_resource,
        link_settings: *const dc_link_settings,
        lane_settings: *const dc_lane_settings,
    );
    pub fn setup_dio_audio_output(
        pipe_ctx: *mut pipe_ctx,
        audio_output: *mut audio_output,
        audio_inst: u32,
    );
    pub fn enable_dio_audio_packet(pipe_ctx: *mut pipe_ctx);
    pub fn disable_dio_audio_packet(pipe_ctx: *mut pipe_ctx);
    pub fn update_dio_stream_allocation_table(
        link: *mut dc_link,
        link_res: *const link_resource,
        table: *const link_mst_stream_allocation_table,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
