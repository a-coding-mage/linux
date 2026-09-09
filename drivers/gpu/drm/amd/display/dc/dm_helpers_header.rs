/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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

//! This file defines helper functions provided by the Display Manager to
//! Display Core.

use core::ffi::c_void;

extern "C" {
    pub fn dm_helpers_allocate_gpu_mem(ctx: *mut dc_context, type_: dc_gpu_mem_alloc_type, size: usize, addr: *mut i64) -> *mut c_void;
    pub fn dm_helpers_free_gpu_mem(ctx: *mut dc_context, type_: dc_gpu_mem_alloc_type, pv_mem: *mut c_void);
    pub fn dm_helpers_parse_edid_caps(link: *mut dc_link, edid: *const dc_edid, edid_caps: *mut dc_edid_caps) -> dc_edid_status;
    pub fn dm_helpers_dp_update_branch_info(ctx: *mut dc_context, link: *const dc_link);
    pub fn dm_helpers_dp_mst_write_payload_allocation_table(ctx: *mut dc_context, stream: *const dc_stream_state, proposed_table: *mut dc_dp_mst_stream_allocation_table, enable: bool) -> bool;
    pub fn dm_helpers_dp_mst_poll_pending_down_reply(ctx: *mut dc_context, link: *const dc_link);
    pub fn dm_helpers_dp_mst_clear_payload_allocation_table(ctx: *mut dc_context, link: *const dc_link);
    pub fn dm_helpers_dp_mst_poll_for_allocation_change_trigger(ctx: *mut dc_context, stream: *const dc_stream_state) -> act_return_status;
    pub fn dm_helpers_dp_mst_send_payload_allocation(ctx: *mut dc_context, stream: *const dc_stream_state);
    pub fn dm_helpers_dp_mst_update_mst_mgr_for_deallocation(ctx: *mut dc_context, stream: *const dc_stream_state);
    pub fn dm_helpers_dp_mst_start_top_mgr(ctx: *mut dc_context, link: *const dc_link, boot: bool) -> bool;
    pub fn dm_helpers_dp_mst_stop_top_mgr(ctx: *mut dc_context, link: *mut dc_link) -> bool;
    pub fn dm_helpers_dp_mst_update_branch_bandwidth(ctx: *mut dc_context, link: *mut dc_link);
    pub fn dm_helpers_dp_read_dpcd(ctx: *mut dc_context, link: *const dc_link, address: u32, data: *mut u8, size: u32) -> bool;
    pub fn dm_helpers_dp_write_dpcd(ctx: *mut dc_context, link: *const dc_link, address: u32, data: *const u8, size: u32) -> bool;
    pub fn dm_helpers_submit_i2c(ctx: *mut dc_context, link: *const dc_link, cmd: *mut i2c_command) -> bool;
    pub fn dm_helpers_execute_fused_io(ctx: *mut dc_context, link: *mut dc_link, commands: *mut dmub_rb_cmd, count: u8, timeout_us: u32) -> bool;
    pub fn dm_helpers_dp_write_dsc_enable(ctx: *mut dc_context, stream: *const dc_stream_state, enable: bool) -> bool;
    pub fn dm_helpers_dp_write_hblank_reduction(ctx: *mut dc_context, stream: *const dc_stream_state) -> bool;
    pub fn dm_helpers_is_dp_sink_present(link: *mut dc_link) -> bool;
    pub fn dm_helpers_mst_enable_stream_features(stream: *const dc_stream_state);
    pub fn dm_helpers_read_local_edid(ctx: *mut dc_context, link: *mut dc_link, sink: *mut dc_sink) -> dc_edid_status;
    pub fn dm_helpers_read_mccs_caps(ctx: *mut dc_context, link: *mut dc_link, sink: *mut dc_sink);
    pub fn dm_helpers_mccs_vcp_set(ctx: *mut dc_context, link: *mut dc_link, sink: *mut dc_sink);
    pub fn dm_helpers_submit_i2c_over_aux(ddc: *mut ddc_service, address: u32, offset: u8, cmd_buffer: *mut u8, len: u32, read: bool) -> bool;
    pub fn dm_helpers_dp_handle_test_pattern_request(ctx: *mut dc_context, link: *const dc_link, dpcd_test_pattern: link_test_pattern, dpcd_test_params: test_misc) -> bool;
    pub fn dm_set_dcn_clocks(ctx: *mut dc_context, clks: *mut dc_clocks);
    pub fn dm_helpers_enable_periodic_detection(ctx: *mut dc_context, enable: bool);
    pub fn dm_set_phyd32clk(ctx: *mut dc_context, freq_khz: i32);
    pub fn dm_helpers_dmub_outbox_interrupt_control(ctx: *mut dc_context, enable: bool) -> bool;
    pub fn dm_helpers_dmu_timeout(ctx: *mut dc_context);
    pub fn dm_helpers_smu_timeout(ctx: *mut dc_context, msg_id: u32, param: u32, timeout_us: u32);
    pub fn dm_helpers_init_panel_settings(ctx: *mut dc_context, config: *mut dc_panel_config, sink: *mut dc_sink);
    pub fn dm_helpers_override_panel_settings(ctx: *mut dc_context, link: *mut dc_link);
    pub fn dm_helper_dmub_aux_transfer_sync(ctx: *mut dc_context, link: *const dc_link, payload: *mut aux_payload, operation_result: *mut aux_return_code_type) -> i32;
    pub fn dm_helpers_dmub_set_config_sync(ctx: *mut dc_context, link: *const dc_link, payload: *mut set_config_cmd_payload, operation_result: *mut set_config_status) -> i32;
    pub fn dm_get_adaptive_sync_support_type(link: *mut dc_link) -> adaptive_sync_type;
    pub fn dm_helpers_get_sbios_edid(link: *mut dc_link, edid: *mut dc_edid) -> dc_edid_status;
    pub fn dm_helpers_is_fullscreen(ctx: *mut dc_context, stream: *mut dc_stream_state) -> bool;
    pub fn dm_helpers_is_hdr_on(ctx: *mut dc_context, stream: *mut dc_stream_state) -> bool;
}

// 0x1 = Result_OK, 0xFE = Result_UnkmownCmd, 0x0 = Status_Busy
#[inline]
pub const fn is_smu_timeout(result: u32) -> bool {
    result == 0x0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
