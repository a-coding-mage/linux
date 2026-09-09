/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020-2026 Intel Corporation
 */

// Translated from ivpu_jsm_msg.h. The declarations below are supplied by
// vpu_jsm_api.h and other parts of the surrounding C interface.

use core::ffi::c_char;

pub enum vpu_ipc_msg_type {}
pub enum ivpu_device {}
pub enum vpu_jsm_msg {}

unsafe extern "C" {
    pub fn ivpu_jsm_msg_type_to_str(type_: vpu_ipc_msg_type) -> *const c_char;

    pub fn ivpu_jsm_register_db(
        vdev: *mut ivpu_device,
        ctx_id: u32,
        db_id: u32,
        jobq_base: u64,
        jobq_size: u32,
    ) -> i32;
    pub fn ivpu_jsm_unregister_db(vdev: *mut ivpu_device, db_id: u32) -> i32;
    pub fn ivpu_jsm_get_heartbeat(
        vdev: *mut ivpu_device,
        engine: u32,
        heartbeat: *mut u64,
    ) -> i32;
    pub fn ivpu_jsm_reset_engine(
        vdev: *mut ivpu_device,
        engine: u32,
        response: *mut vpu_jsm_msg,
    ) -> i32;
    pub fn ivpu_jsm_preempt_engine(vdev: *mut ivpu_device, engine: u32, preempt_id: u32) -> i32;
    pub fn ivpu_jsm_dyndbg_control(
        vdev: *mut ivpu_device,
        command: *mut c_char,
        size: usize,
    ) -> i32;
    pub fn ivpu_jsm_trace_get_capability(
        vdev: *mut ivpu_device,
        trace_destination_mask: *mut u32,
        trace_hw_component_mask: *mut u64,
    ) -> i32;
    pub fn ivpu_jsm_trace_set_config(
        vdev: *mut ivpu_device,
        trace_level: u32,
        trace_destination_mask: u32,
        trace_hw_component_mask: u64,
    ) -> i32;
    pub fn ivpu_jsm_context_release(vdev: *mut ivpu_device, host_ssid: u32) -> i32;
    pub fn ivpu_jsm_pwr_d0i3_enter(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_jsm_hws_create_cmdq(
        vdev: *mut ivpu_device,
        ctx_id: u32,
        cmdq_group: u32,
        cmdq_id: u32,
        pid: u32,
        engine: u32,
        cmdq_base: u64,
        cmdq_size: u32,
    ) -> i32;
    pub fn ivpu_jsm_hws_destroy_cmdq(vdev: *mut ivpu_device, ctx_id: u32, cmdq_id: u32) -> i32;
    pub fn ivpu_jsm_hws_register_db(
        vdev: *mut ivpu_device,
        ctx_id: u32,
        cmdq_id: u32,
        db_id: u32,
        cmdq_base: u64,
        cmdq_size: u32,
    ) -> i32;
    pub fn ivpu_jsm_hws_resume_engine(vdev: *mut ivpu_device, engine: u32) -> i32;
    pub fn ivpu_jsm_hws_set_context_sched_properties(
        vdev: *mut ivpu_device,
        ctx_id: u32,
        cmdq_id: u32,
        priority: u32,
    ) -> i32;
    pub fn ivpu_jsm_hws_set_scheduling_log(
        vdev: *mut ivpu_device,
        engine_idx: u32,
        host_ssid: u32,
        vpu_log_buffer_va: u64,
    ) -> i32;
    pub fn ivpu_jsm_hws_setup_priority_bands(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_jsm_metric_streamer_start(
        vdev: *mut ivpu_device,
        metric_group_mask: u64,
        sampling_rate: u64,
        buffer_addr: u64,
        buffer_size: u64,
    ) -> i32;
    pub fn ivpu_jsm_metric_streamer_stop(vdev: *mut ivpu_device, metric_group_mask: u64) -> i32;
    pub fn ivpu_jsm_metric_streamer_update(
        vdev: *mut ivpu_device,
        metric_group_mask: u64,
        buffer_addr: u64,
        buffer_size: u64,
        bytes_written: *mut u64,
    ) -> i32;
    pub fn ivpu_jsm_metric_streamer_info(
        vdev: *mut ivpu_device,
        metric_group_mask: u64,
        buffer_addr: u64,
        buffer_size: u64,
        sample_size: *mut u32,
        info_size: *mut u64,
    ) -> i32;
    pub fn ivpu_jsm_dct_enable(vdev: *mut ivpu_device, active_us: u32, inactive_us: u32) -> i32;
    pub fn ivpu_jsm_dct_disable(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_jsm_state_dump(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_jsm_state_dump_no_reply(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_jsm_msg_freq_config(
        vdev: *mut ivpu_device,
        min_ratio: u16,
        pn_ratio: u16,
        max_ratio: u16,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
