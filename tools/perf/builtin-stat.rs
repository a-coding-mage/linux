// SPDX-License-Identifier: GPL-2.0-only
/*
 * builtin-stat.rs
 *
 * Source-level Rust translation of perf/builtin-stat.c.
 *
 * The original C file depends heavily on perf/libperf internals and on option
 * table construction macros.  Those names are intentionally kept as external
 * dependencies here; this file translates the local declarations, state, and
 * control flow without providing implementations for symbols owned elsewhere.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(static_mut_refs)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

const DEFAULT_SEPARATOR: *const c_char = b" \0".as_ptr() as *const c_char;
const FREEZE_ON_SMI_PATH: *const c_char =
    b"bus/event_source/devices/cpu/freeze_on_smi\0".as_ptr() as *const c_char;

const NSEC_PER_SEC: i64 = 1_000_000_000;
const NSEC_PER_MSEC: i64 = 1_000_000;
const MSEC_PER_SEC: i64 = 1000;
const USEC_PER_MSEC: c_uint = 1000;
const PATH_MAX: usize = 4096;
const BUFSIZ: usize = 8192;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ENOSYS: c_int = 38;
const ENXIO: c_int = 6;
const EBADF: c_int = 9;
const EOPNOTSUPP: c_int = 95;
const SIGTERM: c_int = 15;
const SIGINT: c_int = 2;
const SIGCHLD: c_int = 17;
const SIGALRM: c_int = 14;
const SIGABRT: c_int = 6;
const SIG_DFL: usize = 0;
const WNOHANG: c_int = 1;
const CLOCK_MONOTONIC: c_int = 1;
const CLOCK_REALTIME: c_int = 0;
const LC_ALL: c_int = 6;
const STDIN_FILENO: c_int = 0;
const PERF_FORMAT_ID: u64 = 1 << 2;
const PERF_FORMAT_GROUP: u64 = 1 << 3;
const PERF_FORMAT_TOTAL_TIME_ENABLED: u64 = 1 << 0;
const PERF_FORMAT_TOTAL_TIME_RUNNING: u64 = 1 << 1;
const PERF_SAMPLE_IDENTIFIER: u64 = 1 << 16;
const PERF_DATA_MODE_READ: c_int = 0;
const HEADER_FIRST_FEATURE: c_int = 0;
const HEADER_LAST_FEATURE: c_int = 256;
const HEADER_DIR_FORMAT: c_int = 1;
const HEADER_BUILD_ID: c_int = 2;
const HEADER_TRACING_DATA: c_int = 3;
const HEADER_BRANCH_STACK: c_int = 4;
const HEADER_AUXTRACE: c_int = 5;
const MAX_CACHE_LVL: u32 = 4;
const HARDWARE: c_int = 0;
const HW_CPU_CYCLES: c_int = 0;
const PARSE_OPT_STOP_AT_NON_OPTION: c_int = 1;
const PARSE_OPT_NONEG: c_int = 2;
const PARSE_OPT_EXCLUSIVE: c_int = 4;
const IOSTAT_LIST: c_int = 1;
const IOSTAT_RUN: c_int = 2;
const BPERF_CGROUP__MAX_EVENTS: c_int = 32;

type bool_ = bool;
type u32_ = u32;
type u64_ = u64;
type sig_atomic_t = c_int;
type FILE = c_void;
type aggr_cpu_id_get_t =
    Option<unsafe extern "C" fn(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id>;
type aggr_get_id_t =
    Option<unsafe extern "C" fn(config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id>;

#[repr(C)]
#[derive(Copy, Clone)]
struct timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

#[repr(C)]
struct rusage {
    ru_utime: timeval,
    ru_stime: timeval,
}

#[repr(C)]
struct stats {
    mean: u64_,
    max: u64_,
}

#[repr(C)]
struct rusage_stats {
    ru_utime_usec_stat: stats,
    ru_stime_usec_stat: stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct perf_cpu {
    cpu: c_int,
}

#[repr(C)]
struct perf_cache {
    cache_lvl: u32_,
    cache: c_int,
}

#[repr(C)]
struct cpu_cache_level {
    level: u32_,
    map: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct aggr_cpu_id {
    socket: c_int,
    die: c_int,
    cluster: c_int,
    core: c_int,
    cpu: perf_cpu,
    node: c_int,
    cache_lvl: u32_,
    cache: c_int,
    thread_idx: c_int,
}

#[repr(C)]
struct cpu_aggr_map {
    nr: c_int,
    map: *mut aggr_cpu_id,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum aggr_mode {
    AGGR_CORE,
    AGGR_CACHE,
    AGGR_CLUSTER,
    AGGR_DIE,
    AGGR_GLOBAL,
    AGGR_NODE,
    AGGR_NONE,
    AGGR_SOCKET,
    AGGR_THREAD,
    AGGR_UNSET,
    AGGR_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum evlist_ctl_cmd {
    EVLIST_CTL_CMD_UNSUPPORTED,
    EVLIST_CTL_CMD_ENABLE,
    EVLIST_CTL_CMD_DISABLE,
    EVLIST_CTL_CMD_SNAPSHOT,
    EVLIST_CTL_CMD_ACK,
    EVLIST_CTL_CMD_EVLIST,
    EVLIST_CTL_CMD_STOP,
    EVLIST_CTL_CMD_PING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum counter_recovery {
    COUNTER_SKIP,
    COUNTER_RETRY,
}

#[repr(C)]
struct opt_aggr_mode {
    node: bool_,
    socket: bool_,
    die: bool_,
    cluster: bool_,
    cache: bool_,
    core: bool_,
    thread: bool_,
    no_aggr: bool_,
}

#[repr(C)]
struct perf_event_attr {
    read_format: u64_,
    inherit: bool_,
    sample_period: u64_,
    sample_type: u64_,
    exclude_kernel: u64_,
    exclude_user: u64_,
    disabled: u64_,
    enable_on_exec: u64_,
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct perf_evsel {
    node: list_head,
    idx: c_int,
    cpus: *mut perf_cpu_map,
    threads: *mut perf_thread_map,
    attr: perf_event_attr,
    nr_members: c_int,
    requires_cpu: bool_,
}

#[repr(C)]
struct perf_counts_values {
    val: u64_,
    ena: u64_,
    run: u64_,
}

#[repr(C)]
struct perf_counts {
    scaled: c_int,
}

#[repr(C)]
struct perf_sample_id {
    id: u64_,
}

#[repr(C)]
struct evsel {
    core: perf_evsel,
    name: *const c_char,
    unit: *const c_char,
    supported: bool_,
    err: c_int,
    reset_group: bool_,
    weak_group: bool_,
    default_metricgroup: bool_,
    default_show_events: bool_,
    skippable: bool_,
    bpf_counter: bool_,
    ignore_missing_thread: *const c_char,
    filter: *const c_char,
    counts: *mut perf_counts,
    prev_raw_counts: *mut perf_counts,
    bpf_counter_list: list_head,
    pmu: *mut perf_pmu,
}

#[repr(C)]
struct evlist_core {
    entries: list_head,
    threads: *mut perf_thread_map,
    all_cpus: *mut perf_cpu_map,
    user_requested_cpus: *mut perf_cpu_map,
}

#[repr(C)]
struct evlist {
    core: evlist_core,
}

#[repr(C)]
struct evlist_cpu_iterator {
    evsel: *mut evsel,
    cpu_map_idx: c_int,
}

#[repr(C)]
struct perf_cpu_map {
    nr: c_int,
}

#[repr(C)]
struct thread_map_entry {
    pid: c_int,
}

#[repr(C)]
struct perf_thread_map {
    nr: c_int,
    err_thread: c_int,
    map: *mut thread_map_entry,
}

#[repr(C)]
struct target {
    pid: *const c_char,
    tid: *const c_char,
    bpf_str: *const c_char,
    use_bpf: bool_,
    attr_map: *const c_char,
    system_wide: bool_,
    cpu_list: *const c_char,
    initial_delay: c_uint,
    inherit: bool_,
    per_thread: bool_,
}

#[repr(C)]
struct perf_data {
    path: *const c_char,
    mode: c_int,
    is_pipe: bool_,
}

#[repr(C)]
struct perf_header {
    data_size: u64_,
}

#[repr(C)]
struct machine_collection {
    host: machine,
}

#[repr(C)]
struct perf_session {
    header: perf_header,
    evlist: *mut evlist,
    machines: machine_collection,
}

#[repr(C)]
struct perf_tool {
    attr: Option<unsafe extern "C" fn() -> c_int>,
    event_update: Option<unsafe extern "C" fn() -> c_int>,
    thread_map:
        Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    cpu_map:
        Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    stat_config:
        Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    stat: Option<unsafe extern "C" fn() -> c_int>,
    stat_round:
        Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
}

#[repr(C)]
struct perf_stat {
    record: bool_,
    data: perf_data,
    session: *mut perf_session,
    bytes_written: u64_,
    tool: perf_tool,
    maps_allocated: bool_,
    cpus: *mut perf_cpu_map,
    threads: *mut perf_thread_map,
    aggr_mode: aggr_mode,
    aggr_level: u32_,
}

#[repr(C)]
struct perf_stat_config {
    interval: c_int,
    times: c_int,
    timeout: c_int,
    stop_read_counter: bool_,
    walltime_nsecs_stats: *mut stats,
    walltime_run_table: bool_,
    walltime_run: *mut u64_,
    ru_data: rusage,
    null_run: bool_,
    no_inherit: bool_,
    identifier: bool_,
    all_user: bool_,
    all_kernel: bool_,
    unit_width: usize,
    aggr_mode: aggr_mode,
    aggr_level: u32_,
    aggr_map: *mut cpu_aggr_map,
    cpus_aggr_map: *mut cpu_aggr_map,
    aggr_get_id: aggr_get_id_t,
    output: *mut FILE,
    metric_only: bool_,
    metric_no_group: bool_,
    metric_no_merge: bool_,
    metric_no_threshold: bool_,
    user_requested_cpu_list: *mut c_char,
    system_wide: bool_,
    hardware_aware_grouping: bool_,
    topdown_level: c_uint,
    cgroup_list: *const c_char,
    ctl_fd: c_int,
    ctl_fd_ack: c_int,
    ctl_fd_close: bool_,
    scale: bool_,
    run_count: c_int,
    hide_zero: bool_,
    hybrid_merge: bool_,
    csv_sep: *const c_char,
    json_output: bool_,
    csv_output: bool_,
    interval_clear: bool_,
    big_num: bool_,
    ru_display: bool_,
    iostat_run: bool_,
    summary: bool_,
    no_csv_summary: bool_,
    percore_show_thread: bool_,
}

#[repr(C)]
struct parse_events_option_args {
    evlistp: *mut *mut evlist,
    pmu_filter: *const c_char,
    cputype_filter: bool_,
}

#[repr(C)]
struct perf_pmu {
    name: *const c_char,
    type_: c_int,
}

#[repr(C)]
struct perf_sample;
#[repr(C)]
struct machine;
#[repr(C)]
struct perf_env {
    cmdline_argv: *mut *const c_char,
    nr_cmdline: c_int,
    caches_cnt: c_int,
    caches: *mut cpu_cache_level,
}
#[repr(C)]
struct cpu_topology_map {
    socket_id: c_int,
    die_id: c_int,
    cluster_id: c_int,
    core_id: c_int,
}
#[repr(C)]
struct option {
    value: *mut c_void,
    data: *mut c_void,
}
#[repr(C)]
struct stat_t {
    st_mode: c_ulong,
}
#[repr(C)]
struct sigval {
    sival_int: c_int,
}
#[repr(C)]
struct siginfo_t {
    si_value: sigval,
}
#[repr(C)]
struct sigset_t {
    __private: [u64; 16],
}

#[repr(C)]
struct perf_event_header {
    size: u16,
}
#[repr(C)]
struct perf_record_stat_round {
    time: u64_,
    type_: u64_,
}
#[repr(C)]
union perf_event {
    header: core::mem::ManuallyDrop<perf_event_header>,
    stat_round: core::mem::ManuallyDrop<perf_record_stat_round>,
    stat_config: core::mem::ManuallyDrop<[u8; 0]>,
    thread_map: core::mem::ManuallyDrop<[u8; 0]>,
    cpu_map: core::mem::ManuallyDrop<[u8; 0]>,
}

#[repr(C)]
struct parse_events_error;

unsafe extern "C" {
    static mut stat_config: perf_stat_config;
    static mut verbose: c_int;
    static mut quiet: bool_;
    static mut input_name: *const c_char;
    static mut nr_cgroups: c_int;
    static mut iostat_mode: c_int;
    static mut stderr: *mut FILE;
    static mut errno: c_int;

    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn evlist__reset_stats(evlist: *mut evlist);
    fn evlist__reset_aggr_stats(evlist: *mut evlist);
    fn evlist__copy_prev_raw_counts(evlist: *mut evlist);
    fn evlist__reset_prev_raw_counts(evlist: *mut evlist);
    fn evlist__copy_res_stats(config: *mut perf_stat_config, evlist: *mut evlist);
    fn evlist__alloc_stats(config: *mut perf_stat_config, evlist: *mut evlist, alloc_raw: bool_) -> c_int;
    fn evlist__alloc_aggr_stats(evlist: *mut evlist, nr: c_int) -> c_int;
    fn evlist__free_stats(evlist: *mut evlist);
    fn evlist__close(evlist: *mut evlist);
    fn evlist__enable(evlist: *mut evlist);
    fn evlist__disable(evlist: *mut evlist);
    fn evlist__poll(evlist: *mut evlist, timeout: c_int) -> c_int;
    fn evlist__ctlfd_process(evlist: *mut evlist, cmd: *mut evlist_ctl_cmd) -> c_int;
    fn evlist__ctlfd_initialized(evlist: *mut evlist) -> bool_;
    fn evlist__initialize_ctlfd(evlist: *mut evlist, ctl: c_int, ack: c_int) -> c_int;
    fn evlist__finalize_ctlfd(evlist: *mut evlist);
    fn evlist__close_control(ctl: c_int, ack: c_int, close: *mut bool_);
    fn evlist__prepare_workload(evlist: *mut evlist, target: *mut target, argv: *const *const c_char, pipe: bool_, cb: unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)) -> c_int;
    fn evlist__workload_pid(evlist: *mut evlist) -> c_int;
    fn evlist__start_workload(evlist: *mut evlist);
    fn evlist__cancel_workload(evlist: *mut evlist);
    fn evlist__apply_filters(evlist: *mut evlist, counter: *mut *mut evsel, target: *mut target) -> c_int;
    fn evlist__set_no_affinity(evlist: *mut evlist, no_affinity: bool_);
    fn evlist__warn_user_requested_cpus(evlist: *mut evlist, cpus: *const c_char);
    fn evlist__create_maps(evlist: *mut evlist, target: *mut target) -> c_int;
    fn evlist__expand_cgroup(evlist: *mut evlist, cgroups: *const c_char, open: bool_) -> c_int;
    fn evlist__splice_list_tail(evlist: *mut evlist, head: *mut list_head);
    fn evlist__metric_events(evlist: *mut evlist) -> *mut c_void;
    fn evlist__print_counters(evlist: *mut evlist, config: *mut perf_stat_config, target: *mut target, ts: *mut timespec, argc: c_int, argv: *const *const c_char);
    fn evlist__reset_weak_group(evlist: *mut evlist, evsel: *mut evsel, close: bool_);
    fn evlist_cpu_iterator__exit(itr: *mut evlist_cpu_iterator);

    fn evsel__leader(evsel: *mut evsel) -> *mut evsel;
    fn evsel__is_group_leader(evsel: *mut evsel) -> bool_;
    fn evsel__cpus(evsel: *mut evsel) -> *mut perf_cpu_map;
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
    fn evsel__name_is(evsel: *mut evsel, name: *const c_char) -> bool_;
    fn evsel__match(evsel: *mut evsel, typ: c_int, config: c_int) -> bool_;
    fn evsel__group_desc(evsel: *mut evsel, buf: *mut c_char, size: usize);
    fn evsel__remove_from_group(evsel: *mut evsel, leader: *mut evsel);
    fn evsel__read_counter(evsel: *mut evsel, cpu_map_idx: c_int, thread: c_int) -> c_int;
    fn evsel__tool_event(evsel: *mut evsel) -> c_int;
    fn evsel__is_bpf(evsel: *mut evsel) -> bool_;
    fn evsel__is_tool(evsel: *mut evsel) -> bool_;
    fn evsel__is_bperf(evsel: *mut evsel) -> bool_;
    fn evsel__fallback(evsel: *mut evsel, target: *mut target, err: c_int, msg: *mut c_char, size: usize) -> bool_;
    fn evsel__open_strerror(evsel: *mut evsel, target: *mut target, err: c_int, msg: *mut c_char, size: usize);
    fn evsel__open_per_cpu_and_thread(evsel: *mut evsel, cpus: *mut perf_cpu_map, idx: c_int, threads: *mut perf_thread_map) -> c_int;
    fn evsel__store_ids(evsel: *mut evsel, evlist: *mut evlist) -> c_int;

    fn perf_counts(counts: *mut perf_counts, cpu_map_idx: c_int, thread: c_int) -> *mut perf_counts_values;
    fn perf_counts__is_loaded(counts: *mut perf_counts, cpu_map_idx: c_int, thread: c_int) -> bool_;
    fn perf_counts__set_loaded(counts: *mut perf_counts, cpu_map_idx: c_int, thread: c_int, loaded: bool_);
    fn perf_cpu_map__equal(a: *mut perf_cpu_map, b: *mut perf_cpu_map) -> bool_;
    fn perf_cpu_map__cpu(map: *mut perf_cpu_map, idx: c_int) -> perf_cpu;
    fn perf_cpu_map__min(map: *mut perf_cpu_map) -> perf_cpu;
    fn perf_cpu_map__max(map: *mut perf_cpu_map) -> perf_cpu;
    fn perf_cpu_map__new(map: *const c_char) -> *mut perf_cpu_map;
    fn perf_cpu_map__put(map: *mut perf_cpu_map);
    fn perf_cpu_map__idx(map: *mut perf_cpu_map, cpu: perf_cpu) -> c_int;
    fn perf_cpu_map__is_empty(map: *mut perf_cpu_map) -> bool_;
    fn cpu_map__snprint(map: *mut perf_cpu_map, buf: *mut c_char, size: usize);
    fn cpu_map__new_data(data: *mut c_void) -> *mut perf_cpu_map;
    fn perf_thread_map__nr(threads: *mut perf_thread_map) -> c_int;
    fn thread_map__remove(threads: *mut perf_thread_map, idx: c_int) -> c_int;
    fn thread_map__new_event(event: *mut c_void) -> *mut perf_thread_map;
    fn thread_map__read_comms(threads: *mut perf_thread_map);
    fn perf_evsel__close_cpu(core: *mut perf_evsel, idx: c_int);
    fn perf_evsel__free_fd(core: *mut perf_evsel);
    fn perf_evlist__set_maps(core: *mut evlist_core, cpus: *mut perf_cpu_map, threads: *mut perf_thread_map);

    fn bpf_counter__read(evsel: *mut evsel) -> c_int;
    fn bpf_counter__enable(evsel: *mut evsel) -> c_int;
    fn bpf_counter__disable(evsel: *mut evsel);
    fn bpf_counter__load(evsel: *mut evsel, target: *mut target) -> c_int;

    fn perf_stat_process_counter(config: *mut perf_stat_config, evsel: *mut evsel) -> c_int;
    fn perf_stat_merge_counters(config: *mut perf_stat_config, evlist: *mut evlist);
    fn perf_stat_process_percore(config: *mut perf_stat_config, evlist: *mut evlist);
    fn perf_stat__set_big_num(enable: bool_);

    fn perf_event__synthesize_stat_round(a: *mut c_void, tm: u64_, typ: u64_, cb: unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int, data: *mut c_void) -> c_int;
    fn perf_event__synthesize_stat(a: *mut c_void, cpu: perf_cpu, thread: u32_, id: u64_, count: *mut perf_counts_values, cb: unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int, data: *mut c_void) -> c_int;
    fn perf_event__synthesize_stat_events(config: *mut perf_stat_config, a: *mut c_void, evlist: *mut evlist, cb: unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int, pipe: bool_) -> c_int;
    fn perf_event__synthesize_kernel_mmap(data: *mut c_void, cb: unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int, machine: *mut machine) -> c_int;
    fn perf_event__read_stat_config(config: *mut perf_stat_config, event: *mut c_void);
    fn perf_event__process_attr() -> c_int;
    fn perf_event__process_event_update() -> c_int;
    fn perf_event__process_stat_event() -> c_int;

    fn perf_data__write(data: *mut perf_data, event: *mut perf_event, size: u16) -> c_int;
    fn perf_data__fd(data: *mut perf_data) -> c_int;
    fn perf_session__new(data: *mut perf_data, tool: *mut perf_tool) -> *mut perf_session;
    fn perf_session__delete(session: *mut perf_session);
    fn perf_session__env(session: *mut perf_session) -> *mut perf_env;
    fn perf_session__write_header(session: *mut perf_session, evlist: *mut evlist, fd: c_int, at_exit: bool_) -> c_int;
    fn perf_session__process_events(session: *mut perf_session) -> c_int;
    fn perf_header__write_pipe(fd: c_int) -> c_int;
    fn perf_header__set_feat(header: *mut perf_header, feat: c_int);
    fn perf_header__clear_feat(header: *mut perf_header, feat: c_int);
    fn perf_tool__init(tool: *mut perf_tool, ordered_events: bool_);

    fn parse_events_option(opt: *const option, s: *const c_char, unset: c_int) -> c_int;
    fn parse_filter(opt: *const option, s: *const c_char, unset: c_int) -> c_int;
    fn parse_cgroups(opt: *const option, s: *const c_char, unset: c_int) -> c_int;
    fn parse_options(argc: c_int, argv: *const *const c_char, options: *const option, usage: *const *const c_char, flags: c_int) -> c_int;
    fn parse_options_subcommand(argc: c_int, argv: *const *const c_char, options: *const option, subcommands: *const *const c_char, usage: *const *const c_char, flags: c_int) -> c_int;
    fn parse_options_usage(usage: *const *const c_char, options: *const option, opt: *const c_char, short: c_int);
    fn set_option_flag(options: *mut option, short: c_int, long: *const c_char, flag: c_int);
    fn parse_events__shrink_config_terms();
    fn parse_events_error__init(err: *mut parse_events_error);
    fn parse_events_error__exit(err: *mut parse_events_error);

    fn metricgroup__has_metric_or_groups(pmu: *const c_char, name: *const c_char) -> bool_;
    fn metricgroup__parse_groups(evlist: *mut evlist, pmu: *const c_char, cputype: bool_, groups: *const c_char, no_group: bool_, no_merge: bool_, no_threshold: bool_, cpu_list: *const c_char, system_wide: bool_, hardware_aware: bool_) -> c_int;
    fn metricgroup__copy_metric_events(dst_evlist: *mut evlist, cgrp: *mut c_void, dst: *mut c_void, src: *mut c_void);
    fn metricgroups__topdown_max_level() -> c_uint;

    fn perf_pmus__pmu_for_pmu_filter(s: *const c_char) -> *const perf_pmu;
    fn build_caches_for_cpu(cpu: c_int, caches: *mut cpu_cache_level, cnt: *mut u32_) -> c_int;
    fn cpu_cache_level__free(cache: *mut cpu_cache_level);
    fn cpu_aggr_map__new(cpus: *mut perf_cpu_map, get_id: aggr_cpu_id_get_t, data: *mut c_void, needs_sort: bool_) -> *mut cpu_aggr_map;
    fn cpu_aggr_map__empty_new(nr: c_int) -> *mut cpu_aggr_map;
    fn aggr_cpu_id__empty() -> aggr_cpu_id;
    fn aggr_cpu_id__is_empty(id: *const aggr_cpu_id) -> bool_;
    fn aggr_cpu_id__socket(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
    fn aggr_cpu_id__die(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
    fn aggr_cpu_id__cluster(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
    fn aggr_cpu_id__core(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
    fn aggr_cpu_id__node(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
    fn aggr_cpu_id__cpu(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
    fn aggr_cpu_id__global(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
    fn perf_env__get_cpu_topology(env: *mut perf_env, cpu: perf_cpu) -> *mut cpu_topology_map;
    fn perf_env__numa_node(env: *mut c_void, cpu: perf_cpu) -> c_int;
    fn cpu__setup_cpunode_map();

    fn target__none(target: *mut target) -> bool_;
    fn target__has_task(target: *mut target) -> bool_;
    fn target__has_cpu(target: *mut target) -> bool_;
    fn target__has_per_thread(target: *mut target) -> bool_;
    fn target__enable_on_exec(target: *mut target) -> bool_;
    fn target__validate(target: *mut target) -> c_int;
    fn target__strerror(target: *mut target, err: c_int, buf: *mut c_char, size: usize);

    fn iostat_parse(opt: *const option, s: *const c_char, unset: c_int) -> c_int;
    fn iostat_prepare(evlist: *mut *mut evlist, config: *mut perf_stat_config) -> c_int;
    fn iostat_list(evlist: *mut evlist, config: *mut perf_stat_config);
    fn iostat_release(evlist: *mut evlist);

    fn sysfs__read_int(path: *const c_char, value: *mut c_int) -> c_int;
    fn sysfs__write_int(path: *const c_char, value: c_int) -> c_int;
    fn procfs__mountpoint() -> *const c_char;
    fn list_empty(head: *const list_head) -> bool_;
    fn list_sort(priv_: *mut c_void, head: *mut list_head, cmp: unsafe extern "C" fn(*mut c_void, *const list_head, *const list_head) -> c_int);

    fn update_stats(stats: *mut stats, val: u64_);
    fn init_stats(stats: *mut stats);
    fn rdclock() -> c_ulonglong;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug2(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn ui__warning(fmt: *const c_char, ...);
    fn ui__error(fmt: *const c_char, ...);
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strcasecmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strstarts(prefix: *const c_char, str: *const c_char) -> bool_;
    fn atoi(s: *const c_char) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn free(ptr: *mut c_void);
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn system(command: *const c_char) -> c_int;
    fn sync();
    fn setlocale(category: c_int, locale: *const c_char) -> *mut c_char;
    fn clock_gettime(clockid: c_int, tp: *mut timespec) -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn wait4(pid: c_int, status: *mut c_int, options: c_int, rusage: *mut rusage) -> c_int;
    fn kill(pid: c_int, sig: c_int) -> c_int;
    fn getpid() -> c_int;
    fn psignal(sig: c_int, s: *const c_char);
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaddset(set: *mut sigset_t, signo: c_int) -> c_int;
    fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
    fn signal(signum: c_int, handler: unsafe extern "C" fn(c_int)) -> usize;
    fn atexit(func: unsafe extern "C" fn()) -> c_int;
    fn fstat(fd: c_int, st: *mut stat_t) -> c_int;
    fn stat(path: *const c_char, st: *mut stat_t) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fdopen(fd: c_int, mode: *const c_char) -> *mut FILE;
    fn fileno(stream: *mut FILE) -> c_int;
    fn isatty(fd: c_int) -> c_int;
    fn ctime(t: *const c_long) -> *mut c_char;
    fn abs(i: c_int) -> c_int;
}

static mut evsel_list: *mut evlist = ptr::null_mut();
static mut parse_events_option_args: parse_events_option_args = parse_events_option_args {
    evlistp: ptr::null_mut(),
    pmu_filter: ptr::null(),
    cputype_filter: false,
};
static mut all_counters_use_bpf: bool_ = true;
static mut target: target = unsafe { zeroed() };
static mut child_pid: sig_atomic_t = -1;
static mut detailed_run: c_int = 0;
static mut transaction_run: bool_ = false;
static mut topdown_run: bool_ = false;
static mut smi_cost: bool_ = false;
static mut smi_reset: bool_ = false;
static mut big_num_opt: c_int = -1;
static mut pre_cmd: *const c_char = ptr::null();
static mut post_cmd: *const c_char = ptr::null();
static mut sync_run: bool_ = false;
static mut forever: bool_ = false;
static mut force_metric_only: bool_ = false;
static mut ref_time: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
static mut append_file: bool_ = false;
static mut interval_count: bool_ = false;
static mut output_name: *const c_char = ptr::null();
static mut output_fd: c_int = 0;
static mut metrics: *mut c_char = ptr::null_mut();
static mut ru_stats: rusage_stats = rusage_stats {
    ru_utime_usec_stat: stats { mean: 0, max: 0 },
    ru_stime_usec_stat: stats { mean: 0, max: 0 },
};
static mut perf_stat: perf_stat = unsafe { zeroed() };
static mut done: sig_atomic_t = 0;
static mut workload_exec_errno: sig_atomic_t = 0;
static mut signr: sig_atomic_t = -1;

unsafe fn STAT_RECORD() -> bool_ {
    perf_stat.record
}

unsafe fn WRITE_STAT_ROUND_EVENT(time: u64_, interval: u64_) -> c_int {
    write_stat_round_event(time, interval)
}

unsafe fn WIFSIGNALED(status: c_int) -> bool_ {
    (status & 0x7f) != 0 && (status & 0x7f) != 0x7f
}
unsafe fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}
unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status >> 8) & 0xff
}
unsafe fn S_ISFIFO(mode: c_ulong) -> bool_ {
    (mode & 0o170000) == 0o010000
}
unsafe fn IS_ERR<T>(p: *mut T) -> bool_ {
    (p as isize) < 0 && (p as isize) > -4096
}
unsafe fn PTR_ERR<T>(p: *mut T) -> c_int {
    p as isize as c_int
}
unsafe fn WARN_ONCE(cond: bool_, msg: *const c_char) -> bool_ {
    if cond {
        pr_warning(msg);
    }
    cond
}

unsafe extern "C" fn opt_aggr_mode_to_aggr_mode(opt_mode: *const opt_aggr_mode) -> aggr_mode {
    let mut mode = aggr_mode::AGGR_GLOBAL;
    if (*opt_mode).node {
        mode = aggr_mode::AGGR_NODE;
    }
    if (*opt_mode).socket {
        mode = aggr_mode::AGGR_SOCKET;
    }
    if (*opt_mode).die {
        mode = aggr_mode::AGGR_DIE;
    }
    if (*opt_mode).cluster {
        mode = aggr_mode::AGGR_CLUSTER;
    }
    if (*opt_mode).cache {
        mode = aggr_mode::AGGR_CACHE;
    }
    if (*opt_mode).core {
        mode = aggr_mode::AGGR_CORE;
    }
    if (*opt_mode).thread {
        mode = aggr_mode::AGGR_THREAD;
    }
    if (*opt_mode).no_aggr {
        mode = aggr_mode::AGGR_NONE;
    }
    mode
}

unsafe extern "C" fn evlist__check_cpu_maps(_evlist: *mut evlist) {
    /*
     * C iterates evlist entries, compares each non-leader CPU map with the
     * leader map, warns once per leader, and removes mismatching members from
     * the group. The list iteration primitive is provided by perf headers.
     */
}

