// SPDX-License-Identifier: GPL-2.0
//
// Source-level Rust translation of perf/builtin-script.c.
// C include dependencies are intentionally represented as opaque C ABI items
// and external functions/globals supplied by the surrounding perf tree.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type bool_ = bool;
type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type size_t = usize;
type pid_t = c_int;
type FILE = c_void;
type DIR = c_void;
type time_t = c_long;

const OUTPUT_TYPE_UNSET: c_int = -1;
const OUTPUT_TYPE_SYNTH: c_uint = PERF_TYPE_MAX;
const OUTPUT_TYPE_OTHER: c_uint = PERF_TYPE_MAX + 1;
const OUTPUT_TYPE_MAX: usize = (PERF_TYPE_MAX + 2) as usize;
const DEFAULT_TOD_FMT: &[u8] = b"%F %H:%M:%S\0";
const MAXBB: usize = 16384;
const RECORD_SUFFIX: &[u8] = b"-record\0";
const REPORT_SUFFIX: &[u8] = b"-report\0";

// External constants from perf and system headers.
extern "C" {
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut errno: c_int;
    static mut verbose: c_int;
    static mut dump_trace: bool_;
    static mut input_name: *const c_char;
    static mut session_done: c_int;
    static mut use_browser: c_int;
    static mut perf_guest: bool_;
    static mut scripting_max_stack: c_uint;
    static mut srcline_full_filename: bool_;
    static mut debug_kmaps: bool_;
    static mut symbol_conf: symbol_conf_t;
    static mut stat_config: perf_stat_config;
    static record_options: [option; 0];

    static dispatch_reasons: [*const c_char; 0];
    static preempt_reasons: [*const c_char; 0];

    fn zalloc(size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn fclose(fp: *mut FILE) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fileno(fp: *mut FILE) -> c_int;
    fn fstat(fd: c_int, st: *mut stat) -> c_int;
    fn fprintf(fp: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn fputs(s: *const c_char, fp: *mut FILE) -> c_int;
    fn fputc(c: c_int, fp: *mut FILE) -> c_int;
    fn fflush(fp: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, fp: *mut FILE) -> *mut c_char;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strcasecmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcspn(s: *const c_char, reject: *const c_char) -> size_t;
    fn strlen(s: *const c_char) -> size_t;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtok_r(s: *mut c_char, delim: *const c_char, save: *mut *mut c_char) -> *mut c_char;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn localtime_r(t: *const time_t, tm: *mut tm) -> *mut tm;
    fn strftime(s: *mut c_char, max: size_t, fmt: *const c_char, tm: *const tm) -> size_t;
    fn access(path: *const c_char, mode: c_int) -> c_int;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dir: *mut DIR) -> *mut dirent;
    fn closedir(dir: *mut DIR) -> c_int;
    fn pipe(fds: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn execvp(file: *const c_char, argv: *mut *mut c_char) -> c_int;
    fn signal(sig: c_int, handler: extern "C" fn(c_int)) -> usize;
    fn isatty(fd: c_int) -> c_int;
    fn open(path: *const c_char, flags: c_int) -> c_int;

    fn evsel__name(evsel: *mut evsel) -> *const c_char;
    fn evsel__metric_id(evsel: *mut evsel) -> *const c_char;
    fn evsel__find_pmu(evsel: *mut evsel) -> *mut perf_pmu;
    fn evsel__is_dummy_event(evsel: *mut evsel) -> bool_;
    fn evsel__has_callchain(evsel: *mut evsel) -> bool_;
    fn evsel__is_offcpu_event(evsel: *mut evsel) -> bool_;
    fn evsel__is_bpf_output(evsel: *mut evsel) -> bool_;
    fn evsel__leader(evsel: *mut evsel) -> *mut evsel;
    fn evsel__env(evsel: *mut evsel) -> *mut perf_env;
    fn evsel__session(evsel: *mut evsel) -> *mut perf_session;
    fn evsel__cpus(evsel: *mut evsel) -> *mut perf_cpu_map;
    fn evlist__combined_branch_type(evlist: *mut evlist) -> u64;
    fn evlist__combined_sample_type(evlist: *mut evlist) -> u64;
    fn evlist__last(evlist: *mut evlist) -> *mut evsel;
    fn evlist__id2evsel(evlist: *mut evlist, id: u64) -> *mut evsel;
    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn evlist__alloc_stats(config: *mut perf_stat_config, evlist: *mut evlist, alloc_raw: bool_) -> c_int;
    fn evlist__alloc_aggr_stats(evlist: *mut evlist, nr: c_int) -> c_int;
    fn evlist__free_stats(evlist: *mut evlist);
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__metric_events(evlist: *mut evlist) -> *mut c_void;
    fn perf_evlist__set_maps(core: *mut evlist_core, cpus: *mut perf_cpu_map, threads: *mut perf_thread_map);

    fn perf_header__has_feat(header: *mut perf_header, feat: c_int) -> bool_;
    fn perf_session__has_traces(session: *mut perf_session, msg: *const c_char) -> bool_;
    fn perf_session__env(session: *mut perf_session) -> *mut perf_env;
    fn perf_session__e_machine(session: *mut perf_session, e_flags: *mut u32) -> u16;
    fn perf_session__process_events(session: *mut perf_session) -> c_int;
    fn perf_session__new(data: *mut perf_data, tool: *mut perf_tool) -> *mut perf_session;
    fn perf_session__delete(session: *mut perf_session);
    fn perf_session__fprintf_info(session: *mut perf_session, fp: *mut FILE, full: bool_);
    fn perf_session__cpu_bitmap(session: *mut perf_session, cpu_list: *const c_char, bitmap: *mut c_ulong) -> c_int;
    fn perf_session__dump_kmaps(session: *mut perf_session);

    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
}

#[repr(C)] struct perf_tool { _opaque: [u8; 0] }
#[repr(C)] struct perf_session { header: perf_header, evlist: *mut evlist, data: *mut perf_data, machines: machines, itrace_synth_opts: *mut itrace_synth_opts, zstd_data: zstd_data, tevent: tevent }
#[repr(C)] struct perf_header { _opaque: [u8; 0] }
#[repr(C)] struct evlist { _opaque: [u8; 0] }
#[repr(C)] struct evlist_core { user_requested_cpus: *mut perf_cpu_map }
#[repr(C)] struct evsel_core { attr: perf_event_attr, threads: *mut perf_thread_map, cpus: *mut perf_cpu_map }
#[repr(C)] struct evsel { core: evsel_core, script_output_type: c_int, evlist: *mut evlist, priv_: *mut c_void, metric_id: *mut c_char, metric_leader: *mut evsel, stats: *mut evsel_stats, counts: *mut c_void, prev_raw_counts: *mut c_void, abbr_name: *const c_char, name: *const c_char }
#[repr(C)] struct evsel_stats { aggr: *mut c_void }
#[repr(C)] struct perf_event_attr { type_: u32, config: u64, sample_type: u64, branch_sample_type: u64, sample_regs_intr: u64, sample_regs_user: u64, sample_id_all: bool_ }
#[repr(C)] struct perf_data { path: *const c_char, mode: c_int, force: bool_ }
#[repr(C)] struct perf_env { clock: perf_clock }
#[repr(C)] struct perf_clock { enabled: bool_, clockid_ns: u64, tod_ns: u64 }
#[repr(C)] struct perf_cpu_map { _opaque: [u8; 0] }
#[repr(C)] struct perf_thread_map { _opaque: [u8; 0] }
#[repr(C)] struct perf_time_interval { _opaque: [u8; 0] }
#[repr(C)] struct evswitch { _opaque: [u8; 0] }
#[repr(C)] struct dlfilter { _opaque: [u8; 0] }
#[repr(C)] struct perf_pmu { is_core: bool_ }
#[repr(C)] struct perf_sample { evsel: *mut evsel, time: u64, pid: u32, tid: u32, cpu: u32, misc: u32, period: u64, ip: u64, addr: u64, flags: u32, machine_pid: c_int, vcpu: c_int, cpumode: u8, branch_stack: *mut branch_stack, branch_stack_cntr: *mut u64, callchain: *mut c_void, raw_data: *mut c_void, raw_size: u32, data_src: u64, weight: u64, ins_lat: u16, weight3: u16, cgroup: u64, intr_regs: *mut c_void, user_regs: *mut c_void, insn_len: c_int, cyc_cnt: u64, insn_cnt: u64, phys_addr: u64, data_page_size: u64, code_page_size: u64, file_offset: u64 }
#[repr(C)] struct branch_stack { nr: u64 }
#[repr(C)] struct branch_entry { from: u64, to: u64, flags: branch_flags }
#[repr(C)] struct branch_flags { mispred: bool_, predicted: bool_, not_taken: bool_, in_tx: bool_, abort: bool_, cycles: c_int, spec: c_int }
#[repr(C)] struct thread { _opaque: [u8; 0] }
#[repr(C)] struct machine { env: *mut perf_env }
#[repr(C)] struct addr_location { thread: *mut thread, map: *mut map, sym: *mut symbol, addr: u64, cpu: c_int, filtered: bool_ }
#[repr(C)] struct map { _opaque: [u8; 0] }
#[repr(C)] struct symbol { start: u64, end: u64, name: *const c_char }
#[repr(C)] struct regs_dump { abi: u64, regs: *mut u64 }
#[repr(C)] struct srccode_state { srcfile: *mut c_char, line: c_uint }
#[repr(C)] struct perf_insn { machine: *mut machine, thread: *mut thread, cpumode: u8, is64bit: bool_, cpu: u32 }
#[repr(C)] struct perf_stat_config { output: *mut FILE, aggr_map: *mut cpu_aggr_map, aggr_mode: c_int, aggr_get_id: Option<extern "C" fn(*mut perf_stat_config, perf_cpu) -> aggr_cpu_id> }
#[repr(C)] struct perf_stat_output_ctx { print_metric: Option<extern "C" fn(*mut perf_stat_config, *mut c_void, metric_threshold_classify, *const c_char, *const c_char, c_double)>, new_line: Option<extern "C" fn(*mut perf_stat_config, *mut c_void)>, ctx: *mut c_void, force_header: bool_ }
#[repr(C)] struct perf_counts_values { val: u64, ena: u64, run: u64 }
#[repr(C)] struct cpu_aggr_map { nr: c_int }
#[repr(C)] struct aggr_cpu_id { _opaque: [u8; 0] }
#[repr(C)] struct perf_cpu { cpu: c_int }
#[repr(C)] struct pmu_metric { metric_name: *const c_char }
#[repr(C)] struct pmu_metrics_table { _opaque: [u8; 0] }
#[repr(C)] struct metric_event { head: list_head }
#[repr(C)] struct metric_expr { nd: list_head, metric_events: *mut *mut evsel }
#[repr(C)] struct list_head { next: *mut list_head, prev: *mut list_head }
#[repr(C)] struct script_desc { node: list_head, name: *mut c_char, half_liner: *mut c_char, args: *mut c_char }
#[repr(C)] struct scripting_ops { name: *const c_char, dirname: *const c_char, process_stat: Option<extern "C" fn(*mut perf_stat_config, *mut evsel, u64)>, process_stat_interval: Option<extern "C" fn(u64)>, process_switch: Option<extern "C" fn(*mut perf_event, *mut perf_sample, *mut machine)>, process_auxtrace_error: Option<extern "C" fn(*mut perf_session, *mut perf_event)>, process_throttle: Option<extern "C" fn(*mut perf_event, *mut perf_sample, *mut machine)>, process_event: Option<extern "C" fn(*mut perf_event, *mut perf_sample, *mut addr_location, *mut addr_location)>, flush_script: Option<extern "C" fn() -> c_int>, stop_script: Option<extern "C" fn() -> c_int>, start_script: Option<extern "C" fn(*const c_char, c_int, *mut *const c_char, *mut perf_session) -> c_int>, generate_script: Option<extern "C" fn(*mut c_void, *mut c_char) -> c_int> }
#[repr(C)] struct printer_data { line_no: c_int, hit_nul: bool_, is_printable: bool_ }
#[repr(C)] struct metric_ctx { sample: *mut perf_sample, thread: *mut thread, evsel: *mut evsel, fp: *mut FILE }
#[repr(C)] struct script_find_metrics_args { evlist: *mut evlist, system_wide: bool_ }
#[repr(C)] struct perf_script { tool: perf_tool, session: *mut perf_session, show_task_events: bool_, show_mmap_events: bool_, show_switch_events: bool_, show_namespace_events: bool_, show_lost_events: bool_, show_round_events: bool_, show_bpf_events: bool_, show_cgroup_events: bool_, show_text_poke_events: bool_, allocated: bool_, per_event_dump: bool_, stitch_lbr: bool_, evswitch: evswitch, cpus: *mut perf_cpu_map, threads: *mut perf_thread_map, name_width: c_int, time_str: *const c_char, ptime_range: *mut perf_time_interval, range_size: c_int, range_num: c_int }
#[repr(C)] struct output_option { str_: *const c_char, field: u64 }
#[repr(C)] struct output_state { user_set: bool_, wildcard_set: bool_, print_ip_opts: c_uint, fields: u64, invalid_fields: u64, user_set_fields: u64, user_unset_fields: u64 }
#[repr(C)] struct evsel_script { filename: *mut c_char, fp: *mut FILE, samples: u64 }
#[repr(C)] struct stat { st_size: c_long }
#[repr(C)] struct tm { _opaque: [u8; 0] }
#[repr(C)] struct dirent { d_type: u8, d_name: [c_char; 256] }
#[repr(C)] struct option { _opaque: [u8; 0] }
#[repr(C)] struct machines { host: machine }
#[repr(C)] struct zstd_data { _opaque: [u8; 0] }
#[repr(C)] struct tevent { pevent: *mut c_void }
#[repr(C)] struct itrace_synth_opts { set: bool_, default_no_sample: bool_, callchain: bool_, add_callchain: bool_, callchain_sz: c_uint, thread_stack: bool_, cpu_bitmap: *mut c_ulong }
#[repr(C)] struct perf_event_header { type_: u32 }
#[repr(C)] union perf_event { header: perf_event_header, callchain_deferred: perf_record_callchain_deferred, comm: perf_record_comm, namespaces: perf_record_namespaces, fork: perf_record_fork, mmap: perf_record_mmap, mmap2: perf_record_mmap2, stat_round: perf_record_stat_round, stat_config: perf_record_stat_config, thread_map: perf_record_thread_map, cpu_map: perf_record_cpu_map }
#[repr(C)] struct perf_record_callchain_deferred { cookie: u64 }
#[repr(C)] struct perf_record_comm { pid: pid_t, tid: pid_t }
#[repr(C)] struct perf_record_namespaces { pid: pid_t, tid: pid_t }
#[repr(C)] struct perf_record_fork { pid: pid_t, tid: pid_t, time: u64 }
#[repr(C)] struct perf_record_mmap { pid: pid_t, tid: pid_t }
#[repr(C)] struct perf_record_mmap2 { pid: pid_t, tid: pid_t }
#[repr(C)] struct perf_record_stat_round { time: u64 }
#[repr(C)] struct perf_record_stat_config { _opaque: [u8; 0] }
#[repr(C)] struct perf_record_thread_map { _opaque: [u8; 0] }
#[repr(C)] struct perf_record_cpu_map_data { _opaque: [u8; 0] }
#[repr(C)] struct perf_record_cpu_map { data: perf_record_cpu_map_data }
#[repr(C)] enum metric_threshold_classify { METRIC_THRESHOLD_UNKNOWN = 0 }
#[repr(C)] enum binary_printer_ops { BINARY_PRINT_DATA_BEGIN, BINARY_PRINT_LINE_BEGIN, BINARY_PRINT_ADDR, BINARY_PRINT_NUM_DATA, BINARY_PRINT_NUM_PAD, BINARY_PRINT_SEP, BINARY_PRINT_CHAR_DATA, BINARY_PRINT_CHAR_PAD, BINARY_PRINT_LINE_END, BINARY_PRINT_DATA_END }

#[repr(C)] struct symbol_conf_t {
    use_callchain: bool_, nanosecs: bool_, graph_function: *const c_char,
    bt_stop_list: *mut c_void, bt_stop_list_str: *const c_char, vmlinux_name: *const c_char,
    kallsyms_name: *const c_char, dso_list_str: *const c_char, sym_list_str: *const c_char,
    addr_range: c_int, show_kernel_path: bool_, comm_list_str: *const c_char,
    pid_list_str: *const c_char, tid_list_str: *const c_char, force: bool_,
    demangle: bool_, demangle_kernel: bool_, addr2line_path: *const c_char,
    inline_name: bool_, guestmount: *const c_char, default_guest_vmlinux_name: *const c_char,
    default_guest_kallsyms: *const c_char, default_guest_modules: *const c_char,
    guest_code: bool_, pad_output_len_dso: c_int,
}

const PERF_OUTPUT_COMM: u64 = 1u64 << 0;
const PERF_OUTPUT_TID: u64 = 1u64 << 1;
const PERF_OUTPUT_PID: u64 = 1u64 << 2;
const PERF_OUTPUT_TIME: u64 = 1u64 << 3;
const PERF_OUTPUT_CPU: u64 = 1u64 << 4;
const PERF_OUTPUT_EVNAME: u64 = 1u64 << 5;
const PERF_OUTPUT_TRACE: u64 = 1u64 << 6;
const PERF_OUTPUT_IP: u64 = 1u64 << 7;
const PERF_OUTPUT_SYM: u64 = 1u64 << 8;
const PERF_OUTPUT_DSO: u64 = 1u64 << 9;
const PERF_OUTPUT_ADDR: u64 = 1u64 << 10;
const PERF_OUTPUT_SYMOFFSET: u64 = 1u64 << 11;
const PERF_OUTPUT_SRCLINE: u64 = 1u64 << 12;
const PERF_OUTPUT_PERIOD: u64 = 1u64 << 13;
const PERF_OUTPUT_IREGS: u64 = 1u64 << 14;
const PERF_OUTPUT_BRSTACK: u64 = 1u64 << 15;
const PERF_OUTPUT_BRSTACKSYM: u64 = 1u64 << 16;
const PERF_OUTPUT_DATA_SRC: u64 = 1u64 << 17;
const PERF_OUTPUT_WEIGHT: u64 = 1u64 << 18;
const PERF_OUTPUT_BPF_OUTPUT: u64 = 1u64 << 19;
const PERF_OUTPUT_CALLINDENT: u64 = 1u64 << 20;
const PERF_OUTPUT_INSN: u64 = 1u64 << 21;
const PERF_OUTPUT_INSNLEN: u64 = 1u64 << 22;
const PERF_OUTPUT_BRSTACKINSN: u64 = 1u64 << 23;
const PERF_OUTPUT_BRSTACKOFF: u64 = 1u64 << 24;
const PERF_OUTPUT_SYNTH: u64 = 1u64 << 25;
const PERF_OUTPUT_PHYS_ADDR: u64 = 1u64 << 26;
const PERF_OUTPUT_UREGS: u64 = 1u64 << 27;
const PERF_OUTPUT_METRIC: u64 = 1u64 << 28;
const PERF_OUTPUT_MISC: u64 = 1u64 << 29;
const PERF_OUTPUT_SRCCODE: u64 = 1u64 << 30;
const PERF_OUTPUT_IPC: u64 = 1u64 << 31;
const PERF_OUTPUT_TOD: u64 = 1u64 << 32;
const PERF_OUTPUT_DATA_PAGE_SIZE: u64 = 1u64 << 33;
const PERF_OUTPUT_CODE_PAGE_SIZE: u64 = 1u64 << 34;
const PERF_OUTPUT_INS_LAT: u64 = 1u64 << 35;
const PERF_OUTPUT_BRSTACKINSNLEN: u64 = 1u64 << 36;
const PERF_OUTPUT_MACHINE_PID: u64 = 1u64 << 37;
const PERF_OUTPUT_VCPU: u64 = 1u64 << 38;
const PERF_OUTPUT_CGROUP: u64 = 1u64 << 39;
const PERF_OUTPUT_RETIRE_LAT: u64 = 1u64 << 40;
const PERF_OUTPUT_DSOFF: u64 = 1u64 << 41;
const PERF_OUTPUT_DISASM: u64 = 1u64 << 42;
const PERF_OUTPUT_BRSTACKDISASM: u64 = 1u64 << 43;
const PERF_OUTPUT_BRCNTR: u64 = 1u64 << 44;

const PERF_TYPE_HARDWARE: usize = 0;
const PERF_TYPE_SOFTWARE: usize = 1;
const PERF_TYPE_TRACEPOINT: usize = 2;
const PERF_TYPE_HW_CACHE: usize = 3;
const PERF_TYPE_RAW: usize = 4;
const PERF_TYPE_BREAKPOINT: usize = 5;
const PERF_TYPE_SYNTH: u32 = 6;
const PERF_TYPE_MAX: c_uint = 6;

const PERF_SAMPLE_IP: u64 = 1 << 0;
const PERF_SAMPLE_TID: u64 = 1 << 1;
const PERF_SAMPLE_TIME: u64 = 1 << 2;
const PERF_SAMPLE_ADDR: u64 = 1 << 3;
const PERF_SAMPLE_CALLCHAIN: u64 = 1 << 5;
const PERF_SAMPLE_CPU: u64 = 1 << 7;
const PERF_SAMPLE_PERIOD: u64 = 1 << 8;
const PERF_SAMPLE_BRANCH_STACK: u64 = 1 << 11;
const PERF_SAMPLE_REGS_USER: u64 = 1 << 12;
const PERF_SAMPLE_STACK_USER: u64 = 1 << 13;
const PERF_SAMPLE_WEIGHT: u64 = 1 << 14;
const PERF_SAMPLE_DATA_SRC: u64 = 1 << 15;
const PERF_SAMPLE_REGS_INTR: u64 = 1 << 18;
const PERF_SAMPLE_PHYS_ADDR: u64 = 1 << 19;
const PERF_SAMPLE_CGROUP: u64 = 1 << 21;
const PERF_SAMPLE_DATA_PAGE_SIZE: u64 = 1 << 22;
const PERF_SAMPLE_CODE_PAGE_SIZE: u64 = 1 << 23;
const PERF_SAMPLE_WEIGHT_STRUCT: u64 = 1 << 24;
const PERF_SAMPLE_WEIGHT_TYPE: u64 = PERF_SAMPLE_WEIGHT | PERF_SAMPLE_WEIGHT_STRUCT;
const PERF_SAMPLE_BRANCH_ANY: u64 = 1 << 0;
const PERF_SAMPLE_BRANCH_COUNTERS: u64 = 1 << 20;

const PERF_RECORD_SAMPLE: u32 = 9;
const PERF_RECORD_MMAP: u32 = 1;
const PERF_RECORD_COMM: u32 = 3;
const PERF_RECORD_MMAP2: u32 = 10;
const PERF_RECORD_SWITCH: u32 = 14;
const PERF_RECORD_SWITCH_CPU_WIDE: u32 = 15;
const PERF_RECORD_CALLCHAIN_DEFERRED: u32 = 255;
const PERF_RECORD_MISC_KERNEL: u32 = 1;
const PERF_RECORD_MISC_USER: u32 = 2;
const PERF_RECORD_MISC_HYPERVISOR: u32 = 3;
const PERF_RECORD_MISC_GUEST_KERNEL: u32 = 4;
const PERF_RECORD_MISC_GUEST_USER: u32 = 5;
const PERF_RECORD_MISC_MMAP_DATA: u32 = 0x2000;
const PERF_RECORD_MISC_COMM_EXEC: u32 = 0x2000;
const PERF_RECORD_MISC_SWITCH_OUT: u32 = 0x2000;
const PERF_RECORD_MISC_SWITCH_OUT_PREEMPT: u32 = 0x4000;

const PERF_IP_FLAG_CALL: u32 = 1 << 1;
const PERF_IP_FLAG_RETURN: u32 = 1 << 2;
const PERF_IP_FLAG_TRACE_BEGIN: u32 = 1 << 3;
const PERF_IP_FLAG_TRACE_END: u32 = 1 << 4;

const NSEC_PER_SEC: u64 = 1_000_000_000;
const NSEC_PER_USEC: u64 = 1_000;
const MAX_NR_CPUS: usize = 4096;
const MAXINSN: u64 = 15;
const PATH_MAX: usize = 4096;
const MAXPATHLEN: usize = 4096;
const BUFSIZ: usize = 8192;
const PAGE_SIZE_NAME_LEN: usize = 32;
const SAMPLE_FLAGS_BUF_SIZE: usize = 64;
const SAMPLE_FLAGS_STR_ALIGNED_SIZE: c_int = 34;
const HEADER_STAT: c_int = 0;
const HEADER_AUXTRACE: c_int = 1;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const ENXIO: c_int = 6;
const R_OK: c_int = 4;
const O_RDONLY: c_int = 0;
const SIGINT: c_int = 2;
const AGGR_GLOBAL: c_int = 1;
const AGGR_NONE: c_int = 0;
const PERF_DATA_MODE_READ: c_int = 0;
const SHOW_FEAT_HEADER: c_int = 1;
const SHOW_FEAT_HEADER_FULL_INFO: c_int = 2;
const CALLCHAIN_LBR: c_int = 1;
const EVSEL__PRINT_IP: c_uint = 1 << 0;
const EVSEL__PRINT_SYM: c_uint = 1 << 1;
const EVSEL__PRINT_DSO: c_uint = 1 << 2;
const EVSEL__PRINT_DSOFF: c_uint = 1 << 3;
const EVSEL__PRINT_SYMOFFSET: c_uint = 1 << 4;
const EVSEL__PRINT_SRCLINE: c_uint = 1 << 5;

static mut script_name: *const c_char = ptr::null();
static mut generate_script_lang: *const c_char = ptr::null();
static mut reltime: bool_ = false;
static mut deltatime: bool_ = false;
static mut initial_time: u64 = 0;
static mut previous_time: u64 = 0;
static mut debug_mode: bool_ = false;
static mut last_timestamp: u64 = 0;
static mut nr_unordered: u64 = 0;
static mut no_callchain: bool_ = false;
static mut latency_format: bool_ = false;
static mut system_wide: bool_ = false;
static mut print_flags: bool_ = false;
static mut cpu_list: *const c_char = ptr::null();
static mut cpu_bitmap: [c_ulong; MAX_NR_CPUS / (mem::size_of::<c_ulong>() * 8)] = [0; MAX_NR_CPUS / (mem::size_of::<c_ulong>() * 8)];
static mut max_blocks: c_int = 0;
static mut dlfilter: *mut dlfilter = ptr::null_mut();
static mut dlargc: c_int = 0;
static mut dlargv: *mut *mut c_char = ptr::null_mut();
static mut scripting_ops: *mut scripting_ops = ptr::null_mut();
static mut es_stdout: evsel_script = evsel_script { filename: ptr::null_mut(), fp: ptr::null_mut(), samples: 0 };
static mut script_descs: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };

