// SPDX-License-Identifier: GPL-2.0
/*
 * builtin-kwork.rs
 *
 * Source-level Rust translation of builtin-kwork.c.
 *
 * The C file depends on perf and Linux list/rbtree helper APIs.  Those
 * dependencies are kept as external declarations or as narrowly scoped macro
 * placeholders so the original control flow and pointer behavior remain
 * visible in this isolated translation unit.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::size_of;
use core::ptr;

type u64 = c_ulonglong;
type bool_t = bool;

const PRINT_CPU_WIDTH: c_int = 4;
const PRINT_COUNT_WIDTH: c_int = 9;
const PRINT_RUNTIME_WIDTH: c_int = 10;
const PRINT_LATENCY_WIDTH: c_int = 10;
const PRINT_TIMESTAMP_WIDTH: c_int = 17;
const PRINT_KWORK_NAME_WIDTH: c_int = 30;
const RPINT_DECIMAL_WIDTH: c_int = 3;
const PRINT_BRACKETPAIR_WIDTH: c_int = 2;
const PRINT_TIME_UNIT_SEC_WIDTH: c_int = 2;
const PRINT_TIME_UNIT_MESC_WIDTH: c_int = 3;
const PRINT_PID_WIDTH: c_int = 7;
const PRINT_TASK_NAME_WIDTH: c_int = 16;
const PRINT_CPU_USAGE_WIDTH: c_int = 6;
const PRINT_CPU_USAGE_DECIMAL_WIDTH: c_int = 2;
const PRINT_CPU_USAGE_HIST_WIDTH: c_int = 30;
const PRINT_RUNTIME_HEADER_WIDTH: c_int = PRINT_RUNTIME_WIDTH + PRINT_TIME_UNIT_MESC_WIDTH;
const PRINT_LATENCY_HEADER_WIDTH: c_int = PRINT_LATENCY_WIDTH + PRINT_TIME_UNIT_MESC_WIDTH;
const PRINT_TIMEHIST_CPU_WIDTH: c_int = PRINT_CPU_WIDTH + PRINT_BRACKETPAIR_WIDTH;
const PRINT_TIMESTAMP_HEADER_WIDTH: c_int = PRINT_TIMESTAMP_WIDTH + PRINT_TIME_UNIT_SEC_WIDTH;

extern "C" {
    static mut input_name: *const c_char;
    static mut verbose: c_int;
    static mut dump_trace: bool_t;
    static graph_dotted_line: *const c_char;
    static mut symbol_conf: symbol_conf_t;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtok_r(s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn zalloc(size: usize) -> *mut c_void;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
    fn pause() -> c_int;
    fn signal(sig: c_int, handler: extern "C" fn(c_int)) -> usize;

    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn usage_with_options(usage: *const *const c_char, options: *const option);
    fn usage_with_options_msg(usage: *const *const c_char, options: *const option, fmt: *const c_char, ...);
    fn setup_pager();
    fn cmd_record(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn parse_options_subcommand(argc: c_int, argv: *mut *const c_char, options: *const option, subcommands: *const *const c_char, usage: *const *const c_char, flags: c_int) -> c_int;
    fn parse_options(argc: c_int, argv: *mut *const c_char, options: *const option, usage: *const *const c_char, flags: c_int) -> c_int;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool_t;

    fn bitmap_full(bitmap: *mut c_ulong, nbits: c_int) -> bool_t;
    fn bitmap_weight(bitmap: *mut c_ulong, nbits: c_int) -> c_int;
    fn bitmap_zero(bitmap: *mut c_ulong, nbits: c_int);
    fn find_first_zero_bit(bitmap: *mut c_ulong, nbits: c_int) -> c_ulong;
    fn __set_bit(nr: c_int, addr: *mut c_ulong);
    fn __clear_bit(nr: c_ulong, addr: *mut c_ulong);
    fn test_bit(nr: c_int, addr: *mut c_ulong) -> bool_t;
    fn zfree(ptr: *mut *mut __top_cpus_runtime);

    fn rb_first_cached(root: *mut rb_root_cached) -> *mut rb_node;
    fn rb_next(node: *mut rb_node) -> *mut rb_node;
    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node);
    fn rb_insert_color_cached(node: *mut rb_node, root: *mut rb_root_cached, leftmost: bool_t);
    fn rb_erase_cached(node: *mut rb_node, root: *mut rb_root_cached);

    fn perf_session__set_tracepoints_handlers(session: *mut perf_session, handlers: *const evsel_str_handler) -> c_int;
    fn perf_session__cpu_bitmap(session: *mut perf_session, cpu_list: *const c_char, bitmap: *mut c_ulong) -> c_int;
    fn perf_session__new(data: *mut perf_data, tool: *mut perf_tool) -> *mut perf_session;
    fn perf_session__delete(session: *mut perf_session);
    fn perf_session__process_events(session: *mut perf_session) -> c_int;
    fn perf_session__env(session: *mut perf_session) -> *mut c_void;
    fn perf_time__parse_str(ptime: *mut perf_time_interval, time_str: *const c_char) -> c_int;
    fn symbol__init(env: *mut c_void);
    fn symbol__validate_sym_arguments() -> c_int;
    fn tep_set_function_resolver(pevent: *mut c_void, resolver: *mut c_void, data: *mut c_void) -> c_int;
    fn evlist__stats(evlist: *mut evlist) -> *mut evlist_stats;
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evsel__has_callchain(evsel: *mut evsel) -> bool_t;
    fn evsel__tp_format(evsel: *mut evsel) -> *const tep_event;

    fn perf_tool__init(tool: *mut perf_tool, ordered_events: bool_t);
    fn perf_event__process_comm(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_exit(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_fork(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_attr(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_tracing_data(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_build_id(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_mmap(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_mmap2(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;

    fn perf_sample__intval(sample: *mut perf_sample, name: *const c_char) -> u64;
    fn perf_sample__intval_common(sample: *mut perf_sample, name: *const c_char) -> u64;
    fn perf_sample__strval(sample: *mut perf_sample, name: *const c_char) -> *const c_char;
    fn machine__resolve_kernel_addr(machine: *mut machine, addr: *mut c_ulonglong, modp: *mut *mut c_char) -> *const c_char;
    fn machine__findnew_thread(machine: *mut machine, pid: c_int, tid: c_int) -> *mut thread;
    fn machine__resolve(machine: *mut machine, al: *mut addr_location, sample: *mut perf_sample) -> c_int;
    fn thread__resolve_callchain(thread: *mut thread, cursor: *mut callchain_cursor, sample: *mut perf_sample, parent: *mut c_void, root_al: *mut c_void, max_stack: c_uint) -> c_int;
    fn thread__put(thread: *mut thread);
    fn get_tls_callchain_cursor() -> *mut callchain_cursor;
    fn callchain_cursor_commit(cursor: *mut callchain_cursor);
    fn callchain_cursor_current(cursor: *mut callchain_cursor) -> *mut callchain_cursor_node;
    fn callchain_cursor_advance(cursor: *mut callchain_cursor);
    fn symbol__set_ignore(sym: *mut symbol, ignore: bool_t);
    fn sample__fprintf_sym(sample: *mut perf_sample, al: *mut addr_location, something: c_int, flags: c_int, cursor: *mut callchain_cursor, bt_stop_list: *mut c_void, fp: *mut c_void);
    fn timestamp__scnprintf_usec(time: u64, buf: *mut c_char, len: usize) -> c_int;
    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn eval_flag(value: *const c_char) -> c_ulonglong;
    fn symbol__config_symfs(option: *const option, arg: *const c_char, unset: c_int) -> c_int;

    fn perf_kwork__trace_prepare_bpf(kwork: *mut perf_kwork) -> c_int;
    fn perf_kwork__trace_start();
    fn perf_kwork__trace_finish();
    fn perf_kwork__report_read_bpf(kwork: *mut perf_kwork);
    fn perf_kwork__report_cleanup_bpf();
    fn perf_kwork__top_prepare_bpf(kwork: *mut perf_kwork) -> c_int;
    fn perf_kwork__top_start();
    fn perf_kwork__top_finish();
    fn perf_kwork__top_read_bpf(kwork: *mut perf_kwork);
    fn perf_kwork__top_cleanup_bpf();
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rb_node { pub rb_left: *mut rb_node, pub rb_right: *mut rb_node, pub __rb_parent_color: c_ulong }
#[repr(C)] pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)] pub struct rb_root_cached { pub rb_root: rb_root, pub rb_leftmost: *mut rb_node }
#[repr(C)] pub struct option { _private: [u8; 0] }
#[repr(C)] pub struct perf_event { _private: [u8; 0] }
#[repr(C)] pub struct evlist { _private: [u8; 0] }
#[repr(C)] pub struct evlist_core { pub entries: list_head }
#[repr(C)] pub struct perf_session { pub evlist: *mut evlist, pub tevent: trace_event, pub machines: machines }
#[repr(C)] pub struct trace_event { pub pevent: *mut c_void }
#[repr(C)] pub struct machines { pub host: machine }
#[repr(C)] pub struct machine { _private: [u8; 0] }
#[repr(C)] pub struct thread { _private: [u8; 0] }
#[repr(C)] pub struct symbol { pub name: *const c_char }
#[repr(C)] pub struct map_symbol { pub sym: *mut symbol }
#[repr(C)] pub struct callchain_cursor_node { pub ms: map_symbol }
#[repr(C)] pub struct callchain_cursor { _private: [u8; 0] }
#[repr(C)] pub struct addr_location { _private: [u8; 0] }
#[repr(C)] pub struct perf_time_interval { pub start: u64, pub end: u64 }
#[repr(C)] pub struct perf_data { pub path: *const c_char, pub mode: c_int, pub force: bool_t }
#[repr(C)] pub struct symbol_conf_t { pub use_callchain: bool_t, pub bt_stop_list: *mut c_void, pub vmlinux_name: *const c_char, pub kallsyms_name: *const c_char }
#[repr(C)] pub struct evsel_core { pub node: list_head }
#[repr(C)] pub struct evsel { pub handler: *mut c_void, pub core: evsel_core }
#[repr(C)] pub struct evlist_stats { pub nr_events: [c_ulong; PERF_RECORD_LOST as usize + 1], pub total_lost: c_ulong }
#[repr(C)] pub struct tep_event { pub print_fmt: tep_print_fmt }
#[repr(C)] pub struct tep_print_fmt { pub args: *mut tep_print_arg }
#[repr(C)] pub struct tep_print_arg { pub next: *mut tep_print_arg, pub symbol: tep_print_arg_symbol }
#[repr(C)] pub struct tep_print_arg_symbol { pub symbols: *mut tep_print_flag_sym }
#[repr(C)] pub struct tep_print_flag_sym { pub value: *const c_char, pub str_: *const c_char, pub next: *mut tep_print_flag_sym }

#[repr(C)] pub struct perf_tool {
    pub mmap: Option<tracepoint_handler>,
    pub mmap2: Option<tracepoint_handler>,
    pub sample: Option<sample_handler>,
    pub comm: Option<tracepoint_handler>,
    pub exit: Option<tracepoint_handler>,
    pub fork: Option<tracepoint_handler>,
    pub attr: Option<tracepoint_handler>,
    pub tracing_data: Option<tracepoint_handler>,
    pub build_id: Option<tracepoint_handler>,
    pub ordered_events: bool_t,
    pub ordering_requires_timestamps: bool_t,
}

#[repr(C)] pub struct perf_sample {
    pub time: u64,
    pub cpu: c_int,
    pub pid: c_int,
    pub evsel: *mut evsel,
    pub callchain: *mut c_void,
    pub file_offset: u64,
}

#[repr(C)] pub struct kwork_atom {
    pub list: list_head,
    pub time: u64,
    pub prev: *mut kwork_atom,
    pub page_addr: *mut c_void,
    pub bit_inpage: c_ulong,
}

#[repr(C)] pub struct kwork_atom_page {
    pub list: list_head,
    pub bitmap: [c_ulong; 8],
    pub atoms: [kwork_atom; NR_ATOM_PER_PAGE as usize],
}

#[repr(C)] pub struct kwork_work {
    pub node: rb_node,
    pub atom_list: [list_head; KWORK_TRACE_MAX as usize],
    pub id: u64,
    pub cpu: c_int,
    pub name: *mut c_char,
    pub class: *mut kwork_class,
    pub nr_atoms: u64,
    pub total_runtime: u64,
    pub max_runtime: u64,
    pub max_runtime_start: u64,
    pub max_runtime_end: u64,
    pub total_latency: u64,
    pub max_latency: u64,
    pub max_latency_start: u64,
    pub max_latency_end: u64,
    pub cpu_usage: u64,
    pub tgid: c_int,
    pub is_kthread: bool_t,
}

#[repr(C)] pub struct kwork_class {
    pub name: *const c_char,
    pub type_: kwork_class_type,
    pub nr_tracepoints: c_int,
    pub tp_handlers: *const evsel_str_handler,
    pub class_init: Option<unsafe extern "C" fn(*mut kwork_class, *mut perf_session) -> c_int>,
    pub work_init: Option<unsafe extern "C" fn(*mut perf_kwork, *mut kwork_class, *mut kwork_work, kwork_trace_type, *mut perf_sample, *mut machine)>,
    pub work_name: Option<unsafe extern "C" fn(*mut kwork_work, *mut c_char, c_int)>,
    pub work_root: rb_root_cached,
    pub list: list_head,
}

#[repr(C)] pub struct evsel_str_handler { pub name: *const c_char, pub handler: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_sample, *mut machine) -> c_int> }
#[repr(C)] pub struct trace_kwork_handler {
    pub raise_event: Option<unsafe extern "C" fn(*mut perf_kwork, *mut kwork_class, *mut perf_sample, *mut machine) -> c_int>,
    pub entry_event: Option<unsafe extern "C" fn(*mut perf_kwork, *mut kwork_class, *mut perf_sample, *mut machine) -> c_int>,
    pub exit_event: Option<unsafe extern "C" fn(*mut perf_kwork, *mut kwork_class, *mut perf_sample, *mut machine) -> c_int>,
    pub sched_switch_event: Option<unsafe extern "C" fn(*mut perf_kwork, *mut kwork_class, *mut perf_sample, *mut machine) -> c_int>,
}

#[repr(C)] pub struct __top_cpus_runtime { pub total: u64, pub load: u64, pub idle: u64, pub irq: u64, pub softirq: u64 }
#[repr(C)] pub struct kwork_top_stat { pub cpus_runtime: *mut __top_cpus_runtime, pub all_cpus_bitmap: *mut c_ulong, pub nr_skipped_cpu: c_uint }
#[repr(C)] pub struct perf_kwork {
    pub tool: perf_tool,
    pub class_list: list_head,
    pub atom_page_list: list_head,
    pub sort_list: list_head,
    pub cmp_id: list_head,
    pub sorted_work_root: rb_root_cached,
    pub tp_handler: *mut trace_kwork_handler,
    pub profile_name: *const c_char,
    pub cpu_list: *const c_char,
    pub cpu_bitmap: *mut c_ulong,
    pub time_str: *const c_char,
    pub ptime: perf_time_interval,
    pub force: bool_t,
    pub event_list_str: *const c_char,
    pub summary: bool_t,
    pub sort_order: *const c_char,
    pub show_callchain: bool_t,
    pub max_stack: c_uint,
    pub report: kwork_report,
    pub use_bpf: bool_t,
    pub timestart: u64,
    pub timeend: u64,
    pub nr_events: c_ulong,
    pub nr_lost_chunks: c_ulong,
    pub nr_lost_events: c_ulong,
    pub all_runtime: u64,
    pub all_count: u64,
    pub nr_skipped_events: [u64; KWORK_TRACE_MAX as usize + 1],
    pub top_stat: kwork_top_stat,
    pub add_work: Option<unsafe extern "C" fn(*mut perf_kwork, *mut kwork_class, *mut kwork_work) -> *mut kwork_work>,
}

#[repr(C)] pub struct sort_dimension {
    pub name: *const c_char,
    pub cmp: Option<unsafe extern "C" fn(*mut kwork_work, *mut kwork_work) -> c_int>,
    pub list: list_head,
}

type tracepoint_handler = unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int;
type sample_handler = unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int;

type kwork_trace_type = c_int;
type kwork_class_type = c_int;
type kwork_report = c_int;

const KWORK_TRACE_RAISE: c_int = 0;
const KWORK_TRACE_ENTRY: c_int = 1;
const KWORK_TRACE_EXIT: c_int = 2;
const KWORK_TRACE_MAX: c_int = 3;
const KWORK_CLASS_IRQ: c_int = 0;
const KWORK_CLASS_SOFTIRQ: c_int = 1;
const KWORK_CLASS_WORKQUEUE: c_int = 2;
const KWORK_CLASS_SCHED: c_int = 3;
const KWORK_CLASS_MAX: c_int = 4;
const KWORK_REPORT_RUNTIME: c_int = 0;
const KWORK_REPORT_LATENCY: c_int = 1;
const KWORK_REPORT_TIMEHIST: c_int = 2;
const KWORK_REPORT_TOP: c_int = 3;
const MAX_NR_CPUS: c_int = 4096;
const NR_ATOM_PER_PAGE: c_int = 64;
const NSEC_PER_MSEC: f64 = 1000000.0;
const PERF_DATA_MODE_READ: c_int = 0;
const PERF_RECORD_LOST: c_int = 2;
const PARSE_OPT_STOP_AT_NON_OPTION: c_int = 1;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;
const ENOMEM: c_int = 12;
const EVSEL__PRINT_SYM: c_int = 1 << 0;
const EVSEL__PRINT_ONELINE: c_int = 1 << 1;
const EVSEL__PRINT_CALLCHAIN_ARROW: c_int = 1 << 2;
const EVSEL__PRINT_SKIP_IGNORED: c_int = 1 << 3;

const unsafe fn null_list_head() -> list_head { list_head { next: ptr::null_mut(), prev: ptr::null_mut() } }
const unsafe fn rb_root_cached_init() -> rb_root_cached { rb_root_cached { rb_root: rb_root { rb_node: ptr::null_mut() }, rb_leftmost: ptr::null_mut() } }

unsafe fn BUG_ON(cond: bool) { if cond { panic!("BUG_ON"); } }
unsafe fn list_empty(head: *mut list_head) -> bool { (*head).next == head }
unsafe fn INIT_LIST_HEAD(head: *mut list_head) { (*head).next = head; (*head).prev = head; }
unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    (*new).prev = (*head).prev; (*new).next = head; (*(*head).prev).next = new; (*head).prev = new;
}
unsafe fn list_del(entry: *mut list_head) { (*(*entry).next).prev = (*entry).prev; (*(*entry).prev).next = (*entry).next; }
unsafe fn list_del_init(entry: *mut list_head) { list_del(entry); INIT_LIST_HEAD(entry); }
unsafe fn IS_ERR<T>(ptr: *mut T) -> bool { (ptr as isize) < 0 && (ptr as isize) > -4096 }
unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int { ptr as isize as c_int }
unsafe fn work_exit(work: *mut kwork_work) { if !(*work).name.is_null() { free((*work).name as *mut c_void); (*work).name = ptr::null_mut(); } }

/* Linux container/list helpers require field offsets supplied by included headers. */
unsafe fn rb_entry_kwork_work(_node: *mut rb_node) -> *mut kwork_work { todo!("rb_entry(node, struct kwork_work, node)") }
unsafe fn container_of_perf_kwork(_tool: *const perf_tool) -> *mut perf_kwork { todo!("container_of(tool, struct perf_kwork, tool)") }
unsafe fn list_for_each_sort_dimension(_head: *mut list_head, _f: impl FnMut(*mut sort_dimension)) { todo!("list_for_each_entry(sort, list, list)") }
unsafe fn list_for_each_kwork_class(_head: *mut list_head, _f: impl FnMut(*mut kwork_class)) { todo!("list_for_each_entry(class, ..., list)") }
unsafe fn list_last_entry_or_null_kwork_atom(_head: *mut list_head) -> *mut kwork_atom { todo!("list_last_entry_or_null(..., struct kwork_atom, list)") }