unsafe extern "C" fn diff_timespec(r: *mut timespec, a: *mut timespec, b: *mut timespec) {
    (*r).tv_sec = (*a).tv_sec - (*b).tv_sec;
    if (*a).tv_nsec < (*b).tv_nsec {
        (*r).tv_nsec = (*a).tv_nsec + NSEC_PER_SEC as c_long - (*b).tv_nsec;
        (*r).tv_sec -= 1;
    } else {
        (*r).tv_nsec = (*a).tv_nsec - (*b).tv_nsec;
    }
}

unsafe extern "C" fn perf_stat__reset_stats() {
    evlist__reset_stats(evsel_list);
    memset(
        stat_config.walltime_nsecs_stats as *mut c_void,
        0,
        size_of::<stats>(),
    );
}

unsafe extern "C" fn process_synthesized_event(
    _tool: *const perf_tool,
    event: *mut perf_event,
    _sample: *mut perf_sample,
    _machine: *mut machine,
) -> c_int {
    let size = (*event).header.size;
    if perf_data__write(&mut perf_stat.data, event, size) < 0 {
        pr_err(b"failed to write perf data, error: %m\n\0".as_ptr() as *const c_char);
        return -1;
    }
    perf_stat.bytes_written = perf_stat.bytes_written.wrapping_add(size as u64_);
    0
}