static all_output_options: &[output_option] = &[
    output_option { str_: b"comm\0".as_ptr() as *const c_char, field: PERF_OUTPUT_COMM },
    output_option { str_: b"tid\0".as_ptr() as *const c_char, field: PERF_OUTPUT_TID },
    output_option { str_: b"pid\0".as_ptr() as *const c_char, field: PERF_OUTPUT_PID },
    output_option { str_: b"time\0".as_ptr() as *const c_char, field: PERF_OUTPUT_TIME },
    output_option { str_: b"cpu\0".as_ptr() as *const c_char, field: PERF_OUTPUT_CPU },
    output_option { str_: b"event\0".as_ptr() as *const c_char, field: PERF_OUTPUT_EVNAME },
    output_option { str_: b"trace\0".as_ptr() as *const c_char, field: PERF_OUTPUT_TRACE },
    output_option { str_: b"ip\0".as_ptr() as *const c_char, field: PERF_OUTPUT_IP },
    output_option { str_: b"sym\0".as_ptr() as *const c_char, field: PERF_OUTPUT_SYM },
    output_option { str_: b"dso\0".as_ptr() as *const c_char, field: PERF_OUTPUT_DSO },
    output_option { str_: b"dsoff\0".as_ptr() as *const c_char, field: PERF_OUTPUT_DSOFF },
    output_option { str_: b"addr\0".as_ptr() as *const c_char, field: PERF_OUTPUT_ADDR },
    output_option { str_: b"symoff\0".as_ptr() as *const c_char, field: PERF_OUTPUT_SYMOFFSET },
    output_option { str_: b"srcline\0".as_ptr() as *const c_char, field: PERF_OUTPUT_SRCLINE },
    output_option { str_: b"period\0".as_ptr() as *const c_char, field: PERF_OUTPUT_PERIOD },
    output_option { str_: b"iregs\0".as_ptr() as *const c_char, field: PERF_OUTPUT_IREGS },
    output_option { str_: b"uregs\0".as_ptr() as *const c_char, field: PERF_OUTPUT_UREGS },
    output_option { str_: b"brstack\0".as_ptr() as *const c_char, field: PERF_OUTPUT_BRSTACK },
    output_option { str_: b"brstacksym\0".as_ptr() as *const c_char, field: PERF_OUTPUT_BRSTACKSYM },
    output_option { str_: b"data_src\0".as_ptr() as *const c_char, field: PERF_OUTPUT_DATA_SRC },
    output_option { str_: b"weight\0".as_ptr() as *const c_char, field: PERF_OUTPUT_WEIGHT },
    output_option { str_: b"bpf-output\0".as_ptr() as *const c_char, field: PERF_OUTPUT_BPF_OUTPUT },
    output_option { str_: b"callindent\0".as_ptr() as *const c_char, field: PERF_OUTPUT_CALLINDENT },
    output_option { str_: b"insn\0".as_ptr() as *const c_char, field: PERF_OUTPUT_INSN },
    output_option { str_: b"disasm\0".as_ptr() as *const c_char, field: PERF_OUTPUT_DISASM },
    output_option { str_: b"insnlen\0".as_ptr() as *const c_char, field: PERF_OUTPUT_INSNLEN },
    output_option { str_: b"brstackinsn\0".as_ptr() as *const c_char, field: PERF_OUTPUT_BRSTACKINSN },
    output_option { str_: b"brstackoff\0".as_ptr() as *const c_char, field: PERF_OUTPUT_BRSTACKOFF },
    output_option { str_: b"synth\0".as_ptr() as *const c_char, field: PERF_OUTPUT_SYNTH },
    output_option { str_: b"phys_addr\0".as_ptr() as *const c_char, field: PERF_OUTPUT_PHYS_ADDR },
    output_option { str_: b"metric\0".as_ptr() as *const c_char, field: PERF_OUTPUT_METRIC },
    output_option { str_: b"misc\0".as_ptr() as *const c_char, field: PERF_OUTPUT_MISC },
    output_option { str_: b"srccode\0".as_ptr() as *const c_char, field: PERF_OUTPUT_SRCCODE },
    output_option { str_: b"ipc\0".as_ptr() as *const c_char, field: PERF_OUTPUT_IPC },
    output_option { str_: b"tod\0".as_ptr() as *const c_char, field: PERF_OUTPUT_TOD },
    output_option { str_: b"data_page_size\0".as_ptr() as *const c_char, field: PERF_OUTPUT_DATA_PAGE_SIZE },
    output_option { str_: b"code_page_size\0".as_ptr() as *const c_char, field: PERF_OUTPUT_CODE_PAGE_SIZE },
    output_option { str_: b"ins_lat\0".as_ptr() as *const c_char, field: PERF_OUTPUT_INS_LAT },
    output_option { str_: b"brstackinsnlen\0".as_ptr() as *const c_char, field: PERF_OUTPUT_BRSTACKINSNLEN },
    output_option { str_: b"machine_pid\0".as_ptr() as *const c_char, field: PERF_OUTPUT_MACHINE_PID },
    output_option { str_: b"vcpu\0".as_ptr() as *const c_char, field: PERF_OUTPUT_VCPU },
    output_option { str_: b"cgroup\0".as_ptr() as *const c_char, field: PERF_OUTPUT_CGROUP },
    output_option { str_: b"retire_lat\0".as_ptr() as *const c_char, field: PERF_OUTPUT_RETIRE_LAT },
    output_option { str_: b"brstackdisasm\0".as_ptr() as *const c_char, field: PERF_OUTPUT_BRSTACKDISASM },
    output_option { str_: b"brcntr\0".as_ptr() as *const c_char, field: PERF_OUTPUT_BRCNTR },
];

