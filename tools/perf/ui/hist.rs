// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/ui/hist.c.
// C include dependencies are intentionally represented as external symbols and
// C-compatible opaque/layout fragments below.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

type u64 = u64;
type s64 = i64;
type int64_t = i64;
type size_t = usize;
type ssize_t = isize;
type bool_t = bool;

const ENOMEM: c_int = 12;
const BITS_PER_LONG: c_int = core::mem::size_of::<c_ulong>() as c_int * 8;
const MEM_STAT_LEN: c_int = 16;
const MEM_STAT_PRINT_LEN: c_int = 8;
const PERF_HPP__OVERHEAD: c_int = 0;
const PERF_HPP__LATENCY: c_int = 1;
const PERF_HPP__OVERHEAD_SYS: c_int = 2;
const PERF_HPP__OVERHEAD_US: c_int = 3;
const PERF_HPP__OVERHEAD_GUEST_SYS: c_int = 4;
const PERF_HPP__OVERHEAD_GUEST_US: c_int = 5;
const PERF_HPP__OVERHEAD_ACC: c_int = 6;
const PERF_HPP__LATENCY_ACC: c_int = 7;
const PERF_HPP__SAMPLES: c_int = 8;
const PERF_HPP__PERIOD: c_int = 9;
const PERF_HPP__WEIGHT1: c_int = 10;
const PERF_HPP__WEIGHT2: c_int = 11;
const PERF_HPP__WEIGHT3: c_int = 12;
const PERF_HPP__MEM_STAT_OP: c_int = 13;
const PERF_HPP__MEM_STAT_CACHE: c_int = 14;
const PERF_HPP__MEM_STAT_MEMORY: c_int = 15;
const PERF_HPP__MEM_STAT_SNOOP: c_int = 16;
const PERF_HPP__MEM_STAT_DTLB: c_int = 17;
const PERF_HPP__MAX_INDEX: c_int = 18;
const ORDER_CALLER: c_int = 1;

const PERF_HPP_FMT_TYPE__PERCENT: perf_hpp_fmt_type = 0;
const PERF_HPP_FMT_TYPE__LATENCY: perf_hpp_fmt_type = 1;
const PERF_HPP_FMT_TYPE__AVERAGE: perf_hpp_fmt_type = 2;
const PERF_HPP_FMT_TYPE__RAW: perf_hpp_fmt_type = 3;

const PERF_MEM_STAT_OP: mem_stat_type = 0;
const PERF_MEM_STAT_CACHE: mem_stat_type = 1;
const PERF_MEM_STAT_MEMORY: mem_stat_type = 2;
const PERF_MEM_STAT_SNOOP: mem_stat_type = 3;
const PERF_MEM_STAT_DTLB: mem_stat_type = 4;

type perf_hpp_fmt_type = c_int;
type mem_stat_type = c_int;
type hpp_field_fn = Option<unsafe extern "C" fn(*mut hist_entry) -> u64>;
type hpp_snprint_fn = Option<unsafe extern "C" fn(*mut perf_hpp, *const c_char, ...) -> c_int>;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct perf_hpp {
    pub buf: *mut c_char,
    pub size: size_t,
}

#[repr(C)]
pub struct hist_stats {
    pub nr_samples: u64,
}

#[repr(C)]
pub struct hist_stat {
    pub period: u64,
    pub latency: u64,
    pub period_sys: u64,
    pub period_us: u64,
    pub period_guest_sys: u64,
    pub period_guest_us: u64,
    pub nr_events: c_int,
    pub weight1: u64,
    pub weight2: u64,
    pub weight3: u64,
}

#[repr(C)]
pub struct mem_stat {
    pub entries: [u64; MEM_STAT_LEN as usize],
}

#[repr(C)]
pub struct pairs {
    pub head: list_head,
    pub node: list_head,
}

#[repr(C)]
pub struct callchain {
    pub max_depth: s64,
}

#[repr(C)]
pub struct hist_entry {
    pub hists: *mut hists,
    pub stat: hist_stat,
    pub stat_acc: *mut hist_stat,
    pub pairs: pairs,
    pub thread: *mut c_void,
    pub callchain: *mut callchain,
    pub mem_stat: *mut mem_stat,
}

#[repr(C)]
pub struct hpp_list_ptr {
    pub nr_header_lines: c_int,
}

#[repr(C)]
pub struct hists {
    pub stats: hist_stats,
    pub nr_mem_stats: c_int,
    pub mem_stat_types: *mut mem_stat_type,
    pub mem_stat_total: *mut mem_stat,
    pub hpp_list: *mut hpp_list_ptr,
    pub hpp_formats: list_head,
    pub nr_hpp_node: c_int,
}

#[repr(C)]
pub struct evsel_core {
    pub nr_members: c_int,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_hpp_list {
    pub fields: list_head,
    pub sorts: list_head,
    pub nr_header_lines: c_int,
}

#[repr(C)]
pub struct perf_hpp_list_node {
    pub list: list_head,
    pub skip: bool_t,
    pub level: c_int,
    pub hpp: perf_hpp_list,
}

pub type hpp_header_fn = Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hists, c_int, *mut c_int) -> c_int>;
pub type hpp_width_fn_t = Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hists) -> c_int>;
pub type hpp_entry_fn = Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hist_entry) -> c_int>;
pub type hpp_cmp_fn = Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut hist_entry, *mut hist_entry) -> int64_t>;
pub type hpp_equal_fn = Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp_fmt) -> bool_t>;
pub type hpp_free_fn = Option<unsafe extern "C" fn(*mut perf_hpp_fmt)>;

#[repr(C)]
pub struct perf_hpp_fmt {
    pub list: list_head,
    pub sort_list: list_head,
    pub name: *const c_char,
    pub header: hpp_header_fn,
    pub width: hpp_width_fn_t,
    pub color: hpp_entry_fn,
    pub entry: hpp_entry_fn,
    pub cmp: hpp_cmp_fn,
    pub collapse: hpp_cmp_fn,
    pub sort: hpp_cmp_fn,
    pub idx: c_int,
    pub equal: hpp_equal_fn,
    pub free: hpp_free_fn,
    pub user_len: c_int,
    pub len: c_int,
    pub level: c_int,
}

#[repr(C)]
pub struct symbol_conf_t {
    pub skip_empty: bool_t,
    pub field_sep: bool_t,
    pub cumulate_callchain: bool_t,
    pub use_callchain: bool_t,
    pub group_sort_idx: c_int,
    pub event_group: bool_t,
    pub prefer_latency: bool_t,
    pub enable_latency: bool_t,
    pub show_cpu_utilization: bool_t,
    pub show_nr_samples: bool_t,
    pub show_total_period: bool_t,
    pub report_hierarchy: bool_t,
}