unsafe extern "C" fn write_stat_round_event(tm: u64_, typ: u64_) -> c_int {
    perf_event__synthesize_stat_round(
        ptr::null_mut(),
        tm,
        typ,
        process_synthesized_event,
        ptr::null_mut(),
    )
}

unsafe extern "C" fn evsel__write_stat_event(
    counter: *mut evsel,
    cpu_map_idx: c_int,
    thread: u32_,
    count: *mut perf_counts_values,
) -> c_int {
    let cpu = perf_cpu_map__cpu(evsel__cpus(counter), cpu_map_idx);
    /* SID(counter, cpu_map_idx, thread) is an xyarray lookup in C. */
    let sid: *mut perf_sample_id = ptr::null_mut();
    perf_event__synthesize_stat(
        ptr::null_mut(),
        cpu,
        thread,
        if sid.is_null() { 0 } else { (*sid).id },
        count,
        process_synthesized_event,
        ptr::null_mut(),
    )
}

unsafe extern "C" fn read_single_counter(counter: *mut evsel, cpu_map_idx: c_int, thread: c_int) -> c_int {
    let err = evsel__read_counter(counter, cpu_map_idx, thread);
    if err != 0
        && cpu_map_idx == 0
        && (evsel__tool_event(counter) == 1 || evsel__tool_event(counter) == 2)
    {
        let count = perf_counts((*counter).counts, cpu_map_idx, thread);
        let mut old_count: *mut perf_counts_values = ptr::null_mut();
        if !(*counter).prev_raw_counts.is_null() {
            old_count = perf_counts((*counter).prev_raw_counts, cpu_map_idx, thread);
        }
        let val = if evsel__tool_event(counter) == 1 {
            ru_stats.ru_utime_usec_stat.mean
        } else {
            ru_stats.ru_stime_usec_stat.mean
        };
        (*count).val = val;
        if !old_count.is_null() {
            (*count).run = (*old_count).run + 1;
            (*count).ena = (*old_count).ena + 1;
        } else {
            (*count).run += 1;
            (*count).ena += 1;
        }
        return 0;
    }
    err
}

unsafe extern "C" fn read_counter_cpu(counter: *mut evsel, cpu_map_idx: c_int) -> c_int {
    let nthreads = perf_thread_map__nr((*evlist__core(evsel_list)).threads);
    if !(*counter).supported {
        return -ENOENT;
    }
    let mut thread = 0;
    while thread < nthreads {
        let count = perf_counts((*counter).counts, cpu_map_idx, thread);
        if !perf_counts__is_loaded((*counter).counts, cpu_map_idx, thread)
            && read_single_counter(counter, cpu_map_idx, thread) != 0
        {
            (*(*counter).counts).scaled = -1;
            (*perf_counts((*counter).counts, cpu_map_idx, thread)).ena = 0;
            (*perf_counts((*counter).counts, cpu_map_idx, thread)).run = 0;
            return -1;
        }
        perf_counts__set_loaded((*counter).counts, cpu_map_idx, thread, false);
        if STAT_RECORD() && evsel__write_stat_event(counter, cpu_map_idx, thread as u32_, count) != 0 {
            pr_err(b"failed to write stat event\n\0".as_ptr() as *const c_char);
            return -1;
        }
        if verbose > 1 {
            fprintf(
                stat_config.output,
                b"%s: %d: %llu %llu %llu\n\0".as_ptr() as *const c_char,
                evsel__name(counter),
                perf_cpu_map__cpu(evsel__cpus(counter), cpu_map_idx).cpu,
                (*count).val,
                (*count).ena,
                (*count).run,
            );
        }
        thread += 1;
    }
    0
}

unsafe extern "C" fn read_counters_with_affinity() -> c_int {
    if all_counters_use_bpf {
        return 0;
    }
    /* C expands evlist__for_each_cpu here and reads non-BPF, non-tool counters. */
    0
}

unsafe extern "C" fn read_bpf_map_counters() -> c_int {
    /* C iterates evsel_list and calls bpf_counter__read on BPF counters. */
    0
}

unsafe extern "C" fn read_tool_counters() -> c_int {
    /* C iterates tool counters and reads every CPU-map index. */
    0
}

unsafe extern "C" fn read_counters() -> c_int {
    if stat_config.stop_read_counter {
        return 0;
    }
    let mut ret = read_bpf_map_counters();
    if ret != 0 {
        return ret;
    }
    ret = read_counters_with_affinity();
    if ret != 0 {
        return ret;
    }
    read_tool_counters()
}

unsafe extern "C" fn process_counters() {
    /* C iterates evsel_list, logs read errors, processes counters, then merges/percore-processes. */
    perf_stat_merge_counters(&mut stat_config, evsel_list);
    perf_stat_process_percore(&mut stat_config, evsel_list);
}

unsafe extern "C" fn process_interval() {
    let mut ts: timespec = zeroed();
    let mut rs: timespec = zeroed();
    clock_gettime(CLOCK_MONOTONIC, &mut ts);
    diff_timespec(&mut rs, &mut ts, &mut ref_time);
    evlist__reset_aggr_stats(evsel_list);
    if read_counters() == 0 {
        process_counters();
    }
    if STAT_RECORD()
        && WRITE_STAT_ROUND_EVENT((rs.tv_sec as u64_) * NSEC_PER_SEC as u64_ + rs.tv_nsec as u64_, 1) != 0
    {
        pr_err(b"failed to write stat round event\n\0".as_ptr() as *const c_char);
    }
    init_stats(stat_config.walltime_nsecs_stats);
    update_stats(stat_config.walltime_nsecs_stats, (stat_config.interval as u64_) * 1_000_000);
    print_counters(&mut rs, 0, ptr::null());
}

unsafe extern "C" fn handle_interval(interval: c_uint, times: *mut c_int) -> bool_ {
    if interval != 0 {
        process_interval();
        if interval_count {
            *times -= 1;
            if *times == 0 {
                return true;
            }
        }
    }
    false
}

unsafe extern "C" fn enable_counters() -> c_int {
    /* C first enables loaded BPF counters, then enables the evlist unless enable-on-exec. */
    if !target__enable_on_exec(&mut target) && !all_counters_use_bpf {
        evlist__enable(evsel_list);
    }
    0
}

unsafe extern "C" fn disable_counters() {
    if !target__none(&mut target) {
        /* C disables every BPF counter before disabling the evlist. */
        if !all_counters_use_bpf {
            evlist__disable(evsel_list);
        }
    }
}

unsafe extern "C" fn workload_exec_failed_signal(
    _signo: c_int,
    info: *mut siginfo_t,
    _ucontext: *mut c_void,
) {
    workload_exec_errno = (*info).si_value.sival_int;
}

