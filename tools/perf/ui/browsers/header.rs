// SPDX-License-Identifier: GPL-2.0
// Translated from perf/ui/browsers/header.c.
// Original dependencies: ui/browser.h, ui/keysyms.h, ui/ui.h, ui/util.h,
// ui/libslang.h, util/header.h, util/session.h, sys/ttydefaults.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_session {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ui_browser {
    pub entries: *mut c_void,
    pub refresh: Option<unsafe extern "C" fn(*mut ui_browser) -> c_uint>,
    pub seek: Option<unsafe extern "C" fn(*mut ui_browser, c_ulong)>,
    pub write: Option<unsafe extern "C" fn(*mut ui_browser, *mut c_void, c_int)>,
    pub nr_entries: c_int,
    pub priv_: *mut c_void,
    pub width: c_uint,
}

type c_uint = u32;

unsafe extern "C" {
    static HE_COLORSET_SELECTED: c_int;
    static HE_COLORSET_NORMAL: c_int;

    static K_RIGHT: c_int;
    static K_LEFT: c_int;
    static K_F1: c_int;
    static K_ESC: c_int;

    fn ui_browser__is_current_entry(browser: *mut ui_browser, row: c_int) -> bool;
    fn ui_browser__set_color(browser: *mut ui_browser, color: c_int);
    fn ui_browser__write_nstring(browser: *mut ui_browser, str_: *const c_char, width: c_uint);
    fn ui_browser__show(
        browser: *mut ui_browser,
        title: *const c_char,
        helpline: *const c_char,
    ) -> c_int;
    fn ui_browser__run(browser: *mut ui_browser, delay_secs: c_int) -> c_int;
    fn ui_browser__help_window(browser: *mut ui_browser, text: *const c_char);
    fn ui_browser__warn_unhandled_hotkey(
        browser: *mut ui_browser,
        key: c_int,
        delay_secs: c_int,
        msg: *const c_char,
    );
    fn ui_browser__hide(browser: *mut ui_browser);
    fn ui_browser__argv_refresh(browser: *mut ui_browser) -> c_uint;
    fn ui_browser__argv_seek(browser: *mut ui_browser, offset: c_ulong);

    fn perf_header__fprintf_info(session: *mut perf_session, fp: *mut FILE, full: bool);

    fn open_memstream(ptr: *mut *mut c_char, sizeloc: *mut size_t) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);

    fn BUG_ON(condition: c_int);
}

const fn CTRL(ch: u8) -> c_int {
    (ch & 0x1f) as c_int
}

unsafe extern "C" fn ui_browser__argv_write(
    browser: *mut ui_browser,
    entry: *mut c_void,
    row: c_int,
) {
    let arg = entry as *mut *mut c_char;
    let mut str_ = *arg;
    let empty: [c_char; 2] = [b' ' as c_char, 0];
    let current_entry = ui_browser__is_current_entry(browser, row);
    let offset = (*browser).priv_ as c_ulong;

    if offset as size_t >= strlen(str_) {
        str_ = empty.as_ptr() as *mut c_char;
    } else {
        str_ = str_.add(offset as usize);
    }

    ui_browser__set_color(
        browser,
        if current_entry {
            HE_COLORSET_SELECTED
        } else {
            HE_COLORSET_NORMAL
        },
    );

    ui_browser__write_nstring(browser, str_, (*browser).width);
}

unsafe fn list_menu__run(menu: *mut ui_browser) -> c_int {
    let mut key: c_int;
    let mut offset: c_ulong;
    static help: &[u8] = b"h/?/F1        Show this window\n\
UP/DOWN/PGUP\n\
PGDN/SPACE\n\
LEFT/RIGHT    Navigate\n\
q/ESC/CTRL+C  Exit browser\0";

    if ui_browser__show(
        menu,
        c"Header information".as_ptr(),
        c"Press 'q' to exit".as_ptr(),
    ) < 0
    {
        return -1;
    }

    loop {
        key = ui_browser__run(menu, 0);

        if key == K_RIGHT {
            offset = (*menu).priv_ as c_ulong;
            offset = offset.wrapping_add(10);
            (*menu).priv_ = offset as *mut c_void;
            continue;
        } else if key == K_LEFT {
            offset = (*menu).priv_ as c_ulong;
            if offset >= 10 {
                offset -= 10;
            }
            (*menu).priv_ = offset as *mut c_void;
            continue;
        } else if key == K_F1 || key == b'h' as c_int || key == b'?' as c_int {
            ui_browser__help_window(menu, help.as_ptr() as *const c_char);
            continue;
        } else if key == K_ESC || key == b'q' as c_int || key == CTRL(b'c') {
            key = -1;
        } else {
            ui_browser__warn_unhandled_hotkey(
                menu,
                key,
                0,
                c", use 'h'/'?'/F1 to see actions".as_ptr(),
            );
            continue;
        }

        break;
    }

    ui_browser__hide(menu);
    key
}

unsafe fn ui__list_menu(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut menu = ui_browser {
        entries: argv as *mut c_void,
        refresh: Some(ui_browser__argv_refresh),
        seek: Some(ui_browser__argv_seek),
        write: Some(ui_browser__argv_write),
        nr_entries: argc,
        priv_: ptr::null_mut(),
        width: 0,
    };

    list_menu__run(&mut menu)
}

#[no_mangle]
pub unsafe extern "C" fn tui__header_window(session: *mut perf_session) -> c_int {
    let mut i: c_int;
    let mut argc: c_int = 0;
    let mut argv: *mut *mut c_char;
    let mut ptr_: *mut c_char = ptr::null_mut();
    let mut pos: *mut c_char;
    let mut size: size_t = 0;
    let fp = open_memstream(&mut ptr_, &mut size);

    perf_header__fprintf_info(session, fp, true);
    fclose(fp);

    pos = ptr_;
    argc = 0;
    loop {
        pos = strchr(pos, b'\n' as c_int);
        if pos.is_null() {
            break;
        }
        argc += 1;
        pos = pos.add(1);
    }

    argv = calloc((argc + 1) as size_t, size_of::<*mut c_char>()) as *mut *mut c_char;
    if argv.is_null() {
        goto_out(argv, ptr_);
        return 0;
    }

    pos = ptr_;
    *argv.add(0) = pos;
    i = 1;
    loop {
        pos = strchr(pos, b'\n' as c_int);
        if pos.is_null() {
            break;
        }
        *pos = 0;
        pos = pos.add(1);
        *argv.add(i as usize) = pos;
        i += 1;
    }

    BUG_ON((i != argc + 1) as c_int);

    ui__list_menu(argc, argv);

    goto_out(argv, ptr_);
    0
}

unsafe fn goto_out(argv: *mut *mut c_char, ptr_: *mut c_char) {
    free(argv as *mut c_void);
    free(ptr_ as *mut c_void);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
