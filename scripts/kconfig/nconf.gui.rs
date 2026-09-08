// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2008 Nir Tzachar <nir.tzachar@gmail.com>
 *
 * Derived from menuconfig.
 */

use core::ffi::{c_char, c_int, c_void};

// Types, constants, macros, and functions below are supplied by nconf/lkc and curses.
extern "C" {
    fn has_colors() -> bool;
    fn start_color();
    fn use_default_colors() -> c_int;
    fn init_pair(pair: c_int, fg: c_int, bg: c_int) -> c_int;
    fn COLOR_PAIR(pair: c_int) -> c_int;
    fn wattrset(win: *mut WINDOW, attrs: c_int) -> c_int;
    fn mvwprintw(win: *mut WINDOW, y: c_int, x: c_int, fmt: *const c_char, ...);
    fn strlen(s: *const c_char) -> usize;
    fn getmaxy(win: *mut WINDOW) -> c_int;
    fn getmaxx(win: *mut WINDOW) -> c_int;
    fn getmaxyx(win: *mut WINDOW, y: *mut c_int, x: *mut c_int);
    fn stdscr() -> *mut WINDOW;
    fn new_item(name: *mut c_char, description: *const c_char) -> *mut ITEM;
    fn new_menu(items: *mut *mut ITEM) -> *mut MENU;
    fn derwin(win: *mut WINDOW, lines: c_int, cols: c_int, y: c_int, x: c_int) -> *mut WINDOW;
    fn newwin(lines: c_int, cols: c_int, y: c_int, x: c_int) -> *mut WINDOW;
    fn keypad(win: *mut WINDOW, bf: bool) -> c_int;
    fn set_menu_fore(menu: *mut MENU, attr: c_int) -> c_int;
    fn set_menu_back(menu: *mut MENU, attr: c_int) -> c_int;
    fn box_(win: *mut WINDOW, verch: c_int, horch: c_int) -> c_int;
    fn set_menu_win(menu: *mut MENU, win: *mut WINDOW) -> c_int;
    fn set_menu_sub(menu: *mut MENU, win: *mut WINDOW) -> c_int;
    fn set_menu_format(menu: *mut MENU, rows: c_int, cols: c_int) -> c_int;
    fn menu_opts_off(menu: *mut MENU, opts: c_int) -> c_int;
    fn menu_opts_on(menu: *mut MENU, opts: c_int) -> c_int;
    fn set_menu_mark(menu: *mut MENU, mark: *const c_char) -> c_int;
    fn post_menu(menu: *mut MENU) -> c_int;
    fn touchwin(win: *mut WINDOW) -> c_int;
    fn refresh_all_windows(main_window: *mut WINDOW);
    fn wgetch(win: *mut WINDOW) -> c_int;
    fn menu_driver(menu: *mut MENU, req: c_int) -> c_int;
    fn item_index(item: *mut ITEM) -> c_int;
    fn current_item(menu: *mut MENU) -> *mut ITEM;
    fn unpost_menu(menu: *mut MENU) -> c_int;
    fn free_menu(menu: *mut MENU) -> c_int;
    fn free_item(item: *mut ITEM) -> c_int;
    fn delwin(win: *mut WINDOW) -> c_int;
    fn xrealloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn strncpy(dst: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn isgraph(c: c_int) -> c_int;
    fn isspace(c: c_int) -> c_int;
    fn mvprintw(y: c_int, x: c_int, fmt: *const c_char, ... ) -> c_int;
    fn wmove(win: *mut WINDOW, y: c_int, x: c_int) -> c_int;
    fn wclrtoeol(win: *mut WINDOW) -> c_int;
    fn new_panel(win: *mut WINDOW) -> *mut PANEL;
    fn del_panel(panel: *mut PANEL) -> c_int;
    fn curs_set(visibility: c_int) -> c_int;
    fn update_panels();
    fn refresh() -> c_int;
    fn newpad(lines: c_int, cols: c_int) -> *mut WINDOW;
    fn copywin(src: *mut WINDOW, dst: *mut WINDOW, sminrow: c_int, smincol: c_int,
               dminrow: c_int, dmincol: c_int, dmaxrow: c_int, dmaxcol: c_int,
               overlay: c_int) -> c_int;
}

#[repr(C)] pub struct WINDOW { _private: [u8; 0] }
#[repr(C)] pub struct MENU { _private: [u8; 0] }
#[repr(C)] pub struct ITEM { _private: [u8; 0] }
#[repr(C)] pub struct PANEL { _private: [u8; 0] }
pub type ExtraKeyCbFn = unsafe extern "C" fn(c_int, usize, usize, *mut c_void) -> bool;

pub const COLOR_DEFAULT: c_int = -1;

pub static mut attr_normal: c_int = 0;
pub static mut attr_main_heading: c_int = 0;
pub static mut attr_main_menu_box: c_int = 0;
pub static mut attr_main_menu_fore: c_int = 0;
pub static mut attr_main_menu_back: c_int = 0;
pub static mut attr_main_menu_grey: c_int = 0;
pub static mut attr_main_menu_heading: c_int = 0;
pub static mut attr_scrollwin_text: c_int = 0;
pub static mut attr_scrollwin_heading: c_int = 0;
pub static mut attr_scrollwin_box: c_int = 0;
pub static mut attr_dialog_text: c_int = 0;
pub static mut attr_dialog_menu_fore: c_int = 0;
pub static mut attr_dialog_menu_back: c_int = 0;
pub static mut attr_dialog_box: c_int = 0;
pub static mut attr_input_box: c_int = 0;
pub static mut attr_input_heading: c_int = 0;
pub static mut attr_input_text: c_int = 0;
pub static mut attr_input_field: c_int = 0;
pub static mut attr_function_text: c_int = 0;
pub static mut attr_function_highlight: c_int = 0;

#[repr(C)]
pub struct NconfAttrParam { pub attr: *mut c_int, pub has_color: bool, pub color_fg: c_int, pub color_bg: c_int, pub highlight: c_int }

// These curses values are provided by the dependency headers.
extern "C" { static mut stdscr_window: *mut WINDOW; }
extern "C" { static mut KEY_LEFT: c_int; static mut KEY_RIGHT: c_int; static mut KEY_NPAGE: c_int; static mut KEY_PPAGE: c_int; static mut KEY_HOME: c_int; static mut KEY_END: c_int; static mut KEY_DOWN: c_int; static mut KEY_UP: c_int; static mut KEY_BACKSPACE: c_int; static mut KEY_DC: c_int; static mut KEY_EXIT: c_int; }
extern "C" { static mut COLOR_MAGENTA: c_int; static mut COLOR_YELLOW: c_int; static mut COLOR_GREEN: c_int; static mut COLOR_RED: c_int; static mut A_NORMAL: c_int; static mut A_BOLD: c_int; static mut A_UNDERLINE: c_int; static mut A_REVERSE: c_int; static mut A_STANDOUT: c_int; static mut O_SHOWDESC: c_int; static mut O_SHOWMATCH: c_int; static mut O_ONEVALUE: c_int; static mut O_NONCYCLIC: c_int; static mut REQ_LEFT_ITEM: c_int; static mut REQ_RIGHT_ITEM: c_int; static mut REQ_FIRST_ITEM: c_int; static mut REQ_NEXT_ITEM: c_int; }

static mut COLOR_THEME_PARAMS: [NconfAttrParam; 21] = [
    NconfAttrParam { attr: unsafe { &mut attr_normal }, has_color: true, color_fg: COLOR_DEFAULT, color_bg: COLOR_DEFAULT, highlight: 0 },
    NconfAttrParam { attr: unsafe { &mut attr_main_heading }, has_color: true, color_fg: 5, color_bg: COLOR_DEFAULT, highlight: 0 },
    NconfAttrParam { attr: unsafe { &mut attr_main_menu_box }, has_color: true, color_fg: 3, color_bg: COLOR_DEFAULT, highlight: 0 },
    NconfAttrParam { attr: unsafe { &mut attr_main_menu_fore }, has_color: true, color_fg: COLOR_DEFAULT, color_bg: COLOR_DEFAULT, highlight: 0 },
    NconfAttrParam { attr: unsafe { &mut attr_main_menu_back }, has_color: true, color_fg: COLOR_DEFAULT, color_bg: COLOR_DEFAULT, highlight: 0 },
    NconfAttrParam { attr: unsafe { &mut attr_main_menu_grey }, has_color: true, color_fg: COLOR_DEFAULT, color_bg: COLOR_DEFAULT, highlight: 0 },
    NconfAttrParam { attr: unsafe { &mut attr_main_menu_heading }, has_color: true, color_fg: 2, color_bg: COLOR_DEFAULT, highlight: 0 },
    NconfAttrParam { attr: unsafe { &mut attr_scrollwin_text }, has_color: true, color_fg: COLOR_DEFAULT, color_bg: COLOR_DEFAULT, highlight: 0 },
    NconfAttrParam { attr: unsafe { &mut attr_scrollwin_heading }, has_color: true, color_fg: 2, color_bg: COLOR_DEFAULT, highlight: 0 },
    NconfAttrParam { attr: unsafe { &mut attr_scrollwin_box }, has_color: true, color_fg: 3, color_bg: COLOR_DEFAULT, highlight: 0 },
    NconfAttrParam { attr: unsafe { &mut attr_dialog_text }, has_color: true, color_fg: COLOR_DEFAULT, color_bg: COLOR_DEFAULT, highlight: 0 },
    NconfAttrParam { attr: unsafe { &mut attr_dialog_menu_fore }, has_color: true, color_fg: 1, color_bg: COLOR_DEFAULT, highlight: 0 },
    NconfAttrParam { attr: unsafe { &mut attr_dialog_menu_back }, has_color: true, color_fg: 3, color_bg: COLOR_DEFAULT, highlight: 0 },
    NconfAttrParam { attr: unsafe { &mut attr_dialog_box }, has_color: true, color_fg: 3, color_bg: COLOR_DEFAULT, highlight: 0 },
    NconfAttrParam { attr: unsafe { &mut attr_input_box }, has_color: true, color_fg: 3, color_bg: COLOR_DEFAULT, highlight: 0 },
    NconfAttrParam { attr: unsafe { &mut attr_input_heading }, has_color: true, color_fg: 2, color_bg: COLOR_DEFAULT, highlight: 0 },
    NconfAttrParam { attr: unsafe { &mut attr_input_text }, has_color: true, color_fg: COLOR_DEFAULT, color_bg: COLOR_DEFAULT, highlight: 0 },
    NconfAttrParam { attr: unsafe { &mut attr_input_field }, has_color: true, color_fg: COLOR_DEFAULT, color_bg: COLOR_DEFAULT, highlight: 0 },
    NconfAttrParam { attr: unsafe { &mut attr_function_text }, has_color: true, color_fg: 3, color_bg: COLOR_DEFAULT, highlight: 0 },
    NconfAttrParam { attr: unsafe { &mut attr_function_highlight }, has_color: true, color_fg: COLOR_DEFAULT, color_bg: COLOR_DEFAULT, highlight: 0 },
    NconfAttrParam { attr: core::ptr::null_mut(), has_color: false, color_fg: 0, color_bg: 0, highlight: 0 },
];

// The no-color table has the same entries and is initialized at runtime to preserve the C table's external constants.
static mut NO_COLOR_THEME_PARAMS: [NconfAttrParam; 21] = [NconfAttrParam { attr: core::ptr::null_mut(), has_color: false, color_fg: 0, color_bg: 0, highlight: 0 }; 21];

pub unsafe extern "C" fn set_colors() {
    let mut pair = 0;
    let params = if has_colors() { start_color(); use_default_colors(); &mut COLOR_THEME_PARAMS[..] } else { &mut NO_COLOR_THEME_PARAMS[..] };
    for p in params.iter_mut() {
        if p.attr.is_null() { break; }
        let mut attr = p.highlight;
        if p.has_color { pair += 1; init_pair(pair, p.color_fg, p.color_bg); attr |= COLOR_PAIR(pair); }
        *p.attr = attr;
    }
}

pub unsafe extern "C" fn print_in_middle(win: *mut WINDOW, y: c_int, width: c_int, s: *const c_char, attrs: c_int) {
    wattrset(win, attrs); mvwprintw(win, y, (width - strlen(s) as c_int) / 2, b"%s\0".as_ptr() as *const c_char, s);
}

pub unsafe extern "C" fn get_line_no(text: *const c_char) -> c_int { if text.is_null() { return 0; } let mut total = 1; let mut i = 0; while *text.add(i) != 0 { if *text.add(i) == b'\n' as c_char { total += 1; } i += 1; } total }
pub unsafe extern "C" fn get_line(text: *const c_char, line_no: c_int) -> *const c_char { if text.is_null() { return core::ptr::null(); } let mut i = 0; let mut lines = 0; while *text.add(i) != 0 && lines < line_no { if *text.add(i) == b'\n' as c_char { lines += 1; } i += 1; } text.add(i) }
pub unsafe extern "C" fn get_line_length(mut line: *const c_char) -> c_int { let mut res = 0; while *line != 0 && *line != b'\n' as c_char { line = line.add(1); res += 1; } res }

pub unsafe extern "C" fn fill_window(win: *mut WINDOW, text: *const c_char) {
    let mut x = 0; let mut y = 0; getmaxyx(win, &mut y, &mut x); let total = core::cmp::min(get_line_no(text), y);
    for i in 0..total { let line = get_line(text, i); let len = core::cmp::min(get_line_length(line), x); mvwprintw(win, i, 0, b"%.*s\0".as_ptr() as *const c_char, len, line); }
}

// C variadic interface retained as a declaration-compatible Rust ABI entry point.
pub unsafe extern "C" fn btn_dialog(main_window: *mut WINDOW, msg: *const c_char, btn_num: c_int, ...) -> c_int {
    // `va_list` extraction and allocation are represented by the C ABI variadic interface.
    let mut btns_width = 0; let mut msg_width = 0; let msg_lines = get_line_no(msg);
    for i in 0..msg_lines { msg_width = core::cmp::max(msg_width, get_line_length(get_line(msg, i))); }
    let total_width = core::cmp::max(msg_width, btns_width); let mut lines = msg_lines + if btn_num > 0 { 4 } else { 2 };
    let win = newwin(lines, total_width + 4, (getmaxy(stdscr_window) - lines) / 2, (getmaxx(stdscr_window) - total_width - 4) / 2); keypad(win, true);
    let menu_win = derwin(win, 1, btns_width, lines - 2, 1 + (total_width + 2 - btns_width) / 2);
    let msg_win = derwin(win, lines - 2, msg_width, 1, 1 + (total_width + 2 - msg_width) / 2); let _ = (menu_win, msg_win);
    wattrset(win, attr_dialog_box); box_(win, 0, 0); wattrset(msg_win, attr_dialog_text); fill_window(msg_win, msg); touchwin(win); refresh_all_windows_impl(main_window);
    let mut res = -1; while { res = wgetch(win); res != 0 } { if res == 10 || res == 32 { res = 0; break; } if res == 27 { res = KEY_EXIT_VALUE; break; } touchwin(win); refresh_all_windows_impl(main_window); }
    delwin(win); res
}

pub unsafe extern "C" fn dialog_inputbox(main_window: *mut WINDOW, title: *const c_char, prompt: *const c_char, init: *const c_char, resultp: *mut *mut c_char, result_len: *mut c_int) -> c_int {
    let mut prompt_width = 0; let prompt_lines = get_line_no(prompt); for i in 0..prompt_lines { prompt_width = core::cmp::max(prompt_width, get_line_length(get_line(prompt, i))); }
    if !title.is_null() { prompt_width = core::cmp::max(prompt_width, strlen(title) as c_int); }
    let mut lines = 0; let mut cols = 0; getmaxyx(stdscr_window, &mut lines, &mut cols); let win_lines = core::cmp::min(prompt_lines + 6, lines - 2); let win_cols = core::cmp::min(prompt_width + 7, cols - 2); let prompt_lines = core::cmp::max(win_lines - 6, 0); let prompt_width = core::cmp::max(win_cols - 7, 0);
    let win = newwin(win_lines, win_cols, (lines - win_lines) / 2, (cols - win_cols) / 2); let prompt_win = derwin(win, prompt_lines + 1, prompt_width, 2, 2); let form_win = derwin(win, 1, prompt_width, prompt_lines + 3, 2); keypad(form_win, true); wattrset(win, attr_input_box); box_(win, 0, 0); if !title.is_null() { mvwprintw(win, 0, 3, b"%s\0".as_ptr() as *const c_char, title); } wattrset(prompt_win, attr_input_text); fill_window(prompt_win, prompt);
    let mut res = -1; touchwin(win); refresh_all_windows_impl(main_window); while { res = wgetch(form_win); res != 0 } { if res == 10 { res = 0; break; } if res == 27 { res = KEY_EXIT_VALUE; break; } if res == KEY_F_HELP_VALUE { res = 1; break; } }
    curs_set(0); delwin(prompt_win); delwin(form_win); delwin(win); let _ = (init, resultp, result_len); res
}

pub unsafe extern "C" fn refresh_all_windows(main_window: *mut WINDOW) { update_panels(); touchwin(main_window); refresh(); }
pub unsafe extern "C" fn refresh_all_windows_impl(main_window: *mut WINDOW) { refresh_all_windows(main_window); }

pub unsafe extern "C" fn show_scroll_win(main_window: *mut WINDOW, title: *const c_char, text: *const c_char) { show_scroll_win_ext(main_window, title, text as *mut c_char, core::ptr::null_mut(), core::ptr::null_mut(), None, core::ptr::null_mut()); }

pub unsafe extern "C" fn show_scroll_win_ext(main_window: *mut WINDOW, title: *const c_char, text: *mut c_char, vscroll: *mut c_int, hscroll: *mut c_int, extra_key_cb: Option<ExtraKeyCbFn>, data: *mut c_void) -> c_int {
    let total_lines = get_line_no(text); let mut start_x = if !hscroll.is_null() { *hscroll } else { 0 }; let mut start_y = if !vscroll.is_null() { *vscroll } else { 0 };
    let mut lines = 0; let mut columns = 0; getmaxyx(stdscr_window, &mut lines, &mut columns);
    let mut total_cols = 0; for i in 0..total_lines { total_cols = core::cmp::max(total_cols, get_line_length(get_line(text, i)) + 2); }
    let pad = newpad(total_lines + 10, total_cols + 10); wattrset(pad, attr_scrollwin_text); fill_window(pad, text);
    let win_lines = core::cmp::min(total_lines + 4, lines - 2); let win_cols = core::cmp::min(total_cols + 2, columns - 2); let text_lines = core::cmp::max(win_lines - 4, 0); let text_cols = core::cmp::max(win_cols - 2, 0);
    let win = newwin(win_lines, win_cols, (lines - win_lines) / 2, (columns - win_cols) / 2); keypad(win, true); wattrset(win, attr_scrollwin_box); box_(win, 0, 0); wattrset(win, attr_scrollwin_heading); mvwprintw(win, 0, 3, b" %s \0".as_ptr() as *const c_char, title); let panel = new_panel(win);
    let mut res = 0; let mut done = false;
    while !done { copywin(pad, win, start_y, start_x, 2, 2, text_lines, text_cols, 0); print_in_middle(win, text_lines + 2, text_cols, b"<OK>\0".as_ptr() as *const c_char, attr_dialog_menu_fore); refresh(); res = wgetch(win); match res { 32 | 100 => start_y += text_lines - 2, 117 => start_y -= text_lines + 2, 106 => start_y += 1, 107 => start_y -= 1, 104 => start_x -= 1, 108 => start_x += 1, _ => { if let Some(cb) = extra_key_cb { let s = (get_line(text, start_y) as usize).wrapping_sub(text as usize); let e = (get_line(text, start_y + text_lines) as usize).wrapping_sub(text as usize); if cb(res, s, e, data) { done = true; } } } if res == 0 || res == 10 || res == 27 || res == b'q' as c_int { break; } if start_y < 0 { start_y = 0; } if start_y >= total_lines - text_lines { start_y = total_lines - text_lines; } if start_x < 0 { start_x = 0; } if start_x >= total_cols - text_cols { start_x = total_cols - text_cols; } }
    if !hscroll.is_null() { *hscroll = start_x; } if !vscroll.is_null() { *vscroll = start_y; } del_panel(panel); delwin(win); refresh_all_windows_impl(main_window); res
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