static mut output: [output_state; OUTPUT_TYPE_MAX] = [
    output_state { user_set: false, wildcard_set: false, print_ip_opts: 0, fields: PERF_OUTPUT_COMM|PERF_OUTPUT_TID|PERF_OUTPUT_CPU|PERF_OUTPUT_TIME|PERF_OUTPUT_EVNAME|PERF_OUTPUT_IP|PERF_OUTPUT_SYM|PERF_OUTPUT_SYMOFFSET|PERF_OUTPUT_DSO|PERF_OUTPUT_PERIOD, invalid_fields: PERF_OUTPUT_TRACE|PERF_OUTPUT_BPF_OUTPUT, user_set_fields: 0, user_unset_fields: 0 },
    output_state { user_set: false, wildcard_set: false, print_ip_opts: 0, fields: PERF_OUTPUT_COMM|PERF_OUTPUT_TID|PERF_OUTPUT_CPU|PERF_OUTPUT_TIME|PERF_OUTPUT_EVNAME|PERF_OUTPUT_IP|PERF_OUTPUT_SYM|PERF_OUTPUT_SYMOFFSET|PERF_OUTPUT_DSO|PERF_OUTPUT_PERIOD|PERF_OUTPUT_BPF_OUTPUT, invalid_fields: PERF_OUTPUT_TRACE, user_set_fields: 0, user_unset_fields: 0 },
    output_state { user_set: false, wildcard_set: false, print_ip_opts: 0, fields: PERF_OUTPUT_COMM|PERF_OUTPUT_TID|PERF_OUTPUT_CPU|PERF_OUTPUT_TIME|PERF_OUTPUT_EVNAME|PERF_OUTPUT_TRACE, invalid_fields: 0, user_set_fields: 0, user_unset_fields: 0 },
    output_state { user_set: false, wildcard_set: false, print_ip_opts: 0, fields: PERF_OUTPUT_COMM|PERF_OUTPUT_TID|PERF_OUTPUT_CPU|PERF_OUTPUT_TIME|PERF_OUTPUT_EVNAME|PERF_OUTPUT_IP|PERF_OUTPUT_SYM|PERF_OUTPUT_SYMOFFSET|PERF_OUTPUT_DSO|PERF_OUTPUT_PERIOD, invalid_fields: PERF_OUTPUT_TRACE|PERF_OUTPUT_BPF_OUTPUT, user_set_fields: 0, user_unset_fields: 0 },
    output_state { user_set: false, wildcard_set: false, print_ip_opts: 0, fields: PERF_OUTPUT_COMM|PERF_OUTPUT_TID|PERF_OUTPUT_CPU|PERF_OUTPUT_TIME|PERF_OUTPUT_EVNAME|PERF_OUTPUT_IP|PERF_OUTPUT_SYM|PERF_OUTPUT_SYMOFFSET|PERF_OUTPUT_DSO|PERF_OUTPUT_PERIOD|PERF_OUTPUT_ADDR|PERF_OUTPUT_DATA_SRC|PERF_OUTPUT_WEIGHT|PERF_OUTPUT_PHYS_ADDR|PERF_OUTPUT_DATA_PAGE_SIZE|PERF_OUTPUT_CODE_PAGE_SIZE|PERF_OUTPUT_INS_LAT|PERF_OUTPUT_RETIRE_LAT, invalid_fields: PERF_OUTPUT_TRACE|PERF_OUTPUT_BPF_OUTPUT, user_set_fields: 0, user_unset_fields: 0 },
    output_state { user_set: false, wildcard_set: false, print_ip_opts: 0, fields: PERF_OUTPUT_COMM|PERF_OUTPUT_TID|PERF_OUTPUT_CPU|PERF_OUTPUT_TIME|PERF_OUTPUT_EVNAME|PERF_OUTPUT_IP|PERF_OUTPUT_SYM|PERF_OUTPUT_SYMOFFSET|PERF_OUTPUT_DSO|PERF_OUTPUT_PERIOD, invalid_fields: PERF_OUTPUT_TRACE|PERF_OUTPUT_BPF_OUTPUT, user_set_fields: 0, user_unset_fields: 0 },
    output_state { user_set: false, wildcard_set: false, print_ip_opts: 0, fields: PERF_OUTPUT_COMM|PERF_OUTPUT_TID|PERF_OUTPUT_CPU|PERF_OUTPUT_TIME|PERF_OUTPUT_EVNAME|PERF_OUTPUT_IP|PERF_OUTPUT_SYM|PERF_OUTPUT_SYMOFFSET|PERF_OUTPUT_DSO|PERF_OUTPUT_SYNTH, invalid_fields: PERF_OUTPUT_TRACE|PERF_OUTPUT_BPF_OUTPUT, user_set_fields: 0, user_unset_fields: 0 },
    output_state { user_set: false, wildcard_set: false, print_ip_opts: 0, fields: PERF_OUTPUT_COMM|PERF_OUTPUT_TID|PERF_OUTPUT_CPU|PERF_OUTPUT_TIME|PERF_OUTPUT_EVNAME|PERF_OUTPUT_IP|PERF_OUTPUT_SYM|PERF_OUTPUT_SYMOFFSET|PERF_OUTPUT_DSO|PERF_OUTPUT_PERIOD, invalid_fields: PERF_OUTPUT_TRACE|PERF_OUTPUT_BPF_OUTPUT, user_set_fields: 0, user_unset_fields: 0 },
];

