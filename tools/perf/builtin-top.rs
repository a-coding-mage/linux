// SPDX-License-Identifier: GPL-2.0-only
/*
 * builtin-top.rs
 *
 * Rust translation of perf/builtin-top.c.
 *
 * Builtin top command: Display a continuously updated profile of
 * any workload, CPU or specific PID.
 *
 * Copyright (C) 2008, Red Hat Inc, Ingo Molnar <mingo@redhat.com>
 *              2011, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 *
 * Improvements and fixes by:
 *
 *   Arjan van de Ven <arjan@linux.intel.com>
 *   Yanmin Zhang <yanmin.zhang@intel.com>
 *   Wu Fengguang <fengguang.wu@intel.com>
 *   Mike Galbraith <efault@gmx.de>
 *   Paul Mackerras <paulus@samba.org>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(clippy::missing_safety_doc)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut};

type bool_ = bool;
type u64 = u64;
type size_t = usize;
type sig_atomic_t = c_int;
type pthread_t = c_ulong;
type FILE = c_void;

const HEADER_LINE_NR: c_int = 5;
const BUFSIZ: usize = 8192;
const UINT_MAX: c_uint = c_uint::MAX;
const ULLONG_MAX: c_ulonglong = c_ulonglong::MAX;
const MSEC_PER_SEC: c_int = 1000;
const NSEC_PER_SEC: u64 = 1_000_000_000;

const PERF_RECORD_SAMPLE: c_uint = 9;
const PERF_RECORD_LOST: c_uint = 2;
const PERF_RECORD_LOST_SAMPLES: c_uint = 13;
const PERF_RECORD_MAX: c_uint = 64;
const PERF_RECORD_MISC_EXACT_IP: c_uint = 1 << 14;
const PERF_RECORD_MISC_USER: c_uint = 1;
const PERF_RECORD_MISC_KERNEL: c_uint = 2;
const PERF_RECORD_MISC_GUEST_KERNEL: c_uint = 3;
const PERF_RECORD_MISC_GUEST_USER: c_uint = 4;
const PERF_SAMPLE_BRANCH_ANY: u64 = 1;
const PERF_MAX_STACK_DEPTH: c_int = 127;

const DSO_BINARY_TYPE__KALLSYMS: c_int = 1;
const STB_GLOBAL: c_int = 1;
const STB_LOCAL: c_int = 0;
const ERANGE: c_int = 34;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ENOSYS: c_int = 38;
const EINTR: c_int = 4;
const POLLIN: c_short = 0x001;
type c_short = i16;
const SIGWINCH: c_int = 28;
const SIG_DFL: usize = 0;
const SIGSEGV: c_int = 11;
const SIGFPE: c_int = 8;
const SIGINT: c_int = 2;
const SIGQUIT: c_int = 3;
const SIGTERM: c_int = 15;
const TCSAFLUSH: c_int = 2;
const CLONE_FS: c_int = 0x0000_0200;
const PR_SET_NAME: c_int = 15;
const SCHED_FIFO: c_int = 1;
const K_RELOAD: c_int = 1;
const OE_FLUSH__TOP: c_int = 1;
const BKW_MMAP_DATA_PENDING: c_int = 0;
const BKW_MMAP_EMPTY: c_int = 1;
const BKW_MMAP_RUNNING: c_int = 2;
const CALLCHAIN_NONE: c_int = 0;
const CALLCHAIN_FP: c_int = 1;
const CALLCHAIN_LBR: c_int = 2;
const CHAIN_NONE: c_int = 0;
const CCKEY_ADDRESS: c_int = 1;
const ORDER_CALLER: c_int = 1;
const SORT_MODE__TOP: c_int = 1;
const EM_S390: c_int = 22;
const EM_HOST: c_int = 0;
const AGGR_NONE: c_int = 0;
const EVSEL__CONFIG_TERM_OVERWRITE: c_int = 1;
const HAVE_FILE_HANDLE: bool = false;
const HAVE_SLANG_SUPPORT: bool = false;
const HAVE_LIBPFM: bool = false;
const HAVE_LIBBPF_SUPPORT: bool = false;

#[repr(C)] pub struct winsize { pub ws_row: c_int, pub ws_col: c_int }
#[repr(C)] pub struct perf_top {
    pub tool: perf_tool,
    pub evlist: *mut evlist,
    pub sb_evlist: *mut evlist,
    pub session: *mut perf_session,
    pub record_opts: record_opts,
    pub sym_filter_entry: *mut hist_entry,
    pub sym_evsel: *mut evsel,
    pub sym_filter: *const c_char,
    pub winsize: winsize,
    pub print_entries: c_int,
    pub delay_secs: c_int,
    pub count_filter: c_int,
    pub min_percent: f32,
    pub max_stack: c_uint,
    pub nr_threads_synthesize: c_uint,
    pub hide_user_symbols: bool,
    pub hide_kernel_symbols: bool,
    pub zero: bool,
    pub dump_symtab: bool,
    pub use_stdio: bool,
    pub use_tui: bool,
    pub realtime_prio: c_int,
    pub exact_samples: u64,
    pub samples: u64,
    pub us_samples: u64,
    pub kernel_samples: u64,
    pub guest_kernel_samples: u64,
    pub guest_us_samples: u64,
    pub lost: u64,
    pub lost_total: u64,
    pub drop: u64,
    pub drop_total: u64,
    pub vmlinux_warned: bool,
    pub stitch_lbr: bool,
    pub uid_str: *const c_char,
    pub qe: queue_pair,
    pub evswitch: evswitch,
}
#[repr(C)] pub struct queue_pair { pub data: [ordered_events; 2], pub in_: *mut ordered_events, pub rotate: bool, pub mutex: mutex, pub cond: cond }
#[repr(C)] pub struct perf_tool { pub namespace_events: bool, pub cgroup_events: bool }
#[repr(C)] pub struct record_opts { pub mmap_pages: c_uint, pub user_freq: c_uint, pub user_interval: c_ulonglong, pub freq: c_uint, pub target: target, pub overwrite: bool, pub sample_time: bool, pub sample_time_set: bool, pub branch_stack: u64, pub record_namespaces: bool, pub record_cgroup: bool, pub no_bpf_event: bool, pub no_inherit: bool }
#[repr(C)] pub struct target { pub uses_mmap: bool, pub pid: *const c_char, pub tid: *const c_char, pub system_wide: bool, pub cpu_list: *const c_char }
#[repr(C)] pub struct evlist { _private: [u8; 0] }
#[repr(C)] pub struct evsel { pub core: evsel_core, pub config_terms: list_head, pub evlist: *mut evlist, pub filter: *const c_char }
#[repr(C)] pub struct evsel_core { pub idx: c_int, pub cpus: *mut c_void, pub threads: *mut c_void, pub attr: perf_event_attr }
#[repr(C)] pub struct perf_event_attr { pub write_backward: bool }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct evsel_config_term { pub list: list_head, pub type_: c_int, pub val: evsel_config_term_val }
#[repr(C)] pub union evsel_config_term_val { pub overwrite: bool }
#[repr(C)] pub struct hists { pub lock: mutex, pub entries: rb_root_cached, pub uid_filter_str: *const c_char }
#[repr(C)] pub struct hist_entry { pub ms: map_symbol, pub hists: *mut hists, pub rb_node: rb_node }
#[repr(C)] pub struct map_symbol { pub sym: *mut symbol, pub map: *mut map }
#[repr(C)] pub struct symbol { pub name: *mut c_char, pub start: u64, pub end: u64 }
#[repr(C)] pub struct annotation { pub src: *mut c_void }
#[repr(C)] pub struct map { _private: [u8; 0] }
#[repr(C)] pub struct dso { _private: [u8; 0] }
#[repr(C)] pub struct rb_node { _private: [u8; 0] }
#[repr(C)] pub struct rb_root_cached { _private: [u8; 0] }
#[repr(C)] pub struct perf_sample { pub pid: c_int, pub evsel: *mut evsel, pub id: u64, pub cpumode: c_uint, pub branch_stack: *mut c_void }
#[repr(C)] pub struct perf_event_header { pub type_: c_uint, pub misc: c_uint }
#[repr(C)] pub struct perf_event_lost { pub header: perf_event_header, pub lost: u64 }
#[repr(C)] pub struct perf_event_lost_samples { pub header: perf_event_header, pub lost: u64 }
#[repr(C)] pub union perf_event { pub header: perf_event_header, pub lost: perf_event_lost, pub lost_samples: perf_event_lost_samples }
#[repr(C)] pub struct perf_session { pub evlist: *mut evlist, pub machines: machines }
#[repr(C)] pub struct machines { pub host: machine }
#[repr(C)] pub struct machine { pub env: *mut perf_env, pub kptr_restrict_warned: bool }
#[repr(C)] pub struct perf_env { _private: [u8; 0] }
#[repr(C)] pub struct addr_location { pub thread: *mut thread, pub cpumode: c_uint, pub map: *mut map, pub sym: *mut symbol, pub addr: u64 }
#[repr(C)] pub struct thread { _private: [u8; 0] }
#[repr(C)] pub struct hist_entry_iter { pub sample: *mut perf_sample, pub add_entry_cb: Option<unsafe extern "C" fn(*mut hist_entry_iter, *mut addr_location, bool, *mut c_void) -> c_int>, pub ops: *const c_void, pub he: *mut hist_entry }
#[repr(C)] pub struct mmap { pub core: mmap_core }
#[repr(C)] pub struct mmap_core { _private: [u8; 0] }
#[repr(C)] pub struct ordered_event { pub event: *mut perf_event, pub timestamp: u64 }
#[repr(C)] pub struct ordered_events { pub nr_events: u64, pub data: *mut c_void }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct cond { _private: [u8; 0] }
#[repr(C)] pub struct pollfd { pub fd: c_int, pub events: c_short, pub revents: c_short }
#[repr(C)] pub struct termios { _private: [u8; 0] }
#[repr(C)] pub struct utsname { pub sysname: [c_char; 65], pub nodename: [c_char; 65], pub release: [c_char; 65], pub version: [c_char; 65], pub machine: [c_char; 65] }
#[repr(C)] pub struct sched_param { pub sched_priority: c_int }
#[repr(C)] pub struct intlist { _private: [u8; 0] }
#[repr(C)] pub struct option { pub value: *mut c_void }
#[repr(C)] pub struct parse_events_option_args { pub evlistp: *mut *mut evlist }
#[repr(C)] pub struct hist_browser_timer { pub timer: Option<unsafe extern "C" fn(*mut c_void)>, pub arg: *mut c_void, pub refresh: c_int }
#[repr(C)] pub struct callchain_param { pub enabled: bool, pub record_mode: c_int, pub mode: c_int, pub key: c_int, pub branch_callstack: bool, pub order_set: bool, pub order: c_int }
#[repr(C)] pub struct annotation_options { pub min_pcnt: c_int, pub context: c_int, pub objdump_path: *mut c_char, pub disassembler_style: *mut c_char, pub addr2line_path: *mut c_char, pub prefix: *const c_char, pub prefix_strip: *const c_char, pub annotate_src: bool, pub show_asm_raw: bool }
#[repr(C)] pub struct symbol_conf_t { pub event_group: bool, pub use_callchain: bool, pub cumulate_callchain: bool, pub kptr_restrict: bool, pub vmlinux_name: *const c_char, pub ignore_vmlinux: bool, pub kallsyms_name: *const c_char, pub show_nr_samples: bool, pub show_total_period: bool, pub dso_list_str: *const c_char, pub comm_list_str: *const c_char, pub sym_list_str: *const c_char, pub demangle_kernel: bool, pub addr2line_path: *mut c_char, pub raw_trace: bool, pub report_hierarchy: bool, pub force: bool, pub group_sort_idx: c_int, pub show_branchflag_count: bool, pub col_width_list_str: *const c_char, pub try_vmlinux_path: bool }
#[repr(C)] pub struct perf_hpp_list_t { pub sym: bool, pub socket: bool, pub need_collapse: c_int }
#[repr(C)] pub struct stat_config_t { _private: [u8; 0] }
#[repr(C)] pub struct perf_missing_features_t { pub write_backward: bool }
#[repr(C)] pub struct evlist_stats { pub nr_lost_warned: c_int, pub nr_events: [c_int; 256], pub total_lost: u64, pub total_lost_samples: u64, pub nr_unprocessable_samples: c_uint, pub nr_unknown_events: c_uint }
#[repr(C)] pub struct evlist_core { pub nr_mmaps: c_int, pub threads: *mut c_void, pub entries: list_head }
#[repr(C)] pub struct evswitch { _private: [u8; 0] }

unsafe extern "C" {
    static mut session_done: c_int;
    static mut perf_guest: bool;
    static mut use_browser: c_int;
    static mut annotate_opts: annotation_options;
    static mut symbol_conf: symbol_conf_t;
    static mut callchain_param: callchain_param;
    static mut perf_hpp_list: perf_hpp_list_t;
    static mut perf_missing_features: perf_missing_features_t;
    static mut verbose: c_int;
    static mut sort_order: *const c_char;
    static mut field_order: *const c_char;
    static mut stat_config: stat_config_t;
    static mut sort__mode: c_int;
    static mut proc_map_timeout: c_uint;
    static mut nr_cgroups: c_int;
    static perf_version_string: *const c_char;
    static graph_dotted_line: *const c_char;
    static hist_iter_cumulative: c_void;
    static hist_iter_normal: c_void;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut stdin: *mut FILE;
    static mut errno: c_int;

    fn get_term_dimensions(ws: *mut winsize);
    fn hists_to_evsel(hists: *mut hists) -> *mut evsel;
    fn map__dso(map: *mut map) -> *mut dso;
    fn dso__symtab_type(dso: *mut dso) -> c_int;
    fn dso__is_kcore(dso: *mut dso) -> bool;
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn dso__symtab_origin(dso: *mut dso) -> c_int;
    fn dso__strerror_load(dso: *mut dso, buf: *mut c_char, bufsiz: size_t);
    fn symbol__annotation(sym: *mut symbol) -> *mut annotation;
    fn annotation__lock(notes: *mut annotation);
    fn annotation__unlock(notes: *mut annotation);
    fn annotation__trylock(notes: *mut annotation) -> bool;
    fn symbol__hists(sym: *mut symbol, nr: c_int) -> bool;
    fn symbol__annotate(ms: *mut map_symbol, evsel: *mut evsel, arg: *mut c_void) -> c_int;
    fn symbol__strerror_disassemble(ms: *mut map_symbol, err: c_int, msg: *mut c_char, size: size_t);
    fn symbol__annotate_zero_histograms(sym: *mut symbol);
    fn symbol__annotate_zero_histogram(sym: *mut symbol, evsel: *mut evsel);
    fn symbol__annotate_decay_histogram(sym: *mut symbol, evsel: *mut evsel);
    fn symbol__calc_percent(sym: *mut symbol, evsel: *mut evsel);
    fn symbol__binding(sym: *mut symbol) -> c_int;
    fn symbol__is_idle(sym: *mut symbol, dso: *mut dso, env: *mut perf_env) -> bool;
    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn evlist__enabled(evlist: *mut evlist) -> bool;
    fn evlist__stats(evlist: *mut evlist) -> *mut evlist_stats;
    fn evsel__hists(evsel: *mut evsel) -> *mut hists;
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
    fn evsel__is_group_leader(evsel: *mut evsel) -> bool;
    fn evsel__leader(evsel: *mut evsel) -> *mut evsel;
    fn hists__unlink(hists: *mut hists);
    fn hists__delete_entries(hists: *mut hists);
    fn hists__decay_entries(hists: *mut hists, hide_user: bool, hide_kernel: bool);
    fn hists__collapse_resort(hists: *mut hists, arg: *mut c_void);
    fn hists__match(leader: *mut hists, hists: *mut hists);
    fn hists__link(leader: *mut hists, hists: *mut hists);
    fn evsel__output_resort(evsel: *mut evsel, arg: *mut c_void);
    fn perf_top__header_snprintf(top: *mut perf_top, bf: *mut c_char, size: size_t);
    fn hists__output_recalc_col_len(hists: *mut hists, entries: c_int);
    fn hists__fprintf(hists: *mut hists, show_header: bool, entries: c_int, width: c_int, min_percent: f32, fp: *mut FILE, no_callchain: bool);
    fn hist_entry__annotate_printf(he: *mut hist_entry, evsel: *mut evsel) -> c_int;
    fn hist_entry__inc_addr_samples(he: *mut hist_entry, sample: *mut perf_sample, ip: u64) -> c_int;
    fn hist__account_cycles(branch_stack: *mut c_void, al: *mut addr_location, sample: *mut perf_sample, no_branch_any: bool, total_cycles: *mut c_void);
    fn hist_entry_iter__add(iter: *mut hist_entry_iter, al: *mut addr_location, max_stack: c_uint, arg: *mut c_void) -> c_int;
    fn ui__warning(fmt: *const c_char, ...);
    fn ui__error(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug2(fmt: *const c_char, ...);
    fn eprintf(level: c_int, verbose: c_int, fmt: *const c_char, ...);
    fn color_fprintf(fp: *mut FILE, color: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn puts(s: *const c_char) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(fp: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn fflush(fp: *mut FILE) -> c_int;
    fn putchar(c: c_int) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> isize;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strtof(nptr: *const c_char, endptr: *mut *mut c_char) -> f32;
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> *const c_char;
    fn free(ptr: *mut c_void);
    fn sleep(seconds: c_uint) -> c_uint;
    fn usleep(usec: c_uint) -> c_int;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn getc(stream: *mut FILE) -> c_int;
    fn clearerr(stream: *mut FILE);
    fn set_term_quiet_input(save: *mut termios);
    fn tcsetattr(fd: c_int, optional_actions: c_int, termios_p: *const termios) -> c_int;
    fn signal(signum: c_int, handler: usize) -> usize;
    fn sighandler_dump_stack(sig: c_int);
    fn pthread__unblock_sigwinch();
    fn pthread_create(thread: *mut pthread_t, attr: *const c_void, start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn prctl(option: c_int, arg2: *const c_char, arg3: c_ulong, arg4: c_ulong, arg5: c_ulong) -> c_int;
    fn sched_setscheduler(pid: c_int, policy: c_int, param: *const sched_param) -> c_int;
    fn uname(buf: *mut utsname) -> c_int;
    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn mutex_init(m: *mut mutex);
    fn mutex_destroy(m: *mut mutex);
    fn cond_init(c: *mut cond);
    fn cond_destroy(c: *mut cond);
    fn cond_wait(c: *mut cond, m: *mut mutex);
    fn cond_signal(c: *mut cond);
    fn rb_first_cached(root: *mut rb_root_cached) -> *mut rb_node;
    fn rb_next(node: *mut rb_node) -> *mut rb_node;
    fn rb_entry_hist_entry(node: *mut rb_node) -> *mut hist_entry;
    fn map__start(map: *mut map) -> u64;
    fn map__end(map: *mut map) -> u64;
    fn map__set_erange_warned(map: *mut map);
    fn map__erange_warned(map: *mut map) -> bool;
    fn map__has_symbols(map: *mut map) -> bool;
    fn __map__is_kernel(map: *mut map) -> bool;
    fn evlist__for_each_entry_next(evlist: *mut evlist, pos: *mut evsel) -> *mut evsel;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__selected(evlist: *mut evlist) -> *mut evsel;
    fn evlist__tui_browse_hists(evlist: *mut evlist, help: *const c_char, hbt: *mut hist_browser_timer, min_percent: f32, env: *mut perf_env, warn_lost: bool) -> c_int;
    fn perf_session__env(session: *mut perf_session) -> *mut perf_env;
    fn perf_session__fprintf_dsos(session: *mut perf_session, fp: *mut FILE);
    fn perf_session__register_idle_thread(session: *mut perf_session) -> c_int;
    fn perf_session__find_machine(session: *mut perf_session, pid: c_int) -> *mut machine;
    fn perf_session__set_id_hdr_size(session: *mut perf_session);
    fn perf_session__delete(session: *mut perf_session);
    fn __perf_session__new(data: *mut c_void, tool: *mut c_void, trace_event_repipe: bool, env: *mut perf_env) -> *mut perf_session;
    fn IS_ERR(ptr: *mut perf_session) -> bool;
    fn PTR_ERR(ptr: *mut perf_session) -> c_int;
    fn evlist__config(evlist: *mut evlist, opts: *mut record_opts, callchain: *mut callchain_param);
    fn evsel__open(counter: *mut evsel, cpus: *mut c_void, threads: *mut c_void) -> c_int;
    fn evsel__fallback(counter: *mut evsel, target: *mut target, err: c_int, msg: *mut c_char, size: size_t) -> bool;
    fn evsel__open_strerror(counter: *mut evsel, target: *mut target, err: c_int, msg: *mut c_char, size: size_t);
    fn evlist__apply_filters(evlist: *mut evlist, counter: *mut *mut evsel, target: *mut target) -> c_int;
    fn evlist__do_mmap(evlist: *mut evlist, pages: c_uint) -> c_int;
    fn evlist__overwrite_mmap(evlist: *mut evlist) -> *mut mmap;
    fn evlist__mmap(evlist: *mut evlist) -> *mut mmap;
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__toggle_bkw_mmap(evlist: *mut evlist, state: c_int);
    fn perf_mmap__read_init(core: *mut mmap_core) -> c_int;
    fn perf_mmap__read_event(core: *mut mmap_core) -> *mut perf_event;
    fn perf_mmap__consume(core: *mut mmap_core);
    fn perf_mmap__read_done(core: *mut mmap_core);
    fn evlist__parse_sample_timestamp(evlist: *mut evlist, event: *mut perf_event, timestamp: *mut u64) -> c_int;
    fn ordered_events__queue(qe: *mut ordered_events, event: *mut perf_event, timestamp: u64, file_offset: u64, file_path: *const c_char) -> c_int;
    fn ordered_events__init(qe: *mut ordered_events, deliver: unsafe extern "C" fn(*mut ordered_events, *mut ordered_event) -> c_int, data: *mut c_void);
    fn ordered_events__set_copy_on_queue(qe: *mut ordered_events, copy: bool);
    fn ordered_events__free(qe: *mut ordered_events);
    fn ordered_events__flush(qe: *mut ordered_events, how: c_int) -> c_int;
    fn callchain_register_param(callchain: *mut callchain_param) -> c_int;
    fn parse_callchain_top_opt(arg: *const c_char) -> c_int;
    fn perf_default_config(var: *const c_char, value: *const c_char, cb: *mut c_void) -> c_int;
    fn perf_config_bool(var: *const c_char, value: *const c_char) -> bool;
    fn perf_env__lookup_objdump(env: *mut perf_env, path: *mut *mut c_char) -> c_int;
    fn perf_set_multithreaded();
    fn perf_set_singlethreaded();
    fn perf_event__synthesize_bpf_events(session: *mut perf_session, process: *mut c_void, machine: *mut machine, opts: *mut record_opts) -> c_int;
    fn perf_event__synthesize_cgroups(tool: *mut perf_tool, process: *mut c_void, machine: *mut machine) -> c_int;
    static perf_event__process: c_void;
    fn machine__synthesize_threads(machine: *mut machine, target: *mut target, threads: *mut c_void, mmap: bool, fork: bool, nr_threads: c_uint);
    fn perf_env__read_cpu_topology_map(env: *mut perf_env) -> c_int;
    fn evlist__uniquify_evsel_names(evlist: *mut evlist, config: *mut stat_config_t);
    fn evlist__enable(evlist: *mut evlist);
    fn evlist__poll(evlist: *mut evlist, timeout: c_int) -> c_int;
    fn target__none(target: *mut target) -> bool;
    fn perf_sample__init(sample: *mut perf_sample, all: bool);
    fn perf_sample__exit(sample: *mut perf_sample);
    fn evlist__parse_sample(evlist: *mut evlist, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn evlist__id2evsel(evlist: *mut evlist, id: u64) -> *mut evsel;
    fn evswitch__discard(evswitch: *mut evswitch, evsel: *mut evsel) -> bool;
    fn events_stats__inc(stats: *mut evlist_stats, type_: c_uint);
    fn machine__process_event(machine: *mut machine, event: *mut perf_event, sample: *mut perf_sample);
    fn intlist__new(arg: *mut c_void) -> *mut intlist;
    fn intlist__has_entry(list: *mut intlist, i: c_int) -> bool;
    fn intlist__add(list: *mut intlist, i: c_int) -> c_int;
    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn machine__resolve(machine: *mut machine, al: *mut addr_location, sample: *mut perf_sample) -> c_int;
    fn thread__set_lbr_stitch_enable(thread: *mut thread, enable: bool);
    fn evlist__exclude_kernel(evlist: *mut evlist) -> bool;
    fn evlist__new() -> *mut evlist;
    fn evlist__new_default(target: *mut target, callchain: bool) -> *mut evlist;
    fn evlist__splice_list_tail(evlist: *mut evlist, entries: *mut list_head);
    fn evlist__put(evlist: *mut evlist);
    fn perf_env__init(env: *mut perf_env);
    fn perf_env__exit(env: *mut perf_env);
    fn perf_config(cb: unsafe extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn perf_env__set_cmdline(env: *mut perf_env, argc: c_int, argv: *const *const c_char) -> c_int;
    fn perf_env__read_cpuid(env: *mut perf_env) -> c_int;
    fn parse_options(argc: c_int, argv: *const *const c_char, options: *const option, usage: *const *const c_char, flags: c_int) -> c_int;
    fn usage_with_options(usage: *const *const c_char, options: *const option);
    fn parse_options_usage(usage: *const *const c_char, options: *const option, opt: *const c_char, unset: c_int);
    fn symbol__validate_sym_arguments() -> c_int;
    fn annotate_check_args() -> c_int;
    fn target__validate(target: *mut target) -> c_int;
    fn target__strerror(target: *mut target, status: c_int, errbuf: *mut c_char, bufsiz: size_t);
    fn evswitch__init(evswitch: *mut evswitch, evlist: *mut evlist, fp: *mut FILE) -> c_int;
    fn perf_env__read_core_pmu_caps(env: *mut perf_env) -> c_int;
    fn setup_browser(fallback_to_pager: bool);
    fn evlist__set_session(evlist: *mut evlist, session: *mut perf_session);
    fn setup_sorting(evlist: *mut evlist, env: *mut perf_env) -> c_int;
    fn parse_uid(s: *const c_char) -> c_uint;
    fn parse_uid_filter(evlist: *mut evlist, uid: c_uint) -> c_int;
    fn evlist__create_maps(evlist: *mut evlist, target: *mut target) -> c_int;
    fn record_opts__config(opts: *mut record_opts) -> c_int;
    fn perf_hpp__cancel_cumulate(evlist: *mut evlist);
    fn symbol__annotation_init() -> c_int;
    fn annotation_config__init();
    fn annotation_options__init();
    fn annotation_options__exit();
    fn symbol__init(arg: *mut c_void) -> c_int;
    fn sort__setup_elide(fp: *mut FILE);
    fn evlist__needs_bpf_sb_event(evlist: *mut evlist) -> bool;
    fn evlist__add_bpf_sb_event(evlist: *mut evlist, env: *mut perf_env) -> c_int;
    fn evlist__start_sb_thread(evlist: *mut evlist, target: *mut target) -> c_int;
    fn evlist__stop_sb_thread(evlist: *mut evlist);
    fn sysctl__max_stack() -> c_uint;
    fn parse_events_option(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn parse_filter(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn evlist__parse_mmap_pages(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn record__parse_freq(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn report_parse_ignore_callees_opt(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn parse_filter_percentage(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn parse_branch_stack(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn parse_cgroups(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn parse_libpfm_events_option(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
}

static mut done: sig_atomic_t = 0;
static mut resize: sig_atomic_t = 0;
static mut last_timestamp: u64 = 0;

const fn cstr(s: &'static [u8]) -> *const c_char { s.as_ptr() as *const c_char }

unsafe fn evlist_for_each(mut evlist: *mut evlist, mut f: impl FnMut(*mut evsel)) {
    let mut pos = evlist__first(evlist);
    while !pos.is_null() {
        f(pos);
        pos = evlist__for_each_entry_next(evlist, pos);
    }
}

unsafe fn isdigit_c(c: c_char) -> bool { (c as u8).wrapping_sub(b'0') <= 9 }

unsafe extern "C" fn perf_top__update_print_entries(top: *mut perf_top) {
    (*top).print_entries = (*top).winsize.ws_row - HEADER_LINE_NR;
}

unsafe extern "C" fn winch_sig(_sig: c_int) { resize = 1; }

unsafe extern "C" fn perf_top__resize(top: *mut perf_top) {
    get_term_dimensions(&mut (*top).winsize);
    perf_top__update_print_entries(top);
}

unsafe extern "C" fn perf_top__parse_source(top: *mut perf_top, he: *mut hist_entry) -> c_int {
    let mut err = -1;
    if he.is_null() || (*he).ms.sym.is_null() { return -1; }
    let evsel = hists_to_evsel((*he).hists);
    let sym = (*he).ms.sym;
    let map = (*he).ms.map;
    let dso = map__dso(map);
    /*
     * We can't annotate with just /proc/kallsyms
     */
    if dso__symtab_type(dso) == DSO_BINARY_TYPE__KALLSYMS && !dso__is_kcore(dso) {
        pr_err(cstr(b"Can't annotate %s: No vmlinux file was found in the path\n\0"), (*sym).name);
        sleep(1);
        return -1;
    }
    let notes = symbol__annotation(sym);
    annotation__lock(notes);
    if !symbol__hists(sym, evlist__nr_entries((*top).evlist)) {
        annotation__unlock(notes);
        pr_err(cstr(b"Not enough memory for annotating '%s' symbol!\n\0"), (*sym).name);
        sleep(1);
        return err;
    }
    err = symbol__annotate(&mut (*he).ms, evsel, null_mut());
    if err == 0 {
        (*top).sym_filter_entry = he;
    } else {
        let mut msg = [0 as c_char; BUFSIZ];
        symbol__strerror_disassemble(&mut (*he).ms, err, msg.as_mut_ptr(), msg.len());
        pr_err(cstr(b"Couldn't annotate %s: %s\n\0"), (*sym).name, msg.as_ptr());
    }
    annotation__unlock(notes);
    err
}

