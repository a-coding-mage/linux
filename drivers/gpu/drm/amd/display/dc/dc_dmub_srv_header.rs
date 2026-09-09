/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

#[repr(C)]
pub struct dc_dmub_srv {
    pub dmub: *mut dmub_srv,
    pub ctx: *mut dc_context,
    pub dm: *mut core::ffi::c_void,
    pub idle_exit_counter: i32,
    pub driver_signals: dmub_shared_state_ips_driver_signals,
    pub idle_allowed: bool,
    pub needs_idle_wake: bool,
    pub cursor_offload_enabled: bool,
}

extern "C" {
    pub fn dc_dmub_srv_wait_for_pending(dc_dmub_srv: *mut dc_dmub_srv) -> bool;
    pub fn dc_dmub_srv_optimized_init_done(dc_dmub_srv: *mut dc_dmub_srv) -> bool;
    pub fn dc_dmub_srv_cmd_list_queue_execute(dc_dmub_srv: *mut dc_dmub_srv, count: core::ffi::c_uint, cmd_list: *mut dmub_rb_cmd) -> bool;
    pub fn dc_dmub_srv_wait_for_idle(dc_dmub_srv: *mut dc_dmub_srv, wait_type: dm_dmub_wait_type, cmd_list: *mut dmub_rb_cmd) -> bool;
    pub fn dc_dmub_srv_cmd_run(dc_dmub_srv: *mut dc_dmub_srv, cmd: *mut dmub_rb_cmd, wait_type: dm_dmub_wait_type) -> bool;
    pub fn dc_dmub_srv_cmd_run_list(dc_dmub_srv: *mut dc_dmub_srv, count: core::ffi::c_uint, cmd_list: *mut dmub_rb_cmd, wait_type: dm_dmub_wait_type) -> bool;
    pub fn dc_dmub_srv_notify_stream_mask(dc_dmub_srv: *mut dc_dmub_srv, stream_mask: core::ffi::c_uint) -> bool;
    pub fn dc_dmub_srv_is_restore_required(dc_dmub_srv: *mut dc_dmub_srv) -> bool;
    pub fn dc_dmub_srv_get_dmub_outbox0_msg(dc: *const dc, entry: *mut dmcub_trace_buf_entry) -> bool;
    pub fn dc_dmub_trace_event_control(dc: *mut dc, enable: bool);
    pub fn dc_dmub_srv_drr_update_cmd(dc: *mut dc, tg_inst: u32, vtotal_min: u32, vtotal_max: u32);
    pub fn dc_dmub_srv_set_drr_manual_trigger_cmd(dc: *mut dc, tg_inst: u32);
    pub fn dc_dmub_srv_p_state_delegate(dc: *mut dc, enable_pstate: bool, context: *mut dc_state) -> bool;
    pub fn dc_dmub_srv_query_caps_cmd(dc_dmub_srv: *mut dc_dmub_srv);
    pub fn dc_dmub_srv_get_visual_confirm_color_cmd(dc: *mut dc, pipe_ctx: *mut pipe_ctx);
    pub fn dc_dmub_srv_clear_inbox0_ack(dmub_srv: *mut dc_dmub_srv);
    pub fn dc_dmub_srv_wait_for_inbox0_ack(dmub_srv: *mut dc_dmub_srv);
    pub fn dc_dmub_srv_send_inbox0_cmd(dmub_srv: *mut dc_dmub_srv, data: dmub_inbox0_data_register);
    pub fn dc_dmub_srv_get_diagnostic_data(dc_dmub_srv: *mut dc_dmub_srv) -> bool;
    pub fn dc_dmub_setup_subvp_dmub_command(dc: *mut dc, context: *mut dc_state, enable: bool);
    pub fn dc_dmub_srv_log_diagnostic_data(dc_dmub_srv: *mut dc_dmub_srv);
    pub fn dc_send_update_cursor_info_to_dmu(p_ctx: *mut pipe_ctx, pipe_idx: u8);
    pub fn dc_dmub_check_min_version(srv: *mut dmub_srv) -> bool;
    pub fn dc_dmub_srv_enable_dpia_trace(dc: *const dc);
    pub fn dc_dmub_srv_subvp_save_surf_addr(dc_dmub_srv: *const dc_dmub_srv, addr: *const dc_plane_address, subvp_index: u8);
    pub fn dc_dmub_srv_is_hw_pwr_up(dc_dmub_srv: *mut dc_dmub_srv, wait: bool) -> bool;
    pub fn dc_dmub_srv_apply_idle_power_optimizations(dc: *const dc, allow_idle: bool);
    pub fn dc_dmub_srv_set_power_state(dc_dmub_srv: *mut dc_dmub_srv, power_state: dc_acpi_cm_power_state);
    pub fn dc_dmub_srv_notify_fw_dc_power_state(dc_dmub_srv: *mut dc_dmub_srv, power_state: dc_acpi_cm_power_state);
    pub fn dc_dmub_srv_should_detect(dc_dmub_srv: *mut dc_dmub_srv) -> bool;
    pub fn dc_wake_and_execute_dmub_cmd(ctx: *const dc_context, cmd: *mut dmub_rb_cmd, wait_type: dm_dmub_wait_type) -> bool;
    pub fn dc_wake_and_execute_dmub_cmd_list(ctx: *const dc_context, count: core::ffi::c_uint, cmd: *mut dmub_rb_cmd, wait_type: dm_dmub_wait_type) -> bool;
    pub fn dc_wake_and_execute_gpint(ctx: *const dc_context, command_code: dmub_gpint_command, param: u16, response: *mut u32, wait_type: dm_dmub_wait_type) -> bool;
    pub fn dc_dmub_srv_fams2_update_config(dc: *mut dc, context: *mut dc_state, enable: bool);
    pub fn dc_dmub_srv_fams2_drr_update(dc: *mut dc, tg_inst: u32, vtotal_min: u32, vtotal_max: u32, vtotal_mid: u32, vtotal_mid_frame_num: u32, program_manual_trigger: bool);
    pub fn dc_dmub_srv_fams2_passthrough_flip(dc: *mut dc, state: *mut dc_state, stream: *mut dc_stream_state, srf_updates: *mut dc_surface_update, surface_count: i32);
    pub fn dmub_lsdma_init(dc_dmub_srv: *mut dc_dmub_srv) -> bool;
}