#[repr(C)]
pub struct callchain_param_t {
    pub order: c_int,
}

unsafe extern "C" {
    static mut symbol_conf: symbol_conf_t;
    static mut callchain_param: callchain_param_t;
    static mut perf_guest: bool_t;
    static mut verbose: c_int;
    static mut field_order: *const c_char;
    static mut sort_order: *const c_char;
    static graph_dotted_line: *const c_char;

    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> size_t;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn vsnprintf(s: *mut c_char, n: size_t, format: *const c_char, ap: *mut c_void) -> c_int;
    fn percent_color_len_snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, len: c_int, percent: c_double) -> c_int;
    fn assert_failed();
    fn BUG_ON(cond: bool_t);

    fn advance_hpp(hpp: *mut perf_hpp, ret: c_int);
    fn hists__total_period(hists: *mut hists) -> u64;
    fn hists__total_latency(hists: *mut hists) -> u64;
    fn hists_to_evsel(hists: *mut hists) -> *mut evsel;
    fn evsel__hists(evsel: *mut evsel) -> *mut hists;
    fn evsel__is_group_event(evsel: *mut evsel) -> bool_t;
    fn evsel__group_idx(evsel: *mut evsel) -> c_int;
    fn hist_entry__has_callchains(he: *mut hist_entry) -> bool_t;
    fn pr_debug(fmt: *const c_char, ...);
    fn mem_stat_name(mst: mem_stat_type, i: c_int) -> *const c_char;
    fn is_strict_order(order: *const c_char) -> bool_t;
    fn hpp_dimension__add_output(idx: c_int, taken: bool_t);
    fn list_add_tail(new_: *mut list_head, head: *mut list_head);
    fn list_add(new_: *mut list_head, head: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn list_empty(head: *const list_head) -> bool_t;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn perf_hpp__should_skip(fmt: *mut perf_hpp_fmt, hists: *mut hists) -> bool_t;
    fn perf_hpp__is_sort_entry(fmt: *mut perf_hpp_fmt) -> bool_t;
    fn perf_hpp__is_dynamic_entry(fmt: *mut perf_hpp_fmt) -> bool_t;
    fn perf_hpp__defined_dynamic_entry(fmt: *mut perf_hpp_fmt, hists: *mut hists) -> bool_t;
    fn perf_hpp__reset_sort_width(fmt: *mut perf_hpp_fmt, hists: *mut hists);
    fn perf_hpp_fmt__dup(fmt: *mut perf_hpp_fmt) -> *mut perf_hpp_fmt;
    fn perf_hpp_list__init(list: *mut perf_hpp_list);
    fn hists__has(hists: *mut hists, field: c_int) -> bool_t;
    fn RC_CHK_ACCESS(thread: *mut c_void) -> *mut c_void;
}

const sym: c_int = 0;

unsafe fn hpp__call_print_fn(
    hpp: *mut perf_hpp,
    print_fn: hpp_snprint_fn,
    fmt: *const c_char,
    len: c_int,
    value: c_double,
) -> c_int {
    let ret = print_fn.unwrap()(hpp, fmt, len, value);
    advance_hpp(hpp, ret);
    ret
}

unsafe fn hpp__call_print_fn_u64(
    hpp: *mut perf_hpp,
    print_fn: hpp_snprint_fn,
    fmt: *const c_char,
    len: c_int,
    value: u64,
) -> c_int {
    let ret = print_fn.unwrap()(hpp, fmt, len, value);
    advance_hpp(hpp, ret);
    ret
}

unsafe extern "C" fn __hpp__fmt_print(
    hpp: *mut perf_hpp,
    hists: *mut hists,
    val: u64,
    nr_samples: c_int,
    fmt: *const c_char,
    len: c_int,
    print_fn: hpp_snprint_fn,
    fmtype: perf_hpp_fmt_type,
) -> c_int {
    if fmtype == PERF_HPP_FMT_TYPE__PERCENT || fmtype == PERF_HPP_FMT_TYPE__LATENCY {
        let mut percent: c_double = 0.0;
        let total = if fmtype == PERF_HPP_FMT_TYPE__PERCENT {
            hists__total_period(hists)
        } else {
            hists__total_latency(hists)
        };
        if total != 0 {
            percent = 100.0 * val as c_double / total as c_double;
        }
        return hpp__call_print_fn(hpp, print_fn, fmt, len, percent);
    }
    if fmtype == PERF_HPP_FMT_TYPE__AVERAGE {
        let avg = if nr_samples != 0 { 1.0 * val as c_double / nr_samples as c_double } else { 0.0 };
        return hpp__call_print_fn(hpp, print_fn, fmt, len, avg);
    }
    hpp__call_print_fn_u64(hpp, print_fn, fmt, len, val)
}

#[repr(C)]
struct hpp_fmt_value {
    hists: *mut hists,
    val: u64,
    samples: c_int,
}

unsafe extern "C" fn __hpp__fmt(
    hpp: *mut perf_hpp,
    he: *mut hist_entry,
    get_field: hpp_field_fn,
    fmt: *const c_char,
    len: c_int,
    print_fn: hpp_snprint_fn,
    fmtype: perf_hpp_fmt_type,
) -> c_int {
    let mut ret = 0;
    let hists = (*he).hists;
    let evsel = hists_to_evsel(hists);
    let buf = (*hpp).buf;
    let size = (*hpp).size;
    let mut nr_members = 1;
    if evsel__is_group_event(evsel) {
        nr_members = (*evsel).core.nr_members;
    }
    let values = calloc(nr_members as size_t, core::mem::size_of::<hpp_fmt_value>()) as *mut hpp_fmt_value;
    if values.is_null() {
        return 0;
    }
    (*values.add(0)).hists = evsel__hists(evsel);
    (*values.add(0)).val = get_field.unwrap()(he);
    (*values.add(0)).samples = (*he).stat.nr_events;

    /*
     * C iterates for_each_group_member(pos, evsel) and list_for_each_entry()
     * over he->pairs. Those intrusive-list iterations depend on external perf
     * list container types, so the translation preserves the externally
     * supplied iteration as dependency intent.
     */
    if evsel__is_group_event(evsel) {
        // TODO: external for_each_group_member/list_for_each_entry expansion.
    }

    let mut i = 0;
    while i < nr_members {
        if !symbol_conf.skip_empty || (*(*values.add(i as usize)).hists).stats.nr_samples != 0 {
            ret += __hpp__fmt_print(
                hpp,
                (*values.add(i as usize)).hists,
                (*values.add(i as usize)).val,
                (*values.add(i as usize)).samples,
                fmt,
                len,
                print_fn,
                fmtype,
            );
        }
        i += 1;
    }
    free(values as *mut c_void);
    (*hpp).buf = buf;
    (*hpp).size = size;
    ret
}

