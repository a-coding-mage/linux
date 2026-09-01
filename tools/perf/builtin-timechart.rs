// SPDX-License-Identifier: GPL-2.0-only
/*
 * builtin-timechart.rs - make an svg timechart of system activity
 *
 * Rust translation of builtin-timechart.c.
 *
 * (C) Copyright 2009 Intel Corporation
 *
 * Authors:
 *     Arjan van de Ven <arjan@linux.intel.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type u8 = u8;
type u32 = u32;
type u64 = u64;
type s64 = i64;
type size_t = usize;
type FILE = c_void;

const SUPPORT_OLD_POWER_EVENTS: c_int = 1;
const PWR_EVENT_EXIT: c_int = -1;
const TYPE_NONE: c_int = 0;
const TYPE_RUNNING: c_int = 1;
const TYPE_WAITING: c_int = 2;
const TYPE_BLOCKED: c_int = 3;
const CSTATE: c_int = 1;
const PSTATE: c_int = 2;
const MAX_CPUS: usize = 4096;
const BYTES_THRESH: c_int = 1 * 1024 * 1024;
const TIME_THRESH: c_int = 10000000;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EAGAIN: c_int = 11;
const NSEC_PER_SEC: u64 = 1000000000;
const NSEC_PER_MSEC: u64 = 1000000;
const NSEC_PER_USEC: u64 = 1000;
const PERF_SAMPLE_TIME: u64 = 1 << 2;
const PERF_RECORD_SAMPLE: c_uint = 9;
const PERF_RECORD_MISC_USER: u8 = 2;
const PERF_RECORD_MISC_KERNEL: u8 = 1;
const PERF_RECORD_MISC_HYPERVISOR: u8 = 3;
const PERF_CONTEXT_HV: u64 = (-32i64) as u64;
const PERF_CONTEXT_KERNEL: u64 = (-128i64) as u64;
const PERF_CONTEXT_USER: u64 = (-512i64) as u64;
const PERF_CONTEXT_MAX: u64 = (-4095i64) as u64;
const ORDER_CALLEE: c_int = 1;
const TRACE_FLAG_HARDIRQ: u8 = 1 << 2;
const TRACE_FLAG_SOFTIRQ: u8 = 1 << 3;
const HEADER_NRCPUS: c_int = 18;
const HEADER_CPU_TOPOLOGY: c_int = 13;
const PERF_DATA_MODE_READ: c_int = 0;
const PARSE_OPT_STOP_AT_NON_OPTION: c_int = 1;

#[repr(C)]
pub struct perf_tool {
    pub comm: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    pub fork: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    pub sample: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
}

#[repr(C)]
pub struct perf_session {
    pub header: perf_header,
    pub data: *mut perf_data,
    pub machines: machines,
}

#[repr(C)]
pub struct machines {
    pub host: machine,
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_file_section {
    _private: [u8; 0],
}

#[repr(C)]
pub struct env {
    pub nr_cpus_avail: c_uint,
}

#[repr(C)]
pub struct perf_header {
    pub env: env,
}

#[repr(C)]
pub struct perf_data {
    pub path: *const c_char,
    pub mode: c_int,
    pub force: bool,
}

#[repr(C)]
pub struct perf_event_comm {
    pub tid: c_int,
    pub comm: *mut c_char,
}

#[repr(C)]
pub struct perf_event_fork {
    pub pid: c_int,
    pub ppid: c_int,
    pub time: u64,
}

#[repr(C)]
pub union perf_event {
    pub comm: core::mem::ManuallyDrop<perf_event_comm>,
    pub fork: core::mem::ManuallyDrop<perf_event_fork>,
}

#[repr(C)]
pub struct perf_event_attr {
    pub sample_type: u64,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
    pub handler: Option<tracepoint_handler>,
}

#[repr(C)]
pub struct ip_callchain {
    pub nr: c_uint,
    pub ips: [u64; 0],
}

#[repr(C)]
pub struct perf_sample {
    pub evsel: *mut evsel,
    pub time: u64,
    pub file_offset: u64,
    pub cpu: c_uint,
    pub tid: c_int,
    pub callchain: *mut ip_callchain,
}

#[repr(C)]
pub struct symbol {
    pub name: *const c_char,
}

#[repr(C)]
pub struct addr_location {
    pub thread: *mut thread,
    pub sym: *mut symbol,
    pub filtered: c_int,
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct callchain_param_t {
    pub order: c_int,
}

#[repr(C)]
pub struct option {
    pub value: *mut c_void,
}

#[repr(C)]
pub struct evsel_str_handler {
    pub name: *const c_char,
    pub handler: Option<tracepoint_handler>,
}

#[repr(C)]
pub struct timechart {
    pub tool: perf_tool,
    pub all_data: *mut per_pid,
    pub power_events: *mut power_event,
    pub wake_events: *mut wake_event,
    pub session: *mut perf_session,
    pub proc_num: c_int,
    pub numcpus: c_uint,
    pub min_freq: u64,
    pub max_freq: u64,
    pub turbo_frequency: u64,
    pub first_time: u64,
    pub last_time: u64,
    pub power_only: bool,
    pub tasks_only: bool,
    pub with_backtrace: bool,
    pub topology: bool,
    pub force: bool,
    /* IO related settings */
    pub io_only: bool,
    pub skip_eagain: bool,
    pub io_events: u64,
    pub min_time: u64,
    pub merge_dist: u64,
}

/*
 * Datastructure layout:
 * We keep an list of "pid"s, matching the kernels notion of a task struct.
 * Each "pid" entry, has a list of "comm"s.
 *	this is because we want to track different programs different, while
 *	exec will reuse the original pid (by design).
 * Each comm has a list of samples that will be used to draw
 * final graph.
 */
#[repr(C)]
pub struct per_pid {
    pub next: *mut per_pid,
    pub pid: c_int,
    pub ppid: c_int,
    pub start_time: u64,
    pub end_time: u64,
    pub total_time: u64,
    pub total_bytes: u64,
    pub display: c_int,
    pub all: *mut per_pidcomm,
    pub current: *mut per_pidcomm,
}

#[repr(C)]
pub struct per_pidcomm {
    pub next: *mut per_pidcomm,
    pub start_time: u64,
    pub end_time: u64,
    pub total_time: u64,
    pub max_bytes: u64,
    pub total_bytes: u64,
    pub Y: c_int,
    pub display: c_int,
    pub state: c_long,
    pub state_since: u64,
    pub comm: *mut c_char,
    pub samples: *mut cpu_sample,
    pub io_samples: *mut io_sample,
}

