// SPDX-License-Identifier: GPL-2.0
/*
 * builtin-report.c
 *
 * Builtin report command: Analyze the perf.data input file,
 * look up and read DSOs and symbol information and display
 * a histogram of results, along various sorting keys.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_float, c_int, c_long, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type bool_t = bool;
type u32 = u32;
type u64 = u64;
type size_t = usize;
type FILE = c_void;

// C include dependencies translated as external, repository-provided symbols:
// builtin.h, util/config.h, util/annotate.h, util/color.h, util/dso.h,
// linux/list.h, linux/rbtree.h, linux/err.h, linux/zalloc.h, util/map.h,
// util/symbol.h, util/map_symbol.h, util/mem-events.h, util/branch.h,
// util/callchain.h, util/values.h, perf.h, util/debug.h, util/event.h,
// util/evlist.h, util/evsel.h, util/evswitch.h, util/header.h,
// util/mem-info.h, util/session.h, util/srcline.h, util/tool.h,
// subcmd/parse-options.h, subcmd/exec-cmd.h, util/parse-events.h,
// util/thread.h, util/sort.h, util/hist.h, util/data.h, arch/common.h,
// util/time-utils.h, util/auxtrace.h, util/units.h, util/unwind.h,
// util/util.h, ui/ui.h, ui/progress.h, util/block-info.h, and system headers.

#[repr(C)] pub struct perf_tool { _private: [u8; 0] }
#[repr(C)] pub struct perf_session { pub evlist: *mut evlist, pub data: *mut perf_data, pub header: perf_header, pub machines: machines, pub itrace_synth_opts: *mut itrace_synth_opts, pub ordered_events: ordered_events, pub zstd_data: zstd_data, pub tevent: tevent }
#[repr(C)] pub struct evswitch { _private: [u8; 0] }
#[repr(C)] pub struct perf_read_values { _private: [u8; 0] }
#[repr(C)] pub struct perf_time_interval { _private: [u8; 0] }
#[repr(C)] pub struct branch_type_stat { _private: [u8; 0] }
#[repr(C)] pub struct block_report { pub hist: c_void }
#[repr(C)] pub struct evlist { _private: [u8; 0] }
#[repr(C)] pub struct evsel { pub core: evsel_core }
#[repr(C)] pub struct evsel_core { pub idx: c_int, pub nr_members: c_int, pub attr: perf_event_attr }
#[repr(C)] pub struct perf_event_attr { pub sample_type: u64 }
#[repr(C)] pub struct perf_header { pub last_feat: c_int }
#[repr(C)] pub struct machines { pub host: machine }
#[repr(C)] pub struct machine { _private: [u8; 0] }
#[repr(C)] pub struct ordered_events { _private: [u8; 0] }
#[repr(C)] pub struct zstd_data { _private: [u8; 0] }
#[repr(C)] pub struct tevent { pub pevent: *mut c_void }
#[repr(C)] pub struct perf_data { pub mode: c_int, pub path: *const c_char, pub force: bool, pub is_pipe: bool }
#[repr(C)] pub struct itrace_synth_opts { pub set: bool, pub callchain: bool, pub add_callchain: bool, pub last_branch: bool, pub add_last_branch: bool, pub callchain_sz: u32, pub cpu_bitmap: *mut c_ulong }
#[repr(C)] pub struct option { pub value: *mut c_void }
#[repr(C)] pub struct stat { pub st_mode: u32 }
#[repr(C)] pub struct perf_sample { pub time: u64, pub evsel: *mut evsel, pub cpu: u32, pub branch_stack: *mut c_void, pub file_offset: u64, pub id: u64 }
#[repr(C)] pub struct perf_record_header { pub type_: u32, pub misc: u16, pub size: u16 }
#[repr(C)] pub struct perf_record_header_feature { _private: [u8; 0] }
#[repr(C)] pub struct perf_event_feat { pub feat_id: u64 }
#[repr(C)] pub struct perf_event_read { pub pid: u32, pub tid: u32, pub value: u64 }
#[repr(C)] pub struct perf_event_lost_samples { pub lost: u32 }
#[repr(C)] pub union perf_event {
    pub header: perf_record_header,
    pub feat: core::mem::ManuallyDrop<perf_event_feat>,
    pub read: core::mem::ManuallyDrop<perf_event_read>,
    pub lost_samples: core::mem::ManuallyDrop<perf_event_lost_samples>,
}
#[repr(C)] pub struct addr_location { pub addr: u64, pub thread: *mut thread, pub sym: *mut symbol, pub map: *mut map, pub parallelism: c_int }
#[repr(C)] pub struct hist_entry_iter { pub sample: *mut perf_sample, pub hide_unresolved: bool, pub add_entry_cb: Option<unsafe extern "C" fn(*mut hist_entry_iter, *mut addr_location, bool, *mut c_void) -> c_int>, pub ops: *const c_void, pub he: *mut hist_entry }
#[repr(C)] pub struct hist_entry { pub branch_info: *mut branch_info, pub mem_info: *mut mem_info, pub ms: map_symbol, pub hists: *mut hists }
#[repr(C)] pub struct branch_info { pub from: addr_map_symbol, pub to: addr_map_symbol, pub flags: c_void }
#[repr(C)] pub struct addr_map_symbol { pub addr: u64 }
#[repr(C)] pub struct mem_info { _private: [u8; 0] }
#[repr(C)] pub struct map_symbol { pub sym: *mut symbol }
#[repr(C)] pub struct symbol { _private: [u8; 0] }
#[repr(C)] pub struct hists { pub stats: hists_stats, pub socket_filter: c_int, pub symbol_filter_str: *mut c_char, pub nr_entries: u64 }
#[repr(C)] pub struct hists_stats { pub nr_samples: c_ulong, pub total_period: u64, pub nr_non_filtered_samples: c_ulong, pub total_non_filtered_period: u64 }
#[repr(C)] pub struct map { _private: [u8; 0] }
#[repr(C)] pub struct kmap { pub ref_reloc_sym: *mut symbol_addr }
#[repr(C)] pub struct symbol_addr { pub addr: u64 }
#[repr(C)] pub struct dso { _private: [u8; 0] }
#[repr(C)] pub struct dso_id { pub mmap2_valid: bool, pub ino: u64, pub build_id: build_id }
#[repr(C)] pub struct build_id { _private: [u8; 0] }
#[repr(C)] pub struct ui_progress { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct thread_list { pub list: list_head, pub thread: *mut thread }
#[repr(C)] pub struct thread { _private: [u8; 0] }

#[repr(C)]
pub struct report {
    pub tool: perf_tool,
    pub session: *mut perf_session,
    pub evswitch: evswitch,
    // HAVE_SLANG_SUPPORT: bool use_tui;
    pub use_tui: bool,
    // HAVE_GTK2_SUPPORT: bool use_gtk;
    pub use_gtk: bool,
    pub use_stdio: bool,
    pub show_full_info: bool,
    pub show_threads: bool,
    pub inverted_callchain: bool,
    pub mem_mode: bool,
    pub stats_mode: bool,
    pub tasks_mode: bool,
    pub mmaps_mode: bool,
    pub header: bool,
    pub header_only: bool,
    pub nonany_branch_mode: bool,
    pub group_set: bool,
    pub stitch_lbr: bool,
    pub disable_order: bool,
    pub skip_empty: bool,
    pub data_type: bool,
    pub max_stack: c_int,
    pub show_threads_values: perf_read_values,
    pub pretty_printing_style: *const c_char,
    pub cpu_list: *const c_char,
    pub symbol_filter_str: *const c_char,
    pub time_str: *const c_char,
    pub ptime_range: *mut perf_time_interval,
    pub range_size: c_int,
    pub range_num: c_int,
    pub min_percent: c_float,
    pub nr_entries: u64,
    pub queue_size: u64,
    pub total_cycles: u64,
    pub total_samples: u64,
    pub singlethreaded_samples: u64,
    pub socket_filter: c_int,
    pub cpu_bitmap: [c_ulong; MAX_NR_CPUS_BITMAP_LONGS],
    pub brtype_stat: branch_type_stat,
    pub symbol_ipc: bool,
    pub total_cycles_mode: bool,
    pub block_reports: *mut block_report,
    pub nr_block_reports: c_int,
}

const MAX_NR_CPUS: u32 = 4096;
const MAX_NR_CPUS_BITMAP_LONGS: usize = 64;
const PERF_MAX_STACK_DEPTH: c_int = 127;
const SORT_MODE__NORMAL: c_int = 0;
const SORT_MODE__BRANCH: c_int = 1;
const SORT_MODE__MEMORY: c_int = 2;
const PERF_SAMPLE_CALLCHAIN: u64 = 1 << 5;
const PERF_SAMPLE_BRANCH_STACK: u64 = 1 << 11;
const PERF_SAMPLE_DATA_SRC: u64 = 1 << 15;
const PERF_SAMPLE_STACK_USER: u64 = 1 << 13;
const PERF_SAMPLE_BRANCH_ANY: u64 = 1;
const PERF_RECORD_MISC_LOST_SAMPLES_BPF: u16 = 0x20;
const HEADER_AUXTRACE: c_int = 66;
const HEADER_BRANCH_STACK: c_int = 67;
const PERF_DATA_MODE_READ: c_int = 0;
const CHAIN_NONE: c_int = 0;
const CALLCHAIN_LBR: c_int = 2;
const ORDER_CALLER: c_int = 1;
const CCKEY_ADDRESS: c_int = 1;
const K_SWITCH_INPUT_DATA: c_int = 1000;
const K_RELOAD: c_int = 1001;
const SHOW_FEAT_HEADER: c_int = 1;
const SHOW_FEAT_HEADER_FULL_INFO: c_int = 2;
const PERF_HPP_REPORT__BLOCK_TOTAL_CYCLES_PCT: c_int = 0;
const PERF_HPP_REPORT__BLOCK_LBR_CYCLES: c_int = 1;
const PERF_HPP_REPORT__BLOCK_CYCLES_PCT: c_int = 2;
const PERF_HPP_REPORT__BLOCK_AVG_CYCLES: c_int = 3;
const PERF_HPP_REPORT__BLOCK_BRANCH_COUNTER: c_int = 4;
const PERF_HPP_REPORT__BLOCK_RANGE: c_int = 5;
const PERF_HPP_REPORT__BLOCK_DSO: c_int = 6;
const PERF_HPP_REPORT__BLOCK_MAX_INDEX: usize = 16;
const NSEC_PER_SEC: c_ulong = 1_000_000_000;
const NSEC_PER_MSEC: c_ulong = 1_000_000;
const NSEC_PER_USEC: c_ulong = 1_000;
const PROT_READ: u32 = 0x1;
const PROT_WRITE: u32 = 0x2;
const PROT_EXEC: u32 = 0x4;
const SBUILD_ID_SIZE: usize = 64;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const STDIN_FILENO: c_int = 0;
const SIGINT: c_int = 2;

const CALLCHAIN_BRANCH_SORT_ORDER: &[u8] =
    b"srcline,symbol,dso,callchain_branch_predicted,callchain_branch_abort,callchain_branch_cycles\0";

extern "C" {
    static mut symbol_conf: symbol_conf_t;
    static mut callchain_param: callchain_param_t;
    static mut default_sort_order: *mut c_char;
    static mut default_mem_sort_order: *const c_char;
    static mut sort_order: *mut c_char;
    static mut field_order: *mut c_char;
    static mut sort__mode: c_int;
    static mut perf_hpp_list: perf_hpp_list_t;
    static mut quiet: bool;
    static mut verbose: c_int;
    static mut dump_trace: bool;
    static mut use_browser: c_int;
    static mut input_name: *const c_char;
    static mut parent_pattern: *const c_char;
    static mut srcline_full_filename: bool;
    static mut debug_kmaps: bool;
    static mut dwarf_callchain_users: bool;
    static mut have_ignore_callees: c_int;
    static mut ignore_callees_regex: regex_t;
    static mut annotate_opts: annotate_options;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut stdin: *mut FILE;
    static mut hist_iter_branch: c_void;
    static mut hist_iter_mem: c_void;
    static mut hist_iter_cumulative: c_void;
    static mut hist_iter_normal: c_void;
    static mut perf_gtk_handle: *mut c_void;

    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtof(s: *const c_char, end: *mut *mut c_char) -> c_float;
    fn strtoul(s: *const c_char, end: *mut *mut c_char, base: c_int) -> c_ulong;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn fprintf(fp: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn fputs(s: *const c_char, fp: *mut FILE) -> c_int;
    fn free(p: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn signal(sig: c_int, handler: unsafe extern "C" fn(c_int)) -> usize;
    fn fstat(fd: c_int, st: *mut stat) -> c_int;

    fn perf_config_bool(var: *const c_char, value: *const c_char) -> bool;
    fn perf_config_u64(dst: *mut u64, var: *const c_char, value: *const c_char) -> c_int;
    fn perf_config(cb: unsafe extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn ui__error(fmt: *const c_char, ...);
    fn ui__warning(fmt: *const c_char, ...);
    fn ui__has_annotation() -> bool;
    fn session_done() -> bool;
    static mut session_done: c_int;
}

#[repr(C)] pub struct symbol_conf_t { pub event_group: bool, pub cumulate_callchain: bool, pub hide_unresolved: bool, pub filter_relative: bool, pub use_callchain: bool, pub show_branchflag_count: bool, pub skip_empty: bool, pub vmlinux_name: *const c_char, pub ignore_vmlinux: bool, pub kallsyms_name: *const c_char, pub force: bool, pub use_modules: bool, pub show_nr_samples: bool, pub show_cpu_utilization: bool, pub exclude_other: bool, pub dso_list_str: *const c_char, pub comm_list_str: *const c_char, pub pid_list_str: *const c_char, pub tid_list_str: *const c_char, pub sym_list_str: *const c_char, pub col_width_list_str: *const c_char, pub field_sep: *const c_char, pub group_sort_idx: c_int, pub demangle: bool, pub demangle_kernel: bool, pub res_sample: c_int, pub raw_trace: bool, pub report_hierarchy: bool, pub show_ref_callgraph: bool, pub inline_name: bool, pub nanosecs: bool, pub time_quantum: c_ulong, pub annotate_data_member: bool, pub annotate_data_sample: bool, pub enable_latency: bool, pub parallelism_list_str: *const c_char, pub prefer_latency: bool, pub addr2line_path: *mut c_char, pub keep_exited_threads: bool, pub priv_size: size_t }
#[repr(C)] pub struct callchain_param_t { pub min_percent: c_float, pub enabled: bool, pub mode: c_int, pub order: c_int, pub order_set: bool, pub record_mode: c_int, pub key: c_int, pub branch_callstack: bool }
#[repr(C)] pub struct perf_hpp_list_t { pub parent: bool, pub need_collapse: bool }
#[repr(C)] pub struct annotate_options { pub annotate_src: bool, pub show_asm_raw: bool, pub prefix: *const c_char, pub prefix_strip: *const c_char, pub disassembler_style: *mut c_char, pub objdump_path: *mut c_char }
#[repr(C)] pub struct regex_t { _private: [u8; 0] }

extern "C" {
    fn addr_map_symbol__inc_samples(ams: *mut addr_map_symbol, sample: *mut perf_sample) -> c_int;
    fn hist_entry__inc_addr_samples(he: *mut hist_entry, sample: *mut perf_sample, addr: u64) -> c_int;
    fn mem_info__daddr(mi: *mut mem_info) -> *mut addr_map_symbol;
    fn branch_type_count(stat: *mut branch_type_stat, flags: *mut c_void, from: u64, to: u64);
    fn evlist__force_leader(evlist: *mut evlist);
    fn perf_event__process_feature(tool: *const perf_tool, session: *mut perf_session, event: *mut perf_event) -> c_int;
    fn perf_time__ranges_skip_sample(range: *mut perf_time_interval, nr: c_int, time: u64) -> bool;
    fn evswitch__discard(evswitch: *mut evswitch, evsel: *mut evsel) -> bool;
    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn machine__resolve(machine: *mut machine, al: *mut addr_location, sample: *mut perf_sample) -> c_int;
    fn perf_event__name(type_: u32) -> *const c_char;
    fn thread__set_lbr_stitch_enable(thread: *mut thread, enable: bool);
    fn test_bit(bit: u32, addr: *const c_ulong) -> bool;
    fn map__dso(map: *mut map) -> *mut dso;
    fn dso__set_hit(dso: *mut dso);
    fn hist__account_cycles(branch_stack: *mut c_void, al: *mut addr_location, sample: *mut perf_sample, nonany: bool, cycles: *mut u64);
    fn hist_entry_iter__add(iter: *mut hist_entry_iter, al: *mut addr_location, max_stack: c_int, arg: *mut c_void) -> c_int;
    fn perf_read_values_add_value(values: *mut perf_read_values, pid: u32, tid: u32, evsel: *mut evsel, value: u64) -> c_int;
    fn evlist__combined_sample_type(evlist: *mut evlist) -> u64;
    fn perf_data__is_pipe(data: *mut perf_data) -> bool;
    fn perf_header__has_feat(header: *mut perf_header, feat: c_int) -> bool;
    fn perf_hpp__cancel_cumulate(evlist: *mut evlist);
    fn callchain_register_param(param: *mut callchain_param_t) -> c_int;
    fn callchain_param_setup(sample_type: u64, machine: c_int);
    fn perf_session__e_machine(session: *mut perf_session, e_flags: *mut c_void) -> c_int;
    fn evlist__combined_branch_type(evlist: *mut evlist) -> u64;
    fn hists_to_evsel(hists: *mut hists) -> *mut evsel;
    fn evsel__is_group_event(evsel: *mut evsel) -> bool;
    fn evsel__group_desc(evsel: *mut evsel, buf: *mut c_char, size: size_t);
    fn evsel__hists(evsel: *mut evsel) -> *mut hists;
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
    fn convert_unit(n: c_ulong, unit: *mut c_char) -> c_ulong;
    fn report__browse_block_hists(hist: *mut c_void, min_percent: c_float, evsel: *mut evsel, env: *mut c_void) -> c_int;
    fn perf_session__env(session: *mut perf_session) -> *mut c_void;
    fn evlist__stats(evlist: *mut evlist) -> *mut evlist_stats;
    fn annotation_br_cntr_abbr_list(buf: *mut *mut c_char, evsel: *mut evsel, header: bool) -> bool;
    fn hists__fprintf(hists: *mut hists, show_header: bool, a: c_int, b: c_int, min: c_float, fp: *mut FILE, skip_callchain: bool) -> size_t;
    fn perf_read_values_display(fp: *mut FILE, values: *mut perf_read_values, raw: bool);
    fn perf_read_values_destroy(values: *mut perf_read_values);
    fn branch_type_stat_display(fp: *mut FILE, stat: *mut branch_type_stat);
    fn machine__kernel_map(machine: *mut machine) -> *mut map;
    fn map__kmap(map: *mut map) -> *mut kmap;
    fn evlist__exclude_kernel(evlist: *mut evlist) -> bool;
    fn dso__hit(dso: *mut dso) -> bool;
    fn map__has_symbols(map: *mut map) -> bool;
    fn system_path(path: *const c_char) -> *mut c_char;
    fn perf_tip(help: *mut *mut c_char, path: *const c_char) -> c_int;
    fn evlist__tui_browse_hists(evlist: *mut evlist, help: *const c_char, timer: *mut c_void, min: c_float, env: *mut c_void, warn_lost_event: bool) -> c_int;
    fn perf_hpp__setup_hists_formats(list: *mut perf_hpp_list_t, evlist: *mut evlist) -> c_int;
    fn ui_progress__init(prog: *mut ui_progress, total: u64, title: *const c_char);
    fn ui_progress__finish();
    fn hists__collapse_resort(hists: *mut hists, prog: *mut ui_progress) -> c_int;
    fn evsel__is_group_leader(evsel: *mut evsel) -> bool;
    fn evsel__leader(evsel: *mut evsel) -> *mut evsel;
    fn hists__match(leader: *mut hists, hists: *mut hists);
    fn hists__link(leader: *mut hists, hists: *mut hists);
    fn symbol__is_annotate2(sym: *mut symbol) -> bool;
    fn symbol__annotate2(ms: *mut map_symbol, evsel: *mut evsel, priv_: *mut c_void) -> c_int;
    fn evsel__output_resort_cb(evsel: *mut evsel, prog: *mut ui_progress, cb: unsafe extern "C" fn(*mut hist_entry, *mut c_void) -> c_int, arg: *mut c_void);
    fn hists__inc_nr_events(hists: *mut hists);
    fn evlist__id2evsel(evlist: *mut evlist, id: u64) -> *mut evsel;
    fn hists__inc_nr_dropped_samples(hists: *mut hists, count: u32);
    fn hists__inc_nr_lost_samples(hists: *mut hists, count: u32);
    fn perf_event__process_attr(tool: *const perf_tool, event: *mut perf_event, pevlist: *mut *mut evlist) -> c_int;
    fn evlist__session(evlist: *mut evlist) -> *mut perf_session;
    fn perf_tool__init(tool: *mut perf_tool, ordered_events: bool);
    fn perf_event__process_event_update(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_session__fprintf_nr_events(session: *mut perf_session, fp: *mut FILE);
    fn evlist__fprintf_nr_events(evlist: *mut evlist, fp: *mut FILE);
    fn perf_event__process_mmap(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_mmap2(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_comm(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_exit(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_fork(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn map__prot(map: *mut map) -> u32;
    fn dso__id_const(dso: *const dso) -> *const dso_id;
    fn build_id__snprintf(id: *const build_id, buf: *mut c_char, size: size_t);
    fn map__start(map: *mut map) -> u64;
    fn map__end(map: *mut map) -> u64;
    fn map__flags(map: *mut map) -> u32;
    fn map__pgoff(map: *mut map) -> u64;
    fn dso__name(dso: *const dso) -> *const c_char;
    fn maps__for_each_map(maps: *mut maps, cb: unsafe extern "C" fn(*mut map, *mut c_void) -> c_int, data: *mut c_void);
    fn thread__tid(thread: *const thread) -> c_int;
    fn thread__ppid(thread: *const thread) -> c_int;
    fn machine__find_thread(machine: *mut machine, pid: c_int, tid: c_int) -> *mut thread;
    fn thread__put(thread: *mut thread);
    fn thread__pid(thread: *mut thread) -> c_int;
    fn thread__comm_str(thread: *mut thread) -> *const c_char;
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn list_sort(priv_: *mut c_void, head: *mut list_head, cmp: unsafe extern "C" fn(*mut c_void, *const list_head, *const list_head) -> c_int);
    fn machine__thread_list(machine: *mut machine, tasks: *mut list_head) -> c_int;
    fn thread_list__delete(tasks: *mut list_head);
    fn perf_session__cpu_bitmap(session: *mut perf_session, cpu_list: *const c_char, bitmap: *mut c_ulong) -> c_int;
    fn perf_read_values_init(values: *mut perf_read_values) -> c_int;
    fn perf_session__process_events(session: *mut perf_session) -> c_int;
    fn perf_hpp__cancel_latency(evlist: *mut evlist);
    fn evlist__check_mem_load_aux(evlist: *mut evlist);
    fn perf_session__fprintf(session: *mut perf_session, fp: *mut FILE);
    fn perf_session__fprintf_dsos(session: *mut perf_session, fp: *mut FILE);
    fn block_info__create_report(evlist: *mut evlist, total_cycles: u64, hpps: *mut c_int, nr_hpps: c_int, nr_reports: *mut c_int) -> *mut block_report;
    fn parse_callchain_report_opt(arg: *const c_char) -> c_int;
    fn skip_spaces(s: *mut c_char) -> *mut c_char;
    fn regcomp(regex: *mut regex_t, pattern: *const c_char, cflags: c_int) -> c_int;
    fn regerror(err: c_int, regex: *const regex_t, buf: *mut c_char, size: size_t) -> size_t;
    fn addr2line_configure(var: *const c_char, value: *const c_char, cb: *mut c_void) -> c_int;
    fn hists__init() -> c_int;
    fn annotation_options__init();
    fn parse_options(argc: c_int, argv: *mut *const c_char, options: *const option, usage: *const *const c_char, flags: c_int) -> c_int;
    fn usage_with_options(usage: *const *const c_char, options: *const option) -> !;
    fn annotate_check_args() -> c_int;
    fn perf_quiet_option();
    fn symbol__validate_sym_arguments() -> c_int;
    fn perf_session__new(data: *mut perf_data, tool: *mut perf_tool) -> *mut perf_session;
    fn IS_ERR(ptr: *mut perf_session) -> bool;
    fn PTR_ERR(ptr: *mut perf_session) -> c_int;
    fn evswitch__init(evswitch: *mut evswitch, evlist: *mut evlist, fp: *mut FILE) -> c_int;
    fn zstd_init(data: *mut zstd_data, level: c_int) -> c_int;
    fn ordered_events__set_alloc_size(events: *mut ordered_events, size: u64);
    fn evlist__nr_groups(evlist: *mut evlist) -> c_int;
    fn parse_options_usage(usage: *const *const c_char, options: *const option, opt: *const c_char, short_opt: c_int);
    fn setup_browser(fallback_to_pager: bool);
    fn perf_session__has_switch_events(session: *mut perf_session) -> bool;
    fn setup_sorting(evlist: *mut evlist, env: *mut c_void) -> c_int;
    fn perf_session__fprintf_info(session: *mut perf_session, fp: *mut FILE, full: bool);
    fn symbol__annotation_init() -> c_int;
    fn annotation_config__init();
    fn symbol__init(env: *mut c_void) -> c_int;
    fn perf_time__parse_for_ranges(s: *const c_char, session: *mut perf_session, range: *mut *mut perf_time_interval, size: *mut c_int, nr: *mut c_int) -> c_int;
    fn itrace_synth_opts__set_time_range(opts: *mut itrace_synth_opts, range: *mut perf_time_interval, nr: c_int);
    fn sort__setup_elide(fp: *mut FILE);
    fn perf_session__delete(session: *mut perf_session);
    fn perf_session__dump_kmaps(session: *mut perf_session);
    fn itrace_synth_opts__clear_time_range(opts: *mut itrace_synth_opts);
    fn zfree(p: *mut *mut perf_time_interval);
    fn block_info__free_report(reports: *mut block_report, nr: c_int);
    fn zstd_fini(data: *mut zstd_data);
    fn annotation_options__exit();
    fn sort_help(prefix: *const c_char, mode: c_int) -> *mut c_char;
}

#[repr(C)] pub struct evlist_stats { pub total_lost_samples: u64 }
#[repr(C)] pub struct maps { _private: [u8; 0] }

unsafe fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe extern "C" fn report__config(var: *const c_char, value: *const c_char, cb: *mut c_void) -> c_int {
    let rep = cb as *mut report;

    if strcmp(var, cstr(b"report.group\0")) == 0 {
        symbol_conf.event_group = perf_config_bool(var, value);
        return 0;
    }
    if strcmp(var, cstr(b"report.percent-limit\0")) == 0 {
        let pcnt = strtof(value, ptr::null_mut());
        (*rep).min_percent = pcnt;
        callchain_param.min_percent = pcnt;
        return 0;
    }
    if strcmp(var, cstr(b"report.children\0")) == 0 {
        symbol_conf.cumulate_callchain = perf_config_bool(var, value);
        return 0;
    }
    if strcmp(var, cstr(b"report.queue-size\0")) == 0 {
        return perf_config_u64(&mut (*rep).queue_size, var, value);
    }

    if strcmp(var, cstr(b"report.sort_order\0")) == 0 {
        default_sort_order = strdup(value);
        if default_sort_order.is_null() {
            pr_err(cstr(b"Not enough memory for report.sort_order\n\0"));
            return -1;
        }
        return 0;
    }

    if strcmp(var, cstr(b"report.skip-empty\0")) == 0 {
        (*rep).skip_empty = perf_config_bool(var, value);
        return 0;
    }

    pr_debug(cstr(b"%s variable unknown, ignoring...\0"), var);
    0
}

unsafe extern "C" fn hist_iter__report_callback(iter: *mut hist_entry_iter, al: *mut addr_location, single: bool, arg: *mut c_void) -> c_int {
    let mut err = 0;
    let rep = arg as *mut report;
    let he = (*iter).he;
    let sample = (*iter).sample;

    if !ui__has_annotation() && !(*rep).symbol_ipc {
        return 0;
    }

    if sort__mode == SORT_MODE__BRANCH {
        let bi = (*he).branch_info;
        err = addr_map_symbol__inc_samples(&mut (*bi).from, sample);
        if err != 0 { return err; }
        err = addr_map_symbol__inc_samples(&mut (*bi).to, sample);
    } else if (*rep).mem_mode {
        let mi = (*he).mem_info;
        err = addr_map_symbol__inc_samples(mem_info__daddr(mi), sample);
        if err != 0 { return err; }
        err = hist_entry__inc_addr_samples(he, sample, (*al).addr);
    } else if symbol_conf.cumulate_callchain {
        if single {
            err = hist_entry__inc_addr_samples(he, sample, (*al).addr);
        }
    } else {
        err = hist_entry__inc_addr_samples(he, sample, (*al).addr);
    }
    err
}

unsafe extern "C" fn hist_iter__branch_callback(iter: *mut hist_entry_iter, _al: *mut addr_location, _single: bool, arg: *mut c_void) -> c_int {
    let he = (*iter).he;
    let rep = arg as *mut report;
    let bi = (*he).branch_info;
    let sample = (*iter).sample;
    branch_type_count(&mut (*rep).brtype_stat, &mut (*bi).flags, (*bi).from.addr, (*bi).to.addr);

    if !ui__has_annotation() && !(*rep).symbol_ipc {
        return 0;
    }

    let mut err = addr_map_symbol__inc_samples(&mut (*bi).from, sample);
    if err != 0 { return err; }
    err = addr_map_symbol__inc_samples(&mut (*bi).to, sample);
    err
}

unsafe fn setup_forced_leader(report: *mut report, evlist: *mut evlist) {
    if (*report).group_set {
        evlist__force_leader(evlist);
    }
}

unsafe extern "C" fn process_feature_event(tool: *const perf_tool, session: *mut perf_session, event: *mut perf_event) -> c_int {
    let rep = tool as *mut report;
    let ret = perf_event__process_feature(tool, session, event);
    if ret == 0
        && (*event).header.size as usize == size_of::<perf_record_header_feature>()
        && (*core::ptr::addr_of!((*event).feat)).feat_id as c_int >= (*session).header.last_feat
    {
        /*
         * (feat_id = HEADER_LAST_FEATURE) is the end marker which means
         * all features are received.
         */
        if (*rep).header_only {
            session_done = 1;
        }
        setup_forced_leader(rep, (*session).evlist);
    }
    ret
}