unsafe extern "C" fn __zero_source_counters(he: *mut hist_entry) {
    let sym = (*he).ms.sym;
    symbol__annotate_zero_histograms(sym);
}

unsafe extern "C" fn ui__warn_map_erange(map: *mut map, sym: *mut symbol, ip: u64) {
    let mut uts: utsname = core::mem::zeroed();
    let err = uname(&mut uts);
    let dso = map__dso(map);
    ui__warning(
        cstr(b"Out of bounds address found:\n\nAddr:   %llx\nDSO:    %s %c\nMap:    %llx-%llx\nSymbol: %llx-%llx %c %s\nArch:   %s\nKernel: %s\nTools:  %s\n\nNot all samples will be on the annotation output.\n\nPlease report to linux-kernel@vger.kernel.org\n\0"),
        ip,
        dso__long_name(dso),
        dso__symtab_origin(dso),
        map__start(map),
        map__end(map),
        (*sym).start,
        (*sym).end,
        if symbol__binding(sym) == STB_GLOBAL { b'g' as c_int } else if symbol__binding(sym) == STB_LOCAL { b'l' as c_int } else { b'w' as c_int },
        (*sym).name,
        if err != 0 { cstr(b"[unknown]\0") } else { uts.machine.as_ptr() },
        if err != 0 { cstr(b"[unknown]\0") } else { uts.release.as_ptr() },
        perf_version_string,
    );
    if use_browser <= 0 { sleep(5); }
    map__set_erange_warned(map);
}

