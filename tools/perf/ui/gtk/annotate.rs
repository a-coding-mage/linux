// SPDX-License-Identifier: GPL-2.0
// Translated from perf/ui/gtk/annotate.c.
// Dependencies originally provided by:
// "gtk.h", "util/sort.h", "util/debug.h", "util/annotate.h",
// "util/evlist.h", "util/evsel.h", "util/map.h", "util/dso.h",
// "util/symbol.h", "ui/helpline.h", <inttypes.h>, and <signal.h>.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_int, c_long, c_ulong, c_void};

type size_t = usize;
type u64 = u64;
type s64 = i64;
type GType = c_ulong;
type gboolean = c_int;

const ANN_COL__PERCENT: c_int = 0;
const ANN_COL__OFFSET: c_int = 1;
const ANN_COL__LINE: c_int = 2;
const MAX_ANN_COLS: usize = 3;

const FALSE: c_int = 0;
const TRUE: c_int = 1;
const G_TYPE_STRING: GType = 64;
const GTK_WINDOW_TOPLEVEL: c_int = 0;
const GTK_POLICY_AUTOMATIC: c_int = 1;
const GTK_WIN_POS_CENTER: c_int = 1;
const SIGSEGV: c_int = 11;
const SIGFPE: c_int = 8;
const SIGINT: c_int = 2;
const SIGQUIT: c_int = 3;
const SIGTERM: c_int = 15;

static col_names: [*const c_char; MAX_ANN_COLS] = [
    b"Overhead\0".as_ptr() as *const c_char,
    b"Offset\0".as_ptr() as *const c_char,
    b"Line\0".as_ptr() as *const c_char,
];

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct annotated_line {
    pub node: list_head,
    pub offset: s64,
    pub line: *mut c_char,
}

#[repr(C)]
pub struct disasm_line {
    pub al: annotated_line,
}

#[repr(C)]
pub struct annotated_source {
    pub source: list_head,
}

#[repr(C)]
pub struct annotation {
    pub src: *mut annotated_source,
}

#[repr(C)]
pub struct sym_hist {
    pub nr_samples: u64,
}

#[repr(C)]
pub struct sym_hist_entry {
    pub nr_samples: u64,
}