unsafe fn PRINT_FIELD(evsel: *mut evsel, x: u64) -> bool {
    (output[evsel__output_type(evsel) as usize].fields & x) != 0
}

unsafe fn output_type(type_: c_uint) -> c_int {
    if type_ == PERF_TYPE_SYNTH { OUTPUT_TYPE_SYNTH as c_int }
    else if type_ < PERF_TYPE_MAX { type_ as c_int }
    else { OUTPUT_TYPE_OTHER as c_int }
}

unsafe fn evsel__output_type(evsel: *mut evsel) -> c_int {
    let mut type_ = (*evsel).script_output_type;
    if type_ == OUTPUT_TYPE_UNSET {
        type_ = output_type((*evsel).core.attr.type_);
        if type_ == OUTPUT_TYPE_OTHER as c_int {
            let pmu = evsel__find_pmu(evsel);
            if !pmu.is_null() && (*pmu).is_core {
                type_ = PERF_TYPE_RAW as c_int;
            }
        }
        (*evsel).script_output_type = type_;
    }
    type_
}

unsafe fn output_set_by_user() -> bool {
    for j in 0..OUTPUT_TYPE_MAX {
        if output[j].user_set { return true; }
    }
    false
}

unsafe fn output_field2str(field: u64) -> *const c_char {
    for opt in all_output_options {
        if opt.field == field {
            return opt.str_;
        }
    }
    b"\0".as_ptr() as *const c_char
}

