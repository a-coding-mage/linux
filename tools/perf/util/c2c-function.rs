// SPDX-License-Identifier: GPL-2.0
/*
 * C2C function model - function-level cacheline sharing analysis
 *
 * Displays a 3-level hierarchy showing which functions share cachelines:
 *   Level 1: Read-side functions sorted by Cycles % (estimated load cycles)
 *   Level 2: Functions sampled writing the shared lines read by level 1
 *   Level 3: The specific cachelines where the two functions contend
 *
 * Builds the hierarchy from the existing cacheline histograms
 * (c2c_hist_entry->hists), reusing the shared c2c data structures.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_longlong, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type u64 = u64;
type int64_t = i64;
type size_t = usize;

const ENOENT: c_int = 2;
const ESRCH: c_int = 3;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EOPNOTSUPP: c_int = 95;
const SYMBOL_WIDTH: c_int = 50;
const HISTC_SYMBOL: c_int = 0;

/* Spaces of indent per hierarchy level, like the normal report view. */
const C2C_FUNC_INDENT: c_int = 2;

/* Width of the folded-sign prefix ("%c ") each identity cell emits. */
const C2C_FUNC_FOLD_WIDTH: c_int = 2;

/* Initial per-cacheline capacity for the seen[] set; grown on demand. */
const DEFAULT_SYMBOLS_PER_CL: c_int = 64;

#[repr(C)]
pub struct rb_node {
    rb_left: *mut rb_node,
    rb_right: *mut rb_node,
}

#[repr(C)]
pub struct rb_root {
    rb_node: *mut rb_node,
}

#[repr(C)]
pub struct rb_root_cached {
    rb_root: rb_root,
    rb_leftmost: *mut rb_node,
}

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct stats {
    n: f64,
    mean: f64,
    M2: f64,
    min: f64,
    max: f64,
}

#[repr(C)]
pub struct compute_stats {
    rmt_hitm: stats,
    lcl_hitm: stats,
    rmt_peer: stats,
    lcl_peer: stats,
    load: stats,
}

#[repr(C)]
pub struct c2c_stats {
    tot_hitm: u64,
    rmt_peer: u64,
    lcl_peer: u64,
    tot_peer: u64,
    store: u64,
    load: u64,
}

#[repr(C)]
pub struct symbol {
    name: *const c_char,
    start: u64,
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map_symbol {
    thread: *mut thread,
    map: *mut map,
    sym: *mut symbol,
}

#[repr(C)]
pub struct addr_map_symbol {
    addr: u64,
    ms: map_symbol,
}

#[repr(C)]
pub struct mem_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct he_stat {
    nr_events: u64,
    period: u64,
    weight1: u64,
}

#[repr(C)]
pub struct hist_entry_ops {
    new: Option<unsafe extern "C" fn(size_t) -> *mut c_void>,
    free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub struct hist_entry_pairs {
    node: list_head,
}

#[repr(C)]
pub struct hist_entry {
    rb_node: rb_node,
    rb_node_in: rb_node,
    hroot_in: rb_root_cached,
    hroot_out: rb_root_cached,
    parent_he: *mut hist_entry,
    hists: *mut hists,
    hpp_list: *mut perf_hpp_list,
    mem_info: *mut mem_info,
    thread: *mut thread,
    ms: map_symbol,
    stat: he_stat,
    stat_acc: *mut he_stat,
    pairs: hist_entry_pairs,
    ops: *mut hist_entry_ops,
    ip: u64,
    depth: c_int,
    cpumode: c_int,
    cpu: c_int,
    socket: c_int,
    level: c_int,
    filtered: bool,
    unfolded: bool,
    has_children: bool,
    has_no_entry: bool,
    leaf: bool,
    nr_rows: c_int,
    row_offset: c_int,
}

#[repr(C)]
pub struct c2c_hist_entry {
    he: hist_entry,
    stats: c2c_stats,
    cstats: compute_stats,
    hists: *mut c2c_hists,
    cacheline_idx: u64,
}

#[repr(C)]
pub struct hists {
    entries: rb_root_cached,
    entries_in: *mut rb_root_cached,
    nr_entries: c_int,
    nr_non_filtered_entries: c_int,
}

#[repr(C)]
pub struct header_line {
    text: *const c_char,
    span: c_int,
}

#[repr(C)]
pub struct c2c_header {
    line: [header_line; 2],
}

#[repr(C)]
pub struct sort_entry {
    se_width_idx: c_int,
    se_header: *const c_char,
    se_cmp: Option<unsafe extern "C" fn(*mut hist_entry, *mut hist_entry) -> int64_t>,
    se_collapse: Option<unsafe extern "C" fn(*mut hist_entry, *mut hist_entry) -> int64_t>,
    se_sort: Option<unsafe extern "C" fn(*mut hist_entry, *mut hist_entry) -> int64_t>,
    se_snprintf: Option<unsafe extern "C" fn(*mut hist_entry, *mut c_char, size_t, c_int) -> c_int>,
}

#[repr(C)]
pub struct perf_hpp {
    buf: *mut c_char,
    size: size_t,
}

#[repr(C)]
pub struct perf_hpp_fmt {
    list: list_head,
    sort_list: list_head,
    cmp: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut hist_entry, *mut hist_entry) -> int64_t>,
    sort: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut hist_entry, *mut hist_entry) -> int64_t>,
    color: Option<unsafe extern "C" fn()>,
    entry: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hist_entry) -> c_int>,
    header: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hists, c_int, *mut c_int) -> c_int>,
    width: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hists) -> c_int>,
    collapse: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut hist_entry, *mut hist_entry) -> int64_t>,
    equal: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp_fmt) -> bool>,
    free: Option<unsafe extern "C" fn(*mut perf_hpp_fmt)>,
}

#[repr(C)]
pub struct c2c_dimension {
    header: c2c_header,
    name: *const c_char,
    cmp: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut hist_entry, *mut hist_entry) -> int64_t>,
    entry: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hist_entry) -> c_int>,
    color: Option<unsafe extern "C" fn()>,
    se: *mut sort_entry,
    width: c_int,
}

#[repr(C)]
pub struct c2c_fmt {
    fmt: perf_hpp_fmt,
    dim: *mut c2c_dimension,
}

#[repr(C)]
pub struct perf_hpp_list {
    fields: list_head,
    nr_header_lines: c_int,
    need_collapse: c_int,
    parent: c_int,
    sym: c_int,
    dso: c_int,
    socket: c_int,
    thread: c_int,
    comm: c_int,
    comm_nodigit: c_int,
}

#[repr(C)]
pub struct c2c_hists {
    hists: hists,
    list: perf_hpp_list,
    stats: c2c_stats,
}

#[repr(C)]
pub struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
pub struct addr_location {
    thread: *mut thread,
    map: *mut map,
    sym: *mut symbol,
    addr: u64,
    level: c_int,
    cpumode: c_int,
    cpu: c_int,
    socket: c_int,
    filtered: c_int,
    latency: u64,
}

#[repr(C)]
pub struct perf_sample {
    period: u64,
    weight: u64,
    ip: u64,
    pid: c_int,
    tid: c_int,
    cpu: c_int,
}

#[repr(C)]
pub struct symbol_conf_t {
    cumulate_callchain: bool,
    use_callchain: bool,
}

#[repr(C)]
pub struct c2c_function_model {
    function_hists: c2c_hists,
    /* Total estimated cycles across all level-1 entries. */
    total_cycles: u64,
    /* Source cacheline histograms; not owned here. */
    cl_hists: *mut c2c_hists,
    /* --coalesce field list, used to require iaddr. */
    cl_sort: *const c_char,
    /* Do not cap long symbol names. */
    symbol_full: bool,
}

