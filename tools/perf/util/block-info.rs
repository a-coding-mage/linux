// SPDX-License-Identifier: GPL-2.0

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_double, c_float, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u64 = u64;
type int64_t = i64;

const NUM_SPARKS: usize = 32;
const PERF_HPP_REPORT__BLOCK_TOTAL_CYCLES_PCT: c_int = 0;
const PERF_HPP_REPORT__BLOCK_LBR_CYCLES: c_int = 1;
const PERF_HPP_REPORT__BLOCK_CYCLES_PCT: c_int = 2;
const PERF_HPP_REPORT__BLOCK_AVG_CYCLES: c_int = 3;
const PERF_HPP_REPORT__BLOCK_RANGE: c_int = 4;
const PERF_HPP_REPORT__BLOCK_DSO: c_int = 5;
const PERF_HPP_REPORT__BLOCK_BRANCH_COUNTER: c_int = 6;
const PERF_HPP_REPORT__BLOCK_MAX_INDEX: usize = 7;

const SRCLINE_UNKNOWN: *mut c_char = 0 as *mut c_char;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root_cached {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    pub name: *const c_char,
    pub start: u64,
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
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
pub struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
pub struct addr_location {
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct map_symbol {
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct cyc_hist {
    pub start: u64,
    pub cycles: u64,
    pub cycles_aggr: u64,
    pub num: u64,
    pub num_aggr: u64,
    pub cycles_spark: [u64; NUM_SPARKS],
}

#[repr(C)]
pub struct annotation_branch {
    pub cycles_hist: *mut cyc_hist,
    pub br_cntr: *mut u64,
}

#[repr(C)]
pub struct annotation {
    pub branch: *mut annotation_branch,
}

#[repr(C)]
pub struct hists {
    pub entries: rb_root_cached,
}

#[repr(C)]
pub struct perf_hpp {
    pub buf: *mut c_char,
    pub size: usize,
}

#[repr(C)]
pub struct perf_hpp_fmt {
    pub list: list_head,
    pub sort_list: list_head,
    pub header: Option<
        unsafe extern "C" fn(
            *mut perf_hpp_fmt,
            *mut perf_hpp,
            *mut hists,
            c_int,
            *mut c_int,
        ) -> c_int,
    >,
    pub width: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hists) -> c_int>,
    pub entry: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hist_entry) -> c_int>,
    pub color: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hist_entry) -> c_int>,
    pub cmp: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut hist_entry, *mut hist_entry) -> int64_t>,
    pub sort: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut hist_entry, *mut hist_entry) -> int64_t>,
}

#[repr(C)]
pub struct perf_hpp_list {
    pub nr_header_lines: c_int,
}

#[repr(C)]
pub struct block_info {
    pub sym: *mut symbol,
    pub start: u64,
    pub end: c_int,
    pub cycles: u64,
    pub cycles_aggr: u64,
    pub num: u64,
    pub num_aggr: u64,
    pub total_cycles: u64,
    pub cycles_spark: [u64; NUM_SPARKS],
    pub br_cntr_nr: c_uint,
    pub br_cntr: *mut u64,
    pub evsel: *mut evsel,
}

type c_uint = u32;

#[repr(C)]
pub struct block_fmt {
    pub fmt: perf_hpp_fmt,
    pub idx: c_int,
    pub header: *const c_char,
    pub width: c_int,
    pub total_cycles: u64,
    pub block_cycles: u64,
}

#[repr(C)]
pub struct block_hist {
    pub block_hists: hists,
    pub block_list: perf_hpp_list,
}

#[repr(C)]
pub struct block_report {
    pub hist: block_hist,
    pub fmts: [block_fmt; PERF_HPP_REPORT__BLOCK_MAX_INDEX],
    pub nr_fmts: c_int,
    pub cycles: u64,
}

#[repr(C)]
pub struct hist_entry {
    pub block_info: *mut block_info,
    pub ms: map_symbol,
    pub hists: *mut hists,
    pub rb_node: rb_node,
}

#[repr(C)]
pub struct symbol_conf_t {
    pub addr2line_disable_warn: bool,
    pub report_individual_block: bool,
}

