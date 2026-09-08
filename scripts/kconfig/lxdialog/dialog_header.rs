/* SPDX-License-Identifier: GPL-2.0+ */
/*
 *  dialog.h -- common declarations for all dialog modules
 *
 *  AUTHOR: Savio Lam (lam836@cs.cuhk.hk)
 */

// C headers and ncurses provide the following types and symbols.
use core::ffi::{c_char, c_int, c_void};
use core::ffi::VaList;

pub type chtype = usize;
#[repr(C)]
pub struct WINDOW {
    _private: [u8; 0],
}

// #define TR(params) _tracef params
pub const KEY_ESC: c_int = 27;
pub const TAB: c_int = 9;
pub const MAX_LEN: usize = 2048;
pub const BUF_SIZE: usize = 10 * 1024;

#[inline]
pub const fn min<T: Ord>(x: T, y: T) -> T { if x < y { x } else { y } }
#[inline]
pub const fn max<T: Ord>(x: T, y: T) -> T { if x > y { x } else { y } }

// ncurses fallback ACS definitions, used when the corresponding definitions
// are unavailable in the build environment.
pub const ACS_ULCORNER: chtype = b'+' as chtype;
pub const ACS_LLCORNER: chtype = b'+' as chtype;
pub const ACS_URCORNER: chtype = b'+' as chtype;
pub const ACS_LRCORNER: chtype = b'+' as chtype;
pub const ACS_HLINE: chtype = b'-' as chtype;
pub const ACS_VLINE: chtype = b'|' as chtype;
pub const ACS_LTEE: chtype = b'+' as chtype;
pub const ACS_RTEE: chtype = b'+' as chtype;
pub const ACS_UARROW: chtype = b'^' as chtype;
pub const ACS_DARROW: chtype = b'v' as chtype;

/* error return codes */
// KEY_MAX is supplied by ncurses.
pub const ERRDISPLAYTOOSMALL: c_int = KEY_MAX + 1;

/* Color definitions */
#[repr(C)]
pub struct dialog_color {
    pub atr: chtype,
    pub fg: c_int,
    pub bg: c_int,
    pub hl: c_int,
}

#[repr(C)]
pub struct subtitle_list {
    pub next: *mut subtitle_list,
    pub text: *const c_char,
}

#[repr(C)]
pub struct dialog_info {
    pub backtitle: *const c_char,
    pub subtitles: *mut subtitle_list,
    pub screen: dialog_color,
    pub shadow: dialog_color,
    pub dialog: dialog_color,
    pub title: dialog_color,
    pub border: dialog_color,
    pub button_active: dialog_color,
    pub button_inactive: dialog_color,
    pub button_key_active: dialog_color,
    pub button_key_inactive: dialog_color,
    pub button_label_active: dialog_color,
    pub button_label_inactive: dialog_color,
    pub inputbox: dialog_color,
    pub position_indicator: dialog_color,
    pub menubox: dialog_color,
    pub menubox_border: dialog_color,
    pub item: dialog_color,
    pub item_selected: dialog_color,
    pub tag: dialog_color,
    pub tag_selected: dialog_color,
    pub tag_key: dialog_color,
    pub tag_key_selected: dialog_color,
    pub check: dialog_color,
    pub check_selected: dialog_color,
    pub uarrow: dialog_color,
    pub darrow: dialog_color,
}

extern "C" {
    pub static mut dlg: dialog_info;
    pub static mut dialog_input_result: [c_char; 0];
    pub static mut saved_x: c_int;
    pub static mut saved_y: c_int;

    pub fn item_reset();
    pub fn item_make(fmt: *const c_char, ...);
    pub fn item_add_str(fmt: *const c_char, ...);
    pub fn item_set_tag(tag: c_char);
    pub fn item_set_data(p: *mut c_void);
    pub fn item_set_selected(val: c_int);
    pub fn item_activate_selected() -> c_int;
    pub fn item_data() -> *mut c_void;
    pub fn item_tag() -> c_char;
}