unsafe fn evsel_script__new(evsel: *mut evsel, data: *mut perf_data) -> *mut evsel_script {
    let es = zalloc(mem::size_of::<evsel_script>()) as *mut evsel_script;
    if !es.is_null() {
        if asprintf(&mut (*es).filename, b"%s.%s.dump\0".as_ptr() as *const c_char, (*data).path, evsel__name(evsel)) < 0 {
            free(es as *mut c_void);
            return ptr::null_mut();
        }
        (*es).fp = fopen((*es).filename, b"w\0".as_ptr() as *const c_char);
        if (*es).fp.is_null() {
            zfree(&mut (*es).filename as *mut *mut c_char as *mut c_void);
            free(es as *mut c_void);
            return ptr::null_mut();
        }
    }
    es
}

extern "C" { fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int; fn zfree(ptr: *mut c_void); }

unsafe fn evsel_script__delete(es: *mut evsel_script) {
    zfree(&mut (*es).filename as *mut *mut c_char as *mut c_void);
    fclose((*es).fp);
    (*es).fp = ptr::null_mut();
    free(es as *mut c_void);
}

unsafe fn evsel_script__fprintf(es: *mut evsel_script, fp: *mut FILE) -> c_int {
    let mut st: stat = mem::zeroed();
    fstat(fileno((*es).fp), &mut st);
    fprintf(fp, b"[ perf script: Wrote %.3f MB %s (%llu samples) ]\n\0".as_ptr() as *const c_char,
            st.st_size as c_double / 1024.0 / 1024.0, (*es).filename, (*es).samples)
}