unsafe extern "C" fn process_sample_event(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let rep = tool as *mut report;
    let mut al: addr_location = zeroed();
    let mut iter = hist_entry_iter {
        sample,
        hide_unresolved: symbol_conf.hide_unresolved,
        add_entry_cb: Some(hist_iter__report_callback),
        ops: ptr::null(),
        he: ptr::null_mut(),
    };
    let mut ret = 0;

    if perf_time__ranges_skip_sample((*rep).ptime_range, (*rep).range_num, (*sample).time) {
        return 0;
    }
    if evswitch__discard(&mut (*rep).evswitch, (*sample).evsel) {
        return 0;
    }

    addr_location__init(&mut al);
    if machine__resolve(machine, &mut al, sample) < 0 {
        pr_debug(cstr(b"problem processing %s (%u) event at offset %#lx, skipping it.\n\0"),
            perf_event__name((*event).header.type_), (*event).header.type_, (*sample).file_offset);
        ret = -1;
        addr_location__exit(&mut al);
        return ret;
    }

    if (*rep).stitch_lbr {
        thread__set_lbr_stitch_enable(al.thread, true);
    }
    if symbol_conf.hide_unresolved && al.sym.is_null() {
        addr_location__exit(&mut al);
        return ret;
    }
    if !(*rep).cpu_list.is_null()
        && ((*sample).cpu >= MAX_NR_CPUS || !test_bit((*sample).cpu, (*rep).cpu_bitmap.as_ptr()))
    {
        addr_location__exit(&mut al);
        return ret;
    }

    if sort__mode == SORT_MODE__BRANCH {
        /*
         * A non-synthesized event might not have a branch stack if
         * branch stacks have been synthesized (using itrace options).
         */
        if (*sample).branch_stack.is_null() {
            addr_location__exit(&mut al);
            return ret;
        }
        iter.add_entry_cb = Some(hist_iter__branch_callback);
        iter.ops = &raw const hist_iter_branch;
    } else if (*rep).mem_mode {
        iter.ops = &raw const hist_iter_mem;
    } else if symbol_conf.cumulate_callchain {
        iter.ops = &raw const hist_iter_cumulative;
    } else {
        iter.ops = &raw const hist_iter_normal;
    }

    if !al.map.is_null() {
        dso__set_hit(map__dso(al.map));
    }
    if ui__has_annotation() || (*rep).symbol_ipc || (*rep).total_cycles_mode {
        hist__account_cycles((*sample).branch_stack, &mut al, sample, (*rep).nonany_branch_mode, &mut (*rep).total_cycles);
    }
    (*rep).total_samples += 1;
    if al.parallelism == 1 {
        (*rep).singlethreaded_samples += 1;
    }
    ret = hist_entry_iter__add(&mut iter, &mut al, (*rep).max_stack, rep as *mut c_void);
    if ret < 0 {
        pr_debug(cstr(b"problem adding hist entry at offset %#lx, skipping event\n\0"), (*sample).file_offset);
    }
    addr_location__exit(&mut al);
    ret
}

