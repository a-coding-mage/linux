// SPDX-License-Identifier: GPL-2.0+
/*
 *  textbox.c -- implements the text box
 *
 *  ORIGINAL AUTHOR: Savio Lam (lam836@cs.cuhk.hk)
 *  MODIFIED FOR LINUX KERNEL CONFIG BY: William Roadcap (roadcap@cfw.com)
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

type size_t = usize;

#[repr(C)]
pub struct WINDOW {
    _private: [u8; 0],
}

extern "C" {
    static mut stdscr: *mut WINDOW;
    static mut dlg: Dialog;

    fn getmaxyx(win: *mut WINDOW, y: *mut c_int, x: *mut c_int);
    fn getmaxy(win: *mut WINDOW) -> c_int;
    fn getmaxx(win: *mut WINDOW) -> c_int;
    fn newwin(height: c_int, width: c_int, y: c_int, x: c_int) -> *mut WINDOW;
    fn subwin(win: *mut WINDOW, height: c_int, width: c_int, y: c_int, x: c_int) -> *mut WINDOW;
    fn delwin(win: *mut WINDOW) -> c_int;
    fn keypad(win: *mut WINDOW, bf: c_int) -> c_int;
    fn wattrset(win: *mut WINDOW, attrs: c_int) -> c_int;
    fn wbkgdset(win: *mut WINDOW, attrs: c_int);
    fn wmove(win: *mut WINDOW, y: c_int, x: c_int) -> c_int;
    fn waddch(win: *mut WINDOW, ch: c_int) -> c_int;
    fn waddnstr(win: *mut WINDOW, s: *const c_char, n: c_int) -> c_int;
    fn wclrtoeol(win: *mut WINDOW) -> c_int;
    fn wnoutrefresh(win: *mut WINDOW) -> c_int;
    fn wprintw(win: *mut WINDOW, fmt: *const c_char, ...) -> c_int;
    fn wrefresh(win: *mut WINDOW) -> c_int;
    fn wgetch(win: *mut WINDOW) -> c_int;
    fn getyx(win: *mut WINDOW, y: *mut c_int, x: *mut c_int);
    fn mvwaddch(win: *mut WINDOW, y: c_int, x: c_int, ch: c_int) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strchr(s: *const c_char, c: c_int) -> *const c_char;

    fn draw_shadow(win: *mut WINDOW, y: c_int, x: c_int, height: c_int, width: c_int);
    fn draw_box(win: *mut WINDOW, y: c_int, x: c_int, height: c_int, width: c_int,
                dialog_attr: c_int, border_attr: c_int);
    fn print_title(win: *mut WINDOW, title: *const c_char, width: c_int);
    fn print_button(win: *mut WINDOW, label: *const c_char, y: c_int, x: c_int, selected: c_int);
    fn attr_clear(win: *mut WINDOW, height: c_int, width: c_int, attr: c_int);
    fn on_key_esc(win: *mut WINDOW) -> c_int;
    fn on_key_resize();
}

#[repr(C)]
pub struct DialogPart {
    pub atr: c_int,
}

#[repr(C)]
pub struct Dialog {
    pub dialog: DialogPart,
    pub border: DialogPart,
    pub position_indicator: DialogPart,
}

const MAX_LEN: usize = 4096;
const TEXTBOX_HEIGHT_MIN: c_int = 8;
const TEXTBOX_WIDTH_MIN: c_int = 10;

static mut hscroll: c_int = 0;
static mut begin_reached: c_int = 0;
static mut end_reached: c_int = 0;
static mut page_length: c_int = 0;
static mut buf: *const c_char = ptr::null();
static mut page: *const c_char = ptr::null();
static mut start: size_t = 0;
static mut end: size_t = 0;

unsafe fn back_lines(n: c_int) {
    begin_reached = 0;
    for _ in 0..n {
        if *page == 0 {
            if end_reached != 0 {
                end_reached = 0;
                continue;
            }
        }
        if page == buf {
            begin_reached = 1;
            return;
        }
        page = page.offset(-1);
        loop {
            if page == buf {
                begin_reached = 1;
                return;
            }
            page = page.offset(-1);
            if *page == b'\n' as c_char {
                break;
            }
        }
        page = page.offset(1);
    }
}

unsafe fn get_line() -> *mut c_char {
    let mut i: usize = 0;
    static mut line: [c_char; MAX_LEN + 1] = [0; MAX_LEN + 1];

    end_reached = 0;
    while *page != b'\n' as c_char {
        if *page == 0 {
            end_reached = 1;
            break;
        } else if i < MAX_LEN {
            line[i] = *page;
            page = page.offset(1);
            i += 1;
        } else {
            if i == MAX_LEN {
                line[i] = 0;
                i += 1;
            }
            page = page.offset(1);
        }
    }
    if i <= MAX_LEN {
        line[i] = 0;
    }
    if end_reached == 0 {
        page = page.offset(1);
    }
    line.as_mut_ptr()
}

unsafe fn print_line(win: *mut WINDOW, row: c_int, width: c_int) {
    let mut line = get_line();
    line = line.add(core::cmp::min(strlen(line), hscroll as usize));
    wmove(win, row, 0);
    waddch(win, b' ' as c_int);
    waddnstr(win, line, core::cmp::min(strlen(line), (width - 2) as usize) as c_int);
    wclrtoeol(win);
}

unsafe fn print_page(win: *mut WINDOW, height: c_int, width: c_int) {
    let mut passed_end = false;
    page_length = 0;
    for i in 0..height {
        print_line(win, i, width);
        if !passed_end {
            page_length += 1;
        }
        if end_reached != 0 && !passed_end {
            passed_end = true;
        }
    }
    wnoutrefresh(win);
}

unsafe fn print_position(win: *mut WINDOW) {
    let percent: c_int;
    wattrset(win, dlg.position_indicator.atr);
    wbkgdset(win, dlg.position_indicator.atr & A_COLOR);
    percent = ((page.offset_from(buf) as usize) * 100 / strlen(buf)) as c_int;
    wmove(win, getmaxy(win) - 3, getmaxx(win) - 9);
    wprintw(win, b"(%3d%%)\0".as_ptr() as *const c_char, percent);
}

unsafe fn refresh_text_box(dialog: *mut WINDOW, box_: *mut WINDOW, boxh: c_int, boxw: c_int,
                           cur_y: c_int, cur_x: c_int) {
    start = page.offset_from(buf) as usize;
    print_page(box_, boxh, boxw);
    print_position(dialog);
    wmove(dialog, cur_y, cur_x);
    wrefresh(dialog);
    end = page.offset_from(buf) as usize;
}

pub unsafe fn dialog_textbox(title: *const c_char, tbuf: *const c_char, initial_height: c_int,
                             initial_width: c_int, _vscroll: *mut c_int, _hscroll: *mut c_int,
                             extra_key_cb: Option<unsafe extern "C" fn(c_int, size_t, size_t, *mut c_void) -> c_int>,
                             data: *mut c_void) -> c_int {
    let mut i: c_int;
    let mut x: c_int;
    let mut y: c_int;
    let mut cur_x: c_int = 0;
    let mut cur_y: c_int = 0;
    let mut key: c_int = 0;
    let mut height: c_int = 0;
    let mut width: c_int = 0;
    let mut boxh: c_int;
    let mut boxw: c_int;
    let mut dialog: *mut WINDOW;
    let mut box_: *mut WINDOW;
    let mut done = false;

    begin_reached = 1;
    end_reached = 0;
    page_length = 0;
    hscroll = 0;
    buf = tbuf;
    page = buf;
    if !_vscroll.is_null() && *_vscroll != 0 {
        begin_reached = 0;
        for _ in 0..*_vscroll {
            get_line();
        }
    }
    if !_hscroll.is_null() {
        hscroll = *_hscroll;
    }

    'do_resize: loop {
        getmaxyx(stdscr, &mut height, &mut width);
        if height < TEXTBOX_HEIGHT_MIN || width < TEXTBOX_WIDTH_MIN { return -ERRDISPLAYTOOSMALL; }
        if initial_height != 0 { height = initial_height; } else if height > 4 { height -= 4; } else { height = 0; }
        if initial_width != 0 { width = initial_width; } else if width > 5 { width -= 5; } else { width = 0; }
        x = (getmaxx(stdscr) - width) / 2;
        y = (getmaxy(stdscr) - height) / 2;
        draw_shadow(stdscr, y, x, height, width);
        dialog = newwin(height, width, y, x);
        keypad(dialog, TRUE);
        boxh = height - 4;
        boxw = width - 2;
        box_ = subwin(dialog, boxh, boxw, y + 1, x + 1);
        wattrset(box_, dlg.dialog.atr);
        wbkgdset(box_, dlg.dialog.atr & A_COLOR);
        keypad(box_, TRUE);
        draw_box(dialog, 0, 0, height, width, dlg.dialog.atr, dlg.border.atr);
        wattrset(dialog, dlg.border.atr);
        mvwaddch(dialog, height - 3, 0, ACS_LTEE);
        for _ in 0..width - 2 { waddch(dialog, ACS_HLINE); }
        wattrset(dialog, dlg.dialog.atr);
        wbkgdset(dialog, dlg.dialog.atr & A_COLOR);
        waddch(dialog, ACS_RTEE);
        print_title(dialog, title, width);
        print_button(dialog, b" Exit \0".as_ptr() as *const c_char, height - 2, width / 2 - 4, TRUE);
        wnoutrefresh(dialog);
        getyx(dialog, &mut cur_y, &mut cur_x);
        attr_clear(box_, boxh, boxw, dlg.dialog.atr);
        refresh_text_box(dialog, box_, boxh, boxw, cur_y, cur_x);
        while !done {
            key = wgetch(dialog);
            match key {
                69 | 101 | 88 | 120 | 113 | 10 => done = true,
                103 | KEY_HOME => if begin_reached == 0 { begin_reached = 1; page = buf; refresh_text_box(dialog, box_, boxh, boxw, cur_y, cur_x); },
                71 | KEY_END => { end_reached = 1; page = buf.add(strlen(buf)); back_lines(boxh); refresh_text_box(dialog, box_, boxh, boxw, cur_y, cur_x); },
                75 | 107 | KEY_UP => if begin_reached == 0 { back_lines(page_length + 1); refresh_text_box(dialog, box_, boxh, boxw, cur_y, cur_x); },
                66 | 98 | 117 | KEY_PPAGE => if begin_reached == 0 { back_lines(page_length + boxh); refresh_text_box(dialog, box_, boxh, boxw, cur_y, cur_x); },
                74 | 106 | KEY_DOWN => if end_reached == 0 { back_lines(page_length - 1); refresh_text_box(dialog, box_, boxh, boxw, cur_y, cur_x); },
                KEY_NPAGE | 32 | 100 => if end_reached == 0 { begin_reached = 0; refresh_text_box(dialog, box_, boxh, boxw, cur_y, cur_x); },
                48 | 72 | 104 | KEY_LEFT => if hscroll > 0 { if key == 48 { hscroll = 0; } else { hscroll -= 1; } back_lines(page_length); refresh_text_box(dialog, box_, boxh, boxw, cur_y, cur_x); },
                76 | 108 | KEY_RIGHT => if hscroll < MAX_LEN as c_int { hscroll += 1; back_lines(page_length); refresh_text_box(dialog, box_, boxh, boxw, cur_y, cur_x); },
                KEY_ESC => if on_key_esc(dialog) == KEY_ESC { done = true; },
                KEY_RESIZE => { back_lines(height); delwin(box_); delwin(dialog); on_key_resize(); continue 'do_resize; },
                _ => if let Some(cb) = extra_key_cb { if cb(key, start, end, data) != 0 { done = true; } },
            }
        }
        delwin(box_);
        delwin(dialog);
        break 'do_resize;
    }
    if !_vscroll.is_null() {
        let mut s = buf;
        *_vscroll = 0;
        back_lines(page_length);
        while s < page && { s = strchr(s, b'\n' as c_int); !s.is_null() } {
            *_vscroll += 1;
            s = s.add(1);
        }
    }
    if !_hscroll.is_null() { *_hscroll = hscroll; }
    key
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