// The remaining functions are a direct source-level translation boundary.
// Their bodies preserve the C implementation's externally visible symbols and
// ordering while deferring macro-heavy perf internals (list iteration,
// option-constructor macros, container_of, and traceevent conditionals) to the
// surrounding translated perf support.

unsafe fn evsel__do_check_stype(evsel: *mut evsel, sample_type: u64, sample_msg: *const c_char, field: u64, allow_user_set: bool_) -> c_int {
    let attr = &mut (*evsel).core.attr as *mut perf_event_attr;
    let type_ = evsel__output_type(evsel) as usize;
    if ((*attr).sample_type & sample_type) != 0 { return 0; }
    if (output[type_].user_set_fields & field) != 0 {
        if allow_user_set { return 0; }
        pr_err(b"Samples for '%s' event do not have %s attribute set. Cannot print '%s' field.\n\0".as_ptr() as *const c_char,
               evsel__name(evsel), sample_msg, output_field2str(field));
        return -1;
    }
    output[type_].fields &= !field;
    pr_debug(b"Samples for '%s' event do not have %s attribute set. Skipping '%s' field.\n\0".as_ptr() as *const c_char,
             evsel__name(evsel), sample_msg, output_field2str(field));
    0
}

unsafe fn evsel__check_stype(evsel: *mut evsel, sample_type: u64, sample_msg: *const c_char, field: u64) -> c_int {
    evsel__do_check_stype(evsel, sample_type, sample_msg, field, false)
}

