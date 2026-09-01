// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/ui/browsers/hists.c.
// C include dependencies are intentionally represented as external types,
// constants, globals and functions supplied by the surrounding repository.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::{c_char, c_double, c_float, c_int, c_long, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of_mut, null, null_mut};

type bool_ = bool;
type size_t = usize;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type off_t = i64;
type FILE = c_void;
type DIR = c_void;

const LEVEL_OFFSET_STEP: c_int = 3;
const MAX_OPTIONS: usize = 32;
const HIERARCHY_INDENT: c_int = 3;
const PERF_RECORD_LOST: usize = 2;
const ANNOTATION_DUMMY_LEN: u64 = 1;
const BITS_PER_LONG: c_int = 64;
const NSEC_PER_MSEC: c_ulong = 1_000_000;
const F_OK: c_int = 0;
const DT_REG: u8 = 8;
const SEEK_SET: c_int = 0;
const SEEK_CUR: c_int = 1;
const SEEK_END: c_int = 2;
const K_TIMER: c_int = -1;
const K_TAB: c_int = -2;
const K_UNTAB: c_int = -3;
const K_ENTER: c_int = 10;
const K_RIGHT: c_int = 1001;
const K_LEFT: c_int = 1002;
const K_ESC: c_int = 27;
const K_F1: c_int = 1003;
const K_SWITCH_INPUT_DATA: c_int = 1004;
const K_RELOAD: c_int = 1005;
const HE_COLORSET_NORMAL: c_int = 0;
const HE_COLORSET_SELECTED: c_int = 1;
const HE_COLORSET_ROOT: c_int = 2;
const HE_COLORSET_TOP: c_int = 3;
const SLSMG_RARROW_CHAR: c_int = '>' as c_int;
const NO_ADDR: u64 = !0u64;
const HISTC_THREAD: c_int = 0;
const HISTC_DSO: c_int = 1;
const HISTC_SOCKET: c_int = 2;
const PERF_HPP__OVERHEAD: usize = 0;
const PERF_HPP__LATENCY: usize = 1;
const PERF_HPP__OVERHEAD_SYS: usize = 2;
const PERF_HPP__OVERHEAD_US: usize = 3;
const PERF_HPP__OVERHEAD_GUEST_SYS: usize = 4;
const PERF_HPP__OVERHEAD_GUEST_US: usize = 5;
const PERF_HPP__OVERHEAD_ACC: usize = 6;
const PERF_HPP__LATENCY_ACC: usize = 7;
const PERF_HPP__MEM_STAT_OP: usize = 8;
const PERF_HPP__MEM_STAT_CACHE: usize = 9;
const PERF_HPP__MEM_STAT_MEMORY: usize = 10;
const PERF_HPP__MEM_STAT_SNOOP: usize = 11;
const PERF_HPP__MEM_STAT_DTLB: usize = 12;
const PERF_HPP_FMT_TYPE__PERCENT: c_int = 0;
const PERF_HPP_FMT_TYPE__LATENCY: c_int = 1;
const PERF_MEM_STAT_OP: c_int = 0;
const PERF_MEM_STAT_CACHE: c_int = 1;
const PERF_MEM_STAT_MEMORY: c_int = 2;
const PERF_MEM_STAT_SNOOP: c_int = 3;
const PERF_MEM_STAT_DTLB: c_int = 4;
const CHAIN_FLAT: c_int = 0;
const CHAIN_FOLDED: c_int = 1;
const CHAIN_GRAPH_REL: c_int = 2;
const HMD_FORCE_CHILD: c_int = 1;
const SORT_MODE__BRANCH: c_int = 1;
const A_NORMAL: rstype = 0;
const A_ASM: rstype = 1;
const A_SOURCE: rstype = 2;

fn CTRL(c: c_int) -> c_int { c & 0x1f }

#[repr(C)] pub struct rb_node { _priv: [u8; 0] }
#[repr(C)] pub struct rb_root { _priv: [u8; 0] }
#[repr(C)] pub struct rb_root_cached { pub rb_root: rb_root }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct dso { _priv: [u8; 0] }
#[repr(C)] pub struct map { _priv: [u8; 0] }
#[repr(C)] pub struct maps { _priv: [u8; 0] }
#[repr(C)] pub struct machine { pub vmlinux_map: *mut map }
#[repr(C)] pub struct thread { _priv: [u8; 0] }
#[repr(C)] pub struct symbol { pub name: *const c_char }
#[repr(C)] pub struct annotation { pub src: *mut c_void }
#[repr(C)] pub struct annotated_source { _priv: [u8; 0] }
#[repr(C)] pub struct res_sample { _priv: [u8; 0] }
pub type rstype = c_int;

#[repr(C)] pub struct map_symbol { pub map: *mut map, pub sym: *mut symbol, pub thread: *mut thread }
#[repr(C)] pub struct hist_stat { pub period: u64, pub latency: u64, pub period_sys: u64, pub period_us: u64, pub period_guest_sys: u64, pub period_guest_us: u64 }
#[repr(C)] pub struct mem_type_self { pub type_name: *const c_char }
#[repr(C)] pub struct mem_type { pub histograms: *mut c_void, pub self_: mem_type_self }
#[repr(C)] pub struct callchain_list { pub list: list_head, pub ms: map_symbol, pub has_children: bool, pub unfolded: bool }
#[repr(C)] pub struct callchain_node { pub rb_node: rb_node, pub rb_root: rb_root, pub val: list_head, pub parent_val: list_head, pub children_hit: u64 }
#[repr(C)] pub struct branch_side { pub ms: map_symbol, pub al_addr: u64 }
#[repr(C)] pub struct branch_info { pub from: branch_side, pub to: branch_side }
#[repr(C)] pub struct perf_hpp { pub buf: *mut c_char, pub size: size_t, pub ptr: *mut c_void }
#[repr(C)] pub struct perf_hpp_fmt {
    pub color: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hist_entry) -> c_int>,
    pub entry: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hist_entry) -> c_int>,
    pub header: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hists, c_int, *mut c_int) -> size_t>,
    pub width: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut c_void, *mut hists) -> c_int>,
    pub user_len: c_int,
    pub len: c_int,
}
#[repr(C)] pub struct perf_hpp_list { pub nr_header_lines: c_int }
#[repr(C)] pub struct perf_hpp_list_node { pub list: list_head, pub hpp: perf_hpp_list }
#[repr(C)] pub struct hists_stats { pub nr_samples: c_ulong }
#[repr(C)] pub struct hists {
    pub entries: rb_root_cached, pub nr_entries: u32, pub nr_non_filtered_entries: u64,
    pub hpp_list: *mut perf_hpp_list, pub hpp_formats: list_head, pub nr_hpp_node: c_int,
    pub stats: hists_stats, pub thread_filter: *mut thread, pub dso_filter: *mut dso,
    pub socket_filter: c_int, pub symbol_filter_str: *mut c_char,
}
#[repr(C)] pub struct hist_entry {
    pub rb_node: rb_node, pub hroot_out: rb_root_cached, pub sorted_chain: rb_root,
    pub hists: *mut hists, pub hpp_list: *mut perf_hpp_list, pub ms: map_symbol,
    pub stat: hist_stat, pub stat_acc: *mut hist_stat, pub thread: *mut thread,
    pub branch_info: *mut branch_info, pub mem_type: *mut mem_type, pub res_samples: *mut res_sample,
    pub callchain: *mut c_void, pub filtered: bool, pub leaf: bool, pub unfolded: bool,
    pub has_children: bool, pub init_have_children: bool, pub has_no_entry: bool,
    pub row_offset: off_t, pub nr_rows: u16, pub depth: c_int, pub socket: c_int,
    pub ip: u64, pub time: c_ulong, pub num_res: c_int,
}
#[repr(C)] pub struct ui_browser {
    pub entries: *mut c_void, pub top: *mut rb_node, pub nr_entries: u64, pub extra_title_lines: u16,
    pub rows: u16, pub index: u16, pub top_idx: u16, pub width: c_int, pub horiz_scroll: c_int,
    pub navkeypressed: bool, pub use_navkeypressed: bool, pub no_samples_msg: *const c_char,
    pub priv_: *mut c_void,
    pub refresh: Option<unsafe extern "C" fn(*mut ui_browser) -> c_uint>,
    pub refresh_dimensions: Option<unsafe extern "C" fn(*mut ui_browser)>,
    pub seek: Option<unsafe extern "C" fn(*mut ui_browser, off_t, c_int)>,
    pub write: Option<unsafe extern "C" fn(*mut ui_browser, *mut c_void, c_int)>,
    pub filter: Option<unsafe extern "C" fn(*mut ui_browser, *mut c_void) -> bool>,
}
type c_uint = u32;
#[repr(C)] pub struct hist_browser_timer { pub refresh: c_int, pub timer: Option<unsafe extern "C" fn(*mut c_void)>, pub arg: *mut c_void }
#[repr(C)] pub struct perf_env { pub arch: *mut c_char }
#[repr(C)] pub struct hist_browser {
    pub b: ui_browser, pub hists: *mut hists, pub hbt: *mut hist_browser_timer, pub env: *mut perf_env,
    pub title: Option<unsafe extern "C" fn(*mut hist_browser, *mut c_char, size_t) -> c_int>,
    pub he_selection: *mut hist_entry, pub selection: *mut map_symbol, pub block_evsel: *mut evsel,
    pub pstack: *mut c_void, pub min_pcnt: c_double, pub nr_callchain_rows: u64,
    pub nr_hierarchy_entries: u64, pub nr_non_filtered_entries: u64, pub show_headers: bool,
    pub c2c_filter: bool, pub show_dso: bool, pub print_seq: c_int,
}
#[repr(C)] pub struct evlist { _priv: [u8; 0] }
#[repr(C)] pub struct evsel_core { pub nr_members: c_int, pub node: list_head }
#[repr(C)] pub struct evsel { pub core: evsel_core, pub evlist: *mut evlist }
#[repr(C)] pub struct evlist_core { pub entries: list_head }
#[repr(C)] pub struct perf_top { pub lost: u64, pub lost_total: u64, pub drop: u64, pub drop_total: u64, pub zero: bool, pub evlist: *mut evlist }
#[repr(C)] pub struct block_hist { pub block_hists: hists }
#[repr(C)] pub struct dirent { pub d_type: u8, pub d_name: [c_char; 256] }
#[repr(C)] pub struct evlist_stats { pub nr_lost_warned: u64, pub nr_events: [u64; 256] }
#[repr(C)] pub struct symbol_conf_t {
    pub has_filter: bool, pub report_hierarchy: bool, pub show_hist_headers: bool,
    pub use_callchain: bool, pub show_branchflag_count: bool, pub cumulate_callchain: bool,
    pub report_individual_block: bool, pub event_group: bool, pub filter_relative: c_int,
    pub group_sort_idx: c_int, pub col_width_list_str: *const c_char,
    pub field_sep: *const c_char, pub time_quantum: c_ulong,
}
#[repr(C)] pub struct callchain_param_t {
    pub mode: c_int, pub min_percent: c_double,
    pub sort: Option<unsafe extern "C" fn(*mut rb_root, *mut c_void, u64, *mut callchain_param_t)>,
}
#[repr(C)] pub struct annotate_opts_t { pub objdump_path: *mut c_char, pub show_br_cntr: bool }