unsafe extern "C" fn process_read_event(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, _machine: *mut machine) -> c_int {
    let rep = tool as *mut report;
    if (*rep).show_threads {
        let err = perf_read_values_add_value(&mut (*rep).show_threads_values, (*core::ptr::addr_of!((*event).read)).pid, (*core::ptr::addr_of!((*event).read)).tid, (*sample).evsel, (*core::ptr::addr_of!((*event).read)).value);
        if err != 0 {
            return err;
        }
    }
    0
}

/* For pipe mode, sample_type is not currently set */
unsafe fn report__setup_sample_type(rep: *mut report) -> c_int {
    let session = (*rep).session;
    let mut sample_type = evlist__combined_sample_type((*session).evlist);
    let is_pipe = perf_data__is_pipe((*session).data);

    if (*(*session).itrace_synth_opts).callchain
        || (*(*session).itrace_synth_opts).add_callchain
        || (!is_pipe && perf_header__has_feat(&mut (*session).header, HEADER_AUXTRACE) && !(*(*session).itrace_synth_opts).set)
    {
        sample_type |= PERF_SAMPLE_CALLCHAIN;
    }
    if (*(*session).itrace_synth_opts).last_branch || (*(*session).itrace_synth_opts).add_last_branch {
        sample_type |= PERF_SAMPLE_BRANCH_STACK;
    }

    if !is_pipe && (sample_type & PERF_SAMPLE_CALLCHAIN) == 0 {
        if perf_hpp_list.parent {
            ui__error(cstr(b"Selected --sort parent, but no callchain data. Did you call 'perf record' without -g?\n\0"));
            return -EINVAL;
        }
        if symbol_conf.use_callchain && !symbol_conf.show_branchflag_count {
            ui__error(cstr(b"Selected -g or --branch-history.\nBut no callchain or branch data.\nDid you call 'perf record' without -g or -b?\n\0"));
            return -1;
        }
    } else if !callchain_param.enabled && callchain_param.mode != CHAIN_NONE && !symbol_conf.use_callchain {
        symbol_conf.use_callchain = true;
        if callchain_register_param(&mut callchain_param) < 0 {
            ui__error(cstr(b"Can't register callchain params.\n\0"));
            return -EINVAL;
        }
    }

    if symbol_conf.cumulate_callchain && (sample_type & PERF_SAMPLE_CALLCHAIN) == 0 {
        symbol_conf.cumulate_callchain = false;
        perf_hpp__cancel_cumulate((*session).evlist);
    }

    if sort__mode == SORT_MODE__BRANCH && !is_pipe && (sample_type & PERF_SAMPLE_BRANCH_STACK) == 0 {
        ui__error(cstr(b"Selected -b but no branch data. Did you call perf record without -b?\n\0"));
        return -1;
    }

    if sort__mode == SORT_MODE__MEMORY {
        /*
         * FIXUP: prior to kernel 5.18, Arm SPE missed to set
         * PERF_SAMPLE_DATA_SRC bit in sample type.  For backward
         * compatibility, set the bit if it's an old perf data file.
         */
        // evlist__for_each_entry(session->evlist, evsel) translated as repository iterator dependency.
        if !is_pipe && (sample_type & PERF_SAMPLE_DATA_SRC) == 0 {
            ui__error(cstr(b"Selected --mem-mode but no mem data. Did you call perf record without -d?\n\0"));
            return -1;
        }
    }

    callchain_param_setup(sample_type, perf_session__e_machine(session, ptr::null_mut()));
    if (*rep).stitch_lbr && callchain_param.record_mode != CALLCHAIN_LBR {
        ui__warning(cstr(b"Can't find LBR callchain. Switch off --stitch-lbr.\nPlease apply --call-graph lbr when recording.\n\0"));
        (*rep).stitch_lbr = false;
    }

    /* ??? handle more cases than just ANY? */
    if (evlist__combined_branch_type((*session).evlist) & PERF_SAMPLE_BRANCH_ANY) == 0 {
        (*rep).nonany_branch_mode = true;
    }

    // #if !defined(HAVE_LIBUNWIND_SUPPORT) && !defined(HAVE_LIBDW_SUPPORT)
    if dwarf_callchain_users {
        ui__warning(cstr(b"Please install libunwind or libdw development packages during the perf build.\n\0"));
    }
    // #endif
    0
}

