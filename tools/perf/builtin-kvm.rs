// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/builtin-kvm.c.
// C include dependencies are represented by extern declarations and opaque
// types below; feature-gated C preprocessor regions are preserved with cfgs
// where a direct local mapping is possible.

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

type u64 = u64;
type s64 = i64;
type int64_t = i64;
type uint16_t = u16;
type size_t = usize;
type bool_ = bool;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ESRCH: c_int = 3;
const EAGAIN: c_int = 11;
const ENOTSUP: c_int = 95;
const O_NONBLOCK: c_int = 0o4000;
const F_GETFL: c_int = 3;
const F_SETFL: c_int = 4;
const POLLIN: c_int = 0x001;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;
const TCSAFLUSH: c_int = 2;
const CLOCK_MONOTONIC: c_int = 1;
const TFD_NONBLOCK: c_int = O_NONBLOCK;
const PERF_DATA_MODE_READ: c_int = 0;
const PERF_DATA_MODE_WRITE: c_int = 1;
const PARSE_OPT_HIDDEN: c_int = 1;
const PARSE_OPT_DISABLED: c_int = 2;
const PARSE_OPT_STOP_AT_NON_OPTION: c_int = 4;
const EM_HOST: uint16_t = 0;
const INVALID_KEY: u64 = !0;
const DEFAULT_VCPU_NUM: c_int = 8;
const KVM_EVENT_NAME_LEN: c_int = 40;
const NSEC_PER_USEC: u64 = 1000;
const STRERR_BUFSIZE: usize = 128;
const BUFSIZ: usize = 8192;
const OE_FLUSH__ROUND: c_int = 0;
const ULLONG_MAX: u64 = u64::MAX;
const CONSOLE_CLEAR: *const c_char = b"\x1b[H\x1b[J\0".as_ptr() as *const c_char;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rb_node { _private: [u8; 0] }
#[repr(C)] pub struct rb_root_cached { _private: [u8; 0] }
#[repr(C)] pub struct perf_hpp { pub buf: *mut c_char, pub size: size_t }
#[repr(C)] pub struct hists {
    pub entries: rb_root_cached,
    pub entries_collapsed: rb_root_cached,
    pub entries_in: *mut rb_root_cached,
    pub nr_entries: u64,
}
#[repr(C)] pub struct perf_hpp_list { pub nr_header_lines: c_int }
#[repr(C)] pub struct perf_hpp_fmt {
    pub list: list_head,
    pub sort_list: list_head,
    pub cmp: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut hist_entry, *mut hist_entry) -> int64_t>,
    pub sort: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut hist_entry, *mut hist_entry) -> int64_t>,
    pub color: *mut c_void,
    pub entry: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hist_entry) -> c_int>,
    pub header: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hists, c_int, *mut c_int) -> c_int>,
    pub width: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hists) -> c_int>,
    pub collapse: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut hist_entry, *mut hist_entry) -> int64_t>,
    pub equal: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp_fmt) -> bool>,
    pub free: Option<unsafe extern "C" fn(*mut perf_hpp_fmt)>,
}
#[repr(C)] pub struct stats { pub n: u64, pub mean: u64, pub max: u64, pub min: u64 }
#[repr(C)] pub struct kvm_event_stats { pub time: u64, pub stats: stats }
#[repr(C)] pub struct kvm_info { pub name: *mut c_char }
#[repr(C)] pub struct hist_entry {
    pub rb_node: rb_node,
    pub rb_node_in: rb_node,
    pub filtered: c_int,
    pub hists: *mut hists,
    pub kvm_info: *mut kvm_info,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct event_key { pub key: u64, pub exit_reasons: *mut c_void }
#[repr(C)] pub struct kvm_event {
    pub he: hist_entry,
    pub perf_kvm: *mut perf_kvm_stat,
    pub key: event_key,
    pub total: kvm_event_stats,
    pub vcpu: *mut kvm_event_stats,
    pub max_vcpu: c_int,
}
#[repr(C)] pub struct kvm_hists { pub hists: hists, pub list: perf_hpp_list }
#[repr(C)] pub struct kvm_dimension {
    pub name: *const c_char,
    pub header: *const c_char,
    pub width: c_int,
    pub cmp: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut hist_entry, *mut hist_entry) -> int64_t>,
    pub entry: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hist_entry) -> c_int>,
}
#[repr(C)] pub struct kvm_fmt { pub fmt: perf_hpp_fmt, pub dim: *mut kvm_dimension }
#[repr(C)] pub struct perf_sample { pub time: u64, pub pid: c_int, pub tid: c_int, pub file_offset: u64, pub evsel: *mut evsel }
#[repr(C)] pub struct perf_event_header { pub type_: c_uint }
#[repr(C)] pub union perf_event { pub header: core::mem::ManuallyDrop<perf_event_header> }
#[repr(C)] pub struct evsel_core { pub attr: perf_event_attr }
#[repr(C)] pub struct evsel { pub core: evsel_core }
#[repr(C)] pub struct perf_event_attr {
    pub mmap: c_uint, pub comm: c_uint, pub task: c_uint, pub sample_period: u64,
    pub watermark: c_uint, pub wakeup_events: c_uint, pub disabled: c_uint,
}
#[repr(C)] pub struct thread { _private: [u8; 0] }
#[repr(C)] pub struct machine { _private: [u8; 0] }
#[repr(C)] pub struct maps { _private: [u8; 0] }
#[repr(C)] pub struct perf_env { pub cpuid: *mut c_char }
#[repr(C)] pub struct perf_data { pub path: *const c_char, pub mode: c_int, pub force: bool }
#[repr(C)] pub struct ordered_events { pub next_flush: u64 }
#[repr(C)] pub struct machines { pub host: machine }
#[repr(C)] pub struct perf_session { pub ordered_events: ordered_events, pub evlist: *mut evlist, pub machines: machines }
#[repr(C)] pub struct perf_tool {
    pub sample: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    pub comm: *mut c_void,
    pub namespaces: *mut c_void,
    pub exit: *mut c_void,
    pub fork: *mut c_void,
    pub lost: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
}
#[repr(C)] pub struct target { pub system_wide: bool, pub pid: *mut c_char, pub uses_mmap: bool }
#[repr(C)] pub struct record_opts { pub target: target, pub mmap_pages: c_uint, pub user_interval: c_uint }
#[repr(C)] pub struct addr_location { _private: [u8; 0] }
#[repr(C)] pub struct intlist { _private: [u8; 0] }
#[repr(C)] pub struct perf_kvm_stat {
    pub file_name: *const c_char,
    pub trace_vcpu: c_int,
    pub report_event: *const c_char,
    pub sort_key: *const c_char,
    pub force: bool,
    pub use_stdio: bool,
    pub live: bool,
    pub duration: u64,
    pub display_time: c_uint,
    pub timerfd: c_int,
    pub total_count: u64,
    pub total_time: u64,
    pub lost_events: u64,
    pub exit_reasons: *mut c_void,
    pub events_ops: *const kvm_events_ops,
    pub tool: perf_tool,
    pub session: *mut perf_session,
    pub evlist: *mut evlist,
    pub opts: record_opts,
    pub al: addr_location,
    pub pid_list: *mut intlist,
}
#[repr(C)] pub struct child_event_ops {
    pub name: *const c_char,
    pub get_key: Option<unsafe extern "C" fn(*mut perf_sample, *mut event_key)>,
}
#[repr(C)] pub struct kvm_events_ops {
    pub name: *const c_char,
    pub child_ops: *const child_event_ops,
    pub decode_key: Option<unsafe extern "C" fn(*mut perf_kvm_stat, *const event_key, *mut c_char)>,
    pub is_begin_event: Option<unsafe extern "C" fn(*mut perf_sample, *mut event_key) -> bool>,
    pub is_end_event: Option<unsafe extern "C" fn(*mut perf_sample, *mut event_key) -> bool>,
}
#[repr(C)] pub struct kvm_reg_events_ops { pub name: *const c_char, pub ops: *const kvm_events_ops }
#[repr(C)] pub struct hist_entry_ops {
    pub new: Option<unsafe extern "C" fn(size_t) -> *mut c_void>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
}
#[repr(C)] pub struct ui_progress { _private: [u8; 0] }
#[repr(C)] pub struct hist_browser {
    pub hists: *mut hists,
    pub nr_non_filtered_entries: u64,
    pub title: Option<unsafe extern "C" fn(*mut hist_browser, *mut c_char, size_t) -> c_int>,
}
#[repr(C)] pub struct perf_cpu { pub cpu: c_int }
#[repr(C)] pub struct evlist { _private: [u8; 0] }
#[repr(C)] pub struct evlist_core { pub nr_mmaps: c_int, pub pollfd: fdarray, pub threads: *mut c_void }
#[repr(C)] pub struct mmap { pub core: perf_mmap }
#[repr(C)] pub struct perf_mmap { _private: [u8; 0] }
#[repr(C)] pub struct fdarray { pub entries: *mut pollfd }
#[repr(C)] pub struct pollfd { pub fd: c_int, pub events: i16, pub revents: i16 }
#[repr(C)] pub struct option { _private: [u8; 0] }
#[repr(C)] pub struct symbol_conf_t {
    pub guestmount: *mut c_char,
    pub default_guest_vmlinux_name: *mut c_char,
    pub default_guest_kallsyms: *mut c_char,
    pub default_guest_modules: *mut c_char,
    pub guest_code: bool,
}
#[repr(C)] pub struct timeval { pub tv_sec: c_long, pub tv_usec: c_long }
#[repr(C)] pub struct tm { _private: [u8; 0] }
#[repr(C)] pub struct itimerspec { pub it_interval: timespec, pub it_value: timespec }
#[repr(C)] pub struct timespec { pub tv_sec: c_long, pub tv_nsec: c_long }
#[repr(C)] pub struct termios { _private: [u8; 0] }

