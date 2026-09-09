// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.
// Dependency: utm_qos_model_types.h

#[repr(C)]
pub struct utm_qos_model_dchub_memory_path_latency_v2 {
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
pub struct utm_qos_model_dchub_memory_path_bandwidth_v2 {
    pub nominal_bandwidth_KBps: u32,
    pub urgent_bandwidth_KBps: u32,
}

#[repr(C)]
pub struct utm_qos_model_dchub_memory_path_qos_v2 {
    pub latency_upper_bound: utm_qos_model_dchub_memory_path_latency_v2,
    pub bandwidth_lower_bound: utm_qos_model_dchub_memory_path_bandwidth_v2,
}

#[repr(C)]
pub struct utm_qos_model_dchub_v2 {
    pub latencies: [utm_qos_model_dchub_memory_path_latency_v2; MAX_UTM_SOP_COUNT],
    pub max_nominal_utm_budget_percent: u8,
    pub min_nominal_utm_budget_percent: u8,
    pub max_urgent_utm_budget_percent: u8,
    pub min_urgent_utm_budget_percent: u8,
}

#[inline]
pub unsafe fn dchub_v2_is_qos_latency_supported_by_sop(
    model: &utm_qos_model,
    qos_latency: &utm_qos_model_dchub_memory_path_latency_v2,
    sop_index: u8,
) -> bool {
    let sop_latency = &(*model.dchub_v2).latencies[sop_index as usize];

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
pub unsafe fn dchub_v2_get_sop_total_available_bandwidth_KBps(
    model: &utm_qos_model,
    total_available_bandwidth: &mut utm_qos_model_dchub_memory_path_bandwidth_v2,
    sop_index: u8,
) {
    let sop = &(*model.sops.add(sop_index as usize));
    let socbb = &model.socbb;
    let dram_available_bandwidth_KBps_nominal = sop.uclk_khz as u64
        * socbb.dram_channel_count as u64
        * socbb.dram_channel_width_bytes as u64
        * socbb.dram_transactions_per_clock as u64
        * socbb.dram_derate_percent_nominal as u64 / 100;
    let dram_available_bandwidth_KBps_urgent = sop.uclk_khz as u64
        * socbb.dram_channel_count as u64
        * socbb.dram_channel_width_bytes as u64
        * socbb.dram_transactions_per_clock as u64
        * socbb.dram_derate_percent_urgent as u64 / 100;
    let fabric_available_bandwidth_KBps_nominal = sop.fclk_khz as u64
        * socbb.fabric_datapath_to_dcn_data_return_bytes as u64
        * socbb.fabric_derate_percent_nominal as u64 / 100;
    let fabric_available_bandwidth_KBps_urgent = sop.fclk_khz as u64
        * socbb.fabric_datapath_to_dcn_data_return_bytes as u64
        * socbb.fabric_derate_percent_urgent as u64 / 100;

    total_available_bandwidth.nominal_bandwidth_KBps =
        if dram_available_bandwidth_KBps_nominal < fabric_available_bandwidth_KBps_nominal {
            dram_available_bandwidth_KBps_nominal as u32
        } else {
            fabric_available_bandwidth_KBps_nominal as u32
        };
    total_available_bandwidth.urgent_bandwidth_KBps =
        if dram_available_bandwidth_KBps_urgent < fabric_available_bandwidth_KBps_urgent {
            dram_available_bandwidth_KBps_urgent as u32
        } else {
            fabric_available_bandwidth_KBps_urgent as u32
        };
}

#[inline]
pub unsafe fn dchub_v2_is_qos_bandwidth_supported_by_sop(
    model: &utm_qos_model,
    qos_bandwidth: &utm_qos_model_dchub_memory_path_bandwidth_v2,
    sop_index: u8,
    nominal_utm_budget_percent: u8,
    urgent_utm_budget_percent: u8,
) -> bool {
    let mut available_bandwidth = utm_qos_model_dchub_memory_path_bandwidth_v2 {
        nominal_bandwidth_KBps: 0,
        urgent_bandwidth_KBps: 0,
    };
    let dchub = &*model.dchub_v2;

    if nominal_utm_budget_percent > dchub.max_nominal_utm_budget_percent
        || nominal_utm_budget_percent < dchub.min_nominal_utm_budget_percent
        || urgent_utm_budget_percent > dchub.max_urgent_utm_budget_percent
        || urgent_utm_budget_percent < dchub.min_urgent_utm_budget_percent
    {
        return false;
    }

    dchub_v2_get_sop_total_available_bandwidth_KBps(model, &mut available_bandwidth, sop_index);
    let nominal_available_bandwidth_KBps = available_bandwidth.nominal_bandwidth_KBps as u64;
    let urgent_available_bandwidth_KBps = available_bandwidth.urgent_bandwidth_KBps as u64;

    if nominal_available_bandwidth_KBps * nominal_utm_budget_percent as u64 / 100
        < qos_bandwidth.nominal_bandwidth_KBps as u64
    {
        false
    } else if urgent_available_bandwidth_KBps * urgent_utm_budget_percent as u64 / 100
        < qos_bandwidth.urgent_bandwidth_KBps as u64
    {
        false
    } else {
        true
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