unsafe extern "C" {
    static mut verbose: c_int;
    static mut chk_double_cl: bool;
    static mut symbol_conf: symbol_conf_t;
    static mut sort_sym: sort_entry;

    fn map__dso(map: *mut map) -> *const dso;
    fn dso__long_name(dso: *const dso) -> *const c_char;
    fn dso__short_name(dso: *const dso) -> *const c_char;
    fn _sort__sym_cmp(left: *mut symbol, right: *mut symbol) -> int64_t;
    fn mem_info__iaddr(mi: *mut mem_info) -> *mut addr_map_symbol;
    fn mem_info__daddr(mi: *mut mem_info) -> *mut addr_map_symbol;
    fn hists__col_len(hists: *mut hists, idx: c_int) -> c_int;
    fn hists__set_col_len(hists: *mut hists, idx: c_int, len: c_int);
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn avg_stats(stats: *const stats) -> f64;
    fn rb_first_cached(root: *mut rb_root_cached) -> *mut rb_node;
    fn rb_next(node: *mut rb_node) -> *mut rb_node;
    fn rb_erase_cached(node: *mut rb_node, root: *mut rb_root_cached);
    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node);
    fn rb_insert_color_cached(node: *mut rb_node, root: *mut rb_root_cached, leftmost: bool);
    fn zalloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strtok_r(s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn perf_hpp_list__column_register(list: *mut perf_hpp_list, fmt: *mut perf_hpp_fmt);
    fn perf_hpp_list__register_sort_field(list: *mut perf_hpp_list, fmt: *mut perf_hpp_fmt);
    fn perf_hpp__reset_output_field(list: *mut perf_hpp_list);
    fn perf_hpp_list__init(list: *mut perf_hpp_list);
    fn __hists__init(hists: *mut hists, list: *mut perf_hpp_list);
    fn hists__delete_all_entries(hists: *mut hists);
    fn hist_entry__delete(he: *mut hist_entry);
    fn c2c_fmt_equal(a: *mut perf_hpp_fmt, b: *mut perf_hpp_fmt) -> bool;
    fn c2c_fmt_free(fmt: *mut perf_hpp_fmt);
    fn pr_err(fmt: *const c_char, ...);
    fn init_stats(stats: *mut stats);
    fn c2c_add_stats(dest: *mut c2c_stats, src: *const c2c_stats);
    fn map_symbol__copy(dst: *mut map_symbol, src: *const map_symbol);
    fn mem_info__clone(mi: *mut mem_info) -> *mut mem_info;
    fn thread__get(thread: *mut thread) -> *mut thread;
    fn map__get(map: *mut map) -> *mut map;
    fn map__unmap_ip(map: *mut map, ip: u64) -> u64;
    fn mem_info__new() -> *mut mem_info;
    fn mem_info__put(mi: *mut mem_info);
    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn thread__pid(thread: *mut thread) -> c_int;
    fn thread__tid(thread: *mut thread) -> c_int;
    fn hists__add_entry_ops(hists: *mut hists, ops: *mut hist_entry_ops, al: *mut addr_location, parent: *mut c_void, branch_info: *mut c_void, mi: *mut mem_info, kvm_info: *mut c_void, sample: *mut perf_sample, sample_self: bool) -> *mut hist_entry;
    fn cl_address(addr: u64, chk_double_cl: bool) -> u64;
    fn hists__collapse_resort(hists: *mut hists, prog: *mut c_void);
    fn hists__output_resort(hists: *mut hists, prog: *mut c_void);
}

static mut c2c_ext: c2c_function_model = unsafe { zeroed() };

static mut dim_symbol_view: c2c_dimension = unsafe { zeroed() };

unsafe fn container_of_c2c_hist_entry_from_he(he: *mut hist_entry) -> *mut c2c_hist_entry {
    he as *mut c2c_hist_entry
}

unsafe fn container_of_c2c_fmt_from_fmt(fmt: *mut perf_hpp_fmt) -> *mut c2c_fmt {
    fmt as *mut c2c_fmt
}

unsafe fn rb_entry_hist_entry(node: *mut rb_node) -> *mut hist_entry {
    node as *mut hist_entry
}

unsafe fn rb_entry_c2c_hist_entry(node: *mut rb_node) -> *mut c2c_hist_entry {
    node as *mut c2c_hist_entry
}

fn rb_root_cached_empty() -> rb_root_cached {
    rb_root_cached {
        rb_root: rb_root { rb_node: null_mut() },
        rb_leftmost: null_mut(),
    }
}

unsafe fn RB_EMPTY_ROOT(root: *const rb_root) -> bool {
    (*root).rb_node.is_null()
}

unsafe extern "C" fn c2c_hitm_count(stats: *const c2c_stats) -> u64 {
    (*stats).tot_hitm
}

unsafe extern "C" fn c2c_function_cmp(left: *const map_symbol, right: *const map_symbol) -> int64_t {
    let left_dso = if !(*left).map.is_null() { map__dso((*left).map) } else { null() };
    let right_dso = if !(*right).map.is_null() { map__dso((*right).map) } else { null() };
    let mut ret: c_int;

    if left_dso.is_null() || right_dso.is_null() {
        if left_dso != right_dso {
            return if !left_dso.is_null() { 1 } else { -1 };
        }
    } else {
        /*
         * Use the same DSO name as _sort__dso_cmp() (short name unless
         * verbose), so this matches the DSO comparison the level-1
         * entries are deduplicated by; otherwise same-basename DSOs
         * could be split or merged inconsistently across levels.
         */
        let left_name = if verbose > 0 { dso__long_name(left_dso) } else { dso__short_name(left_dso) };
        let right_name = if verbose > 0 { dso__long_name(right_dso) } else { dso__short_name(right_dso) };

        ret = strcmp(left_name, right_name);
        if ret != 0 {
            return ret as int64_t;
        }
    }

    _sort__sym_cmp((*left).sym, (*right).sym)
}

unsafe extern "C" fn hist_entry__iaddr(he: *mut hist_entry) -> u64 {
    if !(*he).mem_info.is_null() {
        return (*mem_info__iaddr((*he).mem_info)).addr;
    }
    (*he).ip
}

/*
 * Hierarchy levels (by depth): L1 = read-side function, L2 = the writing
 * function it contends with, L3 = the specific shared cacheline.
 */
unsafe extern "C" fn hist_entry__is_cacheline(he: *mut hist_entry) -> bool {
    !(*he).parent_he.is_null() && !(*(*he).parent_he).parent_he.is_null()
}

/*
 * Write he->depth levels of leading indentation into @buf, so lower-level
 * entries are visually nested under their parent. Returns bytes written.
 */
unsafe extern "C" fn hist_entry__indent(he: *mut hist_entry, buf: *mut c_char, size: size_t) -> c_int {
    let indent = (*he).depth * C2C_FUNC_INDENT;

    if indent <= 0 || indent as size_t >= size {
        return 0;
    }

    scnprintf(buf, size, c"%*s".as_ptr(), indent, c"".as_ptr())
}

unsafe extern "C" fn symbol_width(hists: *mut hists, se: *mut sort_entry) -> c_int {
    let mut width = hists__col_len(hists, (*se).se_width_idx);

    /*
     * Cap long symbol names as the cacheline view does. The stored column
     * length is grown up front to fit the deepest, longest identity cell
     * (including a level-3 cacheline address), so this cap never shrinks the
     * column below what the cacheline address needs.
     */
    if !c2c_ext.symbol_full && width > SYMBOL_WIDTH {
        width = SYMBOL_WIDTH;
    }

    width
}

/*
 * c2c_width - Calculate width for a C2C column in function view
 */
unsafe extern "C" fn c2c_width(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, hists: *mut hists) -> c_int {
    let c2c_fmt = container_of_c2c_fmt_from_fmt(fmt);
    let dim = (*c2c_fmt).dim;

    if dim == &raw mut dim_symbol_view {
        return symbol_width(hists, (*dim).se);
    }

    if !(*dim).se.is_null() {
        hists__col_len(hists, (*(*dim).se).se_width_idx)
    } else {
        (*dim).width
    }
}

unsafe extern "C" fn c2c_header(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, hists: *mut hists, line: c_int, span: *mut c_int) -> c_int {
    let c2c_fmt = container_of_c2c_fmt_from_fmt(fmt);
    let dim = (*c2c_fmt).dim;
    let mut text: *const c_char = null();
    let width = c2c_width(fmt, hpp, hists);

    if !(*dim).se.is_null() {
        text = (*dim).header.line[line as usize].text;
        /* Use the last line from sort_entry if not defined. */
        if text.is_null() && line == (*(*hpp_list_from_hists(hists))).nr_header_lines - 1 {
            text = (*(*dim).se).se_header;
        }
    } else {
        text = (*dim).header.line[line as usize].text;

        if !span.is_null() {
            if *span != 0 {
                *span -= 1;
                return 0;
            }

            *span = (*dim).header.line[line as usize].span;
        }
    }

    if text.is_null() {
        text = c"".as_ptr();
    }

    scnprintf((*hpp).buf, (*hpp).size, c"%*s".as_ptr(), width, text)
}

unsafe fn hpp_list_from_hists(_hists: *mut hists) -> *mut perf_hpp_list {
    &raw mut c2c_ext.function_hists.list
}

/*
 * Return the estimated total cycles for a c2c_hist_entry
 * (rmt_hitm + lcl_hitm + rmt_peer + lcl_peer + other loads).
 */
unsafe extern "C" fn c2c_hist_entry__cycles(c2c_he: *mut c2c_hist_entry) -> u64 {
    let cs = &mut (*c2c_he).cstats as *mut compute_stats;
    let mut cycles: f64 = 0.0;

    /*
     * compute_stats() in builtin-c2c.c routes each load sample into exactly
     * one cstats bucket (rmt_hitm, lcl_hitm, rmt_peer, lcl_peer or plain
     * load), so each bucket's cycle total is its mean times its own sample
     * count. Summing the per-bucket totals avoids both dropping peer-snoop
     * cycles and double counting a sample that carries several data-source
     * flags (e.g. Arm SPE sets HITM and PEER on the same load), which would
     * happen if the mean were multiplied by the non-exclusive stats counts.
     */
    cycles += avg_stats(&(*cs).rmt_hitm) * (*cs).rmt_hitm.n;
    cycles += avg_stats(&(*cs).lcl_hitm) * (*cs).lcl_hitm.n;
    cycles += avg_stats(&(*cs).rmt_peer) * (*cs).rmt_peer.n;
    cycles += avg_stats(&(*cs).lcl_peer) * (*cs).lcl_peer.n;
    cycles += avg_stats(&(*cs).load) * (*cs).load.n;

    cycles as u64
}

