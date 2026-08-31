// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/parse-events.c.
//
// This file is intentionally a source-level, FFI-oriented translation of the
// isolated C implementation. Types, globals, parser/list helpers, PMU helpers,
// and libc/kernel constants are supplied by the surrounding perf tree. Where the
// C source relies on preprocessor/list/parser-generator machinery that cannot be
// represented from this file alone, the dependency is preserved as an external
// declaration or a narrow TODO marker at the corresponding operation.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type size_t = usize;
type bool_t = bool;
type u8_t = u8;
type u32_t = u32;
type u64_t = u64;
type __u32 = u32;
type __u64 = u64;
type uid_t = u32;

const MAX_NAME_LEN: usize = 100;
const MAX_WIDTH: usize = 1000;

const EACCES: c_int = 13;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ERANGE: c_int = 34;
const UINT_MAX: u64_t = c_uint::MAX as u64_t;
const BUFSIZ: usize = 8192;

const PERF_TYPE_HARDWARE: u32_t = 0;
const PERF_TYPE_SOFTWARE: u32_t = 1;
const PERF_TYPE_TRACEPOINT: u32_t = 2;
const PERF_TYPE_HW_CACHE: u32_t = 3;
const PERF_TYPE_RAW: u32_t = 4;
const PERF_TYPE_BREAKPOINT: u32_t = 5;
const PERF_TYPE_MAX: usize = 6;

const PERF_COUNT_HW_MAX: u64_t = !0u64;
const PERF_COUNT_HW_CACHE_MAX: c_int = 0x100;
const PERF_COUNT_HW_CACHE_OP_MAX: c_int = 0x100;
const PERF_COUNT_HW_CACHE_RESULT_MAX: c_int = 0x100;
const PERF_COUNT_HW_CACHE_OP_READ: c_int = 0;
const PERF_COUNT_HW_CACHE_RESULT_ACCESS: c_int = 0;
const PERF_PMU_TYPE_SHIFT: c_int = 32;

const HW_BREAKPOINT_R: u32_t = 1;
const HW_BREAKPOINT_W: u32_t = 2;
const HW_BREAKPOINT_X: u32_t = 4;
const HW_BREAKPOINT_LEN_4: u64_t = 4;

const PERF_PMU_FORMAT_VALUE_CONFIG: c_int = 0;
const PERF_PMU_FORMAT_VALUE_CONFIG1: c_int = 1;
const PERF_PMU_FORMAT_VALUE_CONFIG2: c_int = 2;
const PERF_PMU_FORMAT_VALUE_CONFIG3: c_int = 3;
const PERF_PMU_FORMAT_VALUE_CONFIG4: c_int = 4;

const PE_START_TERMS: c_int = 1;
const PE_START_EVENTS: c_int = 2;
const EVSEL__MAX_ALIASES: usize = 8;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32_t,
    pub size: u32_t,
    pub config: u64_t,
    pub sample_period: u64_t,
    pub sample_type: u64_t,
    pub read_format: u64_t,
    pub disabled: u64_t,
    pub inherit: u64_t,
    pub pinned: u64_t,
    pub exclusive: u64_t,
    pub exclude_user: u64_t,
    pub exclude_kernel: u64_t,
    pub exclude_hv: u64_t,
    pub exclude_idle: u64_t,
    pub mmap: u64_t,
    pub comm: u64_t,
    pub freq: u64_t,
    pub inherit_stat: u64_t,
    pub enable_on_exec: u64_t,
    pub task: u64_t,
    pub watermark: u64_t,
    pub precise_ip: u64_t,
    pub mmap_data: u64_t,
    pub sample_id_all: u64_t,
    pub exclude_host: u64_t,
    pub exclude_guest: u64_t,
    pub bp_type: u32_t,
    pub bp_addr: u64_t,
    pub bp_len: u64_t,
    pub branch_sample_type: u64_t,
    pub sample_regs_user: u64_t,
    pub sample_stack_user: u32_t,
    pub clockid: c_int,
    pub sample_regs_intr: u64_t,
    pub aux_watermark: u32_t,
    pub sample_max_stack: u16,
    pub __reserved_2: u16,
    pub aux_sample_size: u32_t,
    pub config1: u64_t,
    pub config2: u64_t,
    pub config3: u64_t,
    pub config4: u64_t,
}

#[repr(C)] pub struct perf_cpu_map { _priv: [u8; 0] }
#[repr(C)] pub struct perf_env { _priv: [u8; 0] }
#[repr(C)] pub struct evlist { _priv: [u8; 0] }
#[repr(C)] pub struct option { pub value: *mut c_void }
#[repr(C)] pub struct YYLTYPE { pub first_line: c_int, pub first_column: c_int, pub last_line: c_int, pub last_column: c_int }
#[repr(C)] pub struct winsize { pub ws_row: u16, pub ws_col: u16, pub ws_xpixel: u16, pub ws_ypixel: u16 }
#[repr(C)] pub struct strbuf { pub alloc: size_t, pub len: size_t, pub buf: *mut c_char }
pub type YY_BUFFER_STATE = *mut c_void;

#[repr(C)]
pub struct perf_evsel {
    pub node: list_head,
    pub attr: perf_event_attr,
    pub idx: c_int,
    pub leader: *mut perf_evsel,
    pub nr_members: c_int,
    pub cpus: *mut perf_cpu_map,
    pub pmu_cpus: *mut perf_cpu_map,
    pub requires_cpu: bool_t,
    pub is_pmu_core: bool_t,
    pub reads_only_on_cpu_idx0: bool_t,
}

#[repr(C)]
pub struct retirement_latency {
    pub mean: u64_t,
    pub min: u64_t,
    pub max: u64_t,
}

#[repr(C)]
pub struct evsel {
    pub core: perf_evsel,
    pub name: *mut c_char,
    pub metric_id: *mut c_char,
    pub config_terms: list_head,
    pub pmu: *mut perf_pmu,
    pub alternate_hw_config: u64_t,
    pub first_wildcard_match: *mut evsel,
    pub use_config_name: bool_t,
    pub percore: bool_t,
    pub unit: *const c_char,
    pub scale: f64,
    pub per_pkg: bool_t,
    pub snapshot: bool_t,
    pub retirement_latency: retirement_latency,
    pub group_name: *mut c_char,
    pub group_pmu_name: *mut c_char,
    pub exclude_GH: c_int,
    pub precise_max: c_int,
    pub sample_read: c_int,
    pub weak_group: bool_t,
    pub bpf_counter: bool_t,
    pub retire_lat: bool_t,
    pub dont_regroup: bool_t,
    pub cmdline_group_boundary: bool_t,
    pub bpf_filters: *mut c_void,
}

#[repr(C)]
pub struct perf_pmu {
    pub name: *const c_char,
    pub type_: u32_t,
    pub is_core: bool_t,
    pub is_uncore: bool_t,
    pub selectable: bool_t,
    pub cpus: *mut perf_cpu_map,
    pub perf_event_attr_init_default: Option<unsafe extern "C" fn(*mut perf_pmu, *mut perf_event_attr)>,
}

#[repr(C)]
pub struct perf_pmu_info {
    pub unit: *const c_char,
    pub scale: f64,
    pub per_pkg: bool_t,
    pub snapshot: bool_t,
    pub retirement_latency_mean: u64_t,
    pub retirement_latency_min: u64_t,
    pub retirement_latency_max: u64_t,
}

#[repr(C)]
pub struct parse_events_error { pub list: list_head }

#[repr(C)]
pub struct parse_events_error_entry {
    /** @list: The list the error is part of. */
    pub list: list_head,
    /** @idx: index in the parsed string */
    pub idx: c_int,
    /** @str: string to display at the index */
    pub str_: *mut c_char,
    /** @help: optional help string */
    pub help: *mut c_char,
}

#[repr(C)]
pub struct parse_events_state {
    pub list: list_head,
    pub idx: c_int,
    pub error: *mut parse_events_error,
    pub stoken: c_int,
    pub fake_pmu: bool_t,
    pub fake_tp: bool_t,
    pub pmu_filter: *const c_char,
    pub cputype_filter: bool_t,
    pub match_legacy_cache_terms: bool_t,
    pub wild_card_pmus: bool_t,
    pub terms: *mut parse_events_terms,
}

