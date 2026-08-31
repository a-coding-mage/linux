/* SPDX-License-Identifier: GPL-2.0 */

// Translated from perf/ui/browser.h. C includes:
// <linux/types.h>, <stdarg.h>, <sys/types.h>

use core::ffi::{c_char, c_double, c_int, c_uint, c_void};

pub type u64 = u64;
pub type u32 = u32;
pub type u16 = u16;
pub type u8 = u8;
pub type off_t = isize;

// File-local stand-in for C va_list from <stdarg.h>; exact representation is
// supplied by the target C ABI.
pub type va_list = *mut c_void;

pub const HE_COLORSET_TOP: c_int = 50;
pub const HE_COLORSET_MEDIUM: c_int = 51;
pub const HE_COLORSET_NORMAL: c_int = 52;
pub const HE_COLORSET_SELECTED: c_int = 53;
pub const HE_COLORSET_JUMP_ARROWS: c_int = 54;
pub const HE_COLORSET_ADDR: c_int = 55;
pub const HE_COLORSET_ROOT: c_int = 56;

#[repr(C)]
pub struct ui_browser {
    pub index: u64,
    pub top_idx: u64,
    pub top: *mut c_void,
    pub entries: *mut c_void,
    pub y: u16,
    pub x: u16,
    pub width: u16,
    pub height: u16,
    pub rows: u16,
    pub columns: u16,
    pub horiz_scroll: u16,
    pub extra_title_lines: u8,
    pub current_color: c_int,
    pub priv_: *mut c_void,
    pub title: *mut c_char,
    pub helpline: *mut c_char,
    pub no_samples_msg: *const c_char,
    pub refresh_dimensions: Option<unsafe extern "C" fn(browser: *mut ui_browser)>,
    pub refresh: Option<unsafe extern "C" fn(browser: *mut ui_browser) -> c_uint>,
    pub write: Option<unsafe extern "C" fn(browser: *mut ui_browser, entry: *mut c_void, row: c_int)>,
    pub seek: Option<unsafe extern "C" fn(browser: *mut ui_browser, offset: off_t, whence: c_int)>,
    pub filter: Option<unsafe extern "C" fn(browser: *mut ui_browser, entry: *mut c_void) -> bool>,
    pub nr_entries: u32,
    pub navkeypressed: bool,
    pub use_navkeypressed: bool,
}

#[repr(C)]
pub struct perf_session {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn ui_browser__set_color(browser: *mut ui_browser, color: c_int) -> c_int;
    pub fn ui_browser__set_percent_color(browser: *mut ui_browser, percent: c_double, current: bool);
    pub fn ui_browser__is_current_entry(browser: *mut ui_browser, row: c_uint) -> bool;
    pub fn ui_browser__refresh_dimensions(browser: *mut ui_browser);
    pub fn ui_browser__reset_index(browser: *mut ui_browser);

    pub fn ui_browser__gotorc_title(browser: *mut ui_browser, y: c_int, x: c_int);
    pub fn ui_browser__gotorc(browser: *mut ui_browser, y: c_int, x: c_int);
    pub fn ui_browser__write_nstring(browser: *mut ui_browser, msg: *const c_char, width: c_uint);
    pub fn ui_browser__vprintf(browser: *mut ui_browser, fmt: *const c_char, args: va_list);
    pub fn ui_browser__printf(browser: *mut ui_browser, fmt: *const c_char, ...);
    pub fn ui_browser__write_graph(browser: *mut ui_browser, graph: c_int);
    pub fn __ui_browser__line_arrow(browser: *mut ui_browser, column: c_uint, start: u64, end: u64);
    pub fn ui_browser__mark_fused(
        browser: *mut ui_browser,
        column: c_uint,
        row: c_uint,
        diff: c_int,
        arrow_down: bool,
    );
    pub fn __ui_browser__show_title(browser: *mut ui_browser, title: *const c_char);
    pub fn ui_browser__show_title(browser: *mut ui_browser, title: *const c_char);
    pub fn ui_browser__show(
        browser: *mut ui_browser,
        title: *const c_char,
        helpline: *const c_char,
        ...
    ) -> c_int;
    pub fn ui_browser__hide(browser: *mut ui_browser);
    pub fn ui_browser__refresh(browser: *mut ui_browser) -> c_int;
    pub fn ui_browser__run(browser: *mut ui_browser, delay_secs: c_int) -> c_int;
    pub fn ui_browser__update_nr_entries(browser: *mut ui_browser, nr_entries: u32);
    pub fn ui_browser__handle_resize(browser: *mut ui_browser);
    pub fn __ui_browser__vline(browser: *mut ui_browser, column: c_uint, start: u16, end: u16);

    pub fn ui_browser__warning(browser: *mut ui_browser, timeout: c_int, format: *const c_char, ...) -> c_int;
    pub fn ui_browser__warn_unhandled_hotkey(
        browser: *mut ui_browser,
        key: c_int,
        timeout: c_int,
        help: *const c_char,
    ) -> c_int;
    pub fn ui_browser__help_window(browser: *mut ui_browser, text: *const c_char) -> c_int;
    pub fn ui_browser__dialog_yesno(browser: *mut ui_browser, text: *const c_char) -> bool;
    pub fn ui_browser__input_window(
        title: *const c_char,
        text: *const c_char,
        input: *mut c_char,
        exit_msg: *const c_char,
        delay_sec: c_int,
    ) -> c_int;
    pub fn tui__header_window(session: *mut perf_session) -> c_int;

    pub fn ui_browser__argv_seek(browser: *mut ui_browser, offset: off_t, whence: c_int);
    pub fn ui_browser__argv_refresh(browser: *mut ui_browser) -> c_uint;

    pub fn ui_browser__rb_tree_seek(browser: *mut ui_browser, offset: off_t, whence: c_int);
    pub fn ui_browser__rb_tree_refresh(browser: *mut ui_browser) -> c_uint;

    pub fn ui_browser__list_head_seek(browser: *mut ui_browser, offset: off_t, whence: c_int);
    pub fn ui_browser__list_head_refresh(browser: *mut ui_browser) -> c_uint;

    pub fn ui_browser__init();
}
