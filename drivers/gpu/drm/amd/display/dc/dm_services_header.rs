/*
 * Copyright 2015 Advanced Micro Devices, Inc.
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

/** This file defines external dependencies of Display Core. */

// Dependencies supplied by the surrounding translation unit:
// dm_services_types.h, logger_interface.h, and link_service_types.h.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)] pub struct dmub_srv { _private: [u8; 0] }
#[repr(C)] pub struct dc_dmub_srv { _private: [u8; 0] }
#[repr(C)] pub union dmub_rb_cmd { _private: [u8; 0] }

extern "C" {
    pub fn dm_register_interrupt(ctx: *mut dc_context, int_params: *mut dc_interrupt_params,
        ih: interrupt_handler, handler_args: *mut c_void) -> irq_handler_idx;
    pub fn dm_read_reg_func(ctx: *const dc_context, address: u32, func_name: *const c_char) -> u32;
    pub fn dm_write_reg_func(ctx: *const dc_context, address: u32, value: u32, func_name: *const c_char);

    pub fn cgs_read_ind_register(cgs_device: *mut c_void, addr_space: cgs_ind_reg, index: u32) -> u32;
    pub fn cgs_write_ind_register(cgs_device: *mut c_void, addr_space: cgs_ind_reg, index: u32, value: u32);

    pub fn generic_reg_set_ex(ctx: *const dc_context, addr: u32, reg_val: u32, n: c_int,
        shift1: u8, mask1: u32, field_value1: u32, ... ) -> u32;
    pub fn generic_reg_update_ex(ctx: *const dc_context, addr: u32, n: c_int,
        shift1: u8, mask1: u32, field_value1: u32, ... ) -> u32;
    pub fn dc_dmub_srv_create(dc: *mut dc, dmub: *mut dmub_srv) -> *mut dc_dmub_srv;
    pub fn dc_dmub_srv_destroy(dmub_srv: *mut *mut dc_dmub_srv);
    pub fn generic_reg_wait(ctx: *const dc_context, addr: u32, mask: u32, shift: u32,
        condition_value: u32, delay_between_poll_us: c_uint, time_out_num_tries: c_uint,
        func_name: *const c_char, line: c_int);
    pub fn snprintf_count(p_buf: *mut c_char, buf_size: c_uint, fmt: *const c_char, ...) -> c_uint;

    pub fn dm_pp_get_clock_levels_by_type(ctx: *const dc_context, clk_type: dm_pp_clock_type,
        clk_level_info: *mut dm_pp_clock_levels) -> bool;
    pub fn dm_pp_get_clock_levels_by_type_with_latency(ctx: *const dc_context, clk_type: dm_pp_clock_type,
        clk_level_info: *mut dm_pp_clock_levels_with_latency) -> bool;
    pub fn dm_pp_get_clock_levels_by_type_with_voltage(ctx: *const dc_context, clk_type: dm_pp_clock_type,
        clk_level_info: *mut dm_pp_clock_levels_with_voltage) -> bool;
    pub fn dm_pp_notify_wm_clock_changes(ctx: *const dc_context,
        wm_with_clock_ranges: *mut dm_pp_wm_sets_with_clock_ranges) -> bool;
    pub fn dm_pp_get_funcs(ctx: *mut dc_context, funcs: *mut pp_smu_funcs);
    pub fn dm_pp_apply_display_requirements(ctx: *const dc_context,
        pp_display_cfg: *const dm_pp_display_configuration) -> bool;
    pub fn dm_pp_apply_clock_for_voltage_request(ctx: *const dc_context,
        clock_for_voltage_req: *mut dm_pp_clock_for_voltage_req) -> bool;

    pub fn dm_query_extended_brightness_caps(ctx: *mut dc_context, display: dm_acpi_display_type,
        p_caps: *mut dm_acpi_atif_backlight_caps) -> bool;
    pub fn dm_dmcu_set_pipe(ctx: *mut dc_context, controller_id: c_uint) -> bool;
    pub fn dm_get_elapse_time_in_ns(ctx: *mut dc_context, current_time_stamp: u64,
        last_time_stamp: u64) -> u64;
    pub fn ktime_get_raw_ns() -> u64;
    pub fn dm_perf_trace_timestamp(func_name: *const c_char, line: c_uint, ctx: *mut dc_context);
    pub fn dm_trace_smu_enter(msg_id: u32, param_in: u32, delay: c_uint, ctx: *mut dc_context);
    pub fn dm_trace_smu_exit(success: bool, response: u32, ctx: *mut dc_context);
    pub fn dm_execute_dmub_cmd(ctx: *const dc_context, cmd: *mut dmub_rb_cmd,
        wait_type: dm_dmub_wait_type) -> bool;
    pub fn dm_execute_dmub_cmd_list(ctx: *const dc_context, count: c_uint, cmd: *mut dmub_rb_cmd,
        wait_type: dm_dmub_wait_type) -> bool;
    pub fn dm_acpi_process_phy_transition_interlock(ctx: *const dc_context,
        params: dm_process_phy_transition_init_params);
    pub fn dm_dtn_log_begin(ctx: *mut dc_context, log_ctx: *mut dc_log_buffer_ctx);
    pub fn dm_dtn_log_append_v(ctx: *mut dc_context, log_ctx: *mut dc_log_buffer_ctx,
        msg: *const c_char, ...);
    pub fn dm_dtn_log_end(ctx: *mut dc_context, log_ctx: *mut dc_log_buffer_ctx);
    pub fn dce_version_to_string(version: c_int) -> *mut c_char;
    pub fn dc_supports_vrr(v: dce_version) -> bool;
}