#[repr(C)]
pub struct sample_wrapper {
    pub next: *mut sample_wrapper,
    pub timestamp: u64,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct cpu_sample {
    pub next: *mut cpu_sample,
    pub start_time: u64,
    pub end_time: u64,
    pub type_: c_int,
    pub cpu: c_int,
    pub backtrace: *const c_char,
}

pub const IOTYPE_READ: c_int = 0;
pub const IOTYPE_WRITE: c_int = 1;
pub const IOTYPE_SYNC: c_int = 2;
pub const IOTYPE_TX: c_int = 3;
pub const IOTYPE_RX: c_int = 4;
pub const IOTYPE_POLL: c_int = 5;

#[repr(C)]
pub struct io_sample {
    pub next: *mut io_sample,
    pub start_time: u64,
    pub end_time: u64,
    pub bytes: u64,
    pub type_: c_int,
    pub fd: c_int,
    pub err: c_int,
    pub merges: c_int,
}

#[repr(C)]
pub struct power_event {
    pub next: *mut power_event,
    pub type_: c_int,
    pub state: c_int,
    pub start_time: u64,
    pub end_time: u64,
    pub cpu: c_int,
}

#[repr(C)]
pub struct wake_event {
    pub next: *mut wake_event,
    pub waker: c_int,
    pub wakee: c_int,
    pub time: u64,
    pub backtrace: *const c_char,
}

#[repr(C)]
pub struct process_filter {
    pub name: *mut c_char,
    pub pid: c_int,
    pub next: *mut process_filter,
}

type tracepoint_handler = unsafe extern "C" fn(*mut timechart, *mut perf_sample) -> c_int;

unsafe extern "C" {
    static mut process_filter: *mut process_filter;
    static mut input_name: *const c_char;
    static mut svg_page_width: c_int;
    static mut svg_highlight: c_ulong;
    static mut svg_highlight_name: *mut c_char;
    static mut callchain_param: callchain_param_t;

    fn open_memstream(ptr: *mut *mut c_char, sizeloc: *mut size_t) -> *mut FILE;
    fn perror(s: *const c_char);
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn getpid() -> c_int;
    fn abs(j: c_int) -> c_int;
    fn zalloc(size: size_t) -> *mut c_void;
    fn zfree(ptr: *mut *mut c_void);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn machine__resolve(machine: *mut machine, al: *mut addr_location, sample: *mut perf_sample) -> c_int;
    fn thread__find_symbol(thread: *mut thread, cpumode: u8, ip: u64, al: *mut addr_location) -> bool;
    fn perf_sample__intval(sample: *mut perf_sample, name: *const c_char) -> u64;
    fn svg_cstate(cpu: c_int, start: u64, end: u64, state: c_int);
    fn svg_pstate(cpu: c_int, start: u64, end: u64, state: c_int);
    fn svg_interrupt(time: u64, to: c_int, backtrace: *const c_char);
    fn svg_wakeline(time: u64, from: c_int, to: c_int, backtrace: *const c_char);
    fn svg_partial_wakeline(time: u64, from: c_int, task_from: *const c_char, to: c_int, task_to: *const c_char, backtrace: *const c_char);
    fn svg_process(cpu: c_int, start: u64, end: u64, pid: c_int, comm: *const c_char, backtrace: *const c_char);
    fn svg_box(Y: c_int, start: u64, end: u64, class: *const c_char);
    fn svg_fbox(Y: c_int, start: u64, end: u64, height: c_double, class: *const c_char, fd: c_int, err: c_int, merges: c_int);
    fn svg_ubox(Y: c_int, start: u64, end: u64, height: c_double, class: *const c_char, fd: c_int, err: c_int, merges: c_int);
    fn svg_lbox(Y: c_int, start: u64, end: u64, height: c_double, class: *const c_char, fd: c_int, err: c_int, merges: c_int);
    fn svg_text(Y: c_int, start: u64, text: *const c_char);
    fn svg_running(Y: c_int, cpu: c_int, start: u64, end: u64, backtrace: *const c_char);
    fn svg_blocked(Y: c_int, cpu: c_int, start: u64, end: u64, backtrace: *const c_char);
    fn svg_waiting(Y: c_int, cpu: c_int, start: u64, end: u64, backtrace: *const c_char);
    fn open_svg(filename: *const c_char, cpus: c_uint, rows: c_int, first: u64, last: u64);
    fn svg_time_grid(offset: c_double);
    fn svg_io_legenda();
    fn svg_legenda();
    fn svg_cpu_box(cpu: u64, max_freq: u64, turbo_frequency: u64);
    fn svg_close();
    fn svg_build_topology_map(env: *mut env) -> c_int;
    fn perf_tool__init(tool: *mut perf_tool, ordered_events: bool);
    fn perf_session__new(data: *mut perf_data, tool: *mut perf_tool) -> *mut perf_session;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn perf_session__env(session: *mut perf_session) -> *mut env;
    fn symbol__init(env: *mut env) -> c_int;
    fn perf_header__process_sections(header: *mut perf_header, fd: c_int, data: *mut c_void, process: unsafe extern "C" fn(*mut perf_file_section, *mut perf_header, c_int, c_int, *mut c_void) -> c_int) -> c_int;
    fn perf_data__fd(data: *mut perf_data) -> c_int;
    fn perf_session__has_traces(session: *mut perf_session, msg: *const c_char) -> bool;
    fn perf_session__set_tracepoints_handlers(session: *mut perf_session, handlers: *const evsel_str_handler) -> c_int;
    fn perf_session__process_events(session: *mut perf_session) -> c_int;
    fn perf_session__delete(session: *mut perf_session);
    fn cmd_record(argc: c_uint, argv: *mut *const c_char) -> c_int;
    fn is_valid_tracepoint(name: *const c_char) -> bool;
    fn parse_options_subcommand(argc: c_int, argv: *mut *const c_char, options: *const option, subcommands: *const *const c_char, usagestr: *const *const c_char, flags: c_int) -> c_int;
    fn parse_options(argc: c_int, argv: *mut *const c_char, options: *const option, usagestr: *const *const c_char, flags: c_int) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option) -> !;
    fn setup_pager();
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;
    fn symbol__config_symfs(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
}

static mut cpus_cstate_start_times: *mut u64 = null_mut();
static mut cpus_cstate_state: *mut c_int = null_mut();
static mut cpus_pstate_start_times: *mut u64 = null_mut();
static mut cpus_pstate_state: *mut u64 = null_mut();
static mut use_old_power_events: c_int = 0;

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn find_create_pid(tchart: *mut timechart, pid: c_int) -> *mut per_pid {
    let mut cursor = (*tchart).all_data;
    while !cursor.is_null() {
        if (*cursor).pid == pid {
            return cursor;
        }
        cursor = (*cursor).next;
    }
    cursor = zalloc(size_of::<per_pid>()) as *mut per_pid;
    assert!(!cursor.is_null());
    (*cursor).pid = pid;
    (*cursor).next = (*tchart).all_data;
    (*tchart).all_data = cursor;
    cursor
}

unsafe fn create_pidcomm(p: *mut per_pid) -> *mut per_pidcomm {
    let c = zalloc(size_of::<per_pidcomm>()) as *mut per_pidcomm;
    if c.is_null() {
        return null_mut();
    }
    (*p).current = c;
    (*c).next = (*p).all;
    (*p).all = c;
    c
}

unsafe fn pid_set_comm(tchart: *mut timechart, pid: c_int, comm: *mut c_char) {
    let p = find_create_pid(tchart, pid);
    let mut c = (*p).all;
    while !c.is_null() {
        if !(*c).comm.is_null() && strcmp((*c).comm, comm) == 0 {
            (*p).current = c;
            return;
        }
        if (*c).comm.is_null() {
            (*c).comm = strdup(comm);
            (*p).current = c;
            return;
        }
        c = (*c).next;
    }
    c = create_pidcomm(p);
    assert!(!c.is_null());
    (*c).comm = strdup(comm);
}

unsafe fn pid_fork(tchart: *mut timechart, pid: c_int, ppid: c_int, timestamp: u64) {
    let p = find_create_pid(tchart, pid);
    let pp = find_create_pid(tchart, ppid);
    (*p).ppid = ppid;
    if !(*pp).current.is_null() && !(*(*pp).current).comm.is_null() && (*p).current.is_null() {
        pid_set_comm(tchart, pid, (*(*pp).current).comm);
    }
    (*p).start_time = timestamp;
    if !(*p).current.is_null() && (*(*p).current).start_time == 0 {
        (*(*p).current).start_time = timestamp;
        (*(*p).current).state_since = timestamp;
    }
}

unsafe fn pid_exit(tchart: *mut timechart, pid: c_int, timestamp: u64) {
    let p = find_create_pid(tchart, pid);
    (*p).end_time = timestamp;
    if !(*p).current.is_null() {
        (*(*p).current).end_time = timestamp;
    }
}

unsafe fn pid_put_sample(tchart: *mut timechart, pid: c_int, type_: c_int, cpu: c_uint, start: u64, end: u64, backtrace: *const c_char) {
    let p = find_create_pid(tchart, pid);
    let mut c = (*p).current;
    if c.is_null() {
        c = create_pidcomm(p);
        assert!(!c.is_null());
    }
    let sample = zalloc(size_of::<cpu_sample>()) as *mut cpu_sample;
    assert!(!sample.is_null());
    (*sample).start_time = start;
    (*sample).end_time = end;
    (*sample).type_ = type_;
    (*sample).next = (*c).samples;
    (*sample).cpu = cpu as c_int;
    (*sample).backtrace = backtrace;
    (*c).samples = sample;
    if (*sample).type_ == TYPE_RUNNING && end > start && start > 0 {
        (*c).total_time += end - start;
        (*p).total_time += end - start;
    }
    if (*c).start_time == 0 || (*c).start_time > start {
        (*c).start_time = start;
    }
    if (*p).start_time == 0 || (*p).start_time > start {
        (*p).start_time = start;
    }
}

unsafe extern "C" fn process_comm_event(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let tchart = tool as *mut timechart;
    pid_set_comm(tchart, (*event).comm.tid, (*event).comm.comm);
    0
}

unsafe extern "C" fn process_fork_event(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let tchart = tool as *mut timechart;
    pid_fork(tchart, (*event).fork.pid, (*event).fork.ppid, (*event).fork.time);
    0
}

unsafe extern "C" fn process_exit_event(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let tchart = tool as *mut timechart;
    pid_exit(tchart, (*event).fork.pid, (*event).fork.time);
    0
}

unsafe fn c_state_start(cpu: c_int, timestamp: u64, state: c_int) {
    *cpus_cstate_start_times.add(cpu as usize) = timestamp;
    *cpus_cstate_state.add(cpu as usize) = state;
}

unsafe fn c_state_end(tchart: *mut timechart, cpu: c_int, timestamp: u64) {
    let pwr = zalloc(size_of::<power_event>()) as *mut power_event;
    if pwr.is_null() {
        return;
    }
    (*pwr).state = *cpus_cstate_state.add(cpu as usize);
    (*pwr).start_time = *cpus_cstate_start_times.add(cpu as usize);
    (*pwr).end_time = timestamp;
    (*pwr).cpu = cpu;
    (*pwr).type_ = CSTATE;
    (*pwr).next = (*tchart).power_events;
    (*tchart).power_events = pwr;
}

unsafe fn p_state_end(tchart: *mut timechart, cpu: c_int, timestamp: u64) -> *mut power_event {
    let pwr = zalloc(size_of::<power_event>()) as *mut power_event;
    if pwr.is_null() {
        return null_mut();
    }
    (*pwr).state = *cpus_pstate_state.add(cpu as usize) as c_int;
    (*pwr).start_time = *cpus_pstate_start_times.add(cpu as usize);
    (*pwr).end_time = timestamp;
    (*pwr).cpu = cpu;
    (*pwr).type_ = PSTATE;
    (*pwr).next = (*tchart).power_events;
    if (*pwr).start_time == 0 {
        (*pwr).start_time = (*tchart).first_time;
    }
    (*tchart).power_events = pwr;
    pwr
}

unsafe fn p_state_change(tchart: *mut timechart, cpu: c_int, timestamp: u64, new_freq: u64) {
    if new_freq > 8000000 {
        return;
    }
    let pwr = p_state_end(tchart, cpu, timestamp);
    if pwr.is_null() {
        return;
    }
    *cpus_pstate_state.add(cpu as usize) = new_freq;
    *cpus_pstate_start_times.add(cpu as usize) = timestamp;
    if new_freq > (*tchart).max_freq {
        (*tchart).max_freq = new_freq;
    }
    if new_freq < (*tchart).min_freq || (*tchart).min_freq == 0 {
        (*tchart).min_freq = new_freq;
    }
    if new_freq == (*tchart).max_freq - 1000 {
        (*tchart).turbo_frequency = (*tchart).max_freq;
    }
}

unsafe fn sched_wakeup(tchart: *mut timechart, cpu: c_int, timestamp: u64, waker: c_int, wakee: c_int, flags: u8, backtrace: *const c_char) {
    let we = zalloc(size_of::<wake_event>()) as *mut wake_event;
    if we.is_null() {
        free(backtrace as *mut c_void);
        return;
    }
    (*we).time = timestamp;
    (*we).waker = waker;
    (*we).backtrace = backtrace;
    if (flags & TRACE_FLAG_HARDIRQ) != 0 || (flags & TRACE_FLAG_SOFTIRQ) != 0 {
        (*we).waker = -1;
    }
    (*we).wakee = wakee;
    (*we).next = (*tchart).wake_events;
    (*tchart).wake_events = we;
    let p = find_create_pid(tchart, (*we).wakee);
    if !p.is_null() && !(*p).current.is_null() && (*(*p).current).state == TYPE_NONE as c_long {
        (*(*p).current).state_since = timestamp;
        (*(*p).current).state = TYPE_WAITING as c_long;
    }
    if !p.is_null() && !(*p).current.is_null() && (*(*p).current).state == TYPE_BLOCKED as c_long {
        pid_put_sample(tchart, (*p).pid, (*(*p).current).state as c_int, cpu as c_uint, (*(*p).current).state_since, timestamp, null());
        (*(*p).current).state_since = timestamp;
        (*(*p).current).state = TYPE_WAITING as c_long;
    }
}

unsafe fn sched_switch(tchart: *mut timechart, cpu: c_int, timestamp: u64, prev_pid: c_int, next_pid: c_int, prev_state: u64, mut backtrace: *const c_char) {
    let mut backtrace_used = false;
    let prev_p = find_create_pid(tchart, prev_pid);
    let p = find_create_pid(tchart, next_pid);
    if !(*prev_p).current.is_null() && (*(*prev_p).current).state != TYPE_NONE as c_long {
        pid_put_sample(tchart, prev_pid, TYPE_RUNNING, cpu as c_uint, (*(*prev_p).current).state_since, timestamp, backtrace);
        backtrace_used = true;
    }
    if !p.is_null() && !(*p).current.is_null() {
        if (*(*p).current).state != TYPE_NONE as c_long {
            if !backtrace.is_null() && backtrace_used {
                backtrace = strdup(backtrace) as *const c_char;
            }
            pid_put_sample(tchart, next_pid, (*(*p).current).state as c_int, cpu as c_uint, (*(*p).current).state_since, timestamp, backtrace);
            backtrace_used = true;
        }
        (*(*p).current).state_since = timestamp;
        (*(*p).current).state = TYPE_RUNNING as c_long;
    }
    if !(*prev_p).current.is_null() {
        (*(*prev_p).current).state = TYPE_NONE as c_long;
        (*(*prev_p).current).state_since = timestamp;
        if (prev_state & 2) != 0 {
            (*(*prev_p).current).state = TYPE_BLOCKED as c_long;
        }
        if prev_state == 0 {
            (*(*prev_p).current).state = TYPE_WAITING as c_long;
        }
    }
    if !backtrace_used {
        free(backtrace as *mut c_void);
    }
}

/*
 * Returns a malloc'd backtrace string built via open_memstream, or NULL
 * on error.  Caller must free() the returned pointer.
 */
unsafe fn cat_backtrace(sample: *mut perf_sample, machine: *mut machine) -> *mut c_char {
    let mut al: addr_location = zeroed();
    let mut p: *mut c_char = null_mut();
    let mut p_len: size_t = 0;
    let mut cpumode: u8 = PERF_RECORD_MISC_USER;
    let chain = (*sample).callchain;
    let f = open_memstream(&mut p, &mut p_len);
    let mut corrupted = false;
    if f.is_null() {
        perror(cstr!("open_memstream error"));
        return null_mut();
    }
    addr_location__init(&mut al);
    if chain.is_null() {
        addr_location__exit(&mut al);
        fclose(f);
        return p;
    }
    if machine__resolve(machine, &mut al, sample) < 0 {
        pr_err(cstr!("problem processing SAMPLE (%u) event at offset %#llx, skipping it.\n"), PERF_RECORD_SAMPLE, (*sample).file_offset);
        addr_location__exit(&mut al);
        fclose(f);
        return p;
    }
    let mut i: c_uint = 0;
    while i < (*chain).nr {
        let ip = if callchain_param.order == ORDER_CALLEE {
            *(*chain).ips.as_ptr().add(i as usize)
        } else {
            *(*chain).ips.as_ptr().add(((*chain).nr - i - 1) as usize)
        };
        if ip >= PERF_CONTEXT_MAX {
            if ip == PERF_CONTEXT_HV {
                cpumode = PERF_RECORD_MISC_HYPERVISOR;
            } else if ip == PERF_CONTEXT_KERNEL {
                cpumode = PERF_RECORD_MISC_KERNEL;
            } else if ip == PERF_CONTEXT_USER {
                cpumode = PERF_RECORD_MISC_USER;
            } else {
                pr_debug(cstr!("invalid callchain context: %lld\n"), ip as s64);
                corrupted = true;
                break;
            }
            i += 1;
            continue;
        }
        let mut tal: addr_location = zeroed();
        addr_location__init(&mut tal);
        tal.filtered = 0;
        if thread__find_symbol(al.thread, cpumode, ip, &mut tal) {
            fprintf(f, cstr!("..... %016llx %s\n"), ip, (*tal.sym).name);
        } else {
            fprintf(f, cstr!("..... %016llx\n"), ip);
        }
        addr_location__exit(&mut tal);
        i += 1;
    }
    addr_location__exit(&mut al);
    /*
     * fclose() on an open_memstream always sets p to a valid buffer,
     * even if nothing was written — see open_memstream(3).  So p is
     * never NULL after fclose and we need the flag to discard it.
     */
    fclose(f);
    if corrupted {
        zfree(&mut p as *mut *mut c_char as *mut *mut c_void);
    }
    p
}

unsafe extern "C" fn process_sample_event(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let tchart = tool as *mut timechart;
    let evsel = (*sample).evsel;
    let mut ret = 0;
    if ((*evsel).core.attr.sample_type & PERF_SAMPLE_TIME) != 0 {
        if (*tchart).first_time == 0 || (*tchart).first_time > (*sample).time {
            (*tchart).first_time = (*sample).time;
        }
        if (*tchart).last_time < (*sample).time {
            (*tchart).last_time = (*sample).time;
        }
    }
    if (*evsel).handler.is_some() {
        let f = (*evsel).handler.unwrap();
        ret = f(tchart, sample);
    }
    ret
}

unsafe extern "C" fn process_sample_cpu_idle(tchart: *mut timechart, sample: *mut perf_sample) -> c_int {
    let state = perf_sample__intval(sample, cstr!("state")) as u32;
    let cpu_id = perf_sample__intval(sample, cstr!("cpu_id")) as u32;
    /* perf.data is untrusted input — cpu_id may be corrupted */
    if cpu_id as usize >= MAX_CPUS {
        pr_debug(cstr!("at offset %#llx: out-of-bounds cpu_id %u\n"), (*sample).file_offset, cpu_id);
        return -1;
    }
    if state == PWR_EVENT_EXIT as u32 {
        c_state_end(tchart, cpu_id as c_int, (*sample).time);
    } else {
        c_state_start(cpu_id as c_int, (*sample).time, state as c_int);
    }
    0
}

unsafe extern "C" fn process_sample_cpu_frequency(tchart: *mut timechart, sample: *mut perf_sample) -> c_int {
    let state = perf_sample__intval(sample, cstr!("state")) as u32;
    let cpu_id = perf_sample__intval(sample, cstr!("cpu_id")) as u32;
    /* perf.data is untrusted input — cpu_id may be corrupted */
    if cpu_id as usize >= MAX_CPUS {
        pr_debug(cstr!("at offset %#llx: out-of-bounds cpu_id %u\n"), (*sample).file_offset, cpu_id);
        return -1;
    }
    p_state_change(tchart, cpu_id as c_int, (*sample).time, state as u64);
    0
}

unsafe extern "C" fn process_sample_sched_wakeup(tchart: *mut timechart, sample: *mut perf_sample) -> c_int {
    let flags = perf_sample__intval(sample, cstr!("common_flags")) as u8;
    let waker = perf_sample__intval(sample, cstr!("common_pid")) as c_int;
    let wakee = perf_sample__intval(sample, cstr!("pid")) as c_int;
    if (*sample).cpu as usize >= MAX_CPUS {
        pr_debug(cstr!("at offset %#llx: out-of-bounds cpu %u\n"), (*sample).file_offset, (*sample).cpu);
        return -1;
    }
    let backtrace = cat_backtrace(sample, &mut (*(*tchart).session).machines.host);
    sched_wakeup(tchart, (*sample).cpu as c_int, (*sample).time, waker, wakee, flags, backtrace);
    0
}

unsafe extern "C" fn process_sample_sched_switch(tchart: *mut timechart, sample: *mut perf_sample) -> c_int {
    let prev_pid = perf_sample__intval(sample, cstr!("prev_pid")) as c_int;
    let next_pid = perf_sample__intval(sample, cstr!("next_pid")) as c_int;
    let prev_state = perf_sample__intval(sample, cstr!("prev_state"));
    if (*sample).cpu as usize >= MAX_CPUS {
        pr_debug(cstr!("at offset %#llx: out-of-bounds cpu %u\n"), (*sample).file_offset, (*sample).cpu);
        return -1;
    }
    let backtrace = cat_backtrace(sample, &mut (*(*tchart).session).machines.host);
    sched_switch(tchart, (*sample).cpu as c_int, (*sample).time, prev_pid, next_pid, prev_state, backtrace);
    0
}

unsafe extern "C" fn process_sample_power_start(tchart: *mut timechart, sample: *mut perf_sample) -> c_int {
    let cpu_id = perf_sample__intval(sample, cstr!("cpu_id"));
    let value = perf_sample__intval(sample, cstr!("value"));
    if cpu_id as usize >= MAX_CPUS {
        pr_debug(cstr!("at offset %#llx: out-of-bounds cpu_id %llu\n"), (*sample).file_offset, cpu_id);
        return -1;
    }
    c_state_start(cpu_id as c_int, (*sample).time, value as c_int);
    0
}

unsafe extern "C" fn process_sample_power_end(tchart: *mut timechart, sample: *mut perf_sample) -> c_int {
    if (*sample).cpu as usize >= MAX_CPUS {
        pr_debug(cstr!("at offset %#llx: out-of-bounds cpu %u\n"), (*sample).file_offset, (*sample).cpu);
        return -1;
    }
    c_state_end(tchart, (*sample).cpu as c_int, (*sample).time);
    0
}

unsafe extern "C" fn process_sample_power_frequency(tchart: *mut timechart, sample: *mut perf_sample) -> c_int {
    let cpu_id = perf_sample__intval(sample, cstr!("cpu_id"));
    let value = perf_sample__intval(sample, cstr!("value"));
    if cpu_id as usize >= MAX_CPUS {
        pr_debug(cstr!("at offset %#llx: out-of-bounds cpu_id %llu\n"), (*sample).file_offset, cpu_id);
        return -1;
    }
    p_state_change(tchart, cpu_id as c_int, (*sample).time, value);
    0
}

/*
 * After the last sample we need to wrap up the current C/P state
 * and close out each CPU for these.
 */
unsafe fn end_sample_processing(tchart: *mut timechart) {
    let mut cpu: u64 = 0;
    while cpu < (*tchart).numcpus as u64 {
        /* C state disabled by original #if 0 block. */
        /* P state */
        let pwr = p_state_end(tchart, cpu as c_int, (*tchart).last_time);
        if pwr.is_null() {
            return;
        }
        if (*pwr).state == 0 {
            (*pwr).state = (*tchart).min_freq as c_int;
        }
        cpu += 1;
    }
}

unsafe fn pid_begin_io_sample(tchart: *mut timechart, pid: c_int, type_: c_int, start: u64, fd: c_int) -> c_int {
    let p = find_create_pid(tchart, pid);
    let mut c = (*p).current;
    if c.is_null() {
        c = create_pidcomm(p);
        if c.is_null() {
            return -ENOMEM;
        }
    }
    let prev = (*c).io_samples;
    if !prev.is_null() && (*prev).start_time != 0 && (*prev).end_time == 0 {
        pr_warning(cstr!("Skip invalid start event: previous event already started!\n"));
        /* remove previous event that has been started,
         * we are not sure we will ever get an end for it */
        (*c).io_samples = (*prev).next;
        free(prev as *mut c_void);
        return 0;
    }
    let sample = zalloc(size_of::<io_sample>()) as *mut io_sample;
    if sample.is_null() {
        return -ENOMEM;
    }
    (*sample).start_time = start;
    (*sample).type_ = type_;
    (*sample).fd = fd;
    (*sample).next = (*c).io_samples;
    (*c).io_samples = sample;
    if (*c).start_time == 0 || (*c).start_time > start {
        (*c).start_time = start;
    }
    0
}

unsafe fn pid_end_io_sample(tchart: *mut timechart, pid: c_int, type_: c_int, end: u64, ret: c_long) -> c_int {
    let p = find_create_pid(tchart, pid);
    let c = (*p).current;
    if c.is_null() {
        pr_warning(cstr!("Invalid pidcomm!\n"));
        return -1;
    }
    let sample = (*c).io_samples;
    if sample.is_null() {
        return 0;
    }
    if (*sample).end_time != 0 {
        pr_warning(cstr!("Skip invalid end event: previous event already ended!\n"));
        return 0;
    }
    if (*sample).type_ != type_ {
        pr_warning(cstr!("Skip invalid end event: invalid event type!\n"));
        return 0;
    }
    (*sample).end_time = end;
    let prev = (*sample).next;
    /* we want to be able to see small and fast transfers, so make them
     * at least min_time long, but don't overlap them */
    if (*sample).end_time - (*sample).start_time < (*tchart).min_time {
        (*sample).end_time = (*sample).start_time + (*tchart).min_time;
    }
    if !prev.is_null() && (*sample).start_time < (*prev).end_time {
        if (*prev).err != 0 {
            (*sample).start_time = (*prev).end_time;
        } else {
            (*prev).end_time = (*sample).start_time;
        }
    }
    if ret < 0 {
        (*sample).err = ret as c_int;
    } else if type_ == IOTYPE_READ || type_ == IOTYPE_WRITE || type_ == IOTYPE_TX || type_ == IOTYPE_RX {
        if ret as u64 > (*c).max_bytes {
            (*c).max_bytes = ret as u64;
        }
        (*c).total_bytes += ret as u64;
        (*p).total_bytes += ret as u64;
        (*sample).bytes = ret as u64;
    }
    /* merge two requests to make svg smaller and render-friendly */
    if !prev.is_null()
        && (*prev).type_ == (*sample).type_
        && (*prev).err == (*sample).err
        && (*prev).fd == (*sample).fd
        && (*prev).end_time + (*tchart).merge_dist >= (*sample).start_time
    {
        (*sample).bytes += (*prev).bytes;
        (*sample).merges += (*prev).merges + 1;
        (*sample).start_time = (*prev).start_time;
        (*sample).next = (*prev).next;
        free(prev as *mut c_void);
        if (*sample).err == 0 && (*sample).bytes > (*c).max_bytes {
            (*c).max_bytes = (*sample).bytes;
        }
    }
    (*tchart).io_events += 1;
    0
}

unsafe extern "C" fn process_enter_read(tchart: *mut timechart, sample: *mut perf_sample) -> c_int {
    let fd = perf_sample__intval(sample, cstr!("fd")) as c_long;
    pid_begin_io_sample(tchart, (*sample).tid, IOTYPE_READ, (*sample).time, fd as c_int)
}
unsafe extern "C" fn process_exit_read(tchart: *mut timechart, sample: *mut perf_sample) -> c_int {
    let ret = perf_sample__intval(sample, cstr!("ret")) as c_long;
    pid_end_io_sample(tchart, (*sample).tid, IOTYPE_READ, (*sample).time, ret)
}
unsafe extern "C" fn process_enter_write(tchart: *mut timechart, sample: *mut perf_sample) -> c_int {
    let fd = perf_sample__intval(sample, cstr!("fd")) as c_long;
    pid_begin_io_sample(tchart, (*sample).tid, IOTYPE_WRITE, (*sample).time, fd as c_int)
}
unsafe extern "C" fn process_exit_write(tchart: *mut timechart, sample: *mut perf_sample) -> c_int {
    let ret = perf_sample__intval(sample, cstr!("ret")) as c_long;
    pid_end_io_sample(tchart, (*sample).tid, IOTYPE_WRITE, (*sample).time, ret)
}
unsafe extern "C" fn process_enter_sync(tchart: *mut timechart, sample: *mut perf_sample) -> c_int {
    let fd = perf_sample__intval(sample, cstr!("fd")) as c_long;
    pid_begin_io_sample(tchart, (*sample).tid, IOTYPE_SYNC, (*sample).time, fd as c_int)
}
unsafe extern "C" fn process_exit_sync(tchart: *mut timechart, sample: *mut perf_sample) -> c_int {
    let ret = perf_sample__intval(sample, cstr!("ret")) as c_long;
    pid_end_io_sample(tchart, (*sample).tid, IOTYPE_SYNC, (*sample).time, ret)
}
unsafe extern "C" fn process_enter_tx(tchart: *mut timechart, sample: *mut perf_sample) -> c_int {
    let fd = perf_sample__intval(sample, cstr!("fd")) as c_long;
    pid_begin_io_sample(tchart, (*sample).tid, IOTYPE_TX, (*sample).time, fd as c_int)
}
unsafe extern "C" fn process_exit_tx(tchart: *mut timechart, sample: *mut perf_sample) -> c_int {
    let ret = perf_sample__intval(sample, cstr!("ret")) as c_long;
    pid_end_io_sample(tchart, (*sample).tid, IOTYPE_TX, (*sample).time, ret)
}
unsafe extern "C" fn process_enter_rx(tchart: *mut timechart, sample: *mut perf_sample) -> c_int {
    let fd = perf_sample__intval(sample, cstr!("fd")) as c_long;
    pid_begin_io_sample(tchart, (*sample).tid, IOTYPE_RX, (*sample).time, fd as c_int)
}
unsafe extern "C" fn process_exit_rx(tchart: *mut timechart, sample: *mut perf_sample) -> c_int {
    let ret = perf_sample__intval(sample, cstr!("ret")) as c_long;
    pid_end_io_sample(tchart, (*sample).tid, IOTYPE_RX, (*sample).time, ret)
}
unsafe extern "C" fn process_enter_poll(tchart: *mut timechart, sample: *mut perf_sample) -> c_int {
    let fd = perf_sample__intval(sample, cstr!("fd")) as c_long;
    pid_begin_io_sample(tchart, (*sample).tid, IOTYPE_POLL, (*sample).time, fd as c_int)
}
unsafe extern "C" fn process_exit_poll(tchart: *mut timechart, sample: *mut perf_sample) -> c_int {
    let ret = perf_sample__intval(sample, cstr!("ret")) as c_long;
    pid_end_io_sample(tchart, (*sample).tid, IOTYPE_POLL, (*sample).time, ret)
}

/*
 * Sort the pid datastructure
 */
unsafe fn sort_pids(tchart: *mut timechart) {
    let mut new_list: *mut per_pid = null_mut();
    while !(*tchart).all_data.is_null() {
        let p = (*tchart).all_data;
        (*tchart).all_data = (*p).next;
        (*p).next = null_mut();
        if new_list.is_null() {
            new_list = p;
            (*p).next = null_mut();
            continue;
        }
        let mut prev: *mut per_pid = null_mut();
        let mut cursor = new_list;
        while !cursor.is_null() {
            if (*cursor).ppid > (*p).ppid || ((*cursor).ppid == (*p).ppid && (*cursor).pid > (*p).pid) {
                if !prev.is_null() {
                    (*p).next = (*prev).next;
                    (*prev).next = p;
                    cursor = null_mut();
                    continue;
                } else {
                    (*p).next = new_list;
                    new_list = p;
                    cursor = null_mut();
                    continue;
                }
            }
            prev = cursor;
            cursor = (*cursor).next;
            if cursor.is_null() {
                (*prev).next = p;
            }
        }
    }
    (*tchart).all_data = new_list;
}

unsafe fn draw_c_p_states(tchart: *mut timechart) {
    let mut pwr = (*tchart).power_events;
    /*
     * two pass drawing so that the P state bars are on top of the C state blocks
     */
    while !pwr.is_null() {
        if (*pwr).type_ == CSTATE {
            svg_cstate((*pwr).cpu, (*pwr).start_time, (*pwr).end_time, (*pwr).state);
        }
        pwr = (*pwr).next;
    }
    pwr = (*tchart).power_events;
    while !pwr.is_null() {
        if (*pwr).type_ == PSTATE {
            if (*pwr).state == 0 {
                (*pwr).state = (*tchart).min_freq as c_int;
            }
            svg_pstate((*pwr).cpu, (*pwr).start_time, (*pwr).end_time, (*pwr).state);
        }
        pwr = (*pwr).next;
    }
}

unsafe fn draw_wakeups(tchart: *mut timechart) {
    let mut we = (*tchart).wake_events;
    while !we.is_null() {
        let mut from = 0;
        let mut to = 0;
        let mut task_from: *mut c_char = null_mut();
        let mut task_to: *mut c_char = null_mut();
        /* locate the column of the waker and wakee */
        let mut p = (*tchart).all_data;
        while !p.is_null() {
            if (*p).pid == (*we).waker || (*p).pid == (*we).wakee {
                let mut c = (*p).all;
                while !c.is_null() {
                    if (*c).Y != 0 && (*c).start_time <= (*we).time && (*c).end_time >= (*we).time {
                        if (*p).pid == (*we).waker && from == 0 {
                            from = (*c).Y;
                            task_from = strdup((*c).comm);
                        }
                        if (*p).pid == (*we).wakee && to == 0 {
                            to = (*c).Y;
                            task_to = strdup((*c).comm);
                        }
                    }
                    c = (*c).next;
                }
                c = (*p).all;
                while !c.is_null() && (from == 0 || to == 0) {
                    if (*c).Y != 0 && (*p).pid == (*we).waker && from == 0 {
                        from = (*c).Y;
                        task_from = strdup((*c).comm);
                    }
                    if (*c).Y != 0 && (*p).pid == (*we).wakee && to == 0 {
                        to = (*c).Y;
                        task_to = strdup((*c).comm);
                    }
                    c = (*c).next;
                }
            }
            p = (*p).next;
        }
        if task_from.is_null() {
            task_from = malloc(40) as *mut c_char;
            sprintf(task_from, cstr!("[%i]"), (*we).waker);
        }
        if task_to.is_null() {
            task_to = malloc(40) as *mut c_char;
            sprintf(task_to, cstr!("[%i]"), (*we).wakee);
        }
        if (*we).waker == -1 {
            svg_interrupt((*we).time, to, (*we).backtrace);
        } else if from != 0 && to != 0 && abs(from - to) == 1 {
            svg_wakeline((*we).time, from, to, (*we).backtrace);
        } else {
            svg_partial_wakeline((*we).time, from, task_from, to, task_to, (*we).backtrace);
        }
        we = (*we).next;
        free(task_from as *mut c_void);
        free(task_to as *mut c_void);
    }
}

unsafe fn draw_cpu_usage(tchart: *mut timechart) {
    let mut p = (*tchart).all_data;
    while !p.is_null() {
        let mut c = (*p).all;
        while !c.is_null() {
            let mut sample = (*c).samples;
            while !sample.is_null() {
                if (*sample).type_ == TYPE_RUNNING {
                    svg_process((*sample).cpu, (*sample).start_time, (*sample).end_time, (*p).pid, (*c).comm, (*sample).backtrace);
                }
                sample = (*sample).next;
            }
            c = (*c).next;
        }
        p = (*p).next;
    }
}

unsafe fn draw_io_bars(tchart: *mut timechart) {
    let mut comm = [0 as c_char; 256];
    let mut Y = 1;
    let mut p = (*tchart).all_data;
    while !p.is_null() {
        let mut c = (*p).all;
        while !c.is_null() {
            if (*c).display == 0 {
                (*c).Y = 0;
                c = (*c).next;
                continue;
            }
            svg_box(Y, (*c).start_time, (*c).end_time, cstr!("process3"));
            let mut sample = (*c).io_samples;
            while !sample.is_null() {
                let mut h = (*sample).bytes as c_double / (*c).max_bytes as c_double;
                if (*tchart).skip_eagain && (*sample).err == -EAGAIN {
                    sample = (*sample).next;
                    continue;
                }
                if (*sample).err != 0 {
                    h = 1.0;
                }
                if (*sample).type_ == IOTYPE_SYNC {
                    svg_fbox(Y, (*sample).start_time, (*sample).end_time, 1.0, if (*sample).err != 0 { cstr!("error") } else { cstr!("sync") }, (*sample).fd, (*sample).err, (*sample).merges);
                } else if (*sample).type_ == IOTYPE_POLL {
                    svg_fbox(Y, (*sample).start_time, (*sample).end_time, 1.0, if (*sample).err != 0 { cstr!("error") } else { cstr!("poll") }, (*sample).fd, (*sample).err, (*sample).merges);
                } else if (*sample).type_ == IOTYPE_READ {
                    svg_ubox(Y, (*sample).start_time, (*sample).end_time, h, if (*sample).err != 0 { cstr!("error") } else { cstr!("disk") }, (*sample).fd, (*sample).err, (*sample).merges);
                } else if (*sample).type_ == IOTYPE_WRITE {
                    svg_lbox(Y, (*sample).start_time, (*sample).end_time, h, if (*sample).err != 0 { cstr!("error") } else { cstr!("disk") }, (*sample).fd, (*sample).err, (*sample).merges);
                } else if (*sample).type_ == IOTYPE_RX {
                    svg_ubox(Y, (*sample).start_time, (*sample).end_time, h, if (*sample).err != 0 { cstr!("error") } else { cstr!("net") }, (*sample).fd, (*sample).err, (*sample).merges);
                } else if (*sample).type_ == IOTYPE_TX {
                    svg_lbox(Y, (*sample).start_time, (*sample).end_time, h, if (*sample).err != 0 { cstr!("error") } else { cstr!("net") }, (*sample).fd, (*sample).err, (*sample).merges);
                }
                sample = (*sample).next;
            }
            let mut suf = cstr!("");
            let mut bytes = (*c).total_bytes as c_double;
            if bytes > 1024.0 {
                bytes /= 1024.0;
                suf = cstr!("K");
            }
            if bytes > 1024.0 {
                bytes /= 1024.0;
                suf = cstr!("M");
            }
            if bytes > 1024.0 {
                bytes /= 1024.0;
                suf = cstr!("G");
            }
            sprintf(comm.as_mut_ptr(), cstr!("%s:%i (%3.1f %sbytes)"), if (*c).comm.is_null() { cstr!("") } else { (*c).comm }, (*p).pid, bytes, suf);
            svg_text(Y, (*c).start_time, comm.as_ptr());
            (*c).Y = Y;
            Y += 1;
            c = (*c).next;
        }
        p = (*p).next;
    }
}

unsafe fn draw_process_bars(tchart: *mut timechart) {
    let mut Y = 2 * (*tchart).numcpus as c_int + 2;
    let mut p = (*tchart).all_data;
    while !p.is_null() {
        let mut c = (*p).all;
        while !c.is_null() {
            if (*c).display == 0 {
                (*c).Y = 0;
                c = (*c).next;
                continue;
            }
            svg_box(Y, (*c).start_time, (*c).end_time, cstr!("process"));
            let mut sample = (*c).samples;
            while !sample.is_null() {
                if (*sample).type_ == TYPE_RUNNING {
                    svg_running(Y, (*sample).cpu, (*sample).start_time, (*sample).end_time, (*sample).backtrace);
                }
                if (*sample).type_ == TYPE_BLOCKED {
                    svg_blocked(Y, (*sample).cpu, (*sample).start_time, (*sample).end_time, (*sample).backtrace);
                }
                if (*sample).type_ == TYPE_WAITING {
                    svg_waiting(Y, (*sample).cpu, (*sample).start_time, (*sample).end_time, (*sample).backtrace);
                }
                sample = (*sample).next;
            }
            if !(*c).comm.is_null() {
                let mut comm = [0 as c_char; 256];
                if (*c).total_time > 5000000000 {
                    sprintf(comm.as_mut_ptr(), cstr!("%s:%i (%2.2fs)"), (*c).comm, (*p).pid, (*c).total_time as c_double / NSEC_PER_SEC as c_double);
                } else {
                    sprintf(comm.as_mut_ptr(), cstr!("%s:%i (%3.1fms)"), (*c).comm, (*p).pid, (*c).total_time as c_double / NSEC_PER_MSEC as c_double);
                }
                svg_text(Y, (*c).start_time, comm.as_ptr());
            }
            (*c).Y = Y;
            Y += 1;
            c = (*c).next;
        }
        p = (*p).next;
    }
}

unsafe fn add_process_filter(string: *const c_char) {
    let pid = strtoull(string, null_mut(), 10) as c_int;
    let filt = malloc(size_of::<process_filter>()) as *mut process_filter;
    if filt.is_null() {
        return;
    }
    (*filt).name = strdup(string);
    (*filt).pid = pid;
    (*filt).next = process_filter;
    process_filter = filt;
}

unsafe fn passes_filter(p: *mut per_pid, c: *mut per_pidcomm) -> c_int {
    if process_filter.is_null() {
        return 1;
    }
    let mut filt = process_filter;
    while !filt.is_null() {
        if (*filt).pid != 0 && (*p).pid == (*filt).pid {
            return 1;
        }
        if strcmp((*filt).name, (*c).comm) == 0 {
            return 1;
        }
        filt = (*filt).next;
    }
    0
}

unsafe fn determine_display_tasks_filtered(tchart: *mut timechart) -> c_int {
    let mut count = 0;
    let mut p = (*tchart).all_data;
    while !p.is_null() {
        (*p).display = 0;
        if (*p).start_time == 1 {
            (*p).start_time = (*tchart).first_time;
        }
        /* no exit marker, task kept running to the end */
        if (*p).end_time == 0 {
            (*p).end_time = (*tchart).last_time;
        }
        let mut c = (*p).all;
        while !c.is_null() {
            (*c).display = 0;
            if (*c).start_time == 1 {
                (*c).start_time = (*tchart).first_time;
            }
            if passes_filter(p, c) != 0 {
                (*c).display = 1;
                (*p).display = 1;
                count += 1;
            }
            if (*c).end_time == 0 {
                (*c).end_time = (*tchart).last_time;
            }
            c = (*c).next;
        }
        p = (*p).next;
    }
    count
}

unsafe fn determine_display_tasks(tchart: *mut timechart, threshold: u64) -> c_int {
    let mut count = 0;
    let mut p = (*tchart).all_data;
    while !p.is_null() {
        (*p).display = 0;
        if (*p).start_time == 1 {
            (*p).start_time = (*tchart).first_time;
        }
        /* no exit marker, task kept running to the end */
        if (*p).end_time == 0 {
            (*p).end_time = (*tchart).last_time;
        }
        if (*p).total_time >= threshold {
            (*p).display = 1;
        }
        let mut c = (*p).all;
        while !c.is_null() {
            (*c).display = 0;
            if (*c).start_time == 1 {
                (*c).start_time = (*tchart).first_time;
            }
            if (*c).total_time >= threshold {
                (*c).display = 1;
                count += 1;
            }
            if (*c).end_time == 0 {
                (*c).end_time = (*tchart).last_time;
            }
            c = (*c).next;
        }
        p = (*p).next;
    }
    count
}

unsafe fn determine_display_io_tasks(timechart: *mut timechart, threshold: u64) -> c_int {
    let mut count = 0;
    let mut p = (*timechart).all_data;
    while !p.is_null() {
        /* no exit marker, task kept running to the end */
        if (*p).end_time == 0 {
            (*p).end_time = (*timechart).last_time;
        }
        let mut c = (*p).all;
        while !c.is_null() {
            (*c).display = 0;
            if (*c).total_bytes >= threshold {
                (*c).display = 1;
                count += 1;
            }
            if (*c).end_time == 0 {
                (*c).end_time = (*timechart).last_time;
            }
            c = (*c).next;
        }
        p = (*p).next;
    }
    count
}

unsafe fn write_svg_file(tchart: *mut timechart, filename: *const c_char) {
    let mut count: c_int;
    let mut thresh: c_int = if (*tchart).io_events != 0 { BYTES_THRESH } else { TIME_THRESH };
    if (*tchart).power_only {
        (*tchart).proc_num = 0;
    }
    /* We'd like to show at least proc_num tasks;
     * be less picky if we have fewer */
    loop {
        if !process_filter.is_null() {
            count = determine_display_tasks_filtered(tchart);
        } else if (*tchart).io_events != 0 {
            count = determine_display_io_tasks(tchart, thresh as u64);
        } else {
            count = determine_display_tasks(tchart, thresh as u64);
        }
        thresh /= 10;
        if !process_filter.is_null() || thresh == 0 || count >= (*tchart).proc_num {
            break;
        }
    }
    if (*tchart).proc_num == 0 {
        count = 0;
    }
    if (*tchart).io_events != 0 {
        open_svg(filename, 0, count, (*tchart).first_time, (*tchart).last_time);
        svg_time_grid(0.5);
        svg_io_legenda();
        draw_io_bars(tchart);
    } else {
        open_svg(filename, (*tchart).numcpus, count, (*tchart).first_time, (*tchart).last_time);
        svg_time_grid(0.0);
        svg_legenda();
        let mut i: u64 = 0;
        while i < (*tchart).numcpus as u64 {
            svg_cpu_box(i, (*tchart).max_freq, (*tchart).turbo_frequency);
            i += 1;
        }
        draw_cpu_usage(tchart);
        if (*tchart).proc_num != 0 {
            draw_process_bars(tchart);
        }
        if !(*tchart).tasks_only {
            draw_c_p_states(tchart);
        }
        if (*tchart).proc_num != 0 {
            draw_wakeups(tchart);
        }
    }
    svg_close();
}

unsafe fn timechart__release(tchart: *mut timechart) {
    let mut p = (*tchart).all_data;
    let mut pwr = (*tchart).power_events;
    let mut we = (*tchart).wake_events;
    while !p.is_null() {
        let next_pid = (*p).next;
        let mut c = (*p).all;
        while !c.is_null() {
            let next_comm = (*c).next;
            let mut cs = (*c).samples;
            let mut ios = (*c).io_samples;
            while !cs.is_null() {
                let next = (*cs).next;
                zfree(&mut (*cs).backtrace as *mut *const c_char as *mut *mut c_void);
                (*cs).next = null_mut();
                free(cs as *mut c_void);
                cs = next;
            }
            while !ios.is_null() {
                let next = (*ios).next;
                (*ios).next = null_mut();
                free(ios as *mut c_void);
                ios = next;
            }
            zfree(&mut (*c).comm as *mut *mut c_char as *mut *mut c_void);
            (*c).next = null_mut();
            free(c as *mut c_void);
            c = next_comm;
        }
        (*p).next = null_mut();
        free(p as *mut c_void);
        p = next_pid;
    }
    while !pwr.is_null() {
        let next = (*pwr).next;
        (*pwr).next = null_mut();
        free(pwr as *mut c_void);
        pwr = next;
    }
    while !we.is_null() {
        let next = (*we).next;
        zfree(&mut (*we).backtrace as *mut *const c_char as *mut *mut c_void);
        (*we).next = null_mut();
        free(we as *mut c_void);
        we = next;
    }
}

unsafe extern "C" fn process_header(section: *mut perf_file_section, ph: *mut perf_header, feat: c_int, fd: c_int, data: *mut c_void) -> c_int {
    let tchart = data as *mut timechart;
    match feat {
        HEADER_NRCPUS => {
            (*tchart).numcpus = (*ph).env.nr_cpus_avail;
            if (*tchart).numcpus as usize > MAX_CPUS {
                (*tchart).numcpus = MAX_CPUS as c_uint;
            }
        }
        HEADER_CPU_TOPOLOGY => {
            if !(*tchart).topology {
                return 0;
            }
            if svg_build_topology_map(&mut (*ph).env) != 0 {
                fprintf(2 as *mut FILE, cstr!("problem building topology\n"));
            }
        }
        _ => {}
    }
    0
}

unsafe fn __cmd_timechart(tchart: *mut timechart, output_name: *const c_char) -> c_int {
    let power_tracepoints = [
        evsel_str_handler { name: cstr!("power:cpu_idle"), handler: Some(process_sample_cpu_idle) },
        evsel_str_handler { name: cstr!("power:cpu_frequency"), handler: Some(process_sample_cpu_frequency) },
        evsel_str_handler { name: cstr!("sched:sched_wakeup"), handler: Some(process_sample_sched_wakeup) },
        evsel_str_handler { name: cstr!("sched:sched_switch"), handler: Some(process_sample_sched_switch) },
        evsel_str_handler { name: cstr!("power:power_start"), handler: Some(process_sample_power_start) },
        evsel_str_handler { name: cstr!("power:power_end"), handler: Some(process_sample_power_end) },
        evsel_str_handler { name: cstr!("power:power_frequency"), handler: Some(process_sample_power_frequency) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_read"), handler: Some(process_enter_read) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_pread64"), handler: Some(process_enter_read) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_readv"), handler: Some(process_enter_read) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_preadv"), handler: Some(process_enter_read) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_write"), handler: Some(process_enter_write) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_pwrite64"), handler: Some(process_enter_write) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_writev"), handler: Some(process_enter_write) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_pwritev"), handler: Some(process_enter_write) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_sync"), handler: Some(process_enter_sync) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_sync_file_range"), handler: Some(process_enter_sync) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_fsync"), handler: Some(process_enter_sync) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_msync"), handler: Some(process_enter_sync) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_recvfrom"), handler: Some(process_enter_rx) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_recvmmsg"), handler: Some(process_enter_rx) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_recvmsg"), handler: Some(process_enter_rx) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_sendto"), handler: Some(process_enter_tx) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_sendmsg"), handler: Some(process_enter_tx) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_sendmmsg"), handler: Some(process_enter_tx) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_epoll_pwait"), handler: Some(process_enter_poll) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_epoll_wait"), handler: Some(process_enter_poll) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_poll"), handler: Some(process_enter_poll) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_ppoll"), handler: Some(process_enter_poll) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_pselect6"), handler: Some(process_enter_poll) },
        evsel_str_handler { name: cstr!("syscalls:sys_enter_select"), handler: Some(process_enter_poll) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_read"), handler: Some(process_exit_read) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_pread64"), handler: Some(process_exit_read) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_readv"), handler: Some(process_exit_read) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_preadv"), handler: Some(process_exit_read) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_write"), handler: Some(process_exit_write) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_pwrite64"), handler: Some(process_exit_write) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_writev"), handler: Some(process_exit_write) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_pwritev"), handler: Some(process_exit_write) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_sync"), handler: Some(process_exit_sync) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_sync_file_range"), handler: Some(process_exit_sync) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_fsync"), handler: Some(process_exit_sync) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_msync"), handler: Some(process_exit_sync) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_recvfrom"), handler: Some(process_exit_rx) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_recvmmsg"), handler: Some(process_exit_rx) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_recvmsg"), handler: Some(process_exit_rx) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_sendto"), handler: Some(process_exit_tx) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_sendmsg"), handler: Some(process_exit_tx) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_sendmmsg"), handler: Some(process_exit_tx) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_epoll_pwait"), handler: Some(process_exit_poll) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_epoll_wait"), handler: Some(process_exit_poll) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_poll"), handler: Some(process_exit_poll) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_ppoll"), handler: Some(process_exit_poll) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_pselect6"), handler: Some(process_exit_poll) },
        evsel_str_handler { name: cstr!("syscalls:sys_exit_select"), handler: Some(process_exit_poll) },
    ];
    let mut data = perf_data { path: input_name, mode: PERF_DATA_MODE_READ, force: (*tchart).force };
    let mut ret = -EINVAL;
    perf_tool__init(&mut (*tchart).tool, true);
    (*tchart).tool.comm = Some(process_comm_event);
    (*tchart).tool.fork = Some(process_fork_event);
    (*tchart).tool.exit = Some(process_exit_event);
    (*tchart).tool.sample = Some(process_sample_event);
    let session = perf_session__new(&mut data, &mut (*tchart).tool);
    if IS_ERR(session as *const c_void) {
        return PTR_ERR(session as *const c_void);
    }
    (*tchart).session = session;
    symbol__init(perf_session__env(session));
    perf_header__process_sections(&mut (*session).header, perf_data__fd((*session).data), tchart as *mut c_void, process_header);
    if !perf_session__has_traces(session, cstr!("timechart record")) {
        perf_session__delete(session);
        return ret;
    }
    if perf_session__set_tracepoints_handlers(session, power_tracepoints.as_ptr()) != 0 {
        pr_err(cstr!("Initializing session tracepoint handlers failed\n"));
        perf_session__delete(session);
        return ret;
    }
    ret = perf_session__process_events(session);
    if ret == 0 {
        end_sample_processing(tchart);
        sort_pids(tchart);
        write_svg_file(tchart, output_name);
        pr_info(cstr!("Written %2.1f seconds of trace to %s.\n"), ((*tchart).last_time - (*tchart).first_time) as c_double / NSEC_PER_SEC as c_double, output_name);
    }
    perf_session__delete(session);
    ret
}

