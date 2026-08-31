/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency intent from C header: #include "pmu.h" */

use std::os::raw::{c_char, c_int};

pub enum evsel {}
pub enum perf_thread_map {}
pub enum print_callbacks {}
pub enum perf_pmu {}
pub enum perf_cpu_map {}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum tool_pmu_event {
    TOOL_PMU__EVENT_NONE = 0,
    TOOL_PMU__EVENT_DURATION_TIME,
    TOOL_PMU__EVENT_USER_TIME,
    TOOL_PMU__EVENT_SYSTEM_TIME,
    TOOL_PMU__EVENT_HAS_PMEM,
    TOOL_PMU__EVENT_NUM_CORES,
    TOOL_PMU__EVENT_NUM_CPUS,
    TOOL_PMU__EVENT_NUM_CPUS_ONLINE,
    TOOL_PMU__EVENT_NUM_DIES,
    TOOL_PMU__EVENT_NUM_PACKAGES,
    TOOL_PMU__EVENT_SLOTS,
    TOOL_PMU__EVENT_SMT_ON,
    TOOL_PMU__EVENT_SYSTEM_TSC_FREQ,
    TOOL_PMU__EVENT_CORE_WIDE,
    TOOL_PMU__EVENT_TARGET_CPU,

    TOOL_PMU__EVENT_MAX,
}

/*
 * C macro:
 * #define tool_pmu__for_each_event(ev) \
 *     for ((ev) = TOOL_PMU__EVENT_DURATION_TIME; (ev) < TOOL_PMU__EVENT_MAX; ev++)
 */

unsafe extern "C" {
    pub fn tool_pmu__event_to_str(ev: tool_pmu_event) -> *const c_char;
    pub fn tool_pmu__str_to_event(str_: *const c_char) -> tool_pmu_event;
    pub fn tool_pmu__skip_event(name: *const c_char) -> bool;
    pub fn tool_pmu__num_skip_events() -> c_int;

    pub fn tool_pmu__read_event(
        ev: tool_pmu_event,
        evsel: *mut evsel,
        system_wide: bool,
        user_requested_cpu_list: *const c_char,
        result: *mut u64,
    ) -> bool;

    pub fn tool_pmu__cpu_slots_per_cycle() -> u64;

    pub fn perf_pmu__is_tool(pmu: *const perf_pmu) -> bool;

    pub fn evsel__is_tool(evsel: *const evsel) -> bool;
    pub fn evsel__tool_event(evsel: *const evsel) -> tool_pmu_event;
    pub fn evsel__tool_pmu_event_name(evsel: *const evsel) -> *const c_char;
    pub fn evsel__tool_pmu_prepare_open(
        evsel: *mut evsel,
        cpus: *mut perf_cpu_map,
        nthreads: c_int,
    ) -> c_int;
    pub fn evsel__tool_pmu_open(
        evsel: *mut evsel,
        threads: *mut perf_thread_map,
        start_cpu_map_idx: c_int,
        end_cpu_map_idx: c_int,
    ) -> c_int;
    pub fn evsel__tool_pmu_enable_cpu(evsel: *mut evsel, cpu_map_idx: c_int) -> c_int;
    pub fn evsel__tool_pmu_enable(evsel: *mut evsel) -> c_int;
    pub fn evsel__tool_pmu_disable_cpu(evsel: *mut evsel, cpu_map_idx: c_int) -> c_int;
    pub fn evsel__tool_pmu_disable(evsel: *mut evsel) -> c_int;
    pub fn evsel__tool_pmu_read(evsel: *mut evsel, cpu_map_idx: c_int, thread: c_int) -> c_int;

    pub fn tool_pmu__new() -> *mut perf_pmu;
}