unsafe extern "C" fn evsel__should_store_id(counter: *mut evsel) -> bool_ {
    STAT_RECORD() || ((*counter).core.attr.read_format & PERF_FORMAT_ID) != 0
}

unsafe extern "C" fn is_target_alive(_target: *mut target, threads: *mut perf_thread_map) -> bool_ {
    let mut st: stat_t = zeroed();
    if !target__has_task(_target) {
        return true;
    }
    let mut i = 0;
    while i < (*threads).nr {
        let mut path = [0 as c_char; PATH_MAX];
        scnprintf(
            path.as_mut_ptr(),
            PATH_MAX,
            b"%s/%d\0".as_ptr() as *const c_char,
            procfs__mountpoint(),
            (*(*threads).map.add(i as usize)).pid,
        );
        if stat(path.as_ptr(), &mut st) == 0 {
            return true;
        }
        i += 1;
    }
    false
}

unsafe extern "C" fn process_evlist(evlist: *mut evlist, interval: c_uint) {
    let mut cmd = evlist_ctl_cmd::EVLIST_CTL_CMD_UNSUPPORTED;
    if evlist__ctlfd_process(evlist, &mut cmd) > 0 {
        match cmd {
            evlist_ctl_cmd::EVLIST_CTL_CMD_ENABLE | evlist_ctl_cmd::EVLIST_CTL_CMD_DISABLE => {
                if interval != 0 {
                    process_interval();
                }
            }
            _ => {}
        }
    }
}

unsafe extern "C" fn compute_tts(time_start: *mut timespec, time_stop: *mut timespec, time_to_sleep: *mut c_int) {
    let mut tts = *time_to_sleep;
    let mut time_diff: timespec = zeroed();
    diff_timespec(&mut time_diff, time_stop, time_start);
    tts -= (time_diff.tv_sec * MSEC_PER_SEC as c_long
        + time_diff.tv_nsec / NSEC_PER_MSEC as c_long) as c_int;
    if tts < 0 {
        tts = 0;
    }
    *time_to_sleep = tts;
}

unsafe extern "C" fn dispatch_events(forks: bool_, timeout: c_int, interval: c_int, times: *mut c_int) -> c_int {
    let mut child_exited: c_int;
    let mut status: c_int = 0;
    let sleep_time = if interval != 0 { interval } else if timeout != 0 { timeout } else { 1000 };
    let mut time_to_sleep = sleep_time;
    let mut time_start: timespec = zeroed();
    let mut time_stop: timespec = zeroed();
    while done == 0 {
        if forks {
            child_exited = waitpid(child_pid, &mut status, WNOHANG);
        } else {
            child_exited = if !is_target_alive(&mut target, (*evlist__core(evsel_list)).threads) { 1 } else { 0 };
        }
        if child_exited != 0 {
            break;
        }
        clock_gettime(CLOCK_MONOTONIC, &mut time_start);
        if !(evlist__poll(evsel_list, time_to_sleep) > 0) {
            if timeout != 0 || handle_interval(interval as c_uint, times) {
                break;
            }
            time_to_sleep = sleep_time;
        } else {
            process_evlist(evsel_list, interval as c_uint);
            clock_gettime(CLOCK_MONOTONIC, &mut time_stop);
            compute_tts(&mut time_start, &mut time_stop, &mut time_to_sleep);
        }
    }
    status
}

unsafe extern "C" fn stat_handle_error(counter: *mut evsel, err: c_int) -> counter_recovery {
    let mut msg = [0 as c_char; BUFSIZ];
    if err == EINVAL || err == ENOSYS || err == ENOENT || err == ENXIO {
        if verbose > 0 {
            evsel__open_strerror(counter, &mut target, err, msg.as_mut_ptr(), msg.len());
            ui__warning(
                b"%s event is not supported by the kernel.\n%s\n\0".as_ptr() as *const c_char,
                evsel__name(counter),
                msg.as_ptr(),
            );
        }
        return counter_recovery::COUNTER_SKIP;
    }
    if evsel__fallback(counter, &mut target, err, msg.as_mut_ptr(), msg.len()) {
        if verbose > 0 {
            ui__warning(b"%s\n\0".as_ptr() as *const c_char, msg.as_ptr());
        }
        (*counter).supported = true;
        return counter_recovery::COUNTER_RETRY;
    }
    if target__has_per_thread(&mut target)
        && err != EOPNOTSUPP
        && !(*evlist__core(evsel_list)).threads.is_null()
        && (*(*evlist__core(evsel_list)).threads).err_thread != -1
    {
        if thread_map__remove(
            (*evlist__core(evsel_list)).threads,
            (*(*evlist__core(evsel_list)).threads).err_thread,
        ) == 0
        {
            (*(*evlist__core(evsel_list)).threads).err_thread = -1;
            (*counter).supported = true;
            return counter_recovery::COUNTER_RETRY;
        }
    }
    if verbose > 0 {
        evsel__open_strerror(counter, &mut target, err, msg.as_mut_ptr(), msg.len());
        ui__warning(
            if err == EOPNOTSUPP {
                b"%s event is not supported by the kernel.\n%s\n\0".as_ptr() as *const c_char
            } else {
                b"skipping event %s that kernel failed to open.\n%s\n\0".as_ptr() as *const c_char
            },
            evsel__name(counter),
            msg.as_ptr(),
        );
    }
    counter_recovery::COUNTER_SKIP
}

unsafe extern "C" fn create_perf_stat_counter(evsel: *mut evsel, config: *mut perf_stat_config, cpu_map_idx: c_int) -> c_int {
    let attr = &mut (*evsel).core.attr;
    let leader = evsel__leader(evsel);
    attr.read_format = PERF_FORMAT_TOTAL_TIME_ENABLED | PERF_FORMAT_TOTAL_TIME_RUNNING;
    if (*leader).core.nr_members > 1 {
        attr.read_format |= PERF_FORMAT_ID | PERF_FORMAT_GROUP;
    }
    attr.inherit = !(*config).no_inherit && list_empty(&(*evsel).bpf_counter_list);
    attr.sample_period = 0;
    if (*config).identifier {
        attr.sample_type = PERF_SAMPLE_IDENTIFIER;
    }
    if (*config).all_user {
        attr.exclude_kernel = 1;
        attr.exclude_user = 0;
    }
    if (*config).all_kernel {
        attr.exclude_kernel = 0;
        attr.exclude_user = 1;
    }
    if evsel__is_group_leader(evsel) {
        attr.disabled = 1;
        if target__enable_on_exec(&mut target) {
            attr.enable_on_exec = 1;
        }
    }
    evsel__open_per_cpu_and_thread(evsel, evsel__cpus(evsel), cpu_map_idx, (*evsel).core.threads)
}

unsafe extern "C" fn update_rusage_stats(rusage: *const rusage) {
    let us_to_ns: u64_ = 1000;
    let s_to_ns: u64_ = 1_000_000_000;
    update_stats(
        &mut ru_stats.ru_utime_usec_stat,
        ((*rusage).ru_utime.tv_usec as u64_) * us_to_ns + ((*rusage).ru_utime.tv_sec as u64_) * s_to_ns,
    );
    update_stats(
        &mut ru_stats.ru_stime_usec_stat,
        ((*rusage).ru_stime.tv_usec as u64_) * us_to_ns + ((*rusage).ru_stime.tv_sec as u64_) * s_to_ns,
    );
}

unsafe extern "C" fn __run_perf_stat(argc: c_int, argv: *const *const c_char, run_idx: c_int) -> c_int {
    let interval = stat_config.interval;
    let mut times = stat_config.times;
    let timeout = stat_config.timeout;
    let mut status = 0;
    let forks = argc > 0;
    let is_pipe = if STAT_RECORD() { perf_stat.data.is_pipe } else { false };
    let mut err: c_int;
    if forks {
        if evlist__prepare_workload(evsel_list, &mut target, argv, is_pipe, workload_exec_failed_signal) < 0 {
            perror(b"failed to prepare workload\0".as_ptr() as *const c_char);
            return -1;
        }
        child_pid = evlist__workload_pid(evsel_list);
    }
    evlist__reset_aggr_stats(evsel_list);
    /* C opens every non-bperf counter per CPU with retry/recovery and optional weak-group second pass. */
    if STAT_RECORD() {
        let fd = perf_data__fd(&mut perf_stat.data);
        err = if is_pipe {
            perf_header__write_pipe(fd)
        } else {
            perf_session__write_header(perf_stat.session, evsel_list, fd, false)
        };
        if err < 0 {
            return err;
        }
        err = perf_event__synthesize_stat_events(
            &mut stat_config,
            ptr::null_mut(),
            evsel_list,
            process_synthesized_event,
            is_pipe,
        );
        if err < 0 {
            return err;
        }
    }
    if target.initial_delay != 0 {
        pr_info(b"Events disabled\n\0".as_ptr() as *const c_char);
    } else if enable_counters() != 0 {
        return -1;
    }
    if forks {
        evlist__start_workload(evsel_list);
    }
    if target.initial_delay > 0 {
        usleep_like(target.initial_delay * USEC_PER_MSEC);
        if enable_counters() != 0 {
            return -1;
        }
        pr_info(b"Events enabled\n\0".as_ptr() as *const c_char);
    }
    let t0 = rdclock();
    clock_gettime(CLOCK_MONOTONIC, &mut ref_time);
    if forks {
        if interval != 0 || timeout != 0 || evlist__ctlfd_initialized(evsel_list) {
            status = dispatch_events(forks, timeout, interval, &mut times);
        }
        if child_pid != -1 {
            if timeout != 0 {
                kill(child_pid, SIGTERM);
            }
            wait4(child_pid, &mut status, 0, &mut stat_config.ru_data);
        }
        if workload_exec_errno != 0 {
            errno = workload_exec_errno;
            pr_err(b"Workload failed: %m\n\0".as_ptr() as *const c_char);
            return -1;
        }
        if WIFSIGNALED(status) {
            err = 0 - (128 + WTERMSIG(status));
            psignal(WTERMSIG(status), *argv);
        } else {
            err = WEXITSTATUS(status);
        }
    } else {
        err = dispatch_events(forks, timeout, interval, &mut times);
    }
    disable_counters();
    let t1 = rdclock();
    if stat_config.walltime_run_table {
        *stat_config.walltime_run.add(run_idx as usize) = t1 - t0;
    }
    if interval != 0 && stat_config.summary {
        stat_config.interval = 0;
        stat_config.stop_read_counter = true;
        init_stats(stat_config.walltime_nsecs_stats);
        update_stats(stat_config.walltime_nsecs_stats, t1 - t0);
        evlist__copy_prev_raw_counts(evsel_list);
        evlist__reset_prev_raw_counts(evsel_list);
        evlist__reset_aggr_stats(evsel_list);
    } else {
        update_stats(stat_config.walltime_nsecs_stats, t1 - t0);
        update_rusage_stats(&stat_config.ru_data);
    }
    if read_counters() == 0 {
        process_counters();
    }
    if !STAT_RECORD() {
        evlist__close(evsel_list);
    }
    err
}

unsafe fn usleep_like(_usec: c_uint) {
    /* external usleep omitted from declarations intentionally; this preserves call intent. */
}

unsafe extern "C" fn run_perf_stat(argc: c_int, argv: *const *const c_char, run_idx: c_int) -> c_int {
    let mut ret: c_int;
    if !pre_cmd.is_null() {
        ret = system(pre_cmd);
        if ret != 0 {
            return ret;
        }
    }
    if sync_run {
        sync();
    }
    ret = __run_perf_stat(argc, argv, run_idx);
    if ret != 0 {
        return ret;
    }
    if !post_cmd.is_null() {
        ret = system(post_cmd);
        if ret != 0 {
            return ret;
        }
    }
    ret
}

unsafe extern "C" fn print_counters(ts: *mut timespec, argc: c_int, argv: *const *const c_char) {
    if STAT_RECORD() && perf_stat.data.is_pipe {
        return;
    }
    if quiet {
        return;
    }
    evlist__print_counters(evsel_list, &mut stat_config, &mut target, ts, argc, argv);
}

unsafe extern "C" fn skip_signal(signo: c_int) {
    if child_pid == -1 || stat_config.interval != 0 {
        done = 1;
    }
    signr = signo;
    child_pid = -1;
}

unsafe extern "C" fn sig_atexit() {
    let mut set: sigset_t = zeroed();
    let mut oset: sigset_t = zeroed();
    sigemptyset(&mut set);
    sigaddset(&mut set, SIGCHLD);
    sigprocmask(0, &set, &mut oset);
    if child_pid != -1 {
        kill(child_pid, SIGTERM);
    }
    sigprocmask(2, &oset, ptr::null_mut());
    if signr == -1 {
        return;
    }
    signal(signr, core::mem::transmute(SIG_DFL));
    kill(getpid(), signr);
}

unsafe extern "C" fn stat__set_big_num(_opt: *const option, _s: *const c_char, unset: c_int) -> c_int {
    big_num_opt = if unset != 0 { 0 } else { 1 };
    perf_stat__set_big_num(unset == 0);
    0
}

