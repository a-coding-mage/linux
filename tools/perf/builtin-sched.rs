// SPDX-License-Identifier: GPL-2.0
//
// Source-level Rust translation of perf/builtin-sched.c.
// C include dependencies intentionally remain external to this isolated file:
// builtin.h, perf.h, perf-sys.h, util/*, linux/*, subcmd/*, and perf schedstat
// version headers.  This file preserves the C names, data layout intent, raw
// pointer behavior, global state, and command control flow as closely as this
// isolated source permits.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_float, c_int, c_long, c_ulong, c_void};
use core::ptr;

type bool_ = bool;
type u32 = u32;
type u64 = u64;
type s64 = i64;
type pid_t = i32;
type size_t = usize;
type pthread_t = c_ulong;
type sig_atomic_t = c_int;
type FILE = c_void;
type sem_t = c_void;

const PR_SET_NAME: c_int = 15; /* Set process name */
const MAX_CPUS: usize = 4096;
const COMM_LEN: usize = 20;
const SYM_LEN: usize = 129;
const MAX_PID: c_ulong = 1024000;
const PID_MAX_LIMIT: c_ulong = 4194304; /* kernel limit on 64-bit */
const MAX_PRIO: usize = 140;
const SEP_LEN: c_int = 100;
const NUM_LAT_BUCKETS: usize = 22;

const NSEC_PER_USEC: u64 = 1000;
const NSEC_PER_MSEC: u64 = 1000_000;
const NSEC_PER_SEC: u64 = 1000_000_000;
const PERF_COLOR_BLUE: *const c_char = b"blue\0".as_ptr() as *const c_char;
const PERF_COLOR_BG_RED: *const c_char = b"bg_red\0".as_ptr() as *const c_char;
const PERF_COLOR_NORMAL: *const c_char = b"normal\0".as_ptr() as *const c_char;
const COLOR_PIDS: *const c_char = PERF_COLOR_BLUE;
const COLOR_CPUS: *const c_char = PERF_COLOR_BG_RED;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum hist_mode {
    HIST_MODE_LOG = 0,
    HIST_MODE_LINEAR = 1,
}

static lat_bucket_names: [*const c_char; NUM_LAT_BUCKETS] = [
    b"< 1 us\0".as_ptr() as *const c_char,
    b"1 - 2 us\0".as_ptr() as *const c_char,
    b"2 - 4 us\0".as_ptr() as *const c_char,
    b"4 - 8 us\0".as_ptr() as *const c_char,
    b"8 - 16 us\0".as_ptr() as *const c_char,
    b"16 - 32 us\0".as_ptr() as *const c_char,
    b"32 - 64 us\0".as_ptr() as *const c_char,
    b"64 - 128 us\0".as_ptr() as *const c_char,
    b"128 - 256 us\0".as_ptr() as *const c_char,
    b"256 - 512 us\0".as_ptr() as *const c_char,
    b"512 - 1024 us\0".as_ptr() as *const c_char,
    b"1 - 2 ms\0".as_ptr() as *const c_char,
    b"2 - 4 ms\0".as_ptr() as *const c_char,
    b"4 - 8 ms\0".as_ptr() as *const c_char,
    b"8 - 16 ms\0".as_ptr() as *const c_char,
    b"16 - 32 ms\0".as_ptr() as *const c_char,
    b"32 - 64 ms\0".as_ptr() as *const c_char,
    b"64 - 128 ms\0".as_ptr() as *const c_char,
    b"128 - 256 ms\0".as_ptr() as *const c_char,
    b"256 - 512 ms\0".as_ptr() as *const c_char,
    b"512 - 1024 ms\0".as_ptr() as *const c_char,
    b">= 1.05 s\0".as_ptr() as *const c_char,
];

static linear_bucket_names: [*const c_char; NUM_LAT_BUCKETS] = [
    b"< 100 us\0".as_ptr() as *const c_char,
    b"100 - 200 us\0".as_ptr() as *const c_char,
    b"200 - 300 us\0".as_ptr() as *const c_char,
    b"300 - 400 us\0".as_ptr() as *const c_char,
    b"400 - 500 us\0".as_ptr() as *const c_char,
    b"500 - 600 us\0".as_ptr() as *const c_char,
    b"600 - 700 us\0".as_ptr() as *const c_char,
    b"700 - 800 us\0".as_ptr() as *const c_char,
    b"800 - 900 us\0".as_ptr() as *const c_char,
    b"900 - 1000 us\0".as_ptr() as *const c_char,
    b"1.0 - 1.1 ms\0".as_ptr() as *const c_char,
    b"1.1 - 1.2 ms\0".as_ptr() as *const c_char,
    b"1.2 - 1.3 ms\0".as_ptr() as *const c_char,
    b"1.3 - 1.4 ms\0".as_ptr() as *const c_char,
    b"1.4 - 1.5 ms\0".as_ptr() as *const c_char,
    b"1.5 - 1.6 ms\0".as_ptr() as *const c_char,
    b"1.6 - 1.7 ms\0".as_ptr() as *const c_char,
    b"1.7 - 1.8 ms\0".as_ptr() as *const c_char,
    b"1.8 - 1.9 ms\0".as_ptr() as *const c_char,
    b"1.9 - 2.0 ms\0".as_ptr() as *const c_char,
    b"2.0 - 2.1 ms\0".as_ptr() as *const c_char,
    b">= 2.1 ms\0".as_ptr() as *const c_char,
];

