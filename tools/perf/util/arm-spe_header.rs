/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Arm Statistical Profiling Extensions (SPE) support
 * Copyright (c) 2017-2018, Arm Ltd.
 */

pub const ARM_SPE_PMU_NAME: &[u8; 9] = b"arm_spe_\0";

pub const ARM_SPE_PMU_TYPE: u32 = 0;
pub const ARM_SPE_PER_CPU_MMAPS: u32 = 1;
pub const ARM_SPE_AUXTRACE_V1_PRIV_MAX: u32 = 2;

pub const ARM_SPE_AUXTRACE_V1_PRIV_SIZE: usize =
    (ARM_SPE_AUXTRACE_V1_PRIV_MAX as usize) * core::mem::size_of::<u64>();

/*
 * The old metadata format (defined above) does not include a
 * field for version number. Version 1 is reserved and starts
 * from version 2.
 */
pub const ARM_SPE_HEADER_VERSION: u32 = 0;
/* Number of sizeof(u64) */
pub const ARM_SPE_HEADER_SIZE: u32 = 1;
/* PMU type shared by CPUs */
pub const ARM_SPE_PMU_TYPE_V2: u32 = 2;
/* Number of CPUs */
pub const ARM_SPE_CPUS_NUM: u32 = 3;
pub const ARM_SPE_AUXTRACE_PRIV_MAX: u32 = 4;

/* Magic number */
pub const ARM_SPE_MAGIC: u32 = 0;
/* CPU logical number in system */
pub const ARM_SPE_CPU: u32 = 1;
/* Number of parameters */
pub const ARM_SPE_CPU_NR_PARAMS: u32 = 2;
/* CPU MIDR */
pub const ARM_SPE_CPU_MIDR: u32 = 3;
/* Associated PMU type */
pub const ARM_SPE_CPU_PMU_TYPE: u32 = 4;
/* Minimal interval */
pub const ARM_SPE_CAP_MIN_IVAL: u32 = 5;
/* Event filter */
pub const ARM_SPE_CAP_EVENT_FILTER: u32 = 6;
pub const ARM_SPE_CPU_PRIV_MAX: u32 = 7;

pub const ARM_SPE_HEADER_CURRENT_VERSION: u32 = 2;

#[repr(C)]
pub union perf_event {
    _bindgen_union_align: [u64; 0],
}

#[repr(C)]
pub struct perf_session {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_pmu {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct auxtrace_record {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn arm_spe_recording_init(
        err: *mut libc::c_int,
        arm_spe_pmu: *mut perf_pmu,
    ) -> *mut auxtrace_record;

    pub fn arm_spe_process_auxtrace_info(
        event: *mut perf_event,
        session: *mut perf_session,
    ) -> libc::c_int;

    pub fn arm_spe_pmu_default_config(
        arm_spe_pmu: *const perf_pmu,
        attr: *mut perf_event_attr,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