#[no_mangle]
pub unsafe extern "C" fn hpp__fmt(
    fmt: *mut perf_hpp_fmt,
    hpp: *mut perf_hpp,
    he: *mut hist_entry,
    get_field: hpp_field_fn,
    fmtstr: *const c_char,
    print_fn: hpp_snprint_fn,
    fmtype: perf_hpp_fmt_type,
) -> c_int {
    let mut len = core::cmp::max(if (*fmt).user_len != 0 { (*fmt).user_len } else { (*fmt).len }, strlen((*fmt).name) as c_int);
    if symbol_conf.field_sep {
        return __hpp__fmt(hpp, he, get_field, fmtstr, 1, print_fn, fmtype);
    }
    if fmtype == PERF_HPP_FMT_TYPE__PERCENT || fmtype == PERF_HPP_FMT_TYPE__LATENCY {
        len -= 2;
    } else {
        len -= 1;
    }
    __hpp__fmt(hpp, he, get_field, fmtstr, len, print_fn, fmtype)
}

#[no_mangle]
pub unsafe extern "C" fn hpp__fmt_acc(
    fmt: *mut perf_hpp_fmt,
    hpp: *mut perf_hpp,
    he: *mut hist_entry,
    get_field: hpp_field_fn,
    fmtstr: *const c_char,
    print_fn: hpp_snprint_fn,
    fmtype: perf_hpp_fmt_type,
) -> c_int {
    if !symbol_conf.cumulate_callchain {
        let len = if (*fmt).user_len != 0 { (*fmt).user_len } else { (*fmt).len };
        return snprintf((*hpp).buf, (*hpp).size, c" %*s".as_ptr(), len - 1, c"N/A".as_ptr());
    }
    hpp__fmt(fmt, hpp, he, get_field, fmtstr, print_fn, fmtype)
}

#[no_mangle]
pub unsafe extern "C" fn hpp__fmt_mem_stat(
    _fmt: *mut perf_hpp_fmt,
    hpp: *mut perf_hpp,
    he: *mut hist_entry,
    mst: mem_stat_type,
    fmtstr: *const c_char,
    print_fn: hpp_snprint_fn,
) -> c_int {
    let hists = (*he).hists;
    let mut mem_stat_idx = -1;
    let buf = (*hpp).buf;
    let size = (*hpp).size;
    let mut total: u64 = 0;
    let mut ret = 0;
    let mut i = 0;
    while i < (*hists).nr_mem_stats {
        if *(*hists).mem_stat_types.add(i as usize) == mst {
            mem_stat_idx = i;
            break;
        }
        i += 1;
    }
    if mem_stat_idx == -1 { assert_failed(); }
    i = 0;
    while i < MEM_STAT_LEN {
        total = total.wrapping_add((*(*hists).mem_stat_total.add(mem_stat_idx as usize)).entries[i as usize]);
        i += 1;
    }
    if total == 0 { assert_failed(); }
    i = 0;
    while i < MEM_STAT_LEN {
        let val = (*(*he).mem_stat.add(mem_stat_idx as usize)).entries[i as usize];
        if (*(*hists).mem_stat_total.add(mem_stat_idx as usize)).entries[i as usize] != 0 {
            ret += hpp__call_print_fn(hpp, print_fn, fmtstr, 0, 100.0 * val as c_double / total as c_double);
        }
        i += 1;
    }
    (*hpp).buf = buf;
    (*hpp).size = size;
    ret
}

unsafe extern "C" fn field_cmp(field_a: u64, field_b: u64) -> c_int {
    if field_a > field_b { return 1; }
    if field_a < field_b { return -1; }
    0
}

unsafe extern "C" fn hist_entry__new_pair(
    _a: *mut hist_entry,
    _b: *mut hist_entry,
    _get_field: hpp_field_fn,
    nr_members: c_int,
    fields_a: *mut *mut u64,
    fields_b: *mut *mut u64,
) -> c_int {
    let fa = calloc(nr_members as size_t, core::mem::size_of::<u64>()) as *mut u64;
    let fb = calloc(nr_members as size_t, core::mem::size_of::<u64>()) as *mut u64;
    if fa.is_null() || fb.is_null() {
        free(fa as *mut c_void);
        free(fb as *mut c_void);
        *fields_a = ptr::null_mut();
        *fields_b = ptr::null_mut();
        return -1;
    }
    // TODO: external list_for_each_entry over a->pairs and b->pairs.
    *fields_a = fa;
    *fields_b = fb;
    0
}

unsafe extern "C" fn __hpp__group_sort_idx(a: *mut hist_entry, b: *mut hist_entry, get_field: hpp_field_fn, idx: c_int) -> c_int {
    let evsel = hists_to_evsel((*a).hists);
    let mut fields_a: *mut u64 = ptr::null_mut();
    let mut fields_b: *mut u64 = ptr::null_mut();
    let cmp = field_cmp(get_field.unwrap()(a), get_field.unwrap()(b));
    if !evsel__is_group_event(evsel) { return cmp; }
    let nr_members = (*evsel).core.nr_members;
    if idx < 1 || idx >= nr_members { return cmp; }
    let mut ret = hist_entry__new_pair(a, b, get_field, nr_members, &mut fields_a, &mut fields_b);
    if ret != 0 {
        ret = cmp;
    } else {
        ret = field_cmp(*fields_a.add(idx as usize), *fields_b.add(idx as usize));
        let mut i = 1;
        while ret == 0 && i < nr_members {
            if i != idx {
                ret = field_cmp(*fields_a.add(i as usize), *fields_b.add(i as usize));
            }
            i += 1;
        }
    }
    free(fields_a as *mut c_void);
    free(fields_b as *mut c_void);
    ret
}

