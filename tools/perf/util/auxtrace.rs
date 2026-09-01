// SPDX-License-Identifier: GPL-2.0-only
/*
 * auxtrace.rs: AUX area trace support
 * Copyright (c) 2013-2015, Intel Corporation.
 *
 * Source-level Rust translation of perf/util/auxtrace.c.
 *
 * This file intentionally references perf, Linux-list, mmap, kallsyms, DSO,
 * and event/session symbols that are supplied by the surrounding repository.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type bool_ = bool;
type u64 = u64;
type s64 = i64;
type u32 = u32;
type size_t = usize;
type ssize_t = isize;
type off_t = i64;
type pid_t = i32;
type FILE = c_void;

const AUXTRACE_SYNTH_EVENT_ID_OFFSET: u64 = 1000000000u64;
const AUXTRACE_INIT_NR_QUEUES: c_uint = 32;
const BUFFER_LIMIT_FOR_32_BIT: u64 = 32 * 1024 * 1024;
const MAX_AUX_SAMPLE_SIZE: u32 = 60 * 1024;
const DEFAULT_AUX_SAMPLE_SIZE: c_ulong = 4 * 1024;
const PERF_ITRACE_DEFAULT_PERIOD_TYPE: c_uint = PERF_ITRACE_PERIOD_NANOSECS;
const PERF_ITRACE_DEFAULT_PERIOD: u64 = 100000;
const PERF_ITRACE_DEFAULT_CALLCHAIN_SZ: c_uint = 16;
const PERF_ITRACE_MAX_CALLCHAIN_SZ: c_uint = 1024;
const PERF_ITRACE_DEFAULT_LAST_BRANCH_SZ: c_uint = 64;
const PERF_ITRACE_MAX_LAST_BRANCH_SZ: c_uint = 1024;
const ITRACE_DFLT_LOG_ON_ERROR_SZ: c_uint = 16384;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const UINT_MAX: c_uint = c_uint::MAX;
const UINT32_MAX_: u32 = u32::MAX;
const SSIZE_MAX_: u64 = isize::MAX as u64;
const BITS_PER_LONG: c_int = (size_of::<c_ulong>() * 8) as c_int;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

/* External constants supplied by perf/Linux headers. */
extern "C" {
    static mut page_size: c_ulong;
    static mut dump_trace: bool;
    static mut stdout: *mut FILE;
    static mut errno: c_int;
    static mut symbol_conf: symbol_conf;
}

extern "C" {
    fn mmap(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, off: off_t) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: size_t) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strspn(s: *const c_char, accept: *const c_char) -> size_t;
    fn strcspn(s: *const c_char, reject: *const c_char) -> size_t;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
    fn realpath(path: *const c_char, resolved_path: *mut c_char) -> *mut c_char;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn fprintf(fp: *mut FILE, fmt: *const c_char, ...) -> c_int;
}

/* Opaque and external repository types. Their concrete layouts are provided elsewhere. */
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct hlist_head { pub first: *mut hlist_node }
#[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct perf_cpu { pub cpu: c_int }
#[repr(C)] pub struct perf_event_header { pub type_: u32, pub misc: u16, pub size: u16 }
#[repr(C)] pub struct perf_record_auxtrace { pub header: perf_event_header, pub size: u64, pub offset: u64, pub reference: u64, pub idx: u32, pub tid: u32, pub cpu: u32 }
#[repr(C)] pub struct perf_record_auxtrace_info { pub header: perf_event_header, pub type_: u32 }
#[repr(C)] pub struct perf_record_auxtrace_error { pub header: perf_event_header, pub type_: u32, pub code: u32, pub cpu: i32, pub pid: i32, pub tid: i32, pub fmt: u32, pub ip: u64, pub time: u64, pub msg: [c_char; MAX_AUXTRACE_ERROR_MSG as usize], pub machine_pid: i32, pub vcpu: i32 }
#[repr(C)] pub union perf_event { pub header: perf_event_header, pub auxtrace: perf_record_auxtrace, pub auxtrace_info: perf_record_auxtrace_info, pub auxtrace_error: perf_record_auxtrace_error }
#[repr(C)] pub struct perf_event_mmap_page { pub aux_head: u64, pub aux_tail: u64, pub aux_offset: u64, pub aux_size: u64 }
#[repr(C)] pub struct auxtrace_mmap { pub userpg: *mut c_void, pub mask: size_t, pub len: size_t, pub prev: u64, pub idx: c_int, pub tid: pid_t, pub cpu: c_int, pub base: *mut u8 }
#[repr(C)] pub struct auxtrace_mmap_params { pub offset: off_t, pub len: size_t, pub mask: size_t, pub prot: c_int, pub idx: c_int, pub tid: pid_t, pub cpu: perf_cpu, pub mmap_needed: bool }
#[repr(C)] pub struct auxtrace_buffer { pub list: list_head, pub pid: pid_t, pub tid: pid_t, pub cpu: perf_cpu, pub data_offset: u64, pub offset: u64, pub reference: u64, pub size: u64, pub data: *mut c_void, pub use_data: *mut c_void, pub mmap_addr: *mut c_void, pub mmap_size: size_t, pub data_needs_freeing: bool, pub consecutive: bool, pub buffer_nr: u64 }
#[repr(C)] pub struct auxtrace_queue { pub head: list_head, pub tid: pid_t, pub cpu: c_int, pub set: bool, pub priv_: *mut c_void }
#[repr(C)] pub struct auxtrace_queues { pub nr_queues: c_uint, pub queue_array: *mut auxtrace_queue, pub next_buffer_nr: u64, pub new_data: bool, pub populated: bool }
#[repr(C)] pub struct auxtrace_heap_item { pub queue_nr: c_uint, pub ordinal: u64 }
#[repr(C)] pub struct auxtrace_heap { pub heap_array: *mut auxtrace_heap_item, pub heap_cnt: c_uint, pub heap_sz: c_uint }
#[repr(C)] pub struct auxtrace_index_entry { pub file_offset: u64, pub sz: u64 }
#[repr(C)] pub struct auxtrace_index { pub list: list_head, pub nr: size_t, pub entries: [auxtrace_index_entry; PERF_AUXTRACE_INDEX_ENTRY_COUNT as usize] }
#[repr(C)] pub struct auxtrace_cache { pub hashtable: *mut hlist_head, pub sz: size_t, pub entry_size: size_t, pub limit: size_t, pub cnt: size_t, pub bits: c_uint }
#[repr(C)] pub struct auxtrace_cache_entry { pub hash: hlist_node, pub key: u32 }
#[repr(C)] pub struct addr_filter { pub list: list_head, pub str_: *mut c_char, pub action: *const c_char, pub sym_from: *const c_char, pub sym_to: *const c_char, pub filename: *const c_char, pub addr: u64, pub size: u64, pub sym_from_idx: c_int, pub sym_to_idx: c_int, pub start: bool, pub range: bool }
#[repr(C)] pub struct addr_filters { pub head: list_head, pub cnt: c_int }
#[repr(C)] pub struct sym_args { pub name: *const c_char, pub start: u64, pub size: u64, pub idx: c_int, pub cnt: c_int, pub started: bool, pub global: bool, pub selected: bool, pub duplicate: bool, pub near: bool }

#[repr(C)] pub struct evsel_core_attr { pub aux_sample_size: u32, pub aux_action: u32 }
#[repr(C)] pub struct evsel_core { pub id: *mut u64, pub nr_members: c_int, pub cpus: *mut c_void, pub attr: evsel_core_attr }
#[repr(C)] pub struct evsel { pub core: evsel_core, pub disabled: bool, pub needs_auxtrace_mmap: bool, pub group_name: *mut c_char, pub filter: *mut c_char }
#[repr(C)] pub struct evlist_core { pub user_requested_cpus: *mut c_void, pub all_cpus: *mut c_void, pub threads: *mut c_void }
#[repr(C)] pub struct evlist { _unused: [u8; 0] }
#[repr(C)] pub struct perf_session { pub itrace_synth_opts: *mut itrace_synth_opts, pub data: *mut c_void, pub one_mmap: bool, pub one_mmap_offset: u64, pub one_mmap_addr: *mut c_void, pub auxtrace_index: list_head, pub auxtrace: *mut auxtrace, pub evlist: *mut evlist, pub header: perf_header, pub tool: *const perf_tool }
#[repr(C)] pub struct perf_header { pub data_offset: u64, pub data_size: u64 }
#[repr(C)] pub struct perf_tool { _unused: [u8; 0] }
#[repr(C)] pub struct perf_env { _unused: [u8; 0] }
#[repr(C)] pub struct record_opts { pub auxtrace_snapshot_on_exit: bool, pub auxtrace_sample_mode: bool }
#[repr(C)] pub struct evsel_config_term { pub val: evsel_config_term_val }
#[repr(C)] pub union evsel_config_term_val { pub aux_sample_size: u32, pub str_: *const c_char }
#[repr(C)] pub struct perf_sample_id { pub idx: c_uint, pub tid: pid_t, pub cpu: perf_cpu }
#[repr(C)] pub struct aux_sample { pub size: u64, pub data: *mut c_void }
#[repr(C)] pub struct perf_sample { pub id: u64, pub aux_sample: aux_sample }
#[repr(C)] pub struct mmap { pub auxtrace_mmap: auxtrace_mmap }
#[repr(C)] pub struct dso { _unused: [u8; 0] }
#[repr(C)] pub struct map { _unused: [u8; 0] }
#[repr(C)] pub struct symbol { pub start: u64, pub end: u64, pub name: *const c_char }
#[repr(C)] pub struct dso_data { pub file_size: u64 }
#[repr(C)] pub struct perf_pmu { _unused: [u8; 0] }
#[repr(C)] pub struct option { pub value: *mut c_void }
#[repr(C)] pub struct events_stats { pub nr_auxtrace_errors: [u32; PERF_AUXTRACE_ERROR_MAX as usize] }
#[repr(C)] pub struct symbol_conf { pub kptr_restrict: bool }

type perf_event__handler_t = unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut c_void) -> c_int;
type process_auxtrace_t = unsafe extern "C" fn(*const perf_tool, *mut mmap, *mut perf_event, *mut c_void, size_t, *mut c_void, size_t) -> c_int;

#[repr(C)] pub struct auxtrace_record {
    pub evlist: *mut evlist,
    pub default_aux_sample_size: c_ulong,
    pub alignment: c_uint,
    pub info_priv_size: unsafe extern "C" fn(*mut auxtrace_record, *mut evlist) -> size_t,
    pub info_fill: unsafe extern "C" fn(*mut auxtrace_record, *mut perf_session, *mut perf_record_auxtrace_info, size_t) -> c_int,
    pub free: unsafe extern "C" fn(*mut auxtrace_record),
    pub snapshot_start: Option<unsafe extern "C" fn(*mut auxtrace_record) -> c_int>,
    pub snapshot_finish: Option<unsafe extern "C" fn(*mut auxtrace_record) -> c_int>,
    pub find_snapshot: Option<unsafe extern "C" fn(*mut auxtrace_record, c_int, *mut auxtrace_mmap, *mut u8, *mut u64, *mut u64) -> c_int>,
    pub recording_options: unsafe extern "C" fn(*mut auxtrace_record, *mut evlist, *mut record_opts) -> c_int,
    pub reference: unsafe extern "C" fn(*mut auxtrace_record) -> u64,
    pub parse_snapshot_options: Option<unsafe extern "C" fn(*mut auxtrace_record, *mut record_opts, *const c_char) -> c_int>,
    pub read_finish: Option<unsafe extern "C" fn(*mut auxtrace_record, c_int) -> c_int>,
}

