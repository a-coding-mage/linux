// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/ui/tui/util.c. C include dependencies are represented as
// external declarations below.

use core::ffi::{c_char, c_int, c_uchar, c_void};

type bool_ = bool;
type VaList = *mut c_void;

const K_RIGHT: c_int = 0; /* external keysyms.h value */
const K_ENTER: c_int = 0; /* external keysyms.h value */
const K_LEFT: c_int = 0; /* external keysyms.h value */
const K_ESC: c_int = 0; /* external keysyms.h value */
const K_TIMER: c_int = 0; /* external keysyms.h value */
const K_BKSPC: c_int = 0; /* external keysyms.h value */
const HE_COLORSET_SELECTED: c_int = 0; /* external ui color value */
const HE_COLORSET_NORMAL: c_int = 0; /* external ui color value */

const fn CTRL(x: c_int) -> c_int {
    x & 0x1f
}

#[repr(C)]
pub struct ui_browser {
    pub entries: *mut c_void,
    pub refresh: Option<unsafe extern "C" fn(*mut ui_browser) -> c_int>,
    pub seek: Option<unsafe extern "C" fn(*mut ui_browser, i64, c_int)>,
    pub write: Option<unsafe extern "C" fn(*mut ui_browser, *mut c_void, c_int)>,
    pub nr_entries: c_int,
    pub index: c_int,
    pub width: c_int,
}

#[repr(C)]
pub struct perf_error_ops {
    pub error: Option<unsafe extern "C" fn(*const c_char, VaList) -> c_int>,
    pub warning: Option<unsafe extern "C" fn(*const c_char, VaList) -> c_int>,
}

unsafe extern "C" {
    static mut SLtt_Screen_Rows: c_int;
    static mut SLtt_Screen_Cols: c_int;
    static mut ui__lock: c_void;

    fn ui_browser__is_current_entry(browser: *mut ui_browser, row: c_int) -> bool_;
    fn ui_browser__set_color(browser: *mut ui_browser, color: c_int);
    fn ui_browser__write_nstring(browser: *mut ui_browser, str_: *const c_char, len: c_int);
    fn ui_browser__show(
        browser: *mut ui_browser,
        title: *const c_char,
        helpline: *const c_char,
    ) -> c_int;
    fn ui_browser__run(browser: *mut ui_browser, delay_secs: c_int) -> c_int;
    fn ui_browser__hide(browser: *mut ui_browser);
    fn ui_browser__argv_refresh(browser: *mut ui_browser) -> c_int;
    fn ui_browser__argv_seek(browser: *mut ui_browser, offset: i64, whence: c_int);

    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);

    fn SLsmg_set_color(color: c_int);
    fn SLsmg_draw_box(r: c_int, c: c_int, dr: c_int, dc: c_int);
    fn SLsmg_gotorc(r: c_int, c: c_int);
    fn SLsmg_write_string(str_: *const c_char);
    fn SLsmg_write_wrapped_string(
        str_: *mut c_uchar,
        r: c_int,
        c: c_int,
        dr: c_int,
        dc: c_int,
        fill: c_int,
    );
    fn SLsmg_write_nstring(str_: *const c_char, len: c_int);
    fn SLsmg_write_char(ch: c_int);
    fn SLsmg_refresh();

    fn ui__getch(delay_secs: c_int) -> c_int;
    fn ui_helpline__push(msg: *const c_char);

    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn vasprintf(strp: *mut *mut c_char, fmt: *const c_char, ap: VaList) -> c_int;
    fn free(ptr: *mut c_void);
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn vfprintf(stream: *mut c_void, format: *const c_char, ap: VaList) -> c_int;

    static mut stderr: *mut c_void;
}

unsafe extern "C" fn ui_browser__argv_write(
    browser: *mut ui_browser,
    entry: *mut c_void,
    row: c_int,
) {
    let arg = entry as *mut *mut c_char;
    let current_entry = ui_browser__is_current_entry(browser, row);

    ui_browser__set_color(
        browser,
        if current_entry {
            HE_COLORSET_SELECTED
        } else {
            HE_COLORSET_NORMAL
        },
    );
    ui_browser__write_nstring(browser, *arg, (*browser).width);
}