unsafe extern "C" fn perf_top__record_precise_ip(top: *mut perf_top, he: *mut hist_entry, sample: *mut perf_sample, ip: u64) {
    let sym = (*he).ms.sym;
    if sym.is_null() || (use_browser == 0 && ((*top).sym_filter_entry.is_null() || (*(*top).sym_filter_entry).ms.sym != sym)) {
        return;
    }
    let notes = symbol__annotation(sym);
    if !annotation__trylock(notes) { return; }
    let err = hist_entry__inc_addr_samples(he, sample, ip);
    annotation__unlock(notes);
    if err != 0 {
        /*
         * This function is now called with he->hists->lock held.
         * Release it before going to sleep.
         */
        mutex_unlock(&mut (*(*he).hists).lock);
        if err == -ERANGE && !map__erange_warned((*he).ms.map) {
            ui__warn_map_erange((*he).ms.map, sym, ip);
        } else if err == -ENOMEM {
            pr_err(cstr(b"Not enough memory for annotating '%s' symbol!\n\0"), (*sym).name);
            sleep(1);
        }
        mutex_lock(&mut (*(*he).hists).lock);
    }
}

unsafe extern "C" fn perf_top__show_details(top: *mut perf_top) {
    let he = (*top).sym_filter_entry;
    if he.is_null() { return; }
    let evsel = hists_to_evsel((*he).hists);
    let symbol = (*he).ms.sym;
    let notes = symbol__annotation(symbol);
    annotation__lock(notes);
    symbol__calc_percent(symbol, evsel);
    if (*notes).src.is_null() {
        annotation__unlock(notes);
        return;
    }
    printf(cstr(b"Showing %s for %s\n\0"), evsel__name((*top).sym_evsel), (*symbol).name);
    printf(cstr(b"  Events  Pcnt (>=%d%%)\n\0"), annotate_opts.min_pcnt);
    let more = hist_entry__annotate_printf(he, (*top).sym_evsel);
    if evlist__enabled((*top).evlist) {
        if (*top).zero { symbol__annotate_zero_histogram(symbol, (*top).sym_evsel); }
        else { symbol__annotate_decay_histogram(symbol, (*top).sym_evsel); }
    }
    if more != 0 {
        printf(cstr(b"%d lines not displayed, maybe increase display entries [e]\n\0"), more);
    }
    annotation__unlock(notes);
}