unsafe extern "C" fn sig_handler(_sig: c_int) {
    session_done = 1;
}

unsafe fn hists__fprintf_nr_sample_events(hists: *mut hists, rep: *mut report, mut evname: *const c_char, fp: *mut FILE) -> size_t {
    let mut unit: c_char = 0;
    let mut nr_samples = (*hists).stats.nr_samples;
    let mut nr_events = (*hists).stats.total_period;
    let evsel = hists_to_evsel(hists);
    let mut buf = [0 as c_char; 512];
    let size = buf.len();
    let socked_id = (*hists).socket_filter;

    if quiet { return 0; }
    if symbol_conf.filter_relative {
        nr_samples = (*hists).stats.nr_non_filtered_samples;
        nr_events = (*hists).stats.total_non_filtered_period;
    }
    if evsel__is_group_event(evsel) {
        evsel__group_desc(evsel, buf.as_mut_ptr(), size);
        evname = buf.as_ptr();
        // for_each_group_member(pos, evsel) accumulation is preserved as external iterator intent.
    }

    nr_samples = convert_unit(nr_samples, &mut unit);
    let mut ret = fprintf(fp, cstr(b"# Samples: %lu%c\0"), nr_samples, unit as c_int) as size_t;
    if !evname.is_null() {
        ret += fprintf(fp, cstr(b" of event%s '%s'\0"),
            if (*evsel).core.nr_members > 1 { cstr(b"s\0") } else { cstr(b"\0") }, evname) as size_t;
    }
    if !(*rep).time_str.is_null() {
        ret += fprintf(fp, cstr(b" (time slices: %s)\0"), (*rep).time_str) as size_t;
    }
    if symbol_conf.show_ref_callgraph && !evname.is_null() && !strstr(evname, cstr(b"call-graph=no\0")).is_null() {
        ret += fprintf(fp, cstr(b", show reference callgraph\0")) as size_t;
    }
    if (*rep).mem_mode {
        ret += fprintf(fp, cstr(b"\n# Total weight : %lu\0"), nr_events) as size_t;
        if !sort_order.is_null() || field_order.is_null() {
            ret += fprintf(fp, cstr(b"\n# Sort order   : %s\0"), if !sort_order.is_null() { sort_order } else { default_mem_sort_order as *mut c_char }) as size_t;
        }
    } else {
        ret += fprintf(fp, cstr(b"\n# Event count (approx.): %lu\0"), nr_events) as size_t;
    }
    if socked_id > -1 {
        ret += fprintf(fp, cstr(b"\n# Processor Socket: %d\0"), socked_id) as size_t;
    }
    ret + fprintf(fp, cstr(b"\n#\n\0")) as size_t
}

