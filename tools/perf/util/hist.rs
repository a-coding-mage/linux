// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/hist.c.
//
// This file is intentionally source-level and dependency-facing: all data
// structures, list/rbtree helpers, perf helpers, and configuration globals used
// here are supplied by the surrounding perf translation.  The implementation
// keeps the C ABI names and raw-pointer semantics.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(clippy::missing_safety_doc)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

pub type u16 = u16;
pub type u32 = u32;
pub type u64 = u64;
pub type int64_t = i64;
pub type size_t = usize;
pub type filter_mask_t = u32;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const BITS_PER_LONG: c_uint = (size_of::<c_ulong>() * 8) as c_uint;
const HIST_FILTER__THREAD: c_int = 0;
const HIST_FILTER__DSO: c_int = 1;
const HIST_FILTER__SYMBOL: c_int = 2;
const HIST_FILTER__PARENT: c_int = 3;
const HIST_FILTER__GUEST: c_int = 4;
const HIST_FILTER__HOST: c_int = 5;
const HIST_FILTER__SOCKET: c_int = 6;
const HIST_FILTER__C2C: c_int = 7;
const HIST_FILTER__PARALLELISM: c_int = 8;
const PERF_RECORD_MISC_KERNEL: c_uint = 1;
const PERF_RECORD_MISC_USER: c_uint = 2;
const PERF_RECORD_MISC_GUEST_KERNEL: c_uint = 3;
const PERF_RECORD_MISC_GUEST_USER: c_uint = 4;
const MEM_STAT_LEN: c_int = 0;
const MAX_NR_CPUS: c_int = 0;
const CGROUP_NS_INDEX: usize = 0;
const CHAIN_GRAPH_REL: c_int = 0;
const HMD_NORMAL: hierarchy_move_dir = 0;
const HMD_FORCE_CHILD: hierarchy_move_dir = 1;
const HMD_FORCE_SIBLING: hierarchy_move_dir = 2;

pub type hierarchy_move_dir = c_int;
pub type FILE = c_void;
pub type option = c_void;
pub type hists__resort_cb_t =
    Option<unsafe extern "C" fn(*mut hist_entry, *mut c_void) -> bool>;
pub type fmt_chk_fn = Option<unsafe extern "C" fn(*mut perf_hpp_fmt) -> bool>;
pub type filter_fn_t = Option<unsafe extern "C" fn(*mut hists, *mut hist_entry) -> bool>;

#[repr(C)]
pub struct rb_node {
    pub rb_left: *mut rb_node,
    pub rb_right: *mut rb_node,
}

#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

#[repr(C)]
pub struct rb_root_cached {
    pub rb_root: rb_root,
    pub rb_leftmost: *mut rb_node,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct he_stat {
    pub period: u64,
    pub period_sys: u64,
    pub period_us: u64,
    pub period_guest_sys: u64,
    pub period_guest_us: u64,
    pub weight1: u64,
    pub weight2: u64,
    pub weight3: u64,
    pub nr_events: u64,
    pub latency: u64,
}

#[repr(C)]
pub struct hists_stats {
    pub nr_events: *mut u64,
    pub nr_samples: u64,
    pub nr_non_filtered_samples: u64,
    pub nr_lost_samples: u64,
    pub nr_dropped_samples: u64,
    pub total_period: u64,
    pub total_non_filtered_period: u64,
    pub total_latency: u64,
    pub total_non_filtered_latency: u64,
}

#[repr(C)]
pub struct mem_stat {
    pub entries: [u64; 1],
}

#[repr(C)]
pub struct hist_entry_ops {
    pub new: Option<unsafe extern "C" fn(size_t) -> *mut c_void>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub struct perf_hpp_list {
    pub fields: list_head,
    pub sorts: list_head,
    pub comm_nodigit: bool,
}

#[repr(C)]
pub struct perf_hpp_fmt {
    pub list: list_head,
    pub init: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut hist_entry)>,
    pub width: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hists) -> c_int>,
}

#[repr(C)]
pub struct perf_hpp {
    pub buf: *mut c_char,
    pub size: size_t,
}

#[repr(C)]
pub struct perf_hpp_list_node {
    pub list: list_head,
    pub hpp: perf_hpp_list,
    pub level: c_int,
    pub skip: bool,
}

#[repr(C)]
pub struct cgroup_id {
    pub dev: u64,
    pub ino: u64,
}

