// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright IBM Corp. 2018
 * Auxtrace support for s390 CPU-Measurement Sampling Facility
 *
 * Author(s):  Thomas Richter <tmricht@linux.ibm.com>
 */

// C header guard omitted in Rust.

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
pub struct auxtrace_record {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn s390_cpumsf_recording_init(
        err: *mut ::std::os::raw::c_int,
        s390_cpumsf_pmu: *mut perf_pmu,
    ) -> *mut auxtrace_record;

    pub fn s390_cpumsf_process_auxtrace_info(
        event: *mut perf_event,
        session: *mut perf_session,
    ) -> ::std::os::raw::c_int;
}