#[repr(C)]
pub struct symbol {
    pub start: u64,
    pub name: *const c_char,
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
pub struct map_symbol {
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hist_browser_timer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hist_entry {
    pub ms: map_symbol,
}

#[repr(C)]
pub struct symbol_conf_t {
    pub event_group: bool,
}

#[repr(C)]
pub struct perf_gtk_context {
    pub main_window: *mut GtkWidget,
    pub notebook: *mut GtkWidget,
}

#[repr(C)]
pub struct GtkWidget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkCellRenderer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkListStore {
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
pub struct GtkTreeModel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkContainer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkWindow {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkScrolledWindow {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkNotebook {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkBox {
    _private: [u8; 0],
}

extern "C" {
    static mut symbol_conf: symbol_conf_t;
    static mut pgctx: *mut perf_gtk_context;

    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn signal(sig: c_int, handler: unsafe extern "C" fn(c_int)) -> unsafe extern "C" fn(c_int);

    fn perf_gtk__get_percent_color(percent: c_double) -> *const c_char;
    fn symbol__annotation(sym: *mut symbol) -> *mut annotation;
    fn annotation__histogram(notes: *mut annotation, evsel: *const evsel) -> *mut sym_hist;
    fn annotated_source__hist_entry(
        src: *mut annotated_source,
        evsel: *const evsel,
        offset: s64,
    ) -> *mut sym_hist_entry;
    fn map__rip_2objdump(map: *mut map, rip: u64) -> u64;
    fn map__dso(map: *mut map) -> *mut dso;
    fn dso__annotate_warned(dso: *mut dso) -> c_int;
    fn dso__set_annotate_warned(dso: *mut dso);
    fn symbol__annotate(ms: *mut map_symbol, evsel: *mut evsel, arg: *mut c_void) -> c_int;
    fn symbol__strerror_disassemble(
        ms: *mut map_symbol,
        err: c_int,
        buf: *mut c_char,
        buflen: size_t,
    );
    fn symbol__calc_percent(sym: *mut symbol, evsel: *mut evsel);
    fn ui__error(fmt: *const c_char, ...);
    fn perf_gtk__is_active_context(pgctx: *mut perf_gtk_context) -> c_int;
    fn perf_gtk__signal(sig: c_int);
    fn perf_gtk__activate_context(window: *mut GtkWidget) -> *mut perf_gtk_context;
    fn perf_gtk__setup_info_bar() -> *mut GtkWidget;
    fn perf_gtk__setup_statusbar() -> *mut GtkWidget;
    fn perf_gtk__resize_window(window: *mut GtkWidget);
    fn perf_gtk__deactivate_context(pgctx: *mut *mut perf_gtk_context);
    fn evsel__is_group_event(evsel: *mut evsel) -> c_int;
    fn evsel__leader(evsel: *mut evsel) -> *mut evsel;
    fn evsel__next_group_evsel(evsel: *mut evsel) -> *mut evsel;
    fn disasm_line__free(dl: *mut disasm_line);
    fn list_del_init(entry: *mut list_head);

    fn g_markup_escape_text(text: *const c_char, length: c_long) -> *mut c_char;
    fn g_free(mem: *mut c_void);
    fn g_object_unref(object: *mut c_void);
    fn g_signal_connect_data(
        instance: *mut c_void,
        detailed_signal: *const c_char,
        c_handler: *mut c_void,
        data: *mut c_void,
        destroy_data: *mut c_void,
        connect_flags: c_int,
    ) -> c_ulong;

    fn gtk_list_store_newv(n_columns: c_int, types: *mut GType) -> *mut GtkListStore;
    fn gtk_tree_view_new() -> *mut GtkWidget;
    fn gtk_cell_renderer_text_new() -> *mut GtkCellRenderer;
    fn gtk_tree_view_insert_column_with_attributes(
        tree_view: *mut GtkTreeView,
        position: c_int,
        title: *const c_char,
        cell: *mut GtkCellRenderer,
        first_property_name: *const c_char,
        ...
    ) -> c_int;
    fn gtk_tree_view_set_model(tree_view: *mut GtkTreeView, model: *mut GtkTreeModel);
    fn gtk_list_store_append(list_store: *mut GtkListStore, iter: *mut GtkTreeIter);
    fn gtk_list_store_set(list_store: *mut GtkListStore, iter: *mut GtkTreeIter, ...);
    fn gtk_container_add(container: *mut GtkContainer, widget: *mut GtkWidget);
    fn gtk_window_new(kind: c_int) -> *mut GtkWidget;
    fn gtk_window_set_title(window: *mut GtkWindow, title: *const c_char);
    fn gtk_main_quit();
    fn gtk_vbox_new(homogeneous: gboolean, spacing: c_int) -> *mut GtkWidget;
    fn gtk_notebook_new() -> *mut GtkWidget;
    fn gtk_box_pack_start(
        box_: *mut GtkBox,
        child: *mut GtkWidget,
        expand: gboolean,
        fill: gboolean,
        padding: c_int,
    );
    fn gtk_scrolled_window_new(hadjustment: *mut c_void, vadjustment: *mut c_void) -> *mut GtkWidget;
    fn gtk_label_new(str_: *const c_char) -> *mut GtkWidget;
    fn gtk_scrolled_window_set_policy(
        scrolled_window: *mut GtkScrolledWindow,
        hscrollbar_policy: c_int,
        vscrollbar_policy: c_int,
    );
    fn gtk_notebook_append_page(
        notebook: *mut GtkNotebook,
        child: *mut GtkWidget,
        tab_label: *mut GtkWidget,
    ) -> c_int;
    fn gtk_widget_show_all(widget: *mut GtkWidget);
    fn gtk_window_set_position(window: *mut GtkWindow, position: c_int);
    fn gtk_main();
}

unsafe fn GTK_TREE_VIEW(widget: *mut GtkWidget) -> *mut GtkTreeView {
    widget as *mut GtkTreeView
}

unsafe fn GTK_TREE_MODEL(store: *mut GtkListStore) -> *mut GtkTreeModel {
    store as *mut GtkTreeModel
}

unsafe fn GTK_CONTAINER(widget: *mut GtkWidget) -> *mut GtkContainer {
    widget as *mut GtkContainer
}

unsafe fn GTK_WINDOW(widget: *mut GtkWidget) -> *mut GtkWindow {
    widget as *mut GtkWindow
}

unsafe fn GTK_SCROLLED_WINDOW(widget: *mut GtkWidget) -> *mut GtkScrolledWindow {
    widget as *mut GtkScrolledWindow
}

unsafe fn GTK_NOTEBOOK(widget: *mut GtkWidget) -> *mut GtkNotebook {
    widget as *mut GtkNotebook
}

unsafe fn GTK_BOX(widget: *mut GtkWidget) -> *mut GtkBox {
    widget as *mut GtkBox
}

unsafe fn list_entry_disasm_line(ptr: *mut list_head) -> *mut disasm_line {
    ptr as *mut disasm_line
}

unsafe fn perf_gtk__get_percent(
    buf: *mut c_char,
    size: size_t,
    sym: *mut symbol,
    dl: *mut disasm_line,
    evsel: *const evsel,
) -> c_int {
    let mut percent: c_double = 0.0;
    let mut ret: c_int = 0;
    let mut nr_samples: u64 = 0;

    strcpy(buf, b"\0".as_ptr() as *const c_char);

    if (*dl).al.offset == -1 as s64 {
        return 0;
    }

    let notes = symbol__annotation(sym);
    let symhist = annotation__histogram(notes, evsel);
    let entry = annotated_source__hist_entry((*notes).src, evsel, (*dl).al.offset);
    if !entry.is_null() {
        nr_samples = (*entry).nr_samples;
    }

    if !symbol_conf.event_group && nr_samples == 0 {
        return 0;
    }

    percent = 100.0 * nr_samples as c_double / (*symhist).nr_samples as c_double;

    let markup = perf_gtk__get_percent_color(percent);
    if !markup.is_null() {
        ret += scnprintf(buf, size, b"%s\0".as_ptr() as *const c_char, markup);
    }
    ret += scnprintf(
        buf.add(ret as usize),
        size.wrapping_sub(ret as usize),
        b"%6.2f%%\0".as_ptr() as *const c_char,
        percent,
    );
    if !markup.is_null() {
        ret += scnprintf(
            buf.add(ret as usize),
            size.wrapping_sub(ret as usize),
            b"</span>\0".as_ptr() as *const c_char,
        );
    }

    ret
}

unsafe fn perf_gtk__get_offset(
    buf: *mut c_char,
    size: size_t,
    ms: *mut map_symbol,
    dl: *mut disasm_line,
) -> c_int {
    let start: u64 = map__rip_2objdump((*ms).map, (*(*ms).sym).start);

    strcpy(buf, b"\0".as_ptr() as *const c_char);

    if (*dl).al.offset == -1 as s64 {
        return 0;
    }

    scnprintf(
        buf,
        size,
        b"%lx\0".as_ptr() as *const c_char,
        start.wrapping_add((*dl).al.offset as u64),
    )
}

unsafe fn perf_gtk__get_line(buf: *mut c_char, size: size_t, dl: *mut disasm_line) -> c_int {
    let mut ret: c_int = 0;
    let line = g_markup_escape_text((*dl).al.line, -1);
    let mut markup: *const c_char = b"<span fgcolor='gray'>\0".as_ptr() as *const c_char;

    strcpy(buf, b"\0".as_ptr() as *const c_char);

    if line.is_null() {
        return 0;
    }

    if (*dl).al.offset != -1 as s64 {
        markup = core::ptr::null();
    }

    if !markup.is_null() {
        ret += scnprintf(buf, size, b"%s\0".as_ptr() as *const c_char, markup);
    }
    ret += scnprintf(
        buf.add(ret as usize),
        size.wrapping_sub(ret as usize),
        b"%s\0".as_ptr() as *const c_char,
        line,
    );
    if !markup.is_null() {
        ret += scnprintf(
            buf.add(ret as usize),
            size.wrapping_sub(ret as usize),
            b"</span>\0".as_ptr() as *const c_char,
        );
    }

    g_free(line as *mut c_void);
    ret
}

unsafe fn perf_gtk__annotate_symbol(
    window: *mut GtkWidget,
    ms: *mut map_symbol,
    evsel: *mut evsel,
    _hbt: *mut hist_browser_timer,
) -> c_int {
    let sym = (*ms).sym;
    let notes = symbol__annotation(sym);
    let mut col_types: [GType; MAX_ANN_COLS] = [0; MAX_ANN_COLS];
    let mut s: [c_char; 512] = [0; 512];

    for i in 0..MAX_ANN_COLS {
        col_types[i] = G_TYPE_STRING;
    }
    let store = gtk_list_store_newv(MAX_ANN_COLS as c_int, col_types.as_mut_ptr());

    let view = gtk_tree_view_new();
    let renderer = gtk_cell_renderer_text_new();

    for i in 0..MAX_ANN_COLS {
        gtk_tree_view_insert_column_with_attributes(
            GTK_TREE_VIEW(view),
            -1,
            col_names[i],
            renderer,
            b"markup\0".as_ptr() as *const c_char,
            i as c_int,
            core::ptr::null_mut::<c_void>(),
        );
    }

    gtk_tree_view_set_model(GTK_TREE_VIEW(view), GTK_TREE_MODEL(store));
    g_object_unref(GTK_TREE_MODEL(store) as *mut c_void);

    let head = &mut (*(*notes).src).source as *mut list_head;
    let mut node = (*head).next;
    while node != head {
        let pos = list_entry_disasm_line(node);
        node = (*node).next;

        let mut iter: GtkTreeIter = core::mem::zeroed();
        let mut ret: c_int = 0;

        gtk_list_store_append(store, &mut iter);

        if evsel__is_group_event(evsel) != 0 {
            let mut cur_evsel = evsel__leader(evsel);
            while !cur_evsel.is_null() {
                ret += perf_gtk__get_percent(
                    s.as_mut_ptr().add(ret as usize),
                    s.len().wrapping_sub(ret as usize),
                    sym,
                    pos,
                    cur_evsel,
                );
                ret += scnprintf(
                    s.as_mut_ptr().add(ret as usize),
                    s.len().wrapping_sub(ret as usize),
                    b" \0".as_ptr() as *const c_char,
                );
                cur_evsel = evsel__next_group_evsel(cur_evsel);
            }
        } else {
            ret = perf_gtk__get_percent(s.as_mut_ptr(), s.len(), sym, pos, evsel);
        }

        if ret != 0 {
            gtk_list_store_set(store, &mut iter, ANN_COL__PERCENT, s.as_ptr(), -1);
        }
        if perf_gtk__get_offset(s.as_mut_ptr(), s.len(), ms, pos) != 0 {
            gtk_list_store_set(store, &mut iter, ANN_COL__OFFSET, s.as_ptr(), -1);
        }
        if perf_gtk__get_line(s.as_mut_ptr(), s.len(), pos) != 0 {
            gtk_list_store_set(store, &mut iter, ANN_COL__LINE, s.as_ptr(), -1);
        }
    }

    gtk_container_add(GTK_CONTAINER(window), view);

    let mut node = (*head).next;
    while node != head {
        let pos = list_entry_disasm_line(node);
        let n = (*node).next;
        list_del_init(&mut (*pos).al.node);
        disasm_line__free(pos);
        node = n;
    }

    0
}

unsafe fn symbol__gtk_annotate(
    ms: *mut map_symbol,
    evsel: *mut evsel,
    hbt: *mut hist_browser_timer,
) -> c_int {
    let dso = map__dso((*ms).map);
    let sym = (*ms).sym;
    let window: *mut GtkWidget;
    let notebook: *mut GtkWidget;
    let scrolled_window: *mut GtkWidget;
    let tab_label: *mut GtkWidget;
    let err: c_int;

    if dso__annotate_warned(dso) != 0 {
        return -1;
    }

    err = symbol__annotate(ms, evsel, core::ptr::null_mut());
    if err != 0 {
        let mut msg: [c_char; 8192] = [0; 8192];

        dso__set_annotate_warned(dso);
        symbol__strerror_disassemble(ms, err, msg.as_mut_ptr(), msg.len());
        ui__error(
            b"Couldn't annotate %s: %s\n\0".as_ptr() as *const c_char,
            (*sym).name,
            msg.as_ptr(),
        );
        return -1;
    }

    symbol__calc_percent(sym, evsel);

    if perf_gtk__is_active_context(pgctx) != 0 {
        window = (*pgctx).main_window;
        notebook = (*pgctx).notebook;
    } else {
        let vbox: *mut GtkWidget;
        let infobar: *mut GtkWidget;
        let statbar: *mut GtkWidget;

        signal(SIGSEGV, perf_gtk__signal);
        signal(SIGFPE, perf_gtk__signal);
        signal(SIGINT, perf_gtk__signal);
        signal(SIGQUIT, perf_gtk__signal);
        signal(SIGTERM, perf_gtk__signal);

        window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
        gtk_window_set_title(GTK_WINDOW(window), b"perf annotate\0".as_ptr() as *const c_char);

        g_signal_connect_data(
            window as *mut c_void,
            b"delete_event\0".as_ptr() as *const c_char,
            gtk_main_quit as *mut c_void,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            0,
        );

        pgctx = perf_gtk__activate_context(window);
        if pgctx.is_null() {
            return -1;
        }

        vbox = gtk_vbox_new(FALSE, 0);
        notebook = gtk_notebook_new();
        (*pgctx).notebook = notebook;

        gtk_box_pack_start(GTK_BOX(vbox), notebook, TRUE, TRUE, 0);

        infobar = perf_gtk__setup_info_bar();
        if !infobar.is_null() {
            gtk_box_pack_start(GTK_BOX(vbox), infobar, FALSE, FALSE, 0);
        }

        statbar = perf_gtk__setup_statusbar();
        gtk_box_pack_start(GTK_BOX(vbox), statbar, FALSE, FALSE, 0);

        gtk_container_add(GTK_CONTAINER(window), vbox);
    }

    scrolled_window = gtk_scrolled_window_new(core::ptr::null_mut(), core::ptr::null_mut());
    tab_label = gtk_label_new((*sym).name);

    gtk_scrolled_window_set_policy(
        GTK_SCROLLED_WINDOW(scrolled_window),
        GTK_POLICY_AUTOMATIC,
        GTK_POLICY_AUTOMATIC,
    );

    gtk_notebook_append_page(GTK_NOTEBOOK(notebook), scrolled_window, tab_label);

    perf_gtk__annotate_symbol(scrolled_window, ms, evsel, hbt);
    0
}

#[no_mangle]
pub unsafe extern "C" fn hist_entry__gtk_annotate(
    he: *mut hist_entry,
    evsel: *mut evsel,
    hbt: *mut hist_browser_timer,
) -> c_int {
    symbol__gtk_annotate(&mut (*he).ms, evsel, hbt)
}

#[no_mangle]
pub unsafe extern "C" fn perf_gtk__show_annotations() {
    let window: *mut GtkWidget;

    if perf_gtk__is_active_context(pgctx) == 0 {
        return;
    }

    window = (*pgctx).main_window;
    gtk_widget_show_all(window);

    perf_gtk__resize_window(window);
    gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);

    gtk_main();

    perf_gtk__deactivate_context(&mut pgctx);
}