unsafe extern "C" fn perf_top__resort_hists(t: *mut perf_top) {
    let evlist = (*t).evlist;
    evlist_for_each(evlist, |pos| {
        let hists = evsel__hists(pos);
        hists__unlink(hists);
        if evlist__enabled(evlist) {
            if (*t).zero { hists__delete_entries(hists); }
            else { hists__decay_entries(hists, (*t).hide_user_symbols, (*t).hide_kernel_symbols); }
        }
        hists__collapse_resort(hists, null_mut());
        if symbol_conf.event_group && !evsel__is_group_leader(pos) {
            let leader_hists = evsel__hists(evsel__leader(pos));
            hists__match(leader_hists, hists);
            hists__link(leader_hists, hists);
        }
    });
    evlist_for_each(evlist, |pos| evsel__output_resort(pos, null_mut()));
}

unsafe extern "C" fn perf_top__print_sym_table(top: *mut perf_top) {
    let mut bf = [0 as c_char; 160];
    let mut printed = 0;
    let win_width = (*top).winsize.ws_col - 1;
    let evsel = (*top).sym_evsel;
    let hists = evsel__hists(evsel);
    puts(cstr(b"\x1b[H\x1b[J\0"));
    perf_top__header_snprintf(top, bf.as_mut_ptr(), bf.len());
    printf(cstr(b"%s\n\0"), bf.as_ptr());
    printf(cstr(b"%-*.*s\n\0"), win_width, win_width, graph_dotted_line);
    let stats = evlist__stats((*top).evlist);
    if !(*top).record_opts.overwrite && (*stats).nr_lost_warned != (*stats).nr_events[PERF_RECORD_LOST as usize] {
        (*stats).nr_lost_warned = (*stats).nr_events[PERF_RECORD_LOST as usize];
        color_fprintf(stdout, cstr(b"red\0"), cstr(b"WARNING: LOST %d chunks, Check IO/CPU overload\0"), (*stats).nr_lost_warned);
        printed += 1;
    }
    if !(*top).sym_filter_entry.is_null() {
        perf_top__show_details(top);
        return;
    }
    perf_top__resort_hists(top);
    hists__output_recalc_col_len(hists, (*top).print_entries - printed);
    putchar('\n' as c_int);
    hists__fprintf(hists, false, (*top).print_entries - printed, win_width, (*top).min_percent, stdout, !symbol_conf.use_callchain);
}

unsafe extern "C" fn prompt_integer(target: *mut c_int, msg: *const c_char) {
    let mut buf: *mut c_char = null_mut();
    let mut dummy: size_t = 0;
    fprintf(stdout, cstr(b"\n%s: \0"), msg);
    if getline(&mut buf, &mut dummy, stdin) < 0 { return; }
    let p = strchr(buf, '\n' as c_int);
    if !p.is_null() { *p = 0; }
    let mut p2 = buf;
    while *p2 != 0 {
        if !isdigit_c(*p2) { free(buf as *mut c_void); return; }
        p2 = p2.add(1);
    }
    *target = strtoul(buf, null_mut(), 10) as c_int;
    free(buf as *mut c_void);
}

unsafe extern "C" fn prompt_percent(target: *mut c_int, msg: *const c_char) {
    let mut tmp = 0;
    prompt_integer(&mut tmp, msg);
    if tmp >= 0 && tmp <= 100 { *target = tmp; }
}

unsafe extern "C" fn perf_top__prompt_symbol(top: *mut perf_top, msg: *const c_char) {
    let mut buf: *mut c_char = null_mut();
    let syme = (*top).sym_filter_entry;
    let mut found: *mut hist_entry = null_mut();
    let hists = evsel__hists((*top).sym_evsel);
    let mut dummy: size_t = 0;
    /* zero counters of active symbol */
    if !syme.is_null() {
        __zero_source_counters(syme);
        (*top).sym_filter_entry = null_mut();
    }
    fprintf(stdout, cstr(b"\n%s: \0"), msg);
    if getline(&mut buf, &mut dummy, stdin) < 0 { free(buf as *mut c_void); return; }
    let p = strchr(buf, '\n' as c_int);
    if !p.is_null() { *p = 0; }
    let mut next = rb_first_cached(&mut (*hists).entries);
    while !next.is_null() {
        let n = rb_entry_hist_entry(next);
        if !(*n).ms.sym.is_null() && strcmp(buf, (*(*n).ms.sym).name) == 0 {
            found = n;
            break;
        }
        next = rb_next(&mut (*n).rb_node);
    }
    if found.is_null() {
        fprintf(stderr, cstr(b"Sorry, %s is not active.\n\0"), buf);
        sleep(1);
    } else {
        perf_top__parse_source(top, found);
    }
    free(buf as *mut c_void);
}

unsafe extern "C" fn perf_top__print_mapped_keys(top: *mut perf_top) {
    let mut name: *mut c_char = null_mut();
    if !(*top).sym_filter_entry.is_null() {
        let sym = (*(*top).sym_filter_entry).ms.sym;
        name = (*sym).name;
    }
    fprintf(stdout, cstr(b"\nMapped keys:\n\0"));
    fprintf(stdout, cstr(b"\t[d]     display refresh delay.             \t(%d)\n\0"), (*top).delay_secs);
    fprintf(stdout, cstr(b"\t[e]     display entries (lines).           \t(%d)\n\0"), (*top).print_entries);
    if evlist__nr_entries((*top).evlist) > 1 {
        fprintf(stdout, cstr(b"\t[E]     active event counter.              \t(%s)\n\0"), evsel__name((*top).sym_evsel));
    }
    fprintf(stdout, cstr(b"\t[f]     profile display filter (count).    \t(%d)\n\0"), (*top).count_filter);
    fprintf(stdout, cstr(b"\t[F]     annotate display filter (percent). \t(%d%%)\n\0"), annotate_opts.min_pcnt);
    fprintf(stdout, cstr(b"\t[s]     annotate symbol.                   \t(%s)\n\0"), if !name.is_null() { name } else { cstr(b"NULL\0") as *mut c_char });
    fprintf(stdout, cstr(b"\t[S]     stop annotation.\n\0"));
    fprintf(stdout, cstr(b"\t[K]     hide kernel symbols.             \t(%s)\n\0"), if (*top).hide_kernel_symbols { cstr(b"yes\0") } else { cstr(b"no\0") });
    fprintf(stdout, cstr(b"\t[U]     hide user symbols.               \t(%s)\n\0"), if (*top).hide_user_symbols { cstr(b"yes\0") } else { cstr(b"no\0") });
    fprintf(stdout, cstr(b"\t[z]     toggle sample zeroing.             \t(%d)\n\0"), if (*top).zero { 1 } else { 0 });
    fprintf(stdout, cstr(b"\t[qQ]    quit.\n\0"));
}

unsafe extern "C" fn perf_top__key_mapped(top: *mut perf_top, c: c_int) -> c_int {
    match c as u8 as char {
        'd' | 'e' | 'f' | 'z' | 'q' | 'Q' | 'K' | 'U' | 'F' | 's' | 'S' => 1,
        'E' => if evlist__nr_entries((*top).evlist) > 1 { 1 } else { 0 },
        _ => 0,
    }
}

