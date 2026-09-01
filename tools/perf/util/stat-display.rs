/* Translated from perf/util/stat-display.c. */

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};

type FILE = c_void;
type u64 = u64;

const CNTR_NOT_SUPPORTED: *const c_char = b"<not supported>\0".as_ptr() as *const c_char;
const CNTR_NOT_COUNTED: *const c_char = b"<not counted>\0".as_ptr() as *const c_char;

const MGROUP_LEN: c_int = 50;
const METRIC_LEN: c_int = 38;
const EVNAME_LEN: c_int = 32;
const COUNTS_LEN: c_int = 18;
const INTERVAL_LEN: c_int = 16;
const CGROUP_LEN: c_int = 16;
const COMM_LEN: c_int = 16;
const PID_LEN: c_int = 7;
const CPUS_LEN: c_int = 4;

const NSEC_PER_SEC: c_double = 1000000000.0;
const USEC_PER_SEC: c_double = 1000000.0;

const PERF_COLOR_RED: *const c_char = b"\0".as_ptr() as *const c_char;
const PERF_COLOR_MAGENTA: *const c_char = b"\0".as_ptr() as *const c_char;
const PERF_COLOR_YELLOW: *const c_char = b"\0".as_ptr() as *const c_char;
const PERF_COLOR_GREEN: *const c_char = b"\0".as_ptr() as *const c_char;
const CONSOLE_CLEAR: *const c_char = b"\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aggr_mode {
    AGGR_CORE = 0,
    AGGR_CACHE = 1,
    AGGR_CLUSTER = 2,
    AGGR_DIE = 3,
    AGGR_SOCKET = 4,
    AGGR_NODE = 5,
    AGGR_NONE = 6,
    AGGR_THREAD = 7,
    AGGR_GLOBAL = 8,
    AGGR_UNSET = 9,
    AGGR_MAX = 10,
}

use aggr_mode::*;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum metric_threshold_classify {
    METRIC_THRESHOLD_UNKNOWN = 0,
    METRIC_THRESHOLD_BAD = 1,
    METRIC_THRESHOLD_NEARLY_BAD = 2,
    METRIC_THRESHOLD_LESS_GOOD = 3,
    METRIC_THRESHOLD_GOOD = 4,
}

use metric_threshold_classify::*;

static aggr_header_lens: [c_int; 11] = [18, 22, 20, 12, 6, 6, 6, 16, 0, 0, 0];

static aggr_header_csv: [*const c_char; 11] = [
    b"core,ctrs,\0".as_ptr() as *const c_char,
    b"cache,ctrs,\0".as_ptr() as *const c_char,
    b"cluster,ctrs,\0".as_ptr() as *const c_char,
    b"die,ctrs,\0".as_ptr() as *const c_char,
    b"socket,ctrs,\0".as_ptr() as *const c_char,
    b"node,\0".as_ptr() as *const c_char,
    b"cpu,\0".as_ptr() as *const c_char,
    b"comm-pid,\0".as_ptr() as *const c_char,
    b"\0".as_ptr() as *const c_char,
    core::ptr::null(),
    core::ptr::null(),
];

static aggr_header_std: [*const c_char; 11] = [
    b"core\0".as_ptr() as *const c_char,
    b"cache\0".as_ptr() as *const c_char,
    b"cluster\0".as_ptr() as *const c_char,
    b"die\0".as_ptr() as *const c_char,
    b"socket\0".as_ptr() as *const c_char,
    b"node\0".as_ptr() as *const c_char,
    b"cpu\0".as_ptr() as *const c_char,
    b"comm-pid\0".as_ptr() as *const c_char,
    b"\0".as_ptr() as *const c_char,
    core::ptr::null(),
    core::ptr::null(),
];

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aggr_cpu_id {
    pub socket: c_int,
    pub die: c_int,
    pub core: c_int,
    pub cache_lvl: c_int,
    pub cache: c_int,
    pub cluster: c_int,
    pub node: c_int,
    pub cpu: perf_cpu,
    pub thread_idx: c_int,
}

#[repr(C)]
pub struct cpu_aggr_map {
    pub nr: c_int,
    pub map: *mut aggr_cpu_id,
}

#[repr(C)]
pub struct perf_counts_values {
    pub val: u64,
    pub ena: u64,
    pub run: u64,
}

#[repr(C)]
pub struct perf_stat_aggr {
    pub counts: perf_counts_values,
    pub nr: c_int,
}

#[repr(C)]
pub struct stats;

#[repr(C)]
pub struct perf_stat_evsel {
    pub res_stats: stats,
    pub aggr: *mut perf_stat_aggr,
}

#[repr(C)]
pub struct perf_pmu {
    pub is_core: bool,
}

#[repr(C)]
pub struct cgroup {
    pub name: *const c_char,
}

#[repr(C)]
pub struct perf_cpu_map;

#[repr(C)]
pub struct perf_thread_map;

#[repr(C)]
pub struct evsel_core {
    pub threads: *mut perf_thread_map,
    pub cpus: *mut perf_cpu_map,
    pub is_pmu_core: bool,
}

#[repr(C)]
pub struct evlist_core {
    pub user_requested_cpus: *mut perf_cpu_map,
}

#[repr(C)]
pub struct evlist {
    pub core: evlist_core,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
    pub stats: *mut perf_stat_evsel,
    pub percore: bool,
    pub evlist: *mut evlist,
    pub cgrp: *mut cgroup,
    pub scale: c_double,
    pub supported: bool,
    pub unit: *const c_char,
    pub counts: *mut counts,
    pub pmu: *mut perf_pmu,
    pub skippable: bool,
    pub default_metricgroup: bool,
    pub default_show_events: bool,
    pub first_wildcard_match: *mut evsel,
    pub metric_leader: *mut evsel,
    pub priv_: *mut c_void,
}

#[repr(C)]
pub struct counts {
    pub scaled: c_int,
}

#[repr(C)]
pub struct timeval {
    pub tv_sec: c_double,
    pub tv_usec: c_double,
}

#[repr(C)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
}

pub type aggr_get_id_t = Option<unsafe extern "C" fn(*mut perf_stat_config, perf_cpu) -> aggr_cpu_id>;

#[repr(C)]
pub struct perf_stat_config {
    pub output: *mut FILE,
    pub csv_sep: *const c_char,
    pub json_output: bool,
    pub csv_output: bool,
    pub interval: bool,
    pub aggr_mode: aggr_mode,
    pub percore_show_thread: bool,
    pub run_count: c_int,
    pub cgroup_list: *mut c_void,
    pub metric_only: bool,
    pub metric_only_len: c_int,
    pub unit_width: c_int,
    pub iostat_run: bool,
    pub big_num: bool,
    pub print_free_counters_hint: c_int,
    pub hide_zero: bool,
    pub system_wide: bool,
    pub aggr_map: *mut cpu_aggr_map,
    pub aggr_get_id: aggr_get_id_t,
    pub hybrid_merge: bool,
    pub summary: bool,
    pub no_csv_summary: bool,
    pub interval_clear: bool,
    pub null_run: bool,
    pub walltime_nsecs_stats: *mut stats,
    pub walltime_run: *mut u64,
    pub walltime_run_table: bool,
    pub ru_display: bool,
    pub ru_data: rusage,
}

#[repr(C)]
pub struct target {
    pub bpf_str: *const c_char,
    pub system_wide: bool,
    pub cpu_list: *const c_char,
    pub pid: *const c_char,
    pub tid: *const c_char,
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

pub type print_metric_t = Option<unsafe extern "C" fn(*mut perf_stat_config, *mut c_void, metric_threshold_classify, *const c_char, *const c_char, c_double)>;
pub type new_line_t = Option<unsafe extern "C" fn(*mut perf_stat_config, *mut c_void)>;
pub type print_metricgroup_header_t = Option<unsafe extern "C" fn(*mut perf_stat_config, *mut c_void, *const c_char)>;

#[repr(C)]
pub struct perf_stat_output_ctx {
    pub print_metric: print_metric_t,
    pub new_line: new_line_t,
    pub print_metricgroup_header: print_metricgroup_header_t,
    pub ctx: *mut c_void,
    pub force_header: bool,
}

#[repr(C)]
pub struct outstate {
    /* Std mode: insert a newline before the next metric */
    pub newline: bool,
    /* JSON mode: track need for comma for a previous field or not */
    pub first: bool,
    /* Num CSV separators remaining to pad out when not all fields are printed */
    pub csv_col_pad: c_int,