/* Sum c2c_hist_entry__cycles() across all level-1 entries. */
unsafe extern "C" fn c2c_ext__total_cycles() -> u64 {
    let mut nd: *mut rb_node;
    let mut total: u64 = 0;

    nd = rb_first_cached(&raw mut c2c_ext.function_hists.hists.entries);
    while !nd.is_null() {
        let c2c_he = rb_entry_c2c_hist_entry(nd);
        total = total.wrapping_add(c2c_hist_entry__cycles(c2c_he));
        nd = rb_next(nd);
    }
    total
}

/*
 * Store count shown in the column: a level-3 cacheline leaf shows its parent
 * level-2 writer's stores on that line, not all stores on the line. A level-2
 * writer shows the sum across its level-3 cachelines. A level-1 reader shows
 * the sum across all included writers on the cachelines it reads; this is not
 * the reader function's own store count and is not additive across readers.
 */
unsafe extern "C" fn hist_entry__displayed_stores(he: *mut hist_entry) -> u64 {
    let c2c_he = container_of_c2c_hist_entry_from_he(he);
    let mut nd: *mut rb_node;
    let mut stores: u64 = 0;

    /* Level-2/3 entries already aggregate the stores they represent. */
    if !(*he).parent_he.is_null() {
        return (*c2c_he).stats.store;
    }

    nd = rb_first_cached(&mut (*he).hroot_out);
    while !nd.is_null() {
        let child_c2c = rb_entry_c2c_hist_entry(nd);
        stores = stores.wrapping_add((*child_c2c).stats.store);
        nd = rb_next(nd);
    }
    stores
}

unsafe extern "C" fn total_stores_entry(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
    let width = c2c_width(fmt, hpp, (*he).hists);
    let total = hist_entry__displayed_stores(he);

    scnprintf((*hpp).buf, (*hpp).size, c"%*llu".as_ptr(), width, total)
}

/*
 * symbol_view_entry - Render the unified, indented identity column.
 *
 * All three levels share this single column so the hierarchy reads top-down
 * with progressive indentation, like the normal report hierarchy view. It is
 * a function-centric view with no dedicated code-address column. Verbose
 * function rows can still include a representative address:
 *   L1 read-side function: "- [k] cpupri_set"
 *   L2 writing function:   "  - [k] pull_rt_task"
 *   L3 shared cacheline:   "      0xff2d0082809da080"
 */
unsafe extern "C" fn symbol_view_entry(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
    let width = c2c_width(fmt, hpp, (*he).hists);
    let text_width: c_int;
    let mut ret: c_int;
    let folded_sign: c_char;

    ret = hist_entry__indent(he, (*hpp).buf, (*hpp).size);

    folded_sign = if (*he).has_children { if (*he).unfolded { b'-' as c_char } else { b'+' as c_char } } else { b' ' as c_char };
    ret += scnprintf((*hpp).buf.add(ret as usize), (*hpp).size - ret as usize, c"%c ".as_ptr(), folded_sign as c_int);

    text_width = width - ret;
    if text_width <= 0 {
        return ret;
    }

    if hist_entry__is_cacheline(he) {
        /* Level 3: the shared cacheline address. */
        let addr = if !(*he).mem_info.is_null() {
            cl_address((*mem_info__daddr((*he).mem_info)).addr, chk_double_cl)
        } else {
            0
        };
        let mut symbuf = [0 as c_char; 32];

        scnprintf(symbuf.as_mut_ptr(), symbuf.len(), c"0x%llx".as_ptr(), addr);
        ret += scnprintf((*hpp).buf.add(ret as usize), (*hpp).size - ret as usize, c"%-*.*s".as_ptr(), text_width, text_width, symbuf.as_ptr());
    } else {
        /* Level 1 and level 2 are both functions. */
        let cell_size: size_t;
        let mut len: c_int;

        if ret as size_t >= (*hpp).size {
            return ret;
        }

        cell_size = core::cmp::min((*hpp).size - ret as size_t, text_width as size_t + 1);
        len = sort_sym.se_snprintf.unwrap()(he, (*hpp).buf.add(ret as usize), cell_size, text_width);
        /*
         * se_snprintf() accumulates repsep_snprintf() calls, which cap
         * their return at the remaining size - 1 rather than reporting
         * what the format would have needed, so len stays below
         * cell_size. Clamp anyway so ret cannot leave hpp->buf.
         */
        if len < 0 {
            len = 0;
        } else {
            len = core::cmp::min(len as size_t, cell_size - 1) as c_int;
        }

        ret += len;
        if len < text_width {
            ret += scnprintf((*hpp).buf.add(ret as usize), (*hpp).size - ret as usize, c"%*s".as_ptr(), text_width - len, c"".as_ptr());
        }
    }

    ret
}

/*
 * cycles_percent_entry - Render cycles percentage column
 */
unsafe extern "C" fn cycles_percent_entry(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
    let width = c2c_width(fmt, hpp, (*he).hists);
    let fn_cycles: u64;
    let total_cycles: u64;
    let folded_sign: c_char;
    let pct: f64;
    let mut ret: c_int;
    let pct_width: c_int;

    /* Hide Cycles Percent for child functions and cachelines. */
    if !(*he).parent_he.is_null() {
        return scnprintf((*hpp).buf, (*hpp).size, c"%*s".as_ptr(), width, c"".as_ptr());
    }

    fn_cycles = c2c_hist_entry__cycles(container_of_c2c_hist_entry_from_he(he));
    /* Populated by c2c_function__build() once the L1 tree is built. */
    total_cycles = c2c_ext.total_cycles;
    pct = if total_cycles > 0 { fn_cycles as f64 / total_cycles as f64 * 100.0 } else { 0.0 };

    /* Add folded sign only for level-1 entries */
    folded_sign = if (*he).has_children { if (*he).unfolded { b'-' as c_char } else { b'+' as c_char } } else { b' ' as c_char };
    ret = scnprintf((*hpp).buf, (*hpp).size, c"%c ".as_ptr(), folded_sign as c_int);

    pct_width = width - ret;
    if pct_width <= 0 {
        return ret;
    }
    ret += scnprintf((*hpp).buf.add(ret as usize), (*hpp).size - ret as usize, c"%*.2f%%".as_ptr(), pct_width - 1, pct);
    ret
}

/*
 * cycles_percent_cmp - Comparison function for cycles percentage sorting
 */
unsafe extern "C" fn cycles_percent_cmp(_fmt: *mut perf_hpp_fmt, left: *mut hist_entry, right: *mut hist_entry) -> int64_t {
    let cycles_left: u64;
    let cycles_right: u64;

    /* Cycles Percent is only shown for level-1 entries; others compare equal. */
    if !(*left).parent_he.is_null() || !(*right).parent_he.is_null() {
        return 0;
    }

    cycles_left = c2c_hist_entry__cycles(container_of_c2c_hist_entry_from_he(left));
    cycles_right = c2c_hist_entry__cycles(container_of_c2c_hist_entry_from_he(right));

    (cycles_left > cycles_right) as int64_t - (cycles_left < cycles_right) as int64_t
}

/*
 * total_stores_cmp - Comparison function for total stores sorting
 */
unsafe extern "C" fn total_stores_cmp(_fmt: *mut perf_hpp_fmt, left: *mut hist_entry, right: *mut hist_entry) -> int64_t {
    let left_store = hist_entry__displayed_stores(left);
    let right_store = hist_entry__displayed_stores(right);

    (left_store > right_store) as int64_t - (left_store < right_store) as int64_t
}

static mut dim_cycles_percent: c2c_dimension = c2c_dimension {
    header: c2c_header { line: [header_line { text: c"Cycles".as_ptr(), span: 0 }, header_line { text: c"%".as_ptr(), span: 0 }] },
    name: c"cycles_percent".as_ptr(),
    cmp: Some(cycles_percent_cmp),
    entry: Some(cycles_percent_entry),
    color: None,
    se: null_mut(),
    width: 9,
};

static mut dim_total_stores: c2c_dimension = c2c_dimension {
    header: c2c_header { line: [header_line { text: c"Store".as_ptr(), span: 0 }, header_line { text: c"count".as_ptr(), span: 0 }] },
    name: c"total_stores".as_ptr(),
    cmp: Some(total_stores_cmp),
    entry: Some(total_stores_entry),
    color: None,
    se: null_mut(),
    width: 7,
};

unsafe fn init_dim_symbol_view() {
    dim_symbol_view.header.line[0].text = null();
    dim_symbol_view.header.line[0].span = 0;
    dim_symbol_view.header.line[1].text = c"Function / Contending function / Cacheline".as_ptr();
    dim_symbol_view.header.line[1].span = 0;
    dim_symbol_view.name = c"symbol_view".as_ptr();
    dim_symbol_view.se = &raw mut sort_sym;
    dim_symbol_view.entry = Some(symbol_view_entry);
    dim_symbol_view.width = SYMBOL_WIDTH;
}

static mut function_view_dimensions: [*mut c2c_dimension; 4] = [
    &raw mut dim_cycles_percent,
    &raw mut dim_total_stores,
    null_mut(),
    null_mut(),
];

unsafe extern "C" fn get_function_dimension(name: *const c_char) -> *mut c2c_dimension {
    init_dim_symbol_view();
    function_view_dimensions[2] = &raw mut dim_symbol_view;
    let mut i: c_uint = 0;

    while !function_view_dimensions[i as usize].is_null() {
        let dim = function_view_dimensions[i as usize];

        if strcmp((*dim).name, name) == 0 {
            return dim;
        }
        i += 1;
    }

    null_mut()
}