unsafe extern "C" fn perf_top__handle_keypress(top: *mut perf_top, mut c: c_int) -> bool {
    let mut ret = true;
    if perf_top__key_mapped(top, c) == 0 {
        let mut stdin_poll = pollfd { fd: 0, events: POLLIN, revents: 0 };
        let mut save: termios = core::mem::zeroed();
        perf_top__print_mapped_keys(top);
        fprintf(stdout, cstr(b"\nEnter selection, or unmapped key to continue: \0"));
        fflush(stdout);
        set_term_quiet_input(&mut save);
        poll(&mut stdin_poll, 1, -1);
        c = getc(stdin);
        tcsetattr(0, TCSAFLUSH, &save);
        if perf_top__key_mapped(top, c) == 0 { return ret; }
    }
    match c as u8 as char {
        'd' => { prompt_integer(&mut (*top).delay_secs, cstr(b"Enter display delay\0")); if (*top).delay_secs < 1 { (*top).delay_secs = 1; } }
        'e' => {
            prompt_integer(&mut (*top).print_entries, cstr(b"Enter display entries (lines)\0"));
            if (*top).print_entries == 0 { perf_top__resize(top); signal(SIGWINCH, winch_sig as usize); } else { signal(SIGWINCH, SIG_DFL); }
        }
        'E' => {
            if evlist__nr_entries((*top).evlist) > 1 {
                /* Select 0 as the default event: */
                let mut counter = 0;
                fprintf(stderr, cstr(b"\nAvailable events:\0"));
                evlist_for_each((*top).evlist, |pos| {
                    (*top).sym_evsel = pos;
                    fprintf(stderr, cstr(b"\n\t%d %s\0"), (*pos).core.idx, evsel__name(pos));
                });
                prompt_integer(&mut counter, cstr(b"Enter details event counter\0"));
                if counter >= evlist__nr_entries((*top).evlist) {
                    (*top).sym_evsel = evlist__first((*top).evlist);
                    fprintf(stderr, cstr(b"Sorry, no such event, using %s.\n\0"), evsel__name((*top).sym_evsel));
                    sleep(1);
                } else {
                    evlist_for_each((*top).evlist, |pos| {
                        if (*pos).core.idx == counter { (*top).sym_evsel = pos; }
                    });
                }
            } else { (*top).sym_evsel = evlist__first((*top).evlist); }
        }
        'f' => prompt_integer(&mut (*top).count_filter, cstr(b"Enter display event count filter\0")),
        'F' => prompt_percent(&mut annotate_opts.min_pcnt, cstr(b"Enter details display event filter (percent)\0")),
        'K' => (*top).hide_kernel_symbols = !(*top).hide_kernel_symbols,
        'q' | 'Q' => {
            printf(cstr(b"exiting.\n\0"));
            if (*top).dump_symtab { perf_session__fprintf_dsos((*top).session, stderr); }
            ret = false;
        }
        's' => perf_top__prompt_symbol(top, cstr(b"Enter details symbol\0")),
        'S' => {
            if !(*top).sym_filter_entry.is_null() {
                let syme = (*top).sym_filter_entry;
                (*top).sym_filter_entry = null_mut();
                __zero_source_counters(syme);
            }
        }
        'U' => (*top).hide_user_symbols = !(*top).hide_user_symbols,
        'z' => (*top).zero = !(*top).zero,
        _ => {}
    }
    ret
}

unsafe extern "C" fn perf_top__sort_new_samples(arg: *mut c_void) {
    let t = arg as *mut perf_top;
    if !evlist__selected((*t).evlist).is_null() { (*t).sym_evsel = evlist__selected((*t).evlist); }
    perf_top__resort_hists(t);
    if (*t).lost != 0 || (*t).drop != 0 {
        pr_warning(cstr(b"Too slow to read ring buffer (change period (-c/-F) or limit CPUs (-C)\n\0"));
    }
}

unsafe extern "C" fn stop_top() {
    session_done = 1;
    done = 1;
}

unsafe extern "C" fn display_thread_tui(arg: *mut c_void) -> *mut c_void {
    let top = arg as *mut perf_top;
    let help = cstr(b"For a higher level overview, try: perf top --sort comm,dso\0");
    let mut hbt = hist_browser_timer { timer: Some(perf_top__sort_new_samples), arg: top as *mut c_void, refresh: (*top).delay_secs };
    /* In order to read symbols from other namespaces perf top needs setns(2). */
    unshare(CLONE_FS);
    prctl(PR_SET_NAME, cstr(b"perf-top-UI\0"), 0, 0, 0);
'repeat: loop {
        perf_top__sort_new_samples(top as *mut c_void);
        evlist_for_each((*top).evlist, |pos| {
            let hists = evsel__hists(pos);
            (*hists).uid_filter_str = (*top).uid_str;
        });
        let ret = evlist__tui_browse_hists((*top).evlist, help, &mut hbt, (*top).min_percent, perf_session__env((*top).session), !(*top).record_opts.overwrite);
        if ret == K_RELOAD {
            (*top).zero = true;
            continue 'repeat;
        } else {
            stop_top();
        }
        break;
    }
    null_mut()
}

unsafe extern "C" fn display_sig(_sig: c_int) { stop_top(); }

unsafe extern "C" fn display_setup_sig() {
    signal(SIGSEGV, sighandler_dump_stack as usize);
    signal(SIGFPE, sighandler_dump_stack as usize);
    signal(SIGINT, display_sig as usize);
    signal(SIGQUIT, display_sig as usize);
    signal(SIGTERM, display_sig as usize);
}

unsafe extern "C" fn display_thread(arg: *mut c_void) -> *mut c_void {
    let mut stdin_poll = pollfd { fd: 0, events: POLLIN, revents: 0 };
    let mut save: termios = core::mem::zeroed();
    let top = arg as *mut perf_top;
    unshare(CLONE_FS);
    prctl(PR_SET_NAME, cstr(b"perf-top-UI\0"), 0, 0, 0);
    display_setup_sig();
    pthread__unblock_sigwinch();
'repeat: loop {
        let delay_msecs = (*top).delay_secs * MSEC_PER_SEC;
        set_term_quiet_input(&mut save);
        /* trash return */
        clearerr(stdin);
        if poll(&mut stdin_poll, 1, 0) > 0 { getc(stdin); }
        while done == 0 {
            perf_top__print_sym_table(top);
            /*
             * Either timeout expired or we got an EINTR due to SIGWINCH,
             * refresh screen in both cases.
             */
            match poll(&mut stdin_poll, 1, delay_msecs) {
                0 => continue,
                -1 if errno == EINTR => continue,
                _ => {
                    let c = getc(stdin);
                    tcsetattr(0, TCSAFLUSH, &save);
                    if perf_top__handle_keypress(top, c) { continue 'repeat; }
                    stop_top();
                }
            }
        }
        break;
    }
    tcsetattr(0, TCSAFLUSH, &save);
    null_mut()
}

unsafe extern "C" fn hist_iter__top_callback(iter: *mut hist_entry_iter, al: *mut addr_location, single: bool, arg: *mut c_void) -> c_int {
    let top = arg as *mut perf_top;
    if perf_hpp_list.sym && single {
        perf_top__record_precise_ip(top, (*iter).he, (*iter).sample, (*al).addr);
    }
    hist__account_cycles((*(*iter).sample).branch_stack, al, (*iter).sample, !((*top).record_opts.branch_stack & PERF_SAMPLE_BRANCH_ANY != 0), null_mut());
    0
}

unsafe extern "C" fn perf_event__process_sample(tool: *const perf_tool, event: *const perf_event, sample: *mut perf_sample, mut machine: *mut machine) {
    let top = tool as *mut perf_top;
    let mut al: addr_location = core::mem::zeroed();
    static mut seen: *mut intlist = null_mut();
    if machine.is_null() && perf_guest {
        if seen.is_null() { seen = intlist__new(null_mut()); }
        if !intlist__has_entry(seen, (*sample).pid) {
            pr_err(cstr(b"Can't find guest [%d]'s kernel information\n\0"), (*sample).pid);
            intlist__add(seen, (*sample).pid);
        }
        return;
    }
    if machine.is_null() {
        let stats = evlist__stats((*(*top).session).evlist);
        pr_err(cstr(b"%u unprocessable samples recorded.\r\0"), (*stats).nr_unprocessable_samples);
        (*stats).nr_unprocessable_samples += 1;
        return;
    }
    if ((*event).header.misc & PERF_RECORD_MISC_EXACT_IP) != 0 { (*top).exact_samples += 1; }
    addr_location__init(&mut al);
    if machine__resolve(machine, &mut al, sample) < 0 { addr_location__exit(&mut al); return; }
    if (*top).stitch_lbr { thread__set_lbr_stitch_enable(al.thread, true); }
    if !(*machine).kptr_restrict_warned && symbol_conf.kptr_restrict && al.cpumode == PERF_RECORD_MISC_KERNEL {
        if !evlist__exclude_kernel((*(*top).session).evlist) {
            ui__warning(cstr(b"Kernel address maps (/proc/{kallsyms,modules}) are restricted.\n\nCheck /proc/sys/kernel/kptr_restrict and /proc/sys/kernel/perf_event_paranoid.\n\nKernel%s samples will not be resolved.\n\0"), if !al.map.is_null() && map__has_symbols(al.map) { cstr(b" modules\0") } else { cstr(b"\0") });
            if use_browser <= 0 { sleep(5); }
        }
        (*machine).kptr_restrict_warned = true;
    }
    if al.sym.is_null() && !al.map.is_null() {
        let msg = cstr(b"Kernel samples will not be resolved.\n\0");
        if !(*machine).kptr_restrict_warned && !(*top).vmlinux_warned && __map__is_kernel(al.map) && !map__has_symbols(al.map) {
            if !symbol_conf.vmlinux_name.is_null() {
                let mut serr = [0 as c_char; 256];
                dso__strerror_load(map__dso(al.map), serr.as_mut_ptr(), serr.len());
                ui__warning(cstr(b"The %s file can't be used: %s\n%s\0"), symbol_conf.vmlinux_name, serr.as_ptr(), msg);
            } else {
                ui__warning(cstr(b"A vmlinux file was not found.\n%s\0"), msg);
            }
            if use_browser <= 0 { sleep(5); }
            (*top).vmlinux_warned = true;
        }
    }
    if al.sym.is_null() || !symbol__is_idle(al.sym, if !al.map.is_null() { map__dso(al.map) } else { null_mut() }, (*machine).env) {
        let hists = evsel__hists((*sample).evsel);
        let mut iter = hist_entry_iter { sample, add_entry_cb: Some(hist_iter__top_callback), ops: null(), he: null_mut() };
        if symbol_conf.cumulate_callchain { iter.ops = &hist_iter_cumulative as *const _ as *const c_void; }
        else { iter.ops = &hist_iter_normal as *const _ as *const c_void; }
        mutex_lock(&mut (*hists).lock);
        if hist_entry_iter__add(&mut iter, &mut al, (*top).max_stack, top as *mut c_void) < 0 {
            pr_err(cstr(b"Problem incrementing symbol period, skipping event\n\0"));
        }
        mutex_unlock(&mut (*hists).lock);
    }
    addr_location__exit(&mut al);
}