#[repr(C)]
pub struct persistent_data_flag { pub save_per_link: bool, pub save_per_edid: bool }

#[inline] pub unsafe fn dm_read_index_reg(ctx: *const dc_context, addr_space: cgs_ind_reg, index: u32) -> u32 {
    cgs_read_ind_register((*ctx).cgs_device, addr_space, index)
}
#[inline] pub unsafe fn dm_write_index_reg(ctx: *const dc_context, addr_space: cgs_ind_reg, index: u32, value: u32) {
    cgs_write_ind_register((*ctx).cgs_device, addr_space, index, value)
}
#[inline] pub fn get_reg_field_value_ex(reg_value: u32, mask: u32, shift: u8) -> u32 { (mask & reg_value) >> shift }
#[inline] pub fn set_reg_field_value_ex(reg_value: u32, value: u32, mask: u32, shift: u8) -> u32 {
    assert!(mask != 0); (reg_value & !mask) | (mask & value.wrapping_shl(shift as u32))
}

pub struct dc_context { pub cgs_device: *mut c_void }
pub struct dc { _private: [u8; 0] }
pub struct dc_interrupt_params { _private: [u8; 0] }
pub struct dc_log_buffer_ctx { _private: [u8; 0] }
pub struct dm_pp_clock_levels { _private: [u8; 0] }
pub struct dm_pp_clock_levels_with_latency { _private: [u8; 0] }
pub struct dm_pp_clock_levels_with_voltage { _private: [u8; 0] }
pub struct dm_pp_wm_sets_with_clock_ranges { _private: [u8; 0] }
pub struct pp_smu_funcs { _private: [u8; 0] }
pub struct dm_pp_display_configuration { _private: [u8; 0] }
pub struct dm_pp_clock_for_voltage_req { _private: [u8; 0] }
pub struct dm_acpi_atif_backlight_caps { _private: [u8; 0] }
pub struct dm_process_phy_transition_init_params { _private: [u8; 0] }
pub type irq_handler_idx = u32;
pub type interrupt_handler = Option<unsafe extern "C" fn(*mut c_void)>;
pub type cgs_ind_reg = u32;
pub type dm_pp_clock_type = u32;
pub type dm_acpi_display_type = u32;
pub type dm_dmub_wait_type = u32;
pub type dce_version = u32;

#[inline]
pub unsafe fn dm_get_timestamp(_ctx: *mut dc_context) -> u64 { ktime_get_raw_ns() }

// C macro equivalents. `__func__`, `__LINE__`, variadic arguments, and token
// pasting are represented by explicit Rust arguments or remain dependency
// supplied at each call site.
#[macro_export]
macro_rules! dm_read_reg { ($ctx:expr, $address:expr) => {
    unsafe { $crate::dm_read_reg_func($ctx, $address, core::ptr::null()) }
} }
#[macro_export]
macro_rules! dm_write_reg { ($ctx:expr, $address:expr, $value:expr) => {
    unsafe { $crate::dm_write_reg_func($ctx, $address, $value, core::ptr::null()) }
} }
#[macro_export]
macro_rules! get_reg_field_value { ($reg_value:expr, $mask:expr, $shift:expr) => {
    $crate::get_reg_field_value_ex($reg_value, $mask, $shift)
} }
#[macro_export]
macro_rules! set_reg_field_value { ($reg_value:expr, $value:expr, $mask:expr, $shift:expr) => {
    $reg_value = $crate::set_reg_field_value_ex($reg_value, $value, $mask, $shift)
} }
#[macro_export]
macro_rules! dm_log_to_buffer { ($buffer:expr, $size:expr, $fmt:expr, $args:expr) => {
    unsafe { vsnprintf($buffer, $size, $fmt, $args) }
} }
#[macro_export]
macro_rules! perf_trace { ($ctx:expr) => {
    unsafe { $crate::dm_perf_trace_timestamp(core::ptr::null(), 0, $ctx) }
} }
#[macro_export]
macro_rules! trace_smu_msg_delay { ($msg_id:expr, $param_in:expr, $delay:expr, $ctx:expr) => {
    unsafe { $crate::dm_trace_smu_enter($msg_id, $param_in, $delay, $ctx) }
} }
#[macro_export]
macro_rules! trace_smu_msg { ($msg_id:expr, $param_in:expr, $ctx:expr) => {
    unsafe { $crate::dm_trace_smu_enter($msg_id, $param_in, 0, $ctx) }
} }
#[macro_export]
macro_rules! trace_smu_msg_exit { ($success:expr, $response:expr, $ctx:expr) => {
    unsafe { $crate::dm_trace_smu_exit($success, $response, $ctx) }
} }

extern "C" { pub fn vsnprintf(buffer: *mut c_char, size: c_uint, fmt: *const c_char, ...) -> c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
