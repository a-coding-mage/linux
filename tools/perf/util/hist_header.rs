/* SPDX-License-Identifier: GPL-2.0 */

// Translated from perf/util/hist.h. C include dependencies are represented as
// opaque declarations and C-compatible type aliases for the surrounding crate.

use core::ffi::{c_char, c_double, c_float, c_int, c_long, c_ulong, c_void};

pub type s32 = i32;
pub type s64 = i64;
pub type size_t = usize;
pub type int64_t = i64;
pub type filter_mask_t = u16;

pub const MEM_STAT_LEN: usize = 8;
pub const NO_ADDR: u64 = 0;
pub const HIERARCHY_INDENT: c_int = 3;

// Fallback key constants used when HAVE_SLANG_SUPPORT is not enabled in C.
pub const K_LEFT: c_int = -1000;
pub const K_RIGHT: c_int = -2000;
pub const K_SWITCH_INPUT_DATA: c_int = -3000;
pub const K_RELOAD: c_int = -4000;

#[repr(C)]
pub struct rb_root_cached {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hists_stats {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map_symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct callchain_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct simd_flags {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stats {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct comm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct annotated_data_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct addr_location {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mem_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvm_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct branch_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct branch_stack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct block_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ui_progress {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol_conf {
    pub cumulate_callchain: bool,
    pub field_sep: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum hist_filter {
    HIST_FILTER__DSO,
    HIST_FILTER__THREAD,
    HIST_FILTER__PARENT,
    HIST_FILTER__SYMBOL,
    HIST_FILTER__GUEST,
    HIST_FILTER__HOST,
    HIST_FILTER__SOCKET,
    HIST_FILTER__C2C,
    HIST_FILTER__PARALLELISM,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum hist_column {
    HISTC_SYMBOL,
    HISTC_TIME,
    HISTC_DSO,
    HISTC_THREAD,
    HISTC_TGID,
    HISTC_COMM,
    HISTC_COMM_NODIGIT,
    HISTC_CGROUP_ID,
    HISTC_CGROUP,
    HISTC_PARENT,
    HISTC_PARALLELISM,
    HISTC_CPU,
    HISTC_SOCKET,
    HISTC_SRCLINE,
    HISTC_SRCFILE,
    HISTC_MISPREDICT,
    HISTC_IN_TX,
    HISTC_ABORT,
    HISTC_SYMBOL_FROM,
    HISTC_SYMBOL_TO,
    HISTC_DSO_FROM,
    HISTC_DSO_TO,
    HISTC_LOCAL_WEIGHT,
    HISTC_GLOBAL_WEIGHT,
    HISTC_CODE_PAGE_SIZE,
    HISTC_MEM_DADDR_SYMBOL,
    HISTC_MEM_DADDR_DSO,
    HISTC_MEM_PHYS_DADDR,
    HISTC_MEM_DATA_PAGE_SIZE,
    HISTC_MEM_LOCKED,
    HISTC_MEM_TLB,
    HISTC_MEM_LVL,
    HISTC_MEM_SNOOP,
    HISTC_MEM_DCACHELINE,
    HISTC_MEM_IADDR_SYMBOL,
    HISTC_TRANSACTION,
    HISTC_CYCLES,
    HISTC_SRCLINE_FROM,
    HISTC_SRCLINE_TO,
    HISTC_TRACE,
    HISTC_SYM_SIZE,
    HISTC_DSO_SIZE,
    HISTC_SYMBOL_IPC,
    HISTC_MEM_BLOCKED,
    HISTC_LOCAL_INS_LAT,
    HISTC_GLOBAL_INS_LAT,
    HISTC_LOCAL_P_STAGE_CYC,
    HISTC_GLOBAL_P_STAGE_CYC,
    HISTC_ADDR_FROM,
    HISTC_ADDR_TO,
    HISTC_ADDR,
    HISTC_SIMD,
    HISTC_TYPE,
    HISTC_TYPE_OFFSET,
    HISTC_SYMBOL_OFFSET,
    HISTC_TYPE_CACHELINE,
    HISTC_CALLCHAIN_BRANCH_PREDICTED,
    HISTC_CALLCHAIN_BRANCH_ABORT,
    HISTC_CALLCHAIN_BRANCH_CYCLES,
    HISTC_NR_COLS,
}

pub const HISTC_NR_COLS_USIZE: usize = hist_column::HISTC_NR_COLS as usize;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mem_stat_type {
    _Unknown = 0,
}

#[repr(C)]
pub struct he_mem_stat {
    /* meaning of entries depends on enum mem_stat_type */
    pub entries: [u64; MEM_STAT_LEN],
}

#[repr(C)]
pub struct hists {
    pub entries_in_array: [rb_root_cached; 2],
    pub entries_in: *mut rb_root_cached,
    pub entries: rb_root_cached,
    pub entries_collapsed: rb_root_cached,
    pub nr_entries: u64,
    pub nr_non_filtered_entries: u64,
    pub callchain_period: u64,
    pub callchain_non_filtered_period: u64,
    pub callchain_latency: u64,
    pub callchain_non_filtered_latency: u64,
    pub thread_filter: *mut thread,
    pub dso_filter: *const dso,
    pub uid_filter_str: *const c_char,
    pub symbol_filter_str: *const c_char,
    pub parallelism_filter: *mut c_ulong,
    pub lock: mutex,
    pub stats: hists_stats,
    pub event_stream: u64,
    pub col_len: [u16; HISTC_NR_COLS_USIZE],
    pub has_callchains: bool,
    pub socket_filter: c_int,
    pub hpp_list: *mut perf_hpp_list,
    pub hpp_formats: list_head,
    pub nr_hpp_node: c_int,
    pub nr_mem_stats: c_int,
    pub mem_stat_types: *mut mem_stat_type,
    pub mem_stat_total: *mut he_mem_stat,
}

pub unsafe fn hists__has(__h: *mut hists, __f: unsafe extern "C" fn(*mut perf_hpp_list) -> bool) -> bool {
    unsafe { __f((*__h).hpp_list) }
}

#[repr(C)]
pub struct hist_entry_iter {
    pub total: c_int,
    pub curr: c_int,
    pub sample: *mut perf_sample,
    pub he: *mut hist_entry,
    pub parent: *mut symbol,
    pub mi: *mut mem_info,
    pub bi: *mut branch_info,
    pub he_cache: *mut *mut hist_entry,
    pub ops: *const hist_iter_ops,
    /* user-defined callback function (optional) */
    pub add_entry_cb: Option<
        unsafe extern "C" fn(*mut hist_entry_iter, *mut addr_location, bool, *mut c_void) -> c_int,
    >,
    pub hide_unresolved: bool,
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
pub struct res_sample {
    pub time: u64,
    pub cpu: c_int,
    pub tid: c_int,
}

#[repr(C)]
pub struct he_stat {
    pub period: u64,
    /*
     * Period re-scaled from CPU time to wall-clock time (divided by the
     * parallelism at the time of the sample). This represents effect of
     * the event on latency rather than CPU consumption.
     */
    pub latency: u64,
    pub period_sys: u64,
    pub period_us: u64,
    pub period_guest_sys: u64,
    pub period_guest_us: u64,
    pub weight1: u64,
    pub weight2: u64,
    pub weight3: u64,
    pub nr_events: u32,
}

#[repr(C)]
pub struct namespace_id {
    pub dev: u64,
    pub ino: u64,
}

#[repr(C)]
pub union hist_entry_diff_union {
    /* PERF_HPP__DELTA */
    pub period_ratio_delta: c_double,
    /* PERF_HPP__RATIO */
    pub period_ratio: c_double,
    /* HISTC_WEIGHTED_DIFF */
    pub wdiff: s64,
    /* PERF_HPP_DIFF__CYCLES */
    pub cycles: s64,
}

pub const NUM_SPARKS: usize = 0;

#[repr(C)]
pub struct hist_entry_diff {
    pub computed: bool,
    pub u: hist_entry_diff_union,
    pub stats: stats,
    pub svals: [c_ulong; NUM_SPARKS],
}

#[repr(C)]
pub struct hist_entry_ops {
    pub new: Option<unsafe extern "C" fn(size_t) -> *mut c_void>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub union hist_entry_pairs {
    pub node: list_head,
    pub head: list_head,
}

#[repr(C)]
pub struct hist_entry_tui {
    pub row_offset: u16,
    pub nr_rows: u16,
    pub init_have_children: bool,
    pub unfolded: bool,
    pub has_children: bool,
    pub has_no_entry: bool,
}

#[repr(C)]
pub union hist_entry_diff_or_tui {
    /*
     * Since perf diff only supports the stdio output, TUI
     * fields are only accessed from perf report (or perf
     * top).  So make it a union to reduce memory usage.
     */
    pub diff: hist_entry_diff,
    pub tui: hist_entry_tui,
}

#[repr(C)]
pub struct hist_entry_hierarchy_roots {
    pub hroot_in: rb_root_cached,
    pub hroot_out: rb_root_cached,
}

#[repr(C)]
pub union hist_entry_hierarchy {
    /* this is for hierarchical entry structure */
    pub roots: hist_entry_hierarchy_roots,
    /* leaf entry has callchains */
    pub sorted_chain: rb_root,
}

/**
 * struct hist_entry - histogram entry
 *
 * @row_offset - offset from the first callchain expanded to appear on screen
 * @nr_rows - rows expanded in callchain, recalculated on folding/unfolding
 */
#[repr(C)]
pub struct hist_entry {
    pub rb_node_in: rb_node,
    pub rb_node: rb_node,
    pub pairs: hist_entry_pairs,
    pub stat: he_stat,
    pub stat_acc: *mut he_stat,
    pub mem_stat: *mut he_mem_stat,
    pub ms: map_symbol,
    pub thread: *mut thread,
    pub comm: *mut comm,
    pub cgroup_id: namespace_id,
    pub cgroup: u64,
    pub ip: u64,
    pub transaction: u64,
    pub code_page_size: u64,
    pub weight: u64,
    pub ins_lat: u64,
    /** @weight3: On x86 holds retire_lat, on powerpc holds p_stage_cyc. */
    pub weight3: u64,
    pub socket: s32,
    pub cpu: s32,
    pub parallelism: c_int,
    pub mem_type_off: c_int,
    pub cpumode: u8,
    pub depth: u8,
    pub simd_flags: simd_flags,
    /* We are added by hists__add_dummy_entry. */
    pub dummy: bool,
    pub leaf: bool,
    pub level: c_char,
    pub filtered: filter_mask_t,
    pub callchain_size: u16,
    pub diff_or_tui: hist_entry_diff_or_tui,
    pub srcline: *mut c_char,
    pub srcfile: *mut c_char,
    pub parent: *mut symbol,
    pub branch_info: *mut branch_info,
    pub time: c_long,
    pub hists: *mut hists,
    pub mem_info: *mut mem_info,
    pub block_info: *mut block_info,
    pub kvm_info: *mut kvm_info,
    pub raw_data: *mut c_void,
    pub raw_size: u32,
    pub num_res: c_int,
    pub res_samples: *mut res_sample,
    pub trace_output: *mut c_void,
    pub hpp_list: *mut perf_hpp_list,
    pub parent_he: *mut hist_entry,
    pub ops: *mut hist_entry_ops,
    pub mem_type: *mut annotated_data_type,
    pub hierarchy: hist_entry_hierarchy,
    /* C flexible array member: struct callchain_root callchain[0]; must be last member. */
}

#[inline]
pub unsafe fn hist_entry__has_callchains(he: *mut hist_entry) -> bool {
    unsafe { (*he).callchain_size != 0 }
}

unsafe extern "C" {
    pub fn list_empty(head: *const list_head) -> c_int;
    pub fn list_add_tail(new: *mut list_head, head: *mut list_head);
}

#[inline]
pub unsafe fn hist_entry__has_pairs(he: *mut hist_entry) -> bool {
    unsafe { list_empty(core::ptr::addr_of!((*he).pairs.node)) == 0 }
}

#[inline]
pub unsafe fn hist_entry__next_pair(he: *mut hist_entry) -> *mut hist_entry {
    if unsafe { hist_entry__has_pairs(he) } {
        // C used list_entry(he->pairs.node.next, struct hist_entry, pairs.node).
        unsafe { (*he).pairs.node.next as *mut hist_entry }
    } else {
        core::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn hist_entry__add_pair(pair: *mut hist_entry, he: *mut hist_entry) {
    unsafe {
        list_add_tail(
            core::ptr::addr_of_mut!((*pair).pairs.node),
            core::ptr::addr_of_mut!((*he).pairs.head),
        );
    }
}

pub type hists__resort_cb_t = Option<unsafe extern "C" fn(*mut hist_entry, *mut c_void) -> c_int>;

#[repr(C)]
pub struct hists_evsel {
    pub evsel: evsel,
    pub hists: hists,
}

#[inline]
pub unsafe fn hists_to_evsel(hists: *mut hists) -> *mut evsel {
    hists.cast::<hists_evsel>().cast::<evsel>()
}

#[inline]
pub unsafe fn evsel__hists(evsel: *mut evsel) -> *mut hists {
    unsafe { core::ptr::addr_of_mut!((*(evsel as *mut hists_evsel)).hists) }
}

#[inline]
pub unsafe fn hists__has_callchains(hists: *mut hists) -> bool {
    unsafe { (*hists).has_callchains }
}

#[repr(C)]
pub struct perf_hpp {
    pub buf: *mut c_char,
    pub size: size_t,
    pub sep: *const c_char,
    pub ptr: *mut c_void,
    pub skip: bool,
}

pub type perf_hpp_fmt_cmp_t =
    Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut hist_entry, *mut hist_entry) -> int64_t>;

#[repr(C)]
pub struct perf_hpp_fmt {
    pub name: *const c_char,
    pub header: Option<
        unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hists, c_int, *mut c_int) -> c_int,
    >,
    pub width: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hists) -> c_int>,
    pub init: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut hist_entry)>,
    pub color: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hist_entry) -> c_int>,
    pub entry: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hist_entry) -> c_int>,
    pub cmp: perf_hpp_fmt_cmp_t,
    pub collapse: perf_hpp_fmt_cmp_t,
    pub sort: perf_hpp_fmt_cmp_t,
    pub equal: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp_fmt) -> bool>,
    pub free: Option<unsafe extern "C" fn(*mut perf_hpp_fmt)>,
    pub list: list_head,
    pub sort_list: list_head,
    pub elide: bool,
    pub len: c_int,
    pub user_len: c_int,
    pub idx: c_int,
    pub level: c_int,
}

#[repr(C)]
pub struct perf_hpp_list {
    pub fields: list_head,
    pub sorts: list_head,
    pub nr_header_lines: c_int,
    pub need_collapse: c_int,
    pub parent: c_int,
    pub sym: c_int,
    pub dso: c_int,
    pub socket: c_int,
    pub thread: c_int,
    pub comm: c_int,
    pub comm_nodigit: c_int,
}

#[repr(C)]
pub struct perf_hpp_list_node {
    pub list: list_head,
    pub hpp: perf_hpp_list,
    pub level: c_int,
    pub skip: bool,
}

// C iterator macros map to list traversal helpers in the surrounding code:
// perf_hpp_list__for_each_format, perf_hpp_list__for_each_format_safe,
// perf_hpp_list__for_each_sort_list, perf_hpp_list__for_each_sort_list_safe,
// hists__for_each_format, and hists__for_each_sort_list.

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum perf_hpp_format_index {
    /* Matches perf_hpp__format array. */
    PERF_HPP__OVERHEAD,
    PERF_HPP__LATENCY,
    PERF_HPP__OVERHEAD_SYS,
    PERF_HPP__OVERHEAD_US,
    PERF_HPP__OVERHEAD_GUEST_SYS,
    PERF_HPP__OVERHEAD_GUEST_US,
    PERF_HPP__OVERHEAD_ACC,
    PERF_HPP__LATENCY_ACC,
    PERF_HPP__SAMPLES,
    PERF_HPP__PERIOD,
    PERF_HPP__WEIGHT1,
    PERF_HPP__WEIGHT2,
    PERF_HPP__WEIGHT3,
    PERF_HPP__MEM_STAT_OP,
    PERF_HPP__MEM_STAT_CACHE,
    PERF_HPP__MEM_STAT_MEMORY,
    PERF_HPP__MEM_STAT_SNOOP,
    PERF_HPP__MEM_STAT_DTLB,
    PERF_HPP__MAX_INDEX,
}

pub const PERF_HPP__MAX_INDEX_USIZE: usize = perf_hpp_format_index::PERF_HPP__MAX_INDEX as usize;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum perf_hpp_fmt_type {
    PERF_HPP_FMT_TYPE__RAW,
    PERF_HPP_FMT_TYPE__PERCENT,
    PERF_HPP_FMT_TYPE__LATENCY,
    PERF_HPP_FMT_TYPE__AVERAGE,
}

pub type hpp_field_fn = Option<unsafe extern "C" fn(*mut hist_entry) -> u64>;
pub type hpp_callback_fn = Option<unsafe extern "C" fn(*mut perf_hpp, bool) -> c_int>;
pub type hpp_snprint_fn = Option<unsafe extern "C" fn(*mut perf_hpp, *const c_char, ...) -> c_int>;

#[inline]
pub unsafe fn advance_hpp(hpp: *mut perf_hpp, inc: c_int) {
    unsafe {
        (*hpp).buf = (*hpp).buf.add(inc as usize);
        (*hpp).size = (*hpp).size.wrapping_sub(inc as size_t);
    }
}

pub const COLOR_MAXLEN: size_t = 0;
pub const PERF_COLOR_RESET_SIZE: size_t = 0;

#[inline]
pub unsafe fn perf_hpp__use_color() -> size_t {
    unsafe { ((*symbol_conf).field_sep.is_null()) as size_t }
}

#[inline]
pub unsafe fn perf_hpp__color_overhead() -> size_t {
    if unsafe { perf_hpp__use_color() } != 0 {
        (COLOR_MAXLEN + PERF_COLOR_RESET_SIZE) * PERF_HPP__MAX_INDEX_USIZE
    } else {
        0
    }
}

#[repr(C)]
pub struct hist_browser_timer {
    pub timer: Option<unsafe extern "C" fn(*mut c_void)>,
    pub arg: *mut c_void,
    pub refresh: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rstype {
    A_NORMAL,
    A_ASM,
    A_SOURCE,
}

#[repr(C)]
pub struct block_hist {
    pub block_hists: hists,
    pub block_list: perf_hpp_list,
    pub block_fmt: perf_hpp_fmt,
    pub block_idx: c_int,
    pub valid: bool,
    pub he: hist_entry,
}

// HAVE_SLANG_SUPPORT declarations are external when enabled by the C build.
// The inline fallback definitions below match the #else branch in hist.h.
#[inline]
pub unsafe fn evlist__tui_browse_hists(
    _evlist: *mut evlist,
    _help: *const c_char,
    _hbt: *mut hist_browser_timer,
    _min_pcnt: c_float,
    _env: *mut perf_env,
    _warn_lost_event: bool,
) -> c_int {
    0
}

#[inline]
pub unsafe fn __hist_entry__tui_annotate(
    _he: *mut hist_entry,
    _ms: *mut map_symbol,
    _evsel: *mut evsel,
    _hbt: *mut hist_browser_timer,
    _al_addr: u64,
) -> c_int {
    0
}

#[inline]
pub unsafe fn hist_entry__tui_annotate(
    _he: *mut hist_entry,
    _evsel: *mut evsel,
    _hbt: *mut hist_browser_timer,
    _al_addr: u64,
) -> c_int {
    0
}

#[inline]
pub unsafe fn script_browse(_script_opt: *const c_char, _evsel: *mut evsel) -> c_int {
    0
}

#[inline]
pub unsafe fn res_sample_browse(
    _res_samples: *mut res_sample,
    _num_res: c_int,
    _evsel: *mut evsel,
    _rstype: rstype,
) -> c_int {
    0
}

#[inline]
pub unsafe fn res_sample_init() {}

#[inline]
pub unsafe fn block_hists_tui_browse(
    _bh: *mut block_hist,
    _evsel: *mut evsel,
    _min_percent: c_float,
    _env: *mut perf_env,
) -> c_int {
    0
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum hierarchy_move_dir {
    HMD_NORMAL,
    HMD_FORCE_SIBLING,
    HMD_FORCE_CHILD,
}

#[inline]
pub unsafe fn rb_hierarchy_next(node: *mut rb_node) -> *mut rb_node {
    unsafe { __rb_hierarchy_next(node, hierarchy_move_dir::HMD_NORMAL) }
}

#[inline]
pub unsafe fn hists__has_filter(hists: *mut hists) -> bool {
    unsafe {
        !(*hists).thread_filter.is_null()
            || !(*hists).dso_filter.is_null()
            || !(*hists).symbol_filter_str.is_null()
            || ((*hists).socket_filter > -1)
            || !(*hists).parallelism_filter.is_null()
    }
}

#[inline]
pub unsafe fn hist_entry__get_percent_limit(he: *mut hist_entry) -> c_float {
    unsafe {
        let mut period = (*he).stat.period;
        let total_period = hists__total_period((*he).hists);

        if total_period == 0 {
            return 0.0;
        }

        if symbol_conf.cumulate_callchain {
            period = (*(*he).stat_acc).period;
        }

        (period as c_double * 100.0 / total_period as c_double) as c_float
    }
}

#[inline]
pub unsafe fn perf_hpp__column_register(format: *mut perf_hpp_fmt) {
    unsafe { perf_hpp_list__column_register(core::ptr::addr_of_mut!(perf_hpp_list), format) }
}

#[inline]
pub unsafe fn perf_hpp__register_sort_field(format: *mut perf_hpp_fmt) {
    unsafe { perf_hpp_list__register_sort_field(core::ptr::addr_of_mut!(perf_hpp_list), format) }
}

#[inline]
pub unsafe fn perf_hpp__prepend_sort_field(format: *mut perf_hpp_fmt) {
    unsafe { perf_hpp_list__prepend_sort_field(core::ptr::addr_of_mut!(perf_hpp_list), format) }
}

#[inline]
pub unsafe fn perf_hpp__should_skip(format: *mut perf_hpp_fmt, hists: *mut hists) -> bool {
    unsafe {
        if (*format).elide {
            return true;
        }

        if perf_hpp__is_dynamic_entry(format) && !perf_hpp__defined_dynamic_entry(format, hists) {
            return true;
        }

        false
    }
}

#[inline]
pub unsafe fn hists__scnprintf_title(hists: *mut hists, bf: *mut c_char, size: size_t) -> c_int {
    unsafe { __hists__scnprintf_title(hists, bf, size, true) }
}

unsafe extern "C" {
    pub static hist_iter_normal: hist_iter_ops;
    pub static hist_iter_branch: hist_iter_ops;
    pub static hist_iter_mem: hist_iter_ops;
    pub static hist_iter_cumulative: hist_iter_ops;

    pub static mut perf_hpp_list: perf_hpp_list;
    pub static mut perf_hpp__format: [perf_hpp_fmt; 0];
    pub static symbol_conf: symbol_conf;

    pub fn hists__add_entry(
        hists: *mut hists,
        al: *mut addr_location,
        parent: *mut symbol,
        bi: *mut branch_info,
        mi: *mut mem_info,
        ki: *mut kvm_info,
        sample: *mut perf_sample,
        sample_self: bool,
    ) -> *mut hist_entry;
    pub fn hists__add_entry_ops(
        hists: *mut hists,
        ops: *mut hist_entry_ops,
        al: *mut addr_location,
        sym_parent: *mut symbol,
        bi: *mut branch_info,
        mi: *mut mem_info,
        ki: *mut kvm_info,
        sample: *mut perf_sample,
        sample_self: bool,
    ) -> *mut hist_entry;
    pub fn hists__add_entry_block(
        hists: *mut hists,
        al: *mut addr_location,
        bi: *mut block_info,
    ) -> *mut hist_entry;
    pub fn hist_entry_iter__add(
        iter: *mut hist_entry_iter,
        al: *mut addr_location,
        max_stack_depth: c_int,
        arg: *mut c_void,
    ) -> c_int;

    pub fn hist_entry__transaction_len() -> c_int;
    pub fn hist_entry__sort_snprintf(
        he: *mut hist_entry,
        bf: *mut c_char,
        size: size_t,
        hists: *mut hists,
    ) -> c_int;
    pub fn hist_entry__snprintf_alignment(
        he: *mut hist_entry,
        hpp: *mut perf_hpp,
        fmt: *mut perf_hpp_fmt,
        printed: c_int,
    ) -> c_int;
    pub fn hist_entry__sym_snprintf(
        he: *mut hist_entry,
        bf: *mut c_char,
        size: size_t,
        width: c_uint,
    ) -> c_int;
    pub fn hist_entry__delete(he: *mut hist_entry);

    pub fn evsel__output_resort_cb(
        evsel: *mut evsel,
        prog: *mut ui_progress,
        cb: hists__resort_cb_t,
        cb_arg: *mut c_void,
    );
    pub fn evsel__output_resort(evsel: *mut evsel, prog: *mut ui_progress);
    pub fn hists__output_resort(hists: *mut hists, prog: *mut ui_progress);
    pub fn hists__output_resort_cb(hists: *mut hists, prog: *mut ui_progress, cb: hists__resort_cb_t);
    pub fn hists__collapse_resort(hists: *mut hists, prog: *mut ui_progress) -> c_int;
    pub fn hists__decay_entries(hists: *mut hists, zap_user: bool, zap_kernel: bool);
    pub fn hists__delete_entries(hists: *mut hists);
    pub fn hists__delete_all_entries(hists: *mut hists);
    pub fn hists__output_recalc_col_len(hists: *mut hists, max_rows: c_int);
    pub fn hists__get_entry(hists: *mut hists, idx: c_int) -> *mut hist_entry;
    pub fn hists__total_period(hists: *mut hists) -> u64;
    pub fn hists__total_latency(hists: *mut hists) -> u64;
    pub fn hists__reset_stats(hists: *mut hists);
    pub fn hists__inc_stats(hists: *mut hists, h: *mut hist_entry);
    pub fn hists__inc_nr_events(hists: *mut hists);
    pub fn hists__inc_nr_samples(hists: *mut hists, filtered: bool);
    pub fn hists__inc_nr_lost_samples(hists: *mut hists, lost: u32);
    pub fn hists__inc_nr_dropped_samples(hists: *mut hists, lost: u32);
    pub fn hists__fprintf(
        hists: *mut hists,
        show_header: bool,
        max_rows: c_int,
        max_cols: c_int,
        min_pcnt: c_float,
        fp: *mut FILE,
        ignore_callchains: bool,
    ) -> size_t;
    pub fn evlist__fprintf_nr_events(evlist: *mut evlist, fp: *mut FILE) -> size_t;
    pub fn hists__filter_by_dso(hists: *mut hists);
    pub fn hists__filter_by_thread(hists: *mut hists);
    pub fn hists__filter_by_symbol(hists: *mut hists);
    pub fn hists__filter_by_socket(hists: *mut hists);
    pub fn hists__filter_by_parallelism(hists: *mut hists);
    pub fn hists__col_len(hists: *mut hists, col: hist_column) -> u16;
    pub fn hists__set_col_len(hists: *mut hists, col: hist_column, len: u16);
    pub fn hists__new_col_len(hists: *mut hists, col: hist_column, len: u16) -> bool;
    pub fn hists__reset_col_len(hists: *mut hists);
    pub fn hists__calc_col_len(hists: *mut hists, he: *mut hist_entry);
    pub fn hists__match(leader: *mut hists, other: *mut hists);
    pub fn hists__link(leader: *mut hists, other: *mut hists) -> c_int;
    pub fn hists__unlink(hists: *mut hists) -> c_int;
    pub fn hists__init() -> c_int;
    pub fn __hists__init(hists: *mut hists, hpp_list: *mut perf_hpp_list) -> c_int;
    pub fn hists__get_rotate_entries_in(hists: *mut hists) -> *mut rb_root_cached;

    pub fn perf_hpp_list__column_register(list: *mut perf_hpp_list, format: *mut perf_hpp_fmt);
    pub fn perf_hpp_list__register_sort_field(list: *mut perf_hpp_list, format: *mut perf_hpp_fmt);
    pub fn perf_hpp_list__prepend_sort_field(list: *mut perf_hpp_list, format: *mut perf_hpp_fmt);
    pub fn perf_hpp__init();
    pub fn perf_hpp__cancel_cumulate(evlist: *mut evlist);
    pub fn perf_hpp__cancel_latency(evlist: *mut evlist);
    pub fn perf_hpp__setup_output_field(list: *mut perf_hpp_list);
    pub fn perf_hpp__reset_output_field(list: *mut perf_hpp_list);
    pub fn perf_hpp__append_sort_keys(list: *mut perf_hpp_list);
    pub fn perf_hpp__setup_hists_formats(list: *mut perf_hpp_list, evlist: *mut evlist) -> c_int;
    pub fn perf_hpp__alloc_mem_stats(list: *mut perf_hpp_list, evlist: *mut evlist) -> c_int;
    pub fn perf_hpp__is_sort_entry(format: *mut perf_hpp_fmt) -> bool;
    pub fn perf_hpp__is_dynamic_entry(format: *mut perf_hpp_fmt) -> bool;
    pub fn perf_hpp__defined_dynamic_entry(fmt: *mut perf_hpp_fmt, hists: *mut hists) -> bool;
    pub fn perf_hpp__is_trace_entry(fmt: *mut perf_hpp_fmt) -> bool;
    pub fn perf_hpp__is_srcline_entry(fmt: *mut perf_hpp_fmt) -> bool;
    pub fn perf_hpp__is_srcfile_entry(fmt: *mut perf_hpp_fmt) -> bool;
    pub fn perf_hpp__is_thread_entry(fmt: *mut perf_hpp_fmt) -> bool;
    pub fn perf_hpp__is_comm_entry(fmt: *mut perf_hpp_fmt) -> bool;
    pub fn perf_hpp__is_dso_entry(fmt: *mut perf_hpp_fmt) -> bool;
    pub fn perf_hpp__is_sym_entry(fmt: *mut perf_hpp_fmt) -> bool;
    pub fn perf_hpp__is_parallelism_entry(fmt: *mut perf_hpp_fmt) -> bool;
    pub fn perf_hpp_fmt__dup(fmt: *mut perf_hpp_fmt) -> *mut perf_hpp_fmt;
    pub fn hist_entry__filter(he: *mut hist_entry, type_: c_int, arg: *const c_void) -> c_int;
    pub fn perf_hpp__reset_width(fmt: *mut perf_hpp_fmt, hists: *mut hists);
    pub fn perf_hpp__reset_sort_width(fmt: *mut perf_hpp_fmt, hists: *mut hists);
    pub fn perf_hpp__set_user_width(width_list_str: *const c_char);
    pub fn hists__reset_column_width(hists: *mut hists);
    pub fn hpp__fmt(
        fmt: *mut perf_hpp_fmt,
        hpp: *mut perf_hpp,
        he: *mut hist_entry,
        get_field: hpp_field_fn,
        fmtstr: *const c_char,
        print_fn: hpp_snprint_fn,
        fmtype: perf_hpp_fmt_type,
    ) -> c_int;
    pub fn hpp__fmt_acc(
        fmt: *mut perf_hpp_fmt,
        hpp: *mut perf_hpp,
        he: *mut hist_entry,
        get_field: hpp_field_fn,
        fmtstr: *const c_char,
        print_fn: hpp_snprint_fn,
        fmtype: perf_hpp_fmt_type,
    ) -> c_int;
    pub fn hpp__fmt_mem_stat(
        fmt: *mut perf_hpp_fmt,
        hpp: *mut perf_hpp,
        he: *mut hist_entry,
        mst: mem_stat_type,
        fmtstr: *const c_char,
        print_fn: hpp_snprint_fn,
    ) -> c_int;

    pub fn attr_to_script(buf: *mut c_char, attr: *mut perf_event_attr);
    pub fn run_script(cmd: *mut c_char);

    pub fn hists__sort_list_width(hists: *mut hists) -> c_uint;
    pub fn hists__overhead_width(hists: *mut hists) -> c_uint;
    pub fn hist__account_cycles(
        bs: *mut branch_stack,
        al: *mut addr_location,
        sample: *mut perf_sample,
        nonany_branch_mode: bool,
        total_cycles: *mut u64,
    );
    pub fn parse_filter_percentage(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    pub fn perf_hist_config(var: *const c_char, value: *const c_char) -> c_int;
    pub fn perf_hpp_list__init(list: *mut perf_hpp_list);
    pub fn rb_hierarchy_last(node: *mut rb_node) -> *mut rb_node;
    pub fn __rb_hierarchy_next(node: *mut rb_node, hmd: hierarchy_move_dir) -> *mut rb_node;
    pub fn rb_hierarchy_prev(node: *mut rb_node) -> *mut rb_node;
    pub fn hist_entry__has_hierarchy_children(he: *mut hist_entry, limit: c_float) -> bool;
    pub fn hpp_color_scnprintf(hpp: *mut perf_hpp, fmt: *const c_char, ...) -> c_int;
    pub fn __hpp__slsmg_color_printf(hpp: *mut perf_hpp, fmt: *const c_char, ...) -> c_int;
    pub fn __hist_entry__snprintf(
        he: *mut hist_entry,
        hpp: *mut perf_hpp,
        hpp_list: *mut perf_hpp_list,
    ) -> c_int;
    pub fn hists__fprintf_headers(hists: *mut hists, fp: *mut FILE) -> c_int;
    pub fn __hists__scnprintf_title(
        hists: *mut hists,
        bf: *mut c_char,
        size: size_t,
        show_freq: bool,
    ) -> c_int;
}

pub type c_uint = u32;