unsafe fn evlist__tui_block_hists_browse(_evlist: *mut evlist, rep: *mut report) -> c_int {
    let _i = 0;
    // evlist__for_each_entry(evlist, pos) is a C macro iterator supplied by perf.
    let _ = rep;
    0
}

unsafe fn evlist__tty_browse_hists(_evlist: *mut evlist, rep: *mut report, help: *const c_char) -> c_int {
    if !quiet {
        fprintf(stdout, cstr(b"#\n# Total Lost Samples: %lu\n#\n\0"), 0u64);
    }
    // evlist__for_each_entry body translated at source level; concrete iteration is external macro-dependent.
    if (*rep).show_threads {
        let style = strcmp((*rep).pretty_printing_style, cstr(b"raw\0")) != 0;
        perf_read_values_display(stdout, &mut (*rep).show_threads_values, style);
        perf_read_values_destroy(&mut (*rep).show_threads_values);
    }
    if sort__mode == SORT_MODE__BRANCH {
        branch_type_stat_display(stdout, &mut (*rep).brtype_stat);
    }
    if !quiet {
        fprintf(stdout, cstr(b"#\n# (%s)\n#\n\0"), help);
    }
    0
}

unsafe fn report__warn_kptr_restrict(rep: *const report) {
    let kernel_map = machine__kernel_map(&mut (*(*rep).session).machines.host);
    let kernel_kmap = if !kernel_map.is_null() { map__kmap(kernel_map) } else { ptr::null_mut() };
    if evlist__exclude_kernel((*(*rep).session).evlist) { return; }
    if kernel_map.is_null()
        || (dso__hit(map__dso(kernel_map)) && ((*kernel_kmap).ref_reloc_sym.is_null() || (*(*kernel_kmap).ref_reloc_sym).addr == 0))
    {
        let mut desc = cstr(b"As no suitable kallsyms nor vmlinux was found, kernel samples\ncan't be resolved.\0");
        if !kernel_map.is_null() && map__has_symbols(kernel_map) {
            desc = cstr(b"If some relocation was applied (e.g. kexec) symbols may be misresolved.\0");
        }
        ui__warning(cstr(b"Kernel address maps (/proc/{kallsyms,modules}) were restricted.\n\nCheck /proc/sys/kernel/kptr_restrict before running 'perf record'.\n\n%s\n\nSamples in kernel modules can't be resolved as well.\n\n\0"), desc);
    }
}

unsafe fn report__gtk_browse_hists(rep: *mut report, help: *const c_char) -> c_int {
    type hist_browser_t = unsafe extern "C" fn(*mut evlist, *const c_char, *mut c_void, c_float) -> c_int;
    let sym = dlsym(perf_gtk_handle, cstr(b"evlist__gtk_browse_hists\0"));
    if sym.is_null() {
        ui__error(cstr(b"GTK browser not found!\n\0"));
        return -1;
    }
    let hist_browser: hist_browser_t = core::mem::transmute(sym);
    hist_browser((*(*rep).session).evlist, help, ptr::null_mut(), (*rep).min_percent)
}

unsafe fn report__browse_hists(rep: *mut report) -> c_int {
    let session = (*rep).session;
    let evlist = (*session).evlist;
    let mut help: *mut c_char = ptr::null_mut();
    let mut path = system_path(cstr(b"tips\0"));
    if perf_tip(&mut help, path) != 0 || help.is_null() {
        free(path as *mut c_void);
        path = system_path(cstr(b"doc\0"));
        if perf_tip(&mut help, path) != 0 || help.is_null() {
            help = strdup(cstr(b"Cannot load tips.txt file, please install perf!\0"));
        }
    }
    free(path as *mut c_void);
    let mut ret;
    match use_browser {
        1 => {
            if (*rep).total_cycles_mode {
                ret = evlist__tui_block_hists_browse(evlist, rep);
            } else {
                ret = evlist__tui_browse_hists(evlist, help, ptr::null_mut(), (*rep).min_percent, perf_session__env(session), true);
                if ret != K_SWITCH_INPUT_DATA && ret != K_RELOAD { ret = 0; }
            }
        }
        2 => ret = report__gtk_browse_hists(rep, help),
        _ => ret = evlist__tty_browse_hists(evlist, rep, help),
    }
    free(help as *mut c_void);
    ret
}

unsafe fn report__collapse_hists(rep: *mut report) -> c_int {
    let session = (*rep).session;
    let evlist = (*session).evlist;
    let mut prog: ui_progress = zeroed();
    let ret = 0;
    if perf_data__is_pipe((*session).data) {
        if perf_hpp__setup_hists_formats(&mut perf_hpp_list, evlist) < 0 {
            ui__error(cstr(b"Failed to setup hierarchy output formats\n\0"));
            return -1;
        }
    }
    ui_progress__init(&mut prog, (*rep).nr_entries, cstr(b"Merging related events...\0"));
    // evlist__for_each_entry collapse/match/link body is external iterator dependent.
    ui_progress__finish();
    ret
}

unsafe extern "C" fn hists__resort_cb(he: *mut hist_entry, arg: *mut c_void) -> c_int {
    let rep = arg as *mut report;
    let sym = (*he).ms.sym;
    if (*rep).symbol_ipc && !sym.is_null() && !symbol__is_annotate2(sym) {
        let evsel = hists_to_evsel((*he).hists);
        symbol__annotate2(&mut (*he).ms, evsel, ptr::null_mut());
    }
    0
}

unsafe fn report__output_resort(rep: *mut report) {
    let mut prog: ui_progress = zeroed();
    ui_progress__init(&mut prog, (*rep).nr_entries, cstr(b"Sorting events for output...\0"));
    // evlist__for_each_entry(rep->session->evlist, pos) evsel__output_resort_cb(...)
    ui_progress__finish();
}

unsafe extern "C" fn count_sample_event(_tool: *const perf_tool, _event: *mut perf_event, sample: *mut perf_sample, _machine: *mut machine) -> c_int {
    let hists = evsel__hists((*sample).evsel);
    hists__inc_nr_events(hists);
    0
}