unsafe fn popup_menu__run(menu: *mut ui_browser, keyp: *mut c_int) -> c_int {
    let mut key: c_int;

    if ui_browser__show(
        menu,
        c" ".as_ptr(),
        c"ESC: exit, ENTER|->: Select option".as_ptr(),
    ) < 0
    {
        return -1;
    }

    loop {
        key = ui_browser__run(menu, 0);

        match key {
            K_RIGHT | K_ENTER => {
                key = (*menu).index;
            }
            K_LEFT | K_ESC | b'q' as c_int => {
                key = -1;
            }
            x if x == CTRL(b'c' as c_int) => {
                key = -1;
            }
            _ => {
                if !keyp.is_null() {
                    *keyp = key;
                    key = (*menu).nr_entries;
                    break;
                }
                continue;
            }
        }

        break;
    }

    ui_browser__hide(menu);
    key
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui__popup_menu(
    argc: c_int,
    argv: *const *mut c_char,
    keyp: *mut c_int,
) -> c_int {
    let mut menu = ui_browser {
        entries: argv as *mut c_void,
        refresh: Some(ui_browser__argv_refresh),
        seek: Some(ui_browser__argv_seek),
        write: Some(ui_browser__argv_write),
        nr_entries: argc,
        index: 0,
        width: 0,
    };
    popup_menu__run(&mut menu, keyp)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui_browser__input_window(
    title: *const c_char,
    text: *const c_char,
    input: *mut c_char,
    exit_msg: *const c_char,
    delay_secs: c_int,
) -> c_int {
    let mut x: c_int;
    let mut y: c_int;
    let mut len: c_int;
    let mut key: c_int;
    let mut max_len: c_int = 60;
    let mut nr_lines: c_int = 0;
    static mut BUF: [c_char; 50] = [0; 50];
    let mut t: *const c_char;

    t = text;
    loop {
        let mut sep = strchr(t, b'\n' as c_int) as *const c_char;

        if sep.is_null() {
            sep = strchr(t, b'\0' as c_int) as *const c_char;
        }
        len = sep.offset_from(t) as c_int;
        if max_len < len {
            max_len = len;
        }
        nr_lines += 1;
        if *sep == b'\0' as c_char {
            break;
        }
        t = sep.add(1);
    }

    mutex_lock(core::ptr::addr_of_mut!(ui__lock));

    max_len += 2;
    nr_lines += 8;
    y = SLtt_Screen_Rows / 2 - nr_lines / 2;
    x = SLtt_Screen_Cols / 2 - max_len / 2;

    SLsmg_set_color(0);
    SLsmg_draw_box(y, x, nr_lines, max_len);
    x += 1;
    if !title.is_null() {
        SLsmg_gotorc(y, x + 1);
        SLsmg_write_string(title);
    }
    y += 1;
    SLsmg_gotorc(y, x);
    nr_lines -= 7;
    max_len -= 2;
    SLsmg_write_wrapped_string(text as *mut c_uchar, y, x, nr_lines, max_len, 1);
    y += nr_lines;
    len = 5;
    while {
        let old = len;
        len -= 1;
        old != 0
    } {
        SLsmg_gotorc(y + len - 1, x);
        SLsmg_write_nstring(c" ".as_ptr(), max_len);
    }
    SLsmg_draw_box(y, x + 1, 3, max_len - 2);
    y += 1;

    SLsmg_gotorc(y + 3, x);
    SLsmg_write_nstring(exit_msg, max_len);
    SLsmg_refresh();

    mutex_unlock(core::ptr::addr_of_mut!(ui__lock));

    x += 2;
    len = 0;
    key = ui__getch(delay_secs);
    while key != K_TIMER && key != K_ENTER && key != K_ESC {
        mutex_lock(core::ptr::addr_of_mut!(ui__lock));

        if key == K_BKSPC {
            if len == 0 {
                mutex_unlock(core::ptr::addr_of_mut!(ui__lock));
                key = ui__getch(delay_secs);
                continue;
            }
            len -= 1;
            SLsmg_gotorc(y, x + len);
            SLsmg_write_char(b' ' as c_int);
        } else {
            BUF[len as usize] = key as c_char;
            SLsmg_gotorc(y, x + len);
            len += 1;
            SLsmg_write_char(key);
        }
        SLsmg_refresh();

        mutex_unlock(core::ptr::addr_of_mut!(ui__lock));

        /* XXX more graceful overflow handling needed */
        if len == (core::mem::size_of::<[c_char; 50]>() - 1) as c_int {
            ui_helpline__push(c"maximum size of symbol name reached!".as_ptr());
            key = K_ENTER;
            break;
        }
        key = ui__getch(delay_secs);
    }

    BUF[len as usize] = b'\0' as c_char;
    strncpy(input, core::ptr::addr_of!(BUF).cast::<c_char>(), (len + 1) as usize);
    key
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __ui__info_window(
    title: *const c_char,
    text: *const c_char,
    exit_msg: *const c_char,
) {
    let mut x: c_int;
    let mut y: c_int;
    let mut max_len: c_int = 0;
    let mut nr_lines: c_int = 0;
    let mut t: *const c_char;

    t = text;
    loop {
        let mut sep = strchr(t, b'\n' as c_int) as *const c_char;
        let len: c_int;

        if sep.is_null() {
            sep = strchr(t, b'\0' as c_int) as *const c_char;
        }
        len = sep.offset_from(t) as c_int;
        if max_len < len {
            max_len = len;
        }
        nr_lines += 1;
        if *sep == b'\0' as c_char {
            break;
        }
        t = sep.add(1);
    }

    max_len += 2;
    nr_lines += 2;
    if !exit_msg.is_null() {
        nr_lines += 2;
    }
    y = SLtt_Screen_Rows / 2 - nr_lines / 2;
    x = SLtt_Screen_Cols / 2 - max_len / 2;

    SLsmg_set_color(0);
    SLsmg_draw_box(y, x, nr_lines, max_len);
    x += 1;
    if !title.is_null() {
        SLsmg_gotorc(y, x + 1);
        SLsmg_write_string(title);
    }
    y += 1;
    SLsmg_gotorc(y, x);
    if !exit_msg.is_null() {
        nr_lines -= 2;
    }
    max_len -= 2;
    SLsmg_write_wrapped_string(text as *mut c_uchar, y, x, nr_lines, max_len, 1);
    if !exit_msg.is_null() {
        SLsmg_gotorc(y + nr_lines - 2, x);
        SLsmg_write_nstring(c" ".as_ptr(), max_len);
        SLsmg_gotorc(y + nr_lines - 1, x);
        SLsmg_write_nstring(exit_msg, max_len);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui__info_window(title: *const c_char, text: *const c_char) {
    mutex_lock(core::ptr::addr_of_mut!(ui__lock));
    __ui__info_window(title, text, core::ptr::null());
    SLsmg_refresh();
    mutex_unlock(core::ptr::addr_of_mut!(ui__lock));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui__question_window(
    title: *const c_char,
    text: *const c_char,
    exit_msg: *const c_char,
    delay_secs: c_int,
) -> c_int {
    mutex_lock(core::ptr::addr_of_mut!(ui__lock));
    __ui__info_window(title, text, exit_msg);
    SLsmg_refresh();
    mutex_unlock(core::ptr::addr_of_mut!(ui__lock));
    ui__getch(delay_secs)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui__help_window(text: *const c_char) -> c_int {
    ui__question_window(c"Help".as_ptr(), text, c"Press any key...".as_ptr(), 0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui__dialog_yesno(msg: *const c_char) -> c_int {
    ui__question_window(
        core::ptr::null(),
        msg,
        c"Enter: Yes, ESC: No".as_ptr(),
        0,
    )
}

unsafe fn __ui__warning(title: *const c_char, format: *const c_char, args: VaList) -> c_int {
    let mut s: *mut c_char = core::ptr::null_mut();

    if vasprintf(&mut s, format, args) > 0 {
        let key: c_int;

        key = ui__question_window(title, s, c"Press any key...".as_ptr(), 0);
        free(s as *mut c_void);
        return key;
    }

    fprintf(stderr, c"%s\n".as_ptr(), title);
    vfprintf(stderr, format, args);
    K_ESC
}

unsafe extern "C" fn perf_tui__error(format: *const c_char, args: VaList) -> c_int {
    __ui__warning(c"Error:".as_ptr(), format, args)
}

unsafe extern "C" fn perf_tui__warning(format: *const c_char, args: VaList) -> c_int {
    __ui__warning(c"Warning:".as_ptr(), format, args)
}

#[unsafe(no_mangle)]
pub static mut perf_tui_eops: perf_error_ops = perf_error_ops {
    error: Some(perf_tui__error),
    warning: Some(perf_tui__warning),
};
