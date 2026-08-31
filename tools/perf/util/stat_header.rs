/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from perf/util/stat.h. */
/* C includes removed: linux/types.h, stdio.h, sys/types.h, sys/resource.h, cpumap.h, counts.h. */

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type u32 = u32;
pub type u64 = u64;
pub type size_t = usize;
pub type FILE = c_void;

/* Forward declarations from other headers. */
pub enum perf_cpu_map {}
pub enum timespec {}
pub enum evsel {}
pub enum evlist {}
pub enum perf_tool {}
pub enum perf_session {}
pub enum target {}
pub enum metric_expr {}
pub enum perf_event {}

#[repr(C)]
pub struct stats {
    pub n: f64,
    pub mean: f64,
    pub M2: f64,
    pub max: u64,
    pub min: u64,
}

/* hold aggregated event info */
#[repr(C)]
pub struct perf_stat_aggr {
    /* aggregated values */
    pub counts: perf_counts_values,
    /* number of entries (CPUs) aggregated */
    pub nr: c_int,
    /* whether any entry has failed to read/process event */
    pub failed: bool,
    /* to mark this data is processed already */
    pub used: bool,
}

/* per-evsel event stats */
#[repr(C)]
pub struct perf_stat_evsel {
    /* used for repeated runs */
    pub res_stats: stats,
    /* number of allocated 'aggr' */
    pub nr_aggr: c_int,
    /* aggregated event values */
    pub aggr: *mut perf_stat_aggr,
    /* used for group read */
    pub group_data: *mut u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum aggr_mode {
    AGGR_NONE,
    AGGR_GLOBAL,
    AGGR_SOCKET,
    AGGR_DIE,
    AGGR_CLUSTER,
    AGGR_CACHE,
    AGGR_CORE,
    AGGR_THREAD,
    AGGR_UNSET,
    AGGR_NODE,
    AGGR_MAX,
}

pub type aggr_get_id_t =
    Option<unsafe extern "C" fn(config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id>;

#[repr(C)]
pub struct perf_stat_config {
    pub aggr_mode: aggr_mode,
    pub aggr_level: u32,
    pub scale: bool,
    pub no_inherit: bool,
    pub identifier: bool,
    pub csv_output: bool,
    pub json_output: bool,
    pub interval_clear: bool,
    pub metric_only: bool,
    pub null_run: bool,
    pub hide_zero: bool,
    pub ru_display: bool,
    pub big_num: bool,
    pub hybrid_merge: bool,
    pub walltime_run_table: bool,
    pub all_kernel: bool,
    pub all_user: bool,
    pub percore_show_thread: bool,
    pub summary: bool,
    pub no_csv_summary: bool,
    pub metric_no_group: bool,
    pub metric_no_merge: bool,
    pub metric_no_threshold: bool,
    pub hardware_aware_grouping: bool,
    pub stop_read_counter: bool,
    pub iostat_run: bool,
    pub user_requested_cpu_list: *mut c_char,
    pub system_wide: bool,
    pub output: *mut FILE,
    pub interval: c_uint,
    pub timeout: c_uint,
    pub unit_width: c_uint,
    pub metric_only_len: c_uint,
    pub times: c_int,
    pub run_count: c_int,
    pub print_free_counters_hint: c_int,
    pub csv_sep: *const c_char,
    pub walltime_nsecs_stats: *mut stats,
    pub ru_data: rusage,
    pub aggr_map: *mut cpu_aggr_map,
    pub aggr_get_id: aggr_get_id_t,
    pub cpus_aggr_map: *mut cpu_aggr_map,
    pub walltime_run: *mut u64,
    pub ctl_fd: c_int,
    pub ctl_fd_ack: c_int,
    pub ctl_fd_close: bool,
    pub cgroup_list: *const c_char,
    pub topdown_level: c_uint,
}

unsafe extern "C" {
    pub static mut stat_config: perf_stat_config;

    pub fn perf_stat__set_big_num(set: c_int);

    pub fn update_stats(stats: *mut stats, val: u64);
    pub fn avg_stats(stats: *mut stats) -> f64;
    pub fn stddev_stats(stats: *mut stats) -> f64;
    pub fn rel_stddev_stats(stddev: f64, avg: f64) -> f64;
}

#[inline]
pub unsafe fn init_stats(stats: *mut stats) {
    unsafe {
        (*stats).n = 0.0;
        (*stats).mean = 0.0;
        (*stats).M2 = 0.0;
        (*stats).min = -(1_i32 as i64) as u64;
        (*stats).max = 0;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum metric_threshold_classify {
    METRIC_THRESHOLD_UNKNOWN,
    METRIC_THRESHOLD_BAD,
    METRIC_THRESHOLD_NEARLY_BAD,
    METRIC_THRESHOLD_LESS_GOOD,
    METRIC_THRESHOLD_GOOD,
}

unsafe extern "C" {
    pub fn metric_threshold_classify__color(
        thresh: metric_threshold_classify,
    ) -> *const c_char;
}

pub type print_metric_t = Option<
    unsafe extern "C" fn(
        config: *mut perf_stat_config,
        ctx: *mut c_void,
        thresh: metric_threshold_classify,
        fmt: *const c_char,
        unit: *const c_char,
        val: f64,
    ),
>;

pub type new_line_t =
    Option<unsafe extern "C" fn(config: *mut perf_stat_config, ctx: *mut c_void)>;

/* Used to print the display name of the Default metricgroup for now. */
pub type print_metricgroup_header_t = Option<
    unsafe extern "C" fn(
        config: *mut perf_stat_config,
        ctx: *mut c_void,
        metricgroup_name: *const c_char,
    ),
>;

unsafe extern "C" {
    pub fn perf_stat__reset_shadow_stats();
}

#[repr(C)]
pub struct perf_stat_output_ctx {
    pub ctx: *mut c_void,
    pub print_metric: print_metric_t,
    pub new_line: new_line_t,
    pub print_metricgroup_header: print_metricgroup_header_t,
    pub force_header: bool,
}

unsafe extern "C" {
    pub fn perf_stat__print_shadow_stats(
        config: *mut perf_stat_config,
        evsel: *mut evsel,
        aggr_idx: c_int,
        out: *mut perf_stat_output_ctx,
    );
    pub fn perf_stat__skip_metric_event(evsel: *mut evsel) -> bool;
    pub fn perf_stat__print_shadow_stats_metricgroup(
        config: *mut perf_stat_config,
        evsel: *mut evsel,
        aggr_idx: c_int,
        num: *mut c_int,
        from: *mut c_void,
        out: *mut perf_stat_output_ctx,
    ) -> *mut c_void;

    pub fn evlist__alloc_stats(
        config: *mut perf_stat_config,
        evlist: *mut evlist,
        alloc_raw: bool,
    ) -> c_int;
    pub fn evlist__free_stats(evlist: *mut evlist);
    pub fn evlist__reset_stats(evlist: *mut evlist);
    pub fn evlist__reset_prev_raw_counts(evlist: *mut evlist);
    pub fn evlist__copy_prev_raw_counts(evlist: *mut evlist);
    pub fn evlist__save_aggr_prev_raw_counts(evlist: *mut evlist);

    pub fn evlist__alloc_aggr_stats(evlist: *mut evlist, nr_aggr: c_int) -> c_int;
    pub fn evlist__reset_aggr_stats(evlist: *mut evlist);
    pub fn evlist__copy_res_stats(config: *mut perf_stat_config, evlist: *mut evlist);

    pub fn perf_stat_process_counter(
        config: *mut perf_stat_config,
        counter: *mut evsel,
    ) -> c_int;
    pub fn perf_stat_merge_counters(config: *mut perf_stat_config, evlist: *mut evlist);
    pub fn perf_stat_process_percore(config: *mut perf_stat_config, evlist: *mut evlist);

    pub fn perf_event__process_stat_event(
        tool: *const perf_tool,
        session: *mut perf_session,
        event: *mut perf_event,
    ) -> c_int;

    pub fn perf_event__fprintf_stat(event: *mut perf_event, fp: *mut FILE) -> size_t;
    pub fn perf_event__fprintf_stat_round(event: *mut perf_event, fp: *mut FILE) -> size_t;
    pub fn perf_event__fprintf_stat_config(event: *mut perf_event, fp: *mut FILE) -> size_t;

    pub fn evlist__print_counters(
        evlist: *mut evlist,
        config: *mut perf_stat_config,
        _target: *mut target,
        ts: *mut timespec,
        argc: c_int,
        argv: *const *const c_char,
    );

    pub fn test_generic_metric(mexp: *mut metric_expr, aggr_idx: c_int) -> f64;
}
