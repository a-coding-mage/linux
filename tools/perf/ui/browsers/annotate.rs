// SPDX-License-Identifier: GPL-2.0
// Translated from perf/ui/browsers/annotate.c.
// C include dependencies are preserved here as external declarations/opaque types.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

type size_t = usize;
type bool_t = bool;
type u8 = u8;
type u32 = u32;
type u64 = u64;
type s64 = i64;
type off_t = i64;

const SEEK_CUR: c_int = 1;
const SEEK_SET: c_int = 0;
const PATH_MAX: usize = 4096;
const BUFSIZ: usize = 8192;
const SYM_TITLE_MAX_SIZE: usize = PATH_MAX + 64;

const HE_COLORSET_SELECTED: c_int = 1;
const HE_COLORSET_TOP: c_int = 2;
const HE_COLORSET_MEDIUM: c_int = 3;
const HE_COLORSET_NORMAL: c_int = 4;
const HE_COLORSET_JUMP_ARROWS: c_int = 5;
const HE_COLORSET_ROOT: c_int = 6;

const K_ENTER: c_int = 13;
const K_TIMER: c_int = -1;
const K_TAB: c_int = 9;
const K_UNTAB: c_int = -9;
const K_F1: c_int = 265;
const K_RIGHT: c_int = 261;
const K_LEFT: c_int = 260;
const K_ESC: c_int = 27;
const NO_ADDR: u64 = !0u64;

const PERCENT_HITS_LOCAL: c_int = 0;
const PERCENT_HITS_GLOBAL: c_int = 1;
const PERCENT_PERIOD_LOCAL: c_int = 2;
const PERCENT_PERIOD_GLOBAL: c_int = 3;
const ANNOTATION__MIN_OFFSET_LEVEL: c_int = 0;
const ANNOTATION__MAX_OFFSET_LEVEL: c_int = 2;