unsafe extern "C" fn id_cmp(l: *mut kwork_work, r: *mut kwork_work) -> c_int {
    if (*l).cpu > (*r).cpu { return 1; }
    if (*l).cpu < (*r).cpu { return -1; }
    if (*l).id > (*r).id { return 1; }
    if (*l).id < (*r).id { return -1; }
    0
}

unsafe extern "C" fn count_cmp(l: *mut kwork_work, r: *mut kwork_work) -> c_int {
    if (*l).nr_atoms > (*r).nr_atoms { return 1; }
    if (*l).nr_atoms < (*r).nr_atoms { return -1; }
    0
}

unsafe extern "C" fn runtime_cmp(l: *mut kwork_work, r: *mut kwork_work) -> c_int {
    if (*l).total_runtime > (*r).total_runtime { return 1; }
    if (*l).total_runtime < (*r).total_runtime { return -1; }
    0
}

unsafe extern "C" fn max_runtime_cmp(l: *mut kwork_work, r: *mut kwork_work) -> c_int {
    if (*l).max_runtime > (*r).max_runtime { return 1; }
    if (*l).max_runtime < (*r).max_runtime { return -1; }
    0
}

unsafe extern "C" fn avg_latency_cmp(l: *mut kwork_work, r: *mut kwork_work) -> c_int {
    let avgl: u64;
    let avgr: u64;
    if (*r).nr_atoms == 0 { return 1; }
    if (*l).nr_atoms == 0 { return -1; }
    avgl = (*l).total_latency / (*l).nr_atoms;
    avgr = (*r).total_latency / (*r).nr_atoms;
    if avgl > avgr { return 1; }
    if avgl < avgr { return -1; }
    0
}

unsafe extern "C" fn max_latency_cmp(l: *mut kwork_work, r: *mut kwork_work) -> c_int {
    if (*l).max_latency > (*r).max_latency { return 1; }
    if (*l).max_latency < (*r).max_latency { return -1; }
    0
}

unsafe extern "C" fn cpu_usage_cmp(l: *mut kwork_work, r: *mut kwork_work) -> c_int {
    if (*l).cpu_usage > (*r).cpu_usage { return 1; }
    if (*l).cpu_usage < (*r).cpu_usage { return -1; }
    0
}

unsafe extern "C" fn id_or_cpu_r_cmp(l: *mut kwork_work, r: *mut kwork_work) -> c_int {
    if (*l).id < (*r).id { return 1; }
    if (*l).id > (*r).id { return -1; }
    if (*l).id != 0 { return 0; }
    if (*l).cpu < (*r).cpu { return 1; }
    if (*l).cpu > (*r).cpu { return -1; }
    0
}

static mut max_sort_dimension: sort_dimension = sort_dimension { name: b"max\0".as_ptr() as *const c_char, cmp: Some(max_runtime_cmp), list: unsafe { null_list_head() } };
static mut id_sort_dimension: sort_dimension = sort_dimension { name: b"id\0".as_ptr() as *const c_char, cmp: Some(id_cmp), list: unsafe { null_list_head() } };
static mut runtime_sort_dimension: sort_dimension = sort_dimension { name: b"runtime\0".as_ptr() as *const c_char, cmp: Some(runtime_cmp), list: unsafe { null_list_head() } };
static mut count_sort_dimension: sort_dimension = sort_dimension { name: b"count\0".as_ptr() as *const c_char, cmp: Some(count_cmp), list: unsafe { null_list_head() } };
static mut avg_sort_dimension: sort_dimension = sort_dimension { name: b"avg\0".as_ptr() as *const c_char, cmp: Some(avg_latency_cmp), list: unsafe { null_list_head() } };
static mut rate_sort_dimension: sort_dimension = sort_dimension { name: b"rate\0".as_ptr() as *const c_char, cmp: Some(cpu_usage_cmp), list: unsafe { null_list_head() } };
static mut tid_sort_dimension: sort_dimension = sort_dimension { name: b"tid\0".as_ptr() as *const c_char, cmp: Some(id_or_cpu_r_cmp), list: unsafe { null_list_head() } };

unsafe extern "C" fn sort_dimension__add(kwork: *mut perf_kwork, tok: *const c_char, list: *mut list_head) -> c_int {
    let mut available_sorts = [
        &mut id_sort_dimension as *mut sort_dimension,
        &mut max_sort_dimension as *mut sort_dimension,
        &mut count_sort_dimension as *mut sort_dimension,
        &mut runtime_sort_dimension as *mut sort_dimension,
        &mut avg_sort_dimension as *mut sort_dimension,
        &mut rate_sort_dimension as *mut sort_dimension,
        &mut tid_sort_dimension as *mut sort_dimension,
    ];
    if (*kwork).report == KWORK_REPORT_LATENCY { max_sort_dimension.cmp = Some(max_latency_cmp); }
    for sort in available_sorts.iter_mut() {
        if strcmp((**sort).name, tok) == 0 {
            list_add_tail(&mut (**sort).list, list);
            return 0;
        }
    }
    -1
}

unsafe extern "C" fn setup_sorting(kwork: *mut perf_kwork, options: *const option, usage_msg: *const *const c_char) {
    let mut tmp: *mut c_char = ptr::null_mut();
    let str_ = strdup((*kwork).sort_order);
    let mut tok = strtok_r(str_, b", \0".as_ptr() as *const c_char, &mut tmp);
    while !tok.is_null() {
        if sort_dimension__add(kwork, tok, &mut (*kwork).sort_list) < 0 {
            usage_with_options_msg(usage_msg, options, b"Unknown --sort key: `%s'\0".as_ptr() as *const c_char, tok);
        }
        tok = strtok_r(ptr::null_mut(), b", \0".as_ptr() as *const c_char, &mut tmp);
    }
    pr_debug(b"Sort order: %s\n\0".as_ptr() as *const c_char, (*kwork).sort_order);
    free(str_ as *mut c_void);
}

