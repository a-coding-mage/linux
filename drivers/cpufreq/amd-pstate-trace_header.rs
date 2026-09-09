/* SPDX-License-Identifier: GPL-2.0 */
/*
 * amd-pstate-trace.h - AMD Processor P-state Frequency Driver Tracer
 *
 * Copyright (C) 2021 Advanced Micro Devices, Inc. All Rights Reserved.
 *
 * Author: Huang Rui <ray.huang@amd.com>
 */

// TRACE_SYSTEM amd_cpu
// TRACE_INCLUDE_FILE amd-pstate-trace
// The C tracepoint framework generates the declarations below.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AmdPstatePerfEntry {
    pub min_perf: u8,
    pub target_perf: u8,
    pub capacity: u8,
    pub freq: u64,
    pub mperf: u64,
    pub aperf: u64,
    pub tsc: u64,
    pub cpu_id: u32,
    pub fast_switch: bool,
}

pub const AMD_PSTATE_PERF_PRINTK: &str =
    "amd_min_perf=%hhu amd_des_perf=%hhu amd_max_perf=%hhu freq=%llu mperf=%llu aperf=%llu tsc=%llu cpu_id=%u fast_switch=%s";

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AmdPstateEppPerfEntry {
    pub cpu_id: u32,
    pub highest_perf: u8,
    pub epp: u8,
    pub min_perf: u8,
    pub max_perf: u8,
    pub boost: bool,
    pub changed: bool,
}

pub const AMD_PSTATE_EPP_PERF_PRINTK: &str =
    "cpu%u: [%hhu<->%hhu]/%hhu, epp=%hhu, boost=%u, changed=%u";

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AmdPstateCppcReq2Entry {
    pub cpu_id: u32,
    pub floor_perf: u8,
    pub changed: bool,
    pub err_code: i32,
}

pub const AMD_PSTATE_CPPC_REQ2_PRINTK: &str =
    "cpu%u: floor_perf=%u, changed=%u (error = %d)";

extern "C" {
    pub fn trace_amd_pstate_perf(
        min_perf: u8,
        target_perf: u8,
        capacity: u8,
        freq: u64,
        mperf: u64,
        aperf: u64,
        tsc: u64,
        cpu_id: u32,
        fast_switch: bool,
    );

    pub fn trace_amd_pstate_epp_perf(
        cpu_id: u32,
        highest_perf: u8,
        epp: u8,
        min_perf: u8,
        max_perf: u8,
        boost: bool,
        changed: bool,
    );

    pub fn trace_amd_pstate_cppc_req2(
        cpu_id: u32,
        floor_perf: u8,
        changed: bool,
        err_code: i32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