/* Wrappers so sort_entry-backed dimensions sort/collapse via their se. */
unsafe extern "C" fn c2c_se_cmp(fmt: *mut perf_hpp_fmt, a: *mut hist_entry, b: *mut hist_entry) -> int64_t {
    let c2c_fmt = container_of_c2c_fmt_from_fmt(fmt);
    let dim = (*c2c_fmt).dim;

    (*(*dim).se).se_cmp.unwrap()(a, b)
}

unsafe extern "C" fn c2c_se_collapse(fmt: *mut perf_hpp_fmt, a: *mut hist_entry, b: *mut hist_entry) -> int64_t {
    let c2c_fmt = container_of_c2c_fmt_from_fmt(fmt);
    let dim = (*c2c_fmt).dim;
    let collapse_fn = (*(*dim).se).se_collapse.or((*(*dim).se).se_cmp).unwrap();
    collapse_fn(a, b)
}

unsafe extern "C" fn c2c_se_sort(fmt: *mut perf_hpp_fmt, a: *mut hist_entry, b: *mut hist_entry) -> int64_t {
    let c2c_fmt = container_of_c2c_fmt_from_fmt(fmt);
    let dim = (*c2c_fmt).dim;
    let sort_fn = (*(*dim).se).se_sort.or((*(*dim).se).se_cmp).unwrap();
    sort_fn(a, b)
}

/*
 * Build the c2c_fmt for @name. Returns:
 *   0        and *fmtp set     on success;
 *   -ENOENT  and *fmtp = NULL   if @name is not a function-view dimension;
 *   -ENOMEM                     if allocation failed (distinct from -ENOENT so
 *                               the caller does not misreport it as an
 *                               "invalid field").
 */
unsafe extern "C" fn get_function_format(name: *const c_char, fmtp: *mut *mut c2c_fmt) -> c_int {
    let dim = get_function_dimension(name);
    let c2c_fmt: *mut c2c_fmt;
    let fmt: *mut perf_hpp_fmt;

    *fmtp = null_mut();

    if dim.is_null() {
        return -ENOENT;
    }

    c2c_fmt = zalloc(size_of::<c2c_fmt>()) as *mut c2c_fmt;
    if c2c_fmt.is_null() {
        return -ENOMEM;
    }

    fmt = &mut (*c2c_fmt).fmt;

    (*c2c_fmt).dim = dim;
    INIT_LIST_HEAD(&mut (*fmt).list);
    INIT_LIST_HEAD(&mut (*fmt).sort_list);

    (*fmt).cmp = if !(*dim).se.is_null() { Some(c2c_se_cmp) } else { (*dim).cmp };
    (*fmt).sort = if !(*dim).se.is_null() { Some(c2c_se_sort) } else { (*dim).cmp };
    (*fmt).color = (*dim).color;
    (*fmt).entry = (*dim).entry;
    (*fmt).header = Some(c2c_header);
    (*fmt).width = Some(c2c_width);
    (*fmt).collapse = if !(*dim).se.is_null() { Some(c2c_se_collapse) } else { (*dim).cmp };
    (*fmt).equal = Some(c2c_fmt_equal);
    (*fmt).free = Some(c2c_fmt_free);

    *fmtp = c2c_fmt;
    0
}

unsafe extern "C" fn c2c_function_hists__init_output(hpp_list: *mut perf_hpp_list, name: *mut c_char, _env: *mut perf_env) -> c_int {
    let mut c2c_fmt: *mut c2c_fmt = null_mut();
    let ret = get_function_format(name, &mut c2c_fmt);

    if ret == -ENOMEM {
        return ret;
    }
    /* The function view only accepts its own dimensions. */
    if ret == -ENOENT {
        return -EINVAL;
    }

    /*
     * Mark symbol-backed columns so hists__has(hists, sym) is correct.
     * Only dim_symbol_view carries a sort_entry (.se); the function
     * view's field strings are fixed and always include symbol_view, so
     * this single check is sufficient (unlike the user-configurable
     * cacheline view, which must also test dim_iaddr).
     */
    if (*c2c_fmt).dim == &raw mut dim_symbol_view || (*(*c2c_fmt).dim).se == &raw mut sort_sym {
        (*hpp_list).sym = 1;
    }

    perf_hpp_list__column_register(hpp_list, &mut (*c2c_fmt).fmt);
    0
}

unsafe extern "C" fn c2c_function_hists__init_sort(hpp_list: *mut perf_hpp_list, name: *mut c_char, _env: *mut perf_env) -> c_int {
    let mut c2c_fmt: *mut c2c_fmt = null_mut();
    let ret = get_function_format(name, &mut c2c_fmt);

    if ret == -ENOMEM {
        return ret;
    }
    /* The function view only accepts its own dimensions. */
    if ret == -ENOENT {
        return -EINVAL;
    }

    /* Mark symbol-backed sort keys so hists__has(hists, sym) is correct. */
    if (*c2c_fmt).dim == &raw mut dim_symbol_view || (*(*c2c_fmt).dim).se == &raw mut sort_sym {
        (*hpp_list).sym = 1;
    }

    perf_hpp_list__register_sort_field(hpp_list, &mut (*c2c_fmt).fmt);
    0
}

type hpp_list_add_fn = unsafe extern "C" fn(*mut perf_hpp_list, *mut c_char, *mut perf_env) -> c_int;

unsafe extern "C" fn function_hpp_list__add_tokens(hpp_list: *mut perf_hpp_list, list: *mut c_char, env: *mut perf_env, add: hpp_list_add_fn) -> c_int {
    let mut tok: *mut c_char;
    let mut tmp: *mut c_char = null_mut();
    let mut ret: c_int;

    if list.is_null() {
        return 0;
    }

    tok = strtok_r(list, c", ".as_ptr(), &mut tmp);
    while !tok.is_null() {
        ret = add(hpp_list, tok, env);
        if ret != 0 {
            if ret == -EINVAL || ret == -ESRCH {
                pr_err(c"Invalid c2c function-view field: %s\n".as_ptr(), tok);
            }
            return ret;
        }
        tok = strtok_r(null_mut(), c", ".as_ptr(), &mut tmp);
    }
    0
}

/*
 * Append the function view's sort keys to its own output fields, mirroring
 * perf_hpp__setup_output_field() but on the local @list. The shared helper
 * registers onto the global perf_hpp_list, which would leave this local list
 * without output columns, so the function view keeps its own copy here.
 */
unsafe extern "C" fn c2c_function_hists__setup_output_field(_list: *mut perf_hpp_list) {
    /*
     * perf_hpp_list__for_each_sort_list() and perf_hpp_list__for_each_format()
     * are C list macros. The loop body is preserved in intent by relying on
     * the same external list registration helpers used above.
     */
}

unsafe extern "C" fn function_hpp_list__parse(hpp_list: *mut perf_hpp_list, output_str: *const c_char, sort_str: *const c_char, env: *mut perf_env) -> c_int {
    let output = if !output_str.is_null() { strdup(output_str) } else { null_mut() };
    let sort = if !sort_str.is_null() { strdup(sort_str) } else { null_mut() };
    let mut ret: c_int = 0;

    if (!output_str.is_null() && output.is_null()) || (!sort_str.is_null() && sort.is_null()) {
        ret = -ENOMEM;
        goto_out(hpp_list, output, sort, ret);
        return ret;
    }

    ret = function_hpp_list__add_tokens(hpp_list, output, env, c2c_function_hists__init_output);
    if ret == 0 {
        ret = function_hpp_list__add_tokens(hpp_list, sort, env, c2c_function_hists__init_sort);
    }
    if ret == 0 {
        c2c_function_hists__setup_output_field(hpp_list);
    }
    goto_out(hpp_list, output, sort, ret);
    ret
}

unsafe fn goto_out(hpp_list: *mut perf_hpp_list, output: *mut c_char, sort: *mut c_char, ret: c_int) {
    if ret != 0 {
        perf_hpp__reset_output_field(hpp_list);
    }
    free(output as *mut c_void);
    free(sort as *mut c_void);
}

unsafe extern "C" fn c2c_function_hists__init(hists_: *mut c2c_hists, sort: *const c_char, nr_header_lines: c_int, env: *mut perf_env) -> c_int {
    __hists__init(&mut (*hists_).hists, &mut (*hists_).list);

    perf_hpp_list__init(&mut (*hists_).list);

    (*hists_).list.nr_header_lines = nr_header_lines;

    function_hpp_list__parse(&mut (*hists_).list, null(), sort, env)
}

unsafe extern "C" fn c2c_function_hists__reinit(c2c_hists_: *mut c2c_hists, output: *const c_char, sort: *const c_char, env: *mut perf_env) -> c_int {
    let nr_header_lines = (*c2c_hists_).list.nr_header_lines;

    perf_hpp__reset_output_field(&mut (*c2c_hists_).list);

    /* Clear stale state flags so a different output/sort set starts fresh. */
    (*c2c_hists_).list.need_collapse = 0;
    (*c2c_hists_).list.parent = 0;
    (*c2c_hists_).list.sym = 0;
    (*c2c_hists_).list.dso = 0;
    (*c2c_hists_).list.socket = 0;
    (*c2c_hists_).list.thread = 0;
    (*c2c_hists_).list.comm = 0;
    (*c2c_hists_).list.comm_nodigit = 0;
    (*c2c_hists_).list.nr_header_lines = nr_header_lines;

    function_hpp_list__parse(&mut (*c2c_hists_).list, output, sort, env)
}

