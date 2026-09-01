// SPDX-License-Identifier: GPL-2.0
// Rust source-level translation of perf/builtin-lock.c.
// C include dependencies intentionally remain external to this translated unit.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_float, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u32 = u32;
type u64 = u64;
type FILE = c_void;
type sig_atomic_t = c_int;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EEXIST: c_int = 17;
const INT_MAX: c_int = c_int::MAX;
const ULLONG_MAX: u64 = u64::MAX;
const WNOHANG: c_int = 1;
const SIGINT: c_int = 2;
const SIGCHLD: c_int = 17;
const SIGTERM: c_int = 15;
const PERF_DATA_MODE_READ: c_int = 0;
const PERF_CONTEXT_MAX: u64 = !0u64 << 63;

extern "C" {
    static mut input_name: *const c_char;
    static mut quiet: bool_;
    static mut verbose: c_int;
    static mut dump_trace: bool_;
    static mut symbol_conf: symbol_conf;
    static mut lockhash_table: *mut hlist_head;
    static mut stderr: *mut FILE;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn free(ptr: *mut c_void);
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn zalloc(size: usize) -> *mut c_void;
    fn zfree(ptr: *mut *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strncpy(dst: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strtok_r(s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    static mut errno: c_int;
    fn bzero(s: *mut c_void, n: usize);
    fn signal(sig: c_int, handler: Option<unsafe extern "C" fn(c_int)>) -> usize;
    fn sleep(seconds: c_uint) -> c_uint;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;

    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn scnprintf(buf: *mut c_char, size: c_int, fmt: *const c_char, ...) -> c_int;
    fn strlcpy(dst: *mut c_char, src: *const c_char, size: c_int) -> c_int;
    fn setup_pager();
    fn usage_with_options(usage: *mut *const c_char, options: *const option);
    fn parse_options_usage(usage: *const *const c_char, options: *const option, opt: *const c_char, unset: c_int);
    fn parse_options(argc: c_int, argv: *const *const c_char, options: *const option, usage: *const *const c_char, flags: c_int) -> c_int;
    fn parse_options_subcommand(argc: c_int, argv: *const *const c_char, options: *const option, subcommands: *const *const c_char, usage: *mut *const c_char, flags: c_int) -> c_int;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool_;

    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, rb_link: *mut *mut rb_node);
    fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);
    fn rb_first(root: *const rb_root) -> *mut rb_node;
    fn rb_next(node: *const rb_node) -> *mut rb_node;
    fn list_add(new_: *mut list_head, head: *mut list_head);
    fn list_add_tail(new_: *mut list_head, head: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn list_empty(head: *const list_head) -> c_int;
    fn hash_long(val: c_ulong, bits: c_uint) -> c_ulong;

    fn perf_sample__strval(sample: *mut perf_sample, name: *const c_char) -> *const c_char;
    fn perf_sample__intval(sample: *mut perf_sample, name: *const c_char) -> u64;
    fn perf_event__process_event_update(tool: *const perf_tool, event: *mut perf_event, pevlist: *mut *mut evlist) -> c_int;
    fn perf_session__set_tracepoints_handlers(session: *mut perf_session, handlers: *const evsel_str_handler) -> c_int;
    fn perf_event__name(type_: c_uint) -> *const c_char;
    fn perf_tool__init(tool: *mut perf_tool, ordered_events: bool_);
    fn perf_session__new(data: *mut perf_data, tool: *mut perf_tool) -> *mut perf_session;
    fn __perf_session__new(data: *mut perf_data, tool: *mut perf_tool, trace_event_repipe: bool_, env: *mut perf_env) -> *mut perf_session;
    fn perf_session__delete(session: *mut perf_session);
    fn perf_session__process_events(session: *mut perf_session) -> c_int;
    fn perf_session__has_traces(session: *mut perf_session, msg: *const c_char) -> bool_;
    fn perf_session__env(session: *mut perf_session) -> *mut perf_env;
    fn perf_session__findnew(session: *mut perf_session, tid: c_int) -> *mut thread;
    fn symbol__init(env: *mut perf_env) -> c_int;
    fn machine__findnew_thread(machine: *mut machine, pid: c_int, tid: c_int) -> *mut thread;
    fn machine__kernel_map(machine: *mut machine) -> *mut map;
    fn machine__find_kernel_symbol(machine: *mut machine, addr: u64, mapp: *mut *mut map) -> *mut symbol;
    fn machine__find_kernel_symbol_by_name(machine: *mut machine, name: *const c_char, mapp: *mut *mut map) -> *mut symbol;
    fn machine__is_lock_function(machine: *mut machine, ip: u64) -> bool_;
    fn thread__put(thread: *mut thread);
    fn thread__comm_str(thread: *mut thread) -> *const c_char;
    fn get_tls_callchain_cursor() -> *mut callchain_cursor;
    fn thread__resolve_callchain(thread: *mut thread, cursor: *mut callchain_cursor, sample: *mut perf_sample, parent: *mut c_void, root_al: *mut c_void, max_stack: c_int) -> c_int;
    fn callchain_cursor_commit(cursor: *mut callchain_cursor);
    fn callchain_cursor_current(cursor: *mut callchain_cursor) -> *mut callchain_cursor_node;
    fn callchain_cursor_advance(cursor: *mut callchain_cursor);
    fn map__map_ip(map: *mut map, ip: u64) -> u64;
    fn map__unmap_ip(map: *mut map, ip: u64) -> u64;
    fn map__load(map: *mut map) -> c_int;
    fn needs_callstack() -> bool_;
    fn match_callstack_filter(machine: *mut machine, callstack: *mut u64, max_stack: c_int) -> bool_;
    fn pop_owner_stack_trace(con: *mut lock_contention) -> *mut lock_stat;
    fn target__validate(target: *mut target) -> c_int;
    fn target__strerror(target: *mut target, err: c_int, buf: *mut c_char, size: usize);
    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn evlist__create_maps(evlist: *mut evlist, target: *mut target) -> c_int;
    fn evlist__prepare_workload(evlist: *mut evlist, target: *mut target, argv: *const *const c_char, pipe_output: bool_, argv_exec: *mut c_void) -> c_int;
    fn evlist__start_workload(evlist: *mut evlist);
    fn evlist__workload_pid(evlist: *mut evlist) -> c_int;
    fn evlist__find_evsel_by_str(evlist: *mut evlist, str_: *const c_char) -> *mut evsel;
    fn lock_contention_prepare(con: *mut lock_contention) -> c_int;
    fn lock_contention_start();
    fn lock_contention_stop();
    fn lock_contention_read(con: *mut lock_contention) -> c_int;
    fn lock_contention_finish(con: *mut lock_contention);
    fn perf_env__init(env: *mut perf_env);
    fn perf_env__exit(env: *mut perf_env);
    fn is_valid_tracepoint(name: *const c_char) -> bool_;
    fn cmd_record(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_script(argc: c_int, argv: *const *const c_char) -> c_int;
    fn sysctl__max_stack() -> c_long;
    fn parse_call_stack(opt: *const option, str_: *const c_char, unset: c_int) -> c_int;
    fn cgroup__new(name: *const c_char, do_open: bool_) -> *mut cgroup;
    fn read_cgroup_id(cgrp: *mut cgroup) -> c_int;
    fn cgroup__put(cgrp: *mut cgroup);

    static mut perf_event__process_attr: usize;
    static mut perf_event__process_comm: usize;
    static mut perf_event__process_mmap: usize;
    static mut perf_event__process_mmap2: usize;
    static mut perf_event__process_namespaces: usize;
    static mut perf_event__process_tracing_data: usize;
}

#[repr(C)] struct rb_node { rb_left: *mut rb_node, rb_right: *mut rb_node, rb_parent_color: c_ulong }
#[repr(C)] struct rb_root { rb_node: *mut rb_node }
#[repr(C)] struct list_head { next: *mut list_head, prev: *mut list_head }
#[repr(C)] struct hlist_head { first: *mut hlist_node }
#[repr(C)] struct hlist_node { next: *mut hlist_node, pprev: *mut *mut hlist_node }
#[repr(C)] struct perf_session { machines: machines, evlist: *mut evlist }
#[repr(C)] struct machines { host: machine }
#[repr(C)] struct machine { _priv: [u8; 0] }
#[repr(C)] struct target { system_wide: bool_, cpu_list: *const c_char, pid: *const c_char, tid: *const c_char }
#[repr(C)] struct perf_tool { attr: usize, event_update: unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut *mut evlist) -> c_int, sample: unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int, comm: usize, mmap: usize, mmap2: usize, namespaces: usize, tracing_data: usize }
#[repr(C)] struct perf_data { path: *const c_char, mode: c_int, force: bool_, is_pipe: bool_ }
#[repr(C)] struct perf_env { _priv: [u8; 0] }
#[repr(C)] struct option { value: *mut c_void }
#[repr(C)] struct evlist { _priv: [u8; 0] }
#[repr(C)] struct evsel { handler: Option<unsafe extern "C" fn(*mut perf_sample) -> c_int> }
#[repr(C)] struct perf_event_header { type_: c_uint }
#[repr(C)] union perf_event { header: core::mem::ManuallyDrop<perf_event_header> }
#[repr(C)] struct callchain { nr: u64, ips: [u64; 0] }
#[repr(C)] struct perf_sample { tid: u32, pid: c_int, time: u64, callchain: *mut callchain, evsel: *mut evsel, file_offset: u64 }
#[repr(C)] struct map { _priv: [u8; 0] }
#[repr(C)] struct symbol { start: u64, name: *const c_char }
#[repr(C)] struct map_symbol { map: *mut map, sym: *mut symbol }
#[repr(C)] struct callchain_cursor { _priv: [u8; 0] }
#[repr(C)] struct callchain_cursor_node { ms: map_symbol, ip: u64 }
#[repr(C)] struct thread { _priv: [u8; 0] }
#[repr(C)] struct cgroup { id: u64 }
#[repr(C)] struct symbol_conf { field_sep: *const c_char, vmlinux_name: *const c_char, kallsyms_name: *const c_char, allow_aliases: bool_ }

#[repr(C)] struct thread_stat { rb: rb_node, tid: u32, seq_list: list_head }
#[repr(C)] struct lock_seq_stat { list: list_head, state: c_int, addr: u64, read_count: c_int, prev_event_time: u64 }
#[repr(C)] struct lock_filter { types: *mut c_uint, nr_types: c_int, addrs: *mut c_ulong, nr_addrs: c_int, syms: *mut *mut c_char, nr_syms: c_int, cgrps: *mut u64, nr_cgrps: c_int, slabs: *mut *mut c_char, nr_slabs: c_int }
#[repr(C)] struct lock_delay { sym: *mut c_char, time: c_ulong }
#[repr(C)] struct lock_contention_fails { task: c_int, stack: c_int, time: c_int, data: c_int }
#[repr(C)] struct lock_contention { target: *mut target, map_nr_entries: c_ulong, max_stack: c_int, stack_skip: c_int, filters: *mut lock_filter, delays: *mut lock_delay, nr_delays: c_int, save_callstack: bool_, owner: bool_, cgroups: rb_root, result: *mut hlist_head, machine: *mut machine, aggr_mode: lock_aggr_mode, evlist: *mut evlist, nr_filtered: c_int, fails: lock_contention_fails }
#[repr(C)] struct lock_stat { rb: rb_node, hash_entry: hlist_node, addr: u64, name: *const c_char, flags: c_uint, nr_acquired: u64, nr_contended: u64, avg_wait_time: u64, wait_time_total: u64, wait_time_max: u64, wait_time_min: u64, nr_trylock: u64, nr_readlock: u64, nr_acquire: u64, nr_release: u64, broken: c_int, combined: c_int, callstack: *mut u64 }
#[repr(C)] struct evsel_str_handler { name: *const c_char, handler: Option<unsafe extern "C" fn(*mut perf_sample) -> c_int> }

type lock_aggr_mode = c_int;
const LOCK_AGGR_ADDR: lock_aggr_mode = 0;
const LOCK_AGGR_TASK: lock_aggr_mode = 1;
const LOCK_AGGR_CALLER: lock_aggr_mode = 2;
const LOCK_AGGR_CGROUP: lock_aggr_mode = 3;
const LOCKHASH_SIZE: usize = 4096;
const MAX_ENTRIES: c_ulong = 10240;
const CONTENTION_STACK_DEPTH: c_int = 8;
const CONTENTION_STACK_SKIP: c_int = 3;
const LCB_F_SPIN: c_uint = 1 << 0;
const LCB_F_READ: c_uint = 1 << 1;
const LCB_F_WRITE: c_uint = 1 << 2;
const LCB_F_RT: c_uint = 1 << 3;
const LCB_F_PERCPU: c_uint = 1 << 4;
const LCB_F_MUTEX: c_uint = 1 << 5;
const LCB_F_TYPE_MASK: c_uint = 0xffff;
const SEQ_STATE_UNINITIALIZED: c_int = 0;
const SEQ_STATE_ACQUIRING: c_int = 1;
const SEQ_STATE_ACQUIRED: c_int = 2;
const SEQ_STATE_READ_ACQUIRED: c_int = 3;
const SEQ_STATE_CONTENDED: c_int = 4;
const SEQ_STATE_RELEASED: c_int = 5;
const PARSE_OPT_STOP_AT_NON_OPTION: c_int = 1;

const fn cstr(bytes: &'static [u8]) -> *const c_char { bytes.as_ptr() as *const c_char }
unsafe fn INIT_LIST_HEAD(list: *mut list_head) { (*list).next = list; (*list).prev = list; }
unsafe fn INIT_HLIST_HEAD(h: *mut hlist_head) { (*h).first = ptr::null_mut(); }
unsafe fn BUG_ON<T>(_v: T) {}
unsafe fn IS_ERR<T>(p: *mut T) -> bool { (p as isize) < 0 && (p as isize) > -4096 }
unsafe fn PTR_ERR<T>(p: *mut T) -> c_int { p as isize as c_int }
unsafe fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize { N }
unsafe fn container_of<T>(ptr: *mut c_void, offset: usize) -> *mut T { (ptr as *mut u8).sub(offset) as *mut T }
unsafe fn lock_stat_find(_key: u64) -> *mut lock_stat { ptr::null_mut() }
unsafe fn lock_stat_findnew(_key: u64, _name: *const c_char, _flags: c_uint) -> *mut lock_stat { ptr::null_mut() }

static mut session: *mut perf_session = ptr::null_mut();
static mut target: target = target { system_wide: false, cpu_list: ptr::null(), pid: ptr::null(), tid: ptr::null() };
static mut thread_stats: rb_root = rb_root { rb_node: ptr::null_mut() };
static mut combine_locks: bool_ = false;
static mut show_thread_stats: bool_ = false;
static mut show_lock_addrs: bool_ = false;
static mut show_lock_owner: bool_ = false;
static mut show_lock_cgroups: bool_ = false;
static mut use_bpf: bool_ = false;
static mut bpf_map_entries: c_ulong = MAX_ENTRIES;
static mut max_stack_depth: c_int = CONTENTION_STACK_DEPTH;
static mut stack_skip: c_int = CONTENTION_STACK_SKIP;
static mut print_nr_entries: c_int = INT_MAX / 2;
static mut output_name: *const c_char = ptr::null();
static mut lock_output: *mut FILE = ptr::null_mut();
static mut filters: lock_filter = lock_filter { types: ptr::null_mut(), nr_types: 0, addrs: ptr::null_mut(), nr_addrs: 0, syms: ptr::null_mut(), nr_syms: 0, cgrps: ptr::null_mut(), nr_cgrps: 0, slabs: ptr::null_mut(), nr_slabs: 0 };
static mut delays: *mut lock_delay = ptr::null_mut();
static mut nr_delays: c_int = 0;
static mut aggr_mode: lock_aggr_mode = LOCK_AGGR_ADDR;

unsafe fn thread_stat_find(tid: u32) -> *mut thread_stat {
    let mut node = thread_stats.rb_node;
    while !node.is_null() {
        let st = container_of::<thread_stat>(node as *mut c_void, 0);
        if (*st).tid == tid { return st; }
        else if tid < (*st).tid { node = (*node).rb_left; }
        else { node = (*node).rb_right; }
    }
    ptr::null_mut()
}

unsafe fn thread_stat_insert(new_: *mut thread_stat) {
    let mut rb: *mut *mut rb_node = &mut thread_stats.rb_node;
    let mut parent: *mut rb_node = ptr::null_mut();
    while !(*rb).is_null() {
        let p = container_of::<thread_stat>(*rb as *mut c_void, 0);
        parent = *rb;
        if (*new_).tid < (*p).tid { rb = &mut (**rb).rb_left; }
        else if (*new_).tid > (*p).tid { rb = &mut (**rb).rb_right; }
        else { BUG_ON(cstr(b"inserting invalid thread_stat\n\0")); }
    }
    rb_link_node(&mut (*new_).rb, parent, rb);
    rb_insert_color(&mut (*new_).rb, &mut thread_stats);
}

unsafe fn thread_stat_findnew_after_first(tid: u32) -> *mut thread_stat {
    let st0 = thread_stat_find(tid);
    if !st0.is_null() { return st0; }
    let st = zalloc(size_of::<thread_stat>()) as *mut thread_stat;
    if st.is_null() { pr_err(cstr(b"memory allocation failed\n\0")); return ptr::null_mut(); }
    (*st).tid = tid;
    INIT_LIST_HEAD(&mut (*st).seq_list);
    thread_stat_insert(st);
    st
}

type thread_stat_findnew_fn = unsafe fn(u32) -> *mut thread_stat;
static mut thread_stat_findnew: thread_stat_findnew_fn = thread_stat_findnew_first;

unsafe fn thread_stat_findnew_first(tid: u32) -> *mut thread_stat {
    let st = zalloc(size_of::<thread_stat>()) as *mut thread_stat;
    if st.is_null() { pr_err(cstr(b"memory allocation failed\n\0")); return ptr::null_mut(); }
    (*st).tid = tid;
    INIT_LIST_HEAD(&mut (*st).seq_list);
    rb_link_node(&mut (*st).rb, ptr::null_mut(), &mut thread_stats.rb_node);
    rb_insert_color(&mut (*st).rb, &mut thread_stats);
    thread_stat_findnew = thread_stat_findnew_after_first;
    st
}

macro_rules! single_key {
    ($name:ident, $member:ident) => {
        unsafe extern "C" fn $name(one: *mut lock_stat, two: *mut lock_stat) -> c_int {
            ((*one).$member > (*two).$member) as c_int
        }
    };
}
single_key!(lock_stat_key_nr_acquired, nr_acquired);
single_key!(lock_stat_key_nr_contended, nr_contended);
single_key!(lock_stat_key_avg_wait_time, avg_wait_time);
single_key!(lock_stat_key_wait_time_total, wait_time_total);
single_key!(lock_stat_key_wait_time_max, wait_time_max);

unsafe extern "C" fn lock_stat_key_wait_time_min(one: *mut lock_stat, two: *mut lock_stat) -> c_int {
    let mut s1 = (*one).wait_time_min;
    let mut s2 = (*two).wait_time_min;
    if s1 == ULLONG_MAX { s1 = 0; }
    if s2 == ULLONG_MAX { s2 = 0; }
    (s1 > s2) as c_int
}

#[repr(C)]
struct lock_key {
    /* name: the value for specify by user */
    name: *const c_char,
    /* header: the string printed on the header line */
    header: *const c_char,
    /* len: the printing width of the field */
    len: c_int,
    /* key: a pointer to function to compare two lock stats for sorting */
    key: Option<unsafe extern "C" fn(*mut lock_stat, *mut lock_stat) -> c_int>,
    /* print: a pointer to function to print a given lock stats */
    print: Option<unsafe extern "C" fn(*mut lock_key, *mut lock_stat)>,
    /* list: list entry to link this */
    list: list_head,
}

#[repr(C)] struct time_table { base: c_float, unit: *const c_char }
unsafe fn lock_stat_key_print_time(nsec: c_ulonglong, len: c_int) {
    static table: [time_table; 6] = [
        time_table { base: 1e9f32 * 3600.0, unit: cstr(b"h \0") },
        time_table { base: 1e9f32 * 60.0, unit: cstr(b"m \0") },
        time_table { base: 1e9f32, unit: cstr(b"s \0") },
        time_table { base: 1e6f32, unit: cstr(b"ms\0") },
        time_table { base: 1e3f32, unit: cstr(b"us\0") },
        time_table { base: 0.0, unit: ptr::null() },
    ];
    if len == 0 {
        fprintf(lock_output, cstr(b"%llu\0"), nsec);
        return;
    }
    let mut i = 0usize;
    while !table[i].unit.is_null() {
        if (nsec as c_float) >= table[i].base {
            fprintf(lock_output, cstr(b"%*.2f %s\0"), len - 3, (nsec as c_double) / (table[i].base as c_double), table[i].unit);
            return;
        }
        i += 1;
    }
    fprintf(lock_output, cstr(b"%*llu %s\0"), len - 3, nsec, cstr(b"ns\0"));
}

macro_rules! print_key {
    ($name:ident, $member:ident) => {
        unsafe extern "C" fn $name(key: *mut lock_key, ls: *mut lock_stat) {
            fprintf(lock_output, cstr(b"%*llu\0"), (*key).len, (*ls).$member as c_ulonglong);
        }
    };
}
macro_rules! print_time {
    ($name:ident, $member:ident) => {
        unsafe extern "C" fn $name(key: *mut lock_key, ls: *mut lock_stat) {
            lock_stat_key_print_time((*ls).$member as c_ulonglong, (*key).len);
        }
    };
}
print_key!(lock_stat_key_print_nr_acquired, nr_acquired);
print_key!(lock_stat_key_print_nr_contended, nr_contended);
print_time!(lock_stat_key_print_avg_wait_time, avg_wait_time);
print_time!(lock_stat_key_print_wait_time_total, wait_time_total);
print_time!(lock_stat_key_print_wait_time_max, wait_time_max);

unsafe extern "C" fn lock_stat_key_print_wait_time_min(key: *mut lock_key, ls: *mut lock_stat) {
    let mut wait_time = (*ls).wait_time_min;
    if wait_time == ULLONG_MAX { wait_time = 0; }
    lock_stat_key_print_time(wait_time, (*key).len);
}

static mut sort_key: *const c_char = cstr(b"acquired\0");
static mut compare: Option<unsafe extern "C" fn(*mut lock_stat, *mut lock_stat) -> c_int> = None;
static mut sorted: rb_root = rb_root { rb_node: ptr::null_mut() };
static mut result: rb_root = rb_root { rb_node: ptr::null_mut() };
static mut lock_keys: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
static mut output_fields: *const c_char = ptr::null();

macro_rules! key_item {
    ($name:expr, $header:expr, $key:ident, $print:ident, $len:expr) => {
        lock_key { name: cstr($name), header: cstr($header), len: $len, key: Some($key), print: Some($print), list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() } }
    };
}
static mut report_keys: [lock_key; 7] = [
    key_item!(b"acquired\0", b"acquired\0", lock_stat_key_nr_acquired, lock_stat_key_print_nr_acquired, 10),
    key_item!(b"contended\0", b"contended\0", lock_stat_key_nr_contended, lock_stat_key_print_nr_contended, 10),
    key_item!(b"avg_wait\0", b"avg wait\0", lock_stat_key_avg_wait_time, lock_stat_key_print_avg_wait_time, 12),
    key_item!(b"wait_total\0", b"total wait\0", lock_stat_key_wait_time_total, lock_stat_key_print_wait_time_total, 12),
    key_item!(b"wait_max\0", b"max wait\0", lock_stat_key_wait_time_max, lock_stat_key_print_wait_time_max, 12),
    key_item!(b"wait_min\0", b"min wait\0", lock_stat_key_wait_time_min, lock_stat_key_print_wait_time_min, 12),
    lock_key { name: ptr::null(), header: ptr::null(), len: 0, key: None, print: None, list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() } },
];
static mut contention_keys: [lock_key; 6] = [
    key_item!(b"contended\0", b"contended\0", lock_stat_key_nr_contended, lock_stat_key_print_nr_contended, 10),
    key_item!(b"wait_total\0", b"total wait\0", lock_stat_key_wait_time_total, lock_stat_key_print_wait_time_total, 12),
    key_item!(b"wait_max\0", b"max wait\0", lock_stat_key_wait_time_max, lock_stat_key_print_wait_time_max, 12),
    key_item!(b"wait_min\0", b"min wait\0", lock_stat_key_wait_time_min, lock_stat_key_print_wait_time_min, 12),
    key_item!(b"avg_wait\0", b"avg wait\0", lock_stat_key_avg_wait_time, lock_stat_key_print_avg_wait_time, 12),
    lock_key { name: ptr::null(), header: ptr::null(), len: 0, key: None, print: None, list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() } },
];

unsafe fn select_key(contention: bool_) -> c_int {
    let keys = if contention { contention_keys.as_mut_ptr() } else { report_keys.as_mut_ptr() };
    let mut i = 0;
    while !(*keys.add(i)).name.is_null() {
        if strcmp((*keys.add(i)).name, sort_key) == 0 {
            compare = (*keys.add(i)).key;
            if list_empty(&(*keys.add(i)).list) != 0 { list_add_tail(&mut (*keys.add(i)).list, &mut lock_keys); }
            return 0;
        }
        i += 1;
    }
    pr_err(cstr(b"Unknown compare key: %s\n\0"), sort_key);
    -1
}

unsafe fn add_output_field(contention: bool_, name: *mut c_char) -> c_int {
    let keys = if contention { contention_keys.as_mut_ptr() } else { report_keys.as_mut_ptr() };
    let mut i = 0;
    while !(*keys.add(i)).name.is_null() {
        if strcmp((*keys.add(i)).name, name) == 0 {
            if list_empty(&(*keys.add(i)).list) != 0 { list_add_tail(&mut (*keys.add(i)).list, &mut lock_keys); }
            return 0;
        }
        i += 1;
    }
    pr_err(cstr(b"Unknown output field: %s\n\0"), name);
    -1
}

unsafe fn setup_output_field(contention: bool_, str_: *const c_char) -> c_int {
    let keys = if contention { contention_keys.as_mut_ptr() } else { report_keys.as_mut_ptr() };
    if str_.is_null() {
        let mut i = 0;
        while !(*keys.add(i)).name.is_null() {
            list_add_tail(&mut (*keys.add(i)).list, &mut lock_keys);
            i += 1;
        }
        return 0;
    }
    let mut i = 0;
    while !(*keys.add(i)).name.is_null() { INIT_LIST_HEAD(&mut (*keys.add(i)).list); i += 1; }
    let orig = strdup(str_);
    if orig.is_null() { return -ENOMEM; }
    let mut tmp = orig;
    let mut ret = 0;
    loop {
        let tok = strsep(&mut tmp, cstr(b",\0"));
        if tok.is_null() { break; }
        ret = add_output_field(contention, tok);
        if ret < 0 { break; }
    }
    free(orig as *mut c_void);
    ret
}

unsafe fn combine_lock_stats(st: *mut lock_stat) {
    let mut rb: *mut *mut rb_node = &mut sorted.rb_node;
    let mut parent: *mut rb_node = ptr::null_mut();
    while !(*rb).is_null() {
        let p = container_of::<lock_stat>(*rb as *mut c_void, 0);
        parent = *rb;
        let ret = if !(*st).name.is_null() && !(*p).name.is_null() { strcmp((*st).name, (*p).name) } else { ((!(*st).name.is_null()) as c_int) - ((!(*p).name.is_null()) as c_int) };
        if ret == 0 {
            (*p).nr_acquired += (*st).nr_acquired;
            (*p).nr_contended += (*st).nr_contended;
            (*p).wait_time_total += (*st).wait_time_total;
            if (*p).nr_contended != 0 { (*p).avg_wait_time = (*p).wait_time_total / (*p).nr_contended; }
            if (*p).wait_time_min > (*st).wait_time_min { (*p).wait_time_min = (*st).wait_time_min; }
            if (*p).wait_time_max < (*st).wait_time_max { (*p).wait_time_max = (*st).wait_time_max; }
            (*p).broken |= (*st).broken;
            (*st).combined = 1;
            return;
        }
        if ret < 0 { rb = &mut (**rb).rb_left; } else { rb = &mut (**rb).rb_right; }
    }
    rb_link_node(&mut (*st).rb, parent, rb);
    rb_insert_color(&mut (*st).rb, &mut sorted);
}

unsafe fn insert_to(rr: *mut rb_root, st: *mut lock_stat, bigger: Option<unsafe extern "C" fn(*mut lock_stat, *mut lock_stat) -> c_int>) {
    let mut rb: *mut *mut rb_node = &mut (*rr).rb_node;
    let mut parent: *mut rb_node = ptr::null_mut();
    while !(*rb).is_null() {
        let p = container_of::<lock_stat>(*rb as *mut c_void, 0);
        parent = *rb;
        if bigger.unwrap()(st, p) != 0 { rb = &mut (**rb).rb_left; } else { rb = &mut (**rb).rb_right; }
    }
    rb_link_node(&mut (*st).rb, parent, rb);
    rb_insert_color(&mut (*st).rb, rr);
}

unsafe fn insert_to_result(st: *mut lock_stat, bigger: Option<unsafe extern "C" fn(*mut lock_stat, *mut lock_stat) -> c_int>) {
    if combine_locks && (*st).combined != 0 { return; }
    insert_to(&mut result, st, bigger);
}

unsafe fn pop_from(rr: *mut rb_root) -> *mut lock_stat {
    let mut node = (*rr).rb_node;
    if node.is_null() { return ptr::null_mut(); }
    while !(*node).rb_left.is_null() { node = (*node).rb_left; }
    rb_erase(node, rr);
    container_of::<lock_stat>(node as *mut c_void, 0)
}
unsafe fn pop_from_result() -> *mut lock_stat { pop_from(&mut result) }

#[repr(C)]
struct trace_lock_handler {
    acquire_event: Option<unsafe extern "C" fn(*mut perf_sample) -> c_int>,
    acquired_event: Option<unsafe extern "C" fn(*mut perf_sample) -> c_int>,
    contended_event: Option<unsafe extern "C" fn(*mut perf_sample) -> c_int>,
    release_event: Option<unsafe extern "C" fn(*mut perf_sample) -> c_int>,
    contention_begin_event: Option<unsafe extern "C" fn(*mut perf_sample) -> c_int>,
    contention_end_event: Option<unsafe extern "C" fn(*mut perf_sample) -> c_int>,
}

unsafe fn get_seq(ts: *mut thread_stat, addr: u64) -> *mut lock_seq_stat {
    let mut pos = (*ts).seq_list.next;
    while pos != &mut (*ts).seq_list {
        let seq = container_of::<lock_seq_stat>(pos as *mut c_void, 0);
        if (*seq).addr == addr { return seq; }
        pos = (*pos).next;
    }
    let seq = zalloc(size_of::<lock_seq_stat>()) as *mut lock_seq_stat;
    if seq.is_null() { pr_err(cstr(b"memory allocation failed\n\0")); return ptr::null_mut(); }
    (*seq).state = SEQ_STATE_UNINITIALIZED;
    (*seq).addr = addr;
    list_add(&mut (*seq).list, &mut (*ts).seq_list);
    seq
}

const BROKEN_ACQUIRE: usize = 0;
const BROKEN_ACQUIRED: usize = 1;
const BROKEN_CONTENDED: usize = 2;
const BROKEN_RELEASE: usize = 3;
const BROKEN_MAX: usize = 4;
static mut bad_hist: [c_int; BROKEN_MAX] = [0; BROKEN_MAX];
const TRY_LOCK: c_int = 1;
const READ_LOCK: c_int = 2;

unsafe fn get_key_by_aggr_mode_simple(key: *mut u64, addr: u64, tid: u32) -> c_int {
    match aggr_mode {
        LOCK_AGGR_ADDR => *key = addr,
        LOCK_AGGR_TASK => *key = tid as u64,
        LOCK_AGGR_CALLER | LOCK_AGGR_CGROUP | _ => { pr_err(cstr(b"Invalid aggregation mode: %d\n\0"), aggr_mode); return -EINVAL; }
    }
    0
}

unsafe fn get_key_by_aggr_mode(key: *mut u64, addr: u64, sample: *mut perf_sample) -> c_int {
    if aggr_mode == LOCK_AGGR_CALLER { *key = callchain_id(sample); return 0; }
    get_key_by_aggr_mode_simple(key, addr, (*sample).tid)
}

unsafe extern "C" fn report_lock_acquire_event(sample: *mut perf_sample) -> c_int {
    let name = perf_sample__strval(sample, cstr(b"name\0"));
    let addr = perf_sample__intval(sample, cstr(b"lockdep_addr\0"));
    let flag = perf_sample__intval(sample, cstr(b"flags\0")) as c_int;
    let mut key = 0;
    let ret = get_key_by_aggr_mode_simple(&mut key, addr, (*sample).tid);
    if ret < 0 { return ret; }
    let ls = lock_stat_findnew(key, name, 0);
    if ls.is_null() { return -ENOMEM; }
    let ts = thread_stat_findnew((*sample).tid);
    if ts.is_null() { return -ENOMEM; }
    let seq = get_seq(ts, addr);
    if seq.is_null() { return -ENOMEM; }
    match (*seq).state {
        SEQ_STATE_UNINITIALIZED | SEQ_STATE_RELEASED => {
            if flag == 0 { (*seq).state = SEQ_STATE_ACQUIRING; } else {
                if flag & TRY_LOCK != 0 { (*ls).nr_trylock += 1; }
                if flag & READ_LOCK != 0 { (*ls).nr_readlock += 1; }
                (*seq).state = SEQ_STATE_READ_ACQUIRED; (*seq).read_count = 1; (*ls).nr_acquired += 1;
            }
        }
        SEQ_STATE_READ_ACQUIRED => {
            if flag & READ_LOCK != 0 { (*seq).read_count += 1; (*ls).nr_acquired += 1; return 0; }
            if (*ls).broken == 0 { (*ls).broken = 1; bad_hist[BROKEN_ACQUIRE] += 1; }
            list_del_init(&mut (*seq).list); free(seq as *mut c_void); return 0;
        }
        SEQ_STATE_ACQUIRED | SEQ_STATE_ACQUIRING | SEQ_STATE_CONTENDED => {
            if (*ls).broken == 0 { (*ls).broken = 1; bad_hist[BROKEN_ACQUIRE] += 1; }
            list_del_init(&mut (*seq).list); free(seq as *mut c_void); return 0;
        }
        _ => BUG_ON(cstr(b"Unknown state of lock sequence found!\n\0")),
    }
    (*ls).nr_acquire += 1;
    (*seq).prev_event_time = (*sample).time;
    0
}

unsafe extern "C" fn report_lock_acquired_event(sample: *mut perf_sample) -> c_int {
    let name = perf_sample__strval(sample, cstr(b"name\0"));
    let addr = perf_sample__intval(sample, cstr(b"lockdep_addr\0"));
    let mut key = 0;
    let ret = get_key_by_aggr_mode_simple(&mut key, addr, (*sample).tid);
    if ret < 0 { return ret; }
    let ls = lock_stat_findnew(key, name, 0);
    if ls.is_null() { return -ENOMEM; }
    let ts = thread_stat_findnew((*sample).tid);
    if ts.is_null() { return -ENOMEM; }
    let seq = get_seq(ts, addr);
    if seq.is_null() { return -ENOMEM; }
    match (*seq).state {
        SEQ_STATE_UNINITIALIZED => return 0,
        SEQ_STATE_ACQUIRING => {}
        SEQ_STATE_CONTENDED => {
            let contended_term = (*sample).time - (*seq).prev_event_time;
            (*ls).wait_time_total += contended_term;
            if contended_term < (*ls).wait_time_min { (*ls).wait_time_min = contended_term; }
            if (*ls).wait_time_max < contended_term { (*ls).wait_time_max = contended_term; }
        }
        SEQ_STATE_RELEASED | SEQ_STATE_ACQUIRED | SEQ_STATE_READ_ACQUIRED => {
            if (*ls).broken == 0 { (*ls).broken = 1; bad_hist[BROKEN_ACQUIRED] += 1; }
            list_del_init(&mut (*seq).list); free(seq as *mut c_void); return 0;
        }
        _ => BUG_ON(cstr(b"Unknown state of lock sequence found!\n\0")),
    }
    (*seq).state = SEQ_STATE_ACQUIRED;
    (*ls).nr_acquired += 1;
    (*ls).avg_wait_time = if (*ls).nr_contended != 0 { (*ls).wait_time_total / (*ls).nr_contended } else { 0 };
    (*seq).prev_event_time = (*sample).time;
    0
}

unsafe extern "C" fn report_lock_contended_event(sample: *mut perf_sample) -> c_int {
    let name = perf_sample__strval(sample, cstr(b"name\0"));
    let addr = perf_sample__intval(sample, cstr(b"lockdep_addr\0"));
    let mut key = 0;
    let ret = get_key_by_aggr_mode_simple(&mut key, addr, (*sample).tid);
    if ret < 0 { return ret; }
    let ls = lock_stat_findnew(key, name, 0);
    if ls.is_null() { return -ENOMEM; }
    let ts = thread_stat_findnew((*sample).tid);
    if ts.is_null() { return -ENOMEM; }
    let seq = get_seq(ts, addr);
    if seq.is_null() { return -ENOMEM; }
    match (*seq).state {
        SEQ_STATE_UNINITIALIZED => return 0,
        SEQ_STATE_ACQUIRING => {}
        SEQ_STATE_RELEASED | SEQ_STATE_ACQUIRED | SEQ_STATE_READ_ACQUIRED | SEQ_STATE_CONTENDED => {
            if (*ls).broken == 0 { (*ls).broken = 1; bad_hist[BROKEN_CONTENDED] += 1; }
            list_del_init(&mut (*seq).list); free(seq as *mut c_void); return 0;
        }
        _ => BUG_ON(cstr(b"Unknown state of lock sequence found!\n\0")),
    }
    (*seq).state = SEQ_STATE_CONTENDED;
    (*ls).nr_contended += 1;
    (*ls).avg_wait_time = (*ls).wait_time_total / (*ls).nr_contended;
    (*seq).prev_event_time = (*sample).time;
    0
}

unsafe extern "C" fn report_lock_release_event(sample: *mut perf_sample) -> c_int {
    let name = perf_sample__strval(sample, cstr(b"name\0"));
    let addr = perf_sample__intval(sample, cstr(b"lockdep_addr\0"));
    let mut key = 0;
    let ret = get_key_by_aggr_mode_simple(&mut key, addr, (*sample).tid);
    if ret < 0 { return ret; }
    let ls = lock_stat_findnew(key, name, 0);
    if ls.is_null() { return -ENOMEM; }
    let ts = thread_stat_findnew((*sample).tid);
    if ts.is_null() { return -ENOMEM; }
    let seq = get_seq(ts, addr);
    if seq.is_null() { return -ENOMEM; }
    match (*seq).state {
        SEQ_STATE_UNINITIALIZED => return 0,
        SEQ_STATE_ACQUIRED => {}
        SEQ_STATE_READ_ACQUIRED => { (*seq).read_count -= 1; BUG_ON((*seq).read_count < 0); if (*seq).read_count != 0 { (*ls).nr_release += 1; return 0; } }
        SEQ_STATE_ACQUIRING | SEQ_STATE_CONTENDED | SEQ_STATE_RELEASED => { if (*ls).broken == 0 { (*ls).broken = 1; bad_hist[BROKEN_RELEASE] += 1; } list_del_init(&mut (*seq).list); free(seq as *mut c_void); return 0; }
        _ => BUG_ON(cstr(b"Unknown state of lock sequence found!\n\0")),
    }
    (*ls).nr_release += 1;
    list_del_init(&mut (*seq).list);
    free(seq as *mut c_void);
    0
}

unsafe fn get_symbol_name_offset(map: *mut map, sym: *mut symbol, ip: u64, buf: *mut c_char, size: c_int) -> c_int {
    if map.is_null() || sym.is_null() { *buf = 0; return 0; }
    let offset = map__map_ip(map, ip) - (*sym).start;
    if offset != 0 { scnprintf(buf, size, cstr(b"%s+%#lx\0"), (*sym).name, offset as c_ulong) } else { strlcpy(buf, (*sym).name, size) }
}

unsafe fn lock_contention_caller(sample: *mut perf_sample, buf: *mut c_char, size: c_int) -> c_int {
    if show_thread_stats { return -1; }
    let machine = &mut (*session).machines.host as *mut machine;
    let thread = machine__findnew_thread(machine, -1, (*sample).pid);
    if thread.is_null() { return -1; }
    let cursor = get_tls_callchain_cursor();
    let ret = thread__resolve_callchain(thread, cursor, sample, ptr::null_mut(), ptr::null_mut(), max_stack_depth);
    if ret != 0 { thread__put(thread); return -1; }
    callchain_cursor_commit(cursor);
    thread__put(thread);
    let mut skip = 0;
    loop {
        let node = callchain_cursor_current(cursor);
        if node.is_null() { break; }
        skip += 1;
        if skip <= stack_skip { callchain_cursor_advance(cursor); continue; }
        let sym = (*node).ms.sym;
        if !sym.is_null() && !machine__is_lock_function(machine, (*node).ip) {
            get_symbol_name_offset((*node).ms.map, sym, (*node).ip, buf, size);
            return 0;
        }
        callchain_cursor_advance(cursor);
    }
    -1
}

unsafe fn callchain_id(sample: *mut perf_sample) -> u64 {
    let machine = &mut (*session).machines.host as *mut machine;
    let thread = machine__findnew_thread(machine, -1, (*sample).pid);
    if thread.is_null() { return -1i64 as u64; }
    let cursor = get_tls_callchain_cursor();
    let ret = thread__resolve_callchain(thread, cursor, sample, ptr::null_mut(), ptr::null_mut(), max_stack_depth);
    thread__put(thread);
    if ret != 0 { return -1i64 as u64; }
    callchain_cursor_commit(cursor);
    let mut hash = 0u64;
    let mut skip = 0;
    loop {
        let node = callchain_cursor_current(cursor);
        if node.is_null() { break; }
        skip += 1;
        if skip <= stack_skip { callchain_cursor_advance(cursor); continue; }
        if !(*node).ms.sym.is_null() && machine__is_lock_function(machine, (*node).ip) { callchain_cursor_advance(cursor); continue; }
        hash ^= hash_long((*node).ip as c_ulong, 64) as u64;
        callchain_cursor_advance(cursor);
    }
    hash
}

unsafe fn get_callstack(sample: *mut perf_sample, max_stack: c_int) -> *mut u64 {
    if (*sample).callchain.is_null() { pr_debug(cstr(b"Sample unexpectedly missing callchain\n\0")); return ptr::null_mut(); }
    let callstack = calloc(max_stack as usize, size_of::<u64>()) as *mut u64;
    if callstack.is_null() { pr_debug(cstr(b"Failed to allocate callstack\n\0")); return ptr::null_mut(); }
    let mut i = 0u64;
    let mut c = 0;
    while i < (*(*sample).callchain).nr && c < max_stack {
        let ip = *(*(*sample).callchain).ips.as_ptr().add(i as usize);
        if ip < PERF_CONTEXT_MAX { *callstack.add(c as usize) = ip; c += 1; }
        i += 1;
    }
    callstack
}

unsafe extern "C" fn report_lock_contention_begin_event(sample: *mut perf_sample) -> c_int {
    static mut kmap_loaded: bool_ = false;
    let addr = perf_sample__intval(sample, cstr(b"lock_addr\0"));
    let flags = perf_sample__intval(sample, cstr(b"flags\0")) as c_uint;
    let machine = &mut (*session).machines.host as *mut machine;
    let mut kmap: *mut map = ptr::null_mut();
    let mut sym: *mut symbol;
    let mut key = 0;
    let mut ret = get_key_by_aggr_mode(&mut key, addr, sample);
    if ret < 0 { return ret; }
    if !kmap_loaded {
        map__load(machine__kernel_map(machine));
        kmap_loaded = true;
        let mut i = 0;
        while i < filters.nr_syms {
            sym = machine__find_kernel_symbol_by_name(machine, *filters.syms.add(i as usize), &mut kmap);
            if sym.is_null() { pr_warning(cstr(b"ignore unknown symbol: %s\n\0"), *filters.syms.add(i as usize)); i += 1; continue; }
            let addrs = realloc(filters.addrs as *mut c_void, ((filters.nr_addrs + 1) as usize) * size_of::<c_ulong>()) as *mut c_ulong;
            if addrs.is_null() { pr_warning(cstr(b"memory allocation failure\n\0")); return -ENOMEM; }
            *addrs.add(filters.nr_addrs as usize) = map__unmap_ip(kmap, (*sym).start) as c_ulong;
            filters.nr_addrs += 1; filters.addrs = addrs; i += 1;
        }
    }
    let mut ls = lock_stat_find(key);
    if ls.is_null() {
        let mut buf = [0 as c_char; 128];
        let mut name = cstr(b"\0");
        match aggr_mode {
            LOCK_AGGR_ADDR => { sym = machine__find_kernel_symbol(machine, key, &mut kmap); if !sym.is_null() { name = (*sym).name; } }
            LOCK_AGGR_CALLER => { name = buf.as_mut_ptr(); if lock_contention_caller(sample, buf.as_mut_ptr(), size_of::<[c_char;128]>() as c_int) < 0 { name = cstr(b"Unknown\0"); } }
            _ => {}
        }
        ls = lock_stat_findnew(key, name, flags);
        if ls.is_null() { return -ENOMEM; }
    }
    if filters.nr_types != 0 {
        let mut found = false;
        let mut i = 0; while i < filters.nr_types { if flags == *filters.types.add(i as usize) { found = true; break; } i += 1; }
        if !found { return 0; }
    }
    if filters.nr_addrs != 0 {
        let mut found = false;
        let mut i = 0; while i < filters.nr_addrs { if addr == *filters.addrs.add(i as usize) as u64 { found = true; break; } i += 1; }
        if !found { return 0; }
    }
    if needs_callstack() {
        let callstack = get_callstack(sample, max_stack_depth);
        if callstack.is_null() { return 0; }
        if !match_callstack_filter(machine, callstack, max_stack_depth) { free(callstack as *mut c_void); return 0; }
        if (*ls).callstack.is_null() { (*ls).callstack = callstack; } else { free(callstack as *mut c_void); }
    }
    let ts = thread_stat_findnew((*sample).tid);
    if ts.is_null() { return -ENOMEM; }
    let seq = get_seq(ts, addr);
    if seq.is_null() { return -ENOMEM; }
    match (*seq).state {
        SEQ_STATE_UNINITIALIZED | SEQ_STATE_ACQUIRED => {}
        SEQ_STATE_CONTENDED => return 0,
        SEQ_STATE_ACQUIRING | SEQ_STATE_READ_ACQUIRED | SEQ_STATE_RELEASED => {
            if (*ls).broken == 0 { (*ls).broken = 1; bad_hist[BROKEN_CONTENDED] += 1; }
            list_del_init(&mut (*seq).list); free(seq as *mut c_void); return 0;
        }
        _ => BUG_ON(cstr(b"Unknown state of lock sequence found!\n\0")),
    }
    if (*seq).state != SEQ_STATE_CONTENDED { (*seq).state = SEQ_STATE_CONTENDED; (*seq).prev_event_time = (*sample).time; (*ls).nr_contended += 1; }
    0
}

unsafe extern "C" fn report_lock_contention_end_event(sample: *mut perf_sample) -> c_int {
    let addr = perf_sample__intval(sample, cstr(b"lock_addr\0"));
    let mut key = 0;
    let ret = get_key_by_aggr_mode(&mut key, addr, sample);
    if ret < 0 { return ret; }
    let ls = lock_stat_find(key);
    if ls.is_null() { return 0; }
    let ts = thread_stat_find((*sample).tid);
    if ts.is_null() { return 0; }
    let seq = get_seq(ts, addr);
    if seq.is_null() { return -ENOMEM; }
    match (*seq).state {
        SEQ_STATE_UNINITIALIZED => return 0,
        SEQ_STATE_CONTENDED => {
            let contended_term = (*sample).time - (*seq).prev_event_time;
            (*ls).wait_time_total += contended_term;
            if contended_term < (*ls).wait_time_min { (*ls).wait_time_min = contended_term; }
            if (*ls).wait_time_max < contended_term { (*ls).wait_time_max = contended_term; }
        }
        SEQ_STATE_ACQUIRING | SEQ_STATE_ACQUIRED | SEQ_STATE_READ_ACQUIRED | SEQ_STATE_RELEASED => {
            if (*ls).broken == 0 { (*ls).broken = 1; bad_hist[BROKEN_ACQUIRED] += 1; }
            list_del_init(&mut (*seq).list); free(seq as *mut c_void); return 0;
        }
        _ => BUG_ON(cstr(b"Unknown state of lock sequence found!\n\0")),
    }
    (*seq).state = SEQ_STATE_ACQUIRED;
    (*ls).nr_acquired += 1;
    (*ls).avg_wait_time = (*ls).wait_time_total / (*ls).nr_acquired;
    0
}

static report_lock_ops: trace_lock_handler = trace_lock_handler { acquire_event: Some(report_lock_acquire_event), acquired_event: Some(report_lock_acquired_event), contended_event: Some(report_lock_contended_event), release_event: Some(report_lock_release_event), contention_begin_event: Some(report_lock_contention_begin_event), contention_end_event: Some(report_lock_contention_end_event) };
static contention_lock_ops: trace_lock_handler = trace_lock_handler { acquire_event: None, acquired_event: None, contended_event: None, release_event: None, contention_begin_event: Some(report_lock_contention_begin_event), contention_end_event: Some(report_lock_contention_end_event) };
static mut trace_handler: *const trace_lock_handler = ptr::null();

macro_rules! process_lock_event {
    ($name:ident, $field:ident) => {
        unsafe extern "C" fn $name(sample: *mut perf_sample) -> c_int {
            if !trace_handler.is_null() {
                if let Some(f) = (*trace_handler).$field { return f(sample); }
            }
            0
        }
    };
}
process_lock_event!(evsel__process_lock_acquire, acquire_event);
process_lock_event!(evsel__process_lock_acquired, acquired_event);
process_lock_event!(evsel__process_lock_contended, contended_event);
process_lock_event!(evsel__process_lock_release, release_event);
process_lock_event!(evsel__process_contention_begin, contention_begin_event);
process_lock_event!(evsel__process_contention_end, contention_end_event);

unsafe fn print_bad_events(bad: c_int, total: c_int) {
    let mut broken = 0;
    let name = [cstr(b"acquire\0"), cstr(b"acquired\0"), cstr(b"contended\0"), cstr(b"release\0")];
    for i in 0..BROKEN_MAX { broken += bad_hist[i]; }
    if quiet || total == 0 || (broken == 0 && verbose <= 0) { return; }
    fprintf(lock_output, cstr(b"\n=== output for debug ===\n\n\0"));
    fprintf(lock_output, cstr(b"bad: %d, total: %d\n\0"), bad, total);
    fprintf(lock_output, cstr(b"bad rate: %.2f %%\n\0"), (bad as c_double) / (total as c_double) * 100.0);
    fprintf(lock_output, cstr(b"histogram of events caused bad sequence\n\0"));
    for i in 0..BROKEN_MAX { fprintf(lock_output, cstr(b" %10s: %d\n\0"), name[i], bad_hist[i]); }
}

unsafe fn for_each_lock_key(mut f: impl FnMut(*mut lock_key)) {
    let mut pos = lock_keys.next;
    while pos != &mut lock_keys && !pos.is_null() {
        f(container_of::<lock_key>(pos as *mut c_void, 5 * size_of::<usize>()));
        pos = (*pos).next;
    }
}

unsafe fn print_result() {
    if !quiet {
        fprintf(lock_output, cstr(b"%20s \0"), cstr(b"Name\0"));
        for_each_lock_key(|key| unsafe { fprintf(lock_output, cstr(b"%*s \0"), (*key).len, (*key).header); });
        fprintf(lock_output, cstr(b"\n\n\0"));
    }
    let mut bad = 0;
    let mut total = 0;
    let mut printed = 0;
    loop {
        let st = pop_from_result();
        if st.is_null() { break; }
        total += 1;
        if (*st).broken != 0 { bad += 1; }
        if (*st).nr_acquired == 0 { continue; }
        let mut cut_name = [0 as c_char; 20];
        bzero(cut_name.as_mut_ptr() as *mut c_void, 20);
        if strlen((*st).name) < 20 {
            let mut name = (*st).name;
            if show_thread_stats {
                let t = perf_session__findnew(session, (*st).addr as c_int);
                name = thread__comm_str(t);
            }
            fprintf(lock_output, cstr(b"%20s \0"), name);
        } else {
            strncpy(cut_name.as_mut_ptr(), (*st).name, 16);
            cut_name[16] = b'.' as c_char; cut_name[17] = b'.' as c_char; cut_name[18] = b'.' as c_char; cut_name[19] = 0;
            fprintf(lock_output, cstr(b"%20s \0"), cut_name.as_ptr());
        }
        for_each_lock_key(|key| unsafe { (*key).print.unwrap()(key, st); fprintf(lock_output, cstr(b" \0")); });
        fprintf(lock_output, cstr(b"\n\0"));
        printed += 1;
        if printed >= print_nr_entries { break; }
    }
    print_bad_events(bad, total);
}

static mut info_threads: bool_ = false;
static mut info_map: bool_ = false;

unsafe fn dump_threads() {
    fprintf(lock_output, cstr(b"%10s: comm\n\0"), cstr(b"Thread ID\0"));
    let mut node = rb_first(&thread_stats);
    while !node.is_null() {
        let st = container_of::<thread_stat>(node as *mut c_void, 0);
        let t = perf_session__findnew(session, (*st).tid as c_int);
        fprintf(lock_output, cstr(b"%10d: %s\n\0"), (*st).tid, thread__comm_str(t));
        node = rb_next(node);
        thread__put(t);
    }
}

unsafe extern "C" fn compare_maps(a: *mut lock_stat, b: *mut lock_stat) -> c_int {
    let ret = if !(*a).name.is_null() && !(*b).name.is_null() { strcmp((*a).name, (*b).name) } else { ((!(*a).name.is_null()) as c_int) - ((!(*b).name.is_null()) as c_int) };
    if ret == 0 { ((*a).addr < (*b).addr) as c_int } else { (ret < 0) as c_int }
}

unsafe fn dump_map() {
    fprintf(lock_output, cstr(b"Address of instance: name of class\n\0"));
    for i in 0..LOCKHASH_SIZE {
        let mut hn = (*lockhash_table.add(i)).first;
        while !hn.is_null() {
            let st = container_of::<lock_stat>(hn as *mut c_void, size_of::<rb_node>());
            insert_to_result(st, Some(compare_maps));
            hn = (*hn).next;
        }
    }
    loop {
        let st = pop_from_result();
        if st.is_null() { break; }
        fprintf(lock_output, cstr(b" %#llx: %s\n\0"), (*st).addr as c_ulonglong, (*st).name);
    }
}

unsafe fn dump_info() {
    if info_threads { dump_threads(); }
    if info_map { if info_threads { fputc('\n' as c_int, lock_output); } dump_map(); }
}

static lock_tracepoints: [evsel_str_handler; 4] = [
    evsel_str_handler { name: cstr(b"lock:lock_acquire\0"), handler: Some(evsel__process_lock_acquire) },
    evsel_str_handler { name: cstr(b"lock:lock_acquired\0"), handler: Some(evsel__process_lock_acquired) },
    evsel_str_handler { name: cstr(b"lock:lock_contended\0"), handler: Some(evsel__process_lock_contended) },
    evsel_str_handler { name: cstr(b"lock:lock_release\0"), handler: Some(evsel__process_lock_release) },
];
static contention_tracepoints: [evsel_str_handler; 2] = [
    evsel_str_handler { name: cstr(b"lock:contention_begin\0"), handler: Some(evsel__process_contention_begin) },
    evsel_str_handler { name: cstr(b"lock:contention_end\0"), handler: Some(evsel__process_contention_end) },
];

unsafe extern "C" fn process_event_update(tool: *const perf_tool, event: *mut perf_event, pevlist: *mut *mut evlist) -> c_int {
    let ret = perf_event__process_event_update(tool, event, pevlist);
    if ret < 0 { return ret; }
    perf_session__set_tracepoints_handlers(session, lock_tracepoints.as_ptr());
    perf_session__set_tracepoints_handlers(session, contention_tracepoints.as_ptr());
    0
}

type tracepoint_handler = unsafe extern "C" fn(*mut perf_sample) -> c_int;

unsafe extern "C" fn process_sample_event(_tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let evsel = (*sample).evsel;
    let mut err = 0;
    let thread = machine__findnew_thread(machine, (*sample).pid, (*sample).tid as c_int);
    if thread.is_null() {
        pr_debug(cstr(b"problem processing %s (%u) event at offset %#llx, skipping it.\n\0"), perf_event__name((*event).header.type_), (*event).header.type_, (*sample).file_offset);
        return -1;
    }
    if !(*evsel).handler.is_none() { err = (*evsel).handler.unwrap()(sample); }
    thread__put(thread);
    err
}

unsafe fn combine_result() {
    if !combine_locks { return; }
    for i in 0..LOCKHASH_SIZE {
        let mut hn = (*lockhash_table.add(i)).first;
        while !hn.is_null() {
            combine_lock_stats(container_of::<lock_stat>(hn as *mut c_void, size_of::<rb_node>()));
            hn = (*hn).next;
        }
    }
}

unsafe fn sort_result() {
    for i in 0..LOCKHASH_SIZE {
        let mut hn = (*lockhash_table.add(i)).first;
        while !hn.is_null() {
            insert_to_result(container_of::<lock_stat>(hn as *mut c_void, size_of::<rb_node>()), compare);
            hn = (*hn).next;
        }
    }
}

#[repr(C)] struct lock_type_entry { flags: c_uint, flags_name: *const c_char, lock_name: *const c_char }
static lock_type_table: [lock_type_entry; 14] = [
    lock_type_entry { flags: 0, flags_name: cstr(b"semaphore\0"), lock_name: cstr(b"semaphore\0") },
    lock_type_entry { flags: LCB_F_SPIN, flags_name: cstr(b"spinlock\0"), lock_name: cstr(b"spinlock\0") },
    lock_type_entry { flags: LCB_F_SPIN | LCB_F_READ, flags_name: cstr(b"rwlock:R\0"), lock_name: cstr(b"rwlock\0") },
    lock_type_entry { flags: LCB_F_SPIN | LCB_F_WRITE, flags_name: cstr(b"rwlock:W\0"), lock_name: cstr(b"rwlock\0") },
    lock_type_entry { flags: LCB_F_READ, flags_name: cstr(b"rwsem:R\0"), lock_name: cstr(b"rwsem\0") },
    lock_type_entry { flags: LCB_F_WRITE, flags_name: cstr(b"rwsem:W\0"), lock_name: cstr(b"rwsem\0") },
    lock_type_entry { flags: LCB_F_RT, flags_name: cstr(b"rt-mutex\0"), lock_name: cstr(b"rt-mutex\0") },
    lock_type_entry { flags: LCB_F_RT | LCB_F_READ, flags_name: cstr(b"rwlock-rt:R\0"), lock_name: cstr(b"rwlock-rt\0") },
    lock_type_entry { flags: LCB_F_RT | LCB_F_WRITE, flags_name: cstr(b"rwlock-rt:W\0"), lock_name: cstr(b"rwlock-rt\0") },
    lock_type_entry { flags: LCB_F_PERCPU | LCB_F_READ, flags_name: cstr(b"pcpu-sem:R\0"), lock_name: cstr(b"percpu-rwsem\0") },
    lock_type_entry { flags: LCB_F_PERCPU | LCB_F_WRITE, flags_name: cstr(b"pcpu-sem:W\0"), lock_name: cstr(b"percpu-rwsem\0") },
    lock_type_entry { flags: LCB_F_MUTEX, flags_name: cstr(b"mutex\0"), lock_name: cstr(b"mutex\0") },
    lock_type_entry { flags: LCB_F_MUTEX | LCB_F_SPIN, flags_name: cstr(b"mutex\0"), lock_name: cstr(b"mutex\0") },
    lock_type_entry { flags: LCB_F_MUTEX | LCB_F_SPIN, flags_name: cstr(b"mutex:spin\0"), lock_name: cstr(b"mutex-spin\0") },
];

unsafe fn get_type_flags_name(mut flags: c_uint) -> *const c_char {
    flags &= LCB_F_TYPE_MASK;
    for e in lock_type_table.iter() { if e.flags == flags { return e.flags_name; } }
    cstr(b"unknown\0")
}
unsafe fn get_type_lock_name(mut flags: c_uint) -> *const c_char {
    flags &= LCB_F_TYPE_MASK;
    for e in lock_type_table.iter() { if e.flags == flags { return e.lock_name; } }
    cstr(b"unknown\0")
}

unsafe fn lock_filter_finish() {
    zfree(&mut filters.types as *mut _ as *mut *mut c_void); filters.nr_types = 0;
    zfree(&mut filters.addrs as *mut _ as *mut *mut c_void); filters.nr_addrs = 0;
    for i in 0..filters.nr_syms { free(*filters.syms.add(i as usize) as *mut c_void); }
    zfree(&mut filters.syms as *mut _ as *mut *mut c_void); filters.nr_syms = 0;
    zfree(&mut filters.cgrps as *mut _ as *mut *mut c_void); filters.nr_cgrps = 0;
    for i in 0..filters.nr_slabs { free(*filters.slabs.add(i as usize) as *mut c_void); }
    zfree(&mut filters.slabs as *mut _ as *mut *mut c_void); filters.nr_slabs = 0;
}

unsafe fn sort_contention_result() { sort_result(); }

unsafe fn print_header_stdio() {
    for_each_lock_key(|key| unsafe { fprintf(lock_output, cstr(b"%*s \0"), (*key).len, (*key).header); });
    match aggr_mode {
        LOCK_AGGR_TASK => fprintf(lock_output, cstr(b"  %10s   %s\n\n\0"), cstr(b"pid\0"), if show_lock_owner { cstr(b"owner\0") } else { cstr(b"comm\0") }),
        LOCK_AGGR_CALLER => fprintf(lock_output, cstr(b"  %10s   %s\n\n\0"), cstr(b"type\0"), cstr(b"caller\0")),
        LOCK_AGGR_ADDR => fprintf(lock_output, cstr(b"  %16s   %s\n\n\0"), cstr(b"address\0"), cstr(b"symbol\0")),
        LOCK_AGGR_CGROUP => fprintf(lock_output, cstr(b"  %s\n\n\0"), cstr(b"cgroup\0")),
        _ => 0,
    };
}

unsafe fn print_header_csv(sep: *const c_char) {
    fprintf(lock_output, cstr(b"# output: \0"));
    for_each_lock_key(|key| unsafe { fprintf(lock_output, cstr(b"%s%s \0"), (*key).header, sep); });
    match aggr_mode {
        LOCK_AGGR_TASK => { fprintf(lock_output, cstr(b"%s%s %s\n\0"), cstr(b"pid\0"), sep, if show_lock_owner { cstr(b"owner\0") } else { cstr(b"comm\0") }); }
        LOCK_AGGR_CALLER => { fprintf(lock_output, cstr(b"%s%s %s\0"), cstr(b"type\0"), sep, cstr(b"caller\0")); if verbose > 0 { fprintf(lock_output, cstr(b"%s %s\0"), sep, cstr(b"stacktrace\0")); } fprintf(lock_output, cstr(b"\n\0")); }
        LOCK_AGGR_ADDR => { fprintf(lock_output, cstr(b"%s%s %s%s %s\n\0"), cstr(b"address\0"), sep, cstr(b"symbol\0"), sep, cstr(b"type\0")); }
        LOCK_AGGR_CGROUP => { fprintf(lock_output, cstr(b"%s\n\0"), cstr(b"cgroup\0")); }
        _ => {}
    }
}
unsafe fn print_header() { if !quiet { if !symbol_conf.field_sep.is_null() { print_header_csv(symbol_conf.field_sep); } else { print_header_stdio(); } } }

unsafe fn print_lock_stat_stdio(con: *mut lock_contention, st: *mut lock_stat) {
    for_each_lock_key(|key| unsafe { (*key).print.unwrap()(key, st); fprintf(lock_output, cstr(b" \0")); });
    match aggr_mode {
        LOCK_AGGR_CALLER => { fprintf(lock_output, cstr(b"  %10s   %s\n\0"), get_type_flags_name((*st).flags), (*st).name); }
        LOCK_AGGR_TASK => { let pid = (*st).addr as c_int; let t = perf_session__findnew(session, pid); fprintf(lock_output, cstr(b"  %10d   %s\n\0"), pid, if pid == -1 { cstr(b"Unknown\0") } else { thread__comm_str(t) }); }
        LOCK_AGGR_ADDR => { fprintf(lock_output, cstr(b"  %016llx   %s (%s)\n\0"), (*st).addr as c_ulonglong, (*st).name, get_type_lock_name((*st).flags)); }
        LOCK_AGGR_CGROUP => { fprintf(lock_output, cstr(b"  %s\n\0"), (*st).name); }
        _ => {}
    }
    if aggr_mode == LOCK_AGGR_CALLER && verbose > 0 {
        let mut kmap: *mut map = ptr::null_mut();
        let mut buf = [0 as c_char; 128];
        for i in 0..max_stack_depth {
            if (*st).callstack.is_null() || *(*st).callstack.add(i as usize) == 0 { break; }
            let ip = *(*st).callstack.add(i as usize);
            let sym = machine__find_kernel_symbol((*con).machine, ip, &mut kmap);
            get_symbol_name_offset(kmap, sym, ip, buf.as_mut_ptr(), size_of::<[c_char;128]>() as c_int);
            fprintf(lock_output, cstr(b"\t\t\t%#lx  %s\n\0"), ip as c_ulong, buf.as_ptr());
        }
    }
}

unsafe fn print_lock_stat_csv(con: *mut lock_contention, st: *mut lock_stat, sep: *const c_char) {
    for_each_lock_key(|key| unsafe { (*key).print.unwrap()(key, st); fprintf(lock_output, cstr(b"%s \0"), sep); });
    match aggr_mode {
        LOCK_AGGR_CALLER => { fprintf(lock_output, cstr(b"%s%s %s\0"), get_type_flags_name((*st).flags), sep, (*st).name); if verbose <= 0 { fprintf(lock_output, cstr(b"\n\0")); } }
        LOCK_AGGR_TASK => { let pid = (*st).addr as c_int; let t = perf_session__findnew(session, pid); fprintf(lock_output, cstr(b"%d%s %s\n\0"), pid, sep, if pid == -1 { cstr(b"Unknown\0") } else { thread__comm_str(t) }); }
        LOCK_AGGR_ADDR => { fprintf(lock_output, cstr(b"%llx%s %s%s %s\n\0"), (*st).addr as c_ulonglong, sep, (*st).name, sep, get_type_lock_name((*st).flags)); }
        LOCK_AGGR_CGROUP => { fprintf(lock_output, cstr(b"%s\n\0"), (*st).name); }
        _ => {}
    }
    if aggr_mode == LOCK_AGGR_CALLER && verbose > 0 {
        let mut kmap: *mut map = ptr::null_mut();
        let mut buf = [0 as c_char; 128];
        for i in 0..max_stack_depth {
            if (*st).callstack.is_null() || *(*st).callstack.add(i as usize) == 0 { break; }
            let ip = *(*st).callstack.add(i as usize);
            let sym = machine__find_kernel_symbol((*con).machine, ip, &mut kmap);
            get_symbol_name_offset(kmap, sym, ip, buf.as_mut_ptr(), size_of::<[c_char;128]>() as c_int);
            fprintf(lock_output, cstr(b"%s %#lx %s\0"), if i != 0 { cstr(b":\0") } else { sep }, ip as c_ulong, buf.as_ptr());
        }
        fprintf(lock_output, cstr(b"\n\0"));
    }
}

unsafe fn print_lock_stat(con: *mut lock_contention, st: *mut lock_stat) {
    if !symbol_conf.field_sep.is_null() { print_lock_stat_csv(con, st, symbol_conf.field_sep); } else { print_lock_stat_stdio(con, st); }
}

unsafe fn print_footer_stdio(mut total: c_int, bad: c_int, fails: *mut lock_contention_fails) {
    let broken = (*fails).task + (*fails).stack + (*fails).time + (*fails).data;
    if !use_bpf { print_bad_events(bad, total); }
    if quiet || total == 0 || (broken == 0 && verbose <= 0) { return; }
    total += broken;
    fprintf(lock_output, cstr(b"\n=== output for debug ===\n\n\0"));
    fprintf(lock_output, cstr(b"bad: %d, total: %d\n\0"), broken, total);
    fprintf(lock_output, cstr(b"bad rate: %.2f %%\n\0"), 100.0 * (broken as c_double) / (total as c_double));
    fprintf(lock_output, cstr(b"histogram of failure reasons\n\0"));
    fprintf(lock_output, cstr(b" %10s: %d\n\0"), cstr(b"task\0"), (*fails).task);
    fprintf(lock_output, cstr(b" %10s: %d\n\0"), cstr(b"stack\0"), (*fails).stack);
    fprintf(lock_output, cstr(b" %10s: %d\n\0"), cstr(b"time\0"), (*fails).time);
    fprintf(lock_output, cstr(b" %10s: %d\n\0"), cstr(b"data\0"), (*fails).data);
}

unsafe fn print_footer_csv(mut total: c_int, mut bad: c_int, fails: *mut lock_contention_fails, sep: *const c_char) {
    if use_bpf { bad = (*fails).task + (*fails).stack + (*fails).time + (*fails).data; }
    if quiet || total == 0 || (bad == 0 && verbose <= 0) { return; }
    total += bad;
    fprintf(lock_output, cstr(b"# debug: total=%d%s bad=%d\0"), total, sep, bad);
    if use_bpf {
        fprintf(lock_output, cstr(b"%s bad_%s=%d\0"), sep, cstr(b"task\0"), (*fails).task);
        fprintf(lock_output, cstr(b"%s bad_%s=%d\0"), sep, cstr(b"stack\0"), (*fails).stack);
        fprintf(lock_output, cstr(b"%s bad_%s=%d\0"), sep, cstr(b"time\0"), (*fails).time);
        fprintf(lock_output, cstr(b"%s bad_%s=%d\0"), sep, cstr(b"data\0"), (*fails).data);
    } else {
        let name = [cstr(b"acquire\0"), cstr(b"acquired\0"), cstr(b"contended\0"), cstr(b"release\0")];
        for i in 0..BROKEN_MAX { fprintf(lock_output, cstr(b"%s bad_%s=%d\0"), sep, name[i], bad_hist[i]); }
    }
    fprintf(lock_output, cstr(b"\n\0"));
}
unsafe fn print_footer(total: c_int, bad: c_int, fails: *mut lock_contention_fails) {
    if !symbol_conf.field_sep.is_null() { print_footer_csv(total, bad, fails, symbol_conf.field_sep); } else { print_footer_stdio(total, bad, fails); }
}

unsafe fn print_contention_result(con: *mut lock_contention) {
    if !quiet { print_header(); }
    let mut bad = 0;
    let mut total = 0;
    let mut printed = 0;
    loop {
        let st = pop_from_result();
        if st.is_null() { break; }
        total += if use_bpf { (*st).nr_contended as c_int } else { 1 };
        if (*st).broken != 0 { bad += 1; }
        if (*st).wait_time_total == 0 { continue; }
        print_lock_stat(con, st);
        printed += 1;
        if printed >= print_nr_entries { break; }
    }
    if (*con).owner && (*con).save_callstack && verbose > 0 {
        let mut root = rb_root { rb_node: ptr::null_mut() };
        if !symbol_conf.field_sep.is_null() { fprintf(lock_output, cstr(b"# owner stack trace:\n\0")); } else { fprintf(lock_output, cstr(b"\n=== owner stack trace ===\n\n\0")); }
        loop { let st = pop_owner_stack_trace(con); if st.is_null() { break; } insert_to(&mut root, st, compare); }
        loop { let st = pop_from(&mut root); if st.is_null() { break; } print_lock_stat(con, st); free(st as *mut c_void); }
    }
    if print_nr_entries != 0 {
        loop {
            let st = pop_from_result();
            if st.is_null() { break; }
            total += if use_bpf { (*st).nr_contended as c_int } else { 1 };
            if (*st).broken != 0 { bad += 1; }
        }
    }
    total += (*con).nr_filtered;
    print_footer(total, bad, &mut (*con).fails);
}

static mut force: bool_ = false;

unsafe fn __cmd_report(display_info: bool_) -> c_int {
    let mut err = -EINVAL;
    let mut eops: perf_tool = core::mem::zeroed();
    let mut data = perf_data { path: input_name, mode: PERF_DATA_MODE_READ, force, is_pipe: false };
    perf_tool__init(&mut eops, true);
    eops.attr = perf_event__process_attr;
    eops.event_update = process_event_update;
    eops.sample = process_sample_event;
    eops.comm = perf_event__process_comm; eops.mmap = perf_event__process_mmap; eops.mmap2 = perf_event__process_mmap2; eops.namespaces = perf_event__process_namespaces; eops.tracing_data = perf_event__process_tracing_data;
    session = perf_session__new(&mut data, &mut eops);
    if IS_ERR(session) { pr_err(cstr(b"Initializing perf session failed\n\0")); return PTR_ERR(session); }
    symbol_conf.allow_aliases = true;
    symbol__init(perf_session__env(session));
    if !data.is_pipe {
        if !perf_session__has_traces(session, cstr(b"lock record\0")) { goto_out_delete_report(err); return err; }
        if perf_session__set_tracepoints_handlers(session, lock_tracepoints.as_ptr()) != 0 { pr_err(cstr(b"Initializing perf session tracepoint handlers failed\n\0")); goto_out_delete_report(err); return err; }
        if perf_session__set_tracepoints_handlers(session, contention_tracepoints.as_ptr()) != 0 { pr_err(cstr(b"Initializing perf session tracepoint handlers failed\n\0")); goto_out_delete_report(err); return err; }
    }
    if setup_output_field(false, output_fields) != 0 { perf_session__delete(session); return err; }
    if select_key(false) != 0 { perf_session__delete(session); return err; }
    if show_thread_stats { aggr_mode = LOCK_AGGR_TASK; }
    err = perf_session__process_events(session);
    if err == 0 {
        setup_pager();
        if display_info { dump_info(); } else { combine_result(); sort_result(); print_result(); }
    }
    perf_session__delete(session);
    err
}
unsafe fn goto_out_delete_report(_err: c_int) { perf_session__delete(session); }

static mut done: sig_atomic_t = 0;
unsafe extern "C" fn sighandler(_sig: c_int) { done = 1; }

unsafe fn check_lock_contention_options(options: *const option, usage: *const *const c_char) -> c_int {
    if show_thread_stats && show_lock_addrs { pr_err(cstr(b"Cannot use thread and addr mode together\n\0")); parse_options_usage(usage, options, cstr(b"threads\0"), 0); parse_options_usage(ptr::null(), options, cstr(b"lock-addr\0"), 0); return -1; }
    if show_lock_owner && !use_bpf { pr_err(cstr(b"Lock owners are available only with BPF\n\0")); parse_options_usage(usage, options, cstr(b"lock-owner\0"), 0); parse_options_usage(ptr::null(), options, cstr(b"use-bpf\0"), 0); return -1; }
    if show_lock_owner && show_lock_addrs { pr_err(cstr(b"Cannot use owner and addr mode together\n\0")); parse_options_usage(usage, options, cstr(b"lock-owner\0"), 0); parse_options_usage(ptr::null(), options, cstr(b"lock-addr\0"), 0); return -1; }
    if show_lock_cgroups && !use_bpf { pr_err(cstr(b"Cgroups are available only with BPF\n\0")); parse_options_usage(usage, options, cstr(b"lock-cgroup\0"), 0); parse_options_usage(ptr::null(), options, cstr(b"use-bpf\0"), 0); return -1; }
    if show_lock_cgroups && show_lock_addrs { pr_err(cstr(b"Cannot use cgroup and addr mode together\n\0")); parse_options_usage(usage, options, cstr(b"lock-cgroup\0"), 0); parse_options_usage(ptr::null(), options, cstr(b"lock-addr\0"), 0); return -1; }
    if show_lock_cgroups && show_thread_stats { pr_err(cstr(b"Cannot use cgroup and thread mode together\n\0")); parse_options_usage(usage, options, cstr(b"lock-cgroup\0"), 0); parse_options_usage(ptr::null(), options, cstr(b"threads\0"), 0); return -1; }
    if !symbol_conf.field_sep.is_null() {
        if !strstr(symbol_conf.field_sep, cstr(b":\0")).is_null() || !strstr(symbol_conf.field_sep, cstr(b"+\0")).is_null() || !strstr(symbol_conf.field_sep, cstr(b".\0")).is_null() {
            pr_err(cstr(b"Cannot use the separator that is already used\n\0")); parse_options_usage(usage, options, cstr(b"x\0"), 1); return -1;
        }
    }
    if show_lock_owner && !show_thread_stats { pr_warning(cstr(b"Now -o try to show owner's callstack instead of pid and comm.\n\0")); pr_warning(cstr(b"Please use -t option too to keep the old behavior.\n\0")); }
    0
}

unsafe fn __cmd_contention(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut err = -EINVAL;
    let mut eops: perf_tool = core::mem::zeroed();
    let mut data = perf_data { path: input_name, mode: PERF_DATA_MODE_READ, force, is_pipe: false };
    let mut con = lock_contention { target: &mut target, map_nr_entries: bpf_map_entries, max_stack: max_stack_depth, stack_skip, filters: &mut filters, delays, nr_delays, save_callstack: needs_callstack(), owner: show_lock_owner, cgroups: rb_root { rb_node: ptr::null_mut() }, result: ptr::null_mut(), machine: ptr::null_mut(), aggr_mode: LOCK_AGGR_ADDR, evlist: ptr::null_mut(), nr_filtered: 0, fails: lock_contention_fails { task: 0, stack: 0, time: 0, data: 0 } };
    let mut host_env: perf_env = core::mem::zeroed();
    lockhash_table = calloc(LOCKHASH_SIZE, size_of::<hlist_head>()) as *mut hlist_head;
    if lockhash_table.is_null() { return -ENOMEM; }
    con.result = lockhash_table;
    perf_tool__init(&mut eops, true);
    eops.attr = perf_event__process_attr; eops.event_update = process_event_update; eops.sample = process_sample_event; eops.comm = perf_event__process_comm; eops.mmap = perf_event__process_mmap; eops.mmap2 = perf_event__process_mmap2; eops.tracing_data = perf_event__process_tracing_data;
    perf_env__init(&mut host_env);
    session = __perf_session__new(if use_bpf { ptr::null_mut() } else { &mut data }, &mut eops, false, &mut host_env);
    if IS_ERR(session) { pr_err(cstr(b"Initializing perf session failed\n\0")); err = PTR_ERR(session); session = ptr::null_mut(); }
    else {
        con.machine = &mut (*session).machines.host;
        con.aggr_mode = if show_thread_stats { LOCK_AGGR_TASK } else if show_lock_addrs { LOCK_AGGR_ADDR } else if show_lock_cgroups { LOCK_AGGR_CGROUP } else { LOCK_AGGR_CALLER };
        aggr_mode = con.aggr_mode;
        if con.aggr_mode == LOCK_AGGR_CALLER { con.save_callstack = true; }
        symbol_conf.allow_aliases = true; symbol__init(perf_session__env(session));
        if use_bpf {
            err = target__validate(&mut target);
            if err == 0 {
                done = 0; signal(SIGINT, Some(sighandler)); signal(SIGCHLD, Some(sighandler)); signal(SIGTERM, Some(sighandler));
                con.evlist = evlist__new();
                if con.evlist.is_null() { err = -ENOMEM; } else {
                    err = evlist__create_maps(con.evlist, &mut target);
                    if err >= 0 && argc != 0 { err = evlist__prepare_workload(con.evlist, &mut target, argv, false, ptr::null_mut()); }
                    if err >= 0 { err = lock_contention_prepare(&mut con); if err < 0 { pr_err(cstr(b"lock contention BPF setup failed\n\0")); } }
                }
            } else { let mut errbuf = [0 as c_char; 512]; target__strerror(&mut target, err, errbuf.as_mut_ptr(), 512); pr_err(cstr(b"%s\n\0"), errbuf.as_ptr()); }
        } else if !data.is_pipe {
            if !perf_session__has_traces(session, cstr(b"lock record\0")) { err = -EINVAL; }
            else if evlist__find_evsel_by_str((*session).evlist, cstr(b"lock:contention_begin\0")).is_null() { pr_err(cstr(b"lock contention evsel not found\n\0")); err = -EINVAL; }
            else if perf_session__set_tracepoints_handlers(session, contention_tracepoints.as_ptr()) != 0 { pr_err(cstr(b"Initializing perf session tracepoint handlers failed\n\0")); err = -EINVAL; }
        }
        if err >= 0 {
            err = setup_output_field(true, output_fields);
            if err != 0 { pr_err(cstr(b"Failed to setup output field\n\0")); }
        }
        if err == 0 { err = select_key(true); }
        if err == 0 && !symbol_conf.field_sep.is_null() { let mut i = 0; while !contention_keys[i].name.is_null() { contention_keys[i].len = 0; i += 1; } }
        if err == 0 {
            if use_bpf {
                lock_contention_start(); if argc != 0 { evlist__start_workload(con.evlist); }
                while done == 0 { if argc != 0 && waitpid(evlist__workload_pid(con.evlist), ptr::null_mut(), WNOHANG) > 0 { break; } sleep(1); }
                lock_contention_stop(); lock_contention_read(&mut con);
            } else { err = perf_session__process_events(session); }
        }
        if err == 0 { setup_pager(); sort_contention_result(); print_contention_result(&mut con); }
    }
    lock_filter_finish(); evlist__put(con.evlist); lock_contention_finish(&mut con); perf_session__delete(session); perf_env__exit(&mut host_env); zfree(&mut lockhash_table as *mut _ as *mut *mut c_void);
    err
}

unsafe fn __cmd_record(argc: c_int, argv: *const *const c_char) -> c_int {
    let record_args = [cstr(b"record\0"), cstr(b"-R\0"), cstr(b"-m\0"), cstr(b"1024\0"), cstr(b"-c\0"), cstr(b"1\0"), cstr(b"--synth\0"), cstr(b"task\0")];
    let callgraph_args = [cstr(b"--call-graph\0"), cstr(b"fp,8\0")];
    let mut has_lock_stat = true;
    for tp in lock_tracepoints.iter() {
        if !is_valid_tracepoint(tp.name) { pr_debug(cstr(b"tracepoint %s is not enabled. Are CONFIG_LOCKDEP and CONFIG_LOCK_STAT enabled?\n\0"), tp.name); has_lock_stat = false; break; }
    }
    let mut nr_callgraph_args = 0usize;
    if !has_lock_stat {
        for tp in contention_tracepoints.iter() {
            if !is_valid_tracepoint(tp.name) { pr_err(cstr(b"tracepoint %s is not enabled.\n\0"), tp.name); return 1; }
        }
        nr_callgraph_args = callgraph_args.len();
    }
    let nr_tracepoints = if has_lock_stat { lock_tracepoints.len() } else { contention_tracepoints.len() };
    let rec_argc = record_args.len() + nr_callgraph_args + argc as usize - 1 + 2 * nr_tracepoints;
    let rec_argv = calloc(rec_argc + 1, size_of::<*const c_char>()) as *mut *const c_char;
    if rec_argv.is_null() { return -ENOMEM; }
    let mut i = 0usize;
    for a in record_args.iter() { *rec_argv.add(i) = *a; i += 1; }
    for j in 0..nr_tracepoints {
        *rec_argv.add(i) = cstr(b"-e\0"); i += 1;
        *rec_argv.add(i) = if has_lock_stat { lock_tracepoints[j].name } else { contention_tracepoints[j].name }; i += 1;
    }
    for j in 0..nr_callgraph_args { *rec_argv.add(i) = callgraph_args[j]; i += 1; }
    let mut j = 1usize;
    while j < argc as usize { *rec_argv.add(i) = *argv.add(j); i += 1; j += 1; }
    BUG_ON(i != rec_argc);
    let ret = cmd_record(i as c_int, rec_argv as *const *const c_char);
    free(rec_argv as *mut c_void);
    ret
}

unsafe extern "C" fn parse_map_entry(opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    let len = (*opt).value as *mut c_ulong;
    let mut endptr: *mut c_char = ptr::null_mut();
    errno = 0;
    let val = strtoul(str_, &mut endptr, 0);
    if *endptr != 0 || errno != 0 { pr_err(cstr(b"invalid BPF map length: %s\n\0"), str_); return -1; }
    *len = val; 0
}

unsafe extern "C" fn parse_max_stack(opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    let len = (*opt).value as *mut c_int;
    let mut endptr: *mut c_char = ptr::null_mut();
    errno = 0;
    let val = strtol(str_, &mut endptr, 0);
    if *endptr != 0 || errno != 0 { pr_err(cstr(b"invalid max stack depth: %s\n\0"), str_); return -1; }
    if val < 0 || val > sysctl__max_stack() { pr_err(cstr(b"invalid max stack depth: %ld\n\0"), val); return -1; }
    *len = val as c_int; 0
}

unsafe fn add_lock_type(flags: c_uint) -> bool_ {
    let tmp = realloc(filters.types as *mut c_void, ((filters.nr_types + 1) as usize) * size_of::<c_uint>()) as *mut c_uint;
    if tmp.is_null() { return false; }
    *tmp.add(filters.nr_types as usize) = flags; filters.nr_types += 1; filters.types = tmp; true
}

unsafe extern "C" fn parse_lock_type(_opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    let s = strdup(str_);
    if s.is_null() { return -1; }
    let mut tmp: *mut c_char = ptr::null_mut();
    let mut tok = strtok_r(s, cstr(b", \0"), &mut tmp);
    while !tok.is_null() {
        let mut found = false;
        if !strchr(tok, ':' as c_int).is_null() {
            for e in lock_type_table.iter() { if strcmp(e.flags_name, tok) == 0 && add_lock_type(e.flags) { found = true; break; } }
            if !found { pr_err(cstr(b"Unknown lock flags name: %s\n\0"), tok); free(s as *mut c_void); return -1; }
        } else {
            if strcmp(tok, cstr(b"pcpu-sem\0")) == 0 { tok = cstr(b"percpu-rwsem\0") as *mut c_char; }
            for e in lock_type_table.iter() { if strcmp(e.lock_name, tok) == 0 { if add_lock_type(e.flags) { found = true; } else { free(s as *mut c_void); return -1; } } }
            if !found { pr_err(cstr(b"Unknown lock name: %s\n\0"), tok); free(s as *mut c_void); return -1; }
        }
        tok = strtok_r(ptr::null_mut(), cstr(b", \0"), &mut tmp);
    }
    free(s as *mut c_void); 0
}

unsafe fn add_lock_addr(addr: c_ulong) -> bool_ {
    let tmp = realloc(filters.addrs as *mut c_void, ((filters.nr_addrs + 1) as usize) * size_of::<c_ulong>()) as *mut c_ulong;
    if tmp.is_null() { pr_err(cstr(b"Memory allocation failure\n\0")); return false; }
    *tmp.add(filters.nr_addrs as usize) = addr; filters.nr_addrs += 1; filters.addrs = tmp; true
}
unsafe fn add_lock_sym(name: *mut c_char) -> bool_ {
    let sym = strdup(name);
    if sym.is_null() { pr_err(cstr(b"Memory allocation failure\n\0")); return false; }
    let tmp = realloc(filters.syms as *mut c_void, ((filters.nr_syms + 1) as usize) * size_of::<*mut c_char>()) as *mut *mut c_char;
    if tmp.is_null() { pr_err(cstr(b"Memory allocation failure\n\0")); free(sym as *mut c_void); return false; }
    *tmp.add(filters.nr_syms as usize) = sym; filters.nr_syms += 1; filters.syms = tmp; true
}
unsafe fn add_lock_slab(name: *mut c_char) -> bool_ {
    let sym = strdup(name);
    if sym.is_null() { pr_err(cstr(b"Memory allocation failure\n\0")); return false; }
    let tmp = realloc(filters.slabs as *mut c_void, ((filters.nr_slabs + 1) as usize) * size_of::<*mut c_char>()) as *mut *mut c_char;
    if tmp.is_null() { pr_err(cstr(b"Memory allocation failure\n\0")); return false; }
    *tmp.add(filters.nr_slabs as usize) = sym; filters.nr_slabs += 1; filters.slabs = tmp; true
}

unsafe extern "C" fn parse_lock_addr(_opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    let s = strdup(str_);
    if s.is_null() { return -1; }
    let mut tmp: *mut c_char = ptr::null_mut();
    let mut ret = 0;
    let mut tok = strtok_r(s, cstr(b", \0"), &mut tmp);
    while !tok.is_null() {
        let mut end: *mut c_char = ptr::null_mut();
        let addr = strtoul(tok, &mut end, 16);
        if *end == 0 { if !add_lock_addr(addr) { ret = -1; break; } }
        else if *tok == '&' as c_char { if !add_lock_slab(tok.add(1)) { ret = -1; break; } }
        else if !add_lock_sym(tok) { ret = -1; break; }
        tok = strtok_r(ptr::null_mut(), cstr(b", \0"), &mut tmp);
    }
    free(s as *mut c_void); ret
}

unsafe extern "C" fn parse_output(opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    let name = (*opt).value as *mut *const c_char;
    if str_.is_null() { return -1; }
    lock_output = fopen(str_, cstr(b"w\0"));
    if lock_output.is_null() { pr_err(cstr(b"Cannot open %s\n\0"), str_); return -1; }
    *name = str_; 0
}

unsafe fn add_lock_cgroup(name: *mut c_char) -> bool_ {
    let cgrp = cgroup__new(name, false);
    if cgrp.is_null() { pr_err(cstr(b"Failed to create cgroup: %s\n\0"), name); return false; }
    if read_cgroup_id(cgrp) < 0 { pr_err(cstr(b"Failed to read cgroup id for %s\n\0"), name); cgroup__put(cgrp); return false; }
    let tmp = realloc(filters.cgrps as *mut c_void, ((filters.nr_cgrps + 1) as usize) * size_of::<u64>()) as *mut u64;
    if tmp.is_null() { pr_err(cstr(b"Memory allocation failure\n\0")); return false; }
    *tmp.add(filters.nr_cgrps as usize) = (*cgrp).id; filters.nr_cgrps += 1; filters.cgrps = tmp; cgroup__put(cgrp); true
}
unsafe extern "C" fn parse_cgroup_filter(_opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    let s = strdup(str_);
    if s.is_null() { return -1; }
    let mut tmp: *mut c_char = ptr::null_mut();
    let mut ret = 0;
    let mut tok = strtok_r(s, cstr(b", \0"), &mut tmp);
    while !tok.is_null() { if !add_lock_cgroup(tok) { ret = -1; break; } tok = strtok_r(ptr::null_mut(), cstr(b", \0"), &mut tmp); }
    free(s as *mut c_void); ret
}

unsafe fn add_lock_delay(spec: *mut c_char) -> bool_ {
    let at = strchr(spec, '@' as c_int);
    if at.is_null() { pr_err(cstr(b"lock delay should have '@' sign: %s\n\0"), spec); return false; }
    if at == spec { pr_err(cstr(b"lock delay should have time before '@': %s\n\0"), spec); return false; }
    *at = 0;
    let mut pos: *mut c_char = ptr::null_mut();
    let mut duration = strtoul(spec, &mut pos, 0);
    if strcmp(pos, cstr(b"ns\0")) == 0 { duration *= 1; }
    else if strcmp(pos, cstr(b"us\0")) == 0 { duration *= 1000; }
    else if strcmp(pos, cstr(b"ms\0")) == 0 { duration *= 1000 * 1000; }
    else if *pos != 0 { pr_err(cstr(b"invalid delay time: %s@%s\n\0"), spec, at.add(1)); return false; }
    if duration > 10 * 1000 * 1000 { pr_err(cstr(b"lock delay is too long: %s (> 10ms)\n\0"), spec); return false; }
    let tmp = realloc(delays as *mut c_void, ((nr_delays + 1) as usize) * size_of::<lock_delay>()) as *mut lock_delay;
    if tmp.is_null() { pr_err(cstr(b"Memory allocation failure\n\0")); return false; }
    delays = tmp;
    (*delays.add(nr_delays as usize)).sym = strdup(at.add(1));
    if (*delays.add(nr_delays as usize)).sym.is_null() { pr_err(cstr(b"Memory allocation failure\n\0")); return false; }
    (*delays.add(nr_delays as usize)).time = duration;
    nr_delays += 1; true
}
unsafe extern "C" fn parse_lock_delay(_opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    let s = strdup(str_);
    if s.is_null() { return -1; }
    let mut tmp: *mut c_char = ptr::null_mut();
    let mut ret = 0;
    let mut tok = strtok_r(s, cstr(b", \0"), &mut tmp);
    while !tok.is_null() { if !add_lock_delay(tok) { ret = -1; break; } tok = strtok_r(ptr::null_mut(), cstr(b", \0"), &mut tmp); }
    free(s as *mut c_void); ret
}

#[no_mangle]
pub unsafe extern "C" fn cmd_lock(mut argc: c_int, argv: *const *const c_char) -> c_int {
    // Option-table macro initializers from C are dependency-provided; this translation
    // preserves the command dispatch and option effects while representing tables opaquely.
    let lock_options: [option; 1] = [option { value: ptr::null_mut() }];
    let info_options: [option; 1] = [option { value: ptr::null_mut() }];
    let report_options: [option; 1] = [option { value: ptr::null_mut() }];
    let mut contention_options: [option; 1] = [option { value: ptr::null_mut() }];
    let info_usage = [cstr(b"perf lock info [<options>]\0"), ptr::null()];
    let lock_subcommands = [cstr(b"record\0"), cstr(b"report\0"), cstr(b"script\0"), cstr(b"info\0"), cstr(b"contention\0"), ptr::null()];
    let mut lock_usage = [ptr::null(), ptr::null()];
    let report_usage = [cstr(b"perf lock report [<options>]\0"), ptr::null()];
    let contention_usage = [cstr(b"perf lock contention [<options>]\0"), ptr::null()];
    let mut rc = 0;
    lockhash_table = calloc(LOCKHASH_SIZE, size_of::<hlist_head>()) as *mut hlist_head;
    if lockhash_table.is_null() { return -ENOMEM; }
    for i in 0..LOCKHASH_SIZE { INIT_HLIST_HEAD(lockhash_table.add(i)); }
    lock_output = stderr;
    argc = parse_options_subcommand(argc, argv, lock_options.as_ptr(), lock_subcommands.as_ptr(), lock_usage.as_mut_ptr(), PARSE_OPT_STOP_AT_NON_OPTION);
    if argc == 0 { usage_with_options(lock_usage.as_mut_ptr(), lock_options.as_ptr()); }
    let arg0 = *argv;
    if strlen(arg0) > 2 && strstarts(cstr(b"record\0"), arg0) {
        return __cmd_record(argc, argv);
    } else if strlen(arg0) > 2 && strstarts(cstr(b"report\0"), arg0) {
        trace_handler = &report_lock_ops;
        if argc != 0 {
            argc = parse_options(argc, argv, report_options.as_ptr(), report_usage.as_ptr(), 0);
            if argc != 0 { usage_with_options(report_usage.as_ptr() as *mut *const c_char, report_options.as_ptr()); }
        }
        rc = __cmd_report(false);
    } else if strcmp(arg0, cstr(b"script\0")) == 0 {
        rc = cmd_script(argc, argv);
    } else if strcmp(arg0, cstr(b"info\0")) == 0 {
        if argc != 0 {
            argc = parse_options(argc, argv, info_options.as_ptr(), info_usage.as_ptr(), 0);
            if argc != 0 { usage_with_options(info_usage.as_ptr() as *mut *const c_char, info_options.as_ptr()); }
        }
        if !info_threads && !info_map { info_threads = true; info_map = true; }
        trace_handler = &report_lock_ops;
        rc = __cmd_report(true);
    } else if strlen(arg0) > 2 && strstarts(cstr(b"contention\0"), arg0) {
        trace_handler = &contention_lock_ops;
        sort_key = cstr(b"wait_total\0");
        output_fields = cstr(b"contended,wait_total,wait_max,avg_wait\0");
        // #ifndef HAVE_BPF_SKEL: set_option_nobuild(contention_options, 'b', "use-bpf", "no BUILD_BPF_SKEL=1", false);
        if argc != 0 { argc = parse_options(argc, argv, contention_options.as_mut_ptr(), contention_usage.as_ptr(), 0); }
        if check_lock_contention_options(contention_options.as_ptr(), contention_usage.as_ptr()) < 0 { return -1; }
        rc = __cmd_contention(argc, argv);
    } else {
        usage_with_options(lock_usage.as_mut_ptr(), lock_options.as_ptr());
    }
    free(lock_usage[0] as *mut c_void);
    zfree(&mut lockhash_table as *mut _ as *mut *mut c_void);
    rc
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