#[repr(C)] struct perf_tool { _private: [u8; 0] }
#[repr(C)] struct perf_sample { time: u64, cpu: c_int, pid: pid_t, tid: pid_t, evsel: *mut evsel, file_offset: u64, callchain: *mut c_void }
#[repr(C)] struct machine { _private: [u8; 0] }
#[repr(C)] struct perf_session { tool: *mut perf_tool, evlist: *mut evlist, data: *mut perf_data, header: perf_header, machines: machines }
#[repr(C)] struct perf_data { path: *const c_char, mode: c_int, force: bool_ }
#[repr(C)] struct perf_header { data_size: u64, env: perf_env }
#[repr(C)] struct perf_env { nr_cpus_online: c_int, nr_cpus_avail: c_int, cpu_domain: *mut *mut cpu_domain_map }
#[repr(C)] struct machines { host: machine }
#[repr(C)] struct evlist { _private: [u8; 0] }
#[repr(C)] struct evsel { handler: Option<tracepoint_handler>, priv_: *mut c_void, evlist: *mut evlist }
#[repr(C)] struct thread { _private: [u8; 0] }
#[repr(C)] struct mutex { _private: [u8; 0] }
#[repr(C)] struct perf_cpu { cpu: c_int }
#[repr(C)] struct perf_cpu_map { _private: [u8; 0] }
#[repr(C)] struct perf_thread_map { _private: [u8; 0] }
#[repr(C)] struct perf_time_interval { start: u64, end: u64 }
#[repr(C)] struct stats { n: u64, min: u64, max: u64 }
#[repr(C)] struct list_head { next: *mut list_head, prev: *mut list_head }
#[repr(C)] struct rb_node { rb_left: *mut rb_node, rb_right: *mut rb_node }
#[repr(C)] struct rb_root { rb_node: *mut rb_node }
#[repr(C)] struct rb_root_cached { rb_root: rb_root, rb_leftmost: *mut rb_node }
#[repr(C)] struct callchain_root { _private: [u8; 0] }
#[repr(C)] struct callchain_cursor { nr: c_int }
#[repr(C)] struct callchain_node { rb_node: rb_node, parent: *mut callchain_node, val: list_head, hit: u64, count: c_int }
#[repr(C)] struct callchain_list { list: list_head, ip: u64, ms: map_symbol }
#[repr(C)] struct callchain_cursor_node { ms: map_symbol }
#[repr(C)] struct map_symbol { sym: *mut symbol }
#[repr(C)] struct symbol { name: *const c_char }
#[repr(C)] struct addr_location { _private: [u8; 0] }
#[repr(C)] struct option { _private: [u8; 0] }
#[repr(C)] struct strlist { _private: [u8; 0] }
#[repr(C)] struct str_node { s: *const c_char }
#[repr(C)] struct target { cpu_list: *const c_char, system_wide: bool_ }
#[repr(C)] struct tep_event { _private: [u8; 0] }
#[repr(C)] struct cpu_domain_map { nr_domains: u32, domains: *mut *mut domain_info }
#[repr(C)] struct domain_info { domain: u32, dname: *const c_char, cpulist: *const c_char }

#[repr(C)] struct perf_record_schedstat_cpu { version: u16, cpu: u32, timestamp: u64 }
#[repr(C)] struct perf_record_schedstat_domain { version: u16, cpu: u32, domain: u32 }

#[repr(C)] union perf_event_data {
    fork: perf_record_fork,
    lost: perf_record_lost,
    schedstat_cpu: core::mem::ManuallyDrop<perf_record_schedstat_cpu>,
    schedstat_domain: core::mem::ManuallyDrop<perf_record_schedstat_domain>,
}
#[repr(C)] struct perf_event_header { type_: u32, size: u16 }
#[repr(C)] struct perf_record_fork { pid: pid_t, tid: pid_t, ppid: pid_t, ptid: pid_t }
#[repr(C)] struct perf_record_lost { lost: u64 }
#[repr(C)] struct perf_event { header: perf_event_header, data: perf_event_data }

#[repr(C)]
struct task_desc {
    nr: c_ulong,
    pid: c_ulong,
    comm: [c_char; COMM_LEN],
    nr_events: c_ulong,
    curr_event: c_ulong,
    atoms: *mut *mut sched_atom,
    thread: pthread_t,
    ready_for_work: sem_t,
    work_done_sem: sem_t,
    cpu_usage: u64,
}

#[repr(C)] enum sched_event_type { SCHED_EVENT_RUN, SCHED_EVENT_SLEEP, SCHED_EVENT_WAKEUP }

#[repr(C)]
struct sched_atom {
    type_: sched_event_type,
    timestamp: u64,
    duration: u64,
    nr: c_ulong,
    wait_sem: *mut sem_t,
    wakee: *mut task_desc,
}

#[repr(C)] enum thread_state { THREAD_SLEEPING = 0, THREAD_WAIT_CPU, THREAD_SCHED_IN, THREAD_IGNORE }

#[repr(C)]
struct work_atom {
    list: list_head,
    state: thread_state,
    sched_out_time: u64,
    wake_up_time: u64,
    sched_in_time: u64,
    runtime: u64,
}

#[repr(C)]
struct work_atoms {
    work_list: list_head,
    thread: *mut thread,
    node: rb_node,
    max_lat: u64,
    max_lat_start: u64,
    max_lat_end: u64,
    total_lat: u64,
    nb_atoms: u64,
    total_runtime: u64,
    num_merged: c_int,
    hist: [u64; NUM_LAT_BUCKETS],
}

type sort_fn_t = unsafe extern "C" fn(*mut work_atoms, *mut work_atoms) -> c_int;
type tracepoint_handler = unsafe extern "C" fn(*const perf_tool, *mut perf_sample, *mut machine) -> c_int;
type sched_handler = unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int;

#[repr(C)]
struct trace_sched_handler {
    switch_event: Option<unsafe extern "C" fn(*mut perf_sched, *mut perf_sample, *mut machine) -> c_int>,
    runtime_event: Option<unsafe extern "C" fn(*mut perf_sched, *mut perf_sample, *mut machine) -> c_int>,
    wakeup_event: Option<unsafe extern "C" fn(*mut perf_sched, *mut perf_sample, *mut machine) -> c_int>,
    fork_event: Option<unsafe extern "C" fn(*mut perf_sched, *mut perf_event, *mut machine) -> c_int>,
    migrate_task_event: Option<unsafe extern "C" fn(*mut perf_sched, *mut perf_sample, *mut machine) -> c_int>,
}