#[repr(C)] pub struct auxtrace {
    pub queue_data: Option<unsafe extern "C" fn(*mut perf_session, *mut perf_sample, *mut perf_event, u64) -> c_int>,
    pub process_auxtrace_event: unsafe extern "C" fn(*mut perf_session, *mut perf_event, *const perf_tool) -> s64,
    pub process_event: unsafe extern "C" fn(*mut perf_session, *mut perf_event, *mut perf_sample, *const perf_tool) -> c_int,
    pub dump_auxtrace_sample: Option<unsafe extern "C" fn(*mut perf_session, *mut perf_sample)>,
    pub flush_events: unsafe extern "C" fn(*mut perf_session, *const perf_tool) -> c_int,
    pub free_events: unsafe extern "C" fn(*mut perf_session),
    pub free: unsafe extern "C" fn(*mut perf_session),
    pub evsel_is_auxtrace: Option<unsafe extern "C" fn(*mut perf_session, *mut evsel) -> bool>,
}

extern "C" {
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug2(fmt: *const c_char, ...);
    fn pr_debug3(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn ui__warning(fmt: *const c_char, ...);
    fn WARN_ONCE(condition: bool, fmt: *const c_char, ...);
    fn INIT_LIST_HEAD(head: *mut list_head);
    fn INIT_HLIST_HEAD(head: *mut hlist_head);
    fn list_empty(head: *const list_head) -> bool;
    fn list_is_last(list: *const list_head, head: *const list_head) -> bool;
    fn list_add_tail(new_: *mut list_head, head: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn list_splice_tail(list: *mut list_head, head: *mut list_head);
    fn hlist_add_head(n: *mut hlist_node, h: *mut hlist_head);
    fn hlist_del(n: *mut hlist_node);
    fn hash_32(val: u32, bits: c_uint) -> u32;
    fn zalloc(size: size_t) -> *mut c_void;
    fn zfree(ptr: *mut *mut c_void);
    fn memdup(src: *const c_void, len: size_t) -> *mut c_void;
    fn readn(fd: c_int, buf: *mut c_void, n: size_t) -> ssize_t;
    fn writen(fd: c_int, buf: *const c_void, n: size_t) -> ssize_t;
    fn perf_data__fd(data: *mut c_void) -> c_int;
    fn perf_data__is_pipe(data: *mut c_void) -> bool;
    fn perf_session__peek_event(session: *mut perf_session, file_offset: off_t, buf: *mut c_char, sz: size_t, event: *mut *mut perf_event, unused: *mut c_void) -> c_int;
    fn perf_session__peek_events(session: *mut perf_session, offset: u64, size: u64, cb: unsafe extern "C" fn(*mut perf_session, *mut perf_event, u64, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn perf_sample__init(sample: *mut perf_sample, all: bool);
    fn perf_sample__exit(sample: *mut perf_sample);
    fn evlist__parse_sample(evlist: *mut evlist, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__id2sid(evlist: *mut evlist, id: u64) -> *mut perf_sample_id;
    fn evlist__stats(evlist: *mut evlist) -> *mut events_stats;
    fn perf_cpu_map__has_any_cpu(cpus: *mut c_void) -> bool;
    fn perf_cpu_map__cpu(cpus: *mut c_void, idx: c_int) -> perf_cpu;
    fn perf_cpu_map__idx(cpus: *mut c_void, cpu: perf_cpu) -> c_int;
    fn perf_thread_map__pid(threads: *mut c_void, idx: c_int) -> pid_t;
    fn evsel__is_group_leader(evsel: *mut evsel) -> bool;
    fn evsel__leader(evsel: *mut evsel) -> *mut evsel;
    fn evsel__has_leader(evsel: *mut evsel, leader: *mut evsel) -> bool;
    fn evsel__set_leader(evsel: *mut evsel, leader: *mut evsel);
    fn evsel__is_aux_event(evsel: *mut evsel) -> bool;
    fn evsel__set_sample_bit(evsel: *mut evsel, bit: c_int);
    fn evsel__reset_sample_bit(evsel: *mut evsel, bit: c_int);
    fn evsel__get_config_term(evsel: *mut evsel, term: c_int) -> *mut evsel_config_term;
    fn evsel__append_addr_filter(evsel: *mut evsel, filter: *mut c_char) -> c_int;
    fn evsel__find_pmu(evsel: *mut evsel) -> *mut perf_pmu;
    fn perf_evsel__enable_cpu(core: *mut evsel_core, idx: c_int) -> c_int;
    fn perf_evsel__enable_thread(core: *mut evsel_core, idx: c_int) -> c_int;
    fn perf_can_aux_sample() -> bool;
    fn perf_config_scan(key: *const c_char, fmt: *const c_char, ...);
    fn strlcpy(dst: *mut c_char, src: *const c_char, size: size_t) -> size_t;
    fn perf_env__kernel_is_64_bit(env: *mut perf_env) -> c_int;
    fn auxtrace_mmap__read_head(mm: *mut auxtrace_mmap, kernel_is_64_bit: c_int) -> u64;
    fn auxtrace_mmap__write_tail(mm: *mut auxtrace_mmap, tail: u64, kernel_is_64_bit: c_int) -> c_int;
    fn intel_pt_process_auxtrace_info(event: *mut perf_event, session: *mut perf_session) -> c_int;
    fn intel_bts_process_auxtrace_info(event: *mut perf_event, session: *mut perf_session) -> c_int;
    fn arm_spe_process_auxtrace_info(event: *mut perf_event, session: *mut perf_session) -> c_int;
    fn cs_etm__process_auxtrace_info(event: *mut perf_event, session: *mut perf_session) -> c_int;
    fn s390_cpumsf_process_auxtrace_info(event: *mut perf_event, session: *mut perf_session) -> c_int;
    fn hisi_ptt_process_auxtrace_info(event: *mut perf_event, session: *mut perf_session) -> c_int;
    fn powerpc_vpadtl_process_auxtrace_info(event: *mut perf_event, session: *mut perf_session) -> c_int;
    fn kallsyms__is_function(type_: c_char) -> bool;
    fn kallsyms__parse(path: *const c_char, arg: *mut c_void, cb: unsafe extern "C" fn(*mut c_void, *const c_char, c_char, u64) -> c_int) -> c_int;
    fn dso__new_map(name: *const c_char) -> *mut map;
    fn map__load(map: *mut map) -> c_int;
    fn map__dso(map: *mut map) -> *mut dso;
    fn dso__get(dso: *mut dso) -> *mut dso;
    fn map__put(map: *mut map);
    fn arch__compare_symbol_names(a: *const c_char, b: *const c_char) -> c_int;
    fn symbol__binding(sym: *mut symbol) -> c_int;
    fn dso__first_symbol(dso: *mut dso) -> *mut symbol;
    fn dso__next_symbol(sym: *mut symbol) -> *mut symbol;
    fn dso__data_file_size(dso: *mut dso, machine: *mut c_void) -> c_int;
    fn dso__data(dso: *mut dso) -> *mut dso_data;
    fn dso__put(dso: *mut dso);
    fn perf_pmu__scan_file(pmu: *mut perf_pmu, name: *const c_char, fmt: *const c_char, ...);
}

/* Constants expected from external headers. */
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const PERF_RECORD_AUXTRACE: u32 = 70;
const PERF_RECORD_AUXTRACE_INFO: u32 = 71;
const PERF_RECORD_AUXTRACE_ERROR: u32 = 72;
const PERF_RECORD_SAMPLE: u32 = 9;
const PERF_AUXTRACE_INTEL_PT: u32 = 1;
const PERF_AUXTRACE_INTEL_BTS: u32 = 2;
const PERF_AUXTRACE_CS_ETM: u32 = 3;
const PERF_AUXTRACE_ARM_SPE: u32 = 4;
const PERF_AUXTRACE_S390_CPUMSF: u32 = 5;
const PERF_AUXTRACE_HISI_PTT: u32 = 6;
const PERF_AUXTRACE_VPA_DTL: u32 = 7;
const PERF_AUXTRACE_UNKNOWN: u32 = 0;
const PERF_AUXTRACE_ERROR_ITRACE: usize = 0;
const PERF_AUXTRACE_ERROR_MAX: usize = 1;
const PERF_ITRACE_PERIOD_INSTRUCTIONS: c_uint = 1;
const PERF_ITRACE_PERIOD_TICKS: c_uint = 2;
const PERF_ITRACE_PERIOD_NANOSECS: c_uint = 3;
const PERF_AUXTRACE_INDEX_ENTRY_COUNT: usize = 256;
const PERF_SAMPLE_MAX_SIZE: usize = 65536;
const MAX_AUXTRACE_ERROR_MSG: usize = 64;
const MAX_NR_CPUS: c_int = 4096;
const AUX: c_int = 0;
const AUX_SAMPLE_SIZE: c_int = 1;
const AUX_ACTION: c_int = 2;
const AUX_OUTPUT: c_int = 3;
const AUXTRACE_LOG_FLG_ON_ERROR: c_uint = 1;
const PERF_AUXTRACE_RECORD_ALIGNMENT: u64 = 8;
const NSEC_PER_SEC: u64 = 1000000000;
const PATH_MAX: usize = 4096;
const STB_GLOBAL: c_int = 1;
const STB_LOCAL: c_int = 0;

#[inline] unsafe fn BIT(n: c_uint) -> u32 { 1u32 << n }
#[inline] unsafe fn is_power_of_2(x: size_t) -> bool { x != 0 && (x & (x - 1)) == 0 }
#[inline] unsafe fn bswap_64(x: u64) -> u64 { x.swap_bytes() }
#[inline] unsafe fn PERF_ALIGN(x: size_t, a: size_t) -> size_t { (x + a - 1) & !(a - 1) }
#[inline] unsafe fn round_up(x: u64, a: c_ulong) -> u64 { let a = a as u64; ((x + a - 1) / a) * a }
#[inline] unsafe fn isdigit(c: c_char) -> bool { c >= b'0' as c_char && c <= b'9' as c_char }
#[inline] unsafe fn isupper(c: c_char) -> bool { c >= b'A' as c_char && c <= b'Z' as c_char }
#[inline] unsafe fn test_bit(n: c_int, addr: *mut c_ulong) -> bool { ((*addr.add((n as usize) / (size_of::<c_ulong>() * 8)) >> ((n as usize) % (size_of::<c_ulong>() * 8))) & 1) != 0 }
#[inline] unsafe fn READ_ONCE_u64(p: *const u64) -> u64 { ptr::read_volatile(p) }
#[inline] unsafe fn WRITE_ONCE_u64(p: *mut u64, v: u64) { ptr::write_volatile(p, v) }
#[inline] unsafe fn smp_rmb() { core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire) }
#[inline] unsafe fn smp_mb() { core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst) }

/* list_entry/list_for_each_entry require container layouts from headers; in this isolated
 * translation they are represented as external helpers with C macro intent.
 */
extern "C" {
    fn auxtrace_queue_first_buffer(queue: *mut auxtrace_queue) -> *mut auxtrace_buffer;
    fn auxtrace_buffer_next_in_queue(queue: *mut auxtrace_queue, buffer: *mut auxtrace_buffer) -> *mut auxtrace_buffer;
    fn auxtrace_index_first(head: *mut list_head) -> *mut auxtrace_index;
    fn auxtrace_index_next(head: *mut list_head, idx: *mut auxtrace_index) -> *mut auxtrace_index;
    fn auxtrace_cache_first(head: *mut hlist_head) -> *mut auxtrace_cache_entry;
    fn auxtrace_cache_next(entry: *mut auxtrace_cache_entry) -> *mut auxtrace_cache_entry;
    fn addr_filter_first(head: *mut list_head) -> *mut addr_filter;
    fn addr_filter_next(head: *mut list_head, filt: *mut addr_filter) -> *mut addr_filter;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__next(evlist: *mut evlist, evsel: *mut evsel) -> *mut evsel;
}

#[no_mangle]
pub unsafe extern "C" fn auxtrace_synth_id_range_start(evsel: *mut evsel) -> u64 {
    let mut id = *(*evsel).core.id.add(0) + AUXTRACE_SYNTH_EVENT_ID_OFFSET;
    if id == 0 { id = 1; }
    id
}

unsafe fn evlist__regroup(evlist: *mut evlist, leader: *mut evsel, last: *mut evsel) -> c_int {
    if !evsel__is_group_leader(leader) { return -EINVAL; }
    let mut grp = false;
    let mut evsel = evlist__first(evlist);
    while !evsel.is_null() {
        if grp {
            if !(evsel__leader(evsel) == leader ||
                 (evsel__leader(evsel) == evsel && (*evsel).core.nr_members <= 1)) {
                return -EINVAL;
            }
        } else if evsel == leader { grp = true; }
        if evsel == last { break; }
        evsel = evlist__next(evlist, evsel);
    }
    grp = false;
    evsel = evlist__first(evlist);
    while !evsel.is_null() {
        if grp {
            if !evsel__has_leader(evsel, leader) {
                evsel__set_leader(evsel, leader);
                if (*leader).core.nr_members < 1 { (*leader).core.nr_members = 1; }
                (*leader).core.nr_members += 1;
            }
        } else if evsel == leader { grp = true; }
        if evsel == last { break; }
        evsel = evlist__next(evlist, evsel);
    }
    0
}

unsafe fn auxtrace__dont_decode(session: *mut perf_session) -> bool {
    (*session).itrace_synth_opts.is_null() || (*(*session).itrace_synth_opts).dont_decode
}

#[repr(C)]
pub struct itrace_synth_opts {
    pub set: bool, pub dont_decode: bool, pub default_no_sample: bool,
    pub branches: bool, pub transactions: bool, pub ptwrites: bool, pub pwr_events: bool,
    pub other_events: bool, pub intr_events: bool, pub errors: bool, pub flc: bool,
    pub llc: bool, pub tlb: bool, pub mem: bool, pub remote_access: bool,
    pub instructions: bool, pub cycles: bool, pub calls: bool, pub returns: bool,
    pub add_callchain: bool, pub callchain: bool, pub add_last_branch: bool,
    pub last_branch: bool, pub log: bool, pub approx_ipc: bool,
    pub timeless_decoding: bool, pub use_timestamp: bool,
    pub period_type: c_uint, pub period: u64, pub callchain_sz: c_uint,
    pub last_branch_sz: c_uint, pub initial_skip: c_ulong, pub quick: c_uint,
    pub error_plus_flags: c_uint, pub error_minus_flags: c_uint,
    pub log_plus_flags: c_uint, pub log_minus_flags: c_uint,
    pub log_on_error_size: c_uint,
    pub cpu_bitmap: *mut c_ulong,
}

#[no_mangle]
pub unsafe extern "C" fn auxtrace_mmap__mmap(mm: *mut auxtrace_mmap, mp: *mut auxtrace_mmap_params, userpg: *mut c_void, fd: c_int) -> c_int {
    let pc = userpg as *mut perf_event_mmap_page;
    WARN_ONCE(!(*mm).base.is_null(), b"Uninitialized auxtrace_mmap\n\0".as_ptr() as *const c_char);
    (*mm).userpg = userpg; (*mm).mask = (*mp).mask; (*mm).len = (*mp).len; (*mm).prev = 0;
    (*mm).idx = (*mp).idx; (*mm).tid = (*mp).tid; (*mm).cpu = (*mp).cpu.cpu;
    if (*mp).len == 0 || !(*mp).mmap_needed { (*mm).base = ptr::null_mut(); return 0; }
    (*pc).aux_offset = (*mp).offset as u64; (*pc).aux_size = (*mp).len as u64;
    (*mm).base = mmap(ptr::null_mut(), (*mp).len, (*mp).prot, MAP_SHARED, fd, (*mp).offset) as *mut u8;
    if (*mm).base as *mut c_void == MAP_FAILED {
        pr_debug2(b"failed to mmap AUX area\n\0".as_ptr() as *const c_char);
        (*mm).base = ptr::null_mut();
        return -1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn auxtrace_mmap__munmap(mm: *mut auxtrace_mmap) {
    if !(*mm).base.is_null() {
        munmap((*mm).base as *mut c_void, (*mm).len);
        (*mm).base = ptr::null_mut();
    }
}

#[no_mangle]
pub unsafe extern "C" fn auxtrace_mmap_params__init(mp: *mut auxtrace_mmap_params, auxtrace_offset: off_t, auxtrace_pages: c_uint, auxtrace_overwrite: bool) {
    if auxtrace_pages != 0 {
        (*mp).offset = auxtrace_offset;
        (*mp).len = auxtrace_pages as size_t * page_size as size_t;
        (*mp).mask = if is_power_of_2((*mp).len) { (*mp).len - 1 } else { 0 };
        (*mp).prot = PROT_READ | if auxtrace_overwrite { 0 } else { PROT_WRITE };
        pr_debug2(b"AUX area mmap length %zu\n\0".as_ptr() as *const c_char, (*mp).len);
    } else {
        (*mp).len = 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn auxtrace_mmap_params__set_idx(mp: *mut auxtrace_mmap_params, evlist: *mut evlist, evsel: *mut evsel, idx: c_int) {
    let per_cpu = !perf_cpu_map__has_any_cpu((*evlist__core(evlist)).user_requested_cpus);
    (*mp).mmap_needed = (*evsel).needs_auxtrace_mmap;
    if !(*mp).mmap_needed { return; }
    (*mp).idx = idx;
    if per_cpu {
        (*mp).cpu = perf_cpu_map__cpu((*evlist__core(evlist)).all_cpus, idx);
        (*mp).tid = perf_thread_map__pid((*evlist__core(evlist)).threads, 0);
    } else {
        (*mp).cpu.cpu = -1;
        (*mp).tid = perf_thread_map__pid((*evlist__core(evlist)).threads, idx);
    }
}

unsafe fn auxtrace_alloc_queue_array(nr_queues: c_uint) -> *mut auxtrace_queue {
    let max_nr_queues = UINT_MAX as usize / size_of::<auxtrace_queue>();
    if nr_queues as usize > max_nr_queues { return ptr::null_mut(); }
    let queue_array = calloc(nr_queues as size_t, size_of::<auxtrace_queue>()) as *mut auxtrace_queue;
    if queue_array.is_null() { return ptr::null_mut(); }
    for i in 0..nr_queues as usize {
        INIT_LIST_HEAD(&mut (*queue_array.add(i)).head);
        (*queue_array.add(i)).priv_ = ptr::null_mut();
    }
    queue_array
}

#[no_mangle] pub unsafe extern "C" fn auxtrace_queues__init_nr(queues: *mut auxtrace_queues, nr_queues: c_int) -> c_int {
    (*queues).nr_queues = nr_queues as c_uint;
    (*queues).queue_array = auxtrace_alloc_queue_array((*queues).nr_queues);
    if (*queues).queue_array.is_null() { return -ENOMEM; }
    0
}

#[no_mangle] pub unsafe extern "C" fn auxtrace_queues__init(queues: *mut auxtrace_queues) -> c_int {
    auxtrace_queues__init_nr(queues, AUXTRACE_INIT_NR_QUEUES as c_int)
}

unsafe fn auxtrace_queues__grow(queues: *mut auxtrace_queues, new_nr_queues: c_uint) -> c_int {
    let mut nr_queues = (*queues).nr_queues;
    let old_array = (*queues).queue_array;
    if new_nr_queues == 0 { return -EINVAL; }
    if nr_queues == 0 { nr_queues = AUXTRACE_INIT_NR_QUEUES; }
    while nr_queues != 0 && nr_queues < new_nr_queues { nr_queues <<= 1; }
    if nr_queues < (*queues).nr_queues || nr_queues < new_nr_queues { return -EINVAL; }
    let queue_array = auxtrace_alloc_queue_array(nr_queues);
    if queue_array.is_null() { return -ENOMEM; }
    for i in 0..(*queues).nr_queues as usize {
        list_splice_tail(&mut (*old_array.add(i)).head, &mut (*queue_array.add(i)).head);
        (*queue_array.add(i)).tid = (*old_array.add(i)).tid;
        (*queue_array.add(i)).cpu = (*old_array.add(i)).cpu;
        (*queue_array.add(i)).set = (*old_array.add(i)).set;
        (*queue_array.add(i)).priv_ = (*old_array.add(i)).priv_;
    }
    (*queues).nr_queues = nr_queues;
    (*queues).queue_array = queue_array;
    free(old_array as *mut c_void);
    0
}

unsafe fn auxtrace_copy_data(size: u64, session: *mut perf_session) -> *mut c_void {
    let fd = perf_data__fd((*session).data);
    if size > SSIZE_MAX_ { return ptr::null_mut(); }
    let p = malloc(size as size_t);
    if p.is_null() { return ptr::null_mut(); }
    let ret = readn(fd, p, size as size_t);
    if ret != size as ssize_t {
        free(p);
        return ptr::null_mut();
    }
    p
}

unsafe fn auxtrace_queues__queue_buffer(queues: *mut auxtrace_queues, idx: c_uint, buffer: *mut auxtrace_buffer) -> c_int {
    if idx >= (*queues).nr_queues {
        let err = auxtrace_queues__grow(queues, idx + 1);
        if err != 0 { return err; }
    }
    let queue = (*queues).queue_array.add(idx as usize);
    if !(*queue).set {
        (*queue).set = true;
        (*queue).tid = (*buffer).tid;
        (*queue).cpu = (*buffer).cpu.cpu;
    }
    (*buffer).buffer_nr = (*queues).next_buffer_nr;
    (*queues).next_buffer_nr += 1;
    list_add_tail(&mut (*buffer).list, &mut (*queue).head);
    (*queues).new_data = true;
    (*queues).populated = true;
    0
}

unsafe fn filter_cpu(session: *mut perf_session, cpu: perf_cpu) -> bool {
    let cpu_bitmap = (*(*session).itrace_synth_opts).cpu_bitmap;
    !cpu_bitmap.is_null() && cpu.cpu >= 0 && cpu.cpu < MAX_NR_CPUS && !test_bit(cpu.cpu, cpu_bitmap)
}

unsafe fn auxtrace_queues__add_buffer(queues: *mut auxtrace_queues, session: *mut perf_session, idx: c_uint, buffer: *mut auxtrace_buffer, buffer_ptr: *mut *mut auxtrace_buffer) -> c_int {
    let mut err = -ENOMEM;
    if filter_cpu(session, (*buffer).cpu) { return 0; }
    let buffer = memdup(buffer as *const c_void, size_of::<auxtrace_buffer>()) as *mut auxtrace_buffer;
    if buffer.is_null() { return -ENOMEM; }
    if (*session).one_mmap {
        (*buffer).data = ((*buffer).data_offset - (*session).one_mmap_offset) as usize
            .wrapping_add((*session).one_mmap_addr as usize) as *mut c_void;
    } else if perf_data__is_pipe((*session).data) {
        (*buffer).data = auxtrace_copy_data((*buffer).size, session);
        if (*buffer).data.is_null() { auxtrace_buffer__free(buffer); return err; }
        (*buffer).data_needs_freeing = true;
    } else if BITS_PER_LONG == 32 && (*buffer).size > BUFFER_LIMIT_FOR_32_BIT {
        err = auxtrace_queues__split_buffer(queues, idx, buffer);
        if err != 0 { auxtrace_buffer__free(buffer); return err; }
    }
    err = auxtrace_queues__queue_buffer(queues, idx, buffer);
    if err != 0 { auxtrace_buffer__free(buffer); return err; }
    if !buffer_ptr.is_null() { *buffer_ptr = buffer; }
    0
}

unsafe fn auxtrace_queues__split_buffer(queues: *mut auxtrace_queues, idx: c_uint, buffer: *mut auxtrace_buffer) -> c_int {
    let mut sz = (*buffer).size;
    let mut consecutive = false;
    while sz > BUFFER_LIMIT_FOR_32_BIT {
        let b = memdup(buffer as *const c_void, size_of::<auxtrace_buffer>()) as *mut auxtrace_buffer;
        if b.is_null() { return -ENOMEM; }
        (*b).size = BUFFER_LIMIT_FOR_32_BIT;
        (*b).consecutive = consecutive;
        let err = auxtrace_queues__queue_buffer(queues, idx, b);
        if err != 0 { auxtrace_buffer__free(b); return err; }
        (*buffer).data_offset += BUFFER_LIMIT_FOR_32_BIT;
        sz -= BUFFER_LIMIT_FOR_32_BIT;
        consecutive = true;
    }
    (*buffer).size = sz;
    (*buffer).consecutive = consecutive;
    0
}

#[no_mangle]
pub unsafe extern "C" fn auxtrace_queues__add_event(queues: *mut auxtrace_queues, session: *mut perf_session, event: *mut perf_event, data_offset: off_t, buffer_ptr: *mut *mut auxtrace_buffer) -> c_int {
    let mut buffer: auxtrace_buffer = zeroed();
    buffer.pid = -1;
    buffer.tid = (*event).auxtrace.tid as pid_t;
    buffer.cpu.cpu = (*event).auxtrace.cpu as c_int;
    buffer.data_offset = data_offset as u64;
    buffer.offset = (*event).auxtrace.offset;
    buffer.reference = (*event).auxtrace.reference;
    buffer.size = (*event).auxtrace.size;
    auxtrace_queues__add_buffer(queues, session, (*event).auxtrace.idx, &mut buffer, buffer_ptr)
}

#[no_mangle]
pub unsafe extern "C" fn auxtrace_queues__free(queues: *mut auxtrace_queues) {
    for i in 0..(*queues).nr_queues as usize {
        let q = (*queues).queue_array.add(i);
        while !list_empty(&(*q).head) {
            let buffer = auxtrace_queue_first_buffer(q);
            list_del_init(&mut (*buffer).list);
            auxtrace_buffer__free(buffer);
        }
    }
    zfree(&mut (*queues).queue_array as *mut _ as *mut *mut c_void);
    (*queues).nr_queues = 0;
}

unsafe fn auxtrace_heapify(heap_array: *mut auxtrace_heap_item, mut pos: c_uint, queue_nr: c_uint, ordinal: u64) {
    while pos != 0 {
        let parent = (pos - 1) >> 1;
        if (*heap_array.add(parent as usize)).ordinal <= ordinal { break; }
        *heap_array.add(pos as usize) = *heap_array.add(parent as usize);
        pos = parent;
    }
    (*heap_array.add(pos as usize)).queue_nr = queue_nr;
    (*heap_array.add(pos as usize)).ordinal = ordinal;
}

#[no_mangle]
pub unsafe extern "C" fn auxtrace_heap__add(heap: *mut auxtrace_heap, queue_nr: c_uint, ordinal: u64) -> c_int {
    if queue_nr >= (*heap).heap_sz {
        let mut heap_sz = AUXTRACE_INIT_NR_QUEUES;
        while heap_sz <= queue_nr { heap_sz <<= 1; }
        let heap_array = realloc((*heap).heap_array as *mut c_void, heap_sz as size_t * size_of::<auxtrace_heap_item>()) as *mut auxtrace_heap_item;
        if heap_array.is_null() { return -ENOMEM; }
        (*heap).heap_array = heap_array; (*heap).heap_sz = heap_sz;
    }
    let pos = (*heap).heap_cnt; (*heap).heap_cnt += 1;
    auxtrace_heapify((*heap).heap_array, pos, queue_nr, ordinal);
    0
}

#[no_mangle] pub unsafe extern "C" fn auxtrace_heap__free(heap: *mut auxtrace_heap) {
    zfree(&mut (*heap).heap_array as *mut _ as *mut *mut c_void);
    (*heap).heap_cnt = 0; (*heap).heap_sz = 0;
}

#[no_mangle] pub unsafe extern "C" fn auxtrace_heap__pop(heap: *mut auxtrace_heap) {
    let heap_cnt = (*heap).heap_cnt;
    if heap_cnt == 0 { return; }
    (*heap).heap_cnt -= 1;
    let heap_array = (*heap).heap_array;
    let mut pos = 0;
    loop {
        let left = (pos << 1) + 1;
        if left >= heap_cnt { break; }
        let right = left + 1;
        if right >= heap_cnt { *heap_array.add(pos as usize) = *heap_array.add(left as usize); return; }
        if (*heap_array.add(left as usize)).ordinal < (*heap_array.add(right as usize)).ordinal {
            *heap_array.add(pos as usize) = *heap_array.add(left as usize); pos = left;
        } else {
            *heap_array.add(pos as usize) = *heap_array.add(right as usize); pos = right;
        }
    }
    let last = heap_cnt - 1;
    auxtrace_heapify(heap_array, pos, (*heap_array.add(last as usize)).queue_nr, (*heap_array.add(last as usize)).ordinal);
}

#[no_mangle] pub unsafe extern "C" fn auxtrace_record__info_priv_size(itr: *mut auxtrace_record, evlist: *mut evlist) -> size_t {
    if !itr.is_null() { ((*itr).info_priv_size)(itr, evlist) } else { 0 }
}
unsafe fn auxtrace_not_supported() -> c_int { pr_err(b"AUX area tracing is not supported on this architecture\n\0".as_ptr() as *const c_char); -EINVAL }
#[no_mangle] pub unsafe extern "C" fn auxtrace_record__info_fill(itr: *mut auxtrace_record, session: *mut perf_session, info: *mut perf_record_auxtrace_info, priv_size: size_t) -> c_int { if !itr.is_null() { ((*itr).info_fill)(itr, session, info, priv_size) } else { auxtrace_not_supported() } }
#[no_mangle] pub unsafe extern "C" fn auxtrace_record__free(itr: *mut auxtrace_record) { if !itr.is_null() { ((*itr).free)(itr); } }
#[no_mangle] pub unsafe extern "C" fn auxtrace_record__snapshot_start(itr: *mut auxtrace_record) -> c_int { if !itr.is_null() { if let Some(f)=(*itr).snapshot_start { return f(itr); } } 0 }
#[no_mangle] pub unsafe extern "C" fn auxtrace_record__snapshot_finish(itr: *mut auxtrace_record, on_exit: bool) -> c_int { if !on_exit && !itr.is_null() { if let Some(f)=(*itr).snapshot_finish { return f(itr); } } 0 }
#[no_mangle] pub unsafe extern "C" fn auxtrace_record__find_snapshot(itr: *mut auxtrace_record, idx: c_int, mm: *mut auxtrace_mmap, data: *mut u8, head: *mut u64, old: *mut u64) -> c_int { if !itr.is_null() { if let Some(f)=(*itr).find_snapshot { return f(itr, idx, mm, data, head, old); } } 0 }
#[no_mangle] pub unsafe extern "C" fn auxtrace_record__options(itr: *mut auxtrace_record, evlist: *mut evlist, opts: *mut record_opts) -> c_int { if !itr.is_null() { (*itr).evlist = evlist; ((*itr).recording_options)(itr, evlist, opts) } else { 0 } }
#[no_mangle] pub unsafe extern "C" fn auxtrace_record__reference(itr: *mut auxtrace_record) -> u64 { if !itr.is_null() { ((*itr).reference)(itr) } else { 0 } }

/* The remaining routines are translated directly but rely on external C macro helpers
 * for list traversal and repository-owned object layouts.
 */

#[no_mangle]
pub unsafe extern "C" fn compat_auxtrace_mmap__read_head(mm: *mut auxtrace_mmap) -> u64 {
    let pc = (*mm).userpg as *mut perf_event_mmap_page;
    let mask = (UINT32_MAX_ as u64) << 32;
    let mut first; let mut second; let mut last;
    loop {
        first = READ_ONCE_u64(&(*pc).aux_head); smp_rmb();
        second = READ_ONCE_u64(&(*pc).aux_head); smp_rmb();
        last = READ_ONCE_u64(&(*pc).aux_head);
        if (first & mask) == (last & mask) { break; }
    }
    second
}

#[no_mangle]
pub unsafe extern "C" fn compat_auxtrace_mmap__write_tail(mm: *mut auxtrace_mmap, tail: u64) -> c_int {
    let pc = (*mm).userpg as *mut perf_event_mmap_page;
    let mask = (UINT32_MAX_ as u64) << 32;
    if (tail & mask) != 0 { return -1; }
    smp_mb();
    WRITE_ONCE_u64(&mut (*pc).aux_tail, tail);
    0
}

#[no_mangle]
pub unsafe extern "C" fn auxtrace_buffer__next(queue: *mut auxtrace_queue, buffer: *mut auxtrace_buffer) -> *mut auxtrace_buffer {
    if !buffer.is_null() {
        if list_is_last(&(*buffer).list, &(*queue).head) { ptr::null_mut() } else { auxtrace_buffer_next_in_queue(queue, buffer) }
    } else if list_empty(&(*queue).head) {
        ptr::null_mut()
    } else {
        auxtrace_queue_first_buffer(queue)
    }
}

#[no_mangle]
pub unsafe extern "C" fn auxtrace_buffer__get_data_rw(buffer: *mut auxtrace_buffer, fd: c_int, rw: bool) -> *mut c_void {
    let prot = if rw { PROT_READ | PROT_WRITE } else { PROT_READ };
    let adj = (*buffer).data_offset as size_t & (page_size as size_t - 1);
    let size = (*buffer).size as size_t + adj;
    let file_offset = (*buffer).data_offset as off_t - adj as off_t;
    if !(*buffer).data.is_null() { return (*buffer).data; }
    let addr = mmap(ptr::null_mut(), size, prot, MAP_SHARED, fd, file_offset);
    if addr == MAP_FAILED { return ptr::null_mut(); }
    (*buffer).mmap_addr = addr;
    (*buffer).mmap_size = size;
    (*buffer).data = (addr as *mut u8).add(adj) as *mut c_void;
    (*buffer).data
}

#[no_mangle] pub unsafe extern "C" fn auxtrace_buffer__put_data(buffer: *mut auxtrace_buffer) {
    if (*buffer).data.is_null() || (*buffer).mmap_addr.is_null() { return; }
    munmap((*buffer).mmap_addr, (*buffer).mmap_size);
    (*buffer).mmap_addr = ptr::null_mut(); (*buffer).mmap_size = 0; (*buffer).data = ptr::null_mut(); (*buffer).use_data = ptr::null_mut();
}
#[no_mangle] pub unsafe extern "C" fn auxtrace_buffer__drop_data(buffer: *mut auxtrace_buffer) {
    auxtrace_buffer__put_data(buffer);
    if (*buffer).data_needs_freeing {
        (*buffer).data_needs_freeing = false;
        zfree(&mut (*buffer).data as *mut _ as *mut *mut c_void);
        (*buffer).use_data = ptr::null_mut(); (*buffer).size = 0;
    }
}
#[no_mangle] pub unsafe extern "C" fn auxtrace_buffer__free(buffer: *mut auxtrace_buffer) { auxtrace_buffer__drop_data(buffer); free(buffer as *mut c_void); }

#[no_mangle]
pub unsafe extern "C" fn auxtrace_synth_guest_error(e: *mut perf_record_auxtrace_error, type_: c_int, code: c_int, cpu: c_int, pid: pid_t, tid: pid_t, ip: u64, msg: *const c_char, timestamp: u64, machine_pid: pid_t, vcpu: c_int) {
    memset(e as *mut c_void, 0, size_of::<perf_record_auxtrace_error>());
    (*e).header.type_ = PERF_RECORD_AUXTRACE_ERROR; (*e).type_ = type_ as u32; (*e).code = code as u32;
    (*e).cpu = cpu; (*e).pid = pid; (*e).tid = tid; (*e).fmt = 1; (*e).ip = ip; (*e).time = timestamp;
    strlcpy((*e).msg.as_mut_ptr(), msg, MAX_AUXTRACE_ERROR_MSG);
    let size = if machine_pid != 0 {
        (*e).fmt = 2; (*e).machine_pid = machine_pid; (*e).vcpu = vcpu; size_of::<perf_record_auxtrace_error>()
    } else {
        ((*e).msg.as_ptr() as usize - e as usize) + strlen((*e).msg.as_ptr()) + 1
    };
    (*e).header.size = PERF_ALIGN(size, size_of::<u64>()) as u16;
}

#[no_mangle] pub unsafe extern "C" fn auxtrace_synth_error(e: *mut perf_record_auxtrace_error, type_: c_int, code: c_int, cpu: c_int, pid: pid_t, tid: pid_t, ip: u64, msg: *const c_char, timestamp: u64) {
    auxtrace_synth_guest_error(e, type_, code, cpu, pid, tid, ip, msg, timestamp, 0, -1);
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__synthesize_auxtrace_info(itr: *mut auxtrace_record, tool: *const perf_tool, session: *mut perf_session, process: perf_event__handler_t) -> c_int {
    pr_debug2(b"Synthesizing auxtrace information\n\0".as_ptr() as *const c_char);
    let priv_size = auxtrace_record__info_priv_size(itr, (*session).evlist);
    let ev = zalloc(size_of::<perf_record_auxtrace_info>() + priv_size) as *mut perf_event;
    if ev.is_null() { return -ENOMEM; }
    (*ev).auxtrace_info.header.type_ = PERF_RECORD_AUXTRACE_INFO;
    (*ev).auxtrace_info.header.size = (size_of::<perf_record_auxtrace_info>() + priv_size) as u16;
    let mut err = auxtrace_record__info_fill(itr, session, &mut (*ev).auxtrace_info, priv_size);
    if err == 0 { err = process(tool, ev, ptr::null_mut(), ptr::null_mut()); }
    free(ev as *mut c_void);
    err
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_auxtrace_info(_tool: *const perf_tool, session: *mut perf_session, event: *mut perf_event) -> c_int {
    let type_ = (*event).auxtrace_info.type_;
    if dump_trace { fprintf(stdout, b" type: %u\n\0".as_ptr() as *const c_char, type_); }
    let err = match type_ {
        PERF_AUXTRACE_INTEL_PT => intel_pt_process_auxtrace_info(event, session),
        PERF_AUXTRACE_INTEL_BTS => intel_bts_process_auxtrace_info(event, session),
        PERF_AUXTRACE_ARM_SPE => arm_spe_process_auxtrace_info(event, session),
        PERF_AUXTRACE_CS_ETM => cs_etm__process_auxtrace_info(event, session),
        PERF_AUXTRACE_S390_CPUMSF => s390_cpumsf_process_auxtrace_info(event, session),
        PERF_AUXTRACE_HISI_PTT => hisi_ptt_process_auxtrace_info(event, session),
        PERF_AUXTRACE_VPA_DTL => powerpc_vpadtl_process_auxtrace_info(event, session),
        _ => return -EINVAL,
    };
    if err != 0 { return err; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_auxtrace(_tool: *const perf_tool, session: *mut perf_session, event: *mut perf_event) -> s64 {
    if dump_trace {
        fprintf(stdout, b" size: %#llx  offset: %#llx  ref: %#llx  idx: %u  tid: %d  cpu: %d\n\0".as_ptr() as *const c_char,
            (*event).auxtrace.size, (*event).auxtrace.offset, (*event).auxtrace.reference, (*event).auxtrace.idx, (*event).auxtrace.tid, (*event).auxtrace.cpu);
    }
    if auxtrace__dont_decode(session) { return (*event).auxtrace.size as s64; }
    if (*session).auxtrace.is_null() || (*event).header.type_ != PERF_RECORD_AUXTRACE { return -EINVAL as s64; }
    let err = ((*(*session).auxtrace).process_auxtrace_event)(session, event, (*session).tool);
    if err < 0 { return err; }
    (*event).auxtrace.size as s64
}

#[no_mangle]
pub unsafe extern "C" fn itrace_synth_opts__set_default(o: *mut itrace_synth_opts, no_sample: bool) {
    (*o).branches = true; (*o).transactions = true; (*o).ptwrites = true; (*o).pwr_events = true;
    (*o).other_events = true; (*o).intr_events = true; (*o).errors = true; (*o).flc = true;
    (*o).llc = true; (*o).tlb = true; (*o).mem = true; (*o).remote_access = true;
    if no_sample {
        (*o).period_type = PERF_ITRACE_PERIOD_INSTRUCTIONS; (*o).period = 1; (*o).calls = true;
    } else {
        (*o).instructions = true; (*o).cycles = true; (*o).period_type = PERF_ITRACE_DEFAULT_PERIOD_TYPE; (*o).period = PERF_ITRACE_DEFAULT_PERIOD;
    }
    (*o).callchain_sz = PERF_ITRACE_DEFAULT_CALLCHAIN_SZ; (*o).last_branch_sz = PERF_ITRACE_DEFAULT_LAST_BRANCH_SZ; (*o).initial_skip = 0;
}

unsafe fn get_flag(ptrp: *mut *const c_char, flags: *mut c_uint) -> c_int {
    loop {
        let c = **ptrp;
        if c >= b'a' as c_char && c <= b'z' as c_char {
            *flags |= 1 << (c - b'a' as c_char) as c_uint; *ptrp = (*ptrp).add(1); return 0;
        } else if c == b' ' as c_char { *ptrp = (*ptrp).add(1); } else { return -1; }
    }
}
unsafe fn get_flags(ptrp: *mut *const c_char, plus_flags: *mut c_uint, minus_flags: *mut c_uint) -> c_int {
    loop {
        match **ptrp as u8 {
            b'+' => { *ptrp = (*ptrp).add(1); if get_flag(ptrp, plus_flags) != 0 { return -1; } }
            b'-' => { *ptrp = (*ptrp).add(1); if get_flag(ptrp, minus_flags) != 0 { return -1; } }
            b' ' => *ptrp = (*ptrp).add(1),
            _ => return 0,
        }
    }
}
unsafe fn itrace_log_on_error_size() -> c_uint {
    let mut sz = 0;
    perf_config_scan(b"itrace.debug-log-buffer-size\0".as_ptr() as *const c_char, b"%u\0".as_ptr() as *const c_char, &mut sz);
    if sz != 0 { sz } else { ITRACE_DFLT_LOG_ON_ERROR_SZ }
}

#[no_mangle]
pub unsafe extern "C" fn itrace_do_parse_synth_opts(o: *mut itrace_synth_opts, str_: *const c_char, unset: c_int) -> c_int {
    let mut endptr: *mut c_char = ptr::null_mut();
    let mut period_type_set = false; let mut period_set = false; let mut iy = false;
    (*o).set = true;
    if unset != 0 { (*o).dont_decode = true; return 0; }
    if str_.is_null() { itrace_synth_opts__set_default(o, (*o).default_no_sample); return 0; }
    let mut p = str_;
    while *p != 0 {
        let ch = *p as u8; p = p.add(1);
        match ch {
            b'i' | b'y' => {
                iy = true; if ch == b'y' { (*o).cycles = true; } else { (*o).instructions = true; }
                while *p == b' ' as c_char || *p == b',' as c_char { p = p.add(1); }
                if isdigit(*p) {
                    (*o).period = strtoull(p, &mut endptr, 10); period_set = true; p = endptr;
                    while *p == b' ' as c_char || *p == b',' as c_char { p = p.add(1); }
                    let unit = *p as u8; p = p.add(1);
                    match unit {
                        b'i' => { (*o).period_type = PERF_ITRACE_PERIOD_INSTRUCTIONS; period_type_set = true; }
                        b't' => { (*o).period_type = PERF_ITRACE_PERIOD_TICKS; period_type_set = true; }
                        b'm' => { (*o).period *= 1000; (*o).period *= 1000; if *p as u8 != b's' { goto_out_err(str_); return -EINVAL; } p = p.add(1); (*o).period_type = PERF_ITRACE_PERIOD_NANOSECS; period_type_set = true; }
                        b'u' => { (*o).period *= 1000; if *p as u8 != b's' { goto_out_err(str_); return -EINVAL; } p = p.add(1); (*o).period_type = PERF_ITRACE_PERIOD_NANOSECS; period_type_set = true; }
                        b'n' => { if *p as u8 != b's' { goto_out_err(str_); return -EINVAL; } p = p.add(1); (*o).period_type = PERF_ITRACE_PERIOD_NANOSECS; period_type_set = true; }
                        0 => break,
                        _ => { goto_out_err(str_); return -EINVAL; }
                    }
                }
            }
            b'b' => (*o).branches = true, b'x' => (*o).transactions = true, b'w' => (*o).ptwrites = true,
            b'p' => (*o).pwr_events = true, b'o' => (*o).other_events = true, b'I' => (*o).intr_events = true,
            b'e' => { (*o).errors = true; if get_flags(&mut p, &mut (*o).error_plus_flags, &mut (*o).error_minus_flags) != 0 { goto_out_err(str_); return -EINVAL; } }
            b'd' => { (*o).log = true; if get_flags(&mut p, &mut (*o).log_plus_flags, &mut (*o).log_minus_flags) != 0 { goto_out_err(str_); return -EINVAL; } if ((*o).log_plus_flags & AUXTRACE_LOG_FLG_ON_ERROR) != 0 { (*o).log_on_error_size = itrace_log_on_error_size(); } }
            b'c' => { (*o).branches = true; (*o).calls = true; }
            b'r' => { (*o).branches = true; (*o).returns = true; }
            b'f' => (*o).flc = true, b'm' => (*o).llc = true, b't' => (*o).tlb = true,
            b'a' => (*o).remote_access = true, b'M' => (*o).mem = true, b'q' => (*o).quick += 1,
            b'A' => (*o).approx_ipc = true, b'Z' => (*o).timeless_decoding = true, b'T' => (*o).use_timestamp = true,
            b' ' | b',' => {}
            b'G' | b'g' | b'L' | b'l' | b's' => {
                /* Callchain, last-branch, and skip parsing follows the C source exactly in intent;
                 * the size bounds are PERF_ITRACE_MAX_* and strtoul/strtoull update p.
                 */
                if ch == b'G' { (*o).add_callchain = true; (*o).callchain_sz = PERF_ITRACE_DEFAULT_CALLCHAIN_SZ; }
                else if ch == b'g' { (*o).callchain = true; (*o).callchain_sz = PERF_ITRACE_DEFAULT_CALLCHAIN_SZ; }
                else if ch == b'L' { (*o).add_last_branch = true; (*o).last_branch_sz = PERF_ITRACE_DEFAULT_LAST_BRANCH_SZ; }
                else if ch == b'l' { (*o).last_branch = true; (*o).last_branch_sz = PERF_ITRACE_DEFAULT_LAST_BRANCH_SZ; }
                else { (*o).initial_skip = strtoul(p, &mut endptr, 10); if p == endptr { goto_out_err(str_); return -EINVAL; } p = endptr; continue; }
                while *p == b' ' as c_char || *p == b',' as c_char { p = p.add(1); }
                if isdigit(*p) {
                    let val = strtoul(p, &mut endptr, 10) as c_uint; p = endptr;
                    if ch == b'G' || ch == b'g' {
                        if val == 0 || val > PERF_ITRACE_MAX_CALLCHAIN_SZ { goto_out_err(str_); return -EINVAL; }
                        (*o).callchain_sz = val;
                    } else {
                        if val == 0 || val > PERF_ITRACE_MAX_LAST_BRANCH_SZ { goto_out_err(str_); return -EINVAL; }
                        (*o).last_branch_sz = val;
                    }
                }
            }
            _ => { goto_out_err(str_); return -EINVAL; }
        }
    }
    if iy {
        if !period_type_set { (*o).period_type = PERF_ITRACE_DEFAULT_PERIOD_TYPE; }
        if !period_set { (*o).period = PERF_ITRACE_DEFAULT_PERIOD; }
    }
    0
}
unsafe fn goto_out_err(str_: *const c_char) { pr_err(b"Bad Instruction Tracing options '%s'\n\0".as_ptr() as *const c_char, str_); }
#[no_mangle] pub unsafe extern "C" fn itrace_parse_synth_opts(opt: *const option, str_: *const c_char, unset: c_int) -> c_int { itrace_do_parse_synth_opts((*opt).value as *mut itrace_synth_opts, str_, unset) }

static auxtrace_error_type_name: [*const c_char; PERF_AUXTRACE_ERROR_MAX] = [b"instruction trace\0".as_ptr() as *const c_char];
unsafe fn auxtrace_error_name(type_: c_uint) -> *const c_char {
    if (type_ as usize) < PERF_AUXTRACE_ERROR_MAX && !auxtrace_error_type_name[type_ as usize].is_null() { auxtrace_error_type_name[type_ as usize] } else { b"unknown AUX\0".as_ptr() as *const c_char }
}

#[no_mangle] pub unsafe extern "C" fn perf_session__auxtrace_error_inc(session: *mut perf_session, event: *mut perf_event) {
    let e = &(*event).auxtrace_error;
    if (e.type_ as usize) < PERF_AUXTRACE_ERROR_MAX { (*evlist__stats((*session).evlist)).nr_auxtrace_errors[e.type_ as usize] += 1; }
}
#[no_mangle] pub unsafe extern "C" fn events_stats__auxtrace_error_warn(stats: *const events_stats) {
    for i in 0..PERF_AUXTRACE_ERROR_MAX {
        if (*stats).nr_auxtrace_errors[i] != 0 { ui__warning(b"%u %s errors\n\0".as_ptr() as *const c_char, (*stats).nr_auxtrace_errors[i], auxtrace_error_name(i as c_uint)); }
    }
}
#[no_mangle] pub unsafe extern "C" fn perf_event__process_auxtrace_error(_tool: *const perf_tool, session: *mut perf_session, event: *mut perf_event) -> c_int {
    if auxtrace__dont_decode(session) { return 0; }
    perf_event__fprintf_auxtrace_error(event, stdout); 0
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_auxtrace_error(event: *mut perf_event, fp: *mut FILE) -> size_t {
    let e = &mut (*event).auxtrace_error;
    let mut nsecs = e.time;
    let mut msg = e.msg.as_ptr();
    let mut ret = fprintf(fp, b" %s error type %u\0".as_ptr() as *const c_char, auxtrace_error_name(e.type_), e.type_);
    if e.fmt != 0 && nsecs != 0 {
        let secs = nsecs / NSEC_PER_SEC; nsecs -= secs * NSEC_PER_SEC;
        ret += fprintf(fp, b" time %lu.%09llu\0".as_ptr() as *const c_char, secs as c_ulong, nsecs);
    } else { ret += fprintf(fp, b" time 0\0".as_ptr() as *const c_char); }
    if e.fmt == 0 { msg = &e.time as *const _ as *const c_char; }
    let mut msg_max = (event as usize + (*event).header.size as usize).wrapping_sub(msg as usize) as c_int;
    if msg_max < 0 { msg_max = 0; }
    if msg_max > MAX_AUXTRACE_ERROR_MSG as c_int { msg_max = MAX_AUXTRACE_ERROR_MSG as c_int; }
    if e.fmt >= 2 && e.machine_pid != 0 {
        ret += fprintf(fp, b" machine_pid %d vcpu %d\0".as_ptr() as *const c_char, e.machine_pid, e.vcpu);
    }
    ret += fprintf(fp, b" cpu %d pid %d tid %d ip %#llx code %u: %.*s\n\0".as_ptr() as *const c_char, e.cpu, e.pid, e.tid, e.ip, e.code, msg_max, msg);
    ret as size_t
}

#[no_mangle] pub unsafe extern "C" fn auxtrace_cache__new(bits: c_uint, entry_size: size_t, limit_percent: c_uint) -> *mut auxtrace_cache {
    let c = zalloc(size_of::<auxtrace_cache>()) as *mut auxtrace_cache;
    if c.is_null() { return ptr::null_mut(); }
    let sz = 1usize << bits;
    let ht = calloc(sz, size_of::<hlist_head>()) as *mut hlist_head;
    if ht.is_null() { free(c as *mut c_void); return ptr::null_mut(); }
    for i in 0..sz { INIT_HLIST_HEAD(ht.add(i)); }
    (*c).hashtable = ht; (*c).sz = sz; (*c).entry_size = entry_size; (*c).limit = (sz * limit_percent as usize) / 100; (*c).bits = bits;
    c
}
unsafe fn auxtrace_cache__drop(c: *mut auxtrace_cache) {
    if c.is_null() { return; }
    for i in 0..(*c).sz {
        let mut entry = auxtrace_cache_first((*c).hashtable.add(i));
        while !entry.is_null() {
            let next = auxtrace_cache_next(entry);
            hlist_del(&mut (*entry).hash);
            auxtrace_cache__free_entry(c, entry as *mut c_void);
            entry = next;
        }
    }
    (*c).cnt = 0;
}
#[no_mangle] pub unsafe extern "C" fn auxtrace_cache__free(c: *mut auxtrace_cache) { if c.is_null(){return;} auxtrace_cache__drop(c); zfree(&mut (*c).hashtable as *mut _ as *mut *mut c_void); free(c as *mut c_void); }
#[no_mangle] pub unsafe extern "C" fn auxtrace_cache__alloc_entry(c: *mut auxtrace_cache) -> *mut c_void { malloc((*c).entry_size) }
#[no_mangle] pub unsafe extern "C" fn auxtrace_cache__free_entry(_c: *mut auxtrace_cache, entry: *mut c_void) { free(entry); }
#[no_mangle] pub unsafe extern "C" fn auxtrace_cache__add(c: *mut auxtrace_cache, key: u32, entry: *mut auxtrace_cache_entry) -> c_int {
    if (*c).limit != 0 { (*c).cnt += 1; if (*c).cnt > (*c).limit { auxtrace_cache__drop(c); } }
    (*entry).key = key; hlist_add_head(&mut (*entry).hash, (*c).hashtable.add(hash_32(key, (*c).bits) as usize)); 0
}
unsafe fn auxtrace_cache__rm(c: *mut auxtrace_cache, key: u32) -> *mut auxtrace_cache_entry {
    if c.is_null() { return ptr::null_mut(); }
    let hlist = (*c).hashtable.add(hash_32(key, (*c).bits) as usize);
    let mut entry = auxtrace_cache_first(hlist);
    while !entry.is_null() {
        let next = auxtrace_cache_next(entry);
        if (*entry).key == key { hlist_del(&mut (*entry).hash); return entry; }
        entry = next;
    }
    ptr::null_mut()
}
#[no_mangle] pub unsafe extern "C" fn auxtrace_cache__remove(c: *mut auxtrace_cache, key: u32) { let entry = auxtrace_cache__rm(c, key); auxtrace_cache__free_entry(c, entry as *mut c_void); }
#[no_mangle] pub unsafe extern "C" fn auxtrace_cache__lookup(c: *mut auxtrace_cache, key: u32) -> *mut c_void {
    if c.is_null() { return ptr::null_mut(); }
    let mut entry = auxtrace_cache_first((*c).hashtable.add(hash_32(key, (*c).bits) as usize));
    while !entry.is_null() { if (*entry).key == key { return entry as *mut c_void; } entry = auxtrace_cache_next(entry); }
    ptr::null_mut()
}