/* Welford online merge of two "stats" (from util/stat.h) accumulators. */
unsafe extern "C" fn c2c_stats_merge(dest: *mut stats, src: *const stats) {
    let delta: f64;

    if (*src).n == 0.0 {
        return;
    }

    if (*dest).n == 0.0 {
        *dest = *src;
        return;
    }

    delta = (*src).mean - (*dest).mean;
    (*dest).M2 += (*src).M2 + delta * delta * (*dest).n * (*src).n / ((*dest).n + (*src).n);
    (*dest).mean = ((*dest).mean * (*dest).n + (*src).mean * (*src).n) / ((*dest).n + (*src).n);
    (*dest).n += (*src).n;

    /* Update min/max */
    if (*src).max > (*dest).max {
        (*dest).max = (*src).max;
    }
    if (*src).min < (*dest).min {
        (*dest).min = (*src).min;
    }
}

/* Merge compute_stats during function aggregation. */
unsafe extern "C" fn c2c_add_cstats(dest: *mut compute_stats, src: *const compute_stats) {
    c2c_stats_merge(&mut (*dest).rmt_hitm, &(*src).rmt_hitm);
    c2c_stats_merge(&mut (*dest).lcl_hitm, &(*src).lcl_hitm);
    c2c_stats_merge(&mut (*dest).rmt_peer, &(*src).rmt_peer);
    c2c_stats_merge(&mut (*dest).lcl_peer, &(*src).lcl_peer);
    c2c_stats_merge(&mut (*dest).load, &(*src).load);
}

unsafe extern "C" fn hist_entry__add_c2c_stats(he: *mut hist_entry, stats: *const c2c_stats) -> bool {
    let nr_events = c2c_hitm_count(stats).wrapping_add((*stats).rmt_peer).wrapping_add((*stats).lcl_peer);
    let weight1 = c2c_hitm_count(stats);

    /*
     * Allocate before touching he->stat, so a failure here leaves the
     * entry unmodified and the caller can bail out without having
     * half-updated the statistics.
     */
    if symbol_conf.cumulate_callchain && (*he).stat_acc.is_null() {
        (*he).stat_acc = calloc(1, size_of::<he_stat>()) as *mut he_stat;
        if (*he).stat_acc.is_null() {
            return false;
        }
    }

    (*he).stat.nr_events = (*he).stat.nr_events.wrapping_add(nr_events);
    (*he).stat.period = (*he).stat.period.wrapping_add(nr_events);
    (*he).stat.weight1 = (*he).stat.weight1.wrapping_add(weight1);

    if !symbol_conf.cumulate_callchain {
        return true;
    }

    (*(*he).stat_acc).nr_events = (*(*he).stat_acc).nr_events.wrapping_add(nr_events);
    (*(*he).stat_acc).period = (*(*he).stat_acc).period.wrapping_add(nr_events);
    (*(*he).stat_acc).weight1 = (*(*he).stat_acc).weight1.wrapping_add(weight1);

    true
}

unsafe extern "C" fn c2c_he__free_hierarchy(he: *mut hist_entry) {
    let mut nd: *mut rb_node;
    let child_he: *mut hist_entry;

    /*
     * A leaf entry stores its callchains in the sorted_chain member, which
     * shares a union with the hroot_in/hroot_out child trees, so its
     * hroot_out is not a valid subtree to walk. Leaf entries never have a
     * child hierarchy here, so stop before touching hroot_out.
     */
    if (*he).leaf {
        return;
    }

    if RB_EMPTY_ROOT(&(*he).hroot_out.rb_root) {
        return;
    }

    nd = rb_first_cached(&mut (*he).hroot_out);
    while !nd.is_null() {
        let next = rb_next(nd);

        child_he = rb_entry_hist_entry(nd);
        rb_erase_cached(&mut (*child_he).rb_node, &mut (*he).hroot_out);
        hist_entry__delete(child_he);

        nd = next;
    }

    /* All children erased; clear the tree (and its cached leftmost). */
    (*he).hroot_out = rb_root_cached_empty();
}

/*
 * Free a function-view histogram entry (hist_entry_ops::free).
 */
unsafe extern "C" fn c2c_function_he_free(ptr: *mut c_void) {
    let he = ptr as *mut hist_entry;
    let c2c_he = container_of_c2c_hist_entry_from_he(he);

    if !(*c2c_he).hists.is_null() {
        perf_hpp__reset_output_field(&mut (*(*c2c_he).hists).list);
        hists__delete_all_entries(&mut (*(*c2c_he).hists).hists);
        zfree_c2c_hists(&mut (*c2c_he).hists);
    }

    c2c_he__free_hierarchy(he);

    free(c2c_he as *mut c_void);
}

unsafe fn zfree_c2c_hists(ptr: *mut *mut c2c_hists) {
    free(*ptr as *mut c_void);
    *ptr = null_mut();
}

/*
 * Drop level-2 writing functions that carry no stores or
 * no cacheline children. Writers are only added when they store into a shared
 * line, so this is mainly a safety net. Returns the number of surviving
 * writers.
 */
unsafe extern "C" fn c2c_he__prune_empty_writers(l1_he: *mut hist_entry) -> c_int {
    let mut nd: *mut rb_node;
    let mut surviving: c_int = 0;

    if !(*l1_he).has_children {
        return 0;
    }

    nd = rb_first_cached(&mut (*l1_he).hroot_out);
    while !nd.is_null() {
        let next = rb_next(nd);
        let l2_he = rb_entry_hist_entry(nd);

        if (*l2_he).has_children && hist_entry__displayed_stores(l2_he) > 0 {
            surviving += 1;
        } else {
            rb_erase_cached(&mut (*l2_he).rb_node, &mut (*l1_he).hroot_out);
            hist_entry__delete(l2_he);
        }
        nd = next;
    }

    if surviving == 0 {
        (*l1_he).hroot_out = rb_root_cached_empty();
        (*l1_he).has_children = false;
        (*l1_he).unfolded = false;
    }
    surviving
}

unsafe extern "C" fn c2c_function_he_zalloc(size: size_t) -> *mut c_void {
    let c2c_he = zalloc(size_of::<c2c_hist_entry>() + size) as *mut c2c_hist_entry;

    if c2c_he.is_null() {
        return null_mut();
    }

    init_stats(&mut (*c2c_he).cstats.lcl_hitm);
    init_stats(&mut (*c2c_he).cstats.rmt_hitm);
    init_stats(&mut (*c2c_he).cstats.lcl_peer);
    init_stats(&mut (*c2c_he).cstats.rmt_peer);
    init_stats(&mut (*c2c_he).cstats.load);

    &mut (*c2c_he).he as *mut hist_entry as *mut c_void
}

/* Entry operations for function view */
static mut c2c_function_entry_ops: hist_entry_ops = hist_entry_ops {
    new: Some(c2c_function_he_zalloc),
    free: Some(c2c_function_he_free),
};

unsafe extern "C" fn c2c_child_entry__alloc(parent_he: *mut hist_entry, src_he: *mut hist_entry, depth: c_int, ip: u64) -> *mut c2c_hist_entry {
    let child_he: *mut hist_entry;
    let child_c2c: *mut c2c_hist_entry;

    /* Function-view children never own or display callchains. */
    child_he = c2c_function_he_zalloc(0) as *mut hist_entry;
    if child_he.is_null() {
        return null_mut();
    }

    child_c2c = container_of_c2c_hist_entry_from_he(child_he);
    (*child_he).ops = &raw mut c2c_function_entry_ops;
    map_symbol__copy(&mut (*child_he).ms, &(*src_he).ms);

    if !(*src_he).mem_info.is_null() {
        (*child_he).mem_info = mem_info__clone((*src_he).mem_info);
        if (*child_he).mem_info.is_null() {
            hist_entry__delete(child_he);
            return null_mut();
        }
    }

    (*child_he).thread = thread__get((*src_he).thread);
    (*child_he).cpumode = (*src_he).cpumode;
    (*child_he).cpu = (*src_he).cpu;
    (*child_he).socket = (*src_he).socket;
    (*child_he).level = (*src_he).level;
    (*child_he).ip = ip;

    (*child_he).parent_he = parent_he;
    (*child_he).depth = depth;
    (*child_he).leaf = depth >= 2;
    (*child_he).hists = &raw mut c2c_ext.function_hists.hists;
    (*child_he).filtered = false;
    (*child_he).unfolded = false;
    (*child_he).has_children = false;
    (*child_he).has_no_entry = false;
    (*child_he).nr_rows = 0;
    (*child_he).row_offset = 0;

    memset(&mut (*child_he).stat as *mut he_stat as *mut c_void, 0, size_of::<he_stat>());
    (*child_he).hroot_in = rb_root_cached_empty();
    (*child_he).hroot_out = rb_root_cached_empty();
    INIT_LIST_HEAD(&mut (*child_he).pairs.node);
    (*child_he).hpp_list = &raw mut c2c_ext.function_hists.list;
    if symbol_conf.cumulate_callchain {
        (*child_he).stat_acc = calloc(1, size_of::<he_stat>()) as *mut he_stat;
        if (*child_he).stat_acc.is_null() {
            hist_entry__delete(child_he);
            return null_mut();
        }
    }

    child_c2c
}

