// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Must match DALSMC_MAX_UTM_SOP_COUNT in dalsmc.h without including it.
pub const UTM_QOS_MODEL_V3_MAX_LOAD_LEVEL_COUNT: usize = 3;
pub const UTM_QOS_MODEL_V3_MAX_SOP_COUNT: usize = 5;

pub const UTM_QOS_MODEL_V3_LOAD_LEVEL_IDLE: u32 = 0;
pub const UTM_QOS_MODEL_V3_LOAD_LEVEL_ACTIVE_ALTERNATE_PSTATE: u32 = 1;
pub const UTM_QOS_MODEL_V3_LOAD_LEVEL_ACTIVE: u32 = 2;

/**
 * utm_qos_model_dchub_v3_sop_entry - Per-SOP QoS parameters for one load level.
 *
 * All latency fields are in picoseconds. All bandwidth fields are in KBps.
 * Budget percentage and derate are pre-applied — callers use values
 * directly without further scaling.
 */
#[repr(C)]
pub struct utm_qos_model_dchub_v3_sop_entry {
    /* latencies */
    pub urgent_ramp_ps: u32,
    pub t_trip_ps: u32,
    pub meta_trip_to_mem_ps: u32,
    pub max_req_latency_urg_ps: u32,
    pub avg_req_latency_urg_ps: u32,
    pub max_req_latency_non_urg_ps: u32,
    pub avg_req_latency_non_urg_ps: u32,
    pub df_response_time_ps: u32,
    /* bandwidths (budget allocation and derate pre-applied) */
    pub urgent_bandwidth_KBps: u32,
    pub nominal_bandwidth_KBps: u32,
    pub lsdma_bandwidth_KBps: u32,
}

/**
 * utm_qos_model_dchub_v3 - DCN6 flat UTM QoS table.
 *
 * Indexed as sops[load_level][sop_index]. Load level constants:
 *   UTM_QOS_MODEL_V3_LOAD_LEVEL_IDLE                    (max budget %)
 *   UTM_QOS_MODEL_V3_LOAD_LEVEL_ACTIVE_ALTERNATE_PSTATE (min budget %)
 *   UTM_QOS_MODEL_V3_LOAD_LEVEL_ACTIVE                  (same as alt pstate, lsdma=0)
 */
#[repr(C)]
pub struct utm_qos_model_dchub_v3 {
    pub load_level_count: u8,
    pub sop_count: u8,
    pub sops: [[utm_qos_model_dchub_v3_sop_entry; UTM_QOS_MODEL_V3_MAX_SOP_COUNT];
        UTM_QOS_MODEL_V3_MAX_LOAD_LEVEL_COUNT],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