#[repr(C)]
struct block_header_column {
    name: *const c_char,
    width: c_int,
}

const fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

static mut block_columns: [block_header_column; PERF_HPP_REPORT__BLOCK_MAX_INDEX] = [
    block_header_column {
        name: cstr(b"Sampled Cycles%\0"),
        width: 15,
    },
    block_header_column {
        name: cstr(b"Sampled Cycles\0"),
        width: 14,
    },
    block_header_column {
        name: cstr(b"Avg Cycles%\0"),
        width: 11,
    },
    block_header_column {
        name: cstr(b"Avg Cycles\0"),
        width: 10,
    },
    block_header_column {
        name: cstr(b"[Program Block Range]\0"),
        width: 70,
    },
    block_header_column {
        name: cstr(b"Shared Object\0"),
        width: 20,
    },
    block_header_column {
        name: cstr(b"Branch Counter\0"),
        width: 30,
    },
];

unsafe extern "C" {
    static mut symbol_conf: symbol_conf_t;
    static mut use_browser: c_int;
    static mut stdout: *mut c_void;

    fn zalloc(size: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;

    fn symbol__annotation(sym: *mut symbol) -> *mut annotation;
    fn symbol__size(sym: *mut symbol) -> usize;
    fn hists_to_evsel(hists: *mut hists) -> *mut evsel;
    fn hists__add_entry_block(
        hists: *mut hists,
        al: *mut addr_location,
        bi: *mut block_info,
    ) -> *mut hist_entry;
    fn hpp_color_scnprintf(hpp: *mut perf_hpp, fmt: *const c_char, ...) -> c_int;
    fn __hpp__slsmg_color_printf(hpp: *mut perf_hpp, fmt: *const c_char, ...) -> c_int;
    fn map__srcline(map: *mut map, addr: u64, sym: *mut symbol) -> *mut c_char;
    fn zfree_srcline(line: *mut *mut c_char);
    fn map__dso(map: *mut map) -> *mut dso;
    fn dso__short_name(dso: *mut dso) -> *const c_char;
    fn annotation_br_cntr_entry(
        buf: *mut *mut c_char,
        br_cntr_nr: c_uint,
        br_cntr: *mut u64,
        num_aggr: u64,
        evsel: *mut evsel,
    ) -> c_int;
    fn perf_hpp_list__column_register(hpp_list: *mut perf_hpp_list, fmt: *mut perf_hpp_fmt);
    fn __hists__init(hists: *mut hists, hpp_list: *mut perf_hpp_list);
    fn perf_hpp_list__init(hpp_list: *mut perf_hpp_list);
    fn perf_hpp_list__register_sort_field(hpp_list: *mut perf_hpp_list, fmt: *mut perf_hpp_fmt);
    fn rb_first_cached(root: *mut rb_root_cached) -> *mut rb_node;
    fn rb_next(node: *mut rb_node) -> *mut rb_node;
    fn hists__output_resort(hists: *mut hists, ignored: *mut c_void);
    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn evlist__nr_br_cntr(evlist: *mut evlist) -> c_uint;
    fn evsel__hists(evsel: *mut evsel) -> *mut hists;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__next(evlist: *mut evlist, pos: *mut evsel) -> *mut evsel;
    fn hists__delete_entries(hists: *mut hists);
    fn hists__fprintf(
        hists: *mut hists,
        show_header: bool,
        max_rows: c_int,
        max_cols: c_int,
        min_percent: c_float,
        fp: *mut c_void,
        use_callchain: bool,
    );
    fn block_hists_tui_browse(
        bh: *mut block_hist,
        evsel: *mut evsel,
        min_percent: c_float,
        env: *mut perf_env,
    ) -> c_int;
}

unsafe fn container_of_block_fmt(fmt: *mut perf_hpp_fmt) -> *mut block_fmt {
    fmt as *mut block_fmt
}

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

unsafe fn BUG_ON(cond: bool) {
    if cond {
        core::intrinsics::abort();
    }
}

unsafe fn rb_entry_hist_entry(node: *mut rb_node) -> *mut hist_entry {
    (node as *mut u8).sub(core::mem::offset_of!(hist_entry, rb_node)) as *mut hist_entry
}

unsafe fn block_info__new(br_cntr_nr: c_uint) -> *mut block_info {
    let bi = zalloc(size_of::<block_info>()) as *mut block_info;

    if !bi.is_null() && br_cntr_nr != 0 {
        (*bi).br_cntr = calloc(br_cntr_nr as usize, size_of::<u64>()) as *mut u64;
        if (*bi).br_cntr.is_null() {
            free(bi as *mut c_void);
            return ptr::null_mut();
        }
    }

    bi
}

#[no_mangle]
pub unsafe extern "C" fn block_info__delete(bi: *mut block_info) {
    if !bi.is_null() {
        free((*bi).br_cntr as *mut c_void);
    }
    free(bi as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn __block_info__cmp(left: *mut hist_entry, right: *mut hist_entry) -> int64_t {
    let bi_l = (*left).block_info;
    let bi_r = (*right).block_info;
    let cmp: c_int;

    if (*bi_l).sym.is_null() || (*bi_r).sym.is_null() {
        if (*bi_l).sym.is_null() && (*bi_r).sym.is_null() {
            return -1;
        } else if (*bi_l).sym.is_null() {
            return -1;
        } else {
            return 1;
        }
    }

    cmp = strcmp((*(*bi_l).sym).name, (*(*bi_r).sym).name);
    if cmp != 0 {
        return cmp as int64_t;
    }

    if (*bi_l).start != (*bi_r).start {
        return ((*bi_r).start.wrapping_sub((*bi_l).start)) as int64_t;
    }

    ((*bi_r).end - (*bi_l).end) as int64_t
}

#[no_mangle]
pub unsafe extern "C" fn block_info__cmp(
    _fmt: *mut perf_hpp_fmt,
    left: *mut hist_entry,
    right: *mut hist_entry,
) -> int64_t {
    __block_info__cmp(left, right)
}

unsafe fn init_block_info(
    bi: *mut block_info,
    sym: *mut symbol,
    ch: *mut cyc_hist,
    offset: c_int,
    total_cycles: u64,
    br_cntr_nr: c_uint,
    br_cntr: *mut u64,
    evsel: *mut evsel,
) {
    (*bi).sym = sym;
    (*bi).start = (*ch).start;
    (*bi).end = offset;
    (*bi).cycles = (*ch).cycles;
    (*bi).cycles_aggr = (*ch).cycles_aggr;
    (*bi).num = (*ch).num;
    (*bi).num_aggr = (*ch).num_aggr;
    (*bi).total_cycles = total_cycles;

    memcpy(
        (*bi).cycles_spark.as_mut_ptr() as *mut c_void,
        (*ch).cycles_spark.as_ptr() as *const c_void,
        NUM_SPARKS * size_of::<u64>(),
    );

    if !br_cntr.is_null() && br_cntr_nr != 0 {
        (*bi).br_cntr_nr = br_cntr_nr;
        memcpy(
            (*bi).br_cntr as *mut c_void,
            br_cntr.add(offset as usize * br_cntr_nr as usize) as *const c_void,
            br_cntr_nr as usize * size_of::<u64>(),
        );
    }
    (*bi).evsel = evsel;
}

#[no_mangle]
pub unsafe extern "C" fn block_info__process_sym(
    he: *mut hist_entry,
    bh: *mut block_hist,
    block_cycles_aggr: *mut u64,
    total_cycles: u64,
    br_cntr_nr: c_uint,
) -> c_int {
    let notes: *mut annotation;
    let ch: *mut cyc_hist;
    static mut al: addr_location = addr_location {
        map: ptr::null_mut(),
        sym: ptr::null_mut(),
    };
    let mut cycles: u64 = 0;

    if (*he).ms.map.is_null() || (*he).ms.sym.is_null() {
        return 0;
    }

    memset(&raw mut al as *mut c_void, 0, size_of::<addr_location>());
    al.map = (*he).ms.map;
    al.sym = (*he).ms.sym;

    notes = symbol__annotation((*he).ms.sym);
    if notes.is_null() || (*notes).branch.is_null() || (*(*notes).branch).cycles_hist.is_null() {
        return 0;
    }
    ch = (*(*notes).branch).cycles_hist;
    for i in 0..symbol__size((*he).ms.sym) {
        let chi = ch.add(i);
        if (*chi).num_aggr != 0 {
            let bi: *mut block_info;
            let he_block: *mut hist_entry;

            bi = block_info__new(br_cntr_nr);
            if bi.is_null() {
                return -1;
            }

            init_block_info(
                bi,
                (*he).ms.sym,
                chi,
                i as c_int,
                total_cycles,
                br_cntr_nr,
                (*(*notes).branch).br_cntr,
                hists_to_evsel((*he).hists),
            );
            cycles = cycles.wrapping_add((*bi).cycles_aggr / (*bi).num_aggr);

            he_block = hists__add_entry_block(&mut (*bh).block_hists, &raw mut al, bi);
            if he_block.is_null() {
                block_info__delete(bi);
                return -1;
            }
        }
    }

    if !block_cycles_aggr.is_null() {
        *block_cycles_aggr = (*block_cycles_aggr).wrapping_add(cycles);
    }

    0
}

unsafe extern "C" fn block_column_header(
    fmt: *mut perf_hpp_fmt,
    hpp: *mut perf_hpp,
    _hists: *mut hists,
    _line: c_int,
    _span: *mut c_int,
) -> c_int {
    let block_fmt = container_of_block_fmt(fmt);

    scnprintf(
        (*hpp).buf,
        (*hpp).size,
        cstr(b"%*s\0"),
        (*block_fmt).width,
        (*block_fmt).header,
    )
}

unsafe extern "C" fn block_column_width(
    fmt: *mut perf_hpp_fmt,
    _hpp: *mut perf_hpp,
    _hists: *mut hists,
) -> c_int {
    let block_fmt = container_of_block_fmt(fmt);

    (*block_fmt).width
}

unsafe fn color_pct(hpp: *mut perf_hpp, width: c_int, pct: c_double) -> c_int {
    // HAVE_SLANG_SUPPORT: use the slang color printer in browser mode when that build option exists.
    if use_browser != 0 {
        return __hpp__slsmg_color_printf(hpp, cstr(b"%*.2f%%\0"), width - 1, pct);
    }
    hpp_color_scnprintf(hpp, cstr(b"%*.2f%%\0"), width - 1, pct)
}

unsafe extern "C" fn block_total_cycles_pct_entry(
    fmt: *mut perf_hpp_fmt,
    hpp: *mut perf_hpp,
    he: *mut hist_entry,
) -> c_int {
    let block_fmt = container_of_block_fmt(fmt);
    let bi = (*he).block_info;
    let mut ratio: c_double = 0.0;

    if (*block_fmt).total_cycles != 0 {
        ratio = (*bi).cycles_aggr as c_double / (*block_fmt).total_cycles as c_double;
    }

    color_pct(hpp, (*block_fmt).width, 100.0 * ratio)
}

unsafe extern "C" fn block_total_cycles_pct_sort(
    fmt: *mut perf_hpp_fmt,
    left: *mut hist_entry,
    right: *mut hist_entry,
) -> int64_t {
    let block_fmt = container_of_block_fmt(fmt);
    let bi_l = (*left).block_info;
    let bi_r = (*right).block_info;
    let l: c_double;
    let r: c_double;

    if (*block_fmt).total_cycles != 0 {
        l = ((*bi_l).cycles_aggr as c_double / (*block_fmt).total_cycles as c_double) * 100000.0;
        r = ((*bi_r).cycles_aggr as c_double / (*block_fmt).total_cycles as c_double) * 100000.0;
        return l as int64_t - r as int64_t;
    }

    0
}

unsafe fn cycles_string(cycles: u64, buf: *mut c_char, size: c_int) {
    if cycles >= 1000000 {
        scnprintf(buf, size as usize, cstr(b"%.1fM\0"), cycles as c_double / 1000000.0);
    } else if cycles >= 1000 {
        scnprintf(buf, size as usize, cstr(b"%.1fK\0"), cycles as c_double / 1000.0);
    } else {
        scnprintf(buf, size as usize, cstr(b"%1d\0"), cycles);
    }
}

unsafe extern "C" fn block_cycles_lbr_entry(
    fmt: *mut perf_hpp_fmt,
    hpp: *mut perf_hpp,
    he: *mut hist_entry,
) -> c_int {
    let block_fmt = container_of_block_fmt(fmt);
    let bi = (*he).block_info;
    let mut cycles_buf = [0 as c_char; 16];

    cycles_string((*bi).cycles_aggr, cycles_buf.as_mut_ptr(), size_of::<[c_char; 16]>() as c_int);

    scnprintf(
        (*hpp).buf,
        (*hpp).size,
        cstr(b"%*s\0"),
        (*block_fmt).width,
        cycles_buf.as_mut_ptr(),
    )
}

unsafe extern "C" fn block_cycles_pct_entry(
    fmt: *mut perf_hpp_fmt,
    hpp: *mut perf_hpp,
    he: *mut hist_entry,
) -> c_int {
    let block_fmt = container_of_block_fmt(fmt);
    let bi = (*he).block_info;
    let mut ratio: c_double = 0.0;
    let avg: u64;

    if (*block_fmt).block_cycles != 0 && (*bi).num_aggr != 0 {
        avg = (*bi).cycles_aggr / (*bi).num_aggr;
        ratio = avg as c_double / (*block_fmt).block_cycles as c_double;
    }

    color_pct(hpp, (*block_fmt).width, 100.0 * ratio)
}

unsafe extern "C" fn block_avg_cycles_entry(
    fmt: *mut perf_hpp_fmt,
    hpp: *mut perf_hpp,
    he: *mut hist_entry,
) -> c_int {
    let block_fmt = container_of_block_fmt(fmt);
    let bi = (*he).block_info;
    let mut cycles_buf = [0 as c_char; 16];

    cycles_string(
        (*bi).cycles_aggr / (*bi).num_aggr,
        cycles_buf.as_mut_ptr(),
        size_of::<[c_char; 16]>() as c_int,
    );

    scnprintf(
        (*hpp).buf,
        (*hpp).size,
        cstr(b"%*s\0"),
        (*block_fmt).width,
        cycles_buf.as_mut_ptr(),
    )
}

unsafe extern "C" fn block_range_entry(
    fmt: *mut perf_hpp_fmt,
    hpp: *mut perf_hpp,
    he: *mut hist_entry,
) -> c_int {
    let block_fmt = container_of_block_fmt(fmt);
    let bi = (*he).block_info;
    let mut buf = [0 as c_char; 128];
    let mut start_line: *mut c_char;
    let mut end_line: *mut c_char;

    symbol_conf.addr2line_disable_warn = true;

    start_line = map__srcline((*he).ms.map, (*(*bi).sym).start + (*bi).start, (*he).ms.sym);

    end_line = map__srcline(
        (*he).ms.map,
        (*(*bi).sym).start + (*bi).end as u64,
        (*he).ms.sym,
    );

    if start_line != SRCLINE_UNKNOWN && end_line != SRCLINE_UNKNOWN {
        scnprintf(
            buf.as_mut_ptr(),
            size_of::<[c_char; 128]>(),
            cstr(b"[%s -> %s]\0"),
            start_line,
            end_line,
        );
    } else {
        scnprintf(
            buf.as_mut_ptr(),
            size_of::<[c_char; 128]>(),
            cstr(b"[%7lx -> %7lx]\0"),
            (*bi).start as c_ulong,
            (*bi).end as c_ulong,
        );
    }

    zfree_srcline(&mut start_line);
    zfree_srcline(&mut end_line);

    scnprintf(
        (*hpp).buf,
        (*hpp).size,
        cstr(b"%*s\0"),
        (*block_fmt).width,
        buf.as_mut_ptr(),
    )
}

unsafe extern "C" fn block_dso_entry(
    fmt: *mut perf_hpp_fmt,
    hpp: *mut perf_hpp,
    he: *mut hist_entry,
) -> c_int {
    let block_fmt = container_of_block_fmt(fmt);
    let map = (*he).ms.map;

    if !map.is_null() && !map__dso(map).is_null() {
        return scnprintf(
            (*hpp).buf,
            (*hpp).size,
            cstr(b"%*s\0"),
            (*block_fmt).width,
            dso__short_name(map__dso(map)),
        );
    }

    scnprintf(
        (*hpp).buf,
        (*hpp).size,
        cstr(b"%*s\0"),
        (*block_fmt).width,
        cstr(b"[unknown]\0"),
    )
}

unsafe fn init_block_header(block_fmt: *mut block_fmt) {
    let fmt = &mut (*block_fmt).fmt as *mut perf_hpp_fmt;

    BUG_ON((*block_fmt).idx >= PERF_HPP_REPORT__BLOCK_MAX_INDEX as c_int);

    (*block_fmt).header = block_columns[(*block_fmt).idx as usize].name;
    (*block_fmt).width = block_columns[(*block_fmt).idx as usize].width;

    (*fmt).header = Some(block_column_header);
    (*fmt).width = Some(block_column_width);
}

unsafe extern "C" fn block_branch_counter_entry(
    fmt: *mut perf_hpp_fmt,
    hpp: *mut perf_hpp,
    he: *mut hist_entry,
) -> c_int {
    let block_fmt = container_of_block_fmt(fmt);
    let bi = (*he).block_info;
    let mut buf: *mut c_char = ptr::null_mut();
    let ret: c_int;

    if annotation_br_cntr_entry(
        &mut buf,
        (*bi).br_cntr_nr,
        (*bi).br_cntr,
        (*bi).num_aggr,
        (*bi).evsel,
    ) != 0
    {
        return 0;
    }

    ret = scnprintf((*hpp).buf, (*hpp).size, cstr(b"%*s\0"), (*block_fmt).width, buf);
    free(buf as *mut c_void);
    ret
}

unsafe fn hpp_register(block_fmt: *mut block_fmt, idx: c_int, hpp_list: *mut perf_hpp_list) {
    let fmt = &mut (*block_fmt).fmt as *mut perf_hpp_fmt;

    (*block_fmt).idx = idx;
    INIT_LIST_HEAD(&mut (*fmt).list);
    INIT_LIST_HEAD(&mut (*fmt).sort_list);

    match idx {
        PERF_HPP_REPORT__BLOCK_TOTAL_CYCLES_PCT => {
            (*fmt).color = Some(block_total_cycles_pct_entry);
            (*fmt).cmp = Some(block_info__cmp);
            (*fmt).sort = Some(block_total_cycles_pct_sort);
        }
        PERF_HPP_REPORT__BLOCK_LBR_CYCLES => {
            (*fmt).entry = Some(block_cycles_lbr_entry);
        }
        PERF_HPP_REPORT__BLOCK_CYCLES_PCT => {
            (*fmt).color = Some(block_cycles_pct_entry);
        }
        PERF_HPP_REPORT__BLOCK_AVG_CYCLES => {
            (*fmt).entry = Some(block_avg_cycles_entry);
        }
        PERF_HPP_REPORT__BLOCK_RANGE => {
            (*fmt).entry = Some(block_range_entry);
        }
        PERF_HPP_REPORT__BLOCK_DSO => {
            (*fmt).entry = Some(block_dso_entry);
        }
        PERF_HPP_REPORT__BLOCK_BRANCH_COUNTER => {
            (*fmt).entry = Some(block_branch_counter_entry);
        }
        _ => {
            return;
        }
    }

    init_block_header(block_fmt);
    perf_hpp_list__column_register(hpp_list, fmt);
}

unsafe fn register_block_columns(
    hpp_list: *mut perf_hpp_list,
    block_fmts: *mut block_fmt,
    block_hpps: *mut c_int,
    nr_hpps: c_int,
) {
    for i in 0..nr_hpps {
        hpp_register(block_fmts.add(i as usize), *block_hpps.add(i as usize), hpp_list);
    }
}

unsafe fn init_block_hist(
    bh: *mut block_hist,
    block_fmts: *mut block_fmt,
    block_hpps: *mut c_int,
    nr_hpps: c_int,
) {
    __hists__init(&mut (*bh).block_hists, &mut (*bh).block_list);
    perf_hpp_list__init(&mut (*bh).block_list);
    (*bh).block_list.nr_header_lines = 1;

    register_block_columns(&mut (*bh).block_list, block_fmts, block_hpps, nr_hpps);

    /* Sort by the first fmt */
    perf_hpp_list__register_sort_field(&mut (*bh).block_list, &mut (*block_fmts).fmt);
}

unsafe fn process_block_report(
    hists: *mut hists,
    block_report: *mut block_report,
    total_cycles: u64,
    block_hpps: *mut c_int,
    nr_hpps: c_int,
    br_cntr_nr: c_uint,
) -> c_int {
    let mut next = rb_first_cached(&mut (*hists).entries);
    let bh = &mut (*block_report).hist as *mut block_hist;
    let mut he: *mut hist_entry;

    if nr_hpps > PERF_HPP_REPORT__BLOCK_MAX_INDEX as c_int {
        return -1;
    }

    (*block_report).nr_fmts = nr_hpps;
    init_block_hist(bh, (*block_report).fmts.as_mut_ptr(), block_hpps, nr_hpps);

    while !next.is_null() {
        he = rb_entry_hist_entry(next);
        block_info__process_sym(he, bh, &mut (*block_report).cycles, total_cycles, br_cntr_nr);
        next = rb_next(&mut (*he).rb_node);
    }

    for i in 0..nr_hpps {
        (*block_report).fmts[i as usize].total_cycles = total_cycles;
        (*block_report).fmts[i as usize].block_cycles = (*block_report).cycles;
    }

    hists__output_resort(&mut (*bh).block_hists, ptr::null_mut());
    0
}

#[no_mangle]
pub unsafe extern "C" fn block_info__create_report(
    evlist: *mut evlist,
    total_cycles: u64,
    block_hpps: *mut c_int,
    nr_hpps: c_int,
    nr_reps: *mut c_int,
) -> *mut block_report {
    let block_reports: *mut block_report;
    let nr_hists = evlist__nr_entries(evlist);
    let mut i: c_int = 0;
    let mut pos: *mut evsel;

    block_reports = calloc(nr_hists as usize, size_of::<block_report>()) as *mut block_report;
    if block_reports.is_null() {
        return ptr::null_mut();
    }

    pos = evlist__first(evlist);
    while !pos.is_null() {
        let hists = evsel__hists(pos);

        process_block_report(
            hists,
            block_reports.add(i as usize),
            total_cycles,
            block_hpps,
            nr_hpps,
            evlist__nr_br_cntr(evlist),
        );
        i += 1;
        pos = evlist__next(evlist, pos);
    }

    *nr_reps = nr_hists;
    block_reports
}

#[no_mangle]
pub unsafe extern "C" fn block_info__free_report(reps: *mut block_report, nr_reps: c_int) {
    for i in 0..nr_reps {
        hists__delete_entries(&mut (*reps.add(i as usize)).hist.block_hists);
    }

    free(reps as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn report__browse_block_hists(
    bh: *mut block_hist,
    min_percent: c_float,
    evsel: *mut evsel,
    env: *mut perf_env,
) -> c_int {
    let ret: c_int;

    match use_browser {
        0 => {
            symbol_conf.report_individual_block = true;
            hists__fprintf(
                &mut (*bh).block_hists,
                true,
                0,
                0,
                min_percent,
                stdout,
                true,
            );
            return 0;
        }
        1 => {
            symbol_conf.report_individual_block = true;
            ret = block_hists_tui_browse(bh, evsel, min_percent, env);
            return ret;
        }
        _ => {
            return -1;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn block_info__total_cycles_percent(he: *mut hist_entry) -> c_float {
    let bi = (*he).block_info;

    if (*bi).total_cycles != 0 {
        return ((*bi).cycles as c_double * 100.0 / (*bi).total_cycles as c_double) as c_float;
    }

    0.0
}