unsafe extern "C" {
    static mut symbol_conf: symbol_conf_t;
    static mut callchain_param: callchain_param_t;
    static mut perf_hpp__format: [perf_hpp_fmt; 32];
    static mut input_name: *const c_char;
    static mut verbose: c_int;
    static mut sort_order: *const c_char;
    static mut sort__mode: c_int;
    static mut annotate_opts: annotate_opts_t;
    static mut errno: c_int;

    fn rb_first_cached(root: *const rb_root_cached) -> *mut rb_node;
    fn rb_first(root: *const rb_root) -> *mut rb_node;
    fn rb_last(root: *const rb_root) -> *mut rb_node;
    fn rb_next(node: *const rb_node) -> *mut rb_node;
    fn rb_hierarchy_next(node: *mut rb_node) -> *mut rb_node;
    fn rb_hierarchy_prev(node: *mut rb_node) -> *mut rb_node;
    fn rb_hierarchy_last(node: *mut rb_node) -> *mut rb_node;
    fn __rb_hierarchy_next(node: *mut rb_node, mode: c_int) -> *mut rb_node;
    fn RB_EMPTY_ROOT(root: *const rb_root) -> bool;

    fn hists__has_filter(hists: *mut hists) -> bool;
    fn hists__sort_list_width(hists: *mut hists) -> c_int;
    fn hists__total_period(hists: *mut hists) -> u64;
    fn hists__has_callchains(hists: *mut hists) -> bool;
    fn hists__reset_column_width(hists: *mut hists);
    fn hists__filter_by_thread(hists: *mut hists);
    fn hists__filter_by_dso(hists: *mut hists);
    fn hists__filter_by_socket(hists: *mut hists);
    fn hists__filter_by_symbol(hists: *mut hists);
    fn __hists__scnprintf_title(hists: *mut hists, bf: *mut c_char, size: size_t, show_freq: bool) -> c_int;
    fn hist_entry__get_percent_limit(he: *mut hist_entry) -> c_float;
    fn hist_entry__has_callchains(he: *mut hist_entry) -> bool;
    fn hist_entry__has_hierarchy_children(he: *mut hist_entry, min: c_double) -> bool;
    fn hist_entry__sym_snprintf(he: *mut hist_entry, bf: *mut c_char, size: size_t, width: c_int) -> c_int;
    fn hist_entry__snprintf_alignment(he: *mut hist_entry, hpp: *mut perf_hpp, fmt: *mut perf_hpp_fmt, printed: c_int) -> c_int;
    fn hists_to_evsel(hists: *mut hists) -> *mut evsel;
    fn evsel__hists(evsel: *mut evsel) -> *mut hists;
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
    fn evsel__group_name(evsel: *mut evsel) -> *const c_char;
    fn evsel__is_group_event(evsel: *mut evsel) -> bool;
    fn evsel__is_group_leader(evsel: *mut evsel) -> bool;
    fn evsel__is_dummy_event(evsel: *mut evsel) -> bool;
    fn evsel__next(evsel: *mut evsel) -> *mut evsel;
    fn evsel__prev(evsel: *mut evsel) -> *mut evsel;
    fn evsel__first(evlist: *mut evlist) -> *mut evsel;
    fn evsel__last(evlist: *mut evlist) -> *mut evsel;
    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__stats(evlist: *mut evlist) -> *mut evlist_stats;
    fn evlist__set_selected(evlist: *mut evlist, evsel: *mut evsel);
    fn evlist__toggle_enable(evlist: *mut evlist);
    fn evlist__enabled(evlist: *mut evlist) -> bool;

    fn ui_browser__refresh_dimensions(browser: *mut ui_browser);
    fn ui_browser__reset_index(browser: *mut ui_browser);
    fn ui_browser__update_nr_entries(browser: *mut ui_browser, nr_entries: u64);
    fn ui_browser__warning(browser: *mut ui_browser, delay_secs: c_int, fmt: *const c_char, ...) -> c_int;
    fn ui_browser__show(browser: *mut ui_browser, title: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn ui_browser__hide(browser: *mut ui_browser);
    fn ui_browser__run(browser: *mut ui_browser, delay_secs: c_int) -> c_int;
    fn ui_browser__show_title(browser: *mut ui_browser, title: *const c_char);
    fn ui_browser__is_current_entry(browser: *mut ui_browser, row: u16) -> bool;
    fn ui_browser__set_color(browser: *mut ui_browser, color: c_int);
    fn ui_browser__set_percent_color(browser: *mut ui_browser, percent: c_double, current: bool);
    fn ui_browser__gotorc(browser: *mut ui_browser, row: u16, col: c_int);
    fn ui_browser__gotorc_title(browser: *mut ui_browser, row: c_int, col: c_int);
    fn ui_browser__write_nstring(browser: *mut ui_browser, s: *const c_char, width: c_int);
    fn ui_browser__write_graph(browser: *mut ui_browser, ch: c_int);
    fn ui_browser__printf(browser: *mut ui_browser, fmt: *const c_char, ...) -> c_int;
    fn ui_browser__input_window(title: *const c_char, text: *const c_char, input: *mut c_char, exit_msg: *const c_char, delay_secs: c_int) -> c_int;
    fn ui_browser__help_window(browser: *mut ui_browser, text: *const c_char);
    fn ui_browser__dialog_yesno(browser: *mut ui_browser, text: *const c_char) -> bool;
    fn ui_browser__warn_unhandled_hotkey(browser: *mut ui_browser, key: c_int, delay_secs: c_int, msg: *const c_char);
    fn ui_browser__handle_resize(browser: *mut ui_browser);
    fn ui_browser__list_head_refresh(browser: *mut ui_browser) -> c_uint;
    fn ui_browser__list_head_seek(browser: *mut ui_browser, offset: off_t, whence: c_int);

    fn ui_helpline__push(msg: *const c_char);
    fn ui_helpline__pop();
    fn ui_helpline__fpush(fmt: *const c_char, ...) -> c_int;
    fn ui__warning(fmt: *const c_char, ...) -> c_int;
    fn ui__popup_menu(n: c_int, options: *mut *mut c_char, key: *mut c_int) -> c_int;
    fn ui__question_window(title: *const c_char, text: *const c_char, exit_msg: *const c_char, delay: c_int);

    fn perf_hpp__should_skip(fmt: *mut perf_hpp_fmt, hists: *mut hists) -> bool;
    fn hpp__fmt(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry, get: unsafe extern "C" fn(*mut hist_entry) -> u64, fmtstr: *const c_char, printer: unsafe extern "C" fn(*mut perf_hpp, *const c_char, ...) -> c_int, fmttype: c_int) -> c_int;
    fn hpp__fmt_mem_stat(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry, typ: c_int, fmtstr: *const c_char, printer: unsafe extern "C" fn(*mut perf_hpp, *const c_char, ...) -> c_int) -> c_int;
    fn perf_hpp__set_elide(field: c_int, elide: bool);
    fn perf_hpp__set_user_width(s: *const c_char);
    fn advance_hpp(hpp: *mut perf_hpp, inc: c_int);

    fn callchain_list__sym_name(cl: *mut callchain_list, bf: *mut c_char, size: size_t, show_dso: bool) -> *const c_char;
    fn callchain_list_counts__printf_value(cl: *mut callchain_list, arg: *mut c_void, buf: *mut c_char, size: size_t);
    fn callchain_node__scnprintf_value(node: *mut callchain_node, buf: *mut c_char, size: size_t, total: u64) -> c_int;
    fn callchain_cumul_hits(node: *mut callchain_node) -> u64;
    fn callchain_node__make_parent_list(node: *mut callchain_node);
    fn res_sample_init();
    fn res_sample_browse(samples: *mut res_sample, num: c_int, evsel: *mut evsel, typ: rstype);

    fn symbol__annotation(sym: *mut symbol) -> *mut annotation;
    fn symbol__new(addr: u64, len: u64, binding: c_int, typ: c_int, name: *const c_char) -> *mut symbol;
    fn symbol__delete(sym: *mut symbol);
    fn symbol__hists(sym: *mut symbol, nr: c_int) -> *mut annotated_source;
    fn dso__insert_symbol(dso: *mut dso, sym: *mut symbol);
    fn dso__annotate_warned(dso: *mut dso) -> bool;
    fn dso__short_name(dso: *mut dso) -> *const c_char;
    fn dso__get(dso: *mut dso) -> *mut dso;
    fn dso__put(dso: *mut dso);
    fn map__dso(map: *mut map) -> *mut dso;
    fn map__get(map: *mut map) -> *mut map;
    fn map__browse(map: *mut map);
    fn __map__is_kernel(map: *mut map) -> bool;
    fn map_symbol__copy(dst: *mut map_symbol, src: *mut map_symbol);
    fn map_symbol__exit(ms: *mut map_symbol);
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn maps__machine(maps: *mut maps) -> *mut machine;
    fn thread__get(thread: *mut thread) -> *mut thread;
    fn thread__zput(thread: *mut thread);
    fn thread__comm_set(thread: *mut thread) -> bool;
    fn thread__comm_str(thread: *mut thread) -> *const c_char;
    fn thread__tid(thread: *mut thread) -> c_int;
    fn pstack__new(n: c_int) -> *mut c_void;
    fn pstack__delete(ps: *mut c_void);
    fn pstack__push(ps: *mut c_void, ptr: *mut c_void);
    fn pstack__remove(ps: *mut c_void, ptr: *mut c_void);
    fn pstack__empty(ps: *mut c_void) -> bool;
    fn pstack__peek(ps: *mut c_void) -> *const c_void;
    fn hists__has(hists: *mut hists, field: c_int) -> bool;
    fn hist_entry__annotate_data_tui(he: *mut hist_entry, evsel: *mut evsel, hbt: *mut hist_browser_timer);
    fn __hist_entry__tui_annotate(he: *mut hist_entry, ms: *mut map_symbol, evsel: *mut evsel, hbt: *mut hist_browser_timer, addr: u64) -> c_int;
    fn perf_env__lookup_objdump(env: *mut perf_env, path: *mut *mut c_char) -> c_int;
    fn perf_top__reset_sample_counters(top: *mut perf_top);
    fn script_browse(script_opt: *mut c_char, evsel: *mut evsel);
    fn timestamp__scnprintf_usec(t: c_ulong, bf: *mut c_char, size: size_t) -> c_int;
    fn block_info__total_cycles_percent(he: *mut hist_entry) -> c_float;
    fn tui__header_window(session: *mut c_void);
    fn evsel__session(evsel: *mut evsel) -> *mut c_void;
    fn annotation_br_cntr_abbr_list(text: *mut *mut c_char, evsel: *mut evsel, flag: bool) -> c_int;
    fn convert_unit(n: c_ulong, unit: *mut c_char) -> c_ulong;
    fn is_perf_magic(magic: u64) -> bool;

    fn malloc(size: size_t) -> *mut c_void;
    fn zalloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(ptr: *mut c_void, val: c_int, n: size_t) -> *mut c_void;
    fn memmove(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strtod(s: *const c_char, end: *mut *mut c_char) -> c_double;
    fn strim(s: *mut c_char) -> *mut c_char;
    fn skip_spaces(s: *const c_char) -> *const c_char;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn fprintf(fp: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(fp: *mut FILE) -> c_int;
    fn fread(ptr: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn access(path: *const c_char, mode: c_int) -> c_int;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> *const c_char;
    fn isspace(c: c_int) -> c_int;
    fn SLang_reset_tty();
    fn SLang_init_tty(a: c_int, b: c_int, c: c_int);
    fn SLtty_set_suspend_state(state: bool);
}

unsafe fn rb_entry_hist_entry(nd: *mut rb_node) -> *mut hist_entry { nd as *mut hist_entry }
unsafe fn rb_entry_callchain_node(nd: *mut rb_node) -> *mut callchain_node { nd as *mut callchain_node }
unsafe fn container_of_hist_browser(browser: *mut ui_browser) -> *mut hist_browser { browser as *mut hist_browser }
unsafe fn container_of_callchain_list(ms: *mut map_symbol) -> *mut callchain_list { ms as *mut callchain_list }
unsafe fn list_empty(head: *const list_head) -> bool { (*head).next == head as *mut list_head }
unsafe fn list_first_callchain(head: *mut list_head) -> *mut callchain_list { (*head).next as *mut callchain_list }
unsafe fn list_last_callchain(head: *mut list_head) -> *mut callchain_list { (*head).prev as *mut callchain_list }
unsafe fn list_first_hpp_node(head: *mut list_head) -> *mut perf_hpp_list_node { (*head).next as *mut perf_hpp_list_node }
unsafe fn zfree_char(pp: *mut *mut c_char) { if !(*pp).is_null() { free(*pp as *mut c_void); *pp = null_mut(); } }

#[no_mangle]
pub unsafe extern "C" fn hist_browser__init_hpp();

unsafe extern "C" fn hist_browser__has_filter(hb: *mut hist_browser) -> bool {
    hists__has_filter((*hb).hists) || (*hb).min_pcnt != 0.0 || symbol_conf.has_filter || (*hb).c2c_filter
}

unsafe extern "C" fn hist_browser__get_folding(browser: *mut hist_browser) -> c_int {
    let hists = (*browser).hists;
    let mut unfolded_rows = 0;
    let mut nd = rb_first_cached(addr_of_mut!((*hists).entries));
    loop {
        nd = hists__filter_entries(nd, (*browser).min_pcnt as c_float);
        if nd.is_null() { break; }
        let he = rb_entry_hist_entry(nd);
        if (*he).leaf && (*he).unfolded { unfolded_rows += (*he).nr_rows as c_int; }
        nd = rb_hierarchy_next(nd);
    }
    unfolded_rows
}

unsafe extern "C" fn hist_browser__set_title_space(hb: *mut hist_browser) {
    let browser = addr_of_mut!((*hb).b);
    let hpp_list = (*(*hb).hists).hpp_list;
    (*browser).extra_title_lines = if (*hb).show_headers { (*hpp_list).nr_header_lines as u16 } else { 0 };
}

unsafe extern "C" fn hist_browser__nr_entries(hb: *mut hist_browser) -> u32 {
    let nr_entries = if symbol_conf.report_hierarchy {
        (*hb).nr_hierarchy_entries as u32
    } else if hist_browser__has_filter(hb) {
        (*hb).nr_non_filtered_entries as u32
    } else {
        (*(*hb).hists).nr_entries
    };
    (*hb).nr_callchain_rows = hist_browser__get_folding(hb) as u64;
    nr_entries.wrapping_add((*hb).nr_callchain_rows as u32)
}

unsafe extern "C" fn hist_browser__update_rows(hb: *mut hist_browser) {
    let browser = addr_of_mut!((*hb).b);
    let hpp_list = (*(*hb).hists).hpp_list;
    if !(*hb).show_headers {
        (*browser).rows = (*browser).rows.wrapping_add((*browser).extra_title_lines);
        (*browser).extra_title_lines = 0;
        return;
    }
    (*browser).extra_title_lines = (*hpp_list).nr_header_lines as u16;
    (*browser).rows = (*browser).rows.wrapping_sub((*browser).extra_title_lines);
    let index_row = (*browser).index.wrapping_sub((*browser).top_idx);
    if index_row >= (*browser).rows {
        (*browser).index = (*browser).index.wrapping_sub(index_row.wrapping_sub((*browser).rows).wrapping_add(1));
    }
}

unsafe extern "C" fn hist_browser__refresh_dimensions(browser: *mut ui_browser) {
    let hb = container_of_hist_browser(browser);
    (*browser).width = 3 + hists__sort_list_width((*hb).hists) + size_of::<[c_char; 4]>() as c_int;
    ui_browser__refresh_dimensions(browser);
}

unsafe extern "C" fn hist_browser__reset(browser: *mut hist_browser) {
    (*browser).nr_callchain_rows = 0;
    hist_browser__update_nr_entries(browser);
    (*browser).b.nr_entries = hist_browser__nr_entries(browser) as u64;
    hist_browser__refresh_dimensions(addr_of_mut!((*browser).b));
    ui_browser__reset_index(addr_of_mut!((*browser).b));
}

fn tree__folded_sign(unfolded: bool) -> c_char { if unfolded { b'-' as c_char } else { b'+' as c_char } }
unsafe fn hist_entry__folded(he: *const hist_entry) -> c_char { if (*he).has_children { tree__folded_sign((*he).unfolded) } else { b' ' as c_char } }
unsafe fn callchain_list__folded(cl: *const callchain_list) -> c_char { if (*cl).has_children { tree__folded_sign((*cl).unfolded) } else { b' ' as c_char } }
unsafe fn callchain_list__set_folding(cl: *mut callchain_list, unfold: bool) { (*cl).unfolded = if unfold { (*cl).has_children } else { false }; }

unsafe fn for_each_callchain(mut head: *mut list_head, mut f: impl FnMut(*mut callchain_list)) {
    let mut pos = (*head).next;
    while pos != head {
        let chain = pos as *mut callchain_list;
        pos = (*pos).next;
        f(chain);
    }
}

unsafe extern "C" fn callchain_node__count_rows_rb_tree(node: *mut callchain_node) -> c_int {
    let mut n = 0;
    let mut nd = rb_first(addr_of_mut!((*node).rb_root));
    while !nd.is_null() {
        let child = rb_entry_callchain_node(nd);
        let mut folded_sign = b' ' as c_char;
        for_each_callchain(addr_of_mut!((*child).val), |chain| {
            n += 1;
            folded_sign = unsafe { callchain_list__folded(chain) };
        });
        if folded_sign == b'-' as c_char { n += callchain_node__count_rows_rb_tree(child); }
        nd = rb_next(nd);
    }
    n
}

unsafe extern "C" fn callchain_node__count_flat_rows(node: *mut callchain_node) -> c_int {
    let mut folded_sign = 0 as c_char;
    let mut n = 0;
    for_each_callchain(addr_of_mut!((*node).parent_val), |chain| {
        if folded_sign == 0 {
            folded_sign = unsafe { callchain_list__folded(chain) };
            if folded_sign == b'+' as c_char { n = -1; return; }
        }
        if n >= 0 { n += 1; }
    });
    if n < 0 { return 1; }
    for_each_callchain(addr_of_mut!((*node).val), |chain| {
        if folded_sign == 0 {
            folded_sign = unsafe { callchain_list__folded(chain) };
            if folded_sign == b'+' as c_char { n = -1; return; }
        }
        if n >= 0 { n += 1; }
    });
    if n < 0 { 1 } else { n }
}

unsafe extern "C" fn callchain_node__count_folded_rows(_node: *mut callchain_node) -> c_int { 1 }

unsafe extern "C" fn callchain_node__count_rows(node: *mut callchain_node) -> c_int {
    if callchain_param.mode == CHAIN_FLAT { return callchain_node__count_flat_rows(node); }
    if callchain_param.mode == CHAIN_FOLDED { return callchain_node__count_folded_rows(node); }
    let mut unfolded = false;
    let mut n = 0;
    for_each_callchain(addr_of_mut!((*node).val), |chain| { n += 1; unfolded = unsafe { (*chain).unfolded }; });
    if unfolded { n += callchain_node__count_rows_rb_tree(node); }
    n
}

unsafe extern "C" fn callchain__count_rows(chain: *mut rb_root) -> c_int {
    let mut n = 0;
    let mut nd = rb_first(chain);
    while !nd.is_null() {
        n += callchain_node__count_rows(rb_entry_callchain_node(nd));
        nd = rb_next(nd);
    }
    n
}

unsafe extern "C" fn hierarchy_count_rows(hb: *mut hist_browser, he: *mut hist_entry, include_children: bool) -> c_int {
    if (*he).leaf { return callchain__count_rows(addr_of_mut!((*he).sorted_chain)); }
    if (*he).has_no_entry { return 1; }
    let mut count = 0;
    let mut node = rb_first_cached(addr_of_mut!((*he).hroot_out));
    while !node.is_null() {
        let child = rb_entry_hist_entry(node);
        let percent = hist_entry__get_percent_limit(child);
        if !(*child).filtered && percent >= (*hb).min_pcnt as c_float {
            count += 1;
            if include_children && (*child).unfolded { count += hierarchy_count_rows(hb, child, true); }
        }
        node = rb_next(node);
    }
    count
}

unsafe extern "C" fn hist_entry__toggle_fold(he: *mut hist_entry) -> bool {
    if he.is_null() || !(*he).has_children { return false; }
    (*he).unfolded = !(*he).unfolded;
    true
}

unsafe extern "C" fn callchain_list__toggle_fold(cl: *mut callchain_list) -> bool {
    if cl.is_null() || !(*cl).has_children { return false; }
    (*cl).unfolded = !(*cl).unfolded;
    true
}

unsafe extern "C" fn hist_entry__init_have_children(he: *mut hist_entry) {
    if (*he).init_have_children { return; }
    if (*he).leaf {
        (*he).has_children = !RB_EMPTY_ROOT(addr_of_mut!((*he).sorted_chain));
        callchain__init_have_children(addr_of_mut!((*he).sorted_chain));
    } else {
        (*he).has_children = !RB_EMPTY_ROOT(addr_of_mut!((*he).hroot_out.rb_root));
    }
    (*he).init_have_children = true;
}

unsafe extern "C" fn hist_browser__selection_has_children(browser: *mut hist_browser) -> bool {
    let he = (*browser).he_selection;
    let ms = (*browser).selection;
    if he.is_null() || ms.is_null() { return false; }
    if ms == addr_of_mut!((*he).ms) { (*he).has_children } else { (*container_of_callchain_list(ms)).has_children }
}

unsafe extern "C" fn hist_browser__selection_unfolded(browser: *mut hist_browser) -> bool {
    let he = (*browser).he_selection;
    let ms = (*browser).selection;
    if he.is_null() || ms.is_null() { return false; }
    if ms == addr_of_mut!((*he).ms) { (*he).unfolded } else { (*container_of_callchain_list(ms)).unfolded }
}

unsafe extern "C" fn hist_browser__selection_sym_name(browser: *mut hist_browser, bf: *mut c_char, size: size_t) -> *mut c_char {
    let he = (*browser).he_selection;
    let ms = (*browser).selection;
    if he.is_null() || ms.is_null() { return null_mut(); }
    if ms == addr_of_mut!((*he).ms) {
        hist_entry__sym_snprintf(he, bf, size, 0);
        return bf.add(4);
    }
    callchain_list__sym_name(container_of_callchain_list(ms), bf, size, (*browser).show_dso) as *mut c_char
}

unsafe extern "C" fn hist_browser__toggle_fold(browser: *mut hist_browser) -> bool {
    let he = (*browser).he_selection;
    let ms = (*browser).selection;
    if he.is_null() || ms.is_null() { return false; }
    let has_children = if ms == addr_of_mut!((*he).ms) { hist_entry__toggle_fold(he) } else { callchain_list__toggle_fold(container_of_callchain_list(ms)) };
    if has_children {
        let mut child_rows = 0;
        hist_entry__init_have_children(he);
        (*browser).b.nr_entries = (*browser).b.nr_entries.wrapping_sub((*he).nr_rows as u64);
        if (*he).leaf { (*browser).nr_callchain_rows = (*browser).nr_callchain_rows.wrapping_sub((*he).nr_rows as u64); }
        else { (*browser).nr_hierarchy_entries = (*browser).nr_hierarchy_entries.wrapping_sub((*he).nr_rows as u64); }
        if symbol_conf.report_hierarchy { child_rows = hierarchy_count_rows(browser, he, true); }
        if (*he).unfolded {
            (*he).nr_rows = if (*he).leaf { callchain__count_rows(addr_of_mut!((*he).sorted_chain)) as u16 } else { hierarchy_count_rows(browser, he, false) as u16 };
            if symbol_conf.report_hierarchy { (*browser).b.nr_entries = (*browser).b.nr_entries.wrapping_add((child_rows - (*he).nr_rows as c_int) as u64); }
            if !(*he).leaf && (*he).nr_rows == 0 { (*he).has_no_entry = true; (*he).nr_rows = 1; }
        } else {
            if symbol_conf.report_hierarchy { (*browser).b.nr_entries = (*browser).b.nr_entries.wrapping_sub((child_rows - (*he).nr_rows as c_int) as u64); }
            if (*he).has_no_entry { (*he).has_no_entry = false; }
            (*he).nr_rows = 0;
        }
        (*browser).b.nr_entries = (*browser).b.nr_entries.wrapping_add((*he).nr_rows as u64);
        if (*he).leaf { (*browser).nr_callchain_rows = (*browser).nr_callchain_rows.wrapping_add((*he).nr_rows as u64); }
        else { (*browser).nr_hierarchy_entries = (*browser).nr_hierarchy_entries.wrapping_add((*he).nr_rows as u64); }
        return true;
    }
    false
}

unsafe extern "C" fn callchain_node__init_have_children_rb_tree(node: *mut callchain_node) {
    let mut nd = rb_first(addr_of_mut!((*node).rb_root));
    while !nd.is_null() {
        let child = rb_entry_callchain_node(nd);
        let mut first = true;
        for_each_callchain(addr_of_mut!((*child).val), |chain| unsafe {
            if first {
                first = false;
                (*chain).has_children = (*chain).list.next != addr_of_mut!((*child).val) || !RB_EMPTY_ROOT(addr_of_mut!((*child).rb_root));
            } else {
                (*chain).has_children = (*chain).list.next == addr_of_mut!((*child).val) && !RB_EMPTY_ROOT(addr_of_mut!((*child).rb_root));
            }
        });
        callchain_node__init_have_children_rb_tree(child);
        nd = rb_next(nd);
    }
}

unsafe extern "C" fn callchain_node__init_have_children(node: *mut callchain_node, has_sibling: bool) {
    let mut chain = list_first_callchain(addr_of_mut!((*node).val));
    (*chain).has_children = has_sibling;
    if !list_empty(addr_of_mut!((*node).val)) {
        chain = list_last_callchain(addr_of_mut!((*node).val));
        (*chain).has_children = !RB_EMPTY_ROOT(addr_of_mut!((*node).rb_root));
    }
    callchain_node__init_have_children_rb_tree(node);
}

unsafe extern "C" fn callchain__init_have_children(root: *mut rb_root) {
    let first = rb_first(root);
    let has_sibling = !first.is_null() && !rb_next(first).is_null();
    let mut nd = rb_first(root);
    while !nd.is_null() {
        let node = rb_entry_callchain_node(nd);
        callchain_node__init_have_children(node, has_sibling);
        if callchain_param.mode == CHAIN_FLAT || callchain_param.mode == CHAIN_FOLDED {
            callchain_node__make_parent_list(node);
        }
        nd = rb_next(nd);
    }
}

unsafe extern "C" fn callchain_node__set_folding_rb_tree(node: *mut callchain_node, unfold: bool) -> c_int {
    let mut n = 0;
    let mut nd = rb_first(addr_of_mut!((*node).rb_root));
    while !nd.is_null() {
        let child = rb_entry_callchain_node(nd);
        let mut has_children = false;
        for_each_callchain(addr_of_mut!((*child).val), |chain| unsafe {
            n += 1;
            callchain_list__set_folding(chain, unfold);
            has_children = (*chain).has_children;
        });
        if has_children { n += callchain_node__set_folding_rb_tree(child, unfold); }
        nd = rb_next(nd);
    }
    n
}

unsafe extern "C" fn callchain_node__set_folding(node: *mut callchain_node, unfold: bool) -> c_int {
    let mut has_children = false;
    let mut n = 0;
    for_each_callchain(addr_of_mut!((*node).val), |chain| unsafe {
        n += 1;
        callchain_list__set_folding(chain, unfold);
        has_children = (*chain).has_children;
    });
    if has_children { n += callchain_node__set_folding_rb_tree(node, unfold); }
    n
}

unsafe extern "C" fn callchain__set_folding(chain: *mut rb_root, unfold: bool) -> c_int {
    let mut n = 0;
    let mut nd = rb_first(chain);
    while !nd.is_null() {
        n += callchain_node__set_folding(rb_entry_callchain_node(nd), unfold);
        nd = rb_next(nd);
    }
    n
}

unsafe extern "C" fn hierarchy_set_folding(hb: *mut hist_browser, he: *mut hist_entry, _unfold: bool) -> c_int {
    let mut n = 0;
    let mut nd = rb_first_cached(addr_of_mut!((*he).hroot_out));
    while !nd.is_null() {
        let child = rb_entry_hist_entry(nd);
        let percent = hist_entry__get_percent_limit(child);
        if !(*child).filtered && percent >= (*hb).min_pcnt as c_float { n += 1; }
        nd = rb_next(nd);
    }
    n
}

unsafe extern "C" fn hist_entry__set_folding(he: *mut hist_entry, hb: *mut hist_browser, unfold: bool) {
    hist_entry__init_have_children(he);
    (*he).unfolded = if unfold { (*he).has_children } else { false };
    if (*he).has_children {
        let n = if (*he).leaf { callchain__set_folding(addr_of_mut!((*he).sorted_chain), unfold) } else { hierarchy_set_folding(hb, he, unfold) };
        (*he).nr_rows = if unfold { n as u16 } else { 0 };
    } else {
        (*he).nr_rows = 0;
    }
}

unsafe extern "C" fn __hist_browser__set_folding(browser: *mut hist_browser, unfold: bool) {
    let mut nd = rb_first_cached(addr_of_mut!((*(*browser).hists).entries));
    while !nd.is_null() {
        let he = rb_entry_hist_entry(nd);
        nd = __rb_hierarchy_next(nd, HMD_FORCE_CHILD);
        hist_entry__set_folding(he, browser, unfold);
        let percent = hist_entry__get_percent_limit(he) as c_double;
        if (*he).filtered || percent < (*browser).min_pcnt { continue; }
        if (*he).depth == 0 || unfold { (*browser).nr_hierarchy_entries += 1; }
        if (*he).leaf { (*browser).nr_callchain_rows += (*he).nr_rows as u64; }
        else if unfold && !hist_entry__has_hierarchy_children(he, (*browser).min_pcnt) {
            (*browser).nr_hierarchy_entries += 1;
            (*he).has_no_entry = true;
            (*he).nr_rows = 1;
        } else {
            (*he).has_no_entry = false;
        }
    }
}

unsafe extern "C" fn hist_browser__set_folding(browser: *mut hist_browser, unfold: bool) {
    (*browser).nr_hierarchy_entries = 0;
    (*browser).nr_callchain_rows = 0;
    __hist_browser__set_folding(browser, unfold);
    (*browser).b.nr_entries = hist_browser__nr_entries(browser) as u64;
    ui_browser__reset_index(addr_of_mut!((*browser).b));
}

unsafe extern "C" fn hist_browser__set_folding_selected(browser: *mut hist_browser, unfold: bool) {
    if (*browser).he_selection.is_null() || unfold == (*(*browser).he_selection).unfolded { return; }
    hist_browser__toggle_fold(browser);
}

unsafe extern "C" fn ui_browser__warn_lost_events(browser: *mut ui_browser) {
    ui_browser__warning(browser, 4, c"Events are being lost, check IO/CPU overload!\n\nYou may want to run 'perf' using a RT scheduler policy:\n\n perf top -r 80\n\nOr reduce the sampling frequency.".as_ptr());
}

unsafe extern "C" fn hist_browser__title(browser: *mut hist_browser, bf: *mut c_char, size: size_t) -> c_int {
    match (*browser).title { Some(f) => f(browser, bf, size), None => 0 }
}

unsafe extern "C" fn hist_browser__handle_hotkey(browser: *mut hist_browser, warn_lost_event: bool, title: *mut c_char, size: size_t, key: c_int) -> c_int {
    match key {
        K_TIMER => {
            let hbt = (*browser).hbt;
            let evsel = hists_to_evsel((*browser).hists);
            if !hbt.is_null() {
                if let Some(timer) = (*hbt).timer { timer((*hbt).arg); }
            }
            if hist_browser__has_filter(browser) || symbol_conf.report_hierarchy { hist_browser__update_nr_entries(browser); }
            let nr_entries = hist_browser__nr_entries(browser) as u64;
            ui_browser__update_nr_entries(addr_of_mut!((*browser).b), nr_entries);
            if warn_lost_event {
                let stats = evlist__stats((*evsel).evlist);
                if (*stats).nr_lost_warned != (*stats).nr_events[PERF_RECORD_LOST] {
                    (*stats).nr_lost_warned = (*stats).nr_events[PERF_RECORD_LOST];
                    ui_browser__warn_lost_events(addr_of_mut!((*browser).b));
                }
            }
            hist_browser__title(browser, title, size);
            ui_browser__show_title(addr_of_mut!((*browser).b), title);
        }
        x if x == b'D' as c_int => {
            static mut seq: c_int = 0;
            let h = rb_entry_hist_entry((*browser).b.top);
            ui_helpline__pop();
            ui_helpline__fpush(c"%d: nr_ent=(%d,%d), etl: %d, rows=%d, idx=%d, fve: idx=%d, row_off=%d, nrows=%d".as_ptr(), seq, (*browser).b.nr_entries as c_int, (*(*browser).hists).nr_entries, (*browser).b.extra_title_lines as c_int, (*browser).b.rows as c_int, (*browser).b.index as c_int, (*browser).b.top_idx as c_int, (*h).row_offset as c_int, (*h).nr_rows as c_int);
            seq += 1;
        }
        x if x == b'C' as c_int => hist_browser__set_folding(browser, false),
        x if x == b'c' as c_int => hist_browser__set_folding_selected(browser, false),
        x if x == b'E' as c_int => hist_browser__set_folding(browser, true),
        x if x == b'e' as c_int => { hist_browser__toggle_fold(browser); }
        x if x == b'H' as c_int => { (*browser).show_headers = !(*browser).show_headers; hist_browser__update_rows(browser); }
        x if x == b'+' as c_int => { if !hist_browser__toggle_fold(browser) { return -1; } }
        _ => return -1,
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn hist_browser__run(browser: *mut hist_browser, help: *const c_char, warn_lost_event: bool, mut key: c_int) -> c_int {
    let mut title = [0 as c_char; 160];
    let hbt = (*browser).hbt;
    let delay_secs = if !hbt.is_null() { (*hbt).refresh } else { 0 };
    (*browser).b.entries = addr_of_mut!((*(*browser).hists).entries) as *mut c_void;
    (*browser).b.nr_entries = hist_browser__nr_entries(browser) as u64;
    hist_browser__title(browser, title.as_mut_ptr(), title.len());
    if ui_browser__show(addr_of_mut!((*browser).b), title.as_ptr(), c"%s".as_ptr(), help) < 0 { return -1; }
    if key != 0 && hist_browser__handle_hotkey(browser, warn_lost_event, title.as_mut_ptr(), title.len(), key) != 0 { ui_browser__hide(addr_of_mut!((*browser).b)); return key; }
    loop {
        key = ui_browser__run(addr_of_mut!((*browser).b), delay_secs);
        if hist_browser__handle_hotkey(browser, warn_lost_event, title.as_mut_ptr(), title.len(), key) != 0 { break; }
    }
    ui_browser__hide(addr_of_mut!((*browser).b));
    key
}

#[repr(C)] pub struct callchain_print_arg { pub row_offset: off_t, pub is_current_entry: bool, pub fp: *mut FILE, pub printed: c_int }
type print_callchain_entry_fn = unsafe extern "C" fn(*mut hist_browser, *mut callchain_list, *const c_char, c_int, u16, *mut callchain_print_arg);
type check_output_full_fn = unsafe extern "C" fn(*mut hist_browser, u16) -> bool;

unsafe extern "C" fn hist_browser__show_callchain_entry(browser: *mut hist_browser, chain: *mut callchain_list, str_: *const c_char, offset: c_int, row: u16, arg: *mut callchain_print_arg) {
    let folded_sign = callchain_list__folded(chain);
    let show_annotated = (*browser).show_dso && !(*chain).ms.sym.is_null() && !(*symbol__annotation((*chain).ms.sym)).src.is_null();
    let mut color = HE_COLORSET_NORMAL;
    let width = (*browser).b.width - (offset + 2);
    if ui_browser__is_current_entry(addr_of_mut!((*browser).b), row) {
        (*browser).selection = addr_of_mut!((*chain).ms);
        color = HE_COLORSET_SELECTED;
        (*arg).is_current_entry = true;
    }
    ui_browser__set_color(addr_of_mut!((*browser).b), color);
    ui_browser__gotorc(addr_of_mut!((*browser).b), row, 0);
    ui_browser__write_nstring(addr_of_mut!((*browser).b), c" ".as_ptr(), offset);
    ui_browser__printf(addr_of_mut!((*browser).b), c"%c".as_ptr(), folded_sign as c_int);
    ui_browser__write_graph(addr_of_mut!((*browser).b), if show_annotated { SLSMG_RARROW_CHAR } else { b' ' as c_int });
    ui_browser__write_nstring(addr_of_mut!((*browser).b), str_, width);
}

unsafe extern "C" fn hist_browser__fprintf_callchain_entry(_b: *mut hist_browser, chain: *mut callchain_list, str_: *const c_char, offset: c_int, _row: u16, arg: *mut callchain_print_arg) {
    let folded_sign = callchain_list__folded(chain);
    (*arg).printed += fprintf((*arg).fp, c"%*s%c %s\n".as_ptr(), offset, c" ".as_ptr(), folded_sign as c_int, str_);
}

unsafe extern "C" fn hist_browser__check_output_full(browser: *mut hist_browser, row: u16) -> bool { (*browser).b.rows == row }
unsafe extern "C" fn hist_browser__check_dump_full(_browser: *mut hist_browser, _row: u16) -> bool { false }

unsafe extern "C" fn hist_browser__show_callchain_list(browser: *mut hist_browser, node: *mut callchain_node, chain: *mut callchain_list, row: u16, total: u64, need_percent: bool, offset: c_int, print: print_callchain_entry_fn, arg: *mut callchain_print_arg) -> c_int {
    let mut bf = [0 as c_char; 1024];
    let mut buf = [0 as c_char; 64];
    let mut alloc_str: *mut c_char = null_mut();
    let mut alloc_str2: *mut c_char = null_mut();
    if (*arg).row_offset != 0 { (*arg).row_offset -= 1; return 0; }
    let mut str_ = callchain_list__sym_name(chain, bf.as_mut_ptr(), bf.len(), (*browser).show_dso);
    if symbol_conf.show_branchflag_count {
        callchain_list_counts__printf_value(chain, null_mut(), buf.as_mut_ptr(), buf.len());
        if asprintf(addr_of_mut!(alloc_str2), c"%s%s".as_ptr(), str_, buf.as_ptr()) < 0 { str_ = c"Not enough memory!".as_ptr(); } else { str_ = alloc_str2; }
    }
    if need_percent {
        callchain_node__scnprintf_value(node, buf.as_mut_ptr(), buf.len(), total);
        if asprintf(addr_of_mut!(alloc_str), c"%s %s".as_ptr(), buf.as_ptr(), str_) < 0 { str_ = c"Not enough memory!".as_ptr(); } else { str_ = alloc_str; }
    }
    print(browser, chain, str_, offset, row, arg);
    free(alloc_str as *mut c_void);
    free(alloc_str2 as *mut c_void);
    1
}

unsafe extern "C" fn check_percent_display(node: *mut rb_node, parent_total: u64) -> bool {
    if node.is_null() { return false; }
    if !rb_next(node).is_null() { return true; }
    callchain_cumul_hits(rb_entry_callchain_node(node)) != parent_total
}

unsafe extern "C" fn hist_browser__show_callchain_graph(browser: *mut hist_browser, root: *mut rb_root, level: c_int, mut row: u16, total: u64, parent_total: u64, print: print_callchain_entry_fn, arg: *mut callchain_print_arg, is_output_full: check_output_full_fn) -> c_int {
    let first_row = row;
    let offset = level * LEVEL_OFFSET_STEP;
    let percent_total = if callchain_param.mode == CHAIN_GRAPH_REL { parent_total } else { total };
    let mut node = rb_first(root);
    let need_percent = check_percent_display(node, parent_total);
    while !node.is_null() {
        let child = rb_entry_callchain_node(node);
        let next = rb_next(node);
        let mut folded_sign = b' ' as c_char;
        let mut first = true;
        let mut extra_offset = 0;
        for_each_callchain(addr_of_mut!((*child).val), |chain| unsafe {
            let was_first = first;
            if first { first = false; } else if need_percent { extra_offset = LEVEL_OFFSET_STEP; }
            folded_sign = callchain_list__folded(chain);
            row = row.wrapping_add(hist_browser__show_callchain_list(browser, child, chain, row, percent_total, was_first && need_percent, offset + extra_offset, print, arg) as u16);
        });
        if is_output_full(browser, row) { break; }
        if folded_sign == b'-' as c_char {
            let new_level = level + if extra_offset != 0 { 2 } else { 1 };
            row = row.wrapping_add(hist_browser__show_callchain_graph(browser, addr_of_mut!((*child).rb_root), new_level, row, total, (*child).children_hit, print, arg, is_output_full) as u16);
        }
        if is_output_full(browser, row) { break; }
        node = next;
    }
    row.wrapping_sub(first_row) as c_int
}

unsafe extern "C" fn hist_browser__show_callchain_flat(browser: *mut hist_browser, root: *mut rb_root, row: u16, total: u64, parent_total: u64, print: print_callchain_entry_fn, arg: *mut callchain_print_arg, is_output_full: check_output_full_fn) -> c_int {
    hist_browser__show_callchain_graph(browser, root, 1, row, total, parent_total, print, arg, is_output_full)
}

unsafe extern "C" fn hist_browser__folded_callchain_str(browser: *mut hist_browser, chain: *mut callchain_list, value_str: *mut c_char, old_str: *mut c_char) -> *mut c_char {
    let mut bf = [0 as c_char; 1024];
    let str_ = callchain_list__sym_name(chain, bf.as_mut_ptr(), bf.len(), (*browser).show_dso);
    let mut newp: *mut c_char = null_mut();
    if !old_str.is_null() {
        let sep = if !symbol_conf.field_sep.is_null() { symbol_conf.field_sep } else { c";".as_ptr() };
        if asprintf(addr_of_mut!(newp), c"%s%s%s".as_ptr(), old_str, sep, str_) < 0 { newp = null_mut(); }
    } else if !value_str.is_null() {
        if asprintf(addr_of_mut!(newp), c"%s %s".as_ptr(), value_str, str_) < 0 { newp = null_mut(); }
    } else if asprintf(addr_of_mut!(newp), c"%s".as_ptr(), str_) < 0 { newp = null_mut(); }
    newp
}

unsafe extern "C" fn hist_browser__show_callchain_folded(browser: *mut hist_browser, root: *mut rb_root, mut row: u16, total: u64, parent_total: u64, print: print_callchain_entry_fn, arg: *mut callchain_print_arg, is_output_full: check_output_full_fn) -> c_int {
    let first_row = row;
    let offset = LEVEL_OFFSET_STEP;
    let mut node = rb_first(root);
    let need_percent = check_percent_display(node, parent_total);
    while !node.is_null() {
        let child = rb_entry_callchain_node(node);
        let next = rb_next(node);
        let mut first_chain: *mut callchain_list = null_mut();
        let mut first = true;
        let mut value_str: *mut c_char = null_mut();
        let mut value_str_alloc: *mut c_char = null_mut();
        let mut chain_str: *mut c_char = null_mut();
        let mut chain_str_alloc: *mut c_char = null_mut();
        if (*arg).row_offset != 0 { (*arg).row_offset -= 1; node = next; continue; }
        if need_percent {
            let mut buf = [0 as c_char; 64];
            callchain_node__scnprintf_value(child, buf.as_mut_ptr(), buf.len(), total);
            if asprintf(addr_of_mut!(value_str), c"%s".as_ptr(), buf.as_ptr()) < 0 { value_str = c"<...>".as_ptr() as *mut c_char; } else { value_str_alloc = value_str; }
        }
        for_each_callchain(addr_of_mut!((*child).parent_val), |chain| unsafe {
            chain_str = hist_browser__folded_callchain_str(browser, chain, value_str, chain_str);
            if first { first = false; first_chain = chain; }
            if chain_str.is_null() { chain_str = c"Not enough memory!".as_ptr() as *mut c_char; } else { chain_str_alloc = chain_str; }
        });
        for_each_callchain(addr_of_mut!((*child).val), |chain| unsafe {
            chain_str = hist_browser__folded_callchain_str(browser, chain, value_str, chain_str);
            if first { first = false; first_chain = chain; }
            if chain_str.is_null() { chain_str = c"Not enough memory!".as_ptr() as *mut c_char; } else { chain_str_alloc = chain_str; }
        });
        print(browser, first_chain, chain_str, offset, row, arg);
        row = row.wrapping_add(1);
        free(value_str_alloc as *mut c_void);
        free(chain_str_alloc as *mut c_void);
        if is_output_full(browser, row) { break; }
        node = next;
    }
    row.wrapping_sub(first_row) as c_int
}

unsafe extern "C" fn hist_browser__show_callchain(browser: *mut hist_browser, entry: *mut hist_entry, level: c_int, row: u16, print: print_callchain_entry_fn, arg: *mut callchain_print_arg, is_output_full: check_output_full_fn) -> c_int {
    let total = hists__total_period((*entry).hists);
    let parent_total = if symbol_conf.cumulate_callchain { (*(*entry).stat_acc).period } else { (*entry).stat.period };
    let printed = if callchain_param.mode == CHAIN_FLAT {
        hist_browser__show_callchain_flat(browser, addr_of_mut!((*entry).sorted_chain), row, total, parent_total, print, arg, is_output_full)
    } else if callchain_param.mode == CHAIN_FOLDED {
        hist_browser__show_callchain_folded(browser, addr_of_mut!((*entry).sorted_chain), row, total, parent_total, print, arg, is_output_full)
    } else {
        hist_browser__show_callchain_graph(browser, addr_of_mut!((*entry).sorted_chain), level, row, total, parent_total, print, arg, is_output_full)
    };
    if (*arg).is_current_entry { (*browser).he_selection = entry; }
    printed
}

#[repr(C)] pub struct hpp_arg { pub b: *mut ui_browser, pub folded_sign: c_char, pub current_entry: bool }

#[no_mangle]
pub unsafe extern "C" fn __hpp__slsmg_color_printf(hpp: *mut perf_hpp, fmt: *const c_char, mut _args: ...) -> c_int {
    let arg = (*hpp).ptr as *mut hpp_arg;
    ui_browser__set_percent_color((*arg).b, 0.0, (*arg).current_entry);
    let ret = scnprintf((*hpp).buf, (*hpp).size, fmt);
    ui_browser__printf((*arg).b, c"%s".as_ptr(), (*hpp).buf);
    ret
}

macro_rules! hpp_percent_fns {
    ($get:ident, $fun:ident, $field:ident, $typ:expr) => {
        unsafe extern "C" fn $get(he: *mut hist_entry) -> u64 { (*he).stat.$field }
        unsafe extern "C" fn $fun(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
            hpp__fmt(fmt, hpp, he, $get, c" %*.2f%%".as_ptr(), __hpp__slsmg_color_printf, $typ)
        }
    }
}
macro_rules! hpp_acc_percent_fns {
    ($get:ident, $fun:ident, $field:ident, $typ:expr) => {
        unsafe extern "C" fn $get(he: *mut hist_entry) -> u64 { (*(*he).stat_acc).$field }
        unsafe extern "C" fn $fun(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
            if !symbol_conf.cumulate_callchain {
                let arg = (*hpp).ptr as *mut hpp_arg;
                let len = if (*fmt).user_len != 0 { (*fmt).user_len } else { (*fmt).len };
                let ret = scnprintf((*hpp).buf, (*hpp).size, c"%*s".as_ptr(), len, c"N/A".as_ptr());
                ui_browser__printf((*arg).b, c"%s".as_ptr(), (*hpp).buf);
                return ret;
            }
            hpp__fmt(fmt, hpp, he, $get, c" %*.2f%%".as_ptr(), __hpp__slsmg_color_printf, $typ)
        }
    }
}
macro_rules! hpp_mem_stat_fns {
    ($fun:ident, $typ:expr) => {
        unsafe extern "C" fn $fun(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
            hpp__fmt_mem_stat(fmt, hpp, he, $typ, c" %5.1f%%".as_ptr(), __hpp__slsmg_color_printf)
        }
    }
}
hpp_percent_fns!(__hpp_get_period, hist_browser__hpp_color_overhead, period, PERF_HPP_FMT_TYPE__PERCENT);
hpp_percent_fns!(__hpp_get_latency, hist_browser__hpp_color_latency, latency, PERF_HPP_FMT_TYPE__LATENCY);
hpp_percent_fns!(__hpp_get_period_sys, hist_browser__hpp_color_overhead_sys, period_sys, PERF_HPP_FMT_TYPE__PERCENT);
hpp_percent_fns!(__hpp_get_period_us, hist_browser__hpp_color_overhead_us, period_us, PERF_HPP_FMT_TYPE__PERCENT);
hpp_percent_fns!(__hpp_get_period_guest_sys, hist_browser__hpp_color_overhead_guest_sys, period_guest_sys, PERF_HPP_FMT_TYPE__PERCENT);
hpp_percent_fns!(__hpp_get_period_guest_us, hist_browser__hpp_color_overhead_guest_us, period_guest_us, PERF_HPP_FMT_TYPE__PERCENT);
hpp_acc_percent_fns!(__hpp_get_acc_period, hist_browser__hpp_color_overhead_acc, period, PERF_HPP_FMT_TYPE__PERCENT);
hpp_acc_percent_fns!(__hpp_get_acc_latency, hist_browser__hpp_color_latency_acc, latency, PERF_HPP_FMT_TYPE__LATENCY);
hpp_mem_stat_fns!(hist_browser__hpp_color_mem_stat_op, PERF_MEM_STAT_OP);
hpp_mem_stat_fns!(hist_browser__hpp_color_mem_stat_cache, PERF_MEM_STAT_CACHE);
hpp_mem_stat_fns!(hist_browser__hpp_color_mem_stat_memory, PERF_MEM_STAT_MEMORY);
hpp_mem_stat_fns!(hist_browser__hpp_color_mem_stat_snoop, PERF_MEM_STAT_SNOOP);
hpp_mem_stat_fns!(hist_browser__hpp_color_mem_stat_dtlb, PERF_MEM_STAT_DTLB);

#[no_mangle]
pub unsafe extern "C" fn hist_browser__init_hpp() {
    perf_hpp__format[PERF_HPP__OVERHEAD].color = Some(hist_browser__hpp_color_overhead);
    perf_hpp__format[PERF_HPP__LATENCY].color = Some(hist_browser__hpp_color_latency);
    perf_hpp__format[PERF_HPP__OVERHEAD_SYS].color = Some(hist_browser__hpp_color_overhead_sys);
    perf_hpp__format[PERF_HPP__OVERHEAD_US].color = Some(hist_browser__hpp_color_overhead_us);
    perf_hpp__format[PERF_HPP__OVERHEAD_GUEST_SYS].color = Some(hist_browser__hpp_color_overhead_guest_sys);
    perf_hpp__format[PERF_HPP__OVERHEAD_GUEST_US].color = Some(hist_browser__hpp_color_overhead_guest_us);
    perf_hpp__format[PERF_HPP__OVERHEAD_ACC].color = Some(hist_browser__hpp_color_overhead_acc);
    perf_hpp__format[PERF_HPP__LATENCY_ACC].color = Some(hist_browser__hpp_color_latency_acc);
    perf_hpp__format[PERF_HPP__MEM_STAT_OP].color = Some(hist_browser__hpp_color_mem_stat_op);
    perf_hpp__format[PERF_HPP__MEM_STAT_CACHE].color = Some(hist_browser__hpp_color_mem_stat_cache);
    perf_hpp__format[PERF_HPP__MEM_STAT_MEMORY].color = Some(hist_browser__hpp_color_mem_stat_memory);
    perf_hpp__format[PERF_HPP__MEM_STAT_SNOOP].color = Some(hist_browser__hpp_color_mem_stat_snoop);
    perf_hpp__format[PERF_HPP__MEM_STAT_DTLB].color = Some(hist_browser__hpp_color_mem_stat_dtlb);
    res_sample_init();
}

unsafe extern "C" fn hists__filter_entries(mut nd: *mut rb_node, min_pcnt: c_float) -> *mut rb_node {
    while !nd.is_null() {
        let h = rb_entry_hist_entry(nd);
        let percent = hist_entry__get_percent_limit(h);
        if !(*h).filtered && percent >= min_pcnt { return nd; }
        nd = if !rb_next(nd).is_null() { rb_next(nd) } else { rb_hierarchy_next(nd) };
    }
    null_mut()
}

unsafe extern "C" fn hists__filter_prev_entries(mut nd: *mut rb_node, min_pcnt: c_float) -> *mut rb_node {
    while !nd.is_null() {
        let h = rb_entry_hist_entry(nd);
        let percent = hist_entry__get_percent_limit(h);
        if !(*h).filtered && percent >= min_pcnt { return nd; }
        nd = rb_hierarchy_prev(nd);
    }
    null_mut()
}

unsafe extern "C" fn ui_browser__hists_init_top(browser: *mut ui_browser) {
    if (*browser).top.is_null() {
        let hb = container_of_hist_browser(browser);
        (*browser).top = rb_first_cached(addr_of_mut!((*(*hb).hists).entries));
    }
}

unsafe extern "C" fn ui_browser__hists_seek(browser: *mut ui_browser, mut offset: off_t, whence: c_int) {
    let hb = container_of_hist_browser(browser);
    if (*browser).nr_entries == 0 { return; }
    ui_browser__hists_init_top(browser);
    let mut first = true;
    let mut nd = match whence {
        SEEK_SET => hists__filter_entries(rb_first((*browser).entries as *mut rb_root), (*hb).min_pcnt as c_float),
        SEEK_CUR => (*browser).top,
        SEEK_END => { first = false; hists__filter_prev_entries(rb_hierarchy_last(rb_last((*browser).entries as *mut rb_root)), (*hb).min_pcnt as c_float) }
        _ => return,
    };
    if !(*browser).top.is_null() { (*rb_entry_hist_entry((*browser).top)).row_offset = 0; }
    if nd.is_null() { return; }
    if offset > 0 {
        while offset != 0 {
            let h = rb_entry_hist_entry(nd);
            if (*h).unfolded && (*h).leaf {
                let remaining = (*h).nr_rows as off_t - (*h).row_offset;
                if offset > remaining { offset -= remaining; (*h).row_offset = 0; }
                else { (*h).row_offset += offset; offset = 0; (*browser).top = nd; break; }
            }
            nd = hists__filter_entries(rb_hierarchy_next(nd), (*hb).min_pcnt as c_float);
            if nd.is_null() { break; }
            offset -= 1;
            (*browser).top = nd;
        }
    } else if offset < 0 {
        loop {
            let h = rb_entry_hist_entry(nd);
            if (*h).unfolded && (*h).leaf {
                if first {
                    if -offset > (*h).row_offset { offset += (*h).row_offset; (*h).row_offset = 0; }
                    else { (*h).row_offset += offset; offset = 0; (*browser).top = nd; break; }
                } else if -offset > (*h).nr_rows as off_t { offset += (*h).nr_rows as off_t; (*h).row_offset = 0; }
                else { (*h).row_offset = (*h).nr_rows as off_t + offset; offset = 0; (*browser).top = nd; break; }
            }
            nd = hists__filter_prev_entries(rb_hierarchy_prev(nd), (*hb).min_pcnt as c_float);
            if nd.is_null() { break; }
            offset += 1;
            (*browser).top = nd;
            if offset == 0 {
                let h = rb_entry_hist_entry(nd);
                if (*h).unfolded && (*h).leaf { (*h).row_offset = (*h).nr_rows as off_t; }
                break;
            }
            first = false;
        }
    } else {
        (*browser).top = nd;
        (*rb_entry_hist_entry(nd)).row_offset = 0;
    }
}

unsafe extern "C" fn hist_browser__show_entry(browser: *mut hist_browser, entry: *mut hist_entry, mut row: u16) -> c_int {
    let current_entry = ui_browser__is_current_entry(addr_of_mut!((*browser).b), row);
    let use_callchain = hist_entry__has_callchains(entry) && symbol_conf.use_callchain;
    let mut folded_sign = b' ' as c_char;
    if current_entry { (*browser).he_selection = entry; (*browser).selection = addr_of_mut!((*entry).ms); }
    if use_callchain { hist_entry__init_have_children(entry); folded_sign = hist_entry__folded(entry); }
    let mut printed = 0;
    if (*entry).row_offset == 0 {
        ui_browser__gotorc(addr_of_mut!((*browser).b), row, 0);
        ui_browser__printf(addr_of_mut!((*browser).b), c"%c ".as_ptr(), folded_sign as c_int);
        ui_browser__write_nstring(addr_of_mut!((*browser).b), c"".as_ptr(), (*browser).b.width);
        row = row.wrapping_add(1);
        printed += 1;
    }
    if folded_sign == b'-' as c_char && row != (*browser).b.rows {
        let mut arg = callchain_print_arg { row_offset: (*entry).row_offset, is_current_entry: current_entry, fp: null_mut(), printed: 0 };
        printed += hist_browser__show_callchain(browser, entry, 1, row, hist_browser__show_callchain_entry, addr_of_mut!(arg), hist_browser__check_output_full);
    }
    printed
}

unsafe extern "C" fn hist_browser__show_hierarchy_entry(browser: *mut hist_browser, entry: *mut hist_entry, row: u16, level: c_int) -> c_int {
    hist_entry__init_have_children(entry);
    if ui_browser__is_current_entry(addr_of_mut!((*browser).b), row) { (*browser).he_selection = entry; (*browser).selection = addr_of_mut!((*entry).ms); }
    ui_browser__gotorc(addr_of_mut!((*browser).b), row, 0);
    ui_browser__write_nstring(addr_of_mut!((*browser).b), c"".as_ptr(), level * HIERARCHY_INDENT);
    ui_browser__printf(addr_of_mut!((*browser).b), c"%c ".as_ptr(), hist_entry__folded(entry) as c_int);
    ui_browser__write_nstring(addr_of_mut!((*browser).b), c"".as_ptr(), (*browser).b.width);
    1
}

unsafe extern "C" fn hist_browser__show_no_entry(browser: *mut hist_browser, row: u16, level: c_int) -> c_int {
    if ui_browser__is_current_entry(addr_of_mut!((*browser).b), row) { (*browser).he_selection = null_mut(); (*browser).selection = null_mut(); }
    ui_browser__gotorc(addr_of_mut!((*browser).b), row, 0);
    ui_browser__write_nstring(addr_of_mut!((*browser).b), c"".as_ptr(), level * HIERARCHY_INDENT);
    ui_browser__printf(addr_of_mut!((*browser).b), c"  no entry >= %.2f%%".as_ptr(), (*browser).min_pcnt);
    1
}

unsafe extern "C" fn advance_hpp_check(hpp: *mut perf_hpp, inc: c_int) -> c_int {
    advance_hpp(hpp, inc);
    ((*hpp).size <= 0) as c_int
}

unsafe extern "C" fn hists_browser__scnprintf_headers(browser: *mut hist_browser, buf: *mut c_char, size: size_t, line: c_int) -> c_int {
    let hists = (*browser).hists;
    let mut dummy_hpp = perf_hpp { buf, size, ptr: null_mut() };
    let mut ret: size_t = 0;
    if hists__has_callchains(hists) && symbol_conf.use_callchain {
        ret = scnprintf(buf, size, c"  ".as_ptr()) as size_t;
        if advance_hpp_check(addr_of_mut!(dummy_hpp), ret as c_int) != 0 { return ret as c_int; }
    }
    ret as c_int
}

unsafe extern "C" fn hists_browser__scnprintf_hierarchy_headers(browser: *mut hist_browser, buf: *mut c_char, size: size_t, line: c_int) -> c_int {
    let mut dummy_hpp = perf_hpp { buf, size, ptr: null_mut() };
    let ret = scnprintf(buf, size, c"  ".as_ptr());
    advance_hpp_check(addr_of_mut!(dummy_hpp), ret);
    ret
}

unsafe extern "C" fn hist_browser__show_headers(browser: *mut hist_browser) {
    let mut headers = [0 as c_char; 1024];
    let hpp_list = (*(*browser).hists).hpp_list;
    let mut line = 0;
    while line < (*hpp_list).nr_header_lines {
        if symbol_conf.report_hierarchy { hists_browser__scnprintf_hierarchy_headers(browser, headers.as_mut_ptr(), headers.len(), line); }
        else { hists_browser__scnprintf_headers(browser, headers.as_mut_ptr(), headers.len(), line); }
        ui_browser__gotorc_title(addr_of_mut!((*browser).b), line, 0);
        ui_browser__set_color(addr_of_mut!((*browser).b), HE_COLORSET_ROOT);
        ui_browser__write_nstring(addr_of_mut!((*browser).b), headers.as_ptr(), (*browser).b.width + 1);
        line += 1;
    }
}

unsafe extern "C" fn hist_browser__refresh(browser: *mut ui_browser) -> c_uint {
    let hb = container_of_hist_browser(browser);
    let mut row: u16 = 0;
    if (*hb).show_headers { hist_browser__show_headers(hb); }
    ui_browser__hists_init_top(browser);
    (*hb).he_selection = null_mut();
    (*hb).selection = null_mut();
    let mut nd = (*browser).top;
    while !nd.is_null() {
        let h = rb_entry_hist_entry(nd);
        if (*h).filtered { (*h).unfolded = false; nd = rb_hierarchy_next(nd); continue; }
        let percent = if symbol_conf.report_individual_block { block_info__total_cycles_percent(h) } else { hist_entry__get_percent_limit(h) };
        if percent >= (*hb).min_pcnt as c_float {
            if symbol_conf.report_hierarchy {
                row = row.wrapping_add(hist_browser__show_hierarchy_entry(hb, h, row, (*h).depth) as u16);
                if row == (*browser).rows { break; }
                if (*h).has_no_entry { hist_browser__show_no_entry(hb, row, (*h).depth + 1); row = row.wrapping_add(1); }
            } else {
                row = row.wrapping_add(hist_browser__show_entry(hb, h, row) as u16);
            }
        }
        if row == (*browser).rows { break; }
        nd = rb_hierarchy_next(nd);
    }
    row as c_uint
}

unsafe extern "C" fn hist_browser__fprintf_callchain(browser: *mut hist_browser, he: *mut hist_entry, fp: *mut FILE, level: c_int) -> c_int {
    let mut arg = callchain_print_arg { row_offset: 0, is_current_entry: false, fp, printed: 0 };
    hist_browser__show_callchain(browser, he, level, 0, hist_browser__fprintf_callchain_entry, addr_of_mut!(arg), hist_browser__check_dump_full);
    arg.printed
}

unsafe extern "C" fn hist_browser__fprintf_entry(browser: *mut hist_browser, he: *mut hist_entry, fp: *mut FILE) -> c_int {
    let folded_sign = if hist_entry__has_callchains(he) && symbol_conf.use_callchain { hist_entry__folded(he) } else { b' ' as c_char };
    let mut printed = 0;
    if folded_sign != b' ' as c_char { printed += fprintf(fp, c"%c ".as_ptr(), folded_sign as c_int); }
    printed += fprintf(fp, c"\n".as_ptr());
    if folded_sign == b'-' as c_char { printed += hist_browser__fprintf_callchain(browser, he, fp, 1); }
    printed
}

unsafe extern "C" fn hist_browser__fprintf_hierarchy_entry(browser: *mut hist_browser, he: *mut hist_entry, fp: *mut FILE, level: c_int) -> c_int {
    let mut printed = fprintf(fp, c"%*s%c\n".as_ptr(), level * HIERARCHY_INDENT, c"".as_ptr(), hist_entry__folded(he) as c_int);
    if (*he).leaf && hist_entry__folded(he) == b'-' as c_char { printed += hist_browser__fprintf_callchain(browser, he, fp, (*he).depth + 1); }
    printed
}

unsafe extern "C" fn hist_browser__fprintf(browser: *mut hist_browser, fp: *mut FILE) -> c_int {
    let mut nd = hists__filter_entries(rb_first((*browser).b.entries as *mut rb_root), (*browser).min_pcnt as c_float);
    let mut printed = 0;
    while !nd.is_null() {
        let h = rb_entry_hist_entry(nd);
        printed += if symbol_conf.report_hierarchy { hist_browser__fprintf_hierarchy_entry(browser, h, fp, (*h).depth) } else { hist_browser__fprintf_entry(browser, h, fp) };
        nd = hists__filter_entries(rb_hierarchy_next(nd), (*browser).min_pcnt as c_float);
    }
    printed
}

unsafe extern "C" fn hist_browser__dump(browser: *mut hist_browser) -> c_int {
    let mut filename = [0 as c_char; 64];
    loop {
        scnprintf(filename.as_mut_ptr(), filename.len(), c"perf.hist.%d".as_ptr(), (*browser).print_seq);
        if access(filename.as_ptr(), F_OK) != 0 { break; }
        (*browser).print_seq += 1;
        if (*browser).print_seq == 8192 {
            ui_helpline__fpush(c"Too many perf.hist.N files, nothing written!".as_ptr());
            return -1;
        }
    }
    let fp = fopen(filename.as_ptr(), c"w".as_ptr());
    if fp.is_null() {
        let mut bf = [0 as c_char; 64];
        let err = str_error_r(errno, bf.as_mut_ptr(), bf.len());
        ui_helpline__fpush(c"Couldn't write to %s: %s".as_ptr(), filename.as_ptr(), err);
        return -1;
    }
    (*browser).print_seq += 1;
    hist_browser__fprintf(browser, fp);
    fclose(fp);
    ui_helpline__fpush(c"%s written!".as_ptr(), filename.as_ptr());
    0
}

#[no_mangle]
pub unsafe extern "C" fn hist_browser__init(browser: *mut hist_browser, hists: *mut hists) {
    (*browser).hists = hists;
    (*browser).b.refresh = Some(hist_browser__refresh);
    (*browser).b.refresh_dimensions = Some(hist_browser__refresh_dimensions);
    (*browser).b.seek = Some(ui_browser__hists_seek);
    (*browser).b.use_navkeypressed = true;
    (*browser).show_headers = symbol_conf.show_hist_headers;
    hist_browser__set_title_space(browser);
    hists__reset_column_width(hists);
}

#[no_mangle]
pub unsafe extern "C" fn hist_browser__new(hists: *mut hists) -> *mut hist_browser {
    let browser = zalloc(size_of::<hist_browser>()) as *mut hist_browser;
    if !browser.is_null() { hist_browser__init(browser, hists); }
    browser
}

unsafe extern "C" fn perf_evsel_browser__new(evsel: *mut evsel, hbt: *mut hist_browser_timer, env: *mut perf_env) -> *mut hist_browser {
    let browser = hist_browser__new(evsel__hists(evsel));
    if !browser.is_null() { (*browser).hbt = hbt; (*browser).env = env; (*browser).title = Some(hists_browser__scnprintf_title); }
    browser
}

#[no_mangle]
pub unsafe extern "C" fn hist_browser__delete(browser: *mut hist_browser) { free(browser as *mut c_void); }

unsafe extern "C" fn hist_browser__selected_entry(browser: *mut hist_browser) -> *mut hist_entry { (*browser).he_selection }
unsafe extern "C" fn hist_browser__selected_thread(browser: *mut hist_browser) -> *mut thread { (*(*browser).he_selection).thread }
unsafe extern "C" fn hist_browser__selected_res_sample(browser: *mut hist_browser) -> *mut res_sample { if !(*browser).he_selection.is_null() { (*(*browser).he_selection).res_samples } else { null_mut() } }
unsafe fn is_report_browser(timer: *mut c_void) -> bool { timer.is_null() }

unsafe extern "C" fn hists_browser__scnprintf_title(browser: *mut hist_browser, bf: *mut c_char, size: size_t) -> c_int {
    let hbt = (*browser).hbt;
    let mut printed = __hists__scnprintf_title((*browser).hists, bf, size, !is_report_browser(hbt as *mut c_void));
    if !is_report_browser(hbt as *mut c_void) {
        let top = (*hbt).arg as *mut perf_top;
        printed += scnprintf(bf.add(printed as usize), size - printed as usize, c" lost: %lu/%lu".as_ptr(), (*top).lost, (*top).lost_total);
        printed += scnprintf(bf.add(printed as usize), size - printed as usize, c" drop: %lu/%lu".as_ptr(), (*top).drop, (*top).drop_total);
        if (*top).zero { printed += scnprintf(bf.add(printed as usize), size - printed as usize, c" [z]".as_ptr()); }
        perf_top__reset_sample_counters(top);
    }
    printed
}

#[repr(C)] pub struct popup_action {
    pub time: c_ulong,
    pub fn_: Option<unsafe extern "C" fn(*mut hist_browser, *mut popup_action) -> c_int>,
    pub ms: map_symbol,
    pub socket: c_int,
    pub rstype: rstype,
}

unsafe fn free_popup_options(options: *mut *mut c_char, n: c_int) {
    for i in 0..n as isize { zfree_char(options.offset(i)); }
}

unsafe fn free_popup_actions(actions: *mut popup_action, n: c_int) {
    for i in 0..n as isize {
        map_symbol__exit(addr_of_mut!((*actions.offset(i)).ms));
        memset(actions.offset(i) as *mut c_void, 0, size_of::<popup_action>());
    }
}

static mut is_input_name_malloced: bool = false;

unsafe extern "C" fn switch_data_file() -> c_int {
    let pwd = getenv(c"PWD".as_ptr());
    if pwd.is_null() { return -1; }
    let pwd_dir = opendir(pwd);
    if pwd_dir.is_null() { return -1; }
    let mut options: [*mut c_char; 32] = [null_mut(); 32];
    let mut abs_path: [*mut c_char; 32] = [null_mut(); 32];
    let mut nr_options = 0;
    let mut ret = -1;
    loop {
        let dent = readdir(pwd_dir);
        if dent.is_null() { break; }
        if (*dent).d_type != DT_REG { continue; }
        let mut path = [0 as c_char; 4096];
        let name = (*dent).d_name.as_mut_ptr();
        snprintf(path.as_mut_ptr(), path.len(), c"%s/%s".as_ptr(), pwd, name);
        let file = fopen(path.as_ptr(), c"r".as_ptr());
        if file.is_null() { continue; }
        let mut magic: u64 = 0;
        if fread(addr_of_mut!(magic) as *mut c_void, 1, 8, file) >= 8 && is_perf_magic(magic) {
            options[nr_options] = strdup(name);
            abs_path[nr_options] = strdup(path.as_ptr());
            nr_options += 1;
        }
        fclose(file);
        if nr_options >= 32 { ui__warning(c"Too many perf data files in PWD!\nOnly the first 32 files will be listed.\n".as_ptr()); break; }
    }
    closedir(pwd_dir);
    if nr_options != 0 {
        let choice = ui__popup_menu(nr_options as c_int, options.as_mut_ptr(), null_mut());
        if choice < nr_options as c_int && choice >= 0 {
            let tmp = strdup(abs_path[choice as usize]);
            if !tmp.is_null() {
                if is_input_name_malloced { free(input_name as *mut c_void); }
                input_name = tmp;
                is_input_name_malloced = true;
                ret = 0;
            } else { ui__warning(c"Data switch failed due to memory shortage!\n".as_ptr()); }
        }
    }
    free_popup_options(options.as_mut_ptr(), nr_options as c_int);
    free_popup_options(abs_path.as_mut_ptr(), nr_options as c_int);
    ret
}

unsafe extern "C" fn do_annotate(browser: *mut hist_browser, act: *mut popup_action) -> c_int {
    if annotate_opts.objdump_path.is_null() && perf_env__lookup_objdump((*browser).env, addr_of_mut!(annotate_opts.objdump_path)) != 0 { return 0; }
    let notes = symbol__annotation((*act).ms.sym);
    if (*notes).src.is_null() { return 0; }
    let evsel = if !(*browser).block_evsel.is_null() { (*browser).block_evsel } else { hists_to_evsel((*browser).hists) };
    let he = hist_browser__selected_entry(browser);
    let err = __hist_entry__tui_annotate(he, addr_of_mut!((*act).ms), evsel, (*browser).hbt, NO_ADDR);
    if (err == b'q' as c_int || err == CTRL(b'c' as c_int)) && !(*he).branch_info.is_null() { return 1; }
    ui_browser__update_nr_entries(addr_of_mut!((*browser).b), (*(*browser).hists).nr_entries as u64);
    if err != 0 { ui_browser__handle_resize(addr_of_mut!((*browser).b)); }
    0
}

unsafe extern "C" fn symbol__new_unresolved(addr: u64, map: *mut map) -> *mut symbol {
    let mut name = [0 as c_char; 64];
    snprintf(name.as_mut_ptr(), name.len(), c"%.*lx".as_ptr(), BITS_PER_LONG / 4, addr);
    let sym = symbol__new(addr, ANNOTATION_DUMMY_LEN, 0, 0, name.as_ptr());
    if !sym.is_null() {
        let src = symbol__hists(sym, 1);
        if src.is_null() { symbol__delete(sym); return null_mut(); }
        dso__insert_symbol(map__dso(map), sym);
    }
    sym
}

unsafe extern "C" fn add_annotate_opt(act: *mut popup_action, optstr: *mut *mut c_char, ms: *mut map_symbol, addr: u64) -> c_int {
    if (*ms).map.is_null() || map__dso((*ms).map).is_null() || dso__annotate_warned(map__dso((*ms).map)) { return 0; }
    if (*ms).sym.is_null() { (*ms).sym = symbol__new_unresolved(addr, (*ms).map); }
    if (*ms).sym.is_null() || (*symbol__annotation((*ms).sym)).src.is_null() { return 0; }
    if asprintf(optstr, c"Annotate %s".as_ptr(), (*(*ms).sym).name) < 0 { return 0; }
    map_symbol__copy(addr_of_mut!((*act).ms), ms);
    (*act).fn_ = Some(do_annotate);
    1
}

unsafe extern "C" fn do_annotate_type(browser: *mut hist_browser, _act: *mut popup_action) -> c_int {
    hist_entry__annotate_data_tui((*browser).he_selection, hists_to_evsel((*browser).hists), (*browser).hbt);
    ui_browser__handle_resize(addr_of_mut!((*browser).b));
    0
}

unsafe extern "C" fn add_annotate_type_opt(act: *mut popup_action, optstr: *mut *mut c_char, he: *mut hist_entry) -> c_int {
    if he.is_null() || (*he).mem_type.is_null() || (*(*he).mem_type).histograms.is_null() { return 0; }
    if asprintf(optstr, c"Annotate type %s".as_ptr(), (*(*he).mem_type).self_.type_name) < 0 { return 0; }
    (*act).fn_ = Some(do_annotate_type);
    1
}

unsafe extern "C" fn do_zoom_thread(browser: *mut hist_browser, act: *mut popup_action) -> c_int {
    let thread = (*act).ms.thread;
    if thread.is_null() { return 0; }
    if !(*(*browser).hists).thread_filter.is_null() {
        pstack__remove((*browser).pstack, addr_of_mut!((*(*browser).hists).thread_filter) as *mut c_void);
        perf_hpp__set_elide(HISTC_THREAD, false);
        thread__zput((*(*browser).hists).thread_filter);
    } else {
        (*(*browser).hists).thread_filter = thread__get(thread);
        perf_hpp__set_elide(HISTC_THREAD, false);
        pstack__push((*browser).pstack, addr_of_mut!((*(*browser).hists).thread_filter) as *mut c_void);
    }
    hists__filter_by_thread((*browser).hists);
    hist_browser__reset(browser);
    0
}

unsafe extern "C" fn add_thread_opt(browser: *mut hist_browser, act: *mut popup_action, optstr: *mut *mut c_char, thread: *mut thread) -> c_int {
    if thread.is_null() { return 0; }
    let in_out = if !(*(*browser).hists).thread_filter.is_null() { c"out of".as_ptr() } else { c"into".as_ptr() };
    if asprintf(optstr, c"Zoom %s %s(%d) thread".as_ptr(), in_out, thread__comm_str(thread), thread__tid(thread)) < 0 { return 0; }
    (*act).ms.thread = thread__get(thread);
    (*act).fn_ = Some(do_zoom_thread);
    1
}

unsafe extern "C" fn hists_browser__zoom_map(browser: *mut hist_browser, map: *mut map) -> c_int {
    if map.is_null() { return 0; }
    if !(*(*browser).hists).dso_filter.is_null() {
        pstack__remove((*browser).pstack, addr_of_mut!((*(*browser).hists).dso_filter) as *mut c_void);
        perf_hpp__set_elide(HISTC_DSO, false);
        dso__put((*(*browser).hists).dso_filter);
        (*(*browser).hists).dso_filter = null_mut();
        ui_helpline__pop();
    } else {
        let dso = map__dso(map);
        (*(*browser).hists).dso_filter = dso__get(dso);
        perf_hpp__set_elide(HISTC_DSO, true);
        pstack__push((*browser).pstack, addr_of_mut!((*(*browser).hists).dso_filter) as *mut c_void);
    }
    hists__filter_by_dso((*browser).hists);
    hist_browser__reset(browser);
    0
}

unsafe extern "C" fn do_zoom_dso(browser: *mut hist_browser, act: *mut popup_action) -> c_int { hists_browser__zoom_map(browser, (*act).ms.map) }
unsafe extern "C" fn add_dso_opt(browser: *mut hist_browser, act: *mut popup_action, optstr: *mut *mut c_char, map: *mut map) -> c_int {
    if map.is_null() { return 0; }
    if asprintf(optstr, c"Zoom %s %s DSO (use the 'k' hotkey to zoom directly into the kernel)".as_ptr(), if !(*(*browser).hists).dso_filter.is_null() { c"out of".as_ptr() } else { c"into".as_ptr() }, if __map__is_kernel(map) { c"the Kernel".as_ptr() } else { dso__short_name(map__dso(map)) }) < 0 { return 0; }
    (*act).ms.map = map__get(map);
    (*act).fn_ = Some(do_zoom_dso);
    1
}
unsafe extern "C" fn do_toggle_callchain(browser: *mut hist_browser, _act: *mut popup_action) -> c_int { hist_browser__toggle_fold(browser); 0 }
unsafe extern "C" fn add_callchain_toggle_opt(browser: *mut hist_browser, act: *mut popup_action, optstr: *mut *mut c_char) -> c_int {
    let mut sym_name = [0 as c_char; 512];
    if !hist_browser__selection_has_children(browser) { return 0; }
    if asprintf(optstr, c"%s [%s] callchain (one level, same as '+' hotkey, use 'e'/'c' for the whole main level entry)".as_ptr(), if hist_browser__selection_unfolded(browser) { c"Collapse".as_ptr() } else { c"Expand".as_ptr() }, hist_browser__selection_sym_name(browser, sym_name.as_mut_ptr(), sym_name.len())) < 0 { return 0; }
    (*act).fn_ = Some(do_toggle_callchain);
    1
}
unsafe extern "C" fn do_browse_map(_browser: *mut hist_browser, act: *mut popup_action) -> c_int { map__browse((*act).ms.map); 0 }
unsafe extern "C" fn add_map_opt(_browser: *mut hist_browser, act: *mut popup_action, optstr: *mut *mut c_char, map: *mut map) -> c_int {
    if map.is_null() || asprintf(optstr, c"Browse map details".as_ptr()) < 0 { return 0; }
    (*act).ms.map = map__get(map); (*act).fn_ = Some(do_browse_map); 1
}

unsafe extern "C" fn do_run_script(browser: *mut hist_browser, act: *mut popup_action) -> c_int {
    let mut len = 100;
    if !(*act).ms.thread.is_null() { len += strlen(thread__comm_str((*act).ms.thread)) as c_int; }
    else if !(*act).ms.sym.is_null() { len += strlen((*(*act).ms.sym).name) as c_int; }
    let script_opt = malloc(len as size_t) as *mut c_char;
    if script_opt.is_null() { return -1; }
    *script_opt = 0;
    let mut n = 0;
    if !(*act).ms.thread.is_null() { n = scnprintf(script_opt, len as size_t, c" -c %s ".as_ptr(), thread__comm_str((*act).ms.thread)); }
    else if !(*act).ms.sym.is_null() { n = scnprintf(script_opt, len as size_t, c" -S %s ".as_ptr(), (*(*act).ms.sym).name); }
    if (*act).time != 0 {
        let mut start = [0 as c_char; 32]; let mut end = [0 as c_char; 32];
        let mut starttime = (*act).time; let mut endtime = (*act).time + symbol_conf.time_quantum;
        if starttime == endtime { starttime -= NSEC_PER_MSEC; endtime += NSEC_PER_MSEC; }
        timestamp__scnprintf_usec(starttime, start.as_mut_ptr(), start.len());
        timestamp__scnprintf_usec(endtime, end.as_mut_ptr(), end.len());
        snprintf(script_opt.add(n as usize), (len - n) as size_t, c" --time %s,%s".as_ptr(), start.as_ptr(), end.as_ptr());
    }
    script_browse(script_opt, hists_to_evsel((*browser).hists));
    free(script_opt as *mut c_void);
    0
}

unsafe extern "C" fn do_res_sample_script(browser: *mut hist_browser, act: *mut popup_action) -> c_int {
    let he = hist_browser__selected_entry(browser);
    res_sample_browse((*he).res_samples, (*he).num_res, hists_to_evsel((*browser).hists), (*act).rstype);
    0
}

unsafe extern "C" fn add_script_opt_2(act: *mut popup_action, optstr: *mut *mut c_char, thread: *mut thread, sym: *mut symbol, tstr: *const c_char) -> c_int {
    let ok = if !thread.is_null() { asprintf(optstr, c"Run scripts for samples of thread [%s]%s".as_ptr(), thread__comm_str(thread), tstr) }
    else if !sym.is_null() { asprintf(optstr, c"Run scripts for samples of symbol [%s]%s".as_ptr(), (*sym).name, tstr) }
    else { asprintf(optstr, c"Run scripts for all samples%s".as_ptr(), tstr) };
    if ok < 0 { return 0; }
    (*act).ms.thread = thread__get(thread);
    (*act).ms.sym = sym;
    (*act).fn_ = Some(do_run_script);
    1
}

unsafe extern "C" fn add_script_opt(browser: *mut hist_browser, act: *mut popup_action, optstr: *mut *mut c_char, thread: *mut thread, sym: *mut symbol) -> c_int {
    add_script_opt_2(act, optstr, thread, sym, c"".as_ptr())
}

unsafe extern "C" fn add_res_sample_opt(_browser: *mut hist_browser, act: *mut popup_action, optstr: *mut *mut c_char, res_sample: *mut res_sample, typ: rstype) -> c_int {
    if res_sample.is_null() { return 0; }
    if asprintf(optstr, c"Show context for individual samples %s".as_ptr(), if typ == A_ASM { c"with assembler".as_ptr() } else if typ == A_SOURCE { c"with source".as_ptr() } else { c"".as_ptr() }) < 0 { return 0; }
    (*act).fn_ = Some(do_res_sample_script);
    (*act).rstype = typ;
    1
}

unsafe extern "C" fn do_switch_data(_browser: *mut hist_browser, _act: *mut popup_action) -> c_int {
    if switch_data_file() != 0 {
        ui__warning(c"Won't switch the data files due to\nno valid data file get selected!\n".as_ptr());
        return 0;
    }
    K_SWITCH_INPUT_DATA
}
unsafe extern "C" fn add_switch_opt(browser: *mut hist_browser, act: *mut popup_action, optstr: *mut *mut c_char) -> c_int {
    if !is_report_browser((*browser).hbt as *mut c_void) { return 0; }
    if asprintf(optstr, c"Switch to another data file in PWD".as_ptr()) < 0 { return 0; }
    (*act).fn_ = Some(do_switch_data); 1
}
unsafe extern "C" fn do_exit_browser(_browser: *mut hist_browser, _act: *mut popup_action) -> c_int { 0 }
unsafe extern "C" fn add_exit_opt(_browser: *mut hist_browser, act: *mut popup_action, optstr: *mut *mut c_char) -> c_int {
    if asprintf(optstr, c"Exit".as_ptr()) < 0 { return 0; }
    (*act).fn_ = Some(do_exit_browser); 1
}
unsafe extern "C" fn do_zoom_socket(browser: *mut hist_browser, act: *mut popup_action) -> c_int {
    if (*act).socket < 0 { return 0; }
    if (*(*browser).hists).socket_filter > -1 {
        pstack__remove((*browser).pstack, addr_of_mut!((*(*browser).hists).socket_filter) as *mut c_void);
        (*(*browser).hists).socket_filter = -1; perf_hpp__set_elide(HISTC_SOCKET, false);
    } else {
        (*(*browser).hists).socket_filter = (*act).socket; perf_hpp__set_elide(HISTC_SOCKET, true);
        pstack__push((*browser).pstack, addr_of_mut!((*(*browser).hists).socket_filter) as *mut c_void);
    }
    hists__filter_by_socket((*browser).hists); hist_browser__reset(browser); 0
}
unsafe extern "C" fn add_socket_opt(browser: *mut hist_browser, act: *mut popup_action, optstr: *mut *mut c_char, socket_id: c_int) -> c_int {
    if socket_id < 0 { return 0; }
    if asprintf(optstr, c"Zoom %s Processor Socket %d".as_ptr(), if (*(*browser).hists).socket_filter > -1 { c"out of".as_ptr() } else { c"into".as_ptr() }, socket_id) < 0 { return 0; }
    (*act).socket = socket_id; (*act).fn_ = Some(do_zoom_socket); 1
}

unsafe extern "C" fn hist_browser__update_nr_entries(hb: *mut hist_browser) {
    if (*hb).min_pcnt == 0.0 && !symbol_conf.report_hierarchy {
        (*hb).nr_non_filtered_entries = (*(*hb).hists).nr_non_filtered_entries;
        return;
    }
    let mut nr_entries = 0u64;
    let mut nd = rb_first_cached(addr_of_mut!((*(*hb).hists).entries));
    loop {
        nd = hists__filter_entries(nd, (*hb).min_pcnt as c_float);
        if nd.is_null() { break; }
        nr_entries += 1;
        nd = rb_hierarchy_next(nd);
    }
    (*hb).nr_non_filtered_entries = nr_entries;
    (*hb).nr_hierarchy_entries = nr_entries;
}

unsafe extern "C" fn hist_browser__update_percent_limit(hb: *mut hist_browser, percent: c_double) {
    let mut nd = rb_first_cached(addr_of_mut!((*(*hb).hists).entries));
    let mut total = hists__total_period((*hb).hists);
    let mut min_callchain_hits = (total as c_double * (percent / 100.0)) as u64;
    (*hb).min_pcnt = percent;
    callchain_param.min_percent = percent;
    loop {
        nd = hists__filter_entries(nd, (*hb).min_pcnt as c_float);
        if nd.is_null() { break; }
        let he = rb_entry_hist_entry(nd);
        if (*he).has_no_entry { (*he).has_no_entry = false; (*he).nr_rows = 0; }
        if (*he).leaf && hist_entry__has_callchains(he) && symbol_conf.use_callchain {
            if callchain_param.mode == CHAIN_GRAPH_REL {
                total = (*he).stat.period;
                if symbol_conf.cumulate_callchain { total = (*(*he).stat_acc).period; }
                min_callchain_hits = (total as c_double * (percent / 100.0)) as u64;
            }
            if let Some(sort) = callchain_param.sort { sort(addr_of_mut!((*he).sorted_chain), (*he).callchain, min_callchain_hits, addr_of_mut!(callchain_param)); }
        }
        nd = __rb_hierarchy_next(nd, HMD_FORCE_CHILD);
        (*he).init_have_children = false;
        hist_entry__set_folding(he, hb, false);
    }
}

unsafe extern "C" fn evsel__hists_browse(evsel: *mut evsel, nr_events: c_int, helpline: *const c_char, left_exits: bool, hbt: *mut hist_browser_timer, min_pcnt: c_float, env: *mut perf_env, warn_lost_event: bool) -> c_int {
    let hists = evsel__hists(evsel);
    let browser = perf_evsel_browser__new(evsel, hbt, env);
    if browser.is_null() { return -1; }
    SLang_reset_tty(); SLang_init_tty(0, 0, 0); SLtty_set_suspend_state(true);
    if min_pcnt != 0.0 { (*browser).min_pcnt = min_pcnt as c_double; }
    hist_browser__update_nr_entries(browser);
    (*browser).pstack = pstack__new(3);
    if (*browser).pstack.is_null() { hist_browser__delete(browser); return -1; }
    ui_helpline__push(helpline);
    if !symbol_conf.col_width_list_str.is_null() { perf_hpp__set_user_width(symbol_conf.col_width_list_str); }
    if !is_report_browser(hbt as *mut c_void) { (*browser).b.no_samples_msg = c"Collecting samples...".as_ptr(); }
    let mut key = hist_browser__run(browser, helpline, warn_lost_event, 0);
    pstack__delete((*browser).pstack);
    thread__zput((*hists).thread_filter);
    dso__put((*hists).dso_filter);
    (*hists).dso_filter = null_mut();
    perf_hpp__set_elide(HISTC_DSO, false);
    perf_hpp__set_elide(HISTC_THREAD, false);
    hists__filter_by_dso(hists);
    hists__filter_by_thread(hists);
    hist_browser__delete(browser);
    key
}

#[repr(C)] pub struct evsel_menu { pub b: ui_browser, pub selection: *mut evsel, pub lost_events: bool, pub lost_events_warned: bool, pub min_pcnt: c_float, pub env: *mut perf_env }

unsafe extern "C" fn perf_evsel_menu__write(browser: *mut ui_browser, entry: *mut c_void, row: c_int) {
    let menu = browser as *mut evsel_menu;
    let evsel = entry as *mut evsel;
    let hists = evsel__hists(evsel);
    let current_entry = ui_browser__is_current_entry(browser, row as u16);
    let mut unit = 0 as c_char;
    let mut nr_events = convert_unit((*hists).stats.nr_samples, addr_of_mut!(unit));
    let ev_name = evsel__name(evsel);
    let mut bf = [0 as c_char; 256];
    let printed = scnprintf(bf.as_mut_ptr(), bf.len(), c"%lu%c%s%s".as_ptr(), nr_events, unit as c_int, if unit == b' ' as c_char { c"".as_ptr() } else { c" ".as_ptr() }, ev_name);
    ui_browser__set_color(browser, if current_entry { HE_COLORSET_SELECTED } else { HE_COLORSET_NORMAL });
    ui_browser__printf(browser, c"%s".as_ptr(), bf.as_ptr());
    ui_browser__write_nstring(browser, c" ".as_ptr(), (*browser).width - printed);
    if current_entry { (*menu).selection = evsel; }
}

unsafe extern "C" fn perf_evsel_menu__run(menu: *mut evsel_menu, nr_events: c_int, help: *const c_char, hbt: *mut hist_browser_timer, warn_lost_event: bool) -> c_int {
    let evlist = (*menu).b.priv_ as *mut evlist;
    let title = c"Available samples".as_ptr();
    let delay_secs = if !hbt.is_null() { (*hbt).refresh } else { 0 };
    if ui_browser__show(addr_of_mut!((*menu).b), title, c"ESC: exit, ENTER|->: Browse histograms".as_ptr()) < 0 { return -1; }
    let mut key;
    loop {
        key = ui_browser__run(addr_of_mut!((*menu).b), delay_secs);
        match key {
            K_TIMER => { if !hbt.is_null() { if let Some(timer) = (*hbt).timer { timer((*hbt).arg); } } continue; }
            K_RIGHT | K_ENTER => {
                if (*menu).selection.is_null() { continue; }
                evlist__set_selected(evlist, (*menu).selection);
                if !hbt.is_null() { if let Some(timer) = (*hbt).timer { timer((*hbt).arg); } }
                key = evsel__hists_browse((*menu).selection, nr_events, help, true, hbt, (*menu).min_pcnt, (*menu).env, warn_lost_event);
                ui_browser__show_title(addr_of_mut!((*menu).b), title);
                if key == K_SWITCH_INPUT_DATA || key == K_RELOAD || key == b'q' as c_int || key == CTRL(b'c' as c_int) { break; }
            }
            K_LEFT => continue,
            K_ESC => { if !ui_browser__dialog_yesno(addr_of_mut!((*menu).b), c"Do you really want to exit?".as_ptr()) { continue; } break; }
            x if x == b'q' as c_int || x == CTRL(b'c' as c_int) => break,
            _ => { ui_browser__warn_unhandled_hotkey(addr_of_mut!((*menu).b), key, delay_secs, null()); continue; }
        }
    }
    ui_browser__hide(addr_of_mut!((*menu).b));
    key
}

unsafe extern "C" fn filter_group_entries(_browser: *mut ui_browser, entry: *mut c_void) -> bool {
    let evsel = entry as *mut evsel;
    symbol_conf.event_group && !evsel__is_group_leader(evsel)
}

unsafe extern "C" fn __evlist__tui_browse_hists(evlist: *mut evlist, nr_entries: c_int, help: *const c_char, hbt: *mut hist_browser_timer, min_pcnt: c_float, env: *mut perf_env, warn_lost_event: bool) -> c_int {
    let mut menu: evsel_menu = zeroed();
    menu.b.entries = addr_of_mut!((*evlist__core(evlist)).entries) as *mut c_void;
    menu.b.refresh = Some(ui_browser__list_head_refresh);
    menu.b.seek = Some(ui_browser__list_head_seek);
    menu.b.write = Some(perf_evsel_menu__write);
    menu.b.filter = Some(filter_group_entries);
    menu.b.nr_entries = nr_entries as u64;
    menu.b.priv_ = evlist as *mut c_void;
    menu.min_pcnt = min_pcnt;
    menu.env = env;
    ui_helpline__push(c"Press ESC to exit".as_ptr());
    perf_evsel_menu__run(addr_of_mut!(menu), nr_entries, help, hbt, warn_lost_event)
}

unsafe extern "C" fn evlist__single_entry(evlist: *mut evlist) -> bool {
    let nr_entries = evlist__nr_entries(evlist);
    if nr_entries == 1 { return true; }
    if nr_entries == 2 {
        let last = evsel__last(evlist);
        if evsel__is_dummy_event(last) { return true; }
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn evlist__tui_browse_hists(evlist: *mut evlist, help: *const c_char, hbt: *mut hist_browser_timer, min_pcnt: c_float, env: *mut perf_env, warn_lost_event: bool) -> c_int {
    let mut nr_entries = evlist__nr_entries(evlist);
    if evlist__single_entry(evlist) {
        let first = evsel__first(evlist);
        return evsel__hists_browse(first, nr_entries, help, false, hbt, min_pcnt, env, warn_lost_event);
    }
    __evlist__tui_browse_hists(evlist, nr_entries, help, hbt, min_pcnt, env, warn_lost_event)
}

unsafe extern "C" fn block_hists_browser__title(browser: *mut hist_browser, bf: *mut c_char, size: size_t) -> c_int {
    let hists = evsel__hists((*browser).block_evsel);
    let evname = evsel__name((*browser).block_evsel);
    let ret = scnprintf(bf, size, c"# Samples: %lu".as_ptr(), (*hists).stats.nr_samples);
    if !evname.is_null() { scnprintf(bf.add(ret as usize), size - ret as usize, c" of event '%s'".as_ptr(), evname); }
    0
}

#[no_mangle]
pub unsafe extern "C" fn block_hists_tui_browse(bh: *mut block_hist, evsel: *mut evsel, min_percent: c_float, env: *mut perf_env) -> c_int {
    let hists = addr_of_mut!((*bh).block_hists);
    let browser = hist_browser__new(hists);
    if browser.is_null() { return -1; }
    (*browser).block_evsel = evsel;
    (*browser).title = Some(block_hists_browser__title);
    (*browser).min_pcnt = min_percent as c_double;
    (*browser).env = env;
    SLang_reset_tty(); SLang_init_tty(0, 0, 0); SLtty_set_suspend_state(true);
    let mut action: popup_action = zeroed();
    let mut br_cntr_text: *mut c_char = null_mut();
    if annotation_br_cntr_abbr_list(addr_of_mut!(br_cntr_text), evsel, false) == 0 { annotate_opts.show_br_cntr = true; }
    loop {
        let key = hist_browser__run(browser, c"? - help".as_ptr(), true, 0);
        match key {
            x if x == b'q' as c_int || x == K_ESC => break,
            x if x == b'?' as c_int => { ui_browser__help_window(addr_of_mut!((*browser).b), c" q/ESC         Quit \n B             Branch counter abbr list (Optional)\n".as_ptr()); }
            x if x == b'a' as c_int || x == K_ENTER => {
                if (*browser).selection.is_null() || (*(*browser).selection).sym.is_null() { continue; }
                action.ms.map = (*(*browser).selection).map;
                action.ms.sym = (*(*browser).selection).sym;
                do_annotate(browser, addr_of_mut!(action));
            }
            x if x == b'B' as c_int => {
                if !br_cntr_text.is_null() { ui__question_window(c"Branch counter abbr list".as_ptr(), br_cntr_text, c"Press any key...".as_ptr(), 0); }
                else { ui__question_window(c"Branch counter abbr list".as_ptr(), c"\n The branch counter is not available.\n".as_ptr(), c"Press any key...".as_ptr(), 0); }
            }
            _ => ui_browser__warn_unhandled_hotkey(addr_of_mut!((*browser).b), key, 0, c", use '?' to see actions".as_ptr()),
        }
    }
    hist_browser__delete(browser);
    free(br_cntr_text as *mut c_void);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