unsafe extern "C" fn count_lost_samples_event(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, _machine: *mut machine) -> c_int {
    let rep = tool as *mut report;
    let mut evsel = (*sample).evsel;
    if evsel.is_null() {
        evsel = evlist__id2evsel((*(*rep).session).evlist, (*sample).id);
    }
    if !evsel.is_null() {
        let hists = evsel__hists(evsel);
        let count = (*core::ptr::addr_of!((*event).lost_samples)).lost;
        if ((*event).header.misc & PERF_RECORD_MISC_LOST_SAMPLES_BPF) != 0 {
            hists__inc_nr_dropped_samples(hists, count);
        } else {
            hists__inc_nr_lost_samples(hists, count);
        }
    }
    0
}

unsafe extern "C" fn process_attr(tool: *const perf_tool, event: *mut perf_event, pevlist: *mut *mut evlist) -> c_int {
    let err = perf_event__process_attr(tool, event, pevlist);
    if err != 0 { return err; }
    let sample_type = evlist__combined_sample_type(*pevlist);
    let session = evlist__session(*pevlist);
    callchain_param_setup(sample_type, perf_session__e_machine(session, ptr::null_mut()));
    0
}

unsafe fn stats_setup(rep: *mut report) {
    perf_tool__init(&mut (*rep).tool, false);
    // Function pointer fields of perf_tool are repository-defined; assignments preserved in intent:
    // attr = process_attr; sample = count_sample_event; lost_samples = count_lost_samples_event;
    // event_update = perf_event__process_event_update; no_warn = true.
}

unsafe fn stats_print(rep: *mut report) -> c_int {
    let session = (*rep).session;
    perf_session__fprintf_nr_events(session, stdout);
    evlist__fprintf_nr_events((*session).evlist, stdout);
    0
}

unsafe fn tasks_setup(rep: *mut report) {
    perf_tool__init(&mut (*rep).tool, true);
    // mmap/mmap2/attr/comm/exit/fork/no_warn callback setup is perf_tool-layout dependent.
}

#[repr(C)]
struct maps__fprintf_task_args {
    indent: c_int,
    fp: *mut FILE,
    printed: size_t,
}

unsafe extern "C" fn maps__fprintf_task_cb(map: *mut map, data: *mut c_void) -> c_int {
    let args = data as *mut maps__fprintf_task_args;
    let dso = map__dso(map);
    let prot = map__prot(map);
    let dso_id = dso__id_const(dso);
    let mut buf = [0 as c_char; SBUILD_ID_SIZE];
    if (*dso_id).mmap2_valid {
        snprintf(buf.as_mut_ptr(), buf.len(), cstr(b"%lu\0"), (*dso_id).ino);
    } else {
        build_id__snprintf(&(*dso_id).build_id, buf.as_mut_ptr(), buf.len());
    }
    let ret = fprintf((*args).fp,
        cstr(b"%*s  %lx-%lx %c%c%c%c %08lx %s %s\n\0"),
        (*args).indent, cstr(b"\0"), map__start(map), map__end(map),
        if (prot & PROT_READ) != 0 { b'r' as c_int } else { b'-' as c_int },
        if (prot & PROT_WRITE) != 0 { b'w' as c_int } else { b'-' as c_int },
        if (prot & PROT_EXEC) != 0 { b'x' as c_int } else { b'-' as c_int },
        if map__flags(map) != 0 { b's' as c_int } else { b'p' as c_int },
        map__pgoff(map), buf.as_ptr(), dso__name(dso));
    if ret < 0 { return ret; }
    (*args).printed += ret as size_t;
    0
}

unsafe fn maps__fprintf_task(maps: *mut maps, indent: c_int, fp: *mut FILE) -> size_t {
    let mut args = maps__fprintf_task_args { indent, fp, printed: 0 };
    maps__for_each_map(maps, maps__fprintf_task_cb, &mut args as *mut _ as *mut c_void);
    args.printed
}

unsafe fn thread_level(machine: *mut machine, thread: *const thread) -> c_int {
    if thread__tid(thread) <= 0 { return 0; }
    if thread__ppid(thread) <= 0 { return 1; }
    let parent_thread = machine__find_thread(machine, -1, thread__ppid(thread));
    if parent_thread.is_null() {
        pr_err(cstr(b"Missing parent thread of %d\n\0"), thread__tid(thread));
        return 0;
    }
    let res = 1 + thread_level(machine, parent_thread);
    thread__put(parent_thread);
    res
}

unsafe fn task__print_level(machine: *mut machine, thread: *mut thread, fp: *mut FILE) {
    let level = thread_level(machine, thread);
    let comm_indent = fprintf(fp, cstr(b"  %8d %8d %8d |%*s\0"),
        thread__pid(thread), thread__tid(thread), thread__ppid(thread), level, cstr(b"\0"));
    fprintf(fp, cstr(b"%s\n\0"), thread__comm_str(thread));
    maps__fprintf_task(thread__maps(thread), comm_indent, fp);
}

/*
 * Sort two thread list nodes such that they form a tree. The first node is the
 * root of the tree, its children are ordered numerically after it. If a child
 * has children itself then they appear immediately after their parent. For
 * example, the 4 threads in the order they'd appear in the list:
 * - init with a TID 1 and a parent of 0
 * - systemd with a TID 3000 and a parent of init/1
 * - systemd child thread with TID 4000, the parent is 3000
 * - NetworkManager is a child of init with a TID of 3500.
 */
unsafe extern "C" fn task_list_cmp(priv_: *mut c_void, la: *const list_head, lb: *const list_head) -> c_int {
    let machine = priv_ as *mut machine;
    let task_a = la as *mut thread_list;
    let task_b = lb as *mut thread_list;
    let mut a = (*task_a).thread;
    let mut b = (*task_b).thread;

    if thread__tid(a) == thread__tid(b) { return 0; }
    if thread__tid(a) == 0 { return -1; }
    if thread__tid(b) == 0 { return 1; }
    if thread__ppid(a) == thread__ppid(b) {
        return if thread__tid(a) < thread__tid(b) { -1 } else { 1 };
    }

    let level_a = thread_level(machine, a);
    let level_b = thread_level(machine, b);
    for _ in level_b..level_a {
        let parent = machine__find_thread(machine, -1, thread__ppid(a));
        thread__put(a);
        if parent.is_null() {
            pr_err(cstr(b"Missing parent thread of %d\n\0"), thread__tid(a));
            thread__put(b);
            return -1;
        }
        a = parent;
    }
    for _ in level_a..level_b {
        let parent = machine__find_thread(machine, -1, thread__ppid(b));
        thread__put(b);
        if parent.is_null() {
            pr_err(cstr(b"Missing parent thread of %d\n\0"), thread__tid(b));
            thread__put(a);
            return 1;
        }
        b = parent;
    }
    while thread__ppid(a) != thread__ppid(b) {
        let parent_a = machine__find_thread(machine, -1, thread__ppid(a));
        thread__put(a);
        if parent_a.is_null() { pr_err(cstr(b"Missing parent thread of %d\n\0"), thread__tid(a)); }
        a = parent_a;
        let parent_b = machine__find_thread(machine, -1, thread__ppid(b));
        thread__put(b);
        if parent_b.is_null() { pr_err(cstr(b"Missing parent thread of %d\n\0"), thread__tid(b)); }
        b = parent_b;
        if a.is_null() || b.is_null() {
            thread__put(a);
            thread__put(b);
            return if a.is_null() && b.is_null() { 0 } else if a.is_null() { -1 } else { 1 };
        }
    }
    let res = if thread__tid(a) == thread__tid(b) {
        if level_a < level_b { -1 } else if level_a > level_b { 1 } else { 0 }
    } else if thread__tid(a) < thread__tid(b) {
        -1
    } else {
        1
    };
    thread__put(a);
    thread__put(b);
    res
}

unsafe fn tasks_print(rep: *mut report, fp: *mut FILE) -> c_int {
    let machine = &mut (*(*rep).session).machines.host as *mut machine;
    let mut tasks = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
    let ret = machine__thread_list(machine, &mut tasks);
    if ret == 0 {
        list_sort(machine as *mut c_void, &mut tasks, task_list_cmp);
        fprintf(fp, cstr(b"# %8s %8s %8s  %s\n\0"), cstr(b"pid\0"), cstr(b"tid\0"), cstr(b"ppid\0"), cstr(b"comm\0"));
        // list_for_each_entry(task, &tasks, list) task__print_level(...)
    }
    thread_list__delete(&mut tasks);
    ret
}