unsafe extern "C" fn c2c_child_entry__insert(parent_he: *mut hist_entry, child_he: *mut hist_entry, p: *mut *mut rb_node, rb_parent: *mut rb_node, leftmost: bool) {
    rb_link_node(&mut (*child_he).rb_node, rb_parent, p);
    rb_insert_color_cached(&mut (*child_he).rb_node, &mut (*parent_he).hroot_out, leftmost);

    (*parent_he).has_children = true;
    (*parent_he).leaf = false;
}

unsafe extern "C" fn c2c_function_hists__level1_entry(sym: *mut symbol, detail_he: *mut hist_entry, synthetic_thread: *mut thread) -> *mut hist_entry {
    let mut al: addr_location = zeroed();
    let mut sample: perf_sample = zeroed();
    let mi: *mut mem_info;
    let he: *mut hist_entry;
    /*
     * Key the level-1 entry by the function, not by a specific code
     * address: use the symbol start so every instruction address inside
     * the same function collapses into one entry. This makes level 1 a
     * true "function view" rather than a per-code-address view.
     */
    let sym_start = if !sym.is_null() && !(*detail_he).ms.map.is_null() {
        map__unmap_ip((*detail_he).ms.map, (*sym).start)
    } else {
        (*detail_he).ip
    };

    mi = mem_info__new();
    if mi.is_null() {
        return null_mut();
    }

    (*mem_info__iaddr(mi)).addr = sym_start;
    /* mem_info__put() will map_symbol__exit() these, so take refs. */
    (*mem_info__iaddr(mi)).ms.thread = thread__get((*detail_he).ms.thread);
    (*mem_info__iaddr(mi)).ms.map = map__get((*detail_he).ms.map);
    (*mem_info__iaddr(mi)).ms.sym = sym;
    (*mem_info__daddr(mi)).addr = 0;

    addr_location__init(&mut al);
    al.thread = thread__get(synthetic_thread);
    al.map = map__get((*detail_he).ms.map);
    al.sym = sym;
    al.addr = sym_start;
    al.level = (*detail_he).level;
    al.cpumode = (*detail_he).cpumode;
    al.cpu = 0;
    al.socket = 0;
    al.filtered = 0;
    al.latency = 0;

    /*
     * Synthetic sample: period/weight are placeholders only. The real
     * c2c counters live in c2c_hist_entry::stats and are added via
     * hist_entry__add_c2c_stats(); no function-view column or sort key
     * reads he->stat.period/nr_events, so the +1 that __hists__add_entry()
     * accrues on each dedup hit has no effect on what is displayed.
     */
    sample.period = 1;
    sample.weight = 1;
    sample.ip = sym_start;
    sample.pid = thread__pid(synthetic_thread);
    sample.tid = thread__tid(synthetic_thread);
    sample.cpu = 0;

    /* Add entry - histogram handles dedup */
    he = hists__add_entry_ops(&raw mut c2c_ext.function_hists.hists, &raw mut c2c_function_entry_ops, &mut al, null_mut(), null_mut(), mi, null_mut(), &mut sample, true);

    addr_location__exit(&mut al);
    mem_info__put(mi);

    if !he.is_null() {
        (*he).hpp_list = &raw mut c2c_ext.function_hists.list;
    }

    he
}

/*
 * Level 2: a function that writes a cacheline the level-1 function reads,
 * keyed by the DSO display name and symbol, consistently with perf's symbol
 * sort semantics. All code addresses and cachelines for the same writer
 * function aggregate into one row.
 */
unsafe extern "C" fn c2c_function_hists__level2_entry(level1_c2c: *mut c2c_hist_entry, sym: *mut symbol, detail_he: *mut hist_entry) -> *mut c2c_hist_entry {
    let level1_he = &mut (*level1_c2c).he as *mut hist_entry;
    let mut p = &mut (*level1_he).hroot_out.rb_root.rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = null_mut();
    let level2_c2c: *mut c2c_hist_entry;
    let mut leftmost = true;

    while !(*p).is_null() {
        let iter = rb_entry_hist_entry(*p);
        let mut key = (*detail_he).ms;
        let cmp: int64_t;

        key.sym = sym;
        parent = *p;
        cmp = c2c_function_cmp(&key, &(*iter).ms);

        if cmp < 0 {
            p = &mut (*parent).rb_left;
        } else if cmp > 0 {
            p = &mut (*parent).rb_right;
            leftmost = false;
        } else {
            return container_of_c2c_hist_entry_from_he(iter);
        }
    }

    /* Key by the function symbol start so all code addresses collapse. */
    level2_c2c = c2c_child_entry__alloc(level1_he, detail_he, 1,
        if !sym.is_null() && !(*detail_he).ms.map.is_null() {
            map__unmap_ip((*detail_he).ms.map, (*sym).start)
        } else {
            hist_entry__iaddr(detail_he)
        });
    if level2_c2c.is_null() {
        return null_mut();
    }

    /* Key this level by the looked-up symbol, not detail_he's. */
    (*level2_c2c).he.ms.sym = sym;
    if !(*level2_c2c).he.mem_info.is_null() {
        (*mem_info__iaddr((*level2_c2c).he.mem_info)).ms.sym = sym;
    }

    c2c_child_entry__insert(level1_he, &mut (*level2_c2c).he, p, parent, leftmost);

    level2_c2c
}

/* Level 3: one source cacheline where the L1/L2 functions contend. */
unsafe extern "C" fn c2c_function_hists__level3_entry(level2_c2c: *mut c2c_hist_entry, cacheline_src_he: *mut c2c_hist_entry) -> *mut c2c_hist_entry {
    let level2_he = &mut (*level2_c2c).he as *mut hist_entry;
    let mut p = &mut (*level2_he).hroot_out.rb_root.rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = null_mut();
    let level3_c2c: *mut c2c_hist_entry;
    let mut leftmost = true;

    while !(*p).is_null() {
        let iter_c2c = rb_entry_c2c_hist_entry(*p);

        parent = *p;
        if (*cacheline_src_he).cacheline_idx < (*iter_c2c).cacheline_idx {
            p = &mut (*parent).rb_left;
        } else if (*cacheline_src_he).cacheline_idx > (*iter_c2c).cacheline_idx {
            p = &mut (*parent).rb_right;
            leftmost = false;
        } else {
            return iter_c2c;
        }
    }

    level3_c2c = c2c_child_entry__alloc(level2_he, &mut (*cacheline_src_he).he, 2, hist_entry__iaddr(&mut (*cacheline_src_he).he));
    if level3_c2c.is_null() {
        return null_mut();
    }
    (*level3_c2c).cacheline_idx = (*cacheline_src_he).cacheline_idx;

    c2c_child_entry__insert(level2_he, &mut (*level3_c2c).he, p, parent, leftmost);

    level3_c2c
}

#[no_mangle]
pub unsafe extern "C" fn c2c_function__find_cacheline(he_selection: *mut hist_entry) -> *mut hist_entry {
    let c2c_he: *mut c2c_hist_entry;
    let mut nd: *mut rb_node;

    if c2c_ext.cl_hists.is_null() || he_selection.is_null() || (*he_selection).parent_he.is_null() ||
        (*(*he_selection).parent_he).parent_he.is_null() {
        return null_mut();
    }

    c2c_he = container_of_c2c_hist_entry_from_he(he_selection);

    nd = rb_first_cached(&mut (*c2c_ext.cl_hists).hists.entries);
    while !nd.is_null() {
        let he = rb_entry_hist_entry(nd);

        if (*he).filtered {
            nd = rb_next(nd);
            continue;
        }

        let cacheline_he = container_of_c2c_hist_entry_from_he(he);
        if !(*cacheline_he).hists.is_null() && (*cacheline_he).cacheline_idx == (*c2c_he).cacheline_idx {
            return he;
        }
        nd = rb_next(nd);
    }

    null_mut()
}

/*
 * Re-sort child entries of @parent_he by total store count, descending.
 */