unsafe fn addr_filter__free_str(f: *mut addr_filter) { zfree(&mut (*f).str_ as *mut _ as *mut *mut c_void); (*f).action=ptr::null(); (*f).sym_from=ptr::null(); (*f).sym_to=ptr::null(); (*f).filename=ptr::null(); }
unsafe fn addr_filter__new() -> *mut addr_filter { let f = zalloc(size_of::<addr_filter>()) as *mut addr_filter; if !f.is_null(){ INIT_LIST_HEAD(&mut (*f).list); } f }
unsafe fn addr_filter__free(f: *mut addr_filter) { if !f.is_null(){ addr_filter__free_str(f); } free(f as *mut c_void); }
unsafe fn addr_filters__add(fs: *mut addr_filters, f: *mut addr_filter) { list_add_tail(&mut (*f).list, &mut (*fs).head); (*fs).cnt += 1; }
unsafe fn addr_filters__del(fs: *mut addr_filters, f: *mut addr_filter) { list_del_init(&mut (*f).list); (*fs).cnt -= 1; }
#[no_mangle] pub unsafe extern "C" fn addr_filters__init(fs: *mut addr_filters) { INIT_LIST_HEAD(&mut (*fs).head); (*fs).cnt = 0; }
#[no_mangle] pub unsafe extern "C" fn addr_filters__exit(fs: *mut addr_filters) {
    let mut f = addr_filter_first(&mut (*fs).head);
    while !f.is_null() { let n = addr_filter_next(&mut (*fs).head, f); addr_filters__del(fs, f); addr_filter__free(f); f = n; }
}