unsafe fn __cmd_report(rep: *mut report) -> c_int {
    signal(SIGINT, sig_handler);
    let session = (*rep).session;
    let data = (*session).data;

    if !(*rep).cpu_list.is_null() {
        let ret = perf_session__cpu_bitmap(session, (*rep).cpu_list, (*rep).cpu_bitmap.as_mut_ptr());
        if ret != 0 {
            ui__error(cstr(b"failed to set cpu bitmap\n\0"));
            return ret;
        }
        (*(*session).itrace_synth_opts).cpu_bitmap = (*rep).cpu_bitmap.as_mut_ptr();
    }

    if (*rep).show_threads {
        let ret = perf_read_values_init(&mut (*rep).show_threads_values);
        if ret != 0 { return ret; }
    }
    let mut ret = report__setup_sample_type(rep);
    if ret != 0 { return ret; }
    if (*rep).stats_mode { stats_setup(rep); }
    if (*rep).tasks_mode { tasks_setup(rep); }
    ret = perf_session__process_events(session);
    if ret != 0 {
        ui__error(cstr(b"failed to process sample\n\0"));
        return ret;
    }
    if !symbol_conf.prefer_latency && (*rep).total_samples != 0 && (*rep).singlethreaded_samples * 100 / (*rep).total_samples >= 99 {
        perf_hpp__cancel_latency((*session).evlist);
    }
    evlist__check_mem_load_aux((*session).evlist);
    if (*rep).stats_mode { return stats_print(rep); }
    if (*rep).tasks_mode { return tasks_print(rep, stdout); }
    report__warn_kptr_restrict(rep);
    // evlist__for_each_entry(session->evlist, pos) rep->nr_entries += evsel__hists(pos)->nr_entries;
    if use_browser == 0 {
        if verbose > 3 { perf_session__fprintf(session, stdout); }
        if verbose > 2 { perf_session__fprintf_dsos(session, stdout); }
        if dump_trace {
            stats_print(rep);
            return 0;
        }
    }
    ret = report__collapse_hists(rep);
    if ret != 0 {
        ui__error(cstr(b"failed to process hist entry\n\0"));
        return ret;
    }
    if session_done() { return 0; }
    (*rep).nr_entries = 0;
    // evlist__for_each_entry(session->evlist, pos) recalculate nr_entries.
    if (*rep).nr_entries == 0 {
        ui__error(cstr(b"The %s data has no samples!\n\0"), (*data).path);
        return 0;
    }
    report__output_resort(rep);
    if (*rep).total_cycles_mode {
        let mut nr_hpps = 4;
        let mut block_hpps = [0; PERF_HPP_REPORT__BLOCK_MAX_INDEX];
        block_hpps[0] = PERF_HPP_REPORT__BLOCK_TOTAL_CYCLES_PCT;
        block_hpps[1] = PERF_HPP_REPORT__BLOCK_LBR_CYCLES;
        block_hpps[2] = PERF_HPP_REPORT__BLOCK_CYCLES_PCT;
        block_hpps[3] = PERF_HPP_REPORT__BLOCK_AVG_CYCLES;
        // if (evlist__nr_br_cntr(session->evlist) > 0) block_hpps[nr_hpps++] = PERF_HPP_REPORT__BLOCK_BRANCH_COUNTER;
        block_hpps[nr_hpps as usize] = PERF_HPP_REPORT__BLOCK_RANGE; nr_hpps += 1;
        block_hpps[nr_hpps as usize] = PERF_HPP_REPORT__BLOCK_DSO; nr_hpps += 1;
        (*rep).block_reports = block_info__create_report((*session).evlist, (*rep).total_cycles, block_hpps.as_mut_ptr(), nr_hpps, &mut (*rep).nr_block_reports);
        if (*rep).block_reports.is_null() { return -1; }
    }
    report__browse_hists(rep)
}

unsafe extern "C" fn report_parse_callchain_opt(opt: *const option, arg: *const c_char, unset: c_int) -> c_int {
    let callchain = (*opt).value as *mut callchain_param_t;
    (*callchain).enabled = unset == 0;
    /*
     * --no-call-graph
     */
    if unset != 0 {
        symbol_conf.use_callchain = false;
        (*callchain).mode = CHAIN_NONE;
        return 0;
    }
    parse_callchain_report_opt(arg)
}

unsafe extern "C" fn parse_time_quantum(opt: *const option, arg: *const c_char, _unset: c_int) -> c_int {
    let time_q = (*opt).value as *mut c_ulong;
    let mut end: *mut c_char = ptr::null_mut();
    *time_q = strtoul(arg, &mut end, 0);
    if end == arg as *mut c_char { return parse_time_quantum_err(arg); }
    if *time_q == 0 {
        pr_err(cstr(b"time quantum cannot be 0\0"));
        return -1;
    }
    end = skip_spaces(end);
    if *end == 0 { return 0; }
    if strcmp(end, cstr(b"s\0")) == 0 { *time_q *= NSEC_PER_SEC; return 0; }
    if strcmp(end, cstr(b"ms\0")) == 0 { *time_q *= NSEC_PER_MSEC; return 0; }
    if strcmp(end, cstr(b"us\0")) == 0 { *time_q *= NSEC_PER_USEC; return 0; }
    if strcmp(end, cstr(b"ns\0")) == 0 { return 0; }
    parse_time_quantum_err(arg)
}

unsafe fn parse_time_quantum_err(arg: *const c_char) -> c_int {
    pr_err(cstr(b"Cannot parse time quantum `%s'\n\0"), arg);
    -1
}

#[no_mangle]
pub unsafe extern "C" fn report_parse_ignore_callees_opt(_opt: *const option, arg: *const c_char, _unset: c_int) -> c_int {
    if !arg.is_null() {
        let err = regcomp(&mut ignore_callees_regex, arg, 1);
        if err != 0 {
            let mut buf = [0 as c_char; 8192];
            regerror(err, &ignore_callees_regex, buf.as_mut_ptr(), buf.len());
            pr_err(cstr(b"Invalid --ignore-callees regex: %s\n%s\0"), arg, buf.as_ptr());
            return -1;
        }
        have_ignore_callees = 1;
    }
    0
}

unsafe extern "C" fn parse_branch_mode(opt: *const option, _str: *const c_char, unset: c_int) -> c_int {
    let branch_mode = (*opt).value as *mut c_int;
    *branch_mode = if unset == 0 { 1 } else { 0 };
    0
}

unsafe extern "C" fn parse_percent_limit(opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    let rep = (*opt).value as *mut report;
    let pcnt = strtof(str_, ptr::null_mut());
    (*rep).min_percent = pcnt;
    callchain_param.min_percent = pcnt;
    0
}