unsafe fn timechart__io_record(argc: c_int, argv: *mut *const c_char, output_data: *const c_char) -> c_int {
    let common_args = [cstr!("record"), cstr!("-a"), cstr!("-R"), cstr!("-c"), cstr!("1"), cstr!("-o"), output_data];
    let disk_events = [cstr!("syscalls:sys_enter_read"), cstr!("syscalls:sys_enter_pread64"), cstr!("syscalls:sys_enter_readv"), cstr!("syscalls:sys_enter_preadv"), cstr!("syscalls:sys_enter_write"), cstr!("syscalls:sys_enter_pwrite64"), cstr!("syscalls:sys_enter_writev"), cstr!("syscalls:sys_enter_pwritev"), cstr!("syscalls:sys_enter_sync"), cstr!("syscalls:sys_enter_sync_file_range"), cstr!("syscalls:sys_enter_fsync"), cstr!("syscalls:sys_enter_msync"), cstr!("syscalls:sys_exit_read"), cstr!("syscalls:sys_exit_pread64"), cstr!("syscalls:sys_exit_readv"), cstr!("syscalls:sys_exit_preadv"), cstr!("syscalls:sys_exit_write"), cstr!("syscalls:sys_exit_pwrite64"), cstr!("syscalls:sys_exit_writev"), cstr!("syscalls:sys_exit_pwritev"), cstr!("syscalls:sys_exit_sync"), cstr!("syscalls:sys_exit_sync_file_range"), cstr!("syscalls:sys_exit_fsync"), cstr!("syscalls:sys_exit_msync")];
    let net_events = [cstr!("syscalls:sys_enter_recvfrom"), cstr!("syscalls:sys_enter_recvmmsg"), cstr!("syscalls:sys_enter_recvmsg"), cstr!("syscalls:sys_enter_sendto"), cstr!("syscalls:sys_enter_sendmsg"), cstr!("syscalls:sys_enter_sendmmsg"), cstr!("syscalls:sys_exit_recvfrom"), cstr!("syscalls:sys_exit_recvmmsg"), cstr!("syscalls:sys_exit_recvmsg"), cstr!("syscalls:sys_exit_sendto"), cstr!("syscalls:sys_exit_sendmsg"), cstr!("syscalls:sys_exit_sendmmsg")];
    let poll_events = [cstr!("syscalls:sys_enter_epoll_pwait"), cstr!("syscalls:sys_enter_epoll_wait"), cstr!("syscalls:sys_enter_poll"), cstr!("syscalls:sys_enter_ppoll"), cstr!("syscalls:sys_enter_pselect6"), cstr!("syscalls:sys_enter_select"), cstr!("syscalls:sys_exit_epoll_pwait"), cstr!("syscalls:sys_exit_epoll_wait"), cstr!("syscalls:sys_exit_poll"), cstr!("syscalls:sys_exit_ppoll"), cstr!("syscalls:sys_exit_pselect6"), cstr!("syscalls:sys_exit_select")];
    let mut rec_argc = common_args.len() as c_uint + disk_events.len() as c_uint * 4 + net_events.len() as c_uint * 4 + poll_events.len() as c_uint * 4 + argc as c_uint;
    let rec_argv = calloc(rec_argc as usize + 1, size_of::<*const c_char>()) as *mut *const c_char;
    if rec_argv.is_null() {
        return -ENOMEM;
    }
    let mut filter: *mut c_char = null_mut();
    if asprintf(&mut filter, cstr!("common_pid != %d"), getpid()) < 0 {
        free(rec_argv as *mut c_void);
        return -ENOMEM;
    }
    let mut p = rec_argv;
    for arg in common_args {
        *p = arg;
        p = p.add(1);
    }
    for ev in disk_events {
        if !is_valid_tracepoint(ev) {
            rec_argc -= 4;
            continue;
        }
        *p = cstr!("-e"); p = p.add(1); *p = ev; p = p.add(1); *p = cstr!("--filter"); p = p.add(1); *p = filter; p = p.add(1);
    }
    for ev in net_events {
        if !is_valid_tracepoint(ev) {
            rec_argc -= 4;
            continue;
        }
        *p = cstr!("-e"); p = p.add(1); *p = ev; p = p.add(1); *p = cstr!("--filter"); p = p.add(1); *p = filter; p = p.add(1);
    }
    for ev in poll_events {
        if !is_valid_tracepoint(ev) {
            rec_argc -= 4;
            continue;
        }
        *p = cstr!("-e"); p = p.add(1); *p = ev; p = p.add(1); *p = cstr!("--filter"); p = p.add(1); *p = filter; p = p.add(1);
    }
    let mut i = 0;
    while i < argc {
        *p = *argv.add(i as usize);
        p = p.add(1);
        i += 1;
    }
    let ret = cmd_record(rec_argc, rec_argv);
    free(rec_argv as *mut c_void);
    free(filter as *mut c_void);
    ret
}