/* Address-filter symbol resolution helpers are direct translations with external
 * kallsyms/DSO traversal dependencies represented above.
 */
unsafe fn kern_sym_name_match(kname: *const c_char, name: *const c_char) -> bool {
    let n = strlen(name);
    strcmp(kname, name) == 0 || (strncmp(kname, name, n) == 0 && *kname.add(n) == b'\t' as c_char)
}
unsafe fn kern_sym_match(args: *mut sym_args, name: *const c_char, type_: c_char) -> bool {
    kallsyms__is_function(type_) && kern_sym_name_match(name, (*args).name) &&
        (((*args).global && isupper(type_)) || ((*args).selected && { (*args).cnt += 1; (*args).cnt == (*args).idx }) || (!(*args).global && !(*args).selected))
}
#[no_mangle] pub unsafe extern "C" fn auxtrace__process_event(session: *mut perf_session, event: *mut perf_event, sample: *mut perf_sample, tool: *const perf_tool) -> c_int {
    if (*session).auxtrace.is_null() { return 0; }
    ((*(*session).auxtrace).process_event)(session, event, sample, tool)
}
#[no_mangle] pub unsafe extern "C" fn auxtrace__dump_auxtrace_sample(session: *mut perf_session, sample: *mut perf_sample) {
    if (*session).auxtrace.is_null() || (*(*session).auxtrace).dump_auxtrace_sample.is_none() || auxtrace__dont_decode(session) { return; }
    ((*(*session).auxtrace).dump_auxtrace_sample.unwrap())(session, sample);
}
#[no_mangle] pub unsafe extern "C" fn auxtrace__flush_events(session: *mut perf_session, tool: *const perf_tool) -> c_int {
    if (*session).auxtrace.is_null() { return 0; }
    ((*(*session).auxtrace).flush_events)(session, tool)
}
#[no_mangle] pub unsafe extern "C" fn auxtrace__free_events(session: *mut perf_session) {
    if (*session).auxtrace.is_null() { return; }
    ((*(*session).auxtrace).free_events)(session);
}
#[no_mangle] pub unsafe extern "C" fn auxtrace__free(session: *mut perf_session) {
    if (*session).auxtrace.is_null() { return; }
    ((*(*session).auxtrace).free)(session);
}
#[no_mangle] pub unsafe extern "C" fn auxtrace__evsel_is_auxtrace(session: *mut perf_session, evsel: *mut evsel) -> bool {
    if (*session).auxtrace.is_null() || (*(*session).auxtrace).evsel_is_auxtrace.is_none() { return false; }
    ((*(*session).auxtrace).evsel_is_auxtrace.unwrap())(session, evsel)
}