const fn CTRL(c: u8) -> c_int {
    (c & 0x1f) as c_int
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct rb_node {
    pub rb_left: *mut rb_node,
    pub rb_right: *mut rb_node,
    pub rb_parent_color: c_ulong,
}

#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

const RB_ROOT: rb_root = rb_root { rb_node: ptr::null_mut() };

#[repr(C)]
pub struct ui_browser {
    pub refresh: Option<unsafe extern "C" fn(*mut ui_browser) -> c_uint>,
    pub seek: Option<unsafe extern "C" fn(*mut ui_browser, off_t, c_int)>,
    pub write: Option<unsafe extern "C" fn(*mut ui_browser, *mut c_void, c_int)>,
    pub filter: Option<unsafe extern "C" fn(*mut ui_browser, *mut c_void) -> bool_t>,
    pub extra_title_lines: c_int,
    pub priv_: *mut c_void,
    pub use_navkeypressed: bool_t,
    pub navkeypressed: bool_t,
    pub width: c_uint,
    pub rows: c_uint,
    pub height: c_uint,
    pub top_idx: u32,
    pub index: u32,
    pub top: *mut c_void,
    pub entries: *mut list_head,
    pub nr_entries: c_uint,
}

#[repr(C)]
pub struct rb_node_wrapper {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct arch {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct hist_browser_timer {
    pub refresh: c_int,
    pub timer: Option<unsafe extern "C" fn(*mut c_void)>,
    pub arg: *mut c_void,
}

#[repr(C)]
pub struct hists {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct map {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    pub evlist: *mut evlist,
}

#[repr(C)]
pub struct symbol {
    pub name: *const c_char,
    pub start: u64,
}

#[repr(C)]
pub struct map_symbol {
    pub thread: *mut thread,
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct hist_entry {
    pub hists: *mut hists,
    pub thread: *mut thread,
    pub cpumode: c_int,
    pub ms: map_symbol,
}

#[repr(C)]
pub struct debuginfo {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct hashmap {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct hashmap_entry {
    pub pvalue: *mut c_void,
}

#[repr(C)]
pub struct annotation_options {
    pub hide_src_code: bool_t,
    pub jump_arrows: bool_t,
    pub percent_type: c_int,
    pub annotate_src: bool_t,
    pub hide_src_code_on_title: bool_t,
    pub code_with_type: bool_t,
    pub use_offset: bool_t,
    pub offset_level: c_int,
    pub show_nr_jumps: bool_t,
    pub show_linenr: bool_t,
    pub show_minmax_cycle: bool_t,
}

#[repr(C)]
pub struct symbol_conf_t {
    pub show_total_period: bool_t,
    pub show_nr_samples: bool_t,
}

#[repr(C)]
pub struct annotation_data {
    pub percent: [c_double; 4],
}

#[repr(C)]
pub struct cycles_info {
    pub ipc: c_int,
}

#[repr(C)]
pub struct annotation_line {
    pub node: list_head,
    pub rb_node: rb_node,
    pub offset: s64,
    pub idx: u32,
    pub idx_asm: c_int,
    pub data_nr: c_int,
    pub data: *mut annotation_data,
    pub cycles: *mut cycles_info,
    pub line: *const c_char,
    pub fileloc: *const c_char,
}

#[repr(C)]
pub struct ins {
    pub name: *const c_char,
    pub ops: *mut c_void,
}

#[repr(C)]
pub struct locked_ops {
    pub ins: ins,
}

#[repr(C)]
pub struct target_ops {
    pub offset: u64,
    pub outside: bool_t,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct disasm_ops {
    pub locked: locked_ops,
    pub target: target_ops,
}

#[repr(C)]
pub struct disasm_line {
    pub al: annotation_line,
    pub ins: ins,
    pub ops: disasm_ops,
}

#[repr(C)]
pub struct widths {
    pub addr: c_int,
    pub max_line_len: c_uint,
}

#[repr(C)]
pub struct annotated_source {
    pub source: list_head,
    pub widths: widths,
    pub max_jump_sources: c_int,
    pub nr_entries: c_uint,
    pub nr_asm_entries: c_uint,
    pub tried_source: bool_t,
}

#[repr(C)]
pub struct annotation {
    pub src: *mut annotated_source,
}

#[repr(C)]
pub struct annotation_write_ops {
    pub first_line: bool_t,
    pub current_entry: bool_t,
    pub change_color: bool_t,
    pub width: c_uint,
    pub obj: *mut c_void,
    pub set_color: Option<unsafe extern "C" fn(*mut c_void, c_int) -> c_int>,
    pub set_percent_color: Option<unsafe extern "C" fn(*mut c_void, c_double, bool_t)>,
    pub set_jumps_percent_color: Option<unsafe extern "C" fn(*mut c_void, c_int, bool_t) -> c_int>,
    pub printf: Option<unsafe extern "C" fn(*mut c_void, *const c_char, ...)>,
    pub write_graph: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
}

#[repr(C)]
pub struct annotation_print_data {
    pub he: *mut hist_entry,
    pub arch: *const arch,
    pub evsel: *mut evsel,
    pub dbg: *mut debuginfo,
    pub type_hash: *mut hashmap,
}

#[repr(C)]
pub struct annotate_browser {
    pub b: ui_browser,
    pub entries: rb_root,
    pub curr_hot: *mut rb_node,
    pub selection: *mut annotation_line,
    pub arch: *const arch,
    /*
     * perf top can delete hist_entry anytime.  Callers should make sure
     * its lifetime.
     */
    pub he: *mut hist_entry,
    pub dbg: *mut debuginfo,
    pub evsel: *mut evsel,
    pub type_hash: *mut hashmap,
    pub searching_backwards: bool_t,
    pub search_bf: [c_char; 128],
}

/* A copy of target hist_entry for perf top. */
static mut annotate_he: hist_entry = unsafe { MaybeUninit::<hist_entry>::zeroed().assume_init() };

extern "C" {
    static mut annotate_opts: annotation_options;
    static mut symbol_conf: symbol_conf_t;

    fn symbol__annotation(sym: *mut symbol) -> *mut annotation;
    fn annotation_line__filter(al: *mut annotation_line) -> bool_t;
    fn ui_browser__set_color(browser: *mut c_void, color: c_int) -> c_int;
    fn ui_browser__write_graph(browser: *mut c_void, graph: c_int);
    fn ui_browser__set_percent_color(browser: *mut c_void, percent: c_double, current: bool_t);
    fn ui_browser__vprintf(browser: *mut c_void, fmt: *const c_char, args: *mut c_void);
    fn ui_browser__is_current_entry(browser: *mut ui_browser, row: c_int) -> bool_t;
    fn annotation_line__write(al: *mut annotation_line, notes: *mut annotation, ops: *mut annotation_write_ops, apd: *mut annotation_print_data);
    fn ins__is_lock(ins: *const ins) -> bool_t;
    fn ins__is_fused(arch: *const arch, name: *const c_char, cursor_name: *const c_char) -> bool_t;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn disasm_line__is_valid_local_jump(cursor: *mut disasm_line, sym: *mut symbol) -> bool_t;
    fn annotation__pcnt_width(notes: *mut annotation) -> u8;
    fn annotation__br_cntr_width() -> u8;
    fn annotation__cycles_width(notes: *mut annotation) -> c_int;
    fn annotated_source__get_line(src: *mut annotated_source, offset: u64) -> *mut annotation_line;
    fn ui_helpline__printf(fmt: *const c_char, ...);
    fn ui_helpline__puts(s: *const c_char);
    fn ui_helpline__push(s: *const c_char);
    fn ui_helpline__pop();
    fn ui_helpline__fpush(fmt: *const c_char, ...);
    fn __ui_browser__line_arrow(browser: *mut ui_browser, column: c_int, from: c_uint, to: c_uint);
    fn ui_browser__mark_fused(browser: *mut ui_browser, column: c_int, from: c_uint, diff: c_int, down: bool_t);
    fn ui_browser__list_head_refresh(browser: *mut ui_browser) -> c_int;
    fn __ui_browser__vline(browser: *mut ui_browser, column: c_int, start: c_int, end: c_uint);
    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node);
    fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);
    fn rb_last(root: *mut rb_root) -> *mut rb_node;
    fn rb_first(root: *mut rb_root) -> *mut rb_node;
    fn rb_prev(node: *mut rb_node) -> *mut rb_node;
    fn rb_next(node: *mut rb_node) -> *mut rb_node;
    fn ui_browser__refresh_dimensions(browser: *mut ui_browser);
    fn annotation__lock(notes: *mut annotation);
    fn annotation__unlock(notes: *mut annotation);
    fn symbol__calc_percent(sym: *mut symbol, evsel: *mut evsel);
    fn annotation_data__percent(data: *mut annotation_data, percent_type: c_int) -> c_double;
    fn annotated_source__purge(src: *mut annotated_source);
    fn symbol__annotate2(ms: *mut map_symbol, evsel: *mut evsel, arch: *mut *const arch) -> c_int;
    fn ui__warning(fmt: *const c_char, ...);
    fn ui_browser__reset_index(browser: *mut ui_browser);
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn map__dso(map: *mut map) -> *mut dso;
    fn percent_type_str(percent_type: c_int) -> *const c_char;
    fn ui_browser__gotorc_title(browser: *mut ui_browser, y: c_int, x: c_int);
    fn ui_browser__write_nstring(browser: *mut ui_browser, s: *const c_char, n: c_uint);
    fn symbol__hists(sym: *mut symbol, nr: c_int) -> bool_t;
    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn __hist_entry__tui_annotate(he: *mut hist_entry, ms: *mut map_symbol, evsel: *mut evsel, hbt: *mut hist_browser_timer, al_addr: u64) -> c_int;
    fn ins__is_jump(ins: *const ins) -> bool_t;
    fn ui_browser__input_window(title: *const c_char, text: *const c_char, input: *mut c_char, exit_msg: *const c_char, delay_secs: c_int) -> c_int;
    fn ui_browser__show(browser: *mut ui_browser, title: *mut c_char, help: *const c_char) -> c_int;
    fn hists__scnprintf_title(hists: *mut hists, bf: *mut c_char, size: size_t) -> c_int;
    fn dso__debuginfo_warned(dso: *mut dso) -> bool_t;
    fn dso__set_debuginfo_warned(dso: *mut dso);
    fn dso__set_annotate_warned(dso: *mut dso);
    fn symbol__strerror_disassemble(ms: *mut map_symbol, err: c_int, msg: *mut c_char, size: size_t);
    fn ui__error(fmt: *const c_char, ...);
    fn evsel__hists(evsel: *mut evsel) -> *mut hists;
    fn annotation_br_cntr_abbr_list(text: *mut *mut c_char, evsel: *mut evsel, full: bool_t);
    fn ui_browser__run(browser: *mut ui_browser, delay_secs: c_int) -> c_int;
    fn symbol__annotate_decay_histogram(sym: *mut symbol, evsel: *mut evsel);
    fn ui_browser__help_window(browser: *mut ui_browser, text: *const c_char);
    fn script_browse(a: *mut c_void, b: *mut c_void);
    fn annotation__update_column_widths(notes: *mut annotation);
    fn ins__is_ret(ins: *const ins) -> bool_t;
    fn map_symbol__annotation_dump(ms: *mut map_symbol, evsel: *mut evsel, he: *mut hist_entry);
    fn annotation__toggle_full_addr(notes: *mut annotation, ms: *mut map_symbol);
    fn dso__debuginfo(dso: *mut dso) -> *mut debuginfo;
    fn hashmap__new(hash: unsafe extern "C" fn(c_long, *mut c_void) -> size_t, equal: unsafe extern "C" fn(c_long, c_long, *mut c_void) -> bool_t, ctx: *mut c_void) -> *mut hashmap;
    fn ui_browser__warn_unhandled_hotkey(browser: *mut ui_browser, key: c_int, delay_secs: c_int, msg: *const c_char);
    fn ui_browser__hide(browser: *mut ui_browser);
    fn free(ptr: *mut c_void);
    fn SLang_reset_tty();
    fn SLang_init_tty(a: c_int, b: c_int, c: c_int);
    fn SLtty_set_suspend_state(state: bool_t);
    fn list_empty(head: *const list_head) -> c_int;
    fn symbol__is_annotate2(sym: *mut symbol) -> bool_t;
    fn map_symbol__get_arch(ms: *mut map_symbol, arch: *mut *const arch) -> c_int;
    fn thread__get(thread: *mut thread) -> *mut thread;
    fn map_symbol__copy(dst: *mut map_symbol, src: *mut map_symbol);
    fn debuginfo__delete(dbg: *mut debuginfo);
    fn hashmap__free(map: *mut hashmap);
    fn thread__zput(thread: *mut thread);
    fn map_symbol__exit(ms: *mut map_symbol);
}

unsafe fn IS_ERR_OR_NULL<T>(ptr: *mut T) -> bool_t {
    ptr.is_null()
}

unsafe fn WARN_ON(v: c_int) -> c_int {
    v
}

unsafe fn RB_EMPTY_NODE(node: *mut rb_node) -> bool_t {
    (*node).rb_parent_color == node as c_ulong
}

unsafe fn RB_CLEAR_NODE(node: *mut rb_node) {
    (*node).rb_parent_color = node as c_ulong;
}

unsafe fn container_of_browser(browser: *mut ui_browser) -> *mut annotate_browser {
    browser as *mut annotate_browser
}

unsafe fn rb_entry_annotation_line(node: *mut rb_node) -> *mut annotation_line {
    let uninit = MaybeUninit::<annotation_line>::uninit();
    let base = uninit.as_ptr();
    let off = (&(*base).rb_node as *const _ as usize) - (base as usize);
    (node as *mut u8).sub(off) as *mut annotation_line
}

unsafe fn disasm_line(al: *mut annotation_line) -> *mut disasm_line {
    al as *mut disasm_line
}

unsafe fn list_first_entry_annotation_line(head: *mut list_head) -> *mut annotation_line {
    (*head).next as *mut annotation_line
}

unsafe fn list_entry_annotation_line(node: *mut list_head) -> *mut annotation_line {
    node as *mut annotation_line
}

unsafe fn list_prev_entry_disasm_line(cursor: *mut disasm_line) -> *mut disasm_line {
    (*(*cursor).al.node.prev).prev as *mut disasm_line
}

unsafe extern "C" fn type_hash(key: c_long, _ctx: *mut c_void) -> size_t {
    key as size_t
}

unsafe extern "C" fn type_equal(key1: c_long, key2: c_long, _ctx: *mut c_void) -> bool_t {
    key1 == key2
}

unsafe fn browser__annotation(browser: *mut ui_browser) -> *mut annotation {
    let ms = (*browser).priv_ as *mut map_symbol;
    symbol__annotation((*ms).sym)
}

unsafe extern "C" fn disasm_line__filter(_browser: *mut ui_browser, entry: *mut c_void) -> bool_t {
    let al = list_entry_annotation_line(entry as *mut list_head);
    annotation_line__filter(al)
}

unsafe fn ui_browser__jumps_percent_color(browser: *mut ui_browser, nr: c_int, current: bool_t) -> c_int {
    let notes = browser__annotation(browser);

    if current && (!(*browser).use_navkeypressed || (*browser).navkeypressed) {
        return HE_COLORSET_SELECTED;
    }
    if nr == (*(*notes).src).max_jump_sources {
        return HE_COLORSET_TOP;
    }
    if nr > 1 {
        return HE_COLORSET_MEDIUM;
    }
    HE_COLORSET_NORMAL
}

unsafe extern "C" fn ui_browser__set_jumps_percent_color(browser: *mut c_void, nr: c_int, current: bool_t) -> c_int {
    let color = ui_browser__jumps_percent_color(browser as *mut ui_browser, nr, current);
    ui_browser__set_color(browser, color)
}

unsafe extern "C" fn annotate_browser__set_color(browser: *mut c_void, color: c_int) -> c_int {
    ui_browser__set_color(browser, color)
}

unsafe extern "C" fn annotate_browser__write_graph(browser: *mut c_void, graph: c_int) {
    ui_browser__write_graph(browser, graph);
}

unsafe extern "C" fn annotate_browser__set_percent_color(browser: *mut c_void, percent: c_double, current: bool_t) {
    ui_browser__set_percent_color(browser, percent, current);
}

unsafe extern "C" fn annotate_browser__printf(browser: *mut c_void, fmt: *const c_char, mut _args: ...) {
    // Rust cannot portably forward a C variadic va_list here without target-specific va_list support.
    // Preserve the call site interface; the dependency remains external in the original C implementation.
    let _ = (browser, fmt);
}

unsafe extern "C" fn annotate_browser__write(browser: *mut ui_browser, entry: *mut c_void, row: c_int) {
    let ab = container_of_browser(browser);
    let notes = browser__annotation(browser);
    let al = list_entry_annotation_line(entry as *mut list_head);
    let is_current_entry = ui_browser__is_current_entry(browser, row);
    let mut ops = annotation_write_ops {
        first_line: row == 0,
        current_entry: is_current_entry,
        change_color: !annotate_opts.hide_src_code
            && (!is_current_entry || ((*browser).use_navkeypressed && !(*browser).navkeypressed)),
        width: (*browser).width,
        obj: browser as *mut c_void,
        set_color: Some(annotate_browser__set_color),
        set_percent_color: Some(annotate_browser__set_percent_color),
        set_jumps_percent_color: Some(ui_browser__set_jumps_percent_color),
        printf: Some(annotate_browser__printf),
        write_graph: Some(annotate_browser__write_graph),
    };
    let mut apd = annotation_print_data {
        he: (*ab).he,
        arch: (*ab).arch,
        evsel: (*ab).evsel,
        dbg: (*ab).dbg,
        type_hash: ptr::null_mut(),
    };

    /* The scroll bar isn't being used */
    if !(*browser).navkeypressed {
        ops.width += 1;
    }

    if !IS_ERR_OR_NULL((*ab).type_hash) {
        apd.type_hash = (*ab).type_hash;
    }

    annotation_line__write(al, notes, &mut ops, &mut apd);

    if ops.current_entry {
        (*ab).selection = al;
    }
}

unsafe fn is_fused(ab: *mut annotate_browser, cursor: *mut disasm_line) -> c_int {
    let mut pos = list_prev_entry_disasm_line(cursor);
    let name: *const c_char;
    let mut diff = 1;

    while !pos.is_null() && (*pos).al.offset == -1 {
        pos = list_prev_entry_disasm_line(pos);
        if !annotate_opts.hide_src_code {
            diff += 1;
        }
    }

    if pos.is_null() {
        return 0;
    }

    if ins__is_lock(&(*pos).ins) {
        name = (*pos).ops.locked.ins.name;
    } else {
        name = (*pos).ins.name;
    }

    if name.is_null() || (*cursor).ins.name.is_null() {
        return 0;
    }

    if ins__is_fused((*ab).arch, name, (*cursor).ins.name) {
        return diff;
    }
    0
}

unsafe fn annotate_browser__draw_current_jump(browser: *mut ui_browser) {
    let ab = container_of_browser(browser);
    let cursor = disasm_line((*ab).selection);
    let target: *mut annotation_line;
    let from: c_uint;
    let to: c_uint;
    let ms = (*ab).b.priv_ as *mut map_symbol;
    let sym = (*ms).sym;
    let notes = symbol__annotation(sym);
    let pcnt_width = annotation__pcnt_width(notes);
    let cntr_width = annotation__br_cntr_width();
    let width: c_int;
    let mut diff = 0;

    /* PLT symbols contain external offsets */
    if !strstr((*sym).name, b"@plt\0".as_ptr() as *const c_char).is_null() {
        return;
    }

    if !disasm_line__is_valid_local_jump(cursor, sym) {
        return;
    }

    /*
     * This first was seen with a gcc function, _cpp_lex_token, that
     * has the usual jumps:
     *
     *  │1159e6c: ↓ jne    115aa32 <_cpp_lex_token@@Base+0xf92>
     *
     * I.e. jumps to a label inside that function (_cpp_lex_token), and
     * those works, but also this kind:
     *
     *  │1159e8b: ↓ jne    c469be <cpp_named_operator2name@@Base+0xa72>
     *
     *  I.e. jumps to another function, outside _cpp_lex_token, which
     *  are not being correctly handled generating as a side effect references
     *  to ab->offset[] entries that are set to NULL, so to make this code
     *  more robust, check that here.
     *
     *  A proper fix for will be put in place, looking at the function
     *  name right after the '<' token and probably treating this like a
     *  'call' instruction.
     */
    target = annotated_source__get_line((*notes).src, (*cursor).ops.target.offset);
    if target.is_null() {
        ui_helpline__printf(
            b"WARN: jump target inconsistency, press 'o', notes->offsets[%#x] = NULL\n\0".as_ptr() as *const c_char,
            (*cursor).ops.target.offset as c_uint,
        );
        return;
    }

    if annotate_opts.hide_src_code {
        from = (*cursor).al.idx_asm as c_uint;
        to = (*target).idx_asm as c_uint;
    } else {
        from = (*cursor).al.idx as c_uint;
        to = (*target).idx as c_uint;
    }

    width = annotation__cycles_width(notes);

    ui_browser__set_color(browser as *mut c_void, HE_COLORSET_JUMP_ARROWS);
    __ui_browser__line_arrow(
        browser,
        pcnt_width as c_int + 2 + (*(*notes).src).widths.addr + width + cntr_width as c_int,
        from,
        to,
    );

    diff = is_fused(ab, cursor);
    if diff > 0 {
        ui_browser__mark_fused(
            browser,
            pcnt_width as c_int + 3 + (*(*notes).src).widths.addr + width + cntr_width as c_int,
            from.wrapping_sub(diff as c_uint),
            diff,
            to > from,
        );
    }
}

unsafe extern "C" fn annotate_browser__refresh(browser: *mut ui_browser) -> c_uint {
    let notes = browser__annotation(browser);
    let ret = ui_browser__list_head_refresh(browser);
    let pcnt_width = annotation__pcnt_width(notes) as c_int;

    if annotate_opts.jump_arrows {
        annotate_browser__draw_current_jump(browser);
    }

    ui_browser__set_color(browser as *mut c_void, HE_COLORSET_NORMAL);
    __ui_browser__vline(browser, pcnt_width, 0, (*browser).rows - 1);
    ret as c_uint
}

unsafe fn disasm__cmp(a: *mut annotation_line, b: *mut annotation_line, percent_type: c_int) -> c_double {
    let mut i = 0;

    while i < (*a).data_nr {
        let ad = (*a).data.add(i as usize);
        let bd = (*b).data.add(i as usize);
        if (*ad).percent[percent_type as usize] == (*bd).percent[percent_type as usize] {
            i += 1;
            continue;
        }
        return (*ad).percent[percent_type as usize] - (*bd).percent[percent_type as usize];
    }
    0.0
}

unsafe fn disasm_rb_tree__insert(browser: *mut annotate_browser, al: *mut annotation_line) {
    let root = &mut (*browser).entries as *mut rb_root;
    let mut p = &mut (*root).rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = ptr::null_mut();

    while !(*p).is_null() {
        parent = *p;
        let l = rb_entry_annotation_line(parent);

        if disasm__cmp(al, l, annotate_opts.percent_type) < 0.0 {
            p = &mut (**p).rb_left;
        } else {
            p = &mut (**p).rb_right;
        }
    }
    rb_link_node(&mut (*al).rb_node, parent, p);
    rb_insert_color(&mut (*al).rb_node, root);
}

unsafe fn annotate_browser__set_top(browser: *mut annotate_browser, mut pos: *mut annotation_line, idx: u32) {
    let mut back: c_uint;

    ui_browser__refresh_dimensions(&mut (*browser).b);
    back = (*browser).b.height / 2;
    (*browser).b.top_idx = idx;
    (*browser).b.index = idx;

    while (*browser).b.top_idx != 0 && back != 0 {
        pos = list_entry_annotation_line((*pos).node.prev);

        if annotation_line__filter(pos) {
            continue;
        }

        (*browser).b.top_idx -= 1;
        back -= 1;
    }

    (*browser).b.top = pos as *mut c_void;
    (*browser).b.navkeypressed = true;
}

unsafe fn annotate_browser__set_rb_top(browser: *mut annotate_browser, nd: *mut rb_node) {
    let pos = rb_entry_annotation_line(nd);
    let mut idx = (*pos).idx;

    if annotate_opts.hide_src_code {
        idx = (*pos).idx_asm as u32;
    }
    annotate_browser__set_top(browser, pos, idx);
    (*browser).curr_hot = nd;
}

unsafe fn annotate_browser__calc_percent(browser: *mut annotate_browser, evsel: *mut evsel) {
    let ms = (*browser).b.priv_ as *mut map_symbol;
    let sym = (*ms).sym;
    let notes = symbol__annotation(sym);
    let mut pos: *mut disasm_line;

    (*browser).entries = RB_ROOT;

    annotation__lock(notes);

    symbol__calc_percent(sym, evsel);

    pos = (*(*notes).src).source.next as *mut disasm_line;
    while &mut (*pos).al.node as *mut list_head != &mut (*(*notes).src).source as *mut list_head {
        let mut max_percent = 0.0;
        let mut i = 0;

        if (*pos).al.offset == -1 {
            RB_CLEAR_NODE(&mut (*pos).al.rb_node);
            pos = (*pos).al.node.next as *mut disasm_line;
            continue;
        }

        while i < (*pos).al.data_nr {
            let percent = annotation_data__percent((*pos).al.data.add(i as usize), annotate_opts.percent_type);
            if max_percent < percent {
                max_percent = percent;
            }
            i += 1;
        }

        if max_percent < 0.01 && ((*pos).al.cycles.is_null() || (*(*pos).al.cycles).ipc == 0) {
            RB_CLEAR_NODE(&mut (*pos).al.rb_node);
            pos = (*pos).al.node.next as *mut disasm_line;
            continue;
        }
        disasm_rb_tree__insert(browser, &mut (*pos).al);
        pos = (*pos).al.node.next as *mut disasm_line;
    }
    annotation__unlock(notes);

    (*browser).curr_hot = rb_last(&mut (*browser).entries);
}

unsafe fn annotate_browser__find_new_asm_line(browser: *mut annotate_browser, idx_asm: c_int) -> *mut annotation_line {
    let head = (*browser).b.entries;
    let mut al = (*head).next as *mut annotation_line;

    /* find an annotation line in the new list with the same idx_asm */
    while &mut (*al).node as *mut list_head != head {
        if (*al).idx_asm == idx_asm {
            return al;
        }
        al = (*al).node.next as *mut annotation_line;
    }

    /* There are no asm lines */
    ptr::null_mut()
}

unsafe fn annotate_browser__find_next_asm_line(browser: *mut annotate_browser, al: *mut annotation_line) -> *mut annotation_line {
    let mut it = al;

    /* find next asm line */
    it = (*it).node.next as *mut annotation_line;
    while &mut (*it).node as *mut list_head != (*browser).b.entries {
        if (*it).idx_asm >= 0 {
            return it;
        }
        it = (*it).node.next as *mut annotation_line;
    }

    /* no asm line found forwards, try backwards */
    it = (*al).node.prev as *mut annotation_line;
    while &mut (*it).node as *mut list_head != (*browser).b.entries {
        if (*it).idx_asm >= 0 {
            return it;
        }
        it = (*it).node.prev as *mut annotation_line;
    }

    /* There are no asm lines */
    ptr::null_mut()
}

unsafe fn annotation__has_source(notes: *mut annotation) -> bool_t {
    let mut found_asm = false;
    let head = &mut (*(*notes).src).source as *mut list_head;
    let mut al = (*head).next as *mut annotation_line;

    /* Let's skip the first non-asm lines which present regardless of source. */
    while &mut (*al).node as *mut list_head != head {
        if (*al).offset >= 0 {
            found_asm = true;
            break;
        }
        al = (*al).node.next as *mut annotation_line;
    }

    if found_asm {
        /* After assembly lines, any line without offset means source. */
        al = (*al).node.next as *mut annotation_line;
        while &mut (*al).node as *mut list_head != head {
            if (*al).offset == -1 {
                return true;
            }
            al = (*al).node.next as *mut annotation_line;
        }
    }
    false
}

unsafe fn annotate_browser__toggle_source(browser: *mut annotate_browser, evsel: *mut evsel) -> bool_t {
    let notes = browser__annotation(&mut (*browser).b);
    let mut al: *mut annotation_line;
    let mut offset = ((*browser).b.index - (*browser).b.top_idx) as off_t;

    if (*browser).b.nr_entries == 0 {
        return false;
    }

    ((*browser).b.seek.unwrap())(&mut (*browser).b, offset, SEEK_CUR);
    al = list_entry_annotation_line((*browser).b.top as *mut list_head);

    if !annotate_opts.annotate_src {
        annotate_opts.annotate_src = true;
    }

    /*
     * It's about to get source code annotation for the first time.
     * Drop the existing annotation_lines and get the new one with source.
     * And then move to the original line at the same asm index.
     */
    if annotate_opts.hide_src_code && !(*(*notes).src).tried_source {
        let ms = (*browser).b.priv_ as *mut map_symbol;
        let orig_idx_asm = (*al).idx_asm;

        /* annotate again with source code info */
        annotate_opts.hide_src_code = false;
        annotated_source__purge((*notes).src);
        symbol__annotate2(ms, evsel, &mut (*browser).arch);
        annotate_opts.hide_src_code = true;

        /* should be after annotated_source__purge() */
        (*(*notes).src).tried_source = true;

        if !annotation__has_source(notes) {
            ui__warning(b"Annotation has no source code.\0".as_ptr() as *const c_char);
        }

        (*browser).b.entries = &mut (*(*notes).src).source;
        al = annotate_browser__find_new_asm_line(browser, orig_idx_asm);
        if al.is_null() {
            al = list_first_entry_annotation_line(&mut (*(*notes).src).source);
        }
        ((*browser).b.seek.unwrap())(&mut (*browser).b, (*al).idx_asm as off_t, SEEK_SET);
    }

    if annotate_opts.hide_src_code {
        if (*al).idx_asm < offset as c_int {
            offset = (*al).idx as off_t;
        }

        (*browser).b.nr_entries = (*(*notes).src).nr_entries;
        annotate_opts.hide_src_code = false;
        ((*browser).b.seek.unwrap())(&mut (*browser).b, -offset, SEEK_CUR);
        (*browser).b.top_idx = ((*al).idx as off_t - offset) as u32;
        (*browser).b.index = (*al).idx;
    } else {
        if (*al).idx_asm < 0 {
            /* move cursor to next asm line */
            al = annotate_browser__find_next_asm_line(browser, al);
            if al.is_null() {
                ((*browser).b.seek.unwrap())(&mut (*browser).b, -offset, SEEK_CUR);
                return false;
            }
        }

        if (*al).idx_asm < offset as c_int {
            offset = (*al).idx_asm as off_t;
        }

        (*browser).b.nr_entries = (*(*notes).src).nr_asm_entries;
        annotate_opts.hide_src_code = true;
        ((*browser).b.seek.unwrap())(&mut (*browser).b, -offset, SEEK_CUR);
        (*browser).b.top_idx = ((*al).idx_asm as off_t - offset) as u32;
        (*browser).b.index = (*al).idx_asm as u32;
    }

    if annotate_opts.hide_src_code_on_title {
        annotate_opts.hide_src_code_on_title = false;
    }

    true
}

unsafe fn annotate_browser__show_full_location(browser: *mut ui_browser) {
    let ab = container_of_browser(browser);
    let cursor = disasm_line((*ab).selection);
    let al = &mut (*cursor).al as *mut annotation_line;

    if (*al).offset != -1 {
        ui_helpline__puts(b"Only available for source code lines.\0".as_ptr() as *const c_char);
    } else if (*al).fileloc.is_null() {
        ui_helpline__puts(b"No source file location.\0".as_ptr() as *const c_char);
    } else {
        let mut help_line = [0 as c_char; SYM_TITLE_MAX_SIZE];
        sprintf(help_line.as_mut_ptr(), b"Source file location: %s\0".as_ptr() as *const c_char, (*al).fileloc);
        ui_helpline__puts(help_line.as_ptr());
    }
}

unsafe fn ui_browser__init_asm_mode(browser: *mut ui_browser) {
    let notes = browser__annotation(browser);
    (*browser).nr_entries = (*(*notes).src).nr_asm_entries;
    ui_browser__reset_index(browser);
}

unsafe fn sym_title(sym: *mut symbol, map: *mut map, title: *mut c_char, sz: size_t, percent_type: c_int) -> c_int {
    snprintf(
        title,
        sz,
        b"%s  %s [Percent: %s] %s\0".as_ptr() as *const c_char,
        (*sym).name,
        dso__long_name(map__dso(map)),
        percent_type_str(percent_type),
        if annotate_opts.code_with_type { b"[Type]\0".as_ptr() } else { b"\0".as_ptr() } as *const c_char,
    )
}

unsafe fn annotate_browser__show_function_title(browser: *mut annotate_browser) {
    let b = &mut (*browser).b as *mut ui_browser;
    let ms = (*b).priv_ as *mut map_symbol;
    let sym = (*ms).sym;
    let mut title = [0 as c_char; SYM_TITLE_MAX_SIZE];

    sym_title(sym, (*ms).map, title.as_mut_ptr(), title.len(), annotate_opts.percent_type);

    ui_browser__gotorc_title(b, 0, 0);
    ui_browser__set_color(b as *mut c_void, HE_COLORSET_ROOT);
    ui_browser__write_nstring(b, title.as_ptr(), (*b).width + 1);
}

unsafe fn annotate_browser__callq(browser: *mut annotate_browser, evsel: *mut evsel, hbt: *mut hist_browser_timer) -> bool_t {
    let ms = (*browser).b.priv_ as *mut map_symbol;
    let mut target_ms: map_symbol = MaybeUninit::zeroed().assume_init();
    let dl = disasm_line((*browser).selection);
    let notes: *mut annotation;

    if (*dl).ops.target.sym.is_null() {
        ui_helpline__puts(b"The called function was not found.\0".as_ptr() as *const c_char);
        return true;
    }

    notes = symbol__annotation((*dl).ops.target.sym);
    annotation__lock(notes);

    if !symbol__hists((*dl).ops.target.sym, evlist__nr_entries((*evsel).evlist)) {
        annotation__unlock(notes);
        ui__warning(
            b"Not enough memory for annotating '%s' symbol!\n\0".as_ptr() as *const c_char,
            (*(*dl).ops.target.sym).name,
        );
        return true;
    }

    target_ms.thread = (*ms).thread;
    target_ms.map = (*ms).map;
    target_ms.sym = (*dl).ops.target.sym;
    annotation__unlock(notes);
    __hist_entry__tui_annotate((*browser).he, &mut target_ms, evsel, hbt, NO_ADDR);

    /*
     * The annotate_browser above changed the title with the target function
     * and now it's back to the original function.  Refresh the header line
     * for the original function again.
     */
    annotate_browser__show_function_title(browser);
    true
}

unsafe fn annotate_browser__find_offset(browser: *mut annotate_browser, offset: s64, idx: *mut s64) -> *mut disasm_line {
    let notes = browser__annotation(&mut (*browser).b);
    let head = &mut (*(*notes).src).source as *mut list_head;
    let mut pos = (*head).next as *mut disasm_line;

    *idx = 0;
    while &mut (*pos).al.node as *mut list_head != head {
        if (*pos).al.offset == offset {
            return pos;
        }
        if !annotation_line__filter(&mut (*pos).al) {
            *idx += 1;
        }
        pos = (*pos).al.node.next as *mut disasm_line;
    }

    ptr::null_mut()
}

unsafe fn annotate_browser__jump(browser: *mut annotate_browser, evsel: *mut evsel, hbt: *mut hist_browser_timer) -> bool_t {
    let mut dl = disasm_line((*browser).selection);
    let offset: u64;
    let mut idx: s64 = 0;

    if !ins__is_jump(&(*dl).ins) {
        return false;
    }

    if (*dl).ops.target.outside {
        annotate_browser__callq(browser, evsel, hbt);
        return true;
    }

    offset = (*dl).ops.target.offset;
    dl = annotate_browser__find_offset(browser, offset as s64, &mut idx);
    if dl.is_null() {
        ui_helpline__printf(b"Invalid jump offset: %lx\0".as_ptr() as *const c_char, offset);
        return true;
    }

    annotate_browser__set_top(browser, &mut (*dl).al, idx as u32);

    true
}

unsafe fn annotate_browser__find_string(browser: *mut annotate_browser, s: *mut c_char, idx: *mut s64) -> *mut annotation_line {
    let notes = browser__annotation(&mut (*browser).b);
    let mut al = (*browser).selection;

    *idx = (*browser).b.index as s64;
    al = (*al).node.next as *mut annotation_line;
    while &mut (*al).node as *mut list_head != &mut (*(*notes).src).source as *mut list_head {
        if annotation_line__filter(al) {
            al = (*al).node.next as *mut annotation_line;
            continue;
        }

        *idx += 1;

        if !(*al).line.is_null() && !strstr((*al).line, s).is_null() {
            return al;
        }
        al = (*al).node.next as *mut annotation_line;
    }

    ptr::null_mut()
}

unsafe fn __annotate_browser__search(browser: *mut annotate_browser) -> bool_t {
    let mut idx: s64 = 0;

    let al = annotate_browser__find_string(browser, (*browser).search_bf.as_mut_ptr(), &mut idx);
    if al.is_null() {
        ui_helpline__puts(b"String not found!\0".as_ptr() as *const c_char);
        return false;
    }

    annotate_browser__set_top(browser, al, idx as u32);
    (*browser).searching_backwards = false;
    true
}

unsafe fn annotate_browser__find_string_reverse(browser: *mut annotate_browser, s: *mut c_char, idx: *mut s64) -> *mut annotation_line {
    let notes = browser__annotation(&mut (*browser).b);
    let mut al = (*browser).selection;

    *idx = (*browser).b.index as s64;
    al = (*al).node.prev as *mut annotation_line;
    while &mut (*al).node as *mut list_head != &mut (*(*notes).src).source as *mut list_head {
        if annotation_line__filter(al) {
            al = (*al).node.prev as *mut annotation_line;
            continue;
        }

        *idx -= 1;

        if !(*al).line.is_null() && !strstr((*al).line, s).is_null() {
            return al;
        }
        al = (*al).node.prev as *mut annotation_line;
    }

    ptr::null_mut()
}

unsafe fn __annotate_browser__search_reverse(browser: *mut annotate_browser) -> bool_t {
    let mut idx: s64 = 0;

    let al = annotate_browser__find_string_reverse(browser, (*browser).search_bf.as_mut_ptr(), &mut idx);
    if al.is_null() {
        ui_helpline__puts(b"String not found!\0".as_ptr() as *const c_char);
        return false;
    }

    annotate_browser__set_top(browser, al, idx as u32);
    (*browser).searching_backwards = true;
    true
}

unsafe fn annotate_browser__search_window(browser: *mut annotate_browser, delay_secs: c_int) -> bool_t {
    if ui_browser__input_window(
        b"Search\0".as_ptr() as *const c_char,
        b"String: \0".as_ptr() as *const c_char,
        (*browser).search_bf.as_mut_ptr(),
        b"ENTER: OK, ESC: Cancel\0".as_ptr() as *const c_char,
        delay_secs * 2,
    ) != K_ENTER
        || (*browser).search_bf[0] == 0
    {
        return false;
    }

    true
}

unsafe fn annotate_browser__search(browser: *mut annotate_browser, delay_secs: c_int) -> bool_t {
    if annotate_browser__search_window(browser, delay_secs) {
        return __annotate_browser__search(browser);
    }

    false
}

unsafe fn annotate_browser__continue_search(browser: *mut annotate_browser, delay_secs: c_int) -> bool_t {
    if (*browser).search_bf[0] == 0 {
        return annotate_browser__search(browser, delay_secs);
    }

    __annotate_browser__search(browser)
}

unsafe fn annotate_browser__search_reverse(browser: *mut annotate_browser, delay_secs: c_int) -> bool_t {
    if annotate_browser__search_window(browser, delay_secs) {
        return __annotate_browser__search_reverse(browser);
    }

    false
}

unsafe fn annotate_browser__continue_search_reverse(browser: *mut annotate_browser, delay_secs: c_int) -> bool_t {
    if (*browser).search_bf[0] == 0 {
        return annotate_browser__search_reverse(browser, delay_secs);
    }

    __annotate_browser__search_reverse(browser)
}

unsafe fn annotate_browser__show(browser: *mut annotate_browser, title: *mut c_char, help: *const c_char) -> c_int {
    if ui_browser__show(&mut (*browser).b, title, help) < 0 {
        return -1;
    }

    annotate_browser__show_function_title(browser);
    0
}

unsafe fn switch_percent_type(opts: *mut annotation_options, base: bool_t) {
    match (*opts).percent_type {
        PERCENT_HITS_LOCAL => {
            if base {
                (*opts).percent_type = PERCENT_PERIOD_LOCAL;
            } else {
                (*opts).percent_type = PERCENT_HITS_GLOBAL;
            }
        }
        PERCENT_HITS_GLOBAL => {
            if base {
                (*opts).percent_type = PERCENT_PERIOD_GLOBAL;
            } else {
                (*opts).percent_type = PERCENT_HITS_LOCAL;
            }
        }
        PERCENT_PERIOD_LOCAL => {
            if base {
                (*opts).percent_type = PERCENT_HITS_LOCAL;
            } else {
                (*opts).percent_type = PERCENT_PERIOD_GLOBAL;
            }
        }
        PERCENT_PERIOD_GLOBAL => {
            if base {
                (*opts).percent_type = PERCENT_HITS_GLOBAL;
            } else {
                (*opts).percent_type = PERCENT_PERIOD_LOCAL;
            }
        }
        _ => {
            WARN_ON(1);
        }
    }
}

unsafe fn annotate__scnprintf_title(hists: *mut hists, bf: *mut c_char, size: size_t) -> c_int {
    let mut printed = hists__scnprintf_title(hists, bf, size);

    if !annotate_opts.hide_src_code_on_title {
        printed += scnprintf(
            bf.add(printed as usize),
            size - printed as usize,
            b" [source: %s]\0".as_ptr() as *const c_char,
            if annotate_opts.hide_src_code { b"OFF\0".as_ptr() } else { b"On\0".as_ptr() } as *const c_char,
        );
    }

    printed
}

unsafe fn annotate_browser__debuginfo_warning(browser: *mut annotate_browser) {
    let ms = (*browser).b.priv_ as *mut map_symbol;
    let dso = map__dso((*ms).map);

    if (*browser).dbg.is_null() && annotate_opts.code_with_type && !dso__debuginfo_warned(dso) {
        ui__warning(
            b"DWARF debuginfo not found.\n\nData-type in this DSO will not be displayed.\nPlease make sure to have debug information.\0"
                .as_ptr() as *const c_char,
        );
        dso__set_debuginfo_warned(dso);
    }
}

unsafe fn annotate_browser__curr_hot_offset(browser: *mut annotate_browser) -> s64 {
    let mut al: *mut annotation_line = ptr::null_mut();

    if !(*browser).curr_hot.is_null() {
        al = rb_entry_annotation_line((*browser).curr_hot);
    }

    if !al.is_null() { (*al).offset } else { 0 }
}

unsafe fn annotate_browser__symbol_annotate_error(browser: *mut annotate_browser, err: c_int) {
    let ms = (*browser).b.priv_ as *mut map_symbol;
    let sym = (*ms).sym;
    let dso = map__dso((*ms).map);
    let mut msg = [0 as c_char; BUFSIZ];

    dso__set_annotate_warned(dso);
    symbol__strerror_disassemble(ms, err, msg.as_mut_ptr(), msg.len());
    ui__error(b"Couldn't annotate %s:\n%s\0".as_ptr() as *const c_char, (*sym).name, msg.as_ptr());
}

unsafe fn annotate_browser__run(browser: *mut annotate_browser, evsel: *mut evsel, hbt: *mut hist_browser_timer) -> c_int {
    let mut nd: *mut rb_node = ptr::null_mut();
    let hists = evsel__hists(evsel);
    let ms = (*browser).b.priv_ as *mut map_symbol;
    let sym = (*ms).sym;
    let notes = symbol__annotation((*ms).sym);
    let help = b"Press 'h' for help on key bindings\0".as_ptr() as *const c_char;
    let delay_secs = if !hbt.is_null() { (*hbt).refresh } else { 0 };
    let mut br_cntr_text: *mut c_char = ptr::null_mut();
    let mut title = [0 as c_char; 256];
    let mut key: c_int;

    annotate__scnprintf_title(hists, title.as_mut_ptr(), title.len());
    if annotate_browser__show(browser, title.as_mut_ptr(), help) < 0 {
        return -1;
    }

    annotate_browser__calc_percent(browser, evsel);

    if !(*browser).selection.is_null() {
        (*browser).curr_hot = &mut (*(*browser).selection).rb_node;
        (*browser).b.use_navkeypressed = false;
    }

    if !(*browser).curr_hot.is_null() {
        annotate_browser__set_rb_top(browser, (*browser).curr_hot);
        (*browser).b.navkeypressed = false;
    }

    nd = (*browser).curr_hot;

    annotation_br_cntr_abbr_list(&mut br_cntr_text, evsel, false);

    annotate_browser__debuginfo_warning(browser);

    loop {
        key = ui_browser__run(&mut (*browser).b, delay_secs);

        if delay_secs != 0 {
            annotate_browser__calc_percent(browser, evsel);
            /*
             * Current line focus got out of the list of most active
             * lines, NULL it so that if TAB|UNTAB is pressed, we
             * move to curr_hot (current hottest line).
             */
            if !nd.is_null() && RB_EMPTY_NODE(nd) {
                nd = ptr::null_mut();
            }
        }

        match key {
            K_TIMER => {
                if !hbt.is_null() {
                    if let Some(timer) = (*hbt).timer {
                        timer((*hbt).arg);
                    }
                }

                if delay_secs != 0 {
                    symbol__annotate_decay_histogram(sym, evsel);
                    annotate__scnprintf_title(hists, title.as_mut_ptr(), title.len());
                    annotate_browser__show(browser, title.as_mut_ptr(), help);
                }
                continue;
            }
            K_TAB => {
                if !nd.is_null() {
                    nd = rb_prev(nd);
                    if nd.is_null() {
                        nd = rb_last(&mut (*browser).entries);
                    }
                } else {
                    nd = (*browser).curr_hot;
                }
            }
            K_UNTAB => {
                if !nd.is_null() {
                    nd = rb_next(nd);
                    if nd.is_null() {
                        nd = rb_first(&mut (*browser).entries);
                    }
                } else {
                    nd = (*browser).curr_hot;
                }
            }
            K_F1 | 104 => {
                ui_browser__help_window(&mut (*browser).b, b"UP/DOWN/PGUP\nPGDN/SPACE    Navigate\n</>           Move to prev/next symbol\nq/ESC/CTRL+C  Exit\n\nENTER         Go to target\nH             Go to hottest instruction\nTAB/shift+TAB Cycle thru hottest instructions\nj             Toggle showing jump to target arrows\nJ             Toggle showing number of jump sources on targets\nn             Search next string\no             Toggle disassembler output/simplified view\nO             Bump offset level (jump targets -> +call -> all -> cycle thru)\ns             Toggle source code view\nt             Circulate percent, total period, samples view\nc             Show min/max cycle\n/             Search string\nk             Toggle line numbers\nl             Show full source file location\nP             Print to [symbol_name].annotation file.\nr             Run available scripts\np             Toggle percent type [local/global]\nb             Toggle percent base [period/hits]\nB             Branch counter abbr list (Optional)\n?             Search string backwards\nf             Toggle showing offsets to full address\nT             Toggle data type display\n\0".as_ptr() as *const c_char);
                continue;
            }
            114 => {
                script_browse(ptr::null_mut(), ptr::null_mut());
                annotate_browser__show(browser, title.as_mut_ptr(), help);
                continue;
            }
            107 => {
                annotate_opts.show_linenr = !annotate_opts.show_linenr;
                continue;
            }
            108 => {
                annotate_browser__show_full_location(&mut (*browser).b);
                continue;
            }
            72 => {
                nd = (*browser).curr_hot;
            }
            115 => {
                let mut al: *mut annotation_line;
                let offset = annotate_browser__curr_hot_offset(browser);

                if annotate_browser__toggle_source(browser, evsel) {
                    ui_helpline__puts(help);
                }

                /* Update the annotation browser's rb_tree, and reset the nd */
                annotate_browser__calc_percent(browser, evsel);
                /* Try to find the same asm line as before */
                al = annotated_source__get_line((*notes).src, offset as u64);
                (*browser).curr_hot = if !al.is_null() { &mut (*al).rb_node } else { ptr::null_mut() };
                nd = (*browser).curr_hot;

                annotate__scnprintf_title(hists, title.as_mut_ptr(), title.len());
                annotate_browser__show(browser, title.as_mut_ptr(), help);
                continue;
            }
            111 => {
                annotate_opts.use_offset = !annotate_opts.use_offset;
                annotation__update_column_widths(notes);
                continue;
            }
            79 => {
                annotate_opts.offset_level += 1;
                if annotate_opts.offset_level > ANNOTATION__MAX_OFFSET_LEVEL {
                    annotate_opts.offset_level = ANNOTATION__MIN_OFFSET_LEVEL;
                }
                continue;
            }
            106 => {
                annotate_opts.jump_arrows = !annotate_opts.jump_arrows;
                continue;
            }
            74 => {
                annotate_opts.show_nr_jumps = !annotate_opts.show_nr_jumps;
                annotation__update_column_widths(notes);
                continue;
            }
            47 => {
                if annotate_browser__search(browser, delay_secs) {
                    ui_helpline__puts(help);
                }
                continue;
            }
            110 => {
                if if (*browser).searching_backwards {
                    annotate_browser__continue_search_reverse(browser, delay_secs)
                } else {
                    annotate_browser__continue_search(browser, delay_secs)
                } {
                    ui_helpline__puts(help);
                }
                continue;
            }
            63 => {
                if annotate_browser__search_reverse(browser, delay_secs) {
                    ui_helpline__puts(help);
                }
                continue;
            }
            68 => {
                static mut seq: c_int = 0;
                ui_helpline__pop();
                ui_helpline__fpush(
                    b"%d: nr_ent=%d, height=%d, idx=%d, top_idx=%d, nr_asm_entries=%d\0".as_ptr() as *const c_char,
                    seq,
                    (*browser).b.nr_entries,
                    (*browser).b.height,
                    (*browser).b.index,
                    (*browser).b.top_idx,
                    (*(*notes).src).nr_asm_entries,
                );
                seq += 1;
                continue;
            }
            K_ENTER | K_RIGHT => {
                let dl = disasm_line((*browser).selection);

                if (*browser).selection.is_null() {
                    ui_helpline__puts(b"Huh? No selection. Report to linux-kernel@vger.kernel.org\0".as_ptr() as *const c_char);
                } else if (*(*browser).selection).offset == -1 {
                    ui_helpline__puts(b"Actions are only available for assembly lines.\0".as_ptr() as *const c_char);
                } else if (*dl).ins.ops.is_null() {
                    ui_helpline__puts(b"Actions are only available for function call/return & jump/branch instructions.\0".as_ptr() as *const c_char);
                } else if ins__is_ret(&(*dl).ins) {
                    break;
                } else if !(annotate_browser__jump(browser, evsel, hbt) || annotate_browser__callq(browser, evsel, hbt)) {
                    ui_helpline__puts(b"Actions are only available for function call/return & jump/branch instructions.\0".as_ptr() as *const c_char);
                }
                continue;
            }
            80 => {
                map_symbol__annotation_dump(ms, evsel, (*browser).he);
                continue;
            }
            116 => {
                if symbol_conf.show_total_period {
                    symbol_conf.show_total_period = false;
                    symbol_conf.show_nr_samples = true;
                } else if symbol_conf.show_nr_samples {
                    symbol_conf.show_nr_samples = false;
                } else {
                    symbol_conf.show_total_period = true;
                }
                annotation__update_column_widths(notes);
                continue;
            }
            99 => {
                if annotate_opts.show_minmax_cycle {
                    annotate_opts.show_minmax_cycle = false;
                } else {
                    annotate_opts.show_minmax_cycle = true;
                }
                annotation__update_column_widths(notes);
                continue;
            }
            112 | 98 => {
                switch_percent_type(&mut annotate_opts, key == 98);
                annotate__scnprintf_title(hists, title.as_mut_ptr(), title.len());
                annotate_browser__show(browser, title.as_mut_ptr(), help);
                continue;
            }
            66 => {
                if !br_cntr_text.is_null() {
                    ui_browser__help_window(&mut (*browser).b, br_cntr_text);
                } else {
                    ui_browser__help_window(&mut (*browser).b, b"\n The branch counter is not available.\n\0".as_ptr() as *const c_char);
                }
                continue;
            }
            102 => {
                annotation__toggle_full_addr(notes, ms);
                continue;
            }
            84 => {
                annotate_opts.code_with_type ^= true;
                if (*browser).dbg.is_null() {
                    (*browser).dbg = dso__debuginfo(map__dso((*ms).map));
                }
                if (*browser).type_hash.is_null() {
                    (*browser).type_hash = hashmap__new(type_hash, type_equal, ptr::null_mut());
                }
                annotate_browser__show(browser, title.as_mut_ptr(), help);
                annotate_browser__debuginfo_warning(browser);
                continue;
            }
            K_LEFT | 60 | 62 | K_ESC | 113 => break,
            x if x == CTRL(b'c') => break,
            _ => {
                ui_browser__warn_unhandled_hotkey(&mut (*browser).b, key, delay_secs, b", use 'h'/F1 to see actions\0".as_ptr() as *const c_char);
                continue;
            }
        }

        if !nd.is_null() {
            annotate_browser__set_rb_top(browser, nd);
        }
    }

    ui_browser__hide(&mut (*browser).b);
    free(br_cntr_text as *mut c_void);
    key
}

#[no_mangle]
pub unsafe extern "C" fn hist_entry__tui_annotate(he: *mut hist_entry, evsel: *mut evsel, hbt: *mut hist_browser_timer, al_addr: u64) -> c_int {
    /* reset abort key so that it can get Ctrl-C as a key */
    SLang_reset_tty();
    SLang_init_tty(0, 0, 0);
    SLtty_set_suspend_state(true);

    __hist_entry__tui_annotate(he, &mut (*he).ms, evsel, hbt, al_addr)
}

#[no_mangle]
pub unsafe extern "C" fn __hist_entry__tui_annotate(he: *mut hist_entry, ms: *mut map_symbol, evsel: *mut evsel, hbt: *mut hist_browser_timer, al_addr: u64) -> c_int {
    let sym = (*ms).sym;
    let notes = symbol__annotation(sym);
    let mut browser: annotate_browser = MaybeUninit::zeroed().assume_init();
    browser.b.refresh = Some(annotate_browser__refresh);
    browser.b.seek = Some(ui_browser__list_head_seek);
    browser.b.write = Some(annotate_browser__write);
    browser.b.filter = Some(disasm_line__filter);
    browser.b.extra_title_lines = 1; /* for hists__scnprintf_title() */
    browser.b.priv_ = ms as *mut c_void;
    browser.b.use_navkeypressed = true;
    browser.he = he;
    browser.evsel = evsel;

    let dso: *mut dso;
    let mut ret = -1;
    let mut err: c_int;
    let not_annotated = list_empty(&mut (*(*notes).src).source) != 0;

    if sym.is_null() {
        return -1;
    }

    dso = map__dso((*ms).map);
    if dso__annotate_warned(dso) {
        return -1;
    }

    if not_annotated || !symbol__is_annotate2(sym) {
        err = symbol__annotate2(ms, evsel, &mut browser.arch);
        if err != 0 {
            annotate_browser__symbol_annotate_error(&mut browser, err);
            return -1;
        }

        if !annotate_opts.hide_src_code {
            (*(*notes).src).tried_source = true;
            if !annotation__has_source(notes) {
                ui__warning(b"Annotation has no source code.\0".as_ptr() as *const c_char);
            }
        }
    } else {
        err = map_symbol__get_arch(ms, &mut browser.arch);
        if err != 0 {
            annotate_browser__symbol_annotate_error(&mut browser, err);
            return -1;
        }
    }

    /* Copy necessary information when it's called from perf top */
    if !hbt.is_null() && he != &mut annotate_he {
        annotate_he.hists = (*he).hists;
        annotate_he.thread = thread__get((*he).thread);
        annotate_he.cpumode = (*he).cpumode;
        map_symbol__copy(&mut annotate_he.ms, ms);

        browser.he = &mut annotate_he;
    }

    ui_helpline__push(b"Press ESC to exit\0".as_ptr() as *const c_char);

    if annotate_opts.code_with_type {
        browser.dbg = dso__debuginfo(dso);
        browser.type_hash = hashmap__new(type_hash, type_equal, ptr::null_mut());
    }

    browser.b.width = (*(*notes).src).widths.max_line_len;
    browser.b.nr_entries = (*(*notes).src).nr_entries;
    browser.b.entries = &mut (*(*notes).src).source;
    browser.b.width += 18; /* Percentage */

    if annotate_opts.hide_src_code {
        ui_browser__init_asm_mode(&mut browser.b);
    }

    /*
     * If al_addr is set, it means that there should be a line
     * intentionally selected, not based on the percentages
     * which caculated by the event sampling. In this case, we
     * convey this information into the browser selection, where
     * the selection in other cases should be empty.
     */
    if al_addr != NO_ADDR {
        let al = annotated_source__get_line((*notes).src, al_addr - (*sym).start);

        browser.selection = al;
    }

    ret = annotate_browser__run(&mut browser, evsel, hbt);

    debuginfo__delete(browser.dbg);

    if !IS_ERR_OR_NULL(browser.type_hash) {
        // hashmap__for_each_entry(browser.type_hash, cur, bkt)
        //     zfree(&cur->pvalue);
        // Iteration macro is external to this isolated file; keep the cleanup hook explicit.
        hashmap__free(browser.type_hash);
    }

    if not_annotated && !(*(*notes).src).tried_source {
        annotated_source__purge((*notes).src);
    }

    if !hbt.is_null() && he != &mut annotate_he {
        thread__zput(annotate_he.thread);
        map_symbol__exit(&mut annotate_he.ms);
    }

    ret
}

extern "C" {
    fn ui_browser__list_head_seek(browser: *mut ui_browser, offset: off_t, whence: c_int);
    fn dso__annotate_warned(dso: *mut dso) -> bool_t;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
