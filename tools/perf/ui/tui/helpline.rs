// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/ui/tui/helpline.c.
// External declarations correspond to symbols provided by included C headers.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};

type size_t = usize;
type va_list = *mut c_void;

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ui_helpline {
    pub pop: Option<unsafe extern "C" fn()>,
    pub push: Option<unsafe extern "C" fn(msg: *const c_char)>,
    pub show: Option<unsafe extern "C" fn(format: *const c_char, ap: va_list) -> c_int>,
}

extern "C" {
    static mut SLtt_Screen_Rows: c_int;
    static mut SLtt_Screen_Cols: c_int;
    static mut ui__lock: mutex;
    static mut ui_helpline__current: [c_char; 1024];
    static mut helpline_fns: *mut ui_helpline;

    fn SLsmg_gotorc(r: c_int, c: c_int);
    fn SLsmg_set_color(color: c_int);
    fn SLsmg_write_nstring(s: *const c_char, len: c_int);
    fn SLsmg_refresh();
    fn strlcpy(dest: *mut c_char, src: *const c_char, size: size_t) -> size_t;
    fn vscnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, args: va_list) -> c_int;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn ui_helpline__puts(msg: *const c_char);
}

#[no_mangle]
pub static mut ui_helpline__last_msg: [c_char; 1024] = [0; 1024];

#[no_mangle]
pub static mut tui_helpline__set: bool = false;

unsafe extern "C" fn tui_helpline__pop() {}

unsafe extern "C" fn tui_helpline__push(msg: *const c_char) {
    let sz: size_t = core::mem::size_of_val(&ui_helpline__current);

    SLsmg_gotorc(SLtt_Screen_Rows - 1, 0);
    SLsmg_set_color(0);
    SLsmg_write_nstring(msg, SLtt_Screen_Cols);
    SLsmg_refresh();
    strlcpy(ui_helpline__current.as_mut_ptr(), msg, sz);
}

unsafe extern "C" fn tui_helpline__show(format: *const c_char, ap: va_list) -> c_int {
    let ret: c_int;
    static mut backlog: c_int = 0;

    mutex_lock(&mut ui__lock);
    ret = vscnprintf(
        ui_helpline__last_msg.as_mut_ptr().add(backlog as usize),
        core::mem::size_of_val(&ui_helpline__last_msg) - backlog as usize,
        format,
        ap,
    );
    backlog += ret;

    tui_helpline__set = true;

    if ui_helpline__last_msg[(backlog - 1) as usize] == b'\n' as c_char {
        ui_helpline__puts(ui_helpline__last_msg.as_ptr());
        SLsmg_refresh();
        backlog = 0;
    }
    mutex_unlock(&mut ui__lock);

    ret
}

#[no_mangle]
pub static mut tui_helpline_fns: ui_helpline = ui_helpline {
    pop: Some(tui_helpline__pop),
    push: Some(tui_helpline__push),
    show: Some(tui_helpline__show),
};

#[no_mangle]
pub unsafe extern "C" fn ui_helpline__init() {
    static SPACE: &[u8; 2] = b" \0";

    helpline_fns = &mut tui_helpline_fns;
    ui_helpline__puts(SPACE.as_ptr() as *const c_char);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