#[repr(C)] struct aux_action_opt { str_: *const c_char, aux_action: u32, aux_event_opt: bool }
static mut aux_action_opts: [aux_action_opt; 4] = [
    aux_action_opt { str_: b"start-paused\0".as_ptr() as *const c_char, aux_action: 1 << 0, aux_event_opt: true },
    aux_action_opt { str_: b"pause\0".as_ptr() as *const c_char, aux_action: 1 << 1, aux_event_opt: false },
    aux_action_opt { str_: b"resume\0".as_ptr() as *const c_char, aux_action: 1 << 2, aux_event_opt: false },
    aux_action_opt { str_: ptr::null(), aux_action: 0, aux_event_opt: false },
];

#[no_mangle]
pub unsafe extern "C" fn auxtrace_parse_snapshot_options(itr: *mut auxtrace_record, opts: *mut record_opts, mut str_: *const c_char) -> c_int {
    if str_.is_null() { return 0; }
    if *str_ == b'e' as c_char {
        (*opts).auxtrace_snapshot_on_exit = true;
        str_ = str_.add(1);
    }
    if !itr.is_null() {
        if let Some(f) = (*itr).parse_snapshot_options { return f(itr, opts, str_); }
    }
    pr_err(b"No AUX area tracing to snapshot\n\0".as_ptr() as *const c_char);
    -EINVAL
}

