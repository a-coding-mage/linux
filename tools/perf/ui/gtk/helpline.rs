// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_uint, c_void};

// C dependencies: "gtk.h", <stdio.h>, <string.h>, <linux/kernel.h>,
// "../ui.h", and "../helpline.h".

type va_list = *mut c_void;

#[repr(C)]
pub struct GtkStatusbar {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_gtk_context {
    pub statbar: *mut c_void,
    pub statbar_ctx_id: c_uint,
}

#[repr(C)]
pub struct ui_helpline {
    pub pop: Option<unsafe extern "C" fn()>,
    pub push: Option<unsafe extern "C" fn(msg: *const c_char)>,
    pub show: Option<unsafe extern "C" fn(fmt: *const c_char, ap: va_list) -> c_int>,
}

unsafe extern "C" {
    static mut pgctx: *mut perf_gtk_context;
    static mut ui_helpline__current: [c_char; 512];
    static mut helpline_fns: *mut ui_helpline;

    fn perf_gtk__is_active_context(ctx: *mut perf_gtk_context) -> bool;
    fn gtk_statusbar_pop(statusbar: *mut GtkStatusbar, context_id: c_uint);
    fn gtk_statusbar_push(
        statusbar: *mut GtkStatusbar,
        context_id: c_uint,
        text: *const c_char,
    ) -> c_uint;
    fn vscnprintf(
        buf: *mut c_char,
        size: usize,
        fmt: *const c_char,
        args: va_list,
    ) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn ui_helpline__puts(msg: *const c_char);
}

#[inline]
unsafe fn GTK_STATUSBAR(ptr: *mut c_void) -> *mut GtkStatusbar {
    ptr as *mut GtkStatusbar
}

unsafe extern "C" fn gtk_helpline_pop() {
    if !unsafe { perf_gtk__is_active_context(pgctx) } {
        return;
    }

    unsafe {
        gtk_statusbar_pop(
            GTK_STATUSBAR((*pgctx).statbar),
            (*pgctx).statbar_ctx_id,
        );
    }
}

unsafe extern "C" fn gtk_helpline_push(msg: *const c_char) {
    if !unsafe { perf_gtk__is_active_context(pgctx) } {
        return;
    }

    unsafe {
        gtk_statusbar_push(
            GTK_STATUSBAR((*pgctx).statbar),
            (*pgctx).statbar_ctx_id,
            msg,
        );
    }
}

unsafe extern "C" fn gtk_helpline_show(fmt: *const c_char, ap: va_list) -> c_int {
    let ret: c_int;
    let mut ptr: *mut c_char;
    static mut backlog: c_int = 0;

    unsafe {
        ret = vscnprintf(
            ui_helpline__current.as_mut_ptr().offset(backlog as isize),
            ui_helpline__current.len().wrapping_sub(backlog as usize),
            fmt,
            ap,
        );
        backlog += ret;

        /* only first line can be displayed */
        ptr = strchr(ui_helpline__current.as_ptr(), '\n' as c_int);
        if !ptr.is_null()
            && ptr.offset_from(ui_helpline__current.as_ptr()) <= backlog as isize
        {
            *ptr = '\0' as c_char;
            ui_helpline__puts(ui_helpline__current.as_ptr());
            backlog = 0;
        }
    }

    ret
}

static mut gtk_helpline_fns: ui_helpline = ui_helpline {
    pop: Some(gtk_helpline_pop),
    push: Some(gtk_helpline_push),
    show: Some(gtk_helpline_show),
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_gtk__init_helpline() {
    unsafe {
        helpline_fns = &raw mut gtk_helpline_fns;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