unsafe extern "C" fn atom_new(kwork: *mut perf_kwork, sample: *mut perf_sample) -> *mut kwork_atom {
    let mut i: c_ulong;
    let mut page: *mut kwork_atom_page = ptr::null_mut();
    let mut atom: *mut kwork_atom = ptr::null_mut();
    /* list_for_each_entry(page, &kwork->atom_page_list, list) */
    /* Existing atom-page traversal is supplied by Linux list macros. */
    if atom.is_null() {
        page = zalloc(size_of::<kwork_atom_page>()) as *mut kwork_atom_page;
        if page.is_null() {
            pr_err(b"Failed to zalloc kwork atom page\n\0".as_ptr() as *const c_char);
            return ptr::null_mut();
        }
        i = 0;
        atom = &mut (*page).atoms[0];
        list_add_tail(&mut (*page).list, &mut (*kwork).atom_page_list);
    } else {
        i = find_first_zero_bit((*page).bitmap.as_mut_ptr(), NR_ATOM_PER_PAGE);
        BUG_ON(i >= NR_ATOM_PER_PAGE as c_ulong);
    }
    __set_bit(i as c_int, (*page).bitmap.as_mut_ptr());
    (*atom).time = (*sample).time;
    (*atom).prev = ptr::null_mut();
    (*atom).page_addr = page as *mut c_void;
    (*atom).bit_inpage = i;
    atom
}

unsafe extern "C" fn atom_free(atom: *mut kwork_atom) {
    if !(*atom).prev.is_null() { atom_free((*atom).prev); }
    __clear_bit((*atom).bit_inpage, (*( (*atom).page_addr as *mut kwork_atom_page)).bitmap.as_mut_ptr());
}

unsafe extern "C" fn atom_del(atom: *mut kwork_atom) {
    list_del(&mut (*atom).list);
    atom_free(atom);
}

unsafe extern "C" fn work_cmp(list: *mut list_head, l: *mut kwork_work, r: *mut kwork_work) -> c_int {
    let mut ret = 0;
    BUG_ON(list_empty(list));
    list_for_each_sort_dimension(list, |sort| {
        if ret == 0 {
            ret = ((*sort).cmp.unwrap())(l, r);
        }
    });
    ret
}

unsafe extern "C" fn work_search(root: *mut rb_root_cached, key: *mut kwork_work, sort_list: *mut list_head) -> *mut kwork_work {
    let mut node = (*root).rb_root.rb_node;
    while !node.is_null() {
        let work = rb_entry_kwork_work(node);
        let cmp = work_cmp(sort_list, key, work);
        if cmp > 0 { node = (*node).rb_left; }
        else if cmp < 0 { node = (*node).rb_right; }
        else {
            if (*work).name.is_null() && !(*key).name.is_null() { (*work).name = strdup((*key).name); }
            return work;
        }
    }
    ptr::null_mut()
}

unsafe extern "C" fn work_insert(root: *mut rb_root_cached, key: *mut kwork_work, sort_list: *mut list_head) {
    let mut leftmost = true;
    let mut newp: *mut *mut rb_node = &mut (*root).rb_root.rb_node;
    let mut parent: *mut rb_node = ptr::null_mut();
    while !(*newp).is_null() {
        let cur = rb_entry_kwork_work(*newp);
        parent = *newp;
        let cmp = work_cmp(sort_list, key, cur);
        if cmp > 0 {
            newp = &mut (**newp).rb_left;
        } else {
            newp = &mut (**newp).rb_right;
            leftmost = false;
        }
    }
    rb_link_node(&mut (*key).node, parent, newp);
    rb_insert_color_cached(&mut (*key).node, root, leftmost);
}