unsafe extern "C" fn perf_top__process_lost(top: *mut perf_top, event: *mut perf_event, evsel: *mut evsel) {
    (*top).lost += (*event).lost.lost;
    (*top).lost_total += (*event).lost.lost;
    (*evlist__stats((*evsel).evlist)).total_lost += (*event).lost.lost;
}

unsafe extern "C" fn perf_top__process_lost_samples(top: *mut perf_top, event: *mut perf_event, evsel: *mut evsel) {
    (*top).lost += (*event).lost_samples.lost;
    (*top).lost_total += (*event).lost_samples.lost;
    (*evlist__stats((*evsel).evlist)).total_lost_samples += (*event).lost_samples.lost;
}

unsafe extern "C" fn perf_top__mmap_read_idx(top: *mut perf_top, idx: c_int) {
    let opts = &mut (*top).record_opts;
    let evlist = (*top).evlist;
    let md = if opts.overwrite { evlist__overwrite_mmap(evlist).add(idx as usize) } else { evlist__mmap(evlist).add(idx as usize) };
    if perf_mmap__read_init(&mut (*md).core) < 0 { return; }
    loop {
        let event = perf_mmap__read_event(&mut (*md).core);
        if event.is_null() { break; }
        let mut ret = evlist__parse_sample_timestamp(evlist, event, &mut last_timestamp);
        if ret != 0 && ret != -1 { break; }
        ret = ordered_events__queue((*top).qe.in_, event, last_timestamp, 0, null());
        if ret != 0 { break; }
        perf_mmap__consume(&mut (*md).core);
        if (*top).qe.rotate {
            mutex_lock(&mut (*top).qe.mutex);
            (*top).qe.rotate = false;
            cond_signal(&mut (*top).qe.cond);
            mutex_unlock(&mut (*top).qe.mutex);
        }
    }
    perf_mmap__read_done(&mut (*md).core);
}

unsafe extern "C" fn perf_top__mmap_read(top: *mut perf_top) {
    let overwrite = (*top).record_opts.overwrite;
    let evlist = (*top).evlist;
    if overwrite { evlist__toggle_bkw_mmap(evlist, BKW_MMAP_DATA_PENDING); }
    for i in 0..(*evlist__core((*top).evlist)).nr_mmaps {
        perf_top__mmap_read_idx(top, i);
    }
    if overwrite {
        evlist__toggle_bkw_mmap(evlist, BKW_MMAP_EMPTY);
        evlist__toggle_bkw_mmap(evlist, BKW_MMAP_RUNNING);
    }
}

/*
 * Check per-event overwrite term.
 * perf top should support consistent term for all events.
 */
unsafe extern "C" fn perf_top__overwrite_check(top: *mut perf_top) -> c_int {
    let opts = &mut (*top).record_opts;
    let evlist = (*top).evlist;
    let mut overwrite = -1;
    evlist_for_each(evlist, |evsel| {
        let mut set = -1;
        let mut term_head = (*evsel).config_terms.next;
        while !term_head.is_null() && term_head != &mut (*evsel).config_terms {
            let term = term_head as *mut evsel_config_term;
            if (*term).type_ == EVSEL__CONFIG_TERM_OVERWRITE {
                set = if (*term).val.overwrite { 1 } else { 0 };
            }
            term_head = (*term_head).next;
        }
        if overwrite == -2 { return; }
        if overwrite < 0 && set < 0 { return; }
        if overwrite >= 0 && set >= 0 && overwrite != set { overwrite = -2; return; }
        if overwrite >= 0 && set < 0 { overwrite = -2; return; }
        if overwrite < 0 && set >= 0 {
            if evsel == evlist__first(evlist) { overwrite = set; } else { overwrite = -2; }
        }
    });
    if overwrite == -2 { return -1; }
    if overwrite >= 0 && opts.overwrite != (overwrite != 0) { opts.overwrite = overwrite != 0; }
    0
}

unsafe extern "C" fn perf_top_overwrite_fallback(top: *mut perf_top, evsel: *mut evsel) -> c_int {
    let opts = &mut (*top).record_opts;
    let evlist = (*top).evlist;
    if !opts.overwrite { return 0; }
    /* only fall back when first event fails */
    if evsel != evlist__first(evlist) { return 0; }
    evlist_for_each(evlist, |counter| (*counter).core.attr.write_backward = false);
    opts.overwrite = false;
    pr_debug2(cstr(b"fall back to non-overwrite mode\n\0"));
    1
}

unsafe extern "C" fn perf_top__start_counters(top: *mut perf_top) -> c_int {
    let mut msg = [0 as c_char; BUFSIZ];
    let evlist = (*top).evlist;
    let opts = &mut (*top).record_opts;
    if perf_top__overwrite_check(top) != 0 {
        ui__error(cstr(b"perf top only support consistent per-event overwrite setting for all events\n\0"));
        return -1;
    }
    evlist__config(evlist, opts, &mut callchain_param);
    let mut failed = false;
    evlist_for_each(evlist, |counter| {
        if failed { return; }
        loop {
            if evsel__open(counter, (*counter).core.cpus, (*counter).core.threads) >= 0 { break; }
            if perf_missing_features.write_backward && perf_top_overwrite_fallback(top, counter) != 0 { continue; }
            if evsel__fallback(counter, &mut opts.target, errno, msg.as_mut_ptr(), msg.len()) {
                if verbose > 0 { ui__warning(cstr(b"%s\n\0"), msg.as_ptr()); }
                continue;
            }
            evsel__open_strerror(counter, &mut opts.target, errno, msg.as_mut_ptr(), msg.len());
            ui__error(cstr(b"%s\n\0"), msg.as_ptr());
            failed = true;
            break;
        }
    });
    if failed { return -1; }
    let mut counter: *mut evsel = null_mut();
    if evlist__apply_filters(evlist, &mut counter, &mut opts.target) != 0 {
        pr_err(cstr(b"failed to set filter \"%s\" on event %s with %d (%s)\n\0"), if !(*counter).filter.is_null() { (*counter).filter } else { cstr(b"BPF\0") }, evsel__name(counter), errno, str_error_r(errno, msg.as_mut_ptr(), msg.len()));
        return -1;
    }
    if evlist__do_mmap(evlist, opts.mmap_pages) < 0 {
        ui__error(cstr(b"Failed to mmap with %d (%s)\n\0"), errno, str_error_r(errno, msg.as_mut_ptr(), msg.len()));
        return -1;
    }
    0
}

unsafe extern "C" fn callchain_param__setup_sample_type(callchain: *mut callchain_param) -> c_int {
    if (*callchain).mode != CHAIN_NONE {
        if callchain_register_param(callchain) < 0 {
            ui__error(cstr(b"Can't register callchain params.\n\0"));
            return -EINVAL;
        }
    }
    0
}

unsafe extern "C" fn rotate_queues(top: *mut perf_top) -> *mut ordered_events {
    let inq = (*top).qe.in_;
    if (*top).qe.in_ == &mut (*top).qe.data[1] { (*top).qe.in_ = &mut (*top).qe.data[0]; }
    else { (*top).qe.in_ = &mut (*top).qe.data[1]; }
    inq
}

unsafe extern "C" fn process_thread(arg: *mut c_void) -> *mut c_void {
    let top = arg as *mut perf_top;
    while done == 0 {
        let inq = (*top).qe.in_;
        if (*inq).nr_events == 0 {
            usleep(100);
            continue;
        }
        let out = rotate_queues(top);
        mutex_lock(&mut (*top).qe.mutex);
        (*top).qe.rotate = true;
        cond_wait(&mut (*top).qe.cond, &mut (*top).qe.mutex);
        mutex_unlock(&mut (*top).qe.mutex);
        if ordered_events__flush(out, OE_FLUSH__TOP) != 0 {
            pr_err(cstr(b"failed to process events\n\0"));
        }
    }
    null_mut()
}

/*
 * Allow only 'top->delay_secs' seconds behind samples.
 */
unsafe extern "C" fn should_drop(qevent: *mut ordered_event, top: *mut perf_top) -> c_int {
    let event = (*qevent).event;
    if (*event).header.type_ != PERF_RECORD_SAMPLE { return 0; }
    let delay_timestamp = (*qevent).timestamp + (*top).delay_secs as u64 * NSEC_PER_SEC;
    (delay_timestamp < last_timestamp) as c_int
}

unsafe extern "C" fn deliver_event(qe: *mut ordered_events, qevent: *mut ordered_event) -> c_int {
    let top = (*qe).data as *mut perf_top;
    let evlist = (*top).evlist;
    let session = (*top).session;
    let event = (*qevent).event;
    let mut sample: perf_sample = core::mem::zeroed();
    let mut ret = -1;
    if should_drop(qevent, top) != 0 {
        (*top).drop += 1;
        (*top).drop_total += 1;
        return 0;
    }
    perf_sample__init(&mut sample, false);
    ret = evlist__parse_sample(evlist, event, &mut sample);
    if ret != 0 {
        pr_err(cstr(b"Can't parse sample, err = %d\n\0"), ret);
        perf_sample__exit(&mut sample);
        return ret;
    }
    let mut evsel = sample.evsel;
    if evsel.is_null() { evsel = evlist__id2evsel((*session).evlist, sample.id); }
    assert!(!evsel.is_null());
    if (*event).header.type_ == PERF_RECORD_SAMPLE {
        if evswitch__discard(&mut (*top).evswitch, evsel) {
            perf_sample__exit(&mut sample);
            return 0;
        }
        (*top).samples += 1;
    }
    let machine: *mut machine = match sample.cpumode {
        PERF_RECORD_MISC_USER => { (*top).us_samples += 1; if (*top).hide_user_symbols { perf_sample__exit(&mut sample); return ret; } &mut (*session).machines.host }
        PERF_RECORD_MISC_KERNEL => { (*top).kernel_samples += 1; if (*top).hide_kernel_symbols { perf_sample__exit(&mut sample); return ret; } &mut (*session).machines.host }
        PERF_RECORD_MISC_GUEST_KERNEL => { (*top).guest_kernel_samples += 1; perf_session__find_machine(session, sample.pid) }
        PERF_RECORD_MISC_GUEST_USER => { (*top).guest_us_samples += 1; perf_sample__exit(&mut sample); return ret; }
        _ => { if (*event).header.type_ == PERF_RECORD_SAMPLE { perf_sample__exit(&mut sample); return ret; } &mut (*session).machines.host }
    };
    if (*event).header.type_ == PERF_RECORD_SAMPLE {
        perf_event__process_sample(&(*top).tool, event, &mut sample, machine);
    } else if (*event).header.type_ == PERF_RECORD_LOST {
        perf_top__process_lost(top, event, evsel);
    } else if (*event).header.type_ == PERF_RECORD_LOST_SAMPLES {
        perf_top__process_lost_samples(top, event, evsel);
    } else if (*event).header.type_ < PERF_RECORD_MAX {
        events_stats__inc(evlist__stats((*session).evlist), (*event).header.type_);
        machine__process_event(machine, event, &mut sample);
    } else {
        (*evlist__stats((*session).evlist)).nr_unknown_events += 1;
    }
    ret = 0;
    perf_sample__exit(&mut sample);
    ret
}

