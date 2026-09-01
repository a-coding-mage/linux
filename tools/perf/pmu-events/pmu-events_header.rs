/* SPDX-License-Identifier: GPL-2.0 */

use std::os::raw::{c_char, c_int, c_void};

pub enum perf_pmu {}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum aggr_mode_class {
    PerChip = 1,
    PerCore,
}

/**
 * enum metric_event_groups - How events within a pmu_metric should be grouped.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum metric_event_groups {
    /**
     * @MetricGroupEvents: Default, group events within the metric.
     */
    MetricGroupEvents = 0,
    /**
     * @MetricNoGroupEvents: Don't group events for the metric.
     */
    MetricNoGroupEvents = 1,
    /**
     * @MetricNoGroupEventsNmi:
     * Don't group events for the metric if the NMI watchdog is enabled.
     */
    MetricNoGroupEventsNmi = 2,
    /**
     * @MetricNoGroupEventsSmt:
     * Don't group events for the metric if SMT is enabled.
     */
    MetricNoGroupEventsSmt = 3,
    /**
     * @MetricNoGroupEventsThresholdAndNmi:
     * Don't group events for the metric thresholds and if the NMI watchdog
     * is enabled.
     */
    MetricNoGroupEventsThresholdAndNmi = 4,
}

/*
 * Describe each PMU event. Each CPU has a table of PMU events.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmu_event {
    pub name: *const c_char,
    pub compat: *const c_char,
    pub event: *const c_char,
    pub desc: *const c_char,
    pub topic: *const c_char,
    pub long_desc: *const c_char,
    pub pmu: *const c_char,
    pub unit: *const c_char,
    pub retirement_latency_mean: *const c_char,
    pub retirement_latency_min: *const c_char,
    pub retirement_latency_max: *const c_char,
    pub perpkg: bool,
    pub deprecated: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmu_metric {
    pub pmu: *const c_char,
    pub metric_name: *const c_char,
    pub metric_group: *const c_char,
    pub metric_expr: *const c_char,
    pub metric_threshold: *const c_char,
    pub unit: *const c_char,
    pub compat: *const c_char,
    pub desc: *const c_char,
    pub long_desc: *const c_char,
    pub metricgroup_no_group: *const c_char,
    pub default_metricgroup_name: *const c_char,
    pub aggr_mode: aggr_mode_class,
    pub event_grouping: metric_event_groups,
    pub default_show_events: bool,
}

pub enum pmu_events_table {}
pub enum pmu_metrics_table {}

pub const PMU_EVENTS__NOT_FOUND: c_int = -1000;
pub const PMU_METRICS__NOT_FOUND: c_int = -1000;

pub type pmu_event_iter_fn = Option<
    unsafe extern "C" fn(
        pe: *const pmu_event,
        table: *const pmu_events_table,
        data: *mut c_void,
    ) -> c_int,
>;

pub type pmu_metric_iter_fn = Option<
    unsafe extern "C" fn(
        pm: *const pmu_metric,
        table: *const pmu_metrics_table,
        data: *mut c_void,
    ) -> c_int,
>;

pub type pmu_metrics_table_iter_t =
    Option<unsafe extern "C" fn(table: *const pmu_metrics_table, data: *mut c_void) -> c_int>;

unsafe extern "C" {
    pub fn pmu_events_table__for_each_event(
        table: *const pmu_events_table,
        pmu: *mut perf_pmu,
        fn_: pmu_event_iter_fn,
        data: *mut c_void,
    ) -> c_int;

    /*
     * Search for a table and entry matching with pmu__name_wildcard_match or any
     * tables if pmu is NULL. Each matching event has fn called on it. 0 implies to
     * success/continue the search while non-zero means to terminate. The special
     * value PMU_EVENTS__NOT_FOUND is used to indicate no event was found in one of
     * the tables which doesn't terminate the search of all tables.
     */
    pub fn pmu_events_table__find_event(
        table: *const pmu_events_table,
        pmu: *mut perf_pmu,
        name: *const c_char,
        fn_: pmu_event_iter_fn,
        data: *mut c_void,
    ) -> c_int;

    pub fn pmu_events_table__num_events(
        table: *const pmu_events_table,
        pmu: *mut perf_pmu,
    ) -> usize;

    pub fn pmu_metrics_table__for_each_metric(
        table: *const pmu_metrics_table,
        fn_: pmu_metric_iter_fn,
        data: *mut c_void,
    ) -> c_int;

    pub fn pmu_metrics_table__name(table: *const pmu_metrics_table) -> *const c_char;

    pub fn pmu_metrics_table__iterate_tables(
        fn_: pmu_metrics_table_iter_t,
        data: *mut c_void,
    ) -> c_int;

    /*
     * Search for a table and entry matching with pmu__name_wildcard_match or any
     * tables if pmu is NULL. Each matching metric has fn called on it. 0 implies to
     * success/continue the search while non-zero means to terminate. The special
     * value PMU_METRICS__NOT_FOUND is used to indicate no metric was found in one
     * of the tables which doesn't terminate the search of all tables.
     */
    pub fn pmu_metrics_table__find_metric(
        table: *const pmu_metrics_table,
        pmu: *mut perf_pmu,
        metric: *const c_char,
        fn_: pmu_metric_iter_fn,
        data: *mut c_void,
    ) -> c_int;

    pub fn perf_pmu__find_events_table(pmu: *mut perf_pmu) -> *const pmu_events_table;
    pub fn perf_pmu__default_core_events_table() -> *const pmu_events_table;
    pub fn pmu_metrics_table__find() -> *const pmu_metrics_table;
    pub fn pmu_metrics_table__default() -> *const pmu_metrics_table;
    pub fn find_core_events_table(
        arch: *const c_char,
        cpuid: *const c_char,
    ) -> *const pmu_events_table;
    pub fn find_core_metrics_table(
        arch: *const c_char,
        cpuid: *const c_char,
    ) -> *const pmu_metrics_table;
    pub fn pmu_for_each_core_event(fn_: pmu_event_iter_fn, data: *mut c_void) -> c_int;
    pub fn pmu_for_each_core_metric(fn_: pmu_metric_iter_fn, data: *mut c_void) -> c_int;

    pub fn find_sys_events_table(name: *const c_char) -> *const pmu_events_table;
    pub fn find_sys_metrics_table(name: *const c_char) -> *const pmu_metrics_table;
    pub fn pmu_for_each_sys_event(fn_: pmu_event_iter_fn, data: *mut c_void) -> c_int;
    pub fn pmu_for_each_sys_metric(fn_: pmu_metric_iter_fn, data: *mut c_void) -> c_int;

    pub fn describe_metricgroup(group: *const c_char) -> *const c_char;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
