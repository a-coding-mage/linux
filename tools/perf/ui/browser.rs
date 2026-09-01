// SPDX-License-Identifier: GPL-2.0
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![feature(c_variadic)]

use core::ffi::{c_char, c_double, c_int, c_uint, c_void, VaList};

type bool_ = bool;
type off_t = i64;
type u16 = u16;
type u32 = u32;
type u64 = u64;

const SEEK_SET: c_int = 0;
const SEEK_CUR: c_int = 1;
const SEEK_END: c_int = 2;

const HE_COLORSET_SELECTED: c_int = 1;
const HE_COLORSET_TOP: c_int = 2;
const HE_COLORSET_MEDIUM: c_int = 3;
const HE_COLORSET_NORMAL: c_int = 4;
const HE_COLORSET_JUMP_ARROWS: c_int = 5;
const HE_COLORSET_ADDR: c_int = 6;
const HE_COLORSET_ROOT: c_int = 7;

const MIN_RED: c_double = 5.0;
const MIN_GREEN: c_double = 0.5;

const K_RESIZE: c_int = -2;
const K_ENTER: c_int = 10;
const K_DOWN: c_int = 1001;
const K_UP: c_int = 1002;
const K_LEFT: c_int = 1003;
const K_RIGHT: c_int = 1004;
const K_PGDN: c_int = 1005;
const K_PGUP: c_int = 1006;
const K_HOME: c_int = 1007;
const K_END: c_int = 1008;

const SLSMG_DIAMOND_CHAR: c_int = 0;
const SLSMG_CKBRD_CHAR: c_int = 0;
const SLSMG_LLCORN_CHAR: c_int = 0;
const SLSMG_ULCORN_CHAR: c_int = 0;
const SLSMG_HLINE_CHAR: c_int = 0;
const SLSMG_RARROW_CHAR: c_int = 0;
const SLSMG_LTEE_CHAR: c_int = 0;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
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
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ui_browser {
    pub entries: *mut c_void,
    pub top: *mut c_void,
    pub title: *mut c_char,
    pub helpline: *mut c_char,
    pub no_samples_msg: *const c_char,
    pub refresh: Option<unsafe extern "C" fn(*mut ui_browser) -> c_uint>,
    pub write: Option<unsafe extern "C" fn(*mut ui_browser, *mut c_void, c_int)>,
    pub seek: Option<unsafe extern "C" fn(*mut ui_browser, off_t, c_int)>,
    pub filter: Option<unsafe extern "C" fn(*mut ui_browser, *mut c_void) -> bool_>,
    pub refresh_dimensions: Option<unsafe extern "C" fn(*mut ui_browser)>,
    pub nr_entries: u32,
    pub rows: u32,
    pub width: c_int,
    pub height: c_int,
    pub y: c_int,
    pub x: c_int,
    pub extra_title_lines: c_int,
    pub current_color: c_int,
    pub top_idx: u64,
    pub index: u64,
    pub horiz_scroll: u32,
    pub columns: u32,
    pub use_navkeypressed: bool_,
    pub navkeypressed: bool_,
}

#[repr(C)]
struct ui_browser_colorset {
    name: *const c_char,
    fg: *const c_char,
    bg: *const c_char,
    colorset: c_int,
}