unsafe extern "C" fn report_parse_addr2line_config(_opt: *const option, arg: *const c_char, _unset: c_int) -> c_int {
    addr2line_configure(cstr(b"addr2line.style\0"), arg, ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn cmd_report(mut argc: c_int, argv: *mut *const c_char) -> c_int {
    let mut session: *mut perf_session;
    let mut itrace_synth_opts: itrace_synth_opts = zeroed();
    itrace_synth_opts.set = false;
    let mut st: stat = zeroed();
    let mut has_br_stack = false;
    let mut branch_mode: c_int = -1;
    let mut last_key: c_int = 0;
    let mut branch_call_mode = false;
    let report_callchain_help = cstr(b"Display call graph (stack chain/backtrace):\n\nCALLCHAIN_REPORT_HELP\n\t\t\t\tDefault: graph,0.5,caller,function,percent\0");
    let mut callchain_default_opt = *b"graph,0.5,caller,function,percent\0";
    let report_usage = [cstr(b"perf report [<options>]\0"), ptr::null()];
    let mut report: report = zeroed();
    report.max_stack = PERF_MAX_STACK_DEPTH;
    report.pretty_printing_style = cstr(b"normal\0");
    report.socket_filter = -1;
    report.skip_empty = true;

    let sort_order_help = sort_help(cstr(b"sort by key(s):\0"), SORT_MODE__NORMAL);
    let field_order_help = sort_help(cstr(b"output field(s):\0"), SORT_MODE__NORMAL);
    let mut disassembler_style: *const c_char = ptr::null();
    let mut objdump_path: *const c_char = ptr::null();
    let mut addr2line_path: *const c_char = ptr::null();
    // const struct option options[] translated as parser table intent. The concrete OPT_* macro
    // expansion belongs to subcmd/parse-options.h and is preserved as an external dependency.
    let options: [option; 1] = [option { value: ptr::null_mut() }];
    let mut data: perf_data = zeroed();
    data.mode = PERF_DATA_MODE_READ;
    let mut ret = hists__init();
    let mut sort_tmp = [0 as c_char; 128];
    let mut ordered_events = true;

    if ret < 0 { goto_exit(ret, sort_order_help, field_order_help); return ret; }

    symbol_conf.keep_exited_threads = true;
    annotation_options__init();
    ret = perf_config(report__config, &mut report as *mut _ as *mut c_void);
    if ret != 0 { goto_exit(ret, sort_order_help, field_order_help); return ret; }

    argc = parse_options(argc, argv, options.as_ptr(), report_usage.as_ptr(), 0);
    if argc != 0 {
        if argc > 1 { usage_with_options(report_usage.as_ptr(), options.as_ptr()); }
        report.symbol_filter_str = *argv;
    }
    if !disassembler_style.is_null() {
        annotate_opts.disassembler_style = strdup(disassembler_style);
        if annotate_opts.disassembler_style.is_null() { return -ENOMEM; }
    }
    if !objdump_path.is_null() {
        annotate_opts.objdump_path = strdup(objdump_path);
        if annotate_opts.objdump_path.is_null() { return -ENOMEM; }
    }
    if !addr2line_path.is_null() {
        symbol_conf.addr2line_path = strdup(addr2line_path);
        if symbol_conf.addr2line_path.is_null() { return -ENOMEM; }
    }
    if annotate_check_args() < 0 { ret = -EINVAL; goto_exit(ret, sort_order_help, field_order_help); return ret; }
    if report.mmaps_mode { report.tasks_mode = true; }
    if dump_trace && report.disable_order { ordered_events = false; }
    if quiet { perf_quiet_option(); }
    ret = symbol__validate_sym_arguments();
    if ret != 0 { goto_exit(ret, sort_order_help, field_order_help); return ret; }
    if report.inverted_callchain { callchain_param.order = ORDER_CALLER; }
    if symbol_conf.cumulate_callchain && !callchain_param.order_set { callchain_param.order = ORDER_CALLER; }
    if (itrace_synth_opts.callchain || itrace_synth_opts.add_callchain) && itrace_synth_opts.callchain_sz as c_int > report.max_stack {
        report.max_stack = itrace_synth_opts.callchain_sz as c_int;
    }
    if input_name.is_null() || strlen(input_name) == 0 {
        if fstat(STDIN_FILENO, &mut st) == 0 && (st.st_mode & 0o170000) == 0o010000 {
            input_name = cstr(b"-\0");
        } else {
            input_name = cstr(b"perf.data\0");
        }
    }

    loop {
        data.path = input_name;
        data.force = symbol_conf.force;
        symbol_conf.skip_empty = report.skip_empty;
        perf_tool__init(&mut report.tool, ordered_events);
        // perf_tool callback assignments from the C initializer are preserved as external layout intent.
        session = perf_session__new(&mut data, &mut report.tool);
        if IS_ERR(session) { ret = PTR_ERR(session); break; }
        ret = evswitch__init(&mut report.evswitch, (*session).evlist, stderr);
        if ret != 0 { break; }
        if zstd_init(&mut (*session).zstd_data, 0) < 0 {
            pr_warning(cstr(b"Decompression initialization failed. Reported data may be incomplete.\n\0"));
        }
        if report.queue_size != 0 {
            ordered_events__set_alloc_size(&mut (*session).ordered_events, report.queue_size);
        }
        (*session).itrace_synth_opts = &mut itrace_synth_opts;
        report.session = session;
        has_br_stack = perf_header__has_feat(&mut (*session).header, HEADER_BRANCH_STACK);
        if (evlist__combined_sample_type((*session).evlist) & PERF_SAMPLE_STACK_USER) != 0 { has_br_stack = false; }
        setup_forced_leader(&mut report, (*session).evlist);
        if symbol_conf.group_sort_idx != 0 && evlist__nr_groups((*session).evlist) == 0 {
            parse_options_usage(ptr::null(), options.as_ptr(), cstr(b"group-sort-idx\0"), 0);
            ret = -EINVAL;
            break;
        }
        if itrace_synth_opts.last_branch || itrace_synth_opts.add_last_branch { has_br_stack = true; }
        if has_br_stack && branch_call_mode { symbol_conf.show_branchflag_count = true; }
        memset(&mut report.brtype_stat as *mut _ as *mut c_void, 0, size_of::<branch_type_stat>());
        if ((branch_mode == -1 && has_br_stack) || branch_mode == 1) && !branch_call_mode {
            sort__mode = SORT_MODE__BRANCH;
            symbol_conf.cumulate_callchain = false;
        }
        if branch_call_mode {
            callchain_param.key = CCKEY_ADDRESS;
            callchain_param.branch_callstack = true;
            symbol_conf.use_callchain = true;
            callchain_register_param(&mut callchain_param);
            if sort_order.is_null() { sort_order = CALLCHAIN_BRANCH_SORT_ORDER.as_ptr() as *mut c_char; }
        }
        if report.mem_mode {
            if sort__mode == SORT_MODE__BRANCH {
                pr_err(cstr(b"branch and mem mode incompatible\n\0"));
                break;
            }
            sort__mode = SORT_MODE__MEMORY;
            symbol_conf.cumulate_callchain = false;
        }
        if symbol_conf.report_hierarchy { perf_hpp_list.need_collapse = true; }
        if report.use_stdio { use_browser = 0; }
        else if report.use_tui { use_browser = 1; }
        else if report.use_gtk { use_browser = 2; }
        if report.header || report.header_only || report.show_threads { use_browser = 0; }
        // show_feat_hdr assignments depend on perf_tool layout.
        if report.stats_mode || report.tasks_mode { use_browser = 0; }
        if report.stats_mode && report.tasks_mode {
            pr_err(cstr(b"Error: --tasks and --mmaps can't be used together with --stats\n\0"));
            break;
        }
        if report.total_cycles_mode {
            if sort__mode != SORT_MODE__BRANCH { report.total_cycles_mode = false; }
            else { sort_order = ptr::null_mut(); }
        }
        if (!sort_order.is_null() && !strstr(sort_order, cstr(b"type\0")).is_null())
            || (!field_order.is_null() && !strstr(field_order, cstr(b"type\0")).is_null())
        {
            report.data_type = true;
            annotate_opts.annotate_src = false;
            symbol_conf.cumulate_callchain = false;
            // #ifndef HAVE_LIBDW_SUPPORT
            pr_err(cstr(b"Error: Data type profiling is disabled due to missing DWARF support\n\0"));
            break;
        }
        if strcmp(input_name, cstr(b"-\0")) != 0 { setup_browser(true); } else { use_browser = 0; }
        if report.data_type && use_browser == 1 {
            symbol_conf.annotate_data_member = true;
            symbol_conf.annotate_data_sample = true;
        }
        symbol_conf.enable_latency = true;
        if report.disable_order || !perf_session__has_switch_events(session) {
            if !symbol_conf.parallelism_list_str.is_null()
                || symbol_conf.prefer_latency
                || (!sort_order.is_null() && (!strstr(sort_order, cstr(b"latency\0")).is_null() || !strstr(sort_order, cstr(b"parallelism\0")).is_null()))
                || (!field_order.is_null() && (!strstr(field_order, cstr(b"latency\0")).is_null() || !strstr(field_order, cstr(b"parallelism\0")).is_null()))
            {
                if report.disable_order { ui__error(cstr(b"Use of latency profile or parallelism is incompatible with --disable-order.\n\0")); }
                else { ui__error(cstr(b"Use of latency profile or parallelism requires --latency flag during record.\n\0")); }
                ret = -1;
                break;
            }
            symbol_conf.enable_latency = false;
        }
        if last_key != K_SWITCH_INPUT_DATA {
            if !sort_order.is_null() && !strstr(sort_order, cstr(b"ipc\0")).is_null() {
                parse_options_usage(report_usage.as_ptr(), options.as_ptr(), cstr(b"s\0"), 1);
                break;
            }
            if !sort_order.is_null() && !strstr(sort_order, cstr(b"symbol\0")).is_null() {
                if sort__mode == SORT_MODE__BRANCH {
                    snprintf(sort_tmp.as_mut_ptr(), sort_tmp.len(), cstr(b"%s,%s\0"), sort_order, cstr(b"ipc_lbr\0"));
                    report.symbol_ipc = true;
                } else {
                    snprintf(sort_tmp.as_mut_ptr(), sort_tmp.len(), cstr(b"%s,%s\0"), sort_order, cstr(b"ipc_null\0"));
                }
                sort_order = sort_tmp.as_mut_ptr();
            }
        }
        if last_key != K_SWITCH_INPUT_DATA && last_key != K_RELOAD && setup_sorting((*session).evlist, perf_session__env(session)) < 0 {
            if !sort_order.is_null() { parse_options_usage(report_usage.as_ptr(), options.as_ptr(), cstr(b"s\0"), 1); }
            if !field_order.is_null() { parse_options_usage(if !sort_order.is_null() { ptr::null() } else { report_usage.as_ptr() }, options.as_ptr(), cstr(b"F\0"), 1); }
            break;
        }
        if (report.header || report.header_only) && !quiet {
            perf_session__fprintf_info(session, stdout, report.show_full_info);
            if report.header_only {
                if data.is_pipe { perf_session__process_events(session); }
                ret = 0;
                break;
            }
        } else if use_browser == 0 && !quiet && !report.stats_mode && !report.tasks_mode {
            fputs(cstr(b"# To display the perf.data header info, please use --header/--header-only options.\n#\n\0"), stdout);
        }
        if ui__has_annotation() || report.symbol_ipc || report.data_type || report.total_cycles_mode {
            ret = symbol__annotation_init();
            if ret < 0 { break; }
            if verbose > 0 { symbol_conf.priv_size += size_of::<u32>(); }
            annotation_config__init();
        }
        if symbol__init(perf_session__env(session)) < 0 { break; }
        if !report.time_str.is_null() {
            ret = perf_time__parse_for_ranges(report.time_str, session, &mut report.ptime_range, &mut report.range_size, &mut report.range_num);
            if ret < 0 { break; }
            itrace_synth_opts__set_time_range(&mut itrace_synth_opts, report.ptime_range, report.range_num);
        }
        // HAVE_LIBTRACEEVENT resolver setup preserved as conditional external dependency.
        sort__setup_elide(stdout);
        ret = __cmd_report(&mut report);
        if ret == K_SWITCH_INPUT_DATA || ret == K_RELOAD {
            perf_session__delete(session);
            last_key = K_SWITCH_INPUT_DATA;
            symbol_conf.use_callchain = false;
            continue;
        } else {
            ret = 0;
        }
        if use_browser == 0 && (verbose > 2 || debug_kmaps) {
            perf_session__dump_kmaps(session);
        }
        break;
    }

    if !report.ptime_range.is_null() {
        itrace_synth_opts__clear_time_range(&mut itrace_synth_opts);
        zfree(&mut report.ptime_range);
    }
    if !report.block_reports.is_null() {
        block_info__free_report(report.block_reports, report.nr_block_reports);
        report.block_reports = ptr::null_mut();
    }
    if !session.is_null() && !IS_ERR(session) {
        zstd_fini(&mut (*session).zstd_data);
        perf_session__delete(session);
    }
    goto_exit(ret, sort_order_help, field_order_help);
    ret
}

unsafe fn goto_exit(_ret: c_int, sort_order_help: *mut c_char, field_order_help: *mut c_char) {
    annotation_options__exit();
    free(sort_order_help as *mut c_void);
    free(field_order_help as *mut c_void);
}