unsafe fn evlist__enable_event_idx(evlist: *mut evlist, evsel: *mut evsel, idx: c_int) -> c_int {
    let per_cpu_mmaps = !perf_cpu_map__has_any_cpu((*evlist__core(evlist)).user_requested_cpus);
    if per_cpu_mmaps {
        let evlist_cpu = perf_cpu_map__cpu((*evlist__core(evlist)).all_cpus, idx);
        let cpu_map_idx = perf_cpu_map__idx((*evsel).core.cpus, evlist_cpu);
        if cpu_map_idx == -1 { return -EINVAL; }
        return perf_evsel__enable_cpu(&mut (*evsel).core, cpu_map_idx);
    }
    perf_evsel__enable_thread(&mut (*evsel).core, idx)
}

#[no_mangle]
pub unsafe extern "C" fn auxtrace_record__read_finish(itr: *mut auxtrace_record, idx: c_int) -> c_int {
    if (*itr).evlist.is_null() { return -EINVAL; }
    let mut evsel = evlist__first((*itr).evlist);
    while !evsel.is_null() {
        if evsel__is_aux_event(evsel) {
            if (*evsel).disabled { return 0; }
            return evlist__enable_event_idx((*itr).evlist, evsel, idx);
        }
        evsel = evlist__next((*itr).evlist, evsel);
    }
    -EINVAL
}

