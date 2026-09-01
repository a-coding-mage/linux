// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/ui/tui/progress.c.
// C include dependencies:
// <linux/kernel.h>, "../progress.h", "../libslang.h", "../ui.h",
// "tui.h", "units.h", "../browser.h"

use core::ffi::{c_char, c_int, c_void};

const HE_COLORSET_SELECTED: c_int = 0; // Provided by external UI headers in the original C build.

#[repr(C)]
pub struct ui_progress {
    pub curr: u64,
    pub next: u64,
    pub step: u64,
    pub total: u64,
    pub size: usize,
    pub title: *const c_char,
}

#[repr(C)]
pub struct ui_progress_ops {
    pub init: Option<unsafe extern "C" fn(*mut ui_progress)>,
    pub update: Option<unsafe extern "C" fn(*mut ui_progress)>,
    pub finish: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" {
    static mut SLtt_Screen_Cols: c_int;
    static mut SLtt_Screen_Rows: c_int;
    static mut use_browser: c_int;
    static mut ui__lock: c_void;
    static mut ui_progress__ops: *mut ui_progress_ops;

    fn unit_number__scnprintf(buf: *mut c_char, size: usize, n: u64) -> c_int;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn ui__refresh_dimensions(force: bool);
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn SLsmg_set_color(color: c_int);
    fn SLsmg_draw_box(r: c_int, c: c_int, dr: c_int, dc: c_int);
    fn SLsmg_gotorc(r: c_int, c: c_int);
    fn SLsmg_write_string(s: *const c_char);
    fn SLsmg_fill_region(r: c_int, c: c_int, dr: c_int, dc: c_int, ch: c_char);
    fn SLsmg_refresh();
}

unsafe extern "C" fn __tui_progress__init(p: *mut ui_progress) {
    let cols = (SLtt_Screen_Cols - 2) as u64;
    let step = (*p).total / cols;
    (*p).step = if step != 0 { step } else { 1 };
    (*p).next = (*p).step;
}

unsafe extern "C" fn get_title(p: *mut ui_progress, buf: *mut c_char, size: usize) -> c_int {
    let mut buf_cur = [0 as c_char; 20];
    let mut buf_tot = [0 as c_char; 20];
    let mut ret: c_int;

    ret = unit_number__scnprintf(buf_cur.as_mut_ptr(), buf_cur.len(), (*p).curr);
    ret += unit_number__scnprintf(buf_tot.as_mut_ptr(), buf_tot.len(), (*p).total);

    ret + scnprintf(
        buf,
        size,
        b"%s [%s/%s]\0".as_ptr() as *const c_char,
        (*p).title,
        buf_cur.as_ptr(),
        buf_tot.as_ptr(),
    )
}

unsafe extern "C" fn tui_progress__update(p: *mut ui_progress) {
    let mut buf = [0 as c_char; 100];
    let mut title = (*p).title as *mut c_char;
    let bar: c_int;
    let mut y: c_int;
    /*
     * FIXME: We should have a per UI backend way of showing progress,
     * stdio will just show a percentage as NN%, etc.
     */
    if use_browser <= 0 {
        return;
    }

    if (*p).total == 0 {
        return;
    }

    if (*p).size != 0 {
        get_title(p, buf.as_mut_ptr(), buf.len());
        title = buf.as_mut_ptr();
    }

    ui__refresh_dimensions(false);
    mutex_lock(&raw mut ui__lock);
    y = SLtt_Screen_Rows / 2 - 2;
    SLsmg_set_color(0);
    SLsmg_draw_box(y, 0, 3, SLtt_Screen_Cols);
    SLsmg_gotorc(y, 1);
    y += 1;
    SLsmg_write_string(title);
    SLsmg_fill_region(y, 1, 1, SLtt_Screen_Cols - 2, b' ' as c_char);
    SLsmg_set_color(HE_COLORSET_SELECTED);
    bar = (((SLtt_Screen_Cols - 2) as u64 * (*p).curr) / (*p).total) as c_int;
    SLsmg_fill_region(y, 1, 1, bar, b' ' as c_char);
    SLsmg_refresh();
    mutex_unlock(&raw mut ui__lock);
}

unsafe extern "C" fn tui_progress__finish() {
    let y: c_int;

    if use_browser <= 0 {
        return;
    }

    ui__refresh_dimensions(false);
    mutex_lock(&raw mut ui__lock);
    y = SLtt_Screen_Rows / 2 - 2;
    SLsmg_set_color(0);
    SLsmg_fill_region(y, 0, 3, SLtt_Screen_Cols, b' ' as c_char);
    SLsmg_refresh();
    mutex_unlock(&raw mut ui__lock);
}

static mut tui_progress__ops: ui_progress_ops = ui_progress_ops {
    init: Some(__tui_progress__init),
    update: Some(tui_progress__update),
    finish: Some(tui_progress__finish),
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tui_progress__init() {
    ui_progress__ops = &raw mut tui_progress__ops;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