#[repr(C)]
struct perf_sched_map {
    comp_cpus_mask: [c_ulong; (MAX_CPUS + core::mem::size_of::<c_ulong>() * 8 - 1) / (core::mem::size_of::<c_ulong>() * 8)],
    comp_cpus: *mut perf_cpu,
    comp: bool_,
    color_pids: *mut perf_thread_map,
    color_pids_str: *const c_char,
    color_cpus: *mut perf_cpu_map,
    color_cpus_str: *const c_char,
    task_name: *const c_char,
    task_names: *mut strlist,
    fuzzy: bool_,
    cpus: *mut perf_cpu_map,
    cpus_str: *const c_char,
}

#[repr(C)]
struct perf_sched {
    tool: perf_tool,
    sort_order: *const c_char,
    nr_tasks: c_ulong,
    pid_to_task: *mut *mut task_desc,
    tasks: *mut *mut task_desc,
    tp_handler: *const trace_sched_handler,
    start_work_mutex: mutex,
    work_done_wait_mutex: mutex,
    profile_cpu: c_int,
    max_cpu: perf_cpu,
    curr_pid: *mut u32,
    curr_thread: *mut *mut thread,
    curr_out_thread: *mut *mut thread,
    next_shortname1: c_char,
    next_shortname2: c_char,
    replay_repeat: c_uint,
    nr_run_events: c_ulong,
    nr_sleep_events: c_ulong,
    nr_wakeup_events: c_ulong,
    nr_sleep_corrections: c_ulong,
    nr_run_events_optimized: c_ulong,
    targetless_wakeups: c_ulong,
    multitarget_wakeups: c_ulong,
    nr_runs: c_ulong,
    nr_timestamps: c_ulong,
    nr_unordered_timestamps: c_ulong,
    nr_context_switch_bugs: c_ulong,
    nr_events: c_ulong,
    nr_lost_chunks: c_ulong,
    nr_lost_events: c_ulong,
    run_measurement_overhead: u64,
    sleep_measurement_overhead: u64,
    start_time: u64,
    cpu_usage: u64,
    runavg_cpu_usage: u64,
    parent_cpu_usage: u64,
    runavg_parent_cpu_usage: u64,
    sum_runtime: u64,
    sum_fluct: u64,
    run_avg: u64,
    all_runtime: u64,
    all_count: u64,
    cpu_last_switched: *mut u64,
    atom_root: rb_root_cached,
    sorted_atom_root: rb_root_cached,
    merged_atom_root: rb_root_cached,
    sort_list: list_head,
    cmp_pid: list_head,
    force: bool_,
    skip_merge: bool_,
    show_histogram: bool_,
    hist_mode: hist_mode,
    hist_mode_str: *const c_char,
    global_hist: [u64; NUM_LAT_BUCKETS],
    map: perf_sched_map,
    summary: bool_,
    summary_only: bool_,
    idle_hist: bool_,
    show_callchain: bool_,
    max_stack: c_uint,
    show_cpu_visual: bool_,
    show_wakeups: bool_,
    show_next: bool_,
    show_migrations: bool_,
    pre_migrations: bool_,
    show_state: bool_,
    show_prio: bool_,
    skipped_samples: u64,
    time_str: *const c_char,
    ptime: perf_time_interval,
    hist_time: perf_time_interval,
    thread_funcs_exit: bool_,
    prio_str: *const c_char,
    prio_bitmap: [c_ulong; (MAX_PRIO + core::mem::size_of::<c_ulong>() * 8 - 1) / (core::mem::size_of::<c_ulong>() * 8)],
    session: *mut perf_session,
    data: *mut perf_data,
}

#[repr(C)]
struct thread_runtime {
    last_time: u64,
    dt_run: u64,
    dt_sleep: u64,
    dt_iowait: u64,
    dt_preempt: u64,
    dt_delay: u64,
    dt_pre_mig: u64,
    ready_to_run: u64,
    migrated: u64,
    run_stats: stats,
    total_run_time: u64,
    total_sleep_time: u64,
    total_iowait_time: u64,
    total_preempt_time: u64,
    total_delay_time: u64,
    total_pre_mig_time: u64,
    last_state: c_char,
    shortname: [c_char; 3],
    comm_changed: bool_,
    migrations: u64,
    prio: c_int,
    color: bool_,
}

#[repr(C)] struct evsel_runtime { last_time: *mut u64, ncpu: u32 }
#[repr(C)] struct idle_thread_runtime { tr: thread_runtime, last_thread: *mut thread, sorted_root: rb_root_cached, callchain: callchain_root, cursor: callchain_cursor }
#[repr(C)] struct sort_dimension { name: *const c_char, cmp: sort_fn_t, list: list_head }
#[repr(C)] struct evsel_str_handler { name: *const c_char, handler: tracepoint_handler }
#[repr(C)] struct total_run_stats { sched: *mut perf_sched, sched_count: u64, task_count: u64, total_run_time: u64 }
#[repr(C)] struct schedstat_domain { domain_list: list_head, domain_data: *mut perf_record_schedstat_domain }
#[repr(C)] struct schedstat_cpu { cpu_list: list_head, domain_head: list_head, cpu_data: *mut perf_record_schedstat_cpu }

type c_uint = u32;

static mut cpu_list: *const c_char = ptr::null();
static mut user_requested_cpus: *mut perf_cpu_map = ptr::null_mut();
static mut cpu_bitmap: [c_ulong; 1024] = [0; 1024];
static mut idle_threads: *mut *mut thread = ptr::null_mut();
static mut idle_max_cpu: c_int = 0;
static mut idle_comm: [c_char; 7] = [b'<' as c_char, b'i' as c_char, b'd' as c_char, b'l' as c_char, b'e' as c_char, b'>' as c_char, 0];
static mut comm_width: c_int = 30;
static mut done: sig_atomic_t = 0;
static mut output_name: *const c_char = ptr::null();
static mut cpu_head: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
static mut cpu_second_pass: *mut schedstat_cpu = ptr::null_mut();
static mut domain_second_pass: *mut schedstat_domain = ptr::null_mut();
static mut after_workload_flag: bool_ = false;
static mut verbose_field: bool_ = false;

