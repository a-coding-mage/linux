// SPDX-License-Identifier: GPL-2.0-only
//
// Translated from perf/util/metricgroup.h.
// C dependencies preserved by reference:
// - <linux/list.h>
// - <linux/rbtree.h>
// - "pmu-events/pmu-events.h"

use std::os::raw::{c_char, c_int, c_uint, c_void};

pub type rb_node = crate::rb_node;
pub type list_head = crate::list_head;
pub type pmu_metric_iter_fn = crate::pmu_metric_iter_fn;

#[repr(C)]
pub struct evlist {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct option {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct print_callbacks {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct rblist {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct cgroup {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct pmu_metrics_table {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct pmu_metric {
    _unused: [u8; 0],
}

/**
 * A node in a rblist keyed by the evsel. The global rblist of metric events
 * generally exists in evlist. The evsel is looked up in the rblist
 * yielding a list of metric_expr.
 */
#[repr(C)]
pub struct metric_event {
    pub nd: rb_node,
    pub evsel: *mut evsel,
    pub is_default: bool, /* the metric evsel from the Default metricgroup */
    pub head: list_head, /* list of metric_expr */
}

/**
 * A metric referenced by a metric_expr. When parsing a metric expression IDs
 * will be looked up, matching either a value (from metric_events) or a
 * metric_ref. A metric_ref will then be parsed recursively. The metric_refs and
 * metric_events need to be known before parsing so that their values may be
 * placed in the parse context for lookup.
 */
#[repr(C)]
pub struct metric_ref {
    pub metric_name: *const c_char,
    pub metric_expr: *const c_char,
}

/**
 * One in a list of metric_expr associated with an evsel. The data is used to
 * generate a metric value during stat output.
 */
#[repr(C)]
pub struct metric_expr {
    pub nd: list_head,
    /** The expression to parse, for example, "instructions/cycles". */
    pub metric_expr: *const c_char,
    /** The name of the meric such as "IPC". */
    pub metric_name: *const c_char,
    pub metric_threshold: *const c_char,
    /**
     * The "ScaleUnit" that scales and adds a unit to the metric during
     * output. For example, "6.4e-05MiB" means to scale the resulting metric
     * by 6.4e-05 (typically converting a unit like cache lines to something
     * more human intelligible) and then add "MiB" afterward when displayed.
     */
    pub metric_unit: *const c_char,
    /** Displayed metricgroup name of the Default metricgroup */
    pub default_metricgroup_name: *const c_char,
    /** Null terminated array of events used by the metric. */
    pub metric_events: *mut *mut evsel,
    /** Null terminated array of referenced metrics. */
    pub metric_refs: *mut metric_ref,
    /** A value substituted for '?' during parsing. */
    pub runtime: c_int,
}

extern "C" {
    pub fn metricgroup__lookup(
        metric_events: *mut rblist,
        evsel: *mut evsel,
        create: bool,
    ) -> *mut metric_event;

    pub fn metricgroup__parse_groups(
        perf_evlist: *mut evlist,
        pmu: *const c_char,
        cputype_filter: bool,
        str_: *const c_char,
        metric_no_group: bool,
        metric_no_merge: bool,
        metric_no_threshold: bool,
        user_requested_cpu_list: *const c_char,
        system_wide: bool,
        hardware_aware_grouping: bool,
    ) -> c_int;

    pub fn metricgroup__parse_groups_test(
        evlist: *mut evlist,
        table: *const pmu_metrics_table,
        str_: *const c_char,
        cputype_filter: bool,
    ) -> c_int;

    pub fn metricgroup__for_each_metric(
        table: *const pmu_metrics_table,
        fn_: pmu_metric_iter_fn,
        data: *mut c_void,
    ) -> c_int;

    pub fn metricgroup__has_metric_or_groups(
        pmu: *const c_char,
        metric_or_groups: *const c_char,
    ) -> bool;

    pub fn metricgroups__topdown_max_level() -> c_uint;

    pub fn arch_get_runtimeparam(pm: *const pmu_metric) -> c_int;

    pub fn metricgroup__rblist_init(metric_events: *mut rblist);

    pub fn metricgroup__rblist_exit(metric_events: *mut rblist);

    pub fn metricgroup__copy_metric_events(
        evlist: *mut evlist,
        cgrp: *mut cgroup,
        new_metric_events: *mut rblist,
        old_metric_events: *mut rblist,
    ) -> c_int;
}