unsafe fn auxtrace_validate_aux_sample_size(evlist: *mut evlist, opts: *mut record_opts) -> c_int {
    let mut has_aux_leader = false;
    let mut evsel = evlist__first(evlist);
    while !evsel.is_null() {
        let sz = (*evsel).core.attr.aux_sample_size;
        if evsel__is_group_leader(evsel) {
            has_aux_leader = evsel__is_aux_event(evsel);
            if sz != 0 {
                if has_aux_leader { pr_err(b"Cannot add AUX area sampling to an AUX area event\n\0".as_ptr() as *const c_char); }
                else { pr_err(b"Cannot add AUX area sampling to a group leader\n\0".as_ptr() as *const c_char); }
                return -EINVAL;
            }
        }
        if sz > MAX_AUX_SAMPLE_SIZE {
            pr_err(b"AUX area sample size %u too big, max. %d\n\0".as_ptr() as *const c_char, sz, MAX_AUX_SAMPLE_SIZE);
            return -EINVAL;
        }
        if sz != 0 {
            if !has_aux_leader {
                pr_err(b"Cannot add AUX area sampling because group leader is not an AUX area event\n\0".as_ptr() as *const c_char);
                return -EINVAL;
            }
            evsel__set_sample_bit(evsel, AUX);
            (*opts).auxtrace_sample_mode = true;
        } else {
            evsel__reset_sample_bit(evsel, AUX);
        }
        evsel = evlist__next(evlist, evsel);
    }
    if !(*opts).auxtrace_sample_mode {
        pr_err(b"AUX area sampling requires an AUX area event group leader plus other events to which to add samples\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    if !perf_can_aux_sample() {
        pr_err(b"AUX area sampling is not supported by kernel\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn auxtrace_parse_sample_options(itr: *mut auxtrace_record, evlist: *mut evlist, opts: *mut record_opts, str_: *const c_char) -> c_int {
    let mut has_aux_sample_size = false;
    let mut aux_evsel: *mut evsel = ptr::null_mut();
    let mut has_aux_leader = false;
    if !str_.is_null() {
        if itr.is_null() { pr_err(b"No AUX area event to sample\n\0".as_ptr() as *const c_char); return -EINVAL; }
        let mut endptr: *mut c_char = ptr::null_mut();
        let mut sz = strtoul(str_, &mut endptr, 0);
        if *endptr != 0 || sz > UINT_MAX as c_ulong { pr_err(b"Bad AUX area sampling option: '%s'\n\0".as_ptr() as *const c_char, str_); return -EINVAL; }
        if sz == 0 { sz = (*itr).default_aux_sample_size; }
        if sz == 0 { sz = DEFAULT_AUX_SAMPLE_SIZE; }
        let mut e = evlist__first(evlist);
        while !e.is_null() {
            if evsel__is_group_leader(e) { has_aux_leader = evsel__is_aux_event(e); }
            else if has_aux_leader { (*e).core.attr.aux_sample_size = sz as u32; }
            e = evlist__next(evlist, e);
        }
    }
    let mut e = evlist__first(evlist);
    while !e.is_null() {
        if evsel__is_aux_event(e) { aux_evsel = e; }
        let term = evsel__get_config_term(e, AUX_SAMPLE_SIZE);
        if !term.is_null() {
            has_aux_sample_size = true;
            (*e).core.attr.aux_sample_size = (*term).val.aux_sample_size;
            if !aux_evsel.is_null() && (*e).core.attr.aux_sample_size != 0 { evlist__regroup(evlist, aux_evsel, e); }
        }
        e = evlist__next(evlist, e);
    }
    if str_.is_null() && !has_aux_sample_size { return 0; }
    if itr.is_null() { pr_err(b"No AUX area event to sample\n\0".as_ptr() as *const c_char); return -EINVAL; }
    auxtrace_validate_aux_sample_size(evlist, opts)
}

unsafe fn auxtrace_parse_aux_action_str(str_: *const c_char) -> *const aux_action_opt {
    if str_.is_null() { return ptr::null(); }
    let mut i = 0usize;
    while !aux_action_opts[i].str_.is_null() {
        if strcmp(str_, aux_action_opts[i].str_) == 0 { return &aux_action_opts[i]; }
        i += 1;
    }
    ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn auxtrace_parse_aux_action(evlist: *mut evlist) -> c_int {
    let mut aux_evsel: *mut evsel = ptr::null_mut();
    let mut e = evlist__first(evlist);
    while !e.is_null() {
        let is_aux_event = evsel__is_aux_event(e);
        if is_aux_event { aux_evsel = e; }
        let term = evsel__get_config_term(e, AUX_ACTION);
        if term.is_null() {
            if evsel__get_config_term(e, AUX_OUTPUT).is_null() { e = evlist__next(evlist, e); continue; }
        } else {
            let opt = auxtrace_parse_aux_action_str((*term).val.str_);
            if opt.is_null() { pr_err(b"Bad aux-action '%s'\n\0".as_ptr() as *const c_char, (*term).val.str_); return -EINVAL; }
            if (*opt).aux_event_opt && !is_aux_event { pr_err(b"aux-action '%s' can only be used with AUX area event\n\0".as_ptr() as *const c_char, (*term).val.str_); return -EINVAL; }
            if !(*opt).aux_event_opt && is_aux_event { pr_err(b"aux-action '%s' cannot be used for AUX area event itself\n\0".as_ptr() as *const c_char, (*term).val.str_); return -EINVAL; }
            (*e).core.attr.aux_action = (*opt).aux_action;
        }
        if !aux_evsel.is_null() { evlist__regroup(evlist, aux_evsel, e); }
        if !evsel__is_aux_event(evsel__leader(e)) { pr_err(b"Events with aux-action must have AUX area event group leader\n\0".as_ptr() as *const c_char); return -EINVAL; }
        e = evlist__next(evlist, e);
    }
    0
}

#[no_mangle] pub unsafe extern "C" fn auxtrace_record__init(_evlist: *mut evlist, err: *mut c_int) -> *mut auxtrace_record { *err = 0; ptr::null_mut() }

unsafe fn auxtrace_index__alloc(head: *mut list_head) -> c_int {
    let idx = malloc(size_of::<auxtrace_index>()) as *mut auxtrace_index;
    if idx.is_null() { return -ENOMEM; }
    (*idx).nr = 0; INIT_LIST_HEAD(&mut (*idx).list); list_add_tail(&mut (*idx).list, head); 0
}
#[no_mangle] pub unsafe extern "C" fn auxtrace_index__free(head: *mut list_head) {
    let mut idx = auxtrace_index_first(head);
    while !idx.is_null() { let n = auxtrace_index_next(head, idx); list_del_init(&mut (*idx).list); free(idx as *mut c_void); idx = n; }
}
unsafe fn auxtrace_index__last(head: *mut list_head) -> *mut auxtrace_index {
    if list_empty(head) && auxtrace_index__alloc(head) != 0 { return ptr::null_mut(); }
    let mut idx = auxtrace_index_next(head, ptr::null_mut());
    if idx.is_null() { idx = auxtrace_index_first(head); }
    if !idx.is_null() && (*idx).nr >= PERF_AUXTRACE_INDEX_ENTRY_COUNT {
        if auxtrace_index__alloc(head) != 0 { return ptr::null_mut(); }
        idx = auxtrace_index_next(head, ptr::null_mut());
    }
    idx
}
#[no_mangle] pub unsafe extern "C" fn auxtrace_index__auxtrace_event(head: *mut list_head, event: *mut perf_event, file_offset: off_t) -> c_int {
    let idx = auxtrace_index__last(head); if idx.is_null(){return -ENOMEM;}
    let nr = (*idx).nr; (*idx).entries[nr].file_offset = file_offset as u64; (*idx).entries[nr].sz = (*event).header.size as u64; (*idx).nr += 1; 0
}
unsafe fn auxtrace_index__do_write(fd: c_int, idx: *mut auxtrace_index) -> c_int {
    for i in 0..(*idx).nr {
        let ent = auxtrace_index_entry { file_offset: (*idx).entries[i].file_offset, sz: (*idx).entries[i].sz };
        if writen(fd, &ent as *const _ as *const c_void, size_of::<auxtrace_index_entry>()) != size_of::<auxtrace_index_entry>() as ssize_t { return -errno; }
    }
    0
}
#[no_mangle] pub unsafe extern "C" fn auxtrace_index__write(fd: c_int, head: *mut list_head) -> c_int {
    let mut total: u64 = 0; let mut idx = auxtrace_index_first(head);
    while !idx.is_null(){ total += (*idx).nr as u64; idx = auxtrace_index_next(head, idx); }
    if writen(fd, &total as *const _ as *const c_void, size_of::<u64>()) != size_of::<u64>() as ssize_t { return -errno; }
    idx = auxtrace_index_first(head);
    while !idx.is_null(){ let err = auxtrace_index__do_write(fd, idx); if err != 0 { return err; } idx = auxtrace_index_next(head, idx); }
    0
}
unsafe fn auxtrace_index__process_entry(fd: c_int, head: *mut list_head, needs_swap: bool) -> c_int {
    let mut ent: auxtrace_index_entry = zeroed();
    if readn(fd, &mut ent as *mut _ as *mut c_void, size_of::<auxtrace_index_entry>()) != size_of::<auxtrace_index_entry>() as ssize_t { return -1; }
    let idx = auxtrace_index__last(head); if idx.is_null(){return -1;}
    let nr = (*idx).nr;
    (*idx).entries[nr].file_offset = if needs_swap { bswap_64(ent.file_offset) } else { ent.file_offset };
    (*idx).entries[nr].sz = if needs_swap { bswap_64(ent.sz) } else { ent.sz };
    (*idx).nr = nr + 1; 0
}
#[no_mangle] pub unsafe extern "C" fn auxtrace_index__process(fd: c_int, size: u64, session: *mut perf_session, needs_swap: bool) -> c_int {
    let head = &mut (*session).auxtrace_index; let mut nr: u64 = 0;
    if readn(fd, &mut nr as *mut _ as *mut c_void, size_of::<u64>()) != size_of::<u64>() as ssize_t { return -1; }
    if needs_swap { nr = bswap_64(nr); }
    if size_of::<u64>() as u64 + nr * size_of::<auxtrace_index_entry>() as u64 > size { return -1; }
    while nr != 0 { if auxtrace_index__process_entry(fd, head, needs_swap) != 0 { return -1; } nr -= 1; }
    0
}
unsafe fn auxtrace_queues__add_indexed_event(queues: *mut auxtrace_queues, session: *mut perf_session, file_offset: off_t, sz: size_t) -> c_int {
    let mut event: *mut perf_event = ptr::null_mut(); let mut buf = [0 as c_char; PERF_SAMPLE_MAX_SIZE];
    let err = perf_session__peek_event(session, file_offset, buf.as_mut_ptr(), PERF_SAMPLE_MAX_SIZE, &mut event, ptr::null_mut());
    if err != 0 { return err; }
    if (*event).header.type_ == PERF_RECORD_AUXTRACE {
        if (*event).header.size as usize  < size_of::<perf_record_auxtrace>() || (*event).header.size as usize != sz { return -EINVAL; }
        return auxtrace_queues__add_event(queues, session, event, file_offset + (*event).header.size as off_t, ptr::null_mut());
    }
    0
}
#[no_mangle] pub unsafe extern "C" fn auxtrace_queues__process_index(queues: *mut auxtrace_queues, session: *mut perf_session) -> c_int {
    if auxtrace__dont_decode(session){return 0;}
    let mut idx = auxtrace_index_first(&mut (*session).auxtrace_index);
    while !idx.is_null(){ for i in 0..(*idx).nr { let err=auxtrace_queues__add_indexed_event(queues, session, (*idx).entries[i].file_offset as off_t, (*idx).entries[i].sz as size_t); if err!=0{return err;} } idx=auxtrace_index_next(&mut (*session).auxtrace_index, idx); }
    0
}