unsafe fn timechart__record(tchart: *mut timechart, argc: c_int, argv: *mut *const c_char, output_data: *const c_char) -> c_int {
    let common_args = [cstr!("record"), cstr!("-a"), cstr!("-R"), cstr!("-c"), cstr!("1"), cstr!("-o"), output_data];
    let backtrace_args = [cstr!("-g")];
    let power_args = [cstr!("-e"), cstr!("power:cpu_frequency"), cstr!("-e"), cstr!("power:cpu_idle")];
    let old_power_args = [cstr!("-e"), cstr!("power:power_start"), cstr!("-e"), cstr!("power:power_end"), cstr!("-e"), cstr!("power:power_frequency")];
    let tasks_args = [cstr!("-e"), cstr!("sched:sched_wakeup"), cstr!("-e"), cstr!("sched:sched_switch")];
    let mut backtrace_args_no = backtrace_args.len();
    let mut power_args_nr = power_args.len();
    let mut old_power_args_nr = old_power_args.len();
    let mut tasks_args_nr = tasks_args.len();
    if !is_valid_tracepoint(cstr!("power:cpu_idle")) && is_valid_tracepoint(cstr!("power:power_start")) {
        use_old_power_events = 1;
        power_args_nr = 0;
    } else {
        old_power_args_nr = 0;
    }
    if (*tchart).power_only {
        tasks_args_nr = 0;
    }
    if (*tchart).tasks_only {
        power_args_nr = 0;
        old_power_args_nr = 0;
    }
    if !(*tchart).with_backtrace {
        backtrace_args_no = 0;
    }
    let record_elems = common_args.len() + tasks_args_nr + power_args_nr + old_power_args_nr + backtrace_args_no;
    let rec_argc = record_elems as c_uint + argc as c_uint;
    let rec_argv = calloc(rec_argc as usize + 1, size_of::<*const c_char>()) as *mut *const c_char;
    if rec_argv.is_null() {
        return -ENOMEM;
    }
    let mut p = rec_argv;
    for arg in common_args { *p = arg; p = p.add(1); }
    for i in 0..backtrace_args_no { *p = backtrace_args[i]; p = p.add(1); }
    for i in 0..tasks_args_nr { *p = tasks_args[i]; p = p.add(1); }
    for i in 0..power_args_nr { *p = power_args[i]; p = p.add(1); }
    for i in 0..old_power_args_nr { *p = old_power_args[i]; p = p.add(1); }
    let mut j = 0;
    while j < argc {
        *p = *argv.add(j as usize);
        p = p.add(1);
        j += 1;
    }
    let ret = cmd_record(rec_argc, rec_argv);
    free(rec_argv as *mut c_void);
    ret
}