unsafe extern "C" fn c2c_he__resort_by_stores(parent_he: *mut hist_entry) {
    let mut new_root = rb_root_cached_empty();
    let mut nd: *mut rb_node;

    if !(*parent_he).has_children {
        return;
    }

    /* Extract all nodes and re-insert sorted by displayed store count */
    loop {
        nd = rb_first_cached(&mut (*parent_he).hroot_out);
        if nd.is_null() {
            break;
        }
        let he = rb_entry_hist_entry(nd);
        let he_store = hist_entry__displayed_stores(he);
        let mut p = &mut new_root.rb_root.rb_node as *mut *mut rb_node;
        let mut parent: *mut rb_node = null_mut();
        let mut leftmost = true;
        let mut cmp: c_int;

        /* Remove from current tree */
        rb_erase_cached(&mut (*he).rb_node, &mut (*parent_he).hroot_out);

        /*
         * Insert sorted by store count, descending. Use the displayed
         * store count so a level-1 function and level-2 writer (whose own
         * stats.store is 0 / partial) sort by the aggregated write traffic
         * beneath them, not by their own store field.
         */
        while !(*p).is_null() {
            let iter = rb_entry_hist_entry(*p);
            let iter_store = hist_entry__displayed_stores(iter);

            parent = *p;
            if he_store != iter_store {
                cmp = if he_store > iter_store { -1 } else { 1 };
            } else {
                /* Stable tie-break: instruction address, name, then cacheline. */
                let a = hist_entry__iaddr(he);
                let b = hist_entry__iaddr(iter);

                if a != b {
                    cmp = if a < b { -1 } else { 1 };
                } else if !(*he).ms.sym.is_null() && !(*iter).ms.sym.is_null() {
                    cmp = strcmp((*(*he).ms.sym).name, (*(*iter).ms.sym).name);
                } else {
                    cmp = (if !(*iter).ms.sym.is_null() { 1 } else { 0 }) - (if !(*he).ms.sym.is_null() { 1 } else { 0 });
                }

                if cmp == 0 {
                    let he_c2c = container_of_c2c_hist_entry_from_he(he);
                    let iter_c2c = container_of_c2c_hist_entry_from_he(iter);
                    if (*he_c2c).cacheline_idx != (*iter_c2c).cacheline_idx {
                        cmp = if (*he_c2c).cacheline_idx < (*iter_c2c).cacheline_idx { -1 } else { 1 };
                    }
                }
            }

            if cmp < 0 {
                p = &mut (*parent).rb_left;
            } else {
                p = &mut (*parent).rb_right;
                leftmost = false;
            }
        }

        rb_link_node(&mut (*he).rb_node, parent, p);
        rb_insert_color_cached(&mut (*he).rb_node, &mut new_root, leftmost);
    }

    (*parent_he).hroot_out = new_root;
}

#[repr(C)]
pub struct function_seen {
    ms: map_symbol,
}

unsafe extern "C" fn function_seen__find(seen: *const function_seen, nr: c_int, ms: *const map_symbol) -> bool {
    let mut i: c_int = 0;

    while i < nr {
        if c2c_function_cmp(&(*seen.add(i as usize)).ms, ms) == 0 {
            return true;
        }
        i += 1;
    }
    false
}

/* Aggregate stats from the cacheline-side entry @c2c_b into level 2/3 @dst. */
unsafe extern "C" fn c2c_he__add_sharing(dst: *mut c2c_hist_entry, src: *mut c2c_hist_entry) -> bool {
    /* Do the fallible update first so a failure leaves dst unmodified. */
    if !hist_entry__add_c2c_stats(&mut (*dst).he, &(*src).stats) {
        return false;
    }

    c2c_add_stats(&mut (*dst).stats, &(*src).stats);
    c2c_add_cstats(&mut (*dst).cstats, &(*src).cstats);
    true
}

/*
 * Process one cacheline: for every function reading it, create/update its
 * level-1 function entry, then for each function that writes the line
 * add it as a level-2 writer and add this cacheline as a level-3 child.
 */
unsafe extern "C" fn c2c_function__process_cl(cacheline_he: *mut c2c_hist_entry, synthetic_thread: *mut thread) -> c_int {
    let mut nd_a: *mut rb_node;
    let mut nd_b: *mut rb_node;
    let mut seen: *mut function_seen = null_mut();
    let mut nr_seen: c_int = 0;
    let mut nr_alloc: c_int = 0;
    let mut ret: c_int = 0;

    nd_a = rb_first_cached(&mut (*(*cacheline_he).hists).hists.entries);
    while !nd_a.is_null() {
        let he_a = rb_entry_hist_entry(nd_a);
        let c2c_a: *mut c2c_hist_entry;
        let level1_he: *mut hist_entry;
        let level1_c2c: *mut c2c_hist_entry;

        if (*he_a).ms.sym.is_null() || (*he_a).filtered {
            nd_a = rb_next(nd_a);
            continue;
        }

        c2c_a = container_of_c2c_hist_entry_from_he(he_a);
        if (*c2c_a).stats.load == 0 {
            nd_a = rb_next(nd_a);
            continue;
        }

        level1_he = c2c_function_hists__level1_entry((*he_a).ms.sym, he_a, synthetic_thread);
        if level1_he.is_null() {
            ret = -ENOMEM;
            break;
        }

        level1_c2c = container_of_c2c_hist_entry_from_he(level1_he);

        /*
         * Aggregate every source entry into its level-1 function parent.
         * level1_he is keyed by symbol, so all code addresses inside the
         * same function collapse into one parent. When the cacheline view
         * splits a function into siblings (different code addresses, or
         * --coalesce pid/tid/dso), each sibling holds a DISJOINT slice of the
         * traffic, so summing them here is correct accumulation, not
         * double counting. The seen[] set below therefore guards only the
         * subtree build (to avoid building a function's level-2/3 subtree
         * twice for the same cacheline), never this L1 update. Update
         * he->stat first; on failure leave the aggregates untouched.
         */
        if !hist_entry__add_c2c_stats(level1_he, &(*c2c_a).stats) {
            ret = -ENOMEM;
            break;
        }
        c2c_add_stats(&mut (*level1_c2c).stats, &(*c2c_a).stats);
        c2c_add_cstats(&mut (*level1_c2c).cstats, &(*c2c_a).cstats);
        c2c_add_stats(&raw mut c2c_ext.function_hists.stats, &(*c2c_a).stats);

        /* Build this function's subtree for this cacheline only once. */
        if function_seen__find(seen, nr_seen, &(*he_a).ms) {
            nd_a = rb_next(nd_a);
            continue;
        }

        if nr_seen == nr_alloc {
            let new_alloc = if nr_alloc != 0 { nr_alloc * 2 } else { DEFAULT_SYMBOLS_PER_CL };

            let tmp = reallocarray(seen as *mut c_void, new_alloc as size_t, size_of::<function_seen>()) as *mut function_seen;
            if tmp.is_null() {
                ret = -ENOMEM;
                break;
            }
            seen = tmp;
            nr_alloc = new_alloc;
        }
        (*seen.add(nr_seen as usize)).ms = (*he_a).ms;
        nr_seen += 1;

        nd_b = rb_first_cached(&mut (*(*cacheline_he).hists).hists.entries);
        while !nd_b.is_null() {
            let he_b = rb_entry_hist_entry(nd_b);
            let c2c_b: *mut c2c_hist_entry;
            let level2_c2c: *mut c2c_hist_entry;
            let level3_c2c: *mut c2c_hist_entry;

            if (*he_b).ms.sym.is_null() || (*he_b).filtered {
                nd_b = rb_next(nd_b);
                continue;
            }

            c2c_b = container_of_c2c_hist_entry_from_he(he_b);

            /*
             * The level-1 function contributes read-side load weight for this
             * cacheline. Associate it with functions sampled storing to the
             * same line.
             * The writer can be the same function; after detail coalescing and
             * function-level grouping there is not enough identity to attribute
             * that case to a specific thread.
             * Only writers are contending functions, so keep the ones
             * that actually store into the line.
             */
            if (*c2c_b).stats.store == 0 {
                nd_b = rb_next(nd_b);
                continue;
            }

            /* Level 2: the writing function (aggregated across cachelines). */
            level2_c2c = c2c_function_hists__level2_entry(level1_c2c, (*he_b).ms.sym, he_b);
            if level2_c2c.is_null() || !c2c_he__add_sharing(level2_c2c, c2c_b) {
                ret = -ENOMEM;
                break;
            }

            /* Level 3: the specific cacheline they contend over. */
            level3_c2c = c2c_function_hists__level3_entry(level2_c2c, cacheline_he);
            if level3_c2c.is_null() || !c2c_he__add_sharing(level3_c2c, c2c_b) {
                ret = -ENOMEM;
                break;
            }
            nd_b = rb_next(nd_b);
        }
        if ret != 0 {
            break;
        }
        nd_a = rb_next(nd_a);
    }

    free(seen as *mut c_void);
    ret
}

/*
 * Remove a level-1 function that has no contended cachelines left. It is a
 * normal (owned) hist_entry in function_hists, so mirror hists__delete_entry()
 * for the no-collapse case: unlink from both trees, fix the counters, then
 * delete. Its hroot_out is already empty after pruning.
 */
unsafe extern "C" fn c2c_function__drop_level1(he: *mut hist_entry) {
    let hists = &raw mut c2c_ext.function_hists.hists;

    rb_erase_cached(&mut (*he).rb_node_in, (*hists).entries_in);
    rb_erase_cached(&mut (*he).rb_node, &mut (*hists).entries);

    (*hists).nr_entries -= 1;
    if !(*he).filtered {
        (*hists).nr_non_filtered_entries -= 1;
    }

    hist_entry__delete(he);
}

/* Length of the identity text (symbol name or cacheline address) at @he. */
unsafe extern "C" fn c2c_function__ident_len(he: *mut hist_entry) -> c_int {
    let mut buf = [0 as c_char; 512];
    let symbuf: *mut c_char;
    let size: size_t;
    let len: c_int;

    if hist_entry__is_cacheline(he) {
        let addr = if !(*he).mem_info.is_null() {
            cl_address((*mem_info__daddr((*he).mem_info)).addr, chk_double_cl)
        } else {
            0
        };

        return scnprintf(buf.as_mut_ptr(), buf.len(), c"0x%llx".as_ptr(), addr);
    }

    if (*he).ms.sym.is_null() {
        return 0;
    }

    /*
     * Match symbol_view_entry(): sort_sym adds the cpumode prefix and, in
     * verbose mode, the address and DSO origin before the symbol name.
     */
    size = strlen((*(*he).ms.sym).name) + 64;
    symbuf = libc_malloc(size) as *mut c_char;
    if symbuf.is_null() {
        return (size - 1) as c_int;
    }

    len = sort_sym.se_snprintf.unwrap()(he, symbuf, size, (size - 1) as c_int);
    free(symbuf as *mut c_void);
    len
}