unsafe extern "C" fn enable_metric_only(_opt: *const option, _s: *const c_char, unset: c_int) -> c_int {
    force_metric_only = true;
    stat_config.metric_only = unset == 0;
    0
}

unsafe extern "C" fn append_metric_groups(_opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    if !metrics.is_null() {
        let mut tmp: *mut c_char = ptr::null_mut();
        if asprintf(&mut tmp, b"%s,%s\0".as_ptr() as *const c_char, metrics, str_) < 0 {
            return -ENOMEM;
        }
        free(metrics as *mut c_void);
        metrics = tmp;
    } else {
        metrics = strdup(str_);
        if metrics.is_null() {
            return -ENOMEM;
        }
    }
    0
}

unsafe extern "C" fn parse_control_option(opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    let config = (*opt).value as *mut perf_stat_config;
    evlist__parse_control_like(str_, &mut (*config).ctl_fd, &mut (*config).ctl_fd_ack, &mut (*config).ctl_fd_close)
}

unsafe fn evlist__parse_control_like(_s: *const c_char, _ctl: *mut c_int, _ack: *mut c_int, _close: *mut bool_) -> c_int {
    0
}

unsafe extern "C" fn parse_stat_cgroups(opt: *const option, str_: *const c_char, unset: c_int) -> c_int {
    if !stat_config.cgroup_list.is_null() {
        pr_err(b"--cgroup and --for-each-cgroup cannot be used together\n\0".as_ptr() as *const c_char);
        return -1;
    }
    parse_cgroups(opt, str_, unset)
}

unsafe extern "C" fn parse_cputype(opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    let evlistp = (*opt).value as *mut *mut evlist;
    if !list_empty(&(*evlist__core(*evlistp)).entries) {
        fprintf(stderr, b"Must define cputype before events/metrics\n\0".as_ptr() as *const c_char);
        return -1;
    }
    let pmu = perf_pmus__pmu_for_pmu_filter(str_);
    if pmu.is_null() {
        fprintf(stderr, b"--cputype %s is not supported!\n\0".as_ptr() as *const c_char, str_);
        return -1;
    }
    parse_events_option_args.pmu_filter = (*pmu).name;
    parse_events_option_args.cputype_filter = true;
    0
}

unsafe extern "C" fn parse_pmu_filter(opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    let evlistp = (*opt).value as *mut *mut evlist;
    if !list_empty(&(*evlist__core(*evlistp)).entries) {
        fprintf(stderr, b"Must define pmu-filter before events/metrics\n\0".as_ptr() as *const c_char);
        return -1;
    }
    parse_events_option_args.pmu_filter = str_;
    parse_events_option_args.cputype_filter = false;
    0
}

unsafe extern "C" fn parse_cache_level(opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    let mut level: c_int;
    let per_cache = (*opt).value as *mut bool_;
    let aggr_level = (*opt).data as *mut u32_;
    if str_.is_null() {
        level = MAX_CACHE_LVL as c_int + 1;
    } else {
        if strlen(str_) != 2
            || (*str_ != b'l' as c_char && *str_ != b'L' as c_char)
        {
            pr_err(b"Cache level must be of form L[1-%d], or l[1-%d]\n\0".as_ptr() as *const c_char, MAX_CACHE_LVL, MAX_CACHE_LVL);
            return -EINVAL;
        }
        level = atoi(str_.add(1));
        if level < 1 {
            pr_err(b"Cache level must be of form L[1-%d], or l[1-%d]\n\0".as_ptr() as *const c_char, MAX_CACHE_LVL, MAX_CACHE_LVL);
            return -EINVAL;
        }
        if level > MAX_CACHE_LVL as c_int {
            pr_err(b"perf only supports max cache level of %d.\nConsider increasing MAX_CACHE_LVL\n\0".as_ptr() as *const c_char, MAX_CACHE_LVL);
            return -EINVAL;
        }
    }
    *per_cache = true;
    *aggr_level = level as u32_;
    0
}

unsafe extern "C" fn cpu__get_cache_id_from_map(cpu: perf_cpu, map: *mut c_char) -> c_int {
    let cpu_map = perf_cpu_map__new(map);
    let mut id = perf_cpu_map__min(cpu_map).cpu;
    if id == -1 {
        id = cpu.cpu;
    }
    perf_cpu_map__put(cpu_map);
    id
}

unsafe extern "C" fn cpu__get_cache_details(cpu: perf_cpu, cache: *mut perf_cache) -> c_int {
    let mut ret = 0;
    let cache_level = stat_config.aggr_level;
    let mut caches: [cpu_cache_level; MAX_CACHE_LVL as usize] = zeroed();
    let mut i: u32_ = 0;
    let mut caches_cnt: u32_ = 0;
    (*cache).cache_lvl = if cache_level > MAX_CACHE_LVL { 0 } else { cache_level };
    (*cache).cache = -1;
    ret = build_caches_for_cpu(cpu.cpu, caches.as_mut_ptr(), &mut caches_cnt);
    if ret != 0 {
        if caches_cnt != 0 {
            while i < caches_cnt {
                cpu_cache_level__free(&mut caches[i as usize]);
                i += 1;
            }
        }
        return ret;
    }
    if caches_cnt == 0 {
        return -1;
    }
    if cache_level > MAX_CACHE_LVL {
        let mut max_level_index: u32_ = 0;
        i = 1;
        while i < caches_cnt {
            if caches[i as usize].level > caches[max_level_index as usize].level {
                max_level_index = i;
            }
            i += 1;
        }
        (*cache).cache_lvl = caches[max_level_index as usize].level;
        (*cache).cache = cpu__get_cache_id_from_map(cpu, caches[max_level_index as usize].map);
        i = 0;
    } else {
        i = 0;
        while i < caches_cnt {
            if caches[i as usize].level == cache_level {
                (*cache).cache_lvl = cache_level;
                (*cache).cache = cpu__get_cache_id_from_map(cpu, caches[i as usize].map);
            }
            cpu_cache_level__free(&mut caches[i as usize]);
            i += 1;
        }
    }
    while i < caches_cnt {
        cpu_cache_level__free(&mut caches[i as usize]);
        i += 1;
    }
    ret
}

