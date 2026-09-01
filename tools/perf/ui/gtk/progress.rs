// SPDX-License-Identifier: GPL-2.0
//
// Translated from C source that included:
//   <inttypes.h>
//   "gtk.h"
//   "../progress.h"

use core::ffi::{c_char, c_double, c_int, c_void};
use core::ptr;

#[repr(C)]
pub struct GtkWidget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkBox {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkContainer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkWindow {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkProgressBar {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ui_progress {
    pub title: *const c_char,
    pub curr: u64,
    pub total: u64,
}

#[repr(C)]
pub struct ui_progress_ops {
    pub update: Option<unsafe extern "C" fn(*mut ui_progress)>,
    pub finish: Option<unsafe extern "C" fn()>,
}

const TRUE: c_int = 1;
const FALSE: c_int = 0;
const GTK_WINDOW_TOPLEVEL: c_int = 0;
const GTK_WIN_POS_CENTER: c_int = 1;

unsafe extern "C" {
    static mut ui_progress__ops: *mut ui_progress_ops;

    fn gtk_vbox_new(homogeneous: c_int, spacing: c_int) -> *mut GtkWidget;
    fn gtk_label_new(str_: *const c_char) -> *mut GtkWidget;
    fn gtk_window_new(type_: c_int) -> *mut GtkWidget;
    fn gtk_progress_bar_new() -> *mut GtkWidget;
    fn gtk_box_pack_start(
        box_: *mut GtkBox,
        child: *mut GtkWidget,
        expand: c_int,
        fill: c_int,
        padding: c_int,
    );
    fn gtk_container_add(container: *mut GtkContainer, widget: *mut GtkWidget);
    fn gtk_window_set_title(window: *mut GtkWindow, title: *const c_char);
    fn gtk_window_resize(window: *mut GtkWindow, width: c_int, height: c_int);
    fn gtk_window_set_position(window: *mut GtkWindow, position: c_int);
    fn gtk_widget_show_all(widget: *mut GtkWidget);
    fn gtk_progress_bar_set_fraction(pbar: *mut GtkProgressBar, fraction: c_double);
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn gtk_progress_bar_set_text(pbar: *mut GtkProgressBar, text: *const c_char);
    fn gtk_events_pending() -> c_int;
    fn gtk_main_iteration() -> c_int;
    fn gtk_widget_destroy(widget: *mut GtkWidget);
}

static mut dialog: *mut GtkWidget = ptr::null_mut();
static mut progress: *mut GtkWidget = ptr::null_mut();

unsafe extern "C" fn gtk_ui_progress__update(p: *mut ui_progress) {
    let fraction: c_double = if (*p).total != 0 {
        1.0f64 * (*p).curr as c_double / (*p).total as c_double
    } else {
        0.0
    };
    let mut buf = [0 as c_char; 1024];

    if dialog.is_null() {
        let vbox = gtk_vbox_new(TRUE, 5);
        let label = gtk_label_new((*p).title);

        dialog = gtk_window_new(GTK_WINDOW_TOPLEVEL);
        progress = gtk_progress_bar_new();

        gtk_box_pack_start(vbox as *mut GtkBox, label, TRUE, FALSE, 3);
        gtk_box_pack_start(vbox as *mut GtkBox, progress, TRUE, TRUE, 3);

        gtk_container_add(dialog as *mut GtkContainer, vbox);

        gtk_window_set_title(dialog as *mut GtkWindow, c"perf".as_ptr());
        gtk_window_resize(dialog as *mut GtkWindow, 300, 80);
        gtk_window_set_position(dialog as *mut GtkWindow, GTK_WIN_POS_CENTER);

        gtk_widget_show_all(dialog);
    }

    gtk_progress_bar_set_fraction(progress as *mut GtkProgressBar, fraction);
    snprintf(
        buf.as_mut_ptr(),
        buf.len(),
        c"%lu / %lu".as_ptr(),
        (*p).curr,
        (*p).total,
    );
    gtk_progress_bar_set_text(progress as *mut GtkProgressBar, buf.as_ptr());

    /* we didn't call gtk_main yet, so do it manually */
    while gtk_events_pending() != 0 {
        gtk_main_iteration();
    }
}

unsafe extern "C" fn gtk_ui_progress__finish() {
    /* this will also destroy all of its children */
    gtk_widget_destroy(dialog);

    dialog = ptr::null_mut();
}

static mut gtk_ui_progress__ops: ui_progress_ops = ui_progress_ops {
    update: Some(gtk_ui_progress__update),
    finish: Some(gtk_ui_progress__finish),
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gtk_ui_progress__init() {
    ui_progress__ops = &raw mut gtk_ui_progress__ops;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