unsafe extern "C" {
    static mut SLtt_Screen_Cols: c_int;
    static mut SLtt_Screen_Rows: c_int;
    static mut ui__lock: mutex;
    static ui_helpline__current: *const c_char;

    fn SLsmg_set_color(color: c_int);
    fn SLsmg_gotorc(r: c_int, c: c_int);
    fn SLsmg_write_nstring(msg: *const c_char, width: c_uint);
    fn SLsmg_vprintf(fmt: *const c_char, args: VaList);
    fn SLsmg_set_char_set(n: c_int);
    fn SLsmg_write_char(ch: c_int);
    fn SLsmg_fill_region(r: c_int, c: c_int, rows: c_int, cols: c_int, ch: c_int);
    fn SLsmg_refresh();
    fn SLsmg_draw_vline(len: c_uint);
    fn SLsmg_draw_hline(len: c_uint);
    fn SLtt_set_color(obj: c_int, name: *const c_char, fg: *const c_char, bg: *const c_char);

    fn rb_first(root: *mut rb_root) -> *mut rb_node;
    fn rb_last(root: *mut rb_root) -> *mut rb_node;
    fn rb_next(node: *mut rb_node) -> *mut rb_node;
    fn rb_prev(node: *mut rb_node) -> *mut rb_node;

    fn ui__refresh_dimensions(force: bool_);
    fn ui__question_window(title: *const c_char, text: *const c_char, exit_msg: *const c_char, delay_secs: c_int) -> c_int;
    fn ui__help_window(text: *const c_char) -> c_int;
    fn ui__dialog_yesno(text: *const c_char) -> c_int;
    fn __ui__info_window(title: *const c_char, text: *const c_char, exit_msg: *const c_char);
    fn ui__getch(delay_secs: c_int) -> c_int;
    fn ui_helpline__vpush(fmt: *const c_char, args: VaList);
    fn ui_helpline__push(msg: *const c_char);
    fn ui_helpline__pop();
    fn ui_helpline__puts(msg: *const c_char);
    fn key_name(key: c_int, buf: *mut c_char, size: usize);
    fn perf_config(fn_: Option<unsafe extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int>, data: *mut c_void) -> c_int;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn vasprintf(strp: *mut *mut c_char, fmt: *const c_char, ap: VaList) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strchr(s: *mut c_char, c: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool_;
    fn skip_spaces(str_: *const c_char) -> *mut c_char;
    fn toupper(c: c_int) -> c_int;
    fn free(ptr: *mut c_void);
    fn zfree(ptr: *mut *mut c_char);
}

unsafe fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn ui_browser__percent_color(browser: *mut ui_browser, percent: c_double, current: bool_) -> c_int {
    if current && (!(*browser).use_navkeypressed || (*browser).navkeypressed) {
        return HE_COLORSET_SELECTED;
    }
    if percent >= MIN_RED {
        return HE_COLORSET_TOP;
    }
    if percent >= MIN_GREEN {
        return HE_COLORSET_MEDIUM;
    }
    HE_COLORSET_NORMAL
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__set_color(browser: *mut ui_browser, color: c_int) -> c_int {
    let ret = (*browser).current_color;
    (*browser).current_color = color;
    SLsmg_set_color(color);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__set_percent_color(browser: *mut ui_browser, percent: c_double, current: bool_) {
    let color = ui_browser__percent_color(browser, percent, current);
    ui_browser__set_color(browser, color);
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__gotorc_title(browser: *mut ui_browser, y: c_int, x: c_int) {
    SLsmg_gotorc((*browser).y + y, (*browser).x + x);
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__gotorc(browser: *mut ui_browser, y: c_int, x: c_int) {
    SLsmg_gotorc((*browser).y + y + (*browser).extra_title_lines, (*browser).x + x);
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__write_nstring(_browser: *mut ui_browser, msg: *const c_char, width: c_uint) {
    SLsmg_write_nstring(msg, width);
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__vprintf(_browser: *mut ui_browser, fmt: *const c_char, args: VaList) {
    SLsmg_vprintf(fmt, args);
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__printf(browser: *mut ui_browser, fmt: *const c_char, mut args: ...) {
    ui_browser__vprintf(browser, fmt, args.as_va_list());
}

unsafe fn ui_browser__list_head_filter_entries(browser: *mut ui_browser, mut pos: *mut list_head) -> *mut list_head {
    loop {
        if (*browser).filter.is_none() || !((*browser).filter.unwrap())(browser, pos as *mut c_void) {
            return pos;
        }
        pos = (*pos).next;
        if pos == (*browser).entries as *mut list_head {
            break;
        }
    }
    core::ptr::null_mut()
}

unsafe fn ui_browser__list_head_filter_prev_entries(browser: *mut ui_browser, mut pos: *mut list_head) -> *mut list_head {
    loop {
        if (*browser).filter.is_none() || !((*browser).filter.unwrap())(browser, pos as *mut c_void) {
            return pos;
        }
        pos = (*pos).prev;
        if pos == (*browser).entries as *mut list_head {
            break;
        }
    }
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__list_head_seek(browser: *mut ui_browser, mut offset: off_t, whence: c_int) {
    let head = (*browser).entries as *mut list_head;
    let mut pos: *mut list_head;

    if (*browser).nr_entries == 0 {
        return;
    }

    match whence {
        SEEK_SET => pos = ui_browser__list_head_filter_entries(browser, (*head).next),
        SEEK_CUR => pos = (*browser).top as *mut list_head,
        SEEK_END => pos = ui_browser__list_head_filter_prev_entries(browser, (*head).prev),
        _ => return,
    }

    assert!(!pos.is_null());

    if offset > 0 {
        while {
            let old = offset;
            offset -= 1;
            old != 0
        } {
            pos = ui_browser__list_head_filter_entries(browser, (*pos).next);
        }
    } else {
        while {
            let old = offset;
            offset += 1;
            old != 0
        } {
            pos = ui_browser__list_head_filter_prev_entries(browser, (*pos).prev);
        }
    }

    (*browser).top = pos as *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__rb_tree_seek(browser: *mut ui_browser, mut offset: off_t, whence: c_int) {
    let root = (*browser).entries as *mut rb_root;
    let mut nd: *mut rb_node;

    match whence {
        SEEK_SET => nd = rb_first(root),
        SEEK_CUR => nd = (*browser).top as *mut rb_node,
        SEEK_END => nd = rb_last(root),
        _ => return,
    }

    if offset > 0 {
        while {
            let old = offset;
            offset -= 1;
            old != 0
        } {
            nd = rb_next(nd);
        }
    } else {
        while {
            let old = offset;
            offset += 1;
            old != 0
        } {
            nd = rb_prev(nd);
        }
    }

    (*browser).top = nd as *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__rb_tree_refresh(browser: *mut ui_browser) -> c_uint {
    let mut row: c_int = 0;

    if (*browser).top.is_null() {
        (*browser).top = rb_first((*browser).entries as *mut rb_root) as *mut c_void;
    }

    let mut nd = (*browser).top as *mut rb_node;

    while !nd.is_null() {
        ui_browser__gotorc(browser, row, 0);
        ((*browser).write.unwrap())(browser, nd as *mut c_void, row);
        row += 1;
        if row as u32 == (*browser).rows {
            break;
        }
        nd = rb_next(nd);
    }

    row as c_uint
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__is_current_entry(browser: *mut ui_browser, row: c_uint) -> bool_ {
    (*browser).top_idx + row as u64 == (*browser).index
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__refresh_dimensions(browser: *mut ui_browser) {
    (*browser).width = SLtt_Screen_Cols - 1;
    (*browser).height = SLtt_Screen_Rows - 2;
    (*browser).rows = (*browser).height as u32;
    (*browser).rows = (*browser).rows.wrapping_sub((*browser).extra_title_lines as u32);
    (*browser).y = 1;
    (*browser).x = 0;
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__handle_resize(browser: *mut ui_browser) {
    ui__refresh_dimensions(false);
    ui_browser__show(browser, if (*browser).title.is_null() { cstr(b"\0") } else { (*browser).title }, ui_helpline__current);
    ui_browser__refresh(browser);
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__warning(browser: *mut ui_browser, timeout: c_int, format: *const c_char, mut args: ...) -> c_int {
    let mut text: *mut c_char = core::ptr::null_mut();
    let mut key: c_int = 0;

    let err = vasprintf(&mut text, format, args.as_va_list());

    if err < 0 {
        ui_helpline__vpush(format, args.as_va_list());
    } else {
        while {
            key = ui__question_window(cstr(b"Warning!\0"), text, cstr(b"Press any key...\0"), timeout);
            key == K_RESIZE
        } {
            ui_browser__handle_resize(browser);
        }
        free(text as *mut c_void);
    }

    key
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__warn_unhandled_hotkey(browser: *mut ui_browser, key: c_int, timeout: c_int, help: *const c_char) -> c_int {
    let mut kname = [0 as c_char; 32];

    key_name(key, kname.as_mut_ptr(), kname.len());
    ui_browser__warning(browser, timeout, cstr(b"\n'%s' key not associated%s!\n\0"), kname.as_ptr(), if help.is_null() { cstr(b"\0") } else { help })
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__help_window(browser: *mut ui_browser, text: *const c_char) -> c_int {
    let mut key: c_int;

    loop {
        key = ui__help_window(text);
        if key != K_RESIZE {
            break;
        }
        ui_browser__handle_resize(browser);
    }

    key
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__dialog_yesno(browser: *mut ui_browser, text: *const c_char) -> bool_ {
    let mut key: c_int;

    loop {
        key = ui__dialog_yesno(text);
        if key != K_RESIZE {
            break;
        }
        ui_browser__handle_resize(browser);
    }

    key == K_ENTER || toupper(key) == 'Y' as c_int
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__reset_index(browser: *mut ui_browser) {
    (*browser).top_idx = 0;
    (*browser).index = (*browser).top_idx;
    ((*browser).seek.unwrap())(browser, 0, SEEK_SET);
}

#[no_mangle]
pub unsafe extern "C" fn __ui_browser__show_title(browser: *mut ui_browser, title: *const c_char) {
    SLsmg_gotorc(0, 0);
    ui_browser__set_color(browser, HE_COLORSET_ROOT);
    ui_browser__write_nstring(browser, title, ((*browser).width + 1) as c_uint);
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__show_title(browser: *mut ui_browser, title: *const c_char) {
    mutex_lock(&raw mut ui__lock);
    __ui_browser__show_title(browser, title);
    mutex_unlock(&raw mut ui__lock);
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__show(browser: *mut ui_browser, title: *const c_char, helpline: *const c_char, mut ap: ...) -> c_int {
    if (*browser).refresh_dimensions.is_none() {
        (*browser).refresh_dimensions = Some(ui_browser__refresh_dimensions);
    }

    ((*browser).refresh_dimensions.unwrap())(browser);

    mutex_lock(&raw mut ui__lock);
    __ui_browser__show_title(browser, title);

    free((*browser).title as *mut c_void);
    (*browser).title = strdup(title);
    zfree(&mut (*browser).helpline);

    let err = vasprintf(&mut (*browser).helpline, helpline, ap.as_va_list());
    if err > 0 {
        ui_helpline__push((*browser).helpline);
    }
    mutex_unlock(&raw mut ui__lock);
    if err != 0 { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__hide(browser: *mut ui_browser) {
    mutex_lock(&raw mut ui__lock);
    ui_helpline__pop();
    zfree(&mut (*browser).helpline);
    zfree(&mut (*browser).title);
    mutex_unlock(&raw mut ui__lock);
}

unsafe fn ui_browser__scrollbar_set(browser: *mut ui_browser) {
    let height = (*browser).height;
    let mut h = 0;
    let mut pct = 0;
    let col = (*browser).width;
    let mut row = 0;

    if (*browser).nr_entries > 1 {
        pct = (((*browser).index * ((*browser).height - 1) as u64) / ((*browser).nr_entries - 1) as u64) as c_int;
    }

    SLsmg_set_char_set(1);

    while h < height {
        ui_browser__gotorc(browser, row, col);
        row += 1;
        SLsmg_write_char(if h == pct { SLSMG_DIAMOND_CHAR } else { SLSMG_CKBRD_CHAR });
        h += 1;
    }

    SLsmg_set_char_set(0);
}

unsafe fn __ui_browser__refresh(browser: *mut ui_browser) -> c_int {
    let mut width = (*browser).width;

    let row = ((*browser).refresh.unwrap())(browser) as c_int;
    ui_browser__set_color(browser, HE_COLORSET_NORMAL);

    if !(*browser).use_navkeypressed || (*browser).navkeypressed {
        ui_browser__scrollbar_set(browser);
    } else {
        width += 1;
    }

    SLsmg_fill_region((*browser).y + row + (*browser).extra_title_lines, (*browser).x,
                      (*browser).rows as c_int - row, width, ' ' as c_int);

    if (*browser).nr_entries == 0 && !(*browser).no_samples_msg.is_null() {
        __ui__info_window(core::ptr::null(), (*browser).no_samples_msg, core::ptr::null());
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__refresh(browser: *mut ui_browser) -> c_int {
    mutex_lock(&raw mut ui__lock);
    __ui_browser__refresh(browser);
    mutex_unlock(&raw mut ui__lock);

    0
}

/*
 * Here we're updating nr_entries _after_ we started browsing, i.e.  we have to
 * forget about any reference to any entry in the underlying data structure,
 * that is why we do a SEEK_SET. Think about 'perf top' in the hists browser
 * after an output_resort and hist decay.
 */
#[no_mangle]
pub unsafe extern "C" fn ui_browser__update_nr_entries(browser: *mut ui_browser, nr_entries: u32) {
    let mut offset = nr_entries as off_t - (*browser).nr_entries as off_t;

    (*browser).nr_entries = nr_entries;

    if offset < 0 {
        if (*browser).top_idx < (-offset) as u64 {
            offset = -((*browser).top_idx as off_t);
        }

        (*browser).index = ((*browser).index as off_t + offset) as u64;
        (*browser).top_idx = ((*browser).top_idx as off_t + offset) as u64;
    }

    (*browser).top = core::ptr::null_mut();
    ((*browser).seek.unwrap())(browser, (*browser).top_idx as off_t, SEEK_SET);
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__run(browser: *mut ui_browser, delay_secs: c_int) -> c_int {
    let mut err: c_int;
    let mut key: c_int;

    loop {
        let mut offset: off_t;

        mutex_lock(&raw mut ui__lock);
        err = __ui_browser__refresh(browser);
        SLsmg_refresh();
        mutex_unlock(&raw mut ui__lock);
        if err < 0 {
            break;
        }

        key = ui__getch(delay_secs);

        if key == K_RESIZE {
            ui__refresh_dimensions(false);
            ((*browser).refresh_dimensions.unwrap())(browser);
            __ui_browser__show_title(browser, (*browser).title);
            ui_helpline__puts((*browser).helpline);
            continue;
        }

        if (*browser).use_navkeypressed && !(*browser).navkeypressed {
            if key == K_DOWN || key == K_UP ||
               ((*browser).columns != 0 && (key == K_LEFT || key == K_RIGHT)) ||
               key == K_PGDN || key == K_PGUP ||
               key == K_HOME || key == K_END ||
               key == ' ' as c_int {
                (*browser).navkeypressed = true;
                continue;
            } else {
                return key;
            }
        }

        match key {
            K_DOWN => {
                if (*browser).index == (*browser).nr_entries as u64 - 1 {
                    continue;
                }
                (*browser).index += 1;
                if (*browser).index == (*browser).top_idx + (*browser).rows as u64 {
                    (*browser).top_idx += 1;
                    ((*browser).seek.unwrap())(browser, 1, SEEK_CUR);
                }
            }
            K_UP => {
                if (*browser).index == 0 {
                    continue;
                }
                (*browser).index -= 1;
                if (*browser).index < (*browser).top_idx {
                    (*browser).top_idx -= 1;
                    ((*browser).seek.unwrap())(browser, -1, SEEK_CUR);
                }
            }
            K_RIGHT => {
                if (*browser).columns == 0 {
                    return key;
                }
                if (*browser).horiz_scroll < (*browser).columns - 1 {
                    (*browser).horiz_scroll += 1;
                }
            }
            K_LEFT => {
                if (*browser).columns == 0 {
                    return key;
                }
                if (*browser).horiz_scroll != 0 {
                    (*browser).horiz_scroll -= 1;
                } else {
                    return key;
                }
            }
            K_PGDN | 32 => {
                if (*browser).top_idx + (*browser).rows as u64 > (*browser).nr_entries as u64 - 1 {
                    continue;
                }

                offset = (*browser).rows as off_t;
                if (*browser).index + offset as u64 > (*browser).nr_entries as u64 - 1 {
                    offset = (*browser).nr_entries as off_t - 1 - (*browser).index as off_t;
                }
                (*browser).index = ((*browser).index as off_t + offset) as u64;
                (*browser).top_idx = ((*browser).top_idx as off_t + offset) as u64;
                ((*browser).seek.unwrap())(browser, offset, SEEK_CUR);
            }
            K_PGUP => {
                if (*browser).top_idx == 0 {
                    continue;
                }

                if (*browser).top_idx < (*browser).rows as u64 {
                    offset = (*browser).top_idx as off_t;
                } else {
                    offset = (*browser).rows as off_t;
                }

                (*browser).index -= offset as u64;
                (*browser).top_idx -= offset as u64;
                ((*browser).seek.unwrap())(browser, -offset, SEEK_CUR);
            }
            K_HOME => {
                ui_browser__reset_index(browser);
            }
            K_END => {
                offset = (*browser).rows as off_t - 1;
                if offset >= (*browser).nr_entries as off_t {
                    offset = (*browser).nr_entries as off_t - 1;
                }

                (*browser).index = (*browser).nr_entries as u64 - 1;
                (*browser).top_idx = (*browser).index - offset as u64;
                ((*browser).seek.unwrap())(browser, -offset, SEEK_END);
            }
            _ => {
                return key;
            }
        }
    }
    -1
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__list_head_refresh(browser: *mut ui_browser) -> c_uint {
    let head = (*browser).entries as *mut list_head;
    let mut row: c_int = 0;

    if (*browser).nr_entries == 0 {
        return 0;
    }

    if (*browser).top.is_null() || (*browser).top == (*browser).entries {
        (*browser).top = ui_browser__list_head_filter_entries(browser, (*head).next) as *mut c_void;
    }

    let mut pos = (*browser).top as *mut list_head;

    while pos != head {
        if (*browser).filter.is_none() || !((*browser).filter.unwrap())(browser, pos as *mut c_void) {
            ui_browser__gotorc(browser, row, 0);
            ((*browser).write.unwrap())(browser, pos as *mut c_void, row);
            row += 1;
            if row as u32 == (*browser).rows {
                break;
            }
        }
        pos = (*pos).next;
    }

    row as c_uint
}

static mut ui_browser__colorsets: [ui_browser_colorset; 8] = [
    ui_browser_colorset { colorset: HE_COLORSET_TOP, name: b"top\0".as_ptr() as *const c_char, fg: b"red\0".as_ptr() as *const c_char, bg: b"default\0".as_ptr() as *const c_char },
    ui_browser_colorset { colorset: HE_COLORSET_MEDIUM, name: b"medium\0".as_ptr() as *const c_char, fg: b"green\0".as_ptr() as *const c_char, bg: b"default\0".as_ptr() as *const c_char },
    ui_browser_colorset { colorset: HE_COLORSET_NORMAL, name: b"normal\0".as_ptr() as *const c_char, fg: b"default\0".as_ptr() as *const c_char, bg: b"default\0".as_ptr() as *const c_char },
    ui_browser_colorset { colorset: HE_COLORSET_SELECTED, name: b"selected\0".as_ptr() as *const c_char, fg: b"black\0".as_ptr() as *const c_char, bg: b"yellow\0".as_ptr() as *const c_char },
    ui_browser_colorset { colorset: HE_COLORSET_JUMP_ARROWS, name: b"jump_arrows\0".as_ptr() as *const c_char, fg: b"blue\0".as_ptr() as *const c_char, bg: b"default\0".as_ptr() as *const c_char },
    ui_browser_colorset { colorset: HE_COLORSET_ADDR, name: b"addr\0".as_ptr() as *const c_char, fg: b"magenta\0".as_ptr() as *const c_char, bg: b"default\0".as_ptr() as *const c_char },
    ui_browser_colorset { colorset: HE_COLORSET_ROOT, name: b"root\0".as_ptr() as *const c_char, fg: b"white\0".as_ptr() as *const c_char, bg: b"blue\0".as_ptr() as *const c_char },
    ui_browser_colorset { colorset: 0, name: core::ptr::null(), fg: core::ptr::null(), bg: core::ptr::null() },
];

unsafe extern "C" fn ui_browser__color_config(var: *const c_char, value: *const c_char, _data: *mut c_void) -> c_int {
    let mut fg: *mut c_char = core::ptr::null_mut();
    let mut bg: *mut c_char;
    let mut i: usize = 0;

    /* same dir for all commands */
    if (!strstarts(var, cstr(b"colors.\0"))) as c_int != 0 {
        return 0;
    }

    while !ui_browser__colorsets[i].name.is_null() {
        let name = var.add(7);

        if strcmp(ui_browser__colorsets[i].name, name) != 0 {
            i += 1;
            continue;
        }

        fg = strdup(value);
        if fg.is_null() {
            break;
        }

        bg = strchr(fg, ',' as c_int);
        if bg.is_null() {
            break;
        }

        *bg = '\0' as c_char;
        bg = skip_spaces(bg.add(1));
        ui_browser__colorsets[i].bg = bg;
        ui_browser__colorsets[i].fg = fg;
        return 0;
    }

    free(fg as *mut c_void);
    -1
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__argv_seek(browser: *mut ui_browser, offset: off_t, whence: c_int) {
    match whence {
        SEEK_SET => {
            (*browser).top = (*browser).entries;
        }
        SEEK_CUR => {
            (*browser).top = ((*browser).top as *mut *mut c_char).offset(offset as isize) as *mut c_void;
        }
        SEEK_END => {
            (*browser).top = ((*browser).entries as *mut *mut c_char)
                .offset((*browser).nr_entries as isize - 1 + offset as isize) as *mut c_void;
        }
        _ => return,
    }
    assert!(((*browser).top as *mut *mut c_char) < ((*browser).entries as *mut *mut c_char).add((*browser).nr_entries as usize));
    assert!(((*browser).top as *mut *mut c_char) >= ((*browser).entries as *mut *mut c_char));
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__argv_refresh(browser: *mut ui_browser) -> c_uint {
    let mut row: c_uint = 0;
    let mut idx = (*browser).top_idx;
    let mut pos: *mut *mut c_char;

    if (*browser).top.is_null() {
        (*browser).top = (*browser).entries;
    }

    pos = (*browser).top as *mut *mut c_char;
    while idx < (*browser).nr_entries as u64 &&
          row < SLtt_Screen_Rows as c_uint - 1 {
        assert!(pos < ((*browser).entries as *mut *mut c_char).add((*browser).nr_entries as usize));
        if (*browser).filter.is_none() || !((*browser).filter.unwrap())(browser, *pos as *mut c_void) {
            ui_browser__gotorc(browser, row as c_int, 0);
            ((*browser).write.unwrap())(browser, pos as *mut c_void, row as c_int);
            row += 1;
            if row == (*browser).rows {
                break;
            }
        }

        idx += 1;
        pos = pos.add(1);
    }

    row
}

#[no_mangle]
pub unsafe extern "C" fn __ui_browser__vline(browser: *mut ui_browser, column: c_uint, start: u16, end: u16) {
    SLsmg_set_char_set(1);
    ui_browser__gotorc(browser, start as c_int, column as c_int);
    SLsmg_draw_vline((end - start + 1) as c_uint);
    SLsmg_set_char_set(0);
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__write_graph(_browser: *mut ui_browser, graph: c_int) {
    SLsmg_set_char_set(1);
    SLsmg_write_char(graph);
    SLsmg_set_char_set(0);
}

unsafe fn __ui_browser__line_arrow_up(browser: *mut ui_browser, column: c_uint, start: u64, end: u64) {
    let mut row: c_uint;
    let end_row: c_uint;

    SLsmg_set_char_set(1);

    if start < (*browser).top_idx + (*browser).rows as u64 {
        row = (start - (*browser).top_idx) as c_uint;
        ui_browser__gotorc(browser, row as c_int, column as c_int);
        SLsmg_write_char(SLSMG_LLCORN_CHAR);
        ui_browser__gotorc(browser, row as c_int, column as c_int + 1);
        SLsmg_draw_hline(2);

        if row == 0 {
            SLsmg_set_char_set(0);
            return;
        }
        row -= 1;
    } else {
        row = (*browser).rows - 1;
    }

    if end > (*browser).top_idx {
        end_row = (end - (*browser).top_idx) as c_uint;
    } else {
        end_row = 0;
    }

    ui_browser__gotorc(browser, end_row as c_int, column as c_int);
    SLsmg_draw_vline(row - end_row + 1);

    ui_browser__gotorc(browser, end_row as c_int, column as c_int);
    if end >= (*browser).top_idx {
        SLsmg_write_char(SLSMG_ULCORN_CHAR);
        ui_browser__gotorc(browser, end_row as c_int, column as c_int + 1);
        SLsmg_write_char(SLSMG_HLINE_CHAR);
        ui_browser__gotorc(browser, end_row as c_int, column as c_int + 2);
        SLsmg_write_char(SLSMG_RARROW_CHAR);
    }
    SLsmg_set_char_set(0);
}

unsafe fn __ui_browser__line_arrow_down(browser: *mut ui_browser, column: c_uint, start: u64, end: u64) {
    let mut row: c_uint;
    let end_row: c_uint;

    SLsmg_set_char_set(1);

    if start >= (*browser).top_idx {
        row = (start - (*browser).top_idx) as c_uint;
        ui_browser__gotorc(browser, row as c_int, column as c_int);
        SLsmg_write_char(SLSMG_ULCORN_CHAR);
        ui_browser__gotorc(browser, row as c_int, column as c_int + 1);
        SLsmg_draw_hline(2);

        row = row.wrapping_add(1);
        if row == 0 {
            SLsmg_set_char_set(0);
            return;
        }
    } else {
        row = 0;
    }

    if end >= (*browser).top_idx + (*browser).rows as u64 {
        end_row = (*browser).rows - 1;
    } else {
        end_row = (end - (*browser).top_idx) as c_uint;
    }

    ui_browser__gotorc(browser, row as c_int, column as c_int);
    SLsmg_draw_vline(end_row - row + 1);

    ui_browser__gotorc(browser, end_row as c_int, column as c_int);
    if end < (*browser).top_idx + (*browser).rows as u64 {
        SLsmg_write_char(SLSMG_LLCORN_CHAR);
        ui_browser__gotorc(browser, end_row as c_int, column as c_int + 1);
        SLsmg_write_char(SLSMG_HLINE_CHAR);
        ui_browser__gotorc(browser, end_row as c_int, column as c_int + 2);
        SLsmg_write_char(SLSMG_RARROW_CHAR);
    }
    SLsmg_set_char_set(0);
}

#[no_mangle]
pub unsafe extern "C" fn __ui_browser__line_arrow(browser: *mut ui_browser, column: c_uint, start: u64, end: u64) {
    if start > end {
        __ui_browser__line_arrow_up(browser, column, start, end);
    } else {
        __ui_browser__line_arrow_down(browser, column, start, end);
    }
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__mark_fused(browser: *mut ui_browser, column: c_uint, row: c_uint, diff: c_int, arrow_down: bool_) {
    let mut end_row: c_int;

    if diff <= 0 {
        return;
    }

    SLsmg_set_char_set(1);

    if arrow_down {
        if row as u64 + diff as u64 <= (*browser).top_idx {
            return;
        }

        end_row = (row as u64 + diff as u64 - (*browser).top_idx) as c_int;
        ui_browser__gotorc(browser, end_row, column as c_int - 1);
        SLsmg_write_char(SLSMG_LTEE_CHAR);

        loop {
            end_row -= 1;
            if !(end_row >= 0 && end_row > (row as u64 - (*browser).top_idx) as c_int) {
                break;
            }
            ui_browser__gotorc(browser, end_row, column as c_int - 1);
            SLsmg_draw_vline(1);
        }

        end_row = (row as u64 - (*browser).top_idx) as c_int;
        if end_row >= 0 {
            ui_browser__gotorc(browser, end_row, column as c_int - 1);
            SLsmg_write_char(SLSMG_ULCORN_CHAR);
            ui_browser__gotorc(browser, end_row, column as c_int);
            SLsmg_draw_hline(2);
        }
    } else {
        if row as u64 < (*browser).top_idx {
            return;
        }

        end_row = (row as u64 - (*browser).top_idx) as c_int;
        ui_browser__gotorc(browser, end_row, column as c_int - 1);
        SLsmg_write_char(SLSMG_LTEE_CHAR);
        ui_browser__gotorc(browser, end_row, column as c_int);
        SLsmg_draw_hline(2);
    }

    SLsmg_set_char_set(0);
}

#[no_mangle]
pub unsafe extern "C" fn ui_browser__init() {
    let mut i: usize = 0;

    perf_config(Some(ui_browser__color_config), core::ptr::null_mut());

    while !ui_browser__colorsets[i].name.is_null() {
        let c = &mut ui_browser__colorsets[i] as *mut ui_browser_colorset;
        i += 1;
        SLtt_set_color((*c).colorset, (*c).name, (*c).fg, (*c).bg);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