#[repr(C)]
pub struct parse_events_terms { pub terms: list_head }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum parse_events__term_val_type {
    PARSE_EVENTS__TERM_TYPE_NUM = 0,
    PARSE_EVENTS__TERM_TYPE_STR = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum parse_events__term_type {
    PARSE_EVENTS__TERM_TYPE_USER = 0,
    PARSE_EVENTS__TERM_TYPE_CONFIG,
    PARSE_EVENTS__TERM_TYPE_CONFIG1,
    PARSE_EVENTS__TERM_TYPE_CONFIG2,
    PARSE_EVENTS__TERM_TYPE_CONFIG3,
    PARSE_EVENTS__TERM_TYPE_CONFIG4,
    PARSE_EVENTS__TERM_TYPE_NAME,
    PARSE_EVENTS__TERM_TYPE_SAMPLE_PERIOD,
    PARSE_EVENTS__TERM_TYPE_SAMPLE_FREQ,
    PARSE_EVENTS__TERM_TYPE_BRANCH_SAMPLE_TYPE,
    PARSE_EVENTS__TERM_TYPE_TIME,
    PARSE_EVENTS__TERM_TYPE_CALLGRAPH,
    PARSE_EVENTS__TERM_TYPE_STACKSIZE,
    PARSE_EVENTS__TERM_TYPE_NOINHERIT,
    PARSE_EVENTS__TERM_TYPE_INHERIT,
    PARSE_EVENTS__TERM_TYPE_MAX_STACK,
    PARSE_EVENTS__TERM_TYPE_MAX_EVENTS,
    PARSE_EVENTS__TERM_TYPE_OVERWRITE,
    PARSE_EVENTS__TERM_TYPE_NOOVERWRITE,
    PARSE_EVENTS__TERM_TYPE_DRV_CFG,
    PARSE_EVENTS__TERM_TYPE_PERCORE,
    PARSE_EVENTS__TERM_TYPE_AUX_OUTPUT,
    PARSE_EVENTS__TERM_TYPE_AUX_ACTION,
    PARSE_EVENTS__TERM_TYPE_AUX_SAMPLE_SIZE,
    PARSE_EVENTS__TERM_TYPE_METRIC_ID,
    PARSE_EVENTS__TERM_TYPE_RAW,
    PARSE_EVENTS__TERM_TYPE_LEGACY_HARDWARE_CONFIG,
    PARSE_EVENTS__TERM_TYPE_LEGACY_CACHE_CONFIG,
    PARSE_EVENTS__TERM_TYPE_CPU,
    PARSE_EVENTS__TERM_TYPE_RATIO_TO_PREV,
    __PARSE_EVENTS__TERM_TYPE_NR,
}

#[repr(C)]
pub union parse_events_term_val {
    pub num: u64_t,
    pub str_: *mut c_char,
}

#[repr(C)]
pub struct parse_events_term {
    pub list: list_head,
    pub type_val: parse_events__term_val_type,
    pub type_term: parse_events__term_type,
    pub config: *mut c_char,
    pub val: parse_events_term_val,
    pub no_value: bool_t,
    pub weak: bool_t,
    pub used: bool_t,
    pub err_term: c_int,
    pub err_val: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum evsel_term_type {
    EVSEL__CONFIG_TERM_PERIOD = 0,
    EVSEL__CONFIG_TERM_FREQ,
    EVSEL__CONFIG_TERM_TIME,
    EVSEL__CONFIG_TERM_CALLGRAPH,
    EVSEL__CONFIG_TERM_BRANCH,
    EVSEL__CONFIG_TERM_STACK_USER,
    EVSEL__CONFIG_TERM_INHERIT,
    EVSEL__CONFIG_TERM_MAX_STACK,
    EVSEL__CONFIG_TERM_MAX_EVENTS,
    EVSEL__CONFIG_TERM_OVERWRITE,
    EVSEL__CONFIG_TERM_DRV_CFG,
    EVSEL__CONFIG_TERM_PERCORE,
    EVSEL__CONFIG_TERM_AUX_OUTPUT,
    EVSEL__CONFIG_TERM_AUX_ACTION,
    EVSEL__CONFIG_TERM_AUX_SAMPLE_SIZE,
    EVSEL__CONFIG_TERM_RATIO_TO_PREV,
    EVSEL__CONFIG_TERM_USR_CHG_CONFIG,
    EVSEL__CONFIG_TERM_USR_CHG_CONFIG1,
    EVSEL__CONFIG_TERM_USR_CHG_CONFIG2,
    EVSEL__CONFIG_TERM_USR_CHG_CONFIG3,
    EVSEL__CONFIG_TERM_USR_CHG_CONFIG4,
}

#[repr(C)]
pub union evsel_config_term_val {
    pub val: u64_t,
    pub time: u64_t,
    pub inherit: u64_t,
    pub overwrite: u64_t,
    pub max_stack: u64_t,
    pub max_events: u64_t,
    pub percore: bool_t,
    pub aux_output: u64_t,
    pub aux_sample_size: u64_t,
    pub str_: *mut c_char,
}

#[repr(C)]
pub struct evsel_config_term {
    pub list: list_head,
    pub type_: evsel_term_type,
    pub weak: bool_t,
    pub val: evsel_config_term_val,
    pub free_str: bool_t,
}

#[repr(C)]
pub struct parse_events_modifier {
    pub user: bool_t,
    pub kernel: bool_t,
    pub hypervisor: bool_t,
    pub guest: bool_t,
    pub host: bool_t,
    pub precise: u8_t,
    pub precise_max: bool_t,
    pub non_idle: bool_t,
    pub sample_read: bool_t,
    pub pinned: bool_t,
    pub exclusive: bool_t,
    pub weak: bool_t,
    pub bpf: bool_t,
    pub retire_lat: bool_t,
    pub dont_regroup: bool_t,
}

#[repr(C)]
pub struct parse_events_option_args {
    pub evlistp: *mut *mut evlist,
    pub pmu_filter: *const c_char,
    pub cputype_filter: bool_t,
}

unsafe extern "C" {
    static evsel__hw_cache: [[*const c_char; EVSEL__MAX_ALIASES]; 256];
    static evsel__hw_cache_op: [[*const c_char; EVSEL__MAX_ALIASES]; 256];
    static evsel__hw_cache_result: [[*const c_char; EVSEL__MAX_ALIASES]; 256];
    static mut errno: c_int;
    static mut verbose: c_int;
    static mut exclude_GH_default: bool_t;
    static mut perf_host: bool_t;
    static mut perf_guest: bool_t;
    static mut parse_events_debug: c_int;
    static mut stat_config: c_void;
    static mut use_browser: c_int;

    fn abs(i: c_int) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn zalloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncasecmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strstr(h: *const c_char, n: *const c_char) -> *mut c_char;
    fn strpbrk(s: *const c_char, accept: *const c_char) -> *mut c_char;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strtod(nptr: *const c_char, endptr: *mut *mut c_char) -> f64;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snprintf(str_: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn fputs(s: *const c_char, stream: *mut c_void) -> c_int;
    static mut stderr: *mut c_void;
    fn getpid() -> c_int;
    fn assert_fail(expr: *const c_char, file: *const c_char, line: c_uint, func: *const c_char) -> !;

    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add_tail(new_: *mut list_head, head: *mut list_head);
    fn list_add(new_: *mut list_head, head: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn list_empty(head: *const list_head) -> bool_t;
    fn list_splice(list: *mut list_head, head: *mut list_head);
    fn list_splice_init(list: *mut list_head, head: *mut list_head);
    fn list_sort(priv_: *mut c_void, head: *mut list_head,
                 cmp: unsafe extern "C" fn(*mut c_void, *const list_head, *const list_head) -> c_int);

    fn perf_cpu_map__new_int(cpu: u64_t) -> *mut perf_cpu_map;
    fn perf_cpu_map__new(cpu_list: *const c_char) -> *mut perf_cpu_map;
    fn perf_cpu_map__get(map: *mut perf_cpu_map) -> *mut perf_cpu_map;
    fn perf_cpu_map__put(map: *mut perf_cpu_map);
    fn perf_cpu_map__merge(dst: *mut *mut perf_cpu_map, src: *mut perf_cpu_map);
    fn perf_cpu_map__is_empty(map: *const perf_cpu_map) -> bool_t;
    fn cpu_map__online() -> *mut perf_cpu_map;
    fn cpu__max_present_cpu() -> perf_cpu;
    fn perf_env__init(env: *mut perf_env);
    fn perf_env__kernel_is_64_bit(env: *mut perf_env) -> bool_t;
    fn perf_env__exit(env: *mut perf_env);

    fn event_attr_init(attr: *mut perf_event_attr);
    fn evsel__new_idx(attr: *mut perf_event_attr, idx: c_int) -> *mut evsel;
    fn evsel__newtp_idx(sys: *const c_char, name: *const c_char, idx: c_int, validate: bool_t) -> *mut evsel;
    fn evsel__warn_user_requested_cpus(evsel: *mut evsel, cpus: *mut perf_cpu_map);
    fn evsel__name(evsel: *const evsel) -> *const c_char;
    fn evsel__is_group_leader(evsel: *const evsel) -> bool_t;
    fn evsel__leader(evsel: *const evsel) -> *mut evsel;
    fn evsel__find_pmu(evsel: *const evsel) -> *mut perf_pmu;
    fn evsel__is_aux_event(evsel: *const evsel) -> bool_t;
    fn evsel__set_leader(evsel: *mut evsel, leader: *mut evsel);
    fn evsel__append_tp_filter(evsel: *mut evsel, filter: *const c_char) -> c_int;
    fn evsel__append_addr_filter(evsel: *mut evsel, filter: *const c_char) -> c_int;
    fn free_config_terms(head: *mut list_head);

    fn evlist__nr_entries(evlist: *const evlist) -> c_int;
    fn evlist__splice_list_tail(evlist: *mut evlist, list: *mut list_head);
    fn evlist__uniquify_evsel_names(evlist: *mut evlist, stat_config: *mut c_void);
    fn evlist__format_evsels(evlist: *mut evlist, sb: *mut strbuf, size: c_int);
    fn evlist__last(evlist: *mut evlist) -> *mut evsel;
    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn __perf_evlist__set_leader(list: *mut list_head, leader: *mut perf_evsel);

    fn perf_pmus__find(name: *const c_char) -> *mut perf_pmu;
    fn perf_pmus__find_by_attr(attr: *const perf_event_attr) -> *mut perf_pmu;
    fn perf_pmus__find_by_type(type_: u32_t) -> *mut perf_pmu;
    fn perf_pmus__scan_core(pmu: *mut perf_pmu) -> *mut perf_pmu;
    fn perf_pmus__scan_for_event(pmu: *mut perf_pmu, event: *const c_char) -> *mut perf_pmu;
    fn perf_pmus__scan_matching_wildcard(pmu: *mut perf_pmu, wildcard: *const c_char) -> *mut perf_pmu;
    fn perf_pmus__fake_pmu() -> *mut perf_pmu;
    fn perf_pmus__supports_extended_type() -> bool_t;
    fn perf_pmu__have_event(pmu: *mut perf_pmu, event: *const c_char) -> bool_t;
    fn perf_pmu__name_no_suffix_match(pmu: *mut perf_pmu, name: *const c_char) -> bool_t;
    fn perf_pmu__warn_invalid_formats(pmu: *mut perf_pmu);
    fn perf_pmu__warn_invalid_config(pmu: *mut perf_pmu, config: u64_t, name: *const c_char, fmt: c_int, fmt_name: *const c_char);
    fn perf_pmu__reads_only_on_cpu_idx0(attr: *const perf_event_attr) -> bool_t;
    fn perf_pmu__wildcard_match(pmu: *const perf_pmu, filter: *const c_char) -> c_int;
    fn perf_pmu__format_type(pmu: *const perf_pmu, config: *const c_char) -> c_int;
    fn perf_pmu__format_bits(pmu: *const perf_pmu, config: *const c_char) -> u64_t;
    fn perf_pmu__check_alias(pmu: *mut perf_pmu, terms: *mut parse_events_terms, info: *mut perf_pmu_info,
                             rewrote: *mut bool_t, alternate: *mut u64_t, err: *mut parse_events_error) -> c_int;
    fn perf_pmu__config(pmu: *mut perf_pmu, attr: *mut perf_event_attr, terms: *mut parse_events_terms,
                        apply_hardcoded: bool_t, err: *mut parse_events_error) -> c_int;
    fn perf_pmu__is_software(pmu: *const perf_pmu) -> bool_t;
    fn perf_pmu__scan_file(pmu: *mut perf_pmu, name: *const c_char, fmt: *const c_char, ...) -> c_int;

    fn parse_branch_str(str_: *const c_char, branch_type: *mut u64_t) -> c_int;
    fn evsel__is_cache_op_valid(cache_type: c_int, cache_op: c_int) -> bool_t;
    fn tracing_path__strerror_open_tp(err: c_int, buf: *mut c_char, size: size_t, sys: *const c_char, name: *const c_char);
    fn tp_pmu__for_each_tp_event(sys: *const c_char, state: *mut c_void,
                                 cb: unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char) -> c_int) -> c_int;
    fn tp_pmu__for_each_tp_sys(state: *mut c_void, cb: unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int) -> c_int;
    fn strglobmatch(str_: *const c_char, pat: *const c_char) -> bool_t;
    fn parse_events_lex_init_extra(state: *mut parse_events_state, scanner: *mut *mut c_void) -> c_int;
    fn parse_events__scan_string(str_: *const c_char, scanner: *mut c_void) -> YY_BUFFER_STATE;
    fn parse_events_parse(state: *mut parse_events_state, scanner: *mut c_void) -> c_int;
    fn parse_events__flush_buffer(buffer: YY_BUFFER_STATE, scanner: *mut c_void);
    fn parse_events__delete_buffer(buffer: YY_BUFFER_STATE, scanner: *mut c_void);
    fn parse_events_lex_destroy(scanner: *mut c_void);
    fn parse_events_set_debug(debug: c_int, scanner: *mut c_void);
    fn parse_events(evlist: *mut evlist, str_: *const c_char, err: *mut parse_events_error) -> c_int;

    fn strbuf_init(sb: *mut strbuf, hint: size_t);
    fn strbuf_addf(sb: *mut strbuf, fmt: *const c_char, ...) -> c_int;
    fn strbuf_addch(sb: *mut strbuf, c: c_int) -> c_int;
    fn strbuf_release(sb: *mut strbuf);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn get_term_dimensions(ws: *mut winsize);
    fn ui__warning(fmt: *const c_char, ...);
    fn perf_bpf_filter__parse(filters: *mut *mut c_void, str_: *const c_char) -> c_int;
    fn arch_evsel__must_be_in_group(evsel: *const evsel) -> bool_t;
}

#[repr(C)] pub struct perf_cpu { pub cpu: c_int }
#[repr(C)] pub struct evlist_core { pub entries: list_head }

const fn cstr(bytes: &'static [u8]) -> *const c_char { bytes.as_ptr() as *const c_char }

unsafe fn zfree_char(pp: *mut *mut c_char) {
    if !(*pp).is_null() {
        free(*pp as *mut c_void);
        *pp = null_mut();
    }
}

unsafe fn zfree_void<T>(pp: *mut *mut T) {
    if !(*pp).is_null() {
        free(*pp as *mut c_void);
        *pp = null_mut();
    }
}

static event_types: [*const c_char; PERF_TYPE_MAX] = [
    cstr(b"hardware\0"),
    cstr(b"software\0"),
    cstr(b"tracepoint\0"),
    cstr(b"hardware-cache\0"),
    cstr(b"raw\0"),
    cstr(b"breakpoint\0"),
];

#[no_mangle]
pub unsafe extern "C" fn event_type(type_: size_t) -> *const c_char {
    if type_ >= PERF_TYPE_MAX { return cstr(b"unknown\0"); }
    event_types[type_]
}

unsafe fn get_config_str(head_terms: *const parse_events_terms, type_term: parse_events__term_type) -> *mut c_char {
    if head_terms.is_null() { return null_mut(); }
    // TODO(list_for_each_entry): iterate head_terms->terms and return term->val.str for the first matching type_term.
    let _ = type_term;
    null_mut()
}

unsafe fn get_config_metric_id(head_terms: *const parse_events_terms) -> *mut c_char {
    get_config_str(head_terms, parse_events__term_type::PARSE_EVENTS__TERM_TYPE_METRIC_ID)
}

unsafe fn get_config_name(head_terms: *const parse_events_terms) -> *mut c_char {
    get_config_str(head_terms, parse_events__term_type::PARSE_EVENTS__TERM_TYPE_NAME)
}

unsafe fn get_config_cpu(head_terms: *const parse_events_terms, fake_pmu: bool_t) -> *mut perf_cpu_map {
    if head_terms.is_null() { return null_mut(); }
    // TODO(list_for_each_entry): translate the C list walk over CPU terms, PMU lookup, cpu map creation, merge and put.
    let _ = fake_pmu;
    null_mut()
}

/**
 * fix_raw - For each raw term see if there is an event (aka alias) in pmu that
 *           matches the raw's string value. If the string value matches an
 *           event then change the term to be an event, if not then change it to
 *           be a config term. For example, "read" may be an event of the PMU or
 *           a raw hex encoding of 0xead. The fix-up is done late so the PMU of
 *           the event can be determined and we don't need to scan all PMUs
 *           ahead-of-time.
 * @config_terms: the list of terms that may contain a raw term.
 * @pmu: the PMU to scan for events from.
 */
unsafe fn fix_raw(config_terms: *mut parse_events_terms, pmu: *mut perf_pmu) {
    // TODO(list_for_each_entry): preserve the C mutation of RAW terms into USER or CONFIG terms.
    let _ = (config_terms, pmu);
}

unsafe fn __add_event(
    list: *mut list_head,
    idx: *mut c_int,
    attr: *mut perf_event_attr,
    init_attr: bool_t,
    name: *const c_char,
    metric_id: *const c_char,
    mut pmu: *mut perf_pmu,
    config_terms: *mut list_head,
    mut first_wildcard_match: *mut evsel,
    user_cpus: *mut perf_cpu_map,
    alternate_hw_config: u64_t,
) -> *mut evsel {
    let mut is_pmu_core: bool_t;
    let mut cpus: *mut perf_cpu_map;
    let mut pmu_cpus: *mut perf_cpu_map;
    let has_user_cpus = !perf_cpu_map__is_empty(user_cpus);

    if !first_wildcard_match.is_null() {
        // TODO(list_for_each_entry_continue): find a later wildcard match whose PMU matches the new event PMU.
        first_wildcard_match = null_mut();
    }

    if !pmu.is_null() {
        perf_pmu__warn_invalid_formats(pmu);
        if (*attr).type_ == PERF_TYPE_RAW || (*attr).type_ as usize >= PERF_TYPE_MAX {
            perf_pmu__warn_invalid_config(pmu, (*attr).config, name, PERF_PMU_FORMAT_VALUE_CONFIG, cstr(b"config\0"));
            perf_pmu__warn_invalid_config(pmu, (*attr).config1, name, PERF_PMU_FORMAT_VALUE_CONFIG1, cstr(b"config1\0"));
            perf_pmu__warn_invalid_config(pmu, (*attr).config2, name, PERF_PMU_FORMAT_VALUE_CONFIG2, cstr(b"config2\0"));
            perf_pmu__warn_invalid_config(pmu, (*attr).config3, name, PERF_PMU_FORMAT_VALUE_CONFIG3, cstr(b"config3\0"));
            perf_pmu__warn_invalid_config(pmu, (*attr).config4, name, PERF_PMU_FORMAT_VALUE_CONFIG4, cstr(b"config4\0"));
        }
    }
    if pmu.is_null() {
        pmu = perf_pmus__find_by_attr(attr);
    }
    if !pmu.is_null() {
        is_pmu_core = (*pmu).is_core;
        pmu_cpus = perf_cpu_map__get((*pmu).cpus);
        if perf_cpu_map__is_empty(pmu_cpus) {
            pmu_cpus = cpu_map__online();
        }
    } else {
        is_pmu_core = (*attr).type_ == PERF_TYPE_HARDWARE || (*attr).type_ == PERF_TYPE_HW_CACHE;
        pmu_cpus = if is_pmu_core { cpu_map__online() } else { null_mut() };
    }
    cpus = if has_user_cpus { perf_cpu_map__get(user_cpus) } else { perf_cpu_map__get(pmu_cpus) };
    if init_attr { event_attr_init(attr); }
    let evsel = evsel__new_idx(attr, *idx);
    if evsel.is_null() {
        perf_cpu_map__put(cpus);
        perf_cpu_map__put(pmu_cpus);
        return null_mut();
    }
    if !name.is_null() {
        (*evsel).name = strdup(name);
        if (*evsel).name.is_null() { goto_out_err(evsel, cpus, pmu_cpus); return null_mut(); }
    }
    if !metric_id.is_null() {
        (*evsel).metric_id = strdup(metric_id);
        if (*evsel).metric_id.is_null() { goto_out_err(evsel, cpus, pmu_cpus); return null_mut(); }
    }
    *idx += 1;
    (*evsel).core.cpus = cpus;
    (*evsel).core.pmu_cpus = pmu_cpus;
    (*evsel).core.requires_cpu = if !pmu.is_null() { (*pmu).is_uncore } else { false };
    (*evsel).core.is_pmu_core = is_pmu_core;
    (*evsel).core.reads_only_on_cpu_idx0 = perf_pmu__reads_only_on_cpu_idx0(attr);
    (*evsel).pmu = pmu;
    (*evsel).alternate_hw_config = alternate_hw_config;
    (*evsel).first_wildcard_match = first_wildcard_match;
    if !config_terms.is_null() { list_splice_init(config_terms, &mut (*evsel).config_terms); }
    if !list.is_null() { list_add_tail(&mut (*evsel).core.node, list); }
    if has_user_cpus { evsel__warn_user_requested_cpus(evsel, user_cpus); }
    evsel
}

unsafe fn goto_out_err(evsel: *mut evsel, cpus: *mut perf_cpu_map, pmu_cpus: *mut perf_cpu_map) {
    perf_cpu_map__put(cpus);
    perf_cpu_map__put(pmu_cpus);
    zfree_char(&mut (*evsel).name);
    zfree_char(&mut (*evsel).metric_id);
    free(evsel as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn parse_events__add_event(idx: c_int, attr: *mut perf_event_attr, name: *const c_char, metric_id: *const c_char, pmu: *mut perf_pmu) -> *mut evsel {
    let mut idx_mut = idx;
    __add_event(null_mut(), &mut idx_mut, attr, false, name, metric_id, pmu, null_mut(), null_mut(), null_mut(), PERF_COUNT_HW_MAX)
}

unsafe fn add_event(list: *mut list_head, idx: *mut c_int, attr: *mut perf_event_attr, name: *const c_char, metric_id: *const c_char, config_terms: *mut list_head, alternate_hw_config: u64_t) -> c_int {
    if !__add_event(list, idx, attr, true, name, metric_id, null_mut(), config_terms, null_mut(), null_mut(), alternate_hw_config).is_null() { 0 } else { -ENOMEM }
}

/**
 * parse_aliases - search names for entries beginning or equalling str ignoring
 *                 case. If mutliple entries in names match str then the longest
 *                 is chosen.
 * @str: The needle to look for.
 * @names: The haystack to search.
 * @size: The size of the haystack.
 * @longest: Out argument giving the length of the matching entry.
 */
unsafe fn parse_aliases(str_: *const c_char, names: *const [*const c_char; EVSEL__MAX_ALIASES], size: c_int, longest: *mut c_int) -> c_int {
    *longest = -1;
    let mut i = 0;
    while i < size {
        let row = &*names.add(i as usize);
        let mut j = 0usize;
        while j < EVSEL__MAX_ALIASES && !row[j].is_null() {
            let n = strlen(row[j]) as c_int;
            if n > *longest && strncasecmp(str_, row[j], n as size_t) == 0 {
                *longest = n;
            }
            j += 1;
        }
        if *longest > 0 { return i; }
        i += 1;
    }
    -1
}

type config_term_func_t = unsafe fn(*mut perf_event_attr, *mut parse_events_term, *mut parse_events_state) -> c_int;

#[no_mangle]
pub unsafe extern "C" fn parse_events__decode_legacy_cache(name: *const c_char, extended_pmu_type: c_int, config: *mut __u64) -> c_int {
    let mut len: c_int = 0;
    let mut cache_type: c_int;
    let mut cache_op: c_int = -1;
    let mut cache_result: c_int = -1;
    let name_end = name.add(strlen(name) + 1);
    let mut str_ = name;
    cache_type = parse_aliases(str_, evsel__hw_cache.as_ptr(), PERF_COUNT_HW_CACHE_MAX, &mut len);
    if cache_type == -1 { return -EINVAL; }
    str_ = str_.add((len + 1) as usize);
    if str_ < name_end {
        cache_op = parse_aliases(str_, evsel__hw_cache_op.as_ptr(), PERF_COUNT_HW_CACHE_OP_MAX, &mut len);
        if cache_op >= 0 {
            if !evsel__is_cache_op_valid(cache_type, cache_op) { return -EINVAL; }
            str_ = str_.add((len + 1) as usize);
        } else {
            cache_result = parse_aliases(str_, evsel__hw_cache_result.as_ptr(), PERF_COUNT_HW_CACHE_RESULT_MAX, &mut len);
            if cache_result >= 0 { str_ = str_.add((len + 1) as usize); }
        }
    }
    if str_ < name_end {
        if cache_op < 0 {
            cache_op = parse_aliases(str_, evsel__hw_cache_op.as_ptr(), PERF_COUNT_HW_CACHE_OP_MAX, &mut len);
            if cache_op >= 0 && !evsel__is_cache_op_valid(cache_type, cache_op) { return -EINVAL; }
        } else if cache_result < 0 {
            cache_result = parse_aliases(str_, evsel__hw_cache_result.as_ptr(), PERF_COUNT_HW_CACHE_RESULT_MAX, &mut len);
        }
    }
    if cache_op == -1 { cache_op = PERF_COUNT_HW_CACHE_OP_READ; }
    if cache_result == -1 { cache_result = PERF_COUNT_HW_CACHE_RESULT_ACCESS; }
    *config = (cache_type as u64_t) | ((cache_op as u64_t) << 8) | ((cache_result as u64_t) << 16);
    if perf_pmus__supports_extended_type() {
        *config |= (extended_pmu_type as __u64) << PERF_PMU_TYPE_SHIFT;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn parse_events__filter_pmu(parse_state: *const parse_events_state, pmu: *const perf_pmu) -> bool_t {
    if (*parse_state).pmu_filter.is_null() { return false; }
    if (*parse_state).cputype_filter && !(*pmu).is_core { return false; }
    perf_pmu__wildcard_match(pmu, (*parse_state).pmu_filter) == 0
}

unsafe fn tracepoint_error(e: *mut parse_events_error, mut err: c_int, sys: *const c_char, name: *const c_char, column: c_int) {
    let str_: *const c_char;
    let mut help = [0 as c_char; BUFSIZ];
    if e.is_null() { return; }
    err = abs(err);
    match err {
        EACCES => str_ = cstr(b"can't access trace events\0"),
        ENOENT => str_ = cstr(b"unknown tracepoint\0"),
        _ => str_ = cstr(b"failed to add tracepoint\0"),
    }
    tracing_path__strerror_open_tp(err, help.as_mut_ptr(), help.len(), sys, name);
    parse_events_error__handle(e, column, strdup(str_), strdup(help.as_ptr()));
}

unsafe fn add_tracepoint(parse_state: *mut parse_events_state, list: *mut list_head, sys_name: *const c_char, evt_name: *const c_char, err: *mut parse_events_error, head_config: *mut parse_events_terms, loc_: *mut c_void) -> c_int {
    let loc = loc_ as *mut YYLTYPE;
    let evsel = evsel__newtp_idx(sys_name, evt_name, { let old = (*parse_state).idx; (*parse_state).idx += 1; old }, !(*parse_state).fake_tp);
    if is_err(evsel as *mut c_void) {
        tracepoint_error(err, ptr_err(evsel as *mut c_void), sys_name, evt_name, (*loc).first_column);
        return ptr_err(evsel as *mut c_void);
    }
    if !head_config.is_null() {
        let mut config_terms = list_head { next: null_mut(), prev: null_mut() };
        INIT_LIST_HEAD(&mut config_terms);
        if get_config_terms(head_config, &mut config_terms) != 0 { return -ENOMEM; }
        list_splice(&mut config_terms, &mut (*evsel).config_terms);
    }
    list_add_tail(&mut (*evsel).core.node, list);
    0
}

#[repr(C)]
pub struct add_tracepoint_multi_args {
    pub parse_state: *mut parse_events_state,
    pub list: *mut list_head,
    pub sys_glob: *const c_char,
    pub evt_glob: *const c_char,
    pub err: *mut parse_events_error,
    pub head_config: *mut parse_events_terms,
    pub loc: *mut YYLTYPE,
    pub found: c_int,
}

unsafe extern "C" fn add_tracepoint_multi_event_cb(state: *mut c_void, sys_name: *const c_char, evt_name: *const c_char) -> c_int {
    let args = state as *mut add_tracepoint_multi_args;
    if !strglobmatch(evt_name, (*args).evt_glob) { return 0; }
    (*args).found += 1;
    add_tracepoint((*args).parse_state, (*args).list, sys_name, evt_name, (*args).err, (*args).head_config, (*args).loc as *mut c_void)
}

unsafe fn add_tracepoint_multi_event(args: *mut add_tracepoint_multi_args, sys_name: *const c_char) -> c_int {
    if strpbrk((*args).evt_glob, cstr(b"*?\0")).is_null() {
        (*args).found += 1;
        return add_tracepoint((*args).parse_state, (*args).list, sys_name, (*args).evt_glob, (*args).err, (*args).head_config, (*args).loc as *mut c_void);
    }
    tp_pmu__for_each_tp_event(sys_name, args as *mut c_void, add_tracepoint_multi_event_cb)
}

unsafe extern "C" fn add_tracepoint_multi_sys_cb(state: *mut c_void, sys_name: *const c_char) -> c_int {
    let args = state as *mut add_tracepoint_multi_args;
    if !strglobmatch(sys_name, (*args).sys_glob) { return 0; }
    add_tracepoint_multi_event(args, sys_name)
}

unsafe fn add_tracepoint_multi_sys(parse_state: *mut parse_events_state, list: *mut list_head, sys_glob: *const c_char, evt_glob: *const c_char, err: *mut parse_events_error, head_config: *mut parse_events_terms, loc: *mut YYLTYPE) -> c_int {
    let mut args = add_tracepoint_multi_args { parse_state, list, sys_glob, evt_glob, err, head_config, loc, found: 0 };
    let ret = if strpbrk(sys_glob, cstr(b"*?\0")).is_null() {
        add_tracepoint_multi_event(&mut args, sys_glob)
    } else {
        tp_pmu__for_each_tp_sys(&mut args as *mut _ as *mut c_void, add_tracepoint_multi_sys_cb)
    };
    if args.found == 0 {
        tracepoint_error(err, ENOENT, sys_glob, evt_glob, (*loc).first_column);
        return -ENOENT;
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn default_breakpoint_len() -> size_t {
    // C had architecture-specific preprocessor branches:
    // i386 caches kernel bitness and returns sizeof(u64) or sizeof(long);
    // aarch64 returns 4; otherwise returns sizeof(long).
    size_of::<c_ulong>()
}

unsafe fn parse_breakpoint_type(type_: *const c_char, attr: *mut perf_event_attr) -> c_int {
    let mut i = 0usize;
    while i < 3 {
        if type_.is_null() || *type_.add(i) == 0 { break; }
        match *type_.add(i) as u8 as char {
            'r' => { if ((*attr).bp_type & HW_BREAKPOINT_R) != 0 { return -EINVAL; } else { (*attr).bp_type |= HW_BREAKPOINT_R; } }
            'w' => { if ((*attr).bp_type & HW_BREAKPOINT_W) != 0 { return -EINVAL; } else { (*attr).bp_type |= HW_BREAKPOINT_W; } }
            'x' => { if ((*attr).bp_type & HW_BREAKPOINT_X) != 0 { return -EINVAL; } else { (*attr).bp_type |= HW_BREAKPOINT_X; } }
            _ => return -EINVAL,
        }
        i += 1;
    }
    if (*attr).bp_type == 0 { (*attr).bp_type = HW_BREAKPOINT_R | HW_BREAKPOINT_W; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_add_breakpoint(parse_state: *mut parse_events_state, list: *mut list_head, addr: u64_t, type_: *mut c_char, mut len: u64_t, head_config: *mut parse_events_terms) -> c_int {
    let mut attr: perf_event_attr = zeroed();
    let mut config_terms = list_head { next: null_mut(), prev: null_mut() };
    INIT_LIST_HEAD(&mut config_terms);
    attr.bp_addr = addr;
    if parse_breakpoint_type(type_, &mut attr) != 0 { return -EINVAL; }
    if len == 0 {
        len = if attr.bp_type == HW_BREAKPOINT_X { default_breakpoint_len() as u64_t } else { HW_BREAKPOINT_LEN_4 };
    }
    attr.bp_len = len;
    attr.type_ = PERF_TYPE_BREAKPOINT;
    attr.sample_period = 1;
    if !head_config.is_null() {
        if config_attr(&mut attr, head_config, parse_state, config_term_common) != 0 { return -EINVAL; }
        if get_config_terms(head_config, &mut config_terms) != 0 { return -ENOMEM; }
    }
    let name = get_config_name(head_config);
    add_event(list, &mut (*parse_state).idx, &mut attr, name, null(), &mut config_terms, PERF_COUNT_HW_MAX)
}

unsafe fn check_type_val(term: *mut parse_events_term, err: *mut parse_events_error, type_: parse_events__term_val_type) -> c_int {
    if type_ == (*term).type_val { return 0; }
    if !err.is_null() {
        parse_events_error__handle(err, (*term).err_val,
            if type_ == parse_events__term_val_type::PARSE_EVENTS__TERM_TYPE_NUM { strdup(cstr(b"expected numeric value\0")) } else { strdup(cstr(b"expected string value\0")) },
            null_mut());
    }
    -EINVAL
}

static mut config_term_shrinked: bool_t = false;

#[no_mangle]
pub unsafe extern "C" fn parse_events__term_type_str(term_type: parse_events__term_type) -> *const c_char {
    static names: [*const c_char; parse_events__term_type::__PARSE_EVENTS__TERM_TYPE_NR as usize] = [
        cstr(b"<sysfs term>\0"), cstr(b"config\0"), cstr(b"config1\0"), cstr(b"config2\0"), cstr(b"config3\0"),
        cstr(b"config4\0"), cstr(b"name\0"), cstr(b"period\0"), cstr(b"freq\0"), cstr(b"branch_type\0"),
        cstr(b"time\0"), cstr(b"call-graph\0"), cstr(b"stack-size\0"), cstr(b"no-inherit\0"), cstr(b"inherit\0"),
        cstr(b"max-stack\0"), cstr(b"nr\0"), cstr(b"overwrite\0"), cstr(b"no-overwrite\0"), cstr(b"driver-config\0"),
        cstr(b"percore\0"), cstr(b"aux-output\0"), cstr(b"aux-action\0"), cstr(b"aux-sample-size\0"),
        cstr(b"metric-id\0"), cstr(b"raw\0"), cstr(b"legacy-hardware-config\0"), cstr(b"legacy-cache-config\0"),
        cstr(b"cpu\0"), cstr(b"ratio-to-prev\0"),
    ];
    let idx = term_type as isize;
    if idx < 0 || idx as usize >= names.len() { cstr(b"unknown term\0") } else { names[idx as usize] }
}

unsafe fn config_term_avail(term_type: parse_events__term_type, err: *mut parse_events_error) -> bool_t {
    let idx = term_type as isize;
    if idx < 0 || idx >= parse_events__term_type::__PARSE_EVENTS__TERM_TYPE_NR as isize {
        parse_events_error__handle(err, -1, strdup(cstr(b"Invalid term_type\0")), null_mut());
        return false;
    }
    if !config_term_shrinked { return true; }
    match term_type {
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_CONFIG |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_CONFIG1 |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_CONFIG2 |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_CONFIG3 |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_CONFIG4 |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_NAME |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_METRIC_ID |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_SAMPLE_PERIOD |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_PERCORE |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_CPU => true,
        _ => {
            if err.is_null() { return false; }
            let mut err_str: *mut c_char = null_mut();
            if asprintf(&mut err_str, cstr(b"'%s' is not usable in 'perf stat'\0"), parse_events__term_type_str(term_type)) >= 0 {
                parse_events_error__handle(err, -1, err_str, null_mut());
            }
            false
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn parse_events__shrink_config_terms() { config_term_shrinked = true; }

unsafe fn config_term_common(attr: *mut perf_event_attr, term: *mut parse_events_term, parse_state: *mut parse_events_state) -> c_int {
    macro_rules! check_num { () => { if check_type_val(term, (*parse_state).error, parse_events__term_val_type::PARSE_EVENTS__TERM_TYPE_NUM) != 0 { return -EINVAL; } }; }
    macro_rules! check_str { () => { if check_type_val(term, (*parse_state).error, parse_events__term_val_type::PARSE_EVENTS__TERM_TYPE_STR) != 0 { return -EINVAL; } }; }
    match (*term).type_term {
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_CONFIG => { check_num!(); (*attr).config = (*term).val.num; }
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_CONFIG1 => { check_num!(); (*attr).config1 = (*term).val.num; }
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_CONFIG2 => { check_num!(); (*attr).config2 = (*term).val.num; }
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_CONFIG3 => { check_num!(); (*attr).config3 = (*term).val.num; }
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_CONFIG4 => { check_num!(); (*attr).config4 = (*term).val.num; }
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_BRANCH_SAMPLE_TYPE => {
            check_str!();
            if strcmp((*term).val.str_, cstr(b"no\0")) != 0 && parse_branch_str((*term).val.str_, &mut (*attr).branch_sample_type) != 0 {
                parse_events_error__handle((*parse_state).error, (*term).err_val, strdup(cstr(b"invalid branch sample type\0")), null_mut());
                return -EINVAL;
            }
        }
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_TIME |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_PERCORE => {
            check_num!();
            if (*term).val.num > 1 {
                parse_events_error__handle((*parse_state).error, (*term).err_val, strdup(cstr(b"expected 0 or 1\0")), null_mut());
                return -EINVAL;
            }
        }
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_AUX_SAMPLE_SIZE => {
            check_num!();
            if (*term).val.num > UINT_MAX {
                parse_events_error__handle((*parse_state).error, (*term).err_val, strdup(cstr(b"too big\0")), null_mut());
                return -EINVAL;
            }
        }
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_CPU => {
            if (*term).type_val == parse_events__term_val_type::PARSE_EVENTS__TERM_TYPE_NUM {
                if (*term).val.num >= cpu__max_present_cpu().cpu as u64_t {
                    parse_events_error__handle((*parse_state).error, (*term).err_val, strdup(cstr(b"too big\0")), null_mut());
                    return -EINVAL;
                }
            } else {
                if !perf_pmus__find((*term).val.str_).is_null() { }
                else {
                    let map = perf_cpu_map__new((*term).val.str_);
                    if map.is_null() && !(*parse_state).fake_pmu {
                        parse_events_error__handle((*parse_state).error, (*term).err_val, strdup(cstr(b"not a valid PMU or CPU number\0")), null_mut());
                        return -EINVAL;
                    }
                    perf_cpu_map__put(map);
                }
            }
        }
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_RATIO_TO_PREV => {
            check_str!();
            if strtod((*term).val.str_, null_mut()) <= 0.0 {
                parse_events_error__handle((*parse_state).error, (*term).err_val, strdup(cstr(b"zero or negative\0")), null_mut());
                return -EINVAL;
            }
            if errno == ERANGE {
                parse_events_error__handle((*parse_state).error, (*term).err_val, strdup(cstr(b"too big\0")), null_mut());
                return -EINVAL;
            }
        }
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_SAMPLE_PERIOD |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_SAMPLE_FREQ |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_STACKSIZE |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_INHERIT |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_NOINHERIT |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_OVERWRITE |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_NOOVERWRITE |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_MAX_STACK |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_MAX_EVENTS |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_AUX_OUTPUT => { check_num!(); }
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_CALLGRAPH |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_NAME |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_METRIC_ID |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_RAW |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_AUX_ACTION => { check_str!(); }
        _ => {
            parse_events_error__handle((*parse_state).error, (*term).err_term,
                strdup(parse_events__term_type_str((*term).type_term)),
                parse_events_formats_error_string(null_mut()));
            return -EINVAL;
        }
    }
    if !config_term_avail((*term).type_term, (*parse_state).error) { return -EINVAL; }
    0
}

unsafe fn check_pmu_is_core(type_: __u32, term: *const parse_events_term, err: *mut parse_events_error) -> bool_t {
    let mut pmu: *mut perf_pmu = null_mut();
    loop {
        pmu = perf_pmus__scan_core(pmu);
        if pmu.is_null() { break; }
        if (*pmu).type_ == type_ { return true; }
    }
    parse_events_error__handle(err, (*term).err_val, strdup(cstr(b"needs a core PMU\0")), null_mut());
    false
}

unsafe fn config_term_pmu(attr: *mut perf_event_attr, term: *mut parse_events_term, parse_state: *mut parse_events_state) -> c_int {
    if (*term).type_term == parse_events__term_type::PARSE_EVENTS__TERM_TYPE_LEGACY_HARDWARE_CONFIG {
        if check_type_val(term, (*parse_state).error, parse_events__term_val_type::PARSE_EVENTS__TERM_TYPE_NUM) != 0 { return -EINVAL; }
        if (*term).val.num >= PERF_COUNT_HW_MAX {
            parse_events_error__handle((*parse_state).error, (*term).err_val, strdup(cstr(b"too big\0")), null_mut());
            return -EINVAL;
        }
        if !check_pmu_is_core((*attr).type_, term, (*parse_state).error) { return -EINVAL; }
        (*attr).config = (*term).val.num;
        if perf_pmus__supports_extended_type() { (*attr).config |= ((*attr).type_ as __u64) << PERF_PMU_TYPE_SHIFT; }
        (*attr).type_ = PERF_TYPE_HARDWARE;
        return 0;
    }
    if (*term).type_term == parse_events__term_type::PARSE_EVENTS__TERM_TYPE_LEGACY_CACHE_CONFIG {
        if check_type_val(term, (*parse_state).error, parse_events__term_val_type::PARSE_EVENTS__TERM_TYPE_NUM) != 0 { return -EINVAL; }
        let cache_type = ((*term).val.num & 0xff) as c_int;
        let cache_op = (((*term).val.num >> 8) & 0xff) as c_int;
        let cache_result = (((*term).val.num >> 16) & 0xff) as c_int;
        if ((*term).val.num & !0xffffff) != 0 || cache_type >= PERF_COUNT_HW_CACHE_MAX || cache_op >= PERF_COUNT_HW_CACHE_OP_MAX || cache_result >= PERF_COUNT_HW_CACHE_RESULT_MAX {
            parse_events_error__handle((*parse_state).error, (*term).err_val, strdup(cstr(b"too big\0")), null_mut());
            return -EINVAL;
        }
        if !check_pmu_is_core((*attr).type_, term, (*parse_state).error) { return -EINVAL; }
        (*attr).config = (*term).val.num;
        if perf_pmus__supports_extended_type() { (*attr).config |= ((*attr).type_ as __u64) << PERF_PMU_TYPE_SHIFT; }
        (*attr).type_ = PERF_TYPE_HW_CACHE;
        return 0;
    }
    if (*term).type_term == parse_events__term_type::PARSE_EVENTS__TERM_TYPE_USER || (*term).type_term == parse_events__term_type::PARSE_EVENTS__TERM_TYPE_DRV_CFG {
        return 0;
    }
    config_term_common(attr, term, parse_state)
}

unsafe fn config_term_tracepoint(attr: *mut perf_event_attr, term: *mut parse_events_term, parse_state: *mut parse_events_state) -> c_int {
    match (*term).type_term {
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_CALLGRAPH |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_STACKSIZE |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_INHERIT |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_NOINHERIT |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_MAX_STACK |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_MAX_EVENTS |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_OVERWRITE |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_NOOVERWRITE |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_AUX_OUTPUT |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_AUX_ACTION |
        parse_events__term_type::PARSE_EVENTS__TERM_TYPE_AUX_SAMPLE_SIZE => config_term_common(attr, term, parse_state),
        _ => {
            parse_events_error__handle((*parse_state).error, (*term).err_term,
                strdup(parse_events__term_type_str((*term).type_term)),
                strdup(cstr(b"valid terms: call-graph,stack-size\n\0")));
            -EINVAL
        }
    }
}

unsafe fn config_attr(attr: *mut perf_event_attr, head: *const parse_events_terms, parse_state: *mut parse_events_state, config_term: config_term_func_t) -> c_int {
    // TODO(list_for_each_entry): call config_term(attr, term, parse_state) for every term in head->terms.
    let _ = (attr, head, parse_state, config_term);
    0
}

unsafe fn add_config_term(type_: evsel_term_type, head_terms: *mut list_head, weak: bool_t, str_: *mut c_char, val: u64_t) -> *mut evsel_config_term {
    let t = zalloc(size_of::<evsel_config_term>()) as *mut evsel_config_term;
    if t.is_null() { return null_mut(); }
    INIT_LIST_HEAD(&mut (*t).list);
    (*t).type_ = type_;
    (*t).weak = weak;
    match type_ {
        evsel_term_type::EVSEL__CONFIG_TERM_CALLGRAPH |
        evsel_term_type::EVSEL__CONFIG_TERM_BRANCH |
        evsel_term_type::EVSEL__CONFIG_TERM_DRV_CFG |
        evsel_term_type::EVSEL__CONFIG_TERM_RATIO_TO_PREV |
        evsel_term_type::EVSEL__CONFIG_TERM_AUX_ACTION => {
            if !str_.is_null() {
                (*t).val.str_ = strdup(str_);
                if (*t).val.str_.is_null() {
                    zfree_void(&mut (t as *mut evsel_config_term));
                    return null_mut();
                }
                (*t).free_str = true;
            }
        }
        evsel_term_type::EVSEL__CONFIG_TERM_TIME => (*t).val.time = val,
        evsel_term_type::EVSEL__CONFIG_TERM_INHERIT => (*t).val.inherit = val,
        evsel_term_type::EVSEL__CONFIG_TERM_OVERWRITE => (*t).val.overwrite = val,
        evsel_term_type::EVSEL__CONFIG_TERM_MAX_STACK => (*t).val.max_stack = val,
        evsel_term_type::EVSEL__CONFIG_TERM_MAX_EVENTS => (*t).val.max_events = val,
        evsel_term_type::EVSEL__CONFIG_TERM_PERCORE => (*t).val.percore = val != 0,
        evsel_term_type::EVSEL__CONFIG_TERM_AUX_OUTPUT => (*t).val.aux_output = val,
        evsel_term_type::EVSEL__CONFIG_TERM_AUX_SAMPLE_SIZE => (*t).val.aux_sample_size = val,
        _ => (*t).val.val = val,
    }
    list_add_tail(&mut (*t).list, head_terms);
    t
}

unsafe fn get_config_terms(head_config: *const parse_events_terms, head_terms: *mut list_head) -> c_int {
    // TODO(list_for_each_entry): translate parse_events_term types to evsel_config_term entries as in C.
    let _ = (head_config, head_terms);
    0
}

unsafe fn add_cfg_chg(pmu: *const perf_pmu, head_config: *const parse_events_terms, head_terms: *mut list_head, format_type: c_int, term_type: parse_events__term_type, new_term_type: evsel_term_type) -> c_int {
    // TODO(list_for_each_entry): OR changed format bits from USER terms and full mask for explicit configN terms.
    let _ = (pmu, head_config, format_type, term_type);
    let bits: u64_t = 0;
    if bits != 0 && add_config_term(new_term_type, head_terms, false, null_mut(), bits).is_null() { return -ENOMEM; }
    0
}

unsafe fn get_config_chgs(pmu: *const perf_pmu, head_config: *const parse_events_terms, head_terms: *mut list_head) -> c_int {
    let mut ret = add_cfg_chg(pmu, head_config, head_terms, PERF_PMU_FORMAT_VALUE_CONFIG, parse_events__term_type::PARSE_EVENTS__TERM_TYPE_CONFIG, evsel_term_type::EVSEL__CONFIG_TERM_USR_CHG_CONFIG);
    if ret != 0 { return ret; }
    ret = add_cfg_chg(pmu, head_config, head_terms, PERF_PMU_FORMAT_VALUE_CONFIG1, parse_events__term_type::PARSE_EVENTS__TERM_TYPE_CONFIG1, evsel_term_type::EVSEL__CONFIG_TERM_USR_CHG_CONFIG1);
    if ret != 0 { return ret; }
    ret = add_cfg_chg(pmu, head_config, head_terms, PERF_PMU_FORMAT_VALUE_CONFIG2, parse_events__term_type::PARSE_EVENTS__TERM_TYPE_CONFIG2, evsel_term_type::EVSEL__CONFIG_TERM_USR_CHG_CONFIG2);
    if ret != 0 { return ret; }
    ret = add_cfg_chg(pmu, head_config, head_terms, PERF_PMU_FORMAT_VALUE_CONFIG3, parse_events__term_type::PARSE_EVENTS__TERM_TYPE_CONFIG3, evsel_term_type::EVSEL__CONFIG_TERM_USR_CHG_CONFIG3);
    if ret != 0 { return ret; }
    add_cfg_chg(pmu, head_config, head_terms, PERF_PMU_FORMAT_VALUE_CONFIG4, parse_events__term_type::PARSE_EVENTS__TERM_TYPE_CONFIG4, evsel_term_type::EVSEL__CONFIG_TERM_USR_CHG_CONFIG4)
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_add_tracepoint(parse_state: *mut parse_events_state, list: *mut list_head, sys: *const c_char, event: *const c_char, err: *mut parse_events_error, head_config: *mut parse_events_terms, loc_: *mut c_void) -> c_int {
    let loc = loc_ as *mut YYLTYPE;
    if !head_config.is_null() {
        let mut attr: perf_event_attr = zeroed();
        if config_attr(&mut attr, head_config, parse_state, config_term_tracepoint) != 0 { return -EINVAL; }
    }
    add_tracepoint_multi_sys(parse_state, list, sys, event, err, head_config, loc)
}

unsafe fn __parse_events_add_numeric(parse_state: *mut parse_events_state, list: *mut list_head, pmu: *mut perf_pmu, type_: u32_t, extended_type: u32_t, config: u64_t, head_config: *const parse_events_terms, first_wildcard_match: *mut evsel) -> c_int {
    let mut attr: perf_event_attr = zeroed();
    let mut config_terms = list_head { next: null_mut(), prev: null_mut() };
    INIT_LIST_HEAD(&mut config_terms);
    attr.type_ = type_;
    attr.config = config;
    if extended_type != 0 && (type_ == PERF_TYPE_HARDWARE || type_ == PERF_TYPE_HW_CACHE) {
        attr.config |= (extended_type as u64_t) << PERF_PMU_TYPE_SHIFT;
    }
    if !head_config.is_null() {
        if config_attr(&mut attr, head_config, parse_state, config_term_common) != 0 { return -EINVAL; }
        if get_config_terms(head_config, &mut config_terms) != 0 { return -ENOMEM; }
    }
    let cpus = get_config_cpu(head_config, (*parse_state).fake_pmu);
    let ret = if !__add_event(list, &mut (*parse_state).idx, &mut attr, true, get_config_name(head_config), get_config_metric_id(head_config), pmu, &mut config_terms, first_wildcard_match, cpus, PERF_COUNT_HW_MAX).is_null() { 0 } else { -ENOMEM };
    perf_cpu_map__put(cpus);
    free_config_terms(&mut config_terms);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_add_numeric(parse_state: *mut parse_events_state, list: *mut list_head, type_: u32_t, config: u64_t, head_config: *const parse_events_terms, wildcard: bool_t) -> c_int {
    let mut pmu: *mut perf_pmu = null_mut();
    let mut found_supported = false;
    if wildcard && perf_pmus__supports_extended_type() {
        let mut first_wildcard_match: *mut evsel = null_mut();
        loop {
            pmu = perf_pmus__scan_core(pmu);
            if pmu.is_null() { break; }
            found_supported = true;
            if parse_events__filter_pmu(parse_state, pmu) { continue; }
            let ret = __parse_events_add_numeric(parse_state, list, pmu, type_, (*pmu).type_, config, head_config, first_wildcard_match);
            if ret != 0 { return ret; }
            if first_wildcard_match.is_null() {
                // TODO(container_of(list->prev, struct evsel, core.node)).
            }
        }
        if found_supported { return 0; }
    }
    __parse_events_add_numeric(parse_state, list, perf_pmus__find_by_type(type_), type_, 0, config, head_config, null_mut())
}

unsafe fn config_term_percore(config_terms: *mut list_head) -> bool_t {
    // TODO(list_for_each_entry): return term->val.percore for EVSEL__CONFIG_TERM_PERCORE.
    let _ = config_terms;
    false
}

unsafe fn parse_events_add_pmu(parse_state: *mut parse_events_state, list: *mut list_head, pmu: *mut perf_pmu, const_parsed_terms: *const parse_events_terms, first_wildcard_match: *mut evsel) -> c_int {
    let mut alternate_hw_config = PERF_COUNT_HW_MAX;
    let mut attr: perf_event_attr = zeroed();
    let mut info: perf_pmu_info = zeroed();
    let mut config_terms = list_head { next: null_mut(), prev: null_mut() };
    let mut parsed_terms: parse_events_terms = zeroed();
    let mut alias_rewrote_terms = false;
    INIT_LIST_HEAD(&mut config_terms);
    if !(*pmu).perf_event_attr_init_default.is_none() {
        ((*pmu).perf_event_attr_init_default.unwrap())(pmu, &mut attr);
    }
    attr.type_ = (*pmu).type_;
    if const_parsed_terms.is_null() || list_empty(&(*const_parsed_terms).terms) {
        let evsel = __add_event(list, &mut (*parse_state).idx, &mut attr, true, null(), null(), pmu, null_mut(), first_wildcard_match, null_mut(), alternate_hw_config);
        return if !evsel.is_null() { 0 } else { -ENOMEM };
    }
    parse_events_terms__init(&mut parsed_terms);
    if !const_parsed_terms.is_null() {
        let ret = parse_events_terms__copy(const_parsed_terms, &mut parsed_terms);
        if ret != 0 { return ret; }
    }
    fix_raw(&mut parsed_terms, pmu);
    if config_attr(&mut attr, &mut parsed_terms, parse_state, config_term_pmu) != 0 {
        parse_events_terms__exit(&mut parsed_terms);
        return -EINVAL;
    }
    if perf_pmu__check_alias(pmu, &mut parsed_terms, &mut info, &mut alias_rewrote_terms, &mut alternate_hw_config, (*parse_state).error) != 0 {
        parse_events_terms__exit(&mut parsed_terms);
        return -EINVAL;
    }
    if alias_rewrote_terms && config_attr(&mut attr, &mut parsed_terms, parse_state, config_term_pmu) != 0 {
        parse_events_terms__exit(&mut parsed_terms);
        return -EINVAL;
    }
    if get_config_terms(&mut parsed_terms, &mut config_terms) != 0 {
        parse_events_terms__exit(&mut parsed_terms);
        return -ENOMEM;
    }
    if get_config_chgs(pmu, &mut parsed_terms, &mut config_terms) != 0 {
        parse_events_terms__exit(&mut parsed_terms);
        return -ENOMEM;
    }
    if perf_pmu__config(pmu, &mut attr, &mut parsed_terms, false, (*parse_state).error) != 0 {
        free_config_terms(&mut config_terms);
        parse_events_terms__exit(&mut parsed_terms);
        return -EINVAL;
    }
    let term_cpu = get_config_cpu(&mut parsed_terms, (*parse_state).fake_pmu);
    let evsel = __add_event(list, &mut (*parse_state).idx, &mut attr, true, get_config_name(&mut parsed_terms), get_config_metric_id(&mut parsed_terms), pmu, &mut config_terms, first_wildcard_match, term_cpu, alternate_hw_config);
    perf_cpu_map__put(term_cpu);
    if evsel.is_null() {
        parse_events_terms__exit(&mut parsed_terms);
        return -ENOMEM;
    }
    if !(*evsel).name.is_null() { (*evsel).use_config_name = true; }
    (*evsel).percore = config_term_percore(&mut (*evsel).config_terms);
    parse_events_terms__exit(&mut parsed_terms);
    free((*evsel).unit as *mut c_void);
    (*evsel).unit = strdup(info.unit);
    (*evsel).scale = info.scale;
    (*evsel).per_pkg = info.per_pkg;
    (*evsel).snapshot = info.snapshot;
    (*evsel).retirement_latency.mean = info.retirement_latency_mean;
    (*evsel).retirement_latency.min = info.retirement_latency_min;
    (*evsel).retirement_latency.max = info.retirement_latency_max;
    0
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_multi_pmu_add(parse_state: *mut parse_events_state, event_name: *const c_char, const_parsed_terms: *const parse_events_terms, listp: *mut *mut list_head, loc_: *mut c_void) -> c_int {
    let loc = loc_ as *mut YYLTYPE;
    let mut list: *mut list_head = null_mut();
    let mut pmu: *mut perf_pmu = null_mut();
    let mut ok = 0;
    let mut parsed_terms: parse_events_terms = zeroed();
    let first_wildcard_match: *mut evsel = null_mut();
    *listp = null_mut();
    parse_events_terms__init(&mut parsed_terms);
    if !const_parsed_terms.is_null() {
        let ret = parse_events_terms__copy(const_parsed_terms, &mut parsed_terms);
        if ret != 0 { return ret; }
    }
    let config = strdup(event_name);
    if config.is_null() { goto_multi_out(&mut parsed_terms, ok, list, listp); return if ok != 0 { 0 } else { -1 }; }
    let mut term: *mut parse_events_term = null_mut();
    if parse_events_term__num(&mut term, parse_events__term_type::PARSE_EVENTS__TERM_TYPE_USER, config, 1, true, loc as *mut c_void, null_mut()) < 0 {
        zfree_char(&mut (config as *mut c_char));
        goto_multi_out(&mut parsed_terms, ok, list, listp);
        return if ok != 0 { 0 } else { -1 };
    }
    list_add_tail(&mut (*term).list, &mut parsed_terms.terms);
    list = malloc(size_of::<list_head>()) as *mut list_head;
    if list.is_null() { goto_multi_out(&mut parsed_terms, ok, list, listp); return -1; }
    INIT_LIST_HEAD(list);
    loop {
        pmu = perf_pmus__scan_for_event(pmu, event_name);
        if pmu.is_null() { break; }
        if parse_events__filter_pmu(parse_state, pmu) { continue; }
        if !perf_pmu__have_event(pmu, event_name) { continue; }
        if parse_events_add_pmu(parse_state, list, pmu, &parsed_terms, first_wildcard_match) == 0 { ok += 1; }
    }
    if (*parse_state).fake_pmu {
        if parse_events_add_pmu(parse_state, list, perf_pmus__fake_pmu(), &parsed_terms, first_wildcard_match) == 0 { ok += 1; }
    }
    goto_multi_out(&mut parsed_terms, ok, list, listp);
    if ok != 0 { 0 } else { -1 }
}

unsafe fn goto_multi_out(parsed_terms: *mut parse_events_terms, ok: c_int, list: *mut list_head, listp: *mut *mut list_head) {
    parse_events_terms__exit(parsed_terms);
    if ok != 0 { *listp = list; } else { free(list as *mut c_void); }
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_multi_pmu_add_or_add_pmu(parse_state: *mut parse_events_state, event_or_pmu: *const c_char, const_parsed_terms: *const parse_events_terms, listp: *mut *mut list_head, loc_: *mut c_void) -> c_int {
    let loc = loc_ as *mut YYLTYPE;
    let mut pmu: *mut perf_pmu;
    let mut ok = 0;
    let first_wildcard_match: *mut evsel = null_mut();
    *listp = malloc(size_of::<list_head>()) as *mut list_head;
    if (*listp).is_null() { return -ENOMEM; }
    INIT_LIST_HEAD(*listp);
    pmu = perf_pmus__find(event_or_pmu);
    if !pmu.is_null() && parse_events_add_pmu(parse_state, *listp, pmu, const_parsed_terms, first_wildcard_match) == 0 { return 0; }
    if (*parse_state).fake_pmu && parse_events_add_pmu(parse_state, *listp, perf_pmus__fake_pmu(), const_parsed_terms, first_wildcard_match) == 0 { return 0; }
    pmu = null_mut();
    loop {
        pmu = perf_pmus__scan_matching_wildcard(pmu, event_or_pmu);
        if pmu.is_null() { break; }
        if parse_events__filter_pmu(parse_state, pmu) { continue; }
        if parse_events_add_pmu(parse_state, *listp, pmu, const_parsed_terms, first_wildcard_match) == 0 {
            ok += 1;
            (*parse_state).wild_card_pmus = true;
        }
    }
    if ok != 0 { return 0; }
    zfree_void(listp);
    if parse_events_multi_pmu_add(parse_state, event_or_pmu, const_parsed_terms, listp, loc as *mut c_void) == 0 { return 0; }
    let mut help: *mut c_char = null_mut();
    if asprintf(&mut help, cstr(b"Unable to find PMU or event on a PMU of '%s'\0"), event_or_pmu) < 0 { help = null_mut(); }
    parse_events_error__handle((*parse_state).error, (*loc).first_column, strdup(cstr(b"Bad event or PMU\0")), help);
    zfree_void(listp);
    -EINVAL
}

#[no_mangle]
pub unsafe extern "C" fn parse_events__set_leader(name: *mut c_char, list: *mut list_head) {
    if list_empty(list) { return; }
    // TODO(list_first_entry): set first evsel as group leader and store group_name.
    let _ = name;
}

unsafe fn parse_events__modifier_list(parse_state: *mut parse_events_state, loc: *mut YYLTYPE, list: *mut list_head, mod_: parse_events_modifier, group: bool_t) -> c_int {
    if !group && mod_.weak {
        parse_events_error__handle((*parse_state).error, (*loc).first_column, strdup(cstr(b"Weak modifier is for use with groups\0")), null_mut());
        return -EINVAL;
    }
    // TODO(__evlist__for_each_entry): apply exclude, precision and simple modifier fields to every evsel.
    let _ = (list, mod_);
    0
}

#[no_mangle]
pub unsafe extern "C" fn parse_events__modifier_group(parse_state: *mut parse_events_state, loc: *mut c_void, list: *mut list_head, mod_: parse_events_modifier) -> c_int {
    parse_events__modifier_list(parse_state, loc as *mut YYLTYPE, list, mod_, true)
}

#[no_mangle]
pub unsafe extern "C" fn parse_events__modifier_event(parse_state: *mut parse_events_state, loc: *mut c_void, list: *mut list_head, mod_: parse_events_modifier) -> c_int {
    parse_events__modifier_list(parse_state, loc as *mut YYLTYPE, list, mod_, false)
}

#[no_mangle]
pub unsafe extern "C" fn parse_events__set_default_name(list: *mut list_head, name: *mut c_char) -> c_int {
    // TODO(__evlist__for_each_entry): set name on unnamed evsels, duplicating after first use; free unused name.
    let _ = list;
    free(name as *mut c_void);
    0
}

unsafe fn parse_events__scanner(str_: *const c_char, parse_state: *mut parse_events_state) -> c_int {
    let mut buffer: YY_BUFFER_STATE;
    let mut scanner: *mut c_void = null_mut();
    let mut ret = parse_events_lex_init_extra(parse_state, &mut scanner);
    if ret != 0 { return ret; }
    buffer = parse_events__scan_string(str_, scanner);
    // PARSER_DEBUG build-time branch in C enables parse_events_debug and scanner debug here.
    ret = parse_events_parse(parse_state, scanner);
    parse_events__flush_buffer(buffer, scanner);
    parse_events__delete_buffer(buffer, scanner);
    parse_events_lex_destroy(scanner);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_terms(terms: *mut parse_events_terms, str_: *const c_char) -> c_int {
    let mut parse_state: parse_events_state = zeroed();
    parse_state.terms = null_mut();
    parse_state.stoken = PE_START_TERMS;
    let ret = parse_events__scanner(str_, &mut parse_state);
    if ret == 0 { list_splice(&mut (*parse_state.terms).terms, &mut (*terms).terms); }
    zfree_void(&mut parse_state.terms);
    ret
}

unsafe fn evsel__compute_group_pmu_name(evsel: *mut evsel, head: *const list_head) -> c_int {
    // TODO(list_for_each_entry): compute PMU name for grouped events, including software/AUX substitution.
    let pmu = evsel__find_pmu(evsel);
    let pmu = if pmu.is_null() { perf_pmus__scan_core(null_mut()) } else { pmu };
    if pmu.is_null() { return -EINVAL; }
    (*evsel).group_pmu_name = strdup((*pmu).name);
    if (*evsel).group_pmu_name.is_null() { -ENOMEM } else { let _ = head; 0 }
}

#[no_mangle]
pub unsafe extern "C" fn arch_evlist__cmp(lhs: *const evsel, rhs: *const evsel) -> c_int {
    (*lhs).core.idx - (*rhs).core.idx
}

unsafe extern "C" fn evlist__cmp(_fg_idx: *mut c_void, l: *const list_head, r: *const list_head) -> c_int {
    // TODO(container_of): recover evsels from list nodes and compare insertion/group PMU names as in C.
    let _ = (_fg_idx, l, r);
    0
}

#[no_mangle]
pub unsafe extern "C" fn arch_evlist__add_required_events(list: *mut list_head) -> c_int {
    let _ = list;
    0
}

unsafe fn parse_events__sort_events_and_fix_groups(list: *mut list_head) -> c_int {
    let mut force_grouped_idx: c_int = -1;
    let ret = arch_evlist__add_required_events(list);
    if ret != 0 { return ret; }
    // TODO(list_for_each_entry/list_sort): compute group PMU names, sort by evlist__cmp, split/fix groups, and wildcard alias leaders.
    list_sort(&mut force_grouped_idx as *mut _ as *mut c_void, list, evlist__cmp);
    0
}

#[no_mangle]
pub unsafe extern "C" fn __parse_events(evlist: *mut evlist, str_: *const c_char, pmu_filter: *const c_char, cputype_filter: bool_t, err: *mut parse_events_error, fake_pmu: bool_t, warn_if_reordered: bool_t, fake_tp: bool_t) -> c_int {
    let mut parse_state: parse_events_state = zeroed();
    INIT_LIST_HEAD(&mut parse_state.list);
    parse_state.idx = evlist__nr_entries(evlist);
    parse_state.error = err;
    parse_state.stoken = PE_START_EVENTS;
    parse_state.fake_pmu = fake_pmu;
    parse_state.fake_tp = fake_tp;
    parse_state.pmu_filter = pmu_filter;
    parse_state.cputype_filter = cputype_filter;
    parse_state.match_legacy_cache_terms = true;
    let mut ret = parse_events__scanner(str_, &mut parse_state);
    if ret == 0 && list_empty(&mut parse_state.list) { return -1; }
    let ret2 = parse_events__sort_events_and_fix_groups(&mut parse_state.list);
    if ret2 < 0 && ret == 0 { ret = ret2; }
    evlist__splice_list_tail(evlist, &mut parse_state.list);
    if ret2 > 0 && warn_if_reordered && !parse_state.wild_card_pmus {
        evlist__uniquify_evsel_names(evlist, &mut stat_config);
        pr_warning(cstr(b"WARNING: events were regrouped to match PMUs\n\0"));
    }
    if ret == 0 {
        let last = evlist__last(evlist);
        (*last).cmdline_group_boundary = true;
        return 0;
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn parse_event(evlist: *mut evlist, str_: *const c_char) -> c_int {
    let mut err: parse_events_error = zeroed();
    parse_events_error__init(&mut err);
    let ret = parse_events(evlist, str_, &mut err);
    if ret != 0 && verbose > 0 { parse_events_error__print(&mut err, str_); }
    parse_events_error__exit(&mut err);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_error__init(err: *mut parse_events_error) {
    INIT_LIST_HEAD(&mut (*err).list);
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_error__exit(err: *mut parse_events_error) {
    // TODO(list_for_each_entry_safe): free each parse_events_error_entry in err->list.
    let _ = err;
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_error__handle(err: *mut parse_events_error, idx: c_int, str_: *mut c_char, help: *mut c_char) {
    if str_.is_null() || err.is_null() {
        free(str_ as *mut c_void);
        free(help as *mut c_void);
        return;
    }
    let entry = zalloc(size_of::<parse_events_error_entry>()) as *mut parse_events_error_entry;
    if entry.is_null() {
        pr_err(cstr(b"Failed to allocate memory for event parsing error: %s (%s)\n\0"), str_, if help.is_null() { cstr(b"<no help>\0") } else { help });
        free(str_ as *mut c_void);
        free(help as *mut c_void);
        return;
    }
    (*entry).idx = idx;
    (*entry).str_ = str_;
    (*entry).help = help;
    list_add(&mut (*entry).list, &mut (*err).list);
}

unsafe fn get_term_width() -> c_int {
    let mut ws: winsize = zeroed();
    get_term_dimensions(&mut ws);
    if ws.ws_col as c_int > MAX_WIDTH as c_int { MAX_WIDTH as c_int } else { ws.ws_col as c_int }
}

unsafe fn __parse_events_error__print(err_idx: c_int, err_str: *const c_char, err_help: *const c_char, event: *const c_char) {
    let mut str_ = cstr(b"invalid or unsupported event: \0");
    let mut _buf = [0 as c_char; MAX_WIDTH];
    let mut buf = event as *mut c_char;
    let mut idx = 0;
    if !err_str.is_null() {
        let width = get_term_width() - 2;
        let len_event = strlen(event) as c_int;
        let mut cut = 0;
        let max_err_idx = 13;
        str_ = cstr(b"event syntax error: \0");
        let len_str = strlen(str_) as c_int;
        let max_len = width - len_str;
        buf = _buf.as_mut_ptr();
        if err_idx > max_err_idx { cut = err_idx - max_err_idx; }
        strncpy(buf, event.add(cut as usize), max_len as size_t);
        if cut != 0 { *buf = b'.' as c_char; *buf.add(1) = b'.' as c_char; }
        if len_event - cut > max_len {
            *buf.add((max_len - 1) as usize) = b'.' as c_char;
            *buf.add((max_len - 2) as usize) = b'.' as c_char;
            *buf.add(max_len as usize) = 0;
        }
        idx = len_str + err_idx - cut;
    }
    fprintf(stderr, cstr(b"%s'%s'\n\0"), str_, buf);
    if idx != 0 {
        fprintf(stderr, cstr(b"%*s\\___ %s\n\0"), idx + 1, cstr(b"\0"), err_str);
        if !err_help.is_null() { fprintf(stderr, cstr(b"\n%s\n\0"), err_help); }
    }
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_error__print(err: *const parse_events_error, event: *const c_char) {
    // TODO(list_for_each_entry): print each error entry, separated by blank lines.
    let _ = (err, event);
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_error__contains(err: *const parse_events_error, needle: *const c_char) -> bool_t {
    // TODO(list_for_each_entry): return true if any error string contains needle.
    let _ = (err, needle);
    false
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_option(opt: *const option, str_: *const c_char, unset: c_int) -> c_int {
    let args = (*opt).value as *mut parse_events_option_args;
    let mut err: parse_events_error = zeroed();
    let _ = unset;
    parse_events_error__init(&mut err);
    let ret = __parse_events(*(*args).evlistp, str_, (*args).pmu_filter, (*args).cputype_filter, &mut err, false, true, false);
    if ret != 0 {
        parse_events_error__print(&mut err, str_);
        fprintf(stderr, cstr(b"Run 'perf list' for a list of valid events\n\0"));
    }
    parse_events_error__exit(&mut err);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_option_new_evlist(opt: *const option, str_: *const c_char, unset: c_int) -> c_int {
    let args = (*opt).value as *mut parse_events_option_args;
    if (*(*args).evlistp).is_null() {
        *(*args).evlistp = evlist__new();
        if (*(*args).evlistp).is_null() {
            fprintf(stderr, cstr(b"Not enough memory to create evlist\n\0"));
            return -1;
        }
    }
    let ret = parse_events_option(opt, str_, unset);
    if ret != 0 {
        evlist__put(*(*args).evlistp);
        *(*args).evlistp = null_mut();
    }
    ret
}

unsafe fn foreach_evsel_in_last_glob(evlist: *mut evlist, func: unsafe fn(*mut evsel, *const c_void) -> c_int, arg: *const c_void) -> c_int {
    let mut last: *mut evsel = null_mut();
    if evlist__nr_entries(evlist) > 0 { last = evlist__last(evlist); }
    loop {
        let err = func(last, arg);
        if err != 0 { return -1; }
        if last.is_null() { return 0; }
        // TODO(list_entry over last->core.node.prev and evlist core entries boundary).
        if (*last).cmdline_group_boundary { break; }
        break;
    }
    0
}

unsafe fn is_possible_tp_filter(str_: *const c_char) -> bool_t {
    strstr(str_, cstr(b"uid\0")).is_null()
}

unsafe fn set_filter(evsel: *mut evsel, arg: *const c_void) -> c_int {
    let str_ = arg as *const c_char;
    let mut nr_addr_filters: c_int = 0;
    if evsel.is_null() {
        fprintf(stderr, cstr(b"--filter option should follow a -e tracepoint or HW tracer option\n\0"));
        return -1;
    }
    if (*evsel).core.attr.type_ == PERF_TYPE_TRACEPOINT && is_possible_tp_filter(str_) {
        if evsel__append_tp_filter(evsel, str_) < 0 {
            fprintf(stderr, cstr(b"not enough memory to hold filter string\n\0"));
            return -1;
        }
        return 0;
    }
    let pmu = evsel__find_pmu(evsel);
    if !pmu.is_null() {
        perf_pmu__scan_file(pmu, cstr(b"nr_addr_filters\0"), cstr(b"%d\0"), &mut nr_addr_filters);
    }
    if nr_addr_filters == 0 { return perf_bpf_filter__parse(&mut (*evsel).bpf_filters, str_); }
    if evsel__append_addr_filter(evsel, str_) < 0 {
        fprintf(stderr, cstr(b"not enough memory to hold filter string\n\0"));
        return -1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn parse_filter(opt: *const option, str_: *const c_char, unset: c_int) -> c_int {
    let evlist = *((*opt).value as *mut *mut evlist);
    let _ = unset;
    foreach_evsel_in_last_glob(evlist, set_filter, str_ as *const c_void)
}

#[no_mangle]
pub unsafe extern "C" fn parse_uid_filter(evlist: *mut evlist, uid: uid_t) -> c_int {
    let mut opt = option { value: &evlist as *const _ as *mut c_void };
    let mut buf = [0 as c_char; 128];
    snprintf(buf.as_mut_ptr(), buf.len(), cstr(b"uid == %d\0"), uid);
    let ret = parse_filter(&mut opt, buf.as_ptr(), 0);
    if ret != 0 {
        if use_browser >= 1 { ui__warning(cstr(b"Failed to add UID filtering that uses BPF filtering.\n\0")); }
        else { fprintf(stderr, cstr(b"Failed to add UID filtering that uses BPF filtering.\n\0")); }
    }
    ret
}

unsafe fn add_exclude_perf_filter(evsel: *mut evsel, arg: *const c_void) -> c_int {
    let mut new_filter = [0 as c_char; 64];
    let _ = arg;
    if evsel.is_null() || (*evsel).core.attr.type_ != PERF_TYPE_TRACEPOINT {
        fprintf(stderr, cstr(b"--exclude-perf option should follow a -e tracepoint option\n\0"));
        return -1;
    }
    snprintf(new_filter.as_mut_ptr(), new_filter.len(), cstr(b"common_pid != %d\0"), getpid());
    if evsel__append_tp_filter(evsel, new_filter.as_ptr()) < 0 {
        fprintf(stderr, cstr(b"not enough memory to hold filter string\n\0"));
        return -1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn exclude_perf(opt: *const option, arg: *const c_char, unset: c_int) -> c_int {
    let evlist = *((*opt).value as *mut *mut evlist);
    let _ = (arg, unset);
    foreach_evsel_in_last_glob(evlist, add_exclude_perf_filter, null())
}

#[no_mangle]
pub unsafe extern "C" fn parse_events__is_hardcoded_term(term: *mut parse_events_term) -> c_int {
    ((*term).type_term != parse_events__term_type::PARSE_EVENTS__TERM_TYPE_USER) as c_int
}

unsafe fn new_term(_term: *mut *mut parse_events_term, temp: *mut parse_events_term, str_: *mut c_char, num: u64_t) -> c_int {
    let term = malloc(size_of::<parse_events_term>()) as *mut parse_events_term;
    if term.is_null() { return -ENOMEM; }
    core::ptr::copy_nonoverlapping(temp, term, 1);
    INIT_LIST_HEAD(&mut (*term).list);
    (*term).weak = false;
    match (*term).type_val {
        parse_events__term_val_type::PARSE_EVENTS__TERM_TYPE_NUM => (*term).val.num = num,
        parse_events__term_val_type::PARSE_EVENTS__TERM_TYPE_STR => (*term).val.str_ = str_,
    }
    *_term = term;
    0
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_term__num(term: *mut *mut parse_events_term, type_term: parse_events__term_type, config: *const c_char, num: u64_t, no_value: bool_t, loc_term_: *mut c_void, loc_val_: *mut c_void) -> c_int {
    let loc_term = loc_term_ as *mut YYLTYPE;
    let loc_val = loc_val_ as *mut YYLTYPE;
    let mut temp: parse_events_term = zeroed();
    temp.type_val = parse_events__term_val_type::PARSE_EVENTS__TERM_TYPE_NUM;
    temp.type_term = type_term;
    temp.config = if !config.is_null() { config as *mut c_char } else { strdup(parse_events__term_type_str(type_term)) };
    temp.no_value = no_value;
    temp.err_term = if !loc_term.is_null() { (*loc_term).first_column } else { 0 };
    temp.err_val = if !loc_val.is_null() { (*loc_val).first_column } else { 0 };
    new_term(term, &mut temp, null_mut(), num)
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_term__str(term: *mut *mut parse_events_term, type_term: parse_events__term_type, config: *mut c_char, str_: *mut c_char, loc_term_: *mut c_void, loc_val_: *mut c_void) -> c_int {
    let loc_term = loc_term_ as *mut YYLTYPE;
    let loc_val = loc_val_ as *mut YYLTYPE;
    let mut temp: parse_events_term = zeroed();
    temp.type_val = parse_events__term_val_type::PARSE_EVENTS__TERM_TYPE_STR;
    temp.type_term = type_term;
    temp.config = config;
    temp.err_term = if !loc_term.is_null() { (*loc_term).first_column } else { 0 };
    temp.err_val = if !loc_val.is_null() { (*loc_val).first_column } else { 0 };
    new_term(term, &mut temp, str_, 0)
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_term__term(term: *mut *mut parse_events_term, term_lhs: parse_events__term_type, term_rhs: parse_events__term_type, loc_term: *mut c_void, loc_val: *mut c_void) -> c_int {
    parse_events_term__str(term, term_lhs, null_mut(), strdup(parse_events__term_type_str(term_rhs)), loc_term, loc_val)
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_term__clone(new_: *mut *mut parse_events_term, term: *const parse_events_term) -> c_int {
    let mut temp: parse_events_term = core::ptr::read(term);
    temp.used = false;
    if !(*term).config.is_null() {
        temp.config = strdup((*term).config);
        if temp.config.is_null() { return -ENOMEM; }
    }
    if (*term).type_val == parse_events__term_val_type::PARSE_EVENTS__TERM_TYPE_NUM {
        return new_term(new_, &mut temp, null_mut(), (*term).val.num);
    }
    let str_ = strdup((*term).val.str_);
    if str_.is_null() {
        zfree_char(&mut temp.config);
        return -ENOMEM;
    }
    new_term(new_, &mut temp, str_, 0)
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_term__delete(term: *mut parse_events_term) {
    if (*term).type_val != parse_events__term_val_type::PARSE_EVENTS__TERM_TYPE_NUM {
        zfree_char(&mut (*term).val.str_);
    }
    zfree_char(&mut (*term).config);
    free(term as *mut c_void);
}

unsafe fn parse_events_terms__copy(src: *const parse_events_terms, dest: *mut parse_events_terms) -> c_int {
    // TODO(list_for_each_entry): clone every source term and append it to dest.
    let _ = (src, dest);
    0
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_terms__init(terms: *mut parse_events_terms) {
    INIT_LIST_HEAD(&mut (*terms).terms);
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_terms__exit(terms: *mut parse_events_terms) {
    // TODO(list_for_each_entry_safe): delete all terms.
    let _ = terms;
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_terms__delete(terms: *mut parse_events_terms) {
    if terms.is_null() { return; }
    parse_events_terms__exit(terms);
    free(terms as *mut c_void);
}

unsafe fn parse_events_terms__to_strbuf(terms: *const parse_events_terms, sb: *mut strbuf) -> c_int {
    if terms.is_null() { return 0; }
    // TODO(list_for_each_entry): format comma-separated config terms into strbuf.
    let _ = sb;
    0
}

unsafe fn config_terms_list(buf: *mut c_char, buf_sz: size_t) {
    *buf = 0;
    let mut first = true;
    let mut i = 0isize;
    while i < parse_events__term_type::__PARSE_EVENTS__TERM_TYPE_NR as isize {
        let term_type: parse_events__term_type = core::mem::transmute(i as c_int);
        let name = parse_events__term_type_str(term_type);
        if !config_term_avail(term_type, null_mut()) || name.is_null() || *name == b'<' as c_char {
            i += 1;
            continue;
        }
        if strlen(buf) + strlen(name) + 2 >= buf_sz { return; }
        if !first { strcat(buf, cstr(b",\0")); } else { first = false; }
        strcat(buf, name);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn parse_events_formats_error_string(additional_terms: *mut c_char) -> *mut c_char {
    let mut str_: *mut c_char = null_mut();
    let mut static_terms = [0 as c_char; parse_events__term_type::__PARSE_EVENTS__TERM_TYPE_NR as usize * ("no-overwrite".len())];
    config_terms_list(static_terms.as_mut_ptr(), static_terms.len());
    if !additional_terms.is_null() {
        if asprintf(&mut str_, cstr(b"valid terms: %s,%s\0"), additional_terms, static_terms.as_ptr()) < 0 { return null_mut(); }
    } else if asprintf(&mut str_, cstr(b"valid terms: %s\0"), static_terms.as_ptr()) < 0 {
        return null_mut();
    }
    str_
}

unsafe fn is_err(ptr: *mut c_void) -> bool_t {
    let v = ptr as isize;
    v < 0 && v >= -4095
}

unsafe fn ptr_err(ptr: *mut c_void) -> c_int {
    ptr as isize as c_int
}
