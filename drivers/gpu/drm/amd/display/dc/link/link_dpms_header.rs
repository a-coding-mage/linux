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

// Dependency supplied by link_service.h is intentionally not implemented here.

extern "C" {
    pub fn link_set_dpms_on(
        state: *mut dc_state,
        pipe_ctx: *mut pipe_ctx,
    ) -> dc_status;
    pub fn link_set_dpms_off(pipe_ctx: *mut pipe_ctx) -> dc_status;
    pub fn link_resume(link: *mut dc_link);
    pub fn link_blank_all_dp_displays(dc: *mut dc);
    pub fn link_blank_all_edp_displays(dc: *mut dc);
    pub fn link_blank_dp_stream(link: *mut dc_link, hw_init: bool);
    pub fn link_set_all_streams_dpms_off_for_link(link: *mut dc_link);
    pub fn link_get_master_pipes_with_dpms_on(
        link: *const dc_link,
        state: *mut dc_state,
        count: *mut u8,
        pipes: *mut [pipe_ctx; MAX_PIPES],
    );
    pub fn link_increase_mst_payload(pipe_ctx: *mut pipe_ctx, req_pbn: u32) -> dc_status;
    pub fn link_reduce_mst_payload(pipe_ctx: *mut pipe_ctx, req_pbn: u32) -> dc_status;
    pub fn link_set_dsc_pps_packet(
        pipe_ctx: *mut pipe_ctx,
        enable: bool,
        immediate_update: bool,
    ) -> bool;
    pub fn link_calculate_sst_avg_time_slots_per_mtp(
        stream: *const dc_stream_state,
        link: *const dc_link,
    ) -> fixed31_32;
    pub fn link_set_dsc_on_stream(pipe_ctx: *mut pipe_ctx, enable: bool);
    pub fn link_set_dsc_enable(pipe_ctx: *mut pipe_ctx, enable: bool) -> bool;
    pub fn link_update_dsc_config(pipe_ctx: *mut pipe_ctx) -> bool;
    pub fn link_wait_for_unlocked(link: *mut dc_link);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
