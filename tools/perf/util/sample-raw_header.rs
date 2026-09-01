/* SPDX-License-Identifier: GPL-2.0 */

// C dependency intent: <stdbool.h>

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
pub union perf_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn evlist__s390_sample_raw(
        evlist: *mut evlist,
        event: *mut perf_event,
        sample: *mut perf_sample,
    );

    pub fn evlist__has_amd_ibs(evlist: *mut evlist) -> bool;

    pub fn evlist__amd_sample_raw(
        evlist: *mut evlist,
        event: *mut perf_event,
        sample: *mut perf_sample,
    );

    pub fn evlist__init_trace_event_sample_raw(evlist: *mut evlist, env: *mut perf_env);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