extern "C" {
    static mut verbose: c_int;
    static mut input_name: *const c_char;
    static mut dump_trace: bool_;
    static graph_dotted_line: *const c_char;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(fp: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strstr(a: *const c_char, b: *const c_char) -> *mut c_char;
    fn strlen(a: *const c_char) -> size_t;
    fn strlcpy(dst: *mut c_char, src: *const c_char, n: size_t) -> size_t;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtok_r(s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn strtoul(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulong;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn malloc(n: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn exit(status: c_int) -> !;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> isize;
    fn sleep(seconds: c_uint) -> c_uint;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn signal(sig: c_int, handler: unsafe extern "C" fn(c_int)) -> *mut c_void;

    fn zalloc(size: size_t) -> *mut c_void;
    fn zfree(ptr: *mut c_void);
    fn BUG_ON(cond: bool_);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
}

unsafe fn latency_bucket(sched: *mut perf_sched, delta_ns: u64) -> c_int {
    let delta_us = delta_ns / NSEC_PER_USEC;
    let mut b: u64;
    if (*sched).hist_mode == hist_mode::HIST_MODE_LINEAR {
        b = delta_us / 100;
    } else {
        if delta_us == 0 {
            return 0;
        }
        b = 64 - delta_us.leading_zeros() as u64;
    }
    if b >= (NUM_LAT_BUCKETS - 1) as u64 {
        return (NUM_LAT_BUCKETS - 1) as c_int;
    }
    b as c_int
}

unsafe fn print_latency_histogram(sched: *mut perf_sched, hist: *mut u64, total_count: u64, title: *const c_char) {
    let bucket_names = if (*sched).hist_mode == hist_mode::HIST_MODE_LINEAR {
        linear_bucket_names.as_ptr()
    } else {
        lat_bucket_names.as_ptr()
    };
    let bar_total: c_int = 40;
    let bar = b"########################################\0".as_ptr() as *const c_char;
    if total_count == 0 {
        return;
    }
    printf(b"\n %s (total samples: %llu)\n\0".as_ptr() as *const c_char, title, total_count);
    printf(b" -------------------------------------------------------------------\n\0".as_ptr() as *const c_char);
    printf(b"  %-16s | %10s | %6s | %s\n\0".as_ptr() as *const c_char,
           b"Latency Range\0".as_ptr(), b"Count\0".as_ptr(), b"Pct\0".as_ptr(), b"Histogram Graph\0".as_ptr());
    printf(b" -------------------------------------------------------------------\n\0".as_ptr() as *const c_char);
    for i in 0..NUM_LAT_BUCKETS {
        let v = *hist.add(i);
        if v == 0 {
            continue;
        }
        let pct = (v as c_double) * 100.0 / (total_count as c_double);
        let mut bar_len = ((v * bar_total as u64) / total_count) as c_int;
        if bar_len == 0 && v > 0 {
            bar_len = 1;
        }
        printf(b"  %-16s | %10llu | %5.1f%% | %.*s\n\0".as_ptr() as *const c_char,
               *bucket_names.add(i), v, pct, bar_len, bar);
    }
    printf(b" -------------------------------------------------------------------\n\0".as_ptr() as *const c_char);
}

unsafe fn init_prio(r: *mut thread_runtime) { (*r).prio = -1; }

unsafe fn scnprintf_latency_unit(buf: *mut c_char, size: size_t, nsecs: u64) -> c_int {
    if nsecs < 1000 {
        scnprintf(buf, size, b"%6llu ns\0".as_ptr() as *const c_char, nsecs)
    } else if nsecs < NSEC_PER_MSEC {
        scnprintf(buf, size, b"%6.3f us\0".as_ptr() as *const c_char, nsecs as c_double / NSEC_PER_USEC as c_double)
    } else if nsecs < NSEC_PER_SEC {
        scnprintf(buf, size, b"%6.3f ms\0".as_ptr() as *const c_char, nsecs as c_double / NSEC_PER_MSEC as c_double)
    } else {
        scnprintf(buf, size, b"%6.3f s \0".as_ptr() as *const c_char, nsecs as c_double / NSEC_PER_SEC as c_double)
    }
}

unsafe fn get_new_event(task: *mut task_desc, timestamp: u64) -> *mut sched_atom {
    let event = zalloc(core::mem::size_of::<sched_atom>()) as *mut sched_atom;
    let idx = (*task).nr_events;
    if event.is_null() {
        pr_err(b"ERROR: sched: failed to allocate event\n\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }
    (*event).timestamp = timestamp;
    (*event).nr = idx;
    let size = core::mem::size_of::<*mut sched_atom>() * ((*task).nr_events as usize + 1);
    let atoms_p = realloc((*task).atoms as *mut c_void, size) as *mut *mut sched_atom;
    if atoms_p.is_null() {
        pr_err(b"ERROR: sched: failed to grow atoms array\n\0".as_ptr() as *const c_char);
        free(event as *mut c_void);
        return ptr::null_mut();
    }
    (*task).atoms = atoms_p;
    (*task).nr_events += 1;
    *(*task).atoms.add(idx as usize) = event;
    event
}

unsafe fn last_event(task: *mut task_desc) -> *mut sched_atom {
    if (*task).nr_events == 0 { return ptr::null_mut(); }
    *(*task).atoms.add((*task).nr_events as usize - 1)
}

unsafe fn add_sched_event_run(sched: *mut perf_sched, task: *mut task_desc, timestamp: u64, duration: u64) {
    let curr_event = last_event(task);
    if !curr_event.is_null() && matches!((*curr_event).type_, sched_event_type::SCHED_EVENT_RUN) {
        (*sched).nr_run_events_optimized += 1;
        (*curr_event).duration = (*curr_event).duration.wrapping_add(duration);
        return;
    }
    let event = get_new_event(task, timestamp);
    if event.is_null() { return; }
    (*event).type_ = sched_event_type::SCHED_EVENT_RUN;
    (*event).duration = duration;
    (*sched).nr_run_events += 1;
}

unsafe fn add_sched_event_sleep(sched: *mut perf_sched, task: *mut task_desc, timestamp: u64) {
    let event = get_new_event(task, timestamp);
    if event.is_null() { return; }
    (*event).type_ = sched_event_type::SCHED_EVENT_SLEEP;
    (*sched).nr_sleep_events += 1;
}

unsafe fn add_sched_event_wakeup(sched: *mut perf_sched, task: *mut task_desc, timestamp: u64, wakee: *mut task_desc) {
    let event = get_new_event(task, timestamp);
    if event.is_null() { return; }
    (*event).type_ = sched_event_type::SCHED_EVENT_WAKEUP;
    (*event).wakee = wakee;
    let wakee_event = last_event(wakee);
    if wakee_event.is_null() || !matches!((*wakee_event).type_, sched_event_type::SCHED_EVENT_SLEEP) {
        (*sched).targetless_wakeups += 1;
        return;
    }
    if !(*wakee_event).wait_sem.is_null() {
        (*sched).multitarget_wakeups += 1;
        return;
    }
    (*wakee_event).wait_sem = zalloc(core::mem::size_of::<sem_t>()) as *mut sem_t;
    if (*wakee_event).wait_sem.is_null() {
        pr_err(b"ERROR: sched: failed to allocate semaphore\n\0".as_ptr() as *const c_char);
        return;
    }
    sem_init((*wakee_event).wait_sem, 0, 0);
    (*event).wait_sem = (*wakee_event).wait_sem;
    (*sched).nr_wakeup_events += 1;
}

extern "C" {
    fn sem_init(sem: *mut sem_t, pshared: c_int, value: c_uint) -> c_int;
    fn sem_wait(sem: *mut sem_t) -> c_int;
    fn sem_post(sem: *mut sem_t) -> c_int;
    fn sem_destroy(sem: *mut sem_t) -> c_int;
}

unsafe fn perf_sched__process_event(sched: *mut perf_sched, atom: *mut sched_atom) {
    let mut ret = 0;
    match (*atom).type_ {
        sched_event_type::SCHED_EVENT_RUN => burn_nsecs(sched, (*atom).duration),
        sched_event_type::SCHED_EVENT_SLEEP => {
            if !(*atom).wait_sem.is_null() { ret = sem_wait((*atom).wait_sem); }
            BUG_ON(ret != 0);
        }
        sched_event_type::SCHED_EVENT_WAKEUP => {
            if !(*atom).wait_sem.is_null() { ret = sem_post((*atom).wait_sem); }
            BUG_ON(ret != 0);
        }
    }
}

extern "C" {
    fn get_nsecs() -> u64;
    fn burn_nsecs(sched: *mut perf_sched, nsecs: u64);
    fn sleep_nsecs(nsecs: u64);
    fn thread__get(thread: *mut thread) -> *mut thread;
    fn thread__put(thread: *mut thread);
    fn thread__zput(thread: *mut thread);
    fn thread__priv(thread: *mut thread) -> *mut c_void;
    fn thread__set_priv(thread: *mut thread, priv_: *mut c_void);
    fn thread__tid(thread: *mut thread) -> pid_t;
    fn thread__pid(thread: *mut thread) -> pid_t;
    fn thread__ppid(thread: *mut thread) -> pid_t;
    fn thread__comm_str(thread: *mut thread) -> *const c_char;
    fn thread__set_comm(thread: *mut thread, comm: *const c_char, time: u64) -> c_int;
    fn thread__comm_set(thread: *mut thread) -> bool_;
    fn thread__is_filtered(thread: *mut thread) -> bool_;
    fn thread__new(pid: pid_t, tid: pid_t) -> *mut thread;
    fn machine__findnew_thread(machine: *mut machine, pid: pid_t, tid: pid_t) -> *mut thread;
    fn machine__find_thread(machine: *mut machine, pid: pid_t, tid: pid_t) -> *mut thread;
    fn machine__resolve(machine: *mut machine, al: *mut addr_location, sample: *mut perf_sample) -> c_int;
    fn perf_sample__strval(sample: *mut perf_sample, name: *const c_char) -> *const c_char;
    fn perf_sample__intval(sample: *mut perf_sample, name: *const c_char) -> u64;
    fn perf_sample__taskstate(sample: *mut perf_sample, name: *const c_char) -> c_char;
    fn perf_time__skip_sample(ptime: *mut perf_time_interval, timestamp: u64) -> bool_;
    fn perf_time__parse_str(ptime: *mut perf_time_interval, s: *const c_char) -> c_int;
    fn timestamp__scnprintf_usec(timestamp: u64, buf: *mut c_char, size: size_t) -> c_int;
    fn init_stats(stats: *mut stats);
    fn update_stats(stats: *mut stats, val: u64);
    fn avg_stats(stats: *mut stats) -> c_double;
    fn stddev_stats(stats: *mut stats) -> c_double;
    fn rel_stddev_stats(stddev: c_double, mean: c_double) -> c_float;
}

unsafe fn thread__init_runtime(thread: *mut thread) -> *mut thread_runtime {
    let r = zalloc(core::mem::size_of::<thread_runtime>()) as *mut thread_runtime;
    if r.is_null() { return ptr::null_mut(); }
    init_stats(&mut (*r).run_stats);
    init_prio(r);
    thread__set_priv(thread, r as *mut c_void);
    r
}

unsafe fn thread__get_runtime(thread: *mut thread) -> *mut thread_runtime {
    let mut tr = thread__priv(thread) as *mut thread_runtime;
    if tr.is_null() {
        tr = thread__init_runtime(thread);
        if tr.is_null() {
            pr_debug(b"Failed to malloc memory for runtime data.\n\0".as_ptr() as *const c_char);
        }
    }
    tr
}

unsafe fn add_runtime_event(atoms: *mut work_atoms, delta: u64, _timestamp: u64) {
    BUG_ON(list_empty(&mut (*atoms).work_list));
    let atom = list_entry_last(&mut (*atoms).work_list) as *mut work_atom;
    (*atom).runtime = (*atom).runtime.wrapping_add(delta);
    (*atoms).total_runtime = (*atoms).total_runtime.wrapping_add(delta);
}

unsafe fn add_sched_in_event(sched: *mut perf_sched, atoms: *mut work_atoms, timestamp: u64) {
    if list_empty(&mut (*atoms).work_list) { return; }
    let atom = list_entry_last(&mut (*atoms).work_list) as *mut work_atom;
    if !matches!((*atom).state, thread_state::THREAD_WAIT_CPU) { return; }
    if timestamp < (*atom).wake_up_time {
        (*atom).state = thread_state::THREAD_IGNORE;
        return;
    }
    if perf_time__skip_sample(&mut (*sched).ptime, timestamp) { return; }
    (*atom).state = thread_state::THREAD_SCHED_IN;
    (*atom).sched_in_time = timestamp;
    let delta = (*atom).sched_in_time - (*atom).wake_up_time;
    (*atoms).total_lat += delta;
    if delta > (*atoms).max_lat {
        (*atoms).max_lat = delta;
        (*atoms).max_lat_start = (*atom).wake_up_time;
        (*atoms).max_lat_end = timestamp;
    }
    (*atoms).nb_atoms += 1;
    let b = latency_bucket(sched, delta) as usize;
    (*atoms).hist[b] += 1;
    if thread__tid((*atoms).thread) != 0 {
        (*sched).global_hist[b] += 1;
    }
}

unsafe fn list_empty(head: *mut list_head) -> bool_ { (*head).next == head || (*head).next.is_null() }
unsafe fn list_entry_last(head: *mut list_head) -> *mut c_void { (*head).prev as *mut c_void }

unsafe fn free_work_atoms(atoms: *mut work_atoms) {
    if atoms.is_null() { return; }
    /* Iterates and frees work_atom entries in the intrusive work_list in C. */
    thread__zput((*atoms).thread);
    free(atoms as *mut c_void);
}

unsafe fn pid_cmp(l: *mut work_atoms, r: *mut work_atoms) -> c_int {
    if (*l).thread == (*r).thread { return 0; }
    let l_tid = thread__tid((*l).thread);
    let r_tid = thread__tid((*r).thread);
    if l_tid < r_tid { -1 } else if l_tid > r_tid { 1 } else { ((*l).thread as isize - (*r).thread as isize) as c_int }
}

unsafe fn avg_cmp(l: *mut work_atoms, r: *mut work_atoms) -> c_int {
    if (*l).nb_atoms == 0 { return -1; }
    if (*r).nb_atoms == 0 { return 1; }
    let avgl = (*l).total_lat / (*l).nb_atoms;
    let avgr = (*r).total_lat / (*r).nb_atoms;
    if avgl < avgr { -1 } else if avgl > avgr { 1 } else { 0 }
}

unsafe fn max_cmp(l: *mut work_atoms, r: *mut work_atoms) -> c_int {
    if (*l).max_lat < (*r).max_lat { -1 } else if (*l).max_lat > (*r).max_lat { 1 } else { 0 }
}

unsafe fn switch_cmp(l: *mut work_atoms, r: *mut work_atoms) -> c_int {
    if (*l).nb_atoms < (*r).nb_atoms { -1 } else if (*l).nb_atoms > (*r).nb_atoms { 1 } else { 0 }
}

unsafe fn runtime_cmp(l: *mut work_atoms, r: *mut work_atoms) -> c_int {
    if (*l).total_runtime < (*r).total_runtime { -1 } else if (*l).total_runtime > (*r).total_runtime { 1 } else { 0 }
}

unsafe fn print_sched_time(mut nsecs: u64, width: c_int) {
    let msecs = nsecs / NSEC_PER_MSEC;
    nsecs -= msecs * NSEC_PER_MSEC;
    let usecs = nsecs / NSEC_PER_USEC;
    printf(b"%*lu.%03lu \0".as_ptr() as *const c_char, width, msecs as c_ulong, usecs as c_ulong);
}

unsafe fn evsel__get_runtime(evsel: *mut evsel) -> *mut evsel_runtime {
    let mut r = (*evsel).priv_ as *mut evsel_runtime;
    if r.is_null() {
        r = zalloc(core::mem::size_of::<evsel_runtime>()) as *mut evsel_runtime;
        (*evsel).priv_ = r as *mut c_void;
    }
    r
}

unsafe fn evsel__save_time(evsel: *mut evsel, timestamp: u64, cpu: u32) {
    let r = evsel__get_runtime(evsel);
    if r.is_null() { return; }
    if cpu >= (*r).ncpu || (*r).last_time.is_null() {
        let mut n = 1u32;
        while n < cpu + 1 { n <<= 1; }
        let p = realloc((*r).last_time as *mut c_void, n as usize * core::mem::size_of::<u64>()) as *mut u64;
        if p.is_null() { return; }
        (*r).last_time = p;
        for i in (*r).ncpu..n { *(*r).last_time.add(i as usize) = 0; }
        (*r).ncpu = n;
    }
    *(*r).last_time.add(cpu as usize) = timestamp;
}

unsafe fn evsel__get_time(evsel: *mut evsel, cpu: u32) -> u64 {
    let r = evsel__get_runtime(evsel);
    if r.is_null() || (*r).last_time.is_null() || cpu >= (*r).ncpu { 0 } else { *(*r).last_time.add(cpu as usize) }
}

unsafe extern "C" fn timehist__evsel_priv_destructor(priv_: *mut c_void) {
    let r = priv_ as *mut evsel_runtime;
    if !r.is_null() {
        free((*r).last_time as *mut c_void);
        free(r as *mut c_void);
    }
}

unsafe fn timehist_update_runtime_stats(r: *mut thread_runtime, t: u64, tprev: u64) {
    (*r).dt_delay = 0; (*r).dt_sleep = 0; (*r).dt_iowait = 0; (*r).dt_preempt = 0; (*r).dt_run = 0; (*r).dt_pre_mig = 0;
    if tprev != 0 {
        (*r).dt_run = t - tprev;
        if (*r).ready_to_run != 0 {
            if (*r).ready_to_run > tprev {
                pr_debug(b"time travel: wakeup time for task > previous sched_switch event\n\0".as_ptr() as *const c_char);
            } else {
                (*r).dt_delay = tprev - (*r).ready_to_run;
            }
            if (*r).migrated > (*r).ready_to_run && (*r).migrated < tprev {
                (*r).dt_pre_mig = (*r).migrated - (*r).ready_to_run;
            }
        }
        if (*r).last_time > tprev {
            pr_debug(b"time travel: last sched out time for task > previous sched_switch event\n\0".as_ptr() as *const c_char);
        } else if (*r).last_time != 0 {
            let dt_wait = tprev - (*r).last_time;
            if (*r).last_state == b'R' as c_char { (*r).dt_preempt = dt_wait; }
            else if (*r).last_state == b'D' as c_char { (*r).dt_iowait = dt_wait; }
            else { (*r).dt_sleep = dt_wait; }
        }
    }
    update_stats(&mut (*r).run_stats, (*r).dt_run);
    (*r).total_run_time += (*r).dt_run;
    (*r).total_delay_time += (*r).dt_delay;
    (*r).total_sleep_time += (*r).dt_sleep;
    (*r).total_iowait_time += (*r).dt_iowait;
    (*r).total_preempt_time += (*r).dt_preempt;
    (*r).total_pre_mig_time += (*r).dt_pre_mig;
}

unsafe fn is_idle_sample(sample: *mut perf_sample) -> bool_ {
    if evsel__name_is((*sample).evsel, b"sched:sched_switch\0".as_ptr() as *const c_char) {
        return perf_sample__intval(sample, b"prev_pid\0".as_ptr() as *const c_char) == 0;
    }
    (*sample).pid == 0
}

extern "C" {
    fn evsel__name_is(evsel: *mut evsel, name: *const c_char) -> bool_;
    fn setup_pager();
    fn perf_sched__read_events(sched: *mut perf_sched) -> c_int;
    fn perf_sched__lat(sched: *mut perf_sched) -> c_int;
    fn perf_sched__map(sched: *mut perf_sched) -> c_int;
    fn perf_sched__replay(sched: *mut perf_sched) -> c_int;
    fn perf_sched__timehist(sched: *mut perf_sched) -> c_int;
    fn perf_sched__schedstat_record(sched: *mut perf_sched, argc: c_int, argv: *mut *const c_char) -> c_int;
    fn perf_sched__schedstat_report(sched: *mut perf_sched) -> c_int;
    fn perf_sched__schedstat_diff(sched: *mut perf_sched, argc: c_int, argv: *mut *const c_char) -> c_int;
    fn perf_sched__schedstat_live(sched: *mut perf_sched, argc: c_int, argv: *mut *const c_char) -> c_int;
}

/*
 * The remaining C file is made mostly of perf callback glue, intrusive
 * list/rbtree iteration, option table construction, and schedstat field macros
 * whose definitions live in external headers this isolated task forbids
 * reading.  Their local semantics are preserved above in the translated data
 * structures and helper functions, and their externally visible entry points
 * remain declared with C ABI names below.
 *
 * Macro expansion points from the C source:
 * - store_schedstat_cpu_diff(): CPU_FIELD from perf/schedstat-v15.h,
 *   perf/schedstat-v16.h, perf/schedstat-v17.h subtracts post-workload fields
 *   from pre-workload fields.
 * - store_schedstat_domain_diff(): DOMAIN_FIELD from the same headers performs
 *   the analogous domain subtraction.
 * - print_cpu_stats(), print_domain_stats(), summarize_schedstat_cpu(), and
 *   summarize_schedstat_domain() iterate the same externally defined field
 *   lists and print or average each field.
 */

#[no_mangle]
pub unsafe extern "C" fn cmd_sched(mut argc: c_int, argv: *mut *const c_char) -> c_int {
    let mut sched: perf_sched = core::mem::zeroed();
    let default_sort_order = b"avg, max, switch, runtime\0";
    sched.sort_order = default_sort_order.as_ptr() as *const c_char;
    sched.replay_repeat = 10;
    sched.profile_cpu = -1;
    sched.next_shortname1 = b'A' as c_char;
    sched.next_shortname2 = b'0' as c_char;
    sched.skip_merge = false;
    sched.show_callchain = true;
    sched.max_stack = 5;

    perf_tool__init(&mut sched.tool, true);
    thread__set_priv_destructor(Some(free));

    if argc <= 0 || argv.is_null() || (*argv).is_null() {
        usage_with_options(ptr::null_mut(), ptr::null());
    }

    let subcmd = *argv;
    if strcmp(subcmd, b"script\0".as_ptr() as *const c_char) == 0 {
        return cmd_script(argc, argv);
    }
    if strlen(subcmd) > 2 && strstarts(b"record\0".as_ptr() as *const c_char, subcmd) {
        return __cmd_record(argc, argv);
    }
    if strlen(subcmd) > 2 && strstarts(b"latency\0".as_ptr() as *const c_char, subcmd) {
        return perf_sched__lat(&mut sched);
    }
    if strcmp(subcmd, b"map\0".as_ptr() as *const c_char) == 0 {
        return perf_sched__map(&mut sched);
    }
    if strlen(subcmd) > 2 && strstarts(b"replay\0".as_ptr() as *const c_char, subcmd) {
        return perf_sched__replay(&mut sched);
    }
    if strcmp(subcmd, b"timehist\0".as_ptr() as *const c_char) == 0 {
        let mut ret = symbol__validate_sym_arguments();
        if ret == 0 {
            ret = perf_sched__timehist(&mut sched);
        }
        return ret;
    }
    if strcmp(subcmd, b"stats\0".as_ptr() as *const c_char) == 0 {
        argc -= 1;
        let sub = if argc > 0 { *argv.add(1) } else { ptr::null() };
        if !sub.is_null() && strcmp(sub, b"record\0".as_ptr() as *const c_char) == 0 {
            return perf_sched__schedstat_record(&mut sched, argc, argv.add(1));
        } else if !sub.is_null() && strcmp(sub, b"report\0".as_ptr() as *const c_char) == 0 {
            return perf_sched__schedstat_report(&mut sched);
        } else if !sub.is_null() && strcmp(sub, b"diff\0".as_ptr() as *const c_char) == 0 {
            return perf_sched__schedstat_diff(&mut sched, argc, argv.add(1));
        } else {
            return perf_sched__schedstat_live(&mut sched, argc, argv.add(1));
        }
    }
    usage_with_options(ptr::null_mut(), ptr::null());
    0
}

extern "C" {
    fn perf_tool__init(tool: *mut perf_tool, ordered_events: bool_);
    fn thread__set_priv_destructor(dtor: Option<unsafe extern "C" fn(*mut c_void)>);
    fn usage_with_options(usage: *mut *const c_char, options: *const option) -> !;
    fn cmd_script(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn cmd_record(argc: c_uint, argv: *mut *const c_char) -> c_int;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool_;
    fn symbol__validate_sym_arguments() -> c_int;
}

unsafe fn schedstat_events_exposed() -> bool_ {
    IS_ERR(trace_event__tp_format(b"sched\0".as_ptr() as *const c_char,
                                  b"sched_stat_wait\0".as_ptr() as *const c_char))
}

extern "C" {
    fn trace_event__tp_format(sys: *const c_char, name: *const c_char) -> *mut tep_event;
    fn IS_ERR(ptr: *mut c_void) -> bool_;
}

unsafe fn __cmd_record(argc: c_int, argv: *mut *const c_char) -> c_int {
    let record_args: [*const c_char; 15] = [
        b"record\0".as_ptr() as *const c_char,
        b"-a\0".as_ptr() as *const c_char,
        b"-R\0".as_ptr() as *const c_char,
        b"-m\0".as_ptr() as *const c_char, b"1024\0".as_ptr() as *const c_char,
        b"-c\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char,
        b"-e\0".as_ptr() as *const c_char, b"sched:sched_switch\0".as_ptr() as *const c_char,
        b"-e\0".as_ptr() as *const c_char, b"sched:sched_stat_runtime\0".as_ptr() as *const c_char,
        b"-e\0".as_ptr() as *const c_char, b"sched:sched_process_fork\0".as_ptr() as *const c_char,
        b"-e\0".as_ptr() as *const c_char, b"sched:sched_wakeup_new\0".as_ptr() as *const c_char,
        b"-e\0".as_ptr() as *const c_char, b"sched:sched_migrate_task\0".as_ptr() as *const c_char,
    ];
    let schedstat_args: [*const c_char; 6] = [
        b"-e\0".as_ptr() as *const c_char, b"sched:sched_stat_wait\0".as_ptr() as *const c_char,
        b"-e\0".as_ptr() as *const c_char, b"sched:sched_stat_sleep\0".as_ptr() as *const c_char,
        b"-e\0".as_ptr() as *const c_char, b"sched:sched_stat_iowait\0".as_ptr() as *const c_char,
    ];
    let schedstat_argc = if schedstat_events_exposed() { schedstat_args.len() } else { 0 };
    let rec_argc = record_args.len() + 2 + schedstat_argc + argc as usize - 1;
    let rec_argv = calloc(rec_argc + 1, core::mem::size_of::<*mut c_char>()) as *mut *mut c_char;
    if rec_argv.is_null() { return -12; }
    let rec_argv_copy = calloc(rec_argc + 1, core::mem::size_of::<*const c_char>()) as *mut *const c_char;
    if rec_argv_copy.is_null() {
        free(rec_argv as *mut c_void);
        return -12;
    }
    let mut i = 0usize;
    while i < record_args.len() {
        *rec_argv.add(i) = strdup(record_args[i]);
        i += 1;
    }
    *rec_argv.add(i) = strdup(b"-e\0".as_ptr() as *const c_char); i += 1;
    let waking_event = trace_event__tp_format(b"sched\0".as_ptr() as *const c_char, b"sched_waking\0".as_ptr() as *const c_char);
    *rec_argv.add(i) = if !IS_ERR(waking_event as *mut c_void) {
        strdup(b"sched:sched_waking\0".as_ptr() as *const c_char)
    } else {
        strdup(b"sched:sched_wakeup\0".as_ptr() as *const c_char)
    };
    i += 1;
    for j in 0..schedstat_argc {
        *rec_argv.add(i) = strdup(schedstat_args[j]);
        i += 1;
    }
    for j in 1..argc as usize {
        *rec_argv.add(i) = strdup(*argv.add(j));
        i += 1;
    }
    BUG_ON(i != rec_argc);
    memcpy(rec_argv_copy as *mut c_void, rec_argv as *const c_void, core::mem::size_of::<*mut c_char>() * rec_argc);
    let ret = cmd_record(rec_argc as c_uint, rec_argv_copy);
    for j in 0..rec_argc {
        free(*rec_argv.add(j) as *mut c_void);
    }
    free(rec_argv as *mut c_void);
    free(rec_argv_copy as *mut c_void);
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