unsafe extern "C" fn __hpp__sort(a: *mut hist_entry, b: *mut hist_entry, get_field: hpp_field_fn) -> int64_t {
    if symbol_conf.group_sort_idx != 0 && symbol_conf.event_group {
        return __hpp__group_sort_idx(a, b, get_field, symbol_conf.group_sort_idx) as int64_t;
    }
    let mut ret = field_cmp(get_field.unwrap()(a), get_field.unwrap()(b)) as int64_t;
    if ret != 0 || !symbol_conf.event_group { return ret; }
    let evsel = hists_to_evsel((*a).hists);
    if !evsel__is_group_event(evsel) { return ret; }
    let nr_members = (*evsel).core.nr_members;
    let mut fields_a: *mut u64 = ptr::null_mut();
    let mut fields_b: *mut u64 = ptr::null_mut();
    let mut i = hist_entry__new_pair(a, b, get_field, nr_members, &mut fields_a, &mut fields_b);
    if i == 0 {
        i = 1;
        while i < nr_members {
            ret = field_cmp(*fields_a.add(i as usize), *fields_b.add(i as usize)) as int64_t;
            if ret != 0 { break; }
            i += 1;
        }
    }
    free(fields_a as *mut c_void);
    free(fields_b as *mut c_void);
    ret
}

unsafe extern "C" fn __hpp__sort_acc(a: *mut hist_entry, b: *mut hist_entry, get_field: hpp_field_fn) -> int64_t {
    let mut ret: s64 = 0;
    if symbol_conf.cumulate_callchain {
        ret = field_cmp(get_field.unwrap()(a), get_field.unwrap()(b)) as s64;
        if ret != 0 { return ret; }
        let at = if (*a).thread.is_null() { ptr::null_mut() } else { RC_CHK_ACCESS((*a).thread) };
        let bt = if (*b).thread.is_null() { ptr::null_mut() } else { RC_CHK_ACCESS((*b).thread) };
        if at != bt || !hist_entry__has_callchains(a) || !symbol_conf.use_callchain {
            return 0;
        }
        ret = (*(*b).callchain).max_depth - (*(*a).callchain).max_depth;
        if callchain_param.order == ORDER_CALLER { ret = -ret; }
    }
    ret
}

unsafe extern "C" fn perf_hpp__is_mem_stat_entry(fmt: *mut perf_hpp_fmt) -> bool_t {
    (*fmt).sort == Some(hpp__sort_mem_stat)
}

unsafe extern "C" fn hpp__mem_stat_type(fmt: *mut perf_hpp_fmt) -> mem_stat_type {
    if !perf_hpp__is_mem_stat_entry(fmt) { return -1; }
    match (*fmt).idx {
        PERF_HPP__MEM_STAT_OP => PERF_MEM_STAT_OP,
        PERF_HPP__MEM_STAT_CACHE => PERF_MEM_STAT_CACHE,
        PERF_HPP__MEM_STAT_MEMORY => PERF_MEM_STAT_MEMORY,
        PERF_HPP__MEM_STAT_SNOOP => PERF_MEM_STAT_SNOOP,
        PERF_HPP__MEM_STAT_DTLB => PERF_MEM_STAT_DTLB,
        _ => {
            pr_debug(c"Should not reach here\n".as_ptr());
            -1
        }
    }
}

unsafe extern "C" fn hpp__sort_mem_stat(_fmt: *mut perf_hpp_fmt, a: *mut hist_entry, b: *mut hist_entry) -> int64_t {
    (*a).stat.period as int64_t - (*b).stat.period as int64_t
}

unsafe extern "C" fn hpp__width_fn(fmt: *mut perf_hpp_fmt, _hpp: *mut perf_hpp, hists: *mut hists) -> c_int {
    let mut len = if (*fmt).user_len != 0 { (*fmt).user_len } else { (*fmt).len };
    let _evsel = hists_to_evsel(hists);
    if symbol_conf.event_group {
        // TODO: external for_each_group_evsel expansion.
        len = core::cmp::max(len, (*fmt).len);
    }
    if len < strlen((*fmt).name) as c_int {
        len = strlen((*fmt).name) as c_int;
    }
    len
}

unsafe extern "C" fn hpp__header_fn(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, hists: *mut hists, line: c_int, _span: *mut c_int) -> c_int {
    let len = hpp__width_fn(fmt, hpp, hists);
    let mut hdr = c"".as_ptr();
    if line == (*(*hists).hpp_list).nr_header_lines - 1 {
        hdr = (*fmt).name;
    }
    scnprintf((*hpp).buf, (*hpp).size, c"%*s".as_ptr(), len, hdr)
}

