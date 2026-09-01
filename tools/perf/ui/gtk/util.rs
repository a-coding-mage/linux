// SPDX-License-Identifier: GPL-2.0
// Translated from perf/ui/gtk/util.c. Original C dependencies:
// ../util.h, gtk.h, stdlib.h, string.h, linux/zalloc.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]
#![allow(unexpected_cfgs)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

pub type va_list = *mut c_void;

#[repr(C)]
pub struct GtkWidget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkWindow {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkDialog {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkLabel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkInfoBar {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkStatusbar {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_gtk_context {
    pub main_window: *mut GtkWidget,
    #[cfg(HAVE_GTK_INFO_BAR_SUPPORT)]
    pub message_label: *mut GtkWidget,
    #[cfg(HAVE_GTK_INFO_BAR_SUPPORT)]
    pub info_bar: *mut GtkWidget,
    #[cfg(not(HAVE_GTK_INFO_BAR_SUPPORT))]
    pub statbar: *mut GtkWidget,
    #[cfg(not(HAVE_GTK_INFO_BAR_SUPPORT))]
    pub statbar_ctx_id: c_uint,
}

#[repr(C)]
pub struct perf_error_ops {
    pub error: Option<unsafe extern "C" fn(format: *const c_char, args: va_list) -> c_int>,
    pub warning: Option<unsafe extern "C" fn(format: *const c_char, args: va_list) -> c_int>,
}

unsafe extern "C" {
    static mut stderr: *mut FILE;

    static GTK_DIALOG_DESTROY_WITH_PARENT: c_int;
    static GTK_MESSAGE_ERROR: c_int;
    static GTK_MESSAGE_WARNING: c_int;
    static GTK_BUTTONS_CLOSE: c_int;

    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn vasprintf(strp: *mut *mut c_char, fmt: *const c_char, ap: va_list) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn vfprintf(stream: *mut FILE, format: *const c_char, ap: va_list) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;

    fn perf_gtk__is_active_context(ctx: *mut perf_gtk_context) -> c_int;

    fn gtk_message_dialog_new_with_markup(
        parent: *mut GtkWindow,
        flags: c_int,
        type_: c_int,
        buttons: c_int,
        message_format: *const c_char,
        ...
    ) -> *mut GtkWidget;
    fn gtk_dialog_run(dialog: *mut GtkDialog) -> c_int;
    fn gtk_widget_destroy(widget: *mut GtkWidget);

    #[cfg(HAVE_GTK_INFO_BAR_SUPPORT)]
    fn gtk_label_set_text(label: *mut GtkLabel, str_: *const c_char);
    #[cfg(HAVE_GTK_INFO_BAR_SUPPORT)]
    fn gtk_info_bar_set_message_type(info_bar: *mut GtkInfoBar, message_type: c_int);
    #[cfg(HAVE_GTK_INFO_BAR_SUPPORT)]
    fn gtk_widget_show(widget: *mut GtkWidget);

    #[cfg(not(HAVE_GTK_INFO_BAR_SUPPORT))]
    fn gtk_statusbar_pop(statusbar: *mut GtkStatusbar, context_id: c_uint);
    #[cfg(not(HAVE_GTK_INFO_BAR_SUPPORT))]
    fn gtk_statusbar_push(
        statusbar: *mut GtkStatusbar,
        context_id: c_uint,
        text: *const c_char,
    ) -> c_uint;
}

#[inline]
unsafe fn zfree(ptr: *mut *mut perf_gtk_context) {
    if !(*ptr).is_null() {
        free(*ptr as *mut c_void);
        *ptr = ptr::null_mut();
    }
}

#[inline]
unsafe fn GTK_WINDOW(widget: *mut GtkWidget) -> *mut GtkWindow {
    widget as *mut GtkWindow
}

#[inline]
unsafe fn GTK_DIALOG(widget: *mut GtkWidget) -> *mut GtkDialog {
    widget as *mut GtkDialog
}

#[cfg(HAVE_GTK_INFO_BAR_SUPPORT)]
#[inline]
unsafe fn GTK_LABEL(widget: *mut GtkWidget) -> *mut GtkLabel {
    widget as *mut GtkLabel
}

#[cfg(HAVE_GTK_INFO_BAR_SUPPORT)]
#[inline]
unsafe fn GTK_INFO_BAR(widget: *mut GtkWidget) -> *mut GtkInfoBar {
    widget as *mut GtkInfoBar
}

#[cfg(not(HAVE_GTK_INFO_BAR_SUPPORT))]
#[inline]
unsafe fn GTK_STATUSBAR(widget: *mut GtkWidget) -> *mut GtkStatusbar {
    widget as *mut GtkStatusbar
}

#[unsafe(no_mangle)]
pub static mut pgctx: *mut perf_gtk_context = ptr::null_mut();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_gtk__activate_context(
    window: *mut GtkWidget,
) -> *mut perf_gtk_context {
    let ctx: *mut perf_gtk_context = malloc(core::mem::size_of::<perf_gtk_context>())
        as *mut perf_gtk_context;

    if !ctx.is_null() {
        (*ctx).main_window = window;
    }

    ctx
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_gtk__deactivate_context(
    ctx: *mut *mut perf_gtk_context,
) -> c_int {
    if perf_gtk__is_active_context(*ctx) == 0 {
        return -1;
    }

    zfree(ctx);
    0
}

unsafe extern "C" fn perf_gtk__error(format: *const c_char, args: va_list) -> c_int {
    let mut msg: *mut c_char = ptr::null_mut();
    let mut dialog: *mut GtkWidget;

    if perf_gtk__is_active_context(pgctx) == 0 || vasprintf(&mut msg, format, args) < 0 {
        fprintf(stderr, c"Error:\n".as_ptr());
        vfprintf(stderr, format, args);
        fprintf(stderr, c"\n".as_ptr());
        return -1;
    }

    dialog = gtk_message_dialog_new_with_markup(
        GTK_WINDOW((*pgctx).main_window),
        GTK_DIALOG_DESTROY_WITH_PARENT,
        GTK_MESSAGE_ERROR,
        GTK_BUTTONS_CLOSE,
        c"<b>Error</b>\n\n%s".as_ptr(),
        msg,
    );
    gtk_dialog_run(GTK_DIALOG(dialog));

    gtk_widget_destroy(dialog);
    free(msg as *mut c_void);
    0
}

// Original C condition: #ifdef HAVE_GTK_INFO_BAR_SUPPORT
#[cfg(HAVE_GTK_INFO_BAR_SUPPORT)]
unsafe extern "C" fn perf_gtk__warning_info_bar(
    format: *const c_char,
    args: va_list,
) -> c_int {
    let mut msg: *mut c_char = ptr::null_mut();

    if perf_gtk__is_active_context(pgctx) == 0 || vasprintf(&mut msg, format, args) < 0 {
        fprintf(stderr, c"Warning:\n".as_ptr());
        vfprintf(stderr, format, args);
        fprintf(stderr, c"\n".as_ptr());
        return -1;
    }

    gtk_label_set_text(GTK_LABEL((*pgctx).message_label), msg);
    gtk_info_bar_set_message_type(GTK_INFO_BAR((*pgctx).info_bar), GTK_MESSAGE_WARNING);
    gtk_widget_show((*pgctx).info_bar);

    free(msg as *mut c_void);
    0
}

// Original C condition: #else of HAVE_GTK_INFO_BAR_SUPPORT
#[cfg(not(HAVE_GTK_INFO_BAR_SUPPORT))]
unsafe extern "C" fn perf_gtk__warning_statusbar(
    format: *const c_char,
    args: va_list,
) -> c_int {
    let mut msg: *mut c_char = ptr::null_mut();
    let mut p: *mut c_char;

    if perf_gtk__is_active_context(pgctx) == 0 || vasprintf(&mut msg, format, args) < 0 {
        fprintf(stderr, c"Warning:\n".as_ptr());
        vfprintf(stderr, format, args);
        fprintf(stderr, c"\n".as_ptr());
        return -1;
    }

    gtk_statusbar_pop(GTK_STATUSBAR((*pgctx).statbar), (*pgctx).statbar_ctx_id);

    /* Only first line can be displayed */
    p = strchr(msg, '\n' as c_int);
    if !p.is_null() {
        *p = '\0' as c_char;
    }

    gtk_statusbar_push(
        GTK_STATUSBAR((*pgctx).statbar),
        (*pgctx).statbar_ctx_id,
        msg,
    );

    free(msg as *mut c_void);
    0
}

#[unsafe(no_mangle)]
#[cfg(HAVE_GTK_INFO_BAR_SUPPORT)]
pub static mut perf_gtk_eops: perf_error_ops = perf_error_ops {
    error: Some(perf_gtk__error),
    warning: Some(perf_gtk__warning_info_bar),
};

#[unsafe(no_mangle)]
#[cfg(not(HAVE_GTK_INFO_BAR_SUPPORT))]
pub static mut perf_gtk_eops: perf_error_ops = perf_error_ops {
    error: Some(perf_gtk__error),
    warning: Some(perf_gtk__warning_statusbar),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