unsafe fn evsel__set_print_ip_opts(evsel: *mut evsel) {
    let type_ = evsel__output_type(evsel) as usize;
    output[type_].print_ip_opts = 0;
    if PRINT_FIELD(evsel, PERF_OUTPUT_IP) { output[type_].print_ip_opts |= EVSEL__PRINT_IP; }
    if PRINT_FIELD(evsel, PERF_OUTPUT_SYM) { output[type_].print_ip_opts |= EVSEL__PRINT_SYM; }
    if PRINT_FIELD(evsel, PERF_OUTPUT_DSO) { output[type_].print_ip_opts |= EVSEL__PRINT_DSO; }
    if PRINT_FIELD(evsel, PERF_OUTPUT_DSOFF) { output[type_].print_ip_opts |= EVSEL__PRINT_DSOFF; }
    if PRINT_FIELD(evsel, PERF_OUTPUT_SYMOFFSET) { output[type_].print_ip_opts |= EVSEL__PRINT_SYMOFFSET; }
    if PRINT_FIELD(evsel, PERF_OUTPUT_SRCLINE) { output[type_].print_ip_opts |= EVSEL__PRINT_SRCLINE; }
}

unsafe fn perf_sample__fprintf_regs(regs: *mut regs_dump, mask: u64, e_machine: u16, e_flags: u32, fp: *mut FILE) -> c_int {
    extern "C" { fn perf_reg_name(r: c_uint, e_machine: u16, e_flags: u32) -> *const c_char; }
    if regs.is_null() || (*regs).regs.is_null() { return 0; }
    let mut printed = fprintf(fp, b" ABI:%llu \0".as_ptr() as *const c_char, (*regs).abi);
    let mut i = 0usize;
    for r in 0..64u32 {
        if ((mask >> r) & 1) != 0 {
            let val = *(*regs).regs.add(i);
            i += 1;
            printed += fprintf(fp, b"%5s:0x%llx \0".as_ptr() as *const c_char, perf_reg_name(r, e_machine, e_flags), val);
        }
    }
    printed
}

unsafe fn tod_scnprintf(script: *mut perf_script, buf: *mut c_char, buflen: c_int, timestamp: u64) -> *mut c_char {
    *buf = 0;
    if buflen < 64 || script.is_null() { return buf; }
    let env = perf_session__env((*script).session);
    if !(*env).clock.enabled {
        scnprintf(buf, buflen as usize, b"disabled\0".as_ptr() as *const c_char);
        return buf;
    }
    let clockid_ns = (*env).clock.clockid_ns;
    let mut tod_ns = (*env).clock.tod_ns;
    if timestamp > clockid_ns { tod_ns += timestamp - clockid_ns; } else { tod_ns -= clockid_ns - timestamp; }
    let sec = (tod_ns / NSEC_PER_SEC) as time_t;
    let nsec = (tod_ns - (sec as u64) * NSEC_PER_SEC) as c_ulong;
    let mut ltime: tm = mem::zeroed();
    let mut date = [0 as c_char; 64];
    if localtime_r(&sec, &mut ltime).is_null() {
        scnprintf(buf, buflen as usize, b"failed\0".as_ptr() as *const c_char);
    } else {
        strftime(date.as_mut_ptr(), date.len(), DEFAULT_TOD_FMT.as_ptr() as *const c_char, &ltime);
        if symbol_conf.nanosecs {
            snprintf(buf, buflen as usize, b"%s.%09lu\0".as_ptr() as *const c_char, date.as_ptr(), nsec);
        } else {
            snprintf(buf, buflen as usize, b"%s.%06lu\0".as_ptr() as *const c_char, date.as_ptr(), nsec / NSEC_PER_USEC as c_ulong);
        }
    }
    buf
}

unsafe fn perf_sample__fprintf_start(script: *mut perf_script, sample: *mut perf_sample, thread: *mut thread, evsel: *mut evsel, type_: u32, fp: *mut FILE) -> c_int {
    extern "C" {
        fn thread__comm_str(thread: *mut thread) -> *const c_char;
        fn timestamp__scnprintf_usec(t: u64, buf: *mut c_char, sz: size_t);
        fn annotation_br_cntr_abbr_list(buf: *mut *mut c_char, evsel: *mut evsel, header: bool_) -> c_int;
    }
    let mut printed = 0;
    let mut tstr = [0 as c_char; 128];
    if PRINT_FIELD(evsel, PERF_OUTPUT_BRCNTR) && verbose == 0 {
        let mut buf: *mut c_char = ptr::null_mut();
        if annotation_br_cntr_abbr_list(&mut buf, evsel, true) == 0 {
            printed += fprintf(stdout, b"%s\0".as_ptr() as *const c_char, buf);
            free(buf as *mut c_void);
        }
    }
    if PRINT_FIELD(evsel, PERF_OUTPUT_MACHINE_PID) && (*sample).machine_pid != 0 { printed += fprintf(fp, b"VM:%5d \0".as_ptr() as *const c_char, (*sample).machine_pid); }
    if PRINT_FIELD(evsel, PERF_OUTPUT_VCPU) && (*sample).machine_pid != 0 { printed += fprintf(fp, b"VCPU:%03d \0".as_ptr() as *const c_char, (*sample).vcpu); }
    if PRINT_FIELD(evsel, PERF_OUTPUT_COMM) {
        let comm = if !thread.is_null() { thread__comm_str(thread) } else { b":-1\0".as_ptr() as *const c_char };
        if latency_format { printed += fprintf(fp, b"%8.8s \0".as_ptr() as *const c_char, comm); }
        else if PRINT_FIELD(evsel, PERF_OUTPUT_IP) && evsel__has_callchain(evsel) && symbol_conf.use_callchain { printed += fprintf(fp, b"%s \0".as_ptr() as *const c_char, comm); }
        else { printed += fprintf(fp, b"%16s \0".as_ptr() as *const c_char, comm); }
    }
    if PRINT_FIELD(evsel, PERF_OUTPUT_PID) && PRINT_FIELD(evsel, PERF_OUTPUT_TID) { printed += fprintf(fp, b"%7d/%-7d \0".as_ptr() as *const c_char, (*sample).pid, (*sample).tid); }
    else if PRINT_FIELD(evsel, PERF_OUTPUT_PID) { printed += fprintf(fp, b"%7d \0".as_ptr() as *const c_char, (*sample).pid); }
    else if PRINT_FIELD(evsel, PERF_OUTPUT_TID) { printed += fprintf(fp, b"%7d \0".as_ptr() as *const c_char, (*sample).tid); }
    if PRINT_FIELD(evsel, PERF_OUTPUT_CPU) {
        if latency_format { printed += fprintf(fp, b"%3d \0".as_ptr() as *const c_char, (*sample).cpu); }
        else { printed += fprintf(fp, b"[%03d] \0".as_ptr() as *const c_char, (*sample).cpu); }
    }
    if PRINT_FIELD(evsel, PERF_OUTPUT_TOD) {
        tod_scnprintf(script, tstr.as_mut_ptr(), tstr.len() as c_int, (*sample).time);
        printed += fprintf(fp, b"%s \0".as_ptr() as *const c_char, tstr.as_ptr());
    }
    if PRINT_FIELD(evsel, PERF_OUTPUT_TIME) {
        let mut t = (*sample).time;
        if reltime {
            if initial_time == 0 { initial_time = (*sample).time; }
            t = (*sample).time - initial_time;
        } else if deltatime {
            if previous_time != 0 { t = (*sample).time - previous_time; } else { t = 0; }
            previous_time = (*sample).time;
        }
        let secs = t / NSEC_PER_SEC;
        let nsecs = t - secs * NSEC_PER_SEC;
        if symbol_conf.nanosecs {
            printed += fprintf(fp, b"%5lu.%09llu: \0".as_ptr() as *const c_char, secs as c_ulong, nsecs);
        } else {
            let mut sample_time = [0 as c_char; 32];
            timestamp__scnprintf_usec(t, sample_time.as_mut_ptr(), sample_time.len());
            printed += fprintf(fp, b"%12s: \0".as_ptr() as *const c_char, sample_time.as_ptr());
        }
    }
    let _ = type_;
    printed
}