    /*
     * The following don't track state across fields, but are here as a shortcut to
     * pass data to the print functions. The alternative would be to update the
     * function signatures of the entire print stack to pass them through.
     */
    /* Place to output to */
    pub fh: *mut FILE,
    /* Lines are timestamped in --interval-print mode */
    pub timestamp: [c_char; 64],
    /* Num items aggregated in current line. See struct perf_stat_aggr.nr */
    pub aggr_nr: c_int,
    /* Core/socket/die etc ID for the current line */
    pub id: aggr_cpu_id,
    /* Event for current line */
    pub evsel: *mut evsel,
    /* Cgroup for current line */
    pub cgrp: *mut cgroup,
}

extern "C" {
    static mut stdout: *mut FILE;
    static mut verbose: c_int;
    static mut nr_cgroups: c_int;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn puts(s: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn free(ptr: *mut c_void);
    fn abs(i: c_int) -> c_int;
    fn floor(x: c_double) -> c_double;
    fn isdigit(c: c_int) -> c_int;

    fn color_fprintf(fp: *mut FILE, color: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn color_snprintf(bf: *mut c_char, size: usize, color: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn rel_stddev_stats(stddev: c_double, avg: c_double) -> c_double;
    fn stddev_stats(stats: *mut stats) -> c_double;
    fn avg_stats(stats: *mut stats) -> c_double;
    fn perf_thread_map__comm(threads: *mut perf_thread_map, idx: c_int) -> *const c_char;
    fn perf_thread_map__pid(threads: *mut perf_thread_map, idx: c_int) -> c_int;
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
    fn perf_pmus__num_core_pmus() -> c_int;
    fn evsel__find_pmu(evsel: *mut evsel) -> *mut perf_pmu;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__selected(evlist: *mut evlist) -> *mut evsel;
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__uniquify_evsel_names(evlist: *mut evlist, config: *mut perf_stat_config);
    fn evlist__set_selected(evlist: *mut evlist, evsel: *mut evsel);
    fn evsel__cpus(evsel: *mut evsel) -> *mut perf_cpu_map;
    fn evsel__is_tool(evsel: *mut evsel) -> bool;
    fn evsel__is_hybrid(evsel: *mut evsel) -> bool;
    fn perf_stat__skip_metric_event(evsel: *mut evsel) -> bool;
    fn perf_stat__print_shadow_stats(config: *mut perf_stat_config, evsel: *mut evsel, aggr_idx: c_int, out: *mut perf_stat_output_ctx);
    fn perf_stat__print_shadow_stats_metricgroup(config: *mut perf_stat_config, evsel: *mut evsel, aggr_idx: c_int, num: *mut c_int, from: *mut c_void, out: *mut perf_stat_output_ctx) -> *mut c_void;
    fn aggr_cpu_id__equal(a: *const aggr_cpu_id, b: *const aggr_cpu_id) -> bool;
    fn aggr_cpu_id__cpu(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
    fn aggr_cpu_id__core(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
    fn cpu_aggr_map__empty_new(nr: c_int) -> *mut cpu_aggr_map;
    fn perf_cpu_map__has(map: *mut perf_cpu_map, cpu: perf_cpu) -> bool;
    fn perf_cpu_map__nr(map: *mut perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(map: *mut perf_cpu_map, idx: c_int) -> perf_cpu;
    fn skip_spaces(str: *mut c_char) -> *mut c_char;
    fn target__has_task(target: *mut target) -> bool;
    fn sysctl__nmi_watchdog_enabled() -> bool;
    fn iostat_print_header_prefix(config: *mut perf_stat_config);
    fn iostat_print_counters(evlist: *mut evlist, config: *mut perf_stat_config, ts: *mut timespec, timestamp: *mut c_char, cb: *mut c_void, data: *mut c_void);
    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn evlist__entry(evlist: *mut evlist, idx: c_int) -> *mut evsel;
}

unsafe fn cstr_is_null_or_empty(s: *const c_char) -> bool {
    s.is_null() || *s == 0
}

unsafe fn evlist_for_each_entry<F: FnMut(*mut evsel)>(evlist: *mut evlist, mut f: F) {
    let nr = evlist__nr_entries(evlist);
    let mut i = 0;
    while i < nr {
        f(evlist__entry(evlist, i));
        i += 1;
    }
}

unsafe fn cpu_aggr_map_for_each_idx<F: FnMut(c_int)>(map: *mut cpu_aggr_map, mut f: F) {
    let mut idx = 0;
    while idx < (*map).nr {
        f(idx);
        idx += 1;
    }
}

unsafe fn perf_cpu_map_for_each_cpu<F: FnMut(perf_cpu, c_uint)>(map: *mut perf_cpu_map, mut f: F) {
    let nr = perf_cpu_map__nr(map);
    let mut idx = 0;
    while idx < nr {
        f(perf_cpu_map__cpu(map, idx), idx as c_uint);
        idx += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn metric_threshold_classify__color(thresh: metric_threshold_classify) -> *const c_char {
    let colors = [
        b"\0".as_ptr() as *const c_char, /* unknown */
        PERF_COLOR_RED,                  /* bad */
        PERF_COLOR_MAGENTA,              /* nearly bad */
        PERF_COLOR_YELLOW,               /* less good */
        PERF_COLOR_GREEN,                /* good */
    ];
    colors[thresh as usize]
}

unsafe fn metric_threshold_classify__str(thresh: metric_threshold_classify) -> *const c_char {
    let strs = [
        b"unknown\0".as_ptr() as *const c_char,
        b"bad\0".as_ptr() as *const c_char,
        b"nearly bad\0".as_ptr() as *const c_char,
        b"less good\0".as_ptr() as *const c_char,
        b"good\0".as_ptr() as *const c_char,
    ];
    strs[thresh as usize]
}

unsafe fn print_running_std(config: *mut perf_stat_config, run: u64, ena: u64) {
    if run != ena {
        fprintf((*config).output, b"  (%.2f%%)\0".as_ptr() as *const c_char, 100.0 * run as c_double / ena as c_double);
    }
}

unsafe fn print_running_csv(config: *mut perf_stat_config, run: u64, ena: u64) {
    let mut enabled_percent: c_double = 100.0;
    if run != ena {
        enabled_percent = 100.0 * run as c_double / ena as c_double;
    }
    fprintf((*config).output, b"%s%llu%s%.2f\0".as_ptr() as *const c_char, (*config).csv_sep, run, (*config).csv_sep, enabled_percent);
}

unsafe fn json_sep(os: *mut outstate) -> *const c_char {
    let sep = if (*os).first { b"\0".as_ptr() as *const c_char } else { b", \0".as_ptr() as *const c_char };
    (*os).first = false;
    sep
}

macro_rules! json_out {
    ($os:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        fprintf((*$os).fh, concat!("%s", $fmt, "\0").as_ptr() as *const c_char, json_sep($os) $(, $arg)*)
    }};
}

unsafe fn print_running_json(os: *mut outstate, run: u64, ena: u64) {
    let mut enabled_percent: c_double = 100.0;
    if run != ena {
        enabled_percent = 100.0 * run as c_double / ena as c_double;
    }
    json_out!(os, "\"event-runtime\" : %llu, \"pcnt-running\" : %.2f", run, enabled_percent);
}

unsafe fn print_running(config: *mut perf_stat_config, os: *mut outstate, run: u64, ena: u64, before_metric: bool) {
    if (*config).json_output {
        if before_metric {
            print_running_json(os, run, ena);
        }
    } else if (*config).csv_output {
        if before_metric {
            print_running_csv(config, run, ena);
        }
    } else if !before_metric {
        print_running_std(config, run, ena);
    }
}

unsafe fn print_noise_pct_std(config: *mut perf_stat_config, pct: c_double) {
    if pct != 0.0 {
        fprintf((*config).output, b"  ( +-%6.2f%% )\0".as_ptr() as *const c_char, pct);
    }
}

unsafe fn print_noise_pct_csv(config: *mut perf_stat_config, pct: c_double) {
    fprintf((*config).output, b"%s%.2f%%\0".as_ptr() as *const c_char, (*config).csv_sep, pct);
}

unsafe fn print_noise_pct_json(os: *mut outstate, pct: c_double) {
    json_out!(os, "\"variance\" : %.2f", pct);
}

unsafe fn print_noise_pct(config: *mut perf_stat_config, os: *mut outstate, total: c_double, avg: c_double, before_metric: bool) {
    let pct = rel_stddev_stats(total, avg);
    if (*config).json_output {
        if before_metric {
            print_noise_pct_json(os, pct);
        }
    } else if (*config).csv_output {
        if before_metric {
            print_noise_pct_csv(config, pct);
        }
    } else if !before_metric {
        print_noise_pct_std(config, pct);
    }
}

unsafe fn print_noise(config: *mut perf_stat_config, os: *mut outstate, evsel: *mut evsel, avg: c_double, before_metric: bool) {
    if (*config).run_count == 1 {
        return;
    }
    let ps = (*evsel).stats;
    print_noise_pct(config, os, stddev_stats(&mut (*ps).res_stats), avg, before_metric);
}

unsafe fn print_cgroup_std(config: *mut perf_stat_config, cgrp_name: *const c_char) {
    fprintf((*config).output, b" %-*s\0".as_ptr() as *const c_char, CGROUP_LEN, cgrp_name);
}

unsafe fn print_cgroup_csv(config: *mut perf_stat_config, cgrp_name: *const c_char) {
    fprintf((*config).output, b"%s%s\0".as_ptr() as *const c_char, (*config).csv_sep, cgrp_name);
}

unsafe fn print_cgroup_json(os: *mut outstate, cgrp_name: *const c_char) {
    json_out!(os, "\"cgroup\" : \"%s\"", cgrp_name);
}

unsafe fn print_cgroup(config: *mut perf_stat_config, os: *mut outstate, cgrp: *mut cgroup) {
    if nr_cgroups != 0 || !(*config).cgroup_list.is_null() {
        let cgrp_name = if !cgrp.is_null() { (*cgrp).name } else { b"\0".as_ptr() as *const c_char };
        if (*config).json_output {
            print_cgroup_json(os, cgrp_name);
        } else if (*config).csv_output {
            print_cgroup_csv(config, cgrp_name);
        } else {
            print_cgroup_std(config, cgrp_name);
        }
    }
}

unsafe fn print_aggr_id_std(config: *mut perf_stat_config, evsel: *mut evsel, id: aggr_cpu_id, aggr_nr: c_int) {
    let output = (*config).output;
    let idx = (*config).aggr_mode as usize;
    let mut buf = [0 as c_char; 128];
    match (*config).aggr_mode {
        AGGR_CORE => { snprintf(buf.as_mut_ptr(), buf.len(), b"S%d-D%d-C%d\0".as_ptr() as *const c_char, id.socket, id.die, id.core); }
        AGGR_CACHE => { snprintf(buf.as_mut_ptr(), buf.len(), b"S%d-D%d-L%d-ID%d\0".as_ptr() as *const c_char, id.socket, id.die, id.cache_lvl, id.cache); }
        AGGR_CLUSTER => { snprintf(buf.as_mut_ptr(), buf.len(), b"S%d-D%d-CLS%d\0".as_ptr() as *const c_char, id.socket, id.die, id.cluster); }
        AGGR_DIE => { snprintf(buf.as_mut_ptr(), buf.len(), b"S%d-D%d\0".as_ptr() as *const c_char, id.socket, id.die); }
        AGGR_SOCKET => { snprintf(buf.as_mut_ptr(), buf.len(), b"S%d\0".as_ptr() as *const c_char, id.socket); }
        AGGR_NODE => { snprintf(buf.as_mut_ptr(), buf.len(), b"N%d\0".as_ptr() as *const c_char, id.node); }
        AGGR_NONE => {
            if (*evsel).percore && !(*config).percore_show_thread {
                snprintf(buf.as_mut_ptr(), buf.len(), b"S%d-D%d-C%d \0".as_ptr() as *const c_char, id.socket, id.die, id.core);
                fprintf(output, b"%-*s \0".as_ptr() as *const c_char, aggr_header_lens[AGGR_CORE as usize], buf.as_ptr());
            } else if id.cpu.cpu > -1 {
                fprintf(output, b"CPU%-*d \0".as_ptr() as *const c_char, aggr_header_lens[AGGR_NONE as usize] - 3, id.cpu.cpu);
            }
            return;
        }
        AGGR_THREAD => {
            fprintf(output, b"%*s-%-*d \0".as_ptr() as *const c_char,
                    COMM_LEN, perf_thread_map__comm((*evsel).core.threads, id.thread_idx),
                    PID_LEN, perf_thread_map__pid((*evsel).core.threads, id.thread_idx));
            return;
        }
        _ => return,
    }
    fprintf(output, b"%-*s %*d \0".as_ptr() as *const c_char, aggr_header_lens[idx], buf.as_ptr(), 4, aggr_nr);
}

unsafe fn print_aggr_id_csv(config: *mut perf_stat_config, evsel: *mut evsel, id: aggr_cpu_id, aggr_nr: c_int) {
    let output = (*config).output;
    let sep = (*config).csv_sep;
    match (*config).aggr_mode {
        AGGR_CORE => { fprintf(output, b"S%d-D%d-C%d%s%d%s\0".as_ptr() as *const c_char, id.socket, id.die, id.core, sep, aggr_nr, sep); }
        AGGR_CACHE => { fprintf(output, b"S%d-D%d-L%d-ID%d%s%d%s\0".as_ptr() as *const c_char, id.socket, id.die, id.cache_lvl, id.cache, sep, aggr_nr, sep); }
        AGGR_CLUSTER => { fprintf(output, b"S%d-D%d-CLS%d%s%d%s\0".as_ptr() as *const c_char, id.socket, id.die, id.cluster, sep, aggr_nr, sep); }
        AGGR_DIE => { fprintf(output, b"S%d-D%d%s%d%s\0".as_ptr() as *const c_char, id.socket, id.die, sep, aggr_nr, sep); }
        AGGR_SOCKET => { fprintf(output, b"S%d%s%d%s\0".as_ptr() as *const c_char, id.socket, sep, aggr_nr, sep); }
        AGGR_NODE => { fprintf(output, b"N%d%s%d%s\0".as_ptr() as *const c_char, id.node, sep, aggr_nr, sep); }
        AGGR_NONE => {
            if (*evsel).percore && !(*config).percore_show_thread {
                fprintf(output, b"S%d-D%d-C%d%s\0".as_ptr() as *const c_char, id.socket, id.die, id.core, sep);
            } else if id.cpu.cpu > -1 {
                fprintf(output, b"CPU%d%s\0".as_ptr() as *const c_char, id.cpu.cpu, sep);
            }
        }
        AGGR_THREAD => {
            fprintf(output, b"%s-%d%s\0".as_ptr() as *const c_char,
                    perf_thread_map__comm((*evsel).core.threads, id.thread_idx),
                    perf_thread_map__pid((*evsel).core.threads, id.thread_idx), sep);
        }
        _ => {}
    }
}

unsafe fn print_aggr_id_json(config: *mut perf_stat_config, os: *mut outstate, evsel: *mut evsel, id: aggr_cpu_id, aggr_nr: c_int) {
    match (*config).aggr_mode {
        AGGR_CORE => { json_out!(os, "\"core\" : \"S%d-D%d-C%d\", \"counters\" : %d", id.socket, id.die, id.core, aggr_nr); }
        AGGR_CACHE => { json_out!(os, "\"cache\" : \"S%d-D%d-L%d-ID%d\", \"counters\" : %d", id.socket, id.die, id.cache_lvl, id.cache, aggr_nr); }
        AGGR_CLUSTER => { json_out!(os, "\"cluster\" : \"S%d-D%d-CLS%d\", \"counters\" : %d", id.socket, id.die, id.cluster, aggr_nr); }
        AGGR_DIE => { json_out!(os, "\"die\" : \"S%d-D%d\", \"counters\" : %d", id.socket, id.die, aggr_nr); }
        AGGR_SOCKET => { json_out!(os, "\"socket\" : \"S%d\", \"counters\" : %d", id.socket, aggr_nr); }
        AGGR_NODE => { json_out!(os, "\"node\" : \"N%d\", \"counters\" : %d", id.node, aggr_nr); }
        AGGR_NONE => {
            if (*evsel).percore && !(*config).percore_show_thread {
                json_out!(os, "\"core\" : \"S%d-D%d-C%d\"", id.socket, id.die, id.core);
            } else if id.cpu.cpu > -1 {
                json_out!(os, "\"cpu\" : \"%d\"", id.cpu.cpu);
            }
        }
        AGGR_THREAD => {
            json_out!(os, "\"thread\" : \"%s-%d\"",
                      perf_thread_map__comm((*evsel).core.threads, id.thread_idx),
                      perf_thread_map__pid((*evsel).core.threads, id.thread_idx));
        }
        _ => {}
    }
}

unsafe fn aggr_printout(config: *mut perf_stat_config, os: *mut outstate, evsel: *mut evsel, id: aggr_cpu_id, aggr_nr: c_int) {
    if (*config).json_output {
        print_aggr_id_json(config, os, evsel, id, aggr_nr);
    } else if (*config).csv_output {
        print_aggr_id_csv(config, evsel, id, aggr_nr);
    } else {
        print_aggr_id_std(config, evsel, id, aggr_nr);
    }
}

unsafe extern "C" fn new_line_std(_config: *mut perf_stat_config, ctx: *mut c_void) {
    let os = ctx as *mut outstate;
    (*os).newline = true;
}

unsafe fn __new_line_std_csv(config: *mut perf_stat_config, os: *mut outstate) {
    fputc('\n' as c_int, (*os).fh);
    if (*config).interval {
        fputs((*os).timestamp.as_ptr(), (*os).fh);
    }
    aggr_printout(config, os, (*os).evsel, (*os).id, (*os).aggr_nr);
}

unsafe fn __new_line_std(config: *mut perf_stat_config, os: *mut outstate) {
    fprintf((*os).fh, b"%*s\0".as_ptr() as *const c_char, COUNTS_LEN + EVNAME_LEN + (*config).unit_width + 2, b"\0".as_ptr() as *const c_char);
}

unsafe fn do_new_line_std(config: *mut perf_stat_config, os: *mut outstate) {
    __new_line_std_csv(config, os);
    if (*config).aggr_mode == AGGR_NONE {
        fprintf((*os).fh, b"        \0".as_ptr() as *const c_char);
    }
    __new_line_std(config, os);
}

unsafe extern "C" fn print_metric_std(config: *mut perf_stat_config, ctx: *mut c_void, thresh: metric_threshold_classify, fmt: *const c_char, unit: *const c_char, val: c_double) {
    let os = ctx as *mut outstate;
    let out = (*os).fh;
    let mut n: c_int;
    let newline = (*os).newline;
    let color = metric_threshold_classify__color(thresh);
    (*os).newline = false;
    if unit.is_null() || fmt.is_null() {
        fprintf(out, b"%-*s\0".as_ptr() as *const c_char, METRIC_LEN, b"\0".as_ptr() as *const c_char);
        return;
    }
    if newline {
        do_new_line_std(config, os);
    }
    n = fprintf(out, b" # \0".as_ptr() as *const c_char);
    if !color.is_null() {
        n += color_fprintf(out, color, fmt, val);
    } else {
        n += fprintf(out, fmt, val);
    }
    fprintf(out, b" %-*s\0".as_ptr() as *const c_char, METRIC_LEN - n - 1, unit);
}

unsafe extern "C" fn new_line_csv(config: *mut perf_stat_config, ctx: *mut c_void) {
    let os = ctx as *mut outstate;
    __new_line_std_csv(config, os);
    let mut i = 0;
    while i < (*os).csv_col_pad {
        fputs((*config).csv_sep, (*os).fh);
        i += 1;
    }
}

unsafe extern "C" fn print_metric_csv(config: *mut perf_stat_config, ctx: *mut c_void, _thresh: metric_threshold_classify, fmt: *const c_char, unit: *const c_char, val: c_double) {
    let os = ctx as *mut outstate;
    let out = (*os).fh;
    let mut buf = [0 as c_char; 64];
    if unit.is_null() || fmt.is_null() {
        fprintf(out, b"%s%s\0".as_ptr() as *const c_char, (*config).csv_sep, (*config).csv_sep);
        return;
    }
    snprintf(buf.as_mut_ptr(), buf.len(), fmt, val);
    let vals = skip_spaces(buf.as_mut_ptr());
    let mut ends = vals;
    while isdigit(*ends as c_int) != 0 || *ends == '.' as c_char {
        ends = ends.add(1);
    }
    *ends = 0;
    fprintf(out, b"%s%s%s%s\0".as_ptr() as *const c_char, (*config).csv_sep, vals, (*config).csv_sep, skip_spaces(unit as *mut c_char));
}

unsafe extern "C" fn print_metric_json(config: *mut perf_stat_config, ctx: *mut c_void, thresh: metric_threshold_classify, _fmt: *const c_char, unit: *const c_char, val: c_double) {
    let os = ctx as *mut outstate;
    let out = (*os).fh;
    if !unit.is_null() {
        json_out!(os, "\"metric-value\" : \"%f\", \"metric-unit\" : \"%s\"", val, unit);
        if thresh != METRIC_THRESHOLD_UNKNOWN {
            json_out!(os, "\"metric-threshold\" : \"%s\"", metric_threshold_classify__str(thresh));
        }
    }
    if !(*config).metric_only {
        fprintf(out, b"}\0".as_ptr() as *const c_char);
    }
}

unsafe extern "C" fn new_line_json(config: *mut perf_stat_config, ctx: *mut c_void) {
    let os = ctx as *mut outstate;
    fputs(b"\n{\0".as_ptr() as *const c_char, (*os).fh);
    (*os).first = true;
    if (*config).interval {
        json_out!(os, "%s", (*os).timestamp.as_ptr());
    }
    aggr_printout(config, os, (*os).evsel, (*os).id, (*os).aggr_nr);
}

unsafe extern "C" fn print_metricgroup_header_json(config: *mut perf_stat_config, ctx: *mut c_void, metricgroup_name: *const c_char) {
    if metricgroup_name.is_null() {
        return;
    }
    json_out!(ctx as *mut outstate, "\"metricgroup\" : \"%s\"}", metricgroup_name);
    new_line_json(config, ctx);
}

unsafe extern "C" fn print_metricgroup_header_csv(config: *mut perf_stat_config, ctx: *mut c_void, metricgroup_name: *const c_char) {
    let os = ctx as *mut outstate;
    let mut i = 0;
    if metricgroup_name.is_null() {
        while i < (*os).csv_col_pad - 2 {
            fputs((*config).csv_sep, (*os).fh);
            i += 1;
        }
        return;
    }
    while i < (*os).csv_col_pad {
        fputs((*config).csv_sep, (*os).fh);
        i += 1;
    }
    fprintf((*config).output, b"%s\0".as_ptr() as *const c_char, metricgroup_name);
    new_line_csv(config, ctx);
}

unsafe extern "C" fn print_metricgroup_header_std(config: *mut perf_stat_config, ctx: *mut c_void, metricgroup_name: *const c_char) {
    let os = ctx as *mut outstate;
    if metricgroup_name.is_null() {
        __new_line_std(config, os);
        return;
    }
    fprintf((*config).output, b" %*s\0".as_ptr() as *const c_char, (*config).metric_only_len, metricgroup_name);
}

unsafe extern "C" fn print_metric_only(config: *mut perf_stat_config, ctx: *mut c_void, thresh: metric_threshold_classify, fmt: *const c_char, unit: *const c_char, val: c_double) {
    let os = ctx as *mut outstate;
    let out = (*os).fh;
    let mut str_ = [0 as c_char; 1024];
    let color = metric_threshold_classify__color(thresh);
    if unit.is_null() {
        (*os).first = false;
        return;
    }
    let mlen = core::cmp::max(strlen(unit) as c_uint, (*config).metric_only_len as c_uint);
    let used_fmt = if fmt.is_null() { b"\0".as_ptr() as *const c_char } else { fmt };
    let olen = snprintf(str_.as_mut_ptr(), str_.len(), used_fmt, val);
    color_snprintf(str_.as_mut_ptr(), str_.len(), if color.is_null() { b"\0".as_ptr() as *const c_char } else { color }, used_fmt, val);
    fprintf(out, b"%*s%s\0".as_ptr() as *const c_char,
            core::cmp::max(mlen as c_int - olen, 1), b"\0".as_ptr() as *const c_char, str_.as_ptr());
    (*os).first = false;
}

unsafe extern "C" fn print_metric_only_csv(config: *mut perf_stat_config, ctx: *mut c_void, _thresh: metric_threshold_classify, fmt: *const c_char, unit: *const c_char, val: c_double) {
    let os = ctx as *mut outstate;
    let out = (*os).fh;
    let mut buf = [0 as c_char; 64];
    if unit.is_null() {
        return;
    }
    let used_fmt = if fmt.is_null() { b"\0".as_ptr() as *const c_char } else { fmt };
    snprintf(buf.as_mut_ptr(), buf.len(), used_fmt, val);
    let vals = skip_spaces(buf.as_mut_ptr());
    let mut ends = vals;
    while isdigit(*ends as c_int) != 0 || *ends == '.' as c_char {
        ends = ends.add(1);
    }
    *ends = 0;
    fprintf(out, b"%s%s\0".as_ptr() as *const c_char, vals, (*config).csv_sep);
    (*os).first = false;
}

unsafe extern "C" fn print_metric_only_json(_config: *mut perf_stat_config, ctx: *mut c_void, _thresh: metric_threshold_classify, fmt: *const c_char, unit: *const c_char, val: c_double) {
    let os = ctx as *mut outstate;
    let mut buf = [0 as c_char; 64];
    if unit.is_null() || *unit == 0 {
        return;
    }
    let used_fmt = if fmt.is_null() { b"\0".as_ptr() as *const c_char } else { fmt };
    snprintf(buf.as_mut_ptr(), buf.len(), used_fmt, val);
    let mut vals = skip_spaces(buf.as_mut_ptr());
    let mut ends = vals;
    while isdigit(*ends as c_int) != 0 || *ends == '.' as c_char {
        ends = ends.add(1);
    }
    *ends = 0;
    if *vals == 0 {
        vals = b"none\0".as_ptr() as *mut c_char;
    }
    json_out!(os, "\"%s\" : \"%s\"", unit, vals);
}

unsafe extern "C" fn print_metric_header(config: *mut perf_stat_config, ctx: *mut c_void, _thresh: metric_threshold_classify, _fmt: *const c_char, unit: *const c_char, _val: c_double) {
    let os = ctx as *mut outstate;
    /* In case of iostat, print metric header for first root port only */
    if (*config).iostat_run && (*(*os).evsel).priv_ != (*evlist__selected((*(*os).evsel).evlist)).priv_ {
        return;
    }
    if (*(*os).evsel).cgrp != (*os).cgrp {
        return;
    }
    if unit.is_null() {
        return;
    }
    if (*config).json_output {
        return;
    } else if (*config).csv_output {
        fprintf((*os).fh, b"%s%s\0".as_ptr() as *const c_char, unit, (*config).csv_sep);
    } else {
        fprintf((*os).fh, b"%*s \0".as_ptr() as *const c_char, (*config).metric_only_len, unit);
    }
}

unsafe fn print_counter_value_std(config: *mut perf_stat_config, evsel: *mut evsel, avg: c_double, ok: bool) {
    let output = (*config).output;
    let sc = (*evsel).scale;
    let fmt = if (*config).big_num {
        if floor(sc) != sc { b"%'*.2f \0".as_ptr() as *const c_char } else { b"%'*.0f \0".as_ptr() as *const c_char }
    } else if floor(sc) != sc {
        b"%*.2f \0".as_ptr() as *const c_char
    } else {
        b"%*.0f \0".as_ptr() as *const c_char
    };
    let bad_count = if (*evsel).supported { CNTR_NOT_COUNTED } else { CNTR_NOT_SUPPORTED };
    if ok {
        fprintf(output, fmt, COUNTS_LEN, avg);
    } else {
        fprintf(output, b"%*s \0".as_ptr() as *const c_char, COUNTS_LEN, bad_count);
    }
    if !(*evsel).unit.is_null() {
        fprintf(output, b"%-*s \0".as_ptr() as *const c_char, (*config).unit_width, (*evsel).unit);
    }
    fprintf(output, b"%-*s\0".as_ptr() as *const c_char, EVNAME_LEN, evsel__name(evsel));
}

unsafe fn print_counter_value_csv(config: *mut perf_stat_config, evsel: *mut evsel, avg: c_double, ok: bool) {
    let output = (*config).output;
    let sc = (*evsel).scale;
    let sep = (*config).csv_sep;
    let fmt = if floor(sc) != sc { b"%.2f%s\0".as_ptr() as *const c_char } else { b"%.0f%s\0".as_ptr() as *const c_char };
    let bad_count = if (*evsel).supported { CNTR_NOT_COUNTED } else { CNTR_NOT_SUPPORTED };
    if ok {
        fprintf(output, fmt, avg, sep);
    } else {
        fprintf(output, b"%s%s\0".as_ptr() as *const c_char, bad_count, sep);
    }
    if !(*evsel).unit.is_null() {
        fprintf(output, b"%s%s\0".as_ptr() as *const c_char, (*evsel).unit, sep);
    }
    fprintf(output, b"%s\0".as_ptr() as *const c_char, evsel__name(evsel));
}

unsafe fn print_counter_value_json(os: *mut outstate, evsel: *mut evsel, avg: c_double, ok: bool) {
    let bad_count = if (*evsel).supported { CNTR_NOT_COUNTED } else { CNTR_NOT_SUPPORTED };
    if ok {
        json_out!(os, "\"counter-value\" : \"%f\"", avg);
    } else {
        json_out!(os, "\"counter-value\" : \"%s\"", bad_count);
    }
    if !(*evsel).unit.is_null() {
        json_out!(os, "\"unit\" : \"%s\"", (*evsel).unit);
    }
    json_out!(os, "\"event\" : \"%s\"", evsel__name(evsel));
}

unsafe fn print_counter_value(config: *mut perf_stat_config, os: *mut outstate, evsel: *mut evsel, avg: c_double, ok: bool) {
    if (*config).json_output {
        print_counter_value_json(os, evsel, avg, ok);
    } else if (*config).csv_output {
        print_counter_value_csv(config, evsel, avg, ok);
    } else {
        print_counter_value_std(config, evsel, avg, ok);
    }
}

unsafe fn abs_printout(config: *mut perf_stat_config, os: *mut outstate, id: aggr_cpu_id, aggr_nr: c_int, evsel: *mut evsel, avg: c_double, ok: bool) {
    aggr_printout(config, os, evsel, id, aggr_nr);
    print_counter_value(config, os, evsel, avg, ok);
    print_cgroup(config, os, (*evsel).cgrp);
}

unsafe fn evlist__has_hybrid_pmus(evlist: *mut evlist) -> bool {
    let mut last_core_pmu: *mut perf_pmu = core::ptr::null_mut();
    if perf_pmus__num_core_pmus() == 1 {
        return false;
    }
    let mut found = false;
    evlist_for_each_entry(evlist, |evsel| {
        if found {
            return;
        }
        if (*evsel).core.is_pmu_core {
            let pmu = evsel__find_pmu(evsel);
            if pmu == last_core_pmu {
                return;
            }
            if last_core_pmu.is_null() {
                last_core_pmu = pmu;
                return;
            }
            /* A distinct core PMU. */
            found = true;
        }
    });
    found
}

unsafe fn printout(config: *mut perf_stat_config, os: *mut outstate, uval: c_double, run: u64, ena: u64, noise: c_double, aggr_idx: c_int) {
    let mut out: perf_stat_output_ctx = core::mem::zeroed();
    let mut ok = true;
    let counter = (*os).evsel;

    if (*config).csv_output {
        out.print_metric = if (*config).metric_only { Some(print_metric_only_csv) } else { Some(print_metric_csv) };
        out.new_line = if (*config).metric_only { None } else { Some(new_line_csv) };
        out.print_metricgroup_header = Some(print_metricgroup_header_csv);
        (*os).csv_col_pad = 4 + if !(*counter).cgrp.is_null() { 1 } else { 0 };
    } else if (*config).json_output {
        out.print_metric = if (*config).metric_only { Some(print_metric_only_json) } else { Some(print_metric_json) };
        out.new_line = if (*config).metric_only { None } else { Some(new_line_json) };
        out.print_metricgroup_header = Some(print_metricgroup_header_json);
    } else {
        out.print_metric = if (*config).metric_only { Some(print_metric_only) } else { Some(print_metric_std) };
        out.new_line = if (*config).metric_only { None } else { Some(new_line_std) };
        out.print_metricgroup_header = Some(print_metricgroup_header_std);
    }

    if run == 0 || ena == 0 || (*(*counter).counts).scaled == -1 {
        ok = false;
        if (*counter).supported {
            if !evlist__has_hybrid_pmus((*counter).evlist) && !(*counter).pmu.is_null() && (*(*counter).pmu).is_core {
                (*config).print_free_counters_hint = 1;
            }
        }
    }

    out.ctx = os as *mut c_void;
    out.force_header = false;

    if !(*config).metric_only && (!(*counter).default_metricgroup || (*counter).default_show_events) {
        abs_printout(config, os, (*os).id, (*os).aggr_nr, counter, uval, ok);
        print_noise(config, os, counter, noise, true);
        print_running(config, os, run, ena, true);
    }

    if !(*config).metric_only && (*counter).default_metricgroup && !(*counter).default_show_events {
        let mut from: *mut c_void = core::ptr::null_mut();
        aggr_printout(config, os, (*os).evsel, (*os).id, (*os).aggr_nr);
        loop {
            let mut num: c_int = 0;
            if !from.is_null() {
                if (*config).json_output {
                    new_line_json(config, os as *mut c_void);
                } else {
                    __new_line_std_csv(config, os);
                }
            }
            print_noise(config, os, counter, noise, true);
            print_running(config, os, run, ena, true);
            from = perf_stat__print_shadow_stats_metricgroup(config, counter, aggr_idx, &mut num, from, &mut out);
            if from.is_null() {
                break;
            }
        }
    } else {
        perf_stat__print_shadow_stats(config, counter, aggr_idx, &mut out);
    }

    if !(*config).metric_only {
        print_noise(config, os, counter, noise, false);
        print_running(config, os, run, ena, false);
    }
}

/**
 * should_skip_zero_count() - Check if the event should print 0 values.
 * @config: The perf stat configuration (including aggregation mode).
 * @counter: The evsel with its associated cpumap.
 * @id: The aggregation id that is being queried.
 *
 * Due to mismatch between the event cpumap or thread-map and the
 * aggregation mode, sometimes it'd iterate the counter with the map
 * which does not contain any values.
 *
 * For example, uncore events have dedicated CPUs to manage them,
 * result for other CPUs should be zero and skipped.
 *
 * Return: %true if the value should NOT be printed, %false if the value
 * needs to be printed like "<not counted>" or "<not supported>".
 */
unsafe fn should_skip_zero_counter(config: *mut perf_stat_config, counter: *mut evsel, id: *const aggr_cpu_id) -> bool {
    if verbose == 0 && (*counter).skippable && !(*counter).supported {
        return true;
    }
    if (*config).metric_only {
        return false;
    }
    if (*config).hide_zero && (*counter).supported {
        return true;
    }
    if (*config).aggr_mode == AGGR_THREAD && (*config).system_wide {
        return true;
    }
    if (*config).aggr_map.is_null() || (*config).aggr_get_id.is_none() {
        return false;
    }
    if evsel__is_tool(counter) {
        let own_id = ((*config).aggr_get_id.unwrap())(config, perf_cpu { cpu: 0 });
        return !aggr_cpu_id__equal(id, &own_id);
    }
    let mut skip = true;
    perf_cpu_map_for_each_cpu((*counter).core.cpus, |cpu, _idx| {
        let own_id = ((*config).aggr_get_id.unwrap())(config, cpu);
        if aggr_cpu_id__equal(id, &own_id) {
            skip = false;
        }
    });
    skip
}

unsafe fn print_counter_aggrdata(config: *mut perf_stat_config, counter: *mut evsel, aggr_idx: c_int, os: *mut outstate) {
    let output = (*config).output;
    let ps = (*counter).stats;
    let aggr = (*ps).aggr.add(aggr_idx as usize);
    let id = *(*(*config).aggr_map).map.add(aggr_idx as usize);
    let avg = (*aggr).counts.val as c_double;
    (*os).id = id;
    (*os).aggr_nr = (*aggr).nr;
    (*os).evsel = counter;

    /* Skip already merged uncore/hybrid events */
    if (*config).aggr_mode != AGGR_NONE {
        if evsel__is_hybrid(counter) {
            if (*config).hybrid_merge && !(*counter).first_wildcard_match.is_null() {
                return;
            }
        } else if !(*counter).first_wildcard_match.is_null() {
            return;
        }
    }

    let val = (*aggr).counts.val;
    let ena = (*aggr).counts.ena;
    let run = (*aggr).counts.run;

    if perf_stat__skip_metric_event(counter) {
        return;
    }
    if val == 0 && should_skip_zero_counter(config, counter, &id) {
        return;
    }
    if !(*config).metric_only {
        if (*config).json_output {
            (*os).first = true;
            fputc('{' as c_int, output);
        }
        if (*config).interval {
            if (*config).json_output {
                json_out!(os, "%s", (*os).timestamp.as_ptr());
            } else {
                fprintf(output, b"%s\0".as_ptr() as *const c_char, (*os).timestamp.as_ptr());
            }
        } else if (*config).summary && (*config).csv_output && !(*config).no_csv_summary {
            fprintf(output, b"%s%s\0".as_ptr() as *const c_char, b"summary\0".as_ptr() as *const c_char, (*config).csv_sep);
        }
    }

    let uval = val as c_double * (*counter).scale;
    printout(config, os, uval, run, ena, avg, aggr_idx);
    if !(*config).metric_only {
        fputc('\n' as c_int, output);
    }
}

unsafe fn print_metric_begin(config: *mut perf_stat_config, evlist: *mut evlist, os: *mut outstate, aggr_idx: c_int) {
    (*os).first = true;
    if !(*config).metric_only {
        return;
    }
    if (*config).json_output {
        fputc('{' as c_int, (*config).output);
    }
    if (*config).interval {
        if (*config).json_output {
            json_out!(os, "%s", (*os).timestamp.as_ptr());
        } else {
            fprintf((*config).output, b"%s\0".as_ptr() as *const c_char, (*os).timestamp.as_ptr());
        }
    }
    let evsel = evlist__first(evlist);
    let id = *(*(*config).aggr_map).map.add(aggr_idx as usize);
    let aggr = (*(*evsel).stats).aggr.add(aggr_idx as usize);
    aggr_printout(config, os, evsel, id, (*aggr).nr);
    print_cgroup(config, os, if !(*os).cgrp.is_null() { (*os).cgrp } else { (*evsel).cgrp });
}

unsafe fn print_metric_end(config: *mut perf_stat_config, os: *mut outstate) {
    let output = (*config).output;
    if !(*config).metric_only {
        return;
    }
    if (*config).json_output {
        if (*os).first {
            fputs(b"\"metric-value\" : \"none\"\0".as_ptr() as *const c_char, output);
        }
        fputc('}' as c_int, output);
    }
    fputc('\n' as c_int, output);
}

unsafe fn print_aggr(config: *mut perf_stat_config, evlist: *mut evlist, os: *mut outstate) {
    if (*config).aggr_map.is_null() || (*config).aggr_get_id.is_none() {
        return;
    }
    cpu_aggr_map_for_each_idx((*config).aggr_map, |aggr_idx| {
        print_metric_begin(config, evlist, os, aggr_idx);
        evlist_for_each_entry(evlist, |counter| {
            print_counter_aggrdata(config, counter, aggr_idx, os);
        });
        print_metric_end(config, os);
    });
}

unsafe fn print_aggr_cgroup(config: *mut perf_stat_config, evlist: *mut evlist, os: *mut outstate) {
    if (*config).aggr_map.is_null() || (*config).aggr_get_id.is_none() {
        return;
    }
    evlist_for_each_entry(evlist, |evsel| {
        if (*os).cgrp == (*evsel).cgrp {
            return;
        }
        (*os).cgrp = (*evsel).cgrp;
        cpu_aggr_map_for_each_idx((*config).aggr_map, |aggr_idx| {
            print_metric_begin(config, evlist, os, aggr_idx);
            evlist_for_each_entry(evlist, |counter| {
                if (*counter).cgrp != (*os).cgrp {
                    return;
                }
                print_counter_aggrdata(config, counter, aggr_idx, os);
            });
            print_metric_end(config, os);
        });
    });
}

unsafe fn print_counter(config: *mut perf_stat_config, counter: *mut evsel, os: *mut outstate) {
    if (*config).aggr_map.is_null() {
        return;
    }
    cpu_aggr_map_for_each_idx((*config).aggr_map, |aggr_idx| {
        print_counter_aggrdata(config, counter, aggr_idx, os);
    });
}

unsafe fn print_no_aggr_metric(config: *mut perf_stat_config, evlist: *mut evlist, os: *mut outstate) {
    perf_cpu_map_for_each_cpu((*evlist__core(evlist)).user_requested_cpus, |cpu, _all_idx| {
        let mut first = true;
        evlist_for_each_entry(evlist, |counter| {
            if !perf_cpu_map__has(evsel__cpus(counter), cpu) {
                return;
            }
            let mut aggr_idx = 0;
            while aggr_idx < (*(*config).aggr_map).nr {
                if (*(*(*config).aggr_map).map.add(aggr_idx as usize)).cpu.cpu == cpu.cpu {
                    break;
                }
                aggr_idx += 1;
            }
            (*os).evsel = counter;
            (*os).id = aggr_cpu_id__cpu(cpu, core::ptr::null_mut());
            if first {
                print_metric_begin(config, evlist, os, aggr_idx);
                first = false;
            }
            let ps = (*counter).stats;
            let aggr = (*ps).aggr.add(aggr_idx as usize);
            let val = (*aggr).counts.val;
            let ena = (*aggr).counts.ena;
            let run = (*aggr).counts.run;
            let uval = val as c_double * (*counter).scale;
            printout(config, os, uval, run, ena, 1.0, aggr_idx);
        });
        if !first {
            print_metric_end(config, os);
        }
    });
}

unsafe fn print_metric_headers_std(config: *mut perf_stat_config, no_indent: bool) {
    fputc(' ' as c_int, (*config).output);
    if !no_indent {
        let mut len = aggr_header_lens[(*config).aggr_mode as usize];
        if nr_cgroups != 0 || !(*config).cgroup_list.is_null() {
            len += CGROUP_LEN + 1;
        }
        fprintf((*config).output, b"%*s\0".as_ptr() as *const c_char, len, b"\0".as_ptr() as *const c_char);
    }
}

unsafe fn print_metric_headers_csv(config: *mut perf_stat_config, _no_indent: bool) {
    if (*config).interval {
        fprintf((*config).output, b"time%s\0".as_ptr() as *const c_char, (*config).csv_sep);
    }
    if (*config).iostat_run {
        return;
    }
    let mut p = aggr_header_csv[(*config).aggr_mode as usize];
    while *p != 0 {
        if *p == ',' as c_char {
            fputs((*config).csv_sep, (*config).output);
        } else {
            fputc(*p as c_int, (*config).output);
        }
        p = p.add(1);
    }
}

unsafe fn print_metric_headers_json(_config: *mut perf_stat_config, _no_indent: bool) {}

unsafe fn print_metric_headers(config: *mut perf_stat_config, evlist: *mut evlist, no_indent: bool) {
    let mut os: outstate = core::mem::zeroed();
    os.fh = (*config).output;
    let mut out = perf_stat_output_ctx {
        ctx: &mut os as *mut _ as *mut c_void,
        print_metric: Some(print_metric_header),
        new_line: None,
        print_metricgroup_header: None,
        force_header: true,
    };

    if (*config).json_output {
        print_metric_headers_json(config, no_indent);
    } else if (*config).csv_output {
        print_metric_headers_csv(config, no_indent);
    } else {
        print_metric_headers_std(config, no_indent);
    }
    if (*config).iostat_run {
        iostat_print_header_prefix(config);
    }
    if !(*config).cgroup_list.is_null() {
        os.cgrp = (*evlist__first(evlist)).cgrp;
    }
    evlist_for_each_entry(evlist, |counter| {
        if !(*config).iostat_run && (*config).aggr_mode != AGGR_NONE && (*counter).metric_leader != counter {
            return;
        }
        os.evsel = counter;
        perf_stat__print_shadow_stats(config, counter, 0, &mut out);
    });
    if !(*config).json_output {
        fputc('\n' as c_int, (*config).output);
    }
}

unsafe fn prepare_timestamp(config: *mut perf_stat_config, os: *mut outstate, ts: *mut timespec) {
    if (*config).iostat_run {
        return;
    }
    if (*config).json_output {
        scnprintf((*os).timestamp.as_mut_ptr(), (*os).timestamp.len(), b"\"interval\" : %lu.%09lu\0".as_ptr() as *const c_char,
                  (*ts).tv_sec as c_ulong, (*ts).tv_nsec as c_ulong);
    } else if (*config).csv_output {
        scnprintf((*os).timestamp.as_mut_ptr(), (*os).timestamp.len(), b"%lu.%09lu%s\0".as_ptr() as *const c_char,
                  (*ts).tv_sec as c_ulong, (*ts).tv_nsec as c_ulong, (*config).csv_sep);
    } else {
        scnprintf((*os).timestamp.as_mut_ptr(), (*os).timestamp.len(), b"%6lu.%09lu \0".as_ptr() as *const c_char,
                  (*ts).tv_sec as c_ulong, (*ts).tv_nsec as c_ulong);
    }
}

unsafe fn print_header_interval_std(config: *mut perf_stat_config, _target: *mut target, evlist: *mut evlist, _argc: c_int, _argv: *const *const c_char) {
    let output = (*config).output;
    match (*config).aggr_mode {
        AGGR_NODE | AGGR_SOCKET | AGGR_DIE | AGGR_CLUSTER | AGGR_CACHE | AGGR_CORE => {
            fprintf(output, b"#%*s %-*s ctrs\0".as_ptr() as *const c_char,
                    INTERVAL_LEN - 1, b"time\0".as_ptr() as *const c_char,
                    aggr_header_lens[(*config).aggr_mode as usize],
                    aggr_header_std[(*config).aggr_mode as usize]);
        }
        AGGR_NONE => {
            fprintf(output, b"#%*s %-*s\0".as_ptr() as *const c_char,
                    INTERVAL_LEN - 1, b"time\0".as_ptr() as *const c_char,
                    aggr_header_lens[(*config).aggr_mode as usize],
                    aggr_header_std[(*config).aggr_mode as usize]);
        }
        AGGR_THREAD => {
            fprintf(output, b"#%*s %*s-%-*s\0".as_ptr() as *const c_char,
                    INTERVAL_LEN - 1, b"time\0".as_ptr() as *const c_char,
                    COMM_LEN, b"comm\0".as_ptr() as *const c_char, PID_LEN, b"pid\0".as_ptr() as *const c_char);
        }
        AGGR_GLOBAL => {
            if !(*config).iostat_run {
                fprintf(output, b"#%*s\0".as_ptr() as *const c_char, INTERVAL_LEN - 1, b"time\0".as_ptr() as *const c_char);
            }
        }
        _ => {}
    }
    if (*config).metric_only {
        print_metric_headers(config, evlist, true);
    } else {
        fprintf(output, b" %*s %*s events\n\0".as_ptr() as *const c_char,
                COUNTS_LEN, b"counts\0".as_ptr() as *const c_char, (*config).unit_width, b"unit\0".as_ptr() as *const c_char);
    }
}

unsafe fn print_header_std(config: *mut perf_stat_config, target_: *mut target, evlist: *mut evlist, argc: c_int, argv: *const *const c_char) {
    let output = (*config).output;
    fprintf(output, b"\n\0".as_ptr() as *const c_char);
    fprintf(output, b" Performance counter stats for \0".as_ptr() as *const c_char);
    if !(*target_).bpf_str.is_null() {
        fprintf(output, b"'BPF program(s) %s\0".as_ptr() as *const c_char, (*target_).bpf_str);
    } else if (*target_).system_wide {
        fprintf(output, b"'system wide\0".as_ptr() as *const c_char);
    } else if !(*target_).cpu_list.is_null() {
        fprintf(output, b"'CPU(s) %s\0".as_ptr() as *const c_char, (*target_).cpu_list);
    } else if !target__has_task(target_) {
        let first = if !argv.is_null() { *argv } else { b"pipe\0".as_ptr() as *const c_char };
        fprintf(output, b"'%s\0".as_ptr() as *const c_char, first);
        let mut i = 1;
        while !argv.is_null() && i < argc {
            fprintf(output, b" %s\0".as_ptr() as *const c_char, *argv.add(i as usize));
            i += 1;
        }
    } else if !(*target_).pid.is_null() {
        fprintf(output, b"process id '%s\0".as_ptr() as *const c_char, (*target_).pid);
    } else {
        fprintf(output, b"thread id '%s\0".as_ptr() as *const c_char, (*target_).tid);
    }
    fprintf(output, b"'\0".as_ptr() as *const c_char);
    if (*config).run_count > 1 {
        fprintf(output, b" (%d runs)\0".as_ptr() as *const c_char, (*config).run_count);
    }
    fprintf(output, b":\n\n\0".as_ptr() as *const c_char);
    if (*config).metric_only {
        print_metric_headers(config, evlist, false);
    }
}

unsafe fn print_header_csv(config: *mut perf_stat_config, _target: *mut target, evlist: *mut evlist, _argc: c_int, _argv: *const *const c_char) {
    if (*config).metric_only {
        print_metric_headers(config, evlist, true);
    }
}

unsafe fn print_header_json(config: *mut perf_stat_config, _target: *mut target, evlist: *mut evlist, _argc: c_int, _argv: *const *const c_char) {
    if (*config).metric_only {
        print_metric_headers(config, evlist, true);
    }
}

unsafe fn print_header(config: *mut perf_stat_config, target_: *mut target, evlist: *mut evlist, argc: c_int, argv: *const *const c_char) {
    static mut NUM_PRINT_IV: c_int = 0;
    fflush(stdout);
    if (*config).interval_clear {
        puts(CONSOLE_CLEAR);
    }
    if NUM_PRINT_IV == 0 || (*config).interval_clear {
        if (*config).json_output {
            print_header_json(config, target_, evlist, argc, argv);
        } else if (*config).csv_output {
            print_header_csv(config, target_, evlist, argc, argv);
        } else if (*config).interval {
            print_header_interval_std(config, target_, evlist, argc, argv);
        } else {
            print_header_std(config, target_, evlist, argc, argv);
        }
    }
    if NUM_PRINT_IV == 25 {
        NUM_PRINT_IV = 0;
    }
    NUM_PRINT_IV += 1;
}

unsafe fn print_table(config: *mut perf_stat_config, output: *mut FILE, avg: c_double) {
    let mut tmp = [0 as c_char; 64];
    let mut indent: c_int = 0;
    scnprintf(tmp.as_mut_ptr(), 64, b" %17.9f\0".as_ptr() as *const c_char, avg);
    while tmp[indent as usize] == ' ' as c_char {
        indent += 1;
    }
    fprintf(output, b"%*s# Table of individual measurements:\n\0".as_ptr() as *const c_char, indent, b"\0".as_ptr() as *const c_char);
    let mut idx = 0;
    while idx < (*config).run_count {
        let run = *(*config).walltime_run.add(idx as usize) as c_double / NSEC_PER_SEC;
        let n = 1 + abs((100.0 * (run - avg) / run) as c_int / 5);
        fprintf(output, b" %17.9f (%+.9f) \0".as_ptr() as *const c_char, run, run - avg);
        let mut h = 0;
        while h < n {
            fprintf(output, b"#\0".as_ptr() as *const c_char);
            h += 1;
        }
        fprintf(output, b"\n\0".as_ptr() as *const c_char);
        idx += 1;
    }
    fprintf(output, b"\n%*s# Final result:\n\0".as_ptr() as *const c_char, indent, b"\0".as_ptr() as *const c_char);
}

unsafe fn timeval2double(t: *mut timeval) -> c_double {
    (*t).tv_sec + (*t).tv_usec / USEC_PER_SEC
}

unsafe fn print_footer(config: *mut perf_stat_config) {
    let avg = avg_stats((*config).walltime_nsecs_stats) / NSEC_PER_SEC;
    let output = (*config).output;
    if (*config).interval || (*config).csv_output || (*config).json_output {
        return;
    }
    if !(*config).null_run {
        fprintf(output, b"\n\0".as_ptr() as *const c_char);
    }
    if (*config).run_count == 1 {
        fprintf(output, b" %17.9f seconds time elapsed\0".as_ptr() as *const c_char, avg);
        if (*config).ru_display {
            let ru_utime = timeval2double(&mut (*config).ru_data.ru_utime);
            let ru_stime = timeval2double(&mut (*config).ru_data.ru_stime);
            fprintf(output, b"\n\n\0".as_ptr() as *const c_char);
            fprintf(output, b" %17.9f seconds user\n\0".as_ptr() as *const c_char, ru_utime);
            fprintf(output, b" %17.9f seconds sys\n\0".as_ptr() as *const c_char, ru_stime);
        }
    } else {
        let sd = stddev_stats((*config).walltime_nsecs_stats) / NSEC_PER_SEC;
        if (*config).walltime_run_table {
            print_table(config, output, avg);
        }
        fprintf(output, b" %17.9f +- %.9f seconds time elapsed\0".as_ptr() as *const c_char, avg, sd);
        print_noise_pct(config, core::ptr::null_mut(), sd, avg, false);
    }
    fprintf(output, b"\n\n\0".as_ptr() as *const c_char);
    if (*config).print_free_counters_hint != 0 && sysctl__nmi_watchdog_enabled() {
        fprintf(output,
                b"Some events weren't counted. Try disabling the NMI watchdog:\n\techo 0 > /proc/sys/kernel/nmi_watchdog\n\tperf stat ...\n\techo 1 > /proc/sys/kernel/nmi_watchdog\n\0".as_ptr() as *const c_char);
    }
}

unsafe fn print_percore(config: *mut perf_stat_config, counter: *mut evsel, os: *mut outstate) {
    let metric_only = (*config).metric_only;
    let output = (*config).output;
    if (*config).aggr_map.is_null() || (*config).aggr_get_id.is_none() {
        return;
    }
    if (*config).percore_show_thread {
        print_counter(config, counter, os);
        return;
    }
    let core_map = cpu_aggr_map__empty_new((*(*config).aggr_map).nr);
    if core_map.is_null() {
        fprintf(output, b"Cannot allocate per-core aggr map for display\n\0".as_ptr() as *const c_char);
        return;
    }
    let mut core_map_len = 0;
    cpu_aggr_map_for_each_idx((*config).aggr_map, |aggr_idx| {
        let curr_cpu = (*(*(*config).aggr_map).map.add(aggr_idx as usize)).cpu;
        let core_id = aggr_cpu_id__core(curr_cpu, core::ptr::null_mut());
        let mut found = false;
        let mut i = 0;
        while i < core_map_len {
            if aggr_cpu_id__equal((*core_map).map.add(i as usize), &core_id) {
                found = true;
                break;
            }
            i += 1;
        }
        if found {
            return;
        }
        print_counter_aggrdata(config, counter, aggr_idx, os);
        *(*core_map).map.add(core_map_len as usize) = core_id;
        core_map_len += 1;
    });
    free(core_map as *mut c_void);
    if metric_only {
        fputc('\n' as c_int, output);
    }
}

unsafe fn print_cgroup_counter(config: *mut perf_stat_config, evlist: *mut evlist, os: *mut outstate) {
    evlist_for_each_entry(evlist, |counter| {
        if (*os).cgrp != (*counter).cgrp {
            if !(*os).cgrp.is_null() {
                print_metric_end(config, os);
            }
            (*os).cgrp = (*counter).cgrp;
            print_metric_begin(config, evlist, os, 0);
        }
        print_counter(config, counter, os);
    });
    if !(*os).cgrp.is_null() {
        print_metric_end(config, os);
    }
}

#[no_mangle]
pub unsafe extern "C" fn evlist__print_counters(evlist: *mut evlist, config: *mut perf_stat_config, target_: *mut target, ts: *mut timespec, argc: c_int, argv: *const *const c_char) {
    let metric_only = (*config).metric_only;
    let mut os: outstate = core::mem::zeroed();
    os.fh = (*config).output;
    os.first = true;

    evlist__uniquify_evsel_names(evlist, config);
    if (*config).iostat_run {
        evlist__set_selected(evlist, evlist__first(evlist));
    }
    if (*config).interval {
        prepare_timestamp(config, &mut os, ts);
    }
    print_header(config, target_, evlist, argc, argv);

    match (*config).aggr_mode {
        AGGR_CORE | AGGR_CACHE | AGGR_CLUSTER | AGGR_DIE | AGGR_SOCKET | AGGR_NODE => {
            if !(*config).cgroup_list.is_null() {
                print_aggr_cgroup(config, evlist, &mut os);
            } else {
                print_aggr(config, evlist, &mut os);
            }
        }
        AGGR_THREAD | AGGR_GLOBAL => {
            if (*config).iostat_run {
                iostat_print_counters(evlist, config, ts, os.timestamp.as_mut_ptr(), print_counter as *mut c_void, &mut os as *mut _ as *mut c_void);
            } else if !(*config).cgroup_list.is_null() {
                print_cgroup_counter(config, evlist, &mut os);
            } else {
                print_metric_begin(config, evlist, &mut os, 0);
                evlist_for_each_entry(evlist, |counter| {
                    print_counter(config, counter, &mut os);
                });
                print_metric_end(config, &mut os);
            }
        }
        AGGR_NONE => {
            if metric_only {
                print_no_aggr_metric(config, evlist, &mut os);
            } else {
                evlist_for_each_entry(evlist, |counter| {
                    if (*counter).percore {
                        print_percore(config, counter, &mut os);
                    } else {
                        print_counter(config, counter, &mut os);
                    }
                });
            }
        }
        _ => {}
    }
    print_footer(config);
    fflush((*config).output);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