pub const MAXITEMSTR: usize = 200;
#[repr(C)]
pub struct dialog_item {
    pub str_: [c_char; MAXITEMSTR],
    pub tag: c_char,
    pub data: *mut c_void,
    pub selected: c_int,
}

#[repr(C)]
pub struct dialog_list {
    pub node: dialog_item,
    pub next: *mut dialog_list,
}

extern "C" {
    pub static mut item_cur: *mut dialog_list;
    pub static mut item_nil: dialog_list;
    pub static mut item_head: *mut dialog_list;

    pub fn item_count() -> c_int;
    pub fn item_set(n: c_int);
    pub fn item_n() -> c_int;
    pub fn item_str() -> *const c_char;
    pub fn item_is_selected() -> c_int;
    pub fn item_is_tag(tag: c_char) -> c_int;

    pub fn on_key_esc(win: *mut WINDOW) -> c_int;
    pub fn on_key_resize() -> c_int;
}

pub const CHECKLIST_HEIGHT_MIN: c_int = 6;
pub const CHECKLIST_WIDTH_MIN: c_int = 6;
pub const INPUTBOX_HEIGHT_MIN: c_int = 2;
pub const INPUTBOX_WIDTH_MIN: c_int = 2;
pub const MENUBOX_HEIGHT_MIN: c_int = 15;
pub const MENUBOX_WIDTH_MIN: c_int = 65;
pub const TEXTBOX_HEIGHT_MIN: c_int = 8;
pub const TEXTBOX_WIDTH_MIN: c_int = 8;
pub const YESNO_HEIGHT_MIN: c_int = 4;
pub const YESNO_WIDTH_MIN: c_int = 4;
pub const WINDOW_HEIGHT_MIN: c_int = 19;
pub const WINDOW_WIDTH_MIN: c_int = 80;

extern "C" {
    pub fn init_dialog(backtitle: *const c_char) -> c_int;
    pub fn set_dialog_backtitle(backtitle: *const c_char);
    pub fn set_dialog_subtitles(subtitles: *mut subtitle_list);
    pub fn end_dialog(x: c_int, y: c_int);
    pub fn attr_clear(win: *mut WINDOW, height: c_int, width: c_int, attr: chtype);
    pub fn dialog_clear();
    pub fn print_autowrap(win: *mut WINDOW, prompt: *const c_char, width: c_int, y: c_int, x: c_int);
    pub fn print_button(win: *mut WINDOW, label: *const c_char, y: c_int, x: c_int, selected: c_int);
    pub fn print_title(dialog: *mut WINDOW, title: *const c_char, width: c_int);
    pub fn draw_box(win: *mut WINDOW, y: c_int, x: c_int, height: c_int, width: c_int, box_: chtype, border: chtype);
    pub fn draw_shadow(win: *mut WINDOW, y: c_int, x: c_int, height: c_int, width: c_int);

    pub fn first_alpha(string: *const c_char, exempt: *const c_char) -> c_int;
    pub fn dialog_yesno(title: *const c_char, prompt: *const c_char, height: c_int, width: c_int) -> c_int;
    pub fn dialog_msgbox(title: *const c_char, prompt: *const c_char, height: c_int, width: c_int, pause: c_int) -> c_int;
    pub fn dialog_textbox(title: *const c_char, tbuf: *const c_char, initial_height: c_int, initial_width: c_int, vscroll: *mut c_int, hscroll: *mut c_int, extra_key_cb: Option<unsafe extern "C" fn(c_int, usize, usize, *mut c_void) -> c_int>, data: *mut c_void) -> c_int;
    pub fn dialog_menu(title: *const c_char, prompt: *const c_char, selected: *const c_void, s_scroll: *mut c_int) -> c_int;
    pub fn dialog_checklist(title: *const c_char, prompt: *const c_char, height: c_int, width: c_int, list_height: c_int) -> c_int;
    pub fn dialog_inputbox(title: *const c_char, prompt: *const c_char, height: c_int, width: c_int, init: *const c_char) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