unsafe extern "C" fn parse_process(opt: *const option, arg: *const c_char, unset: c_int) -> c_int {
    if !arg.is_null() {
        add_process_filter(arg);
    }
    0
}

unsafe extern "C" fn parse_highlight(opt: *const option, arg: *const c_char, unset: c_int) -> c_int {
    let duration = strtoul(arg, null_mut(), 0);
    if svg_highlight != 0 || !svg_highlight_name.is_null() {
        return -1;
    }
    if duration != 0 {
        svg_highlight = duration;
    } else {
        svg_highlight_name = strdup(arg);
    }
    0
}

unsafe extern "C" fn parse_time(opt: *const option, arg: *const c_char, unset: c_int) -> c_int {
    let mut unit: c_char = b'n' as c_char;
    let value = (*opt).value as *mut u64;
    if sscanf(arg, cstr!("%llu%cs"), value, &mut unit) > 0 {
        match unit as u8 as char {
            'm' => *value *= NSEC_PER_MSEC,
            'u' => *value *= NSEC_PER_USEC,
            'n' => {}
            _ => return -1,
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn cmd_timechart(mut argc: c_int, argv: *mut *const c_char) -> c_int {
    let mut tchart: timechart = zeroed();
    tchart.proc_num = 15;
    tchart.min_time = NSEC_PER_MSEC;
    tchart.merge_dist = 1000;
    let mut output_name = cstr!("output.svg");
    let mut output_record_data = cstr!("perf.data");
    /* Option table macros from parse-options.h are represented here as opaque
     * arrays; the original option construction intent is preserved by the
     * parser calls and callbacks below. */
    let timechart_common_options: [option; 1] = [zeroed()];
    let timechart_options: [option; 1] = [zeroed()];
    let timechart_subcommands = [cstr!("record"), null()];
    let timechart_usage = [cstr!("perf timechart [<options>] {record}"), null()];
    let timechart_record_options: [option; 1] = [zeroed()];
    let timechart_record_usage = [cstr!("perf timechart record [<options>]"), null()];
    let mut ret: c_int;
    cpus_cstate_start_times = calloc(MAX_CPUS, size_of::<u64>()) as *mut u64;
    if cpus_cstate_start_times.is_null() {
        return -ENOMEM;
    }
    cpus_cstate_state = calloc(MAX_CPUS, size_of::<c_int>()) as *mut c_int;
    if cpus_cstate_state.is_null() {
        ret = -ENOMEM;
        timechart__release(&mut tchart);
        zfree(&mut cpus_cstate_start_times as *mut *mut u64 as *mut *mut c_void);
        return ret;
    }
    cpus_pstate_start_times = calloc(MAX_CPUS, size_of::<u64>()) as *mut u64;
    if cpus_pstate_start_times.is_null() {
        ret = -ENOMEM;
        timechart__release(&mut tchart);
        zfree(&mut cpus_cstate_start_times as *mut *mut u64 as *mut *mut c_void);
        zfree(&mut cpus_cstate_state as *mut *mut c_int as *mut *mut c_void);
        return ret;
    }
    cpus_pstate_state = calloc(MAX_CPUS, size_of::<u64>()) as *mut u64;
    if cpus_pstate_state.is_null() {
        ret = -ENOMEM;
        timechart__release(&mut tchart);
        zfree(&mut cpus_cstate_start_times as *mut *mut u64 as *mut *mut c_void);
        zfree(&mut cpus_cstate_state as *mut *mut c_int as *mut *mut c_void);
        zfree(&mut cpus_pstate_start_times as *mut *mut u64 as *mut *mut c_void);
        return ret;
    }
    argc = parse_options_subcommand(argc, argv, timechart_options.as_ptr(), timechart_subcommands.as_ptr(), timechart_usage.as_ptr(), PARSE_OPT_STOP_AT_NON_OPTION);
    if tchart.power_only && tchart.tasks_only {
        pr_err(cstr!("-P and -T options cannot be used at the same time.\n"));
        ret = -1;
        timechart__release(&mut tchart);
        zfree(&mut cpus_cstate_start_times as *mut *mut u64 as *mut *mut c_void);
        zfree(&mut cpus_cstate_state as *mut *mut c_int as *mut *mut c_void);
        zfree(&mut cpus_pstate_start_times as *mut *mut u64 as *mut *mut c_void);
        zfree(&mut cpus_pstate_state as *mut *mut u64 as *mut *mut c_void);
        return ret;
    }
    if argc != 0 && strlen(*argv) > 2 && strstarts(cstr!("record"), *argv) {
        argc = parse_options(argc, argv, timechart_record_options.as_ptr(), timechart_record_usage.as_ptr(), PARSE_OPT_STOP_AT_NON_OPTION);
        if tchart.power_only && tchart.tasks_only {
            pr_err(cstr!("-P and -T options cannot be used at the same time.\n"));
            ret = -1;
        } else if tchart.io_only {
            ret = timechart__io_record(argc, argv, output_record_data);
        } else {
            ret = timechart__record(&mut tchart, argc, argv, output_record_data);
        }
    } else if argc != 0 {
        usage_with_options(timechart_usage.as_ptr(), timechart_options.as_ptr());
    } else {
        setup_pager();
        ret = __cmd_timechart(&mut tchart, output_name);
    }
    timechart__release(&mut tchart);
    zfree(&mut cpus_cstate_start_times as *mut *mut u64 as *mut *mut c_void);
    zfree(&mut cpus_cstate_state as *mut *mut c_int as *mut *mut c_void);
    zfree(&mut cpus_pstate_start_times as *mut *mut u64 as *mut *mut c_void);
    zfree(&mut cpus_pstate_state as *mut *mut u64 as *mut *mut c_void);
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