unsafe extern "C" {
    static mut perf_host: c_int;
    static mut perf_guest: c_int;
    static mut exclude_GH_default: bool;
    static mut use_browser: c_int;
    static mut verbose: c_int;
    static mut proc_map_timeout: c_uint;
    static mut symbol_conf: symbol_conf_t;
    static mut record_options: *mut option;
    static mut record_usage: *const *const c_char;
    static mut stdin: *mut c_void;
    static perf_event__process_comm: *mut c_void;
    static perf_event__process_namespaces: *mut c_void;
    static perf_event__process_exit: *mut c_void;
    static perf_event__process_fork: *mut c_void;

    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtok_r(s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn puts(s: *const c_char) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn localtime_r(timep: *const c_long, result: *mut tm) -> *mut tm;
    fn strftime(s: *mut c_char, max: size_t, format: *const c_char, tm: *const tm) -> size_t;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> isize;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_long;
    fn getc(stream: *mut c_void) -> c_int;
    fn fileno(stream: *mut c_void) -> c_int;
    fn signal(sig: c_int, handler: unsafe extern "C" fn(c_int)) -> *mut c_void;
    fn tcsetattr(fd: c_int, optional_actions: c_int, termios_p: *const termios) -> c_int;
    fn timerfd_create(clockid: c_int, flags: c_int) -> c_int;
    fn timerfd_settime(fd: c_int, flags: c_int, new_value: *const itimerspec, old_value: *mut itimerspec) -> c_int;
    static mut errno: c_int;
}

unsafe extern "C" {
    fn rb_first_cached(root: *const rb_root_cached) -> *mut rb_node;
    fn rb_next(node: *const rb_node) -> *mut rb_node;
    fn zalloc(size: size_t) -> *mut c_void;
    fn init_stats(stats: *mut stats);
    fn update_stats(stats: *mut stats, val: u64);
    fn stddev_stats(stats: *const stats) -> c_double;
    fn avg_stats(stats: *const stats) -> c_double;
    fn rel_stddev_stats(stddev: c_double, avg: c_double) -> c_double;
    fn __hists__init(hists: *mut hists, list: *mut perf_hpp_list);
    fn hists__inc_nr_samples(hists: *mut hists, idx: c_int);
    fn hists__has(hists: *mut hists, what: c_int) -> bool;
    fn hists__add_entry_ops(hists: *mut hists, ops: *mut hist_entry_ops, al: *mut addr_location,
                            a: *mut c_void, b: *mut c_void, c: *mut c_void, ki: *mut kvm_info,
                            sample: *mut perf_sample, sample_self: bool) -> *mut hist_entry;
    fn hists__collapse_resort(hists: *mut hists, cb: *mut c_void);
    fn hists__output_resort_cb(hists: *mut hists, cb1: *mut c_void,
                               cb: unsafe extern "C" fn(*mut hist_entry, *mut c_void) -> c_int);
    fn hists__delete_entries(hists: *mut hists);
    fn perf_hpp_list__init(list: *mut perf_hpp_list);
    fn perf_hpp_list__column_register(list: *mut perf_hpp_list, fmt: *mut perf_hpp_fmt);
    fn perf_hpp_list__register_sort_field(list: *mut perf_hpp_list, fmt: *mut perf_hpp_fmt);
    fn perf_hpp__setup_output_field(list: *mut perf_hpp_list);
    fn perf_hpp__append_sort_keys(list: *mut perf_hpp_list);
    fn perf_hpp__reset_output_field(list: *mut perf_hpp_list);
    fn ui_progress__init(prog: *mut ui_progress, total: u64, title: *const c_char);
    fn ui_progress__finish();
    fn kvm_info__new() -> *mut kvm_info;
    fn kvm_reg_events_ops(e_machine: uint16_t) -> *const kvm_reg_events_ops;
    fn kvm_skip_events(e_machine: uint16_t) -> *const *const c_char;
    fn kvm_events_tp(e_machine: uint16_t) -> *const *const c_char;
    fn setup_kvm_events_tp(kvm: *mut perf_kvm_stat, e_machine: uint16_t) -> c_int;
    fn kvm_entry_event(evsel: *mut evsel) -> bool;
    fn vcpu_id_str(e_machine: uint16_t) -> *const c_char;
    fn perf_sample__intval(sample: *mut perf_sample, name: *const c_char) -> c_int;
    fn evsel__name_is(evsel: *mut evsel, name: *const c_char) -> bool;
    fn thread__priv(thread: *mut thread) -> *mut c_void;
    fn thread__set_priv(thread: *mut thread, priv_: *mut c_void);
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn thread__e_machine(thread: *mut thread, machine: *mut machine, e_flags: *mut c_void) -> uint16_t;
    fn thread__put(thread: *mut thread);
    fn thread__set_priv_destructor(dtor: unsafe extern "C" fn(*mut c_void));
    fn maps__machine(maps: *mut maps) -> *mut machine;
    fn machine__resolve(machine: *mut machine, al: *mut addr_location, sample: *mut perf_sample) -> c_int;
    fn machine__findnew_thread(machine: *mut machine, pid: c_int, tid: c_int) -> *mut thread;
    fn machine__synthesize_threads(machine: *mut machine, target: *mut target, threads: *mut c_void, a: bool, b: bool, c: c_int);
    fn addr_location__exit(al: *mut addr_location);
    fn perf_event__name(type_: c_uint) -> *const c_char;
    fn perf_tool__init(tool: *mut perf_tool, ordered_events: bool);
    fn perf_session__new(file: *mut perf_data, tool: *mut perf_tool) -> *mut perf_session;
    fn perf_session__delete(session: *mut perf_session);
    fn perf_session__env(session: *mut perf_session) -> *mut perf_env;
    fn perf_session__has_traces(session: *mut perf_session, msg: *const c_char) -> bool;
    fn perf_session__e_machine(session: *mut perf_session, e_flags: *mut c_void) -> uint16_t;
    fn perf_session__process_events(session: *mut perf_session) -> c_int;
    fn perf_session__queue_event(session: *mut perf_session, event: *mut perf_event, timestamp: u64,
                                 file_offset: u64, file_path: *mut c_void) -> c_int;
    fn perf_session__set_id_hdr_size(session: *mut perf_session);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn symbol__init(env: *mut perf_env);
    fn disable_buildid_cache();
    fn get_cpuid(buf: *mut c_char, size: size_t, cpu: perf_cpu) -> c_int;
    fn str_error_r(err: c_int, buf: *mut c_char, buflen: size_t) -> *mut c_char;
    fn cpu_isa_init(kvm: *mut perf_kvm_stat, e_machine: uint16_t, cpuid: *mut c_char) -> c_int;
    fn intlist__new(s: *const c_char) -> *mut intlist;
    fn intlist__find(list: *mut intlist, i: c_int) -> *mut c_void;
    fn setup_pager();
    fn setup_browser(fallback_to_pager: bool);
    fn set_term_quiet_input(save: *mut termios);
    fn SLang_reset_tty();
    fn SLang_init_tty(a: c_int, b: c_int, c: c_int);
    fn hist_browser__new(hists: *mut hists) -> *mut hist_browser;
    fn hist_browser__run(browser: *mut hist_browser, help: *const c_char, warn_lost_event: bool, delay_secs: c_int) -> c_int;
    fn hist_browser__delete(browser: *mut hist_browser);
    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn evlist__config(evlist: *mut evlist, opts: *mut record_opts, callchain_param: *mut c_void);
    fn evlist__open(evlist: *mut evlist) -> c_int;
    fn evlist__close(evlist: *mut evlist);
    fn evlist__do_mmap(evlist: *mut evlist, pages: c_uint) -> c_int;
    fn evlist__mmap(evlist: *mut evlist) -> *mut mmap;
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__parse_sample_timestamp(evlist: *mut evlist, event: *mut perf_event, timestamp: *mut u64) -> c_int;
    fn evlist__add_pollfd(evlist: *mut evlist, fd: c_int) -> c_int;
    fn evlist__enable(evlist: *mut evlist);
    fn evlist__disable(evlist: *mut evlist);
    fn evlist__poll(evlist: *mut evlist, timeout: c_int) -> c_int;
    fn evlist__add_newtp(evlist: *mut evlist, sys: *const c_char, name: *const c_char, err: *mut c_void) -> c_int;
    fn evlist__create_maps(evlist: *mut evlist, target: *mut target) -> c_int;
    fn evsel__set_sample_bit(evsel: *mut evsel, bit: c_int);
    fn evsel__reset_sample_bit(evsel: *mut evsel, bit: c_int);
    fn perf_mmap__read_init(map: *mut perf_mmap) -> c_int;
    fn perf_mmap__read_event(map: *mut perf_mmap) -> *mut perf_event;
    fn perf_mmap__consume(map: *mut perf_mmap);
    fn perf_mmap__read_done(map: *mut perf_mmap);
    fn ordered_events__flush(oe: *mut ordered_events, how: c_int) -> c_int;
    fn ordered_events__set_copy_on_queue(oe: *mut ordered_events, copy: bool);
    fn target__validate(target: *mut target) -> c_int;
    fn target__strerror(target: *mut target, err: c_int, buf: *mut c_char, size: size_t);
    fn target__none(target: *mut target) -> bool;
    fn ui__warning(fmt: *const c_char, ...);
    fn ui__error(fmt: *const c_char, ...);
    fn parse_options(argc: c_int, argv: *const *const c_char, options: *const option,
                     usagestr: *const *const c_char, flags: c_int) -> c_int;
    fn parse_options_subcommand(argc: c_int, argv: *const *const c_char, options: *const option,
                                subcommands: *const *const c_char, usagestr: *mut *const c_char,
                                flags: c_int) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option) -> !;
    fn set_option_flag(options: *mut option, short_name: c_int, long_name: *const c_char, flags: c_int);
    fn cmd_record(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_report(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_buildid_list(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_top(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_stat(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_diff(argc: c_int, argv: *const *const c_char) -> c_int;
    fn kvm_need_default_arch_event(e_machine: uint16_t, argc: c_int, argv: *const *const c_char) -> c_int;
    fn kvm_add_default_arch_event(e_machine: uint16_t, i: *mut c_int, argv: *mut *const c_char) -> c_int;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;
}

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char }; }
macro_rules! BUG_ON { ($cond:expr) => { if $cond { core::panic!("BUG_ON") } }; }
macro_rules! ARRAY_SIZE { ($arr:expr) => { ($arr).len() }; }

unsafe fn container_of<T, U>(ptr: *mut U, offset: usize) -> *mut T {
    (ptr as *mut u8).sub(offset) as *mut T
}
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {{
        let uninit = core::mem::MaybeUninit::<$ty>::uninit();
        let base = uninit.as_ptr();
        unsafe { core::ptr::addr_of!((*base).$field) as usize - base as usize }
    }};
}

#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn get_event_time(event: *mut kvm_event, vcpu: c_int) -> u64 {
    if vcpu == -1 { return (*event).total.time; }
    if vcpu >= (*event).max_vcpu { return 0; }
    (*(*event).vcpu.add(vcpu as usize)).time
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn get_event_max(event: *mut kvm_event, vcpu: c_int) -> u64 {
    if vcpu == -1 { return (*event).total.stats.max; }
    if vcpu >= (*event).max_vcpu { return 0; }
    (*(*event).vcpu.add(vcpu as usize)).stats.max
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn get_event_min(event: *mut kvm_event, vcpu: c_int) -> u64 {
    if vcpu == -1 { return (*event).total.stats.min; }
    if vcpu >= (*event).max_vcpu { return 0; }
    (*(*event).vcpu.add(vcpu as usize)).stats.min
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn get_event_count(event: *mut kvm_event, vcpu: c_int) -> u64 {
    if vcpu == -1 { return (*event).total.stats.n; }
    if vcpu >= (*event).max_vcpu { return 0; }
    (*(*event).vcpu.add(vcpu as usize)).stats.n
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn get_event_mean(event: *mut kvm_event, vcpu: c_int) -> u64 {
    if vcpu == -1 { return (*event).total.stats.mean; }
    if vcpu >= (*event).max_vcpu { return 0; }
    (*(*event).vcpu.add(vcpu as usize)).stats.mean
}
#[cfg(feature = "libtraceevent")] unsafe extern "C" fn cmp_event_time(one: *mut kvm_event, two: *mut kvm_event, vcpu: c_int) -> int64_t { get_event_time(one, vcpu) as i64 - get_event_time(two, vcpu) as i64 }
#[cfg(feature = "libtraceevent")] unsafe extern "C" fn cmp_event_max(one: *mut kvm_event, two: *mut kvm_event, vcpu: c_int) -> int64_t { get_event_max(one, vcpu) as i64 - get_event_max(two, vcpu) as i64 }
#[cfg(feature = "libtraceevent")] unsafe extern "C" fn cmp_event_min(one: *mut kvm_event, two: *mut kvm_event, vcpu: c_int) -> int64_t { get_event_min(one, vcpu) as i64 - get_event_min(two, vcpu) as i64 }
#[cfg(feature = "libtraceevent")] unsafe extern "C" fn cmp_event_count(one: *mut kvm_event, two: *mut kvm_event, vcpu: c_int) -> int64_t { get_event_count(one, vcpu) as i64 - get_event_count(two, vcpu) as i64 }
#[cfg(feature = "libtraceevent")] unsafe extern "C" fn cmp_event_mean(one: *mut kvm_event, two: *mut kvm_event, vcpu: c_int) -> int64_t { get_event_mean(one, vcpu) as i64 - get_event_mean(two, vcpu) as i64 }

#[cfg(feature = "libtraceevent")]
static mut kvm_hists: kvm_hists = unsafe { zeroed() };

#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn ev_name_cmp(fmt: *mut perf_hpp_fmt, left: *mut hist_entry, right: *mut hist_entry) -> int64_t {
    /* Return opposite number for sorting in alphabetical order */
    -strcmp((*(*left).kvm_info).name, (*(*right).kvm_info).name) as int64_t
}

#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn fmt_width(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, hists: *mut hists) -> c_int {
    let kvm_fmt = container_of::<kvm_fmt, perf_hpp_fmt>(fmt, offset_of!(kvm_fmt, fmt));
    (*(*kvm_fmt).dim).width
}

#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn ev_name_entry(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
    let width = fmt_width(fmt, hpp, (*he).hists);
    scnprintf((*hpp).buf, (*hpp).size, cstr!("%*s"), width, (*(*he).kvm_info).name)
}

#[cfg(feature = "libtraceevent")] static mut dim_event: kvm_dimension = kvm_dimension { header: cstr!("Event name"), name: cstr!("ev_name"), cmp: Some(ev_name_cmp), entry: Some(ev_name_entry), width: 40 };

#[cfg(feature = "libtraceevent")]
unsafe fn ev_cmp_metric(left: *mut hist_entry, right: *mut hist_entry, cmp: unsafe extern "C" fn(*mut kvm_event, *mut kvm_event, c_int) -> int64_t) -> int64_t {
    let event_left = container_of::<kvm_event, hist_entry>(left, offset_of!(kvm_event, he));
    let event_right = container_of::<kvm_event, hist_entry>(right, offset_of!(kvm_event, he));
    let perf_kvm = (*event_left).perf_kvm;
    cmp(event_left, event_right, (*perf_kvm).trace_vcpu)
}
#[cfg(feature = "libtraceevent")] unsafe extern "C" fn ev_cmp_time(fmt: *mut perf_hpp_fmt, left: *mut hist_entry, right: *mut hist_entry) -> int64_t { ev_cmp_metric(left, right, cmp_event_time) }
#[cfg(feature = "libtraceevent")] unsafe extern "C" fn ev_cmp_count(fmt: *mut perf_hpp_fmt, left: *mut hist_entry, right: *mut hist_entry) -> int64_t { ev_cmp_metric(left, right, cmp_event_count) }
#[cfg(feature = "libtraceevent")] unsafe extern "C" fn ev_cmp_max(fmt: *mut perf_hpp_fmt, left: *mut hist_entry, right: *mut hist_entry) -> int64_t { ev_cmp_metric(left, right, cmp_event_max) }
#[cfg(feature = "libtraceevent")] unsafe extern "C" fn ev_cmp_min(fmt: *mut perf_hpp_fmt, left: *mut hist_entry, right: *mut hist_entry) -> int64_t { ev_cmp_metric(left, right, cmp_event_min) }
#[cfg(feature = "libtraceevent")] unsafe extern "C" fn ev_cmp_mean(fmt: *mut perf_hpp_fmt, left: *mut hist_entry, right: *mut hist_entry) -> int64_t { ev_cmp_metric(left, right, cmp_event_mean) }

#[cfg(feature = "libtraceevent")]
unsafe fn ev_entry_metric(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry, get: unsafe extern "C" fn(*mut kvm_event, c_int) -> u64) -> c_int {
    let event = container_of::<kvm_event, hist_entry>(he, offset_of!(kvm_event, he));
    let width = fmt_width(fmt, hpp, (*he).hists);
    let perf_kvm = (*event).perf_kvm;
    scnprintf((*hpp).buf, (*hpp).size, cstr!("%*lu"), width, get(event, (*perf_kvm).trace_vcpu) as c_ulong)
}
#[cfg(feature = "libtraceevent")] unsafe extern "C" fn ev_entry_time(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int { ev_entry_metric(fmt, hpp, he, get_event_time) }
#[cfg(feature = "libtraceevent")] unsafe extern "C" fn ev_entry_count(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int { ev_entry_metric(fmt, hpp, he, get_event_count) }
#[cfg(feature = "libtraceevent")] unsafe extern "C" fn ev_entry_max(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int { ev_entry_metric(fmt, hpp, he, get_event_max) }
#[cfg(feature = "libtraceevent")] unsafe extern "C" fn ev_entry_min(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int { ev_entry_metric(fmt, hpp, he, get_event_min) }

#[cfg(feature = "libtraceevent")] static mut dim_time: kvm_dimension = kvm_dimension { header: cstr!("Time (ns)"), name: cstr!("time"), cmp: Some(ev_cmp_time), entry: Some(ev_entry_time), width: 12 };
#[cfg(feature = "libtraceevent")] static mut dim_count: kvm_dimension = kvm_dimension { header: cstr!("Samples"), name: cstr!("sample"), cmp: Some(ev_cmp_count), entry: Some(ev_entry_count), width: 12 };
#[cfg(feature = "libtraceevent")] static mut dim_max_time: kvm_dimension = kvm_dimension { header: cstr!("Max Time (ns)"), name: cstr!("max_t"), cmp: Some(ev_cmp_max), entry: Some(ev_entry_max), width: 14 };
#[cfg(feature = "libtraceevent")] static mut dim_min_time: kvm_dimension = kvm_dimension { header: cstr!("Min Time (ns)"), name: cstr!("min_t"), cmp: Some(ev_cmp_min), entry: Some(ev_entry_min), width: 14 };

#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn ev_entry_mean(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
    ev_entry_metric(fmt, hpp, he, get_event_mean)
}
#[cfg(feature = "libtraceevent")] static mut dim_mean_time: kvm_dimension = kvm_dimension { header: cstr!("Mean Time (ns)"), name: cstr!("mean_t"), cmp: Some(ev_cmp_mean), entry: Some(ev_entry_mean), width: 14 };

#[cfg(feature = "libtraceevent")]
unsafe fn PERC_STR(s: *mut c_char, v: c_double) -> *mut c_char {
    scnprintf(s, 10, cstr!("%.2F%%"), v);
    s
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn percent(st: u64, tot: u64) -> c_double {
    if tot != 0 { 100.0 * st as c_double / tot as c_double } else { 0.0 }
}
#[cfg(feature = "libtraceevent")] unsafe extern "C" fn ev_percent_time(he: *mut hist_entry) -> c_int {
    let event = container_of::<kvm_event, hist_entry>(he, offset_of!(kvm_event, he));
    let perf_kvm = (*event).perf_kvm;
    percent(get_event_time(event, (*perf_kvm).trace_vcpu), (*perf_kvm).total_time) as c_int
}
#[cfg(feature = "libtraceevent")] unsafe extern "C" fn ev_percent_count(he: *mut hist_entry) -> c_int {
    let event = container_of::<kvm_event, hist_entry>(he, offset_of!(kvm_event, he));
    let perf_kvm = (*event).perf_kvm;
    percent(get_event_count(event, (*perf_kvm).trace_vcpu), (*perf_kvm).total_count) as c_int
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn ev_entry_time_precent(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
    let width = fmt_width(fmt, hpp, (*he).hists);
    let mut buf = [0 as c_char; 10];
    let per = ev_percent_time(he) as c_double;
    scnprintf((*hpp).buf, (*hpp).size, cstr!("%*s"), width, PERC_STR(buf.as_mut_ptr(), per))
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn ev_cmp_time_precent(fmt: *mut perf_hpp_fmt, left: *mut hist_entry, right: *mut hist_entry) -> int64_t {
    ev_percent_time(left) as int64_t - ev_percent_time(right) as int64_t
}
#[cfg(feature = "libtraceevent")] static mut dim_time_percent: kvm_dimension = kvm_dimension { header: cstr!("Time%"), name: cstr!("percent_time"), cmp: Some(ev_cmp_time_precent), entry: Some(ev_entry_time_precent), width: 12 };
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn ev_entry_count_precent(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
    let width = fmt_width(fmt, hpp, (*he).hists);
    let mut buf = [0 as c_char; 10];
    let per = ev_percent_count(he) as c_double;
    scnprintf((*hpp).buf, (*hpp).size, cstr!("%*s"), width, PERC_STR(buf.as_mut_ptr(), per))
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn ev_cmp_count_precent(fmt: *mut perf_hpp_fmt, left: *mut hist_entry, right: *mut hist_entry) -> int64_t {
    ev_percent_count(left) as int64_t - ev_percent_count(right) as int64_t
}
#[cfg(feature = "libtraceevent")] static mut dim_count_percent: kvm_dimension = kvm_dimension { header: cstr!("Sample%"), name: cstr!("percent_sample"), cmp: Some(ev_cmp_count_precent), entry: Some(ev_entry_count_precent), width: 12 };

#[cfg(feature = "libtraceevent")]
static mut dimensions: [*mut kvm_dimension; 9] = unsafe {
    [&raw mut dim_event, &raw mut dim_time, &raw mut dim_time_percent, &raw mut dim_count,
     &raw mut dim_count_percent, &raw mut dim_max_time, &raw mut dim_min_time,
     &raw mut dim_mean_time, null_mut()]
};

#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn fmt_header(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, hists: *mut hists, line: c_int, span: *mut c_int) -> c_int {
    let kvm_fmt = container_of::<kvm_fmt, perf_hpp_fmt>(fmt, offset_of!(kvm_fmt, fmt));
    let dim = (*kvm_fmt).dim;
    let width = fmt_width(fmt, hpp, hists);
    scnprintf((*hpp).buf, (*hpp).size, cstr!("%*s"), width, (*dim).header)
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn fmt_equal(a: *mut perf_hpp_fmt, b: *mut perf_hpp_fmt) -> bool {
    let kvm_fmt_a = container_of::<kvm_fmt, perf_hpp_fmt>(a, offset_of!(kvm_fmt, fmt));
    let kvm_fmt_b = container_of::<kvm_fmt, perf_hpp_fmt>(b, offset_of!(kvm_fmt, fmt));
    (*kvm_fmt_a).dim == (*kvm_fmt_b).dim
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn fmt_free(fmt: *mut perf_hpp_fmt) {
    let kvm_fmt = container_of::<kvm_fmt, perf_hpp_fmt>(fmt, offset_of!(kvm_fmt, fmt));
    free(kvm_fmt as *mut c_void);
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn get_dimension(name: *const c_char) -> *mut kvm_dimension {
    let mut i = 0usize;
    while !dimensions[i].is_null() {
        if strcmp((*dimensions[i]).name, name) == 0 { return dimensions[i]; }
        i += 1;
    }
    null_mut()
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn get_format(name: *const c_char) -> *mut kvm_fmt {
    let dim = get_dimension(name);
    if dim.is_null() { return null_mut(); }
    let kvm_fmt = zalloc(size_of::<kvm_fmt>()) as *mut kvm_fmt;
    if kvm_fmt.is_null() { return null_mut(); }
    (*kvm_fmt).dim = dim;
    let fmt = &mut (*kvm_fmt).fmt;
    fmt.list.next = &mut fmt.list; fmt.list.prev = &mut fmt.list;
    fmt.sort_list.next = &mut fmt.sort_list; fmt.sort_list.prev = &mut fmt.sort_list;
    fmt.cmp = (*dim).cmp; fmt.sort = (*dim).cmp; fmt.color = null_mut(); fmt.entry = (*dim).entry;
    fmt.header = Some(fmt_header); fmt.width = Some(fmt_width); fmt.collapse = (*dim).cmp;
    fmt.equal = Some(fmt_equal); fmt.free = Some(fmt_free);
    kvm_fmt
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn kvm_hists__init_output(hpp_list: *mut perf_hpp_list, name: *mut c_char) -> c_int {
    let kvm_fmt = get_format(name);
    if kvm_fmt.is_null() { pr_warning(cstr!("Fail to find format for output field %s.\n"), name); return -EINVAL; }
    perf_hpp_list__column_register(hpp_list, &mut (*kvm_fmt).fmt);
    0
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn kvm_hists__init_sort(hpp_list: *mut perf_hpp_list, name: *mut c_char) -> c_int {
    let kvm_fmt = get_format(name);
    if kvm_fmt.is_null() { pr_warning(cstr!("Fail to find format for sorting %s.\n"), name); return -EINVAL; }
    perf_hpp_list__register_sort_field(hpp_list, &mut (*kvm_fmt).fmt);
    0
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn kvm_hpp_list__init(list: *mut c_char, hpp_list: *mut perf_hpp_list,
                                        fn_: Option<unsafe extern "C" fn(*mut perf_hpp_list, *mut c_char) -> c_int>) -> c_int {
    let mut tmp: *mut c_char = null_mut();
    let mut ret = 0;
    if list.is_null() || fn_.is_none() { return 0; }
    let mut tok = strtok_r(list, cstr!(", "), &mut tmp);
    while !tok.is_null() {
        ret = fn_.unwrap()(hpp_list, tok);
        if ret != 0 {
            if ret == -EINVAL { pr_err(cstr!("Invalid field key: '%s'"), tok); }
            else if ret == -ESRCH { pr_err(cstr!("Unknown field key: '%s'"), tok); }
            else { pr_err(cstr!("Fail to initialize for field key: '%s'"), tok); }
            break;
        }
        tok = strtok_r(null_mut(), cstr!(", "), &mut tmp);
    }
    ret
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn kvm_hpp_list__parse(hpp_list: *mut perf_hpp_list, output_: *const c_char, sort_: *const c_char) -> c_int {
    let output = if !output_.is_null() { strdup(output_) } else { null_mut() };
    let sort = if !sort_.is_null() { strdup(sort_) } else { null_mut() };
    let mut ret = kvm_hpp_list__init(output, hpp_list, Some(kvm_hists__init_output));
    if ret == 0 {
        ret = kvm_hpp_list__init(sort, hpp_list, Some(kvm_hists__init_sort));
        if ret == 0 {
            /* Copy sort keys to output fields */
            perf_hpp__setup_output_field(hpp_list);
            /* and then copy output fields to sort keys */
            perf_hpp__append_sort_keys(hpp_list);
        }
    }
    free(output as *mut c_void); free(sort as *mut c_void); ret
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn kvm_hists__init() -> c_int {
    kvm_hists.list.nr_header_lines = 1;
    __hists__init(&mut kvm_hists.hists, &mut kvm_hists.list);
    perf_hpp_list__init(&mut kvm_hists.list);
    kvm_hpp_list__parse(&mut kvm_hists.list, null(), cstr!("ev_name"))
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn kvm_hists__reinit(output: *const c_char, sort: *const c_char) -> c_int {
    perf_hpp__reset_output_field(&mut kvm_hists.list);
    kvm_hpp_list__parse(&mut kvm_hists.list, output, sort)
}

#[cfg(all(feature = "libtraceevent", feature = "slang"))]
unsafe extern "C" fn kvm_browser__update_nr_entries(hb: *mut hist_browser) {
    let mut nd = rb_first_cached(&(*(*hb).hists).entries);
    let mut nr_entries: u64 = 0;
    while !nd.is_null() {
        let he = container_of::<hist_entry, rb_node>(nd, offset_of!(hist_entry, rb_node));
        if (*he).filtered == 0 { nr_entries += 1; }
        nd = rb_next(nd);
    }
    (*hb).nr_non_filtered_entries = nr_entries;
}
#[cfg(all(feature = "libtraceevent", feature = "slang"))]
unsafe extern "C" fn kvm_browser__title(browser: *mut hist_browser, buf: *mut c_char, size: size_t) -> c_int {
    scnprintf(buf, size, cstr!("KVM event statistics (%lu entries)"), (*browser).nr_non_filtered_entries as c_ulong);
    0
}
#[cfg(all(feature = "libtraceevent", feature = "slang"))]
unsafe extern "C" fn perf_kvm_browser__new(hists_: *mut hists) -> *mut hist_browser {
    let browser = hist_browser__new(hists_);
    if !browser.is_null() { (*browser).title = Some(kvm_browser__title); }
    browser
}
#[cfg(all(feature = "libtraceevent", feature = "slang"))]
unsafe extern "C" fn kvm__hists_browse(hists_: *mut hists) -> c_int {
    let browser = perf_kvm_browser__new(hists_);
    if browser.is_null() { return -1; }
    /* reset abort key so that it can get Ctrl-C as a key */
    SLang_reset_tty(); SLang_init_tty(0, 0, 0);
    kvm_browser__update_nr_entries(browser);
    loop {
        let key = hist_browser__run(browser, cstr!("? - help"), true, 0);
        match key { 113 => break, _ => {} }
    }
    hist_browser__delete(browser); 0
}
#[cfg(all(feature = "libtraceevent", feature = "slang"))]
unsafe extern "C" fn kvm_display(kvm: *mut perf_kvm_stat) {
    if use_browser == 0 { print_result(kvm); } else { kvm__hists_browse(&mut kvm_hists.hists); }
}
#[cfg(all(feature = "libtraceevent", not(feature = "slang")))]
unsafe extern "C" fn kvm_display(kvm: *mut perf_kvm_stat) {
    use_browser = 0; print_result(kvm);
}

unsafe extern "C" fn get_filename_for_perf_kvm() -> *const c_char {
    if perf_host != 0 && perf_guest == 0 { cstr!("perf.data.host") }
    else if perf_host == 0 && perf_guest != 0 { cstr!("perf.data.guest") }
    else { cstr!("perf.data.kvm") }
}

#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn register_kvm_events_ops(kvm: *mut perf_kvm_stat, e_machine: uint16_t) -> bool {
    let mut events_ops = kvm_reg_events_ops(e_machine);
    while !(*events_ops).name.is_null() {
        if strcmp((*events_ops).name, (*kvm).report_event) == 0 {
            (*kvm).events_ops = (*events_ops).ops;
            return true;
        }
        events_ops = events_ops.add(1);
    }
    false
}

#[cfg(feature = "libtraceevent")]
#[repr(C)] struct vcpu_event_record { vcpu_id: c_int, start_time: u64, last_event: *mut kvm_event }

#[cfg(all(feature = "timerfd", feature = "libtraceevent"))]
unsafe extern "C" fn clear_events_cache_stats() {
    let root = if hists__has(&mut kvm_hists.hists, 0) { &mut kvm_hists.hists.entries_collapsed } else { kvm_hists.hists.entries_in };
    let mut nd = rb_first_cached(root);
    while !nd.is_null() {
        let he = container_of::<hist_entry, rb_node>(nd, offset_of!(hist_entry, rb_node_in));
        let event = container_of::<kvm_event, hist_entry>(he, offset_of!(kvm_event, he));
        (*event).total.time = 0; init_stats(&mut (*event).total.stats);
        let mut i = 0;
        while i < (*event).max_vcpu {
            (*(*event).vcpu.add(i as usize)).time = 0;
            init_stats(&mut (*(*event).vcpu.add(i as usize)).stats);
            i += 1;
        }
        nd = rb_next(nd);
    }
}

#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn kvm_event_expand(event: *mut kvm_event, vcpu_id: c_int) -> bool {
    let old_max_vcpu = (*event).max_vcpu;
    if vcpu_id < (*event).max_vcpu { return true; }
    while (*event).max_vcpu <= vcpu_id { (*event).max_vcpu += DEFAULT_VCPU_NUM; }
    let prev = (*event).vcpu;
    (*event).vcpu = realloc((*event).vcpu as *mut c_void, ((*event).max_vcpu as usize) * size_of::<kvm_event_stats>()) as *mut kvm_event_stats;
    if (*event).vcpu.is_null() {
        free(prev as *mut c_void); pr_err(cstr!("Not enough memory\n")); return false;
    }
    memset((*event).vcpu.add(old_max_vcpu as usize) as *mut c_void, 0,
           (((*event).max_vcpu - old_max_vcpu) as usize) * size_of::<kvm_event_stats>());
    true
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn kvm_he_zalloc(size: size_t) -> *mut c_void {
    let kvm_ev = zalloc(size + size_of::<kvm_event>()) as *mut kvm_event;
    if kvm_ev.is_null() { return null_mut(); }
    init_stats(&mut (*kvm_ev).total.stats);
    hists__inc_nr_samples(&mut kvm_hists.hists, 0);
    &mut (*kvm_ev).he as *mut hist_entry as *mut c_void
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn kvm_he_free(he: *mut c_void) {
    let kvm_ev = container_of::<kvm_event, hist_entry>(he as *mut hist_entry, offset_of!(kvm_event, he));
    free(kvm_ev as *mut c_void);
}
#[cfg(feature = "libtraceevent")]
static mut kvm_ev_entry_ops: hist_entry_ops = hist_entry_ops { new: Some(kvm_he_zalloc), free: Some(kvm_he_free) };

#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn find_create_kvm_event(kvm: *mut perf_kvm_stat, key: *mut event_key, sample: *mut perf_sample) -> *mut kvm_event {
    BUG_ON!((*key).key == INVALID_KEY);
    let ki = kvm_info__new();
    if ki.is_null() { pr_err(cstr!("Failed to allocate kvm info\n")); return null_mut(); }
    (*(*kvm).events_ops).decode_key.unwrap()(kvm, key, (*ki).name);
    let he = hists__add_entry_ops(&mut kvm_hists.hists, &mut kvm_ev_entry_ops, &mut (*kvm).al, null_mut(), null_mut(), null_mut(), ki, sample, true);
    if he.is_null() { pr_err(cstr!("Failed to allocate hist entry\n")); free(ki as *mut c_void); return null_mut(); }
    let event = container_of::<kvm_event, hist_entry>(he, offset_of!(kvm_event, he));
    if (*event).perf_kvm.is_null() { (*event).perf_kvm = kvm; (*event).key = *key; }
    event
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn handle_begin_event(kvm: *mut perf_kvm_stat, vcpu_record: *mut vcpu_event_record, key: *mut event_key, sample: *mut perf_sample) -> bool {
    let mut event = null_mut();
    if (*key).key != INVALID_KEY { event = find_create_kvm_event(kvm, key, sample); }
    (*vcpu_record).last_event = event; (*vcpu_record).start_time = (*sample).time; true
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn kvm_update_event_stats(kvm_stats: *mut kvm_event_stats, time_diff: u64) {
    (*kvm_stats).time += time_diff; update_stats(&mut (*kvm_stats).stats, time_diff);
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn kvm_event_rel_stddev(vcpu_id: c_int, event: *mut kvm_event) -> c_double {
    let mut kvm_stats = &mut (*event).total as *mut kvm_event_stats;
    if vcpu_id != -1 { kvm_stats = (*event).vcpu.add(vcpu_id as usize); }
    rel_stddev_stats(stddev_stats(&(*kvm_stats).stats), avg_stats(&(*kvm_stats).stats))
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn update_kvm_event(kvm: *mut perf_kvm_stat, event: *mut kvm_event, vcpu_id: c_int, time_diff: u64) -> bool {
    /* Update overall statistics */
    (*kvm).total_count += 1; (*kvm).total_time += time_diff;
    if vcpu_id == -1 { kvm_update_event_stats(&mut (*event).total, time_diff); return true; }
    if !kvm_event_expand(event, vcpu_id) { return false; }
    kvm_update_event_stats((*event).vcpu.add(vcpu_id as usize), time_diff); true
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn is_child_event(kvm: *mut perf_kvm_stat, sample: *mut perf_sample, key: *mut event_key) -> bool {
    let mut child_ops = (*(*kvm).events_ops).child_ops;
    if child_ops.is_null() { return false; }
    while !(*child_ops).name.is_null() {
        if evsel__name_is((*sample).evsel, (*child_ops).name) {
            (*child_ops).get_key.unwrap()(sample, key); return true;
        }
        child_ops = child_ops.add(1);
    }
    false
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn handle_child_event(kvm: *mut perf_kvm_stat, vcpu_record: *mut vcpu_event_record, key: *mut event_key, sample: *mut perf_sample) -> bool {
    let mut event = null_mut();
    if (*key).key != INVALID_KEY { event = find_create_kvm_event(kvm, key, sample); }
    (*vcpu_record).last_event = event; true
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn skip_event(e_machine: uint16_t, event: *const c_char) -> bool {
    let mut skip_events = kvm_skip_events(e_machine);
    while !(*skip_events).is_null() {
        if strcmp(event, *skip_events) == 0 { return true; }
        skip_events = skip_events.add(1);
    }
    false
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn handle_end_event(kvm: *mut perf_kvm_stat, vcpu_record: *mut vcpu_event_record, key: *mut event_key, sample: *mut perf_sample) -> bool {
    let vcpu = if (*kvm).trace_vcpu == -1 { -1 } else { (*vcpu_record).vcpu_id };
    let mut event = (*vcpu_record).last_event;
    let time_begin = (*vcpu_record).start_time;
    /* The begin event is not caught. */
    if time_begin == 0 { return true; }
    /* Both begin and end events did not get the key. */
    if event.is_null() && (*key).key == INVALID_KEY { return true; }
    if event.is_null() { event = find_create_kvm_event(kvm, key, sample); }
    if event.is_null() { return false; }
    (*vcpu_record).last_event = null_mut(); (*vcpu_record).start_time = 0;
    /* seems to happen once in a while during live mode */
    if (*sample).time < time_begin { pr_debug(cstr!("End time before begin time; skipping event.\n")); return true; }
    let time_diff = (*sample).time - time_begin;
    if (*kvm).duration != 0 && time_diff > (*kvm).duration {
        let mut decode = [0 as c_char; KVM_EVENT_NAME_LEN as usize];
        let e_machine = perf_session__e_machine((*kvm).session, null_mut());
        (*(*kvm).events_ops).decode_key.unwrap()(kvm, &(*event).key, decode.as_mut_ptr());
        if !skip_event(e_machine, decode.as_ptr()) {
            pr_info(cstr!("%lu VM %d, vcpu %d: %s event took %luusec\n"), (*sample).time as c_ulong, (*sample).pid, (*vcpu_record).vcpu_id, decode.as_ptr(), (time_diff / NSEC_PER_USEC) as c_ulong);
        }
    }
    update_kvm_event(kvm, event, vcpu, time_diff)
}

#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn per_vcpu_record(thread: *mut thread, sample: *mut perf_sample) -> *mut vcpu_event_record {
    /* Only kvm_entry records vcpu id. */
    if thread__priv(thread).is_null() && kvm_entry_event((*sample).evsel) {
        let machine = maps__machine(thread__maps(thread));
        let e_machine = thread__e_machine(thread, machine, null_mut());
        let vcpu_record = zalloc(size_of::<vcpu_event_record>()) as *mut vcpu_event_record;
        if vcpu_record.is_null() { pr_err(cstr!("%s: Not enough memory\n"), cstr!("per_vcpu_record")); return null_mut(); }
        (*vcpu_record).vcpu_id = perf_sample__intval(sample, vcpu_id_str(e_machine));
        thread__set_priv(thread, vcpu_record as *mut c_void);
    }
    thread__priv(thread) as *mut vcpu_event_record
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn handle_kvm_event(kvm: *mut perf_kvm_stat, thread: *mut thread, sample: *mut perf_sample) -> bool {
    let mut key = event_key { key: INVALID_KEY, exit_reasons: (*kvm).exit_reasons };
    let vcpu_record = per_vcpu_record(thread, sample);
    if vcpu_record.is_null() { return true; }
    /* only process events for vcpus user cares about */
    if (*kvm).trace_vcpu != -1 && (*kvm).trace_vcpu != (*vcpu_record).vcpu_id { return true; }
    if (*(*kvm).events_ops).is_begin_event.unwrap()(sample, &mut key) { return handle_begin_event(kvm, vcpu_record, &mut key, sample); }
    if is_child_event(kvm, sample, &mut key) { return handle_child_event(kvm, vcpu_record, &mut key, sample); }
    if (*(*kvm).events_ops).is_end_event.unwrap()(sample, &mut key) { return handle_end_event(kvm, vcpu_record, &mut key, sample); }
    true
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn is_valid_key(kvm: *mut perf_kvm_stat) -> bool {
    let key_array = [cstr!("ev_name"), cstr!("sample"), cstr!("time"), cstr!("max_t"), cstr!("min_t"), cstr!("mean_t")];
    for key in key_array {
        if strcmp(key, (*kvm).sort_key) == 0 { return true; }
    }
    pr_err(cstr!("Unsupported sort key: %s\n"), (*kvm).sort_key); false
}
#[cfg(feature = "libtraceevent")] unsafe extern "C" fn event_is_valid(event: *mut kvm_event, vcpu: c_int) -> bool { get_event_count(event, vcpu) != 0 }
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn filter_cb(he: *mut hist_entry, arg: *mut c_void) -> c_int {
    let event = container_of::<kvm_event, hist_entry>(he, offset_of!(kvm_event, he));
    let perf_kvm = (*event).perf_kvm;
    (*he).filtered = if !event_is_valid(event, (*perf_kvm).trace_vcpu) { 1 } else { 0 };
    0
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn sort_result(kvm: *mut perf_kvm_stat) {
    let mut prog: ui_progress = zeroed();
    let output_columns = cstr!("ev_name,sample,percent_sample,time,percent_time,max_t,min_t,mean_t");
    kvm_hists__reinit(output_columns, (*kvm).sort_key);
    ui_progress__init(&mut prog, kvm_hists.hists.nr_entries, cstr!("Sorting..."));
    hists__collapse_resort(&mut kvm_hists.hists, null_mut());
    hists__output_resort_cb(&mut kvm_hists.hists, null_mut(), filter_cb);
    ui_progress__finish();
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn print_vcpu_info(kvm: *mut perf_kvm_stat) {
    let vcpu = (*kvm).trace_vcpu;
    pr_info(cstr!("Analyze events for "));
    if (*kvm).opts.target.system_wide { pr_info(cstr!("all VMs, ")); }
    else if !(*kvm).opts.target.pid.is_null() { pr_info(cstr!("pid(s) %s, "), (*kvm).opts.target.pid); }
    else { pr_info(cstr!("dazed and confused on what is monitored, ")); }
    if vcpu == -1 { pr_info(cstr!("all VCPUs:\n\n")); } else { pr_info(cstr!("VCPU %d:\n\n"), vcpu); }
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn show_timeofday() {
    let mut date = [0 as c_char; 64];
    let mut tv: timeval = zeroed();
    let mut ltime: tm = zeroed();
    gettimeofday(&mut tv, null_mut());
    if !localtime_r(&tv.tv_sec, &mut ltime).is_null() {
        strftime(date.as_mut_ptr(), date.len(), cstr!("%H:%M:%S"), &ltime);
        pr_info(cstr!("%s.%06ld"), date.as_ptr(), tv.tv_usec);
    } else { pr_info(cstr!("00:00:00.000000")); }
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn print_result(kvm: *mut perf_kvm_stat) {
    let mut decode = [0 as c_char; KVM_EVENT_NAME_LEN as usize];
    let vcpu = (*kvm).trace_vcpu;
    if (*kvm).live { puts(CONSOLE_CLEAR); show_timeofday(); }
    pr_info(cstr!("\n\n")); print_vcpu_info(kvm);
    pr_info(cstr!("%*s "), KVM_EVENT_NAME_LEN, (*(*kvm).events_ops).name);
    pr_info(cstr!("%10s "), cstr!("Samples")); pr_info(cstr!("%9s "), cstr!("Samples%"));
    pr_info(cstr!("%9s "), cstr!("Time%")); pr_info(cstr!("%11s "), cstr!("Min Time"));
    pr_info(cstr!("%11s "), cstr!("Max Time")); pr_info(cstr!("%16s "), cstr!("Avg time"));
    pr_info(cstr!("\n\n"));
    let mut nd = rb_first_cached(&kvm_hists.hists.entries);
    while !nd.is_null() {
        let he = container_of::<hist_entry, rb_node>(nd, offset_of!(hist_entry, rb_node));
        if (*he).filtered == 0 {
            let event = container_of::<kvm_event, hist_entry>(he, offset_of!(kvm_event, he));
            let ecount = get_event_count(event, vcpu);
            let etime = get_event_time(event, vcpu);
            let max = get_event_max(event, vcpu);
            let min = get_event_min(event, vcpu);
            (*(*kvm).events_ops).decode_key.unwrap()(kvm, &(*event).key, decode.as_mut_ptr());
            pr_info(cstr!("%*s "), KVM_EVENT_NAME_LEN, decode.as_ptr());
            pr_info(cstr!("%10llu "), ecount as c_ulong);
            pr_info(cstr!("%8.2f%% "), ecount as c_double / (*kvm).total_count as c_double * 100.0);
            pr_info(cstr!("%8.2f%% "), etime as c_double / (*kvm).total_time as c_double * 100.0);
            pr_info(cstr!("%9.2fus "), min as c_double / NSEC_PER_USEC as c_double);
            pr_info(cstr!("%9.2fus "), max as c_double / NSEC_PER_USEC as c_double);
            pr_info(cstr!("%9.2fus ( +-%7.2f%% )"), etime as c_double / ecount as c_double / NSEC_PER_USEC as c_double, kvm_event_rel_stddev(vcpu, event));
            pr_info(cstr!("\n"));
        }
        nd = rb_next(nd);
    }
    pr_info(cstr!("\nTotal Samples:%lu, Total events handled time:%.2fus.\n\n"), (*kvm).total_count as c_ulong, (*kvm).total_time as c_double / NSEC_PER_USEC as c_double);
    if (*kvm).lost_events != 0 { pr_info(cstr!("\nLost events: %lu\n\n"), (*kvm).lost_events as c_ulong); }
}

#[cfg(all(feature = "timerfd", feature = "libtraceevent"))]
unsafe extern "C" fn process_lost_event(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let kvm = container_of::<perf_kvm_stat, perf_tool>(tool as *mut perf_tool, offset_of!(perf_kvm_stat, tool));
    (*kvm).lost_events += 1; 0
}

#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn skip_sample(kvm: *mut perf_kvm_stat, sample: *mut perf_sample) -> bool {
    if !(*kvm).pid_list.is_null() && intlist__find((*kvm).pid_list, (*sample).pid).is_null() { return true; }
    false
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn process_sample_event(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let mut err = 0;
    let kvm = container_of::<perf_kvm_stat, perf_tool>(tool as *mut perf_tool, offset_of!(perf_kvm_stat, tool));
    if skip_sample(kvm, sample) { return 0; }
    if machine__resolve(machine, &mut (*kvm).al, sample) < 0 {
        pr_warning(cstr!("WARNING: at offset %#lx: fail to resolve address location, skipping sample\n"), (*sample).file_offset as c_ulong);
        return 0;
    }
    let thread = machine__findnew_thread(machine, (*sample).pid, (*sample).tid);
    if thread.is_null() {
        pr_debug(cstr!("problem processing %s (%u) event at offset %#lx, skipping it.\n"), perf_event__name((*event).header.type_), (*event).header.type_, (*sample).file_offset as c_ulong);
        err = -1;
    } else {
        if !handle_kvm_event(kvm, thread, sample) { err = -1; }
        thread__put(thread);
    }
    addr_location__exit(&mut (*kvm).al); err
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn cpu_isa_config(kvm: *mut perf_kvm_stat) -> c_int {
    let mut buf = [0 as c_char; 128];
    let cpuid: *mut c_char;
    let mut err: c_int;
    if (*kvm).live {
        let cpu = perf_cpu { cpu: -1 };
        err = get_cpuid(buf.as_mut_ptr(), buf.len(), cpu);
        if err != 0 {
            pr_err(cstr!("Failed to look up CPU type: %s\n"), str_error_r(err, buf.as_mut_ptr(), buf.len()));
            return -err;
        }
        cpuid = buf.as_mut_ptr();
    } else {
        cpuid = (*perf_session__env((*kvm).session)).cpuid;
    }
    if cpuid.is_null() { pr_err(cstr!("Failed to look up CPU type\n")); return -EINVAL; }
    let e_machine = perf_session__e_machine((*kvm).session, null_mut());
    err = cpu_isa_init(kvm, e_machine, cpuid);
    if err == -ENOTSUP { pr_err(cstr!("CPU %s is not supported.\n"), cpuid); }
    err
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn verify_vcpu(vcpu: c_int) -> bool {
    if vcpu != -1 && vcpu < 0 { pr_err(cstr!("Invalid vcpu:%d.\n"), vcpu); return false; }
    true
}

#[cfg(all(feature = "timerfd", feature = "libtraceevent"))]
const PERF_KVM__MAX_EVENTS_PER_MMAP: s64 = 25;

#[cfg(all(feature = "timerfd", feature = "libtraceevent"))]
static mut done: c_int = 0;
#[cfg(all(feature = "timerfd", feature = "libtraceevent"))]
unsafe extern "C" fn sig_handler(sig: c_int) { done = 1; }

#[cfg(all(feature = "timerfd", feature = "libtraceevent"))]
unsafe extern "C" fn perf_kvm__mmap_read_idx(kvm: *mut perf_kvm_stat, idx: c_int, mmap_time: *mut u64) -> s64 {
    let evlist = (*kvm).evlist;
    let md = evlist__mmap(evlist).add(idx as usize);
    let mut n: s64 = 0;
    *mmap_time = ULLONG_MAX;
    let mut err = perf_mmap__read_init(&mut (*md).core);
    if err < 0 { return if err == -EAGAIN { 0 } else { -1 }; }
    loop {
        let event = perf_mmap__read_event(&mut (*md).core);
        if event.is_null() { break; }
        let mut timestamp: u64 = 0;
        err = evlist__parse_sample_timestamp(evlist, event, &mut timestamp);
        if err != 0 { perf_mmap__consume(&mut (*md).core); pr_err(cstr!("Failed to parse sample\n")); return -1; }
        err = perf_session__queue_event((*kvm).session, event, timestamp, 0, null_mut());
        /*
         * FIXME: Here we can't consume the event, as perf_session__queue_event will
         *        point to it, and it'll get possibly overwritten by the kernel.
         */
        perf_mmap__consume(&mut (*md).core);
        if err != 0 { pr_err(cstr!("Failed to enqueue sample: %d\n"), err); return -1; }
        if n == 0 { *mmap_time = timestamp; }
        n += 1;
        if n == PERF_KVM__MAX_EVENTS_PER_MMAP { break; }
    }
    perf_mmap__read_done(&mut (*md).core); n
}
#[cfg(all(feature = "timerfd", feature = "libtraceevent"))]
unsafe extern "C" fn perf_kvm__mmap_read(kvm: *mut perf_kvm_stat) -> c_int {
    let mut throttled = 0; let mut ntotal: s64 = 0; let mut flush_time = ULLONG_MAX;
    let mut i = 0;
    while i < (*evlist__core((*kvm).evlist)).nr_mmaps {
        let mut mmap_time: u64 = 0;
        let n = perf_kvm__mmap_read_idx(kvm, i, &mut mmap_time);
        if n < 0 { return -1; }
        if mmap_time < flush_time { flush_time = mmap_time; }
        ntotal += n; if n == PERF_KVM__MAX_EVENTS_PER_MMAP { throttled = 1; }
        i += 1;
    }
    if ntotal != 0 {
        let oe = &mut (*(*kvm).session).ordered_events;
        oe.next_flush = flush_time;
        let err = ordered_events__flush(oe, OE_FLUSH__ROUND);
        if err != 0 {
            if (*kvm).lost_events != 0 { pr_info(cstr!("\nLost events: %lu\n\n"), (*kvm).lost_events as c_ulong); }
            return err;
        }
    }
    throttled
}
#[cfg(all(feature = "timerfd", feature = "libtraceevent"))]
unsafe extern "C" fn perf_kvm__timerfd_create(kvm: *mut perf_kvm_stat) -> c_int {
    let mut new_value: itimerspec = zeroed();
    let mut rc = -1;
    (*kvm).timerfd = timerfd_create(CLOCK_MONOTONIC, TFD_NONBLOCK);
    if (*kvm).timerfd < 0 { pr_err(cstr!("timerfd_create failed\n")); return rc; }
    new_value.it_value.tv_sec = (*kvm).display_time as c_long;
    new_value.it_interval.tv_sec = (*kvm).display_time as c_long;
    if timerfd_settime((*kvm).timerfd, 0, &new_value, null_mut()) != 0 {
        pr_err(cstr!("timerfd_settime failed: %d\n"), errno); close((*kvm).timerfd); return rc;
    }
    rc = 0; rc
}
#[cfg(all(feature = "timerfd", feature = "libtraceevent"))]
unsafe extern "C" fn perf_kvm__handle_timerfd(kvm: *mut perf_kvm_stat) -> c_int {
    let mut c: u64 = 0;
    let rc = read((*kvm).timerfd, &mut c as *mut _ as *mut c_void, size_of::<u64>());
    if rc < 0 {
        if errno == EAGAIN { return 0; }
        pr_err(cstr!("Failed to read timer fd: %d\n"), errno); return -1;
    }
    if rc as usize != size_of::<u64>() { pr_err(cstr!("Error reading timer fd - invalid size returned\n")); return -1; }
    if c != 1 { pr_debug(cstr!("Missed timer beats: %lu\n"), (c - 1) as c_ulong); }
    /* update display */
    sort_result(kvm); print_result(kvm);
    /* Reset sort list to "ev_name" */
    kvm_hists__reinit(null(), cstr!("ev_name"));
    /* reset counts */
    clear_events_cache_stats(); (*kvm).total_count = 0; (*kvm).total_time = 0; (*kvm).lost_events = 0; 0
}
#[cfg(all(feature = "timerfd", feature = "libtraceevent"))]
unsafe extern "C" fn fd_set_nonblock(fd: c_int) -> c_int {
    let arg = fcntl(fd, F_GETFL);
    if arg < 0 { pr_err(cstr!("Failed to get current flags for fd %d\n"), fd); return -1; }
    if fcntl(fd, F_SETFL, arg | O_NONBLOCK as c_long) < 0 { pr_err(cstr!("Failed to set non-block option on fd %d\n"), fd); return -1; }
    0
}
#[cfg(all(feature = "timerfd", feature = "libtraceevent"))]
unsafe extern "C" fn perf_kvm__handle_stdin() -> c_int {
    let c = getc(stdin);
    if c == 'q' as c_int { return 1; }
    0
}

#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn read_events(kvm: *mut perf_kvm_stat) -> c_int {
    let mut file = perf_data { path: (*kvm).file_name, mode: PERF_DATA_MODE_READ, force: (*kvm).force };
    perf_tool__init(&mut (*kvm).tool, true);
    (*kvm).tool.sample = Some(process_sample_event);
    (*kvm).tool.comm = perf_event__process_comm;
    (*kvm).tool.namespaces = perf_event__process_namespaces;
    (*kvm).session = perf_session__new(&mut file, &mut (*kvm).tool);
    if IS_ERR((*kvm).session as *const c_void) { pr_err(cstr!("Initializing perf session failed\n")); return PTR_ERR((*kvm).session as *const c_void); }
    symbol__init(perf_session__env((*kvm).session));
    let mut ret: c_int;
    if !perf_session__has_traces((*kvm).session, cstr!("kvm record")) { ret = -EINVAL; }
    else {
        let e_machine = perf_session__e_machine((*kvm).session, null_mut());
        if !register_kvm_events_ops(kvm, e_machine) { ret = -EINVAL; }
        else {
            /* Do not use 'isa' recorded in kvm_exit tracepoint since it is not traced in the old kernel. */
            ret = cpu_isa_config(kvm);
            if ret >= 0 { ret = perf_session__process_events((*kvm).session); }
        }
    }
    perf_session__delete((*kvm).session); ret
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn parse_target_str(kvm: *mut perf_kvm_stat) -> c_int {
    if !(*kvm).opts.target.pid.is_null() {
        (*kvm).pid_list = intlist__new((*kvm).opts.target.pid);
        if (*kvm).pid_list.is_null() { pr_err(cstr!("Error parsing process id string\n")); return -EINVAL; }
    }
    0
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn kvm_events_report_vcpu(kvm: *mut perf_kvm_stat) -> c_int {
    let mut ret = -EINVAL;
    if parse_target_str(kvm) != 0 { hists__delete_entries(&mut kvm_hists.hists); return ret; }
    if !verify_vcpu((*kvm).trace_vcpu) || !is_valid_key(kvm) { hists__delete_entries(&mut kvm_hists.hists); return ret; }
    if (*kvm).use_stdio { use_browser = 0; setup_pager(); } else { use_browser = 1; }
    setup_browser(false); kvm_hists__init();
    ret = read_events(kvm);
    if ret == 0 { sort_result(kvm); kvm_display(kvm); }
    hists__delete_entries(&mut kvm_hists.hists); ret
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn kvm_events_record(kvm: *mut perf_kvm_stat, argc: c_int, argv: *const *const c_char) -> c_int {
    let record_args = [cstr!("record"), cstr!("-R"), cstr!("-m"), cstr!("1024"), cstr!("-c"), cstr!("1")];
    let kvm_stat_record_usage = [cstr!("perf kvm stat record [<options>]"), null()];
    let e_machine = EM_HOST;
    let mut events_tp_size: c_uint = 0;
    let mut ret = setup_kvm_events_tp(kvm, e_machine);
    if ret < 0 { pr_err(cstr!("Unable to setup the kvm tracepoints\n")); return ret; }
    let mut events_tp = kvm_events_tp(e_machine);
    while !(*events_tp).is_null() { events_tp_size += 1; events_tp = events_tp.add(1); }
    let rec_argc = record_args.len() as c_uint + argc as c_uint + 2 + 2 * events_tp_size;
    let rec_argv = calloc((rec_argc + 1) as usize, size_of::<*const c_char>()) as *mut *const c_char;
    if rec_argv.is_null() { return -ENOMEM; }
    let mut i: c_uint = 0;
    while (i as usize) < record_args.len() { *rec_argv.add(i as usize) = record_args[i as usize]; i += 1; }
    let mut j = 0;
    while j < events_tp_size { *rec_argv.add(i as usize) = cstr!("-e"); i += 1; *rec_argv.add(i as usize) = *kvm_events_tp(e_machine).add(j as usize); i += 1; j += 1; }
    *rec_argv.add(i as usize) = cstr!("-o"); i += 1; *rec_argv.add(i as usize) = (*kvm).file_name; i += 1;
    j = 1; while j < argc as c_uint { *rec_argv.add(i as usize) = *argv.add(j as usize); i += 1; j += 1; }
    set_option_flag(record_options, 'e' as c_int, cstr!("event"), PARSE_OPT_HIDDEN);
    set_option_flag(record_options, 0, cstr!("filter"), PARSE_OPT_HIDDEN);
    set_option_flag(record_options, 'R' as c_int, cstr!("raw-samples"), PARSE_OPT_HIDDEN);
    set_option_flag(record_options, 'F' as c_int, cstr!("freq"), PARSE_OPT_DISABLED);
    set_option_flag(record_options, 0, cstr!("group"), PARSE_OPT_DISABLED);
    set_option_flag(record_options, 'g' as c_int, null(), PARSE_OPT_DISABLED);
    set_option_flag(record_options, 0, cstr!("call-graph"), PARSE_OPT_DISABLED);
    set_option_flag(record_options, 'd' as c_int, cstr!("data"), PARSE_OPT_DISABLED);
    set_option_flag(record_options, 'T' as c_int, cstr!("timestamp"), PARSE_OPT_DISABLED);
    set_option_flag(record_options, 'P' as c_int, cstr!("period"), PARSE_OPT_DISABLED);
    set_option_flag(record_options, 'n' as c_int, cstr!("no-samples"), PARSE_OPT_DISABLED);
    set_option_flag(record_options, 'N' as c_int, cstr!("no-buildid-cache"), PARSE_OPT_DISABLED);
    set_option_flag(record_options, 'B' as c_int, cstr!("no-buildid"), PARSE_OPT_DISABLED);
    set_option_flag(record_options, 'G' as c_int, cstr!("cgroup"), PARSE_OPT_DISABLED);
    set_option_flag(record_options, 'b' as c_int, cstr!("branch-any"), PARSE_OPT_DISABLED);
    set_option_flag(record_options, 'j' as c_int, cstr!("branch-filter"), PARSE_OPT_DISABLED);
    set_option_flag(record_options, 'W' as c_int, cstr!("weight"), PARSE_OPT_DISABLED);
    set_option_flag(record_options, 0, cstr!("transaction"), PARSE_OPT_DISABLED);
    record_usage = kvm_stat_record_usage.as_ptr();
    ret = cmd_record(i as c_int, rec_argv);
    free(rec_argv as *mut c_void); ret
}

#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn kvm_events_report(kvm: *mut perf_kvm_stat, argc: c_int, argv: *const *const c_char) -> c_int {
    /* The local C option table is preserved conceptually; OPT_* macro expansion is external. */
    let kvm_events_report_options: [option; 1] = [zeroed()];
    let kvm_events_report_usage = [cstr!("perf kvm stat report [<options>]"), null()];
    let mut argc = argc;
    if argc != 0 {
        argc = parse_options(argc, argv, kvm_events_report_options.as_ptr(), kvm_events_report_usage.as_ptr(), 0);
        if argc != 0 { usage_with_options(kvm_events_report_usage.as_ptr(), kvm_events_report_options.as_ptr()); }
    }
    #[cfg(not(feature = "slang"))] { (*kvm).use_stdio = true; }
    if (*kvm).opts.target.pid.is_null() { (*kvm).opts.target.system_wide = true; }
    kvm_events_report_vcpu(kvm)
}

#[cfg(all(feature = "timerfd", feature = "libtraceevent"))]
unsafe extern "C" fn kvm_live_event_list() -> *mut evlist {
    let evlist = evlist__new();
    if evlist.is_null() { return null_mut(); }
    let mut err = -1;
    let mut events_tp = kvm_events_tp(EM_HOST);
    while !(*events_tp).is_null() {
        let tp = strdup(*events_tp);
        if tp.is_null() { break; }
        let sys = tp;
        let name = strchr(tp, ':' as c_int);
        if name.is_null() {
            pr_err(cstr!("Error parsing %s tracepoint: subsystem delimiter not found\n"), *events_tp);
            free(tp as *mut c_void); break;
        }
        *name = 0; let name2 = name.add(1);
        if evlist__add_newtp(evlist, sys, name2, null_mut()) != 0 {
            pr_err(cstr!("Failed to add %s tracepoint to the list\n"), *events_tp);
            free(tp as *mut c_void); break;
        }
        free(tp as *mut c_void); events_tp = events_tp.add(1);
    }
    if (*events_tp).is_null() { err = 0; }
    if err != 0 { evlist__put(evlist); return null_mut(); }
    evlist
}

#[cfg(all(feature = "timerfd", feature = "libtraceevent"))]
unsafe extern "C" fn kvm_events_live(kvm: *mut perf_kvm_stat, argc: c_int, argv: *const *const c_char) -> c_int {
    let mut errbuf = [0 as c_char; BUFSIZ];
    let mut err: c_int;
    let live_options: [option; 1] = [zeroed()];
    let live_usage = [cstr!("perf kvm stat live [<options>]"), null()];
    let mut data = perf_data { path: null(), mode: PERF_DATA_MODE_WRITE, force: false };
    perf_tool__init(&mut (*kvm).tool, true);
    (*kvm).tool.sample = Some(process_sample_event); (*kvm).tool.comm = perf_event__process_comm;
    (*kvm).tool.exit = perf_event__process_exit; (*kvm).tool.fork = perf_event__process_fork;
    (*kvm).tool.lost = Some(process_lost_event); (*kvm).tool.namespaces = perf_event__process_namespaces;
    (*kvm).display_time = 1; (*kvm).opts.user_interval = 1; (*kvm).opts.mmap_pages = 512; (*kvm).opts.target.uses_mmap = false;
    symbol__init(null_mut()); disable_buildid_cache(); use_browser = 0;
    let mut argc = argc;
    if argc != 0 {
        argc = parse_options(argc, argv, live_options.as_ptr(), live_usage.as_ptr(), 0);
        if argc != 0 { usage_with_options(live_usage.as_ptr(), live_options.as_ptr()); }
    }
    (*kvm).duration *= NSEC_PER_USEC;   /* convert usec to nsec */
    err = target__validate(&mut (*kvm).opts.target);
    if err != 0 { target__strerror(&mut (*kvm).opts.target, err, errbuf.as_mut_ptr(), BUFSIZ); ui__warning(cstr!("%s"), errbuf.as_ptr()); }
    if target__none(&mut (*kvm).opts.target) { (*kvm).opts.target.system_wide = true; }
    err = setup_kvm_events_tp(kvm, EM_HOST);
    if err < 0 { pr_err(cstr!("Unable to setup the kvm tracepoints\n")); return err; }
    (*kvm).evlist = kvm_live_event_list();
    if (*kvm).evlist.is_null() { err = -1; return err; }
    if evlist__create_maps((*kvm).evlist, &mut (*kvm).opts.target) < 0 { usage_with_options(live_usage.as_ptr(), live_options.as_ptr()); }
    (*kvm).session = perf_session__new(&mut data, &mut (*kvm).tool);
    if IS_ERR((*kvm).session as *const c_void) { err = PTR_ERR((*kvm).session as *const c_void); evlist__put((*kvm).evlist); return err; }
    (*(*kvm).session).evlist = (*kvm).evlist;
    perf_session__set_id_hdr_size((*kvm).session);
    ordered_events__set_copy_on_queue(&mut (*(*kvm).session).ordered_events, true);
    machine__synthesize_threads(&mut (*(*kvm).session).machines.host, &mut (*kvm).opts.target, (*evlist__core((*kvm).evlist)).threads, true, false, 1);
    err = kvm_live_open_events(kvm);
    if err == 0 { err = kvm_events_live_report(kvm); }
    perf_session__delete((*kvm).session); (*kvm).session = null_mut(); evlist__put((*kvm).evlist); err
}

#[cfg(all(feature = "timerfd", feature = "libtraceevent"))]
unsafe extern "C" fn kvm_events_live_report(kvm: *mut perf_kvm_stat) -> c_int {
    let mut save: termios = zeroed();
    (*kvm).live = true;
    let ret = cpu_isa_config(kvm);
    if ret < 0 { return ret; }
    let mut err = -EINVAL;
    if !verify_vcpu((*kvm).trace_vcpu) || !is_valid_key(kvm) || !register_kvm_events_ops(kvm, EM_HOST) { hists__delete_entries(&mut kvm_hists.hists); return err; }
    set_term_quiet_input(&mut save); kvm_hists__init(); signal(SIGINT, sig_handler); signal(SIGTERM, sig_handler);
    if perf_kvm__timerfd_create(kvm) < 0 { hists__delete_entries(&mut kvm_hists.hists); tcsetattr(0, TCSAFLUSH, &save); return -1; }
    if evlist__add_pollfd((*kvm).evlist, (*kvm).timerfd) < 0 { hists__delete_entries(&mut kvm_hists.hists); tcsetattr(0, TCSAFLUSH, &save); return err; }
    let nr_stdin = evlist__add_pollfd((*kvm).evlist, fileno(stdin));
    if nr_stdin < 0 || fd_set_nonblock(fileno(stdin)) != 0 { hists__delete_entries(&mut kvm_hists.hists); tcsetattr(0, TCSAFLUSH, &save); return err; }
    evlist__enable((*kvm).evlist);
    while done == 0 {
        let fda = &mut (*evlist__core((*kvm).evlist)).pollfd;
        let rc = perf_kvm__mmap_read(kvm);
        if rc < 0 { break; }
        err = perf_kvm__handle_timerfd(kvm);
        if err != 0 { break; }
        if ((*fda.entries.add(nr_stdin as usize)).revents as c_int & POLLIN) != 0 { done = perf_kvm__handle_stdin(); }
        if rc == 0 && done == 0 { err = evlist__poll((*kvm).evlist, 100); }
    }
    evlist__disable((*kvm).evlist);
    if err == 0 { sort_result(kvm); print_result(kvm); }
    hists__delete_entries(&mut kvm_hists.hists);
    if (*kvm).timerfd >= 0 { close((*kvm).timerfd); }
    tcsetattr(0, TCSAFLUSH, &save); err
}

#[cfg(all(feature = "timerfd", feature = "libtraceevent"))]
unsafe extern "C" fn kvm_live_open_events(kvm: *mut perf_kvm_stat) -> c_int {
    let mut sbuf = [0 as c_char; STRERR_BUFSIZE];
    let evlist = (*kvm).evlist;
    evlist__config(evlist, &mut (*kvm).opts, null_mut());
    /* C evlist__for_each_entry loop translated as externally-provided iterator intent. */
    let err = evlist__open(evlist);
    if err < 0 {
        printf(cstr!("Couldn't create the events: %s\n"), str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()));
        return -1;
    }
    if evlist__do_mmap(evlist, (*kvm).opts.mmap_pages) < 0 {
        ui__error(cstr!("Failed to mmap the events: %s\n"), str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()));
        evlist__close(evlist); return -1;
    }
    0
}

#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn print_kvm_stat_usage() {
    printf(cstr!("Usage: perf kvm stat <command>\n\n"));
    printf(cstr!("# Available commands:\n"));
    printf(cstr!("\trecord: record kvm events\n"));
    printf(cstr!("\treport: report statistical data of kvm events\n"));
    printf(cstr!("\tlive:   live reporting of statistical data of kvm events\n"));
    printf(cstr!("\nOtherwise, it is the alias of 'perf stat':\n"));
}
#[cfg(feature = "libtraceevent")]
unsafe extern "C" fn kvm_cmd_stat(file_name: *const c_char, argc: c_int, argv: *const *const c_char) -> c_int {
    let mut kvm: perf_kvm_stat = zeroed();
    kvm.file_name = file_name; kvm.trace_vcpu = -1; kvm.report_event = cstr!("vmexit"); kvm.sort_key = cstr!("sample");
    if argc == 1 { print_kvm_stat_usage(); return cmd_stat(argc, argv); }
    if strlen(*argv.add(1)) > 2 && strstarts(cstr!("record"), *argv.add(1)) { return kvm_events_record(&mut kvm, argc - 1, argv.add(1)); }
    if strlen(*argv.add(1)) > 2 && strstarts(cstr!("report"), *argv.add(1)) { return kvm_events_report(&mut kvm, argc - 1, argv.add(1)); }
    #[cfg(all(feature = "timerfd", feature = "libtraceevent"))]
    if strncmp(*argv.add(1), cstr!("live"), 4) == 0 { return kvm_events_live(&mut kvm, argc - 1, argv.add(1)); }
    cmd_stat(argc, argv)
}

unsafe extern "C" fn __cmd_record(file_name: *const c_char, argc: c_int, argv: *const *const c_char) -> c_int {
    let need_arch_event = (kvm_need_default_arch_event(EM_HOST, argc, argv) != 0) as c_int;
    /*
     * Besides the 2 more options "-o" and "filename",
     * kvm_add_default_arch_event() may add 2 extra options,
     * so allocate more items conditionally.
     */
    let rec_argc = argc + 2 + (2 * need_arch_event);
    let rec_argv = calloc((rec_argc + 1) as usize, size_of::<*const c_char>()) as *mut *const c_char;
    if rec_argv.is_null() { return -ENOMEM; }
    let mut i = 0; *rec_argv.add(i) = cstr!("record"); i += 1; *rec_argv.add(i) = cstr!("-o"); i += 1; *rec_argv.add(i) = file_name; i += 1;
    let mut ret = 0;
    if need_arch_event != 0 {
        ret = kvm_add_default_arch_event(EM_HOST, &mut (i as c_int), rec_argv);
        if ret != 0 { free(rec_argv as *mut c_void); return ret; }
    }
    let mut j = 1;
    while j < argc { *rec_argv.add(i) = *argv.add(j as usize); j += 1; i += 1; }
    BUG_ON!(i as c_int != rec_argc);
    ret = cmd_record(i as c_int, rec_argv);
    free(rec_argv as *mut c_void); ret
}
unsafe extern "C" fn __cmd_report(file_name: *const c_char, argc: c_int, argv: *const *const c_char) -> c_int {
    let rec_argc = argc + 2;
    let rec_argv = calloc((rec_argc + 1) as usize, size_of::<*const c_char>()) as *mut *const c_char;
    if rec_argv.is_null() { return -ENOMEM; }
    let mut i = 0; *rec_argv.add(i) = cstr!("report"); i += 1; *rec_argv.add(i) = cstr!("-i"); i += 1; *rec_argv.add(i) = file_name; i += 1;
    let mut j = 1; while j < argc { *rec_argv.add(i) = *argv.add(j as usize); j += 1; i += 1; }
    BUG_ON!(i as c_int != rec_argc);
    let ret = cmd_report(i as c_int, rec_argv); free(rec_argv as *mut c_void); ret
}
unsafe extern "C" fn __cmd_buildid_list(file_name: *const c_char, argc: c_int, argv: *const *const c_char) -> c_int {
    let rec_argc = argc + 2;
    let rec_argv = calloc((rec_argc + 1) as usize, size_of::<*const c_char>()) as *mut *const c_char;
    if rec_argv.is_null() { return -ENOMEM; }
    let mut i = 0; *rec_argv.add(i) = cstr!("buildid-list"); i += 1; *rec_argv.add(i) = cstr!("-i"); i += 1; *rec_argv.add(i) = file_name; i += 1;
    let mut j = 1; while j < argc { *rec_argv.add(i) = *argv.add(j as usize); j += 1; i += 1; }
    BUG_ON!(i as c_int != rec_argc);
    let ret = cmd_buildid_list(i as c_int, rec_argv); free(rec_argv as *mut c_void); ret
}
unsafe extern "C" fn __cmd_top(argc: c_int, argv: *const *const c_char) -> c_int {
    /*
     * kvm_add_default_arch_event() may add 2 extra options, so
     * allocate 2 more pointers in adavance.
     */
    let rec_argc = argc + 2;
    let rec_argv = calloc((rec_argc + 1) as usize, size_of::<*const c_char>()) as *mut *const c_char;
    if rec_argv.is_null() { return -ENOMEM; }
    let mut i = 0; while i < argc { *rec_argv.add(i as usize) = *argv.add(i as usize); i += 1; }
    BUG_ON!(i != argc);
    let mut ret;
    if kvm_need_default_arch_event(EM_HOST, argc, argv) != 0 {
        ret = kvm_add_default_arch_event(EM_HOST, &mut i, rec_argv);
        if ret != 0 { free(rec_argv as *mut c_void); return ret; }
    }
    ret = cmd_top(i, rec_argv); free(rec_argv as *mut c_void); ret
}

#[no_mangle]
pub unsafe extern "C" fn cmd_kvm(mut argc: c_int, argv: *const *const c_char) -> c_int {
    let mut file_name: *const c_char = null();
    /* The local C option table is preserved conceptually; OPT_* macro expansion is external. */
    let kvm_options: [option; 1] = [zeroed()];
    let kvm_subcommands = [cstr!("top"), cstr!("record"), cstr!("report"), cstr!("diff"), cstr!("buildid-list"), cstr!("stat"), null()];
    let mut kvm_usage = [null(), null()];
    exclude_GH_default = true;
    perf_host = 0; perf_guest = 1;
    argc = parse_options_subcommand(argc, argv, kvm_options.as_ptr(), kvm_subcommands.as_ptr(), kvm_usage.as_mut_ptr(), PARSE_OPT_STOP_AT_NON_OPTION);
    if argc == 0 { usage_with_options(kvm_usage.as_ptr(), kvm_options.as_ptr()); }
    thread__set_priv_destructor(free);
    if perf_host == 0 { perf_guest = 1; }
    if file_name.is_null() { file_name = get_filename_for_perf_kvm(); }
    let arg0 = *argv;
    let ret: c_int;
    if strlen(arg0) > 2 && strstarts(cstr!("record"), arg0) { ret = __cmd_record(file_name, argc, argv); }
    else if strlen(arg0) > 2 && strstarts(cstr!("report"), arg0) { ret = __cmd_report(file_name, argc, argv); }
    else if strlen(arg0) > 2 && strstarts(cstr!("diff"), arg0) { ret = cmd_diff(argc, argv); }
    else if strcmp(arg0, cstr!("top")) == 0 { ret = __cmd_top(argc, argv); }
    else if strlen(arg0) > 2 && strstarts(cstr!("buildid-list"), arg0) { ret = __cmd_buildid_list(file_name, argc, argv); }
    else {
        #[cfg(feature = "libtraceevent")]
        {
            if strlen(arg0) > 2 && strstarts(cstr!("stat"), arg0) {
                free(kvm_usage[0] as *mut c_void);
                return kvm_cmd_stat(file_name, argc, argv);
            }
        }
        usage_with_options(kvm_usage.as_ptr(), kvm_options.as_ptr());
    }
    /* free usage string allocated by parse_options_subcommand */
    free(kvm_usage[0] as *mut c_void);
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