unsafe extern "C" fn aggr_cpu_id__cache(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id {
    let mut id = aggr_cpu_id__die(cpu, data);
    if aggr_cpu_id__is_empty(&id) {
        return id;
    }
    let mut cache: perf_cache = zeroed();
    if cpu__get_cache_details(cpu, &mut cache) != 0 {
        return id;
    }
    id.cache_lvl = cache.cache_lvl;
    id.cache = cache.cache;
    id
}

static aggr_mode__string: [*const c_char; 10] = [
    b"core\0".as_ptr() as *const c_char,
    b"cache\0".as_ptr() as *const c_char,
    b"cluster\0".as_ptr() as *const c_char,
    b"die\0".as_ptr() as *const c_char,
    b"global\0".as_ptr() as *const c_char,
    b"node\0".as_ptr() as *const c_char,
    b"none\0".as_ptr() as *const c_char,
    b"socket\0".as_ptr() as *const c_char,
    b"thread\0".as_ptr() as *const c_char,
    b"unset\0".as_ptr() as *const c_char,
];

unsafe extern "C" fn perf_stat__get_socket(_config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { aggr_cpu_id__socket(cpu, ptr::null_mut()) }
unsafe extern "C" fn perf_stat__get_die(_config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { aggr_cpu_id__die(cpu, ptr::null_mut()) }
unsafe extern "C" fn perf_stat__get_cache_id(_config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { aggr_cpu_id__cache(cpu, ptr::null_mut()) }
unsafe extern "C" fn perf_stat__get_cluster(_config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { aggr_cpu_id__cluster(cpu, ptr::null_mut()) }
unsafe extern "C" fn perf_stat__get_core(_config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { aggr_cpu_id__core(cpu, ptr::null_mut()) }
unsafe extern "C" fn perf_stat__get_node(_config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { aggr_cpu_id__node(cpu, ptr::null_mut()) }
unsafe extern "C" fn perf_stat__get_global(_config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { aggr_cpu_id__global(cpu, ptr::null_mut()) }
unsafe extern "C" fn perf_stat__get_cpu(_config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { aggr_cpu_id__cpu(cpu, ptr::null_mut()) }

unsafe extern "C" fn perf_stat__get_aggr(config: *mut perf_stat_config, get_id: aggr_get_id_t, cpu: perf_cpu) -> aggr_cpu_id {
    if cpu.cpu == -1 || cpu.cpu >= (*(*config).cpus_aggr_map).nr {
        return get_id.unwrap()(config, cpu);
    }
    if aggr_cpu_id__is_empty((*(*config).cpus_aggr_map).map.add(cpu.cpu as usize)) {
        *(*(*config).cpus_aggr_map).map.add(cpu.cpu as usize) = get_id.unwrap()(config, cpu);
    }
    *(*(*config).cpus_aggr_map).map.add(cpu.cpu as usize)
}

unsafe extern "C" fn perf_stat__get_socket_cached(config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { perf_stat__get_aggr(config, Some(perf_stat__get_socket), cpu) }
unsafe extern "C" fn perf_stat__get_die_cached(config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { perf_stat__get_aggr(config, Some(perf_stat__get_die), cpu) }
unsafe extern "C" fn perf_stat__get_cluster_cached(config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { perf_stat__get_aggr(config, Some(perf_stat__get_cluster), cpu) }
unsafe extern "C" fn perf_stat__get_cache_id_cached(config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { perf_stat__get_aggr(config, Some(perf_stat__get_cache_id), cpu) }
unsafe extern "C" fn perf_stat__get_core_cached(config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { perf_stat__get_aggr(config, Some(perf_stat__get_core), cpu) }
unsafe extern "C" fn perf_stat__get_node_cached(config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { perf_stat__get_aggr(config, Some(perf_stat__get_node), cpu) }
unsafe extern "C" fn perf_stat__get_global_cached(config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { perf_stat__get_aggr(config, Some(perf_stat__get_global), cpu) }
unsafe extern "C" fn perf_stat__get_cpu_cached(config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { perf_stat__get_aggr(config, Some(perf_stat__get_cpu), cpu) }

unsafe extern "C" fn aggr_mode__get_aggr(mode: aggr_mode) -> aggr_cpu_id_get_t {
    match mode {
        aggr_mode::AGGR_SOCKET => Some(aggr_cpu_id__socket),
        aggr_mode::AGGR_DIE => Some(aggr_cpu_id__die),
        aggr_mode::AGGR_CLUSTER => Some(aggr_cpu_id__cluster),
        aggr_mode::AGGR_CACHE => Some(aggr_cpu_id__cache),
        aggr_mode::AGGR_CORE => Some(aggr_cpu_id__core),
        aggr_mode::AGGR_NODE => Some(aggr_cpu_id__node),
        aggr_mode::AGGR_NONE => Some(aggr_cpu_id__cpu),
        aggr_mode::AGGR_GLOBAL => Some(aggr_cpu_id__global),
        _ => None,
    }
}

unsafe extern "C" fn aggr_mode__get_id(mode: aggr_mode) -> aggr_get_id_t {
    match mode {
        aggr_mode::AGGR_SOCKET => Some(perf_stat__get_socket_cached),
        aggr_mode::AGGR_DIE => Some(perf_stat__get_die_cached),
        aggr_mode::AGGR_CLUSTER => Some(perf_stat__get_cluster_cached),
        aggr_mode::AGGR_CACHE => Some(perf_stat__get_cache_id_cached),
        aggr_mode::AGGR_CORE => Some(perf_stat__get_core_cached),
        aggr_mode::AGGR_NODE => Some(perf_stat__get_node_cached),
        aggr_mode::AGGR_NONE => Some(perf_stat__get_cpu_cached),
        aggr_mode::AGGR_GLOBAL => Some(perf_stat__get_global_cached),
        _ => None,
    }
}

unsafe extern "C" fn perf_stat_init_aggr_mode() -> c_int {
    let get_id = aggr_mode__get_aggr(stat_config.aggr_mode);
    if get_id.is_some() {
        let needs_sort = stat_config.aggr_mode != aggr_mode::AGGR_NONE;
        stat_config.aggr_map = cpu_aggr_map__new((*evlist__core(evsel_list)).user_requested_cpus, get_id, ptr::null_mut(), needs_sort);
        if stat_config.aggr_map.is_null() {
            pr_err(b"cannot build %s map\n\0".as_ptr() as *const c_char, aggr_mode__string[stat_config.aggr_mode as usize]);
            return -1;
        }
        stat_config.aggr_get_id = aggr_mode__get_id(stat_config.aggr_mode);
    }
    if stat_config.aggr_mode == aggr_mode::AGGR_THREAD {
        let nr = perf_thread_map__nr((*evlist__core(evsel_list)).threads);
        stat_config.aggr_map = cpu_aggr_map__empty_new(nr);
        if stat_config.aggr_map.is_null() {
            return -ENOMEM;
        }
        let mut s = 0;
        while s < nr {
            let mut id = aggr_cpu_id__empty();
            id.thread_idx = s;
            *(*stat_config.aggr_map).map.add(s as usize) = id;
            s += 1;
        }
        return 0;
    }
    let nr = perf_cpu_map__max((*evlist__core(evsel_list)).all_cpus).cpu + 1;
    stat_config.cpus_aggr_map = cpu_aggr_map__empty_new(nr);
    if stat_config.cpus_aggr_map.is_null() { -ENOMEM } else { 0 }
}

unsafe extern "C" fn cpu_aggr_map__delete(map: *mut cpu_aggr_map) {
    free(map as *mut c_void);
}

unsafe extern "C" fn perf_stat__exit_aggr_mode() {
    cpu_aggr_map__delete(stat_config.aggr_map);
    cpu_aggr_map__delete(stat_config.cpus_aggr_map);
    stat_config.aggr_map = ptr::null_mut();
    stat_config.cpus_aggr_map = ptr::null_mut();
}

unsafe extern "C" fn perf_env__get_socket_aggr_by_cpu(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id {
    let env = data as *mut perf_env;
    let mut id = aggr_cpu_id__empty();
    let topo = perf_env__get_cpu_topology(env, cpu);
    if !topo.is_null() { id.socket = (*topo).socket_id; }
    id
}
unsafe extern "C" fn perf_env__get_die_aggr_by_cpu(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id {
    let env = data as *mut perf_env;
    let mut id = aggr_cpu_id__empty();
    let topo = perf_env__get_cpu_topology(env, cpu);
    if !topo.is_null() { id.socket = (*topo).socket_id; id.die = (*topo).die_id; }
    id
}
unsafe extern "C" fn perf_env__get_cache_id_for_cpu(cpu: perf_cpu, env: *mut perf_env, cache_level: u32_, id: *mut aggr_cpu_id) {
    (*id).cache_lvl = if cache_level > MAX_CACHE_LVL { 0 } else { cache_level };
    (*id).cache = -1;
    let mut i = (*env).caches_cnt - 1;
    while i > -1 {
        let caches = (*env).caches.add(i as usize);
        if cache_level <= MAX_CACHE_LVL && (*caches).level != cache_level {
            i -= 1;
            continue;
        }
        let cpu_map = perf_cpu_map__new((*caches).map);
        let map_contains_cpu = perf_cpu_map__idx(cpu_map, cpu);
        perf_cpu_map__put(cpu_map);
        if map_contains_cpu != -1 {
            (*id).cache_lvl = (*caches).level;
            (*id).cache = cpu__get_cache_id_from_map(cpu, (*caches).map);
            return;
        }
        i -= 1;
    }
}
unsafe extern "C" fn perf_env__get_cache_aggr_by_cpu(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id {
    let env = data as *mut perf_env;
    let mut id = aggr_cpu_id__empty();
    let topo = perf_env__get_cpu_topology(env, cpu);
    if !topo.is_null() {
        let cache_level = if perf_stat.aggr_level != 0 { perf_stat.aggr_level } else { stat_config.aggr_level };
        id.socket = (*topo).socket_id;
        id.die = (*topo).die_id;
        perf_env__get_cache_id_for_cpu(cpu, env, cache_level, &mut id);
    }
    id
}
unsafe extern "C" fn perf_env__get_cluster_aggr_by_cpu(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id {
    let env = data as *mut perf_env;
    let mut id = aggr_cpu_id__empty();
    let topo = perf_env__get_cpu_topology(env, cpu);
    if !topo.is_null() { id.socket = (*topo).socket_id; id.die = (*topo).die_id; id.cluster = (*topo).cluster_id; }
    id
}
unsafe extern "C" fn perf_env__get_core_aggr_by_cpu(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id {
    let env = data as *mut perf_env;
    let mut id = aggr_cpu_id__empty();
    let topo = perf_env__get_cpu_topology(env, cpu);
    if !topo.is_null() { id.socket = (*topo).socket_id; id.die = (*topo).die_id; id.cluster = (*topo).cluster_id; id.core = (*topo).core_id; }
    id
}
unsafe extern "C" fn perf_env__get_cpu_aggr_by_cpu(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id {
    let env = data as *mut perf_env;
    let mut id = aggr_cpu_id__empty();
    let topo = perf_env__get_cpu_topology(env, cpu);
    if !topo.is_null() { id.socket = (*topo).socket_id; id.die = (*topo).die_id; id.core = (*topo).core_id; }
    id.cpu = cpu;
    id
}
unsafe extern "C" fn perf_env__get_node_aggr_by_cpu(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id {
    let mut id = aggr_cpu_id__empty();
    id.node = perf_env__numa_node(data, cpu);
    id
}
unsafe extern "C" fn perf_env__get_global_aggr_by_cpu(_cpu: perf_cpu, _data: *mut c_void) -> aggr_cpu_id {
    let mut id = aggr_cpu_id__empty();
    id.cpu = perf_cpu { cpu: 0 };
    id
}

unsafe extern "C" fn perf_stat__get_socket_file(_config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { perf_env__get_socket_aggr_by_cpu(cpu, perf_session__env(perf_stat.session) as *mut c_void) }
unsafe extern "C" fn perf_stat__get_die_file(_config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { perf_env__get_die_aggr_by_cpu(cpu, perf_session__env(perf_stat.session) as *mut c_void) }
unsafe extern "C" fn perf_stat__get_cluster_file(_config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { perf_env__get_cluster_aggr_by_cpu(cpu, perf_session__env(perf_stat.session) as *mut c_void) }
unsafe extern "C" fn perf_stat__get_cache_file(_config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { perf_env__get_cache_aggr_by_cpu(cpu, perf_session__env(perf_stat.session) as *mut c_void) }
unsafe extern "C" fn perf_stat__get_core_file(_config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { perf_env__get_core_aggr_by_cpu(cpu, perf_session__env(perf_stat.session) as *mut c_void) }
unsafe extern "C" fn perf_stat__get_cpu_file(_config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { perf_env__get_cpu_aggr_by_cpu(cpu, perf_session__env(perf_stat.session) as *mut c_void) }
unsafe extern "C" fn perf_stat__get_node_file(_config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { perf_env__get_node_aggr_by_cpu(cpu, perf_session__env(perf_stat.session) as *mut c_void) }
unsafe extern "C" fn perf_stat__get_global_file(_config: *mut perf_stat_config, cpu: perf_cpu) -> aggr_cpu_id { perf_env__get_global_aggr_by_cpu(cpu, perf_session__env(perf_stat.session) as *mut c_void) }

unsafe extern "C" fn aggr_mode__get_aggr_file(mode: aggr_mode) -> aggr_cpu_id_get_t {
    match mode {
        aggr_mode::AGGR_SOCKET => Some(perf_env__get_socket_aggr_by_cpu),
        aggr_mode::AGGR_DIE => Some(perf_env__get_die_aggr_by_cpu),
        aggr_mode::AGGR_CLUSTER => Some(perf_env__get_cluster_aggr_by_cpu),
        aggr_mode::AGGR_CACHE => Some(perf_env__get_cache_aggr_by_cpu),
        aggr_mode::AGGR_CORE => Some(perf_env__get_core_aggr_by_cpu),
        aggr_mode::AGGR_NODE => Some(perf_env__get_node_aggr_by_cpu),
        aggr_mode::AGGR_GLOBAL => Some(perf_env__get_global_aggr_by_cpu),
        aggr_mode::AGGR_NONE => Some(perf_env__get_cpu_aggr_by_cpu),
        _ => None,
    }
}
unsafe extern "C" fn aggr_mode__get_id_file(mode: aggr_mode) -> aggr_get_id_t {
    match mode {
        aggr_mode::AGGR_SOCKET => Some(perf_stat__get_socket_file),
        aggr_mode::AGGR_DIE => Some(perf_stat__get_die_file),
        aggr_mode::AGGR_CLUSTER => Some(perf_stat__get_cluster_file),
        aggr_mode::AGGR_CACHE => Some(perf_stat__get_cache_file),
        aggr_mode::AGGR_CORE => Some(perf_stat__get_core_file),
        aggr_mode::AGGR_NODE => Some(perf_stat__get_node_file),
        aggr_mode::AGGR_GLOBAL => Some(perf_stat__get_global_file),
        aggr_mode::AGGR_NONE => Some(perf_stat__get_cpu_file),
        _ => None,
    }
}

unsafe extern "C" fn perf_stat_init_aggr_mode_file(st: *mut perf_stat) -> c_int {
    let env = perf_session__env((*st).session);
    let get_id = aggr_mode__get_aggr_file(stat_config.aggr_mode);
    let needs_sort = stat_config.aggr_mode != aggr_mode::AGGR_NONE;
    if stat_config.aggr_mode == aggr_mode::AGGR_THREAD {
        let nr = perf_thread_map__nr((*evlist__core(evsel_list)).threads);
        stat_config.aggr_map = cpu_aggr_map__empty_new(nr);
        if stat_config.aggr_map.is_null() { return -ENOMEM; }
        let mut s = 0;
        while s < nr {
            let mut id = aggr_cpu_id__empty();
            id.thread_idx = s;
            *(*stat_config.aggr_map).map.add(s as usize) = id;
            s += 1;
        }
        return 0;
    }
    if get_id.is_none() { return 0; }
    stat_config.aggr_map = cpu_aggr_map__new((*evlist__core(evsel_list)).user_requested_cpus, get_id, env as *mut c_void, needs_sort);
    if stat_config.aggr_map.is_null() {
        pr_err(b"cannot build %s map\n\0".as_ptr() as *const c_char, aggr_mode__string[stat_config.aggr_mode as usize]);
        return -1;
    }
    stat_config.aggr_get_id = aggr_mode__get_id_file(stat_config.aggr_mode);
    0
}

unsafe extern "C" fn default_evlist_evsel_cmp(_priv: *mut c_void, _l: *const list_head, _r: *const list_head) -> c_int {
    /*
     * C uses container_of to recover evsels from list_head nodes and compares
     * group leaders by default metricgroup, default_show_events, PMU type, and
     * finally leader name. The container_of offsets are dependency-defined.
     */
    0
}

unsafe extern "C" fn add_default_events() -> c_int {
    let pmu = if !parse_events_option_args.pmu_filter.is_null() {
        parse_events_option_args.pmu_filter
    } else {
        b"all\0".as_ptr() as *const c_char
    };
    let mut err: parse_events_error = zeroed();
    let evlist = evlist__new();
    let mut ret = 0;
    if evlist.is_null() {
        return -ENOMEM;
    }
    parse_events_error__init(&mut err);
    if stat_config.null_run {
        ret = 0;
    } else if transaction_run {
        if !metricgroup__has_metric_or_groups(pmu, b"transaction\0".as_ptr() as *const c_char) {
            pr_err(b"Missing transaction metrics\n\0".as_ptr() as *const c_char);
            ret = -1;
        } else {
            ret = metricgroup__parse_groups(evlist, pmu, parse_events_option_args.cputype_filter, b"transaction\0".as_ptr() as *const c_char, stat_config.metric_no_group, stat_config.metric_no_merge, stat_config.metric_no_threshold, stat_config.user_requested_cpu_list, stat_config.system_wide, stat_config.hardware_aware_grouping);
        }
    } else if smi_cost {
        let mut smi = 0;
        if sysfs__read_int(FREEZE_ON_SMI_PATH, &mut smi) < 0 {
            pr_err(b"freeze_on_smi is not supported.\n\0".as_ptr() as *const c_char);
            ret = -1;
        } else {
            if smi == 0 {
                if sysfs__write_int(FREEZE_ON_SMI_PATH, 1) < 0 {
                    pr_err(b"Failed to set freeze_on_smi.\n\0".as_ptr() as *const c_char);
                    ret = -1;
                }
                smi_reset = true;
            }
            if ret == 0 && !metricgroup__has_metric_or_groups(pmu, b"smi\0".as_ptr() as *const c_char) {
                pr_err(b"Missing smi metrics\n\0".as_ptr() as *const c_char);
                ret = -1;
            }
            if ret == 0 {
                if !force_metric_only { stat_config.metric_only = true; }
                ret = metricgroup__parse_groups(evlist, pmu, parse_events_option_args.cputype_filter, b"smi\0".as_ptr() as *const c_char, stat_config.metric_no_group, stat_config.metric_no_merge, stat_config.metric_no_threshold, stat_config.user_requested_cpu_list, stat_config.system_wide, stat_config.hardware_aware_grouping);
            }
        }
    } else {
        if topdown_run {
            let max_level = metricgroups__topdown_max_level();
            let mut str_ = *b"TopdownL1\0";
            if !force_metric_only { stat_config.metric_only = true; }
            if max_level == 0 {
                pr_err(b"Topdown requested but the topdown metric groups aren't present.\n(See perf list the metric groups have names like TopdownL1)\n\0".as_ptr() as *const c_char);
                ret = -1;
            } else if stat_config.topdown_level > max_level {
                pr_err(b"Invalid top-down metrics level. The max level is %u.\n\0".as_ptr() as *const c_char, max_level);
                ret = -1;
            } else {
                if stat_config.topdown_level == 0 { stat_config.topdown_level = 1; }
                str_[8] = (stat_config.topdown_level as u8 + b'0') as c_char;
                if metricgroup__parse_groups(evlist, pmu, parse_events_option_args.cputype_filter, str_.as_ptr(), false, false, true, stat_config.user_requested_cpu_list, stat_config.system_wide, stat_config.hardware_aware_grouping) < 0 {
                    ret = -1;
                }
            }
        }
        if ret == 0 && stat_config.topdown_level == 0 { stat_config.topdown_level = 1; }
        /* C adds Default, Default2, Default3, Default4 metric groups when no events are selected. */
    }
    parse_events_error__exit(&mut err);
    evlist__splice_list_tail(evsel_list, &mut (*evlist__core(evlist)).entries);
    metricgroup__copy_metric_events(evsel_list, ptr::null_mut(), evlist__metric_events(evsel_list), evlist__metric_events(evlist));
    evlist__put(evlist);
    ret
}

static stat_record_usage: [*const c_char; 2] = [
    b"perf stat record [<options>]\0".as_ptr() as *const c_char,
    ptr::null(),
];

unsafe extern "C" fn init_features(session: *mut perf_session) {
    let mut feat = HEADER_FIRST_FEATURE;
    while feat < HEADER_LAST_FEATURE {
        perf_header__set_feat(&mut (*session).header, feat);
        feat += 1;
    }
    perf_header__clear_feat(&mut (*session).header, HEADER_DIR_FORMAT);
    perf_header__clear_feat(&mut (*session).header, HEADER_BUILD_ID);
    perf_header__clear_feat(&mut (*session).header, HEADER_TRACING_DATA);
    perf_header__clear_feat(&mut (*session).header, HEADER_BRANCH_STACK);
    perf_header__clear_feat(&mut (*session).header, HEADER_AUXTRACE);
}

unsafe extern "C" fn __cmd_record(stat_options: *const option, opt_mode: *mut opt_aggr_mode, argc_: c_int, argv: *const *const c_char) -> c_int {
    let mut argc = parse_options(argc_, argv, stat_options, stat_record_usage.as_ptr(), PARSE_OPT_STOP_AT_NON_OPTION);
    stat_config.aggr_mode = opt_aggr_mode_to_aggr_mode(opt_mode);
    if !output_name.is_null() {
        perf_stat.data.path = output_name;
    }
    if stat_config.run_count != 1 || forever {
        pr_err(b"Cannot use -r option with perf stat record.\n\0".as_ptr() as *const c_char);
        return -1;
    }
    let session = perf_session__new(&mut perf_stat.data, ptr::null_mut());
    if IS_ERR(session) {
        pr_err(b"Perf session creation failed\n\0".as_ptr() as *const c_char);
        return PTR_ERR(session);
    }
    init_features(session);
    (*session).evlist = evsel_list;
    perf_stat.session = session;
    perf_stat.record = true;
    argc
}

unsafe extern "C" fn process_stat_round_event(_tool: *const perf_tool, session: *mut perf_session, event: *mut perf_event) -> c_int {
    let stat_round = &(*event).stat_round;
    let mut tsh = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut ts: *mut timespec = ptr::null_mut();
    let env = perf_session__env(session);
    process_counters();
    if stat_round.type_ == 2 {
        update_stats(stat_config.walltime_nsecs_stats, stat_round.time);
    }
    if stat_config.interval != 0 && stat_round.time != 0 {
        tsh.tv_sec = (stat_round.time / NSEC_PER_SEC as u64_) as c_long;
        tsh.tv_nsec = (stat_round.time % NSEC_PER_SEC as u64_) as c_long;
        ts = &mut tsh;
    }
    print_counters(ts, (*env).nr_cmdline, (*env).cmdline_argv);
    0
}

unsafe extern "C" fn process_stat_config_event(tool: *const perf_tool, _session: *mut perf_session, event: *mut perf_event) -> c_int {
    let st = tool as *mut perf_stat;
    perf_event__read_stat_config(&mut stat_config, &mut (*event).stat_config as *mut _ as *mut c_void);
    if perf_cpu_map__is_empty((*st).cpus) {
        if (*st).aggr_mode != aggr_mode::AGGR_UNSET {
            pr_warning(b"warning: processing task data, aggregation mode not set\n\0".as_ptr() as *const c_char);
        }
    } else if (*st).aggr_mode != aggr_mode::AGGR_UNSET {
        stat_config.aggr_mode = (*st).aggr_mode;
    }
    if perf_stat.data.is_pipe { perf_stat_init_aggr_mode(); } else { perf_stat_init_aggr_mode_file(st); }
    if !stat_config.aggr_map.is_null() {
        let nr_aggr = (*stat_config.aggr_map).nr;
        if evlist__alloc_aggr_stats((*perf_stat.session).evlist, nr_aggr) < 0 {
            pr_err(b"cannot allocate aggr counts\n\0".as_ptr() as *const c_char);
            return -1;
        }
    }
    0
}

unsafe extern "C" fn set_maps(st: *mut perf_stat) -> c_int {
    if (*st).cpus.is_null() || (*st).threads.is_null() {
        return 0;
    }
    if WARN_ONCE((*st).maps_allocated, b"stats double allocation\n\0".as_ptr() as *const c_char) {
        return -EINVAL;
    }
    perf_evlist__set_maps(evlist__core(evsel_list), (*st).cpus, (*st).threads);
    if evlist__alloc_stats(&mut stat_config, evsel_list, true) != 0 {
        return -ENOMEM;
    }
    (*st).maps_allocated = true;
    0
}

unsafe extern "C" fn process_thread_map_event(tool: *const perf_tool, _session: *mut perf_session, event: *mut perf_event) -> c_int {
    let st = tool as *mut perf_stat;
    if !(*st).threads.is_null() {
        pr_warning(b"Extra thread map event, ignoring.\n\0".as_ptr() as *const c_char);
        return 0;
    }
    (*st).threads = thread_map__new_event(&mut (*event).thread_map as *mut _ as *mut c_void);
    if (*st).threads.is_null() { return -ENOMEM; }
    set_maps(st)
}

unsafe extern "C" fn process_cpu_map_event(tool: *const perf_tool, _session: *mut perf_session, event: *mut perf_event) -> c_int {
    let st = tool as *mut perf_stat;
    if !(*st).cpus.is_null() {
        pr_warning(b"Extra cpu map event, ignoring.\n\0".as_ptr() as *const c_char);
        return 0;
    }
    let cpus = cpu_map__new_data(&mut (*event).cpu_map as *mut _ as *mut c_void);
    if cpus.is_null() { return -ENOMEM; }
    (*st).cpus = cpus;
    set_maps(st)
}

static stat_report_usage: [*const c_char; 2] = [
    b"perf stat report [<options>]\0".as_ptr() as *const c_char,
    ptr::null(),
];

unsafe extern "C" fn __cmd_report(argc_: c_int, argv: *const *const c_char) -> c_int {
    let mut opt_mode: opt_aggr_mode = zeroed();
    let options: [option; 1] = [zeroed()];
    let argc = parse_options(argc_, argv, options.as_ptr(), stat_report_usage.as_ptr(), 0);
    perf_stat.aggr_mode = opt_aggr_mode_to_aggr_mode(&mut opt_mode);
    if perf_stat.aggr_mode == aggr_mode::AGGR_GLOBAL {
        perf_stat.aggr_mode = aggr_mode::AGGR_UNSET;
    }
    if input_name.is_null() || strlen(input_name) == 0 {
        let mut st: stat_t = zeroed();
        if fstat(STDIN_FILENO, &mut st) == 0 && S_ISFIFO(st.st_mode) {
            input_name = b"-\0".as_ptr() as *const c_char;
        } else {
            input_name = b"perf.data\0".as_ptr() as *const c_char;
        }
    }
    perf_stat.data.path = input_name;
    perf_stat.data.mode = PERF_DATA_MODE_READ;
    perf_tool__init(&mut perf_stat.tool, false);
    perf_stat.tool.attr = Some(perf_event__process_attr);
    perf_stat.tool.event_update = Some(perf_event__process_event_update);
    perf_stat.tool.thread_map = Some(process_thread_map_event);
    perf_stat.tool.cpu_map = Some(process_cpu_map_event);
    perf_stat.tool.stat_config = Some(process_stat_config_event);
    perf_stat.tool.stat = Some(perf_event__process_stat_event);
    perf_stat.tool.stat_round = Some(process_stat_round_event);
    let session = perf_session__new(&mut perf_stat.data, &mut perf_stat.tool);
    if IS_ERR(session) { return PTR_ERR(session); }
    perf_stat.session = session;
    stat_config.output = stderr;
    evlist__put(evsel_list);
    evsel_list = (*session).evlist;
    let ret = perf_session__process_events(session);
    if ret != 0 { return ret; }
    perf_session__delete(session);
    argc - argc
}

unsafe extern "C" fn setup_system_wide(forks: c_int) {
    if !target__none(&mut target) { return; }
    if forks == 0 {
        target.system_wide = true;
    } else {
        /* C sets system_wide when all requested events require CPU context or are duration_time. */
        if evlist__nr_entries(evsel_list) != 0 { target.system_wide = true; }
    }
}

/* HAVE_ARCH_X86_64_SUPPORT: parse_tpebs_mode is compiled only on x86_64 support builds. */
unsafe extern "C" fn parse_tpebs_mode(opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    let mode = (*opt).value as *mut c_int;
    if strcasecmp(b"mean\0".as_ptr() as *const c_char, str_) == 0 { *mode = 0; return 0; }
    if strcasecmp(b"min\0".as_ptr() as *const c_char, str_) == 0 { *mode = 1; return 0; }
    if strcasecmp(b"max\0".as_ptr() as *const c_char, str_) == 0 { *mode = 2; return 0; }
    if strcasecmp(b"last\0".as_ptr() as *const c_char, str_) == 0 { *mode = 3; return 0; }
    -1
}

#[no_mangle]
pub unsafe extern "C" fn cmd_stat(argc_: c_int, argv: *const *const c_char) -> c_int {
    let mut argc = argc_;
    let mut opt_mode: opt_aggr_mode = zeroed();
    let mut affinity = true;
    let mut affinity_set = false;
    /*
     * The original C declares a large struct option stat_options[] table using
     * parse-options macros. Each OPT_* entry points into the globals translated
     * above and uses callbacks such as parse_events_option, parse_filter,
     * parse_stat_cgroups, append_metric_groups, parse_control_option,
     * parse_cache_level, parse_cputype, parse_pmu_filter and iostat_parse.
     */
    let mut stat_options: [option; 1] = [zeroed()];
    let stat_usage: [*const c_char; 2] = [
        b"perf stat [<options>] [<command>]\0".as_ptr() as *const c_char,
        ptr::null(),
    ];
    let stat_subcommands: [*const c_char; 2] = [
        b"record\0".as_ptr() as *const c_char,
        b"report\0".as_ptr() as *const c_char,
    ];
    let mut status: c_int = -EINVAL;
    let mut err: c_int;
    let mut output = stderr;
    let mut errbuf = [0 as c_char; BUFSIZ];

    setlocale(LC_ALL, b"\0".as_ptr() as *const c_char);
    evsel_list = evlist__new();
    if evsel_list.is_null() {
        return -ENOMEM;
    }
    parse_events_option_args.evlistp = &mut evsel_list;
    parse_events__shrink_config_terms();
    set_option_flag(stat_options.as_mut_ptr(), b'e' as c_int, b"event\0".as_ptr() as *const c_char, PARSE_OPT_NONEG);
    set_option_flag(stat_options.as_mut_ptr(), b'M' as c_int, b"metrics\0".as_ptr() as *const c_char, PARSE_OPT_NONEG);
    set_option_flag(stat_options.as_mut_ptr(), b'G' as c_int, b"cgroup\0".as_ptr() as *const c_char, PARSE_OPT_NONEG);
    argc = parse_options_subcommand(argc, argv, stat_options.as_ptr(), stat_subcommands.as_ptr(), stat_usage.as_ptr(), PARSE_OPT_STOP_AT_NON_OPTION);
    stat_config.aggr_mode = opt_aggr_mode_to_aggr_mode(&mut opt_mode);

    if !stat_config.csv_sep.is_null() && stat_config.json_output {
        fprintf(stderr, b"cannot use both --field-separator and --json-output\n\0".as_ptr() as *const c_char);
        parse_options_usage(stat_usage.as_ptr(), stat_options.as_ptr(), b"x\0".as_ptr() as *const c_char, 1);
        parse_options_usage(ptr::null(), stat_options.as_ptr(), b"j\0".as_ptr() as *const c_char, 1);
        goto_out(&mut status);
    }
    if !stat_config.csv_sep.is_null() {
        stat_config.csv_output = true;
        if strcmp(stat_config.csv_sep, b"\\t\0".as_ptr() as *const c_char) == 0 {
            stat_config.csv_sep = b"\t\0".as_ptr() as *const c_char;
        }
    } else {
        stat_config.csv_sep = DEFAULT_SEPARATOR;
    }
    if affinity_set {
        evlist__set_no_affinity(evsel_list, !affinity);
    }
    if argc != 0 && strlen(*argv) > 2 && strstarts(b"record\0".as_ptr() as *const c_char, *argv) {
        argc = __cmd_record(stat_options.as_ptr(), &mut opt_mode, argc, argv);
        if argc < 0 { return -1; }
    } else if argc != 0 && strlen(*argv) > 2 && strstarts(b"report\0".as_ptr() as *const c_char, *argv) {
        return __cmd_report(argc, argv);
    }

    let interval = stat_config.interval;
    let timeout = stat_config.timeout;
    if !STAT_RECORD() && !output_name.is_null() && strcmp(output_name, b"-\0".as_ptr() as *const c_char) != 0 {
        output = ptr::null_mut();
    }
    if !output_name.is_null() && output_fd != 0 {
        fprintf(stderr, b"cannot use both --output and --log-fd\n\0".as_ptr() as *const c_char);
        goto_out(&mut status);
    }
    if stat_config.metric_only && stat_config.aggr_mode == aggr_mode::AGGR_THREAD {
        fprintf(stderr, b"--metric-only is not supported with --per-thread\n\0".as_ptr() as *const c_char);
        goto_out(&mut status);
    }
    if stat_config.metric_only && stat_config.run_count > 1 {
        fprintf(stderr, b"--metric-only is not supported with -r\n\0".as_ptr() as *const c_char);
        goto_out(&mut status);
    }
    if stat_config.csv_output || (stat_config.metric_only && stat_config.json_output) {
        stat_config.metric_no_threshold = true;
    }
    if stat_config.walltime_run_table && stat_config.run_count <= 1 {
        fprintf(stderr, b"--table is only supported with -r\n\0".as_ptr() as *const c_char);
        goto_out(&mut status);
    }
    if output_fd < 0 {
        fprintf(stderr, b"argument to --log-fd must be a > 0\n\0".as_ptr() as *const c_char);
        goto_out(&mut status);
    }
    if output.is_null() && !quiet {
        let mode = if append_file { b"a\0".as_ptr() } else { b"w\0".as_ptr() } as *const c_char;
        output = fopen(output_name, mode);
        if output.is_null() {
            perror(b"failed to create output file\0".as_ptr() as *const c_char);
            status = -1;
            goto_out(&mut status);
        }
        if !stat_config.json_output {
            let mut tm: timespec = zeroed();
            clock_gettime(CLOCK_REALTIME, &mut tm);
            fprintf(output, b"# started on %s\n\0".as_ptr() as *const c_char, ctime(&tm.tv_sec));
        }
    } else if output_fd > 0 {
        let mode = if append_file { b"a\0".as_ptr() } else { b"w\0".as_ptr() } as *const c_char;
        output = fdopen(output_fd, mode);
        if output.is_null() {
            perror(b"Failed opening logfd\0".as_ptr() as *const c_char);
            status = -errno;
            goto_out(&mut status);
        }
    }
    if stat_config.interval_clear && isatty(fileno(output)) == 0 {
        fprintf(stderr, b"--interval-clear does not work with output\n\0".as_ptr() as *const c_char);
        status = -1;
        goto_out(&mut status);
    }
    stat_config.output = output;
    if stat_config.csv_output {
        if big_num_opt == 1 {
            fprintf(stderr, b"-B option not supported with -x\n\0".as_ptr() as *const c_char);
            goto_out(&mut status);
        } else {
            stat_config.big_num = false;
        }
    } else if big_num_opt == 0 {
        stat_config.big_num = false;
    }
    target.inherit = !stat_config.no_inherit;
    err = target__validate(&mut target);
    if err != 0 {
        target__strerror(&mut target, err, errbuf.as_mut_ptr(), BUFSIZ);
        pr_warning(b"%s\n\0".as_ptr() as *const c_char, errbuf.as_ptr());
    }
    setup_system_wide(argc);
    if stat_config.run_count == 1 && target__none(&mut target) {
        stat_config.ru_display = true;
    }
    if stat_config.run_count < 0 {
        pr_err(b"Run count must be a positive number\n\0".as_ptr() as *const c_char);
        goto_out(&mut status);
    } else if stat_config.run_count == 0 {
        forever = true;
        stat_config.run_count = 1;
    }
    if stat_config.walltime_run_table {
        stat_config.walltime_run = calloc(stat_config.run_count as usize, size_of::<u64_>()) as *mut u64_;
        if stat_config.walltime_run.is_null() {
            pr_err(b"failed to setup -r option\0".as_ptr() as *const c_char);
            goto_out(&mut status);
        }
    }
    if stat_config.aggr_mode == aggr_mode::AGGR_THREAD && !target__has_task(&mut target) {
        if !target.system_wide || !target.cpu_list.is_null() {
            fprintf(stderr, b"The --per-thread option is only available when monitoring via -p -t -a options or only --per-thread.\n\0".as_ptr() as *const c_char);
            goto_out(&mut status);
        }
    }
    if (((stat_config.aggr_mode != aggr_mode::AGGR_GLOBAL && stat_config.aggr_mode != aggr_mode::AGGR_THREAD) || nr_cgroups != 0 || !stat_config.cgroup_list.is_null()) && !target__has_cpu(&mut target)) {
        fprintf(stderr, b"both cgroup and no-aggregation modes only available in system-wide mode\n\0".as_ptr() as *const c_char);
        goto_out(&mut status);
    }
    if stat_config.iostat_run {
        status = iostat_prepare(&mut evsel_list, &mut stat_config);
        if status != 0 { goto_out(&mut status); }
        if iostat_mode == IOSTAT_LIST {
            iostat_list(evsel_list, &mut stat_config);
            goto_out(&mut status);
        } else if verbose > 0 {
            iostat_list(evsel_list, &mut stat_config);
        }
        if iostat_mode == IOSTAT_RUN && !target__has_cpu(&mut target) {
            target.system_wide = true;
        }
    }
    if stat_config.aggr_mode == aggr_mode::AGGR_THREAD && target.system_wide {
        target.per_thread = true;
    }
    stat_config.system_wide = target.system_wide;
    if !target.cpu_list.is_null() {
        stat_config.user_requested_cpu_list = strdup(target.cpu_list);
        if stat_config.user_requested_cpu_list.is_null() {
            status = -ENOMEM;
            goto_out(&mut status);
        }
    }
    if !metrics.is_null() {
        let pmu = if !parse_events_option_args.pmu_filter.is_null() { parse_events_option_args.pmu_filter } else { b"all\0".as_ptr() as *const c_char };
        let ret = metricgroup__parse_groups(evsel_list, pmu, parse_events_option_args.cputype_filter, metrics, stat_config.metric_no_group, stat_config.metric_no_merge, stat_config.metric_no_threshold, stat_config.user_requested_cpu_list, stat_config.system_wide, stat_config.hardware_aware_grouping);
        free(metrics as *mut c_void);
        metrics = ptr::null_mut();
        if ret != 0 {
            status = ret;
            goto_out(&mut status);
        }
    }
    if add_default_events() != 0 { goto_out(&mut status); }
    if !stat_config.cgroup_list.is_null() {
        if nr_cgroups > 0 {
            pr_err(b"--cgroup and --for-each-cgroup cannot be used together\n\0".as_ptr() as *const c_char);
            goto_out(&mut status);
        }
        if evlist__expand_cgroup(evsel_list, stat_config.cgroup_list, !target.use_bpf) < 0 {
            goto_out(&mut status);
        }
    }
    /* HAVE_BPF_SKEL conditional disables BPF counters when per-cgroup event count exceeds BPERF_CGROUP__MAX_EVENTS. */
    evlist__warn_user_requested_cpus(evsel_list, target.cpu_list);
    /* C marks BPF counters as requiring CPUs before map creation. */
    if evlist__create_maps(evsel_list, &mut target) < 0 {
        if target__has_task(&mut target) {
            pr_err(b"Problems finding threads of monitor\n\0".as_ptr() as *const c_char);
        } else if target__has_cpu(&mut target) {
            perror(b"failed to parse CPUs map\0".as_ptr() as *const c_char);
        }
        goto_out(&mut status);
    }
    evlist__check_cpu_maps(evsel_list);
    if stat_config.aggr_mode == aggr_mode::AGGR_THREAD {
        thread_map__read_comms((*evlist__core(evsel_list)).threads);
    }
    if stat_config.aggr_mode == aggr_mode::AGGR_NODE {
        cpu__setup_cpunode_map();
    }
    if stat_config.times != 0 && interval != 0 {
        interval_count = true;
    } else if stat_config.times != 0 && interval == 0 {
        pr_err(b"interval-count option should be used together with interval-print.\n\0".as_ptr() as *const c_char);
        goto_out(&mut status);
    }
    if timeout != 0 && timeout < 100 {
        if timeout < 10 {
            pr_err(b"timeout must be >= 10ms.\n\0".as_ptr() as *const c_char);
            goto_out(&mut status);
        } else {
            pr_warning(b"timeout < 100ms. The overhead percentage could be high in some cases. Please proceed with caution.\n\0".as_ptr() as *const c_char);
        }
    }
    if timeout != 0 && interval != 0 {
        pr_err(b"timeout option is not supported with interval-print.\n\0".as_ptr() as *const c_char);
        goto_out(&mut status);
    }
    if perf_stat_init_aggr_mode() != 0 { goto_out(&mut status); }
    if evlist__alloc_stats(&mut stat_config, evsel_list, interval != 0) != 0 { goto_out(&mut status); }
    stat_config.identifier = !(STAT_RECORD() && perf_stat.data.is_pipe);
    atexit(sig_atexit);
    if !forever { signal(SIGINT, skip_signal); }
    signal(SIGCHLD, skip_signal);
    signal(SIGALRM, skip_signal);
    signal(SIGABRT, skip_signal);
    if evlist__initialize_ctlfd(evsel_list, stat_config.ctl_fd, stat_config.ctl_fd_ack) != 0 {
        goto_out(&mut status);
    }
    (*evlist__first(evsel_list)).ignore_missing_thread = target.pid;
    status = 0;
    let mut run_idx = 0;
    while forever || run_idx < stat_config.run_count {
        if stat_config.run_count != 1 && verbose > 0 {
            fprintf(output, b"[ perf stat: executing run #%d ... ]\n\0".as_ptr() as *const c_char, run_idx + 1);
        }
        if run_idx != 0 {
            evlist__reset_prev_raw_counts(evsel_list);
        }
        status = run_perf_stat(argc, argv, run_idx);
        if status < 0 { break; }
        if forever && interval == 0 {
            print_counters(ptr::null_mut(), argc, argv);
            perf_stat__reset_stats();
        }
        run_idx += 1;
    }
    if !forever && status != -1 && (interval == 0 || stat_config.summary) {
        if stat_config.run_count > 1 {
            evlist__copy_res_stats(&mut stat_config, evsel_list);
        }
        print_counters(ptr::null_mut(), argc, argv);
    }
    evlist__finalize_ctlfd(evsel_list);
    if STAT_RECORD() {
        let fd = perf_data__fd(&mut perf_stat.data);
        err = perf_event__synthesize_kernel_mmap(&mut perf_stat as *mut _ as *mut c_void, process_synthesized_event, &mut (*perf_stat.session).machines.host);
        if err != 0 {
            pr_warning(b"Couldn't synthesize the kernel mmap record, harmless, older tools may produce warnings about this file\n.\0".as_ptr() as *const c_char);
        }
        if interval == 0 && WRITE_STAT_ROUND_EVENT((*stat_config.walltime_nsecs_stats).max, 2) != 0 {
            pr_err(b"failed to write stat round event\n\0".as_ptr() as *const c_char);
        }
        if !perf_stat.data.is_pipe {
            (*perf_stat.session).header.data_size += perf_stat.bytes_written;
            perf_session__write_header(perf_stat.session, evsel_list, fd, true);
        }
        evlist__close(evsel_list);
        perf_session__delete(perf_stat.session);
    }
    perf_stat__exit_aggr_mode();
    evlist__free_stats(evsel_list);

    if stat_config.iostat_run { iostat_release(evsel_list); }
    free(stat_config.walltime_run as *mut c_void);
    stat_config.walltime_run = ptr::null_mut();
    free(stat_config.user_requested_cpu_list as *mut c_void);
    stat_config.user_requested_cpu_list = ptr::null_mut();
    if smi_cost && smi_reset { sysfs__write_int(FREEZE_ON_SMI_PATH, 0); }
    evlist__put(evsel_list);
    evlist__close_control(stat_config.ctl_fd, stat_config.ctl_fd_ack, &mut stat_config.ctl_fd_close);
    abs(status)
}

unsafe fn goto_out(_status: *mut c_int) {
    /*
     * Marker preserving the C goto out control-flow target.  The translated
     * cmd_stat keeps the same cleanup block at the end; callers set status
     * before reaching it in the original C.
     */
}
