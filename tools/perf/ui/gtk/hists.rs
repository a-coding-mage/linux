// SPDX-License-Identifier: GPL-2.0
// Rust translation of perf/ui/gtk/hists.c.
// C includes translated as external dependencies: gtk.h, evlist.h, callchain.h,
// evsel.h, sort.h, hist.h, helpline.h, string2.h, signal.h, stdlib.h,
// linux/string.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_float, c_int, c_uint, c_void};

type u64 = u64;
type size_t = usize;
type gboolean = c_int;
type gpointer = *mut c_void;
type GType = usize;

const MAX_COLUMNS: usize = 32;
const FALSE: gboolean = 0;
const TRUE: gboolean = 1;
const PERF_HPP__OVERHEAD: usize = 0;
const PERF_HPP__OVERHEAD_SYS: usize = 1;
const PERF_HPP__OVERHEAD_US: usize = 2;
const PERF_HPP__OVERHEAD_GUEST_SYS: usize = 3;
const PERF_HPP__OVERHEAD_GUEST_US: usize = 4;
const PERF_HPP__OVERHEAD_ACC: usize = 5;
const G_TYPE_STRING: GType = 0;
const GTK_WINDOW_TOPLEVEL: c_int = 0;
const GTK_POLICY_AUTOMATIC: c_int = 0;
const GTK_WIN_POS_CENTER: c_int = 0;
const SIGSEGV: c_int = 11;
const SIGFPE: c_int = 8;
const SIGINT: c_int = 2;
const SIGQUIT: c_int = 3;
const SIGTERM: c_int = 15;