unsafe extern "C" fn init_process_thread(top: *mut perf_top) {
    ordered_events__init(&mut (*top).qe.data[0], deliver_event, top as *mut c_void);
    ordered_events__init(&mut (*top).qe.data[1], deliver_event, top as *mut c_void);
    ordered_events__set_copy_on_queue(&mut (*top).qe.data[0], true);
    ordered_events__set_copy_on_queue(&mut (*top).qe.data[1], true);
    (*top).qe.in_ = &mut (*top).qe.data[0];
    mutex_init(&mut (*top).qe.mutex);
    cond_init(&mut (*top).qe.cond);
}

unsafe extern "C" fn exit_process_thread(top: *mut perf_top) {
    ordered_events__free(&mut (*top).qe.data[0]);
    ordered_events__free(&mut (*top).qe.data[1]);
    mutex_destroy(&mut (*top).qe.mutex);
    cond_destroy(&mut (*top).qe.cond);
}

unsafe extern "C" fn __cmd_top(top: *mut perf_top) -> c_int {
    let opts = &mut (*top).record_opts;
    let mut thread: pthread_t = 0;
    let mut thread_process: pthread_t = 0;
    let mut ret: c_int;
    if annotate_opts.objdump_path.is_null() {
        ret = perf_env__lookup_objdump(perf_session__env((*top).session), &mut annotate_opts.objdump_path);
        if ret != 0 { return ret; }
    }
    ret = callchain_param__setup_sample_type(&mut callchain_param);
    if ret != 0 { return ret; }
    if perf_session__register_idle_thread((*top).session) < 0 { return ret; }
    if (*top).nr_threads_synthesize > 1 { perf_set_multithreaded(); }
    init_process_thread(top);
    if opts.record_namespaces { (*top).tool.namespace_events = true; }
    if opts.record_cgroup {
        if HAVE_FILE_HANDLE { (*top).tool.cgroup_events = true; }
        else { pr_err(cstr(b"cgroup tracking is not supported.\n\0")); return -1; }
    }
    ret = perf_event__synthesize_bpf_events((*top).session, &perf_event__process as *const _ as *mut c_void, &mut (*(*top).session).machines.host, &mut (*top).record_opts);
    if ret < 0 { pr_debug(cstr(b"Couldn't synthesize BPF events: Pre-existing BPF programs won't have symbols resolved.\n\0")); }
    ret = perf_event__synthesize_cgroups(&mut (*top).tool, &perf_event__process as *const _ as *mut c_void, &mut (*(*top).session).machines.host);
    if ret < 0 { pr_debug(cstr(b"Couldn't synthesize cgroup events.\n\0")); }
    machine__synthesize_threads(&mut (*(*top).session).machines.host, &mut opts.target, (*evlist__core((*top).evlist)).threads, true, false, (*top).nr_threads_synthesize);
    perf_set_multithreaded();
    if perf_hpp_list.socket {
        ret = perf_env__read_cpu_topology_map(perf_session__env((*top).session));
        if ret < 0 {
            let mut errbuf = [0 as c_char; BUFSIZ];
            let err = str_error_r(-ret, errbuf.as_mut_ptr(), errbuf.len());
            ui__error(cstr(b"Could not read the CPU topology map: %s\n\0"), err);
            return ret;
        }
    }
    evlist__uniquify_evsel_names((*top).evlist, &mut stat_config);
    ret = perf_top__start_counters(top);
    if ret != 0 { return ret; }
    (*(*top).session).evlist = (*top).evlist;
    perf_session__set_id_hdr_size((*top).session);
    /*
     * When perf is starting the traced process, all the events (apart from
     * group members) have enable_on_exec=1 set, so don't spoil it by
     * prematurely enabling them.
     */
    if !target__none(&mut opts.target) { evlist__enable((*top).evlist); }
    ret = -1;
    if pthread_create(&mut thread_process, null(), process_thread, top as *mut c_void) != 0 {
        ui__error(cstr(b"Could not create process thread.\n\0"));
        return ret;
    }
    let display: unsafe extern "C" fn(*mut c_void) -> *mut c_void = if use_browser > 0 { display_thread_tui } else { display_thread };
    if pthread_create(&mut thread, null(), display, top as *mut c_void) != 0 {
        ui__error(cstr(b"Could not create display thread.\n\0"));
        cond_signal(&mut (*top).qe.cond);
        pthread_join(thread_process, null_mut());
        perf_set_singlethreaded();
        exit_process_thread(top);
        return ret;
    }
    if (*top).realtime_prio != 0 {
        let param = sched_param { sched_priority: (*top).realtime_prio };
        if sched_setscheduler(0, SCHED_FIFO, &param) != 0 {
            ui__error(cstr(b"Could not set realtime priority.\n\0"));
            pthread_join(thread, null_mut());
            cond_signal(&mut (*top).qe.cond);
            pthread_join(thread_process, null_mut());
            perf_set_singlethreaded();
            exit_process_thread(top);
            return ret;
        }
    }
    /* Wait for a minimal set of events before starting the snapshot */
    evlist__poll((*top).evlist, 100);
    perf_top__mmap_read(top);
    while done == 0 {
        let hits = (*top).samples;
        perf_top__mmap_read(top);
        if opts.overwrite || hits == (*top).samples { ret = evlist__poll((*top).evlist, 100); }
        if resize != 0 {
            perf_top__resize(top);
            resize = 0;
        }
    }
    ret = 0;
    pthread_join(thread, null_mut());
    cond_signal(&mut (*top).qe.cond);
    pthread_join(thread_process, null_mut());
    perf_set_singlethreaded();
    exit_process_thread(top);
    ret
}

unsafe extern "C" fn parse_callchain_opt(opt: *const option, arg: *const c_char, unset: c_int) -> c_int {
    let callchain = (*opt).value as *mut callchain_param;
    (*callchain).enabled = unset == 0;
    (*callchain).record_mode = CALLCHAIN_FP;
    /*
     * --no-call-graph
     */
    if unset != 0 {
        symbol_conf.use_callchain = false;
        (*callchain).record_mode = CALLCHAIN_NONE;
        return 0;
    }
    parse_callchain_top_opt(arg)
}

unsafe extern "C" fn callchain_opt(opt: *const option, _arg: *const c_char, unset: c_int) -> c_int {
    let callchain = (*opt).value as *mut callchain_param;
    /*
     * The -g option only sets the callchain if not already configured by
     * .perfconfig. It does, however, enable it.
     */
    if (*callchain).record_mode != CALLCHAIN_NONE {
        (*callchain).enabled = true;
        return 0;
    }
    parse_callchain_opt(opt, if EM_HOST != EM_S390 { cstr(b"fp\0") } else { cstr(b"dwarf\0") }, unset)
}

unsafe extern "C" fn perf_top_config(mut var: *const c_char, value: *const c_char, cb: *mut c_void) -> c_int {
    if strcmp(var, cstr(b"top.call-graph\0")) == 0 {
        var = cstr(b"call-graph.record-mode\0");
        return perf_default_config(var, value, cb);
    }
    if strcmp(var, cstr(b"top.children\0")) == 0 {
        symbol_conf.cumulate_callchain = perf_config_bool(var, value);
        return 0;
    }
    0
}

unsafe extern "C" fn parse_percent_limit(opt: *const option, arg: *const c_char, _unset: c_int) -> c_int {
    let top = (*opt).value as *mut perf_top;
    (*top).min_percent = strtof(arg, null_mut());
    0
}

/*
 * The C source builds a large static parse-options table in cmd_top using
 * OPT_* macros from perf's option parser. This Rust translation preserves the
 * command setup, validation, and execution order; individual option entries are
 * represented as opaque option records because their macro-expanded layout is
 * supplied by external perf headers.
 */