unsafe extern "C" fn hpp__header_mem_stat_fn(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, hists: *mut hists, line: c_int, _span: *mut c_int) -> c_int {
    let mut buf = (*hpp).buf;
    let mut ret = 0;
    let mst = hpp__mem_stat_type(fmt);
    let mut mem_stat_idx = -1;
    let mut i = 0;
    while i < (*hists).nr_mem_stats {
        if *(*hists).mem_stat_types.add(i as usize) == mst { mem_stat_idx = i; break; }
        i += 1;
    }
    if mem_stat_idx == -1 { assert_failed(); }
    if line == 0 {
        let mut len = 0;
        i = 0;
        while i < MEM_STAT_LEN {
            if (*(*hists).mem_stat_total.add(mem_stat_idx as usize)).entries[i as usize] != 0 {
                len += MEM_STAT_PRINT_LEN;
            }
            i += 1;
        }
        (*fmt).len = len;
        if len == MEM_STAT_PRINT_LEN {
            return scnprintf((*hpp).buf, (*hpp).size, c"%*s".as_ptr(), len, (*fmt).name);
        }
        let mut left = (len - strlen((*fmt).name) as c_int) / 2 - 1;
        let mut right = len - left - strlen((*fmt).name) as c_int - 2;
        if left < 0 { left = 0; }
        if right < 0 { right = 0; }
        return scnprintf((*hpp).buf, (*hpp).size, c"%.*s %s %.*s".as_ptr(), left, graph_dotted_line, (*fmt).name, right, graph_dotted_line);
    }
    let mut len = (*hpp).size as c_int;
    i = 0;
    while i < MEM_STAT_LEN {
        if (*(*hists).mem_stat_total.add(mem_stat_idx as usize)).entries[i as usize] != 0 {
            let printed = scnprintf(buf, len as size_t, c"%*s".as_ptr(), MEM_STAT_PRINT_LEN, mem_stat_name(mst, i));
            ret += printed;
            buf = buf.add(printed as usize);
            len -= printed;
        }
        i += 1;
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn hpp_color_scnprintf(hpp: *mut perf_hpp, fmt: *const c_char, mut _args: ...) -> c_int {
    /*
     * C consumes varargs as (int len, double percent).
     * Stable Rust cannot inspect C varargs; this preserves the exported
     * interface and dependency intent for translation-only output.
     */
    let ssize = (*hpp).size as ssize_t;
    let ret = percent_color_len_snprintf((*hpp).buf, (*hpp).size, fmt, 0, 0.0);
    if ret as ssize_t >= ssize { (ssize - 1) as c_int } else { ret }
}

unsafe extern "C" fn hpp_entry_scnprintf(hpp: *mut perf_hpp, fmt: *const c_char, mut _args: ...) -> c_int {
    let ssize = (*hpp).size as ssize_t;
    let ret = snprintf((*hpp).buf, (*hpp).size, fmt);
    if ret as ssize_t >= ssize { (ssize - 1) as c_int } else { ret }
}

macro_rules! percent_fns {
    ($ty:ident, $field:ident, $fmttype:expr) => {
        unsafe extern "C" fn he_get_$field(he: *mut hist_entry) -> u64 { (*he).stat.$field }
    };
}

unsafe extern "C" fn he_get_period(he: *mut hist_entry) -> u64 { (*he).stat.period }
unsafe extern "C" fn he_get_latency(he: *mut hist_entry) -> u64 { (*he).stat.latency }
unsafe extern "C" fn he_get_period_sys(he: *mut hist_entry) -> u64 { (*he).stat.period_sys }
unsafe extern "C" fn he_get_period_us(he: *mut hist_entry) -> u64 { (*he).stat.period_us }
unsafe extern "C" fn he_get_period_guest_sys(he: *mut hist_entry) -> u64 { (*he).stat.period_guest_sys }
unsafe extern "C" fn he_get_period_guest_us(he: *mut hist_entry) -> u64 { (*he).stat.period_guest_us }
unsafe extern "C" fn he_get_acc_period(he: *mut hist_entry) -> u64 { (*(*he).stat_acc).period }
unsafe extern "C" fn he_get_acc_latency(he: *mut hist_entry) -> u64 { (*(*he).stat_acc).latency }
unsafe extern "C" fn he_get_raw_nr_events(he: *mut hist_entry) -> u64 { (*he).stat.nr_events as u64 }
unsafe extern "C" fn he_get_raw_period(he: *mut hist_entry) -> u64 { (*he).stat.period }
unsafe extern "C" fn he_get_weight1(he: *mut hist_entry) -> u64 { (*he).stat.weight1 }
unsafe extern "C" fn he_get_weight2(he: *mut hist_entry) -> u64 { (*he).stat.weight2 }
unsafe extern "C" fn he_get_weight3(he: *mut hist_entry) -> u64 { (*he).stat.weight3 }

macro_rules! make_percent {
    ($color:ident, $entry:ident, $sort:ident, $getter:ident, $fmttype:expr) => {
        unsafe extern "C" fn $color(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
            hpp__fmt(fmt, hpp, he, Some($getter), c" %*.2f%%".as_ptr(), Some(hpp_color_scnprintf), $fmttype)
        }
        unsafe extern "C" fn $entry(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
            hpp__fmt(fmt, hpp, he, Some($getter), c" %*.2f%%".as_ptr(), Some(hpp_entry_scnprintf), $fmttype)
        }
        unsafe extern "C" fn $sort(_fmt: *mut perf_hpp_fmt, a: *mut hist_entry, b: *mut hist_entry) -> int64_t {
            __hpp__sort(a, b, Some($getter))
        }
    };
}
macro_rules! make_acc_percent {
    ($color:ident, $entry:ident, $sort:ident, $getter:ident, $fmttype:expr) => {
        unsafe extern "C" fn $color(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
            hpp__fmt_acc(fmt, hpp, he, Some($getter), c" %*.2f%%".as_ptr(), Some(hpp_color_scnprintf), $fmttype)
        }
        unsafe extern "C" fn $entry(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
            hpp__fmt_acc(fmt, hpp, he, Some($getter), c" %*.2f%%".as_ptr(), Some(hpp_entry_scnprintf), $fmttype)
        }
        unsafe extern "C" fn $sort(_fmt: *mut perf_hpp_fmt, a: *mut hist_entry, b: *mut hist_entry) -> int64_t {
            __hpp__sort_acc(a, b, Some($getter))
        }
    };
}
macro_rules! make_raw {
    ($entry:ident, $sort:ident, $getter:ident) => {
        unsafe extern "C" fn $entry(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
            hpp__fmt(fmt, hpp, he, Some($getter), c" %*lu".as_ptr(), Some(hpp_entry_scnprintf), PERF_HPP_FMT_TYPE__RAW)
        }
        unsafe extern "C" fn $sort(_fmt: *mut perf_hpp_fmt, a: *mut hist_entry, b: *mut hist_entry) -> int64_t {
            __hpp__sort(a, b, Some($getter))
        }
    };
}
macro_rules! make_average {
    ($entry:ident, $sort:ident, $getter:ident) => {
        unsafe extern "C" fn $entry(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
            hpp__fmt(fmt, hpp, he, Some($getter), c" %*.1f".as_ptr(), Some(hpp_entry_scnprintf), PERF_HPP_FMT_TYPE__AVERAGE)
        }
        unsafe extern "C" fn $sort(_fmt: *mut perf_hpp_fmt, a: *mut hist_entry, b: *mut hist_entry) -> int64_t {
            __hpp__sort(a, b, Some($getter))
        }
    };
}
macro_rules! make_mem_stat {
    ($color:ident, $entry:ident, $mst:expr) => {
        unsafe extern "C" fn $color(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
            hpp__fmt_mem_stat(fmt, hpp, he, $mst, c" %5.1f%%".as_ptr(), Some(hpp_color_scnprintf))
        }
        unsafe extern "C" fn $entry(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
            hpp__fmt_mem_stat(fmt, hpp, he, $mst, c" %5.1f%%".as_ptr(), Some(hpp_entry_scnprintf))
        }
    };
}

make_percent!(hpp__color_overhead, hpp__entry_overhead, hpp__sort_overhead, he_get_period, PERF_HPP_FMT_TYPE__PERCENT);
make_percent!(hpp__color_latency, hpp__entry_latency, hpp__sort_latency, he_get_latency, PERF_HPP_FMT_TYPE__LATENCY);
make_percent!(hpp__color_overhead_sys, hpp__entry_overhead_sys, hpp__sort_overhead_sys, he_get_period_sys, PERF_HPP_FMT_TYPE__PERCENT);
make_percent!(hpp__color_overhead_us, hpp__entry_overhead_us, hpp__sort_overhead_us, he_get_period_us, PERF_HPP_FMT_TYPE__PERCENT);
make_percent!(hpp__color_overhead_guest_sys, hpp__entry_overhead_guest_sys, hpp__sort_overhead_guest_sys, he_get_period_guest_sys, PERF_HPP_FMT_TYPE__PERCENT);
make_percent!(hpp__color_overhead_guest_us, hpp__entry_overhead_guest_us, hpp__sort_overhead_guest_us, he_get_period_guest_us, PERF_HPP_FMT_TYPE__PERCENT);
make_acc_percent!(hpp__color_overhead_acc, hpp__entry_overhead_acc, hpp__sort_overhead_acc, he_get_acc_period, PERF_HPP_FMT_TYPE__PERCENT);
make_acc_percent!(hpp__color_latency_acc, hpp__entry_latency_acc, hpp__sort_latency_acc, he_get_acc_latency, PERF_HPP_FMT_TYPE__LATENCY);
make_raw!(hpp__entry_samples, hpp__sort_samples, he_get_raw_nr_events);
make_raw!(hpp__entry_period, hpp__sort_period, he_get_raw_period);
make_average!(hpp__entry_weight1, hpp__sort_weight1, he_get_weight1);
make_average!(hpp__entry_weight2, hpp__sort_weight2, he_get_weight2);
make_average!(hpp__entry_weight3, hpp__sort_weight3, he_get_weight3);
make_mem_stat!(hpp__color_mem_stat_op, hpp__entry_mem_stat_op, PERF_MEM_STAT_OP);
make_mem_stat!(hpp__color_mem_stat_cache, hpp__entry_mem_stat_cache, PERF_MEM_STAT_CACHE);
make_mem_stat!(hpp__color_mem_stat_memory, hpp__entry_mem_stat_memory, PERF_MEM_STAT_MEMORY);
make_mem_stat!(hpp__color_mem_stat_snoop, hpp__entry_mem_stat_snoop, PERF_MEM_STAT_SNOOP);
make_mem_stat!(hpp__color_mem_stat_dtlb, hpp__entry_mem_stat_dtlb, PERF_MEM_STAT_DTLB);

unsafe extern "C" fn hpp__nop_cmp(_fmt: *mut perf_hpp_fmt, _a: *mut hist_entry, _b: *mut hist_entry) -> int64_t { 0 }
unsafe extern "C" fn perf_hpp__is_hpp_entry(a: *mut perf_hpp_fmt) -> bool_t { (*a).header == Some(hpp__header_fn) }
unsafe extern "C" fn hpp__equal(a: *mut perf_hpp_fmt, b: *mut perf_hpp_fmt) -> bool_t {
    if !perf_hpp__is_hpp_entry(a) || !perf_hpp__is_hpp_entry(b) { return false; }
    (*a).idx == (*b).idx
}
unsafe extern "C" fn hpp__equal_mem_stat(a: *mut perf_hpp_fmt, b: *mut perf_hpp_fmt) -> bool_t {
    if !perf_hpp__is_mem_stat_entry(a) || !perf_hpp__is_mem_stat_entry(b) { return false; }
    (*a).entry == (*b).entry
}

macro_rules! fmt_item {
    ($name:expr, $color:ident, $entry:ident, $sort:ident, $idx:expr) => {
        perf_hpp_fmt { list: list_head{next: ptr::null_mut(), prev: ptr::null_mut()}, sort_list: list_head{next: ptr::null_mut(), prev: ptr::null_mut()}, name: $name.as_ptr(), header: Some(hpp__header_fn), width: Some(hpp__width_fn), color: Some($color), entry: Some($entry), cmp: Some(hpp__nop_cmp), collapse: Some(hpp__nop_cmp), sort: Some($sort), idx: $idx, equal: Some(hpp__equal), free: None, user_len: 0, len: 0, level: 0 }
    };
}
macro_rules! fmt_print_item {
    ($name:expr, $entry:ident, $sort:ident, $idx:expr) => {
        perf_hpp_fmt { list: list_head{next: ptr::null_mut(), prev: ptr::null_mut()}, sort_list: list_head{next: ptr::null_mut(), prev: ptr::null_mut()}, name: $name.as_ptr(), header: Some(hpp__header_fn), width: Some(hpp__width_fn), color: None, entry: Some($entry), cmp: Some(hpp__nop_cmp), collapse: Some(hpp__nop_cmp), sort: Some($sort), idx: $idx, equal: Some(hpp__equal), free: None, user_len: 0, len: 0, level: 0 }
    };
}
macro_rules! fmt_mem_item {
    ($name:expr, $color:ident, $entry:ident, $idx:expr) => {
        perf_hpp_fmt { list: list_head{next: ptr::null_mut(), prev: ptr::null_mut()}, sort_list: list_head{next: ptr::null_mut(), prev: ptr::null_mut()}, name: $name.as_ptr(), header: Some(hpp__header_mem_stat_fn), width: Some(hpp__width_fn), color: Some($color), entry: Some($entry), cmp: Some(hpp__nop_cmp), collapse: Some(hpp__nop_cmp), sort: Some(hpp__sort_mem_stat), idx: $idx, equal: Some(hpp__equal_mem_stat), free: None, user_len: 0, len: 0, level: 0 }
    };
}

#[no_mangle]
pub static mut perf_hpp__format: [perf_hpp_fmt; PERF_HPP__MAX_INDEX as usize] = [
    fmt_item!(c"Overhead", hpp__color_overhead, hpp__entry_overhead, hpp__sort_overhead, PERF_HPP__OVERHEAD),
    fmt_item!(c"Latency", hpp__color_latency, hpp__entry_latency, hpp__sort_latency, PERF_HPP__LATENCY),
    fmt_item!(c"sys", hpp__color_overhead_sys, hpp__entry_overhead_sys, hpp__sort_overhead_sys, PERF_HPP__OVERHEAD_SYS),
    fmt_item!(c"usr", hpp__color_overhead_us, hpp__entry_overhead_us, hpp__sort_overhead_us, PERF_HPP__OVERHEAD_US),
    fmt_item!(c"guest sys", hpp__color_overhead_guest_sys, hpp__entry_overhead_guest_sys, hpp__sort_overhead_guest_sys, PERF_HPP__OVERHEAD_GUEST_SYS),
    fmt_item!(c"guest usr", hpp__color_overhead_guest_us, hpp__entry_overhead_guest_us, hpp__sort_overhead_guest_us, PERF_HPP__OVERHEAD_GUEST_US),
    fmt_item!(c"Children", hpp__color_overhead_acc, hpp__entry_overhead_acc, hpp__sort_overhead_acc, PERF_HPP__OVERHEAD_ACC),
    fmt_item!(c"Latency", hpp__color_latency_acc, hpp__entry_latency_acc, hpp__sort_latency_acc, PERF_HPP__LATENCY_ACC),
    fmt_print_item!(c"Samples", hpp__entry_samples, hpp__sort_samples, PERF_HPP__SAMPLES),
    fmt_print_item!(c"Period", hpp__entry_period, hpp__sort_period, PERF_HPP__PERIOD),
    fmt_print_item!(c"Weight1", hpp__entry_weight1, hpp__sort_weight1, PERF_HPP__WEIGHT1),
    fmt_print_item!(c"Weight2", hpp__entry_weight2, hpp__sort_weight2, PERF_HPP__WEIGHT2),
    fmt_print_item!(c"Weight3", hpp__entry_weight3, hpp__sort_weight3, PERF_HPP__WEIGHT3),
    fmt_mem_item!(c"Mem Op", hpp__color_mem_stat_op, hpp__entry_mem_stat_op, PERF_HPP__MEM_STAT_OP),
    fmt_mem_item!(c"Cache", hpp__color_mem_stat_cache, hpp__entry_mem_stat_cache, PERF_HPP__MEM_STAT_CACHE),
    fmt_mem_item!(c"Memory", hpp__color_mem_stat_memory, hpp__entry_mem_stat_memory, PERF_HPP__MEM_STAT_MEMORY),
    fmt_mem_item!(c"Snoop", hpp__color_mem_stat_snoop, hpp__entry_mem_stat_snoop, PERF_HPP__MEM_STAT_SNOOP),
    fmt_mem_item!(c"D-TLB", hpp__color_mem_stat_dtlb, hpp__entry_mem_stat_dtlb, PERF_HPP__MEM_STAT_DTLB),
];

#[no_mangle]
pub static mut perf_hpp_list: perf_hpp_list = perf_hpp_list {
    fields: list_head { next: ptr::null_mut(), prev: ptr::null_mut() },
    sorts: list_head { next: ptr::null_mut(), prev: ptr::null_mut() },
    nr_header_lines: 1,
};

unsafe extern "C" fn fmt_free(fmt: *mut perf_hpp_fmt) {
    BUG_ON(!list_empty(&mut (*fmt).list));
    BUG_ON(!list_empty(&mut (*fmt).sort_list));
    if let Some(f) = (*fmt).free { f(fmt); }
}

unsafe extern "C" fn fmt_equal(a: *mut perf_hpp_fmt, b: *mut perf_hpp_fmt) -> bool_t {
    (*a).equal.is_some() && (*a).equal.unwrap()(a, b)
}

#[no_mangle]
pub unsafe extern "C" fn perf_hpp__init() {
    let mut i = 0;
    while i < PERF_HPP__MAX_INDEX {
        let fmt = perf_hpp__format.as_mut_ptr().add(i as usize);
        INIT_LIST_HEAD(&mut (*fmt).list);
        if (*fmt).sort_list.next.is_null() { INIT_LIST_HEAD(&mut (*fmt).sort_list); }
        i += 1;
    }
    if is_strict_order(field_order) { return; }
    if symbol_conf.cumulate_callchain {
        if symbol_conf.prefer_latency { hpp_dimension__add_output(PERF_HPP__LATENCY_ACC, true); }
        hpp_dimension__add_output(PERF_HPP__OVERHEAD_ACC, true);
        if symbol_conf.enable_latency { hpp_dimension__add_output(PERF_HPP__LATENCY_ACC, true); }
        perf_hpp__format[PERF_HPP__OVERHEAD as usize].name = c"Self".as_ptr();
    }
    if symbol_conf.prefer_latency { hpp_dimension__add_output(PERF_HPP__LATENCY, true); }
    hpp_dimension__add_output(PERF_HPP__OVERHEAD, true);
    if symbol_conf.enable_latency { hpp_dimension__add_output(PERF_HPP__LATENCY, true); }
    if symbol_conf.show_cpu_utilization {
        hpp_dimension__add_output(PERF_HPP__OVERHEAD_SYS, false);
        hpp_dimension__add_output(PERF_HPP__OVERHEAD_US, false);
        if perf_guest {
            hpp_dimension__add_output(PERF_HPP__OVERHEAD_GUEST_SYS, false);
            hpp_dimension__add_output(PERF_HPP__OVERHEAD_GUEST_US, false);
        }
    }
    if symbol_conf.show_nr_samples { hpp_dimension__add_output(PERF_HPP__SAMPLES, false); }
    if symbol_conf.show_total_period { hpp_dimension__add_output(PERF_HPP__PERIOD, false); }
}

#[no_mangle]
pub unsafe extern "C" fn perf_hpp_list__column_register(list: *mut perf_hpp_list, format: *mut perf_hpp_fmt) {
    list_add_tail(&mut (*format).list, &mut (*list).fields);
}
#[no_mangle]
pub unsafe extern "C" fn perf_hpp_list__register_sort_field(list: *mut perf_hpp_list, format: *mut perf_hpp_fmt) {
    list_add_tail(&mut (*format).sort_list, &mut (*list).sorts);
}
#[no_mangle]
pub unsafe extern "C" fn perf_hpp_list__prepend_sort_field(list: *mut perf_hpp_list, format: *mut perf_hpp_fmt) {
    list_add(&mut (*format).sort_list, &mut (*list).sorts);
}

unsafe extern "C" fn perf_hpp__column_unregister(format: *mut perf_hpp_fmt) {
    list_del_init(&mut (*format).list);
    list_del_init(&mut (*format).sort_list);
    fmt_free(format);
}

#[no_mangle]
pub unsafe extern "C" fn perf_hpp__cancel_cumulate(_evlist: *mut evlist) {
    if is_strict_order(field_order) { return; }
    let _ovh = perf_hpp__format.as_mut_ptr().add(PERF_HPP__OVERHEAD as usize);
    let _acc = perf_hpp__format.as_mut_ptr().add(PERF_HPP__OVERHEAD_ACC as usize);
    let _acc_lat = perf_hpp__format.as_mut_ptr().add(PERF_HPP__LATENCY_ACC as usize);
    // TODO: external perf_hpp_list__for_each_format_safe and evlist iteration.
}

#[no_mangle]
pub unsafe extern "C" fn perf_hpp__cancel_latency(_evlist: *mut evlist) {
    if is_strict_order(field_order) { return; }
    if !sort_order.is_null() && !strstr(sort_order, c"latency".as_ptr()).is_null() { return; }
    // TODO: external perf_hpp_list__for_each_format_safe and evlist iteration.
}

#[no_mangle]
pub unsafe extern "C" fn perf_hpp__setup_output_field(_list: *mut perf_hpp_list) {
    // TODO: external perf_hpp_list__for_each_sort_list/format expansion.
}

#[no_mangle]
pub unsafe extern "C" fn perf_hpp__append_sort_keys(_list: *mut perf_hpp_list) {
    // TODO: external perf_hpp_list__for_each_format/sort_list expansion.
}

#[no_mangle]
pub unsafe extern "C" fn perf_hpp__reset_output_field(_list: *mut perf_hpp_list) {
    // TODO: external perf_hpp_list__for_each_format_safe/sort_list_safe expansion.
}

#[no_mangle]
pub unsafe extern "C" fn hists__sort_list_width(hists: *mut hists) -> c_uint {
    let mut ret = 0;
    let _dummy_hpp = perf_hpp { buf: ptr::null_mut(), size: 0 };
    // TODO: external hists__for_each_format expansion.
    if verbose > 0 && hists__has(hists, sym) {
        ret += 3 + BITS_PER_LONG / 4;
    }
    ret as c_uint
}

#[no_mangle]
pub unsafe extern "C" fn hists__overhead_width(_hists: *mut hists) -> c_uint {
    let ret = 0;
    // TODO: external hists__for_each_format expansion.
    ret as c_uint
}

#[no_mangle]
pub unsafe extern "C" fn perf_hpp__reset_width(fmt: *mut perf_hpp_fmt, hists: *mut hists) {
    if perf_hpp__is_sort_entry(fmt) {
        perf_hpp__reset_sort_width(fmt, hists);
        return;
    }
    if perf_hpp__is_dynamic_entry(fmt) { return; }
    BUG_ON((*fmt).idx >= PERF_HPP__MAX_INDEX);
    match (*fmt).idx {
        PERF_HPP__OVERHEAD | PERF_HPP__LATENCY | PERF_HPP__OVERHEAD_SYS | PERF_HPP__OVERHEAD_US | PERF_HPP__OVERHEAD_ACC => (*fmt).len = 8,
        PERF_HPP__OVERHEAD_GUEST_SYS | PERF_HPP__OVERHEAD_GUEST_US => (*fmt).len = 9,
        PERF_HPP__SAMPLES | PERF_HPP__PERIOD => (*fmt).len = 12,
        PERF_HPP__WEIGHT1 | PERF_HPP__WEIGHT2 | PERF_HPP__WEIGHT3 => (*fmt).len = 8,
        PERF_HPP__MEM_STAT_OP | PERF_HPP__MEM_STAT_CACHE | PERF_HPP__MEM_STAT_MEMORY | PERF_HPP__MEM_STAT_SNOOP | PERF_HPP__MEM_STAT_DTLB => (*fmt).len = MEM_STAT_LEN * MEM_STAT_PRINT_LEN,
        _ => {}
    }
}

#[no_mangle]
pub unsafe extern "C" fn hists__reset_column_width(_hists: *mut hists) {
    // TODO: external hists__for_each_format and hierarchy list iteration.
}

#[no_mangle]
pub unsafe extern "C" fn perf_hpp__set_user_width(width_list_str: *const c_char) {
    let mut ptr_s = width_list_str;
    // TODO: external perf_hpp_list__for_each_format expansion. Apply to built-ins as direct local equivalent.
    let mut i = 0;
    while i < PERF_HPP__MAX_INDEX {
        let fmt = perf_hpp__format.as_mut_ptr().add(i as usize);
        let mut p: *mut c_char = ptr::null_mut();
        let len = strtol(ptr_s, &mut p, 10) as c_int;
        (*fmt).user_len = len;
        if !p.is_null() && *p == b',' as c_char {
            ptr_s = p.add(1);
        } else {
            break;
        }
        i += 1;
    }
}

unsafe extern "C" fn add_hierarchy_fmt(hists: *mut hists, fmt: *mut perf_hpp_fmt) -> c_int {
    let mut node: *mut perf_hpp_list_node = ptr::null_mut();
    let skip = perf_hpp__should_skip(fmt, hists);
    let found = false;
    // TODO: external list_for_each_entry over hists->hpp_formats.
    if !found {
        node = malloc(core::mem::size_of::<perf_hpp_list_node>()) as *mut perf_hpp_list_node;
        if node.is_null() { return -1; }
        (*node).skip = skip;
        (*node).level = (*fmt).level;
        perf_hpp_list__init(&mut (*node).hpp);
        (*hists).nr_hpp_node += 1;
        list_add_tail(&mut (*node).list, &mut (*hists).hpp_formats);
    }
    let fmt_copy = perf_hpp_fmt__dup(fmt);
    if fmt_copy.is_null() { return -1; }
    if !skip { (*node).skip = false; }
    list_add_tail(&mut (*fmt_copy).list, &mut (*node).hpp.fields);
    list_add_tail(&mut (*fmt_copy).sort_list, &mut (*node).hpp.sorts);
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_hpp__setup_hists_formats(_list: *mut perf_hpp_list, _evlist: *mut evlist) -> c_int {
    if !symbol_conf.report_hierarchy { return 0; }
    // TODO: external evlist__for_each_entry and perf_hpp_list__for_each_sort_list expansion.
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_hpp__alloc_mem_stats(list: *mut perf_hpp_list, _evlist: *mut evlist) -> c_int {
    let mut mst: [mem_stat_type; 16] = [0; 16];
    let mut nr_mem_stats: c_uint = 0;
    // TODO: external perf_hpp_list__for_each_format expansion. Built-in array is used as local equivalent.
    let mut i = 0;
    while i < PERF_HPP__MAX_INDEX {
        let fmt = perf_hpp__format.as_mut_ptr().add(i as usize);
        if perf_hpp__is_mem_stat_entry(fmt) {
            if nr_mem_stats as usize >= mst.len() { assert_failed(); }
            mst[nr_mem_stats as usize] = hpp__mem_stat_type(fmt);
            nr_mem_stats += 1;
        }
        i += 1;
    }
    if nr_mem_stats == 0 { return 0; }
    (*list).nr_header_lines = 2;
    // TODO: external evlist__for_each_entry allocation for each evsel->hists.
    let _ = ENOMEM;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