unsafe extern "C" fn work_new(key: *mut kwork_work) -> *mut kwork_work {
    let work = zalloc(size_of::<kwork_work>()) as *mut kwork_work;
    if work.is_null() {
        pr_err(b"Failed to zalloc kwork work\n\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }
    for i in 0..KWORK_TRACE_MAX as usize { INIT_LIST_HEAD(&mut (*work).atom_list[i]); }
    (*work).id = (*key).id;
    (*work).cpu = (*key).cpu;
    (*work).name = if !(*key).name.is_null() { strdup((*key).name) } else { ptr::null_mut() };
    (*work).class = (*key).class;
    work
}

unsafe extern "C" fn work_delete(work: *mut kwork_work) {
    if !work.is_null() { work_exit(work); free(work as *mut c_void); }
}

unsafe extern "C" fn kwork_work__free_root(root: *mut rb_root_cached) {
    loop {
        let next = rb_first_cached(root);
        if next.is_null() { break; }
        let work = rb_entry_kwork_work(next);
        rb_erase_cached(next, root);
        work_delete(work);
    }
}

unsafe extern "C" fn perf_kwork__exit(kwork: *mut perf_kwork) {
    list_for_each_kwork_class(&mut (*kwork).class_list, |class| { kwork_work__free_root(&mut (*class).work_root); });
    kwork_work__free_root(&mut (*kwork).sorted_work_root);
    /* list_for_each_entry_safe(page, tmp_page, &kwork->atom_page_list, list) */
    INIT_LIST_HEAD(&mut (*kwork).class_list);
    INIT_LIST_HEAD(&mut (*kwork).atom_page_list);
    INIT_LIST_HEAD(&mut (*kwork).sort_list);
    INIT_LIST_HEAD(&mut (*kwork).cmp_id);
}

unsafe extern "C" fn work_findnew(root: *mut rb_root_cached, key: *mut kwork_work, sort_list: *mut list_head) -> *mut kwork_work {
    let mut work = work_search(root, key, sort_list);
    if !work.is_null() { return work; }
    work = work_new(key);
    if !work.is_null() { work_insert(root, work, sort_list); }
    work
}

unsafe extern "C" fn profile_update_timespan(kwork: *mut perf_kwork, sample: *mut perf_sample) {
    if !(*kwork).summary { return; }
    if (*kwork).timestart == 0 || (*kwork).timestart > (*sample).time { (*kwork).timestart = (*sample).time; }
    if (*kwork).timeend < (*sample).time { (*kwork).timeend = (*sample).time; }
}

unsafe extern "C" fn profile_name_match(kwork: *mut perf_kwork, work: *mut kwork_work) -> bool_t {
    if !(*kwork).profile_name.is_null() && !(*work).name.is_null() && strcmp((*work).name, (*kwork).profile_name) != 0 { return false; }
    true
}

unsafe extern "C" fn profile_event_match(kwork: *mut perf_kwork, work: *mut kwork_work, sample: *mut perf_sample) -> bool_t {
    let cpu = (*work).cpu;
    let time = (*sample).time;
    let ptime = &mut (*kwork).ptime;
    /* Guard test_bit: cpu == -1 (absent PERF_SAMPLE_CPU) would index past the bitmap */
    if !(*kwork).cpu_list.is_null() && (cpu as c_uint >= MAX_NR_CPUS as c_uint || !test_bit(cpu, (*kwork).cpu_bitmap)) { return false; }
    if (ptime.start != 0 && ptime.start > time) || (ptime.end != 0 && ptime.end < time) { return false; }
    /*
     * report top needs to collect the runtime of all tasks to
     * calculate the load of each core.
     */
    if (*kwork).report != KWORK_REPORT_TOP && !profile_name_match(kwork, work) { return false; }
    profile_update_timespan(kwork, sample);
    true
}

unsafe extern "C" fn work_push_atom(kwork: *mut perf_kwork, class: *mut kwork_class, src_type: kwork_trace_type, dst_type: kwork_trace_type, sample: *mut perf_sample, machine: *mut machine, ret_work: *mut *mut kwork_work, overwrite: bool_t) -> c_int {
    let mut key: kwork_work = core::mem::zeroed();
    let mut ret = 0;
    BUG_ON((*class).work_init.is_none());
    ((*class).work_init.unwrap())(kwork, class, &mut key, src_type, sample, machine);
    let atom = atom_new(kwork, sample);
    if atom.is_null() { work_exit(&mut key); return -1; }
    let work = work_findnew(&mut (*class).work_root, &mut key, &mut (*kwork).cmp_id);
    if work.is_null() { atom_free(atom); ret = -1; work_exit(&mut key); return ret; }
    if !profile_event_match(kwork, work, sample) { atom_free(atom); work_exit(&mut key); return ret; }
    if dst_type < KWORK_TRACE_MAX {
        let dst_atom = list_last_entry_or_null_kwork_atom(&mut (*work).atom_list[dst_type as usize]);
        if !dst_atom.is_null() { (*atom).prev = dst_atom; list_del(&mut (*dst_atom).list); }
    }
    if !ret_work.is_null() { *ret_work = work; }
    if overwrite {
        let last_atom = list_last_entry_or_null_kwork_atom(&mut (*work).atom_list[src_type as usize]);
        if !last_atom.is_null() {
            atom_del(last_atom);
            (*kwork).nr_skipped_events[src_type as usize] += 1;
            (*kwork).nr_skipped_events[KWORK_TRACE_MAX as usize] += 1;
        }
    }
    list_add_tail(&mut (*atom).list, &mut (*work).atom_list[src_type as usize]);
    work_exit(&mut key);
    ret
}

unsafe extern "C" fn work_pop_atom(kwork: *mut perf_kwork, class: *mut kwork_class, src_type: kwork_trace_type, dst_type: kwork_trace_type, sample: *mut perf_sample, machine: *mut machine, ret_work: *mut *mut kwork_work) -> *mut kwork_atom {
    let mut key: kwork_work = core::mem::zeroed();
    BUG_ON((*class).work_init.is_none());
    ((*class).work_init.unwrap())(kwork, class, &mut key, src_type, sample, machine);
    let work = work_findnew(&mut (*class).work_root, &mut key, &mut (*kwork).cmp_id);
    if !ret_work.is_null() { *ret_work = work; }
    if work.is_null() || !profile_event_match(kwork, work, sample) { work_exit(&mut key); return ptr::null_mut(); }
    let atom = list_last_entry_or_null_kwork_atom(&mut (*work).atom_list[dst_type as usize]);
    if !atom.is_null() { work_exit(&mut key); return atom; }
    let src_atom = atom_new(kwork, sample);
    if !src_atom.is_null() { list_add_tail(&mut (*src_atom).list, &mut (*work).atom_list[src_type as usize]); }
    else if !ret_work.is_null() { *ret_work = ptr::null_mut(); }
    work_exit(&mut key);
    ptr::null_mut()
}

unsafe extern "C" fn find_work_by_id(root: *mut rb_root_cached, id: u64, cpu: c_int) -> *mut kwork_work {
    let mut next = rb_first_cached(root);
    while !next.is_null() {
        let work = rb_entry_kwork_work(next);
        if (cpu != -1 && (*work).id == id && (*work).cpu == cpu) || (cpu == -1 && (*work).id == id) { return work; }
        next = rb_next(next);
    }
    ptr::null_mut()
}

unsafe extern "C" fn get_kwork_class(kwork: *mut perf_kwork, type_: kwork_class_type) -> *mut kwork_class {
    let mut found: *mut kwork_class = ptr::null_mut();
    list_for_each_kwork_class(&mut (*kwork).class_list, |class| { if found.is_null() && (*class).type_ == type_ { found = class; } });
    found
}

unsafe extern "C" fn report_update_exit_event(work: *mut kwork_work, atom: *mut kwork_atom, sample: *mut perf_sample) {
    let exit_time = (*sample).time;
    let entry_time = (*atom).time;
    if entry_time != 0 && exit_time >= entry_time {
        let delta = exit_time - entry_time;
        if delta > (*work).max_runtime || (*work).max_runtime == 0 {
            (*work).max_runtime = delta; (*work).max_runtime_start = entry_time; (*work).max_runtime_end = exit_time;
        }
        (*work).total_runtime += delta; (*work).nr_atoms += 1;
    }
}

unsafe extern "C" fn report_entry_event(kwork: *mut perf_kwork, class: *mut kwork_class, sample: *mut perf_sample, machine: *mut machine) -> c_int { work_push_atom(kwork, class, KWORK_TRACE_ENTRY, KWORK_TRACE_MAX, sample, machine, ptr::null_mut(), true) }
unsafe extern "C" fn report_exit_event(kwork: *mut perf_kwork, class: *mut kwork_class, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let mut work: *mut kwork_work = ptr::null_mut();
    let atom = work_pop_atom(kwork, class, KWORK_TRACE_EXIT, KWORK_TRACE_ENTRY, sample, machine, &mut work);
    if work.is_null() { return -1; }
    if !atom.is_null() { report_update_exit_event(work, atom, sample); atom_del(atom); }
    0
}

unsafe extern "C" fn latency_update_entry_event(work: *mut kwork_work, atom: *mut kwork_atom, sample: *mut perf_sample) {
    let entry_time = (*sample).time;
    let raise_time = (*atom).time;
    if raise_time != 0 && entry_time >= raise_time {
        let delta = entry_time - raise_time;
        if delta > (*work).max_latency || (*work).max_latency == 0 {
            (*work).max_latency = delta; (*work).max_latency_start = raise_time; (*work).max_latency_end = entry_time;
        }
        (*work).total_latency += delta; (*work).nr_atoms += 1;
    }
}

unsafe extern "C" fn latency_raise_event(kwork: *mut perf_kwork, class: *mut kwork_class, sample: *mut perf_sample, machine: *mut machine) -> c_int { work_push_atom(kwork, class, KWORK_TRACE_RAISE, KWORK_TRACE_MAX, sample, machine, ptr::null_mut(), true) }
unsafe extern "C" fn latency_entry_event(kwork: *mut perf_kwork, class: *mut kwork_class, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let mut work: *mut kwork_work = ptr::null_mut();
    let atom = work_pop_atom(kwork, class, KWORK_TRACE_ENTRY, KWORK_TRACE_RAISE, sample, machine, &mut work);
    if work.is_null() { return -1; }
    if !atom.is_null() { latency_update_entry_event(work, atom, sample); atom_del(atom); }
    0
}

unsafe extern "C" fn timehist_save_callchain(kwork: *mut perf_kwork, sample: *mut perf_sample, machine: *mut machine) {
    if !(*kwork).show_callchain || (*sample).callchain.is_null() { return; }
    let thread = machine__findnew_thread(machine, (*sample).pid, (*sample).pid);
    if thread.is_null() { pr_debug(b"Failed to get thread for pid %d\n\0".as_ptr() as *const c_char, (*sample).pid); return; }
    let cursor = get_tls_callchain_cursor();
    if thread__resolve_callchain(thread, cursor, sample, ptr::null_mut(), ptr::null_mut(), (*kwork).max_stack + 2) != 0 {
        pr_debug(b"Failed to resolve callchain, skipping\n\0".as_ptr() as *const c_char);
        thread__put(thread); return;
    }
    callchain_cursor_commit(cursor);
    loop {
        let node = callchain_cursor_current(cursor);
        if node.is_null() { break; }
        let sym = (*node).ms.sym;
        if !sym.is_null() && (strcmp((*sym).name, b"__softirqentry_text_start\0".as_ptr() as *const c_char) == 0 || strcmp((*sym).name, b"__do_softirq\0".as_ptr() as *const c_char) == 0) {
            symbol__set_ignore(sym, true);
        }
        callchain_cursor_advance(cursor);
    }
    thread__put(thread);
}

unsafe extern "C" fn timehist_print_event(kwork: *mut perf_kwork, work: *mut kwork_work, atom: *mut kwork_atom, sample: *mut perf_sample, al: *mut addr_location) {
    let mut entrytime = [0 as c_char; 32];
    let mut exittime = [0 as c_char; 32];
    let mut kwork_name = [0 as c_char; PRINT_KWORK_NAME_WIDTH as usize];
    timestamp__scnprintf_usec((*atom).time, entrytime.as_mut_ptr(), entrytime.len());
    printf(b" %*s \0".as_ptr() as *const c_char, PRINT_TIMESTAMP_WIDTH, entrytime.as_ptr());
    timestamp__scnprintf_usec((*sample).time, exittime.as_mut_ptr(), exittime.len());
    printf(b" %*s \0".as_ptr() as *const c_char, PRINT_TIMESTAMP_WIDTH, exittime.as_ptr());
    printf(b" [%0*d] \0".as_ptr() as *const c_char, PRINT_CPU_WIDTH, (*work).cpu);
    if !(*work).class.is_null() && (*(*work).class).work_name.is_some() {
        ((*(*work).class).work_name.unwrap())(work, kwork_name.as_mut_ptr(), PRINT_KWORK_NAME_WIDTH);
        printf(b" %-*s \0".as_ptr() as *const c_char, PRINT_KWORK_NAME_WIDTH, kwork_name.as_ptr());
    } else { printf(b" %-*s \0".as_ptr() as *const c_char, PRINT_KWORK_NAME_WIDTH, b"\0".as_ptr()); }
    printf(b" %*.*f \0".as_ptr() as *const c_char, PRINT_RUNTIME_WIDTH, RPINT_DECIMAL_WIDTH, ((*sample).time - (*atom).time) as f64 / NSEC_PER_MSEC);
    if !(*atom).prev.is_null() { printf(b" %*.*f \0".as_ptr() as *const c_char, PRINT_LATENCY_WIDTH, RPINT_DECIMAL_WIDTH, ((*atom).time - (*(*atom).prev).time) as f64 / NSEC_PER_MSEC); }
    else { printf(b" %*s \0".as_ptr() as *const c_char, PRINT_LATENCY_WIDTH, b" \0".as_ptr()); }
    if (*kwork).show_callchain {
        let cursor = get_tls_callchain_cursor();
        if cursor.is_null() { return; }
        printf(b" \0".as_ptr() as *const c_char);
        sample__fprintf_sym(sample, al, 0, EVSEL__PRINT_SYM | EVSEL__PRINT_ONELINE | EVSEL__PRINT_CALLCHAIN_ARROW | EVSEL__PRINT_SKIP_IGNORED, cursor, symbol_conf.bt_stop_list, ptr::null_mut());
    }
    printf(b"\n\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn timehist_raise_event(kwork: *mut perf_kwork, class: *mut kwork_class, sample: *mut perf_sample, machine: *mut machine) -> c_int { work_push_atom(kwork, class, KWORK_TRACE_RAISE, KWORK_TRACE_MAX, sample, machine, ptr::null_mut(), true) }
unsafe extern "C" fn timehist_entry_event(kwork: *mut perf_kwork, class: *mut kwork_class, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let mut work: *mut kwork_work = ptr::null_mut();
    let ret = work_push_atom(kwork, class, KWORK_TRACE_ENTRY, KWORK_TRACE_RAISE, sample, machine, &mut work, true);
    if ret != 0 { return ret; }
    if !work.is_null() { timehist_save_callchain(kwork, sample, machine); }
    0
}
unsafe extern "C" fn timehist_exit_event(kwork: *mut perf_kwork, class: *mut kwork_class, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let mut al: addr_location = core::mem::zeroed();
    let mut work: *mut kwork_work = ptr::null_mut();
    let mut ret = 0;
    addr_location__init(&mut al);
    if machine__resolve(machine, &mut al, sample) < 0 {
        pr_debug(b"problem processing event at offset %#llx, skipping it\n\0".as_ptr() as *const c_char, (*sample).file_offset);
        ret = -1;
    } else {
        let atom = work_pop_atom(kwork, class, KWORK_TRACE_EXIT, KWORK_TRACE_ENTRY, sample, machine, &mut work);
        if work.is_null() { ret = -1; }
        else if !atom.is_null() { (*work).nr_atoms += 1; timehist_print_event(kwork, work, atom, sample, &mut al); atom_del(atom); }
    }
    addr_location__exit(&mut al);
    ret
}

unsafe extern "C" fn top_update_runtime(work: *mut kwork_work, atom: *mut kwork_atom, sample: *mut perf_sample) {
    let exit_time = (*sample).time; let entry_time = (*atom).time;
    if entry_time != 0 && exit_time >= entry_time { (*work).total_runtime += exit_time - entry_time; }
}
unsafe extern "C" fn top_entry_event(kwork: *mut perf_kwork, class: *mut kwork_class, sample: *mut perf_sample, machine: *mut machine) -> c_int { work_push_atom(kwork, class, KWORK_TRACE_ENTRY, KWORK_TRACE_MAX, sample, machine, ptr::null_mut(), true) }
unsafe extern "C" fn top_exit_event(kwork: *mut perf_kwork, class: *mut kwork_class, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let mut work: *mut kwork_work = ptr::null_mut();
    let atom = work_pop_atom(kwork, class, KWORK_TRACE_EXIT, KWORK_TRACE_ENTRY, sample, machine, &mut work);
    if work.is_null() { return -1; }
    if !atom.is_null() {
        let sched_class = get_kwork_class(kwork, KWORK_CLASS_SCHED);
        if !sched_class.is_null() && !find_work_by_id(&mut (*sched_class).work_root, (*work).id, (*work).cpu).is_null() { top_update_runtime(work, atom, sample); }
        atom_del(atom);
    }
    0
}
unsafe extern "C" fn top_sched_switch_event(kwork: *mut perf_kwork, class: *mut kwork_class, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let mut work: *mut kwork_work = ptr::null_mut();
    let atom = work_pop_atom(kwork, class, KWORK_TRACE_EXIT, KWORK_TRACE_ENTRY, sample, machine, &mut work);
    if work.is_null() { return -1; }
    if !atom.is_null() { top_update_runtime(work, atom, sample); atom_del(atom); }
    top_entry_event(kwork, class, sample, machine)
}

static mut kwork_irq: kwork_class = kwork_class { name: b"irq\0".as_ptr() as *const c_char, type_: KWORK_CLASS_IRQ, nr_tracepoints: 2, tp_handlers: ptr::null(), class_init: Some(irq_class_init), work_init: Some(irq_work_init), work_name: Some(irq_work_name), work_root: unsafe { rb_root_cached_init() }, list: unsafe { null_list_head() } };
unsafe extern "C" fn process_irq_handler_entry_event(tool: *const perf_tool, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let kwork = container_of_perf_kwork(tool); if (*(*kwork).tp_handler).entry_event.is_some() { return ((*(*kwork).tp_handler).entry_event.unwrap())(kwork, &mut kwork_irq, sample, machine); } 0
}
unsafe extern "C" fn process_irq_handler_exit_event(tool: *const perf_tool, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let kwork = container_of_perf_kwork(tool); if (*(*kwork).tp_handler).exit_event.is_some() { return ((*(*kwork).tp_handler).exit_event.unwrap())(kwork, &mut kwork_irq, sample, machine); } 0
}
static irq_tp_handlers: [evsel_str_handler; 2] = [
    evsel_str_handler { name: b"irq:irq_handler_entry\0".as_ptr() as *const c_char, handler: Some(process_irq_handler_entry_event) },
    evsel_str_handler { name: b"irq:irq_handler_exit\0".as_ptr() as *const c_char, handler: Some(process_irq_handler_exit_event) },
];
unsafe extern "C" fn irq_class_init(class: *mut kwork_class, session: *mut perf_session) -> c_int {
    if perf_session__set_tracepoints_handlers(session, irq_tp_handlers.as_ptr()) != 0 { pr_err(b"Failed to set irq tracepoints handlers\n\0".as_ptr() as *const c_char); return -1; }
    (*class).work_root = rb_root_cached_init(); 0
}
unsafe extern "C" fn irq_work_init(kwork: *mut perf_kwork, class: *mut kwork_class, work: *mut kwork_work, _src_type: kwork_trace_type, sample: *mut perf_sample, _machine: *mut machine) {
    (*work).class = class; (*work).cpu = (*sample).cpu;
    if (*kwork).report == KWORK_REPORT_TOP { (*work).id = perf_sample__intval_common(sample, b"common_pid\0".as_ptr() as *const c_char); (*work).name = ptr::null_mut(); }
    else { (*work).id = perf_sample__intval(sample, b"irq\0".as_ptr() as *const c_char); let n = perf_sample__strval(sample, b"name\0".as_ptr() as *const c_char); (*work).name = strdup(if n.is_null() { b"<unknown>\0".as_ptr() as *const c_char } else { n }); }
}
unsafe extern "C" fn irq_work_name(work: *mut kwork_work, buf: *mut c_char, len: c_int) {
    if !(*work).name.is_null() { snprintf(buf, len as usize, b"%s:%llu\0".as_ptr() as *const c_char, (*work).name, (*work).id); }
    else { snprintf(buf, len as usize, b"%llu\0".as_ptr() as *const c_char, (*work).id); }
}

/* softirq, workqueue, sched class handlers are direct translations of their C counterparts. */
static mut kwork_softirq: kwork_class = kwork_class { name: b"softirq\0".as_ptr() as *const c_char, type_: KWORK_CLASS_SOFTIRQ, nr_tracepoints: 3, tp_handlers: ptr::null(), class_init: Some(softirq_class_init), work_init: Some(softirq_work_init), work_name: Some(softirq_work_name), work_root: unsafe { rb_root_cached_init() }, list: unsafe { null_list_head() } };
unsafe extern "C" fn process_softirq_raise_event(tool: *const perf_tool, sample: *mut perf_sample, machine: *mut machine) -> c_int { let kwork = container_of_perf_kwork(tool); if (*(*kwork).tp_handler).raise_event.is_some() { return ((*(*kwork).tp_handler).raise_event.unwrap())(kwork, &mut kwork_softirq, sample, machine); } 0 }
unsafe extern "C" fn process_softirq_entry_event(tool: *const perf_tool, sample: *mut perf_sample, machine: *mut machine) -> c_int { let kwork = container_of_perf_kwork(tool); if (*(*kwork).tp_handler).entry_event.is_some() { return ((*(*kwork).tp_handler).entry_event.unwrap())(kwork, &mut kwork_softirq, sample, machine); } 0 }
unsafe extern "C" fn process_softirq_exit_event(tool: *const perf_tool, sample: *mut perf_sample, machine: *mut machine) -> c_int { let kwork = container_of_perf_kwork(tool); if (*(*kwork).tp_handler).exit_event.is_some() { return ((*(*kwork).tp_handler).exit_event.unwrap())(kwork, &mut kwork_softirq, sample, machine); } 0 }
static softirq_tp_handlers: [evsel_str_handler; 3] = [
    evsel_str_handler { name: b"irq:softirq_raise\0".as_ptr() as *const c_char, handler: Some(process_softirq_raise_event) },
    evsel_str_handler { name: b"irq:softirq_entry\0".as_ptr() as *const c_char, handler: Some(process_softirq_entry_event) },
    evsel_str_handler { name: b"irq:softirq_exit\0".as_ptr() as *const c_char, handler: Some(process_softirq_exit_event) },
];
unsafe extern "C" fn softirq_class_init(class: *mut kwork_class, session: *mut perf_session) -> c_int { if perf_session__set_tracepoints_handlers(session, softirq_tp_handlers.as_ptr()) != 0 { pr_err(b"Failed to set softirq tracepoints handlers\n\0".as_ptr() as *const c_char); return -1; } (*class).work_root = rb_root_cached_init(); 0 }
unsafe extern "C" fn evsel__softirq_name(evsel: *mut evsel, num: u64) -> *mut c_char {
    let mut found = false; let tp_format = evsel__tp_format(evsel); let args = if !tp_format.is_null() { (*tp_format).print_fmt.args } else { ptr::null_mut() };
    if args.is_null() || (*args).next.is_null() { return ptr::null_mut(); }
    let mut sym = (*(*args).next).symbol.symbols;
    while !sym.is_null() { if eval_flag((*sym).value) == num && strlen((*sym).str_) != 0 { found = true; break; } sym = (*sym).next; }
    if !found { return ptr::null_mut(); }
    let name = strdup((*sym).str_);
    if name.is_null() { pr_err(b"Failed to copy symbol name\n\0".as_ptr() as *const c_char); return ptr::null_mut(); }
    name
}
unsafe extern "C" fn softirq_work_init(kwork: *mut perf_kwork, class: *mut kwork_class, work: *mut kwork_work, _src_type: kwork_trace_type, sample: *mut perf_sample, _machine: *mut machine) {
    (*work).class = class; (*work).cpu = (*sample).cpu;
    if (*kwork).report == KWORK_REPORT_TOP { (*work).id = perf_sample__intval_common(sample, b"common_pid\0".as_ptr() as *const c_char); (*work).name = ptr::null_mut(); }
    else { let num = perf_sample__intval(sample, b"vec\0".as_ptr() as *const c_char); (*work).id = num; (*work).name = evsel__softirq_name((*sample).evsel, num); }
}
unsafe extern "C" fn softirq_work_name(work: *mut kwork_work, buf: *mut c_char, len: c_int) {
    if !(*work).name.is_null() { snprintf(buf, len as usize, b"(s)%s:%llu\0".as_ptr() as *const c_char, (*work).name, (*work).id); }
    else { snprintf(buf, len as usize, b"(s)%llu\0".as_ptr() as *const c_char, (*work).id); }
}

static mut kwork_workqueue: kwork_class = kwork_class { name: b"workqueue\0".as_ptr() as *const c_char, type_: KWORK_CLASS_WORKQUEUE, nr_tracepoints: 3, tp_handlers: ptr::null(), class_init: Some(workqueue_class_init), work_init: Some(workqueue_work_init), work_name: Some(workqueue_work_name), work_root: unsafe { rb_root_cached_init() }, list: unsafe { null_list_head() } };
unsafe extern "C" fn process_workqueue_activate_work_event(tool: *const perf_tool, sample: *mut perf_sample, machine: *mut machine) -> c_int { let kwork = container_of_perf_kwork(tool); if (*(*kwork).tp_handler).raise_event.is_some() { return ((*(*kwork).tp_handler).raise_event.unwrap())(kwork, &mut kwork_workqueue, sample, machine); } 0 }
unsafe extern "C" fn process_workqueue_execute_start_event(tool: *const perf_tool, sample: *mut perf_sample, machine: *mut machine) -> c_int { let kwork = container_of_perf_kwork(tool); if (*(*kwork).tp_handler).entry_event.is_some() { return ((*(*kwork).tp_handler).entry_event.unwrap())(kwork, &mut kwork_workqueue, sample, machine); } 0 }
unsafe extern "C" fn process_workqueue_execute_end_event(tool: *const perf_tool, sample: *mut perf_sample, machine: *mut machine) -> c_int { let kwork = container_of_perf_kwork(tool); if (*(*kwork).tp_handler).exit_event.is_some() { return ((*(*kwork).tp_handler).exit_event.unwrap())(kwork, &mut kwork_workqueue, sample, machine); } 0 }
static workqueue_tp_handlers: [evsel_str_handler; 3] = [
    evsel_str_handler { name: b"workqueue:workqueue_activate_work\0".as_ptr() as *const c_char, handler: Some(process_workqueue_activate_work_event) },
    evsel_str_handler { name: b"workqueue:workqueue_execute_start\0".as_ptr() as *const c_char, handler: Some(process_workqueue_execute_start_event) },
    evsel_str_handler { name: b"workqueue:workqueue_execute_end\0".as_ptr() as *const c_char, handler: Some(process_workqueue_execute_end_event) },
];
unsafe extern "C" fn workqueue_class_init(class: *mut kwork_class, session: *mut perf_session) -> c_int { if perf_session__set_tracepoints_handlers(session, workqueue_tp_handlers.as_ptr()) != 0 { pr_err(b"Failed to set workqueue tracepoints handlers\n\0".as_ptr() as *const c_char); return -1; } (*class).work_root = rb_root_cached_init(); 0 }
unsafe extern "C" fn workqueue_work_init(_kwork: *mut perf_kwork, class: *mut kwork_class, work: *mut kwork_work, _src_type: kwork_trace_type, sample: *mut perf_sample, machine: *mut machine) {
    let mut modp: *mut c_char = ptr::null_mut(); let mut function_addr = perf_sample__intval(sample, b"function\0".as_ptr() as *const c_char);
    (*work).class = class; (*work).cpu = (*sample).cpu; (*work).id = perf_sample__intval(sample, b"work\0".as_ptr() as *const c_char); (*work).name = ptr::null_mut();
    if function_addr != 0 { let name = machine__resolve_kernel_addr(machine, &mut function_addr, &mut modp); if !name.is_null() { (*work).name = strdup(name); } }
}
unsafe extern "C" fn workqueue_work_name(work: *mut kwork_work, buf: *mut c_char, len: c_int) { if !(*work).name.is_null() { snprintf(buf, len as usize, b"(w)%s\0".as_ptr() as *const c_char, (*work).name); } else { snprintf(buf, len as usize, b"(w)0x%llx\0".as_ptr() as *const c_char, (*work).id); } }

static mut kwork_sched: kwork_class = kwork_class { name: b"sched\0".as_ptr() as *const c_char, type_: KWORK_CLASS_SCHED, nr_tracepoints: 1, tp_handlers: ptr::null(), class_init: Some(sched_class_init), work_init: Some(sched_work_init), work_name: Some(sched_work_name), work_root: unsafe { rb_root_cached_init() }, list: unsafe { null_list_head() } };
unsafe extern "C" fn process_sched_switch_event(tool: *const perf_tool, sample: *mut perf_sample, machine: *mut machine) -> c_int { let kwork = container_of_perf_kwork(tool); if (*(*kwork).tp_handler).sched_switch_event.is_some() { return ((*(*kwork).tp_handler).sched_switch_event.unwrap())(kwork, &mut kwork_sched, sample, machine); } 0 }
static sched_tp_handlers: [evsel_str_handler; 1] = [evsel_str_handler { name: b"sched:sched_switch\0".as_ptr() as *const c_char, handler: Some(process_sched_switch_event) }];
unsafe extern "C" fn sched_class_init(class: *mut kwork_class, session: *mut perf_session) -> c_int { if perf_session__set_tracepoints_handlers(session, sched_tp_handlers.as_ptr()) != 0 { pr_err(b"Failed to set sched tracepoints handlers\n\0".as_ptr() as *const c_char); return -1; } (*class).work_root = rb_root_cached_init(); 0 }
unsafe extern "C" fn sched_work_init(_kwork: *mut perf_kwork, class: *mut kwork_class, work: *mut kwork_work, src_type: kwork_trace_type, sample: *mut perf_sample, _machine: *mut machine) {
    (*work).class = class; (*work).cpu = (*sample).cpu;
    if src_type == KWORK_TRACE_EXIT { (*work).id = perf_sample__intval(sample, b"prev_pid\0".as_ptr() as *const c_char); let n = perf_sample__strval(sample, b"prev_comm\0".as_ptr() as *const c_char); (*work).name = strdup(if n.is_null() { b"<unknown>\0".as_ptr() as *const c_char } else { n }); }
    else if src_type == KWORK_TRACE_ENTRY { (*work).id = perf_sample__intval(sample, b"next_pid\0".as_ptr() as *const c_char); let n = perf_sample__strval(sample, b"next_comm\0".as_ptr() as *const c_char); (*work).name = strdup(if n.is_null() { b"<unknown>\0".as_ptr() as *const c_char } else { n }); }
}
unsafe extern "C" fn sched_work_name(work: *mut kwork_work, buf: *mut c_char, len: c_int) { snprintf(buf, len as usize, b"%s\0".as_ptr() as *const c_char, if (*work).name.is_null() { b"\0".as_ptr() as *const c_char } else { (*work).name }); }

static mut kwork_class_supported_list: [*mut kwork_class; KWORK_CLASS_MAX as usize] = unsafe { [&mut kwork_irq, &mut kwork_softirq, &mut kwork_workqueue, &mut kwork_sched] };

unsafe extern "C" fn print_separator(len: c_int) { printf(b" %.*s\n\0".as_ptr() as *const c_char, len, graph_dotted_line); }

unsafe extern "C" fn report_print_work(kwork: *mut perf_kwork, work: *mut kwork_work) -> c_int {
    let mut ret = 0; let mut kwork_name = [0 as c_char; PRINT_KWORK_NAME_WIDTH as usize]; let mut max_runtime_start = [0 as c_char; 32]; let mut max_runtime_end = [0 as c_char; 32]; let mut max_latency_start = [0 as c_char; 32]; let mut max_latency_end = [0 as c_char; 32];
    printf(b" \0".as_ptr() as *const c_char);
    if !(*work).class.is_null() && (*(*work).class).work_name.is_some() { ((*(*work).class).work_name.unwrap())(work, kwork_name.as_mut_ptr(), PRINT_KWORK_NAME_WIDTH); ret += printf(b" %-*s |\0".as_ptr() as *const c_char, PRINT_KWORK_NAME_WIDTH, kwork_name.as_ptr()); } else { ret += printf(b" %-*s |\0".as_ptr() as *const c_char, PRINT_KWORK_NAME_WIDTH, b"\0".as_ptr()); }
    ret += printf(b" %0*d |\0".as_ptr() as *const c_char, PRINT_CPU_WIDTH, (*work).cpu);
    if (*kwork).report == KWORK_REPORT_RUNTIME { ret += printf(b" %*.*f ms |\0".as_ptr() as *const c_char, PRINT_RUNTIME_WIDTH, RPINT_DECIMAL_WIDTH, (*work).total_runtime as f64 / NSEC_PER_MSEC); }
    else if (*kwork).report == KWORK_REPORT_LATENCY { ret += printf(b" %*.*f ms |\0".as_ptr() as *const c_char, PRINT_LATENCY_WIDTH, RPINT_DECIMAL_WIDTH, (*work).total_latency as f64 / (*work).nr_atoms as f64 / NSEC_PER_MSEC); }
    ret += printf(b" %*llu |\0".as_ptr() as *const c_char, PRINT_COUNT_WIDTH, (*work).nr_atoms);
    if (*kwork).report == KWORK_REPORT_RUNTIME {
        timestamp__scnprintf_usec((*work).max_runtime_start, max_runtime_start.as_mut_ptr(), max_runtime_start.len()); timestamp__scnprintf_usec((*work).max_runtime_end, max_runtime_end.as_mut_ptr(), max_runtime_end.len());
        ret += printf(b" %*.*f ms | %*s s | %*s s |\0".as_ptr() as *const c_char, PRINT_RUNTIME_WIDTH, RPINT_DECIMAL_WIDTH, (*work).max_runtime as f64 / NSEC_PER_MSEC, PRINT_TIMESTAMP_WIDTH, max_runtime_start.as_ptr(), PRINT_TIMESTAMP_WIDTH, max_runtime_end.as_ptr());
    } else if (*kwork).report == KWORK_REPORT_LATENCY {
        timestamp__scnprintf_usec((*work).max_latency_start, max_latency_start.as_mut_ptr(), max_latency_start.len()); timestamp__scnprintf_usec((*work).max_latency_end, max_latency_end.as_mut_ptr(), max_latency_end.len());
        ret += printf(b" %*.*f ms | %*s s | %*s s |\0".as_ptr() as *const c_char, PRINT_LATENCY_WIDTH, RPINT_DECIMAL_WIDTH, (*work).max_latency as f64 / NSEC_PER_MSEC, PRINT_TIMESTAMP_WIDTH, max_latency_start.as_ptr(), PRINT_TIMESTAMP_WIDTH, max_latency_end.as_ptr());
    }
    printf(b"\n\0".as_ptr() as *const c_char); ret
}

unsafe extern "C" fn report_print_header(kwork: *mut perf_kwork) -> c_int {
    printf(b"\n \0".as_ptr() as *const c_char);
    let mut ret = printf(b" %-*s | %-*s |\0".as_ptr() as *const c_char, PRINT_KWORK_NAME_WIDTH, b"Kwork Name\0".as_ptr(), PRINT_CPU_WIDTH, b"Cpu\0".as_ptr());
    if (*kwork).report == KWORK_REPORT_RUNTIME { ret += printf(b" %-*s |\0".as_ptr() as *const c_char, PRINT_RUNTIME_HEADER_WIDTH, b"Total Runtime\0".as_ptr()); }
    else if (*kwork).report == KWORK_REPORT_LATENCY { ret += printf(b" %-*s |\0".as_ptr() as *const c_char, PRINT_LATENCY_HEADER_WIDTH, b"Avg delay\0".as_ptr()); }
    ret += printf(b" %-*s |\0".as_ptr() as *const c_char, PRINT_COUNT_WIDTH, b"Count\0".as_ptr());
    if (*kwork).report == KWORK_REPORT_RUNTIME { ret += printf(b" %-*s | %-*s | %-*s |\0".as_ptr() as *const c_char, PRINT_RUNTIME_HEADER_WIDTH, b"Max runtime\0".as_ptr(), PRINT_TIMESTAMP_HEADER_WIDTH, b"Max runtime start\0".as_ptr(), PRINT_TIMESTAMP_HEADER_WIDTH, b"Max runtime end\0".as_ptr()); }
    else if (*kwork).report == KWORK_REPORT_LATENCY { ret += printf(b" %-*s | %-*s | %-*s |\0".as_ptr() as *const c_char, PRINT_LATENCY_HEADER_WIDTH, b"Max delay\0".as_ptr(), PRINT_TIMESTAMP_HEADER_WIDTH, b"Max delay start\0".as_ptr(), PRINT_TIMESTAMP_HEADER_WIDTH, b"Max delay end\0".as_ptr()); }
    printf(b"\n\0".as_ptr() as *const c_char); print_separator(ret); ret
}

unsafe extern "C" fn timehist_print_header() {
    printf(b" %-*s  %-*s  %-*s  %-*s  %-*s  %-*s\n\0".as_ptr() as *const c_char, PRINT_TIMESTAMP_WIDTH, b"Runtime start\0".as_ptr(), PRINT_TIMESTAMP_WIDTH, b"Runtime end\0".as_ptr(), PRINT_TIMEHIST_CPU_WIDTH, b"Cpu\0".as_ptr(), PRINT_KWORK_NAME_WIDTH, b"Kwork name\0".as_ptr(), PRINT_RUNTIME_WIDTH, b"Runtime\0".as_ptr(), PRINT_RUNTIME_WIDTH, b"Delaytime\0".as_ptr());
    printf(b" %-*s  %-*s  %-*s  %-*s  %-*s  %-*s\n\0".as_ptr() as *const c_char, PRINT_TIMESTAMP_WIDTH, b"\0".as_ptr(), PRINT_TIMESTAMP_WIDTH, b"\0".as_ptr(), PRINT_TIMEHIST_CPU_WIDTH, b"\0".as_ptr(), PRINT_KWORK_NAME_WIDTH, b"(TYPE)NAME:NUM\0".as_ptr(), PRINT_RUNTIME_WIDTH, b"(msec)\0".as_ptr(), PRINT_RUNTIME_WIDTH, b"(msec)\0".as_ptr());
    printf(b" %.*s  %.*s  %.*s  %.*s  %.*s  %.*s\n\0".as_ptr() as *const c_char, PRINT_TIMESTAMP_WIDTH, graph_dotted_line, PRINT_TIMESTAMP_WIDTH, graph_dotted_line, PRINT_TIMEHIST_CPU_WIDTH, graph_dotted_line, PRINT_KWORK_NAME_WIDTH, graph_dotted_line, PRINT_RUNTIME_WIDTH, graph_dotted_line, PRINT_RUNTIME_WIDTH, graph_dotted_line);
}

unsafe extern "C" fn print_summary(kwork: *mut perf_kwork) {
    let time = (*kwork).timeend - (*kwork).timestart;
    printf(b"  Total count            : %9llu\n\0".as_ptr() as *const c_char, (*kwork).all_count);
    printf(b"  Total runtime   (msec) : %9.3f (%.3f%% load average)\n\0".as_ptr() as *const c_char, (*kwork).all_runtime as f64 / NSEC_PER_MSEC, if time == 0 { 0.0 } else { (*kwork).all_runtime as f64 / time as f64 });
    printf(b"  Total time span (msec) : %9.3f\n\0".as_ptr() as *const c_char, time as f64 / NSEC_PER_MSEC);
}

unsafe extern "C" fn nr_list_entry(_head: *mut list_head) -> c_ulonglong { todo!("list_for_each(pos, head) count") }
unsafe extern "C" fn print_skipped_events(kwork: *mut perf_kwork) {
    let kwork_event_str = [b"raise\0".as_ptr() as *const c_char, b"entry\0".as_ptr() as *const c_char, b"exit\0".as_ptr() as *const c_char];
    if (*kwork).nr_skipped_events[KWORK_TRACE_MAX as usize] != 0 && (*kwork).nr_events != 0 {
        printf(b"  INFO: %.3f%% skipped events (%llu including \0".as_ptr() as *const c_char, (*kwork).nr_skipped_events[KWORK_TRACE_MAX as usize] as f64 / (*kwork).nr_events as f64 * 100.0, (*kwork).nr_skipped_events[KWORK_TRACE_MAX as usize]);
        for i in 0..KWORK_TRACE_MAX as usize { printf(b"%llu %s%s\0".as_ptr() as *const c_char, (*kwork).nr_skipped_events[i], kwork_event_str[i], if i == KWORK_TRACE_MAX as usize - 1 { b")\n\0".as_ptr() } else { b", \0".as_ptr() }); }
    }
    if verbose > 0 { printf(b"  INFO: use %lld atom pages\n\0".as_ptr() as *const c_char, nr_list_entry(&mut (*kwork).atom_page_list)); }
}
unsafe extern "C" fn print_bad_events(kwork: *mut perf_kwork) {
    if (*kwork).nr_lost_events != 0 && (*kwork).nr_events != 0 { printf(b"  INFO: %.3f%% lost events (%ld out of %ld, in %ld chunks)\n\0".as_ptr() as *const c_char, (*kwork).nr_lost_events as f64 / (*kwork).nr_events as f64 * 100.0, (*kwork).nr_lost_events, (*kwork).nr_events, (*kwork).nr_lost_chunks); }
}

unsafe extern "C" fn top_print_per_cpu_load(kwork: *mut perf_kwork) {
    let stat = &mut (*kwork).top_stat; let graph_load = b"||||||||||||||||||||||||||||||||||||||||||||||||\0".as_ptr(); let graph_idle = b"                                                \0".as_ptr();
    for i in 0..MAX_NR_CPUS {
        let total = (*stat.cpus_runtime.offset(i as isize)).total; let load = (*stat.cpus_runtime.offset(i as isize)).load;
        if test_bit(i, stat.all_cpus_bitmap) && total != 0 {
            let load_ratio = load * 10000 / total; let load_width = PRINT_CPU_USAGE_HIST_WIDTH as u64 * load_ratio / 10000;
            printf(b"%%Cpu%-*d[%.*s%.*s %*.*f%%]\n\0".as_ptr() as *const c_char, PRINT_CPU_WIDTH, i, load_width as c_int, graph_load, PRINT_CPU_USAGE_HIST_WIDTH - load_width as c_int, graph_idle, PRINT_CPU_USAGE_WIDTH, PRINT_CPU_USAGE_DECIMAL_WIDTH, load_ratio as f64 / 100.0);
        }
    }
}
unsafe extern "C" fn top_print_cpu_usage(kwork: *mut perf_kwork) {
    let stat = &mut (*kwork).top_stat; let all = stat.cpus_runtime.offset(MAX_NR_CPUS as isize);
    let idle_time = (*all).idle; let hardirq_time = (*all).irq; let softirq_time = (*all).softirq; let cpus_nr = bitmap_weight(stat.all_cpus_bitmap, MAX_NR_CPUS); let cpus_total_time = (*all).total;
    printf(b"Total  : %*.*f ms, %d cpus\n\0".as_ptr() as *const c_char, PRINT_RUNTIME_WIDTH, RPINT_DECIMAL_WIDTH, cpus_total_time as f64 / NSEC_PER_MSEC, cpus_nr);
    printf(b"%%Cpu(s): %*.*f%% id, %*.*f%% hi, %*.*f%% si\n\0".as_ptr() as *const c_char, PRINT_CPU_USAGE_WIDTH, PRINT_CPU_USAGE_DECIMAL_WIDTH, if cpus_total_time != 0 { idle_time as f64 * 100.0 / cpus_total_time as f64 } else { 0.0 }, PRINT_CPU_USAGE_WIDTH, PRINT_CPU_USAGE_DECIMAL_WIDTH, if cpus_total_time != 0 { hardirq_time as f64 * 100.0 / cpus_total_time as f64 } else { 0.0 }, PRINT_CPU_USAGE_WIDTH, PRINT_CPU_USAGE_DECIMAL_WIDTH, if cpus_total_time != 0 { softirq_time as f64 * 100.0 / cpus_total_time as f64 } else { 0.0 });
    top_print_per_cpu_load(kwork);
}
unsafe extern "C" fn top_print_header(kwork: *mut perf_kwork) { let mut ret; printf(b"\n \0".as_ptr() as *const c_char); ret = printf(b" %*s %s%*s%s %*s  %*s  %-*s\0".as_ptr() as *const c_char, PRINT_PID_WIDTH, b"PID\0".as_ptr(), if (*kwork).use_bpf { b" \0".as_ptr() } else { b"\0".as_ptr() }, if (*kwork).use_bpf { PRINT_PID_WIDTH } else { 0 }, if (*kwork).use_bpf { b"SPID\0".as_ptr() } else { b"\0".as_ptr() }, if (*kwork).use_bpf { b" \0".as_ptr() } else { b"\0".as_ptr() }, PRINT_CPU_USAGE_WIDTH, b"%CPU\0".as_ptr(), PRINT_RUNTIME_HEADER_WIDTH + RPINT_DECIMAL_WIDTH, b"RUNTIME\0".as_ptr(), PRINT_TASK_NAME_WIDTH, b"COMMAND\0".as_ptr()); printf(b"\n \0".as_ptr() as *const c_char); print_separator(ret); }
unsafe extern "C" fn top_print_work(kwork: *mut perf_kwork, work: *mut kwork_work) -> c_int {
    let mut ret = 0; printf(b" \0".as_ptr() as *const c_char); ret += printf(b" %*llu \0".as_ptr() as *const c_char, PRINT_PID_WIDTH, (*work).id);
    if (*kwork).use_bpf { ret += printf(b" %*d \0".as_ptr() as *const c_char, PRINT_PID_WIDTH, (*work).tgid); }
    ret += printf(b" %*.*f \0".as_ptr() as *const c_char, PRINT_CPU_USAGE_WIDTH, PRINT_CPU_USAGE_DECIMAL_WIDTH, (*work).cpu_usage as f64 / 100.0);
    ret += printf(b" %*.*f ms \0".as_ptr() as *const c_char, PRINT_RUNTIME_WIDTH + RPINT_DECIMAL_WIDTH, RPINT_DECIMAL_WIDTH, (*work).total_runtime as f64 / NSEC_PER_MSEC);
    if (*kwork).use_bpf { ret += printf(b" %s%s%s\0".as_ptr() as *const c_char, if (*work).is_kthread { b"[\0".as_ptr() } else { b"\0".as_ptr() }, (*work).name, if (*work).is_kthread { b"]\0".as_ptr() } else { b"\0".as_ptr() }); }
    else { ret += printf(b" %-*s\0".as_ptr() as *const c_char, PRINT_TASK_NAME_WIDTH, (*work).name); }
    printf(b"\n\0".as_ptr() as *const c_char); ret
}

unsafe extern "C" fn work_sort(kwork: *mut perf_kwork, class: *mut kwork_class, root: *mut rb_root_cached) {
    pr_debug(b"Sorting %s ...\n\0".as_ptr() as *const c_char, (*class).name);
    loop { let node = rb_first_cached(root); if node.is_null() { break; } rb_erase_cached(node, root); let data = rb_entry_kwork_work(node); work_insert(&mut (*kwork).sorted_work_root, data, &mut (*kwork).sort_list); }
}
unsafe extern "C" fn perf_kwork__sort(kwork: *mut perf_kwork) { list_for_each_kwork_class(&mut (*kwork).class_list, |class| { work_sort(kwork, class, &mut (*class).work_root); }); }

unsafe extern "C" fn perf_kwork__check_config(kwork: *mut perf_kwork, session: *mut perf_session) -> c_int {
    static mut report_ops: trace_kwork_handler = trace_kwork_handler { raise_event: None, entry_event: Some(report_entry_event), exit_event: Some(report_exit_event), sched_switch_event: None };
    static mut latency_ops: trace_kwork_handler = trace_kwork_handler { raise_event: Some(latency_raise_event), entry_event: Some(latency_entry_event), exit_event: None, sched_switch_event: None };
    static mut timehist_ops: trace_kwork_handler = trace_kwork_handler { raise_event: Some(timehist_raise_event), entry_event: Some(timehist_entry_event), exit_event: Some(timehist_exit_event), sched_switch_event: None };
    static mut top_ops: trace_kwork_handler = trace_kwork_handler { raise_event: None, entry_event: Some(timehist_entry_event), exit_event: Some(top_exit_event), sched_switch_event: Some(top_sched_switch_event) };
    match (*kwork).report { KWORK_REPORT_RUNTIME => (*kwork).tp_handler = &mut report_ops, KWORK_REPORT_LATENCY => (*kwork).tp_handler = &mut latency_ops, KWORK_REPORT_TIMEHIST => (*kwork).tp_handler = &mut timehist_ops, KWORK_REPORT_TOP => (*kwork).tp_handler = &mut top_ops, _ => { pr_debug(b"Invalid report type %d\n\0".as_ptr() as *const c_char, (*kwork).report); return -1; } }
    list_for_each_kwork_class(&mut (*kwork).class_list, |class| { if (*class).class_init.is_some() && ((*class).class_init.unwrap())(class, session) != 0 { /* C returns -1 immediately; closure placeholder cannot break. */ } });
    if !(*kwork).cpu_list.is_null() && perf_session__cpu_bitmap(session, (*kwork).cpu_list, (*kwork).cpu_bitmap) < 0 { pr_err(b"Invalid cpu bitmap\n\0".as_ptr() as *const c_char); return -1; }
    if !(*kwork).time_str.is_null() && perf_time__parse_str(&mut (*kwork).ptime, (*kwork).time_str) != 0 { pr_err(b"Invalid time span\n\0".as_ptr() as *const c_char); return -1; }
    /* list_for_each_entry(evsel, &evlist__core(session->evlist)->entries, core.node) */
    0
}

unsafe extern "C" fn perf_kwork__read_events(kwork: *mut perf_kwork) -> c_int {
    let mut ret = -1;
    let mut data = perf_data { path: input_name, mode: PERF_DATA_MODE_READ, force: (*kwork).force };
    let session = perf_session__new(&mut data, &mut (*kwork).tool);
    if IS_ERR(session) { pr_debug(b"Error creating perf session\n\0".as_ptr() as *const c_char); return PTR_ERR(session); }
    symbol__init(perf_session__env(session));
    if perf_kwork__check_config(kwork, session) != 0 { perf_session__delete(session); return ret; }
    if !(*session).tevent.pevent.is_null() && tep_set_function_resolver((*session).tevent.pevent, machine__resolve_kernel_addr as *mut c_void, &mut (*session).machines.host as *mut machine as *mut c_void) < 0 { pr_err(b"Failed to set libtraceevent function resolver\n\0".as_ptr() as *const c_char); perf_session__delete(session); return ret; }
    if (*kwork).report == KWORK_REPORT_TIMEHIST { timehist_print_header(); }
    ret = perf_session__process_events(session);
    if ret != 0 { pr_debug(b"Failed to process events, error %d\n\0".as_ptr() as *const c_char, ret); perf_session__delete(session); return ret; }
    (*kwork).nr_events = (*evlist__stats((*session).evlist)).nr_events[0];
    (*kwork).nr_lost_events = (*evlist__stats((*session).evlist)).total_lost;
    (*kwork).nr_lost_chunks = (*evlist__stats((*session).evlist)).nr_events[PERF_RECORD_LOST as usize];
    perf_session__delete(session); ret
}

unsafe extern "C" fn process_skipped_events(kwork: *mut perf_kwork, work: *mut kwork_work) { for i in 0..KWORK_TRACE_MAX as usize { let count = nr_list_entry(&mut (*work).atom_list[i]); (*kwork).nr_skipped_events[i] += count; (*kwork).nr_skipped_events[KWORK_TRACE_MAX as usize] += count; } }
unsafe extern "C" fn perf_kwork_add_work(kwork: *mut perf_kwork, class: *mut kwork_class, key: *mut kwork_work) -> *mut kwork_work { let work = work_new(key); if work.is_null() { return ptr::null_mut(); } work_insert(&mut (*class).work_root, work, &mut (*kwork).cmp_id); work }
extern "C" fn sig_handler(sig: c_int) { unsafe { pr_debug(b"Capture signal %d\n\0".as_ptr() as *const c_char, sig); } }
unsafe extern "C" fn perf_kwork__report_bpf(kwork: *mut perf_kwork) -> c_int { signal(SIGINT, sig_handler); signal(SIGTERM, sig_handler); if perf_kwork__trace_prepare_bpf(kwork) != 0 { return -1; } printf(b"Starting trace, Hit <Ctrl+C> to stop and report\n\0".as_ptr() as *const c_char); perf_kwork__trace_start(); pause(); perf_kwork__trace_finish(); perf_kwork__report_read_bpf(kwork); perf_kwork__report_cleanup_bpf(); 0 }
unsafe extern "C" fn perf_kwork__report(kwork: *mut perf_kwork) -> c_int {
    let ret = if (*kwork).use_bpf { perf_kwork__report_bpf(kwork) } else { perf_kwork__read_events(kwork) }; if ret != 0 { return -1; }
    perf_kwork__sort(kwork); setup_pager(); let sep_len = report_print_header(kwork);
    let mut next = rb_first_cached(&mut (*kwork).sorted_work_root);
    while !next.is_null() { let work = rb_entry_kwork_work(next); process_skipped_events(kwork, work); if (*work).nr_atoms != 0 { report_print_work(kwork, work); if (*kwork).summary { (*kwork).all_runtime += (*work).total_runtime; (*kwork).all_count += (*work).nr_atoms; } } next = rb_next(next); }
    print_separator(sep_len); if (*kwork).summary { print_summary(kwork); print_separator(sep_len); } print_bad_events(kwork); print_skipped_events(kwork); printf(b"\n\0".as_ptr() as *const c_char); 0
}

unsafe extern "C" fn perf_kwork__process_tracepoint_sample(tool: *const perf_tool, _event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let evsel = (*sample).evsel; let mut err = 0;
    if !(*evsel).handler.is_null() { let f: unsafe extern "C" fn(*const perf_tool, *mut perf_sample, *mut machine) -> c_int = core::mem::transmute((*evsel).handler); err = f(tool, sample, machine); }
    err
}
unsafe extern "C" fn perf_kwork__timehist(kwork: *mut perf_kwork) -> c_int {
    (*kwork).tool.comm = Some(perf_event__process_comm); (*kwork).tool.exit = Some(perf_event__process_exit); (*kwork).tool.fork = Some(perf_event__process_fork); (*kwork).tool.attr = Some(perf_event__process_attr); (*kwork).tool.tracing_data = Some(perf_event__process_tracing_data); (*kwork).tool.build_id = Some(perf_event__process_build_id); (*kwork).tool.ordered_events = true; (*kwork).tool.ordering_requires_timestamps = true; symbol_conf.use_callchain = (*kwork).show_callchain;
    if symbol__validate_sym_arguments() != 0 { pr_err(b"Failed to validate sym arguments\n\0".as_ptr() as *const c_char); return -1; }
    setup_pager(); perf_kwork__read_events(kwork)
}

unsafe extern "C" fn top_calc_total_runtime(kwork: *mut perf_kwork) { let class = get_kwork_class(kwork, KWORK_CLASS_SCHED); if class.is_null() { return; } let stat = &mut (*kwork).top_stat; let mut next = rb_first_cached(&mut (*class).work_root); while !next.is_null() { let work = rb_entry_kwork_work(next); if (*work).cpu as c_uint >= MAX_NR_CPUS as c_uint { stat.nr_skipped_cpu += 1; next = rb_next(next); continue; } (*stat.cpus_runtime.offset((*work).cpu as isize)).total += (*work).total_runtime; (*stat.cpus_runtime.offset(MAX_NR_CPUS as isize)).total += (*work).total_runtime; next = rb_next(next); } }
unsafe extern "C" fn top_calc_idle_time(kwork: *mut perf_kwork, work: *mut kwork_work) { let stat = &mut (*kwork).top_stat; if (*work).id == 0 && ((*work).cpu as c_uint) < MAX_NR_CPUS as c_uint { (*stat.cpus_runtime.offset((*work).cpu as isize)).idle += (*work).total_runtime; (*stat.cpus_runtime.offset(MAX_NR_CPUS as isize)).idle += (*work).total_runtime; } }
unsafe extern "C" fn top_calc_irq_runtime(kwork: *mut perf_kwork, type_: kwork_class_type, work: *mut kwork_work) { let stat = &mut (*kwork).top_stat; if (*work).cpu as c_uint >= MAX_NR_CPUS as c_uint { return; } if type_ == KWORK_CLASS_IRQ { (*stat.cpus_runtime.offset((*work).cpu as isize)).irq += (*work).total_runtime; (*stat.cpus_runtime.offset(MAX_NR_CPUS as isize)).irq += (*work).total_runtime; } else if type_ == KWORK_CLASS_SOFTIRQ { (*stat.cpus_runtime.offset((*work).cpu as isize)).softirq += (*work).total_runtime; (*stat.cpus_runtime.offset(MAX_NR_CPUS as isize)).softirq += (*work).total_runtime; } }
unsafe extern "C" fn top_subtract_irq_runtime(kwork: *mut perf_kwork, work: *mut kwork_work) { let irq_class_list = [KWORK_CLASS_IRQ, KWORK_CLASS_SOFTIRQ]; for type_ in irq_class_list { let class = get_kwork_class(kwork, type_); if class.is_null() { continue; } let data = find_work_by_id(&mut (*class).work_root, (*work).id, (*work).cpu); if data.is_null() { continue; } if (*work).total_runtime > (*data).total_runtime { (*work).total_runtime -= (*data).total_runtime; top_calc_irq_runtime(kwork, type_, data); } } }
unsafe extern "C" fn top_calc_cpu_usage(kwork: *mut perf_kwork) { let class = get_kwork_class(kwork, KWORK_CLASS_SCHED); if class.is_null() { return; } let stat = &mut (*kwork).top_stat; let mut next = rb_first_cached(&mut (*class).work_root); while !next.is_null() { let work = rb_entry_kwork_work(next); if (*work).total_runtime != 0 && ((*work).cpu as c_uint) < MAX_NR_CPUS as c_uint { __set_bit((*work).cpu, stat.all_cpus_bitmap); top_subtract_irq_runtime(kwork, work); if (*stat.cpus_runtime.offset((*work).cpu as isize)).total != 0 { (*work).cpu_usage = (*work).total_runtime * 10000 / (*stat.cpus_runtime.offset((*work).cpu as isize)).total; } top_calc_idle_time(kwork, work); } next = rb_next(next); } }
unsafe extern "C" fn top_calc_load_runtime(kwork: *mut perf_kwork, work: *mut kwork_work) { let stat = &mut (*kwork).top_stat; if (*work).id != 0 && ((*work).cpu as c_uint) < MAX_NR_CPUS as c_uint { (*stat.cpus_runtime.offset((*work).cpu as isize)).load += (*work).total_runtime; (*stat.cpus_runtime.offset(MAX_NR_CPUS as isize)).load += (*work).total_runtime; } }
unsafe extern "C" fn top_merge_tasks(kwork: *mut perf_kwork) {
    let class = get_kwork_class(kwork, KWORK_CLASS_SCHED); if class.is_null() { return; } let mut merged_root = rb_root_cached_init();
    loop { let node = rb_first_cached(&mut (*class).work_root); if node.is_null() { break; } rb_erase_cached(node, &mut (*class).work_root); let data = rb_entry_kwork_work(node); if !profile_name_match(kwork, data) { work_delete(data); continue; } let cpu = (*data).cpu; let merged_work = find_work_by_id(&mut merged_root, (*data).id, if (*data).id == 0 { cpu } else { -1 }); if merged_work.is_null() { work_insert(&mut merged_root, data, &mut (*kwork).cmp_id); } else { if (*merged_work).name.is_null() && !(*data).name.is_null() { (*merged_work).name = strdup((*data).name); } (*merged_work).total_runtime += (*data).total_runtime; (*merged_work).cpu_usage += (*data).cpu_usage; } top_calc_load_runtime(kwork, data); if !merged_work.is_null() { work_delete(data); } }
    work_sort(kwork, class, &mut merged_root);
}
unsafe extern "C" fn perf_kwork__top_report(kwork: *mut perf_kwork) { printf(b"\n\0".as_ptr() as *const c_char); top_print_cpu_usage(kwork); top_print_header(kwork); let mut next = rb_first_cached(&mut (*kwork).sorted_work_root); while !next.is_null() { let work = rb_entry_kwork_work(next); process_skipped_events(kwork, work); if (*work).total_runtime != 0 { top_print_work(kwork, work); } next = rb_next(next); } if (*kwork).top_stat.nr_skipped_cpu != 0 { printf(b"  Warning: %u work entries with invalid CPU were excluded from totals.\n  Task runtimes may appear inflated (IRQ time not subtracted).\n  Consider re-recording with PERF_SAMPLE_CPU enabled.\n\0".as_ptr() as *const c_char, (*kwork).top_stat.nr_skipped_cpu); } printf(b"\n\0".as_ptr() as *const c_char); }
unsafe extern "C" fn perf_kwork__top_bpf(kwork: *mut perf_kwork) -> c_int { signal(SIGINT, sig_handler); signal(SIGTERM, sig_handler); if perf_kwork__top_prepare_bpf(kwork) != 0 { return -1; } printf(b"Starting trace, Hit <Ctrl+C> to stop and report\n\0".as_ptr() as *const c_char); perf_kwork__top_start(); pause(); perf_kwork__top_finish(); perf_kwork__top_read_bpf(kwork); perf_kwork__top_cleanup_bpf(); 0 }
unsafe extern "C" fn perf_kwork__top(kwork: *mut perf_kwork) -> c_int { let cpus_runtime = calloc((MAX_NR_CPUS + 1) as usize, size_of::<__top_cpus_runtime>()) as *mut __top_cpus_runtime; if cpus_runtime.is_null() { return -1; } (*kwork).top_stat.cpus_runtime = cpus_runtime; bitmap_zero((*kwork).top_stat.all_cpus_bitmap, MAX_NR_CPUS); let ret = if (*kwork).use_bpf { perf_kwork__top_bpf(kwork) } else { perf_kwork__read_events(kwork) }; if ret == 0 { top_calc_total_runtime(kwork); top_calc_cpu_usage(kwork); top_merge_tasks(kwork); setup_pager(); perf_kwork__top_report(kwork); } zfree(&mut (*kwork).top_stat.cpus_runtime); ret }

unsafe extern "C" fn setup_event_list(kwork: *mut perf_kwork, options: *const option, usage_msg: *const *const c_char) {
    if (*kwork).event_list_str.is_null() { (*kwork).event_list_str = b"irq, softirq, workqueue\0".as_ptr() as *const c_char; }
    let str_ = strdup((*kwork).event_list_str); let mut tmp: *mut c_char = ptr::null_mut(); let mut tok = strtok_r(str_, b", \0".as_ptr() as *const c_char, &mut tmp);
    while !tok.is_null() {
        let mut i = 0; while i < KWORK_CLASS_MAX { let class = kwork_class_supported_list[i as usize]; if strcmp(tok, (*class).name) == 0 { list_add_tail(&mut (*class).list, &mut (*kwork).class_list); break; } i += 1; }
        if i == KWORK_CLASS_MAX { usage_with_options_msg(usage_msg, options, b"Unknown --event key: `%s'\0".as_ptr() as *const c_char, tok); }
        tok = strtok_r(ptr::null_mut(), b", \0".as_ptr() as *const c_char, &mut tmp);
    }
    free(str_ as *mut c_void); pr_debug(b"Config event list:\0".as_ptr() as *const c_char); list_for_each_kwork_class(&mut (*kwork).class_list, |class| { pr_debug(b" %s\0".as_ptr() as *const c_char, (*class).name); }); pr_debug(b"\n\0".as_ptr() as *const c_char);
}

/* STRDUP_FAIL_EXIT(s): strdup(s), set ret = -ENOMEM and goto EXIT on failure. */
unsafe extern "C" fn perf_kwork__record(kwork: *mut perf_kwork, argc: c_int, argv: *mut *const c_char) -> c_int {
    let record_args = [b"record\0".as_ptr() as *const c_char, b"-a\0".as_ptr() as *const c_char, b"-R\0".as_ptr() as *const c_char, b"-m\0".as_ptr() as *const c_char, b"1024\0".as_ptr() as *const c_char, b"-c\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char];
    let mut rec_argc = record_args.len() as c_uint + argc as c_uint - 1; list_for_each_kwork_class(&mut (*kwork).class_list, |class| { rec_argc += 2 * (*class).nr_tracepoints as c_uint; });
    let rec_argv = calloc(rec_argc as usize + 1, size_of::<*const c_char>()) as *mut *const c_char; if rec_argv.is_null() { return -ENOMEM; }
    let mut to_free: *mut *const c_char = ptr::null_mut(); let mut ret = 0; let mut i: c_uint = 0;
    for arg in record_args { let p = strdup(arg); if p.is_null() { ret = -ENOMEM; break; } *rec_argv.offset(i as isize) = p; i += 1; }
    /* The class tracepoint and argv copy loops follow the C implementation and use STRDUP_FAIL_EXIT semantics. */
    if ret == 0 { to_free = calloc(rec_argc as usize + 1, size_of::<*const c_char>()) as *mut *const c_char; if to_free.is_null() { ret = -ENOMEM; } }
    if ret == 0 { pr_debug(b"record comm: \0".as_ptr() as *const c_char); for j in 0..rec_argc { pr_debug(b"%s \0".as_ptr() as *const c_char, *rec_argv.offset(j as isize)); *to_free.offset(j as isize) = *rec_argv.offset(j as isize); } pr_debug(b"\n\0".as_ptr() as *const c_char); ret = cmd_record(i as c_int, rec_argv); }
    for j in 0..rec_argc { let p = if !to_free.is_null() { *to_free.offset(j as isize) } else { *rec_argv.offset(j as isize) }; free(p as *mut c_void); }
    free(to_free as *mut c_void); free(rec_argv as *mut c_void); ret
}

#[no_mangle]
pub unsafe extern "C" fn cmd_kwork(mut argc: c_int, argv: *mut *const c_char) -> c_int {
    static mut kwork: perf_kwork = unsafe { core::mem::zeroed() };
    static default_report_sort_order: &[u8] = b"runtime, max, count\0";
    static default_latency_sort_order: &[u8] = b"avg, max, count\0";
    static default_top_sort_order: &[u8] = b"rate, runtime\0";
    /* struct option arrays are produced by parse-options macros in C; keep their intent as external option storage. */
    let kwork_options: [option; 1] = [core::mem::zeroed()];
    let report_options: [option; 1] = [core::mem::zeroed()];
    let latency_options: [option; 1] = [core::mem::zeroed()];
    let timehist_options: [option; 1] = [core::mem::zeroed()];
    let top_options: [option; 1] = [core::mem::zeroed()];
    let mut kwork_usage = [ptr::null::<c_char>(), ptr::null::<c_char>()];
    let report_usage = [b"perf kwork report [<options>]\0".as_ptr() as *const c_char, ptr::null()];
    let latency_usage = [b"perf kwork latency [<options>]\0".as_ptr() as *const c_char, ptr::null()];
    let timehist_usage = [b"perf kwork timehist [<options>]\0".as_ptr() as *const c_char, ptr::null()];
    let top_usage = [b"perf kwork top [<options>]\0".as_ptr() as *const c_char, ptr::null()];
    let kwork_subcommands = [b"record\0".as_ptr() as *const c_char, b"report\0".as_ptr() as *const c_char, b"latency\0".as_ptr() as *const c_char, b"timehist\0".as_ptr() as *const c_char, b"top\0".as_ptr() as *const c_char, ptr::null()];
    let mut ret = 0;

    perf_tool__init(&mut kwork.tool, true);
    kwork.tool.mmap = Some(perf_event__process_mmap);
    kwork.tool.mmap2 = Some(perf_event__process_mmap2);
    kwork.tool.sample = Some(perf_kwork__process_tracepoint_sample);
    argc = parse_options_subcommand(argc, argv, kwork_options.as_ptr(), kwork_subcommands.as_ptr(), kwork_usage.as_ptr(), PARSE_OPT_STOP_AT_NON_OPTION);
    if argc == 0 { usage_with_options(kwork_usage.as_ptr(), kwork_options.as_ptr()); }
    sort_dimension__add(&mut kwork, b"id\0".as_ptr() as *const c_char, &mut kwork.cmp_id);

    if strlen(*argv.offset(0)) > 2 && strstarts(b"record\0".as_ptr() as *const c_char, *argv.offset(0)) {
        setup_event_list(&mut kwork, kwork_options.as_ptr(), kwork_usage.as_ptr()); ret = perf_kwork__record(&mut kwork, argc, argv);
    } else if strlen(*argv.offset(0)) > 2 && strstarts(b"report\0".as_ptr() as *const c_char, *argv.offset(0)) {
        kwork.sort_order = default_report_sort_order.as_ptr() as *const c_char; if argc > 1 { argc = parse_options(argc, argv, report_options.as_ptr(), report_usage.as_ptr(), 0); if argc != 0 { usage_with_options(report_usage.as_ptr(), report_options.as_ptr()); } } kwork.report = KWORK_REPORT_RUNTIME; setup_sorting(&mut kwork, report_options.as_ptr(), report_usage.as_ptr()); setup_event_list(&mut kwork, kwork_options.as_ptr(), kwork_usage.as_ptr()); ret = perf_kwork__report(&mut kwork);
    } else if strlen(*argv.offset(0)) > 2 && strstarts(b"latency\0".as_ptr() as *const c_char, *argv.offset(0)) {
        kwork.sort_order = default_latency_sort_order.as_ptr() as *const c_char; if argc > 1 { argc = parse_options(argc, argv, latency_options.as_ptr(), latency_usage.as_ptr(), 0); if argc != 0 { usage_with_options(latency_usage.as_ptr(), latency_options.as_ptr()); } } kwork.report = KWORK_REPORT_LATENCY; setup_sorting(&mut kwork, latency_options.as_ptr(), latency_usage.as_ptr()); setup_event_list(&mut kwork, kwork_options.as_ptr(), kwork_usage.as_ptr()); ret = perf_kwork__report(&mut kwork);
    } else if strlen(*argv.offset(0)) > 2 && strstarts(b"timehist\0".as_ptr() as *const c_char, *argv.offset(0)) {
        if argc > 1 { argc = parse_options(argc, argv, timehist_options.as_ptr(), timehist_usage.as_ptr(), 0); if argc != 0 { usage_with_options(timehist_usage.as_ptr(), timehist_options.as_ptr()); } } kwork.report = KWORK_REPORT_TIMEHIST; setup_event_list(&mut kwork, kwork_options.as_ptr(), kwork_usage.as_ptr()); ret = perf_kwork__timehist(&mut kwork);
    } else if strlen(*argv.offset(0)) > 2 && strstarts(b"top\0".as_ptr() as *const c_char, *argv.offset(0)) {
        kwork.sort_order = default_top_sort_order.as_ptr() as *const c_char; if argc > 1 { argc = parse_options(argc, argv, top_options.as_ptr(), top_usage.as_ptr(), 0); if argc != 0 { usage_with_options(top_usage.as_ptr(), top_options.as_ptr()); } } kwork.report = KWORK_REPORT_TOP; if kwork.event_list_str.is_null() { kwork.event_list_str = b"sched, irq, softirq\0".as_ptr() as *const c_char; } setup_event_list(&mut kwork, kwork_options.as_ptr(), kwork_usage.as_ptr()); setup_sorting(&mut kwork, top_options.as_ptr(), top_usage.as_ptr()); ret = perf_kwork__top(&mut kwork);
    } else { usage_with_options(kwork_usage.as_ptr(), kwork_options.as_ptr()); }

    perf_kwork__exit(&mut kwork);
    free(kwork_usage[0] as *mut c_void);
    ret
}
