/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */
/*
 * Linux DRM clients expose information through usage stats as documented in
 * Documentation/gpu/drm-usage-stats.rst (available online at
 * https://docs.kernel.org/gpu/drm-usage-stats.html). This is a tool like PMU
 * that exposes DRM information.
 */

/* Translated from C header: includes "pmu.h" and <stdbool.h>. */

use core::ffi::{c_char, c_int, c_void};

use crate::{
    evsel, parse_events_error, parse_events_terms, perf_event_attr, perf_pmu,
    perf_pmu_info, pmu_event_callback,
};

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn drm_pmu__exit(pmu: *mut perf_pmu);
    pub fn drm_pmu__have_event(pmu: *const perf_pmu, name: *const c_char) -> bool;
    pub fn drm_pmu__for_each_event(
        pmu: *const perf_pmu,
        state: *mut c_void,
        cb: pmu_event_callback,
    ) -> c_int;
    pub fn drm_pmu__num_events(pmu: *const perf_pmu) -> usize;
    pub fn drm_pmu__config_terms(
        pmu: *const perf_pmu,
        attr: *mut perf_event_attr,
        terms: *mut parse_events_terms,
        err: *mut parse_events_error,
    ) -> c_int;
    pub fn drm_pmu__check_alias(
        pmu: *const perf_pmu,
        terms: *mut parse_events_terms,
        info: *mut perf_pmu_info,
        err: *mut parse_events_error,
    ) -> c_int;

    pub fn perf_pmu__is_drm(pmu: *const perf_pmu) -> bool;
    pub fn evsel__is_drm(evsel: *const evsel) -> bool;

    pub fn perf_pmus__read_drm_pmus(pmus: *mut list_head) -> c_int;

    pub fn evsel__drm_pmu_open(
        evsel: *mut evsel,
        threads: *mut perf_thread_map,
        start_cpu_map_idx: c_int,
        end_cpu_map_idx: c_int,
    ) -> c_int;
    pub fn evsel__drm_pmu_read(
        evsel: *mut evsel,
        cpu_map_idx: c_int,
        thread: c_int,
    ) -> c_int;
}