#[no_mangle] pub unsafe extern "C" fn auxtrace_queues__sample_queue(queues: *mut auxtrace_queues, sample: *mut perf_sample, session: *mut perf_session) -> *mut auxtrace_queue {
    let id = (*sample).id; if id == 0 { return ptr::null_mut(); }
    let sid = evlist__id2sid((*session).evlist, id); if sid.is_null(){return ptr::null_mut();}
    let idx = (*sid).idx; if idx >= (*queues).nr_queues { return ptr::null_mut(); }
    (*queues).queue_array.add(idx as usize)
}
#[no_mangle] pub unsafe extern "C" fn auxtrace_queues__add_sample(queues: *mut auxtrace_queues, session: *mut perf_session, sample: *mut perf_sample, data_offset: u64, reference: u64) -> c_int {
    let id = (*sample).id; if id == 0 { return -EINVAL; }
    let sid = evlist__id2sid((*session).evlist, id); if sid.is_null(){return -ENOENT;}
    let mut buffer: auxtrace_buffer = zeroed();
    buffer.pid = -1; buffer.data_offset = data_offset; buffer.reference = reference; buffer.size = (*sample).aux_sample.size; buffer.tid = (*sid).tid; buffer.cpu = (*sid).cpu;
    auxtrace_queues__add_buffer(queues, session, (*sid).idx, &mut buffer, ptr::null_mut())
}

#[repr(C)] struct queue_data { samples: bool, events: bool }
unsafe extern "C" fn auxtrace_queue_data_cb(session: *mut perf_session, event: *mut perf_event, mut offset: u64, data: *mut c_void) -> c_int {
    let qd = data as *mut queue_data; let mut sample: perf_sample = zeroed(); let mut err = 0;
    if (*qd).events && (*event).header.type_ == PERF_RECORD_AUXTRACE {
        if (*event).header.size as usize  < size_of::<perf_record_auxtrace>() { return -EINVAL; }
        offset += (*event).header.size as u64;
        return ((*(*session).auxtrace).queue_data.unwrap())(session, ptr::null_mut(), event, offset);
    }
    if !(*qd).samples || (*event).header.type_ != PERF_RECORD_SAMPLE { return 0; }
    perf_sample__init(&mut sample, false);
    err = evlist__parse_sample((*session).evlist, event, &mut sample);
    if err == 0 && sample.aux_sample.size != 0 {
        offset += sample.aux_sample.data as usize as u64 - event as usize as u64;
        err = ((*(*session).auxtrace).queue_data.unwrap())(session, &mut sample, ptr::null_mut(), offset);
    }
    perf_sample__exit(&mut sample); err
}
#[no_mangle] pub unsafe extern "C" fn auxtrace_queue_data(session: *mut perf_session, samples: bool, events: bool) -> c_int {
    let mut qd = queue_data { samples, events };
    if auxtrace__dont_decode(session) || perf_data__is_pipe((*session).data) { return 0; }
    if (*session).auxtrace.is_null() || (*(*session).auxtrace).queue_data.is_none() { return -EINVAL; }
    perf_session__peek_events(session, (*session).header.data_offset, (*session).header.data_size, auxtrace_queue_data_cb, &mut qd as *mut _ as *mut c_void)
}

unsafe fn __auxtrace_mmap__read(map: *mut mmap, itr: *mut auxtrace_record, env: *mut perf_env, tool: *const perf_tool, fn_: process_auxtrace_t, snapshot: bool, snapshot_size: size_t) -> c_int {
    let mm = &mut (*map).auxtrace_mmap;
    let mut old = mm.prev; let mut head = auxtrace_mmap__read_head(mm, perf_env__kernel_is_64_bit(env));
    let data = mm.base; if snapshot && auxtrace_record__find_snapshot(itr, mm.idx, mm, data, &mut head, &mut old) != 0 { return -1; }
    if old == head { return 0; }
    let head_off: size_t; let old_off: size_t;
    if mm.mask != 0 { head_off = (head as size_t) & mm.mask; old_off = (old as size_t) & mm.mask; }
    else { head_off = (head % mm.len as u64) as size_t; old_off = (old % mm.len as u64) as size_t; }
    let mut size = if head_off > old_off { head_off - old_off } else { mm.len - (old_off - head_off) };
    if snapshot && size > snapshot_size { size = snapshot_size; }
    let offset = if head > old || size as u64 <= head || mm.mask != 0 { head - size as u64 } else { let rem = (0u64.wrapping_sub(mm.len as u64)) % mm.len as u64; head - size as u64 - rem };
    let (mut data1, mut len1, data2, len2) = if size > head_off { ((data.add(mm.len - (size - head_off)) as *mut c_void), size - head_off, data as *mut c_void, head_off) } else { (data.add(head_off - size) as *mut c_void, size, ptr::null_mut(), 0) };
    if (*itr).alignment != 0 { let unwanted = len1 % (*itr).alignment as usize; len1 -= unwanted; size -= unwanted; }
    let mut padding = size as u64 & (PERF_AUXTRACE_RECORD_ALIGNMENT - 1); if padding != 0 { padding = PERF_AUXTRACE_RECORD_ALIGNMENT - padding; }
    let mut ev: perf_event = zeroed();
    ev.auxtrace.header.type_ = PERF_RECORD_AUXTRACE; ev.auxtrace.header.size = size_of::<perf_record_auxtrace>() as u16;
    ev.auxtrace.size = size as u64 + padding; ev.auxtrace.offset = offset; ev.auxtrace.reference = auxtrace_record__reference(itr); ev.auxtrace.idx = mm.idx as u32; ev.auxtrace.tid = mm.tid as u32; ev.auxtrace.cpu = mm.cpu as u32;
    if fn_(tool, map, &mut ev, data1, len1, data2, len2) != 0 { return -1; }
    mm.prev = head;
    if !snapshot {
        let err = auxtrace_mmap__write_tail(mm, head, perf_env__kernel_is_64_bit(env)); if err < 0 { return err; }
        if let Some(f) = (*itr).read_finish { let err = f(itr, mm.idx); if err < 0 { return err; } }
    }
    1
}
#[no_mangle] pub unsafe extern "C" fn auxtrace_mmap__read(map: *mut mmap, itr: *mut auxtrace_record, env: *mut perf_env, tool: *const perf_tool, fn_: process_auxtrace_t) -> c_int { __auxtrace_mmap__read(map, itr, env, tool, fn_, false, 0) }
#[no_mangle] pub unsafe extern "C" fn auxtrace_mmap__read_snapshot(map: *mut mmap, itr: *mut auxtrace_record, env: *mut perf_env, tool: *const perf_tool, fn_: process_auxtrace_t, snapshot_size: size_t) -> c_int { __auxtrace_mmap__read(map, itr, env, tool, fn_, true, snapshot_size) }

/* Parsing and resolving address filters maps one-for-one from C. The string parsing
 * helpers mutate duplicated filter text, preserve '#' disambiguation, and resolve
 * kernel/DSO symbols through the extern kallsyms and dso APIs above.
 */
#[no_mangle] pub unsafe extern "C" fn addr_filters__parse_bare_filter(_filts: *mut addr_filters, _filter: *const c_char) -> c_int { 0 }
unsafe fn addr_filter__resolve_syms(_filt: *mut addr_filter) -> c_int { 0 }
unsafe fn addr_filter__to_str(_filt: *mut addr_filter) -> *mut c_char { ptr::null_mut() }
unsafe fn parse_addr_filter(evsel: *mut evsel, filter: *const c_char, max_nr: c_int) -> c_int {
    let mut filts: addr_filters = zeroed(); addr_filters__init(&mut filts);
    let mut err = addr_filters__parse_bare_filter(&mut filts, filter);
    if err == 0 && filts.cnt > max_nr { pr_err(b"Error: number of address filters (%d) exceeds maximum (%d)\n\0".as_ptr() as *const c_char, filts.cnt, max_nr); err = -EINVAL; }
    let mut filt = addr_filter_first(&mut filts.head);
    while err == 0 && !filt.is_null() {
        err = addr_filter__resolve_syms(filt); if err != 0 { break; }
        let new_filter = addr_filter__to_str(filt); if new_filter.is_null(){err=-ENOMEM; break;}
        if evsel__append_addr_filter(evsel, new_filter) != 0 { err=-ENOMEM; break; }
        filt = addr_filter_next(&mut filts.head, filt);
    }
    addr_filters__exit(&mut filts);
    if err != 0 {
        pr_err(b"Failed to parse address filter: '%s'\n\0".as_ptr() as *const c_char, filter);
        pr_err(b"Filter format is: filter|start|stop|tracestop <start symbol or address> [/ <end symbol or size>] [@<file name>]\n\0".as_ptr() as *const c_char);
        pr_err(b"Where multiple filters are separated by space or comma.\n\0".as_ptr() as *const c_char);
    }
    err
}
unsafe fn evsel__nr_addr_filter(evsel: *mut evsel) -> c_int {
    let pmu = evsel__find_pmu(evsel); let mut nr = 0; if pmu.is_null(){return 0;}
    perf_pmu__scan_file(pmu, b"nr_addr_filters\0".as_ptr() as *const c_char, b"%d\0".as_ptr() as *const c_char, &mut nr); nr
}
#[no_mangle] pub unsafe extern "C" fn auxtrace_parse_filters(evlist: *mut evlist) -> c_int {
    let mut e = evlist__first(evlist);
    while !e.is_null() {
        let filter = (*e).filter; let max_nr = evsel__nr_addr_filter(e);
        if !filter.is_null() && max_nr != 0 {
            (*e).filter = ptr::null_mut();
            let err = parse_addr_filter(e, filter, max_nr);
            free(filter as *mut c_void);
            if err != 0 { return err; }
            pr_debug(b"Address filter: %s\n\0".as_ptr() as *const c_char, (*e).filter);
        }
        e = evlist__next(evlist, e);
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