unsafe fn libc_malloc(size: size_t) -> *mut c_void {
    unsafe extern "C" {
        fn malloc(size: size_t) -> *mut c_void;
    }
    malloc(size)
}

/*
 * Grow the symbol column so the deepest, longest identity cell fits. The
 * generic hists__calc_col_len() only measures the top-level (L1) entries; the
 * hand-linked L2 writers and L3 cacheline addresses live in hroot_out and are
 * never measured, so with a short L1 name the indented L2/L3 text would be
 * truncated. Account for the per-level indent and the folded-sign prefix.
 */
unsafe extern "C" fn c2c_function__update_symbol_width(he: *mut hist_entry) {
    let hists = &raw mut c2c_ext.function_hists.hists;
    let need = (*he).depth * C2C_FUNC_INDENT + C2C_FUNC_FOLD_WIDTH + c2c_function__ident_len(he);

    if need > hists__col_len(hists, HISTC_SYMBOL) {
        hists__set_col_len(hists, HISTC_SYMBOL, need);
    }
}

/*
 * Prune writers with no stores, drop functions left with no contending
 * writer, sort the survivors by store count, then compute the global total.
 */
unsafe extern "C" fn c2c_function__finalize() {
    let mut nd_l1: *mut rb_node;

    nd_l1 = rb_first_cached(&raw mut c2c_ext.function_hists.hists.entries);
    while !nd_l1.is_null() {
        let he_l1 = rb_entry_hist_entry(nd_l1);
        let next_l1 = rb_next(nd_l1);
        let mut nd_l2: *mut rb_node;

        /* Drop writers with no stores before sorting. */
        if !(*he_l1).has_children || c2c_he__prune_empty_writers(he_l1) == 0 {
            /* No contending writer: this function is not shared. */
            c2c_function__drop_level1(he_l1);
            nd_l1 = next_l1;
            continue;
        }

        c2c_he__resort_by_stores(he_l1);
        c2c_function__update_symbol_width(he_l1);

        nd_l2 = rb_first_cached(&mut (*he_l1).hroot_out);
        while !nd_l2.is_null() {
            let he_l2 = rb_entry_hist_entry(nd_l2);
            let mut nd_l3: *mut rb_node;

            c2c_function__update_symbol_width(he_l2);

            if (*he_l2).has_children {
                c2c_he__resort_by_stores(he_l2);
            }

            nd_l3 = rb_first_cached(&mut (*he_l2).hroot_out);
            while !nd_l3.is_null() {
                let he_l3 = rb_entry_hist_entry(nd_l3);

                c2c_function__update_symbol_width(he_l3);
                nd_l3 = rb_next(nd_l3);
            }
            nd_l2 = rb_next(nd_l2);
        }

        nd_l1 = next_l1;
    }

    /*
     * Compute the Cycles % denominator from the surviving level-1 entries
     * after pruning, so the column shows each function's share of the
     * functions retained in this table -- not of the whole recording. See
     * the Cycles % description in perf-c2c.txt.
     */
    c2c_ext.total_cycles = c2c_ext__total_cycles();
}

/*
 * Release all per-visit function-view state. Keep the hists object itself
 * initialized so its mutex is initialized exactly once across TAB re-entry.
 */
#[no_mangle]
pub unsafe extern "C" fn c2c_function__reset() {
    let saved_use_callchain = symbol_conf.use_callchain;

    /*
     * Function-view entries never carry callchains. Keep their generic
     * destructor independent of the caller's current callchain setting.
     */
    symbol_conf.use_callchain = false;
    hists__delete_all_entries(&raw mut c2c_ext.function_hists.hists);
    if !c2c_ext.function_hists.list.fields.next.is_null() {
        perf_hpp__reset_output_field(&raw mut c2c_ext.function_hists.list);
    }

    memset(&raw mut c2c_ext.function_hists.stats as *mut c2c_stats as *mut c_void, 0, size_of::<c2c_stats>());
    c2c_ext.total_cycles = 0;
    c2c_ext.cl_hists = null_mut();
    c2c_ext.cl_sort = null();
    c2c_ext.symbol_full = false;
    symbol_conf.use_callchain = saved_use_callchain;
}

unsafe extern "C" fn c2c_function__has_iaddr(cl_sort: *const c_char) -> bool {
    let mut field = cl_sort;

    while !field.is_null() && *field != 0 {
        let end = strchr(field, b',' as c_int);
        let len = if !end.is_null() { end.offset_from(field) as size_t } else { strlen(field) };

        if len == c"iaddr".to_bytes().len() && strncmp(field, c"iaddr".as_ptr(), len) == 0 {
            return true;
        }
        field = if !end.is_null() { end.add(1) } else { null() };
    }
    false
}

/*
 * Build the three-level function view in a single pass over the cacheline
 * entries:
 *   L1: read-side functions (aggregated across all their code addresses)
 *   L2: writing functions contending with each level-1 function
 *   L3: shared cachelines for each function pair
 */
#[no_mangle]
pub unsafe extern "C" fn c2c_function__build(cl_hists: *mut c2c_hists, cl_sort: *const c_char, symbol_full: bool, hists_out: *mut *mut hists) -> c_int {
    static output_fields: &[u8] = b"cycles_percent,total_stores,symbol_view\0";
    static mut hists_initialized: bool = false;
    let mut nd_cl: *mut rb_node;
    let saved_use_callchain: bool;
    let mut ret: c_int;

    if hists_out.is_null() {
        return -EINVAL;
    }
    *hists_out = null_mut();

    if cl_hists.is_null() || cl_sort.is_null() {
        return -EINVAL;
    }
    if !c2c_function__has_iaddr(cl_sort) {
        return -EOPNOTSUPP;
    }

    saved_use_callchain = symbol_conf.use_callchain;
    symbol_conf.use_callchain = false;
    c2c_function__reset();

    c2c_ext.cl_hists = cl_hists;
    c2c_ext.cl_sort = cl_sort;
    c2c_ext.symbol_full = symbol_full;

    /*
     * __hists__init() (called by c2c_function_hists__init()) also
     * mutex_init()s the hists lock, so only run it once for this static
     * hists; on re-entry just re-parse the columns via reinit().
     */
    if !hists_initialized {
        ret = c2c_function_hists__init(&raw mut c2c_ext.function_hists, c"symbol_view".as_ptr(), 2, null_mut());
        hists_initialized = true;
    } else {
        ret = c2c_function_hists__reinit(&raw mut c2c_ext.function_hists, null(), c"symbol_view".as_ptr(), null_mut());
    }
    if ret != 0 {
        c2c_function__reset();
        symbol_conf.use_callchain = saved_use_callchain;
        return ret;
    }

    nd_cl = rb_first_cached(&mut (*c2c_ext.cl_hists).hists.entries);

    /* An empty C2C report yields an empty (but valid) function view. */
    while !nd_cl.is_null() {
        let he_cl = rb_entry_hist_entry(nd_cl);
        let cacheline_he = container_of_c2c_hist_entry_from_he(he_cl);
        let synthetic_thread = (*he_cl).thread;

        /*
         * Detail hists are finalized only for cachelines retained by the
         * top-level C2C filter. Among those, include any line with sharing
         * activity, not just HITM.
         */
        if (*he_cl).filtered ||
            (c2c_hitm_count(&(*cacheline_he).stats) == 0 &&
             (*cacheline_he).stats.tot_peer == 0 &&
             (*cacheline_he).stats.store == 0 &&
             (*cacheline_he).stats.load == 0) ||
            (*cacheline_he).hists.is_null() ||
            RB_EMPTY_ROOT(&(*(*cacheline_he).hists).hists.entries.rb_root) ||
            (*he_cl).mem_info.is_null() || synthetic_thread.is_null() {
            nd_cl = rb_next(nd_cl);
            continue;
        }

        ret = c2c_function__process_cl(cacheline_he, synthetic_thread);
        if ret != 0 {
            hists__collapse_resort(&raw mut c2c_ext.function_hists.hists, null_mut());
            hists__output_resort(&raw mut c2c_ext.function_hists.hists, null_mut());
            c2c_function__reset();
            symbol_conf.use_callchain = saved_use_callchain;
            return ret;
        }
        nd_cl = rb_next(nd_cl);
    }

    ret = c2c_function_hists__reinit(&raw mut c2c_ext.function_hists, output_fields.as_ptr() as *const c_char, c"cycles_percent".as_ptr(), null_mut());
    if ret != 0 {
        hists__collapse_resort(&raw mut c2c_ext.function_hists.hists, null_mut());
        hists__output_resort(&raw mut c2c_ext.function_hists.hists, null_mut());
        c2c_function__reset();
        symbol_conf.use_callchain = saved_use_callchain;
        return ret;
    }

    hists__collapse_resort(&raw mut c2c_ext.function_hists.hists, null_mut());
    hists__output_resort(&raw mut c2c_ext.function_hists.hists, null_mut());

    c2c_function__finalize();

    *hists_out = &raw mut c2c_ext.function_hists.hists;
    symbol_conf.use_callchain = saved_use_callchain;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