#[repr(C)]
pub struct lsdma_linear_copy_params {
    pub src_lo: u32, pub src_hi: u32, pub dst_lo: u32, pub dst_hi: u32,
    // count:30, read_compress:2
    pub count_read_compress: u32,
    // tmz:4, cache_policy_src:3, cache_policy_dst:3, data_format:6,
    // num_type:3, write_compress:2, max_com:2, max_uncom:1, reserved0:8
    pub control: u32,
}

extern "C" {
    pub fn dmub_lsdma_send_linear_copy_command(dc_dmub_srv: *mut dc_dmub_srv, copy_data: lsdma_linear_copy_params) -> bool;
}

#[repr(C)]
pub struct lsdma_linear_sub_window_copy_params {
    pub src_lo: u32, pub src_hi: u32, pub dst_lo: u32, pub dst_hi: u32,
    pub src_x_src_y: u32, pub dst_x_dst_y: u32, pub rect_x_rect_y: u32,
    pub src_pitch_dst_pitch: u32, pub src_slice_pitch: u32, pub dst_slice_pitch: u32,
    // tmz:4, element_size:3, src_cache_policy:3, dst_cache_policy:3,
    // data_format:6, num_type:3, read_compress:2, write_compress:2,
    // max_com:2, max_uncom:1, reserved0:3
    pub control: u32,
}

extern "C" {
    pub fn dmub_lsdma_send_linear_sub_window_copy_command(dc_dmub_srv: *mut dc_dmub_srv, copy_data: lsdma_linear_sub_window_copy_params) -> bool;
    pub fn dmub_lsdma_send_pio_copy_command(dc_dmub_srv: *mut dc_dmub_srv, src_addr: u64, dst_addr: u64, byte_count: u32, overlap_disable: u32) -> bool;
    pub fn dmub_lsdma_send_pio_constfill_command(dc_dmub_srv: *mut dc_dmub_srv, dst_addr: u64, byte_count: u32, data: u32) -> bool;
}

#[repr(C)]
pub struct lsdma_send_tiled_to_tiled_copy_command_params {
    pub src_addr: u64, pub dst_addr: u64,
    pub src_x_src_y: u32, pub dst_x_dst_y: u32, pub src_width_dst_width: u32,
    pub rect_x_rect_y: u32, pub src_height_dst_height: u32,
    // data_format:6, swizzle_mode:5, element_size:3, dcc:1, tmz:4,
    // read_compress:2, write_compress:2, max_com:2, max_uncom:1,
    // src_cache_policy:3, dst_cache_policy:3
    pub control: u32,
}

extern "C" {
    pub fn dmub_lsdma_send_tiled_to_tiled_copy_command(dc_dmub_srv: *mut dc_dmub_srv, params: lsdma_send_tiled_to_tiled_copy_command_params) -> bool;
    pub fn dmub_lsdma_send_poll_reg_write_command(dc_dmub_srv: *mut dc_dmub_srv, reg_addr: u32, reg_data: u32) -> bool;
    pub fn dc_dmub_srv_ips_residency_cntl(ctx: *const dc_context, panel_inst: u8, start_measurement: bool) -> bool;
    pub fn dc_dmub_srv_ips_query_residency_info(ctx: *const dc_context, panel_inst: u8, driver_info: *mut dmub_ips_residency_info, ips_mode: ips_residency_mode) -> bool;
    pub fn dc_dmub_srv_cursor_offload_init(dc: *mut dc);
    pub fn dc_dmub_srv_control_cursor_offload(dc: *mut dc, context: *mut dc_state, stream: *const dc_stream_state, enable: bool);
    pub fn dc_dmub_srv_program_cursor_now(dc: *mut dc, pipe: *const pipe_ctx);
    pub fn dc_dmub_srv_is_cursor_offload_enabled(dc: *const dc) -> bool;
    pub fn dc_dmub_srv_boot_time_crc_init(dc: *const dc, gpu_addr: u64, size: u32);
    pub fn dc_dmub_srv_release_hw(dc: *const dc);
    pub fn dc_dmub_srv_log_preos_dmcub_info(dc_dmub_srv: *mut dc_dmub_srv);
    pub fn dc_dmub_srv_ihc_set_dig_hdcp_interrupt_dest(dc_dmub_srv: *mut dc_dmub_srv, dig_id: u8, to_dmu: bool) -> bool;
    pub fn dc_dmub_srv_get_fams2_debug_meta(dc_dmub_srv: *mut dc_dmub_srv);
}

#[repr(C)]
pub struct ips_residency_info {
    pub ips_mode: ips_residency_mode,
    pub residency_percent: core::ffi::c_uint,
    pub entry_counter: core::ffi::c_uint,
    pub total_active_time_us: [core::ffi::c_uint; 2],
    pub total_inactive_time_us: [core::ffi::c_uint; 2],
    pub histogram: [core::ffi::c_uint; 16],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
