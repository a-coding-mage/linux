/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * intel-bts.h: Intel Processor Trace support
 * Copyright (c) 2013-2014, Intel Corporation.
 */

pub const INTEL_BTS_PMU_NAME: &[u8; 10] = b"intel_bts\0";

pub const INTEL_BTS_PMU_TYPE: u32 = 0;
pub const INTEL_BTS_TIME_SHIFT: u32 = 1;
pub const INTEL_BTS_TIME_MULT: u32 = 2;
pub const INTEL_BTS_TIME_ZERO: u32 = 3;
pub const INTEL_BTS_CAP_USER_TIME_ZERO: u32 = 4;
pub const INTEL_BTS_SNAPSHOT_MODE: u32 = 5;
pub const INTEL_BTS_AUXTRACE_PRIV_MAX: u32 = 6;

pub const INTEL_BTS_AUXTRACE_PRIV_SIZE: usize =
    (INTEL_BTS_AUXTRACE_PRIV_MAX as usize) * core::mem::size_of::<u64>();

#[repr(C)]
pub struct auxtrace_record {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_tool {
    _private: [u8; 0],
}

#[repr(C)]
pub union perf_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_session {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn intel_bts_recording_init(err: *mut i32) -> *mut auxtrace_record;

    pub fn intel_bts_process_auxtrace_info(
        event: *mut perf_event,
        session: *mut perf_session,
    ) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
