/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * intel_pt.h: Intel Processor Trace support
 * Copyright (c) 2013-2015, Intel Corporation.
 */

pub const INTEL_PT_PMU_NAME: &[u8] = b"intel_pt\0";

pub const INTEL_PT_PMU_TYPE: u32 = 0;
pub const INTEL_PT_TIME_SHIFT: u32 = 1;
pub const INTEL_PT_TIME_MULT: u32 = 2;
pub const INTEL_PT_TIME_ZERO: u32 = 3;
pub const INTEL_PT_CAP_USER_TIME_ZERO: u32 = 4;
pub const INTEL_PT_TSC_BIT: u32 = 5;
pub const INTEL_PT_NORETCOMP_BIT: u32 = 6;
pub const INTEL_PT_HAVE_SCHED_SWITCH: u32 = 7;
pub const INTEL_PT_SNAPSHOT_MODE: u32 = 8;
pub const INTEL_PT_PER_CPU_MMAPS: u32 = 9;
pub const INTEL_PT_MTC_BIT: u32 = 10;
pub const INTEL_PT_MTC_FREQ_BITS: u32 = 11;
pub const INTEL_PT_TSC_CTC_N: u32 = 12;
pub const INTEL_PT_TSC_CTC_D: u32 = 13;
pub const INTEL_PT_CYC_BIT: u32 = 14;
pub const INTEL_PT_MAX_NONTURBO_RATIO: u32 = 15;
pub const INTEL_PT_FILTER_STR_LEN: u32 = 16;
pub const INTEL_PT_AUXTRACE_PRIV_MAX: u32 = 17;

#[repr(C)]
pub struct auxtrace_record {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_tool {
    _unused: [u8; 0],
}

#[repr(C)]
pub union perf_event {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_session {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_pmu {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn intel_pt_recording_init(err: *mut ::std::os::raw::c_int) -> *mut auxtrace_record;

    pub fn intel_pt_process_auxtrace_info(
        event: *mut perf_event,
        session: *mut perf_session,
    ) -> ::std::os::raw::c_int;

    pub fn intel_pt_pmu_default_config(
        intel_pt_pmu: *const perf_pmu,
        attr: *mut perf_event_attr,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
