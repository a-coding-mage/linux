// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.
// Dependency: utm_qos_model_types.h

#[repr(C)]
pub struct utm_qos_model_dchub_memory_path_latency_v1 {
    pub urgent_ramp_ps: u32,
    pub t_trip_ps: u32,
    pub meta_trip_to_mem_ps: u32,
    pub max_req_latency_urg_ps: u32,
    pub avg_req_latency_urg_ps: u32,
    pub max_req_latency_non_urg_ps: u32,
    pub avg_req_latency_non_urg_ps: u32,
    pub df_response_time_ps: u32,
}

#[repr(C)]
pub struct utm_qos_model_dchub_memory_path_bandwidth_v1 {
    pub nominal_bandwidth_KBps: u32,
    pub urgent_bandwidth_KBps: u32,
}

#[repr(C)]
pub struct utm_qos_model_dchub_memory_path_qos_v1 {
    pub latency_upper_bound: utm_qos_model_dchub_memory_path_latency_v1,
    pub bandwidth_lower_bound: utm_qos_model_dchub_memory_path_bandwidth_v1,
}

#[repr(C)]
pub struct utm_qos_model_dchub_v1 {
    pub latencies: [utm_qos_model_dchub_memory_path_latency_v1; MAX_UTM_SOP_COUNT],
    pub bandwidths: [utm_qos_model_dchub_memory_path_bandwidth_v1; MAX_UTM_SOP_COUNT],
    pub dcfclks_khz: [u32; MAX_UTM_SOP_COUNT],
    pub socclks_khz: [u32; MAX_UTM_SOP_COUNT],
}

#[inline]
pub unsafe fn dchub_v1_is_qos_latency_supported_by_sop(
    model: *const utm_qos_model,
    qos_latency: *const utm_qos_model_dchub_memory_path_latency_v1,
    sop_index: u8,
) -> bool {
    let sop_latency = &(*(*model).dchub_v1).latencies[sop_index as usize];
    let qos_latency = &*qos_latency;

    qos_latency.urgent_ramp_ps >= sop_latency.urgent_ramp_ps
        && qos_latency.t_trip_ps >= sop_latency.t_trip_ps
        && qos_latency.meta_trip_to_mem_ps >= sop_latency.meta_trip_to_mem_ps
        && qos_latency.max_req_latency_urg_ps >= sop_latency.max_req_latency_urg_ps
        && qos_latency.avg_req_latency_urg_ps >= sop_latency.avg_req_latency_urg_ps
        && qos_latency.max_req_latency_non_urg_ps >= sop_latency.max_req_latency_non_urg_ps
        && qos_latency.avg_req_latency_non_urg_ps >= sop_latency.avg_req_latency_non_urg_ps
        && qos_latency.df_response_time_ps >= sop_latency.df_response_time_ps
}

#[inline]
pub unsafe fn dchub_v1_is_qos_bandwidth_supported_by_sop(
    model: *const utm_qos_model,
    qos_bandwidth: *const utm_qos_model_dchub_memory_path_bandwidth_v1,
    sop_index: u8,
) -> bool {
    let bandwidth = &(*(*model).dchub_v1).bandwidths[sop_index as usize];
    let qos_bandwidth = &*qos_bandwidth;

    bandwidth.nominal_bandwidth_KBps >= qos_bandwidth.nominal_bandwidth_KBps
        && bandwidth.urgent_bandwidth_KBps >= qos_bandwidth.urgent_bandwidth_KBps
}

// The source contains a commented-out bandwidth calculation here. Its intent
// is preserved in the source-level translation by this note; it has no effect.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