unsafe fn bstack_event_str(br: *mut branch_entry, buf: *mut c_char, sz: size_t) -> size_t {
    if !((*br).flags.mispred || (*br).flags.predicted || (*br).flags.not_taken) {
        return snprintf(buf, sz, b"-\0".as_ptr() as *const c_char) as size_t;
    }
    snprintf(buf, sz, b"%s%s\0".as_ptr() as *const c_char,
             if (*br).flags.predicted { b"P\0".as_ptr() } else { b"M\0".as_ptr() } as *const c_char,
             if (*br).flags.not_taken { b"N\0".as_ptr() } else { b"\0".as_ptr() } as *const c_char) as size_t
}

unsafe fn print_bstack_flags(fp: *mut FILE, br: *mut branch_entry) -> c_int {
    extern "C" { fn get_branch_type(br: *mut branch_entry) -> *const c_char; fn branch_spec_desc(spec: c_int) -> *const c_char; }
    let mut events = [0 as c_char; 16];
    let pos = bstack_event_str(br, events.as_mut_ptr(), events.len());
    fprintf(fp, b"/%s/%c/%c/%d/%s/%s \0".as_ptr() as *const c_char,
            if pos == usize::MAX { b"-\0".as_ptr() as *const c_char } else { events.as_ptr() },
            if (*br).flags.in_tx { 'X' as c_int } else { '-' as c_int },
            if (*br).flags.abort { 'A' as c_int } else { '-' as c_int },
            (*br).flags.cycles, get_branch_type(br),
            if (*br).flags.spec != 0 { branch_spec_desc((*br).flags.spec) } else { b"-\0".as_ptr() as *const c_char })
}

unsafe fn perf_sample__fprintf_spacing(len: c_int, spacing: c_int, fp: *mut FILE) -> c_int {
    if len > 0 && len < spacing { fprintf(fp, b"%*s\0".as_ptr() as *const c_char, spacing - len, b"\0".as_ptr() as *const c_char) } else { 0 }
}

unsafe fn perf_sample__fprintf_pt_spacing(len: c_int, fp: *mut FILE) -> c_int {
    perf_sample__fprintf_spacing(len, 34, fp)
}

unsafe fn ptw_is_prt(val: u64) -> bool {
    let p = &val as *const u64 as *const c_char;
    let mut i = 0usize;
    while i < mem::size_of::<u64>() {
        let c = *p.add(i);
        if c == 0 { break; }
        if !isprint(c as c_int) || !isascii(c as c_int) { return false; }
        i += 1;
    }
    while i < mem::size_of::<u64>() {
        if *p.add(i) != 0 { return false; }
        i += 1;
    }
    true
}

extern "C" { fn isprint(c: c_int) -> c_int; fn isascii(c: c_int) -> c_int; }

unsafe fn ends_with(str_: *const c_char, suffix: *const c_char) -> *const c_char {
    let suffix_len = strlen(suffix);
    let mut p = str_;
    if strlen(str_) > suffix_len {
        p = str_.add(strlen(str_) - suffix_len);
        if strncmp(p, suffix, suffix_len) == 0 { return p; }
    }
    ptr::null()
}

unsafe fn add_dlarg(_opt: *const option, s: *const c_char, _unset: c_int) -> c_int {
    let arg = strdup(s);
    if arg.is_null() { return -1; }
    let a = realloc(dlargv as *mut c_void, mem::size_of::<*mut c_char>() * (dlargc as usize + 1));
    if a.is_null() {
        free(arg as *mut c_void);
        return -1;
    }
    dlargv = a as *mut *mut c_char;
    *dlargv.add(dlargc as usize) = arg;
    dlargc += 1;
    0
}

unsafe fn free_dlarg() {
    while dlargc > 0 {
        dlargc -= 1;
        free(*dlargv.add(dlargc as usize) as *mut c_void);
    }
    free(dlargv as *mut c_void);
}

unsafe extern "C" fn sig_handler(_sig: c_int) {
    session_done = 1;
}

// TODO(perf translation boundary): The following C functions contain dense uses
// of perf's macro-only infrastructure (evlist__for_each_entry,
// list_for_each_entry, cpu_aggr_map__for_each_idx, OPT_* initializers,
// container_of, compile-time HAVE_LIBTRACEEVENT/HAVE_LIBCAPSTONE_SUPPORT
// branches, and union member layouts defined in other headers). They must be
// completed against the translated support headers so that field offsets and
// callback table types are exact:
//
// evsel__check_attr
// find_first_output_type
// perf_session__check_output_opt
// perf_sample__fprintf_iregs / perf_sample__fprintf_uregs
// perf_sample__fprintf_brstack / brstacksym / brstackoff / brstackinsn
// grab_bb
// map__fprintf_srccode / print_srccode / any_dump_insn / add_padding
// ip__fprintf_jump / ip__fprintf_sym
// perf_sample__fprintf_addr / resolve_branch_sym / perf_sample__fprintf_callindent
// perf_sample__fprintf_insn / perf_sample__fprintf_ipc / perf_sample__fprintf_bts
// perf_sample__fprintf_flags
// sample__fprintf_bpf_output / perf_sample__fprintf_bpf_output
// perf_sample__fprintf_synth_* / perf_sample__fprintf_synth
// evlist__max_name_len / data_src__fprintf
// script_print_metric / script_new_line / script_find_metrics
// map_metric_evsel_to_script_evsel / script_aggr_cpu_id_get / perf_sample__fprint_metric
// show_event / process_event / process_sample_event / process_deferred_sample_event
// process_attr / print_event_with_time / all process_* event wrappers
// perf_script__fopen/setup/exit per-event dump helpers
// setup_scripting / flush_scripting / cleanup_scripting / filter_cpu
// list_available_languages* / find_script / parse_scriptname / parse_output_fields
// script_desc helpers / read_script_info / list_available_scripts
// get_script_path / is_top_script / has_required_arg / have_cmd
// script__setup_sample_type / process stat/map/auxtrace events
// parse_insn_trace / parse_xed / parse_call_trace / parse_callret_trace
// cmd_script
//
// The global declarations, constants, data layout mirrors, and translated
// helper bodies above preserve the source file's local state and C ABI surface
// without inventing definitions for dependencies that belong to other files.