#[no_mangle]
pub unsafe extern "C" fn cmd_top(mut argc: c_int, argv: *const *const c_char) -> c_int {
    let mut errbuf = [0 as c_char; BUFSIZ];
    let mut top: perf_top = core::mem::zeroed();
    top.count_filter = 5;
    top.delay_secs = 2;
    top.record_opts.mmap_pages = UINT_MAX;
    top.record_opts.user_freq = UINT_MAX;
    top.record_opts.user_interval = ULLONG_MAX;
    top.record_opts.freq = 4000; /* 4 KHz */
    top.record_opts.target.uses_mmap = true;
    /*
     * FIXME: This will lose PERF_RECORD_MMAP and other metadata
     * when we pause, fix that and reenable. Probably using a
     * separate evlist with a dummy event, i.e. a non-overwrite
     * ring buffer just for metadata events, while PERF_RECORD_SAMPLE
     * stays in overwrite mode. -acme
     */
    top.record_opts.overwrite = false;
    top.record_opts.sample_time = true;
    top.record_opts.sample_time_set = true;
    top.max_stack = sysctl__max_stack();
    top.nr_threads_synthesize = UINT_MAX;

    let mut parse_events_option_args = parse_events_option_args { evlistp: &mut top.evlist };
    let mut branch_call_mode = false;
    let opts = &mut top.record_opts as *mut record_opts;
    let target = &mut (*opts).target as *mut target;
    let mut disassembler_style: *const c_char = null();
    let mut objdump_path: *const c_char = null();
    let mut addr2line_path: *const c_char = null();
    let top_callchain_help = cstr(b"callchain record/report help\n\t\t\t\tDefault: fp,graph,0.5,caller,function\0");
    let options: [option; 1] = [option { value: null_mut() }];
    let top_usage: [*const c_char; 2] = [cstr(b"perf top [<options>]\0"), null()];
    let mut status = hists__init();
    if status < 0 { return status; }
    annotation_options__init();
    annotate_opts.min_pcnt = 5;
    annotate_opts.context = 4;
    top.evlist = evlist__new();
    if top.evlist.is_null() { return -ENOMEM; }
    let mut host_env: perf_env = core::mem::zeroed();
    perf_env__init(&mut host_env);
    status = perf_config(perf_top_config, &mut top as *mut _ as *mut c_void);
    if status != 0 { goto_out_put_evlist(&mut top, &mut host_env); return status; }
    /*
     * Since the per arch annotation init routine may need the cpuid, read
     * it here, since we are not getting this from the perf.data header.
     */
    status = perf_env__set_cmdline(&mut host_env, argc, argv);
    if status != 0 { goto_out_put_evlist(&mut top, &mut host_env); return status; }
    status = perf_env__read_cpuid(&mut host_env);
    if status != 0 {
        /*
         * Some arches do not provide a get_cpuid(), so just use pr_debug, otherwise
         * warn the user explicitly.
         */
        eprintf(if status == ENOSYS { 1 } else { 0 }, verbose, cstr(b"Couldn't read the cpuid for this machine: %s\n\0"), str_error_r(errno, errbuf.as_mut_ptr(), errbuf.len()));
    }
    argc = parse_options(argc, argv, options.as_ptr(), top_usage.as_ptr(), 0);
    if argc != 0 { usage_with_options(top_usage.as_ptr(), options.as_ptr()); }
    if !disassembler_style.is_null() {
        annotate_opts.disassembler_style = strdup(disassembler_style);
        if annotate_opts.disassembler_style.is_null() { status = -ENOMEM; goto_out_put_evlist(&mut top, &mut host_env); return status; }
    }
    if !objdump_path.is_null() {
        annotate_opts.objdump_path = strdup(objdump_path);
        if annotate_opts.objdump_path.is_null() { status = -ENOMEM; goto_out_put_evlist(&mut top, &mut host_env); return status; }
    }
    if !addr2line_path.is_null() {
        symbol_conf.addr2line_path = strdup(addr2line_path);
        if symbol_conf.addr2line_path.is_null() { status = -ENOMEM; goto_out_put_evlist(&mut top, &mut host_env); return status; }
    }
    status = symbol__validate_sym_arguments();
    if status != 0 { goto_out_put_evlist(&mut top, &mut host_env); return status; }
    if annotate_check_args() < 0 { goto_out_put_evlist(&mut top, &mut host_env); return status; }
    status = target__validate(target);
    if status != 0 {
        target__strerror(target, status, errbuf.as_mut_ptr(), BUFSIZ);
        ui__warning(cstr(b"%s\n\0"), errbuf.as_ptr());
    }
    if target__none(target) { (*target).system_wide = true; }
    if evlist__nr_entries(top.evlist) == 0 {
        let def_evlist = evlist__new_default(target, callchain_param.enabled);
        if def_evlist.is_null() { goto_out_put_evlist(&mut top, &mut host_env); return status; }
        evlist__splice_list_tail(top.evlist, &mut (*evlist__core(def_evlist)).entries);
        evlist__put(def_evlist);
    }
    status = evswitch__init(&mut top.evswitch, top.evlist, stderr);
    if status != 0 { goto_out_put_evlist(&mut top, &mut host_env); return status; }
    if symbol_conf.report_hierarchy {
        /* disable incompatible options */
        symbol_conf.event_group = false;
        symbol_conf.cumulate_callchain = false;
        if !field_order.is_null() {
            pr_err(cstr(b"Error: --hierarchy and --fields options cannot be used together\n\0"));
            parse_options_usage(top_usage.as_ptr(), options.as_ptr(), cstr(b"fields\0"), 0);
            parse_options_usage(null(), options.as_ptr(), cstr(b"hierarchy\0"), 0);
            goto_out_put_evlist(&mut top, &mut host_env);
            return status;
        }
    }
    if top.stitch_lbr && callchain_param.record_mode != CALLCHAIN_LBR {
        pr_err(cstr(b"Error: --stitch-lbr must be used with --call-graph lbr\n\0"));
        goto_out_put_evlist(&mut top, &mut host_env);
        return status;
    }
    if nr_cgroups > 0 && (*opts).record_cgroup {
        pr_err(cstr(b"--cgroup and --all-cgroups cannot be used together\n\0"));
        goto_out_put_evlist(&mut top, &mut host_env);
        return status;
    }
    if branch_call_mode {
        if (*opts).branch_stack == 0 { (*opts).branch_stack = PERF_SAMPLE_BRANCH_ANY; }
        symbol_conf.use_callchain = true;
        callchain_param.key = CCKEY_ADDRESS;
        callchain_param.branch_callstack = true;
        callchain_param.enabled = true;
        if callchain_param.record_mode == CALLCHAIN_NONE { callchain_param.record_mode = CALLCHAIN_FP; }
        callchain_register_param(&mut callchain_param);
        if sort_order.is_null() { sort_order = cstr(b"srcline,symbol,dso\0"); }
    }
    if (*opts).branch_stack != 0 && callchain_param.enabled { symbol_conf.show_branchflag_count = true; }
    if (*opts).branch_stack != 0 {
        status = perf_env__read_core_pmu_caps(&mut host_env);
        if status != 0 {
            pr_err(cstr(b"PMU capability data is not available\n\0"));
            goto_out_put_evlist(&mut top, &mut host_env);
            return status;
        }
    }
    sort__mode = SORT_MODE__TOP;
    /* display thread wants entries to be collapsed in a different tree */
    perf_hpp_list.need_collapse = 1;
    if top.use_stdio { use_browser = 0; }
    else if HAVE_SLANG_SUPPORT && top.use_tui { use_browser = 1; }
    setup_browser(false);
    top.session = __perf_session__new(null_mut(), null_mut(), false, &mut host_env);
    if IS_ERR(top.session) {
        status = PTR_ERR(top.session);
        top.session = null_mut();
        goto_out_put_evlist(&mut top, &mut host_env);
        return status;
    }
    evlist__set_session(top.evlist, top.session);
    if setup_sorting(top.evlist, perf_session__env(top.session)) < 0 {
        if !sort_order.is_null() { parse_options_usage(top_usage.as_ptr(), options.as_ptr(), cstr(b"s\0"), 1); }
        if !field_order.is_null() { parse_options_usage(if !sort_order.is_null() { null() } else { top_usage.as_ptr() }, options.as_ptr(), cstr(b"fields\0"), 0); }
        goto_out_put_evlist(&mut top, &mut host_env);
        return status;
    }
    if !top.uid_str.is_null() {
        let uid = parse_uid(top.uid_str);
        if uid == UINT_MAX {
            ui__error(cstr(b"Invalid User: %s\0"), top.uid_str);
            status = -EINVAL;
            goto_out_put_evlist(&mut top, &mut host_env);
            return status;
        }
        status = parse_uid_filter(top.evlist, uid);
        if status != 0 { goto_out_put_evlist(&mut top, &mut host_env); return status; }
    }
    if evlist__create_maps(top.evlist, target) < 0 {
        ui__error(cstr(b"Couldn't create thread/CPU maps: %s\n\0"), if errno == ENOENT { cstr(b"No such process\0") } else { str_error_r(errno, errbuf.as_mut_ptr(), errbuf.len()) });
        status = -errno;
        goto_out_put_evlist(&mut top, &mut host_env);
        return status;
    }
    if top.delay_secs < 1 { top.delay_secs = 1; }
    if record_opts__config(opts) != 0 {
        status = -EINVAL;
        goto_out_put_evlist(&mut top, &mut host_env);
        return status;
    }
    top.sym_evsel = evlist__first(top.evlist);
    if !callchain_param.enabled {
        symbol_conf.cumulate_callchain = false;
        perf_hpp__cancel_cumulate(top.evlist);
    }
    if symbol_conf.cumulate_callchain && !callchain_param.order_set { callchain_param.order = ORDER_CALLER; }
    status = symbol__annotation_init();
    if status < 0 { goto_out_put_evlist(&mut top, &mut host_env); return status; }
    annotation_config__init();
    symbol_conf.try_vmlinux_path = symbol_conf.vmlinux_name.is_null();
    status = symbol__init(null_mut());
    if status < 0 { goto_out_put_evlist(&mut top, &mut host_env); return status; }
    sort__setup_elide(stdout);
    get_term_dimensions(&mut top.winsize);
    if top.print_entries == 0 {
        perf_top__update_print_entries(&mut top);
        signal(SIGWINCH, winch_sig as usize);
    }
    if !evlist__needs_bpf_sb_event(top.evlist) { top.record_opts.no_bpf_event = true; }
    if HAVE_LIBBPF_SUPPORT && !top.record_opts.no_bpf_event {
        top.sb_evlist = evlist__new();
        if top.sb_evlist.is_null() {
            pr_err(cstr(b"Couldn't create side band evlist.\n.\0"));
            status = -EINVAL;
            goto_out_put_evlist(&mut top, &mut host_env);
            return status;
        }
        if evlist__add_bpf_sb_event(top.sb_evlist, &mut host_env) != 0 {
            pr_err(cstr(b"Couldn't ask for PERF_RECORD_BPF_EVENT side band events.\n.\0"));
            status = -EINVAL;
            evlist__put(top.sb_evlist);
            top.sb_evlist = null_mut();
            goto_out_put_evlist(&mut top, &mut host_env);
            return status;
        }
    }
    if evlist__start_sb_thread(top.sb_evlist, target) != 0 {
        pr_debug(cstr(b"Couldn't start the BPF side band thread:\nBPF programs starting from now on won't be annotatable\n\0"));
        (*opts).no_bpf_event = true;
    }
    status = __cmd_top(&mut top);
    if !(*opts).no_bpf_event { evlist__stop_sb_thread(top.sb_evlist); }
    goto_out_put_evlist(&mut top, &mut host_env);
    status
}

unsafe fn goto_out_put_evlist(top: *mut perf_top, host_env: *mut perf_env) {
    evlist__put((*top).evlist);
    perf_session__delete((*top).session);
    annotation_options__exit();
    perf_env__exit(host_env);
}

unsafe extern "C" {
    fn hists__init() -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