const CHAIN_FLAT: c_int = 0;
const CHAIN_FOLDED: c_int = 1;
const CHAIN_GRAPH_REL: c_int = 2;

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root_cached {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkWidget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkTreeStore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkTreeIter {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkTreeView {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkTreePath {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkTreeViewColumn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkCellRenderer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkTreeModel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_hpp {
    pub buf: *mut c_char,
    pub size: size_t,
}

#[repr(C)]
pub struct perf_hpp_fmt {
    pub name: *const c_char,
    pub color: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hist_entry) -> c_int>,
    pub entry: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hist_entry) -> c_int>,
    pub header: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hists, c_int, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct perf_hpp_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_hpp_list_node {
    pub list: list_head,
    pub hpp: perf_hpp_list,
}

#[repr(C)]
pub struct hist_entry_stat {
    pub period: u64,
    pub period_sys: u64,
    pub period_us: u64,
    pub period_guest_sys: u64,
    pub period_guest_us: u64,
}

#[repr(C)]
pub struct hist_entry {
    pub rb_node: rb_node,
    pub hists: *mut hists,
    pub filtered: bool,
    pub stat: hist_entry_stat,
    pub stat_acc: *mut hist_entry_stat,
    pub sorted_chain: rb_root,
    pub leaf: bool,
    pub hroot_out: rb_root_cached,
    pub hpp_list: *mut perf_hpp_list,
}

#[repr(C)]
pub struct hists {
    pub entries: rb_root_cached,
    pub hpp_formats: list_head,
}

#[repr(C)]
pub struct callchain_node {
    pub rb_node: rb_node,
    pub rb_root: rb_root,
    pub parent_val: list_head,
    pub val: list_head,
    pub val_nr: u64,
    pub children_hit: u64,
}

#[repr(C)]
pub struct callchain_list {
    pub list: list_head,
}

#[repr(C)]
pub struct callchain_param_t {
    pub mode: c_int,
}

#[repr(C)]
pub struct symbol_conf_t {
    pub field_sep: *const c_char,
    pub use_callchain: bool,
    pub cumulate_callchain: bool,
    pub event_group: bool,
    pub report_hierarchy: bool,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
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
pub struct hist_browser_timer {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut perf_hpp__format: [perf_hpp_fmt; 6];
    static mut callchain_param: callchain_param_t;
    static mut symbol_conf: symbol_conf_t;
    static mut pgctx: *mut c_void;

    fn perf_gtk__get_percent_color(percent: c_double) -> *const c_char;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn free(ptr: *mut c_void);
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strim(s: *mut c_char) -> *mut c_char;
    fn signal(signum: c_int, handler: unsafe extern "C" fn(c_int)) -> unsafe extern "C" fn(c_int);

    fn hpp__fmt(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry,
                get_field: unsafe extern "C" fn(*mut hist_entry) -> u64,
                fmtstr: *const c_char,
                snprintf_fn: unsafe extern "C" fn(*mut perf_hpp, *const c_char, ...) -> c_int,
                fmt_percent: bool) -> c_int;
    fn hpp__fmt_acc(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry,
                    get_field: unsafe extern "C" fn(*mut hist_entry) -> u64,
                    fmtstr: *const c_char,
                    snprintf_fn: unsafe extern "C" fn(*mut perf_hpp, *const c_char, ...) -> c_int,
                    fmt_percent: bool) -> c_int;

    fn rb_first(root: *mut rb_root) -> *mut rb_node;
    fn rb_last(root: *mut rb_root) -> *mut rb_node;
    fn rb_next(nd: *mut rb_node) -> *mut rb_node;
    fn rb_first_cached(root: *mut rb_root_cached) -> *mut rb_node;
    fn rb_entry_hist_entry(nd: *mut rb_node) -> *mut hist_entry;
    fn rb_entry_callchain_node(nd: *mut rb_node) -> *mut callchain_node;

    fn list_first_entry_perf_hpp_list_node(head: *mut list_head) -> *mut perf_hpp_list_node;
    fn list_for_each_entry_callchain_list(head: *mut list_head, cb: unsafe extern "C" fn(*mut callchain_list, *mut c_void), data: *mut c_void);
    fn list_for_each_entry_continue_perf_hpp_list_node(pos: *mut perf_hpp_list_node, head: *mut list_head, cb: unsafe extern "C" fn(*mut perf_hpp_list_node, *mut c_void), data: *mut c_void);
    fn hists__for_each_format(hists: *mut hists, cb: unsafe extern "C" fn(*mut perf_hpp_fmt, *mut c_void), data: *mut c_void);
    fn perf_hpp_list__for_each_format(hpp: *mut perf_hpp_list, cb: unsafe extern "C" fn(*mut perf_hpp_fmt, *mut c_void), data: *mut c_void);
    fn evlist__for_each_entry(evlist: *mut evlist, cb: unsafe extern "C" fn(*mut evsel, *mut c_void), data: *mut c_void);

    fn callchain_node__make_parent_list(node: *mut callchain_node);
    fn callchain_node__scnprintf_value(node: *mut callchain_node, buf: *mut c_char, size: size_t, total: u64) -> c_int;
    fn callchain_list__sym_name(chain: *mut callchain_list, buf: *mut c_char, size: size_t, show_dso: bool) -> c_int;
    fn hists__total_period(hists: *mut hists) -> u64;
    fn hist_entry__get_percent_limit(he: *mut hist_entry) -> c_float;
    fn hist_entry__has_callchains(he: *mut hist_entry) -> bool;
    fn hist_entry__has_hierarchy_children(he: *mut hist_entry, min_pcnt: c_float) -> bool;
    fn hists__has(hists: *mut hists, field: c_int) -> bool;
    fn perf_hpp__should_skip(fmt: *mut perf_hpp_fmt, hists: *mut hists) -> bool;
    fn perf_hpp__is_sort_entry(fmt: *mut perf_hpp_fmt) -> bool;
    fn perf_hpp__is_dynamic_entry(fmt: *mut perf_hpp_fmt) -> bool;
    fn advance_hpp(hpp: *mut perf_hpp, inc: c_int);
    fn evsel__hists(pos: *mut evsel) -> *mut hists;
    fn evsel__name(pos: *mut evsel) -> *const c_char;
    fn evsel__is_group_leader(pos: *mut evsel) -> bool;
    fn evsel__group_desc(pos: *mut evsel, buf: *mut c_char, size: size_t);

    fn gtk_tree_store_append(store: *mut GtkTreeStore, iter: *mut GtkTreeIter, parent: *mut GtkTreeIter);
    fn gtk_tree_store_set(store: *mut GtkTreeStore, iter: *mut GtkTreeIter, ...);
    fn gtk_tree_store_newv(n_columns: c_int, types: *mut GType) -> *mut GtkTreeStore;
    fn gtk_tree_view_new() -> *mut GtkWidget;
    fn gtk_cell_renderer_text_new() -> *mut GtkCellRenderer;
    fn gtk_tree_view_insert_column_with_attributes(view: *mut GtkTreeView, position: c_int, title: *const c_char,
                                                   cell: *mut GtkCellRenderer, ...) -> c_int;
    fn gtk_tree_view_get_column(view: *mut GtkTreeView, n: c_int) -> *mut GtkTreeViewColumn;
    fn gtk_tree_view_column_set_resizable(column: *mut GtkTreeViewColumn, resizable: gboolean);
    fn gtk_tree_view_set_expander_column(view: *mut GtkTreeView, column: *mut GtkTreeViewColumn);
    fn gtk_tree_view_set_model(view: *mut GtkTreeView, model: *mut GtkTreeModel);
    fn gtk_tree_view_set_rules_hint(view: *mut GtkTreeView, setting: gboolean);
    fn gtk_tree_view_row_expanded(view: *mut GtkTreeView, path: *mut GtkTreePath) -> bool;
    fn gtk_tree_view_collapse_row(view: *mut GtkTreeView, path: *mut GtkTreePath) -> bool;
    fn gtk_tree_view_expand_row(view: *mut GtkTreeView, path: *mut GtkTreePath, open_all: gboolean);
    fn gtk_container_add(container: *mut GtkWidget, widget: *mut GtkWidget);
    fn g_object_unref(object: *mut c_void);
    fn g_signal_connect(instance: *mut c_void, detailed_signal: *const c_char, c_handler: *mut c_void, data: *mut c_void) -> u64;
    fn gtk_window_new(t: c_int) -> *mut GtkWidget;
    fn gtk_window_set_title(window: *mut GtkWidget, title: *const c_char);
    fn gtk_main_quit();
    fn gtk_vbox_new(homogeneous: gboolean, spacing: c_int) -> *mut GtkWidget;
    fn gtk_notebook_new() -> *mut GtkWidget;
    fn gtk_box_pack_start(box_: *mut GtkWidget, child: *mut GtkWidget, expand: gboolean, fill: gboolean, padding: c_uint);
    fn perf_gtk__setup_info_bar() -> *mut GtkWidget;
    fn perf_gtk__setup_statusbar() -> *mut GtkWidget;
    fn gtk_scrolled_window_new(hadjustment: *mut c_void, vadjustment: *mut c_void) -> *mut GtkWidget;
    fn gtk_scrolled_window_set_policy(scrolled_window: *mut GtkWidget, hscrollbar_policy: c_int, vscrollbar_policy: c_int);
    fn gtk_label_new(str: *const c_char) -> *mut GtkWidget;
    fn gtk_notebook_append_page(notebook: *mut GtkWidget, child: *mut GtkWidget, tab_label: *mut GtkWidget) -> c_int;
    fn gtk_widget_show_all(widget: *mut GtkWidget);
    fn gtk_window_set_position(window: *mut GtkWidget, position: c_int);
    fn gtk_main();
    fn perf_gtk__signal(signum: c_int);
    fn perf_gtk__activate_context(window: *mut GtkWidget) -> *mut c_void;
    fn perf_gtk__deactivate_context(ctx: *mut *mut c_void);
    fn perf_gtk__resize_window(window: *mut GtkWidget);
    fn ui_helpline__push(msg: *const c_char);
}

const sym: c_int = 0;

unsafe extern "C" fn __percent_color_snprintf(hpp: *mut perf_hpp, fmt: *const c_char, mut _args: ...) -> c_int {
    let mut ret: c_int = 0;
    let len: c_int = _args.arg();
    let percent: c_double = _args.arg();
    let buf = (*hpp).buf;
    let size = (*hpp).size;

    let markup = perf_gtk__get_percent_color(percent);
    if !markup.is_null() {
        ret += scnprintf(buf, size, markup);
    }

    ret += scnprintf(buf.add(ret as usize), size.wrapping_sub(ret as usize), fmt, len, percent);

    if !markup.is_null() {
        ret += scnprintf(buf.add(ret as usize), size.wrapping_sub(ret as usize), c"</span>".as_ptr());
    }

    ret
}

unsafe extern "C" fn he_get_period(he: *mut hist_entry) -> u64 { (*he).stat.period }
unsafe extern "C" fn perf_gtk__hpp_color_overhead(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
    hpp__fmt(fmt, hpp, he, he_get_period, c" %*.2f%%".as_ptr(), __percent_color_snprintf, true)
}

unsafe extern "C" fn he_get_period_sys(he: *mut hist_entry) -> u64 { (*he).stat.period_sys }
unsafe extern "C" fn perf_gtk__hpp_color_overhead_sys(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
    hpp__fmt(fmt, hpp, he, he_get_period_sys, c" %*.2f%%".as_ptr(), __percent_color_snprintf, true)
}

unsafe extern "C" fn he_get_period_us(he: *mut hist_entry) -> u64 { (*he).stat.period_us }
unsafe extern "C" fn perf_gtk__hpp_color_overhead_us(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
    hpp__fmt(fmt, hpp, he, he_get_period_us, c" %*.2f%%".as_ptr(), __percent_color_snprintf, true)
}

unsafe extern "C" fn he_get_period_guest_sys(he: *mut hist_entry) -> u64 { (*he).stat.period_guest_sys }
unsafe extern "C" fn perf_gtk__hpp_color_overhead_guest_sys(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
    hpp__fmt(fmt, hpp, he, he_get_period_guest_sys, c" %*.2f%%".as_ptr(), __percent_color_snprintf, true)
}

unsafe extern "C" fn he_get_period_guest_us(he: *mut hist_entry) -> u64 { (*he).stat.period_guest_us }
unsafe extern "C" fn perf_gtk__hpp_color_overhead_guest_us(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
    hpp__fmt(fmt, hpp, he, he_get_period_guest_us, c" %*.2f%%".as_ptr(), __percent_color_snprintf, true)
}

unsafe extern "C" fn he_get_acc_period(he: *mut hist_entry) -> u64 { (*(*he).stat_acc).period }
unsafe extern "C" fn perf_gtk__hpp_color_overhead_acc(fmt: *mut perf_hpp_fmt, hpp: *mut perf_hpp, he: *mut hist_entry) -> c_int {
    hpp__fmt_acc(fmt, hpp, he, he_get_acc_period, c" %*.2f%%".as_ptr(), __percent_color_snprintf, true)
}

#[no_mangle]
pub unsafe extern "C" fn perf_gtk__init_hpp() {
    perf_hpp__format[PERF_HPP__OVERHEAD].color = Some(perf_gtk__hpp_color_overhead);
    perf_hpp__format[PERF_HPP__OVERHEAD_SYS].color = Some(perf_gtk__hpp_color_overhead_sys);
    perf_hpp__format[PERF_HPP__OVERHEAD_US].color = Some(perf_gtk__hpp_color_overhead_us);
    perf_hpp__format[PERF_HPP__OVERHEAD_GUEST_SYS].color = Some(perf_gtk__hpp_color_overhead_guest_sys);
    perf_hpp__format[PERF_HPP__OVERHEAD_GUEST_US].color = Some(perf_gtk__hpp_color_overhead_guest_us);
    perf_hpp__format[PERF_HPP__OVERHEAD_ACC].color = Some(perf_gtk__hpp_color_overhead_acc);
}

unsafe fn add_callchain_list_rows(node: *mut callchain_node, head: *mut list_head, store: *mut GtkTreeStore,
                                  new_parent: *mut GtkTreeIter, iter: *mut GtkTreeIter, need_new_parent: *mut bool,
                                  col: c_int, total: u64) {
    unsafe extern "C" fn each(chain: *mut callchain_list, data: *mut c_void) {
        unsafe {
            let d = data as *mut (*mut callchain_node, *mut GtkTreeStore, *mut GtkTreeIter, *mut GtkTreeIter, *mut bool, c_int, u64);
            let (node, store, new_parent, iter, need_new_parent, col, total) = *d;
            let mut buf = [0 as c_char; 128];

            gtk_tree_store_append(store, iter, new_parent);
            callchain_node__scnprintf_value(node, buf.as_mut_ptr(), buf.len(), total);
            gtk_tree_store_set(store, iter, 0, buf.as_mut_ptr(), -1);
            callchain_list__sym_name(chain, buf.as_mut_ptr(), buf.len(), false);
            gtk_tree_store_set(store, iter, col, buf.as_mut_ptr(), -1);

            if *need_new_parent {
                /*
                 * Only show the top-most symbol in a callchain
                 * if it's not the only callchain.
                 */
                *new_parent = *iter;
                *need_new_parent = false;
            }
        }
    }
    let mut data = (node, store, new_parent, iter, need_new_parent, col, total);
    list_for_each_entry_callchain_list(head, each, &mut data as *mut _ as *mut c_void);
}

unsafe fn perf_gtk__add_callchain_flat(root: *mut rb_root, store: *mut GtkTreeStore,
                                       parent: *mut GtkTreeIter, col: c_int, total: u64) {
    let has_single_node = rb_first(root) == rb_last(root);
    let mut nd = rb_first(root);

    while !nd.is_null() {
        let node = rb_entry_callchain_node(nd);
        let mut iter: GtkTreeIter = core::mem::zeroed();
        let mut new_parent = *parent;
        let mut need_new_parent = !has_single_node;

        callchain_node__make_parent_list(node);
        add_callchain_list_rows(node, &mut (*node).parent_val, store, &mut new_parent, &mut iter, &mut need_new_parent, col, total);
        add_callchain_list_rows(node, &mut (*node).val, store, &mut new_parent, &mut iter, &mut need_new_parent, col, total);

        nd = rb_next(nd);
    }
}

unsafe fn perf_gtk__add_callchain_folded(root: *mut rb_root, store: *mut GtkTreeStore,
                                         parent: *mut GtkTreeIter, col: c_int, total: u64) {
    let mut nd = rb_first(root);
    while !nd.is_null() {
        let node = rb_entry_callchain_node(nd);
        let mut iter: GtkTreeIter = core::mem::zeroed();
        let mut buf = [0 as c_char; 64];
        let mut str_: *mut c_char = core::ptr::null_mut();
        let mut str_alloc: *mut c_char = core::ptr::null_mut();
        let mut first = true;

        callchain_node__make_parent_list(node);

        unsafe extern "C" fn folded_each(chain: *mut callchain_list, data: *mut c_void) {
            unsafe {
                let d = data as *mut (*mut c_char, *mut c_char, bool);
                let (str_ptr, str_alloc, first) = &mut *d;
                let mut name = [0 as c_char; 1024];
                callchain_list__sym_name(chain, name.as_mut_ptr(), name.len(), false);
                let sep = if *first {
                    c"".as_ptr()
                } else if !symbol_conf.field_sep.is_null() {
                    symbol_conf.field_sep
                } else {
                    c"; ".as_ptr()
                };
                if asprintf(str_ptr, c"%s%s%s".as_ptr(),
                            if *first { c"".as_ptr() } else { *str_alloc },
                            sep, name.as_mut_ptr()) < 0 {
                    return;
                }
                *first = false;
                free(*str_alloc as *mut c_void);
                *str_alloc = *str_ptr;
            }
        }

        let mut data = (str_, str_alloc, first);
        list_for_each_entry_callchain_list(&mut (*node).parent_val, folded_each, &mut data as *mut _ as *mut c_void);
        str_ = data.0;
        str_alloc = data.1;
        first = data.2;
        data = (str_, str_alloc, first);
        list_for_each_entry_callchain_list(&mut (*node).val, folded_each, &mut data as *mut _ as *mut c_void);
        str_ = data.0;
        str_alloc = data.1;

        gtk_tree_store_append(store, &mut iter, parent);
        callchain_node__scnprintf_value(node, buf.as_mut_ptr(), buf.len(), total);
        gtk_tree_store_set(store, &mut iter, 0, buf.as_mut_ptr(), -1);
        gtk_tree_store_set(store, &mut iter, col, str_, -1);
        free(str_alloc as *mut c_void);

        nd = rb_next(nd);
    }
}

unsafe fn perf_gtk__add_callchain_graph(root: *mut rb_root, store: *mut GtkTreeStore,
                                        parent: *mut GtkTreeIter, col: c_int, total: u64) {
    let has_single_node = rb_first(root) == rb_last(root);
    let mut nd = rb_first(root);

    while !nd.is_null() {
        let node = rb_entry_callchain_node(nd);
        let mut iter: GtkTreeIter = core::mem::zeroed();
        let mut new_parent = *parent;
        let mut need_new_parent = !has_single_node && (*node).val_nr > 1;

        add_callchain_list_rows(node, &mut (*node).val, store, &mut new_parent, &mut iter, &mut need_new_parent, col, total);

        let child_total = if callchain_param.mode == CHAIN_GRAPH_REL {
            (*node).children_hit
        } else {
            total
        };

        /* Now 'iter' contains info of the last callchain_list */
        perf_gtk__add_callchain_graph(&mut (*node).rb_root, store, &mut iter, col, child_total);
        nd = rb_next(nd);
    }
}

unsafe fn perf_gtk__add_callchain(root: *mut rb_root, store: *mut GtkTreeStore,
                                  parent: *mut GtkTreeIter, col: c_int, total: u64) {
    if callchain_param.mode == CHAIN_FLAT {
        perf_gtk__add_callchain_flat(root, store, parent, col, total);
    } else if callchain_param.mode == CHAIN_FOLDED {
        perf_gtk__add_callchain_folded(root, store, parent, col, total);
    } else {
        perf_gtk__add_callchain_graph(root, store, parent, col, total);
    }
}

unsafe extern "C" fn on_row_activated(view: *mut GtkTreeView, path: *mut GtkTreePath,
                                      _col: *mut GtkTreeViewColumn,
                                      _user_data: gpointer) {
    let expanded = gtk_tree_view_row_expanded(view, path);

    if expanded {
        gtk_tree_view_collapse_row(view, path);
    } else {
        gtk_tree_view_expand_row(view, path, FALSE);
    }
}

unsafe fn perf_gtk__show_hists(window: *mut GtkWidget, hists: *mut hists, min_pcnt: c_float) {
    let mut col_types = [0 as GType; MAX_COLUMNS];
    let mut s = [0 as c_char; 512];
    let mut hpp = perf_hpp { buf: s.as_mut_ptr(), size: s.len() };
    let mut nr_cols: c_int = 0;

    unsafe extern "C" fn count_fmt(_fmt: *mut perf_hpp_fmt, data: *mut c_void) {
        unsafe {
            let pair = data as *mut (*mut GType, *mut c_int);
            let (col_types, nr_cols) = *pair;
            *col_types.add(*nr_cols as usize) = G_TYPE_STRING;
            *nr_cols += 1;
        }
    }
    let mut count_data = (col_types.as_mut_ptr(), &mut nr_cols as *mut c_int);
    hists__for_each_format(hists, count_fmt, &mut count_data as *mut _ as *mut c_void);

    let store = gtk_tree_store_newv(nr_cols, col_types.as_mut_ptr());
    let view = gtk_tree_view_new();
    let renderer = gtk_cell_renderer_text_new();
    let mut col_idx: c_int = 0;
    let mut sym_col: c_int = -1;

    unsafe extern "C" fn add_columns(fmt: *mut perf_hpp_fmt, data: *mut c_void) {
        unsafe {
            let d = data as *mut (*mut hists, *mut GtkWidget, *mut GtkCellRenderer, *mut c_int, *mut c_int);
            let (hists, view, renderer, col_idx, sym_col) = *d;
            if perf_hpp__should_skip(fmt, hists) {
                return;
            }
            /*
             * XXX no way to determine where symcol column is..
             *     Just use last column for now.
             */
            if perf_hpp__is_sort_entry(fmt) {
                *sym_col = *col_idx;
            }
            gtk_tree_view_insert_column_with_attributes(view as *mut GtkTreeView, -1, (*fmt).name,
                                                        renderer, c"markup".as_ptr(), *col_idx, core::ptr::null::<c_void>());
            *col_idx += 1;
        }
    }
    let mut add_col_data = (hists, view, renderer, &mut col_idx as *mut c_int, &mut sym_col as *mut c_int);
    hists__for_each_format(hists, add_columns, &mut add_col_data as *mut _ as *mut c_void);

    for i in 0..nr_cols {
        let column = gtk_tree_view_get_column(view as *mut GtkTreeView, i);
        gtk_tree_view_column_set_resizable(column, TRUE);
        if i == sym_col {
            gtk_tree_view_set_expander_column(view as *mut GtkTreeView, column);
        }
    }

    gtk_tree_view_set_model(view as *mut GtkTreeView, store as *mut GtkTreeModel);
    g_object_unref(store as *mut c_void);

    let mut nd = rb_first_cached(&mut (*hists).entries);
    while !nd.is_null() {
        let h = rb_entry_hist_entry(nd);
        let mut iter: GtkTreeIter = core::mem::zeroed();
        let mut total = hists__total_period((*h).hists);
        let percent: c_float;

        if (*h).filtered {
            nd = rb_next(nd);
            continue;
        }

        percent = hist_entry__get_percent_limit(h);
        if percent < min_pcnt {
            nd = rb_next(nd);
            continue;
        }

        gtk_tree_store_append(store, &mut iter, core::ptr::null_mut());
        col_idx = 0;

        unsafe extern "C" fn fill_fmt(fmt: *mut perf_hpp_fmt, data: *mut c_void) {
            unsafe {
                let d = data as *mut (*mut hists, *mut GtkTreeStore, *mut GtkTreeIter, *mut c_int, *mut perf_hpp, *mut hist_entry);
                let (hists, store, iter, col_idx, hpp, h) = *d;
                if perf_hpp__should_skip(fmt, (*h).hists) {
                    return;
                }
                if let Some(color) = (*fmt).color {
                    color(fmt, hpp, h);
                } else if let Some(entry) = (*fmt).entry {
                    entry(fmt, hpp, h);
                }
                gtk_tree_store_set(store, iter, *col_idx, (*hpp).buf, -1);
                *col_idx += 1;
                let _ = hists;
            }
        }
        let mut fill_data = (hists, store, &mut iter as *mut GtkTreeIter, &mut col_idx as *mut c_int, &mut hpp as *mut perf_hpp, h);
        hists__for_each_format(hists, fill_fmt, &mut fill_data as *mut _ as *mut c_void);

        if hist_entry__has_callchains(h) && symbol_conf.use_callchain && hists__has(hists, sym) {
            if callchain_param.mode == CHAIN_GRAPH_REL {
                total = if symbol_conf.cumulate_callchain {
                    (*(*h).stat_acc).period
                } else {
                    (*h).stat.period
                };
            }
            perf_gtk__add_callchain(&mut (*h).sorted_chain, store, &mut iter, sym_col, total);
        }

        nd = rb_next(nd);
    }

    gtk_tree_view_set_rules_hint(view as *mut GtkTreeView, TRUE);
    g_signal_connect(view as *mut c_void, c"row-activated".as_ptr(), on_row_activated as *mut c_void, core::ptr::null_mut());
    gtk_container_add(window, view);
}

unsafe fn perf_gtk__add_hierarchy_entries(hists: *mut hists, root: *mut rb_root_cached,
                                          store: *mut GtkTreeStore, parent: *mut GtkTreeIter,
                                          hpp: *mut perf_hpp, min_pcnt: c_float) {
    let mut node = rb_first_cached(root);
    let mut total = hists__total_period(hists);

    while !node.is_null() {
        let mut iter: GtkTreeIter = core::mem::zeroed();
        let he = rb_entry_hist_entry(node);

        if (*he).filtered {
            node = rb_next(node);
            continue;
        }

        let percent = hist_entry__get_percent_limit(he);
        if percent < min_pcnt {
            node = rb_next(node);
            continue;
        }

        gtk_tree_store_append(store, &mut iter, parent);
        let mut col_idx: c_int = 0;

        /* the first hpp_list_node is for overhead columns */
        let fmt_node = list_first_entry_perf_hpp_list_node(&mut (*hists).hpp_formats);
        unsafe extern "C" fn overhead_fmt(fmt: *mut perf_hpp_fmt, data: *mut c_void) {
            unsafe {
                let d = data as *mut (*mut GtkTreeStore, *mut GtkTreeIter, *mut c_int, *mut perf_hpp, *mut hist_entry);
                let (store, iter, col_idx, hpp, he) = *d;
                if let Some(color) = (*fmt).color {
                    color(fmt, hpp, he);
                } else if let Some(entry) = (*fmt).entry {
                    entry(fmt, hpp, he);
                }
                gtk_tree_store_set(store, iter, *col_idx, (*hpp).buf, -1);
                *col_idx += 1;
            }
        }
        let mut overhead_data = (store, &mut iter as *mut GtkTreeIter, &mut col_idx as *mut c_int, hpp, he);
        perf_hpp_list__for_each_format(&mut (*fmt_node).hpp, overhead_fmt, &mut overhead_data as *mut _ as *mut c_void);

        let bf = (*hpp).buf;
        let size = (*hpp).size;
        unsafe extern "C" fn leaf_fmt(fmt: *mut perf_hpp_fmt, data: *mut c_void) {
            unsafe {
                let d = data as *mut (*mut perf_hpp, *mut hist_entry);
                let (hpp, he) = *d;
                let ret = if let Some(color) = (*fmt).color {
                    color(fmt, hpp, he)
                } else if let Some(entry) = (*fmt).entry {
                    entry(fmt, hpp, he)
                } else {
                    0
                };
                snprintf((*hpp).buf.add(ret as usize), (*hpp).size.wrapping_sub(ret as usize), c"  ".as_ptr());
                advance_hpp(hpp, ret + 2);
            }
        }
        let mut leaf_data = (hpp, he);
        perf_hpp_list__for_each_format((*he).hpp_list, leaf_fmt, &mut leaf_data as *mut _ as *mut c_void);

        gtk_tree_store_set(store, &mut iter, col_idx, strim(bf), -1);

        if !(*he).leaf {
            (*hpp).buf = bf;
            (*hpp).size = size;
            perf_gtk__add_hierarchy_entries(hists, &mut (*he).hroot_out, store, &mut iter, hpp, min_pcnt);

            if !hist_entry__has_hierarchy_children(he, min_pcnt) {
                let mut buf = [0 as c_char; 32];
                let mut child: GtkTreeIter = core::mem::zeroed();
                snprintf(buf.as_mut_ptr(), buf.len(), c"no entry >= %.2f%%".as_ptr(), min_pcnt as c_double);
                gtk_tree_store_append(store, &mut child, &mut iter);
                gtk_tree_store_set(store, &mut child, col_idx, buf.as_mut_ptr(), -1);
            }
        }

        if (*he).leaf && hist_entry__has_callchains(he) && symbol_conf.use_callchain {
            if callchain_param.mode == CHAIN_GRAPH_REL {
                total = if symbol_conf.cumulate_callchain {
                    (*(*he).stat_acc).period
                } else {
                    (*he).stat.period
                };
            }
            perf_gtk__add_callchain(&mut (*he).sorted_chain, store, &mut iter, col_idx, total);
        }

        node = rb_next(node);
    }
}

unsafe fn perf_gtk__show_hierarchy(window: *mut GtkWidget, hists: *mut hists, min_pcnt: c_float) {
    let mut col_types = [0 as GType; MAX_COLUMNS];
    let mut nr_cols: c_int = 0;
    let mut s = [0 as c_char; 512];
    let mut buf = [0 as c_char; 512];
    let mut hpp = perf_hpp { buf: s.as_mut_ptr(), size: s.len() };

    unsafe extern "C" fn count_hierarchy_fmt(fmt: *mut perf_hpp_fmt, data: *mut c_void) {
        unsafe {
            let d = data as *mut (*mut GType, *mut c_int, bool);
            let (col_types, nr_cols, stopped) = &mut *d;
            if *stopped {
                return;
            }
            if perf_hpp__is_sort_entry(fmt) || perf_hpp__is_dynamic_entry(fmt) {
                *stopped = true;
                return;
            }
            *col_types.add(**nr_cols as usize) = G_TYPE_STRING;
            **nr_cols += 1;
        }
    }
    let mut count_data = (col_types.as_mut_ptr(), &mut nr_cols as *mut c_int, false);
    hists__for_each_format(hists, count_hierarchy_fmt, &mut count_data as *mut _ as *mut c_void);
    col_types[nr_cols as usize] = G_TYPE_STRING;
    nr_cols += 1;

    let store = gtk_tree_store_newv(nr_cols, col_types.as_mut_ptr());
    let view = gtk_tree_view_new();
    let renderer = gtk_cell_renderer_text_new();
    let mut col_idx: c_int = 0;

    /* the first hpp_list_node is for overhead columns */
    let mut fmt_node = list_first_entry_perf_hpp_list_node(&mut (*hists).hpp_formats);
    unsafe extern "C" fn insert_overhead_column(fmt: *mut perf_hpp_fmt, data: *mut c_void) {
        unsafe {
            let d = data as *mut (*mut GtkWidget, *mut GtkCellRenderer, *mut c_int);
            let (view, renderer, col_idx) = *d;
            gtk_tree_view_insert_column_with_attributes(view as *mut GtkTreeView, -1, (*fmt).name,
                                                        renderer, c"markup".as_ptr(), *col_idx, core::ptr::null::<c_void>());
            *col_idx += 1;
        }
    }
    let mut ins_data = (view, renderer, &mut col_idx as *mut c_int);
    perf_hpp_list__for_each_format(&mut (*fmt_node).hpp, insert_overhead_column, &mut ins_data as *mut _ as *mut c_void);

    /* construct merged column header since sort keys share single column */
    buf[0] = 0;
    let mut first_node = true;
    unsafe extern "C" fn continue_node(node: *mut perf_hpp_list_node, data: *mut c_void) {
        unsafe {
            let d = data as *mut (*mut c_char, *mut bool, *mut perf_hpp, *mut hists);
            let (buf, first_node, hpp, hists) = *d;
            if !*first_node {
                strcat(buf, c" / ".as_ptr());
            }
            *first_node = false;

            let mut first_col = true;
            unsafe extern "C" fn header_fmt(fmt: *mut perf_hpp_fmt, data: *mut c_void) {
                unsafe {
                    let d = data as *mut (*mut c_char, *mut bool, *mut perf_hpp, *mut hists);
                    let (buf, first_col, hpp, hists) = *d;
                    if perf_hpp__should_skip(fmt, hists) {
                        return;
                    }
                    if !*first_col {
                        strcat(buf, c"+".as_ptr());
                    }
                    *first_col = false;
                    if let Some(header) = (*fmt).header {
                        header(fmt, hpp, hists, 0, core::ptr::null_mut());
                    }
                    strcat(buf, strim((*hpp).buf));
                }
            }
            let mut header_data = (buf, &mut first_col as *mut bool, hpp, hists);
            perf_hpp_list__for_each_format(&mut (*node).hpp, header_fmt, &mut header_data as *mut _ as *mut c_void);
        }
    }
    let mut cont_data = (buf.as_mut_ptr(), &mut first_node as *mut bool, &mut hpp as *mut perf_hpp, hists);
    list_for_each_entry_continue_perf_hpp_list_node(fmt_node, &mut (*hists).hpp_formats, continue_node, &mut cont_data as *mut _ as *mut c_void);

    gtk_tree_view_insert_column_with_attributes(view as *mut GtkTreeView, -1, buf.as_mut_ptr(),
                                                renderer, c"markup".as_ptr(), col_idx, core::ptr::null::<c_void>());
    col_idx += 1;

    for i in 0..nr_cols {
        let column = gtk_tree_view_get_column(view as *mut GtkTreeView, i);
        gtk_tree_view_column_set_resizable(column, TRUE);
        if i == 0 {
            gtk_tree_view_set_expander_column(view as *mut GtkTreeView, column);
        }
    }

    gtk_tree_view_set_model(view as *mut GtkTreeView, store as *mut GtkTreeModel);
    g_object_unref(store as *mut c_void);

    perf_gtk__add_hierarchy_entries(hists, &mut (*hists).entries, store, core::ptr::null_mut(), &mut hpp, min_pcnt);

    gtk_tree_view_set_rules_hint(view as *mut GtkTreeView, TRUE);
    g_signal_connect(view as *mut c_void, c"row-activated".as_ptr(), on_row_activated as *mut c_void, core::ptr::null_mut());
    gtk_container_add(window, view);
}

#[no_mangle]
pub unsafe extern "C" fn evlist__gtk_browse_hists(evlist: *mut evlist, help: *const c_char,
                                                  _hbt: *mut hist_browser_timer, min_pcnt: c_float) -> c_int {
    signal(SIGSEGV, perf_gtk__signal);
    signal(SIGFPE, perf_gtk__signal);
    signal(SIGINT, perf_gtk__signal);
    signal(SIGQUIT, perf_gtk__signal);
    signal(SIGTERM, perf_gtk__signal);

    let window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_title(window, c"perf report".as_ptr());
    g_signal_connect(window as *mut c_void, c"delete_event".as_ptr(), gtk_main_quit as *mut c_void, core::ptr::null_mut());

    pgctx = perf_gtk__activate_context(window);
    if pgctx.is_null() {
        return -1;
    }

    let vbox = gtk_vbox_new(FALSE, 0);
    let notebook = gtk_notebook_new();
    gtk_box_pack_start(vbox, notebook, TRUE, TRUE, 0);

    let info_bar = perf_gtk__setup_info_bar();
    if !info_bar.is_null() {
        gtk_box_pack_start(vbox, info_bar, FALSE, FALSE, 0);
    }

    let statbar = perf_gtk__setup_statusbar();
    gtk_box_pack_start(vbox, statbar, FALSE, FALSE, 0);
    gtk_container_add(window, vbox);

    unsafe extern "C" fn each_evsel(pos: *mut evsel, data: *mut c_void) {
        unsafe {
            let d = data as *mut (*mut GtkWidget, c_float);
            let (notebook, min_pcnt) = *d;
            let hists = evsel__hists(pos);
            let mut evname = evsel__name(pos);
            let mut buf = [0 as c_char; 512];
            let size = buf.len();

            if symbol_conf.event_group {
                if !evsel__is_group_leader(pos) {
                    return;
                }
                if (*pos).core.nr_members > 1 {
                    evsel__group_desc(pos, buf.as_mut_ptr(), size);
                    evname = buf.as_mut_ptr();
                }
            }

            let scrolled_window = gtk_scrolled_window_new(core::ptr::null_mut(), core::ptr::null_mut());
            gtk_scrolled_window_set_policy(scrolled_window, GTK_POLICY_AUTOMATIC, GTK_POLICY_AUTOMATIC);

            if symbol_conf.report_hierarchy {
                perf_gtk__show_hierarchy(scrolled_window, hists, min_pcnt);
            } else {
                perf_gtk__show_hists(scrolled_window, hists, min_pcnt);
            }

            let tab_label = gtk_label_new(evname);
            gtk_notebook_append_page(notebook, scrolled_window, tab_label);
        }
    }
    let mut evsel_data = (notebook, min_pcnt);
    evlist__for_each_entry(evlist, each_evsel, &mut evsel_data as *mut _ as *mut c_void);

    gtk_widget_show_all(window);
    perf_gtk__resize_window(window);
    gtk_window_set_position(window, GTK_WIN_POS_CENTER);
    ui_helpline__push(help);
    gtk_main();
    perf_gtk__deactivate_context(&mut pgctx);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