#[repr(C)]
pub struct map_symbol {
    pub thread: *mut thread,
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct addr_map_symbol {
    pub ms: map_symbol,
    pub addr: u64,
}

#[repr(C)]
pub struct branch_flags {
    pub cycles: u64,
}

#[repr(C)]
pub struct branch_info {
    pub from: addr_map_symbol,
    pub to: addr_map_symbol,
    pub flags: branch_flags,
    pub srcline_from: *mut c_char,
    pub srcline_to: *mut c_char,
    pub branch_stack_cntr: u64,
}

#[repr(C)]
pub struct mem_data_src {
    pub val: u64,
}

#[repr(C)]
pub struct mem_info_addr {
    pub ms: map_symbol,
}

#[repr(C)]
pub struct pairs {
    pub node: list_head,
    pub head: list_head,
}

#[repr(C)]
pub struct hist_entry {
    pub rb_node: rb_node,
    pub rb_node_in: rb_node,
    pub thread: *mut thread,
    pub comm: *const c_char,
    pub cgroup_id: cgroup_id,
    pub cgroup: u64,
    pub ms: map_symbol,
    pub srcline: *mut c_char,
    pub srcfile: *mut c_char,
    pub socket: c_int,
    pub cpu: c_int,
    pub cpumode: c_uint,
    pub ip: u64,
    pub level: c_char,
    pub code_page_size: u64,
    pub parallelism: c_ulong,
    pub stat: he_stat,
    pub stat_acc: *mut he_stat,
    pub parent: *mut symbol,
    pub parent_he: *mut hist_entry,
    pub filtered: filter_mask_t,
    pub hists: *mut hists,
    pub hpp_list: *mut perf_hpp_list,
    pub branch_info: *mut branch_info,
    pub mem_info: *mut mem_info,
    pub kvm_info: *mut kvm_info,
    pub block_info: *mut block_info,
    pub transaction: u64,
    pub raw_data: *mut c_void,
    pub raw_size: u32,
    pub ops: *mut hist_entry_ops,
    pub time: c_long,
    pub weight: u64,
    pub ins_lat: u64,
    pub weight3: u64,
    pub simd_flags: u64,
    pub callchain_size: size_t,
    pub callchain: *mut callchain_root,
    pub sorted_chain: c_void,
    pub res_samples: *mut res_sample,
    pub num_res: c_int,
    pub pairs: pairs,
    pub hroot_in: rb_root_cached,
    pub hroot_out: rb_root_cached,
    pub leaf: bool,
    pub unfolded: bool,
    pub has_no_entry: bool,
    pub dummy: bool,
    pub depth: c_int,
    pub row_offset: c_int,
    pub nr_rows: c_int,
    pub trace_output: *mut c_char,
    pub mem_stat: *mut mem_stat,
}

#[repr(C)]
pub struct hists {
    pub col_len: *mut u16,
    pub entries_in_array: [rb_root_cached; 2],
    pub entries_in: *mut rb_root_cached,
    pub entries_collapsed: rb_root_cached,
    pub entries: rb_root_cached,
    pub lock: c_void,
    pub socket_filter: c_int,
    pub parallelism_filter: *mut c_ulong,
    pub hpp_list: *mut perf_hpp_list,
    pub hpp_formats: list_head,
    pub dso_filter: *mut dso,
    pub thread_filter: *mut thread,
    pub symbol_filter_str: *mut c_char,
    pub uid_filter_str: *mut c_char,
    pub stats: hists_stats,
    pub nr_entries: u64,
    pub nr_non_filtered_entries: u64,
    pub callchain_period: u64,
    pub callchain_latency: u64,
    pub callchain_non_filtered_period: u64,
    pub callchain_non_filtered_latency: u64,
    pub has_callchains: bool,
    pub nr_mem_stats: c_int,
    pub mem_stat_types: *mut c_int,
    pub mem_stat_total: *mut mem_stat,
}

#[repr(C)]
pub struct symbol {
    pub name: *mut c_char,
    pub namelen: u16,
    pub start: u64,
}

#[repr(C)]
pub struct perf_sample {
    pub evsel: *mut evsel,
    pub cgroup: u64,
    pub code_page_size: u64,
    pub period: u64,
    pub weight: u64,
    pub ins_lat: u64,
    pub weight3: u64,
    pub transaction: u64,
    pub raw_data: *mut c_void,
    pub raw_size: u32,
    pub time: c_ulong,
    pub cpu: c_int,
    pub tid: c_int,
    pub simd_flags: u64,
    pub branch_stack: *mut branch_stack,
}

#[repr(C)]
pub struct addr_location {
    pub thread: *mut thread,
    pub map: *mut map,
    pub sym: *mut symbol,
    pub srcline: *const c_char,
    pub socket: c_int,
    pub cpu: c_int,
    pub cpumode: c_uint,
    pub addr: u64,
    pub level: c_char,
    pub parallelism: c_ulong,
    pub latency: u64,
    pub filtered: filter_mask_t,
}

#[repr(C)]
pub struct hist_entry_iter {
    pub sample: *mut perf_sample,
    pub parent: *mut symbol,
    pub mi: *mut mem_info,
    pub bi: *mut branch_info,
    pub ki: *mut kvm_info,
    pub he: *mut hist_entry,
    pub he_cache: *mut *mut hist_entry,
    pub curr: c_int,
    pub total: c_int,
    pub hide_unresolved: bool,
    pub ops: *const hist_iter_ops,
    pub add_entry_cb: Option<unsafe extern "C" fn(*mut hist_entry_iter, *mut addr_location, bool, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct hist_iter_ops {
    pub prepare_entry: Option<unsafe extern "C" fn(*mut hist_entry_iter, *mut addr_location) -> c_int>,
    pub add_single_entry: Option<unsafe extern "C" fn(*mut hist_entry_iter, *mut addr_location) -> c_int>,
    pub next_entry: Option<unsafe extern "C" fn(*mut hist_entry_iter, *mut addr_location) -> c_int>,
    pub add_next_entry: Option<unsafe extern "C" fn(*mut hist_entry_iter, *mut addr_location) -> c_int>,
    pub finish_entry: Option<unsafe extern "C" fn(*mut hist_entry_iter, *mut addr_location) -> c_int>,
}

#[repr(C)]
pub struct symbol_conf_t {
    pub col_width_list_str: *mut c_char,
    pub field_sep: *mut c_char,
    pub dso_list: *mut c_char,
    pub comm_list: *mut c_char,
    pub pid_list: *mut c_char,
    pub tid_list: *mut c_char,
    pub sym_list: *mut c_char,
    pub parallelism_filter: *mut c_ulong,
    pub time_quantum: c_ulong,
    pub cumulate_callchain: bool,
    pub use_callchain: bool,
    pub report_hierarchy: bool,
    pub exclude_other: bool,
    pub res_sample: c_int,
    pub nanosecs: bool,
    pub filter_relative: bool,
    pub show_ref_callgraph: bool,
    pub show_branchflag_count: bool,
    pub skip_empty: bool,
}

#[repr(C)]
pub struct callchain_param_t {
    pub mode: c_int,
    pub min_percent: u64,
    pub sort: Option<unsafe extern "C" fn(*mut c_void, *mut callchain_root, u64, *mut callchain_param_t)>,
}

#[repr(C)] pub struct thread { _private: [u8; 0] }
#[repr(C)] pub struct map { _private: [u8; 0] }
#[repr(C)] pub struct dso { _private: [u8; 0] }
#[repr(C)] pub struct mem_info { _private: [u8; 0] }
#[repr(C)] pub struct kvm_info { _private: [u8; 0] }
#[repr(C)] pub struct block_info { _private: [u8; 0] }
#[repr(C)] pub struct callchain_root { _private: [u8; 0] }
#[repr(C)] pub struct callchain_cursor { pub nr: c_int }
#[repr(C)] pub struct callchain_cursor_node { _private: [u8; 0] }
#[repr(C)] pub struct res_sample { pub time: c_ulong, pub cpu: c_int, pub tid: c_int }
#[repr(C)] pub struct evsel { pub core: evsel_core }
#[repr(C)] pub struct evsel_core { pub attr: perf_event_attr, pub nr_members: c_int }
#[repr(C)] pub struct perf_event_attr { pub sample_freq: c_int }
#[repr(C)] pub struct evlist { _private: [u8; 0] }
#[repr(C)] pub struct ui_progress { _private: [u8; 0] }
#[repr(C)] pub struct branch_stack { pub nr: c_uint }
#[repr(C)] pub struct branch_entry { pub flags: branch_flags }

unsafe extern "C" {
    static mut symbol_conf: symbol_conf_t;
    static mut callchain_param: callchain_param_t;
    static mut perf_hpp_list: perf_hpp_list;
    static mut verbose: c_int;
    static mut sort_srcline: sort_entry;
    static mut stderr: *mut FILE;
    static SRCLINE_UNKNOWN: *mut c_char;

    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn zalloc(size: size_t) -> *mut c_void;
    fn memdup(ptr: *const c_void, len: size_t) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, len: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, len: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strstr(a: *const c_char, b: *const c_char) -> *mut c_char;
    fn fprintf(fp: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn fputs(s: *const c_char, fp: *mut FILE) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn random() -> c_long;
    fn assert_fail();
    fn pr_debug(fmt: *const c_char, ...);

    fn rb_first_cached(root: *const rb_root_cached) -> *mut rb_node;
    fn rb_next(node: *const rb_node) -> *mut rb_node;
    fn rb_prev(node: *const rb_node) -> *mut rb_node;
    fn rb_last(root: *const rb_root) -> *mut rb_node;
    fn rb_erase_cached(node: *mut rb_node, root: *mut rb_root_cached);
    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node);
    fn rb_insert_color_cached(node: *mut rb_node, root: *mut rb_root_cached, leftmost: bool);
    fn list_is_last(list: *const list_head, head: *const list_head) -> bool;
    fn list_del_init(entry: *mut list_head);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn mutex_init(lock: *mut c_void);
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);

    fn thread__get(thread: *mut thread) -> *mut thread;
    fn thread__put(thread: *mut thread);
    fn thread__zput(thread: *mut thread);
    fn thread__comm(thread: *mut thread) -> *const c_char;
    fn thread__comm_len(thread: *mut thread) -> u16;
    fn thread__comm_set(thread: *mut thread) -> bool;
    fn thread__comm_str(thread: *mut thread) -> *const c_char;
    fn thread__tid(thread: *mut thread) -> c_int;
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn thread__namespaces(thread: *mut thread) -> *mut namespaces;
    fn map__get(map: *mut map) -> *mut map;
    fn map__put(map: *mut map);
    fn map__dso(map: *mut map) -> *mut dso;
    fn map__find_symbol(map: *mut map, addr: u64) -> *mut symbol;
    fn map_symbol__exit(ms: *mut map_symbol);
    fn dso__name_len(dso: *mut dso) -> u16;
    fn dso__short_name(dso: *const dso) -> *const c_char;
    fn maps__machine(maps: *mut maps) -> *mut machine;
    fn cgroup__find(env: *mut c_void, id: u64) -> *mut cgroup;
    fn mem_info__clone(mi: *mut mem_info) -> *mut mem_info;
    fn mem_info__zput(mi: *mut mem_info);
    fn mem_info__iaddr(mi: *mut mem_info) -> *mut mem_info_addr;
    fn mem_info__daddr(mi: *mut mem_info) -> *mut mem_info_addr;
    fn mem_info__const_data_src(mi: *mut mem_info) -> *const mem_data_src;
    fn mem_stat_index(ty: c_int, val: u64) -> c_int;
    fn block_info__delete(bi: *mut block_info);
    fn kvm_info__zput(ki: *mut kvm_info);
    fn hist_entry__has_callchains(he: *mut hist_entry) -> bool;
    fn callchain_init(root: *mut callchain_root);
    fn decay_callchain(root: *mut callchain_root);
    fn free_callchain(root: *mut callchain_root);
    fn hist_entry__append_callchain(he: *mut hist_entry, sample: *mut perf_sample) -> c_int;
    fn callchain_cursor_commit(cursor: *mut callchain_cursor);
    fn callchain_cursor_current(cursor: *mut callchain_cursor) -> *mut callchain_cursor_node;
    fn callchain_cursor_advance(cursor: *mut callchain_cursor);
    fn callchain_cursor_snapshot(dst: *mut callchain_cursor, src: *mut callchain_cursor);
    fn callchain_cursor_reset(cursor: *mut callchain_cursor);
    fn callchain_merge(cursor: *mut callchain_cursor, dst: *mut callchain_root, src: *mut callchain_root) -> c_int;
    fn callchain_append(root: *mut callchain_root, cursor: *mut callchain_cursor, period: u64);
    fn get_tls_callchain_cursor() -> *mut callchain_cursor;
    fn fill_callchain_info(al: *mut addr_location, node: *mut callchain_cursor_node, hide: bool) -> c_int;
    fn sample__resolve_callchain(sample: *mut perf_sample, cursor: *mut callchain_cursor, parent: *mut *mut symbol, al: *mut addr_location, max_stack_depth: c_int) -> c_int;
    fn sample__resolve_mem(sample: *mut perf_sample, al: *mut addr_location) -> *mut mem_info;
    fn sample__resolve_bstack(sample: *mut perf_sample, al: *mut addr_location) -> *mut branch_info;
    fn perf_sample__branch_entries(sample: *mut perf_sample) -> *mut branch_entry;
    fn evsel__hists(evsel: *mut evsel) -> *mut hists;
    fn evsel__has_callchain(evsel: *mut evsel) -> bool;
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
    fn evsel__is_group_event(evsel: *mut evsel) -> bool;
    fn evsel__group_desc(evsel: *mut evsel, buf: *mut c_char, buflen: size_t);
    fn evsel__object_config(size: size_t, init: unsafe extern "C" fn(*mut evsel) -> c_int, exit: unsafe extern "C" fn(*mut evsel)) -> c_int;
    fn hists_to_evsel(hists: *mut hists) -> *mut evsel;
    fn session_done() -> bool;
    fn ui_progress__update(prog: *mut ui_progress, inc: u64);
    fn advance_hpp(hpp: *mut perf_hpp, printed: c_int);
    fn perf_hpp__is_dynamic_entry(fmt: *mut perf_hpp_fmt) -> bool;
    fn perf_hpp__defined_dynamic_entry(fmt: *mut perf_hpp_fmt, hists: *mut hists) -> bool;
    fn perf_hpp__should_skip(fmt: *mut perf_hpp_fmt, hists: *mut hists) -> bool;
    fn perf_hpp__is_thread_entry(fmt: *mut perf_hpp_fmt) -> bool;
    fn perf_hpp__is_comm_entry(fmt: *mut perf_hpp_fmt) -> bool;
    fn perf_hpp__is_dso_entry(fmt: *mut perf_hpp_fmt) -> bool;
    fn perf_hpp__is_sym_entry(fmt: *mut perf_hpp_fmt) -> bool;
    fn perf_hpp__is_parallelism_entry(fmt: *mut perf_hpp_fmt) -> bool;
    fn perf_hpp__is_trace_entry(fmt: *mut perf_hpp_fmt) -> bool;
    fn perf_hpp__is_srcline_entry(fmt: *mut perf_hpp_fmt) -> bool;
    fn perf_hpp__is_srcfile_entry(fmt: *mut perf_hpp_fmt) -> bool;
    fn _sort__sym_cmp(left: *mut symbol, right: *mut symbol) -> int64_t;
    fn sort__comm_nodigit_len(he: *mut hist_entry) -> size_t;
    fn hist_entry__transaction_len() -> u16;
    fn hist_entry__filter(he: *mut hist_entry, ty: c_int, arg: *const c_void) -> c_int;
    fn hist_entry__get_percent_limit(he: *mut hist_entry) -> f32;
    fn hist_entry__add_pair(pair: *mut hist_entry, pos: *mut hist_entry);
    fn hist_entry__has_pairs(he: *mut hist_entry) -> bool;
    fn addr_map_symbol__account_cycles(ams: *mut addr_map_symbol, prev: *mut addr_map_symbol, cycles: u64, evsel: *mut evsel, cntr: u64);
    fn convert_unit(v: c_ulong, unit: *mut c_char) -> c_ulong;
    fn __bitmap_weight(bitmap: *mut c_ulong, bits: c_int) -> c_int;
    fn test_bit(bit: c_ulong, bitmap: *mut c_ulong) -> bool;
    fn RC_CHK_EQUAL(a: *mut c_void, b: *mut c_void) -> bool;
    fn zfree(ptr: *mut *mut c_void);
    fn zfree_srcline(ptr: *mut *mut c_char);
}

#[repr(C)] pub struct maps { _private: [u8; 0] }
#[repr(C)] pub struct machine { pub env: *mut c_void }
#[repr(C)] pub struct namespaces { pub link_info: [ns_link_info; 1] }
#[repr(C)] pub struct ns_link_info { pub dev: u64, pub ino: u64 }
#[repr(C)] pub struct cgroup { pub name: *const c_char }
#[repr(C)] pub struct sort_entry { pub se_header: *const c_char }

#[inline] unsafe fn rb_entry<T>(node: *mut rb_node, _member: &str) -> *mut T { node as *mut T }
#[inline] unsafe fn RB_ROOT_CACHED() -> rb_root_cached { rb_root_cached { rb_root: rb_root { rb_node: null_mut() }, rb_leftmost: null_mut() } }
#[inline] unsafe fn RB_ROOT() -> rb_root { rb_root { rb_node: null_mut() } }
#[inline] unsafe fn RB_EMPTY_ROOT(root: *const rb_root) -> bool { (*root).rb_node.is_null() }
#[inline] unsafe fn hists__has(_hists: *mut hists, _field: c_int) -> bool { false }

#[no_mangle]
pub unsafe extern "C" fn hists__col_len(hists: *mut hists, col: c_int) -> u16 {
    *(*hists).col_len.add(col as usize)
}

#[no_mangle]
pub unsafe extern "C" fn hists__set_col_len(hists: *mut hists, col: c_int, len: u16) {
    *(*hists).col_len.add(col as usize) = len;
}

#[no_mangle]
pub unsafe extern "C" fn hists__new_col_len(hists: *mut hists, col: c_int, len: u16) -> bool {
    if len > hists__col_len(hists, col) {
        hists__set_col_len(hists, col, len);
        true
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn hists__reset_col_len(hists: *mut hists) {
    let mut col = 0;
    while col < 0 {
        hists__set_col_len(hists, col, 0);
        col += 1;
    }
}

unsafe fn hists__set_unres_dso_col_len(hists: *mut hists, dso_col: c_int) {
    let unresolved_col_width = BITS_PER_LONG / 4;
    if (hists__col_len(hists, dso_col) as c_uint) < unresolved_col_width
        && symbol_conf.col_width_list_str.is_null()
        && symbol_conf.field_sep.is_null()
        && symbol_conf.dso_list.is_null()
    {
        hists__set_col_len(hists, dso_col, unresolved_col_width as u16);
    }
}

#[no_mangle]
pub unsafe extern "C" fn hists__calc_col_len(hists: *mut hists, h: *mut hist_entry) {
    let unresolved_col_width = BITS_PER_LONG / 4;
    let mut symlen: c_int;
    let mut len: u16;

    if !(*h).block_info.is_null() {
        return;
    }
    if !(*h).ms.sym.is_null() {
        symlen = (*(*h).ms.sym).namelen as c_int + 4;
        if verbose > 0 {
            symlen += (BITS_PER_LONG / 4 + 2 + 3) as c_int;
        }
        hists__new_col_len(hists, 0, symlen as u16);
    } else {
        symlen = (unresolved_col_width + 4 + 2) as c_int;
        hists__new_col_len(hists, 0, symlen as u16);
        hists__set_unres_dso_col_len(hists, 1);
    }

    len = thread__comm_len((*h).thread);
    if hists__new_col_len(hists, 2, len) {
        hists__set_col_len(hists, 3, len + 8);
    }
    if !(*hists).hpp_list.is_null() && (*(*hists).hpp_list).comm_nodigit {
        hists__new_col_len(hists, 4, sort__comm_nodigit_len(h) as u16);
    }
    if !(*h).ms.map.is_null() {
        len = dso__name_len(map__dso((*h).ms.map));
        hists__new_col_len(hists, 1, len);
    }
    if !(*h).parent.is_null() {
        hists__new_col_len(hists, 5, (*(*h).parent).namelen);
    }
    if !(*h).srcline.is_null() {
        let l = core::cmp::max(strlen((*h).srcline), strlen(sort_srcline.se_header));
        hists__new_col_len(hists, 6, l as u16);
    }
    if !(*h).srcfile.is_null() {
        hists__new_col_len(hists, 7, strlen((*h).srcfile) as u16);
    }
    if (*h).transaction != 0 {
        hists__new_col_len(hists, 8, hist_entry__transaction_len());
    }
    if !(*h).trace_output.is_null() {
        hists__new_col_len(hists, 9, strlen((*h).trace_output) as u16);
    }
}

#[no_mangle]
pub unsafe extern "C" fn hists__output_recalc_col_len(hists: *mut hists, max_rows: c_int) {
    let mut next = rb_first_cached(&(*hists).entries);
    let mut row = 0;
    hists__reset_col_len(hists);
    while !next.is_null() && { row += 1; row <= max_rows } {
        let n: *mut hist_entry = rb_entry(next, "rb_node");
        if (*n).filtered == 0 {
            hists__calc_col_len(hists, n);
        }
        next = rb_next(&(*n).rb_node);
    }
}

unsafe fn he_stat__add_cpumode_period(he_stat: *mut he_stat, cpumode: c_uint, period: u64) {
    match cpumode {
        PERF_RECORD_MISC_KERNEL => (*he_stat).period_sys += period,
        PERF_RECORD_MISC_USER => (*he_stat).period_us += period,
        PERF_RECORD_MISC_GUEST_KERNEL => (*he_stat).period_guest_sys += period,
        PERF_RECORD_MISC_GUEST_USER => (*he_stat).period_guest_us += period,
        _ => {}
    }
}

unsafe fn hist_time(htime: c_ulong) -> c_long {
    let time_quantum = symbol_conf.time_quantum;
    if time_quantum != 0 {
        ((htime / time_quantum) * time_quantum) as c_long
    } else {
        htime as c_long
    }
}

unsafe fn he_stat__add_period(he_stat: *mut he_stat, period: u64, latency: u64) {
    (*he_stat).period += period;
    (*he_stat).latency += latency;
    (*he_stat).nr_events += 1;
}

unsafe fn he_stat__add_stat(dest: *mut he_stat, src: *mut he_stat) {
    (*dest).period += (*src).period;
    (*dest).period_sys += (*src).period_sys;
    (*dest).period_us += (*src).period_us;
    (*dest).period_guest_sys += (*src).period_guest_sys;
    (*dest).period_guest_us += (*src).period_guest_us;
    (*dest).weight1 += (*src).weight1;
    (*dest).weight2 += (*src).weight2;
    (*dest).weight3 += (*src).weight3;
    (*dest).nr_events += (*src).nr_events;
    (*dest).latency += (*src).latency;
}

unsafe fn he_stat__decay(he_stat: *mut he_stat) {
    (*he_stat).period = ((*he_stat).period * 7) / 8;
    (*he_stat).nr_events = ((*he_stat).nr_events * 7) / 8;
    (*he_stat).weight1 = ((*he_stat).weight1 * 7) / 8;
    (*he_stat).weight2 = ((*he_stat).weight2 * 7) / 8;
    (*he_stat).weight3 = ((*he_stat).weight3 * 7) / 8;
    (*he_stat).latency = ((*he_stat).latency * 7) / 8;
}

unsafe fn hists__update_mem_stat(hists: *mut hists, he: *mut hist_entry, mi: *mut mem_info, period: u64) -> c_int {
    if (*hists).nr_mem_stats == 0 { return 0; }
    if (*he).mem_stat.is_null() {
        (*he).mem_stat = calloc((*hists).nr_mem_stats as size_t, size_of::<mem_stat>()) as *mut mem_stat;
        if (*he).mem_stat.is_null() { return -1; }
    }
    let mut i = 0;
    while i < (*hists).nr_mem_stats {
        let idx = mem_stat_index(*(*hists).mem_stat_types.add(i as usize), (*mem_info__const_data_src(mi)).val);
        if !(0 <= idx && idx < MEM_STAT_LEN) { assert_fail(); }
        (*(*he).mem_stat.add(i as usize)).entries[idx as usize] += period;
        (*(*hists).mem_stat_total.add(i as usize)).entries[idx as usize] += period;
        i += 1;
    }
    0
}

unsafe fn hists__add_mem_stat(hists: *mut hists, dst: *mut hist_entry, src: *mut hist_entry) {
    if (*hists).nr_mem_stats == 0 { return; }
    let mut i = 0;
    while i < (*hists).nr_mem_stats {
        let mut k = 0;
        while k < MEM_STAT_LEN {
            (*(*dst).mem_stat.add(i as usize)).entries[k as usize] += (*(*src).mem_stat.add(i as usize)).entries[k as usize];
            k += 1;
        }
        i += 1;
    }
}

unsafe fn hists__clone_mem_stat(hists: *mut hists, dst: *mut hist_entry, src: *mut hist_entry) -> c_int {
    if (*hists).nr_mem_stats == 0 { return 0; }
    (*dst).mem_stat = calloc((*hists).nr_mem_stats as size_t, size_of::<mem_stat>()) as *mut mem_stat;
    if (*dst).mem_stat.is_null() { return -1; }
    let mut i = 0;
    while i < (*hists).nr_mem_stats {
        let mut k = 0;
        while k < MEM_STAT_LEN {
            (*(*dst).mem_stat.add(i as usize)).entries[k as usize] = (*(*src).mem_stat.add(i as usize)).entries[k as usize];
            k += 1;
        }
        i += 1;
    }
    0
}

unsafe fn hists__decay_mem_stat(hists: *mut hists, he: *mut hist_entry) {
    if (*hists).nr_mem_stats == 0 { return; }
    let mut i = 0;
    while i < (*hists).nr_mem_stats {
        let mut k = 0;
        while k < MEM_STAT_LEN {
            (*(*he).mem_stat.add(i as usize)).entries[k as usize] =
                ((*(*he).mem_stat.add(i as usize)).entries[k as usize] * 7) / 8;
            k += 1;
        }
        i += 1;
    }
}

unsafe fn hists__decay_entry(hists: *mut hists, he: *mut hist_entry) -> bool {
    let prev_period = (*he).stat.period;
    let prev_latency = (*he).stat.latency;
    if prev_period == 0 { return true; }
    he_stat__decay(&mut (*he).stat);
    if symbol_conf.cumulate_callchain { he_stat__decay((*he).stat_acc); }
    decay_callchain((*he).callchain);
    hists__decay_mem_stat(hists, he);
    if (*he).depth == 0 {
        let period_diff = prev_period - (*he).stat.period;
        let latency_diff = prev_latency - (*he).stat.latency;
        (*hists).stats.total_period -= period_diff;
        (*hists).stats.total_latency -= latency_diff;
        if (*he).filtered == 0 {
            (*hists).stats.total_non_filtered_period -= period_diff;
            (*hists).stats.total_non_filtered_latency -= latency_diff;
        }
    }
    if !(*he).leaf {
        let mut node = rb_first_cached(&(*he).hroot_out);
        while !node.is_null() {
            let child: *mut hist_entry = rb_entry(node, "rb_node");
            node = rb_next(node);
            if hists__decay_entry(hists, child) { hists__delete_entry(hists, child); }
        }
    }
    (*he).stat.period == 0 && (*he).stat.latency == 0
}

unsafe fn hists__delete_entry(hists: *mut hists, he: *mut hist_entry) {
    let (root_in, root_out) = if !(*he).parent_he.is_null() {
        (&mut (*(*he).parent_he).hroot_in as *mut _, &mut (*(*he).parent_he).hroot_out as *mut _)
    } else if hists__has(hists, 0) {
        (&mut (*hists).entries_collapsed as *mut _, &mut (*hists).entries as *mut _)
    } else {
        ((*hists).entries_in, &mut (*hists).entries as *mut _)
    };
    rb_erase_cached(&mut (*he).rb_node_in, root_in);
    rb_erase_cached(&mut (*he).rb_node, root_out);
    (*hists).nr_entries -= 1;
    if (*he).filtered == 0 { (*hists).nr_non_filtered_entries -= 1; }
    hist_entry__delete(he);
}

#[no_mangle]
pub unsafe extern "C" fn hists__decay_entries(hists: *mut hists, zap_user: bool, zap_kernel: bool) {
    let mut next = rb_first_cached(&(*hists).entries);
    while !next.is_null() {
        let n: *mut hist_entry = rb_entry(next, "rb_node");
        next = rb_next(&(*n).rb_node);
        if (zap_user && (*n).level == b'.' as c_char)
            || (zap_kernel && (*n).level != b'.' as c_char)
            || hists__decay_entry(hists, n)
        {
            hists__delete_entry(hists, n);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn hists__delete_entries(hists: *mut hists) {
    let mut next = rb_first_cached(&(*hists).entries);
    while !next.is_null() {
        let n: *mut hist_entry = rb_entry(next, "rb_node");
        next = rb_next(&(*n).rb_node);
        hists__delete_entry(hists, n);
    }
}

#[no_mangle]
pub unsafe extern "C" fn hists__get_entry(hists: *mut hists, idx: c_int) -> *mut hist_entry {
    let mut next = rb_first_cached(&(*hists).entries);
    let mut i = 0;
    while !next.is_null() {
        let n: *mut hist_entry = rb_entry(next, "rb_node");
        if i == idx { return n; }
        next = rb_next(&(*n).rb_node);
        i += 1;
    }
    null_mut()
}

static mut default_ops: hist_entry_ops = hist_entry_ops { new: Some(hist_entry__zalloc), free: Some(hist_entry__free) };

unsafe extern "C" fn hist_entry__zalloc(size: size_t) -> *mut c_void {
    zalloc(size + size_of::<hist_entry>())
}

unsafe extern "C" fn hist_entry__free(ptr: *mut c_void) {
    free(ptr);
}

unsafe fn hist_entry__init(he: *mut hist_entry, template: *mut hist_entry, sample_self: bool, callchain_size: size_t) -> c_int {
    memcpy(he as *mut c_void, template as *const c_void, size_of::<hist_entry>());
    (*he).callchain_size = callchain_size;
    if symbol_conf.cumulate_callchain {
        (*he).stat_acc = malloc(size_of::<he_stat>()) as *mut he_stat;
        if (*he).stat_acc.is_null() { return -ENOMEM; }
        memcpy((*he).stat_acc as *mut c_void, &(*he).stat as *const _ as *const c_void, size_of::<he_stat>());
        if !sample_self { memset(&mut (*he).stat as *mut _ as *mut c_void, 0, size_of::<he_stat>()); }
    }
    (*he).ms.thread = thread__get((*he).ms.thread);
    (*he).ms.map = map__get((*he).ms.map);
    if !(*he).branch_info.is_null() {
        (*he).branch_info = malloc(size_of::<branch_info>()) as *mut branch_info;
        if (*he).branch_info.is_null() { return -ENOMEM; }
        memcpy((*he).branch_info as *mut c_void, (*template).branch_info as *const c_void, size_of::<branch_info>());
        (*(*he).branch_info).from.ms.thread = thread__get((*(*he).branch_info).from.ms.thread);
        (*(*he).branch_info).from.ms.map = map__get((*(*he).branch_info).from.ms.map);
        (*(*he).branch_info).to.ms.thread = thread__get((*(*he).branch_info).to.ms.thread);
        (*(*he).branch_info).to.ms.map = map__get((*(*he).branch_info).to.ms.map);
    }
    if !(*he).mem_info.is_null() {
        (*he).mem_info = mem_info__clone((*template).mem_info);
        if (*he).mem_info.is_null() { return -ENOMEM; }
    }
    if hist_entry__has_callchains(he) && symbol_conf.use_callchain { callchain_init((*he).callchain); }
    if !(*he).raw_data.is_null() {
        (*he).raw_data = memdup((*he).raw_data, (*he).raw_size as size_t);
        if (*he).raw_data.is_null() { return -ENOMEM; }
    }
    if !(*he).srcline.is_null() && (*he).srcline != SRCLINE_UNKNOWN {
        (*he).srcline = strdup((*he).srcline);
        if (*he).srcline.is_null() { return -ENOMEM; }
    }
    if symbol_conf.res_sample != 0 {
        (*he).res_samples = calloc(symbol_conf.res_sample as size_t, size_of::<res_sample>()) as *mut res_sample;
        if (*he).res_samples.is_null() { return -ENOMEM; }
    }
    INIT_LIST_HEAD(&mut (*he).pairs.node);
    (*he).thread = thread__get((*he).thread);
    (*he).hroot_in = RB_ROOT_CACHED();
    (*he).hroot_out = RB_ROOT_CACHED();
    if !symbol_conf.report_hierarchy { (*he).leaf = true; }
    0
}

unsafe fn hist_entry__new(template: *mut hist_entry, sample_self: bool) -> *mut hist_entry {
    let mut ops = (*template).ops;
    let mut callchain_size = 0usize;
    if ops.is_null() {
        ops = &mut default_ops;
        (*template).ops = ops;
    }
    if symbol_conf.use_callchain { callchain_size = size_of::<callchain_root>(); }
    let he = ((*ops).new.unwrap())(callchain_size) as *mut hist_entry;
    if !he.is_null() && hist_entry__init(he, template, sample_self, callchain_size) != 0 {
        ((*ops).free.unwrap())(he as *mut c_void);
        return null_mut();
    }
    he
}

unsafe fn symbol__parent_filter(parent: *const symbol) -> filter_mask_t {
    if symbol_conf.exclude_other && parent.is_null() { 1u32 << HIST_FILTER__PARENT } else { 0 }
}

unsafe fn hist_entry__add_callchain_period(he: *mut hist_entry, period: u64, latency: u64) {
    if !hist_entry__has_callchains(he) || !symbol_conf.use_callchain { return; }
    (*(*he).hists).callchain_period += period;
    (*(*he).hists).callchain_latency += latency;
    if (*he).filtered == 0 {
        (*(*he).hists).callchain_non_filtered_period += period;
        (*(*he).hists).callchain_non_filtered_latency += latency;
    }
}

unsafe fn hist_entry__cmp(left: *mut hist_entry, right: *mut hist_entry) -> int64_t { hist_entry__cmp_impl((*(*left).hists).hpp_list, left, right, 0, true, false) }
unsafe fn hist_entry__collapse(left: *mut hist_entry, right: *mut hist_entry) -> int64_t { hist_entry__cmp_impl((*(*left).hists).hpp_list, left, right, 0, true, false) }

unsafe fn hists__findnew_entry(hists: *mut hists, entry: *mut hist_entry, al: *const addr_location, sample_self: bool) -> *mut hist_entry {
    let mut p = &mut (*(*hists).entries_in).rb_root.rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = null_mut();
    let period = (*entry).stat.period;
    let latency = (*entry).stat.latency;
    let mut leftmost = true;
    while !(*p).is_null() {
        parent = *p;
        let he: *mut hist_entry = rb_entry(parent, "rb_node_in");
        let cmp = hist_entry__cmp(he, entry);
        if cmp == 0 {
            if sample_self {
                he_stat__add_stat(&mut (*he).stat, &mut (*entry).stat);
                hist_entry__add_callchain_period(he, period, latency);
            }
            if symbol_conf.cumulate_callchain { he_stat__add_period((*he).stat_acc, period, latency); }
            block_info__delete((*entry).block_info);
            kvm_info__zput((*entry).kvm_info);
            if hists__has(hists, 1) && (*he).ms.map != (*entry).ms.map {
                if !(*he).ms.sym.is_null() {
                    let addr = (*(*he).ms.sym).start;
                    (*he).ms.sym = map__find_symbol((*entry).ms.map, addr);
                }
                map__put((*he).ms.map);
                (*he).ms.map = map__get((*entry).ms.map);
            }
            if sample_self { he_stat__add_cpumode_period(&mut (*he).stat, (*al).cpumode, period); }
            if symbol_conf.cumulate_callchain { he_stat__add_cpumode_period((*he).stat_acc, (*al).cpumode, period); }
            if hists__update_mem_stat(hists, he, (*entry).mem_info, period) < 0 {
                hist_entry__delete(he);
                return null_mut();
            }
            return he;
        }
        if cmp < 0 {
            p = &mut (**p).rb_left;
        } else {
            p = &mut (**p).rb_right;
            leftmost = false;
        }
    }
    let he = hist_entry__new(entry, sample_self);
    if he.is_null() { return null_mut(); }
    if sample_self { hist_entry__add_callchain_period(he, period, latency); }
    (*hists).nr_entries += 1;
    rb_link_node(&mut (*he).rb_node_in, parent, p);
    rb_insert_color_cached(&mut (*he).rb_node_in, (*hists).entries_in, leftmost);
    if sample_self { he_stat__add_cpumode_period(&mut (*he).stat, (*al).cpumode, period); }
    if symbol_conf.cumulate_callchain { he_stat__add_cpumode_period((*he).stat_acc, (*al).cpumode, period); }
    if hists__update_mem_stat(hists, he, (*entry).mem_info, period) < 0 {
        hist_entry__delete(he);
        return null_mut();
    }
    he
}

unsafe fn random_max(high: c_uint) -> c_uint {
    let thresh = high.wrapping_neg() % high;
    loop {
        let r = random() as c_uint;
        if r >= thresh { return r % high; }
    }
}

unsafe fn hists__res_sample(he: *mut hist_entry, sample: *mut perf_sample) {
    let j = if (*he).num_res < symbol_conf.res_sample {
        let j = (*he).num_res;
        (*he).num_res += 1;
        j
    } else {
        random_max(symbol_conf.res_sample as c_uint) as c_int
    };
    let r = (*he).res_samples.add(j as usize);
    (*r).time = (*sample).time;
    (*r).cpu = (*sample).cpu;
    (*r).tid = (*sample).tid;
}

#[no_mangle]
pub unsafe extern "C" fn hists__add_entry(hists: *mut hists, al: *mut addr_location, sym_parent: *mut symbol, bi: *mut branch_info, mi: *mut mem_info, ki: *mut kvm_info, sample: *mut perf_sample, sample_self: bool) -> *mut hist_entry {
    __hists__add_entry(hists, al, sym_parent, bi, mi, ki, null_mut(), sample, sample_self, null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn hists__add_entry_ops(hists: *mut hists, ops: *mut hist_entry_ops, al: *mut addr_location, sym_parent: *mut symbol, bi: *mut branch_info, mi: *mut mem_info, ki: *mut kvm_info, sample: *mut perf_sample, sample_self: bool) -> *mut hist_entry {
    __hists__add_entry(hists, al, sym_parent, bi, mi, ki, null_mut(), sample, sample_self, ops)
}

unsafe fn __hists__add_entry(hists: *mut hists, al: *mut addr_location, sym_parent: *mut symbol, bi: *mut branch_info, mi: *mut mem_info, ki: *mut kvm_info, block_info: *mut block_info, sample: *mut perf_sample, sample_self: bool, ops: *mut hist_entry_ops) -> *mut hist_entry {
    let mut entry: hist_entry = zeroed();
    entry.thread = (*al).thread;
    entry.comm = thread__comm((*al).thread);
    entry.cgroup = (*sample).cgroup;
    entry.ms.thread = (*al).thread;
    entry.ms.map = (*al).map;
    entry.ms.sym = (*al).sym;
    entry.srcline = (*al).srcline as *mut c_char;
    entry.socket = (*al).socket;
    entry.cpu = (*al).cpu;
    entry.cpumode = (*al).cpumode;
    entry.ip = (*al).addr;
    entry.level = (*al).level;
    entry.code_page_size = (*sample).code_page_size;
    entry.parallelism = (*al).parallelism;
    entry.stat.nr_events = 1;
    entry.stat.period = (*sample).period;
    entry.stat.weight1 = (*sample).weight;
    entry.stat.weight2 = (*sample).ins_lat;
    entry.stat.weight3 = (*sample).weight3;
    entry.stat.latency = (*al).latency;
    entry.parent = sym_parent;
    entry.filtered = symbol__parent_filter(sym_parent) | (*al).filtered;
    entry.hists = hists;
    entry.branch_info = bi;
    entry.mem_info = mi;
    entry.kvm_info = ki;
    entry.block_info = block_info;
    entry.transaction = (*sample).transaction;
    entry.raw_data = (*sample).raw_data;
    entry.raw_size = (*sample).raw_size;
    entry.ops = ops;
    entry.time = hist_time((*sample).time);
    entry.weight = (*sample).weight;
    entry.ins_lat = (*sample).ins_lat;
    entry.weight3 = (*sample).weight3;
    entry.simd_flags = (*sample).simd_flags;
    let he = hists__findnew_entry(hists, &mut entry, al, sample_self);
    if !(*hists).has_callchains && !he.is_null() && (*he).callchain_size != 0 { (*hists).has_callchains = true; }
    if !he.is_null() && symbol_conf.res_sample != 0 { hists__res_sample(he, sample); }
    he
}

#[no_mangle]
pub unsafe extern "C" fn hists__add_entry_block(hists: *mut hists, al: *mut addr_location, block_info: *mut block_info) -> *mut hist_entry {
    let mut entry: hist_entry = zeroed();
    entry.block_info = block_info;
    entry.hists = hists;
    entry.ms.thread = (*al).thread;
    entry.ms.map = (*al).map;
    entry.ms.sym = (*al).sym;
    hists__findnew_entry(hists, &mut entry, al, false)
}

unsafe extern "C" fn iter_next_nop_entry(_iter: *mut hist_entry_iter, _al: *mut addr_location) -> c_int { 0 }
unsafe extern "C" fn iter_add_next_nop_entry(_iter: *mut hist_entry_iter, _al: *mut addr_location) -> c_int { 0 }
unsafe extern "C" fn iter_prepare_normal_entry(_iter: *mut hist_entry_iter, _al: *mut addr_location) -> c_int { 0 }

unsafe extern "C" fn iter_prepare_mem_entry(iter: *mut hist_entry_iter, al: *mut addr_location) -> c_int {
    let mi = sample__resolve_mem((*iter).sample, al);
    if mi.is_null() { return -ENOMEM; }
    (*iter).mi = mi;
    0
}

unsafe extern "C" fn iter_add_single_mem_entry(iter: *mut hist_entry_iter, al: *mut addr_location) -> c_int {
    let mi = (*iter).mi;
    let sample = (*iter).sample;
    let hists = evsel__hists((*sample).evsel);
    if mi.is_null() { return -EINVAL; }
    let cost = if (*sample).weight != 0 { (*sample).weight } else { 1 };
    (*sample).period = cost;
    let he = hists__add_entry(hists, al, (*iter).parent, null_mut(), mi, null_mut(), sample, true);
    if he.is_null() { return -ENOMEM; }
    (*iter).he = he;
    0
}

unsafe extern "C" fn iter_finish_mem_entry(iter: *mut hist_entry_iter, _al: *mut addr_location) -> c_int {
    let hists = evsel__hists((*(*iter).sample).evsel);
    let he = (*iter).he;
    let mut err = -EINVAL;
    if !he.is_null() {
        hists__inc_nr_samples(hists, (*he).filtered != 0);
        err = hist_entry__append_callchain(he, (*iter).sample);
    }
    mem_info__zput((*iter).mi);
    (*iter).he = null_mut();
    err
}

unsafe extern "C" fn iter_prepare_branch_entry(iter: *mut hist_entry_iter, al: *mut addr_location) -> c_int {
    let sample = (*iter).sample;
    let bi = sample__resolve_bstack(sample, al);
    if bi.is_null() { return -ENOMEM; }
    (*iter).curr = 0;
    (*iter).total = (*(*sample).branch_stack).nr as c_int;
    (*iter).bi = bi;
    0
}

unsafe extern "C" fn iter_add_single_branch_entry(_iter: *mut hist_entry_iter, _al: *mut addr_location) -> c_int { 0 }

unsafe extern "C" fn iter_next_branch_entry(iter: *mut hist_entry_iter, al: *mut addr_location) -> c_int {
    let bi = (*iter).bi;
    let i = (*iter).curr;
    if bi.is_null() || (*iter).curr >= (*iter).total { return 0; }
    thread__put((*al).thread);
    (*al).thread = thread__get((*bi.add(i as usize)).to.ms.thread);
    map__put((*al).map);
    (*al).map = map__get((*bi.add(i as usize)).to.ms.map);
    (*al).sym = (*bi.add(i as usize)).to.ms.sym;
    (*al).addr = (*bi.add(i as usize)).to.addr;
    1
}

unsafe extern "C" fn iter_add_next_branch_entry(iter: *mut hist_entry_iter, al: *mut addr_location) -> c_int {
    let bi = (*iter).bi;
    let sample = (*iter).sample;
    let hists = evsel__hists((*sample).evsel);
    let i = (*iter).curr;
    let mut he: *mut hist_entry = null_mut();
    if !((*iter).hide_unresolved && ((*bi.add(i as usize)).from.ms.sym.is_null() || (*bi.add(i as usize)).to.ms.sym.is_null())) {
        (*sample).period = 1;
        (*sample).weight = if (*bi).flags.cycles != 0 { (*bi).flags.cycles } else { 1 };
        he = hists__add_entry(hists, al, (*iter).parent, bi.add(i as usize), null_mut(), null_mut(), sample, true);
        if he.is_null() { return -ENOMEM; }
    }
    (*iter).he = he;
    (*iter).curr += 1;
    0
}

unsafe fn branch_info__exit(bi: *mut branch_info) {
    map_symbol__exit(&mut (*bi).from.ms);
    map_symbol__exit(&mut (*bi).to.ms);
    zfree_srcline(&mut (*bi).srcline_from);
    zfree_srcline(&mut (*bi).srcline_to);
}

unsafe extern "C" fn iter_finish_branch_entry(iter: *mut hist_entry_iter, _al: *mut addr_location) -> c_int {
    let hists = evsel__hists((*(*iter).sample).evsel);
    let mut i = 0;
    while i < (*iter).total {
        branch_info__exit((*iter).bi.add(i as usize));
        i += 1;
    }
    if !(*iter).he.is_null() { hists__inc_nr_samples(hists, (*(*iter).he).filtered != 0); }
    zfree(&mut (*iter).bi as *mut _ as *mut *mut c_void);
    (*iter).he = null_mut();
    if (*iter).curr >= (*iter).total { 0 } else { -1 }
}

unsafe extern "C" fn iter_add_single_normal_entry(iter: *mut hist_entry_iter, al: *mut addr_location) -> c_int {
    let sample = (*iter).sample;
    let he = hists__add_entry(evsel__hists((*sample).evsel), al, (*iter).parent, null_mut(), null_mut(), null_mut(), sample, true);
    if he.is_null() { return -ENOMEM; }
    (*iter).he = he;
    0
}

unsafe extern "C" fn iter_finish_normal_entry(iter: *mut hist_entry_iter, _al: *mut addr_location) -> c_int {
    let he = (*iter).he;
    let sample = (*iter).sample;
    if he.is_null() { return 0; }
    (*iter).he = null_mut();
    hists__inc_nr_samples(evsel__hists((*sample).evsel), (*he).filtered != 0);
    hist_entry__append_callchain(he, sample)
}

unsafe extern "C" fn iter_prepare_cumulative_entry(iter: *mut hist_entry_iter, _al: *mut addr_location) -> c_int {
    let cursor = get_tls_callchain_cursor();
    if cursor.is_null() { return -ENOMEM; }
    callchain_cursor_commit(cursor);
    let he_cache = calloc(((*cursor).nr + 1) as size_t, size_of::<*mut hist_entry>()) as *mut *mut hist_entry;
    if he_cache.is_null() { return -ENOMEM; }
    (*iter).he_cache = he_cache;
    (*iter).curr = 0;
    0
}

unsafe extern "C" fn iter_add_single_cumulative_entry(iter: *mut hist_entry_iter, al: *mut addr_location) -> c_int {
    let sample = (*iter).sample;
    let hists = evsel__hists((*sample).evsel);
    let he = hists__add_entry(hists, al, (*iter).parent, null_mut(), null_mut(), null_mut(), sample, true);
    if he.is_null() { return -ENOMEM; }
    (*iter).he = he;
    *(*iter).he_cache.add((*iter).curr as usize) = he;
    (*iter).curr += 1;
    hist_entry__append_callchain(he, sample);
    callchain_cursor_commit(get_tls_callchain_cursor());
    hists__inc_nr_samples(hists, (*he).filtered != 0);
    0
}

unsafe extern "C" fn iter_next_cumulative_entry(iter: *mut hist_entry_iter, al: *mut addr_location) -> c_int {
    let node = callchain_cursor_current(get_tls_callchain_cursor());
    if node.is_null() { return 0; }
    fill_callchain_info(al, node, (*iter).hide_unresolved)
}

unsafe fn hist_entry__fast__sym_diff(left: *mut hist_entry, right: *mut hist_entry) -> bool {
    let sym_l = (*left).ms.sym;
    let sym_r = (*right).ms.sym;
    if sym_l.is_null() && sym_r.is_null() { return (*left).ip != (*right).ip; }
    _sort__sym_cmp(sym_l, sym_r) != 0
}

unsafe extern "C" fn iter_add_next_cumulative_entry(iter: *mut hist_entry_iter, al: *mut addr_location) -> c_int {
    let sample = (*iter).sample;
    let evsel = (*sample).evsel;
    let he_cache = (*iter).he_cache;
    let mut he_tmp: hist_entry = zeroed();
    he_tmp.hists = evsel__hists(evsel);
    he_tmp.cpu = (*al).cpu;
    he_tmp.thread = (*al).thread;
    he_tmp.comm = thread__comm((*al).thread);
    he_tmp.ip = (*al).addr;
    he_tmp.ms.thread = (*al).thread;
    he_tmp.ms.map = (*al).map;
    he_tmp.ms.sym = (*al).sym;
    he_tmp.srcline = (*al).srcline as *mut c_char;
    he_tmp.parent = (*iter).parent;
    he_tmp.raw_data = (*sample).raw_data;
    he_tmp.raw_size = (*sample).raw_size;
    let tls_cursor = get_tls_callchain_cursor();
    if tls_cursor.is_null() { return -ENOMEM; }
    let mut cursor: callchain_cursor = zeroed();
    callchain_cursor_snapshot(&mut cursor, tls_cursor);
    callchain_cursor_advance(tls_cursor);
    let fast = hists__has(he_tmp.hists, 1);
    let mut i = 0;
    while i < (*iter).curr {
        if fast && hist_entry__fast__sym_diff(*he_cache.add(i as usize), &mut he_tmp) {
            i += 1;
            continue;
        }
        if hist_entry__cmp(*he_cache.add(i as usize), &mut he_tmp) == 0 {
            (*iter).he = null_mut();
            return 0;
        }
        i += 1;
    }
    let he = hists__add_entry(evsel__hists(evsel), al, (*iter).parent, null_mut(), null_mut(), null_mut(), sample, false);
    if he.is_null() { return -ENOMEM; }
    (*iter).he = he;
    *he_cache.add((*iter).curr as usize) = he;
    (*iter).curr += 1;
    if hist_entry__has_callchains(he) && symbol_conf.use_callchain { callchain_append((*he).callchain, &mut cursor, (*sample).period); }
    0
}

unsafe extern "C" fn iter_finish_cumulative_entry(iter: *mut hist_entry_iter, _al: *mut addr_location) -> c_int {
    mem_info__zput((*iter).mi);
    zfree(&mut (*iter).bi as *mut _ as *mut *mut c_void);
    zfree(&mut (*iter).he_cache as *mut _ as *mut *mut c_void);
    (*iter).he = null_mut();
    0
}

#[no_mangle] pub static hist_iter_mem: hist_iter_ops = hist_iter_ops { prepare_entry: Some(iter_prepare_mem_entry), add_single_entry: Some(iter_add_single_mem_entry), next_entry: Some(iter_next_nop_entry), add_next_entry: Some(iter_add_next_nop_entry), finish_entry: Some(iter_finish_mem_entry) };
#[no_mangle] pub static hist_iter_branch: hist_iter_ops = hist_iter_ops { prepare_entry: Some(iter_prepare_branch_entry), add_single_entry: Some(iter_add_single_branch_entry), next_entry: Some(iter_next_branch_entry), add_next_entry: Some(iter_add_next_branch_entry), finish_entry: Some(iter_finish_branch_entry) };
#[no_mangle] pub static hist_iter_normal: hist_iter_ops = hist_iter_ops { prepare_entry: Some(iter_prepare_normal_entry), add_single_entry: Some(iter_add_single_normal_entry), next_entry: Some(iter_next_nop_entry), add_next_entry: Some(iter_add_next_nop_entry), finish_entry: Some(iter_finish_normal_entry) };
#[no_mangle] pub static hist_iter_cumulative: hist_iter_ops = hist_iter_ops { prepare_entry: Some(iter_prepare_cumulative_entry), add_single_entry: Some(iter_add_single_cumulative_entry), next_entry: Some(iter_next_cumulative_entry), add_next_entry: Some(iter_add_next_cumulative_entry), finish_entry: Some(iter_finish_cumulative_entry) };

#[no_mangle]
pub unsafe extern "C" fn hist_entry_iter__add(iter: *mut hist_entry_iter, al: *mut addr_location, max_stack_depth: c_int, arg: *mut c_void) -> c_int {
    let alm = if !al.is_null() { map__get((*al).map) } else { null_mut() };
    let mut err = sample__resolve_callchain((*iter).sample, get_tls_callchain_cursor(), &mut (*iter).parent, al, max_stack_depth);
    if err != 0 { map__put(alm); return err; }
    err = ((*(*iter).ops).prepare_entry.unwrap())(iter, al);
    if err == 0 { err = ((*(*iter).ops).add_single_entry.unwrap())(iter, al); }
    if err == 0 && !(*iter).he.is_null() {
        if let Some(cb) = (*iter).add_entry_cb { err = cb(iter, al, true, arg); }
    }
    while err == 0 && ((*(*iter).ops).next_entry.unwrap())(iter, al) != 0 {
        err = ((*(*iter).ops).add_next_entry.unwrap())(iter, al);
        if err != 0 { break; }
        if !(*iter).he.is_null() {
            if let Some(cb) = (*iter).add_entry_cb { err = cb(iter, al, false, arg); if err != 0 { break; } }
        }
    }
    let err2 = ((*(*iter).ops).finish_entry.unwrap())(iter, al);
    if err == 0 { err = err2; }
    map__put(alm);
    err
}

unsafe fn hist_entry__cmp_impl(_hpp_list: *mut perf_hpp_list, left: *mut hist_entry, right: *mut hist_entry, _fn_offset: c_ulong, _ignore_dynamic: bool, _ignore_skipped: bool) -> int64_t {
    let cmp = ((*left).filtered != 0) as int64_t - ((*right).filtered != 0) as int64_t;
    if cmp != 0 { return cmp; }
    0
}

unsafe fn hist_entry__sort(left: *mut hist_entry, right: *mut hist_entry) -> int64_t {
    hist_entry__cmp_impl((*(*left).hists).hpp_list, left, right, 0, false, true)
}

unsafe fn hist_entry__collapse_hierarchy(hpp_list: *mut perf_hpp_list, left: *mut hist_entry, right: *mut hist_entry) -> int64_t {
    hist_entry__cmp_impl(hpp_list, left, right, 0, false, false)
}

#[no_mangle]
pub unsafe extern "C" fn hist_entry__delete(he: *mut hist_entry) {
    let ops = (*he).ops;
    thread__zput((*he).thread);
    map_symbol__exit(&mut (*he).ms);
    if !(*he).branch_info.is_null() { branch_info__exit((*he).branch_info); zfree(&mut (*he).branch_info as *mut _ as *mut *mut c_void); }
    if !(*he).mem_info.is_null() {
        map_symbol__exit(&mut (*mem_info__iaddr((*he).mem_info)).ms);
        map_symbol__exit(&mut (*mem_info__daddr((*he).mem_info)).ms);
        mem_info__zput((*he).mem_info);
    }
    if !(*he).block_info.is_null() { block_info__delete((*he).block_info); }
    if !(*he).kvm_info.is_null() { kvm_info__zput((*he).kvm_info); }
    zfree(&mut (*he).res_samples as *mut _ as *mut *mut c_void);
    zfree(&mut (*he).stat_acc as *mut _ as *mut *mut c_void);
    zfree_srcline(&mut (*he).srcline);
    if !(*he).srcfile.is_null() && *(*he).srcfile != 0 { zfree(&mut (*he).srcfile as *mut _ as *mut *mut c_void); }
    free_callchain((*he).callchain);
    zfree(&mut (*he).trace_output as *mut _ as *mut *mut c_void);
    zfree(&mut (*he).raw_data);
    zfree(&mut (*he).mem_stat as *mut _ as *mut *mut c_void);
    ((*ops).free.unwrap())(he as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn hist_entry__snprintf_alignment(he: *mut hist_entry, hpp: *mut perf_hpp, fmt: *mut perf_hpp_fmt, printed: c_int) -> c_int {
    let mut printed = printed;
    if !list_is_last(&(*fmt).list, &(*(*he).hists).hpp_list.as_ref().unwrap().fields) {
        let width = ((*fmt).width.unwrap())(fmt, hpp, (*he).hists);
        if printed < width {
            advance_hpp(hpp, printed);
            printed = scnprintf((*hpp).buf, (*hpp).size, b"%-*s\0".as_ptr() as *const c_char, width - printed, b" \0".as_ptr() as *const c_char);
        }
    }
    printed
}

unsafe fn check_thread_entry(fmt: *mut perf_hpp_fmt) -> bool {
    perf_hpp__is_thread_entry(fmt) || perf_hpp__is_comm_entry(fmt)
}

unsafe fn hist_entry__check_and_remove_filter(he: *mut hist_entry, ty: c_int, check: fmt_chk_fn) {
    let mut parent = (*he).parent_he;
    match ty {
        HIST_FILTER__THREAD => if symbol_conf.comm_list.is_null() && symbol_conf.pid_list.is_null() && symbol_conf.tid_list.is_null() { return; },
        HIST_FILTER__DSO => if symbol_conf.dso_list.is_null() { return; },
        HIST_FILTER__SYMBOL => if symbol_conf.sym_list.is_null() { return; },
        HIST_FILTER__PARALLELISM => if __bitmap_weight(symbol_conf.parallelism_filter, MAX_NR_CPUS + 1) == 0 { return; },
        _ => return,
    }
    let type_match = false; /* perf_hpp_list__for_each_format is supplied externally in C. */
    if type_match {
        if ((*he).filtered & (1u32 << ty)) == 0 {
            while !parent.is_null() {
                (*parent).filtered &= !(1u32 << ty);
                parent = (*parent).parent_he;
            }
        }
    } else if parent.is_null() {
        (*he).filtered |= 1u32 << ty;
    } else {
        (*he).filtered |= (*parent).filtered & (1u32 << ty);
    }
    let _ = check;
}

unsafe fn hist_entry__apply_hierarchy_filters(he: *mut hist_entry) {
    hist_entry__check_and_remove_filter(he, HIST_FILTER__THREAD, Some(check_thread_entry));
    hist_entry__check_and_remove_filter(he, HIST_FILTER__DSO, Some(perf_hpp__is_dso_entry));
    hist_entry__check_and_remove_filter(he, HIST_FILTER__SYMBOL, Some(perf_hpp__is_sym_entry));
    hist_entry__check_and_remove_filter(he, HIST_FILTER__PARALLELISM, Some(perf_hpp__is_parallelism_entry));
    hists__apply_filters((*he).hists, he);
}

unsafe fn hists__apply_filters(hists: *mut hists, he: *mut hist_entry) {
    hists__filter_entry_by_dso(hists, he);
    hists__filter_entry_by_thread(hists, he);
    hists__filter_entry_by_symbol(hists, he);
    hists__filter_entry_by_socket(hists, he);
    hists__filter_entry_by_parallelism(hists, he);
}

#[no_mangle]
pub unsafe extern "C" fn hists__collapse_resort(hists: *mut hists, prog: *mut ui_progress) -> c_int {
    if !hists__has(hists, 0) { return 0; }
    (*hists).nr_entries = 0;
    let root = hists__get_rotate_entries_in(hists);
    let mut next = rb_first_cached(root);
    while !next.is_null() {
        if session_done() { break; }
        let n: *mut hist_entry = rb_entry(next, "rb_node_in");
        next = rb_next(&(*n).rb_node_in);
        rb_erase_cached(&mut (*n).rb_node_in, root);
        let ret = hists__collapse_insert_entry(hists, &mut (*hists).entries_collapsed, n);
        if ret < 0 { return -1; }
        if ret != 0 { hists__apply_filters(hists, n); }
        if !prog.is_null() { ui_progress__update(prog, 1); }
    }
    0
}

unsafe fn hists__collapse_insert_entry(hists: *mut hists, root: *mut rb_root_cached, he: *mut hist_entry) -> c_int {
    let mut p = &mut (*root).rb_root.rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = null_mut();
    let mut leftmost = true;
    while !(*p).is_null() {
        parent = *p;
        let iter: *mut hist_entry = rb_entry(parent, "rb_node_in");
        let cmp = hist_entry__collapse(iter, he);
        if cmp == 0 {
            he_stat__add_stat(&mut (*iter).stat, &mut (*he).stat);
            if symbol_conf.cumulate_callchain { he_stat__add_stat((*iter).stat_acc, (*he).stat_acc); }
            hists__add_mem_stat(hists, iter, he);
            hist_entry__delete(he);
            return 0;
        }
        if cmp < 0 { p = &mut (**p).rb_left; } else { p = &mut (**p).rb_right; leftmost = false; }
    }
    (*hists).nr_entries += 1;
    rb_link_node(&mut (*he).rb_node_in, parent, p);
    rb_insert_color_cached(&mut (*he).rb_node_in, root, leftmost);
    1
}

#[no_mangle]
pub unsafe extern "C" fn hists__get_rotate_entries_in(hists: *mut hists) -> *mut rb_root_cached {
    mutex_lock(&mut (*hists).lock);
    let root = (*hists).entries_in;
    if (*hists).entries_in == &mut (*hists).entries_in_array[0] {
        (*hists).entries_in = &mut (*hists).entries_in_array[1];
    } else {
        (*hists).entries_in = &mut (*hists).entries_in_array[0];
    }
    mutex_unlock(&mut (*hists).lock);
    root
}

unsafe fn hists__reset_filter_stats(hists: *mut hists) {
    (*hists).nr_non_filtered_entries = 0;
    (*hists).stats.total_non_filtered_period = 0;
    (*hists).stats.total_non_filtered_latency = 0;
}

#[no_mangle]
pub unsafe extern "C" fn hists__reset_stats(hists: *mut hists) {
    (*hists).nr_entries = 0;
    (*hists).stats.total_period = 0;
    (*hists).stats.total_latency = 0;
    hists__reset_filter_stats(hists);
}

unsafe fn hists__inc_filter_stats(hists: *mut hists, h: *mut hist_entry) {
    (*hists).nr_non_filtered_entries += 1;
    (*hists).stats.total_non_filtered_period += (*h).stat.period;
    (*hists).stats.total_non_filtered_latency += (*h).stat.latency;
}

#[no_mangle]
pub unsafe extern "C" fn hists__inc_stats(hists: *mut hists, h: *mut hist_entry) {
    if (*h).filtered == 0 { hists__inc_filter_stats(hists, h); }
    (*hists).nr_entries += 1;
    (*hists).stats.total_period += (*h).stat.period;
    (*hists).stats.total_latency += (*h).stat.latency;
}

unsafe fn hierarchy_recalc_total_periods(hists: *mut hists) {
    let mut node = rb_first_cached(&(*hists).entries);
    (*hists).stats.total_period = 0;
    (*hists).stats.total_non_filtered_period = 0;
    (*hists).stats.total_latency = 0;
    (*hists).stats.total_non_filtered_latency = 0;
    while !node.is_null() {
        let he: *mut hist_entry = rb_entry(node, "rb_node");
        node = rb_next(node);
        (*hists).stats.total_period += (*he).stat.period;
        (*hists).stats.total_latency += (*he).stat.latency;
        if (*he).filtered == 0 {
            (*hists).stats.total_non_filtered_period += (*he).stat.period;
            (*hists).stats.total_non_filtered_latency += (*he).stat.latency;
        }
    }
}

unsafe fn __hists__insert_output_entry(entries: *mut rb_root_cached, he: *mut hist_entry, mut min_callchain_hits: u64, use_callchain: bool) {
    if use_callchain {
        if callchain_param.mode == CHAIN_GRAPH_REL {
            let mut total = (*he).stat.period;
            if symbol_conf.cumulate_callchain { total = (*(*he).stat_acc).period; }
            min_callchain_hits = total * (callchain_param.min_percent / 100);
        }
        if let Some(sort) = callchain_param.sort { sort(&mut (*he).sorted_chain, (*he).callchain, min_callchain_hits, &mut callchain_param); }
    }
    let mut p = &mut (*entries).rb_root.rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = null_mut();
    let mut leftmost = true;
    while !(*p).is_null() {
        parent = *p;
        let iter: *mut hist_entry = rb_entry(parent, "rb_node");
        if hist_entry__sort(he, iter) > 0 { p = &mut (**p).rb_left; } else { p = &mut (**p).rb_right; leftmost = false; }
    }
    rb_link_node(&mut (*he).rb_node, parent, p);
    rb_insert_color_cached(&mut (*he).rb_node, entries, leftmost);
}

unsafe fn output_resort(hists: *mut hists, prog: *mut ui_progress, use_callchain: bool, cb: hists__resort_cb_t, cb_arg: *mut c_void) {
    let callchain_total = if symbol_conf.filter_relative { (*hists).callchain_non_filtered_period } else { (*hists).callchain_period };
    let min_callchain_hits = callchain_total * (callchain_param.min_percent / 100);
    hists__reset_stats(hists);
    hists__reset_col_len(hists);
    let root = if hists__has(hists, 0) { &mut (*hists).entries_collapsed } else { (*hists).entries_in };
    let mut next = rb_first_cached(root);
    (*hists).entries = RB_ROOT_CACHED();
    while !next.is_null() {
        let n: *mut hist_entry = rb_entry(next, "rb_node_in");
        next = rb_next(&(*n).rb_node_in);
        if let Some(cb) = cb { if cb(n, cb_arg) { continue; } }
        __hists__insert_output_entry(&mut (*hists).entries, n, min_callchain_hits, use_callchain);
        hists__inc_stats(hists, n);
        if (*n).filtered == 0 { hists__calc_col_len(hists, n); }
        if !prog.is_null() { ui_progress__update(prog, 1); }
    }
}

#[no_mangle]
pub unsafe extern "C" fn evsel__output_resort_cb(evsel: *mut evsel, prog: *mut ui_progress, cb: hists__resort_cb_t, cb_arg: *mut c_void) {
    let mut use_callchain = if !evsel.is_null() && symbol_conf.use_callchain && !symbol_conf.show_ref_callgraph {
        evsel__has_callchain(evsel)
    } else {
        symbol_conf.use_callchain
    };
    use_callchain |= symbol_conf.show_branchflag_count;
    output_resort(evsel__hists(evsel), prog, use_callchain, cb, cb_arg);
}

#[no_mangle] pub unsafe extern "C" fn evsel__output_resort(evsel: *mut evsel, prog: *mut ui_progress) { evsel__output_resort_cb(evsel, prog, None, null_mut()); }
#[no_mangle] pub unsafe extern "C" fn hists__output_resort(hists: *mut hists, prog: *mut ui_progress) { output_resort(hists, prog, symbol_conf.use_callchain, None, null_mut()); }
#[no_mangle] pub unsafe extern "C" fn hists__output_resort_cb(hists: *mut hists, prog: *mut ui_progress, cb: hists__resort_cb_t) { output_resort(hists, prog, symbol_conf.use_callchain, cb, null_mut()); }

unsafe fn can_goto_child(he: *mut hist_entry, hmd: hierarchy_move_dir) -> bool {
    if (*he).leaf || hmd == HMD_FORCE_SIBLING { return false; }
    if (*he).unfolded || hmd == HMD_FORCE_CHILD { return true; }
    false
}

#[no_mangle]
pub unsafe extern "C" fn rb_hierarchy_last(mut node: *mut rb_node) -> *mut rb_node {
    let mut he: *mut hist_entry = rb_entry(node, "rb_node");
    while can_goto_child(he, HMD_NORMAL) {
        node = rb_last(&(*he).hroot_out.rb_root);
        he = rb_entry(node, "rb_node");
    }
    node
}

#[no_mangle]
pub unsafe extern "C" fn __rb_hierarchy_next(mut node: *mut rb_node, hmd: hierarchy_move_dir) -> *mut rb_node {
    let mut he: *mut hist_entry = rb_entry(node, "rb_node");
    if can_goto_child(he, hmd) { node = rb_first_cached(&(*he).hroot_out); } else { node = rb_next(node); }
    while node.is_null() {
        he = (*he).parent_he;
        if he.is_null() { break; }
        node = rb_next(&(*he).rb_node);
    }
    node
}

#[no_mangle]
pub unsafe extern "C" fn rb_hierarchy_prev(mut node: *mut rb_node) -> *mut rb_node {
    let mut he: *mut hist_entry = rb_entry(node, "rb_node");
    node = rb_prev(node);
    if !node.is_null() { return rb_hierarchy_last(node); }
    he = (*he).parent_he;
    if he.is_null() { return null_mut(); }
    &mut (*he).rb_node
}

#[no_mangle]
pub unsafe extern "C" fn hist_entry__has_hierarchy_children(he: *mut hist_entry, limit: f32) -> bool {
    if (*he).leaf { return false; }
    let mut node = rb_first_cached(&(*he).hroot_out);
    if node.is_null() { return false; }
    let mut child: *mut hist_entry = rb_entry(node, "rb_node");
    while !node.is_null() && (*child).filtered != 0 {
        node = rb_next(node);
        if !node.is_null() { child = rb_entry(node, "rb_node"); }
    }
    let percent = if !node.is_null() { hist_entry__get_percent_limit(child) } else { 0.0 };
    !node.is_null() && percent >= limit
}

unsafe fn hists__remove_entry_filter(hists: *mut hists, h: *mut hist_entry, filter: c_int) {
    (*h).filtered &= !(1u32 << filter);
    if symbol_conf.report_hierarchy {
        let mut parent = (*h).parent_he;
        while !parent.is_null() {
            he_stat__add_stat(&mut (*parent).stat, &mut (*h).stat);
            (*parent).filtered &= !(1u32 << filter);
            if (*parent).filtered == 0 {
                (*parent).unfolded = false;
                (*parent).has_no_entry = false;
                (*parent).row_offset = 0;
                (*parent).nr_rows = 0;
            }
            parent = (*parent).parent_he;
        }
    }
    if (*h).filtered != 0 { return; }
    (*h).unfolded = false;
    (*h).has_no_entry = false;
    (*h).row_offset = 0;
    (*h).nr_rows = 0;
    (*hists).stats.nr_non_filtered_samples += (*h).stat.nr_events;
    hists__inc_filter_stats(hists, h);
    hists__calc_col_len(hists, h);
}

unsafe fn hists__filter_entry_by_dso(hists: *mut hists, he: *mut hist_entry) -> bool {
    if !(*hists).dso_filter.is_null() && ((*he).ms.map.is_null() || !RC_CHK_EQUAL(map__dso((*he).ms.map) as *mut c_void, (*hists).dso_filter as *mut c_void)) {
        (*he).filtered |= 1u32 << HIST_FILTER__DSO;
        return true;
    }
    false
}

unsafe fn hists__filter_entry_by_thread(hists: *mut hists, he: *mut hist_entry) -> bool {
    if !(*hists).thread_filter.is_null() && !RC_CHK_EQUAL((*he).thread as *mut c_void, (*hists).thread_filter as *mut c_void) {
        (*he).filtered |= 1u32 << HIST_FILTER__THREAD;
        return true;
    }
    false
}

unsafe fn hists__filter_entry_by_symbol(hists: *mut hists, he: *mut hist_entry) -> bool {
    if !(*hists).symbol_filter_str.is_null() && ((*he).ms.sym.is_null() || strstr((*(*he).ms.sym).name, (*hists).symbol_filter_str).is_null()) {
        (*he).filtered |= 1u32 << HIST_FILTER__SYMBOL;
        return true;
    }
    false
}

unsafe fn hists__filter_entry_by_socket(hists: *mut hists, he: *mut hist_entry) -> bool {
    if (*hists).socket_filter > -1 && (*he).socket != (*hists).socket_filter {
        (*he).filtered |= 1u32 << HIST_FILTER__SOCKET;
        return true;
    }
    false
}

unsafe fn hists__filter_entry_by_parallelism(hists: *mut hists, he: *mut hist_entry) -> bool {
    if test_bit((*he).parallelism, (*hists).parallelism_filter) {
        (*he).filtered |= 1u32 << HIST_FILTER__PARALLELISM;
        return true;
    }
    false
}

unsafe fn hists__filter_by_type(hists: *mut hists, ty: c_int, filter: filter_fn_t) {
    (*hists).stats.nr_non_filtered_samples = 0;
    hists__reset_filter_stats(hists);
    hists__reset_col_len(hists);
    let mut nd = rb_first_cached(&(*hists).entries);
    while !nd.is_null() {
        let h: *mut hist_entry = rb_entry(nd, "rb_node");
        if !filter.unwrap()(hists, h) { hists__remove_entry_filter(hists, h, ty); }
        nd = rb_next(nd);
    }
}

unsafe fn hists__filter_hierarchy(hists: *mut hists, ty: c_int, arg: *const c_void) {
    (*hists).stats.nr_non_filtered_samples = 0;
    hists__reset_filter_stats(hists);
    hists__reset_col_len(hists);
    let mut nd = rb_first_cached(&(*hists).entries);
    while !nd.is_null() {
        let h: *mut hist_entry = rb_entry(nd, "rb_node");
        let ret = hist_entry__filter(h, ty, arg);
        if ret < 0 {
            memset(&mut (*h).stat as *mut _ as *mut c_void, 0, size_of::<he_stat>());
            (*h).filtered |= 1u32 << ty;
            nd = __rb_hierarchy_next(&mut (*h).rb_node, HMD_FORCE_CHILD);
        } else if ret == 1 {
            (*h).filtered |= 1u32 << ty;
            nd = __rb_hierarchy_next(&mut (*h).rb_node, HMD_FORCE_SIBLING);
        } else {
            hists__remove_entry_filter(hists, h, ty);
            nd = __rb_hierarchy_next(&mut (*h).rb_node, HMD_FORCE_SIBLING);
        }
    }
    hierarchy_recalc_total_periods(hists);
}

#[no_mangle] pub unsafe extern "C" fn hists__filter_by_thread(hists: *mut hists) { if symbol_conf.report_hierarchy { hists__filter_hierarchy(hists, HIST_FILTER__THREAD, (*hists).thread_filter as *const c_void); } else { hists__filter_by_type(hists, HIST_FILTER__THREAD, Some(hists__filter_entry_by_thread)); } }
#[no_mangle] pub unsafe extern "C" fn hists__filter_by_dso(hists: *mut hists) { if symbol_conf.report_hierarchy { hists__filter_hierarchy(hists, HIST_FILTER__DSO, (*hists).dso_filter as *const c_void); } else { hists__filter_by_type(hists, HIST_FILTER__DSO, Some(hists__filter_entry_by_dso)); } }
#[no_mangle] pub unsafe extern "C" fn hists__filter_by_symbol(hists: *mut hists) { if symbol_conf.report_hierarchy { hists__filter_hierarchy(hists, HIST_FILTER__SYMBOL, (*hists).symbol_filter_str as *const c_void); } else { hists__filter_by_type(hists, HIST_FILTER__SYMBOL, Some(hists__filter_entry_by_symbol)); } }
#[no_mangle] pub unsafe extern "C" fn hists__filter_by_socket(hists: *mut hists) { if symbol_conf.report_hierarchy { hists__filter_hierarchy(hists, HIST_FILTER__SOCKET, &(*hists).socket_filter as *const _ as *const c_void); } else { hists__filter_by_type(hists, HIST_FILTER__SOCKET, Some(hists__filter_entry_by_socket)); } }
#[no_mangle] pub unsafe extern "C" fn hists__filter_by_parallelism(hists: *mut hists) { if symbol_conf.report_hierarchy { hists__filter_hierarchy(hists, HIST_FILTER__PARALLELISM, (*hists).parallelism_filter as *const c_void); } else { hists__filter_by_type(hists, HIST_FILTER__PARALLELISM, Some(hists__filter_entry_by_parallelism)); } }

#[no_mangle] pub unsafe extern "C" fn events_stats__inc(stats: *mut hists_stats, ty: u32) { *(*stats).nr_events.add(0) += 1; *(*stats).nr_events.add(ty as usize) += 1; }
unsafe fn hists_stats__inc(stats: *mut hists_stats) { (*stats).nr_samples += 1; }
#[no_mangle] pub unsafe extern "C" fn hists__inc_nr_events(hists: *mut hists) { hists_stats__inc(&mut (*hists).stats); }
#[no_mangle] pub unsafe extern "C" fn hists__inc_nr_samples(hists: *mut hists, filtered: bool) { hists_stats__inc(&mut (*hists).stats); if !filtered { (*hists).stats.nr_non_filtered_samples += 1; } }
#[no_mangle] pub unsafe extern "C" fn hists__inc_nr_lost_samples(hists: *mut hists, lost: u32) { (*hists).stats.nr_lost_samples += lost as u64; }
#[no_mangle] pub unsafe extern "C" fn hists__inc_nr_dropped_samples(hists: *mut hists, lost: u32) { (*hists).stats.nr_dropped_samples += lost as u64; }

#[no_mangle]
pub unsafe extern "C" fn hists__match(leader: *mut hists, other: *mut hists) {
    let root = if hists__has(leader, 0) { &mut (*leader).entries_collapsed } else { (*leader).entries_in };
    let mut nd = rb_first_cached(root);
    while !nd.is_null() {
        let pos: *mut hist_entry = rb_entry(nd, "rb_node_in");
        let pair = hists__find_entry(other, pos);
        if !pair.is_null() { hist_entry__add_pair(pair, pos); }
        nd = rb_next(nd);
    }
}

unsafe fn hists__find_entry(hists: *mut hists, he: *mut hist_entry) -> *mut hist_entry {
    let mut n = if hists__has(hists, 0) { (*hists).entries_collapsed.rb_root.rb_node } else { (*(*hists).entries_in).rb_root.rb_node };
    while !n.is_null() {
        let iter: *mut hist_entry = rb_entry(n, "rb_node_in");
        let cmp = hist_entry__collapse(iter, he);
        if cmp < 0 { n = (*n).rb_left; } else if cmp > 0 { n = (*n).rb_right; } else { return iter; }
    }
    null_mut()
}

unsafe fn hists__add_dummy_entry(hists: *mut hists, pair: *mut hist_entry) -> *mut hist_entry {
    let root = if hists__has(hists, 0) { &mut (*hists).entries_collapsed } else { (*hists).entries_in };
    let he = hist_entry__new(pair, true);
    if !he.is_null() {
        memset(&mut (*he).stat as *mut _ as *mut c_void, 0, size_of::<he_stat>());
        (*he).hists = hists;
        if symbol_conf.cumulate_callchain { memset((*he).stat_acc as *mut c_void, 0, size_of::<he_stat>()); }
        rb_link_node(&mut (*he).rb_node_in, null_mut(), &mut (*root).rb_root.rb_node);
        rb_insert_color_cached(&mut (*he).rb_node_in, root, true);
        hists__inc_stats(hists, he);
        (*he).dummy = true;
    }
    he
}

#[no_mangle]
pub unsafe extern "C" fn hists__link(leader: *mut hists, other: *mut hists) -> c_int {
    let root = if hists__has(other, 0) { &mut (*other).entries_collapsed } else { (*other).entries_in };
    let mut nd = rb_first_cached(root);
    while !nd.is_null() {
        let pos: *mut hist_entry = rb_entry(nd, "rb_node_in");
        if !hist_entry__has_pairs(pos) {
            let pair = hists__add_dummy_entry(leader, pos);
            if pair.is_null() { return -1; }
            hist_entry__add_pair(pos, pair);
        }
        nd = rb_next(nd);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn hists__unlink(hists: *mut hists) -> c_int {
    let root = if hists__has(hists, 0) { &mut (*hists).entries_collapsed } else { (*hists).entries_in };
    let mut nd = rb_first_cached(root);
    while !nd.is_null() {
        let pos: *mut hist_entry = rb_entry(nd, "rb_node_in");
        list_del_init(&mut (*pos).pairs.node);
        nd = rb_next(nd);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn hist__account_cycles(bs: *mut branch_stack, al: *mut addr_location, sample: *mut perf_sample, nonany_branch_mode: bool, total_cycles: *mut u64) {
    let entries = perf_sample__branch_entries(sample);
    if !bs.is_null() && (*bs).nr != 0 && (*entries).flags.cycles != 0 {
        let bi = sample__resolve_bstack(sample, al);
        if !bi.is_null() {
            let mut prev: *mut addr_map_symbol = null_mut();
            let mut i = (*bs).nr as c_int - 1;
            while i >= 0 {
                let b = bi.add(i as usize);
                addr_map_symbol__account_cycles(&mut (*b).from, if nonany_branch_mode { null_mut() } else { prev }, (*b).flags.cycles, (*sample).evsel, (*b).branch_stack_cntr);
                prev = &mut (*b).to;
                if !total_cycles.is_null() { *total_cycles += (*b).flags.cycles; }
                i -= 1;
            }
            let mut j = 0;
            while j < (*bs).nr {
                map_symbol__exit(&mut (*bi.add(j as usize)).to.ms);
                map_symbol__exit(&mut (*bi.add(j as usize)).from.ms);
                j += 1;
            }
            free(bi as *mut c_void);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn evlist__fprintf_nr_events(_evlist: *mut evlist, _fp: *mut FILE) -> size_t {
    0
}

#[no_mangle]
pub unsafe extern "C" fn hists__total_period(hists: *mut hists) -> u64 {
    if symbol_conf.filter_relative { (*hists).stats.total_non_filtered_period } else { (*hists).stats.total_period }
}

#[no_mangle]
pub unsafe extern "C" fn hists__total_latency(hists: *mut hists) -> u64 {
    if symbol_conf.filter_relative { (*hists).stats.total_non_filtered_latency } else { (*hists).stats.total_latency }
}

#[no_mangle]
pub unsafe extern "C" fn __hists__scnprintf_title(hists: *mut hists, bf: *mut c_char, size: size_t, show_freq: bool) -> c_int {
    let mut unit: c_char = 0;
    let dso = (*hists).dso_filter;
    let thread = (*hists).thread_filter;
    let socket_id = (*hists).socket_filter;
    let mut nr_samples = (*hists).stats.nr_samples as c_ulong;
    let mut nr_events = (*hists).stats.total_period;
    let evsel = hists_to_evsel(hists);
    let ev_name = evsel__name(evsel);
    let mut sample_freq_str = [0 as c_char; 64];
    if symbol_conf.filter_relative {
        nr_samples = (*hists).stats.nr_non_filtered_samples as c_ulong;
        nr_events = (*hists).stats.total_non_filtered_period;
    }
    if show_freq {
        scnprintf(sample_freq_str.as_mut_ptr(), sample_freq_str.len(), b" %d Hz,\0".as_ptr() as *const c_char, (*evsel).core.attr.sample_freq);
    }
    nr_samples = convert_unit(nr_samples, &mut unit);
    let mut printed = scnprintf(bf, size, b"Samples: %lu%c of event%s '%s',%s%sEvent count (approx.): %llu\0".as_ptr() as *const c_char, nr_samples, unit as c_int, if (*evsel).core.nr_members > 1 { b"s\0".as_ptr() } else { b"\0".as_ptr() }, ev_name, sample_freq_str.as_ptr(), b" \0".as_ptr(), nr_events);
    if !(*hists).uid_filter_str.is_null() { printed += scnprintf(bf.add(printed as usize), size - printed as usize, b", UID: %s\0".as_ptr() as *const c_char, (*hists).uid_filter_str); }
    if !thread.is_null() {
        printed += scnprintf(bf.add(printed as usize), size - printed as usize, b", Thread: %s\0".as_ptr() as *const c_char, if thread__comm_set(thread) { thread__comm_str(thread) } else { b"\0".as_ptr() as *const c_char });
    }
    if !dso.is_null() { printed += scnprintf(bf.add(printed as usize), size - printed as usize, b", DSO: %s\0".as_ptr() as *const c_char, dso__short_name(dso)); }
    if socket_id > -1 { printed += scnprintf(bf.add(printed as usize), size - printed as usize, b", Processor Socket: %d\0".as_ptr() as *const c_char, socket_id); }
    printed
}

#[no_mangle]
pub unsafe extern "C" fn parse_filter_percentage(_opt: *const option, arg: *const c_char, _unset: c_int) -> c_int {
    if strcmp(arg, b"relative\0".as_ptr() as *const c_char) == 0 {
        symbol_conf.filter_relative = true;
    } else if strcmp(arg, b"absolute\0".as_ptr() as *const c_char) == 0 {
        symbol_conf.filter_relative = false;
    } else {
        pr_debug(b"Invalid percentage: %s\n\0".as_ptr() as *const c_char, arg);
        return -1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_hist_config(var: *const c_char, value: *const c_char) -> c_int {
    if strcmp(var, b"hist.percentage\0".as_ptr() as *const c_char) == 0 {
        return parse_filter_percentage(null(), value, 0);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn __hists__init(hists: *mut hists, hpp_list: *mut perf_hpp_list) -> c_int {
    memset(hists as *mut c_void, 0, size_of::<hists>());
    (*hists).entries_in_array[0] = RB_ROOT_CACHED();
    (*hists).entries_in_array[1] = RB_ROOT_CACHED();
    (*hists).entries_in = &mut (*hists).entries_in_array[0];
    (*hists).entries_collapsed = RB_ROOT_CACHED();
    (*hists).entries = RB_ROOT_CACHED();
    mutex_init(&mut (*hists).lock);
    (*hists).socket_filter = -1;
    (*hists).parallelism_filter = symbol_conf.parallelism_filter;
    (*hists).hpp_list = hpp_list;
    INIT_LIST_HEAD(&mut (*hists).hpp_formats);
    0
}

unsafe fn hists__delete_remaining_entries(root: *mut rb_root_cached) {
    while !RB_EMPTY_ROOT(&(*root).rb_root) {
        let node = rb_first_cached(root);
        rb_erase_cached(node, root);
        let he: *mut hist_entry = rb_entry(node, "rb_node_in");
        hist_entry__delete(he);
    }
}

#[no_mangle]
pub unsafe extern "C" fn hists__delete_all_entries(hists: *mut hists) {
    hists__delete_entries(hists);
    hists__delete_remaining_entries(&mut (*hists).entries_in_array[0]);
    hists__delete_remaining_entries(&mut (*hists).entries_in_array[1]);
    hists__delete_remaining_entries(&mut (*hists).entries_collapsed);
}

unsafe extern "C" fn hists_evsel__exit(evsel: *mut evsel) {
    let hists = evsel__hists(evsel);
    hists__delete_all_entries(hists);
    zfree(&mut (*hists).symbol_filter_str as *mut _ as *mut *mut c_void);
    zfree(&mut (*hists).mem_stat_types as *mut _ as *mut *mut c_void);
    zfree(&mut (*hists).mem_stat_total as *mut _ as *mut *mut c_void);
}

unsafe extern "C" fn hists_evsel__init(evsel: *mut evsel) -> c_int {
    __hists__init(evsel__hists(evsel), &mut perf_hpp_list)
}

#[no_mangle]
pub unsafe extern "C" fn hists__init() -> c_int {
    let err = evsel__object_config(size_of::<hists>(), hists_evsel__init, hists_evsel__exit);
    if err != 0 {
        fputs(b"FATAL ERROR: Couldn't setup hists class\n\0".as_ptr() as *const c_char, stderr);
    }
    err
}

#[no_mangle]
pub unsafe extern "C" fn perf_hpp_list__init(list: *mut perf_hpp_list) {
    INIT_LIST_HEAD(&mut (*list).fields);
    INIT_LIST_HEAD(&mut (*list).sorts);
}
